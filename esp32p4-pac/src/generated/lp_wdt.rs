//! LP_WDT
//! Low-power Watchdog Timer

#![allow(dead_code)]

pub const BASE: u32 = 0x50116000;

/// CONFIG0 Register
///
/// need_des
///
/// Address offset: 0x0000
/// Reset value: 0x00013214
///
/// ## Fields
///
/// - **WDT_EN** `[31]` (RW)
///
///   need_des
///
/// - **WDT_STG0** `[30:28]` (RW)
///
///   need_des
///
/// - **WDT_STG1** `[27:25]` (RW)
///
///   need_des
///
/// - **WDT_STG2** `[24:22]` (RW)
///
///   need_des
///
/// - **WDT_STG3** `[21:19]` (RW)
///
///   need_des
///
/// - **WDT_CPU_RESET_LENGTH** `[18:16]` (RW)
///
///   need_des
///
/// - **WDT_SYS_RESET_LENGTH** `[15:13]` (RW)
///
///   need_des
///
/// - **WDT_FLASHBOOT_MOD_EN** `[12]` (RW)
///
///   need_des
///
/// - **WDT_PROCPU_RESET_EN** `[11]` (RW)
///
///   need_des
///
/// - **WDT_APPCPU_RESET_EN** `[10]` (RW)
///
///   need_des
///
/// - **WDT_PAUSE_IN_SLP** `[9]` (RW)
///
///   need_des
///
/// - **WDT_CHIP_RESET_EN** `[8]` (RW)
///
///   need_des
///
/// - **WDT_CHIP_RESET_WIDTH** `[7:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Config0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config0 {
    // ======== Field Constants ========

    /// **WDT_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const WDT_EN_OFFSET: u32 = 31;
    pub const WDT_EN_MASK: u32 = 0x80000000;
    pub const WDT_EN: u32 = 1 << 31;

    /// **WDT_STG0**
    ///
    /// need_des
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const WDT_STG0_OFFSET: u32 = 28;
    pub const WDT_STG0_MASK: u32 = 0x70000000;

    /// **WDT_STG1**
    ///
    /// need_des
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const WDT_STG1_OFFSET: u32 = 25;
    pub const WDT_STG1_MASK: u32 = 0x0e000000;

    /// **WDT_STG2**
    ///
    /// need_des
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const WDT_STG2_OFFSET: u32 = 22;
    pub const WDT_STG2_MASK: u32 = 0x01c00000;

    /// **WDT_STG3**
    ///
    /// need_des
    ///
    /// Bits `[21:19]`, mask `0x00380000`
    pub const WDT_STG3_OFFSET: u32 = 19;
    pub const WDT_STG3_MASK: u32 = 0x00380000;

    /// **WDT_CPU_RESET_LENGTH**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const WDT_CPU_RESET_LENGTH_OFFSET: u32 = 16;
    pub const WDT_CPU_RESET_LENGTH_MASK: u32 = 0x00070000;

    /// **WDT_SYS_RESET_LENGTH**
    ///
    /// need_des
    ///
    /// Bits `[15:13]`, mask `0x0000e000`
    pub const WDT_SYS_RESET_LENGTH_OFFSET: u32 = 13;
    pub const WDT_SYS_RESET_LENGTH_MASK: u32 = 0x0000e000;

    /// **WDT_FLASHBOOT_MOD_EN**
    ///
    /// need_des
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const WDT_FLASHBOOT_MOD_EN_OFFSET: u32 = 12;
    pub const WDT_FLASHBOOT_MOD_EN_MASK: u32 = 0x00001000;
    pub const WDT_FLASHBOOT_MOD_EN: u32 = 1 << 12;

    /// **WDT_PROCPU_RESET_EN**
    ///
    /// need_des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const WDT_PROCPU_RESET_EN_OFFSET: u32 = 11;
    pub const WDT_PROCPU_RESET_EN_MASK: u32 = 0x00000800;
    pub const WDT_PROCPU_RESET_EN: u32 = 1 << 11;

    /// **WDT_APPCPU_RESET_EN**
    ///
    /// need_des
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const WDT_APPCPU_RESET_EN_OFFSET: u32 = 10;
    pub const WDT_APPCPU_RESET_EN_MASK: u32 = 0x00000400;
    pub const WDT_APPCPU_RESET_EN: u32 = 1 << 10;

    /// **WDT_PAUSE_IN_SLP**
    ///
    /// need_des
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const WDT_PAUSE_IN_SLP_OFFSET: u32 = 9;
    pub const WDT_PAUSE_IN_SLP_MASK: u32 = 0x00000200;
    pub const WDT_PAUSE_IN_SLP: u32 = 1 << 9;

    /// **WDT_CHIP_RESET_EN**
    ///
    /// need_des
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const WDT_CHIP_RESET_EN_OFFSET: u32 = 8;
    pub const WDT_CHIP_RESET_EN_MASK: u32 = 0x00000100;
    pub const WDT_CHIP_RESET_EN: u32 = 1 << 8;

    /// **WDT_CHIP_RESET_WIDTH**
    ///
    /// need_des
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WDT_CHIP_RESET_WIDTH_OFFSET: u32 = 0;
    pub const WDT_CHIP_RESET_WIDTH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **WDT_EN** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_EN_MASK) != 0
    }

    /// Set WDT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_en(&self) {
        self.value.modify_bits(|r| r | Self::WDT_EN_MASK);
    }

    /// Clear WDT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_en(&self) {
        self.value.modify_bits(|r| r & !Self::WDT_EN_MASK);
    }

    /// **WDT_STG0** `[30:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg0(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG0_MASK) >> Self::WDT_STG0_OFFSET
    }

    /// Write **WDT_STG0** `[30:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG0_MASK) | ((value << Self::WDT_STG0_OFFSET) & Self::WDT_STG0_MASK)
        });
    }

    /// **WDT_STG1** `[27:25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg1(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG1_MASK) >> Self::WDT_STG1_OFFSET
    }

    /// Write **WDT_STG1** `[27:25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG1_MASK) | ((value << Self::WDT_STG1_OFFSET) & Self::WDT_STG1_MASK)
        });
    }

    /// **WDT_STG2** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg2(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG2_MASK) >> Self::WDT_STG2_OFFSET
    }

    /// Write **WDT_STG2** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG2_MASK) | ((value << Self::WDT_STG2_OFFSET) & Self::WDT_STG2_MASK)
        });
    }

    /// **WDT_STG3** `[21:19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg3(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG3_MASK) >> Self::WDT_STG3_OFFSET
    }

    /// Write **WDT_STG3** `[21:19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG3_MASK) | ((value << Self::WDT_STG3_OFFSET) & Self::WDT_STG3_MASK)
        });
    }

    /// **WDT_CPU_RESET_LENGTH** `[18:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_CPU_RESET_LENGTH_MASK)
            >> Self::WDT_CPU_RESET_LENGTH_OFFSET
    }

    /// Write **WDT_CPU_RESET_LENGTH** `[18:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_cpu_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_CPU_RESET_LENGTH_MASK)
                | ((value << Self::WDT_CPU_RESET_LENGTH_OFFSET) & Self::WDT_CPU_RESET_LENGTH_MASK)
        });
    }

    /// **WDT_SYS_RESET_LENGTH** `[15:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_SYS_RESET_LENGTH_MASK)
            >> Self::WDT_SYS_RESET_LENGTH_OFFSET
    }

    /// Write **WDT_SYS_RESET_LENGTH** `[15:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_sys_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_SYS_RESET_LENGTH_MASK)
                | ((value << Self::WDT_SYS_RESET_LENGTH_OFFSET) & Self::WDT_SYS_RESET_LENGTH_MASK)
        });
    }

    /// **WDT_FLASHBOOT_MOD_EN** `[12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_FLASHBOOT_MOD_EN_MASK) != 0
    }

    /// Set WDT_FLASHBOOT_MOD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_flashboot_mod_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_FLASHBOOT_MOD_EN_MASK);
    }

    /// Clear WDT_FLASHBOOT_MOD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_flashboot_mod_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_FLASHBOOT_MOD_EN_MASK);
    }

    /// **WDT_PROCPU_RESET_EN** `[11]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_PROCPU_RESET_EN_MASK) != 0
    }

    /// Set WDT_PROCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_procpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_PROCPU_RESET_EN_MASK);
    }

    /// Clear WDT_PROCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_procpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_PROCPU_RESET_EN_MASK);
    }

    /// **WDT_APPCPU_RESET_EN** `[10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_APPCPU_RESET_EN_MASK) != 0
    }

    /// Set WDT_APPCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_appcpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_APPCPU_RESET_EN_MASK);
    }

    /// Clear WDT_APPCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_appcpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_APPCPU_RESET_EN_MASK);
    }

    /// **WDT_PAUSE_IN_SLP** `[9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> bool {
        (self.value.read_bits() & Self::WDT_PAUSE_IN_SLP_MASK) != 0
    }

    /// Set WDT_PAUSE_IN_SLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_pause_in_slp(&self) {
        self.value.modify_bits(|r| r | Self::WDT_PAUSE_IN_SLP_MASK);
    }

    /// Clear WDT_PAUSE_IN_SLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_pause_in_slp(&self) {
        self.value.modify_bits(|r| r & !Self::WDT_PAUSE_IN_SLP_MASK);
    }

    /// **WDT_CHIP_RESET_EN** `[8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_chip_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_CHIP_RESET_EN_MASK) != 0
    }

    /// Set WDT_CHIP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_chip_reset_en(&self) {
        self.value.modify_bits(|r| r | Self::WDT_CHIP_RESET_EN_MASK);
    }

    /// Clear WDT_CHIP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_chip_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_CHIP_RESET_EN_MASK);
    }

    /// **WDT_CHIP_RESET_WIDTH** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_chip_reset_width(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_CHIP_RESET_WIDTH_MASK)
            >> Self::WDT_CHIP_RESET_WIDTH_OFFSET
    }

    /// Write **WDT_CHIP_RESET_WIDTH** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_chip_reset_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_CHIP_RESET_WIDTH_MASK)
                | ((value << Self::WDT_CHIP_RESET_WIDTH_OFFSET) & Self::WDT_CHIP_RESET_WIDTH_MASK)
        });
    }
}

