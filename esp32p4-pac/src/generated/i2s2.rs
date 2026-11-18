//! I2S2
//! I2S (Inter-IC Sound) Controller 2

#![allow(dead_code)]

pub const BASE: u32 = 0x500c8000;

/// INT_RAW Register
///
/// I2S interrupt raw register, valid in level.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **TX_HUNG_INT_RAW** `[3]` (RO)
///
///   The raw interrupt status bit for the i2s_tx_hung_int interrupt
///
/// - **RX_HUNG_INT_RAW** `[2]` (RO)
///
///   The raw interrupt status bit for the i2s_rx_hung_int interrupt
///
/// - **TX_DONE_INT_RAW** `[1]` (RO)
///
///   The raw interrupt status bit for the i2s_tx_done_int interrupt
///
/// - **RX_DONE_INT_RAW** `[0]` (RO)
///
///   The raw interrupt status bit for the i2s_rx_done_int interrupt
#[repr(transparent)]
pub struct IntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntRaw {
    // ======== Field Constants ========

    /// **TX_HUNG_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_tx_hung_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_RAW_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_RAW_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_RAW: u32 = 1 << 3;

    /// **RX_HUNG_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_RAW_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_RAW_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_RAW: u32 = 1 << 2;

    /// **TX_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_tx_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DONE_INT_RAW_OFFSET: u32 = 1;
    pub const TX_DONE_INT_RAW_MASK: u32 = 0x00000002;
    pub const TX_DONE_INT_RAW: u32 = 1 << 1;

    /// **RX_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const RX_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_HUNG_INT_RAW** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_RAW_MASK) != 0
    }

    /// **RX_HUNG_INT_RAW** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_RAW_MASK) != 0
    }

    /// **TX_DONE_INT_RAW** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_RAW_MASK) != 0
    }

    /// **RX_DONE_INT_RAW** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn rx_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_DONE_INT_RAW_MASK) != 0
    }
}

/// INT_RAW value type for batch read/modify/write operations
///
/// Use `intraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntRawVal {
    bits: u32,
}

impl IntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_HUNG_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt status bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub const fn tx_hung_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_HUNG_INT_RAW_MASK) != 0
    }

    /// Read **RX_HUNG_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_HUNG_INT_RAW_MASK) != 0
    }

    /// Read **TX_DONE_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub const fn tx_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_DONE_INT_RAW_MASK) != 0
    }

    /// Read **RX_DONE_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn rx_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_DONE_INT_RAW_MASK) != 0
    }
}

impl Default for IntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntRawVal {
        IntRawVal::from_bits(self.value.read_bits())
    }
}

/// INT_ST Register
///
/// I2S interrupt status register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **TX_HUNG_INT_ST** `[3]` (RO)
///
///   The masked interrupt status bit for the i2s_tx_hung_int interrupt
///
/// - **RX_HUNG_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for the i2s_rx_hung_int interrupt
///
/// - **TX_DONE_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for the i2s_tx_done_int interrupt
///
/// - **RX_DONE_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for the i2s_rx_done_int interrupt
#[repr(transparent)]
pub struct IntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntSt {
    // ======== Field Constants ========

    /// **TX_HUNG_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_tx_hung_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_ST_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_ST_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_ST: u32 = 1 << 3;

    /// **RX_HUNG_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_ST_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_ST_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_ST: u32 = 1 << 2;

    /// **TX_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_tx_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DONE_INT_ST_OFFSET: u32 = 1;
    pub const TX_DONE_INT_ST_MASK: u32 = 0x00000002;
    pub const TX_DONE_INT_ST: u32 = 1 << 1;

    /// **RX_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_ST_OFFSET: u32 = 0;
    pub const RX_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_HUNG_INT_ST** `[3]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_ST_MASK) != 0
    }

    /// **RX_HUNG_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_ST_MASK) != 0
    }

    /// **TX_DONE_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_ST_MASK) != 0
    }

    /// **RX_DONE_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn rx_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_DONE_INT_ST_MASK) != 0
    }
}

/// INT_ST value type for batch read/modify/write operations
///
/// Use `intst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStVal {
    bits: u32,
}

impl IntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_HUNG_INT_ST** `[3:3]`
    ///
    /// The masked interrupt status bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub const fn tx_hung_int_st(&self) -> bool {
        (self.bits & IntSt::TX_HUNG_INT_ST_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_st(&self) -> bool {
        (self.bits & IntSt::RX_HUNG_INT_ST_MASK) != 0
    }

    /// Read **TX_DONE_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub const fn tx_done_int_st(&self) -> bool {
        (self.bits & IntSt::TX_DONE_INT_ST_MASK) != 0
    }

    /// Read **RX_DONE_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn rx_done_int_st(&self) -> bool {
        (self.bits & IntSt::RX_DONE_INT_ST_MASK) != 0
    }
}

impl Default for IntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStVal {
        IntStVal::from_bits(self.value.read_bits())
    }
}

/// INT_ENA Register
///
/// I2S interrupt enable register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **TX_HUNG_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the i2s_tx_hung_int interrupt
///
/// - **RX_HUNG_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the i2s_rx_hung_int interrupt
///
/// - **TX_DONE_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the i2s_tx_done_int interrupt
///
/// - **RX_DONE_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the i2s_rx_done_int interrupt
#[repr(transparent)]
pub struct IntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEna {
    // ======== Field Constants ========

    /// **TX_HUNG_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_tx_hung_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_ENA_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_ENA_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_ENA: u32 = 1 << 3;

    /// **RX_HUNG_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_ENA_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_ENA_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_ENA: u32 = 1 << 2;

    /// **TX_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_tx_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DONE_INT_ENA_OFFSET: u32 = 1;
    pub const TX_DONE_INT_ENA_MASK: u32 = 0x00000002;
    pub const TX_DONE_INT_ENA: u32 = 1 << 1;

    /// **RX_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const RX_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_HUNG_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_ENA_MASK) != 0
    }

    /// Set TX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_HUNG_INT_ENA_MASK);
    }

    /// Clear TX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_HUNG_INT_ENA_MASK);
    }

    /// **RX_HUNG_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_ENA_MASK) != 0
    }

    /// Set RX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::RX_HUNG_INT_ENA_MASK);
    }

    /// Clear RX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RX_HUNG_INT_ENA_MASK);
    }

    /// **TX_DONE_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_ENA_MASK) != 0
    }

    /// Set TX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_DONE_INT_ENA_MASK);
    }

    /// Clear TX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_DONE_INT_ENA_MASK);
    }

    /// **RX_DONE_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn rx_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_DONE_INT_ENA_MASK) != 0
    }

    /// Set RX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::RX_DONE_INT_ENA_MASK);
    }

    /// Clear RX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RX_DONE_INT_ENA_MASK);
    }
}

/// INT_ENA value type for batch read/modify/write operations
///
/// Use `intena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaVal {
    bits: u32,
}

impl IntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_HUNG_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub const fn tx_hung_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_HUNG_INT_ENA_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_HUNG_INT_ENA_MASK) != 0
    }

    /// Read **TX_DONE_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub const fn tx_done_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_DONE_INT_ENA_MASK) != 0
    }

    /// Read **RX_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn rx_done_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_DONE_INT_ENA_MASK) != 0
    }

    /// Set **TX_HUNG_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub const fn with_tx_hung_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_HUNG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_HUNG_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn with_rx_hung_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_HUNG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_HUNG_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_DONE_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub const fn with_tx_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn with_rx_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_DONE_INT_ENA_MASK
            },
        }
    }
}

impl Default for IntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntEnaVal {
        IntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntEnaVal) -> IntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_CLR Register
///
/// I2S interrupt clear register.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **TX_HUNG_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the i2s_tx_hung_int interrupt
///
/// - **RX_HUNG_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the i2s_rx_hung_int interrupt
///
/// - **TX_DONE_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the i2s_tx_done_int interrupt
///
/// - **RX_DONE_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the i2s_rx_done_int interrupt
#[repr(transparent)]
pub struct IntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntClr {
    // ======== Field Constants ========

    /// **TX_HUNG_INT_CLR**
    ///
    /// Set this bit to clear the i2s_tx_hung_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_CLR_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_CLR_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_CLR: u32 = 1 << 3;

    /// **RX_HUNG_INT_CLR**
    ///
    /// Set this bit to clear the i2s_rx_hung_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_CLR_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_CLR_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_CLR: u32 = 1 << 2;

    /// **TX_DONE_INT_CLR**
    ///
    /// Set this bit to clear the i2s_tx_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DONE_INT_CLR_OFFSET: u32 = 1;
    pub const TX_DONE_INT_CLR_MASK: u32 = 0x00000002;
    pub const TX_DONE_INT_CLR: u32 = 1 << 1;

    /// **RX_DONE_INT_CLR**
    ///
    /// Set this bit to clear the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const RX_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TX_HUNG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn trigger_tx_hung_int_clr(&self) {
        self.value.write_bits(Self::TX_HUNG_INT_CLR_MASK);
    }

    /// Trigger **RX_HUNG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn trigger_rx_hung_int_clr(&self) {
        self.value.write_bits(Self::RX_HUNG_INT_CLR_MASK);
    }

    /// Trigger **TX_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn trigger_tx_done_int_clr(&self) {
        self.value.write_bits(Self::TX_DONE_INT_CLR_MASK);
    }

    /// Trigger **RX_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn trigger_rx_done_int_clr(&self) {
        self.value.write_bits(Self::RX_DONE_INT_CLR_MASK);
    }
}

/// INT_CLR value type for batch read/modify/write operations
///
/// Use `intclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntClrVal {
    bits: u32,
}

impl IntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TX_HUNG_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub const fn with_tx_hung_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_HUNG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_HUNG_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn with_rx_hung_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_HUNG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_HUNG_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_DONE_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the i2s_tx_done_int interrupt
    #[inline(always)]
    pub const fn with_tx_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_DONE_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn with_rx_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_DONE_INT_CLR_MASK
            },
        }
    }
}

impl Default for IntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// RX_CONF Register
///
/// I2S RX configure register
///
/// Address offset: 0x0020
/// Reset value: 0x00c0b600
///
/// ## Fields
///
/// - **RX_BCK_DIV_NUM** `[26:21]` (RW)
///
///   Bit clock configuration bits in receiver mode.
///
/// - **RX_PDM_EN** `[20]` (RW)
///
///   1: Enable I2S PDM Rx mode . 0: Disable.
///
/// - **RX_TDM_EN** `[19]` (RW)
///
///   1: Enable I2S TDM Rx mode . 0: Disable.
///
/// - **RX_BIT_ORDER** `[18]` (RW)
///
///   I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first.
///
/// - **RX_WS_IDLE_POL** `[17]` (RW)
///
///   0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel.
///
/// - **RX_24_FILL_EN** `[16]` (RW)
///
///   1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits.
///
/// - **RX_LEFT_ALIGN** `[15]` (RW)
///
///   1: I2S RX left alignment mode. 0: I2S RX right alignment mode.
///
/// - **RX_MSB_SHIFT** `[13]` (RW)
///
///   Set this bit to enable receiver in Phillips standard mode
///
/// - **RX_PCM_BYPASS** `[12]` (RW)
///
///   Set this bit to bypass Compress/Decompress module for received data.
///
/// - **RX_PCM_CONF** `[11:10]` (RW)
///
///   I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
///
/// - **RX_MONO_FST_VLD** `[9]` (RW)
///
///   1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode.
///
/// - **RX_UPDATE** `[8]` (RW)
///
///   Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done.
///
/// - **RX_BIG_ENDIAN** `[7]` (RW)
///
///   I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
///
/// - **RX_MONO** `[6]` (RW)
///
///   Set this bit to enable receiver in mono mode
///
/// - **RX_STOP_MODE** `[5:4]` (RW)
///
///   0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
///
/// - **RX_SLAVE_MOD** `[3]` (RW)
///
///   Set this bit to enable slave receiver mode
///
/// - **RX_START** `[2]` (RW)
///
///   Set this bit to start receiving data
///
/// - **RX_FIFO_RESET** `[1]` (WO)
///
///   Set this bit to reset Rx AFIFO
///
/// - **RX_RESET** `[0]` (WO)
///
///   Set this bit to reset receiver
#[repr(transparent)]
pub struct RxConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxConf {
    // ======== Field Constants ========

    /// **RX_BCK_DIV_NUM**
    ///
    /// Bit clock configuration bits in receiver mode.
    ///
    /// Bits `[26:21]`, mask `0x07e00000`
    pub const RX_BCK_DIV_NUM_OFFSET: u32 = 21;
    pub const RX_BCK_DIV_NUM_MASK: u32 = 0x07e00000;

    /// **RX_PDM_EN**
    ///
    /// 1: Enable I2S PDM Rx mode . 0: Disable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_PDM_EN_OFFSET: u32 = 20;
    pub const RX_PDM_EN_MASK: u32 = 0x00100000;
    pub const RX_PDM_EN: u32 = 1 << 20;

    /// **RX_TDM_EN**
    ///
    /// 1: Enable I2S TDM Rx mode . 0: Disable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const RX_TDM_EN_OFFSET: u32 = 19;
    pub const RX_TDM_EN_MASK: u32 = 0x00080000;
    pub const RX_TDM_EN: u32 = 1 << 19;

    /// **RX_BIT_ORDER**
    ///
    /// I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const RX_BIT_ORDER_OFFSET: u32 = 18;
    pub const RX_BIT_ORDER_MASK: u32 = 0x00040000;
    pub const RX_BIT_ORDER: u32 = 1 << 18;

    /// **RX_WS_IDLE_POL**
    ///
    /// 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const RX_WS_IDLE_POL_OFFSET: u32 = 17;
    pub const RX_WS_IDLE_POL_MASK: u32 = 0x00020000;
    pub const RX_WS_IDLE_POL: u32 = 1 << 17;

    /// **RX_24_FILL_EN**
    ///
    /// 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const RX_24_FILL_EN_OFFSET: u32 = 16;
    pub const RX_24_FILL_EN_MASK: u32 = 0x00010000;
    pub const RX_24_FILL_EN: u32 = 1 << 16;

    /// **RX_LEFT_ALIGN**
    ///
    /// 1: I2S RX left alignment mode. 0: I2S RX right alignment mode.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RX_LEFT_ALIGN_OFFSET: u32 = 15;
    pub const RX_LEFT_ALIGN_MASK: u32 = 0x00008000;
    pub const RX_LEFT_ALIGN: u32 = 1 << 15;

    /// **RX_MSB_SHIFT**
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const RX_MSB_SHIFT_OFFSET: u32 = 13;
    pub const RX_MSB_SHIFT_MASK: u32 = 0x00002000;
    pub const RX_MSB_SHIFT: u32 = 1 << 13;

    /// **RX_PCM_BYPASS**
    ///
    /// Set this bit to bypass Compress/Decompress module for received data.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_PCM_BYPASS_OFFSET: u32 = 12;
    pub const RX_PCM_BYPASS_MASK: u32 = 0x00001000;
    pub const RX_PCM_BYPASS: u32 = 1 << 12;

    /// **RX_PCM_CONF**
    ///
    /// I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const RX_PCM_CONF_OFFSET: u32 = 10;
    pub const RX_PCM_CONF_MASK: u32 = 0x00000c00;

    /// **RX_MONO_FST_VLD**
    ///
    /// 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RX_MONO_FST_VLD_OFFSET: u32 = 9;
    pub const RX_MONO_FST_VLD_MASK: u32 = 0x00000200;
    pub const RX_MONO_FST_VLD: u32 = 1 << 9;

    /// **RX_UPDATE**
    ///
    /// Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RX_UPDATE_OFFSET: u32 = 8;
    pub const RX_UPDATE_MASK: u32 = 0x00000100;
    pub const RX_UPDATE: u32 = 1 << 8;

    /// **RX_BIG_ENDIAN**
    ///
    /// I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_BIG_ENDIAN_OFFSET: u32 = 7;
    pub const RX_BIG_ENDIAN_MASK: u32 = 0x00000080;
    pub const RX_BIG_ENDIAN: u32 = 1 << 7;

    /// **RX_MONO**
    ///
    /// Set this bit to enable receiver in mono mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_MONO_OFFSET: u32 = 6;
    pub const RX_MONO_MASK: u32 = 0x00000040;
    pub const RX_MONO: u32 = 1 << 6;

    /// **RX_STOP_MODE**
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const RX_STOP_MODE_OFFSET: u32 = 4;
    pub const RX_STOP_MODE_MASK: u32 = 0x00000030;

    /// **RX_SLAVE_MOD**
    ///
    /// Set this bit to enable slave receiver mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_SLAVE_MOD_OFFSET: u32 = 3;
    pub const RX_SLAVE_MOD_MASK: u32 = 0x00000008;
    pub const RX_SLAVE_MOD: u32 = 1 << 3;

    /// **RX_START**
    ///
    /// Set this bit to start receiving data
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_START_OFFSET: u32 = 2;
    pub const RX_START_MASK: u32 = 0x00000004;
    pub const RX_START: u32 = 1 << 2;

    /// **RX_FIFO_RESET**
    ///
    /// Set this bit to reset Rx AFIFO
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_FIFO_RESET_OFFSET: u32 = 1;
    pub const RX_FIFO_RESET_MASK: u32 = 0x00000002;
    pub const RX_FIFO_RESET: u32 = 1 << 1;

    /// **RX_RESET**
    ///
    /// Set this bit to reset receiver
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_RESET_OFFSET: u32 = 0;
    pub const RX_RESET_MASK: u32 = 0x00000001;
    pub const RX_RESET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_BCK_DIV_NUM** `[26:21]` (RW)
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BCK_DIV_NUM_MASK) >> Self::RX_BCK_DIV_NUM_OFFSET
    }

    /// Write **RX_BCK_DIV_NUM** `[26:21]` (RW)
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub fn write_rx_bck_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BCK_DIV_NUM_MASK)
                | ((value << Self::RX_BCK_DIV_NUM_OFFSET) & Self::RX_BCK_DIV_NUM_MASK)
        });
    }

    /// **RX_PDM_EN** `[20]` (RW)
    ///
    /// 1: Enable I2S PDM Rx mode . 0: Disable.
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_PDM_EN_MASK) != 0
    }

    /// Set RX_PDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pdm_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_PDM_EN_MASK);
    }

    /// Clear RX_PDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pdm_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_PDM_EN_MASK);
    }

    /// **RX_TDM_EN** `[19]` (RW)
    ///
    /// 1: Enable I2S TDM Rx mode . 0: Disable.
    #[inline(always)]
    pub fn rx_tdm_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_EN_MASK) != 0
    }

    /// Set RX_TDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_EN_MASK);
    }

    /// Clear RX_TDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_EN_MASK);
    }

    /// **RX_BIT_ORDER** `[18]` (RW)
    ///
    /// I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first.
    #[inline(always)]
    pub fn rx_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::RX_BIT_ORDER_MASK) != 0
    }

    /// Set RX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_bit_order(&self) {
        self.value.modify_bits(|r| r | Self::RX_BIT_ORDER_MASK);
    }

    /// Clear RX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_bit_order(&self) {
        self.value.modify_bits(|r| r & !Self::RX_BIT_ORDER_MASK);
    }

    /// **RX_WS_IDLE_POL** `[17]` (RW)
    ///
    /// 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub fn rx_ws_idle_pol(&self) -> bool {
        (self.value.read_bits() & Self::RX_WS_IDLE_POL_MASK) != 0
    }

    /// Set RX_WS_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ws_idle_pol(&self) {
        self.value.modify_bits(|r| r | Self::RX_WS_IDLE_POL_MASK);
    }

    /// Clear RX_WS_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ws_idle_pol(&self) {
        self.value.modify_bits(|r| r & !Self::RX_WS_IDLE_POL_MASK);
    }

    /// **RX_24_FILL_EN** `[16]` (RW)
    ///
    /// 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits.
    #[inline(always)]
    pub fn rx_24_fill_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_24_FILL_EN_MASK) != 0
    }

    /// Set RX_24_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_24_fill_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_24_FILL_EN_MASK);
    }

    /// Clear RX_24_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_24_fill_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_24_FILL_EN_MASK);
    }

    /// **RX_LEFT_ALIGN** `[15]` (RW)
    ///
    /// 1: I2S RX left alignment mode. 0: I2S RX right alignment mode.
    #[inline(always)]
    pub fn rx_left_align(&self) -> bool {
        (self.value.read_bits() & Self::RX_LEFT_ALIGN_MASK) != 0
    }

    /// Set RX_LEFT_ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_left_align(&self) {
        self.value.modify_bits(|r| r | Self::RX_LEFT_ALIGN_MASK);
    }

    /// Clear RX_LEFT_ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_left_align(&self) {
        self.value.modify_bits(|r| r & !Self::RX_LEFT_ALIGN_MASK);
    }

    /// **RX_MSB_SHIFT** `[13]` (RW)
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> bool {
        (self.value.read_bits() & Self::RX_MSB_SHIFT_MASK) != 0
    }

    /// Set RX_MSB_SHIFT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_msb_shift(&self) {
        self.value.modify_bits(|r| r | Self::RX_MSB_SHIFT_MASK);
    }

    /// Clear RX_MSB_SHIFT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_msb_shift(&self) {
        self.value.modify_bits(|r| r & !Self::RX_MSB_SHIFT_MASK);
    }

    /// **RX_PCM_BYPASS** `[12]` (RW)
    ///
    /// Set this bit to bypass Compress/Decompress module for received data.
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> bool {
        (self.value.read_bits() & Self::RX_PCM_BYPASS_MASK) != 0
    }

    /// Set RX_PCM_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pcm_bypass(&self) {
        self.value.modify_bits(|r| r | Self::RX_PCM_BYPASS_MASK);
    }

    /// Clear RX_PCM_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pcm_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::RX_PCM_BYPASS_MASK);
    }

    /// **RX_PCM_CONF** `[11:10]` (RW)
    ///
    /// I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PCM_CONF_MASK) >> Self::RX_PCM_CONF_OFFSET
    }

    /// Write **RX_PCM_CONF** `[11:10]` (RW)
    ///
    /// I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub fn write_rx_pcm_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PCM_CONF_MASK)
                | ((value << Self::RX_PCM_CONF_OFFSET) & Self::RX_PCM_CONF_MASK)
        });
    }

    /// **RX_MONO_FST_VLD** `[9]` (RW)
    ///
    /// 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode.
    #[inline(always)]
    pub fn rx_mono_fst_vld(&self) -> bool {
        (self.value.read_bits() & Self::RX_MONO_FST_VLD_MASK) != 0
    }

    /// Set RX_MONO_FST_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_mono_fst_vld(&self) {
        self.value.modify_bits(|r| r | Self::RX_MONO_FST_VLD_MASK);
    }

    /// Clear RX_MONO_FST_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_mono_fst_vld(&self) {
        self.value.modify_bits(|r| r & !Self::RX_MONO_FST_VLD_MASK);
    }

    /// **RX_UPDATE** `[8]` (RW)
    ///
    /// Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub fn rx_update(&self) -> bool {
        (self.value.read_bits() & Self::RX_UPDATE_MASK) != 0
    }

    /// Set RX_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_update(&self) {
        self.value.modify_bits(|r| r | Self::RX_UPDATE_MASK);
    }

    /// Clear RX_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_update(&self) {
        self.value.modify_bits(|r| r & !Self::RX_UPDATE_MASK);
    }

    /// **RX_BIG_ENDIAN** `[7]` (RW)
    ///
    /// I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub fn rx_big_endian(&self) -> bool {
        (self.value.read_bits() & Self::RX_BIG_ENDIAN_MASK) != 0
    }

    /// Set RX_BIG_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_big_endian(&self) {
        self.value.modify_bits(|r| r | Self::RX_BIG_ENDIAN_MASK);
    }

    /// Clear RX_BIG_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_big_endian(&self) {
        self.value.modify_bits(|r| r & !Self::RX_BIG_ENDIAN_MASK);
    }

    /// **RX_MONO** `[6]` (RW)
    ///
    /// Set this bit to enable receiver in mono mode
    #[inline(always)]
    pub fn rx_mono(&self) -> bool {
        (self.value.read_bits() & Self::RX_MONO_MASK) != 0
    }

    /// Set RX_MONO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_mono(&self) {
        self.value.modify_bits(|r| r | Self::RX_MONO_MASK);
    }

    /// Clear RX_MONO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_mono(&self) {
        self.value.modify_bits(|r| r & !Self::RX_MONO_MASK);
    }

    /// **RX_STOP_MODE** `[5:4]` (RW)
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> u32 {
        (self.value.read_bits() & Self::RX_STOP_MODE_MASK) >> Self::RX_STOP_MODE_OFFSET
    }

    /// Write **RX_STOP_MODE** `[5:4]` (RW)
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub fn write_rx_stop_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_STOP_MODE_MASK)
                | ((value << Self::RX_STOP_MODE_OFFSET) & Self::RX_STOP_MODE_MASK)
        });
    }

    /// **RX_SLAVE_MOD** `[3]` (RW)
    ///
    /// Set this bit to enable slave receiver mode
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> bool {
        (self.value.read_bits() & Self::RX_SLAVE_MOD_MASK) != 0
    }

    /// Set RX_SLAVE_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_slave_mod(&self) {
        self.value.modify_bits(|r| r | Self::RX_SLAVE_MOD_MASK);
    }

    /// Clear RX_SLAVE_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_slave_mod(&self) {
        self.value.modify_bits(|r| r & !Self::RX_SLAVE_MOD_MASK);
    }

    /// **RX_START** `[2]` (RW)
    ///
    /// Set this bit to start receiving data
    #[inline(always)]
    pub fn rx_start(&self) -> bool {
        (self.value.read_bits() & Self::RX_START_MASK) != 0
    }

    /// Set RX_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_start(&self) {
        self.value.modify_bits(|r| r | Self::RX_START_MASK);
    }

    /// Clear RX_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_start(&self) {
        self.value.modify_bits(|r| r & !Self::RX_START_MASK);
    }

    /// Trigger **RX_FIFO_RESET** (WO)
    ///
    /// Set this bit to reset Rx AFIFO
    #[inline(always)]
    pub fn trigger_rx_fifo_reset(&self) {
        self.value.modify_bits(|r| r | Self::RX_FIFO_RESET_MASK);
    }

    /// Trigger **RX_RESET** (WO)
    ///
    /// Set this bit to reset receiver
    #[inline(always)]
    pub fn trigger_rx_reset(&self) {
        self.value.modify_bits(|r| r | Self::RX_RESET_MASK);
    }
}

