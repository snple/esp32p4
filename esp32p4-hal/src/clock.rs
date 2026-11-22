//! The system clock tree.
//!
//! `ROOT_CLK` is muxed from `XTAL_CLK` (40 MHz), `CPLL_CLK` (360 or 400 MHz) or
//! `RC_FAST_CLK` (~17.5 MHz), and a chain of dividers turns it into the SoC
//! clocks:
//!
//! ```text
//! XTAL 40M ─┐
//! CPLL     ─┼─ ROOT ─/CPU─ CPU ─/MEM─ MEM ─/SYS─ SYS ─/APB─ APB_CLK
//! RC_FAST  ─┘                                                     (100M max)
//!
//! MPLL 500M ─/─ PLL_F50M / PLL_F25M ──┐
//! SPLL 480M ─/─ PLL_F240M … F20M ─────┼─ peripheral clock muxes ─ APB_CLK
//! XTAL/RC_FAST/PLL_LP ────────────────┘
//! ```
//!
//! [`ClockTree`] owns the two controllers that shape this tree — the HP
//! clock/reset controller and the LP/always-on one — reads the resulting
//! frequencies, and programs the dividers and selectors. It derefs to
//! `HP_SYS_CLKRST`'s register block, so it can be handed to the drivers that take
//! `&hp_sys_clkrst::RegisterBlock`:
//!
//! ```ignore
//! let clocks = ClockTree::init(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST);
//! let group = TimerGroup0::init(dp.TIMG0, &clocks);   // Deref to the registers
//!
//! defmt::info!("cpu {} Hz, apb {} Hz", clocks.cpu_hz(), clocks.apb_hz());
//! ```
//!
//! # What is configured elsewhere
//!
//! The MPLL is brought up by [`crate::psram`], and the audio and SDIO PLLs by
//! whatever uses them. The two PLLs that shape this tree — the CPLL behind
//! `ROOT_CLK` and the SPLL behind the peripheral reference clocks — can be
//! reprogrammed here, divider and calibration included:
//! [`ClockTree::set_pll_frequency`] writes the divider on the analog bus
//! ([`crate::regi2c`]) and runs the self-calibration. Their frequencies are
//! taken from a [`Sources`] table and updated when the tree programs them; with
//! an [`Efuse`](crate::efuse::Efuse) handle, [`ClockTree::init_from_efuse`]
//! fills in the CPLL frequency that matches the die revision.
//!
//! # Changing the CPU frequency
//!
//! [`ClockTree::set_cpu_frequency`] rewrites the divider chain in the order the
//! hardware requires and is `unsafe`: the flash and PSRAM interfaces are timed
//! for the current frequency, and a cache or MSPI re-tune (which ESP-IDF does
//! around such a switch) is the caller's responsibility. Moving to a frequency
//! the CPLL does not produce yet takes one more step — switch `ROOT_CLK` off
//! the CPLL and retune it first:
//!
//! ```ignore
//! // SAFETY: the flash has its own clock and nothing is mapped in PSRAM.
//! unsafe {
//!     clocks.set_root_source(RootSource::Xtal);
//!     clocks.set_pll_frequency(Pll::Cpll, 400);
//!     clocks.set_cpu_frequency(400);
//! }
//! ```

use core::cell::Cell;
use core::ops::Deref;

use esp32p4_pac::pmu::ImmHpCkPower;
use esp32p4_pac::{HP_SYS_CLKRST, LP_AON_CLKRST, PMU, hp_sys_clkrst};

use crate::efuse::chip_version;
use crate::regi2c::{self, cpll, spll};

/// Crystal frequency: the board runs a 40 MHz XTAL.
pub const XTAL_HZ: u32 = 40_000_000;
/// Nominal `RC_FAST_CLK` frequency.
pub const RC_FAST_HZ: u32 = 17_500_000;
/// Nominal `RC_SLOW_CLK` frequency.
pub const RC_SLOW_HZ: u32 = 136_000;
/// Nominal 32 kHz crystal frequency.
pub const XTAL32K_HZ: u32 = 32_768;
/// Nominal external slow-oscillator frequency.
pub const OSC_SLOW_HZ: u32 = 32_768;
/// `PLL_LP_CLK`, the LP PLL's output.
pub const PLL_LP_HZ: u32 = 8_000_000;
/// `MPLL_CLK` as [`crate::psram`] configures it.
pub const MPLL_HZ: u32 = 500_000_000;
/// `SPLL_CLK`, fixed by the analog configuration.
pub const SPLL_HZ: u32 = 480_000_000;
/// `CPLL_CLK` on revisions that run the core at 360 MHz.
pub const CPLL_360M_HZ: u32 = 360_000_000;
/// `CPLL_CLK` on revisions that run the core at 400 MHz.
pub const CPLL_400M_HZ: u32 = 400_000_000;

/// Highest legal `CPU_CLK`.
pub const CPU_MAX_HZ: u32 = 400_000_000;
/// Highest legal `MEM_CLK`.
pub const MEM_MAX_HZ: u32 = 200_000_000;
/// Highest legal `SYS_CLK`.
pub const SYS_MAX_HZ: u32 = 200_000_000;
/// Highest legal `APB_CLK`.
pub const APB_MAX_HZ: u32 = 100_000_000;

/// `ROOT_CLK_CTRL0.REG_SOC_CLK_DIV_UPDATE`: set to latch new dividers, clears
/// once they are in effect.
const SOC_CLK_DIV_UPDATE_BIT: u32 = 1 << 4;

