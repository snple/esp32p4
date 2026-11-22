//! UART (Universal Asynchronous Receiver/Transmitter).
//!
//! UART0-UART4 are the five high-performance UARTs. Their signals are part of
//! the GPIO matrix, so every GPIO can be TX, RX, CTS or RTS — the driver does
//! not restrict the choice the way a fixed-function MCU would. (Peripherals
//! without a matrix signal, such as USB or the EMAC PHY, are the exception;
//! see [`crate::gpio::pins`].)
//!
//! # Features
//!
//! - Polled transmit and receive, blocking or not
//! - DMA transmit and receive through the UHCI and the GDMA-AHB ([`Uhci`])
//! - 5-8 data bits, 1 / 1.5 / 2 stop bits, even/odd parity
//! - Baud rates from a selectable clock source: crystal (40 MHz),
//!   `RC_FAST` (17.5 MHz) or `PLL_F80M` (80 MHz, the reset default and the one
//!   that survives a CPU frequency change)
//! - Signal inversion, loopback and break transmission
//! - Hardware flow control (CTS/RTS) with a configurable RTS threshold
//! - Per-source interrupt enable, status and clear
//!
//! The LP UART shares this register layout but has its own clock tree
//! (`LP_PERI`) and is reached through the LP GPIO matrix, which this HAL does
//! not model yet; it is tracked in the roadmap.
//!
//! # Pin Mapping
//!
//! The signal indices come from [`crate::gpio::signal`]:
//!
//! | UART | TX | RX | CTS | RTS |
//! |------|----|----|-----|-----|
//! | 0-4  | `UARTn_TXD_PAD_OUT` | `UARTn_RXD_PAD_IN` | `UARTn_CTS_PAD_IN` | `UARTn_RTS_PAD_OUT` |
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::Pins;
//! use esp32p4_hal::uart::{Uart1, UartConfig};
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//!
//! let mut uart = Uart1::init(dp.UART1, &UartConfig::default().baudrate(115_200), &dp.HP_SYS_CLKRST);
//! uart.set_pins(pins.pin::<20>(), pins.pin::<21>());
//!
//! uart.write(b"hello\r\n");
//! uart.flush();
//!
//! loop {
//!     if let Some(byte) = uart.read_byte() {
//!         uart.write_byte(byte);
//!     }
//! }
//! ```
//!
//! # DMA
//!
//! All five UARTs reach the GDMA through one [`Uhci`], the packet engine that
//! sits between the GDMA-AHB channels and the UART FIFOs. Its
//! `CONF0.UART_SEL` picks the port, so one UART at a time moves data by DMA; a
//! program that uses two takes the same handle to both, and every transfer
//! re-attaches it.
//!
//! ```ignore
//! use esp32p4_hal::dma::{AhbDma, Descriptor};
//! use esp32p4_hal::uart::Uhci;
//!
//! let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
//! let uhci = Uhci::init(dp.UHCI0, &dp.HP_SYS_CLKRST);
//! let descriptors = [const { Descriptor::new() }; 2];
//!
//! uart.write_dma(&uhci, &dma.tx_channel::<0>(), b"hello", &descriptors, 10_000)?;
//!
//! let mut buffer = [0u8; 5];
//! uart.read_dma(&uhci, &dma.rx_channel::<0>(), &mut buffer, &descriptors, 10_000)?;
//! ```
//!
//! The engine is packet oriented: it only hands bytes to the DMA when a packet
//! ends, and the end has to come from one of its framing conditions. This
//! module frames receives by length, the only byte-exact one, which makes
//! [`Uart1::read_dma`] the DMA twin of [`Uart1::read_timeout`]: exactly
//! `buffer.len()` bytes, or [`Error::Timeout`]. One receive is therefore at
//! most [`DMA_MAX_PACKET`] bytes; split longer transfers.
//!
//! Descriptors and buffers have to live in memory the engine reaches — internal
//! SRAM for now, see [`crate::dma`] — and one descriptor covers 4095 bytes.
//! [`Uart1::rx_begin_dma`] / [`Uart1::tx_begin_dma`] arming one direction and
//! waiting later is what lets a receive run while the transmit is still going:
//! with a 128-byte FIFO, draining after a large transmission would lose it.

use esp32p4_pac::{HP_SYS_CLKRST, UART0, UART1, UART2, UART3, UART4, UHCI0, uhci0};
use pastey::paste;

use crate::dma::{self, Descriptor, Peripheral, ReceiveChannel, TransmitChannel};
use crate::gpio::{InputSignal, OutputSignal, PinOps};
use crate::spin;

/// Number of entries in the TX and RX FIFOs.
const FIFO_DEPTH: u32 = 128;

/// Largest buffer one DMA receive can describe.
///
/// The decoder ends a packet after `UHCI_PKT_THRES` bytes, and that field is 13
/// bits wide. Longer transfers are split by the caller.
pub const DMA_MAX_PACKET: usize = 0x1fff;

/// Width of the `clkdiv_sync.clkdiv` field, in bits: the largest integer
/// divisor the fractional divider supports.
const CLKDIV_MAX: u64 = 0xfff;

/// Number of data bits per frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum DataBits {
    /// 5 data bits.
    Five = 0,
    /// 6 data bits.
    Six = 1,
    /// 7 data bits.
    Seven = 2,
    /// 8 data bits.
    #[default]
    Eight = 3,
}

/// Number of stop bits per frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum StopBits {
    /// 1 stop bit.
    #[default]
    One = 1,
    /// 1.5 stop bits.
    OneAndHalf = 2,
    /// 2 stop bits.
    Two = 3,
}

/// Frame parity.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity bit.
    #[default]
    None,
    /// Even parity (`parity = 0` with the parity bit enabled).
    Even,
    /// Odd parity (`parity = 1` with the parity bit enabled).
    Odd,
}

/// Clock feeding a UART's baud-rate generator.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClock {
    /// `XTAL_CLK`, 40 MHz.
    Xtal,
    /// `RC_FAST_CLK`, nominally 17.5 MHz.
    RcFast,
    /// `PLL_F80M_CLK`, 80 MHz. The reset default, and unaffected by a CPU
    /// frequency change.
    #[default]
    PllF80m,
}

