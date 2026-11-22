//! Remote control transceiver (RMT).
//!
//! Eight channels share one 384-word RAM of 16-bit pulse codes: channels 0–3
//! transmit, channels 4–7 receive. A pulse code is a level plus a duration in
//! channel clock ticks, and a duration of zero terminates a sequence.
//!
//! Both directions are GPIO-matrix signals (`RMT_SIG_PAD_OUT0..3`,
//! `RMT_SIG_PAD_IN0..3`), so any GPIO can carry them.
//!
//! # Features
//!
//! - TX: pulse sequences up to the channel's RAM (96 codes per block), one to
//!   four blocks, idle level selection, carrier modulation, continuous mode,
//!   and the transmit-end interrupt
//! - RX: one to four blocks, idle threshold, glitch filter, and the
//!   receive-end interrupt
//! - Group clock: `XTAL_40M` / `RC_FAST` / `PLL_F80M` with an integer divider,
//!   plus a per-channel divider
//!
//! The driver writes and reads the pulse RAM directly (non-FIFO mode, the path
//! ESP-IDF uses); the GDMA path of channels 3 and 7 is not wired up.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
//! use esp32p4_hal::gpio::{Level, Pins};
//! use esp32p4_hal::rmt::{ClockSource, Pulse, Rmt, RxConfig, TxConfig};
//!
//! let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
//! rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1).unwrap(); // 40 MHz ticks
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT0);
//! pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);
//!
//! // The channel number lives in the handle's type.
//! let tx = rmt.tx_channel::<0>();
//! let rx = rmt.rx_channel::<0>();
//! tx.configure(&TxConfig::default()).unwrap();
//! rx.configure(&RxConfig::default()).unwrap();
//!
//! // Both ends share the pad, so the example sends and listens at once.
//! let mut received = [Pulse::default(); 64];
//! rx.receive(&mut received, 10_000).unwrap();
//! tx.send(
//!     &[Pulse::new(Level::High, 10), Pulse::new(Level::Low, 30)],
//!     10_000,
//! )
//! .unwrap();
//! ```
//!
//! # Hardware Notes
//!
//! - Configuration registers only reach the channel after `CONF_UPDATE_CHn/m`;
//!   the driver writes it on every configuration change.
//! - The pulse RAM is gated by the PMU by default. [`Rmt::init`] forces it on
//!   (`SYS_CONF.MEM_FORCE_PU`), which is what ESP-IDF does as well.
//! - Every non-zero duration must be longer than
//!   `5 * APB period + 6 * RMT clock period` (TRM equation 60.1), so pick the
//!   channel divider with that floor in mind.
//! - `SYS_CONF` has a second clock mux in front of the PCR source. Leaving it
//!   at its reset value (the 80 MHz node) makes the tick rate depend on a clock
//!   that does not match `RMT_CLK_SRC_SEL`; [`Rmt::set_clock_source`] programs
//!   both.
//! - Receive carrier demodulation is enabled out of reset. With the default
//!   thresholds it removes every input pulse, so the receiver reports the idle
//!   gap without ever recording an edge; `RxChannel::configure` turns it off.
//! - The last pulse of a burst is usually lost: the receiver is still counting
//!   it when the idle gap ends the reception.
//! - The reset SVD puts RMT at `0x500D_4000`, which the TRM marks as reserved;
//!   the block is at `0x500A_2000`.

use esp32p4_pac::{HP_SYS_CLKRST, RMT, rmt};

pub use crate::gpio::Level;

/// Pulse codes in one RAM block.
pub const RAM_WORDS_PER_CHANNEL: usize = 48;

/// Pulse codes one RAM block holds (two per 32-bit word).
pub const PULSES_PER_BLOCK: usize = RAM_WORDS_PER_CHANNEL * 2;

/// Bit of a transmit channel's `CHnCONF0` that puts the channel's RAM behind
/// the GDMA interface.
///
/// Only transmit channel 3 has it (TRM 58.3.2.6), and the PAC's SVD predates
/// the field, so it is written by hand.
const TX_DMA_ACCESS_EN: u32 = 1 << 25;

/// The same switch for a receive channel, in `CHmCONF0`; only the fourth
/// receive channel — hardware channel 7 — has it.
const RX_DMA_ACCESS_EN: u32 = 1 << 23;

/// Clock feeding the group divider.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// `XTAL_CLK`, 40 MHz.
    #[default]
    Xtal40m,
    /// `RC_FAST_CLK`.
    RcFast,
    /// `PLL_F80M_CLK`, 80 MHz.
    PllF80m,
}

impl ClockSource {
    /// Value of `HP_SYS_CLKRST.PERI_CLK_CTRL22.REG_RMT_CLK_SRC_SEL`.
    const fn pcr_field(self) -> u32 {
        match self {
            Self::Xtal40m => 0,
            Self::RcFast => 1,
            Self::PllF80m => 2,
        }
    }

