//! `minstret` counter.

use core::arch::asm;

/// Read the low 32 bits of `minstret`.
#[inline(always)]
pub fn read() -> u32 {
    let value;
    unsafe { asm!("csrr {}, minstret", out(reg) value) };
    value
}
