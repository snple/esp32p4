//! Starts and stops an ADC scan from the ETM, without the CPU in the path.
//!
//! The ADC's digital controller has no "start" register the CPU has to write:
//! `ADC_TASK_START0` and `ADC_TASK_STOP0` in the event task matrix do the same
//! thing, so any peripheral event can decide when sampling runs. This example
//! uses the system timer's comparators as the event source, which needs no
//! wiring:
//!
//! 1. arm a 56-sample scan, then let `COMP0` start it 10 ms later — the buffer
//!    must fill on its own, and measuring how long that took shows the delay
//!    really came from the timer;
//! 2. run a continuous stream, then let `COMP1` stop it 2 ms later — frames must
//!    stop arriving.
//!
//! The first section is a control: with the channel disarmed, arming a scan
//! must *not* start it. That ordering matters — `ADC_TASK_START0` latches, so
//! once the controller has run it starts converting on the next `arm_scan` by
//! itself (see [`Adc::arm_scan`]), and the control could never pass afterwards.
//!
//! `ADC_TASK_STOP0` is the reliable half: it stops the controller whether the
//! event comes from the system timer or from the ADC's own threshold monitor,
//! which is also how the matrix's other direction gets exercised without a
//! second peripheral — the ADC drives its own task.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::adc::{
    Adc, AdcInput, Attenuation, DIGI_CLK_HZ, Event, Filter, FilterCoefficient, FilterIndex,
    Monitor, MonitorIndex, Pattern, Sample, Task, Unit,
};
use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::etm::Etm;
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::systimer::{self, Systimer};
use esp32p4_hal::time::Duration;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad under test, ADC1 channel 0.
const ADC_PAD: u32 = 16;
/// Trigger period in controller clocks: one sample every 50 us.
const TIMER_CYCLES: u32 = DIGI_CLK_HZ / 20_000;
/// Samples one armed scan collects.
const SAMPLES: usize = 56;
/// How long the timer waits before starting the scan.
const START_DELAY_US: u64 = 10_000;
/// How long the timer waits before stopping the stream.
const STOP_DELAY_US: u64 = 2_000;
/// Samples per frame in the stream.
const FRAME: usize = 2;
/// Frames in the stream's ring.
const FRAMES: usize = 8;

