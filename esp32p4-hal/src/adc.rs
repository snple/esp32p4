//! ADC (12-bit SAR analog-to-digital converter).
//!
//! The ESP32-P4 has two 12-bit successive-approximation ADCs behind a shared
//! analogue front end:
//!
//! | Unit | Channels | Pads |
//! |------|----------|------|
//! | ADC1 | 8        | GPIO16-GPIO23 |
//! | ADC2 | 6        | GPIO49-GPIO54 |
//!
//! Unlike UART or I2C, an ADC channel is **not** a GPIO-matrix signal: the pad
//! is hard-wired to the SAR input, and the only choice left is which pad to
//! sample. [`AdcInput::new`] reads that mapping from the pad table
//! ([`crate::gpio::pins`]) and leaves the pad floating, which is what an
//! analogue input needs: no input buffer, no pull resistors, no output driver.
//!
//! The hardware is split over two register blocks: the analogue half lives in
//! `LP_ADC` (channel mux, per-channel attenuation, power, result) and the
//! digital half in `ADC` (pattern table, DMA, interrupts). This driver uses the
//! *RTC* (one-shot) controller: software selects one channel, starts one
//! conversion and polls for the result. The digital controller — which scans a
//! pattern table and streams into memory through DMA — is not implemented yet,
//! nor is the low-power ADC reached from the LP core.
//!
//! # Features
//!
//! - One-shot conversions on all 14 ADC pads, blocking or polled
//!   ([`Adc::start`], [`Adc::is_done`], [`Adc::read_result`])
//! - Per-channel attenuation: 0 dB, 2.5 dB, 6 dB or 12 dB
//! - Factory calibration from eFuse: the SAR offset code is applied in
//!   hardware before every conversion, and [`Adc::read_millivolts`] uses the
//!   eFuse curve-fitting data (reference point + error polynomial + per-channel
//!   compensation)
//! - A documented nominal conversion when the eFuse calibration block is
//!   empty
//!
//! # Pin Mapping
//!
//! | Pad | Function | Unit | Channel |
//! |-----|----------|------|---------|
//! | GPIO16-GPIO23 | [`AnalogFunction::Adc1`] | ADC1 | 0-7 |
//! | GPIO49-GPIO54 | [`AnalogFunction::Adc2`] | ADC2 | 0-5 |
//!
//! GPIO54 has no `IO_MUX` register in this chip's register map, so it cannot be
//! driven through [`crate::gpio::Pins`] yet; use GPIO49-GPIO53 for ADC2.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::adc::{Adc, AdcInput, Attenuation};
//! use esp32p4_hal::gpio::Pins;
//!
//! let dp = Peripherals::take().unwrap();
//! let mut pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//! let adc = Adc::init(dp.ADC, dp.LP_ADC, &dp.HP_SYS_CLKRST, &dp.LP_PERI);
//!
//! let mut sense = pins.pin::<16>();
//! let input = AdcInput::new(&mut sense, Attenuation::_12dB);
//!
//! let raw = adc.read_raw(&input);          // 0..=4095
//! let millivolts = adc.read_millivolts(&input);
//! ```
//!
//! # Accuracy
//!
//! [`Adc::read_millivolts`] is only as good as the calibration data burnt into
//! the chip. [`Adc::calibration`] reports whether that data is present; without
//! it the conversion falls back to the datasheet full-scale voltage, which is
//! typically several tens of millivolts off, and the useful range of each
//! attenuation only covers part of the full scale.

use esp32p4_pac::{ADC, HP_SYS_CLKRST, LP_ADC, LP_PERI};

use crate::efuse::Efuse;
use crate::gpio::PinOps;
use crate::gpio::pins::{AnalogFunction, MuxFunction};
use crate::regi2c;
use crate::spin;

/// Full-scale raw code: the SAR always produces 12 bits.
pub const MAX_CODE: u32 = 0x0fff;

/// `LPADC_SARn_CLK_DIV`: `SARCLK = LP ADC clock / (divider + 1)`.
///
/// ESP-IDF's default for one-shot conversions on this chip.
const SAR_CLK_DIV: u32 = 1;