    /// Value of `RMT.SYS_CONF.SCLK_SEL`, which feeds the same clock into the
    /// peripheral's own mux; the two must agree.
    const fn sclk_field(self) -> u32 {
        match self {
            Self::Xtal40m => 3,
            Self::RcFast => 2,
            Self::PllF80m => 1,
        }
    }
}

/// One pulse code: a level held for `period` channel clock ticks.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pulse {
    /// Level driven (TX) or sampled (RX).
    pub level: Level,
    /// Duration in channel ticks, 1–32767. Zero marks the end of a sequence.
    pub period: u16,
}

impl Pulse {
    /// A pulse of `level` lasting `period` ticks.
    pub const fn new(level: Level, period: u16) -> Self {
        Self { level, period }
    }

    /// The end-of-sequence marker.
    pub const END: Self = Self::new(Level::Low, 0);

    /// Whether this pulse ends the sequence.
    pub const fn is_end(self) -> bool {
        self.period == 0
    }

    /// The 16-bit code stored in RAM.
    pub const fn code(self) -> u16 {
        (self.period & 0x7fff) | ((self.level as u16) << 15)
    }

    /// Rebuild a code read back from RAM.
    pub const fn from_code(code: u16) -> Self {
        Self {
            level: if code & 0x8000 != 0 {
                Level::High
            } else {
                Level::Low
            },
            period: code & 0x7fff,
        }
    }
}

/// One 16-bit pulse code, the unit that actually lives in the channel RAM.
///
/// The APB path packs two of these into a 32-bit word, and GDMA moves those
/// words verbatim, so a DMA buffer is a plain array of [`Code`] — two per
/// 32-bit word — and not an array of [`Pulse`], which is four bytes of
/// `level`/`period` instead of one packed code.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Code(u16);

impl Code {
    /// The end-of-sequence marker.
    pub const END: Self = Self(0);

    /// The code for `pulse`.
    #[inline]
    pub const fn from_pulse(pulse: Pulse) -> Self {
        Self(pulse.code())
    }

    /// The pulse `code` decodes to.
    #[inline]
    pub const fn pulse(self) -> Pulse {
        Pulse::from_code(self.0)
    }

    /// The raw 16-bit value.
    #[inline]
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Whether this code ends the sequence.
    #[inline]
    pub const fn is_end(self) -> bool {
        (self.0 & 0x7fff) == 0
    }
}

impl From<Pulse> for Code {
    #[inline]
    fn from(pulse: Pulse) -> Self {
        Self::from_pulse(pulse)
    }
}

impl From<Code> for Pulse {
    #[inline]
    fn from(code: Code) -> Self {
        code.pulse()
    }
}

/// Carrier modulation for a transmit channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Carrier {
    /// Ticks the carrier stays high in one carrier period, minus one.
    pub high: u16,
    /// Ticks the carrier stays low in one carrier period, minus one.
    pub low: u16,
    /// Modulate the high level (`true`) or the low level (`false`).
    pub on_high: bool,
    /// Only modulate while a pulse is being sent, instead of all the time.
    pub only_while_active: bool,
}

impl Carrier {
    /// A 38 kHz, 50 % carrier at 40 MHz ticks (13 ticks per half period).
    pub const fn rc_38khz() -> Self {
        Self {
            high: 12,
            low: 12,
            on_high: true,
            only_while_active: true,
        }
    }
}

/// Static configuration of a transmit channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxConfig {
    /// Per-channel divider: channel clock = group clock / divider, 1–256.
    pub divider: u16,
    /// Level driven while the channel is idle.
    pub idle_level: Level,
    /// Take the idle level from [`idle_level`](Self::idle_level) (`true`) or
    /// from the level of the end marker (`false`).
    pub idle_output: bool,
    /// Number of RAM blocks the channel owns, 1–4.
    pub blocks: u8,
    /// Optional carrier modulation.
    pub carrier: Option<Carrier>,
}

impl TxConfig {
    /// No carrier, idle low, one block.
    pub const fn new(divider: u16) -> Self {
        Self {
            divider,
            idle_level: Level::Low,
            idle_output: true,
            blocks: 1,
            carrier: None,
        }
    }
}

impl Default for TxConfig {
    fn default() -> Self {
        Self::new(1)
    }
}

/// Static configuration of a receive channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RxConfig {
    /// Per-channel divider: channel clock = group clock / divider, 1–256.
    pub divider: u16,
    /// Ticks of unchanged level that end a reception.
    pub idle_threshold: u16,
    /// Number of RAM blocks the channel owns, 1–4.
    pub blocks: u8,
    /// Drop pulses shorter than this many ticks.
    pub filter_threshold: Option<u8>,
}

