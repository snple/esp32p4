//! Moves memory with GDMA-AXI, and checks the CRC unit of its channels.
//!
//! AXI is the controller the high-bandwidth peripherals sit on (LCD/CAM, GP-SPI2/3,
//! PARLIO, AES, SHA), and unlike GDMA-AHB every one of its three pairs can loop a
//! transmit channel into its receive partner. Three checks run in a loop:
//!
//! 1. a 16380-byte copy on each of the three pairs, which needs four descriptors
//!    per direction and verifies the bytes;
//! 2. the same copy with every data burst size, which shows what the burst width
//!    is worth;
//! 3. a 16-bit CRC over 64 bytes, computed by the channel's CRC unit in both
//!    directions and compared against the same LFSR in software.
//!
//! A hand-built chain has to do its own cache maintenance: the engine reads
//! memory, and a buffer the core just wrote is still in the cache.
//! [`AxiDma::copy`] does it internally, the channel sequence below spells it out.
//!
//! Measured: about 33 MB/s through the `Dummy-n` loop-back on any pair, the same
//! as GDMA-AHB reaches, and no gain from a wider burst than 16 bytes — the
//! memory side, not the bus, is the limit here.

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
use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{
    AxiDma, AxiMemChannel, BurstSize, Chain, Crc, Descriptor, Dummy, ReceiveChannel, RxInterrupt,
    TransmitChannel,
};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Payload of the chained copy: four descriptors per direction, 4095 bytes each.
const BULK: usize = 4 * 4095;
/// Payload of the CRC check.
const CRC_LEN: usize = 64;

/// Buffers a burst of up to 128 bytes expects an aligned address.
#[repr(align(128))]
struct Aligned<const N: usize>([u8; N]);

static mut DESCRIPTORS: [Descriptor; 8] = [const { Descriptor::new() }; 8];
static mut SRC: Aligned<BULK> = Aligned([0; BULK]);
static mut DST: Aligned<BULK> = Aligned([0; BULK]);
static mut CRC_SRC: Aligned<CRC_LEN> = Aligned([0; CRC_LEN]);
static mut CRC_DST: Aligned<CRC_LEN> = Aligned([0; CRC_LEN]);

fn fill(buffer: &mut [u8], seed: u32) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = (index as u32 * 13 + seed) as u8;
    }
}

