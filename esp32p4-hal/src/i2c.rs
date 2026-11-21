//! I2C (Inter-Integrated Circuit) master and slave.
//!
//! I2C0 and I2C1 are the two high-performance controllers. Each one is either a
//! [`I2c0`] master or an [`I2c0Slave`] slave; the two types own the same
//! peripheral, so a controller cannot be both at once. Every wait uses a
//! wall-clock deadline rather than an iteration count, so a slave that stretches
//! the clock cannot cause a spurious timeout.
//!
//! SCL and SDA are GPIO-matrix signals, so any GPIO can carry them (unlike the
//! dedicated pads of the USB and EMAC PHYs, see [`crate::gpio::pins`]).
//!
//! # Features
//!
//! - Any SCL rate the clock source and the timing constraints allow, 100 kHz
//!   (standard) and 400 kHz (fast) included
//! - 7-bit addresses; write, read and combined write-read transactions
//! - Hardware clock stretching, bounded by [`I2cConfig::timeout_us`]
//! - NACK, arbitration-lost and timeout reporting, bus-busy detection
//! - Bus recovery by pulsing SCL, and a controller state-machine reset
//! - Per-source interrupt enable, status and clear
//!
//! As a slave:
//!
//! - 7-bit addresses, with the general call address `0x00` if asked for
//! - Receive and transmit through 32-byte FIFOs, blocking or by hand
//! - Clock stretching, so a master waits instead of over- or underrunning a
//!   FIFO, with the cause readable and the release under software control. The
//!   master blocks while the slave holds SCL, so a stretching slave must be
//!   serviced from an interrupt or another thread — a single-threaded polling
//!   loop deadlocks. [`SlaveConfig::stretch`] turns it off for the polled case.
//! - START and STOP detection, addressed state and the master's direction
//!
//! Both SDA and SCL are open-drain, so the pads need external pull-up
//! resistors. [`I2c0::set_pins`] configures the pads and the GPIO matrix; the
//! driver does not enable the (much weaker) internal pull-ups.
//!
//! # Transfer Limits
//!
//! One transaction may use at most eight commands, the number of command
//! registers, and at most one FIFO of data (32 bytes) including the address
//! bytes. The common cases fit:
//!
//! | Transaction | Commands | Data |
//! |-------------|----------|------|
//! | write       | 3-4      | 31 bytes |
//! | read        | 3-4      | 32 bytes |
//! | write-read  | 7-8      | 30 written, 32 read |
//!
//! Longer transfers belong in the caller's hands: split them into several
//! transactions, or wait for the DMA path that the roadmap lists.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::Pins;
//! use esp32p4_hal::i2c::{I2c0, I2cConfig};
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//!
//! let i2c = I2c0::init(dp.I2C0, &I2cConfig::default(), &dp.HP_SYS_CLKRST);
//! i2c.set_pins(pins.pin::<8>(), pins.pin::<9>());
//!
//! // Read register 0x0f of a sensor at 0x48.
//! let mut value = [0u8; 1];
//! i2c.write_read(0x48, &[0x0f], &mut value).unwrap();
//!
//! // Enumerate the bus.
//! for address in 0x08..=0x77 {
//!     if i2c.probe(address).is_ok() {
//!         // found a device
//!     }
//! }
//! ```
//!
//! As a slave the same controller answers to an address instead of driving the
//! bus:
//!
//! ```ignore
//! use esp32p4_hal::i2c::{I2c1Slave, SlaveConfig};
//!
//! let slave = I2c1Slave::init(
//!     dp.I2C1,
//!     &SlaveConfig::default().address(0x55),
//!     &dp.HP_SYS_CLKRST,
//! );
//! slave.set_pins(pins.pin::<20>(), pins.pin::<21>());
//!
//! // Collect what the master writes, up to its stop condition.
//! let mut request = [0u8; 4];
//! let len = slave.read(&mut request).unwrap();
//!
//! // Answer the next read transfer; the bytes come out of the FIFO as the
//! // master clocks them.
//! slave.write(&[0xde, 0xad, 0xbe, 0xef]).unwrap();
//! ```

use esp32p4_pac::{HP_SYS_CLKRST, I2C0, I2C1};
use pastey::paste;

use crate::gpio::{InputSignal, OutputSignal, PinOps};

/// Entries in the TX and RX FIFOs.
const FIFO_DEPTH: usize = 32;
/// Command registers (`COMD0`..`COMD7`).
const COMMAND_SLOTS: usize = 8;

/// Command opcodes (`COMDn.op_code`).
mod op {
    /// Repeated start.
    pub const RESTART: u32 = 6;
    /// Write `byte_num` bytes from the TX FIFO.
    pub const WRITE: u32 = 1;
    /// Read `byte_num` bytes into the RX FIFO.
    pub const READ: u32 = 3;
    /// Stop condition.
    pub const STOP: u32 = 2;
    /// End of the command list. The state machine stops here.
    pub const END: u32 = 4;
}

/// Clock feeding the I2C core clock.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cClock {
    /// `XTAL_CLK`, 40 MHz. The reset default.
    #[default]
    Xtal,
    /// `RC_FAST_CLK`, nominally 17.5 MHz.
    RcFast,
}

impl I2cClock {
    /// Nominal frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Xtal => crate::clock::XTAL_HZ,
            Self::RcFast => crate::clock::RC_FAST_HZ,
        }
    }
}

