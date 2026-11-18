//! `mcause` register.

use core::arch::asm;

/// Interrupt bit on RV32.
pub const INTERRUPT: usize = 1 << 31;

/// Read `mcause`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, mcause", out(reg) value) };
    value
}

/// Returns whether the trap was an interrupt.
#[inline(always)]
pub fn is_interrupt(cause: usize) -> bool {
    cause & INTERRUPT != 0
}

/// Exception or interrupt code without the interrupt bit.
#[inline(always)]
pub fn code(cause: usize) -> usize {
    cause & !INTERRUPT
}
