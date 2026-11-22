//! Streams ADC1 samples into a descriptor ring, with a PWM signal on the input.
//!
//! One jumper turns the ADC into a real measurement instead of floating noise:
//!
//! ```text
//!   GPIO20 LEDC (1 kHz square) ─────────► GPIO16 ADC1 channel 0
//! ```
//!
//! The digital controller converts one pattern entry every `SAMPLE_HZ`th of its
//! 20 MHz clock and hands each conversion to GDMA-AHB, which fills a ring of
//! eight frames. Nothing stops the stream: the ring wraps, and the driver hands
//! a frame back once its owner bit returns and re-arms it. The example drains the
//! ring for `RUN_MS` and then checks the stream itself:
//!
//! 1. the sample count over the measured time matches [`SAMPLE_HZ`];
//! 2. the samples see both levels of the square wave, at the duty cycle it was
//!    configured with;
//! 3. the edges in the stream reconstruct the PWM frequency, which ties the
//!    sampling rate and the sample order together;
//! 4. every sample reports the channel the pattern asked for.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::adc::{Adc, AdcInput, Attenuation, DIGI_CLK_HZ, Pattern, Sample, Unit};
use esp32p4_hal::clock::{ClockTree, RefClock};
use esp32p4_hal::delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::gpio::{OutputSignal, PinOps, Pins};
use esp32p4_hal::ledc::{ChannelConfig, ClockSource, Ledc, TimerConfig};
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad driven by the PWM, jumpered to [`ADC_PAD`].
const PWM_PAD: u32 = 20;
/// ADC1 pad that samples it, i.e. channel 0.
const ADC_PAD: u32 = 16;
/// Square-wave frequency on the PWM pad.
const PWM_HZ: u32 = 1_000;
/// Resolution of the PWM timer; only the duty cycle uses it.
const PWM_RESOLUTION: u32 = 8;

/// Conversions per second across all pattern entries.
const SAMPLE_HZ: u32 = 20_000;
/// Controller clock cycles between two conversions.
const TIMER_CYCLES: u32 = DIGI_CLK_HZ / SAMPLE_HZ;

/// Pattern entries per frame: the same channel eight times, so a frame is eight
/// consecutive samples of it.
const FRAME: usize = 8;
/// Frames the ring holds.
const FRAMES: usize = 8;
/// Samples the ring's buffer holds.
const RING_SAMPLES: usize = FRAME * FRAMES;

/// How long the drain loop runs.
const RUN_MS: u32 = 200;

/// Raw level separating the two halves of the square wave (12 dB attenuation).
const HIGH_RAW: u32 = 2_600;
/// Raw level below which a sample counts as the low half.
const LOW_RAW: u32 = 1_400;

static mut DESCRIPTORS: [Descriptor; FRAMES] = [const { Descriptor::new() }; FRAMES];
static mut RING: [Sample; RING_SAMPLES] = [Sample::from_raw(0); RING_SAMPLES];

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    // The clock tree first: the PWM runs from the 80 MHz reference, which has
    // to be switched on, and `delay::micros` starts at a conservative boot rate
    // that the machine timer has to be told about for the rate check to mean
    // anything.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    clocks.enable_ref(RefClock::F80M, true);
    let (clocks, _lp_aon) = clocks.release();

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &clocks);

    // The PWM drives the ADC input; the jumper between the two pads does the
    // rest, and the ADC pad itself stays a floating analogue input.
    let mut pwm_pin = pins.pin::<PWM_PAD>();
    pwm_pin.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT0);
    let ledc = Ledc::init(dp.LEDC, &clocks);
    ledc.set_clock_source(&clocks, ClockSource::Ref80m);
    let timer = ledc.timer::<0>();
    timer
        .configure(TimerConfig {
            clock: ClockSource::Ref80m,
            frequency: PWM_HZ,
            resolution: PWM_RESOLUTION,
        })
        .unwrap();
    timer.start();
    let channel = ledc.channel::<0>();
    channel.bind(&timer);
    channel.configure(ChannelConfig::new());
    channel.set_duty(1 << (PWM_RESOLUTION - 1));

    // The pad has to be in analogue mode before the SAR sees it; `AdcInput`
    // does that and hands back the channel and attenuation the pattern needs.
    let mut adc_pad = pins.pin::<ADC_PAD>();
    let input = AdcInput::new(&mut adc_pad, Attenuation::_12dB);

    let adc = Adc::init(dp.ADC, dp.LP_ADC, &clocks, &dp.LP_PERI);
    let dma = AhbDma::init(dp.AHB_DMA, &clocks);
    let rx = dma.rx_channel::<0>();

    // SAFETY: the statics are used by nothing else, and only one stream runs.
    let descriptors = unsafe { &*core::ptr::addr_of!(DESCRIPTORS) };
    let ring = unsafe { &mut *core::ptr::addr_of_mut!(RING) };

    let patterns = [Pattern::new(input.channel(), input.attenuation()); FRAME];
    let mut scratch = [Sample::from_raw(0); FRAME];

    loop {
        let stream = adc
            .start_continuous(Unit::Adc1, &patterns, &rx, ring, descriptors, TIMER_CYCLES)
            .expect("starting the stream failed");
        defmt::info!(
            "streaming {} Hz over {} frames of {}, ring of {} samples",
            SAMPLE_HZ,
            FRAMES,
            FRAME,
            RING_SAMPLES
        );

        let (mut total, mut high, mut edges) = (0u64, 0u64, 0u64);
        let (mut lowest, mut highest) = (u32::MAX, 0u32);
        let mut previous_high = None;
        let mut channel_ok = true;
        let start = delay::micros();
        while delay::micros() - start < u64::from(RUN_MS) * 1_000 {
            let copied = stream
                .read_timeout(&mut scratch, 50_000)
                .expect("no frame arrived");
            for sample in &scratch[..copied] {
                channel_ok &= sample.channel() == input.channel();
                lowest = lowest.min(sample.value());
                highest = highest.max(sample.value());
                let is_high = match sample.value() {
                    value if value >= HIGH_RAW => true,
                    value if value <= LOW_RAW => false,
                    // In between is the edge itself; it tells nothing about
                    // level, so leave the state alone.
                    _ => continue,
                };
                if is_high {
                    high += 1;
                }
                if previous_high.is_some_and(|previous| previous != is_high) {
                    edges += 1;
                }
                previous_high = Some(is_high);
            }
            total += copied as u64;
        }
        let elapsed_us = delay::micros() - start;
        stream.stop();

        let measured_hz = total * 1_000_000 / elapsed_us.max(1);
        let duty_percent = high * 100 / total.max(1);
        // Two edges per period, and the stream runs at the sample rate.
        let pwm_hz = edges * 1_000_000 / (2 * elapsed_us.max(1));
        let rate_ok = measured_hz.abs_diff(u64::from(SAMPLE_HZ)) <= u64::from(SAMPLE_HZ) / 20;
        let duty_ok = duty_percent.abs_diff(50) <= 5;
        let pwm_ok = pwm_hz.abs_diff(u64::from(PWM_HZ)) <= u64::from(PWM_HZ) / 5;

        defmt::info!(
            "{} samples in {} us: {} Sa/s, {} % high, {} Hz PWM, raw {}..{}, channel {} -> {}",
            total,
            elapsed_us,
            measured_hz,
            duty_percent,
            pwm_hz,
            lowest,
            highest,
            if channel_ok { "drained" } else { "WRONG" },
            if rate_ok && duty_ok && pwm_ok && channel_ok {
                "ok"
            } else {
                "FAIL"
            }
        );

        delay::delay_ms(500);
    }
}
