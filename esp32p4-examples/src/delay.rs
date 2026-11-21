//! Busy-wait delays on both cores, and across a CPU frequency change.
//!
//! `delay::delay_us` / `delay::delay_ms` wait by reading this core's CLINT
//! counter, so they need no timer handle and two cores waiting at the same time do
//! not interfere — both are only reading a shared counter. The conversion from
//! microseconds to ticks uses `CPU_CLK`, which moves with the CPU divider.
//!
//! Three checks per round:
//!
//! - both cores wait 20 ms *at the same time*, and each one measures its own
//!   20 ms of wall-clock time;
//! - the same with core 1 starting first, so the two windows overlap without
//!   lining up;
//! - after doubling the CPU frequency, another 10 ms wait still measures 10 ms —
//!   the case a stale tick rate would get wrong.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_hal::clock::{ClockTree, SocDividers};
use esp32p4_hal::delay;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::multicore::{CoreId, Multicore, Stack};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Stack for core 1.
static CORE1_STACK: Stack<8192> = Stack::new();

/// Round handshake with core 1: idle, wait `WAIT_US`, done.
const PHASE_IDLE: u32 = 0;
const PHASE_WAIT: u32 = 1;
const PHASE_DONE: u32 = 2;
static PHASE: AtomicU32 = AtomicU32::new(PHASE_IDLE);

/// The wait core 1 is asked to make, in microseconds.
static WAIT_US: AtomicU32 = AtomicU32::new(0);

/// What each core measured, in microseconds.
static ELAPSED_US: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];

/// How much longer than asked a wait may come out.
///
/// Busy-waiting can only overshoot: it stops after the counter has advanced far
/// enough, and the read that notices costs a few ticks.
const TOLERANCE_US: u32 = 1_500;

/// Waits for `us` and reports the wall-clock time it took.
fn wait_and_measure(us: u32) -> u32 {
    let before = delay::micros();
    delay::delay_us(us);
    (delay::micros() - before) as u32
}

/// Core 1's body.
fn core1_main() -> ! {
    // This core's `mtimectl` is its own copy, so the sampling mode has to be set
    // here too — otherwise `micros` could read a torn 64-bit value.
    let _timer = MachineTimer::init(MachineTimer::cpu_hz());
    loop {
        if PHASE.load(Ordering::Acquire) == PHASE_WAIT {
            let elapsed = wait_and_measure(WAIT_US.load(Ordering::Acquire));
            ELAPSED_US[CoreId::Core1.as_index()].store(elapsed, Ordering::Release);
            PHASE.store(PHASE_DONE, Ordering::Release);
        }
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let mut mc = Multicore::init(dp.LP_SYS, dp.HP_SYS, dp.PMU, dp.HP_SYS_CLKRST);
    // SAFETY: `CORE1_STACK` is a static used by nothing else, and `core1_main`
    // never returns.
    unsafe { mc.launch_core1(core1_main, &CORE1_STACK) };
    let running = mc.is_core1_running();
    // The clock tree wants the clock/reset block back for the frequency check, and
    // `Multicore` is done with it now that core 1 is up.
    let (_, _, _, hp_clkrst) = mc.release();
    let clock = ClockTree::init(hp_clkrst, dp.LP_AON_CLKRST);
    let mut timer = MachineTimer::init(clock.cpu_hz());

    defmt::info!(
        "delay: cpu {=u32} Hz, core 1 running {=bool}",
        clock.cpu_hz(),
        running
    );

    loop {
        check_together();
        check_overlapping();
        check_frequency(&clock, &mut timer);
        delay::delay_ms(1_000);
    }
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{=str}: {} ", label, if ok { "ok" } else { "FAIL" });
}

/// Runs one round with core 1 and checks both cores came back on time.
fn run_round(us: u32, label: &str) -> bool {
    WAIT_US.store(us, Ordering::Release);
    PHASE.store(PHASE_WAIT, Ordering::Release);

    // Core 0 waits while core 1 does the same, so both spin at once.
    let elapsed0 = wait_and_measure(us);
    ELAPSED_US[CoreId::Core0.as_index()].store(elapsed0, Ordering::Release);

    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE && waited < 50_000_000 {
        waited += 1;
        core::hint::spin_loop();
    }
    let finished = PHASE.load(Ordering::Acquire) == PHASE_DONE;
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let elapsed1 = ELAPSED_US[CoreId::Core1.as_index()].load(Ordering::Acquire);

    // `micros` truncates, so the measured window can land a microsecond under.
    let long_enough = |elapsed: u32| elapsed + 5 >= us && elapsed <= us + TOLERANCE_US;
    let ok = finished && long_enough(elapsed0) && long_enough(elapsed1);
    defmt::info!(
        "{=str}: core 0 {=u32} us, core 1 {=u32} us, asked {=u32} us",
        label,
        elapsed0,
        elapsed1,
        us
    );
    ok
}

/// Both cores wait the same 20 ms window.
fn check_together() {
    let ok = run_round(20_000, "both cores waiting 20 ms");
    report("together", ok);
}

/// The windows overlap without lining up: core 1 starts first.
fn check_overlapping() {
    WAIT_US.store(12_000, Ordering::Release);
    PHASE.store(PHASE_WAIT, Ordering::Release);

    // Let core 1 get into its wait before core 0 starts its own, so the two
    // windows overlap but one finishes inside the other.
    delay::delay_ms(5);
    let elapsed0 = wait_and_measure(12_000);

    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE && waited < 50_000_000 {
        waited += 1;
        core::hint::spin_loop();
    }
    let finished = PHASE.load(Ordering::Acquire) == PHASE_DONE;
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let elapsed1 = ELAPSED_US[1].load(Ordering::Acquire);
    let on_time = |elapsed: u32| elapsed + 5 >= 12_000 && elapsed <= 12_000 + TOLERANCE_US;
    let ok = finished && on_time(elapsed0) && on_time(elapsed1);
    defmt::info!(
        "offset: core 1 started first, core 0 5 ms later: {=u32} us and {=u32} us of 12000 us",
        elapsed1,
        elapsed0
    );
    report("overlapping", ok);
}

/// The µs conversion follows the CPU frequency.
///
/// Alternates between 90 and 180 MHz so every round crosses a real clock change;
/// a tick rate baked into the code would wait twice as long (or half) on the round
/// after the switch. `set_cpu_hz` is what keeps the conversion honest — the counter
/// is the same, only its rate moved.
fn check_frequency(clock: &ClockTree, timer: &mut MachineTimer) {
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
    timer.set_cpu_hz(after);

    let ok = run_round(10_000, "after the CPU clock changed");
    defmt::info!(
        "frequency: cpu {=u32} Hz -> {=u32} Hz, 10 ms wait re-measured",
        before,
        after
    );
    report("frequency", ok && after != before);
}
