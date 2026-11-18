//! Interrupts: CPU lines, the peripheral matrix, handler ownership and levels.
//!
//! ```text
//!   peripheral IRQ ──► interrupt matrix ──► CPU line 0..31 ──► CLIC ──► core
//!   (e.g. SYSTIMER)     INTERRUPT_CORE0      (one of 32)       (level,
//!                       128 sources                            enable,
//!                                                              pending)
//! ```
//!
//! The chip has 128 peripheral interrupt sources but only 32 CPU interrupt
//! lines, so the matrix maps each source onto a line. Several sources may share
//! a line; a handler then decides which one fired with [`source_pending`]. The
//! CLIC adds a level, an enable bit and a pending bit per line ([`Level`]), and
//! the trap entry dispatches to the handler registered for the line.
//!
//! # Two levels of API
//!
//! - [`claim`] is the normal way to take a line: it checks that nobody else owns
//!   it, routes the source, programs the level, installs the handler and returns
//!   an [`InterruptHandler`] that masks the line and gives the ownership back on
//!   drop.
//! - The raw functions re-exported from `esp32p4-rt` ([`set_handler`],
//!   [`unmask`], [`mask`], …) remain available for the rare case where the line
//!   lifetime is not lexical.
//!
//! Lines the HAL itself drives:
//!
//! | Line | Owner |
//! |------|-------|
//! | 19 | `SYSTIMER` target 0, used by [`Delay::sleep_us`](crate::delay::Delay::sleep_us) |
//! | 20 | Ethernet MAC |
//!
//! Drivers that cannot know the line at compile time take it as an argument
//! instead: [`Tg0T0::enable_interrupt`](crate::timer::Tg0T0::enable_interrupt)
//! and [`Alarm1::enable_interrupt`](crate::systimer::Alarm1::enable_interrupt)
//! claim whatever line the application passes, so the general-purpose timers
//! and the system timer alarms can share the lines nobody else wanted.
//!
//! # Levels
//!
//! `mcliccfg.MNLBITS` is 3, so each line has an 8-bit level byte: three level
//! bits and five low bits hardwired to 1 (level 0 reads as `0x1f`, level 7 as
//! `0xff`). The hardware compares that byte against
//! `max(mintthresh.TH, mintstatus.MIL)` and takes the interrupt only when the
//! byte is greater; the reset threshold of 0 accepts every level.
//!
//! Levels only decide **which of the pending interrupts is taken first**. The
//! trap entry runs with `MIE = 0`, so a handler is never preempted by another
//! interrupt, whatever its level. Keep handlers short and defer the work.
//!
//! # Handler rules
//!
//! - Clear the peripheral's status before returning, otherwise a level-triggered
//!   source re-fires immediately.
//! - Do not block: [`sleep`] would park the core inside the handler.
//! - Shared state with the main loop needs [`free`]/[`Mutex`] or atomics.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::interrupt::{self, CpuInterrupt, Interrupt, Level};
//!
//! fn timer_isr() {
//!     // acknowledge the peripheral here
//! }
//!
//! // Take line 21 for the timer, at the default level, and start taking it.
//! let handler = unsafe {
//!     interrupt::claim(Interrupt::TG0_T0, CpuInterrupt::new(21), Level::LOW, timer_isr)
//! };
//! handler.enable();
//! unsafe { interrupt::enable() };
//! ```
//!
//! Sharing a line:
//!
//! ```ignore
//! fn shared_isr() {
//!     if interrupt::source_pending(Interrupt::LP_GPIO) {
//!         // handle the GPIO source
//!     }
//!     if interrupt::source_pending(Interrupt::RMT) {
//!         // handle RMT
//!     }
//! }
//!
//! let handler = unsafe {
//!     interrupt::claim(Interrupt::LP_GPIO, CpuInterrupt::new(22), Level::LOW, shared_isr)
//! };
//! interrupt::route(Interrupt::RMT, CpuInterrupt::new(22)); // second source
//! ```

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_pac::interrupt_core0::BASE as MATRIX_BASE;
use esp32p4_rt::interrupt as rt;

