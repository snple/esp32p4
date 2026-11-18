//! SHA
//! SHA (Secure Hash Algorithm) Accelerator

#![allow(dead_code)]

pub const BASE: u32 = 0x50091000;

/// MODE Register
///
/// Initial configuration register.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **MODE** `[2:0]` (RW)
///
///   Sha mode.
#[repr(transparent)]
pub struct Mode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mode {
    // ======== Field Constants ========

    /// **MODE**
    ///
    /// Sha mode.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const MODE_OFFSET: u32 = 0;
    pub const MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **MODE** `[2:0]` (RW)
    ///
    /// Sha mode.
    #[inline(always)]
    pub fn mode(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_MASK) >> Self::MODE_OFFSET
    }

    /// Write **MODE** `[2:0]` (RW)
    ///
    /// Sha mode.
    #[inline(always)]
    pub fn write_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE_MASK) | ((value << Self::MODE_OFFSET) & Self::MODE_MASK)
        });
    }
}

/// MODE value type for batch read/modify/write operations
///
/// Use `mode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ModeVal {
    bits: u32,
}

impl ModeVal {
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

    /// Read **MODE** `[2:0]`
    ///
    /// Sha mode.
    #[inline(always)]
    pub const fn mode(&self) -> u32 {
        (self.bits & Mode::MODE_MASK) >> Mode::MODE_OFFSET
    }

    /// Set **MODE** `[2:0]`
    ///
    /// Sha mode.
    #[inline(always)]
    pub const fn with_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mode::MODE_MASK) | ((value << Mode::MODE_OFFSET) & Mode::MODE_MASK),
        }
    }
}

impl Default for ModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Mode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ModeVal {
        ModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ModeVal) -> ModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T_STRING Register
///
/// SHA 512/t configuration register 0.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **T_STRING** `[31:0]` (RW)
///
///   Sha t_string (used if and only if mode == SHA_512/t).
#[repr(transparent)]
pub struct TString {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TString {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TString {
    // ======== Field Constants ========

    /// **T_STRING**
    ///
    /// Sha t_string (used if and only if mode == SHA_512/t).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const T_STRING_OFFSET: u32 = 0;
    pub const T_STRING_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **T_STRING** `[31:0]` (RW)
    ///
    /// Sha t_string (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub fn t_string(&self) -> u32 {
        (self.value.read_bits() & Self::T_STRING_MASK) >> Self::T_STRING_OFFSET
    }

    /// Write **T_STRING** `[31:0]` (RW)
    ///
    /// Sha t_string (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub fn write_t_string(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T_STRING_MASK) | ((value << Self::T_STRING_OFFSET) & Self::T_STRING_MASK)
        });
    }
}

/// T_STRING value type for batch read/modify/write operations
///
/// Use `tstring.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tstring.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TStringVal {
    bits: u32,
}

impl TStringVal {
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

    /// Read **T_STRING** `[31:0]`
    ///
    /// Sha t_string (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub const fn t_string(&self) -> u32 {
        (self.bits & TString::T_STRING_MASK) >> TString::T_STRING_OFFSET
    }

    /// Set **T_STRING** `[31:0]`
    ///
    /// Sha t_string (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub const fn with_t_string(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TString::T_STRING_MASK)
                | ((value << TString::T_STRING_OFFSET) & TString::T_STRING_MASK),
        }
    }
}

impl Default for TStringVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TString {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TStringVal {
        TStringVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TStringVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TStringVal) -> TStringVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T_LENGTH Register
///
/// SHA 512/t configuration register 1.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **T_LENGTH** `[5:0]` (RW)
///
///   Sha t_length (used if and only if mode == SHA_512/t).
#[repr(transparent)]
pub struct TLength {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TLength {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TLength {
    // ======== Field Constants ========

    /// **T_LENGTH**
    ///
    /// Sha t_length (used if and only if mode == SHA_512/t).
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const T_LENGTH_OFFSET: u32 = 0;
    pub const T_LENGTH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **T_LENGTH** `[5:0]` (RW)
    ///
    /// Sha t_length (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub fn t_length(&self) -> u32 {
        (self.value.read_bits() & Self::T_LENGTH_MASK) >> Self::T_LENGTH_OFFSET
    }

