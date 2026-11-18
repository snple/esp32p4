//! EMAC DMA descriptor rings.
//!
//! The ESP32-P4 Ethernet MAC is a DesignWare 10/100 GMAC. Its DMA engine walks a
//! ring of descriptors in memory; each descriptor points at one frame buffer and
//! at the next descriptor. The driver always uses the **enhanced 32-byte
//! descriptor format** (`DMABUSMODE.ALT_DESC_SIZE = 1`), the layout the databook
//! documents for "second address chained" mode:
//!
//! ```text
//!   word 0  TDES0/RDES0   status + ownership + length of a received frame
//!   word 1  TDES1/RDES1   buffer 1 size and the chain bit
//!   word 2                buffer 1 address
//!   word 3                next descriptor address
//!   words 4..7            reserved (timestamp / checksum-offload results)
//! ```
//!
//! Ownership flips between CPU and DMA through bit 31 of word 0. The CPU may
//! only touch a descriptor while it owns it, and has to bounce the cache before
//! and after — see [`crate::cache`].

use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, fence};

use crate::cache;

/// Largest frame the MAC forwards: 1518 bytes plus the 4-byte FCS.
pub const MAX_FRAME_SIZE: usize = 1524;

/// Smallest frame worth handing to the stack (Ethernet header only).
pub const MIN_FRAME_SIZE: usize = 14;

// ── TDES0 bits ───────────────────────────────────────────────────────────────

const TDES0_OWN: u32 = 1 << 31;
/// Interrupt on completion.
const TDES0_IC: u32 = 1 << 30;
/// Last segment of the frame.
const TDES0_LS: u32 = 1 << 29;
/// First segment of the frame.
const TDES0_FS: u32 = 1 << 28;
/// Second address chained: the next descriptor lives at `next_desc`.
const TDES0_TCH: u32 = 1 << 20;

/// Status a transmit descriptor leaves with while the MAC owns it.
const TDES0_TX_READY: u32 = TDES0_FS | TDES0_LS | TDES0_IC | TDES0_TCH;

// ── RDES0 bits ───────────────────────────────────────────────────────────────

const RDES0_OWN: u32 = 1 << 31;
const RDES0_FL_SHIFT: u32 = 16;
const RDES0_FL_MASK: u32 = 0x3fff;
const RDES0_ES: u32 = 1 << 15;
const RDES0_FS: u32 = 1 << 9;
const RDES0_LS: u32 = 1 << 8;

/// Buffer size field shared by TDES1 and RDES1.
const BUF1_SIZE_MASK: u32 = 0x1fff;
/// Second address chained (RDES1 bit 14).
const RDES1_RCH: u32 = 1 << 14;

/// Read/write descriptor word.
#[repr(transparent)]
struct VolatileU32(UnsafeCell<u32>);

impl VolatileU32 {
    const fn new(value: u32) -> Self {
        Self(UnsafeCell::new(value))
    }

    #[inline]
    fn get(&self) -> u32 {
        // SAFETY: the descriptor lives in memory the DMA engine also touches, so
        // every access must stay a real memory access.
        unsafe { read_volatile(self.0.get()) }
    }

    #[inline]
    fn set(&self, value: u32) {
        // SAFETY: see `get`.
        unsafe { write_volatile(self.0.get(), value) }
    }
}

/// One transmit descriptor (enhanced 32-byte format).
///
/// [`Default`] is only there for clippy; build these with [`TDes::new`] in a
/// `const` context instead.
#[repr(C, align(8))]
pub struct TDes {
    tdes0: VolatileU32,
    tdes1: VolatileU32,
    buf_addr: VolatileU32,
    next_desc: VolatileU32,
    /// Words 4-7: reserved, the MAC may write a transmit timestamp here.
    _reserved: [VolatileU32; 4],
}

impl TDes {
    /// A zeroed descriptor, usable in `static` initialisers.
    pub const fn new() -> Self {
        Self {
            tdes0: VolatileU32::new(0),
            tdes1: VolatileU32::new(0),
            buf_addr: VolatileU32::new(0),
            next_desc: VolatileU32::new(0),
            _reserved: [
                VolatileU32::new(0),
                VolatileU32::new(0),
                VolatileU32::new(0),
                VolatileU32::new(0),
            ],
        }
    }

    #[inline]
    fn owned_by_dma(&self) -> bool {
        self.tdes0.get() & TDES0_OWN != 0
    }
}

impl Default for TDes {
    fn default() -> Self {
        Self::new()
    }
}

