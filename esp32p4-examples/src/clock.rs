//! Prints the clock tree and checks the two clock PLLs.
//!
//! ```text
//! cargo run --bin clock
//! ```
//!
//! The tree is printed first; the checks then move the CPU through the CPLL's
//! divider chain and compare the rate the machine timer actually counts against
//! `SYSTIMER`, whose 16 MHz come straight from the crystal. The last check puts
//! `PLL_F80M` — an SPLL output — on `GPIO2` through the LEDC and counts its
//! edges with the PCNT, which is clocked by APB (the CPLL). A wrong PLL divider,
//! an encoding the die does not understand or a calibration that never finished
//! therefore shows up as a FAIL instead of a plausible-looking number.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
// No `defmt::timestamp!` here: it would be derived from the machine timer, and
// this example is about moving the clock that timer counts.

use esp32p4_hal::clock::{ClockTree, HpSource, Pll, RefClock, RootSource};
use esp32p4_hal::delay::{self, DelayUs};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::ledc::{ChannelConfig, ClockSource, Ledc, TimerConfig};
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::pcnt::{ChannelConfig as PcntChannel, Pcnt};
use esp32p4_hal::regi2c::{self, Block};
use esp32p4_hal::systimer::{Systimer, TICK_HZ};
use esp32p4_hal::timer::TimerGroup0;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the LEDC output and the PCNT input.
const PIN: u32 = 2;
/// PWM frequency the LEDC is configured for.
const PWM_HZ: u32 = 100_000;
/// Timer resolution, so the divider is `80 MHz / (100 kHz × 2^6)`.
const PWM_RESOLUTION: u32 = 6;
/// `LEDC_CLK / (PWM × 2^resolution)` = 12.5, in 8.8 fixed point.
const PWM_DIVIDER: u32 = 3200;
/// Counting window for the edge measurement.
const WINDOW_MS: u32 = 100;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let systimer = Systimer::init(dp.SYSTIMER);
    systimer.unit0.start();

    // These drivers borrow the clock/reset peripheral, so they are built before
    // the tree takes it.
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT1);
    pin.route_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);
    let ledc = Ledc::init(dp.LEDC, &dp.HP_SYS_CLKRST);
    ledc.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Ref80m);
    let pwm = ledc.timer::<1>();
    pwm.configure(TimerConfig {
        clock: ClockSource::Ref80m,
        frequency: PWM_HZ,
        resolution: PWM_RESOLUTION,
    })
    .unwrap();
    let channel = ledc.channel::<1>();
    channel.bind(&pwm);
    channel.configure(ChannelConfig::new());
    channel.set_duty(1 << (PWM_RESOLUTION - 1));
    let pcnt = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
    let counter = pcnt.unit::<0>();
    counter.channel::<0>().configure(PcntChannel::rising_edge());
    counter.configure_filter(Some(10)).unwrap();
    counter.set_limits(0, 30_000);
    counter.set_thresholds(0, 0);
    counter.enable(esp32p4_hal::pcnt::Trigger::default());

    // The drivers take the clock tree where they used to take the HP registers:
    // `ClockTree` derefs to `HP_SYS_CLKRST`'s register block.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer_group = TimerGroup0::init(dp.TIMG0, &clocks);
    clocks.enable_ref(RefClock::F80M, true);
    let mut timer = MachineTimer::init(clocks.cpu_hz());

    print_tree(&clocks);
    for clk in RefClock::ALL {
        defmt::info!(
            "{:?}: {} Hz, divided by {} (enabled: {})",
            clk,
            clocks.ref_hz(clk),
            clocks.ref_divider(clk),
            clocks.ref_enabled(clk)
        );
    }
    for pll in [Pll::Cpll, Pll::Spll] {
        defmt::info!(
            "{:?}: {} Hz, analog divider x{}, calibration {:#04x}",
            pll,
            clocks.pll_hz(pll),
            clocks.pll_multiplier(pll),
            pll_status(pll)
        );
    }

    check_rate(&clocks, &systimer, &timer, "as booted");

    // Reprogram the CPLL at the frequency the die runs it at, the way a program
    // that wants a known analog configuration would. `ROOT_CLK` has to leave the
    // CPLL while its divider changes, and powering the analog block down wipes
    // the divider, so the sequence is: switch off the CPLL, power-cycle it, and
    // program it again.
    let cpll_mhz = clocks.pll_hz(Pll::Cpll) / 1_000_000;
    // SAFETY: the flash interface has its own clock and no PSRAM is mapped, so
    // neither cares what `ROOT_CLK` does.
    unsafe { clocks.set_root_source(RootSource::Xtal) };
    clocks.disable_pll(Pll::Cpll);
    let gated_off = !clocks.source_enabled(HpSource::Cpll);
    clocks.enable_pll(Pll::Cpll);
    let reset_multiplier = clocks.pll_multiplier(Pll::Cpll);
    // SAFETY: as above; the CPLL is off `ROOT_CLK` and powered.
    unsafe {
        clocks.set_pll_frequency(Pll::Cpll, cpll_mhz);
        clocks.set_cpu_frequency(cpll_mhz);
    }
    timer.set_cpu_hz(clocks.cpu_hz());
    defmt::info!(
        "CPLL power cycle: gate closed {}, reset divider x{}, now x{}",
        gated_off,
        reset_multiplier,
        clocks.pll_multiplier(Pll::Cpll)
    );
    check_rate(&clocks, &systimer, &timer, "after retuning the CPLL");

    // Walk the divider chain down and back up: every step is a real clock change
    // the machine timer has to follow.
    for step in [cpll_mhz / 2, cpll_mhz / 4, cpll_mhz] {
        // SAFETY: every chain `set_cpu_frequency` builds keeps MEM and SYS at or
        // below half the CPLL and APB at a quarter of it, inside their limits.
        unsafe { clocks.set_cpu_frequency(step) };
        timer.set_cpu_hz(clocks.cpu_hz());
        check_rate(&clocks, &systimer, &timer, "cpu divider chain");
    }

    // `PLL_F80M` comes off the SPLL; APB, which counts the edges, off the CPLL.
    counter.clear();
    pwm.start();
    systimer.unit0.delay_ms(WINDOW_MS);
    pwm.stop();
    let edges = counter.count().max(0) as u64;
    // One edge per PWM period: clock = edges / window × divider × 2^resolution.
    let measured_hz = (edges * 1_000 * u64::from(PWM_DIVIDER) * (1 << PWM_RESOLUTION)
        / (u64::from(WINDOW_MS) * 256)) as u32;
    let expected_hz = clocks.ref_hz(RefClock::F80M);
    let ok = measured_hz.abs_diff(expected_hz) * 100 < expected_hz;
    defmt::info!(
        "PLL_F80M from the SPLL: {} edges in {} ms = {} Hz, tree says {} Hz -> {}",
        edges,
        WINDOW_MS,
        measured_hz,
        expected_hz,
        if ok { "ok" } else { "FAIL" }
    );

    loop {
        systimer.unit0.delay_ms(1_000);
        print_tree(&clocks);
    }
}

