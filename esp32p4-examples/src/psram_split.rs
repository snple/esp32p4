//! Splits the PSRAM window between a global heap and a DMA pool.
//!
//! The MMU maps one contiguous window, but `PsramAllocator::init_raw` can place
//! independent heaps inside it. A useful split is:
//!
//! ```text
//! 0x4800_0000 ┌───────────── cached, global heap (Box/Vec/String) ─────────────┐
//!             │                                                             │
//!             ├───────────── uncached alias, DMA pool ──────────────────────┤
//! 0x8800_0000 └───────────── (same pages, seen through the uncached alias) ───┘
//! ```
//!
//! The DMA pool is addressed through [`Psram::uncached_base`], so buffers taken
//! from it never need cache maintenance before a peripheral transfer.

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own.
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

extern crate alloc;

use alloc::vec::Vec;

use esp32p4::delay::Delay;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::psram::{Psram, PsramAllocator, PsramConfig};

/// LED pin (GPIO2).
const LED_PIN: u32 = 2;

/// Bytes reserved for DMA buffers at the end of the window.
const DMA_POOL_SIZE: usize = 2 * 1024 * 1024;

/// Heaps are placed on MMU page boundaries so the two regions never share one.
const SPLIT_ALIGN: usize = 64 * 1024;

#[global_allocator]
static HEAP: PsramAllocator = PsramAllocator::empty();

/// Pool for buffers handed to peripherals; lives in the uncached alias.
static DMA_POOL: PsramAllocator = PsramAllocator::empty();

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    let psram = Psram::init(PsramConfig::default()).expect("PSRAM init failed");

    // [ ......... global heap (cached) ......... | ... DMA pool (uncached) ... ]
    let dma_len = DMA_POOL_SIZE.min(psram.size() / 4) & !(SPLIT_ALIGN - 1);
    let heap_len = (psram.size() - dma_len) & !(SPLIT_ALIGN - 1);

    let heap_start = psram.base() as usize;
    let dma_start = psram.uncached_base() as usize + heap_len;

    // SAFETY: the two regions are disjoint, page aligned and only handed out by
    // the allocator they were given to.
    unsafe {
        HEAP.init_raw(heap_start as *mut u8, heap_len);
        DMA_POOL.init_raw(dma_start as *mut u8, dma_len);
    }

    // Ordinary allocations land in the cached half.
    let mut samples: Vec<u32> = Vec::with_capacity(4096);
    samples.extend(0..4096u32);
    assert_eq!(samples.iter().sum::<u32>(), (0..4096u32).sum());

    // DMA buffers come from the uncached pool, so a peripheral sees them
    // without any cache writeback.
    let mut buffer: Vec<u8, &PsramAllocator> = Vec::new_in(&DMA_POOL);
    buffer.resize(64 * 1024, 0);
    buffer.fill(0xa5);

    let samples_addr = samples.as_ptr() as usize;
    assert!(
        (heap_start..heap_start + heap_len).contains(&samples_addr),
        "global allocation escaped the cached heap"
    );

    let buffer_addr = buffer.as_ptr() as usize;
    assert!(
        (dma_start..dma_start + dma_len).contains(&buffer_addr),
        "DMA buffer is not in the uncached pool"
    );

    let delay = Delay::init(dp.SYSTIMER);
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