/// One receive descriptor (enhanced 32-byte format).
#[repr(C, align(8))]
pub struct RDes {
    rdes0: VolatileU32,
    rdes1: VolatileU32,
    buf_addr: VolatileU32,
    next_desc: VolatileU32,
    /// Words 4-7: extended status. RDES4 carries checksum-offload results, which
    /// this driver does not enable.
    _reserved: [VolatileU32; 4],
}

impl RDes {
    /// A zeroed descriptor, usable in `static` initialisers.
    pub const fn new() -> Self {
        Self {
            rdes0: VolatileU32::new(0),
            rdes1: VolatileU32::new(0),
            buf_addr: VolatileU32::new(0),
            next_desc: VolatileU32::new(0),
            _reserved: [
                VolatileU32::new(0),
                VolatileU32::new(0),
                VolatileU32::new(0),
                VolatileU32::new(0),
            ],
        }
    }

    #[inline]
    fn owned_by_dma(&self) -> bool {
        self.rdes0.get() & RDES0_OWN != 0
    }

    /// Whether the descriptor closes a complete frame and reports no error.
    #[inline]
    fn is_valid_frame(&self) -> bool {
        let status = self.rdes0.get();
        status & RDES0_ES == 0 && status & RDES0_FS != 0 && status & RDES0_LS != 0
    }

    /// Frame length reported by the MAC, FCS included.
    #[inline]
    fn frame_len(&self) -> usize {
        ((self.rdes0.get() >> RDES0_FL_SHIFT) & RDES0_FL_MASK) as usize
    }
}

impl Default for RDes {
    fn default() -> Self {
        Self::new()
    }
}

/// A single frame buffer, aligned for the 8-byte descriptor format.
#[repr(C, align(8))]
pub struct DmaBuffer([u8; MAX_FRAME_SIZE]);

impl DmaBuffer {
    /// A zeroed buffer, usable in `static` initialisers.
    pub const fn new() -> Self {
        Self([0; MAX_FRAME_SIZE])
    }
}

impl Default for DmaBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// All DMA memory the driver needs.
///
/// Place one in a `static` and lend it to [`Emac::init`](super::Emac::init). The
/// descriptor rings and buffers must stay put for as long as the driver runs,
/// because the MAC walks them by physical address.
///
/// `RX` bounds how many frames can be buffered before the stack drains them,
/// `TX` how many frames may be in flight before [`Emac::transmit`] reports
/// [`TxError::RingFull`](super::TxError::RingFull).
pub struct EmacBuffers<const RX: usize, const TX: usize> {
    tx_descs: [TDes; TX],
    rx_descs: [RDes; RX],
    tx_bufs: [DmaBuffer; TX],
    rx_bufs: [DmaBuffer; RX],
}

impl<const RX: usize, const TX: usize> EmacBuffers<RX, TX> {
    /// A zeroed set of rings, usable in `static` initialisers.
    pub const fn new() -> Self {
        const {
            assert!(RX > 0 && TX > 0, "an EMAC ring needs at least one slot");
        }
        Self {
            tx_descs: [const { TDes::new() }; TX],
            rx_descs: [const { RDes::new() }; RX],
            tx_bufs: [const { DmaBuffer::new() }; TX],
            rx_bufs: [const { DmaBuffer::new() }; RX],
        }
    }
}

impl<const RX: usize, const TX: usize> Default for EmacBuffers<RX, TX> {
    fn default() -> Self {
        Self::new()
    }
}

/// Why a frame was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TxError {
    /// Every transmit descriptor is still owned by the MAC.
    RingFull,
    /// The frame is longer than [`MAX_FRAME_SIZE`].
    FrameTooLarge,
}

/// Transmit ring.
pub(super) struct TxRing<'a> {
    descs: &'a mut [TDes],
    bufs: &'a mut [DmaBuffer],
    index: usize,
}

impl<'a> TxRing<'a> {
    fn new(descs: &'a mut [TDes], bufs: &'a mut [DmaBuffer]) -> Self {
        let mut ring = Self {
            descs,
            bufs,
            index: 0,
        };
        ring.reset();
        ring
    }