impl RxConfig {
    /// One block, no filter.
    pub const fn new(divider: u16, idle_threshold: u16) -> Self {
        Self {
            divider,
            idle_threshold,
            blocks: 1,
            filter_threshold: None,
        }
    }
}

impl Default for RxConfig {
    fn default() -> Self {
        Self::new(1, 0x7fff)
    }
}

/// Errors from the configuration and sequence helpers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The sequence does not fit the channel's RAM, including the end marker.
    TooLong,
    /// The group divider is outside 1–256, or a channel divider is zero.
    Divider,
    /// More RAM blocks were requested than the channel can own.
    Blocks,
    /// The transfer did not finish before the deadline.
    Timeout,
    /// A DMA sequence has no end marker, so the transmitter would never stop.
    NoEnd,
    /// The descriptors do not cover the whole buffer.
    Descriptors,
}

/// RMT controller owning the PAC peripheral.
pub struct Rmt {
    rmt: RMT,
}

impl Rmt {
    /// Initialises the controller: bus clock, group clock, reset release, and
    /// forces the pulse RAM on.
    pub fn init(rmt: RMT, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl2.set_reg_rmt_sys_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_rmt();
        clkrst.hp_rst_en1.clr_reg_rst_en_rmt();
        clkrst.peri_clk_ctrl22.set_reg_rmt_clk_en();

        let self_ = Self { rmt };
        // The PMU may gate the RAM; forcing it on is what ESP-IDF does before
        // touching any pulse, and ESP-IDF accesses the RAM directly rather than
        // through the APB FIFO, so the driver follows the same path.
        self_.rmt.sys_conf.set_mem_force_pu();
        self_.rmt.sys_conf.clr_mem_force_pd();
        self_.rmt.sys_conf.set_mem_clk_force_on();
        self_.rmt.sys_conf.set_clk_en();
        self_.rmt.sys_conf.set_apb_fifo_mask();
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
            rmt: unsafe { RMT::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &rmt::RegisterBlock {
        &self.rmt
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> RMT {
        self.rmt
    }

    /// Selects the group clock source and its integer divider.
    ///
    /// `rmt_sclk = source / divider`; every channel divides it further with its
    /// own `divider`.
    ///
    /// # Errors
    ///
    /// [`Error::Divider`] when `divider` is outside 1–256.
    pub fn set_clock_source(
        &self,
        clkrst: &HP_SYS_CLKRST,
        source: ClockSource,
        divider: u32,
    ) -> Result<(), Error> {
        if !(1..=256).contains(&divider) {
            return Err(Error::Divider);
        }
        // The register holds `divider - 1`; the fractional part is 0/1.
        clkrst
            .peri_clk_ctrl22
            .write_reg_rmt_clk_div_num(divider - 1);
        clkrst.peri_clk_ctrl22.write_reg_rmt_clk_div_numerator(0);
        clkrst.peri_clk_ctrl22.write_reg_rmt_clk_div_denominator(1);
        clkrst
            .peri_clk_ctrl22
            .write_reg_rmt_clk_src_sel(source.pcr_field());

        // `SYS_CONF` has a second mux and divider in front of the same clock.
        // Leaving them at their reset (`SCLK_SEL = 1`, the 80 MHz node) makes
        // the peripheral run from a clock that does not match the PCR source,
        // which shows up as a tick rate far below the configured one.
        let sys = &self.rmt.sys_conf;
        sys.write_sclk_sel(source.sclk_field());
        sys.write_sclk_div_num(0);
        sys.write_sclk_div_a(0);
        sys.write_sclk_div_b(0);
        sys.set_sclk_active();
        Ok(())
    }

    /// A handle to transmit channel `N`.
    ///
    /// `N` is 0–3; anything else fails to compile.
    #[inline]
    pub fn tx_channel<const N: u8>(&self) -> TxChannel<'_, N> {
        const { assert!(N < 4, "this controller has four transmit channels, 0 to 3") };
        TxChannel(self)
    }

    /// A handle to receive channel `N`.
    ///
    /// `N` is 0–3; anything else fails to compile.
    #[inline]
    pub fn rx_channel<const N: u8>(&self) -> RxChannel<'_, N> {
        const { assert!(N < 4, "this controller has four receive channels, 0 to 3") };
        RxChannel(self)
    }
}

/// One transmit channel, `N` of four.
///
/// The handle carries the channel number in its type, so two channels cannot be
/// mixed up; the register names behind it are literals generated once per
/// channel. Transmit channel `N` owns RAM block `N`, so a channel that asks for
/// more than one block (`mem_size`) takes the following ones too and a
/// transmitting channel never overlaps a receiving one on this chip.
pub struct TxChannel<'a, const N: u8>(&'a Rmt);

