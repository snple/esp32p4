//! Machine-mode interrupt control, the CLIC and interrupt dispatch.
//!
//! The ESP32-P4 has no PLIC; it uses an Espressif CLIC instead (TRM chapter 1),
//! with a memory-mapped control array. Each interrupt source owns one **word**,
//! but its fields are single bytes inside it:
//!
//! ```text
//!   0x2080_0000  mcliccfg     [3:1] MNLBITS: how many CTL bits encode the level
//!   0x2080_0004  clicinfo     NUM_INTERRUPTS = 48, CLICINTCTLBITS = 4
//!   0x2080_0008  mthreshold   machine-mode threshold (`mintthresh` mirror)
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
//!
//! # Levels
//!
//! `mcliccfg.MNLBITS` is set to 3, so `clicintctl[i]` programs an 8-bit level
//! byte: the three high bits are the level (0..=7) and the five low bits are
//! hardwired to 1. Level 0 therefore reads as `0x1f`, level 7 as `0xff`.
//!
//! The hardware compares that byte with `max(mintthresh.TH, mintstatus.MIL)`
//! and takes the interrupt only when the byte is *greater*. Levels only order
//! interrupts that are pending at the same time: preemption would additionally
//! require re-enabling `MIE` inside the handler, which the runtime deliberately
//! does not do.
//!
//! # `mintthresh` and `mintstatus` are unusable
//!
//! Both CLIC CSRs (`0x347` and `0xFB1`) raise an illegal-instruction exception
//! on the ESP32-P4 (measured on chip revision v1.3), so the runtime never
//! touches them: the threshold keeps its reset value of 0 — which accepts every
//! level, exactly what a non-nesting dispatcher wants — and the "inside a
//! handler" state that `mintstatus.MIL` would report is tracked in software
//! instead (see [`in_interrupt`]). The threshold also has a memory-mapped mirror
//! at `CLIC + 0x8`, which is the way to raise it if that ever becomes necessary.

use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

use crate::register::mstatus;

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

/// Level bits used by the CTL byte, matching the `mclicfg.MNLBITS` value set in
/// [`init`]. With three bits every level field is significant and the remaining
/// five bits of the byte are hardwired to 1.
const MNLBITS: u32 = 3;

/// Highest level `set_level` accepts (`MNLBITS = 3`).
pub const MAX_LEVEL: u8 = 7;

/// Low bits of the level byte: hardwired to 1 in hardware.
const LEVEL_BYTE_BASE: u8 = 0x1f;

/// Encode a level (0..=7) as the byte the hardware compares.
#[inline]
pub const fn level_byte(level: u8) -> u8 {
    ((level & MAX_LEVEL) << 5) | LEVEL_BYTE_BASE
}

/// The level encoded in a `clicintctl` byte.
#[inline]
pub const fn level_of_byte(byte: u8) -> u8 {
    (byte >> 5) & MAX_LEVEL
}

// The CLIC ids used by the drivers, spelled out as addresses so a typo in the
// arithmetic below is a compile error rather than a silent write to the wrong
// interrupt: id = cpu_int + 16, word = base + 4 * id, field = word + {ip,ie,attr,ctl}.
const _: () = assert!(CLIC_INTS + (19 + FIRST_EXT_INTR_ID as usize) * 4 + 3 == 0x2080_108f);
const _: () = assert!(CLIC_INTS + (20 + FIRST_EXT_INTR_ID as usize) * 4 + 3 == 0x2080_1093);

static HANDLERS: [AtomicUsize; CPU_INTERRUPT_COUNT as usize] =
    [const { AtomicUsize::new(0) }; CPU_INTERRUPT_COUNT as usize];

