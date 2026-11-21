//! The CLINT's machine timer: a per-core comparator on a shared counter.
//!
//! This is the RISC-V equivalent of ARM's SysTick — each core has its own
//! 64-bit comparator and its own interrupt, so core 0 and core 1 can tick
//! independently — except that the counter behind it is **one** counter for the
//! chip, not one per core.
//!
//! # Where the registers are
//!
//! The CLINT has no PAC entry: like the CLIC, its registers sit in a core-local
//! window at a fixed address, and each core sees its own copy at the same
//! address. That is what makes the comparator and the interrupt per core.
//!
//! | Register | Offset | Notes |
//! |----------|--------|-------|
//! | `msip`         | `0x0000` | machine software interrupt pending |
//! | `mtimecmplo`   | `0x4000` | comparator, low half |
//! | `mtimecmphi`   | `0x4004` | comparator, high half |
//! | `mtimeloadlo`  | `0x4008` | counter reload, low half |
//! | `mtimeloadhi`  | `0x400C` | counter reload, high half |
//! | `mtimectl`     | `0x4010` | enable, overflow flag, sampling mode |
//! | `mtimelo`      | `0xBFF8` | counter, low half (read-only) |
//! | `mtimehi`      | `0xBFFC` | counter, high half (read-only) |
//!
//! # What is shared and what is not
//!
//! The counter and its enable bit are shared: `MTIME_EN` is implemented only in
//! core 0's copy of `mtimectl`, and writing it in core 1's has no effect. The
//! comparator and the interrupt are per core — the interrupt is CLIC id 7, which
//! is not one of the 32 lines the peripheral matrix drives, so it goes through
//! [`crate::interrupt::set_machine_timer_handler`] rather than
//! [`crate::interrupt::claim`].
//!
//! # Interrupt behaviour
//!
//! The interrupt is level-triggered and asserts while the counter is at or past
//! the comparator. There is no flag to clear: the handler acknowledges it by
//! moving the comparator forward, which is also how a periodic tick is made.
//!
//! # Tick rate
//!
//! The counter advances once per CPU cycle, so its rate is `CPU_CLK`: 90 MHz on
//! the frequency the chip boots at, and every change to the CPU divider changes
//! it. Use [`ticks_per_us`] / [`ticks_per_ms`] with
//! [`ClockTree::cpu_hz`](crate::clock::ClockTree::cpu_hz) to convert. This is
//! unlike `SYSTIMER`, whose 16 MHz comes from the crystal and does not move.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::machine_timer::{MachineTimer, ticks_per_ms};
//!
//! static TICKS: AtomicU32 = AtomicU32::new(0);
//!
//! fn on_tick() {
//!     TICKS.fetch_add(1, Ordering::Relaxed);
//!     let timer = unsafe { MachineTimer::steal() };
//!     timer.set_target(timer.now() + ticks_per_ms(90_000_000) * 10);
//! }
//!
//! let timer = MachineTimer::init();
//! timer.set_target(timer.now() + ticks_per_ms(90_000_000) * 10);
//! timer.enable_interrupt(Level::LOW, on_tick);
//! unsafe { esp32p4_hal::interrupt::enable() };
//! ```

use core::sync::atomic::{AtomicBool, Ordering};

use crate::interrupt::{self, Level};
use crate::multicore::CoreId;

/// Base of the CLINT window.
///
/// The TRM lists the CLINT registers relative to the "CPU sub-system base" and
/// never spells the base out; it is `0x2000_0000`, not the CLIC's
/// `0x2080_0000` (which is where the interrupt controller lives). The
/// bootloader leaves the counter running, so `mtimectl` already reads back with
/// `MTIME_EN` set.
///
/// Like the CLIC, this window is core-local: both cores see their own
/// comparator at the same address.
const CLINT_BASE: usize = 0x2000_0000;

