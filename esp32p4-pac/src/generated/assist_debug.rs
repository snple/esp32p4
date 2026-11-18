//! ASSIST_DEBUG
//! Debug Assist

#![allow(dead_code)]

pub const BASE: u32 = 0x3ff06000;

/// CORE_0_INTR_ENA Register
///
/// core0 monitor enable configuration register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CORE_0_DRAM0_EXCEPTION_MONITOR_ENA** `[11]` (RW)
///
///   DBUS busy monitor enbale
///
/// - **CORE_0_IRAM0_EXCEPTION_MONITOR_ENA** `[10]` (RW)
///
///   IBUS busy monitor enable
///
/// - **CORE_0_SP_SPILL_MAX_ENA** `[9]` (RW)
///
///   Core0 stackpoint overflow monitor enable
///
/// - **CORE_0_SP_SPILL_MIN_ENA** `[8]` (RW)
///
///   Core0 stackpoint underflow monitor enable
///
/// - **CORE_0_AREA_PIF_1_WR_ENA** `[7]` (RW)
///
///   Core0 PIF area1 write monitor enable
///
/// - **CORE_0_AREA_PIF_1_RD_ENA** `[6]` (RW)
///
///   Core0 PIF area1 read monitor enable
///
/// - **CORE_0_AREA_PIF_0_WR_ENA** `[5]` (RW)
///
///   Core0 PIF area0 write monitor enable
///
/// - **CORE_0_AREA_PIF_0_RD_ENA** `[4]` (RW)
///
///   Core0 PIF area0 read monitor enable
///
/// - **CORE_0_AREA_DRAM0_1_WR_ENA** `[3]` (RW)
///
///   Core0 dram0 area1 write monitor enable
///
/// - **CORE_0_AREA_DRAM0_1_RD_ENA** `[2]` (RW)
///
///   Core0 dram0 area1 read monitor enable
///
/// - **CORE_0_AREA_DRAM0_0_WR_ENA** `[1]` (RW)
///
///   Core0 dram0 area0 write monitor enable
///
/// - **CORE_0_AREA_DRAM0_0_RD_ENA** `[0]` (RW)
///
///   Core0 dram0 area0 read monitor enable
#[repr(transparent)]
pub struct Core0IntrEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0IntrEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0IntrEna {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_ENA**
    ///
    /// DBUS busy monitor enbale
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_OFFSET: u32 = 11;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK: u32 = 0x00000800;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_ENA: u32 = 1 << 11;

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_ENA**
    ///
    /// IBUS busy monitor enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_OFFSET: u32 = 10;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK: u32 = 0x00000400;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_ENA: u32 = 1 << 10;

    /// **CORE_0_SP_SPILL_MAX_ENA**
    ///
    /// Core0 stackpoint overflow monitor enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_0_SP_SPILL_MAX_ENA_OFFSET: u32 = 9;
    pub const CORE_0_SP_SPILL_MAX_ENA_MASK: u32 = 0x00000200;
    pub const CORE_0_SP_SPILL_MAX_ENA: u32 = 1 << 9;

    /// **CORE_0_SP_SPILL_MIN_ENA**
    ///
    /// Core0 stackpoint underflow monitor enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_0_SP_SPILL_MIN_ENA_OFFSET: u32 = 8;
    pub const CORE_0_SP_SPILL_MIN_ENA_MASK: u32 = 0x00000100;
    pub const CORE_0_SP_SPILL_MIN_ENA: u32 = 1 << 8;

    /// **CORE_0_AREA_PIF_1_WR_ENA**
    ///
    /// Core0 PIF area1 write monitor enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_0_AREA_PIF_1_WR_ENA_OFFSET: u32 = 7;
    pub const CORE_0_AREA_PIF_1_WR_ENA_MASK: u32 = 0x00000080;
    pub const CORE_0_AREA_PIF_1_WR_ENA: u32 = 1 << 7;

    /// **CORE_0_AREA_PIF_1_RD_ENA**
    ///
    /// Core0 PIF area1 read monitor enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_0_AREA_PIF_1_RD_ENA_OFFSET: u32 = 6;
    pub const CORE_0_AREA_PIF_1_RD_ENA_MASK: u32 = 0x00000040;
    pub const CORE_0_AREA_PIF_1_RD_ENA: u32 = 1 << 6;

    /// **CORE_0_AREA_PIF_0_WR_ENA**
    ///
    /// Core0 PIF area0 write monitor enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_0_AREA_PIF_0_WR_ENA_OFFSET: u32 = 5;
    pub const CORE_0_AREA_PIF_0_WR_ENA_MASK: u32 = 0x00000020;
    pub const CORE_0_AREA_PIF_0_WR_ENA: u32 = 1 << 5;

    /// **CORE_0_AREA_PIF_0_RD_ENA**
    ///
    /// Core0 PIF area0 read monitor enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_0_AREA_PIF_0_RD_ENA_OFFSET: u32 = 4;
    pub const CORE_0_AREA_PIF_0_RD_ENA_MASK: u32 = 0x00000010;
    pub const CORE_0_AREA_PIF_0_RD_ENA: u32 = 1 << 4;

    /// **CORE_0_AREA_DRAM0_1_WR_ENA**
    ///
    /// Core0 dram0 area1 write monitor enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_0_AREA_DRAM0_1_WR_ENA_OFFSET: u32 = 3;
    pub const CORE_0_AREA_DRAM0_1_WR_ENA_MASK: u32 = 0x00000008;
    pub const CORE_0_AREA_DRAM0_1_WR_ENA: u32 = 1 << 3;

    /// **CORE_0_AREA_DRAM0_1_RD_ENA**
    ///
    /// Core0 dram0 area1 read monitor enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_0_AREA_DRAM0_1_RD_ENA_OFFSET: u32 = 2;
    pub const CORE_0_AREA_DRAM0_1_RD_ENA_MASK: u32 = 0x00000004;
    pub const CORE_0_AREA_DRAM0_1_RD_ENA: u32 = 1 << 2;

    /// **CORE_0_AREA_DRAM0_0_WR_ENA**
    ///
    /// Core0 dram0 area0 write monitor enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_AREA_DRAM0_0_WR_ENA_OFFSET: u32 = 1;
    pub const CORE_0_AREA_DRAM0_0_WR_ENA_MASK: u32 = 0x00000002;
    pub const CORE_0_AREA_DRAM0_0_WR_ENA: u32 = 1 << 1;

    /// **CORE_0_AREA_DRAM0_0_RD_ENA**
    ///
    /// Core0 dram0 area0 read monitor enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_AREA_DRAM0_0_RD_ENA_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_RD_ENA_MASK: u32 = 0x00000001;
    pub const CORE_0_AREA_DRAM0_0_RD_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_ENA** `[11]` (RW)
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Set CORE_0_DRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_dram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// Clear CORE_0_DRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_dram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_ENA** `[10]` (RW)
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Set CORE_0_IRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_iram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// Clear CORE_0_IRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_iram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// **CORE_0_SP_SPILL_MAX_ENA** `[9]` (RW)
    ///
    /// Core0 stackpoint overflow monitor enable
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MAX_ENA_MASK) != 0
    }

    /// Set CORE_0_SP_SPILL_MAX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_sp_spill_max_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_SP_SPILL_MAX_ENA_MASK);
    }

    /// Clear CORE_0_SP_SPILL_MAX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_sp_spill_max_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_SP_SPILL_MAX_ENA_MASK);
    }

    /// **CORE_0_SP_SPILL_MIN_ENA** `[8]` (RW)
    ///
    /// Core0 stackpoint underflow monitor enable
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MIN_ENA_MASK) != 0
    }

    /// Set CORE_0_SP_SPILL_MIN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_sp_spill_min_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_SP_SPILL_MIN_ENA_MASK);
    }

    /// Clear CORE_0_SP_SPILL_MIN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_sp_spill_min_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_SP_SPILL_MIN_ENA_MASK);
    }

    /// **CORE_0_AREA_PIF_1_WR_ENA** `[7]` (RW)
    ///
    /// Core0 PIF area1 write monitor enable
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_WR_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_1_WR_ENA_MASK);
    }

    /// Clear CORE_0_AREA_PIF_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_1_WR_ENA_MASK);
    }

    /// **CORE_0_AREA_PIF_1_RD_ENA** `[6]` (RW)
    ///
    /// Core0 PIF area1 read monitor enable
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_RD_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_1_RD_ENA_MASK);
    }

    /// Clear CORE_0_AREA_PIF_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_1_RD_ENA_MASK);
    }

    /// **CORE_0_AREA_PIF_0_WR_ENA** `[5]` (RW)
    ///
    /// Core0 PIF area0 write monitor enable
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_WR_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_0_WR_ENA_MASK);
    }

    /// Clear CORE_0_AREA_PIF_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_0_WR_ENA_MASK);
    }

    /// **CORE_0_AREA_PIF_0_RD_ENA** `[4]` (RW)
    ///
    /// Core0 PIF area0 read monitor enable
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_RD_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_0_RD_ENA_MASK);
    }

    /// Clear CORE_0_AREA_PIF_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_0_RD_ENA_MASK);
    }

    /// **CORE_0_AREA_DRAM0_1_WR_ENA** `[3]` (RW)
    ///
    /// Core0 dram0 area1 write monitor enable
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_WR_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_1_WR_ENA_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_1_WR_ENA_MASK);
    }

    /// **CORE_0_AREA_DRAM0_1_RD_ENA** `[2]` (RW)
    ///
    /// Core0 dram0 area1 read monitor enable
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_RD_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_1_RD_ENA_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_1_RD_ENA_MASK);
    }

    /// **CORE_0_AREA_DRAM0_0_WR_ENA** `[1]` (RW)
    ///
    /// Core0 dram0 area0 write monitor enable
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_WR_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_0_WR_ENA_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_0_WR_ENA_MASK);
    }

    /// **CORE_0_AREA_DRAM0_0_RD_ENA** `[0]` (RW)
    ///
    /// Core0 dram0 area0 read monitor enable
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_RD_ENA_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_0_RD_ENA_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_0_RD_ENA_MASK);
    }
}

/// CORE_0_INTR_ENA value type for batch read/modify/write operations
///
/// Use `core0intrena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0intrena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0IntrEnaVal {
    bits: u32,
}

impl Core0IntrEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_EXCEPTION_MONITOR_ENA** `[11:11]`
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_EXCEPTION_MONITOR_ENA** `[10:10]`
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MAX_ENA** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor enable
    #[inline(always)]
    pub const fn core_0_sp_spill_max_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_SP_SPILL_MAX_ENA_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MIN_ENA** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor enable
    #[inline(always)]
    pub const fn core_0_sp_spill_min_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_SP_SPILL_MIN_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_WR_ENA** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor enable
    #[inline(always)]
    pub const fn core_0_area_pif_1_wr_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_PIF_1_WR_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_RD_ENA** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor enable
    #[inline(always)]
    pub const fn core_0_area_pif_1_rd_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_PIF_1_RD_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_WR_ENA** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor enable
    #[inline(always)]
    pub const fn core_0_area_pif_0_wr_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_PIF_0_WR_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_RD_ENA** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor enable
    #[inline(always)]
    pub const fn core_0_area_pif_0_rd_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_PIF_0_RD_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_WR_ENA** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor enable
    #[inline(always)]
    pub const fn core_0_area_dram0_1_wr_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_DRAM0_1_WR_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_RD_ENA** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor enable
    #[inline(always)]
    pub const fn core_0_area_dram0_1_rd_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_DRAM0_1_RD_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_WR_ENA** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor enable
    #[inline(always)]
    pub const fn core_0_area_dram0_0_wr_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_DRAM0_0_WR_ENA_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_RD_ENA** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor enable
    #[inline(always)]
    pub const fn core_0_area_dram0_0_rd_ena(&self) -> bool {
        (self.bits & Core0IntrEna::CORE_0_AREA_DRAM0_0_RD_ENA_MASK) != 0
    }

    /// Set **CORE_0_DRAM0_EXCEPTION_MONITOR_ENA** `[11:11]`
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub const fn with_core_0_dram0_exception_monitor_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_IRAM0_EXCEPTION_MONITOR_ENA** `[10:10]`
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub const fn with_core_0_iram0_exception_monitor_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MAX_ENA** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor enable
    #[inline(always)]
    pub const fn with_core_0_sp_spill_max_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_SP_SPILL_MAX_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_SP_SPILL_MAX_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MIN_ENA** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor enable
    #[inline(always)]
    pub const fn with_core_0_sp_spill_min_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_SP_SPILL_MIN_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_SP_SPILL_MIN_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_WR_ENA** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_PIF_1_WR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_PIF_1_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_RD_ENA** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_PIF_1_RD_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_PIF_1_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_WR_ENA** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_PIF_0_WR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_PIF_0_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_RD_ENA** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_PIF_0_RD_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_PIF_0_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_WR_ENA** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_DRAM0_1_WR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_DRAM0_1_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_RD_ENA** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_DRAM0_1_RD_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_DRAM0_1_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_WR_ENA** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_DRAM0_0_WR_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_DRAM0_0_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_RD_ENA** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrEna::CORE_0_AREA_DRAM0_0_RD_ENA_MASK
            } else {
                self.bits & !Core0IntrEna::CORE_0_AREA_DRAM0_0_RD_ENA_MASK
            },
        }
    }
}

impl Default for Core0IntrEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0IntrEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0IntrEnaVal {
        Core0IntrEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0IntrEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0IntrEnaVal) -> Core0IntrEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_INTR_RAW Register
///
/// core0 monitor interrupt status register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CORE_0_DRAM0_EXCEPTION_MONITOR_RAW** `[11]` (RO)
///
///   DBUS busy monitor initerrupt status
///
/// - **CORE_0_IRAM0_EXCEPTION_MONITOR_RAW** `[10]` (RO)
///
///   IBUS busy monitor interrupt status
///
/// - **CORE_0_SP_SPILL_MAX_RAW** `[9]` (RO)
///
///   Core0 stackpoint overflow monitor interrupt status
///
/// - **CORE_0_SP_SPILL_MIN_RAW** `[8]` (RO)
///
///   Core0 stackpoint underflow monitor interrupt status
///
/// - **CORE_0_AREA_PIF_1_WR_RAW** `[7]` (RO)
///
///   Core0 PIF area1 write monitor interrupt status
///
/// - **CORE_0_AREA_PIF_1_RD_RAW** `[6]` (RO)
///
///   Core0 PIF area1 read monitor interrupt status
///
/// - **CORE_0_AREA_PIF_0_WR_RAW** `[5]` (RO)
///
///   Core0 PIF area0 write monitor interrupt status
///
/// - **CORE_0_AREA_PIF_0_RD_RAW** `[4]` (RO)
///
///   Core0 PIF area0 read monitor interrupt status
///
/// - **CORE_0_AREA_DRAM0_1_WR_RAW** `[3]` (RO)
///
///   Core0 dram0 area1 write monitor interrupt status
///
/// - **CORE_0_AREA_DRAM0_1_RD_RAW** `[2]` (RO)
///
///   Core0 dram0 area1 read monitor interrupt status
///
/// - **CORE_0_AREA_DRAM0_0_WR_RAW** `[1]` (RO)
///
///   Core0 dram0 area0 write monitor interrupt status
///
/// - **CORE_0_AREA_DRAM0_0_RD_RAW** `[0]` (RO)
///
///   Core0 dram0 area0 read monitor interrupt status
#[repr(transparent)]
pub struct Core0IntrRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0IntrRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0IntrRaw {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_RAW**
    ///
    /// DBUS busy monitor initerrupt status
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_OFFSET: u32 = 11;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_MASK: u32 = 0x00000800;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RAW: u32 = 1 << 11;

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_RAW**
    ///
    /// IBUS busy monitor interrupt status
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_OFFSET: u32 = 10;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_MASK: u32 = 0x00000400;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RAW: u32 = 1 << 10;

    /// **CORE_0_SP_SPILL_MAX_RAW**
    ///
    /// Core0 stackpoint overflow monitor interrupt status
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_0_SP_SPILL_MAX_RAW_OFFSET: u32 = 9;
    pub const CORE_0_SP_SPILL_MAX_RAW_MASK: u32 = 0x00000200;
    pub const CORE_0_SP_SPILL_MAX_RAW: u32 = 1 << 9;

    /// **CORE_0_SP_SPILL_MIN_RAW**
    ///
    /// Core0 stackpoint underflow monitor interrupt status
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_0_SP_SPILL_MIN_RAW_OFFSET: u32 = 8;
    pub const CORE_0_SP_SPILL_MIN_RAW_MASK: u32 = 0x00000100;
    pub const CORE_0_SP_SPILL_MIN_RAW: u32 = 1 << 8;

    /// **CORE_0_AREA_PIF_1_WR_RAW**
    ///
    /// Core0 PIF area1 write monitor interrupt status
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_0_AREA_PIF_1_WR_RAW_OFFSET: u32 = 7;
    pub const CORE_0_AREA_PIF_1_WR_RAW_MASK: u32 = 0x00000080;
    pub const CORE_0_AREA_PIF_1_WR_RAW: u32 = 1 << 7;

    /// **CORE_0_AREA_PIF_1_RD_RAW**
    ///
    /// Core0 PIF area1 read monitor interrupt status
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_0_AREA_PIF_1_RD_RAW_OFFSET: u32 = 6;
    pub const CORE_0_AREA_PIF_1_RD_RAW_MASK: u32 = 0x00000040;
    pub const CORE_0_AREA_PIF_1_RD_RAW: u32 = 1 << 6;

    /// **CORE_0_AREA_PIF_0_WR_RAW**
    ///
    /// Core0 PIF area0 write monitor interrupt status
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_0_AREA_PIF_0_WR_RAW_OFFSET: u32 = 5;
    pub const CORE_0_AREA_PIF_0_WR_RAW_MASK: u32 = 0x00000020;
    pub const CORE_0_AREA_PIF_0_WR_RAW: u32 = 1 << 5;

    /// **CORE_0_AREA_PIF_0_RD_RAW**
    ///
    /// Core0 PIF area0 read monitor interrupt status
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_0_AREA_PIF_0_RD_RAW_OFFSET: u32 = 4;
    pub const CORE_0_AREA_PIF_0_RD_RAW_MASK: u32 = 0x00000010;
    pub const CORE_0_AREA_PIF_0_RD_RAW: u32 = 1 << 4;

    /// **CORE_0_AREA_DRAM0_1_WR_RAW**
    ///
    /// Core0 dram0 area1 write monitor interrupt status
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_0_AREA_DRAM0_1_WR_RAW_OFFSET: u32 = 3;
    pub const CORE_0_AREA_DRAM0_1_WR_RAW_MASK: u32 = 0x00000008;
    pub const CORE_0_AREA_DRAM0_1_WR_RAW: u32 = 1 << 3;

    /// **CORE_0_AREA_DRAM0_1_RD_RAW**
    ///
    /// Core0 dram0 area1 read monitor interrupt status
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_0_AREA_DRAM0_1_RD_RAW_OFFSET: u32 = 2;
    pub const CORE_0_AREA_DRAM0_1_RD_RAW_MASK: u32 = 0x00000004;
    pub const CORE_0_AREA_DRAM0_1_RD_RAW: u32 = 1 << 2;

    /// **CORE_0_AREA_DRAM0_0_WR_RAW**
    ///
    /// Core0 dram0 area0 write monitor interrupt status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_AREA_DRAM0_0_WR_RAW_OFFSET: u32 = 1;
    pub const CORE_0_AREA_DRAM0_0_WR_RAW_MASK: u32 = 0x00000002;
    pub const CORE_0_AREA_DRAM0_0_WR_RAW: u32 = 1 << 1;

    /// **CORE_0_AREA_DRAM0_0_RD_RAW**
    ///
    /// Core0 dram0 area0 read monitor interrupt status
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_AREA_DRAM0_0_RD_RAW_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_RD_RAW_MASK: u32 = 0x00000001;
    pub const CORE_0_AREA_DRAM0_0_RD_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_RAW** `[11]` (RO)
    ///
    /// DBUS busy monitor initerrupt status
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_RAW** `[10]` (RO)
    ///
    /// IBUS busy monitor interrupt status
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// **CORE_0_SP_SPILL_MAX_RAW** `[9]` (RO)
    ///
    /// Core0 stackpoint overflow monitor interrupt status
    #[inline(always)]
    pub fn core_0_sp_spill_max_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MAX_RAW_MASK) != 0
    }

    /// **CORE_0_SP_SPILL_MIN_RAW** `[8]` (RO)
    ///
    /// Core0 stackpoint underflow monitor interrupt status
    #[inline(always)]
    pub fn core_0_sp_spill_min_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MIN_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_PIF_1_WR_RAW** `[7]` (RO)
    ///
    /// Core0 PIF area1 write monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_WR_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_PIF_1_RD_RAW** `[6]` (RO)
    ///
    /// Core0 PIF area1 read monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_RD_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_PIF_0_WR_RAW** `[5]` (RO)
    ///
    /// Core0 PIF area0 write monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_WR_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_PIF_0_RD_RAW** `[4]` (RO)
    ///
    /// Core0 PIF area0 read monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_RD_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_DRAM0_1_WR_RAW** `[3]` (RO)
    ///
    /// Core0 dram0 area1 write monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_WR_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_DRAM0_1_RD_RAW** `[2]` (RO)
    ///
    /// Core0 dram0 area1 read monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_RD_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_DRAM0_0_WR_RAW** `[1]` (RO)
    ///
    /// Core0 dram0 area0 write monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_WR_RAW_MASK) != 0
    }

    /// **CORE_0_AREA_DRAM0_0_RD_RAW** `[0]` (RO)
    ///
    /// Core0 dram0 area0 read monitor interrupt status
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_RD_RAW_MASK) != 0
    }
}

/// CORE_0_INTR_RAW value type for batch read/modify/write operations
///
/// Use `core0intrraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0intrraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0IntrRawVal {
    bits: u32,
}

impl Core0IntrRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_EXCEPTION_MONITOR_RAW** `[11:11]`
    ///
    /// DBUS busy monitor initerrupt status
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_EXCEPTION_MONITOR_RAW** `[10:10]`
    ///
    /// IBUS busy monitor interrupt status
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MAX_RAW** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor interrupt status
    #[inline(always)]
    pub const fn core_0_sp_spill_max_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_SP_SPILL_MAX_RAW_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MIN_RAW** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor interrupt status
    #[inline(always)]
    pub const fn core_0_sp_spill_min_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_SP_SPILL_MIN_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_WR_RAW** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_pif_1_wr_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_PIF_1_WR_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_RD_RAW** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_pif_1_rd_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_PIF_1_RD_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_WR_RAW** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_pif_0_wr_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_PIF_0_WR_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_RD_RAW** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_pif_0_rd_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_PIF_0_RD_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_WR_RAW** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_dram0_1_wr_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_DRAM0_1_WR_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_RD_RAW** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_dram0_1_rd_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_DRAM0_1_RD_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_WR_RAW** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_dram0_0_wr_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_DRAM0_0_WR_RAW_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_RD_RAW** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor interrupt status
    #[inline(always)]
    pub const fn core_0_area_dram0_0_rd_raw(&self) -> bool {
        (self.bits & Core0IntrRaw::CORE_0_AREA_DRAM0_0_RD_RAW_MASK) != 0
    }
}

impl Default for Core0IntrRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0IntrRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0IntrRawVal {
        Core0IntrRawVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_INTR_RLS Register
///
/// core0 monitor interrupt enable register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CORE_0_DRAM0_EXCEPTION_MONITOR_RLS** `[11]` (RW)
///
///   DBUS busy monitor interrupt enbale
///
/// - **CORE_0_IRAM0_EXCEPTION_MONITOR_RLS** `[10]` (RW)
///
///   IBUS busy monitor interrupt enable
///
/// - **CORE_0_SP_SPILL_MAX_RLS** `[9]` (RW)
///
///   Core0 stackpoint overflow monitor interrupt enable
///
/// - **CORE_0_SP_SPILL_MIN_RLS** `[8]` (RW)
///
///   Core0 stackpoint underflow monitor interrupt enable
///
/// - **CORE_0_AREA_PIF_1_WR_RLS** `[7]` (RW)
///
///   Core0 PIF area1 write monitor interrupt enable
///
/// - **CORE_0_AREA_PIF_1_RD_RLS** `[6]` (RW)
///
///   Core0 PIF area1 read monitor interrupt enable
///
/// - **CORE_0_AREA_PIF_0_WR_RLS** `[5]` (RW)
///
///   Core0 PIF area0 write monitor interrupt enable
///
/// - **CORE_0_AREA_PIF_0_RD_RLS** `[4]` (RW)
///
///   Core0 PIF area0 read monitor interrupt enable
///
/// - **CORE_0_AREA_DRAM0_1_WR_RLS** `[3]` (RW)
///
///   Core0 dram0 area1 write monitor interrupt enable
///
/// - **CORE_0_AREA_DRAM0_1_RD_RLS** `[2]` (RW)
///
///   Core0 dram0 area1 read monitor interrupt enable
///
/// - **CORE_0_AREA_DRAM0_0_WR_RLS** `[1]` (RW)
///
///   Core0 dram0 area0 write monitor interrupt enable
///
/// - **CORE_0_AREA_DRAM0_0_RD_RLS** `[0]` (RW)
///
///   Core0 dram0 area0 read monitor interrupt enable
#[repr(transparent)]
pub struct Core0IntrRls {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0IntrRls {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0IntrRls {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_RLS**
    ///
    /// DBUS busy monitor interrupt enbale
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_OFFSET: u32 = 11;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK: u32 = 0x00000800;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_RLS: u32 = 1 << 11;

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_RLS**
    ///
    /// IBUS busy monitor interrupt enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_OFFSET: u32 = 10;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK: u32 = 0x00000400;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_RLS: u32 = 1 << 10;

    /// **CORE_0_SP_SPILL_MAX_RLS**
    ///
    /// Core0 stackpoint overflow monitor interrupt enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_0_SP_SPILL_MAX_RLS_OFFSET: u32 = 9;
    pub const CORE_0_SP_SPILL_MAX_RLS_MASK: u32 = 0x00000200;
    pub const CORE_0_SP_SPILL_MAX_RLS: u32 = 1 << 9;

    /// **CORE_0_SP_SPILL_MIN_RLS**
    ///
    /// Core0 stackpoint underflow monitor interrupt enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_0_SP_SPILL_MIN_RLS_OFFSET: u32 = 8;
    pub const CORE_0_SP_SPILL_MIN_RLS_MASK: u32 = 0x00000100;
    pub const CORE_0_SP_SPILL_MIN_RLS: u32 = 1 << 8;

    /// **CORE_0_AREA_PIF_1_WR_RLS**
    ///
    /// Core0 PIF area1 write monitor interrupt enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_0_AREA_PIF_1_WR_RLS_OFFSET: u32 = 7;
    pub const CORE_0_AREA_PIF_1_WR_RLS_MASK: u32 = 0x00000080;
    pub const CORE_0_AREA_PIF_1_WR_RLS: u32 = 1 << 7;

    /// **CORE_0_AREA_PIF_1_RD_RLS**
    ///
    /// Core0 PIF area1 read monitor interrupt enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_0_AREA_PIF_1_RD_RLS_OFFSET: u32 = 6;
    pub const CORE_0_AREA_PIF_1_RD_RLS_MASK: u32 = 0x00000040;
    pub const CORE_0_AREA_PIF_1_RD_RLS: u32 = 1 << 6;

    /// **CORE_0_AREA_PIF_0_WR_RLS**
    ///
    /// Core0 PIF area0 write monitor interrupt enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_0_AREA_PIF_0_WR_RLS_OFFSET: u32 = 5;
    pub const CORE_0_AREA_PIF_0_WR_RLS_MASK: u32 = 0x00000020;
    pub const CORE_0_AREA_PIF_0_WR_RLS: u32 = 1 << 5;

    /// **CORE_0_AREA_PIF_0_RD_RLS**
    ///
    /// Core0 PIF area0 read monitor interrupt enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_0_AREA_PIF_0_RD_RLS_OFFSET: u32 = 4;
    pub const CORE_0_AREA_PIF_0_RD_RLS_MASK: u32 = 0x00000010;
    pub const CORE_0_AREA_PIF_0_RD_RLS: u32 = 1 << 4;

    /// **CORE_0_AREA_DRAM0_1_WR_RLS**
    ///
    /// Core0 dram0 area1 write monitor interrupt enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_0_AREA_DRAM0_1_WR_RLS_OFFSET: u32 = 3;
    pub const CORE_0_AREA_DRAM0_1_WR_RLS_MASK: u32 = 0x00000008;
    pub const CORE_0_AREA_DRAM0_1_WR_RLS: u32 = 1 << 3;