/// `LPADC_FORCE_XPD_SARn`: software control, forced on.
const SAR_POWER_ON: u32 = 3;

/// `ADC_XPD_SARn_FORCE` as ESP-IDF writes it. The TRM documents only 0 (no
/// effect) and 1 (force on) for this copy of the field; the driver follows the
/// silicon-tested value.
const SAR_POWER_ON_HP: u32 = 3;

/// `ADC_SARn_DREF`: the reference the offset code is expressed in.
const SAR_DREF: u8 = 4;

/// `LPADC_SAR2_PWDET_CCT`: the power detector's current setting, ESP-IDF's
/// default for this chip.
const PWDET_CCT: u32 = 4;

// ============================================================================
// Register addresses (analogue I2C block 0x69)
// ============================================================================

const SAR1_INITIAL_CODE_LOW: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x0);
const SAR1_INITIAL_CODE_HIGH: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x1);
const SAR1_DREF: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x2);
const SAR2_INITIAL_CODE_LOW: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x3);
const SAR2_INITIAL_CODE_HIGH: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x4);
const SAR2_DREF: regi2c::Register = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x5);

// ============================================================================
// Types
// ============================================================================

/// Which SAR ADC a channel belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unit {
    /// ADC1, eight channels on GPIO16-GPIO23.
    Adc1,
    /// ADC2, six channels on GPIO49-GPIO54.
    Adc2,
}

impl Unit {
    /// Index into the per-unit tables (`0` for ADC1, `1` for ADC2).
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Number of channels on this unit.
    #[inline]
    pub const fn channels(self) -> u8 {
        match self {
            Self::Adc1 => 8,
            Self::Adc2 => 6,
        }
    }

    /// The `LPADC_SARn_ATTEN` shift of `channel`.
    ///
    /// ADC1 uses two bits per channel from bit 0; ADC2's channels are the
    /// internal channels 2-7, so its first channel starts at bit 4.
    #[inline]
    const fn atten_shift(self, channel: u8) -> u32 {
        match self {
            Self::Adc1 => channel as u32 * 2,
            Self::Adc2 => (channel as u32 + 2) * 2,
        }
    }

    /// The `LPADC_SARn_EN_PAD` bit mask of `channel`.
    ///
    /// ADC2's channels are the internal channels 2-7 (see
    /// [`Unit::atten_shift`]).
    #[inline]
    const fn enable_mask(self, channel: u8) -> u32 {
        match self {
            Self::Adc1 => 1 << channel,
            Self::Adc2 => 1 << (channel + 2),
        }
    }
}

/// Input attenuation, i.e. how much of the input range reaches the SAR.
///
/// The full-scale voltage grows with the attenuation, and so does the noise:
/// pick the smallest attenuation that still covers the signal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Attenuation {
    /// 0 dB: full scale ≈ 1.1 V, best for small signals.
    _0dB = 0,
    /// 2.5 dB: full scale ≈ 1.5 V.
    _2_5dB = 1,
    /// 6 dB: full scale ≈ 2.2 V.
    _6dB = 2,
    /// 12 dB: full scale ≈ 3.9 V, clamped by the supply in practice.
    _12dB = 3,
}

impl Attenuation {
    /// The datasheet full-scale voltage, used when the chip has no factory
    /// calibration data.
    #[inline]
    pub const fn full_scale_mv(self) -> u32 {
        match self {
            Self::_0dB => 1100,
            Self::_2_5dB => 1500,
            Self::_6dB => 2200,
            Self::_12dB => 3900,
        }
    }

    /// All four attenuations, weakest first.
    pub const ALL: [Self; 4] = [Self::_0dB, Self::_2_5dB, Self::_6dB, Self::_12dB];
}

/// An analogue input: one ADC pad, one channel, one attenuation.
///
/// Created with [`AdcInput::new`], which looks the pad up in the IO MUX table
/// and leaves it floating. The token holds no registers, only the three numbers
/// [`Adc`] needs to start a conversion.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcInput {
    unit: Unit,
    channel: u8,
    attenuation: Attenuation,
}