/// One receive channel, `N` of four, on hardware channels 4–7.
pub struct RxChannel<'a, const N: u8>(&'a Rmt);

/// Generates a transmit channel's methods.
///
/// The second argument is the channel's number in the *interrupt bit table*,
/// which is not the same as its index: the four transmit channels occupy bits
/// 0–3, mirroring the register block where every one of their accessors is
/// called `…_ch0` because the register *type* already carries the channel.
macro_rules! tx_body {
    ($n:literal) => {
        pastey::paste! {
            impl<'a> TxChannel<'a, $n> {
                /// Applies `config` to this channel.
                ///
                /// # Errors
                ///
                /// [`Error::Divider`] when the channel divider is zero,
                /// [`Error::Blocks`] when `blocks` is more than the channel can
                /// own: channel 0 owns four blocks, channel 3 only one.
                pub fn configure(&self, config: &TxConfig) -> Result<(), Error> {
                    if config.divider == 0 {
                        return Err(Error::Divider);
                    }
                    const USABLE: u8 = 4 - $n;
                    if config.blocks == 0 || config.blocks > USABLE {
                        return Err(Error::Blocks);
                    }
                    let regs = self.0.regs();
                    regs.ref_cnt_rst.[<trigger_tx_ref_cnt_rst_ch $n>]();
                    let conf = &regs.[<tx_ch $n conf0>];
                    if matches!(config.idle_level, Level::High) {
                        conf.[<set_idle_out_lv_ch0>]();
                    } else {
                        conf.[<clr_idle_out_lv_ch0>]();
                    }
                    if config.idle_output {
                        conf.[<set_idle_out_en_ch0>]();
                    } else {
                        conf.[<clr_idle_out_en_ch0>]();
                    }
                    // A divider of 256 is written as zero.
                    conf.[<write_div_cnt_ch0>](if config.divider >= 256 {
                        0
                    } else {
                        u32::from(config.divider)
                    });
                    conf.[<write_mem_size_ch0>](u32::from(config.blocks));
                    match config.carrier {
                        Some(carrier) => {
                            if carrier.on_high {
                                conf.[<set_carrier_out_lv_ch0>]();
                            } else {
                                conf.[<clr_carrier_out_lv_ch0>]();
                            }
                            if carrier.only_while_active {
                                conf.[<set_carrier_eff_en_ch0>]();
                            } else {
                                conf.[<clr_carrier_eff_en_ch0>]();
                            }
                            conf.[<set_carrier_en_ch0>]();
                        }
                        None => conf.[<clr_carrier_en_ch0>](),
                    }
                    conf.[<trigger_conf_update_ch0>]();
                    if let Some(carrier) = config.carrier {
                        self.set_carrier_duty(carrier.high, carrier.low);
                    }
                    Ok(())
                }

                /// Sets the carrier's high and low durations, in ticks minus one.
                pub fn set_carrier_duty(&self, high: u16, low: u16) {
                    let duty = &self.0.regs().[<ch $n carrier_duty>];
                    duty.write_carrier_high_ch(u32::from(high));
                    duty.write_carrier_low_ch(u32::from(low));
                }

                /// Writes a pulse sequence into RAM and starts it.
                ///
                /// An end marker is appended when `pulses` does not end with
                /// one, so at most `capacity()` pulses fit. The call returns once
                /// the transmitter is running; use
                /// [`wait_done`](Self::wait_done) for the end.
                ///
                /// # Errors
                ///
                /// [`Error::TooLong`] when the sequence does not fit the
                /// configured blocks.
                pub fn transmit(&self, pulses: &[Pulse]) -> Result<(), Error> {
                    self.write_pulses(pulses)?;
                    self.start();
                    Ok(())
                }

                /// Loads a pulse sequence without starting the transmitter.
                /// Returns how many codes were written, end marker included.
                ///
                /// # Errors
                ///
                /// [`Error::TooLong`] when the sequence does not fit the
                /// configured blocks.
                pub fn write_pulses(&self, pulses: &[Pulse]) -> Result<usize, Error> {
                    let capacity = self.capacity();
                    let mut count = pulses.len();
                    if count == 0 || !pulses[count - 1].is_end() {
                        count += 1;
                    }
                    if count > capacity {
                        return Err(Error::TooLong);
                    }

                    let regs = self.0.regs();
                    regs.ref_cnt_rst.[<trigger_tx_ref_cnt_rst_ch $n>]();
                    let conf = &regs.[<tx_ch $n conf0>];
                    conf.[<trigger_mem_rd_rst_ch0>]();
                    conf.[<trigger_apb_mem_rst_ch0>]();
                    // SAFETY: the pointer is the base of this channel's pulse
                    // RAM, and `index` stays below `capacity`.
                    let ram = &regs.[<tx_ch $n _mem0>] as *const _
                        as *const crate::pac::common::VolatileCell<u32>;
                    let code = |index: usize| {
                        if index < pulses.len() {
                            pulses[index].code()
                        } else {
                            Pulse::END.code()
                        }
                    };
                    let mut index = 0;
                    while index < count {
                        unsafe {
                            (&*ram.add(index / 2))
                                .write_bits(u32::from(code(index)) | (u32::from(code(index + 1)) << 16));
                        }
                        index += 2;
                    }
                    Ok(count)
                }

                /// Starts transmitting what is already in RAM.
                pub fn start(&self) {
                    let conf = &self.0.regs().[<tx_ch $n conf0>];
                    conf.[<trigger_conf_update_ch0>]();
                    conf.[<trigger_tx_start_ch0>]();
                }

                /// Stops the transmitter immediately.
                pub fn stop(&self) {
                    let conf = &self.0.regs().[<tx_ch $n conf0>];
                    conf.[<set_tx_stop_ch0>]();
                    conf.[<trigger_conf_update_ch0>]();
                }

                /// Enables continuous transmission: the RAM is sent over and over.
                pub fn set_conti_mode(&self, enable: bool) {
                    let conf = &self.0.regs().[<tx_ch $n conf0>];
                    if enable {
                        conf.[<set_tx_conti_mode_ch0>]();
                    } else {
                        conf.[<clr_tx_conti_mode_ch0>]();
                    }
                    conf.[<trigger_conf_update_ch0>]();
                }

                /// Whether the transmitter has finished the sequence.
                pub fn done(&self) -> bool {
                    self.0.regs().int_raw.[<ch $n _tx_end_int_raw>]()
                }

                /// Clears the transmit-end status.
                pub fn clear_done(&self) {
                    self.0
                        .regs()
                        .int_clr
                        .[<trigger_ch $n _tx_end_int_clr>]();
                }

                /// Enables or disables the transmit-end interrupt.
                pub fn enable_end_interrupt(&self, enable: bool) {
                    let int_ena = &self.0.regs().int_ena;
                    if enable {
                        int_ena.[<set_ch $n _tx_end_int_ena>]();
                    } else {
                        int_ena.[<clr_ch $n _tx_end_int_ena>]();
                    }
                }

                /// Spins until the sequence has been sent.
                ///
                /// # Errors
                ///
                /// [`Error::Timeout`] when the transmitter is still running
                /// after `timeout_us` microseconds.
                pub fn wait_done(&self, timeout_us: u64) -> Result<(), Error> {
                    wait_until(timeout_us, || self.done()).ok_or(Error::Timeout)
                }

                /// Sends `pulses` and waits for the transmitter to finish.
                ///
                /// # Errors
                ///
                /// The same as [`transmit`](Self::transmit) and
                /// [`wait_done`](Self::wait_done).
                pub fn send(&self, pulses: &[Pulse], timeout_us: u64) -> Result<(), Error> {
                    self.clear_done();
                    self.transmit(pulses)?;
                    self.wait_done(timeout_us)
                }

                /// Pulse codes the configured blocks can hold.
                pub fn capacity(&self) -> usize {
                    let blocks = self.0.regs().[<tx_ch $n conf0>].[<mem_size_ch0>]();
                    if blocks == 0 {
                        PULSES_PER_BLOCK
                    } else {
                        blocks as usize * PULSES_PER_BLOCK
                    }
                }

                /// The transmitter's state-machine value; zero means idle.
                pub fn state(&self) -> u32 {
                    self.0.regs().[<tx_ch $n status>].[<state_ch0>]()
                }

                /// Whether the channel's RAM is empty.
                pub fn memory_empty(&self) -> bool {
                    self.0.regs().[<tx_ch $n status>].[<mem_empty_ch0>]()
                }
            }
        }
    };
}

