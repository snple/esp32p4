//! General-purpose SPI (GPSPI2/GPSPI3), master and slave.
//!
//! GP-SPI2 and GP-SPI3 are the two SPI controllers the application can use;
//! SPI0/SPI1 and the PSRAM controllers belong to the flash/PSRAM interface.
//!
//! Two transfer paths are available:
//!
//! - **CPU transfers** ([`Spi2::transfer`] and friends): the core fills and
//!   drains the controller's 64-byte buffer (`SPI_W0`..`SPI_W15`) and starts
//!   each transaction itself. No DMA channel is needed, and the length is
//!   limited to [`MAX_TRANSFER`] per chunk.
//! - **DMA transfers** ([`Spi2::transfer_dma`], [`Spi2::write_dma`],
//!   [`Spi2::read_dma`]): GDMA-AXI moves the data, so the length is bounded
//!   only by the descriptor chain, and the CPU is free while the bytes shift.
//!   The master still drives the transaction and CS; the channels have to be
//!   connected to [`AxiPeripheral::Spi2`] / [`AxiPeripheral::Spi3`] by the
//!   driver, so callers only supply the channels and their descriptors.
//!
//! # Features
//!
//! - Master and slave mode, full duplex and 1-bit (a transfer sends and receives
//!   at the same time through `MOSI`/`MISO`)
//! - Clock modes 0-3 (idle level and sample edge), MSB/LSB first
//! - Clock from `XTAL` (40 MHz) through the peripheral's own divider, up to
//!   40 MHz out; every GPIO can be SCK/MOSI/MISO/CS through the GPIO matrix
//! - Hardware chip select with configurable setup and hold time, held across
//!   the 64-byte chunks of one transfer
//! - Slave: full-duplex CPU transactions, with the length the master clocked
//!   reported back
//! - Master DMA in both directions, or one direction at a time in half-duplex
//!   mode (RX-only then gets its clock without a `0xFF` transmit phase)
//!
//! # Pin Mapping
//!
//! The signal indices come from [`crate::gpio::signal`]:
//!
//! | Signal | SPI2 | SPI3 |
//! |--------|------|------|
//! | SCK | `SPI2_CK_PAD_OUT/IN` | `SPI3_CK_PAD_OUT/IN` |
//! | MOSI | `SPI2_D_PAD_OUT/IN` | `SPI3_D_PAD_OUT/IN` |
//! | MISO | `SPI2_Q_PAD_OUT/IN` | `SPI3_QO_PAD_OUT/IN` |
//! | CS | `SPI2_CS_PAD_OUT/IN` | `SPI3_CS_PAD_OUT/IN` |
//!
//! GP-SPI3 has no IO MUX path, so its pins always go through the matrix.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::spi::{Config, Mode, Spi2};
//!
//! let spi = Spi2::init(dp.SPI2, &Config::default().frequency(10_000_000).mode(Mode::_0), &dp.HP_SYS_CLKRST);
//! spi.set_pins(pins.pin::<20>(), pins.pin::<21>(), pins.pin::<22>());
//! spi.set_cs(pins.pin::<23>());
//!
//! let mut rx = [0u8; 4];
//! spi.transfer(&[0x9f, 0, 0, 0], &mut rx);
//! ```
//!
//! # DMA Buffers
//!
//! Buffers handed to the DMA path have to live in internal SRAM and be word
//! aligned, because GDMA-AXI moves memory in 8-byte bursts. The byte count
//! itself is honoured — a 1001-byte receive left the bytes behind the buffer
//! untouched in the tests — so no slack has to be reserved. Descriptors use the
//! same format and rules as the rest of [`crate::dma`]: one `Descriptor` covers
//! up to 4095 bytes, and the array has to stay put while the transfer runs.
//!
//! # Scope
//!
//! Not covered yet: the 2/4/8-bit data modes, SCT (segmented-configure) and the
//! slave's DMA/segmented mode. The clock source is `XTAL`; the PLL source would
//! raise the ceiling from 40 to 80 MHz.

use esp32p4_pac::{HP_SYS_CLKRST, SPI2, SPI3};

use crate::dma::{self, AxiPeripheral, Chain, Descriptor, Owner, ReceiveChannel, TransmitChannel};
use crate::gpio::{InputSignal, OutputSignal, PinOps};
use crate::{cache, delay, spin};

/// Bytes the controller's CPU buffer holds, and therefore the longest single
/// transaction in this driver.
pub const MAX_TRANSFER: usize = 64;

/// Frequency of the clock source the driver programs.
const SOURCE_HZ: u32 = 40_000_000;

/// Largest pre-divider the `SPI_CLKDIV_PRE` field can hold.
const MAX_PRE: u32 = 16;

/// SPI clock mode: the idle level of SCK and the edge data is sampled on.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// CPOL 0, CPHA 0: idle low, sample on the rising edge.
    #[default]
    _0,
    /// CPOL 0, CPHA 1: idle low, sample on the falling edge.
    _1,
    /// CPOL 1, CPHA 0: idle high, sample on the falling edge.
    _2,
    /// CPOL 1, CPHA 1: idle high, sample on the rising edge.
    _3,
}

/// Order in which the bits of each byte leave and enter the shift register.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    /// Most significant bit first, the SPI default.
    #[default]
    MsbFirst,
    /// Least significant bit first.
    LsbFirst,
}

/// Master configuration.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// SCK frequency in Hz. Rounded down to what the divider can produce.
    pub frequency: u32,
    /// Clock mode.
    pub mode: Mode,
    /// Bit order within each byte.
    pub bit_order: BitOrder,
    /// CS setup time in SCK cycles, added to the fixed 1.5 cycle offset.
    /// Zero gives the minimum of 0.5 cycles.
    pub cs_setup: u8,
    /// CS hold time in SCK cycles, added to the fixed 1.5 cycle offset.
    /// Zero gives the minimum of 0.5 cycles.
    pub cs_hold: u8,
}

