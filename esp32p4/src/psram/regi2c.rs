//! Minimal analog-I2C ("regi2c") master.
//!
//! The PLLs and the MSPI PHY are configured through a small analog register
//! file that is only reachable via the LP-domain I2C master in
//! `LP_I2C_ANA_MST`. Every access is a command word written to `I2C0_CTRL`:
//!
//! ```text
//! [24]    write enable
//! [23:16] data
//! [15:8]  register address
//! [7:0]   analog block (slave) id
//! ```
//!
//! The block must first be selected in `ANA_CONF2`; the master also needs its
//! clock enabled, which the ROM bootloader does not leave configured.

use esp32p4_pac::{LP_I2C_ANA_MST, LP_PERI};

/// Analog block id of the MSPI PLL (MPLL) register set.
pub(crate) const MSPI_BLOCK: u8 = 0x63;

const WRITE_EN: u32 = 1 << 24;
const DATA_SHIFT: u32 = 16;
const ADDR_SHIFT: u32 = 8;
const SLAVE_SHIFT: u32 = 0;

/// `ANA_CONF2` master-select bit for each analog block id.
const fn master_select(block: u8) -> Option<u32> {
    Some(match block {
        0x62 => 1 << 6,  // SDIO PLL
        0x63 => 1 << 9,  // MSPI PLL
        0x66 => 1 << 5,  // SYS PLL
        0x67 => 1 << 11, // CPU PLL
        0x69 => 1 << 7,  // SAR I2C
        0x6a => 1 << 12, // BIAS
        0x6d => 1 << 10, // DIG REG
        0x6f => 1 << 8,  // PLLA
        _ => return None,
    })
}

/// Enables the master clock and selects the analog block.
fn select_block(block: u8) {
    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let lp_peri = unsafe { LP_PERI::steal() };

    lp_peri.clk_en.set_ck_en_lp_i2cmst();
    // The master has no clock source out of reset, so transactions never
    // complete until the 160 MHz source is selected.
    mst.clk160m.set_clk_i2c_mst_sel_160m();

    mst.ana_conf2.write_bits(0);
    mst.ana_conf1.write_bits(0);

    if let Some(sel) = master_select(block) {
        mst.ana_conf2.modify_bits(|r| r | sel);
    }
}

#[inline]
fn wait_idle() {
    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    while mst.i2c0_ctrl.i2c0_busy() {
        core::hint::spin_loop();
    }
}

/// Reads an analog register.
pub(crate) fn read(block: u8, reg: u8) -> u8 {
    select_block(block);
    wait_idle();

    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let cmd = ((block as u32) << SLAVE_SHIFT) | ((reg as u32) << ADDR_SHIFT);
    mst.i2c0_ctrl.write_i2c0_ctrl(cmd);
    wait_idle();

    ((mst.i2c0_ctrl.i2c0_ctrl() >> DATA_SHIFT) & 0xff) as u8
}

/// Writes an analog register.
pub(crate) fn write(block: u8, reg: u8, value: u8) {
    select_block(block);
    wait_idle();

    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let cmd = ((block as u32) << SLAVE_SHIFT)
        | ((reg as u32) << ADDR_SHIFT)
        | ((value as u32) << DATA_SHIFT)
        | WRITE_EN;
    mst.i2c0_ctrl.write_i2c0_ctrl(cmd);
    wait_idle();
}

/// Reads `reg`, replaces `value_mask` with `value` and writes it back.
pub(crate) fn modify(block: u8, reg: u8, value_mask: u8, value: u8) {
    let old = read(block, reg);
    write(block, reg, (old & !value_mask) | (value & value_mask));
}