    /// Write **T_LENGTH** `[5:0]` (RW)
    ///
    /// Sha t_length (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub fn write_t_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T_LENGTH_MASK) | ((value << Self::T_LENGTH_OFFSET) & Self::T_LENGTH_MASK)
        });
    }
}

/// T_LENGTH value type for batch read/modify/write operations
///
/// Use `tlength.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tlength.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TLengthVal {
    bits: u32,
}

impl TLengthVal {
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

    /// Read **T_LENGTH** `[5:0]`
    ///
    /// Sha t_length (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub const fn t_length(&self) -> u32 {
        (self.bits & TLength::T_LENGTH_MASK) >> TLength::T_LENGTH_OFFSET
    }

    /// Set **T_LENGTH** `[5:0]`
    ///
    /// Sha t_length (used if and only if mode == SHA_512/t).
    #[inline(always)]
    pub const fn with_t_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TLength::T_LENGTH_MASK)
                | ((value << TLength::T_LENGTH_OFFSET) & TLength::T_LENGTH_MASK),
        }
    }
}

impl Default for TLengthVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TLength {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TLengthVal {
        TLengthVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TLengthVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TLengthVal) -> TLengthVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_BLOCK_NUM Register
///
/// DMA configuration register 0.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **DMA_BLOCK_NUM** `[5:0]` (RW)
///
///   Dma-sha block number.
#[repr(transparent)]
pub struct DmaBlockNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaBlockNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaBlockNum {
    // ======== Field Constants ========

    /// **DMA_BLOCK_NUM**
    ///
    /// Dma-sha block number.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const DMA_BLOCK_NUM_OFFSET: u32 = 0;
    pub const DMA_BLOCK_NUM_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **DMA_BLOCK_NUM** `[5:0]` (RW)
    ///
    /// Dma-sha block number.
    #[inline(always)]
    pub fn dma_block_num(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_BLOCK_NUM_MASK) >> Self::DMA_BLOCK_NUM_OFFSET
    }

    /// Write **DMA_BLOCK_NUM** `[5:0]` (RW)
    ///
    /// Dma-sha block number.
    #[inline(always)]
    pub fn write_dma_block_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_BLOCK_NUM_MASK)
                | ((value << Self::DMA_BLOCK_NUM_OFFSET) & Self::DMA_BLOCK_NUM_MASK)
        });
    }
}

/// DMA_BLOCK_NUM value type for batch read/modify/write operations
///
/// Use `dmablocknum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmablocknum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaBlockNumVal {
    bits: u32,
}

impl DmaBlockNumVal {
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

    /// Read **DMA_BLOCK_NUM** `[5:0]`
    ///
    /// Dma-sha block number.
    #[inline(always)]
    pub const fn dma_block_num(&self) -> u32 {
        (self.bits & DmaBlockNum::DMA_BLOCK_NUM_MASK) >> DmaBlockNum::DMA_BLOCK_NUM_OFFSET
    }

    /// Set **DMA_BLOCK_NUM** `[5:0]`
    ///
    /// Dma-sha block number.
    #[inline(always)]
    pub const fn with_dma_block_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaBlockNum::DMA_BLOCK_NUM_MASK)
                | ((value << DmaBlockNum::DMA_BLOCK_NUM_OFFSET) & DmaBlockNum::DMA_BLOCK_NUM_MASK),
        }
    }
}

impl Default for DmaBlockNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaBlockNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaBlockNumVal {
        DmaBlockNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaBlockNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaBlockNumVal) -> DmaBlockNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// START Register
///
/// Typical SHA configuration register 0.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **START** `[31:1]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct Start {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Start {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Start {
    // ======== Field Constants ========

    /// **START**
    ///
    /// Reserved.
    ///
    /// Bits `[31:1]`, mask `0xfffffffe`
    pub const START_OFFSET: u32 = 1;
    pub const START_MASK: u32 = 0xfffffffe;

    // ======== Field Accessors ========

    /// **START** `[31:1]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn start(&self) -> u32 {
        (self.value.read_bits() & Self::START_MASK) >> Self::START_OFFSET
    }
}