/// CONFIG0 value type for batch read/modify/write operations
///
/// Use `config0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Config0Val {
    bits: u32,
}

impl Config0Val {
    /// Create from raw bits
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
        Self { bits: 0x00013214 }
    }

    /// Read **WDT_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_en(&self) -> bool {
        (self.bits & Config0::WDT_EN_MASK) != 0
    }

    /// Read **WDT_STG0** `[30:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg0(&self) -> u32 {
        (self.bits & Config0::WDT_STG0_MASK) >> Config0::WDT_STG0_OFFSET
    }

    /// Read **WDT_STG1** `[27:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg1(&self) -> u32 {
        (self.bits & Config0::WDT_STG1_MASK) >> Config0::WDT_STG1_OFFSET
    }

    /// Read **WDT_STG2** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg2(&self) -> u32 {
        (self.bits & Config0::WDT_STG2_MASK) >> Config0::WDT_STG2_OFFSET
    }

    /// Read **WDT_STG3** `[21:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg3(&self) -> u32 {
        (self.bits & Config0::WDT_STG3_MASK) >> Config0::WDT_STG3_OFFSET
    }

    /// Read **WDT_CPU_RESET_LENGTH** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_cpu_reset_length(&self) -> u32 {
        (self.bits & Config0::WDT_CPU_RESET_LENGTH_MASK) >> Config0::WDT_CPU_RESET_LENGTH_OFFSET
    }

    /// Read **WDT_SYS_RESET_LENGTH** `[15:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_sys_reset_length(&self) -> u32 {
        (self.bits & Config0::WDT_SYS_RESET_LENGTH_MASK) >> Config0::WDT_SYS_RESET_LENGTH_OFFSET
    }

    /// Read **WDT_FLASHBOOT_MOD_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_flashboot_mod_en(&self) -> bool {
        (self.bits & Config0::WDT_FLASHBOOT_MOD_EN_MASK) != 0
    }

    /// Read **WDT_PROCPU_RESET_EN** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_procpu_reset_en(&self) -> bool {
        (self.bits & Config0::WDT_PROCPU_RESET_EN_MASK) != 0
    }

    /// Read **WDT_APPCPU_RESET_EN** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_appcpu_reset_en(&self) -> bool {
        (self.bits & Config0::WDT_APPCPU_RESET_EN_MASK) != 0
    }

    /// Read **WDT_PAUSE_IN_SLP** `[9:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_pause_in_slp(&self) -> bool {
        (self.bits & Config0::WDT_PAUSE_IN_SLP_MASK) != 0
    }

    /// Read **WDT_CHIP_RESET_EN** `[8:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_chip_reset_en(&self) -> bool {
        (self.bits & Config0::WDT_CHIP_RESET_EN_MASK) != 0
    }

    /// Read **WDT_CHIP_RESET_WIDTH** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_chip_reset_width(&self) -> u32 {
        (self.bits & Config0::WDT_CHIP_RESET_WIDTH_MASK) >> Config0::WDT_CHIP_RESET_WIDTH_OFFSET
    }

    /// Set **WDT_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_EN_MASK
            } else {
                self.bits & !Config0::WDT_EN_MASK
            },
        }
    }

    /// Set **WDT_STG0** `[30:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_STG0_MASK)
                | ((value << Config0::WDT_STG0_OFFSET) & Config0::WDT_STG0_MASK),
        }
    }

    /// Set **WDT_STG1** `[27:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_STG1_MASK)
                | ((value << Config0::WDT_STG1_OFFSET) & Config0::WDT_STG1_MASK),
        }
    }

    /// Set **WDT_STG2** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_STG2_MASK)
                | ((value << Config0::WDT_STG2_OFFSET) & Config0::WDT_STG2_MASK),
        }
    }

    /// Set **WDT_STG3** `[21:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_STG3_MASK)
                | ((value << Config0::WDT_STG3_OFFSET) & Config0::WDT_STG3_MASK),
        }
    }

    /// Set **WDT_CPU_RESET_LENGTH** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_cpu_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_CPU_RESET_LENGTH_MASK)
                | ((value << Config0::WDT_CPU_RESET_LENGTH_OFFSET)
                    & Config0::WDT_CPU_RESET_LENGTH_MASK),
        }
    }

    /// Set **WDT_SYS_RESET_LENGTH** `[15:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_sys_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_SYS_RESET_LENGTH_MASK)
                | ((value << Config0::WDT_SYS_RESET_LENGTH_OFFSET)
                    & Config0::WDT_SYS_RESET_LENGTH_MASK),
        }
    }

    /// Set **WDT_FLASHBOOT_MOD_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_flashboot_mod_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_FLASHBOOT_MOD_EN_MASK
            } else {
                self.bits & !Config0::WDT_FLASHBOOT_MOD_EN_MASK
            },
        }
    }

    /// Set **WDT_PROCPU_RESET_EN** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_procpu_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_PROCPU_RESET_EN_MASK
            } else {
                self.bits & !Config0::WDT_PROCPU_RESET_EN_MASK
            },
        }
    }

    /// Set **WDT_APPCPU_RESET_EN** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_appcpu_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_APPCPU_RESET_EN_MASK
            } else {
                self.bits & !Config0::WDT_APPCPU_RESET_EN_MASK
            },
        }
    }

    /// Set **WDT_PAUSE_IN_SLP** `[9:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_pause_in_slp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_PAUSE_IN_SLP_MASK
            } else {
                self.bits & !Config0::WDT_PAUSE_IN_SLP_MASK
            },
        }
    }

    /// Set **WDT_CHIP_RESET_EN** `[8:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_chip_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config0::WDT_CHIP_RESET_EN_MASK
            } else {
                self.bits & !Config0::WDT_CHIP_RESET_EN_MASK
            },
        }
    }

    /// Set **WDT_CHIP_RESET_WIDTH** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_chip_reset_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config0::WDT_CHIP_RESET_WIDTH_MASK)
                | ((value << Config0::WDT_CHIP_RESET_WIDTH_OFFSET)
                    & Config0::WDT_CHIP_RESET_WIDTH_MASK),
        }
    }
}

