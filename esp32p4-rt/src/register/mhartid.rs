//! `mhartid` register.

use core::arch::asm;

/// Read the current hart ID.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, mhartid", out(reg) value) };
    value
}
