//! GDMA (General Direct Memory Access), the `AHB_DMA` and `AXI_DMA` controllers.
//!
//! GDMA moves data between peripherals and memory, or between two memory
//! regions, without involving the core. Software hands it a list of
//! [`Descriptor`]s, the engine walks that list, and interrupts report progress.
//! Each ESP32-P4 controller has three transmit/receive pairs, and every channel
//! can be wired to any peripheral that sits on its bus.
//!
//! # Features
//!
//! - Both controllers: GDMA-AHB ([`AhbDma`]) for I3C/UHCI/I2S/ADC/RMT and
//!   GDMA-AXI ([`AxiDma`]) for LCD/CAM, GP-SPI2/3, PARLIO, AES, SHA
//! - Three transmit and three receive channels per controller, assigned to
//!   peripherals by software ([`RxChannel::connect`], [`AxiRxChannel::connect`])
//! - Descriptor chains and rings built on caller-owned arrays ([`Chain`])
//! - One driver interface for both engines ([`ReceiveChannel`],
//!   [`TransmitChannel`]), so a peripheral driver does not care which
//!   controller its channels come from
//! - Memory-to-memory copies on AHB channel pairs 1 and 2 ([`AhbDma::copy`])
//! - Per-source interrupts, arbitration priority and burst settings
//! - Cache maintenance for buffers behind the cache: [`AhbDma::copy`] does it,
//!   a peripheral driver does it where it hands buffers over
//!
//! # Architecture
//!
//! ```text
//!   TX channel n ──► peripheral (I3C, UHCI, I2S0-2, ADC, RMT)
//!   RX channel n ◄── peripheral
//!
//!   TX channel n ──┐               memory-to-memory: `mem_trans_en`
//!                  ├──► RX channel n   loops the pair through a `Dummy-n` id
//!   descriptor list ──► engine ──► AHB bus ──► L2MEM / cached PSRAM
//! ```
//!
//! `TX` copies memory to a peripheral, `RX` copies a peripheral into memory. A
//! descriptor list lives in memory and each descriptor points at one buffer;
//! [`Chain`] slices a buffer into 4095-byte chunks and links the descriptors for
//! you.
//!
//! # Memory
//!
//! GDMA-AHB reaches one contiguous window, programmed by [`AhbDma::init`] to
//! `0x4000_0000..0x4FFC_0000` like the reference driver: flash and the cached
//! PSRAM window, then the whole of L2MEM. Both descriptors and buffers have to
//! live inside it — the direct aliases (`0x8FF0_0000` for SRAM, `0x8800_0000`
//! for PSRAM) are *not* reachable, the engine rejects those descriptors with
//! [`Error::DescriptorError`].
//!
//! Descriptors carry their own 8-byte alignment and have to stay put while
//! mounted. They are reached through their direct alias by the core, because
//! this chip caches internal SRAM and PSRAM at their normal addresses and a
//! descriptor written by the core is otherwise invisible to the engine —
//! verified on hardware.
//! [`Chain::mount`] handles that translation, so callers just put descriptors in
//! ordinary memory (a `static`, an array on the stack).
//!
//! Buffers are the caller's memory: their addresses go into the descriptor
//! unchanged. A buffer behind the cache needs maintenance around the transfer —
//! write back before the engine reads, invalidate after it writes — see
//! [`crate::cache`]; [`AhbDma::copy`] does that itself. Internal SRAM works this
//! way, both as a source and as a destination. Cached PSRAM buffers are reached
//! by the engine but the cache helpers in this HAL do not make them coherent
//! again for the core yet, so keep buffers in internal SRAM for now.
//!
//! Buffer addresses need no alignment unless a burst mode is enabled.
//!
//! # Scope
//!
//! The ESP32-P4 has two GDMA controllers and one unrelated DesignWare engine:
//!
//! | Pac peripheral | Base | Role |
//! |----------------|------|------|
//! | `AHB_DMA` | `0x5008_5000` | this module: I3C/UHCI/I2S/ADC/RMT, internal + cached external memory |
//! | `AXI_DMA` | `0x5008_a000` | this module: LCD/CAM, SPI2/3, PARLIO, AES, SHA |
//! | `DMA` | `0x5008_1000` | `DW_GDMA`, a plain memory-to-memory engine used by JPEG/camera; not implemented |
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::dma::{AhbDma, Chain, Descriptor, MemChannel};
//!
//! let dp = Peripherals::take().unwrap();
//! let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
//!
//! // Two descriptors per direction: one copy can move up to 2 * 4095 bytes.
//! let descriptors = [const { Descriptor::new() }; 4];
//!
//! let src = [0x5au8; 4096];
//! let mut dst = [0u8; 4096];
//! dma.copy(MemChannel::Ch1, &src, &mut dst, &descriptors).unwrap();
//! assert_eq!(dst[..], src[..]);
//! ```
//!
//! A peripheral driver builds its own chain and starts the channel:
//!
//! ```ignore
//! use esp32p4_hal::dma::{Chain, Peripheral, RxInterrupt};
//!
//! let rx = dma.rx_channel::<0>();
//! rx.connect(Peripheral::Adc);
//! let chain = Chain::new(&descriptors);
//! chain.mount_ring(buffer.as_mut_ptr(), buffer.len()).unwrap();
//! rx.enable_interrupt(RxInterrupt::SucEof, true);
//! rx.start(chain.head());
//! ```
//!
//! GDMA-AXI looks the same, only the peripheral enum differs:
//!
//! ```ignore
//! use esp32p4_hal::dma::{AxiDma, AxiPeripheral};
//!
//! let dma = AxiDma::init(dp.AXI_DMA, &dp.HP_SYS_CLKRST);
//! let tx = dma.tx_channel::<0>();
//! tx.connect(AxiPeripheral::Spi2);
//! tx.check_owner(true);
//! tx.auto_writeback(true);
//! ```

