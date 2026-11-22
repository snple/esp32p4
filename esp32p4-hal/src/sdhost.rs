//! SD/MMC host controller (`SDHOST`, a Synopsys DesignWare Mobile Storage
//! Host).
//!
//! The controller speaks the SD bus protocol: it shifts command frames out of
//! `CMD`, collects the response in `RESP0..RESP3`, and moves block data through
//! a FIFO that the controller's own DMA engine (the IDMAC) empties into memory.
//!
//! [`SdHost`] is the controller: clock, bus width, commands and block
//! transfers. [`SdCard`] is the card on top of it — the identification sequence
//! from the SD specification, the capacity read out of the CSD, and block reads
//! and writes with 512-byte blocks.
//!
//! # Features
//!
//! - 1-bit and 4-bit data buses, card clock derived from the 160 MHz PLL
//! - Commands with no response, 48-bit (R1/R3/R6/R7) and 136-bit (R2) responses
//! - Block reads and writes (CMD17/18/24/25) through the internal DMA, with the
//!   controller's automatic CMD12 for multi-block transfers
//! - Card identification (CMD0/8/2/3/9/7, ACMD41/6) and CSD decoding
//! - CMD6 high-speed mode: 40 MHz instead of 25 MHz when the card offers it
//! - The card's own write-protect flags from the CSD, which refuse writes
//!
//! # Pin Mapping
//!
//! The P4 gives the two card slots different pin paths: slot 0 (the slot the
//! on-chip SD pads belong to) goes through the IO MUX on fixed pads, slot 1
//! through the GPIO matrix. Commands are addressed to card 0 here, so the
//! signals are the pads' `SD1_*` IO MUX functions:
//!
//! | Signal | IO MUX function | Pad on the P4 package |
//! |--------|-----------------|------------------------|
//! | CLK | `SD1_CCLK_PAD` | GPIO43 |
//! | CMD | `SD1_CCMD_PAD` | GPIO44 |
//! | DAT0-3 | `SD1_CDATA0..3_PAD` | GPIO39-42 |
//!
//! [`SdHost::set_pins`] takes any pads; those without the function panic at that
//! call rather than silently doing nothing.
//!
//! # Example
//!
//! ```ignore
//! let host = SdHost::init(dp.SDHOST, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST, &dp.PMU);
//! host.set_pins(
//!     pins.pin::<43>(),
//!     pins.pin::<44>(),
//!     (pins.pin::<39>(), pins.pin::<40>(), pins.pin::<41>(), pins.pin::<42>()),
//! );
//!
//! let card = SdCard::init(&host)?;
//! let mut block = [0u8; 512];
//! card.read_blocks(&host, 0, &mut block)?;
//! ```
//!
//! # Scope
//!
//! Not covered yet: SDIO function access (CMD52/53 and the card interrupt),
//! UHS-I signalling and MMC's 8-bit bus. The P4 package routes four data lines,
//! so [`BusWidth::Bit4`] is the widest bus the controller is used with here.
//!
//! High speed needs nothing beyond CMD6: the card stays at 3.3 V, which the
//! board already supplies. UHS-I would need 1.8 V switching on the card supply,
//! which this board does not have.

use core::cell::UnsafeCell;

use esp32p4_pac::{HP_SYS_CLKRST, LP_AON_CLKRST, PMU, SDHOST};

use crate::cache;
use crate::gpio::{MuxFunction, PinOps};
use crate::spin;

/// Clock the card clock divider is fed with: `PLL_F160M` through the module
/// divider, which is left at 1.
const MODULE_HZ: u32 = 160_000_000;

/// Block size the card layer uses.
pub const BLOCK_SIZE: usize = 512;

/// Longest block transfer one call accepts, in blocks: the staging buffer is
/// 2 KB, the same as the controller's FIFO.
const MAX_BLOCKS: usize = 4;

/// Bytes the staging buffer holds.
const MAX_TRANSFER: usize = MAX_BLOCKS * BLOCK_SIZE;

/// Card clock during identification: the SD specification caps it at 400 kHz.
const IDENTIFICATION_HZ: u32 = 400_000;

/// Card clock once the card is identified, if it has no high-speed mode.
const DEFAULT_SPEED_HZ: u32 = 25_000_000;

/// Card clock in high-speed mode, which `CMD6` has to enable first.
///
/// High speed allows up to 50 MHz. The divider is `160 MHz / (2 * n)`, so the
/// choices below that cap are 40 MHz (`n = 2`) and 80 MHz (`n = 1`), and 40 MHz
/// is the one to ask for.
const HIGH_SPEED_HZ: u32 = 40_000_000;

/// Bytes of function status `CMD6` returns: 512 bits.
const SWITCH_STATUS_LEN: usize = 64;

/// Function group 1 is the bus "access mode", the one that selects high speed.
const SWITCH_ACCESS_MODE: u8 = 1;

/// Function 1 of the access-mode group is high speed (50 MHz, 3.3 V).
const SWITCH_HIGH_SPEED: u8 = 1;

/// How often ACMD41 may be retried. One attempt takes a good fraction of a
/// millisecond at [`IDENTIFICATION_HZ`], so this covers the second a card may
/// need to finish its own initialisation.
const OCR_POLL_ATTEMPTS: u32 = 2_000;

/// `SDHOST_RINTSTS` / `SDHOST_INTMASK` bits.
///
/// The SVD models them as one 16-bit field, so the meaning of each bit lives
/// here.
mod event {
    /// The card answered with an error bit set in its R1 response.
    pub const RESPONSE_ERROR: u32 = 1 << 1;
    /// Command finished.
    pub const COMMAND_DONE: u32 = 1 << 2;
    /// Data transfer finished.
    pub const DATA_OVER: u32 = 1 << 3;
    /// Response CRC mismatch.
    pub const RESPONSE_CRC: u32 = 1 << 6;
    /// Data CRC mismatch.
    pub const DATA_CRC: u32 = 1 << 7;
    /// No response within the response timeout.
    pub const RESPONSE_TIMEOUT: u32 = 1 << 8;
    /// No data within the data timeout.
    pub const DATA_TIMEOUT: u32 = 1 << 9;
    /// Host did not feed or drain the FIFO in time.
    pub const HOST_TIMEOUT: u32 = 1 << 10;
    /// FIFO overrun or underrun.
    pub const FIFO_ERROR: u32 = 1 << 11;
    /// Write to a register the controller locked.
    pub const HARDWARE_LOCKED: u32 = 1 << 12;
    /// Start bit of the response was missing.
    pub const START_BIT_ERROR: u32 = 1 << 13;
    /// The automatic CMD12 finished.
    pub const AUTO_COMMAND_DONE: u32 = 1 << 14;
    /// End bit of the response was missing.
    pub const END_BIT_ERROR: u32 = 1 << 15;
}

