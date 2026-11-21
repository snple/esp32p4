//! I2S controllers, in standard (two-slot) mode.
//!
//! The ESP32-P4 has three I2S controllers. Each one has a transmit and a receive
//! half with their own clock, format registers and FIFO, and the data path is
//! GDMA-only: there is no CPU-visible data register, so every transfer goes
//! through [`crate::dma`]. This driver covers the two-slot standards —
//! [`Standard::Philips`] and [`Standard::MsbAligned`] — in master or slave role,
//! with 8 to 32 bit slots and a fractional MCLK divider.
//!
//! # Features
//!
//! - Clock tree: source selection (XTAL, PLL_F160M, APLL, external pad),
//!   fractional MCLK divider in `HP_SYS_CLKRST`, and the BCK divider inside the
//!   peripheral — [`StandardConfig`] goes in, [`Clock`] comes out with the
//!   resulting MCLK, BCK and sample rate
//! - Transmit and receive with GDMA descriptor chains, cache maintenance
//!   included ([`I2s0::tx_write`], [`I2s0::rx_read`]), either once or as a
//!   descriptor ring that keeps repeating
//!   ([`I2s0::tx_stream_begin`], [`I2s0::rx_stream_begin`])
//! - Slot formatting: mono or stereo, left/right channel only, MSB first,
//!   WS idle polarity, data bits narrower than the slot
//! - TDM: up to sixteen slots per frame with an active-slot mask, so the DMA
//!   stream only carries the slots that matter ([`TdmConfig`])
//! - Internal BCK/WS sharing for full-duplex in one controller
//!   ([`I2s0::share_bck_ws`]), MCLK pad gate, BCK cycle counter for measuring
//!   the real bit clock, hung-interrupt timing, per-source interrupts
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::dma::{AhbDma, Chain, Descriptor, Peripheral};
//! use esp32p4_hal::gpio::signal::OutputSignal;
//! use esp32p4_hal::gpio::{PinOps, Pins};
//! use esp32p4_hal::i2s::{I2s0, Slot, Standard, StandardConfig};
//!
//! let mut bck = pins.pin::<4>();
//! bck.route_output_signal(OutputSignal::I2S0_O_BCK_PAD_OUT);
//! let mut ws = pins.pin::<5>();
//! ws.route_output_signal(OutputSignal::I2S0_O_WS_PAD_OUT);
//! let mut sd = pins.pin::<6>();
//! sd.route_output_signal(OutputSignal::I2S0_O_SD_PAD_OUT);
//!
//! let i2s = I2s0::init(dp.I2S0, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
//! let clock = i2s.tx_configure(&StandardConfig {
//!     sample_rate: 48_000,
//!     data_bits: 16,
//!     slot_bits: 16,
//!     slot: Slot::Both,
//!     standard: Standard::Philips,
//!     ..StandardConfig::default()
//! })?;
//! assert_eq!(clock.bclk_hz, 1_536_000);
//!
//! let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
//! let tx = dma.tx_channel::<0>();
//! let descriptors = [const { Descriptor::new() }; 2];
//!
//! i2s.tx_start();
//! i2s.tx_write(&tx, &samples, &descriptors)?;
//! i2s.tx_stop();
//! ```
//!
//! # Hardware Notes
//!
//! - There is no CPU data path. `I2S_TXFIFO`/`I2S_RXFIFO` registers do not
//!   exist on this chip: the FIFO is filled and drained by GDMA, so the
//!   controller needs a DMA channel connected to [`Peripheral::I2s0`] and up
//!   [`Peripheral::I2s2`].
//! - The clock tree is *not* in the peripheral. `HP_SYS_CLKRST` holds the source
//!   select, the module gates and a fractional divider
//!   (`f = f_src / (N + b/a)`); `I2S_TX/RX_BCK_DIV_NUM` then divides that down
//!   to BCK. The TRM requires `N >= 2` and a BCK divider of at least 2.
//! - Standard mode is TDM mode with two slots: `TDM_EN` must be set and
//!   `PDM_EN` clear, otherwise the controller sends in a mode that is neither.
//!   `MSB_SHIFT` picks Philips (WS one BCK ahead of the data) over MSB-aligned.
//! - Both halves have their own clock and BCK divider. Two halves of one
//!   controller used as master and slave can share BCK and WS through
//!   [`I2s0::share_bck_ws`], which is how a full-duplex stream stays in sync
//!   without routing the clocks through pads.
//! - The MCLK pad output has its own gate in `LP_AON_CLKRST`
//!   (`hp_pad_i2sN_mclk_en`); it is open after [`I2s0::init`] so a codec can be
//!   clocked before the stream starts.
//! - `RX_EOF_NUM` is the byte count at which the receive half tells the DMA that
//!   a frame ended. It has to match the descriptor it belongs to, so a single
//!   read is limited to one descriptor worth of data.
//! - `TX/RX_UPDATE` latches the format registers into the data path and is
//!   self-clearing; `start` sets it and waits for it to go back to zero, because
//!   the shifter keeps the previous format until it has.
//! - A slave samples the bus with its own module clock, so the TRM wants
//!   `f(I2S_TX/RX_CLK) >= 8 * BCK`. Leaving the divider at its reset value gives
//!   40 MHz / 256 and a slave that receives nothing at all without ever
//!   reporting an error, which is why [`StandardConfig::mclk_multiple`] is
//!   checked in slave mode too ([`Error::SlaveClockTooSlow`]).
//! - A mono stream does not shorten the frame: the hardware always runs two
//!   slots, so BCK keeps its rate and only the bytes DMA pulls out of the buffer
//!   halve. The same holds for TDM slots that the active mask leaves out.
//! - `TX_TDM_SKIP_MSK_EN` is left clear: with it set the hardware shrinks the
//!   frame to the active slots, the receive side has no matching bit, and the
//!   two ends then disagree about the bit clock by a factor of the slot count.
//! - The engine has to be running before the peripheral is started. Starting
//!   the transmit half first starves it, latches `TX_HUNG`, and the FIFO then
//!   takes data that never leaves; `tx_begin` does reset, DMA start, then
//!   `TX_START` in that order.

use core::cell::Cell;

use esp32p4_pac::{HP_SYS_CLKRST, I2S0, I2S1, I2S2, Interrupt, LP_AON_CLKRST, i2s0, i2s1, i2s2};

use crate::dma::{Descriptor, Peripheral, ReceiveChannel, TransmitChannel};

