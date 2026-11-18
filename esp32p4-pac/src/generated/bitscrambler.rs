//! BITSCRAMBLER
//! BITSCRAMBLER Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x500a3000;

/// TX_INST_CFG0 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **TX_INST_POS** `[6:3]` (RW)
///
///   write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
///
/// - **TX_INST_IDX** `[2:0]` (RW)
///
///   write this bits to specify the one of 8 instruction
#[repr(transparent)]
pub struct TxInstCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxInstCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxInstCfg0 {
    // ======== Field Constants ========

    /// **TX_INST_POS**
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    ///
    /// Bits `[6:3]`, mask `0x00000078`
    pub const TX_INST_POS_OFFSET: u32 = 3;
    pub const TX_INST_POS_MASK: u32 = 0x00000078;

    /// **TX_INST_IDX**
    ///
    /// write this bits to specify the one of 8 instruction
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const TX_INST_IDX_OFFSET: u32 = 0;
    pub const TX_INST_IDX_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **TX_INST_POS** `[6:3]` (RW)
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub fn tx_inst_pos(&self) -> u32 {
        (self.value.read_bits() & Self::TX_INST_POS_MASK) >> Self::TX_INST_POS_OFFSET
    }

    /// Write **TX_INST_POS** `[6:3]` (RW)
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub fn write_tx_inst_pos(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_INST_POS_MASK)
                | ((value << Self::TX_INST_POS_OFFSET) & Self::TX_INST_POS_MASK)
        });
    }

    /// **TX_INST_IDX** `[2:0]` (RW)
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub fn tx_inst_idx(&self) -> u32 {
        (self.value.read_bits() & Self::TX_INST_IDX_MASK) >> Self::TX_INST_IDX_OFFSET
    }

    /// Write **TX_INST_IDX** `[2:0]` (RW)
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub fn write_tx_inst_idx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_INST_IDX_MASK)
                | ((value << Self::TX_INST_IDX_OFFSET) & Self::TX_INST_IDX_MASK)
        });
    }
}

/// TX_INST_CFG0 value type for batch read/modify/write operations
///
/// Use `txinstcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txinstcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxInstCfg0Val {
    bits: u32,
}

impl TxInstCfg0Val {
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

    /// Read **TX_INST_POS** `[6:3]`
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub const fn tx_inst_pos(&self) -> u32 {
        (self.bits & TxInstCfg0::TX_INST_POS_MASK) >> TxInstCfg0::TX_INST_POS_OFFSET
    }

    /// Read **TX_INST_IDX** `[2:0]`
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub const fn tx_inst_idx(&self) -> u32 {
        (self.bits & TxInstCfg0::TX_INST_IDX_MASK) >> TxInstCfg0::TX_INST_IDX_OFFSET
    }

    /// Set **TX_INST_POS** `[6:3]`
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub const fn with_tx_inst_pos(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxInstCfg0::TX_INST_POS_MASK)
                | ((value << TxInstCfg0::TX_INST_POS_OFFSET) & TxInstCfg0::TX_INST_POS_MASK),
        }
    }

    /// Set **TX_INST_IDX** `[2:0]`
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub const fn with_tx_inst_idx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxInstCfg0::TX_INST_IDX_MASK)
                | ((value << TxInstCfg0::TX_INST_IDX_OFFSET) & TxInstCfg0::TX_INST_IDX_MASK),
        }
    }
}

impl Default for TxInstCfg0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxInstCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxInstCfg0Val {
        TxInstCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxInstCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxInstCfg0Val) -> TxInstCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_INST_CFG1 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0004
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **TX_INST** `[31:0]` (RW)
///
///   write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
#[repr(transparent)]
pub struct TxInstCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxInstCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxInstCfg1 {
    // ======== Field Constants ========

    /// **TX_INST**
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_INST_OFFSET: u32 = 0;
    pub const TX_INST_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_INST** `[31:0]` (RW)
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
    #[inline(always)]
    pub fn tx_inst(&self) -> u32 {
        (self.value.read_bits() & Self::TX_INST_MASK) >> Self::TX_INST_OFFSET
    }

    /// Write **TX_INST** `[31:0]` (RW)
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
    #[inline(always)]
    pub fn write_tx_inst(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_INST_MASK) | ((value << Self::TX_INST_OFFSET) & Self::TX_INST_MASK)
        });
    }
}

/// TX_INST_CFG1 value type for batch read/modify/write operations
///
/// Use `txinstcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txinstcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxInstCfg1Val {
    bits: u32,
}

impl TxInstCfg1Val {
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
        Self { bits: 0x00000004 }
    }

    /// Read **TX_INST** `[31:0]`
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
    #[inline(always)]
    pub const fn tx_inst(&self) -> u32 {
        (self.bits & TxInstCfg1::TX_INST_MASK) >> TxInstCfg1::TX_INST_OFFSET
    }

    /// Set **TX_INST** `[31:0]`
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG
    #[inline(always)]
    pub const fn with_tx_inst(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxInstCfg1::TX_INST_MASK)
                | ((value << TxInstCfg1::TX_INST_OFFSET) & TxInstCfg1::TX_INST_MASK),
        }
    }
}

impl Default for TxInstCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxInstCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxInstCfg1Val {
        TxInstCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxInstCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxInstCfg1Val) -> TxInstCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_INST_CFG0 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **RX_INST_POS** `[6:3]` (RW)
///
///   write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
///
/// - **RX_INST_IDX** `[2:0]` (RW)
///
///   write this bits to specify the one of 8 instruction
#[repr(transparent)]
pub struct RxInstCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxInstCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxInstCfg0 {
    // ======== Field Constants ========

    /// **RX_INST_POS**
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    ///
    /// Bits `[6:3]`, mask `0x00000078`
    pub const RX_INST_POS_OFFSET: u32 = 3;
    pub const RX_INST_POS_MASK: u32 = 0x00000078;

    /// **RX_INST_IDX**
    ///
    /// write this bits to specify the one of 8 instruction
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const RX_INST_IDX_OFFSET: u32 = 0;
    pub const RX_INST_IDX_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **RX_INST_POS** `[6:3]` (RW)
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub fn rx_inst_pos(&self) -> u32 {
        (self.value.read_bits() & Self::RX_INST_POS_MASK) >> Self::RX_INST_POS_OFFSET
    }

    /// Write **RX_INST_POS** `[6:3]` (RW)
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub fn write_rx_inst_pos(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_INST_POS_MASK)
                | ((value << Self::RX_INST_POS_OFFSET) & Self::RX_INST_POS_MASK)
        });
    }

    /// **RX_INST_IDX** `[2:0]` (RW)
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub fn rx_inst_idx(&self) -> u32 {
        (self.value.read_bits() & Self::RX_INST_IDX_MASK) >> Self::RX_INST_IDX_OFFSET
    }

    /// Write **RX_INST_IDX** `[2:0]` (RW)
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub fn write_rx_inst_idx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_INST_IDX_MASK)
                | ((value << Self::RX_INST_IDX_OFFSET) & Self::RX_INST_IDX_MASK)
        });
    }
}

/// RX_INST_CFG0 value type for batch read/modify/write operations
///
/// Use `rxinstcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxinstcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxInstCfg0Val {
    bits: u32,
}

impl RxInstCfg0Val {
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

    /// Read **RX_INST_POS** `[6:3]`
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub const fn rx_inst_pos(&self) -> u32 {
        (self.bits & RxInstCfg0::RX_INST_POS_MASK) >> RxInstCfg0::RX_INST_POS_OFFSET
    }

    /// Read **RX_INST_IDX** `[2:0]`
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub const fn rx_inst_idx(&self) -> u32 {
        (self.bits & RxInstCfg0::RX_INST_IDX_MASK) >> RxInstCfg0::RX_INST_IDX_OFFSET
    }

    /// Set **RX_INST_POS** `[6:3]`
    ///
    /// write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub const fn with_rx_inst_pos(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxInstCfg0::RX_INST_POS_MASK)
                | ((value << RxInstCfg0::RX_INST_POS_OFFSET) & RxInstCfg0::RX_INST_POS_MASK),
        }
    }

    /// Set **RX_INST_IDX** `[2:0]`
    ///
    /// write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub const fn with_rx_inst_idx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxInstCfg0::RX_INST_IDX_MASK)
                | ((value << RxInstCfg0::RX_INST_IDX_OFFSET) & RxInstCfg0::RX_INST_IDX_MASK),
        }
    }
}

impl Default for RxInstCfg0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxInstCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxInstCfg0Val {
        RxInstCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxInstCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxInstCfg0Val) -> RxInstCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_INST_CFG1 Register
///
/// Control and configuration registers
///
/// Address offset: 0x000c
/// Reset value: 0x0000000c
///
/// ## Fields
///
/// - **RX_INST** `[31:0]` (RW)
///
///   write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
#[repr(transparent)]
pub struct RxInstCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxInstCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxInstCfg1 {
    // ======== Field Constants ========

    /// **RX_INST**
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_INST_OFFSET: u32 = 0;
    pub const RX_INST_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_INST** `[31:0]` (RW)
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
    #[inline(always)]
    pub fn rx_inst(&self) -> u32 {
        (self.value.read_bits() & Self::RX_INST_MASK) >> Self::RX_INST_OFFSET
    }

