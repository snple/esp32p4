//! The CRC unit inside every GDMA channel.
//!
//! Both controllers can compute a CRC over everything a descriptor chain moves:
//! the transmit direction over the data it reads from memory, the receive
//! direction over the data it writes. The result covers one chain and is latched
//! when its last descriptor completes (TRM 6.4.12).
//!
//! The hardware is not wired to one polynomial. It runs an LFSR of
//! `width` bits and takes a **switch matrix** derived from the polynomial, eight
//! data bits per cycle. [`Crc`] builds that matrix from a polynomial with the
//! same construction the reference driver uses, and exposes the LFSR in software
//! through [`update`](Crc::update) so a driver can compare what the engine
//! reports against what the polynomial says it should.
//!
//! Widths run up to 32 bits on GDMA-AHB and 16 bits on GDMA-AXI; the channel
//! setters enforce that.
//!
//! # Bit Order
//!
//! The LFSR shifts towards its most significant bit and takes the polynomial as
//! written, which is the shape of the classic (non-reflected) CRCs: a byte
//! enters most significant bit first. [`Crc::reverse_data`] flips that to least
//! significant bit first, the order reflected polynomials are used with.
//! [`update`](Crc::update) follows whichever the configuration selects, so the
//! software and hardware results stay comparable.
//!
//! `Crc::new(32, 0x04c1_1db7, 0xffff_ffff)` therefore computes CRC-32/MPEG-2,
//! check value `0x0376_e6e7` over `"123456789"` — verified against the engine on
//! hardware.

/// A CRC the engine computes, and the same LFSR in software.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Crc {
    width: u8,
    polynomial: u32,
    init: u32,
    reverse_data: bool,
}

impl Crc {
    /// Widest CRC GDMA-AHB computes.
    pub const AHB_MAX_WIDTH: u8 = 32;
    /// Widest CRC GDMA-AXI computes.
    pub const AXI_MAX_WIDTH: u8 = 16;

    /// A CRC of `width` bits with `polynomial`, starting from `init`.
    ///
    /// `polynomial` is given without its `x^width` term, the way CRC catalogues
    /// write it: `crc[width-1:0] = poly`.
    ///
    /// # Panics
    ///
    /// Panics if `width` is outside 1..=32, or if `polynomial` has bits set
    /// beyond `width`.
    pub const fn new(width: u8, polynomial: u32, init: u32) -> Self {
        assert!(
            width >= 1 && width <= Self::AHB_MAX_WIDTH,
            "CRC width is 1-32 bits"
        );
        assert!(
            polynomial & !mask(width) == 0,
            "CRC polynomial has bits above its width"
        );
        assert!(
            init & !mask(width) == 0,
            "CRC init value has bits above its width"
        );
        Self {
            width,
            polynomial,
            init,
            reverse_data: false,
        }
    }

    /// Feeds each byte least significant bit first instead of most significant
    /// bit first.
    ///
    /// This is the input order reflected polynomials are written for. The LFSR
    /// itself keeps shifting towards its most significant bit either way, so a
    /// mirrored polynomial (`0xedb8_8320` for CRC-32) is not the same CRC as the
    /// one a reflected table-driven implementation computes.
    pub const fn reverse_data(mut self, reverse: bool) -> Self {
        self.reverse_data = reverse;
        self
    }

    /// CRC width in bits.
    #[inline]
    pub const fn width(&self) -> u8 {
        self.width
    }

    /// Polynomial, without its `x^width` term.
    #[inline]
    pub const fn polynomial(&self) -> u32 {
        self.polynomial
    }

    /// Value the LFSR starts from.
    #[inline]
    pub const fn init(&self) -> u32 {
        self.init
    }

    /// Whether the data bits are fed least significant bit first.
    #[inline]
    pub const fn reversed(&self) -> bool {
        self.reverse_data
    }

    /// Runs the LFSR over `data`, continuing from `crc`.
    ///
    /// Start from [`init`](Self::init); the result is what the engine reports for
    /// the same bytes in the same order.
    pub fn update(&self, crc: u32, data: &[u8]) -> u32 {
        let mut lfsr = crc & mask(self.width);
        for &byte in data {
            // `shift` consumes bit 0 first, so a byte that goes in most
            // significant bit first is handed over reversed.
            let data = if self.reverse_data {
                u32::from(byte)
            } else {
                u32::from(byte.reverse_bits())
            };
            lfsr = shift(lfsr, self.width, self.polynomial, data, 8);
        }
        lfsr
    }

    /// [`update`](Self::update) over a whole buffer, starting from `init`.
    pub fn compute(&self, data: &[u8]) -> u32 {
        self.update(self.init, data)
    }

    /// The switch matrix the engine needs, one mask pair per CRC bit.
    ///
    /// Calls `write(crc_bit, lfsr_mask, data_mask)` for bits `0..width`; the
    /// masks are what `*_CRC_EN_WR_DATA` and `*_CRC_DATA_EN_WR_DATA` take for
    /// that bit.
    pub(crate) fn for_each_mask(&self, mut write: impl FnMut(u32, u32, u32)) {
        let mut lfsr_matrix = [0u32; Self::AHB_MAX_WIDTH as usize];
        let mut data_matrix = [0u32; 8];
        for (bit, entry) in lfsr_matrix[..self.width as usize].iter_mut().enumerate() {
            *entry = shift(1 << bit, self.width, self.polynomial, 0, 8);
        }
        for bit in 0..8 {
            // The matrix is stored so that entry `j` belongs to data bit `j`;
            // the generator walks the source bits the other way round.
            data_matrix[7 - bit] = shift(0, self.width, self.polynomial, 1 << bit, 8);
        }

        for bit in 0..u32::from(self.width) {
            let mut lfsr_mask = 0;
            for source in 0..u32::from(self.width) {
                if lfsr_matrix[source as usize] & (1 << bit) != 0 {
                    lfsr_mask |= 1 << source;
                }
            }
            let mut data_mask = 0;
            for source in 0..8 {
                if data_matrix[source as usize] & (1 << bit) != 0 {
                    data_mask |= 1 << source;
                }
            }
            if self.reverse_data {
                data_mask = (data_mask as u8).reverse_bits() as u32;
            }
            write(bit, lfsr_mask, data_mask);
        }
    }
}

/// All bits below `width`.
const fn mask(width: u8) -> u32 {
    if width >= 32 {
        u32::MAX
    } else {
        (1 << width) - 1
    }
}

/// Shifts `bits` data bits into the LFSR one at a time.
///
/// This is the serial definition the switch matrix is derived from: the LFSR
/// shifts towards its most significant bit, and bit `j` of `data` enters at step
/// `j`.
fn shift(mut lfsr: u32, width: u8, polynomial: u32, data: u32, bits: u32) -> u32 {
    let mut step = 0;
    while step < bits {
        let top = (lfsr >> (width - 1)) & 1;
        let input = (data >> step) & 1;
        let mut bit = width;
        while bit > 1 {
            let index = bit - 1;
            let next = (lfsr >> (index - 1)) & 1;
            let value = if polynomial & (1 << index) != 0 {
                next ^ top ^ input
            } else {
                next
            };
            lfsr = (lfsr & !(1 << index)) | (value << index);
            bit -= 1;
        }
        lfsr = (lfsr & !1) | (top ^ input);
        step += 1;
    }
    lfsr
}