/// I2C configuration.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct I2cConfig {
    /// SCL frequency in Hz.
    pub frequency: u32,
    /// Clock source for the bus timing generator.
    pub clock: I2cClock,
    /// How long one transaction may take, in microseconds.
    ///
    /// The transaction is polled with this deadline, so it also bounds how
    /// long a slave may stretch the clock.
    pub timeout_us: u64,
    /// Enable the SCL/SDA input glitch filter.
    pub glitch_filter: bool,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            frequency: 100_000,
            clock: I2cClock::Xtal,
            timeout_us: 100_000,
            glitch_filter: true,
        }
    }
}

impl I2cConfig {
    /// Sets the SCL frequency.
    pub const fn frequency(mut self, frequency: u32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Selects the clock source.
    pub const fn clock(mut self, clock: I2cClock) -> Self {
        self.clock = clock;
        self
    }

    /// Sets the per-transaction deadline.
    pub const fn timeout_us(mut self, timeout_us: u64) -> Self {
        self.timeout_us = timeout_us;
        self
    }

    /// Enables or disables the input glitch filter.
    pub const fn glitch_filter(mut self, enabled: bool) -> Self {
        self.glitch_filter = enabled;
        self
    }
}

/// Configuration of a slave.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SlaveConfig {
    /// The 7-bit address the slave answers to.
    pub address: u8,
    /// Clock source; sets the resolution of the SDA timing.
    pub clock: I2cClock,
    /// How long the blocking calls wait, in microseconds.
    pub timeout_us: u64,
    /// Hold SCL low while the FIFOs need attention.
    ///
    /// On, a master waits instead of overrunning a full receive FIFO or
    /// underrunning an empty transmit FIFO. The cost is that the master blocks
    /// inside its own transfer, so the slave has to be serviced *while* the
    /// transfer runs — from [`SOURCE`](I2c0Slave::SOURCE)'s interrupt, or from
    /// another thread. A single-threaded polling loop deadlocks instead.
    ///
    /// Off, the transfer runs at line rate and the caller has to keep up: one
    /// transfer has to fit the 32-byte FIFOs, and the FIFOs wrap rather than
    /// refuse data, so anything beyond that overwrites what was not read.
    pub stretch: bool,
    /// Answer the general call address `0x00` as well.
    pub general_call: bool,
    /// Enable the SCL/SDA input glitch filter.
    pub glitch_filter: bool,
}

impl Default for SlaveConfig {
    fn default() -> Self {
        Self {
            address: 0x55,
            clock: I2cClock::Xtal,
            timeout_us: 100_000,
            stretch: true,
            general_call: false,
            glitch_filter: true,
        }
    }
}

impl SlaveConfig {
    /// Sets the 7-bit address.
    pub const fn address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    /// Selects the clock source.
    pub const fn clock(mut self, clock: I2cClock) -> Self {
        self.clock = clock;
        self
    }

    /// Sets how long the blocking calls wait.
    pub const fn timeout_us(mut self, timeout_us: u64) -> Self {
        self.timeout_us = timeout_us;
        self
    }

    /// Enables or disables clock stretching.
    pub const fn stretch(mut self, enabled: bool) -> Self {
        self.stretch = enabled;
        self
    }

    /// Enables or disables answering the general call address.
    pub const fn general_call(mut self, enabled: bool) -> Self {
        self.general_call = enabled;
        self
    }

    /// Enables or disables the input glitch filter.
    pub const fn glitch_filter(mut self, enabled: bool) -> Self {
        self.glitch_filter = enabled;
        self
    }
}

/// Why a slave holds SCL low.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StretchCause {
    /// The master addressed this slave for a read and waits for data.
    AddressMatch,
    /// The transmit FIFO ran empty while the master was reading.
    TransmitEmpty,
    /// The receive FIFO is full.
    ReceiveFull,
    /// The slave holds SCL to decide the acknowledge bit
    /// (`SLAVE_BYTE_ACK_CTL_EN`).
    SendingAck,
}

/// Errors reported by a transfer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The addressed device did not acknowledge.
    NoAcknowledge,
    /// Another master won the bus while this one was transmitting.
    ArbitrationLost,
    /// The transaction did not finish within [`I2cConfig::timeout_us`].
    Timeout,
    /// The bus was already busy when the transaction started.
    BusBusy,
    /// The transaction needs more than the eight command registers.
    TooManyCommands,
    /// The transaction moves more data than the FIFO holds.
    TooMuchData,
    /// The address is outside the 7-bit range.
    InvalidAddress,
}

/// One step of a transaction.
#[derive(Debug)]
pub enum Operation<'a> {
    /// Send these bytes.
    Write(&'a [u8]),
    /// Receive into this buffer.
    Read(&'a mut [u8]),
}

/// Interrupt sources.
///
/// Bit positions are those of `I2C_INT_RAW`/`INT_ENA`/`INT_CLR`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Interrupt {
    /// The RX FIFO reached its watermark.
    RxFifoWatermark = 0,
    /// The TX FIFO dropped to its watermark.
    TxFifoWatermark = 1,
    /// A command list finished without a stop condition.
    EndDetect = 3,
    /// Another master took the bus.
    ArbitrationLost = 5,
    /// The whole command list finished.
    TransferComplete = 7,
    /// The bus timed out (`TO.TIME_OUT_VALUE`).
    TimeOut = 8,
    /// A device did not acknowledge.
    Nack = 10,
    /// The SCL state machine timed out.
    SclTimeOut = 13,
    /// A byte was sent or received.
    ByteTransferDone = 4,
    /// A START condition was seen, master or slave.
    StartDetect = 15,
    /// A slave holds SCL low: address match, FIFO full or FIFO empty.
    SlaveStretch = 16,
    /// The general call address `0x00` was received.
    GeneralCall = 17,
    /// A slave saw an address that is not its own.
    SlaveAddressUnmatch = 18,
    /// The receive FIFO overflowed.
    RxFifoOverflow = 2,
    /// The transmit FIFO overflowed.
    TxFifoOverflow = 11,
    /// The receive FIFO was read while empty.
    RxFifoUnderflow = 12,
}