/// Generates a receive channel's methods.
///
/// The receive channels sit at interrupt bits 4–7, which is why this takes the
/// hardware number as a second argument.
macro_rules! rx_body {
    ($n:literal, $h:literal) => {
        pastey::paste! {
            impl<'a> RxChannel<'a, $n> {
                /// Applies `config` to this channel.
                ///
                /// # Errors
                ///
                /// [`Error::Divider`] when the channel divider is zero,
                /// [`Error::Blocks`] when `blocks` is more than the channel can
                /// own: channel 0 owns four blocks, channel 3 only one.
                pub fn configure(&self, config: &RxConfig) -> Result<(), Error> {
                    if config.divider == 0 {
                        return Err(Error::Divider);
                    }
                    const USABLE: u8 = 4 - $n;
                    if config.blocks == 0 || config.blocks > USABLE {
                        return Err(Error::Blocks);
                    }
                    let regs = self.0.regs();
                    regs.ref_cnt_rst.[<trigger_rx_ref_cnt_rst_ch $h>]();
                    let conf0 = &regs.[<rx_ch $n conf0>];
                    // Carrier demodulation is on out of reset, and with the
                    // default thresholds it strips every pulse that is not a
                    // carrier, leaving the receiver with no edges at all.
                    conf0.[<clr_carrier_en_ch4>]();
                    conf0.[<write_div_cnt_ch4>](if config.divider >= 256 {
                        0
                    } else {
                        u32::from(config.divider)
                    });
                    conf0.[<write_idle_thres_ch4>](u32::from(config.idle_threshold));
                    conf0.[<write_mem_size_ch4>](u32::from(config.blocks));

                    let conf1 = &regs.[<rx_ch $n conf1>];
                    match config.filter_threshold {
                        Some(threshold) => {
                            conf1.[<write_rx_filter_thres_ch4>](u32::from(threshold));
                            conf1.[<set_rx_filter_en_ch4>]();
                        }
                        None => conf1.[<clr_rx_filter_en_ch4>](),
                    }
                    conf1.[<trigger_conf_update_ch4>]();
                    Ok(())
                }

                /// Starts receiving. The channel stops on its own once the input
                /// stays at one level for longer than the idle threshold.
                pub fn start(&self) {
                    let conf1 = &self.0.regs().[<rx_ch $n conf1>];
                    conf1.[<trigger_mem_wr_rst_ch4>]();
                    conf1.[<trigger_apb_mem_rst_ch4>]();
                    // The receiver can only write the RAM while it owns it; APB
                    // reads and receiver writes must not overlap.
                    conf1.[<set_mem_owner_ch4>]();
                    conf1.[<set_rx_en_ch4>]();
                    conf1.[<trigger_conf_update_ch4>]();
                }

                /// Stops receiving.
                pub fn stop(&self) {
                    let conf1 = &self.0.regs().[<rx_ch $n conf1>];
                    conf1.[<clr_rx_en_ch4>]();
                    conf1.[<trigger_conf_update_ch4>]();
                }

                /// Whether a reception has finished.
                pub fn done(&self) -> bool {
                    self.0.regs().int_raw.[<ch $h _rx_end_int_raw>]()
                }

                /// Clears the receive-end status.
                pub fn clear_done(&self) {
                    self.0
                        .regs()
                        .int_clr
                        .[<trigger_ch $h _rx_end_int_clr>]();
                }

                /// Enables or disables the receive-end interrupt.
                pub fn enable_end_interrupt(&self, enable: bool) {
                    let int_ena = &self.0.regs().int_ena;
                    if enable {
                        int_ena.[<set_ch $h _rx_end_int_ena>]();
                    } else {
                        int_ena.[<clr_ch $h _rx_end_int_ena>]();
                    }
                }

                /// Number of pulse codes the last reception left in RAM.
                ///
                /// It comes from the write cursor minus the channel's block
                /// start, so it counts codes, not pairs, and can include a
                /// trailing end marker.
                pub fn count(&self) -> usize {
                    let cursor = self.0.regs().[<rx_ch $n status>].[<mem_waddr_ex_ch4>]() as usize;
                    let start = ($h as usize) * RAM_WORDS_PER_CHANNEL;
                    cursor.saturating_sub(start) * 2
                }

                /// Drains the received codes into `buffer`, stopping at an end
                /// marker or when the buffer is full. Returns how many pulses
                /// were stored.
                pub fn read_pulses(&self, buffer: &mut [Pulse]) -> usize {
                    let regs = self.0.regs();
                    // Hand the RAM back to the APB side before reading it.
                    regs.[<rx_ch $n conf1>].[<clr_mem_owner_ch4>]();
                    let mut written = 0;
                    let count = self.count().min(buffer.len());
                    // SAFETY: the cursor never exceeds the channel's blocks, so
                    // `written / 2` stays inside its RAM.
                    let ram = &regs.[<rx_ch $n _mem0>] as *const _
                        as *const crate::pac::common::VolatileCell<u32>;
                    while written < count {
                        let word = unsafe { (&*ram.add(written / 2)).read_bits() };
                        for code in [word as u16, (word >> 16) as u16] {
                            if written >= count || written >= buffer.len() {
                                return written;
                            }
                            let pulse = Pulse::from_code(code);
                            if pulse.is_end() {
                                return written;
                            }
                            buffer[written] = pulse;
                            written += 1;
                        }
                    }
                    written
                }

                /// Receives one burst into `buffer`: starts the channel, waits
                /// for the idle threshold to end it, then drains the RAM.
                ///
                /// # Errors
                ///
                /// [`Error::Timeout`] when the channel is still receiving after
                /// `timeout_us` microseconds.
                pub fn receive(&self, buffer: &mut [Pulse], timeout_us: u64) -> Result<usize, Error> {
                    self.receive_with(buffer, timeout_us, || Ok(()))
                }

                /// Like [`receive`](Self::receive), but runs `trigger` once the
                /// receiver is listening, which is what a loopback needs: the
                /// transmitter has to start after the receiver, or the first edge
                /// is missed.
                ///
                /// # Errors
                ///
                /// [`Error::Timeout`] when the channel is still receiving after
                /// `timeout_us` microseconds, plus whatever `trigger` returns.
                pub fn receive_with<F>(
                    &self,
                    buffer: &mut [Pulse],
                    timeout_us: u64,
                    trigger: F,
                ) -> Result<usize, Error>
                where
                    F: FnOnce() -> Result<(), Error>,
                {
                    self.clear_done();
                    self.start();
                    trigger()?;
                    if wait_until(timeout_us, || self.done()).is_none() {
                        self.stop();
                        return Err(Error::Timeout);
                    }
                    Ok(self.read_pulses(buffer))
                }
            }
        }
    };
}