/// Command status bits that mean the command failed.
const COMMAND_FAILURES: u32 = event::RESPONSE_ERROR
    | event::RESPONSE_CRC
    | event::RESPONSE_TIMEOUT
    | event::START_BIT_ERROR
    | event::END_BIT_ERROR
    | event::HARDWARE_LOCKED;

/// Data status bits that mean the transfer failed. The command phase of a data
/// request reports its own failures through the same register.
const DATA_FAILURES: u32 = event::DATA_TIMEOUT
    | event::DATA_CRC
    | event::HOST_TIMEOUT
    | event::FIFO_ERROR
    | COMMAND_FAILURES;

/// Status bits to clear before a command.
const COMMAND_ERRORS: u32 = event::COMMAND_DONE | COMMAND_FAILURES;

/// Status bits to clear before a data transfer.
const DATA_ERRORS: u32 = event::DATA_OVER | event::AUTO_COMMAND_DONE | DATA_FAILURES;

/// Bits of an R1 response that report an error. The card state field, the
/// "ready for data" bit and the APP_CMD flag are not errors.
const R1_ERRORS: u32 = (1 << 31)
    | (1 << 30)
    | (1 << 29)
    | (1 << 26)
    | (1 << 25)
    | (1 << 23)
    | (1 << 22)
    | (1 << 21)
    | (1 << 20)
    | (1 << 19)
    | (1 << 16)
    | (1 << 15)
    | (1 << 13)
    | (1 << 3);

/// Interrupts the internal DMA latches in `IDSTS`: transfer done, receive done,
/// fatal bus error, descriptor unavailable, card error summary and the two
/// summaries. The status bits only appear if the matching interrupt is on.
const IDMAC_INTERRUPTS: u32 =
    (1 << 0) | (1 << 1) | (1 << 2) | (1 << 4) | (1 << 5) | (1 << 8) | (1 << 9);

/// Internal DMA descriptor flags (TRM 57.8, table 57.8-1).
const DESC_OWNER: u32 = 1 << 31;
const DESC_FIRST: u32 = 1 << 3;
const DESC_LAST: u32 = 1 << 2;

/// Iteration bound for the response to show up in `RESP0`.
///
/// The controller sets `COMMAND_DONE` when the command frame leaves, half a
/// frame before the card's answer is complete, and a 48-bit frame takes 120 us
/// at the 400 kHz identification clock. A quarter of [`spin::SPIN_TIMEOUT`] is
/// about 0.65 ms, which covers the wait with room to spare; anything shorter
/// turns a slow-but-correct answer into a timeout, and anything longer only
/// slows down commands the card really does ignore.
const RESPONSE_SETTLE: u32 = spin::SPIN_TIMEOUT / 4;

/// Iteration bound for a data phase: a card may take hundreds of milliseconds
/// to program a block before it releases DAT0.
const BLOCK_TIMEOUT: u32 = 8 * spin::SPIN_TIMEOUT;

/// Data bus width.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusWidth {
    /// DAT0 only.
    #[default]
    Bit1,
    /// DAT0-DAT3.
    Bit4,
}

/// Shape of the response a command expects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseKind {
    /// No response line at all, as for CMD0.
    None,
    /// 48-bit response with a CRC, R1/R6/R7 style.
    Short,
    /// 48-bit response followed by the card holding DAT0 busy (CMD7, CMD12).
    ShortBusy,
    /// 48-bit response without a CRC, R3/R4 style (OCR, SDIO registers).
    ShortNoCrc,
    /// 136-bit response, R2 style (CID and CSD).
    Long,
}

/// What a command returned.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Response {
    /// The command expects no response.
    None,
    /// `RESP0`: the whole 48-bit frame minus the start and end bits.
    Short(u32),
    /// The 136-bit frame, most significant word first.
    Long([u32; 4]),
}

/// Errors a command or transfer can report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The controller never took the command (its own register stayed busy).
    CommandNotAccepted,
    /// The card did not answer.
    ResponseTimeout,
    /// The response arrived with a bad CRC.
    ResponseCrc,
    /// The card reported an error in its response.
    ResponseError,
    /// The response started or ended at the wrong bit.
    ResponseBit,
    /// A register write hit a locked register.
    HardwareLocked,
    /// The card never sent or accepted data.
    DataTimeout,
    /// The data arrived with a bad CRC.
    DataCrc,
    /// The FIFO overran or underran.
    FifoError,
    /// The internal DMA engine could not access memory.
    DmaBusFault,
    /// The internal DMA engine found no usable descriptor.
    DescriptorUnavailable,
    /// The internal DMA engine reported a card error summary.
    CardErrorSummary,
    /// The card's own write-protect flags are set.
    WriteProtected,
    /// A transfer asked for a shape this driver does not implement.
    Unsupported,
    /// The card did not reach the state the sequence expects.
    CardNotReady,
}

/// SD/MMC host controller.
pub struct SdHost {
    sdhost: SDHOST,
    /// Staging area for the internal DMA: the engine needs word-aligned
    /// buffers, the caller's slices are allowed to be anything.
    bounce: UnsafeCell<Bounce>,
}