    /// **CORE_0_AREA_DRAM0_1_RD_RLS**
    ///
    /// Core0 dram0 area1 read monitor interrupt enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_0_AREA_DRAM0_1_RD_RLS_OFFSET: u32 = 2;
    pub const CORE_0_AREA_DRAM0_1_RD_RLS_MASK: u32 = 0x00000004;
    pub const CORE_0_AREA_DRAM0_1_RD_RLS: u32 = 1 << 2;

    /// **CORE_0_AREA_DRAM0_0_WR_RLS**
    ///
    /// Core0 dram0 area0 write monitor interrupt enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_AREA_DRAM0_0_WR_RLS_OFFSET: u32 = 1;
    pub const CORE_0_AREA_DRAM0_0_WR_RLS_MASK: u32 = 0x00000002;
    pub const CORE_0_AREA_DRAM0_0_WR_RLS: u32 = 1 << 1;

    /// **CORE_0_AREA_DRAM0_0_RD_RLS**
    ///
    /// Core0 dram0 area0 read monitor interrupt enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_AREA_DRAM0_0_RD_RLS_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_RD_RLS_MASK: u32 = 0x00000001;
    pub const CORE_0_AREA_DRAM0_0_RD_RLS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_RLS** `[11]` (RW)
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Set CORE_0_DRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_dram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// Clear CORE_0_DRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_dram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_RLS** `[10]` (RW)
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Set CORE_0_IRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_iram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// Clear CORE_0_IRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_iram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// **CORE_0_SP_SPILL_MAX_RLS** `[9]` (RW)
    ///
    /// Core0 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MAX_RLS_MASK) != 0
    }

    /// Set CORE_0_SP_SPILL_MAX_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_sp_spill_max_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_SP_SPILL_MAX_RLS_MASK);
    }

    /// Clear CORE_0_SP_SPILL_MAX_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_sp_spill_max_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_SP_SPILL_MAX_RLS_MASK);
    }

    /// **CORE_0_SP_SPILL_MIN_RLS** `[8]` (RW)
    ///
    /// Core0 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_SP_SPILL_MIN_RLS_MASK) != 0
    }

    /// Set CORE_0_SP_SPILL_MIN_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_sp_spill_min_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_SP_SPILL_MIN_RLS_MASK);
    }

    /// Clear CORE_0_SP_SPILL_MIN_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_sp_spill_min_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_SP_SPILL_MIN_RLS_MASK);
    }

    /// **CORE_0_AREA_PIF_1_WR_RLS** `[7]` (RW)
    ///
    /// Core0 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_WR_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_1_WR_RLS_MASK);
    }

    /// Clear CORE_0_AREA_PIF_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_1_WR_RLS_MASK);
    }

    /// **CORE_0_AREA_PIF_1_RD_RLS** `[6]` (RW)
    ///
    /// Core0 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_RD_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_1_RD_RLS_MASK);
    }

    /// Clear CORE_0_AREA_PIF_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_1_RD_RLS_MASK);
    }

    /// **CORE_0_AREA_PIF_0_WR_RLS** `[5]` (RW)
    ///
    /// Core0 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_WR_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_0_WR_RLS_MASK);
    }

    /// Clear CORE_0_AREA_PIF_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_0_WR_RLS_MASK);
    }

    /// **CORE_0_AREA_PIF_0_RD_RLS** `[4]` (RW)
    ///
    /// Core0 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_RD_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_PIF_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_pif_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_PIF_0_RD_RLS_MASK);
    }

    /// Clear CORE_0_AREA_PIF_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_pif_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_PIF_0_RD_RLS_MASK);
    }

    /// **CORE_0_AREA_DRAM0_1_WR_RLS** `[3]` (RW)
    ///
    /// Core0 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_WR_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_1_WR_RLS_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_1_WR_RLS_MASK);
    }

    /// **CORE_0_AREA_DRAM0_1_RD_RLS** `[2]` (RW)
    ///
    /// Core0 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_RD_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_1_RD_RLS_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_1_RD_RLS_MASK);
    }

    /// **CORE_0_AREA_DRAM0_0_WR_RLS** `[1]` (RW)
    ///
    /// Core0 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_WR_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_0_WR_RLS_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_0_WR_RLS_MASK);
    }

    /// **CORE_0_AREA_DRAM0_0_RD_RLS** `[0]` (RW)
    ///
    /// Core0 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_RD_RLS_MASK) != 0
    }

    /// Set CORE_0_AREA_DRAM0_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_area_dram0_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_AREA_DRAM0_0_RD_RLS_MASK);
    }

    /// Clear CORE_0_AREA_DRAM0_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_area_dram0_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_AREA_DRAM0_0_RD_RLS_MASK);
    }
}

/// CORE_0_INTR_RLS value type for batch read/modify/write operations
///
/// Use `core0intrrls.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0intrrls.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0IntrRlsVal {
    bits: u32,
}

impl Core0IntrRlsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_EXCEPTION_MONITOR_RLS** `[11:11]`
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_EXCEPTION_MONITOR_RLS** `[10:10]`
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MAX_RLS** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_sp_spill_max_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_SP_SPILL_MAX_RLS_MASK) != 0
    }

    /// Read **CORE_0_SP_SPILL_MIN_RLS** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_sp_spill_min_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_SP_SPILL_MIN_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_WR_RLS** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_pif_1_wr_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_PIF_1_WR_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_1_RD_RLS** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_pif_1_rd_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_PIF_1_RD_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_WR_RLS** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_pif_0_wr_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_PIF_0_WR_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_PIF_0_RD_RLS** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_pif_0_rd_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_PIF_0_RD_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_WR_RLS** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_dram0_1_wr_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_DRAM0_1_WR_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_1_RD_RLS** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_dram0_1_rd_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_DRAM0_1_RD_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_WR_RLS** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_dram0_0_wr_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_DRAM0_0_WR_RLS_MASK) != 0
    }

    /// Read **CORE_0_AREA_DRAM0_0_RD_RLS** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_0_area_dram0_0_rd_rls(&self) -> bool {
        (self.bits & Core0IntrRls::CORE_0_AREA_DRAM0_0_RD_RLS_MASK) != 0
    }

    /// Set **CORE_0_DRAM0_EXCEPTION_MONITOR_RLS** `[11:11]`
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub const fn with_core_0_dram0_exception_monitor_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_IRAM0_EXCEPTION_MONITOR_RLS** `[10:10]`
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_iram0_exception_monitor_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MAX_RLS** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_sp_spill_max_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_SP_SPILL_MAX_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_SP_SPILL_MAX_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MIN_RLS** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_sp_spill_min_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_SP_SPILL_MIN_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_SP_SPILL_MIN_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_WR_RLS** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_PIF_1_WR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_PIF_1_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_RD_RLS** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_PIF_1_RD_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_PIF_1_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_WR_RLS** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_PIF_0_WR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_PIF_0_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_RD_RLS** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_PIF_0_RD_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_PIF_0_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_WR_RLS** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_DRAM0_1_WR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_DRAM0_1_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_RD_RLS** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_DRAM0_1_RD_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_DRAM0_1_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_WR_RLS** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_DRAM0_0_WR_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_DRAM0_0_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_RD_RLS** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrRls::CORE_0_AREA_DRAM0_0_RD_RLS_MASK
            } else {
                self.bits & !Core0IntrRls::CORE_0_AREA_DRAM0_0_RD_RLS_MASK
            },
        }
    }
}

impl Default for Core0IntrRlsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0IntrRls {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0IntrRlsVal {
        Core0IntrRlsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0IntrRlsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0IntrRlsVal) -> Core0IntrRlsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_INTR_CLR Register
///
/// core0 monitor interrupt clr register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **CORE_0_DRAM0_EXCEPTION_MONITOR_CLR** `[11]` (WO)
///
///   DBUS busy monitor interrupt clr
///
/// - **CORE_0_IRAM0_EXCEPTION_MONITOR_CLR** `[10]` (WO)
///
///   IBUS busy monitor interrupt clr
///
/// - **CORE_0_SP_SPILL_MAX_CLR** `[9]` (WO)
///
///   Core0 stackpoint overflow monitor interrupt clr
///
/// - **CORE_0_SP_SPILL_MIN_CLR** `[8]` (WO)
///
///   Core0 stackpoint underflow monitor interrupt clr
///
/// - **CORE_0_AREA_PIF_1_WR_CLR** `[7]` (WO)
///
///   Core0 PIF area1 write monitor interrupt clr
///
/// - **CORE_0_AREA_PIF_1_RD_CLR** `[6]` (WO)
///
///   Core0 PIF area1 read monitor interrupt clr
///
/// - **CORE_0_AREA_PIF_0_WR_CLR** `[5]` (WO)
///
///   Core0 PIF area0 write monitor interrupt clr
///
/// - **CORE_0_AREA_PIF_0_RD_CLR** `[4]` (WO)
///
///   Core0 PIF area0 read monitor interrupt clr
///
/// - **CORE_0_AREA_DRAM0_1_WR_CLR** `[3]` (WO)
///
///   Core0 dram0 area1 write monitor interrupt clr
///
/// - **CORE_0_AREA_DRAM0_1_RD_CLR** `[2]` (WO)
///
///   Core0 dram0 area1 read monitor interrupt clr
///
/// - **CORE_0_AREA_DRAM0_0_WR_CLR** `[1]` (WO)
///
///   Core0 dram0 area0 write monitor interrupt clr
///
/// - **CORE_0_AREA_DRAM0_0_RD_CLR** `[0]` (WO)
///
///   Core0 dram0 area0 read monitor interrupt clr
#[repr(transparent)]
pub struct Core0IntrClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0IntrClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0IntrClr {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_EXCEPTION_MONITOR_CLR**
    ///
    /// DBUS busy monitor interrupt clr
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_OFFSET: u32 = 11;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_MASK: u32 = 0x00000800;
    pub const CORE_0_DRAM0_EXCEPTION_MONITOR_CLR: u32 = 1 << 11;

    /// **CORE_0_IRAM0_EXCEPTION_MONITOR_CLR**
    ///
    /// IBUS busy monitor interrupt clr
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_OFFSET: u32 = 10;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_MASK: u32 = 0x00000400;
    pub const CORE_0_IRAM0_EXCEPTION_MONITOR_CLR: u32 = 1 << 10;

    /// **CORE_0_SP_SPILL_MAX_CLR**
    ///
    /// Core0 stackpoint overflow monitor interrupt clr
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_0_SP_SPILL_MAX_CLR_OFFSET: u32 = 9;
    pub const CORE_0_SP_SPILL_MAX_CLR_MASK: u32 = 0x00000200;
    pub const CORE_0_SP_SPILL_MAX_CLR: u32 = 1 << 9;

    /// **CORE_0_SP_SPILL_MIN_CLR**
    ///
    /// Core0 stackpoint underflow monitor interrupt clr
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_0_SP_SPILL_MIN_CLR_OFFSET: u32 = 8;
    pub const CORE_0_SP_SPILL_MIN_CLR_MASK: u32 = 0x00000100;
    pub const CORE_0_SP_SPILL_MIN_CLR: u32 = 1 << 8;

    /// **CORE_0_AREA_PIF_1_WR_CLR**
    ///
    /// Core0 PIF area1 write monitor interrupt clr
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_0_AREA_PIF_1_WR_CLR_OFFSET: u32 = 7;
    pub const CORE_0_AREA_PIF_1_WR_CLR_MASK: u32 = 0x00000080;
    pub const CORE_0_AREA_PIF_1_WR_CLR: u32 = 1 << 7;

    /// **CORE_0_AREA_PIF_1_RD_CLR**
    ///
    /// Core0 PIF area1 read monitor interrupt clr
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_0_AREA_PIF_1_RD_CLR_OFFSET: u32 = 6;
    pub const CORE_0_AREA_PIF_1_RD_CLR_MASK: u32 = 0x00000040;
    pub const CORE_0_AREA_PIF_1_RD_CLR: u32 = 1 << 6;

    /// **CORE_0_AREA_PIF_0_WR_CLR**
    ///
    /// Core0 PIF area0 write monitor interrupt clr
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_0_AREA_PIF_0_WR_CLR_OFFSET: u32 = 5;
    pub const CORE_0_AREA_PIF_0_WR_CLR_MASK: u32 = 0x00000020;
    pub const CORE_0_AREA_PIF_0_WR_CLR: u32 = 1 << 5;

    /// **CORE_0_AREA_PIF_0_RD_CLR**
    ///
    /// Core0 PIF area0 read monitor interrupt clr
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_0_AREA_PIF_0_RD_CLR_OFFSET: u32 = 4;
    pub const CORE_0_AREA_PIF_0_RD_CLR_MASK: u32 = 0x00000010;
    pub const CORE_0_AREA_PIF_0_RD_CLR: u32 = 1 << 4;

    /// **CORE_0_AREA_DRAM0_1_WR_CLR**
    ///
    /// Core0 dram0 area1 write monitor interrupt clr
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_0_AREA_DRAM0_1_WR_CLR_OFFSET: u32 = 3;
    pub const CORE_0_AREA_DRAM0_1_WR_CLR_MASK: u32 = 0x00000008;
    pub const CORE_0_AREA_DRAM0_1_WR_CLR: u32 = 1 << 3;

    /// **CORE_0_AREA_DRAM0_1_RD_CLR**
    ///
    /// Core0 dram0 area1 read monitor interrupt clr
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_0_AREA_DRAM0_1_RD_CLR_OFFSET: u32 = 2;
    pub const CORE_0_AREA_DRAM0_1_RD_CLR_MASK: u32 = 0x00000004;
    pub const CORE_0_AREA_DRAM0_1_RD_CLR: u32 = 1 << 2;

    /// **CORE_0_AREA_DRAM0_0_WR_CLR**
    ///
    /// Core0 dram0 area0 write monitor interrupt clr
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_AREA_DRAM0_0_WR_CLR_OFFSET: u32 = 1;
    pub const CORE_0_AREA_DRAM0_0_WR_CLR_MASK: u32 = 0x00000002;
    pub const CORE_0_AREA_DRAM0_0_WR_CLR: u32 = 1 << 1;

    /// **CORE_0_AREA_DRAM0_0_RD_CLR**
    ///
    /// Core0 dram0 area0 read monitor interrupt clr
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_AREA_DRAM0_0_RD_CLR_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_RD_CLR_MASK: u32 = 0x00000001;
    pub const CORE_0_AREA_DRAM0_0_RD_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CORE_0_DRAM0_EXCEPTION_MONITOR_CLR** (WO)
    ///
    /// DBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_dram0_exception_monitor_clr(&self) {
        self.value
            .write_bits(Self::CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_MASK);
    }

    /// Trigger **CORE_0_IRAM0_EXCEPTION_MONITOR_CLR** (WO)
    ///
    /// IBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_iram0_exception_monitor_clr(&self) {
        self.value
            .write_bits(Self::CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_MASK);
    }

    /// Trigger **CORE_0_SP_SPILL_MAX_CLR** (WO)
    ///
    /// Core0 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_sp_spill_max_clr(&self) {
        self.value.write_bits(Self::CORE_0_SP_SPILL_MAX_CLR_MASK);
    }

    /// Trigger **CORE_0_SP_SPILL_MIN_CLR** (WO)
    ///
    /// Core0 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_sp_spill_min_clr(&self) {
        self.value.write_bits(Self::CORE_0_SP_SPILL_MIN_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_PIF_1_WR_CLR** (WO)
    ///
    /// Core0 PIF area1 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_pif_1_wr_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_PIF_1_WR_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_PIF_1_RD_CLR** (WO)
    ///
    /// Core0 PIF area1 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_pif_1_rd_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_PIF_1_RD_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_PIF_0_WR_CLR** (WO)
    ///
    /// Core0 PIF area0 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_pif_0_wr_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_PIF_0_WR_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_PIF_0_RD_CLR** (WO)
    ///
    /// Core0 PIF area0 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_pif_0_rd_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_PIF_0_RD_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_DRAM0_1_WR_CLR** (WO)
    ///
    /// Core0 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_dram0_1_wr_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_DRAM0_1_WR_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_DRAM0_1_RD_CLR** (WO)
    ///
    /// Core0 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_dram0_1_rd_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_DRAM0_1_RD_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_DRAM0_0_WR_CLR** (WO)
    ///
    /// Core0 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_dram0_0_wr_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_DRAM0_0_WR_CLR_MASK);
    }

    /// Trigger **CORE_0_AREA_DRAM0_0_RD_CLR** (WO)
    ///
    /// Core0 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_0_area_dram0_0_rd_clr(&self) {
        self.value.write_bits(Self::CORE_0_AREA_DRAM0_0_RD_CLR_MASK);
    }
}

/// CORE_0_INTR_CLR value type for batch read/modify/write operations
///
/// Use `core0intrclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0intrclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0IntrClrVal {
    bits: u32,
}

impl Core0IntrClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **CORE_0_DRAM0_EXCEPTION_MONITOR_CLR** `[11:11]`
    ///
    /// DBUS busy monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_dram0_exception_monitor_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_IRAM0_EXCEPTION_MONITOR_CLR** `[10:10]`
    ///
    /// IBUS busy monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_iram0_exception_monitor_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MAX_CLR** `[9:9]`
    ///
    /// Core0 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_sp_spill_max_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_SP_SPILL_MAX_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_SP_SPILL_MAX_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_SP_SPILL_MIN_CLR** `[8:8]`
    ///
    /// Core0 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_sp_spill_min_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_SP_SPILL_MIN_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_SP_SPILL_MIN_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_WR_CLR** `[7:7]`
    ///
    /// Core0 PIF area1 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_PIF_1_WR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_PIF_1_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_1_RD_CLR** `[6:6]`
    ///
    /// Core0 PIF area1 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_PIF_1_RD_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_PIF_1_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_WR_CLR** `[5:5]`
    ///
    /// Core0 PIF area0 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_PIF_0_WR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_PIF_0_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_PIF_0_RD_CLR** `[4:4]`
    ///
    /// Core0 PIF area0 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_PIF_0_RD_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_PIF_0_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_WR_CLR** `[3:3]`
    ///
    /// Core0 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_DRAM0_1_WR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_DRAM0_1_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_1_RD_CLR** `[2:2]`
    ///
    /// Core0 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_DRAM0_1_RD_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_DRAM0_1_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_WR_CLR** `[1:1]`
    ///
    /// Core0 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_DRAM0_0_WR_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_DRAM0_0_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_0_AREA_DRAM0_0_RD_CLR** `[0:0]`
    ///
    /// Core0 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0IntrClr::CORE_0_AREA_DRAM0_0_RD_CLR_MASK
            } else {
                self.bits & !Core0IntrClr::CORE_0_AREA_DRAM0_0_RD_CLR_MASK
            },
        }
    }
}

impl Default for Core0IntrClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0IntrClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0IntrClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// CORE_0_AREA_DRAM0_0_MIN Register
///
/// core0 dram0 region0 addr configuration register
///
/// Address offset: 0x0010
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_0_AREA_DRAM0_0_MIN** `[31:0]` (RW)
///
///   Core0 dram0 region0 start addr
#[repr(transparent)]
pub struct Core0AreaDram00Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaDram00Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaDram00Min {
    // ======== Field Constants ========

    /// **CORE_0_AREA_DRAM0_0_MIN**
    ///
    /// Core0 dram0 region0 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_DRAM0_0_MIN_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_DRAM0_0_MIN** `[31:0]` (RW)
    ///
    /// Core0 dram0 region0 start addr
    #[inline(always)]
    pub fn core_0_area_dram0_0_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_MIN_MASK)
            >> Self::CORE_0_AREA_DRAM0_0_MIN_OFFSET
    }

    /// Write **CORE_0_AREA_DRAM0_0_MIN** `[31:0]` (RW)
    ///
    /// Core0 dram0 region0 start addr
    #[inline(always)]
    pub fn write_core_0_area_dram0_0_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_DRAM0_0_MIN_MASK)
                | ((value << Self::CORE_0_AREA_DRAM0_0_MIN_OFFSET)
                    & Self::CORE_0_AREA_DRAM0_0_MIN_MASK)
        });
    }
}

/// CORE_0_AREA_DRAM0_0_MIN value type for batch read/modify/write operations
///
/// Use `core0areadram00min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areadram00min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaDram00MinVal {
    bits: u32,
}

impl Core0AreaDram00MinVal {
    /// Create from raw bits
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

    /// Read **CORE_0_AREA_DRAM0_0_MIN** `[31:0]`
    ///
    /// Core0 dram0 region0 start addr
    #[inline(always)]
    pub const fn core_0_area_dram0_0_min(&self) -> u32 {
        (self.bits & Core0AreaDram00Min::CORE_0_AREA_DRAM0_0_MIN_MASK)
            >> Core0AreaDram00Min::CORE_0_AREA_DRAM0_0_MIN_OFFSET
    }

    /// Set **CORE_0_AREA_DRAM0_0_MIN** `[31:0]`
    ///
    /// Core0 dram0 region0 start addr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaDram00Min::CORE_0_AREA_DRAM0_0_MIN_MASK)
                | ((value << Core0AreaDram00Min::CORE_0_AREA_DRAM0_0_MIN_OFFSET)
                    & Core0AreaDram00Min::CORE_0_AREA_DRAM0_0_MIN_MASK),
        }
    }
}

impl Default for Core0AreaDram00MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core0AreaDram00Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaDram00MinVal {
        Core0AreaDram00MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaDram00MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaDram00MinVal) -> Core0AreaDram00MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_DRAM0_0_MAX Register
///
/// core0 dram0 region0 addr configuration register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CORE_0_AREA_DRAM0_0_MAX** `[31:0]` (RW)
///
///   Core0 dram0 region0 end addr
#[repr(transparent)]
pub struct Core0AreaDram00Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaDram00Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaDram00Max {
    // ======== Field Constants ========

    /// **CORE_0_AREA_DRAM0_0_MAX**
    ///
    /// Core0 dram0 region0 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_DRAM0_0_MAX_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_0_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_DRAM0_0_MAX** `[31:0]` (RW)
    ///
    /// Core0 dram0 region0 end addr
    #[inline(always)]
    pub fn core_0_area_dram0_0_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_0_MAX_MASK)
            >> Self::CORE_0_AREA_DRAM0_0_MAX_OFFSET
    }

    /// Write **CORE_0_AREA_DRAM0_0_MAX** `[31:0]` (RW)
    ///
    /// Core0 dram0 region0 end addr
    #[inline(always)]
    pub fn write_core_0_area_dram0_0_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_DRAM0_0_MAX_MASK)
                | ((value << Self::CORE_0_AREA_DRAM0_0_MAX_OFFSET)
                    & Self::CORE_0_AREA_DRAM0_0_MAX_MASK)
        });
    }
}

/// CORE_0_AREA_DRAM0_0_MAX value type for batch read/modify/write operations
///
/// Use `core0areadram00max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areadram00max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaDram00MaxVal {
    bits: u32,
}

impl Core0AreaDram00MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_DRAM0_0_MAX** `[31:0]`
    ///
    /// Core0 dram0 region0 end addr
    #[inline(always)]
    pub const fn core_0_area_dram0_0_max(&self) -> u32 {
        (self.bits & Core0AreaDram00Max::CORE_0_AREA_DRAM0_0_MAX_MASK)
            >> Core0AreaDram00Max::CORE_0_AREA_DRAM0_0_MAX_OFFSET
    }

    /// Set **CORE_0_AREA_DRAM0_0_MAX** `[31:0]`
    ///
    /// Core0 dram0 region0 end addr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_0_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaDram00Max::CORE_0_AREA_DRAM0_0_MAX_MASK)
                | ((value << Core0AreaDram00Max::CORE_0_AREA_DRAM0_0_MAX_OFFSET)
                    & Core0AreaDram00Max::CORE_0_AREA_DRAM0_0_MAX_MASK),
        }
    }
}

impl Default for Core0AreaDram00MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaDram00Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaDram00MaxVal {
        Core0AreaDram00MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaDram00MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaDram00MaxVal) -> Core0AreaDram00MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_DRAM0_1_MIN Register
///
/// core0 dram0 region1 addr configuration register
///
/// Address offset: 0x0018
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_0_AREA_DRAM0_1_MIN** `[31:0]` (RW)
///
///   Core0 dram0 region1 start addr
#[repr(transparent)]
pub struct Core0AreaDram01Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaDram01Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaDram01Min {
    // ======== Field Constants ========

    /// **CORE_0_AREA_DRAM0_1_MIN**
    ///
    /// Core0 dram0 region1 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_DRAM0_1_MIN_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_1_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_DRAM0_1_MIN** `[31:0]` (RW)
    ///
    /// Core0 dram0 region1 start addr
    #[inline(always)]
    pub fn core_0_area_dram0_1_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_MIN_MASK)
            >> Self::CORE_0_AREA_DRAM0_1_MIN_OFFSET
    }

    /// Write **CORE_0_AREA_DRAM0_1_MIN** `[31:0]` (RW)
    ///
    /// Core0 dram0 region1 start addr
    #[inline(always)]
    pub fn write_core_0_area_dram0_1_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_DRAM0_1_MIN_MASK)
                | ((value << Self::CORE_0_AREA_DRAM0_1_MIN_OFFSET)
                    & Self::CORE_0_AREA_DRAM0_1_MIN_MASK)
        });
    }
}

/// CORE_0_AREA_DRAM0_1_MIN value type for batch read/modify/write operations
///
/// Use `core0areadram01min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areadram01min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaDram01MinVal {
    bits: u32,
}

impl Core0AreaDram01MinVal {
    /// Create from raw bits
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

    /// Read **CORE_0_AREA_DRAM0_1_MIN** `[31:0]`
    ///
    /// Core0 dram0 region1 start addr
    #[inline(always)]
    pub const fn core_0_area_dram0_1_min(&self) -> u32 {
        (self.bits & Core0AreaDram01Min::CORE_0_AREA_DRAM0_1_MIN_MASK)
            >> Core0AreaDram01Min::CORE_0_AREA_DRAM0_1_MIN_OFFSET
    }

    /// Set **CORE_0_AREA_DRAM0_1_MIN** `[31:0]`
    ///
    /// Core0 dram0 region1 start addr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaDram01Min::CORE_0_AREA_DRAM0_1_MIN_MASK)
                | ((value << Core0AreaDram01Min::CORE_0_AREA_DRAM0_1_MIN_OFFSET)
                    & Core0AreaDram01Min::CORE_0_AREA_DRAM0_1_MIN_MASK),
        }
    }
}

impl Default for Core0AreaDram01MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core0AreaDram01Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaDram01MinVal {
        Core0AreaDram01MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaDram01MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaDram01MinVal) -> Core0AreaDram01MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_DRAM0_1_MAX Register
///
/// core0 dram0 region1 addr configuration register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **CORE_0_AREA_DRAM0_1_MAX** `[31:0]` (RW)
///
///   Core0 dram0 region1 end addr
#[repr(transparent)]
pub struct Core0AreaDram01Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaDram01Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaDram01Max {
    // ======== Field Constants ========

    /// **CORE_0_AREA_DRAM0_1_MAX**
    ///
    /// Core0 dram0 region1 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_DRAM0_1_MAX_OFFSET: u32 = 0;
    pub const CORE_0_AREA_DRAM0_1_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_DRAM0_1_MAX** `[31:0]` (RW)
    ///
    /// Core0 dram0 region1 end addr
    #[inline(always)]
    pub fn core_0_area_dram0_1_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_DRAM0_1_MAX_MASK)
            >> Self::CORE_0_AREA_DRAM0_1_MAX_OFFSET
    }

    /// Write **CORE_0_AREA_DRAM0_1_MAX** `[31:0]` (RW)
    ///
    /// Core0 dram0 region1 end addr
    #[inline(always)]
    pub fn write_core_0_area_dram0_1_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_DRAM0_1_MAX_MASK)
                | ((value << Self::CORE_0_AREA_DRAM0_1_MAX_OFFSET)
                    & Self::CORE_0_AREA_DRAM0_1_MAX_MASK)
        });
    }
}

/// CORE_0_AREA_DRAM0_1_MAX value type for batch read/modify/write operations
///
/// Use `core0areadram01max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areadram01max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaDram01MaxVal {
    bits: u32,
}

