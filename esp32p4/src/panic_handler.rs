//! Default panic handling for bare-metal `esp32p4` binaries.
//!
//! The panic path runs on the high-performance RISC-V core: machine interrupts
//! are disabled first, the message goes through the active `defmt` transport
//! when `defmt` is enabled, and the core finally executes `ebreak`.
//!
//! With a debugger attached the breakpoint stops the core at the panic site.
//! Without one, the breakpoint escalates to the trap entry in `esp32p4-rt`,
//! which records `mcause`/`mepc` in `TRAP_CAUSE`/`TRAP_EPC` and parks the core.

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

/// Panic entry used by `defmt::panic!`.
///
/// That macro has already emitted the message, so this path traps without
/// printing it a second time through the plain `#[panic_handler]`.
#[cfg(feature = "defmt")]
#[defmt::panic_handler]
fn defmt_panic() -> ! {
    panic_trap()
}

#[cfg(feature = "defmt")]
fn print_panic(info: &PanicInfo) {
    defmt::error!("{}", defmt::Display2Format(info));
}

#[cfg(not(feature = "defmt"))]
fn print_panic(_: &PanicInfo) {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // A logger can panic while the message is being formatted — the USB Serial
    // logger does exactly that when acquired reentrantly. The flag keeps that
    // second panic from recursing into `print_panic`; the trap is reached
    // either way.
    static PANICKED: AtomicBool = AtomicBool::new(false);

    crate::rt::interrupt::disable();

    if !PANICKED.swap(true, Ordering::Relaxed) {
        print_panic(info);
    }

    panic_trap()
}

/// Stop the core on the failure.
///
/// `ebreak` raises a breakpoint exception: a debugger halts at this instruction
/// and can inspect the stack, while a bare board takes the runtime's exception
/// path, which saves the cause and parks the core.
fn panic_trap() -> ! {
    crate::rt::asm::ebreak();

    // Reached only if a debugger resumes from the breakpoint (or if the trap
    // entry returns). `MIE` is clear, so the core sleeps here for good.
    loop {
        crate::rt::asm::wfi();
    }
}