impl AdcInput {
    /// Configure `pin` as an analogue input of whichever ADC it belongs to.
    ///
    /// The pad is switched to the GPIO function and left floating: input
    /// buffer off, both pulls off, output driver off. That is what the SAR
    /// needs to see the pin voltage rather than a driver's level.
    ///
    /// # Panics
    ///
    /// Panics if the pad has no ADC channel; see [`crate::gpio::pins`].
    pub fn new(pin: &mut impl PinOps, attenuation: Attenuation) -> Self {
        let pad = pin.pad();
        let function = pad
            .analog()
            .iter()
            .flatten()
            .find(|function| matches!(function, AnalogFunction::Adc1(_) | AnalogFunction::Adc2(_)))
            .unwrap_or_else(|| panic!("GPIO{} has no ADC channel", pad.number));
        let (unit, channel) = match *function {
            AnalogFunction::Adc1(channel) => (Unit::Adc1, channel),
            AnalogFunction::Adc2(channel) => (Unit::Adc2, channel),
            _ => unreachable!(),
        };

        pin.set_mux_function(MuxFunction::Gpio);
        pin.set_input_buffer_enabled(false);
        pin.set_pull_up_enabled(false);
        pin.set_pull_down_enabled(false);
        pin.set_output_enabled(false);

        Self {
            unit,
            channel,
            attenuation,
        }
    }

    /// The unit this input belongs to.
    #[inline]
    pub const fn unit(&self) -> Unit {
        self.unit
    }

    /// The channel number within [`AdcInput::unit`].
    #[inline]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// The configured attenuation.
    #[inline]
    pub const fn attenuation(&self) -> Attenuation {
        self.attenuation
    }

    /// Change the attenuation; the next conversion uses it.
    #[inline]
    pub fn set_attenuation(&mut self, attenuation: Attenuation) {
        self.attenuation = attenuation;
    }
}

/// Factory calibration, read from eFuse by [`Adc::init`].
///
/// Two things come out of the eFuse block: per-unit/per-attenuation
/// **offset** data, which is handed to the SAR so its comparator starts from
/// the right code, and a **reference point** (`digi` output at a known input
/// voltage) plus an error polynomial, which turn a raw code into millivolts.
///
/// All of it is optional: chips whose calibration block was never programmed
/// report [`Calibration::is_available`] `false` and convert with the nominal
/// full-scale voltage instead.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Calibration {
    /// Offset code for the SAR, per unit and attenuation.
    init_code: [[Option<u16>; 4]; 2],
    /// `reference_mv * 65536 / reference_code`, i.e. millivolts per raw count
    /// in 16.16 fixed point, per unit and attenuation.
    slope: [[Option<u32>; 4]; 2],
    /// Per-channel offset in raw counts, per unit and channel.
    channel_offset: [[i8; 8]; 2],
}

impl Calibration {
    /// Whether the eFuse calibration block is programmed.
    #[inline]
    pub const fn is_available(&self) -> bool {
        self.slope[Unit::Adc1.index()][0].is_some() || self.slope[Unit::Adc2.index()][0].is_some()
    }

