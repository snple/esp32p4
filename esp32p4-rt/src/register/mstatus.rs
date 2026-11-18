//! `mstatus` register.

use core::arch::asm;

/// `MIE` bit: machine interrupt enable.
pub const MIE: usize = 1 << 3;
/// `MPIE` bit: previous machine interrupt enable.
pub const MPIE: usize = 1 << 7;
/// `FS` field: FPU state.
pub const FS_MASK: usize = 0b11 << 13;
/// `FS = Dirty`.
pub const FS_DIRTY: usize = 0b11 << 13;

/// Read `mstatus`.
#[inline(always)]
pub fn read() -> usize {
    let value;
    unsafe { asm!("csrr {}, mstatus", out(reg) value) };
    value
}

/// Write `mstatus`.
///
/// # Safety
///
/// Changing machine status bits can affect interrupt delivery and FPU state.
#[inline(always)]
pub unsafe fn write(value: usize) {
    unsafe { asm!("csrw mstatus, {}", in(reg) value) }
}

/// Returns whether machine interrupts are enabled.
#[inline(always)]
pub fn mie() -> bool {
    read() & MIE != 0
}

/// Set `MIE`.
///
/// # Safety
///
/// Enables machine interrupts.
#[inline(always)]
pub unsafe fn set_mie() {
    unsafe { asm!("csrsi mstatus, 8") }
}

/// Clear `MIE`.
#[inline(always)]
pub fn clear_mie() {
    unsafe { asm!("csrci mstatus, 8") }
}

/// Enable the FPU by setting `FS = Dirty`.
#[inline(always)]
pub fn enable_fpu() {
    unsafe { asm!("csrs mstatus, {}", in(reg) FS_DIRTY) }
}