impl Interrupt {
    /// The mask of this source in the interrupt registers.
    #[inline]
    pub const fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

/// The events that abort or finish a transaction.
const EVENT_MASK: u32 = Interrupt::TransferComplete.mask()
    | Interrupt::Nack.mask()
    | Interrupt::TimeOut.mask()
    | Interrupt::SclTimeOut.mask()
    | Interrupt::ArbitrationLost.mask()
    | Interrupt::EndDetect.mask();

/// Every interrupt source, used to clear the latched flags in one write.
const ALL_INTERRUPTS: u32 = 0x7_ffff;

/// Encodes one command word.
const fn command(byte_num: u32, ack_en: bool, ack_val: bool, op_code: u32) -> u32 {
    (byte_num & 0xff) | ((ack_en as u32) << 8) | ((ack_val as u32) << 10) | ((op_code & 0x7) << 11)
}

/// The command list of one transaction, built into the eight command
/// registers.
struct CommandList {
    words: [u32; COMMAND_SLOTS],
    len: usize,
}

impl CommandList {
    const fn new() -> Self {
        Self {
            words: [0; COMMAND_SLOTS],
            len: 0,
        }
    }

    /// Appends one command word.
    fn push(&mut self, word: u32) -> Result<(), Error> {
        if self.len == COMMAND_SLOTS {
            return Err(Error::TooManyCommands);
        }
        self.words[self.len] = word;
        self.len += 1;
        Ok(())
    }

    fn as_slice(&self) -> &[u32] {
        &self.words[..self.len]
    }
}

/// Bus timing derived from the source clock and the requested SCL rate.
#[derive(Clone, Copy, Debug)]
struct Timing {
    /// Pre-divider applied to the source clock.
    clkm_div: u32,
    /// `scl_low_period`.
    scl_low: u32,
    /// `scl_high_period`.
    scl_high: u32,
    /// `scl_wait_high_period`, the start of the high phase.
    scl_wait_high: u32,
    /// `sda_hold.time`.
    sda_hold: u32,
    /// `sda_sample.time`.
    sda_sample: u32,
    /// `scl_rstart_setup.time` and `scl_stop_setup.time`.
    setup: u32,
    /// `scl_start_hold.time` and `scl_stop_hold.time`.
    hold: u32,
    /// `to.time_out_value`, in units of `2^n` core clock cycles.
    tout: u32,
}

impl Timing {
    /// Computes the timing for `scl_hz` from `source_hz`.
    ///
    /// Mirrors ESP-IDF's `i2c_ll_master_cal_bus_clk`. The high phase is split
    /// into "wait high" and "high" because the peripheral counts them
    /// separately; below 80 kHz a wider wait keeps the resulting frequency
    /// accurate.
    fn new(source_hz: u32, scl_hz: u32) -> Option<Self> {
        if scl_hz == 0 || scl_hz > source_hz / 8 {
            return None;
        }

        let clkm_div = source_hz / (scl_hz * 1024) + 1;
        let sclk = source_hz / clkm_div;
        let half = sclk / scl_hz / 2;
        if half < 8 {
            return None;
        }

        let scl_wait_high = if scl_hz >= 80_000 {
            half / 2 - 2
        } else {
            half / 4
        };

        Some(Self {
            clkm_div,
            scl_low: half,
            scl_high: half - scl_wait_high,
            scl_wait_high,
            sda_hold: half / 4,
            sda_sample: half / 2,
            setup: half,
            hold: half,
            tout: u32::BITS - (5 * half).leading_zeros() + 2,
        })
    }
}

/// Generates one I2C type from the shared register layout.
///
/// `div_ctrl` is the `PERI_CLK_CTRL` register holding the core divider: I2C0
/// keeps it in `PERI_CLK_CTRL10` together with the gate and source select,
/// I2C1 in `PERI_CLK_CTRL11`.
macro_rules! impl_i2c {
    ($Name:ident, $PAC:ident, $mod:ident, $NUM:literal, $scl_out:ident, $scl_in:ident, $sda_out:ident, $sda_in:ident, $div_ctrl:ident) => {
        paste! {
            /// I2C master driver owning the PAC peripheral.
            pub struct $Name {
                i2c: $PAC,
                timeout_us: u64,
            }

            impl $Name {
                /// Controller number, 0 or 1.
                pub const NUM: u8 = $NUM;

                /// Obtains a handle without ownership checks.
                ///
                /// The deadline is the [`I2cConfig::default`] one; use
                /// [`set_timeout_us`](Self::set_timeout_us) on the owning
                /// instance if the bus needs longer.
                ///
                /// # Safety
                ///
                /// The caller must ensure the controller was initialised and
                /// that the returned handle is not used to reconfigure it
                /// concurrently.
                #[inline]
                pub unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self {
                        i2c: unsafe { $PAC::steal() },
                        timeout_us: I2cConfig::default().timeout_us,
                    })
                }

                /// The underlying register block.
                #[inline]
                pub fn regs(&self) -> &esp32p4_pac::$mod::RegisterBlock {
                    &self.i2c
                }

                /// Returns the PAC peripheral.
                #[inline]
                pub fn release(self) -> $PAC {
                    self.i2c
                }

                /// Overrides the per-transaction deadline.
                ///
                /// A handle obtained with [`steal`](Self::steal) keeps the
                /// default, so set the deadline on the owning instance before
                /// publishing it.
                #[inline]
                pub fn set_timeout_us(&mut self, timeout_us: u64) {
                    self.timeout_us = timeout_us;
                }

                // ============================================================
                // Initialisation
                // ============================================================

                /// Initialises the controller as a master and applies `config`.
                ///
                /// # Panics
                ///
                /// Panics if `config.frequency` cannot be produced from
                /// `config.clock`.
                pub fn init(i2c: $PAC, config: &I2cConfig, clkrst: &HP_SYS_CLKRST) -> Self {
                    let timing = Timing::new(config.clock.hz(), config.frequency).unwrap_or_else(
                        || panic!("unsupported I2C frequency: {}", config.frequency),
                    );

                    // Bus clock, reset pulse, controller clock and source.
                    clkrst.soc_clk_ctrl2.[<set_reg_i2c $NUM _apb_clk_en>]();
                    clkrst.hp_rst_en1.[<set_reg_rst_en_i2c $NUM>]();
                    clkrst.hp_rst_en1.[<clr_reg_rst_en_i2c $NUM>]();
                    clkrst.peri_clk_ctrl10.[<set_reg_i2c $NUM _clk_en>]();
                    match config.clock {
                        I2cClock::Xtal => {
                            clkrst.peri_clk_ctrl10.[<clr_reg_i2c $NUM _clk_src_sel>]();
                        }
                        I2cClock::RcFast => {
                            clkrst.peri_clk_ctrl10.[<set_reg_i2c $NUM _clk_src_sel>]();
                        }
                    }
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_num>](timing.clkm_div - 1);
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_numerator>](0);
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_denominator>](0);