impl Default for Config0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Config0Val {
        Config0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Config0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Config0Val) -> Config0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG1 Register
///
/// need_des
///
/// Address offset: 0x0004
/// Reset value: 0x00030d40
///
/// ## Fields
///
/// - **WDT_STG0_HOLD** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Config1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config1 {
    // ======== Field Constants ========

    /// **WDT_STG0_HOLD**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG0_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG0_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG0_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG0_HOLD_MASK) >> Self::WDT_STG0_HOLD_OFFSET
    }

    /// Write **WDT_STG0_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg0_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG0_HOLD_MASK)
                | ((value << Self::WDT_STG0_HOLD_OFFSET) & Self::WDT_STG0_HOLD_MASK)
        });
    }
}

/// CONFIG1 value type for batch read/modify/write operations
///
/// Use `config1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Config1Val {
    bits: u32,
}

impl Config1Val {
    /// Create from raw bits
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
        Self { bits: 0x00030d40 }
    }

    /// Read **WDT_STG0_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg0_hold(&self) -> u32 {
        (self.bits & Config1::WDT_STG0_HOLD_MASK) >> Config1::WDT_STG0_HOLD_OFFSET
    }

    /// Set **WDT_STG0_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg0_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config1::WDT_STG0_HOLD_MASK)
                | ((value << Config1::WDT_STG0_HOLD_OFFSET) & Config1::WDT_STG0_HOLD_MASK),
        }
    }
}