/// `PMU_IMM_HP_CK_POWER` carries the PLL ties in single-bit groups, with the
/// CPLL first and the APLL last (TRM, `PMU_IMM_HP_CK_POWER`).
const TIE_HIGH_XPD_CPLL: u32 = ImmHpCkPower::TIE_HIGH_XPD_PLL_OFFSET;
const TIE_HIGH_XPD_SPLL: u32 = TIE_HIGH_XPD_CPLL + 1;
const TIE_HIGH_XPD_CPLL_I2C: u32 = ImmHpCkPower::TIE_HIGH_XPD_PLL_I2C_OFFSET;
const TIE_HIGH_XPD_SPLL_I2C: u32 = TIE_HIGH_XPD_CPLL_I2C + 1;
const TIE_HIGH_GLOBAL_CPLL_ICG: u32 = ImmHpCkPower::TIE_HIGH_GLOBAL_PLL_ICG_OFFSET;
const TIE_HIGH_GLOBAL_SPLL_ICG: u32 = TIE_HIGH_GLOBAL_CPLL_ICG + 1;
const TIE_LOW_XPD_CPLL: u32 = ImmHpCkPower::TIE_LOW_XPD_PLL_OFFSET;
const TIE_LOW_XPD_SPLL: u32 = TIE_LOW_XPD_CPLL + 1;
const TIE_LOW_XPD_CPLL_I2C: u32 = ImmHpCkPower::TIE_LOW_XPD_PLL_I2C_OFFSET;
const TIE_LOW_XPD_SPLL_I2C: u32 = TIE_LOW_XPD_CPLL_I2C + 1;
const TIE_LOW_GLOBAL_CPLL_ICG: u32 = ImmHpCkPower::TIE_LOW_GLOBAL_PLL_ICG_OFFSET;
const TIE_LOW_GLOBAL_SPLL_ICG: u32 = TIE_LOW_GLOBAL_CPLL_ICG + 1;

/// `OC_REF_DIV`: charge-pump current 5, reference divider 0, feedback
/// calibration off — the byte ESP-IDF writes before calibrating.
const OC_REF_DIV_VALUE: u8 = 5 << 4;
/// `OC_DCUR`: `DLREF_SEL` 1, `DHREF_SEL` 3, pull-down current 3.
const OC_DCUR_VALUE: u8 = (1 << 6) | (3 << 4) | 3;
/// Settling time ESP-IDF waits after calibration before stopping it, in µs.
const PLL_CALIBRATION_SETTLE_US: u32 = 10;

/// Bumped whenever the clock tree changes, so derived rates can be re-derived.
static GENERATION: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);

/// A counter that changes whenever any of the setters below moves the clock tree.
///
/// The CLINT machine timer counts CPU cycles, so anything that converts between
/// it and wall-clock time — [`crate::delay`] parking the core, for instance — has
/// to notice that the CPU frequency moved. Reading this before and after caching a
/// conversion is enough to tell.
#[inline]
pub fn generation() -> u32 {
    GENERATION.load(core::sync::atomic::Ordering::Acquire)
}

/// Marks the clock tree as changed.
fn bump_generation() {
    GENERATION.fetch_add(1, core::sync::atomic::Ordering::Release);
}

/// The frequencies of the clock sources, as the board runs them.
///
/// The tree takes them as an input rather than deriving everything from the
/// registers: the MPLL and the APLL are started by other drivers, and the
/// analog dividers of the two system PLLs only decode with the die revision at
/// hand. [`Sources::DEFAULT`] matches a 40 MHz crystal, a 360 MHz CPLL, the
/// 500 MHz MPLL that [`crate::psram`] starts and the 480 MHz SPLL.
/// [`Sources::from_efuse`] replaces the CPLL entry with the frequency the die
/// actually runs, and [`ClockTree::set_pll_frequency`] updates the table when
/// the tree reprograms a PLL.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Sources {
    /// The crystal.
    pub xtal_hz: u32,
    /// `CPLL_CLK`, 360 or 400 MHz.
    pub cpll_hz: u32,
    /// `MPLL_CLK`, 500 MHz.
    pub mpll_hz: u32,
    /// `SPLL_CLK`, 480 MHz.
    pub spll_hz: u32,
    /// `RC_FAST_CLK`, about 17.5 MHz.
    pub rc_fast_hz: u32,
}

impl Sources {
    /// The frequencies a stock board runs at.
    pub const DEFAULT: Self = Self {
        xtal_hz: XTAL_HZ,
        cpll_hz: CPLL_360M_HZ,
        mpll_hz: MPLL_HZ,
        spll_hz: SPLL_HZ,
        rc_fast_hz: RC_FAST_HZ,
    };

    /// Read the CPLL frequency from the chip version reported by eFuse.
    ///
    /// Everything else keeps [`Sources::DEFAULT`]: the crystal, MPLL and SPLL
    /// do not depend on the die revision.
    pub fn from_efuse(efuse: &crate::efuse::Efuse) -> Self {
        Self {
            cpll_hz: efuse.chip_version().cpll_hz(),
            ..Self::DEFAULT
        }
    }
}

impl Default for Sources {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The source `ROOT_CLK` is muxed from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RootSource {
    /// The 40 MHz crystal.
    Xtal,
    /// `CPLL_CLK`.
    Cpll,
    /// `RC_FAST_CLK`.
    RcFast,
}

impl RootSource {
    const fn select(self) -> u32 {
        match self {
            Self::Xtal => 0,
            Self::Cpll => 1,
            Self::RcFast => 2,
        }
    }

    const fn from_select(value: u32) -> Self {
        match value {
            1 => Self::Cpll,
            2 => Self::RcFast,
            _ => Self::Xtal,
        }
    }
}

/// The integer dividers between `ROOT_CLK` and the SoC buses.
///
/// Each one divides the previous clock: `CPU = ROOT / cpu`, `MEM = CPU / mem`,
/// `SYS = MEM / sys`, `APB = SYS / apb`. A value is 1 to 256.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SocDividers {
    /// `ROOT_CLK` → `CPU_CLK`.
    pub cpu: u32,
    /// `CPU_CLK` → `MEM_CLK`.
    pub mem: u32,
    /// `MEM_CLK` → `SYS_CLK`.
    pub sys: u32,
    /// `SYS_CLK` → `APB_CLK`.
    pub apb: u32,
}

/// A PLL-derived reference clock for peripherals.
///
/// `PLL_F50M` and `PLL_F25M` divide the MPLL; the others divide the SPLL. Each
/// has its own enable bit and an 8-bit divider, so the frequency is
/// `PLL / (CLK_DIV_NUM + 1)` and [`RefClock::nominal_hz`] is only the value
/// after a reset.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefClock {
    /// `PLL_F20M_CLK`, from the SPLL.
    F20M,
    /// `PLL_F25M_CLK`, from the MPLL.
    F25M,
    /// `PLL_F50M_CLK`, from the MPLL.
    F50M,
    /// `PLL_F80M_CLK`, from the SPLL.
    F80M,
    /// `PLL_F120M_CLK`, from the SPLL.
    F120M,
    /// `PLL_F160M_CLK`, from the SPLL.
    F160M,
    /// `PLL_F240M_CLK`, from the SPLL.
    F240M,
}