/// RX_CONF value type for batch read/modify/write operations
///
/// Use `rxconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxConfVal {
    bits: u32,
}

impl RxConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00c0b600 }
    }

    /// Read **RX_BCK_DIV_NUM** `[26:21]`
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub const fn rx_bck_div_num(&self) -> u32 {
        (self.bits & RxConf::RX_BCK_DIV_NUM_MASK) >> RxConf::RX_BCK_DIV_NUM_OFFSET
    }

    /// Read **RX_PDM_EN** `[20:20]`
    ///
    /// 1: Enable I2S PDM Rx mode . 0: Disable.
    #[inline(always)]
    pub const fn rx_pdm_en(&self) -> bool {
        (self.bits & RxConf::RX_PDM_EN_MASK) != 0
    }

    /// Read **RX_TDM_EN** `[19:19]`
    ///
    /// 1: Enable I2S TDM Rx mode . 0: Disable.
    #[inline(always)]
    pub const fn rx_tdm_en(&self) -> bool {
        (self.bits & RxConf::RX_TDM_EN_MASK) != 0
    }

    /// Read **RX_BIT_ORDER** `[18:18]`
    ///
    /// I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first.
    #[inline(always)]
    pub const fn rx_bit_order(&self) -> bool {
        (self.bits & RxConf::RX_BIT_ORDER_MASK) != 0
    }

    /// Read **RX_WS_IDLE_POL** `[17:17]`
    ///
    /// 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub const fn rx_ws_idle_pol(&self) -> bool {
        (self.bits & RxConf::RX_WS_IDLE_POL_MASK) != 0
    }

    /// Read **RX_24_FILL_EN** `[16:16]`
    ///
    /// 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits.
    #[inline(always)]
    pub const fn rx_24_fill_en(&self) -> bool {
        (self.bits & RxConf::RX_24_FILL_EN_MASK) != 0
    }

    /// Read **RX_LEFT_ALIGN** `[15:15]`
    ///
    /// 1: I2S RX left alignment mode. 0: I2S RX right alignment mode.
    #[inline(always)]
    pub const fn rx_left_align(&self) -> bool {
        (self.bits & RxConf::RX_LEFT_ALIGN_MASK) != 0
    }

    /// Read **RX_MSB_SHIFT** `[13:13]`
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    #[inline(always)]
    pub const fn rx_msb_shift(&self) -> bool {
        (self.bits & RxConf::RX_MSB_SHIFT_MASK) != 0
    }

    /// Read **RX_PCM_BYPASS** `[12:12]`
    ///
    /// Set this bit to bypass Compress/Decompress module for received data.
    #[inline(always)]
    pub const fn rx_pcm_bypass(&self) -> bool {
        (self.bits & RxConf::RX_PCM_BYPASS_MASK) != 0
    }

    /// Read **RX_PCM_CONF** `[11:10]`
    ///
    /// I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub const fn rx_pcm_conf(&self) -> u32 {
        (self.bits & RxConf::RX_PCM_CONF_MASK) >> RxConf::RX_PCM_CONF_OFFSET
    }

    /// Read **RX_MONO_FST_VLD** `[9:9]`
    ///
    /// 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode.
    #[inline(always)]
    pub const fn rx_mono_fst_vld(&self) -> bool {
        (self.bits & RxConf::RX_MONO_FST_VLD_MASK) != 0
    }

    /// Read **RX_UPDATE** `[8:8]`
    ///
    /// Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub const fn rx_update(&self) -> bool {
        (self.bits & RxConf::RX_UPDATE_MASK) != 0
    }

    /// Read **RX_BIG_ENDIAN** `[7:7]`
    ///
    /// I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub const fn rx_big_endian(&self) -> bool {
        (self.bits & RxConf::RX_BIG_ENDIAN_MASK) != 0
    }

    /// Read **RX_MONO** `[6:6]`
    ///
    /// Set this bit to enable receiver in mono mode
    #[inline(always)]
    pub const fn rx_mono(&self) -> bool {
        (self.bits & RxConf::RX_MONO_MASK) != 0
    }

    /// Read **RX_STOP_MODE** `[5:4]`
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub const fn rx_stop_mode(&self) -> u32 {
        (self.bits & RxConf::RX_STOP_MODE_MASK) >> RxConf::RX_STOP_MODE_OFFSET
    }

    /// Read **RX_SLAVE_MOD** `[3:3]`
    ///
    /// Set this bit to enable slave receiver mode
    #[inline(always)]
    pub const fn rx_slave_mod(&self) -> bool {
        (self.bits & RxConf::RX_SLAVE_MOD_MASK) != 0
    }

    /// Read **RX_START** `[2:2]`
    ///
    /// Set this bit to start receiving data
    #[inline(always)]
    pub const fn rx_start(&self) -> bool {
        (self.bits & RxConf::RX_START_MASK) != 0
    }

    /// Set **RX_BCK_DIV_NUM** `[26:21]`
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub const fn with_rx_bck_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf::RX_BCK_DIV_NUM_MASK)
                | ((value << RxConf::RX_BCK_DIV_NUM_OFFSET) & RxConf::RX_BCK_DIV_NUM_MASK),
        }
    }

    /// Set **RX_PDM_EN** `[20:20]`
    ///
    /// 1: Enable I2S PDM Rx mode . 0: Disable.
    #[inline(always)]
    pub const fn with_rx_pdm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_PDM_EN_MASK
            } else {
                self.bits & !RxConf::RX_PDM_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_EN** `[19:19]`
    ///
    /// 1: Enable I2S TDM Rx mode . 0: Disable.
    #[inline(always)]
    pub const fn with_rx_tdm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_TDM_EN_MASK
            } else {
                self.bits & !RxConf::RX_TDM_EN_MASK
            },
        }
    }

    /// Set **RX_BIT_ORDER** `[18:18]`
    ///
    /// I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first.
    #[inline(always)]
    pub const fn with_rx_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_BIT_ORDER_MASK
            } else {
                self.bits & !RxConf::RX_BIT_ORDER_MASK
            },
        }
    }

    /// Set **RX_WS_IDLE_POL** `[17:17]`
    ///
    /// 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub const fn with_rx_ws_idle_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_WS_IDLE_POL_MASK
            } else {
                self.bits & !RxConf::RX_WS_IDLE_POL_MASK
            },
        }
    }

    /// Set **RX_24_FILL_EN** `[16:16]`
    ///
    /// 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits.
    #[inline(always)]
    pub const fn with_rx_24_fill_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_24_FILL_EN_MASK
            } else {
                self.bits & !RxConf::RX_24_FILL_EN_MASK
            },
        }
    }

    /// Set **RX_LEFT_ALIGN** `[15:15]`
    ///
    /// 1: I2S RX left alignment mode. 0: I2S RX right alignment mode.
    #[inline(always)]
    pub const fn with_rx_left_align(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_LEFT_ALIGN_MASK
            } else {
                self.bits & !RxConf::RX_LEFT_ALIGN_MASK
            },
        }
    }

    /// Set **RX_MSB_SHIFT** `[13:13]`
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    #[inline(always)]
    pub const fn with_rx_msb_shift(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_MSB_SHIFT_MASK
            } else {
                self.bits & !RxConf::RX_MSB_SHIFT_MASK
            },
        }
    }

    /// Set **RX_PCM_BYPASS** `[12:12]`
    ///
    /// Set this bit to bypass Compress/Decompress module for received data.
    #[inline(always)]
    pub const fn with_rx_pcm_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_PCM_BYPASS_MASK
            } else {
                self.bits & !RxConf::RX_PCM_BYPASS_MASK
            },
        }
    }

    /// Set **RX_PCM_CONF** `[11:10]`
    ///
    /// I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub const fn with_rx_pcm_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf::RX_PCM_CONF_MASK)
                | ((value << RxConf::RX_PCM_CONF_OFFSET) & RxConf::RX_PCM_CONF_MASK),
        }
    }

    /// Set **RX_MONO_FST_VLD** `[9:9]`
    ///
    /// 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode.
    #[inline(always)]
    pub const fn with_rx_mono_fst_vld(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_MONO_FST_VLD_MASK
            } else {
                self.bits & !RxConf::RX_MONO_FST_VLD_MASK
            },
        }
    }

    /// Set **RX_UPDATE** `[8:8]`
    ///
    /// Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub const fn with_rx_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_UPDATE_MASK
            } else {
                self.bits & !RxConf::RX_UPDATE_MASK
            },
        }
    }

    /// Set **RX_BIG_ENDIAN** `[7:7]`
    ///
    /// I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub const fn with_rx_big_endian(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_BIG_ENDIAN_MASK
            } else {
                self.bits & !RxConf::RX_BIG_ENDIAN_MASK
            },
        }
    }

    /// Set **RX_MONO** `[6:6]`
    ///
    /// Set this bit to enable receiver in mono mode
    #[inline(always)]
    pub const fn with_rx_mono(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_MONO_MASK
            } else {
                self.bits & !RxConf::RX_MONO_MASK
            },
        }
    }

    /// Set **RX_STOP_MODE** `[5:4]`
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub const fn with_rx_stop_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf::RX_STOP_MODE_MASK)
                | ((value << RxConf::RX_STOP_MODE_OFFSET) & RxConf::RX_STOP_MODE_MASK),
        }
    }

    /// Set **RX_SLAVE_MOD** `[3:3]`
    ///
    /// Set this bit to enable slave receiver mode
    #[inline(always)]
    pub const fn with_rx_slave_mod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_SLAVE_MOD_MASK
            } else {
                self.bits & !RxConf::RX_SLAVE_MOD_MASK
            },
        }
    }

    /// Set **RX_START** `[2:2]`
    ///
    /// Set this bit to start receiving data
    #[inline(always)]
    pub const fn with_rx_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_START_MASK
            } else {
                self.bits & !RxConf::RX_START_MASK
            },
        }
    }

    /// Set **RX_FIFO_RESET** `[1:1]`
    ///
    /// Set this bit to reset Rx AFIFO
    #[inline(always)]
    pub const fn with_rx_fifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_FIFO_RESET_MASK
            } else {
                self.bits & !RxConf::RX_FIFO_RESET_MASK
            },
        }
    }

    /// Set **RX_RESET** `[0:0]`
    ///
    /// Set this bit to reset receiver
    #[inline(always)]
    pub const fn with_rx_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_RESET_MASK
            } else {
                self.bits & !RxConf::RX_RESET_MASK
            },
        }
    }
}

impl Default for RxConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxConfVal {
        RxConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxConfVal) -> RxConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CONF Register
///
/// I2S TX configure register
///
/// Address offset: 0x0024
/// Reset value: 0x00c0f210
///
/// ## Fields
///
/// - **SIG_LOOPBACK** `[30]` (RW)
///
///   Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals.
///
/// - **TX_CHAN_MOD** `[29:27]` (RW)
///
///   I2S transmitter channel mode configuration bits.
///
/// - **TX_BCK_DIV_NUM** `[26:21]` (RW)
///
///   Bit clock configuration bits in transmitter mode.
///
/// - **TX_PDM_EN** `[20]` (RW)
///
///   1: Enable I2S PDM Tx mode . 0: Disable.
///
/// - **TX_TDM_EN** `[19]` (RW)
///
///   1: Enable I2S TDM Tx mode . 0: Disable.
///
/// - **TX_BIT_ORDER** `[18]` (RW)
///
///   I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first.
///
/// - **TX_WS_IDLE_POL** `[17]` (RW)
///
///   0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel.
///
/// - **TX_24_FILL_EN** `[16]` (RW)
///
///   1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode
///
/// - **TX_LEFT_ALIGN** `[15]` (RW)
///
///   1: I2S TX left alignment mode. 0: I2S TX right alignment mode.
///
/// - **TX_BCK_NO_DLY** `[14]` (RW)
///
///   1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode.
///
/// - **TX_MSB_SHIFT** `[13]` (RW)
///
///   Set this bit to enable transmitter in Phillips standard mode
///
/// - **TX_PCM_BYPASS** `[12]` (RW)
///
///   Set this bit to bypass Compress/Decompress module for transmitted data.
///
/// - **TX_PCM_CONF** `[11:10]` (RW)
///
///   I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
///
/// - **TX_MONO_FST_VLD** `[9]` (RW)
///
///   1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode.
///
/// - **TX_UPDATE** `[8]` (RW)
///
///   Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done.
///
/// - **TX_BIG_ENDIAN** `[7]` (RW)
///
///   I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
///
/// - **TX_MONO** `[6]` (RW)
///
///   Set this bit to enable transmitter in mono mode
///
/// - **TX_CHAN_EQUAL** `[5]` (RW)
///
///   1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode.
///
/// - **TX_STOP_EN** `[4]` (RW)
///
///   Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy
///
/// - **TX_SLAVE_MOD** `[3]` (RW)
///
///   Set this bit to enable slave transmitter mode
///
/// - **TX_START** `[2]` (RW)
///
///   Set this bit to start transmitting data
///
/// - **TX_FIFO_RESET** `[1]` (WO)
///
///   Set this bit to reset Tx AFIFO
///
/// - **TX_RESET** `[0]` (WO)
///
///   Set this bit to reset transmitter
#[repr(transparent)]
pub struct TxConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxConf {
    // ======== Field Constants ========

    /// **SIG_LOOPBACK**
    ///
    /// Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SIG_LOOPBACK_OFFSET: u32 = 30;
    pub const SIG_LOOPBACK_MASK: u32 = 0x40000000;
    pub const SIG_LOOPBACK: u32 = 1 << 30;

    /// **TX_CHAN_MOD**
    ///
    /// I2S transmitter channel mode configuration bits.
    ///
    /// Bits `[29:27]`, mask `0x38000000`
    pub const TX_CHAN_MOD_OFFSET: u32 = 27;
    pub const TX_CHAN_MOD_MASK: u32 = 0x38000000;

    /// **TX_BCK_DIV_NUM**
    ///
    /// Bit clock configuration bits in transmitter mode.
    ///
    /// Bits `[26:21]`, mask `0x07e00000`
    pub const TX_BCK_DIV_NUM_OFFSET: u32 = 21;
    pub const TX_BCK_DIV_NUM_MASK: u32 = 0x07e00000;

    /// **TX_PDM_EN**
    ///
    /// 1: Enable I2S PDM Tx mode . 0: Disable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TX_PDM_EN_OFFSET: u32 = 20;
    pub const TX_PDM_EN_MASK: u32 = 0x00100000;
    pub const TX_PDM_EN: u32 = 1 << 20;

    /// **TX_TDM_EN**
    ///
    /// 1: Enable I2S TDM Tx mode . 0: Disable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TX_TDM_EN_OFFSET: u32 = 19;
    pub const TX_TDM_EN_MASK: u32 = 0x00080000;
    pub const TX_TDM_EN: u32 = 1 << 19;

    /// **TX_BIT_ORDER**
    ///
    /// I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TX_BIT_ORDER_OFFSET: u32 = 18;
    pub const TX_BIT_ORDER_MASK: u32 = 0x00040000;
    pub const TX_BIT_ORDER: u32 = 1 << 18;

    /// **TX_WS_IDLE_POL**
    ///
    /// 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TX_WS_IDLE_POL_OFFSET: u32 = 17;
    pub const TX_WS_IDLE_POL_MASK: u32 = 0x00020000;
    pub const TX_WS_IDLE_POL: u32 = 1 << 17;

    /// **TX_24_FILL_EN**
    ///
    /// 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TX_24_FILL_EN_OFFSET: u32 = 16;
    pub const TX_24_FILL_EN_MASK: u32 = 0x00010000;
    pub const TX_24_FILL_EN: u32 = 1 << 16;

    /// **TX_LEFT_ALIGN**
    ///
    /// 1: I2S TX left alignment mode. 0: I2S TX right alignment mode.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TX_LEFT_ALIGN_OFFSET: u32 = 15;
    pub const TX_LEFT_ALIGN_MASK: u32 = 0x00008000;
    pub const TX_LEFT_ALIGN: u32 = 1 << 15;

    /// **TX_BCK_NO_DLY**
    ///
    /// 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_BCK_NO_DLY_OFFSET: u32 = 14;
    pub const TX_BCK_NO_DLY_MASK: u32 = 0x00004000;
    pub const TX_BCK_NO_DLY: u32 = 1 << 14;

    /// **TX_MSB_SHIFT**
    ///
    /// Set this bit to enable transmitter in Phillips standard mode
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_MSB_SHIFT_OFFSET: u32 = 13;
    pub const TX_MSB_SHIFT_MASK: u32 = 0x00002000;
    pub const TX_MSB_SHIFT: u32 = 1 << 13;

    /// **TX_PCM_BYPASS**
    ///
    /// Set this bit to bypass Compress/Decompress module for transmitted data.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_PCM_BYPASS_OFFSET: u32 = 12;
    pub const TX_PCM_BYPASS_MASK: u32 = 0x00001000;
    pub const TX_PCM_BYPASS: u32 = 1 << 12;

    /// **TX_PCM_CONF**
    ///
    /// I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const TX_PCM_CONF_OFFSET: u32 = 10;
    pub const TX_PCM_CONF_MASK: u32 = 0x00000c00;

    /// **TX_MONO_FST_VLD**
    ///
    /// 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TX_MONO_FST_VLD_OFFSET: u32 = 9;
    pub const TX_MONO_FST_VLD_MASK: u32 = 0x00000200;
    pub const TX_MONO_FST_VLD: u32 = 1 << 9;

    /// **TX_UPDATE**
    ///
    /// Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_UPDATE_OFFSET: u32 = 8;
    pub const TX_UPDATE_MASK: u32 = 0x00000100;
    pub const TX_UPDATE: u32 = 1 << 8;

    /// **TX_BIG_ENDIAN**
    ///
    /// I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_BIG_ENDIAN_OFFSET: u32 = 7;
    pub const TX_BIG_ENDIAN_MASK: u32 = 0x00000080;
    pub const TX_BIG_ENDIAN: u32 = 1 << 7;

    /// **TX_MONO**
    ///
    /// Set this bit to enable transmitter in mono mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_MONO_OFFSET: u32 = 6;
    pub const TX_MONO_MASK: u32 = 0x00000040;
    pub const TX_MONO: u32 = 1 << 6;

    /// **TX_CHAN_EQUAL**
    ///
    /// 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_CHAN_EQUAL_OFFSET: u32 = 5;
    pub const TX_CHAN_EQUAL_MASK: u32 = 0x00000020;
    pub const TX_CHAN_EQUAL: u32 = 1 << 5;

    /// **TX_STOP_EN**
    ///
    /// Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_STOP_EN_OFFSET: u32 = 4;
    pub const TX_STOP_EN_MASK: u32 = 0x00000010;
    pub const TX_STOP_EN: u32 = 1 << 4;

    /// **TX_SLAVE_MOD**
    ///
    /// Set this bit to enable slave transmitter mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_SLAVE_MOD_OFFSET: u32 = 3;
    pub const TX_SLAVE_MOD_MASK: u32 = 0x00000008;
    pub const TX_SLAVE_MOD: u32 = 1 << 3;

    /// **TX_START**
    ///
    /// Set this bit to start transmitting data
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_START_OFFSET: u32 = 2;
    pub const TX_START_MASK: u32 = 0x00000004;
    pub const TX_START: u32 = 1 << 2;

    /// **TX_FIFO_RESET**
    ///
    /// Set this bit to reset Tx AFIFO
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_FIFO_RESET_OFFSET: u32 = 1;
    pub const TX_FIFO_RESET_MASK: u32 = 0x00000002;
    pub const TX_FIFO_RESET: u32 = 1 << 1;

    /// **TX_RESET**
    ///
    /// Set this bit to reset transmitter
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_RESET_OFFSET: u32 = 0;
    pub const TX_RESET_MASK: u32 = 0x00000001;
    pub const TX_RESET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SIG_LOOPBACK** `[30]` (RW)
    ///
    /// Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals.
    #[inline(always)]
    pub fn sig_loopback(&self) -> bool {
        (self.value.read_bits() & Self::SIG_LOOPBACK_MASK) != 0
    }

