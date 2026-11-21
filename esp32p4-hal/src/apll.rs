//! Audio PLL, the clock source for exact audio rates.
//!
//! The APLL is the only clock on the chip that can be tuned to an arbitrary
//! audio frequency — 12.288 MHz for 48 kHz, 11.2896 MHz for 44.1 kHz — which is
//! what a codec clocked from MCLK wants. It has no memory-mapped registers: the
//! coefficients and the calibration state machine live on the analog I2C bus
//! ([`crate::regi2c`]), and the power gates sit in `PMU` and `LP_AON_CLKRST`.
//!
//! # Features
//!
//! - [`Apll::enable`]: coefficient search for a target frequency, power-up,
//!   calibration, and back the frequency the hardware really produces
//! - [`Apll::disable`]: power down in the order the gates have to close
//! - [`Apll::frequency_from`] / [`Apll::coefficients_for`]: the arithmetic on its
//!   own, for checking a configuration without starting the PLL
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::apll::Apll;
//! use esp32p4_hal::i2s::{ClockSource, I2s0};
//!
//! let apll = Apll::enable(12_288_000, &dp.LP_AON_CLKRST, &dp.PMU)?;
//! i2s.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Apll, apll.frequency_hz());
//! ```
//!
//! # Hardware Notes
//!
//! - The output is `xtal * (4 + sdm2 + sdm1/256 + sdm0/65536) / ((o_div + 2) * 2)`.
//!   The multiplier in the numerator has to stay between 350 and 500 MHz, which
//!   is what limits the usable range to roughly 5.3–125 MHz.
//! - The analog block has a mask register per byte, so every write is a
//!   read-modify-write of the byte the field lives in ([`crate::regi2c::Field`]).
//! - Calibration is a sequence of three writes to the calibration byte and then
//!   a poll of `CAL_END`; the writes are whole bytes, not fields, because the
//!   delay, the start bit and the release bit have to change in that order.
//! - `PMU_IMM_HP_CK_POWER` groups the PLL ties into four-bit fields named after
//!   their first bit, and the APLL bit of each pair sits two above the PLL bit —
//!   the offsets below are derived from the generated ones for that reason.

use esp32p4_pac::pmu::ImmHpCkPower;
use esp32p4_pac::{LP_AON_CLKRST, PMU};

use crate::regi2c::{self, apll as regs};

/// Crystal the PLL multiplies, in Hz.
const XTAL_HZ: u32 = crate::clock::XTAL_HZ;
/// The multiplier in the numerator has to stay in this window.
const MULTIPLIER_MIN_HZ: u64 = 350_000_000;
const MULTIPLIER_MAX_HZ: u64 = 500_000_000;
/// Two documented whole-byte writes that release the sigma-delta modulator.
const SDM_STOP_VALUE_1: u8 = 0x09;
const SDM_STOP_VALUE_2: u8 = 0x49;
/// Calibration delays, written in this order to start and release the state
/// machine.
const CAL_DELAY_VALUES: [u8; 3] = [0x0f, 0x3f, 0x1f];

// `PMU_IMM_HP_CK_POWER` carries the PLL and APLL ties in four-bit groups; the
// APLL bit of each group is the PLL bit plus two (TRM, PMU_IMM_HP_CK_POWER).
const TIE_HIGH_XPD_APLL: u32 = ImmHpCkPower::TIE_HIGH_XPD_PLL_OFFSET + 2;
const TIE_HIGH_XPD_APLL_I2C: u32 = ImmHpCkPower::TIE_HIGH_XPD_PLL_I2C_OFFSET + 2;
const TIE_HIGH_GLOBAL_APLL_ICG: u32 = ImmHpCkPower::TIE_HIGH_GLOBAL_PLL_ICG_OFFSET + 2;
const TIE_LOW_XPD_APLL: u32 = ImmHpCkPower::TIE_LOW_XPD_PLL_OFFSET + 2;
const TIE_LOW_XPD_APLL_I2C: u32 = ImmHpCkPower::TIE_LOW_XPD_PLL_I2C_OFFSET + 2;
const TIE_LOW_GLOBAL_APLL_ICG: u32 = ImmHpCkPower::TIE_LOW_GLOBAL_PLL_ICG_OFFSET + 2;

/// The divider chain that produces a frequency.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Coefficients {
    /// Output divider, 0 to 31.
    pub o_div: u8,
    /// Sigma-delta fraction, most significant byte.
    pub sdm2: u8,
    /// Sigma-delta fraction, middle byte.
    pub sdm1: u8,
    /// Sigma-delta fraction, least significant byte.
    pub sdm0: u8,
}

impl Coefficients {
    /// The frequency these coefficients produce.
    pub fn frequency_hz(self) -> u32 {
        let fraction =
            u64::from(self.sdm2) * 65536 + u64::from(self.sdm1) * 256 + u64::from(self.sdm0);
        let numerator = u64::from(XTAL_HZ) * (4 * 65536 + fraction);
        let denominator = u64::from(self.o_div + 2) * 2 * 65536;
        (numerator / denominator) as u32
    }
}

/// The audio PLL, running at a known frequency.
///
/// Dropping it does not power anything down: [`disable`](Self::disable) has to
/// be called, so a driver that hands the frequency to a peripheral keeps the PLL
/// alive while that peripheral uses it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Apll {
    frequency_hz: u32,
    coefficients: Coefficients,
}