    /// The error polynomial of a unit and attenuation.
    ///
    /// Two terms, `error = -(c0 / d0) + (c1 / d1) * v` with `v` in millivolts.
    /// Both terms are positive and the first sign is negative, which is why the
    /// subtraction is inlined here rather than stored per term.
    const fn error_coefficients(unit: Unit, attenuation: Attenuation) -> [(u64, u64); 2] {
        // ESP-IDF `esp_adc/esp32p4/curve_fitting_coefficients.c`, version 1.
        match (unit, attenuation) {
            (Unit::Adc1, Attenuation::_0dB) => [
                (7_170_501_832_480_995, 10_000_000_000_000_000),
                (10_598_497_992_115, 10_000_000_000_000_000),
            ],
            (Unit::Adc1, Attenuation::_2_5dB) => [
                (9_960_085_535_084_866, 10_000_000_000_000_000),
                (15_840_076_608_145, 10_000_000_000_000_000),
            ],
            (Unit::Adc1, Attenuation::_6dB) => [
                (14_711_053_224_678_996, 10_000_000_000_000_000),
                (1_594_266_424_857, 100_000_000_000_000_000),
            ],
            (Unit::Adc1, Attenuation::_12dB) => [
                (28_811_493_455_181_565, 10_000_000_000_000_000),
                (10_082_311_568_625, 10_000_000_000_000_000),
            ],
            (Unit::Adc2, Attenuation::_0dB) => [
                (4_900_967_548_489_932, 10_000_000_000_000_000),
                (5_037_402_667_913, 10_000_000_000_000_000),
            ],
            (Unit::Adc2, Attenuation::_2_5dB) => [
                (7_296_214_814_536_025, 10_000_000_000_000_000),
                (11_021_577_596_635, 10_000_000_000_000_000),
            ],
            (Unit::Adc2, Attenuation::_6dB) => [
                (10_991_620_450_220_592, 10_000_000_000_000_000),
                (11_623_930_881_896, 10_000_000_000_000_000),
            ],
            (Unit::Adc2, Attenuation::_12dB) => [
                (24_421_401_024_626_730, 10_000_000_000_000_000),
                (9_458_501_263_393, 10_000_000_000_000_000),
            ],
        }
    }

    /// The correction the linear estimate needs, in millivolts.
    fn error_mv(unit: Unit, attenuation: Attenuation, millivolts: u64) -> i32 {
        let [(c0, d0), (c1, d1)] = Self::error_coefficients(unit, attenuation);
        let constant = c0 / d0;
        let linear = millivolts * c1 / d1;
        linear as i32 - constant as i32
    }

    /// Convert a raw code to millivolts.
    ///
    /// Uses the eFuse reference point when the chip has one, and the datasheet
    /// full-scale voltage otherwise. `raw` is clamped to [`MAX_CODE`] so a
    /// caller-supplied value cannot
    /// produce a nonsensical voltage.
    pub fn to_millivolts(
        &self,
        unit: Unit,
        channel: u8,
        attenuation: Attenuation,
        raw: u32,
    ) -> u32 {
        let Some(slope) = self.slope[unit.index()][attenuation as usize] else {
            return (raw.min(MAX_CODE) * attenuation.full_scale_mv() + MAX_CODE / 2) / MAX_CODE;
        };

        // The per-channel offset is expressed at 0 dB and grows with the
        // attenuation, as ESP-IDF does it.
        let offset =
            self.channel_offset[unit.index()][channel as usize] as i32 * (4 - attenuation as i32);
        let raw = (raw as i32 - offset).clamp(0, MAX_CODE as i32) as u64;

        let estimate = (raw * slope as u64) / 65536;
        let error = Self::error_mv(unit, attenuation, estimate);
        (estimate as i32 - error).max(0) as u32
    }

