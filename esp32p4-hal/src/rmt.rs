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
//! use esp32p4_hal::rmt::{
//!     ClockSource, Pulse, Rmt, RxChannelIndex, RxConfig, TxChannelIndex, TxConfig,
//! };
//!
//! let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
//! rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1).unwrap(); // 40 MHz ticks
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT0);
//! pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);
//!
//! rmt.configure_tx(TxChannelIndex::Zero, &TxConfig::default()).unwrap();
//! rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::default()).unwrap();
//!
//! // Both ends share the pad, so the example sends and listens at once.
//! let mut received = [Pulse::default(); 64];
//! rmt.receive(RxChannelIndex::Zero, &mut received, 10_000).unwrap();
//! rmt.send(
//!     TxChannelIndex::Zero,
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
//!   gap without ever recording an edge; [`Rmt::configure_rx`] turns it off.
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

/// A transmit channel: hardware channels 0–3.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxChannelIndex {
    /// Hardware channel 0.
    Zero,
    /// Hardware channel 1.
    One,
    /// Hardware channel 2.
    Two,
    /// Hardware channel 3, the one with GDMA access.
    Three,
}

/// A receive channel: hardware channels 4–7, numbered 0–3 here.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxChannelIndex {
    /// Hardware channel 4.
    Zero,
    /// Hardware channel 5.
    One,
    /// Hardware channel 6.
    Two,
    /// Hardware channel 7, the one with GDMA access.
    Three,
}

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
    const fn code(self) -> u32 {
        (self.period as u32 & 0x7fff) | ((self.level as u32) << 15)
    }

    /// Rebuild a code read back from RAM.
    const fn from_code(code: u16) -> Self {
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
}

// ============================================================================
// Per-channel register access
// ============================================================================
//
// Every channel has its own register types (`TxCh0conf0` vs `TxCh1conf0`, …)
// with the channel number baked into each field accessor name. These traits
// give the driver one name per operation; the macro below implements them for
// all four channels.

trait TxConfRegs {
    fn set_idle(&self, high: bool, from_register: bool);
    fn set_divider(&self, divider: u16);
    fn set_blocks(&self, blocks: u8);
    fn set_carrier(&self, carrier: Option<Carrier>);
    fn set_conti_mode(&self, enable: bool);
    fn start(&self);
    fn stop(&self);
    fn reset_pointers(&self);
    fn update(&self);
    fn blocks(&self) -> u32;
}

trait TxStatusRegs {
    fn state(&self) -> u32;
    fn memory_empty(&self) -> bool;
}

trait RxConf0Regs {
    fn set_carrier_demodulation(&self, enable: bool);
    fn set_divider(&self, divider: u16);
    fn set_idle_threshold(&self, threshold: u16);
    fn set_blocks(&self, blocks: u8);
}

trait RxConf1Regs {
    fn set_mem_owner(&self, hardware: bool);
    fn set_filter(&self, threshold: Option<u8>);
    fn reset_pointers(&self);
    fn set_enabled(&self, enable: bool);
    fn update(&self);
}

trait RxStatusRegs {
    fn write_cursor(&self) -> u32;
}

