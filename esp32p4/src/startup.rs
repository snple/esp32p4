//! Core-level start-up initialisation.
//!
//! `#[main]` runs [`init`] before the user's function body. The steps here fix
//! up state that the ESP-IDF second-stage bootloader leaves behind and enable a
//! CPU feature the ROM keeps off; none of them need peripheral ownership, but
//! they must happen before any driver runs.

use esp32p4_pac::{HP_SYS_CLKRST, LP_WDT, PMU, TIMG0, TIMG1};

/// Write-protect unlock key for the LP and TIMG watchdogs.
const WDT_UNLOCK_KEY: u32 = 0x50D8_3AA1;

/// PMU SW-stall value that keeps a core halted (ESP-IDF `cpu_utility_ll.h`).
const CORE_STALL_CODE: u32 = 0x86;

/// `mhcr` bits enabling the return stack, predictive jump and branch target
/// buffer.
const MHCR_BRANCH_PREDICTOR: u32 = (1 << 4) | (1 << 5) | (1 << 12);

/// Initialise the core state that must be correct before any driver exists.
///
/// Called by `#[main]`; calling it twice is harmless but unnecessary.
pub fn init() {
    disable_watchdogs();
    park_core1();
    enable_branch_predictor();
}

/// Stops the watchdogs the bootloader leaves running.
fn disable_watchdogs() {
    // SAFETY: `init` runs once from `#[main]`, before any driver takes these
    // peripherals; only configuration registers are written here.
    let rwdt = unsafe { LP_WDT::steal() };
    let clkrst = unsafe { HP_SYS_CLKRST::steal() };

    // The second-stage bootloader starts the RTC watchdog (9 s by default, see
    // CONFIG_BOOTLOADER_WDT_TIME_MS) and leaves it running. ESP-IDF applications
    // disable it during their own init; a bare-metal program has to do the same
    // or it is reset a few seconds after boot.
    rwdt.wprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
    rwdt.config0.clr_wdt_en();
    rwdt.wprotect.write_wdt_wkey(0);

    // MWDT0/MWDT1 are only running if an earlier program enabled them and its
    // CPU reset left the timer domain untouched. Their clock gate has to be open
    // for the write to land.
    clkrst.peri_clk_ctrl20.set_reg_timergrp0_wdt_clk_en();
    clkrst.peri_clk_ctrl21.set_reg_timergrp1_wdt_clk_en();

    // SAFETY: same as above; these are the TIMG watchdog register blocks.
    let timg0 = unsafe { TIMG0::steal() };
    let timg1 = unsafe { TIMG1::steal() };
    timg0.wdtwprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
    timg0.wdtconfig0.clr_wdt_en();
    timg0.wdtwprotect.write_wdt_wkey(0);
    timg1.wdtwprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
    timg1.wdtconfig0.clr_wdt_en();
    timg1.wdtwprotect.write_wdt_wkey(0);
}

/// Keeps core 1 halted.
///
/// The ROM only starts core 0. Core 1's clock, reset and stall state live in
/// the LP/always-on domain, so they survive a software reset; without this, core
/// 1 can keep executing stale code from the previous program.
fn park_core1() {
    // SAFETY: configuration-only access, as in `disable_watchdogs`.
    let pmu = unsafe { PMU::steal() };
    let clkrst = unsafe { HP_SYS_CLKRST::steal() };

    pmu.cpu_sw_stall
        .write_hpcore1_sw_stall_code(CORE_STALL_CODE);
    clkrst.soc_clk_ctrl0.clr_reg_core1_cpu_clk_en();
    clkrst.hp_rst_en0.set_reg_rst_en_core1_global();
}

/// Turns the branch predictor on.
///
/// The branch predictor issues its own cache requests, so the ROM leaves it off
/// until the cache is fully configured. It only affects performance.
fn enable_branch_predictor() {
    // SAFETY: `mhcr` (CSR 0x7c1) exists on this CPU; it only controls
    // prediction, not architectural state.
    #[allow(asm_sub_register)]
    unsafe {
        core::arch::asm!(
            "fence iorw, iorw",
            "csrs 0x7c1, {mhcr}",
            mhcr = in(reg) MHCR_BRANCH_PREDICTOR,
        );
    }
}