impl Apll {
    /// Finds the coefficients for `target_hz` without touching the hardware.
    ///
    /// Fails when the target is outside what the 350–500 MHz multiplier window
    /// allows, i.e. below about 5.3 MHz or above 125 MHz.
    pub fn coefficients_for(target_hz: u32) -> Result<Coefficients, Error> {
        if target_hz == 0 {
            return Err(Error::Unsupported);
        }
        let target = u64::from(target_hz);
        // Pick the output divider that puts the multiplier inside its window.
        let mut o_div = (MULTIPLIER_MIN_HZ / (target * 2)) as i64 - 1;
        if o_div > 31 {
            return Err(Error::TooLow);
        }
        if o_div < 0 {
            o_div = (MULTIPLIER_MAX_HZ / (target * 2)) as i64 - 2;
            if o_div < 0 {
                return Err(Error::TooHigh);
            }
        }
        let o_div = o_div as u32;
        // multiplier = (o_div + 2) * 2 * target / xtal, in 4.16 fixed point.
        let multiplier = (u64::from(o_div + 2) * 2 * target * 65536) / u64::from(XTAL_HZ);
        let integer = multiplier / 65536;
        if integer < 4 {
            return Err(Error::Unsupported);
        }
        let mut sdm2 = integer - 4;
        let mut fraction = multiplier - integer * 65536;
        // A fraction that rounds up to a whole unit carries into the integer
        // part instead of overflowing the three bytes.
        if fraction > 65536 - 32 {
            sdm2 += 1;
            fraction = 0;
        }
        if sdm2 > 63 {
            return Err(Error::Unsupported);
        }
        Ok(Coefficients {
            o_div: o_div as u8,
            sdm2: sdm2 as u8,
            sdm1: (fraction / 256) as u8,
            sdm0: (fraction % 256) as u8,
        })
    }

    /// The frequency a set of coefficients produces.
    pub fn frequency_from(coefficients: Coefficients) -> u32 {
        coefficients.frequency_hz()
    }

    /// Powers the PLL up, programs `target_hz` and calibrates it.
    ///
    /// Returns the frequency the hardware ends up at, which is the closest one
    /// the crystal and the fractional divider can reach.
    pub fn enable(target_hz: u32, lpaon: &LP_AON_CLKRST, pmu: &PMU) -> Result<Self, Error> {
        let coefficients = Self::coefficients_for(target_hz)?;

        // The gate has to open and the bias blocks wake up before the analog
        // register file answers.
        lpaon
            .lp_aonclkrst_hp_clk_ctrl
            .set_lp_aonclkrst_hp_audio_pll_clk_en();
        set_pmu_bit(pmu, TIE_HIGH_XPD_APLL, true);
        set_pmu_bit(pmu, TIE_HIGH_XPD_APLL_I2C, true);
        set_pmu_bit(pmu, TIE_HIGH_GLOBAL_APLL_ICG, true);

        regi2c::enable_clock();
        Self::program(coefficients);
        Self::calibrate();

        Ok(Self {
            frequency_hz: coefficients.frequency_hz(),
            coefficients,
        })
    }

    /// The frequency the PLL runs at.
    pub const fn frequency_hz(self) -> u32 {
        self.frequency_hz
    }

    /// The coefficients it runs with.
    pub const fn coefficients(self) -> Coefficients {
        self.coefficients
    }

    /// Retunes a running PLL.
    ///
    /// The calibration has to run again, so the output glitches; a peripheral
    /// consuming the PLL should be stopped first.
    pub fn set_frequency(&mut self, target_hz: u32) -> Result<u32, Error> {
        let coefficients = Self::coefficients_for(target_hz)?;
        Self::program(coefficients);
        Self::calibrate();
        self.coefficients = coefficients;
        self.frequency_hz = coefficients.frequency_hz();
        Ok(self.frequency_hz)
    }

    /// Powers the PLL down: the gate closes first, then the bias blocks.
    pub fn disable(self, lpaon: &LP_AON_CLKRST, pmu: &PMU) {
        lpaon
            .lp_aonclkrst_hp_clk_ctrl
            .clr_lp_aonclkrst_hp_audio_pll_clk_en();
        set_pmu_bit(pmu, TIE_LOW_GLOBAL_APLL_ICG, true);
        set_pmu_bit(pmu, TIE_LOW_XPD_APLL, true);
        set_pmu_bit(pmu, TIE_LOW_XPD_APLL_I2C, true);
    }

    fn program(coefficients: Coefficients) {
        regs::DSDM2.write(u32::from(coefficients.sdm2) as u8);
        regs::DSDM0.write(coefficients.sdm0);
        regs::DSDM1.write(coefficients.sdm1);
        // Two whole-byte writes that stop and then release the sigma-delta
        // modulator; the values are the ones the reference driver uses.
        regs::SDM.write(SDM_STOP_VALUE_1);
        regs::SDM.write(SDM_STOP_VALUE_2);
        regs::OUTPUT_DIV.write(coefficients.o_div);
    }

    fn calibrate() {
        // Delay, start, release — three whole-byte writes that move the state
        // machine, so they cannot be spelled as field updates.
        for value in CAL_DELAY_VALUES {
            regs::CAL.write(value);
        }
        crate::spin::spin_wait("apll calibration", || regs::CAL_END.read() != 0);
    }
}

fn set_pmu_bit(pmu: &PMU, bit: u32, high: bool) {
    let mask = 1u32 << bit;
    if high {
        pmu.imm_hp_ck_power.modify_bits(|bits| bits | mask);
    } else {
        pmu.imm_hp_ck_power.modify_bits(|bits| bits & !mask);
    }
}

/// Why a frequency cannot be produced.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The target is below the 5.3 MHz floor of the 350 MHz multiplier window.
    TooLow,
    /// The target is above the 125 MHz ceiling of the 500 MHz multiplier window.
    TooHigh,
    /// The target fits the window but not the divider encoding.
    Unsupported,
}