impl Core0AreaDram01MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_DRAM0_1_MAX** `[31:0]`
    ///
    /// Core0 dram0 region1 end addr
    #[inline(always)]
    pub const fn core_0_area_dram0_1_max(&self) -> u32 {
        (self.bits & Core0AreaDram01Max::CORE_0_AREA_DRAM0_1_MAX_MASK)
            >> Core0AreaDram01Max::CORE_0_AREA_DRAM0_1_MAX_OFFSET
    }

    /// Set **CORE_0_AREA_DRAM0_1_MAX** `[31:0]`
    ///
    /// Core0 dram0 region1 end addr
    #[inline(always)]
    pub const fn with_core_0_area_dram0_1_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaDram01Max::CORE_0_AREA_DRAM0_1_MAX_MASK)
                | ((value << Core0AreaDram01Max::CORE_0_AREA_DRAM0_1_MAX_OFFSET)
                    & Core0AreaDram01Max::CORE_0_AREA_DRAM0_1_MAX_MASK),
        }
    }
}

impl Default for Core0AreaDram01MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaDram01Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaDram01MaxVal {
        Core0AreaDram01MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaDram01MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaDram01MaxVal) -> Core0AreaDram01MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_PIF_0_MIN Register
///
/// core0 PIF region0 addr configuration register
///
/// Address offset: 0x0020
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_0_AREA_PIF_0_MIN** `[31:0]` (RW)
///
///   Core0 PIF region0 start addr
#[repr(transparent)]
pub struct Core0AreaPif0Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaPif0Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaPif0Min {
    // ======== Field Constants ========

    /// **CORE_0_AREA_PIF_0_MIN**
    ///
    /// Core0 PIF region0 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_PIF_0_MIN_OFFSET: u32 = 0;
    pub const CORE_0_AREA_PIF_0_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_PIF_0_MIN** `[31:0]` (RW)
    ///
    /// Core0 PIF region0 start addr
    #[inline(always)]
    pub fn core_0_area_pif_0_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_MIN_MASK)
            >> Self::CORE_0_AREA_PIF_0_MIN_OFFSET
    }

    /// Write **CORE_0_AREA_PIF_0_MIN** `[31:0]` (RW)
    ///
    /// Core0 PIF region0 start addr
    #[inline(always)]
    pub fn write_core_0_area_pif_0_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_PIF_0_MIN_MASK)
                | ((value << Self::CORE_0_AREA_PIF_0_MIN_OFFSET) & Self::CORE_0_AREA_PIF_0_MIN_MASK)
        });
    }
}

/// CORE_0_AREA_PIF_0_MIN value type for batch read/modify/write operations
///
/// Use `core0areapif0min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areapif0min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaPif0MinVal {
    bits: u32,
}

impl Core0AreaPif0MinVal {
    /// Create from raw bits
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

    /// Read **CORE_0_AREA_PIF_0_MIN** `[31:0]`
    ///
    /// Core0 PIF region0 start addr
    #[inline(always)]
    pub const fn core_0_area_pif_0_min(&self) -> u32 {
        (self.bits & Core0AreaPif0Min::CORE_0_AREA_PIF_0_MIN_MASK)
            >> Core0AreaPif0Min::CORE_0_AREA_PIF_0_MIN_OFFSET
    }

    /// Set **CORE_0_AREA_PIF_0_MIN** `[31:0]`
    ///
    /// Core0 PIF region0 start addr
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaPif0Min::CORE_0_AREA_PIF_0_MIN_MASK)
                | ((value << Core0AreaPif0Min::CORE_0_AREA_PIF_0_MIN_OFFSET)
                    & Core0AreaPif0Min::CORE_0_AREA_PIF_0_MIN_MASK),
        }
    }
}

impl Default for Core0AreaPif0MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core0AreaPif0Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaPif0MinVal {
        Core0AreaPif0MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaPif0MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaPif0MinVal) -> Core0AreaPif0MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_PIF_0_MAX Register
///
/// core0 PIF region0 addr configuration register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **CORE_0_AREA_PIF_0_MAX** `[31:0]` (RW)
///
///   Core0 PIF region0 end addr
#[repr(transparent)]
pub struct Core0AreaPif0Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaPif0Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaPif0Max {
    // ======== Field Constants ========

    /// **CORE_0_AREA_PIF_0_MAX**
    ///
    /// Core0 PIF region0 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_PIF_0_MAX_OFFSET: u32 = 0;
    pub const CORE_0_AREA_PIF_0_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_PIF_0_MAX** `[31:0]` (RW)
    ///
    /// Core0 PIF region0 end addr
    #[inline(always)]
    pub fn core_0_area_pif_0_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_0_MAX_MASK)
            >> Self::CORE_0_AREA_PIF_0_MAX_OFFSET
    }

    /// Write **CORE_0_AREA_PIF_0_MAX** `[31:0]` (RW)
    ///
    /// Core0 PIF region0 end addr
    #[inline(always)]
    pub fn write_core_0_area_pif_0_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_PIF_0_MAX_MASK)
                | ((value << Self::CORE_0_AREA_PIF_0_MAX_OFFSET) & Self::CORE_0_AREA_PIF_0_MAX_MASK)
        });
    }
}

/// CORE_0_AREA_PIF_0_MAX value type for batch read/modify/write operations
///
/// Use `core0areapif0max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areapif0max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaPif0MaxVal {
    bits: u32,
}

impl Core0AreaPif0MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_PIF_0_MAX** `[31:0]`
    ///
    /// Core0 PIF region0 end addr
    #[inline(always)]
    pub const fn core_0_area_pif_0_max(&self) -> u32 {
        (self.bits & Core0AreaPif0Max::CORE_0_AREA_PIF_0_MAX_MASK)
            >> Core0AreaPif0Max::CORE_0_AREA_PIF_0_MAX_OFFSET
    }

    /// Set **CORE_0_AREA_PIF_0_MAX** `[31:0]`
    ///
    /// Core0 PIF region0 end addr
    #[inline(always)]
    pub const fn with_core_0_area_pif_0_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaPif0Max::CORE_0_AREA_PIF_0_MAX_MASK)
                | ((value << Core0AreaPif0Max::CORE_0_AREA_PIF_0_MAX_OFFSET)
                    & Core0AreaPif0Max::CORE_0_AREA_PIF_0_MAX_MASK),
        }
    }
}

impl Default for Core0AreaPif0MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaPif0Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaPif0MaxVal {
        Core0AreaPif0MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaPif0MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaPif0MaxVal) -> Core0AreaPif0MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_PIF_1_MIN Register
///
/// core0 PIF region1 addr configuration register
///
/// Address offset: 0x0028
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_0_AREA_PIF_1_MIN** `[31:0]` (RW)
///
///   Core0 PIF region1 start addr
#[repr(transparent)]
pub struct Core0AreaPif1Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaPif1Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaPif1Min {
    // ======== Field Constants ========

    /// **CORE_0_AREA_PIF_1_MIN**
    ///
    /// Core0 PIF region1 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_PIF_1_MIN_OFFSET: u32 = 0;
    pub const CORE_0_AREA_PIF_1_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_PIF_1_MIN** `[31:0]` (RW)
    ///
    /// Core0 PIF region1 start addr
    #[inline(always)]
    pub fn core_0_area_pif_1_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_MIN_MASK)
            >> Self::CORE_0_AREA_PIF_1_MIN_OFFSET
    }

    /// Write **CORE_0_AREA_PIF_1_MIN** `[31:0]` (RW)
    ///
    /// Core0 PIF region1 start addr
    #[inline(always)]
    pub fn write_core_0_area_pif_1_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_PIF_1_MIN_MASK)
                | ((value << Self::CORE_0_AREA_PIF_1_MIN_OFFSET) & Self::CORE_0_AREA_PIF_1_MIN_MASK)
        });
    }
}

/// CORE_0_AREA_PIF_1_MIN value type for batch read/modify/write operations
///
/// Use `core0areapif1min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areapif1min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaPif1MinVal {
    bits: u32,
}

impl Core0AreaPif1MinVal {
    /// Create from raw bits
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

    /// Read **CORE_0_AREA_PIF_1_MIN** `[31:0]`
    ///
    /// Core0 PIF region1 start addr
    #[inline(always)]
    pub const fn core_0_area_pif_1_min(&self) -> u32 {
        (self.bits & Core0AreaPif1Min::CORE_0_AREA_PIF_1_MIN_MASK)
            >> Core0AreaPif1Min::CORE_0_AREA_PIF_1_MIN_OFFSET
    }

    /// Set **CORE_0_AREA_PIF_1_MIN** `[31:0]`
    ///
    /// Core0 PIF region1 start addr
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaPif1Min::CORE_0_AREA_PIF_1_MIN_MASK)
                | ((value << Core0AreaPif1Min::CORE_0_AREA_PIF_1_MIN_OFFSET)
                    & Core0AreaPif1Min::CORE_0_AREA_PIF_1_MIN_MASK),
        }
    }
}

impl Default for Core0AreaPif1MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core0AreaPif1Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaPif1MinVal {
        Core0AreaPif1MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaPif1MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaPif1MinVal) -> Core0AreaPif1MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_PIF_1_MAX Register
///
/// core0 PIF region1 addr configuration register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **CORE_0_AREA_PIF_1_MAX** `[31:0]` (RW)
///
///   Core0 PIF region1 end addr
#[repr(transparent)]
pub struct Core0AreaPif1Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaPif1Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaPif1Max {
    // ======== Field Constants ========

    /// **CORE_0_AREA_PIF_1_MAX**
    ///
    /// Core0 PIF region1 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_PIF_1_MAX_OFFSET: u32 = 0;
    pub const CORE_0_AREA_PIF_1_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_PIF_1_MAX** `[31:0]` (RW)
    ///
    /// Core0 PIF region1 end addr
    #[inline(always)]
    pub fn core_0_area_pif_1_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_PIF_1_MAX_MASK)
            >> Self::CORE_0_AREA_PIF_1_MAX_OFFSET
    }

    /// Write **CORE_0_AREA_PIF_1_MAX** `[31:0]` (RW)
    ///
    /// Core0 PIF region1 end addr
    #[inline(always)]
    pub fn write_core_0_area_pif_1_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_AREA_PIF_1_MAX_MASK)
                | ((value << Self::CORE_0_AREA_PIF_1_MAX_OFFSET) & Self::CORE_0_AREA_PIF_1_MAX_MASK)
        });
    }
}

/// CORE_0_AREA_PIF_1_MAX value type for batch read/modify/write operations
///
/// Use `core0areapif1max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areapif1max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaPif1MaxVal {
    bits: u32,
}

impl Core0AreaPif1MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_PIF_1_MAX** `[31:0]`
    ///
    /// Core0 PIF region1 end addr
    #[inline(always)]
    pub const fn core_0_area_pif_1_max(&self) -> u32 {
        (self.bits & Core0AreaPif1Max::CORE_0_AREA_PIF_1_MAX_MASK)
            >> Core0AreaPif1Max::CORE_0_AREA_PIF_1_MAX_OFFSET
    }

    /// Set **CORE_0_AREA_PIF_1_MAX** `[31:0]`
    ///
    /// Core0 PIF region1 end addr
    #[inline(always)]
    pub const fn with_core_0_area_pif_1_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0AreaPif1Max::CORE_0_AREA_PIF_1_MAX_MASK)
                | ((value << Core0AreaPif1Max::CORE_0_AREA_PIF_1_MAX_OFFSET)
                    & Core0AreaPif1Max::CORE_0_AREA_PIF_1_MAX_MASK),
        }
    }
}

impl Default for Core0AreaPif1MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaPif1Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaPif1MaxVal {
        Core0AreaPif1MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0AreaPif1MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0AreaPif1MaxVal) -> Core0AreaPif1MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_AREA_PC Register
///
/// core0 area pc status register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **CORE_0_AREA_PC** `[31:0]` (RO)
///
///   the stackpointer when first touch region monitor interrupt
#[repr(transparent)]
pub struct Core0AreaPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaPc {
    // ======== Field Constants ========

    /// **CORE_0_AREA_PC**
    ///
    /// the stackpointer when first touch region monitor interrupt
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_PC_OFFSET: u32 = 0;
    pub const CORE_0_AREA_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_PC** `[31:0]` (RO)
    ///
    /// the stackpointer when first touch region monitor interrupt
    #[inline(always)]
    pub fn core_0_area_pc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_PC_MASK) >> Self::CORE_0_AREA_PC_OFFSET
    }
}

/// CORE_0_AREA_PC value type for batch read/modify/write operations
///
/// Use `core0areapc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areapc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaPcVal {
    bits: u32,
}

impl Core0AreaPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_PC** `[31:0]`
    ///
    /// the stackpointer when first touch region monitor interrupt
    #[inline(always)]
    pub const fn core_0_area_pc(&self) -> u32 {
        (self.bits & Core0AreaPc::CORE_0_AREA_PC_MASK) >> Core0AreaPc::CORE_0_AREA_PC_OFFSET
    }
}

impl Default for Core0AreaPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaPcVal {
        Core0AreaPcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_AREA_SP Register
///
/// core0 area sp status register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **CORE_0_AREA_SP** `[31:0]` (RO)
///
///   the PC when first touch region monitor interrupt
#[repr(transparent)]
pub struct Core0AreaSp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0AreaSp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0AreaSp {
    // ======== Field Constants ========

    /// **CORE_0_AREA_SP**
    ///
    /// the PC when first touch region monitor interrupt
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_AREA_SP_OFFSET: u32 = 0;
    pub const CORE_0_AREA_SP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_AREA_SP** `[31:0]` (RO)
    ///
    /// the PC when first touch region monitor interrupt
    #[inline(always)]
    pub fn core_0_area_sp(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_AREA_SP_MASK) >> Self::CORE_0_AREA_SP_OFFSET
    }
}

/// CORE_0_AREA_SP value type for batch read/modify/write operations
///
/// Use `core0areasp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0areasp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0AreaSpVal {
    bits: u32,
}

impl Core0AreaSpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_AREA_SP** `[31:0]`
    ///
    /// the PC when first touch region monitor interrupt
    #[inline(always)]
    pub const fn core_0_area_sp(&self) -> u32 {
        (self.bits & Core0AreaSp::CORE_0_AREA_SP_MASK) >> Core0AreaSp::CORE_0_AREA_SP_OFFSET
    }
}

impl Default for Core0AreaSpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0AreaSp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0AreaSpVal {
        Core0AreaSpVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_SP_MIN Register
///
/// stack min value
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **CORE_0_SP_MIN** `[31:0]` (RW)
///
///   core0 sp region configuration regsiter
#[repr(transparent)]
pub struct Core0SpMin {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0SpMin {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0SpMin {
    // ======== Field Constants ========

    /// **CORE_0_SP_MIN**
    ///
    /// core0 sp region configuration regsiter
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_SP_MIN_OFFSET: u32 = 0;
    pub const CORE_0_SP_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_SP_MIN** `[31:0]` (RW)
    ///
    /// core0 sp region configuration regsiter
    #[inline(always)]
    pub fn core_0_sp_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_SP_MIN_MASK) >> Self::CORE_0_SP_MIN_OFFSET
    }

    /// Write **CORE_0_SP_MIN** `[31:0]` (RW)
    ///
    /// core0 sp region configuration regsiter
    #[inline(always)]
    pub fn write_core_0_sp_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_SP_MIN_MASK)
                | ((value << Self::CORE_0_SP_MIN_OFFSET) & Self::CORE_0_SP_MIN_MASK)
        });
    }
}

/// CORE_0_SP_MIN value type for batch read/modify/write operations
///
/// Use `core0spmin.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0spmin.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0SpMinVal {
    bits: u32,
}

impl Core0SpMinVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_SP_MIN** `[31:0]`
    ///
    /// core0 sp region configuration regsiter
    #[inline(always)]
    pub const fn core_0_sp_min(&self) -> u32 {
        (self.bits & Core0SpMin::CORE_0_SP_MIN_MASK) >> Core0SpMin::CORE_0_SP_MIN_OFFSET
    }

    /// Set **CORE_0_SP_MIN** `[31:0]`
    ///
    /// core0 sp region configuration regsiter
    #[inline(always)]
    pub const fn with_core_0_sp_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0SpMin::CORE_0_SP_MIN_MASK)
                | ((value << Core0SpMin::CORE_0_SP_MIN_OFFSET) & Core0SpMin::CORE_0_SP_MIN_MASK),
        }
    }
}

impl Default for Core0SpMinVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0SpMin {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0SpMinVal {
        Core0SpMinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0SpMinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0SpMinVal) -> Core0SpMinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_SP_MAX Register
///
/// stack max value
///
/// Address offset: 0x003c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_0_SP_MAX** `[31:0]` (RW)
///
///   core0 sp pc status register
#[repr(transparent)]
pub struct Core0SpMax {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0SpMax {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0SpMax {
    // ======== Field Constants ========

    /// **CORE_0_SP_MAX**
    ///
    /// core0 sp pc status register
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_SP_MAX_OFFSET: u32 = 0;
    pub const CORE_0_SP_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_SP_MAX** `[31:0]` (RW)
    ///
    /// core0 sp pc status register
    #[inline(always)]
    pub fn core_0_sp_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_SP_MAX_MASK) >> Self::CORE_0_SP_MAX_OFFSET
    }

    /// Write **CORE_0_SP_MAX** `[31:0]` (RW)
    ///
    /// core0 sp pc status register
    #[inline(always)]
    pub fn write_core_0_sp_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_0_SP_MAX_MASK)
                | ((value << Self::CORE_0_SP_MAX_OFFSET) & Self::CORE_0_SP_MAX_MASK)
        });
    }
}

/// CORE_0_SP_MAX value type for batch read/modify/write operations
///
/// Use `core0spmax.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0spmax.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0SpMaxVal {
    bits: u32,
}

impl Core0SpMaxVal {
    /// Create from raw bits
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

    /// Read **CORE_0_SP_MAX** `[31:0]`
    ///
    /// core0 sp pc status register
    #[inline(always)]
    pub const fn core_0_sp_max(&self) -> u32 {
        (self.bits & Core0SpMax::CORE_0_SP_MAX_MASK) >> Core0SpMax::CORE_0_SP_MAX_OFFSET
    }

    /// Set **CORE_0_SP_MAX** `[31:0]`
    ///
    /// core0 sp pc status register
    #[inline(always)]
    pub const fn with_core_0_sp_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0SpMax::CORE_0_SP_MAX_MASK)
                | ((value << Core0SpMax::CORE_0_SP_MAX_OFFSET) & Core0SpMax::CORE_0_SP_MAX_MASK),
        }
    }
}

impl Default for Core0SpMaxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core0SpMax {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0SpMaxVal {
        Core0SpMaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0SpMaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0SpMaxVal) -> Core0SpMaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_SP_PC Register
///
/// stack monitor pc status register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **CORE_0_SP_PC** `[31:0]` (RO)
///
///   This regsiter stores the PC when trigger stack monitor.
#[repr(transparent)]
pub struct Core0SpPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0SpPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0SpPc {
    // ======== Field Constants ========

    /// **CORE_0_SP_PC**
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_SP_PC_OFFSET: u32 = 0;
    pub const CORE_0_SP_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_SP_PC** `[31:0]` (RO)
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    #[inline(always)]
    pub fn core_0_sp_pc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_SP_PC_MASK) >> Self::CORE_0_SP_PC_OFFSET
    }
}

/// CORE_0_SP_PC value type for batch read/modify/write operations
///
/// Use `core0sppc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0sppc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0SpPcVal {
    bits: u32,
}

impl Core0SpPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_SP_PC** `[31:0]`
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    #[inline(always)]
    pub const fn core_0_sp_pc(&self) -> u32 {
        (self.bits & Core0SpPc::CORE_0_SP_PC_MASK) >> Core0SpPc::CORE_0_SP_PC_OFFSET
    }
}

impl Default for Core0SpPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0SpPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0SpPcVal {
        Core0SpPcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_RCD_EN Register
///
/// record enable configuration register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **CORE_0_RCD_PDEBUGEN** `[1]` (RW)
///
///   Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
///
/// - **CORE_0_RCD_RECORDEN** `[0]` (RW)
///
///   Set 1 to enable record PC
#[repr(transparent)]
pub struct Core0RcdEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0RcdEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0RcdEn {
    // ======== Field Constants ========

    /// **CORE_0_RCD_PDEBUGEN**
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_RCD_PDEBUGEN_OFFSET: u32 = 1;
    pub const CORE_0_RCD_PDEBUGEN_MASK: u32 = 0x00000002;
    pub const CORE_0_RCD_PDEBUGEN: u32 = 1 << 1;

    /// **CORE_0_RCD_RECORDEN**
    ///
    /// Set 1 to enable record PC
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_RCD_RECORDEN_OFFSET: u32 = 0;
    pub const CORE_0_RCD_RECORDEN_MASK: u32 = 0x00000001;
    pub const CORE_0_RCD_RECORDEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_RCD_PDEBUGEN** `[1]` (RW)
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_RCD_PDEBUGEN_MASK) != 0
    }

    /// Set CORE_0_RCD_PDEBUGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_rcd_pdebugen(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_RCD_PDEBUGEN_MASK);
    }

    /// Clear CORE_0_RCD_PDEBUGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_rcd_pdebugen(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_RCD_PDEBUGEN_MASK);
    }

    /// **CORE_0_RCD_RECORDEN** `[0]` (RW)
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_RCD_RECORDEN_MASK) != 0
    }

    /// Set CORE_0_RCD_RECORDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_0_rcd_recorden(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_0_RCD_RECORDEN_MASK);
    }

    /// Clear CORE_0_RCD_RECORDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_0_rcd_recorden(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_0_RCD_RECORDEN_MASK);
    }
}

/// CORE_0_RCD_EN value type for batch read/modify/write operations
///
/// Use `core0rcden.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0rcden.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0RcdEnVal {
    bits: u32,
}

impl Core0RcdEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_RCD_PDEBUGEN** `[1:1]`
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub const fn core_0_rcd_pdebugen(&self) -> bool {
        (self.bits & Core0RcdEn::CORE_0_RCD_PDEBUGEN_MASK) != 0
    }

    /// Read **CORE_0_RCD_RECORDEN** `[0:0]`
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub const fn core_0_rcd_recorden(&self) -> bool {
        (self.bits & Core0RcdEn::CORE_0_RCD_RECORDEN_MASK) != 0
    }

    /// Set **CORE_0_RCD_PDEBUGEN** `[1:1]`
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub const fn with_core_0_rcd_pdebugen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0RcdEn::CORE_0_RCD_PDEBUGEN_MASK
            } else {
                self.bits & !Core0RcdEn::CORE_0_RCD_PDEBUGEN_MASK
            },
        }
    }

    /// Set **CORE_0_RCD_RECORDEN** `[0:0]`
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub const fn with_core_0_rcd_recorden(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core0RcdEn::CORE_0_RCD_RECORDEN_MASK
            } else {
                self.bits & !Core0RcdEn::CORE_0_RCD_RECORDEN_MASK
            },
        }
    }
}

impl Default for Core0RcdEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0RcdEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0RcdEnVal {
        Core0RcdEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0RcdEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0RcdEnVal) -> Core0RcdEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_0_RCD_PDEBUGPC Register
///
/// record status regsiter
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **CORE_0_RCD_PDEBUGPC** `[31:0]` (RO)
///
///   recorded PC
#[repr(transparent)]
pub struct Core0RcdPdebugpc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0RcdPdebugpc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0RcdPdebugpc {
    // ======== Field Constants ========

    /// **CORE_0_RCD_PDEBUGPC**
    ///
    /// recorded PC
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_RCD_PDEBUGPC_OFFSET: u32 = 0;
    pub const CORE_0_RCD_PDEBUGPC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_RCD_PDEBUGPC** `[31:0]` (RO)
    ///
    /// recorded PC
    #[inline(always)]
    pub fn core_0_rcd_pdebugpc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_RCD_PDEBUGPC_MASK)
            >> Self::CORE_0_RCD_PDEBUGPC_OFFSET
    }
}

/// CORE_0_RCD_PDEBUGPC value type for batch read/modify/write operations
///
/// Use `core0rcdpdebugpc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0rcdpdebugpc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0RcdPdebugpcVal {
    bits: u32,
}

impl Core0RcdPdebugpcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_RCD_PDEBUGPC** `[31:0]`
    ///
    /// recorded PC
    #[inline(always)]
    pub const fn core_0_rcd_pdebugpc(&self) -> u32 {
        (self.bits & Core0RcdPdebugpc::CORE_0_RCD_PDEBUGPC_MASK)
            >> Core0RcdPdebugpc::CORE_0_RCD_PDEBUGPC_OFFSET
    }
}

impl Default for Core0RcdPdebugpcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0RcdPdebugpc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0RcdPdebugpcVal {
        Core0RcdPdebugpcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_RCD_PDEBUGSP Register
///
/// record status regsiter
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **CORE_0_RCD_PDEBUGSP** `[31:0]` (RO)
///
///   recorded sp
#[repr(transparent)]
pub struct Core0RcdPdebugsp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0RcdPdebugsp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0RcdPdebugsp {
    // ======== Field Constants ========

    /// **CORE_0_RCD_PDEBUGSP**
    ///
    /// recorded sp
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_RCD_PDEBUGSP_OFFSET: u32 = 0;
    pub const CORE_0_RCD_PDEBUGSP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_RCD_PDEBUGSP** `[31:0]` (RO)
    ///
    /// recorded sp
    #[inline(always)]
    pub fn core_0_rcd_pdebugsp(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_RCD_PDEBUGSP_MASK)
            >> Self::CORE_0_RCD_PDEBUGSP_OFFSET
    }
}

/// CORE_0_RCD_PDEBUGSP value type for batch read/modify/write operations
///
/// Use `core0rcdpdebugsp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0rcdpdebugsp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0RcdPdebugspVal {
    bits: u32,
}

impl Core0RcdPdebugspVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_RCD_PDEBUGSP** `[31:0]`
    ///
    /// recorded sp
    #[inline(always)]
    pub const fn core_0_rcd_pdebugsp(&self) -> u32 {
        (self.bits & Core0RcdPdebugsp::CORE_0_RCD_PDEBUGSP_MASK)
            >> Core0RcdPdebugsp::CORE_0_RCD_PDEBUGSP_OFFSET
    }
}

impl Default for Core0RcdPdebugspVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0RcdPdebugsp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0RcdPdebugspVal {
        Core0RcdPdebugspVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_IRAM0_EXCEPTION_MONITOR_0 Register
///
/// exception monitor status register0
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **CORE_0_IRAM0_RECORDING_LOADSTORE_0** `[25]` (RO)
///
///   reg_core_0_iram0_recording_loadstore_0
///
/// - **CORE_0_IRAM0_RECORDING_WR_0** `[24]` (RO)
///
///   reg_core_0_iram0_recording_wr_0
///
/// - **CORE_0_IRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
///
///   reg_core_0_iram0_recording_addr_0
#[repr(transparent)]
pub struct Core0Iram0ExceptionMonitor0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Iram0ExceptionMonitor0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Iram0ExceptionMonitor0 {
    // ======== Field Constants ========

    /// **CORE_0_IRAM0_RECORDING_LOADSTORE_0**
    ///
    /// reg_core_0_iram0_recording_loadstore_0
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_0_OFFSET: u32 = 25;
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_0_MASK: u32 = 0x02000000;
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_0: u32 = 1 << 25;

    /// **CORE_0_IRAM0_RECORDING_WR_0**
    ///
    /// reg_core_0_iram0_recording_wr_0
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CORE_0_IRAM0_RECORDING_WR_0_OFFSET: u32 = 24;
    pub const CORE_0_IRAM0_RECORDING_WR_0_MASK: u32 = 0x01000000;
    pub const CORE_0_IRAM0_RECORDING_WR_0: u32 = 1 << 24;

    /// **CORE_0_IRAM0_RECORDING_ADDR_0**
    ///
    /// reg_core_0_iram0_recording_addr_0
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_0_IRAM0_RECORDING_ADDR_0_OFFSET: u32 = 0;
    pub const CORE_0_IRAM0_RECORDING_ADDR_0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_0_IRAM0_RECORDING_LOADSTORE_0** `[25]` (RO)
    ///
    /// reg_core_0_iram0_recording_loadstore_0
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_LOADSTORE_0_MASK) != 0
    }

    /// **CORE_0_IRAM0_RECORDING_WR_0** `[24]` (RO)
    ///
    /// reg_core_0_iram0_recording_wr_0
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_WR_0_MASK) != 0
    }

    /// **CORE_0_IRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
    ///
    /// reg_core_0_iram0_recording_addr_0
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_ADDR_0_MASK)
            >> Self::CORE_0_IRAM0_RECORDING_ADDR_0_OFFSET
    }
}

/// CORE_0_IRAM0_EXCEPTION_MONITOR_0 value type for batch read/modify/write operations
///
/// Use `core0iram0exceptionmonitor0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0iram0exceptionmonitor0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Iram0ExceptionMonitor0Val {
    bits: u32,
}

