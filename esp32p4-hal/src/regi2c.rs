//! Analog I2C master ("regi2c").
//!
//! A part of the chip's configuration lives in an analog register file that is
//! not memory mapped: the PLL dividers, the MSPI PHY bias, the ADC calibration
//! values and the temperature-sensor trim. It is only reachable through the
//! LP-domain I2C master in `LP_I2C_ANA_MST`, which speaks a tiny command
//! protocol:
//!
//! ```text
//! bit  25    BUSY   (read only)
//! bit  24    WRITE  1 = write, 0 = read
//! bits 23:16 DATA
//! bits 15:8  register address inside the analog block
//! bits 7:0   analog block id (the I2C slave address)
//! ```
//!
//! Writing a command starts the transfer; the master clears `BUSY` when the
//! analog block has acknowledged it, and a read leaves the result in `DATA`.
//!
//! # Using it
//!
//! Registers are described by [`Register`] and [`Field`] values, so call sites
//! never spell out block ids, addresses and bit positions. The ones the HAL
//! already knows are grouped per block, for example [`mpll`]:
//!
//! ```ignore
//! use esp32p4_hal::regi2c::mpll;
//!
//! let divider = mpll::DIV.read();
//! mpll::DHREF.write(3);                              // bits [5:4] only
//! mpll::IR_CAL_RSTB.update(|v| v & !(1 << 5));       // read-modify-write
//! ```
//!
//! A register without a constant yet is just as easy to describe, and the
//! description can live in the driver instead of here:
//!
//! ```ignore
//! use esp32p4_hal::regi2c::{Block, Field, Register};
//!
//! const OSC: Register = Register::new(Block::SAR_ADC, 2);
//! const SAMPLE_CYCLE: Field = Field::new(OSC, 2, 0);
//!
//! let cycles = SAMPLE_CYCLE.read();
//! ```
//!
//! [`read_raw`] and [`write_raw`] remain for the rare case where only a block
//! id and an address are known at runtime, e.g. when dumping the register file.
//!
//! # Concurrency
//!
//! The master has a single bus and no arbitration: transactions must not
//! interleave. Every operation therefore runs inside a [`CriticalSection`]
//! (interrupts masked for its duration). That is enough on one core; when a
//! second core uses the bus as well, the caller has to provide real mutual
//! exclusion around the sequence.

use esp32p4_pac::{HP_SYS_CLKRST, LP_I2C_ANA_MST, LP_PERI, PMU, SYSTIMER};
use esp32p4_rt::register::mstatus;

/// An analog block, i.e. one I2C slave on the analog bus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Block(u8);

impl Block {
    /// SDIO PLL.
    pub const SDIO_PLL: Self = Self(0x62);
    /// MSPI PLL (MPLL), which also feeds the PSRAM PHY.
    pub const MSPI: Self = Self(0x63);
    /// System PLL (SPLL).
    pub const SYS_PLL: Self = Self(0x66);
    /// CPU PLL (CPLL).
    pub const CPU_PLL: Self = Self(0x67);
    /// SAR ADC and temperature sensor.
    pub const SAR_ADC: Self = Self(0x69);
    /// Bias generator.
    pub const BIAS: Self = Self(0x6a);
    /// Digital regulator block.
    pub const DIG_REG: Self = Self(0x6d);
    /// Audio PLL (PLLA).
    pub const PLLA: Self = Self(0x6f);

    /// Creates a block handle from its I2C slave id.
    ///
    /// Ids that are not one of the constants above are accepted, but every
    /// transaction with them fails with [`Error::UnknownBlock`].
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// The I2C slave id of this block.
    pub const fn id(self) -> u8 {
        self.0
    }

    /// `ANA_CONF2` master-select bit, or `None` for an unknown block.
    const fn select_mask(self) -> Option<u32> {
        Some(match self.0 {
            0x62 => 1 << 6,
            0x63 => 1 << 9,
            0x66 => 1 << 5,
            0x67 => 1 << 11,
            0x69 => 1 << 7,
            0x6a => 1 << 12,
            0x6d => 1 << 10,
            0x6f => 1 << 8,
            _ => return None,
        })
    }
}

