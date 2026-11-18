//! PSRAM controller bring-up (PSRAM MSPI0 / MSPI1).
//!
//! The PSRAM die sits on the *second* MSPI controller (`SPI0` in the SVD,
//! called `MEMSPI2`/PSRAM_MSPI0 in other code bases), which is the one that
//! owns the AXI cache interface and the MMU. `SPI1` (`MEMSPI3`/PSRAM_MSPI1) is
//! the small controller used for direct mode-register transactions.
//!
//! Initialization order matters and follows ESP-IDF:
//!
//! 1. power the PHY LDOs and let them settle,
//! 2. enable the PSRAM clocks and pulse the MSPI reset,
//! 3. program the bus clock divider,
//! 4. enable the DLL, configure the pads and the CS timing,
//! 5. program the PSRAM mode registers through `SPI1`,
//! 6. detect the density, configure the AXI controller,
//! 7. map the pages into the `0x4800_0000` window.
//!
//! Mode-register access is the only part that needs ROM helpers: the command
//! sequencer setup (`esp_rom_spi_cmd_config`) is not documented anywhere but
//! in the ESP-IDF HAL, so the two ROM entry points are provided by `link.x`.

use core::ptr::null_mut;

use esp32p4_pac::{HP_SYS, HP_SYS_CLKRST, PMU, SPI0, SPI1};

use super::{Error, pads};

/// PSRAM page size used by the MMU.
pub(crate) const PAGE_SIZE: usize = 64 * 1024;
/// Number of MMU entries, i.e. the size of the external memory window.
pub(crate) const MMU_ENTRIES: usize = 1024;
/// Base address of the mapped PSRAM window.
pub(crate) const BASE: usize = 0x4800_0000;
/// Uncached alias of the same window, usable for DMA and integrity checks.
pub(crate) const UNCACHED_BASE: usize = 0x8800_0000;

/// Mode-register pair addresses.
const MR0_MR1: u32 = 0x0;
const MR2_MR3: u32 = 0x2;
const MR4_MR5: u32 = 0x4;
const MR8_MR9: u32 = 0x8;

/// Read mode-register command (OPI-DTR, 16-bit command).
const CMD_MR_READ: u16 = 0x4040;
/// Write mode-register command (OPI-DTR, 16-bit command).
const CMD_MR_WRITE: u16 = 0xc0c0;

/// ROM's controller index for the PSRAM MSPI.
const ROM_PSRAM_MSPI: i32 = 3;
/// `ESP_ROM_SPIFLASH_OPI_DTR_MODE`: 8 data lines, double data rate.
const ROM_MODE_OPI_DTR: u32 = 7;

/// `Cache_Invalidate_Addr` bus mask for the L1 data cache plus the L2 cache.
const CACHE_BUS_L1D_AND_L2: u32 = 0x30;

/// Timing parameters for one PSRAM frequency.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Timing {
    /// Bus clock in MHz; must divide the MPLL output evenly.
    pub bus_clock_mhz: u32,
    /// `MR0.read_latency` field value (cycles = 2 * value + 6).
    pub mr0_read_latency: u8,
    /// `MR4.wr_latency` field value.
    pub mr4_write_latency: u8,
    /// Dummy bits of a synchronous read; the register takes `n - 1`.
    pub read_dummy_bits: u8,
    /// Dummy bits of a synchronous write; the register takes `n - 1`.
    pub write_dummy_bits: u8,
    /// Dummy bits of a direct mode-register read.
    pub reg_dummy_bits: u32,
}

/// 250 MHz, the fastest frequency the default MPLL configuration supports.
pub(crate) const MHZ_250: Timing = Timing {
    bus_clock_mhz: 250,
    mr0_read_latency: 6,
    mr4_write_latency: 3,
    read_dummy_bits: 33,
    write_dummy_bits: 15,
    reg_dummy_bits: 16,
};

/// MPLL output frequency that feeds the PSRAM function clock.
pub(crate) const SOURCE_MHZ: u32 = 500;

