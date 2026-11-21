//! Starts the second HP core and checks that it really runs.
//!
//! Core 0 leaves reset at power-on and runs `#[main]`; core 1 sits in reset
//! until [`Multicore::launch_core1`] points it at the runtime's own entry, opens
//! its clock gate, releases the reset and lifts the PMU stall.
//!
//! Every round does four things:
//!
//! - the two cores increment a counter through a [`Mutex`] a fixed number of
//!   times each, and the total has to be exact — a broken lock loses updates;
//! - core 0 pokes core 1 over the inter-core interrupt register and core 1 pokes
//!   back, which exercises both directions of the trigger;
//! - both cores count each other's pokes from an interrupt handler, on a line
//!   each claimed on its own core;
//! - core 0 reads core 1's free-running counter to show it is still executing.
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_hal::delay::Delay;
use esp32p4_hal::interrupt::{self, CpuInterrupt, Level as InterruptLevel};
use esp32p4_hal::machine_timer::{MachineTimer, ticks_per_ms};
use esp32p4_hal::main;
use esp32p4_hal::multicore::{self, CoreId, Multicore, Mutex, Stack};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Stack for core 1, in `.bss`.
static CORE1_STACK: Stack<8192> = Stack::new();

/// Incremented through the lock by both cores.
static COUNTER: Mutex<u32> = Mutex::new(0);

/// Core 1's free-running proof of life.
static CORE1_LOOPS: AtomicU32 = AtomicU32::new(0);

/// How many times core 1's handler has run, i.e. how many pokes it took.
static CORE1_PINGS: AtomicU32 = AtomicU32::new(0);
/// How many of those found the source still asserted, i.e. what
/// [`multicore::signal_pending`] reports.
static CORE1_PINGS_ASSERTED: AtomicU32 = AtomicU32::new(0);
/// How many times core 0's handler has run, i.e. answers received.
static CORE0_IRQS: AtomicU32 = AtomicU32::new(0);
/// The core number core 1 read for itself.
static CORE1_ID: AtomicU32 = AtomicU32::new(u32::MAX);

/// Machine timer ticks each core has served, and the step each re-arms with.
///
/// The comparators are per core while the counter behind them is shared, so the
/// two periods below have to stay independent.
static TIMER_TICKS: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
static TIMER_STEP: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];

/// `CPU_CLK` on this board, which is also the machine timer's rate.
const CPU_HZ: u32 = 90_000_000;

/// Round handshake: core 1 counts while this is [`PHASE_WORK`].
const PHASE_IDLE: u32 = 0;
const PHASE_WORK: u32 = 1;
const PHASE_CLOCK: u32 = 3;
const PHASE_DONE: u32 = 2;
static PHASE: AtomicU32 = AtomicU32::new(PHASE_IDLE);

/// Clock reads that came back inconsistent, per core.
static CLOCK_TORN: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
/// Clock reads the two cores managed during the check.
static CLOCK_READS: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];

/// Increments per core per round.
const WORK: u32 = 100_000;

/// Clock reads per core while both cores hammer the counter.
const CLOCK_READS_PER_CORE: u32 = 200_000;

/// CPU interrupt line core 0 takes for the inter-core poke.
const CORE0_LINE: u8 = 20;
/// CPU interrupt line core 1 takes for the inter-core poke.
const CORE1_LINE: u8 = 21;

/// Core 0's handler: the poke arrives, count it and acknowledge.
fn core0_isr() {
    CORE0_IRQS.fetch_add(1, Ordering::Relaxed);
    multicore::clear_signal();
}

/// Core 1's handler: count the poke and answer it.
///
/// Counting here rather than in the loop is the point: with the interrupt
/// enabled the handler clears the register before a poll could see it.
fn core1_isr() {
    CORE1_PINGS.fetch_add(1, Ordering::Relaxed);
    if multicore::signal_pending() {
        CORE1_PINGS_ASSERTED.fetch_add(1, Ordering::Relaxed);
    }
    multicore::clear_signal();
    multicore::signal(CoreId::Core0);
}

