//! Configures MCPWM0 for 20 kHz and reports what the hardware does.
//!
//! Output A is routed to `GPIO2` and the same pad is fed back into the RMT
//! receiver, so the measured high time can be compared with the programmed
//! comparator value.
//!
//! This example reports rather than asserts: on the current board the
//! configuration registers read back exactly as written (period, comparator,
//! generator actions, timer running) but neither output A nor a forced output
//! produces any level on the pad, so the measurement stays at zero. See the
//! MCPWM section of the README for the state of that investigation.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{Level, PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::mcpwm::{
    ClockSource, GeneratorConfig, Mcpwm0, OperatorIndex, TimerConfig, TimerIndex,
};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::rmt::{ClockSource as RmtClock, Pulse, Rmt, RxChannelIndex, RxConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by MCPWM output A and the RMT receiver.
const PIN: u32 = 2;

/// PWM frequency.
const FREQUENCY: u32 = 20_000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::PWM0_CH0_A_PAD_OUT);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);

    let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
    mcpwm
        .set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1)
        .unwrap();
    mcpwm
        .configure_timer(
            TimerIndex::Zero,
            TimerConfig {
                clock: ClockSource::Xtal40m,
                ..TimerConfig::new(FREQUENCY)
            },
        )
        .unwrap();

    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, RmtClock::Xtal40m, 2)
        .unwrap();
    // 2000 ticks = 100 µs, above the longest level of a 20 kHz period.
    rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::new(1, 2_000))
        .unwrap();

    let period = u32::from(mcpwm.period(TimerIndex::Zero));
    defmt::info!("mcpwm: {=u32} Hz, period {=u32} ticks", FREQUENCY, period);

    for percent in [25u32, 50, 75] {
        let duty = period * percent / 100;
        mcpwm
            .configure_generator(
                OperatorIndex::Zero,
                GeneratorConfig::edge_aligned(TimerIndex::Zero, duty as u16, 0),
            )
            .unwrap();

        mcpwm.start(TimerIndex::Zero);
        delay.delay_ms(1);
        let first = mcpwm.counter(TimerIndex::Zero);
        delay.delay_ms(1);
        let second = mcpwm.counter(TimerIndex::Zero);

        let mut pulses = [Pulse::default(); 8];
        rmt.clear_rx_done(RxChannelIndex::Zero);
        rmt.start_rx(RxChannelIndex::Zero);
        delay.delay_ms(3);
        rmt.stop_rx(RxChannelIndex::Zero);
        let count = rmt.read_pulses(RxChannelIndex::Zero, &mut pulses);
        mcpwm.stop(TimerIndex::Zero);

        let mut high_sum = 0u32;
        let mut high_count = 0u32;
        for pulse in pulses[..count].iter() {
            if matches!(pulse.level, Level::High) {
                high_sum += u32::from(pulse.period);
                high_count += 1;
            }
        }

        defmt::info!(
            "duty {=u32}%: comparator {=u32}, counter {=u16} -> {=u16}, high {=u32} RMT ticks over {=usize} pulses",
            percent,
            duty,
            first,
            second,
            high_sum.checked_div(high_count).unwrap_or(0),
            count
        );
    }

    loop {
        delay.delay_ms(1_000);
    }
}