    /// Read the eFuse calibration block.
    ///
    /// Returns an empty calibration when the block version says the layout is
    /// unknown, which is how an unprogrammed chip reports itself.
    fn load() -> Self {
        let mut calibration = Self::default();

        let efuse = unsafe { Efuse::steal() };
        // BLOCK2 version 1 is the first layout that carries ADC calibration.
        if efuse.block_version().revision() < 1 {
            return calibration;
        }

        let regs = efuse.regs();
        let part1_4 = regs.rd_sys_part1_data4.read_bits();
        let part1_5 = regs.rd_sys_part1_data5.read_bits();
        let part1_6 = regs.rd_sys_part1_data6.read_bits();
        let part1_7 = regs.rd_sys_part1_data7.read_bits();
        let part2_0 = regs.rd_sys_part2_data0.read_bits();
        let part2_1 = regs.rd_sys_part2_data1.read_bits();
        let part2_2 = regs.rd_sys_part2_data2.read_bits();

        // Initial (offset) code per unit and attenuation, `EFUSE_ADCn_AVE_INITCODE_ATTENx`.
        // ESP-IDF adds 1400 to land in the SAR's 12-bit code space.
        let init_code = [
            [
                Some((bits(part1_4, 10, 0) + 1400) as u16),
                Some((bits(part1_4, 20, 10) + 1400) as u16),
                Some((bits(part1_4, 30, 20) + 1400) as u16),
                Some(((bits(part1_4, 32, 30) | bits(part1_5, 8, 0) << 2) + 1400) as u16),
            ],
            [
                Some((bits(part1_5, 18, 8) + 1400) as u16),
                Some((bits(part1_5, 28, 18) + 1400) as u16),
                Some(((bits(part1_5, 32, 28) | bits(part1_6, 6, 0) << 4) + 1400) as u16),
                Some((bits(part1_6, 16, 6) + 1400) as u16),
            ],
        ];

        // Reference point: the raw code the SAR produced at a known input
        // voltage (`EFUSE_ADCn_HI_DOUT_ATTENx`), as a signed offset from the
        // nominal code.
        const REFERENCE_MV: [u32; 4] = [600, 800, 1200, 2300];
        const NOMINAL_CODE: [u32; 4] = [2300, 2300, 2350, 2350];
        let digi = [
            [
                bits(part1_6, 26, 16),
                bits(part1_6, 32, 26) | bits(part1_7, 4, 0) << 6,
                bits(part1_7, 14, 4),
                bits(part1_7, 24, 14),
            ],
            [
                bits(part2_0, 10, 0),
                bits(part2_0, 20, 10),
                bits(part2_0, 30, 20),
                bits(part2_0, 32, 30) | bits(part2_1, 8, 0) << 2,
            ],
        ];

        for unit in [Unit::Adc1, Unit::Adc2] {
            for index in 0..Attenuation::ALL.len() {
                let code = NOMINAL_CODE[index] as i32 + signed(digi[unit.index()][index], 9);
                if code <= 0 {
                    continue;
                }
                let slope = (REFERENCE_MV[index] << 16) / code as u32;
                calibration.slope[unit.index()][index] = Some(slope);
                calibration.init_code[unit.index()][index] = init_code[unit.index()][index];
            }
        }

        // Per-channel offset compensation, `EFUSE_ADCn_CHAN_COMPENSx`: four
        // bits per channel, three magnitude bits plus a sign bit.
        for channel in 0..Unit::Adc1.channels() {
            let raw = if channel < 6 {
                bits(part2_1, 8 + channel as u32 * 4 + 4, 8 + channel as u32 * 4)
            } else {
                bits(
                    part2_2,
                    (channel as u32 - 6) * 4 + 4,
                    (channel as u32 - 6) * 4,
                )
            };
            calibration.channel_offset[Unit::Adc1.index()][channel as usize] = signed(raw, 3) as i8;
        }
        for channel in 0..Unit::Adc2.channels() {
            let shift = 8 + channel as u32 * 4;
            let raw = bits(part2_2, shift + 4, shift);
            calibration.channel_offset[Unit::Adc2.index()][channel as usize] = signed(raw, 3) as i8;
        }

        calibration
    }
}

/// Bits `[msb:lsb]` of `value`, as an unsigned number (`msb` is exclusive).
#[inline]
const fn bits(value: u32, msb: u32, lsb: u32) -> u32 {
    let shifted = value >> lsb;
    if msb >= 32 {
        shifted
    } else {
        shifted & ((1 << (msb - lsb)) - 1)
    }
}

/// A sign-magnitude eFuse value: bit `sign_bit` is the sign, the bits below it
/// the magnitude.
#[inline]
const fn signed(value: u32, sign_bit: u32) -> i32 {
    let magnitude = (value & ((1 << sign_bit) - 1)) as i32;
    if value & (1 << sign_bit) != 0 {
        -magnitude
    } else {
        magnitude
    }
}

/// The two SAR ADCs and the digital controller they share.
///
/// Both ADCs live behind one `Adc`: they share the analogue front end and the
/// calibration data, and a single owner keeps that consistent.
pub struct Adc {
    adc: ADC,
    lp_adc: LP_ADC,
    calibration: Calibration,
}