                    let i2c = Self {
                        i2c,
                        timeout_us: config.timeout_us,
                    };

                    let r = i2c.regs();
                    r.scl_low_period.write_scl_low_period(timing.scl_low - 1);
                    r.scl_high_period.write_scl_high_period(timing.scl_high);
                    r.scl_high_period.write_scl_wait_high_period(timing.scl_wait_high);
                    r.sda_hold.write_time(timing.sda_hold - 1);
                    r.sda_sample.write_time(timing.sda_sample - 1);
                    r.scl_rstart_setup.write_time(timing.setup - 1);
                    r.scl_stop_setup.write_time(timing.setup - 1);
                    r.scl_start_hold.write_time(timing.hold - 1);
                    r.scl_stop_hold.write_time(timing.hold - 1);
                    r.to.write_time_out_value(timing.tout);
                    r.to.set_time_out_en();

                    // Master, open-drain outputs: the peripheral pulls the
                    // pads low and the pull-ups bring them back high.
                    r.ctr.set_ms_mode();
                    r.ctr.clr_sda_force_out();
                    r.ctr.clr_scl_force_out();
                    // Multi-master arbitration is not supported: a second
                    // master on the bus is reported, not negotiated.
                    r.ctr.clr_arbitration_en();
                    r.ctr.clr_rx_full_ack_level();
                    r.ctr.clr_tx_lsb_first();
                    r.ctr.clr_rx_lsb_first();

                    r.fifo_conf.set_tx_fifo_rst();
                    r.fifo_conf.set_rx_fifo_rst();
                    r.fifo_conf.clr_tx_fifo_rst();
                    r.fifo_conf.clr_rx_fifo_rst();

                    if config.glitch_filter {
                        r.filter_cfg.set_scl_filter_en();
                        r.filter_cfg.set_sda_filter_en();
                    } else {
                        r.filter_cfg.clr_scl_filter_en();
                        r.filter_cfg.clr_sda_filter_en();
                    }