/// `mtimectl` at `CLINT_BASE + 0x4010`.
const MTIMECTL: usize = CLINT_BASE + 0x4010;
/// `mtimecmplo` at `CLINT_BASE + 0x4000`.
const MTIMECMPLO: usize = CLINT_BASE + 0x4000;
/// `mtimecmphi` at `CLINT_BASE + 0x4004`.
const MTIMECMPHI: usize = CLINT_BASE + 0x4004;
/// `mtimelo` at `CLINT_BASE + 0xBFF8`.
const MTIMELO: usize = CLINT_BASE + 0xBFF8;
/// `mtimehi` at `CLINT_BASE + 0xBFFC`.
const MTIMEHI: usize = CLINT_BASE + 0xBFFC;

/// `mtimectl.MTIME_EN`.
const MTIMECTL_EN: u32 = 1 << 0;
/// `mtimectl.MTIME_OVF`.
const MTIMECTL_OVF: u32 = 1 << 3;
/// `mtimectl.MTIME_SAM`, sampling the other half when either half is read.
const MTIMECTL_SAM_BOTH: u32 = 0b11 << 4;

/// Whether this core's machine timer interrupt has a handler.
///
/// There is one machine timer interrupt per core (CLIC id 7), so two drivers
/// cannot both have it: the second claim panics instead of silently replacing the
/// first handler.
static CLAIMED: [AtomicBool; CoreId::COUNT] = [const { AtomicBool::new(false) }; CoreId::COUNT];

/// Counter ticks per microsecond, at `cpu_hz`.
///
/// The counter advances once per CPU cycle: measured against `SYSTIMER` it read
/// 90.00 MHz with the boot clock at 90 MHz, and exactly 180.00 MHz after the CPU
/// divider was halved, so it follows `CPU_CLK` rather than a fixed clock.
/// Anything that changes the CPU frequency changes the tick rate with it, which
/// makes this a function of the frequency rather than a constant.
///
/// ```ignore
/// let clock = ClockTree::init(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST);
/// let ticks = MachineTimer::ticks_per_ms(clock.cpu_hz());
/// ```
#[inline]
pub const fn ticks_per_us(cpu_hz: u32) -> u64 {
    cpu_hz as u64 / 1_000_000
}

/// Counter ticks per millisecond, at `cpu_hz`.
#[inline]
pub const fn ticks_per_ms(cpu_hz: u32) -> u64 {
    cpu_hz as u64 / 1_000
}

