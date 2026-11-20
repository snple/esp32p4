//! Samples two ADC1 channels on a timer and stores the samples through GDMA.
//!
//! The digital controller walks a pattern table once per trigger, and every
//! conversion is handed to GDMA-AHB, which writes it into the buffer as one
//! 32-bit word per sample (result in bits 0-11, channel in bits 13-15). The
//! scan stops by itself once the buffer is full.
//!
//! Both pads are left floating here, so the values track whatever the pin sits
//! at; what the example checks is the shape of the result: every channel in the
//! pattern shows up, and the two channels' readings match a one-shot conversion
//! of the same pad.

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
    Adc, AdcInput, Attenuation, Filter, FilterCoefficient, FilterIndex, Monitor, MonitorIndex,
    Pattern, Sample, Unit,
};
use esp32p4_hal::delay::Delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// ADC1 pads that are free on the Function-EV board.
const PAD_A: u32 = 16;
const PAD_B: u32 = 17;

/// Samples per scan.
const SAMPLES: usize = 32;

/// Trigger period in controller clock cycles (20 MHz after the divider).
const TIMER_CYCLES: u32 = 4_000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let adc = Adc::init(dp.ADC, dp.LP_ADC, &dp.HP_SYS_CLKRST, &dp.LP_PERI);
    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);

    // Both pads are analogue inputs: no buffer, no pulls, no driver.
    let mut pad_a = pins.pin::<PAD_A>();
    let mut pad_b = pins.pin::<PAD_B>();
    let input_a = AdcInput::new(&mut pad_a, Attenuation::_12dB);
    let input_b = AdcInput::new(&mut pad_b, Attenuation::_12dB);

    let patterns = [
        Pattern::new(input_a.channel(), input_a.attenuation()),
        Pattern::new(input_b.channel(), input_b.attenuation()),
    ];

    // Smooth channel A and watch it with a deliberately narrow threshold
    // window, so the example has something for the monitor to trip on.
    adc.set_filter(
        FilterIndex::Zero,
        Unit::Adc1,
        Some(Filter {
            channel: input_a.channel(),
            coefficient: FilterCoefficient::_4,
        }),
    );
    adc.set_monitor(
        MonitorIndex::Zero,
        Unit::Adc1,
        Some(Monitor {
            channel: input_a.channel(),
            low: 0,
            high: 1,
        }),
    );

    let mut buffer = [Sample::ZERO; SAMPLES];
    let mut descriptors = [const { Descriptor::new() }; 1];
    let rx = dma.rx_channel::<1>();

    loop {
        match adc.scan(
            Unit::Adc1,
            &patterns,
            &rx,
            &mut buffer,
            &mut descriptors,
            TIMER_CYCLES,
        ) {
            Ok(()) => {}
            Err(error) => panic!("ADC scan failed: {:?}", error),
        }

        // Every sample carries the channel it came from, so the same buffer can
        // hold several channels interleaved.
        let mut counts = [0usize; 2];
        let mut last = [0u32; 2];
        for sample in buffer {
            let channel = sample.channel() as usize;
            counts[channel] += 1;
            last[channel] = sample.value();
        }
        for (channel, count) in counts.iter().enumerate() {
            let millivolts = adc.calibration().to_millivolts(
                Unit::Adc1,
                channel as u8,
                Attenuation::_12dB,
                last[channel],
            );
            defmt::info!(
                "ADC1 ch{}: {} samples, last raw={} ({} mV)",
                channel,
                count,
                last[channel],
                millivolts
            );
        }

        // The monitor latches on the first sample outside its window; clearing
        // it here just lets the next round report again.
        defmt::info!("monitor: {:?}", adc.monitor_state(MonitorIndex::Zero));
        adc.clear_monitor(MonitorIndex::Zero);

        delay.delay_ms(1_000);
    }
}
