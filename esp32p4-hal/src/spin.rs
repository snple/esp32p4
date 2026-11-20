//! Bounded busy-waits.
//!
//! Every register poll in the HAL has an upper bound. Without one, a missing
//! clock or a mis-routed pin turns a driver call into an infinite loop with no
//! diagnostic; with one, a stuck peripheral either panics at the call site or
//! is reported as an error, depending on which helper is used:
//!
//! - [`spin_wait`] / [`spin_wait_max`] panic when the bound is reached. They
//!   suit blocking calls whose failure cannot be handled sensibly.
//! - [`try_spin_wait`] returns `false` instead, for polling APIs that report
//!   the condition through their own `Result`.
//!
//! The bound counts iterations, not time, so it scales with the CPU clock. It
//! is deliberately loose: it exists to catch "this will never finish", not to
//! implement protocol timeouts. Drivers that need a real time limit compare
//! against [`crate::delay::micros`].

/// Iterations a poll may take before it is treated as stuck.
///
/// Roughly 2.6 ms at 400 MHz per million iterations, so the default allows
/// tens of milliseconds of waiting.
pub const SPIN_TIMEOUT: u32 = 1 << 20;

/// Polls `cond` until it returns `true`.
///
/// # Panics
///
/// Panics if `cond` is still `false` after [`SPIN_TIMEOUT`] iterations. `what`
/// names the condition in the panic message.
pub fn spin_wait(what: &str, cond: impl FnMut() -> bool) {
    spin_wait_max(SPIN_TIMEOUT, what, cond);
}

/// [`spin_wait`] with an explicit iteration bound.
///
/// # Panics
///
/// Panics if `cond` is still `false` after `max` iterations.
pub fn spin_wait_max(max: u32, what: &str, mut cond: impl FnMut() -> bool) {
    let mut remaining = max;
    while !cond() {
        if remaining == 0 {
            panic!("timed out waiting for {what}");
        }
        remaining -= 1;
        core::hint::spin_loop();
    }
}

/// Polls `cond` until it returns `true` or [`SPIN_TIMEOUT`] iterations expire.
#[inline]
pub fn try_spin_wait(mut cond: impl FnMut() -> bool) -> bool {
    let mut remaining = SPIN_TIMEOUT;
    loop {
        if cond() {
            return true;
        }
        if remaining == 0 {
            return false;
        }
        remaining -= 1;
        core::hint::spin_loop();
    }
}
