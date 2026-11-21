//! TWAI (CAN 2.0B) controllers.
//!
//! The ESP32-P4 has three TWAI controllers. Each is a classic CAN 2.0B
//! controller (standard and extended identifiers, data and remote frames, up to
//! eight payload bytes); the peripheral has no CAN-FD support, so neither has
//! this driver.
//!
//! # Features
//!
//! - Bit timing computed from the 40 MHz crystal: one call takes a bit rate and
//!   a sample point and fills in the prescaler and both phase segments
//! - Modes: normal, listen-only (never acknowledges, never sends) and self-test
//!   (never acknowledges, so a node without a transceiver raises no acknowledge
//!   error)
//! - One acceptance filter with a 32-bit code and mask, plus helpers for
//!   standard and extended identifiers
//! - Transmission with optional self-reception and single-shot, receive from the
//!   thirteen-byte RX buffer, and the RX FIFO depth
//! - Status, error state, error counters, and per-source interrupts, with
//!   [`Twai0::SOURCE`] / [`Twai1::SOURCE`] / [`Twai2::SOURCE`] for
//!   [`crate::interrupt::claim`]
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
//! use esp32p4_hal::gpio::{PinOps, Pins};
//! use esp32p4_hal::twai::{Config, Filter, Frame, Id, Mode, Timing, Twai0};
//!
//! // Without a transceiver: drive TX out of one pad and take RX back in from
//! // the same pad, so the controller hears what it sends.
//! let mut line = pins.pin::<4>();
//! line.route_output_signal(OutputSignal::TWAI0_TX_PAD_OUT);
//! line.route_input_signal(InputSignal::TWAI0_RX_PAD_IN);
//!
//! let twai = Twai0::init(dp.TWAI0, &dp.HP_SYS_CLKRST);
//! twai.configure(&Config {
//!     timing: Timing::for_bitrate(esp32p4_hal::clock::XTAL_HZ, 500_000, 875)?,
//!     mode: Mode::SelfTest,
//!     filter: Filter::AcceptAll,
//! });
//! twai.start();
//!
//! let mut frame = Frame::data(Id::Standard(0x123), &[1, 2, 3])?;
//! frame.self_reception = true;
//! frame.single_shot = true;
//! twai.transmit(&frame)?;
//! twai.wait_transmitted();
//! let received = twai.receive()?;
//! ```
//!
//! # Hardware Notes
//!
//! - Every register is one byte wide at bits `[7:0]`; the upper bits read back
//!   as zero and are ignored on write.
//! - The transmit buffer, the receive buffer and the acceptance filter *share*
//!   one address window (`0x40`–`0x70`): in reset mode it is the filter's code
//!   and mask registers, otherwise it is the thirteen-byte frame buffer. This
//!   driver writes the filter while reset mode is still set, which is also the
//!   only mode the TRM allows it in.
//! - The frame buffer is thirteen words, each carrying one byte in its low
//!   eight bits. Byte 0 holds DLC, the frame flags and the identifier format;
//!   a standard frame puts its 11-bit identifier in bytes 1–2 and its payload
//!   in bytes 3–10, an extended frame puts its 29-bit identifier in bytes 1–4
//!   and the payload in bytes 5–12. Identifiers are stored big-endian and left
//!   aligned.
//! - A time quantum is `2 * (baud_presc + 1)` clock periods, so the register
//!   field only encodes even dividers: `prescaler / 2 - 1`.
//! - Reading the interrupt register clears every flag except the receive one;
//!   the driver therefore reads it once and decodes the bits instead of asking
//!   the PAC for one flag at a time.
//! - The clock source is the 40 MHz crystal. The clock divider register only
//!   divides `CLKOUT`, not the CAN clock, so it plays no part in the bit rate.
//! - A frame only reaches the receiver when the self-reception command is used;
//!   the self-test *mode* alone just stops the controller from acknowledging.
//!   The bit stream still comes back through the RX input, not through the
//!   peripheral, so a node with nothing attached to the bus sees bit errors and
//!   racks up transmit error counts. Without a transceiver, route the TX output
//!   signal and the RX input signal to the same pad: the pad then drives the
//!   level its own input buffer reads, which is a loopback that needs no
//!   hardware. Self-test mode and self-reception are both required on top of
//!   that.
//! - Reset mode is entered automatically on bus-off, and the configuration
//!   registers can only be written in it. Leaving reset mode starts the
//!   controller.