tx_body!(0);
tx_body!(1);
tx_body!(2);
tx_body!(3);

impl<'a> TxChannel<'a, 3> {
    /// Sends `pulses` out of memory through GDMA instead of the APB FIFO.
    ///
    /// Transmit channel 3 is the only one wired to the GDMA interface, and its
    /// RAM becomes unreachable from the bus once the switch is thrown: the
    /// engine reads `pulses` and wraps around the 48-word RAM as it plays, so a
    /// sequence far longer than the RAM goes out in one call.
    ///
    /// The sequence has to end with [`Pulse::END`] — nothing appends one for
    /// us, and without it the transmitter wraps and never reports done.
    /// `descriptors` carry the buffer; [`Chain`](crate::dma::Chain) mounts one
    /// descriptor per 4095 bytes, i.e. per 1023 pulses.
    ///
    /// # Errors
    ///
    /// [`Error::NoEnd`] when `pulses` has no end marker, [`Error::Descriptors`]
    /// when the descriptors do not cover it, [`Error::Timeout`] when the
    /// transmitter has not finished after `timeout_us` microseconds.
    pub fn send_dma<T: crate::dma::TransmitChannel>(
        &self,
        tx: &T,
        codes: &[Code],
        descriptors: &[crate::dma::Descriptor],
        timeout_us: u64,
    ) -> Result<(), Error> {
        if codes.last().is_none_or(|code| !code.is_end()) {
            return Err(Error::NoEnd);
        }
        let bytes = core::mem::size_of_val(codes);
        // The engine reads memory, not the cache; a dirty line left behind
        // would be sent instead of what the caller filled in.
        crate::cache::writeback(codes.as_ptr().cast(), bytes);
        let chain = crate::dma::Chain::new(descriptors);
        chain
            .mount_eof(codes.as_ptr().cast_mut().cast(), bytes)
            .map_err(|_| Error::Descriptors)?;

        let conf = &self.0.regs().tx_ch3conf0;
        self.clear_done();
        let bits = conf.read_bits();
        conf.write_bits(bits | TX_DMA_ACCESS_EN);
        conf.set_mem_tx_wrap_en_ch0();
        conf.trigger_conf_update_ch0();
        self.0.regs().ch3_tx_lim.write_tx_lim_ch(0);

        tx.connect_to(crate::dma::Peripheral::Rmt);
        tx.reset_channel();
        tx.clear_all_interrupts();
        tx.start_chain(chain.head());
        // The transmitter has to find data in its RAM, or it plays whatever the
        // reset left there: wait for the engine to hand over the first block.
        let filled = wait_until(timeout_us, || !self.memory_empty()).is_some();
        self.start();
        let result = if filled {
            self.wait_done(timeout_us)
        } else {
            Err(Error::Timeout)
        };
        tx.stop_chain();

        let bits = conf.read_bits();
        conf.write_bits(bits & !TX_DMA_ACCESS_EN);
        conf.clr_mem_tx_wrap_en_ch0();
        conf.trigger_conf_update_ch0();
        self.0.regs().ch3_tx_lim.write_tx_lim_ch(128);
        result
    }
}

