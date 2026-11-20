//! GDMA link descriptors.
//!
//! The engine does not take a buffer address and a length; it walks a list of
//! 3-word descriptors in memory (TRM 6.4.1):
//!
//! ```text
//!   word 0   size[11:0] length[23:12] ... err_eof[28] suc_eof[30] owner[31]
//!   word 1   buffer address
//!   word 2   next descriptor address, 0 to end the list
//! ```
//!
//! `owner` flips between CPU and engine: software sets it to [`Owner::Dma`]
//! before mounting a descriptor and the engine clears it when it is done.
//!
//! # Two views of the same memory
//!
//! On this chip both internal SRAM (`0x4FF0_0000`) and PSRAM
//! (`0x4800_0000`) are *cached* at their normal address. A descriptor list is
//! written by the core and updated by the engine several times per transfer,
//! which the caches do not keep coherent — an experiment on hardware shows the
//! engine rejecting a freshly mounted descriptor because it read the stale
//! `owner` bit out of memory.
//!
//! Descriptors therefore live on the *direct* alias (`+0x4000_0000`, see
//! [`cache::uncached_alias`]): every word access
//! in this module goes there, by the core and by the engine, so both always see
//! the same memory and no maintenance call is needed per descriptor.
//! [`Chain::mount`] drops the core's cached copy of the descriptors it
//! programs, so a line evicted later cannot write stale words back over them.
//!
//! Buffers are different: they belong to the caller, who may want to read them
//! through the cached address. Their addresses are stored in the descriptors
//! exactly as given, and the caller maintains the cache around a transfer — see
//! [`crate::cache`]. [`AhbDma::copy`](super::AhbDma::copy) does that for its own
//! transfers.

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

use super::Error;
use crate::cache;

/// Most bytes one descriptor can describe: the `size` and `length` fields are
/// 12 bits wide.
pub const MAX_LEN: usize = 0xfff;

const SIZE_SHIFT: u32 = 0;
const SIZE_MASK: u32 = 0x0fff << SIZE_SHIFT;
const LENGTH_SHIFT: u32 = 12;
const LENGTH_MASK: u32 = 0x0fff << LENGTH_SHIFT;
/// The peripheral flagged the received data as erroneous (UHCI and PARLIO
/// only). Set by hardware on receive descriptors.
const ERR_EOF: u32 = 1 << 28;
/// An EOF descriptor completed, or, on transmit, send an EOF to the peripheral.
const SUC_EOF: u32 = 1 << 30;
/// The engine owns the buffer while this bit is set.
const OWNER_DMA: u32 = 1 << 31;

/// Who may touch the buffer a descriptor points at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Owner {
    /// The core owns the buffer; the engine skips the descriptor.
    Cpu,
    /// The engine owns the buffer; the core must not touch it.
    Dma,
}

/// Descriptor word with volatile access, since the engine writes to it too.
#[repr(transparent)]
struct Word(UnsafeCell<u32>);

impl Word {
    const fn new(value: u32) -> Self {
        Self(UnsafeCell::new(value))
    }

    #[inline]
    fn get(&self) -> u32 {
        // SAFETY: the word lives in memory the engine also touches, so every
        // access must stay a real memory access.
        unsafe { read_volatile(self.0.get()) }
    }

    #[inline]
    fn set(&self, value: u32) {
        // SAFETY: see `get`.
        unsafe { write_volatile(self.0.get(), value) }
    }

    #[inline]
    fn modify(&self, f: impl FnOnce(u32) -> u32) {
        self.set(f(self.get()));
    }
}

/// One link descriptor.
///
/// The words are accessed through `&self`: the engine changes `length`, `owner`
/// and the EOF flags behind the core's back, so a [`Chain`] can hand out shared
/// references to a caller-owned array.
#[repr(C, align(4))]
pub struct Descriptor {
    word0: Word,
    buffer: Word,
    next: Word,
}

impl Descriptor {
    /// A zeroed descriptor, usable in `static` initialisers.
    pub const fn new() -> Self {
        Self {
            word0: Word::new(0),
            buffer: Word::new(0),
            next: Word::new(0),
        }
    }

