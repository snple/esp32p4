//! Starts and stops an ADC scan from the ETM, without the CPU in the path.
//!
//! The ADC's digital controller has no "start" register the CPU has to write:
//! `ADC_TASK_START0` and `ADC_TASK_STOP0` in the event task matrix do the same
//! thing, so any peripheral event can decide when sampling runs. The system
//! timer's comparators are the event source that needs no wiring, and a pin
//! pair closes the loop through GPIO's own side of the matrix:
//!
//! 1. arm a 56-sample scan and let `COMP0` start it 10 ms later — the buffer
//!    must fill on its own, and the measured delay shows the timer did it;
//! 2. run a continuous stream and let `COMP1` stop it 2 ms later — frames must
//!    stop arriving;
//! 3. run a stream again and let the ADC's threshold monitor stop it, which is
//!    the matrix's other direction with no outside peripheral involved;
//! 4. reset the block and check that a plain scan still fills its buffer;
//! 5. let the ADC's `Started` event set GPIO2 through `GPIO_TASK_CH2_SET`;
//! 6. let GPIO3's rising edge start a scan through `GPIO_EVT_CH0_RISE_EDGE`;
//! 7. control: arming with nothing connected to `ADC_TASK_START0` must leave the
//!    buffer empty.
//!
//! GPIO's side of the matrix lives in `GPIO_EXT`, so a pin has to be routed
//! through `init_pad` and given an edge in `GPIO_PINn.INT_TYPE` before its
//! channel sees anything — `set_direction_out` alone leaves the input path
//! disconnected and no event is ever produced.
//!
//! The control sits behind an [`Adc::reset`] because a start task stays latched
//! in the controller: once `START0` has run it, the next `arm_scan` converts
//! without waiting for an event, and only the block reset clears that.

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
use esp32p4_hal::gpio::{EtmEvent, EtmTask, InterruptType, Pins};
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
/// Pin the ADC's start event drives through the matrix's task side.
const FLAG_PIN: u32 = 2;
/// Pin whose rising edge is the matrix's event side.
const TRIGGER_PIN: u32 = 3;

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

    // ---- 2. the ADC's own event stops the stream ---------------------------
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

    // ---- 3. the timer stops a continuous stream ----------------------------
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

    // ---- 4. the block reset leaves a usable driver -------------------------
    // `Adc::reset` pulses `RST_EN_ADC`, which drops every pattern, monitor and
    // filter setting. The driver has to bring the block back from scratch, so
    // a plain scan is the check that nothing was left half configured.
    adc.reset();
    let mut after_reset = [Sample::ZERO; SAMPLES];
    adc.scan(
        Unit::Adc1,
        &patterns,
        &rx,
        &mut after_reset,
        &mut scan_descriptors,
        TIMER_CYCLES,
    )
    .expect("a plain scan after the block reset failed");
    defmt::info!("after reset: a plain scan still fills {} samples", SAMPLES);

    // ---- 5. the ADC's start event drives a GPIO ---------------------------
    // So far the ADC only received tasks. Here it produces one: `ADC_EVT_STARTED`
    // goes through the matrix into `GPIO_TASK_CH2_SET`, so the pin rises when the
    // controller begins converting and no CPU code touches it in between.
    let mut flag = pins.pin::<FLAG_PIN>();
    flag.init_pad();
    flag.enable_input();
    flag.set_direction_out();
    flag.set_low();
    flag.bind_etm_task(FLAG_PIN as u8);

    let mut flag_channel = etm.free_channel().expect("no free ETM channel");
    flag_channel.connect(Event::Started, EtmTask::Set(FLAG_PIN as u8));
    flag_channel.enable(true);
    adc.scan(
        Unit::Adc1,
        &patterns,
        &rx,
        &mut after_reset,
        &mut scan_descriptors,
        TIMER_CYCLES,
    )
    .expect("the scan for the GPIO task failed");
    flag_channel.enable(false);
    flag.enable_etm_task(false);
    let raised = flag.get();
    defmt::info!("gpio task: the flag ended {}", raised);
    assert!(raised, "the ADC's start event did not set the pin");

    // ---- 6. a GPIO edge starts a scan -------------------------------------
    // The matrix's event side: the pin's rising edge is `GPIO_EVT_CH0_RISE_EDGE`
    // and the task is `ADC_TASK_START0`, so the CPU only has to move the pin.
    let mut trigger = pins.pin::<TRIGGER_PIN>();
    trigger.init_pad();
    trigger.enable_input();
    trigger.set_direction_out();
    trigger.set_low();
    trigger.set_interrupt_type(InterruptType::PositiveEdge);
    trigger.bind_etm_event(0);

    let mut trigger_channel = etm.free_channel().expect("no free ETM channel");
    trigger_channel.connect(EtmEvent::Rising(0), Task::StartScan);

    let mut edged = [Sample::ZERO; SAMPLES];
    let armed = adc
        .arm_scan(
            Unit::Adc1,
            &patterns,
            &rx,
            &mut edged,
            &mut scan_descriptors,
            TIMER_CYCLES,
        )
        .expect("arming the edge-triggered scan failed");
    trigger_channel.enable(true);
    trigger.set_high();
    armed
        .wait(50_000)
        .expect("the GPIO edge never started the scan");
    trigger_channel.enable(false);
    trigger.set_low();
    defmt::info!("gpio event: {} samples after the rising edge", SAMPLES);

    // ---- 7. control: arming alone does not start anything ------------------
    // `ADC_TASK_START0` leaves the controller's start condition behind: the
    // frame limit stops the conversions, but the controller is still "started",
    // so the next `arm_scan` — which resets the FSM to reprogram it — converts
    // again without waiting for a trigger. Only the block reset clears that,
    // which is also the cheapest way back to a known state for this control.
    adc.reset();

    // Arm with nothing connected to `ADC_TASK_START0`: the buffer must stay
    // empty until an event arrives.
    let mut still_idle = [Sample::ZERO; SAMPLES];
    let armed = adc
        .arm_scan(
            Unit::Adc1,
            &patterns,
            &rx,
            &mut still_idle,
            &mut scan_descriptors,
            TIMER_CYCLES,
        )
        .expect("arming the second scan failed");
    let idle_run = armed.wait(10_000);
    defmt::info!(
        "second arm, no trigger: {:?}",
        idle_run.map_or("stayed idle", |()| "ran on its own")
    );
    assert!(
        idle_run.is_err(),
        "the controller converted without a start event"
    );

    defmt::info!("ADC ETM self-check passed");

    loop {
        delay::delay_ms(1_000);
    }
}