/// A peripheral interrupt source, i.e. one input of the interrupt matrix.
pub use esp32p4_pac::Interrupt;

pub use esp32p4_rt::interrupt::{
    CPU_INTERRUPT_COUNT, MAX_LEVEL, clear_handler, current_level, disable, enable, in_interrupt,
    is_enabled, is_pending, is_unmasked, level, mask, pend, restore, set_handler, set_level, sleep,
    unmask, unpend,
};

/// Number of interrupt sources the matrix accepts.
pub const SOURCE_COUNT: u32 = 128;

/// One input of the interrupt matrix.
///
/// The vendor SVD only names 95 of the 128 sources (the Ethernet MAC's, for
/// example, is missing), so the routing API takes this type rather than the
/// [`Interrupt`] enum: every named source converts into it with `From`, and the
/// gaps are available as constants or through [`Source::new`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Source(u8);

impl Source {
    /// The source with matrix index `id` (0..=127).
    ///
    /// TRM Table 13.4-1 lists the index of every source; it is also the offset
    /// of the source's map register divided by four.
    pub const fn new(id: u8) -> Self {
        assert!(id < SOURCE_COUNT as u8, "no such interrupt source");
        Self(id)
    }

    /// The matrix index, which is also the offset of its map register divided
    /// by four.
    #[inline]
    pub const fn id(self) -> u8 {
        self.0
    }

    /// Ethernet MAC (`SBD_INTR`), matrix index 92.
    ///
    /// The MAC declares no interrupt in the vendor SVD, so there is no
    /// `Interrupt` variant for it.
    pub const SBD: Self = Self(92);
}

impl From<Interrupt> for Source {
    #[inline]
    fn from(source: Interrupt) -> Self {
        Self(source.number() as u8)
    }
}

/// Offset of `CORE0_INTR_STATUS_REG_0` in the matrix.
const INTR_STATUS: u32 = 0x0200;

/// A CPU interrupt line.
///
/// The value is the line number the matrix maps a source to, not a CLIC id:
/// CLIC ids 16..=47 correspond to lines 0..=31.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CpuInterrupt(u8);

impl CpuInterrupt {
    /// The line with number `line` (0..=31).
    pub const fn new(line: u8) -> Self {
        assert!(
            line < CPU_INTERRUPT_COUNT as u8,
            "no such CPU interrupt line"
        );
        Self(line)
    }

    /// The line number.
    #[inline]
    pub const fn number(self) -> u8 {
        self.0
    }

    /// The line as a mask bit, for a set of lines.
    #[inline]
    pub const fn bit(self) -> u32 {
        1 << self.0
    }
}

impl From<CpuInterrupt> for u32 {
    #[inline]
    fn from(line: CpuInterrupt) -> Self {
        line.0 as u32
    }
}

/// CPU interrupt lines claimed by this HAL.
pub mod lines {
    use super::CpuInterrupt;

    /// `SYSTIMER` target 0 comparator, used for timed sleeps.
    pub const SYSTIMER_TARGET0: CpuInterrupt = CpuInterrupt::new(19);
    /// Ethernet MAC (`ETH_MAC_INTR`, delivered as `SBD_INTR`).
    pub const EMAC: CpuInterrupt = CpuInterrupt::new(20);
}

/// A CLIC interrupt level.
///
/// Level 0 is the reset default and the lowest; level 7 is the highest. The
/// three bits live in the top of the 8-bit `clicintctl` byte, whose low five
/// bits are hardwired to 1 — [`byte`](Self::byte) returns the full byte the
/// hardware compares.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Level(u8);

impl Level {
    /// Level 0, the reset default.
    pub const LOW: Self = Self(0);
    /// Level 7, the highest.
    pub const HIGH: Self = Self(MAX_LEVEL);

    /// The level with number `level` (0..=7).
    pub const fn new(level: u8) -> Self {
        assert!(level <= MAX_LEVEL, "level out of range");
        Self(level)
    }

    /// The level number.
    #[inline]
    pub const fn number(self) -> u8 {
        self.0
    }

    /// The `clicintctl` byte for this level.
    #[inline]
    pub const fn byte(self) -> u8 {
        rt::level_byte(self.0)
    }

