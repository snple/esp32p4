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

use core::cell::{Cell, UnsafeCell};
use core::ffi::c_void;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, fence};

use crate::cache;
use super::stats::Stats;

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

// ── TDES0 status bits the MAC writes when it releases the descriptor ────────

/// Error summary: set when any of the error bits below is set.
const TDES0_ES: u32 = 1 << 15;
/// The DMA ran out of frame data while the frame was going out.
const TDES0_UF: u32 = 1 << 1;
/// Sixteen collisions in a row; the frame was abandoned.
const TDES0_EC: u32 = 1 << 8;
/// A collision arrived after the 64-byte window.
const TDES0_LC: u32 = 1 << 9;
/// The PHY never asserted carrier.
const TDES0_NC: u32 = 1 << 10;
/// Carrier disappeared during the frame.
const TDES0_LOC: u32 = 1 << 11;
/// The DMA flushed the frame instead of sending it.
const TDES0_FF: u32 = 1 << 13;

// ── RDES0 bits ───────────────────────────────────────────────────────────────

const RDES0_OWN: u32 = 1 << 31;
const RDES0_FL_SHIFT: u32 = 16;
const RDES0_FL_MASK: u32 = 0x3fff;
const RDES0_ES: u32 = 1 << 15;
const RDES0_FS: u32 = 1 << 9;
const RDES0_LS: u32 = 1 << 8;
/// FCS mismatch on the received frame.
const RDES0_CE: u32 = 1 << 1;
/// Non-integer number of bytes (odd nibble) — a PHY/RMII timing smell.
const RDES0_DRIBBLE: u32 = 1 << 2;
/// `MII_RXER` was asserted while a frame was coming in.
const RDES0_RE: u32 = 1 << 3;
/// The receive watchdog cut the frame short.
const RDES0_RWT: u32 = 1 << 4;
/// The RX FIFO overflowed in the middle of the frame.
const RDES0_OE: u32 = 1 << 11;
/// The length in the frame header disagreed with the frame.
const RDES0_LE: u32 = 1 << 12;
/// The DMA did not own the next descriptor, so the frame is truncated.
const RDES0_DE: u32 = 1 << 14;

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
///
/// Descriptors are padded to a whole cache line each. They are 32 bytes long,
/// so two of them used to share a line — and cache maintenance works in whole
/// lines: writing one descriptor back also wrote the stale copy of its
/// neighbour over whatever the DMA had just put there. Skipped updates then lose
/// frames with no error recorded anywhere.
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
///
/// Padded to a cache line for the same reason as [`TDes`]: the DMA sets the
/// owner bit of the next descriptor while the CPU owns this one, and a
/// line-granular writeback of this one must not revert it.
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

}

impl Default for RDes {
    fn default() -> Self {
        Self::new()
    }
}

/// A single frame buffer.
///
/// Each buffer starts on a cache line of its own: the maintenance helpers round
/// their range outwards to whole lines, so two buffers sharing a line would mean
/// invalidating one frame's tail (or a neighbour's dirty bytes) every time the
/// other one is read back.
#[repr(C, align(64))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxError {
    /// Every transmit descriptor is still owned by the MAC.
    RingFull,
    /// The frame is longer than [`MAX_FRAME_SIZE`].
    FrameTooLarge,
}

/// Transmit ring.
///
/// All methods take `&self`: the ring index lives in a [`Cell`] and the frame
/// buffers are reached through a raw pointer, which is what lets a network
/// stack hold a receive and a transmit token at the same time. The safety
/// argument is that the driver is single-threaded, interrupts are masked, and
/// each descriptor is only ever handed out by one path at a time.
pub(super) struct TxRing<'a> {
    descs: &'a [TDes],
    bufs: *mut DmaBuffer,
    len: usize,
    index: Cell<usize>,
    counters: Cell<Stats>,
}

impl<'a> TxRing<'a> {
    fn new(descs: &'a mut [TDes], bufs: &'a mut [DmaBuffer]) -> Self {
        let ring = Self {
            descs,
            bufs: bufs.as_mut_ptr(),
            len: descs.len(),
            index: Cell::new(0),
            counters: Cell::new(Stats::default()),
        };
        ring.reset();
        ring
    }