    /// Set SIG_LOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_loopback(&self) {
        self.value.modify_bits(|r| r | Self::SIG_LOOPBACK_MASK);
    }

    /// Clear SIG_LOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_loopback(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_LOOPBACK_MASK);
    }

    /// **TX_CHAN_MOD** `[29:27]` (RW)
    ///
    /// I2S transmitter channel mode configuration bits.
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CHAN_MOD_MASK) >> Self::TX_CHAN_MOD_OFFSET
    }

    /// Write **TX_CHAN_MOD** `[29:27]` (RW)
    ///
    /// I2S transmitter channel mode configuration bits.
    #[inline(always)]
    pub fn write_tx_chan_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CHAN_MOD_MASK)
                | ((value << Self::TX_CHAN_MOD_OFFSET) & Self::TX_CHAN_MOD_MASK)
        });
    }

    /// **TX_BCK_DIV_NUM** `[26:21]` (RW)
    ///
    /// Bit clock configuration bits in transmitter mode.
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BCK_DIV_NUM_MASK) >> Self::TX_BCK_DIV_NUM_OFFSET
    }

    /// Write **TX_BCK_DIV_NUM** `[26:21]` (RW)
    ///
    /// Bit clock configuration bits in transmitter mode.
    #[inline(always)]
    pub fn write_tx_bck_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BCK_DIV_NUM_MASK)
                | ((value << Self::TX_BCK_DIV_NUM_OFFSET) & Self::TX_BCK_DIV_NUM_MASK)
        });
    }

    /// **TX_PDM_EN** `[20]` (RW)
    ///
    /// 1: Enable I2S PDM Tx mode . 0: Disable.
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_EN_MASK) != 0
    }

    /// Set TX_PDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_PDM_EN_MASK);
    }

    /// Clear TX_PDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_PDM_EN_MASK);
    }

    /// **TX_TDM_EN** `[19]` (RW)
    ///
    /// 1: Enable I2S TDM Tx mode . 0: Disable.
    #[inline(always)]
    pub fn tx_tdm_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_EN_MASK) != 0
    }

    /// Set TX_TDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_EN_MASK);
    }

    /// Clear TX_TDM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_EN_MASK);
    }

    /// **TX_BIT_ORDER** `[18]` (RW)
    ///
    /// I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first.
    #[inline(always)]
    pub fn tx_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::TX_BIT_ORDER_MASK) != 0
    }

    /// Set TX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_bit_order(&self) {
        self.value.modify_bits(|r| r | Self::TX_BIT_ORDER_MASK);
    }

    /// Clear TX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_bit_order(&self) {
        self.value.modify_bits(|r| r & !Self::TX_BIT_ORDER_MASK);
    }

    /// **TX_WS_IDLE_POL** `[17]` (RW)
    ///
    /// 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub fn tx_ws_idle_pol(&self) -> bool {
        (self.value.read_bits() & Self::TX_WS_IDLE_POL_MASK) != 0
    }

    /// Set TX_WS_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ws_idle_pol(&self) {
        self.value.modify_bits(|r| r | Self::TX_WS_IDLE_POL_MASK);
    }

    /// Clear TX_WS_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ws_idle_pol(&self) {
        self.value.modify_bits(|r| r & !Self::TX_WS_IDLE_POL_MASK);
    }

    /// **TX_24_FILL_EN** `[16]` (RW)
    ///
    /// 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode
    #[inline(always)]
    pub fn tx_24_fill_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_24_FILL_EN_MASK) != 0
    }

    /// Set TX_24_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_24_fill_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_24_FILL_EN_MASK);
    }

    /// Clear TX_24_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_24_fill_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_24_FILL_EN_MASK);
    }

    /// **TX_LEFT_ALIGN** `[15]` (RW)
    ///
    /// 1: I2S TX left alignment mode. 0: I2S TX right alignment mode.
    #[inline(always)]
    pub fn tx_left_align(&self) -> bool {
        (self.value.read_bits() & Self::TX_LEFT_ALIGN_MASK) != 0
    }

    /// Set TX_LEFT_ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_left_align(&self) {
        self.value.modify_bits(|r| r | Self::TX_LEFT_ALIGN_MASK);
    }

    /// Clear TX_LEFT_ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_left_align(&self) {
        self.value.modify_bits(|r| r & !Self::TX_LEFT_ALIGN_MASK);
    }

    /// **TX_BCK_NO_DLY** `[14]` (RW)
    ///
    /// 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode.
    #[inline(always)]
    pub fn tx_bck_no_dly(&self) -> bool {
        (self.value.read_bits() & Self::TX_BCK_NO_DLY_MASK) != 0
    }

    /// Set TX_BCK_NO_DLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_bck_no_dly(&self) {
        self.value.modify_bits(|r| r | Self::TX_BCK_NO_DLY_MASK);
    }

    /// Clear TX_BCK_NO_DLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_bck_no_dly(&self) {
        self.value.modify_bits(|r| r & !Self::TX_BCK_NO_DLY_MASK);
    }

    /// **TX_MSB_SHIFT** `[13]` (RW)
    ///
    /// Set this bit to enable transmitter in Phillips standard mode
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> bool {
        (self.value.read_bits() & Self::TX_MSB_SHIFT_MASK) != 0
    }

    /// Set TX_MSB_SHIFT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_msb_shift(&self) {
        self.value.modify_bits(|r| r | Self::TX_MSB_SHIFT_MASK);
    }

    /// Clear TX_MSB_SHIFT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_msb_shift(&self) {
        self.value.modify_bits(|r| r & !Self::TX_MSB_SHIFT_MASK);
    }

    /// **TX_PCM_BYPASS** `[12]` (RW)
    ///
    /// Set this bit to bypass Compress/Decompress module for transmitted data.
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> bool {
        (self.value.read_bits() & Self::TX_PCM_BYPASS_MASK) != 0
    }

    /// Set TX_PCM_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pcm_bypass(&self) {
        self.value.modify_bits(|r| r | Self::TX_PCM_BYPASS_MASK);
    }

    /// Clear TX_PCM_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pcm_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::TX_PCM_BYPASS_MASK);
    }

    /// **TX_PCM_CONF** `[11:10]` (RW)
    ///
    /// I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PCM_CONF_MASK) >> Self::TX_PCM_CONF_OFFSET
    }

    /// Write **TX_PCM_CONF** `[11:10]` (RW)
    ///
    /// I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub fn write_tx_pcm_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PCM_CONF_MASK)
                | ((value << Self::TX_PCM_CONF_OFFSET) & Self::TX_PCM_CONF_MASK)
        });
    }

    /// **TX_MONO_FST_VLD** `[9]` (RW)
    ///
    /// 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode.
    #[inline(always)]
    pub fn tx_mono_fst_vld(&self) -> bool {
        (self.value.read_bits() & Self::TX_MONO_FST_VLD_MASK) != 0
    }

    /// Set TX_MONO_FST_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_mono_fst_vld(&self) {
        self.value.modify_bits(|r| r | Self::TX_MONO_FST_VLD_MASK);
    }

    /// Clear TX_MONO_FST_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_mono_fst_vld(&self) {
        self.value.modify_bits(|r| r & !Self::TX_MONO_FST_VLD_MASK);
    }

    /// **TX_UPDATE** `[8]` (RW)
    ///
    /// Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub fn tx_update(&self) -> bool {
        (self.value.read_bits() & Self::TX_UPDATE_MASK) != 0
    }

    /// Set TX_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_update(&self) {
        self.value.modify_bits(|r| r | Self::TX_UPDATE_MASK);
    }

    /// Clear TX_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_update(&self) {
        self.value.modify_bits(|r| r & !Self::TX_UPDATE_MASK);
    }

    /// **TX_BIG_ENDIAN** `[7]` (RW)
    ///
    /// I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub fn tx_big_endian(&self) -> bool {
        (self.value.read_bits() & Self::TX_BIG_ENDIAN_MASK) != 0
    }

    /// Set TX_BIG_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_big_endian(&self) {
        self.value.modify_bits(|r| r | Self::TX_BIG_ENDIAN_MASK);
    }

    /// Clear TX_BIG_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_big_endian(&self) {
        self.value.modify_bits(|r| r & !Self::TX_BIG_ENDIAN_MASK);
    }

    /// **TX_MONO** `[6]` (RW)
    ///
    /// Set this bit to enable transmitter in mono mode
    #[inline(always)]
    pub fn tx_mono(&self) -> bool {
        (self.value.read_bits() & Self::TX_MONO_MASK) != 0
    }

    /// Set TX_MONO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_mono(&self) {
        self.value.modify_bits(|r| r | Self::TX_MONO_MASK);
    }

    /// Clear TX_MONO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_mono(&self) {
        self.value.modify_bits(|r| r & !Self::TX_MONO_MASK);
    }

    /// **TX_CHAN_EQUAL** `[5]` (RW)
    ///
    /// 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode.
    #[inline(always)]
    pub fn tx_chan_equal(&self) -> bool {
        (self.value.read_bits() & Self::TX_CHAN_EQUAL_MASK) != 0
    }

    /// Set TX_CHAN_EQUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_chan_equal(&self) {
        self.value.modify_bits(|r| r | Self::TX_CHAN_EQUAL_MASK);
    }

    /// Clear TX_CHAN_EQUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_chan_equal(&self) {
        self.value.modify_bits(|r| r & !Self::TX_CHAN_EQUAL_MASK);
    }

    /// **TX_STOP_EN** `[4]` (RW)
    ///
    /// Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy
    #[inline(always)]
    pub fn tx_stop_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_STOP_EN_MASK) != 0
    }

    /// Set TX_STOP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_stop_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_STOP_EN_MASK);
    }

    /// Clear TX_STOP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_stop_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_STOP_EN_MASK);
    }

    /// **TX_SLAVE_MOD** `[3]` (RW)
    ///
    /// Set this bit to enable slave transmitter mode
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> bool {
        (self.value.read_bits() & Self::TX_SLAVE_MOD_MASK) != 0
    }

    /// Set TX_SLAVE_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_slave_mod(&self) {
        self.value.modify_bits(|r| r | Self::TX_SLAVE_MOD_MASK);
    }

    /// Clear TX_SLAVE_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_slave_mod(&self) {
        self.value.modify_bits(|r| r & !Self::TX_SLAVE_MOD_MASK);
    }

    /// **TX_START** `[2]` (RW)
    ///
    /// Set this bit to start transmitting data
    #[inline(always)]
    pub fn tx_start(&self) -> bool {
        (self.value.read_bits() & Self::TX_START_MASK) != 0
    }

    /// Set TX_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_start(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_MASK);
    }

    /// Clear TX_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_start(&self) {
        self.value.modify_bits(|r| r & !Self::TX_START_MASK);
    }

    /// Trigger **TX_FIFO_RESET** (WO)
    ///
    /// Set this bit to reset Tx AFIFO
    #[inline(always)]
    pub fn trigger_tx_fifo_reset(&self) {
        self.value.modify_bits(|r| r | Self::TX_FIFO_RESET_MASK);
    }

    /// Trigger **TX_RESET** (WO)
    ///
    /// Set this bit to reset transmitter
    #[inline(always)]
    pub fn trigger_tx_reset(&self) {
        self.value.modify_bits(|r| r | Self::TX_RESET_MASK);
    }
}

/// TX_CONF value type for batch read/modify/write operations
///
/// Use `txconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxConfVal {
    bits: u32,
}

impl TxConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00c0f210 }
    }

    /// Read **SIG_LOOPBACK** `[30:30]`
    ///
    /// Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals.
    #[inline(always)]
    pub const fn sig_loopback(&self) -> bool {
        (self.bits & TxConf::SIG_LOOPBACK_MASK) != 0
    }

    /// Read **TX_CHAN_MOD** `[29:27]`
    ///
    /// I2S transmitter channel mode configuration bits.
    #[inline(always)]
    pub const fn tx_chan_mod(&self) -> u32 {
        (self.bits & TxConf::TX_CHAN_MOD_MASK) >> TxConf::TX_CHAN_MOD_OFFSET
    }

    /// Read **TX_BCK_DIV_NUM** `[26:21]`
    ///
    /// Bit clock configuration bits in transmitter mode.
    #[inline(always)]
    pub const fn tx_bck_div_num(&self) -> u32 {
        (self.bits & TxConf::TX_BCK_DIV_NUM_MASK) >> TxConf::TX_BCK_DIV_NUM_OFFSET
    }

    /// Read **TX_PDM_EN** `[20:20]`
    ///
    /// 1: Enable I2S PDM Tx mode . 0: Disable.
    #[inline(always)]
    pub const fn tx_pdm_en(&self) -> bool {
        (self.bits & TxConf::TX_PDM_EN_MASK) != 0
    }

    /// Read **TX_TDM_EN** `[19:19]`
    ///
    /// 1: Enable I2S TDM Tx mode . 0: Disable.
    #[inline(always)]
    pub const fn tx_tdm_en(&self) -> bool {
        (self.bits & TxConf::TX_TDM_EN_MASK) != 0
    }

    /// Read **TX_BIT_ORDER** `[18:18]`
    ///
    /// I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first.
    #[inline(always)]
    pub const fn tx_bit_order(&self) -> bool {
        (self.bits & TxConf::TX_BIT_ORDER_MASK) != 0
    }

    /// Read **TX_WS_IDLE_POL** `[17:17]`
    ///
    /// 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub const fn tx_ws_idle_pol(&self) -> bool {
        (self.bits & TxConf::TX_WS_IDLE_POL_MASK) != 0
    }

    /// Read **TX_24_FILL_EN** `[16:16]`
    ///
    /// 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode
    #[inline(always)]
    pub const fn tx_24_fill_en(&self) -> bool {
        (self.bits & TxConf::TX_24_FILL_EN_MASK) != 0
    }

    /// Read **TX_LEFT_ALIGN** `[15:15]`
    ///
    /// 1: I2S TX left alignment mode. 0: I2S TX right alignment mode.
    #[inline(always)]
    pub const fn tx_left_align(&self) -> bool {
        (self.bits & TxConf::TX_LEFT_ALIGN_MASK) != 0
    }

    /// Read **TX_BCK_NO_DLY** `[14:14]`
    ///
    /// 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode.
    #[inline(always)]
    pub const fn tx_bck_no_dly(&self) -> bool {
        (self.bits & TxConf::TX_BCK_NO_DLY_MASK) != 0
    }

    /// Read **TX_MSB_SHIFT** `[13:13]`
    ///
    /// Set this bit to enable transmitter in Phillips standard mode
    #[inline(always)]
    pub const fn tx_msb_shift(&self) -> bool {
        (self.bits & TxConf::TX_MSB_SHIFT_MASK) != 0
    }

    /// Read **TX_PCM_BYPASS** `[12:12]`
    ///
    /// Set this bit to bypass Compress/Decompress module for transmitted data.
    #[inline(always)]
    pub const fn tx_pcm_bypass(&self) -> bool {
        (self.bits & TxConf::TX_PCM_BYPASS_MASK) != 0
    }

    /// Read **TX_PCM_CONF** `[11:10]`
    ///
    /// I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub const fn tx_pcm_conf(&self) -> u32 {
        (self.bits & TxConf::TX_PCM_CONF_MASK) >> TxConf::TX_PCM_CONF_OFFSET
    }

    /// Read **TX_MONO_FST_VLD** `[9:9]`
    ///
    /// 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode.
    #[inline(always)]
    pub const fn tx_mono_fst_vld(&self) -> bool {
        (self.bits & TxConf::TX_MONO_FST_VLD_MASK) != 0
    }

    /// Read **TX_UPDATE** `[8:8]`
    ///
    /// Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub const fn tx_update(&self) -> bool {
        (self.bits & TxConf::TX_UPDATE_MASK) != 0
    }

    /// Read **TX_BIG_ENDIAN** `[7:7]`
    ///
    /// I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub const fn tx_big_endian(&self) -> bool {
        (self.bits & TxConf::TX_BIG_ENDIAN_MASK) != 0
    }

    /// Read **TX_MONO** `[6:6]`
    ///
    /// Set this bit to enable transmitter in mono mode
    #[inline(always)]
    pub const fn tx_mono(&self) -> bool {
        (self.bits & TxConf::TX_MONO_MASK) != 0
    }

    /// Read **TX_CHAN_EQUAL** `[5:5]`
    ///
    /// 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode.
    #[inline(always)]
    pub const fn tx_chan_equal(&self) -> bool {
        (self.bits & TxConf::TX_CHAN_EQUAL_MASK) != 0
    }

    /// Read **TX_STOP_EN** `[4:4]`
    ///
    /// Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy
    #[inline(always)]
    pub const fn tx_stop_en(&self) -> bool {
        (self.bits & TxConf::TX_STOP_EN_MASK) != 0
    }

    /// Read **TX_SLAVE_MOD** `[3:3]`
    ///
    /// Set this bit to enable slave transmitter mode
    #[inline(always)]
    pub const fn tx_slave_mod(&self) -> bool {
        (self.bits & TxConf::TX_SLAVE_MOD_MASK) != 0
    }

    /// Read **TX_START** `[2:2]`
    ///
    /// Set this bit to start transmitting data
    #[inline(always)]
    pub const fn tx_start(&self) -> bool {
        (self.bits & TxConf::TX_START_MASK) != 0
    }

    /// Set **SIG_LOOPBACK** `[30:30]`
    ///
    /// Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals.
    #[inline(always)]
    pub const fn with_sig_loopback(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::SIG_LOOPBACK_MASK
            } else {
                self.bits & !TxConf::SIG_LOOPBACK_MASK
            },
        }
    }

    /// Set **TX_CHAN_MOD** `[29:27]`
    ///
    /// I2S transmitter channel mode configuration bits.
    #[inline(always)]
    pub const fn with_tx_chan_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf::TX_CHAN_MOD_MASK)
                | ((value << TxConf::TX_CHAN_MOD_OFFSET) & TxConf::TX_CHAN_MOD_MASK),
        }
    }

    /// Set **TX_BCK_DIV_NUM** `[26:21]`
    ///
    /// Bit clock configuration bits in transmitter mode.
    #[inline(always)]
    pub const fn with_tx_bck_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf::TX_BCK_DIV_NUM_MASK)
                | ((value << TxConf::TX_BCK_DIV_NUM_OFFSET) & TxConf::TX_BCK_DIV_NUM_MASK),
        }
    }

    /// Set **TX_PDM_EN** `[20:20]`
    ///
    /// 1: Enable I2S PDM Tx mode . 0: Disable.
    #[inline(always)]
    pub const fn with_tx_pdm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_PDM_EN_MASK
            } else {
                self.bits & !TxConf::TX_PDM_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_EN** `[19:19]`
    ///
    /// 1: Enable I2S TDM Tx mode . 0: Disable.
    #[inline(always)]
    pub const fn with_tx_tdm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_TDM_EN_MASK
            } else {
                self.bits & !TxConf::TX_TDM_EN_MASK
            },
        }
    }

    /// Set **TX_BIT_ORDER** `[18:18]`
    ///
    /// I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first.
    #[inline(always)]
    pub const fn with_tx_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_BIT_ORDER_MASK
            } else {
                self.bits & !TxConf::TX_BIT_ORDER_MASK
            },
        }
    }

    /// Set **TX_WS_IDLE_POL** `[17:17]`
    ///
    /// 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel.
    #[inline(always)]
    pub const fn with_tx_ws_idle_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_WS_IDLE_POL_MASK
            } else {
                self.bits & !TxConf::TX_WS_IDLE_POL_MASK
            },
        }
    }

    /// Set **TX_24_FILL_EN** `[16:16]`
    ///
    /// 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode
    #[inline(always)]
    pub const fn with_tx_24_fill_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_24_FILL_EN_MASK
            } else {
                self.bits & !TxConf::TX_24_FILL_EN_MASK
            },
        }
    }

    /// Set **TX_LEFT_ALIGN** `[15:15]`
    ///
    /// 1: I2S TX left alignment mode. 0: I2S TX right alignment mode.
    #[inline(always)]
    pub const fn with_tx_left_align(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_LEFT_ALIGN_MASK
            } else {
                self.bits & !TxConf::TX_LEFT_ALIGN_MASK
            },
        }
    }

    /// Set **TX_BCK_NO_DLY** `[14:14]`
    ///
    /// 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode.
    #[inline(always)]
    pub const fn with_tx_bck_no_dly(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_BCK_NO_DLY_MASK
            } else {
                self.bits & !TxConf::TX_BCK_NO_DLY_MASK
            },
        }
    }

    /// Set **TX_MSB_SHIFT** `[13:13]`
    ///
    /// Set this bit to enable transmitter in Phillips standard mode
    #[inline(always)]
    pub const fn with_tx_msb_shift(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_MSB_SHIFT_MASK
            } else {
                self.bits & !TxConf::TX_MSB_SHIFT_MASK
            },
        }
    }

    /// Set **TX_PCM_BYPASS** `[12:12]`
    ///
    /// Set this bit to bypass Compress/Decompress module for transmitted data.
    #[inline(always)]
    pub const fn with_tx_pcm_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_PCM_BYPASS_MASK
            } else {
                self.bits & !TxConf::TX_PCM_BYPASS_MASK
            },
        }
    }

    /// Set **TX_PCM_CONF** `[11:10]`
    ///
    /// I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &
    #[inline(always)]
    pub const fn with_tx_pcm_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf::TX_PCM_CONF_MASK)
                | ((value << TxConf::TX_PCM_CONF_OFFSET) & TxConf::TX_PCM_CONF_MASK),
        }
    }

    /// Set **TX_MONO_FST_VLD** `[9:9]`
    ///
    /// 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode.
    #[inline(always)]
    pub const fn with_tx_mono_fst_vld(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_MONO_FST_VLD_MASK
            } else {
                self.bits & !TxConf::TX_MONO_FST_VLD_MASK
            },
        }
    }

    /// Set **TX_UPDATE** `[8:8]`
    ///
    /// Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done.
    #[inline(always)]
    pub const fn with_tx_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_UPDATE_MASK
            } else {
                self.bits & !TxConf::TX_UPDATE_MASK
            },
        }
    }

    /// Set **TX_BIG_ENDIAN** `[7:7]`
    ///
    /// I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value.
    #[inline(always)]
    pub const fn with_tx_big_endian(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_BIG_ENDIAN_MASK
            } else {
                self.bits & !TxConf::TX_BIG_ENDIAN_MASK
            },
        }
    }

    /// Set **TX_MONO** `[6:6]`
    ///
    /// Set this bit to enable transmitter in mono mode
    #[inline(always)]
    pub const fn with_tx_mono(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_MONO_MASK
            } else {
                self.bits & !TxConf::TX_MONO_MASK
            },
        }
    }

    /// Set **TX_CHAN_EQUAL** `[5:5]`
    ///
    /// 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode.
    #[inline(always)]
    pub const fn with_tx_chan_equal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_CHAN_EQUAL_MASK
            } else {
                self.bits & !TxConf::TX_CHAN_EQUAL_MASK
            },
        }
    }

    /// Set **TX_STOP_EN** `[4:4]`
    ///
    /// Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy
    #[inline(always)]
    pub const fn with_tx_stop_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_STOP_EN_MASK
            } else {
                self.bits & !TxConf::TX_STOP_EN_MASK
            },
        }
    }

    /// Set **TX_SLAVE_MOD** `[3:3]`
    ///
    /// Set this bit to enable slave transmitter mode
    #[inline(always)]
    pub const fn with_tx_slave_mod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_SLAVE_MOD_MASK
            } else {
                self.bits & !TxConf::TX_SLAVE_MOD_MASK
            },
        }
    }

    /// Set **TX_START** `[2:2]`
    ///
    /// Set this bit to start transmitting data
    #[inline(always)]
    pub const fn with_tx_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_START_MASK
            } else {
                self.bits & !TxConf::TX_START_MASK
            },
        }
    }

    /// Set **TX_FIFO_RESET** `[1:1]`
    ///
    /// Set this bit to reset Tx AFIFO
    #[inline(always)]
    pub const fn with_tx_fifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_FIFO_RESET_MASK
            } else {
                self.bits & !TxConf::TX_FIFO_RESET_MASK
            },
        }
    }

    /// Set **TX_RESET** `[0:0]`
    ///
    /// Set this bit to reset transmitter
    #[inline(always)]
    pub const fn with_tx_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxConf::TX_RESET_MASK
            } else {
                self.bits & !TxConf::TX_RESET_MASK
            },
        }
    }
}

impl Default for TxConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxConfVal {
        TxConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxConfVal) -> TxConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CONF1 Register
///
/// I2S RX configure register 1
///
/// Address offset: 0x0028
/// Reset value: 0x787bc000
///
/// ## Fields
///
/// - **RX_TDM_CHAN_BITS** `[31:27]` (RW)
///
///   The Rx bit number for each channel minus 1in TDM mode.
///
/// - **RX_HALF_SAMPLE_BITS** `[26:19]` (RW)
///
///   I2S Rx half sample bits -1.
///
/// - **RX_BITS_MOD** `[18:14]` (RW)
///
///   Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
///
/// - **RX_TDM_WS_WIDTH** `[8:0]` (RW)
///
///   The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
#[repr(transparent)]
pub struct RxConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxConf1 {
    // ======== Field Constants ========

    /// **RX_TDM_CHAN_BITS**
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const RX_TDM_CHAN_BITS_OFFSET: u32 = 27;
    pub const RX_TDM_CHAN_BITS_MASK: u32 = 0xf8000000;

    /// **RX_HALF_SAMPLE_BITS**
    ///
    /// I2S Rx half sample bits -1.
    ///
    /// Bits `[26:19]`, mask `0x07f80000`
    pub const RX_HALF_SAMPLE_BITS_OFFSET: u32 = 19;
    pub const RX_HALF_SAMPLE_BITS_MASK: u32 = 0x07f80000;

    /// **RX_BITS_MOD**
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    ///
    /// Bits `[18:14]`, mask `0x0007c000`
    pub const RX_BITS_MOD_OFFSET: u32 = 14;
    pub const RX_BITS_MOD_MASK: u32 = 0x0007c000;