impl Default for Config1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Config1Val {
        Config1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Config1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Config1Val) -> Config1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG2 Register
///
/// need_des
///
/// Address offset: 0x0008
/// Reset value: 0x00013880
///
/// ## Fields
///
/// - **WDT_STG1_HOLD** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Config2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config2 {
    // ======== Field Constants ========

    /// **WDT_STG1_HOLD**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG1_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG1_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG1_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG1_HOLD_MASK) >> Self::WDT_STG1_HOLD_OFFSET
    }

    /// Write **WDT_STG1_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg1_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG1_HOLD_MASK)
                | ((value << Self::WDT_STG1_HOLD_OFFSET) & Self::WDT_STG1_HOLD_MASK)
        });
    }
}

/// CONFIG2 value type for batch read/modify/write operations
///
/// Use `config2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Config2Val {
    bits: u32,
}

impl Config2Val {
    /// Create from raw bits
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
        Self { bits: 0x00013880 }
    }

    /// Read **WDT_STG1_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg1_hold(&self) -> u32 {
        (self.bits & Config2::WDT_STG1_HOLD_MASK) >> Config2::WDT_STG1_HOLD_OFFSET
    }

    /// Set **WDT_STG1_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg1_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config2::WDT_STG1_HOLD_MASK)
                | ((value << Config2::WDT_STG1_HOLD_OFFSET) & Config2::WDT_STG1_HOLD_MASK),
        }
    }
}

impl Default for Config2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Config2Val {
        Config2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Config2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Config2Val) -> Config2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG3 Register
///
/// need_des
///
/// Address offset: 0x000c
/// Reset value: 0x00000fff
///
/// ## Fields
///
/// - **WDT_STG2_HOLD** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Config3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config3 {
    // ======== Field Constants ========

    /// **WDT_STG2_HOLD**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG2_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG2_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG2_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG2_HOLD_MASK) >> Self::WDT_STG2_HOLD_OFFSET
    }

    /// Write **WDT_STG2_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg2_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG2_HOLD_MASK)
                | ((value << Self::WDT_STG2_HOLD_OFFSET) & Self::WDT_STG2_HOLD_MASK)
        });
    }
}

/// CONFIG3 value type for batch read/modify/write operations
///
/// Use `config3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Config3Val {
    bits: u32,
}

impl Config3Val {
    /// Create from raw bits
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

    /// Read **WDT_STG2_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg2_hold(&self) -> u32 {
        (self.bits & Config3::WDT_STG2_HOLD_MASK) >> Config3::WDT_STG2_HOLD_OFFSET
    }

    /// Set **WDT_STG2_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg2_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config3::WDT_STG2_HOLD_MASK)
                | ((value << Config3::WDT_STG2_HOLD_OFFSET) & Config3::WDT_STG2_HOLD_MASK),
        }
    }
}

impl Default for Config3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Config3Val {
        Config3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Config3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Config3Val) -> Config3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG4 Register