use esp32p4_pac::{AHB_DMA, HP_SYS_CLKRST, ahb_dma};

use crate::{cache, delay};

pub mod axi;
pub mod channel;
pub mod descriptor;

pub use axi::{AxiDma, AxiPeripheral, AxiRxChannel, AxiTxChannel};
pub use channel::{
    Dummy, ReceiveChannel, RxChannel, RxInterrupt, TransmitChannel, TxChannel, TxInterrupt,
};
pub use descriptor::{Chain, Descriptor, Owner};

/// A peripheral a channel can be connected to.
///
/// Implemented by [`Peripheral`] (the GDMA-AHB ids of TRM table 6.4-1) and
/// [`AxiPeripheral`] (the GDMA-AXI ids of table 6.4-2). The two controllers
/// number their peripherals differently, so a value from the wrong enum
/// selects the wrong peripheral — pass the one that matches the controller
/// the channel belongs to.
pub trait PeripheralId: Copy {
    /// Value written to `PERI_IN_SEL_CHn` / `PERI_OUT_SEL_CHn`.
    fn id(self) -> u32;
}

/// A peripheral whose handshake a GDMA-AHB channel can use.
///
/// Values are `PERI_IN_SEL_CHn` / `PERI_OUT_SEL_CHn` from TRM table 6.4-1. A
/// peripheral can feed one channel at a time; two channels selecting the same
/// value hand the same data to two engines, which is never what you want.
/// The GDMA-AXI ids are in [`AxiPeripheral`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Peripheral {
    /// I3C master or slave.
    I3c = 0,
    /// UHCI: the DMA front-end of UART0/UART1.
    Uhci = 2,
    /// I2S0.
    I2s0 = 3,
    /// I2S1.
    I2s1 = 4,
    /// I2S2.
    I2s2 = 5,
    /// ADC continuous-conversion controller.
    Adc = 8,
    /// RMT (remote control transceiver).
    Rmt = 10,
}

impl PeripheralId for Peripheral {
    #[inline]
    fn id(self) -> u32 {
        self as u32
    }
}

/// A channel pair that can copy memory to memory.
///
/// GDMA-AHB only implements the loop-back path on pairs 1 and 2 (TRM 6.4.3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MemChannel {
    /// Pair 1: TX channel 1 into RX channel 1.
    Ch1,
    /// Pair 2: TX channel 2 into RX channel 2.
    Ch2,
}

/// Errors a DMA transfer reports.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The buffer has no bytes; there is nothing to transfer.
    EmptyBuffer,
    /// The buffer needs more descriptors than the chain has.
    NotEnoughDescriptors {
        /// Descriptors the transfer needs.
        needed: usize,
        /// Descriptors the caller provided.
        available: usize,
    },
    /// The engine rejected a descriptor: the owner bit was clear, or the
    /// descriptor address was outside the access window.
    DescriptorError,
    /// The receive chain had no free descriptor left when data arrived.
    DescriptorEmpty,
    /// A channel FIFO overflowed or underflowed.
    Fifo,
    /// The peripheral flagged the data it produced as erroneous.
    Peripheral,
    /// The transfer did not finish before the driver's deadline.
    Timeout,
}

