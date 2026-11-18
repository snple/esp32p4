//! `mcycle` counter.

use core::arch::asm;

/// Read the low 32 bits of `mcycle`.
#[inline(always)]
pub fn read() -> u32 {
    let value;
    unsafe { asm!("csrr {}, mcycle", out(reg) value) };
    value
}

/// Write the low 32 bits of `mcycle`.
#[inline(always)]
pub fn write(value: u32) {
    unsafe { asm!("csrw mcycle, {}", in(reg) value) }
}