/// Clock source of the I2S module clock.
///
/// The register value is what `HP_SYS_CLKRST_I2S_N_TX/RX_CLK_SRC_SEL` takes.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// The 40 MHz crystal.
    #[default]
    Xtal,
    /// `PLL_F160M`; only exists from chip revision 3.0 on.
    Pll160m,
    /// The audio PLL, which `clock` can tune to an exact audio multiple.
    Apll,
    /// `I2S_MCLK` driven from a pad.
    External,
}

impl ClockSource {
    const fn register_value(self) -> u32 {
        match self {
            Self::Xtal => 0,
            Self::Apll => 1,
            Self::External => 2,
            Self::Pll160m => 3,
        }
    }
}

/// The two-slot audio standards this driver speaks.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Standard {
    /// Philips: WS changes one BCK before the data's first bit.
    #[default]
    Philips,
    /// MSB aligned: WS and the data change together.
    MsbAligned,
}

/// Which slot carries data.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slot {
    /// Both slots, interleaved in the DMA buffer.
    #[default]
    Both,
    /// Only the left slot; the DMA buffer holds one word per frame.
    Left,
    /// Only the right slot; the DMA buffer holds one word per frame.
    Right,
}

impl Slot {
    /// Whether the DMA buffer carries one word per frame.
    pub const fn is_mono(self) -> bool {
        !matches!(self, Self::Both)
    }

    /// The channel enable mask in `TX/RX_TDM_CTRL`.
    const fn channel_mask(self) -> u32 {
        match self {
            Self::Both => 0b11,
            Self::Left => 0b01,
            Self::Right => 0b10,
        }
    }
}

/// Format and rate of one direction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StandardConfig {
    /// Sampling rate in Hz.
    pub sample_rate: u32,
    /// Valid bits per sample: 1 to 32.
    pub data_bits: u8,
    /// Bits per slot: 1 to 32, and at least `data_bits`.
    pub slot_bits: u16,
    /// Which slot carries data.
    pub slot: Slot,
    /// Philips or MSB aligned.
    pub standard: Standard,
    /// Whether WS idles high. Low is the usual choice; high selects the
    /// right channel first.
    pub ws_idle_high: bool,
    /// MCLK as a multiple of the sample rate: 128, 256, 384 or 512.
    pub mclk_multiple: u16,
    /// Whether this half drives BCK and WS.
    pub master: bool,
}

impl Default for StandardConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            data_bits: 16,
            slot_bits: 16,
            slot: Slot::Both,
            standard: Standard::Philips,
            ws_idle_high: false,
            mclk_multiple: 256,
            master: true,
        }
    }
}

impl StandardConfig {
    /// Bytes one frame occupies in the DMA buffer.
    ///
    /// A mono stream puts one word in the frame and leaves the other slot
    /// silent, so the frame is half as long on the DMA side while BCK keeps its
    /// rate.
    pub fn frame_bytes(&self) -> usize {
        let slots = if self.slot.is_mono() { 1 } else { 2 };
        slots * usize::from(self.slot_bits).div_ceil(8)
    }

    fn layout(&self) -> Result<Layout, Error> {
        Layout {
            sample_rate: self.sample_rate,
            data_bits: self.data_bits,
            slot_bits: self.slot_bits,
            // A standard frame is always two slots wide; the slot choice only
            // decides which of them carry data.
            channels: 2,
            active: self.slot.channel_mask() as u16,
            standard: self.standard,
            ws_idle_high: self.ws_idle_high,
            mclk_multiple: self.mclk_multiple,
            master: self.master,
        }
        .check()
    }
}

/// Time-division multiplexed frame: one WS period carries up to sixteen slots.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TdmConfig {
    /// Sampling rate in Hz.
    pub sample_rate: u32,
    /// Valid bits per sample: 1 to 32.
    pub data_bits: u8,
    /// Bits per slot: 1 to 32, and at least `data_bits`.
    pub slot_bits: u16,
    /// Slots per frame: 2 to 16. Raised to cover the highest active slot.
    pub channels: u8,
    /// Which slots carry data; the rest stay silent and are skipped in the DMA
    /// stream.
    pub active: u16,
    /// Philips or MSB aligned.
    pub standard: Standard,
    /// Whether WS idles high.
    pub ws_idle_high: bool,
    /// MCLK as a multiple of the sample rate.
    pub mclk_multiple: u16,
    /// Whether this half drives BCK and WS.
    pub master: bool,
}

impl Default for TdmConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            data_bits: 16,
            slot_bits: 16,
            channels: 4,
            active: 0xF,
            standard: Standard::Philips,
            ws_idle_high: false,
            mclk_multiple: 256,
            master: true,
        }
    }
}

impl TdmConfig {
    /// Bytes one frame occupies in the DMA buffer: the active slots only.
    pub fn frame_bytes(&self) -> usize {
        let words = self.active.count_ones() as usize;
        words * usize::from(self.slot_bits).div_ceil(8)
    }

    fn layout(&self) -> Result<Layout, Error> {
        if self.active == 0 {
            return Err(Error::SlotsOutOfRange);
        }
        // The frame has to reach the highest active slot; asking for more slots
        // than that only lengthens the silence between frames.
        let highest = 16 - self.active.leading_zeros() as u8;
        let channels = self.channels.max(highest).max(2);
        Layout {
            sample_rate: self.sample_rate,
            data_bits: self.data_bits,
            slot_bits: self.slot_bits,
            channels,
            active: self.active,
            standard: self.standard,
            ws_idle_high: self.ws_idle_high,
            mclk_multiple: self.mclk_multiple,
            master: self.master,
        }
        .check()
    }
}

/// A frame as the hardware sees it.
///
/// Both [`StandardConfig`] and [`TdmConfig`] reduce to this, so the register
/// work — and the clock arithmetic — happens once.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Layout {
    sample_rate: u32,
    data_bits: u8,
    slot_bits: u16,
    channels: u8,
    active: u16,
    standard: Standard,
    ws_idle_high: bool,
    mclk_multiple: u16,
    master: bool,
}