impl Config {
    /// Sets the SCK frequency, in hertz.
    pub const fn frequency(mut self, frequency: u32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Sets the clock mode.
    pub const fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the bit order.
    pub const fn bit_order(mut self, bit_order: BitOrder) -> Self {
        self.bit_order = bit_order;
        self
    }

    /// Sets the CS setup and hold times, in SCK cycles.
    pub const fn cs_timing(mut self, setup: u8, hold: u8) -> Self {
        self.cs_setup = setup;
        self.cs_hold = hold;
        self
    }
}

impl Default for Config {
    /// 1 MHz, mode 0, MSB first, minimum CS timing.
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: Mode::_0,
            bit_order: BitOrder::MsbFirst,
            cs_setup: 0,
            cs_hold: 0,
        }
    }
}

/// Slave configuration.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SlaveConfig {
    /// Clock mode; has to match the master's.
    pub mode: Mode,
    /// Bit order within each byte; has to match the master's.
    pub bit_order: BitOrder,
}

impl SlaveConfig {
    /// Sets the clock mode.
    pub const fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
}

/// One SCK divider: `SOURCE_HZ / (n * pre)`.
#[derive(Clone, Copy, Debug)]
struct Divider {
    pre: u32,
    n: u32,
    /// Actual frequency, in hertz.
    frequency: u32,
}

impl Divider {
    /// Picks the divider pair that lands closest to `hz`.
    ///
    /// `n` has to be at least 2 so the clock has one high and one low phase;
    /// for each `n` the pre-divider is rounded to the nearest value, and the
    /// closest pair wins — the same search the reference driver does.
    fn new(hz: u32) -> Self {
        if hz == 0 || hz >= SOURCE_HZ {
            return Self {
                pre: 1,
                n: 1,
                frequency: SOURCE_HZ,
            };
        }

        let mut best = Self {
            pre: 1,
            n: 2,
            frequency: SOURCE_HZ / 2,
        };
        let mut best_error = u32::MAX;
        for n in 2..=64 {
            let pre = (SOURCE_HZ / n + hz / 2) / hz;
            // The pre-divider field is four bits wide.
            let pre = pre.clamp(1, MAX_PRE);
            let frequency = SOURCE_HZ / (n * pre);
            let error = frequency.abs_diff(hz);
            if error < best_error {
                best_error = error;
                best = Self { pre, n, frequency };
            }
        }
        best
    }
}

/// Waits until `descriptor` is back with software, i.e. the engine stored or
/// sent the buffer it describes.
///
/// The owner bit is the only completion signal that does not depend on which
/// interrupt bit a direction uses, and it is written back by the engine itself.
fn wait_for_descriptor(descriptor: &Descriptor, deadline: u64) -> Result<(), dma::Error> {
    while descriptor.owner() == Owner::Dma {
        if delay::micros() > deadline {
            return Err(dma::Error::Timeout);
        }
        core::hint::spin_loop();
    }
    Ok(())
}