    /// This descriptor on the direct alias, i.e. the copy the engine sees.
    #[inline]
    fn hw(&self) -> &Self {
        // SAFETY: the alias maps the same memory; `Descriptor` is `repr(C)` and
        // holds nothing but words, so reinterpreting it there is sound.
        unsafe { &*cache::uncached_alias(self as *const Self as *mut u8).cast::<Self>() }
    }

    /// Points the descriptor at `len` bytes starting at `buffer`.
    ///
    /// Both the buffer size and the length are set to `len`: the engine ignores
    /// `size` on transmit and overwrites `length` on receive, so one value
    /// describes both directions.
    ///
    /// The address is stored as given; a cached buffer needs cache maintenance
    /// by the caller, a buffer addressed through its direct alias does not.
    ///
    /// # Panics
    ///
    /// Panics if `len` exceeds [`MAX_LEN`].
    pub fn mount(&self, buffer: *mut u8, len: usize) {
        assert!(len <= MAX_LEN, "descriptor payload exceeds {MAX_LEN} bytes");
        let len = len as u32;
        self.hw().buffer.set(buffer as usize as u32);
        self.hw()
            .word0
            .set((len << LENGTH_SHIFT) & LENGTH_MASK | (len << SIZE_SHIFT) & SIZE_MASK);
    }

    /// Sets the next descriptor, or ends the list with `None`.
    ///
    /// The address stored is the descriptor's ordinary one, not the direct
    /// alias: the engine only accepts list addresses inside its access window,
    /// and that window is described in cached addresses.
    pub fn set_next(&self, next: Option<&Descriptor>) {
        self.hw()
            .next
            .set(next.map_or(0, |d| d as *const Descriptor as usize as u32));
    }

    /// The next descriptor as the engine sees it, or `None` at the end of the
    /// list.
    pub fn next(&self) -> Option<*const Descriptor> {
        match self.hw().next.get() {
            0 => None,
            addr => Some(addr as usize as *const Descriptor),
        }
    }

    /// Hands the buffer to the engine or back to the core.
    pub fn set_owner(&self, owner: Owner) {
        self.hw().word0.modify(|word0| match owner {
            Owner::Dma => word0 | OWNER_DMA,
            Owner::Cpu => word0 & !OWNER_DMA,
        });
    }

    /// Reads back the owner bit, as the engine left it.
    pub fn owner(&self) -> Owner {
        if self.hw().word0.get() & OWNER_DMA != 0 {
            Owner::Dma
        } else {
            Owner::Cpu
        }
    }

    /// Marks the descriptor as the end of a transfer phase.
    ///
    /// On transmit the engine appends an EOF to the data it sends; on receive
    /// the engine sets this bit itself once data carrying an EOF arrives.
    pub fn set_eof(&self, eof: bool) {
        self.hw().word0.modify(|word0| {
            if eof {
                word0 | SUC_EOF
            } else {
                word0 & !SUC_EOF
            }
        });
    }

    /// Reads back the EOF flag.
    pub fn eof(&self) -> bool {
        self.hw().word0.get() & SUC_EOF != 0
    }

    /// Sets how many bytes of the buffer are valid.
    ///
    /// Only meaningful on transmit, where software fills it in; on receive the
    /// engine writes the number of bytes it stored.
    ///
    /// # Panics
    ///
    /// Panics if `len` exceeds [`MAX_LEN`].
    pub fn set_length(&self, len: usize) {
        assert!(len <= MAX_LEN, "descriptor payload exceeds {MAX_LEN} bytes");
        let len = len as u32;
        self.hw()
            .word0
            .modify(|word0| (word0 & !LENGTH_MASK) | ((len << LENGTH_SHIFT) & LENGTH_MASK));
    }

    /// Number of valid bytes in the buffer.
    ///
    /// After a receive descriptor was handed back, this is how much the engine
    /// stored, not the capacity the descriptor was mounted with.
    pub fn length(&self) -> usize {
        ((self.hw().word0.get() & LENGTH_MASK) >> LENGTH_SHIFT) as usize
    }

    /// Buffer capacity the descriptor was mounted with.
    pub fn size(&self) -> usize {
        ((self.hw().word0.get() & SIZE_MASK) >> SIZE_SHIFT) as usize
    }

    /// Buffer address, as handed to the engine.
    pub fn buffer(&self) -> *mut u8 {
        self.hw().buffer.get() as usize as *mut u8
    }