use esp32p4_pac::{HP_SYS_CLKRST, Interrupt, TWAI0, TWAI1, TWAI2, twai0, twai1, twai2};

/// Bytes a frame carries at most.
pub const MAX_DATA_LEN: usize = 8;

/// Bytes the transmit/receive buffer holds.
const FRAME_BUFFER_LEN: usize = 13;

// ============================================================================
// Frames
// ============================================================================

/// A CAN identifier.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Id {
    /// An 11-bit identifier.
    Standard(u16),
    /// A 29-bit identifier.
    Extended(u32),
}

impl Id {
    /// The identifier value, without its length.
    pub const fn value(self) -> u32 {
        match self {
            Self::Standard(id) => id as u32,
            Self::Extended(id) => id,
        }
    }

    /// Whether this is a 29-bit identifier.
    pub const fn is_extended(self) -> bool {
        matches!(self, Self::Extended(_))
    }
}

/// A frame, as transmitted or received.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame {
    /// Standard or extended identifier.
    pub id: Id,
    /// A remote frame asks for data instead of carrying it.
    pub remote: bool,
    /// Also receive this frame while transmitting it.
    ///
    /// This is what makes a single node without a transceiver able to test
    /// itself: the frame is looped back into the receive buffer.
    pub self_reception: bool,
    /// Do not retry when the transmission fails.
    pub single_shot: bool,
    len: u8,
    data: [u8; MAX_DATA_LEN],
}

impl Frame {
    /// A data frame carrying `payload`.
    ///
    /// # Errors
    ///
    /// [`Error::FrameTooLong`] when `payload` is above eight bytes, or the
    /// identifier does not fit its format.
    pub fn data(id: Id, payload: &[u8]) -> Result<Self, Error> {
        if payload.len() > MAX_DATA_LEN {
            return Err(Error::FrameTooLong);
        }
        Self::check_id(id)?;
        let mut data = [0; MAX_DATA_LEN];
        data[..payload.len()].copy_from_slice(payload);
        Ok(Self {
            id,
            remote: false,
            self_reception: false,
            single_shot: false,
            len: payload.len() as u8,
            data,
        })
    }

    /// A remote frame asking for `len` bytes.
    ///
    /// # Errors
    ///
    /// [`Error::FrameTooLong`] when `len` is above eight, or the identifier does
    /// not fit its format.
    pub fn remote(id: Id, len: u8) -> Result<Self, Error> {
        if len > MAX_DATA_LEN as u8 {
            return Err(Error::FrameTooLong);
        }
        Self::check_id(id)?;
        Ok(Self {
            id,
            remote: true,
            self_reception: false,
            single_shot: false,
            len,
            data: [0; MAX_DATA_LEN],
        })
    }

    /// The payload bytes; empty for a remote frame.
    pub fn payload(&self) -> &[u8] {
        if self.remote {
            &self.data[..0]
        } else {
            &self.data[..usize::from(self.len)]
        }
    }

    /// The data length code, i.e. the payload length or the requested length.
    pub const fn len(&self) -> u8 {
        self.len
    }

    /// Whether the frame carries no payload.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn check_id(id: Id) -> Result<(), Error> {
        match id {
            Id::Standard(value) if value > 0x7FF => Err(Error::IdOutOfRange),
            Id::Extended(value) if value > 0x1FFF_FFFF => Err(Error::IdOutOfRange),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// How the controller takes part in bus traffic.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Send, receive and acknowledge.
    #[default]
    Normal,
    /// Receive only: never sends and never acknowledges, so the error counters
    /// stay put. Useful for sniffing a bus without disturbing it.
    ListenOnly,
    /// Loop frames back internally and never acknowledge.
    ///
    /// This is what a node without a transceiver needs: it makes the missing
    /// acknowledge harmless. Pair it with [`Frame::self_reception`] to actually
    /// receive the frame that is sent.
    SelfTest,
}

/// The acceptance filter.
///
/// The hardware compares 32 bits of the frame — starting at the identifier —
/// against `code`, ignoring every bit that `mask` marks as don't-care. A frame is
/// received when only the masked-in bits differ from `code` in no way.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filter {
    /// Receive everything.
    AcceptAll,
    /// Raw code and mask, in the hardware's bit order.
    ///
    /// `code` is compared with the first 32 frame bits after the start-of-frame
    /// bit, most significant bit first; a set bit in `mask` means "must match".
    Raw { code: u32, mask: u32 },
    /// Accept one standard identifier (or a group, with a mask).
    Standard { id: u16, mask: u16 },
    /// Accept one extended identifier (or a group, with a mask).
    Extended { id: u32, mask: u32 },
}