/// A byte register inside an analog block.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Register {
    block: Block,
    addr: u8,
}

impl Register {
    /// Describes a register without touching the bus.
    pub const fn new(block: Block, addr: u8) -> Self {
        Self { block, addr }
    }

    /// The block this register belongs to.
    pub const fn block(self) -> Block {
        self.block
    }

    /// The address of this register inside its block.
    pub const fn addr(self) -> u8 {
        self.addr
    }

    /// Describes bits `msb..=lsb` of this register.
    pub const fn field(self, msb: u8, lsb: u8) -> Field {
        Field::new(self, msb, lsb)
    }

    /// Reads the whole byte.
    pub fn read(self) -> u8 {
        self.try_read()
            .unwrap_or_else(|err| panic_regi2c(err, self))
    }

    /// Writes the whole byte.
    pub fn write(self, value: u8) {
        if let Err(err) = self.try_write(value) {
            panic_regi2c(err, self);
        }
    }

    /// Read-modify-write without exposing the intermediate value.
    pub fn update(self, f: impl FnOnce(u8) -> u8) {
        let _guard = CriticalSection::enter();
        match self.try_read() {
            Ok(old) => {
                if let Err(err) = self.try_write(f(old)) {
                    panic_regi2c(err, self);
                }
            }
            Err(err) => panic_regi2c(err, self),
        }
    }

    /// [`read`](Self::read), reporting a stuck bus instead of panicking.
    pub fn try_read(self) -> Result<u8, Error> {
        let _guard = CriticalSection::enter();
        transfer_read(self.block, self.addr)
    }

    /// [`write`](Self::write), reporting a stuck bus instead of panicking.
    pub fn try_write(self, value: u8) -> Result<(), Error> {
        let _guard = CriticalSection::enter();
        transfer_write(self.block, self.addr, value)
    }
}

/// A bit field inside a register, `lsb..=msb`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Field {
    register: Register,
    msb: u8,
    lsb: u8,
}

impl Field {
    /// Describes bits `msb..=lsb` of `register`.
    ///
    /// # Panics
    ///
    /// Panics unless `lsb <= msb < 8`; the check runs at compile time when the
    /// field is declared `const`.
    pub const fn new(register: Register, msb: u8, lsb: u8) -> Self {
        assert!(lsb <= msb && msb < 8, "regi2c: invalid field range");
        Self { register, msb, lsb }
    }

    /// The register this field lives in.
    pub const fn register(self) -> Register {
        self.register
    }

    /// Mask of the field, already shifted into place.
    pub const fn mask(self) -> u8 {
        let width = self.msb - self.lsb + 1;
        (((1u16 << width) - 1) as u8) << self.lsb
    }

    /// Reads the field, right aligned.
    pub fn read(self) -> u8 {
        self.try_read()
            .unwrap_or_else(|err| panic_regi2c(err, self.register))
    }

    /// Writes the field, leaving the other bits of the register untouched.
    ///
    /// Bits of `value` above the field width are discarded.
    pub fn write(self, value: u8) {
        if let Err(err) = self.try_write(value) {
            panic_regi2c(err, self.register);
        }
    }

    /// [`read`](Self::read), reporting a stuck bus instead of panicking.
    pub fn try_read(self) -> Result<u8, Error> {
        let _guard = CriticalSection::enter();
        let byte = transfer_read(self.register.block, self.register.addr)?;
        Ok((byte & self.mask()) >> self.lsb)
    }

    /// [`write`](Self::write), reporting a stuck bus instead of panicking.
    pub fn try_write(self, value: u8) -> Result<(), Error> {
        // One section: a concurrent transaction must not observe the read
        // without the matching write.
        let _guard = CriticalSection::enter();
        let mask = self.mask();
        let old = transfer_read(self.register.block, self.register.addr)?;
        let new = (old & !mask) | (((value as u16) << self.lsb) as u8 & mask);
        transfer_write(self.register.block, self.register.addr, new)
    }
}

