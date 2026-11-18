//! Rust code generator v3 - Proper memory-mapped registers with field accessors
//!
//! Generates PAC code where:
//! - Each register is a `#[repr(transparent)]` struct with `UnsafeCell<u32>`
//! - RegisterBlock is a `#[repr(C)]` struct that directly maps to hardware memory
//! - Each register has field accessor methods with proper documentation
//! - Supports atomic SET/CLR/XOR operations via RP2350 hardware aliases

use crate::types::*;
use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// Generate all modules
pub fn generate_all(device: &SvdDevice, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    let periph_map: HashMap<_, _> = device
        .peripherals
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();

    // Generate common module
    fs::write(output_dir.join("common.rs"), generate_common())?;
    println!("  Generated common.rs");

    // Collect peripherals to generate
    let peripherals: Vec<_> = device
        .peripherals
        .iter()
        .filter(|p| p.base_address.is_some())
        .collect();

    // Generate each peripheral
    for periph in &peripherals {
        let content = generate_peripheral(periph, &periph_map)?;
        let filename = format!("{}.rs", periph.module_name());
        fs::write(output_dir.join(&filename), content)?;
        println!("  Generated {}", filename);
    }

    // Generate mod.rs
    let mod_content = generate_mod_rs(&peripherals);
    fs::write(output_dir.join("mod.rs"), mod_content)?;
    println!("  Generated mod.rs");

    Ok(())
}