/// Runs one hand-built copy through a channel pair.
///
/// `burst` and `crc` are what the caller wants programmed on both channels; the
/// chain itself is one descriptor per direction, so the buffers have to fit one.
fn copy_manual(
    dma: &AxiDma,
    burst: BurstSize,
    crc: Option<&Crc>,
    src: &mut [u8],
    dst: &mut [u8],
    descriptors: &[Descriptor],
) -> (bool, u32, u32, u64) {
    let tx = dma.tx_channel::<0>();
    let rx = dma.rx_channel::<0>();
    let len = src.len();
    let start = delay::micros();

    tx.reset();
    rx.reset();
    // GDMA-AXI numbers GP-SPI2 as 1, so the free dummy ids start at 6.
    tx.connect_memory(Dummy::D6);
    rx.connect_memory(Dummy::D6);
    tx.check_owner(true);
    rx.check_owner(true);
    tx.data_burst_size(burst);
    rx.data_burst_size(burst);
    tx.auto_writeback(true);
    tx.eof_till_data_popped(true);
    tx.clear_all_interrupts();
    rx.clear_all_interrupts();
    if let Some(crc) = crc {
        tx.set_crc(crc);
        rx.set_crc(crc);
    }

    let (tx_descriptors, rx_descriptors) = descriptors.split_at(descriptors.len() / 2);
    let tx_chain = Chain::new(tx_descriptors);
    let rx_chain = Chain::new(rx_descriptors);
    tx_chain
        .mount_eof(src.as_ptr() as *mut u8, len)
        .expect("mounting the transmit chain failed");
    rx_chain
        .mount(dst.as_mut_ptr(), len)
        .expect("mounting the receive chain failed");

    // A hand-built chain does its own cache maintenance; `copy` does it itself.
    cache::writeback(src.as_ptr().cast(), len);
    cache::writeback_invalidate(dst.as_mut_ptr().cast(), len);

    rx.start_chain(rx_chain.head());
    tx.start_chain(tx_chain.head());

    let deadline = delay::micros() + 100_000;
    while !rx.interrupt_pending(RxInterrupt::SucEof) {
        assert!(delay::micros() < deadline, "the copy did not finish");
        core::hint::spin_loop();
    }
    rx.stop_chain();
    tx.stop_chain();

    cache::invalidate(dst.as_mut_ptr().cast(), len);
    let elapsed = delay::micros() - start;
    // Slice equality goes through `memcmp`; a byte-at-a-time loop is an order of
    // magnitude slower in an unoptimised build and hides the transfer time.
    (dst == src, tx.crc_result(), rx.crc_result(), elapsed)
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    // `delay::micros` starts at a conservative boot rate; publishing the real
    // one keeps the throughput numbers honest.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let dma = AxiDma::init(dp.AXI_DMA, &clocks);

    // SAFETY: the statics are used by nothing else, and only one transfer runs
    // at a time.
    let descriptors = unsafe { &mut *core::ptr::addr_of_mut!(DESCRIPTORS) };
    let src = unsafe { &mut *core::ptr::addr_of_mut!(SRC) };
    let dst = unsafe { &mut *core::ptr::addr_of_mut!(DST) };
    let crc_src = unsafe { &mut *core::ptr::addr_of_mut!(CRC_SRC) };
    let crc_dst = unsafe { &mut *core::ptr::addr_of_mut!(CRC_DST) };

    let src = &mut src.0;
    let dst = &mut dst.0;
    let crc_src = &mut crc_src.0;
    let crc_dst = &mut crc_dst.0;

    let mut seed = 0;
    loop {
        defmt::info!("---- GDMA-AXI round {=u32} ----", seed);
        fill(src, seed);
        crc_dst.fill(0xa5);

        // 1. Sixteen kilobytes on every pair, four descriptors per direction.
        for channel in [AxiMemChannel::Ch0, AxiMemChannel::Ch1, AxiMemChannel::Ch2] {
            dst.fill(0xa5);
            let start = delay::micros();
            let copied = dma
                .copy(channel, src, dst, descriptors)
                .expect("the AXI copy failed");
            let elapsed = delay::micros() - start;
            let rate = copied as u64 * 1_000_000 / elapsed.max(1);
            defmt::info!(
                "{:?} 4 descriptors: {} bytes in {} us ({} KiB/s) -> {}",
                channel,
                copied,
                elapsed,
                rate / 1024,
                if dst[..copied] == src[..copied] {
                    "ok"
                } else {
                    "FAIL"
                }
            );
        }

        // 2. The same 2048-byte copy at every burst width.
        for burst in [
            BurstSize::B8,
            BurstSize::B16,
            BurstSize::B32,
            BurstSize::B64,
            BurstSize::B128,
        ] {
            dst.fill(0xa5);
            let (ok, _, _, elapsed) = copy_manual(&dma, burst, None, src, dst, descriptors);
            let rate = BULK as u64 * 1_000_000 / elapsed.max(1);
            defmt::info!(
                "burst {:?}: {} bytes in {} us ({} KiB/s) -> {}",
                burst,
                BULK,
                elapsed,
                rate / 1024,
                if ok { "ok" } else { "FAIL" }
            );
        }

        // 3. A 16-bit CRC over both directions of a 64-byte copy.
        fill(crc_src, seed);
        let crc = Crc::new(16, 0x1021, 0xffff);
        let (ok, tx_crc, rx_crc, _) = copy_manual(
            &dma,
            BurstSize::B64,
            Some(&crc),
            crc_src,
            crc_dst,
            &descriptors[..2],
        );
        let expected = crc.compute(crc_src);
        defmt::info!(
            "crc-16 over {} bytes: {} (engine tx {=u32:#x} rx {=u32:#x}, software {=u32:#x})",
            CRC_LEN,
            if ok && tx_crc == expected && rx_crc == expected {
                "ok"
            } else {
                "FAIL"
            },
            tx_crc,
            rx_crc,
            expected
        );

        seed = seed.wrapping_add(1);
        delay::delay_ms(2_000);
    }
}