/// Analog registers of the MSPI PLL (MPLL).
///
/// The MPLL divider lives in the analog file even though its calibration status
/// is memory mapped, which is why any clock driver needs this module.
pub mod mpll {
    use super::{Block, Field, Register};

    /// Calibration state machine control (`IR_CAL_RSTB`).
    pub const IR_CAL_RSTB: Register = Register::new(Block::MSPI, 1);
    /// Bit 5 of [`IR_CAL_RSTB`]: toggling it restarts the calibration.
    pub const IR_CAL_RSTB_CAL: Field = Field::new(IR_CAL_RSTB, 5, 5);
    /// Divider register, laid out as `div << 3 | ref_div`.
    pub const DIV: Register = Register::new(Block::MSPI, 2);
    /// Feedback divider of [`DIV`], bits `[7:3]`.
    pub const DIV_RATIO: Field = Field::new(DIV, 7, 3);
    /// Reference divider of [`DIV`], bits `[2:0]`.
    pub const REF_DIV_RATIO: Field = Field::new(DIV, 2, 0);
    /// Duty-cycle reference (`DHREF`), bits `[5:4]` of register 3.
    pub const DHREF: Field = Field::new(Register::new(Block::MSPI, 3), 5, 4);
}

/// Audio PLL (`Apll`), on analog block `0x6f`.
///
/// The coefficients and the calibration live here; [`crate::apll`] drives them.
pub mod apll {
    use super::{Block, Field, Register};

    /// Calibration delay, and the bits that start and release a calibration.
    pub const CAL: Register = Register::new(Block::PLLA, 0);
    /// Calibration delay, `[3:0]` of [`CAL`].
    pub const CAL_DELAY: Field = Field::new(CAL, 3, 0);
    /// `[4]` of [`CAL`]: releases the calibration state machine.
    pub const CAL_RSTB: Field = Field::new(CAL, 4, 4);
    /// `[5]` of [`CAL`]: starts a calibration.
    pub const CAL_START: Field = Field::new(CAL, 5, 5);
    /// `[6]` of [`CAL`]: unstops the calibration clock.
    pub const CAL_UNSTOP: Field = Field::new(CAL, 6, 6);
    /// `[7]` of [`CAL`]: enables the frequency calibration comparator.
    pub const ENABLE_FCAL: Field = Field::new(CAL, 7, 7);

    /// Calibration result, `[7]` of register 3.
    pub const CAL_END: Field = Field::new(Register::new(Block::PLLA, 3), 7, 7);

    /// Output divider `[4:0]` of register 4.
    pub const OUTPUT_DIV: Field = Field::new(Register::new(Block::PLLA, 4), 4, 0);

    /// Sigma-delta stop control, register 5.
    pub const SDM: Register = Register::new(Block::PLLA, 5);
    /// `[5]` of [`SDM`]: stops the sigma-delta modulator.
    pub const SDM_STOP: Field = Field::new(SDM, 5, 5);
    /// `[6]` of [`SDM`]: releases the sigma-delta modulator reset.
    pub const SDM_RSTB: Field = Field::new(SDM, 6, 6);

    /// Fractional part, most significant byte, `[5:0]` of register 7.
    pub const DSDM2: Field = Field::new(Register::new(Block::PLLA, 7), 5, 0);
    /// Fractional part, middle byte: register 8.
    pub const DSDM1: Field = Field::new(Register::new(Block::PLLA, 8), 7, 0);
    /// Fractional part, least significant byte: register 9.
    pub const DSDM0: Field = Field::new(Register::new(Block::PLLA, 9), 7, 0);
}

/// Errors reported by the `try_*` functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The block id does not correspond to any analog slave.
    UnknownBlock(u8),
    /// The master did not complete the transfer in time.
    Timeout,
}