impl RefClock {
    /// All reference clocks, for iterating over them.
    pub const ALL: [Self; 7] = [
        Self::F20M,
        Self::F25M,
        Self::F50M,
        Self::F80M,
        Self::F120M,
        Self::F160M,
        Self::F240M,
    ];

    /// The frequency the clock has with its reset divider and source.
    pub const fn nominal_hz(self) -> u32 {
        match self {
            Self::F20M => 20_000_000,
            Self::F25M => 25_000_000,
            Self::F50M => 50_000_000,
            Self::F80M => 80_000_000,
            Self::F120M => 120_000_000,
            Self::F160M => 160_000_000,
            Self::F240M => 240_000_000,
        }
    }

    /// The PLL this clock divides.
    pub const fn is_from_mpll(self) -> bool {
        matches!(self, Self::F25M | Self::F50M)
    }
}

/// A clock source the LP controller gates for the HP domain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpSource {
    /// `XTAL_CLK`.
    Xtal,
    /// `RC_FAST_CLK` (`FOSC`).
    RcFast,
    /// `CPLL_CLK`.
    Cpll,
    /// `SPLL_CLK`.
    Spll,
    /// `MPLL_CLK`.
    Mpll,
}

/// The two PLLs whose divider and calibration this module drives.
///
/// The CPU PLL feeds `ROOT_CLK`; the system PLL feeds the peripheral reference
/// clocks. Both sit on the analog I2C bus ([`crate::regi2c`]) and answer only
/// while they are powered — the bootloader leaves both running.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll {
    /// `CPLL_CLK`, 360 or 400 MHz depending on the die.
    Cpll,
    /// `SPLL_CLK`, 480 MHz.
    Spll,
}

impl Pll {
    /// The frequencies the SoC supports, in MHz.
    ///
    /// The divider chain, the reference-clock dividers and the peripheral
    /// ratings are all built around these values; the analog divider cannot
    /// reach every multiplier in between, so they are also the only frequencies
    /// that can be selected (measured on a v1.3 die, where the SPLL's feedback
    /// field forces bit 2 of the multiplier set).
    pub const fn supported_mhz(self) -> &'static [u32] {
        match self {
            Self::Cpll => &[360, 400],
            Self::Spll => &[480],
        }
    }
}

/// The source of `LP_SLOW_CLK`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpSlowSource {
    /// The internal slow RC oscillator, about 136 kHz.
    RcSlow,
    /// The external 32 kHz crystal.
    Xtal32k,
    /// The external slow oscillator on `XTAL_32K_N`.
    OscSlow,
}

impl LpSlowSource {
    const fn select(self) -> u32 {
        match self {
            Self::RcSlow => 0,
            Self::Xtal32k => 1,
            Self::OscSlow => 3,
        }
    }

    const fn from_select(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::RcSlow),
            1 => Some(Self::Xtal32k),
            3 => Some(Self::OscSlow),
            _ => None,
        }
    }

    const fn frequency_hz(self) -> u32 {
        match self {
            Self::RcSlow => RC_SLOW_HZ,
            Self::Xtal32k => XTAL32K_HZ,
            Self::OscSlow => OSC_SLOW_HZ,
        }
    }
}

/// The source of `LP_FAST_CLK`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFastSource {
    /// `RC_FAST_CLK`, about 17.5 MHz.
    RcFast,
    /// `XTAL_CLK`, 40 MHz.
    Xtal,
    /// `PLL_LP_CLK`, 8 MHz.
    PllLp,
}

impl LpFastSource {
    const fn select(self) -> u32 {
        match self {
            Self::RcFast => 0,
            Self::Xtal => 1,
            Self::PllLp => 2,
        }
    }

    const fn from_select(value: u32) -> Self {
        match value {
            1 => Self::Xtal,
            2 => Self::PllLp,
            _ => Self::RcFast,
        }
    }

    const fn frequency_hz(self) -> u32 {
        match self {
            Self::RcFast => RC_FAST_HZ,
            Self::Xtal => XTAL_HZ,
            Self::PllLp => PLL_LP_HZ,
        }
    }
}

/// The system clock tree.
///
/// Owns the HP clock/reset controller and the LP/always-on one, and derefs to
/// the HP register block so drivers that take `&hp_sys_clkrst::RegisterBlock`
/// accept `&ClockTree`.
pub struct ClockTree {
    hp: HP_SYS_CLKRST,
    lp: LP_AON_CLKRST,
    /// What the tree runs at. Reprogramming a PLL moves an entry.
    sources: Cell<Sources>,
}

impl ClockTree {
    /// Take both clock controllers.
    ///
    /// Nothing is written to the hardware: the tree the bootloader left behind
    /// stays as it is.
    pub fn init(hp: HP_SYS_CLKRST, lp: LP_AON_CLKRST) -> Self {
        Self::init_with(hp, lp, Sources::DEFAULT)
    }

    /// Take both clock controllers with an explicit source table.
    ///
    /// Use this when the board runs a different crystal or a CPLL at 400 MHz
    /// instead of 360 MHz, so the reported frequencies match the hardware.
    pub fn init_with(hp: HP_SYS_CLKRST, lp: LP_AON_CLKRST, sources: Sources) -> Self {
        Self {
            hp,
            lp,
            sources: Cell::new(sources),
        }
    }

    /// Take both controllers and derive the CPLL frequency from eFuse.
    ///
    /// This is the recommended constructor when an [`Efuse`](crate::efuse::Efuse)
    /// handle is available: dies before v3.0 run the CPLL at 360 MHz, v3.0 and
    /// later at 400 MHz.
    pub fn init_from_efuse(
        hp: HP_SYS_CLKRST,
        lp: LP_AON_CLKRST,
        efuse: &crate::efuse::Efuse,
    ) -> Self {
        Self::init_with(hp, lp, Sources::from_efuse(efuse))
    }

