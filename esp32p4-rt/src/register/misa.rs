//! `misa` register.

use core::arch::asm;

/// Read `misa`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, misa", out(reg) value) };
    value
}
