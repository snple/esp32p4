//! MPLL (MSPI PLL) bring-up.
//!
//! The PSRAM interface is clocked from the MPLL. The ESP32-P4 has no way to
//! divide the 40 MHz crystal up to the PSRAM frequencies, so the PLL has to be
//! started from the analog side: the divider lives in the MPLL register file
//! and is only reachable through [`crate::regi2c`].
//!
//! `MPLL = XTAL * (div + 1) / (ref_div + 1)`, so 500 MHz takes `div = 24` with
//! `ref_div = 1` (`24 * 20 MHz`). The PSRAM bus then runs at half of that,
//! 250 MHz.

use crate::delay::Delay;
use crate::regi2c::mpll;
use esp32p4_pac::{LP_AON_CLKRST, PMU, SYSTIMER};

/// MPLL output frequency in MHz.
pub(crate) const FREQUENCY_MHZ: u32 = 500;

/// `DHREF` value used by ESP-IDF and esp-hal.
const DHREF_VALUE: u8 = 3;

/// `MPLL_DIV` layout: `div << 3 | ref_div`.
const fn div_value(div: u8, ref_div: u8) -> u8 {
    (div << 3) | ref_div
}

/// Starts the MPLL at 500 MHz and waits for its calibration to finish.
pub(crate) fn enable() {
    let delay = Delay::init(unsafe { SYSTIMER::steal() });

    unsafe { PMU::steal() }.rf_pwc.set_mspi_phy_xpd();
    unsafe { LP_AON_CLKRST::steal() }
        .lp_aonclkrst_hp_clk_ctrl
        .set_lp_aonclkrst_hp_mpll_500m_clk_en();

    let clkrst = unsafe { esp32p4_pac::HP_SYS_CLKRST::steal() };
    let pll_ctrl = &clkrst.ana_pll_ctrl0;

    // Calibration starts before the analog configuration is written and stops
    // once the PLL reports that it finished.
    pll_ctrl.clr_reg_mspi_cal_stop();

    mpll::DHREF.write(DHREF_VALUE);

    // Restart the calibration state machine.
    mpll::IR_CAL_RSTB_CAL.write(0);
    mpll::IR_CAL_RSTB_CAL.write(1);

    // 500 MHz / 20 MHz - 1 = 24.
    let div = (FREQUENCY_MHZ / 20) as u8 - 1;
    mpll::DIV.write(div_value(div, 1));

    while !pll_ctrl.reg_mspi_cal_end() {
        core::hint::spin_loop();
    }
    pll_ctrl.set_reg_mspi_cal_stop();

    delay.delay_us(10);
}