    /// Give the two controllers back.
    pub fn release(self) -> (HP_SYS_CLKRST, LP_AON_CLKRST) {
        (self.hp, self.lp)
    }

    /// The source table this tree reports against.
    pub fn sources(&self) -> Sources {
        self.sources.get()
    }

    /// The LP/always-on clock/reset registers.
    pub fn lp(&self) -> &esp32p4_pac::lp_aon_clkrst::RegisterBlock {
        &self.lp
    }

    // ========================================================================
    // Reading the HP tree
    // ========================================================================

    /// The source `ROOT_CLK` is currently muxed from.
    pub fn root_source(&self) -> RootSource {
        RootSource::from_select(
            self.lp
                .lp_aonclkrst_hp_clk_ctrl
                .lp_aonclkrst_hp_root_clk_src_sel(),
        )
    }

    /// The frequency of `ROOT_CLK`, i.e. of the selected source.
    pub fn root_hz(&self) -> u32 {
        let sources = self.sources();
        match self.root_source() {
            RootSource::Xtal => sources.xtal_hz,
            RootSource::Cpll => sources.cpll_hz,
            RootSource::RcFast => sources.rc_fast_hz,
        }
    }

    /// The divider chain from `ROOT_CLK` to the buses.
    ///
    /// The CPU divider's fractional part is reported as zero; [`Self::cpu_hz`]
    /// takes it into account.
    pub fn dividers(&self) -> SocDividers {
        SocDividers {
            cpu: self.hp.root_clk_ctrl0.reg_cpu_clk_div_num() + 1,
            mem: self.hp.root_clk_ctrl1.reg_mem_clk_div_num() + 1,
            sys: self.hp.root_clk_ctrl1.reg_sys_clk_div_num() + 1,
            apb: self.hp.root_clk_ctrl2.reg_apb_clk_div_num() + 1,
        }
    }

    /// `CPU_CLK`, as the hardware divides `ROOT_CLK`.
    ///
    /// The CPU divider is fractional: `ROOT / (integer + numerator/denominator)`
    /// with an all-zero fraction meaning an integer divider.
    pub fn cpu_hz(&self) -> u32 {
        let ctrl = &self.hp.root_clk_ctrl0;
        let integer = (ctrl.reg_cpu_clk_div_num() + 1) as u64;
        let numerator = ctrl.reg_cpu_clk_div_numerator() as u64;
        let denominator = ctrl.reg_cpu_clk_div_denominator() as u64;
        let (numerator, denominator) = if denominator == 0 {
            (0, 1)
        } else {
            (numerator, denominator)
        };
        (self.root_hz() as u64 * denominator / (integer * denominator + numerator)) as u32
    }

    /// `MEM_CLK`.
    pub fn mem_hz(&self) -> u32 {
        self.cpu_hz() / self.dividers().mem
    }

    /// `SYS_CLK`.
    pub fn sys_hz(&self) -> u32 {
        self.mem_hz() / self.dividers().sys
    }

    /// `APB_CLK`.
    pub fn apb_hz(&self) -> u32 {
        self.sys_hz() / self.dividers().apb
    }

    /// Whether the root clock gate is open.
    pub fn root_clock_enabled(&self) -> bool {
        self.lp
            .lp_aonclkrst_hp_clk_ctrl
            .lp_aonclkrst_hp_root_clk_en()
    }

    /// Whether `src` is gated on towards the HP domain.
    pub fn source_enabled(&self, src: HpSource) -> bool {
        let ctrl = &self.lp.lp_aonclkrst_hp_clk_ctrl;
        match src {
            HpSource::Xtal => ctrl.lp_aonclkrst_hp_xtal_40m_clk_en(),
            HpSource::RcFast => ctrl.lp_aonclkrst_hp_fosc_20m_clk_en(),
            HpSource::Cpll => ctrl.lp_aonclkrst_hp_cpll_400m_clk_en(),
            HpSource::Spll => ctrl.lp_aonclkrst_hp_spll_480m_clk_en(),
            HpSource::Mpll => ctrl.lp_aonclkrst_hp_mpll_500m_clk_en(),
        }
    }

    // ========================================================================
    // PLLs
    // ========================================================================

    /// The frequency the tree reports for a PLL.
    ///
    /// This is what the tree was built with or last programmed, not a reading
    /// of the analog block; [`Self::pll_multiplier`] reads that.
    pub fn pll_hz(&self, pll: Pll) -> u32 {
        let sources = self.sources();
        match pll {
            Pll::Cpll => sources.cpll_hz,
            Pll::Spll => sources.spll_hz,
        }
    }

    /// The multiplier the analog block holds, i.e. `PLL / XTAL`.
    ///
    /// Read from the analog divider, so it is what the hardware was last told
    /// rather than what the tree reports; compare it with [`Self::pll_hz`] to
    /// see whether the two agree. For the frequencies the SoC runs at, the
    /// stored value is the multiplier (see [`Pll::supported_mhz`] for the
    /// narrower set the analog block can actually select).
    pub fn pll_multiplier(&self, pll: Pll) -> u32 {
        let encoded = u32::from(pll_divider(pll).read());
        if chip_version().direct_pll_divider() {
            encoded
        } else {
            encoded + 4
        }
    }

    /// Powers a PLL's analog block and gates its output towards the HP domain.
    ///
    /// Both PLLs are running when the program starts, so this only matters for
    /// a PLL a previous [`disable_pll`](Self::disable_pll) closed. The analog
    /// register file loses its contents with the power, so the PLL comes back at
    /// the block's reset divider — call
    /// [`set_pll_frequency`](Self::set_pll_frequency) before using it.
    pub fn enable_pll(&self, pll: Pll) {
        match pll {
            Pll::Cpll => {
                set_pmu_tie(TIE_HIGH_XPD_CPLL, true);
                set_pmu_tie(TIE_HIGH_XPD_CPLL_I2C, true);
                set_pmu_tie(TIE_HIGH_GLOBAL_CPLL_ICG, true);
                self.set_source_enabled(HpSource::Cpll, true);
            }
            Pll::Spll => {
                set_pmu_tie(TIE_HIGH_XPD_SPLL, true);
                set_pmu_tie(TIE_HIGH_XPD_SPLL_I2C, true);
                set_pmu_tie(TIE_HIGH_GLOBAL_SPLL_ICG, true);
                self.set_source_enabled(HpSource::Spll, true);
            }
        }
    }