    /// Write **RX_INST** `[31:0]` (RW)
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
    #[inline(always)]
    pub fn write_rx_inst(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_INST_MASK) | ((value << Self::RX_INST_OFFSET) & Self::RX_INST_MASK)
        });
    }
}

/// RX_INST_CFG1 value type for batch read/modify/write operations
///
/// Use `rxinstcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxinstcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxInstCfg1Val {
    bits: u32,
}

impl RxInstCfg1Val {
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
        Self { bits: 0x0000000c }
    }

    /// Read **RX_INST** `[31:0]`
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
    #[inline(always)]
    pub const fn rx_inst(&self) -> u32 {
        (self.bits & RxInstCfg1::RX_INST_MASK) >> RxInstCfg1::RX_INST_OFFSET
    }

    /// Set **RX_INST** `[31:0]`
    ///
    /// write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG
    #[inline(always)]
    pub const fn with_rx_inst(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxInstCfg1::RX_INST_MASK)
                | ((value << RxInstCfg1::RX_INST_OFFSET) & RxInstCfg1::RX_INST_MASK),
        }
    }
}

impl Default for RxInstCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxInstCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxInstCfg1Val {
        RxInstCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxInstCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxInstCfg1Val) -> RxInstCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_LUT_CFG0 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **TX_LUT_MODE** `[12:11]` (RW)
///
///   write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
///
/// - **TX_LUT_IDX** `[10:0]` (RW)
///
///   write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
#[repr(transparent)]
pub struct TxLutCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxLutCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxLutCfg0 {
    // ======== Field Constants ========

    /// **TX_LUT_MODE**
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    ///
    /// Bits `[12:11]`, mask `0x00001800`
    pub const TX_LUT_MODE_OFFSET: u32 = 11;
    pub const TX_LUT_MODE_MASK: u32 = 0x00001800;

    /// **TX_LUT_IDX**
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const TX_LUT_IDX_OFFSET: u32 = 0;
    pub const TX_LUT_IDX_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **TX_LUT_MODE** `[12:11]` (RW)
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub fn tx_lut_mode(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LUT_MODE_MASK) >> Self::TX_LUT_MODE_OFFSET
    }

    /// Write **TX_LUT_MODE** `[12:11]` (RW)
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub fn write_tx_lut_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LUT_MODE_MASK)
                | ((value << Self::TX_LUT_MODE_OFFSET) & Self::TX_LUT_MODE_MASK)
        });
    }

    /// **TX_LUT_IDX** `[10:0]` (RW)
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
    #[inline(always)]
    pub fn tx_lut_idx(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LUT_IDX_MASK) >> Self::TX_LUT_IDX_OFFSET
    }

    /// Write **TX_LUT_IDX** `[10:0]` (RW)
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
    #[inline(always)]
    pub fn write_tx_lut_idx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LUT_IDX_MASK)
                | ((value << Self::TX_LUT_IDX_OFFSET) & Self::TX_LUT_IDX_MASK)
        });
    }
}

/// TX_LUT_CFG0 value type for batch read/modify/write operations
///
/// Use `txlutcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txlutcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxLutCfg0Val {
    bits: u32,
}

impl TxLutCfg0Val {
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

    /// Read **TX_LUT_MODE** `[12:11]`
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub const fn tx_lut_mode(&self) -> u32 {
        (self.bits & TxLutCfg0::TX_LUT_MODE_MASK) >> TxLutCfg0::TX_LUT_MODE_OFFSET
    }

    /// Read **TX_LUT_IDX** `[10:0]`
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
    #[inline(always)]
    pub const fn tx_lut_idx(&self) -> u32 {
        (self.bits & TxLutCfg0::TX_LUT_IDX_MASK) >> TxLutCfg0::TX_LUT_IDX_OFFSET
    }

    /// Set **TX_LUT_MODE** `[12:11]`
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub const fn with_tx_lut_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxLutCfg0::TX_LUT_MODE_MASK)
                | ((value << TxLutCfg0::TX_LUT_MODE_OFFSET) & TxLutCfg0::TX_LUT_MODE_MASK),
        }
    }

    /// Set **TX_LUT_IDX** `[10:0]`
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode
    #[inline(always)]
    pub const fn with_tx_lut_idx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxLutCfg0::TX_LUT_IDX_MASK)
                | ((value << TxLutCfg0::TX_LUT_IDX_OFFSET) & TxLutCfg0::TX_LUT_IDX_MASK),
        }
    }
}

impl Default for TxLutCfg0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxLutCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxLutCfg0Val {
        TxLutCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxLutCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxLutCfg0Val) -> TxLutCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_LUT_CFG1 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0014
/// Reset value: 0x00000014
///
/// ## Fields
///
/// - **TX_LUT** `[31:0]` (RW)
///
///   write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
#[repr(transparent)]
pub struct TxLutCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxLutCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxLutCfg1 {
    // ======== Field Constants ========

    /// **TX_LUT**
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_LUT_OFFSET: u32 = 0;
    pub const TX_LUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_LUT** `[31:0]` (RW)
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    pub fn tx_lut(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LUT_MASK) >> Self::TX_LUT_OFFSET
    }

    /// Write **TX_LUT** `[31:0]` (RW)
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    pub fn write_tx_lut(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LUT_MASK) | ((value << Self::TX_LUT_OFFSET) & Self::TX_LUT_MASK)
        });
    }
}

/// TX_LUT_CFG1 value type for batch read/modify/write operations
///
/// Use `txlutcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txlutcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxLutCfg1Val {
    bits: u32,
}

impl TxLutCfg1Val {
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
        Self { bits: 0x00000014 }
    }

    /// Read **TX_LUT** `[31:0]`
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    pub const fn tx_lut(&self) -> u32 {
        (self.bits & TxLutCfg1::TX_LUT_MASK) >> TxLutCfg1::TX_LUT_OFFSET
    }

    /// Set **TX_LUT** `[31:0]`
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    pub const fn with_tx_lut(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxLutCfg1::TX_LUT_MASK)
                | ((value << TxLutCfg1::TX_LUT_OFFSET) & TxLutCfg1::TX_LUT_MASK),
        }
    }
}

impl Default for TxLutCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxLutCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxLutCfg1Val {
        TxLutCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxLutCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxLutCfg1Val) -> TxLutCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_LUT_CFG0 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **RX_LUT_MODE** `[12:11]` (RW)
///
///   write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
///
/// - **RX_LUT_IDX** `[10:0]` (RW)
///
///   write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
#[repr(transparent)]
pub struct RxLutCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxLutCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxLutCfg0 {
    // ======== Field Constants ========

    /// **RX_LUT_MODE**
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    ///
    /// Bits `[12:11]`, mask `0x00001800`
    pub const RX_LUT_MODE_OFFSET: u32 = 11;
    pub const RX_LUT_MODE_MASK: u32 = 0x00001800;

    /// **RX_LUT_IDX**
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const RX_LUT_IDX_OFFSET: u32 = 0;
    pub const RX_LUT_IDX_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **RX_LUT_MODE** `[12:11]` (RW)
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub fn rx_lut_mode(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LUT_MODE_MASK) >> Self::RX_LUT_MODE_OFFSET
    }

    /// Write **RX_LUT_MODE** `[12:11]` (RW)
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub fn write_rx_lut_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LUT_MODE_MASK)
                | ((value << Self::RX_LUT_MODE_OFFSET) & Self::RX_LUT_MODE_MASK)
        });
    }

    /// **RX_LUT_IDX** `[10:0]` (RW)
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
    #[inline(always)]
    pub fn rx_lut_idx(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LUT_IDX_MASK) >> Self::RX_LUT_IDX_OFFSET
    }

    /// Write **RX_LUT_IDX** `[10:0]` (RW)
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
    #[inline(always)]
    pub fn write_rx_lut_idx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LUT_IDX_MASK)
                | ((value << Self::RX_LUT_IDX_OFFSET) & Self::RX_LUT_IDX_MASK)
        });
    }
}

/// RX_LUT_CFG0 value type for batch read/modify/write operations
///
/// Use `rxlutcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxlutcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxLutCfg0Val {
    bits: u32,
}

impl RxLutCfg0Val {
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

    /// Read **RX_LUT_MODE** `[12:11]`
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub const fn rx_lut_mode(&self) -> u32 {
        (self.bits & RxLutCfg0::RX_LUT_MODE_MASK) >> RxLutCfg0::RX_LUT_MODE_OFFSET
    }

    /// Read **RX_LUT_IDX** `[10:0]`
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
    #[inline(always)]
    pub const fn rx_lut_idx(&self) -> u32 {
        (self.bits & RxLutCfg0::RX_LUT_IDX_MASK) >> RxLutCfg0::RX_LUT_IDX_OFFSET
    }

