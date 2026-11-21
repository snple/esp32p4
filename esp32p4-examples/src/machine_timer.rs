//! Drives the CLINT machine timer — the per-core counter RISC-V has where ARM
//! has SysTick.
//!
//! The timer is core-local (each core has its own comparator and its own
//! interrupt, CLIC id 7) but the counter behind it is shared, and it advances
//! once per CPU cycle. The checks measure it against `SYSTIMER`, whose 16 MHz
//! comes from the crystal and is therefore an independent reference:
//!
//! - the tick rate matches `CPU_CLK` from the clock tree;
//! - `now()` only moves forward;
//! - the comparator interrupt fires, and how late it is can be measured;
//! - a periodic chain of comparator updates produces the expected number of
//!   ticks per second;
//! - writing the comparator reads back;
//! - the overflow flag is readable and clearable (the counter itself takes
//!   centuries to reach it).
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay::{self, Delay};
use esp32p4_hal::interrupt::Level;
use esp32p4_hal::machine_timer::{MachineTimer, ticks_per_ms};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Interrupts the handler has served.
static TICKS: AtomicU32 = AtomicU32::new(0);
/// Counter value the handler saw, for the latency measurement.
static SERVED_AT: AtomicU32 = AtomicU32::new(0);
/// Set when the handler ran at all.
static FIRED: AtomicBool = AtomicBool::new(false);
/// Counter value the comparator was armed for.
static ARMED_FOR: AtomicU32 = AtomicU32::new(0);
/// Ticks between two chain steps, in counter ticks.
static STEP: AtomicU32 = AtomicU32::new(0);
/// Whether the handler keeps re-arming.
static CHAIN: AtomicBool = AtomicBool::new(false);