    /// Powers a PLL down: the output gate closes first, then the analog block.
    ///
    /// The divider does not survive: the analog registers reset with the power,
    /// so bring the PLL back with [`enable_pll`](Self::enable_pll) and a
    /// [`set_pll_frequency`](Self::set_pll_frequency) call.
    ///
    /// # Panics
    ///
    /// If `ROOT_CLK` runs on the PLL; switch the root away first.
    pub fn disable_pll(&self, pll: Pll) {
        assert!(
            !(pll == Pll::Cpll && self.root_source() == RootSource::Cpll),
            "switch ROOT_CLK off the CPLL before powering it down"
        );
        match pll {
            Pll::Cpll => {
                self.set_source_enabled(HpSource::Cpll, false);
                set_pmu_tie(TIE_LOW_GLOBAL_CPLL_ICG, true);
                set_pmu_tie(TIE_LOW_XPD_CPLL, true);
                set_pmu_tie(TIE_LOW_XPD_CPLL_I2C, true);
            }
            Pll::Spll => {
                self.set_source_enabled(HpSource::Spll, false);
                set_pmu_tie(TIE_LOW_GLOBAL_SPLL_ICG, true);
                set_pmu_tie(TIE_LOW_XPD_SPLL, true);
                set_pmu_tie(TIE_LOW_XPD_SPLL_I2C, true);
            }
        }
    }

    /// Powers a PLL up, programs its analog divider for `mhz` and calibrates it.
    ///
    /// The output is `XTAL × multiplier` (see [`Pll::supported_mhz`] for the
    /// frequencies the SoC runs at), written on the analog bus while the
    /// calibration state machine runs; the call returns once the block reports
    /// the calibration finished. [`Sources`] follows, so [`Self::pll_hz`],
    /// [`Self::ref_hz`] and [`Self::root_hz`] report the new frequency.
    ///
    /// # Panics
    ///
    /// If `mhz` is not one of [`Pll::supported_mhz`], if it is not a whole
    /// multiple of the crystal, or if `ROOT_CLK` still runs on the CPLL.
    ///
    /// # Safety
    ///
    /// The PLL stops producing a usable clock while the divider changes, so
    /// everything derived from it misbehaves: switch `ROOT_CLK` to another
    /// source before retuning the CPLL, and stop the peripherals that use the
    /// reference clocks before retuning the SPLL. The USB Serial/JTAG PHY takes
    /// its 48 MHz from the SPLL, so retuning it drops a host connection for a
    /// moment.
    pub unsafe fn set_pll_frequency(&self, pll: Pll, mhz: u32) -> u32 {
        assert!(
            Pll::supported_mhz(pll).contains(&mhz),
            "the SoC does not run this PLL at {mhz} MHz"
        );
        assert!(
            !(pll == Pll::Cpll && self.root_source() == RootSource::Cpll),
            "switch ROOT_CLK off the CPLL before retuning it"
        );

        let xtal_hz = self.sources().xtal_hz;
        let target_hz = mhz * 1_000_000;
        assert!(
            target_hz.is_multiple_of(xtal_hz),
            "a PLL frequency has to be a whole multiple of the crystal"
        );
        let multiplier = target_hz / xtal_hz;
        assert!(
            (4..=255).contains(&multiplier),
            "the analog divider cannot reach a multiplier of {multiplier}"
        );
        let divider = if chip_version().direct_pll_divider() {
            multiplier
        } else {
            multiplier - 4
        };

        self.enable_pll(pll);

        // The calibration runs while the divider is written and ends once the
        // block has found the capacitance for the new frequency.
        self.start_pll_calibration(pll);
        let (ref_div, feedback, dcur) = pll_registers(pll);
        ref_div.write(OC_REF_DIV_VALUE);
        feedback.write(divider as u8);
        dcur.write(OC_DCUR_VALUE);
        self.wait_pll_calibration(pll);
        crate::delay::delay_us(PLL_CALIBRATION_SETTLE_US);
        self.stop_pll_calibration(pll);

        self.set_pll_hz(pll, target_hz);
        target_hz
    }

    /// Records a PLL frequency in the source table.
    fn set_pll_hz(&self, pll: Pll, hz: u32) {
        let mut sources = self.sources();
        match pll {
            Pll::Cpll => sources.cpll_hz = hz,
            Pll::Spll => sources.spll_hz = hz,
        }
        self.sources.set(sources);
        bump_generation();
    }

    fn start_pll_calibration(&self, pll: Pll) {
        let ctrl = &self.hp.ana_pll_ctrl0;
        match pll {
            Pll::Cpll => ctrl.clr_reg_cpu_pll_cal_stop(),
            Pll::Spll => ctrl.clr_reg_sys_pll_cal_stop(),
        }
    }

    fn wait_pll_calibration(&self, pll: Pll) {
        let ctrl = &self.hp.ana_pll_ctrl0;
        crate::spin::spin_wait("PLL calibration", || match pll {
            Pll::Cpll => ctrl.reg_cpu_pll_cal_end(),
            Pll::Spll => ctrl.reg_sys_pll_cal_end(),
        });
    }

    fn stop_pll_calibration(&self, pll: Pll) {
        let ctrl = &self.hp.ana_pll_ctrl0;
        match pll {
            Pll::Cpll => ctrl.set_reg_cpu_pll_cal_stop(),
            Pll::Spll => ctrl.set_reg_sys_pll_cal_stop(),
        }
    }

    // ========================================================================
    // Reference clocks
    // ========================================================================

    /// The PLL a reference clock divides, in Hz.
    fn ref_pll_hz(&self, clk: RefClock) -> u32 {
        let sources = self.sources();
        if clk.is_from_mpll() {
            sources.mpll_hz
        } else {
            sources.spll_hz
        }
    }