    /// **RX_TDM_WS_WIDTH**
    ///
    /// The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_TDM_WS_WIDTH_OFFSET: u32 = 0;
    pub const RX_TDM_WS_WIDTH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_TDM_CHAN_BITS** `[31:27]` (RW)
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TDM_CHAN_BITS_MASK) >> Self::RX_TDM_CHAN_BITS_OFFSET
    }

    /// Write **RX_TDM_CHAN_BITS** `[31:27]` (RW)
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn write_rx_tdm_chan_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TDM_CHAN_BITS_MASK)
                | ((value << Self::RX_TDM_CHAN_BITS_OFFSET) & Self::RX_TDM_CHAN_BITS_MASK)
        });
    }

    /// **RX_HALF_SAMPLE_BITS** `[26:19]` (RW)
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> u32 {
        (self.value.read_bits() & Self::RX_HALF_SAMPLE_BITS_MASK)
            >> Self::RX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Write **RX_HALF_SAMPLE_BITS** `[26:19]` (RW)
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub fn write_rx_half_sample_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_HALF_SAMPLE_BITS_MASK)
                | ((value << Self::RX_HALF_SAMPLE_BITS_OFFSET) & Self::RX_HALF_SAMPLE_BITS_MASK)
        });
    }

    /// **RX_BITS_MOD** `[18:14]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BITS_MOD_MASK) >> Self::RX_BITS_MOD_OFFSET
    }

    /// Write **RX_BITS_MOD** `[18:14]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn write_rx_bits_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BITS_MOD_MASK)
                | ((value << Self::RX_BITS_MOD_OFFSET) & Self::RX_BITS_MOD_MASK)
        });
    }

    /// **RX_TDM_WS_WIDTH** `[8:0]` (RW)
    ///
    /// The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TDM_WS_WIDTH_MASK) >> Self::RX_TDM_WS_WIDTH_OFFSET
    }

    /// Write **RX_TDM_WS_WIDTH** `[8:0]` (RW)
    ///
    /// The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub fn write_rx_tdm_ws_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TDM_WS_WIDTH_MASK)
                | ((value << Self::RX_TDM_WS_WIDTH_OFFSET) & Self::RX_TDM_WS_WIDTH_MASK)
        });
    }
}

/// RX_CONF1 value type for batch read/modify/write operations
///
/// Use `rxconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxConf1Val {
    bits: u32,
}

impl RxConf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x787bc000 }
    }

    /// Read **RX_TDM_CHAN_BITS** `[31:27]`
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn rx_tdm_chan_bits(&self) -> u32 {
        (self.bits & RxConf1::RX_TDM_CHAN_BITS_MASK) >> RxConf1::RX_TDM_CHAN_BITS_OFFSET
    }

    /// Read **RX_HALF_SAMPLE_BITS** `[26:19]`
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub const fn rx_half_sample_bits(&self) -> u32 {
        (self.bits & RxConf1::RX_HALF_SAMPLE_BITS_MASK) >> RxConf1::RX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Read **RX_BITS_MOD** `[18:14]`
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn rx_bits_mod(&self) -> u32 {
        (self.bits & RxConf1::RX_BITS_MOD_MASK) >> RxConf1::RX_BITS_MOD_OFFSET
    }

    /// Read **RX_TDM_WS_WIDTH** `[8:0]`
    ///
    /// The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub const fn rx_tdm_ws_width(&self) -> u32 {
        (self.bits & RxConf1::RX_TDM_WS_WIDTH_MASK) >> RxConf1::RX_TDM_WS_WIDTH_OFFSET
    }

    /// Set **RX_TDM_CHAN_BITS** `[31:27]`
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn with_rx_tdm_chan_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_TDM_CHAN_BITS_MASK)
                | ((value << RxConf1::RX_TDM_CHAN_BITS_OFFSET) & RxConf1::RX_TDM_CHAN_BITS_MASK),
        }
    }

    /// Set **RX_HALF_SAMPLE_BITS** `[26:19]`
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub const fn with_rx_half_sample_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_HALF_SAMPLE_BITS_MASK)
                | ((value << RxConf1::RX_HALF_SAMPLE_BITS_OFFSET)
                    & RxConf1::RX_HALF_SAMPLE_BITS_MASK),
        }
    }

    /// Set **RX_BITS_MOD** `[18:14]`
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn with_rx_bits_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_BITS_MOD_MASK)
                | ((value << RxConf1::RX_BITS_MOD_OFFSET) & RxConf1::RX_BITS_MOD_MASK),
        }
    }

    /// Set **RX_TDM_WS_WIDTH** `[8:0]`
    ///
    /// The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub const fn with_rx_tdm_ws_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_TDM_WS_WIDTH_MASK)
                | ((value << RxConf1::RX_TDM_WS_WIDTH_OFFSET) & RxConf1::RX_TDM_WS_WIDTH_MASK),
        }
    }
}

impl Default for RxConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxConf1Val {
        RxConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxConf1Val) -> RxConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CONF1 Register
///
/// I2S TX configure register 1
///
/// Address offset: 0x002c
/// Reset value: 0x787bc000
///
/// ## Fields
///
/// - **TX_TDM_CHAN_BITS** `[31:27]` (RW)
///
///   The Tx bit number for each channel minus 1in TDM mode.
///
/// - **TX_HALF_SAMPLE_BITS** `[26:19]` (RW)
///
///   I2S Tx half sample bits -1.
///
/// - **TX_BITS_MOD** `[18:14]` (RW)
///
///   Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
///
/// - **TX_TDM_WS_WIDTH** `[8:0]` (RW)
///
///   The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
#[repr(transparent)]
pub struct TxConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxConf1 {
    // ======== Field Constants ========

    /// **TX_TDM_CHAN_BITS**
    ///
    /// The Tx bit number for each channel minus 1in TDM mode.
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const TX_TDM_CHAN_BITS_OFFSET: u32 = 27;
    pub const TX_TDM_CHAN_BITS_MASK: u32 = 0xf8000000;

    /// **TX_HALF_SAMPLE_BITS**
    ///
    /// I2S Tx half sample bits -1.
    ///
    /// Bits `[26:19]`, mask `0x07f80000`
    pub const TX_HALF_SAMPLE_BITS_OFFSET: u32 = 19;
    pub const TX_HALF_SAMPLE_BITS_MASK: u32 = 0x07f80000;

    /// **TX_BITS_MOD**
    ///
    /// Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    ///
    /// Bits `[18:14]`, mask `0x0007c000`
    pub const TX_BITS_MOD_OFFSET: u32 = 14;
    pub const TX_BITS_MOD_MASK: u32 = 0x0007c000;

    /// **TX_TDM_WS_WIDTH**
    ///
    /// The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TX_TDM_WS_WIDTH_OFFSET: u32 = 0;
    pub const TX_TDM_WS_WIDTH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TX_TDM_CHAN_BITS** `[31:27]` (RW)
    ///
    /// The Tx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn tx_tdm_chan_bits(&self) -> u32 {
        (self.value.read_bits() & Self::TX_TDM_CHAN_BITS_MASK) >> Self::TX_TDM_CHAN_BITS_OFFSET
    }

    /// Write **TX_TDM_CHAN_BITS** `[31:27]` (RW)
    ///
    /// The Tx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn write_tx_tdm_chan_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_TDM_CHAN_BITS_MASK)
                | ((value << Self::TX_TDM_CHAN_BITS_OFFSET) & Self::TX_TDM_CHAN_BITS_MASK)
        });
    }

    /// **TX_HALF_SAMPLE_BITS** `[26:19]` (RW)
    ///
    /// I2S Tx half sample bits -1.
    #[inline(always)]
    pub fn tx_half_sample_bits(&self) -> u32 {
        (self.value.read_bits() & Self::TX_HALF_SAMPLE_BITS_MASK)
            >> Self::TX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Write **TX_HALF_SAMPLE_BITS** `[26:19]` (RW)
    ///
    /// I2S Tx half sample bits -1.
    #[inline(always)]
    pub fn write_tx_half_sample_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_HALF_SAMPLE_BITS_MASK)
                | ((value << Self::TX_HALF_SAMPLE_BITS_OFFSET) & Self::TX_HALF_SAMPLE_BITS_MASK)
        });
    }

    /// **TX_BITS_MOD** `[18:14]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BITS_MOD_MASK) >> Self::TX_BITS_MOD_OFFSET
    }

    /// Write **TX_BITS_MOD** `[18:14]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn write_tx_bits_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BITS_MOD_MASK)
                | ((value << Self::TX_BITS_MOD_OFFSET) & Self::TX_BITS_MOD_MASK)
        });
    }

    /// **TX_TDM_WS_WIDTH** `[8:0]` (RW)
    ///
    /// The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub fn tx_tdm_ws_width(&self) -> u32 {
        (self.value.read_bits() & Self::TX_TDM_WS_WIDTH_MASK) >> Self::TX_TDM_WS_WIDTH_OFFSET
    }

    /// Write **TX_TDM_WS_WIDTH** `[8:0]` (RW)
    ///
    /// The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub fn write_tx_tdm_ws_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_TDM_WS_WIDTH_MASK)
                | ((value << Self::TX_TDM_WS_WIDTH_OFFSET) & Self::TX_TDM_WS_WIDTH_MASK)
        });
    }
}

/// TX_CONF1 value type for batch read/modify/write operations
///
/// Use `txconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxConf1Val {
    bits: u32,
}

impl TxConf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x787bc000 }
    }

    /// Read **TX_TDM_CHAN_BITS** `[31:27]`
    ///
    /// The Tx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn tx_tdm_chan_bits(&self) -> u32 {
        (self.bits & TxConf1::TX_TDM_CHAN_BITS_MASK) >> TxConf1::TX_TDM_CHAN_BITS_OFFSET
    }

    /// Read **TX_HALF_SAMPLE_BITS** `[26:19]`
    ///
    /// I2S Tx half sample bits -1.
    #[inline(always)]
    pub const fn tx_half_sample_bits(&self) -> u32 {
        (self.bits & TxConf1::TX_HALF_SAMPLE_BITS_MASK) >> TxConf1::TX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Read **TX_BITS_MOD** `[18:14]`
    ///
    /// Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn tx_bits_mod(&self) -> u32 {
        (self.bits & TxConf1::TX_BITS_MOD_MASK) >> TxConf1::TX_BITS_MOD_OFFSET
    }

    /// Read **TX_TDM_WS_WIDTH** `[8:0]`
    ///
    /// The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub const fn tx_tdm_ws_width(&self) -> u32 {
        (self.bits & TxConf1::TX_TDM_WS_WIDTH_MASK) >> TxConf1::TX_TDM_WS_WIDTH_OFFSET
    }

    /// Set **TX_TDM_CHAN_BITS** `[31:27]`
    ///
    /// The Tx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn with_tx_tdm_chan_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf1::TX_TDM_CHAN_BITS_MASK)
                | ((value << TxConf1::TX_TDM_CHAN_BITS_OFFSET) & TxConf1::TX_TDM_CHAN_BITS_MASK),
        }
    }

    /// Set **TX_HALF_SAMPLE_BITS** `[26:19]`
    ///
    /// I2S Tx half sample bits -1.
    #[inline(always)]
    pub const fn with_tx_half_sample_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf1::TX_HALF_SAMPLE_BITS_MASK)
                | ((value << TxConf1::TX_HALF_SAMPLE_BITS_OFFSET)
                    & TxConf1::TX_HALF_SAMPLE_BITS_MASK),
        }
    }

    /// Set **TX_BITS_MOD** `[18:14]`
    ///
    /// Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn with_tx_bits_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf1::TX_BITS_MOD_MASK)
                | ((value << TxConf1::TX_BITS_MOD_OFFSET) & TxConf1::TX_BITS_MOD_MASK),
        }
    }

    /// Set **TX_TDM_WS_WIDTH** `[8:0]`
    ///
    /// The width of tx_ws_out at idle level in TDM mode is (I2S_TX_TDM_WS_WIDTH[8:0] +1) * T_bck
    #[inline(always)]
    pub const fn with_tx_tdm_ws_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxConf1::TX_TDM_WS_WIDTH_MASK)
                | ((value << TxConf1::TX_TDM_WS_WIDTH_OFFSET) & TxConf1::TX_TDM_WS_WIDTH_MASK),
        }
    }
}

impl Default for TxConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxConf1Val {
        TxConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxConf1Val) -> TxConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_PCM2PDM_CONF Register
///
/// I2S TX PCM2PDM configuration register
///
/// Address offset: 0x0040
/// Reset value: 0x004aa004
///
/// ## Fields
///
/// - **PCM2PDM_CONV_EN** `[25]` (RW)
///
///   I2S TX PDM Converter enable
///
/// - **TX_PDM_DAC_MODE_EN** `[24]` (RW)
///
///   I2S TX PDM dac 2channel enable
///
/// - **TX_PDM_DAC_2OUT_EN** `[23]` (RW)
///
///   I2S TX PDM dac mode enable
///
/// - **TX_PDM_SIGMADELTA_DITHER** `[22]` (RW)
///
///   I2S TX PDM sigmadelta dither value
///
/// - **TX_PDM_SIGMADELTA_DITHER2** `[21]` (RW)
///
///   I2S TX PDM sigmadelta dither2 value
///
/// - **TX_PDM_SIGMADELTA_IN_SHIFT** `[20:19]` (RW)
///
///   I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
///
/// - **TX_PDM_SINC_IN_SHIFT** `[18:17]` (RW)
///
///   I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
///
/// - **TX_PDM_LP_IN_SHIFT** `[16:15]` (RW)
///
///   I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
///
/// - **TX_PDM_HP_IN_SHIFT** `[14:13]` (RW)
///
///   I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
///
/// - **TX_PDM_PRESCALE** `[12:5]` (RW)
///
///   I2S TX PDM prescale for sigmadelta
///
/// - **TX_PDM_SINC_OSR2** `[4:1]` (RW)
///
///   I2S TX PDM OSR2 value
///
/// - **TX_PDM_HP_BYPASS** `[0]` (RW)
///
///   I2S TX PDM bypass hp filter or not. The option has been removed.
#[repr(transparent)]
pub struct TxPcm2pdmConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxPcm2pdmConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxPcm2pdmConf {
    // ======== Field Constants ========

    /// **PCM2PDM_CONV_EN**
    ///
    /// I2S TX PDM Converter enable
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const PCM2PDM_CONV_EN_OFFSET: u32 = 25;
    pub const PCM2PDM_CONV_EN_MASK: u32 = 0x02000000;
    pub const PCM2PDM_CONV_EN: u32 = 1 << 25;

    /// **TX_PDM_DAC_MODE_EN**
    ///
    /// I2S TX PDM dac 2channel enable
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TX_PDM_DAC_MODE_EN_OFFSET: u32 = 24;
    pub const TX_PDM_DAC_MODE_EN_MASK: u32 = 0x01000000;
    pub const TX_PDM_DAC_MODE_EN: u32 = 1 << 24;

    /// **TX_PDM_DAC_2OUT_EN**
    ///
    /// I2S TX PDM dac mode enable
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TX_PDM_DAC_2OUT_EN_OFFSET: u32 = 23;
    pub const TX_PDM_DAC_2OUT_EN_MASK: u32 = 0x00800000;
    pub const TX_PDM_DAC_2OUT_EN: u32 = 1 << 23;

    /// **TX_PDM_SIGMADELTA_DITHER**
    ///
    /// I2S TX PDM sigmadelta dither value
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TX_PDM_SIGMADELTA_DITHER_OFFSET: u32 = 22;
    pub const TX_PDM_SIGMADELTA_DITHER_MASK: u32 = 0x00400000;
    pub const TX_PDM_SIGMADELTA_DITHER: u32 = 1 << 22;

    /// **TX_PDM_SIGMADELTA_DITHER2**
    ///
    /// I2S TX PDM sigmadelta dither2 value
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TX_PDM_SIGMADELTA_DITHER2_OFFSET: u32 = 21;
    pub const TX_PDM_SIGMADELTA_DITHER2_MASK: u32 = 0x00200000;
    pub const TX_PDM_SIGMADELTA_DITHER2: u32 = 1 << 21;

    /// **TX_PDM_SIGMADELTA_IN_SHIFT**
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    ///
    /// Bits `[20:19]`, mask `0x00180000`
    pub const TX_PDM_SIGMADELTA_IN_SHIFT_OFFSET: u32 = 19;
    pub const TX_PDM_SIGMADELTA_IN_SHIFT_MASK: u32 = 0x00180000;

    /// **TX_PDM_SINC_IN_SHIFT**
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    ///
    /// Bits `[18:17]`, mask `0x00060000`
    pub const TX_PDM_SINC_IN_SHIFT_OFFSET: u32 = 17;
    pub const TX_PDM_SINC_IN_SHIFT_MASK: u32 = 0x00060000;

    /// **TX_PDM_LP_IN_SHIFT**
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    ///
    /// Bits `[16:15]`, mask `0x00018000`
    pub const TX_PDM_LP_IN_SHIFT_OFFSET: u32 = 15;
    pub const TX_PDM_LP_IN_SHIFT_MASK: u32 = 0x00018000;

    /// **TX_PDM_HP_IN_SHIFT**
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    ///
    /// Bits `[14:13]`, mask `0x00006000`
    pub const TX_PDM_HP_IN_SHIFT_OFFSET: u32 = 13;
    pub const TX_PDM_HP_IN_SHIFT_MASK: u32 = 0x00006000;

    /// **TX_PDM_PRESCALE**
    ///
    /// I2S TX PDM prescale for sigmadelta
    ///
    /// Bits `[12:5]`, mask `0x00001fe0`
    pub const TX_PDM_PRESCALE_OFFSET: u32 = 5;
    pub const TX_PDM_PRESCALE_MASK: u32 = 0x00001fe0;

    /// **TX_PDM_SINC_OSR2**
    ///
    /// I2S TX PDM OSR2 value
    ///
    /// Bits `[4:1]`, mask `0x0000001e`
    pub const TX_PDM_SINC_OSR2_OFFSET: u32 = 1;
    pub const TX_PDM_SINC_OSR2_MASK: u32 = 0x0000001e;

    /// **TX_PDM_HP_BYPASS**
    ///
    /// I2S TX PDM bypass hp filter or not. The option has been removed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_PDM_HP_BYPASS_OFFSET: u32 = 0;
    pub const TX_PDM_HP_BYPASS_MASK: u32 = 0x00000001;
    pub const TX_PDM_HP_BYPASS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PCM2PDM_CONV_EN** `[25]` (RW)
    ///
    /// I2S TX PDM Converter enable
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> bool {
        (self.value.read_bits() & Self::PCM2PDM_CONV_EN_MASK) != 0
    }

    /// Set PCM2PDM_CONV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pcm2pdm_conv_en(&self) {
        self.value.modify_bits(|r| r | Self::PCM2PDM_CONV_EN_MASK);
    }

    /// Clear PCM2PDM_CONV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pcm2pdm_conv_en(&self) {
        self.value.modify_bits(|r| r & !Self::PCM2PDM_CONV_EN_MASK);
    }

    /// **TX_PDM_DAC_MODE_EN** `[24]` (RW)
    ///
    /// I2S TX PDM dac 2channel enable
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_DAC_MODE_EN_MASK) != 0
    }

    /// Set TX_PDM_DAC_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_dac_mode_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_PDM_DAC_MODE_EN_MASK);
    }

    /// Clear TX_PDM_DAC_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_dac_mode_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_PDM_DAC_MODE_EN_MASK);
    }

    /// **TX_PDM_DAC_2OUT_EN** `[23]` (RW)
    ///
    /// I2S TX PDM dac mode enable
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_DAC_2OUT_EN_MASK) != 0
    }

    /// Set TX_PDM_DAC_2OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_dac_2out_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_PDM_DAC_2OUT_EN_MASK);
    }

    /// Clear TX_PDM_DAC_2OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_dac_2out_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_PDM_DAC_2OUT_EN_MASK);
    }

    /// **TX_PDM_SIGMADELTA_DITHER** `[22]` (RW)
    ///
    /// I2S TX PDM sigmadelta dither value
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_SIGMADELTA_DITHER_MASK) != 0
    }

    /// Set TX_PDM_SIGMADELTA_DITHER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_sigmadelta_dither(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_PDM_SIGMADELTA_DITHER_MASK);
    }

    /// Clear TX_PDM_SIGMADELTA_DITHER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_sigmadelta_dither(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_PDM_SIGMADELTA_DITHER_MASK);
    }

    /// **TX_PDM_SIGMADELTA_DITHER2** `[21]` (RW)
    ///
    /// I2S TX PDM sigmadelta dither2 value
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_SIGMADELTA_DITHER2_MASK) != 0
    }

    /// Set TX_PDM_SIGMADELTA_DITHER2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_sigmadelta_dither2(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_PDM_SIGMADELTA_DITHER2_MASK);
    }

    /// Clear TX_PDM_SIGMADELTA_DITHER2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_sigmadelta_dither2(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_PDM_SIGMADELTA_DITHER2_MASK);
    }

    /// **TX_PDM_SIGMADELTA_IN_SHIFT** `[20:19]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_SIGMADELTA_IN_SHIFT_MASK)
            >> Self::TX_PDM_SIGMADELTA_IN_SHIFT_OFFSET
    }

    /// Write **TX_PDM_SIGMADELTA_IN_SHIFT** `[20:19]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn write_tx_pdm_sigmadelta_in_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_SIGMADELTA_IN_SHIFT_MASK)
                | ((value << Self::TX_PDM_SIGMADELTA_IN_SHIFT_OFFSET)
                    & Self::TX_PDM_SIGMADELTA_IN_SHIFT_MASK)
        });
    }

    /// **TX_PDM_SINC_IN_SHIFT** `[18:17]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_SINC_IN_SHIFT_MASK)
            >> Self::TX_PDM_SINC_IN_SHIFT_OFFSET
    }

    /// Write **TX_PDM_SINC_IN_SHIFT** `[18:17]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn write_tx_pdm_sinc_in_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_SINC_IN_SHIFT_MASK)
                | ((value << Self::TX_PDM_SINC_IN_SHIFT_OFFSET) & Self::TX_PDM_SINC_IN_SHIFT_MASK)
        });
    }

    /// **TX_PDM_LP_IN_SHIFT** `[16:15]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_LP_IN_SHIFT_MASK) >> Self::TX_PDM_LP_IN_SHIFT_OFFSET
    }

    /// Write **TX_PDM_LP_IN_SHIFT** `[16:15]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn write_tx_pdm_lp_in_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_LP_IN_SHIFT_MASK)
                | ((value << Self::TX_PDM_LP_IN_SHIFT_OFFSET) & Self::TX_PDM_LP_IN_SHIFT_MASK)
        });
    }

    /// **TX_PDM_HP_IN_SHIFT** `[14:13]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_HP_IN_SHIFT_MASK) >> Self::TX_PDM_HP_IN_SHIFT_OFFSET
    }

    /// Write **TX_PDM_HP_IN_SHIFT** `[14:13]` (RW)
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn write_tx_pdm_hp_in_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_HP_IN_SHIFT_MASK)
                | ((value << Self::TX_PDM_HP_IN_SHIFT_OFFSET) & Self::TX_PDM_HP_IN_SHIFT_MASK)
        });
    }

    /// **TX_PDM_PRESCALE** `[12:5]` (RW)
    ///
    /// I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_PRESCALE_MASK) >> Self::TX_PDM_PRESCALE_OFFSET
    }

    /// Write **TX_PDM_PRESCALE** `[12:5]` (RW)
    ///
    /// I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    pub fn write_tx_pdm_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_PRESCALE_MASK)
                | ((value << Self::TX_PDM_PRESCALE_OFFSET) & Self::TX_PDM_PRESCALE_MASK)
        });
    }

    /// **TX_PDM_SINC_OSR2** `[4:1]` (RW)
    ///
    /// I2S TX PDM OSR2 value
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_SINC_OSR2_MASK) >> Self::TX_PDM_SINC_OSR2_OFFSET
    }

    /// Write **TX_PDM_SINC_OSR2** `[4:1]` (RW)
    ///
    /// I2S TX PDM OSR2 value
    #[inline(always)]
    pub fn write_tx_pdm_sinc_osr2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_SINC_OSR2_MASK)
                | ((value << Self::TX_PDM_SINC_OSR2_OFFSET) & Self::TX_PDM_SINC_OSR2_MASK)
        });
    }

    /// **TX_PDM_HP_BYPASS** `[0]` (RW)
    ///
    /// I2S TX PDM bypass hp filter or not. The option has been removed.
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> bool {
        (self.value.read_bits() & Self::TX_PDM_HP_BYPASS_MASK) != 0
    }

    /// Set TX_PDM_HP_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pdm_hp_bypass(&self) {
        self.value.modify_bits(|r| r | Self::TX_PDM_HP_BYPASS_MASK);
    }

    /// Clear TX_PDM_HP_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pdm_hp_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::TX_PDM_HP_BYPASS_MASK);
    }
}

/// TX_PCM2PDM_CONF value type for batch read/modify/write operations
///
/// Use `txpcm2pdmconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txpcm2pdmconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxPcm2pdmConfVal {
    bits: u32,
}

