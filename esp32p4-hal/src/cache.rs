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
//! # Suspending the cache
//!
//! Retuning the flash or PSRAM interface (changing a frequency, remapping an
//! MMU page) requires the external-memory cache to stop fetching:
//!
//! ```ignore
//! #[ramfunc]
//! fn retune() {
//!     let state = unsafe { cache::suspend() };
//!     // Program MSPI/flash timing here; this function runs from SRAM and does
//!     // not touch external memory.
//!     unsafe { cache::resume(state) };
//! }
//! ```
//!
//! The wrappers themselves live in `.ramfunc`, so returning from `suspend` and
//! entering `resume` never touches flash; the code between the two calls must
//! run from RAM as well.
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
pub const LINE_SIZE: usize = 64;
const CACHE_LINE_SIZE: usize = LINE_SIZE;

/// `CACHE_MAP_L1_DCACHE | CACHE_MAP_L2_CACHE`: both levels that can hold a copy
/// of a DMA buffer.
const CACHE_MAP_L1D_AND_L2: u32 = 0x30;

unsafe extern "C" {
    /// ROM: write back (clean) every line covering the range.
    fn Cache_WriteBack_Addr(map: u32, addr: u32, size: u32) -> i32;
    /// ROM: write back and invalidate every line covering the range.
    fn Cache_WriteBack_Invalidate_Addr(map: u32, addr: u32, size: u32) -> i32;
    /// ROM: invalidate every line covering the range.
    fn Cache_Invalidate_Addr(map: u32, addr: u32, size: u32) -> i32;
    /// ROM: stop L1 instruction cache 0.
    fn Cache_Suspend_L1_CORE0_ICache() -> u32;
    /// ROM: resume L1 instruction cache 0.
    fn Cache_Resume_L1_CORE0_ICache(autoload: u32);
    /// ROM: stop L1 instruction cache 1.
    fn Cache_Suspend_L1_CORE1_ICache() -> u32;
    /// ROM: resume L1 instruction cache 1.
    fn Cache_Resume_L1_CORE1_ICache(autoload: u32);
    /// ROM: stop the L1 data cache.
    fn Cache_Suspend_L1_DCache() -> u32;
    /// ROM: resume the L1 data cache.
    fn Cache_Resume_L1_DCache(autoload: u32);
    /// ROM: stop the L2 cache.
    fn Cache_Suspend_L2_Cache() -> u32;
    /// ROM: resume the L2 cache.
    fn Cache_Resume_L2_Cache(autoload: u32);
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
/// The lines stay cached and readable; only their dirty bits are cleared.
#[inline]
pub fn writeback(addr: *const c_void, len: usize) {
    if len == 0 {
        return;
    }
    let (start, size) = line_range(addr.cast(), len);
    // SAFETY: the ROM routine only performs cache maintenance on the range.
    unsafe { Cache_WriteBack_Addr(CACHE_MAP_L1D_AND_L2, start, size) };
}

/// Push CPU writes out and drop the cached copies.
///
/// Use this when a buffer is handed to a bus master and then read back by the
/// CPU: the writeback makes the DMA read the final data, the invalidate makes
/// the CPU see the data the bus master writes afterwards.
#[inline]
pub fn writeback_invalidate(addr: *const c_void, len: usize) {
    if len == 0 {
        return;
    }
    let (start, size) = line_range(addr.cast(), len);
    // SAFETY: the ROM routine only performs cache maintenance on the range.
    unsafe { Cache_WriteBack_Invalidate_Addr(CACHE_MAP_L1D_AND_L2, start, size) };
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

/// Saved autoload state for [`suspend`]/[`resume`].
#[derive(Clone, Copy, Debug)]
pub struct Suspended {
    core0_icache: u32,
    core1_icache: u32,
    dcache: u32,
    l2: u32,
}

/// Stop L1 instruction/data caches and the L2 cache.
///
/// # Safety
///
/// While the caches are stopped the core must not fetch instructions or data
/// from flash or PSRAM. Call this (and [`resume`]) from a `#[ramfunc]` function
/// and keep the whole critical section in RAM.
#[esp32p4_macro::ramfunc]
pub unsafe fn suspend() -> Suspended {
    // SAFETY: the caller guarantees the cache is not needed until `resume`.
    unsafe {
        let core0_icache = Cache_Suspend_L1_CORE0_ICache();
        let core1_icache = Cache_Suspend_L1_CORE1_ICache();
        let dcache = Cache_Suspend_L1_DCache();
        let l2 = Cache_Suspend_L2_Cache();
        Suspended {
            core0_icache,
            core1_icache,
            dcache,
            l2,
        }
    }
}

/// Resume the caches stopped by [`suspend`].
///
/// # Safety
///
/// See [`suspend`]: the caller must still be running from RAM.
#[esp32p4_macro::ramfunc]
pub unsafe fn resume(state: Suspended) {
    // Resume in reverse order, so L2 is back before the L1s that fill from it.
    // SAFETY: the state came from `suspend`, and the caller is still in RAM.
    unsafe {
        Cache_Resume_L2_Cache(state.l2);
        Cache_Resume_L1_DCache(state.dcache);
        Cache_Resume_L1_CORE1_ICache(state.core1_icache);
        Cache_Resume_L1_CORE0_ICache(state.core0_icache);
    }
}

// ============================================================================
// Direct aliases
// ============================================================================

/// HP L2MEM through the cache.
pub const L2MEM_START: usize = 0x4ff0_0000;
/// End of the HP L2MEM window.
pub const L2MEM_END: usize = 0x4ffc_0000;
/// PSRAM through the cache, i.e. [`crate::mmu::CACHED_BASE`].
pub const PSRAM_START: usize = crate::mmu::CACHED_BASE;
/// End of the PSRAM window.
pub const PSRAM_END: usize =
    crate::mmu::CACHED_BASE + crate::mmu::PAGE_COUNT * crate::mmu::PAGE_SIZE;
/// Distance between a cached window and its direct alias.
pub const ALIAS_OFFSET: usize = 0x4000_0000;

/// The same memory, reached without going through the cache.
///
/// Both internal SRAM (`0x4FF0_0000`) and the PSRAM mapping (`0x4800_0000`) are
/// cached at their normal address and reachable directly at `+0x4000_0000`
/// (TRM 9.3.1). Handing a bus master the direct address keeps it off the cache
/// hierarchy entirely: its reads and writes land in memory, not in a cache
/// line, so neither side can see a stale copy and no maintenance is needed.
///
/// Addresses outside the two cached windows — already-direct aliases, the
/// uncached peripherals, LP memory — are returned unchanged.
///
/// The returned pointer refers to the same physical memory as `addr`; using
/// both views of one buffer at the same time is what the caches would not
/// tolerate, so pick one view per buffer.
#[inline]
pub fn uncached_alias(addr: *mut u8) -> *mut u8 {
    let value = addr as usize;
    if (L2MEM_START..L2MEM_END).contains(&value) || (PSRAM_START..PSRAM_END).contains(&value) {
        (value + ALIAS_OFFSET) as *mut u8
    } else {
        addr
    }
}
