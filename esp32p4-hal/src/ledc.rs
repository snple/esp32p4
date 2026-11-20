//! LED PWM controller (LEDC).
//!
//! Four timers, each with a fractional clock divider and its own resolution,
//! drive eight independent PWM generators. Every generator picks one timer, so
//! channels that share a timer share its frequency and resolution.
//!
//! PWM outputs are GPIO-matrix signals (`LEDC_LS_SIG_OUT_PAD_OUT0..7`), so any
//! GPIO can carry them.
//!
//! # Features
//!
//! - Timers: `XTAL_40M` / `FOSC_20M` / `REF_80M` clock sources, 1–20 bit
//!   resolution, fractional divider (8 fractional bits), start/stop/pause and
//!   count readback
//! - Channels: duty in timer counts or as a fraction, phase offset
//!   ([`ChannelConfig::hpoint`]), idle level, output enable
//! - Duty fade: 16 ranges per channel, each with direction, step, number of
//!   steps and overflow count between steps, plus pause/resume
//! - Timer overflow and fade-complete interrupts
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::{Pins, signal::OutputSignal};
//! use esp32p4_hal::ledc::{
//!     ChannelConfig, ChannelIndex, ClockSource, Ledc, Level, TimerConfig, TimerIndex,
//! };
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//!
//! let ledc = Ledc::init(dp.LEDC, &dp.HP_SYS_CLKRST);
//! ledc.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m); // 40 MHz
//!
//! // 5 kHz with 13-bit resolution: divider = 40 MHz / (5 kHz * 2^13).
//! ledc.configure_timer(
//!     TimerIndex::Zero,
//!     TimerConfig { clock: ClockSource::Xtal40m, frequency: 5_000, resolution: 13 },
//! )
//! .unwrap();
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT0);
//!
//! ledc.configure_channel(ChannelIndex::Zero, ChannelConfig::new(TimerIndex::Zero));
//! ledc.set_duty_fraction(ChannelIndex::Zero, 1, 4); // 25 %
//! ledc.start(TimerIndex::Zero);
//! ```
//!
//! # Hardware Notes
//!
//! - `HPOINT_CHn`, `DUTY_CHn`, `SIG_OUT_EN_CHn` and `TIMER_SEL_CHn` only take
//!   effect after `PARA_UP_CHn`, which lands on the timer's next overflow. The
//!   driver sets it for every configuration change, so a change can take up to
//!   one PWM period to appear.
//! - `DUTY_CHn[3:0]` is a 4-bit dither: the duty is one count longer on that
//!   many of every 16 periods. [`Ledc::set_duty`] leaves it at zero.
//! - The gamma fade RAM at `0x400 + channel * 0x40` has no register block of its
//!   own in the TRM; the driver reaches it through the generated `CHn_GAMMA_MEM`
//!   registers.

pub use crate::gpio::Level;

use esp32p4_pac::{HP_SYS_CLKRST, LEDC, ledc};

/// Which timer drives a channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimerIndex {
    /// Timer 0.
    Zero,
    /// Timer 1.
    One,
    /// Timer 2.
    Two,
    /// Timer 3.
    Three,
}

impl TimerIndex {
    /// Register field value (`TIMER_SEL_CHn`).
    const fn field(self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}

/// A PWM generator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChannelIndex {
    /// Channel 0.
    Zero,
    /// Channel 1.
    One,
    /// Channel 2.
    Two,
    /// Channel 3.
    Three,
    /// Channel 4.
    Four,
    /// Channel 5.
    Five,
    /// Channel 6.
    Six,
    /// Channel 7.
    Seven,
}

/// Clock feeding the timers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// `XTAL_40M_CLK`, 40 MHz. The reset default.
    #[default]
    Xtal40m,
    /// `FOSC_20M_CLK`, 20 MHz.
    Fosc20m,
    /// `REF_80M_CLK`, 80 MHz.
    Ref80m,
}

impl ClockSource {
    /// Nominal frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Xtal40m => crate::clock::XTAL_HZ,
            Self::Fosc20m => 20_000_000,
            Self::Ref80m => 80_000_000,
        }
    }

    /// Register field value (`PCR_LEDC_SCLK_SEL`).
    const fn field(self) -> u32 {
        match self {
            Self::Xtal40m => 0,
            Self::Fosc20m => 1,
            Self::Ref80m => 2,
        }
    }
}