impl Core0Iram0ExceptionMonitor0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_IRAM0_RECORDING_LOADSTORE_0** `[25:25]`
    ///
    /// reg_core_0_iram0_recording_loadstore_0
    #[inline(always)]
    pub const fn core_0_iram0_recording_loadstore_0(&self) -> bool {
        (self.bits & Core0Iram0ExceptionMonitor0::CORE_0_IRAM0_RECORDING_LOADSTORE_0_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_RECORDING_WR_0** `[24:24]`
    ///
    /// reg_core_0_iram0_recording_wr_0
    #[inline(always)]
    pub const fn core_0_iram0_recording_wr_0(&self) -> bool {
        (self.bits & Core0Iram0ExceptionMonitor0::CORE_0_IRAM0_RECORDING_WR_0_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_RECORDING_ADDR_0** `[23:0]`
    ///
    /// reg_core_0_iram0_recording_addr_0
    #[inline(always)]
    pub const fn core_0_iram0_recording_addr_0(&self) -> u32 {
        (self.bits & Core0Iram0ExceptionMonitor0::CORE_0_IRAM0_RECORDING_ADDR_0_MASK)
            >> Core0Iram0ExceptionMonitor0::CORE_0_IRAM0_RECORDING_ADDR_0_OFFSET
    }
}

impl Default for Core0Iram0ExceptionMonitor0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Iram0ExceptionMonitor0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Iram0ExceptionMonitor0Val {
        Core0Iram0ExceptionMonitor0Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_IRAM0_EXCEPTION_MONITOR_1 Register
///
/// exception monitor status register1
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **CORE_0_IRAM0_RECORDING_LOADSTORE_1** `[25]` (RO)
///
///   reg_core_0_iram0_recording_loadstore_1
///
/// - **CORE_0_IRAM0_RECORDING_WR_1** `[24]` (RO)
///
///   reg_core_0_iram0_recording_wr_1
///
/// - **CORE_0_IRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
///
///   reg_core_0_iram0_recording_addr_1
#[repr(transparent)]
pub struct Core0Iram0ExceptionMonitor1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Iram0ExceptionMonitor1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Iram0ExceptionMonitor1 {
    // ======== Field Constants ========

    /// **CORE_0_IRAM0_RECORDING_LOADSTORE_1**
    ///
    /// reg_core_0_iram0_recording_loadstore_1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_1_OFFSET: u32 = 25;
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_1_MASK: u32 = 0x02000000;
    pub const CORE_0_IRAM0_RECORDING_LOADSTORE_1: u32 = 1 << 25;

    /// **CORE_0_IRAM0_RECORDING_WR_1**
    ///
    /// reg_core_0_iram0_recording_wr_1
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CORE_0_IRAM0_RECORDING_WR_1_OFFSET: u32 = 24;
    pub const CORE_0_IRAM0_RECORDING_WR_1_MASK: u32 = 0x01000000;
    pub const CORE_0_IRAM0_RECORDING_WR_1: u32 = 1 << 24;

    /// **CORE_0_IRAM0_RECORDING_ADDR_1**
    ///
    /// reg_core_0_iram0_recording_addr_1
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_0_IRAM0_RECORDING_ADDR_1_OFFSET: u32 = 0;
    pub const CORE_0_IRAM0_RECORDING_ADDR_1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_0_IRAM0_RECORDING_LOADSTORE_1** `[25]` (RO)
    ///
    /// reg_core_0_iram0_recording_loadstore_1
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_LOADSTORE_1_MASK) != 0
    }

    /// **CORE_0_IRAM0_RECORDING_WR_1** `[24]` (RO)
    ///
    /// reg_core_0_iram0_recording_wr_1
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_WR_1_MASK) != 0
    }

    /// **CORE_0_IRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
    ///
    /// reg_core_0_iram0_recording_addr_1
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_IRAM0_RECORDING_ADDR_1_MASK)
            >> Self::CORE_0_IRAM0_RECORDING_ADDR_1_OFFSET
    }
}

/// CORE_0_IRAM0_EXCEPTION_MONITOR_1 value type for batch read/modify/write operations
///
/// Use `core0iram0exceptionmonitor1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0iram0exceptionmonitor1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Iram0ExceptionMonitor1Val {
    bits: u32,
}

impl Core0Iram0ExceptionMonitor1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_IRAM0_RECORDING_LOADSTORE_1** `[25:25]`
    ///
    /// reg_core_0_iram0_recording_loadstore_1
    #[inline(always)]
    pub const fn core_0_iram0_recording_loadstore_1(&self) -> bool {
        (self.bits & Core0Iram0ExceptionMonitor1::CORE_0_IRAM0_RECORDING_LOADSTORE_1_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_RECORDING_WR_1** `[24:24]`
    ///
    /// reg_core_0_iram0_recording_wr_1
    #[inline(always)]
    pub const fn core_0_iram0_recording_wr_1(&self) -> bool {
        (self.bits & Core0Iram0ExceptionMonitor1::CORE_0_IRAM0_RECORDING_WR_1_MASK) != 0
    }

    /// Read **CORE_0_IRAM0_RECORDING_ADDR_1** `[23:0]`
    ///
    /// reg_core_0_iram0_recording_addr_1
    #[inline(always)]
    pub const fn core_0_iram0_recording_addr_1(&self) -> u32 {
        (self.bits & Core0Iram0ExceptionMonitor1::CORE_0_IRAM0_RECORDING_ADDR_1_MASK)
            >> Core0Iram0ExceptionMonitor1::CORE_0_IRAM0_RECORDING_ADDR_1_OFFSET
    }
}

impl Default for Core0Iram0ExceptionMonitor1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Iram0ExceptionMonitor1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Iram0ExceptionMonitor1Val {
        Core0Iram0ExceptionMonitor1Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_0 Register
///
/// exception monitor status register2
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_BYTEEN_0** `[16:1]` (RO)
///
///   reg_core_0_dram0_recording_byteen_0
///
/// - **CORE_0_DRAM0_RECORDING_WR_0** `[0]` (RO)
///
///   reg_core_0_dram0_recording_wr_0
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor0 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_BYTEEN_0**
    ///
    /// reg_core_0_dram0_recording_byteen_0
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const CORE_0_DRAM0_RECORDING_BYTEEN_0_OFFSET: u32 = 1;
    pub const CORE_0_DRAM0_RECORDING_BYTEEN_0_MASK: u32 = 0x0001fffe;

    /// **CORE_0_DRAM0_RECORDING_WR_0**
    ///
    /// reg_core_0_dram0_recording_wr_0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_DRAM0_RECORDING_WR_0_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_WR_0_MASK: u32 = 0x00000001;
    pub const CORE_0_DRAM0_RECORDING_WR_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_BYTEEN_0** `[16:1]` (RO)
    ///
    /// reg_core_0_dram0_recording_byteen_0
    #[inline(always)]
    pub fn core_0_dram0_recording_byteen_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_BYTEEN_0_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_BYTEEN_0_OFFSET
    }

    /// **CORE_0_DRAM0_RECORDING_WR_0** `[0]` (RO)
    ///
    /// reg_core_0_dram0_recording_wr_0
    #[inline(always)]
    pub fn core_0_dram0_recording_wr_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_WR_0_MASK) != 0
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_0 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor0Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_BYTEEN_0** `[16:1]`
    ///
    /// reg_core_0_dram0_recording_byteen_0
    #[inline(always)]
    pub const fn core_0_dram0_recording_byteen_0(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor0::CORE_0_DRAM0_RECORDING_BYTEEN_0_MASK)
            >> Core0Dram0ExceptionMonitor0::CORE_0_DRAM0_RECORDING_BYTEEN_0_OFFSET
    }

    /// Read **CORE_0_DRAM0_RECORDING_WR_0** `[0:0]`
    ///
    /// reg_core_0_dram0_recording_wr_0
    #[inline(always)]
    pub const fn core_0_dram0_recording_wr_0(&self) -> bool {
        (self.bits & Core0Dram0ExceptionMonitor0::CORE_0_DRAM0_RECORDING_WR_0_MASK) != 0
    }
}

impl Default for Core0Dram0ExceptionMonitor0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor0Val {
        Core0Dram0ExceptionMonitor0Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_1 Register
///
/// exception monitor status register3
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
///
///   reg_core_0_dram0_recording_addr_0
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor1 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_ADDR_0**
    ///
    /// reg_core_0_dram0_recording_addr_0
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_0_DRAM0_RECORDING_ADDR_0_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_ADDR_0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
    ///
    /// reg_core_0_dram0_recording_addr_0
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_ADDR_0_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_ADDR_0_OFFSET
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_1 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor1Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_ADDR_0** `[23:0]`
    ///
    /// reg_core_0_dram0_recording_addr_0
    #[inline(always)]
    pub const fn core_0_dram0_recording_addr_0(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor1::CORE_0_DRAM0_RECORDING_ADDR_0_MASK)
            >> Core0Dram0ExceptionMonitor1::CORE_0_DRAM0_RECORDING_ADDR_0_OFFSET
    }
}

impl Default for Core0Dram0ExceptionMonitor1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor1Val {
        Core0Dram0ExceptionMonitor1Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_2 Register
///
/// exception monitor status register4
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_PC_0** `[31:0]` (RO)
///
///   reg_core_0_dram0_recording_pc_0
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor2 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_PC_0**
    ///
    /// reg_core_0_dram0_recording_pc_0
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_DRAM0_RECORDING_PC_0_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_PC_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_PC_0** `[31:0]` (RO)
    ///
    /// reg_core_0_dram0_recording_pc_0
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_PC_0_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_PC_0_OFFSET
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_2 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor2Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_PC_0** `[31:0]`
    ///
    /// reg_core_0_dram0_recording_pc_0
    #[inline(always)]
    pub const fn core_0_dram0_recording_pc_0(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor2::CORE_0_DRAM0_RECORDING_PC_0_MASK)
            >> Core0Dram0ExceptionMonitor2::CORE_0_DRAM0_RECORDING_PC_0_OFFSET
    }
}

impl Default for Core0Dram0ExceptionMonitor2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor2Val {
        Core0Dram0ExceptionMonitor2Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_3 Register
///
/// exception monitor status register5
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_BYTEEN_1** `[16:1]` (RO)
///
///   reg_core_0_dram0_recording_byteen_1
///
/// - **CORE_0_DRAM0_RECORDING_WR_1** `[0]` (RO)
///
///   reg_core_0_dram0_recording_wr_1
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor3 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_BYTEEN_1**
    ///
    /// reg_core_0_dram0_recording_byteen_1
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const CORE_0_DRAM0_RECORDING_BYTEEN_1_OFFSET: u32 = 1;
    pub const CORE_0_DRAM0_RECORDING_BYTEEN_1_MASK: u32 = 0x0001fffe;

    /// **CORE_0_DRAM0_RECORDING_WR_1**
    ///
    /// reg_core_0_dram0_recording_wr_1
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_DRAM0_RECORDING_WR_1_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_WR_1_MASK: u32 = 0x00000001;
    pub const CORE_0_DRAM0_RECORDING_WR_1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_BYTEEN_1** `[16:1]` (RO)
    ///
    /// reg_core_0_dram0_recording_byteen_1
    #[inline(always)]
    pub fn core_0_dram0_recording_byteen_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_BYTEEN_1_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_BYTEEN_1_OFFSET
    }

    /// **CORE_0_DRAM0_RECORDING_WR_1** `[0]` (RO)
    ///
    /// reg_core_0_dram0_recording_wr_1
    #[inline(always)]
    pub fn core_0_dram0_recording_wr_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_WR_1_MASK) != 0
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_3 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor3Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_BYTEEN_1** `[16:1]`
    ///
    /// reg_core_0_dram0_recording_byteen_1
    #[inline(always)]
    pub const fn core_0_dram0_recording_byteen_1(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor3::CORE_0_DRAM0_RECORDING_BYTEEN_1_MASK)
            >> Core0Dram0ExceptionMonitor3::CORE_0_DRAM0_RECORDING_BYTEEN_1_OFFSET
    }

    /// Read **CORE_0_DRAM0_RECORDING_WR_1** `[0:0]`
    ///
    /// reg_core_0_dram0_recording_wr_1
    #[inline(always)]
    pub const fn core_0_dram0_recording_wr_1(&self) -> bool {
        (self.bits & Core0Dram0ExceptionMonitor3::CORE_0_DRAM0_RECORDING_WR_1_MASK) != 0
    }
}

impl Default for Core0Dram0ExceptionMonitor3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor3Val {
        Core0Dram0ExceptionMonitor3Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_4 Register
///
/// exception monitor status register6
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
///
///   reg_core_0_dram0_recording_addr_1
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor4 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_ADDR_1**
    ///
    /// reg_core_0_dram0_recording_addr_1
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_0_DRAM0_RECORDING_ADDR_1_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_ADDR_1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
    ///
    /// reg_core_0_dram0_recording_addr_1
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_ADDR_1_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_ADDR_1_OFFSET
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_4 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor4Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_ADDR_1** `[23:0]`
    ///
    /// reg_core_0_dram0_recording_addr_1
    #[inline(always)]
    pub const fn core_0_dram0_recording_addr_1(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor4::CORE_0_DRAM0_RECORDING_ADDR_1_MASK)
            >> Core0Dram0ExceptionMonitor4::CORE_0_DRAM0_RECORDING_ADDR_1_OFFSET
    }
}

impl Default for Core0Dram0ExceptionMonitor4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor4Val {
        Core0Dram0ExceptionMonitor4Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_5 Register
///
/// exception monitor status register7
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **CORE_0_DRAM0_RECORDING_PC_1** `[31:0]` (RO)
///
///   reg_core_0_dram0_recording_pc_1
#[repr(transparent)]
pub struct Core0Dram0ExceptionMonitor5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0Dram0ExceptionMonitor5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0Dram0ExceptionMonitor5 {
    // ======== Field Constants ========

    /// **CORE_0_DRAM0_RECORDING_PC_1**
    ///
    /// reg_core_0_dram0_recording_pc_1
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_DRAM0_RECORDING_PC_1_OFFSET: u32 = 0;
    pub const CORE_0_DRAM0_RECORDING_PC_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_DRAM0_RECORDING_PC_1** `[31:0]` (RO)
    ///
    /// reg_core_0_dram0_recording_pc_1
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_DRAM0_RECORDING_PC_1_MASK)
            >> Self::CORE_0_DRAM0_RECORDING_PC_1_OFFSET
    }
}

/// CORE_0_DRAM0_EXCEPTION_MONITOR_5 value type for batch read/modify/write operations
///
/// Use `core0dram0exceptionmonitor5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0dram0exceptionmonitor5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0Dram0ExceptionMonitor5Val {
    bits: u32,
}

impl Core0Dram0ExceptionMonitor5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DRAM0_RECORDING_PC_1** `[31:0]`
    ///
    /// reg_core_0_dram0_recording_pc_1
    #[inline(always)]
    pub const fn core_0_dram0_recording_pc_1(&self) -> u32 {
        (self.bits & Core0Dram0ExceptionMonitor5::CORE_0_DRAM0_RECORDING_PC_1_MASK)
            >> Core0Dram0ExceptionMonitor5::CORE_0_DRAM0_RECORDING_PC_1_OFFSET
    }
}

impl Default for Core0Dram0ExceptionMonitor5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0Dram0ExceptionMonitor5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0Dram0ExceptionMonitor5Val {
        Core0Dram0ExceptionMonitor5Val::from_bits(self.value.read_bits())
    }
}

/// CORE_0_LASTPC_BEFORE_EXCEPTION Register
///
/// cpu status register
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **CORE_0_LASTPC_BEFORE_EXC** `[31:0]` (RO)
///
///   cpu's lastpc before exception
#[repr(transparent)]
pub struct Core0LastpcBeforeException {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0LastpcBeforeException {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0LastpcBeforeException {
    // ======== Field Constants ========

    /// **CORE_0_LASTPC_BEFORE_EXC**
    ///
    /// cpu's lastpc before exception
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_0_LASTPC_BEFORE_EXC_OFFSET: u32 = 0;
    pub const CORE_0_LASTPC_BEFORE_EXC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_0_LASTPC_BEFORE_EXC** `[31:0]` (RO)
    ///
    /// cpu's lastpc before exception
    #[inline(always)]
    pub fn core_0_lastpc_before_exc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_0_LASTPC_BEFORE_EXC_MASK)
            >> Self::CORE_0_LASTPC_BEFORE_EXC_OFFSET
    }
}

/// CORE_0_LASTPC_BEFORE_EXCEPTION value type for batch read/modify/write operations
///
/// Use `core0lastpcbeforeexception.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0lastpcbeforeexception.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0LastpcBeforeExceptionVal {
    bits: u32,
}

impl Core0LastpcBeforeExceptionVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_LASTPC_BEFORE_EXC** `[31:0]`
    ///
    /// cpu's lastpc before exception
    #[inline(always)]
    pub const fn core_0_lastpc_before_exc(&self) -> u32 {
        (self.bits & Core0LastpcBeforeException::CORE_0_LASTPC_BEFORE_EXC_MASK)
            >> Core0LastpcBeforeException::CORE_0_LASTPC_BEFORE_EXC_OFFSET
    }
}

impl Default for Core0LastpcBeforeExceptionVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0LastpcBeforeException {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0LastpcBeforeExceptionVal {
        Core0LastpcBeforeExceptionVal::from_bits(self.value.read_bits())
    }
}

/// CORE_0_DEBUG_MODE Register
///
/// cpu status register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **CORE_0_DEBUG_MODULE_ACTIVE** `[1]` (RO)
///
///   cpu debug_module active status
///
/// - **CORE_0_DEBUG_MODE** `[0]` (RO)
///
///   cpu debug mode status, 1 means cpu enter debug mode.
#[repr(transparent)]
pub struct Core0DebugMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0DebugMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0DebugMode {
    // ======== Field Constants ========

    /// **CORE_0_DEBUG_MODULE_ACTIVE**
    ///
    /// cpu debug_module active status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_0_DEBUG_MODULE_ACTIVE_OFFSET: u32 = 1;
    pub const CORE_0_DEBUG_MODULE_ACTIVE_MASK: u32 = 0x00000002;
    pub const CORE_0_DEBUG_MODULE_ACTIVE: u32 = 1 << 1;

    /// **CORE_0_DEBUG_MODE**
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_0_DEBUG_MODE_OFFSET: u32 = 0;
    pub const CORE_0_DEBUG_MODE_MASK: u32 = 0x00000001;
    pub const CORE_0_DEBUG_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_0_DEBUG_MODULE_ACTIVE** `[1]` (RO)
    ///
    /// cpu debug_module active status
    #[inline(always)]
    pub fn core_0_debug_module_active(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DEBUG_MODULE_ACTIVE_MASK) != 0
    }

    /// **CORE_0_DEBUG_MODE** `[0]` (RO)
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    #[inline(always)]
    pub fn core_0_debug_mode(&self) -> bool {
        (self.value.read_bits() & Self::CORE_0_DEBUG_MODE_MASK) != 0
    }
}

/// CORE_0_DEBUG_MODE value type for batch read/modify/write operations
///
/// Use `core0debugmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0debugmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0DebugModeVal {
    bits: u32,
}

impl Core0DebugModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_0_DEBUG_MODULE_ACTIVE** `[1:1]`
    ///
    /// cpu debug_module active status
    #[inline(always)]
    pub const fn core_0_debug_module_active(&self) -> bool {
        (self.bits & Core0DebugMode::CORE_0_DEBUG_MODULE_ACTIVE_MASK) != 0
    }

    /// Read **CORE_0_DEBUG_MODE** `[0:0]`
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    #[inline(always)]
    pub const fn core_0_debug_mode(&self) -> bool {
        (self.bits & Core0DebugMode::CORE_0_DEBUG_MODE_MASK) != 0
    }
}

impl Default for Core0DebugModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0DebugMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0DebugModeVal {
        Core0DebugModeVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_INTR_ENA Register
///
/// core1 monitor enable configuration register
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **CORE_1_DRAM0_EXCEPTION_MONITOR_ENA** `[11]` (RW)
///
///   DBUS busy monitor enbale
///
/// - **CORE_1_IRAM0_EXCEPTION_MONITOR_ENA** `[10]` (RW)
///
///   IBUS busy monitor enable
///
/// - **CORE_1_SP_SPILL_MAX_ENA** `[9]` (RW)
///
///   Core1 stackpoint overflow monitor enable
///
/// - **CORE_1_SP_SPILL_MIN_ENA** `[8]` (RW)
///
///   Core1 stackpoint underflow monitor enable
///
/// - **CORE_1_AREA_PIF_1_WR_ENA** `[7]` (RW)
///
///   Core1 PIF area1 write monitor enable
///
/// - **CORE_1_AREA_PIF_1_RD_ENA** `[6]` (RW)
///
///   Core1 PIF area1 read monitor enable
///
/// - **CORE_1_AREA_PIF_0_WR_ENA** `[5]` (RW)
///
///   Core1 PIF area0 write monitor enable
///
/// - **CORE_1_AREA_PIF_0_RD_ENA** `[4]` (RW)
///
///   Core1 PIF area0 read monitor enable
///
/// - **CORE_1_AREA_DRAM0_1_WR_ENA** `[3]` (RW)
///
///   Core1 dram0 area1 write monitor enable
///
/// - **CORE_1_AREA_DRAM0_1_RD_ENA** `[2]` (RW)
///
///   Core1 dram0 area1 read monitor enable
///
/// - **CORE_1_AREA_DRAM0_0_WR_ENA** `[1]` (RW)
///
///   Core1 dram0 area0 write monitor enable
///
/// - **CORE_1_AREA_DRAM0_0_RD_ENA** `[0]` (RW)
///
///   Core1 dram0 area0 read monitor enable
#[repr(transparent)]
pub struct Core1IntrEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1IntrEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1IntrEna {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_ENA**
    ///
    /// DBUS busy monitor enbale
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_OFFSET: u32 = 11;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK: u32 = 0x00000800;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_ENA: u32 = 1 << 11;

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_ENA**
    ///
    /// IBUS busy monitor enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_OFFSET: u32 = 10;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK: u32 = 0x00000400;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_ENA: u32 = 1 << 10;

    /// **CORE_1_SP_SPILL_MAX_ENA**
    ///
    /// Core1 stackpoint overflow monitor enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_1_SP_SPILL_MAX_ENA_OFFSET: u32 = 9;
    pub const CORE_1_SP_SPILL_MAX_ENA_MASK: u32 = 0x00000200;
    pub const CORE_1_SP_SPILL_MAX_ENA: u32 = 1 << 9;

    /// **CORE_1_SP_SPILL_MIN_ENA**
    ///
    /// Core1 stackpoint underflow monitor enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_1_SP_SPILL_MIN_ENA_OFFSET: u32 = 8;
    pub const CORE_1_SP_SPILL_MIN_ENA_MASK: u32 = 0x00000100;
    pub const CORE_1_SP_SPILL_MIN_ENA: u32 = 1 << 8;

    /// **CORE_1_AREA_PIF_1_WR_ENA**
    ///
    /// Core1 PIF area1 write monitor enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_1_AREA_PIF_1_WR_ENA_OFFSET: u32 = 7;
    pub const CORE_1_AREA_PIF_1_WR_ENA_MASK: u32 = 0x00000080;
    pub const CORE_1_AREA_PIF_1_WR_ENA: u32 = 1 << 7;

    /// **CORE_1_AREA_PIF_1_RD_ENA**
    ///
    /// Core1 PIF area1 read monitor enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_1_AREA_PIF_1_RD_ENA_OFFSET: u32 = 6;
    pub const CORE_1_AREA_PIF_1_RD_ENA_MASK: u32 = 0x00000040;
    pub const CORE_1_AREA_PIF_1_RD_ENA: u32 = 1 << 6;

    /// **CORE_1_AREA_PIF_0_WR_ENA**
    ///
    /// Core1 PIF area0 write monitor enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_1_AREA_PIF_0_WR_ENA_OFFSET: u32 = 5;
    pub const CORE_1_AREA_PIF_0_WR_ENA_MASK: u32 = 0x00000020;
    pub const CORE_1_AREA_PIF_0_WR_ENA: u32 = 1 << 5;

    /// **CORE_1_AREA_PIF_0_RD_ENA**
    ///
    /// Core1 PIF area0 read monitor enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_1_AREA_PIF_0_RD_ENA_OFFSET: u32 = 4;
    pub const CORE_1_AREA_PIF_0_RD_ENA_MASK: u32 = 0x00000010;
    pub const CORE_1_AREA_PIF_0_RD_ENA: u32 = 1 << 4;

    /// **CORE_1_AREA_DRAM0_1_WR_ENA**
    ///
    /// Core1 dram0 area1 write monitor enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_1_AREA_DRAM0_1_WR_ENA_OFFSET: u32 = 3;
    pub const CORE_1_AREA_DRAM0_1_WR_ENA_MASK: u32 = 0x00000008;
    pub const CORE_1_AREA_DRAM0_1_WR_ENA: u32 = 1 << 3;

    /// **CORE_1_AREA_DRAM0_1_RD_ENA**
    ///
    /// Core1 dram0 area1 read monitor enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_1_AREA_DRAM0_1_RD_ENA_OFFSET: u32 = 2;
    pub const CORE_1_AREA_DRAM0_1_RD_ENA_MASK: u32 = 0x00000004;
    pub const CORE_1_AREA_DRAM0_1_RD_ENA: u32 = 1 << 2;

    /// **CORE_1_AREA_DRAM0_0_WR_ENA**
    ///
    /// Core1 dram0 area0 write monitor enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_AREA_DRAM0_0_WR_ENA_OFFSET: u32 = 1;
    pub const CORE_1_AREA_DRAM0_0_WR_ENA_MASK: u32 = 0x00000002;
    pub const CORE_1_AREA_DRAM0_0_WR_ENA: u32 = 1 << 1;

    /// **CORE_1_AREA_DRAM0_0_RD_ENA**
    ///
    /// Core1 dram0 area0 read monitor enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_AREA_DRAM0_0_RD_ENA_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_RD_ENA_MASK: u32 = 0x00000001;
    pub const CORE_1_AREA_DRAM0_0_RD_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_ENA** `[11]` (RW)
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Set CORE_1_DRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_dram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// Clear CORE_1_DRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_dram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_ENA** `[10]` (RW)
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Set CORE_1_IRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_iram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// Clear CORE_1_IRAM0_EXCEPTION_MONITOR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_iram0_exception_monitor_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK);
    }

    /// **CORE_1_SP_SPILL_MAX_ENA** `[9]` (RW)
    ///
    /// Core1 stackpoint overflow monitor enable
    #[inline(always)]
    pub fn core_1_sp_spill_max_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MAX_ENA_MASK) != 0
    }

    /// Set CORE_1_SP_SPILL_MAX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_sp_spill_max_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_SP_SPILL_MAX_ENA_MASK);
    }

    /// Clear CORE_1_SP_SPILL_MAX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_sp_spill_max_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_SP_SPILL_MAX_ENA_MASK);
    }

    /// **CORE_1_SP_SPILL_MIN_ENA** `[8]` (RW)
    ///
    /// Core1 stackpoint underflow monitor enable
    #[inline(always)]
    pub fn core_1_sp_spill_min_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MIN_ENA_MASK) != 0
    }

    /// Set CORE_1_SP_SPILL_MIN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_sp_spill_min_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_SP_SPILL_MIN_ENA_MASK);
    }

    /// Clear CORE_1_SP_SPILL_MIN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_sp_spill_min_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_SP_SPILL_MIN_ENA_MASK);
    }

    /// **CORE_1_AREA_PIF_1_WR_ENA** `[7]` (RW)
    ///
    /// Core1 PIF area1 write monitor enable
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_WR_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_1_WR_ENA_MASK);
    }

    /// Clear CORE_1_AREA_PIF_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_1_WR_ENA_MASK);
    }

    /// **CORE_1_AREA_PIF_1_RD_ENA** `[6]` (RW)
    ///
    /// Core1 PIF area1 read monitor enable
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_RD_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_1_RD_ENA_MASK);
    }

    /// Clear CORE_1_AREA_PIF_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_1_RD_ENA_MASK);
    }

    /// **CORE_1_AREA_PIF_0_WR_ENA** `[5]` (RW)
    ///
    /// Core1 PIF area0 write monitor enable
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_WR_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_0_WR_ENA_MASK);
    }

    /// Clear CORE_1_AREA_PIF_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_0_WR_ENA_MASK);
    }

    /// **CORE_1_AREA_PIF_0_RD_ENA** `[4]` (RW)
    ///
    /// Core1 PIF area0 read monitor enable
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_RD_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_0_RD_ENA_MASK);
    }

    /// Clear CORE_1_AREA_PIF_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_0_RD_ENA_MASK);
    }

    /// **CORE_1_AREA_DRAM0_1_WR_ENA** `[3]` (RW)
    ///
    /// Core1 dram0 area1 write monitor enable
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_WR_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_1_WR_ENA_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_1_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_1_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_1_WR_ENA_MASK);
    }

    /// **CORE_1_AREA_DRAM0_1_RD_ENA** `[2]` (RW)
    ///
    /// Core1 dram0 area1 read monitor enable
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_RD_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_1_RD_ENA_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_1_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_1_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_1_RD_ENA_MASK);
    }

    /// **CORE_1_AREA_DRAM0_0_WR_ENA** `[1]` (RW)
    ///
    /// Core1 dram0 area0 write monitor enable
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_WR_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_0_WR_ENA_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_0_WR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_0_wr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_0_WR_ENA_MASK);
    }

    /// **CORE_1_AREA_DRAM0_0_RD_ENA** `[0]` (RW)
    ///
    /// Core1 dram0 area0 read monitor enable
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_RD_ENA_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_0_RD_ENA_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_0_RD_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_0_rd_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_0_RD_ENA_MASK);
    }
}