/// Window the engine checks descriptor addresses against.
///
/// Covers the cached aliases of flash (`0x4000_0000`) and PSRAM
/// (`0x4800_0000`) up to L2MEM's end, the same range the reference driver
/// programs. The uncached PSRAM alias (`0x8800_0000`) is *not* inside it, so
/// descriptors must not be placed there.
const ACCESS_WINDOW_START: u32 = 0x4000_0000;
const ACCESS_WINDOW_END: u32 = 0x4ffc_0000;

/// Fixed part of a memory-to-memory deadline, in microseconds.
const COPY_HEAD_US: u64 = 5_000;
/// Throughput assumed when sizing a memory-to-memory deadline, in bytes per
/// microsecond. Chosen well below what the engine reaches so that only a stuck
/// channel hits it.
const COPY_BYTES_PER_US: u64 = 4;

/// The GDMA-AHB controller.
///
/// Owns the PAC peripheral; the channels are zero-sized handles over its
/// register blocks.
pub struct AhbDma {
    ahb: AHB_DMA,
}

impl AhbDma {
    /// Takes ownership of the controller and brings it up.
    ///
    /// Enables the bus clock, pulses the module reset and programs the access
    /// window used for descriptor checks.
    pub fn init(ahb: AHB_DMA, clkrst: &HP_SYS_CLKRST) -> Self {
        // Bus clock and reset, mirroring the reference driver. Both default to
        // "clock on, out of reset"; set them explicitly so a driver that ran
        // before us cannot leave the controller parked.
        clkrst.soc_clk_ctrl1.set_reg_ahb_pdma_sys_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_ahb_pdma();
        clkrst.hp_rst_en1.clr_reg_rst_en_ahb_pdma();

        ahb.intr_mem_start_addr
            .write_access_intr_mem_start_addr(ACCESS_WINDOW_START);
        ahb.intr_mem_end_addr
            .write_access_intr_mem_end_addr(ACCESS_WINDOW_END);

        Self { ahb }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// [`init`](Self::init) must have run, and the returned handle must not be
    /// used to reconfigure the controller while the owning instance is in use.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            ahb: unsafe { AHB_DMA::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &ahb_dma::RegisterBlock {
        &self.ahb
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> AHB_DMA {
        self.ahb
    }

    /// Receive channel `N`.
    ///
    /// The handle is zero-sized; nothing stops two callers from asking for the
    /// same channel, so a driver has to keep track of which channels it owns.
    #[inline]
    pub fn rx_channel<const N: usize>(&self) -> RxChannel<N> {
        RxChannel::new()
    }

    /// Transmit channel `N`.
    ///
    /// See [`rx_channel`](Self::rx_channel).
    #[inline]
    pub fn tx_channel<const N: usize>(&self) -> TxChannel<N> {
        TxChannel::new()
    }

    /// Copies `src` into `dst` through `channel` and waits for it to finish.
    ///
    /// Transfers `min(src.len(), dst.len())` bytes and returns that count. The
    /// buffers must not overlap. It is not an optimal memcpy — without burst
    /// mode the engine reaches about 23 MB/s out of internal SRAM — but it is a
    /// complete round trip through both halves of a channel pair, which is what
    /// a peripheral driver exercises.
    ///
    /// `descriptors` is split into a transmit half and a receive half, so it
    /// needs `2 * ceil(len / 4095)` entries. The array's alignment is part of
    /// [`Descriptor`], so only its placement is the caller's business: it has to
    /// be in memory the engine can reach (see the [module docs](self#memory)).
    ///
    /// The destination is invalidated after the transfer, so the core reads
    /// what the engine stored; the source is written back before it. See the
    /// [module docs](self#memory) for what that means for external memory.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotEnoughDescriptors`] without touching the hardware if
    /// the array is too small, and one of the descriptor, FIFO or timeout
    /// variants if the transfer fails.
    pub fn copy(
        &self,
        channel: MemChannel,
        src: &[u8],
        dst: &mut [u8],
        descriptors: &[Descriptor],
    ) -> Result<usize, Error> {
        let len = src.len().min(dst.len());
        if len == 0 {
            return Ok(0);
        }
        let chunks = len.div_ceil(descriptor::MAX_LEN);
        if descriptors.len() < 2 * chunks {
            return Err(Error::NotEnoughDescriptors {
                needed: 2 * chunks,
                available: descriptors.len(),
            });
        }

        let (tx_descriptors, rx_descriptors) = descriptors.split_at(chunks);
        match channel {
            MemChannel::Ch1 => self.copy_ch1(src, dst, len, tx_descriptors, rx_descriptors),
            MemChannel::Ch2 => self.copy_ch2(src, dst, len, tx_descriptors, rx_descriptors),
        }
    }
}

/// Generates the memory-to-memory routine for one channel pair.
///
/// The bodies differ only in the channel index, and the index has to be a
/// literal for the register names to resolve, so each pair gets its own method.
macro_rules! impl_mem2mem {
    ($n:literal, $name:ident) => {
        impl AhbDma {
            fn $name(
                &self,
                src: &[u8],
                dst: &mut [u8],
                len: usize,
                tx_descriptors: &[Descriptor],
                rx_descriptors: &[Descriptor],
            ) -> Result<usize, Error> {
                let rx = self.rx_channel::<$n>();
                let tx = self.tx_channel::<$n>();

                // A channel keeps its configuration across transfers, so reset
                // it and program everything this routine depends on.
                rx.reset();
                tx.reset();
                rx.connect_memory(Dummy::D1);
                tx.connect_memory(Dummy::D1);
                rx.check_owner(true);
                tx.check_owner(true);
                rx.descriptor_burst(true);
                tx.descriptor_burst(true);
                tx.eof_till_data_popped(true);
                tx.auto_writeback(true);
                rx.clear_all_interrupts();
                tx.clear_all_interrupts();

                let tx_chain = Chain::new(tx_descriptors);
                let rx_chain = Chain::new(rx_descriptors);
                // The transmit side carries the EOF that makes the receive side
                // finish; without it the receive channel never reports SUC_EOF.
                tx_chain.mount_eof(src.as_ptr() as *mut u8, len)?;
                rx_chain.mount(dst.as_mut_ptr(), len)?;

                cache::writeback(src.as_ptr().cast(), len);
                cache::writeback_invalidate(dst.as_mut_ptr().cast(), len);

                rx.start(rx_chain.head());
                tx.start(tx_chain.head());

                let result = wait_for_copy(|| rx.interrupt_status(), || tx.interrupt_status(), len);
                if result.is_err() {
                    // Leave both halves idle so the next transfer starts clean.
                    rx.stop();
                    tx.stop();
                } else {
                    // The engine writes memory, not the cache: drop whatever the
                    // cache picked up while the transfer ran so the caller reads
                    // the data the engine stored.
                    cache::invalidate(dst.as_mut_ptr().cast(), len);
                }
                result.map(|()| len)
            }
        }
    };
}

impl_mem2mem!(1, copy_ch1);
impl_mem2mem!(2, copy_ch2);

/// Polls a memory-to-memory transfer to completion.
///
/// Takes the status readings as closures because the channel methods only exist
/// for concrete channel indices.
fn wait_for_copy(
    mut rx_status: impl FnMut() -> u32,
    mut tx_status: impl FnMut() -> u32,
    len: usize,
) -> Result<(), Error> {
    let deadline = delay::micros() + COPY_HEAD_US + len as u64 / COPY_BYTES_PER_US;
    loop {
        let rx_status = rx_status();
        let tx_status = tx_status();

        if rx_status & RxInterrupt::DscrErr.mask() != 0
            || tx_status & TxInterrupt::DscrErr.mask() != 0
        {
            return Err(Error::DescriptorError);
        }
        if rx_status & RxInterrupt::DscrEmpty.mask() != 0 {
            return Err(Error::DescriptorEmpty);
        }
        if rx_status & (RxInterrupt::FifoOverflow.mask() | RxInterrupt::FifoUnderflow.mask()) != 0
            || tx_status & (TxInterrupt::FifoOverflow.mask() | TxInterrupt::FifoUnderflow.mask())
                != 0
        {
            return Err(Error::Fifo);
        }
        if rx_status & RxInterrupt::ErrEof.mask() != 0 {
            return Err(Error::Peripheral);
        }
        if rx_status & RxInterrupt::SucEof.mask() != 0 {
            return Ok(());
        }
        if delay::micros() > deadline {
            return Err(Error::Timeout);
        }
        core::hint::spin_loop();
    }
}