impl Filter {
    /// The code/mask pair the registers get, or `None` for [`Filter::AcceptAll`].
    fn code_and_mask(self) -> (u32, u32) {
        match self {
            Self::AcceptAll => (0, 0xFFFF_FFFF),
            Self::Raw { code, mask } => (code, !mask),
            // The 11 identifier bits are the first 11 bits of the frame after
            // the start-of-frame bit, so they live in bits 31..21 of the code.
            Self::Standard { id, mask } => {
                let id = u32::from(id & 0x7FF) << 21;
                let relevant = u32::from(mask & 0x7FF) << 21;
                (id, !relevant)
            }
            // An extended frame starts with its 29 identifier bits, so they sit
            // in bits 31..3.
            Self::Extended { id, mask } => {
                let id = (id & 0x1FFF_FFFF) << 3;
                let relevant = (mask & 0x1FFF_FFFF) << 3;
                (id, !relevant)
            }
        }
    }
}

/// Bit timing.
///
/// One bit is `1 + tseg1 + tseg2` time quanta long, one quantum is `prescaler`
/// clock periods, and the sample point sits after `1 + tseg1` of them. [`Timing::for_bitrate`] fills all of this in from a bit
/// rate and a sample point, which is what a bus expects: 87.5 % is the CAN
/// recommendation, 75 % is common on long buses.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Timing {
    /// Divider from the 40 MHz crystal to one time quantum, 2–32768 and even.
    pub prescaler: u16,
    /// Time quanta before the sample point, 1–16 (propagation + phase segment 1).
    pub tseg1: u8,
    /// Time quanta after the sample point, 1–8 (phase segment 2).
    pub tseg2: u8,
    /// How far a re-synchronisation may shift the sample point, 1–4.
    pub sjw: u8,
    /// Sample the bus three times per bit instead of once; only useful at low
    /// and medium bit rates.
    pub triple_sampling: bool,
}

impl Timing {
    /// Computes the timing for a bit rate and a sample point in permille.
    ///
    /// The search walks the quantum counts from the finest resolution down and
    /// keeps the first combination that hits the bit rate exactly; the prescaler
    /// is then an even number, as the hardware requires.
    ///
    /// # Errors
    ///
    /// [`Error::BitrateUnsupported`] when the clock cannot produce that bit rate
    /// with 8–25 quanta per bit.
    pub fn for_bitrate(
        clock_hz: u32,
        bitrate: u32,
        sample_point_permille: u16,
    ) -> Result<Self, Error> {
        if bitrate == 0 || !(100..=900).contains(&sample_point_permille) {
            return Err(Error::BitrateUnsupported);
        }
        // One bit is `quanta` time quanta, so the prescaler is
        // `clock / (bitrate * quanta)` and only integer results are usable.
        let target = u64::from(sample_point_permille);
        let mut best: Option<(u64, Self)> = None;
        for quanta in 8..=25u64 {
            let bit_ticks = u64::from(bitrate) * quanta;
            if u64::from(clock_hz) % bit_ticks != 0 {
                continue;
            }
            let prescaler = u64::from(clock_hz) / bit_ticks;
            if !(2..=32_768).contains(&prescaler) || prescaler % 2 != 0 {
                continue;
            }
            // tseg2 decides the sample point; walk the legal range and keep the
            // closest one, since the requested point is rarely reachable.
            for tseg2 in 1..=8u64 {
                let Some(tseg1) = quanta.checked_sub(1 + tseg2) else {
                    continue;
                };
                if !(1..=16).contains(&tseg1) {
                    continue;
                }
                let point = 1000 * (1 + tseg1) / quanta;
                let distance = point.abs_diff(target);
                if best
                    .as_ref()
                    .is_some_and(|(closest, _)| *closest <= distance)
                {
                    continue;
                }
                best = Some((
                    distance,
                    Self {
                        prescaler: prescaler as u16,
                        tseg1: tseg1 as u8,
                        tseg2: tseg2 as u8,
                        sjw: tseg2.min(4) as u8,
                        triple_sampling: false,
                    },
                ));
            }
        }
        best.map(|(_, timing)| timing)
            .ok_or(Error::BitrateUnsupported)
    }

