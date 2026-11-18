//! PSRAM PHY pad configuration.
//!
//! The PSRAM die is connected through dedicated MSPI pads that are *not* part
//! of `IO_MUX` (and therefore not in the PAC): they live in their own
//! `IOMUX_MSPI_PIN` block at `0x500E_1200`. Each pad register carries a drive
//! strength field in `[13:12]`; the two DQS pads additionally need their
//! output driver powered up (`XPD`, bit 0), which is what enables the DDR
//! strobe latch.
//!
//! The ROM bootloader does not touch these registers unless it initializes
//! PSRAM itself, so the full set is programmed here.

/// `IOMUX_MSPI_PIN` block base address.
const BASE: usize = 0x500e_1200;

/// Register offsets of the PSRAM pads.
const PSRAM_D: usize = 0x1c;
const PSRAM_Q: usize = 0x20;
const PSRAM_WP: usize = 0x24;
const PSRAM_HOLD: usize = 0x28;
const PSRAM_DQ4: usize = 0x2c;
const PSRAM_DQ5: usize = 0x30;
const PSRAM_DQ6: usize = 0x34;
const PSRAM_DQ7: usize = 0x38;
const PSRAM_DQS0: usize = 0x3c;
const PSRAM_CK: usize = 0x40;
const PSRAM_CS: usize = 0x44;
const PSRAM_DQ8: usize = 0x48;
const PSRAM_DQS1: usize = 0x68;

/// Drive strength field, `[13:12]`.
const DRV_SHIFT: u32 = 12;
const DRV_MASK: u32 = 0b11 << DRV_SHIFT;
/// 20 mA, matching the value IDF's timing calibration uses.
const DRV_20MA: u32 = 2;

/// DQS output driver enable.
const XPD: u32 = 1 << 0;

#[inline]
fn modify(offset: usize, f: impl FnOnce(u32) -> u32) {
    // SAFETY: `IOMUX_MSPI_PIN` is a fixed register block nobody else owns, and
    // the offsets are all inside it.
    let ptr = (BASE + offset) as *mut u32;
    unsafe { ptr.write_volatile(f(ptr.read_volatile())) };
}

#[inline]
fn set_drive_strength(offset: usize) {
    modify(offset, |v| (v & !DRV_MASK) | (DRV_20MA << DRV_SHIFT));
}

/// Configures every PSRAM pad for `mode`.
pub(crate) fn configure(is_hex: bool) {
    for offset in [
        PSRAM_D, PSRAM_Q, PSRAM_WP, PSRAM_HOLD, PSRAM_DQ4, PSRAM_DQ5, PSRAM_DQ6, PSRAM_DQ7,
        PSRAM_CK, PSRAM_CS,
    ] {
        set_drive_strength(offset);
    }

    // The DQS pads also need their output stage powered.
    modify(PSRAM_DQS0, |v| {
        (v & !DRV_MASK) | (DRV_20MA << DRV_SHIFT) | XPD
    });

    if is_hex {
        for offset in [PSRAM_DQ8, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64] {
            set_drive_strength(offset);
        }
        modify(PSRAM_DQS1, |v| {
            (v & !DRV_MASK) | (DRV_20MA << DRV_SHIFT) | XPD
        });
    }
}