    /// Link the descriptors into a ring and hand them all to the CPU.
    fn reset(&self) {
        for i in 0..self.len {
            let next = &self.descs[(i + 1) % self.len] as *const TDes as u32;
            let buffer = self.buffer_addr(i);

            let desc = &self.descs[i];
            desc.tdes0.set(0);
            desc.tdes1.set(0);
            desc.buf_addr.set(buffer);
            desc.next_desc.set(next);
            cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());
        }
        self.index.set(0);
    }

    /// Address of the first descriptor, for `DMATXBASEADDR`.
    pub(super) fn base_ptr(&self) -> *const TDes {
        self.descs.as_ptr()
    }

    /// Address of slot `i`'s frame buffer.
    #[inline]
    fn buffer_addr(&self, i: usize) -> u32 {
        // SAFETY: `i` is always a ring index below `len`.
        unsafe {
            let buffer: &DmaBuffer = &*self.bufs.add(i);
            buffer.0.as_ptr() as u32
        }
    }

    /// Slot `i`'s frame buffer, for writing.
    ///
    /// Returning `&mut` from `&self` is the whole point of the interior
    /// mutability here; see the type-level comment.
    #[inline]
    #[expect(
        clippy::mut_from_ref,
        reason = "the ring owns the buffers exclusively and hands out one slot at a time"
    )]
    fn buffer(&self, i: usize) -> &mut DmaBuffer {
        // SAFETY: the caller holds the only handle to the ring, and slot `i` is
        // only reachable while the CPU owns its descriptor.
        unsafe { &mut *self.bufs.add(i) }
    }

    /// Descriptor `i`, refreshed from memory.
    #[inline]
    fn desc(&self, i: usize) -> &TDes {
        let desc = &self.descs[i];
        cache::invalidate(desc as *const TDes as *const c_void, size_of::<TDes>());
        desc
    }

    /// Queue one frame for transmission.
    pub(super) fn transmit(&self, frame: &[u8]) -> Result<(), TxError> {
        self.transmit_with(frame.len(), |buffer| buffer.copy_from_slice(frame))
    }

    /// Fill the next free slot with `len` bytes produced by `f` and hand it to
    /// the MAC.
    ///
    /// `f` writes straight into the DMA buffer, so no copy is needed.
    pub(super) fn transmit_with<R>(
        &self,
        len: usize,
        f: impl FnOnce(&mut [u8]) -> R,
    ) -> Result<R, TxError> {
        if len > MAX_FRAME_SIZE {
            return Err(TxError::FrameTooLarge);
        }

        let idx = self.index.get();
        let desc = self.desc(idx);
        if desc.owned_by_dma() {
            return Err(TxError::RingFull);
        }
        // The status of the frame that used this slot last is still in TDES0.
        // Decode it before the write below throws it away.
        self.account_status(desc.tdes0.get());

        let mut counters = self.counters.get();
        counters.tx_frames = counters.tx_frames.wrapping_add(1);
        counters.tx_bytes = counters.tx_bytes.wrapping_add(len as u32);
        self.counters.set(counters);

        let result = f(&mut self.buffer(idx).0[..len]);
        cache::writeback(self.buffer_addr(idx) as *const c_void, len);

        let desc = &self.descs[idx];
        desc.tdes1.set(len as u32 & BUF1_SIZE_MASK);
        desc.tdes0.set(TDES0_TX_READY);
        cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());

        // Ownership is the last thing the MAC may observe.
        desc.tdes0.set(TDES0_TX_READY | TDES0_OWN);
        cache::writeback(desc as *const TDes as *const c_void, size_of::<TDes>());
        fence(Ordering::Release);

        self.index.set((idx + 1) % self.len);
        Ok(result)
    }

    /// Whether the next slot is free.
    pub(super) fn has_capacity(&self) -> bool {
        !self.desc(self.index.get()).owned_by_dma()
    }

    /// A snapshot of this ring's counters.
    pub(super) fn stats(&self) -> Stats {
        self.counters.get()
    }

    /// Fold the status word of a finished frame into the counters.
    fn account_status(&self, status: u32) {
        // A descriptor that has never been used reads back as zero, and the
        // control bits this driver sets are all above bit 15, so nothing has to
        // be remembered about which slots were used.
        let mut counters = self.counters.get();
        if status & TDES0_ES != 0 {
            counters.tx_errors = counters.tx_errors.wrapping_add(1);
        }
        if status & TDES0_UF != 0 {
            counters.tx_underflows = counters.tx_underflows.wrapping_add(1);
        }
        if status & TDES0_EC != 0 {
            counters.tx_excessive_collisions =
                counters.tx_excessive_collisions.wrapping_add(1);
        }
        if status & TDES0_LC != 0 {
            counters.tx_late_collisions = counters.tx_late_collisions.wrapping_add(1);
        }
        if status & (TDES0_NC | TDES0_LOC) != 0 {
            counters.tx_carrier_errors = counters.tx_carrier_errors.wrapping_add(1);
        }
        if status & TDES0_FF != 0 {
            counters.tx_flushed = counters.tx_flushed.wrapping_add(1);
        }
        self.counters.set(counters);
    }
}

/// Receive ring.
///
/// Same interior-mutability reasoning as [`TxRing`]: the stack reads a frame
/// and transmits a reply from the same `&self` borrow.
pub(super) struct RxRing<'a> {
    descs: &'a [RDes],
    bufs: *mut DmaBuffer,
    len: usize,
    index: Cell<usize>,
    counters: Cell<Stats>,
}

impl<'a> RxRing<'a> {
    fn new(descs: &'a mut [RDes], bufs: &'a mut [DmaBuffer]) -> Self {
        let ring = Self {
            descs,
            bufs: bufs.as_mut_ptr(),
            len: descs.len(),
            index: Cell::new(0),
            counters: Cell::new(Stats::default()),
        };
        ring.reset();
        ring
    }