/// Generates a master type from the shared register layout.
macro_rules! impl_spi_master {
    (
        $Name:ident, $PAC:ident, $mod:ident, $NUM:literal,
        clock: $clock:ident, axi: $axi:ident,
        ck_out: $ck_out:ident, mosi_out: $mosi_out:ident, miso_in: $miso_in:ident, cs_out: $cs_out:ident
    ) => {
        /// SPI master owning the PAC peripheral.
        pub struct $Name {
            spi: $PAC,
        }

        impl $Name {
            /// SPI instance number, 2 or 3.
            pub const NUM: u8 = $NUM;

            /// Takes ownership, enables the clocks and programs `config`.
            ///
            /// # Panics
            ///
            /// Panics if `frequency` is above what the `XTAL` source can drive
            /// (40 MHz); use [`frequency`](Self::frequency) to read back what
            /// the divider actually produces.
            pub fn init(spi: $PAC, config: &Config, clkrst: &HP_SYS_CLKRST) -> Self {
                $clock(clkrst);

                let r = &spi;
                // Everything back to its reset value for the USR machine.
                r.spi_user.write_bits(0);
                r.spi_slave.write_bits(0);
                r.spi_dma_conf.write_bits(0);
                r.spi_user1.write_bits(0);

                let divider = Divider::new(config.frequency);
                assert!(
                    config.frequency <= SOURCE_HZ,
                    "SPI clock above the 40 MHz the XTAL source can drive"
                );
                r.spi_clock.write_bits(0);
                if divider.n == 1 {
                    r.spi_clock.set_spi_clk_equ_sysclk();
                } else {
                    r.spi_clock.write_spi_clkcnt_n(divider.n - 1);
                    // 50 % duty cycle: the high phase is half of the period.
                    r.spi_clock.write_spi_clkcnt_h(divider.n / 2 - 1);
                    r.spi_clock.write_spi_clkcnt_l(divider.n - 1);
                    r.spi_clock.write_spi_clkdiv_pre(divider.pre - 1);
                }

                Self::apply_bit_order(&spi, config.bit_order);

                // Full duplex, CS timing, clock phase.
                r.spi_user.set_spi_doutdin();
                if config.cs_setup > 0 {
                    r.spi_user.set_spi_cs_setup();
                    r.spi_user1
                        .write_spi_cs_setup_time(u32::from(config.cs_setup) - 1);
                }
                if config.cs_hold > 0 {
                    r.spi_user.set_spi_cs_hold();
                    r.spi_user1
                        .write_spi_cs_hold_time(u32::from(config.cs_hold) - 1);
                }
                Self::apply_mode(&spi, config.mode);

                // CS0 and the clock come from the peripheral, active low.
                r.spi_misc.clr_spi_cs0_dis();
                r.spi_misc.write_spi_master_cs_pol(0);
                r.spi_misc.clr_spi_ck_dis();

                Self::apply_config(&spi);
                Self { spi }
            }

            /// Obtains a handle without ownership checks.
            ///
            /// # Safety
            ///
            /// [`init`](Self::init) must have run, and the caller must not start
            /// a second transfer on the same instance.
            #[inline]
            pub unsafe fn steal() -> crate::Steal<Self> {
                crate::Steal(Self {
                    spi: unsafe { $PAC::steal() },
                })
            }

            /// The underlying register block.
            #[inline]
            pub fn regs(&self) -> &esp32p4_pac::$mod::RegisterBlock {
                &self.spi
            }

            /// Returns the PAC peripheral.
            #[inline]
            pub fn release(self) -> $PAC {
                self.spi
            }

            /// SCK frequency the divider actually produces, in hertz.
            #[inline]
            pub fn frequency(&self) -> u32 {
                Divider::new(self.programmed_frequency()).frequency
            }

            /// Switches the clock mode of a configured master.
            pub fn set_mode(&self, mode: Mode) {
                Self::apply_mode(&self.spi, mode);
                Self::apply_config(&self.spi);
            }

            /// Switches the bit order of a configured master.
            pub fn set_bit_order(&self, order: BitOrder) {
                Self::apply_bit_order(&self.spi, order);
                Self::apply_config(&self.spi);
            }

            /// Routes SCK, MOSI and MISO.
            ///
            /// Pins are ordinary GPIOs; the GPIO matrix carries the signals.
            pub fn set_pins<CK: PinOps, MOSI: PinOps, MISO: PinOps>(
                &self,
                mut ck: CK,
                mut mosi: MOSI,
                mut miso: MISO,
            ) {
                ck.route_output_signal(OutputSignal::$ck_out);
                mosi.route_output_signal(OutputSignal::$mosi_out);
                miso.route_input_signal(InputSignal::$miso_in);
            }

            /// Routes the hardware chip select.
            ///
            /// The peripheral drives CS low around every transfer and respects
            /// [`Config::cs_setup`] / [`Config::cs_hold`]; without this call the
            /// CS signal goes nowhere and the device has to be selected
            /// externally.
            pub fn set_cs<CS: PinOps>(&self, mut cs: CS) {
                cs.route_output_signal(OutputSignal::$cs_out);
            }

            /// Full-duplex transfer of `max(tx.len(), rx.len())` bytes.
            ///
            /// Bytes past the end of `tx` are sent as `0xFF`, and bytes past the
            /// end of `rx` are dropped. Returns the number of bytes exchanged,
            /// which is what the caller can rely on in both buffers.
            pub fn transfer(&self, tx: &[u8], rx: &mut [u8]) -> usize {
                let total = tx.len().max(rx.len());
                if total == 0 {
                    return 0;
                }
                self.transfer_chunks(tx, Some(rx));
                total
            }

            /// Sends `tx`, ignoring what comes back.
            pub fn write(&self, tx: &[u8]) {
                if !tx.is_empty() {
                    self.transfer_chunks(tx, None);
                }
            }

            /// Reads `rx` bytes, sending `0xFF` while the device clocks out.
            pub fn read(&self, rx: &mut [u8]) {
                if !rx.is_empty() {
                    self.transfer_chunks(&[], Some(rx));
                }
            }

            /// Waits until the controller has finished the current transaction.
            pub fn flush(&self) {
                let r = self.regs();
                spin::spin_wait("SPI transaction", || {
                    r.spi_dma_int_raw.spi_trans_done_int_raw()
                });
            }

            // ================================================================
            // DMA transfers (GDMA-AXI)
            // ================================================================

            /// Full-duplex DMA transfer of `tx.len()` bytes.
            ///
            /// `tx` is clocked out while the master clocks `rx` in, exactly like
            /// [`transfer`](Self::transfer) but without the 64-byte limit: the
            /// descriptors carry the data, and one descriptor covers up to 4095
            /// bytes. `descriptors` is split into a transmit half and a receive
            /// half, so it needs `2 * ceil(tx.len() / 4095)` entries.
            ///
            /// The channels are connected to this SPI instance, and configured
            /// for descriptor write-back, by this call, so they can be handed
            /// over without any preparation. They have to belong to
            /// [`AxiDma`](crate::dma::AxiDma); SPI is not on the AHB controller.
            ///
            /// Returns the number of bytes exchanged. `rx` is invalidated in the
            /// cache after the transfer, so the caller reads what the engine
            /// stored.
            ///
            /// # Errors
            ///
            /// [`dma::Error::NotEnoughDescriptors`] if the array is too small,
            /// the descriptor/FIFO variants if the engine rejects a descriptor,
            /// and [`dma::Error::Timeout`] if the transaction does not finish
            /// within `timeout_us`.
            ///
            /// # Panics
            ///
            /// Panics if the buffers differ in length; use
            /// [`write_dma`](Self::write_dma) or [`read_dma`](Self::read_dma)
            /// for a one-way transfer.
            pub fn transfer_dma(
                &self,
                tx_dma: &impl TransmitChannel,
                rx_dma: &impl ReceiveChannel,
                tx: &[u8],
                rx: &mut [u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<usize, dma::Error> {
                assert_eq!(
                    tx.len(),
                    rx.len(),
                    "a full-duplex DMA transfer needs equally long buffers"
                );
                let len = tx.len();
                if len == 0 {
                    return Ok(0);
                }
                let chunks = len.div_ceil(dma::descriptor::MAX_LEN);
                if descriptors.len() < 2 * chunks {
                    return Err(dma::Error::NotEnoughDescriptors {
                        needed: 2 * chunks,
                        available: descriptors.len(),
                    });
                }

                let (tx_descriptors, rx_descriptors) = descriptors.split_at(chunks);
                let tx_chain = Chain::new(tx_descriptors);
                let rx_chain = Chain::new(rx_descriptors);
                let tx_used = tx_chain.mount_eof(tx.as_ptr() as *mut u8, len)?;
                let rx_used = rx_chain.mount(rx.as_mut_ptr(), len)?;

                self.dma_setup(true, true, len as u32 * 8);
                Self::prepare_tx_dma(tx_dma, AxiPeripheral::$axi);
                Self::prepare_rx_dma(rx_dma, AxiPeripheral::$axi);
                cache::writeback(tx.as_ptr().cast(), len);
                cache::writeback_invalidate(rx.as_mut_ptr().cast(), len);

                let deadline = delay::micros() + timeout_us;
                rx_dma.start_chain(rx_chain.head());
                tx_dma.start_chain(tx_chain.head());

                let result = self
                    .run_dma(deadline)
                    .and_then(|()| wait_for_descriptor(&tx_descriptors[tx_used - 1], deadline))
                    .and_then(|()| wait_for_descriptor(&rx_descriptors[rx_used - 1], deadline));
                if result.is_err() {
                    tx_dma.stop_chain();
                    tx_dma.reset_channel();
                    rx_dma.stop_chain();
                    rx_dma.reset_channel();
                }
                self.end_dma();
                if result.is_ok() {
                    cache::invalidate(rx.as_mut_ptr().cast(), len);
                }
                result.map(|()| len)
            }

            /// Sends `tx` through the DMA engine and returns once it is out.
            ///
            /// Half duplex: only the transmit phase runs, so the controller
            /// needs no receive descriptor and the clock stops after
            /// `tx.len()` bytes. `descriptors` needs `ceil(tx.len() / 4095)`
            /// entries.
            ///
            /// # Errors
            ///
            /// See [`transfer_dma`](Self::transfer_dma).
            pub fn write_dma(
                &self,
                dma: &impl TransmitChannel,
                tx: &[u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<(), dma::Error> {
                if tx.is_empty() {
                    return Ok(());
                }
                let chain = Chain::new(descriptors);
                let used = chain.mount_eof(tx.as_ptr() as *mut u8, tx.len())?;

                self.dma_setup(true, false, tx.len() as u32 * 8);
                Self::prepare_tx_dma(dma, AxiPeripheral::$axi);
                cache::writeback(tx.as_ptr().cast(), tx.len());

                let deadline = delay::micros() + timeout_us;
                dma.start_chain(chain.head());

                let result = self
                    .run_dma(deadline)
                    .and_then(|()| wait_for_descriptor(&descriptors[used - 1], deadline));
                if result.is_err() {
                    dma.stop_chain();
                    dma.reset_channel();
                }
                self.end_dma();
                result
            }

            /// Clocks `rx.len()` bytes in through the DMA engine.
            ///
            /// Half duplex: only the receive phase runs, so nothing has to be
            /// sent as padding and `descriptors` needs
            /// `ceil(rx.len() / 4095)` entries.
            ///
            /// The buffer has to be word aligned; see the
            /// [module docs](self#dma-buffers).
            ///
            /// # Errors
            ///
            /// See [`transfer_dma`](Self::transfer_dma).
            pub fn read_dma(
                &self,
                dma: &impl ReceiveChannel,
                rx: &mut [u8],
                descriptors: &[Descriptor],
                timeout_us: u64,
            ) -> Result<(), dma::Error> {
                let len = rx.len();
                if len == 0 {
                    return Ok(());
                }
                let chain = Chain::new(descriptors);
                let used = chain.mount(rx.as_mut_ptr(), len)?;

                self.dma_setup(false, true, len as u32 * 8);
                Self::prepare_rx_dma(dma, AxiPeripheral::$axi);
                cache::writeback_invalidate(rx.as_mut_ptr().cast(), len);

                let deadline = delay::micros() + timeout_us;
                dma.start_chain(chain.head());

                let result = self
                    .run_dma(deadline)
                    .and_then(|()| wait_for_descriptor(&descriptors[used - 1], deadline));
                if result.is_err() {
                    dma.stop_chain();
                    dma.reset_channel();
                }
                self.end_dma();
                if result.is_ok() {
                    cache::invalidate(rx.as_mut_ptr().cast(), len);
                }
                result
            }

            /// Programs the controller for one DMA transaction.
            ///
            /// `bits` is the length of the enabled phases, `tx`/`rx` select
            /// them. The registers are shadowed, so the caller still has to
            /// apply them through [`apply_config`](Self::apply_config) before
            /// starting the transaction.
            fn dma_setup(&self, tx: bool, rx: bool, bits: u32) {
                let r = self.regs();
                // A DMA transfer is one transaction: there is no chunk boundary
                // the select line would have to stay low across.
                r.spi_misc.clr_spi_cs_keep_active();

                // Full duplex shifts both ways at once. A one-way transfer has
                // to switch to half duplex: in full duplex the clock waits for
                // a transmit byte that a receive-only transfer never provides.
                if tx && rx {
                    r.spi_user.set_spi_doutdin();
                } else {
                    r.spi_user.clr_spi_doutdin();
                }
                if tx {
                    r.spi_user.set_spi_usr_mosi();
                    // Do not start the clock before the engine delivered the
                    // first byte.
                    r.spi_slave.set_spi_mst_fd_wait_dma_tx_data();
                } else {
                    r.spi_user.clr_spi_usr_mosi();
                    r.spi_slave.clr_spi_mst_fd_wait_dma_tx_data();
                }
                if rx {
                    r.spi_user.set_spi_usr_miso();
                } else {
                    r.spi_user.clr_spi_usr_miso();
                }
                r.spi_ms_dlen.write_spi_ms_data_bitlen(bits - 1);

                // Both DMA FIFOs keep their pointers across transactions.
                r.spi_dma_conf.trigger_spi_dma_afifo_rst();
                r.spi_dma_conf.trigger_spi_rx_afifo_rst();
                // The engine's EOF comes from the end of the transaction, not
                // from a byte counter, and the master never segments.
                r.spi_dma_conf.clr_spi_rx_eof_en();
                r.spi_dma_conf.clr_spi_dma_slv_seg_trans_en();
                if tx {
                    r.spi_dma_conf.set_spi_dma_tx_ena();
                } else {
                    r.spi_dma_conf.clr_spi_dma_tx_ena();
                }
                if rx {
                    r.spi_dma_conf.set_spi_dma_rx_ena();
                } else {
                    r.spi_dma_conf.clr_spi_dma_rx_ena();
                }
                r.spi_dma_int_clr.write_bits(u32::MAX);
            }

            /// Connects and configures a transmit channel for SPI traffic.
            fn prepare_tx_dma(dma: &impl TransmitChannel, peripheral: AxiPeripheral) {
                dma.connect_to(peripheral);
                dma.check_owner(true);
                dma.descriptor_burst(true);
                // Hand each descriptor back once its buffer is out, and report
                // EOF only after the controller shifted the last byte.
                dma.auto_writeback(true);
                dma.eof_till_data_popped(true);
                dma.clear_all_interrupts();
                dma.reset_channel();
            }

            /// Connects and configures a receive channel for SPI traffic.
            fn prepare_rx_dma(dma: &impl ReceiveChannel, peripheral: AxiPeripheral) {
                dma.connect_to(peripheral);
                dma.check_owner(true);
                dma.descriptor_burst(true);
                dma.clear_all_interrupts();
                dma.reset_channel();
            }

            /// Applies the configuration, starts the transaction and waits for
            /// the controller to finish shifting.
            fn run_dma(&self, deadline: u64) -> Result<(), dma::Error> {
                let r = self.regs();
                Self::apply_config(&self.spi);
                r.spi_cmd.set_spi_usr();
                while !r.spi_dma_int_raw.spi_trans_done_int_raw() {
                    if delay::micros() > deadline {
                        return Err(dma::Error::Timeout);
                    }
                    core::hint::spin_loop();
                }
                Ok(())
            }

            /// Disables the DMA paths again, so the next CPU transfer sees an
            /// idle controller.
            fn end_dma(&self) {
                let r = self.regs();
                r.spi_dma_conf.clr_spi_dma_tx_ena();
                r.spi_dma_conf.clr_spi_dma_rx_ena();
                r.spi_dma_int_clr.write_bits(u32::MAX);
            }

            /// Programs the bit order of both directions.
            fn apply_bit_order(spi: &$PAC, order: BitOrder) {
                let bits = match order {
                    BitOrder::MsbFirst => 0,
                    BitOrder::LsbFirst => 1,
                };
                spi.spi_ctrl.write_spi_wr_bit_order(bits);
                spi.spi_ctrl.write_spi_rd_bit_order(bits);
            }

            /// Programs the clock mode.
            fn apply_mode(spi: &$PAC, mode: Mode) {
                match mode {
                    Mode::_0 => {
                        spi.spi_misc.clr_spi_ck_idle_edge();
                        spi.spi_user.clr_spi_ck_out_edge();
                    }
                    Mode::_1 => {
                        spi.spi_misc.clr_spi_ck_idle_edge();
                        spi.spi_user.set_spi_ck_out_edge();
                    }
                    Mode::_2 => {
                        spi.spi_misc.set_spi_ck_idle_edge();
                        spi.spi_user.set_spi_ck_out_edge();
                    }
                    Mode::_3 => {
                        spi.spi_misc.set_spi_ck_idle_edge();
                        spi.spi_user.clr_spi_ck_out_edge();
                    }
                }
            }

            /// Frequency the divider was programmed with, read back from the
            /// register so [`frequency`](Self::frequency) needs no stored state.
            fn programmed_frequency(&self) -> u32 {
                let r = self.regs();
                if r.spi_clock.spi_clk_equ_sysclk() {
                    return SOURCE_HZ;
                }
                let n = r.spi_clock.spi_clkcnt_n() + 1;
                let pre = r.spi_clock.spi_clkdiv_pre() + 1;
                SOURCE_HZ / (n * pre)
            }

            /// Pushes the shadowed configuration into the SPI clock domain.
            fn apply_config(spi: &$PAC) {
                spi.spi_cmd.trigger_spi_update();
                spin::spin_wait("SPI register update", || {
                    spi.spi_cmd.read_bits() & esp32p4_pac::$mod::SpiCmd::SPI_UPDATE_MASK == 0
                });
            }

            /// Runs the transfer in 64-byte chunks, holding CS between them.
            fn transfer_chunks(&self, tx: &[u8], rx: Option<&mut [u8]>) {
                let total = match &rx {
                    Some(rx) => tx.len().max(rx.len()),
                    None => tx.len(),
                };
                let r = self.regs();

                // The CPU path fills and drains the same FIFOs a DMA transfer
                // would, so make sure no DMA path is still attached to them.
                r.spi_dma_conf.clr_spi_dma_tx_ena();
                r.spi_dma_conf.clr_spi_dma_rx_ena();

                // Full duplex again: a half-duplex DMA transfer leaves the
                // controller in the one-way mode.
                r.spi_user.set_spi_doutdin();

                // The clock only runs while data is available to shift out, so
                // the transmit phase stays enabled even for a read: those bytes
                // are the `0xFF` padding.
                r.spi_user.set_spi_usr_mosi();
                r.spi_slave.set_spi_mst_fd_wait_dma_tx_data();
                if rx.is_some() {
                    r.spi_user.set_spi_usr_miso();
                } else {
                    r.spi_user.clr_spi_usr_miso();
                }

                let mut rx = rx;
                let mut offset = 0;
                while offset < total {
                    let len = (total - offset).min(MAX_TRANSFER);
                    let last = offset + len == total;

                    // The data buffer sits behind a pair of CPU FIFOs whose
                    // pointers persist across transactions; both have to be
                    // reset or the next transfer reads and writes the wrong
                    // slots (TRM 46.5.6, `SPI_BUF_AFIFO_RST`/`SPI_RX_AFIFO_RST`).
                    r.spi_dma_conf.trigger_spi_buf_afifo_rst();
                    for index in 0..len {
                        let byte = tx.get(offset + index).copied().unwrap_or(0xff);
                        self.write_byte(index, byte);
                    }
                    r.spi_dma_conf.trigger_spi_rx_afifo_rst();
                    r.spi_ms_dlen.write_spi_ms_data_bitlen(len as u32 * 8 - 1);

                    // CS stays low over the chunk boundaries of one transfer.
                    if last {
                        r.spi_misc.clr_spi_cs_keep_active();
                    } else {
                        r.spi_misc.set_spi_cs_keep_active();
                    }

                    Self::apply_config(&self.spi);
                    r.spi_dma_int_clr.trigger_spi_trans_done_int_clr();
                    r.spi_cmd.set_spi_usr();
                    spin::spin_wait("SPI transaction", || {
                        r.spi_dma_int_raw.spi_trans_done_int_raw()
                    });

                    if let Some(rx) = rx.as_deref_mut() {
                        for index in offset..(offset + len).min(rx.len()) {
                            rx[index] = self.read_byte(index - offset);
                        }
                    }
                    offset += len;
                }
                r.spi_misc.clr_spi_cs_keep_active();
            }

            /// Reads one byte out of the controller's data buffer.
            fn read_byte(&self, index: usize) -> u8 {
                (self.read_word(index / 4) >> (8 * (index % 4))) as u8
            }

            /// Writes one byte into the controller's data buffer.
            fn write_byte(&self, index: usize, byte: u8) {
                let shift = 8 * (index % 4);
                let word = self.read_word(index / 4);
                self.write_word(
                    index / 4,
                    (word & !(0xffu32 << shift)) | (u32::from(byte) << shift),
                );
            }

            fn read_word(&self, index: usize) -> u32 {
                // SAFETY: `SPI_W0`..`SPI_W15` are the 16 consecutive words of the
                // data buffer (TRM 46.5.6); the index stays inside it.
                unsafe { core::ptr::read_volatile(self.word_ptr(index)) }
            }

            fn write_word(&self, index: usize, value: u32) {
                // SAFETY: see `read_word`.
                unsafe { core::ptr::write_volatile(self.word_ptr(index) as *mut u32, value) }
            }

            fn word_ptr(&self, index: usize) -> *const u32 {
                debug_assert!(index < MAX_TRANSFER / 4);
                // SAFETY: the peripheral is map'd for the program's lifetime.
                unsafe { (core::ptr::addr_of!(self.regs().spi_w0) as *const u32).add(index) }
            }
        }
    };
}

/// Enables the bus, APB and functional clocks of GP-SPI2.
fn clock_spi2(clkrst: &HP_SYS_CLKRST) {
    clkrst.soc_clk_ctrl1.set_reg_gpspi2_sys_clk_en();
    clkrst.soc_clk_ctrl2.set_reg_gpspi2_apb_clk_en();
    clkrst.hp_rst_en2.set_reg_rst_en_spi2();
    clkrst.hp_rst_en2.clr_reg_rst_en_spi2();
    // XTAL into the peripheral, no pre-division.
    clkrst.peri_clk_ctrl116.write_reg_gpspi2_clk_src_sel(0);
    clkrst.peri_clk_ctrl116.write_reg_gpspi2_hs_clk_div_num(0);
    clkrst.peri_clk_ctrl116.write_reg_gpspi2_mst_clk_div_num(0);
    clkrst.peri_clk_ctrl116.set_reg_gpspi2_hs_clk_en();
    clkrst.peri_clk_ctrl116.set_reg_gpspi2_mst_clk_en();
}

/// Enables the bus, APB and functional clocks of GP-SPI3.
fn clock_spi3(clkrst: &HP_SYS_CLKRST) {
    clkrst.soc_clk_ctrl1.set_reg_gpspi3_sys_clk_en();
    clkrst.soc_clk_ctrl2.set_reg_gpspi3_apb_clk_en();
    clkrst.hp_rst_en2.set_reg_rst_en_spi3();
    clkrst.hp_rst_en2.clr_reg_rst_en_spi3();
    clkrst.peri_clk_ctrl116.write_reg_gpspi3_clk_src_sel(0);
    clkrst.peri_clk_ctrl117.write_reg_gpspi3_mst_clk_div_num(0);
    clkrst.peri_clk_ctrl117.write_reg_gpspi3_hs_clk_div_num(0);
    clkrst.peri_clk_ctrl116.set_reg_gpspi3_hs_clk_en();
    clkrst.peri_clk_ctrl117.set_reg_gpspi3_mst_clk_en();
}

impl_spi_master!(
    Spi2,
    SPI2,
    spi2,
    2,
    clock: clock_spi2, axi: Spi2,
    ck_out: SPI2_CK_PAD_OUT,
    mosi_out: SPI2_D_PAD_OUT,
    miso_in: SPI2_Q_PAD_IN,
    cs_out: SPI2_CS_PAD_OUT
);

impl_spi_master!(
    Spi3,
    SPI3,
    spi3,
    3,
    clock: clock_spi3, axi: Spi3,
    ck_out: SPI3_CK_PAD_OUT,
    mosi_out: SPI3_D_PAD_OUT,
    miso_in: SPI3_Q_PAD_IN,
    cs_out: SPI3_CS_PAD_OUT
);

/// Generates a slave type from the shared register layout.
macro_rules! impl_spi_slave {
    (
        $Name:ident, $PAC:ident, $mod:ident, $NUM:literal,
        clock: $clock:ident,
        ck_in: $ck_in:ident, mosi_in: $mosi_in:ident, miso_out: $miso_out:ident, cs_in: $cs_in:ident
    ) => {
        /// SPI slave owning the PAC peripheral.
        pub struct $Name {
            spi: $PAC,
        }

        impl $Name {
            /// SPI instance number, 2 or 3.
            pub const NUM: u8 = $NUM;

            /// Takes ownership, enables the clocks and configures slave mode.
            ///
            /// Slave mode takes SCK from the master, so only the bus and
            /// register clocks are enabled here.
            pub fn init(spi: $PAC, config: &SlaveConfig, clkrst: &HP_SYS_CLKRST) -> Self {
                $clock(clkrst);

                let r = &spi;
                r.spi_clock.write_bits(0);
                r.spi_user.write_bits(0);
                r.spi_ctrl.write_bits(0);

                // Full duplex: both data lines are active for the whole
                // transaction, and the master decides how long it runs.
                r.spi_user.set_spi_doutdin();
                r.spi_slave.set_mode();
                r.spi_slave.trigger_spi_soft_reset();

                match config.bit_order {
                    BitOrder::MsbFirst => {
                        r.spi_ctrl.write_spi_rd_bit_order(0);
                        r.spi_ctrl.write_spi_wr_bit_order(0);
                    }
                    BitOrder::LsbFirst => {
                        r.spi_ctrl.write_spi_rd_bit_order(1);
                        r.spi_ctrl.write_spi_wr_bit_order(1);
                    }
                }
                Self::apply_mode(&spi, config.mode);

                // One transaction at a time: no segmented DMA transfer, and the
                // receive link may end before the bit counter is reached.
                r.spi_dma_conf.clr_spi_rx_eof_en();
                r.spi_dma_conf.clr_spi_dma_slv_seg_trans_en();
                r.spi_slave.clr_spi_rsck_data_out();

                Self { spi }
            }

            /// Obtains a handle without ownership checks.
            ///
            /// # Safety
            ///
            /// [`init`](Self::init) must have run, and the caller must not start
            /// a second transaction on the same instance.
            #[inline]
            pub unsafe fn steal() -> crate::Steal<Self> {
                crate::Steal(Self {
                    spi: unsafe { $PAC::steal() },
                })
            }

            /// The underlying register block.
            #[inline]
            pub fn regs(&self) -> &esp32p4_pac::$mod::RegisterBlock {
                &self.spi
            }

            /// Returns the PAC peripheral.
            #[inline]
            pub fn release(self) -> $PAC {
                self.spi
            }

            /// Switches the clock mode of a configured slave.
            ///
            /// Resets the shift logic, so do it between transactions.
            pub fn set_mode(&self, mode: Mode) {
                Self::apply_mode(&self.spi, mode);
                self.regs().spi_slave.trigger_spi_soft_reset();
            }

            /// Routes SCK, MOSI, MISO and CS from the master.
            ///
            /// The CS pad gets a pull-up so a floating select line reads as
            /// "not selected".
            pub fn set_pins<CK: PinOps, MOSI: PinOps, MISO: PinOps, CS: PinOps>(
                &self,
                mut ck: CK,
                mut mosi: MOSI,
                mut miso: MISO,
                mut cs: CS,
            ) {
                ck.route_input_signal(InputSignal::$ck_in);
                mosi.route_input_signal(InputSignal::$mosi_in);
                miso.route_output_signal(OutputSignal::$miso_out);
                cs.route_input_signal(InputSignal::$cs_in);
                cs.set_pull_up_enabled(true);
            }

            /// Arms one transaction: `tx` is clocked out while the master
            /// clocks, and whatever the master sends is collected by
            /// [`finish`](Self::finish).
            ///
            /// Bytes past the end of `tx` are sent as `0xFF`. A CPU-driven
            /// transaction carries at most [`MAX_TRANSFER`] bytes.
            ///
            /// # Panics
            ///
            /// Panics if `tx` is empty or longer than [`MAX_TRANSFER`].
            pub fn start(&self, tx: &[u8]) {
                assert!(!tx.is_empty(), "an SPI slave transaction needs data");
                assert!(
                    tx.len() <= MAX_TRANSFER,
                    "a CPU-driven slave transaction holds at most {MAX_TRANSFER} bytes"
                );

                let r = self.regs();
                // Both data lines are active for the whole transaction: the
                // master decides when it starts and how long it runs.
                r.spi_user.set_spi_usr_mosi();
                r.spi_user.set_spi_usr_miso();
                for (index, byte) in tx.iter().enumerate() {
                    self.write_byte(index, *byte);
                }
                // The buffer's FIFO pointers survive a transaction, so they have
                // to be rewound before each one (same as the master).
                r.spi_dma_conf.trigger_spi_buf_afifo_rst();
                r.spi_dma_conf.trigger_spi_rx_afifo_rst();
                r.spi_ms_dlen
                    .write_spi_ms_data_bitlen(tx.len() as u32 * 8 - 1);
                r.spi_dma_int_clr.trigger_spi_trans_done_int_clr();
                r.spi_cmd.set_spi_usr();
            }

            /// Waits for the armed transaction to finish and returns how many
            /// bytes the master clocked in.
            ///
            /// `rx` keeps its contents past the returned count; the length comes
            /// from the controller's bit counter, so it is the master's decision,
            /// not a slice length.
            ///
            /// # Panics
            ///
            /// Panics if no master starts a transaction before the spin bound
            /// expires.
            pub fn finish(&self, rx: &mut [u8]) -> usize {
                let r = self.regs();
                // The master may take a while to show up; give it an order of
                // magnitude more polling than a peripheral handshake needs.
                spin::spin_wait_max(16 * spin::SPIN_TIMEOUT, "SPI slave transaction", || {
                    r.spi_dma_int_raw.spi_trans_done_int_raw()
                });

                let bits = r.spi_slave1.spi_slv_data_bitlen();
                let received = (bits as usize / 8).min(rx.len());
                for (index, byte) in rx.iter_mut().enumerate().take(received) {
                    *byte = self.read_byte(index);
                }
                received
            }

            /// [`start`](Self::start) followed by [`finish`](Self::finish).
            ///
            /// Only useful when the master is already clocking, or when the
            /// caller drives the master from another context; otherwise arm the
            /// transaction with `start` first and collect it after the master
            /// ran.
            ///
            /// # Panics
            ///
            /// See [`start`](Self::start) and [`finish`](Self::finish).
            pub fn transfer(&self, tx: &[u8], rx: &mut [u8]) -> usize {
                self.start(tx);
                self.finish(rx)
            }

            /// Programs the clock mode.
            fn apply_mode(spi: &$PAC, mode: Mode) {
                match mode {
                    Mode::_0 => {
                        spi.spi_misc.clr_spi_ck_idle_edge();
                        spi.spi_user.clr_spi_rsck_i_edge();
                        spi.spi_user.clr_spi_tsck_i_edge();
                        spi.spi_slave.clr_spi_clk_mode_13();
                    }
                    Mode::_1 => {
                        spi.spi_misc.clr_spi_ck_idle_edge();
                        spi.spi_user.set_spi_rsck_i_edge();
                        spi.spi_user.set_spi_tsck_i_edge();
                        spi.spi_slave.set_spi_clk_mode_13();
                    }
                    Mode::_2 => {
                        spi.spi_misc.set_spi_ck_idle_edge();
                        spi.spi_user.set_spi_rsck_i_edge();
                        spi.spi_user.set_spi_tsck_i_edge();
                        spi.spi_slave.clr_spi_clk_mode_13();
                    }
                    Mode::_3 => {
                        spi.spi_misc.set_spi_ck_idle_edge();
                        spi.spi_user.clr_spi_rsck_i_edge();
                        spi.spi_user.clr_spi_tsck_i_edge();
                        spi.spi_slave.set_spi_clk_mode_13();
                    }
                }
            }

            /// Reads one byte out of the controller's data buffer.
            fn read_byte(&self, index: usize) -> u8 {
                (self.read_word(index / 4) >> (8 * (index % 4))) as u8
            }

            /// Writes one byte into the controller's data buffer.
            fn write_byte(&self, index: usize, byte: u8) {
                let shift = 8 * (index % 4);
                let word = self.read_word(index / 4);
                self.write_word(
                    index / 4,
                    (word & !(0xffu32 << shift)) | (u32::from(byte) << shift),
                );
            }

            fn read_word(&self, index: usize) -> u32 {
                // SAFETY: `SPI_W0`..`SPI_W15` are the 16 consecutive words of the
                // data buffer (TRM 46.5.6); the index stays inside it.
                unsafe { core::ptr::read_volatile(self.word_ptr(index)) }
            }

            fn write_word(&self, index: usize, value: u32) {
                // SAFETY: see `read_word`.
                unsafe { core::ptr::write_volatile(self.word_ptr(index) as *mut u32, value) }
            }

            fn word_ptr(&self, index: usize) -> *const u32 {
                debug_assert!(index < MAX_TRANSFER / 4);
                // SAFETY: the peripheral is mapped for the program's lifetime.
                unsafe { (core::ptr::addr_of!(self.regs().spi_w0) as *const u32).add(index) }
            }
        }
    };
}

impl_spi_slave!(
    SpiSlave2,
    SPI2,
    spi2,
    2,
    clock: clock_spi2,
    ck_in: SPI2_CK_PAD_IN,
    mosi_in: SPI2_D_PAD_IN,
    miso_out: SPI2_Q_PAD_OUT,
    cs_in: SPI2_CS_PAD_IN
);

impl_spi_slave!(
    SpiSlave3,
    SPI3,
    spi3,
    3,
    clock: clock_spi3,
    ck_in: SPI3_CK_PAD_IN,
    mosi_in: SPI3_D_PAD_IN,
    miso_out: SPI3_QO_PAD_OUT,
    cs_in: SPI3_CS_PAD_IN
);
