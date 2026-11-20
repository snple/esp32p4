//! External-memory MMU window used by the PSRAM controller.
//!
//! The CPU reaches external RAM through 1024 MMU entries of 64 KB each. Entry
//! `n` backs the virtual page at `0x4800_0000 + n * 64 KB`; the same page is
//! also visible through the uncached alias `0x8800_0000 + n * 64 KB`.
//!
//! ```text
//!   physical page 0 ─┬─► MMU entry 0   ─► 0x4800_0000 (cached, through L2)
//!                    └────────────────► 0x8800_0000 (uncached alias)
//! ```
//!
//! [`Region::map`] programs `SPI_MEM_MMU_ITEM_*` and tracks which entries are in
//! use, so two callers cannot silently overwrite each other's mapping. The
//! controller itself is brought up by [`crate::psram`]; mapping before that
//! clocks the MSPI block is not supported.
//!
//! # Features
//!
//! - Page-granular map/unmap with a shared ownership bitmap
//! - Cached and uncached addresses for the mapped window
//! - Cache invalidation around map/unmap
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::mmu::Region;
//!
//! // Alias physical page 0 at entry 512 (offset 32 MB into the window).
//! let alias = Region::map(0, 512, 1).unwrap();
//! let ptr = alias.cached_ptr();
//! // ...
//! alias.unmap();
//! ```

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_pac::PSRAM_MSPI0;

/// MMU page size in bytes.
pub const PAGE_SIZE: usize = 64 * 1024;
/// Number of MMU entries (the cached and uncached windows are this large).
pub const PAGE_COUNT: usize = 1024;
/// Start of the cached external-memory window.
pub const CACHED_BASE: usize = 0x4800_0000;
/// Start of the uncached alias of the same window.
pub const UNCACHED_BASE: usize = 0x8800_0000;

/// `SPI_MMU_ITEM_CONTENT.ACCESS_SPIRAM`.
const ACCESS_SPIRAM: u32 = 1 << 10;
/// `SPI_MMU_ITEM_CONTENT.VALID`.
const VALID: u32 = 1 << 11;
/// Width of the physical page number in `SPI_MMU_ITEM_CONTENT`.
const PHYSICAL_PAGE_MASK: u32 = 0x3ff;

const BITMAP_WORDS: usize = PAGE_COUNT.div_ceil(32);

/// Entries handed out by [`Region::map`].
static MAPPED: [AtomicU32; BITMAP_WORDS] = [const { AtomicU32::new(0) }; BITMAP_WORDS];

/// Errors reported by [`Region::map`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A region must contain at least one page.
    Empty,
    /// The MMU entry range does not fit in the 1024-entry window.
    EntryOutOfRange,
    /// The physical page range does not fit in the 10-bit page number.
    PhysicalOutOfRange,
    /// At least one of the requested MMU entries is already mapped.
    AlreadyMapped,
}

/// A mapped range of external-memory pages.
///
/// The handle tracks ownership of its MMU entries; call [`Region::unmap`] to
/// release them. Dropping the handle leaves the mapping in place, because the
/// memory may still be in use by another owner.
#[derive(PartialEq, Eq, Debug)]
pub struct Region {
    physical_page: u16,
    first_entry: u16,
    pages: u16,
}

impl Region {
    /// Map `pages` consecutive physical pages at `first_entry`.
    ///
    /// `physical_page` and `first_entry` are page indices, not addresses; both
    /// ranges must fit in the 1024-entry window. The entries are claimed
    /// atomically, so a concurrent or overlapping mapping fails with
    /// [`Error::AlreadyMapped`] instead of corrupting the table.
    pub fn map(physical_page: u32, first_entry: u32, pages: u32) -> Result<Self, Error> {
        if pages == 0 {
            return Err(Error::Empty);
        }
        if first_entry as usize + pages as usize > PAGE_COUNT {
            return Err(Error::EntryOutOfRange);
        }
        if physical_page as usize + pages as usize > PAGE_COUNT {
            return Err(Error::PhysicalOutOfRange);
        }

        for (claimed, entry) in (first_entry..first_entry + pages).enumerate() {
            let word = &MAPPED[(entry / 32) as usize];
            let bit = 1 << (entry % 32);
            if word.fetch_or(bit, Ordering::AcqRel) & bit != 0 {
                // Put back the entries claimed by this call before failing.
                for entry in first_entry..first_entry + claimed as u32 {
                    MAPPED[(entry / 32) as usize].fetch_and(!(1 << (entry % 32)), Ordering::AcqRel);
                }
                return Err(Error::AlreadyMapped);
            }
        }

        let region = Self {
            physical_page: physical_page as u16,
            first_entry: first_entry as u16,
            pages: pages as u16,
        };
        region.program();

        // Drop any line the cache may hold for the window we just changed.
        crate::cache::invalidate(region.cached_ptr().cast(), region.size());
        Ok(region)
    }

    /// Unmap the region and release its MMU entries.
    pub fn unmap(self) {
        // Invalidate while the mapping is still valid, so a line belonging to
        // the previous mapping cannot be reused after the entries are free.
        crate::cache::invalidate(self.cached_ptr().cast(), self.size());

        let spi = unsafe { PSRAM_MSPI0::steal() };
        for entry in self.entry_range() {
            spi.spi_mem_mmu_item_index
                .write_spi_mmu_item_index(entry as u32);
            spi.spi_mem_mmu_item_content.write_bits(0);
        }

        for entry in self.entry_range() {
            MAPPED[entry / 32].fetch_and(!(1 << (entry % 32)), Ordering::AcqRel);
        }
    }

    /// First MMU entry of the region.
    #[inline]
    pub const fn first_entry(&self) -> u16 {
        self.first_entry
    }

    /// Number of mapped pages.
    #[inline]
    pub const fn pages(&self) -> u16 {
        self.pages
    }

    /// First physical page.
    #[inline]
    pub const fn physical_page(&self) -> u16 {
        self.physical_page
    }

    /// Size of the region in bytes.
    #[inline]
    pub const fn size(&self) -> usize {
        self.pages as usize * PAGE_SIZE
    }

    /// Whether the region contains no pages (always false for a mapped region).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.pages == 0
    }

    /// Address of the cached window.
    #[inline]
    pub const fn cached_addr(&self) -> usize {
        CACHED_BASE + self.first_entry as usize * PAGE_SIZE
    }

    /// Address of the uncached alias of the same pages.
    #[inline]
    pub const fn uncached_addr(&self) -> usize {
        UNCACHED_BASE + self.first_entry as usize * PAGE_SIZE
    }

    /// Pointer to the cached window.
    #[inline]
    pub const fn cached_ptr(&self) -> *mut u8 {
        self.cached_addr() as *mut u8
    }

    /// Pointer to the uncached alias, for DMA buffers that need no maintenance.
    #[inline]
    pub const fn uncached_ptr(&self) -> *mut u8 {
        self.uncached_addr() as *mut u8
    }

    fn entry_range(&self) -> impl Iterator<Item = usize> {
        self.first_entry as usize..self.first_entry as usize + self.pages as usize
    }

    fn program(&self) {
        let spi = unsafe { PSRAM_MSPI0::steal() };
        for (offset, entry) in self.entry_range().enumerate() {
            spi.spi_mem_mmu_item_index
                .write_spi_mmu_item_index(entry as u32);
            let page = self.physical_page as u32 + offset as u32;
            spi.spi_mem_mmu_item_content
                .write_bits((page & PHYSICAL_PAGE_MASK) | ACCESS_SPIRAM | VALID);
        }
    }
}