impl Adc {
    /// Bring both SAR ADCs up and read their factory calibration.
    ///
    /// This opens the three clock gates (the `ADC` APB clock, the HP SAR clock
    /// and `LP_PERI.CK_EN_LP_ADC`), pulses both resets, powers the analogue I2C
    /// bus that holds the SAR trim registers, and loads the eFuse calibration
    /// data. The SARs stay powered down until the first conversion.
    pub fn init(adc: ADC, lp_adc: LP_ADC, clkrst: &HP_SYS_CLKRST, lp_peri: &LP_PERI) -> Self {
        clkrst.soc_clk_ctrl2.set_reg_adc_apb_clk_en();
        clkrst.peri_clk_ctrl23.set_reg_adc_clk_en();
        lp_peri.clk_en.set_ck_en_lp_adc();

        clkrst.hp_rst_en2.set_reg_rst_en_adc();
        lp_peri.reset_en.set_rst_en_lp_adc();
        clkrst.hp_rst_en2.clr_reg_rst_en_adc();
        lp_peri.reset_en.clr_rst_en_lp_adc();

        regi2c::enable_clock();
        regi2c::sar_periph_enable();

        // The power-detect current setting ADC2 shares its SAR with. ESP-IDF
        // writes it in every one-shot setup; with the reset value of zero ADC2
        // conversions were visibly noisy.
        lp_adc.meas2_mux.write_sar2_pwdet_cct(PWDET_CCT);

        Self {
            adc,
            lp_adc,
            calibration: Calibration::load(),
        }
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - [`Adc::init`] has run, so the clocks, resets and analogue bus are up;
    /// - no other handle to the same peripherals is used concurrently.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            adc: unsafe { ADC::steal() },
            lp_adc: unsafe { LP_ADC::steal() },
            calibration: Calibration::load(),
        })
    }

    /// The digital controller's register block.
    #[inline]
    pub fn regs(&self) -> &esp32p4_pac::adc::RegisterBlock {
        &self.adc
    }

    /// The analogue half's register block.
    #[inline]
    pub fn lp_regs(&self) -> &esp32p4_pac::lp_adc::RegisterBlock {
        &self.lp_adc
    }

    /// Give the PAC peripherals back.
    #[inline]
    pub fn release(self) -> (ADC, LP_ADC) {
        (self.adc, self.lp_adc)
    }

    /// The factory calibration loaded at init.
    #[inline]
    pub fn calibration(&self) -> &Calibration {
        &self.calibration
    }

    /// Configure the channel and start one conversion.
    ///
    /// Returns immediately: poll [`Adc::is_done`] and read the result with
    /// [`Adc::read_result`], or let [`Adc::read_raw`] do both.
    pub fn start(&self, input: &AdcInput) {
        self.power_up(input.unit);
        self.set_attenuation(input.unit, input.channel, input.attenuation);
        self.apply_offset_code(input.unit, input.attenuation);

        match input.unit {
            Unit::Adc1 => {
                let ctrl2 = &self.lp_adc.meas1_ctrl2;
                // `SARn_DIG_FORCE = 0`: the LP (RTC) controller owns the SAR.
                self.lp_adc.meas1_mux.clr_sar1_dig_force();
                ctrl2.set_meas1_start_force();
                ctrl2.set_sar1_en_pad_force();
                self.lp_adc.reader1_ctrl.write_sar1_clk_div(SAR_CLK_DIV);
                ctrl2.write_sar1_en_pad(input.unit.enable_mask(input.channel));
                ctrl2.clr_meas1_start_sar();
                ctrl2.set_meas1_start_sar();
            }
            Unit::Adc2 => {
                let ctrl2 = &self.lp_adc.meas2_ctrl2;
                // ADC2 selects the LP controller with `SAR2_RTC_FORCE = 1`.
                self.lp_adc.meas2_mux.set_sar2_rtc_force();
                ctrl2.set_meas2_start_force();
                ctrl2.set_sar2_en_pad_force();
                self.lp_adc.reader2_ctrl.write_sar2_clk_div(SAR_CLK_DIV);
                ctrl2.write_sar2_en_pad(input.unit.enable_mask(input.channel));
                ctrl2.clr_meas2_start_sar();
                ctrl2.set_meas2_start_sar();
            }
        }
    }

    /// Whether the conversion started by [`Adc::start`] has finished.
    #[inline]
    pub fn is_done(&self, unit: Unit) -> bool {
        match unit {
            Unit::Adc1 => self.lp_adc.meas1_ctrl2.meas1_done_sar(),
            Unit::Adc2 => self.lp_adc.meas2_ctrl2.meas2_done_sar(),
        }
    }

    /// The 12-bit result of the last finished conversion.
    #[inline]
    pub fn read_result(&self, unit: Unit) -> u16 {
        let data = match unit {
            Unit::Adc1 => self.lp_adc.meas1_ctrl2.meas1_data_sar(),
            Unit::Adc2 => self.lp_adc.meas2_ctrl2.meas2_data_sar(),
        };
        (data as u16) & MAX_CODE as u16
    }

    /// Read one raw sample, blocking until the conversion finishes.
    ///
    /// # Panics
    ///
    /// Panics if the conversion does not finish within
    /// [`SPIN_TIMEOUT`](crate::spin::SPIN_TIMEOUT) iterations.
    pub fn read_raw(&self, input: &AdcInput) -> u16 {
        self.start(input);
        spin::spin_wait("ADC conversion", || self.is_done(input.unit));
        self.read_result(input.unit)
    }

    /// Read one calibrated sample in millivolts.
    ///
    /// See [`Calibration::to_millivolts`] for what "calibrated" means on a chip
    /// without factory data.
    pub fn read_millivolts(&self, input: &AdcInput) -> u32 {
        let raw = self.read_raw(input);
        self.calibration
            .to_millivolts(input.unit, input.channel, input.attenuation, raw as u32)
    }

    /// Power the SAR of `unit` on and keep it on.
    ///
    /// The one-shot controller cannot start a conversion from a powered-down
    /// SAR, and the SAR stays on afterwards; there is no power-down path in the
    /// driver yet.
    fn power_up(&self, unit: Unit) {
        match unit {
            Unit::Adc1 => {
                self.lp_adc.force_wpd_sar.write_force_xpd_sar1(SAR_POWER_ON);
                self.adc.ctrl.write_xpd_sar1_force(SAR_POWER_ON_HP);
            }
            Unit::Adc2 => {
                self.lp_adc.force_wpd_sar.write_force_xpd_sar2(SAR_POWER_ON);
                self.adc.ctrl.write_xpd_sar2_force(SAR_POWER_ON_HP);
            }
        }
    }

    /// Write the per-channel attenuation field.
    fn set_attenuation(&self, unit: Unit, channel: u8, attenuation: Attenuation) {
        let shift = unit.atten_shift(channel);
        match unit {
            Unit::Adc1 => {
                let atten = &self.lp_adc.atten1;
                atten.write_sar1_atten(
                    (atten.sar1_atten() & !(0b11 << shift)) | ((attenuation as u32) << shift),
                );
            }
            Unit::Adc2 => {
                let atten = &self.lp_adc.atten2;
                atten.write_sar2_atten(
                    (atten.sar2_atten() & !(0b11 << shift)) | ((attenuation as u32) << shift),
                );
            }
        }
    }

    /// Hand the SAR the offset code for `unit` and `attenuation`.
    ///
    /// Both the reference selection and the code go through the analogue I2C
    /// bus, which [`Adc::init`] powered on. Chips without factory data keep the
    /// reset values.
    fn apply_offset_code(&self, unit: Unit, attenuation: Attenuation) {
        let Some(code) = self.calibration.init_code[unit.index()][attenuation as usize] else {
            return;
        };

        let (dref, low, high) = match unit {
            Unit::Adc1 => (SAR1_DREF, SAR1_INITIAL_CODE_LOW, SAR1_INITIAL_CODE_HIGH),
            Unit::Adc2 => (SAR2_DREF, SAR2_INITIAL_CODE_LOW, SAR2_INITIAL_CODE_HIGH),
        };
        dref.field(6, 4).write(SAR_DREF);
        low.write(code as u8);
        high.field(3, 0).write((code >> 8) as u8);
    }
}
