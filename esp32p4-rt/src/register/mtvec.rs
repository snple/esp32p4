//! `mtvec` register.

use core::arch::asm;

/// Read `mtvec`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, mtvec", out(reg) value) };
    value
}

/// Write `mtvec`.
///
/// # Safety
///
/// The address must point to a valid trap entry with the requested mode.
#[inline(always)]
pub unsafe fn write(value: usize) {
    unsafe { asm!("csrw mtvec, {}", in(reg) value) }
}