    /// The divider the reference clock is programmed with.
    pub fn ref_divider(&self, clk: RefClock) -> u32 {
        let hp = &self.hp;
        match clk {
            RefClock::F20M => hp.ref_clk_ctrl1.reg_ref_20m_clk_div_num() + 1,
            RefClock::F25M => hp.ref_clk_ctrl0.reg_ref_25m_clk_div_num() + 1,
            RefClock::F50M => hp.ref_clk_ctrl0.reg_ref_50m_clk_div_num() + 1,
            RefClock::F80M => hp.ref_clk_ctrl1.reg_ref_80m_clk_div_num() + 1,
            RefClock::F120M => hp.ref_clk_ctrl1.reg_ref_120m_clk_div_num() + 1,
            RefClock::F160M => hp.ref_clk_ctrl0.reg_ref_160m_clk_div_num() + 1,
            RefClock::F240M => hp.ref_clk_ctrl0.reg_ref_240m_clk_div_num() + 1,
        }
    }

    /// The frequency of a reference clock, whatever its divider is set to.
    pub fn ref_hz(&self, clk: RefClock) -> u32 {
        self.ref_pll_hz(clk) / self.ref_divider(clk)
    }

    /// Whether a reference clock is gated on.
    pub fn ref_enabled(&self, clk: RefClock) -> bool {
        let hp = &self.hp;
        match clk {
            RefClock::F20M => hp.ref_clk_ctrl2.reg_ref_20m_clk_en(),
            RefClock::F25M => hp.ref_clk_ctrl1.reg_ref_25m_clk_en(),
            RefClock::F50M => hp.ref_clk_ctrl1.reg_ref_50m_clk_en(),
            RefClock::F80M => hp.ref_clk_ctrl2.reg_ref_80m_clk_en(),
            RefClock::F120M => hp.ref_clk_ctrl2.reg_ref_120m_clk_en(),
            RefClock::F160M => hp.ref_clk_ctrl2.reg_ref_160m_clk_en(),
            RefClock::F240M => hp.ref_clk_ctrl1.reg_ref_240m_clk_en(),
        }
    }

    /// Reprogram a reference clock's divider.
    ///
    /// The divider is 1 to 256; the result is `PLL / divider`, which may exceed
    /// the clock's nominal frequency and the peripheral's rating. Stop the
    /// peripherals that use the clock first.
    pub fn set_ref_divider(&self, clk: RefClock, divider: u32) {
        assert!(
            (1..=256).contains(&divider),
            "a reference clock divides by 1..=256"
        );
        let encoded = divider - 1;
        let hp = &self.hp;
        match clk {
            RefClock::F20M => hp.ref_clk_ctrl1.write_reg_ref_20m_clk_div_num(encoded),
            RefClock::F25M => hp.ref_clk_ctrl0.write_reg_ref_25m_clk_div_num(encoded),
            RefClock::F50M => hp.ref_clk_ctrl0.write_reg_ref_50m_clk_div_num(encoded),
            RefClock::F80M => hp.ref_clk_ctrl1.write_reg_ref_80m_clk_div_num(encoded),
            RefClock::F120M => hp.ref_clk_ctrl1.write_reg_ref_120m_clk_div_num(encoded),
            RefClock::F160M => hp.ref_clk_ctrl0.write_reg_ref_160m_clk_div_num(encoded),
            RefClock::F240M => hp.ref_clk_ctrl0.write_reg_ref_240m_clk_div_num(encoded),
        }
    }

    /// Gate a reference clock on or off.
    ///
    /// The clock must exist in the tree before a peripheral selects it: the
    /// gate bit is what turns the divider's output on.
    pub fn enable_ref(&self, clk: RefClock, enable: bool) {
        let hp = &self.hp;
        match (clk, enable) {
            (RefClock::F20M, true) => hp.ref_clk_ctrl2.set_reg_ref_20m_clk_en(),
            (RefClock::F20M, false) => hp.ref_clk_ctrl2.clr_reg_ref_20m_clk_en(),
            (RefClock::F25M, true) => hp.ref_clk_ctrl1.set_reg_ref_25m_clk_en(),
            (RefClock::F25M, false) => hp.ref_clk_ctrl1.clr_reg_ref_25m_clk_en(),
            (RefClock::F50M, true) => hp.ref_clk_ctrl1.set_reg_ref_50m_clk_en(),
            (RefClock::F50M, false) => hp.ref_clk_ctrl1.clr_reg_ref_50m_clk_en(),
            (RefClock::F80M, true) => hp.ref_clk_ctrl2.set_reg_ref_80m_clk_en(),
            (RefClock::F80M, false) => hp.ref_clk_ctrl2.clr_reg_ref_80m_clk_en(),
            (RefClock::F120M, true) => hp.ref_clk_ctrl2.set_reg_ref_120m_clk_en(),
            (RefClock::F120M, false) => hp.ref_clk_ctrl2.clr_reg_ref_120m_clk_en(),
            (RefClock::F160M, true) => hp.ref_clk_ctrl2.set_reg_ref_160m_clk_en(),
            (RefClock::F160M, false) => hp.ref_clk_ctrl2.clr_reg_ref_160m_clk_en(),
            (RefClock::F240M, true) => hp.ref_clk_ctrl1.set_reg_ref_240m_clk_en(),
            (RefClock::F240M, false) => hp.ref_clk_ctrl1.clr_reg_ref_240m_clk_en(),
        }
    }

    // ========================================================================
    // LP clocks
    // ========================================================================

    /// The source of `LP_SLOW_CLK`, or `None` if the mux holds its reserved
    /// value.
    pub fn lp_slow_source(&self) -> Option<LpSlowSource> {
        LpSlowSource::from_select(self.lp.lp_aonclkrst_lp_clk_conf.lp_aonclkrst_slow_clk_sel())
    }

    /// The frequency of `LP_SLOW_CLK`.
    pub fn lp_slow_hz(&self) -> u32 {
        self.lp_slow_source().map_or(0, LpSlowSource::frequency_hz)
    }

    /// The source of `LP_FAST_CLK`.
    pub fn lp_fast_source(&self) -> LpFastSource {
        LpFastSource::from_select(self.lp.lp_aonclkrst_lp_clk_conf.lp_aonclkrst_fast_clk_sel())
    }

    /// The frequency of `LP_FAST_CLK`.
    pub fn lp_fast_hz(&self) -> u32 {
        self.lp_fast_source().frequency_hz()
    }

