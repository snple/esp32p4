//! In-package PSRAM support.
//!
//! ESP32-P4 dies with embedded PSRAM stack 16 MB or 32 MB of pseudo-static RAM
//! on top of the SoC, wired to dedicated MSPI pads. It is invisible until the
//! MSPI controllers, the PHY and the MMU are programmed, so [`Psram::init`]
//! runs the full bring-up and maps the die into the `0x4800_0000` window.
//!
//! ```text
//! 0x4800_0000 .. 0x4BFF_FFFF   cached window (goes through L2)
//! 0x8800_0000 .. 0x8BFF_FFFF   uncached alias of the same pages
//! ```
//!
//! # Features
//!
//! - 16-line (hex, default) and 8-line (oct) DDR interfaces
//! - Automatic density detection through mode register 2 (16/32 MB)
//! - 250 MHz bus clock from the MPLL, no PLL configuration from the
//!   application required
//! - Optional allocator: [`PsramAllocator`]
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::psram::{Psram, PsramConfig};
//!
//! let psram = Psram::init(PsramConfig::default()).unwrap();
//! let (ptr, len) = psram.raw_parts();
//! let ram = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
//! ram[0] = 0x5a;
//! ```
//!
//! # Cache coherency
//!
//! CPU accesses go through the L2 cache. Peripherals using DMA should either
//! use the uncached alias (`0x8800_0000`) or maintain the cache around the
//! transfer.

mod alloc;
mod mpll;
mod mspi;
mod pads;

pub use alloc::PsramAllocator;

use core::sync::atomic::{AtomicBool, Ordering};

/// PSRAM bus width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsramMode {
    /// 16-line DDR, the interface used by the in-package dies.
    #[default]
    Hex,
    /// 8-line DDR.
    Oct,
}

/// Requested PSRAM size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsramSize {
    /// Ask the die through mode register 2.
    #[default]
    Auto,
    /// Map a fixed number of bytes (clamped to the detected density).
    Bytes(usize),
}

/// PSRAM configuration.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PsramConfig {
    /// Bus width.
    pub mode: PsramMode,
    /// Size to map.
    pub size: PsramSize,
    /// Enable the silicon revision 3.0 workaround.
    ///
    /// ESP32-P4 v3.0 (ECO5) needs two dummy reads and a controller reset before
    /// the MMU mapping is committed. Enable it when running on such a die.
    pub rev3_workaround: bool,
    /// Write and read back a pattern after mapping.
    pub verify: bool,
}

impl Default for PsramConfig {
    fn default() -> Self {
        Self {
            mode: PsramMode::Hex,
            size: PsramSize::Auto,
            rev3_workaround: false,
            verify: true,
        }
    }
}

/// Errors reported by [`Psram::init`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The PSRAM configuration registers did not answer in time.
    Timeout,
    /// The density bits of mode register 2 are unknown.
    NotDetected,
    /// The requested bus clock does not divide the MPLL output.
    UnsupportedClock,
    /// The read-back check after mapping failed.
    VerifyFailed,
    /// PSRAM has already been initialized.
    AlreadyInitialized,
}

/// Handle to the mapped PSRAM window.
pub struct Psram {
    start: usize,
    size: usize,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

impl Psram {
    /// Initializes the in-package PSRAM and maps it at `0x4800_0000`.
    pub fn init(config: PsramConfig) -> Result<Self, Error> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(Error::AlreadyInitialized);
        }

        debug!(
            "PSRAM: initialising ({}, rev3 workaround: {})",
            config.mode, config.rev3_workaround
        );

        mpll::enable();

        let is_hex = matches!(config.mode, PsramMode::Hex);
        let detected = match mspi::init(&mspi::MHZ_250, is_hex, config.rev3_workaround) {
            Ok(size) => size,
            Err(err) => {
                INITIALIZED.store(false, Ordering::Release);
                return Err(err);
            }
        };
        let size = match config.size {
            PsramSize::Auto => detected,
            PsramSize::Bytes(bytes) => bytes.min(detected),
        };
        mspi::map(size);
        info!("PSRAM: {} bytes mapped at {:#x}", size, mspi::BASE);

        let psram = Self {
            start: mspi::BASE,
            size,
        };

        if config.verify && !psram.check() {
            warn!("PSRAM: read-back check failed");
            INITIALIZED.store(false, Ordering::Release);
            return Err(Error::VerifyFailed);
        }

        Ok(psram)
    }

    /// First address of the mapped window.
    #[inline]
    pub const fn base(&self) -> *mut u8 {
        self.start as *mut u8
    }

    /// Size of the mapped window in bytes.
    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Raw pointer and length of the mapped window.
    #[inline]
    pub const fn raw_parts(&self) -> (*mut u8, usize) {
        (self.base(), self.size)
    }

    /// Uncached alias of the mapped window, bypassing the L2 cache.
    ///
    /// Use this for DMA buffers or when the cache must not interfere.
    #[inline]
    pub const fn uncached_base(&self) -> *mut u8 {
        (mspi::UNCACHED_BASE + (self.start - mspi::BASE)) as *mut u8
    }

    /// Writes a pattern to the start of the window and reads it back.
    ///
    /// The check goes through the uncached alias so it exercises the PSRAM die
    /// instead of the L2 cache.
    pub fn check(&self) -> bool {
        const PATTERN: &[u32] = &[0x5555_5555, 0xaaaa_aaaa, 0x0123_4567, 0xfedc_ba98];

        let base = self.uncached_base() as *mut u32;
        // SAFETY: the window was mapped and is large enough for the pattern.
        unsafe {
            for (i, value) in PATTERN.iter().enumerate() {
                base.add(i).write_volatile(*value);
            }
            for (i, value) in PATTERN.iter().enumerate() {
                if base.add(i).read_volatile() != *value {
                    return false;
                }
            }
        }
        true
    }
}
