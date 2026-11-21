//! Drives TWAI0 in self-test mode and checks what comes back.
//!
//! Self reception makes the controller receive the frame it just sent, and
//! self-test mode keeps the missing acknowledge from turning into error frames.
//! The receive side still comes from the RX input though, so the transmit level
//! has to come back — and that is done here without any hardware: `TWAI0_TX` is
//! routed out of `GPIO4` while `TWAI0_RX` is routed in from the same pad, so the
//! input buffer reads exactly what the pad drives.
//!
//! With a transceiver attached, route `TWAI0_RX` from the transceiver's `RXD` pad
//! instead and drop the loopback.
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

use esp32p4_hal::clock::XTAL_HZ;
use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::interrupt::{self, CpuInterrupt, Level as InterruptLevel};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::twai::{Config, Filter, Frame, Id, Interrupts, Mode, Timing, Twai0};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad carrying both `TWAI0_TX` and `TWAI0_RX`, which loops the bus back into
/// the controller.
const LOOP_PIN: u32 = 4;

/// Bit rate the checks run at.
const BITRATE: u32 = 500_000;

/// CPU line the TWAI0 interrupt is routed to.
const TWAI_CPU_LINE: u8 = 23;

/// Transmit interrupts seen by [`twai_isr`].
static TX_INTERRUPTS: AtomicU32 = AtomicU32::new(0);

/// Receive interrupts seen by [`twai_isr`].
static RX_INTERRUPTS: AtomicU32 = AtomicU32::new(0);

/// Identifier of the last frame the handler pulled out of the buffer.
static LAST_ID: AtomicU32 = AtomicU32::new(0);