    /// The bit rate this timing produces from `clock_hz`.
    pub fn bitrate(&self, clock_hz: u32) -> u32 {
        let quanta = 1 + u32::from(self.tseg1) + u32::from(self.tseg2);
        clock_hz / (u32::from(self.prescaler) * quanta)
    }

    /// The sample point in permille.
    pub fn sample_point(&self) -> u16 {
        let quanta = 1 + u16::from(self.tseg1) + u16::from(self.tseg2);
        1000 * (1 + u16::from(self.tseg1)) / quanta
    }
}

/// Controller setup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Bit rate and sample point.
    pub timing: Timing,
    /// How the controller takes part in bus traffic.
    pub mode: Mode,
    /// Which frames reach the receive buffer.
    pub filter: Filter,
}

// ============================================================================
// Status and errors
// ============================================================================

/// What the controller is doing right now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Status {
    /// A received frame is waiting in the receive buffer.
    pub frame_available: bool,
    /// The transmit buffer is free.
    pub transmit_idle: bool,
    /// The last transmission finished.
    pub transmission_complete: bool,
    /// The receive FIFO overflowed and frames were lost.
    pub overrun: bool,
    /// The controller saw an error and is error-passive or bus-off.
    pub error: bool,
    /// The controller left the bus after too many errors.
    pub bus_off: bool,
}

/// Error state of the controller.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusState {
    /// Fewer than 96 errors: the controller sends error flags as before.
    Active,
    /// Above 96 errors: the controller only sends passive error flags.
    Passive,
    /// Above 255 transmit errors: the controller left the bus. Reset mode is
    /// entered automatically, so [`Twai0::start`] brings it back.
    Off,
}

/// The error counters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ErrorCounters {
    /// Receive error counter.
    pub receive: u8,
    /// Transmit error counter.
    pub transmit: u8,
}

/// Interrupt sources.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Interrupts(u32);

impl Interrupts {
    /// No source at all.
    pub const NONE: Self = Self(0);
    /// A frame reached the receive buffer.
    pub const RECEIVE: Self = Self(1 << 0);
    /// A transmission finished.
    pub const TRANSMIT: Self = Self(1 << 1);
    /// The error warning limit was crossed.
    pub const ERROR_WARNING: Self = Self(1 << 2);
    /// The receive FIFO overflowed; frames were lost.
    pub const DATA_OVERRUN: Self = Self(1 << 3);
    /// The timestamp counter wrapped.
    pub const TIMESTAMP_OVERFLOW: Self = Self(1 << 4);
    /// The controller became error-passive.
    pub const ERROR_PASSIVE: Self = Self(1 << 5);
    /// The controller lost the arbitration.
    pub const ARBITRATION_LOST: Self = Self(1 << 6);
    /// A bus error was detected.
    pub const BUS_ERROR: Self = Self(1 << 7);
    /// The bus went idle.
    pub const BUS_IDLE: Self = Self(1 << 8);

    /// Both interrupt sources a plain send/receive loop needs.
    pub const DATA: Self = Self(Self::RECEIVE.0 | Self::TRANSMIT.0);

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

/// Errors from the driver.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The payload or the requested length is above eight bytes.
    FrameTooLong,
    /// The identifier does not fit its format.
    IdOutOfRange,
    /// The clock cannot produce the requested bit rate.
    BitrateUnsupported,
    /// No frame is waiting in the receive buffer.
    NoFrame,
    /// The transmit buffer is still busy with the previous frame.
    TransmitBusy,
}

// ============================================================================
// Controllers
// ============================================================================