/// Interrupt-free region guarding one transaction.
///
/// Only the `MIE` bit is touched, so the FPU state and the rest of `mstatus`
/// are left alone. Nested sections are supported: the innermost restore keeps
/// interrupts disabled and the outermost one restores the original state.
pub struct CriticalSection {
    was_enabled: bool,
}

impl CriticalSection {
    /// Masks machine interrupts until the value is dropped.
    pub fn enter() -> Self {
        let was_enabled = mstatus::mie();
        mstatus::clear_mie();
        Self { was_enabled }
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        if self.was_enabled {
            // SAFETY: re-enabling a previously enabled interrupt source can
            // only deliver interrupts that were going to be delivered anyway.
            unsafe { mstatus::set_mie() };
        }
    }
}

/// Runs `f` with machine interrupts masked.
pub fn free<R>(f: impl FnOnce() -> R) -> R {
    let _guard = CriticalSection::enter();
    f()
}

/// Reads a register described only by a block and an address.
///
/// Prefer [`Register`]/[`Field`] constants; this is the escape hatch for
/// register dumps and values only known at runtime.
pub fn read_raw(block: Block, addr: u8) -> u8 {
    try_read_raw(block, addr).unwrap_or_else(|err| panic_raw(err, block, addr))
}

/// Writes a register described only by a block and an address.
pub fn write_raw(block: Block, addr: u8, value: u8) {
    if let Err(err) = try_write_raw(block, addr, value) {
        panic_raw(err, block, addr);
    }
}

/// [`read_raw`], reporting a stuck bus instead of panicking.
pub fn try_read_raw(block: Block, addr: u8) -> Result<u8, Error> {
    let _guard = CriticalSection::enter();
    transfer_read(block, addr)
}

/// [`write_raw`], reporting a stuck bus instead of panicking.
pub fn try_write_raw(block: Block, addr: u8, value: u8) -> Result<(), Error> {
    let _guard = CriticalSection::enter();
    transfer_write(block, addr, value)
}

/// Brings the master up: reference clock, bus clock gate, reset pulse and the
/// clock-source selection.
///
/// Safe to call at any time; the bootloader may already have done part of it.
pub fn init() {
    enable_clock();
    reset();
    // The reset clears the clock-source selection, so configure it last.
    configure_clock();
}

/// Enables the 160 MHz reference and the master's bus clock gate.
pub fn enable_clock() {
    // The master always runs from the 160 MHz reference.
    unsafe { HP_SYS_CLKRST::steal() }
        .ref_clk_ctrl2
        .set_reg_ref_160m_clk_en();
    unsafe { LP_PERI::steal() }.clk_en.set_ck_en_lp_i2cmst();
    configure_clock();
}

/// Closes the master's bus clock gate.
///
/// The 160 MHz reference is left running because other peripherals share it.
pub fn disable_clock() {
    unsafe { LP_PERI::steal() }.clk_en.clr_ck_en_lp_i2cmst();
}

/// Whether the master's bus clock gate is open.
pub fn clock_enabled() -> bool {
    unsafe { LP_PERI::steal() }.clk_en.ck_en_lp_i2cmst()
}

/// Pulses the master's reset.
///
/// Discards any transfer that is stuck on the bus; the clock source has to be
/// selected again afterwards.
pub fn reset() {
    let lp_peri = unsafe { LP_PERI::steal() };
    lp_peri.reset_en.set_rst_en_lp_i2cmst();
    lp_peri.reset_en.clr_rst_en_lp_i2cmst();
}

/// Selects the 160 MHz reference as the master's clock source.
fn configure_clock() {
    unsafe { LP_I2C_ANA_MST::steal() }
        .clk160m
        .set_clk_i2c_mst_sel_160m();
}

/// Opens the bus clock gate if somebody closed it.
#[inline]
fn ensure_clock() {
    if !clock_enabled() {
        enable_clock();
    }
}

/// Points the master at `block`.
#[inline]
fn select_block(block: Block) -> Result<(), Error> {
    let sel = block.select_mask().ok_or(Error::UnknownBlock(block.id()))?;
    let mst = unsafe { LP_I2C_ANA_MST::steal() };

    // One master at a time: drop every other select bit first.
    mst.ana_conf2.write_bits(0);
    mst.ana_conf1.write_bits(0);
    mst.ana_conf2.modify_bits(|r| r | sel);
    Ok(())
}

