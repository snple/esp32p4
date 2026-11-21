//! Puts two TWAI controllers on one bus and checks that they talk to each other.
//!
//! The `twai` example runs a single controller in self-test mode, where the
//! missing acknowledge is faked by the controller itself. This one is a real
//! bus with two nodes in normal mode, so every frame has to be acknowledged by
//! the *other* controller. If acknowledge handling were wrong, the
//! transmitter's error counter would climb, the controller would go
//! error-passive and finally bus-off, and the checks below would say so.
//!
//! Wiring: `GPIO16`–`GPIO20` and `GPIO17`–`GPIO21` have to be jumpered.
//!
//! No transceiver is needed. A real bus is a wired-OR of every transmitter with
//! a single pull-up, so each jumper becomes such a bus: `TWAI0_TX` is routed out
//! of both `GPIO16` and `GPIO21`, `TWAI1_TX` out of both `GPIO17` and `GPIO20`,
//! and each controller reads its own pad back as its receive input. Both wires
//! then carry both nodes, and a node hears its own dominant bits the way it must
//! — a transmitter that only hears the peer never sees itself win arbitration
//! and drops off the bus on the first bit error.
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::clock::XTAL_HZ;
use esp32p4_hal::delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::twai::{BusState, Config, Filter, Frame, Id, Mode, Timing, Twai0, Twai1};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Carries `TWAI0_TX` out and `TWAI0_RX` back in.
const PIN_A: u32 = 16;
/// Carries `TWAI1_TX` out and `TWAI1_RX` back in.
const PIN_B: u32 = 17;
/// Drives `TWAI1_TX` into [`PIN_A`]'s wire.
const PIN_A_PEER: u32 = 20;
/// Drives `TWAI0_TX` into [`PIN_B`]'s wire.
const PIN_B_PEER: u32 = 21;

/// Bit rate both nodes run at.
const BITRATE: u32 = 500_000;
/// Sample point in permille, the CAN recommendation for a 40 MHz clock.
const SAMPLE_POINT: u16 = 875;

/// How long to wait for a frame that should arrive.
const ARRIVE_TIMEOUT_US: u32 = 2_000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // Each jumpered pair becomes one wired-OR bus carrying both transmitters.
    //
    // A CAN transmitter only pulls the bus dominant and leaves the recessive
    // level to a pull-up, so the pads are open-drain: two of them can share a
    // wire without fighting, exactly like the wired-OR bus a transceiver makes.
    let mut pin_a = pins.pin::<PIN_A>();
    pin_a.route_output_signal(OutputSignal::TWAI0_TX_PAD_OUT);
    pin_a.set_open_drain(true);
    pin_a.set_pull_up_enabled(true);
    pin_a.route_input_signal(InputSignal::TWAI0_RX_PAD_IN);

    let mut pin_b = pins.pin::<PIN_B>();
    pin_b.route_output_signal(OutputSignal::TWAI1_TX_PAD_OUT);
    pin_b.set_open_drain(true);
    pin_b.set_pull_up_enabled(true);
    pin_b.route_input_signal(InputSignal::TWAI1_RX_PAD_IN);

    let mut peer_a = pins.pin::<PIN_A_PEER>();
    peer_a.route_output_signal(OutputSignal::TWAI1_TX_PAD_OUT);
    peer_a.set_open_drain(true);
    peer_a.set_pull_up_enabled(true);

    let mut peer_b = pins.pin::<PIN_B_PEER>();
    peer_b.route_output_signal(OutputSignal::TWAI0_TX_PAD_OUT);
    peer_b.set_open_drain(true);
    peer_b.set_pull_up_enabled(true);

    let Ok(timing) = Timing::for_bitrate(XTAL_HZ, BITRATE, SAMPLE_POINT) else {
        defmt::info!("twai bus: timing rejected -> FAIL");
        loop {
            delay::delay_ms(1_000);
        }
    };

    let node_a = Twai0::init(dp.TWAI0, &dp.HP_SYS_CLKRST);
    let node_b = Twai1::init(dp.TWAI1, &dp.HP_SYS_CLKRST);

    // Both start in normal mode: no self-test, so the acknowledge has to come
    // from the other controller.
    node_a.configure(&Config {
        timing,
        mode: Mode::Normal,
        filter: Filter::AcceptAll,
    });
    node_b.configure(&Config {
        timing,
        mode: Mode::Normal,
        filter: Filter::AcceptAll,
    });
    node_a.start();
    node_b.start();

    defmt::info!(
        "twai bus: a stopped {=bool} txidle {=bool} state {=?} | b stopped {=bool} txidle {=bool} state {=?}",
        node_a.is_stopped(),
        node_a.status().transmit_idle,
        node_a.bus_state(),
        node_b.is_stopped(),
        node_b.status().transmit_idle,
        node_b.bus_state()
    );

    loop {
        check_standard_frame(&node_a, &node_b);
        check_extended_frame(&node_b, &node_a);
        check_remote_frame(&node_a, &node_b);
        check_filter(&node_a, &node_b);
        check_idle_bus(&node_a, &node_b);
        delay::delay_ms(1_000);
    }
}