/// How a timer's counter is clocked.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimerConfig {
    /// Clock source.
    pub clock: ClockSource,
    /// PWM frequency in Hz.
    pub frequency: u32,
    /// Counter width in bits, 1–20. Higher resolution lowers the top
    /// frequency: `frequency * 2^resolution` must not exceed the clock.
    pub resolution: u32,
}

impl TimerConfig {
    /// 5 kHz at 13 bits from the 40 MHz crystal, a typical LED setting.
    pub const fn new(frequency: u32, resolution: u32) -> Self {
        Self {
            clock: ClockSource::Xtal40m,
            frequency,
            resolution,
        }
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self::new(5_000, 13)
    }
}

impl Level {
    const fn field(self) -> bool {
        matches!(self, Self::High)
    }
}

/// Static configuration of one PWM generator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChannelConfig {
    /// Timer that drives the channel.
    pub timer: TimerIndex,
    /// Initial duty in timer counts, `0..=max_duty`.
    pub duty: u32,
    /// Counter value at which the output goes high, which shifts the phase
    /// within the period.
    pub hpoint: u32,
    /// Level driven while the output is disabled.
    pub idle_level: Level,
}

impl ChannelConfig {
    /// Zero duty, zero phase, idle low.
    pub const fn new(timer: TimerIndex) -> Self {
        Self {
            timer,
            duty: 0,
            hpoint: 0,
            idle_level: Level::Low,
        }
    }
}

/// One interval of a duty fade.
///
/// The duty changes by `scale` counts every `cycles` counter overflows, and it
/// does so `steps` times before the next interval starts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FadeRange {
    /// Whether the duty increases (`true`) or decreases (`false`).
    pub increasing: bool,
    /// Overflows between two duty steps, 1–1023.
    pub cycles: u16,
    /// Duty change per step, in counts, 1–1023.
    pub scale: u16,
    /// Number of steps, 1–1023.
    pub steps: u16,
}

impl FadeRange {
    /// Linear ramp over `steps` steps.
    pub const fn linear(increasing: bool, cycles: u16, scale: u16, steps: u16) -> Self {
        Self {
            increasing,
            cycles,
            scale,
            steps,
        }
    }

    /// The 32-bit word the fade RAM expects.
    const fn word(self) -> u32 {
        (self.increasing as u32)
            | ((self.cycles as u32 & 0x3ff) << 1)
            | ((self.scale as u32 & 0x3ff) << 11)
            | ((self.steps as u32 & 0x3ff) << 21)
    }
}

/// Fade intervals a channel can hold.
pub const FADE_RANGES: usize = 16;

/// Errors from the configuration setters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The resolution is outside 1–20 bits.
    Resolution,
    /// `frequency * 2^resolution` exceeds the clock source.
    FrequencyTooHigh,
    /// The divider the frequency asks for exceeds its 10-bit integer part.
    FrequencyTooLow,
    /// The fade range index is not below [`FADE_RANGES`].
    FadeRange,
    /// The fade interval count is zero or above [`FADE_RANGES`].
    FadeRangeCount,
}

/// Split a divider into the 10-bit integer and 8-bit fractional parts the
/// register holds.
const fn divider_parts(divider: u32) -> (u32, u32) {
    (divider >> 8, divider & 0xff)
}

macro_rules! on_timer {
    ($self:expr, $timer:expr, |$conf:ident| $($body:tt)*) => {
        match $timer {
            TimerIndex::Zero => {
                let $conf = &$self.ledc.timer0_conf;
                $($body)*
            }
            TimerIndex::One => {
                let $conf = &$self.ledc.timer1_conf;
                $($body)*
            }
            TimerIndex::Two => {
                let $conf = &$self.ledc.timer2_conf;
                $($body)*
            }
            TimerIndex::Three => {
                let $conf = &$self.ledc.timer3_conf;
                $($body)*
            }
        }
    };
}