/// Set while a registered handler runs, i.e. what `mintstatus.MIL != 0` reports
/// on a controller that implements it.
static IN_HANDLER: AtomicBool = AtomicBool::new(false);
/// Level of the handler that is running, or 0 outside a handler.
static HANDLER_LEVEL: AtomicU8 = AtomicU8::new(0);

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
///
/// This deliberately leaves the interrupt threshold alone: `mintthresh` (CSR
/// `0x347`) is not implemented on this silicon, and its reset value of 0 already
/// accepts every level (see the module documentation).
pub fn init() {
    // SAFETY: fixed MMIO addresses, and no interrupt is enabled yet.
    unsafe {
        let cfg = CLIC_CONFIG as *mut u32;
        let value = core::ptr::read_volatile(cfg);
        core::ptr::write_volatile(cfg, (value & !(0b111 << 1)) | (MNLBITS << 1));
    }
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

/// Remove the handler of CPU interrupt line `cpu_int`.
///
/// The line keeps its CLIC configuration; a spurious interrupt dispatches to
/// nothing. Mask the line first with [`mask`].
pub fn clear_handler(cpu_int: u32) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    HANDLERS[cpu_int as usize].store(0, Ordering::Release);
}

/// Unmask CPU interrupt line `cpu_int`.
///
/// The line is level-triggered and not hardware-vectored, so it enters through
/// the common trap entry. Its level is whatever [`set_level`] programmed
/// (level 0 after reset).
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
        // `clicintattr`: MODE (bits 7:6) is hardwired to machine mode
        // (`mcliccfg.NMBITS = 0`), but it is written explicitly to keep the
        // byte at its documented value; `TRIG = 0b00` selects a positive level
        // and `SHV = 0` the shared, non-vectored trap entry.
        core::ptr::write_volatile(int_byte(cpu_int, 2), 0b11 << 6);
        core::ptr::write_volatile(int_byte(cpu_int, 1), 1);
    }
}

/// Program the CLIC level of CPU interrupt line `cpu_int`.
///
/// Levels order interrupts that are pending at the same time; a higher level is
/// taken first. They do not cause nesting, because the trap entry runs with
/// `MIE = 0` (see the module documentation).
///
/// Must be called before [`unmask`] to take effect on the first interrupt.
pub fn set_level(cpu_int: u32, level: u8) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    assert!(level <= MAX_LEVEL, "level out of range");
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::write_volatile(int_byte(cpu_int, 3), level_byte(level)) };
}

/// The CLIC level programmed for `cpu_int`.
#[inline]
pub fn level(cpu_int: u32) -> u8 {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    // SAFETY: fixed MMIO address.
    let byte = unsafe { core::ptr::read_volatile(int_byte(cpu_int, 3)) };
    level_of_byte(byte)
}

/// Set the CLIC pending bit of `cpu_int`.
///
/// The runtime configures lines as level-triggered, so the bit follows the
/// source: a write only sticks while the source is asserting. Useful to
/// re-trigger a line whose source is still active.
pub fn pend(cpu_int: u32) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::write_volatile(int_byte(cpu_int, 0), 1) };
}

/// Clear the CLIC pending bit of `cpu_int`.
///
/// A level-triggered source that is still asserting sets it again immediately.
pub fn unpend(cpu_int: u32) {
    assert!(cpu_int < CPU_INTERRUPT_COUNT, "no such CPU interrupt line");
    // SAFETY: fixed MMIO address.
    unsafe { core::ptr::write_volatile(int_byte(cpu_int, 0), 0) };
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

    let previous_in_handler = IN_HANDLER.swap(true, Ordering::Relaxed);
    let previous_level = HANDLER_LEVEL.swap(level(cpu_int), Ordering::Relaxed);

    let handler = HANDLERS[cpu_int as usize].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `set_handler` only ever stores a `fn()`.
        let handler: fn() = unsafe { core::mem::transmute(handler) };
        handler();
    }

    HANDLER_LEVEL.store(previous_level, Ordering::Relaxed);
    IN_HANDLER.store(previous_in_handler, Ordering::Relaxed);
}

/// Decode `mcause` into `(is_interrupt, code)`.
#[inline]
pub(crate) fn decode_cause(cause: usize) -> (bool, usize) {
    (cause & (1 << 31) != 0, cause & 0x0fff)
}

/// Whether the core is currently executing an interrupt handler.
///
/// Tracked by [`dispatch`] rather than read from `mintstatus`, which is not
/// implemented on this chip (see the module documentation).
#[inline]
pub fn in_interrupt() -> bool {
    IN_HANDLER.load(Ordering::Relaxed)
}

/// The CLIC level of the handler being executed, or 0 outside a handler.
#[inline]
pub fn current_level() -> u8 {
    HANDLER_LEVEL.load(Ordering::Relaxed)
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