/// Handles one comparator match.
///
/// The interrupt is level-triggered, so the handler acknowledges it by moving the
/// comparator: one step forward, or all the way to the next far target when the
/// chain is off.
fn on_tick() {
    // SAFETY: `init` has run and the handle only touches this core's comparator.
    let timer = unsafe { MachineTimer::steal() };
    let now = timer.now();
    TICKS.fetch_add(1, Ordering::Relaxed);
    SERVED_AT.store(now as u32, Ordering::Release);
    FIRED.store(true, Ordering::Release);

    if CHAIN.load(Ordering::Acquire) {
        let step = u64::from(STEP.load(Ordering::Acquire));
        timer.set_target(now + step);
    } else {
        timer.set_target(u64::MAX);
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init();
    let clock = ClockTree::init(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST);

    let timer = MachineTimer::init();
    timer.enable_interrupt(Level::LOW, on_tick);
    // SAFETY: the comparator line and its handler are installed.
    unsafe { esp32p4_hal::interrupt::enable() };

    defmt::info!(
        "machine timer: running {=bool}, cpu {=u32} Hz, {=u64} ticks/us",
        timer.is_running(),
        clock.cpu_hz(),
        ticks_per_ms(clock.cpu_hz()) / 1_000
    );

    let cpu_hz = clock.cpu_hz();
    loop {
        check_rate(&timer, &delay, cpu_hz);
        check_monotonic(&timer);
        check_interrupt(&timer, &delay, cpu_hz);
        check_periodic(&timer, &delay, cpu_hz);
        check_target(&timer);
        check_overflow(&timer);
        delay.delay_ms(1_000);
    }
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{=str}: {} ", label, if ok { "ok" } else { "FAIL" });
}

/// The counter advances at `CPU_CLK`, measured against `SYSTIMER`.
fn check_rate(timer: &MachineTimer, delay: &Delay, cpu_hz: u32) {
    // A long window keeps the few microseconds of sampling skew at either end
    // below the tolerance.
    let m0 = timer.now();
    let s0 = delay::micros();
    delay.delay_ms(50);
    let s1 = delay::micros();
    let m1 = timer.now();

    let m_ticks = m1.wrapping_sub(m0);
    let us = s1 - s0;
    let measured_hz = (u128::from(m_ticks) * 1_000_000 / u128::from(us)) as u64;
    // Both references come off different clocks, so allow half a percent for the
    // sampling edges at either end of the window.
    let off = measured_hz.abs_diff(u64::from(cpu_hz));
    let ok = off * 200 < u64::from(cpu_hz);
    defmt::info!(
        "rate: {=u64} ticks in {=u64} us = {=u64} Hz, cpu {=u32} Hz, off by {=u64} Hz",
        m_ticks,
        us,
        measured_hz,
        cpu_hz,
        off
    );
    report("rate", ok);
}

/// Reading the counter never goes backwards.
fn check_monotonic(timer: &MachineTimer) {
    let mut previous = timer.now();
    let mut regressions = 0u32;
    for _ in 0..100_000 {
        let now = timer.now();
        if now < previous {
            regressions += 1;
        }
        previous = now;
    }
    defmt::info!("monotonic: 100000 reads, {=u32} regressions", regressions);
    report("monotonic", regressions == 0);
}

/// One comparator match, and how late the handler was.
fn check_interrupt(timer: &MachineTimer, delay: &Delay, cpu_hz: u32) {
    FIRED.store(false, Ordering::Release);
    CHAIN.store(false, Ordering::Release);

    let target = timer.now() + ticks_per_ms(cpu_hz);
    ARMED_FOR.store(target as u32, Ordering::Release);
    timer.set_target(target);

    let mut waited = 0u32;
    while !FIRED.load(Ordering::Acquire) && waited < 2_000_000 {
        waited += 1;
        delay.delay_us(1);
    }
    let fired = FIRED.load(Ordering::Acquire);
    let served = SERVED_AT.load(Ordering::Acquire);
    // The counter is 64-bit and the store only kept the low half; the test arms
    // one millisecond ahead, so comparing low halves is enough.
    let late = served.wrapping_sub(target as u32);

    defmt::info!(
        "interrupt: fired {=bool}, armed for {=u32:#x}, served at {=u32:#x}, {=u32} ticks late",
        fired,
        target as u32,
        served,
        late
    );
    report("interrupt", fired && !timer.interrupt_pending());
}

/// Re-arming from the handler produces a tick at a steady interval.
fn check_periodic(timer: &MachineTimer, delay: &Delay, cpu_hz: u32) {
    let step = ticks_per_ms(cpu_hz);
    STEP.store(step as u32, Ordering::Release);
    TICKS.store(0, Ordering::Release);
    CHAIN.store(true, Ordering::Release);
    timer.set_target(timer.now() + step);

    let start = delay::micros();
    delay.delay_ms(20);
    let elapsed_us = delay::micros() - start;
    CHAIN.store(false, Ordering::Release);
    // Acknowledge the last one so the level-triggered interrupt is not left
    // asserting.
    timer.set_target(u64::MAX);

    let ticks = TICKS.load(Ordering::Acquire);
    // One tick per millisecond over the measured window, give or take the ends.
    let expected = (elapsed_us / 1_000) as u32;
    let ok = ticks + 1 >= expected && ticks <= expected + 1;
    defmt::info!(
        "periodic: {=u32} ticks in {=u64} us, expected about {=u32}",
        ticks,
        elapsed_us,
        expected
    );
    report("periodic", ok);
}

/// The comparator reads back what was written to it.
fn check_target(timer: &MachineTimer) {
    let wanted = 0x0000_0123_4567_89ABu64;
    timer.set_target(wanted);
    let read_back = timer.target();
    timer.set_target(u64::MAX);
    defmt::info!("target: wrote {=u64:#x}, read {=u64:#x}", wanted, read_back);
    report("target", read_back == wanted);
}

/// The overflow flag is clear on a counter that has never wrapped, and clearing
/// it sticks.
fn check_overflow(timer: &MachineTimer) {
    let before = timer.overflowed();
    timer.clear_overflow();
    let after = timer.overflowed();
    defmt::info!(
        "overflow: set {=bool}, after clear {=bool}, counter {=u64:#x}",
        before,
        after,
        timer.now()
    );
    report("overflow", !before && !after);
}