    /// Set **RX_LUT_MODE** `[12:11]`
    ///
    /// write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes
    #[inline(always)]
    pub const fn with_rx_lut_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxLutCfg0::RX_LUT_MODE_MASK)
                | ((value << RxLutCfg0::RX_LUT_MODE_OFFSET) & RxLutCfg0::RX_LUT_MODE_MASK),
        }
    }

    /// Set **RX_LUT_IDX** `[10:0]`
    ///
    /// write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode
    #[inline(always)]
    pub const fn with_rx_lut_idx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxLutCfg0::RX_LUT_IDX_MASK)
                | ((value << RxLutCfg0::RX_LUT_IDX_OFFSET) & RxLutCfg0::RX_LUT_IDX_MASK),
        }
    }
}

impl Default for RxLutCfg0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxLutCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxLutCfg0Val {
        RxLutCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxLutCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxLutCfg0Val) -> RxLutCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_LUT_CFG1 Register
///
/// Control and configuration registers
///
/// Address offset: 0x001c
/// Reset value: 0x0000001c
///
/// ## Fields
///
/// - **RX_LUT** `[31:0]` (RW)
///
///   write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
#[repr(transparent)]
pub struct RxLutCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxLutCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxLutCfg1 {
    // ======== Field Constants ========

    /// **RX_LUT**
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_LUT_OFFSET: u32 = 0;
    pub const RX_LUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_LUT** `[31:0]` (RW)
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
    #[inline(always)]
    pub fn rx_lut(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LUT_MASK) >> Self::RX_LUT_OFFSET
    }

    /// Write **RX_LUT** `[31:0]` (RW)
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
    #[inline(always)]
    pub fn write_rx_lut(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LUT_MASK) | ((value << Self::RX_LUT_OFFSET) & Self::RX_LUT_MASK)
        });
    }
}

/// RX_LUT_CFG1 value type for batch read/modify/write operations
///
/// Use `rxlutcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxlutcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxLutCfg1Val {
    bits: u32,
}

impl RxLutCfg1Val {
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
        Self { bits: 0x0000001c }
    }

    /// Read **RX_LUT** `[31:0]`
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
    #[inline(always)]
    pub const fn rx_lut(&self) -> u32 {
        (self.bits & RxLutCfg1::RX_LUT_MASK) >> RxLutCfg1::RX_LUT_OFFSET
    }

    /// Set **RX_LUT** `[31:0]`
    ///
    /// write this bits to update LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_RX_LUT_CFG0_REG
    #[inline(always)]
    pub const fn with_rx_lut(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxLutCfg1::RX_LUT_MASK)
                | ((value << RxLutCfg1::RX_LUT_OFFSET) & RxLutCfg1::RX_LUT_MASK),
        }
    }
}

impl Default for RxLutCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxLutCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxLutCfg1Val {
        RxLutCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxLutCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxLutCfg1Val) -> RxLutCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_TAILING_BITS Register
///
/// Control and configuration registers
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **TX_TAILING_BITS** `[15:0]` (RW)
///
///   write this bits to specify the extra data bit length after getting EOF
#[repr(transparent)]
pub struct TxTailingBits {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxTailingBits {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxTailingBits {
    // ======== Field Constants ========

    /// **TX_TAILING_BITS**
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const TX_TAILING_BITS_OFFSET: u32 = 0;
    pub const TX_TAILING_BITS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **TX_TAILING_BITS** `[15:0]` (RW)
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub fn tx_tailing_bits(&self) -> u32 {
        (self.value.read_bits() & Self::TX_TAILING_BITS_MASK) >> Self::TX_TAILING_BITS_OFFSET
    }

    /// Write **TX_TAILING_BITS** `[15:0]` (RW)
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub fn write_tx_tailing_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_TAILING_BITS_MASK)
                | ((value << Self::TX_TAILING_BITS_OFFSET) & Self::TX_TAILING_BITS_MASK)
        });
    }
}

/// TX_TAILING_BITS value type for batch read/modify/write operations
///
/// Use `txtailingbits.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txtailingbits.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxTailingBitsVal {
    bits: u32,
}

impl TxTailingBitsVal {
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

    /// Read **TX_TAILING_BITS** `[15:0]`
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub const fn tx_tailing_bits(&self) -> u32 {
        (self.bits & TxTailingBits::TX_TAILING_BITS_MASK) >> TxTailingBits::TX_TAILING_BITS_OFFSET
    }

    /// Set **TX_TAILING_BITS** `[15:0]`
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub const fn with_tx_tailing_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxTailingBits::TX_TAILING_BITS_MASK)
                | ((value << TxTailingBits::TX_TAILING_BITS_OFFSET)
                    & TxTailingBits::TX_TAILING_BITS_MASK),
        }
    }
}

impl Default for TxTailingBitsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxTailingBits {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxTailingBitsVal {
        TxTailingBitsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxTailingBitsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxTailingBitsVal) -> TxTailingBitsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_TAILING_BITS Register
///
/// Control and configuration registers
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **RX_TAILING_BITS** `[15:0]` (RW)
///
///   write this bits to specify the extra data bit length after getting EOF
#[repr(transparent)]
pub struct RxTailingBits {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxTailingBits {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxTailingBits {
    // ======== Field Constants ========

    /// **RX_TAILING_BITS**
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RX_TAILING_BITS_OFFSET: u32 = 0;
    pub const RX_TAILING_BITS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RX_TAILING_BITS** `[15:0]` (RW)
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub fn rx_tailing_bits(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TAILING_BITS_MASK) >> Self::RX_TAILING_BITS_OFFSET
    }

    /// Write **RX_TAILING_BITS** `[15:0]` (RW)
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub fn write_rx_tailing_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TAILING_BITS_MASK)
                | ((value << Self::RX_TAILING_BITS_OFFSET) & Self::RX_TAILING_BITS_MASK)
        });
    }
}

/// RX_TAILING_BITS value type for batch read/modify/write operations
///
/// Use `rxtailingbits.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxtailingbits.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxTailingBitsVal {
    bits: u32,
}

impl RxTailingBitsVal {
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

    /// Read **RX_TAILING_BITS** `[15:0]`
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub const fn rx_tailing_bits(&self) -> u32 {
        (self.bits & RxTailingBits::RX_TAILING_BITS_MASK) >> RxTailingBits::RX_TAILING_BITS_OFFSET
    }

    /// Set **RX_TAILING_BITS** `[15:0]`
    ///
    /// write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub const fn with_rx_tailing_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxTailingBits::RX_TAILING_BITS_MASK)
                | ((value << RxTailingBits::RX_TAILING_BITS_OFFSET)
                    & RxTailingBits::RX_TAILING_BITS_MASK),
        }
    }
}

impl Default for RxTailingBitsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxTailingBits {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxTailingBitsVal {
        RxTailingBitsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxTailingBitsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxTailingBitsVal) -> RxTailingBitsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CTRL Register
///
/// Control and configuration registers
///
/// Address offset: 0x0028
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **TX_FIFO_RST** `[8]` (WO)
///
///   write this bit to reset the bitscrambler tx fifo
///
/// - **TX_RD_DUMMY** `[7]` (RW)
///
///   write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
///
/// - **TX_HALT_MODE** `[6]` (RW)
///
///   write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
///
/// - **TX_FETCH_MODE** `[5]` (RW)
///
///   write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
///
/// - **TX_COND_MODE** `[4]` (RW)
///
///   write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
///
/// - **TX_EOF_MODE** `[3]` (RW)
///
///   write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
///
/// - **TX_HALT** `[2]` (RW)
///
///   write this bit to halt the bitscrambler tx core
///
/// - **TX_PAUSE** `[1]` (RW)
///
///   write this bit to pause the bitscrambler tx core
///
/// - **TX_ENA** `[0]` (RW)
///
///   write this bit to enable the bitscrambler tx
#[repr(transparent)]
pub struct TxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCtrl {
    // ======== Field Constants ========

    /// **TX_FIFO_RST**
    ///
    /// write this bit to reset the bitscrambler tx fifo
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_FIFO_RST_OFFSET: u32 = 8;
    pub const TX_FIFO_RST_MASK: u32 = 0x00000100;
    pub const TX_FIFO_RST: u32 = 1 << 8;

    /// **TX_RD_DUMMY**
    ///
    /// write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_RD_DUMMY_OFFSET: u32 = 7;
    pub const TX_RD_DUMMY_MASK: u32 = 0x00000080;
    pub const TX_RD_DUMMY: u32 = 1 << 7;

    /// **TX_HALT_MODE**
    ///
    /// write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_HALT_MODE_OFFSET: u32 = 6;
    pub const TX_HALT_MODE_MASK: u32 = 0x00000040;
    pub const TX_HALT_MODE: u32 = 1 << 6;

    /// **TX_FETCH_MODE**
    ///
    /// write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_FETCH_MODE_OFFSET: u32 = 5;
    pub const TX_FETCH_MODE_MASK: u32 = 0x00000020;
    pub const TX_FETCH_MODE: u32 = 1 << 5;

    /// **TX_COND_MODE**
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_COND_MODE_OFFSET: u32 = 4;
    pub const TX_COND_MODE_MASK: u32 = 0x00000010;
    pub const TX_COND_MODE: u32 = 1 << 4;

