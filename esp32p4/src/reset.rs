//! Peripheral resets in the HP domain (`HP_RST_EN0/1/2`).
//!
//! ESP32-P4 has no single `RESETS` block like the RP2350: the reset bits of the
//! HP peripherals live in the clock/reset controller `HP_SYS_CLKRST`, split over
//! three 32-bit registers. A bit **holds its peripheral in reset while it is
//! set**, so releasing a reset means clearing the bit. That is the opposite
//! polarity of RP2350's `RESET` register, but the same idea.
//!
//! There is no `RESET_DONE` handshake: the write lands in the always-on reset
//! controller, and the peripheral needs its bus clock to run before it can be
//! reached again. Drivers that release a reset right before touching the
//! peripheral therefore rely on the clock gate (see [`crate::startup`]) rather
//! than on a status bit.
//!
//! # Scope
//!
//! This module covers the HP peripherals. The LP/always-on block keeps its own
//! reset bits in `LP_AON_CLKRST` (`LP_RST_EN`, `HP_SDMMC_EMAC_RST_CTRL`), and
//! the HP cores are reset from the LP domain as well; those are
//! [`crate::system`]'s business.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::reset::Resettable;
//! use esp32p4_pac::{I2C0, I2C1};
//!
//! // Reset and release again: every register back at its hardware default.
//! I2C0::init();
//!
//! // Or hold the peripheral in reset and release it later.
//! I2C0::reset();
//! I2C0::unreset();
//!
//! // Several peripherals at once.
//! esp32p4::reset::reset(I2C0::RESET_MASK | I2C1::RESET_MASK);
//! ```

use core::ops::{BitOr, BitOrAssign};

use esp32p4_pac::{HP_SYS_CLKRST, hp_sys_clkrst};

/// The reset bits of one or more HP peripherals.
///
/// The three bitmaps mirror `HP_RST_EN0/1/2`, one bit per peripheral. Build a
/// mask from a [`Resettable::RESET_MASK`] and combine several with `|`; the
/// fields are private, so a mask can never name a bit that does not exist.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResetMask {
    en0: u32,
    en1: u32,
    en2: u32,
}

impl ResetMask {
    /// No reset bits at all.
    pub const NONE: Self = Self {
        en0: 0,
        en1: 0,
        en2: 0,
    };

    /// The bit of one register; `reg` is `0`, `1` or `2`.
    const fn bit(reg: u8, bit: u8) -> Self {
        let mask = 1u32 << bit;
        match reg {
            0 => Self {
                en0: mask,
                en1: 0,
                en2: 0,
            },
            1 => Self {
                en0: 0,
                en1: mask,
                en2: 0,
            },
            _ => Self {
                en0: 0,
                en1: 0,
                en2: mask,
            },
        }
    }

    /// The union of two masks, usable in constant expressions.
    pub const fn or(self, other: Self) -> Self {
        Self {
            en0: self.en0 | other.en0,
            en1: self.en1 | other.en1,
            en2: self.en2 | other.en2,
        }
    }
}

impl BitOr for ResetMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs)
    }
}

impl BitOrAssign for ResetMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.or(rhs);
    }
}

/// A peripheral that can be reset through [`HP_SYS_CLKRST`].
///
/// The methods are safe because a mask can only be built from the constants this
/// module provides, but they still have consequences: only reset a peripheral
/// whose driver your code owns, and stop it before holding it in reset.
pub trait Resettable {
    /// The peripheral's reset bits.
    const RESET_MASK: ResetMask;

    /// Reset the peripheral and release it again.
    ///
    /// Every register returns to its hardware default, and the peripheral is
    /// left out of reset. The clock gate is not touched: enable the clock before
    /// calling this, then configure the peripheral from scratch.
    fn init() {
        reset(Self::RESET_MASK);
        unreset(Self::RESET_MASK);
    }

    /// Hold the peripheral in reset.
    fn reset() {
        reset(Self::RESET_MASK);
    }

    /// Release the peripheral from reset.
    fn unreset() {
        unreset(Self::RESET_MASK);
    }
}

/// Hold every peripheral in `mask` in reset.
pub fn reset(mask: ResetMask) {
    let clkrst = clkrst();
    clkrst.hp_rst_en0.modify_bits(|bits| bits | mask.en0);
    clkrst.hp_rst_en1.modify_bits(|bits| bits | mask.en1);
    clkrst.hp_rst_en2.modify_bits(|bits| bits | mask.en2);
}

/// Release every peripheral in `mask` from reset.
pub fn unreset(mask: ResetMask) {
    let clkrst = clkrst();
    clkrst.hp_rst_en0.modify_bits(|bits| bits & !mask.en0);
    clkrst.hp_rst_en1.modify_bits(|bits| bits & !mask.en1);
    clkrst.hp_rst_en2.modify_bits(|bits| bits & !mask.en2);
}