fn generate_mod_rs(peripherals: &[&Peripheral]) -> String {
    let mut out = String::new();

    writeln!(out, "//! ESP32-P4 Hardware Register Definitions").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(
        out,
        "//! Auto-generated from esp32p4.svd - DO NOT EDIT MANUALLY"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_camel_case_types)]").unwrap();
    writeln!(out, "#![allow(clippy::identity_op)]").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "use core::marker::PhantomData;").unwrap();
    writeln!(out, "use core::ops::Deref;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "pub mod common;").unwrap();
    writeln!(out).unwrap();

    // Collect all unique interrupt sources. The ESP32-P4 exposes a 128-entry
    // interrupt matrix; the SVD lists the peripheral-side source numbers.
    let mut all_interrupts: Vec<&Interrupt> = peripherals
        .iter()
        .flat_map(|p| p.interrupts.iter())
        .collect();
    all_interrupts.sort_by_key(|int| int.value);
    all_interrupts.dedup_by_key(|int| int.value);

    if !all_interrupts.is_empty() {
        writeln!(out, "/// ESP32-P4 interrupt sources.").unwrap();
        writeln!(out, "#[derive(Copy, Clone, Debug, PartialEq, Eq)]").unwrap();
        writeln!(out, "#[repr(u16)]").unwrap();
        writeln!(out, "pub enum Interrupt {{").unwrap();
        for int in &all_interrupts {
            let desc = if int.description.is_empty() {
                "interrupt source".to_string()
            } else {
                short_desc(&int.description)
            };
            writeln!(out, "    /// {} - {}", int.value, desc).unwrap();
            writeln!(
                out,
                "    {} = {},",
                safe_shouty_snake_case(&int.name),
                int.value
            )
            .unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "impl Interrupt {{").unwrap();
        writeln!(out, "    /// Raw interrupt-matrix source number.").unwrap();
        writeln!(out, "    #[inline(always)]").unwrap();
        writeln!(out, "    pub const fn number(self) -> u16 {{").unwrap();
        writeln!(out, "        self as u16").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
    }

    // Generate peripheral structs
    // Generate peripheral! macro
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripheral Wrapper Macro").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// Helper macro to create peripheral wrappers").unwrap();
    writeln!(out, "macro_rules! peripheral {{").unwrap();
    writeln!(out, "    ($name:ident, $reg:ty, $base:expr) => {{").unwrap();
    writeln!(out, "        impl $name {{").unwrap();
    writeln!(
        out,
        "            /// Pointer to the peripheral's register block"
    )
    .unwrap();
    writeln!(
        out,
        "            pub const PTR: *const $reg = $base as *const $reg;"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            /// Steal an instance of this peripheral").unwrap();
    writeln!(out, "            ///").unwrap();
    writeln!(out, "            /// # Safety").unwrap();
    writeln!(out, "            ///").unwrap();
    writeln!(
        out,
        "            /// Ensure that the new instance of the peripheral cannot be used in a way"
    )
    .unwrap();
    writeln!(
        out,
        "            /// that may race with any existing instances."
    )
    .unwrap();
    writeln!(out, "            #[inline]").unwrap();
    writeln!(out, "            pub const unsafe fn steal() -> Self {{").unwrap();
    writeln!(out, "                Self {{").unwrap();
    writeln!(out, "                    _marker: PhantomData,").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        impl Deref for $name {{").unwrap();
    writeln!(out, "            type Target = $reg;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            #[inline(always)]").unwrap();
    writeln!(out, "            fn deref(&self) -> &Self::Target {{").unwrap();
    writeln!(out, "                unsafe {{ &*Self::PTR }}").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        impl core::fmt::Debug for $name {{").unwrap();
    writeln!(
        out,
        "            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{"
    )
    .unwrap();
    writeln!(
        out,
        "                f.debug_struct(stringify!($name)).finish()"
    )
    .unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }};").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripheral Definitions").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    for periph in peripherals {
        let struct_name = safe_shouty_snake_case(&periph.name);
        let mod_name = periph.module_name();
        let base_addr = periph.base_address.unwrap();

        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "pub struct {} {{", struct_name).unwrap();
        writeln!(out, "    _marker: PhantomData<*const ()>,").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "unsafe impl Send for {} {{}}", struct_name).unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "peripheral!({}, {}::RegisterBlock, {:#010x});",
            struct_name, mod_name, base_addr
        )
        .unwrap();
        writeln!(out).unwrap();
        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} register module", periph.name).unwrap();
        }
        writeln!(out, "pub mod {};", mod_name).unwrap();
        writeln!(out).unwrap();
    }

    // Generate Peripherals struct
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripherals - All device peripherals").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "use core::sync::atomic::{{AtomicBool, Ordering}};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "/// Guard to ensure peripherals are taken only once").unwrap();
    writeln!(out, "static TAKEN: AtomicBool = AtomicBool::new(false);").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// All the peripherals.").unwrap();
    writeln!(out, "#[allow(non_snake_case)]").unwrap();
    writeln!(out, "pub struct Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = safe_shouty_snake_case(&periph.name);
        if !periph.description.is_empty() {
            writeln!(out, "    /// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "    /// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "    pub {}: {},", struct_name, struct_name).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "impl Peripherals {{").unwrap();
    writeln!(out, "    /// Returns all the peripherals *once*.").unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub fn take() -> Option<Self> {{").unwrap();
    writeln!(out, "        if TAKEN.swap(true, Ordering::AcqRel) {{").unwrap();
    writeln!(out, "            None").unwrap();
    writeln!(out, "        }} else {{").unwrap();
    writeln!(out, "            Some(unsafe {{ Self::steal() }})").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    /// Unchecked version of `Peripherals::take`.").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(out, "    /// # Safety").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(
        out,
        "    /// Each of the returned peripherals must be used at most once."
    )
    .unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub unsafe fn steal() -> Self {{").unwrap();
    // Rust 2024: `unsafe fn` bodies are not implicit unsafe blocks, so the
    // inner `unsafe {}` is required for the `X::steal()` calls below
    // (see the `unsafe-op-in-unsafe-fn` lint).
    writeln!(out, "        unsafe {{").unwrap();
    writeln!(out, "            Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = safe_shouty_snake_case(&periph.name);
        writeln!(
            out,
            "                {}: {}::steal(),",
            struct_name, struct_name
        )
        .unwrap();
    }
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    /// Release the peripherals back to the pool").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(
        out,
        "    /// After calling this, `take()` will succeed again."
    )
    .unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub fn release(self) {{").unwrap();
    writeln!(out, "        TAKEN.store(false, Ordering::Release);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn generate_common() -> String {
    r##"//! Common types and utilities for register access

#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> VolatileCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}

"##
    .to_string()
}

/// ESP32-P4 has no Cortex-M-style atomic SET/CLR/XOR register aliases.
///
/// Every writable register therefore uses `VolatileCell`; the generated
/// set/clear helpers perform a volatile read-modify-write. Peripherals that
/// provide W1TS/W1TC registers (e.g. GPIO) expose them as normal registers.
fn supports_atomic_aliases(_base_address: u32) -> bool {
    false
}

fn generate_peripheral(
    periph: &Peripheral,
    periph_map: &HashMap<&str, &Peripheral>,
) -> Result<String> {
    let registers = if let Some(ref derived) = periph.derived_from {
        if let Some(base) = periph_map.get(derived.as_str()) {
            &base.registers
        } else {
            &periph.registers
        }
    } else {
        &periph.registers
    };

    let registers = expand_registers(registers);
    generate_generic(periph, &registers)
}

/// Expand `<dim>` register arrays into ordinary registers and sort them by
/// offset. ESP SVDs use `%s` / `[%s]` placeholders in the array name.
fn expand_registers(registers: &[Register]) -> Vec<Register> {
    let mut expanded = Vec::new();
    for reg in registers {
        for (offset, name, description) in reg.expanded() {
            let mut reg = reg.clone();
            reg.offset = offset;
            reg.name = name;
            reg.description = description;
            reg.dim = None;
            reg.dim_increment = None;
            expanded.push(reg);
        }
    }
    expanded.sort_by_key(|r| r.offset);
    expanded
}

fn generate_generic(periph: &Peripheral, registers: &[Register]) -> Result<String> {
    let mut out = String::new();

    // Check if this peripheral supports atomic aliases
    let supports_atomic = periph
        .base_address
        .map(supports_atomic_aliases)
        .unwrap_or(false);

    write_header(&mut out, periph, supports_atomic)?;

    // Two SVD registers can collapse to the same CamelCase type name (for
    // example `OUT_CONF0_CH%s` element 0 and a literal `OUT_CONF0_CH0`).
    // Assign deterministic, unique type names before emitting any code.
    let mut type_names = Vec::with_capacity(registers.len());
    let mut used_names = HashSet::new();
    for reg in registers {
        let base = safe_type_name(&reg.name);
        let mut name = base.clone();
        let mut suffix = 1;
        while !used_names.insert(name.clone()) {
            name = format!("{}{}", base, suffix);
            suffix += 1;
        }
        type_names.push(name);
    }

    // Generate each register struct with field accessors
    for (reg, type_name) in registers.iter().zip(&type_names) {
        write_register_struct(&mut out, reg, supports_atomic, type_name)?;
        // Generate the value struct for batch read/write operations
        write_register_value_struct(&mut out, reg, type_name)?;
    }

    // Generate RegisterBlock
    write_register_block(&mut out, &periph.name, registers, &type_names)?;

    Ok(out)
}

/// Generate a single register struct with all field accessors
fn write_register_struct(
    out: &mut String,
    reg: &Register,
    supports_atomic: bool,
    struct_name: &str,
) -> Result<()> {
    // Document the register
    writeln!(out, "/// {} Register", reg.name)?;
    if !reg.description.is_empty() {
        writeln!(out, "///")?;
        for line in clean_desc(&reg.description).lines() {
            writeln!(out, "/// {}", line)?;
        }
    }
    writeln!(out, "///")?;
    writeln!(out, "/// Address offset: {:#06x}", reg.offset)?;
    if let Some(reset_val) = reg.reset_value {
        writeln!(out, "/// Reset value: {:#010x}", reset_val)?;
    }

    // Document fields
    if !reg.fields.is_empty() {
        writeln!(out, "///")?;
        writeln!(out, "/// ## Fields")?;
        writeln!(out, "///")?;
        for (i, field) in reg.fields.iter().enumerate() {
            if i > 0 {
                writeln!(out, "///")?;
            }
            let access_str = match field.access {
                Access::ReadOnly => "RO",
                Access::WriteOnly => "WO",
                Access::ReadWrite => "RW",
            };
            let bit_range = if field.lsb == field.msb {
                format!("`[{}]`", field.lsb)
            } else {
                format!("`[{}:{}]`", field.msb, field.lsb)
            };
            // First line: field name, bit range, access type
            writeln!(
                out,
                "/// - **{}** {} ({})",
                field.name, bit_range, access_str
            )?;
            // Description lines (preserve original line breaks)
            if !field.description.is_empty() {
                writeln!(out, "///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "///   {}", line)?;
                }
            }
        }
    }

    // ESP32-P4 has no hardware SET/CLR/XOR register aliases, so every
    // register is a plain volatile cell. Peripherals that offer W1TS/W1TC
    // registers expose those as ordinary write-only registers.
    let cell_type = "crate::common::VolatileCell<u32>";

    writeln!(out, "#[repr(transparent)]")?;
    writeln!(out, "pub struct {} {{", struct_name)?;
    writeln!(out, "    value: {},", cell_type)?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl core::ops::Deref for {} {{", struct_name)?;
    writeln!(out, "    type Target = {};", cell_type)?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    fn deref(&self) -> &Self::Target {{")?;
    writeln!(out, "        &self.value")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", struct_name)?;

    // Generate field constants
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Constants ========")?;
        writeln!(out)?;
        for field in &reg.fields {
            let field_shouty = safe_shouty_snake_case(&field.name);

            // Field description (preserve original line breaks)
            writeln!(out, "    /// **{}**", field.name)?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
                writeln!(out, "    ///")?;
            }
            writeln!(
                out,
                "    /// Bits `[{}:{}]`, mask `{:#010x}`",
                field.msb,
                field.lsb,
                field.mask()
            )?;
            writeln!(
                out,
                "    pub const {}_OFFSET: u32 = {};",
                field_shouty, field.lsb
            )?;
            writeln!(
                out,
                "    pub const {}_MASK: u32 = {:#010x};",
                field_shouty,
                field.mask()
            )?;

            // For single-bit fields, add a simple bit constant
            if field.width() == 1 {
                writeln!(
                    out,
                    "    pub const {}: u32 = 1 << {};",
                    field_shouty, field.lsb
                )?;
            }

            // Enum values - generate both shifted (for direct register access) and unshifted (for xxxVal)
            for ev in &field.enum_values {
                let ev_name = safe_shouty_snake_case(&ev.name);
                if !ev.description.is_empty() {
                    writeln!(out, "    /// {} = {}", ev.name, ev.value)?;
                    for line in clean_desc(&ev.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                // Shifted value - for direct register write via Basic Access Methods
                writeln!(
                    out,
                    "    pub const {}_{}: u32 = {} << {};",
                    field_shouty, ev_name, ev.value, field.lsb
                )?;
                // Unshifted value - for use with xxxVal's with_xxx() methods
                writeln!(
                    out,
                    "    pub const {}_{}_VAL: u32 = {};",
                    field_shouty, ev_name, ev.value
                )?;
            }
            writeln!(out)?;
        }
    }

    // Generate field accessor methods
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Accessors ========")?;
        writeln!(out)?;

        for field in &reg.fields {
            let field_snake = safe_snake_case(&field.name);
            let field_shouty = safe_shouty_snake_case(&field.name);
            let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
            let width = field.width();

            // Check if we will generate any method for this field
            let will_generate_read = is_readable;
            // RW multi-bit field in RW register -> write_xxx method using modify
            let will_generate_write = matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width > 1;
            // RW single-bit field in RW register -> set_xxx/clr_xxx methods
            let will_generate_set_clr = matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width == 1;
            // WriteOnly single-bit field -> trigger_xxx method
            let will_generate_trigger = matches!(field.access, Access::WriteOnly) && width == 1;
            // WriteOnly multi-bit field -> write_xxx method
            let will_generate_write_only = matches!(field.access, Access::WriteOnly) && width > 1;

            // Skip if no methods will be generated
            if !will_generate_read
                && !will_generate_write
                && !will_generate_set_clr
                && !will_generate_trigger
                && !will_generate_write_only
            {
                continue;
            }

            // Access string for documentation
            let access_str = match field.access {
                Access::ReadOnly => "RO",
                Access::WriteOnly => "WO",
                Access::ReadWrite => "RW",
            };

            // Bit range string
            let bit_range = if field.lsb == field.msb {
                format!("`[{}]`", field.lsb)
            } else {
                format!("`[{}:{}]`", field.msb, field.lsb)
            };

            // Document the field (only once, before the first method)
            // Skip for WriteOnly fields as they'll be documented with their trigger/write methods
            if !matches!(field.access, Access::WriteOnly) {
                writeln!(
                    out,
                    "    /// **{}** {} ({})",
                    field.name, bit_range, access_str
                )?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
            }

            // Read accessor (for readable fields)
            if is_readable {
                if width == 1 {
                    // Single bit - return bool
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> bool {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.value.read_bits() & Self::{}_MASK) != 0",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                } else {
                    // Multi-bit - return u32
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> u32 {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.value.read_bits() & Self::{}_MASK) >> Self::{}_OFFSET",
                        field_shouty, field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                }
                writeln!(out)?;
            }

            // Write accessor (for RW single-bit fields in RW register)
            if matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width == 1
            {
                if supports_atomic {
                    // Use atomic set/clr methods (generated on the register struct)
                    writeln!(out, "    /// Set {} bit atomically (RW)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("set", &field.name)
                    )?;
                    writeln!(out, "        self.set_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;

                    writeln!(out, "    /// Clear {} bit atomically (RW)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("clr", &field.name)
                    )?;
                    writeln!(out, "        self.clr_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                } else {
                    // No atomic support - use read-modify-write
                    writeln!(out, "    /// Set {} bit (RW) - NOT atomic", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("set", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r | Self::{}_MASK);",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;

                    writeln!(out, "    /// Clear {} bit (RW) - NOT atomic", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("clr", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r & !Self::{}_MASK);",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                }
            }

            // Write accessor (for RW multi-bit fields in RW register) - use modify
            if matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width > 1
            {
                writeln!(out, "    /// Write **{}** {} (RW)", field.name, bit_range)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self, value: u32) {{",
                    prefixed_method_name("write", &field.name)
                )?;
                writeln!(
                    out,
                    "        self.value.modify_bits(|r| (r & !Self::{}_MASK) | ((value << Self::{}_OFFSET) & Self::{}_MASK));",
                    field_shouty, field_shouty, field_shouty
                )?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }

            // WriteOnly fields - trigger method (single bit)
            // Use set_bits to atomically set the bit without clearing other bits (like EN)
            // For peripherals without atomic support, use modify_bits
            if matches!(field.access, Access::WriteOnly) && width == 1 {
                writeln!(out, "    /// Trigger **{}** (WO)", field.name)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self) {{",
                    prefixed_method_name("trigger", &field.name)
                )?;
                if supports_atomic {
                    writeln!(out, "        self.set_bits(Self::{}_MASK);", field_shouty)?;
                } else if matches!(reg.access, Access::WriteOnly) {
                    // WO register: cannot read, use direct write
                    writeln!(
                        out,
                        "        self.value.write_bits(Self::{}_MASK);",
                        field_shouty
                    )?;
                } else {
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r | Self::{}_MASK);",
                        field_shouty
                    )?;
                }
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }

            // WriteOnly fields - write method (multi-bit)
            if matches!(field.access, Access::WriteOnly) && width > 1 {
                writeln!(out, "    /// Write **{}** {} (WO)", field.name, bit_range)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self, value: u32) {{",
                    prefixed_method_name("write", &field.name)
                )?;
                writeln!(
                    out,
                    "        self.value.write_bits((value << Self::{}_OFFSET) & Self::{}_MASK);",
                    field_shouty, field_shouty
                )?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }
        }
    }

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the value struct for batch read/write operations
/// This allows reading/writing the entire register once and manipulating fields in memory
fn write_register_value_struct(out: &mut String, reg: &Register, struct_name: &str) -> Result<()> {
    let val_name = format!("{}Val", struct_name);

    // Only generate for registers with fields
    if reg.fields.is_empty() {
        return Ok(());
    }

    writeln!(
        out,
        "/// {} value type for batch read/modify/write operations",
        reg.name
    )?;
    writeln!(out, "///")?;
    writeln!(
        out,
        "/// Use `{}.read_val()` to read the register into this struct,",
        struct_name.to_lowercase()
    )?;
    writeln!(
        out,
        "/// modify fields with builder methods, then write back with `{}.write_val(val)`.",
        struct_name.to_lowercase()
    )?;
    writeln!(out, "#[derive(Clone, Copy)]")?;
    writeln!(out, "pub struct {} {{", val_name)?;
    writeln!(out, "    bits: u32,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", val_name)?;

    // Constructor from raw bits
    writeln!(out, "    /// Create from raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn from_bits(bits: u32) -> Self {{")?;
    writeln!(out, "        Self {{ bits }}")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Get raw bits
    writeln!(out, "    /// Get the raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn bits(&self) -> u32 {{")?;
    writeln!(out, "        self.bits")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Default (reset value or zero)
    // Check if any field is named "reset" to avoid method name collision
    let has_reset_field = reg.fields.iter().any(|f| f.name.to_lowercase() == "reset");

    if let Some(reset_val) = reg.reset_value
        && !has_reset_field
    {
        writeln!(out, "    /// Create with reset value")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub const fn reset_val() -> Self {{")?;
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // Field reader methods
    for field in &reg.fields {
        let field_snake = safe_snake_case(&field.name);
        let field_shouty = safe_shouty_snake_case(&field.name);
        let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
        let width = field.width();

        if is_readable {
            writeln!(
                out,
                "    /// Read **{}** `[{}:{}]`",
                field.name, field.msb, field.lsb
            )?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(out, "    pub const fn {}(&self) -> bool {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) != 0",
                    struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(out, "    pub const fn {}(&self) -> u32 {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) >> {}::{}_OFFSET",
                    struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    // Field builder methods (for writable fields)
    for field in &reg.fields {
        let field_shouty = safe_shouty_snake_case(&field.name);
        let is_writable = matches!(field.access, Access::WriteOnly | Access::ReadWrite);
        let width = field.width();
        let with_method = prefixed_method_name("with", &field.name);

        if is_writable {
            writeln!(
                out,
                "    /// Set **{}** `[{}:{}]`",
                field.name, field.msb, field.lsb
            )?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: bool) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(out, "            bits: if value {{")?;
                writeln!(
                    out,
                    "                self.bits | {}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }} else {{")?;
                writeln!(
                    out,
                    "                self.bits & !{}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: u32) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(
                    out,
                    "            bits: (self.bits & !{}::{}_MASK) | ((value << {}::{}_OFFSET) & {}::{}_MASK)",
                    struct_name, field_shouty, struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Default impl
    writeln!(out, "impl Default for {} {{", val_name)?;
    writeln!(out, "    fn default() -> Self {{")?;
    if reg.reset_value.is_some() && !has_reset_field {
        writeln!(out, "        Self::reset_val()")?;
    } else if let Some(reset_val) = reg.reset_value {
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
    } else {
        writeln!(out, "        Self {{ bits: 0 }}")?;
    }
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Add read_val/write_val/modify_val methods that use XxxVal to the register struct
    writeln!(out, "impl {} {{", struct_name)?;

    if matches!(reg.access, Access::ReadOnly | Access::ReadWrite) {
        writeln!(out, "    /// Read the register into a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn read_val(&self) -> {} {{", val_name)?;
        writeln!(
            out,
            "        {}::from_bits(self.value.read_bits())",
            val_name
        )?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::WriteOnly | Access::ReadWrite) {
        writeln!(out, "    /// Write a value struct to the register")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn write_val(&self, val: {}) {{", val_name)?;
        writeln!(out, "        self.value.write_bits(val.bits())")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::ReadWrite) {
        writeln!(out, "    /// Modify the register using a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(
            out,
            "    pub fn modify_val<F: FnOnce({}) -> {}>(&self, f: F) {{",
            val_name, val_name
        )?;
        writeln!(out, "        self.write_val(f(self.read_val()))")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the RegisterBlock struct
fn write_register_block(
    out: &mut String,
    _periph_name: &str,
    registers: &[Register],
    type_names: &[String],
) -> Result<()> {
    writeln!(out, "/// Register block")?;
    writeln!(out, "#[repr(C)]")?;
    writeln!(out, "pub struct RegisterBlock {{")?;

    // `expand_registers` already sorted the list by address offset.
    let mut prev_end: u32 = 0;
    let mut padding_idx = 0;

    for (i, reg) in registers.iter().enumerate() {
        if reg.offset > prev_end {
            let gap = reg.offset - prev_end;
            writeln!(out, "    _reserved{}: [u8; {}],", padding_idx, gap)?;
            padding_idx += 1;
        }

        // Array element 0 and a literal register can share a name, so derive
        // the field name from the already-unique type name.
        let type_name = &type_names[i];
        let field_name = safe_snake_case(type_name);

        writeln!(out, "    /// {:#06x}", reg.offset)?;
        if !reg.description.is_empty() {
            for line in clean_desc(&reg.description).lines() {
                writeln!(out, "    /// {}", line)?;
            }
        }
        writeln!(out, "    pub {}: {},", field_name, type_name)?;

        prev_end = reg.offset + 4;
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn escape_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

/// Prefix an identifier that would otherwise start with a digit.
fn validate_ident(s: String) -> String {
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", s)
    } else {
        s
    }
}

/// Safe snake_case for standalone identifiers (e.g., field reads)
fn safe_snake_case(name: &str) -> String {
    let snake = validate_ident(name.to_snake_case());
    escape_keyword(&snake)
}

/// Safe Shouty_Snake_Case for constants and enum variants.
fn safe_shouty_snake_case(name: &str) -> String {
    validate_ident(name.to_shouty_snake_case())
}

/// Safe UpperCamelCase for type names.
fn safe_type_name(name: &str) -> String {
    validate_ident(name.to_upper_camel_case())
}

/// Build prefixed method name, handling keyword escaping properly
/// For `write_match`, we want `write_match_` to avoid the raw keyword issue
fn prefixed_method_name(prefix: &str, field_name: &str) -> String {
    let base = field_name.to_snake_case();
    let combined = format!("{}_{}", prefix, base);
    // If the base name is a keyword, append underscore rather than using r#
    if RUST_KEYWORDS.contains(&base.as_str()) {
        format!("{}_", combined)
    } else {
        combined
    }
}

fn write_header(out: &mut String, periph: &Peripheral, supports_atomic: bool) -> Result<()> {
    writeln!(out, "//! {}", periph.name)?;
    if !periph.description.is_empty() {
        // Only include first few meaningful lines of description
        // Skip configuration constant lists and boilerplate
        let desc = clean_desc(&periph.description);
        let meaningful_lines: Vec<&str> = desc
            .lines()
            .filter(|line| {
                // Skip lines that indicate a list of constants follows
                !line.contains("List of configuration") && !line.contains("you may see references")
            })
            .take_while(|line| {
                // Stop at configuration constant lists (lines with lots of dots or equals)
                !line.contains("....") && !line.contains(" = ")
            })
            .take(3) // Max 3 lines
            .collect();
        for line in meaningful_lines {
            writeln!(out, "//! {}", line)?;
        }
    }
    if supports_atomic {
        writeln!(out, "//!")?;
        writeln!(
            out,
            "//! This peripheral supports atomic SET/CLR/XOR register aliases."
        )?;
    }
    writeln!(out)?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out)?;
    if let Some(addr) = periph.base_address {
        writeln!(out, "pub const BASE: u32 = {:#010x};", addr)?;
        writeln!(out)?;
    }
    Ok(())
}

/// Trim trailing blank line from output buffer.
/// Many code-gen loops emit `writeln!(out)?;` after each item for spacing,
/// which leaves an unwanted blank line right before the closing `}`.
fn trim_trailing_blank_line(out: &mut String) {
    if out.ends_with("\n\n") {
        out.truncate(out.len() - 1);
    }
}

/// Clean up description: normalize whitespace within each line, preserve original line breaks
fn clean_desc(s: &str) -> String {
    s.lines()
        .map(|line| {
            // Normalize whitespace within each line
            line.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Get the first line of description (for short summaries in mod.rs)
fn short_desc(s: &str) -> String {
    clean_desc(s).lines().next().unwrap_or("").to_string()
}
