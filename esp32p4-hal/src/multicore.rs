//! Second HP core, inter-core signalling and shared-state locks.
//!
//! The ESP32-P4 has three cores: two "high performance" RISC-V cores that run
//! the same instruction set and share memory, and a small LP core that this
//! module does not touch. Only core 0 leaves reset at power-on; [`Multicore`]
//! starts core 1.
//!
//! # What a started core inherits
//!
//! Core 1 runs the same code, from the same flash mapping, with its own stack
//! and its own CLIC (the interrupt controller is per hart, which is why the
//! interrupt tables and line claims in [`crate::interrupt`] are per core). It
//! does *not* get its own `Peripherals`: the PAC hands out one set. Anything
//! two cores touch has to be shared deliberately — put it in a static, protect
//! it with a [`Mutex`], and make sure the peripheral it drives is owned by
//! exactly one of them.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::multicore::{self, CoreId, Mutex, Multicore, Stack};
//!
//! static STACK: Stack<8192> = Stack::new();
//! static COUNTER: Mutex<u32> = Mutex::new(0);
//!
//! fn core1() -> ! {
//!     loop {
//!         *COUNTER.lock() += 1;
//!     }
//! }
//!
//! let mut mc = Multicore::init(dp.LP_SYS, dp.HP_SYS, dp.PMU, dp.HP_SYS_CLKRST);
//! mc.launch_core1(core1, &STACK);
//! assert_eq!(multicore::core_id(), CoreId::Core0);
//! ```
//!
//! # Signalling
//!
//! There is no inter-core FIFO on this chip, but there are four software
//! interrupt sources, and the register that raises one names the core it is
//! aimed at: [`signal`] pokes the other core, which takes it as an ordinary
//! interrupt on the line it routes [`CoreId::interrupt_source`] to.
//!
//! # Why the locks are software
//!
//! Unlike the ESP32, this chip has no spinlock peripheral. [`Mutex`] is a
//! test-and-set on an `AtomicBool` with acquire/release ordering: correct across
//! the two cores because they are cache-coherent through the shared L1 and the
//! `A` extension's atomics go to memory, but it *spins*, so it must not be held
//! across a wait for the other core.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use esp32p4_pac::{HP_SYS, HP_SYS_CLKRST, LP_SYS, PMU};

use crate::interrupt::Source;

/// PMU stall code that halts a core.
const PARK_CODE: u32 = 0x86;
/// PMU stall code that lets a core run.
///
/// The complementary value, `0x86`, halts it; [`crate::startup`] parks core 1
/// with it at boot.
const UNSTALL_CODE: u32 = 0xff;

/// Interrupt-matrix index of `CPU_INTR_FROM_CPU_0`.
///
/// The four inter-core sources are not at the bottom of the matrix: they sit at
/// `COREx_CPU_INT_FROM_CPU_n_MAP`, which is index 79 in both cores' matrices.
const FROM_CPU_SOURCE_BASE: u8 = 79;

/// Which HP core is executing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoreId {
    /// The core that leaves reset first and runs `#[main]`.
    Core0 = 0,
    /// The core [`Multicore::launch_core1`] starts.
    Core1 = 1,
}

impl CoreId {
    /// Number of HP cores.
    pub const COUNT: usize = 2;

    /// The core this code runs on, from `mhartid`.
    #[inline]
    pub fn current() -> Self {
        match crate::interrupt::hart() {
            0 => Self::Core0,
            _ => Self::Core1,
        }
    }

    /// The core with index `index`, or `None` above [`COUNT`](Self::COUNT).
    #[inline]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Core0),
            1 => Some(Self::Core1),
            _ => None,
        }
    }

    /// Index of this core, usable for arrays.
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }

    /// The other HP core.
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            Self::Core0 => Self::Core1,
            Self::Core1 => Self::Core0,
        }
    }

    /// The software interrupt source that reaches this core.
    ///
    /// The four `CPU_INTR_FROM_CPU_n` sources are numbered by the core they are
    /// aimed at, so the source to route on a core is the core's own index.
    /// [`signal`] raises exactly that source on the target.
    #[inline]
    pub const fn interrupt_source(self) -> Source {
        Source::new(FROM_CPU_SOURCE_BASE + self as u8)
    }
}

/// A stack for a core started by [`Multicore::launch_core1`].
///
/// Lives in `.bss`, so it costs its size in RAM and nothing else. 4 KiB is
/// enough for ordinary code; anything that formats or recurses deeply wants
/// more.
#[repr(align(16))]
pub struct Stack<const N: usize> {
    memory: UnsafeCell<[u8; N]>,
}

// SAFETY: the stack is only ever used by the core it is handed to, and handing
// it over takes `&'static self`, so it cannot be aliased by the caller.
unsafe impl<const N: usize> Sync for Stack<N> {}

