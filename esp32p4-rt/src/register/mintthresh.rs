//! `mintthresh` register: the CLIC interrupt level threshold.
//!
//! The 8-bit `TH` field is compared with `mintstatus.MIL`; the larger of the two
//! is the effective threshold, and an interrupt is only taken when its
//! `clicintctl` level byte is *greater*. `clicintctl` resets to `0x1f` (level 0,
//! low bits hardwired to 1) and `TH` to 0, so every level is accepted after
//! reset.
//!
//! The runtime never raises the threshold: handlers run with `MIE = 0`, so no
//! interrupt can preempt another one in the first place.

use core::arch::asm;

/// Read `mintthresh`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, 0x347", out(reg) value) };
    value
}

/// Write `mintthresh`.
///
/// # Safety
///
/// Raising the threshold masks interrupts; lowering it can let one fire while
/// the handler for a lower-priority interrupt is still running.
#[inline(always)]
pub unsafe fn write(value: usize) {
    unsafe { asm!("csrw 0x347, {}", in(reg) value) }
}