    /// The divider from `LP_DYN_FAST_CLK` to `LP_PERI_CLK`.
    pub fn lp_peri_divider(&self) -> u32 {
        self.lp
            .lp_aonclkrst_lp_clk_conf
            .lp_aonclkrst_lp_peri_div_num()
            + 1
    }

    /// The frequency of `LP_PERI_CLK`.
    ///
    /// While the chip is active `LP_DYN_FAST_CLK` is `LP_FAST_CLK`, so this is
    /// [`Self::lp_fast_hz`] divided by [`Self::lp_peri_divider`].
    pub fn lp_peri_hz(&self) -> u32 {
        self.lp_fast_hz() / self.lp_peri_divider()
    }

    /// Select the source of `LP_SLOW_CLK`.
    ///
    /// The LP watchdog counts on this clock, so its timeout changes with the
    /// source.
    pub fn set_lp_slow_source(&self, src: LpSlowSource) {
        self.lp
            .lp_aonclkrst_lp_clk_conf
            .write_lp_aonclkrst_slow_clk_sel(src.select());
    }

    /// Select the source of `LP_FAST_CLK`.
    pub fn set_lp_fast_source(&self, src: LpFastSource) {
        self.lp
            .lp_aonclkrst_lp_clk_conf
            .write_lp_aonclkrst_fast_clk_sel(src.select());
    }

    /// Divide `LP_PERI_CLK` from `LP_DYN_FAST_CLK`.
    ///
    /// The divider is 1 to 64.
    pub fn set_lp_peri_divider(&self, divider: u32) {
        assert!(
            (1..=64).contains(&divider),
            "the LP peripheral divider divides by 1..=64"
        );
        self.lp
            .lp_aonclkrst_lp_clk_conf
            .write_lp_aonclkrst_lp_peri_div_num(divider - 1);
    }

    // ========================================================================
    // Writing the HP tree
    // ========================================================================

    /// Gate a clock source on or off towards the HP domain.
    ///
    /// Gating a source that is still selected as `ROOT_CLK`, or that a
    /// peripheral clock derives from, stops the tree: switch
    /// [`root_source`](Self::root_source) first.
    pub fn set_source_enabled(&self, src: HpSource, enable: bool) {
        let ctrl = &self.lp.lp_aonclkrst_hp_clk_ctrl;
        match (src, enable) {
            (HpSource::Xtal, true) => ctrl.set_lp_aonclkrst_hp_xtal_40m_clk_en(),
            (HpSource::Xtal, false) => ctrl.clr_lp_aonclkrst_hp_xtal_40m_clk_en(),
            (HpSource::RcFast, true) => ctrl.set_lp_aonclkrst_hp_fosc_20m_clk_en(),
            (HpSource::RcFast, false) => ctrl.clr_lp_aonclkrst_hp_fosc_20m_clk_en(),
            (HpSource::Cpll, true) => ctrl.set_lp_aonclkrst_hp_cpll_400m_clk_en(),
            (HpSource::Cpll, false) => ctrl.clr_lp_aonclkrst_hp_cpll_400m_clk_en(),
            (HpSource::Spll, true) => ctrl.set_lp_aonclkrst_hp_spll_480m_clk_en(),
            (HpSource::Spll, false) => ctrl.clr_lp_aonclkrst_hp_spll_480m_clk_en(),
            (HpSource::Mpll, true) => ctrl.set_lp_aonclkrst_hp_mpll_500m_clk_en(),
            (HpSource::Mpll, false) => ctrl.clr_lp_aonclkrst_hp_mpll_500m_clk_en(),
        }
    }

    /// Switch `ROOT_CLK` to another source.
    ///
    /// The new source must already be running and its frequency must suit the
    /// current dividers. This only moves the mux; use
    /// [`set_cpu_frequency`](Self::set_cpu_frequency) to change the divider
    /// chain as well.
    ///
    /// # Safety
    ///
    /// Everything downstream runs off `ROOT_CLK`: flash, PSRAM and every
    /// peripheral. Switching to a faster source without retuning them first
    /// breaks the buses. The caller owns the consequences.
    pub unsafe fn set_root_source(&self, src: RootSource) {
        self.lp
            .lp_aonclkrst_hp_clk_ctrl
            .write_lp_aonclkrst_hp_root_clk_src_sel(src.select());
        bump_generation();
    }

    /// Program the whole divider chain at once.
    ///
    /// The dividers are written in the order the hardware needs, each latched
    /// with `SOC_CLK_DIV_UPDATE`, and the resulting frequencies are checked
    /// against [`CPU_MAX_HZ`], [`MEM_MAX_HZ`], [`SYS_MAX_HZ`] and
    /// [`APB_MAX_HZ`] first: the hardware silently corrects a chain that would
    /// exceed them, and the correction is not visible in the registers.
    ///
    /// `ROOT_CLK` itself is not switched; the dividers apply to whatever source
    /// is selected.
    ///
    /// # Safety
    ///
    /// As for [`set_root_source`](Self::set_root_source): flash and PSRAM are
    /// timed for the current frequencies.
    pub unsafe fn set_dividers(&self, dividers: SocDividers) {
        for divider in [dividers.cpu, dividers.mem, dividers.sys, dividers.apb] {
            assert!(
                (1..=256).contains(&divider),
                "an SoC divider divides by 1..=256"
            );
        }

        let cpu = self.root_hz() / dividers.cpu;
        let mem = cpu / dividers.mem;
        let sys = mem / dividers.sys;
        let apb = sys / dividers.apb;
        assert!(cpu <= CPU_MAX_HZ, "CPU_CLK would exceed its limit");
        assert!(mem <= MEM_MAX_HZ, "MEM_CLK would exceed its limit");
        assert!(sys <= SYS_MAX_HZ, "SYS_CLK would exceed its limit");
        assert!(apb <= APB_MAX_HZ, "APB_CLK would exceed its limit");

        // Rising frequencies need the faster clocks in place first and falling
        // ones the slower clocks first, so no intermediate state violates the
        // bus limits.
        if cpu > self.cpu_hz() {
            self.set_apb_divider(dividers.apb);
            self.set_sys_divider(dividers.sys);
            self.set_mem_divider(dividers.mem);
            self.set_cpu_divider(dividers.cpu);
        } else {
            self.set_cpu_divider(dividers.cpu);
            self.set_mem_divider(dividers.mem);
            self.set_sys_divider(dividers.sys);
            self.set_apb_divider(dividers.apb);
        }
        bump_generation();
    }

