//! # esp32p4-rt
//!
//! Minimal runtime support for the ESP32-P4 high-performance RISC-V cores.
//!
//! This crate provides:
//! - `_start`: stack setup, `.bss` / `.ram.high` zeroing, `mtvec` setup and the
//!   call into `main` (in [`rt`])
//! - a direct-mode trap entry that saves the integer register file and calls
//!   `_start_trap_rust`
//! - RISC-V assembly helpers and machine-mode CSR access
//!
//! ## Startup sequence
//!
//! The ESP-IDF 2nd-stage bootloader maps the flash segments, copies the RAM
//! segments and jumps to `_start` in machine mode. The runtime then:
//!
//! 1. sets `gp` and `sp`
//! 2. zeroes `.bss` and the second RAM window, `.ram.high`
//! 3. installs the direct-mode trap vector and disables interrupts
//! 4. enables the FPU (`mstatus.FS = Dirty`)
//! 5. calls `main`
//!
//! ## Linker script
//!
//! `build.rs` copies `link.x` into `OUT_DIR` and adds it to the linker search
//! path. `link.x` includes `memory.x`, which the application supplies from its
//! own build script.
//!
//! The layout matches what the ESP-IDF 2nd-stage bootloader expects:
//!
//! - flash segment 0: `.flash.appdesc` (the `esp_app_desc_t` that espflash
//!   requires) followed by `.rodata`
//! - flash segment 1: `.init`, `.text` and `.trap`, separated from segment 0
//!   by a dummy gap section
//! - RAM: `.data` (copied by the bootloader), `.bss` (zeroed here) and stack
//! - RAM_HIGH: `.ram.high` (zeroed here), for buffers too big for `RAM`
//!
//! The dummy gap is required because the bootloader asserts that the image has
//! exactly two flash segments on ESP32-P4.

#![no_std]
// The `asm_sub_register` lint only applies to x86 sub-register formatting.
// This crate targets RV32, where every `reg` operand names a full 32-bit
// register, so the suggestion (`{n:e}` / `{n:r}`) does not apply.
#![allow(asm_sub_register)]

mod app_desc;
pub mod asm;
pub mod interrupt;
pub mod register;

#[cfg(feature = "rt")]
mod rt;

#[cfg(feature = "rt")]
pub use rt::TrapFrame;
#[cfg(feature = "rt")]
pub use rt::{CORE1_ENTRY, CORE1_STACK, TRAP_CAUSE, TRAP_EPC, core1_boot_address, set_core1_entry};