///
/// need_des
///
/// Address offset: 0x0010
/// Reset value: 0x00000fff
///
/// ## Fields
///
/// - **WDT_STG3_HOLD** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Config4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config4 {
    // ======== Field Constants ========

    /// **WDT_STG3_HOLD**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG3_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG3_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG3_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG3_HOLD_MASK) >> Self::WDT_STG3_HOLD_OFFSET
    }

    /// Write **WDT_STG3_HOLD** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_stg3_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG3_HOLD_MASK)
                | ((value << Self::WDT_STG3_HOLD_OFFSET) & Self::WDT_STG3_HOLD_MASK)
        });
    }
}

/// CONFIG4 value type for batch read/modify/write operations
///
/// Use `config4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Config4Val {
    bits: u32,
}

impl Config4Val {
    /// Create from raw bits
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

    /// Read **WDT_STG3_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_stg3_hold(&self) -> u32 {
        (self.bits & Config4::WDT_STG3_HOLD_MASK) >> Config4::WDT_STG3_HOLD_OFFSET
    }

    /// Set **WDT_STG3_HOLD** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_stg3_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config4::WDT_STG3_HOLD_MASK)
                | ((value << Config4::WDT_STG3_HOLD_OFFSET) & Config4::WDT_STG3_HOLD_MASK),
        }
    }
}

impl Default for Config4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Config4Val {
        Config4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Config4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Config4Val) -> Config4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FEED Register
///
/// need_des
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **FEED** `[31]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct Feed {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Feed {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Feed {
    // ======== Field Constants ========

    /// **FEED**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FEED_OFFSET: u32 = 31;
    pub const FEED_MASK: u32 = 0x80000000;
    pub const FEED: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **FEED** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_feed(&self) {
        self.value.write_bits(Self::FEED_MASK);
    }
}

/// FEED value type for batch read/modify/write operations
///
/// Use `feed.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `feed.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FeedVal {
    bits: u32,
}

impl FeedVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **FEED** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_feed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Feed::FEED_MASK
            } else {
                self.bits & !Feed::FEED_MASK
            },
        }
    }
}

impl Default for FeedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Feed {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FeedVal) {
        self.value.write_bits(val.bits())
    }
}

/// WPROTECT Register
///
/// need_des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **WDT_WKEY** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Wprotect {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wprotect {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wprotect {
    // ======== Field Constants ========

    /// **WDT_WKEY**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_WKEY_OFFSET: u32 = 0;
    pub const WDT_WKEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_WKEY** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn wdt_wkey(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_WKEY_MASK) >> Self::WDT_WKEY_OFFSET
    }

    /// Write **WDT_WKEY** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_wdt_wkey(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_WKEY_MASK) | ((value << Self::WDT_WKEY_OFFSET) & Self::WDT_WKEY_MASK)
        });
    }
}

/// WPROTECT value type for batch read/modify/write operations
///
/// Use `wprotect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wprotect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WprotectVal {
    bits: u32,
}

impl WprotectVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WDT_WKEY** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn wdt_wkey(&self) -> u32 {
        (self.bits & Wprotect::WDT_WKEY_MASK) >> Wprotect::WDT_WKEY_OFFSET
    }

    /// Set **WDT_WKEY** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_wdt_wkey(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wprotect::WDT_WKEY_MASK)
                | ((value << Wprotect::WDT_WKEY_OFFSET) & Wprotect::WDT_WKEY_MASK),
        }
    }
}

impl Default for WprotectVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Wprotect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WprotectVal {
        WprotectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WprotectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WprotectVal) -> WprotectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SWD_CONFIG Register