    /// **TX_EOF_MODE**
    ///
    /// write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_EOF_MODE_OFFSET: u32 = 3;
    pub const TX_EOF_MODE_MASK: u32 = 0x00000008;
    pub const TX_EOF_MODE: u32 = 1 << 3;

    /// **TX_HALT**
    ///
    /// write this bit to halt the bitscrambler tx core
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_HALT_OFFSET: u32 = 2;
    pub const TX_HALT_MASK: u32 = 0x00000004;
    pub const TX_HALT: u32 = 1 << 2;

    /// **TX_PAUSE**
    ///
    /// write this bit to pause the bitscrambler tx core
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_PAUSE_OFFSET: u32 = 1;
    pub const TX_PAUSE_MASK: u32 = 0x00000002;
    pub const TX_PAUSE: u32 = 1 << 1;

    /// **TX_ENA**
    ///
    /// write this bit to enable the bitscrambler tx
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_ENA_OFFSET: u32 = 0;
    pub const TX_ENA_MASK: u32 = 0x00000001;
    pub const TX_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TX_FIFO_RST** (WO)
    ///
    /// write this bit to reset the bitscrambler tx fifo
    #[inline(always)]
    pub fn trigger_tx_fifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::TX_FIFO_RST_MASK);
    }

    /// **TX_RD_DUMMY** `[7]` (RW)
    ///
    /// write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub fn tx_rd_dummy(&self) -> bool {
        (self.value.read_bits() & Self::TX_RD_DUMMY_MASK) != 0
    }

    /// Set TX_RD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_rd_dummy(&self) {
        self.value.modify_bits(|r| r | Self::TX_RD_DUMMY_MASK);
    }

    /// Clear TX_RD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_rd_dummy(&self) {
        self.value.modify_bits(|r| r & !Self::TX_RD_DUMMY_MASK);
    }

    /// **TX_HALT_MODE** `[6]` (RW)
    ///
    /// write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub fn tx_halt_mode(&self) -> bool {
        (self.value.read_bits() & Self::TX_HALT_MODE_MASK) != 0
    }

    /// Set TX_HALT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_halt_mode(&self) {
        self.value.modify_bits(|r| r | Self::TX_HALT_MODE_MASK);
    }

    /// Clear TX_HALT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_halt_mode(&self) {
        self.value.modify_bits(|r| r & !Self::TX_HALT_MODE_MASK);
    }

    /// **TX_FETCH_MODE** `[5]` (RW)
    ///
    /// write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub fn tx_fetch_mode(&self) -> bool {
        (self.value.read_bits() & Self::TX_FETCH_MODE_MASK) != 0
    }

    /// Set TX_FETCH_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_fetch_mode(&self) {
        self.value.modify_bits(|r| r | Self::TX_FETCH_MODE_MASK);
    }

    /// Clear TX_FETCH_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_fetch_mode(&self) {
        self.value.modify_bits(|r| r & !Self::TX_FETCH_MODE_MASK);
    }

    /// **TX_COND_MODE** `[4]` (RW)
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub fn tx_cond_mode(&self) -> bool {
        (self.value.read_bits() & Self::TX_COND_MODE_MASK) != 0
    }

    /// Set TX_COND_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_cond_mode(&self) {
        self.value.modify_bits(|r| r | Self::TX_COND_MODE_MASK);
    }

    /// Clear TX_COND_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_cond_mode(&self) {
        self.value.modify_bits(|r| r & !Self::TX_COND_MODE_MASK);
    }

    /// **TX_EOF_MODE** `[3]` (RW)
    ///
    /// write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    #[inline(always)]
    pub fn tx_eof_mode(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_MODE_MASK) != 0
    }

    /// Set TX_EOF_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_eof_mode(&self) {
        self.value.modify_bits(|r| r | Self::TX_EOF_MODE_MASK);
    }

    /// Clear TX_EOF_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_eof_mode(&self) {
        self.value.modify_bits(|r| r & !Self::TX_EOF_MODE_MASK);
    }

    /// **TX_HALT** `[2]` (RW)
    ///
    /// write this bit to halt the bitscrambler tx core
    #[inline(always)]
    pub fn tx_halt(&self) -> bool {
        (self.value.read_bits() & Self::TX_HALT_MASK) != 0
    }

    /// Set TX_HALT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_halt(&self) {
        self.value.modify_bits(|r| r | Self::TX_HALT_MASK);
    }

    /// Clear TX_HALT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_halt(&self) {
        self.value.modify_bits(|r| r & !Self::TX_HALT_MASK);
    }

    /// **TX_PAUSE** `[1]` (RW)
    ///
    /// write this bit to pause the bitscrambler tx core
    #[inline(always)]
    pub fn tx_pause(&self) -> bool {
        (self.value.read_bits() & Self::TX_PAUSE_MASK) != 0
    }

    /// Set TX_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_pause(&self) {
        self.value.modify_bits(|r| r | Self::TX_PAUSE_MASK);
    }

    /// Clear TX_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_pause(&self) {
        self.value.modify_bits(|r| r & !Self::TX_PAUSE_MASK);
    }

    /// **TX_ENA** `[0]` (RW)
    ///
    /// write this bit to enable the bitscrambler tx
    #[inline(always)]
    pub fn tx_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_ENA_MASK) != 0
    }

    /// Set TX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_ENA_MASK);
    }

    /// Clear TX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_ENA_MASK);
    }
}

/// TX_CTRL value type for batch read/modify/write operations
///
/// Use `txctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCtrlVal {
    bits: u32,
}

impl TxCtrlVal {
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
        Self { bits: 0x00000004 }
    }

    /// Read **TX_RD_DUMMY** `[7:7]`
    ///
    /// write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub const fn tx_rd_dummy(&self) -> bool {
        (self.bits & TxCtrl::TX_RD_DUMMY_MASK) != 0
    }

    /// Read **TX_HALT_MODE** `[6:6]`
    ///
    /// write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub const fn tx_halt_mode(&self) -> bool {
        (self.bits & TxCtrl::TX_HALT_MODE_MASK) != 0
    }

    /// Read **TX_FETCH_MODE** `[5:5]`
    ///
    /// write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub const fn tx_fetch_mode(&self) -> bool {
        (self.bits & TxCtrl::TX_FETCH_MODE_MASK) != 0
    }

    /// Read **TX_COND_MODE** `[4:4]`
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub const fn tx_cond_mode(&self) -> bool {
        (self.bits & TxCtrl::TX_COND_MODE_MASK) != 0
    }

    /// Read **TX_EOF_MODE** `[3:3]`
    ///
    /// write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    #[inline(always)]
    pub const fn tx_eof_mode(&self) -> bool {
        (self.bits & TxCtrl::TX_EOF_MODE_MASK) != 0
    }

    /// Read **TX_HALT** `[2:2]`
    ///
    /// write this bit to halt the bitscrambler tx core
    #[inline(always)]
    pub const fn tx_halt(&self) -> bool {
        (self.bits & TxCtrl::TX_HALT_MASK) != 0
    }

    /// Read **TX_PAUSE** `[1:1]`
    ///
    /// write this bit to pause the bitscrambler tx core
    #[inline(always)]
    pub const fn tx_pause(&self) -> bool {
        (self.bits & TxCtrl::TX_PAUSE_MASK) != 0
    }

    /// Read **TX_ENA** `[0:0]`
    ///
    /// write this bit to enable the bitscrambler tx
    #[inline(always)]
    pub const fn tx_ena(&self) -> bool {
        (self.bits & TxCtrl::TX_ENA_MASK) != 0
    }

    /// Set **TX_FIFO_RST** `[8:8]`
    ///
    /// write this bit to reset the bitscrambler tx fifo
    #[inline(always)]
    pub const fn with_tx_fifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_FIFO_RST_MASK
            } else {
                self.bits & !TxCtrl::TX_FIFO_RST_MASK
            },
        }
    }

    /// Set **TX_RD_DUMMY** `[7:7]`
    ///
    /// write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub const fn with_tx_rd_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_RD_DUMMY_MASK
            } else {
                self.bits & !TxCtrl::TX_RD_DUMMY_MASK
            },
        }
    }

    /// Set **TX_HALT_MODE** `[6:6]`
    ///
    /// write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub const fn with_tx_halt_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_HALT_MODE_MASK
            } else {
                self.bits & !TxCtrl::TX_HALT_MODE_MASK
            },
        }
    }

    /// Set **TX_FETCH_MODE** `[5:5]`
    ///
    /// write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub const fn with_tx_fetch_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_FETCH_MODE_MASK
            } else {
                self.bits & !TxCtrl::TX_FETCH_MODE_MASK
            },
        }
    }

    /// Set **TX_COND_MODE** `[4:4]`
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub const fn with_tx_cond_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_COND_MODE_MASK
            } else {
                self.bits & !TxCtrl::TX_COND_MODE_MASK
            },
        }
    }

    /// Set **TX_EOF_MODE** `[3:3]`
    ///
    /// write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    #[inline(always)]
    pub const fn with_tx_eof_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_EOF_MODE_MASK
            } else {
                self.bits & !TxCtrl::TX_EOF_MODE_MASK
            },
        }
    }

    /// Set **TX_HALT** `[2:2]`
    ///
    /// write this bit to halt the bitscrambler tx core
    #[inline(always)]
    pub const fn with_tx_halt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_HALT_MASK
            } else {
                self.bits & !TxCtrl::TX_HALT_MASK
            },
        }
    }

    /// Set **TX_PAUSE** `[1:1]`
    ///
    /// write this bit to pause the bitscrambler tx core
    #[inline(always)]
    pub const fn with_tx_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_PAUSE_MASK
            } else {
                self.bits & !TxCtrl::TX_PAUSE_MASK
            },
        }
    }

    /// Set **TX_ENA** `[0:0]`
    ///
    /// write this bit to enable the bitscrambler tx
    #[inline(always)]
    pub const fn with_tx_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCtrl::TX_ENA_MASK
            } else {
                self.bits & !TxCtrl::TX_ENA_MASK
            },
        }
    }
}