/// START value type for batch read/modify/write operations
///
/// Use `start.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `start.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StartVal {
    bits: u32,
}

impl StartVal {
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

    /// Read **START** `[31:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn start(&self) -> u32 {
        (self.bits & Start::START_MASK) >> Start::START_OFFSET
    }
}

impl Default for StartVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Start {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StartVal {
        StartVal::from_bits(self.value.read_bits())
    }
}

/// CONTINUE Register
///
/// Typical SHA configuration register 1.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CONTINUE** `[31:1]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct Continue {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Continue {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Continue {
    // ======== Field Constants ========

    /// **CONTINUE**
    ///
    /// Reserved.
    ///
    /// Bits `[31:1]`, mask `0xfffffffe`
    pub const CONTINUE_OFFSET: u32 = 1;
    pub const CONTINUE_MASK: u32 = 0xfffffffe;

    // ======== Field Accessors ========

    /// **CONTINUE** `[31:1]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn r#continue(&self) -> u32 {
        (self.value.read_bits() & Self::CONTINUE_MASK) >> Self::CONTINUE_OFFSET
    }
}

/// CONTINUE value type for batch read/modify/write operations
///
/// Use `continue.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `continue.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ContinueVal {
    bits: u32,
}

impl ContinueVal {
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

    /// Read **CONTINUE** `[31:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn r#continue(&self) -> u32 {
        (self.bits & Continue::CONTINUE_MASK) >> Continue::CONTINUE_OFFSET
    }
}

impl Default for ContinueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Continue {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ContinueVal {
        ContinueVal::from_bits(self.value.read_bits())
    }
}

/// BUSY Register
///
/// Busy register.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **STATE** `[0]` (RO)
///
///   Sha busy state. 1'b0: idle. 1'b1: busy.
#[repr(transparent)]
pub struct Busy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Busy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Busy {
    // ======== Field Constants ========

    /// **STATE**
    ///
    /// Sha busy state. 1'b0: idle. 1'b1: busy.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const STATE_OFFSET: u32 = 0;
    pub const STATE_MASK: u32 = 0x00000001;
    pub const STATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **STATE** `[0]` (RO)
    ///
    /// Sha busy state. 1'b0: idle. 1'b1: busy.
    #[inline(always)]
    pub fn state(&self) -> bool {
        (self.value.read_bits() & Self::STATE_MASK) != 0
    }
}

/// BUSY value type for batch read/modify/write operations
///
/// Use `busy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `busy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusyVal {
    bits: u32,
}

impl BusyVal {
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

    /// Read **STATE** `[0:0]`
    ///
    /// Sha busy state. 1'b0: idle. 1'b1: busy.
    #[inline(always)]
    pub const fn state(&self) -> bool {
        (self.bits & Busy::STATE_MASK) != 0
    }
}

impl Default for BusyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Busy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusyVal {
        BusyVal::from_bits(self.value.read_bits())
    }
}

/// DMA_START Register
///
/// DMA configuration register 1.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **DMA_START** `[0]` (WO)
///
///   Start dma-sha.
#[repr(transparent)]
pub struct DmaStart {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaStart {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaStart {
    // ======== Field Constants ========

    /// **DMA_START**
    ///
    /// Start dma-sha.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMA_START_OFFSET: u32 = 0;
    pub const DMA_START_MASK: u32 = 0x00000001;
    pub const DMA_START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DMA_START** (WO)
    ///
    /// Start dma-sha.
    #[inline(always)]
    pub fn trigger_dma_start(&self) {
        self.value.write_bits(Self::DMA_START_MASK);
    }
}

/// DMA_START value type for batch read/modify/write operations
///
/// Use `dmastart.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmastart.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaStartVal {
    bits: u32,
}

impl DmaStartVal {
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

    /// Set **DMA_START** `[0:0]`
    ///
    /// Start dma-sha.
    #[inline(always)]
    pub const fn with_dma_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaStart::DMA_START_MASK
            } else {
                self.bits & !DmaStart::DMA_START_MASK
            },
        }
    }
}

impl Default for DmaStartVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaStart {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaStartVal) {
        self.value.write_bits(val.bits())
    }
}