    /// Decode a `clicintctl` byte.
    #[inline]
    pub const fn from_byte(byte: u8) -> Self {
        Self(rt::level_of_byte(byte))
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::LOW
    }
}

// ============================================================================
// Interrupt matrix
// ============================================================================

/// Address of the matrix's map register for `source`.
///
/// The 128 map registers are consecutive, four bytes apart and in source order
/// (`CORE0_INTR_STATUS_REG_0` covers sources 0..=31, and so on), so the source
/// id doubles as the register index.
#[inline]
fn map_register(source: Source) -> *mut u32 {
    (MATRIX_BASE + 4 * source.id() as u32) as *mut u32
}

/// Map `source` onto CPU interrupt line `line`.
///
/// A source may be mapped onto exactly one line per core; mapping it again moves
/// it. Several sources may share a line. The source stays pending until its
/// peripheral is acknowledged, so route it before unmasking the line.
pub fn route(source: impl Into<Source>, line: CpuInterrupt) {
    // SAFETY: `Source` only holds indices below `SOURCE_COUNT`, and the block is
    // MMIO.
    unsafe { core::ptr::write_volatile(map_register(source.into()), line.number() as u32) };
}

/// The line `source` is currently mapped to.
pub fn route_of(source: impl Into<Source>) -> CpuInterrupt {
    // SAFETY: see `route`.
    let value = unsafe { core::ptr::read_volatile(map_register(source.into())) };
    CpuInterrupt::new((value & 0x1f) as u8)
}

/// Whether `source` is currently asserting.
///
/// The matrix latches one status bit per source, so a handler for a shared line
/// can tell its sources apart without touching the peripherals.
#[inline]
pub fn source_pending(source: impl Into<Source>) -> bool {
    let id = source.into().id() as u32;
    // SAFETY: `INTR_STATUS` holds four registers covering all 128 sources.
    let status = unsafe {
        core::ptr::read_volatile((MATRIX_BASE + INTR_STATUS + 4 * (id / 32)) as *const u32)
    };
    status & (1 << (id % 32)) != 0
}

// ============================================================================
// Line ownership
// ============================================================================

/// Lines currently handed out by [`claim`].
static CLAIMED: AtomicU32 = AtomicU32::new(0);

/// A CPU interrupt line taken by [`claim`].
///
/// Dropping it masks the line, removes the handler and gives the line back;
/// [`leak`](Self::leak) keeps it for the rest of the program, which is what
/// drivers whose line outlives their handle need.
pub struct InterruptHandler {
    line: CpuInterrupt,
}

impl InterruptHandler {
    /// The line this handler owns.
    #[inline]
    pub fn line(&self) -> CpuInterrupt {
        self.line
    }

    /// Unmask the line, so a pending source can be taken.
    pub fn enable(&self) {
        // SAFETY: `claim` installed the handler for this line.
        unsafe { rt::unmask(self.line.number() as u32) };
    }

    /// Mask the line, leaving a pending request latched in the CLIC.
    pub fn disable(&self) {
        rt::mask(self.line.number() as u32);
    }

    /// Whether the line is unmasked.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        rt::is_unmasked(self.line.number() as u32)
    }

    /// Keep the handler installed for the rest of the program.
    pub fn leak(self) {
        core::mem::forget(self);
    }
}

impl Drop for InterruptHandler {
    fn drop(&mut self) {
        let line = self.line.number() as u32;
        rt::mask(line);
        rt::clear_handler(line);
        CLAIMED.fetch_and(!self.line.bit(), Ordering::AcqRel);
    }
}

impl core::fmt::Debug for InterruptHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InterruptHandler")
            .field("line", &self.line)
            .field("enabled", &self.is_enabled())
            .finish()
    }
}

