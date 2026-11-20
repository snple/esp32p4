//! Temperature sensor (`LP_TSENS`).
//!
//! The P4 has one analogue temperature sensor sitting in the low-power domain.
//! It measures the die temperature continuously while powered and puts the
//! result into a byte-wide register; the accuracy depends on which of the five
//! measurement ranges is selected, so the driver reads the value, picks the
//! range the reading falls into and converts it with that range's offset.
//!
//! The raw value is not calibrated in eFuse: the conversion is the linear
//! formula from the reference manual, `T = 0.4386 * value - 27.88 * offset -
//! 20.52`, with `offset` from the selected range.
//!
//! # Features
//!
//! - Power-up, clock enable and reset of the sensor
//! - Readings in milli-degrees Celsius, with automatic range selection
//!   ([`TemperatureSensor::update`]) or a fixed range
//! - Hardware monitoring: sample rate, absolute/delta wake-up mode, high and
//!   low thresholds and the wake-up interrupt
//!
//! # Example
//!
//! ```ignore
//! let mut sensor = TemperatureSensor::init(dp.LP_TSENS, &dp.LP_PERI);
//!
//! loop {
//!     defmt::info!("{} m°C", sensor.update());
//!     delay.delay_ms(1_000);
//! }
//! ```
//!
//! # Scope
//!
//! The sensor's ETM tasks and events are not modelled; neither is the
//! low-power-core side of the wake-up path.

use esp32p4_pac::{LP_PERI, LP_TSENS, SYSTIMER};

use crate::delay::Delay;
use crate::regi2c;

/// `I2C_SARADC_TSENS_DAC` in the analogue block: the low four bits of the
/// sensor register at address 0x6 select the measurement range.
const TSENS_DAC: regi2c::Field = regi2c::Register::new(regi2c::Block::SAR_ADC, 0x6).field(3, 0);

/// How long the output needs to approach the real die temperature after a
/// power-up or a range change.
const SETTLE_US: u32 = 100;

/// Measurement range of the sensor.
///
/// Each range trades span for accuracy: the narrower ranges step finer, which
/// is why a reading outside the current range has to be taken again in a
/// wider one. The values come from the reference manual's table 64.4-1.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    /// 50-125 °C, offset -2, ±3 °C.
    High,
    /// 20-100 °C, offset -1, ±2 °C.
    Warm,
    /// -10-80 °C, offset 0, ±1 °C.
    #[default]
    Normal,
    /// -30-50 °C, offset 1, ±2 °C.
    Cool,
    /// -40-20 °C, offset 2, ±3 °C.
    Cold,
}

impl Range {
    /// All ranges, narrowest span first.
    pub const ALL: [Self; 5] = [Self::High, Self::Warm, Self::Normal, Self::Cool, Self::Cold];

    /// Value the analogue block expects for this range.
    const fn dac(self) -> u8 {
        match self {
            Self::High => 5,
            Self::Warm => 7,
            Self::Normal => 15,
            Self::Cool => 11,
            Self::Cold => 10,
        }
    }

    /// Offset the conversion formula subtracts.
    const fn offset(self) -> i32 {
        match self {
            Self::High => -2,
            Self::Warm => -1,
            Self::Normal => 0,
            Self::Cool => 1,
            Self::Cold => 2,
        }
    }

    /// Temperatures this range covers, in whole degrees Celsius.
    pub const fn limits(self) -> (i32, i32) {
        match self {
            Self::High => (50, 125),
            Self::Warm => (20, 100),
            Self::Normal => (-10, 80),
            Self::Cool => (-30, 50),
            Self::Cold => (-40, 20),
        }
    }

    /// Range whose span covers `millidegrees`, preferring the current one when
    /// it already does.
    fn for_temperature(millidegrees: i32, current: Self) -> Self {
        let (low, high) = current.limits();
        if millidegrees >= low * 1_000 && millidegrees <= high * 1_000 {
            return current;
        }
        Self::ALL
            .into_iter()
            .find(|range| {
                let (low, high) = range.limits();
                millidegrees >= low * 1_000 && millidegrees <= high * 1_000
            })
            // Outside every span: keep the closest end, the reading is only
            // meaningful as "very hot" or "very cold" then.
            .unwrap_or(current)
    }
}

/// One-wire temperature sensor in the low-power domain.
pub struct TemperatureSensor {
    tsens: LP_TSENS,
    range: Range,
}

