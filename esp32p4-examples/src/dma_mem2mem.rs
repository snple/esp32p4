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

use esp32p4_hal::delay::Delay;
use esp32p4_hal::dma::{AhbDma, Descriptor, MemChannel};
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

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init();
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

        delay.delay_ms(1_000);
    }
}
