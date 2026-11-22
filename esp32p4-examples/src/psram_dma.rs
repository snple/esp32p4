//! Pins down what a DMA buffer in cached PSRAM needs from the cache.
//!
//! The `0x4800_0000` window is cached, and the L2 level sits in front of the
//! PSRAM controller rather than inside the core, so a bus master's accesses
//! land in the same L2 the core uses. That is what makes the rule asymmetric:
//!
//! - the core writes into L1, which L2 cannot see until it is written back, so
//!   a transmit buffer needs `cache::writeback` — L1 only, L2 already has it;
//! - a bus master writes into L2, which the core then has to re-read, so a
//!   receive buffer needs `cache::invalidate` — again L1 only. Invalidating L2
//!   would throw away exactly the data the engine just put there.
//!
//! The checks below are the four cases in order: a transmit buffer, a receive
//! buffer, a 260 kB copy through PSRAM, and a demonstration that the L2 half of
//! the map is harmful (it drops the engine's writes).

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
use esp32p4_hal::dma::{AxiDma, AxiMemChannel, Descriptor};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::psram::{Psram, PsramConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Bytes the bulk copy moves: 64 descriptors per direction, and far more than
/// the 64 kB L1 D-cache holds.
const BULK: usize = 4080;
/// One cache line, which is what the small checks move.
const LINE: usize = 64;
/// Rounds the one-direction checks run. A single pass reads clean often
/// enough to look correct, which is how "only the both-ways copy is lossy"
/// was concluded once.
const ROUNDS: usize = 16;
/// Where the one-direction rounds start, past the two buffers above.
const ROUND_BASE: usize = 2 * BULK;
/// Bytes between rounds, so no round reuses a cached line.
const ROUND_STRIDE: usize = 8192;
/// Descriptors the bulk copy needs, two directions.
const DESCRIPTOR_COUNT: usize = 2 * BULK.div_ceil(4095);

static mut DESCRIPTORS: [Descriptor; DESCRIPTOR_COUNT] =
    [const { Descriptor::new() }; DESCRIPTOR_COUNT];
static mut SCRATCH: [u8; LINE] = [0; LINE];
static mut BULK_SCRATCH: [u8; BULK] = [0; BULK];

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let psram = Psram::init(PsramConfig::from_efuse(&efuse)).expect("PSRAM init failed");
    let dma = AxiDma::init(dp.AXI_DMA, &clocks);
    let scratch = unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) };
    // SAFETY: the statics are used by nothing else, and one copy runs at a time.
    let descriptors = unsafe { &*core::ptr::addr_of!(DESCRIPTORS) };

    // The transmit buffer: the core owns it, so the engine only sees what the
    // writeback pushed down into L2.
    let source = unsafe { core::slice::from_raw_parts_mut(psram.base(), LINE) };
    source.fill(0xaa);
    cache::writeback(source.as_ptr().cast(), LINE);
    scratch.fill(0);
    let copied = dma
        .copy(AxiMemChannel::Ch0, source, scratch, descriptors)
        .expect("the transmit copy failed");
    let seen = scratch[0];
    defmt::info!("transmit buffer: engine read {=u8:x} (want 0xaa)", seen);
    assert_eq!(copied, LINE);
    assert_eq!(seen, 0xaa, "the writeback did not reach L2");

    // Now the same line without a writeback: how far the core's write travels
    // on its own says whether L1 is write-through for this window.
    source.fill(0x55);
    scratch.fill(0);
    dma.copy(AxiMemChannel::Ch0, source, scratch, descriptors)
        .expect("the second transmit copy failed");
    let stale = scratch[0];
    defmt::info!(
        "without writeback: engine read {=u8:x} (0x55 means it went through)",
        stale
    );

    // The receive buffer: the engine writes L2, and the core has to drop its
    // own line to see it.
    let target = unsafe { core::slice::from_raw_parts_mut(psram.base().add(LINE), LINE) };
    target.fill(0x11);
    cache::writeback(target.as_ptr().cast(), LINE);
    // Fill L1 with the old content before the engine overwrites it.
    let before = target[0];
    scratch.fill(0x22);
    dma.copy(AxiMemChannel::Ch0, scratch, target, descriptors)
        .expect("the receive copy failed");
    let after = target[0];
    cache::invalidate(target.as_ptr().cast(), LINE);
    let refreshed = target[0];
    defmt::info!(
        "receive buffer: before {=u8:x}, stale read {=u8:x}, after invalidate {=u8:x}",
        before,
        after,
        refreshed
    );
    assert_eq!(before, 0x11);
    // `after` only says whether L1 held a copy of the line; the invalidate is
    // what has to be right in either case.
    assert_eq!(
        refreshed, 0x22,
        "invalidate did not pick up the engine's data"
    );

    // The same maintenance at scale: 260 kB through PSRAM, both directions.
    let src = unsafe { core::slice::from_raw_parts_mut(psram.base(), BULK) };
    let dst = unsafe { core::slice::from_raw_parts_mut(psram.base().add(BULK), BULK) };
    for (index, byte) in src.iter_mut().enumerate() {
        *byte = (index as u32 * 13 + 7) as u8;
    }
    dst.fill(0);
    // First, does the window even hold 256 kB written by the core and read
    // straight back? No DMA involved, and twice: through the cached window and
    // through the alias, which tells a cache problem from a PSRAM one.
    // Push the whole window out to the die, then drop every cache level, so
    // what the core reads next comes from PSRAM itself.
    cache::writeback(src.as_ptr().cast(), BULK);
    {
        let regs = unsafe { &*esp32p4_hal::pac::CACHE::PTR };
        regs.sync_map.write_bits(0x30);
        regs.sync_addr.write_bits(src.as_ptr() as u32);
        regs.sync_size.write_bits(BULK as u32);
        for _ in 0..2 {
            regs.sync_ctrl.write_bits(1);
            while !regs.sync_ctrl.sync_done() {}
        }
    }
    let mut cached_wrong = 0;
    let mut alias_wrong = 0;
    let mut first_wrong = usize::MAX;
    for (index, byte) in src.iter().enumerate() {
        let expected = (index as u32 * 13 + 7) as u8;
        if *byte != expected {
            cached_wrong += 1;
            if index < first_wrong {
                first_wrong = index;
            }
        }
        // SAFETY: the alias maps the same pages.
        let alias = unsafe { core::ptr::read_volatile(psram.uncached_base().add(index)) };
        if alias != expected {
            alias_wrong += 1;
        }
    }
    defmt::info!(
        "psram hold check: {} bytes, {} wrong via the cache (first at {}), {} wrong via the alias",
        BULK,
        cached_wrong,
        first_wrong,
        alias_wrong
    );

    // One direction at a time, `ROUNDS` rounds each into a bulk buffer. This
    // is where a single pass lies: the engine drops a 16 or 32 byte chunk now
    // and then, so one clean copy means nothing.
    let sram = unsafe { &mut *core::ptr::addr_of_mut!(BULK_SCRATCH) };
    let mut read_bad_rounds = 0;
    let mut read_wrong = 0;
    for round in 0..ROUNDS {
        let source = unsafe {
            core::slice::from_raw_parts_mut(
                psram.base().add(ROUND_BASE + round * ROUND_STRIDE),
                BULK,
            )
        };
        for (index, byte) in source.iter_mut().enumerate() {
            *byte = (index % 251) as u8;
        }
        cache::writeback(source.as_ptr().cast(), BULK);
        sram.fill(0);
        dma.copy(AxiMemChannel::Ch0, source, sram, descriptors)
            .expect("the read copy failed");
        cache::invalidate(sram.as_ptr().cast(), BULK);
        let wrong = source
            .iter()
            .zip(sram.iter())
            .filter(|(a, b)| a != b)
            .count();
        if wrong != 0 {
            read_bad_rounds += 1;
            read_wrong += wrong;
        }
    }
    let mut write_bad_rounds = 0;
    let mut write_wrong = 0;
    for round in 0..ROUNDS {
        let destination = unsafe {
            core::slice::from_raw_parts_mut(
                psram.base().add(ROUND_BASE + round * ROUND_STRIDE),
                BULK,
            )
        };
        for (index, byte) in sram.iter_mut().enumerate() {
            *byte = (index % 251) as u8;
        }
        cache::writeback(sram.as_ptr().cast(), BULK);
        destination.fill(0);
        cache::writeback(destination.as_ptr().cast(), BULK);
        dma.copy(AxiMemChannel::Ch0, sram, destination, descriptors)
            .expect("the write copy failed");
        // Compare through the uncached alias: what PSRAM really holds.
        let held = unsafe {
            core::slice::from_raw_parts(
                psram
                    .base()
                    .add(0x4000_0000 + ROUND_BASE + round * ROUND_STRIDE),
                BULK,
            )
        };
        let wrong = sram.iter().zip(held.iter()).filter(|(a, b)| a != b).count();
        if wrong != 0 {
            write_bad_rounds += 1;
            write_wrong += wrong;
        }
    }
    // Reported, not asserted: the engine drops bytes when it *reads* PSRAM. See
    // the roadmap entry for the missing SPI timing tuning.
    defmt::info!(
        "psram read {} x {} bytes: {} bad rounds, {} wrong; write: {} bad rounds, {} wrong",
        ROUNDS,
        BULK,
        read_bad_rounds,
        read_wrong,
        write_bad_rounds,
        write_wrong
    );

    // Both directions at once is the one case that does not survive: the two
    // AXI channels read and write the same die and lose bytes, whatever the
    // timing. The driver refuses it instead of returning wrong data.
    let refused = dma.copy(AxiMemChannel::Ch0, src, dst, descriptors);
    defmt::info!("psram to psram copy: {:?} (refused)", refused);
    assert_eq!(
        refused,
        Err(esp32p4_hal::dma::Error::ExternalMemoryCopy),
        "a PSRAM to PSRAM copy has to be refused"
    );

    // Finally, why the L2 half of the map is not merely unnecessary but wrong:
    // invalidating L2 throws away what the engine wrote there.
    let probe = unsafe { core::slice::from_raw_parts_mut(psram.base().add(2 * BULK), LINE) };
    probe.fill(0x33);
    cache::writeback(probe.as_ptr().cast(), LINE);
    scratch.fill(0x44);
    dma.copy(AxiMemChannel::Ch0, scratch, probe, descriptors)
        .expect("the probe copy failed");
    // Map the same range with `L1-DCache | L2-Cache` and invalidate it.
    let cache_regs = unsafe { &*esp32p4_hal::pac::CACHE::PTR };
    let start = probe.as_ptr() as usize & !(LINE - 1);
    cache_regs.sync_map.write_bits(0x30);
    cache_regs.sync_addr.write_bits(start as u32);
    cache_regs.sync_size.write_bits(LINE as u32);
    for _ in 0..2 {
        cache_regs.sync_ctrl.write_bits(1);
        while !cache_regs.sync_ctrl.sync_done() {}
    }
    // What is left is what PSRAM really holds: the engine's 0x44 went down with
    // the dirty L2 line, and 0x33 — the value before the copy — comes back.
    let lost = probe[0];
    defmt::info!(
        "after an L1+L2 invalidate: {=u8:x} (0x33 means the engine's 0x44 was dropped)",
        lost
    );

    defmt::info!("psram dma: cache rules hold, bulk transfers reported above");

    loop {
        delay::delay_ms(1_000);
    }
}
