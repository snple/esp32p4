//! `mepc` register.

use core::arch::asm;

/// Read `mepc`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, mepc", out(reg) value) };
    value
}

/// Write `mepc`.
///
/// # Safety
///
/// The value must be a valid instruction address.
#[inline(always)]
pub unsafe fn write(value: usize) {
    unsafe { asm!("csrw mepc, {}", in(reg) value) }
}
