//! Machine-mode interrupt control, the CLIC and interrupt dispatch.
//!
//! The ESP32-P4 has no PLIC; it uses an Espressif CLIC instead (TRM chapter 1),
//! with a memory-mapped control array. Each interrupt source owns one **word**,
//! but its fields are single bytes inside it:
//!
//! ```text
//!   0x2080_0000  mcliccfg     [3:1] MNLBITS: how many CTL bits encode the level
//!   0x2080_0004  clicinfo     NUM_INTERRUPTS = 48, CLICINTCTLBITS = 4
//!   0x2080_0008  mthreshold   machine-mode threshold (CSR `mintthresh` mirrors it)
//!
//!   0x2080_1000 + 4*i + 0   clicintip[i]    pending
//!   0x2080_1000 + 4*i + 1   clicintie[i]    enable
//!   0x2080_1000 + 4*i + 2   clicintattr[i]  [2:1] TRIG, [0] SHV
//!   0x2080_1000 + 4*i + 3   clicintctl[i]   level in the high bits, priority below
//! ```
//!
//! Interrupt ids 0..15 belong to the CLINT (software and timer interrupts); the
//! 32 lines the peripheral interrupt matrix can drive start at id 16. When one
//! of them fires, `mcause` reads `0x8000_0000 | id`, so the CPU interrupt line
//! is `(mcause & 0xfff) - 16`.
//!
//! Handlers are plain `fn()`s kept in a table indexed by CPU interrupt line. The
//! trap entry runs with `MIE = 0`, so a handler cannot be preempted by another
//! interrupt on this core — nesting is not supported.

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::register::{mintthresh, mstatus};

/// Base of the CLIC's own configuration registers.
const CLIC_CONFIG: usize = 0x2080_0000;
/// Base of the per-interrupt byte fields.
const CLIC_INTS: usize = 0x2080_1000;

/// First CLIC id that belongs to a CPU interrupt line.
///
/// Lower ids are the CLINT's software and timer interrupts.
pub const FIRST_EXT_INTR_ID: u32 = 16;

/// Number of CPU interrupt lines the interrupt matrix can drive.
pub const CPU_INTERRUPT_COUNT: u32 = 32;

/// Level bits used by the CTL byte, matching the `mcliccfg.MNLBITS` value set in
/// [`init`]: three bits of level, five bits of intra-level priority.
const MNLBITS: u32 = 3;

/// Priority given to every line this runtime unmasks: level 1 with the highest
/// intra-level priority, so it sits above the threshold of 0.
const PRIORITY_BYTE: u8 = 0x1f;

// The CLIC ids used by the drivers, spelled out as addresses so a typo in the
// arithmetic below is a compile error rather than a silent write to the wrong
// interrupt: id = cpu_int + 16, word = base + 4 * id, field = word + {ip,ie,attr,ctl}.
const _: () = assert!(CLIC_INTS + (19 + FIRST_EXT_INTR_ID as usize) * 4 + 3 == 0x2080_108f);
const _: () = assert!(CLIC_INTS + (20 + FIRST_EXT_INTR_ID as usize) * 4 + 3 == 0x2080_1093);

static HANDLERS: [AtomicUsize; CPU_INTERRUPT_COUNT as usize] =
    [const { AtomicUsize::new(0) }; CPU_INTERRUPT_COUNT as usize];

/// Byte address of `clicintXX[i]` for `field` 0..=3.
#[inline]
fn int_byte(cpu_int: u32, field: usize) -> *mut u8 {
    (CLIC_INTS + (cpu_int + FIRST_EXT_INTR_ID) as usize * 4 + field) as *mut u8
}

/// Number of interrupt sources the CLIC reports.
///
/// A sanity check on the base address: this chip returns 48.
#[inline]
pub fn interrupt_count() -> u32 {
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::read_volatile((CLIC_CONFIG + 0x4) as *const u32) & 0x1fff }
}