/// `esp_rom_spi_cmd_t` as defined by `esp32p4/rom/opi_flash.h`.
#[repr(C)]
struct RomSpiCmd {
    cmd: u16,
    cmd_bit_len: u16,
    addr: *mut u32,
    addr_bit_len: u32,
    tx_data: *mut u32,
    tx_data_bit_len: u32,
    rx_data: *mut u32,
    rx_data_bit_len: u32,
    dummy_bit_len: u32,
}

unsafe extern "C" {
    fn esp_rom_spi_set_op_mode(spi_num: i32, mode: u32);
    fn esp_rom_spi_cmd_config(spi_num: i32, pcmd: *mut RomSpiCmd);
    fn Cache_Invalidate_Addr(bus: u32, addr: u32, size: u32);
}

/// Powers the MSPI PHY LDOs.
pub(crate) fn phy_ldo_init() {
    let pmu = unsafe { PMU::steal() };
    pmu.ext_ldo_p00p1a.write_bits(0x4020_0100);
    pmu.ext_ldo_p00p1a_ana.write_bits(0xb100_0000);
    pmu.ext_ldo_p00p2a.write_bits(0x4020_0000);
    pmu.ext_ldo_p00p2a_ana.write_bits(0xa000_0000);
    pmu.ext_ldo_p00p3a.write_bits(0x4020_0000);
    pmu.ext_ldo_p00p3a_ana.write_bits(0xa000_0000);
    pmu.ext_ldo_p10p1a.write_bits(0x4020_0180);
    pmu.ext_ldo_p10p1a_ana.write_bits(0x5700_0000);
    pmu.ext_ldo_p10p2a.write_bits(0x4020_0000);
    pmu.ext_ldo_p10p2a_ana.write_bits(0xa000_0000);
    pmu.ext_ldo_p10p3a.write_bits(0x4020_0000);
    pmu.ext_ldo_p10p3a_ana.write_bits(0xa000_0000);

    // Let the outputs settle before the PHY is exercised.
    crate::delay::Delay::init(unsafe { esp32p4_pac::SYSTIMER::steal() }).delay_us(50);
}

/// Enables the PSRAM bus/function clocks and selects the MPLL as source.
pub(crate) fn enable_clocks() {
    let clkrst = unsafe { HP_SYS_CLKRST::steal() };
    clkrst.soc_clk_ctrl0.set_reg_psram_sys_clk_en();

    let peri = &clkrst.peri_clk_ctrl00;
    peri.set_reg_psram_core_clk_en();
    peri.set_reg_psram_pll_clk_en();
    // 1 = MPLL.
    peri.write_reg_psram_clk_src_sel(1);
}

/// Pulses the reset of both PSRAM MSPI controllers.
pub(crate) fn reset_module() {
    let rst = unsafe { HP_SYS_CLKRST::steal() };
    let reg = &rst.hp_rst_en0;
    reg.set_reg_rst_en_dual_mspi_axi();
    reg.set_reg_rst_en_dual_mspi_apb();
    reg.clr_reg_rst_en_dual_mspi_axi();
    reg.clr_reg_rst_en_dual_mspi_apb();
}

/// `SRAM_CLK`/`CLOCK` divider value for `div`.
const fn bus_clock_bits(div: u32) -> u32 {
    const EQU_SYSCLK: u32 = 1 << 31;
    if div <= 1 {
        EQU_SYSCLK
    } else {
        let n = div - 1;
        let h = div / 2 - 1;
        (n << 16) | (h << 8) | n
    }
}

/// Sets the bus clock divider on both PSRAM controllers.
fn set_bus_clock(timing: &Timing) -> Result<(), Error> {
    if !SOURCE_MHZ.is_multiple_of(timing.bus_clock_mhz) {
        return Err(Error::UnsupportedClock);
    }
    let bits = bus_clock_bits(SOURCE_MHZ / timing.bus_clock_mhz);

    unsafe { SPI0::steal() }.spi_mem_sram_clk.write_bits(bits);
    unsafe { SPI1::steal() }.spi_mem_clock.write_bits(bits);
    Ok(())
}

/// Enables DLL timing calibration on both controllers.
fn enable_dll() {
    let spi0 = unsafe { SPI0::steal() };
    spi0.spi_mem_timing_cali.set_spi_mem_dll_timing_cali();
    spi0.spi_smem_timing_cali.set_spi_smem_dll_timing_cali();
}

