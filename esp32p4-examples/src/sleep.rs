//! Sleeps on both cores at once, and across a CPU frequency change.
//!
//! This is the regression test for the way sleeping used to work. `SYSTIMER` is
//! one peripheral for the whole chip, so parking a core on one of its comparators
//! meant sharing that comparator between the cores: whichever core woke up first
//! disabled the other's wakeup source, and the other core sat in `wfi` until some
//! unrelated interrupt arrived. Sleeping now parks each core on its own CLINT
//! machine timer comparator, so the two are independent — and because that
//! counter runs on CPU cycles, the µs conversion is calibrated against `SYSTIMER`
//! and re-derived whenever the clock tree moves.
//!
//! Three checks per round:
//!
//! - both cores sleep 20 ms *at the same time*, and each one measures its own
//!   20 ms of wall-clock time;
//! - the same, with core 1 starting a little later, so the windows overlap but do
//!   not line up;
//! - after doubling the CPU frequency, another 10 ms sleep on both cores still
//!   measures 10 ms — the case a hard-coded tick rate would get wrong.
//!
//! `SYSTIMER`, which is 16 MHz from the crystal and unaffected by the CPU
//! frequency, is the reference for all of it.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use esp32p4_hal::clock::{ClockTree, SocDividers};
use esp32p4_hal::delay::Delay;
use esp32p4_hal::main;
use esp32p4_hal::multicore::{CoreId, Multicore, Stack};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Stack for core 1.
static CORE1_STACK: Stack<8192> = Stack::new();

/// Round handshake with core 1: idle, sleep `SLEEP_US`, done.
const PHASE_IDLE: u32 = 0;
const PHASE_SLEEP: u32 = 1;
const PHASE_DONE: u32 = 2;
static PHASE: AtomicU32 = AtomicU32::new(PHASE_IDLE);

/// The sleep core 1 is asked to make, in microseconds.
static SLEEP_US: AtomicU32 = AtomicU32::new(0);

/// What each core measured, in microseconds of `SYSTIMER` time.
static ELAPSED_US: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
/// Whether each core's sleep reached its deadline.
static COMPLETED: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];

/// Tolerance for a 20 ms sleep: the wake-up crosses the interrupt path, and the
/// measurement brackets it with two `SYSTIMER` reads.
const TOLERANCE_US: u32 = 1_500;

/// Sleeps for `us` and reports the wall-clock time it took.
fn sleep_and_measure(us: u32) -> (u32, bool) {
    let delay = Delay::init();
    // The first sleep after a clock change also calibrates the machine timer
    // against `SYSTIMER`; warm that up so the measurement below is of the sleep.
    delay.sleep_us(1);
    let before = delay.now_us();
    let completed = delay.sleep_us(us);
    let after = delay.now_us();
    ((after - before) as u32, completed)
}