impl TxPcm2pdmConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x004aa004 }
    }

    /// Read **PCM2PDM_CONV_EN** `[25:25]`
    ///
    /// I2S TX PDM Converter enable
    #[inline(always)]
    pub const fn pcm2pdm_conv_en(&self) -> bool {
        (self.bits & TxPcm2pdmConf::PCM2PDM_CONV_EN_MASK) != 0
    }

    /// Read **TX_PDM_DAC_MODE_EN** `[24:24]`
    ///
    /// I2S TX PDM dac 2channel enable
    #[inline(always)]
    pub const fn tx_pdm_dac_mode_en(&self) -> bool {
        (self.bits & TxPcm2pdmConf::TX_PDM_DAC_MODE_EN_MASK) != 0
    }

    /// Read **TX_PDM_DAC_2OUT_EN** `[23:23]`
    ///
    /// I2S TX PDM dac mode enable
    #[inline(always)]
    pub const fn tx_pdm_dac_2out_en(&self) -> bool {
        (self.bits & TxPcm2pdmConf::TX_PDM_DAC_2OUT_EN_MASK) != 0
    }

    /// Read **TX_PDM_SIGMADELTA_DITHER** `[22:22]`
    ///
    /// I2S TX PDM sigmadelta dither value
    #[inline(always)]
    pub const fn tx_pdm_sigmadelta_dither(&self) -> bool {
        (self.bits & TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER_MASK) != 0
    }

    /// Read **TX_PDM_SIGMADELTA_DITHER2** `[21:21]`
    ///
    /// I2S TX PDM sigmadelta dither2 value
    #[inline(always)]
    pub const fn tx_pdm_sigmadelta_dither2(&self) -> bool {
        (self.bits & TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER2_MASK) != 0
    }

    /// Read **TX_PDM_SIGMADELTA_IN_SHIFT** `[20:19]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn tx_pdm_sigmadelta_in_shift(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_SIGMADELTA_IN_SHIFT_MASK)
            >> TxPcm2pdmConf::TX_PDM_SIGMADELTA_IN_SHIFT_OFFSET
    }

    /// Read **TX_PDM_SINC_IN_SHIFT** `[18:17]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn tx_pdm_sinc_in_shift(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_SINC_IN_SHIFT_MASK)
            >> TxPcm2pdmConf::TX_PDM_SINC_IN_SHIFT_OFFSET
    }

    /// Read **TX_PDM_LP_IN_SHIFT** `[16:15]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn tx_pdm_lp_in_shift(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_LP_IN_SHIFT_MASK)
            >> TxPcm2pdmConf::TX_PDM_LP_IN_SHIFT_OFFSET
    }

    /// Read **TX_PDM_HP_IN_SHIFT** `[14:13]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn tx_pdm_hp_in_shift(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_HP_IN_SHIFT_MASK)
            >> TxPcm2pdmConf::TX_PDM_HP_IN_SHIFT_OFFSET
    }

    /// Read **TX_PDM_PRESCALE** `[12:5]`
    ///
    /// I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    pub const fn tx_pdm_prescale(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_PRESCALE_MASK) >> TxPcm2pdmConf::TX_PDM_PRESCALE_OFFSET
    }

    /// Read **TX_PDM_SINC_OSR2** `[4:1]`
    ///
    /// I2S TX PDM OSR2 value
    #[inline(always)]
    pub const fn tx_pdm_sinc_osr2(&self) -> u32 {
        (self.bits & TxPcm2pdmConf::TX_PDM_SINC_OSR2_MASK) >> TxPcm2pdmConf::TX_PDM_SINC_OSR2_OFFSET
    }

    /// Read **TX_PDM_HP_BYPASS** `[0:0]`
    ///
    /// I2S TX PDM bypass hp filter or not. The option has been removed.
    #[inline(always)]
    pub const fn tx_pdm_hp_bypass(&self) -> bool {
        (self.bits & TxPcm2pdmConf::TX_PDM_HP_BYPASS_MASK) != 0
    }

    /// Set **PCM2PDM_CONV_EN** `[25:25]`
    ///
    /// I2S TX PDM Converter enable
    #[inline(always)]
    pub const fn with_pcm2pdm_conv_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::PCM2PDM_CONV_EN_MASK
            } else {
                self.bits & !TxPcm2pdmConf::PCM2PDM_CONV_EN_MASK
            },
        }
    }

    /// Set **TX_PDM_DAC_MODE_EN** `[24:24]`
    ///
    /// I2S TX PDM dac 2channel enable
    #[inline(always)]
    pub const fn with_tx_pdm_dac_mode_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::TX_PDM_DAC_MODE_EN_MASK
            } else {
                self.bits & !TxPcm2pdmConf::TX_PDM_DAC_MODE_EN_MASK
            },
        }
    }

    /// Set **TX_PDM_DAC_2OUT_EN** `[23:23]`
    ///
    /// I2S TX PDM dac mode enable
    #[inline(always)]
    pub const fn with_tx_pdm_dac_2out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::TX_PDM_DAC_2OUT_EN_MASK
            } else {
                self.bits & !TxPcm2pdmConf::TX_PDM_DAC_2OUT_EN_MASK
            },
        }
    }

    /// Set **TX_PDM_SIGMADELTA_DITHER** `[22:22]`
    ///
    /// I2S TX PDM sigmadelta dither value
    #[inline(always)]
    pub const fn with_tx_pdm_sigmadelta_dither(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER_MASK
            } else {
                self.bits & !TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER_MASK
            },
        }
    }

    /// Set **TX_PDM_SIGMADELTA_DITHER2** `[21:21]`
    ///
    /// I2S TX PDM sigmadelta dither2 value
    #[inline(always)]
    pub const fn with_tx_pdm_sigmadelta_dither2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER2_MASK
            } else {
                self.bits & !TxPcm2pdmConf::TX_PDM_SIGMADELTA_DITHER2_MASK
            },
        }
    }

    /// Set **TX_PDM_SIGMADELTA_IN_SHIFT** `[20:19]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn with_tx_pdm_sigmadelta_in_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_SIGMADELTA_IN_SHIFT_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_SIGMADELTA_IN_SHIFT_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_SIGMADELTA_IN_SHIFT_MASK),
        }
    }

    /// Set **TX_PDM_SINC_IN_SHIFT** `[18:17]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn with_tx_pdm_sinc_in_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_SINC_IN_SHIFT_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_SINC_IN_SHIFT_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_SINC_IN_SHIFT_MASK),
        }
    }

    /// Set **TX_PDM_LP_IN_SHIFT** `[16:15]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn with_tx_pdm_lp_in_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_LP_IN_SHIFT_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_LP_IN_SHIFT_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_LP_IN_SHIFT_MASK),
        }
    }

    /// Set **TX_PDM_HP_IN_SHIFT** `[14:13]`
    ///
    /// I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub const fn with_tx_pdm_hp_in_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_HP_IN_SHIFT_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_HP_IN_SHIFT_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_HP_IN_SHIFT_MASK),
        }
    }

    /// Set **TX_PDM_PRESCALE** `[12:5]`
    ///
    /// I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    pub const fn with_tx_pdm_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_PRESCALE_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_PRESCALE_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_PRESCALE_MASK),
        }
    }

    /// Set **TX_PDM_SINC_OSR2** `[4:1]`
    ///
    /// I2S TX PDM OSR2 value
    #[inline(always)]
    pub const fn with_tx_pdm_sinc_osr2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf::TX_PDM_SINC_OSR2_MASK)
                | ((value << TxPcm2pdmConf::TX_PDM_SINC_OSR2_OFFSET)
                    & TxPcm2pdmConf::TX_PDM_SINC_OSR2_MASK),
        }
    }

    /// Set **TX_PDM_HP_BYPASS** `[0:0]`
    ///
    /// I2S TX PDM bypass hp filter or not. The option has been removed.
    #[inline(always)]
    pub const fn with_tx_pdm_hp_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxPcm2pdmConf::TX_PDM_HP_BYPASS_MASK
            } else {
                self.bits & !TxPcm2pdmConf::TX_PDM_HP_BYPASS_MASK
            },
        }
    }
}

impl Default for TxPcm2pdmConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxPcm2pdmConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxPcm2pdmConfVal {
        TxPcm2pdmConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxPcm2pdmConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxPcm2pdmConfVal) -> TxPcm2pdmConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_PCM2PDM_CONF1 Register
///
/// I2S TX PCM2PDM configuration register
///
/// Address offset: 0x0044
/// Reset value: 0x03f783c0
///
/// ## Fields
///
/// - **TX_IIR_HP_MULT12_0** `[25:23]` (RW)
///
///   The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
///
/// - **TX_IIR_HP_MULT12_5** `[22:20]` (RW)
///
///   The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
///
/// - **TX_PDM_FS** `[19:10]` (RW)
///
///   I2S TX PDM Fs
///
/// - **TX_PDM_FP** `[9:0]` (RW)
///
///   I2S TX PDM Fp
#[repr(transparent)]
pub struct TxPcm2pdmConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxPcm2pdmConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxPcm2pdmConf1 {
    // ======== Field Constants ========

    /// **TX_IIR_HP_MULT12_0**
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
    ///
    /// Bits `[25:23]`, mask `0x03800000`
    pub const TX_IIR_HP_MULT12_0_OFFSET: u32 = 23;
    pub const TX_IIR_HP_MULT12_0_MASK: u32 = 0x03800000;

    /// **TX_IIR_HP_MULT12_5**
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const TX_IIR_HP_MULT12_5_OFFSET: u32 = 20;
    pub const TX_IIR_HP_MULT12_5_MASK: u32 = 0x00700000;

    /// **TX_PDM_FS**
    ///
    /// I2S TX PDM Fs
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const TX_PDM_FS_OFFSET: u32 = 10;
    pub const TX_PDM_FS_MASK: u32 = 0x000ffc00;

    /// **TX_PDM_FP**
    ///
    /// I2S TX PDM Fp
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const TX_PDM_FP_OFFSET: u32 = 0;
    pub const TX_PDM_FP_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **TX_IIR_HP_MULT12_0** `[25:23]` (RW)
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&self) -> u32 {
        (self.value.read_bits() & Self::TX_IIR_HP_MULT12_0_MASK) >> Self::TX_IIR_HP_MULT12_0_OFFSET
    }

    /// Write **TX_IIR_HP_MULT12_0** `[25:23]` (RW)
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub fn write_tx_iir_hp_mult12_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_IIR_HP_MULT12_0_MASK)
                | ((value << Self::TX_IIR_HP_MULT12_0_OFFSET) & Self::TX_IIR_HP_MULT12_0_MASK)
        });
    }

    /// **TX_IIR_HP_MULT12_5** `[22:20]` (RW)
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&self) -> u32 {
        (self.value.read_bits() & Self::TX_IIR_HP_MULT12_5_MASK) >> Self::TX_IIR_HP_MULT12_5_OFFSET
    }

    /// Write **TX_IIR_HP_MULT12_5** `[22:20]` (RW)
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub fn write_tx_iir_hp_mult12_5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_IIR_HP_MULT12_5_MASK)
                | ((value << Self::TX_IIR_HP_MULT12_5_OFFSET) & Self::TX_IIR_HP_MULT12_5_MASK)
        });
    }

    /// **TX_PDM_FS** `[19:10]` (RW)
    ///
    /// I2S TX PDM Fs
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_FS_MASK) >> Self::TX_PDM_FS_OFFSET
    }

    /// Write **TX_PDM_FS** `[19:10]` (RW)
    ///
    /// I2S TX PDM Fs
    #[inline(always)]
    pub fn write_tx_pdm_fs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_FS_MASK) | ((value << Self::TX_PDM_FS_OFFSET) & Self::TX_PDM_FS_MASK)
        });
    }

    /// **TX_PDM_FP** `[9:0]` (RW)
    ///
    /// I2S TX PDM Fp
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PDM_FP_MASK) >> Self::TX_PDM_FP_OFFSET
    }

    /// Write **TX_PDM_FP** `[9:0]` (RW)
    ///
    /// I2S TX PDM Fp
    #[inline(always)]
    pub fn write_tx_pdm_fp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PDM_FP_MASK) | ((value << Self::TX_PDM_FP_OFFSET) & Self::TX_PDM_FP_MASK)
        });
    }
}

/// TX_PCM2PDM_CONF1 value type for batch read/modify/write operations
///
/// Use `txpcm2pdmconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txpcm2pdmconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxPcm2pdmConf1Val {
    bits: u32,
}

impl TxPcm2pdmConf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x03f783c0 }
    }

    /// Read **TX_IIR_HP_MULT12_0** `[25:23]`
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub const fn tx_iir_hp_mult12_0(&self) -> u32 {
        (self.bits & TxPcm2pdmConf1::TX_IIR_HP_MULT12_0_MASK)
            >> TxPcm2pdmConf1::TX_IIR_HP_MULT12_0_OFFSET
    }

    /// Read **TX_IIR_HP_MULT12_5** `[22:20]`
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn tx_iir_hp_mult12_5(&self) -> u32 {
        (self.bits & TxPcm2pdmConf1::TX_IIR_HP_MULT12_5_MASK)
            >> TxPcm2pdmConf1::TX_IIR_HP_MULT12_5_OFFSET
    }

    /// Read **TX_PDM_FS** `[19:10]`
    ///
    /// I2S TX PDM Fs
    #[inline(always)]
    pub const fn tx_pdm_fs(&self) -> u32 {
        (self.bits & TxPcm2pdmConf1::TX_PDM_FS_MASK) >> TxPcm2pdmConf1::TX_PDM_FS_OFFSET
    }

    /// Read **TX_PDM_FP** `[9:0]`
    ///
    /// I2S TX PDM Fp
    #[inline(always)]
    pub const fn tx_pdm_fp(&self) -> u32 {
        (self.bits & TxPcm2pdmConf1::TX_PDM_FP_MASK) >> TxPcm2pdmConf1::TX_PDM_FP_OFFSET
    }

    /// Set **TX_IIR_HP_MULT12_0** `[25:23]`
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub const fn with_tx_iir_hp_mult12_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf1::TX_IIR_HP_MULT12_0_MASK)
                | ((value << TxPcm2pdmConf1::TX_IIR_HP_MULT12_0_OFFSET)
                    & TxPcm2pdmConf1::TX_IIR_HP_MULT12_0_MASK),
        }
    }

    /// Set **TX_IIR_HP_MULT12_5** `[22:20]`
    ///
    /// The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn with_tx_iir_hp_mult12_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf1::TX_IIR_HP_MULT12_5_MASK)
                | ((value << TxPcm2pdmConf1::TX_IIR_HP_MULT12_5_OFFSET)
                    & TxPcm2pdmConf1::TX_IIR_HP_MULT12_5_MASK),
        }
    }

    /// Set **TX_PDM_FS** `[19:10]`
    ///
    /// I2S TX PDM Fs
    #[inline(always)]
    pub const fn with_tx_pdm_fs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf1::TX_PDM_FS_MASK)
                | ((value << TxPcm2pdmConf1::TX_PDM_FS_OFFSET) & TxPcm2pdmConf1::TX_PDM_FS_MASK),
        }
    }

    /// Set **TX_PDM_FP** `[9:0]`
    ///
    /// I2S TX PDM Fp
    #[inline(always)]
    pub const fn with_tx_pdm_fp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxPcm2pdmConf1::TX_PDM_FP_MASK)
                | ((value << TxPcm2pdmConf1::TX_PDM_FP_OFFSET) & TxPcm2pdmConf1::TX_PDM_FP_MASK),
        }
    }
}

impl Default for TxPcm2pdmConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxPcm2pdmConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxPcm2pdmConf1Val {
        TxPcm2pdmConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxPcm2pdmConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxPcm2pdmConf1Val) -> TxPcm2pdmConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_PDM2PCM_CONF Register
///
/// I2S RX configure register
///
/// Address offset: 0x0048
/// Reset value: 0xf8200000
///
/// ## Fields
///
/// - **RX_IIR_HP_MULT12_0** `[31:29]` (RW)
///
///   The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
///
/// - **RX_IIR_HP_MULT12_5** `[28:26]` (RW)
///
///   The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
///
/// - **RX_PDM_HP_BYPASS** `[25]` (RW)
///
///   I2S PDM RX bypass hp filter or not.
///
/// - **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]` (RW)
///
///   Configure PDM RX amplify number.
///
/// - **RX_PDM_SINC_DSR_16_EN** `[20]` (RW)
///
///   Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
///
/// - **RX_PDM2PCM_EN** `[19]` (RW)
///
///   1: Enable PDM2PCM RX mode. 0: DIsable.
#[repr(transparent)]
pub struct RxPdm2pcmConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxPdm2pcmConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxPdm2pcmConf {
    // ======== Field Constants ========

    /// **RX_IIR_HP_MULT12_0**
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const RX_IIR_HP_MULT12_0_OFFSET: u32 = 29;
    pub const RX_IIR_HP_MULT12_0_MASK: u32 = 0xe0000000;

    /// **RX_IIR_HP_MULT12_5**
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const RX_IIR_HP_MULT12_5_OFFSET: u32 = 26;
    pub const RX_IIR_HP_MULT12_5_MASK: u32 = 0x1c000000;

    /// **RX_PDM_HP_BYPASS**
    ///
    /// I2S PDM RX bypass hp filter or not.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const RX_PDM_HP_BYPASS_OFFSET: u32 = 25;
    pub const RX_PDM_HP_BYPASS_MASK: u32 = 0x02000000;
    pub const RX_PDM_HP_BYPASS: u32 = 1 << 25;

    /// **RX_PDM2PCM_AMPLIFY_NUM**
    ///
    /// Configure PDM RX amplify number.
    ///
    /// Bits `[24:21]`, mask `0x01e00000`
    pub const RX_PDM2PCM_AMPLIFY_NUM_OFFSET: u32 = 21;
    pub const RX_PDM2PCM_AMPLIFY_NUM_MASK: u32 = 0x01e00000;

    /// **RX_PDM_SINC_DSR_16_EN**
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_PDM_SINC_DSR_16_EN_OFFSET: u32 = 20;
    pub const RX_PDM_SINC_DSR_16_EN_MASK: u32 = 0x00100000;
    pub const RX_PDM_SINC_DSR_16_EN: u32 = 1 << 20;

    /// **RX_PDM2PCM_EN**
    ///
    /// 1: Enable PDM2PCM RX mode. 0: DIsable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const RX_PDM2PCM_EN_OFFSET: u32 = 19;
    pub const RX_PDM2PCM_EN_MASK: u32 = 0x00080000;
    pub const RX_PDM2PCM_EN: u32 = 1 << 19;

    // ======== Field Accessors ========

    /// **RX_IIR_HP_MULT12_0** `[31:29]` (RW)
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub fn rx_iir_hp_mult12_0(&self) -> u32 {
        (self.value.read_bits() & Self::RX_IIR_HP_MULT12_0_MASK) >> Self::RX_IIR_HP_MULT12_0_OFFSET
    }

    /// Write **RX_IIR_HP_MULT12_0** `[31:29]` (RW)
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub fn write_rx_iir_hp_mult12_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_IIR_HP_MULT12_0_MASK)
                | ((value << Self::RX_IIR_HP_MULT12_0_OFFSET) & Self::RX_IIR_HP_MULT12_0_MASK)
        });
    }

    /// **RX_IIR_HP_MULT12_5** `[28:26]` (RW)
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub fn rx_iir_hp_mult12_5(&self) -> u32 {
        (self.value.read_bits() & Self::RX_IIR_HP_MULT12_5_MASK) >> Self::RX_IIR_HP_MULT12_5_OFFSET
    }

    /// Write **RX_IIR_HP_MULT12_5** `[28:26]` (RW)
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub fn write_rx_iir_hp_mult12_5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_IIR_HP_MULT12_5_MASK)
                | ((value << Self::RX_IIR_HP_MULT12_5_OFFSET) & Self::RX_IIR_HP_MULT12_5_MASK)
        });
    }

    /// **RX_PDM_HP_BYPASS** `[25]` (RW)
    ///
    /// I2S PDM RX bypass hp filter or not.
    #[inline(always)]
    pub fn rx_pdm_hp_bypass(&self) -> bool {
        (self.value.read_bits() & Self::RX_PDM_HP_BYPASS_MASK) != 0
    }

    /// Set RX_PDM_HP_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pdm_hp_bypass(&self) {
        self.value.modify_bits(|r| r | Self::RX_PDM_HP_BYPASS_MASK);
    }

    /// Clear RX_PDM_HP_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pdm_hp_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::RX_PDM_HP_BYPASS_MASK);
    }

    /// **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]` (RW)
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub fn rx_pdm2pcm_amplify_num(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PDM2PCM_AMPLIFY_NUM_MASK)
            >> Self::RX_PDM2PCM_AMPLIFY_NUM_OFFSET
    }

    /// Write **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]` (RW)
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub fn write_rx_pdm2pcm_amplify_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PDM2PCM_AMPLIFY_NUM_MASK)
                | ((value << Self::RX_PDM2PCM_AMPLIFY_NUM_OFFSET)
                    & Self::RX_PDM2PCM_AMPLIFY_NUM_MASK)
        });
    }

    /// **RX_PDM_SINC_DSR_16_EN** `[20]` (RW)
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_PDM_SINC_DSR_16_EN_MASK) != 0
    }

    /// Set RX_PDM_SINC_DSR_16_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pdm_sinc_dsr_16_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_PDM_SINC_DSR_16_EN_MASK);
    }

    /// Clear RX_PDM_SINC_DSR_16_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pdm_sinc_dsr_16_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_PDM_SINC_DSR_16_EN_MASK);
    }

    /// **RX_PDM2PCM_EN** `[19]` (RW)
    ///
    /// 1: Enable PDM2PCM RX mode. 0: DIsable.
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_PDM2PCM_EN_MASK) != 0
    }

    /// Set RX_PDM2PCM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pdm2pcm_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_PDM2PCM_EN_MASK);
    }

    /// Clear RX_PDM2PCM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pdm2pcm_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_PDM2PCM_EN_MASK);
    }
}

/// RX_PDM2PCM_CONF value type for batch read/modify/write operations
///
/// Use `rxpdm2pcmconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxpdm2pcmconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxPdm2pcmConfVal {
    bits: u32,
}

impl RxPdm2pcmConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xf8200000 }
    }

    /// Read **RX_IIR_HP_MULT12_0** `[31:29]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub const fn rx_iir_hp_mult12_0(&self) -> u32 {
        (self.bits & RxPdm2pcmConf::RX_IIR_HP_MULT12_0_MASK)
            >> RxPdm2pcmConf::RX_IIR_HP_MULT12_0_OFFSET
    }

    /// Read **RX_IIR_HP_MULT12_5** `[28:26]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn rx_iir_hp_mult12_5(&self) -> u32 {
        (self.bits & RxPdm2pcmConf::RX_IIR_HP_MULT12_5_MASK)
            >> RxPdm2pcmConf::RX_IIR_HP_MULT12_5_OFFSET
    }

    /// Read **RX_PDM_HP_BYPASS** `[25:25]`
    ///
    /// I2S PDM RX bypass hp filter or not.
    #[inline(always)]
    pub const fn rx_pdm_hp_bypass(&self) -> bool {
        (self.bits & RxPdm2pcmConf::RX_PDM_HP_BYPASS_MASK) != 0
    }

    /// Read **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]`
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub const fn rx_pdm2pcm_amplify_num(&self) -> u32 {
        (self.bits & RxPdm2pcmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK)
            >> RxPdm2pcmConf::RX_PDM2PCM_AMPLIFY_NUM_OFFSET
    }

    /// Read **RX_PDM_SINC_DSR_16_EN** `[20:20]`
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    #[inline(always)]
    pub const fn rx_pdm_sinc_dsr_16_en(&self) -> bool {
        (self.bits & RxPdm2pcmConf::RX_PDM_SINC_DSR_16_EN_MASK) != 0
    }

    /// Read **RX_PDM2PCM_EN** `[19:19]`
    ///
    /// 1: Enable PDM2PCM RX mode. 0: DIsable.
    #[inline(always)]
    pub const fn rx_pdm2pcm_en(&self) -> bool {
        (self.bits & RxPdm2pcmConf::RX_PDM2PCM_EN_MASK) != 0
    }

    /// Set **RX_IIR_HP_MULT12_0** `[31:29]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub const fn with_rx_iir_hp_mult12_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdm2pcmConf::RX_IIR_HP_MULT12_0_MASK)
                | ((value << RxPdm2pcmConf::RX_IIR_HP_MULT12_0_OFFSET)
                    & RxPdm2pcmConf::RX_IIR_HP_MULT12_0_MASK),
        }
    }

    /// Set **RX_IIR_HP_MULT12_5** `[28:26]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn with_rx_iir_hp_mult12_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdm2pcmConf::RX_IIR_HP_MULT12_5_MASK)
                | ((value << RxPdm2pcmConf::RX_IIR_HP_MULT12_5_OFFSET)
                    & RxPdm2pcmConf::RX_IIR_HP_MULT12_5_MASK),
        }
    }

    /// Set **RX_PDM_HP_BYPASS** `[25:25]`
    ///
    /// I2S PDM RX bypass hp filter or not.
    #[inline(always)]
    pub const fn with_rx_pdm_hp_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxPdm2pcmConf::RX_PDM_HP_BYPASS_MASK
            } else {
                self.bits & !RxPdm2pcmConf::RX_PDM_HP_BYPASS_MASK
            },
        }
    }

    /// Set **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]`
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub const fn with_rx_pdm2pcm_amplify_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdm2pcmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK)
                | ((value << RxPdm2pcmConf::RX_PDM2PCM_AMPLIFY_NUM_OFFSET)
                    & RxPdm2pcmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK),
        }
    }

    /// Set **RX_PDM_SINC_DSR_16_EN** `[20:20]`
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    #[inline(always)]
    pub const fn with_rx_pdm_sinc_dsr_16_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxPdm2pcmConf::RX_PDM_SINC_DSR_16_EN_MASK
            } else {
                self.bits & !RxPdm2pcmConf::RX_PDM_SINC_DSR_16_EN_MASK
            },
        }
    }

    /// Set **RX_PDM2PCM_EN** `[19:19]`
    ///
    /// 1: Enable PDM2PCM RX mode. 0: DIsable.
    #[inline(always)]
    pub const fn with_rx_pdm2pcm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxPdm2pcmConf::RX_PDM2PCM_EN_MASK
            } else {
                self.bits & !RxPdm2pcmConf::RX_PDM2PCM_EN_MASK
            },
        }
    }
}