impl<const N: usize> Stack<N> {
    /// A zeroed stack.
    pub const fn new() -> Self {
        Self {
            memory: UnsafeCell::new([0; N]),
        }
    }

    /// Top of the stack, which grows down.
    #[inline]
    pub fn top(&self) -> usize {
        self.memory.get() as usize + N
    }

    /// Size in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Whether the stack has no room at all.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Entry point the trampoline calls, written by core 0 before the core starts.
static CORE1_ENTRY_FN: AtomicUsize = AtomicUsize::new(0);

/// Set once core 1 is executing Rust code.
static CORE1_ALIVE: AtomicBool = AtomicBool::new(false);

/// What core 1 actually starts at: it marks itself alive and calls the entry.
///
/// The chip starts a core at a bare address with no arguments, so the entry
/// point travels through [`CORE1_ENTRY_FN`] instead.
fn core1_trampoline() -> ! {
    CORE1_ALIVE.store(true, Ordering::Release);
    // SAFETY: `launch_core1` stores a `fn() -> !` here before the core starts,
    // and the core reads it once.
    let entry: fn() -> ! = unsafe { core::mem::transmute(CORE1_ENTRY_FN.load(Ordering::Acquire)) };
    entry()
}

/// Controller for the second HP core.
pub struct Multicore {
    lp_sys: LP_SYS,
    hp_sys: HP_SYS,
    pmu: PMU,
    clkrst: HP_SYS_CLKRST,
}

impl Multicore {
    /// Takes the peripherals that start core 1.
    ///
    /// The PMU and the clock/reset block are used by other drivers too — this
    /// only touches them inside [`launch_core1`](Self::launch_core1) and
    /// [`reset_core1`](Self::reset_core1), so the caller can hand them back with
    /// [`release`](Self::release) as soon as the core is up.
    ///
    /// # Panics
    ///
    /// Panics when called from core 1, which has no second core to start.
    pub fn init(lp_sys: LP_SYS, hp_sys: HP_SYS, pmu: PMU, clkrst: HP_SYS_CLKRST) -> Self {
        assert_eq!(
            CoreId::current(),
            CoreId::Core0,
            "Multicore must be created on core 0"
        );
        Self {
            lp_sys,
            hp_sys,
            pmu,
            clkrst,
        }
    }

    /// Starts `entry` on core 1 with `stack`.
    ///
    /// Writes the core-1 boot address, opens its clock gate, releases it from
    /// reset and clears the stall the boot path left on it, in that order: the
    /// core starts executing as soon as the reset goes away, so everything it
    /// needs has to be in place before that write.
    ///
    /// Returns once core 1 has reached `entry`.
    ///
    /// # Safety
    ///
    /// - `stack` must be a `static`, and nothing else may use it.
    /// - `entry` must never return.
    /// - Core 1 must not have been started already: the platform hands out one
    ///   set of peripherals, and a second start would run two cores on one set.
    pub unsafe fn launch_core1<const N: usize>(
        &mut self,
        entry: fn() -> !,
        stack: &'static Stack<N>,
    ) {
        assert!(
            N >= 256,
            "core 1 needs at least 256 bytes of stack, {} given",
            N
        );
        assert_eq!(
            CoreId::current(),
            CoreId::Core0,
            "core 1 has to be started from core 0"
        );
        assert!(!self.is_core1_running(), "core 1 is already running");

        CORE1_ALIVE.store(false, Ordering::Release);
        CORE1_ENTRY_FN.store(entry as usize, Ordering::Release);
        // SAFETY: the caller guarantees the stack, and the trampoline never
        // returns because `entry` does not.
        unsafe { esp32p4_rt::set_core1_entry(core1_trampoline, stack.top()) };

        // Where the core starts. Writing this before the reset is released is
        // what makes the core come up in `_start_core1` instead of at whatever
        // the ROM's reset vector holds.
        self.lp_sys
            .boot_addr_hp_core1
            .write_boot_addr_hp_core1(esp32p4_rt::core1_boot_address() as u32);

        // Clock gate, CLIC clock and the global reset that holds the core.
        self.clkrst.soc_clk_ctrl0.set_reg_core1_cpu_clk_en();
        self.clkrst.soc_clk_ctrl0.set_reg_core1_clic_clk_en();
        self.clkrst.hp_rst_en0.clr_reg_rst_en_core1_global();

        // Releasing the reset is not enough: the boot path stalls the core
        // through the PMU, and the stall has to be lifted for it to fetch.
        self.pmu
            .cpu_sw_stall
            .write_hpcore1_sw_stall_code(UNSTALL_CODE);
        crate::spin::spin_wait("core 1 to leave the stall", || {
            !self.hp_sys.cpu_corestalled_st.reg_core1_corestalled_st()
        });

        crate::spin::spin_wait("core 1 to start", || CORE1_ALIVE.load(Ordering::Acquire));
    }