macro_rules! impl_twai {
    ($Name:ident, $PAC:ident, $mod:ident, $apb:ident, $clk_en:ident, $src_sel:ident, $rst:ident, $irq:ident) => {
        pastey::paste! {
            /// TWAI controller owning the PAC peripheral.
            pub struct $Name {
                twai: $PAC,
            }

            impl $Name {
                /// The interrupt source of this controller.
                pub const SOURCE: Interrupt = Interrupt::$irq;

                /// Gates the register clock, releases the reset and leaves the
                /// controller in reset mode so it can be configured.
                pub fn init(twai: $PAC, clkrst: &HP_SYS_CLKRST) -> Self {
                    clkrst.soc_clk_ctrl2.[<set_ $apb>]();
                    clkrst.peri_clk_ctrl115.[<set_ $clk_en>]();
                    // The clock source is the 40 MHz crystal; the divider
                    // register only feeds CLKOUT, not the CAN clock.
                    clkrst.peri_clk_ctrl115.[<clr_ $src_sel>]();
                    clkrst.hp_rst_en1.[<set_ $rst>]();
                    clkrst.hp_rst_en1.[<clr_ $rst>]();
                    let this = Self { twai };
                    this.regs().mode.set_reset_mode();
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
                        twai: unsafe { $PAC::steal() },
                    })
                }

                /// The underlying register block.
                #[inline]
                pub fn regs(&self) -> &$mod::RegisterBlock {
                    &self.twai
                }

                /// Returns the PAC peripheral.
                #[inline]
                pub fn release(self) -> $PAC {
                    self.twai
                }

                /// The crystal the bit timing is derived from.
                pub const CLOCK_HZ: u32 = crate::clock::XTAL_HZ;

                /// Applies `config` and leaves the controller in reset mode.
                ///
                /// Everything that reset mode guards — timing, filter, mode — is
                /// written here; [`start`](Self::start) then connects the
                /// controller to the bus.
                pub fn configure(&self, config: &Config) {
                    let regs = self.regs();
                    regs.mode.set_reset_mode();

                    let timing = &config.timing;
                    let timing0 = &regs.bus_timing0;
                    // One time quantum is `2 * (baud_presc + 1)` clock periods.
                    timing0.write_baud_presc(u32::from(timing.prescaler / 2 - 1));
                    timing0.write_sync_jump_width(u32::from(timing.sjw - 1));
                    let timing1 = &regs.bus_timing1;
                    timing1.write_time_segment1(u32::from(timing.tseg1 - 1));
                    timing1.write_time_segment2(u32::from(timing.tseg2 - 1));
                    if timing.triple_sampling {
                        timing1.set_time_sampling();
                    } else {
                        timing1.clr_time_sampling();
                    }

                    // The acceptance filter lives at the address of the frame
                    // buffer, which is only mapped while reset mode is set.
                    let (code, mask) = config.filter.code_and_mask();
                    regs.data0.write_data_0(code >> 24);
                    regs.data1.write_data_1((code >> 16) & 0xFF);
                    regs.data2.write_data_2((code >> 8) & 0xFF);
                    regs.data3.write_data_3(code & 0xFF);
                    regs.data4.write_data_4(mask >> 24);
                    regs.data5.write_data_5((mask >> 16) & 0xFF);
                    regs.data6.write_data_6((mask >> 8) & 0xFF);
                    regs.data7.write_data_7(mask & 0xFF);
                    // The hardware exposes one filter, so it is always single.
                    regs.mode.set_acceptance_filter_mode();

                    match config.mode {
                        Mode::Normal => {
                            regs.mode.clr_listen_only_mode();
                            regs.mode.clr_self_test_mode();
                        }
                        Mode::ListenOnly => {
                            regs.mode.set_listen_only_mode();
                            regs.mode.clr_self_test_mode();
                        }
                        Mode::SelfTest => {
                            regs.mode.clr_listen_only_mode();
                            regs.mode.set_self_test_mode();
                        }
                    }
                }

                /// Leaves reset mode: the controller starts taking part in the
                /// bus.
                pub fn start(&self) {
                    self.regs().mode.clr_reset_mode();
                    // The controller needs a few clock cycles to leave reset
                    // mode; reading the bit back is the documented way to wait.
                    crate::spin::spin_wait("twai leaves reset mode", || {
                        !self.regs().mode.reset_mode()
                    });
                }

                /// Enters reset mode: the controller leaves the bus and the
                /// configuration registers become writable again.
                pub fn stop(&self) {
                    self.regs().mode.set_reset_mode();
                }

                /// Whether the controller is held in reset mode.
                pub fn is_stopped(&self) -> bool {
                    self.regs().mode.reset_mode()
                }

                /// Queues `frame` for transmission.
                ///
                /// Returns as soon as the frame is in the buffer; poll
                /// [`status`](Self::status) or wait for the transmit interrupt to
                /// know when it left. [`Frame::self_reception`] loops the frame
                /// back into the receive buffer, [`Frame::single_shot`] gives up
                /// after one attempt instead of retrying.
                ///
                /// # Errors
                ///
                /// [`Error::TransmitBusy`] when the previous frame is still in
                /// the buffer.
                pub fn transmit(&self, frame: &Frame) -> Result<(), Error> {
                    let regs = self.regs();
                    if !regs.status.transmit_buffer() {
                        return Err(Error::TransmitBusy);
                    }

                    let (id_bytes, id_len) = format_id(frame.id);
                    let mut bytes = [0u8; FRAME_BUFFER_LEN];
                    bytes[0] = (frame.len & 0x0F)
                        | ((frame.self_reception as u8) << 4)
                        | ((frame.single_shot as u8) << 5)
                        | ((frame.remote as u8) << 6)
                        | ((frame.id.is_extended() as u8) << 7);
                    bytes[1..][..id_len].copy_from_slice(&id_bytes[..id_len]);
                    let data_at = 1 + id_len;
                    bytes[data_at..][..MAX_DATA_LEN].copy_from_slice(&frame.data);
                    self.write_frame_buffer(&bytes);

                    // The command register is written once: a single-shot frame
                    // needs TX_REQUEST and ABORT_TX in the same write.
                    let mut command = if frame.self_reception { 1 << 4 } else { 1 << 0 };
                    if frame.single_shot {
                        command |= 1 << 1;
                    }
                    regs.cmd.write_bits(command);
                    Ok(())
                }

                /// Waits until the last frame left the buffer.
                ///
                /// Also returns on a failed attempt: the completion flag is set
                /// whether the frame went through or the controller gave up.
                pub fn wait_transmitted(&self) {
                    crate::spin::spin_wait("twai transmission", || {
                        self.regs().status.transmission_complete()
                    });
                }

                /// Aborts the pending transmission.
                pub fn abort_transmission(&self) {
                    self.regs().cmd.trigger_abort_tx();
                }

                /// Reads the next received frame out of the buffer and releases
                /// it.
                ///
                /// # Errors
                ///
                /// [`Error::NoFrame`] when no frame is waiting.
                pub fn receive(&self) -> Result<Frame, Error> {
                    let regs = self.regs();
                    if !regs.status.receive_buffer() {
                        return Err(Error::NoFrame);
                    }
                    let bytes = self.read_frame_buffer();
                    // Releasing the buffer hands the window to the next frame in
                    // the FIFO, so the bytes are read first.
                    regs.cmd.trigger_release_buffer();
                    Ok(parse_frame(&bytes))
                }

                /// How many frames wait in the receive FIFO.
                pub fn frames_pending(&self) -> u8 {
                    self.regs().rx_message_counter.rx_message_counter() as u8
                }

                /// Copies the thirteen frame bytes into the buffer.
                fn write_frame_buffer(&self, bytes: &[u8; FRAME_BUFFER_LEN]) {
                    let regs = self.regs();
                    regs.data0.write_data_0(u32::from(bytes[0]));
                    regs.data1.write_data_1(u32::from(bytes[1]));
                    regs.data2.write_data_2(u32::from(bytes[2]));
                    regs.data3.write_data_3(u32::from(bytes[3]));
                    regs.data4.write_data_4(u32::from(bytes[4]));
                    regs.data5.write_data_5(u32::from(bytes[5]));
                    regs.data6.write_data_6(u32::from(bytes[6]));
                    regs.data7.write_data_7(u32::from(bytes[7]));
                    regs.data8.write_data_8(u32::from(bytes[8]));
                    regs.data9.write_data_9(u32::from(bytes[9]));
                    regs.data10.write_data_10(u32::from(bytes[10]));
                    regs.data11.write_data_11(u32::from(bytes[11]));
                    regs.data12.write_data_12(u32::from(bytes[12]));
                }

                /// Reads the thirteen frame bytes out of the buffer.
                fn read_frame_buffer(&self) -> [u8; FRAME_BUFFER_LEN] {
                    let regs = self.regs();
                    [
                        regs.data0.data_0() as u8,
                        regs.data1.data_1() as u8,
                        regs.data2.data_2() as u8,
                        regs.data3.data_3() as u8,
                        regs.data4.data_4() as u8,
                        regs.data5.data_5() as u8,
                        regs.data6.data_6() as u8,
                        regs.data7.data_7() as u8,
                        regs.data8.data_8() as u8,
                        regs.data9.data_9() as u8,
                        regs.data10.data_10() as u8,
                        regs.data11.data_11() as u8,
                        regs.data12.data_12() as u8,
                    ]
                }

                /// Drops the overrun flag after the FIFO overflowed.
                pub fn clear_overrun(&self) {
                    self.regs().cmd.trigger_clear_data_overrun();
                }

                /// What the controller is doing right now.
                pub fn status(&self) -> Status {
                    let status = &self.regs().status;
                    Status {
                        frame_available: status.receive_buffer(),
                        transmit_idle: status.transmit_buffer(),
                        transmission_complete: status.transmission_complete(),
                        overrun: status.overrun(),
                        error: status.err(),
                        bus_off: status.node_bus_off(),
                    }
                }

                /// The error state the counters put the controller in.
                pub fn bus_state(&self) -> BusState {
                    if self.regs().status.node_bus_off() {
                        BusState::Off
                    } else if self.regs().status.err() {
                        BusState::Passive
                    } else {
                        BusState::Active
                    }
                }

                /// Both error counters.
                pub fn error_counters(&self) -> ErrorCounters {
                    let regs = self.regs();
                    ErrorCounters {
                        receive: regs.rx_err_cnt.rx_err_cnt() as u8,
                        transmit: regs.tx_err_cnt.tx_err_cnt() as u8,
                    }
                }

                /// Enables exactly the given interrupt sources.
                pub fn enable_interrupts(&self, sources: Interrupts) {
                    // The enable register has one bit per source, at the same
                    // position as the interrupt register's flag.
                    self.regs().interrupt_enable.write_bits(sources.raw());
                }

                /// Disables every interrupt source.
                pub fn disable_interrupts(&self) {
                    self.regs().interrupt_enable.write_bits(0);
                }

                /// The sources that fired since the last call.
                ///
                /// Reading the register clears every flag except the receive
                /// one, so this is both the poll and the acknowledge.
                pub fn interrupts(&self) -> Interrupts {
                    Interrupts(self.regs().interrupt.read_bits() & 0x1FF)
                }
            }
        }
    };
}