impl Default for TxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCtrlVal {
        TxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCtrlVal) -> TxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CTRL Register
///
/// Control and configuration registers
///
/// Address offset: 0x002c
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **RX_FIFO_RST** `[8]` (WO)
///
///   write this bit to reset the bitscrambler rx fifo
///
/// - **RX_RD_DUMMY** `[7]` (RW)
///
///   write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data
///
/// - **RX_HALT_MODE** `[6]` (RW)
///
///   write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back
///
/// - **RX_FETCH_MODE** `[5]` (RW)
///
///   write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
///
/// - **RX_COND_MODE** `[4]` (RW)
///
///   write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
///
/// - **RX_EOF_MODE** `[3]` (RW)
///
///   write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo
///
/// - **RX_HALT** `[2]` (RW)
///
///   write this bit to halt the bitscrambler rx core
///
/// - **RX_PAUSE** `[1]` (RW)
///
///   write this bit to pause the bitscrambler rx core
///
/// - **RX_ENA** `[0]` (RW)
///
///   write this bit to enable the bitscrambler rx
#[repr(transparent)]
pub struct RxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCtrl {
    // ======== Field Constants ========

    /// **RX_FIFO_RST**
    ///
    /// write this bit to reset the bitscrambler rx fifo
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RX_FIFO_RST_OFFSET: u32 = 8;
    pub const RX_FIFO_RST_MASK: u32 = 0x00000100;
    pub const RX_FIFO_RST: u32 = 1 << 8;

    /// **RX_RD_DUMMY**
    ///
    /// write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_RD_DUMMY_OFFSET: u32 = 7;
    pub const RX_RD_DUMMY_MASK: u32 = 0x00000080;
    pub const RX_RD_DUMMY: u32 = 1 << 7;

    /// **RX_HALT_MODE**
    ///
    /// write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_HALT_MODE_OFFSET: u32 = 6;
    pub const RX_HALT_MODE_MASK: u32 = 0x00000040;
    pub const RX_HALT_MODE: u32 = 1 << 6;

    /// **RX_FETCH_MODE**
    ///
    /// write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_FETCH_MODE_OFFSET: u32 = 5;
    pub const RX_FETCH_MODE_MASK: u32 = 0x00000020;
    pub const RX_FETCH_MODE: u32 = 1 << 5;

    /// **RX_COND_MODE**
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_COND_MODE_OFFSET: u32 = 4;
    pub const RX_COND_MODE_MASK: u32 = 0x00000010;
    pub const RX_COND_MODE: u32 = 1 << 4;

    /// **RX_EOF_MODE**
    ///
    /// write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_EOF_MODE_OFFSET: u32 = 3;
    pub const RX_EOF_MODE_MASK: u32 = 0x00000008;
    pub const RX_EOF_MODE: u32 = 1 << 3;

    /// **RX_HALT**
    ///
    /// write this bit to halt the bitscrambler rx core
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HALT_OFFSET: u32 = 2;
    pub const RX_HALT_MASK: u32 = 0x00000004;
    pub const RX_HALT: u32 = 1 << 2;

    /// **RX_PAUSE**
    ///
    /// write this bit to pause the bitscrambler rx core
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_PAUSE_OFFSET: u32 = 1;
    pub const RX_PAUSE_MASK: u32 = 0x00000002;
    pub const RX_PAUSE: u32 = 1 << 1;

    /// **RX_ENA**
    ///
    /// write this bit to enable the bitscrambler rx
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_ENA_OFFSET: u32 = 0;
    pub const RX_ENA_MASK: u32 = 0x00000001;
    pub const RX_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **RX_FIFO_RST** (WO)
    ///
    /// write this bit to reset the bitscrambler rx fifo
    #[inline(always)]
    pub fn trigger_rx_fifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::RX_FIFO_RST_MASK);
    }

    /// **RX_RD_DUMMY** `[7]` (RW)
    ///
    /// write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub fn rx_rd_dummy(&self) -> bool {
        (self.value.read_bits() & Self::RX_RD_DUMMY_MASK) != 0
    }

    /// Set RX_RD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_rd_dummy(&self) {
        self.value.modify_bits(|r| r | Self::RX_RD_DUMMY_MASK);
    }

    /// Clear RX_RD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_rd_dummy(&self) {
        self.value.modify_bits(|r| r & !Self::RX_RD_DUMMY_MASK);
    }

    /// **RX_HALT_MODE** `[6]` (RW)
    ///
    /// write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub fn rx_halt_mode(&self) -> bool {
        (self.value.read_bits() & Self::RX_HALT_MODE_MASK) != 0
    }

    /// Set RX_HALT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_halt_mode(&self) {
        self.value.modify_bits(|r| r | Self::RX_HALT_MODE_MASK);
    }

    /// Clear RX_HALT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_halt_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RX_HALT_MODE_MASK);
    }

    /// **RX_FETCH_MODE** `[5]` (RW)
    ///
    /// write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub fn rx_fetch_mode(&self) -> bool {
        (self.value.read_bits() & Self::RX_FETCH_MODE_MASK) != 0
    }

    /// Set RX_FETCH_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fetch_mode(&self) {
        self.value.modify_bits(|r| r | Self::RX_FETCH_MODE_MASK);
    }

    /// Clear RX_FETCH_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fetch_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FETCH_MODE_MASK);
    }

    /// **RX_COND_MODE** `[4]` (RW)
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub fn rx_cond_mode(&self) -> bool {
        (self.value.read_bits() & Self::RX_COND_MODE_MASK) != 0
    }

    /// Set RX_COND_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_cond_mode(&self) {
        self.value.modify_bits(|r| r | Self::RX_COND_MODE_MASK);
    }

    /// Clear RX_COND_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_cond_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RX_COND_MODE_MASK);
    }

    /// **RX_EOF_MODE** `[3]` (RW)
    ///
    /// write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo
    #[inline(always)]
    pub fn rx_eof_mode(&self) -> bool {
        (self.value.read_bits() & Self::RX_EOF_MODE_MASK) != 0
    }

    /// Set RX_EOF_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_eof_mode(&self) {
        self.value.modify_bits(|r| r | Self::RX_EOF_MODE_MASK);
    }

    /// Clear RX_EOF_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_eof_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EOF_MODE_MASK);
    }

    /// **RX_HALT** `[2]` (RW)
    ///
    /// write this bit to halt the bitscrambler rx core
    #[inline(always)]
    pub fn rx_halt(&self) -> bool {
        (self.value.read_bits() & Self::RX_HALT_MASK) != 0
    }

    /// Set RX_HALT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_halt(&self) {
        self.value.modify_bits(|r| r | Self::RX_HALT_MASK);
    }

    /// Clear RX_HALT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_halt(&self) {
        self.value.modify_bits(|r| r & !Self::RX_HALT_MASK);
    }

    /// **RX_PAUSE** `[1]` (RW)
    ///
    /// write this bit to pause the bitscrambler rx core
    #[inline(always)]
    pub fn rx_pause(&self) -> bool {
        (self.value.read_bits() & Self::RX_PAUSE_MASK) != 0
    }

    /// Set RX_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_pause(&self) {
        self.value.modify_bits(|r| r | Self::RX_PAUSE_MASK);
    }

    /// Clear RX_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_pause(&self) {
        self.value.modify_bits(|r| r & !Self::RX_PAUSE_MASK);
    }

    /// **RX_ENA** `[0]` (RW)
    ///
    /// write this bit to enable the bitscrambler rx
    #[inline(always)]
    pub fn rx_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_ENA_MASK) != 0
    }

    /// Set RX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ena(&self) {
        self.value.modify_bits(|r| r | Self::RX_ENA_MASK);
    }

    /// Clear RX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RX_ENA_MASK);
    }
}

/// RX_CTRL value type for batch read/modify/write operations
///
/// Use `rxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCtrlVal {
    bits: u32,
}

