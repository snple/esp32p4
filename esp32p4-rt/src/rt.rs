//! Startup code and trap handling.
//!
//! The reset entry (`_start`) and the trap entry (`_start_trap`) are written in
//! assembly because they must run before Rust code may use the stack. They live
//! here, next to [`TrapFrame`], rather than in a separate `.S` file, matching the
//! way the other crates in this workspace keep assembly inline.

use crate::{asm, register};

/// Whether the trap frame carries caller-saved FPU state.
///
/// Set by the build script from the target triple: rustc does not report the
/// `f` extension through `cfg(target_feature)`, so a `riscv32imac` build is the
/// one that leaves `riscv_fpu` unset and skips `fsw`/`flw` entirely.
#[cfg(riscv_fpu)]
const FP_FRAME: u32 = 1;
#[cfg(not(riscv_fpu))]
const FP_FRAME: u32 = 0;

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

    /* Direct-mode trap vector: every exception and interrupt enters through
       `_start_trap`, which dispatches on mcause. */
    la t0, _start_trap
    csrw mtvec, t0
    csrw mie, zero
    csrci mstatus, 8          /* clear MIE */

    /* Configure the CLIC but leave every line masked; drivers unmask the
       lines they own. */
    call {clic_init}

    /* Enable the FPU (mstatus.FS = Dirty) so F-extension code can run. */
    li t0, 0x6000
    csrs mstatus, t0

    call main

4:
    j 4b
.size _start, . - _start

.section .trap, "ax"
/* The HP core runs in CLIC mode only: `mtvec` reads back with MODE = 3 and the
   trap vector base is `mtvec & !0x3f`. The entry therefore has to be 64-byte
   aligned, or every exception jumps into the middle of this function and the
   core hangs inside a fault loop that never records `TRAP_CAUSE`. */
.balign 64
.global _start_trap
.type _start_trap, @function
_start_trap:
    addi sp, sp, -160
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

    /* Caller-saved FPU state. The interrupted code may hold values here, and
       the handler is ordinary Rust that the compiler is free to use FP
       registers in, so they have to be preserved. Targets without the F
       extension skip the block via `.equ` below.

       Frame: 16 integer words (0..60), fcsr (64), 20 FP words (68..144). */
    .if {SAVE_FP}
    frcsr t0
    sw t0, 64(sp)
    fsw ft0,  68(sp)
    fsw ft1,  72(sp)
    fsw ft2,  76(sp)
    fsw ft3,  80(sp)
    fsw ft4,  84(sp)
    fsw ft5,  88(sp)
    fsw ft6,  92(sp)
    fsw ft7,  96(sp)
    fsw ft8, 100(sp)
    fsw ft9, 104(sp)
    fsw ft10, 108(sp)
    fsw ft11, 112(sp)
    fsw fa0, 116(sp)
    fsw fa1, 120(sp)
    fsw fa2, 124(sp)
    fsw fa3, 128(sp)
    fsw fa4, 132(sp)
    fsw fa5, 136(sp)
    fsw fa6, 140(sp)
    fsw fa7, 144(sp)
    .endif

    mv a0, sp
    call _start_trap_rust

    .if {SAVE_FP}
    flw ft0,  68(sp)
    flw ft1,  72(sp)
    flw ft2,  76(sp)
    flw ft3,  80(sp)
    flw ft4,  84(sp)
    flw ft5,  88(sp)
    flw ft6,  92(sp)
    flw ft7,  96(sp)
    flw ft8, 100(sp)
    flw ft9, 104(sp)
    flw ft10, 108(sp)
    flw ft11, 112(sp)
    flw fa0, 116(sp)
    flw fa1, 120(sp)
    flw fa2, 124(sp)
    flw fa3, 128(sp)
    flw fa4, 132(sp)
    flw fa5, 136(sp)
    flw fa6, 140(sp)
    flw fa7, 144(sp)
    lw t0, 64(sp)
    fscsr t0
    .endif

    lw ra,  0(sp)
    lw t0,  4(sp)
    lw t1,  8(sp)
    lw t2, 12(sp)
    lw a0, 16(sp)
    lw a1, 20(sp)
    lw a2, 24(sp)
    lw a3, 28(sp)
    lw a4, 32(sp)
    lw a5, 36(sp)
    lw a6, 40(sp)
    lw a7, 44(sp)
    lw t3, 48(sp)
    lw t4, 52(sp)
    lw t5, 56(sp)
    lw t6, 60(sp)
    addi sp, sp, 160
    mret