/// CORE_1_INTR_ENA value type for batch read/modify/write operations
///
/// Use `core1intrena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1intrena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1IntrEnaVal {
    bits: u32,
}

impl Core1IntrEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_EXCEPTION_MONITOR_ENA** `[11:11]`
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_EXCEPTION_MONITOR_ENA** `[10:10]`
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub const fn core_1_iram0_exception_monitor_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MAX_ENA** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor enable
    #[inline(always)]
    pub const fn core_1_sp_spill_max_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_SP_SPILL_MAX_ENA_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MIN_ENA** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor enable
    #[inline(always)]
    pub const fn core_1_sp_spill_min_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_SP_SPILL_MIN_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_WR_ENA** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor enable
    #[inline(always)]
    pub const fn core_1_area_pif_1_wr_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_PIF_1_WR_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_RD_ENA** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor enable
    #[inline(always)]
    pub const fn core_1_area_pif_1_rd_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_PIF_1_RD_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_WR_ENA** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor enable
    #[inline(always)]
    pub const fn core_1_area_pif_0_wr_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_PIF_0_WR_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_RD_ENA** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor enable
    #[inline(always)]
    pub const fn core_1_area_pif_0_rd_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_PIF_0_RD_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_WR_ENA** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor enable
    #[inline(always)]
    pub const fn core_1_area_dram0_1_wr_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_DRAM0_1_WR_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_RD_ENA** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor enable
    #[inline(always)]
    pub const fn core_1_area_dram0_1_rd_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_DRAM0_1_RD_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_WR_ENA** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor enable
    #[inline(always)]
    pub const fn core_1_area_dram0_0_wr_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_DRAM0_0_WR_ENA_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_RD_ENA** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor enable
    #[inline(always)]
    pub const fn core_1_area_dram0_0_rd_ena(&self) -> bool {
        (self.bits & Core1IntrEna::CORE_1_AREA_DRAM0_0_RD_ENA_MASK) != 0
    }

    /// Set **CORE_1_DRAM0_EXCEPTION_MONITOR_ENA** `[11:11]`
    ///
    /// DBUS busy monitor enbale
    #[inline(always)]
    pub const fn with_core_1_dram0_exception_monitor_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_DRAM0_EXCEPTION_MONITOR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_IRAM0_EXCEPTION_MONITOR_ENA** `[10:10]`
    ///
    /// IBUS busy monitor enable
    #[inline(always)]
    pub const fn with_core_1_iram0_exception_monitor_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_IRAM0_EXCEPTION_MONITOR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MAX_ENA** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor enable
    #[inline(always)]
    pub const fn with_core_1_sp_spill_max_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_SP_SPILL_MAX_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_SP_SPILL_MAX_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MIN_ENA** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor enable
    #[inline(always)]
    pub const fn with_core_1_sp_spill_min_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_SP_SPILL_MIN_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_SP_SPILL_MIN_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_WR_ENA** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_PIF_1_WR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_PIF_1_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_RD_ENA** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_PIF_1_RD_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_PIF_1_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_WR_ENA** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_PIF_0_WR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_PIF_0_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_RD_ENA** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_PIF_0_RD_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_PIF_0_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_WR_ENA** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_DRAM0_1_WR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_DRAM0_1_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_RD_ENA** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_DRAM0_1_RD_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_DRAM0_1_RD_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_WR_ENA** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_wr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_DRAM0_0_WR_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_DRAM0_0_WR_ENA_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_RD_ENA** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_rd_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrEna::CORE_1_AREA_DRAM0_0_RD_ENA_MASK
            } else {
                self.bits & !Core1IntrEna::CORE_1_AREA_DRAM0_0_RD_ENA_MASK
            },
        }
    }
}

impl Default for Core1IntrEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1IntrEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1IntrEnaVal {
        Core1IntrEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1IntrEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1IntrEnaVal) -> Core1IntrEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_INTR_RAW Register
///
/// core1 monitor interrupt status register
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **CORE_1_DRAM0_EXCEPTION_MONITOR_RAW** `[11]` (RO)
///
///   DBUS busy monitor initerrupt status
///
/// - **CORE_1_IRAM0_EXCEPTION_MONITOR_RAW** `[10]` (RO)
///
///   IBUS busy monitor interrupt status
///
/// - **CORE_1_SP_SPILL_MAX_RAW** `[9]` (RO)
///
///   Core1 stackpoint overflow monitor interrupt status
///
/// - **CORE_1_SP_SPILL_MIN_RAW** `[8]` (RO)
///
///   Core1 stackpoint underflow monitor interrupt status
///
/// - **CORE_1_AREA_PIF_1_WR_RAW** `[7]` (RO)
///
///   Core1 PIF area1 write monitor interrupt status
///
/// - **CORE_1_AREA_PIF_1_RD_RAW** `[6]` (RO)
///
///   Core1 PIF area1 read monitor interrupt status
///
/// - **CORE_1_AREA_PIF_0_WR_RAW** `[5]` (RO)
///
///   Core1 PIF area0 write monitor interrupt status
///
/// - **CORE_1_AREA_PIF_0_RD_RAW** `[4]` (RO)
///
///   Core1 PIF area0 read monitor interrupt status
///
/// - **CORE_1_AREA_DRAM0_1_WR_RAW** `[3]` (RO)
///
///   Core1 dram0 area1 write monitor interrupt status
///
/// - **CORE_1_AREA_DRAM0_1_RD_RAW** `[2]` (RO)
///
///   Core1 dram0 area1 read monitor interrupt status
///
/// - **CORE_1_AREA_DRAM0_0_WR_RAW** `[1]` (RO)
///
///   Core1 dram0 area0 write monitor interrupt status
///
/// - **CORE_1_AREA_DRAM0_0_RD_RAW** `[0]` (RO)
///
///   Core1 dram0 area0 read monitor interrupt status
#[repr(transparent)]
pub struct Core1IntrRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1IntrRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1IntrRaw {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_RAW**
    ///
    /// DBUS busy monitor initerrupt status
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_OFFSET: u32 = 11;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_MASK: u32 = 0x00000800;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RAW: u32 = 1 << 11;

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_RAW**
    ///
    /// IBUS busy monitor interrupt status
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_OFFSET: u32 = 10;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_MASK: u32 = 0x00000400;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RAW: u32 = 1 << 10;

    /// **CORE_1_SP_SPILL_MAX_RAW**
    ///
    /// Core1 stackpoint overflow monitor interrupt status
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_1_SP_SPILL_MAX_RAW_OFFSET: u32 = 9;
    pub const CORE_1_SP_SPILL_MAX_RAW_MASK: u32 = 0x00000200;
    pub const CORE_1_SP_SPILL_MAX_RAW: u32 = 1 << 9;

    /// **CORE_1_SP_SPILL_MIN_RAW**
    ///
    /// Core1 stackpoint underflow monitor interrupt status
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_1_SP_SPILL_MIN_RAW_OFFSET: u32 = 8;
    pub const CORE_1_SP_SPILL_MIN_RAW_MASK: u32 = 0x00000100;
    pub const CORE_1_SP_SPILL_MIN_RAW: u32 = 1 << 8;

    /// **CORE_1_AREA_PIF_1_WR_RAW**
    ///
    /// Core1 PIF area1 write monitor interrupt status
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_1_AREA_PIF_1_WR_RAW_OFFSET: u32 = 7;
    pub const CORE_1_AREA_PIF_1_WR_RAW_MASK: u32 = 0x00000080;
    pub const CORE_1_AREA_PIF_1_WR_RAW: u32 = 1 << 7;

    /// **CORE_1_AREA_PIF_1_RD_RAW**
    ///
    /// Core1 PIF area1 read monitor interrupt status
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_1_AREA_PIF_1_RD_RAW_OFFSET: u32 = 6;
    pub const CORE_1_AREA_PIF_1_RD_RAW_MASK: u32 = 0x00000040;
    pub const CORE_1_AREA_PIF_1_RD_RAW: u32 = 1 << 6;

    /// **CORE_1_AREA_PIF_0_WR_RAW**
    ///
    /// Core1 PIF area0 write monitor interrupt status
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_1_AREA_PIF_0_WR_RAW_OFFSET: u32 = 5;
    pub const CORE_1_AREA_PIF_0_WR_RAW_MASK: u32 = 0x00000020;
    pub const CORE_1_AREA_PIF_0_WR_RAW: u32 = 1 << 5;

    /// **CORE_1_AREA_PIF_0_RD_RAW**
    ///
    /// Core1 PIF area0 read monitor interrupt status
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_1_AREA_PIF_0_RD_RAW_OFFSET: u32 = 4;
    pub const CORE_1_AREA_PIF_0_RD_RAW_MASK: u32 = 0x00000010;
    pub const CORE_1_AREA_PIF_0_RD_RAW: u32 = 1 << 4;

    /// **CORE_1_AREA_DRAM0_1_WR_RAW**
    ///
    /// Core1 dram0 area1 write monitor interrupt status
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_1_AREA_DRAM0_1_WR_RAW_OFFSET: u32 = 3;
    pub const CORE_1_AREA_DRAM0_1_WR_RAW_MASK: u32 = 0x00000008;
    pub const CORE_1_AREA_DRAM0_1_WR_RAW: u32 = 1 << 3;

    /// **CORE_1_AREA_DRAM0_1_RD_RAW**
    ///
    /// Core1 dram0 area1 read monitor interrupt status
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_1_AREA_DRAM0_1_RD_RAW_OFFSET: u32 = 2;
    pub const CORE_1_AREA_DRAM0_1_RD_RAW_MASK: u32 = 0x00000004;
    pub const CORE_1_AREA_DRAM0_1_RD_RAW: u32 = 1 << 2;

    /// **CORE_1_AREA_DRAM0_0_WR_RAW**
    ///
    /// Core1 dram0 area0 write monitor interrupt status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_AREA_DRAM0_0_WR_RAW_OFFSET: u32 = 1;
    pub const CORE_1_AREA_DRAM0_0_WR_RAW_MASK: u32 = 0x00000002;
    pub const CORE_1_AREA_DRAM0_0_WR_RAW: u32 = 1 << 1;

    /// **CORE_1_AREA_DRAM0_0_RD_RAW**
    ///
    /// Core1 dram0 area0 read monitor interrupt status
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_AREA_DRAM0_0_RD_RAW_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_RD_RAW_MASK: u32 = 0x00000001;
    pub const CORE_1_AREA_DRAM0_0_RD_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_RAW** `[11]` (RO)
    ///
    /// DBUS busy monitor initerrupt status
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_RAW** `[10]` (RO)
    ///
    /// IBUS busy monitor interrupt status
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// **CORE_1_SP_SPILL_MAX_RAW** `[9]` (RO)
    ///
    /// Core1 stackpoint overflow monitor interrupt status
    #[inline(always)]
    pub fn core_1_sp_spill_max_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MAX_RAW_MASK) != 0
    }

    /// **CORE_1_SP_SPILL_MIN_RAW** `[8]` (RO)
    ///
    /// Core1 stackpoint underflow monitor interrupt status
    #[inline(always)]
    pub fn core_1_sp_spill_min_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MIN_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_PIF_1_WR_RAW** `[7]` (RO)
    ///
    /// Core1 PIF area1 write monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_WR_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_PIF_1_RD_RAW** `[6]` (RO)
    ///
    /// Core1 PIF area1 read monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_RD_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_PIF_0_WR_RAW** `[5]` (RO)
    ///
    /// Core1 PIF area0 write monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_WR_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_PIF_0_RD_RAW** `[4]` (RO)
    ///
    /// Core1 PIF area0 read monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_RD_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_DRAM0_1_WR_RAW** `[3]` (RO)
    ///
    /// Core1 dram0 area1 write monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_WR_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_DRAM0_1_RD_RAW** `[2]` (RO)
    ///
    /// Core1 dram0 area1 read monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_RD_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_DRAM0_0_WR_RAW** `[1]` (RO)
    ///
    /// Core1 dram0 area0 write monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_WR_RAW_MASK) != 0
    }

    /// **CORE_1_AREA_DRAM0_0_RD_RAW** `[0]` (RO)
    ///
    /// Core1 dram0 area0 read monitor interrupt status
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_RD_RAW_MASK) != 0
    }
}

/// CORE_1_INTR_RAW value type for batch read/modify/write operations
///
/// Use `core1intrraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1intrraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1IntrRawVal {
    bits: u32,
}

impl Core1IntrRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_EXCEPTION_MONITOR_RAW** `[11:11]`
    ///
    /// DBUS busy monitor initerrupt status
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_EXCEPTION_MONITOR_RAW** `[10:10]`
    ///
    /// IBUS busy monitor interrupt status
    #[inline(always)]
    pub const fn core_1_iram0_exception_monitor_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MAX_RAW** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor interrupt status
    #[inline(always)]
    pub const fn core_1_sp_spill_max_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_SP_SPILL_MAX_RAW_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MIN_RAW** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor interrupt status
    #[inline(always)]
    pub const fn core_1_sp_spill_min_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_SP_SPILL_MIN_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_WR_RAW** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_pif_1_wr_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_PIF_1_WR_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_RD_RAW** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_pif_1_rd_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_PIF_1_RD_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_WR_RAW** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_pif_0_wr_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_PIF_0_WR_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_RD_RAW** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_pif_0_rd_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_PIF_0_RD_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_WR_RAW** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_dram0_1_wr_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_DRAM0_1_WR_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_RD_RAW** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_dram0_1_rd_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_DRAM0_1_RD_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_WR_RAW** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_dram0_0_wr_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_DRAM0_0_WR_RAW_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_RD_RAW** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor interrupt status
    #[inline(always)]
    pub const fn core_1_area_dram0_0_rd_raw(&self) -> bool {
        (self.bits & Core1IntrRaw::CORE_1_AREA_DRAM0_0_RD_RAW_MASK) != 0
    }
}

impl Default for Core1IntrRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1IntrRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1IntrRawVal {
        Core1IntrRawVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_INTR_RLS Register
///
/// core1 monitor interrupt enable register
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **CORE_1_DRAM0_EXCEPTION_MONITOR_RLS** `[11]` (RW)
///
///   DBUS busy monitor interrupt enbale
///
/// - **CORE_1_IRAM0_EXCEPTION_MONITOR_RLS** `[10]` (RW)
///
///   IBUS busy monitor interrupt enable
///
/// - **CORE_1_SP_SPILL_MAX_RLS** `[9]` (RW)
///
///   Core1 stackpoint overflow monitor interrupt enable
///
/// - **CORE_1_SP_SPILL_MIN_RLS** `[8]` (RW)
///
///   Core1 stackpoint underflow monitor interrupt enable
///
/// - **CORE_1_AREA_PIF_1_WR_RLS** `[7]` (RW)
///
///   Core1 PIF area1 write monitor interrupt enable
///
/// - **CORE_1_AREA_PIF_1_RD_RLS** `[6]` (RW)
///
///   Core1 PIF area1 read monitor interrupt enable
///
/// - **CORE_1_AREA_PIF_0_WR_RLS** `[5]` (RW)
///
///   Core1 PIF area0 write monitor interrupt enable
///
/// - **CORE_1_AREA_PIF_0_RD_RLS** `[4]` (RW)
///
///   Core1 PIF area0 read monitor interrupt enable
///
/// - **CORE_1_AREA_DRAM0_1_WR_RLS** `[3]` (RW)
///
///   Core1 dram0 area1 write monitor interrupt enable
///
/// - **CORE_1_AREA_DRAM0_1_RD_RLS** `[2]` (RW)
///
///   Core1 dram0 area1 read monitor interrupt enable
///
/// - **CORE_1_AREA_DRAM0_0_WR_RLS** `[1]` (RW)
///
///   Core1 dram0 area0 write monitor interrupt enable
///
/// - **CORE_1_AREA_DRAM0_0_RD_RLS** `[0]` (RW)
///
///   Core1 dram0 area0 read monitor interrupt enable
#[repr(transparent)]
pub struct Core1IntrRls {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1IntrRls {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1IntrRls {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_RLS**
    ///
    /// DBUS busy monitor interrupt enbale
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_OFFSET: u32 = 11;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK: u32 = 0x00000800;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_RLS: u32 = 1 << 11;

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_RLS**
    ///
    /// IBUS busy monitor interrupt enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_OFFSET: u32 = 10;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK: u32 = 0x00000400;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_RLS: u32 = 1 << 10;

    /// **CORE_1_SP_SPILL_MAX_RLS**
    ///
    /// Core1 stackpoint overflow monitor interrupt enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_1_SP_SPILL_MAX_RLS_OFFSET: u32 = 9;
    pub const CORE_1_SP_SPILL_MAX_RLS_MASK: u32 = 0x00000200;
    pub const CORE_1_SP_SPILL_MAX_RLS: u32 = 1 << 9;

    /// **CORE_1_SP_SPILL_MIN_RLS**
    ///
    /// Core1 stackpoint underflow monitor interrupt enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_1_SP_SPILL_MIN_RLS_OFFSET: u32 = 8;
    pub const CORE_1_SP_SPILL_MIN_RLS_MASK: u32 = 0x00000100;
    pub const CORE_1_SP_SPILL_MIN_RLS: u32 = 1 << 8;

    /// **CORE_1_AREA_PIF_1_WR_RLS**
    ///
    /// Core1 PIF area1 write monitor interrupt enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_1_AREA_PIF_1_WR_RLS_OFFSET: u32 = 7;
    pub const CORE_1_AREA_PIF_1_WR_RLS_MASK: u32 = 0x00000080;
    pub const CORE_1_AREA_PIF_1_WR_RLS: u32 = 1 << 7;

    /// **CORE_1_AREA_PIF_1_RD_RLS**
    ///
    /// Core1 PIF area1 read monitor interrupt enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_1_AREA_PIF_1_RD_RLS_OFFSET: u32 = 6;
    pub const CORE_1_AREA_PIF_1_RD_RLS_MASK: u32 = 0x00000040;
    pub const CORE_1_AREA_PIF_1_RD_RLS: u32 = 1 << 6;

    /// **CORE_1_AREA_PIF_0_WR_RLS**
    ///
    /// Core1 PIF area0 write monitor interrupt enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_1_AREA_PIF_0_WR_RLS_OFFSET: u32 = 5;
    pub const CORE_1_AREA_PIF_0_WR_RLS_MASK: u32 = 0x00000020;
    pub const CORE_1_AREA_PIF_0_WR_RLS: u32 = 1 << 5;

    /// **CORE_1_AREA_PIF_0_RD_RLS**
    ///
    /// Core1 PIF area0 read monitor interrupt enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_1_AREA_PIF_0_RD_RLS_OFFSET: u32 = 4;
    pub const CORE_1_AREA_PIF_0_RD_RLS_MASK: u32 = 0x00000010;
    pub const CORE_1_AREA_PIF_0_RD_RLS: u32 = 1 << 4;

    /// **CORE_1_AREA_DRAM0_1_WR_RLS**
    ///
    /// Core1 dram0 area1 write monitor interrupt enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_1_AREA_DRAM0_1_WR_RLS_OFFSET: u32 = 3;
    pub const CORE_1_AREA_DRAM0_1_WR_RLS_MASK: u32 = 0x00000008;
    pub const CORE_1_AREA_DRAM0_1_WR_RLS: u32 = 1 << 3;

    /// **CORE_1_AREA_DRAM0_1_RD_RLS**
    ///
    /// Core1 dram0 area1 read monitor interrupt enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_1_AREA_DRAM0_1_RD_RLS_OFFSET: u32 = 2;
    pub const CORE_1_AREA_DRAM0_1_RD_RLS_MASK: u32 = 0x00000004;
    pub const CORE_1_AREA_DRAM0_1_RD_RLS: u32 = 1 << 2;

    /// **CORE_1_AREA_DRAM0_0_WR_RLS**
    ///
    /// Core1 dram0 area0 write monitor interrupt enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_AREA_DRAM0_0_WR_RLS_OFFSET: u32 = 1;
    pub const CORE_1_AREA_DRAM0_0_WR_RLS_MASK: u32 = 0x00000002;
    pub const CORE_1_AREA_DRAM0_0_WR_RLS: u32 = 1 << 1;

    /// **CORE_1_AREA_DRAM0_0_RD_RLS**
    ///
    /// Core1 dram0 area0 read monitor interrupt enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_AREA_DRAM0_0_RD_RLS_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_RD_RLS_MASK: u32 = 0x00000001;
    pub const CORE_1_AREA_DRAM0_0_RD_RLS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_RLS** `[11]` (RW)
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Set CORE_1_DRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_dram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// Clear CORE_1_DRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_dram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_RLS** `[10]` (RW)
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Set CORE_1_IRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_iram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// Clear CORE_1_IRAM0_EXCEPTION_MONITOR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_iram0_exception_monitor_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK);
    }

    /// **CORE_1_SP_SPILL_MAX_RLS** `[9]` (RW)
    ///
    /// Core1 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub fn core_1_sp_spill_max_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MAX_RLS_MASK) != 0
    }

    /// Set CORE_1_SP_SPILL_MAX_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_sp_spill_max_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_SP_SPILL_MAX_RLS_MASK);
    }

    /// Clear CORE_1_SP_SPILL_MAX_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_sp_spill_max_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_SP_SPILL_MAX_RLS_MASK);
    }

    /// **CORE_1_SP_SPILL_MIN_RLS** `[8]` (RW)
    ///
    /// Core1 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub fn core_1_sp_spill_min_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_SP_SPILL_MIN_RLS_MASK) != 0
    }

    /// Set CORE_1_SP_SPILL_MIN_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_sp_spill_min_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_SP_SPILL_MIN_RLS_MASK);
    }

    /// Clear CORE_1_SP_SPILL_MIN_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_sp_spill_min_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_SP_SPILL_MIN_RLS_MASK);
    }

    /// **CORE_1_AREA_PIF_1_WR_RLS** `[7]` (RW)
    ///
    /// Core1 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_WR_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_1_WR_RLS_MASK);
    }

    /// Clear CORE_1_AREA_PIF_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_1_WR_RLS_MASK);
    }

    /// **CORE_1_AREA_PIF_1_RD_RLS** `[6]` (RW)
    ///
    /// Core1 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_RD_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_1_RD_RLS_MASK);
    }

    /// Clear CORE_1_AREA_PIF_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_1_RD_RLS_MASK);
    }

    /// **CORE_1_AREA_PIF_0_WR_RLS** `[5]` (RW)
    ///
    /// Core1 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_WR_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_0_WR_RLS_MASK);
    }

    /// Clear CORE_1_AREA_PIF_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_0_WR_RLS_MASK);
    }

    /// **CORE_1_AREA_PIF_0_RD_RLS** `[4]` (RW)
    ///
    /// Core1 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_RD_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_PIF_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_pif_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_PIF_0_RD_RLS_MASK);
    }

    /// Clear CORE_1_AREA_PIF_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_pif_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_PIF_0_RD_RLS_MASK);
    }

    /// **CORE_1_AREA_DRAM0_1_WR_RLS** `[3]` (RW)
    ///
    /// Core1 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_WR_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_1_WR_RLS_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_1_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_1_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_1_WR_RLS_MASK);
    }

    /// **CORE_1_AREA_DRAM0_1_RD_RLS** `[2]` (RW)
    ///
    /// Core1 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_RD_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_1_RD_RLS_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_1_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_1_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_1_RD_RLS_MASK);
    }

    /// **CORE_1_AREA_DRAM0_0_WR_RLS** `[1]` (RW)
    ///
    /// Core1 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_WR_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_0_WR_RLS_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_0_WR_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_0_wr_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_0_WR_RLS_MASK);
    }

    /// **CORE_1_AREA_DRAM0_0_RD_RLS** `[0]` (RW)
    ///
    /// Core1 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_rls(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_RD_RLS_MASK) != 0
    }

    /// Set CORE_1_AREA_DRAM0_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_area_dram0_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_AREA_DRAM0_0_RD_RLS_MASK);
    }

    /// Clear CORE_1_AREA_DRAM0_0_RD_RLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_area_dram0_0_rd_rls(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_AREA_DRAM0_0_RD_RLS_MASK);
    }
}

/// CORE_1_INTR_RLS value type for batch read/modify/write operations
///
/// Use `core1intrrls.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1intrrls.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1IntrRlsVal {
    bits: u32,
}

impl Core1IntrRlsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_EXCEPTION_MONITOR_RLS** `[11:11]`
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_EXCEPTION_MONITOR_RLS** `[10:10]`
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_iram0_exception_monitor_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MAX_RLS** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_sp_spill_max_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_SP_SPILL_MAX_RLS_MASK) != 0
    }

    /// Read **CORE_1_SP_SPILL_MIN_RLS** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_sp_spill_min_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_SP_SPILL_MIN_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_WR_RLS** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_pif_1_wr_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_PIF_1_WR_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_1_RD_RLS** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_pif_1_rd_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_PIF_1_RD_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_WR_RLS** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_pif_0_wr_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_PIF_0_WR_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_PIF_0_RD_RLS** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_pif_0_rd_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_PIF_0_RD_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_WR_RLS** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_dram0_1_wr_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_DRAM0_1_WR_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_1_RD_RLS** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_dram0_1_rd_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_DRAM0_1_RD_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_WR_RLS** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_dram0_0_wr_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_DRAM0_0_WR_RLS_MASK) != 0
    }

    /// Read **CORE_1_AREA_DRAM0_0_RD_RLS** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn core_1_area_dram0_0_rd_rls(&self) -> bool {
        (self.bits & Core1IntrRls::CORE_1_AREA_DRAM0_0_RD_RLS_MASK) != 0
    }

    /// Set **CORE_1_DRAM0_EXCEPTION_MONITOR_RLS** `[11:11]`
    ///
    /// DBUS busy monitor interrupt enbale
    #[inline(always)]
    pub const fn with_core_1_dram0_exception_monitor_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_DRAM0_EXCEPTION_MONITOR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_IRAM0_EXCEPTION_MONITOR_RLS** `[10:10]`
    ///
    /// IBUS busy monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_iram0_exception_monitor_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_IRAM0_EXCEPTION_MONITOR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MAX_RLS** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_sp_spill_max_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_SP_SPILL_MAX_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_SP_SPILL_MAX_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MIN_RLS** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_sp_spill_min_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_SP_SPILL_MIN_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_SP_SPILL_MIN_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_WR_RLS** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_PIF_1_WR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_PIF_1_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_RD_RLS** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_PIF_1_RD_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_PIF_1_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_WR_RLS** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_PIF_0_WR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_PIF_0_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_RD_RLS** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_PIF_0_RD_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_PIF_0_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_WR_RLS** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_DRAM0_1_WR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_DRAM0_1_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_RD_RLS** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_DRAM0_1_RD_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_DRAM0_1_RD_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_WR_RLS** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_wr_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_DRAM0_0_WR_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_DRAM0_0_WR_RLS_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_RD_RLS** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor interrupt enable
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_rd_rls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrRls::CORE_1_AREA_DRAM0_0_RD_RLS_MASK
            } else {
                self.bits & !Core1IntrRls::CORE_1_AREA_DRAM0_0_RD_RLS_MASK
            },
        }
    }
}

