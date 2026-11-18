//! Machine-mode interrupt control.

use crate::register::mstatus;

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
/// The caller must ensure that the trap handler and every interrupt source
/// are ready to run.
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
