//! Moves UART data with the UHCI and the GDMA.
//!
//! `UART1` and `UART2` run in internal loopback, so the transmitter feeds the
//! receiver without any wiring; everything below still goes through the real
//! DMA path: GDMA transmit channel → UHCI encoder → UART TX → (loopback) →
//! UART RX → UHCI decoder → GDMA receive channel.
//!
//! The checks are:
//!
//! 1. a 64-byte round trip through [`write_dma`]/[`read_dma`];
//! 2. a 4096-byte round trip that needs two descriptors, with the receive
//!    armed before the transmit starts — the receiver has to keep up with the
//!    transmitter instead of draining a 128-byte FIFO that already overflowed;
//! 3. a short packet: the receive times out, and the packet after it still
//!    arrives intact because the aborted receive was dropped;
//! 4. the same round trip on `UART2`, which re-attaches the one UHCI to the
//!    other port;
//! 5. a transmit that needs more descriptors than it was given, and a receive
//!    longer than one decoder packet.
//!
//! [`write_dma`]: esp32p4_hal::uart::Uart1::write_dma
//! [`read_dma`]: esp32p4_hal::uart::Uart1::read_dma

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay;
use esp32p4_hal::dma::Error as DmaError;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::uart::{DMA_MAX_PACKET, Error, Uart1, Uart2, UartConfig, Uhci};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Baud rate for the DMA transfers; the internal loopback handles it easily.
const BAUD: u32 = 2_000_000;
/// Transfer timeout: several times what 4096 bytes need at `BAUD`, so only a
/// stuck channel hits it.
const TIMEOUT_US: u64 = 4 * (4096 * 10 * 1_000_000 / BAUD as u64);

/// Payload of the small round trip.
const SMALL: usize = 64;
/// Payload of the multi-descriptor round trip.
const BULK: usize = 4096;

static mut DESCRIPTORS: [Descriptor; 4] = [const { Descriptor::new() }; 4];
static mut SENT: [u8; BULK] = [0; BULK];
static mut RECEIVED: [u8; BULK] = [0; BULK];
static mut OVERSIZED: [u8; DMA_MAX_PACKET + 1] = [0; DMA_MAX_PACKET + 1];

fn fill(buffer: &mut [u8], seed: u32) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = (index as u32 * 7 + seed) as u8;
    }
}