///
/// need_des
///
/// Address offset: 0x001c
/// Reset value: 0x12c00000
///
/// ## Fields
///
/// - **SWD_FEED** `[31]` (WO)
///
///   need_des
///
/// - **SWD_DISABLE** `[30]` (RW)
///
///   need_des
///
/// - **SWD_SIGNAL_WIDTH** `[29:20]` (RW)
///
///   need_des
///
/// - **SWD_RST_FLAG_CLR** `[19]` (WO)
///
///   need_des
///
/// - **SWD_AUTO_FEED_EN** `[18]` (RW)
///
///   need_des
///
/// - **SWD_RESET_FLAG** `[0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct SwdConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwdConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwdConfig {
    // ======== Field Constants ========

    /// **SWD_FEED**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SWD_FEED_OFFSET: u32 = 31;
    pub const SWD_FEED_MASK: u32 = 0x80000000;
    pub const SWD_FEED: u32 = 1 << 31;

    /// **SWD_DISABLE**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SWD_DISABLE_OFFSET: u32 = 30;
    pub const SWD_DISABLE_MASK: u32 = 0x40000000;
    pub const SWD_DISABLE: u32 = 1 << 30;

    /// **SWD_SIGNAL_WIDTH**
    ///
    /// need_des
    ///
    /// Bits `[29:20]`, mask `0x3ff00000`
    pub const SWD_SIGNAL_WIDTH_OFFSET: u32 = 20;
    pub const SWD_SIGNAL_WIDTH_MASK: u32 = 0x3ff00000;

    /// **SWD_RST_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SWD_RST_FLAG_CLR_OFFSET: u32 = 19;
    pub const SWD_RST_FLAG_CLR_MASK: u32 = 0x00080000;
    pub const SWD_RST_FLAG_CLR: u32 = 1 << 19;

    /// **SWD_AUTO_FEED_EN**
    ///
    /// need_des
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SWD_AUTO_FEED_EN_OFFSET: u32 = 18;
    pub const SWD_AUTO_FEED_EN_MASK: u32 = 0x00040000;
    pub const SWD_AUTO_FEED_EN: u32 = 1 << 18;

    /// **SWD_RESET_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SWD_RESET_FLAG_OFFSET: u32 = 0;
    pub const SWD_RESET_FLAG_MASK: u32 = 0x00000001;
    pub const SWD_RESET_FLAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SWD_FEED** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_swd_feed(&self) {
        self.value.modify_bits(|r| r | Self::SWD_FEED_MASK);
    }

    /// **SWD_DISABLE** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn swd_disable(&self) -> bool {
        (self.value.read_bits() & Self::SWD_DISABLE_MASK) != 0
    }

    /// Set SWD_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swd_disable(&self) {
        self.value.modify_bits(|r| r | Self::SWD_DISABLE_MASK);
    }

    /// Clear SWD_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swd_disable(&self) {
        self.value.modify_bits(|r| r & !Self::SWD_DISABLE_MASK);
    }

    /// **SWD_SIGNAL_WIDTH** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn swd_signal_width(&self) -> u32 {
        (self.value.read_bits() & Self::SWD_SIGNAL_WIDTH_MASK) >> Self::SWD_SIGNAL_WIDTH_OFFSET
    }

    /// Write **SWD_SIGNAL_WIDTH** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_swd_signal_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SWD_SIGNAL_WIDTH_MASK)
                | ((value << Self::SWD_SIGNAL_WIDTH_OFFSET) & Self::SWD_SIGNAL_WIDTH_MASK)
        });
    }

    /// Trigger **SWD_RST_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_swd_rst_flag_clr(&self) {
        self.value.modify_bits(|r| r | Self::SWD_RST_FLAG_CLR_MASK);
    }

    /// **SWD_AUTO_FEED_EN** `[18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> bool {
        (self.value.read_bits() & Self::SWD_AUTO_FEED_EN_MASK) != 0
    }

    /// Set SWD_AUTO_FEED_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swd_auto_feed_en(&self) {
        self.value.modify_bits(|r| r | Self::SWD_AUTO_FEED_EN_MASK);
    }

    /// Clear SWD_AUTO_FEED_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swd_auto_feed_en(&self) {
        self.value.modify_bits(|r| r & !Self::SWD_AUTO_FEED_EN_MASK);
    }

    /// **SWD_RESET_FLAG** `[0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> bool {
        (self.value.read_bits() & Self::SWD_RESET_FLAG_MASK) != 0
    }
}

/// SWD_CONFIG value type for batch read/modify/write operations
///
/// Use `swdconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swdconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwdConfigVal {
    bits: u32,
}

impl SwdConfigVal {
    /// Create from raw bits
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
        Self { bits: 0x12c00000 }
    }

    /// Read **SWD_DISABLE** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn swd_disable(&self) -> bool {
        (self.bits & SwdConfig::SWD_DISABLE_MASK) != 0
    }

    /// Read **SWD_SIGNAL_WIDTH** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn swd_signal_width(&self) -> u32 {
        (self.bits & SwdConfig::SWD_SIGNAL_WIDTH_MASK) >> SwdConfig::SWD_SIGNAL_WIDTH_OFFSET
    }

    /// Read **SWD_AUTO_FEED_EN** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn swd_auto_feed_en(&self) -> bool {
        (self.bits & SwdConfig::SWD_AUTO_FEED_EN_MASK) != 0
    }

    /// Read **SWD_RESET_FLAG** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn swd_reset_flag(&self) -> bool {
        (self.bits & SwdConfig::SWD_RESET_FLAG_MASK) != 0
    }

    /// Set **SWD_FEED** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_feed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwdConfig::SWD_FEED_MASK
            } else {
                self.bits & !SwdConfig::SWD_FEED_MASK
            },
        }
    }

    /// Set **SWD_DISABLE** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwdConfig::SWD_DISABLE_MASK
            } else {
                self.bits & !SwdConfig::SWD_DISABLE_MASK
            },
        }
    }

    /// Set **SWD_SIGNAL_WIDTH** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_signal_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwdConfig::SWD_SIGNAL_WIDTH_MASK)
                | ((value << SwdConfig::SWD_SIGNAL_WIDTH_OFFSET)
                    & SwdConfig::SWD_SIGNAL_WIDTH_MASK),
        }
    }

    /// Set **SWD_RST_FLAG_CLR** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_rst_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwdConfig::SWD_RST_FLAG_CLR_MASK
            } else {
                self.bits & !SwdConfig::SWD_RST_FLAG_CLR_MASK
            },
        }
    }

    /// Set **SWD_AUTO_FEED_EN** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_auto_feed_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwdConfig::SWD_AUTO_FEED_EN_MASK
            } else {
                self.bits & !SwdConfig::SWD_AUTO_FEED_EN_MASK
            },
        }
    }
}

impl Default for SwdConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwdConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwdConfigVal {
        SwdConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwdConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwdConfigVal) -> SwdConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SWD_WPROTECT Register
