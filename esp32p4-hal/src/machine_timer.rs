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
//! The counter advances once per CPU cycle, so its rate is `CPU_CLK`: a quarter of
//! the CPLL, which is 90 MHz on a pre-v3.0 die and 100 MHz afterwards, and every
//! change to the CPU divider changes it. Until a program records the real value
//! with [`init`](MachineTimer::init) — pass
//! [`ClockTree::cpu_hz`](crate::clock::ClockTree::cpu_hz) — delays assume
//! [`BOOT_CPU_HZ`], the faster of the two boot rates, so they err long rather than
//! short. This is unlike `SYSTIMER`, whose 16 MHz comes from the crystal and does
//! not move.
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
//! let timer = MachineTimer::init(90_000_000); // CPU_CLK, 90 MHz as booted
//! timer.set_target(timer.now() + ticks_per_ms(90_000_000) * 10);
//! timer.enable_interrupt(Level::LOW, on_tick);
//! unsafe { esp32p4_hal::interrupt::enable() };
//! ```

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::delay::DelayUs;
use crate::interrupt::{self, Level};
use crate::multicore::CoreId;

/// The highest `CPU_CLK` a bootloader leaves the chip at: the 400 MHz CPLL
/// divided by four.
///
/// The second-stage bootloader always divides by four, so the rate is a quarter of
/// whatever the die's CPLL runs at: dies before v3.0 have a 360 MHz CPLL and boot
/// at 90 MHz, v3.0 and later have 400 MHz and boot at 100 MHz.
///
/// Delays convert microseconds to ticks as `us * CPU_HZ / 1e6`, so overestimating
/// the rate overestimates the tick count and a wait only ever ends *late*, never
/// early. Assuming the higher of the two therefore keeps every wait long enough on
/// both: the cost is that a 90 MHz part waits 11 % longer than asked, and the
/// alternative — assuming 90 MHz — would make a 100 MHz part wait 10 % *less* than
/// asked, which is the direction that breaks register timing.
///
/// Call [`init`](MachineTimer::init) with
/// [`ClockTree::cpu_hz`](crate::clock::ClockTree::cpu_hz) for the exact rate.
pub const BOOT_CPU_HZ: u32 = 100_000_000;

/// `CPU_CLK` in Hz: `mtime`'s rate.
///
/// The rate belongs to the chip, not to a handle: `mtime` is one counter and both
/// cores read the same rate. A `static` keeps every handle — the owner's and any
/// [`steal`](MachineTimer::steal)ed copy — looking at the same number. It starts at
/// [`BOOT_CPU_HZ`] and [`init`](MachineTimer::init) / [`set_cpu_hz`](MachineTimer::set_cpu_hz)
/// replace it with a measured value.
static CPU_HZ: AtomicU32 = AtomicU32::new(BOOT_CPU_HZ);

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

impl DelayUs for MachineTimer {
    /// Busy-waits `us` microseconds on this core's machine timer.
    ///
    /// Waiting reads the counter and nothing else, so it needs no comparator and
    /// two cores can wait at the same time without disturbing each other. The
    /// conversion keeps full precision: `us * CPU_CLK / 1e6` rather than rounding
    /// to whole ticks per microsecond first.
    ///
    /// The rate comes from [`cpu_hz`](MachineTimer::cpu_hz), which starts at
    /// [`BOOT_CPU_HZ`] and follows [`init`](MachineTimer::init) /
    /// [`set_cpu_hz`](MachineTimer::set_cpu_hz), so this works before the program
    /// has looked at the clock tree.
    #[inline]
    fn delay_us(&self, us: u32) {
        if us == 0 {
            return;
        }
        let ticks = u64::from(us) * u64::from(Self::cpu_hz()) / 1_000_000;
        let start = self.now();
        while self.now().wrapping_sub(start) < ticks {
            core::hint::spin_loop();
        }
    }
}

impl MachineTimer {
    /// Takes this core's machine timer and sets up the counter reads.
    ///
    /// Nothing here starts the counter: the second-stage bootloader already did,
    /// and `MTIME_EN` is only implemented in core 0's copy of `mtimectl`, so
    /// writing it from core 1 would do nothing anyway. [`is_running`](Self::is_running)
    /// reports what the counter is actually doing.
    ///
    /// `cpu_hz` is the rate of `CPU_CLK`, which is what this counter counts; pass
    /// [`ClockTree::cpu_hz`](crate::clock::ClockTree::cpu_hz). The rate is stored
    /// because [`DelayUs`] needs it and there is no way to read it from the CLINT
    /// itself; call [`set_cpu_hz`](Self::set_cpu_hz) if the CPU frequency changes.
    pub fn init(cpu_hz: u32) -> Self {
        CPU_HZ.store(cpu_hz, Ordering::Release);
        let this = Self { _private: () };

        // Sample the other half of the counter whenever either half is read, so
        // `now` needs no retry loop even though the counter is shared. Read,
        // modify and write: `EN` and `OVF` belong to the bootloader and to
        // `clear_overflow`, and a plain store would wipe the bits set above.
        write(MTIMECTL, read(MTIMECTL) | MTIMECTL_SAM_BOTH);
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

    /// Records a new `CPU_CLK` rate after the clock tree moved.
    ///
    /// Only needed for waiting: the comparator and [`now`](Self::now) count in
    /// ticks and never look at the rate. This updates the chip-wide value, which
    /// is why it needs `&mut self` — a [`Steal`](crate::Steal)ed copy can read the
    /// rate but not change it.
    pub fn set_cpu_hz(&mut self, cpu_hz: u32) {
        CPU_HZ.store(cpu_hz, Ordering::Release);
    }

    /// Counter ticks per microsecond at the current rate.
    #[inline]
    pub fn ticks_per_us(&self) -> u64 {
        u64::from(Self::cpu_hz()) / 1_000_000
    }

    /// The rate `mtime` is currently being converted with.
    #[inline]
    pub fn cpu_hz() -> u32 {
        CPU_HZ.load(Ordering::Acquire)
    }

    /// Microseconds since the counter started.
    ///
    /// The resolution is one CPU tick, but the result is whole microseconds: the
    /// rate is divided out once, so no accumulation error builds up over a long
    /// run. A rate below 1 MHz would divide by zero, which cannot happen on this
    /// chip.
    #[inline]
    pub fn micros(&self) -> u64 {
        self.now() / self.ticks_per_us().max(1)
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
