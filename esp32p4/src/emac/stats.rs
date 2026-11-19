//! Traffic and error counters for the MAC.
//!
//! The P4's EMAC has no Synopsys MMC counter block, so [`Stats`] mixes three
//! sources that are all cheap to keep:
//!
//! * software counts of the frames and bytes the rings move — they walk every
//!   descriptor anyway,
//! * the status words the hardware writes back into the descriptors, which the
//!   driver would otherwise overwrite unseen; this is where CRC errors,
//!   overruns and transmit underflows show up,
//! * the DMA's latched error flags, accumulated by the interrupt handler because
//!   `DMASTATUS` is write-one-to-clear, and its missed-frame counter.
//!
//! Sample them with [`Emac::stats`](super::Emac::stats) and use
//! [`Stats::delta_since`] to log rates instead of ever-growing totals.

use core::sync::atomic::{AtomicU32, Ordering};

/// Frame, byte and error counters.
///
/// Every field is monotonic for as long as the driver lives (the underlying
/// hardware counters wrap, the software totals do not).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stats {
    /// Frames handed to the stack.
    pub rx_frames: u32,
    /// Bytes handed to the stack; the FCS is already stripped.
    pub rx_bytes: u32,
    /// Frames queued for transmission.
    pub tx_frames: u32,
    /// Bytes queued for transmission.
    pub tx_bytes: u32,

    /// Descriptors recycled because the frame carried an error (see the fields
    /// below for the breakdown).
    pub rx_errors: u32,
    /// FCS (CRC) mismatch reported by the MAC.
    pub rx_crc_errors: u32,
    /// Frames whose length was not a whole number of halfwords.
    pub rx_alignment_errors: u32,
    /// `MII_RXER` asserted during the frame, i.e. the PHY saw a coding error.
    pub rx_mii_errors: u32,
    /// The MAC's receive watchdog cut the frame short.
    pub rx_watchdog_errors: u32,
    /// The RX FIFO overflowed in the middle of the frame, corrupting it.
    pub rx_overrun_errors: u32,
    /// The length in the frame header disagreed with the frame.
    pub rx_length_errors: u32,
    /// The DMA did not own the next descriptor, so the frame was truncated.
    pub rx_descriptor_errors: u32,
    /// Descriptors recycled because the length made no sense (runt, oversized,
    /// or a frame without both the first and the last descriptor bit).
    pub rx_bad_frames: u32,

    /// Frames the MAC could not store at all, usually because every descriptor
    /// was still owned by the stack.
    pub rx_missed: u32,
    /// Frames dropped by an RX FIFO overflow.
    pub rx_fifo_overflows: u32,

    /// Transmit descriptors whose status carried the error summary.
    pub tx_errors: u32,
    /// The DMA ran out of frame data mid-transmission.
    pub tx_underflows: u32,
    /// Half-duplex only: a collision arrived after the 64-byte window.
    pub tx_late_collisions: u32,
    /// Half-duplex only: sixteen collisions in a row, frame abandoned.
    pub tx_excessive_collisions: u32,
    /// The PHY did not report carrier while the frame was going out.
    pub tx_carrier_errors: u32,
    /// The frame was flushed before it went out.
    pub tx_flushed: u32,

    /// `DMASTATUS.RECV_OVFLOW` — the RX FIFO overflowed during a burst.
    pub dma_rx_overflow: u32,
    /// `DMASTATUS.RECV_BUF_UNAVAIL` — no descriptor was free when a frame came in.
    pub dma_rx_unavailable: u32,
    /// `DMASTATUS.RECV_WDT_TO` — the receive watchdog fired.
    pub dma_rx_watchdog: u32,
    /// `DMASTATUS.RECV_PROC_STOP` — the receive DMA stopped by itself.
    pub dma_rx_stopped: u32,
    /// `DMASTATUS.TRANS_UNDFLOW`.
    pub dma_tx_underflow: u32,
    /// `DMASTATUS.TRANS_BUF_UNAVAIL`.
    pub dma_tx_unavailable: u32,
    /// `DMASTATUS.TRANS_JABBER_TO` — the transmit jabber timer fired.
    pub dma_tx_jabber: u32,
    /// `DMASTATUS.FATAL_BUS_ERR_INT` — the DMA could not reach memory.
    pub dma_bus_errors: u32,
}