                    r.ctr.trigger_conf_upgate();
                    i2c
                }

                /// Routes `scl` and `sda` to this controller.
                ///
                /// Both pads become open-drain, with the output enable coming
                /// from the peripheral, which is what lets the bus be shared.
                pub fn set_pins<SCL: PinOps, SDA: PinOps>(&self, mut scl: SCL, mut sda: SDA) {
                    scl.route_output_signal_open_drain(OutputSignal::$scl_out);
                    scl.route_input_signal(InputSignal::$scl_in);
                    scl.set_open_drain_enabled(true);
                    sda.route_output_signal_open_drain(OutputSignal::$sda_out);
                    sda.route_input_signal(InputSignal::$sda_in);
                    sda.set_open_drain_enabled(true);
                }

                // ============================================================
                // Transfers
                // ============================================================

                /// Checks whether a device answers at `address`.
                ///
                /// This is an address-only write, so it reports
                /// [`Error::NoAcknowledge`] when nothing is there.
                pub fn probe(&self, address: u8) -> Result<(), Error> {
                    self.transaction(address, &mut [])
                }

                /// Writes `data` to `address`.
                pub fn write(&self, address: u8, data: &[u8]) -> Result<(), Error> {
                    self.transaction(address, &mut [Operation::Write(data)])
                }

                /// Reads `data.len()` bytes from `address`.
                pub fn read(&self, address: u8, data: &mut [u8]) -> Result<(), Error> {
                    self.transaction(address, &mut [Operation::Read(data)])
                }

                /// Writes `write`, then reads into `read` without releasing the
                /// bus in between — the usual register-read sequence.
                pub fn write_read(
                    &self,
                    address: u8,
                    write: &[u8],
                    read: &mut [u8],
                ) -> Result<(), Error> {
                    self.transaction(
                        address,
                        &mut [Operation::Write(write), Operation::Read(read)],
                    )
                }

                /// Runs a sequence of operations against `address`.
                ///
                /// Each operation after the first starts with a repeated start,
                /// so the bus stays owned for the whole sequence. See the
                /// [module documentation](self) for the command and FIFO
                /// limits.
                pub fn transaction(&self, address: u8, ops: &mut [Operation<'_>]) -> Result<(), Error> {
                    if address > 0x7f {
                        return Err(Error::InvalidAddress);
                    }

                    let r = self.regs();
                    if r.sr.bus_busy() {
                        return Err(Error::BusBusy);
                    }

                    // Drop anything a failed earlier transaction left behind.
                    r.fifo_conf.set_tx_fifo_rst();
                    r.fifo_conf.set_rx_fifo_rst();
                    r.fifo_conf.clr_tx_fifo_rst();
                    r.fifo_conf.clr_rx_fifo_rst();

                    let mut list = CommandList::new();
                    let mut tx_used = 0usize;
                    let mut rx_expected = 0usize;

                    list.push(command(0, false, false, op::RESTART))?;

                    if ops.is_empty() {
                        // Address-only write: the probe.
                        list.push(command(1, true, false, op::WRITE))?;
                        r.data.write_bits(u32::from(address << 1));
                        tx_used += 1;
                    }

                    for (index, operation) in ops.iter().enumerate() {
                        match operation {
                            Operation::Write(data) => {
                                if tx_used + 1 + data.len() > FIFO_DEPTH {
                                    return Err(Error::TooMuchData);
                                }
                                if index > 0 {
                                    list.push(command(0, false, false, op::RESTART))?;
                                }
                                // The address byte shares the command with the
                                // payload: one WRITE covers both.
                                list.push(command(
                                    (1 + data.len()) as u32,
                                    true,
                                    false,
                                    op::WRITE,
                                ))?;
                                r.data.write_bits(u32::from(address << 1));
                                for &byte in data.iter() {
                                    r.data.write_bits(u32::from(byte));
                                }
                                tx_used += 1 + data.len();
                            }
                            Operation::Read(data) => {
                                if data.is_empty() {
                                    continue;
                                }
                                if tx_used + 1 > FIFO_DEPTH
                                    || rx_expected + data.len() > FIFO_DEPTH
                                {
                                    return Err(Error::TooMuchData);
                                }
                                if index > 0 {
                                    list.push(command(0, false, false, op::RESTART))?;
                                }
                                list.push(command(1, true, false, op::WRITE))?;
                                r.data.write_bits(u32::from((address << 1) | 1));
                                tx_used += 1;

                                // All bytes but the last carry an ACK; the last
                                // one a NACK so the slave releases SDA.
                                if data.len() > 1 {
                                    list.push(command(
                                        (data.len() - 1) as u32,
                                        false,
                                        false,
                                        op::READ,
                                    ))?;
                                }
                                list.push(command(1, false, true, op::READ))?;
                                rx_expected += data.len();
                            }
                        }
                    }

                    list.push(command(0, false, false, op::STOP))?;
                    // Without an END the state machine keeps executing the
                    // stale command slots, which stalls the bus.
                    list.push(command(0, false, false, op::END))?;

                    self.run_commands(list.as_slice())?;
                    self.collect(ops, rx_expected)
                }

                /// Writes command slot `index`.
                ///
                /// The eight command registers are separate PAC types, so the
                /// slot number cannot be an array index.
                fn write_command_word(
                    regs: &esp32p4_pac::$mod::RegisterBlock,
                    index: usize,
                    value: u32,
                ) {
                    match index {
                        0 => regs.comd0.write_bits(value),
                        1 => regs.comd1.write_bits(value),
                        2 => regs.comd2.write_bits(value),
                        3 => regs.comd3.write_bits(value),
                        4 => regs.comd4.write_bits(value),
                        5 => regs.comd5.write_bits(value),
                        6 => regs.comd6.write_bits(value),
                        _ => regs.comd7.write_bits(value),
                    }
                }

                /// Issues `commands` and waits for the list to finish.
                fn run_commands(&self, commands: &[u32]) -> Result<(), Error> {
                    let r = self.regs();

                    for (index, &value) in commands.iter().enumerate() {
                        Self::write_command_word(r, index, value);
                    }

                    // Start from a clean set of latched events.
                    r.int_clr.write_bits(EVENT_MASK);
                    r.ctr.trigger_trans_start();

                    let deadline = crate::delay::micros() + self.timeout_us;
                    let outcome = loop {
                        let raw = r.int_raw.read_bits();
                        if raw & Interrupt::Nack.mask() != 0 {
                            break Err(Error::NoAcknowledge);
                        }
                        if raw & Interrupt::ArbitrationLost.mask() != 0 {
                            break Err(Error::ArbitrationLost);
                        }
                        if raw & (Interrupt::TimeOut.mask() | Interrupt::SclTimeOut.mask()) != 0
                            || crate::delay::micros() > deadline
                        {
                            break Err(Error::Timeout);
                        }
                        if raw & (Interrupt::TransferComplete.mask() | Interrupt::EndDetect.mask())
                            != 0
                        {
                            break Ok(());
                        }
                        core::hint::spin_loop();
                    };

                    r.int_clr.write_bits(EVENT_MASK);
                    if outcome == Err(Error::Timeout) {
                        // Leave the peripheral in a defined state: a slave may
                        // still hold SDA low.
                        r.ctr.trigger_fsm_rst();
                    }
                    outcome
                }

                /// Moves the received bytes from the FIFO into the caller's
                /// buffers, in order.
                fn collect(&self, ops: &mut [Operation<'_>], expected: usize) -> Result<(), Error> {
                    let r = self.regs();
                    let mut remaining = (r.sr.rxfifo_cnt() as usize).min(expected);

                    for operation in ops.iter_mut() {
                        if let Operation::Read(buffer) = operation {
                            for slot in buffer.iter_mut() {
                                if remaining == 0 {
                                    return Err(Error::Timeout);
                                }
                                *slot = r.data.read_bits() as u8;
                                remaining -= 1;
                            }
                        }
                    }
                    Ok(())
                }

                // ============================================================
                // Bus recovery
                // ============================================================

                /// Pulses SCL until a slave that holds SDA low lets go.
                ///
                /// Nine pulses cover one byte plus the acknowledge bit, the
                /// longest a slave can stall the bus.
                pub fn clear_bus(&self) {
                    let r = self.regs();
                    r.scl_sp_conf.write_scl_rst_slv_num(9);
                    r.scl_sp_conf.set_scl_rst_slv_en();
                    r.ctr.trigger_conf_upgate();
                    crate::spin::spin_wait("I2C bus recovery", || {
                        !r.scl_sp_conf.scl_rst_slv_en()
                    });
                }

                /// Resets the controller's state machine.
                pub fn reset_fsm(&self) {
                    self.regs().ctr.trigger_fsm_rst();
                }

                /// Whether the bus is currently busy.
                #[inline]
                pub fn is_bus_busy(&self) -> bool {
                    self.regs().sr.bus_busy()
                }

                // ============================================================
                // Interrupts
                // ============================================================

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

                /// Whether `interrupt` is pending, enabled or not.
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
            }
        }
    };
}