impl Default for Core1IntrRlsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1IntrRls {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1IntrRlsVal {
        Core1IntrRlsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1IntrRlsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1IntrRlsVal) -> Core1IntrRlsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_INTR_CLR Register
///
/// core1 monitor interrupt clr register
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **CORE_1_DRAM0_EXCEPTION_MONITOR_CLR** `[11]` (WO)
///
///   DBUS busy monitor interrupt clr
///
/// - **CORE_1_IRAM0_EXCEPTION_MONITOR_CLR** `[10]` (WO)
///
///   IBUS busy monitor interrupt clr
///
/// - **CORE_1_SP_SPILL_MAX_CLR** `[9]` (WO)
///
///   Core1 stackpoint overflow monitor interrupt clr
///
/// - **CORE_1_SP_SPILL_MIN_CLR** `[8]` (WO)
///
///   Core1 stackpoint underflow monitor interrupt clr
///
/// - **CORE_1_AREA_PIF_1_WR_CLR** `[7]` (WO)
///
///   Core1 PIF area1 write monitor interrupt clr
///
/// - **CORE_1_AREA_PIF_1_RD_CLR** `[6]` (WO)
///
///   Core1 PIF area1 read monitor interrupt clr
///
/// - **CORE_1_AREA_PIF_0_WR_CLR** `[5]` (WO)
///
///   Core1 PIF area0 write monitor interrupt clr
///
/// - **CORE_1_AREA_PIF_0_RD_CLR** `[4]` (WO)
///
///   Core1 PIF area0 read monitor interrupt clr
///
/// - **CORE_1_AREA_DRAM0_1_WR_CLR** `[3]` (WO)
///
///   Core1 dram0 area1 write monitor interrupt clr
///
/// - **CORE_1_AREA_DRAM0_1_RD_CLR** `[2]` (WO)
///
///   Core1 dram0 area1 read monitor interrupt clr
///
/// - **CORE_1_AREA_DRAM0_0_WR_CLR** `[1]` (WO)
///
///   Core1 dram0 area0 write monitor interrupt clr
///
/// - **CORE_1_AREA_DRAM0_0_RD_CLR** `[0]` (WO)
///
///   Core1 dram0 area0 read monitor interrupt clr
#[repr(transparent)]
pub struct Core1IntrClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1IntrClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1IntrClr {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_EXCEPTION_MONITOR_CLR**
    ///
    /// DBUS busy monitor interrupt clr
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_OFFSET: u32 = 11;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_MASK: u32 = 0x00000800;
    pub const CORE_1_DRAM0_EXCEPTION_MONITOR_CLR: u32 = 1 << 11;

    /// **CORE_1_IRAM0_EXCEPTION_MONITOR_CLR**
    ///
    /// IBUS busy monitor interrupt clr
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_OFFSET: u32 = 10;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_MASK: u32 = 0x00000400;
    pub const CORE_1_IRAM0_EXCEPTION_MONITOR_CLR: u32 = 1 << 10;

    /// **CORE_1_SP_SPILL_MAX_CLR**
    ///
    /// Core1 stackpoint overflow monitor interrupt clr
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE_1_SP_SPILL_MAX_CLR_OFFSET: u32 = 9;
    pub const CORE_1_SP_SPILL_MAX_CLR_MASK: u32 = 0x00000200;
    pub const CORE_1_SP_SPILL_MAX_CLR: u32 = 1 << 9;

    /// **CORE_1_SP_SPILL_MIN_CLR**
    ///
    /// Core1 stackpoint underflow monitor interrupt clr
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE_1_SP_SPILL_MIN_CLR_OFFSET: u32 = 8;
    pub const CORE_1_SP_SPILL_MIN_CLR_MASK: u32 = 0x00000100;
    pub const CORE_1_SP_SPILL_MIN_CLR: u32 = 1 << 8;

    /// **CORE_1_AREA_PIF_1_WR_CLR**
    ///
    /// Core1 PIF area1 write monitor interrupt clr
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CORE_1_AREA_PIF_1_WR_CLR_OFFSET: u32 = 7;
    pub const CORE_1_AREA_PIF_1_WR_CLR_MASK: u32 = 0x00000080;
    pub const CORE_1_AREA_PIF_1_WR_CLR: u32 = 1 << 7;

    /// **CORE_1_AREA_PIF_1_RD_CLR**
    ///
    /// Core1 PIF area1 read monitor interrupt clr
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CORE_1_AREA_PIF_1_RD_CLR_OFFSET: u32 = 6;
    pub const CORE_1_AREA_PIF_1_RD_CLR_MASK: u32 = 0x00000040;
    pub const CORE_1_AREA_PIF_1_RD_CLR: u32 = 1 << 6;

    /// **CORE_1_AREA_PIF_0_WR_CLR**
    ///
    /// Core1 PIF area0 write monitor interrupt clr
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE_1_AREA_PIF_0_WR_CLR_OFFSET: u32 = 5;
    pub const CORE_1_AREA_PIF_0_WR_CLR_MASK: u32 = 0x00000020;
    pub const CORE_1_AREA_PIF_0_WR_CLR: u32 = 1 << 5;

    /// **CORE_1_AREA_PIF_0_RD_CLR**
    ///
    /// Core1 PIF area0 read monitor interrupt clr
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE_1_AREA_PIF_0_RD_CLR_OFFSET: u32 = 4;
    pub const CORE_1_AREA_PIF_0_RD_CLR_MASK: u32 = 0x00000010;
    pub const CORE_1_AREA_PIF_0_RD_CLR: u32 = 1 << 4;

    /// **CORE_1_AREA_DRAM0_1_WR_CLR**
    ///
    /// Core1 dram0 area1 write monitor interrupt clr
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE_1_AREA_DRAM0_1_WR_CLR_OFFSET: u32 = 3;
    pub const CORE_1_AREA_DRAM0_1_WR_CLR_MASK: u32 = 0x00000008;
    pub const CORE_1_AREA_DRAM0_1_WR_CLR: u32 = 1 << 3;

    /// **CORE_1_AREA_DRAM0_1_RD_CLR**
    ///
    /// Core1 dram0 area1 read monitor interrupt clr
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE_1_AREA_DRAM0_1_RD_CLR_OFFSET: u32 = 2;
    pub const CORE_1_AREA_DRAM0_1_RD_CLR_MASK: u32 = 0x00000004;
    pub const CORE_1_AREA_DRAM0_1_RD_CLR: u32 = 1 << 2;

    /// **CORE_1_AREA_DRAM0_0_WR_CLR**
    ///
    /// Core1 dram0 area0 write monitor interrupt clr
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_AREA_DRAM0_0_WR_CLR_OFFSET: u32 = 1;
    pub const CORE_1_AREA_DRAM0_0_WR_CLR_MASK: u32 = 0x00000002;
    pub const CORE_1_AREA_DRAM0_0_WR_CLR: u32 = 1 << 1;

    /// **CORE_1_AREA_DRAM0_0_RD_CLR**
    ///
    /// Core1 dram0 area0 read monitor interrupt clr
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_AREA_DRAM0_0_RD_CLR_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_RD_CLR_MASK: u32 = 0x00000001;
    pub const CORE_1_AREA_DRAM0_0_RD_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CORE_1_DRAM0_EXCEPTION_MONITOR_CLR** (WO)
    ///
    /// DBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_dram0_exception_monitor_clr(&self) {
        self.value
            .write_bits(Self::CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_MASK);
    }

    /// Trigger **CORE_1_IRAM0_EXCEPTION_MONITOR_CLR** (WO)
    ///
    /// IBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_iram0_exception_monitor_clr(&self) {
        self.value
            .write_bits(Self::CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_MASK);
    }

    /// Trigger **CORE_1_SP_SPILL_MAX_CLR** (WO)
    ///
    /// Core1 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_sp_spill_max_clr(&self) {
        self.value.write_bits(Self::CORE_1_SP_SPILL_MAX_CLR_MASK);
    }

    /// Trigger **CORE_1_SP_SPILL_MIN_CLR** (WO)
    ///
    /// Core1 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_sp_spill_min_clr(&self) {
        self.value.write_bits(Self::CORE_1_SP_SPILL_MIN_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_PIF_1_WR_CLR** (WO)
    ///
    /// Core1 PIF area1 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_pif_1_wr_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_PIF_1_WR_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_PIF_1_RD_CLR** (WO)
    ///
    /// Core1 PIF area1 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_pif_1_rd_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_PIF_1_RD_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_PIF_0_WR_CLR** (WO)
    ///
    /// Core1 PIF area0 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_pif_0_wr_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_PIF_0_WR_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_PIF_0_RD_CLR** (WO)
    ///
    /// Core1 PIF area0 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_pif_0_rd_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_PIF_0_RD_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_DRAM0_1_WR_CLR** (WO)
    ///
    /// Core1 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_dram0_1_wr_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_DRAM0_1_WR_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_DRAM0_1_RD_CLR** (WO)
    ///
    /// Core1 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_dram0_1_rd_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_DRAM0_1_RD_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_DRAM0_0_WR_CLR** (WO)
    ///
    /// Core1 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_dram0_0_wr_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_DRAM0_0_WR_CLR_MASK);
    }

    /// Trigger **CORE_1_AREA_DRAM0_0_RD_CLR** (WO)
    ///
    /// Core1 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    pub fn trigger_core_1_area_dram0_0_rd_clr(&self) {
        self.value.write_bits(Self::CORE_1_AREA_DRAM0_0_RD_CLR_MASK);
    }
}

/// CORE_1_INTR_CLR value type for batch read/modify/write operations
///
/// Use `core1intrclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1intrclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1IntrClrVal {
    bits: u32,
}

impl Core1IntrClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **CORE_1_DRAM0_EXCEPTION_MONITOR_CLR** `[11:11]`
    ///
    /// DBUS busy monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_dram0_exception_monitor_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_IRAM0_EXCEPTION_MONITOR_CLR** `[10:10]`
    ///
    /// IBUS busy monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_iram0_exception_monitor_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MAX_CLR** `[9:9]`
    ///
    /// Core1 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_sp_spill_max_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_SP_SPILL_MAX_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_SP_SPILL_MAX_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_SP_SPILL_MIN_CLR** `[8:8]`
    ///
    /// Core1 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_sp_spill_min_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_SP_SPILL_MIN_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_SP_SPILL_MIN_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_WR_CLR** `[7:7]`
    ///
    /// Core1 PIF area1 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_PIF_1_WR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_PIF_1_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_1_RD_CLR** `[6:6]`
    ///
    /// Core1 PIF area1 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_PIF_1_RD_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_PIF_1_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_WR_CLR** `[5:5]`
    ///
    /// Core1 PIF area0 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_PIF_0_WR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_PIF_0_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_PIF_0_RD_CLR** `[4:4]`
    ///
    /// Core1 PIF area0 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_PIF_0_RD_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_PIF_0_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_WR_CLR** `[3:3]`
    ///
    /// Core1 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_DRAM0_1_WR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_DRAM0_1_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_1_RD_CLR** `[2:2]`
    ///
    /// Core1 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_DRAM0_1_RD_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_DRAM0_1_RD_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_WR_CLR** `[1:1]`
    ///
    /// Core1 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_wr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_DRAM0_0_WR_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_DRAM0_0_WR_CLR_MASK
            },
        }
    }

    /// Set **CORE_1_AREA_DRAM0_0_RD_CLR** `[0:0]`
    ///
    /// Core1 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_rd_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1IntrClr::CORE_1_AREA_DRAM0_0_RD_CLR_MASK
            } else {
                self.bits & !Core1IntrClr::CORE_1_AREA_DRAM0_0_RD_CLR_MASK
            },
        }
    }
}

impl Default for Core1IntrClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1IntrClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1IntrClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// CORE_1_AREA_DRAM0_0_MIN Register
///
/// core1 dram0 region0 addr configuration register
///
/// Address offset: 0x0090
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_1_AREA_DRAM0_0_MIN** `[31:0]` (RW)
///
///   Core1 dram0 region0 start addr
#[repr(transparent)]
pub struct Core1AreaDram00Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaDram00Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaDram00Min {
    // ======== Field Constants ========

    /// **CORE_1_AREA_DRAM0_0_MIN**
    ///
    /// Core1 dram0 region0 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_DRAM0_0_MIN_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_DRAM0_0_MIN** `[31:0]` (RW)
    ///
    /// Core1 dram0 region0 start addr
    #[inline(always)]
    pub fn core_1_area_dram0_0_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_MIN_MASK)
            >> Self::CORE_1_AREA_DRAM0_0_MIN_OFFSET
    }

    /// Write **CORE_1_AREA_DRAM0_0_MIN** `[31:0]` (RW)
    ///
    /// Core1 dram0 region0 start addr
    #[inline(always)]
    pub fn write_core_1_area_dram0_0_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_DRAM0_0_MIN_MASK)
                | ((value << Self::CORE_1_AREA_DRAM0_0_MIN_OFFSET)
                    & Self::CORE_1_AREA_DRAM0_0_MIN_MASK)
        });
    }
}

/// CORE_1_AREA_DRAM0_0_MIN value type for batch read/modify/write operations
///
/// Use `core1areadram00min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areadram00min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaDram00MinVal {
    bits: u32,
}

impl Core1AreaDram00MinVal {
    /// Create from raw bits
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

    /// Read **CORE_1_AREA_DRAM0_0_MIN** `[31:0]`
    ///
    /// Core1 dram0 region0 start addr
    #[inline(always)]
    pub const fn core_1_area_dram0_0_min(&self) -> u32 {
        (self.bits & Core1AreaDram00Min::CORE_1_AREA_DRAM0_0_MIN_MASK)
            >> Core1AreaDram00Min::CORE_1_AREA_DRAM0_0_MIN_OFFSET
    }

    /// Set **CORE_1_AREA_DRAM0_0_MIN** `[31:0]`
    ///
    /// Core1 dram0 region0 start addr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaDram00Min::CORE_1_AREA_DRAM0_0_MIN_MASK)
                | ((value << Core1AreaDram00Min::CORE_1_AREA_DRAM0_0_MIN_OFFSET)
                    & Core1AreaDram00Min::CORE_1_AREA_DRAM0_0_MIN_MASK),
        }
    }
}

impl Default for Core1AreaDram00MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core1AreaDram00Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaDram00MinVal {
        Core1AreaDram00MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaDram00MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaDram00MinVal) -> Core1AreaDram00MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_DRAM0_0_MAX Register
///
/// core1 dram0 region0 addr configuration register
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **CORE_1_AREA_DRAM0_0_MAX** `[31:0]` (RW)
///
///   Core1 dram0 region0 end addr
#[repr(transparent)]
pub struct Core1AreaDram00Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaDram00Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaDram00Max {
    // ======== Field Constants ========

    /// **CORE_1_AREA_DRAM0_0_MAX**
    ///
    /// Core1 dram0 region0 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_DRAM0_0_MAX_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_0_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_DRAM0_0_MAX** `[31:0]` (RW)
    ///
    /// Core1 dram0 region0 end addr
    #[inline(always)]
    pub fn core_1_area_dram0_0_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_0_MAX_MASK)
            >> Self::CORE_1_AREA_DRAM0_0_MAX_OFFSET
    }

    /// Write **CORE_1_AREA_DRAM0_0_MAX** `[31:0]` (RW)
    ///
    /// Core1 dram0 region0 end addr
    #[inline(always)]
    pub fn write_core_1_area_dram0_0_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_DRAM0_0_MAX_MASK)
                | ((value << Self::CORE_1_AREA_DRAM0_0_MAX_OFFSET)
                    & Self::CORE_1_AREA_DRAM0_0_MAX_MASK)
        });
    }
}

/// CORE_1_AREA_DRAM0_0_MAX value type for batch read/modify/write operations
///
/// Use `core1areadram00max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areadram00max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaDram00MaxVal {
    bits: u32,
}

impl Core1AreaDram00MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_DRAM0_0_MAX** `[31:0]`
    ///
    /// Core1 dram0 region0 end addr
    #[inline(always)]
    pub const fn core_1_area_dram0_0_max(&self) -> u32 {
        (self.bits & Core1AreaDram00Max::CORE_1_AREA_DRAM0_0_MAX_MASK)
            >> Core1AreaDram00Max::CORE_1_AREA_DRAM0_0_MAX_OFFSET
    }

    /// Set **CORE_1_AREA_DRAM0_0_MAX** `[31:0]`
    ///
    /// Core1 dram0 region0 end addr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_0_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaDram00Max::CORE_1_AREA_DRAM0_0_MAX_MASK)
                | ((value << Core1AreaDram00Max::CORE_1_AREA_DRAM0_0_MAX_OFFSET)
                    & Core1AreaDram00Max::CORE_1_AREA_DRAM0_0_MAX_MASK),
        }
    }
}

impl Default for Core1AreaDram00MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaDram00Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaDram00MaxVal {
        Core1AreaDram00MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaDram00MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaDram00MaxVal) -> Core1AreaDram00MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_DRAM0_1_MIN Register
///
/// core1 dram0 region1 addr configuration register
///
/// Address offset: 0x0098
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_1_AREA_DRAM0_1_MIN** `[31:0]` (RW)
///
///   Core1 dram0 region1 start addr
#[repr(transparent)]
pub struct Core1AreaDram01Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaDram01Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaDram01Min {
    // ======== Field Constants ========

    /// **CORE_1_AREA_DRAM0_1_MIN**
    ///
    /// Core1 dram0 region1 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_DRAM0_1_MIN_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_1_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_DRAM0_1_MIN** `[31:0]` (RW)
    ///
    /// Core1 dram0 region1 start addr
    #[inline(always)]
    pub fn core_1_area_dram0_1_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_MIN_MASK)
            >> Self::CORE_1_AREA_DRAM0_1_MIN_OFFSET
    }

    /// Write **CORE_1_AREA_DRAM0_1_MIN** `[31:0]` (RW)
    ///
    /// Core1 dram0 region1 start addr
    #[inline(always)]
    pub fn write_core_1_area_dram0_1_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_DRAM0_1_MIN_MASK)
                | ((value << Self::CORE_1_AREA_DRAM0_1_MIN_OFFSET)
                    & Self::CORE_1_AREA_DRAM0_1_MIN_MASK)
        });
    }
}

/// CORE_1_AREA_DRAM0_1_MIN value type for batch read/modify/write operations
///
/// Use `core1areadram01min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areadram01min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaDram01MinVal {
    bits: u32,
}

impl Core1AreaDram01MinVal {
    /// Create from raw bits
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

    /// Read **CORE_1_AREA_DRAM0_1_MIN** `[31:0]`
    ///
    /// Core1 dram0 region1 start addr
    #[inline(always)]
    pub const fn core_1_area_dram0_1_min(&self) -> u32 {
        (self.bits & Core1AreaDram01Min::CORE_1_AREA_DRAM0_1_MIN_MASK)
            >> Core1AreaDram01Min::CORE_1_AREA_DRAM0_1_MIN_OFFSET
    }

    /// Set **CORE_1_AREA_DRAM0_1_MIN** `[31:0]`
    ///
    /// Core1 dram0 region1 start addr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaDram01Min::CORE_1_AREA_DRAM0_1_MIN_MASK)
                | ((value << Core1AreaDram01Min::CORE_1_AREA_DRAM0_1_MIN_OFFSET)
                    & Core1AreaDram01Min::CORE_1_AREA_DRAM0_1_MIN_MASK),
        }
    }
}

impl Default for Core1AreaDram01MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core1AreaDram01Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaDram01MinVal {
        Core1AreaDram01MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaDram01MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaDram01MinVal) -> Core1AreaDram01MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_DRAM0_1_MAX Register
///
/// core1 dram0 region1 addr configuration register
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **CORE_1_AREA_DRAM0_1_MAX** `[31:0]` (RW)
///
///   Core1 dram0 region1 end addr
#[repr(transparent)]
pub struct Core1AreaDram01Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaDram01Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaDram01Max {
    // ======== Field Constants ========

    /// **CORE_1_AREA_DRAM0_1_MAX**
    ///
    /// Core1 dram0 region1 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_DRAM0_1_MAX_OFFSET: u32 = 0;
    pub const CORE_1_AREA_DRAM0_1_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_DRAM0_1_MAX** `[31:0]` (RW)
    ///
    /// Core1 dram0 region1 end addr
    #[inline(always)]
    pub fn core_1_area_dram0_1_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_DRAM0_1_MAX_MASK)
            >> Self::CORE_1_AREA_DRAM0_1_MAX_OFFSET
    }

    /// Write **CORE_1_AREA_DRAM0_1_MAX** `[31:0]` (RW)
    ///
    /// Core1 dram0 region1 end addr
    #[inline(always)]
    pub fn write_core_1_area_dram0_1_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_DRAM0_1_MAX_MASK)
                | ((value << Self::CORE_1_AREA_DRAM0_1_MAX_OFFSET)
                    & Self::CORE_1_AREA_DRAM0_1_MAX_MASK)
        });
    }
}

/// CORE_1_AREA_DRAM0_1_MAX value type for batch read/modify/write operations
///
/// Use `core1areadram01max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areadram01max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaDram01MaxVal {
    bits: u32,
}

impl Core1AreaDram01MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_DRAM0_1_MAX** `[31:0]`
    ///
    /// Core1 dram0 region1 end addr
    #[inline(always)]
    pub const fn core_1_area_dram0_1_max(&self) -> u32 {
        (self.bits & Core1AreaDram01Max::CORE_1_AREA_DRAM0_1_MAX_MASK)
            >> Core1AreaDram01Max::CORE_1_AREA_DRAM0_1_MAX_OFFSET
    }

    /// Set **CORE_1_AREA_DRAM0_1_MAX** `[31:0]`
    ///
    /// Core1 dram0 region1 end addr
    #[inline(always)]
    pub const fn with_core_1_area_dram0_1_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaDram01Max::CORE_1_AREA_DRAM0_1_MAX_MASK)
                | ((value << Core1AreaDram01Max::CORE_1_AREA_DRAM0_1_MAX_OFFSET)
                    & Core1AreaDram01Max::CORE_1_AREA_DRAM0_1_MAX_MASK),
        }
    }
}

impl Default for Core1AreaDram01MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaDram01Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaDram01MaxVal {
        Core1AreaDram01MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaDram01MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaDram01MaxVal) -> Core1AreaDram01MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_PIF_0_MIN Register
///
/// core1 PIF region0 addr configuration register
///
/// Address offset: 0x00a0
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_1_AREA_PIF_0_MIN** `[31:0]` (RW)
///
///   Core1 PIF region0 start addr
#[repr(transparent)]
pub struct Core1AreaPif0Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaPif0Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaPif0Min {
    // ======== Field Constants ========

    /// **CORE_1_AREA_PIF_0_MIN**
    ///
    /// Core1 PIF region0 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_PIF_0_MIN_OFFSET: u32 = 0;
    pub const CORE_1_AREA_PIF_0_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_PIF_0_MIN** `[31:0]` (RW)
    ///
    /// Core1 PIF region0 start addr
    #[inline(always)]
    pub fn core_1_area_pif_0_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_MIN_MASK)
            >> Self::CORE_1_AREA_PIF_0_MIN_OFFSET
    }

    /// Write **CORE_1_AREA_PIF_0_MIN** `[31:0]` (RW)
    ///
    /// Core1 PIF region0 start addr
    #[inline(always)]
    pub fn write_core_1_area_pif_0_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_PIF_0_MIN_MASK)
                | ((value << Self::CORE_1_AREA_PIF_0_MIN_OFFSET) & Self::CORE_1_AREA_PIF_0_MIN_MASK)
        });
    }
}

/// CORE_1_AREA_PIF_0_MIN value type for batch read/modify/write operations
///
/// Use `core1areapif0min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areapif0min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaPif0MinVal {
    bits: u32,
}

impl Core1AreaPif0MinVal {
    /// Create from raw bits
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

    /// Read **CORE_1_AREA_PIF_0_MIN** `[31:0]`
    ///
    /// Core1 PIF region0 start addr
    #[inline(always)]
    pub const fn core_1_area_pif_0_min(&self) -> u32 {
        (self.bits & Core1AreaPif0Min::CORE_1_AREA_PIF_0_MIN_MASK)
            >> Core1AreaPif0Min::CORE_1_AREA_PIF_0_MIN_OFFSET
    }

    /// Set **CORE_1_AREA_PIF_0_MIN** `[31:0]`
    ///
    /// Core1 PIF region0 start addr
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaPif0Min::CORE_1_AREA_PIF_0_MIN_MASK)
                | ((value << Core1AreaPif0Min::CORE_1_AREA_PIF_0_MIN_OFFSET)
                    & Core1AreaPif0Min::CORE_1_AREA_PIF_0_MIN_MASK),
        }
    }
}

impl Default for Core1AreaPif0MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core1AreaPif0Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaPif0MinVal {
        Core1AreaPif0MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaPif0MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaPif0MinVal) -> Core1AreaPif0MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_PIF_0_MAX Register
///
/// core1 PIF region0 addr configuration register
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **CORE_1_AREA_PIF_0_MAX** `[31:0]` (RW)
///
///   Core1 PIF region0 end addr
#[repr(transparent)]
pub struct Core1AreaPif0Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaPif0Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaPif0Max {
    // ======== Field Constants ========

    /// **CORE_1_AREA_PIF_0_MAX**
    ///
    /// Core1 PIF region0 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_PIF_0_MAX_OFFSET: u32 = 0;
    pub const CORE_1_AREA_PIF_0_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_PIF_0_MAX** `[31:0]` (RW)
    ///
    /// Core1 PIF region0 end addr
    #[inline(always)]
    pub fn core_1_area_pif_0_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_0_MAX_MASK)
            >> Self::CORE_1_AREA_PIF_0_MAX_OFFSET
    }

    /// Write **CORE_1_AREA_PIF_0_MAX** `[31:0]` (RW)
    ///
    /// Core1 PIF region0 end addr
    #[inline(always)]
    pub fn write_core_1_area_pif_0_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_PIF_0_MAX_MASK)
                | ((value << Self::CORE_1_AREA_PIF_0_MAX_OFFSET) & Self::CORE_1_AREA_PIF_0_MAX_MASK)
        });
    }
}

/// CORE_1_AREA_PIF_0_MAX value type for batch read/modify/write operations
///
/// Use `core1areapif0max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areapif0max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaPif0MaxVal {
    bits: u32,
}

impl Core1AreaPif0MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_PIF_0_MAX** `[31:0]`
    ///
    /// Core1 PIF region0 end addr
    #[inline(always)]
    pub const fn core_1_area_pif_0_max(&self) -> u32 {
        (self.bits & Core1AreaPif0Max::CORE_1_AREA_PIF_0_MAX_MASK)
            >> Core1AreaPif0Max::CORE_1_AREA_PIF_0_MAX_OFFSET
    }

    /// Set **CORE_1_AREA_PIF_0_MAX** `[31:0]`
    ///
    /// Core1 PIF region0 end addr
    #[inline(always)]
    pub const fn with_core_1_area_pif_0_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaPif0Max::CORE_1_AREA_PIF_0_MAX_MASK)
                | ((value << Core1AreaPif0Max::CORE_1_AREA_PIF_0_MAX_OFFSET)
                    & Core1AreaPif0Max::CORE_1_AREA_PIF_0_MAX_MASK),
        }
    }
}

impl Default for Core1AreaPif0MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaPif0Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaPif0MaxVal {
        Core1AreaPif0MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaPif0MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaPif0MaxVal) -> Core1AreaPif0MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_PIF_1_MIN Register
///
/// core1 PIF region1 addr configuration register
///
/// Address offset: 0x00a8
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_1_AREA_PIF_1_MIN** `[31:0]` (RW)
///
///   Core1 PIF region1 start addr
#[repr(transparent)]
pub struct Core1AreaPif1Min {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaPif1Min {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaPif1Min {
    // ======== Field Constants ========

    /// **CORE_1_AREA_PIF_1_MIN**
    ///
    /// Core1 PIF region1 start addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_PIF_1_MIN_OFFSET: u32 = 0;
    pub const CORE_1_AREA_PIF_1_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_PIF_1_MIN** `[31:0]` (RW)
    ///
    /// Core1 PIF region1 start addr
    #[inline(always)]
    pub fn core_1_area_pif_1_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_MIN_MASK)
            >> Self::CORE_1_AREA_PIF_1_MIN_OFFSET
    }

    /// Write **CORE_1_AREA_PIF_1_MIN** `[31:0]` (RW)
    ///
    /// Core1 PIF region1 start addr
    #[inline(always)]
    pub fn write_core_1_area_pif_1_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_PIF_1_MIN_MASK)
                | ((value << Self::CORE_1_AREA_PIF_1_MIN_OFFSET) & Self::CORE_1_AREA_PIF_1_MIN_MASK)
        });
    }
}

/// CORE_1_AREA_PIF_1_MIN value type for batch read/modify/write operations
///
/// Use `core1areapif1min.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areapif1min.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaPif1MinVal {
    bits: u32,
}