impl Layout {
    fn check(self) -> Result<Self, Error> {
        if !(1..=32).contains(&self.data_bits) || !(1..=32).contains(&self.slot_bits) {
            return Err(Error::BitsOutOfRange);
        }
        if u16::from(self.data_bits) > self.slot_bits {
            return Err(Error::DataWiderThanSlot);
        }
        if self.sample_rate == 0 {
            return Err(Error::RateUnsupported);
        }
        if !(2..=16).contains(&self.channels) || u32::from(self.active) >> self.channels != 0 {
            return Err(Error::SlotsOutOfRange);
        }
        // The frame has to fit the 9-bit WS width and the 8-bit half-frame
        // register, and its halves have to be whole words.
        if self.frame_bits() > 512 || !self.frame_bits().is_multiple_of(2) {
            return Err(Error::SlotsOutOfRange);
        }
        Ok(self)
    }

    /// Bits in one WS period.
    fn frame_bits(&self) -> u32 {
        u32::from(self.slot_bits) * u32::from(self.channels)
    }

    /// Both the WS width and the half-frame point sit in the middle of a frame.
    fn half_frame(&self) -> u32 {
        self.frame_bits() / 2
    }

    fn bclk_hz(&self) -> u32 {
        self.sample_rate * self.frame_bits()
    }

    fn mclk_hz(&self) -> u32 {
        self.sample_rate * u32::from(self.mclk_multiple)
    }
}

/// The clock the configuration ended up with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clock {
    /// Frequency of the clock source.
    pub source_hz: u32,
    /// Module clock, `source / (integer + numerator / denominator)`.
    pub mclk_hz: u32,
    /// Bit clock, `mclk / bck_divisor`.
    pub bclk_hz: u32,
    /// The `BCK_DIV_NUM` field value plus one.
    pub bck_divisor: u32,
    /// Integer part of the MCLK divider.
    pub mclk_integer: u32,
    /// Fraction numerator of the MCLK divider.
    pub mclk_numerator: u32,
    /// Fraction denominator of the MCLK divider.
    pub mclk_denominator: u32,
}

/// Interrupt sources.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Interrupts(u32);

impl Interrupts {
    /// No source at all.
    pub const NONE: Self = Self(0);
    /// The receive half finished a frame.
    pub const RX_DONE: Self = Self(1 << 0);
    /// The transmit half finished a frame.
    pub const TX_DONE: Self = Self(1 << 1);
    /// The receive half starved.
    pub const RX_HUNG: Self = Self(1 << 2);
    /// The transmit half starved.
    pub const TX_HUNG: Self = Self(1 << 3);

    /// Adds another source.
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether `other` is part of this set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Whether no source is set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The raw register value.
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Configuration errors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// `data_bits` or `slot_bits` is outside 1..=32.
    BitsOutOfRange,
    /// More valid bits than the slot has room for.
    DataWiderThanSlot,
    /// The slot count, the active-slot mask or the frame length is not one the
    /// hardware can produce.
    SlotsOutOfRange,
    /// The clock source cannot produce this sample rate.
    RateUnsupported,
    /// A single read has to fit one descriptor, which holds 4095 bytes.
    FrameTooLong,
    /// A slave half needs a module clock of at least eight BCK periods.
    SlaveClockTooSlow,
    /// The controller did not finish in time.
    Timeout,
    /// GDMA refused the transfer.
    Dma(crate::dma::Error),
}

// ============================================================================
// Clock arithmetic
// ============================================================================

/// The MCLK divider split into the five coefficients the hardware wants.
struct Divider {
    integer: u32,
    numerator: u32,
    denominator: u32,
    n: u32,
    x: u32,
    y: u32,
    z: u32,
    yn1: bool,
}

/// Splits `source / mclk` into an integer and a fraction of at most 1/512.
fn mclk_divider(source_hz: u32, mclk_hz: u32) -> Result<Divider, Error> {
    if mclk_hz == 0 || source_hz == 0 {
        return Err(Error::RateUnsupported);
    }
    // The integer part has to be at least 2, which is `source > 2 * mclk`.
    let mut integer = source_hz / mclk_hz;
    let mut remainder = source_hz % mclk_hz;
    if integer < 2 {
        // A source that is nominally twice the target lands a hair below it
        // once it comes back from a PLL, and the hardware cannot divide by less
        // than two. Round up instead of failing: the resulting error is in the
        // parts-per-million range.
        if u64::from(source_hz) * 100 < u64::from(mclk_hz) * 199 {
            return Err(Error::RateUnsupported);
        }
        integer = 2;
        remainder = 0;
    }
    const A: u32 = 512;
    let mut b = ((u64::from(remainder) * u64::from(A) + u64::from(mclk_hz) / 2)
        / u64::from(mclk_hz)) as u32;
    if b >= A {
        b = 0;
        integer += 1;
    }
    if integer > 256 {
        return Err(Error::RateUnsupported);
    }

    let (n, x, y, z, yn1) = if b == 0 {
        // Integer division: `X` and `Z` clear, `Y` = 1.
        (integer, 0, 1, 0, false)
    } else if b <= A / 2 {
        (integer, A / b - 1, A % b, b, false)
    } else {
        let rest = A - b;
        (integer, A / rest - 1, A % rest, rest, true)
    };
    // `N` is eight bits wide with 0 meaning 256 and 1 meaning 2.
    let n = match n {
        256 => 0,
        2 => 1,
        other => other,
    };
    Ok(Divider {
        integer,
        numerator: b,
        denominator: A,
        n,
        x,
        y,
        z,
        yn1,
    })
}

/// The clock source frequency, given the crystal.
fn source_hz(source: ClockSource, external_hz: u32) -> u32 {
    match source {
        ClockSource::Xtal => crate::clock::XTAL_HZ,
        ClockSource::Pll160m => 160_000_000,
        // A caller-selected APLL or pad clock: the value is passed in.
        ClockSource::Apll | ClockSource::External => external_hz,
    }
}

// ============================================================================
// Controllers
// ============================================================================