impl Default for RxPdm2pcmConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxPdm2pcmConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxPdm2pcmConfVal {
        RxPdm2pcmConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxPdm2pcmConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxPdm2pcmConfVal) -> RxPdm2pcmConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_TDM_CTRL Register
///
/// I2S TX TDM mode control register
///
/// Address offset: 0x0050
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **RX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
///
///   The total channel number of I2S TX TDM mode.
///
/// - **RX_TDM_CHAN15_EN** `[15]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN14_EN** `[14]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN13_EN** `[13]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN12_EN** `[12]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN11_EN** `[11]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN10_EN** `[10]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN9_EN** `[9]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_CHAN8_EN** `[8]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN7_EN** `[7]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN6_EN** `[6]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN5_EN** `[5]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN4_EN** `[4]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN3_EN** `[3]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN2_EN** `[2]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN1_EN** `[1]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
///
/// - **RX_TDM_PDM_CHAN0_EN** `[0]` (RW)
///
///   1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
#[repr(transparent)]
pub struct RxTdmCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxTdmCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxTdmCtrl {
    // ======== Field Constants ========

    /// **RX_TDM_TOT_CHAN_NUM**
    ///
    /// The total channel number of I2S TX TDM mode.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const RX_TDM_TOT_CHAN_NUM_OFFSET: u32 = 16;
    pub const RX_TDM_TOT_CHAN_NUM_MASK: u32 = 0x000f0000;

    /// **RX_TDM_CHAN15_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RX_TDM_CHAN15_EN_OFFSET: u32 = 15;
    pub const RX_TDM_CHAN15_EN_MASK: u32 = 0x00008000;
    pub const RX_TDM_CHAN15_EN: u32 = 1 << 15;

    /// **RX_TDM_CHAN14_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const RX_TDM_CHAN14_EN_OFFSET: u32 = 14;
    pub const RX_TDM_CHAN14_EN_MASK: u32 = 0x00004000;
    pub const RX_TDM_CHAN14_EN: u32 = 1 << 14;

    /// **RX_TDM_CHAN13_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const RX_TDM_CHAN13_EN_OFFSET: u32 = 13;
    pub const RX_TDM_CHAN13_EN_MASK: u32 = 0x00002000;
    pub const RX_TDM_CHAN13_EN: u32 = 1 << 13;

    /// **RX_TDM_CHAN12_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_TDM_CHAN12_EN_OFFSET: u32 = 12;
    pub const RX_TDM_CHAN12_EN_MASK: u32 = 0x00001000;
    pub const RX_TDM_CHAN12_EN: u32 = 1 << 12;

    /// **RX_TDM_CHAN11_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RX_TDM_CHAN11_EN_OFFSET: u32 = 11;
    pub const RX_TDM_CHAN11_EN_MASK: u32 = 0x00000800;
    pub const RX_TDM_CHAN11_EN: u32 = 1 << 11;

    /// **RX_TDM_CHAN10_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RX_TDM_CHAN10_EN_OFFSET: u32 = 10;
    pub const RX_TDM_CHAN10_EN_MASK: u32 = 0x00000400;
    pub const RX_TDM_CHAN10_EN: u32 = 1 << 10;

    /// **RX_TDM_CHAN9_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RX_TDM_CHAN9_EN_OFFSET: u32 = 9;
    pub const RX_TDM_CHAN9_EN_MASK: u32 = 0x00000200;
    pub const RX_TDM_CHAN9_EN: u32 = 1 << 9;

    /// **RX_TDM_CHAN8_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RX_TDM_CHAN8_EN_OFFSET: u32 = 8;
    pub const RX_TDM_CHAN8_EN_MASK: u32 = 0x00000100;
    pub const RX_TDM_CHAN8_EN: u32 = 1 << 8;

    /// **RX_TDM_PDM_CHAN7_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_TDM_PDM_CHAN7_EN_OFFSET: u32 = 7;
    pub const RX_TDM_PDM_CHAN7_EN_MASK: u32 = 0x00000080;
    pub const RX_TDM_PDM_CHAN7_EN: u32 = 1 << 7;

    /// **RX_TDM_PDM_CHAN6_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_TDM_PDM_CHAN6_EN_OFFSET: u32 = 6;
    pub const RX_TDM_PDM_CHAN6_EN_MASK: u32 = 0x00000040;
    pub const RX_TDM_PDM_CHAN6_EN: u32 = 1 << 6;

    /// **RX_TDM_PDM_CHAN5_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_TDM_PDM_CHAN5_EN_OFFSET: u32 = 5;
    pub const RX_TDM_PDM_CHAN5_EN_MASK: u32 = 0x00000020;
    pub const RX_TDM_PDM_CHAN5_EN: u32 = 1 << 5;

    /// **RX_TDM_PDM_CHAN4_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_TDM_PDM_CHAN4_EN_OFFSET: u32 = 4;
    pub const RX_TDM_PDM_CHAN4_EN_MASK: u32 = 0x00000010;
    pub const RX_TDM_PDM_CHAN4_EN: u32 = 1 << 4;

    /// **RX_TDM_PDM_CHAN3_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_TDM_PDM_CHAN3_EN_OFFSET: u32 = 3;
    pub const RX_TDM_PDM_CHAN3_EN_MASK: u32 = 0x00000008;
    pub const RX_TDM_PDM_CHAN3_EN: u32 = 1 << 3;

    /// **RX_TDM_PDM_CHAN2_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_TDM_PDM_CHAN2_EN_OFFSET: u32 = 2;
    pub const RX_TDM_PDM_CHAN2_EN_MASK: u32 = 0x00000004;
    pub const RX_TDM_PDM_CHAN2_EN: u32 = 1 << 2;

    /// **RX_TDM_PDM_CHAN1_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_TDM_PDM_CHAN1_EN_OFFSET: u32 = 1;
    pub const RX_TDM_PDM_CHAN1_EN_MASK: u32 = 0x00000002;
    pub const RX_TDM_PDM_CHAN1_EN: u32 = 1 << 1;

    /// **RX_TDM_PDM_CHAN0_EN**
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_TDM_PDM_CHAN0_EN_OFFSET: u32 = 0;
    pub const RX_TDM_PDM_CHAN0_EN_MASK: u32 = 0x00000001;
    pub const RX_TDM_PDM_CHAN0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TDM_TOT_CHAN_NUM_MASK)
            >> Self::RX_TDM_TOT_CHAN_NUM_OFFSET
    }

    /// Write **RX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub fn write_rx_tdm_tot_chan_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TDM_TOT_CHAN_NUM_MASK)
                | ((value << Self::RX_TDM_TOT_CHAN_NUM_OFFSET) & Self::RX_TDM_TOT_CHAN_NUM_MASK)
        });
    }

    /// **RX_TDM_CHAN15_EN** `[15]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan15_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN15_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan15_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN15_EN_MASK);
    }

    /// Clear RX_TDM_CHAN15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan15_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN15_EN_MASK);
    }

    /// **RX_TDM_CHAN14_EN** `[14]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan14_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN14_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan14_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN14_EN_MASK);
    }

    /// Clear RX_TDM_CHAN14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan14_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN14_EN_MASK);
    }

    /// **RX_TDM_CHAN13_EN** `[13]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan13_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN13_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan13_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN13_EN_MASK);
    }

    /// Clear RX_TDM_CHAN13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan13_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN13_EN_MASK);
    }

    /// **RX_TDM_CHAN12_EN** `[12]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan12_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN12_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan12_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN12_EN_MASK);
    }

    /// Clear RX_TDM_CHAN12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan12_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN12_EN_MASK);
    }

    /// **RX_TDM_CHAN11_EN** `[11]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan11_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN11_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan11_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN11_EN_MASK);
    }

    /// Clear RX_TDM_CHAN11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan11_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN11_EN_MASK);
    }

    /// **RX_TDM_CHAN10_EN** `[10]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan10_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN10_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan10_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN10_EN_MASK);
    }

    /// Clear RX_TDM_CHAN10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan10_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN10_EN_MASK);
    }

    /// **RX_TDM_CHAN9_EN** `[9]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan9_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN9_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan9_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN9_EN_MASK);
    }

    /// Clear RX_TDM_CHAN9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan9_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN9_EN_MASK);
    }

    /// **RX_TDM_CHAN8_EN** `[8]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_chan8_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_CHAN8_EN_MASK) != 0
    }

    /// Set RX_TDM_CHAN8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_chan8_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TDM_CHAN8_EN_MASK);
    }

    /// Clear RX_TDM_CHAN8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_chan8_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TDM_CHAN8_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN7_EN** `[7]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan7_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN7_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN7_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN7_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN6_EN** `[6]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan6_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN6_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN6_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN6_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN5_EN** `[5]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan5_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN5_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN5_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN5_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN4_EN** `[4]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan4_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN4_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN4_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN4_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN3_EN** `[3]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan3_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN3_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN3_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN3_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN2_EN** `[2]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan2_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN2_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN2_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN2_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN1_EN** `[1]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN1_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN1_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN1_EN_MASK);
    }

    /// **RX_TDM_PDM_CHAN0_EN** `[0]` (RW)
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TDM_PDM_CHAN0_EN_MASK) != 0
    }

    /// Set RX_TDM_PDM_CHAN0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tdm_pdm_chan0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_TDM_PDM_CHAN0_EN_MASK);
    }

    /// Clear RX_TDM_PDM_CHAN0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tdm_pdm_chan0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TDM_PDM_CHAN0_EN_MASK);
    }
}

/// RX_TDM_CTRL value type for batch read/modify/write operations
///
/// Use `rxtdmctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxtdmctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxTdmCtrlVal {
    bits: u32,
}

impl RxTdmCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **RX_TDM_TOT_CHAN_NUM** `[19:16]`
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub const fn rx_tdm_tot_chan_num(&self) -> u32 {
        (self.bits & RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_MASK) >> RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_OFFSET
    }

    /// Read **RX_TDM_CHAN15_EN** `[15:15]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan15_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN15_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN14_EN** `[14:14]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan14_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN14_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN13_EN** `[13:13]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan13_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN13_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN12_EN** `[12:12]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan12_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN12_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN11_EN** `[11:11]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan11_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN11_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN10_EN** `[10:10]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan10_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN10_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN9_EN** `[9:9]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan9_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN9_EN_MASK) != 0
    }

    /// Read **RX_TDM_CHAN8_EN** `[8:8]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_chan8_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_CHAN8_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN7_EN** `[7:7]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan7_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN7_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN6_EN** `[6:6]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan6_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN6_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN5_EN** `[5:5]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan5_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN5_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN4_EN** `[4:4]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan4_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN4_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN3_EN** `[3:3]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan3_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN3_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN2_EN** `[2:2]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan2_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN2_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN1_EN** `[1:1]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan1_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN1_EN_MASK) != 0
    }

    /// Read **RX_TDM_PDM_CHAN0_EN** `[0:0]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn rx_tdm_pdm_chan0_en(&self) -> bool {
        (self.bits & RxTdmCtrl::RX_TDM_PDM_CHAN0_EN_MASK) != 0
    }

    /// Set **RX_TDM_TOT_CHAN_NUM** `[19:16]`
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub const fn with_rx_tdm_tot_chan_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_MASK)
                | ((value << RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_OFFSET)
                    & RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_MASK),
        }
    }

    /// Set **RX_TDM_CHAN15_EN** `[15:15]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan15_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN15_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN15_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN14_EN** `[14:14]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan14_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN14_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN14_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN13_EN** `[13:13]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan13_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN13_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN13_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN12_EN** `[12:12]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan12_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN12_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN12_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN11_EN** `[11:11]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan11_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN11_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN11_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN10_EN** `[10:10]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan10_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN10_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN10_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN9_EN** `[9:9]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan9_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN9_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN9_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN8_EN** `[8:8]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_chan8_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_CHAN8_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_CHAN8_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN7_EN** `[7:7]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN7_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN7_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN6_EN** `[6:6]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN6_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN6_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN5_EN** `[5:5]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN5_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN5_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN4_EN** `[4:4]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN4_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN4_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN3_EN** `[3:3]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN3_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN3_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN2_EN** `[2:2]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN2_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN2_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN1_EN** `[1:1]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN1_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN1_EN_MASK
            },
        }
    }

    /// Set **RX_TDM_PDM_CHAN0_EN** `[0:0]`
    ///
    /// 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub const fn with_rx_tdm_pdm_chan0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxTdmCtrl::RX_TDM_PDM_CHAN0_EN_MASK
            } else {
                self.bits & !RxTdmCtrl::RX_TDM_PDM_CHAN0_EN_MASK
            },
        }
    }
}

impl Default for RxTdmCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxTdmCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxTdmCtrlVal {
        RxTdmCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxTdmCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxTdmCtrlVal) -> RxTdmCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_TDM_CTRL Register
///
/// I2S TX TDM mode control register
///
/// Address offset: 0x0054
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **TX_TDM_SKIP_MSK_EN** `[20]` (RW)
///
///   When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels.
///
/// - **TX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
///
///   The total channel number of I2S TX TDM mode.
///
/// - **TX_TDM_CHAN15_EN** `[15]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN14_EN** `[14]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN13_EN** `[13]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN12_EN** `[12]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN11_EN** `[11]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN10_EN** `[10]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN9_EN** `[9]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN8_EN** `[8]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN7_EN** `[7]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN6_EN** `[6]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN5_EN** `[5]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN4_EN** `[4]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN3_EN** `[3]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN2_EN** `[2]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN1_EN** `[1]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel.
///
/// - **TX_TDM_CHAN0_EN** `[0]` (RW)
///
///   1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel.
#[repr(transparent)]
pub struct TxTdmCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxTdmCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxTdmCtrl {
    // ======== Field Constants ========

    /// **TX_TDM_SKIP_MSK_EN**
    ///
    /// When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TX_TDM_SKIP_MSK_EN_OFFSET: u32 = 20;
    pub const TX_TDM_SKIP_MSK_EN_MASK: u32 = 0x00100000;
    pub const TX_TDM_SKIP_MSK_EN: u32 = 1 << 20;

    /// **TX_TDM_TOT_CHAN_NUM**
    ///
    /// The total channel number of I2S TX TDM mode.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const TX_TDM_TOT_CHAN_NUM_OFFSET: u32 = 16;
    pub const TX_TDM_TOT_CHAN_NUM_MASK: u32 = 0x000f0000;

    /// **TX_TDM_CHAN15_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TX_TDM_CHAN15_EN_OFFSET: u32 = 15;
    pub const TX_TDM_CHAN15_EN_MASK: u32 = 0x00008000;
    pub const TX_TDM_CHAN15_EN: u32 = 1 << 15;

    /// **TX_TDM_CHAN14_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_TDM_CHAN14_EN_OFFSET: u32 = 14;
    pub const TX_TDM_CHAN14_EN_MASK: u32 = 0x00004000;
    pub const TX_TDM_CHAN14_EN: u32 = 1 << 14;

    /// **TX_TDM_CHAN13_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_TDM_CHAN13_EN_OFFSET: u32 = 13;
    pub const TX_TDM_CHAN13_EN_MASK: u32 = 0x00002000;
    pub const TX_TDM_CHAN13_EN: u32 = 1 << 13;

    /// **TX_TDM_CHAN12_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_TDM_CHAN12_EN_OFFSET: u32 = 12;
    pub const TX_TDM_CHAN12_EN_MASK: u32 = 0x00001000;
    pub const TX_TDM_CHAN12_EN: u32 = 1 << 12;

    /// **TX_TDM_CHAN11_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TX_TDM_CHAN11_EN_OFFSET: u32 = 11;
    pub const TX_TDM_CHAN11_EN_MASK: u32 = 0x00000800;
    pub const TX_TDM_CHAN11_EN: u32 = 1 << 11;

    /// **TX_TDM_CHAN10_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TX_TDM_CHAN10_EN_OFFSET: u32 = 10;
    pub const TX_TDM_CHAN10_EN_MASK: u32 = 0x00000400;
    pub const TX_TDM_CHAN10_EN: u32 = 1 << 10;

    /// **TX_TDM_CHAN9_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TX_TDM_CHAN9_EN_OFFSET: u32 = 9;
    pub const TX_TDM_CHAN9_EN_MASK: u32 = 0x00000200;
    pub const TX_TDM_CHAN9_EN: u32 = 1 << 9;

    /// **TX_TDM_CHAN8_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_TDM_CHAN8_EN_OFFSET: u32 = 8;
    pub const TX_TDM_CHAN8_EN_MASK: u32 = 0x00000100;
    pub const TX_TDM_CHAN8_EN: u32 = 1 << 8;

    /// **TX_TDM_CHAN7_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_TDM_CHAN7_EN_OFFSET: u32 = 7;
    pub const TX_TDM_CHAN7_EN_MASK: u32 = 0x00000080;
    pub const TX_TDM_CHAN7_EN: u32 = 1 << 7;

    /// **TX_TDM_CHAN6_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_TDM_CHAN6_EN_OFFSET: u32 = 6;
    pub const TX_TDM_CHAN6_EN_MASK: u32 = 0x00000040;
    pub const TX_TDM_CHAN6_EN: u32 = 1 << 6;

    /// **TX_TDM_CHAN5_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_TDM_CHAN5_EN_OFFSET: u32 = 5;
    pub const TX_TDM_CHAN5_EN_MASK: u32 = 0x00000020;
    pub const TX_TDM_CHAN5_EN: u32 = 1 << 5;

    /// **TX_TDM_CHAN4_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_TDM_CHAN4_EN_OFFSET: u32 = 4;
    pub const TX_TDM_CHAN4_EN_MASK: u32 = 0x00000010;
    pub const TX_TDM_CHAN4_EN: u32 = 1 << 4;

    /// **TX_TDM_CHAN3_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_TDM_CHAN3_EN_OFFSET: u32 = 3;
    pub const TX_TDM_CHAN3_EN_MASK: u32 = 0x00000008;
    pub const TX_TDM_CHAN3_EN: u32 = 1 << 3;

    /// **TX_TDM_CHAN2_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_TDM_CHAN2_EN_OFFSET: u32 = 2;
    pub const TX_TDM_CHAN2_EN_MASK: u32 = 0x00000004;
    pub const TX_TDM_CHAN2_EN: u32 = 1 << 2;

    /// **TX_TDM_CHAN1_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_TDM_CHAN1_EN_OFFSET: u32 = 1;
    pub const TX_TDM_CHAN1_EN_MASK: u32 = 0x00000002;
    pub const TX_TDM_CHAN1_EN: u32 = 1 << 1;

    /// **TX_TDM_CHAN0_EN**
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_TDM_CHAN0_EN_OFFSET: u32 = 0;
    pub const TX_TDM_CHAN0_EN_MASK: u32 = 0x00000001;
    pub const TX_TDM_CHAN0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_TDM_SKIP_MSK_EN** `[20]` (RW)
    ///
    /// When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels.
    #[inline(always)]
    pub fn tx_tdm_skip_msk_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_SKIP_MSK_EN_MASK) != 0
    }

    /// Set TX_TDM_SKIP_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_skip_msk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_TDM_SKIP_MSK_EN_MASK);
    }

    /// Clear TX_TDM_SKIP_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_skip_msk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_TDM_SKIP_MSK_EN_MASK);
    }

    /// **TX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub fn tx_tdm_tot_chan_num(&self) -> u32 {
        (self.value.read_bits() & Self::TX_TDM_TOT_CHAN_NUM_MASK)
            >> Self::TX_TDM_TOT_CHAN_NUM_OFFSET
    }

    /// Write **TX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub fn write_tx_tdm_tot_chan_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_TDM_TOT_CHAN_NUM_MASK)
                | ((value << Self::TX_TDM_TOT_CHAN_NUM_OFFSET) & Self::TX_TDM_TOT_CHAN_NUM_MASK)
        });
    }

    /// **TX_TDM_CHAN15_EN** `[15]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan15_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN15_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan15_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN15_EN_MASK);
    }

    /// Clear TX_TDM_CHAN15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan15_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN15_EN_MASK);
    }

    /// **TX_TDM_CHAN14_EN** `[14]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan14_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN14_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan14_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN14_EN_MASK);
    }

    /// Clear TX_TDM_CHAN14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan14_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN14_EN_MASK);
    }

    /// **TX_TDM_CHAN13_EN** `[13]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan13_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN13_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan13_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN13_EN_MASK);
    }

    /// Clear TX_TDM_CHAN13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan13_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN13_EN_MASK);
    }

    /// **TX_TDM_CHAN12_EN** `[12]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan12_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN12_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan12_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN12_EN_MASK);
    }

    /// Clear TX_TDM_CHAN12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan12_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN12_EN_MASK);
    }

    /// **TX_TDM_CHAN11_EN** `[11]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan11_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN11_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan11_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN11_EN_MASK);
    }

    /// Clear TX_TDM_CHAN11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan11_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN11_EN_MASK);
    }

    /// **TX_TDM_CHAN10_EN** `[10]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan10_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN10_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan10_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN10_EN_MASK);
    }

    /// Clear TX_TDM_CHAN10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan10_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN10_EN_MASK);
    }

    /// **TX_TDM_CHAN9_EN** `[9]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan9_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN9_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan9_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN9_EN_MASK);
    }

    /// Clear TX_TDM_CHAN9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan9_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN9_EN_MASK);
    }

    /// **TX_TDM_CHAN8_EN** `[8]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan8_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN8_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan8_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN8_EN_MASK);
    }

    /// Clear TX_TDM_CHAN8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan8_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN8_EN_MASK);
    }

    /// **TX_TDM_CHAN7_EN** `[7]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan7_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN7_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan7_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN7_EN_MASK);
    }

    /// Clear TX_TDM_CHAN7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan7_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN7_EN_MASK);
    }

    /// **TX_TDM_CHAN6_EN** `[6]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan6_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN6_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan6_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN6_EN_MASK);
    }

    /// Clear TX_TDM_CHAN6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan6_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN6_EN_MASK);
    }

    /// **TX_TDM_CHAN5_EN** `[5]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan5_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN5_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan5_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN5_EN_MASK);
    }

    /// Clear TX_TDM_CHAN5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan5_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN5_EN_MASK);
    }

    /// **TX_TDM_CHAN4_EN** `[4]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan4_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN4_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan4_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN4_EN_MASK);
    }

    /// Clear TX_TDM_CHAN4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan4_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN4_EN_MASK);
    }

    /// **TX_TDM_CHAN3_EN** `[3]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan3_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN3_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan3_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN3_EN_MASK);
    }

    /// Clear TX_TDM_CHAN3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan3_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN3_EN_MASK);
    }

    /// **TX_TDM_CHAN2_EN** `[2]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan2_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN2_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan2_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN2_EN_MASK);
    }

    /// Clear TX_TDM_CHAN2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan2_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN2_EN_MASK);
    }

    /// **TX_TDM_CHAN1_EN** `[1]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan1_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN1_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan1_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN1_EN_MASK);
    }

    /// Clear TX_TDM_CHAN1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan1_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN1_EN_MASK);
    }

    /// **TX_TDM_CHAN0_EN** `[0]` (RW)
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub fn tx_tdm_chan0_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TDM_CHAN0_EN_MASK) != 0
    }

    /// Set TX_TDM_CHAN0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_tdm_chan0_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TDM_CHAN0_EN_MASK);
    }

    /// Clear TX_TDM_CHAN0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_tdm_chan0_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_TDM_CHAN0_EN_MASK);
    }
}

