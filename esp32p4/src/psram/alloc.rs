//! Heap allocator backed by PSRAM.
//!
//! Two ways to use it:
//!
//! ```ignore
//! use esp32p4::psram::{Psram, PsramAllocator, PsramConfig};
//!
//! #[global_allocator]
//! static HEAP: PsramAllocator = PsramAllocator::empty();
//!
//! let psram = Psram::init(PsramConfig::default()).unwrap();
//! HEAP.init(&psram);
//!
//! // Through the global allocator (`Box`/`Vec`/`String`).
//! let boxed = alloc::boxed::Box::new(42u32);
//!
//! // Or as an explicit `Allocator` (stable since Rust 1.100), keeping other
//! // allocations on a different heap.
//! let mut v: alloc::vec::Vec<u32, &PsramAllocator> = alloc::vec::Vec::new_in(&HEAP);
//! v.push(1);
//! ```

use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::NonNull;

use linked_list_allocator::LockedHeap;

use super::Psram;

/// First-fit heap living in PSRAM.
pub struct PsramAllocator {
    heap: LockedHeap,
}

impl PsramAllocator {
    /// Creates an empty allocator; call [`init`](Self::init) before allocating.
    pub const fn empty() -> Self {
        Self {
            heap: LockedHeap::empty(),
        }
    }

    /// Places the heap over the whole mapped PSRAM window.
    pub fn init(&self, psram: &Psram) {
        let (ptr, len) = psram.raw_parts();
        // SAFETY: the window is mapped, exclusively owned and never aliased by
        // other allocations.
        unsafe { self.heap.lock().init(ptr, len) };
    }

    /// Places the heap over `[start, start + size)`.
    ///
    /// # Safety
    ///
    /// The region must be valid, writable RAM that is not used for anything
    /// else.
    pub unsafe fn init_raw(&self, start: *mut u8, size: usize) {
        unsafe { self.heap.lock().init(start, size) };
    }
}

unsafe impl GlobalAlloc for PsramAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.heap.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.heap.dealloc(ptr, layout) }
    }
}

// SAFETY: every operation forwards to the heap's `GlobalAlloc` implementation,
// whose contract matches `Allocator`'s.
unsafe impl Allocator for PsramAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: the heap reports allocation failure with a null pointer.
        let ptr = unsafe { self.heap.alloc(layout) };
        let ptr = NonNull::new(ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { self.heap.dealloc(ptr.as_ptr(), layout) }
    }
}