///
/// need_des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **SWD_WKEY** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct SwdWprotect {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwdWprotect {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwdWprotect {
    // ======== Field Constants ========

    /// **SWD_WKEY**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SWD_WKEY_OFFSET: u32 = 0;
    pub const SWD_WKEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SWD_WKEY** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn swd_wkey(&self) -> u32 {
        (self.value.read_bits() & Self::SWD_WKEY_MASK) >> Self::SWD_WKEY_OFFSET
    }

    /// Write **SWD_WKEY** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_swd_wkey(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SWD_WKEY_MASK) | ((value << Self::SWD_WKEY_OFFSET) & Self::SWD_WKEY_MASK)
        });
    }
}

/// SWD_WPROTECT value type for batch read/modify/write operations
///
/// Use `swdwprotect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swdwprotect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwdWprotectVal {
    bits: u32,
}

impl SwdWprotectVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SWD_WKEY** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn swd_wkey(&self) -> u32 {
        (self.bits & SwdWprotect::SWD_WKEY_MASK) >> SwdWprotect::SWD_WKEY_OFFSET
    }

    /// Set **SWD_WKEY** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_swd_wkey(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwdWprotect::SWD_WKEY_MASK)
                | ((value << SwdWprotect::SWD_WKEY_OFFSET) & SwdWprotect::SWD_WKEY_MASK),
        }
    }
}

impl Default for SwdWprotectVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SwdWprotect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwdWprotectVal {
        SwdWprotectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwdWprotectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwdWprotectVal) -> SwdWprotectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **LP_WDT_INT_RAW** `[31]` (RW)
///
///   need_des
///
/// - **SUPER_WDT_INT_RAW** `[30]` (RW)
///
///   need_des
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

    /// **LP_WDT_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_WDT_INT_RAW_OFFSET: u32 = 31;
    pub const LP_WDT_INT_RAW_MASK: u32 = 0x80000000;
    pub const LP_WDT_INT_RAW: u32 = 1 << 31;

    /// **SUPER_WDT_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SUPER_WDT_INT_RAW_OFFSET: u32 = 30;
    pub const SUPER_WDT_INT_RAW_MASK: u32 = 0x40000000;
    pub const SUPER_WDT_INT_RAW: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **LP_WDT_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_wdt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_WDT_INT_RAW_MASK) != 0
    }

    /// Set LP_WDT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_wdt_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::LP_WDT_INT_RAW_MASK);
    }

    /// Clear LP_WDT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_wdt_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::LP_WDT_INT_RAW_MASK);
    }

    /// **SUPER_WDT_INT_RAW** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn super_wdt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SUPER_WDT_INT_RAW_MASK) != 0
    }

    /// Set SUPER_WDT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_super_wdt_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SUPER_WDT_INT_RAW_MASK);
    }

    /// Clear SUPER_WDT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_super_wdt_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SUPER_WDT_INT_RAW_MASK);
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

    /// Read **LP_WDT_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_wdt_int_raw(&self) -> bool {
        (self.bits & IntRaw::LP_WDT_INT_RAW_MASK) != 0
    }

    /// Read **SUPER_WDT_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn super_wdt_int_raw(&self) -> bool {
        (self.bits & IntRaw::SUPER_WDT_INT_RAW_MASK) != 0
    }

    /// Set **LP_WDT_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_wdt_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::LP_WDT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::LP_WDT_INT_RAW_MASK
            },
        }
    }

    /// Set **SUPER_WDT_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_super_wdt_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SUPER_WDT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SUPER_WDT_INT_RAW_MASK
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
/// need_des
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **LP_WDT_INT_ST** `[31]` (RO)
///
///   need_des
///
/// - **SUPER_WDT_INT_ST** `[30]` (RO)
///
///   need_des
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

    /// **LP_WDT_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_WDT_INT_ST_OFFSET: u32 = 31;
    pub const LP_WDT_INT_ST_MASK: u32 = 0x80000000;
    pub const LP_WDT_INT_ST: u32 = 1 << 31;

    /// **SUPER_WDT_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SUPER_WDT_INT_ST_OFFSET: u32 = 30;
    pub const SUPER_WDT_INT_ST_MASK: u32 = 0x40000000;
    pub const SUPER_WDT_INT_ST: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **LP_WDT_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_wdt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_WDT_INT_ST_MASK) != 0
    }

    /// **SUPER_WDT_INT_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn super_wdt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SUPER_WDT_INT_ST_MASK) != 0
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

    /// Read **LP_WDT_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_wdt_int_st(&self) -> bool {
        (self.bits & IntSt::LP_WDT_INT_ST_MASK) != 0
    }

    /// Read **SUPER_WDT_INT_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn super_wdt_int_st(&self) -> bool {
        (self.bits & IntSt::SUPER_WDT_INT_ST_MASK) != 0
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
/// need_des
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **LP_WDT_INT_ENA** `[31]` (RW)
///
///   need_des
///
/// - **SUPER_WDT_INT_ENA** `[30]` (RW)
///
///   need_des
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

    /// **LP_WDT_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_WDT_INT_ENA_OFFSET: u32 = 31;
    pub const LP_WDT_INT_ENA_MASK: u32 = 0x80000000;
    pub const LP_WDT_INT_ENA: u32 = 1 << 31;

    /// **SUPER_WDT_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SUPER_WDT_INT_ENA_OFFSET: u32 = 30;
    pub const SUPER_WDT_INT_ENA_MASK: u32 = 0x40000000;
    pub const SUPER_WDT_INT_ENA: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **LP_WDT_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_wdt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_WDT_INT_ENA_MASK) != 0
    }

    /// Set LP_WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_wdt_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::LP_WDT_INT_ENA_MASK);
    }

    /// Clear LP_WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_wdt_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::LP_WDT_INT_ENA_MASK);
    }

    /// **SUPER_WDT_INT_ENA** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn super_wdt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SUPER_WDT_INT_ENA_MASK) != 0
    }

    /// Set SUPER_WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_super_wdt_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SUPER_WDT_INT_ENA_MASK);
    }

    /// Clear SUPER_WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_super_wdt_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SUPER_WDT_INT_ENA_MASK);
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

    /// Read **LP_WDT_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_wdt_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_WDT_INT_ENA_MASK) != 0
    }

    /// Read **SUPER_WDT_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn super_wdt_int_ena(&self) -> bool {
        (self.bits & IntEna::SUPER_WDT_INT_ENA_MASK) != 0
    }

    /// Set **LP_WDT_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_wdt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_WDT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_WDT_INT_ENA_MASK
            },
        }
    }

    /// Set **SUPER_WDT_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_super_wdt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SUPER_WDT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SUPER_WDT_INT_ENA_MASK
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
/// need_des
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **LP_WDT_INT_CLR** `[31]` (WO)
///
///   need_des
///
/// - **SUPER_WDT_INT_CLR** `[30]` (WO)
///
///   need_des
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

    /// **LP_WDT_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_WDT_INT_CLR_OFFSET: u32 = 31;
    pub const LP_WDT_INT_CLR_MASK: u32 = 0x80000000;
    pub const LP_WDT_INT_CLR: u32 = 1 << 31;

    /// **SUPER_WDT_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SUPER_WDT_INT_CLR_OFFSET: u32 = 30;
    pub const SUPER_WDT_INT_CLR_MASK: u32 = 0x40000000;
    pub const SUPER_WDT_INT_CLR: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// Trigger **LP_WDT_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_wdt_int_clr(&self) {
        self.value.write_bits(Self::LP_WDT_INT_CLR_MASK);
    }

    /// Trigger **SUPER_WDT_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_super_wdt_int_clr(&self) {
        self.value.write_bits(Self::SUPER_WDT_INT_CLR_MASK);
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

    /// Set **LP_WDT_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_wdt_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_WDT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_WDT_INT_CLR_MASK
            },
        }
    }

    /// Set **SUPER_WDT_INT_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_super_wdt_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SUPER_WDT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SUPER_WDT_INT_CLR_MASK
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