/// Each core's own machine timer handler: count and re-arm.
fn core0_timer_isr() {
    // SAFETY: `MachineTimer::init` has run on this core.
    let timer = unsafe { MachineTimer::steal() };
    timer.set_target(timer.now() + u64::from(TIMER_STEP[0].load(Ordering::Acquire)));
    TIMER_TICKS[0].fetch_add(1, Ordering::Relaxed);
}

fn core1_timer_isr() {
    // SAFETY: `MachineTimer::init` has run on this core.
    let timer = unsafe { MachineTimer::steal() };
    timer.set_target(timer.now() + u64::from(TIMER_STEP[1].load(Ordering::Acquire)));
    TIMER_TICKS[1].fetch_add(1, Ordering::Relaxed);
}

/// Core 1's body.
fn core1_main() -> ! {
    // Its own CLIC lines are claimed on its own core, which is what the per-core
    // handler table buys.
    multicore::clear_signal();
    // SAFETY: this line is used by nobody else, and the source flag was cleared
    // above so the first interrupt can only come from a new poke.
    let handler = unsafe {
        interrupt::claim(
            CoreId::Core1.interrupt_source(),
            CpuInterrupt::new(CORE1_LINE),
            InterruptLevel::LOW,
            core1_isr,
        )
    };
    handler.enable();
    // SAFETY: the source is claimed and its handler installed.
    unsafe { interrupt::enable() };
    handler.leak();

    // Which core this code thinks it is: the runtime reads `mhartid`.
    CORE1_ID.store(CoreId::current().as_index() as u32, Ordering::Release);

    // This core's own comparator, on the counter core 0 started. Twice the
    // period of core 0's, so the counts have to come out in a 2:1 ratio.
    let timer = MachineTimer::init();
    TIMER_STEP[1].store(ticks_per_ms(CPU_HZ) as u32 * 2, Ordering::Release);
    timer.set_target(timer.now() + u64::from(TIMER_STEP[1].load(Ordering::Acquire)));
    timer.enable_interrupt(InterruptLevel::LOW, core1_timer_isr);

    loop {
        CORE1_LOOPS.fetch_add(1, Ordering::Relaxed);

        let phase = PHASE.load(Ordering::Acquire);
        if phase == PHASE_WORK {
            for _ in 0..WORK {
                *COUNTER.lock() += 1;
            }
            PHASE.store(PHASE_DONE, Ordering::Release);
        } else if phase == PHASE_CLOCK {
            read_clock_hard(CoreId::Core1.as_index());
            PHASE.store(PHASE_DONE, Ordering::Release);
        }
    }
}

/// Reads the shared clock back to back, counting reads that jump.
///
/// Both cores hammer the one snapshot register at the same time; the values have
/// to stay consistent and monotonic. This cannot reproduce the rare failure mode
/// — a snapshot torn across the low word's wrap, which needs two reads to
/// collide within 268 seconds of each other — but it does catch a read that
/// returns nonsense or runs backwards under contention.
fn read_clock_hard(core: usize) {
    let mut torn = 0u32;
    let mut reads = 0u32;
    let mut previous = esp32p4_hal::delay::micros();
    for _ in 0..CLOCK_READS_PER_CORE {
        let now = esp32p4_hal::delay::micros();
        // 1 ms of drift between two adjacent reads would already be absurd.
        if now < previous || now - previous > 1_000 {
            torn += 1;
        }
        previous = now;
        reads += 1;
    }
    CLOCK_TORN[core].store(torn, Ordering::Release);
    CLOCK_READS[core].store(reads, Ordering::Release);
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut mc = Multicore::init(dp.LP_SYS, dp.HP_SYS, dp.PMU, dp.HP_SYS_CLKRST);
    // Core 0's own line for the poke from core 1.
    multicore::clear_signal();
    // SAFETY: this line is used by nobody else, and the source flag was cleared
    // above so the first interrupt can only come from a new poke.
    let handler = unsafe {
        interrupt::claim(
            CoreId::Core0.interrupt_source(),
            CpuInterrupt::new(CORE0_LINE),
            InterruptLevel::LOW,
            core0_isr,
        )
    };
    handler.enable();
    // SAFETY: the source is claimed and its handler installed.
    unsafe { interrupt::enable() };
    handler.leak();

    defmt::info!(
        "multicore: running on core {}, core 1 running {=bool}",
        CoreId::current().as_index(),
        mc.is_core1_running()
    );

    // SAFETY: `CORE1_STACK` is a static used by nothing else, and `core1_main`
    // never returns.
    unsafe { mc.launch_core1(core1_main, &CORE1_STACK) };

    // Core 0's own comparator, one tick per millisecond.
    let timer = MachineTimer::init();
    TIMER_STEP[0].store(ticks_per_ms(CPU_HZ) as u32, Ordering::Release);
    timer.set_target(timer.now() + u64::from(TIMER_STEP[0].load(Ordering::Acquire)));
    timer.enable_interrupt(InterruptLevel::LOW, core0_timer_isr);

    check_core_id(&mc, &delay);
    loop {
        check_counters(&mc);
        check_signal(&delay);
        check_shared_clock();
        check_per_core_timers(&delay);
        check_identity_seen_by_core1(&delay);
        delay.delay_ms(1_000);
    }
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{=str}: {} ", label, if ok { "ok" } else { "FAIL" });
}

