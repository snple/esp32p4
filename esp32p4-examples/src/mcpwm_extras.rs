//! Drives the MCPWM extras — sync, capture, fault trips, carrier, the ETM hook,
//! the event comparators and the interrupts — and checks each of them on the
//! board.
//!
//! Wiring: `GPIO2` carries `PWM0_CH0_A` and is fed back into `CAP0` and the RMT
//! receiver; `GPIO3` is driven by software and feeds `FAULT0`.
//!
//! Every check prints `ok` or `FAIL` on its own, so one broken feature still
//! leaves the others readable. The whole set repeats once a second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};
use esp32p4_hal::delay::Delay;

use esp32p4_hal::etm::Etm;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{Level, Pin, PinOps, Pins};
use esp32p4_hal::interrupt::{self, CpuInterrupt, Level as InterruptLevel};
use esp32p4_hal::main;
use esp32p4_hal::mcpwm::mc0::Event;
use esp32p4_hal::mcpwm::{
    CaptureConfig, CaptureEdge, CarrierConfig, CbcRefresh, ClockSource, CountDirection,
    EventComparator, FaultMask, GeneratorConfig, Mcpwm0, OperatorOutput, PwmAction, SyncOutSource,
    SyncSource, TimerConfig, TripConfig, TripMode,
};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::rmt::{ClockSource as RmtClock, Pulse, Rmt, RxChannelIndex, RxConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad that carries the PWM output and feeds `CAP0` plus the RMT receiver.
const PIN: u32 = 2;

/// Pad that feeds `FAULT0`.
const FAULT_PIN: u32 = 3;

/// PWM frequency.
const FREQUENCY: u32 = 20_000;

/// Half of the period, i.e. 50 % duty.
const DUTY: u16 = 999;

/// CPU line the MCPWM0 interrupt is routed to.
const MCPWM_CPU_LINE: u8 = 22;

/// Timer-zero interrupts seen by [`mcpwm_isr`].
static TICKS: AtomicU32 = AtomicU32::new(0);

fn mcpwm_isr() {
    // SAFETY: `Mcpwm0::init` has run; the ISR only reads and acknowledges.
    let mcpwm = unsafe { Mcpwm0::steal() };
    let event = mcpwm.timer::<0>().zero_event();
    if mcpwm.interrupt_pending(event) {
        mcpwm.clear_interrupt(event);
        TICKS.fetch_add(1, Ordering::Relaxed);
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::PWM0_CH0_A_PAD_OUT);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);
    pin.route_input_signal(InputSignal::PWM0_CAP0_PAD_IN);

    let mut fault_pin = pins.pin::<FAULT_PIN>();
    fault_pin.into_output();
    // Routing the input after `into_output` keeps the pad's input buffer on, so
    // the level also reaches `FAULT0` through the matrix.
    fault_pin.route_input_signal(InputSignal::PWM0_F0_PAD_IN);
    fault_pin.set_low();

    let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
    mcpwm
        .set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1)
        .unwrap();
    let timer0 = mcpwm.timer::<0>();
    let timer1 = mcpwm.timer::<1>();
    // Handles of different `N` are different types, so no loop over them.
    timer0
        .configure(&dp.HP_SYS_CLKRST, TimerConfig::new(FREQUENCY))
        .unwrap();
    timer1
        .configure(&dp.HP_SYS_CLKRST, TimerConfig::new(FREQUENCY))
        .unwrap();
    mcpwm
        .operator::<0>()
        .configure(&timer0, GeneratorConfig::edge_aligned(DUTY, DUTY));

    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, RmtClock::Xtal40m, 1)
        .unwrap();
    rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::new(1, 4_000))
        .unwrap();

    let etm = Etm::init(dp.SOC_ETM, &dp.HP_SYS_CLKRST);

    let period = u32::from(timer0.period());
    defmt::info!(
        "mcpwm extras: {=u32} Hz, period {=u32} ticks, capture timer on 40 MHz",
        FREQUENCY,
        period
    );

    loop {
        check_sync(&mcpwm, &dp.HP_SYS_CLKRST, &delay);
        check_capture(&mcpwm, &delay);
        check_fault(&mcpwm, &rmt, &pin, &fault_pin, &delay);
        check_carrier(&mcpwm, &rmt, &delay);
        check_etm(&mcpwm, &etm, &delay);
        check_event_comparators(&mcpwm, &etm, &delay);
        check_interrupt(&mcpwm, &delay);

        delay.delay_ms(1_000);
    }
}