/// Index of the first byte that differs, if any.
fn first_mismatch(a: &[u8], b: &[u8]) -> Option<usize> {
    a.iter().zip(b).position(|(x, y)| x != y)
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{}: {}", label, if ok { "ok" } else { "FAIL" });
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
    let uhci = Uhci::init(dp.UHCI0, &dp.HP_SYS_CLKRST);

    let config = UartConfig::default().baudrate(BAUD).loopback(true);
    let uart1 = Uart1::init(dp.UART1, &config, &dp.HP_SYS_CLKRST);
    let uart2 = Uart2::init(dp.UART2, &config, &dp.HP_SYS_CLKRST);
    let tx = dma.tx_channel::<0>();
    let rx = dma.rx_channel::<0>();

    // SAFETY: the statics are used by nothing else, and only one transfer runs
    // at a time.
    let descriptors = unsafe { &mut *core::ptr::addr_of_mut!(DESCRIPTORS) };
    let sent = unsafe { &mut *core::ptr::addr_of_mut!(SENT) };
    let received = unsafe { &mut *core::ptr::addr_of_mut!(RECEIVED) };
    let oversized = unsafe { &mut *core::ptr::addr_of_mut!(OVERSIZED) };

    defmt::info!(
        "UART{} at {} baud, UHCI attached to UART{}",
        Uart1::NUM,
        uart1.baudrate(config.clock, &dp.HP_SYS_CLKRST),
        uhci.attached_uart()
    );

    // 1. Small round trip, one descriptor per direction.
    fill(&mut sent[..SMALL], 1);
    let start = delay::micros();
    let tx_result = uart1.write_dma(&uhci, &tx, &sent[..SMALL], &descriptors[..1], TIMEOUT_US);
    let sent_in = delay::micros() - start;
    let count = uart1.read_dma(
        &uhci,
        &rx,
        &mut received[..SMALL],
        &descriptors[2..3],
        TIMEOUT_US,
    );
    let total = delay::micros() - start;
    report(
        "64-byte round trip",
        tx_result.is_ok()
            && count == Ok(SMALL)
            && first_mismatch(&received[..SMALL], &sent[..SMALL]).is_none(),
    );
    defmt::info!(
        "  sent in {} us, {} bytes back {} us later",
        sent_in,
        count.unwrap_or(0),
        total - sent_in
    );

    // 2. 4096 bytes need two descriptors. The receive is armed first: with a
    // 128-byte UART FIFO, draining after the fact would lose everything.
    fill(&mut sent[..BULK], 3);
    uart1
        .rx_begin_dma(&uhci, &rx, &mut received[..BULK], &descriptors[2..4])
        .expect("arming the bulk receive failed");
    let tx_result = uart1.write_dma(&uhci, &tx, &sent[..BULK], &descriptors[..2], TIMEOUT_US);
    let count = uart1.rx_wait_dma(
        &uhci,
        &rx,
        &mut received[..BULK],
        &descriptors[2..4],
        TIMEOUT_US,
    );
    report(
        "4096-byte round trip",
        tx_result.is_ok()
            && count == Ok(BULK)
            && first_mismatch(&received[..BULK], &sent[..BULK]).is_none(),
    );
    defmt::info!("  {} bytes", count.unwrap_or(0));

    // 3. Ten bytes do not complete a 64-byte packet. The receive gives up, and
    // the packet after it arrives from its first byte: the aborted receive is
    // dropped, not carried over.
    fill(&mut sent[..SMALL], 7);
    let tx_result = uart1.write_dma(&uhci, &tx, &sent[..10], &descriptors[..1], TIMEOUT_US);
    let short = uart1.read_dma(
        &uhci,
        &rx,
        &mut received[..SMALL],
        &descriptors[2..3],
        20_000,
    );
    let tx_result2 = uart1.write_dma(&uhci, &tx, &sent[..SMALL], &descriptors[..1], TIMEOUT_US);
    let after_short = uart1.read_dma(
        &uhci,
        &rx,
        &mut received[..SMALL],
        &descriptors[2..3],
        TIMEOUT_US,
    );
    report(
        "short packet times out and is dropped",
        tx_result.is_ok()
            && short == Err(Error::Timeout)
            && tx_result2.is_ok()
            && after_short == Ok(SMALL)
            && first_mismatch(&received[..SMALL], &sent[..SMALL]).is_none(),
    );
    defmt::info!("  short packet: {:?}, next packet {:?}", short, after_short);

    // 4. The same UHCI, another port: `attach` has to move the decoder.
    fill(&mut sent[..SMALL], 5);
    let tx_result = uart2.write_dma(&uhci, &tx, &sent[..SMALL], &descriptors[..1], TIMEOUT_US);
    let count = uart2.read_dma(
        &uhci,
        &rx,
        &mut received[..SMALL],
        &descriptors[2..3],
        TIMEOUT_US,
    );
    report(
        "round trip on UART2",
        tx_result.is_ok()
            && uhci.attached_uart() == Uart2::NUM
            && count == Ok(SMALL)
            && first_mismatch(&received[..SMALL], &sent[..SMALL]).is_none(),
    );

    // 5. The two limits of the DMA path.
    let shortage = uart2.write_dma(&uhci, &tx, &sent[..BULK], &descriptors[..1], TIMEOUT_US);
    report(
        "descriptor shortage is reported",
        shortage
            == Err(Error::Dma(DmaError::NotEnoughDescriptors {
                needed: 2,
                available: 1,
            })),
    );
    let too_long = uart2.read_dma(&uhci, &rx, oversized, &descriptors[2..4], TIMEOUT_US);
    report(
        "oversized receive is rejected",
        too_long == Err(Error::BufferTooLong),
    );

    loop {
        delay::delay_ms(1_000);
    }
}