impl RxCtrlVal {
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
        Self { bits: 0x00000004 }
    }

    /// Read **RX_RD_DUMMY** `[7:7]`
    ///
    /// write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub const fn rx_rd_dummy(&self) -> bool {
        (self.bits & RxCtrl::RX_RD_DUMMY_MASK) != 0
    }

    /// Read **RX_HALT_MODE** `[6:6]`
    ///
    /// write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub const fn rx_halt_mode(&self) -> bool {
        (self.bits & RxCtrl::RX_HALT_MODE_MASK) != 0
    }

    /// Read **RX_FETCH_MODE** `[5:5]`
    ///
    /// write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub const fn rx_fetch_mode(&self) -> bool {
        (self.bits & RxCtrl::RX_FETCH_MODE_MASK) != 0
    }

    /// Read **RX_COND_MODE** `[4:4]`
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub const fn rx_cond_mode(&self) -> bool {
        (self.bits & RxCtrl::RX_COND_MODE_MASK) != 0
    }

    /// Read **RX_EOF_MODE** `[3:3]`
    ///
    /// write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo
    #[inline(always)]
    pub const fn rx_eof_mode(&self) -> bool {
        (self.bits & RxCtrl::RX_EOF_MODE_MASK) != 0
    }

    /// Read **RX_HALT** `[2:2]`
    ///
    /// write this bit to halt the bitscrambler rx core
    #[inline(always)]
    pub const fn rx_halt(&self) -> bool {
        (self.bits & RxCtrl::RX_HALT_MASK) != 0
    }

    /// Read **RX_PAUSE** `[1:1]`
    ///
    /// write this bit to pause the bitscrambler rx core
    #[inline(always)]
    pub const fn rx_pause(&self) -> bool {
        (self.bits & RxCtrl::RX_PAUSE_MASK) != 0
    }

    /// Read **RX_ENA** `[0:0]`
    ///
    /// write this bit to enable the bitscrambler rx
    #[inline(always)]
    pub const fn rx_ena(&self) -> bool {
        (self.bits & RxCtrl::RX_ENA_MASK) != 0
    }

    /// Set **RX_FIFO_RST** `[8:8]`
    ///
    /// write this bit to reset the bitscrambler rx fifo
    #[inline(always)]
    pub const fn with_rx_fifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_FIFO_RST_MASK
            } else {
                self.bits & !RxCtrl::RX_FIFO_RST_MASK
            },
        }
    }

    /// Set **RX_RD_DUMMY** `[7:7]`
    ///
    /// write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub const fn with_rx_rd_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_RD_DUMMY_MASK
            } else {
                self.bits & !RxCtrl::RX_RD_DUMMY_MASK
            },
        }
    }

    /// Set **RX_HALT_MODE** `[6:6]`
    ///
    /// write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub const fn with_rx_halt_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_HALT_MODE_MASK
            } else {
                self.bits & !RxCtrl::RX_HALT_MODE_MASK
            },
        }
    }

    /// Set **RX_FETCH_MODE** `[5:5]`
    ///
    /// write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub const fn with_rx_fetch_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_FETCH_MODE_MASK
            } else {
                self.bits & !RxCtrl::RX_FETCH_MODE_MASK
            },
        }
    }

    /// Set **RX_COND_MODE** `[4:4]`
    ///
    /// write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub const fn with_rx_cond_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_COND_MODE_MASK
            } else {
                self.bits & !RxCtrl::RX_COND_MODE_MASK
            },
        }
    }

    /// Set **RX_EOF_MODE** `[3:3]`
    ///
    /// write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo
    #[inline(always)]
    pub const fn with_rx_eof_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_EOF_MODE_MASK
            } else {
                self.bits & !RxCtrl::RX_EOF_MODE_MASK
            },
        }
    }

    /// Set **RX_HALT** `[2:2]`
    ///
    /// write this bit to halt the bitscrambler rx core
    #[inline(always)]
    pub const fn with_rx_halt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_HALT_MASK
            } else {
                self.bits & !RxCtrl::RX_HALT_MASK
            },
        }
    }

    /// Set **RX_PAUSE** `[1:1]`
    ///
    /// write this bit to pause the bitscrambler rx core
    #[inline(always)]
    pub const fn with_rx_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_PAUSE_MASK
            } else {
                self.bits & !RxCtrl::RX_PAUSE_MASK
            },
        }
    }

    /// Set **RX_ENA** `[0:0]`
    ///
    /// write this bit to enable the bitscrambler rx
    #[inline(always)]
    pub const fn with_rx_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCtrl::RX_ENA_MASK
            } else {
                self.bits & !RxCtrl::RX_ENA_MASK
            },
        }
    }
}

impl Default for RxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCtrlVal {
        RxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCtrlVal) -> RxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_STATE Register
///
/// Status registers
///
/// Address offset: 0x0030
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **TX_EOF_TRACE_CLR** `[31]` (WO)
///
///   write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers
///
/// - **TX_EOF_OVERLOAD** `[30]` (RO)
///
///   represents the some EOFs will be lost for bitscrambler tx core
///
/// - **TX_EOF_GET_CNT** `[29:16]` (RO)
///
///   represents the bytes numbers of bitscrambler tx core when get EOF
///
/// - **TX_FIFO_EMPTY** `[4]` (RO)
///
///   represents the bitscrambler tx fifo in empty state
///
/// - **TX_IN_PAUSE** `[3]` (RO)
///
///   represents the bitscrambler tx core in pause mode
///
/// - **TX_IN_WAIT** `[2]` (RO)
///
///   represents the bitscrambler tx core in wait mode to wait write back done
///
/// - **TX_IN_RUN** `[1]` (RO)
///
///   represents the bitscrambler tx core in run mode
///
/// - **TX_IN_IDLE** `[0]` (RO)
///
///   represents the bitscrambler tx core in halt mode
#[repr(transparent)]
pub struct TxState {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxState {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxState {
    // ======== Field Constants ========

    /// **TX_EOF_TRACE_CLR**
    ///
    /// write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_EOF_TRACE_CLR_OFFSET: u32 = 31;
    pub const TX_EOF_TRACE_CLR_MASK: u32 = 0x80000000;
    pub const TX_EOF_TRACE_CLR: u32 = 1 << 31;

    /// **TX_EOF_OVERLOAD**
    ///
    /// represents the some EOFs will be lost for bitscrambler tx core
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TX_EOF_OVERLOAD_OFFSET: u32 = 30;
    pub const TX_EOF_OVERLOAD_MASK: u32 = 0x40000000;
    pub const TX_EOF_OVERLOAD: u32 = 1 << 30;

    /// **TX_EOF_GET_CNT**
    ///
    /// represents the bytes numbers of bitscrambler tx core when get EOF
    ///
    /// Bits `[29:16]`, mask `0x3fff0000`
    pub const TX_EOF_GET_CNT_OFFSET: u32 = 16;
    pub const TX_EOF_GET_CNT_MASK: u32 = 0x3fff0000;

    /// **TX_FIFO_EMPTY**
    ///
    /// represents the bitscrambler tx fifo in empty state
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_FIFO_EMPTY_OFFSET: u32 = 4;
    pub const TX_FIFO_EMPTY_MASK: u32 = 0x00000010;
    pub const TX_FIFO_EMPTY: u32 = 1 << 4;

    /// **TX_IN_PAUSE**
    ///
    /// represents the bitscrambler tx core in pause mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_IN_PAUSE_OFFSET: u32 = 3;
    pub const TX_IN_PAUSE_MASK: u32 = 0x00000008;
    pub const TX_IN_PAUSE: u32 = 1 << 3;

    /// **TX_IN_WAIT**
    ///
    /// represents the bitscrambler tx core in wait mode to wait write back done
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_IN_WAIT_OFFSET: u32 = 2;
    pub const TX_IN_WAIT_MASK: u32 = 0x00000004;
    pub const TX_IN_WAIT: u32 = 1 << 2;

    /// **TX_IN_RUN**
    ///
    /// represents the bitscrambler tx core in run mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_IN_RUN_OFFSET: u32 = 1;
    pub const TX_IN_RUN_MASK: u32 = 0x00000002;
    pub const TX_IN_RUN: u32 = 1 << 1;

    /// **TX_IN_IDLE**
    ///
    /// represents the bitscrambler tx core in halt mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_IN_IDLE_OFFSET: u32 = 0;
    pub const TX_IN_IDLE_MASK: u32 = 0x00000001;
    pub const TX_IN_IDLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TX_EOF_TRACE_CLR** (WO)
    ///
    /// write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers
    #[inline(always)]
    pub fn trigger_tx_eof_trace_clr(&self) {
        self.value.modify_bits(|r| r | Self::TX_EOF_TRACE_CLR_MASK);
    }

    /// **TX_EOF_OVERLOAD** `[30]` (RO)
    ///
    /// represents the some EOFs will be lost for bitscrambler tx core
    #[inline(always)]
    pub fn tx_eof_overload(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_OVERLOAD_MASK) != 0
    }

