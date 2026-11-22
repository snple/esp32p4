//! Shows which internal SRAM the linker handed out, and proves it is RAM.
//!
//! The 768 KB of HP L2MEM is not one usable window: the L2 cache and the mask
//! ROM's own `.data`/`.bss` sit inside it, at addresses that depend on the chip
//! revision, so `memory.x` splits what is left into `RAM` (`.data`, `.bss`, the
//! stack) and `RAM_HIGH` (the `.ram.high` section, for buffers too big for the
//! first). A region that has drifted onto cache memory still accepts writes —
//! they just do not survive cache traffic — which is why the check here writes a
//! pattern, thrashes the 128 kB L2 cache with a megabyte of PSRAM, and reads the
//! pattern back.
//!
//! The buffers are 96 kB of `.bss` and 256 kB of `.ram.high`: the first cannot
//! cross into the ROM's window, and the second is past it, in the window that a
//! single contiguous `memory.x` used to skip.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::ptr::addr_of_mut;

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::psram::{Psram, PsramConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Bytes of PSRAM the thrash walks: eight times the 128 kB L2 cache.
const THRASH: usize = 1 << 20;
/// Size of the buffer that has to live in `.bss`.
const LOW_LEN: usize = 96 * 1024;
/// Size of the buffer that only fits in the second window.
const HIGH_LEN: usize = 256 * 1024;

/// `.bss`: zeroed by the runtime, next to `.data` and the stack.
static mut LOW: [u8; LOW_LEN] = [0; LOW_LEN];
/// `.ram.high`: `NOLOAD`, zeroed by the runtime from linker symbols.
#[unsafe(link_section = ".ram.high")]
static mut HIGH: [u8; HIGH_LEN] = [0; HIGH_LEN];

unsafe extern "C" {
    /// First byte of `.data`.
    static _sdata: u8;
    /// First byte above `.bss`.
    static _ebss: u8;
    /// First byte above the stack.
    static _stack_start: u8;
    /// First byte of `.ram.high`.
    static _sramhigh: u8;
    /// First byte above `.ram.high`.
    static _eramhigh: u8;
}

/// Writes `seed + index` into every byte of `buffer`.
fn fill(buffer: &mut [u8], seed: u8) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = seed.wrapping_add(index as u8);
    }
}

/// Whether `buffer` still holds what [`fill`] wrote with `seed`.
fn intact(buffer: &[u8], seed: u8) -> bool {
    buffer
        .iter()
        .enumerate()
        .all(|(index, byte)| *byte == seed.wrapping_add(index as u8))
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let sdata = core::ptr::addr_of!(_sdata) as usize;
    let ebss = core::ptr::addr_of!(_ebss) as usize;
    let stack_start = core::ptr::addr_of!(_stack_start) as usize;
    let high = core::ptr::addr_of!(_sramhigh) as usize;
    let high_end = core::ptr::addr_of!(_eramhigh) as usize;
    defmt::info!(
        "RAM 0x{=usize:x}: data/bss 0x{=usize:x}..0x{=usize:x}, stack to 0x{=usize:x}",
        sdata,
        sdata,
        ebss,
        stack_start
    );
    defmt::info!(
        "RAM_HIGH 0x{=usize:x}..0x{=usize:x}, {} kB of .ram.high",
        high,
        high_end,
        (high_end - high) / 1024
    );

    let (low, high) = unsafe { (&mut *addr_of_mut!(LOW), &mut *addr_of_mut!(HIGH)) };
    assert!(low.iter().all(|byte| *byte == 0), ".bss was not zeroed");
    assert!(
        high.iter().all(|byte| *byte == 0),
        ".ram.high was not zeroed"
    );
    defmt::info!("both windows started out zeroed");

    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (_clocks, _lp_aon) = clocks.release();
    let psram = Psram::init(PsramConfig::from_efuse(&efuse)).expect("PSRAM init failed");
    let thrash = unsafe { core::slice::from_raw_parts_mut(psram.base(), THRASH) };

    fill(low, 0x11);
    fill(high, 0x22);

    // Two passes over a megabyte: writing it fills the cache, reading it walks
    // the whole thing again. Anything that is really cache memory is gone.
    for round in 0..2u8 {
        thrash.fill(round);
        let mut sum = 0u32;
        for byte in thrash.iter() {
            sum = sum.wrapping_add(u32::from(*byte));
        }
        core::hint::black_box(sum);
    }

    assert!(intact(low, 0x11), ".bss lost its contents to cache traffic");
    assert!(
        intact(high, 0x22),
        ".ram.high lost its contents to cache traffic"
    );
    defmt::info!(
        "{} kB of .bss and {} kB of .ram.high survived",
        LOW_LEN / 1024,
        HIGH_LEN / 1024
    );

    loop {
        esp32p4_hal::delay::delay_ms(1_000);
    }
}