/// Sync: timer 0's zero events reload timer 1's counter, so a counter that has
/// been left running for a while jumps back to the phase as soon as the sync
/// input is enabled.
fn check_sync(mcpwm: &Mcpwm0, clkrst: &esp32p4_hal::pac::HP_SYS_CLKRST, delay: &Delay) {
    const PHASE: u16 = 50;

    // A slow timer 1 (1 kHz) makes the jump unambiguous: it cannot wrap within
    // the window the check looks at.
    let timer0 = mcpwm.timer::<0>();
    let timer1 = mcpwm.timer::<1>();
    timer1.configure(clkrst, TimerConfig::new(1_000)).unwrap();
    timer0.sync_output(SyncOutSource::Zero);
    timer1.sync_input(Some(SyncSource::Timer0));
    timer1.set_sync_phase(PHASE, CountDirection::Up);

    timer0.stop();
    timer1.stop();
    timer1.enable_sync_input(false);
    timer0.start();
    timer1.start();

    // Let timer 1 wander off on its own, then arm the sync input.
    delay.delay_us(300);
    let free_running = timer1.counter();
    timer1.enable_sync_input(true);
    // One timer-0 period is 50 µs; wait a bit more so the reload has happened.
    delay.delay_us(80);
    let synced = timer1.counter();

    // A counter that is reloaded every timer-0 period (2000 ticks) cannot span
    // more than that, while a free-running one moves 40 MHz / 25 ns per tick.
    let mut lowest = u16::MAX;
    let mut highest = 0;
    for _ in 0..32 {
        let value = timer1.counter();
        lowest = lowest.min(value);
        highest = highest.max(value);
        delay.delay_us(3);
    }
    let span = u32::from(highest - lowest);

    // Restore the 20 kHz timer for the checks that follow.
    timer1
        .configure(clkrst, TimerConfig::new(FREQUENCY))
        .unwrap();

    let ok = free_running > 5_000
        && (PHASE..=PHASE + 2_000).contains(&synced)
        && synced < free_running
        && span <= 2_100;
    defmt::info!(
        "sync: timer1 free {=u16} -> reloaded {=u16} with phase {=u16}, spans {=u32} -> {}",
        free_running,
        synced,
        PHASE,
        span,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Capture: the PWM output loops back into `CAP0`, so successive rising-edge
/// captures are exactly one period apart.
fn check_capture(mcpwm: &Mcpwm0, delay: &Delay) {
    let timer = mcpwm.timer::<0>();
    let capture = mcpwm.capture::<0>();
    mcpwm.capture_timer().start();
    capture
        .configure(CaptureConfig {
            edge: CaptureEdge::Rising,
            prescale: 1,
            invert: false,
        })
        .unwrap();
    capture.enable(true);
    timer.start();

    // Poll for a few captured values and compare consecutive ones.
    let mut previous = capture.value();
    let mut worst = 0u32;
    let mut captures = 0u32;
    for _ in 0..400 {
        let value = capture.value();
        if value != previous {
            let delta = value.wrapping_sub(previous);
            // The first interval started mid-period, so it is not a full one.
            if captures > 0 {
                worst = worst.max(delta.abs_diff(2_000));
            }
            captures += 1;
            previous = value;
        }
        delay.delay_us(10);
    }
    capture.enable(false);
    timer.stop();

    let rising = capture.last_rising();
    let ok = captures >= 3 && worst <= 2 && rising;
    defmt::info!(
        "capture: {=u32} rising edges, delta off by at most {=u32} ticks, edge rising {=bool} -> {}",
        captures,
        worst,
        rising,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Fault trips: a pin-driven fault forces the outputs, and the trip is released
/// either by software (one-shot) or by the next zero event (cycle-by-cycle).
fn check_fault(
    mcpwm: &Mcpwm0,
    rmt: &Rmt,
    pin: &Pin<PIN>,
    fault_pin: &Pin<FAULT_PIN>,
    delay: &Delay,
) {
    let timer = mcpwm.timer::<0>();
    let operator = mcpwm.operator::<0>();
    mcpwm.fault::<0>().enable(true);

    // One-shot: hold the output high by force, trip it, then release it.
    timer.stop();
    operator.force(OperatorOutput::A, Level::High);
    operator.configure_trip(TripConfig {
        mode: TripMode::OneShot,
        sources: mcpwm.fault::<0>().mask(),
        software: false,
        output_a: PwmAction::Low,
        output_b: PwmAction::Low,
        refresh: CbcRefresh::Zero,
    });

    fault_pin.set_high();
    let forced_low = count_high(delay, || pin.get());
    let latched = operator.trip_status().one_shot;

    fault_pin.set_low();
    operator.release_trip();
    let released = operator.trip_status().one_shot;
    let back_high = count_high(delay, || pin.get());
    operator.release_force(OperatorOutput::A);
    // Disarm the one-shot trip again: leaving it armed would latch it a second
    // time when the same fault input trips the cycle-by-cycle test below.
    operator.configure_trip(TripConfig {
        mode: TripMode::OneShot,
        sources: FaultMask::NONE,
        software: false,
        output_a: PwmAction::Hold,
        output_b: PwmAction::Hold,
        refresh: CbcRefresh::Zero,
    });

    // Cycle-by-cycle: with the timer running the trip shows up as a flat low
    // level, and the next zero event hands the waveform back.
    operator.configure_trip(TripConfig {
        mode: TripMode::CycleByCycle,
        sources: mcpwm.fault::<0>().mask(),
        software: false,
        output_a: PwmAction::Low,
        output_b: PwmAction::Low,
        refresh: CbcRefresh::Zero,
    });
    timer.start();
    let (clean_high, _) = measure(rmt, delay);

    fault_pin.set_high();
    delay.delay_ms(1);
    let (tripped_high, _) = measure(rmt, delay);
    let cbc_on = operator.trip_status().cycle_by_cycle;

    fault_pin.set_low();
    delay.delay_ms(2);
    let (recovered_high, _) = measure(rmt, delay);
    let cbc_after = operator.trip_status().cycle_by_cycle;
    timer.stop();
    mcpwm.fault::<0>().disable();

    let ok = forced_low == 0
        && latched
        && !released
        && back_high == 16
        && clean_high.abs_diff(u32::from(DUTY)) <= 4
        && tripped_high == 0
        && cbc_on
        && !cbc_after
        && recovered_high.abs_diff(u32::from(DUTY)) <= 4;
    defmt::info!(
        "fault: ost low {=u32}/16 latched {=bool} released {=bool} high {=u32}/16, cbc {=u32} then {=u32} recovered {=u32} on {=bool} -> {}",
        forced_low,
        latched,
        released,
        back_high,
        clean_high,
        tripped_high,
        recovered_high,
        cbc_after,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Carrier: the High level of each PWM period turns into a burst of carrier
/// periods, so the measured level times shrink from ticks to a fraction of a
/// tick period.
fn check_carrier(mcpwm: &Mcpwm0, rmt: &Rmt, delay: &Delay) {
    // One carrier period is 8 * (prescale + 1) = 128 PWM_clk ticks, so a 4/8
    // duty shows up as 64 ticks high and 64 low.
    let timer = mcpwm.timer::<0>();
    let operator = mcpwm.operator::<0>();
    operator
        .configure_carrier(CarrierConfig {
            prescale: 16,
            duty: 4,
            first_pulse: 1,
            invert_input: false,
            invert_output: false,
        })
        .unwrap();
    timer.start();
    // Only the steady carrier pulses are interesting: the burst gaps and the
    // wider first pulse of each burst would drag the average around.
    let (carrier_high, carrier_low) = measure_short_pulses(rmt, delay, 100);

    operator.disable_carrier();
    delay.delay_ms(1);
    let (plain_high, _) = measure(rmt, delay);
    timer.stop();

    // Pulses at the burst boundaries are cut short, so compare the carrier
    // period rather than each level, and keep a floor on both.
    let carrier_period = carrier_high + carrier_low;
    let ok = (120..=136).contains(&carrier_period)
        && carrier_high >= 40
        && carrier_low >= 40
        && plain_high.abs_diff(u32::from(DUTY)) <= 4;
    defmt::info!(
        "carrier: high {=u32} low {=u32} (period {=u32}) with carrier, {=u32} without -> {}",
        carrier_high,
        carrier_low,
        carrier_period,
        plain_high,
        if ok { "ok" } else { "FAIL" }
    );
}

/// ETM: the timer-zero event triggers a capture through the matrix, so the
/// captured values advance one PWM period at a time.
fn check_etm(mcpwm: &Mcpwm0, etm: &Etm, delay: &Delay) {
    let timer = mcpwm.timer::<0>();
    let capture = mcpwm.capture::<1>();
    capture
        .configure(CaptureConfig {
            edge: CaptureEdge::Rising,
            prescale: 1,
            invert: false,
        })
        .unwrap();
    capture.enable(true);
    mcpwm.enable_event(timer.zero_event(), true);
    mcpwm.enable_task(capture.capture_task(), true);

    let mut channel = etm.free_channel().unwrap();
    channel.connect(timer.zero_event(), capture.capture_task());
    channel.enable(true);

    mcpwm.capture_timer().start();
    timer.start();
    delay.delay_ms(1);

    let armed = channel.is_enabled()
        && channel.event().raw() == 63
        && channel.task().raw() == 133
        && mcpwm.regs().evt_en.evt_timer0_tez_en()
        && mcpwm.regs().task_en.task_cap1_en();

    let mut previous = capture.value();
    let mut worst = 0u32;
    let mut captures = 0u32;
    for _ in 0..400 {
        let value = capture.value();
        if value != previous {
            worst = worst.max(value.wrapping_sub(previous).abs_diff(2_000));
            captures += 1;
            previous = value;
        }
        delay.delay_us(10);
    }

    channel.enable(false);
    mcpwm.enable_event(timer.zero_event(), false);
    mcpwm.enable_task(capture.capture_task(), false);
    capture.enable(false);
    timer.stop();

    let ok = armed && captures >= 3 && worst <= 2;
    defmt::info!(
        "etm: channel {=usize} armed {=bool}, {=u32} captures, delta off by at most {=u32} -> {}",
        channel.index(),
        armed,
        captures,
        worst,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Average high and low time of the pulses shorter than `limit` ticks, in
/// 40 MHz ticks.
fn measure_short_pulses(rmt: &Rmt, delay: &Delay, limit: u16) -> (u32, u32) {
    let mut pulses = [Pulse::default(); 16];
    rmt.clear_rx_done(RxChannelIndex::Zero);
    rmt.start_rx(RxChannelIndex::Zero);
    delay.delay_ms(3);
    rmt.stop_rx(RxChannelIndex::Zero);
    let count = rmt.read_pulses(RxChannelIndex::Zero, &mut pulses);

    let mut high_sum = 0u32;
    let mut high_count = 0u32;
    let mut low_sum = 0u32;
    let mut low_count = 0u32;
    for pulse in pulses[..count].iter().filter(|p| p.period <= limit) {
        match pulse.level {
            Level::High => {
                high_sum += u32::from(pulse.period);
                high_count += 1;
            }
            Level::Low => {
                low_sum += u32::from(pulse.period);
                low_count += 1;
            }
        }
    }

    (
        high_sum.checked_div(high_count).unwrap_or(0),
        low_sum.checked_div(low_count).unwrap_or(0),
    )
}

/// Event comparators: `E1`/`E2` fire at a counter value and only feed the ETM,
/// so a capture triggered by one lands exactly that many ticks after the
/// timer-zero capture.
fn check_event_comparators(mcpwm: &Mcpwm0, etm: &Etm, delay: &Delay) {
    let timer = mcpwm.timer::<0>();
    let capture1 = mcpwm.capture::<1>();
    let capture2 = mcpwm.capture::<2>();
    let config = CaptureConfig {
        edge: CaptureEdge::Rising,
        prescale: 1,
        invert: false,
    };
    capture1.configure(config).unwrap();
    capture2.configure(config).unwrap();
    capture1.enable(true);
    capture2.enable(true);
    mcpwm.enable_event(timer.zero_event(), true);
    // Each capture channel has its own task bit.
    mcpwm.enable_task(capture1.capture_task(), true);
    mcpwm.enable_task(capture2.capture_task(), true);

    let mut zero_channel = etm.channel(0).unwrap();
    zero_channel.connect(timer.zero_event(), capture1.capture_task());
    zero_channel.enable(true);

    mcpwm.capture_timer().start();
    timer.start();

    run_comparator(mcpwm, etm, delay, EventComparator::E1, 500);
    run_comparator(mcpwm, etm, delay, EventComparator::E2, 1_200);

    zero_channel.enable(false);
    mcpwm.enable_event(timer.zero_event(), false);
    mcpwm.enable_task(capture1.capture_task(), false);
    mcpwm.enable_task(capture2.capture_task(), false);
    capture1.enable(false);
    capture2.enable(false);
    timer.stop();
}

/// Points one comparator at a value, captures on it through the ETM and
/// compares that capture with the timer-zero one.
fn run_comparator(
    mcpwm: &Mcpwm0,
    etm: &Etm,
    delay: &Delay,
    comparator: EventComparator,
    value: u16,
) {
    let operator = mcpwm.operator::<0>();
    let capture1 = mcpwm.capture::<1>();
    let capture2 = mcpwm.capture::<2>();
    operator.set_event_comparator(comparator, value);
    operator.enable_event_comparator(comparator, true);

    let mut channel = etm.channel(1).unwrap();
    channel.connect(
        operator.event_comparator_id(comparator),
        capture2.capture_task(),
    );
    channel.enable(true);
    delay.delay_ms(1);

    // Both captures repeat once per period, so the difference modulo one period
    // is the fixed distance between the two events.
    let zero = capture1.value();
    let compared = capture2.value();
    let offset = phase_offset(compared, zero, 2_000);

    channel.enable(false);
    operator.enable_event_comparator(comparator, false);

    let ok = offset.abs_diff(u32::from(value)) <= 2;
    defmt::info!(
        "comparator at {=u16}: captured {=u32} ticks after the zero event -> {}",
        value,
        offset,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Interrupts: the timer-zero event drives the CPU line and the handler
/// acknowledges it, so the count has to match the elapsed time.
fn check_interrupt(mcpwm: &Mcpwm0, delay: &Delay) {
    mcpwm.clear_interrupts();
    let timer = mcpwm.timer::<0>();
    mcpwm.enable_interrupt(timer.zero_event(), true);

    // SAFETY: the line is not used by any other driver, and the source register
    // was cleared above so the first interrupt can only come from a new event.
    let handler = unsafe {
        interrupt::claim(
            Mcpwm0::SOURCE,
            CpuInterrupt::new(MCPWM_CPU_LINE),
            InterruptLevel::LOW,
            mcpwm_isr,
        )
    };
    handler.enable();
    // SAFETY: the MCPWM0 source is claimed and its handler is installed.
    unsafe { interrupt::enable() };

    let timer = mcpwm.timer::<0>();
    TICKS.store(0, Ordering::Relaxed);
    timer.start();
    // 10 ms at 20 kHz is 200 events.
    delay.delay_ms(10);
    let counted = TICKS.load(Ordering::Relaxed);

    // Mask the line so the handler stops acknowledging: the status bit then
    // stays latched until it is cleared.
    handler.disable();
    delay.delay_ms(1);
    let pending = mcpwm.interrupt_pending(timer.zero_event());
    mcpwm.clear_interrupts();
    let cleared = !mcpwm.interrupt_pending(timer.zero_event());
    timer.stop();
    // `Event::all()` walks the whole bit table, so a blanket disable is one
    // loop; the check only armed one event, the rest are already off.
    for event in Event::ALL {
        mcpwm.enable_interrupt(event, false);
    }
    handler.disable();

    let ok = (190..=210).contains(&counted) && pending && cleared;
    defmt::info!(
        "interrupt: {=u32}/200 timer-zero interrupts, pending {=bool}, cleared {=bool} -> {}",
        counted,
        pending,
        cleared,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Distance from `earlier` to `later` folded into `0..period`, for two
/// timestamps of a periodic signal.
fn phase_offset(later: u32, earlier: u32, period: u32) -> u32 {
    let diff = (i64::from(later) - i64::from(earlier)) % i64::from(period);
    if diff < 0 {
        (diff + i64::from(period)) as u32
    } else {
        diff as u32
    }
}

/// How many of 16 reads, 1 ms apart, see the pad high.
fn count_high(delay: &Delay, read: impl Fn() -> bool) -> u32 {
    let mut high = 0;
    for _ in 0..16 {
        delay.delay_ms(1);
        if read() {
            high += 1;
        }
    }
    high
}

/// Average high and low time of the pulses the RMT receiver sees, in 40 MHz
/// ticks.
fn measure(rmt: &Rmt, delay: &Delay) -> (u32, u32) {
    let mut pulses = [Pulse::default(); 16];
    rmt.clear_rx_done(RxChannelIndex::Zero);
    rmt.start_rx(RxChannelIndex::Zero);
    delay.delay_ms(3);
    rmt.stop_rx(RxChannelIndex::Zero);
    let count = rmt.read_pulses(RxChannelIndex::Zero, &mut pulses);
    // The first level is whatever the receiver caught mid-pulse.
    let pulses = if count > 2 {
        &pulses[1..count]
    } else {
        &pulses[..count]
    };

    let mut high_sum = 0u32;
    let mut high_count = 0u32;
    let mut low_sum = 0u32;
    let mut low_count = 0u32;
    for pulse in pulses.iter() {
        match pulse.level {
            Level::High => {
                high_sum += u32::from(pulse.period);
                high_count += 1;
            }
            Level::Low => {
                low_sum += u32::from(pulse.period);
                low_count += 1;
            }
        }
    }

    (
        high_sum.checked_div(high_count).unwrap_or(0),
        low_sum.checked_div(low_count).unwrap_or(0),
    )
}