    /// **TX_EOF_GET_CNT** `[29:16]` (RO)
    ///
    /// represents the bytes numbers of bitscrambler tx core when get EOF
    #[inline(always)]
    pub fn tx_eof_get_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_EOF_GET_CNT_MASK) >> Self::TX_EOF_GET_CNT_OFFSET
    }

    /// **TX_FIFO_EMPTY** `[4]` (RO)
    ///
    /// represents the bitscrambler tx fifo in empty state
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_EMPTY_MASK) != 0
    }

    /// **TX_IN_PAUSE** `[3]` (RO)
    ///
    /// represents the bitscrambler tx core in pause mode
    #[inline(always)]
    pub fn tx_in_pause(&self) -> bool {
        (self.value.read_bits() & Self::TX_IN_PAUSE_MASK) != 0
    }

    /// **TX_IN_WAIT** `[2]` (RO)
    ///
    /// represents the bitscrambler tx core in wait mode to wait write back done
    #[inline(always)]
    pub fn tx_in_wait(&self) -> bool {
        (self.value.read_bits() & Self::TX_IN_WAIT_MASK) != 0
    }

    /// **TX_IN_RUN** `[1]` (RO)
    ///
    /// represents the bitscrambler tx core in run mode
    #[inline(always)]
    pub fn tx_in_run(&self) -> bool {
        (self.value.read_bits() & Self::TX_IN_RUN_MASK) != 0
    }

    /// **TX_IN_IDLE** `[0]` (RO)
    ///
    /// represents the bitscrambler tx core in halt mode
    #[inline(always)]
    pub fn tx_in_idle(&self) -> bool {
        (self.value.read_bits() & Self::TX_IN_IDLE_MASK) != 0
    }
}

/// TX_STATE value type for batch read/modify/write operations
///
/// Use `txstate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txstate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxStateVal {
    bits: u32,
}

impl TxStateVal {
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

    /// Read **TX_EOF_OVERLOAD** `[30:30]`
    ///
    /// represents the some EOFs will be lost for bitscrambler tx core
    #[inline(always)]
    pub const fn tx_eof_overload(&self) -> bool {
        (self.bits & TxState::TX_EOF_OVERLOAD_MASK) != 0
    }

    /// Read **TX_EOF_GET_CNT** `[29:16]`
    ///
    /// represents the bytes numbers of bitscrambler tx core when get EOF
    #[inline(always)]
    pub const fn tx_eof_get_cnt(&self) -> u32 {
        (self.bits & TxState::TX_EOF_GET_CNT_MASK) >> TxState::TX_EOF_GET_CNT_OFFSET
    }

    /// Read **TX_FIFO_EMPTY** `[4:4]`
    ///
    /// represents the bitscrambler tx fifo in empty state
    #[inline(always)]
    pub const fn tx_fifo_empty(&self) -> bool {
        (self.bits & TxState::TX_FIFO_EMPTY_MASK) != 0
    }

    /// Read **TX_IN_PAUSE** `[3:3]`
    ///
    /// represents the bitscrambler tx core in pause mode
    #[inline(always)]
    pub const fn tx_in_pause(&self) -> bool {
        (self.bits & TxState::TX_IN_PAUSE_MASK) != 0
    }

    /// Read **TX_IN_WAIT** `[2:2]`
    ///
    /// represents the bitscrambler tx core in wait mode to wait write back done
    #[inline(always)]
    pub const fn tx_in_wait(&self) -> bool {
        (self.bits & TxState::TX_IN_WAIT_MASK) != 0
    }

    /// Read **TX_IN_RUN** `[1:1]`
    ///
    /// represents the bitscrambler tx core in run mode
    #[inline(always)]
    pub const fn tx_in_run(&self) -> bool {
        (self.bits & TxState::TX_IN_RUN_MASK) != 0
    }

    /// Read **TX_IN_IDLE** `[0:0]`
    ///
    /// represents the bitscrambler tx core in halt mode
    #[inline(always)]
    pub const fn tx_in_idle(&self) -> bool {
        (self.bits & TxState::TX_IN_IDLE_MASK) != 0
    }

    /// Set **TX_EOF_TRACE_CLR** `[31:31]`
    ///
    /// write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers
    #[inline(always)]
    pub const fn with_tx_eof_trace_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxState::TX_EOF_TRACE_CLR_MASK
            } else {
                self.bits & !TxState::TX_EOF_TRACE_CLR_MASK
            },
        }
    }
}

impl Default for TxStateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxState {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxStateVal {
        TxStateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxStateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxStateVal) -> TxStateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_STATE Register
///
/// Status registers
///
/// Address offset: 0x0034
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **RX_EOF_TRACE_CLR** `[31]` (WO)
///
///   write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers
///
/// - **RX_EOF_OVERLOAD** `[30]` (RO)
///
///   represents the some EOFs will be lost for bitscrambler rx core
///
/// - **RX_EOF_GET_CNT** `[29:16]` (RO)
///
///   represents the bytes numbers of bitscrambler rx core when get EOF
///
/// - **RX_FIFO_FULL** `[4]` (RO)
///
///   represents the bitscrambler rx fifo in full state
///
/// - **RX_IN_PAUSE** `[3]` (RO)
///
///   represents the bitscrambler rx core in pause mode
///
/// - **RX_IN_WAIT** `[2]` (RO)
///
///   represents the bitscrambler rx core in wait mode to wait write back done
///
/// - **RX_IN_RUN** `[1]` (RO)
///
///   represents the bitscrambler rx core in run mode
///
/// - **RX_IN_IDLE** `[0]` (RO)
///
///   represents the bitscrambler rx core in halt mode
#[repr(transparent)]
pub struct RxState {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxState {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxState {
    // ======== Field Constants ========

    /// **RX_EOF_TRACE_CLR**
    ///
    /// write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RX_EOF_TRACE_CLR_OFFSET: u32 = 31;
    pub const RX_EOF_TRACE_CLR_MASK: u32 = 0x80000000;
    pub const RX_EOF_TRACE_CLR: u32 = 1 << 31;

    /// **RX_EOF_OVERLOAD**
    ///
    /// represents the some EOFs will be lost for bitscrambler rx core
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RX_EOF_OVERLOAD_OFFSET: u32 = 30;
    pub const RX_EOF_OVERLOAD_MASK: u32 = 0x40000000;
    pub const RX_EOF_OVERLOAD: u32 = 1 << 30;

    /// **RX_EOF_GET_CNT**
    ///
    /// represents the bytes numbers of bitscrambler rx core when get EOF
    ///
    /// Bits `[29:16]`, mask `0x3fff0000`
    pub const RX_EOF_GET_CNT_OFFSET: u32 = 16;
    pub const RX_EOF_GET_CNT_MASK: u32 = 0x3fff0000;

    /// **RX_FIFO_FULL**
    ///
    /// represents the bitscrambler rx fifo in full state
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FIFO_FULL_OFFSET: u32 = 4;
    pub const RX_FIFO_FULL_MASK: u32 = 0x00000010;
    pub const RX_FIFO_FULL: u32 = 1 << 4;

    /// **RX_IN_PAUSE**
    ///
    /// represents the bitscrambler rx core in pause mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_IN_PAUSE_OFFSET: u32 = 3;
    pub const RX_IN_PAUSE_MASK: u32 = 0x00000008;
    pub const RX_IN_PAUSE: u32 = 1 << 3;

    /// **RX_IN_WAIT**
    ///
    /// represents the bitscrambler rx core in wait mode to wait write back done
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_IN_WAIT_OFFSET: u32 = 2;
    pub const RX_IN_WAIT_MASK: u32 = 0x00000004;
    pub const RX_IN_WAIT: u32 = 1 << 2;

    /// **RX_IN_RUN**
    ///
    /// represents the bitscrambler rx core in run mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_IN_RUN_OFFSET: u32 = 1;
    pub const RX_IN_RUN_MASK: u32 = 0x00000002;
    pub const RX_IN_RUN: u32 = 1 << 1;

    /// **RX_IN_IDLE**
    ///
    /// represents the bitscrambler rx core in halt mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_IN_IDLE_OFFSET: u32 = 0;
    pub const RX_IN_IDLE_MASK: u32 = 0x00000001;
    pub const RX_IN_IDLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **RX_EOF_TRACE_CLR** (WO)
    ///
    /// write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers
    #[inline(always)]
    pub fn trigger_rx_eof_trace_clr(&self) {
        self.value.modify_bits(|r| r | Self::RX_EOF_TRACE_CLR_MASK);
    }

    /// **RX_EOF_OVERLOAD** `[30]` (RO)
    ///
    /// represents the some EOFs will be lost for bitscrambler rx core
    #[inline(always)]
    pub fn rx_eof_overload(&self) -> bool {
        (self.value.read_bits() & Self::RX_EOF_OVERLOAD_MASK) != 0
    }

    /// **RX_EOF_GET_CNT** `[29:16]` (RO)
    ///
    /// represents the bytes numbers of bitscrambler rx core when get EOF
    #[inline(always)]
    pub fn rx_eof_get_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_EOF_GET_CNT_MASK) >> Self::RX_EOF_GET_CNT_OFFSET
    }

