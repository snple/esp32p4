//! Runs ADC2 through all three sampling paths.
//!
//! ADC2 has six channels on GPIO49-GPIO54 (the last pad has no IO_MUX register
//! in this chip's map, so five are reachable). The pads are left floating, which
//! is enough to check that the paths work:
//!
//! 1. one-shot conversions of every reachable pad;
//! 2. a two-channel pattern scan through GDMA-AHB, checking that both channels
//!    arrive, interleaved, and that the sample's channel field carries ADC2's
//!    *internal* channel numbers 2 and 3;
//! 3. a continuous stream for 200 ms, checking the sample rate matches the timer
//!    period and that the channel field is right there too.
//!
//! The third part is what the driver used to refuse: continuous mode was
//! documented as ADC1-only, and ADC2's was said to be disabled by an errata.
//! On this board it runs exactly like ADC1, at the same rate.

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
use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Samples a scan collects.
const SCAN_SAMPLES: usize = 32;
/// Trigger period in controller clocks, for 20 kSa/s.
const TIMER_CYCLES: u32 = DIGI_CLK_HZ / 20_000;
/// Samples per frame in the stream: one pass over a two-entry table.
const FRAME: usize = 2;
/// Frames the ring holds.
const FRAMES: usize = 8;
/// How long the stream runs.
const RUN_US: u64 = 200_000;
/// ADC2's channels arrive as the internal channels 2..8.
const INTERNAL_CHANNEL_BASE: u8 = 2;

static mut DESCRIPTORS: [Descriptor; FRAMES] = [const { Descriptor::new() }; FRAMES];
static mut RING: [Sample; FRAME * FRAMES] = [Sample::ZERO; FRAME * FRAMES];

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    // The rate check converts a sample count into a sample rate, so the machine
    // timer has to know the CPU clock.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &clocks);
    let adc = Adc::init(dp.ADC, dp.LP_ADC, &clocks, &dp.LP_PERI);
    let dma = AhbDma::init(dp.AHB_DMA, &clocks);

    let mut pad0 = pins.pin::<49>();
    let mut pad1 = pins.pin::<50>();
    let mut pad2 = pins.pin::<51>();
    let mut pad3 = pins.pin::<52>();
    let mut pad4 = pins.pin::<53>();
    let input0 = AdcInput::new(&mut pad0, Attenuation::_12dB);
    let inputs = [
        input0,
        AdcInput::new(&mut pad1, Attenuation::_12dB),
        AdcInput::new(&mut pad2, Attenuation::_12dB),
        AdcInput::new(&mut pad3, Attenuation::_12dB),
        AdcInput::new(&mut pad4, Attenuation::_12dB),
    ];

    for input in &inputs {
        let raw = adc.read_raw(input);
        let millivolts = adc.calibration().to_millivolts(
            Unit::Adc2,
            input.channel(),
            input.attenuation(),
            u32::from(raw),
        );
        defmt::info!(
            "ADC2 ch{}: one-shot raw={} ({} mV)",
            input.channel(),
            raw,
            millivolts
        );
    }

    // Two channels through the digital controller.
    let patterns = [
        Pattern::new(inputs[0].channel(), inputs[0].attenuation()),
        Pattern::new(inputs[1].channel(), inputs[1].attenuation()),
    ];
    let mut buffer = [Sample::ZERO; SCAN_SAMPLES];
    let mut scan_descriptors = [const { Descriptor::new() }; 1];
    let rx = dma.rx_channel::<0>();
    adc.scan(
        Unit::Adc2,
        &patterns,
        &rx,
        &mut buffer,
        &mut scan_descriptors,
        TIMER_CYCLES,
    )
    .expect("ADC2 scan failed");

    let mut scan_counts = [0usize; 2];
    for sample in &buffer {
        let channel = sample.channel();
        assert!(
            (INTERNAL_CHANNEL_BASE..INTERNAL_CHANNEL_BASE + 2).contains(&channel),
            "scan sample carried channel {}, not ADC2's internal 2 or 3",
            channel
        );
        scan_counts[usize::from(channel - INTERNAL_CHANNEL_BASE)] += 1;
    }
    assert_eq!(
        scan_counts,
        [SCAN_SAMPLES / 2; 2],
        "the two pattern entries did not alternate"
    );
    defmt::info!(
        "ADC2 scan: {} samples each on internal channels 2 and 3",
        SCAN_SAMPLES / 2
    );

    // And the same two channels, continuously.
    // SAFETY: the statics are used by nothing else, and only one stream runs.
    let descriptors = unsafe { &*core::ptr::addr_of!(DESCRIPTORS) };
    let ring = unsafe { &mut *core::ptr::addr_of_mut!(RING) };
    let mut scratch = [Sample::ZERO; FRAME];
    let stream = adc
        .start_continuous(Unit::Adc2, &patterns, &rx, ring, descriptors, TIMER_CYCLES)
        .expect("starting the ADC2 stream failed");

    let start = delay::micros();
    let mut total = 0usize;
    while delay::micros() - start < RUN_US {
        let copied = stream.read_timeout(&mut scratch, 50_000).unwrap_or(0);
        for sample in &scratch[..copied] {
            let channel = sample.channel();
            assert!(
                (INTERNAL_CHANNEL_BASE..INTERNAL_CHANNEL_BASE + 2).contains(&channel),
                "stream sample carried channel {}",
                channel
            );
        }
        total += copied;
    }
    let elapsed = delay::micros() - start;
    stream.stop();

    let rate = (total as u64 * 1_000_000 / elapsed) as u32;
    defmt::info!(
        "ADC2 continuous: {} samples in {} us -> {} Sa/s (wanted {})",
        total,
        elapsed,
        rate,
        DIGI_CLK_HZ / TIMER_CYCLES
    );
    assert!(
        rate.abs_diff(DIGI_CLK_HZ / TIMER_CYCLES) * 20 < DIGI_CLK_HZ / TIMER_CYCLES,
        "the stream ran at {} Sa/s, more than 5% off the timer period",
        rate
    );

    defmt::info!("ADC2 self-check passed");

    loop {
        delay::delay_ms(1_000);
    }
}
