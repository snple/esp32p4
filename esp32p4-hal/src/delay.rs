//! Delay abstraction trait.
//!
//! This module is only the trait: waiting is a capability of a counter, so every
//! [`DelayUs`] implementor is a timer the caller owns. Nothing here touches
//! hardware, and there is no global delay source.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::delay::DelayUs;
//!
//! fn blink(delay: &impl DelayUs) {
//!     delay.delay_ms(500);
//! }
//! ```

/// Microsecond delay trait.
///
/// Upper-layer code stays independent of the delay source by taking
/// `&impl DelayUs` (or `&dyn DelayUs`) instead of a concrete timer.
///
/// The tick rate is a property of the implementor, which is why reading the clock
/// and converting microseconds to ticks happen there rather than here.
pub trait DelayUs {
    /// Delays for the given number of microseconds (busy-wait).
    fn delay_us(&self, us: u32);

    /// Delays for the given number of milliseconds.
    ///
    /// The default implementation calls `delay_us(ms * 1_000)`; override it for
    /// better efficiency if needed.
    #[inline]
    fn delay_ms(&self, ms: u32) {
        self.delay_us(ms.saturating_mul(1_000));
    }

    /// Delays for the given number of seconds.
    ///
    /// The default implementation loops over `delay_ms(1_000)` so that a long
    /// wait cannot overflow the millisecond arithmetic.
    #[inline]
    fn delay_secs(&self, secs: u32) {
        for _ in 0..secs {
            self.delay_ms(1_000);
        }
    }
}

impl<T: DelayUs> DelayUs for crate::Steal<T> {
    #[inline]
    fn delay_us(&self, us: u32) {
        self.0.delay_us(us)
    }
}

/// Busy-waits `us` microseconds on this core's machine timer.
///
/// The escape hatch for code that has to pause while holding no timer of its own —
/// an analog power-up sequence, a PHY settling delay, a `main` loop that only needs
/// to blink. It only *reads* the CLINT counter: it takes no comparator, claims no
/// interrupt and writes nothing, so it cannot disturb the code that owns the timer.
///
/// Prefer [`DelayUs`] on a timer you own when you have one; this is the
/// zero-setup form. The rate is whatever
/// [`MachineTimer::cpu_hz`](crate::machine_timer::MachineTimer::cpu_hz) reports, so
/// the wait is approximate — good for "at least this long", not for a deadline.
///
/// The rate comes from [`MachineTimer::cpu_hz`](crate::machine_timer::MachineTimer::cpu_hz),
/// which starts at a conservative boot rate and follows whatever the program
/// records, so this works before anything has taken the clock tree. Applications
/// should still prefer [`DelayUs`] on a timer they own; this exists so a driver
/// does not have to thread a handle through every register sequence.
#[inline]
pub fn delay_us(us: u32) {
    // SAFETY: reading the shared counter aliases no write, and the bootloader
    // leaves it running.
    unsafe { crate::machine_timer::MachineTimer::steal() }.delay_us(us);
}

/// Busy-waits `ms` milliseconds. See [`delay_us`].
#[inline]
pub fn delay_ms(ms: u32) {
    unsafe { crate::machine_timer::MachineTimer::steal() }.delay_ms(ms);
}

/// Microseconds since the machine timer's counter started.
///
/// Handy as a `defmt` timestamp: `defmt::timestamp!("{=u64:us}", micros());`.
///
/// This reads this core's CLINT counter, the same one [`delay_us`] waits on, so it
/// costs one 64-bit read and works from the first line of the program. The rate it
/// divides by is [`MachineTimer::cpu_hz`](crate::machine_timer::MachineTimer::cpu_hz),
/// which starts at a conservative boot value and follows what the program records,
/// so the timestamp is accurate to about one part in ten until
/// [`MachineTimer::init`](crate::machine_timer::MachineTimer::init) is called with
/// the measured rate.
#[inline]
pub fn micros() -> u64 {
    // SAFETY: reading the shared counter aliases no write.
    unsafe { crate::machine_timer::MachineTimer::steal() }.micros()
}