    /// Run the CPU on the CPLL at `mhz`.
    ///
    /// Supported frequencies are half and a quarter of the CPLL: 360, 180 and
    /// 90 MHz on a 360 MHz CPLL, 400, 200 and 100 MHz on a 400 MHz one. The
    /// divider chain is chosen so the buses stay inside their limits
    /// (`MEM` = CPU/2 and `APB` = CPU/4 for the top step, `APB` = CPU/2 for the
    /// middle one, everything undivided for the last), updated in the safe order
    /// and the mux is switched to the CPLL last, exactly like ESP-IDF's
    /// `rtc_clk.c`.
    ///
    /// # Safety
    ///
    /// - The CPLL must already run at [`Sources::cpll_hz`] and be gated on; this
    ///   driver does not start or calibrate it.
    /// - Flash and PSRAM are timed for the current frequency. ESP-IDF suspends
    ///   the external-memory cache and re-tunes the MSPI around the switch; a
    ///   program running from PSRAM has to do the same with
    ///   [`cache::suspend`](crate::cache::suspend) /
    ///   [`cache::resume`](crate::cache::resume).
    pub unsafe fn set_cpu_frequency(&self, mhz: u32) {
        let target = mhz.saturating_mul(1_000_000);
        let (mem, apb) = match mhz {
            360 | 400 => (2, 2),
            180 | 200 => (1, 2),
            90 | 100 => (1, 1),
            _ => panic!("unsupported CPU frequency"),
        };
        let cpll_hz = self.sources().cpll_hz;
        assert!(
            cpll_hz.is_multiple_of(target),
            "the CPU frequency must divide the CPLL"
        );
        assert!(self.source_enabled(HpSource::Cpll), "the CPLL is gated off");

        // The limits are checked against the CPLL, the source the tree ends up
        // on, not against whatever ROOT_CLK is running from now.
        let root = cpll_hz;
        let cpu_div = root / target;
        let cpu = root / cpu_div;
        let mem_hz = cpu / mem;
        let sys_hz = mem_hz;
        let apb_hz = sys_hz / apb;
        assert!(cpu <= CPU_MAX_HZ, "CPU_CLK would exceed its limit");
        assert!(mem_hz <= MEM_MAX_HZ, "MEM_CLK would exceed its limit");
        assert!(sys_hz <= SYS_MAX_HZ, "SYS_CLK would exceed its limit");
        assert!(apb_hz <= APB_MAX_HZ, "APB_CLK would exceed its limit");

        let rising = target > self.cpu_hz();
        // SAFETY: the caller vouched for the CPLL and the MSPI timing.
        unsafe {
            if rising {
                self.set_apb_divider(apb);
                self.set_sys_divider(1);
                self.set_mem_divider(mem);
                self.set_cpu_divider(cpu_div);
            } else {
                self.set_cpu_divider(cpu_div);
                self.set_mem_divider(mem);
                self.set_sys_divider(1);
                self.set_apb_divider(apb);
            }

            // `SOC_CLK_DIV_UPDATE` does not control the root mux, so the source
            // changes last, when the dividers already match the new frequency.
            self.set_root_source(RootSource::Cpll);
        }
    }

    /// Latch the divider registers into the clock tree.
    fn bus_update(&self) {
        let ctrl = &self.hp.root_clk_ctrl0;
        ctrl.trigger_reg_soc_clk_div_update();
        while ctrl.read_bits() & SOC_CLK_DIV_UPDATE_BIT != 0 {
            core::hint::spin_loop();
        }
    }

    fn set_cpu_divider(&self, divider: u32) {
        let ctrl = &self.hp.root_clk_ctrl0;
        ctrl.write_reg_cpu_clk_div_num(divider - 1);
        ctrl.write_reg_cpu_clk_div_numerator(0);
        ctrl.write_reg_cpu_clk_div_denominator(0);
        self.bus_update();
    }

    fn set_mem_divider(&self, divider: u32) {
        self.hp
            .root_clk_ctrl1
            .write_reg_mem_clk_div_num(divider - 1);
        self.bus_update();
    }

    fn set_sys_divider(&self, divider: u32) {
        self.hp
            .root_clk_ctrl1
            .write_reg_sys_clk_div_num(divider - 1);
        self.bus_update();
    }

    fn set_apb_divider(&self, divider: u32) {
        self.hp
            .root_clk_ctrl2
            .write_reg_apb_clk_div_num(divider - 1);
        self.bus_update();
    }
}

impl Deref for ClockTree {
    type Target = hp_sys_clkrst::RegisterBlock;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.hp
    }
}

/// Sets or clears one of the PLL power ties in `PMU_IMM_HP_CK_POWER`.
///
/// The register is shared with the drivers that power the other PLLs, so this
/// is a read-modify-write. Only the named bit carries a 1, which is what makes
/// the write act on it.
fn set_pmu_tie(bit: u32, high: bool) {
    let mask = 1 << bit;
    // SAFETY: `PMU_IMM_HP_CK_POWER` only holds these ties; no driver owns the
    // whole block, and the write is a single read-modify-write.
    let pmu = unsafe { PMU::steal() };
    let bits = if high {
        pmu.imm_hp_ck_power.read_bits() | mask
    } else {
        pmu.imm_hp_ck_power.read_bits() & !mask
    };
    pmu.imm_hp_ck_power.write_bits(bits);
}

/// The divider, feedback and current registers of a PLL.
fn pll_registers(pll: Pll) -> (regi2c::Register, regi2c::Register, regi2c::Register) {
    match pll {
        Pll::Cpll => (cpll::OC_REF_DIV, cpll::OC_DIV_7_0, cpll::OC_DCUR),
        Pll::Spll => (spll::OC_REF_DIV, spll::OC_DIV_7_0, spll::OC_DCUR),
    }
}

/// The feedback divider register of a PLL.
fn pll_divider(pll: Pll) -> regi2c::Register {
    match pll {
        Pll::Cpll => cpll::OC_DIV_7_0,
        Pll::Spll => spll::OC_DIV_7_0,
    }
}