/// Prints the root, bus and LP clocks.
fn print_tree(clocks: &ClockTree) {
    defmt::info!(
        "root {:?} {} MHz: cpu {} MHz, mem {} MHz, sys {} MHz, apb {} MHz",
        clocks.root_source(),
        clocks.root_hz() / 1_000_000,
        clocks.cpu_hz() / 1_000_000,
        clocks.mem_hz() / 1_000_000,
        clocks.sys_hz() / 1_000_000,
        clocks.apb_hz() / 1_000_000,
    );
    defmt::info!(
        "lp slow {:?} {} Hz, lp fast {:?} {} Hz, lp peri {} Hz",
        clocks.lp_slow_source(),
        clocks.lp_slow_hz(),
        clocks.lp_fast_source(),
        clocks.lp_fast_hz(),
        clocks.lp_peri_hz(),
    );
}

/// The calibration byte of a PLL: `OR_CAL_END`, `OR_LOCK` and the capacitance
/// the state machine settled on.
fn pll_status(pll: Pll) -> u8 {
    let block = match pll {
        Pll::Cpll => Block::CPU_PLL,
        Pll::Spll => Block::SYS_PLL,
    };
    regi2c::read_raw(block, 8)
}

/// Compares the machine timer's rate with the clock tree, over `SYSTIMER`.
fn check_rate(clocks: &ClockTree, systimer: &Systimer, timer: &MachineTimer, label: &str) {
    // Let the divider change settle before sampling.
    delay::delay_us(1_000);
    let cycles0 = timer.now();
    let us0 = systimer.unit0.ticks();
    systimer.unit0.delay_ms(50);
    let us1 = systimer.unit0.ticks();
    let cycles1 = timer.now();

    let cycles = cycles1.wrapping_sub(cycles0);
    let us = (us1 - us0) * 1_000_000 / u64::from(TICK_HZ);
    let measured_hz = (cycles * 1_000_000 / us) as u32;
    // The two references come off different clocks; allow for the sampling edges.
    let ok = measured_hz.abs_diff(clocks.cpu_hz()) * 200 < clocks.cpu_hz();
    defmt::info!(
        "{}: cpu {} Hz, measured {} Hz over {} us -> {}",
        label,
        clocks.cpu_hz(),
        measured_hz,
        us,
        if ok { "ok" } else { "FAIL" }
    );
}