/// TX_TDM_CTRL value type for batch read/modify/write operations
///
/// Use `txtdmctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txtdmctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxTdmCtrlVal {
    bits: u32,
}

impl TxTdmCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **TX_TDM_SKIP_MSK_EN** `[20:20]`
    ///
    /// When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels.
    #[inline(always)]
    pub const fn tx_tdm_skip_msk_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_SKIP_MSK_EN_MASK) != 0
    }

    /// Read **TX_TDM_TOT_CHAN_NUM** `[19:16]`
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub const fn tx_tdm_tot_chan_num(&self) -> u32 {
        (self.bits & TxTdmCtrl::TX_TDM_TOT_CHAN_NUM_MASK) >> TxTdmCtrl::TX_TDM_TOT_CHAN_NUM_OFFSET
    }

    /// Read **TX_TDM_CHAN15_EN** `[15:15]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan15_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN15_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN14_EN** `[14:14]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan14_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN14_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN13_EN** `[13:13]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan13_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN13_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN12_EN** `[12:12]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan12_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN12_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN11_EN** `[11:11]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan11_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN11_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN10_EN** `[10:10]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan10_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN10_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN9_EN** `[9:9]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan9_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN9_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN8_EN** `[8:8]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan8_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN8_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN7_EN** `[7:7]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan7_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN7_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN6_EN** `[6:6]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan6_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN6_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN5_EN** `[5:5]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan5_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN5_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN4_EN** `[4:4]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan4_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN4_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN3_EN** `[3:3]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan3_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN3_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN2_EN** `[2:2]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan2_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN2_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN1_EN** `[1:1]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan1_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN1_EN_MASK) != 0
    }

    /// Read **TX_TDM_CHAN0_EN** `[0:0]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn tx_tdm_chan0_en(&self) -> bool {
        (self.bits & TxTdmCtrl::TX_TDM_CHAN0_EN_MASK) != 0
    }

    /// Set **TX_TDM_SKIP_MSK_EN** `[20:20]`
    ///
    /// When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels.
    #[inline(always)]
    pub const fn with_tx_tdm_skip_msk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_SKIP_MSK_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_SKIP_MSK_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_TOT_CHAN_NUM** `[19:16]`
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub const fn with_tx_tdm_tot_chan_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTdmCtrl::TX_TDM_TOT_CHAN_NUM_MASK)
                | ((value << TxTdmCtrl::TX_TDM_TOT_CHAN_NUM_OFFSET)
                    & TxTdmCtrl::TX_TDM_TOT_CHAN_NUM_MASK),
        }
    }

    /// Set **TX_TDM_CHAN15_EN** `[15:15]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan15_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN15_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN15_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN14_EN** `[14:14]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan14_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN14_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN14_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN13_EN** `[13:13]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan13_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN13_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN13_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN12_EN** `[12:12]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan12_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN12_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN12_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN11_EN** `[11:11]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan11_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN11_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN11_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN10_EN** `[10:10]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan10_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN10_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN10_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN9_EN** `[9:9]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan9_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN9_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN9_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN8_EN** `[8:8]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan8_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN8_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN8_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN7_EN** `[7:7]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN7_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN7_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN6_EN** `[6:6]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN6_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN6_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN5_EN** `[5:5]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN5_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN5_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN4_EN** `[4:4]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN4_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN4_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN3_EN** `[3:3]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN3_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN3_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN2_EN** `[2:2]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN2_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN2_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN1_EN** `[1:1]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN1_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN1_EN_MASK
            },
        }
    }

    /// Set **TX_TDM_CHAN0_EN** `[0:0]`
    ///
    /// 1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel.
    #[inline(always)]
    pub const fn with_tx_tdm_chan0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxTdmCtrl::TX_TDM_CHAN0_EN_MASK
            } else {
                self.bits & !TxTdmCtrl::TX_TDM_CHAN0_EN_MASK
            },
        }
    }
}

impl Default for TxTdmCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxTdmCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxTdmCtrlVal {
        TxTdmCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxTdmCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxTdmCtrlVal) -> TxTdmCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_TIMING Register
///
/// I2S RX timing control register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **RX_BCK_IN_DM** `[29:28]` (RW)
///
///   The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_WS_IN_DM** `[25:24]` (RW)
///
///   The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_BCK_OUT_DM** `[21:20]` (RW)
///
///   The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_WS_OUT_DM** `[17:16]` (RW)
///
///   The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_SD3_IN_DM** `[13:12]` (RW)
///
///   The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_SD2_IN_DM** `[9:8]` (RW)
///
///   The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_SD1_IN_DM** `[5:4]` (RW)
///
///   The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **RX_SD_IN_DM** `[1:0]` (RW)
///
///   The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
#[repr(transparent)]
pub struct RxTiming {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxTiming {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxTiming {
    // ======== Field Constants ========

    /// **RX_BCK_IN_DM**
    ///
    /// The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const RX_BCK_IN_DM_OFFSET: u32 = 28;
    pub const RX_BCK_IN_DM_MASK: u32 = 0x30000000;

    /// **RX_WS_IN_DM**
    ///
    /// The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const RX_WS_IN_DM_OFFSET: u32 = 24;
    pub const RX_WS_IN_DM_MASK: u32 = 0x03000000;

    /// **RX_BCK_OUT_DM**
    ///
    /// The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const RX_BCK_OUT_DM_OFFSET: u32 = 20;
    pub const RX_BCK_OUT_DM_MASK: u32 = 0x00300000;

    /// **RX_WS_OUT_DM**
    ///
    /// The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const RX_WS_OUT_DM_OFFSET: u32 = 16;
    pub const RX_WS_OUT_DM_MASK: u32 = 0x00030000;

    /// **RX_SD3_IN_DM**
    ///
    /// The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const RX_SD3_IN_DM_OFFSET: u32 = 12;
    pub const RX_SD3_IN_DM_MASK: u32 = 0x00003000;

    /// **RX_SD2_IN_DM**
    ///
    /// The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const RX_SD2_IN_DM_OFFSET: u32 = 8;
    pub const RX_SD2_IN_DM_MASK: u32 = 0x00000300;

    /// **RX_SD1_IN_DM**
    ///
    /// The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const RX_SD1_IN_DM_OFFSET: u32 = 4;
    pub const RX_SD1_IN_DM_MASK: u32 = 0x00000030;

    /// **RX_SD_IN_DM**
    ///
    /// The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const RX_SD_IN_DM_OFFSET: u32 = 0;
    pub const RX_SD_IN_DM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **RX_BCK_IN_DM** `[29:28]` (RW)
    ///
    /// The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_bck_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BCK_IN_DM_MASK) >> Self::RX_BCK_IN_DM_OFFSET
    }

    /// Write **RX_BCK_IN_DM** `[29:28]` (RW)
    ///
    /// The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_bck_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BCK_IN_DM_MASK)
                | ((value << Self::RX_BCK_IN_DM_OFFSET) & Self::RX_BCK_IN_DM_MASK)
        });
    }

    /// **RX_WS_IN_DM** `[25:24]` (RW)
    ///
    /// The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_ws_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_WS_IN_DM_MASK) >> Self::RX_WS_IN_DM_OFFSET
    }

    /// Write **RX_WS_IN_DM** `[25:24]` (RW)
    ///
    /// The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_ws_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_WS_IN_DM_MASK)
                | ((value << Self::RX_WS_IN_DM_OFFSET) & Self::RX_WS_IN_DM_MASK)
        });
    }

    /// **RX_BCK_OUT_DM** `[21:20]` (RW)
    ///
    /// The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_bck_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BCK_OUT_DM_MASK) >> Self::RX_BCK_OUT_DM_OFFSET
    }

    /// Write **RX_BCK_OUT_DM** `[21:20]` (RW)
    ///
    /// The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_bck_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BCK_OUT_DM_MASK)
                | ((value << Self::RX_BCK_OUT_DM_OFFSET) & Self::RX_BCK_OUT_DM_MASK)
        });
    }

    /// **RX_WS_OUT_DM** `[17:16]` (RW)
    ///
    /// The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_ws_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_WS_OUT_DM_MASK) >> Self::RX_WS_OUT_DM_OFFSET
    }

    /// Write **RX_WS_OUT_DM** `[17:16]` (RW)
    ///
    /// The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_ws_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_WS_OUT_DM_MASK)
                | ((value << Self::RX_WS_OUT_DM_OFFSET) & Self::RX_WS_OUT_DM_MASK)
        });
    }

    /// **RX_SD3_IN_DM** `[13:12]` (RW)
    ///
    /// The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_sd3_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SD3_IN_DM_MASK) >> Self::RX_SD3_IN_DM_OFFSET
    }

    /// Write **RX_SD3_IN_DM** `[13:12]` (RW)
    ///
    /// The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_sd3_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_SD3_IN_DM_MASK)
                | ((value << Self::RX_SD3_IN_DM_OFFSET) & Self::RX_SD3_IN_DM_MASK)
        });
    }

    /// **RX_SD2_IN_DM** `[9:8]` (RW)
    ///
    /// The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_sd2_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SD2_IN_DM_MASK) >> Self::RX_SD2_IN_DM_OFFSET
    }

    /// Write **RX_SD2_IN_DM** `[9:8]` (RW)
    ///
    /// The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_sd2_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_SD2_IN_DM_MASK)
                | ((value << Self::RX_SD2_IN_DM_OFFSET) & Self::RX_SD2_IN_DM_MASK)
        });
    }

    /// **RX_SD1_IN_DM** `[5:4]` (RW)
    ///
    /// The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_sd1_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SD1_IN_DM_MASK) >> Self::RX_SD1_IN_DM_OFFSET
    }

    /// Write **RX_SD1_IN_DM** `[5:4]` (RW)
    ///
    /// The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_sd1_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_SD1_IN_DM_MASK)
                | ((value << Self::RX_SD1_IN_DM_OFFSET) & Self::RX_SD1_IN_DM_MASK)
        });
    }

    /// **RX_SD_IN_DM** `[1:0]` (RW)
    ///
    /// The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn rx_sd_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SD_IN_DM_MASK) >> Self::RX_SD_IN_DM_OFFSET
    }

    /// Write **RX_SD_IN_DM** `[1:0]` (RW)
    ///
    /// The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_rx_sd_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_SD_IN_DM_MASK)
                | ((value << Self::RX_SD_IN_DM_OFFSET) & Self::RX_SD_IN_DM_MASK)
        });
    }
}

/// RX_TIMING value type for batch read/modify/write operations
///
/// Use `rxtiming.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxtiming.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxTimingVal {
    bits: u32,
}

impl RxTimingVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_BCK_IN_DM** `[29:28]`
    ///
    /// The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_bck_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_BCK_IN_DM_MASK) >> RxTiming::RX_BCK_IN_DM_OFFSET
    }

    /// Read **RX_WS_IN_DM** `[25:24]`
    ///
    /// The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_ws_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_WS_IN_DM_MASK) >> RxTiming::RX_WS_IN_DM_OFFSET
    }

    /// Read **RX_BCK_OUT_DM** `[21:20]`
    ///
    /// The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_bck_out_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_BCK_OUT_DM_MASK) >> RxTiming::RX_BCK_OUT_DM_OFFSET
    }

    /// Read **RX_WS_OUT_DM** `[17:16]`
    ///
    /// The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_ws_out_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_WS_OUT_DM_MASK) >> RxTiming::RX_WS_OUT_DM_OFFSET
    }

    /// Read **RX_SD3_IN_DM** `[13:12]`
    ///
    /// The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_sd3_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_SD3_IN_DM_MASK) >> RxTiming::RX_SD3_IN_DM_OFFSET
    }

    /// Read **RX_SD2_IN_DM** `[9:8]`
    ///
    /// The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_sd2_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_SD2_IN_DM_MASK) >> RxTiming::RX_SD2_IN_DM_OFFSET
    }

    /// Read **RX_SD1_IN_DM** `[5:4]`
    ///
    /// The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_sd1_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_SD1_IN_DM_MASK) >> RxTiming::RX_SD1_IN_DM_OFFSET
    }

    /// Read **RX_SD_IN_DM** `[1:0]`
    ///
    /// The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn rx_sd_in_dm(&self) -> u32 {
        (self.bits & RxTiming::RX_SD_IN_DM_MASK) >> RxTiming::RX_SD_IN_DM_OFFSET
    }

    /// Set **RX_BCK_IN_DM** `[29:28]`
    ///
    /// The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_bck_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_BCK_IN_DM_MASK)
                | ((value << RxTiming::RX_BCK_IN_DM_OFFSET) & RxTiming::RX_BCK_IN_DM_MASK),
        }
    }

    /// Set **RX_WS_IN_DM** `[25:24]`
    ///
    /// The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_ws_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_WS_IN_DM_MASK)
                | ((value << RxTiming::RX_WS_IN_DM_OFFSET) & RxTiming::RX_WS_IN_DM_MASK),
        }
    }

    /// Set **RX_BCK_OUT_DM** `[21:20]`
    ///
    /// The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_bck_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_BCK_OUT_DM_MASK)
                | ((value << RxTiming::RX_BCK_OUT_DM_OFFSET) & RxTiming::RX_BCK_OUT_DM_MASK),
        }
    }

    /// Set **RX_WS_OUT_DM** `[17:16]`
    ///
    /// The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_ws_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_WS_OUT_DM_MASK)
                | ((value << RxTiming::RX_WS_OUT_DM_OFFSET) & RxTiming::RX_WS_OUT_DM_MASK),
        }
    }

    /// Set **RX_SD3_IN_DM** `[13:12]`
    ///
    /// The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_sd3_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_SD3_IN_DM_MASK)
                | ((value << RxTiming::RX_SD3_IN_DM_OFFSET) & RxTiming::RX_SD3_IN_DM_MASK),
        }
    }

    /// Set **RX_SD2_IN_DM** `[9:8]`
    ///
    /// The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_sd2_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_SD2_IN_DM_MASK)
                | ((value << RxTiming::RX_SD2_IN_DM_OFFSET) & RxTiming::RX_SD2_IN_DM_MASK),
        }
    }

    /// Set **RX_SD1_IN_DM** `[5:4]`
    ///
    /// The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_sd1_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_SD1_IN_DM_MASK)
                | ((value << RxTiming::RX_SD1_IN_DM_OFFSET) & RxTiming::RX_SD1_IN_DM_MASK),
        }
    }

    /// Set **RX_SD_IN_DM** `[1:0]`
    ///
    /// The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_rx_sd_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTiming::RX_SD_IN_DM_MASK)
                | ((value << RxTiming::RX_SD_IN_DM_OFFSET) & RxTiming::RX_SD_IN_DM_MASK),
        }
    }
}

impl Default for RxTimingVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxTiming {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxTimingVal {
        RxTimingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxTimingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxTimingVal) -> RxTimingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_TIMING Register
///
/// I2S TX timing control register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **TX_BCK_IN_DM** `[29:28]` (RW)
///
///   The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **TX_WS_IN_DM** `[25:24]` (RW)
///
///   The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **TX_BCK_OUT_DM** `[21:20]` (RW)
///
///   The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **TX_WS_OUT_DM** `[17:16]` (RW)
///
///   The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **TX_SD1_OUT_DM** `[5:4]` (RW)
///
///   The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
///
/// - **TX_SD_OUT_DM** `[1:0]` (RW)
///
///   The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
#[repr(transparent)]
pub struct TxTiming {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxTiming {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxTiming {
    // ======== Field Constants ========

    /// **TX_BCK_IN_DM**
    ///
    /// The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const TX_BCK_IN_DM_OFFSET: u32 = 28;
    pub const TX_BCK_IN_DM_MASK: u32 = 0x30000000;

    /// **TX_WS_IN_DM**
    ///
    /// The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const TX_WS_IN_DM_OFFSET: u32 = 24;
    pub const TX_WS_IN_DM_MASK: u32 = 0x03000000;

    /// **TX_BCK_OUT_DM**
    ///
    /// The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const TX_BCK_OUT_DM_OFFSET: u32 = 20;
    pub const TX_BCK_OUT_DM_MASK: u32 = 0x00300000;

    /// **TX_WS_OUT_DM**
    ///
    /// The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const TX_WS_OUT_DM_OFFSET: u32 = 16;
    pub const TX_WS_OUT_DM_MASK: u32 = 0x00030000;

    /// **TX_SD1_OUT_DM**
    ///
    /// The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const TX_SD1_OUT_DM_OFFSET: u32 = 4;
    pub const TX_SD1_OUT_DM_MASK: u32 = 0x00000030;

    /// **TX_SD_OUT_DM**
    ///
    /// The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TX_SD_OUT_DM_OFFSET: u32 = 0;
    pub const TX_SD_OUT_DM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_BCK_IN_DM** `[29:28]` (RW)
    ///
    /// The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_bck_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BCK_IN_DM_MASK) >> Self::TX_BCK_IN_DM_OFFSET
    }

    /// Write **TX_BCK_IN_DM** `[29:28]` (RW)
    ///
    /// The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_bck_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BCK_IN_DM_MASK)
                | ((value << Self::TX_BCK_IN_DM_OFFSET) & Self::TX_BCK_IN_DM_MASK)
        });
    }

    /// **TX_WS_IN_DM** `[25:24]` (RW)
    ///
    /// The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_ws_in_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_WS_IN_DM_MASK) >> Self::TX_WS_IN_DM_OFFSET
    }

    /// Write **TX_WS_IN_DM** `[25:24]` (RW)
    ///
    /// The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_ws_in_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_WS_IN_DM_MASK)
                | ((value << Self::TX_WS_IN_DM_OFFSET) & Self::TX_WS_IN_DM_MASK)
        });
    }

    /// **TX_BCK_OUT_DM** `[21:20]` (RW)
    ///
    /// The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_bck_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BCK_OUT_DM_MASK) >> Self::TX_BCK_OUT_DM_OFFSET
    }

    /// Write **TX_BCK_OUT_DM** `[21:20]` (RW)
    ///
    /// The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_bck_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BCK_OUT_DM_MASK)
                | ((value << Self::TX_BCK_OUT_DM_OFFSET) & Self::TX_BCK_OUT_DM_MASK)
        });
    }

    /// **TX_WS_OUT_DM** `[17:16]` (RW)
    ///
    /// The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_ws_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_WS_OUT_DM_MASK) >> Self::TX_WS_OUT_DM_OFFSET
    }

    /// Write **TX_WS_OUT_DM** `[17:16]` (RW)
    ///
    /// The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_ws_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_WS_OUT_DM_MASK)
                | ((value << Self::TX_WS_OUT_DM_OFFSET) & Self::TX_WS_OUT_DM_MASK)
        });
    }

    /// **TX_SD1_OUT_DM** `[5:4]` (RW)
    ///
    /// The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_sd1_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SD1_OUT_DM_MASK) >> Self::TX_SD1_OUT_DM_OFFSET
    }

    /// Write **TX_SD1_OUT_DM** `[5:4]` (RW)
    ///
    /// The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_sd1_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_SD1_OUT_DM_MASK)
                | ((value << Self::TX_SD1_OUT_DM_OFFSET) & Self::TX_SD1_OUT_DM_MASK)
        });
    }

    /// **TX_SD_OUT_DM** `[1:0]` (RW)
    ///
    /// The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn tx_sd_out_dm(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SD_OUT_DM_MASK) >> Self::TX_SD_OUT_DM_OFFSET
    }

    /// Write **TX_SD_OUT_DM** `[1:0]` (RW)
    ///
    /// The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub fn write_tx_sd_out_dm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_SD_OUT_DM_MASK)
                | ((value << Self::TX_SD_OUT_DM_OFFSET) & Self::TX_SD_OUT_DM_MASK)
        });
    }
}

/// TX_TIMING value type for batch read/modify/write operations
///
/// Use `txtiming.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txtiming.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxTimingVal {
    bits: u32,
}

impl TxTimingVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_BCK_IN_DM** `[29:28]`
    ///
    /// The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_bck_in_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_BCK_IN_DM_MASK) >> TxTiming::TX_BCK_IN_DM_OFFSET
    }

    /// Read **TX_WS_IN_DM** `[25:24]`
    ///
    /// The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_ws_in_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_WS_IN_DM_MASK) >> TxTiming::TX_WS_IN_DM_OFFSET
    }

    /// Read **TX_BCK_OUT_DM** `[21:20]`
    ///
    /// The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_bck_out_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_BCK_OUT_DM_MASK) >> TxTiming::TX_BCK_OUT_DM_OFFSET
    }

    /// Read **TX_WS_OUT_DM** `[17:16]`
    ///
    /// The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_ws_out_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_WS_OUT_DM_MASK) >> TxTiming::TX_WS_OUT_DM_OFFSET
    }

    /// Read **TX_SD1_OUT_DM** `[5:4]`
    ///
    /// The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_sd1_out_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_SD1_OUT_DM_MASK) >> TxTiming::TX_SD1_OUT_DM_OFFSET
    }

    /// Read **TX_SD_OUT_DM** `[1:0]`
    ///
    /// The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn tx_sd_out_dm(&self) -> u32 {
        (self.bits & TxTiming::TX_SD_OUT_DM_MASK) >> TxTiming::TX_SD_OUT_DM_OFFSET
    }

    /// Set **TX_BCK_IN_DM** `[29:28]`
    ///
    /// The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_bck_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_BCK_IN_DM_MASK)
                | ((value << TxTiming::TX_BCK_IN_DM_OFFSET) & TxTiming::TX_BCK_IN_DM_MASK),
        }
    }

    /// Set **TX_WS_IN_DM** `[25:24]`
    ///
    /// The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_ws_in_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_WS_IN_DM_MASK)
                | ((value << TxTiming::TX_WS_IN_DM_OFFSET) & TxTiming::TX_WS_IN_DM_MASK),
        }
    }

    /// Set **TX_BCK_OUT_DM** `[21:20]`
    ///
    /// The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_bck_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_BCK_OUT_DM_MASK)
                | ((value << TxTiming::TX_BCK_OUT_DM_OFFSET) & TxTiming::TX_BCK_OUT_DM_MASK),
        }
    }

    /// Set **TX_WS_OUT_DM** `[17:16]`
    ///
    /// The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_ws_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_WS_OUT_DM_MASK)
                | ((value << TxTiming::TX_WS_OUT_DM_OFFSET) & TxTiming::TX_WS_OUT_DM_MASK),
        }
    }

    /// Set **TX_SD1_OUT_DM** `[5:4]`
    ///
    /// The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_sd1_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_SD1_OUT_DM_MASK)
                | ((value << TxTiming::TX_SD1_OUT_DM_OFFSET) & TxTiming::TX_SD1_OUT_DM_MASK),
        }
    }

    /// Set **TX_SD_OUT_DM** `[1:0]`
    ///
    /// The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used.
    #[inline(always)]
    pub const fn with_tx_sd_out_dm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTiming::TX_SD_OUT_DM_MASK)
                | ((value << TxTiming::TX_SD_OUT_DM_OFFSET) & TxTiming::TX_SD_OUT_DM_MASK),
        }
    }
}

impl Default for TxTimingVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxTiming {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxTimingVal {
        TxTimingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxTimingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxTimingVal) -> TxTimingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LC_HUNG_CONF Register
///
/// I2S HUNG configure register.
///
/// Address offset: 0x0060
/// Reset value: 0x00000810
///
/// ## Fields
///
/// - **LC_FIFO_TIMEOUT_ENA** `[11]` (RW)
///
///   The enable bit for FIFO timeout
///
/// - **LC_FIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
///
///   The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
///
/// - **LC_FIFO_TIMEOUT** `[7:0]` (RW)
///
///   the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
#[repr(transparent)]
pub struct LcHungConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcHungConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcHungConf {
    // ======== Field Constants ========