impl TemperatureSensor {
    /// Takes the sensor, clocks it, releases its reset and powers it up.
    ///
    /// The analogue range register lives in the SAR block, so this also brings
    /// up the analogue I2C bridge; doing so twice (next to [`crate::adc`]) is
    /// harmless.
    pub fn init(tsens: LP_TSENS, lp_peri: &LP_PERI) -> Self {
        lp_peri.clk_en.set_ck_en_lp_tsens();
        lp_peri.reset_en.set_rst_en_lp_tsens();
        lp_peri.reset_en.clr_rst_en_lp_tsens();

        regi2c::enable_clock();
        regi2c::sar_periph_enable();

        let mut sensor = Self {
            tsens,
            range: Range::default(),
        };
        sensor.set_range(Range::default());
        sensor.regs().ctrl.set_power_up();
        // `XPD_WAIT` plus the settling time the manual asks for.
        delay_us(SETTLE_US);

        sensor
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// [`init`](Self::init) must have run, and no other handle may be used at
    /// the same time.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            tsens: unsafe { LP_TSENS::steal() },
            range: Range::default(),
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &esp32p4_pac::lp_tsens::RegisterBlock {
        &self.tsens
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> LP_TSENS {
        self.tsens
    }

    /// Range the readings are currently taken in.
    #[inline]
    pub fn range(&self) -> Range {
        self.range
    }

    /// Switches the measurement range.
    ///
    /// Only the accuracy changes; the sensor keeps running. Give it about
    /// 100 us before trusting the next reading, which
    /// [`update`](Self::update) does.
    pub fn set_range(&mut self, range: Range) {
        self.range = range;
        TSENS_DAC.write(range.dac());
    }

    /// Raw reading, valid for the current range.
    #[inline]
    pub fn raw(&self) -> u8 {
        self.regs().ctrl.out() as u8
    }

    /// Temperature in the current range, in milli-degrees Celsius.
    ///
    /// Computed as `0.4386 * value - 27.88 * offset - 20.52` in fixed point,
    /// so it needs no floating point support.
    pub fn millidegrees(&self) -> i32 {
        let value = i32::from(self.raw());
        (4_386 * value - 278_800 * self.range.offset() - 205_200) / 10
    }

    /// Temperature in the current range, in degrees Celsius.
    pub fn celsius(&self) -> f32 {
        self.millidegrees() as f32 / 1_000.0
    }

    /// Reads the temperature, switching the range if the reading does not fit
    /// the current one.
    ///
    /// Returns milli-degrees Celsius.
    pub fn update(&mut self) -> i32 {
        let mut millidegrees = self.millidegrees();
        let wanted = Range::for_temperature(millidegrees, self.range);
        if wanted != self.range {
            self.set_range(wanted);
            delay_us(SETTLE_US);
            millidegrees = self.millidegrees();
        }
        millidegrees
    }

    /// Sets how often the hardware monitors the temperature, in clock cycles
    /// of the low-power clock. `0xffff` is the slowest rate.
    pub fn set_sample_rate(&self, cycles: u16) {
        self.regs().sample_rate.write_sample_rate(u32::from(cycles));
    }

    /// Enables or disables the hardware's own sampling, which the monitoring
    /// thresholds are compared against.
    pub fn enable_sampling(&self, enable: bool) {
        if enable {
            self.regs().ctrl.set_sample_en();
        } else {
            self.regs().ctrl.clr_sample_en();
        }
    }

    /// Sets the monitoring thresholds, in raw sensor units.
    ///
    /// In absolute mode they are compared against the reading, in delta mode
    /// against the difference between two consecutive readings.
    pub fn set_thresholds(&self, low: u8, high: u8) {
        let wakeup = &self.regs().wakeup_ctrl;
        wakeup.write_wakeup_th_low(u32::from(low));
        wakeup.write_wakeup_th_high(u32::from(high));
    }

    /// Enables or disables the monitoring comparison.
    pub fn enable_monitoring(&self, enable: bool) {
        if enable {
            self.regs().wakeup_ctrl.set_wakeup_en();
        } else {
            self.regs().wakeup_ctrl.clr_wakeup_en();
        }
    }

    /// Enables or disables the delta (relative) monitoring mode.
    ///
    /// Absolute mode (the default) wakes when the reading leaves the
    /// `low..high` window; delta mode wakes when it moves by more than the
    /// thresholds between two samples.
    pub fn set_monitoring_delta(&self, delta: bool) {
        if delta {
            self.regs().wakeup_ctrl.set_wakeup_mode();
        } else {
            self.regs().wakeup_ctrl.clr_wakeup_mode();
        }
    }

    /// Which side of the window tripped the last wake-up, or `None` while
    /// monitoring has not tripped.
    pub fn monitoring_reason(&self) -> Option<bool> {
        if !self.interrupt_pending() {
            return None;
        }
        Some(self.regs().wakeup_ctrl.wakeup_over_upper_th())
    }

    /// Whether the monitoring interrupt is latched.
    #[inline]
    pub fn interrupt_pending(&self) -> bool {
        self.regs().int_raw.cocpu_tsens_wake_int_raw()
    }

    /// Clears the latched monitoring interrupt.
    #[inline]
    pub fn clear_interrupt(&self) {
        self.regs().int_clr.trigger_cocpu_tsens_wake_int_clr();
    }

    /// Enables or disables the monitoring interrupt.
    ///
    /// The interrupt line itself is `LP_TSENS_INTR` in the interrupt matrix;
    /// this only unmasks the sensor's own contribution.
    pub fn enable_interrupt(&self, enable: bool) {
        if enable {
            self.regs().int_ena.set_cocpu_tsens_wake_int_ena();
        } else {
            self.regs().int_ena.clr_cocpu_tsens_wake_int_ena();
        }
    }
}

/// Waits the sensor's settling time.
///
/// The sensor has no timebase of its own that software could poll, so this uses
/// the system timer. It is started on first use, which is also what
/// [`crate::delay::Delay`] does.
fn delay_us(us: u32) {
    // SAFETY: the delay driver only ever touches SYSTIMER's counter and
    // comparator 0, and it tolerates being constructed repeatedly.
    let delay = Delay::init(unsafe { SYSTIMER::steal() });
    delay.delay_us(us);
}