macro_rules! impl_channel_regs {
    ($tx_conf:ident, $tx_status:ident, $rx_conf0:ident, $rx_conf1:ident, $rx_status:ident) => {
        pastey::paste! {
            impl TxConfRegs for rmt::$tx_conf {
                fn set_idle(&self, high: bool, from_register: bool) {
                    if high {
                        self.[<set_idle_out_lv_ch0>]();
                    } else {
                        self.[<clr_idle_out_lv_ch0>]();
                    }
                    if from_register {
                        self.[<set_idle_out_en_ch0>]();
                    } else {
                        self.[<clr_idle_out_en_ch0>]();
                    }
                }

                fn set_divider(&self, divider: u16) {
                    // Zero means a divider of 256.
                    self.[<write_div_cnt_ch0>](if divider >= 256 {
                        0
                    } else {
                        u32::from(divider)
                    });
                }

                fn set_blocks(&self, blocks: u8) {
                    self.[<write_mem_size_ch0>](u32::from(blocks));
                }

                fn set_carrier(&self, carrier: Option<Carrier>) {
                    match carrier {
                        Some(carrier) => {
                            if carrier.on_high {
                                self.[<set_carrier_out_lv_ch0>]();
                            } else {
                                self.[<clr_carrier_out_lv_ch0>]();
                            }
                            if carrier.only_while_active {
                                self.[<set_carrier_eff_en_ch0>]();
                            } else {
                                self.[<clr_carrier_eff_en_ch0>]();
                            }
                            self.[<set_carrier_en_ch0>]();
                        }
                        None => self.[<clr_carrier_en_ch0>](),
                    }
                }

                fn set_conti_mode(&self, enable: bool) {
                    if enable {
                        self.[<set_tx_conti_mode_ch0>]();
                    } else {
                        self.[<clr_tx_conti_mode_ch0>]();
                    }
                }

                fn start(&self) {
                    self.[<trigger_conf_update_ch0>]();
                    self.[<trigger_tx_start_ch0>]();
                }

                fn stop(&self) {
                    self.[<set_tx_stop_ch0>]();
                    self.[<trigger_conf_update_ch0>]();
                }

                fn reset_pointers(&self) {
                    self.[<trigger_mem_rd_rst_ch0>]();
                    self.[<trigger_apb_mem_rst_ch0>]();
                }

                fn update(&self) {
                    self.[<trigger_conf_update_ch0>]();
                }

                fn blocks(&self) -> u32 {
                    self.[<mem_size_ch0>]()
                }
            }

            impl TxStatusRegs for rmt::$tx_status {
                fn state(&self) -> u32 {
                    self.[<state_ch0>]()
                }

                fn memory_empty(&self) -> bool {
                    self.[<mem_empty_ch0>]()
                }
            }

            impl RxConf0Regs for rmt::$rx_conf0 {
                fn set_carrier_demodulation(&self, enable: bool) {
                    if enable {
                        self.[<set_carrier_en_ch4>]();
                    } else {
                        self.[<clr_carrier_en_ch4>]();
                    }
                }

                fn set_divider(&self, divider: u16) {
                    self.[<write_div_cnt_ch4>](if divider >= 256 {
                        0
                    } else {
                        u32::from(divider)
                    });
                }

                fn set_idle_threshold(&self, threshold: u16) {
                    self.[<write_idle_thres_ch4>](u32::from(threshold));
                }

                fn set_blocks(&self, blocks: u8) {
                    self.[<write_mem_size_ch4>](u32::from(blocks));
                }
            }

            impl RxConf1Regs for rmt::$rx_conf1 {
                fn set_mem_owner(&self, hardware: bool) {
                    if hardware {
                        self.[<set_mem_owner_ch4>]();
                    } else {
                        self.[<clr_mem_owner_ch4>]();
                    }
                }

                fn set_filter(&self, threshold: Option<u8>) {
                    match threshold {
                        Some(threshold) => {
                            self.[<write_rx_filter_thres_ch4>](u32::from(threshold));
                            self.[<set_rx_filter_en_ch4>]();
                        }
                        None => self.[<clr_rx_filter_en_ch4>](),
                    }
                }

                fn reset_pointers(&self) {
                    self.[<trigger_mem_wr_rst_ch4>]();
                    self.[<trigger_apb_mem_rst_ch4>]();
                }

                fn set_enabled(&self, enable: bool) {
                    if enable {
                        self.[<set_rx_en_ch4>]();
                    } else {
                        self.[<clr_rx_en_ch4>]();
                    }
                    self.[<trigger_conf_update_ch4>]();
                }

                fn update(&self) {
                    self.[<trigger_conf_update_ch4>]();
                }
            }

            impl RxStatusRegs for rmt::$rx_status {
                fn write_cursor(&self) -> u32 {
                    self.[<mem_waddr_ex_ch4>]()
                }
            }
        }
    };
}

