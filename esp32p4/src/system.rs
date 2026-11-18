//! Reset control and reset-reason reporting.
//!
//! ESP32-P4 has no Cortex-M style `SYSRESETREQ`; resets are triggered from the
//! always-on (LP) block:
//!
//! | Function | Trigger | Reset type | Reset code |
//! |----------|---------|------------|------------|
//! | [`software_reset`] | `LP_SYS.SYS_CTRL.SYS_SW_RST` | HP core reset | `0x03` |
//! | [`software_reset_cpu`] | `LP_AON_CLKRST.HPCPU_RESET_CTRL0.HPCOREx_SW_RESET` | CPU reset | `0x0C` |
//!
//! # Why the trigger lives in the LP domain
//!
//! A core cannot reset itself: the reset request must be latched by logic that
//! keeps running while the target is held in reset, and the HP clock/bus must
//! stay alive until the reset is released. Every reset trigger on the ESP32-P4
//! therefore sits in the always-on LP domain:
//!
//! - `LP_SYS` (`0x5011_0000`, named `LP_SYSTEM` in the TRM) owns the HP *core*
//!   reset (`LP_SYSTEM_SYS_SW_RST`, TRM 22-SYSREG-LP, `SYS_CTRL` bit 1);
//! - `LP_AON_CLKRST` (`0x5011_1000`, `LP_CLKRST` in the TRM) owns the per-CPU
//!   reset and the reset-cause registers;
//! - `HP_SYS` (`0x500E_5000`) has no reset-trigger field at all.
//!
//! Writing `LP_SYSTEM_SYS_CTRL` must be a read-modify-write: the same register
//! holds live `DIG_FIB`/`ANA_FIB`/`IO_MUX_RESET_DISABLE` fields whose reset
//! values (`0xff`/`0x7f`) would otherwise be clobbered — the generated
//! `trigger_sys_sw_rst()` already does the read-modify-write.
//!
//! After a reset is released, the cause is read from
//! `LP_AON_CLKRST.RESET_CAUSE` — the same register the ROM's
//! `rtc_get_reset_reason()` reports, so no ROM symbol is needed.
//!
//! The reset codes match the ESP32-P4 TRM (Reset and Clock chapter) and the
//! `soc_reset_reason_t` / `RESET_REASON_*` values used by ESP-IDF.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::system::{software_reset, reset_reason, Cpu};
//!
//! if reset_reason(Cpu::ProCpu) == Some(esp32p4::system::ResetReason::ChipPowerOn) {
//!     // cold boot
//! }
//!
//! software_reset();
//! ```

use esp32p4_pac::{LP_AON_CLKRST, LP_SYS};
use esp32p4_rt::register::mhartid;

/// One of the two HP cores.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Cpu {
    /// HP core 0 — the core the ROM boots and the one `#[main]` runs on.
    ProCpu = 0,
    /// HP core 1 — held in reset by [`crate::startup::init`].
    AppCpu = 1,
}

impl Cpu {
    /// The core this code is running on.
    #[inline]
    pub fn current() -> Self {
        if mhartid::read() == 1 {
            Self::AppCpu
        } else {
            Self::ProCpu
        }
    }

    /// The other core.
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            Self::ProCpu => Self::AppCpu,
            Self::AppCpu => Self::ProCpu,
        }
    }
}

/// Cause of the most recent reset of a core.
///
/// Discriminants are the raw `LP_AON_CLKRST.RESET_CAUSE` codes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResetReason {
    /// `0x01` — power-on or brown-out chip reset that restarted the whole chip.
    ChipPowerOn = 0x01,
    /// `0x03` — HP core software reset, triggered by [`software_reset`].
    CoreSoftware = 0x03,
    /// `0x05` — PMU core reset (also reported when waking from deep sleep).
    CorePmuPowerDown = 0x05,
    /// `0x07` — MWDT core reset.
    CoreMwdg = 0x07,
    /// `0x09` — RWDT core reset.
    CoreRwdg = 0x09,
    /// `0x0b` — MWDT CPU reset.
    CpuMwdg = 0x0b,
    /// `0x0c` — HP CPU0/1 software reset, triggered by [`software_reset_cpu`].
    CpuSoftware = 0x0c,
    /// `0x0d` — RWDT CPU reset.
    CpuRwdg = 0x0d,
    /// `0x0f` — brown-out system reset.
    SysBrownOut = 0x0f,
    /// `0x10` — RWDT system reset.
    SysRwdg = 0x10,
    /// `0x12` — super watchdog reset.
    SysSuperWdg = 0x12,
    /// `0x13` — power glitch reset.
    SysPowerGlitch = 0x13,
    /// `0x14` — eFuse CRC error reset.
    CoreEfuseCrc = 0x14,
    /// `0x16` — reset requested over the USB Serial/JTAG JTAG interface.
    CoreUsbJtag = 0x16,
    /// `0x17` — reset requested over the USB Serial/JTAG UART interface.
    CoreUsbUart = 0x17,
    /// `0x18` — JTAG CPU reset.
    CpuJtag = 0x18,
    /// `0x1a` — CPU lockup reset.
    CpuLockup = 0x1a,
    /// `0x1b` — power-supply detect (PSDET) reset.
    SysPsdet = 0x1b,
}