/// Only core 0 is running `main`.
fn check_core_id(mc: &Multicore, delay: &Delay) {
    let here = CoreId::current();
    // `launch_core1` returns as soon as core 1 reaches its entry, which is a few
    // instructions before the loop counter moves.
    let mut waited = 0u32;
    while CORE1_LOOPS.load(Ordering::Acquire) == 0 && waited < 50_000 {
        waited += 1;
        delay.delay_us(1);
    }
    let running = mc.is_core1_running();
    let alive = CORE1_LOOPS.load(Ordering::Acquire) != 0;
    defmt::info!(
        "identity: this core {}, core 1 out of reset {=bool}, core 1 executing {=bool}",
        here.as_index(),
        running,
        alive
    );
    report("identity", here == CoreId::Core0 && running && alive);
}

/// Both cores count through the lock, and nothing is lost.
fn check_counters(mc: &Multicore) {
    *COUNTER.lock() = 0;
    PHASE.store(PHASE_WORK, Ordering::Release);

    let before = CORE1_LOOPS.load(Ordering::Relaxed);
    for _ in 0..WORK {
        *COUNTER.lock() += 1;
    }
    // Bounded wait for core 1 to finish its half.
    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE {
        if waited == 20_000_000 {
            break;
        }
        waited += 1;
        core::hint::spin_loop();
    }
    let phase = PHASE.load(Ordering::Acquire);
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let total = *COUNTER.lock();
    let expected = 2 * WORK;
    let progressing = CORE1_LOOPS.load(Ordering::Relaxed) != before;
    defmt::info!(
        "counters: total {=u32} of {=u32}, core 1 finished {=bool}, core 1 still looping {=bool}, running {=bool}",
        total,
        expected,
        phase == PHASE_DONE,
        progressing,
        mc.is_core1_running()
    );
    report(
        "counters",
        total == expected && phase == PHASE_DONE && progressing,
    );
}

/// A poke crosses to core 1 and its answer comes back.
fn check_signal(delay: &Delay) {
    let pings_before = CORE1_PINGS.load(Ordering::Acquire);
    let irqs_before = CORE0_IRQS.load(Ordering::Acquire);

    multicore::signal(CoreId::Core1);

    let mut answered = false;
    for _ in 0..50_000 {
        if CORE1_PINGS.load(Ordering::Acquire) > pings_before
            && CORE0_IRQS.load(Ordering::Acquire) > irqs_before
        {
            answered = true;
            break;
        }
        // Core 1 answers with an interrupt, so let it run: the poke takes the
        // interrupt path on both cores.
        delay.delay_us(1);
    }

    let pings = CORE1_PINGS.load(Ordering::Acquire);
    let asserted = CORE1_PINGS_ASSERTED.load(Ordering::Acquire);
    defmt::info!(
        "signal: core 1 handler {=u32} (was {=u32}, {=u32} saw the flag), core 0 handler {=u32} (was {=u32}), answered {=bool}",
        pings,
        pings_before,
        asserted,
        CORE0_IRQS.load(Ordering::Relaxed),
        irqs_before,
        answered
    );
    // Every poke has to be visible as a pending flag inside the handler, which
    // is what makes the same register usable as a polled handshake.
    report("signal", answered && asserted == pings);
}