    /// Link the descriptors and hand them to the MAC.
    fn reset(&self) {
        for i in 0..self.len {
            let next = &self.descs[(i + 1) % self.len] as *const RDes as u32;
            // SAFETY: `i` is always a ring index below `len`.
            let buffer = unsafe {
                let buffer: &DmaBuffer = &*self.bufs.add(i);
                buffer.0.as_ptr() as u32
            };

            let desc = &self.descs[i];
            desc.rdes0.set(RDES0_OWN);
            desc.rdes1
                .set(MAX_FRAME_SIZE as u32 & BUF1_SIZE_MASK | RDES1_RCH);
            desc.buf_addr.set(buffer);
            desc.next_desc.set(next);
            cache::writeback(desc as *const RDes as *const c_void, size_of::<RDes>());
        }
        self.index.set(0);
        fence(Ordering::Release);
    }

    /// Address of the first descriptor, for `DMARXBASEADDR`.
    pub(super) fn base_ptr(&self) -> *const RDes {
        self.descs.as_ptr()
    }

    /// Descriptor `i`, refreshed from memory.
    #[inline]
    fn desc(&self, i: usize) -> &RDes {
        let desc = &self.descs[i];
        cache::invalidate(desc as *const RDes as *const c_void, size_of::<RDes>());
        desc
    }

    /// Return the next received frame, if one is ready.
    ///
    /// The slice stays valid until [`RxRing::release`]. Slots holding a broken
    /// or runt frame are recycled on the way.
    pub(super) fn receive(&self) -> Option<&[u8]> {
        loop {
            let idx = self.index.get();
            let desc = self.desc(idx);

            if desc.owned_by_dma() {
                return None;
            }

            let status = desc.rdes0.get();
            let complete = status & RDES0_ES == 0 && status & (RDES0_FS | RDES0_LS) != 0;
            // The MAC appends a 4-byte FCS that never reaches the stack.
            let len = ((status >> RDES0_FL_SHIFT) & RDES0_FL_MASK) as usize;
            let len = len.saturating_sub(4);

            if !complete || !(MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&len) {
                self.account_error(status, len);
                self.hand_back(idx);
                continue;
            }

            let mut counters = self.counters.get();
            counters.rx_frames = counters.rx_frames.wrapping_add(1);
            counters.rx_bytes = counters.rx_bytes.wrapping_add(len as u32);
            self.counters.set(counters);

            // SAFETY: `idx` is a ring index and the CPU owns the descriptor.
            let buffer = unsafe {
                let buffer: &DmaBuffer = &*self.bufs.add(idx);
                &buffer.0[..len]
            };
            cache::invalidate(buffer.as_ptr().cast(), len);
            return Some(buffer);
        }
    }

    /// A snapshot of this ring's counters.
    pub(super) fn stats(&self) -> Stats {
        self.counters.get()
    }

    /// How many descriptors currently hold a frame the stack has not read.
    pub(super) fn pending(&self) -> usize {
        (0..self.len)
            .filter(|&i| !self.desc(i).owned_by_dma())
            .count()
    }

    /// Fold the status word of a rejected frame into the counters.
    ///
    /// This is the only place a bad frame is visible: the driver drops it
    /// silently otherwise, and a CRC or dribble error here means the wire, the
    /// PHY or the RMII timing is in trouble rather than the network stack.
    fn account_error(&self, status: u32, len: usize) {
        let mut counters = self.counters.get();
        if status & RDES0_ES != 0 {
            counters.rx_errors = counters.rx_errors.wrapping_add(1);
        }
        if status & RDES0_CE != 0 {
            counters.rx_crc_errors = counters.rx_crc_errors.wrapping_add(1);
        }
        if status & RDES0_DRIBBLE != 0 {
            counters.rx_alignment_errors = counters.rx_alignment_errors.wrapping_add(1);
        }
        if status & RDES0_RE != 0 {
            counters.rx_mii_errors = counters.rx_mii_errors.wrapping_add(1);
        }
        if status & RDES0_RWT != 0 {
            counters.rx_watchdog_errors = counters.rx_watchdog_errors.wrapping_add(1);
        }
        if status & RDES0_OE != 0 {
            counters.rx_overrun_errors = counters.rx_overrun_errors.wrapping_add(1);
        }
        if status & RDES0_LE != 0 {
            counters.rx_length_errors = counters.rx_length_errors.wrapping_add(1);
        }
        if status & RDES0_DE != 0 {
            counters.rx_descriptor_errors = counters.rx_descriptor_errors.wrapping_add(1);
        }
        if !(MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&len) {
            counters.rx_bad_frames = counters.rx_bad_frames.wrapping_add(1);
        }
        self.counters.set(counters);
    }

    /// Give the current slot back to the MAC.
    pub(super) fn release(&self) {
        self.hand_back(self.index.get());
    }

    /// Mark slot `idx` as owned by the MAC and advance.
    fn hand_back(&self, idx: usize) {
        let desc = &self.descs[idx];
        desc.rdes0.set(RDES0_OWN);
        cache::writeback(desc as *const RDes as *const c_void, size_of::<RDes>());
        fence(Ordering::Release);
        self.index.set((idx + 1) % self.len);
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
