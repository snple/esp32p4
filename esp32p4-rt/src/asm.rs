//! RISC-V assembly helpers.

use core::arch::asm;

/// No-operation.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack)) }
}

/// Wait for an interrupt.
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack)) }
}

/// Full memory fence.
#[inline(always)]
pub fn fence() {
    unsafe { asm!("fence") }
}

/// Instruction fence.
#[inline(always)]
pub fn fence_i() {
    unsafe { asm!("fence.i") }
}

/// Breakpoint.
#[inline(always)]
pub fn ebreak() {
    unsafe { asm!("ebreak") }
}

/// Busy-wait for roughly `cycles` core clock cycles.
///
/// The loop body is a single `addi`/`bnez` pair, so the delay tracks the
/// requested count to within a few cycles of loop overhead.
#[inline(always)]
pub fn delay_cycles(cycles: u32) {
    if cycles == 0 {
        return;
    }

    // A numeric label must not consist of `0`/`1` digits only (`1b` would read
    // as a binary literal), so this loop counts from 2.
    unsafe {
        asm!(
            "2:",
            "addi {n}, {n}, -1",
            "bnez {n}, 2b",
            n = inout(reg) cycles => _,
            options(nomem, nostack),
        );
    }
}