    /// Whether the peripheral flagged the received data as erroneous.
    pub fn error(&self) -> bool {
        self.hw().word0.get() & ERR_EOF != 0
    }
}

impl Default for Descriptor {
    fn default() -> Self {
        Self::new()
    }
}

/// Where a chain ends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Terminator {
    /// The last descriptor's `next` is 0.
    Null,
    /// The last descriptor's `next` points back at the head: the engine starts
    /// over, which is what a receive ring wants.
    Ring,
}

/// A slice of descriptors used as one list.
///
/// Building one is free: it only borrows the caller's descriptors. The
/// descriptors themselves have to live in memory the engine can reach and must
/// stay put while mounted; internal SRAM or the PSRAM window both work.
pub struct Chain<'a> {
    descriptors: &'a [Descriptor],
}

impl<'a> Chain<'a> {
    /// Wraps a caller-owned descriptor array.
    #[inline]
    pub const fn new(descriptors: &'a [Descriptor]) -> Self {
        Self { descriptors }
    }

    /// The descriptors behind the chain.
    #[inline]
    pub const fn descriptors(&self) -> &'a [Descriptor] {
        self.descriptors
    }

    /// Number of descriptors in the chain.
    #[inline]
    pub const fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Whether the chain has no descriptors at all.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    /// The first descriptor, i.e. what to program into the channel.
    ///
    /// # Panics
    ///
    /// Panics if the chain is empty.
    #[inline]
    pub fn head(&self) -> &'a Descriptor {
        &self.descriptors[0]
    }

    /// Descriptor `index`, or `None` past the end.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&'a Descriptor> {
        self.descriptors.get(index)
    }

    /// Mounts `len` bytes at `buffer` over the chain and hands them to the
    /// engine, ending the list after the last used descriptor.
    ///
    /// Returns the number of descriptors used — the buffer may need fewer than
    /// the chain has, in which case the rest is left alone.
    pub fn mount(&self, buffer: *mut u8, len: usize) -> Result<usize, Error> {
        self.mount_with(buffer, len, Terminator::Null)
    }

    /// Like [`mount`](Self::mount), but the last descriptor also carries the
    /// EOF flag.
    ///
    /// A transmit chain has to be terminated this way: without an EOF the
    /// peripheral never learns that the phase ended, and the transfer
    /// completion interrupt never fires.
    pub fn mount_eof(&self, buffer: *mut u8, len: usize) -> Result<usize, Error> {
        let used = self.mount_with(buffer, len, Terminator::Null)?;
        self.descriptors[used - 1].set_eof(true);
        Ok(used)
    }

    /// Mounts `len` bytes at `buffer` as a ring: the last used descriptor
    /// points back at the head, so the engine starts over after it.
    ///
    /// No descriptor carries the EOF flag; a receive ring cycles until software
    /// stops the channel.
    pub fn mount_ring(&self, buffer: *mut u8, len: usize) -> Result<usize, Error> {
        self.mount_with(buffer, len, Terminator::Ring)
    }

    fn mount_with(&self, buffer: *mut u8, len: usize, end: Terminator) -> Result<usize, Error> {
        if len == 0 {
            return Err(Error::EmptyBuffer);
        }
        let used = len.div_ceil(MAX_LEN);
        if used > self.descriptors.len() {
            return Err(Error::NotEnoughDescriptors {
                needed: used,
                available: self.descriptors.len(),
            });
        }

        // The descriptors are programmed through the direct alias, so drop any
        // cached copy first: a dirty line evicted later would otherwise write
        // stale words back over what the engine is using.
        cache::writeback_invalidate(
            self.descriptors.as_ptr().cast(),
            used * core::mem::size_of::<Descriptor>(),
        );

        for (index, descriptor) in self.descriptors[..used].iter().enumerate() {
            let offset = index * MAX_LEN;
            descriptor.set_eof(false);
            descriptor.mount(buffer.wrapping_add(offset), (len - offset).min(MAX_LEN));
            let next = match index + 1 {
                i if i < used => Some(&self.descriptors[i]),
                _ => match end {
                    Terminator::Null => None,
                    Terminator::Ring => Some(&self.descriptors[0]),
                },
            };
            descriptor.set_next(next);
            descriptor.set_owner(Owner::Dma);
        }
        Ok(used)
    }
}
