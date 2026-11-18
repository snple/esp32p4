//! `mintthresh` register: the CLIC interrupt priority threshold.
//!
//! Interrupts whose priority is not *greater* than the threshold are held off.
//! The reset value is 0, so any interrupt with a nonzero priority is taken.

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