impl Core1AreaPif1MinVal {
    /// Create from raw bits
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

    /// Read **CORE_1_AREA_PIF_1_MIN** `[31:0]`
    ///
    /// Core1 PIF region1 start addr
    #[inline(always)]
    pub const fn core_1_area_pif_1_min(&self) -> u32 {
        (self.bits & Core1AreaPif1Min::CORE_1_AREA_PIF_1_MIN_MASK)
            >> Core1AreaPif1Min::CORE_1_AREA_PIF_1_MIN_OFFSET
    }

    /// Set **CORE_1_AREA_PIF_1_MIN** `[31:0]`
    ///
    /// Core1 PIF region1 start addr
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaPif1Min::CORE_1_AREA_PIF_1_MIN_MASK)
                | ((value << Core1AreaPif1Min::CORE_1_AREA_PIF_1_MIN_OFFSET)
                    & Core1AreaPif1Min::CORE_1_AREA_PIF_1_MIN_MASK),
        }
    }
}

impl Default for Core1AreaPif1MinVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core1AreaPif1Min {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaPif1MinVal {
        Core1AreaPif1MinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaPif1MinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaPif1MinVal) -> Core1AreaPif1MinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_PIF_1_MAX Register
///
/// core1 PIF region1 addr configuration register
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **CORE_1_AREA_PIF_1_MAX** `[31:0]` (RW)
///
///   Core1 PIF region1 end addr
#[repr(transparent)]
pub struct Core1AreaPif1Max {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaPif1Max {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaPif1Max {
    // ======== Field Constants ========

    /// **CORE_1_AREA_PIF_1_MAX**
    ///
    /// Core1 PIF region1 end addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_PIF_1_MAX_OFFSET: u32 = 0;
    pub const CORE_1_AREA_PIF_1_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_PIF_1_MAX** `[31:0]` (RW)
    ///
    /// Core1 PIF region1 end addr
    #[inline(always)]
    pub fn core_1_area_pif_1_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_PIF_1_MAX_MASK)
            >> Self::CORE_1_AREA_PIF_1_MAX_OFFSET
    }

    /// Write **CORE_1_AREA_PIF_1_MAX** `[31:0]` (RW)
    ///
    /// Core1 PIF region1 end addr
    #[inline(always)]
    pub fn write_core_1_area_pif_1_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_AREA_PIF_1_MAX_MASK)
                | ((value << Self::CORE_1_AREA_PIF_1_MAX_OFFSET) & Self::CORE_1_AREA_PIF_1_MAX_MASK)
        });
    }
}

/// CORE_1_AREA_PIF_1_MAX value type for batch read/modify/write operations
///
/// Use `core1areapif1max.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areapif1max.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaPif1MaxVal {
    bits: u32,
}

impl Core1AreaPif1MaxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_PIF_1_MAX** `[31:0]`
    ///
    /// Core1 PIF region1 end addr
    #[inline(always)]
    pub const fn core_1_area_pif_1_max(&self) -> u32 {
        (self.bits & Core1AreaPif1Max::CORE_1_AREA_PIF_1_MAX_MASK)
            >> Core1AreaPif1Max::CORE_1_AREA_PIF_1_MAX_OFFSET
    }

    /// Set **CORE_1_AREA_PIF_1_MAX** `[31:0]`
    ///
    /// Core1 PIF region1 end addr
    #[inline(always)]
    pub const fn with_core_1_area_pif_1_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1AreaPif1Max::CORE_1_AREA_PIF_1_MAX_MASK)
                | ((value << Core1AreaPif1Max::CORE_1_AREA_PIF_1_MAX_OFFSET)
                    & Core1AreaPif1Max::CORE_1_AREA_PIF_1_MAX_MASK),
        }
    }
}

impl Default for Core1AreaPif1MaxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaPif1Max {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaPif1MaxVal {
        Core1AreaPif1MaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1AreaPif1MaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1AreaPif1MaxVal) -> Core1AreaPif1MaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_AREA_PC Register
///
/// core1 area pc status register
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **CORE_1_AREA_PC** `[31:0]` (RO)
///
///   the stackpointer when first touch region monitor interrupt
#[repr(transparent)]
pub struct Core1AreaPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaPc {
    // ======== Field Constants ========

    /// **CORE_1_AREA_PC**
    ///
    /// the stackpointer when first touch region monitor interrupt
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_PC_OFFSET: u32 = 0;
    pub const CORE_1_AREA_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_PC** `[31:0]` (RO)
    ///
    /// the stackpointer when first touch region monitor interrupt
    #[inline(always)]
    pub fn core_1_area_pc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_PC_MASK) >> Self::CORE_1_AREA_PC_OFFSET
    }
}

/// CORE_1_AREA_PC value type for batch read/modify/write operations
///
/// Use `core1areapc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areapc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaPcVal {
    bits: u32,
}

impl Core1AreaPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_PC** `[31:0]`
    ///
    /// the stackpointer when first touch region monitor interrupt
    #[inline(always)]
    pub const fn core_1_area_pc(&self) -> u32 {
        (self.bits & Core1AreaPc::CORE_1_AREA_PC_MASK) >> Core1AreaPc::CORE_1_AREA_PC_OFFSET
    }
}

impl Default for Core1AreaPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaPcVal {
        Core1AreaPcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_AREA_SP Register
///
/// core1 area sp status register
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **CORE_1_AREA_SP** `[31:0]` (RO)
///
///   the PC when first touch region monitor interrupt
#[repr(transparent)]
pub struct Core1AreaSp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1AreaSp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1AreaSp {
    // ======== Field Constants ========

    /// **CORE_1_AREA_SP**
    ///
    /// the PC when first touch region monitor interrupt
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_AREA_SP_OFFSET: u32 = 0;
    pub const CORE_1_AREA_SP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_AREA_SP** `[31:0]` (RO)
    ///
    /// the PC when first touch region monitor interrupt
    #[inline(always)]
    pub fn core_1_area_sp(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_AREA_SP_MASK) >> Self::CORE_1_AREA_SP_OFFSET
    }
}

/// CORE_1_AREA_SP value type for batch read/modify/write operations
///
/// Use `core1areasp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1areasp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1AreaSpVal {
    bits: u32,
}

impl Core1AreaSpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_AREA_SP** `[31:0]`
    ///
    /// the PC when first touch region monitor interrupt
    #[inline(always)]
    pub const fn core_1_area_sp(&self) -> u32 {
        (self.bits & Core1AreaSp::CORE_1_AREA_SP_MASK) >> Core1AreaSp::CORE_1_AREA_SP_OFFSET
    }
}

impl Default for Core1AreaSpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1AreaSp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1AreaSpVal {
        Core1AreaSpVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_SP_MIN Register
///
/// stack min value
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **CORE_1_SP_MIN** `[31:0]` (RW)
///
///   core1 sp region configuration regsiter
#[repr(transparent)]
pub struct Core1SpMin {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1SpMin {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1SpMin {
    // ======== Field Constants ========

    /// **CORE_1_SP_MIN**
    ///
    /// core1 sp region configuration regsiter
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_SP_MIN_OFFSET: u32 = 0;
    pub const CORE_1_SP_MIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_SP_MIN** `[31:0]` (RW)
    ///
    /// core1 sp region configuration regsiter
    #[inline(always)]
    pub fn core_1_sp_min(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_SP_MIN_MASK) >> Self::CORE_1_SP_MIN_OFFSET
    }

    /// Write **CORE_1_SP_MIN** `[31:0]` (RW)
    ///
    /// core1 sp region configuration regsiter
    #[inline(always)]
    pub fn write_core_1_sp_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_SP_MIN_MASK)
                | ((value << Self::CORE_1_SP_MIN_OFFSET) & Self::CORE_1_SP_MIN_MASK)
        });
    }
}

/// CORE_1_SP_MIN value type for batch read/modify/write operations
///
/// Use `core1spmin.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1spmin.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1SpMinVal {
    bits: u32,
}

impl Core1SpMinVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_SP_MIN** `[31:0]`
    ///
    /// core1 sp region configuration regsiter
    #[inline(always)]
    pub const fn core_1_sp_min(&self) -> u32 {
        (self.bits & Core1SpMin::CORE_1_SP_MIN_MASK) >> Core1SpMin::CORE_1_SP_MIN_OFFSET
    }

    /// Set **CORE_1_SP_MIN** `[31:0]`
    ///
    /// core1 sp region configuration regsiter
    #[inline(always)]
    pub const fn with_core_1_sp_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1SpMin::CORE_1_SP_MIN_MASK)
                | ((value << Core1SpMin::CORE_1_SP_MIN_OFFSET) & Core1SpMin::CORE_1_SP_MIN_MASK),
        }
    }
}

impl Default for Core1SpMinVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1SpMin {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1SpMinVal {
        Core1SpMinVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1SpMinVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1SpMinVal) -> Core1SpMinVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_SP_MAX Register
///
/// stack max value
///
/// Address offset: 0x00bc
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CORE_1_SP_MAX** `[31:0]` (RW)
///
///   core1 sp pc status register
#[repr(transparent)]
pub struct Core1SpMax {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1SpMax {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1SpMax {
    // ======== Field Constants ========

    /// **CORE_1_SP_MAX**
    ///
    /// core1 sp pc status register
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_SP_MAX_OFFSET: u32 = 0;
    pub const CORE_1_SP_MAX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_SP_MAX** `[31:0]` (RW)
    ///
    /// core1 sp pc status register
    #[inline(always)]
    pub fn core_1_sp_max(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_SP_MAX_MASK) >> Self::CORE_1_SP_MAX_OFFSET
    }

    /// Write **CORE_1_SP_MAX** `[31:0]` (RW)
    ///
    /// core1 sp pc status register
    #[inline(always)]
    pub fn write_core_1_sp_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_1_SP_MAX_MASK)
                | ((value << Self::CORE_1_SP_MAX_OFFSET) & Self::CORE_1_SP_MAX_MASK)
        });
    }
}

/// CORE_1_SP_MAX value type for batch read/modify/write operations
///
/// Use `core1spmax.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1spmax.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1SpMaxVal {
    bits: u32,
}

impl Core1SpMaxVal {
    /// Create from raw bits
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

    /// Read **CORE_1_SP_MAX** `[31:0]`
    ///
    /// core1 sp pc status register
    #[inline(always)]
    pub const fn core_1_sp_max(&self) -> u32 {
        (self.bits & Core1SpMax::CORE_1_SP_MAX_MASK) >> Core1SpMax::CORE_1_SP_MAX_OFFSET
    }

    /// Set **CORE_1_SP_MAX** `[31:0]`
    ///
    /// core1 sp pc status register
    #[inline(always)]
    pub const fn with_core_1_sp_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1SpMax::CORE_1_SP_MAX_MASK)
                | ((value << Core1SpMax::CORE_1_SP_MAX_OFFSET) & Core1SpMax::CORE_1_SP_MAX_MASK),
        }
    }
}

impl Default for Core1SpMaxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Core1SpMax {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1SpMaxVal {
        Core1SpMaxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1SpMaxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1SpMaxVal) -> Core1SpMaxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_SP_PC Register
///
/// stack monitor pc status register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **CORE_1_SP_PC** `[31:0]` (RO)
///
///   This regsiter stores the PC when trigger stack monitor.
#[repr(transparent)]
pub struct Core1SpPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1SpPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1SpPc {
    // ======== Field Constants ========

    /// **CORE_1_SP_PC**
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_SP_PC_OFFSET: u32 = 0;
    pub const CORE_1_SP_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_SP_PC** `[31:0]` (RO)
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    #[inline(always)]
    pub fn core_1_sp_pc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_SP_PC_MASK) >> Self::CORE_1_SP_PC_OFFSET
    }
}

/// CORE_1_SP_PC value type for batch read/modify/write operations
///
/// Use `core1sppc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1sppc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1SpPcVal {
    bits: u32,
}

impl Core1SpPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_SP_PC** `[31:0]`
    ///
    /// This regsiter stores the PC when trigger stack monitor.
    #[inline(always)]
    pub const fn core_1_sp_pc(&self) -> u32 {
        (self.bits & Core1SpPc::CORE_1_SP_PC_MASK) >> Core1SpPc::CORE_1_SP_PC_OFFSET
    }
}

impl Default for Core1SpPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1SpPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1SpPcVal {
        Core1SpPcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_RCD_EN Register
///
/// record enable configuration register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **CORE_1_RCD_PDEBUGEN** `[1]` (RW)
///
///   Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
///
/// - **CORE_1_RCD_RECORDEN** `[0]` (RW)
///
///   Set 1 to enable record PC
#[repr(transparent)]
pub struct Core1RcdEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1RcdEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1RcdEn {
    // ======== Field Constants ========

    /// **CORE_1_RCD_PDEBUGEN**
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_RCD_PDEBUGEN_OFFSET: u32 = 1;
    pub const CORE_1_RCD_PDEBUGEN_MASK: u32 = 0x00000002;
    pub const CORE_1_RCD_PDEBUGEN: u32 = 1 << 1;

    /// **CORE_1_RCD_RECORDEN**
    ///
    /// Set 1 to enable record PC
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_RCD_RECORDEN_OFFSET: u32 = 0;
    pub const CORE_1_RCD_RECORDEN_MASK: u32 = 0x00000001;
    pub const CORE_1_RCD_RECORDEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_RCD_PDEBUGEN** `[1]` (RW)
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub fn core_1_rcd_pdebugen(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_RCD_PDEBUGEN_MASK) != 0
    }

    /// Set CORE_1_RCD_PDEBUGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_rcd_pdebugen(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_RCD_PDEBUGEN_MASK);
    }

    /// Clear CORE_1_RCD_PDEBUGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_rcd_pdebugen(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_RCD_PDEBUGEN_MASK);
    }

    /// **CORE_1_RCD_RECORDEN** `[0]` (RW)
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub fn core_1_rcd_recorden(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_RCD_RECORDEN_MASK) != 0
    }

    /// Set CORE_1_RCD_RECORDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_1_rcd_recorden(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_1_RCD_RECORDEN_MASK);
    }

    /// Clear CORE_1_RCD_RECORDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_1_rcd_recorden(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_1_RCD_RECORDEN_MASK);
    }
}

/// CORE_1_RCD_EN value type for batch read/modify/write operations
///
/// Use `core1rcden.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1rcden.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1RcdEnVal {
    bits: u32,
}

impl Core1RcdEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_RCD_PDEBUGEN** `[1:1]`
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub const fn core_1_rcd_pdebugen(&self) -> bool {
        (self.bits & Core1RcdEn::CORE_1_RCD_PDEBUGEN_MASK) != 0
    }

    /// Read **CORE_1_RCD_RECORDEN** `[0:0]`
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub const fn core_1_rcd_recorden(&self) -> bool {
        (self.bits & Core1RcdEn::CORE_1_RCD_RECORDEN_MASK) != 0
    }

    /// Set **CORE_1_RCD_PDEBUGEN** `[1:1]`
    ///
    /// Set 1 to enable cpu pdebug function, must set this bit can get cpu PC
    #[inline(always)]
    pub const fn with_core_1_rcd_pdebugen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1RcdEn::CORE_1_RCD_PDEBUGEN_MASK
            } else {
                self.bits & !Core1RcdEn::CORE_1_RCD_PDEBUGEN_MASK
            },
        }
    }

    /// Set **CORE_1_RCD_RECORDEN** `[0:0]`
    ///
    /// Set 1 to enable record PC
    #[inline(always)]
    pub const fn with_core_1_rcd_recorden(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Core1RcdEn::CORE_1_RCD_RECORDEN_MASK
            } else {
                self.bits & !Core1RcdEn::CORE_1_RCD_RECORDEN_MASK
            },
        }
    }
}

impl Default for Core1RcdEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1RcdEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1RcdEnVal {
        Core1RcdEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1RcdEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1RcdEnVal) -> Core1RcdEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_1_RCD_PDEBUGPC Register
///
/// record status regsiter
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **CORE_1_RCD_PDEBUGPC** `[31:0]` (RO)
///
///   recorded PC
#[repr(transparent)]
pub struct Core1RcdPdebugpc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1RcdPdebugpc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1RcdPdebugpc {
    // ======== Field Constants ========

    /// **CORE_1_RCD_PDEBUGPC**
    ///
    /// recorded PC
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_RCD_PDEBUGPC_OFFSET: u32 = 0;
    pub const CORE_1_RCD_PDEBUGPC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_RCD_PDEBUGPC** `[31:0]` (RO)
    ///
    /// recorded PC
    #[inline(always)]
    pub fn core_1_rcd_pdebugpc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_RCD_PDEBUGPC_MASK)
            >> Self::CORE_1_RCD_PDEBUGPC_OFFSET
    }
}

/// CORE_1_RCD_PDEBUGPC value type for batch read/modify/write operations
///
/// Use `core1rcdpdebugpc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1rcdpdebugpc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1RcdPdebugpcVal {
    bits: u32,
}

impl Core1RcdPdebugpcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_RCD_PDEBUGPC** `[31:0]`
    ///
    /// recorded PC
    #[inline(always)]
    pub const fn core_1_rcd_pdebugpc(&self) -> u32 {
        (self.bits & Core1RcdPdebugpc::CORE_1_RCD_PDEBUGPC_MASK)
            >> Core1RcdPdebugpc::CORE_1_RCD_PDEBUGPC_OFFSET
    }
}

impl Default for Core1RcdPdebugpcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1RcdPdebugpc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1RcdPdebugpcVal {
        Core1RcdPdebugpcVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_RCD_PDEBUGSP Register
///
/// record status regsiter
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **CORE_1_RCD_PDEBUGSP** `[31:0]` (RO)
///
///   recorded sp
#[repr(transparent)]
pub struct Core1RcdPdebugsp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1RcdPdebugsp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1RcdPdebugsp {
    // ======== Field Constants ========

    /// **CORE_1_RCD_PDEBUGSP**
    ///
    /// recorded sp
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_RCD_PDEBUGSP_OFFSET: u32 = 0;
    pub const CORE_1_RCD_PDEBUGSP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_RCD_PDEBUGSP** `[31:0]` (RO)
    ///
    /// recorded sp
    #[inline(always)]
    pub fn core_1_rcd_pdebugsp(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_RCD_PDEBUGSP_MASK)
            >> Self::CORE_1_RCD_PDEBUGSP_OFFSET
    }
}

/// CORE_1_RCD_PDEBUGSP value type for batch read/modify/write operations
///
/// Use `core1rcdpdebugsp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1rcdpdebugsp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1RcdPdebugspVal {
    bits: u32,
}

impl Core1RcdPdebugspVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_RCD_PDEBUGSP** `[31:0]`
    ///
    /// recorded sp
    #[inline(always)]
    pub const fn core_1_rcd_pdebugsp(&self) -> u32 {
        (self.bits & Core1RcdPdebugsp::CORE_1_RCD_PDEBUGSP_MASK)
            >> Core1RcdPdebugsp::CORE_1_RCD_PDEBUGSP_OFFSET
    }
}

impl Default for Core1RcdPdebugspVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1RcdPdebugsp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1RcdPdebugspVal {
        Core1RcdPdebugspVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_IRAM0_EXCEPTION_MONITOR_0 Register
///
/// exception monitor status register0
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **CORE_1_IRAM0_RECORDING_LOADSTORE_0** `[25]` (RO)
///
///   reg_core_1_iram0_recording_loadstore_0
///
/// - **CORE_1_IRAM0_RECORDING_WR_0** `[24]` (RO)
///
///   reg_core_1_iram0_recording_wr_0
///
/// - **CORE_1_IRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
///
///   reg_core_1_iram0_recording_addr_0
#[repr(transparent)]
pub struct Core1Iram0ExceptionMonitor0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Iram0ExceptionMonitor0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Iram0ExceptionMonitor0 {
    // ======== Field Constants ========

    /// **CORE_1_IRAM0_RECORDING_LOADSTORE_0**
    ///
    /// reg_core_1_iram0_recording_loadstore_0
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_0_OFFSET: u32 = 25;
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_0_MASK: u32 = 0x02000000;
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_0: u32 = 1 << 25;

    /// **CORE_1_IRAM0_RECORDING_WR_0**
    ///
    /// reg_core_1_iram0_recording_wr_0
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CORE_1_IRAM0_RECORDING_WR_0_OFFSET: u32 = 24;
    pub const CORE_1_IRAM0_RECORDING_WR_0_MASK: u32 = 0x01000000;
    pub const CORE_1_IRAM0_RECORDING_WR_0: u32 = 1 << 24;

    /// **CORE_1_IRAM0_RECORDING_ADDR_0**
    ///
    /// reg_core_1_iram0_recording_addr_0
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_1_IRAM0_RECORDING_ADDR_0_OFFSET: u32 = 0;
    pub const CORE_1_IRAM0_RECORDING_ADDR_0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_1_IRAM0_RECORDING_LOADSTORE_0** `[25]` (RO)
    ///
    /// reg_core_1_iram0_recording_loadstore_0
    #[inline(always)]
    pub fn core_1_iram0_recording_loadstore_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_LOADSTORE_0_MASK) != 0
    }

    /// **CORE_1_IRAM0_RECORDING_WR_0** `[24]` (RO)
    ///
    /// reg_core_1_iram0_recording_wr_0
    #[inline(always)]
    pub fn core_1_iram0_recording_wr_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_WR_0_MASK) != 0
    }

    /// **CORE_1_IRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
    ///
    /// reg_core_1_iram0_recording_addr_0
    #[inline(always)]
    pub fn core_1_iram0_recording_addr_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_ADDR_0_MASK)
            >> Self::CORE_1_IRAM0_RECORDING_ADDR_0_OFFSET
    }
}

/// CORE_1_IRAM0_EXCEPTION_MONITOR_0 value type for batch read/modify/write operations
///
/// Use `core1iram0exceptionmonitor0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1iram0exceptionmonitor0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Iram0ExceptionMonitor0Val {
    bits: u32,
}

impl Core1Iram0ExceptionMonitor0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_IRAM0_RECORDING_LOADSTORE_0** `[25:25]`
    ///
    /// reg_core_1_iram0_recording_loadstore_0
    #[inline(always)]
    pub const fn core_1_iram0_recording_loadstore_0(&self) -> bool {
        (self.bits & Core1Iram0ExceptionMonitor0::CORE_1_IRAM0_RECORDING_LOADSTORE_0_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_RECORDING_WR_0** `[24:24]`
    ///
    /// reg_core_1_iram0_recording_wr_0
    #[inline(always)]
    pub const fn core_1_iram0_recording_wr_0(&self) -> bool {
        (self.bits & Core1Iram0ExceptionMonitor0::CORE_1_IRAM0_RECORDING_WR_0_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_RECORDING_ADDR_0** `[23:0]`
    ///
    /// reg_core_1_iram0_recording_addr_0
    #[inline(always)]
    pub const fn core_1_iram0_recording_addr_0(&self) -> u32 {
        (self.bits & Core1Iram0ExceptionMonitor0::CORE_1_IRAM0_RECORDING_ADDR_0_MASK)
            >> Core1Iram0ExceptionMonitor0::CORE_1_IRAM0_RECORDING_ADDR_0_OFFSET
    }
}

impl Default for Core1Iram0ExceptionMonitor0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Iram0ExceptionMonitor0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Iram0ExceptionMonitor0Val {
        Core1Iram0ExceptionMonitor0Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_IRAM0_EXCEPTION_MONITOR_1 Register
///
/// exception monitor status register1
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **CORE_1_IRAM0_RECORDING_LOADSTORE_1** `[25]` (RO)
///
///   reg_core_1_iram0_recording_loadstore_1
///
/// - **CORE_1_IRAM0_RECORDING_WR_1** `[24]` (RO)
///
///   reg_core_1_iram0_recording_wr_1
///
/// - **CORE_1_IRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
///
///   reg_core_1_iram0_recording_addr_1
#[repr(transparent)]
pub struct Core1Iram0ExceptionMonitor1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Iram0ExceptionMonitor1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Iram0ExceptionMonitor1 {
    // ======== Field Constants ========

    /// **CORE_1_IRAM0_RECORDING_LOADSTORE_1**
    ///
    /// reg_core_1_iram0_recording_loadstore_1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_1_OFFSET: u32 = 25;
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_1_MASK: u32 = 0x02000000;
    pub const CORE_1_IRAM0_RECORDING_LOADSTORE_1: u32 = 1 << 25;

    /// **CORE_1_IRAM0_RECORDING_WR_1**
    ///
    /// reg_core_1_iram0_recording_wr_1
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CORE_1_IRAM0_RECORDING_WR_1_OFFSET: u32 = 24;
    pub const CORE_1_IRAM0_RECORDING_WR_1_MASK: u32 = 0x01000000;
    pub const CORE_1_IRAM0_RECORDING_WR_1: u32 = 1 << 24;

    /// **CORE_1_IRAM0_RECORDING_ADDR_1**
    ///
    /// reg_core_1_iram0_recording_addr_1
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_1_IRAM0_RECORDING_ADDR_1_OFFSET: u32 = 0;
    pub const CORE_1_IRAM0_RECORDING_ADDR_1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_1_IRAM0_RECORDING_LOADSTORE_1** `[25]` (RO)
    ///
    /// reg_core_1_iram0_recording_loadstore_1
    #[inline(always)]
    pub fn core_1_iram0_recording_loadstore_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_LOADSTORE_1_MASK) != 0
    }

    /// **CORE_1_IRAM0_RECORDING_WR_1** `[24]` (RO)
    ///
    /// reg_core_1_iram0_recording_wr_1
    #[inline(always)]
    pub fn core_1_iram0_recording_wr_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_WR_1_MASK) != 0
    }

    /// **CORE_1_IRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
    ///
    /// reg_core_1_iram0_recording_addr_1
    #[inline(always)]
    pub fn core_1_iram0_recording_addr_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_IRAM0_RECORDING_ADDR_1_MASK)
            >> Self::CORE_1_IRAM0_RECORDING_ADDR_1_OFFSET
    }
}

/// CORE_1_IRAM0_EXCEPTION_MONITOR_1 value type for batch read/modify/write operations
///
/// Use `core1iram0exceptionmonitor1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1iram0exceptionmonitor1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Iram0ExceptionMonitor1Val {
    bits: u32,
}

impl Core1Iram0ExceptionMonitor1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_IRAM0_RECORDING_LOADSTORE_1** `[25:25]`
    ///
    /// reg_core_1_iram0_recording_loadstore_1
    #[inline(always)]
    pub const fn core_1_iram0_recording_loadstore_1(&self) -> bool {
        (self.bits & Core1Iram0ExceptionMonitor1::CORE_1_IRAM0_RECORDING_LOADSTORE_1_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_RECORDING_WR_1** `[24:24]`
    ///
    /// reg_core_1_iram0_recording_wr_1
    #[inline(always)]
    pub const fn core_1_iram0_recording_wr_1(&self) -> bool {
        (self.bits & Core1Iram0ExceptionMonitor1::CORE_1_IRAM0_RECORDING_WR_1_MASK) != 0
    }

    /// Read **CORE_1_IRAM0_RECORDING_ADDR_1** `[23:0]`
    ///
    /// reg_core_1_iram0_recording_addr_1
    #[inline(always)]
    pub const fn core_1_iram0_recording_addr_1(&self) -> u32 {
        (self.bits & Core1Iram0ExceptionMonitor1::CORE_1_IRAM0_RECORDING_ADDR_1_MASK)
            >> Core1Iram0ExceptionMonitor1::CORE_1_IRAM0_RECORDING_ADDR_1_OFFSET
    }
}

impl Default for Core1Iram0ExceptionMonitor1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Iram0ExceptionMonitor1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Iram0ExceptionMonitor1Val {
        Core1Iram0ExceptionMonitor1Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_0 Register
///
/// exception monitor status register2
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_BYTEEN_0** `[16:1]` (RO)
///
///   reg_core_1_dram0_recording_byteen_0
///
/// - **CORE_1_DRAM0_RECORDING_WR_0** `[0]` (RO)
///
///   reg_core_1_dram0_recording_wr_0
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor0 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_BYTEEN_0**
    ///
    /// reg_core_1_dram0_recording_byteen_0
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const CORE_1_DRAM0_RECORDING_BYTEEN_0_OFFSET: u32 = 1;
    pub const CORE_1_DRAM0_RECORDING_BYTEEN_0_MASK: u32 = 0x0001fffe;

    /// **CORE_1_DRAM0_RECORDING_WR_0**
    ///
    /// reg_core_1_dram0_recording_wr_0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_DRAM0_RECORDING_WR_0_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_WR_0_MASK: u32 = 0x00000001;
    pub const CORE_1_DRAM0_RECORDING_WR_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_BYTEEN_0** `[16:1]` (RO)
    ///
    /// reg_core_1_dram0_recording_byteen_0
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_BYTEEN_0_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_BYTEEN_0_OFFSET
    }

    /// **CORE_1_DRAM0_RECORDING_WR_0** `[0]` (RO)
    ///
    /// reg_core_1_dram0_recording_wr_0
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_0(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_WR_0_MASK) != 0
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_0 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor0Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_BYTEEN_0** `[16:1]`
    ///
    /// reg_core_1_dram0_recording_byteen_0
    #[inline(always)]
    pub const fn core_1_dram0_recording_byteen_0(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor0::CORE_1_DRAM0_RECORDING_BYTEEN_0_MASK)
            >> Core1Dram0ExceptionMonitor0::CORE_1_DRAM0_RECORDING_BYTEEN_0_OFFSET
    }

    /// Read **CORE_1_DRAM0_RECORDING_WR_0** `[0:0]`
    ///
    /// reg_core_1_dram0_recording_wr_0
    #[inline(always)]
    pub const fn core_1_dram0_recording_wr_0(&self) -> bool {
        (self.bits & Core1Dram0ExceptionMonitor0::CORE_1_DRAM0_RECORDING_WR_0_MASK) != 0
    }
}