rx_body!(0, 4);
rx_body!(1, 5);
rx_body!(2, 6);
rx_body!(3, 7);

impl<'a> RxChannel<'a, 3> {
    /// Receives into `buffer` through GDMA instead of the APB FIFO.
    ///
    /// Receive channel 3 — hardware channel 7 — is the only one wired to the
    /// GDMA interface. The engine writes the received codes into `buffer` and
    /// the RAM wraps underneath, so a burst longer than 48 words arrives in one
    /// go; the transfer ends when the receiver sees an end marker, which is
    /// what raises the channel's EOF on the DMA side.
    ///
    /// Returns how many pulses were stored. The end marker itself is a
    /// half-word on the wire, so it lands in `buffer` truncated rather than
    /// dropped; compare against a sequence that carries one.
    ///
    /// # Errors
    ///
    /// [`Error::Descriptors`] when the descriptors do not cover `buffer`,
    /// [`Error::Timeout`] when nothing ended the reception within `timeout_us`
    /// microseconds.
    pub fn receive_dma<R: crate::dma::ReceiveChannel>(
        &self,
        rx: &R,
        buffer: &mut [Code],
        descriptors: &[crate::dma::Descriptor],
        timeout_us: u64,
    ) -> Result<usize, Error> {
        self.receive_dma_with(rx, buffer, descriptors, timeout_us, || Ok(()))
    }