impl ResetReason {
    /// Decode a raw `RESET_CAUSE` value.
    #[inline]
    pub const fn from_code(code: u32) -> Option<Self> {
        Some(match code {
            0x01 => Self::ChipPowerOn,
            0x03 => Self::CoreSoftware,
            0x05 => Self::CorePmuPowerDown,
            0x07 => Self::CoreMwdg,
            0x09 => Self::CoreRwdg,
            0x0b => Self::CpuMwdg,
            0x0c => Self::CpuSoftware,
            0x0d => Self::CpuRwdg,
            0x0f => Self::SysBrownOut,
            0x10 => Self::SysRwdg,
            0x12 => Self::SysSuperWdg,
            0x13 => Self::SysPowerGlitch,
            0x14 => Self::CoreEfuseCrc,
            0x16 => Self::CoreUsbJtag,
            0x17 => Self::CoreUsbUart,
            0x18 => Self::CpuJtag,
            0x1a => Self::CpuLockup,
            0x1b => Self::SysPsdet,
            _ => return None,
        })
    }
}

/// Raw `RESET_CAUSE` code of the most recent reset of `cpu`.
#[inline]
pub fn reset_reason_code(cpu: Cpu) -> u32 {
    // SAFETY: the LP_AON_CLKRST block is always-on and this is a read-only
    // register; no driver owns it.
    let clkrst = unsafe { LP_AON_CLKRST::steal() };
    let cause = &clkrst.lp_aonclkrst_reset_cause;
    match cpu {
        Cpu::ProCpu => cause.lp_aonclkrst_hpcore0_reset_cause(),
        Cpu::AppCpu => cause.lp_aonclkrst_hpcore1_reset_cause(),
    }
}

/// Reason of the most recent reset of `cpu`, or `None` for an unknown code.
#[inline]
pub fn reset_reason(cpu: Cpu) -> Option<ResetReason> {
    ResetReason::from_code(reset_reason_code(cpu))
}

/// Resets the HP digital system and restarts the chip.
///
/// This asserts `LP_SYSTEM_SYS_SW_RST` (`LP_SYS.SYS_CTRL` bit 1) — the hardware
/// "HP core software reset" (`0x03`): both HP cores, the HP peripherals, the HP
/// memories and the caches are reset, the LP/RTC domain keeps its state, and the
/// ROM bootloader runs again, reloading the application from flash.
///
/// Core 1 needs no special handling before the reset: [`crate::startup::init`]
/// stalls and clocks it down on every boot and never hands it a boot address, so
/// the ROM only brings up core 0 again (`crate::startup::init` then re-parks it).
///
/// Unlike a CPU reset ([`software_reset_cpu`]), all HP peripheral configuration
/// is lost, so drivers do not need to clean up before calling this.
#[inline]
pub fn software_reset() -> ! {
    // SAFETY: a system reset is not owned by any driver, and this never returns.
    unsafe { LP_SYS::steal() }.sys_ctrl.trigger_sys_sw_rst();

    // The reset is asynchronous; stay here until it takes effect.
    loop {
        esp32p4_rt::asm::wfi();
    }
}

/// Resets a single HP CPU, leaving the rest of the HP domain untouched.
///
/// This asserts `LP_CLKRST_HPCORE0_SW_RESET` / `LP_CLKRST_HPCORE1_SW_RESET`
/// (`LP_AON_CLKRST.HPCPU_RESET_CTRL0`, TRM 22-SYSREG-LP) — the hardware
/// "software CPU reset" (`0x0c`). Peripherals, HP memories and caches are
/// preserved, so the caller is responsible for any cleanup.
///
/// The reset core restarts from the ROM. Core 1 only leaves the ROM once core 0
/// has written its entry point, so resetting [`Cpu::AppCpu`] on its own leaves
/// it waiting inside the ROM.
///
/// # Warning
///
/// When `cpu` is the current core, this function does not return normally:
/// execution stops when the reset takes effect.
#[inline]
pub fn software_reset_cpu(cpu: Cpu) {
    // SAFETY: LP-domain reset control is not owned by any driver.
    let clkrst = unsafe { LP_AON_CLKRST::steal() };
    let ctrl = &clkrst.lp_aonclkrst_hpcpu_reset_ctrl0;
    match cpu {
        Cpu::ProCpu => ctrl.trigger_lp_aonclkrst_hpcore0_sw_reset(),
        Cpu::AppCpu => ctrl.trigger_lp_aonclkrst_hpcore1_sw_reset(),
    }
}