/// DATE Register
///
/// need_des
///
/// Address offset: 0x03fc
/// Reset value: 0x02112080
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   need_des
///
/// - **LP_WDT_DATE** `[30:0]` (RW)
///
///   need_des
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

    /// **CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **LP_WDT_DATE**
    ///
    /// need_des
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const LP_WDT_DATE_OFFSET: u32 = 0;
    pub const LP_WDT_DATE_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
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

    /// **LP_WDT_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_wdt_date(&self) -> u32 {
        (self.value.read_bits() & Self::LP_WDT_DATE_MASK) >> Self::LP_WDT_DATE_OFFSET
    }

    /// Write **LP_WDT_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_wdt_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_WDT_DATE_MASK)
                | ((value << Self::LP_WDT_DATE_OFFSET) & Self::LP_WDT_DATE_MASK)
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
        Self { bits: 0x02112080 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Date::CLK_EN_MASK) != 0
    }

    /// Read **LP_WDT_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_wdt_date(&self) -> u32 {
        (self.bits & Date::LP_WDT_DATE_MASK) >> Date::LP_WDT_DATE_OFFSET
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Date::CLK_EN_MASK
            } else {
                self.bits & !Date::CLK_EN_MASK
            },
        }
    }

    /// Set **LP_WDT_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_wdt_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::LP_WDT_DATE_MASK)
                | ((value << Date::LP_WDT_DATE_OFFSET) & Date::LP_WDT_DATE_MASK),
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
    /// need_des
    pub config0: Config0,
    /// 0x0004
    /// need_des
    pub config1: Config1,
    /// 0x0008
    /// need_des
    pub config2: Config2,
    /// 0x000c
    /// need_des
    pub config3: Config3,
    /// 0x0010
    /// need_des
    pub config4: Config4,
    /// 0x0014
    /// need_des
    pub feed: Feed,
    /// 0x0018
    /// need_des
    pub wprotect: Wprotect,
    /// 0x001c
    /// need_des
    pub swd_config: SwdConfig,
    /// 0x0020
    /// need_des
    pub swd_wprotect: SwdWprotect,
    /// 0x0024
    /// need_des
    pub int_raw: IntRaw,
    /// 0x0028
    /// need_des
    pub int_st: IntSt,
    /// 0x002c
    /// need_des
    pub int_ena: IntEna,
    /// 0x0030
    /// need_des
    pub int_clr: IntClr,
    _reserved0: [u8; 968],
    /// 0x03fc
    /// need_des
    pub date: Date,
}