/// Waits until the master reports the bus idle.
#[inline]
fn wait_idle() -> Result<(), Error> {
    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let mut timeout = TIMEOUT_ITERATIONS;
    while mst.i2c0_ctrl.read_bits() & BUSY != 0 {
        if timeout == 0 {
            return Err(Error::Timeout);
        }
        timeout -= 1;
        core::hint::spin_loop();
    }
    Ok(())
}

/// One read transaction. The caller must hold a [`CriticalSection`].
fn transfer_read(block: Block, addr: u8) -> Result<u8, Error> {
    ensure_clock();
    select_block(block)?;
    wait_idle()?;

    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let cmd = ((block.id() as u32) << SLAVE_SHIFT) | ((addr as u32) << ADDR_SHIFT);
    mst.i2c0_ctrl.write_i2c0_ctrl(cmd);
    wait_idle()?;

    Ok(((mst.i2c0_ctrl.read_bits() >> DATA_SHIFT) & 0xff) as u8)
}

/// One write transaction. The caller must hold a [`CriticalSection`].
fn transfer_write(block: Block, addr: u8, value: u8) -> Result<(), Error> {
    ensure_clock();
    select_block(block)?;
    wait_idle()?;

    let mst = unsafe { LP_I2C_ANA_MST::steal() };
    let cmd = ((block.id() as u32) << SLAVE_SHIFT)
        | ((addr as u32) << ADDR_SHIFT)
        | ((value as u32) << DATA_SHIFT)
        | WRITE_EN;
    mst.i2c0_ctrl.write_i2c0_ctrl(cmd);
    wait_idle()
}

#[cold]
fn panic_regi2c(err: Error, register: Register) -> ! {
    panic_raw(err, register.block, register.addr)
}

#[cold]
fn panic_raw(err: Error, block: Block, addr: u8) -> ! {
    error!(
        "regi2c: block {:#x} register {:#x}: {}",
        block.id(),
        addr,
        match err {
            Error::UnknownBlock(_) => "unknown analog block",
            Error::Timeout => "transaction timed out",
        }
    );
    match err {
        Error::UnknownBlock(id) => panic!("regi2c: unknown analog block {id:#04x}"),
        Error::Timeout => panic!(
            "regi2c: timeout on block {:#04x} register {addr:#04x}",
            block.id()
        ),
    }
}

/// Powers the analog I2C bus used by the SAR ADC and the temperature sensor.
///
/// The PLL blocks are always powered; this path exists because the ADC and
/// TSENS share the same bus but sit behind a separate power switch.
pub fn sar_periph_enable() {
    let pmu = unsafe { PMU::steal() };
    // Recommended by the digital design: reset the bus, power it, release the
    // reset, waiting 1 µs in between.
    pmu.rf_pwc.clr_perif_i2c_rstb();
    crate::delay::Delay::init(unsafe { SYSTIMER::steal() }).delay_us(1);
    pmu.rf_pwc.set_xpd_perif_i2c();
    pmu.rf_pwc.set_perif_i2c_rstb();
}

/// Removes power from the SAR ADC / temperature sensor analog bus.
pub fn sar_periph_disable() {
    unsafe { PMU::steal() }.rf_pwc.clr_xpd_perif_i2c();
}

/// `I2C0_CTRL` bit that starts a write transfer.
const WRITE_EN: u32 = 1 << 24;
/// `I2C0_CTRL` bit that reports a transfer in progress.
const BUSY: u32 = 1 << 25;
const DATA_SHIFT: u32 = 16;
const ADDR_SHIFT: u32 = 8;
const SLAVE_SHIFT: u32 = 0;

/// Polling budget for one transfer, roughly 100 µs at any plausible core clock.
const TIMEOUT_ITERATIONS: u32 = 100_000;