    /// Link the descriptors into a ring and hand them all to the CPU.
    fn reset(&mut self) {
        let n = self.descs.len();
        for i in 0..n {
            let next = &self.descs[(i + 1) % n] as *const TDes as u32;
            let buffer = self.bufs[i].0.as_mut_ptr() as u32;

            let desc = &mut self.descs[i];
            desc.tdes0.set(0);
            desc.tdes1.set(0);
            desc.buf_addr.set(buffer);
            desc.next_desc.set(next);
            cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());
        }
        self.index = 0;
    }

    /// Address of the first descriptor, for `DMATXBASEADDR`.
    pub(super) fn base_ptr(&self) -> *const TDes {
        self.descs.as_ptr()
    }

    /// Queue one frame for transmission.
    pub(super) fn transmit(&mut self, frame: &[u8]) -> Result<(), TxError> {
        if frame.len() > MAX_FRAME_SIZE {
            return Err(TxError::FrameTooLarge);
        }

        let idx = self.index;
        let desc = &self.descs[idx];
        cache::invalidate(desc as *const TDes as *const c_void, size_of::<TDes>());
        if desc.owned_by_dma() {
            return Err(TxError::RingFull);
        }

        let buffer = &mut self.bufs[idx].0;
        buffer[..frame.len()].copy_from_slice(frame);
        cache::writeback(buffer.as_ptr().cast(), frame.len());

        desc.tdes1.set(frame.len() as u32 & BUF1_SIZE_MASK);
        desc.tdes0.set(TDES0_TX_READY);
        cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());

        // Ownership is the last thing the MAC may observe.
        desc.tdes0.set(TDES0_TX_READY | TDES0_OWN);
        cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());
        fence(Ordering::Release);

        self.index = (idx + 1) % self.descs.len();
        Ok(())
    }

    /// Whether the next slot is free.
    pub(super) fn has_capacity(&self) -> bool {
        let desc = &self.descs[self.index];
        cache::invalidate(desc as *const TDes as *const c_void, size_of::<TDes>());
        !desc.owned_by_dma()
    }
}

/// Receive ring.
pub(super) struct RxRing<'a> {
    descs: &'a mut [RDes],
    bufs: &'a mut [DmaBuffer],
    index: usize,
}

impl<'a> RxRing<'a> {
    fn new(descs: &'a mut [RDes], bufs: &'a mut [DmaBuffer]) -> Self {
        let mut ring = Self {
            descs,
            bufs,
            index: 0,
        };
        ring.reset();
        ring
    }

    /// Link the descriptors and hand them to the MAC.
    fn reset(&mut self) {
        let n = self.descs.len();
        for i in 0..n {
            let next = &self.descs[(i + 1) % n] as *const RDes as u32;
            let buffer = self.bufs[i].0.as_mut_ptr() as u32;

            let desc = &mut self.descs[i];
            desc.rdes0.set(RDES0_OWN);
            desc.rdes1
                .set(MAX_FRAME_SIZE as u32 & BUF1_SIZE_MASK | RDES1_RCH);
            desc.buf_addr.set(buffer);
            desc.next_desc.set(next);
            cache::writeback(desc as *const RDes as *const c_void, size_of::<RDes>());
        }
        self.index = 0;
        fence(Ordering::Release);
    }

    /// Address of the first descriptor, for `DMARXBASEADDR`.
    pub(super) fn base_ptr(&self) -> *const RDes {
        self.descs.as_ptr()
    }

    /// Return the next received frame, if one is ready.
    ///
    /// The returned slice stays valid until [`RxRing::release`]. Slots holding a
    /// broken or runt frame are recycled on the way.
    pub(super) fn receive(&mut self) -> Option<&[u8]> {
        loop {
            let idx = self.index;
            let desc = &self.descs[idx];
            cache::invalidate(desc as *const RDes as *const c_void, size_of::<RDes>());

            if desc.owned_by_dma() {
                return None;
            }

            let valid = desc.is_valid_frame();
            // The MAC appends a 4-byte FCS that never reaches the stack.
            let len = desc.frame_len().saturating_sub(4);

            if !valid || !(MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&len) {
                self.hand_back(idx);
                continue;
            }

            cache::invalidate(self.bufs[idx].0.as_ptr().cast(), len);
            return Some(&self.bufs[idx].0[..len]);
        }
    }

    /// Give the current slot back to the MAC.
    pub(super) fn release(&mut self) {
        self.hand_back(self.index);
    }

    /// Mark slot `idx` as owned by the MAC and advance.
    fn hand_back(&mut self, idx: usize) {
        let desc = &mut self.descs[idx];
        desc.rdes0.set(RDES0_OWN);
        cache::writeback(desc as *const RDes as *const c_void, size_of::<RDes>());
        fence(Ordering::Release);
        self.index = (idx + 1) % self.descs.len();
    }
}

/// Split the storage block into the two rings.
pub(super) fn rings<const RX: usize, const TX: usize>(
    buffers: &mut EmacBuffers<RX, TX>,
) -> (TxRing<'_>, RxRing<'_>) {
    let EmacBuffers {
        tx_descs,
        rx_descs,
        tx_bufs,
        rx_bufs,
    } = buffers;

    (
        TxRing::new(tx_descs, tx_bufs),
        RxRing::new(rx_descs, rx_bufs),
    )
}