/// Take CPU interrupt line `line` for `source` and install `handler`.
///
/// The line is routed to `handler`, its level is programmed and the handler is
/// registered, but the line is left **masked**: call
/// [`InterruptHandler::enable`] once the peripheral is ready to interrupt (its
/// status acknowledged, its error sources unmasked).
///
/// # Panics
///
/// Panics when another [`claim`] still owns the line, which catches two drivers
/// silently overwriting each other's handler.
///
/// # Safety
///
/// The caller must ensure the source cannot be pending in a way that makes the
/// first interrupt unsafe — normally by acknowledging the peripheral before
/// enabling the line. The handler must clear its peripheral's status.
pub unsafe fn claim(
    source: impl Into<Source>,
    line: CpuInterrupt,
    level: Level,
    handler: fn(),
) -> InterruptHandler {
    let bit = line.bit();
    let mut claimed = CLAIMED.load(Ordering::Relaxed);
    loop {
        assert!(
            claimed & bit == 0,
            "CPU interrupt line {} is already in use",
            line.number()
        );
        match CLAIMED.compare_exchange_weak(
            claimed,
            claimed | bit,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(actual) => claimed = actual,
        }
    }

    route(source, line);
    rt::set_level(line.number() as u32, level.number());
    rt::set_handler(line.number() as u32, handler);
    // Drop a request left over from before the claim; if the source is still
    // asserting, the level-triggered CLIC sets it again right away.
    rt::unpend(line.number() as u32);

    InterruptHandler { line }
}

// ============================================================================
// Critical sections
// ============================================================================

/// Proof that machine interrupts are disabled.
///
/// Handed to the closure of [`free`]; guards such as [`Mutex`] require it, so
/// shared state can only be touched inside a critical section.
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _marker: core::marker::PhantomData<*const ()>,
}

impl CriticalSection {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// Run `f` with machine interrupts disabled, restoring the previous state after.
///
/// The closure receives a [`CriticalSection`] token that [`Mutex`] requires.
/// Sections nest: the outermost one decides when interrupts come back.
///
/// Keep them short — a critical section delays every interrupt, and the timer
/// used by [`sleep`] cannot wake the core until it ends.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    let was_enabled = rt::disable();
    let result = f(CriticalSection::new());
    // SAFETY: `was_enabled` is the state `disable` returned, and each `free`
    // restores exactly its own state.
    unsafe { rt::restore(was_enabled) };
    result
}

/// A value shared between an interrupt handler and the main loop.
///
/// Access requires a [`CriticalSection`] token, so it can only be reached with
/// interrupts disabled — which is what makes the `Sync` implementation sound on
/// a single core.
///
/// ```ignore
/// use core::cell::RefCell;
/// use esp32p4::interrupt::{self, Mutex};
///
/// static QUEUE: Mutex<RefCell<Queue>> = Mutex::new(RefCell::new(Queue::new()));
///
/// interrupt::free(|cs| QUEUE.borrow(cs).borrow_mut().push(byte));
/// ```
pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

// SAFETY: the only way to reach `data` is through a `CriticalSection` token,
// which proves that interrupts (and therefore any handler that could touch the
// value) are disabled on this single-core chip.
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Wrap `value`.
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
        }
    }

    /// Borrow the value.
    ///
    /// The token proves interrupts are disabled. The reference is tied to the
    /// `Mutex`, not to the token, so it *can* outlive the critical section:
    /// copy the data out instead of returning the reference from [`free`] when
    /// the value has interior mutability that a handler also uses.
    #[inline]
    pub fn borrow(&self, _cs: CriticalSection) -> &T {
        // SAFETY: see the `Sync` implementation.
        unsafe { &*self.data.get() }
    }

    /// Raw pointer to the value, for handing it to code that cannot pass the
    /// token around.
    ///
    /// # Safety
    ///
    /// The caller must still guarantee that no interrupt handler touches the
    /// value concurrently.
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.data.get()
    }
}

/// Disable machine interrupts until the returned guard is dropped.
///
/// Interrupts stay enabled in the CLIC; only the core's `MIE` bit is cleared,
/// so nesting with [`free`] and [`disable`] works as expected.
pub fn disable_all() -> InterruptGuard {
    let was_enabled = rt::disable();
    InterruptGuard { was_enabled }
}

/// Restores the interrupt state captured by [`disable_all`] when dropped.
pub struct InterruptGuard {
    was_enabled: bool,
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.was_enabled {
            // SAFETY: interrupts were enabled when the guard was created.
            unsafe { rt::enable() };
        }
    }
}
