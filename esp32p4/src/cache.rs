//! L1 data cache maintenance for DMA buffers.
//!
//! The ESP32-P4 core runs with its L1 D-cache enabled, while bus masters such as
//! the Ethernet MAC and the DMA engines read and write physical memory directly.
//! Any buffer shared with them has to be brought into a well-defined state:
//!
//! ```text
//!   CPU writes buffer ──► writeback() ──► DMA reads memory
//!   DMA writes memory ──► invalidate() ──► CPU reads buffer
//! ```
//!
//! Both helpers round down to cache line boundaries and dispatch to the mask ROM
//! routines (`Cache_WriteBack_Addr` / `Cache_Invalidate_Addr`), so the whole
//! address range containing `[addr, addr + len)` is covered. Calls are cheap but
//! not free: trim buffers to what actually moves.
//!
//! # Uncached memory
//!
//! Buffers placed in the PSRAM uncached window (`0x8800_0000`, see
//! [`crate::psram`]) or any other non-cacheable region need no maintenance at
//! all. Calling these functions for them is harmless.

use core::ffi::c_void;

/// Line size assumed when aligning the range.
///
/// The P4 L1 D-cache uses 64-byte lines; the ROM routine handles the smaller L2
/// line size internally, so aligning to the larger of the two is enough. Reading
/// the real value would cost a ROM call on every transfer.
const CACHE_LINE_SIZE: usize = 64;

/// `CACHE_MAP_L1_DCACHE | CACHE_MAP_L2_CACHE`: both levels that can hold a copy
/// of a DMA buffer.
const CACHE_MAP_L1D_AND_L2: u32 = 0x30;

unsafe extern "C" {
    /// ROM: write back (clean) and invalidate every line covering the range.
    fn Cache_WriteBack_Addr(map: u32, addr: u32, size: u32) -> i32;
    /// ROM: invalidate every line covering the range.
    fn Cache_Invalidate_Addr(map: u32, addr: u32, size: u32) -> i32;
}

/// Align `addr`/`len` to whole cache lines.
#[inline]
fn line_range(addr: *const u8, len: usize) -> (u32, u32) {
    let start = (addr as usize) & !(CACHE_LINE_SIZE - 1);
    let end = (addr as usize)
        .saturating_add(len)
        .div_ceil(CACHE_LINE_SIZE)
        * CACHE_LINE_SIZE;
    (start as u32, (end - start) as u32)
}

/// Push CPU writes out to memory so a bus master sees them.
///
/// Also invalidates the lines, so the CPU must not keep using the buffer
/// afterwards without re-reading it.
#[inline]
pub fn writeback(addr: *const c_void, len: usize) {
    if len == 0 {
        return;
    }
    let (start, size) = line_range(addr.cast(), len);
    // SAFETY: the ROM routine only performs cache maintenance on the range.
    unsafe { Cache_WriteBack_Addr(CACHE_MAP_L1D_AND_L2, start, size) };
}

/// Drop cached copies so the CPU re-reads what a bus master wrote.
///
/// Any dirty line in the range is lost, which is exactly what a receive buffer
/// wants.
#[inline]
pub fn invalidate(addr: *const c_void, len: usize) {
    if len == 0 {
        return;
    }
    let (start, size) = line_range(addr.cast(), len);
    // SAFETY: the ROM routine only performs cache maintenance on the range.
    unsafe { Cache_Invalidate_Addr(CACHE_MAP_L1D_AND_L2, start, size) };
}