    /// **LC_FIFO_TIMEOUT_ENA**
    ///
    /// The enable bit for FIFO timeout
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LC_FIFO_TIMEOUT_ENA_OFFSET: u32 = 11;
    pub const LC_FIFO_TIMEOUT_ENA_MASK: u32 = 0x00000800;
    pub const LC_FIFO_TIMEOUT_ENA: u32 = 1 << 11;

    /// **LC_FIFO_TIMEOUT_SHIFT**
    ///
    /// The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const LC_FIFO_TIMEOUT_SHIFT_OFFSET: u32 = 8;
    pub const LC_FIFO_TIMEOUT_SHIFT_MASK: u32 = 0x00000700;

    /// **LC_FIFO_TIMEOUT**
    ///
    /// the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const LC_FIFO_TIMEOUT_OFFSET: u32 = 0;
    pub const LC_FIFO_TIMEOUT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **LC_FIFO_TIMEOUT_ENA** `[11]` (RW)
    ///
    /// The enable bit for FIFO timeout
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&self) -> bool {
        (self.value.read_bits() & Self::LC_FIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Set LC_FIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lc_fifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LC_FIFO_TIMEOUT_ENA_MASK);
    }

    /// Clear LC_FIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lc_fifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LC_FIFO_TIMEOUT_ENA_MASK);
    }

    /// **LC_FIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
    ///
    /// The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&self) -> u32 {
        (self.value.read_bits() & Self::LC_FIFO_TIMEOUT_SHIFT_MASK)
            >> Self::LC_FIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Write **LC_FIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
    ///
    /// The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
    #[inline(always)]
    pub fn write_lc_fifo_timeout_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LC_FIFO_TIMEOUT_SHIFT_MASK)
                | ((value << Self::LC_FIFO_TIMEOUT_SHIFT_OFFSET) & Self::LC_FIFO_TIMEOUT_SHIFT_MASK)
        });
    }

    /// **LC_FIFO_TIMEOUT** `[7:0]` (RW)
    ///
    /// the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
    #[inline(always)]
    pub fn lc_fifo_timeout(&self) -> u32 {
        (self.value.read_bits() & Self::LC_FIFO_TIMEOUT_MASK) >> Self::LC_FIFO_TIMEOUT_OFFSET
    }

    /// Write **LC_FIFO_TIMEOUT** `[7:0]` (RW)
    ///
    /// the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
    #[inline(always)]
    pub fn write_lc_fifo_timeout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LC_FIFO_TIMEOUT_MASK)
                | ((value << Self::LC_FIFO_TIMEOUT_OFFSET) & Self::LC_FIFO_TIMEOUT_MASK)
        });
    }
}

/// LC_HUNG_CONF value type for batch read/modify/write operations
///
/// Use `lchungconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lchungconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcHungConfVal {
    bits: u32,
}

impl LcHungConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000810 }
    }

    /// Read **LC_FIFO_TIMEOUT_ENA** `[11:11]`
    ///
    /// The enable bit for FIFO timeout
    #[inline(always)]
    pub const fn lc_fifo_timeout_ena(&self) -> bool {
        (self.bits & LcHungConf::LC_FIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Read **LC_FIFO_TIMEOUT_SHIFT** `[10:8]`
    ///
    /// The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
    #[inline(always)]
    pub const fn lc_fifo_timeout_shift(&self) -> u32 {
        (self.bits & LcHungConf::LC_FIFO_TIMEOUT_SHIFT_MASK)
            >> LcHungConf::LC_FIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Read **LC_FIFO_TIMEOUT** `[7:0]`
    ///
    /// the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
    #[inline(always)]
    pub const fn lc_fifo_timeout(&self) -> u32 {
        (self.bits & LcHungConf::LC_FIFO_TIMEOUT_MASK) >> LcHungConf::LC_FIFO_TIMEOUT_OFFSET
    }

    /// Set **LC_FIFO_TIMEOUT_ENA** `[11:11]`
    ///
    /// The enable bit for FIFO timeout
    #[inline(always)]
    pub const fn with_lc_fifo_timeout_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcHungConf::LC_FIFO_TIMEOUT_ENA_MASK
            } else {
                self.bits & !LcHungConf::LC_FIFO_TIMEOUT_ENA_MASK
            },
        }
    }

    /// Set **LC_FIFO_TIMEOUT_SHIFT** `[10:8]`
    ///
    /// The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift
    #[inline(always)]
    pub const fn with_lc_fifo_timeout_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcHungConf::LC_FIFO_TIMEOUT_SHIFT_MASK)
                | ((value << LcHungConf::LC_FIFO_TIMEOUT_SHIFT_OFFSET)
                    & LcHungConf::LC_FIFO_TIMEOUT_SHIFT_MASK),
        }
    }

    /// Set **LC_FIFO_TIMEOUT** `[7:0]`
    ///
    /// the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value
    #[inline(always)]
    pub const fn with_lc_fifo_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcHungConf::LC_FIFO_TIMEOUT_MASK)
                | ((value << LcHungConf::LC_FIFO_TIMEOUT_OFFSET)
                    & LcHungConf::LC_FIFO_TIMEOUT_MASK),
        }
    }
}

impl Default for LcHungConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcHungConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcHungConfVal {
        LcHungConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcHungConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcHungConfVal) -> LcHungConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RXEOF_NUM Register
///
/// I2S RX data number control register.
///
/// Address offset: 0x0064
/// Reset value: 0x00000040
///
/// ## Fields
///
/// - **RX_EOF_NUM** `[11:0]` (RW)
///
///   The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
#[repr(transparent)]
pub struct RxeofNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxeofNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxeofNum {
    // ======== Field Constants ========

    /// **RX_EOF_NUM**
    ///
    /// The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const RX_EOF_NUM_OFFSET: u32 = 0;
    pub const RX_EOF_NUM_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **RX_EOF_NUM** `[11:0]` (RW)
    ///
    /// The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
    #[inline(always)]
    pub fn rx_eof_num(&self) -> u32 {
        (self.value.read_bits() & Self::RX_EOF_NUM_MASK) >> Self::RX_EOF_NUM_OFFSET
    }

    /// Write **RX_EOF_NUM** `[11:0]` (RW)
    ///
    /// The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
    #[inline(always)]
    pub fn write_rx_eof_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_EOF_NUM_MASK)
                | ((value << Self::RX_EOF_NUM_OFFSET) & Self::RX_EOF_NUM_MASK)
        });
    }
}

/// RXEOF_NUM value type for batch read/modify/write operations
///
/// Use `rxeofnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxeofnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxeofNumVal {
    bits: u32,
}

impl RxeofNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000040 }
    }

    /// Read **RX_EOF_NUM** `[11:0]`
    ///
    /// The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
    #[inline(always)]
    pub const fn rx_eof_num(&self) -> u32 {
        (self.bits & RxeofNum::RX_EOF_NUM_MASK) >> RxeofNum::RX_EOF_NUM_OFFSET
    }

    /// Set **RX_EOF_NUM** `[11:0]`
    ///
    /// The receive data bit length is (I2S_RX_BITS_MOD[4:0] + 1) * (REG_RX_EOF_NUM[11:0] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel.
    #[inline(always)]
    pub const fn with_rx_eof_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxeofNum::RX_EOF_NUM_MASK)
                | ((value << RxeofNum::RX_EOF_NUM_OFFSET) & RxeofNum::RX_EOF_NUM_MASK),
        }
    }
}

impl Default for RxeofNumVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxeofNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxeofNumVal {
        RxeofNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxeofNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxeofNumVal) -> RxeofNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONF_SIGLE_DATA Register
///
/// I2S signal data register
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SINGLE_DATA** `[31:0]` (RW)
///
///   The configured constant channel data to be sent out.
#[repr(transparent)]
pub struct ConfSigleData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ConfSigleData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ConfSigleData {
    // ======== Field Constants ========

    /// **SINGLE_DATA**
    ///
    /// The configured constant channel data to be sent out.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SINGLE_DATA_OFFSET: u32 = 0;
    pub const SINGLE_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SINGLE_DATA** `[31:0]` (RW)
    ///
    /// The configured constant channel data to be sent out.
    #[inline(always)]
    pub fn single_data(&self) -> u32 {
        (self.value.read_bits() & Self::SINGLE_DATA_MASK) >> Self::SINGLE_DATA_OFFSET
    }

    /// Write **SINGLE_DATA** `[31:0]` (RW)
    ///
    /// The configured constant channel data to be sent out.
    #[inline(always)]
    pub fn write_single_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SINGLE_DATA_MASK)
                | ((value << Self::SINGLE_DATA_OFFSET) & Self::SINGLE_DATA_MASK)
        });
    }
}

/// CONF_SIGLE_DATA value type for batch read/modify/write operations
///
/// Use `confsigledata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `confsigledata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfSigleDataVal {
    bits: u32,
}

impl ConfSigleDataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SINGLE_DATA** `[31:0]`
    ///
    /// The configured constant channel data to be sent out.
    #[inline(always)]
    pub const fn single_data(&self) -> u32 {
        (self.bits & ConfSigleData::SINGLE_DATA_MASK) >> ConfSigleData::SINGLE_DATA_OFFSET
    }

    /// Set **SINGLE_DATA** `[31:0]`
    ///
    /// The configured constant channel data to be sent out.
    #[inline(always)]
    pub const fn with_single_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ConfSigleData::SINGLE_DATA_MASK)
                | ((value << ConfSigleData::SINGLE_DATA_OFFSET) & ConfSigleData::SINGLE_DATA_MASK),
        }
    }
}

impl Default for ConfSigleDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ConfSigleData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfSigleDataVal {
        ConfSigleDataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfSigleDataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfSigleDataVal) -> ConfSigleDataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATE Register
///
/// I2S TX status register
///
/// Address offset: 0x006c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **TX_IDLE** `[0]` (RO)
///
///   1: i2s_tx is idle state. 0: i2s_tx is working.
#[repr(transparent)]
pub struct State {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for State {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl State {
    // ======== Field Constants ========

    /// **TX_IDLE**
    ///
    /// 1: i2s_tx is idle state. 0: i2s_tx is working.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_IDLE_OFFSET: u32 = 0;
    pub const TX_IDLE_MASK: u32 = 0x00000001;
    pub const TX_IDLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_IDLE** `[0]` (RO)
    ///
    /// 1: i2s_tx is idle state. 0: i2s_tx is working.
    #[inline(always)]
    pub fn tx_idle(&self) -> bool {
        (self.value.read_bits() & Self::TX_IDLE_MASK) != 0
    }
}

/// STATE value type for batch read/modify/write operations
///
/// Use `state.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `state.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StateVal {
    bits: u32,
}

impl StateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000001 }
    }

    /// Read **TX_IDLE** `[0:0]`
    ///
    /// 1: i2s_tx is idle state. 0: i2s_tx is working.
    #[inline(always)]
    pub const fn tx_idle(&self) -> bool {
        (self.bits & State::TX_IDLE_MASK) != 0
    }
}

impl Default for StateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl State {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StateVal {
        StateVal::from_bits(self.value.read_bits())
    }
}

/// ETM_CONF Register
///
/// I2S ETM configure register
///
/// Address offset: 0x0070
/// Reset value: 0x00010040
///
/// ## Fields
///
/// - **ETM_RX_RECEIVE_WORD_NUM** `[19:10]` (RW)
///
///   I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
///
/// - **ETM_TX_SEND_WORD_NUM** `[9:0]` (RW)
///
///   I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
#[repr(transparent)]
pub struct EtmConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmConf {
    // ======== Field Constants ========

    /// **ETM_RX_RECEIVE_WORD_NUM**
    ///
    /// I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const ETM_RX_RECEIVE_WORD_NUM_OFFSET: u32 = 10;
    pub const ETM_RX_RECEIVE_WORD_NUM_MASK: u32 = 0x000ffc00;

    /// **ETM_TX_SEND_WORD_NUM**
    ///
    /// I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const ETM_TX_SEND_WORD_NUM_OFFSET: u32 = 0;
    pub const ETM_TX_SEND_WORD_NUM_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **ETM_RX_RECEIVE_WORD_NUM** `[19:10]` (RW)
    ///
    /// I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub fn etm_rx_receive_word_num(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_RX_RECEIVE_WORD_NUM_MASK)
            >> Self::ETM_RX_RECEIVE_WORD_NUM_OFFSET
    }

    /// Write **ETM_RX_RECEIVE_WORD_NUM** `[19:10]` (RW)
    ///
    /// I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub fn write_etm_rx_receive_word_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_RX_RECEIVE_WORD_NUM_MASK)
                | ((value << Self::ETM_RX_RECEIVE_WORD_NUM_OFFSET)
                    & Self::ETM_RX_RECEIVE_WORD_NUM_MASK)
        });
    }

    /// **ETM_TX_SEND_WORD_NUM** `[9:0]` (RW)
    ///
    /// I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub fn etm_tx_send_word_num(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TX_SEND_WORD_NUM_MASK)
            >> Self::ETM_TX_SEND_WORD_NUM_OFFSET
    }

    /// Write **ETM_TX_SEND_WORD_NUM** `[9:0]` (RW)
    ///
    /// I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub fn write_etm_tx_send_word_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TX_SEND_WORD_NUM_MASK)
                | ((value << Self::ETM_TX_SEND_WORD_NUM_OFFSET) & Self::ETM_TX_SEND_WORD_NUM_MASK)
        });
    }
}

/// ETM_CONF value type for batch read/modify/write operations
///
/// Use `etmconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmConfVal {
    bits: u32,
}

impl EtmConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00010040 }
    }

    /// Read **ETM_RX_RECEIVE_WORD_NUM** `[19:10]`
    ///
    /// I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub const fn etm_rx_receive_word_num(&self) -> u32 {
        (self.bits & EtmConf::ETM_RX_RECEIVE_WORD_NUM_MASK)
            >> EtmConf::ETM_RX_RECEIVE_WORD_NUM_OFFSET
    }

    /// Read **ETM_TX_SEND_WORD_NUM** `[9:0]`
    ///
    /// I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub const fn etm_tx_send_word_num(&self) -> u32 {
        (self.bits & EtmConf::ETM_TX_SEND_WORD_NUM_MASK) >> EtmConf::ETM_TX_SEND_WORD_NUM_OFFSET
    }

    /// Set **ETM_RX_RECEIVE_WORD_NUM** `[19:10]`
    ///
    /// I2S ETM receive x words event. When receiving word number of reg_etm_rx_receive_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub const fn with_etm_rx_receive_word_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmConf::ETM_RX_RECEIVE_WORD_NUM_MASK)
                | ((value << EtmConf::ETM_RX_RECEIVE_WORD_NUM_OFFSET)
                    & EtmConf::ETM_RX_RECEIVE_WORD_NUM_MASK),
        }
    }

    /// Set **ETM_TX_SEND_WORD_NUM** `[9:0]`
    ///
    /// I2S ETM send x words event. When sending word number of reg_etm_tx_send_word_num[9:0], i2s will trigger an etm event.
    #[inline(always)]
    pub const fn with_etm_tx_send_word_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmConf::ETM_TX_SEND_WORD_NUM_MASK)
                | ((value << EtmConf::ETM_TX_SEND_WORD_NUM_OFFSET)
                    & EtmConf::ETM_TX_SEND_WORD_NUM_MASK),
        }
    }
}

impl Default for EtmConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EtmConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmConfVal {
        EtmConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmConfVal) -> EtmConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FIFO_CNT Register
///
/// I2S sync counter register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **TX_FIFO_CNT_RST** `[31]` (WO)
///
///   Set this bit to reset tx fifo counter.
///
/// - **TX_FIFO_CNT** `[30:0]` (RO)
///
///   tx fifo counter value.
#[repr(transparent)]
pub struct FifoCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoCnt {
    // ======== Field Constants ========

    /// **TX_FIFO_CNT_RST**
    ///
    /// Set this bit to reset tx fifo counter.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_FIFO_CNT_RST_OFFSET: u32 = 31;
    pub const TX_FIFO_CNT_RST_MASK: u32 = 0x80000000;
    pub const TX_FIFO_CNT_RST: u32 = 1 << 31;

    /// **TX_FIFO_CNT**
    ///
    /// tx fifo counter value.
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const TX_FIFO_CNT_OFFSET: u32 = 0;
    pub const TX_FIFO_CNT_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// Trigger **TX_FIFO_CNT_RST** (WO)
    ///
    /// Set this bit to reset tx fifo counter.
    #[inline(always)]
    pub fn trigger_tx_fifo_cnt_rst(&self) {
        self.value.modify_bits(|r| r | Self::TX_FIFO_CNT_RST_MASK);
    }

    /// **TX_FIFO_CNT** `[30:0]` (RO)
    ///
    /// tx fifo counter value.
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_FIFO_CNT_MASK) >> Self::TX_FIFO_CNT_OFFSET
    }
}

/// FIFO_CNT value type for batch read/modify/write operations
///
/// Use `fifocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoCntVal {
    bits: u32,
}

impl FifoCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_FIFO_CNT** `[30:0]`
    ///
    /// tx fifo counter value.
    #[inline(always)]
    pub const fn tx_fifo_cnt(&self) -> u32 {
        (self.bits & FifoCnt::TX_FIFO_CNT_MASK) >> FifoCnt::TX_FIFO_CNT_OFFSET
    }

    /// Set **TX_FIFO_CNT_RST** `[31:31]`
    ///
    /// Set this bit to reset tx fifo counter.
    #[inline(always)]
    pub const fn with_tx_fifo_cnt_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoCnt::TX_FIFO_CNT_RST_MASK
            } else {
                self.bits & !FifoCnt::TX_FIFO_CNT_RST_MASK
            },
        }
    }
}

impl Default for FifoCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FifoCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoCntVal {
        FifoCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoCntVal) -> FifoCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BCK_CNT Register
///
/// I2S sync counter register
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **TX_BCK_CNT_RST** `[31]` (WO)
///
///   Set this bit to reset tx bck counter.
///
/// - **TX_BCK_CNT** `[30:0]` (RO)
///
///   tx bck counter value.
#[repr(transparent)]
pub struct BckCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BckCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BckCnt {
    // ======== Field Constants ========

    /// **TX_BCK_CNT_RST**
    ///
    /// Set this bit to reset tx bck counter.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_BCK_CNT_RST_OFFSET: u32 = 31;
    pub const TX_BCK_CNT_RST_MASK: u32 = 0x80000000;
    pub const TX_BCK_CNT_RST: u32 = 1 << 31;

    /// **TX_BCK_CNT**
    ///
    /// tx bck counter value.
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const TX_BCK_CNT_OFFSET: u32 = 0;
    pub const TX_BCK_CNT_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// Trigger **TX_BCK_CNT_RST** (WO)
    ///
    /// Set this bit to reset tx bck counter.
    #[inline(always)]
    pub fn trigger_tx_bck_cnt_rst(&self) {
        self.value.modify_bits(|r| r | Self::TX_BCK_CNT_RST_MASK);
    }

    /// **TX_BCK_CNT** `[30:0]` (RO)
    ///
    /// tx bck counter value.
    #[inline(always)]
    pub fn tx_bck_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BCK_CNT_MASK) >> Self::TX_BCK_CNT_OFFSET
    }
}

/// BCK_CNT value type for batch read/modify/write operations
///
/// Use `bckcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bckcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BckCntVal {
    bits: u32,
}

impl BckCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_BCK_CNT** `[30:0]`
    ///
    /// tx bck counter value.
    #[inline(always)]
    pub const fn tx_bck_cnt(&self) -> u32 {
        (self.bits & BckCnt::TX_BCK_CNT_MASK) >> BckCnt::TX_BCK_CNT_OFFSET
    }

    /// Set **TX_BCK_CNT_RST** `[31:31]`
    ///
    /// Set this bit to reset tx bck counter.
    #[inline(always)]
    pub const fn with_tx_bck_cnt_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BckCnt::TX_BCK_CNT_RST_MASK
            } else {
                self.bits & !BckCnt::TX_BCK_CNT_RST_MASK
            },
        }
    }
}

impl Default for BckCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BckCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BckCntVal {
        BckCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BckCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BckCntVal) -> BckCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GATE Register
///
/// Clock gate register
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   set this bit to enable clock gate
#[repr(transparent)]
pub struct ClkGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGate {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// set this bit to enable clock gate
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// set this bit to enable clock gate
    #[inline(always)]
    pub fn clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CLK_EN_MASK) != 0
    }

    /// Set CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CLK_EN_MASK);
    }

    /// Clear CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CLK_EN_MASK);
    }
}

/// CLK_GATE value type for batch read/modify/write operations
///
/// Use `clkgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGateVal {
    bits: u32,
}

impl ClkGateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_EN** `[0:0]`
    ///
    /// set this bit to enable clock gate
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClkGate::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// set this bit to enable clock gate
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGate::CLK_EN_MASK
            } else {
                self.bits & !ClkGate::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClkGateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGateVal {
        ClkGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGateVal) -> ClkGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Version control register
///
/// Address offset: 0x0080
/// Reset value: 0x02303240
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   I2S version control register
#[repr(transparent)]
pub struct Date {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Date {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Date {
    // ======== Field Constants ========

    /// **DATE**
    ///
    /// I2S version control register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// I2S version control register
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// I2S version control register
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
        });
    }
}

/// DATE value type for batch read/modify/write operations
///
/// Use `date.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `date.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DateVal {
    bits: u32,
}

impl DateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x02303240 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// I2S version control register
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// I2S version control register
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::DATE_MASK) | ((value << Date::DATE_OFFSET) & Date::DATE_MASK),
        }
    }
}

impl Default for DateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Date {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DateVal {
        DateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DateVal) -> DateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12],
    /// 0x000c
    /// I2S interrupt raw register, valid in level.
    pub int_raw: IntRaw,
    /// 0x0010
    /// I2S interrupt status register.
    pub int_st: IntSt,
    /// 0x0014
    /// I2S interrupt enable register.
    pub int_ena: IntEna,
    /// 0x0018
    /// I2S interrupt clear register.
    pub int_clr: IntClr,
    _reserved1: [u8; 4],
    /// 0x0020
    /// I2S RX configure register
    pub rx_conf: RxConf,
    /// 0x0024
    /// I2S TX configure register
    pub tx_conf: TxConf,
    /// 0x0028
    /// I2S RX configure register 1
    pub rx_conf1: RxConf1,
    /// 0x002c
    /// I2S TX configure register 1
    pub tx_conf1: TxConf1,
    _reserved2: [u8; 16],
    /// 0x0040
    /// I2S TX PCM2PDM configuration register
    pub tx_pcm2pdm_conf: TxPcm2pdmConf,
    /// 0x0044
    /// I2S TX PCM2PDM configuration register
    pub tx_pcm2pdm_conf1: TxPcm2pdmConf1,
    /// 0x0048
    /// I2S RX configure register
    pub rx_pdm2pcm_conf: RxPdm2pcmConf,
    _reserved3: [u8; 4],
    /// 0x0050
    /// I2S TX TDM mode control register
    pub rx_tdm_ctrl: RxTdmCtrl,
    /// 0x0054
    /// I2S TX TDM mode control register
    pub tx_tdm_ctrl: TxTdmCtrl,
    /// 0x0058
    /// I2S RX timing control register
    pub rx_timing: RxTiming,
    /// 0x005c
    /// I2S TX timing control register
    pub tx_timing: TxTiming,
    /// 0x0060
    /// I2S HUNG configure register.
    pub lc_hung_conf: LcHungConf,
    /// 0x0064
    /// I2S RX data number control register.
    pub rxeof_num: RxeofNum,
    /// 0x0068
    /// I2S signal data register
    pub conf_sigle_data: ConfSigleData,
    /// 0x006c
    /// I2S TX status register
    pub state: State,
    /// 0x0070
    /// I2S ETM configure register
    pub etm_conf: EtmConf,
    /// 0x0074
    /// I2S sync counter register
    pub fifo_cnt: FifoCnt,
    /// 0x0078
    /// I2S sync counter register
    pub bck_cnt: BckCnt,
    /// 0x007c
    /// Clock gate register
    pub clk_gate: ClkGate,
    /// 0x0080
    /// Version control register
    pub date: Date,
}