/// Programs the CS setup/hold timing of the AXI controller.
fn set_cs_timing() {
    const CS_SETUP_TIME: u32 = 4;
    const CS_HOLD_TIME: u32 = 4;
    const CS_HOLD_DELAY: u32 = 3;

    let ac = &unsafe { SPI0::steal() }.spi_smem_ac;
    ac.set_spi_smem_cs_setup();
    ac.set_spi_smem_cs_hold();
    ac.set_spi_smem_split_trans_en();
    ac.write_spi_smem_cs_setup_time(CS_SETUP_TIME - 1);
    ac.write_spi_smem_cs_hold_time(CS_HOLD_TIME - 1);
    ac.write_spi_smem_cs_hold_delay(CS_HOLD_DELAY - 1);
}

/// Configures the AXI controller for 16-line DDR access.
fn configure_controller(timing: &Timing, is_hex: bool) {
    let spi = unsafe { SPI0::steal() };

    let sctrl = &spi.spi_mem_cache_sctrl;
    sctrl.set_spi_mem_cache_usr_saddr_4byte();
    sctrl.set_spi_mem_usr_wr_sram_dummy();
    sctrl.set_spi_mem_usr_rd_sram_dummy();
    sctrl.set_spi_mem_cache_sram_usr_rcmd();
    sctrl.write_spi_mem_sram_rdummy_cyclelen(timing.read_dummy_bits as u32);
    sctrl.write_spi_mem_sram_wdummy_cyclelen(timing.write_dummy_bits as u32);
    sctrl.write_spi_mem_sram_addr_bitlen(31);
    sctrl.set_spi_mem_cache_sram_usr_wcmd();
    sctrl.set_spi_mem_sram_oct();

    let cmd = &spi.spi_mem_sram_cmd;
    cmd.set_spi_mem_sdin_oct();
    cmd.set_spi_mem_sdout_oct();
    cmd.set_spi_mem_saddr_oct();
    cmd.set_spi_mem_scmd_oct();
    cmd.set_spi_mem_sdummy_wout();
    if is_hex {
        cmd.set_spi_mem_sdin_hex();
        cmd.set_spi_mem_sdout_hex();
    }

    let drd = &spi.spi_mem_sram_drd_cmd;
    drd.write_spi_mem_cache_sram_usr_rd_cmd_bitlen(15);
    drd.write_spi_mem_cache_sram_usr_rd_cmd_value(0);
    let dwr = &spi.spi_mem_sram_dwr_cmd;
    dwr.write_spi_mem_cache_sram_usr_wr_cmd_bitlen(15);
    dwr.write_spi_mem_cache_sram_usr_wr_cmd_value(0x8080);

    spi.spi_mem_ctrl1.set_spi_mem_ar_splice_en();
    spi.spi_mem_ctrl1.set_spi_mem_aw_splice_en();

    let ddr = &spi.spi_smem_ddr;
    ddr.set_spi_smem_var_dummy();
    ddr.clr_wdat_swp();
    ddr.clr_rdat_swp();
    ddr.set_en();

    spi.spi_mem_cache_fctrl.set_spi_mem_axi_req_en();
    spi.spi_mem_cache_fctrl.clr_spi_close_axi_inf_en();
}

/// Waits until the direct-mode transaction started on `SPI1` completes.
fn kick_and_wait() -> Result<(), Error> {
    const MAX_ITERATIONS: u32 = 1_000_000;

    let spi = unsafe { SPI1::steal() };
    // Select PSRAM (CS1) and deselect flash (CS0).
    spi.spi_mem_misc.clr_spi_mem_cs1_dis();
    spi.spi_mem_misc.set_spi_mem_cs0_dis();

    spi.spi_mem_cmd.set_spi_mem_usr();

    let mut timeout = MAX_ITERATIONS;
    while spi.spi_mem_cmd.spi_mem_usr() {
        if timeout == 0 {
            return Err(Error::Timeout);
        }
        timeout -= 1;
        core::hint::spin_loop();
    }

    Ok(())
}