impl_channel_regs!(TxCh0conf0, TxCh0status, RxCh0conf0, RxCh0conf1, RxCh0status);
impl_channel_regs!(TxCh1conf0, TxCh1status, RxCh1conf0, RxCh1conf1, RxCh1status);
impl_channel_regs!(TxCh2conf0, TxCh2status, RxCh2conf0, RxCh2conf1, RxCh2status);
impl_channel_regs!(TxCh3conf0, TxCh3status, RxCh3conf0, RxCh3conf1, RxCh3status);

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

    fn tx_conf(&self, channel: TxChannelIndex) -> &dyn TxConfRegs {
        match channel {
            TxChannelIndex::Zero => &self.rmt.tx_ch0conf0,
            TxChannelIndex::One => &self.rmt.tx_ch1conf0,
            TxChannelIndex::Two => &self.rmt.tx_ch2conf0,
            TxChannelIndex::Three => &self.rmt.tx_ch3conf0,
        }
    }

    fn tx_status(&self, channel: TxChannelIndex) -> &dyn TxStatusRegs {
        match channel {
            TxChannelIndex::Zero => &self.rmt.tx_ch0status,
            TxChannelIndex::One => &self.rmt.tx_ch1status,
            TxChannelIndex::Two => &self.rmt.tx_ch2status,
            TxChannelIndex::Three => &self.rmt.tx_ch3status,
        }
    }

    /// Base of the channel's pulse RAM as a `VolatileCell` array.
    ///
    /// The SVD gives every word its own register type; the driver walks them
    /// through this pointer instead of a 48-arm match. `index` is bounded by
    /// the configured block count.
    fn tx_ram(&self, channel: TxChannelIndex) -> *const crate::pac::common::VolatileCell<u32> {
        match channel {
            TxChannelIndex::Zero => {
                &self.rmt.tx_ch0_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            TxChannelIndex::One => {
                &self.rmt.tx_ch1_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            TxChannelIndex::Two => {
                &self.rmt.tx_ch2_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            TxChannelIndex::Three => {
                &self.rmt.tx_ch3_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
        }
    }

    fn rx_ram(&self, channel: RxChannelIndex) -> *const crate::pac::common::VolatileCell<u32> {
        match channel {
            RxChannelIndex::Zero => {
                &self.rmt.rx_ch0_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            RxChannelIndex::One => {
                &self.rmt.rx_ch1_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            RxChannelIndex::Two => {
                &self.rmt.rx_ch2_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
            RxChannelIndex::Three => {
                &self.rmt.rx_ch3_mem0 as *const _ as *const crate::pac::common::VolatileCell<u32>
            }
        }
    }

    fn rx_conf0(&self, channel: RxChannelIndex) -> &dyn RxConf0Regs {
        match channel {
            RxChannelIndex::Zero => &self.rmt.rx_ch0conf0,
            RxChannelIndex::One => &self.rmt.rx_ch1conf0,
            RxChannelIndex::Two => &self.rmt.rx_ch2conf0,
            RxChannelIndex::Three => &self.rmt.rx_ch3conf0,
        }
    }

    fn rx_conf1(&self, channel: RxChannelIndex) -> &dyn RxConf1Regs {
        match channel {
            RxChannelIndex::Zero => &self.rmt.rx_ch0conf1,
            RxChannelIndex::One => &self.rmt.rx_ch1conf1,
            RxChannelIndex::Two => &self.rmt.rx_ch2conf1,
            RxChannelIndex::Three => &self.rmt.rx_ch3conf1,
        }
    }

    fn rx_status(&self, channel: RxChannelIndex) -> &dyn RxStatusRegs {
        match channel {
            RxChannelIndex::Zero => &self.rmt.rx_ch0status,
            RxChannelIndex::One => &self.rmt.rx_ch1status,
            RxChannelIndex::Two => &self.rmt.rx_ch2status,
            RxChannelIndex::Three => &self.rmt.rx_ch3status,
        }
    }

    // ========================================================================
    // Transmit
    // ========================================================================

    /// Applies `config` to a transmit channel.
    ///
    /// # Errors
    ///
    /// [`Error::Divider`] when the channel divider is zero, [`Error::Blocks`]
    /// when `blocks` is outside what the channel can own.
    pub fn configure_tx(&self, channel: TxChannelIndex, config: &TxConfig) -> Result<(), Error> {
        if config.divider == 0 {
            return Err(Error::Divider);
        }
        let usable = match channel {
            TxChannelIndex::Zero => 4,
            TxChannelIndex::One => 3,
            TxChannelIndex::Two => 2,
            TxChannelIndex::Three => 1,
        };
        if config.blocks == 0 || config.blocks > usable {
            return Err(Error::Blocks);
        }

        self.reset_tx_clock_divider(channel);
        let conf = self.tx_conf(channel);
        conf.set_idle(matches!(config.idle_level, Level::High), config.idle_output);
        conf.set_divider(config.divider);
        conf.set_blocks(config.blocks);
        conf.set_carrier(config.carrier);
        conf.update();

        if let Some(carrier) = config.carrier {
            self.set_tx_carrier_duty(channel, carrier.high, carrier.low);
        }
        Ok(())
    }

    /// Sets the carrier's high and low durations, in ticks minus one.
    pub fn set_tx_carrier_duty(&self, channel: TxChannelIndex, high: u16, low: u16) {
        match channel {
            TxChannelIndex::Zero => {
                self.rmt
                    .ch0carrier_duty
                    .write_carrier_high_ch(u32::from(high));
                self.rmt
                    .ch0carrier_duty
                    .write_carrier_low_ch(u32::from(low));
            }
            TxChannelIndex::One => {
                self.rmt
                    .ch1carrier_duty
                    .write_carrier_high_ch(u32::from(high));
                self.rmt
                    .ch1carrier_duty
                    .write_carrier_low_ch(u32::from(low));
            }
            TxChannelIndex::Two => {
                self.rmt
                    .ch2carrier_duty
                    .write_carrier_high_ch(u32::from(high));
                self.rmt
                    .ch2carrier_duty
                    .write_carrier_low_ch(u32::from(low));
            }
            TxChannelIndex::Three => {
                self.rmt
                    .ch3carrier_duty
                    .write_carrier_high_ch(u32::from(high));
                self.rmt
                    .ch3carrier_duty
                    .write_carrier_low_ch(u32::from(low));
            }
        }
    }

    /// Writes a pulse sequence into the channel's RAM and starts it.
    ///
    /// An end marker is appended when `pulses` does not end with one, so at most
    /// `blocks * 96 - 1` pulses fit. The call returns once the transmitter is
    /// running; use [`wait_tx_done`](Self::wait_tx_done) for the end.
    ///
    /// # Errors
    ///
    /// [`Error::TooLong`] when the sequence does not fit the configured blocks.
    pub fn transmit(&self, channel: TxChannelIndex, pulses: &[Pulse]) -> Result<(), Error> {
        self.write_pulses(channel, pulses)?;
        self.start_tx(channel);
        Ok(())
    }

    /// Loads a pulse sequence without starting the transmitter. Returns how
    /// many codes were written, including the end marker.
    ///
    /// # Errors
    ///
    /// [`Error::TooLong`] when the sequence does not fit the configured blocks.
    pub fn write_pulses(&self, channel: TxChannelIndex, pulses: &[Pulse]) -> Result<usize, Error> {
        let capacity = self.tx_capacity(channel);
        let mut count = pulses.len();
        if count == 0 || !pulses[count - 1].is_end() {
            count += 1;
        }
        if count > capacity {
            return Err(Error::TooLong);
        }

        self.reset_tx_clock_divider(channel);
        self.tx_conf(channel).reset_pointers();
        let ram = self.tx_ram(channel);
        let code = |index: usize| {
            if index < pulses.len() {
                pulses[index].code()
            } else {
                Pulse::END.code()
            }
        };
        let mut index = 0;
        while index < count {
            // SAFETY: `ram` is the channel's 48-word (times `blocks`) pulse
            // RAM and `index` stays below `tx_capacity`.
            unsafe {
                (&*ram.add(index / 2)).write_bits(code(index) | (code(index + 1) << 16));
            }
            index += 2;
        }
        Ok(count)
    }

    /// Starts transmitting what is already in the channel's RAM.
    pub fn start_tx(&self, channel: TxChannelIndex) {
        self.tx_conf(channel).start();
    }

    /// Stops the transmitter immediately.
    pub fn stop_tx(&self, channel: TxChannelIndex) {
        self.tx_conf(channel).stop();
    }

    /// Enables continuous transmission: the RAM is sent over and over.
    pub fn set_tx_conti_mode(&self, channel: TxChannelIndex, enable: bool) {
        let conf = self.tx_conf(channel);
        conf.set_conti_mode(enable);
        conf.update();
    }

    /// Whether the transmitter has finished the sequence.
    pub fn tx_done(&self, channel: TxChannelIndex) -> bool {
        match channel {
            TxChannelIndex::Zero => self.rmt.int_raw.ch0_tx_end_int_raw(),
            TxChannelIndex::One => self.rmt.int_raw.ch1_tx_end_int_raw(),
            TxChannelIndex::Two => self.rmt.int_raw.ch2_tx_end_int_raw(),
            TxChannelIndex::Three => self.rmt.int_raw.ch3_tx_end_int_raw(),
        }
    }

    /// Clears the transmit-end status.
    pub fn clear_tx_done(&self, channel: TxChannelIndex) {
        match channel {
            TxChannelIndex::Zero => self.rmt.int_clr.trigger_ch0_tx_end_int_clr(),
            TxChannelIndex::One => self.rmt.int_clr.trigger_ch1_tx_end_int_clr(),
            TxChannelIndex::Two => self.rmt.int_clr.trigger_ch2_tx_end_int_clr(),
            TxChannelIndex::Three => self.rmt.int_clr.trigger_ch3_tx_end_int_clr(),
        }
    }

    /// Enables the transmit-end interrupt.
    pub fn enable_tx_end_interrupt(&self, channel: TxChannelIndex, enable: bool) {
        match (channel, enable) {
            (TxChannelIndex::Zero, true) => self.rmt.int_ena.set_ch0_tx_end_int_ena(),
            (TxChannelIndex::Zero, false) => self.rmt.int_ena.clr_ch0_tx_end_int_ena(),
            (TxChannelIndex::One, true) => self.rmt.int_ena.set_ch1_tx_end_int_ena(),
            (TxChannelIndex::One, false) => self.rmt.int_ena.clr_ch1_tx_end_int_ena(),
            (TxChannelIndex::Two, true) => self.rmt.int_ena.set_ch2_tx_end_int_ena(),
            (TxChannelIndex::Two, false) => self.rmt.int_ena.clr_ch2_tx_end_int_ena(),
            (TxChannelIndex::Three, true) => self.rmt.int_ena.set_ch3_tx_end_int_ena(),
            (TxChannelIndex::Three, false) => self.rmt.int_ena.clr_ch3_tx_end_int_ena(),
        }
    }

    /// Spins until the sequence has been sent.
    ///
    /// # Errors
    ///
    /// [`Error::Timeout`] when the transmitter is still running after
    /// `timeout_us` microseconds.
    pub fn wait_tx_done(&self, channel: TxChannelIndex, timeout_us: u64) -> Result<(), Error> {
        wait_until(timeout_us, || self.tx_done(channel)).ok_or(Error::Timeout)
    }

    /// Sends `pulses` and waits for the transmitter to finish.
    ///
    /// # Errors
    ///
    /// The same as [`transmit`](Self::transmit) and
    /// [`wait_tx_done`](Self::wait_tx_done).
    pub fn send(
        &self,
        channel: TxChannelIndex,
        pulses: &[Pulse],
        timeout_us: u64,
    ) -> Result<(), Error> {
        self.clear_tx_done(channel);
        self.transmit(channel, pulses)?;
        self.wait_tx_done(channel, timeout_us)
    }

    /// Pulse codes the channel's configured blocks can hold.
    pub fn tx_capacity(&self, channel: TxChannelIndex) -> usize {
        let blocks = self.tx_conf(channel).blocks();
        if blocks == 0 {
            PULSES_PER_BLOCK
        } else {
            blocks as usize * PULSES_PER_BLOCK
        }
    }

    /// The transmitter's state-machine value; zero means idle.
    pub fn tx_state(&self, channel: TxChannelIndex) -> u32 {
        self.tx_status(channel).state()
    }

    /// Whether the channel's RAM is empty.
    pub fn tx_memory_empty(&self, channel: TxChannelIndex) -> bool {
        self.tx_status(channel).memory_empty()
    }

    fn reset_tx_clock_divider(&self, channel: TxChannelIndex) {
        match channel {
            TxChannelIndex::Zero => self.rmt.ref_cnt_rst.trigger_tx_ref_cnt_rst_ch0(),
            TxChannelIndex::One => self.rmt.ref_cnt_rst.trigger_tx_ref_cnt_rst_ch1(),
            TxChannelIndex::Two => self.rmt.ref_cnt_rst.trigger_tx_ref_cnt_rst_ch2(),
            TxChannelIndex::Three => self.rmt.ref_cnt_rst.trigger_tx_ref_cnt_rst_ch3(),
        }
    }

    // ========================================================================
    // Receive
    // ========================================================================

    /// Applies `config` to a receive channel.
    ///
    /// # Errors
    ///
    /// [`Error::Divider`] when the channel divider is zero, [`Error::Blocks`]
    /// when `blocks` is outside what the channel can own.
    pub fn configure_rx(&self, channel: RxChannelIndex, config: &RxConfig) -> Result<(), Error> {
        if config.divider == 0 {
            return Err(Error::Divider);
        }
        let usable = match channel {
            RxChannelIndex::Zero => 4,
            RxChannelIndex::One => 3,
            RxChannelIndex::Two => 2,
            RxChannelIndex::Three => 1,
        };
        if config.blocks == 0 || config.blocks > usable {
            return Err(Error::Blocks);
        }

        self.reset_rx_clock_divider(channel);
        let conf0 = self.rx_conf0(channel);
        // Carrier demodulation is on out of reset, and with the default
        // thresholds it strips every pulse that is not a carrier, leaving the
        // receiver with no edges at all.
        conf0.set_carrier_demodulation(false);
        conf0.set_divider(config.divider);
        conf0.set_idle_threshold(config.idle_threshold);
        conf0.set_blocks(config.blocks);

        let conf1 = self.rx_conf1(channel);
        conf1.set_filter(config.filter_threshold);
        conf1.update();
        Ok(())
    }

    /// Starts receiving. The channel stops on its own once the input stays at
    /// one level for longer than the idle threshold.
    pub fn start_rx(&self, channel: RxChannelIndex) {
        let conf1 = self.rx_conf1(channel);
        conf1.reset_pointers();
        // The receiver can only write the RAM while it owns it; APB reads and
        // receiver writes must not overlap.
        conf1.set_mem_owner(true);
        conf1.set_enabled(true);
    }

    /// Stops receiving.
    pub fn stop_rx(&self, channel: RxChannelIndex) {
        self.rx_conf1(channel).set_enabled(false);
    }

    /// Whether a reception has finished.
    pub fn rx_done(&self, channel: RxChannelIndex) -> bool {
        match channel {
            RxChannelIndex::Zero => self.rmt.int_raw.ch4_rx_end_int_raw(),
            RxChannelIndex::One => self.rmt.int_raw.ch5_rx_end_int_raw(),
            RxChannelIndex::Two => self.rmt.int_raw.ch6_rx_end_int_raw(),
            RxChannelIndex::Three => self.rmt.int_raw.ch7_rx_end_int_raw(),
        }
    }

    /// Clears the receive-end status.
    pub fn clear_rx_done(&self, channel: RxChannelIndex) {
        match channel {
            RxChannelIndex::Zero => self.rmt.int_clr.trigger_ch4_rx_end_int_clr(),
            RxChannelIndex::One => self.rmt.int_clr.trigger_ch5_rx_end_int_clr(),
            RxChannelIndex::Two => self.rmt.int_clr.trigger_ch6_rx_end_int_clr(),
            RxChannelIndex::Three => self.rmt.int_clr.trigger_ch7_rx_end_int_clr(),
        }
    }

    /// Enables the receive-end interrupt.
    pub fn enable_rx_end_interrupt(&self, channel: RxChannelIndex, enable: bool) {
        match (channel, enable) {
            (RxChannelIndex::Zero, true) => self.rmt.int_ena.set_ch4_rx_end_int_ena(),
            (RxChannelIndex::Zero, false) => self.rmt.int_ena.clr_ch4_rx_end_int_ena(),
            (RxChannelIndex::One, true) => self.rmt.int_ena.set_ch5_rx_end_int_ena(),
            (RxChannelIndex::One, false) => self.rmt.int_ena.clr_ch5_rx_end_int_ena(),
            (RxChannelIndex::Two, true) => self.rmt.int_ena.set_ch6_rx_end_int_ena(),
            (RxChannelIndex::Two, false) => self.rmt.int_ena.clr_ch6_rx_end_int_ena(),
            (RxChannelIndex::Three, true) => self.rmt.int_ena.set_ch7_rx_end_int_ena(),
            (RxChannelIndex::Three, false) => self.rmt.int_ena.clr_ch7_rx_end_int_ena(),
        }
    }

    /// Number of pulse codes the last reception left in RAM.
    ///
    /// It comes from the write cursor minus the channel's block start, so it
    /// counts codes, not pairs, and can include a trailing end marker.
    pub fn rx_count(&self, channel: RxChannelIndex) -> usize {
        let cursor = self.rx_status(channel).write_cursor() as usize;
        let start = (channel as usize + 4) * RAM_WORDS_PER_CHANNEL;
        cursor.saturating_sub(start) * 2
    }

    /// Drains the received codes into `buffer`, stopping at an end marker or
    /// when the buffer is full. Returns how many pulses were stored.
    pub fn read_pulses(&self, channel: RxChannelIndex, buffer: &mut [Pulse]) -> usize {
        // Hand the RAM back to the APB side before reading it.
        self.rx_conf1(channel).set_mem_owner(false);
        let count = self.rx_count(channel).min(buffer.len());
        let ram = self.rx_ram(channel);
        let mut written = 0;
        while written < count {
            // SAFETY: the cursor never exceeds the channel's 48-word blocks,
            // so `written / 2` is inside the RAM.
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

    /// Receives one burst into `buffer`: starts the channel, waits for the idle
    /// threshold to end it, then drains the RAM.
    ///
    /// # Errors
    ///
    /// [`Error::Timeout`] when the channel is still receiving after
    /// `timeout_us` microseconds.
    pub fn receive(
        &self,
        channel: RxChannelIndex,
        buffer: &mut [Pulse],
        timeout_us: u64,
    ) -> Result<usize, Error> {
        self.receive_with(channel, buffer, timeout_us, || Ok(()))
    }

    /// Like [`receive`](Self::receive), but runs `trigger` after the receiver
    /// is listening, which is what a loopback needs: the transmitter has to
    /// start after the receiver, or the first edge is missed.
    ///
    /// # Errors
    ///
    /// [`Error::Timeout`] when the channel is still receiving after
    /// `timeout_us` microseconds, plus whatever `trigger` returns.
    pub fn receive_with<F>(
        &self,
        channel: RxChannelIndex,
        buffer: &mut [Pulse],
        timeout_us: u64,
        trigger: F,
    ) -> Result<usize, Error>
    where
        F: FnOnce() -> Result<(), Error>,
    {
        self.clear_rx_done(channel);
        self.start_rx(channel);
        trigger()?;

        if wait_until(timeout_us, || self.rx_done(channel)).is_none() {
            self.stop_rx(channel);
            return Err(Error::Timeout);
        }
        Ok(self.read_pulses(channel, buffer))
    }

    fn reset_rx_clock_divider(&self, channel: RxChannelIndex) {
        match channel {
            RxChannelIndex::Zero => self.rmt.ref_cnt_rst.trigger_rx_ref_cnt_rst_ch4(),
            RxChannelIndex::One => self.rmt.ref_cnt_rst.trigger_rx_ref_cnt_rst_ch5(),
            RxChannelIndex::Two => self.rmt.ref_cnt_rst.trigger_rx_ref_cnt_rst_ch6(),
            RxChannelIndex::Three => self.rmt.ref_cnt_rst.trigger_rx_ref_cnt_rst_ch7(),
        }
    }
}

/// Polls `done` until it returns true or `timeout_us` microseconds pass.
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