/// DMA_CONTINUE Register
///
/// DMA configuration register 2.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **DMA_CONTINUE** `[0]` (WO)
///
///   Continue dma-sha.
#[repr(transparent)]
pub struct DmaContinue {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaContinue {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaContinue {
    // ======== Field Constants ========

    /// **DMA_CONTINUE**
    ///
    /// Continue dma-sha.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMA_CONTINUE_OFFSET: u32 = 0;
    pub const DMA_CONTINUE_MASK: u32 = 0x00000001;
    pub const DMA_CONTINUE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DMA_CONTINUE** (WO)
    ///
    /// Continue dma-sha.
    #[inline(always)]
    pub fn trigger_dma_continue(&self) {
        self.value.write_bits(Self::DMA_CONTINUE_MASK);
    }
}

/// DMA_CONTINUE value type for batch read/modify/write operations
///
/// Use `dmacontinue.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmacontinue.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaContinueVal {
    bits: u32,
}

impl DmaContinueVal {
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

    /// Set **DMA_CONTINUE** `[0:0]`
    ///
    /// Continue dma-sha.
    #[inline(always)]
    pub const fn with_dma_continue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaContinue::DMA_CONTINUE_MASK
            } else {
                self.bits & !DmaContinue::DMA_CONTINUE_MASK
            },
        }
    }
}

impl Default for DmaContinueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaContinue {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaContinueVal) {
        self.value.write_bits(val.bits())
    }
}

/// CLEAR_IRQ Register
///
/// Interrupt clear register.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **CLEAR_INTERRUPT** `[0]` (WO)
///
///   Clear sha interrupt.
#[repr(transparent)]
pub struct ClearIrq {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClearIrq {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClearIrq {
    // ======== Field Constants ========

    /// **CLEAR_INTERRUPT**
    ///
    /// Clear sha interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLEAR_INTERRUPT_OFFSET: u32 = 0;
    pub const CLEAR_INTERRUPT_MASK: u32 = 0x00000001;
    pub const CLEAR_INTERRUPT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CLEAR_INTERRUPT** (WO)
    ///
    /// Clear sha interrupt.
    #[inline(always)]
    pub fn trigger_clear_interrupt(&self) {
        self.value.write_bits(Self::CLEAR_INTERRUPT_MASK);
    }
}

/// CLEAR_IRQ value type for batch read/modify/write operations
///
/// Use `clearirq.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clearirq.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClearIrqVal {
    bits: u32,
}

impl ClearIrqVal {
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

    /// Set **CLEAR_INTERRUPT** `[0:0]`
    ///
    /// Clear sha interrupt.
    #[inline(always)]
    pub const fn with_clear_interrupt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClearIrq::CLEAR_INTERRUPT_MASK
            } else {
                self.bits & !ClearIrq::CLEAR_INTERRUPT_MASK
            },
        }
    }
}

impl Default for ClearIrqVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClearIrq {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClearIrqVal) {
        self.value.write_bits(val.bits())
    }
}

/// IRQ_ENA Register
///
/// Interrupt enable register.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **INTERRUPT_ENA** `[0]` (RW)
///
///   Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable.
#[repr(transparent)]
pub struct IrqEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IrqEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IrqEna {
    // ======== Field Constants ========

    /// **INTERRUPT_ENA**
    ///
    /// Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INTERRUPT_ENA_OFFSET: u32 = 0;
    pub const INTERRUPT_ENA_MASK: u32 = 0x00000001;
    pub const INTERRUPT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INTERRUPT_ENA** `[0]` (RW)
    ///
    /// Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable.
    #[inline(always)]
    pub fn interrupt_ena(&self) -> bool {
        (self.value.read_bits() & Self::INTERRUPT_ENA_MASK) != 0
    }

    /// Set INTERRUPT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_interrupt_ena(&self) {
        self.value.modify_bits(|r| r | Self::INTERRUPT_ENA_MASK);
    }

    /// Clear INTERRUPT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_interrupt_ena(&self) {
        self.value.modify_bits(|r| r & !Self::INTERRUPT_ENA_MASK);
    }
}

