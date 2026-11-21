//! Time and duration value types.
//!
//! Nothing here reads a clock: [`Duration`] and [`Instant`] are plain values, so
//! they cost nothing and can be used in any context. Reading the time is a
//! property of a timer you own — see [`DelayUs`](crate::delay::DelayUs) for the
//! waiting side and [`Counter`](crate::systimer::Counter) for the reading side.
//!
//! Both types are counted in microseconds. Converting to a timer's ticks needs
//! that timer's rate, so it happens in the timer, through
//! [`Duration::as_ticks`].

use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// A span of time, counted in microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration {
    micros: u64,
}

impl Duration {
    /// No time at all.
    pub const ZERO: Duration = Duration { micros: 0 };

    /// The longest representable span.
    pub const MAX: Duration = Duration { micros: u64::MAX };

    /// A span of `micros` microseconds.
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// A span of `millis` milliseconds.
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            micros: millis.saturating_mul(1_000),
        }
    }

    /// A span of `secs` seconds.
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        Self {
            micros: secs.saturating_mul(1_000_000),
        }
    }

    /// The period of a `hz` hertz signal.
    ///
    /// # Panics
    ///
    /// Panics if `hz` is zero.
    #[inline]
    pub const fn from_hz(hz: u32) -> Self {
        assert!(hz > 0, "frequency must be non-zero");
        Self {
            micros: 1_000_000 / hz as u64,
        }
    }

    /// The span in whole microseconds.
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// The span in whole milliseconds, rounded down.
    #[inline]
    pub const fn as_millis(&self) -> u64 {
        self.micros / 1_000
    }

    /// The span in whole seconds, rounded down.
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.micros / 1_000_000
    }

    /// The span as a count of ticks of a counter running at `freq_hz`.
    ///
    /// Rounded down, so the count never exceeds the span.
    ///
    /// # Panics
    ///
    /// Panics if `freq_hz` is zero.
    #[inline]
    pub const fn as_ticks(&self, freq_hz: u32) -> u64 {
        assert!(freq_hz > 0, "frequency must be non-zero");
        self.micros * freq_hz as u64 / 1_000_000
    }

    /// Whether the span is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.micros == 0
    }

    /// `self + rhs`, or `None` if the sum does not fit in 64 bits.
    #[inline]
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_add(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// `self - rhs`, or `None` if `rhs` is the longer span.
    #[inline]
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_sub(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// `self + rhs`, saturating at the longest span.
    #[inline]
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros.saturating_add(rhs.micros),
        }
    }

    /// `self - rhs`, saturating at zero.
    #[inline]
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros.saturating_sub(rhs.micros),
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    #[inline]
    fn add(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros + rhs.micros,
        }
    }
}

impl AddAssign for Duration {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.micros += rhs.micros;
    }
}

impl Sub for Duration {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros - rhs.micros,
        }
    }
}

impl SubAssign for Duration {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.micros -= rhs.micros;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn mul(self, rhs: u32) -> Duration {
        Duration {
            micros: self.micros.saturating_mul(rhs as u64),
        }
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn div(self, rhs: u32) -> Duration {
        Duration {
            micros: self.micros / rhs as u64,
        }
    }
}

/// A point in time, as microseconds since some epoch.
///
/// The epoch is whatever the counter that produced it started from, so an
/// `Instant` is only comparable with instants from the same counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Instant {
    micros: u64,
}

impl Instant {
    /// The epoch itself.
    pub const EPOCH: Instant = Instant { micros: 0 };

    /// A point `micros` microseconds after the epoch.
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// The point in microseconds since the epoch.
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// The span between `earlier` and `self`, zero if `earlier` is later.
    #[inline]
    pub const fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            micros: self.micros.saturating_sub(earlier.micros),
        }
    }

    /// The span between `earlier` and `self`, or `None` if `earlier` is later.
    #[inline]
    pub const fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        match self.micros.checked_sub(earlier.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// Whether `self` is later than `other`.
    #[inline]
    pub const fn is_after(&self, other: Instant) -> bool {
        self.micros > other.micros
    }

    /// A point `duration` after `self`, or `None` on overflow.
    #[inline]
    pub const fn checked_add(self, duration: Duration) -> Option<Instant> {
        match self.micros.checked_add(duration.micros) {
            Some(micros) => Some(Instant { micros }),
            None => None,
        }
    }

    /// A point `duration` before `self`, or `None` on underflow.
    #[inline]
    pub const fn checked_sub(self, duration: Duration) -> Option<Instant> {
        match self.micros.checked_sub(duration.micros) {
            Some(micros) => Some(Instant { micros }),
            None => None,
        }
    }

    /// A point `duration` after `self`, saturating at the end of the range.
    #[inline]
    pub const fn saturating_add(self, duration: Duration) -> Instant {
        Instant {
            micros: self.micros.saturating_add(duration.micros),
        }
    }

    /// A point `duration` before `self`, saturating at the epoch.
    #[inline]
    pub const fn saturating_sub(self, duration: Duration) -> Instant {
        Instant {
            micros: self.micros.saturating_sub(duration.micros),
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros + rhs.micros,
        }
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.micros += rhs.micros;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros - rhs.micros,
        }
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.micros -= rhs.micros;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}