.size _start_trap, . - _start_trap
"#,
    clic_init = sym crate::interrupt::init,
    SAVE_FP = const FP_FRAME,
);

/// Entry point core 1 jumps to inside this runtime.
///
/// Core 1 starts at a bare reset vector with no arguments, so the entry and the
/// stack are handed over through these two words instead.
#[unsafe(no_mangle)]
pub static CORE1_ENTRY: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
/// Stack top core 1 loads before it runs anything else.
#[unsafe(no_mangle)]
pub static CORE1_STACK: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

core::arch::global_asm!(
    r#"
.section .init, "ax"
/* Entry point for HP core 1, which starts with nothing set up: the core comes
   out of reset in machine mode with whatever `sp` and `gp` the ROM left, its
   own CLIC unconfigured and the FPU off. Everything `_start` does for core 0
   except the `.bss` zeroing (shared, already done) is done here. */
.global _start_core1
.type _start_core1, @function
_start_core1:
    /* Stack first: nothing may touch it before this. */
    la t0, {stack}
    lw sp, 0(t0)
    /* Keep the entry in a callee-saved register: the calls below are ordinary
       Rust functions and may clobber the argument registers. */
    la t0, {entry}
    lw s0, 0(t0)

    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    /* Its own trap vector: the CLIC is per hart. */
    la t0, _start_trap
    csrw mtvec, t0
    csrw mie, zero
    csrci mstatus, 8          /* clear MIE */

    call {clic_init}

    /* Enable the FPU (mstatus.FS = Dirty) so F-extension code can run. */
    li t0, 0x6000
    csrs mstatus, t0

    jalr zero, 0(s0)

    /* The entry point must never return. */
5:
    j 5b
.size _start_core1, . - _start_core1
"#,
    clic_init = sym crate::interrupt::init,
    entry = sym CORE1_ENTRY,
    stack = sym CORE1_STACK,
);

/// Hands core 1 its entry point and stack.
///
/// # Safety
///
/// `stack_top` must be the top of a writable region nothing else uses, 16-byte
/// aligned and large enough for the trap frame; `entry` must never return.
pub unsafe fn set_core1_entry(entry: fn() -> !, stack_top: usize) {
    use core::sync::atomic::Ordering;
    CORE1_STACK.store(stack_top, Ordering::Release);
    CORE1_ENTRY.store(entry as usize, Ordering::Release);
}

/// The address core 1 has to start at.
///
/// This is what goes into the chip's core-1 boot address register before the
/// core is released from reset.
#[inline]
pub fn core1_boot_address() -> usize {
    unsafe extern "C" {
        fn _start_core1() -> !;
    }
    _start_core1 as *const () as usize
}

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
/// Interrupts are dispatched to the handler registered for their CPU interrupt
/// line and then return to the interrupted code. Exceptions have no recovery
/// path in this minimal runtime, so they park the core with `TRAP_CAUSE` /
/// `TRAP_EPC` left readable from a debugger.
#[unsafe(no_mangle)]
pub extern "C" fn _start_trap_rust(_frame: &mut TrapFrame) {
    let cause = register::mcause::read();
    let (is_interrupt, code) = crate::interrupt::decode_cause(cause);

    if is_interrupt {
        // `mcause` holds the CLIC id itself, which covers the CLINT's own
        // software and timer interrupts as well as the CPU interrupt lines.
        crate::interrupt::dispatch(code as u32);
        return;
    }

    let epc = register::mepc::read();
    unsafe {
        TRAP_CAUSE = cause;
        TRAP_EPC = epc;
    }
    loop {
        asm::wfi();
    }
}