impl Default for Core1Dram0ExceptionMonitor0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor0Val {
        Core1Dram0ExceptionMonitor0Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_1 Register
///
/// exception monitor status register3
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
///
///   reg_core_1_dram0_recording_addr_0
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor1 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_ADDR_0**
    ///
    /// reg_core_1_dram0_recording_addr_0
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_1_DRAM0_RECORDING_ADDR_0_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_ADDR_0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_ADDR_0** `[23:0]` (RO)
    ///
    /// reg_core_1_dram0_recording_addr_0
    #[inline(always)]
    pub fn core_1_dram0_recording_addr_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_ADDR_0_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_ADDR_0_OFFSET
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_1 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor1Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_ADDR_0** `[23:0]`
    ///
    /// reg_core_1_dram0_recording_addr_0
    #[inline(always)]
    pub const fn core_1_dram0_recording_addr_0(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor1::CORE_1_DRAM0_RECORDING_ADDR_0_MASK)
            >> Core1Dram0ExceptionMonitor1::CORE_1_DRAM0_RECORDING_ADDR_0_OFFSET
    }
}

impl Default for Core1Dram0ExceptionMonitor1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor1Val {
        Core1Dram0ExceptionMonitor1Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_2 Register
///
/// exception monitor status register4
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_PC_0** `[31:0]` (RO)
///
///   reg_core_1_dram0_recording_pc_0
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor2 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_PC_0**
    ///
    /// reg_core_1_dram0_recording_pc_0
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_DRAM0_RECORDING_PC_0_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_PC_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_PC_0** `[31:0]` (RO)
    ///
    /// reg_core_1_dram0_recording_pc_0
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_PC_0_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_PC_0_OFFSET
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_2 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor2Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_PC_0** `[31:0]`
    ///
    /// reg_core_1_dram0_recording_pc_0
    #[inline(always)]
    pub const fn core_1_dram0_recording_pc_0(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor2::CORE_1_DRAM0_RECORDING_PC_0_MASK)
            >> Core1Dram0ExceptionMonitor2::CORE_1_DRAM0_RECORDING_PC_0_OFFSET
    }
}

impl Default for Core1Dram0ExceptionMonitor2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor2Val {
        Core1Dram0ExceptionMonitor2Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_3 Register
///
/// exception monitor status register5
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_BYTEEN_1** `[16:1]` (RO)
///
///   reg_core_1_dram0_recording_byteen_1
///
/// - **CORE_1_DRAM0_RECORDING_WR_1** `[0]` (RO)
///
///   reg_core_1_dram0_recording_wr_1
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor3 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_BYTEEN_1**
    ///
    /// reg_core_1_dram0_recording_byteen_1
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const CORE_1_DRAM0_RECORDING_BYTEEN_1_OFFSET: u32 = 1;
    pub const CORE_1_DRAM0_RECORDING_BYTEEN_1_MASK: u32 = 0x0001fffe;

    /// **CORE_1_DRAM0_RECORDING_WR_1**
    ///
    /// reg_core_1_dram0_recording_wr_1
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_DRAM0_RECORDING_WR_1_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_WR_1_MASK: u32 = 0x00000001;
    pub const CORE_1_DRAM0_RECORDING_WR_1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_BYTEEN_1** `[16:1]` (RO)
    ///
    /// reg_core_1_dram0_recording_byteen_1
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_BYTEEN_1_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_BYTEEN_1_OFFSET
    }

    /// **CORE_1_DRAM0_RECORDING_WR_1** `[0]` (RO)
    ///
    /// reg_core_1_dram0_recording_wr_1
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_1(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_WR_1_MASK) != 0
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_3 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor3Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_BYTEEN_1** `[16:1]`
    ///
    /// reg_core_1_dram0_recording_byteen_1
    #[inline(always)]
    pub const fn core_1_dram0_recording_byteen_1(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor3::CORE_1_DRAM0_RECORDING_BYTEEN_1_MASK)
            >> Core1Dram0ExceptionMonitor3::CORE_1_DRAM0_RECORDING_BYTEEN_1_OFFSET
    }

    /// Read **CORE_1_DRAM0_RECORDING_WR_1** `[0:0]`
    ///
    /// reg_core_1_dram0_recording_wr_1
    #[inline(always)]
    pub const fn core_1_dram0_recording_wr_1(&self) -> bool {
        (self.bits & Core1Dram0ExceptionMonitor3::CORE_1_DRAM0_RECORDING_WR_1_MASK) != 0
    }
}

impl Default for Core1Dram0ExceptionMonitor3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor3Val {
        Core1Dram0ExceptionMonitor3Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_4 Register
///
/// exception monitor status register6
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
///
///   reg_core_1_dram0_recording_addr_1
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor4 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_ADDR_1**
    ///
    /// reg_core_1_dram0_recording_addr_1
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const CORE_1_DRAM0_RECORDING_ADDR_1_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_ADDR_1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_ADDR_1** `[23:0]` (RO)
    ///
    /// reg_core_1_dram0_recording_addr_1
    #[inline(always)]
    pub fn core_1_dram0_recording_addr_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_ADDR_1_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_ADDR_1_OFFSET
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_4 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor4Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_ADDR_1** `[23:0]`
    ///
    /// reg_core_1_dram0_recording_addr_1
    #[inline(always)]
    pub const fn core_1_dram0_recording_addr_1(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor4::CORE_1_DRAM0_RECORDING_ADDR_1_MASK)
            >> Core1Dram0ExceptionMonitor4::CORE_1_DRAM0_RECORDING_ADDR_1_OFFSET
    }
}

impl Default for Core1Dram0ExceptionMonitor4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor4Val {
        Core1Dram0ExceptionMonitor4Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_5 Register
///
/// exception monitor status register7
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **CORE_1_DRAM0_RECORDING_PC_1** `[31:0]` (RO)
///
///   reg_core_1_dram0_recording_pc_1
#[repr(transparent)]
pub struct Core1Dram0ExceptionMonitor5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1Dram0ExceptionMonitor5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1Dram0ExceptionMonitor5 {
    // ======== Field Constants ========

    /// **CORE_1_DRAM0_RECORDING_PC_1**
    ///
    /// reg_core_1_dram0_recording_pc_1
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_DRAM0_RECORDING_PC_1_OFFSET: u32 = 0;
    pub const CORE_1_DRAM0_RECORDING_PC_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_DRAM0_RECORDING_PC_1** `[31:0]` (RO)
    ///
    /// reg_core_1_dram0_recording_pc_1
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_DRAM0_RECORDING_PC_1_MASK)
            >> Self::CORE_1_DRAM0_RECORDING_PC_1_OFFSET
    }
}

/// CORE_1_DRAM0_EXCEPTION_MONITOR_5 value type for batch read/modify/write operations
///
/// Use `core1dram0exceptionmonitor5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1dram0exceptionmonitor5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1Dram0ExceptionMonitor5Val {
    bits: u32,
}

impl Core1Dram0ExceptionMonitor5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DRAM0_RECORDING_PC_1** `[31:0]`
    ///
    /// reg_core_1_dram0_recording_pc_1
    #[inline(always)]
    pub const fn core_1_dram0_recording_pc_1(&self) -> u32 {
        (self.bits & Core1Dram0ExceptionMonitor5::CORE_1_DRAM0_RECORDING_PC_1_MASK)
            >> Core1Dram0ExceptionMonitor5::CORE_1_DRAM0_RECORDING_PC_1_OFFSET
    }
}

impl Default for Core1Dram0ExceptionMonitor5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1Dram0ExceptionMonitor5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1Dram0ExceptionMonitor5Val {
        Core1Dram0ExceptionMonitor5Val::from_bits(self.value.read_bits())
    }
}

/// CORE_1_LASTPC_BEFORE_EXCEPTION Register
///
/// cpu status register
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **CORE_1_LASTPC_BEFORE_EXC** `[31:0]` (RO)
///
///   cpu's lastpc before exception
#[repr(transparent)]
pub struct Core1LastpcBeforeException {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1LastpcBeforeException {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1LastpcBeforeException {
    // ======== Field Constants ========

    /// **CORE_1_LASTPC_BEFORE_EXC**
    ///
    /// cpu's lastpc before exception
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE_1_LASTPC_BEFORE_EXC_OFFSET: u32 = 0;
    pub const CORE_1_LASTPC_BEFORE_EXC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE_1_LASTPC_BEFORE_EXC** `[31:0]` (RO)
    ///
    /// cpu's lastpc before exception
    #[inline(always)]
    pub fn core_1_lastpc_before_exc(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_1_LASTPC_BEFORE_EXC_MASK)
            >> Self::CORE_1_LASTPC_BEFORE_EXC_OFFSET
    }
}

/// CORE_1_LASTPC_BEFORE_EXCEPTION value type for batch read/modify/write operations
///
/// Use `core1lastpcbeforeexception.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1lastpcbeforeexception.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1LastpcBeforeExceptionVal {
    bits: u32,
}

impl Core1LastpcBeforeExceptionVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_LASTPC_BEFORE_EXC** `[31:0]`
    ///
    /// cpu's lastpc before exception
    #[inline(always)]
    pub const fn core_1_lastpc_before_exc(&self) -> u32 {
        (self.bits & Core1LastpcBeforeException::CORE_1_LASTPC_BEFORE_EXC_MASK)
            >> Core1LastpcBeforeException::CORE_1_LASTPC_BEFORE_EXC_OFFSET
    }
}

impl Default for Core1LastpcBeforeExceptionVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1LastpcBeforeException {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1LastpcBeforeExceptionVal {
        Core1LastpcBeforeExceptionVal::from_bits(self.value.read_bits())
    }
}

/// CORE_1_DEBUG_MODE Register
///
/// cpu status register
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **CORE_1_DEBUG_MODULE_ACTIVE** `[1]` (RO)
///
///   cpu debug_module active status
///
/// - **CORE_1_DEBUG_MODE** `[0]` (RO)
///
///   cpu debug mode status, 1 means cpu enter debug mode.
#[repr(transparent)]
pub struct Core1DebugMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1DebugMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1DebugMode {
    // ======== Field Constants ========

    /// **CORE_1_DEBUG_MODULE_ACTIVE**
    ///
    /// cpu debug_module active status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE_1_DEBUG_MODULE_ACTIVE_OFFSET: u32 = 1;
    pub const CORE_1_DEBUG_MODULE_ACTIVE_MASK: u32 = 0x00000002;
    pub const CORE_1_DEBUG_MODULE_ACTIVE: u32 = 1 << 1;

    /// **CORE_1_DEBUG_MODE**
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_1_DEBUG_MODE_OFFSET: u32 = 0;
    pub const CORE_1_DEBUG_MODE_MASK: u32 = 0x00000001;
    pub const CORE_1_DEBUG_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_1_DEBUG_MODULE_ACTIVE** `[1]` (RO)
    ///
    /// cpu debug_module active status
    #[inline(always)]
    pub fn core_1_debug_module_active(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DEBUG_MODULE_ACTIVE_MASK) != 0
    }

    /// **CORE_1_DEBUG_MODE** `[0]` (RO)
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    #[inline(always)]
    pub fn core_1_debug_mode(&self) -> bool {
        (self.value.read_bits() & Self::CORE_1_DEBUG_MODE_MASK) != 0
    }
}

/// CORE_1_DEBUG_MODE value type for batch read/modify/write operations
///
/// Use `core1debugmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1debugmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1DebugModeVal {
    bits: u32,
}

impl Core1DebugModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_1_DEBUG_MODULE_ACTIVE** `[1:1]`
    ///
    /// cpu debug_module active status
    #[inline(always)]
    pub const fn core_1_debug_module_active(&self) -> bool {
        (self.bits & Core1DebugMode::CORE_1_DEBUG_MODULE_ACTIVE_MASK) != 0
    }

    /// Read **CORE_1_DEBUG_MODE** `[0:0]`
    ///
    /// cpu debug mode status, 1 means cpu enter debug mode.
    #[inline(always)]
    pub const fn core_1_debug_mode(&self) -> bool {
        (self.bits & Core1DebugMode::CORE_1_DEBUG_MODE_MASK) != 0
    }
}

impl Default for Core1DebugModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1DebugMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1DebugModeVal {
        Core1DebugModeVal::from_bits(self.value.read_bits())
    }
}

/// CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 Register
///
/// exception monitor status register6
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0** `[19:0]` (RW)
///
///   reg_core_x_iram0_dram0_limit_cycle_0
#[repr(transparent)]
pub struct CoreXIram0Dram0ExceptionMonitor0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreXIram0Dram0ExceptionMonitor0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreXIram0Dram0ExceptionMonitor0 {
    // ======== Field Constants ========

    /// **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0**
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_0
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_OFFSET: u32 = 0;
    pub const CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0** `[19:0]` (RW)
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_0
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK)
            >> Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_OFFSET
    }

    /// Write **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0** `[19:0]` (RW)
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_0
    #[inline(always)]
    pub fn write_core_x_iram0_dram0_limit_cycle_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK)
                | ((value << Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_OFFSET)
                    & Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK)
        });
    }
}

/// CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 value type for batch read/modify/write operations
///
/// Use `corexiram0dram0exceptionmonitor0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `corexiram0dram0exceptionmonitor0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreXIram0Dram0ExceptionMonitor0Val {
    bits: u32,
}

impl CoreXIram0Dram0ExceptionMonitor0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0** `[19:0]`
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_0
    #[inline(always)]
    pub const fn core_x_iram0_dram0_limit_cycle_0(&self) -> u32 {
        (self.bits & CoreXIram0Dram0ExceptionMonitor0::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK)
            >> CoreXIram0Dram0ExceptionMonitor0::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_OFFSET
    }

    /// Set **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0** `[19:0]`
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_0
    #[inline(always)]
    pub const fn with_core_x_iram0_dram0_limit_cycle_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !CoreXIram0Dram0ExceptionMonitor0::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK)
                | ((value
                    << CoreXIram0Dram0ExceptionMonitor0::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_OFFSET)
                    & CoreXIram0Dram0ExceptionMonitor0::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_MASK),
        }
    }
}

impl Default for CoreXIram0Dram0ExceptionMonitor0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreXIram0Dram0ExceptionMonitor0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreXIram0Dram0ExceptionMonitor0Val {
        CoreXIram0Dram0ExceptionMonitor0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreXIram0Dram0ExceptionMonitor0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(CoreXIram0Dram0ExceptionMonitor0Val) -> CoreXIram0Dram0ExceptionMonitor0Val,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 Register
///
/// exception monitor status register7
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1** `[19:0]` (RW)
///
///   reg_core_x_iram0_dram0_limit_cycle_1
#[repr(transparent)]
pub struct CoreXIram0Dram0ExceptionMonitor1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreXIram0Dram0ExceptionMonitor1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreXIram0Dram0ExceptionMonitor1 {
    // ======== Field Constants ========

    /// **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1**
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_1
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_OFFSET: u32 = 0;
    pub const CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1** `[19:0]` (RW)
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_1
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK)
            >> Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_OFFSET
    }

    /// Write **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1** `[19:0]` (RW)
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_1
    #[inline(always)]
    pub fn write_core_x_iram0_dram0_limit_cycle_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK)
                | ((value << Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_OFFSET)
                    & Self::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK)
        });
    }
}

/// CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 value type for batch read/modify/write operations
///
/// Use `corexiram0dram0exceptionmonitor1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `corexiram0dram0exceptionmonitor1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreXIram0Dram0ExceptionMonitor1Val {
    bits: u32,
}

impl CoreXIram0Dram0ExceptionMonitor1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1** `[19:0]`
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_1
    #[inline(always)]
    pub const fn core_x_iram0_dram0_limit_cycle_1(&self) -> u32 {
        (self.bits & CoreXIram0Dram0ExceptionMonitor1::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK)
            >> CoreXIram0Dram0ExceptionMonitor1::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_OFFSET
    }

    /// Set **CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1** `[19:0]`
    ///
    /// reg_core_x_iram0_dram0_limit_cycle_1
    #[inline(always)]
    pub const fn with_core_x_iram0_dram0_limit_cycle_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !CoreXIram0Dram0ExceptionMonitor1::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK)
                | ((value
                    << CoreXIram0Dram0ExceptionMonitor1::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_OFFSET)
                    & CoreXIram0Dram0ExceptionMonitor1::CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_MASK),
        }
    }
}

impl Default for CoreXIram0Dram0ExceptionMonitor1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreXIram0Dram0ExceptionMonitor1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreXIram0Dram0ExceptionMonitor1Val {
        CoreXIram0Dram0ExceptionMonitor1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreXIram0Dram0ExceptionMonitor1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(CoreXIram0Dram0ExceptionMonitor1Val) -> CoreXIram0Dram0ExceptionMonitor1Val,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// CLOCK_GATE Register
///
/// clock register
///
/// Address offset: 0x0108
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   Set 1 force on the clock gate
#[repr(transparent)]
pub struct ClockGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClockGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClockGate {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Set 1 force on the clock gate
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// Set 1 force on the clock gate
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

/// CLOCK_GATE value type for batch read/modify/write operations
///
/// Use `clockgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clockgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClockGateVal {
    bits: u32,
}

impl ClockGateVal {
    /// Create from raw bits
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

    /// Read **CLK_EN** `[0:0]`
    ///
    /// Set 1 force on the clock gate
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClockGate::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// Set 1 force on the clock gate
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClockGate::CLK_EN_MASK
            } else {
                self.bits & !ClockGate::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClockGateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClockGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClockGateVal {
        ClockGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClockGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClockGateVal) -> ClockGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// version register
///
/// Address offset: 0x03fc
/// Reset value: 0x02109130
///
/// ## Fields
///
/// - **ASSIST_DEBUG_DATE** `[27:0]` (RW)
///
///   version register
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

    /// **ASSIST_DEBUG_DATE**
    ///
    /// version register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const ASSIST_DEBUG_DATE_OFFSET: u32 = 0;
    pub const ASSIST_DEBUG_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **ASSIST_DEBUG_DATE** `[27:0]` (RW)
    ///
    /// version register
    #[inline(always)]
    pub fn assist_debug_date(&self) -> u32 {
        (self.value.read_bits() & Self::ASSIST_DEBUG_DATE_MASK) >> Self::ASSIST_DEBUG_DATE_OFFSET
    }

    /// Write **ASSIST_DEBUG_DATE** `[27:0]` (RW)
    ///
    /// version register
    #[inline(always)]
    pub fn write_assist_debug_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ASSIST_DEBUG_DATE_MASK)
                | ((value << Self::ASSIST_DEBUG_DATE_OFFSET) & Self::ASSIST_DEBUG_DATE_MASK)
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
        Self { bits: 0x02109130 }
    }

    /// Read **ASSIST_DEBUG_DATE** `[27:0]`
    ///
    /// version register
    #[inline(always)]
    pub const fn assist_debug_date(&self) -> u32 {
        (self.bits & Date::ASSIST_DEBUG_DATE_MASK) >> Date::ASSIST_DEBUG_DATE_OFFSET
    }

    /// Set **ASSIST_DEBUG_DATE** `[27:0]`
    ///
    /// version register
    #[inline(always)]
    pub const fn with_assist_debug_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::ASSIST_DEBUG_DATE_MASK)
                | ((value << Date::ASSIST_DEBUG_DATE_OFFSET) & Date::ASSIST_DEBUG_DATE_MASK),
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
    /// core0 monitor enable configuration register
    pub core0_intr_ena: Core0IntrEna,
    /// 0x0004
    /// core0 monitor interrupt status register
    pub core0_intr_raw: Core0IntrRaw,
    /// 0x0008
    /// core0 monitor interrupt enable register
    pub core0_intr_rls: Core0IntrRls,
    /// 0x000c
    /// core0 monitor interrupt clr register
    pub core0_intr_clr: Core0IntrClr,
    /// 0x0010
    /// core0 dram0 region0 addr configuration register
    pub core0_area_dram00_min: Core0AreaDram00Min,
    /// 0x0014
    /// core0 dram0 region0 addr configuration register
    pub core0_area_dram00_max: Core0AreaDram00Max,
    /// 0x0018
    /// core0 dram0 region1 addr configuration register
    pub core0_area_dram01_min: Core0AreaDram01Min,
    /// 0x001c
    /// core0 dram0 region1 addr configuration register
    pub core0_area_dram01_max: Core0AreaDram01Max,
    /// 0x0020
    /// core0 PIF region0 addr configuration register
    pub core0_area_pif0_min: Core0AreaPif0Min,
    /// 0x0024
    /// core0 PIF region0 addr configuration register
    pub core0_area_pif0_max: Core0AreaPif0Max,
    /// 0x0028
    /// core0 PIF region1 addr configuration register
    pub core0_area_pif1_min: Core0AreaPif1Min,
    /// 0x002c
    /// core0 PIF region1 addr configuration register
    pub core0_area_pif1_max: Core0AreaPif1Max,
    /// 0x0030
    /// core0 area pc status register
    pub core0_area_pc: Core0AreaPc,
    /// 0x0034
    /// core0 area sp status register
    pub core0_area_sp: Core0AreaSp,
    /// 0x0038
    /// stack min value
    pub core0_sp_min: Core0SpMin,
    /// 0x003c
    /// stack max value
    pub core0_sp_max: Core0SpMax,
    /// 0x0040
    /// stack monitor pc status register
    pub core0_sp_pc: Core0SpPc,
    /// 0x0044
    /// record enable configuration register
    pub core0_rcd_en: Core0RcdEn,
    /// 0x0048
    /// record status regsiter
    pub core0_rcd_pdebugpc: Core0RcdPdebugpc,
    /// 0x004c
    /// record status regsiter
    pub core0_rcd_pdebugsp: Core0RcdPdebugsp,
    /// 0x0050
    /// exception monitor status register0
    pub core0_iram0_exception_monitor0: Core0Iram0ExceptionMonitor0,
    /// 0x0054
    /// exception monitor status register1
    pub core0_iram0_exception_monitor1: Core0Iram0ExceptionMonitor1,
    /// 0x0058
    /// exception monitor status register2
    pub core0_dram0_exception_monitor0: Core0Dram0ExceptionMonitor0,
    /// 0x005c
    /// exception monitor status register3
    pub core0_dram0_exception_monitor1: Core0Dram0ExceptionMonitor1,
    /// 0x0060
    /// exception monitor status register4
    pub core0_dram0_exception_monitor2: Core0Dram0ExceptionMonitor2,
    /// 0x0064
    /// exception monitor status register5
    pub core0_dram0_exception_monitor3: Core0Dram0ExceptionMonitor3,
    /// 0x0068
    /// exception monitor status register6
    pub core0_dram0_exception_monitor4: Core0Dram0ExceptionMonitor4,
    /// 0x006c
    /// exception monitor status register7
    pub core0_dram0_exception_monitor5: Core0Dram0ExceptionMonitor5,
    /// 0x0070
    /// cpu status register
    pub core0_lastpc_before_exception: Core0LastpcBeforeException,
    /// 0x0074
    /// cpu status register
    pub core0_debug_mode: Core0DebugMode,
    _reserved0: [u8; 8],
    /// 0x0080
    /// core1 monitor enable configuration register
    pub core1_intr_ena: Core1IntrEna,
    /// 0x0084
    /// core1 monitor interrupt status register
    pub core1_intr_raw: Core1IntrRaw,
    /// 0x0088
    /// core1 monitor interrupt enable register
    pub core1_intr_rls: Core1IntrRls,
    /// 0x008c
    /// core1 monitor interrupt clr register
    pub core1_intr_clr: Core1IntrClr,
    /// 0x0090
    /// core1 dram0 region0 addr configuration register
    pub core1_area_dram00_min: Core1AreaDram00Min,
    /// 0x0094
    /// core1 dram0 region0 addr configuration register
    pub core1_area_dram00_max: Core1AreaDram00Max,
    /// 0x0098
    /// core1 dram0 region1 addr configuration register
    pub core1_area_dram01_min: Core1AreaDram01Min,
    /// 0x009c
    /// core1 dram0 region1 addr configuration register
    pub core1_area_dram01_max: Core1AreaDram01Max,
    /// 0x00a0
    /// core1 PIF region0 addr configuration register
    pub core1_area_pif0_min: Core1AreaPif0Min,
    /// 0x00a4
    /// core1 PIF region0 addr configuration register
    pub core1_area_pif0_max: Core1AreaPif0Max,
    /// 0x00a8
    /// core1 PIF region1 addr configuration register
    pub core1_area_pif1_min: Core1AreaPif1Min,
    /// 0x00ac
    /// core1 PIF region1 addr configuration register
    pub core1_area_pif1_max: Core1AreaPif1Max,
    /// 0x00b0
    /// core1 area pc status register
    pub core1_area_pc: Core1AreaPc,
    /// 0x00b4
    /// core1 area sp status register
    pub core1_area_sp: Core1AreaSp,
    /// 0x00b8
    /// stack min value
    pub core1_sp_min: Core1SpMin,
    /// 0x00bc
    /// stack max value
    pub core1_sp_max: Core1SpMax,
    /// 0x00c0
    /// stack monitor pc status register
    pub core1_sp_pc: Core1SpPc,
    /// 0x00c4
    /// record enable configuration register
    pub core1_rcd_en: Core1RcdEn,
    /// 0x00c8
    /// record status regsiter
    pub core1_rcd_pdebugpc: Core1RcdPdebugpc,
    /// 0x00cc
    /// record status regsiter
    pub core1_rcd_pdebugsp: Core1RcdPdebugsp,
    /// 0x00d0
    /// exception monitor status register0
    pub core1_iram0_exception_monitor0: Core1Iram0ExceptionMonitor0,
    /// 0x00d4
    /// exception monitor status register1
    pub core1_iram0_exception_monitor1: Core1Iram0ExceptionMonitor1,
    /// 0x00d8
    /// exception monitor status register2
    pub core1_dram0_exception_monitor0: Core1Dram0ExceptionMonitor0,
    /// 0x00dc
    /// exception monitor status register3
    pub core1_dram0_exception_monitor1: Core1Dram0ExceptionMonitor1,
    /// 0x00e0
    /// exception monitor status register4
    pub core1_dram0_exception_monitor2: Core1Dram0ExceptionMonitor2,
    /// 0x00e4
    /// exception monitor status register5
    pub core1_dram0_exception_monitor3: Core1Dram0ExceptionMonitor3,
    /// 0x00e8
    /// exception monitor status register6
    pub core1_dram0_exception_monitor4: Core1Dram0ExceptionMonitor4,
    /// 0x00ec
    /// exception monitor status register7
    pub core1_dram0_exception_monitor5: Core1Dram0ExceptionMonitor5,
    /// 0x00f0
    /// cpu status register
    pub core1_lastpc_before_exception: Core1LastpcBeforeException,
    /// 0x00f4
    /// cpu status register
    pub core1_debug_mode: Core1DebugMode,
    _reserved1: [u8; 8],
    /// 0x0100
    /// exception monitor status register6
    pub core_x_iram0_dram0_exception_monitor0: CoreXIram0Dram0ExceptionMonitor0,
    /// 0x0104
    /// exception monitor status register7
    pub core_x_iram0_dram0_exception_monitor1: CoreXIram0Dram0ExceptionMonitor1,
    /// 0x0108
    /// clock register
    pub clock_gate: ClockGate,
    _reserved2: [u8; 752],
    /// 0x03fc
    /// version register
    pub date: Date,
}