/// Generates the one-way transfer helper for one pair of controllers.
///
/// The two controllers do not share a trait — that is deliberate, `Twai0` and
/// `Twai1` are separate types with their own inherent methods — so the helper is
/// written once here and expanded per direction, exactly like the driver's own
/// per-instance code.
macro_rules! transfer_body {
    ($name:ident, $Tx:ident, $Rx:ident, $wait:ident) => {
        /// A frame from `tx` reaches `rx` unchanged, and `tx` collects no error,
        /// which is only possible because `rx` acknowledged it.
        fn $name(tx: &$Tx, rx: &$Rx, sent: Frame, label: &str) -> Option<Frame> {
            // Drop anything left over from the previous round.
            while rx.receive().is_ok() {}

            if tx.transmit(&sent).is_err() {
                return None;
            }
            let done = $wait(tx, ARRIVE_TIMEOUT_US);

            for _ in 0..ARRIVE_TIMEOUT_US / 10 {
                if let Ok(frame) = rx.receive() {
                    let matched = frame.id == sent.id
                        && frame.remote == sent.remote
                        && (!sent.remote || frame.len() == sent.len())
                        && (sent.remote || frame.payload() == sent.payload());
                    let counters = tx.error_counters();
                    let acked = counters.transmit == 0 && tx.bus_state() == BusState::Active;
                    let ok = matched && acked && done;
                    defmt::info!(
                        "{=str}: content {=bool}, complete {=bool}, acknowledged {=bool}, tec {=u8} -> {}",
                        label,
                        matched,
                        done,
                        acked,
                        counters.transmit,
                        if ok { "ok" } else { "FAIL" }
                    );
                    return ok.then_some(frame);
                }
                delay::delay_us(10);
            }
            let counters = tx.error_counters();
            defmt::info!(
                "{=str}: nothing arrived, complete {=bool}, tec/rec {=u8}/{=u8} -> FAIL",
                label,
                done,
                counters.transmit,
                counters.receive
            );
            None
        }
    };
}

transfer_body!(transfer_ab, Twai0, Twai1, wait_a);
transfer_body!(transfer_ba, Twai1, Twai0, wait_b);

/// Generates the bounded wait for one controller.
///
/// [`wait_transmitted`](Twai0::wait_transmitted) spins until the frame is out,
/// which never happens if the bus has no other node to acknowledge it; a
/// diagnostic wants a timeout and the counters instead of a panic.
macro_rules! wait_body {
    ($name:ident, $Node:ident) => {
        /// Waits for the frame to leave the buffer, aborting it on timeout.
        fn $name(node: &$Node, timeout_us: u32) -> bool {
            for _ in 0..timeout_us / 10 {
                if node.status().transmission_complete {
                    return true;
                }
                delay::delay_us(10);
            }
            node.abort_transmission();
            false
        }
    };
}

wait_body!(wait_a, Twai0);
wait_body!(wait_b, Twai1);

/// A standard-id data frame goes from one node to the other.
fn check_standard_frame(a: &Twai0, b: &Twai1) {
    let sent = Frame::data(Id::Standard(0x123), &[0x11, 0x22, 0x33]).unwrap();
    transfer_ab(a, b, sent, "standard");
}

/// An extended-id data frame goes the other way.
fn check_extended_frame(b: &Twai1, a: &Twai0) {
    let sent = Frame::data(Id::Extended(0x1ABCDE), &[0xAA, 0x55]).unwrap();
    transfer_ba(b, a, sent, "extended");
}

/// A remote frame keeps its identifier and length and carries no data.
fn check_remote_frame(a: &Twai0, b: &Twai1) {
    let sent = Frame::remote(Id::Standard(0x321), 4).unwrap();
    transfer_ab(a, b, sent, "remote");
}

/// A filter that rejects the identifier keeps the frame out of the receiver —
/// and the transmitter still sees an acknowledge, because the hardware
/// acknowledges before the acceptance filter runs.
fn check_filter(a: &Twai0, b: &Twai1) {
    while b.receive().is_ok() {}
    b.configure(&Config {
        timing: Timing::for_bitrate(XTAL_HZ, BITRATE, SAMPLE_POINT).unwrap(),
        mode: Mode::Normal,
        filter: Filter::Standard {
            id: 0x555,
            mask: 0x7FF,
        },
    });
    b.start();

    let sent = Frame::data(Id::Standard(0x123), &[0x01]).unwrap();
    let queued = a.transmit(&sent).is_ok();
    wait_a(a, ARRIVE_TIMEOUT_US);
    delay::delay_us(500);
    let rejected = b.receive().is_err();
    // The acceptance filter runs after the acknowledge, so the transmitter still
    // sees a healthy bus.
    let acknowledged = a.error_counters().transmit == 0 && a.bus_state() == BusState::Active;

    // Back to accepting everything for the next round.
    b.configure(&Config {
        timing: Timing::for_bitrate(XTAL_HZ, BITRATE, SAMPLE_POINT).unwrap(),
        mode: Mode::Normal,
        filter: Filter::AcceptAll,
    });
    b.start();

    let ok = queued && rejected && acknowledged;
    defmt::info!(
        "filter: queued {=bool}, rejected {=bool}, still acked {=bool} -> {}",
        queued,
        rejected,
        acknowledged,
        if ok { "ok" } else { "FAIL" }
    );
}

/// After all of that traffic neither node may have collected an error.
fn check_idle_bus(a: &Twai0, b: &Twai1) {
    let counters_a = a.error_counters();
    let counters_b = b.error_counters();
    let ok = counters_a.transmit == 0
        && counters_a.receive == 0
        && counters_b.transmit == 0
        && counters_b.receive == 0
        && a.bus_state() == BusState::Active
        && b.bus_state() == BusState::Active;
    defmt::info!(
        "idle: tec/rec {=u8}/{=u8} and {=u8}/{=u8}, states {=?}/{=?} -> {}",
        counters_a.transmit,
        counters_a.receive,
        counters_b.transmit,
        counters_b.receive,
        a.bus_state(),
        b.bus_state(),
        if ok { "ok" } else { "FAIL" }
    );
}