/// Reads a mode-register pair from the PSRAM die.
fn mr_read(timing: &Timing, mr_addr: u32) -> Result<(u8, u8), Error> {
    let mut addr = mr_addr;
    let mut rx = [0u8; 2];
    let mut conf = RomSpiCmd {
        cmd: CMD_MR_READ,
        cmd_bit_len: 16,
        addr: &raw mut addr,
        addr_bit_len: 32,
        tx_data: null_mut(),
        tx_data_bit_len: 0,
        rx_data: rx.as_mut_ptr().cast(),
        rx_data_bit_len: 16,
        dummy_bit_len: timing.reg_dummy_bits,
    };

    // SAFETY: the ROM helpers only touch the MSPI controller and the command
    // descriptor, which lives for the duration of the call.
    unsafe {
        esp_rom_spi_set_op_mode(ROM_PSRAM_MSPI, ROM_MODE_OPI_DTR);
        esp_rom_spi_cmd_config(ROM_PSRAM_MSPI, &raw mut conf);
    }
    kick_and_wait()?;

    // The received bytes land in W0, low half first.
    let word = unsafe { SPI1::steal() }.spi_mem_w0.read_bits();
    rx[0] = word as u8;
    rx[1] = (word >> 8) as u8;

    Ok((rx[0], rx[1]))
}

/// Writes a mode-register pair of the PSRAM die.
fn mr_write(mr_addr: u32, low: u8, high: u8) -> Result<(), Error> {
    let mut addr = mr_addr;
    let mut tx = ((low as u16) | ((high as u16) << 8)).to_le_bytes();
    let mut conf = RomSpiCmd {
        cmd: CMD_MR_WRITE,
        cmd_bit_len: 16,
        addr: &raw mut addr,
        addr_bit_len: 32,
        tx_data: tx.as_mut_ptr().cast(),
        tx_data_bit_len: 16,
        rx_data: null_mut(),
        rx_data_bit_len: 0,
        dummy_bit_len: 0,
    };

    // SAFETY: see `mr_read`.
    unsafe {
        esp_rom_spi_set_op_mode(ROM_PSRAM_MSPI, ROM_MODE_OPI_DTR);
        esp_rom_spi_cmd_config(ROM_PSRAM_MSPI, &raw mut conf);
    }
    kick_and_wait()
}

/// Programs MR0 (latency), MR4 (write latency) and MR8 (burst type).
fn init_mode_registers(timing: &Timing, is_hex: bool) -> Result<(), Error> {
    let (mut mr0, mr1) = mr_read(timing, MR0_MR1)?;
    mr0 &= !(0x3 | (0x7 << 2) | (1 << 5));
    mr0 |= (timing.mr0_read_latency & 0x7) << 2;
    // Latency type stays at the default (1) for the AP die.
    mr0 |= 1 << 5;
    mr_write(MR0_MR1, mr0, mr1)?;

    let (mut mr4, mr5) = mr_read(timing, MR4_MR5)?;
    mr4 &= !(0x7 << 5);
    mr4 |= (timing.mr4_write_latency & 0x7) << 5;
    mr_write(MR4_MR5, mr4, mr5)?;

    let (mut mr8, mr9) = mr_read(timing, MR8_MR9)?;
    mr8 &= !(0x3 | (1 << 2) | (1 << 3) | (1 << 6));
    // Burst length 8, robust burst on, 16-line mode.
    mr8 |= 3 | (1 << 3);
    if is_hex {
        mr8 |= 1 << 6;
    }
    mr_write(MR8_MR9, mr8, mr9)
}

/// Detects the density from the `MR2` density bits.
fn detect_size(timing: &Timing) -> Option<usize> {
    let (mr2, _) = mr_read(timing, MR2_MR3).ok()?;
    Some(match mr2 & 0x7 {
        0x1 => 4 << 20,
        0x3 => 8 << 20,
        0x5 => 16 << 20,
        0x6 => 64 << 20,
        0x7 => 32 << 20,
        _ => return None,
    })
}