/// Core 1 reads a different `mhartid` than core 0.
fn check_identity_seen_by_core1(delay: &Delay) {
    let mut waited = 0u32;
    let mut seen = CORE1_ID.load(Ordering::Acquire);
    while seen == u32::MAX && waited < 50_000 {
        waited += 1;
        delay.delay_us(1);
        seen = CORE1_ID.load(Ordering::Acquire);
    }
    let ok = seen == CoreId::Core1.as_index() as u32 && multicore::core_id() == CoreId::Core0;
    defmt::info!(
        "core id: core 1 sees {=u32}, core 0 sees {=usize}",
        seen,
        multicore::core_id().as_index()
    );
    report("core id", ok);
}

/// Both cores read the shared clock at once, and it stays consistent.
fn check_shared_clock() {
    CLOCK_TORN[0].store(0, Ordering::Release);
    CLOCK_TORN[1].store(0, Ordering::Release);
    CLOCK_READS[0].store(0, Ordering::Release);
    CLOCK_READS[1].store(0, Ordering::Release);

    PHASE.store(PHASE_CLOCK, Ordering::Release);
    read_clock_hard(CoreId::Core0.as_index());

    let mut waited = 0u32;
    while PHASE.load(Ordering::Acquire) != PHASE_DONE {
        if waited == 50_000_000 {
            break;
        }
        waited += 1;
        core::hint::spin_loop();
    }
    let phase = PHASE.load(Ordering::Acquire);
    PHASE.store(PHASE_IDLE, Ordering::Release);

    let torn = CLOCK_TORN[0].load(Ordering::Acquire) + CLOCK_TORN[1].load(Ordering::Acquire);
    let reads = CLOCK_READS[0].load(Ordering::Acquire) + CLOCK_READS[1].load(Ordering::Acquire);
    defmt::info!(
        "shared clock: {=u32} reads from both cores, {=u32} inconsistent, core 1 finished {=bool}",
        reads,
        torn,
        phase == PHASE_DONE
    );
    report(
        "shared clock",
        torn == 0 && reads >= 2 * CLOCK_READS_PER_CORE && phase == PHASE_DONE,
    );
}

/// Each core's machine timer ticks on its own comparator.
///
/// Both run off the one shared counter, but the comparators, the interrupts and
/// the handler tables are per core: core 0 asks for a tick every millisecond and
/// core 1 every two, so twenty milliseconds of running has to produce about
/// twenty on one and ten on the other.
fn check_per_core_timers(delay: &Delay) {
    let before = [
        TIMER_TICKS[0].load(Ordering::Relaxed),
        TIMER_TICKS[1].load(Ordering::Relaxed),
    ];
    delay.delay_ms(20);
    let after = [
        TIMER_TICKS[0].load(Ordering::Relaxed),
        TIMER_TICKS[1].load(Ordering::Relaxed),
    ];

    let core0 = after[0].wrapping_sub(before[0]);
    let core1 = after[1].wrapping_sub(before[1]);
    // Twenty milliseconds: about 20 ticks at 1 ms and 10 at 2 ms, with one either
    // way for the edges of the window.
    let ok = (19..=21).contains(&core0) && (9..=11).contains(&core1);
    defmt::info!(
        "per-core timers: core 0 {=u32} ticks at 1 ms, core 1 {=u32} ticks at 2 ms, over 20 ms",
        core0,
        core1
    );
    report("per-core timers", ok);
}
