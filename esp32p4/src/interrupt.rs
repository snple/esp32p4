//! Interrupt lines and dispatch.
//!
//! The chip routes each peripheral interrupt source through a matrix
//! (`INTERRUPT_CORE0`) to one of 32 CPU interrupt lines, which the CLIC then
//! delivers to the core. Drivers in this crate claim a fixed line each:
//!
//! | Line | Owner |
//! |------|-------|
//! | 19 | `SYSTIMER` target 0, used by [`delay::Delay::sleep_us`](crate::delay::Delay::sleep_us) |
//! | 20 | Ethernet MAC |
//!
//! The lines are constants rather than an allocator because a bare-metal
//! application has a fixed set of drivers; pick a free number if you add one.
//!
//! Handlers run with interrupts disabled and must not nest. They should do the
//! minimum — clear the peripheral's status and record that work is pending —
//! and leave the actual processing to the main loop.

pub use esp32p4_rt::interrupt::{
    CPU_INTERRUPT_COUNT, disable, enable, free, is_enabled, is_pending, is_unmasked, mask, restore,
    set_handler, sleep, unmask,
};

/// CPU interrupt lines claimed by this HAL.
pub mod lines {
    /// `SYSTIMER` target 0 comparator, used for timed sleeps.
    pub const SYSTIMER_TARGET0: u32 = 19;
    /// Ethernet MAC (`ETH_MAC_INTR`).
    pub const EMAC: u32 = 20;
}