macro_rules! impl_i2s {
    (
        $Name:ident, $PAC:ident, $mod:ident, $irq:ident, $peri:ident,
        $apb:ident, $rst:ident, $mclk_gate:ident,
        $rx_clk:ident, $rx_clk_en:ident, $rx_clk_src:ident,
        $tx_clk:ident, $tx_clk_en:ident, $tx_clk_src:ident,
        $rx_n_reg:ident, $rx_n:ident, $rx_x_reg:ident, $rx_x:ident,
        $rx_y_reg:ident, $rx_y:ident, $rx_z_reg:ident, $rx_z:ident, $rx_yn_reg:ident, $rx_yn:ident,
        $tx_n_reg:ident, $tx_n:ident, $tx_x_reg:ident, $tx_x:ident,
        $tx_y_reg:ident, $tx_y:ident, $tx_z_reg:ident, $tx_z:ident, $tx_yn_reg:ident, $tx_yn:ident,
        $mst_reg:ident, $mst:ident
    ) => {
        pastey::paste! {
            /// I2S controller owning the PAC peripheral.
            pub struct $Name {
                i2s: $PAC,
                /// Frequency of the clock source selected for the MCLK divider.
                source_hz: Cell<u32>,
                /// Bit clock the last configuration asks for, used to size the
                /// deadline of a transfer.
                bclk_hz: Cell<u32>,
                /// Bytes the running transmit transfer moves; the wait needs
                /// them to know how long to be patient.
                tx_bytes: Cell<u32>,
            }

            impl $Name {
                /// The interrupt source of this controller.
                pub const SOURCE: Interrupt = Interrupt::$irq;

                /// Opens the bus and module clocks, releases the reset and
                /// silences both halves.
                pub fn init(i2s: $PAC, clkrst: &HP_SYS_CLKRST, lpaon: &LP_AON_CLKRST) -> Self {
                    clkrst.soc_clk_ctrl2.[<set_ $apb>]();
                    // The MCLK pad has its own gate, so a codec can be clocked
                    // as soon as the controller is up.
                    lpaon.lp_aonclkrst_hp_clk_ctrl.[<set_ $mclk_gate>]();
                    clkrst.hp_rst_en2.[<set_ $rst>]();
                    clkrst.hp_rst_en2.[<clr_ $rst>]();
                    clkrst.[<$rx_clk>].[<set_ $rx_clk_en>]();
                    clkrst.[<$tx_clk>].[<set_ $tx_clk_en>]();

                    let this = Self {
                        i2s,
                        source_hz: Cell::new(crate::clock::XTAL_HZ),
                        bclk_hz: Cell::new(1),
                        tx_bytes: Cell::new(0),
                    };
                    let regs = this.regs();
                    regs.tx_conf.trigger_tx_reset();
                    regs.tx_conf.trigger_tx_fifo_reset();
                    regs.rx_conf.trigger_rx_reset();
                    regs.rx_conf.trigger_rx_fifo_reset();
                    this
                }

                /// Obtains a handle without ownership checks.
                ///
                /// # Safety
                ///
                /// The caller must ensure the controller was initialised and
                /// that the returned handle is not used to reconfigure it
                /// concurrently.
                #[inline]
                pub unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self {
                        i2s: unsafe { $PAC::steal() },
                        source_hz: Cell::new(crate::clock::XTAL_HZ),
                        bclk_hz: Cell::new(1),
                        tx_bytes: Cell::new(0),
                    })
                }

                /// The underlying register block.
                #[inline]
                pub fn regs(&self) -> &$mod::RegisterBlock {
                    &self.i2s
                }

                /// Returns the PAC peripheral.
                #[inline]
                pub fn release(self) -> $PAC {
                    self.i2s
                }

                /// Selects the clock source both halves divide from.
                ///
                /// `external_hz` is the frequency of the clock on the MCLK pad
                /// or of the audio PLL; it is ignored for [`ClockSource::Xtal`]
                /// and [`ClockSource::Pll160m`]. The divider is recomputed on
                /// the next `tx_configure` / `rx_configure`.
                pub fn set_clock_source(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    source: ClockSource,
                    external_hz: u32,
                ) {
                    let value = source.register_value();
                    clkrst.[<$rx_clk>].[<write_ $rx_clk_src>](value);
                    clkrst.[<$tx_clk>].[<write_ $tx_clk_src>](value);
                    self.source_hz.set(source_hz(source, external_hz));
                }

                /// The frequency the MCLK divider starts from.
                #[inline]
                pub fn source_hz(&self) -> u32 {
                    self.source_hz.get()
                }

                /// Configures the transmit half for standard mode and returns
                /// the clock it will run at.
                pub fn tx_configure(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    config: &StandardConfig,
                ) -> Result<Clock, Error> {
                    self.tx_apply(clkrst, &config.layout()?)
                }

                /// Configures the transmit half for TDM mode.
                pub fn tx_configure_tdm(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    config: &TdmConfig,
                ) -> Result<Clock, Error> {
                    self.tx_apply(clkrst, &config.layout()?)
                }

                /// Configures the receive half for standard mode and returns
                /// the clock it will run at.
                ///
                /// A slave half takes BCK and WS from the master, so
                /// [`Clock`] describes the format rather than a clock the half
                /// generates.
                pub fn rx_configure(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    config: &StandardConfig,
                ) -> Result<Clock, Error> {
                    self.rx_apply(clkrst, &config.layout()?)
                }

                /// Configures the receive half for TDM mode.
                pub fn rx_configure_tdm(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    config: &TdmConfig,
                ) -> Result<Clock, Error> {
                    self.rx_apply(clkrst, &config.layout()?)
                }

                fn tx_apply(&self, clkrst: &HP_SYS_CLKRST, layout: &Layout) -> Result<Clock, Error> {
                    let clock = self.program_clock(clkrst, layout, MclkOwner::Transmit)?;

                    let r = self.regs();
                    r.tx_conf.trigger_tx_reset();
                    r.tx_conf.trigger_tx_fifo_reset();
                    if layout.master {
                        r.tx_conf.clr_tx_slave_mod();
                    } else {
                        r.tx_conf.set_tx_slave_mod();
                    }
                    // The peripheral has no separate "standard" bit: a
                    // two-slot frame is TDM with two slots, and PDM must stay
                    // off or the controller sends in neither mode.
                    r.tx_conf.set_tx_tdm_en();
                    r.tx_conf.clr_tx_pdm_en();
                    r.tx_conf1
                        .write_tx_bits_mod(u32::from(layout.data_bits - 1));
                    r.tx_conf1
                        .write_tx_tdm_chan_bits(u32::from(layout.slot_bits - 1));
                    r.tx_conf1
                        .write_tx_half_sample_bits(layout.half_frame() - 1);
                    r.tx_conf1
                        .write_tx_tdm_ws_width(layout.half_frame() - 1);
                    match layout.standard {
                        Standard::Philips => r.tx_conf.set_tx_msb_shift(),
                        Standard::MsbAligned => r.tx_conf.clr_tx_msb_shift(),
                    }
                    if layout.ws_idle_high {
                        r.tx_conf.set_tx_ws_idle_pol();
                    } else {
                        r.tx_conf.clr_tx_ws_idle_pol();
                    }
                    r.tx_conf.clr_tx_bit_order();
                    r.tx_conf.clr_tx_mono();
                    // One write describes the whole frame: the slot enables and
                    // the slot count. `SKIP_MSK_EN` stays clear on purpose —
                    // with it set the hardware shrinks the frame to the active
                    // slots, which the receive side cannot express and which
                    // would silently change the bit clock.
                    let tdm = u32::from(layout.active) | (u32::from(layout.channels - 1) << 16);
                    r.tx_tdm_ctrl.write_bits(tdm);
                    Ok(clock)
                }

                fn rx_apply(&self, clkrst: &HP_SYS_CLKRST, layout: &Layout) -> Result<Clock, Error> {
                    let clock = self.program_clock(clkrst, layout, MclkOwner::Receive)?;

                    let r = self.regs();
                    r.rx_conf.trigger_rx_reset();
                    r.rx_conf.trigger_rx_fifo_reset();
                    if layout.master {
                        r.rx_conf.clr_rx_slave_mod();
                    } else {
                        r.rx_conf.set_rx_slave_mod();
                    }
                    r.rx_conf.set_rx_tdm_en();
                    r.rx_conf.clr_rx_pdm_en();
                    r.rx_conf1
                        .write_rx_bits_mod(u32::from(layout.data_bits - 1));
                    r.rx_conf1
                        .write_rx_tdm_chan_bits(u32::from(layout.slot_bits - 1));
                    r.rx_conf1
                        .write_rx_half_sample_bits(layout.half_frame() - 1);
                    r.rx_conf1
                        .write_rx_tdm_ws_width(layout.half_frame() - 1);
                    match layout.standard {
                        Standard::Philips => r.rx_conf.set_rx_msb_shift(),
                        Standard::MsbAligned => r.rx_conf.clr_rx_msb_shift(),
                    }
                    if layout.ws_idle_high {
                        r.rx_conf.set_rx_ws_idle_pol();
                    } else {
                        r.rx_conf.clr_rx_ws_idle_pol();
                    }
                    r.rx_conf.clr_rx_bit_order();
                    r.rx_conf.clr_rx_mono();
                    // The receive side has no skip bit: the slot enables alone
                    // decide which slots reach the DMA.
                    let tdm = u32::from(layout.active) | (u32::from(layout.channels - 1) << 16);
                    r.rx_tdm_ctrl.write_bits(tdm);
                    Ok(clock)
                }

                /// Starts the transmit half.
                ///
                /// Latches the format registers first: `TX_UPDATE` is
                /// self-clearing, and the data path keeps the previous format
                /// until it has gone through.
                pub fn tx_start(&self) {
                    let r = self.regs();
                    r.tx_conf.set_tx_update();
                    crate::spin::spin_wait("i2s tx update", || !r.tx_conf.tx_update());
                    r.tx_conf.set_tx_start();
                }

                /// Stops the transmit half.
                pub fn tx_stop(&self) {
                    self.regs().tx_conf.clr_tx_start();
                }

                /// Starts the receive half.
                pub fn rx_start(&self) {
                    let r = self.regs();
                    r.rx_conf.set_rx_update();
                    crate::spin::spin_wait("i2s rx update", || !r.rx_conf.rx_update());
                    r.rx_conf.set_rx_start();
                }

                /// Stops the receive half.
                pub fn rx_stop(&self) {
                    self.regs().rx_conf.clr_rx_start();
                }

                /// Whether the transmit half has drained its FIFO and shifted
                /// out the last bit.
                pub fn tx_idle(&self) -> bool {
                    self.regs().state.tx_idle()
                }

                /// Shares BCK and WS between the two halves.
                ///
                /// That is what a full-duplex stream inside one controller
                /// needs: the transmit half runs as master and the receive half
                /// as slave, and the clocks never leave the peripheral.
                pub fn share_bck_ws(&self, enable: bool) {
                    if enable {
                        self.regs().tx_conf.set_sig_loopback();
                    } else {
                        self.regs().tx_conf.clr_sig_loopback();
                    }
                }

                /// How many BCK cycles the transmit half has generated.
                ///
                /// The counter is free running once TX is configured; reading
                /// it twice around a known delay measures the real bit clock.
                pub fn tx_bck_count(&self) -> u32 {
                    self.regs().bck_cnt.tx_bck_cnt()
                }

                /// Zeroes the BCK cycle counter.
                pub fn reset_tx_bck_counter(&self) {
                    self.regs().bck_cnt.trigger_tx_bck_cnt_rst();
                }

                /// Sets the hung timeout, in `1024 * 2^shift` module clock
                /// cycles.
                ///
                /// A slave that never sees BCK or WS hangs; this is how long
                /// the peripheral waits before it says so.
                pub fn set_hung_timeout(&self, timeout: u8, shift: u8) {
                    let r = self.regs();
                    r.lc_hung_conf.write_lc_fifo_timeout(u32::from(timeout));
                    r.lc_hung_conf
                        .write_lc_fifo_timeout_shift(u32::from(shift));
                    r.lc_hung_conf.set_lc_fifo_timeout_ena();
                }

                /// Enables exactly the given interrupt sources.
                pub fn enable_interrupts(&self, sources: Interrupts) {
                    let r = self.regs();
                    if sources.contains(Interrupts::RX_DONE) {
                        r.int_ena.set_rx_done_int_ena();
                    } else {
                        r.int_ena.clr_rx_done_int_ena();
                    }
                    if sources.contains(Interrupts::TX_DONE) {
                        r.int_ena.set_tx_done_int_ena();
                    } else {
                        r.int_ena.clr_tx_done_int_ena();
                    }
                    if sources.contains(Interrupts::RX_HUNG) {
                        r.int_ena.set_rx_hung_int_ena();
                    } else {
                        r.int_ena.clr_rx_hung_int_ena();
                    }
                    if sources.contains(Interrupts::TX_HUNG) {
                        r.int_ena.set_tx_hung_int_ena();
                    } else {
                        r.int_ena.clr_tx_hung_int_ena();
                    }
                }

                /// Disables every interrupt source.
                pub fn disable_interrupts(&self) {
                    let r = self.regs();
                    r.int_ena.clr_rx_done_int_ena();
                    r.int_ena.clr_tx_done_int_ena();
                    r.int_ena.clr_rx_hung_int_ena();
                    r.int_ena.clr_tx_hung_int_ena();
                }

                /// The latched interrupt sources.
                pub fn interrupts(&self) -> Interrupts {
                    Interrupts(self.regs().int_st.read_bits() & 0xF)
                }

                /// Acknowledges `sources`.
                pub fn clear_interrupts(&self, sources: Interrupts) {
                    let r = self.regs();
                    if sources.contains(Interrupts::RX_DONE) {
                        r.int_clr.trigger_rx_done_int_clr();
                    }
                    if sources.contains(Interrupts::TX_DONE) {
                        r.int_clr.trigger_tx_done_int_clr();
                    }
                    if sources.contains(Interrupts::RX_HUNG) {
                        r.int_clr.trigger_rx_hung_int_clr();
                    }
                    if sources.contains(Interrupts::TX_HUNG) {
                        r.int_clr.trigger_tx_hung_int_clr();
                    }
                }

                /// Points the transmit DMA channel at `buffer` and starts it.
                ///
                /// Paired with [`tx_wait`](Self::tx_wait); [`tx_write`](Self::tx_write)
                /// is both in one call. The buffer stays owned by the engine
                /// until the wait returns, so it has to stay put and unmodified.
                pub fn tx_begin(
                    &self,
                    dma: &impl TransmitChannel,
                    buffer: &[u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    if buffer.is_empty() {
                        return Err(Error::Dma(crate::dma::Error::EmptyBuffer));
                    }
                    self.tx_launch(dma, buffer.as_ptr(), buffer.len(), descriptors, false)?;
                    self.tx_bytes.set(buffer.len() as u32);
                    Ok(())
                }

                /// Starts a transmit stream that repeats `buffer` forever.
                ///
                /// The descriptors are mounted as a ring, so the engine walks
                /// them round and round; nothing reports a frame end, which is
                /// what a looping tone or a DMA-fed codec needs. Stop it with
                /// [`tx_stream_stop`](Self::tx_stream_stop).
                pub fn tx_stream_begin(
                    &self,
                    dma: &impl TransmitChannel,
                    buffer: &[u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    if buffer.is_empty() {
                        return Err(Error::Dma(crate::dma::Error::EmptyBuffer));
                    }
                    self.tx_launch(dma, buffer.as_ptr(), buffer.len(), descriptors, true)
                }

                /// Stops the stream started by
                /// [`tx_stream_begin`](Self::tx_stream_begin).
                pub fn tx_stream_stop(&self, dma: &impl TransmitChannel) {
                    self.tx_stop();
                    dma.stop_chain();
                    dma.clear_all_interrupts();
                }

                fn tx_launch(
                    &self,
                    dma: &impl TransmitChannel,
                    buffer: *const u8,
                    len: usize,
                    descriptors: &[Descriptor],
                    ring: bool,
                ) -> Result<(), Error> {
                    let chain = crate::dma::Chain::new(descriptors);
                    if ring {
                        chain
                            .mount_ring(buffer as *mut u8, len)
                            .map_err(Error::Dma)?;
                    } else {
                        // The trailing EOF is what tells the engine the phase
                        // ended; without it the channel would wait forever.
                        chain.mount_eof(buffer as *mut u8, len).map_err(Error::Dma)?;
                    }

                    // A fresh half: the shifter and its FIFO are reset right
                    // before the engine is pointed at the buffer, the same order
                    // the reference driver uses. Starting the peripheral before
                    // the engine has data starves it and it stays hungry.
                    let r = self.regs();
                    r.tx_conf.trigger_tx_reset();
                    r.tx_conf.trigger_tx_fifo_reset();
                    dma.connect_to(Peripheral::$peri);
                    dma.reset_channel();
                    dma.clear_all_interrupts();
                    crate::cache::writeback(buffer.cast(), len);
                    dma.start_chain(chain.head());
                    r.int_clr.trigger_tx_hung_int_clr();
                    r.int_clr.trigger_tx_done_int_clr();
                    self.tx_start();
                    Ok(())
                }

                /// Waits for the transmission started by
                /// [`tx_begin`](Self::tx_begin) and returns once the last bit
                /// left the shift register.
                pub fn tx_wait(&self, dma: &impl TransmitChannel) -> Result<(), Error> {
                    let bytes = self.tx_bytes.get() as usize;
                    let deadline = crate::delay::micros() + self.transfer_budget(bytes);
                    while !dma.idle() {
                        if crate::delay::micros() > deadline {
                            dma.stop_chain();
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    // The descriptor is done, but the last word may still be in
                    // the shift register.
                    while !self.tx_idle() {
                        if crate::delay::micros() > deadline {
                            self.tx_stop();
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    self.tx_stop();
                    Ok(())
                }

                /// Sends `buffer` and waits until the peripheral shifted it
                /// out.
                pub fn tx_write(
                    &self,
                    dma: &impl TransmitChannel,
                    buffer: &[u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    self.tx_begin(dma, buffer, descriptors)?;
                    self.tx_wait(dma)
                }

                /// Points the receive DMA channel at `buffer` and starts it.
                ///
                /// One read is one frame the peripheral can count, so `buffer`
                /// may not exceed a single descriptor.
                pub fn rx_begin(
                    &self,
                    dma: &impl ReceiveChannel,
                    buffer: &mut [u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    self.rx_launch(dma, buffer, descriptors, false)
                }

                /// Starts a receive stream that keeps refilling `buffer`.
                ///
                /// The descriptors are mounted as a ring, so the engine
                /// overwrites the buffer lap after lap; the newest samples are
                /// always the ones just written. Stop it with
                /// [`rx_stream_stop`](Self::rx_stream_stop).
                pub fn rx_stream_begin(
                    &self,
                    dma: &impl ReceiveChannel,
                    buffer: &mut [u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    self.rx_launch(dma, buffer, descriptors, true)
                }

                /// Stops the stream started by
                /// [`rx_stream_begin`](Self::rx_stream_begin) and makes the
                /// buffer readable.
                pub fn rx_stream_stop(&self, dma: &impl ReceiveChannel, buffer: &mut [u8]) {
                    dma.stop_chain();
                    self.rx_stop();
                    dma.clear_interrupt(crate::dma::RxInterrupt::SucEof);
                    crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                }

                fn rx_launch(
                    &self,
                    dma: &impl ReceiveChannel,
                    buffer: &mut [u8],
                    descriptors: &[Descriptor],
                    ring: bool,
                ) -> Result<(), Error> {
                    if buffer.is_empty() {
                        return Err(Error::Dma(crate::dma::Error::EmptyBuffer));
                    }
                    if buffer.len() > 4095 {
                        return Err(Error::FrameTooLong);
                    }
                    let bytes = buffer.as_mut_ptr() as *mut u8;
                    let len = buffer.len();
                    crate::cache::writeback_invalidate(bytes.cast(), len);

                    // The peripheral counts the bytes of a frame itself and
                    // signals the DMA when it has this many.
                    self.regs().rxeof_num.write_rx_eof_num(len as u32);
                    let chain = crate::dma::Chain::new(descriptors);
                    if ring {
                        chain.mount_ring(bytes, len).map_err(Error::Dma)?;
                    } else {
                        chain.mount(bytes, len).map_err(Error::Dma)?;
                    }

                    let r = self.regs();
                    r.rx_conf.trigger_rx_reset();
                    r.rx_conf.trigger_rx_fifo_reset();
                    dma.connect_to(Peripheral::$peri);
                    dma.reset_channel();
                    dma.clear_interrupt(crate::dma::RxInterrupt::SucEof);
                    dma.start_chain(chain.head());
                    r.int_clr.trigger_rx_done_int_clr();
                    r.int_clr.trigger_rx_hung_int_clr();
                    self.rx_start();
                    Ok(())
                }

                /// Waits for the frame started by
                /// [`rx_begin`](Self::rx_begin), stops the channel and makes the
                /// buffer readable again.
                pub fn rx_wait(
                    &self,
                    dma: &impl ReceiveChannel,
                    buffer: &mut [u8],
                ) -> Result<(), Error> {
                    let deadline = crate::delay::micros() + self.transfer_budget(buffer.len());
                    loop {
                        if dma.interrupt_pending(crate::dma::RxInterrupt::SucEof) {
                            break;
                        }
                        if crate::delay::micros() > deadline {
                            dma.stop_chain();
                            self.rx_stop();
                            // Whatever arrived is still worth seeing, so drop
                            // the stale cache lines even on the way out.
                            crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    dma.stop_chain();
                    dma.clear_interrupt(crate::dma::RxInterrupt::SucEof);
                    self.rx_stop();
                    // The engine writes memory, not the cache.
                    crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                    Ok(())
                }

                /// Receives one frame into `buffer`.
                pub fn rx_read(
                    &self,
                    dma: &impl ReceiveChannel,
                    buffer: &mut [u8],
                    descriptors: &[Descriptor],
                ) -> Result<(), Error> {
                    self.rx_begin(dma, buffer, descriptors)?;
                    self.rx_wait(dma, buffer)
                }

                /// Writes the transmit half's MCLK divider.
                ///
                /// The coefficients are double buffered, so the reference
                /// driver first switches to a small division and then writes
                /// the target one; the order of the fields matters as well.
                fn program_tx_divider(&self, clkrst: &HP_SYS_CLKRST, d: &Divider) {
                    clkrst.[<$tx_n_reg>].[<write_ $tx_n>](2);
                    clkrst.[<$tx_yn_reg>].[<clr_ $tx_yn>]();
                    clkrst.[<$tx_y_reg>].[<write_ $tx_y>](1);
                    clkrst.[<$tx_z_reg>].[<write_ $tx_z>](0);
                    clkrst.[<$tx_x_reg>].[<write_ $tx_x>](0);
                    if d.yn1 {
                        clkrst.[<$tx_yn_reg>].[<set_ $tx_yn>]();
                    } else {
                        clkrst.[<$tx_yn_reg>].[<clr_ $tx_yn>]();
                    }
                    clkrst.[<$tx_z_reg>].[<write_ $tx_z>](d.z);
                    clkrst.[<$tx_y_reg>].[<write_ $tx_y>](d.y);
                    clkrst.[<$tx_x_reg>].[<write_ $tx_x>](d.x);
                    clkrst.[<$tx_n_reg>].[<write_ $tx_n>](d.n);
                }

                /// Writes the receive half's MCLK divider.
                fn program_rx_divider(&self, clkrst: &HP_SYS_CLKRST, d: &Divider) {
                    clkrst.[<$rx_n_reg>].[<write_ $rx_n>](2);
                    clkrst.[<$rx_yn_reg>].[<clr_ $rx_yn>]();
                    clkrst.[<$rx_y_reg>].[<write_ $rx_y>](1);
                    clkrst.[<$rx_z_reg>].[<write_ $rx_z>](0);
                    clkrst.[<$rx_x_reg>].[<write_ $rx_x>](0);
                    if d.yn1 {
                        clkrst.[<$rx_yn_reg>].[<set_ $rx_yn>]();
                    } else {
                        clkrst.[<$rx_yn_reg>].[<clr_ $rx_yn>]();
                    }
                    clkrst.[<$rx_z_reg>].[<write_ $rx_z>](d.z);
                    clkrst.[<$rx_y_reg>].[<write_ $rx_y>](d.y);
                    clkrst.[<$rx_x_reg>].[<write_ $rx_x>](d.x);
                    clkrst.[<$rx_n_reg>].[<write_ $rx_n>](d.n);
                }

                /// Programs the MCLK divider, its owner and, in master mode,
                /// the BCK divider.
                fn program_clock(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    layout: &Layout,
                    owner: MclkOwner,
                ) -> Result<Clock, Error> {
                    let bclk_hz = layout.bclk_hz();
                    let mclk_hz = layout.mclk_hz();
                    if bclk_hz == 0 || mclk_hz == 0 {
                        return Err(Error::RateUnsupported);
                    }
                    let bck_divisor = mclk_hz / bclk_hz;
                    // The divider field cannot hold 1, so the smallest usable
                    // divisor is 2; below 8 a slave samples unreliably.
                    if bck_divisor < 2 || mclk_hz % bclk_hz != 0 {
                        return Err(Error::RateUnsupported);
                    }
                    let divisor = mclk_divider(self.source_hz(), mclk_hz)?;
                    // `BCK_DIV_NUM` is six bits wide, so 64 is the largest
                    // divisor it can hold.
                    if bck_divisor > 64 {
                        return Err(Error::RateUnsupported);
                    }
                    self.bclk_hz.set(bclk_hz);

                    // A slave half takes BCK and WS from the bus, so it must
                    // not touch which half owns MCLK or the BCK divider — but it
                    // still needs its own module clock, at least eight BCK
                    // periods per bit, or it cannot sample the line.
                    if !layout.master && mclk_hz < bclk_hz * 8 {
                        return Err(Error::SlaveClockTooSlow);
                    }
                    match owner {
                        MclkOwner::Transmit => {
                            if layout.master {
                                clkrst.[<$mst_reg>].[<set_ $mst>]();
                                self.regs().tx_conf.write_tx_bck_div_num(bck_divisor - 1);
                            }
                            self.program_tx_divider(clkrst, &divisor);
                        }
                        MclkOwner::Receive => {
                            if layout.master {
                                clkrst.[<$mst_reg>].[<clr_ $mst>]();
                                self.regs().rx_conf.write_rx_bck_div_num(bck_divisor - 1);
                            }
                            self.program_rx_divider(clkrst, &divisor);
                        }
                    }

                    let realized_mclk = ((u64::from(self.source_hz())
                        * u64::from(divisor.denominator))
                        / (u64::from(divisor.integer) * u64::from(divisor.denominator)
                            + u64::from(divisor.numerator))) as u32;
                    Ok(Clock {
                        source_hz: self.source_hz(),
                        mclk_hz: realized_mclk,
                        bclk_hz,
                        bck_divisor,
                        mclk_integer: divisor.integer,
                        mclk_numerator: divisor.numerator,
                        mclk_denominator: divisor.denominator,
                    })
                }

                /// How long a transfer of `bytes` may take before the driver
                /// gives up on it.
                fn transfer_budget(&self, bytes: usize) -> u64 {
                    // Eight bit clocks per byte plus a fixed grace period, so a
                    // stopped clock still fails in bounded time.
                    (bytes as u64 * 8 * 1_000_000) / u64::from(self.bclk_hz.get().max(1))
                        + 50_000
                }
            }
        }
    };
}

/// Which half owns the shared MCLK divider.
enum MclkOwner {
    Transmit,
    Receive,
}

// The clock fields live in the shared `HP_SYS_CLKRST` block, and each instance
// spreads its dividers over different `PERI_CLK_CTRL` registers.
impl_i2s!(
    I2s0,
    I2S0,
    i2s0,
    I2S0,
    I2s0,
    reg_i2s0_apb_clk_en,
    reg_rst_en_i2s0_apb,
    lp_aonclkrst_hp_pad_i2s0_mclk_en,
    peri_clk_ctrl11,
    reg_i2s0_rx_clk_en,
    reg_i2s0_rx_clk_src_sel,
    peri_clk_ctrl13,
    reg_i2s0_tx_clk_en,
    reg_i2s0_tx_clk_src_sel,
    peri_clk_ctrl12,
    reg_i2s0_rx_div_n,
    peri_clk_ctrl12,
    reg_i2s0_rx_div_x,
    peri_clk_ctrl12,
    reg_i2s0_rx_div_y,
    peri_clk_ctrl13,
    reg_i2s0_rx_div_z,
    peri_clk_ctrl13,
    reg_i2s0_rx_div_yn1,
    peri_clk_ctrl13,
    reg_i2s0_tx_div_n,
    peri_clk_ctrl13,
    reg_i2s0_tx_div_x,
    peri_clk_ctrl14,
    reg_i2s0_tx_div_y,
    peri_clk_ctrl14,
    reg_i2s0_tx_div_z,
    peri_clk_ctrl14,
    reg_i2s0_tx_div_yn1,
    peri_clk_ctrl14,
    reg_i2s0_mst_clk_sel
);

impl_i2s!(
    I2s1,
    I2S1,
    i2s1,
    I2S1,
    I2s1,
    reg_i2s1_apb_clk_en,
    reg_rst_en_i2s1_apb,
    lp_aonclkrst_hp_pad_i2s1_mclk_en,
    peri_clk_ctrl14,
    reg_i2s1_rx_clk_en,
    reg_i2s1_rx_clk_src_sel,
    peri_clk_ctrl15,
    reg_i2s1_tx_clk_en,
    reg_i2s1_tx_clk_src_sel,
    peri_clk_ctrl14,
    reg_i2s1_rx_div_n,
    peri_clk_ctrl15,
    reg_i2s1_rx_div_x,
    peri_clk_ctrl15,
    reg_i2s1_rx_div_y,
    peri_clk_ctrl15,
    reg_i2s1_rx_div_z,
    peri_clk_ctrl15,
    reg_i2s1_rx_div_yn1,
    peri_clk_ctrl16,
    reg_i2s1_tx_div_n,
    peri_clk_ctrl16,
    reg_i2s1_tx_div_x,
    peri_clk_ctrl16,
    reg_i2s1_tx_div_y,
    peri_clk_ctrl17,
    reg_i2s1_tx_div_z,
    peri_clk_ctrl17,
    reg_i2s1_tx_div_yn1,
    peri_clk_ctrl17,
    reg_i2s1_mst_clk_sel
);

impl_i2s!(
    I2s2,
    I2S2,
    i2s2,
    I2S2,
    I2s2,
    reg_i2s2_apb_clk_en,
    reg_rst_en_i2s2_apb,
    lp_aonclkrst_hp_pad_i2s2_mclk_en,
    peri_clk_ctrl17,
    reg_i2s2_rx_clk_en,
    reg_i2s2_rx_clk_src_sel,
    peri_clk_ctrl18,
    reg_i2s2_tx_clk_en,
    reg_i2s2_tx_clk_src_sel,
    peri_clk_ctrl17,
    reg_i2s2_rx_div_n,
    peri_clk_ctrl17,
    reg_i2s2_rx_div_x,
    peri_clk_ctrl18,
    reg_i2s2_rx_div_y,
    peri_clk_ctrl18,
    reg_i2s2_rx_div_z,
    peri_clk_ctrl18,
    reg_i2s2_rx_div_yn1,
    peri_clk_ctrl18,
    reg_i2s2_tx_div_n,
    peri_clk_ctrl19,
    reg_i2s2_tx_div_x,
    peri_clk_ctrl19,
    reg_i2s2_tx_div_y,
    peri_clk_ctrl19,
    reg_i2s2_tx_div_z,
    peri_clk_ctrl19,
    reg_i2s2_tx_div_yn1,
    peri_clk_ctrl19,
    reg_i2s2_mst_clk_sel
);