/// IRQ_ENA value type for batch read/modify/write operations
///
/// Use `irqena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqEnaVal {
    bits: u32,
}

impl IrqEnaVal {
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

    /// Read **INTERRUPT_ENA** `[0:0]`
    ///
    /// Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable.
    #[inline(always)]
    pub const fn interrupt_ena(&self) -> bool {
        (self.bits & IrqEna::INTERRUPT_ENA_MASK) != 0
    }

    /// Set **INTERRUPT_ENA** `[0:0]`
    ///
    /// Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable.
    #[inline(always)]
    pub const fn with_interrupt_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IrqEna::INTERRUPT_ENA_MASK
            } else {
                self.bits & !IrqEna::INTERRUPT_ENA_MASK
            },
        }
    }
}

impl Default for IrqEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IrqEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IrqEnaVal {
        IrqEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IrqEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IrqEnaVal) -> IrqEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Date register.
///
/// Address offset: 0x002c
/// Reset value: 0x20201229
///
/// ## Fields
///
/// - **DATE** `[29:0]` (RW)
///
///   Sha date information/ sha version information.
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
    /// Sha date information/ sha version information.
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[29:0]` (RW)
    ///
    /// Sha date information/ sha version information.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[29:0]` (RW)
    ///
    /// Sha date information/ sha version information.
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
        Self { bits: 0x20201229 }
    }

    /// Read **DATE** `[29:0]`
    ///
    /// Sha date information/ sha version information.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[29:0]`
    ///
    /// Sha date information/ sha version information.
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

/// H_MEM[0] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0040
#[repr(transparent)]
pub struct HMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem0 {}

/// H_MEM[1] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0044
#[repr(transparent)]
pub struct HMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem1 {}

/// H_MEM[2] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0048
#[repr(transparent)]
pub struct HMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem2 {}

/// H_MEM[3] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x004c
#[repr(transparent)]
pub struct HMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem3 {}

/// H_MEM[4] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0050
#[repr(transparent)]
pub struct HMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem4 {}

/// H_MEM[5] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0054
#[repr(transparent)]
pub struct HMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem5 {}

/// H_MEM[6] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0058
#[repr(transparent)]
pub struct HMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem6 {}

/// H_MEM[7] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x005c
#[repr(transparent)]
pub struct HMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem7 {}

/// H_MEM[8] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0060
#[repr(transparent)]
pub struct HMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem8 {}

/// H_MEM[9] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0064
#[repr(transparent)]
pub struct HMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem9 {}

/// H_MEM[10] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0068
#[repr(transparent)]
pub struct HMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem10 {}

/// H_MEM[11] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x006c
#[repr(transparent)]
pub struct HMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem11 {}

/// H_MEM[12] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0070
#[repr(transparent)]
pub struct HMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem12 {}

/// H_MEM[13] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0074
#[repr(transparent)]
pub struct HMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem13 {}

/// H_MEM[14] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x0078
#[repr(transparent)]
pub struct HMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem14 {}

/// H_MEM[15] Register
///
/// Sha H memory which contains intermediate hash or finial hash.
///
/// Address offset: 0x007c
#[repr(transparent)]
pub struct HMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem15 {}

/// M_MEM[0] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0080
#[repr(transparent)]
pub struct MMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem0 {}

/// M_MEM[1] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0084
#[repr(transparent)]
pub struct MMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem1 {}

/// M_MEM[2] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0088
#[repr(transparent)]
pub struct MMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem2 {}

/// M_MEM[3] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x008c
#[repr(transparent)]
pub struct MMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem3 {}

/// M_MEM[4] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0090
#[repr(transparent)]
pub struct MMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem4 {}

/// M_MEM[5] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0094
#[repr(transparent)]
pub struct MMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem5 {}

/// M_MEM[6] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x0098
#[repr(transparent)]
pub struct MMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem6 {}

/// M_MEM[7] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x009c
#[repr(transparent)]
pub struct MMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem7 {}

/// M_MEM[8] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00a0
#[repr(transparent)]
pub struct MMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem8 {}

/// M_MEM[9] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00a4
#[repr(transparent)]
pub struct MMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem9 {}

/// M_MEM[10] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00a8
#[repr(transparent)]
pub struct MMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem10 {}