impl UartClock {
    /// Nominal frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Xtal => crate::clock::XTAL_HZ,
            Self::RcFast => crate::clock::RC_FAST_HZ,
            Self::PllF80m => 80_000_000,
        }
    }

    /// Value of `PERI_CLK_CTRL11x.UARTn_CLK_SRC_SEL`.
    const fn select(self) -> u32 {
        match self {
            Self::Xtal => 0,
            Self::RcFast => 1,
            Self::PllF80m => 2,
        }
    }
}

/// UART configuration.
///
/// The `Default` matches the peripheral's reset state (8 data bits, no parity,
/// one stop bit, both FIFO thresholds at 96) except for the baud rate, which is
/// [`115_200`](Self::baudrate) rather than the reset divider value, and the
/// clock source, which is [`UartClock::PllF80m`].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UartConfig {
    /// Baud rate in bit/s.
    pub baudrate: u32,
    /// Data bits per frame.
    pub data_bits: DataBits,
    /// Stop bits per frame.
    pub stop_bits: StopBits,
    /// Frame parity.
    pub parity: Parity,
    /// Clock source for the baud-rate generator.
    pub clock: UartClock,
    /// Invert the TX line (idle low instead of idle high).
    pub invert_tx: bool,
    /// Invert the RX line.
    pub invert_rx: bool,
    /// Connect TX back to RX internally.
    pub loopback: bool,
    /// Let RTS tell the peer to stop sending when the RX FIFO passes
    /// [`rts_threshold`](Self::rts_threshold).
    pub rts_flow_control: bool,
    /// Stop transmitting while CTS is de-asserted.
    pub cts_flow_control: bool,
    /// RX FIFO level that asserts RTS (`hwfc_conf_sync.rx_flow_thrhd`).
    pub rts_threshold: u8,
    /// RX FIFO level that raises the RX-full interrupt (`conf1.rxfifo_full_thrhd`).
    pub rx_fifo_threshold: u8,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            clock: UartClock::PllF80m,
            invert_tx: false,
            invert_rx: false,
            loopback: false,
            rts_flow_control: false,
            cts_flow_control: false,
            rts_threshold: 96,
            rx_fifo_threshold: 96,
        }
    }
}

impl UartConfig {
    /// Sets the baud rate.
    pub const fn baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }

    /// Sets the frame format.
    pub const fn format(
        mut self,
        data_bits: DataBits,
        stop_bits: StopBits,
        parity: Parity,
    ) -> Self {
        self.data_bits = data_bits;
        self.stop_bits = stop_bits;
        self.parity = parity;
        self
    }

    /// Selects the baud-rate clock source.
    pub const fn clock(mut self, clock: UartClock) -> Self {
        self.clock = clock;
        self
    }

    /// Inverts the TX and/or RX line.
    pub const fn invert(mut self, tx: bool, rx: bool) -> Self {
        self.invert_tx = tx;
        self.invert_rx = rx;
        self
    }

    /// Connects TX to RX internally.
    pub const fn loopback(mut self, loopback: bool) -> Self {
        self.loopback = loopback;
        self
    }

    /// Enables hardware flow control; `threshold` is the RX level that
    /// de-asserts RTS.
    pub const fn flow_control(mut self, cts: bool, rts: bool, threshold: u8) -> Self {
        self.cts_flow_control = cts;
        self.rts_flow_control = rts;
        self.rts_threshold = threshold;
        self
    }
}

/// Receive errors.
///
/// The UART latches these per FIFO entry, not per byte, so they are reported by
/// [`Uart0::take_error`] rather than attached to a byte.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The receive FIFO overflowed; at least one byte was lost.
    Overrun,
    /// A frame was not terminated by a valid stop bit.
    Frame,
    /// The parity bit did not match.
    Parity,
    /// A break condition (long low level) was detected.
    Break,
    /// The caller's time limit expired before the transfer finished.
    Timeout,
    /// A DMA transfer failed; see [`crate::dma::Error`].
    Dma(dma::Error),
    /// A DMA receive asked for a buffer the decoder cannot frame.
    ///
    /// Its packet counter is 13 bits wide, so one receive is at most
    /// [`DMA_MAX_PACKET`] bytes; split larger transfers.
    BufferTooLong,
}

/// Interrupt sources.
///
/// Bit positions are those of `UART_INT_RAW`/`INT_ENA`/`INT_CLR`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Interrupt {
    /// The RX FIFO reached [`UartConfig::rx_fifo_threshold`].
    RxFifoFull = 0,
    /// The TX FIFO drained below `conf1.txfifo_empty_thrhd`.
    TxFifoEmpty = 1,
    /// A parity error was received.
    ParityError = 2,
    /// A framing error was received.
    FrameError = 3,
    /// The RX FIFO overflowed.
    RxFifoOverflow = 4,
    /// A break condition was received.
    BreakDetected = 7,
    /// The receiver saw no data for a character time (`tout_conf`).
    RxTimeout = 8,
    /// All data in the TX FIFO, including the shift register, was sent.
    TxDone = 14,
}

impl Interrupt {
    /// The mask of this source in the interrupt registers.
    #[inline]
    pub const fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

/// Integer and fractional parts of one baud-rate divider.
#[derive(Clone, Copy, Debug)]
struct BaudDivider {
    /// Pre-divider on the clock source (`UARTn_SCLK_DIV_NUM + 1`).
    predivider: u32,
    /// `clkdiv_sync.clkdiv`.
    integer: u32,
    /// `clkdiv_sync.clkdiv_frag`, in sixteenths.
    fraction: u32,
}

impl BaudDivider {
    /// Computes the divider for `source_hz` and `baudrate`, or `None` when the
    /// baud rate is out of range.
    ///
    /// The pre-divider keeps the fractional divider in range at very high
    /// baud rates; for the usual rates it stays at 1.
    fn new(source_hz: u32, baudrate: u32) -> Option<Self> {
        if baudrate == 0 {
            return None;
        }

        let source = u64::from(source_hz);
        let baud = u64::from(baudrate);
        let predivider = source.div_ceil(CLKDIV_MAX * baud).max(1);
        if predivider > 256 {
            return None;
        }

        let divider = (source << 4) / (baud * predivider);
        if divider >> 4 > CLKDIV_MAX {
            return None;
        }

        Some(Self {
            predivider: predivider as u32,
            integer: (divider >> 4) as u32,
            fraction: (divider & 0xf) as u32,
        })
    }

