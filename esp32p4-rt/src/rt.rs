//! Startup code and trap handling.
//!
//! The reset entry (`_start`) and the trap entry (`_start_trap`) are written in
//! assembly because they must run before Rust code may use the stack. They live
//! here, next to [`TrapFrame`], rather than in a separate `.S` file, matching the
//! way the other crates in this workspace keep assembly inline.

use crate::{asm, register};

core::arch::global_asm!(
    r#"
.section .init, "ax"
.global _start
.type _start, @function
_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    la sp, _stack_start

    /* Zero .bss */
    la t0, _sbss
    la t1, _ebss
2:
    bgeu t0, t1, 3f
    sw zero, 0(t0)
    addi t0, t0, 4
    j 2b
3:

    /* .data was copied to RAM by the 2nd-stage bootloader. */

    /* Direct-mode trap vector. No interrupts are enabled by the runtime. */
    la t0, _start_trap
    csrw mtvec, t0
    csrw mie, zero
    csrci mstatus, 8          /* clear MIE */

    /* Enable the FPU (mstatus.FS = Dirty) so F-extension code can run. */
    li t0, 0x6000
    csrs mstatus, t0

    call main

4:
    j 4b
.size _start, . - _start

.section .trap, "ax"
.align 4
.global _start_trap
.type _start_trap, @function
_start_trap:
    addi sp, sp, -64
    sw ra,  0(sp)
    sw t0,  4(sp)
    sw t1,  8(sp)
    sw t2, 12(sp)
    sw a0, 16(sp)
    sw a1, 20(sp)
    sw a2, 24(sp)
    sw a3, 28(sp)
    sw a4, 32(sp)
    sw a5, 36(sp)
    sw a6, 40(sp)
    sw a7, 44(sp)
    sw t3, 48(sp)
    sw t4, 52(sp)
    sw t5, 56(sp)
    sw t6, 60(sp)

    mv a0, sp
    call _start_trap_rust

5:
    j 5b
.size _start_trap, . - _start_trap
"#
);

/// Integer register frame saved by `_start_trap`.
#[repr(C)]
pub struct TrapFrame {
    /// Return address.
    pub ra: usize,
    /// Temporary register t0.
    pub t0: usize,
    /// Temporary register t1.
    pub t1: usize,
    /// Temporary register t2.
    pub t2: usize,
    /// Argument register a0.
    pub a0: usize,
    /// Argument register a1.
    pub a1: usize,
    /// Argument register a2.
    pub a2: usize,
    /// Argument register a3.
    pub a3: usize,
    /// Argument register a4.
    pub a4: usize,
    /// Argument register a5.
    pub a5: usize,
    /// Argument register a6.
    pub a6: usize,
    /// Argument register a7.
    pub a7: usize,
    /// Temporary register t3.
    pub t3: usize,
    /// Temporary register t4.
    pub t4: usize,
    /// Temporary register t5.
    pub t5: usize,
    /// Temporary register t6.
    pub t6: usize,
}

/// Last `mcause` value captured by the trap handler.
#[unsafe(no_mangle)]
pub static mut TRAP_CAUSE: usize = 0;
/// Last `mepc` value captured by the trap handler.
#[unsafe(no_mangle)]
pub static mut TRAP_EPC: usize = 0;

/// Rust-side trap entry called from `_start_trap`.
///
/// The minimal runtime has no console yet, so an unexpected trap parks the
/// core. `TRAP_CAUSE` / `TRAP_EPC` remain readable from a debugger.
#[unsafe(no_mangle)]
pub extern "C" fn _start_trap_rust(_frame: &TrapFrame) -> ! {
    let cause = register::mcause::read();
    let epc = register::mepc::read();
    unsafe {
        TRAP_CAUSE = cause;
        TRAP_EPC = epc;
    }
    loop {
        asm::wfi();
    }
}
