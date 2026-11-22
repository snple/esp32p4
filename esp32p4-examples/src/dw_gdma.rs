//! Moves memory with the `DMA` controller (DW_GDMA).
//!
//! A different engine from the two GDMA controllers: four channels, one block
//! per transfer described by four registers, no descriptor chain. Each round
//! copies a buffer with every transfer width and verifies the bytes, then
//! repeats one copy enough times to report a throughput.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{Burst, DwGdma, Error, LinkListItem, TransferConfig, Width};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Payload of one copy: 16 kB, well inside what a single block can carry.
const LEN: usize = 16 * 1024;
/// Repeats for the throughput figure.
const ROUNDS: u32 = 16;

#[repr(align(64))]
struct Aligned<const N: usize>([u8; N]);

static mut SRC: Aligned<LEN> = Aligned([0; LEN]);
static mut DST: Aligned<LEN> = Aligned([0; LEN]);
static mut ITEM: LinkListItem = LinkListItem::new();

fn fill(buffer: &mut [u8], seed: u32) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = (index as u32 * 13 + seed) as u8;
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    // `delay::micros` starts at a conservative boot rate; publishing the real one
    // keeps the throughput numbers honest.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let dma = DwGdma::init(dp.DMA, &clocks);
    let channel = dma.channel::<0>();

    // SAFETY: the statics are used by nothing else, and one copy runs at a time.
    let src = unsafe { &mut *core::ptr::addr_of_mut!(SRC) };
    let dst = unsafe { &mut *core::ptr::addr_of_mut!(DST) };
    let src = &mut src.0;
    let dst = &mut dst.0;
    let item = unsafe { &mut *core::ptr::addr_of_mut!(ITEM) };

    let mut seed = 0;
    loop {
        defmt::info!("---- DW_GDMA round {=u32} ----", seed);
        fill(src, seed);

        // Width and burst are the two knobs the block form has; both are
        // exercised so a wrong field encoding shows up as a byte mismatch.
        for width in [Width::Bits8, Width::Bits16, Width::Bits32, Width::Bits64] {
            for burst in [Burst::B1, Burst::B16, Burst::B256] {
                let config = TransferConfig {
                    width,
                    src_burst: burst,
                    dst_burst: burst,
                    ..TransferConfig::default()
                };
                channel.configure(&config);
                dst.fill(0);
                esp32p4_hal::cache::writeback(src.as_ptr().cast(), LEN);
                channel
                    .start(src.as_ptr(), dst.as_mut_ptr(), LEN, item)
                    .expect("starting the copy failed");
                let deadline = delay::micros() + 100_000;
                loop {
                    let status = channel.interrupt_status();
                    if status != 0 {
                        defmt::info!("status {=u32:x}", status);
                        channel.clear_interrupts();
                        break;
                    }
                    assert!(delay::micros() < deadline, "the copy did not finish");
                }
                esp32p4_hal::cache::invalidate(dst.as_mut_ptr().cast(), LEN);
                let mismatches = src.iter().zip(dst.iter()).filter(|(a, b)| a != b).count();
                if mismatches != 0 {
                    let mut shown = 0;
                    for index in 0..LEN {
                        if src[index] != dst[index] && shown < 8 {
                            defmt::info!("  {}: {=u8:x} vs {=u8:x}", index, src[index], dst[index]);
                            shown += 1;
                        }
                    }
                }
                defmt::info!(
                    "width {} B, burst {:?}: {} mismatches",
                    width.bytes(),
                    burst,
                    mismatches
                );
                assert_eq!(mismatches, 0, "DW_GDMA copy mismatch");
            }
        }

        // The convenience path picks the width from the buffers' alignment.
        dst.fill(0);
        let copied = channel
            .copy(src, dst, item, 100_000)
            .expect("the convenience copy failed");
        let mismatches = src.iter().zip(dst.iter()).filter(|(a, b)| a != b).count();
        defmt::info!("copy {} bytes, {} mismatches", copied, mismatches);
        assert_eq!(copied, LEN);
        assert_eq!(mismatches, 0);

        // Refuses a buffer it cannot express rather than transferring garbage.
        assert_eq!(channel.copy(&[], dst, item, 1_000), Err(Error::BufferSize));

        let start = delay::micros();
        for _ in 0..ROUNDS {
            channel
                .copy(src, dst, item, 100_000)
                .expect("the timed copy failed");
        }
        let elapsed = delay::micros() - start;
        let rate = LEN as u64 * u64::from(ROUNDS) * 1_000_000 / elapsed.max(1);
        defmt::info!(
            "throughput: {} kB in {} us -> {} MB/s",
            LEN * ROUNDS as usize / 1024,
            elapsed,
            rate / 1_000_000
        );

        seed += 1;
        delay::delay_ms(1_000);
    }
}