    /// Whether core 1 is out of reset and running.
    #[inline]
    pub fn is_core1_running(&self) -> bool {
        !self.hp_sys.cpu_corestalled_st.reg_core1_corestalled_st()
            && self.clkrst.soc_clk_ctrl0.reg_core1_cpu_clk_en()
            && !self.clkrst.hp_rst_en0.reg_rst_en_core1_global()
    }

    /// Puts core 1 back into reset.
    ///
    /// The core stops where it is; nothing is unwound, so this is for a core
    /// that is known to be idle.
    pub fn reset_core1(&mut self) {
        self.clkrst.hp_rst_en0.set_reg_rst_en_core1_global();
        self.pmu.cpu_sw_stall.write_hpcore1_sw_stall_code(PARK_CODE);
        CORE1_ALIVE.store(false, Ordering::Release);
    }

    /// Returns the peripherals handed to [`init`](Self::init).
    pub fn release(self) -> (LP_SYS, HP_SYS, PMU, HP_SYS_CLKRST) {
        (self.lp_sys, self.hp_sys, self.pmu, self.clkrst)
    }
}

/// Which core this code runs on.
#[inline]
pub fn core_id() -> CoreId {
    CoreId::current()
}

/// The HP system register block, which holds the inter-core triggers.
#[inline]
fn hp_sys() -> &'static esp32p4_pac::hp_sys::RegisterBlock {
    // SAFETY: a plain register block; nothing here owns state.
    unsafe { &*HP_SYS::PTR }
}

/// Raises the software interrupt of `target`.
///
/// The interrupt stays asserted until the target clears it, which is what makes
/// it usable as a flag as well as a poke: [`wait_for_signal`] clears it.
#[inline]
pub fn signal(target: CoreId) {
    let r = hp_sys();
    match target {
        CoreId::Core0 => r.cpu_intr_from_cpu0.set_cpu_intr_from_cpu_0(),
        CoreId::Core1 => r.cpu_intr_from_cpu1.set_cpu_intr_from_cpu_1(),
    }
}

/// Clears the software interrupt aimed at this core.
#[inline]
pub fn clear_signal() {
    let r = hp_sys();
    match CoreId::current() {
        CoreId::Core0 => r.cpu_intr_from_cpu0.clr_cpu_intr_from_cpu_0(),
        CoreId::Core1 => r.cpu_intr_from_cpu1.clr_cpu_intr_from_cpu_1(),
    }
}

/// Whether the other core has signalled this one.
#[inline]
pub fn signal_pending() -> bool {
    let r = hp_sys();
    match CoreId::current() {
        CoreId::Core0 => r.cpu_intr_from_cpu0.cpu_intr_from_cpu_0(),
        CoreId::Core1 => r.cpu_intr_from_cpu1.cpu_intr_from_cpu_1(),
    }
}

/// Spins until [`signal`] has been called for this core, then clears it.
///
/// For a core that has nothing else to do; a core with work to run in the
/// meantime routes [`CoreId::interrupt_source`] to a line and uses the interrupt
/// instead.
pub fn wait_for_signal() {
    crate::spin::spin_wait("an inter-core signal", signal_pending);
    clear_signal();
}

/// A mutual-exclusion lock shared between the cores.
///
/// The chip has no spinlock peripheral, so this is a test-and-set on an atomic
/// flag: correct across the cores (both are cache-coherent, and the `A`
/// extension's atomics reach memory), but it spins, so it must not be held while
/// waiting for the other core.
pub struct Mutex<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// SAFETY: the flag serialises access to `value`.
unsafe impl<T: Send> Sync for Mutex<T> {}
// SAFETY: access to `value` is exclusive while the guard lives.
unsafe impl<T: Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    /// Wraps `value` in a lock. Usable as the initialiser of a `static`.
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Takes the lock, spinning until it is free.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        MutexGuard { mutex: self }
    }

    /// Takes the lock if it is free right now.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| MutexGuard { mutex: self })
    }

    /// Returns the value.
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Returns a mutable reference, which needs no lock because it borrows the
    /// `Mutex` exclusively.
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

/// Exclusive access to a [`Mutex`]'s contents.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the guard holds the lock.
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the guard holds the lock.
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

/// Runs `f` with the lock held.
#[inline]
pub fn with_lock<T, R>(mutex: &Mutex<T>, f: impl FnOnce(&mut T) -> R) -> R {
    let mut guard = mutex.lock();
    f(&mut guard)
}