/// M_MEM[11] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00ac
#[repr(transparent)]
pub struct MMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem11 {}

/// M_MEM[12] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00b0
#[repr(transparent)]
pub struct MMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem12 {}

/// M_MEM[13] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00b4
#[repr(transparent)]
pub struct MMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem13 {}

/// M_MEM[14] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00b8
#[repr(transparent)]
pub struct MMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem14 {}

/// M_MEM[15] Register
///
/// Sha M memory which contains message.
///
/// Address offset: 0x00bc
#[repr(transparent)]
pub struct MMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem15 {}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Initial configuration register.
    pub mode: Mode,
    /// 0x0004
    /// SHA 512/t configuration register 0.
    pub t_string: TString,
    /// 0x0008
    /// SHA 512/t configuration register 1.
    pub t_length: TLength,
    /// 0x000c
    /// DMA configuration register 0.
    pub dma_block_num: DmaBlockNum,
    /// 0x0010
    /// Typical SHA configuration register 0.
    pub start: Start,
    /// 0x0014
    /// Typical SHA configuration register 1.
    pub r#continue: Continue,
    /// 0x0018
    /// Busy register.
    pub busy: Busy,
    /// 0x001c
    /// DMA configuration register 1.
    pub dma_start: DmaStart,
    /// 0x0020
    /// DMA configuration register 2.
    pub dma_continue: DmaContinue,
    /// 0x0024
    /// Interrupt clear register.
    pub clear_irq: ClearIrq,
    /// 0x0028
    /// Interrupt enable register.
    pub irq_ena: IrqEna,
    /// 0x002c
    /// Date register.
    pub date: Date,
    _reserved0: [u8; 16],
    /// 0x0040
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem0: HMem0,
    /// 0x0044
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem1: HMem1,
    /// 0x0048
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem2: HMem2,
    /// 0x004c
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem3: HMem3,
    /// 0x0050
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem4: HMem4,
    /// 0x0054
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem5: HMem5,
    /// 0x0058
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem6: HMem6,
    /// 0x005c
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem7: HMem7,
    /// 0x0060
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem8: HMem8,
    /// 0x0064
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem9: HMem9,
    /// 0x0068
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem10: HMem10,
    /// 0x006c
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem11: HMem11,
    /// 0x0070
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem12: HMem12,
    /// 0x0074
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem13: HMem13,
    /// 0x0078
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem14: HMem14,
    /// 0x007c
    /// Sha H memory which contains intermediate hash or finial hash.
    pub h_mem15: HMem15,
    /// 0x0080
    /// Sha M memory which contains message.
    pub m_mem0: MMem0,
    /// 0x0084
    /// Sha M memory which contains message.
    pub m_mem1: MMem1,
    /// 0x0088
    /// Sha M memory which contains message.
    pub m_mem2: MMem2,
    /// 0x008c
    /// Sha M memory which contains message.
    pub m_mem3: MMem3,
    /// 0x0090
    /// Sha M memory which contains message.
    pub m_mem4: MMem4,
    /// 0x0094
    /// Sha M memory which contains message.
    pub m_mem5: MMem5,
    /// 0x0098
    /// Sha M memory which contains message.
    pub m_mem6: MMem6,
    /// 0x009c
    /// Sha M memory which contains message.
    pub m_mem7: MMem7,
    /// 0x00a0
    /// Sha M memory which contains message.
    pub m_mem8: MMem8,
    /// 0x00a4
    /// Sha M memory which contains message.
    pub m_mem9: MMem9,
    /// 0x00a8
    /// Sha M memory which contains message.
    pub m_mem10: MMem10,
    /// 0x00ac
    /// Sha M memory which contains message.
    pub m_mem11: MMem11,
    /// 0x00b0
    /// Sha M memory which contains message.
    pub m_mem12: MMem12,
    /// 0x00b4
    /// Sha M memory which contains message.
    pub m_mem13: MMem13,
    /// 0x00b8
    /// Sha M memory which contains message.
    pub m_mem14: MMem14,
    /// 0x00bc
    /// Sha M memory which contains message.
    pub m_mem15: MMem15,
}
