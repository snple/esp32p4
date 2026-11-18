//! `mintstatus` register: the CLIC level of the interrupt being handled.
//!
//! `MIL` (bits 31:24) holds the 8-bit level byte of the currently executing
//! machine-mode interrupt, or zero when the core is not in a handler. `UIL`
//! (bits 7:0) is hardwired to 0 because the chip has no user mode interrupts.
//!
//! The byte is the same encoding as `clicintctl[i]`: three level bits at the
//! top (see [`mintthresh`](super::mintthresh)) and five low bits fixed to 1.

use core::arch::asm;

/// `MIL` field: the level of the interrupt being handled.
pub const MIL_SHIFT: usize = 24;
/// Mask for the `MIL` field.
pub const MIL_MASK: usize = 0xff << MIL_SHIFT;

/// Read `mintstatus`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    // SAFETY: reading a read-only CSR has no side effects.
    unsafe { asm!("csrr {}, 0xfb1", out(reg) value) };
    value
}

/// The raw 8-bit level byte, or 0 outside a handler.
#[inline(always)]
pub fn mil() -> u8 {
    ((read() & MIL_MASK) >> MIL_SHIFT) as u8
}

/// Whether the core is currently executing an interrupt handler.
#[inline(always)]
pub fn in_interrupt() -> bool {
    mil() != 0
}

/// The CLIC level the running handler was taken at, or 0 outside a handler.
#[inline(always)]
pub fn level() -> u8 {
    (mil() >> 5) & 0x7
}