macro_rules! on_channel {
    ($self:expr, $channel:expr, |$conf0:ident, $hpoint:ident, $conf1:ident, $duty:ident, $duty_r:ident, $gamma:ident, $ram:ident| $($body:tt)*) => {
        match $channel {
            ChannelIndex::Zero => {
                let $conf0 = &$self.ledc.ch0_conf0;
                let $hpoint = &$self.ledc.ch0_hpoint;
                let $conf1 = &$self.ledc.ch0_conf1;
                let $duty = &$self.ledc.ch0_duty;
                let $duty_r = &$self.ledc.ch0_duty_r;
                let $gamma = &$self.ledc.ch0_gamma_conf;
                let $ram = &$self.ledc.ch0_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::One => {
                let $conf0 = &$self.ledc.ch1_conf0;
                let $hpoint = &$self.ledc.ch1_hpoint;
                let $conf1 = &$self.ledc.ch1_conf1;
                let $duty = &$self.ledc.ch1_duty;
                let $duty_r = &$self.ledc.ch1_duty_r;
                let $gamma = &$self.ledc.ch1_gamma_conf;
                let $ram = &$self.ledc.ch1_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Two => {
                let $conf0 = &$self.ledc.ch2_conf0;
                let $hpoint = &$self.ledc.ch2_hpoint;
                let $conf1 = &$self.ledc.ch2_conf1;
                let $duty = &$self.ledc.ch2_duty;
                let $duty_r = &$self.ledc.ch2_duty_r;
                let $gamma = &$self.ledc.ch2_gamma_conf;
                let $ram = &$self.ledc.ch2_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Three => {
                let $conf0 = &$self.ledc.ch3_conf0;
                let $hpoint = &$self.ledc.ch3_hpoint;
                let $conf1 = &$self.ledc.ch3_conf1;
                let $duty = &$self.ledc.ch3_duty;
                let $duty_r = &$self.ledc.ch3_duty_r;
                let $gamma = &$self.ledc.ch3_gamma_conf;
                let $ram = &$self.ledc.ch3_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Four => {
                let $conf0 = &$self.ledc.ch4_conf0;
                let $hpoint = &$self.ledc.ch4_hpoint;
                let $conf1 = &$self.ledc.ch4_conf1;
                let $duty = &$self.ledc.ch4_duty;
                let $duty_r = &$self.ledc.ch4_duty_r;
                let $gamma = &$self.ledc.ch4_gamma_conf;
                let $ram = &$self.ledc.ch4_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Five => {
                let $conf0 = &$self.ledc.ch5_conf0;
                let $hpoint = &$self.ledc.ch5_hpoint;
                let $conf1 = &$self.ledc.ch5_conf1;
                let $duty = &$self.ledc.ch5_duty;
                let $duty_r = &$self.ledc.ch5_duty_r;
                let $gamma = &$self.ledc.ch5_gamma_conf;
                let $ram = &$self.ledc.ch5_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Six => {
                let $conf0 = &$self.ledc.ch6_conf0;
                let $hpoint = &$self.ledc.ch6_hpoint;
                let $conf1 = &$self.ledc.ch6_conf1;
                let $duty = &$self.ledc.ch6_duty;
                let $duty_r = &$self.ledc.ch6_duty_r;
                let $gamma = &$self.ledc.ch6_gamma_conf;
                let $ram = &$self.ledc.ch6_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
            ChannelIndex::Seven => {
                let $conf0 = &$self.ledc.ch7_conf0;
                let $hpoint = &$self.ledc.ch7_hpoint;
                let $conf1 = &$self.ledc.ch7_conf1;
                let $duty = &$self.ledc.ch7_duty;
                let $duty_r = &$self.ledc.ch7_duty_r;
                let $gamma = &$self.ledc.ch7_gamma_conf;
                let $ram = &$self.ledc.ch7_gamma_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>;
                $($body)*
            }
        }
    };
}
/// LED PWM controller owning the PAC peripheral.
pub struct Ledc {
    ledc: LEDC,
}

impl Ledc {
    /// Initialises the controller: bus clock, core clock, reset release.
    ///
    /// The timer clock source keeps its reset value (`XTAL_40M`); use
    /// [`set_clock_source`](Self::set_clock_source) to change it.
    pub fn init(ledc: LEDC, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl3.set_reg_ledc_apb_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_ledc();
        clkrst.hp_rst_en1.clr_reg_rst_en_ledc();
        clkrst.peri_clk_ctrl22.set_reg_ledc_clk_en();

        let self_ = Self { ledc };
        // Force the register clock gate open: without it the channel registers
        // and the fade RAM read back as zero.
        self_.ledc.conf.set_clk_en();
        self_
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the controller was initialised and that the
    /// returned handle is not used to reconfigure it concurrently.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            ledc: unsafe { LEDC::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &ledc::RegisterBlock {
        &self.ledc
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> LEDC {
        self.ledc
    }

    /// Selects the timer clock source.
    ///
    /// Timers already running keep their divider, so the frequency changes
    /// with the clock; reconfigure them to keep the same PWM frequency.
    pub fn set_clock_source(&self, clkrst: &HP_SYS_CLKRST, source: ClockSource) {
        clkrst
            .peri_clk_ctrl22
            .write_reg_ledc_clk_src_sel(source.field());
    }

    // ========================================================================
    // Timers
    // ========================================================================

    /// Applies `config` to a timer: clock divider, resolution, then `PARA_UP`.
    ///
    /// # Errors
    ///
    /// [`Error::Resolution`] when the resolution is outside 1–20 bits,
    /// [`Error::FrequencyTooHigh`] when the frequency needs a divider below 1,
    /// and [`Error::FrequencyTooLow`] when it needs more than 1023 + 255/256.
    pub fn configure_timer(&self, timer: TimerIndex, config: TimerConfig) -> Result<(), Error> {
        if !(1..=20).contains(&config.resolution) {
            return Err(Error::Resolution);
        }

        // divider = clock / (frequency * 2^resolution), in 8.8 fixed point.
        let divider =
            ((config.clock.hz() as u64) << 8) / ((config.frequency as u64) << config.resolution);
        if divider == 0 {
            return Err(Error::FrequencyTooHigh);
        }
        let (integer, fraction) = divider_parts(divider as u32);
        if integer > 1023 {
            return Err(Error::FrequencyTooLow);
        }

        on_timer!(self, timer, |conf| {
            conf.write_timer_duty_res(config.resolution);
            conf.write_clk_div_timer(integer << 8 | fraction);
            conf.trigger_timer_para_up();
        });
        Ok(())
    }

    /// Runs the timer.
    ///
    /// Releases the counter reset and the pause; the counter reset is asserted
    /// out of reset, so a timer that is only un-paused never counts.
    pub fn start(&self, timer: TimerIndex) {
        on_timer!(self, timer, |conf| {
            conf.clr_timer_rst();
            conf.clr_timer_pause();
        });
    }

    /// Freezes the timer, holding the current output level.
    pub fn stop(&self, timer: TimerIndex) {
        on_timer!(self, timer, |conf| conf.set_timer_pause());
    }

    /// Resets the counter to zero. The timer keeps running or stays paused.
    pub fn reset_counter(&self, timer: TimerIndex) {
        on_timer!(self, timer, |conf| {
            conf.set_timer_rst();
            conf.clr_timer_rst();
        });
    }

    /// The counter's current value.
    pub fn counter(&self, timer: TimerIndex) -> u32 {
        match timer {
            TimerIndex::Zero => self.ledc.timer0_value.timer_cnt(),
            TimerIndex::One => self.ledc.timer1_value.timer_cnt(),
            TimerIndex::Two => self.ledc.timer2_value.timer_cnt(),
            TimerIndex::Three => self.ledc.timer3_value.timer_cnt(),
        }
    }

    /// The highest duty the timer's resolution allows, `2^resolution - 1`.
    pub fn max_duty(&self, timer: TimerIndex) -> u32 {
        let resolution = match timer {
            TimerIndex::Zero => self.ledc.timer0_conf.timer_duty_res(),
            TimerIndex::One => self.ledc.timer1_conf.timer_duty_res(),
            TimerIndex::Two => self.ledc.timer2_conf.timer_duty_res(),
            TimerIndex::Three => self.ledc.timer3_conf.timer_duty_res(),
        };
        (1u32 << resolution) - 1
    }

    /// Whether the timer has overflowed since the last clear.
    pub fn timer_overflow_pending(&self, timer: TimerIndex) -> bool {
        match timer {
            TimerIndex::Zero => self.ledc.int_raw.timer0_ovf_int_raw(),
            TimerIndex::One => self.ledc.int_raw.timer1_ovf_int_raw(),
            TimerIndex::Two => self.ledc.int_raw.timer2_ovf_int_raw(),
            TimerIndex::Three => self.ledc.int_raw.timer3_ovf_int_raw(),
        }
    }

    /// Clears the timer's overflow status.
    pub fn clear_timer_overflow(&self, timer: TimerIndex) {
        match timer {
            TimerIndex::Zero => self.ledc.int_clr.trigger_timer0_ovf_int_clr(),
            TimerIndex::One => self.ledc.int_clr.trigger_timer1_ovf_int_clr(),
            TimerIndex::Two => self.ledc.int_clr.trigger_timer2_ovf_int_clr(),
            TimerIndex::Three => self.ledc.int_clr.trigger_timer3_ovf_int_clr(),
        }
    }

    /// Enables the timer's overflow interrupt.
    pub fn enable_timer_overflow_interrupt(&self, timer: TimerIndex, enable: bool) {
        match (timer, enable) {
            (TimerIndex::Zero, true) => self.ledc.int_ena.set_timer0_ovf_int_ena(),
            (TimerIndex::Zero, false) => self.ledc.int_ena.clr_timer0_ovf_int_ena(),
            (TimerIndex::One, true) => self.ledc.int_ena.set_timer1_ovf_int_ena(),
            (TimerIndex::One, false) => self.ledc.int_ena.clr_timer1_ovf_int_ena(),
            (TimerIndex::Two, true) => self.ledc.int_ena.set_timer2_ovf_int_ena(),
            (TimerIndex::Two, false) => self.ledc.int_ena.clr_timer2_ovf_int_ena(),
            (TimerIndex::Three, true) => self.ledc.int_ena.set_timer3_ovf_int_ena(),
            (TimerIndex::Three, false) => self.ledc.int_ena.clr_timer3_ovf_int_ena(),
        }
    }

    // ========================================================================
    // Channels
    // ========================================================================

    /// Applies `config` to a channel and enables its output, without starting
    /// the timer.
    pub fn configure_channel(&self, channel: ChannelIndex, config: ChannelConfig) {
        on_channel!(self, channel, |conf0, hpoint, _u2, duty, _u3, _u4, _u5| {
            conf0.write_timer_sel_ch(config.timer.field());
            conf0.write_ovf_num_ch(0);
            conf0.clr_idle_lv_ch();
            if config.idle_level.field() {
                conf0.set_idle_lv_ch();
            }
            hpoint.write_hpoint_ch(config.hpoint);
            duty.write_duty_ch(config.duty << 4);
            conf0.set_sig_out_en_ch();
            conf0.trigger_para_up_ch();
        });
    }

    /// Sets the duty in timer counts and applies it on the next overflow.
    ///
    /// `DUTY_START` is set along with it: the hardware only copies `DUTY` into
    /// the comparator while that bit is set, so a plain duty change would
    /// otherwise never take effect.
    pub fn set_duty(&self, channel: ChannelIndex, duty: u32) {
        on_channel!(self, channel, |conf0,
                                    _u1,
                                    conf1,
                                    duty_reg,
                                    _u3,
                                    _u4,
                                    _u5| {
            duty_reg.write_duty_ch(duty << 4);
            conf1.set_duty_start_ch();
            conf0.trigger_para_up_ch();
        });
    }

    /// Sets the duty as `numerator / denominator` of the timer's full range,
    /// rounded down. Returns the count that was written.
    ///
    /// # Panics
    ///
    /// Panics when `denominator` is zero.
    pub fn set_duty_fraction(
        &self,
        channel: ChannelIndex,
        numerator: u32,
        denominator: u32,
    ) -> u32 {
        assert!(denominator != 0, "LEDC duty denominator cannot be zero");
        let timer = self.channel_timer(channel);
        let duty = (self.max_duty(timer) as u64 * numerator as u64 / denominator as u64) as u32;
        self.set_duty(channel, duty);
        duty
    }

    /// The duty the generator is currently using, in timer counts.
    ///
    /// During a fade this follows the ramp; [`set_duty`](Self::set_duty) reads
    /// back the configured value one overflow later.
    pub fn duty(&self, channel: ChannelIndex) -> u32 {
        on_channel!(self, channel, |_u0, _u1, _u2, _u2, duty_r, _u4, _u5| duty_r
            .duty_ch_r()
            >> 4)
    }

    /// The timer whose counter drives the channel.
    pub fn channel_timer(&self, channel: ChannelIndex) -> TimerIndex {
        let field = on_channel!(self, channel, |conf0, _u1, _u2, _u2, _u3, _u4, _u5| conf0
            .timer_sel_ch());
        match field {
            0 => TimerIndex::Zero,
            1 => TimerIndex::One,
            2 => TimerIndex::Two,
            _ => TimerIndex::Three,
        }
    }

    /// Enables or disables the PWM output. While disabled the channel drives
    /// its idle level.
    pub fn enable(&self, channel: ChannelIndex, enable: bool) {
        on_channel!(self, channel, |conf0, _u1, _u2, _u2, _u3, _u4, _u5| {
            if enable {
                conf0.set_sig_out_en_ch();
            } else {
                conf0.clr_sig_out_en_ch();
            }
            conf0.trigger_para_up_ch();
        });
    }

    /// Sets the level driven while the output is disabled.
    pub fn set_idle_level(&self, channel: ChannelIndex, level: Level) {
        on_channel!(self, channel, |conf0, _u1, _u2, _u2, _u3, _u4, _u5| {
            conf0.clr_idle_lv_ch();
            if level.field() {
                conf0.set_idle_lv_ch();
            }
            conf0.trigger_para_up_ch();
        });
    }

    // ========================================================================
    // Duty fade
    // ========================================================================

    /// Writes one fade interval into the channel's gamma RAM.
    ///
    /// # Errors
    ///
    /// [`Error::FadeRange`] when `range` is not below [`FADE_RANGES`].
    pub fn set_fade_range(
        &self,
        channel: ChannelIndex,
        range: usize,
        fade: FadeRange,
    ) -> Result<(), Error> {
        if range >= FADE_RANGES {
            return Err(Error::FadeRange);
        }
        on_channel!(self, channel, |_u0, _u1, _u2, _u2, _u3, _u4, ram| {
            // SAFETY: `ram` points at `CHn_GAMMA_MEM0`, the base of the
            // channel's 16-word fade RAM (TRM 58.7, `0x400 + n * 0x40`).
            unsafe {
                let cell = &*ram.add(range);
                cell.write_bits(fade.word());
            }
        });
        Ok(())
    }

    /// Sets how many fade intervals take part in one fade, 1–16.
    ///
    /// # Errors
    ///
    /// [`Error::FadeRangeCount`] when the count is out of range.
    pub fn set_fade_range_count(&self, channel: ChannelIndex, count: usize) -> Result<(), Error> {
        if !(1..=FADE_RANGES).contains(&count) {
            return Err(Error::FadeRangeCount);
        }
        on_channel!(self, channel, |conf0, _u1, _u2, _u2, _u3, gamma, _u5| {
            gamma.write_ch_gamma_entry_num(count as u32);
            conf0.trigger_para_up_ch();
        });
        Ok(())
    }

    /// Enables the fade: the duty starts ramping from its current value.
    pub fn start_fade(&self, channel: ChannelIndex) {
        on_channel!(self, channel, |conf0, _u1, conf1, _u3, _u4, _u5, _u6| {
            conf1.set_duty_start_ch();
            conf0.trigger_para_up_ch();
        });
    }

    /// Stops the fade. The duty keeps the value it had reached.
    pub fn stop_fade(&self, channel: ChannelIndex) {
        on_channel!(self, channel, |conf0, _u1, conf1, _u3, _u4, _u5, _u6| {
            conf1.clr_duty_start_ch();
            conf0.trigger_para_up_ch();
        });
    }

    /// Pauses a running fade, keeping the current duty.
    pub fn pause_fade(&self, channel: ChannelIndex) {
        on_channel!(self, channel, |_u0, _u1, _u2, _u2, _u3, gamma, _u5| {
            gamma.trigger_ch_gamma_pause();
        });
    }

    /// Resumes a paused fade from the interval it stopped in.
    pub fn resume_fade(&self, channel: ChannelIndex) {
        on_channel!(self, channel, |_u0, _u1, _u2, _u2, _u3, gamma, _u5| {
            gamma.trigger_ch_gamma_resume();
        });
    }

    /// Whether a fade has completed since the last clear.
    pub fn fade_done(&self, channel: ChannelIndex) -> bool {
        match channel {
            ChannelIndex::Zero => self.ledc.int_raw.duty_chng_end_ch0_int_raw(),
            ChannelIndex::One => self.ledc.int_raw.duty_chng_end_ch1_int_raw(),
            ChannelIndex::Two => self.ledc.int_raw.duty_chng_end_ch2_int_raw(),
            ChannelIndex::Three => self.ledc.int_raw.duty_chng_end_ch3_int_raw(),
            ChannelIndex::Four => self.ledc.int_raw.duty_chng_end_ch4_int_raw(),
            ChannelIndex::Five => self.ledc.int_raw.duty_chng_end_ch5_int_raw(),
            ChannelIndex::Six => self.ledc.int_raw.duty_chng_end_ch6_int_raw(),
            ChannelIndex::Seven => self.ledc.int_raw.duty_chng_end_ch7_int_raw(),
        }
    }

    /// Clears the fade-complete status.
    pub fn clear_fade_done(&self, channel: ChannelIndex) {
        match channel {
            ChannelIndex::Zero => self.ledc.int_clr.trigger_duty_chng_end_ch0_int_clr(),
            ChannelIndex::One => self.ledc.int_clr.trigger_duty_chng_end_ch1_int_clr(),
            ChannelIndex::Two => self.ledc.int_clr.trigger_duty_chng_end_ch2_int_clr(),
            ChannelIndex::Three => self.ledc.int_clr.trigger_duty_chng_end_ch3_int_clr(),
            ChannelIndex::Four => self.ledc.int_clr.trigger_duty_chng_end_ch4_int_clr(),
            ChannelIndex::Five => self.ledc.int_clr.trigger_duty_chng_end_ch5_int_clr(),
            ChannelIndex::Six => self.ledc.int_clr.trigger_duty_chng_end_ch6_int_clr(),
            ChannelIndex::Seven => self.ledc.int_clr.trigger_duty_chng_end_ch7_int_clr(),
        }
    }

    /// Enables the fade-complete interrupt.
    pub fn enable_fade_interrupt(&self, channel: ChannelIndex, enable: bool) {
        match (channel, enable) {
            (ChannelIndex::Zero, true) => self.ledc.int_ena.set_duty_chng_end_ch0_int_ena(),
            (ChannelIndex::Zero, false) => self.ledc.int_ena.clr_duty_chng_end_ch0_int_ena(),
            (ChannelIndex::One, true) => self.ledc.int_ena.set_duty_chng_end_ch1_int_ena(),
            (ChannelIndex::One, false) => self.ledc.int_ena.clr_duty_chng_end_ch1_int_ena(),
            (ChannelIndex::Two, true) => self.ledc.int_ena.set_duty_chng_end_ch2_int_ena(),
            (ChannelIndex::Two, false) => self.ledc.int_ena.clr_duty_chng_end_ch2_int_ena(),
            (ChannelIndex::Three, true) => self.ledc.int_ena.set_duty_chng_end_ch3_int_ena(),
            (ChannelIndex::Three, false) => self.ledc.int_ena.clr_duty_chng_end_ch3_int_ena(),
            (ChannelIndex::Four, true) => self.ledc.int_ena.set_duty_chng_end_ch4_int_ena(),
            (ChannelIndex::Four, false) => self.ledc.int_ena.clr_duty_chng_end_ch4_int_ena(),
            (ChannelIndex::Five, true) => self.ledc.int_ena.set_duty_chng_end_ch5_int_ena(),
            (ChannelIndex::Five, false) => self.ledc.int_ena.clr_duty_chng_end_ch5_int_ena(),
            (ChannelIndex::Six, true) => self.ledc.int_ena.set_duty_chng_end_ch6_int_ena(),
            (ChannelIndex::Six, false) => self.ledc.int_ena.clr_duty_chng_end_ch6_int_ena(),
            (ChannelIndex::Seven, true) => self.ledc.int_ena.set_duty_chng_end_ch7_int_ena(),
            (ChannelIndex::Seven, false) => self.ledc.int_ena.clr_duty_chng_end_ch7_int_ena(),
        }
    }
}