    /// **RX_FIFO_FULL** `[4]` (RO)
    ///
    /// represents the bitscrambler rx fifo in full state
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_FULL_MASK) != 0
    }

    /// **RX_IN_PAUSE** `[3]` (RO)
    ///
    /// represents the bitscrambler rx core in pause mode
    #[inline(always)]
    pub fn rx_in_pause(&self) -> bool {
        (self.value.read_bits() & Self::RX_IN_PAUSE_MASK) != 0
    }

    /// **RX_IN_WAIT** `[2]` (RO)
    ///
    /// represents the bitscrambler rx core in wait mode to wait write back done
    #[inline(always)]
    pub fn rx_in_wait(&self) -> bool {
        (self.value.read_bits() & Self::RX_IN_WAIT_MASK) != 0
    }

    /// **RX_IN_RUN** `[1]` (RO)
    ///
    /// represents the bitscrambler rx core in run mode
    #[inline(always)]
    pub fn rx_in_run(&self) -> bool {
        (self.value.read_bits() & Self::RX_IN_RUN_MASK) != 0
    }

    /// **RX_IN_IDLE** `[0]` (RO)
    ///
    /// represents the bitscrambler rx core in halt mode
    #[inline(always)]
    pub fn rx_in_idle(&self) -> bool {
        (self.value.read_bits() & Self::RX_IN_IDLE_MASK) != 0
    }
}

/// RX_STATE value type for batch read/modify/write operations
///
/// Use `rxstate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxstate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxStateVal {
    bits: u32,
}

impl RxStateVal {
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

    /// Read **RX_EOF_OVERLOAD** `[30:30]`
    ///
    /// represents the some EOFs will be lost for bitscrambler rx core
    #[inline(always)]
    pub const fn rx_eof_overload(&self) -> bool {
        (self.bits & RxState::RX_EOF_OVERLOAD_MASK) != 0
    }

    /// Read **RX_EOF_GET_CNT** `[29:16]`
    ///
    /// represents the bytes numbers of bitscrambler rx core when get EOF
    #[inline(always)]
    pub const fn rx_eof_get_cnt(&self) -> u32 {
        (self.bits & RxState::RX_EOF_GET_CNT_MASK) >> RxState::RX_EOF_GET_CNT_OFFSET
    }

    /// Read **RX_FIFO_FULL** `[4:4]`
    ///
    /// represents the bitscrambler rx fifo in full state
    #[inline(always)]
    pub const fn rx_fifo_full(&self) -> bool {
        (self.bits & RxState::RX_FIFO_FULL_MASK) != 0
    }

    /// Read **RX_IN_PAUSE** `[3:3]`
    ///
    /// represents the bitscrambler rx core in pause mode
    #[inline(always)]
    pub const fn rx_in_pause(&self) -> bool {
        (self.bits & RxState::RX_IN_PAUSE_MASK) != 0
    }

    /// Read **RX_IN_WAIT** `[2:2]`
    ///
    /// represents the bitscrambler rx core in wait mode to wait write back done
    #[inline(always)]
    pub const fn rx_in_wait(&self) -> bool {
        (self.bits & RxState::RX_IN_WAIT_MASK) != 0
    }

    /// Read **RX_IN_RUN** `[1:1]`
    ///
    /// represents the bitscrambler rx core in run mode
    #[inline(always)]
    pub const fn rx_in_run(&self) -> bool {
        (self.bits & RxState::RX_IN_RUN_MASK) != 0
    }

    /// Read **RX_IN_IDLE** `[0:0]`
    ///
    /// represents the bitscrambler rx core in halt mode
    #[inline(always)]
    pub const fn rx_in_idle(&self) -> bool {
        (self.bits & RxState::RX_IN_IDLE_MASK) != 0
    }

    /// Set **RX_EOF_TRACE_CLR** `[31:31]`
    ///
    /// write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers
    #[inline(always)]
    pub const fn with_rx_eof_trace_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxState::RX_EOF_TRACE_CLR_MASK
            } else {
                self.bits & !RxState::RX_EOF_TRACE_CLR_MASK
            },
        }
    }
}

impl Default for RxStateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxState {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxStateVal {
        RxStateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxStateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxStateVal) -> RxStateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYS Register
///
/// Control and configuration registers
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Reserved
///
/// - **LOOP_MODE** `[0]` (RW)
///
///   write this bit to set the bitscrambler tx loop back to DMA rx
#[repr(transparent)]
pub struct Sys {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sys {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sys {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Reserved
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **LOOP_MODE**
    ///
    /// write this bit to set the bitscrambler tx loop back to DMA rx
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOOP_MODE_OFFSET: u32 = 0;
    pub const LOOP_MODE_MASK: u32 = 0x00000001;
    pub const LOOP_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Reserved
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

    /// **LOOP_MODE** `[0]` (RW)
    ///
    /// write this bit to set the bitscrambler tx loop back to DMA rx
    #[inline(always)]
    pub fn loop_mode(&self) -> bool {
        (self.value.read_bits() & Self::LOOP_MODE_MASK) != 0
    }

    /// Set LOOP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loop_mode(&self) {
        self.value.modify_bits(|r| r | Self::LOOP_MODE_MASK);
    }

    /// Clear LOOP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loop_mode(&self) {
        self.value.modify_bits(|r| r & !Self::LOOP_MODE_MASK);
    }
}

/// SYS value type for batch read/modify/write operations
///
/// Use `sys.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sys.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysVal {
    bits: u32,
}

impl SysVal {
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

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Sys::CLK_EN_MASK) != 0
    }

    /// Read **LOOP_MODE** `[0:0]`
    ///
    /// write this bit to set the bitscrambler tx loop back to DMA rx
    #[inline(always)]
    pub const fn loop_mode(&self) -> bool {
        (self.bits & Sys::LOOP_MODE_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sys::CLK_EN_MASK
            } else {
                self.bits & !Sys::CLK_EN_MASK
            },
        }
    }

    /// Set **LOOP_MODE** `[0:0]`
    ///
    /// write this bit to set the bitscrambler tx loop back to DMA rx
    #[inline(always)]
    pub const fn with_loop_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sys::LOOP_MODE_MASK
            } else {
                self.bits & !Sys::LOOP_MODE_MASK
            },
        }
    }
}

impl Default for SysVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sys {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysVal {
        SysVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysVal) -> SysVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VERSION Register
///
/// Control and configuration registers
///
/// Address offset: 0x00fc
/// Reset value: 0x02303240
///
/// ## Fields
///
/// - **BITSCRAMBLER_VER** `[27:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Version {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Version {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Version {
    // ======== Field Constants ========

    /// **BITSCRAMBLER_VER**
    ///
    /// Reserved
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const BITSCRAMBLER_VER_OFFSET: u32 = 0;
    pub const BITSCRAMBLER_VER_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **BITSCRAMBLER_VER** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn bitscrambler_ver(&self) -> u32 {
        (self.value.read_bits() & Self::BITSCRAMBLER_VER_MASK) >> Self::BITSCRAMBLER_VER_OFFSET
    }

    /// Write **BITSCRAMBLER_VER** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_bitscrambler_ver(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BITSCRAMBLER_VER_MASK)
                | ((value << Self::BITSCRAMBLER_VER_OFFSET) & Self::BITSCRAMBLER_VER_MASK)
        });
    }
}

/// VERSION value type for batch read/modify/write operations
///
/// Use `version.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `version.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VersionVal {
    bits: u32,
}

impl VersionVal {
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

    /// Read **BITSCRAMBLER_VER** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn bitscrambler_ver(&self) -> u32 {
        (self.bits & Version::BITSCRAMBLER_VER_MASK) >> Version::BITSCRAMBLER_VER_OFFSET
    }

    /// Set **BITSCRAMBLER_VER** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_bitscrambler_ver(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Version::BITSCRAMBLER_VER_MASK)
                | ((value << Version::BITSCRAMBLER_VER_OFFSET) & Version::BITSCRAMBLER_VER_MASK),
        }
    }
}

impl Default for VersionVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Version {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VersionVal {
        VersionVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VersionVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VersionVal) -> VersionVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and configuration registers
    pub tx_inst_cfg0: TxInstCfg0,
    /// 0x0004
    /// Control and configuration registers
    pub tx_inst_cfg1: TxInstCfg1,
    /// 0x0008
    /// Control and configuration registers
    pub rx_inst_cfg0: RxInstCfg0,
    /// 0x000c
    /// Control and configuration registers
    pub rx_inst_cfg1: RxInstCfg1,
    /// 0x0010
    /// Control and configuration registers
    pub tx_lut_cfg0: TxLutCfg0,
    /// 0x0014
    /// Control and configuration registers
    pub tx_lut_cfg1: TxLutCfg1,
    /// 0x0018
    /// Control and configuration registers
    pub rx_lut_cfg0: RxLutCfg0,
    /// 0x001c
    /// Control and configuration registers
    pub rx_lut_cfg1: RxLutCfg1,
    /// 0x0020
    /// Control and configuration registers
    pub tx_tailing_bits: TxTailingBits,
    /// 0x0024
    /// Control and configuration registers
    pub rx_tailing_bits: RxTailingBits,
    /// 0x0028
    /// Control and configuration registers
    pub tx_ctrl: TxCtrl,
    /// 0x002c
    /// Control and configuration registers
    pub rx_ctrl: RxCtrl,
    /// 0x0030
    /// Status registers
    pub tx_state: TxState,
    /// 0x0034
    /// Status registers
    pub rx_state: RxState,
    _reserved0: [u8; 192],
    /// 0x00f8
    /// Control and configuration registers
    pub sys: Sys,
    /// 0x00fc
    /// Control and configuration registers
    pub version: Version,
}
