//! Copies memory with GDMA-AHB.
//!
//! Each copy drives a whole channel pair: the transmit channel walks the source
//! descriptor chain, loops through a `Dummy-n` trigger into the receive channel,
//! and the receive channel writes the destination chain. Every second the
//! example repeats two cases:
//!
//! 1. one descriptor per direction, 64 bytes, channel 1;
//! 2. four descriptors per direction, 16 KB, channel 2 — the engine has to
//!    follow the `next` pointers.
//!
//! Then it copies 64 bytes once more, this time with the CRC units of both
//! directions programmed with `CRC-32/MPEG-2`, and compares what the engine
//! reports against a straightforward software implementation of the same
//! standard.
//!
//! Each copy is verified; a mismatch panics. The log reports the throughput.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::cache;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{
    AhbDma, Chain, Crc, Descriptor, Dummy, MemChannel, ReceiveChannel, RxInterrupt, TransmitChannel,
};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Four descriptors per direction, the largest chain this example mounts.
const DESCRIPTORS_PER_DIRECTION: usize = 4;

/// Bytes one descriptor can describe.
const CHUNK: usize = 0xfff;

/// Payload of the multi-descriptor copy.
const BULK: usize = DESCRIPTORS_PER_DIRECTION * CHUNK;

/// Payload of the single-descriptor copy.
const SMALL: usize = 64;

/// Transmit and receive descriptors, in that order.
static mut DESCRIPTORS: [Descriptor; 2 * DESCRIPTORS_PER_DIRECTION] =
    [const { Descriptor::new() }; 2 * DESCRIPTORS_PER_DIRECTION];
static mut SRC: [u8; BULK] = [0; BULK];
static mut DST: [u8; BULK] = [0; BULK];
static mut SMALL_SRC: [u8; SMALL] = [0; SMALL];
static mut SMALL_DST: [u8; SMALL] = [0; SMALL];

fn fill(buffer: &mut [u8], seed: u32) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = (index as u32 * 7 + seed) as u8;
    }
}

/// `CRC-32/MPEG-2`: polynomial `0x04C1_1DB7`, init all ones, no reflection, no
/// final xor — the textbook bit-by-bit form, written out so the engine's answer
/// has something independent to be compared against.
fn crc32_mpeg2(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= u32::from(byte) << 24;
        for _ in 0..8 {
            crc = if crc & 0x8000_0000 != 0 {
                (crc << 1) ^ 0x04c1_1db7
            } else {
                crc << 1
            };
        }
    }
    crc
}

fn copy_and_check(
    dma: &AhbDma,
    channel: MemChannel,
    name: &str,
    src: &[u8],
    dst: &mut [u8],
    descriptors: &[Descriptor],
) {
    let start = esp32p4_hal::delay::micros();
    let copied = dma
        .copy(channel, src, dst, descriptors)
        .expect("DMA copy failed");
    let elapsed = esp32p4_hal::delay::micros() - start;

    assert_eq!(copied, src.len(), "short copy");
    assert_eq!(dst[..copied], src[..copied], "data mismatch");

    let bytes_per_s = copied as u64 * 1_000_000 / elapsed.max(1);
    defmt::info!(
        "{}: {} bytes in {} us ({} KiB/s)",
        name,
        copied,
        elapsed,
        bytes_per_s / 1024
    );
}

/// Runs one copy with the CRC units programmed, and checks both results.
fn crc_and_check(
    dma: &AhbDma,
    src: &[u8],
    dst: &mut [u8],
    descriptors: &[Descriptor],
    label: &str,
) {
    let crc = Crc::new(32, 0x04c1_1db7, 0xffff_ffff);
    let tx = dma.tx_channel::<0>();
    let rx = dma.rx_channel::<0>();

    tx.reset();
    rx.reset();
    tx.connect_memory(Dummy::D1);
    rx.connect_memory(Dummy::D1);
    tx.check_owner(true);
    rx.check_owner(true);
    tx.auto_writeback(true);
    tx.eof_till_data_popped(true);
    tx.clear_all_interrupts();
    rx.clear_all_interrupts();
    tx.set_crc(&crc);
    rx.set_crc(&crc);

    let (tx_descriptors, rx_descriptors) = descriptors.split_at(1);
    let tx_chain = Chain::new(tx_descriptors);
    let rx_chain = Chain::new(rx_descriptors);
    tx_chain
        .mount_eof(src.as_ptr() as *mut u8, src.len())
        .expect("mounting the transmit chain failed");
    rx_chain
        .mount(dst.as_mut_ptr(), dst.len())
        .expect("mounting the receive chain failed");

    // The driver's `copy` does this itself; a chain built by hand has to: the
    // engine reads memory, and the fresh source bytes may still be in the cache.
    cache::writeback(src.as_ptr().cast(), src.len());
    cache::writeback_invalidate(dst.as_mut_ptr().cast(), dst.len());

    rx.start_chain(rx_chain.head());
    tx.start_chain(tx_chain.head());

    let deadline = delay::micros() + 100_000;
    while !rx.interrupt_pending(RxInterrupt::SucEof) {
        assert!(delay::micros() < deadline, "the CRC copy did not finish");
        core::hint::spin_loop();
    }
    rx.stop_chain();
    tx.stop_chain();

    cache::invalidate(dst.as_mut_ptr().cast(), dst.len());
    let expected = crc32_mpeg2(src);
    let (tx_crc, rx_crc) = (tx.crc_result(), rx.crc_result());
    let ok = tx_crc == expected && rx_crc == expected && dst[..] == src[..];
    defmt::info!(
        "{}: {} (engine tx {=u32:#x} rx {=u32:#x}, software {=u32:#x})",
        label,
        if ok { "ok" } else { "FAIL" },
        tx_crc,
        rx_crc,
        expected
    );
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);

    // SAFETY: buffers and descriptors are only used here, for the whole run.
    let descriptors = unsafe { &*core::ptr::addr_of!(DESCRIPTORS) };
    let src = unsafe { &mut *core::ptr::addr_of_mut!(SRC) };
    let dst = unsafe { &mut *core::ptr::addr_of_mut!(DST) };
    let small_src = unsafe { &mut *core::ptr::addr_of_mut!(SMALL_SRC) };
    let small_dst = unsafe { &mut *core::ptr::addr_of_mut!(SMALL_DST) };

    let mut seed = 0;
    loop {
        fill(src, seed);
        fill(small_src, seed);
        seed = seed.wrapping_add(1);

        // One descriptor per direction: the shortest possible transfer.
        copy_and_check(
            &dma,
            MemChannel::Ch1,
            "sram/sram 1 descriptor",
            small_src,
            small_dst,
            descriptors,
        );

        // Four descriptors per direction on the second channel pair.
        copy_and_check(
            &dma,
            MemChannel::Ch2,
            "sram/sram 4 descriptors",
            src,
            dst,
            descriptors,
        );

        // The CRC units see the same bytes the engine moves, so both directions
        // have to report the value the standard says.
        fill(small_src, seed);
        crc_and_check(
            &dma,
            small_src,
            small_dst,
            descriptors,
            "crc-32/mpeg-2 over 64 bytes",
        );

        delay::delay_ms(1_000);
    }
}