/// The clock/reset controller's register block.
fn clkrst() -> &'static hp_sys_clkrst::RegisterBlock {
    // SAFETY: the clock/reset controller is always mapped and lives for the
    // program.
    unsafe { &*HP_SYS_CLKRST::PTR }
}

/// Implements [`Resettable`] for the PAC types whose reset bits the HAL knows.
///
/// `$reg` is the `HP_RST_EN` register (0, 1 or 2) and `$bit` the bit inside it.
/// Peripherals with more than one reset bit (a UART's APB and core clocks, for
/// example) list them all.
macro_rules! impl_resettable {
    ($($type:ty => $reg:literal : $bit:literal),+ $(,)?) => {
        $(
            impl Resettable for $type {
                const RESET_MASK: ResetMask = ResetMask::bit($reg, $bit);
            }
        )+
    };
}

impl_resettable!(
    // HP_RST_EN0
    esp32p4_pac::JPEG => 0:30,
    esp32p4_pac::ISP => 0:29,
    esp32p4_pac::MIPI_CSI_BRIDGE => 0:28,
    esp32p4_pac::MIPI_CSI_HOST => 0:27,
    esp32p4_pac::MIPI_DSI_BRIDGE => 0:26,
    esp32p4_pac::DMA => 0:21,
    esp32p4_pac::TRACE1 => 0:10,
    esp32p4_pac::TRACE0 => 0:9,
    // HP_RST_EN1
    esp32p4_pac::SOC_ETM => 1:31,
    esp32p4_pac::PCNT => 1:30,
    esp32p4_pac::LEDC => 1:29,
    esp32p4_pac::TWAI2 => 1:28,
    esp32p4_pac::TWAI1 => 1:27,
    esp32p4_pac::TWAI0 => 1:26,
    esp32p4_pac::MCPWM1 => 1:25,
    esp32p4_pac::MCPWM0 => 1:24,
    esp32p4_pac::RMT => 1:23,
    esp32p4_pac::I2C0 => 1:22,
    esp32p4_pac::I2C1 => 1:21,
    esp32p4_pac::I3C_SLV => 1:20,
    esp32p4_pac::I3C_MST => 1:19,
    esp32p4_pac::UHCI0 => 1:18,
    esp32p4_pac::TIMG1 => 1:7,
    esp32p4_pac::TIMG0 => 1:6,
    esp32p4_pac::SYSTIMER => 1:5,
    esp32p4_pac::IO_MUX => 1:3,
    esp32p4_pac::AXI_DMA => 1:2,
    esp32p4_pac::AHB_DMA => 1:1,
    esp32p4_pac::PPA => 1:0,
    // HP_RST_EN2
    esp32p4_pac::H264 => 2:24,
    esp32p4_pac::ECC => 2:22,
    esp32p4_pac::RSA => 2:21,
    esp32p4_pac::ECDSA => 2:20,
    esp32p4_pac::HMAC => 2:19,
    esp32p4_pac::SHA => 2:18,
    esp32p4_pac::DS => 2:17,
    esp32p4_pac::AES => 2:16,
    esp32p4_pac::ADC => 2:10,
    esp32p4_pac::LCD_CAM => 2:9,
    esp32p4_pac::SPI3 => 2:8,
    esp32p4_pac::SPI2 => 2:7,
    esp32p4_pac::I2S2 => 2:6,
    esp32p4_pac::I2S1 => 2:5,
    esp32p4_pac::I2S0 => 2:4,
);

/// Implements [`Resettable`] for the peripherals whose reset covers more than
/// one bit.
///
/// A UART has a reset for its APB interface and one for its core; the
/// bit-scrambler and the parallel IO have one per direction on top of the block
/// reset. Resetting the peripheral means holding all of them.
macro_rules! impl_resettable_multi {
    ($($type:ty => $reg:literal : [$($bit:literal),+]),+ $(,)?) => {
        $(
            impl Resettable for $type {
                const RESET_MASK: ResetMask = ResetMask::NONE$(
                    .or(ResetMask::bit($reg, $bit))
                )+;
            }
        )+
    };
}

impl_resettable_multi!(
    esp32p4_pac::UART4 => 1: [17, 12],
    esp32p4_pac::UART3 => 1: [16, 11],
    esp32p4_pac::UART2 => 1: [15, 10],
    esp32p4_pac::UART1 => 1: [14, 9],
    esp32p4_pac::UART0 => 1: [13, 8],
    esp32p4_pac::BITSCRAMBLER => 2: [13, 12, 11],
    esp32p4_pac::PARL_IO => 2: [3, 2, 1],
);