/// Configure the CLIC for direct (non-vectored) delivery.
///
/// Every interrupt then enters through the single trap entry, which dispatches
/// on `mcause` — no vector table in RAM.
///
/// Called once from `_start`.
pub fn init() {
    // SAFETY: fixed MMIO addresses, and no interrupt is enabled yet.
    unsafe {
        let cfg = CLIC_CONFIG as *mut u32;
        let value = core::ptr::read_volatile(cfg);
        core::ptr::write_volatile(cfg, (value & !(0b111 << 1)) | (MNLBITS << 1));
    }
    // SAFETY: the reset value is 0, but a previous program may have raised it.
    unsafe { mintthresh::write(0) };
}

/// Disable machine interrupts and return the previous `MIE` state.
#[inline(always)]
pub fn disable() -> bool {
    let was_enabled = mstatus::mie();
    mstatus::clear_mie();
    was_enabled
}

/// Enable machine interrupts.
///
/// # Safety
///
/// The trap handler and every unmasked interrupt source must be ready to run.
#[inline(always)]
pub unsafe fn enable() {
    unsafe { mstatus::set_mie() }
}

/// Restore the `MIE` state returned by [`disable`].
///
/// # Safety
///
/// Must be called with the same nesting discipline as [`disable`].
#[inline(always)]
pub unsafe fn restore(was_enabled: bool) {
    if was_enabled {
        unsafe { enable() }
    }
}

/// Returns whether machine interrupts are currently enabled.
#[inline(always)]
pub fn is_enabled() -> bool {
    mstatus::mie()
}

/// Run `f` with machine interrupts disabled.
#[inline(always)]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let was_enabled = disable();
    let result = f();
    unsafe { restore(was_enabled) };
    result
}

/// Register the handler for CPU interrupt line `cpu_int`.
///
/// Call [`unmask`] afterwards, and [`enable`] once, to actually take the
/// interrupt.
pub fn set_handler(cpu_int: u32, handler: fn()) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    HANDLERS[cpu_int as usize].store(handler as usize, Ordering::Release);
}

/// Unmask CPU interrupt line `cpu_int`.
///
/// The line is level-triggered and not hardware-vectored, so it enters through
/// the common trap entry.
///
/// # Safety
///
/// A handler must be registered for this line, and firing it immediately must be
/// safe — that is, the peripheral's interrupt source is either idle or already
/// routed to this line.
pub unsafe fn unmask(cpu_int: u32) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");

    // SAFETY: fixed MMIO addresses; the caller vouched for the consequences.
    unsafe {
        // Level-triggered (`TRIG = 0b00`) and direct mode (`SHV = 0`).
        core::ptr::write_volatile(int_byte(cpu_int, 2), 0);
        core::ptr::write_volatile(int_byte(cpu_int, 3), PRIORITY_BYTE);
        core::ptr::write_volatile(int_byte(cpu_int, 1), 1);
    }
}

/// Mask CPU interrupt line `cpu_int`.
pub fn mask(cpu_int: u32) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::write_volatile(int_byte(cpu_int, 1), 0) };
}

/// Returns whether `cpu_int` is currently unmasked.
#[inline]
pub fn is_unmasked(cpu_int: u32) -> bool {
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::read_volatile(int_byte(cpu_int, 1)) & 1 != 0 }
}

/// Returns whether `cpu_int` is pending in the CLIC.
#[inline]
pub fn is_pending(cpu_int: u32) -> bool {
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::read_volatile(int_byte(cpu_int, 0)) & 1 != 0 }
}

/// Run the handler registered for `cpu_int`.
///
/// Called from the trap entry, which runs with interrupts already disabled.
pub(crate) fn dispatch(cpu_int: u32) {
    if cpu_int >= CPU_INTERRUPT_COUNT {
        return;
    }

    let handler = HANDLERS[cpu_int as usize].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `set_handler` only ever stores a `fn()`.
        let handler: fn() = unsafe { core::mem::transmute(handler) };
        handler();
    }
}

/// Decode `mcause` into `(is_interrupt, code)`.
#[inline]
pub(crate) fn decode_cause(cause: usize) -> (bool, usize) {
    (cause & (1 << 31) != 0, cause & 0x0fff)
}

/// Stop the core until an interrupt is pending.
///
/// `wfi` also wakes for an unmasked interrupt that is held off by a raised
/// threshold, so it doubles as the sleep primitive of a poll loop.
#[inline]
pub fn sleep() {
    // SAFETY: `wfi` only parks the core until an interrupt is pending.
    unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
}