impl Stats {
    /// What happened between `earlier` and `self`, field by field.
    pub fn delta_since(&self, earlier: &Stats) -> Stats {
        Stats {
            rx_frames: self.rx_frames.wrapping_sub(earlier.rx_frames),
            rx_bytes: self.rx_bytes.wrapping_sub(earlier.rx_bytes),
            tx_frames: self.tx_frames.wrapping_sub(earlier.tx_frames),
            tx_bytes: self.tx_bytes.wrapping_sub(earlier.tx_bytes),
            rx_errors: self.rx_errors.wrapping_sub(earlier.rx_errors),
            rx_crc_errors: self.rx_crc_errors.wrapping_sub(earlier.rx_crc_errors),
            rx_alignment_errors: self
                .rx_alignment_errors
                .wrapping_sub(earlier.rx_alignment_errors),
            rx_mii_errors: self.rx_mii_errors.wrapping_sub(earlier.rx_mii_errors),
            rx_watchdog_errors: self
                .rx_watchdog_errors
                .wrapping_sub(earlier.rx_watchdog_errors),
            rx_overrun_errors: self
                .rx_overrun_errors
                .wrapping_sub(earlier.rx_overrun_errors),
            rx_length_errors: self.rx_length_errors.wrapping_sub(earlier.rx_length_errors),
            rx_descriptor_errors: self
                .rx_descriptor_errors
                .wrapping_sub(earlier.rx_descriptor_errors),
            rx_bad_frames: self.rx_bad_frames.wrapping_sub(earlier.rx_bad_frames),
            rx_missed: self.rx_missed.wrapping_sub(earlier.rx_missed),
            rx_fifo_overflows: self
                .rx_fifo_overflows
                .wrapping_sub(earlier.rx_fifo_overflows),
            tx_errors: self.tx_errors.wrapping_sub(earlier.tx_errors),
            tx_underflows: self.tx_underflows.wrapping_sub(earlier.tx_underflows),
            tx_late_collisions: self
                .tx_late_collisions
                .wrapping_sub(earlier.tx_late_collisions),
            tx_excessive_collisions: self
                .tx_excessive_collisions
                .wrapping_sub(earlier.tx_excessive_collisions),
            tx_carrier_errors: self
                .tx_carrier_errors
                .wrapping_sub(earlier.tx_carrier_errors),
            tx_flushed: self.tx_flushed.wrapping_sub(earlier.tx_flushed),
            dma_rx_overflow: self.dma_rx_overflow.wrapping_sub(earlier.dma_rx_overflow),
            dma_rx_unavailable: self
                .dma_rx_unavailable
                .wrapping_sub(earlier.dma_rx_unavailable),
            dma_rx_watchdog: self.dma_rx_watchdog.wrapping_sub(earlier.dma_rx_watchdog),
            dma_rx_stopped: self.dma_rx_stopped.wrapping_sub(earlier.dma_rx_stopped),
            dma_tx_underflow: self.dma_tx_underflow.wrapping_sub(earlier.dma_tx_underflow),
            dma_tx_unavailable: self
                .dma_tx_unavailable
                .wrapping_sub(earlier.dma_tx_unavailable),
            dma_tx_jabber: self.dma_tx_jabber.wrapping_sub(earlier.dma_tx_jabber),
            dma_bus_errors: self.dma_bus_errors.wrapping_sub(earlier.dma_bus_errors),
        }
    }

    /// Fold another set of counters into this one.
    pub(super) fn merge(&mut self, other: &Stats) {
        self.rx_frames = self.rx_frames.wrapping_add(other.rx_frames);
        self.rx_bytes = self.rx_bytes.wrapping_add(other.rx_bytes);
        self.tx_frames = self.tx_frames.wrapping_add(other.tx_frames);
        self.tx_bytes = self.tx_bytes.wrapping_add(other.tx_bytes);
        self.rx_errors = self.rx_errors.wrapping_add(other.rx_errors);
        self.rx_crc_errors = self.rx_crc_errors.wrapping_add(other.rx_crc_errors);
        self.rx_alignment_errors = self
            .rx_alignment_errors
            .wrapping_add(other.rx_alignment_errors);
        self.rx_mii_errors = self.rx_mii_errors.wrapping_add(other.rx_mii_errors);
        self.rx_watchdog_errors = self
            .rx_watchdog_errors
            .wrapping_add(other.rx_watchdog_errors);
        self.rx_overrun_errors = self
            .rx_overrun_errors
            .wrapping_add(other.rx_overrun_errors);
        self.rx_length_errors = self.rx_length_errors.wrapping_add(other.rx_length_errors);
        self.rx_descriptor_errors = self
            .rx_descriptor_errors
            .wrapping_add(other.rx_descriptor_errors);
        self.rx_bad_frames = self.rx_bad_frames.wrapping_add(other.rx_bad_frames);
        self.rx_missed = self.rx_missed.wrapping_add(other.rx_missed);
        self.rx_fifo_overflows = self
            .rx_fifo_overflows
            .wrapping_add(other.rx_fifo_overflows);
        self.tx_errors = self.tx_errors.wrapping_add(other.tx_errors);
        self.tx_underflows = self.tx_underflows.wrapping_add(other.tx_underflows);
        self.tx_late_collisions = self
            .tx_late_collisions
            .wrapping_add(other.tx_late_collisions);
        self.tx_excessive_collisions = self
            .tx_excessive_collisions
            .wrapping_add(other.tx_excessive_collisions);
        self.tx_carrier_errors = self
            .tx_carrier_errors
            .wrapping_add(other.tx_carrier_errors);
        self.tx_flushed = self.tx_flushed.wrapping_add(other.tx_flushed);
        self.dma_rx_overflow = self.dma_rx_overflow.wrapping_add(other.dma_rx_overflow);
        self.dma_rx_unavailable = self
            .dma_rx_unavailable
            .wrapping_add(other.dma_rx_unavailable);
        self.dma_rx_watchdog = self.dma_rx_watchdog.wrapping_add(other.dma_rx_watchdog);
        self.dma_rx_stopped = self.dma_rx_stopped.wrapping_add(other.dma_rx_stopped);
        self.dma_tx_underflow = self.dma_tx_underflow.wrapping_add(other.dma_tx_underflow);
        self.dma_tx_unavailable = self
            .dma_tx_unavailable
            .wrapping_add(other.dma_tx_unavailable);
        self.dma_tx_jabber = self.dma_tx_jabber.wrapping_add(other.dma_tx_jabber);
        self.dma_bus_errors = self.dma_bus_errors.wrapping_add(other.dma_bus_errors);
    }
}