    /// Like [`receive_dma`](Self::receive_dma), but runs `trigger` once the
    /// receiver is listening — a loopback needs the transmitter to start after
    /// it, or the first edge is missed.
    ///
    /// # Errors
    ///
    /// The same as [`receive_dma`](Self::receive_dma), plus whatever `trigger`
    /// returns.
    pub fn receive_dma_with<R, F>(
        &self,
        rx: &R,
        buffer: &mut [Code],
        descriptors: &[crate::dma::Descriptor],
        timeout_us: u64,
        trigger: F,
    ) -> Result<usize, Error>
    where
        R: crate::dma::ReceiveChannel,
        F: FnOnce() -> Result<(), Error>,
    {
        let bytes = core::mem::size_of_val(buffer);
        let chain = crate::dma::Chain::new(descriptors);
        chain
            .mount(buffer.as_mut_ptr().cast(), bytes)
            .map_err(|_| Error::Descriptors)?;
        // The engine writes memory behind the cache's back, in both
        // directions: push out what we hold, then read the result fresh.
        crate::cache::writeback_invalidate(buffer.as_mut_ptr().cast(), bytes);

        let conf0 = &self.0.regs().rx_ch3conf0;
        let bits = conf0.read_bits();
        conf0.write_bits(bits | RX_DMA_ACCESS_EN);
        let conf1 = &self.0.regs().rx_ch3conf1;
        conf1.set_mem_rx_wrap_en_ch4();
        conf1.trigger_conf_update_ch4();
        // Ping-pong threshold: the whole point of the DMA path is that the
        // engine keeps draining the RAM, so there is no limit to stop at. It
        // goes back to its reset value once the transfer is over, because the
        // APB path relies on it.
        self.0.regs().ch3_rx_lim.write_rx_lim_ch4(0);

        rx.connect_to(crate::dma::Peripheral::Rmt);
        rx.reset_channel();
        rx.clear_all_interrupts();
        rx.start_chain(chain.head());
        self.clear_done();
        let exchange = (|| -> Result<(), Error> {
            self.start();
            trigger()?;
            wait_until(timeout_us, || {
                rx.interrupt_pending(crate::dma::RxInterrupt::SucEof)
            })
            .ok_or(Error::Timeout)?;
            Ok(())
        })();
        self.stop();
        rx.stop_chain();

        let bits = conf0.read_bits();
        conf0.write_bits(bits & !RX_DMA_ACCESS_EN);
        conf1.clr_mem_rx_wrap_en_ch4();
        conf1.trigger_conf_update_ch4();
        self.0.regs().ch3_rx_lim.write_rx_lim_ch4(128);
        exchange?;

        // The engine rewrites each finished descriptor's length with what it
        // actually received, so the total is the sum up to the EOF one.
        let last = rx.last_eof_descriptor();
        let mut received = 0;
        let mut current: *const crate::dma::Descriptor = chain.head();
        loop {
            // SAFETY: every step follows a `next` pointer the chain mounted
            // from the caller's own descriptors.
            let descriptor = unsafe { &*current };
            received += descriptor.length();
            let done = core::ptr::eq(current, last);
            match descriptor.next() {
                Some(next) if !done => current = next,
                _ => break,
            }
        }
        crate::cache::invalidate(buffer.as_mut_ptr().cast(), received.min(bytes));
        Ok(received / core::mem::size_of::<Code>())
    }
}

fn wait_until(timeout_us: u64, mut done: impl FnMut() -> bool) -> Option<()> {
    let deadline = crate::delay::micros().wrapping_add(timeout_us);
    while !done() {
        // Wrapping arithmetic keeps the comparison valid across the counter's
        // wrap; half the range is the "already past due" boundary.
        if crate::delay::micros().wrapping_sub(deadline) < u64::MAX / 2 {
            return None;
        }
    }
    Some(())
}