/// Core 1's body.
fn core1_main() -> ! {
    loop {
        if PHASE.load(Ordering::Acquire) == PHASE_SLEEP {
            let (elapsed, completed) = sleep_and_measure(SLEEP_US.load(Ordering::Acquire));
            ELAPSED_US[CoreId::Core1.as_index()].store(elapsed, Ordering::Release);
            COMPLETED[CoreId::Core1.as_index()].store(completed, Ordering::Release);
            PHASE.store(PHASE_DONE, Ordering::Release);
        }
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init();

    // Sleeping parks the core, which needs interrupts.
    // SAFETY: nothing is armed yet and every enabled source has a handler.
    unsafe { esp32p4_hal::interrupt::enable() };

    let mut mc = Multicore::init(dp.LP_SYS, dp.HP_SYS, dp.PMU, dp.HP_SYS_CLKRST);
    // SAFETY: `CORE1_STACK` is a static used by nothing else, and `core1_main`
    // never returns.
    unsafe { mc.launch_core1(core1_main, &CORE1_STACK) };
    let running = mc.is_core1_running();
    // The clock tree wants the clock/reset block back for the frequency check, and
    // `Multicore` is done with it now that core 1 is up.
    let (_, _, _, hp_clkrst) = mc.release();
    let clock = ClockTree::init(hp_clkrst, dp.LP_AON_CLKRST);

    defmt::info!(
        "sleep: cpu {=u32} Hz, core 1 running {=bool}",
        clock.cpu_hz(),
        running
    );

    loop {
        check_together();
        check_overlapping(&delay);
        check_frequency(&clock);
        delay.delay_ms(1_000);
    }
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{=str}: {} ", label, if ok { "ok" } else { "FAIL" });
}

/// Runs one sleep round with core 1 and checks both cores came back on time.
fn run_round(us: u32, label: &str) -> bool {
    SLEEP_US.store(us, Ordering::Release);
    COMPLETED[0].store(false, Ordering::Release);
    COMPLETED[1].store(false, Ordering::Release);
    PHASE.store(PHASE_SLEEP, Ordering::Release);

    // Core 0 sleeps while core 1 does the same, so the two comparators are armed
    // at once.
    let (elapsed0, completed0) = sleep_and_measure(us);
    ELAPSED_US[CoreId::Core0.as_index()].store(elapsed0, Ordering::Release);
    COMPLETED[CoreId::Core0.as_index()].store(completed0, Ordering::Release);

    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE && waited < 50_000_000 {
        waited += 1;
        core::hint::spin_loop();
    }
    let finished = PHASE.load(Ordering::Acquire) == PHASE_DONE;
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let elapsed1 = ELAPSED_US[CoreId::Core1.as_index()].load(Ordering::Acquire);
    let completed1 = COMPLETED[CoreId::Core1.as_index()].load(Ordering::Acquire);

    // `now_us` truncates, so the measured window can land a microsecond under.
    let long_enough = |elapsed: u32| elapsed + 5 >= us && elapsed <= us + TOLERANCE_US;
    let ok = finished && completed0 && completed1 && long_enough(elapsed0) && long_enough(elapsed1);
    defmt::info!(
        "{=str}: core 0 {=u32} us (done {=bool}), core 1 {=u32} us (done {=bool}), asked {=u32} us",
        label,
        elapsed0,
        completed0,
        elapsed1,
        completed1,
        us
    );
    ok
}

/// Both cores sleep the same 20 ms window.
fn check_together() {
    let ok = run_round(20_000, "both cores sleeping 20 ms");
    report("together", ok);
}

/// The windows overlap without lining up: core 1 arms its comparator first.
fn check_overlapping(delay: &Delay) {
    SLEEP_US.store(12_000, Ordering::Release);
    COMPLETED[1].store(false, Ordering::Release);
    PHASE.store(PHASE_SLEEP, Ordering::Release);

    // Let core 1 get into its sleep before core 0 starts its own, so the two
    // comparators are armed at different times and one finishes inside the other.
    delay.delay_ms(5);
    let (elapsed0, completed0) = sleep_and_measure(12_000);

    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE && waited < 50_000_000 {
        waited += 1;
        core::hint::spin_loop();
    }
    let finished = PHASE.load(Ordering::Acquire) == PHASE_DONE;
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let elapsed1 = ELAPSED_US[1].load(Ordering::Acquire);
    let completed1 = COMPLETED[1].load(Ordering::Acquire);
    let on_time = |elapsed: u32| elapsed + 5 >= 12_000 && elapsed <= 12_000 + TOLERANCE_US;
    let ok = finished && completed0 && completed1 && on_time(elapsed0) && on_time(elapsed1);
    defmt::info!(
        "offset: core 1 armed first (done {=bool}), core 0 5 ms later (done {=bool}): {=u32} us and {=u32} us of 12000 us",
        completed1,
        completed0,
        elapsed1,
        elapsed0
    );
    report("overlapping", ok);
}

/// The µs conversion follows the CPU frequency.
///
/// Alternates between 90 and 180 MHz so every round crosses a real clock change;
/// a tick rate baked into the code would sleep twice as long (or half) on the
/// round after the switch.
fn check_frequency(clock: &ClockTree) {
    let before = clock.cpu_hz();
    let dividers = if before >= 180_000_000 {
        // Back to the boot chain: CPU 90, MEM 90, SYS 90, APB 90.
        SocDividers {
            cpu: 4,
            mem: 1,
            sys: 1,
            apb: 1,
        }
    } else {
        // CPU 180, MEM 180, SYS 180, APB 90.
        SocDividers {
            cpu: 2,
            mem: 1,
            sys: 1,
            apb: 2,
        }
    };
    // SAFETY: both chains keep MEM and SYS at or below 180 MHz and APB at or below
    // 90 MHz, inside their limits, and the flash clock has its own divider so it
    // does not move with the CPU. Verified on this board.
    unsafe { clock.set_dividers(dividers) };
    let after = clock.cpu_hz();

    let ok = run_round(10_000, "after the CPU clock changed");
    defmt::info!(
        "frequency: cpu {=u32} Hz -> {=u32} Hz, 10 ms sleep re-measured",
        before,
        after
    );
    report("frequency", ok && after != before);
}
