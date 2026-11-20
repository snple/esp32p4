//! The counters the MAC actually has.
//!
//! This MAC has no Synopsys MMC block, and it is not worth pretending it does:
//! the only real counters are in `DMAMISSEDFR`. Everything else is a latched
//! status bit or a per-frame flag in a descriptor, visible for one moment and
//! then gone, so the driver does not keep a parallel tally of them.

/// Frames the MAC dropped, as counted by `DMAMISSEDFR`.
///
/// The register is read-clear, so [`Emac::stats`](super::Emac::stats)
/// accumulates it; these totals grow for as long as the driver lives.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stats {
    /// No receive descriptor was free, so the frame had nowhere to go. Growing
    /// here means the stack is not draining the ring fast enough.
    pub rx_missed: u32,
    /// The RX FIFO overflowed while a frame was coming in.
    pub rx_fifo_overflows: u32,
}