    /// The baud rate the hardware actually produces.
    fn actual_baud(&self, source_hz: u32) -> u32 {
        let divisor = u64::from(self.predivider) * u64::from(self.integer * 16 + self.fraction);
        ((u64::from(source_hz) << 4) / divisor) as u32
    }
}

/// The UHCI: the DMA front-end the five UARTs share.
///
/// `UHCI0` sits between a GDMA-AHB channel pair and the UART FIFOs: the
/// transmit channel feeds the TX FIFO, the receive channel drains the RX FIFO
/// into memory, and `CONF0.UART_SEL` decides which of UART0-UART4 is connected.
/// One UART at a time can move data by DMA, and it needs one transmit and one
/// receive channel from [`crate::dma::AhbDma`].
///
/// The handle only owns the UHCI registers; the channels stay with whoever
/// called [`AhbDma::tx_channel`](crate::dma::AhbDma::tx_channel) /
/// [`rx_channel`](crate::dma::AhbDma::rx_channel), and are passed to the
/// transfers below. Reattaching to another UART is part of every transfer, so a
/// program that uses two UARTs only has to wave the handle around.
///
/// # Example
///
/// ```ignore
/// use esp32p4_hal::dma::{AhbDma, Descriptor};
/// use esp32p4_hal::uart::{Uart1, Uhci};
///
/// let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
/// let uhci = Uhci::init(dp.UHCI0, &dp.HP_SYS_CLKRST);
/// let descriptors = [const { Descriptor::new() }; 2];
///
/// let mut buffer = [0u8; 64];
/// uart.write_dma(&uhci, &dma.tx_channel::<0>(), b"hello", &descriptors, 10_000)?;
/// let received = uart.read_dma(&uhci, &dma.rx_channel::<0>(), &mut buffer, &descriptors, 10_000)?;
/// ```
pub struct Uhci {
    uhci: UHCI0,
}

impl Uhci {
    /// Takes ownership of the UHCI and brings it up for raw byte streaming.
    ///
    /// Enables the bus and register clocks, pulses the reset and clears the
    /// HCI framing controls: no separator characters, no packet headers, no
    /// CRC. Both the encoder and the decoder then pass bytes through unchanged.
    pub fn init(uhci: UHCI0, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl1.set_reg_uhci_sys_clk_en();
        clkrst.soc_clk_ctrl2.set_reg_uhci_apb_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_uhci();
        clkrst.hp_rst_en1.clr_reg_rst_en_uhci();

        let this = Self { uhci };
        this.configure();
        this
    }

    /// Resets the whole controller and brings it back to the [`init`](Self::init)
    /// state.
    ///
    /// Drops the encoder and decoder state, including the bytes a receive had
    /// already taken out of the UART FIFO; the port selection goes back to
    /// UART0. [`rx_wait_dma`](Uart1::rx_wait_dma) calls this when a receive
    /// gives up mid-packet.
    pub fn reset(&self) {
        // SAFETY: the reset line is a single write-only bit that nothing else
        // in this HAL takes ownership of, and the clock gate is already open.
        let clkrst = unsafe { HP_SYS_CLKRST::steal() };
        clkrst.hp_rst_en1.set_reg_rst_en_uhci();
        clkrst.hp_rst_en1.clr_reg_rst_en_uhci();
        self.configure();
    }