fn twai_isr() {
    // SAFETY: `Twai0::init` has run; the handler only reads and acknowledges.
    let twai = unsafe { Twai0::steal() };
    // Reading the register clears every flag but the receive one, so this is
    // also the acknowledge.
    let fired = twai.interrupts();
    if fired.contains(Interrupts::RECEIVE)
        && let Ok(frame) = twai.receive()
    {
        LAST_ID.store(frame.id.value(), Ordering::Relaxed);
        RX_INTERRUPTS.fetch_add(1, Ordering::Relaxed);
    }
    if fired.contains(Interrupts::TRANSMIT) {
        TX_INTERRUPTS.fetch_add(1, Ordering::Relaxed);
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init();

    let mut loop_pin = pins.pin::<LOOP_PIN>();
    loop_pin.route_output_signal(OutputSignal::TWAI0_TX_PAD_OUT);
    loop_pin.route_input_signal(InputSignal::TWAI0_RX_PAD_IN);
    let _loop_pin = loop_pin;

    let twai = Twai0::init(dp.TWAI0, &dp.HP_SYS_CLKRST);
    let timing = Timing::for_bitrate(XTAL_HZ, BITRATE, 875).unwrap();
    defmt::info!(
        "twai: {=u32} bit/s, prescaler {=u16}, tseg1 {=u8}, tseg2 {=u8}, sjw {=u8}, sample point {=u16} permille",
        timing.bitrate(XTAL_HZ),
        timing.prescaler,
        timing.tseg1,
        timing.tseg2,
        timing.sjw,
        timing.sample_point()
    );

    twai.configure(&Config {
        timing,
        mode: Mode::SelfTest,
        filter: Filter::AcceptAll,
    });
    twai.start();

    loop {
        check_standard_frame(&twai, &delay);
        check_extended_frame(&twai, &delay);
        check_remote_frame(&twai, &delay);
        check_filter(&twai, &delay);
        check_interrupt(&twai, &delay);
        check_timing();
        check_rejects();

        let counters = twai.error_counters();
        defmt::debug!(
            "bus: {=?}, tec {=u8}, rec {=u8}, pending {=u8}",
            twai.bus_state(),
            counters.transmit,
            counters.receive,
            twai.frames_pending()
        );

        delay.delay_ms(1_000);
    }
}

/// A standard-id data frame comes back byte for byte.
fn check_standard_frame(twai: &Twai0, delay: &Delay) {
    let sent = Frame::data(Id::Standard(0x123), &[0x11, 0x22, 0x33]).unwrap();
    let received = round_trip(twai, delay, sent);

    let ok = match received {
        Some(frame) => frame.id == sent.id && frame.payload() == sent.payload() && !frame.remote,
        None => false,
    };
    defmt::info!(
        "standard frame: {=?} -> {}",
        received,
        if ok { "ok" } else { "FAIL" }
    );
}

/// The 29-bit path, with a full payload.
fn check_extended_frame(twai: &Twai0, delay: &Delay) {
    let payload = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let sent = Frame::data(Id::Extended(0x1AB_CDEF), &payload).unwrap();
    let received = round_trip(twai, delay, sent);

    let ok = match received {
        Some(frame) => {
            frame.id == sent.id && frame.payload() == payload && frame.len() == payload.len() as u8
        }
        None => false,
    };
    defmt::info!(
        "extended frame: {=?} -> {}",
        received,
        if ok { "ok" } else { "FAIL" }
    );
}

/// A remote frame keeps its requested length and carries no payload.
fn check_remote_frame(twai: &Twai0, delay: &Delay) {
    let sent = Frame::remote(Id::Standard(0x321), 4).unwrap();
    let received = round_trip(twai, delay, sent);

    let ok = match received {
        Some(frame) => frame.remote && frame.len() == 4 && frame.payload().is_empty(),
        None => false,
    };
    defmt::info!(
        "remote frame: {=?} -> {}",
        received,
        if ok { "ok" } else { "FAIL" }
    );
}

/// The acceptance filter lets the matching identifier through and drops the
/// neighbouring one.
fn check_filter(twai: &Twai0, delay: &Delay) {
    let timing = Timing::for_bitrate(XTAL_HZ, BITRATE, 875).unwrap();
    let config = |filter| Config {
        timing,
        mode: Mode::SelfTest,
        filter,
    };

    // Only 0x123 (all 11 bits) is accepted.
    twai.stop();
    twai.configure(&config(Filter::Standard {
        id: 0x123,
        mask: 0x7FF,
    }));
    twai.start();

    let accepted = round_trip(
        twai,
        delay,
        Frame::data(Id::Standard(0x123), &[0xA5]).unwrap(),
    )
    .is_some();
    let rejected = round_trip(
        twai,
        delay,
        Frame::data(Id::Standard(0x124), &[0xA5]).unwrap(),
    )
    .is_none();

    // Back to accepting everything for the checks that follow.
    twai.stop();
    twai.configure(&config(Filter::AcceptAll));
    twai.start();

    let ok = accepted && rejected;
    defmt::info!(
        "filter: 0x123 {=bool}, 0x124 {=bool} -> {}",
        accepted,
        !rejected,
        if ok { "ok" } else { "FAIL" }
    );
}

/// The timing search hits the common bit rates.
fn check_timing() {
    let mut ok = true;
    for (bitrate, sample_point) in [
        (1_000_000, 875),
        (500_000, 875),
        (250_000, 875),
        (125_000, 875),
    ] {
        match Timing::for_bitrate(XTAL_HZ, bitrate, sample_point) {
            Ok(timing) => {
                let realized = timing.bitrate(XTAL_HZ);
                let point = timing.sample_point();
                // The requested sample point is not always reachable from a
                // 40 MHz crystal, so 3 % off is fine.
                ok &= realized == bitrate && point.abs_diff(sample_point) <= 30;
                defmt::debug!(
                    "{=u32} bit/s: prescaler {=u16}, tseg1 {=u8}, tseg2 {=u8}, sample point {=u16} permille",
                    realized,
                    timing.prescaler,
                    timing.tseg1,
                    timing.tseg2,
                    point
                );
            }
            Err(_) => ok = false,
        }
    }
    defmt::info!(
        "timing: 1M/500k/250k/125k at 87.5 % -> {}",
        if ok { "ok" } else { "FAIL" }
    );
}

/// Frames that cannot exist are rejected before they reach the buffer, and a bit
/// rate the crystal cannot produce is refused instead of being approximated.
fn check_rejects() {
    let too_long = Frame::data(Id::Standard(0x100), &[0; 9]).is_err();
    let remote_too_long = Frame::remote(Id::Standard(0x100), 9).is_err();
    let standard_range = Frame::data(Id::Standard(0x800), &[]).is_err();
    let extended_range = Frame::data(Id::Extended(0x2000_0000), &[]).is_err();
    let bad_bitrate = Timing::for_bitrate(XTAL_HZ, 1_234_567, 875).is_err();

    let ok = too_long && remote_too_long && standard_range && extended_range && bad_bitrate;
    defmt::info!(
        "rejects: {=bool} {=bool} {=bool} {=bool} {=bool} -> {}",
        too_long,
        remote_too_long,
        standard_range,
        extended_range,
        bad_bitrate,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Sends `frame` with self-reception and waits for the looped-back copy.
fn round_trip(twai: &Twai0, delay: &Delay, mut frame: Frame) -> Option<Frame> {
    frame.self_reception = true;
    // Without an acknowledge the controller would retry forever.
    frame.single_shot = true;
    twai.transmit(&frame).ok()?;
    twai.wait_transmitted();

    // The loopback copy reaches the receive buffer within a few bit times.
    for _ in 0..500 {
        if twai.frames_pending() > 0 {
            return twai.receive().ok();
        }
        delay.delay_us(10);
    }
    None
}

/// Interrupts: the transmitter and the self-received frame both raise one, and
/// the handler finds them through the interrupt register.
fn check_interrupt(twai: &Twai0, delay: &Delay) {
    twai.disable_interrupts();
    // Drop anything the polling checks left behind, then clear the flags.
    while twai.frames_pending() > 0 {
        let _ = twai.receive();
    }
    let _ = twai.interrupts();

    twai.enable_interrupts(Interrupts::DATA);
    // SAFETY: the line is not used by anything else, and the peripheral status
    // was acknowledged above so the first interrupt can only come from a new
    // frame.
    let handler = unsafe {
        interrupt::claim(
            Twai0::SOURCE,
            CpuInterrupt::new(TWAI_CPU_LINE),
            InterruptLevel::LOW,
            twai_isr,
        )
    };
    TX_INTERRUPTS.store(0, Ordering::Relaxed);
    RX_INTERRUPTS.store(0, Ordering::Relaxed);
    LAST_ID.store(0, Ordering::Relaxed);
    handler.enable();
    // SAFETY: the TWAI0 source is claimed and its handler is installed.
    unsafe { interrupt::enable() };

    let mut frame = Frame::data(Id::Standard(0x055), &[0xDE, 0xAD]).unwrap();
    frame.self_reception = true;
    frame.single_shot = true;
    let queued = twai.transmit(&frame).is_ok();
    delay.delay_ms(2);

    handler.disable();
    twai.disable_interrupts();
    // Everything the handler acknowledged is gone; a flag here would mean one
    // was raised and never read.
    let quiet = twai.interrupts().is_empty();
    let transmitted = TX_INTERRUPTS.load(Ordering::Relaxed);
    let received = RX_INTERRUPTS.load(Ordering::Relaxed);
    let id = LAST_ID.load(Ordering::Relaxed);

    let ok = queued && transmitted >= 1 && received >= 1 && id == 0x055 && quiet;
    defmt::info!(
        "interrupt: {=u32} tx, {=u32} rx, last id {=u32:#x}, quiet {=bool} -> {}",
        transmitted,
        received,
        id,
        quiet,
        if ok { "ok" } else { "FAIL" }
    );
}