impl_i2c!(
    I2c0,
    I2C0,
    i2c0,
    0,
    I2C0_SCL_PAD_OUT,
    I2C0_SCL_PAD_IN,
    I2C0_SDA_PAD_OUT,
    I2C0_SDA_PAD_IN,
    peri_clk_ctrl10
);
impl_i2c!(
    I2c1,
    I2C1,
    i2c1,
    1,
    I2C1_SCL_PAD_OUT,
    I2C1_SCL_PAD_IN,
    I2C1_SDA_PAD_OUT,
    I2C1_SDA_PAD_IN,
    peri_clk_ctrl11
);

/// Generates one I2C slave type from the shared register layout.
///
/// The parameters match [`impl_i2c!`]: the master and the slave of one
/// controller are the same peripheral, so they share the clock gate, the reset
/// and the pad signals, and the two types are kept apart only so that the
/// peripheral can be owned by one role at a time.
macro_rules! impl_i2c_slave {
    ($Name:ident, $PAC:ident, $mod:ident, $NUM:literal, $scl_out:ident, $scl_in:ident, $sda_out:ident, $sda_in:ident, $div_ctrl:ident, $irq:ident) => {
        paste! {
            /// I2C slave driver owning the PAC peripheral.
            pub struct $Name {
                i2c: $PAC,
                timeout_us: u64,
            }

            impl $Name {
                /// Controller number, 0 or 1.
                pub const NUM: u8 = $NUM;

                /// The interrupt matrix source of this controller.
                ///
                /// A slave is serviced from an interrupt in practice: while it
                /// holds SCL the master is blocked inside its own transfer, so
                /// a polling loop never gets to run.
                pub const SOURCE: crate::interrupt::Interrupt = crate::interrupt::Interrupt::$irq;

                /// Obtains a handle without ownership checks.
                ///
                /// The deadline and the address are left as the peripheral has
                /// them; use [`set_timeout_us`](Self::set_timeout_us) and
                /// [`set_address`](Self::set_address) on the owning instance.
                ///
                /// # Safety
                ///
                /// The caller must ensure the controller was initialised and
                /// that the returned handle is not used to reconfigure it
                /// concurrently.
                #[inline]
                pub unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self {
                        i2c: unsafe { $PAC::steal() },
                        timeout_us: SlaveConfig::default().timeout_us,
                    })
                }

                /// The underlying register block.
                #[inline]
                pub fn regs(&self) -> &esp32p4_pac::$mod::RegisterBlock {
                    &self.i2c
                }

                /// Returns the PAC peripheral.
                pub fn release(self) -> $PAC {
                    self.i2c
                }

                /// Overrides the deadline of the blocking calls.
                #[inline]
                pub fn set_timeout_us(&mut self, timeout_us: u64) {
                    self.timeout_us = timeout_us;
                }

                // ============================================================
                // Initialisation
                // ============================================================

                /// Initialises the controller as a slave and applies `config`.
                ///
                /// # Panics
                ///
                /// Panics if `config.address` is outside the 7-bit range.
                pub fn init(i2c: $PAC, config: &SlaveConfig, clkrst: &HP_SYS_CLKRST) -> Self {
                    assert!(
                        config.address <= 0x7f,
                        "I2C slave address must be 7 bits: {}",
                        config.address
                    );

                    // Bus clock, reset pulse, controller clock and source.
                    clkrst.soc_clk_ctrl2.[<set_reg_i2c $NUM _apb_clk_en>]();
                    clkrst.hp_rst_en1.[<set_reg_rst_en_i2c $NUM>]();
                    clkrst.hp_rst_en1.[<clr_reg_rst_en_i2c $NUM>]();
                    clkrst.peri_clk_ctrl10.[<set_reg_i2c $NUM _clk_en>]();
                    match config.clock {
                        I2cClock::Xtal => {
                            clkrst.peri_clk_ctrl10.[<clr_reg_i2c $NUM _clk_src_sel>]();
                        }
                        I2cClock::RcFast => {
                            clkrst.peri_clk_ctrl10.[<set_reg_i2c $NUM _clk_src_sel>]();
                        }
                    }
                    // The slave does not generate SCL, so the core clock only
                    // sets the resolution of the hold and sample delays: run it
                    // undivided.
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_num>](0);
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_numerator>](0);
                    clkrst.$div_ctrl.[<write_reg_i2c $NUM _clk_div_denominator>](0);

                    let i2c = Self {
                        i2c,
                        timeout_us: config.timeout_us,
                    };

                    let r = i2c.regs();
                    // Slave, open-drain outputs: the peripheral pulls the pads
                    // low and the pull-ups bring them back high.
                    r.ctr.clr_ms_mode();
                    r.ctr.clr_sda_force_out();
                    r.ctr.clr_scl_force_out();
                    r.ctr.clr_arbitration_en();
                    r.ctr.clr_rx_lsb_first();
                    r.ctr.clr_tx_lsb_first();
                    // Transmit as soon as the master addresses a read, instead
                    // of waiting for `TRANS_START` before every transfer.
                    r.ctr.set_slv_tx_auto_start_en();
                    if config.general_call {
                        r.ctr.set_addr_broadcasting_en();
                    } else {
                        r.ctr.clr_addr_broadcasting_en();
                    }

                    r.fifo_conf.clr_nonfifo_en();
                    // Pointer mode: the watermarks then count what the FIFOs
                    // hold rather than where the pointers are, which is what
                    // the receive side of a slave is serviced by.
                    r.fifo_conf.set_fifo_prt_en();
                    r.fifo_conf
                        .write_txfifo_wm_thrhd((FIFO_DEPTH / 2) as u32);
                    r.fifo_conf
                        .write_rxfifo_wm_thrhd((FIFO_DEPTH / 2) as u32);
                    // A full FIFO must stretch the clock, not refuse the byte:
                    // with this set the slave answers with a NACK instead, and
                    // the master never learns that it has to wait.
                    r.ctr.clr_rx_full_ack_level();
                    r.fifo_conf.set_tx_fifo_rst();
                    r.fifo_conf.set_rx_fifo_rst();
                    r.fifo_conf.clr_tx_fifo_rst();
                    r.fifo_conf.clr_rx_fifo_rst();

                    r.slave_addr.write_slave_addr(u32::from(config.address));
                    r.slave_addr.clr_addr_10bit_en();

                    // Ten core cycles: the sample point of an incoming bit and
                    // the hold time of an outgoing one.
                    r.sda_hold.write_time(10);
                    r.sda_sample.write_time(10);
                    // Only the value: the SCL timeout aborts master transfers,
                    // and a slave has no command list to abort.
                    r.to.write_time_out_value(0x1f);

                    r.scl_stretch_conf.write_stretch_protect_num(0x3ff);
                    if config.stretch {
                        r.scl_stretch_conf.set_slave_scl_stretch_en();
                    } else {
                        r.scl_stretch_conf.clr_slave_scl_stretch_en();
                    }
                    r.scl_stretch_conf.trigger_slave_scl_stretch_clr();

                    if config.glitch_filter {
                        r.filter_cfg.set_scl_filter_en();
                        r.filter_cfg.set_sda_filter_en();
                    } else {
                        r.filter_cfg.clr_scl_filter_en();
                        r.filter_cfg.clr_sda_filter_en();
                    }

                    r.int_clr.write_bits(ALL_INTERRUPTS);
                    r.ctr.trigger_conf_upgate();
                    i2c
                }

                /// Routes `scl` and `sda` to this controller.
                ///
                /// Both pads become open-drain, with the output enable coming
                /// from the peripheral, which is what lets the bus be shared.
                pub fn set_pins<SCL: PinOps, SDA: PinOps>(&self, mut scl: SCL, mut sda: SDA) {
                    scl.route_output_signal_open_drain(OutputSignal::$scl_out);
                    scl.route_input_signal(InputSignal::$scl_in);
                    scl.set_open_drain_enabled(true);
                    sda.route_output_signal_open_drain(OutputSignal::$sda_out);
                    sda.route_input_signal(InputSignal::$sda_in);
                    sda.set_open_drain_enabled(true);
                }

                // ============================================================
                // Address
                // ============================================================

                /// The address the slave answers to.
                #[inline]
                pub fn address(&self) -> u8 {
                    self.regs().slave_addr.slave_addr() as u8
                }

                /// Changes the address.
                ///
                /// The write is guarded by reset mode on the master side; a
                /// slave must simply not be addressed while this runs.
                pub fn set_address(&self, address: u8) -> Result<(), Error> {
                    if address > 0x7f {
                        return Err(Error::InvalidAddress);
                    }
                    self.regs()
                        .slave_addr
                        .write_slave_addr(u32::from(address));
                    Ok(())
                }

                /// Enables or disables clock stretching.
                pub fn set_stretch(&self, enabled: bool) {
                    let r = self.regs();
                    if enabled {
                        r.scl_stretch_conf.set_slave_scl_stretch_en();
                    } else {
                        r.scl_stretch_conf.clr_slave_scl_stretch_en();
                        r.scl_stretch_conf.trigger_slave_scl_stretch_clr();
                    }
                }

                /// Enables or disables answering addresses without a
                /// `TRANS_START` first.
                pub fn set_auto_start(&self, enabled: bool) {
                    let r = self.regs();
                    if enabled {
                        r.ctr.set_slv_tx_auto_start_en();
                    } else {
                        r.ctr.clr_slv_tx_auto_start_en();
                    }
                }

                // ============================================================
                // Transfers
                // ============================================================

                /// Waits for the master to finish writing and copies what it sent.
                ///
                /// The end of a transfer is a latched stop condition, so a call
                /// made after the master is done returns at once. The flag is
                /// cleared on return; a call with no transfer in flight waits
                /// for [`SlaveConfig::timeout_us`] and then reports
                /// [`Error::Timeout`].
                ///
                /// Bytes beyond `buffer.len()` are dropped, so a short buffer
                /// cannot leave a full FIFO stalling the bus. Any stretch found
                /// on the way is released, because the FIFO has just been
                /// drained.
                pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
                    let r = self.regs();
                    let deadline = crate::delay::micros() + self.timeout_us;
                    let mut count = 0;

                    loop {
                        count += self.read_available(&mut buffer[count..]);
                        if count == buffer.len() {
                            while self.rx_fifo_count() != 0 {
                                let _ = r.data.read_bits();
                            }
                        }
                        self.release_stretch();

                        if r.int_raw.read_bits() & Interrupt::TransferComplete.mask() != 0 {
                            self.clear_interrupt(Interrupt::TransferComplete);
                            return Ok(count);
                        }
                        if crate::delay::micros() > deadline {
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                }

                /// Queues `data` for the master's next read.
                ///
                /// Waits for room in the transmit FIFO, bounded by
                /// [`SlaveConfig::timeout_us`], and releases a pending
                /// address-match stretch so the transfer can start. Returns the
                /// number of bytes queued.
                pub fn write(&self, data: &[u8]) -> Result<usize, Error> {
                    let deadline = crate::delay::micros() + self.timeout_us;
                    let mut queued = 0;

                    while queued < data.len() {
                        queued += self.write_available(&data[queued..]);
                        self.release_stretch();
                        if queued == data.len() {
                            break;
                        }
                        if crate::delay::micros() > deadline {
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    Ok(queued)
                }

                /// Copies what the receive FIFO holds, without waiting.
                pub fn read_available(&self, buffer: &mut [u8]) -> usize {
                    let r = self.regs();
                    let count = (r.sr.rxfifo_cnt() as usize).min(buffer.len());
                    for slot in buffer[..count].iter_mut() {
                        *slot = r.data.read_bits() as u8;
                    }
                    count
                }

                /// Queues as much of `data` as the transmit FIFO takes, without
                /// waiting.
                pub fn write_available(&self, data: &[u8]) -> usize {
                    let r = self.regs();
                    let free = FIFO_DEPTH - r.sr.txfifo_cnt() as usize;
                    let count = free.min(data.len());
                    for &byte in &data[..count] {
                        r.data.write_bits(u32::from(byte));
                    }
                    count
                }

                // ============================================================
                // Status
                // ============================================================

                /// Whether the master has this slave's address on the bus right
                /// now.
                #[inline]
                pub fn is_addressed(&self) -> bool {
                    self.regs().sr.slave_addressed()
                }

                /// Whether the current transfer is a read of this slave.
                ///
                /// Only meaningful while [`is_addressed`](Self::is_addressed).
                #[inline]
                pub fn master_requests_read(&self) -> bool {
                    let r = self.regs();
                    r.sr.slave_addressed() && r.sr.slave_rw()
                }

                /// Why SCL is held low, if it is.
                pub fn stretch_cause(&self) -> Option<StretchCause> {
                    match self.regs().sr.stretch_cause() {
                        0 => Some(StretchCause::AddressMatch),
                        1 => Some(StretchCause::TransmitEmpty),
                        2 => Some(StretchCause::ReceiveFull),
                        3 => Some(StretchCause::SendingAck),
                        _ => None,
                    }
                }

                /// Releases SCL after a stretch.
                #[inline]
                pub fn release_stretch(&self) {
                    self.regs().scl_stretch_conf.trigger_slave_scl_stretch_clr();
                }

                /// Whether a START condition was seen since the last clear.
                #[inline]
                pub fn start_detected(&self) -> bool {
                    self.regs().int_raw.read_bits() & Interrupt::StartDetect.mask() != 0
                }

                /// Clears the START condition flag.
                #[inline]
                pub fn clear_start(&self) {
                    self.clear_interrupt(Interrupt::StartDetect);
                }

                /// Whether a STOP condition was seen since the last clear.
                #[inline]
                pub fn stop_detected(&self) -> bool {
                    self.regs().int_raw.read_bits() & Interrupt::TransferComplete.mask() != 0
                }

                /// Clears the STOP condition flag.
                #[inline]
                pub fn clear_stop(&self) {
                    self.clear_interrupt(Interrupt::TransferComplete);
                }

                /// Bytes waiting in the receive FIFO.
                #[inline]
                pub fn rx_fifo_count(&self) -> u8 {
                    self.regs().sr.rxfifo_cnt() as u8
                }

                /// Bytes waiting in the transmit FIFO.
                #[inline]
                pub fn tx_fifo_count(&self) -> u8 {
                    self.regs().sr.txfifo_cnt() as u8
                }

                /// Empties both FIFOs.
                pub fn reset_fifos(&self) {
                    let r = self.regs();
                    r.fifo_conf.set_tx_fifo_rst();
                    r.fifo_conf.set_rx_fifo_rst();
                    r.fifo_conf.clr_tx_fifo_rst();
                    r.fifo_conf.clr_rx_fifo_rst();
                }

                /// Whether the bus is currently busy.
                #[inline]
                pub fn is_bus_busy(&self) -> bool {
                    self.regs().sr.bus_busy()
                }

                // ============================================================
                // Interrupts
                // ============================================================

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

                /// Whether `interrupt` is pending, enabled or not.
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
            }
        }
    };
}

impl_i2c_slave!(
    I2c0Slave,
    I2C0,
    i2c0,
    0,
    I2C0_SCL_PAD_OUT,
    I2C0_SCL_PAD_IN,
    I2C0_SDA_PAD_OUT,
    I2C0_SDA_PAD_IN,
    peri_clk_ctrl10,
    I2C0
);
impl_i2c_slave!(
    I2c1Slave,
    I2C1,
    i2c1,
    1,
    I2C1_SCL_PAD_OUT,
    I2C1_SCL_PAD_IN,
    I2C1_SDA_PAD_OUT,
    I2C1_SDA_PAD_IN,
    peri_clk_ctrl11,
    I2C1
);