    /// Programs the framing controls for a raw byte stream.
    fn configure(&self) {
        let r = self.regs();
        // The register file needs its clock even when no transfer runs.
        r.conf0.write_bits(0);
        r.conf0.set_clk_en();
        r.conf1.write_bits(0);
        // A separator character would make the decoder drop bytes; the
        // reference driver disables all escaping for the same reason.
        r.escape_conf.write_bits(0);
        r.esc_conf0.write_bits(0);
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The UHCI must have been initialised with [`init`](Self::init), and the
    /// caller must not start a transfer concurrently with the owner.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            uhci: unsafe { UHCI0::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &uhci0::RegisterBlock {
        &self.uhci
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> UHCI0 {
        self.uhci
    }

    /// The UART number the UHCI is attached to, `0` to `4`.
    #[inline]
    pub fn attached_uart(&self) -> u8 {
        self.regs().conf0.uart_sel() as u8
    }

    /// Points the UHCI at `uart`.
    ///
    /// Nothing is reset here: a receive armed on this UART has to survive the
    /// transmit call that follows it. Each direction resets its own state
    /// machine when it is armed.
    fn attach(&self, uart: u8) {
        assert!(uart <= 4, "the UHCI only drives UART0-UART4");
        let r = self.regs();
        if r.conf0.uart_sel() == u32::from(uart) {
            return;
        }
        r.conf0.write_uart_sel(u32::from(uart));
    }

    /// Restarts the decoder, dropping a half-received packet.
    ///
    /// A receive that timed out mid-packet leaves its byte counter loaded;
    /// without this the next packet would be assembled from the old bytes.
    #[inline]
    fn reset_decoder(&self) {
        let conf0 = &self.regs().conf0;
        conf0.set_rx_rst();
        conf0.clr_rx_rst();
    }

    /// Restarts the encoder.
    #[inline]
    fn reset_encoder(&self) {
        let conf0 = &self.regs().conf0;
        conf0.set_tx_rst();
        conf0.clr_tx_rst();
    }

    /// Makes the decoder end a receive packet after exactly `len` bytes.
    ///
    /// The decoder is packet oriented: it only hands bytes to the DMA when a
    /// packet ends, and the end comes from one of its EOF conditions. A length
    /// is the only one that is byte-exact — the idle detector can close a packet
    /// one byte early — so receives are framed by `len`, and `HEAD_EN` stays off
    /// so the configured value is used rather than a header.
    #[inline]
    fn set_rx_packet_length(&self, len: usize) {
        let r = self.regs();
        r.pkt_thres.write_pkt_thrs(len as u32);
        r.conf0.set_len_eof_en();
        r.conf0.clr_uart_idle_eof_en();
    }
}

/// Bytes the receive engine stored, up to and including the descriptor that
/// carries the EOF flag.
///
/// The engine writes the byte count into each finished descriptor's length
/// field, so the sum is what arrived; descriptors past the EOF are still
/// mounted with their capacity and must not be counted.
fn received_bytes(chain: &dma::Chain<'_>) -> usize {
    let mut total = 0;
    for descriptor in chain.descriptors() {
        total += descriptor.length();
        if descriptor.eof() {
            break;
        }
    }
    total
}

/// Generates one UART type from the shared register layout.
macro_rules! impl_uart {
    ($Name:ident, $PAC:ident, $mod:ident, $NUM:literal, $tx:ident, $rx:ident, $cts:ident, $rts:ident) => {
        /// UART driver owning the PAC peripheral.
        pub struct $Name {
            uart: $PAC,
        }

        impl $Name {
            /// UART number, 0-4.
            pub const NUM: u8 = $NUM;

            /// Obtains a handle without ownership checks.
            ///
            /// Useful in an interrupt handler, where the owning instance lives
            /// in `main()`. Only the `&self` API is reachable through
            /// [`Steal`](crate::Steal).
            ///
            /// # Safety
            ///
            /// The caller must ensure the UART was initialised and that the
            /// returned handle is not used to reconfigure it concurrently.
            #[inline]
            pub unsafe fn steal() -> crate::Steal<Self> {
                crate::Steal(Self {
                    uart: unsafe { $PAC::steal() },
                })
            }

            /// The underlying register block.
            #[inline]
            pub fn regs(&self) -> &esp32p4_pac::$mod::RegisterBlock {
                &self.uart
            }

            /// Returns the PAC peripheral.
            #[inline]
            pub fn release(self) -> $PAC {
                self.uart
            }

            /// Pushes the shadowed configuration registers into the UART's core
            /// clock domain.
            ///
            /// `CONF0`/`CONF1`/`CLKDIV` writes only take effect after
            /// `REG_UPDATE.REG_UPDATE` is set; the bit clears when the update
            /// has been applied.
            fn sync(&self) {
                let r = self.regs();
                r.reg_update.set_reg_update();
                while r.reg_update.reg_update() {
                    core::hint::spin_loop();
                }
            }

            // ================================================================
            // Frame format
            // ================================================================

            /// Applies `data_bits`, `stop_bits` and `parity`.
            ///
            /// Takes effect after the current frame; call
            /// [`flush`](Self::flush) first if a partial frame in flight must
            /// finish with the old format.
            pub fn set_format(&self, data_bits: DataBits, stop_bits: StopBits, parity: Parity) {
                let r = self.regs();
                r.conf0_sync.write_bit_num(data_bits as u32);
                r.conf0_sync.write_stop_bit_num(stop_bits as u32);
                match parity {
                    Parity::None => r.conf0_sync.clr_parity_en(),
                    Parity::Even => {
                        r.conf0_sync.clr_parity();
                        r.conf0_sync.set_parity_en();
                    }
                    Parity::Odd => {
                        r.conf0_sync.set_parity();
                        r.conf0_sync.set_parity_en();
                    }
                }
                self.sync();
            }

            /// Inverts the TX and/or RX line.
            ///
            /// An inverted line idles low; the receiver samples it through the
            /// same inversion, so both ends must agree.
            pub fn set_inverted(&self, tx: bool, rx: bool) {
                let r = self.regs();
                if tx {
                    r.conf0_sync.set_txd_inv();
                } else {
                    r.conf0_sync.clr_txd_inv();
                }
                if rx {
                    r.conf0_sync.set_rxd_inv();
                } else {
                    r.conf0_sync.clr_rxd_inv();
                }
                self.sync();
            }

            /// Connects the transmitter to the receiver inside the peripheral.
            pub fn set_loopback(&self, enabled: bool) {
                let r = self.regs();
                if enabled {
                    r.conf0_sync.set_loopback();
                } else {
                    r.conf0_sync.clr_loopback();
                }
                self.sync();
            }

            /// Drives a break condition on TX (keeps the line low).
            pub fn set_break(&self, enabled: bool) {
                let r = self.regs();
                if enabled {
                    r.conf0_sync.set_txd_brk();
                } else {
                    r.conf0_sync.clr_txd_brk();
                }
                self.sync();
            }

            /// Sets the receiver's idle-detection timeout, in bit times.
            ///
            /// This is what makes the [`Interrupt::RxTimeout`] fire after a
            /// partial frame, which is how protocols without a length prefix
            /// detect the end of a message.
            pub fn set_rx_timeout(&self, bit_times: u8) {
                let r = self.regs();
                let symbol_bits =
                    1 + match r.conf0_sync.bit_num() {
                        0 => 5,
                        1 => 6,
                        2 => 7,
                        _ => 8,
                    } + u32::from(r.conf0_sync.parity_en())
                        + if r.conf0_sync.stop_bit_num() > 1 {
                            2
                        } else {
                            1
                        };
                r.tout_conf_sync
                    .write_rx_tout_thrhd((symbol_bits * u32::from(bit_times)) & 0x3ff);
                r.tout_conf_sync.set_rx_tout_en();
                self.sync();
            }

            // ================================================================
            // Hardware flow control
            // ================================================================

            /// Enables CTS (stop transmitting) and/or RTS (ask the peer to
            /// stop) hardware flow control.
            ///
            /// `threshold` is the RX FIFO level at which RTS is de-asserted.
            pub fn set_flow_control(&self, cts: bool, rts: bool, threshold: u8) {
                let r = self.regs();
                r.hwfc_conf_sync.write_rx_flow_thrhd(u32::from(threshold));
                if rts {
                    r.hwfc_conf_sync.set_rx_flow_en();
                } else {
                    r.hwfc_conf_sync.clr_rx_flow_en();
                }
                if cts {
                    r.conf0_sync.set_tx_flow_en();
                } else {
                    r.conf0_sync.clr_tx_flow_en();
                }
                self.sync();
            }

            /// Whether the peer is ready to receive (CTS asserted).
            #[inline]
            pub fn cts_asserted(&self) -> bool {
                !self.regs().status.ctsn()
            }

            /// Sets the RX FIFO level used by [`Interrupt::RxFifoFull`] and the
            /// TX FIFO level used by [`Interrupt::TxFifoEmpty`].
            pub fn set_fifo_thresholds(&self, rx_level: u8, tx_level: u8) {
                let r = self.regs();
                r.conf1.write_rxfifo_full_thrhd(u32::from(rx_level));
                r.conf1.write_txfifo_empty_thrhd(u32::from(tx_level));
                self.sync();
            }

            // ================================================================
            // Data transfer
            // ================================================================

            /// Writes one byte, waiting for room in the TX FIFO.
            #[inline]
            pub fn write_byte(&self, byte: u8) {
                spin::spin_wait("UART TX FIFO space", || {
                    self.regs().status.txfifo_cnt() < FIFO_DEPTH
                });
                self.regs().fifo.write_bits(u32::from(byte));
            }

            /// Writes a buffer, waiting for room as needed.
            pub fn write(&self, data: &[u8]) {
                for &byte in data {
                    self.write_byte(byte);
                }
            }

            /// Writes one byte if the TX FIFO has room.
            ///
            /// Returns `false` when the FIFO is full.
            #[inline]
            pub fn try_write_byte(&self, byte: u8) -> bool {
                if self.regs().status.txfifo_cnt() >= FIFO_DEPTH {
                    return false;
                }
                self.regs().fifo.write_bits(u32::from(byte));
                true
            }

            /// Waits until the TX FIFO and the shift register are empty.
            pub fn flush(&self) {
                spin::spin_wait("UART transmitter idle", || self.is_tx_idle());
            }

            /// Whether the transmitter has nothing left to send.
            #[inline]
            pub fn is_tx_idle(&self) -> bool {
                let r = self.regs();
                r.status.txfifo_cnt() == 0 && r.fsm_status.st_utx_out() == 0
            }

            /// Number of bytes waiting in the RX FIFO.
            #[inline]
            pub fn rx_fifo_count(&self) -> u32 {
                self.regs().status.rxfifo_cnt()
            }

            /// Reads one byte without waiting.
            #[inline]
            pub fn read_byte(&self) -> Option<u8> {
                let r = self.regs();
                if r.status.rxfifo_cnt() == 0 {
                    return None;
                }
                Some(r.fifo.rxfifo_rd_byte() as u8)
            }

            /// Reads one byte, waiting for it to arrive.
            pub fn read_byte_blocking(&self) -> u8 {
                spin::spin_wait("UART RX data", || self.regs().status.rxfifo_cnt() > 0);
                self.regs().fifo.rxfifo_rd_byte() as u8
            }

            /// Reads up to `buf.len()` bytes without waiting.
            ///
            /// Returns the number of bytes stored in `buf`.
            pub fn read(&self, buf: &mut [u8]) -> usize {
                let mut count = 0;
                while count < buf.len() {
                    match self.read_byte() {
                        Some(byte) => {
                            buf[count] = byte;
                            count += 1;
                        }
                        None => break,
                    }
                }
                count
            }

            /// Reads exactly `buf.len()` bytes, waiting at most `timeout_us`
            /// microseconds in total.
            ///
            /// Returns [`Error::Timeout`] if the deadline passes first; the
            /// bytes read up to that point are in `buf`.
            pub fn read_timeout(&self, buf: &mut [u8], timeout_us: u64) -> Result<(), Error> {
                let deadline = crate::delay::micros() + timeout_us;
                for slot in buf.iter_mut() {
                    while self.regs().status.rxfifo_cnt() == 0 {
                        if crate::delay::micros() > deadline {
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    *slot = self.regs().fifo.rxfifo_rd_byte() as u8;
                }
                Ok(())
            }

            /// Drops whatever sits in the receive FIFO.
            ///
            /// Used after a DMA receive gave up: those bytes belong to a packet
            /// the caller will never see, and leaving them would prepend them to
            /// the next one.
            fn reset_rx_fifo(&self) {
                let conf0 = &self.regs().conf0_sync;
                // `conf0_sync` is shadowed and the reset line is level
                // sensitive: the pulse only happens if each edge is applied.
                conf0.set_rxfifo_rst();
                self.sync();
                conf0.clr_rxfifo_rst();
                self.sync();
            }

            // ================================================================
            // DMA transfers (UHCI + GDMA)
            // ================================================================

            /// Points the UHCI's transmit channel at `buffer` and starts it.
            ///
            /// Paired with [`tx_wait_dma`](Self::tx_wait_dma); most callers
            /// want [`write_dma`](Self::write_dma) instead. The buffer belongs
            /// to the engine until the wait returns, and needs
            /// [`descriptors`](Self::write_dma) for its `4095`-byte chunks.
            pub fn tx_begin_dma(
                &self,
                uhci: &Uhci,
                dma: &impl TransmitChannel,
                buffer: &[u8],
                descriptors: &[Descriptor],
            ) -> Result<(), Error> {
                if buffer.is_empty() {
                    return Ok(());
                }
                uhci.attach(Self::NUM);
                uhci.reset_encoder();

                let chain = dma::Chain::new(descriptors);
                // The trailing EOF is what tells the engine the phase ended.
                chain
                    .mount_eof(buffer.as_ptr() as *mut u8, buffer.len())
                    .map_err(Error::Dma)?;

                dma.connect_to(Peripheral::Uhci);
                dma.reset_channel();
                dma.clear_all_interrupts();
                crate::cache::writeback(buffer.as_ptr().cast(), buffer.len());
                dma.start_chain(chain.head());
                Ok(())
            }

            /// Waits for the transmission started by
            /// [`tx_begin_dma`](Self::tx_begin_dma) to leave the wire.
            ///
            /// The engine reports the end of the chain once it handed every
            /// byte to the UART, so this also waits for the transmitter to
            /// drain its FIFO and shift register.
            pub fn tx_wait_dma(
                &self,
                dma: &impl TransmitChannel,
                timeout_us: u64,
            ) -> Result<(), Error> {
                let deadline = crate::delay::micros() + timeout_us;
                loop {
                    if dma.interrupt_pending(dma::TxInterrupt::DscrErr) {
                        dma.stop_chain();
                        dma.clear_all_interrupts();
                        return Err(Error::Dma(dma::Error::DescriptorError));
                    }
                    if dma.interrupt_pending(dma::TxInterrupt::FifoOverflow)
                        || dma.interrupt_pending(dma::TxInterrupt::FifoUnderflow)
                    {
                        dma.stop_chain();
                        dma.clear_all_interrupts();
                        return Err(Error::Dma(dma::Error::Fifo));
                    }
                    // `TotalEof` marks the last descriptor of the chain, the
                    // one that carries the EOF flag the mount added.
                    if dma.interrupt_pending(dma::TxInterrupt::TotalEof) {
                        break;
                    }
                    if crate::delay::micros() > deadline {
                        dma.reset_channel();
                        dma.clear_all_interrupts();
                        return Err(Error::Timeout);
                    }
                    core::hint::spin_loop();
                }

                while !self.is_tx_idle() {
                    if crate::delay::micros() > deadline {
                        dma.reset_channel();
                        dma.clear_all_interrupts();
                        return Err(Error::Timeout);
                    }
                    core::hint::spin_loop();
                }
                dma.stop_chain();
                dma.clear_all_interrupts();
                Ok(())
            }

            /// Sends `buffer` through the UHCI and the GDMA and waits until it
            /// left the transmitter.
            ///
            /// `descriptors` needs `ceil(buffer.len() / 4095)` entries, and the
            /// buffer has to live in memory the engine reaches (internal SRAM
            /// for now, see [`crate::dma`]). `timeout_us` bounds the whole
            /// transfer, which is one bit time per bit plus the engine's
            /// start-up.
            pub fn write_dma(
                &self,
                uhci: &Uhci,
                dma: &impl TransmitChannel,
                buffer: &[u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<(), Error> {
                self.tx_begin_dma(uhci, dma, buffer, descriptors)?;
                self.tx_wait_dma(dma, timeout_us)
            }

            /// Points the UHCI's receive channel at `buffer` and starts it.
            ///
            /// The decoder is told to end the packet after `buffer.len()`
            /// bytes, so the engine stops once exactly that many arrived.
            /// Paired with [`rx_wait_dma`](Self::rx_wait_dma).
            ///
            /// A receive may be armed before the first byte arrives, and it
            /// survives a [`write_dma`](Self::write_dma) on the same UART: only
            /// a transfer that attaches the UHCI to a *different* UART resets
            /// the decoder, so do not overlap those.
            ///
            /// # Errors
            ///
            /// [`Error::BufferTooLong`] when `buffer` is longer than
            /// [`DMA_MAX_PACKET`].
            pub fn rx_begin_dma(
                &self,
                uhci: &Uhci,
                dma: &impl ReceiveChannel,
                buffer: &mut [u8],
                descriptors: &[Descriptor],
            ) -> Result<(), Error> {
                if buffer.is_empty() {
                    return Ok(());
                }
                if buffer.len() > DMA_MAX_PACKET {
                    return Err(Error::BufferTooLong);
                }
                uhci.attach(Self::NUM);
                uhci.reset_decoder();
                uhci.set_rx_packet_length(buffer.len());

                let chain = dma::Chain::new(descriptors);
                chain
                    .mount(buffer.as_mut_ptr(), buffer.len())
                    .map_err(Error::Dma)?;

                dma.connect_to(Peripheral::Uhci);
                dma.reset_channel();
                dma.clear_all_interrupts();
                // The engine writes memory, not the cache: drop whatever the
                // core put there so no stale line comes back later.
                crate::cache::writeback_invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                dma.start_chain(chain.head());
                Ok(())
            }

            /// Waits for the receive started by
            /// [`rx_begin_dma`](Self::rx_begin_dma) and makes `buffer`
            /// readable.
            ///
            /// Returns the number of bytes the engine stored, which is
            /// `buffer.len()` once the packet the decoder was told to expect
            /// arrived.
            ///
            /// A shorter transmission keeps the decoder from closing its
            /// packet, so this reports [`Error::Timeout`] when the deadline
            /// passes and gives the receive path a clean restart: the channel,
            /// the decoder and the receive FIFO are dropped, so the next packet
            /// starts from the first byte. Part of the missing packet may
            /// already be in `buffer` — with no packet end there is no length to
            /// report, so those bytes are the caller's to discard.
            pub fn rx_wait_dma(
                &self,
                uhci: &Uhci,
                dma: &impl ReceiveChannel,
                buffer: &mut [u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<usize, Error> {
                let deadline = crate::delay::micros() + timeout_us;
                let count = loop {
                    if dma.interrupt_pending(dma::RxInterrupt::ErrEof) {
                        dma.stop_chain();
                        dma.clear_all_interrupts();
                        crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                        return Err(Error::Dma(dma::Error::Peripheral));
                    }
                    if dma.interrupt_pending(dma::RxInterrupt::DscrErr) {
                        dma.stop_chain();
                        dma.clear_all_interrupts();
                        crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                        return Err(Error::Dma(dma::Error::DescriptorError));
                    }
                    if dma.interrupt_pending(dma::RxInterrupt::SucEof) {
                        break received_bytes(&dma::Chain::new(descriptors));
                    }
                    if dma.interrupt_pending(dma::RxInterrupt::DscrEmpty) {
                        // Every descriptor filled: the whole buffer is data.
                        break buffer.len();
                    }
                    if crate::delay::micros() > deadline {
                        dma.clear_all_interrupts();
                        // A plain stop leaves the channel's write offset where
                        // the aborted packet got to, so the next buffer would be
                        // filled from the middle. Reset the channel and drop the
                        // bytes the decoder has not handed over, or they would
                        // lead the next packet.
                        dma.reset_channel();
                        uhci.reset();
                        self.reset_rx_fifo();
                        crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                        return Err(Error::Timeout);
                    }
                    core::hint::spin_loop();
                };

                dma.stop_chain();
                dma.clear_all_interrupts();
                crate::cache::invalidate(buffer.as_mut_ptr().cast(), buffer.len());
                Ok(count)
            }

            /// Receives exactly `buffer.len()` bytes through the UHCI.
            ///
            /// The DMA twin of [`read_timeout`](Self::read_timeout): returns
            /// once the whole buffer was filled, or [`Error::Timeout`] when the
            /// deadline passes first.
            pub fn read_dma(
                &self,
                uhci: &Uhci,
                dma: &impl ReceiveChannel,
                buffer: &mut [u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<usize, Error> {
                self.rx_begin_dma(uhci, dma, buffer, descriptors)?;
                self.rx_wait_dma(uhci, dma, buffer, descriptors, timeout_us)
            }

            /// Returns and clears the first latched receive error.
            ///
            /// Errors are sticky until cleared, so a byte that follows a lost
            /// one still reports the loss.
            pub fn take_error(&self) -> Option<Error> {
                let r = self.regs();
                let raw = &r.int_raw;
                let error = if raw.rxfifo_ovf_int_raw() {
                    Error::Overrun
                } else if raw.frm_err_int_raw() {
                    Error::Frame
                } else if raw.parity_err_int_raw() {
                    Error::Parity
                } else if raw.brk_det_int_raw() {
                    Error::Break
                } else {
                    return None;
                };
                r.int_clr.write_bits(
                    Interrupt::RxFifoOverflow.mask()
                        | Interrupt::FrameError.mask()
                        | Interrupt::ParityError.mask()
                        | Interrupt::BreakDetected.mask(),
                );
                Some(error)
            }

            // ================================================================
            // Interrupts
            // ================================================================

            /// Enables `interrupt`.
            #[inline]
            pub fn enable_interrupt(&self, interrupt: Interrupt) {
                let r = self.regs();
                r.int_ena.modify_bits(|v| v | interrupt.mask());
            }

            /// Disables `interrupt`.
            #[inline]
            pub fn disable_interrupt(&self, interrupt: Interrupt) {
                let r = self.regs();
                r.int_ena.modify_bits(|v| v & !interrupt.mask());
            }

            /// Whether `interrupt` is enabled.
            #[inline]
            pub fn interrupt_enabled(&self, interrupt: Interrupt) -> bool {
                self.regs().int_ena.read_bits() & interrupt.mask() != 0
            }

            /// Whether `interrupt` is pending, regardless of whether it is
            /// enabled.
            #[inline]
            pub fn interrupt_pending(&self, interrupt: Interrupt) -> bool {
                self.regs().int_raw.read_bits() & interrupt.mask() != 0
            }

            /// Clears `interrupt`.
            #[inline]
            pub fn clear_interrupt(&self, interrupt: Interrupt) {
                self.regs().int_clr.write_bits(interrupt.mask());
            }

            /// Disables every interrupt source.
            pub fn disable_all_interrupts(&self) {
                self.regs().int_ena.write_bits(0);
            }

            // ================================================================
            // Pin routing
            // ================================================================

            /// Routes `tx` and `rx` to this UART.
            ///
            /// The RX pad gets a pull-up, so an unconnected or idle line reads
            /// as the mark level instead of floating.
            pub fn set_pins<TX: PinOps, RX: PinOps>(&self, mut tx: TX, mut rx: RX) {
                self.set_tx_pin(&mut tx);
                self.set_rx_pin(&mut rx);
            }

            /// Routes only the transmitter.
            pub fn set_tx_pin<TX: PinOps>(&self, tx: &mut TX) {
                tx.route_output_signal(OutputSignal::$tx);
            }

            /// Routes only the receiver.
            pub fn set_rx_pin<RX: PinOps>(&self, rx: &mut RX) {
                rx.route_input_signal(InputSignal::$rx);
                rx.set_pull_up_enabled(true);
            }

            /// Routes the CTS and RTS flow-control pins.
            pub fn set_flow_control_pins<CTS: PinOps, RTS: PinOps>(
                &self,
                mut cts: CTS,
                mut rts: RTS,
            ) {
                cts.route_input_signal(InputSignal::$cts);
                rts.route_output_signal(OutputSignal::$rts);
            }

            // ================================================================
            // Configuration shared with `init`
            // ================================================================

            /// Programs everything except the clock source and dividers, which
            /// depend on the instance's clock-tree registers.
            fn apply(&self, config: &UartConfig) {
                let r = self.regs();

                r.conf0_sync.write_bit_num(config.data_bits as u32);
                r.conf0_sync.write_stop_bit_num(config.stop_bits as u32);
                match config.parity {
                    Parity::None => r.conf0_sync.clr_parity_en(),
                    Parity::Even => {
                        r.conf0_sync.clr_parity();
                        r.conf0_sync.set_parity_en();
                    }
                    Parity::Odd => {
                        r.conf0_sync.set_parity();
                        r.conf0_sync.set_parity_en();
                    }
                }

                // IR mode and break stay off; inversion and loopback follow the
                // configuration.
                r.conf0_sync.clr_irda_en();
                if config.invert_tx {
                    r.conf0_sync.set_txd_inv();
                } else {
                    r.conf0_sync.clr_txd_inv();
                }
                if config.invert_rx {
                    r.conf0_sync.set_rxd_inv();
                } else {
                    r.conf0_sync.clr_rxd_inv();
                }
                if config.loopback {
                    r.conf0_sync.set_loopback();
                } else {
                    r.conf0_sync.clr_loopback();
                }
                r.conf0_sync.clr_txd_brk();

                r.hwfc_conf_sync
                    .write_rx_flow_thrhd(u32::from(config.rts_threshold));
                if config.rts_flow_control {
                    r.hwfc_conf_sync.set_rx_flow_en();
                } else {
                    r.hwfc_conf_sync.clr_rx_flow_en();
                }
                if config.cts_flow_control {
                    r.conf0_sync.set_tx_flow_en();
                } else {
                    r.conf0_sync.clr_tx_flow_en();
                }

                r.conf1
                    .write_rxfifo_full_thrhd(u32::from(config.rx_fifo_threshold));

                r.clk_conf.set_tx_sclk_en();
                r.clk_conf.set_rx_sclk_en();
                self.sync();

                // Empty the FIFOs last: enabling loopback or inversion can put a
                // glitch byte into the receiver, and that one would otherwise
                // sit in front of the first real frame. The reset line lives in
                // a shadowed register and is level sensitive, so it takes an
                // update for each edge.
                r.conf0_sync.set_rxfifo_rst();
                r.conf0_sync.set_txfifo_rst();
                self.sync();
                r.conf0_sync.clr_rxfifo_rst();
                r.conf0_sync.clr_txfifo_rst();
                self.sync();
            }
        }
    };
}

impl_uart!(
    Uart0,
    UART0,
    uart0,
    0,
    UART0_TXD_PAD_OUT,
    UART0_RXD_PAD_IN,
    UART0_CTS_PAD_IN,
    UART0_RTS_PAD_OUT
);
impl_uart!(
    Uart1,
    UART1,
    uart1,
    1,
    UART1_TXD_PAD_OUT,
    UART1_RXD_PAD_IN,
    UART1_CTS_PAD_IN,
    UART1_RTS_PAD_OUT
);
impl_uart!(
    Uart2,
    UART2,
    uart2,
    2,
    UART2_TXD_PAD_OUT,
    UART2_RXD_PAD_IN,
    UART2_CTS_PAD_IN,
    UART2_RTS_PAD_OUT
);
impl_uart!(
    Uart3,
    UART3,
    uart3,
    3,
    UART3_TXD_PAD_OUT,
    UART3_RXD_PAD_IN,
    UART3_CTS_PAD_IN,
    UART3_RTS_PAD_OUT
);
impl_uart!(
    Uart4,
    UART4,
    uart4,
    4,
    UART4_TXD_PAD_OUT,
    UART4_RXD_PAD_IN,
    UART4_CTS_PAD_IN,
    UART4_RTS_PAD_OUT
);

/// Generates `init` and the baud-rate API for one UART.
///
/// The clock-tree fields are named after the instance, so the macro takes the
/// instance number and the two `PERI_CLK_CTRL11x` registers that hold its
/// source select / gate and its pre-divider.
///
/// `prediv_ctrl` is one register after `clk_ctrl`: UART0 has its gate and
/// source select in `PERI_CLK_CTRL110` and its pre-divider in
/// `PERI_CLK_CTRL111`, UART1 in 111/112, and so on.
macro_rules! impl_uart_clock {
    ($Name:ident, $PAC:ident, $num:literal, $clk_ctrl:ident, $prediv_ctrl:ident) => {
        paste! {
            impl $Name {
                /// Initialises the UART and applies `config`.
                ///
                /// `clkrst` is only borrowed: the clock tree stays with the
                /// caller. Pins are routed separately, through
                /// [`Self::set_pins`].
                ///
                /// # Panics
                ///
                /// Panics if `config.baudrate` cannot be produced from
                /// `config.clock`.
                pub fn init(uart: $PAC, config: &UartConfig, clkrst: &HP_SYS_CLKRST) -> Self {
                    let uart = Self { uart };
                    let divider = BaudDivider::new(config.clock.hz(), config.baudrate)
                        .unwrap_or_else(|| panic!("unsupported UART baud rate: {}", config.baudrate));

                    // Bus clock, reset pulse, then the controller clock.
                    clkrst.soc_clk_ctrl1.[<set_reg_uart $num _sys_clk_en>]();
                    clkrst.soc_clk_ctrl2.[<set_reg_uart $num _apb_clk_en>]();
                    clkrst.hp_rst_en1.[<set_reg_rst_en_uart $num _apb>]();
                    clkrst.hp_rst_en1.[<clr_reg_rst_en_uart $num _apb>]();
                    clkrst.$clk_ctrl.[<set_reg_uart $num _clk_en>]();
                    clkrst.$clk_ctrl.[<write_reg_uart $num _clk_src_sel>](config.clock.select());
                    clkrst.$prediv_ctrl.[<write_reg_uart $num _sclk_div_num>](divider.predivider - 1);

                    let r = uart.regs();
                    r.clkdiv_sync.write_clkdiv(divider.integer);
                    r.clkdiv_sync.write_clkdiv_frag(divider.fraction);

                    uart.apply(config);
                    uart
                }

                /// Reconfigures the baud rate, keeping the frame format.
                ///
                /// Returns the baud rate the hardware produces, which differs
                /// from the request when the divider has to round.
                ///
                /// # Panics
                ///
                /// Panics if `baudrate` cannot be produced from `clock`.
                pub fn set_baudrate(
                    &self,
                    baudrate: u32,
                    clock: UartClock,
                    clkrst: &HP_SYS_CLKRST,
                ) -> u32 {
                    let divider = BaudDivider::new(clock.hz(), baudrate)
                        .unwrap_or_else(|| panic!("unsupported UART baud rate: {baudrate}"));

                    let r = self.regs();
                    r.clkdiv_sync.write_clkdiv(divider.integer);
                    r.clkdiv_sync.write_clkdiv_frag(divider.fraction);
                    clkrst.$prediv_ctrl.[<write_reg_uart $num _sclk_div_num>](divider.predivider - 1);
                    self.sync();

                    divider.actual_baud(clock.hz())
                }

                /// The baud rate currently programmed, assuming `clock` drives
                /// the baud-rate generator.
                pub fn baudrate(&self, clock: UartClock, clkrst: &HP_SYS_CLKRST) -> u32 {
                    let divider = BaudDivider {
                        predivider: clkrst.$prediv_ctrl.[<reg_uart $num _sclk_div_num>]() + 1,
                        integer: self.regs().clkdiv_sync.clkdiv(),
                        fraction: self.regs().clkdiv_sync.clkdiv_frag(),
                    };
                    divider.actual_baud(clock.hz())
                }
            }
        }
    };
}

impl_uart_clock!(Uart0, UART0, 0, peri_clk_ctrl110, peri_clk_ctrl111);
impl_uart_clock!(Uart1, UART1, 1, peri_clk_ctrl111, peri_clk_ctrl112);
impl_uart_clock!(Uart2, UART2, 2, peri_clk_ctrl112, peri_clk_ctrl113);
impl_uart_clock!(Uart3, UART3, 3, peri_clk_ctrl113, peri_clk_ctrl114);
impl_uart_clock!(Uart4, UART4, 4, peri_clk_ctrl114, peri_clk_ctrl115);