impl SdHost {
    /// Takes the controller, powers and clocks it, and resets the datapath.
    ///
    /// The reset outputs the controller's card clock; call
    /// [`set_clock`](Self::set_clock) before the first command.
    pub fn init(sdhost: SDHOST, clkrst: &HP_SYS_CLKRST, lp: &LP_AON_CLKRST, pmu: &PMU) -> Self {
        power_card_domain(pmu);
        clock_peripheral(clkrst, lp);

        let r = &sdhost;
        // The DesignWare reset clears FIFO, DMA and the card-facing state.
        r.ctrl.set_controller_reset();
        r.ctrl.set_fifo_reset();
        r.ctrl.set_dma_reset();
        spin::spin_wait("SDHOST reset", || {
            let c = r.ctrl.read_val();
            !c.controller_reset() && !c.fifo_reset() && !c.dma_reset()
        });

        // Nothing is interrupt driven here: every wait polls the status
        // registers.
        r.intmask.write_bits(0);
        r.ctrl.clr_int_enable();
        r.rintsts.write_bits(u32::MAX);
        r.idsts.write_bits(u32::MAX);

        // Long enough for the slowest card and for a multi-block transfer.
        r.tmout.write_response_timeout(0xff);
        r.tmout.write_data_timeout(0xff_ffff);

        Self {
            sdhost,
            bounce: UnsafeCell::new(Bounce::new()),
        }
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
            sdhost: unsafe { SDHOST::steal() },
            bounce: UnsafeCell::new(Bounce::new()),
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &esp32p4_pac::sdhost::RegisterBlock {
        &self.sdhost
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> SDHOST {
        self.sdhost
    }

    /// Routes the clock, command and data lines to the card slot.
    ///
    /// CMD and DAT0-3 are bidirectional, so their input buffers and pull-ups
    /// are enabled as well; the pads' `SD1_*` functions drive the outputs.
    pub fn set_pins<CLK: PinOps, CMD: PinOps, D0: PinOps, D1: PinOps, D2: PinOps, D3: PinOps>(
        &self,
        mut clk: CLK,
        mut cmd: CMD,
        data: (D0, D1, D2, D3),
    ) {
        clk.set_mux_function(MuxFunction::Sd1Cclk);

        cmd.set_mux_function(MuxFunction::Sd1Ccmd);
        cmd.set_input_buffer_enabled(true);
        cmd.set_pull_up_enabled(true);

        let (mut d0, mut d1, mut d2, mut d3) = data;
        let functions = [
            MuxFunction::Sd1Cdata0,
            MuxFunction::Sd1Cdata1,
            MuxFunction::Sd1Cdata2,
            MuxFunction::Sd1Cdata3,
        ];
        let pins = [&mut d0 as &mut dyn PinOps, &mut d1, &mut d2, &mut d3];
        for (index, pin) in pins.into_iter().enumerate() {
            pin.set_mux_function(functions[index]);
            pin.set_input_buffer_enabled(true);
            pin.set_pull_up_enabled(true);
        }
    }

    /// Sets the data bus width.
    ///
    /// The card has to be told the same width (ACMD6); do that before widening
    /// the controller.
    pub fn set_bus_width(&self, width: BusWidth) {
        let r = self.regs();
        match width {
            BusWidth::Bit1 => r.ctype.write_card_width4(0),
            BusWidth::Bit4 => r.ctype.write_card_width4(1),
        }
    }

    /// Programs the card clock, rounded down to what the divider can produce.
    ///
    /// The card clock is `160 MHz / (2 * divider)`, so the slowest clock is
    /// about 313 kHz — inside the 100-400 kHz the identification phase asks
    /// for.
    pub fn set_clock(&self, hz: u32) {
        let divider = card_divider(hz);
        let r = self.regs();

        // Clock register changes only reach the card interface after an
        // "update clock registers only" command, which needs the clock off.
        r.clkena.write_cclk_enable(0);
        self.update_clock_registers();

        r.clkdiv.write_clk_divider0(divider);
        // Divider 0 feeds the card clock; the P4's SD pads belong to the first
        // card slot.
        r.clksrc.write_clksrc(0);

        r.clkena.write_cclk_enable(1);
        // Keep the clock running while the card is idle: the card needs it for
        // its own timeout detection.
        r.clkena.write_lp_enable(0);
        self.update_clock_registers();
    }

    /// Card clock the divider currently produces, in hertz.
    pub fn frequency(&self) -> u32 {
        let divider = self.regs().clkdiv.clk_divider0();
        if divider == 0 {
            MODULE_HZ
        } else {
            MODULE_HZ / (2 * divider)
        }
    }

    /// Sends one command and waits for it to complete.
    pub fn send_command(
        &self,
        index: u8,
        argument: u32,
        kind: ResponseKind,
    ) -> Result<Response, Error> {
        let r = self.regs();
        r.rintsts.write_int_status_raw(COMMAND_ERRORS);
        r.cmdarg.write_bits(argument);

        r.cmd.write_index(u32::from(index));
        r.cmd.write_card_number(0);
        r.cmd.set_use_hole();
        r.cmd.set_wait_prvdata_complete();
        if matches!(kind, ResponseKind::None) {
            r.cmd.clr_response_expect();
        } else {
            r.cmd.set_response_expect();
        }
        if matches!(kind, ResponseKind::Long) {
            r.cmd.set_response_length();
        } else {
            r.cmd.clr_response_length();
        }
        if matches!(
            kind,
            ResponseKind::Short | ResponseKind::ShortBusy | ResponseKind::Long
        ) {
            r.cmd.set_check_response_crc();
        } else {
            r.cmd.clr_check_response_crc();
        }
        r.cmd.clr_data_expected();
        r.cmd.clr_stop_abort_cmd();
        r.cmd.clr_send_auto_stop();
        r.cmd.clr_update_clock_registers_only();
        r.cmd.set_start_cmd();
        wait_command_accepted(r)?;

        if matches!(kind, ResponseKind::None) {
            return Ok(Response::None);
        }

        let completed = spin::try_spin_wait(|| {
            r.rintsts.int_status_raw() & (event::COMMAND_DONE | COMMAND_FAILURES) != 0
        });
        let status = r.rintsts.int_status_raw();
        command_status(status)?;
        if !completed {
            // Neither done nor failed: the controller is not clocking at all.
            return Err(Error::CommandNotAccepted);
        }

        // `COMMAND_DONE` shows up before the response lands in `RESP0`, so wait
        // for the status register to report this command's index; the comment
        // on `RESPONSE_SETTLE` explains how long that can take at the
        // identification clock.
        let settled = spin::try_spin_wait_max(RESPONSE_SETTLE, || {
            r.status.response_index() == u32::from(index)
        });

        // A card that ignores a command can still let `COMMAND_DONE` through
        // without `RESPONSE_TIMEOUT`, and then `RESP0` holds the previous
        // command's answer: reading it invents a response. The response index
        // is the tell, but the controller only updates it for frames it can
        // check — R3/R4 (sent as `ShortNoCrc`) and R2 leave the field alone,
        // which is why it is checked for the CRC-carrying kinds only.
        if matches!(kind, ResponseKind::Short | ResponseKind::ShortBusy) && !settled {
            return Err(Error::ResponseTimeout);
        }

        // The controller files a 136-bit frame as four words, lowest bits in
        // `RESP0`; report it most significant word first so the bit numbering
        // matches the specification.
        let response = match kind {
            ResponseKind::Long => Response::Long([
                r.resp3.read_bits(),
                r.resp2.read_bits(),
                r.resp1.read_bits(),
                r.resp0.read_bits(),
            ]),
            _ => Response::Short(r.resp0.read_bits()),
        };
        r.rintsts.write_int_status_raw(COMMAND_ERRORS);

        if matches!(kind, ResponseKind::ShortBusy) {
            wait_not_busy(r)?;
        }
        Ok(response)
    }

    /// Reads `buf.len() / 512` blocks starting at `block`.
    ///
    /// The controller's internal DMA moves the data; `buf` is copied through
    /// the driver's staging buffer, so it needs no alignment.
    ///
    /// # Panics
    ///
    /// Panics unless `buf` holds one to four whole blocks.
    pub fn read_blocks(&self, block: u32, buf: &mut [u8]) -> Result<(), Error> {
        let blocks = check_blocks(buf.len());
        let index = if blocks == 1 { 17 } else { 18 };

        self.prepare_data_transfer(buf.len());
        self.start_data_command(index, block, BLOCK_SIZE, buf.len(), false)?;
        self.finish_data_transfer()?;
        if blocks > 1 {
            self.stop_transfer()?;
        }

        let bounce = self.bounce();
        bounce.collect(buf.len());
        buf.copy_from_slice(&bounce.data()[..buf.len()]);
        Ok(())
    }

    /// Writes `buf.len() / 512` blocks starting at `block`.
    ///
    /// The controller's internal DMA moves the data; `buf` is copied through
    /// the driver's staging buffer, so it needs no alignment.
    ///
    /// # Panics
    ///
    /// Panics unless `buf` holds one to four whole blocks.
    pub fn write_blocks(&self, block: u32, buf: &[u8]) -> Result<(), Error> {
        let blocks = check_blocks(buf.len());
        let index = if blocks == 1 { 24 } else { 25 };

        self.bounce().data_mut()[..buf.len()].copy_from_slice(buf);
        self.prepare_data_transfer(buf.len());
        self.start_data_command(index, block, BLOCK_SIZE, buf.len(), true)?;
        self.finish_data_transfer()?;
        if blocks > 1 {
            self.stop_transfer()?;
        }
        Ok(())
    }

    /// Reads `buf.len()` bytes of command data, one block of `block_size`.
    ///
    /// `CMD6` is the reason this exists: it answers with 64 bytes of function
    /// status, which is neither a memory block nor a whole 512-byte sector.
    ///
    /// # Panics
    ///
    /// Panics unless `buf` is exactly one `block_size` block of at most
    /// `MAX_TRANSFER` bytes, with a length the FIFO can take (a multiple of
    /// four).
    pub fn read_command_data(
        &self,
        index: u8,
        argument: u32,
        block_size: usize,
        buf: &mut [u8],
    ) -> Result<(), Error> {
        assert_eq!(buf.len(), block_size, "command data is exactly one block");
        assert!(
            buf.len() <= MAX_TRANSFER && buf.len().is_multiple_of(4),
            "command data is at most {MAX_TRANSFER} bytes, in whole words"
        );

        self.prepare_data_transfer(buf.len());
        self.start_data_command(index, argument, block_size, buf.len(), false)?;
        self.finish_data_transfer()?;

        let bounce = self.bounce();
        bounce.collect(buf.len());
        buf.copy_from_slice(&bounce.data()[..buf.len()]);
        Ok(())
    }

    /// Points the descriptor at `blocks` blocks of the staging buffer and arms
    /// the internal DMA for it.
    fn prepare_data_transfer(&self, len: usize) {
        let r = self.regs();
        reset_fifo(r);
        r.rintsts.write_int_status_raw(DATA_ERRORS);
        r.idsts.write_bits(u32::MAX);

        let bounce = self.bounce();
        bounce.describe(len as u32);
        bounce.stage(len);

        // The FIFO belongs to the DMA engine from here on.
        r.ctrl.set_dma_enable();
        r.ctrl.set_use_internal_dma();
        // `BMOD.SWR` clears the IDMAC's own registers, `DBADDR` included, so it
        // has to run before the descriptor base is written.
        r.bmod.set_swr();
        r.idinten.write_bits(IDMAC_INTERRUPTS);
        r.dbaddr.write_bits(bounce.descriptor_address());
        r.bmod.set_de();
        r.bmod.set_fb();
        r.pldmnd.write_bits(1);
    }

    /// Programs `BLKSIZ`/`BYTCNT` and starts a data command.
    fn start_data_command(
        &self,
        index: u8,
        argument: u32,
        block_size: usize,
        len: usize,
        write: bool,
    ) -> Result<(), Error> {
        let r = self.regs();
        r.rintsts.write_int_status_raw(COMMAND_ERRORS);
        r.blksiz.write_bits(block_size as u32);
        r.bytcnt.write_bits(len as u32);

        r.cmdarg.write_bits(argument);
        r.cmd.write_index(u32::from(index));
        r.cmd.write_card_number(0);
        r.cmd.set_use_hole();
        r.cmd.set_wait_prvdata_complete();
        r.cmd.set_response_expect();
        r.cmd.clr_response_length();
        r.cmd.set_check_response_crc();
        r.cmd.set_data_expected();
        if write {
            r.cmd.set_read_write();
        } else {
            r.cmd.clr_read_write();
        }
        // The controller's automatic CMD12 never completes a multi-block
        // *write* (the data phase hangs before `DATA_OVER`), so the stop
        // command is sent by hand in `stop_transfer` for both directions.
        r.cmd.clr_send_auto_stop();
        r.cmd.clr_stop_abort_cmd();
        r.cmd.clr_update_clock_registers_only();
        r.cmd.set_start_cmd();
        wait_command_accepted(r)?;

        // The data phase only starts once the card accepted the command, so a
        // rejection has to be reported here rather than as a data timeout.
        spin::try_spin_wait(|| {
            r.rintsts.int_status_raw() & (event::COMMAND_DONE | COMMAND_FAILURES) != 0
        });
        command_status(r.rintsts.int_status_raw())?;
        let settled = spin::try_spin_wait_max(RESPONSE_SETTLE, || {
            r.status.response_index() == u32::from(index)
        });
        if !settled {
            return Err(Error::ResponseTimeout);
        }
        if r.resp0.read_bits() & R1_ERRORS != 0 {
            return Err(Error::ResponseError);
        }
        Ok(())
    }

    /// Waits for the data phase to finish and checks its status bits.
    fn finish_data_transfer(&self) -> Result<(), Error> {
        let r = self.regs();
        let mut last = 0;
        let completed = spin::try_spin_wait_max(BLOCK_TIMEOUT, || {
            last = r.rintsts.int_status_raw();
            last & (event::DATA_OVER | DATA_FAILURES) != 0
        });
        r.rintsts.write_int_status_raw(DATA_ERRORS);
        let idmac = r.idsts.read_bits();
        r.idsts.write_bits(u32::MAX);
        stop_dma(r);

        if idmac & (1 << 2) != 0 {
            return Err(Error::DmaBusFault);
        }
        if idmac & (1 << 4) != 0 {
            return Err(Error::DescriptorUnavailable);
        }
        if idmac & (1 << 5) != 0 {
            return Err(Error::CardErrorSummary);
        }
        data_status(last)?;
        if !completed {
            return Err(Error::DataTimeout);
        }
        Ok(())
    }

    /// Ends a multi-block transfer with CMD12 and waits for the card to let go
    /// of DAT0.
    fn stop_transfer(&self) -> Result<(), Error> {
        self.send_command(12, 0, ResponseKind::ShortBusy)?;
        wait_not_busy(self.regs())
    }

    /// The DMA staging area.
    ///
    /// Transfers take `&self` and the controller drives one card slot, so a
    /// second caller cannot be inside a transfer at the same time.
    #[inline]
    fn bounce(&self) -> &Bounce {
        // SAFETY: see the comment on the method.
        unsafe { &*self.bounce.get() }
    }

    /// Commits clock register writes: the controller only latches them when
    /// this special command runs.
    fn update_clock_registers(&self) {
        let r = self.regs();
        r.cmdarg.write_bits(0);
        r.cmd.write_index(0);
        r.cmd.write_card_number(0);
        r.cmd.set_update_clock_registers_only();
        r.cmd.set_wait_prvdata_complete();
        r.cmd.clr_response_expect();
        r.cmd.clr_data_expected();
        r.cmd.set_start_cmd();
        // A clock update that never lands means the controller is dead, which
        // the first command would report anyway.
        spin::spin_wait("SDHOST clock update", || !r.cmd.start_cmd());
    }
}

/// Turns a command's raw status bits into an error, ignoring the harmless ones.
fn command_status(status: u32) -> Result<(), Error> {
    if status & event::HARDWARE_LOCKED != 0 {
        Err(Error::HardwareLocked)
    } else if status & event::RESPONSE_TIMEOUT != 0 {
        Err(Error::ResponseTimeout)
    } else if status & event::RESPONSE_CRC != 0 {
        Err(Error::ResponseCrc)
    } else if status & (event::START_BIT_ERROR | event::END_BIT_ERROR) != 0 {
        Err(Error::ResponseBit)
    } else if status & event::RESPONSE_ERROR != 0 {
        Err(Error::ResponseError)
    } else {
        Ok(())
    }
}

/// Turns a data phase's raw status bits into an error.
fn data_status(status: u32) -> Result<(), Error> {
    if status & event::FIFO_ERROR != 0 {
        Err(Error::FifoError)
    } else if status & (event::DATA_TIMEOUT | event::HOST_TIMEOUT) != 0 {
        Err(Error::DataTimeout)
    } else if status & event::DATA_CRC != 0 {
        Err(Error::DataCrc)
    } else if status & (event::START_BIT_ERROR | event::END_BIT_ERROR) != 0 {
        Err(Error::ResponseBit)
    } else {
        command_status(status)
    }
}

/// Waits until the controller accepts the command it was given.
fn wait_command_accepted(r: &esp32p4_pac::sdhost::RegisterBlock) -> Result<(), Error> {
    if !spin::try_spin_wait(|| !r.cmd.start_cmd()) {
        return Err(Error::CommandNotAccepted);
    }
    // The hardware lock error means the write did not take effect at all.
    if r.rintsts.int_status_raw() & event::HARDWARE_LOCKED != 0 {
        r.rintsts.write_int_status_raw(event::HARDWARE_LOCKED);
        return Err(Error::HardwareLocked);
    }
    Ok(())
}

/// Waits until the card releases DAT0, which it holds busy after some commands
/// and while it programs a block.
fn wait_not_busy(r: &esp32p4_pac::sdhost::RegisterBlock) -> Result<(), Error> {
    if spin::try_spin_wait_max(BLOCK_TIMEOUT, || !r.status.data_busy()) {
        Ok(())
    } else {
        Err(Error::CardNotReady)
    }
}

/// Releases the FIFO from the internal DMA engine.
fn stop_dma(r: &esp32p4_pac::sdhost::RegisterBlock) {
    r.ctrl.clr_use_internal_dma();
    r.ctrl.clr_dma_enable();
    r.bmod.clr_de();
    r.bmod.clr_fb();
    r.ctrl.set_dma_reset();
}

/// Empties the data FIFO and waits for the reset to land.
fn reset_fifo(r: &esp32p4_pac::sdhost::RegisterBlock) {
    r.ctrl.set_fifo_reset();
    spin::spin_wait("SDHOST FIFO reset", || !r.ctrl.read_val().fifo_reset());
}

/// Divider for a target card clock; `0` means "no division".
fn card_divider(hz: u32) -> u32 {
    if hz == 0 || hz >= MODULE_HZ {
        return 0;
    }
    (MODULE_HZ.div_ceil(2 * hz)).clamp(1, 255)
}

/// Validates a block buffer and returns its block count.
fn check_blocks(len: usize) -> usize {
    assert!(
        len.is_multiple_of(BLOCK_SIZE) && len > 0,
        "an SD transfer is a whole number of {BLOCK_SIZE}-byte blocks"
    );
    let blocks = len / BLOCK_SIZE;
    assert!(
        blocks <= MAX_BLOCKS,
        "one transfer covers at most {MAX_BLOCKS} blocks"
    );
    blocks
}

/// Descriptor and data staging in one block, so both land where the DMA engine
/// can reach them with one address computation.
#[repr(C, align(8))]
struct Bounce {
    descriptor: [u32; 4],
    data: [u8; MAX_TRANSFER],
}

impl Bounce {
    const fn new() -> Self {
        Self {
            descriptor: [0; 4],
            data: [0; MAX_TRANSFER],
        }
    }

    /// Address of the descriptor as the DMA engine has to see it.
    fn descriptor_address(&self) -> u32 {
        self as *const Self as u32
    }

    /// Data buffer as the DMA engine has to see it.
    fn data_address(&self) -> u32 {
        self.data.as_ptr() as u32
    }

    /// The staged data. The CPU writes and reads it through the cache, so
    /// [`stage`](Self::stage) and [`collect`](Self::collect) have to bracket
    /// the transfer.
    fn data(&self) -> &[u8] {
        &self.data
    }

    /// The staging buffer, mutably.
    ///
    /// Only one transfer is in flight at a time, and the reference never
    /// outlives the call that asked for it.
    #[allow(clippy::mut_from_ref)]
    fn data_mut(&self) -> &mut [u8] {
        // SAFETY: see the comment on the method.
        unsafe { &mut *core::ptr::addr_of!(self.data).cast_mut() }
    }

    /// Hands the descriptor and the data over to the DMA engine.
    fn stage(&self, len: usize) {
        cache::writeback_invalidate(self.descriptor_address() as *const _, 16);
        cache::writeback(self.data_address() as *const _, len);
    }

    /// Takes the received data back from the DMA engine.
    fn collect(&self, len: usize) {
        cache::invalidate(self.data_address() as *const _, len);
    }

    /// Points the one-entry descriptor ring at `len` bytes of the staging
    /// buffer.
    ///
    /// The words go through the CPU cache like any other memory; the caller is
    /// responsible for writing them back before pointing the engine at them.
    fn describe(&self, len: u32) {
        let base = self.descriptor_address();
        // SAFETY: `base` is this struct's descriptor, which only this driver
        // writes, and the offsets stay inside it.
        unsafe {
            core::ptr::write_volatile((base + 4) as *mut u32, len & 0x1fff);
            core::ptr::write_volatile((base + 8) as *mut u32, self.data_address());
            // No chaining: this one descriptor is the whole list, so `DES3`
            // stays zero and the engine stops when it is done.
            core::ptr::write_volatile((base + 12) as *mut u32, 0);
            // Owner last: the engine may pick the descriptor up as soon as it
            // sees the bit.
            core::ptr::write_volatile(base as *mut u32, DESC_OWNER | DESC_FIRST | DESC_LAST);
        }
    }
}

/// An SD memory card that finished identification.
pub struct SdCard {
    rca: u16,
    cid: [u32; 4],
    csd: [u32; 4],
    blocks: u32,
    high_capacity: bool,
    high_speed: bool,
    write_protected: bool,
}

impl SdCard {
    /// Identifies the card on `host` and leaves it at 25 MHz on a 4-bit bus.
    ///
    /// The sequence is the one from the SD specification: CMD0 to idle, CMD8
    /// to learn the supply voltage, ACMD41 until the card reports ready, CMD2
    /// for the CID, CMD3 for the address, CMD9 for the CSD, CMD7 to select it,
    /// ACMD6 for the bus width and CMD16 for the block length.
    pub fn init(host: &SdHost) -> Result<Self, Error> {
        host.set_bus_width(BusWidth::Bit1);
        host.set_clock(IDENTIFICATION_HZ);

        host.send_command(0, 0, ResponseKind::None)?;

        // A v1 card ignores CMD8 and times out, which is how they are told
        // apart from v2 cards.
        let high_capacity_capable = match host.send_command(8, 0x1aa, ResponseKind::Short) {
            Ok(Response::Short(response)) if response & 0xfff == 0x1aa => true,
            Ok(_) => return Err(Error::Unsupported),
            Err(Error::ResponseTimeout) => false,
            Err(error) => return Err(error),
        };

        // ACMD41: HCS asks for high capacity support, the lower bits offer the
        // 2.7-3.6 V range. Bit 31 stays clear until the card is ready.
        let argument = 0x00ff_8000 | if high_capacity_capable { 1 << 30 } else { 0 };
        let mut ready = false;
        for _ in 0..OCR_POLL_ATTEMPTS {
            host.send_command(55, 0, ResponseKind::Short)?;
            match host.send_command(41, argument, ResponseKind::ShortNoCrc) {
                Ok(Response::Short(ocr)) if ocr & (1 << 31) != 0 => {
                    ready = true;
                    break;
                }
                Ok(_) => {}
                // Some cards answer with an error while they are still busy.
                Err(Error::ResponseError) => {}
                Err(error) => return Err(error),
            }
        }
        if !ready {
            return Err(Error::CardNotReady);
        }

        let cid = match host.send_command(2, 0, ResponseKind::Long)? {
            Response::Long(words) => words,
            _ => return Err(Error::Unsupported),
        };
        let rca = match host.send_command(3, 0, ResponseKind::Short)? {
            Response::Short(response) => (response >> 16) as u16,
            _ => return Err(Error::Unsupported),
        };
        let csd = match host.send_command(9, u32::from(rca) << 16, ResponseKind::Long)? {
            Response::Long(words) => words,
            _ => return Err(Error::Unsupported),
        };
        host.send_command(7, u32::from(rca) << 16, ResponseKind::ShortBusy)?;

        let decoded = decode_csd(csd).ok_or(Error::Unsupported)?;

        // Four data lines, then the block length the driver always uses.
        host.send_command(55, u32::from(rca) << 16, ResponseKind::Short)?;
        host.send_command(6, 2, ResponseKind::Short)?;
        host.send_command(16, BLOCK_SIZE as u32, ResponseKind::Short)?;
        host.set_bus_width(BusWidth::Bit4);
        host.set_clock(DEFAULT_SPEED_HZ);

        // CMD6 is what unlocks 50 MHz, and only cards that advertise the switch
        // command class answer it.
        let high_speed = decoded.command_classes & (1 << 10) != 0 && enable_high_speed(host)?;

        Ok(Self {
            rca,
            cid,
            csd,
            blocks: decoded.blocks,
            high_capacity: decoded.high_capacity,
            high_speed,
            write_protected: decoded.write_protected,
        })
    }

    /// Relative card address assigned by CMD3.
    #[inline]
    pub fn rca(&self) -> u16 {
        self.rca
    }

    /// Card identification register, most significant word first.
    #[inline]
    pub fn cid(&self) -> [u32; 4] {
        self.cid
    }

    /// Card specific data register, most significant word first.
    #[inline]
    pub fn csd(&self) -> [u32; 4] {
        self.csd
    }

    /// Card size in 512-byte blocks.
    #[inline]
    pub fn blocks(&self) -> u32 {
        self.blocks
    }

    /// Whether the card addresses memory in 512-byte blocks (SDHC/SDXC) rather
    /// than in bytes (SDSC).
    #[inline]
    pub fn high_capacity(&self) -> bool {
        self.high_capacity
    }

    /// Whether `CMD6` put the card in high-speed mode and the clock went up to
    /// the high-speed rate. Cards without the switch function stay at 25 MHz.
    #[inline]
    pub fn high_speed(&self) -> bool {
        self.high_speed
    }

    /// Whether the card reads its own write-protect flags as set.
    ///
    /// This is the card's temporary or permanent write protection from the
    /// CSD, not a write-protect switch on the socket.
    #[inline]
    pub fn write_protected(&self) -> bool {
        self.write_protected
    }

    /// Reads whole blocks starting at `block`.
    ///
    /// # Panics
    ///
    /// See [`SdHost::read_blocks`].
    pub fn read_blocks(&self, host: &SdHost, block: u32, buf: &mut [u8]) -> Result<(), Error> {
        self.check_range(block, buf.len())?;
        host.read_blocks(self.address(block), buf)
    }

    /// Writes whole blocks starting at `block`.
    ///
    /// # Panics
    ///
    /// See [`SdHost::write_blocks`].
    pub fn write_blocks(&self, host: &SdHost, block: u32, buf: &[u8]) -> Result<(), Error> {
        if self.write_protected {
            return Err(Error::WriteProtected);
        }
        self.check_range(block, buf.len())?;
        host.write_blocks(self.address(block), buf)
    }

    /// Turns a block index into the argument the block commands expect: byte
    /// addressing for SDSC, block addressing for SDHC/SDXC.
    #[inline]
    fn address(&self, block: u32) -> u32 {
        if self.high_capacity {
            block
        } else {
            block * BLOCK_SIZE as u32
        }
    }

    fn check_range(&self, block: u32, len: usize) -> Result<(), Error> {
        let end = block as usize + len / BLOCK_SIZE;
        if end > self.blocks as usize {
            return Err(Error::Unsupported);
        }
        Ok(())
    }
}

/// Reads the capacity out of a CSD register, in 512-byte blocks, and whether
/// the card uses block addressing.
///
/// The CSD arrives as four words, most significant first, so bit 127 is the top
/// bit of `csd[0]`.
fn decode_csd(csd: [u32; 4]) -> Option<DecodedCsd> {
    let structure = csd[0] >> 30;
    let (blocks, high_capacity) = match structure {
        // Version 2.0 (SDHC/SDXC): C_SIZE counts 512 KB units, which is 1024
        // blocks each.
        1 => {
            let c_size = u64::from(((csd[1] & 0x3f) << 16) | ((csd[2] >> 16) & 0xffff));
            (((c_size + 1) * 1024) as u32, true)
        }
        // Version 1.0 (SDSC): capacity follows from the block count, the
        // multiplier and the read block length.
        0 => {
            let read_bl_len = (csd[1] >> 9) & 0xf;
            let c_size = ((csd[1] >> 13) & 0x3ff) << 2 | (csd[2] >> 30) & 0x3;
            let c_size_mult = (csd[2] >> 15) & 0x7;
            let bytes = (u64::from(c_size) + 1) << (c_size_mult + 2 + read_bl_len);
            ((bytes / BLOCK_SIZE as u64) as u32, false)
        }
        _ => return None,
    };

    Some(DecodedCsd {
        blocks,
        high_capacity,
        // Card command classes, CSD bits 95..84; class 10 is the switch
        // function command.
        command_classes: ((csd[1] >> 20) & 0xfff) as u16,
        // PERM_WRITE_PROTECT (bit 13) and TMP_WRITE_PROTECT (bit 12).
        write_protected: csd[3] & ((1 << 13) | (1 << 12)) != 0,
    })
}

/// The parts of the CSD the driver uses.
struct DecodedCsd {
    /// Capacity in 512-byte blocks.
    blocks: u32,
    /// Whether the card is addressed in blocks rather than bytes.
    high_capacity: bool,
    /// Command classes the card supports (CSD bits 95..84).
    command_classes: u16,
    /// Whether one of the card's own write-protect flags is set.
    write_protected: bool,
}

/// Asks the card to switch to high speed with `CMD6`, and raises the clock to
/// the high-speed rate if it accepted.
///
/// Mode 0 of `CMD6` reads the card's function status without changing
/// anything; mode 1 changes it. Both answer with the same 512-bit status, whose
/// bit numbering is the one in the specification: bit 511 is the first bit on
/// the bus.
fn enable_high_speed(host: &SdHost) -> Result<bool, Error> {
    let mut status = [0u8; SWITCH_STATUS_LEN];

    switch_function(host, 0, SWITCH_ACCESS_MODE, 0, &mut status)?;
    // Group 1's support mask is status bits 400..416, one bit per function.
    let supported = switch_field(&status, 400, 16);
    if supported & (1 << SWITCH_HIGH_SPEED) == 0 {
        return Ok(false);
    }

    switch_function(host, 1, SWITCH_ACCESS_MODE, SWITCH_HIGH_SPEED, &mut status)?;
    // The selection it landed on is status bits 376..380.
    if switch_field(&status, 376, 4) != u32::from(SWITCH_HIGH_SPEED) {
        return Ok(false);
    }

    host.set_clock(HIGH_SPEED_HZ);
    Ok(true)
}

/// Sends `CMD6` for one function group and collects its 512-bit status.
///
/// `mode` is 0 to query and 1 to switch. Every group other than `group` is
/// asked to stay as it is, which the specification spells as function `0xf`.
fn switch_function(
    host: &SdHost,
    mode: u32,
    group: u8,
    function: u8,
    status: &mut [u8; SWITCH_STATUS_LEN],
) -> Result<(), Error> {
    let shift = (group - 1) * 4;
    let keep = 0x00ff_ffff & !(0xf << shift);
    let argument = (mode << 31) | (u32::from(function) << shift) | keep;
    host.read_command_data(6, argument, SWITCH_STATUS_LEN, status)
}

/// One bit of the `CMD6` status, numbered as the specification numbers it
/// (511 is the first bit on the bus).
fn switch_bit(status: &[u8; SWITCH_STATUS_LEN], bit: usize) -> u32 {
    let index = SWITCH_STATUS_LEN * 8 - 1 - bit;
    u32::from(status[index / 8] >> (7 - index % 8) & 1)
}

/// A field of the `CMD6` status: `len` bits starting at `start`, which is the
/// field's least significant bit.
fn switch_field(status: &[u8; SWITCH_STATUS_LEN], start: usize, len: usize) -> u32 {
    let mut value = 0;
    for bit in (start..start + len).rev() {
        value = (value << 1) | switch_bit(status, bit);
    }
    value
}

/// Brings up the on-chip regulator that feeds the SD pads and the card.
///
/// The P4 routes the SD interface's IO supply through analog LDO channel 4.
/// Without it the card sees no supply and every command times out, which looks
/// like a wiring fault.
fn power_card_domain(pmu: &PMU) {
    // Limit the inrush current while the output capacitor charges.
    pmu.ext_ldo_p10p2a_ana.set_ana_0p2a_en_cur_lim_1();
    pmu.ext_ldo_p10p2a_ana.set_ana_0p2a_en_vdet_1();

    // Tie the output to the 3.3 V rail and switch it on: software picks the
    // tie-high bit, so the reference divider plays no part.
    pmu.ext_ldo_p10p2a.set_0p2a_force_tieh_sel_1();
    pmu.ext_ldo_p10p2a.write_0p2a_tieh_sel_1(0);
    pmu.ext_ldo_p10p2a.set_0p2a_tieh_1();
    pmu.ext_ldo_p10p2a.set_0p2a_xpd_1();

    // Let the output settle, then drop the inrush limit.
    for _ in 0..200_000 {
        core::hint::spin_loop();
    }
    pmu.ext_ldo_p10p2a_ana.clr_ana_0p2a_en_cur_lim_1();
}

/// Enables the bus clock, releases the reset and programs the card clock domain
/// (`sdio_ls_*` in `HP_SYS_CLKRST`).
fn clock_peripheral(clkrst: &HP_SYS_CLKRST, lp: &LP_AON_CLKRST) {
    clkrst.soc_clk_ctrl1.set_reg_sdmmc_sys_clk_en();
    lp.lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
        .set_lp_aonclkrst_rst_en_sdmmc();
    lp.lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
        .clr_lp_aonclkrst_rst_en_sdmmc();

    // 160 MHz PLL as the card clock source, no low-speed division.
    clkrst.ref_clk_ctrl2.set_reg_ref_160m_clk_en();
    clkrst.peri_clk_ctrl01.clr_reg_sdio_ls_clk_src_sel();
    clkrst.peri_clk_ctrl01.set_reg_sdio_hs_mode();
    clkrst.peri_clk_ctrl01.set_reg_sdio_ls_clk_en();

    // Drive, sample and internal clocks with their default phases.
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_clk_edge_n(0);
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_clk_edge_h(0);
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_clk_edge_l(0);
    clkrst.peri_clk_ctrl02.set_reg_sdio_ls_drv_clk_en();
    clkrst.peri_clk_ctrl02.set_reg_sdio_ls_sam_clk_en();
    clkrst.peri_clk_ctrl02.set_reg_sdio_ls_slf_clk_en();
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_drv_clk_edge_sel(1);
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_sam_clk_edge_sel(0);
    clkrst.peri_clk_ctrl02.write_reg_sdio_ls_slf_clk_edge_sel(0);
    clkrst
        .peri_clk_ctrl02
        .trigger_reg_sdio_ls_clk_edge_cfg_update();
}
