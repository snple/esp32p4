//! Timekeeping on the system timer's 16 MHz tick.
//!
//! `SYSTIMER` unit 0 divides `XTAL_CLK` (40 MHz) by 2.5, so it ticks 16 million
//! times a second and wraps after about 8.9 years at 52 bits. That makes it the
//! monotonic clock of the chip: [`Instant`] is a counter value, [`Duration`] is
//! a number of ticks, and one tick is exactly 62.5 ns.
//!
//! The counter is left free-running. Nothing here configures the comparator, so
//! this is independent of [`Delay`](crate::delay::Delay), which only borrows
//! comparator 0 to sleep.
//!
//! ```ignore
//! use esp32p4_hal::time::{Duration, Instant};
//!
//! let start = Instant::now();
//! // ...
//! let elapsed = start.elapsed();
//! assert!(elapsed < Duration::from_millis(10));
//! ```

use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use core::sync::atomic::{AtomicBool, Ordering};

use esp32p4_pac::SYSTIMER;

/// System timer ticks per second.
pub const TICK_HZ: u64 = 16_000_000;

/// System timer ticks per microsecond.
pub const TICKS_PER_US: u64 = TICK_HZ / 1_000_000;

/// Set once the counter has been started.
static STARTED: AtomicBool = AtomicBool::new(false);

/// Opens the timer's clock gate and starts unit 0 counting.
///
/// The bootloader normally leaves the timer running, but a program that was
/// reset by software can find the clock gate closed, and [`Instant::now`] has to
/// work before any driver has been created. The stall-on-core-halt bits stay
/// clear so the counter keeps counting while the CPU is idle.
///
/// Idempotent and safe to call from any context.
pub(crate) fn start() {
    if STARTED.swap(true, Ordering::AcqRel) {
        return;
    }

    let systimer = unsafe { &*SYSTIMER::PTR };
    systimer.conf.set_clk_en();
    systimer.conf.set_timer_unit0_work_en();
    systimer.conf.clr_timer_unit0_core0_stall_en();
    systimer.conf.clr_timer_unit0_core1_stall_en();
}

/// Latches and returns the raw 52-bit counter of unit 0.
///
/// The counter runs in the `CNT_CLK` domain and cannot be read directly: the
/// update register asks the hardware to copy it into `UNIT0_VALUE_HI/LO`, and
/// `VALUE_VALID` says when the copy is done.
#[inline]
pub(crate) fn read_ticks() -> u64 {
    let systimer = unsafe { &*SYSTIMER::PTR };

    systimer.unit0_op.trigger_timer_unit0_update();
    while !systimer.unit0_op.timer_unit0_value_valid() {
        core::hint::spin_loop();
    }

    let hi = systimer.unit0_value_hi.read_bits() as u64;
    let lo = systimer.unit0_value_lo.read_bits() as u64;
    (hi << 32) | lo
}

/// A span of time, counted in 16 MHz system timer ticks.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration {
    ticks: u64,
}

impl Duration {
    /// No time at all.
    pub const ZERO: Self = Self::from_ticks(0);
    /// The longest representable span.
    pub const MAX: Self = Self::from_ticks(u64::MAX);

    /// A span of `ticks` system timer ticks.
    pub const fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }

    /// A span of `micros` microseconds.
    pub const fn from_micros(micros: u64) -> Self {
        Self::from_ticks(micros.saturating_mul(TICKS_PER_US))
    }

    /// A span of `millis` milliseconds.
    pub const fn from_millis(millis: u64) -> Self {
        Self::from_ticks(millis.saturating_mul(TICKS_PER_US * 1_000))
    }

    /// A span of `secs` seconds.
    pub const fn from_secs(secs: u64) -> Self {
        Self::from_ticks(secs.saturating_mul(TICK_HZ))
    }

    /// The span in system timer ticks.
    pub const fn as_ticks(&self) -> u64 {
        self.ticks
    }

    /// The span in whole microseconds, rounded down.
    pub const fn as_micros(&self) -> u64 {
        self.ticks / TICKS_PER_US
    }

    /// The span in whole milliseconds, rounded down.
    pub const fn as_millis(&self) -> u64 {
        self.ticks / (TICKS_PER_US * 1_000)
    }

    /// The span in whole seconds, rounded down.
    pub const fn as_secs(&self) -> u64 {
        self.ticks / TICK_HZ
    }

    /// Whether the span is zero ticks long.
    pub const fn is_zero(&self) -> bool {
        self.ticks == 0
    }

    /// `self + rhs`, or `None` if the sum does not fit in 64 bits.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.ticks.checked_add(rhs.ticks) {
            Some(ticks) => Some(Self::from_ticks(ticks)),
            None => None,
        }
    }

    /// `self - rhs`, or `None` if `rhs` is the longer span.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.ticks.checked_sub(rhs.ticks) {
            Some(ticks) => Some(Self::from_ticks(ticks)),
            None => None,
        }
    }

    /// `self - rhs`, saturating at zero.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_ticks(self.ticks.saturating_sub(rhs.ticks))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs).expect("duration overflow")
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).expect("duration underflow")
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self::from_ticks(self.ticks.saturating_mul(rhs as u64))
    }
}

impl Div<u32> for Duration {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Self::from_ticks(self.ticks / rhs as u64)
    }
}

impl core::fmt::Debug for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}us", self.as_micros())
    }
}

/// A point in time, as counted by the system timer since boot.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    /// The counter value at boot.
    pub const EPOCH: Self = Self::from_ticks(0);

    /// A point `ticks` system timer ticks after boot.
    pub const fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }

    /// The current time.
    #[inline]
    pub fn now() -> Self {
        start();
        Self::from_ticks(read_ticks())
    }

    /// The time as a raw system timer tick count.
    pub const fn as_ticks(&self) -> u64 {
        self.ticks
    }

    /// A point `duration` after `self`, or `None` on overflow.
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        match self.ticks.checked_add(duration.ticks) {
            Some(ticks) => Some(Self::from_ticks(ticks)),
            None => None,
        }
    }

    /// A point `duration` before `self`, or `None` on underflow.
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        match self.ticks.checked_sub(duration.ticks) {
            Some(ticks) => Some(Self::from_ticks(ticks)),
            None => None,
        }
    }

    /// A point `duration` after `self`, saturating at the end of the tick range.
    pub const fn saturating_add(self, duration: Duration) -> Self {
        Self::from_ticks(self.ticks.saturating_add(duration.ticks))
    }

    /// A point `duration` before `self`, saturating at the epoch.
    pub const fn saturating_sub(self, duration: Duration) -> Self {
        Self::from_ticks(self.ticks.saturating_sub(duration.ticks))
    }

    /// The span between `earlier` and `self`, or `None` if `earlier` is later.
    pub const fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        match self.ticks.checked_sub(earlier.ticks) {
            Some(ticks) => Some(Duration::from_ticks(ticks)),
            None => None,
        }
    }

    /// The span between `earlier` and `self`, zero if `earlier` is later.
    pub const fn duration_since(self, earlier: Self) -> Duration {
        Duration::from_ticks(self.ticks.saturating_sub(earlier.ticks))
    }

    /// The span since `self`, zero if the clock went backwards.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        self.checked_add(rhs).expect("instant overflow")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        self.checked_sub(rhs).expect("instant underflow")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.checked_duration_since(rhs).expect("instant underflow")
    }
}

impl core::fmt::Debug for Instant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}us", self.ticks / TICKS_PER_US)
    }
}