// ── DMASTATUS bits the handler latches ──────────────────────────────────────

/// `DMASTATUS` bits worth counting, as they appear in the register.
pub(super) const STATUS_TX_UNDERFLOW: u32 = 1 << 5;
pub(super) const STATUS_TX_UNAVAILABLE: u32 = 1 << 2;
pub(super) const STATUS_TX_JABBER: u32 = 1 << 3;
pub(super) const STATUS_RX_OVERFLOW: u32 = 1 << 4;
pub(super) const STATUS_RX_UNAVAILABLE: u32 = 1 << 7;
pub(super) const STATUS_RX_STOPPED: u32 = 1 << 8;
pub(super) const STATUS_RX_WATCHDOG: u32 = 1 << 9;
pub(super) const STATUS_BUS_ERROR: u32 = 1 << 13;

/// The `DMASTATUS` error flags, accumulated because the register itself is
/// write-one-to-clear and the handler has to clear it on every interrupt.
///
/// Only the handler writes these, so a plain count of occurrences is enough.
pub(super) struct DmaErrors {
    rx_overflow: AtomicU32,
    rx_unavailable: AtomicU32,
    rx_watchdog: AtomicU32,
    rx_stopped: AtomicU32,
    tx_underflow: AtomicU32,
    tx_unavailable: AtomicU32,
    tx_jabber: AtomicU32,
    bus_errors: AtomicU32,
}

impl DmaErrors {
    pub(super) const fn new() -> Self {
        Self {
            rx_overflow: AtomicU32::new(0),
            rx_unavailable: AtomicU32::new(0),
            rx_watchdog: AtomicU32::new(0),
            rx_stopped: AtomicU32::new(0),
            tx_underflow: AtomicU32::new(0),
            tx_unavailable: AtomicU32::new(0),
            tx_jabber: AtomicU32::new(0),
            bus_errors: AtomicU32::new(0),
        }
    }

    /// Count every error flag set in `status`; called from the handler with the
    /// value it is about to clear.
    pub(super) fn account(&self, status: u32) {
        count(&self.rx_overflow, status & STATUS_RX_OVERFLOW != 0);
        count(&self.rx_unavailable, status & STATUS_RX_UNAVAILABLE != 0);
        count(&self.rx_watchdog, status & STATUS_RX_WATCHDOG != 0);
        count(&self.rx_stopped, status & STATUS_RX_STOPPED != 0);
        count(&self.tx_underflow, status & STATUS_TX_UNDERFLOW != 0);
        count(&self.tx_unavailable, status & STATUS_TX_UNAVAILABLE != 0);
        count(&self.tx_jabber, status & STATUS_TX_JABBER != 0);
        count(&self.bus_errors, status & STATUS_BUS_ERROR != 0);
    }

    /// Fold the accumulated flags into a snapshot.
    pub(super) fn merge_into(&self, stats: &mut Stats) {
        stats.dma_rx_overflow = self.rx_overflow.load(Ordering::Relaxed);
        stats.dma_rx_unavailable = self.rx_unavailable.load(Ordering::Relaxed);
        stats.dma_rx_watchdog = self.rx_watchdog.load(Ordering::Relaxed);
        stats.dma_rx_stopped = self.rx_stopped.load(Ordering::Relaxed);
        stats.dma_tx_underflow = self.tx_underflow.load(Ordering::Relaxed);
        stats.dma_tx_unavailable = self.tx_unavailable.load(Ordering::Relaxed);
        stats.dma_tx_jabber = self.tx_jabber.load(Ordering::Relaxed);
        stats.dma_bus_errors = self.bus_errors.load(Ordering::Relaxed);
    }
}

#[inline]
fn count(counter: &AtomicU32, hit: bool) {
    if hit {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}