/// Splits an identifier into the buffer's big-endian, left-aligned bytes.
fn format_id(id: Id) -> ([u8; 4], usize) {
    match id {
        // 11 bits, left aligned in two bytes.
        Id::Standard(value) => {
            let shifted = u32::from(value & 0x7FF) << 5;
            ([(shifted >> 8) as u8, shifted as u8, 0, 0], 2)
        }
        // 29 bits, left aligned in four bytes.
        Id::Extended(value) => {
            let shifted = (value & 0x1FFF_FFFF) << 3;
            (
                [
                    (shifted >> 24) as u8,
                    (shifted >> 16) as u8,
                    (shifted >> 8) as u8,
                    shifted as u8,
                ],
                4,
            )
        }
    }
}

/// Parses the thirteen buffer bytes back into a frame.
fn parse_frame(bytes: &[u8; FRAME_BUFFER_LEN]) -> Frame {
    let extended = bytes[0] & 0x80 != 0;
    let (id, data_at) = if extended {
        let shifted = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        (Id::Extended(shifted >> 3), 5)
    } else {
        let shifted = u32::from(u16::from_be_bytes([bytes[1], bytes[2]]));
        (Id::Standard((shifted >> 5) as u16), 3)
    };
    let remote = bytes[0] & 0x40 != 0;
    let len = (bytes[0] & 0x0F).min(MAX_DATA_LEN as u8);
    let mut data = [0u8; MAX_DATA_LEN];
    // The buffer keeps whatever the previous owner left in the unused bytes.
    if !remote {
        data[..usize::from(len)].copy_from_slice(&bytes[data_at..][..usize::from(len)]);
    }
    Frame {
        id,
        remote,
        self_reception: false,
        single_shot: false,
        len,
        data,
    }
}

impl_twai!(
    Twai0,
    TWAI0,
    twai0,
    reg_twai0_apb_clk_en,
    reg_twai0_clk_en,
    reg_twai0_clk_src_sel,
    reg_rst_en_can0,
    TWAI0
);
impl_twai!(
    Twai1,
    TWAI1,
    twai1,
    reg_twai1_apb_clk_en,
    reg_twai1_clk_en,
    reg_twai1_clk_src_sel,
    reg_rst_en_can1,
    TWAI1
);
impl_twai!(
    Twai2,
    TWAI2,
    twai2,
    reg_twai2_apb_clk_en,
    reg_twai2_clk_en,
    reg_twai2_clk_src_sel,
    reg_rst_en_can2,
    TWAI2
);