/// Maps `size` bytes of PSRAM starting at [`BASE`].
pub(crate) fn map(size: usize) {
    let pages = (size / PAGE_SIZE).min(MMU_ENTRIES);
    let spi = unsafe { SPI0::steal() };

    for page in 0..pages {
        spi.spi_mem_mmu_item_index
            .write_spi_mmu_item_index(page as u32);
        // paddr[9:0] | access_spiram (bit 10) | valid (bit 11)
        spi.spi_mem_mmu_item_content
            .write_bits((page as u32 & 0x3ff) | (1 << 10) | (1 << 11));
    }

    // Drop any line the cache may have prefetched from the window.
    unsafe { Cache_Invalidate_Addr(CACHE_BUS_L1D_AND_L2, BASE as u32, size as u32) };
}

/// Silicon revision 3.0 needs two dummy reads and a controller reset before
/// the final mapping takes effect (ESP-IDF `esp_psram_p4_rev3_workaround`).
pub(crate) fn rev3_workaround() {
    let spi = unsafe { SPI0::steal() };

    let cache_fctrl = spi.spi_mem_cache_fctrl.read_bits();
    let cache_sctrl = spi.spi_mem_cache_sctrl.read_bits();
    let sram_cmd = spi.spi_mem_sram_cmd.read_bits();
    let sram_drd_cmd = spi.spi_mem_sram_drd_cmd.read_bits();
    let sram_dwr_cmd = spi.spi_mem_sram_dwr_cmd.read_bits();
    let sram_clk = spi.spi_mem_sram_clk.read_bits();
    let ctrl1 = spi.spi_mem_ctrl1.read_bits();
    let smem_ddr = spi.spi_smem_ddr.read_bits();
    let timing_cali = spi.spi_mem_timing_cali.read_bits();
    let smem_timing_cali = spi.spi_smem_timing_cali.read_bits();
    let smem_ac = spi.spi_smem_ac.read_bits();

    // Suppress the bus-error response so the dummy reads below cannot trap.
    unsafe { HP_SYS::steal() }.core_err_resp_dis.write_bits(0x7);

    // Map physical page 0 to MMU entry 0, then touch both cached and uncached
    // aliases.
    spi.spi_mem_mmu_item_index.write_spi_mmu_item_index(0);
    spi.spi_mem_mmu_item_content
        .write_bits((1 << 10) | (1 << 11));
    unsafe {
        let _ = core::ptr::read_volatile(0x8800_0000u32 as *const u32);
        let _ = core::ptr::read_volatile(0x8800_0080u32 as *const u32);
    }
    crate::delay::Delay::init(unsafe { esp32p4_pac::SYSTIMER::steal() }).delay_us(1);

    reset_module();

    unsafe { HP_SYS::steal() }.core_err_resp_dis.write_bits(0);

    spi.spi_mem_cache_fctrl.write_bits(cache_fctrl);
    spi.spi_mem_cache_sctrl.write_bits(cache_sctrl);
    spi.spi_mem_sram_cmd.write_bits(sram_cmd);
    spi.spi_mem_sram_drd_cmd.write_bits(sram_drd_cmd);
    spi.spi_mem_sram_dwr_cmd.write_bits(sram_dwr_cmd);
    spi.spi_mem_sram_clk.write_bits(sram_clk);
    spi.spi_mem_ctrl1.write_bits(ctrl1);
    spi.spi_smem_ddr.write_bits(smem_ddr);
    spi.spi_mem_timing_cali.write_bits(timing_cali);
    spi.spi_smem_timing_cali.write_bits(smem_timing_cali);
    spi.spi_smem_ac.write_bits(smem_ac);
}

/// Brings up the controller and the PSRAM die, returning the detected size.
///
/// The pages are not mapped yet; call [`map`] once the size to expose is known.
pub(crate) fn init(timing: &Timing, is_hex: bool, rev3: bool) -> Result<usize, Error> {
    phy_ldo_init();
    enable_clocks();
    reset_module();
    set_bus_clock(timing)?;

    enable_dll();
    pads::configure(is_hex);
    set_cs_timing();

    init_mode_registers(timing, is_hex)?;
    let size = detect_size(timing).ok_or(Error::NotDetected)?;

    configure_controller(timing, is_hex);

    if rev3 {
        rev3_workaround();
    }

    Ok(size)
}