#[inline]
fn read(address: usize) -> u32 {
    // SAFETY: fixed, core-local MMIO addresses.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline]
fn write(address: usize, value: u32) {
    // SAFETY: fixed, core-local MMIO addresses.
    unsafe { core::ptr::write_volatile(address as *mut u32, value) }
}

/// This core's machine timer.
///
/// Owning one says nothing about the shared counter: every core has a handle, and
/// which core enabled the counter is not observable from here.
pub struct MachineTimer {
    _private: (),
}

impl MachineTimer {
    /// Takes this core's machine timer and starts the shared counter.
    ///
    /// Enabling the counter only works from core 0 (the enable bit is only
    /// implemented in core 0's copy), so this is harmless but ineffective on
    /// core 1. Programs that start core 1 before creating a timer should create
    /// one on core 0 first.
    pub fn init() -> Self {
        let this = Self { _private: () };

        // Sample the other half of the counter whenever either half is read, so
        // `now` needs no retry loop even though the counter is shared.
        write(
            MTIMECTL,
            (read(MTIMECTL) & !MTIMECTL_SAM_BOTH) | MTIMECTL_SAM_BOTH,
        );
        write(MTIMECTL, MTIMECTL_EN);
        this.clear_overflow();

        // A comparator of all ones never matches; leave the interrupt alone.
        this.set_target(u64::MAX);
        this
    }

    /// Obtains a handle without taking ownership.
    ///
    /// # Safety
    ///
    /// The caller must ensure [`init`](Self::init) has run and that the handle is
    /// not used to change the comparator while an interrupt is armed on it.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self { _private: () })
    }

    /// The counter, as 64 bits of ticks since the chip came out of reset.
    ///
    /// The two halves are sampled consistently: [`init`](Self::init) sets the
    /// sampling mode that latches the high half when the low half is read.
    #[inline]
    pub fn now(&self) -> u64 {
        let lo = u64::from(read(MTIMELO));
        let hi = u64::from(read(MTIMEHI));
        (hi << 32) | lo
    }

    /// The comparator value.
    #[inline]
    pub fn target(&self) -> u64 {
        let hi = u64::from(read(MTIMECMPHI));
        let lo = u64::from(read(MTIMECMPLO));
        (hi << 32) | lo
    }

    /// Moves the comparator to `ticks`.
    ///
    /// The high half is set to all ones first and written last, so the timer can
    /// never fire on a half-written value — the usual RISC-V sequence for a
    /// 64-bit comparator behind two 32-bit registers.
    #[inline]
    pub fn set_target(&self, ticks: u64) {
        write(MTIMECMPHI, u32::MAX);
        write(MTIMECMPLO, ticks as u32);
        write(MTIMECMPHI, (ticks >> 32) as u32);
    }

    /// Arms the interrupt for `ticks` counter ticks from now.
    #[inline]
    pub fn set_target_after(&self, ticks: u64) {
        self.set_target(self.now().wrapping_add(ticks));
    }

    /// Whether the counter has wrapped past `0xFFFF_FFFF_FFFF_FFFF`.
    ///
    /// Hardware sets the flag at the maximum value; writing zero clears it.
    #[inline]
    pub fn overflowed(&self) -> bool {
        read(MTIMECTL) & MTIMECTL_OVF != 0
    }

    /// Clears the overflow flag.
    #[inline]
    pub fn clear_overflow(&self) {
        write(MTIMECTL, read(MTIMECTL) & !MTIMECTL_OVF);
    }

    /// Whether the counter is running.
    ///
    /// Only core 0's copy reports the real state; on core 1 this reads whatever
    /// the core-1 copy holds, which hardware ignores.
    #[inline]
    pub fn is_running(&self) -> bool {
        read(MTIMECTL) & MTIMECTL_EN != 0
    }

    /// Routes this core's machine timer interrupt to `handler`.
    ///
    /// The interrupt fires while the counter is at or past the comparator, so the
    /// handler has to move the comparator forward (or mask the interrupt) to
    /// acknowledge it.
    ///
    /// # Panics
    ///
    /// Panics when this core's machine timer interrupt already has a handler:
    /// [`crate::delay::Delay::sleep_ticks`] takes it the first time it parks the
    /// core, and a program that wants its own periodic tick has to pick one or the
    /// other.
    pub fn enable_interrupt(&self, level: Level, handler: fn()) {
        assert!(
            !CLAIMED[CoreId::current().as_index()].swap(true, Ordering::AcqRel),
            "this core's machine timer interrupt is already in use"
        );
        interrupt::set_machine_timer_handler(handler);
        interrupt::set_machine_timer_level(level.number());
        interrupt::unmask_machine_timer();
    }

    /// Masks this core's machine timer interrupt.
    ///
    /// The comparator keeps its value; the interrupt fires again as soon as it is
    /// unmasked if the counter has passed it.
    pub fn disable_interrupt(&self) {
        interrupt::mask_machine_timer();
    }

    /// Removes the handler and masks the interrupt.
    pub fn release_interrupt(&self) {
        interrupt::mask_machine_timer();
        interrupt::clear_machine_timer_handler();
        CLAIMED[CoreId::current().as_index()].store(false, Ordering::Release);
    }

    /// Whether this core's machine timer interrupt is unmasked.
    #[inline]
    pub fn interrupt_enabled(&self) -> bool {
        interrupt::machine_timer_unmasked()
    }

    /// Whether this core's machine timer interrupt is still asserting.
    ///
    /// The interrupt is level-triggered on `counter >= comparator`, so this stays
    /// true until the comparator is moved past the counter.
    #[inline]
    pub fn interrupt_pending(&self) -> bool {
        interrupt::machine_timer_pending()
    }
}