static mut DESCRIPTORS: [Descriptor; FRAMES] = [const { Descriptor::new() }; FRAMES];
static mut RING: [Sample; FRAME * FRAMES] = [Sample::ZERO; FRAME * FRAMES];

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &clocks);
    let adc = Adc::init(dp.ADC, dp.LP_ADC, &clocks, &dp.LP_PERI);
    let dma = AhbDma::init(dp.AHB_DMA, &clocks);
    let systimer = Systimer::init(dp.SYSTIMER);
    let etm = Etm::init(dp.SOC_ETM, &clocks);

    // `COMP0` and `COMP1` drive the matrix; the CPU interrupt stays masked, so
    // the events only travel through the ETM.
    systimer.unit0.start();
    systimer.enable_events(true);

    let mut pad = pins.pin::<ADC_PAD>();
    let input = AdcInput::new(&mut pad, Attenuation::_12dB);
    let patterns = [Pattern::new(input.channel(), input.attenuation())];
    let rx = dma.rx_channel::<0>();

    let mut scan_descriptors = [const { Descriptor::new() }; 1];

    // ---- 0. control: arming alone must not start anything -----------------
    let mut control = [Sample::ZERO; SAMPLES];
    let armed = adc
        .arm_scan(
            Unit::Adc1,
            &patterns,
            &rx,
            &mut control,
            &mut scan_descriptors,
            TIMER_CYCLES,
        )
        .expect("arming the control scan failed");
    systimer.alarm0.set_target(
        systimer
            .unit0
            .now()
            .saturating_add(Duration::from_micros(1_000)),
    );
    let never_started = armed.wait(10_000);
    defmt::info!(
        "control: disarmed channel -> {:?}",
        never_started.map_or("nothing happened", |()| "the scan ran anyway")
    );
    assert!(
        never_started.is_err(),
        "the scan started with the ETM channel disarmed"
    );

    // ---- 1. the timer starts a one-shot scan -------------------------------
    let mut start_channel = etm.free_channel().expect("no free ETM channel");
    start_channel.connect(systimer::Event::Alarm0, Task::StartScan);

    let mut buffer = [Sample::ZERO; SAMPLES];
    let armed = adc
        .arm_scan(
            Unit::Adc1,
            &patterns,
            &rx,
            &mut buffer,
            &mut scan_descriptors,
            TIMER_CYCLES,
        )
        .expect("arming the scan failed");

    start_channel.enable(true);
    let armed_at = delay::micros();
    systimer.alarm0.set_target(
        systimer
            .unit0
            .now()
            .saturating_add(Duration::from_micros(START_DELAY_US)),
    );
    armed
        .wait(50_000)
        .expect("the timer never started the scan");
    let elapsed = delay::micros() - armed_at;
    // Drop the comparator's pending request before disarming, so nothing is
    // left pointing at `ADC_TASK_START0`.
    systimer.alarm0.clear();
    start_channel.enable(false);
    assert!(!start_channel.is_enabled());

    let low = buffer
        .iter()
        .map(|sample| sample.value())
        .min()
        .unwrap_or(0);
    let high = buffer
        .iter()
        .map(|sample| sample.value())
        .max()
        .unwrap_or(0);
    let conversion_us =
        u64::from(SAMPLES as u32) * u64::from(TIMER_CYCLES) * 1_000_000 / u64::from(DIGI_CLK_HZ);
    defmt::info!(
        "start: {} samples after {} us (timer {} us + conversion {} us), raw {}..{}",
        SAMPLES,
        elapsed,
        START_DELAY_US,
        conversion_us,
        low,
        high
    );
    assert!(
        elapsed >= START_DELAY_US,
        "the scan finished before the timer fired"
    );
    assert!(
        elapsed < START_DELAY_US + 4 * conversion_us,
        "the scan took too long to be the timer's doing"
    );

    // ---- 3. the ADC's own event stops the stream ---------------------------
    // The matrix also runs the other way: `ADC_EVT_EQ_ABOVE_THRESH0` is the
    // ADC's own threshold monitor, wired back to `ADC_TASK_STOP0`. A monitor
    // window of 0..=1 raw counts means every non-zero conversion trips it.
    adc.set_filter(
        FilterIndex::Zero,
        Unit::Adc1,
        Some(Filter {
            channel: input.channel(),
            coefficient: FilterCoefficient::_4,
        }),
    );
    adc.set_monitor(
        MonitorIndex::Zero,
        Unit::Adc1,
        Some(Monitor {
            channel: input.channel(),
            low: 0,
            high: 1,
        }),
    );

    let mut self_stop = etm.free_channel().expect("no free ETM channel");
    self_stop.connect(Event::AboveThreshold0, Task::Stop);
    self_stop.enable(true);

    let ring = unsafe { &mut *core::ptr::addr_of_mut!(RING) };
    let stream_descriptors = unsafe { &*core::ptr::addr_of!(DESCRIPTORS) };
    let stream_patterns = [Pattern::new(input.channel(), input.attenuation()); FRAME];
    let mut scratch = [Sample::ZERO; FRAME];
    let stream = adc
        .start_continuous(
            Unit::Adc1,
            &stream_patterns,
            &rx,
            ring,
            stream_descriptors,
            TIMER_CYCLES,
        )
        .expect("starting the threshold stream failed");
    let first = stream
        .read_timeout(&mut scratch, 50_000)
        .expect("the threshold stream produced no frame");
    delay::delay_ms(5);
    self_stop.enable(false);
    let mut drained = 0;
    while stream.read(&mut scratch) > 0 {
        drained += 1;
    }
    let after = stream.read(&mut scratch);
    stream.stop();
    adc.clear_monitor(MonitorIndex::Zero);
    adc.set_filter(FilterIndex::Zero, Unit::Adc1, None);
    adc.set_monitor(MonitorIndex::Zero, Unit::Adc1, None);

    defmt::info!(
        "self-stop: first frame {}, {} frames drained, {} samples after that",
        first,
        drained,
        after
    );
    assert_eq!(
        after, 0,
        "the stream kept running after its own threshold event"
    );

    // ---- 4. the timer stops a continuous stream ----------------------------
    let ring = unsafe { &mut *core::ptr::addr_of_mut!(RING) };
    let stream = adc
        .start_continuous(
            Unit::Adc1,
            &stream_patterns,
            &rx,
            ring,
            stream_descriptors,
            TIMER_CYCLES,
        )
        .expect("starting the timer-stop stream failed");

    let first = stream
        .read_timeout(&mut scratch, 50_000)
        .expect("the timer-stop stream produced no frame");
    defmt::info!("stream: first frame brought {} samples", first);

    let mut stop_channel = etm.free_channel().expect("no free ETM channel");
    stop_channel.connect(systimer::Event::Alarm1, Task::Stop);
    stop_channel.enable(true);
    systimer.alarm1.set_target(
        systimer
            .unit0
            .now()
            .saturating_add(Duration::from_micros(STOP_DELAY_US)),
    );
    // Give the stop event time to arrive, then drain whatever the engine had
    // already finished and look for anything after that.
    delay::delay_ms(5);
    stop_channel.enable(false);
    let mut drained = 0;
    while stream.read(&mut scratch) > 0 {
        drained += 1;
    }
    let after = stream.read(&mut scratch);
    stream.stop();

    defmt::info!(
        "stop: {} frames drained after the timer, {} samples after that",
        drained,
        after
    );
    assert_eq!(after, 0, "the stream kept producing frames after STOP0");

    defmt::info!("ADC ETM self-check passed");

    loop {
        delay::delay_ms(1_000);
    }
}
