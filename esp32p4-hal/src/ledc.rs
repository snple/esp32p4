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
//!   many of every 16 periods. `Channel::set_duty` leaves it at zero.
//! - The gamma fade RAM at `0x400 + channel * 0x40` has no register block of its
//!   own in the TRM; the driver reaches it through the generated `CHn_GAMMA_MEM`
//!   registers.

pub use crate::gpio::Level;

use esp32p4_pac::{HP_SYS_CLKRST, LEDC, ledc};

/// Clock feeding the timers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// `XTAL_40M_CLK`, 40 MHz.
    Xtal40m,
    /// `FOSC_20M_CLK`, 20 MHz.
    Fosc20m,
    /// `REF_80M_CLK`, 80 MHz. What the timer counts after reset, provided the
    /// reference clock is running: switch it on with
    /// [`ClockTree::enable_ref`](crate::clock::ClockTree::enable_ref), or the
    /// timer never counts.
    #[default]
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
    /// A configuration on the reference clock the timer selects after reset.
    ///
    /// `frequency * 2^resolution` has to fit 80 MHz, and the 80 MHz reference
    /// has to be enabled with
    /// [`ClockTree::enable_ref`](crate::clock::ClockTree::enable_ref). Use
    /// [`ClockSource::Xtal40m`] to run from the crystal instead, which is
    /// always on.
    pub const fn new(frequency: u32, resolution: u32) -> Self {
        Self {
            clock: ClockSource::Ref80m,
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
    /// Initial duty in timer counts, `0..=max_duty`.
    pub duty: u32,
    /// Counter value at which the output goes high, which shifts the phase
    /// within the period.
    pub hpoint: u32,
    /// Level driven while the output is disabled.
    pub idle_level: Level,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelConfig {
    /// Zero duty, zero phase, idle low.
    pub const fn new() -> Self {
        Self {
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

/// LED PWM controller owning the PAC peripheral.
pub struct Ledc {
    ledc: LEDC,
}

impl Ledc {
    /// Initialises the controller: bus clock, core clock, reset release.
    ///
    /// The timer clock source keeps its reset value (`REF_80M_CLK`, 80 MHz),
    /// which is only a *selection*: the 80 MHz reference has to be enabled
    /// ([`ClockTree::enable_ref`](crate::clock::ClockTree::enable_ref)) or the
    /// timer stays stopped. [`ClockSource::Xtal40m`] needs no such step.
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
}

/// A PWM timer, `N` of four.
///
/// The handle carries the timer number in its type; the register names behind it
/// are literals that the `timer_body!` macro generates once per timer, so nothing here is
/// dispatched at run time.
pub struct Timer<'a, const N: u8>(&'a Ledc);

/// A PWM generator, `N` of eight.
pub struct Channel<'a, const N: u8>(&'a Ledc);

impl Ledc {
    /// A handle to timer `N`.
    ///
    /// `N` is 0–3; anything else fails to compile.
    #[inline]
    pub fn timer<const N: u8>(&self) -> Timer<'_, N> {
        const { assert!(N < 4, "this controller has four timers, 0 to 3") };
        Timer(self)
    }

    /// A handle to channel `N`.
    ///
    /// `N` is 0–7; anything else fails to compile.
    #[inline]
    pub fn channel<const N: u8>(&self) -> Channel<'_, N> {
        const { assert!(N < 8, "this controller has eight channels, 0 to 7") };
        Channel(self)
    }
}

/// Generates the per-timer methods.
macro_rules! timer_body {
    ($n:literal) => {
        pastey::paste! {
            impl<'a> Timer<'a, $n> {
                /// Applies `config`: clock divider, resolution, then `PARA_UP`.
                ///
                /// # Errors
                ///
                /// [`Error::Resolution`] when the resolution is outside 1–20
                /// bits, [`Error::FrequencyTooHigh`] when the frequency needs a
                /// divider below 1, and [`Error::FrequencyTooLow`] when it needs
                /// more than 1023 + 255/256.
                pub fn configure(&self, config: TimerConfig) -> Result<(), Error> {
                    if !(1..=20).contains(&config.resolution) {
                        return Err(Error::Resolution);
                    }
                    // divider = clock / (frequency * 2^resolution), 8.8 fixed.
                    let divider = ((config.clock.hz() as u64) << 8)
                        / ((config.frequency as u64) << config.resolution);
                    if divider == 0 {
                        return Err(Error::FrequencyTooHigh);
                    }
                    let (integer, fraction) = divider_parts(divider as u32);
                    if integer > 1023 {
                        return Err(Error::FrequencyTooLow);
                    }
                    let conf = &self.0.regs().[<timer $n _conf>];
                    conf.write_timer_duty_res(config.resolution);
                    conf.write_clk_div_timer(integer << 8 | fraction);
                    conf.trigger_timer_para_up();
                    Ok(())
                }

                /// Runs the timer.
                ///
                /// Releases the counter reset and the pause; the counter reset
                /// is asserted out of reset, so a timer that is only un-paused
                /// never counts.
                pub fn start(&self) {
                    let conf = &self.0.regs().[<timer $n _conf>];
                    conf.clr_timer_rst();
                    conf.clr_timer_pause();
                }

                /// Freezes the timer, holding the current output level.
                pub fn stop(&self) {
                    self.0.regs().[<timer $n _conf>].set_timer_pause();
                }

                /// Resets the counter to zero, running or paused.
                pub fn reset_counter(&self) {
                    let conf = &self.0.regs().[<timer $n _conf>];
                    conf.set_timer_rst();
                    conf.clr_timer_rst();
                }

                /// The counter's current value.
                pub fn counter(&self) -> u32 {
                    self.0.regs().[<timer $n _value>].timer_cnt()
                }

                /// The highest duty this timer's resolution allows.
                pub fn max_duty(&self) -> u32 {
                    let resolution = self.0.regs().[<timer $n _conf>].timer_duty_res();
                    (1u32 << resolution) - 1
                }

                /// `numerator / denominator` of this timer's full range, rounded
                /// down: the count to hand to
                /// [`Channel::set_duty`].
                ///
                /// Only the timer knows its resolution, which is why this is not
                /// a method on the channel.
                ///
                /// # Panics
                ///
                /// Panics when `denominator` is zero.
                pub fn duty_fraction(&self, numerator: u32, denominator: u32) -> u32 {
                    assert!(denominator != 0, "LEDC duty denominator cannot be zero");
                    (self.max_duty() as u64 * numerator as u64 / denominator as u64) as u32
                }

                /// Whether the timer has overflowed since the last clear.
                pub fn overflow_pending(&self) -> bool {
                    self.0.regs().int_raw.[<timer $n _ovf_int_raw>]()
                }

                /// Clears the overflow status.
                pub fn clear_overflow(&self) {
                    self.0.regs().int_clr.[<trigger_timer $n _ovf_int_clr>]();
                }

                /// Enables or disables the overflow interrupt.
                pub fn enable_overflow_interrupt(&self, enable: bool) {
                    let int_ena = &self.0.regs().int_ena;
                    if enable {
                        int_ena.[<set_timer $n _ovf_int_ena>]();
                    } else {
                        int_ena.[<clr_timer $n _ovf_int_ena>]();
                    }
                }
            }
        }
    };
}

/// Generates the per-channel methods.
///
/// The timer a channel runs from is a register field (`TIMER_SEL_CHn`), so it is
/// set through [`bind`](Channel::bind) from a timer handle rather than from an
/// index value.
macro_rules! channel_body {
    ($n:literal) => {
        pastey::paste! {
            impl<'a> Channel<'a, $n> {
                /// Applies `config` and enables the output, without starting the
                /// timer. Call [`bind`](Self::bind) to pick the timer.
                pub fn configure(&self, config: ChannelConfig) {
                    let regs = self.0.regs();
                    let conf0 = &regs.[<ch $n _conf0>];
                    conf0.write_ovf_num_ch(0);
                    conf0.clr_idle_lv_ch();
                    if config.idle_level.field() {
                        conf0.set_idle_lv_ch();
                    }
                    regs.[<ch $n _hpoint>].write_hpoint_ch(config.hpoint);
                    regs.[<ch $n _duty>].write_duty_ch(config.duty << 4);
                    conf0.set_sig_out_en_ch();
                    conf0.trigger_para_up_ch();
                }

                /// Points the channel at `timer`.
                ///
                /// The timer number lives in the timer's type, so a channel can
                /// only be bound to a handle that exists.
                pub fn bind<const M: u8>(&self, timer: &Timer<'_, M>) {
                    let _ = timer;
                    let conf0 = &self.0.regs().[<ch $n _conf0>];
                    conf0.write_timer_sel_ch(M as u32);
                    conf0.trigger_para_up_ch();
                }

                /// Sets the duty in timer counts and applies it on the next
                /// overflow.
                ///
                /// `DUTY_START` is set along with it: the hardware only copies
                /// `DUTY` into the comparator while that bit is set, so a plain
                /// duty change would otherwise never take effect.
                pub fn set_duty(&self, duty: u32) {
                    let regs = self.0.regs();
                    let conf0 = &regs.[<ch $n _conf0>];
                    regs.[<ch $n _duty>].write_duty_ch(duty << 4);
                    regs.[<ch $n _conf1>].set_duty_start_ch();
                    conf0.trigger_para_up_ch();
                }

                /// The duty the generator is currently using, in timer counts.
                ///
                /// During a fade this follows the ramp;
                /// [`set_duty`](Self::set_duty) reads back the configured value
                /// one overflow later.
                pub fn duty(&self) -> u32 {
                    self.0.regs().[<ch $n _duty_r>].duty_ch_r() >> 4
                }

                /// Enables or disables the PWM output. While disabled the
                /// channel drives its idle level.
                pub fn enable(&self, enable: bool) {
                    let conf0 = &self.0.regs().[<ch $n _conf0>];
                    if enable {
                        conf0.set_sig_out_en_ch();
                    } else {
                        conf0.clr_sig_out_en_ch();
                    }
                    conf0.trigger_para_up_ch();
                }

                /// Sets the level driven while the output is disabled.
                pub fn set_idle_level(&self, level: Level) {
                    let conf0 = &self.0.regs().[<ch $n _conf0>];
                    conf0.clr_idle_lv_ch();
                    if level.field() {
                        conf0.set_idle_lv_ch();
                    }
                    conf0.trigger_para_up_ch();
                }

                /// Writes one fade interval into the channel's gamma RAM.
                ///
                /// # Errors
                ///
                /// [`Error::FadeRange`] when `range` is not below
                /// [`FADE_RANGES`].
                pub fn set_fade_range(&self, range: usize, fade: FadeRange) -> Result<(), Error> {
                    if range >= FADE_RANGES {
                        return Err(Error::FadeRange);
                    }
                    // SAFETY: the pointer is the base of this channel's 16-word
                    // fade RAM (TRM 58.7, `0x400 + n * 0x40`), and `range` is
                    // checked against its length above.
                    let ram = &self.0.regs().[<ch $n _gamma_mem0>] as *const _
                        as *const crate::pac::common::VolatileCell<u32>;
                    unsafe {
                        let cell = &*ram.add(range);
                        cell.write_bits(fade.word());
                    }
                    Ok(())
                }

                /// Sets how many fade intervals take part in one fade, 1–16.
                ///
                /// # Errors
                ///
                /// [`Error::FadeRangeCount`] when the count is out of range.
                pub fn set_fade_range_count(&self, count: usize) -> Result<(), Error> {
                    if !(1..=FADE_RANGES).contains(&count) {
                        return Err(Error::FadeRangeCount);
                    }
                    let regs = self.0.regs();
                    regs.[<ch $n _gamma_conf>].write_ch_gamma_entry_num(count as u32);
                    regs.[<ch $n _conf0>].trigger_para_up_ch();
                    Ok(())
                }

                /// Enables the fade: the duty starts ramping from its value.
                pub fn start_fade(&self) {
                    let regs = self.0.regs();
                    regs.[<ch $n _conf1>].set_duty_start_ch();
                    regs.[<ch $n _conf0>].trigger_para_up_ch();
                }

                /// Stops the fade. The duty keeps the value it had reached.
                pub fn stop_fade(&self) {
                    let regs = self.0.regs();
                    regs.[<ch $n _conf1>].clr_duty_start_ch();
                    regs.[<ch $n _conf0>].trigger_para_up_ch();
                }

                /// Pauses a running fade, keeping the current duty.
                pub fn pause_fade(&self) {
                    self.0.regs().[<ch $n _gamma_conf>].trigger_ch_gamma_pause();
                }

                /// Resumes a paused fade from the interval it stopped in.
                pub fn resume_fade(&self) {
                    self.0.regs().[<ch $n _gamma_conf>].trigger_ch_gamma_resume();
                }

                /// Whether a fade has completed since the last clear.
                pub fn fade_done(&self) -> bool {
                    self.0.regs().int_raw.[<duty_chng_end_ch $n _int_raw>]()
                }

                /// Clears the fade-complete status.
                pub fn clear_fade_done(&self) {
                    self.0.regs().int_clr.[<trigger_duty_chng_end_ch $n _int_clr>]();
                }

                /// Enables or disables the fade-complete interrupt.
                pub fn enable_fade_interrupt(&self, enable: bool) {
                    let int_ena = &self.0.regs().int_ena;
                    if enable {
                        int_ena.[<set_duty_chng_end_ch $n _int_ena>]();
                    } else {
                        int_ena.[<clr_duty_chng_end_ch $n _int_ena>]();
                    }
                }
            }
        }
    };
}

timer_body!(0);
timer_body!(1);
timer_body!(2);
timer_body!(3);

channel_body!(0);
channel_body!(1);
channel_body!(2);
channel_body!(3);
channel_body!(4);
channel_body!(5);
channel_body!(6);
channel_body!(7);
