//! RMT
//! Remote Control

#![allow(dead_code)]

pub const BASE: u32 = 0x500d4000;

/// TX_CH0DATA Register
///
/// The read and write data register for CHANNEL0 by apb fifo access.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel %s via APB FIFO.
#[repr(transparent)]
pub struct TxCh0data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh0data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh0data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel %s via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// TX_CH0DATA value type for batch read/modify/write operations
///
/// Use `txch0data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch0data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh0dataVal {
    bits: u32,
}

impl TxCh0dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & TxCh0data::CHDATA_MASK) >> TxCh0data::CHDATA_OFFSET
    }
}

impl Default for TxCh0dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh0data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh0dataVal {
        TxCh0dataVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH1DATA Register
///
/// The read and write data register for CHANNEL1 by apb fifo access.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel %s via APB FIFO.
#[repr(transparent)]
pub struct TxCh1data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh1data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh1data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel %s via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// TX_CH1DATA value type for batch read/modify/write operations
///
/// Use `txch1data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch1data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh1dataVal {
    bits: u32,
}

impl TxCh1dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & TxCh1data::CHDATA_MASK) >> TxCh1data::CHDATA_OFFSET
    }
}

impl Default for TxCh1dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh1data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh1dataVal {
        TxCh1dataVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH2DATA Register
///
/// The read and write data register for CHANNEL2 by apb fifo access.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel %s via APB FIFO.
#[repr(transparent)]
pub struct TxCh2data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh2data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh2data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel %s via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// TX_CH2DATA value type for batch read/modify/write operations
///
/// Use `txch2data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch2data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh2dataVal {
    bits: u32,
}

impl TxCh2dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & TxCh2data::CHDATA_MASK) >> TxCh2data::CHDATA_OFFSET
    }
}

impl Default for TxCh2dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh2data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh2dataVal {
        TxCh2dataVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH3DATA Register
///
/// The read and write data register for CHANNEL3 by apb fifo access.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel %s via APB FIFO.
#[repr(transparent)]
pub struct TxCh3data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh3data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh3data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel %s via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// TX_CH3DATA value type for batch read/modify/write operations
///
/// Use `txch3data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch3data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh3dataVal {
    bits: u32,
}

impl TxCh3dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & TxCh3data::CHDATA_MASK) >> TxCh3data::CHDATA_OFFSET
    }
}

impl Default for TxCh3dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh3data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh3dataVal {
        TxCh3dataVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH0DATA Register
///
/// The read and write data register for CHANNEL$n by apb fifo access.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel 0 via APB FIFO.
#[repr(transparent)]
pub struct RxCh0data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh0data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh0data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel 0 via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// RX_CH0DATA value type for batch read/modify/write operations
///
/// Use `rxch0data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch0data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh0dataVal {
    bits: u32,
}

impl RxCh0dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & RxCh0data::CHDATA_MASK) >> RxCh0data::CHDATA_OFFSET
    }
}

impl Default for RxCh0dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCh0data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh0dataVal {
        RxCh0dataVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH1DATA Register
///
/// The read and write data register for CHANNEL$n by apb fifo access.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel 0 via APB FIFO.
#[repr(transparent)]
pub struct RxCh1data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh1data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh1data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel 0 via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// RX_CH1DATA value type for batch read/modify/write operations
///
/// Use `rxch1data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch1data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh1dataVal {
    bits: u32,
}

impl RxCh1dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & RxCh1data::CHDATA_MASK) >> RxCh1data::CHDATA_OFFSET
    }
}

impl Default for RxCh1dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCh1data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh1dataVal {
        RxCh1dataVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH2DATA Register
///
/// The read and write data register for CHANNEL$n by apb fifo access.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel 0 via APB FIFO.
#[repr(transparent)]
pub struct RxCh2data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh2data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh2data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel 0 via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// RX_CH2DATA value type for batch read/modify/write operations
///
/// Use `rxch2data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch2data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh2dataVal {
    bits: u32,
}

impl RxCh2dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & RxCh2data::CHDATA_MASK) >> RxCh2data::CHDATA_OFFSET
    }
}

impl Default for RxCh2dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCh2data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh2dataVal {
        RxCh2dataVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH3DATA Register
///
/// The read and write data register for CHANNEL$n by apb fifo access.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **CHDATA** `[31:0]` (RO)
///
///   Read and write data for channel 0 via APB FIFO.
#[repr(transparent)]
pub struct RxCh3data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh3data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh3data {
    // ======== Field Constants ========

    /// **CHDATA**
    ///
    /// Read and write data for channel 0 via APB FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHDATA_OFFSET: u32 = 0;
    pub const CHDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHDATA** `[31:0]` (RO)
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub fn chdata(&self) -> u32 {
        (self.value.read_bits() & Self::CHDATA_MASK) >> Self::CHDATA_OFFSET
    }
}

/// RX_CH3DATA value type for batch read/modify/write operations
///
/// Use `rxch3data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch3data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh3dataVal {
    bits: u32,
}

impl RxCh3dataVal {
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

    /// Read **CHDATA** `[31:0]`
    ///
    /// Read and write data for channel 0 via APB FIFO.
    #[inline(always)]
    pub const fn chdata(&self) -> u32 {
        (self.bits & RxCh3data::CHDATA_MASK) >> RxCh3data::CHDATA_OFFSET
    }
}

impl Default for RxCh3dataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCh3data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh3dataVal {
        RxCh3dataVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH0CONF0 Register
///
/// Channel 0 configure register 0
///
/// Address offset: 0x0020
/// Reset value: 0x00710200
///
/// ## Fields
///
/// - **CONF_UPDATE_CH0** `[24]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH0** `[23]` (WO)
///
///   Reserved
///
/// - **CARRIER_OUT_LV_CH0** `[22]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH0** `[21]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **CARRIER_EFF_EN_CH0** `[20]` (RW)
///
///   1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
///
/// - **MEM_SIZE_CH0** `[19:16]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **DIV_CNT_CH0** `[15:8]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
///
/// - **TX_STOP_CH0** `[7]` (RW)
///
///   Set this bit to stop the transmitter of CHANNEL%s sending data out.
///
/// - **IDLE_OUT_EN_CH0** `[6]` (RW)
///
///   This is the output enable-control bit for CHANNEL%s in IDLE state.
///
/// - **IDLE_OUT_LV_CH0** `[5]` (RW)
///
///   This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
///
/// - **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
///
/// - **TX_CONTI_MODE_CH0** `[3]` (RW)
///
///   Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
///
/// - **APB_MEM_RST_CH0** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_RD_RST_CH0** `[1]` (WO)
///
///   Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
///
/// - **TX_START_CH0** `[0]` (WO)
///
///   Set this bit to start sending data on CHANNEL%s.
#[repr(transparent)]
pub struct TxCh0conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh0conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh0conf0 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH0**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CONF_UPDATE_CH0_OFFSET: u32 = 24;
    pub const CONF_UPDATE_CH0_MASK: u32 = 0x01000000;
    pub const CONF_UPDATE_CH0: u32 = 1 << 24;

    /// **AFIFO_RST_CH0**
    ///
    /// Reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const AFIFO_RST_CH0_OFFSET: u32 = 23;
    pub const AFIFO_RST_CH0_MASK: u32 = 0x00800000;
    pub const AFIFO_RST_CH0: u32 = 1 << 23;

    /// **CARRIER_OUT_LV_CH0**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CARRIER_OUT_LV_CH0_OFFSET: u32 = 22;
    pub const CARRIER_OUT_LV_CH0_MASK: u32 = 0x00400000;
    pub const CARRIER_OUT_LV_CH0: u32 = 1 << 22;

    /// **CARRIER_EN_CH0**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CARRIER_EN_CH0_OFFSET: u32 = 21;
    pub const CARRIER_EN_CH0_MASK: u32 = 0x00200000;
    pub const CARRIER_EN_CH0: u32 = 1 << 21;

    /// **CARRIER_EFF_EN_CH0**
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CARRIER_EFF_EN_CH0_OFFSET: u32 = 20;
    pub const CARRIER_EFF_EN_CH0_MASK: u32 = 0x00100000;
    pub const CARRIER_EFF_EN_CH0: u32 = 1 << 20;

    /// **MEM_SIZE_CH0**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const MEM_SIZE_CH0_OFFSET: u32 = 16;
    pub const MEM_SIZE_CH0_MASK: u32 = 0x000f0000;

    /// **DIV_CNT_CH0**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DIV_CNT_CH0_OFFSET: u32 = 8;
    pub const DIV_CNT_CH0_MASK: u32 = 0x0000ff00;

    /// **TX_STOP_CH0**
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_STOP_CH0_OFFSET: u32 = 7;
    pub const TX_STOP_CH0_MASK: u32 = 0x00000080;
    pub const TX_STOP_CH0: u32 = 1 << 7;

    /// **IDLE_OUT_EN_CH0**
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDLE_OUT_EN_CH0_OFFSET: u32 = 6;
    pub const IDLE_OUT_EN_CH0_MASK: u32 = 0x00000040;
    pub const IDLE_OUT_EN_CH0: u32 = 1 << 6;

    /// **IDLE_OUT_LV_CH0**
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDLE_OUT_LV_CH0_OFFSET: u32 = 5;
    pub const IDLE_OUT_LV_CH0_MASK: u32 = 0x00000020;
    pub const IDLE_OUT_LV_CH0: u32 = 1 << 5;

    /// **MEM_TX_WRAP_EN_CH0**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TX_WRAP_EN_CH0_OFFSET: u32 = 4;
    pub const MEM_TX_WRAP_EN_CH0_MASK: u32 = 0x00000010;
    pub const MEM_TX_WRAP_EN_CH0: u32 = 1 << 4;

    /// **TX_CONTI_MODE_CH0**
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_CONTI_MODE_CH0_OFFSET: u32 = 3;
    pub const TX_CONTI_MODE_CH0_MASK: u32 = 0x00000008;
    pub const TX_CONTI_MODE_CH0: u32 = 1 << 3;

    /// **APB_MEM_RST_CH0**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH0_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH0_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH0: u32 = 1 << 2;

    /// **MEM_RD_RST_CH0**
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_RD_RST_CH0_OFFSET: u32 = 1;
    pub const MEM_RD_RST_CH0_MASK: u32 = 0x00000002;
    pub const MEM_RD_RST_CH0: u32 = 1 << 1;

    /// **TX_START_CH0**
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_START_CH0_OFFSET: u32 = 0;
    pub const TX_START_CH0_MASK: u32 = 0x00000001;
    pub const TX_START_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH0** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH0_MASK);
    }

    /// Trigger **AFIFO_RST_CH0** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH0_MASK);
    }

    /// **CARRIER_OUT_LV_CH0** `[22]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// **CARRIER_EN_CH0** `[21]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH0_MASK);
    }

    /// Clear CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH0_MASK);
    }

    /// **CARRIER_EFF_EN_CH0** `[20]` (RW)
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// Clear CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH0_MASK) >> Self::MEM_SIZE_CH0_OFFSET
    }

    /// Write **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH0_MASK)
                | ((value << Self::MEM_SIZE_CH0_OFFSET) & Self::MEM_SIZE_CH0_MASK)
        });
    }

    /// **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH0_MASK) >> Self::DIV_CNT_CH0_OFFSET
    }

    /// Write **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH0_MASK)
                | ((value << Self::DIV_CNT_CH0_OFFSET) & Self::DIV_CNT_CH0_MASK)
        });
    }

    /// **TX_STOP_CH0** `[7]` (RW)
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_STOP_CH0_MASK) != 0
    }

    /// Set TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_STOP_CH0_MASK);
    }

    /// Clear TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::TX_STOP_CH0_MASK);
    }

    /// **IDLE_OUT_EN_CH0** `[6]` (RW)
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// Clear IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// **IDLE_OUT_LV_CH0** `[5]` (RW)
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// Clear IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Set MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// Clear MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// **TX_CONTI_MODE_CH0** `[3]` (RW)
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_conti_mode_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Clear TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_conti_mode_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Trigger **APB_MEM_RST_CH0** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH0_MASK);
    }

    /// Trigger **MEM_RD_RST_CH0** (WO)
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub fn trigger_mem_rd_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::MEM_RD_RST_CH0_MASK);
    }

    /// Trigger **TX_START_CH0** (WO)
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub fn trigger_tx_start_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_CH0_MASK);
    }
}

/// TX_CH0CONF0 value type for batch read/modify/write operations
///
/// Use `txch0conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch0conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh0conf0Val {
    bits: u32,
}

impl TxCh0conf0Val {
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
        Self { bits: 0x00710200 }
    }

    /// Read **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Read **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::CARRIER_EN_CH0_MASK) != 0
    }

    /// Read **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn carrier_eff_en_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Read **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch0(&self) -> u32 {
        (self.bits & TxCh0conf0::MEM_SIZE_CH0_MASK) >> TxCh0conf0::MEM_SIZE_CH0_OFFSET
    }

    /// Read **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch0(&self) -> u32 {
        (self.bits & TxCh0conf0::DIV_CNT_CH0_MASK) >> TxCh0conf0::DIV_CNT_CH0_OFFSET
    }

    /// Read **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn tx_stop_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::TX_STOP_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn idle_out_en_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn idle_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Read **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Read **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn tx_conti_mode_ch0(&self) -> bool {
        (self.bits & TxCh0conf0::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH0** `[24:24]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::CONF_UPDATE_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::CONF_UPDATE_CH0_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH0** `[23:23]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::AFIFO_RST_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::AFIFO_RST_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::CARRIER_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::CARRIER_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::CARRIER_EN_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::CARRIER_EN_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn with_carrier_eff_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::CARRIER_EFF_EN_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::CARRIER_EFF_EN_CH0_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh0conf0::MEM_SIZE_CH0_MASK)
                | ((value << TxCh0conf0::MEM_SIZE_CH0_OFFSET) & TxCh0conf0::MEM_SIZE_CH0_MASK),
        }
    }

    /// Set **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh0conf0::DIV_CNT_CH0_MASK)
                | ((value << TxCh0conf0::DIV_CNT_CH0_OFFSET) & TxCh0conf0::DIV_CNT_CH0_MASK),
        }
    }

    /// Set **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn with_tx_stop_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::TX_STOP_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::TX_STOP_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::IDLE_OUT_EN_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::IDLE_OUT_EN_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::IDLE_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::IDLE_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_tx_wrap_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::MEM_TX_WRAP_EN_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::MEM_TX_WRAP_EN_CH0_MASK
            },
        }
    }

    /// Set **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_conti_mode_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::TX_CONTI_MODE_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::TX_CONTI_MODE_CH0_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH0** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::APB_MEM_RST_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::APB_MEM_RST_CH0_MASK
            },
        }
    }

    /// Set **MEM_RD_RST_CH0** `[1:1]`
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub const fn with_mem_rd_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::MEM_RD_RST_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::MEM_RD_RST_CH0_MASK
            },
        }
    }

    /// Set **TX_START_CH0** `[0:0]`
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_start_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh0conf0::TX_START_CH0_MASK
            } else {
                self.bits & !TxCh0conf0::TX_START_CH0_MASK
            },
        }
    }
}

impl Default for TxCh0conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxCh0conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh0conf0Val {
        TxCh0conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCh0conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCh0conf0Val) -> TxCh0conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH1CONF0 Register
///
/// Channel 1 configure register 0
///
/// Address offset: 0x0024
/// Reset value: 0x00710200
///
/// ## Fields
///
/// - **CONF_UPDATE_CH0** `[24]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH0** `[23]` (WO)
///
///   Reserved
///
/// - **CARRIER_OUT_LV_CH0** `[22]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH0** `[21]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **CARRIER_EFF_EN_CH0** `[20]` (RW)
///
///   1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
///
/// - **MEM_SIZE_CH0** `[19:16]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **DIV_CNT_CH0** `[15:8]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
///
/// - **TX_STOP_CH0** `[7]` (RW)
///
///   Set this bit to stop the transmitter of CHANNEL%s sending data out.
///
/// - **IDLE_OUT_EN_CH0** `[6]` (RW)
///
///   This is the output enable-control bit for CHANNEL%s in IDLE state.
///
/// - **IDLE_OUT_LV_CH0** `[5]` (RW)
///
///   This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
///
/// - **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
///
/// - **TX_CONTI_MODE_CH0** `[3]` (RW)
///
///   Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
///
/// - **APB_MEM_RST_CH0** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_RD_RST_CH0** `[1]` (WO)
///
///   Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
///
/// - **TX_START_CH0** `[0]` (WO)
///
///   Set this bit to start sending data on CHANNEL%s.
#[repr(transparent)]
pub struct TxCh1conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh1conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh1conf0 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH0**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CONF_UPDATE_CH0_OFFSET: u32 = 24;
    pub const CONF_UPDATE_CH0_MASK: u32 = 0x01000000;
    pub const CONF_UPDATE_CH0: u32 = 1 << 24;

    /// **AFIFO_RST_CH0**
    ///
    /// Reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const AFIFO_RST_CH0_OFFSET: u32 = 23;
    pub const AFIFO_RST_CH0_MASK: u32 = 0x00800000;
    pub const AFIFO_RST_CH0: u32 = 1 << 23;

    /// **CARRIER_OUT_LV_CH0**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CARRIER_OUT_LV_CH0_OFFSET: u32 = 22;
    pub const CARRIER_OUT_LV_CH0_MASK: u32 = 0x00400000;
    pub const CARRIER_OUT_LV_CH0: u32 = 1 << 22;

    /// **CARRIER_EN_CH0**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CARRIER_EN_CH0_OFFSET: u32 = 21;
    pub const CARRIER_EN_CH0_MASK: u32 = 0x00200000;
    pub const CARRIER_EN_CH0: u32 = 1 << 21;

    /// **CARRIER_EFF_EN_CH0**
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CARRIER_EFF_EN_CH0_OFFSET: u32 = 20;
    pub const CARRIER_EFF_EN_CH0_MASK: u32 = 0x00100000;
    pub const CARRIER_EFF_EN_CH0: u32 = 1 << 20;

    /// **MEM_SIZE_CH0**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const MEM_SIZE_CH0_OFFSET: u32 = 16;
    pub const MEM_SIZE_CH0_MASK: u32 = 0x000f0000;

    /// **DIV_CNT_CH0**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DIV_CNT_CH0_OFFSET: u32 = 8;
    pub const DIV_CNT_CH0_MASK: u32 = 0x0000ff00;

    /// **TX_STOP_CH0**
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_STOP_CH0_OFFSET: u32 = 7;
    pub const TX_STOP_CH0_MASK: u32 = 0x00000080;
    pub const TX_STOP_CH0: u32 = 1 << 7;

    /// **IDLE_OUT_EN_CH0**
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDLE_OUT_EN_CH0_OFFSET: u32 = 6;
    pub const IDLE_OUT_EN_CH0_MASK: u32 = 0x00000040;
    pub const IDLE_OUT_EN_CH0: u32 = 1 << 6;

    /// **IDLE_OUT_LV_CH0**
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDLE_OUT_LV_CH0_OFFSET: u32 = 5;
    pub const IDLE_OUT_LV_CH0_MASK: u32 = 0x00000020;
    pub const IDLE_OUT_LV_CH0: u32 = 1 << 5;

    /// **MEM_TX_WRAP_EN_CH0**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TX_WRAP_EN_CH0_OFFSET: u32 = 4;
    pub const MEM_TX_WRAP_EN_CH0_MASK: u32 = 0x00000010;
    pub const MEM_TX_WRAP_EN_CH0: u32 = 1 << 4;

    /// **TX_CONTI_MODE_CH0**
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_CONTI_MODE_CH0_OFFSET: u32 = 3;
    pub const TX_CONTI_MODE_CH0_MASK: u32 = 0x00000008;
    pub const TX_CONTI_MODE_CH0: u32 = 1 << 3;

    /// **APB_MEM_RST_CH0**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH0_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH0_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH0: u32 = 1 << 2;

    /// **MEM_RD_RST_CH0**
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_RD_RST_CH0_OFFSET: u32 = 1;
    pub const MEM_RD_RST_CH0_MASK: u32 = 0x00000002;
    pub const MEM_RD_RST_CH0: u32 = 1 << 1;

    /// **TX_START_CH0**
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_START_CH0_OFFSET: u32 = 0;
    pub const TX_START_CH0_MASK: u32 = 0x00000001;
    pub const TX_START_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH0** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH0_MASK);
    }

    /// Trigger **AFIFO_RST_CH0** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH0_MASK);
    }

    /// **CARRIER_OUT_LV_CH0** `[22]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// **CARRIER_EN_CH0** `[21]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH0_MASK);
    }

    /// Clear CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH0_MASK);
    }

    /// **CARRIER_EFF_EN_CH0** `[20]` (RW)
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// Clear CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH0_MASK) >> Self::MEM_SIZE_CH0_OFFSET
    }

    /// Write **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH0_MASK)
                | ((value << Self::MEM_SIZE_CH0_OFFSET) & Self::MEM_SIZE_CH0_MASK)
        });
    }

    /// **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH0_MASK) >> Self::DIV_CNT_CH0_OFFSET
    }

    /// Write **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH0_MASK)
                | ((value << Self::DIV_CNT_CH0_OFFSET) & Self::DIV_CNT_CH0_MASK)
        });
    }

    /// **TX_STOP_CH0** `[7]` (RW)
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_STOP_CH0_MASK) != 0
    }

    /// Set TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_STOP_CH0_MASK);
    }

    /// Clear TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::TX_STOP_CH0_MASK);
    }

    /// **IDLE_OUT_EN_CH0** `[6]` (RW)
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// Clear IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// **IDLE_OUT_LV_CH0** `[5]` (RW)
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// Clear IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Set MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// Clear MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// **TX_CONTI_MODE_CH0** `[3]` (RW)
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_conti_mode_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Clear TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_conti_mode_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Trigger **APB_MEM_RST_CH0** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH0_MASK);
    }

    /// Trigger **MEM_RD_RST_CH0** (WO)
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub fn trigger_mem_rd_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::MEM_RD_RST_CH0_MASK);
    }

    /// Trigger **TX_START_CH0** (WO)
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub fn trigger_tx_start_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_CH0_MASK);
    }
}

/// TX_CH1CONF0 value type for batch read/modify/write operations
///
/// Use `txch1conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch1conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh1conf0Val {
    bits: u32,
}

impl TxCh1conf0Val {
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
        Self { bits: 0x00710200 }
    }

    /// Read **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Read **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::CARRIER_EN_CH0_MASK) != 0
    }

    /// Read **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn carrier_eff_en_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Read **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch0(&self) -> u32 {
        (self.bits & TxCh1conf0::MEM_SIZE_CH0_MASK) >> TxCh1conf0::MEM_SIZE_CH0_OFFSET
    }

    /// Read **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch0(&self) -> u32 {
        (self.bits & TxCh1conf0::DIV_CNT_CH0_MASK) >> TxCh1conf0::DIV_CNT_CH0_OFFSET
    }

    /// Read **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn tx_stop_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::TX_STOP_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn idle_out_en_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn idle_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Read **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Read **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn tx_conti_mode_ch0(&self) -> bool {
        (self.bits & TxCh1conf0::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH0** `[24:24]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::CONF_UPDATE_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::CONF_UPDATE_CH0_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH0** `[23:23]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::AFIFO_RST_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::AFIFO_RST_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::CARRIER_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::CARRIER_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::CARRIER_EN_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::CARRIER_EN_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn with_carrier_eff_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::CARRIER_EFF_EN_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::CARRIER_EFF_EN_CH0_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh1conf0::MEM_SIZE_CH0_MASK)
                | ((value << TxCh1conf0::MEM_SIZE_CH0_OFFSET) & TxCh1conf0::MEM_SIZE_CH0_MASK),
        }
    }

    /// Set **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh1conf0::DIV_CNT_CH0_MASK)
                | ((value << TxCh1conf0::DIV_CNT_CH0_OFFSET) & TxCh1conf0::DIV_CNT_CH0_MASK),
        }
    }

    /// Set **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn with_tx_stop_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::TX_STOP_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::TX_STOP_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::IDLE_OUT_EN_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::IDLE_OUT_EN_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::IDLE_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::IDLE_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_tx_wrap_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::MEM_TX_WRAP_EN_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::MEM_TX_WRAP_EN_CH0_MASK
            },
        }
    }

    /// Set **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_conti_mode_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::TX_CONTI_MODE_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::TX_CONTI_MODE_CH0_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH0** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::APB_MEM_RST_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::APB_MEM_RST_CH0_MASK
            },
        }
    }

    /// Set **MEM_RD_RST_CH0** `[1:1]`
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub const fn with_mem_rd_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::MEM_RD_RST_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::MEM_RD_RST_CH0_MASK
            },
        }
    }

    /// Set **TX_START_CH0** `[0:0]`
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_start_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh1conf0::TX_START_CH0_MASK
            } else {
                self.bits & !TxCh1conf0::TX_START_CH0_MASK
            },
        }
    }
}

impl Default for TxCh1conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxCh1conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh1conf0Val {
        TxCh1conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCh1conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCh1conf0Val) -> TxCh1conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH2CONF0 Register
///
/// Channel 2 configure register 0
///
/// Address offset: 0x0028
/// Reset value: 0x00710200
///
/// ## Fields
///
/// - **CONF_UPDATE_CH0** `[24]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH0** `[23]` (WO)
///
///   Reserved
///
/// - **CARRIER_OUT_LV_CH0** `[22]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH0** `[21]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **CARRIER_EFF_EN_CH0** `[20]` (RW)
///
///   1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
///
/// - **MEM_SIZE_CH0** `[19:16]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **DIV_CNT_CH0** `[15:8]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
///
/// - **TX_STOP_CH0** `[7]` (RW)
///
///   Set this bit to stop the transmitter of CHANNEL%s sending data out.
///
/// - **IDLE_OUT_EN_CH0** `[6]` (RW)
///
///   This is the output enable-control bit for CHANNEL%s in IDLE state.
///
/// - **IDLE_OUT_LV_CH0** `[5]` (RW)
///
///   This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
///
/// - **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
///
/// - **TX_CONTI_MODE_CH0** `[3]` (RW)
///
///   Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
///
/// - **APB_MEM_RST_CH0** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_RD_RST_CH0** `[1]` (WO)
///
///   Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
///
/// - **TX_START_CH0** `[0]` (WO)
///
///   Set this bit to start sending data on CHANNEL%s.
#[repr(transparent)]
pub struct TxCh2conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh2conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh2conf0 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH0**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CONF_UPDATE_CH0_OFFSET: u32 = 24;
    pub const CONF_UPDATE_CH0_MASK: u32 = 0x01000000;
    pub const CONF_UPDATE_CH0: u32 = 1 << 24;

    /// **AFIFO_RST_CH0**
    ///
    /// Reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const AFIFO_RST_CH0_OFFSET: u32 = 23;
    pub const AFIFO_RST_CH0_MASK: u32 = 0x00800000;
    pub const AFIFO_RST_CH0: u32 = 1 << 23;

    /// **CARRIER_OUT_LV_CH0**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CARRIER_OUT_LV_CH0_OFFSET: u32 = 22;
    pub const CARRIER_OUT_LV_CH0_MASK: u32 = 0x00400000;
    pub const CARRIER_OUT_LV_CH0: u32 = 1 << 22;

    /// **CARRIER_EN_CH0**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CARRIER_EN_CH0_OFFSET: u32 = 21;
    pub const CARRIER_EN_CH0_MASK: u32 = 0x00200000;
    pub const CARRIER_EN_CH0: u32 = 1 << 21;

    /// **CARRIER_EFF_EN_CH0**
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CARRIER_EFF_EN_CH0_OFFSET: u32 = 20;
    pub const CARRIER_EFF_EN_CH0_MASK: u32 = 0x00100000;
    pub const CARRIER_EFF_EN_CH0: u32 = 1 << 20;

    /// **MEM_SIZE_CH0**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const MEM_SIZE_CH0_OFFSET: u32 = 16;
    pub const MEM_SIZE_CH0_MASK: u32 = 0x000f0000;

    /// **DIV_CNT_CH0**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DIV_CNT_CH0_OFFSET: u32 = 8;
    pub const DIV_CNT_CH0_MASK: u32 = 0x0000ff00;

    /// **TX_STOP_CH0**
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_STOP_CH0_OFFSET: u32 = 7;
    pub const TX_STOP_CH0_MASK: u32 = 0x00000080;
    pub const TX_STOP_CH0: u32 = 1 << 7;

    /// **IDLE_OUT_EN_CH0**
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDLE_OUT_EN_CH0_OFFSET: u32 = 6;
    pub const IDLE_OUT_EN_CH0_MASK: u32 = 0x00000040;
    pub const IDLE_OUT_EN_CH0: u32 = 1 << 6;

    /// **IDLE_OUT_LV_CH0**
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDLE_OUT_LV_CH0_OFFSET: u32 = 5;
    pub const IDLE_OUT_LV_CH0_MASK: u32 = 0x00000020;
    pub const IDLE_OUT_LV_CH0: u32 = 1 << 5;

    /// **MEM_TX_WRAP_EN_CH0**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TX_WRAP_EN_CH0_OFFSET: u32 = 4;
    pub const MEM_TX_WRAP_EN_CH0_MASK: u32 = 0x00000010;
    pub const MEM_TX_WRAP_EN_CH0: u32 = 1 << 4;

    /// **TX_CONTI_MODE_CH0**
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_CONTI_MODE_CH0_OFFSET: u32 = 3;
    pub const TX_CONTI_MODE_CH0_MASK: u32 = 0x00000008;
    pub const TX_CONTI_MODE_CH0: u32 = 1 << 3;

    /// **APB_MEM_RST_CH0**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH0_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH0_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH0: u32 = 1 << 2;

    /// **MEM_RD_RST_CH0**
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_RD_RST_CH0_OFFSET: u32 = 1;
    pub const MEM_RD_RST_CH0_MASK: u32 = 0x00000002;
    pub const MEM_RD_RST_CH0: u32 = 1 << 1;

    /// **TX_START_CH0**
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_START_CH0_OFFSET: u32 = 0;
    pub const TX_START_CH0_MASK: u32 = 0x00000001;
    pub const TX_START_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH0** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH0_MASK);
    }

    /// Trigger **AFIFO_RST_CH0** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH0_MASK);
    }

    /// **CARRIER_OUT_LV_CH0** `[22]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// **CARRIER_EN_CH0** `[21]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH0_MASK);
    }

    /// Clear CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH0_MASK);
    }

    /// **CARRIER_EFF_EN_CH0** `[20]` (RW)
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// Clear CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH0_MASK) >> Self::MEM_SIZE_CH0_OFFSET
    }

    /// Write **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH0_MASK)
                | ((value << Self::MEM_SIZE_CH0_OFFSET) & Self::MEM_SIZE_CH0_MASK)
        });
    }

    /// **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH0_MASK) >> Self::DIV_CNT_CH0_OFFSET
    }

    /// Write **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH0_MASK)
                | ((value << Self::DIV_CNT_CH0_OFFSET) & Self::DIV_CNT_CH0_MASK)
        });
    }

    /// **TX_STOP_CH0** `[7]` (RW)
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_STOP_CH0_MASK) != 0
    }

    /// Set TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_STOP_CH0_MASK);
    }

    /// Clear TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::TX_STOP_CH0_MASK);
    }

    /// **IDLE_OUT_EN_CH0** `[6]` (RW)
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// Clear IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// **IDLE_OUT_LV_CH0** `[5]` (RW)
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// Clear IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Set MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// Clear MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// **TX_CONTI_MODE_CH0** `[3]` (RW)
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_conti_mode_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Clear TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_conti_mode_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Trigger **APB_MEM_RST_CH0** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH0_MASK);
    }

    /// Trigger **MEM_RD_RST_CH0** (WO)
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub fn trigger_mem_rd_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::MEM_RD_RST_CH0_MASK);
    }

    /// Trigger **TX_START_CH0** (WO)
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub fn trigger_tx_start_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_CH0_MASK);
    }
}

/// TX_CH2CONF0 value type for batch read/modify/write operations
///
/// Use `txch2conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch2conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh2conf0Val {
    bits: u32,
}

impl TxCh2conf0Val {
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
        Self { bits: 0x00710200 }
    }

    /// Read **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Read **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::CARRIER_EN_CH0_MASK) != 0
    }

    /// Read **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn carrier_eff_en_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Read **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch0(&self) -> u32 {
        (self.bits & TxCh2conf0::MEM_SIZE_CH0_MASK) >> TxCh2conf0::MEM_SIZE_CH0_OFFSET
    }

    /// Read **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch0(&self) -> u32 {
        (self.bits & TxCh2conf0::DIV_CNT_CH0_MASK) >> TxCh2conf0::DIV_CNT_CH0_OFFSET
    }

    /// Read **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn tx_stop_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::TX_STOP_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn idle_out_en_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn idle_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Read **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Read **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn tx_conti_mode_ch0(&self) -> bool {
        (self.bits & TxCh2conf0::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH0** `[24:24]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::CONF_UPDATE_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::CONF_UPDATE_CH0_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH0** `[23:23]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::AFIFO_RST_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::AFIFO_RST_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::CARRIER_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::CARRIER_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::CARRIER_EN_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::CARRIER_EN_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn with_carrier_eff_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::CARRIER_EFF_EN_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::CARRIER_EFF_EN_CH0_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh2conf0::MEM_SIZE_CH0_MASK)
                | ((value << TxCh2conf0::MEM_SIZE_CH0_OFFSET) & TxCh2conf0::MEM_SIZE_CH0_MASK),
        }
    }

    /// Set **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh2conf0::DIV_CNT_CH0_MASK)
                | ((value << TxCh2conf0::DIV_CNT_CH0_OFFSET) & TxCh2conf0::DIV_CNT_CH0_MASK),
        }
    }

    /// Set **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn with_tx_stop_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::TX_STOP_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::TX_STOP_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::IDLE_OUT_EN_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::IDLE_OUT_EN_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::IDLE_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::IDLE_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_tx_wrap_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::MEM_TX_WRAP_EN_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::MEM_TX_WRAP_EN_CH0_MASK
            },
        }
    }

    /// Set **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_conti_mode_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::TX_CONTI_MODE_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::TX_CONTI_MODE_CH0_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH0** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::APB_MEM_RST_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::APB_MEM_RST_CH0_MASK
            },
        }
    }

    /// Set **MEM_RD_RST_CH0** `[1:1]`
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub const fn with_mem_rd_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::MEM_RD_RST_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::MEM_RD_RST_CH0_MASK
            },
        }
    }

    /// Set **TX_START_CH0** `[0:0]`
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_start_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh2conf0::TX_START_CH0_MASK
            } else {
                self.bits & !TxCh2conf0::TX_START_CH0_MASK
            },
        }
    }
}

impl Default for TxCh2conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxCh2conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh2conf0Val {
        TxCh2conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCh2conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCh2conf0Val) -> TxCh2conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH3CONF0 Register
///
/// Channel 3 configure register 0
///
/// Address offset: 0x002c
/// Reset value: 0x00710200
///
/// ## Fields
///
/// - **CONF_UPDATE_CH0** `[24]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH0** `[23]` (WO)
///
///   Reserved
///
/// - **CARRIER_OUT_LV_CH0** `[22]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH0** `[21]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **CARRIER_EFF_EN_CH0** `[20]` (RW)
///
///   1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
///
/// - **MEM_SIZE_CH0** `[19:16]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **DIV_CNT_CH0** `[15:8]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
///
/// - **TX_STOP_CH0** `[7]` (RW)
///
///   Set this bit to stop the transmitter of CHANNEL%s sending data out.
///
/// - **IDLE_OUT_EN_CH0** `[6]` (RW)
///
///   This is the output enable-control bit for CHANNEL%s in IDLE state.
///
/// - **IDLE_OUT_LV_CH0** `[5]` (RW)
///
///   This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
///
/// - **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
///
/// - **TX_CONTI_MODE_CH0** `[3]` (RW)
///
///   Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
///
/// - **APB_MEM_RST_CH0** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_RD_RST_CH0** `[1]` (WO)
///
///   Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
///
/// - **TX_START_CH0** `[0]` (WO)
///
///   Set this bit to start sending data on CHANNEL%s.
#[repr(transparent)]
pub struct TxCh3conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh3conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh3conf0 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH0**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CONF_UPDATE_CH0_OFFSET: u32 = 24;
    pub const CONF_UPDATE_CH0_MASK: u32 = 0x01000000;
    pub const CONF_UPDATE_CH0: u32 = 1 << 24;

    /// **AFIFO_RST_CH0**
    ///
    /// Reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const AFIFO_RST_CH0_OFFSET: u32 = 23;
    pub const AFIFO_RST_CH0_MASK: u32 = 0x00800000;
    pub const AFIFO_RST_CH0: u32 = 1 << 23;

    /// **CARRIER_OUT_LV_CH0**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CARRIER_OUT_LV_CH0_OFFSET: u32 = 22;
    pub const CARRIER_OUT_LV_CH0_MASK: u32 = 0x00400000;
    pub const CARRIER_OUT_LV_CH0: u32 = 1 << 22;

    /// **CARRIER_EN_CH0**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CARRIER_EN_CH0_OFFSET: u32 = 21;
    pub const CARRIER_EN_CH0_MASK: u32 = 0x00200000;
    pub const CARRIER_EN_CH0: u32 = 1 << 21;

    /// **CARRIER_EFF_EN_CH0**
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CARRIER_EFF_EN_CH0_OFFSET: u32 = 20;
    pub const CARRIER_EFF_EN_CH0_MASK: u32 = 0x00100000;
    pub const CARRIER_EFF_EN_CH0: u32 = 1 << 20;

    /// **MEM_SIZE_CH0**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const MEM_SIZE_CH0_OFFSET: u32 = 16;
    pub const MEM_SIZE_CH0_MASK: u32 = 0x000f0000;

    /// **DIV_CNT_CH0**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DIV_CNT_CH0_OFFSET: u32 = 8;
    pub const DIV_CNT_CH0_MASK: u32 = 0x0000ff00;

    /// **TX_STOP_CH0**
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_STOP_CH0_OFFSET: u32 = 7;
    pub const TX_STOP_CH0_MASK: u32 = 0x00000080;
    pub const TX_STOP_CH0: u32 = 1 << 7;

    /// **IDLE_OUT_EN_CH0**
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDLE_OUT_EN_CH0_OFFSET: u32 = 6;
    pub const IDLE_OUT_EN_CH0_MASK: u32 = 0x00000040;
    pub const IDLE_OUT_EN_CH0: u32 = 1 << 6;

    /// **IDLE_OUT_LV_CH0**
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDLE_OUT_LV_CH0_OFFSET: u32 = 5;
    pub const IDLE_OUT_LV_CH0_MASK: u32 = 0x00000020;
    pub const IDLE_OUT_LV_CH0: u32 = 1 << 5;

    /// **MEM_TX_WRAP_EN_CH0**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TX_WRAP_EN_CH0_OFFSET: u32 = 4;
    pub const MEM_TX_WRAP_EN_CH0_MASK: u32 = 0x00000010;
    pub const MEM_TX_WRAP_EN_CH0: u32 = 1 << 4;

    /// **TX_CONTI_MODE_CH0**
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_CONTI_MODE_CH0_OFFSET: u32 = 3;
    pub const TX_CONTI_MODE_CH0_MASK: u32 = 0x00000008;
    pub const TX_CONTI_MODE_CH0: u32 = 1 << 3;

    /// **APB_MEM_RST_CH0**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH0_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH0_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH0: u32 = 1 << 2;

    /// **MEM_RD_RST_CH0**
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_RD_RST_CH0_OFFSET: u32 = 1;
    pub const MEM_RD_RST_CH0_MASK: u32 = 0x00000002;
    pub const MEM_RD_RST_CH0: u32 = 1 << 1;

    /// **TX_START_CH0**
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_START_CH0_OFFSET: u32 = 0;
    pub const TX_START_CH0_MASK: u32 = 0x00000001;
    pub const TX_START_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH0** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH0_MASK);
    }

    /// Trigger **AFIFO_RST_CH0** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH0_MASK);
    }

    /// **CARRIER_OUT_LV_CH0** `[22]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH0_MASK);
    }

    /// **CARRIER_EN_CH0** `[21]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH0_MASK);
    }

    /// Clear CARRIER_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH0_MASK);
    }

    /// **CARRIER_EFF_EN_CH0** `[20]` (RW)
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Set CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// Clear CARRIER_EFF_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_eff_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_EFF_EN_CH0_MASK);
    }

    /// **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH0_MASK) >> Self::MEM_SIZE_CH0_OFFSET
    }

    /// Write **MEM_SIZE_CH0** `[19:16]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH0_MASK)
                | ((value << Self::MEM_SIZE_CH0_OFFSET) & Self::MEM_SIZE_CH0_MASK)
        });
    }

    /// **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH0_MASK) >> Self::DIV_CNT_CH0_OFFSET
    }

    /// Write **DIV_CNT_CH0** `[15:8]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH0_MASK)
                | ((value << Self::DIV_CNT_CH0_OFFSET) & Self::DIV_CNT_CH0_MASK)
        });
    }

    /// **TX_STOP_CH0** `[7]` (RW)
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_STOP_CH0_MASK) != 0
    }

    /// Set TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_STOP_CH0_MASK);
    }

    /// Clear TX_STOP_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_stop_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::TX_STOP_CH0_MASK);
    }

    /// **IDLE_OUT_EN_CH0** `[6]` (RW)
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// Clear IDLE_OUT_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_EN_CH0_MASK);
    }

    /// **IDLE_OUT_LV_CH0** `[5]` (RW)
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Set IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// Clear IDLE_OUT_LV_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_out_lv_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_OUT_LV_CH0_MASK);
    }

    /// **MEM_TX_WRAP_EN_CH0** `[4]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Set MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// Clear MEM_TX_WRAP_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_tx_wrap_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_TX_WRAP_EN_CH0_MASK);
    }

    /// **TX_CONTI_MODE_CH0** `[3]` (RW)
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> bool {
        (self.value.read_bits() & Self::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_conti_mode_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Clear TX_CONTI_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_conti_mode_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CONTI_MODE_CH0_MASK);
    }

    /// Trigger **APB_MEM_RST_CH0** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH0_MASK);
    }

    /// Trigger **MEM_RD_RST_CH0** (WO)
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub fn trigger_mem_rd_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::MEM_RD_RST_CH0_MASK);
    }

    /// Trigger **TX_START_CH0** (WO)
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub fn trigger_tx_start_ch0(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_CH0_MASK);
    }
}

/// TX_CH3CONF0 value type for batch read/modify/write operations
///
/// Use `txch3conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch3conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh3conf0Val {
    bits: u32,
}

impl TxCh3conf0Val {
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
        Self { bits: 0x00710200 }
    }

    /// Read **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::CARRIER_OUT_LV_CH0_MASK) != 0
    }

    /// Read **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::CARRIER_EN_CH0_MASK) != 0
    }

    /// Read **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn carrier_eff_en_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::CARRIER_EFF_EN_CH0_MASK) != 0
    }

    /// Read **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch0(&self) -> u32 {
        (self.bits & TxCh3conf0::MEM_SIZE_CH0_MASK) >> TxCh3conf0::MEM_SIZE_CH0_OFFSET
    }

    /// Read **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch0(&self) -> u32 {
        (self.bits & TxCh3conf0::DIV_CNT_CH0_MASK) >> TxCh3conf0::DIV_CNT_CH0_OFFSET
    }

    /// Read **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn tx_stop_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::TX_STOP_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn idle_out_en_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::IDLE_OUT_EN_CH0_MASK) != 0
    }

    /// Read **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn idle_out_lv_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::IDLE_OUT_LV_CH0_MASK) != 0
    }

    /// Read **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn mem_tx_wrap_en_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::MEM_TX_WRAP_EN_CH0_MASK) != 0
    }

    /// Read **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn tx_conti_mode_ch0(&self) -> bool {
        (self.bits & TxCh3conf0::TX_CONTI_MODE_CH0_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH0** `[24:24]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::CONF_UPDATE_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::CONF_UPDATE_CH0_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH0** `[23:23]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::AFIFO_RST_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::AFIFO_RST_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_OUT_LV_CH0** `[22:22]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::CARRIER_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::CARRIER_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH0** `[21:21]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::CARRIER_EN_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::CARRIER_EN_CH0_MASK
            },
        }
    }

    /// Set **CARRIER_EFF_EN_CH0** `[20:20]`
    ///
    /// 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub const fn with_carrier_eff_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::CARRIER_EFF_EN_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::CARRIER_EFF_EN_CH0_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH0** `[19:16]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh3conf0::MEM_SIZE_CH0_MASK)
                | ((value << TxCh3conf0::MEM_SIZE_CH0_OFFSET) & TxCh3conf0::MEM_SIZE_CH0_MASK),
        }
    }

    /// Set **DIV_CNT_CH0** `[15:8]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCh3conf0::DIV_CNT_CH0_MASK)
                | ((value << TxCh3conf0::DIV_CNT_CH0_OFFSET) & TxCh3conf0::DIV_CNT_CH0_MASK),
        }
    }

    /// Set **TX_STOP_CH0** `[7:7]`
    ///
    /// Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub const fn with_tx_stop_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::TX_STOP_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::TX_STOP_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_EN_CH0** `[6:6]`
    ///
    /// This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::IDLE_OUT_EN_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::IDLE_OUT_EN_CH0_MASK
            },
        }
    }

    /// Set **IDLE_OUT_LV_CH0** `[5:5]`
    ///
    /// This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub const fn with_idle_out_lv_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::IDLE_OUT_LV_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::IDLE_OUT_LV_CH0_MASK
            },
        }
    }

    /// Set **MEM_TX_WRAP_EN_CH0** `[4:4]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_tx_wrap_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::MEM_TX_WRAP_EN_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::MEM_TX_WRAP_EN_CH0_MASK
            },
        }
    }

    /// Set **TX_CONTI_MODE_CH0** `[3:3]`
    ///
    /// Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_conti_mode_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::TX_CONTI_MODE_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::TX_CONTI_MODE_CH0_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH0** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::APB_MEM_RST_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::APB_MEM_RST_CH0_MASK
            },
        }
    }

    /// Set **MEM_RD_RST_CH0** `[1:1]`
    ///
    /// Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    pub const fn with_mem_rd_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::MEM_RD_RST_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::MEM_RD_RST_CH0_MASK
            },
        }
    }

    /// Set **TX_START_CH0** `[0:0]`
    ///
    /// Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_tx_start_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCh3conf0::TX_START_CH0_MASK
            } else {
                self.bits & !TxCh3conf0::TX_START_CH0_MASK
            },
        }
    }
}

impl Default for TxCh3conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxCh3conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh3conf0Val {
        TxCh3conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCh3conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCh3conf0Val) -> TxCh3conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH0CONF0 Register
///
/// Channel 0 configure register 0
///
/// Address offset: 0x0030
/// Reset value: 0x317fff02
///
/// ## Fields
///
/// - **CARRIER_OUT_LV_CH4** `[29]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH4** `[28]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **MEM_SIZE_CH4** `[27:24]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **IDLE_THRES_CH4** `[22:8]` (RW)
///
///   When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
///
/// - **DIV_CNT_CH4** `[7:0]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
#[repr(transparent)]
pub struct RxCh0conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh0conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh0conf0 {
    // ======== Field Constants ========

    /// **CARRIER_OUT_LV_CH4**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CARRIER_OUT_LV_CH4_OFFSET: u32 = 29;
    pub const CARRIER_OUT_LV_CH4_MASK: u32 = 0x20000000;
    pub const CARRIER_OUT_LV_CH4: u32 = 1 << 29;

    /// **CARRIER_EN_CH4**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CARRIER_EN_CH4_OFFSET: u32 = 28;
    pub const CARRIER_EN_CH4_MASK: u32 = 0x10000000;
    pub const CARRIER_EN_CH4: u32 = 1 << 28;

    /// **MEM_SIZE_CH4**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const MEM_SIZE_CH4_OFFSET: u32 = 24;
    pub const MEM_SIZE_CH4_MASK: u32 = 0x0f000000;

    /// **IDLE_THRES_CH4**
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    ///
    /// Bits `[22:8]`, mask `0x007fff00`
    pub const IDLE_THRES_CH4_OFFSET: u32 = 8;
    pub const IDLE_THRES_CH4_MASK: u32 = 0x007fff00;

    /// **DIV_CNT_CH4**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DIV_CNT_CH4_OFFSET: u32 = 0;
    pub const DIV_CNT_CH4_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CARRIER_OUT_LV_CH4** `[29]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// **CARRIER_EN_CH4** `[28]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH4_MASK) != 0
    }

    /// Set CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH4_MASK);
    }

    /// Clear CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH4_MASK);
    }

    /// **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH4_MASK) >> Self::MEM_SIZE_CH4_OFFSET
    }

    /// Write **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH4_MASK)
                | ((value << Self::MEM_SIZE_CH4_OFFSET) & Self::MEM_SIZE_CH4_MASK)
        });
    }

    /// **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn idle_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::IDLE_THRES_CH4_MASK) >> Self::IDLE_THRES_CH4_OFFSET
    }

    /// Write **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn write_idle_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDLE_THRES_CH4_MASK)
                | ((value << Self::IDLE_THRES_CH4_OFFSET) & Self::IDLE_THRES_CH4_MASK)
        });
    }

    /// **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH4_MASK) >> Self::DIV_CNT_CH4_OFFSET
    }

    /// Write **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH4_MASK)
                | ((value << Self::DIV_CNT_CH4_OFFSET) & Self::DIV_CNT_CH4_MASK)
        });
    }
}

/// RX_CH0CONF0 value type for batch read/modify/write operations
///
/// Use `rxch0conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch0conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh0conf0Val {
    bits: u32,
}

impl RxCh0conf0Val {
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
        Self { bits: 0x317fff02 }
    }

    /// Read **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch4(&self) -> bool {
        (self.bits & RxCh0conf0::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Read **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch4(&self) -> bool {
        (self.bits & RxCh0conf0::CARRIER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch4(&self) -> u32 {
        (self.bits & RxCh0conf0::MEM_SIZE_CH4_MASK) >> RxCh0conf0::MEM_SIZE_CH4_OFFSET
    }

    /// Read **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn idle_thres_ch4(&self) -> u32 {
        (self.bits & RxCh0conf0::IDLE_THRES_CH4_MASK) >> RxCh0conf0::IDLE_THRES_CH4_OFFSET
    }

    /// Read **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch4(&self) -> u32 {
        (self.bits & RxCh0conf0::DIV_CNT_CH4_MASK) >> RxCh0conf0::DIV_CNT_CH4_OFFSET
    }

    /// Set **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf0::CARRIER_OUT_LV_CH4_MASK
            } else {
                self.bits & !RxCh0conf0::CARRIER_OUT_LV_CH4_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf0::CARRIER_EN_CH4_MASK
            } else {
                self.bits & !RxCh0conf0::CARRIER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh0conf0::MEM_SIZE_CH4_MASK)
                | ((value << RxCh0conf0::MEM_SIZE_CH4_OFFSET) & RxCh0conf0::MEM_SIZE_CH4_MASK),
        }
    }

    /// Set **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn with_idle_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh0conf0::IDLE_THRES_CH4_MASK)
                | ((value << RxCh0conf0::IDLE_THRES_CH4_OFFSET) & RxCh0conf0::IDLE_THRES_CH4_MASK),
        }
    }

    /// Set **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh0conf0::DIV_CNT_CH4_MASK)
                | ((value << RxCh0conf0::DIV_CNT_CH4_OFFSET) & RxCh0conf0::DIV_CNT_CH4_MASK),
        }
    }
}

impl Default for RxCh0conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh0conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh0conf0Val {
        RxCh0conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh0conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh0conf0Val) -> RxCh0conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH0CONF1 Register
///
/// Channel 0 configure register 1
///
/// Address offset: 0x0034
/// Reset value: 0x000001e8
///
/// ## Fields
///
/// - **CONF_UPDATE_CH4** `[15]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH4** `[14]` (WO)
///
///   Reserved
///
/// - **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
///
/// - **RX_FILTER_THRES_CH4** `[12:5]` (RW)
///
///   Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
///
/// - **RX_FILTER_EN_CH4** `[4]` (RW)
///
///   This is the receive filter's enable bit for CHANNEL%s.
///
/// - **MEM_OWNER_CH4** `[3]` (RW)
///
///   This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
///
/// - **APB_MEM_RST_CH4** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_WR_RST_CH4** `[1]` (WO)
///
///   Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
///
/// - **RX_EN_CH4** `[0]` (RW)
///
///   Set this bit to enable receiver to receive data on CHANNEL%s.
#[repr(transparent)]
pub struct RxCh0conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh0conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh0conf1 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH4**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CONF_UPDATE_CH4_OFFSET: u32 = 15;
    pub const CONF_UPDATE_CH4_MASK: u32 = 0x00008000;
    pub const CONF_UPDATE_CH4: u32 = 1 << 15;

    /// **AFIFO_RST_CH4**
    ///
    /// Reserved
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const AFIFO_RST_CH4_OFFSET: u32 = 14;
    pub const AFIFO_RST_CH4_MASK: u32 = 0x00004000;
    pub const AFIFO_RST_CH4: u32 = 1 << 14;

    /// **MEM_RX_WRAP_EN_CH4**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MEM_RX_WRAP_EN_CH4_OFFSET: u32 = 13;
    pub const MEM_RX_WRAP_EN_CH4_MASK: u32 = 0x00002000;
    pub const MEM_RX_WRAP_EN_CH4: u32 = 1 << 13;

    /// **RX_FILTER_THRES_CH4**
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    ///
    /// Bits `[12:5]`, mask `0x00001fe0`
    pub const RX_FILTER_THRES_CH4_OFFSET: u32 = 5;
    pub const RX_FILTER_THRES_CH4_MASK: u32 = 0x00001fe0;

    /// **RX_FILTER_EN_CH4**
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FILTER_EN_CH4_OFFSET: u32 = 4;
    pub const RX_FILTER_EN_CH4_MASK: u32 = 0x00000010;
    pub const RX_FILTER_EN_CH4: u32 = 1 << 4;

    /// **MEM_OWNER_CH4**
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MEM_OWNER_CH4_OFFSET: u32 = 3;
    pub const MEM_OWNER_CH4_MASK: u32 = 0x00000008;
    pub const MEM_OWNER_CH4: u32 = 1 << 3;

    /// **APB_MEM_RST_CH4**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH4_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH4_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH4: u32 = 1 << 2;

    /// **MEM_WR_RST_CH4**
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_WR_RST_CH4_OFFSET: u32 = 1;
    pub const MEM_WR_RST_CH4_MASK: u32 = 0x00000002;
    pub const MEM_WR_RST_CH4: u32 = 1 << 1;

    /// **RX_EN_CH4**
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_EN_CH4_OFFSET: u32 = 0;
    pub const RX_EN_CH4_MASK: u32 = 0x00000001;
    pub const RX_EN_CH4: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH4** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH4_MASK);
    }

    /// Trigger **AFIFO_RST_CH4** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH4_MASK);
    }

    /// **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Set MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// Clear MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn rx_filter_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FILTER_THRES_CH4_MASK)
            >> Self::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Write **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn write_rx_filter_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_FILTER_THRES_CH4_MASK)
                | ((value << Self::RX_FILTER_THRES_CH4_OFFSET) & Self::RX_FILTER_THRES_CH4_MASK)
        });
    }

    /// **RX_FILTER_EN_CH4** `[4]` (RW)
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub fn rx_filter_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Set RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_FILTER_EN_CH4_MASK);
    }

    /// Clear RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FILTER_EN_CH4_MASK);
    }

    /// **MEM_OWNER_CH4** `[3]` (RW)
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub fn mem_owner_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_CH4_MASK) != 0
    }

    /// Set MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_OWNER_CH4_MASK);
    }

    /// Clear MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_OWNER_CH4_MASK);
    }

    /// Trigger **APB_MEM_RST_CH4** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH4_MASK);
    }

    /// Trigger **MEM_WR_RST_CH4** (WO)
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub fn trigger_mem_wr_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_WR_RST_CH4_MASK);
    }

    /// **RX_EN_CH4** `[0]` (RW)
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub fn rx_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_EN_CH4_MASK) != 0
    }

    /// Set RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_EN_CH4_MASK);
    }

    /// Clear RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EN_CH4_MASK);
    }
}

/// RX_CH0CONF1 value type for batch read/modify/write operations
///
/// Use `rxch0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh0conf1Val {
    bits: u32,
}

impl RxCh0conf1Val {
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
        Self { bits: 0x000001e8 }
    }

    /// Read **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.bits & RxCh0conf1::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Read **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn rx_filter_thres_ch4(&self) -> u32 {
        (self.bits & RxCh0conf1::RX_FILTER_THRES_CH4_MASK) >> RxCh0conf1::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Read **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn rx_filter_en_ch4(&self) -> bool {
        (self.bits & RxCh0conf1::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn mem_owner_ch4(&self) -> bool {
        (self.bits & RxCh0conf1::MEM_OWNER_CH4_MASK) != 0
    }

    /// Read **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn rx_en_ch4(&self) -> bool {
        (self.bits & RxCh0conf1::RX_EN_CH4_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH4** `[15:15]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::CONF_UPDATE_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::CONF_UPDATE_CH4_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH4** `[14:14]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::AFIFO_RST_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::AFIFO_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_rx_wrap_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::MEM_RX_WRAP_EN_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::MEM_RX_WRAP_EN_CH4_MASK
            },
        }
    }

    /// Set **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn with_rx_filter_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh0conf1::RX_FILTER_THRES_CH4_MASK)
                | ((value << RxCh0conf1::RX_FILTER_THRES_CH4_OFFSET)
                    & RxCh0conf1::RX_FILTER_THRES_CH4_MASK),
        }
    }

    /// Set **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_filter_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::RX_FILTER_EN_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::RX_FILTER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn with_mem_owner_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::MEM_OWNER_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::MEM_OWNER_CH4_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH4** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::APB_MEM_RST_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::APB_MEM_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_WR_RST_CH4** `[1:1]`
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub const fn with_mem_wr_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::MEM_WR_RST_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::MEM_WR_RST_CH4_MASK
            },
        }
    }

    /// Set **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh0conf1::RX_EN_CH4_MASK
            } else {
                self.bits & !RxCh0conf1::RX_EN_CH4_MASK
            },
        }
    }
}

impl Default for RxCh0conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh0conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh0conf1Val {
        RxCh0conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh0conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh0conf1Val) -> RxCh0conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH1CONF0 Register
///
/// Channel 1 configure register 0
///
/// Address offset: 0x0038
/// Reset value: 0x317fff02
///
/// ## Fields
///
/// - **CARRIER_OUT_LV_CH4** `[29]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH4** `[28]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **MEM_SIZE_CH4** `[27:24]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **IDLE_THRES_CH4** `[22:8]` (RW)
///
///   When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
///
/// - **DIV_CNT_CH4** `[7:0]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
#[repr(transparent)]
pub struct RxCh1conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh1conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh1conf0 {
    // ======== Field Constants ========

    /// **CARRIER_OUT_LV_CH4**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CARRIER_OUT_LV_CH4_OFFSET: u32 = 29;
    pub const CARRIER_OUT_LV_CH4_MASK: u32 = 0x20000000;
    pub const CARRIER_OUT_LV_CH4: u32 = 1 << 29;

    /// **CARRIER_EN_CH4**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CARRIER_EN_CH4_OFFSET: u32 = 28;
    pub const CARRIER_EN_CH4_MASK: u32 = 0x10000000;
    pub const CARRIER_EN_CH4: u32 = 1 << 28;

    /// **MEM_SIZE_CH4**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const MEM_SIZE_CH4_OFFSET: u32 = 24;
    pub const MEM_SIZE_CH4_MASK: u32 = 0x0f000000;

    /// **IDLE_THRES_CH4**
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    ///
    /// Bits `[22:8]`, mask `0x007fff00`
    pub const IDLE_THRES_CH4_OFFSET: u32 = 8;
    pub const IDLE_THRES_CH4_MASK: u32 = 0x007fff00;

    /// **DIV_CNT_CH4**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DIV_CNT_CH4_OFFSET: u32 = 0;
    pub const DIV_CNT_CH4_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CARRIER_OUT_LV_CH4** `[29]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// **CARRIER_EN_CH4** `[28]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH4_MASK) != 0
    }

    /// Set CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH4_MASK);
    }

    /// Clear CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH4_MASK);
    }

    /// **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH4_MASK) >> Self::MEM_SIZE_CH4_OFFSET
    }

    /// Write **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH4_MASK)
                | ((value << Self::MEM_SIZE_CH4_OFFSET) & Self::MEM_SIZE_CH4_MASK)
        });
    }

    /// **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn idle_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::IDLE_THRES_CH4_MASK) >> Self::IDLE_THRES_CH4_OFFSET
    }

    /// Write **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn write_idle_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDLE_THRES_CH4_MASK)
                | ((value << Self::IDLE_THRES_CH4_OFFSET) & Self::IDLE_THRES_CH4_MASK)
        });
    }

    /// **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH4_MASK) >> Self::DIV_CNT_CH4_OFFSET
    }

    /// Write **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH4_MASK)
                | ((value << Self::DIV_CNT_CH4_OFFSET) & Self::DIV_CNT_CH4_MASK)
        });
    }
}

/// RX_CH1CONF0 value type for batch read/modify/write operations
///
/// Use `rxch1conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch1conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh1conf0Val {
    bits: u32,
}

impl RxCh1conf0Val {
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
        Self { bits: 0x317fff02 }
    }

    /// Read **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch4(&self) -> bool {
        (self.bits & RxCh1conf0::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Read **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch4(&self) -> bool {
        (self.bits & RxCh1conf0::CARRIER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch4(&self) -> u32 {
        (self.bits & RxCh1conf0::MEM_SIZE_CH4_MASK) >> RxCh1conf0::MEM_SIZE_CH4_OFFSET
    }

    /// Read **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn idle_thres_ch4(&self) -> u32 {
        (self.bits & RxCh1conf0::IDLE_THRES_CH4_MASK) >> RxCh1conf0::IDLE_THRES_CH4_OFFSET
    }

    /// Read **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch4(&self) -> u32 {
        (self.bits & RxCh1conf0::DIV_CNT_CH4_MASK) >> RxCh1conf0::DIV_CNT_CH4_OFFSET
    }

    /// Set **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf0::CARRIER_OUT_LV_CH4_MASK
            } else {
                self.bits & !RxCh1conf0::CARRIER_OUT_LV_CH4_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf0::CARRIER_EN_CH4_MASK
            } else {
                self.bits & !RxCh1conf0::CARRIER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh1conf0::MEM_SIZE_CH4_MASK)
                | ((value << RxCh1conf0::MEM_SIZE_CH4_OFFSET) & RxCh1conf0::MEM_SIZE_CH4_MASK),
        }
    }

    /// Set **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn with_idle_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh1conf0::IDLE_THRES_CH4_MASK)
                | ((value << RxCh1conf0::IDLE_THRES_CH4_OFFSET) & RxCh1conf0::IDLE_THRES_CH4_MASK),
        }
    }

    /// Set **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh1conf0::DIV_CNT_CH4_MASK)
                | ((value << RxCh1conf0::DIV_CNT_CH4_OFFSET) & RxCh1conf0::DIV_CNT_CH4_MASK),
        }
    }
}

impl Default for RxCh1conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh1conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh1conf0Val {
        RxCh1conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh1conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh1conf0Val) -> RxCh1conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH1CONF1 Register
///
/// Channel 1 configure register 1
///
/// Address offset: 0x003c
/// Reset value: 0x000001e8
///
/// ## Fields
///
/// - **CONF_UPDATE_CH4** `[15]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH4** `[14]` (WO)
///
///   Reserved
///
/// - **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
///
/// - **RX_FILTER_THRES_CH4** `[12:5]` (RW)
///
///   Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
///
/// - **RX_FILTER_EN_CH4** `[4]` (RW)
///
///   This is the receive filter's enable bit for CHANNEL%s.
///
/// - **MEM_OWNER_CH4** `[3]` (RW)
///
///   This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
///
/// - **APB_MEM_RST_CH4** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_WR_RST_CH4** `[1]` (WO)
///
///   Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
///
/// - **RX_EN_CH4** `[0]` (RW)
///
///   Set this bit to enable receiver to receive data on CHANNEL%s.
#[repr(transparent)]
pub struct RxCh1conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh1conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh1conf1 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH4**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CONF_UPDATE_CH4_OFFSET: u32 = 15;
    pub const CONF_UPDATE_CH4_MASK: u32 = 0x00008000;
    pub const CONF_UPDATE_CH4: u32 = 1 << 15;

    /// **AFIFO_RST_CH4**
    ///
    /// Reserved
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const AFIFO_RST_CH4_OFFSET: u32 = 14;
    pub const AFIFO_RST_CH4_MASK: u32 = 0x00004000;
    pub const AFIFO_RST_CH4: u32 = 1 << 14;

    /// **MEM_RX_WRAP_EN_CH4**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MEM_RX_WRAP_EN_CH4_OFFSET: u32 = 13;
    pub const MEM_RX_WRAP_EN_CH4_MASK: u32 = 0x00002000;
    pub const MEM_RX_WRAP_EN_CH4: u32 = 1 << 13;

    /// **RX_FILTER_THRES_CH4**
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    ///
    /// Bits `[12:5]`, mask `0x00001fe0`
    pub const RX_FILTER_THRES_CH4_OFFSET: u32 = 5;
    pub const RX_FILTER_THRES_CH4_MASK: u32 = 0x00001fe0;

    /// **RX_FILTER_EN_CH4**
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FILTER_EN_CH4_OFFSET: u32 = 4;
    pub const RX_FILTER_EN_CH4_MASK: u32 = 0x00000010;
    pub const RX_FILTER_EN_CH4: u32 = 1 << 4;

    /// **MEM_OWNER_CH4**
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MEM_OWNER_CH4_OFFSET: u32 = 3;
    pub const MEM_OWNER_CH4_MASK: u32 = 0x00000008;
    pub const MEM_OWNER_CH4: u32 = 1 << 3;

    /// **APB_MEM_RST_CH4**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH4_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH4_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH4: u32 = 1 << 2;

    /// **MEM_WR_RST_CH4**
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_WR_RST_CH4_OFFSET: u32 = 1;
    pub const MEM_WR_RST_CH4_MASK: u32 = 0x00000002;
    pub const MEM_WR_RST_CH4: u32 = 1 << 1;

    /// **RX_EN_CH4**
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_EN_CH4_OFFSET: u32 = 0;
    pub const RX_EN_CH4_MASK: u32 = 0x00000001;
    pub const RX_EN_CH4: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH4** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH4_MASK);
    }

    /// Trigger **AFIFO_RST_CH4** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH4_MASK);
    }

    /// **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Set MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// Clear MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn rx_filter_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FILTER_THRES_CH4_MASK)
            >> Self::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Write **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn write_rx_filter_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_FILTER_THRES_CH4_MASK)
                | ((value << Self::RX_FILTER_THRES_CH4_OFFSET) & Self::RX_FILTER_THRES_CH4_MASK)
        });
    }

    /// **RX_FILTER_EN_CH4** `[4]` (RW)
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub fn rx_filter_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Set RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_FILTER_EN_CH4_MASK);
    }

    /// Clear RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FILTER_EN_CH4_MASK);
    }

    /// **MEM_OWNER_CH4** `[3]` (RW)
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub fn mem_owner_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_CH4_MASK) != 0
    }

    /// Set MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_OWNER_CH4_MASK);
    }

    /// Clear MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_OWNER_CH4_MASK);
    }

    /// Trigger **APB_MEM_RST_CH4** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH4_MASK);
    }

    /// Trigger **MEM_WR_RST_CH4** (WO)
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub fn trigger_mem_wr_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_WR_RST_CH4_MASK);
    }

    /// **RX_EN_CH4** `[0]` (RW)
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub fn rx_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_EN_CH4_MASK) != 0
    }

    /// Set RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_EN_CH4_MASK);
    }

    /// Clear RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EN_CH4_MASK);
    }
}

/// RX_CH1CONF1 value type for batch read/modify/write operations
///
/// Use `rxch1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh1conf1Val {
    bits: u32,
}

impl RxCh1conf1Val {
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
        Self { bits: 0x000001e8 }
    }

    /// Read **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.bits & RxCh1conf1::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Read **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn rx_filter_thres_ch4(&self) -> u32 {
        (self.bits & RxCh1conf1::RX_FILTER_THRES_CH4_MASK) >> RxCh1conf1::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Read **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn rx_filter_en_ch4(&self) -> bool {
        (self.bits & RxCh1conf1::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn mem_owner_ch4(&self) -> bool {
        (self.bits & RxCh1conf1::MEM_OWNER_CH4_MASK) != 0
    }

    /// Read **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn rx_en_ch4(&self) -> bool {
        (self.bits & RxCh1conf1::RX_EN_CH4_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH4** `[15:15]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::CONF_UPDATE_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::CONF_UPDATE_CH4_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH4** `[14:14]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::AFIFO_RST_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::AFIFO_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_rx_wrap_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::MEM_RX_WRAP_EN_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::MEM_RX_WRAP_EN_CH4_MASK
            },
        }
    }

    /// Set **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn with_rx_filter_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh1conf1::RX_FILTER_THRES_CH4_MASK)
                | ((value << RxCh1conf1::RX_FILTER_THRES_CH4_OFFSET)
                    & RxCh1conf1::RX_FILTER_THRES_CH4_MASK),
        }
    }

    /// Set **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_filter_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::RX_FILTER_EN_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::RX_FILTER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn with_mem_owner_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::MEM_OWNER_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::MEM_OWNER_CH4_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH4** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::APB_MEM_RST_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::APB_MEM_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_WR_RST_CH4** `[1:1]`
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub const fn with_mem_wr_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::MEM_WR_RST_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::MEM_WR_RST_CH4_MASK
            },
        }
    }

    /// Set **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh1conf1::RX_EN_CH4_MASK
            } else {
                self.bits & !RxCh1conf1::RX_EN_CH4_MASK
            },
        }
    }
}

impl Default for RxCh1conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh1conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh1conf1Val {
        RxCh1conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh1conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh1conf1Val) -> RxCh1conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH2CONF0 Register
///
/// Channel 2 configure register 0
///
/// Address offset: 0x0040
/// Reset value: 0x317fff02
///
/// ## Fields
///
/// - **CARRIER_OUT_LV_CH4** `[29]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH4** `[28]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **MEM_SIZE_CH4** `[27:24]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **IDLE_THRES_CH4** `[22:8]` (RW)
///
///   When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
///
/// - **DIV_CNT_CH4** `[7:0]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
#[repr(transparent)]
pub struct RxCh2conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh2conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh2conf0 {
    // ======== Field Constants ========

    /// **CARRIER_OUT_LV_CH4**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CARRIER_OUT_LV_CH4_OFFSET: u32 = 29;
    pub const CARRIER_OUT_LV_CH4_MASK: u32 = 0x20000000;
    pub const CARRIER_OUT_LV_CH4: u32 = 1 << 29;

    /// **CARRIER_EN_CH4**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CARRIER_EN_CH4_OFFSET: u32 = 28;
    pub const CARRIER_EN_CH4_MASK: u32 = 0x10000000;
    pub const CARRIER_EN_CH4: u32 = 1 << 28;

    /// **MEM_SIZE_CH4**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const MEM_SIZE_CH4_OFFSET: u32 = 24;
    pub const MEM_SIZE_CH4_MASK: u32 = 0x0f000000;

    /// **IDLE_THRES_CH4**
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    ///
    /// Bits `[22:8]`, mask `0x007fff00`
    pub const IDLE_THRES_CH4_OFFSET: u32 = 8;
    pub const IDLE_THRES_CH4_MASK: u32 = 0x007fff00;

    /// **DIV_CNT_CH4**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DIV_CNT_CH4_OFFSET: u32 = 0;
    pub const DIV_CNT_CH4_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CARRIER_OUT_LV_CH4** `[29]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// **CARRIER_EN_CH4** `[28]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH4_MASK) != 0
    }

    /// Set CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH4_MASK);
    }

    /// Clear CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH4_MASK);
    }

    /// **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH4_MASK) >> Self::MEM_SIZE_CH4_OFFSET
    }

    /// Write **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH4_MASK)
                | ((value << Self::MEM_SIZE_CH4_OFFSET) & Self::MEM_SIZE_CH4_MASK)
        });
    }

    /// **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn idle_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::IDLE_THRES_CH4_MASK) >> Self::IDLE_THRES_CH4_OFFSET
    }

    /// Write **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn write_idle_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDLE_THRES_CH4_MASK)
                | ((value << Self::IDLE_THRES_CH4_OFFSET) & Self::IDLE_THRES_CH4_MASK)
        });
    }

    /// **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH4_MASK) >> Self::DIV_CNT_CH4_OFFSET
    }

    /// Write **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH4_MASK)
                | ((value << Self::DIV_CNT_CH4_OFFSET) & Self::DIV_CNT_CH4_MASK)
        });
    }
}

/// RX_CH2CONF0 value type for batch read/modify/write operations
///
/// Use `rxch2conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch2conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh2conf0Val {
    bits: u32,
}

impl RxCh2conf0Val {
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
        Self { bits: 0x317fff02 }
    }

    /// Read **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch4(&self) -> bool {
        (self.bits & RxCh2conf0::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Read **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch4(&self) -> bool {
        (self.bits & RxCh2conf0::CARRIER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch4(&self) -> u32 {
        (self.bits & RxCh2conf0::MEM_SIZE_CH4_MASK) >> RxCh2conf0::MEM_SIZE_CH4_OFFSET
    }

    /// Read **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn idle_thres_ch4(&self) -> u32 {
        (self.bits & RxCh2conf0::IDLE_THRES_CH4_MASK) >> RxCh2conf0::IDLE_THRES_CH4_OFFSET
    }

    /// Read **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch4(&self) -> u32 {
        (self.bits & RxCh2conf0::DIV_CNT_CH4_MASK) >> RxCh2conf0::DIV_CNT_CH4_OFFSET
    }

    /// Set **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf0::CARRIER_OUT_LV_CH4_MASK
            } else {
                self.bits & !RxCh2conf0::CARRIER_OUT_LV_CH4_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf0::CARRIER_EN_CH4_MASK
            } else {
                self.bits & !RxCh2conf0::CARRIER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh2conf0::MEM_SIZE_CH4_MASK)
                | ((value << RxCh2conf0::MEM_SIZE_CH4_OFFSET) & RxCh2conf0::MEM_SIZE_CH4_MASK),
        }
    }

    /// Set **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn with_idle_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh2conf0::IDLE_THRES_CH4_MASK)
                | ((value << RxCh2conf0::IDLE_THRES_CH4_OFFSET) & RxCh2conf0::IDLE_THRES_CH4_MASK),
        }
    }

    /// Set **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh2conf0::DIV_CNT_CH4_MASK)
                | ((value << RxCh2conf0::DIV_CNT_CH4_OFFSET) & RxCh2conf0::DIV_CNT_CH4_MASK),
        }
    }
}

impl Default for RxCh2conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh2conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh2conf0Val {
        RxCh2conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh2conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh2conf0Val) -> RxCh2conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH2CONF1 Register
///
/// Channel 2 configure register 1
///
/// Address offset: 0x0044
/// Reset value: 0x000001e8
///
/// ## Fields
///
/// - **CONF_UPDATE_CH4** `[15]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH4** `[14]` (WO)
///
///   Reserved
///
/// - **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
///
/// - **RX_FILTER_THRES_CH4** `[12:5]` (RW)
///
///   Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
///
/// - **RX_FILTER_EN_CH4** `[4]` (RW)
///
///   This is the receive filter's enable bit for CHANNEL%s.
///
/// - **MEM_OWNER_CH4** `[3]` (RW)
///
///   This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
///
/// - **APB_MEM_RST_CH4** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_WR_RST_CH4** `[1]` (WO)
///
///   Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
///
/// - **RX_EN_CH4** `[0]` (RW)
///
///   Set this bit to enable receiver to receive data on CHANNEL%s.
#[repr(transparent)]
pub struct RxCh2conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh2conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh2conf1 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH4**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CONF_UPDATE_CH4_OFFSET: u32 = 15;
    pub const CONF_UPDATE_CH4_MASK: u32 = 0x00008000;
    pub const CONF_UPDATE_CH4: u32 = 1 << 15;

    /// **AFIFO_RST_CH4**
    ///
    /// Reserved
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const AFIFO_RST_CH4_OFFSET: u32 = 14;
    pub const AFIFO_RST_CH4_MASK: u32 = 0x00004000;
    pub const AFIFO_RST_CH4: u32 = 1 << 14;

    /// **MEM_RX_WRAP_EN_CH4**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MEM_RX_WRAP_EN_CH4_OFFSET: u32 = 13;
    pub const MEM_RX_WRAP_EN_CH4_MASK: u32 = 0x00002000;
    pub const MEM_RX_WRAP_EN_CH4: u32 = 1 << 13;

    /// **RX_FILTER_THRES_CH4**
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    ///
    /// Bits `[12:5]`, mask `0x00001fe0`
    pub const RX_FILTER_THRES_CH4_OFFSET: u32 = 5;
    pub const RX_FILTER_THRES_CH4_MASK: u32 = 0x00001fe0;

    /// **RX_FILTER_EN_CH4**
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FILTER_EN_CH4_OFFSET: u32 = 4;
    pub const RX_FILTER_EN_CH4_MASK: u32 = 0x00000010;
    pub const RX_FILTER_EN_CH4: u32 = 1 << 4;

    /// **MEM_OWNER_CH4**
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MEM_OWNER_CH4_OFFSET: u32 = 3;
    pub const MEM_OWNER_CH4_MASK: u32 = 0x00000008;
    pub const MEM_OWNER_CH4: u32 = 1 << 3;

    /// **APB_MEM_RST_CH4**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH4_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH4_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH4: u32 = 1 << 2;

    /// **MEM_WR_RST_CH4**
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_WR_RST_CH4_OFFSET: u32 = 1;
    pub const MEM_WR_RST_CH4_MASK: u32 = 0x00000002;
    pub const MEM_WR_RST_CH4: u32 = 1 << 1;

    /// **RX_EN_CH4**
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_EN_CH4_OFFSET: u32 = 0;
    pub const RX_EN_CH4_MASK: u32 = 0x00000001;
    pub const RX_EN_CH4: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH4** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH4_MASK);
    }

    /// Trigger **AFIFO_RST_CH4** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH4_MASK);
    }

    /// **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Set MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// Clear MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn rx_filter_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FILTER_THRES_CH4_MASK)
            >> Self::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Write **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn write_rx_filter_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_FILTER_THRES_CH4_MASK)
                | ((value << Self::RX_FILTER_THRES_CH4_OFFSET) & Self::RX_FILTER_THRES_CH4_MASK)
        });
    }

    /// **RX_FILTER_EN_CH4** `[4]` (RW)
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub fn rx_filter_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Set RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_FILTER_EN_CH4_MASK);
    }

    /// Clear RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FILTER_EN_CH4_MASK);
    }

    /// **MEM_OWNER_CH4** `[3]` (RW)
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub fn mem_owner_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_CH4_MASK) != 0
    }

    /// Set MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_OWNER_CH4_MASK);
    }

    /// Clear MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_OWNER_CH4_MASK);
    }

    /// Trigger **APB_MEM_RST_CH4** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH4_MASK);
    }

    /// Trigger **MEM_WR_RST_CH4** (WO)
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub fn trigger_mem_wr_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_WR_RST_CH4_MASK);
    }

    /// **RX_EN_CH4** `[0]` (RW)
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub fn rx_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_EN_CH4_MASK) != 0
    }

    /// Set RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_EN_CH4_MASK);
    }

    /// Clear RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EN_CH4_MASK);
    }
}

/// RX_CH2CONF1 value type for batch read/modify/write operations
///
/// Use `rxch2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh2conf1Val {
    bits: u32,
}

impl RxCh2conf1Val {
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
        Self { bits: 0x000001e8 }
    }

    /// Read **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.bits & RxCh2conf1::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Read **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn rx_filter_thres_ch4(&self) -> u32 {
        (self.bits & RxCh2conf1::RX_FILTER_THRES_CH4_MASK) >> RxCh2conf1::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Read **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn rx_filter_en_ch4(&self) -> bool {
        (self.bits & RxCh2conf1::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn mem_owner_ch4(&self) -> bool {
        (self.bits & RxCh2conf1::MEM_OWNER_CH4_MASK) != 0
    }

    /// Read **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn rx_en_ch4(&self) -> bool {
        (self.bits & RxCh2conf1::RX_EN_CH4_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH4** `[15:15]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::CONF_UPDATE_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::CONF_UPDATE_CH4_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH4** `[14:14]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::AFIFO_RST_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::AFIFO_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_rx_wrap_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::MEM_RX_WRAP_EN_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::MEM_RX_WRAP_EN_CH4_MASK
            },
        }
    }

    /// Set **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn with_rx_filter_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh2conf1::RX_FILTER_THRES_CH4_MASK)
                | ((value << RxCh2conf1::RX_FILTER_THRES_CH4_OFFSET)
                    & RxCh2conf1::RX_FILTER_THRES_CH4_MASK),
        }
    }

    /// Set **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_filter_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::RX_FILTER_EN_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::RX_FILTER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn with_mem_owner_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::MEM_OWNER_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::MEM_OWNER_CH4_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH4** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::APB_MEM_RST_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::APB_MEM_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_WR_RST_CH4** `[1:1]`
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub const fn with_mem_wr_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::MEM_WR_RST_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::MEM_WR_RST_CH4_MASK
            },
        }
    }

    /// Set **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh2conf1::RX_EN_CH4_MASK
            } else {
                self.bits & !RxCh2conf1::RX_EN_CH4_MASK
            },
        }
    }
}

impl Default for RxCh2conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh2conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh2conf1Val {
        RxCh2conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh2conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh2conf1Val) -> RxCh2conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH3CONF0 Register
///
/// Channel 3 configure register 0
///
/// Address offset: 0x0048
/// Reset value: 0x317fff02
///
/// ## Fields
///
/// - **CARRIER_OUT_LV_CH4** `[29]` (RW)
///
///   This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
///
/// - **CARRIER_EN_CH4** `[28]` (RW)
///
///   This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
///
/// - **MEM_SIZE_CH4** `[27:24]` (RW)
///
///   This register is used to configure the maximum size of memory allocated to CHANNEL%s.
///
/// - **IDLE_THRES_CH4** `[22:8]` (RW)
///
///   When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
///
/// - **DIV_CNT_CH4** `[7:0]` (RW)
///
///   This register is used to configure the divider for clock of CHANNEL%s.
#[repr(transparent)]
pub struct RxCh3conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh3conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh3conf0 {
    // ======== Field Constants ========

    /// **CARRIER_OUT_LV_CH4**
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CARRIER_OUT_LV_CH4_OFFSET: u32 = 29;
    pub const CARRIER_OUT_LV_CH4_MASK: u32 = 0x20000000;
    pub const CARRIER_OUT_LV_CH4: u32 = 1 << 29;

    /// **CARRIER_EN_CH4**
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CARRIER_EN_CH4_OFFSET: u32 = 28;
    pub const CARRIER_EN_CH4_MASK: u32 = 0x10000000;
    pub const CARRIER_EN_CH4: u32 = 1 << 28;

    /// **MEM_SIZE_CH4**
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const MEM_SIZE_CH4_OFFSET: u32 = 24;
    pub const MEM_SIZE_CH4_MASK: u32 = 0x0f000000;

    /// **IDLE_THRES_CH4**
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    ///
    /// Bits `[22:8]`, mask `0x007fff00`
    pub const IDLE_THRES_CH4_OFFSET: u32 = 8;
    pub const IDLE_THRES_CH4_MASK: u32 = 0x007fff00;

    /// **DIV_CNT_CH4**
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DIV_CNT_CH4_OFFSET: u32 = 0;
    pub const DIV_CNT_CH4_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CARRIER_OUT_LV_CH4** `[29]` (RW)
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Set CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// Clear CARRIER_OUT_LV_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_out_lv_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::CARRIER_OUT_LV_CH4_MASK);
    }

    /// **CARRIER_EN_CH4** `[28]` (RW)
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::CARRIER_EN_CH4_MASK) != 0
    }

    /// Set CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CARRIER_EN_CH4_MASK);
    }

    /// Clear CARRIER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_carrier_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::CARRIER_EN_CH4_MASK);
    }

    /// **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_SIZE_CH4_MASK) >> Self::MEM_SIZE_CH4_OFFSET
    }

    /// Write **MEM_SIZE_CH4** `[27:24]` (RW)
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn write_mem_size_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_SIZE_CH4_MASK)
                | ((value << Self::MEM_SIZE_CH4_OFFSET) & Self::MEM_SIZE_CH4_MASK)
        });
    }

    /// **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn idle_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::IDLE_THRES_CH4_MASK) >> Self::IDLE_THRES_CH4_OFFSET
    }

    /// Write **IDLE_THRES_CH4** `[22:8]` (RW)
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn write_idle_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDLE_THRES_CH4_MASK)
                | ((value << Self::IDLE_THRES_CH4_OFFSET) & Self::IDLE_THRES_CH4_MASK)
        });
    }

    /// **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_CNT_CH4_MASK) >> Self::DIV_CNT_CH4_OFFSET
    }

    /// Write **DIV_CNT_CH4** `[7:0]` (RW)
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn write_div_cnt_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_CNT_CH4_MASK)
                | ((value << Self::DIV_CNT_CH4_OFFSET) & Self::DIV_CNT_CH4_MASK)
        });
    }
}

/// RX_CH3CONF0 value type for batch read/modify/write operations
///
/// Use `rxch3conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch3conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh3conf0Val {
    bits: u32,
}

impl RxCh3conf0Val {
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
        Self { bits: 0x317fff02 }
    }

    /// Read **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn carrier_out_lv_ch4(&self) -> bool {
        (self.bits & RxCh3conf0::CARRIER_OUT_LV_CH4_MASK) != 0
    }

    /// Read **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn carrier_en_ch4(&self) -> bool {
        (self.bits & RxCh3conf0::CARRIER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn mem_size_ch4(&self) -> u32 {
        (self.bits & RxCh3conf0::MEM_SIZE_CH4_MASK) >> RxCh3conf0::MEM_SIZE_CH4_OFFSET
    }

    /// Read **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn idle_thres_ch4(&self) -> u32 {
        (self.bits & RxCh3conf0::IDLE_THRES_CH4_MASK) >> RxCh3conf0::IDLE_THRES_CH4_OFFSET
    }

    /// Read **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn div_cnt_ch4(&self) -> u32 {
        (self.bits & RxCh3conf0::DIV_CNT_CH4_MASK) >> RxCh3conf0::DIV_CNT_CH4_OFFSET
    }

    /// Set **CARRIER_OUT_LV_CH4** `[29:29]`
    ///
    /// This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub const fn with_carrier_out_lv_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf0::CARRIER_OUT_LV_CH4_MASK
            } else {
                self.bits & !RxCh3conf0::CARRIER_OUT_LV_CH4_MASK
            },
        }
    }

    /// Set **CARRIER_EN_CH4** `[28:28]`
    ///
    /// This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub const fn with_carrier_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf0::CARRIER_EN_CH4_MASK
            } else {
                self.bits & !RxCh3conf0::CARRIER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_SIZE_CH4** `[27:24]`
    ///
    /// This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub const fn with_mem_size_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh3conf0::MEM_SIZE_CH4_MASK)
                | ((value << RxCh3conf0::MEM_SIZE_CH4_OFFSET) & RxCh3conf0::MEM_SIZE_CH4_MASK),
        }
    }

    /// Set **IDLE_THRES_CH4** `[22:8]`
    ///
    /// When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub const fn with_idle_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh3conf0::IDLE_THRES_CH4_MASK)
                | ((value << RxCh3conf0::IDLE_THRES_CH4_OFFSET) & RxCh3conf0::IDLE_THRES_CH4_MASK),
        }
    }

    /// Set **DIV_CNT_CH4** `[7:0]`
    ///
    /// This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub const fn with_div_cnt_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh3conf0::DIV_CNT_CH4_MASK)
                | ((value << RxCh3conf0::DIV_CNT_CH4_OFFSET) & RxCh3conf0::DIV_CNT_CH4_MASK),
        }
    }
}

impl Default for RxCh3conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh3conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh3conf0Val {
        RxCh3conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh3conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh3conf0Val) -> RxCh3conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH3CONF1 Register
///
/// Channel 3 configure register 1
///
/// Address offset: 0x004c
/// Reset value: 0x000001e8
///
/// ## Fields
///
/// - **CONF_UPDATE_CH4** `[15]` (WO)
///
///   synchronization bit for CHANNEL%s
///
/// - **AFIFO_RST_CH4** `[14]` (WO)
///
///   Reserved
///
/// - **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
///
///   This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
///
/// - **RX_FILTER_THRES_CH4** `[12:5]` (RW)
///
///   Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
///
/// - **RX_FILTER_EN_CH4** `[4]` (RW)
///
///   This is the receive filter's enable bit for CHANNEL%s.
///
/// - **MEM_OWNER_CH4** `[3]` (RW)
///
///   This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
///
/// - **APB_MEM_RST_CH4** `[2]` (WO)
///
///   Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
///
/// - **MEM_WR_RST_CH4** `[1]` (WO)
///
///   Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
///
/// - **RX_EN_CH4** `[0]` (RW)
///
///   Set this bit to enable receiver to receive data on CHANNEL%s.
#[repr(transparent)]
pub struct RxCh3conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh3conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh3conf1 {
    // ======== Field Constants ========

    /// **CONF_UPDATE_CH4**
    ///
    /// synchronization bit for CHANNEL%s
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CONF_UPDATE_CH4_OFFSET: u32 = 15;
    pub const CONF_UPDATE_CH4_MASK: u32 = 0x00008000;
    pub const CONF_UPDATE_CH4: u32 = 1 << 15;

    /// **AFIFO_RST_CH4**
    ///
    /// Reserved
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const AFIFO_RST_CH4_OFFSET: u32 = 14;
    pub const AFIFO_RST_CH4_MASK: u32 = 0x00004000;
    pub const AFIFO_RST_CH4: u32 = 1 << 14;

    /// **MEM_RX_WRAP_EN_CH4**
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MEM_RX_WRAP_EN_CH4_OFFSET: u32 = 13;
    pub const MEM_RX_WRAP_EN_CH4_MASK: u32 = 0x00002000;
    pub const MEM_RX_WRAP_EN_CH4: u32 = 1 << 13;

    /// **RX_FILTER_THRES_CH4**
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    ///
    /// Bits `[12:5]`, mask `0x00001fe0`
    pub const RX_FILTER_THRES_CH4_OFFSET: u32 = 5;
    pub const RX_FILTER_THRES_CH4_MASK: u32 = 0x00001fe0;

    /// **RX_FILTER_EN_CH4**
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FILTER_EN_CH4_OFFSET: u32 = 4;
    pub const RX_FILTER_EN_CH4_MASK: u32 = 0x00000010;
    pub const RX_FILTER_EN_CH4: u32 = 1 << 4;

    /// **MEM_OWNER_CH4**
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MEM_OWNER_CH4_OFFSET: u32 = 3;
    pub const MEM_OWNER_CH4_MASK: u32 = 0x00000008;
    pub const MEM_OWNER_CH4: u32 = 1 << 3;

    /// **APB_MEM_RST_CH4**
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const APB_MEM_RST_CH4_OFFSET: u32 = 2;
    pub const APB_MEM_RST_CH4_MASK: u32 = 0x00000004;
    pub const APB_MEM_RST_CH4: u32 = 1 << 2;

    /// **MEM_WR_RST_CH4**
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_WR_RST_CH4_OFFSET: u32 = 1;
    pub const MEM_WR_RST_CH4_MASK: u32 = 0x00000002;
    pub const MEM_WR_RST_CH4: u32 = 1 << 1;

    /// **RX_EN_CH4**
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_EN_CH4_OFFSET: u32 = 0;
    pub const RX_EN_CH4_MASK: u32 = 0x00000001;
    pub const RX_EN_CH4: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONF_UPDATE_CH4** (WO)
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub fn trigger_conf_update_ch4(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPDATE_CH4_MASK);
    }

    /// Trigger **AFIFO_RST_CH4** (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn trigger_afifo_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::AFIFO_RST_CH4_MASK);
    }

    /// **MEM_RX_WRAP_EN_CH4** `[13]` (RW)
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Set MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// Clear MEM_RX_WRAP_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_rx_wrap_en_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_RX_WRAP_EN_CH4_MASK);
    }

    /// **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn rx_filter_thres_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FILTER_THRES_CH4_MASK)
            >> Self::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Write **RX_FILTER_THRES_CH4** `[12:5]` (RW)
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub fn write_rx_filter_thres_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_FILTER_THRES_CH4_MASK)
                | ((value << Self::RX_FILTER_THRES_CH4_OFFSET) & Self::RX_FILTER_THRES_CH4_MASK)
        });
    }

    /// **RX_FILTER_EN_CH4** `[4]` (RW)
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub fn rx_filter_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Set RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_FILTER_EN_CH4_MASK);
    }

    /// Clear RX_FILTER_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_filter_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FILTER_EN_CH4_MASK);
    }

    /// **MEM_OWNER_CH4** `[3]` (RW)
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub fn mem_owner_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_CH4_MASK) != 0
    }

    /// Set MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_OWNER_CH4_MASK);
    }

    /// Clear MEM_OWNER_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_owner_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_OWNER_CH4_MASK);
    }

    /// Trigger **APB_MEM_RST_CH4** (WO)
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub fn trigger_apb_mem_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::APB_MEM_RST_CH4_MASK);
    }

    /// Trigger **MEM_WR_RST_CH4** (WO)
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub fn trigger_mem_wr_rst_ch4(&self) {
        self.value.modify_bits(|r| r | Self::MEM_WR_RST_CH4_MASK);
    }

    /// **RX_EN_CH4** `[0]` (RW)
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub fn rx_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::RX_EN_CH4_MASK) != 0
    }

    /// Set RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r | Self::RX_EN_CH4_MASK);
    }

    /// Clear RX_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_en_ch4(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EN_CH4_MASK);
    }
}

/// RX_CH3CONF1 value type for batch read/modify/write operations
///
/// Use `rxch3conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch3conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh3conf1Val {
    bits: u32,
}

impl RxCh3conf1Val {
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
        Self { bits: 0x000001e8 }
    }

    /// Read **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn mem_rx_wrap_en_ch4(&self) -> bool {
        (self.bits & RxCh3conf1::MEM_RX_WRAP_EN_CH4_MASK) != 0
    }

    /// Read **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn rx_filter_thres_ch4(&self) -> u32 {
        (self.bits & RxCh3conf1::RX_FILTER_THRES_CH4_MASK) >> RxCh3conf1::RX_FILTER_THRES_CH4_OFFSET
    }

    /// Read **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn rx_filter_en_ch4(&self) -> bool {
        (self.bits & RxCh3conf1::RX_FILTER_EN_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn mem_owner_ch4(&self) -> bool {
        (self.bits & RxCh3conf1::MEM_OWNER_CH4_MASK) != 0
    }

    /// Read **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn rx_en_ch4(&self) -> bool {
        (self.bits & RxCh3conf1::RX_EN_CH4_MASK) != 0
    }

    /// Set **CONF_UPDATE_CH4** `[15:15]`
    ///
    /// synchronization bit for CHANNEL%s
    #[inline(always)]
    pub const fn with_conf_update_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::CONF_UPDATE_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::CONF_UPDATE_CH4_MASK
            },
        }
    }

    /// Set **AFIFO_RST_CH4** `[14:14]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_afifo_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::AFIFO_RST_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::AFIFO_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_RX_WRAP_EN_CH4** `[13:13]`
    ///
    /// This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size.
    #[inline(always)]
    pub const fn with_mem_rx_wrap_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::MEM_RX_WRAP_EN_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::MEM_RX_WRAP_EN_CH4_MASK
            },
        }
    }

    /// Set **RX_FILTER_THRES_CH4** `[12:5]`
    ///
    /// Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode).
    #[inline(always)]
    pub const fn with_rx_filter_thres_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCh3conf1::RX_FILTER_THRES_CH4_MASK)
                | ((value << RxCh3conf1::RX_FILTER_THRES_CH4_OFFSET)
                    & RxCh3conf1::RX_FILTER_THRES_CH4_MASK),
        }
    }

    /// Set **RX_FILTER_EN_CH4** `[4:4]`
    ///
    /// This is the receive filter's enable bit for CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_filter_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::RX_FILTER_EN_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::RX_FILTER_EN_CH4_MASK
            },
        }
    }

    /// Set **MEM_OWNER_CH4** `[3:3]`
    ///
    /// This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram.
    #[inline(always)]
    pub const fn with_mem_owner_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::MEM_OWNER_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::MEM_OWNER_CH4_MASK
            },
        }
    }

    /// Set **APB_MEM_RST_CH4** `[2:2]`
    ///
    /// Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    pub const fn with_apb_mem_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::APB_MEM_RST_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::APB_MEM_RST_CH4_MASK
            },
        }
    }

    /// Set **MEM_WR_RST_CH4** `[1:1]`
    ///
    /// Set this bit to reset write ram address for CHANNEL%s by accessing receiver.
    #[inline(always)]
    pub const fn with_mem_wr_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::MEM_WR_RST_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::MEM_WR_RST_CH4_MASK
            },
        }
    }

    /// Set **RX_EN_CH4** `[0:0]`
    ///
    /// Set this bit to enable receiver to receive data on CHANNEL%s.
    #[inline(always)]
    pub const fn with_rx_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCh3conf1::RX_EN_CH4_MASK
            } else {
                self.bits & !RxCh3conf1::RX_EN_CH4_MASK
            },
        }
    }
}

impl Default for RxCh3conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh3conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh3conf1Val {
        RxCh3conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCh3conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCh3conf1Val) -> RxCh3conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH0STATUS Register
///
/// Channel 0 status register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **APB_MEM_WR_ERR_CH0** `[26]` (RO)
///
///   This status bit will be set if the offset address out of memory size when writes via APB bus.
///
/// - **MEM_EMPTY_CH0** `[25]` (RO)
///
///   This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
///
/// - **STATE_CH0** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_WADDR_CH0** `[20:11]` (RO)
///
///   This register records the memory address offset when writes RAM over APB bus.
///
/// - **MEM_RADDR_EX_CH0** `[9:0]` (RO)
///
///   This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct TxCh0status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh0status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh0status {
    // ======== Field Constants ========

    /// **APB_MEM_WR_ERR_CH0**
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const APB_MEM_WR_ERR_CH0_OFFSET: u32 = 26;
    pub const APB_MEM_WR_ERR_CH0_MASK: u32 = 0x04000000;
    pub const APB_MEM_WR_ERR_CH0: u32 = 1 << 26;

    /// **MEM_EMPTY_CH0**
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_EMPTY_CH0_OFFSET: u32 = 25;
    pub const MEM_EMPTY_CH0_MASK: u32 = 0x02000000;
    pub const MEM_EMPTY_CH0: u32 = 1 << 25;

    /// **STATE_CH0**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH0_OFFSET: u32 = 22;
    pub const STATE_CH0_MASK: u32 = 0x01c00000;

    /// **APB_MEM_WADDR_CH0**
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_WADDR_CH0_OFFSET: u32 = 11;
    pub const APB_MEM_WADDR_CH0_MASK: u32 = 0x001ff800;

    /// **MEM_RADDR_EX_CH0**
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_RADDR_EX_CH0_OFFSET: u32 = 0;
    pub const MEM_RADDR_EX_CH0_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_WR_ERR_CH0** `[26]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// **MEM_EMPTY_CH0** `[25]` (RO)
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_EMPTY_CH0_MASK) != 0
    }

    /// **STATE_CH0** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH0_MASK) >> Self::STATE_CH0_OFFSET
    }

    /// **APB_MEM_WADDR_CH0** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_WADDR_CH0_MASK) >> Self::APB_MEM_WADDR_CH0_OFFSET
    }

    /// **MEM_RADDR_EX_CH0** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_RADDR_EX_CH0_MASK) >> Self::MEM_RADDR_EX_CH0_OFFSET
    }
}

/// TX_CH0STATUS value type for batch read/modify/write operations
///
/// Use `txch0status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch0status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh0statusVal {
    bits: u32,
}

impl TxCh0statusVal {
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

    /// Read **APB_MEM_WR_ERR_CH0** `[26:26]`
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub const fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.bits & TxCh0status::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// Read **MEM_EMPTY_CH0** `[25:25]`
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub const fn mem_empty_ch0(&self) -> bool {
        (self.bits & TxCh0status::MEM_EMPTY_CH0_MASK) != 0
    }

    /// Read **STATE_CH0** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch0(&self) -> u32 {
        (self.bits & TxCh0status::STATE_CH0_MASK) >> TxCh0status::STATE_CH0_OFFSET
    }

    /// Read **APB_MEM_WADDR_CH0** `[20:11]`
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.bits & TxCh0status::APB_MEM_WADDR_CH0_MASK) >> TxCh0status::APB_MEM_WADDR_CH0_OFFSET
    }

    /// Read **MEM_RADDR_EX_CH0** `[9:0]`
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.bits & TxCh0status::MEM_RADDR_EX_CH0_MASK) >> TxCh0status::MEM_RADDR_EX_CH0_OFFSET
    }
}

impl Default for TxCh0statusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh0status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh0statusVal {
        TxCh0statusVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH1STATUS Register
///
/// Channel 1 status register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **APB_MEM_WR_ERR_CH0** `[26]` (RO)
///
///   This status bit will be set if the offset address out of memory size when writes via APB bus.
///
/// - **MEM_EMPTY_CH0** `[25]` (RO)
///
///   This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
///
/// - **STATE_CH0** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_WADDR_CH0** `[20:11]` (RO)
///
///   This register records the memory address offset when writes RAM over APB bus.
///
/// - **MEM_RADDR_EX_CH0** `[9:0]` (RO)
///
///   This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct TxCh1status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh1status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh1status {
    // ======== Field Constants ========

    /// **APB_MEM_WR_ERR_CH0**
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const APB_MEM_WR_ERR_CH0_OFFSET: u32 = 26;
    pub const APB_MEM_WR_ERR_CH0_MASK: u32 = 0x04000000;
    pub const APB_MEM_WR_ERR_CH0: u32 = 1 << 26;

    /// **MEM_EMPTY_CH0**
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_EMPTY_CH0_OFFSET: u32 = 25;
    pub const MEM_EMPTY_CH0_MASK: u32 = 0x02000000;
    pub const MEM_EMPTY_CH0: u32 = 1 << 25;

    /// **STATE_CH0**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH0_OFFSET: u32 = 22;
    pub const STATE_CH0_MASK: u32 = 0x01c00000;

    /// **APB_MEM_WADDR_CH0**
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_WADDR_CH0_OFFSET: u32 = 11;
    pub const APB_MEM_WADDR_CH0_MASK: u32 = 0x001ff800;

    /// **MEM_RADDR_EX_CH0**
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_RADDR_EX_CH0_OFFSET: u32 = 0;
    pub const MEM_RADDR_EX_CH0_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_WR_ERR_CH0** `[26]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// **MEM_EMPTY_CH0** `[25]` (RO)
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_EMPTY_CH0_MASK) != 0
    }

    /// **STATE_CH0** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH0_MASK) >> Self::STATE_CH0_OFFSET
    }

    /// **APB_MEM_WADDR_CH0** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_WADDR_CH0_MASK) >> Self::APB_MEM_WADDR_CH0_OFFSET
    }

    /// **MEM_RADDR_EX_CH0** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_RADDR_EX_CH0_MASK) >> Self::MEM_RADDR_EX_CH0_OFFSET
    }
}

/// TX_CH1STATUS value type for batch read/modify/write operations
///
/// Use `txch1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh1statusVal {
    bits: u32,
}

impl TxCh1statusVal {
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

    /// Read **APB_MEM_WR_ERR_CH0** `[26:26]`
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub const fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.bits & TxCh1status::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// Read **MEM_EMPTY_CH0** `[25:25]`
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub const fn mem_empty_ch0(&self) -> bool {
        (self.bits & TxCh1status::MEM_EMPTY_CH0_MASK) != 0
    }

    /// Read **STATE_CH0** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch0(&self) -> u32 {
        (self.bits & TxCh1status::STATE_CH0_MASK) >> TxCh1status::STATE_CH0_OFFSET
    }

    /// Read **APB_MEM_WADDR_CH0** `[20:11]`
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.bits & TxCh1status::APB_MEM_WADDR_CH0_MASK) >> TxCh1status::APB_MEM_WADDR_CH0_OFFSET
    }

    /// Read **MEM_RADDR_EX_CH0** `[9:0]`
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.bits & TxCh1status::MEM_RADDR_EX_CH0_MASK) >> TxCh1status::MEM_RADDR_EX_CH0_OFFSET
    }
}

impl Default for TxCh1statusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh1status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh1statusVal {
        TxCh1statusVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH2STATUS Register
///
/// Channel 2 status register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **APB_MEM_WR_ERR_CH0** `[26]` (RO)
///
///   This status bit will be set if the offset address out of memory size when writes via APB bus.
///
/// - **MEM_EMPTY_CH0** `[25]` (RO)
///
///   This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
///
/// - **STATE_CH0** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_WADDR_CH0** `[20:11]` (RO)
///
///   This register records the memory address offset when writes RAM over APB bus.
///
/// - **MEM_RADDR_EX_CH0** `[9:0]` (RO)
///
///   This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct TxCh2status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh2status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh2status {
    // ======== Field Constants ========

    /// **APB_MEM_WR_ERR_CH0**
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const APB_MEM_WR_ERR_CH0_OFFSET: u32 = 26;
    pub const APB_MEM_WR_ERR_CH0_MASK: u32 = 0x04000000;
    pub const APB_MEM_WR_ERR_CH0: u32 = 1 << 26;

    /// **MEM_EMPTY_CH0**
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_EMPTY_CH0_OFFSET: u32 = 25;
    pub const MEM_EMPTY_CH0_MASK: u32 = 0x02000000;
    pub const MEM_EMPTY_CH0: u32 = 1 << 25;

    /// **STATE_CH0**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH0_OFFSET: u32 = 22;
    pub const STATE_CH0_MASK: u32 = 0x01c00000;

    /// **APB_MEM_WADDR_CH0**
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_WADDR_CH0_OFFSET: u32 = 11;
    pub const APB_MEM_WADDR_CH0_MASK: u32 = 0x001ff800;

    /// **MEM_RADDR_EX_CH0**
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_RADDR_EX_CH0_OFFSET: u32 = 0;
    pub const MEM_RADDR_EX_CH0_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_WR_ERR_CH0** `[26]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// **MEM_EMPTY_CH0** `[25]` (RO)
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_EMPTY_CH0_MASK) != 0
    }

    /// **STATE_CH0** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH0_MASK) >> Self::STATE_CH0_OFFSET
    }

    /// **APB_MEM_WADDR_CH0** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_WADDR_CH0_MASK) >> Self::APB_MEM_WADDR_CH0_OFFSET
    }

    /// **MEM_RADDR_EX_CH0** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_RADDR_EX_CH0_MASK) >> Self::MEM_RADDR_EX_CH0_OFFSET
    }
}

/// TX_CH2STATUS value type for batch read/modify/write operations
///
/// Use `txch2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh2statusVal {
    bits: u32,
}

impl TxCh2statusVal {
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

    /// Read **APB_MEM_WR_ERR_CH0** `[26:26]`
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub const fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.bits & TxCh2status::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// Read **MEM_EMPTY_CH0** `[25:25]`
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub const fn mem_empty_ch0(&self) -> bool {
        (self.bits & TxCh2status::MEM_EMPTY_CH0_MASK) != 0
    }

    /// Read **STATE_CH0** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch0(&self) -> u32 {
        (self.bits & TxCh2status::STATE_CH0_MASK) >> TxCh2status::STATE_CH0_OFFSET
    }

    /// Read **APB_MEM_WADDR_CH0** `[20:11]`
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.bits & TxCh2status::APB_MEM_WADDR_CH0_MASK) >> TxCh2status::APB_MEM_WADDR_CH0_OFFSET
    }

    /// Read **MEM_RADDR_EX_CH0** `[9:0]`
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.bits & TxCh2status::MEM_RADDR_EX_CH0_MASK) >> TxCh2status::MEM_RADDR_EX_CH0_OFFSET
    }
}

impl Default for TxCh2statusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh2status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh2statusVal {
        TxCh2statusVal::from_bits(self.value.read_bits())
    }
}

/// TX_CH3STATUS Register
///
/// Channel 3 status register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **APB_MEM_WR_ERR_CH0** `[26]` (RO)
///
///   This status bit will be set if the offset address out of memory size when writes via APB bus.
///
/// - **MEM_EMPTY_CH0** `[25]` (RO)
///
///   This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
///
/// - **STATE_CH0** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_WADDR_CH0** `[20:11]` (RO)
///
///   This register records the memory address offset when writes RAM over APB bus.
///
/// - **MEM_RADDR_EX_CH0** `[9:0]` (RO)
///
///   This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct TxCh3status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCh3status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCh3status {
    // ======== Field Constants ========

    /// **APB_MEM_WR_ERR_CH0**
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const APB_MEM_WR_ERR_CH0_OFFSET: u32 = 26;
    pub const APB_MEM_WR_ERR_CH0_MASK: u32 = 0x04000000;
    pub const APB_MEM_WR_ERR_CH0: u32 = 1 << 26;

    /// **MEM_EMPTY_CH0**
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_EMPTY_CH0_OFFSET: u32 = 25;
    pub const MEM_EMPTY_CH0_MASK: u32 = 0x02000000;
    pub const MEM_EMPTY_CH0: u32 = 1 << 25;

    /// **STATE_CH0**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH0_OFFSET: u32 = 22;
    pub const STATE_CH0_MASK: u32 = 0x01c00000;

    /// **APB_MEM_WADDR_CH0**
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_WADDR_CH0_OFFSET: u32 = 11;
    pub const APB_MEM_WADDR_CH0_MASK: u32 = 0x001ff800;

    /// **MEM_RADDR_EX_CH0**
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_RADDR_EX_CH0_OFFSET: u32 = 0;
    pub const MEM_RADDR_EX_CH0_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_WR_ERR_CH0** `[26]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// **MEM_EMPTY_CH0** `[25]` (RO)
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> bool {
        (self.value.read_bits() & Self::MEM_EMPTY_CH0_MASK) != 0
    }

    /// **STATE_CH0** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH0_MASK) >> Self::STATE_CH0_OFFSET
    }

    /// **APB_MEM_WADDR_CH0** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_WADDR_CH0_MASK) >> Self::APB_MEM_WADDR_CH0_OFFSET
    }

    /// **MEM_RADDR_EX_CH0** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_RADDR_EX_CH0_MASK) >> Self::MEM_RADDR_EX_CH0_OFFSET
    }
}

/// TX_CH3STATUS value type for batch read/modify/write operations
///
/// Use `txch3status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txch3status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCh3statusVal {
    bits: u32,
}

impl TxCh3statusVal {
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

    /// Read **APB_MEM_WR_ERR_CH0** `[26:26]`
    ///
    /// This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub const fn apb_mem_wr_err_ch0(&self) -> bool {
        (self.bits & TxCh3status::APB_MEM_WR_ERR_CH0_MASK) != 0
    }

    /// Read **MEM_EMPTY_CH0** `[25:25]`
    ///
    /// This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub const fn mem_empty_ch0(&self) -> bool {
        (self.bits & TxCh3status::MEM_EMPTY_CH0_MASK) != 0
    }

    /// Read **STATE_CH0** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch0(&self) -> u32 {
        (self.bits & TxCh3status::STATE_CH0_MASK) >> TxCh3status::STATE_CH0_OFFSET
    }

    /// Read **APB_MEM_WADDR_CH0** `[20:11]`
    ///
    /// This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_waddr_ch0(&self) -> u32 {
        (self.bits & TxCh3status::APB_MEM_WADDR_CH0_MASK) >> TxCh3status::APB_MEM_WADDR_CH0_OFFSET
    }

    /// Read **MEM_RADDR_EX_CH0** `[9:0]`
    ///
    /// This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_raddr_ex_ch0(&self) -> u32 {
        (self.bits & TxCh3status::MEM_RADDR_EX_CH0_MASK) >> TxCh3status::MEM_RADDR_EX_CH0_OFFSET
    }
}

impl Default for TxCh3statusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCh3status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCh3statusVal {
        TxCh3statusVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH0STATUS Register
///
/// Channel 0 status register
///
/// Address offset: 0x0060
/// Reset value: 0x000600c0
///
/// ## Fields
///
/// - **APB_MEM_RD_ERR_CH4** `[27]` (RO)
///
///   This status bit will be set if the offset address out of memory size when reads via APB bus.
///
/// - **MEM_FULL_CH4** `[26]` (RO)
///
///   This status bit will be set if the receiver receives more data than the memory size.
///
/// - **MEM_OWNER_ERR_CH4** `[25]` (RO)
///
///   This status bit will be set when the ownership of memory block is wrong.
///
/// - **STATE_CH4** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_RADDR_CH4** `[20:11]` (RO)
///
///   This register records the memory address offset when reads RAM over APB bus.
///
/// - **MEM_WADDR_EX_CH4** `[9:0]` (RO)
///
///   This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct RxCh0status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh0status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh0status {
    // ======== Field Constants ========

    /// **APB_MEM_RD_ERR_CH4**
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const APB_MEM_RD_ERR_CH4_OFFSET: u32 = 27;
    pub const APB_MEM_RD_ERR_CH4_MASK: u32 = 0x08000000;
    pub const APB_MEM_RD_ERR_CH4: u32 = 1 << 27;

    /// **MEM_FULL_CH4**
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MEM_FULL_CH4_OFFSET: u32 = 26;
    pub const MEM_FULL_CH4_MASK: u32 = 0x04000000;
    pub const MEM_FULL_CH4: u32 = 1 << 26;

    /// **MEM_OWNER_ERR_CH4**
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_OWNER_ERR_CH4_OFFSET: u32 = 25;
    pub const MEM_OWNER_ERR_CH4_MASK: u32 = 0x02000000;
    pub const MEM_OWNER_ERR_CH4: u32 = 1 << 25;

    /// **STATE_CH4**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH4_OFFSET: u32 = 22;
    pub const STATE_CH4_MASK: u32 = 0x01c00000;

    /// **APB_MEM_RADDR_CH4**
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_RADDR_CH4_OFFSET: u32 = 11;
    pub const APB_MEM_RADDR_CH4_MASK: u32 = 0x001ff800;

    /// **MEM_WADDR_EX_CH4**
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_WADDR_EX_CH4_OFFSET: u32 = 0;
    pub const MEM_WADDR_EX_CH4_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_RD_ERR_CH4** `[27]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// **MEM_FULL_CH4** `[26]` (RO)
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_CH4_MASK) != 0
    }

    /// **MEM_OWNER_ERR_CH4** `[25]` (RO)
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// **STATE_CH4** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH4_MASK) >> Self::STATE_CH4_OFFSET
    }

    /// **APB_MEM_RADDR_CH4** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_RADDR_CH4_MASK) >> Self::APB_MEM_RADDR_CH4_OFFSET
    }

    /// **MEM_WADDR_EX_CH4** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_WADDR_EX_CH4_MASK) >> Self::MEM_WADDR_EX_CH4_OFFSET
    }
}

/// RX_CH0STATUS value type for batch read/modify/write operations
///
/// Use `rxch0status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch0status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh0statusVal {
    bits: u32,
}

impl RxCh0statusVal {
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
        Self { bits: 0x000600c0 }
    }

    /// Read **APB_MEM_RD_ERR_CH4** `[27:27]`
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub const fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.bits & RxCh0status::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// Read **MEM_FULL_CH4** `[26:26]`
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub const fn mem_full_ch4(&self) -> bool {
        (self.bits & RxCh0status::MEM_FULL_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_ERR_CH4** `[25:25]`
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub const fn mem_owner_err_ch4(&self) -> bool {
        (self.bits & RxCh0status::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// Read **STATE_CH4** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch4(&self) -> u32 {
        (self.bits & RxCh0status::STATE_CH4_MASK) >> RxCh0status::STATE_CH4_OFFSET
    }

    /// Read **APB_MEM_RADDR_CH4** `[20:11]`
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.bits & RxCh0status::APB_MEM_RADDR_CH4_MASK) >> RxCh0status::APB_MEM_RADDR_CH4_OFFSET
    }

    /// Read **MEM_WADDR_EX_CH4** `[9:0]`
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.bits & RxCh0status::MEM_WADDR_EX_CH4_MASK) >> RxCh0status::MEM_WADDR_EX_CH4_OFFSET
    }
}

impl Default for RxCh0statusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh0status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh0statusVal {
        RxCh0statusVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH1STATUS Register
///
/// Channel 1 status register
///
/// Address offset: 0x0064
/// Reset value: 0x000600c0
///
/// ## Fields
///
/// - **APB_MEM_RD_ERR_CH4** `[27]` (RO)
///
///   This status bit will be set if the offset address out of memory size when reads via APB bus.
///
/// - **MEM_FULL_CH4** `[26]` (RO)
///
///   This status bit will be set if the receiver receives more data than the memory size.
///
/// - **MEM_OWNER_ERR_CH4** `[25]` (RO)
///
///   This status bit will be set when the ownership of memory block is wrong.
///
/// - **STATE_CH4** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_RADDR_CH4** `[20:11]` (RO)
///
///   This register records the memory address offset when reads RAM over APB bus.
///
/// - **MEM_WADDR_EX_CH4** `[9:0]` (RO)
///
///   This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct RxCh1status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh1status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh1status {
    // ======== Field Constants ========

    /// **APB_MEM_RD_ERR_CH4**
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const APB_MEM_RD_ERR_CH4_OFFSET: u32 = 27;
    pub const APB_MEM_RD_ERR_CH4_MASK: u32 = 0x08000000;
    pub const APB_MEM_RD_ERR_CH4: u32 = 1 << 27;

    /// **MEM_FULL_CH4**
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MEM_FULL_CH4_OFFSET: u32 = 26;
    pub const MEM_FULL_CH4_MASK: u32 = 0x04000000;
    pub const MEM_FULL_CH4: u32 = 1 << 26;

    /// **MEM_OWNER_ERR_CH4**
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_OWNER_ERR_CH4_OFFSET: u32 = 25;
    pub const MEM_OWNER_ERR_CH4_MASK: u32 = 0x02000000;
    pub const MEM_OWNER_ERR_CH4: u32 = 1 << 25;

    /// **STATE_CH4**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH4_OFFSET: u32 = 22;
    pub const STATE_CH4_MASK: u32 = 0x01c00000;

    /// **APB_MEM_RADDR_CH4**
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_RADDR_CH4_OFFSET: u32 = 11;
    pub const APB_MEM_RADDR_CH4_MASK: u32 = 0x001ff800;

    /// **MEM_WADDR_EX_CH4**
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_WADDR_EX_CH4_OFFSET: u32 = 0;
    pub const MEM_WADDR_EX_CH4_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_RD_ERR_CH4** `[27]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// **MEM_FULL_CH4** `[26]` (RO)
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_CH4_MASK) != 0
    }

    /// **MEM_OWNER_ERR_CH4** `[25]` (RO)
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// **STATE_CH4** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH4_MASK) >> Self::STATE_CH4_OFFSET
    }

    /// **APB_MEM_RADDR_CH4** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_RADDR_CH4_MASK) >> Self::APB_MEM_RADDR_CH4_OFFSET
    }

    /// **MEM_WADDR_EX_CH4** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_WADDR_EX_CH4_MASK) >> Self::MEM_WADDR_EX_CH4_OFFSET
    }
}

/// RX_CH1STATUS value type for batch read/modify/write operations
///
/// Use `rxch1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh1statusVal {
    bits: u32,
}

impl RxCh1statusVal {
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
        Self { bits: 0x000600c0 }
    }

    /// Read **APB_MEM_RD_ERR_CH4** `[27:27]`
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub const fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.bits & RxCh1status::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// Read **MEM_FULL_CH4** `[26:26]`
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub const fn mem_full_ch4(&self) -> bool {
        (self.bits & RxCh1status::MEM_FULL_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_ERR_CH4** `[25:25]`
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub const fn mem_owner_err_ch4(&self) -> bool {
        (self.bits & RxCh1status::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// Read **STATE_CH4** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch4(&self) -> u32 {
        (self.bits & RxCh1status::STATE_CH4_MASK) >> RxCh1status::STATE_CH4_OFFSET
    }

    /// Read **APB_MEM_RADDR_CH4** `[20:11]`
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.bits & RxCh1status::APB_MEM_RADDR_CH4_MASK) >> RxCh1status::APB_MEM_RADDR_CH4_OFFSET
    }

    /// Read **MEM_WADDR_EX_CH4** `[9:0]`
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.bits & RxCh1status::MEM_WADDR_EX_CH4_MASK) >> RxCh1status::MEM_WADDR_EX_CH4_OFFSET
    }
}

impl Default for RxCh1statusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh1status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh1statusVal {
        RxCh1statusVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH2STATUS Register
///
/// Channel 2 status register
///
/// Address offset: 0x0068
/// Reset value: 0x000600c0
///
/// ## Fields
///
/// - **APB_MEM_RD_ERR_CH4** `[27]` (RO)
///
///   This status bit will be set if the offset address out of memory size when reads via APB bus.
///
/// - **MEM_FULL_CH4** `[26]` (RO)
///
///   This status bit will be set if the receiver receives more data than the memory size.
///
/// - **MEM_OWNER_ERR_CH4** `[25]` (RO)
///
///   This status bit will be set when the ownership of memory block is wrong.
///
/// - **STATE_CH4** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_RADDR_CH4** `[20:11]` (RO)
///
///   This register records the memory address offset when reads RAM over APB bus.
///
/// - **MEM_WADDR_EX_CH4** `[9:0]` (RO)
///
///   This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct RxCh2status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh2status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh2status {
    // ======== Field Constants ========

    /// **APB_MEM_RD_ERR_CH4**
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const APB_MEM_RD_ERR_CH4_OFFSET: u32 = 27;
    pub const APB_MEM_RD_ERR_CH4_MASK: u32 = 0x08000000;
    pub const APB_MEM_RD_ERR_CH4: u32 = 1 << 27;

    /// **MEM_FULL_CH4**
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MEM_FULL_CH4_OFFSET: u32 = 26;
    pub const MEM_FULL_CH4_MASK: u32 = 0x04000000;
    pub const MEM_FULL_CH4: u32 = 1 << 26;

    /// **MEM_OWNER_ERR_CH4**
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_OWNER_ERR_CH4_OFFSET: u32 = 25;
    pub const MEM_OWNER_ERR_CH4_MASK: u32 = 0x02000000;
    pub const MEM_OWNER_ERR_CH4: u32 = 1 << 25;

    /// **STATE_CH4**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH4_OFFSET: u32 = 22;
    pub const STATE_CH4_MASK: u32 = 0x01c00000;

    /// **APB_MEM_RADDR_CH4**
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_RADDR_CH4_OFFSET: u32 = 11;
    pub const APB_MEM_RADDR_CH4_MASK: u32 = 0x001ff800;

    /// **MEM_WADDR_EX_CH4**
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_WADDR_EX_CH4_OFFSET: u32 = 0;
    pub const MEM_WADDR_EX_CH4_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_RD_ERR_CH4** `[27]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// **MEM_FULL_CH4** `[26]` (RO)
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_CH4_MASK) != 0
    }

    /// **MEM_OWNER_ERR_CH4** `[25]` (RO)
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// **STATE_CH4** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH4_MASK) >> Self::STATE_CH4_OFFSET
    }

    /// **APB_MEM_RADDR_CH4** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_RADDR_CH4_MASK) >> Self::APB_MEM_RADDR_CH4_OFFSET
    }

    /// **MEM_WADDR_EX_CH4** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_WADDR_EX_CH4_MASK) >> Self::MEM_WADDR_EX_CH4_OFFSET
    }
}

/// RX_CH2STATUS value type for batch read/modify/write operations
///
/// Use `rxch2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh2statusVal {
    bits: u32,
}

impl RxCh2statusVal {
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
        Self { bits: 0x000600c0 }
    }

    /// Read **APB_MEM_RD_ERR_CH4** `[27:27]`
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub const fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.bits & RxCh2status::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// Read **MEM_FULL_CH4** `[26:26]`
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub const fn mem_full_ch4(&self) -> bool {
        (self.bits & RxCh2status::MEM_FULL_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_ERR_CH4** `[25:25]`
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub const fn mem_owner_err_ch4(&self) -> bool {
        (self.bits & RxCh2status::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// Read **STATE_CH4** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch4(&self) -> u32 {
        (self.bits & RxCh2status::STATE_CH4_MASK) >> RxCh2status::STATE_CH4_OFFSET
    }

    /// Read **APB_MEM_RADDR_CH4** `[20:11]`
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.bits & RxCh2status::APB_MEM_RADDR_CH4_MASK) >> RxCh2status::APB_MEM_RADDR_CH4_OFFSET
    }

    /// Read **MEM_WADDR_EX_CH4** `[9:0]`
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.bits & RxCh2status::MEM_WADDR_EX_CH4_MASK) >> RxCh2status::MEM_WADDR_EX_CH4_OFFSET
    }
}

impl Default for RxCh2statusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh2status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh2statusVal {
        RxCh2statusVal::from_bits(self.value.read_bits())
    }
}

/// RX_CH3STATUS Register
///
/// Channel 3 status register
///
/// Address offset: 0x006c
/// Reset value: 0x000600c0
///
/// ## Fields
///
/// - **APB_MEM_RD_ERR_CH4** `[27]` (RO)
///
///   This status bit will be set if the offset address out of memory size when reads via APB bus.
///
/// - **MEM_FULL_CH4** `[26]` (RO)
///
///   This status bit will be set if the receiver receives more data than the memory size.
///
/// - **MEM_OWNER_ERR_CH4** `[25]` (RO)
///
///   This status bit will be set when the ownership of memory block is wrong.
///
/// - **STATE_CH4** `[24:22]` (RO)
///
///   This register records the FSM status of CHANNEL%s.
///
/// - **APB_MEM_RADDR_CH4** `[20:11]` (RO)
///
///   This register records the memory address offset when reads RAM over APB bus.
///
/// - **MEM_WADDR_EX_CH4** `[9:0]` (RO)
///
///   This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
#[repr(transparent)]
pub struct RxCh3status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCh3status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCh3status {
    // ======== Field Constants ========

    /// **APB_MEM_RD_ERR_CH4**
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const APB_MEM_RD_ERR_CH4_OFFSET: u32 = 27;
    pub const APB_MEM_RD_ERR_CH4_MASK: u32 = 0x08000000;
    pub const APB_MEM_RD_ERR_CH4: u32 = 1 << 27;

    /// **MEM_FULL_CH4**
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MEM_FULL_CH4_OFFSET: u32 = 26;
    pub const MEM_FULL_CH4_MASK: u32 = 0x04000000;
    pub const MEM_FULL_CH4: u32 = 1 << 26;

    /// **MEM_OWNER_ERR_CH4**
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_OWNER_ERR_CH4_OFFSET: u32 = 25;
    pub const MEM_OWNER_ERR_CH4_MASK: u32 = 0x02000000;
    pub const MEM_OWNER_ERR_CH4: u32 = 1 << 25;

    /// **STATE_CH4**
    ///
    /// This register records the FSM status of CHANNEL%s.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const STATE_CH4_OFFSET: u32 = 22;
    pub const STATE_CH4_MASK: u32 = 0x01c00000;

    /// **APB_MEM_RADDR_CH4**
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const APB_MEM_RADDR_CH4_OFFSET: u32 = 11;
    pub const APB_MEM_RADDR_CH4_MASK: u32 = 0x001ff800;

    /// **MEM_WADDR_EX_CH4**
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const MEM_WADDR_EX_CH4_OFFSET: u32 = 0;
    pub const MEM_WADDR_EX_CH4_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **APB_MEM_RD_ERR_CH4** `[27]` (RO)
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// **MEM_FULL_CH4** `[26]` (RO)
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_CH4_MASK) != 0
    }

    /// **MEM_OWNER_ERR_CH4** `[25]` (RO)
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> bool {
        (self.value.read_bits() & Self::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// **STATE_CH4** `[24:22]` (RO)
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_CH4_MASK) >> Self::STATE_CH4_OFFSET
    }

    /// **APB_MEM_RADDR_CH4** `[20:11]` (RO)
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::APB_MEM_RADDR_CH4_MASK) >> Self::APB_MEM_RADDR_CH4_OFFSET
    }

    /// **MEM_WADDR_EX_CH4** `[9:0]` (RO)
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_WADDR_EX_CH4_MASK) >> Self::MEM_WADDR_EX_CH4_OFFSET
    }
}

/// RX_CH3STATUS value type for batch read/modify/write operations
///
/// Use `rxch3status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxch3status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCh3statusVal {
    bits: u32,
}

impl RxCh3statusVal {
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
        Self { bits: 0x000600c0 }
    }

    /// Read **APB_MEM_RD_ERR_CH4** `[27:27]`
    ///
    /// This status bit will be set if the offset address out of memory size when reads via APB bus.
    #[inline(always)]
    pub const fn apb_mem_rd_err_ch4(&self) -> bool {
        (self.bits & RxCh3status::APB_MEM_RD_ERR_CH4_MASK) != 0
    }

    /// Read **MEM_FULL_CH4** `[26:26]`
    ///
    /// This status bit will be set if the receiver receives more data than the memory size.
    #[inline(always)]
    pub const fn mem_full_ch4(&self) -> bool {
        (self.bits & RxCh3status::MEM_FULL_CH4_MASK) != 0
    }

    /// Read **MEM_OWNER_ERR_CH4** `[25:25]`
    ///
    /// This status bit will be set when the ownership of memory block is wrong.
    #[inline(always)]
    pub const fn mem_owner_err_ch4(&self) -> bool {
        (self.bits & RxCh3status::MEM_OWNER_ERR_CH4_MASK) != 0
    }

    /// Read **STATE_CH4** `[24:22]`
    ///
    /// This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub const fn state_ch4(&self) -> u32 {
        (self.bits & RxCh3status::STATE_CH4_MASK) >> RxCh3status::STATE_CH4_OFFSET
    }

    /// Read **APB_MEM_RADDR_CH4** `[20:11]`
    ///
    /// This register records the memory address offset when reads RAM over APB bus.
    #[inline(always)]
    pub const fn apb_mem_raddr_ch4(&self) -> u32 {
        (self.bits & RxCh3status::APB_MEM_RADDR_CH4_MASK) >> RxCh3status::APB_MEM_RADDR_CH4_OFFSET
    }

    /// Read **MEM_WADDR_EX_CH4** `[9:0]`
    ///
    /// This register records the memory address offset when receiver of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub const fn mem_waddr_ex_ch4(&self) -> u32 {
        (self.bits & RxCh3status::MEM_WADDR_EX_CH4_MASK) >> RxCh3status::MEM_WADDR_EX_CH4_OFFSET
    }
}

impl Default for RxCh3statusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxCh3status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCh3statusVal {
        RxCh3statusVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// Raw interrupt status
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **RX_CH7_DMA_ACCESS_FAIL_INT_RAW** `[29]` (RW)
///
///   The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails.
///
/// - **TX_CH3_DMA_ACCESS_FAIL_INT_RAW** `[28]` (RW)
///
///   The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails.
///
/// - **CH7_RX_THR_EVENT_INT_RAW** `[27]` (RW)
///
///   The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value.
///
/// - **CH6_RX_THR_EVENT_INT_RAW** `[26]` (RW)
///
///   The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value.
///
/// - **CH5_RX_THR_EVENT_INT_RAW** `[25]` (RW)
///
///   The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value.
///
/// - **CH4_RX_THR_EVENT_INT_RAW** `[24]` (RW)
///
///   The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value.
///
/// - **RX_CH7_ERR_INT_RAW** `[23]` (RW)
///
///   The interrupt raw bit for CHANNEL7. Triggered when error occurs.
///
/// - **RX_CH6_ERR_INT_RAW** `[22]` (RW)
///
///   The interrupt raw bit for CHANNEL6. Triggered when error occurs.
///
/// - **RX_CH5_ERR_INT_RAW** `[21]` (RW)
///
///   The interrupt raw bit for CHANNEL5. Triggered when error occurs.
///
/// - **RX_CH4_ERR_INT_RAW** `[20]` (RW)
///
///   The interrupt raw bit for CHANNEL4. Triggered when error occurs.
///
/// - **CH7_RX_END_INT_RAW** `[19]` (RW)
///
///   The interrupt raw bit for CHANNEL7. Triggered when reception done.
///
/// - **CH6_RX_END_INT_RAW** `[18]` (RW)
///
///   The interrupt raw bit for CHANNEL6. Triggered when reception done.
///
/// - **CH5_RX_END_INT_RAW** `[17]` (RW)
///
///   The interrupt raw bit for CHANNEL5. Triggered when reception done.
///
/// - **CH4_RX_END_INT_RAW** `[16]` (RW)
///
///   The interrupt raw bit for CHANNEL4. Triggered when reception done.
///
/// - **CH3_TX_LOOP_INT_RAW** `[15]` (RW)
///
///   The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
///
/// - **CH2_TX_LOOP_INT_RAW** `[14]` (RW)
///
///   The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
///
/// - **CH1_TX_LOOP_INT_RAW** `[13]` (RW)
///
///   The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
///
/// - **CH0_TX_LOOP_INT_RAW** `[12]` (RW)
///
///   The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
///
/// - **CH3_TX_THR_EVENT_INT_RAW** `[11]` (RW)
///
///   The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
///
/// - **CH2_TX_THR_EVENT_INT_RAW** `[10]` (RW)
///
///   The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
///
/// - **CH1_TX_THR_EVENT_INT_RAW** `[9]` (RW)
///
///   The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
///
/// - **CH0_TX_THR_EVENT_INT_RAW** `[8]` (RW)
///
///   The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
///
/// - **TX_CH3_ERR_INT_RAW** `[7]` (RW)
///
///   The interrupt raw bit for CHANNEL3. Triggered when error occurs.
///
/// - **TX_CH2_ERR_INT_RAW** `[6]` (RW)
///
///   The interrupt raw bit for CHANNEL2. Triggered when error occurs.
///
/// - **TX_CH1_ERR_INT_RAW** `[5]` (RW)
///
///   The interrupt raw bit for CHANNEL1. Triggered when error occurs.
///
/// - **TX_CH0_ERR_INT_RAW** `[4]` (RW)
///
///   The interrupt raw bit for CHANNEL0. Triggered when error occurs.
///
/// - **CH3_TX_END_INT_RAW** `[3]` (RW)
///
///   The interrupt raw bit for CHANNEL3. Triggered when transmission done.
///
/// - **CH2_TX_END_INT_RAW** `[2]` (RW)
///
///   The interrupt raw bit for CHANNEL2. Triggered when transmission done.
///
/// - **CH1_TX_END_INT_RAW** `[1]` (RW)
///
///   The interrupt raw bit for CHANNEL1. Triggered when transmission done.
///
/// - **CH0_TX_END_INT_RAW** `[0]` (RW)
///
///   The interrupt raw bit for CHANNEL0. Triggered when transmission done.
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

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_RAW_OFFSET: u32 = 29;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK: u32 = 0x20000000;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_RAW: u32 = 1 << 29;

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_RAW_OFFSET: u32 = 28;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK: u32 = 0x10000000;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_RAW: u32 = 1 << 28;

    /// **CH7_RX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CH7_RX_THR_EVENT_INT_RAW_OFFSET: u32 = 27;
    pub const CH7_RX_THR_EVENT_INT_RAW_MASK: u32 = 0x08000000;
    pub const CH7_RX_THR_EVENT_INT_RAW: u32 = 1 << 27;

    /// **CH6_RX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CH6_RX_THR_EVENT_INT_RAW_OFFSET: u32 = 26;
    pub const CH6_RX_THR_EVENT_INT_RAW_MASK: u32 = 0x04000000;
    pub const CH6_RX_THR_EVENT_INT_RAW: u32 = 1 << 26;

    /// **CH5_RX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CH5_RX_THR_EVENT_INT_RAW_OFFSET: u32 = 25;
    pub const CH5_RX_THR_EVENT_INT_RAW_MASK: u32 = 0x02000000;
    pub const CH5_RX_THR_EVENT_INT_RAW: u32 = 1 << 25;

    /// **CH4_RX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CH4_RX_THR_EVENT_INT_RAW_OFFSET: u32 = 24;
    pub const CH4_RX_THR_EVENT_INT_RAW_MASK: u32 = 0x01000000;
    pub const CH4_RX_THR_EVENT_INT_RAW: u32 = 1 << 24;

    /// **RX_CH7_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when error occurs.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RX_CH7_ERR_INT_RAW_OFFSET: u32 = 23;
    pub const RX_CH7_ERR_INT_RAW_MASK: u32 = 0x00800000;
    pub const RX_CH7_ERR_INT_RAW: u32 = 1 << 23;

    /// **RX_CH6_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when error occurs.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const RX_CH6_ERR_INT_RAW_OFFSET: u32 = 22;
    pub const RX_CH6_ERR_INT_RAW_MASK: u32 = 0x00400000;
    pub const RX_CH6_ERR_INT_RAW: u32 = 1 << 22;

    /// **RX_CH5_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when error occurs.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const RX_CH5_ERR_INT_RAW_OFFSET: u32 = 21;
    pub const RX_CH5_ERR_INT_RAW_MASK: u32 = 0x00200000;
    pub const RX_CH5_ERR_INT_RAW: u32 = 1 << 21;

    /// **RX_CH4_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when error occurs.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_CH4_ERR_INT_RAW_OFFSET: u32 = 20;
    pub const RX_CH4_ERR_INT_RAW_MASK: u32 = 0x00100000;
    pub const RX_CH4_ERR_INT_RAW: u32 = 1 << 20;

    /// **CH7_RX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when reception done.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CH7_RX_END_INT_RAW_OFFSET: u32 = 19;
    pub const CH7_RX_END_INT_RAW_MASK: u32 = 0x00080000;
    pub const CH7_RX_END_INT_RAW: u32 = 1 << 19;

    /// **CH6_RX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when reception done.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CH6_RX_END_INT_RAW_OFFSET: u32 = 18;
    pub const CH6_RX_END_INT_RAW_MASK: u32 = 0x00040000;
    pub const CH6_RX_END_INT_RAW: u32 = 1 << 18;

    /// **CH5_RX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when reception done.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CH5_RX_END_INT_RAW_OFFSET: u32 = 17;
    pub const CH5_RX_END_INT_RAW_MASK: u32 = 0x00020000;
    pub const CH5_RX_END_INT_RAW: u32 = 1 << 17;

    /// **CH4_RX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when reception done.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CH4_RX_END_INT_RAW_OFFSET: u32 = 16;
    pub const CH4_RX_END_INT_RAW_MASK: u32 = 0x00010000;
    pub const CH4_RX_END_INT_RAW: u32 = 1 << 16;

    /// **CH3_TX_LOOP_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CH3_TX_LOOP_INT_RAW_OFFSET: u32 = 15;
    pub const CH3_TX_LOOP_INT_RAW_MASK: u32 = 0x00008000;
    pub const CH3_TX_LOOP_INT_RAW: u32 = 1 << 15;

    /// **CH2_TX_LOOP_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CH2_TX_LOOP_INT_RAW_OFFSET: u32 = 14;
    pub const CH2_TX_LOOP_INT_RAW_MASK: u32 = 0x00004000;
    pub const CH2_TX_LOOP_INT_RAW: u32 = 1 << 14;

    /// **CH1_TX_LOOP_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CH1_TX_LOOP_INT_RAW_OFFSET: u32 = 13;
    pub const CH1_TX_LOOP_INT_RAW_MASK: u32 = 0x00002000;
    pub const CH1_TX_LOOP_INT_RAW: u32 = 1 << 13;

    /// **CH0_TX_LOOP_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CH0_TX_LOOP_INT_RAW_OFFSET: u32 = 12;
    pub const CH0_TX_LOOP_INT_RAW_MASK: u32 = 0x00001000;
    pub const CH0_TX_LOOP_INT_RAW: u32 = 1 << 12;

    /// **CH3_TX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH3_TX_THR_EVENT_INT_RAW_OFFSET: u32 = 11;
    pub const CH3_TX_THR_EVENT_INT_RAW_MASK: u32 = 0x00000800;
    pub const CH3_TX_THR_EVENT_INT_RAW: u32 = 1 << 11;

    /// **CH2_TX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH2_TX_THR_EVENT_INT_RAW_OFFSET: u32 = 10;
    pub const CH2_TX_THR_EVENT_INT_RAW_MASK: u32 = 0x00000400;
    pub const CH2_TX_THR_EVENT_INT_RAW: u32 = 1 << 10;

    /// **CH1_TX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH1_TX_THR_EVENT_INT_RAW_OFFSET: u32 = 9;
    pub const CH1_TX_THR_EVENT_INT_RAW_MASK: u32 = 0x00000200;
    pub const CH1_TX_THR_EVENT_INT_RAW: u32 = 1 << 9;

    /// **CH0_TX_THR_EVENT_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH0_TX_THR_EVENT_INT_RAW_OFFSET: u32 = 8;
    pub const CH0_TX_THR_EVENT_INT_RAW_MASK: u32 = 0x00000100;
    pub const CH0_TX_THR_EVENT_INT_RAW: u32 = 1 << 8;

    /// **TX_CH3_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when error occurs.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_CH3_ERR_INT_RAW_OFFSET: u32 = 7;
    pub const TX_CH3_ERR_INT_RAW_MASK: u32 = 0x00000080;
    pub const TX_CH3_ERR_INT_RAW: u32 = 1 << 7;

    /// **TX_CH2_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when error occurs.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_CH2_ERR_INT_RAW_OFFSET: u32 = 6;
    pub const TX_CH2_ERR_INT_RAW_MASK: u32 = 0x00000040;
    pub const TX_CH2_ERR_INT_RAW: u32 = 1 << 6;

    /// **TX_CH1_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when error occurs.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_CH1_ERR_INT_RAW_OFFSET: u32 = 5;
    pub const TX_CH1_ERR_INT_RAW_MASK: u32 = 0x00000020;
    pub const TX_CH1_ERR_INT_RAW: u32 = 1 << 5;

    /// **TX_CH0_ERR_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when error occurs.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_CH0_ERR_INT_RAW_OFFSET: u32 = 4;
    pub const TX_CH0_ERR_INT_RAW_MASK: u32 = 0x00000010;
    pub const TX_CH0_ERR_INT_RAW: u32 = 1 << 4;

    /// **CH3_TX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmission done.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_TX_END_INT_RAW_OFFSET: u32 = 3;
    pub const CH3_TX_END_INT_RAW_MASK: u32 = 0x00000008;
    pub const CH3_TX_END_INT_RAW: u32 = 1 << 3;

    /// **CH2_TX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmission done.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_TX_END_INT_RAW_OFFSET: u32 = 2;
    pub const CH2_TX_END_INT_RAW_MASK: u32 = 0x00000004;
    pub const CH2_TX_END_INT_RAW: u32 = 1 << 2;

    /// **CH1_TX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmission done.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_TX_END_INT_RAW_OFFSET: u32 = 1;
    pub const CH1_TX_END_INT_RAW_MASK: u32 = 0x00000002;
    pub const CH1_TX_END_INT_RAW: u32 = 1 << 1;

    /// **CH0_TX_END_INT_RAW**
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmission done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_TX_END_INT_RAW_OFFSET: u32 = 0;
    pub const CH0_TX_END_INT_RAW_MASK: u32 = 0x00000001;
    pub const CH0_TX_END_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_RAW** `[29]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails.
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK) != 0
    }

    /// Set RX_CH7_DMA_ACCESS_FAIL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch7_dma_access_fail_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK);
    }

    /// Clear RX_CH7_DMA_ACCESS_FAIL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch7_dma_access_fail_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK);
    }

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_RAW** `[28]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails.
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK) != 0
    }

    /// Set TX_CH3_DMA_ACCESS_FAIL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch3_dma_access_fail_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK);
    }

    /// Clear TX_CH3_DMA_ACCESS_FAIL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch3_dma_access_fail_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK);
    }

    /// **CH7_RX_THR_EVENT_INT_RAW** `[27]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH7_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch7_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH7_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH7_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch7_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH7_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH6_RX_THR_EVENT_INT_RAW** `[26]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH6_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch6_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH6_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH6_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch6_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH6_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH5_RX_THR_EVENT_INT_RAW** `[25]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH5_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch5_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH5_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH5_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch5_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH5_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH4_RX_THR_EVENT_INT_RAW** `[24]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH4_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch4_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH4_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH4_RX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch4_rx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH4_RX_THR_EVENT_INT_RAW_MASK);
    }

    /// **RX_CH7_ERR_INT_RAW** `[23]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when error occurs.
    #[inline(always)]
    pub fn rx_ch7_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH7_ERR_INT_RAW_MASK) != 0
    }

    /// Set RX_CH7_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch7_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH7_ERR_INT_RAW_MASK);
    }

    /// Clear RX_CH7_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch7_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH7_ERR_INT_RAW_MASK);
    }

    /// **RX_CH6_ERR_INT_RAW** `[22]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when error occurs.
    #[inline(always)]
    pub fn rx_ch6_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH6_ERR_INT_RAW_MASK) != 0
    }

    /// Set RX_CH6_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch6_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH6_ERR_INT_RAW_MASK);
    }

    /// Clear RX_CH6_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch6_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH6_ERR_INT_RAW_MASK);
    }

    /// **RX_CH5_ERR_INT_RAW** `[21]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when error occurs.
    #[inline(always)]
    pub fn rx_ch5_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH5_ERR_INT_RAW_MASK) != 0
    }

    /// Set RX_CH5_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch5_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH5_ERR_INT_RAW_MASK);
    }

    /// Clear RX_CH5_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch5_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH5_ERR_INT_RAW_MASK);
    }

    /// **RX_CH4_ERR_INT_RAW** `[20]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when error occurs.
    #[inline(always)]
    pub fn rx_ch4_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH4_ERR_INT_RAW_MASK) != 0
    }

    /// Set RX_CH4_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch4_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH4_ERR_INT_RAW_MASK);
    }

    /// Clear RX_CH4_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch4_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH4_ERR_INT_RAW_MASK);
    }

    /// **CH7_RX_END_INT_RAW** `[19]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when reception done.
    #[inline(always)]
    pub fn ch7_rx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_END_INT_RAW_MASK) != 0
    }

    /// Set CH7_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch7_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH7_RX_END_INT_RAW_MASK);
    }

    /// Clear CH7_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch7_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH7_RX_END_INT_RAW_MASK);
    }

    /// **CH6_RX_END_INT_RAW** `[18]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when reception done.
    #[inline(always)]
    pub fn ch6_rx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_END_INT_RAW_MASK) != 0
    }

    /// Set CH6_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch6_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH6_RX_END_INT_RAW_MASK);
    }

    /// Clear CH6_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch6_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH6_RX_END_INT_RAW_MASK);
    }

    /// **CH5_RX_END_INT_RAW** `[17]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when reception done.
    #[inline(always)]
    pub fn ch5_rx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_END_INT_RAW_MASK) != 0
    }

    /// Set CH5_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch5_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH5_RX_END_INT_RAW_MASK);
    }

    /// Clear CH5_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch5_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH5_RX_END_INT_RAW_MASK);
    }

    /// **CH4_RX_END_INT_RAW** `[16]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when reception done.
    #[inline(always)]
    pub fn ch4_rx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_END_INT_RAW_MASK) != 0
    }

    /// Set CH4_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch4_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH4_RX_END_INT_RAW_MASK);
    }

    /// Clear CH4_RX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch4_rx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH4_RX_END_INT_RAW_MASK);
    }

    /// **CH3_TX_LOOP_INT_RAW** `[15]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch3_tx_loop_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Set CH3_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_LOOP_INT_RAW_MASK);
    }

    /// Clear CH3_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_LOOP_INT_RAW_MASK);
    }

    /// **CH2_TX_LOOP_INT_RAW** `[14]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch2_tx_loop_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Set CH2_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_LOOP_INT_RAW_MASK);
    }

    /// Clear CH2_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_LOOP_INT_RAW_MASK);
    }

    /// **CH1_TX_LOOP_INT_RAW** `[13]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Set CH1_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_LOOP_INT_RAW_MASK);
    }

    /// Clear CH1_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_LOOP_INT_RAW_MASK);
    }

    /// **CH0_TX_LOOP_INT_RAW** `[12]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Set CH0_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_LOOP_INT_RAW_MASK);
    }

    /// Clear CH0_TX_LOOP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_loop_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_LOOP_INT_RAW_MASK);
    }

    /// **CH3_TX_THR_EVENT_INT_RAW** `[11]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH3_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH3_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH2_TX_THR_EVENT_INT_RAW** `[10]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH2_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH2_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH1_TX_THR_EVENT_INT_RAW** `[9]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH1_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH1_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// **CH0_TX_THR_EVENT_INT_RAW** `[8]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Set CH0_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// Clear CH0_TX_THR_EVENT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_thr_event_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_THR_EVENT_INT_RAW_MASK);
    }

    /// **TX_CH3_ERR_INT_RAW** `[7]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when error occurs.
    #[inline(always)]
    pub fn tx_ch3_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_ERR_INT_RAW_MASK) != 0
    }

    /// Set TX_CH3_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch3_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH3_ERR_INT_RAW_MASK);
    }

    /// Clear TX_CH3_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch3_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH3_ERR_INT_RAW_MASK);
    }

    /// **TX_CH2_ERR_INT_RAW** `[6]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when error occurs.
    #[inline(always)]
    pub fn tx_ch2_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH2_ERR_INT_RAW_MASK) != 0
    }

    /// Set TX_CH2_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch2_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH2_ERR_INT_RAW_MASK);
    }

    /// Clear TX_CH2_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch2_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH2_ERR_INT_RAW_MASK);
    }

    /// **TX_CH1_ERR_INT_RAW** `[5]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when error occurs.
    #[inline(always)]
    pub fn tx_ch1_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH1_ERR_INT_RAW_MASK) != 0
    }

    /// Set TX_CH1_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch1_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH1_ERR_INT_RAW_MASK);
    }

    /// Clear TX_CH1_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch1_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH1_ERR_INT_RAW_MASK);
    }

    /// **TX_CH0_ERR_INT_RAW** `[4]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when error occurs.
    #[inline(always)]
    pub fn tx_ch0_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH0_ERR_INT_RAW_MASK) != 0
    }

    /// Set TX_CH0_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch0_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH0_ERR_INT_RAW_MASK);
    }

    /// Clear TX_CH0_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch0_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH0_ERR_INT_RAW_MASK);
    }

    /// **CH3_TX_END_INT_RAW** `[3]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmission done.
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_END_INT_RAW_MASK) != 0
    }

    /// Set CH3_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_END_INT_RAW_MASK);
    }

    /// Clear CH3_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_END_INT_RAW_MASK);
    }

    /// **CH2_TX_END_INT_RAW** `[2]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmission done.
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_END_INT_RAW_MASK) != 0
    }

    /// Set CH2_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_END_INT_RAW_MASK);
    }

    /// Clear CH2_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_END_INT_RAW_MASK);
    }

    /// **CH1_TX_END_INT_RAW** `[1]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmission done.
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_END_INT_RAW_MASK) != 0
    }

    /// Set CH1_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_END_INT_RAW_MASK);
    }

    /// Clear CH1_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_END_INT_RAW_MASK);
    }

    /// **CH0_TX_END_INT_RAW** `[0]` (RW)
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmission done.
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_END_INT_RAW_MASK) != 0
    }

    /// Set CH0_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_END_INT_RAW_MASK);
    }

    /// Clear CH0_TX_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_END_INT_RAW_MASK);
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

    /// Read **RX_CH7_DMA_ACCESS_FAIL_INT_RAW** `[29:29]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails.
    #[inline(always)]
    pub const fn rx_ch7_dma_access_fail_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK) != 0
    }

    /// Read **TX_CH3_DMA_ACCESS_FAIL_INT_RAW** `[28:28]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails.
    #[inline(always)]
    pub const fn tx_ch3_dma_access_fail_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK) != 0
    }

    /// Read **CH7_RX_THR_EVENT_INT_RAW** `[27:27]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn ch7_rx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH7_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH6_RX_THR_EVENT_INT_RAW** `[26:26]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn ch6_rx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH6_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH5_RX_THR_EVENT_INT_RAW** `[25:25]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn ch5_rx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH5_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH4_RX_THR_EVENT_INT_RAW** `[24:24]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn ch4_rx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH4_RX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **RX_CH7_ERR_INT_RAW** `[23:23]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when error occurs.
    #[inline(always)]
    pub const fn rx_ch7_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_CH7_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RX_CH6_ERR_INT_RAW** `[22:22]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when error occurs.
    #[inline(always)]
    pub const fn rx_ch6_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_CH6_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RX_CH5_ERR_INT_RAW** `[21:21]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when error occurs.
    #[inline(always)]
    pub const fn rx_ch5_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_CH5_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RX_CH4_ERR_INT_RAW** `[20:20]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when error occurs.
    #[inline(always)]
    pub const fn rx_ch4_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_CH4_ERR_INT_RAW_MASK) != 0
    }

    /// Read **CH7_RX_END_INT_RAW** `[19:19]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when reception done.
    #[inline(always)]
    pub const fn ch7_rx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH7_RX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH6_RX_END_INT_RAW** `[18:18]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when reception done.
    #[inline(always)]
    pub const fn ch6_rx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH6_RX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH5_RX_END_INT_RAW** `[17:17]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when reception done.
    #[inline(always)]
    pub const fn ch5_rx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH5_RX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH4_RX_END_INT_RAW** `[16:16]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when reception done.
    #[inline(always)]
    pub const fn ch4_rx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH4_RX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH3_TX_LOOP_INT_RAW** `[15:15]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn ch3_tx_loop_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH3_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Read **CH2_TX_LOOP_INT_RAW** `[14:14]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn ch2_tx_loop_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH2_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Read **CH1_TX_LOOP_INT_RAW** `[13:13]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn ch1_tx_loop_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH1_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Read **CH0_TX_LOOP_INT_RAW** `[12:12]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn ch0_tx_loop_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH0_TX_LOOP_INT_RAW_MASK) != 0
    }

    /// Read **CH3_TX_THR_EVENT_INT_RAW** `[11:11]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn ch3_tx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH3_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH2_TX_THR_EVENT_INT_RAW** `[10:10]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn ch2_tx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH2_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH1_TX_THR_EVENT_INT_RAW** `[9:9]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn ch1_tx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH1_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **CH0_TX_THR_EVENT_INT_RAW** `[8:8]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn ch0_tx_thr_event_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH0_TX_THR_EVENT_INT_RAW_MASK) != 0
    }

    /// Read **TX_CH3_ERR_INT_RAW** `[7:7]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when error occurs.
    #[inline(always)]
    pub const fn tx_ch3_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_CH3_ERR_INT_RAW_MASK) != 0
    }

    /// Read **TX_CH2_ERR_INT_RAW** `[6:6]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when error occurs.
    #[inline(always)]
    pub const fn tx_ch2_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_CH2_ERR_INT_RAW_MASK) != 0
    }

    /// Read **TX_CH1_ERR_INT_RAW** `[5:5]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when error occurs.
    #[inline(always)]
    pub const fn tx_ch1_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_CH1_ERR_INT_RAW_MASK) != 0
    }

    /// Read **TX_CH0_ERR_INT_RAW** `[4:4]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when error occurs.
    #[inline(always)]
    pub const fn tx_ch0_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_CH0_ERR_INT_RAW_MASK) != 0
    }

    /// Read **CH3_TX_END_INT_RAW** `[3:3]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmission done.
    #[inline(always)]
    pub const fn ch3_tx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH3_TX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH2_TX_END_INT_RAW** `[2:2]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmission done.
    #[inline(always)]
    pub const fn ch2_tx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH2_TX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH1_TX_END_INT_RAW** `[1:1]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmission done.
    #[inline(always)]
    pub const fn ch1_tx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH1_TX_END_INT_RAW_MASK) != 0
    }

    /// Read **CH0_TX_END_INT_RAW** `[0:0]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmission done.
    #[inline(always)]
    pub const fn ch0_tx_end_int_raw(&self) -> bool {
        (self.bits & IntRaw::CH0_TX_END_INT_RAW_MASK) != 0
    }

    /// Set **RX_CH7_DMA_ACCESS_FAIL_INT_RAW** `[29:29]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails.
    #[inline(always)]
    pub const fn with_rx_ch7_dma_access_fail_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_CH7_DMA_ACCESS_FAIL_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_CH3_DMA_ACCESS_FAIL_INT_RAW** `[28:28]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails.
    #[inline(always)]
    pub const fn with_tx_ch3_dma_access_fail_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_CH3_DMA_ACCESS_FAIL_INT_RAW_MASK
            },
        }
    }

    /// Set **CH7_RX_THR_EVENT_INT_RAW** `[27:27]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn with_ch7_rx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH7_RX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH7_RX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH6_RX_THR_EVENT_INT_RAW** `[26:26]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn with_ch6_rx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH6_RX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH6_RX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH5_RX_THR_EVENT_INT_RAW** `[25:25]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn with_ch5_rx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH5_RX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH5_RX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH4_RX_THR_EVENT_INT_RAW** `[24:24]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value.
    #[inline(always)]
    pub const fn with_ch4_rx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH4_RX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH4_RX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_CH7_ERR_INT_RAW** `[23:23]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_rx_ch7_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_CH7_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_CH7_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_CH6_ERR_INT_RAW** `[22:22]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_rx_ch6_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_CH6_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_CH6_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_CH5_ERR_INT_RAW** `[21:21]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_rx_ch5_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_CH5_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_CH5_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_CH4_ERR_INT_RAW** `[20:20]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_rx_ch4_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_CH4_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_CH4_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **CH7_RX_END_INT_RAW** `[19:19]`
    ///
    /// The interrupt raw bit for CHANNEL7. Triggered when reception done.
    #[inline(always)]
    pub const fn with_ch7_rx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH7_RX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH7_RX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH6_RX_END_INT_RAW** `[18:18]`
    ///
    /// The interrupt raw bit for CHANNEL6. Triggered when reception done.
    #[inline(always)]
    pub const fn with_ch6_rx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH6_RX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH6_RX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH5_RX_END_INT_RAW** `[17:17]`
    ///
    /// The interrupt raw bit for CHANNEL5. Triggered when reception done.
    #[inline(always)]
    pub const fn with_ch5_rx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH5_RX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH5_RX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH4_RX_END_INT_RAW** `[16:16]`
    ///
    /// The interrupt raw bit for CHANNEL4. Triggered when reception done.
    #[inline(always)]
    pub const fn with_ch4_rx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH4_RX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH4_RX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH3_TX_LOOP_INT_RAW** `[15:15]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn with_ch3_tx_loop_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH3_TX_LOOP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH3_TX_LOOP_INT_RAW_MASK
            },
        }
    }

    /// Set **CH2_TX_LOOP_INT_RAW** `[14:14]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn with_ch2_tx_loop_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH2_TX_LOOP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH2_TX_LOOP_INT_RAW_MASK
            },
        }
    }

    /// Set **CH1_TX_LOOP_INT_RAW** `[13:13]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn with_ch1_tx_loop_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH1_TX_LOOP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH1_TX_LOOP_INT_RAW_MASK
            },
        }
    }

    /// Set **CH0_TX_LOOP_INT_RAW** `[12:12]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub const fn with_ch0_tx_loop_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH0_TX_LOOP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH0_TX_LOOP_INT_RAW_MASK
            },
        }
    }

    /// Set **CH3_TX_THR_EVENT_INT_RAW** `[11:11]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn with_ch3_tx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH3_TX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH3_TX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH2_TX_THR_EVENT_INT_RAW** `[10:10]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn with_ch2_tx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH2_TX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH2_TX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH1_TX_THR_EVENT_INT_RAW** `[9:9]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn with_ch1_tx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH1_TX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH1_TX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **CH0_TX_THR_EVENT_INT_RAW** `[8:8]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub const fn with_ch0_tx_thr_event_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH0_TX_THR_EVENT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH0_TX_THR_EVENT_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_CH3_ERR_INT_RAW** `[7:7]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_tx_ch3_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_CH3_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_CH3_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_CH2_ERR_INT_RAW** `[6:6]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_tx_ch2_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_CH2_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_CH2_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_CH1_ERR_INT_RAW** `[5:5]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_tx_ch1_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_CH1_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_CH1_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_CH0_ERR_INT_RAW** `[4:4]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when error occurs.
    #[inline(always)]
    pub const fn with_tx_ch0_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_CH0_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_CH0_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **CH3_TX_END_INT_RAW** `[3:3]`
    ///
    /// The interrupt raw bit for CHANNEL3. Triggered when transmission done.
    #[inline(always)]
    pub const fn with_ch3_tx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH3_TX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH3_TX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH2_TX_END_INT_RAW** `[2:2]`
    ///
    /// The interrupt raw bit for CHANNEL2. Triggered when transmission done.
    #[inline(always)]
    pub const fn with_ch2_tx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH2_TX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH2_TX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH1_TX_END_INT_RAW** `[1:1]`
    ///
    /// The interrupt raw bit for CHANNEL1. Triggered when transmission done.
    #[inline(always)]
    pub const fn with_ch1_tx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH1_TX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH1_TX_END_INT_RAW_MASK
            },
        }
    }

    /// Set **CH0_TX_END_INT_RAW** `[0:0]`
    ///
    /// The interrupt raw bit for CHANNEL0. Triggered when transmission done.
    #[inline(always)]
    pub const fn with_ch0_tx_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CH0_TX_END_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CH0_TX_END_INT_RAW_MASK
            },
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntRawVal) -> IntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST Register
///
/// Masked interrupt status
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RX_CH7_DMA_ACCESS_FAIL_INT_ST** `[29]` (RO)
///
///   The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT.
///
/// - **TX_CH3_DMA_ACCESS_FAIL_INT_ST** `[28]` (RO)
///
///   The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT.
///
/// - **CH7_RX_THR_EVENT_INT_ST** `[27]` (RO)
///
///   The masked interrupt status bit for CH7_RX_THR_EVENT_INT.
///
/// - **CH6_RX_THR_EVENT_INT_ST** `[26]` (RO)
///
///   The masked interrupt status bit for CH6_RX_THR_EVENT_INT.
///
/// - **CH5_RX_THR_EVENT_INT_ST** `[25]` (RO)
///
///   The masked interrupt status bit for CH5_RX_THR_EVENT_INT.
///
/// - **CH4_RX_THR_EVENT_INT_ST** `[24]` (RO)
///
///   The masked interrupt status bit for CH4_RX_THR_EVENT_INT.
///
/// - **RX_CH7_ERR_INT_ST** `[23]` (RO)
///
///   The masked interrupt status bit for CH7_ERR_INT.
///
/// - **RX_CH6_ERR_INT_ST** `[22]` (RO)
///
///   The masked interrupt status bit for CH6_ERR_INT.
///
/// - **RX_CH5_ERR_INT_ST** `[21]` (RO)
///
///   The masked interrupt status bit for CH5_ERR_INT.
///
/// - **RX_CH4_ERR_INT_ST** `[20]` (RO)
///
///   The masked interrupt status bit for CH4_ERR_INT.
///
/// - **CH7_RX_END_INT_ST** `[19]` (RO)
///
///   The masked interrupt status bit for CH7_RX_END_INT.
///
/// - **CH6_RX_END_INT_ST** `[18]` (RO)
///
///   The masked interrupt status bit for CH6_RX_END_INT.
///
/// - **CH5_RX_END_INT_ST** `[17]` (RO)
///
///   The masked interrupt status bit for CH5_RX_END_INT.
///
/// - **CH4_RX_END_INT_ST** `[16]` (RO)
///
///   The masked interrupt status bit for CH4_RX_END_INT.
///
/// - **CH3_TX_LOOP_INT_ST** `[15]` (RO)
///
///   The masked interrupt status bit for CH3_TX_LOOP_INT.
///
/// - **CH2_TX_LOOP_INT_ST** `[14]` (RO)
///
///   The masked interrupt status bit for CH2_TX_LOOP_INT.
///
/// - **CH1_TX_LOOP_INT_ST** `[13]` (RO)
///
///   The masked interrupt status bit for CH1_TX_LOOP_INT.
///
/// - **CH0_TX_LOOP_INT_ST** `[12]` (RO)
///
///   The masked interrupt status bit for CH0_TX_LOOP_INT.
///
/// - **CH3_TX_THR_EVENT_INT_ST** `[11]` (RO)
///
///   The masked interrupt status bit for CH3_TX_THR_EVENT_INT.
///
/// - **CH2_TX_THR_EVENT_INT_ST** `[10]` (RO)
///
///   The masked interrupt status bit for CH2_TX_THR_EVENT_INT.
///
/// - **CH1_TX_THR_EVENT_INT_ST** `[9]` (RO)
///
///   The masked interrupt status bit for CH1_TX_THR_EVENT_INT.
///
/// - **CH0_TX_THR_EVENT_INT_ST** `[8]` (RO)
///
///   The masked interrupt status bit for CH0_TX_THR_EVENT_INT.
///
/// - **TX_CH3_ERR_INT_ST** `[7]` (RO)
///
///   The masked interrupt status bit for CH3_ERR_INT.
///
/// - **TX_CH2_ERR_INT_ST** `[6]` (RO)
///
///   The masked interrupt status bit for CH2_ERR_INT.
///
/// - **TX_CH1_ERR_INT_ST** `[5]` (RO)
///
///   The masked interrupt status bit for CH1_ERR_INT.
///
/// - **TX_CH0_ERR_INT_ST** `[4]` (RO)
///
///   The masked interrupt status bit for CH0_ERR_INT.
///
/// - **CH3_TX_END_INT_ST** `[3]` (RO)
///
///   The masked interrupt status bit for CH3_TX_END_INT.
///
/// - **CH2_TX_END_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for CH2_TX_END_INT.
///
/// - **CH1_TX_END_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for CH1_TX_END_INT.
///
/// - **CH0_TX_END_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for CH0_TX_END_INT.
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

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_ST**
    ///
    /// The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ST_OFFSET: u32 = 29;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ST_MASK: u32 = 0x20000000;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ST: u32 = 1 << 29;

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_ST**
    ///
    /// The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ST_OFFSET: u32 = 28;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ST_MASK: u32 = 0x10000000;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ST: u32 = 1 << 28;

    /// **CH7_RX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH7_RX_THR_EVENT_INT.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CH7_RX_THR_EVENT_INT_ST_OFFSET: u32 = 27;
    pub const CH7_RX_THR_EVENT_INT_ST_MASK: u32 = 0x08000000;
    pub const CH7_RX_THR_EVENT_INT_ST: u32 = 1 << 27;

    /// **CH6_RX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH6_RX_THR_EVENT_INT.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CH6_RX_THR_EVENT_INT_ST_OFFSET: u32 = 26;
    pub const CH6_RX_THR_EVENT_INT_ST_MASK: u32 = 0x04000000;
    pub const CH6_RX_THR_EVENT_INT_ST: u32 = 1 << 26;

    /// **CH5_RX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH5_RX_THR_EVENT_INT.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CH5_RX_THR_EVENT_INT_ST_OFFSET: u32 = 25;
    pub const CH5_RX_THR_EVENT_INT_ST_MASK: u32 = 0x02000000;
    pub const CH5_RX_THR_EVENT_INT_ST: u32 = 1 << 25;

    /// **CH4_RX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH4_RX_THR_EVENT_INT.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CH4_RX_THR_EVENT_INT_ST_OFFSET: u32 = 24;
    pub const CH4_RX_THR_EVENT_INT_ST_MASK: u32 = 0x01000000;
    pub const CH4_RX_THR_EVENT_INT_ST: u32 = 1 << 24;

    /// **RX_CH7_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH7_ERR_INT.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RX_CH7_ERR_INT_ST_OFFSET: u32 = 23;
    pub const RX_CH7_ERR_INT_ST_MASK: u32 = 0x00800000;
    pub const RX_CH7_ERR_INT_ST: u32 = 1 << 23;

    /// **RX_CH6_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH6_ERR_INT.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const RX_CH6_ERR_INT_ST_OFFSET: u32 = 22;
    pub const RX_CH6_ERR_INT_ST_MASK: u32 = 0x00400000;
    pub const RX_CH6_ERR_INT_ST: u32 = 1 << 22;

    /// **RX_CH5_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH5_ERR_INT.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const RX_CH5_ERR_INT_ST_OFFSET: u32 = 21;
    pub const RX_CH5_ERR_INT_ST_MASK: u32 = 0x00200000;
    pub const RX_CH5_ERR_INT_ST: u32 = 1 << 21;

    /// **RX_CH4_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH4_ERR_INT.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_CH4_ERR_INT_ST_OFFSET: u32 = 20;
    pub const RX_CH4_ERR_INT_ST_MASK: u32 = 0x00100000;
    pub const RX_CH4_ERR_INT_ST: u32 = 1 << 20;

    /// **CH7_RX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH7_RX_END_INT.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CH7_RX_END_INT_ST_OFFSET: u32 = 19;
    pub const CH7_RX_END_INT_ST_MASK: u32 = 0x00080000;
    pub const CH7_RX_END_INT_ST: u32 = 1 << 19;

    /// **CH6_RX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH6_RX_END_INT.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CH6_RX_END_INT_ST_OFFSET: u32 = 18;
    pub const CH6_RX_END_INT_ST_MASK: u32 = 0x00040000;
    pub const CH6_RX_END_INT_ST: u32 = 1 << 18;

    /// **CH5_RX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH5_RX_END_INT.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CH5_RX_END_INT_ST_OFFSET: u32 = 17;
    pub const CH5_RX_END_INT_ST_MASK: u32 = 0x00020000;
    pub const CH5_RX_END_INT_ST: u32 = 1 << 17;

    /// **CH4_RX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH4_RX_END_INT.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CH4_RX_END_INT_ST_OFFSET: u32 = 16;
    pub const CH4_RX_END_INT_ST_MASK: u32 = 0x00010000;
    pub const CH4_RX_END_INT_ST: u32 = 1 << 16;

    /// **CH3_TX_LOOP_INT_ST**
    ///
    /// The masked interrupt status bit for CH3_TX_LOOP_INT.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CH3_TX_LOOP_INT_ST_OFFSET: u32 = 15;
    pub const CH3_TX_LOOP_INT_ST_MASK: u32 = 0x00008000;
    pub const CH3_TX_LOOP_INT_ST: u32 = 1 << 15;

    /// **CH2_TX_LOOP_INT_ST**
    ///
    /// The masked interrupt status bit for CH2_TX_LOOP_INT.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CH2_TX_LOOP_INT_ST_OFFSET: u32 = 14;
    pub const CH2_TX_LOOP_INT_ST_MASK: u32 = 0x00004000;
    pub const CH2_TX_LOOP_INT_ST: u32 = 1 << 14;

    /// **CH1_TX_LOOP_INT_ST**
    ///
    /// The masked interrupt status bit for CH1_TX_LOOP_INT.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CH1_TX_LOOP_INT_ST_OFFSET: u32 = 13;
    pub const CH1_TX_LOOP_INT_ST_MASK: u32 = 0x00002000;
    pub const CH1_TX_LOOP_INT_ST: u32 = 1 << 13;

    /// **CH0_TX_LOOP_INT_ST**
    ///
    /// The masked interrupt status bit for CH0_TX_LOOP_INT.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CH0_TX_LOOP_INT_ST_OFFSET: u32 = 12;
    pub const CH0_TX_LOOP_INT_ST_MASK: u32 = 0x00001000;
    pub const CH0_TX_LOOP_INT_ST: u32 = 1 << 12;

    /// **CH3_TX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH3_TX_THR_EVENT_INT.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH3_TX_THR_EVENT_INT_ST_OFFSET: u32 = 11;
    pub const CH3_TX_THR_EVENT_INT_ST_MASK: u32 = 0x00000800;
    pub const CH3_TX_THR_EVENT_INT_ST: u32 = 1 << 11;

    /// **CH2_TX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH2_TX_THR_EVENT_INT.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH2_TX_THR_EVENT_INT_ST_OFFSET: u32 = 10;
    pub const CH2_TX_THR_EVENT_INT_ST_MASK: u32 = 0x00000400;
    pub const CH2_TX_THR_EVENT_INT_ST: u32 = 1 << 10;

    /// **CH1_TX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH1_TX_THR_EVENT_INT.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH1_TX_THR_EVENT_INT_ST_OFFSET: u32 = 9;
    pub const CH1_TX_THR_EVENT_INT_ST_MASK: u32 = 0x00000200;
    pub const CH1_TX_THR_EVENT_INT_ST: u32 = 1 << 9;

    /// **CH0_TX_THR_EVENT_INT_ST**
    ///
    /// The masked interrupt status bit for CH0_TX_THR_EVENT_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH0_TX_THR_EVENT_INT_ST_OFFSET: u32 = 8;
    pub const CH0_TX_THR_EVENT_INT_ST_MASK: u32 = 0x00000100;
    pub const CH0_TX_THR_EVENT_INT_ST: u32 = 1 << 8;

    /// **TX_CH3_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH3_ERR_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_CH3_ERR_INT_ST_OFFSET: u32 = 7;
    pub const TX_CH3_ERR_INT_ST_MASK: u32 = 0x00000080;
    pub const TX_CH3_ERR_INT_ST: u32 = 1 << 7;

    /// **TX_CH2_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH2_ERR_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_CH2_ERR_INT_ST_OFFSET: u32 = 6;
    pub const TX_CH2_ERR_INT_ST_MASK: u32 = 0x00000040;
    pub const TX_CH2_ERR_INT_ST: u32 = 1 << 6;

    /// **TX_CH1_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH1_ERR_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_CH1_ERR_INT_ST_OFFSET: u32 = 5;
    pub const TX_CH1_ERR_INT_ST_MASK: u32 = 0x00000020;
    pub const TX_CH1_ERR_INT_ST: u32 = 1 << 5;

    /// **TX_CH0_ERR_INT_ST**
    ///
    /// The masked interrupt status bit for CH0_ERR_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_CH0_ERR_INT_ST_OFFSET: u32 = 4;
    pub const TX_CH0_ERR_INT_ST_MASK: u32 = 0x00000010;
    pub const TX_CH0_ERR_INT_ST: u32 = 1 << 4;

    /// **CH3_TX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH3_TX_END_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_TX_END_INT_ST_OFFSET: u32 = 3;
    pub const CH3_TX_END_INT_ST_MASK: u32 = 0x00000008;
    pub const CH3_TX_END_INT_ST: u32 = 1 << 3;

    /// **CH2_TX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH2_TX_END_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_TX_END_INT_ST_OFFSET: u32 = 2;
    pub const CH2_TX_END_INT_ST_MASK: u32 = 0x00000004;
    pub const CH2_TX_END_INT_ST: u32 = 1 << 2;

    /// **CH1_TX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH1_TX_END_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_TX_END_INT_ST_OFFSET: u32 = 1;
    pub const CH1_TX_END_INT_ST_MASK: u32 = 0x00000002;
    pub const CH1_TX_END_INT_ST: u32 = 1 << 1;

    /// **CH0_TX_END_INT_ST**
    ///
    /// The masked interrupt status bit for CH0_TX_END_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_TX_END_INT_ST_OFFSET: u32 = 0;
    pub const CH0_TX_END_INT_ST_MASK: u32 = 0x00000001;
    pub const CH0_TX_END_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_ST** `[29]` (RO)
    ///
    /// The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH7_DMA_ACCESS_FAIL_INT_ST_MASK) != 0
    }

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_ST** `[28]` (RO)
    ///
    /// The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_DMA_ACCESS_FAIL_INT_ST_MASK) != 0
    }

    /// **CH7_RX_THR_EVENT_INT_ST** `[27]` (RO)
    ///
    /// The masked interrupt status bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH6_RX_THR_EVENT_INT_ST** `[26]` (RO)
    ///
    /// The masked interrupt status bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH5_RX_THR_EVENT_INT_ST** `[25]` (RO)
    ///
    /// The masked interrupt status bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH4_RX_THR_EVENT_INT_ST** `[24]` (RO)
    ///
    /// The masked interrupt status bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **RX_CH7_ERR_INT_ST** `[23]` (RO)
    ///
    /// The masked interrupt status bit for CH7_ERR_INT.
    #[inline(always)]
    pub fn rx_ch7_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH7_ERR_INT_ST_MASK) != 0
    }

    /// **RX_CH6_ERR_INT_ST** `[22]` (RO)
    ///
    /// The masked interrupt status bit for CH6_ERR_INT.
    #[inline(always)]
    pub fn rx_ch6_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH6_ERR_INT_ST_MASK) != 0
    }

    /// **RX_CH5_ERR_INT_ST** `[21]` (RO)
    ///
    /// The masked interrupt status bit for CH5_ERR_INT.
    #[inline(always)]
    pub fn rx_ch5_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH5_ERR_INT_ST_MASK) != 0
    }

    /// **RX_CH4_ERR_INT_ST** `[20]` (RO)
    ///
    /// The masked interrupt status bit for CH4_ERR_INT.
    #[inline(always)]
    pub fn rx_ch4_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH4_ERR_INT_ST_MASK) != 0
    }

    /// **CH7_RX_END_INT_ST** `[19]` (RO)
    ///
    /// The masked interrupt status bit for CH7_RX_END_INT.
    #[inline(always)]
    pub fn ch7_rx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_END_INT_ST_MASK) != 0
    }

    /// **CH6_RX_END_INT_ST** `[18]` (RO)
    ///
    /// The masked interrupt status bit for CH6_RX_END_INT.
    #[inline(always)]
    pub fn ch6_rx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_END_INT_ST_MASK) != 0
    }

    /// **CH5_RX_END_INT_ST** `[17]` (RO)
    ///
    /// The masked interrupt status bit for CH5_RX_END_INT.
    #[inline(always)]
    pub fn ch5_rx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_END_INT_ST_MASK) != 0
    }

    /// **CH4_RX_END_INT_ST** `[16]` (RO)
    ///
    /// The masked interrupt status bit for CH4_RX_END_INT.
    #[inline(always)]
    pub fn ch4_rx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_END_INT_ST_MASK) != 0
    }

    /// **CH3_TX_LOOP_INT_ST** `[15]` (RO)
    ///
    /// The masked interrupt status bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch3_tx_loop_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_LOOP_INT_ST_MASK) != 0
    }

    /// **CH2_TX_LOOP_INT_ST** `[14]` (RO)
    ///
    /// The masked interrupt status bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch2_tx_loop_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_LOOP_INT_ST_MASK) != 0
    }

    /// **CH1_TX_LOOP_INT_ST** `[13]` (RO)
    ///
    /// The masked interrupt status bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch1_tx_loop_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_LOOP_INT_ST_MASK) != 0
    }

    /// **CH0_TX_LOOP_INT_ST** `[12]` (RO)
    ///
    /// The masked interrupt status bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch0_tx_loop_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_LOOP_INT_ST_MASK) != 0
    }

    /// **CH3_TX_THR_EVENT_INT_ST** `[11]` (RO)
    ///
    /// The masked interrupt status bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH2_TX_THR_EVENT_INT_ST** `[10]` (RO)
    ///
    /// The masked interrupt status bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH1_TX_THR_EVENT_INT_ST** `[9]` (RO)
    ///
    /// The masked interrupt status bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **CH0_TX_THR_EVENT_INT_ST** `[8]` (RO)
    ///
    /// The masked interrupt status bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// **TX_CH3_ERR_INT_ST** `[7]` (RO)
    ///
    /// The masked interrupt status bit for CH3_ERR_INT.
    #[inline(always)]
    pub fn tx_ch3_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_ERR_INT_ST_MASK) != 0
    }

    /// **TX_CH2_ERR_INT_ST** `[6]` (RO)
    ///
    /// The masked interrupt status bit for CH2_ERR_INT.
    #[inline(always)]
    pub fn tx_ch2_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH2_ERR_INT_ST_MASK) != 0
    }

    /// **TX_CH1_ERR_INT_ST** `[5]` (RO)
    ///
    /// The masked interrupt status bit for CH1_ERR_INT.
    #[inline(always)]
    pub fn tx_ch1_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH1_ERR_INT_ST_MASK) != 0
    }

    /// **TX_CH0_ERR_INT_ST** `[4]` (RO)
    ///
    /// The masked interrupt status bit for CH0_ERR_INT.
    #[inline(always)]
    pub fn tx_ch0_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH0_ERR_INT_ST_MASK) != 0
    }

    /// **CH3_TX_END_INT_ST** `[3]` (RO)
    ///
    /// The masked interrupt status bit for CH3_TX_END_INT.
    #[inline(always)]
    pub fn ch3_tx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_END_INT_ST_MASK) != 0
    }

    /// **CH2_TX_END_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for CH2_TX_END_INT.
    #[inline(always)]
    pub fn ch2_tx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_END_INT_ST_MASK) != 0
    }

    /// **CH1_TX_END_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for CH1_TX_END_INT.
    #[inline(always)]
    pub fn ch1_tx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_END_INT_ST_MASK) != 0
    }

    /// **CH0_TX_END_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for CH0_TX_END_INT.
    #[inline(always)]
    pub fn ch0_tx_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_END_INT_ST_MASK) != 0
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

    /// Read **RX_CH7_DMA_ACCESS_FAIL_INT_ST** `[29:29]`
    ///
    /// The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn rx_ch7_dma_access_fail_int_st(&self) -> bool {
        (self.bits & IntSt::RX_CH7_DMA_ACCESS_FAIL_INT_ST_MASK) != 0
    }

    /// Read **TX_CH3_DMA_ACCESS_FAIL_INT_ST** `[28:28]`
    ///
    /// The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn tx_ch3_dma_access_fail_int_st(&self) -> bool {
        (self.bits & IntSt::TX_CH3_DMA_ACCESS_FAIL_INT_ST_MASK) != 0
    }

    /// Read **CH7_RX_THR_EVENT_INT_ST** `[27:27]`
    ///
    /// The masked interrupt status bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch7_rx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH7_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH6_RX_THR_EVENT_INT_ST** `[26:26]`
    ///
    /// The masked interrupt status bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch6_rx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH6_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH5_RX_THR_EVENT_INT_ST** `[25:25]`
    ///
    /// The masked interrupt status bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch5_rx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH5_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH4_RX_THR_EVENT_INT_ST** `[24:24]`
    ///
    /// The masked interrupt status bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch4_rx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH4_RX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **RX_CH7_ERR_INT_ST** `[23:23]`
    ///
    /// The masked interrupt status bit for CH7_ERR_INT.
    #[inline(always)]
    pub const fn rx_ch7_err_int_st(&self) -> bool {
        (self.bits & IntSt::RX_CH7_ERR_INT_ST_MASK) != 0
    }

    /// Read **RX_CH6_ERR_INT_ST** `[22:22]`
    ///
    /// The masked interrupt status bit for CH6_ERR_INT.
    #[inline(always)]
    pub const fn rx_ch6_err_int_st(&self) -> bool {
        (self.bits & IntSt::RX_CH6_ERR_INT_ST_MASK) != 0
    }

    /// Read **RX_CH5_ERR_INT_ST** `[21:21]`
    ///
    /// The masked interrupt status bit for CH5_ERR_INT.
    #[inline(always)]
    pub const fn rx_ch5_err_int_st(&self) -> bool {
        (self.bits & IntSt::RX_CH5_ERR_INT_ST_MASK) != 0
    }

    /// Read **RX_CH4_ERR_INT_ST** `[20:20]`
    ///
    /// The masked interrupt status bit for CH4_ERR_INT.
    #[inline(always)]
    pub const fn rx_ch4_err_int_st(&self) -> bool {
        (self.bits & IntSt::RX_CH4_ERR_INT_ST_MASK) != 0
    }

    /// Read **CH7_RX_END_INT_ST** `[19:19]`
    ///
    /// The masked interrupt status bit for CH7_RX_END_INT.
    #[inline(always)]
    pub const fn ch7_rx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH7_RX_END_INT_ST_MASK) != 0
    }

    /// Read **CH6_RX_END_INT_ST** `[18:18]`
    ///
    /// The masked interrupt status bit for CH6_RX_END_INT.
    #[inline(always)]
    pub const fn ch6_rx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH6_RX_END_INT_ST_MASK) != 0
    }

    /// Read **CH5_RX_END_INT_ST** `[17:17]`
    ///
    /// The masked interrupt status bit for CH5_RX_END_INT.
    #[inline(always)]
    pub const fn ch5_rx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH5_RX_END_INT_ST_MASK) != 0
    }

    /// Read **CH4_RX_END_INT_ST** `[16:16]`
    ///
    /// The masked interrupt status bit for CH4_RX_END_INT.
    #[inline(always)]
    pub const fn ch4_rx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH4_RX_END_INT_ST_MASK) != 0
    }

    /// Read **CH3_TX_LOOP_INT_ST** `[15:15]`
    ///
    /// The masked interrupt status bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch3_tx_loop_int_st(&self) -> bool {
        (self.bits & IntSt::CH3_TX_LOOP_INT_ST_MASK) != 0
    }

    /// Read **CH2_TX_LOOP_INT_ST** `[14:14]`
    ///
    /// The masked interrupt status bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch2_tx_loop_int_st(&self) -> bool {
        (self.bits & IntSt::CH2_TX_LOOP_INT_ST_MASK) != 0
    }

    /// Read **CH1_TX_LOOP_INT_ST** `[13:13]`
    ///
    /// The masked interrupt status bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch1_tx_loop_int_st(&self) -> bool {
        (self.bits & IntSt::CH1_TX_LOOP_INT_ST_MASK) != 0
    }

    /// Read **CH0_TX_LOOP_INT_ST** `[12:12]`
    ///
    /// The masked interrupt status bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch0_tx_loop_int_st(&self) -> bool {
        (self.bits & IntSt::CH0_TX_LOOP_INT_ST_MASK) != 0
    }

    /// Read **CH3_TX_THR_EVENT_INT_ST** `[11:11]`
    ///
    /// The masked interrupt status bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch3_tx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH3_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH2_TX_THR_EVENT_INT_ST** `[10:10]`
    ///
    /// The masked interrupt status bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch2_tx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH2_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH1_TX_THR_EVENT_INT_ST** `[9:9]`
    ///
    /// The masked interrupt status bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch1_tx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH1_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **CH0_TX_THR_EVENT_INT_ST** `[8:8]`
    ///
    /// The masked interrupt status bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch0_tx_thr_event_int_st(&self) -> bool {
        (self.bits & IntSt::CH0_TX_THR_EVENT_INT_ST_MASK) != 0
    }

    /// Read **TX_CH3_ERR_INT_ST** `[7:7]`
    ///
    /// The masked interrupt status bit for CH3_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch3_err_int_st(&self) -> bool {
        (self.bits & IntSt::TX_CH3_ERR_INT_ST_MASK) != 0
    }

    /// Read **TX_CH2_ERR_INT_ST** `[6:6]`
    ///
    /// The masked interrupt status bit for CH2_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch2_err_int_st(&self) -> bool {
        (self.bits & IntSt::TX_CH2_ERR_INT_ST_MASK) != 0
    }

    /// Read **TX_CH1_ERR_INT_ST** `[5:5]`
    ///
    /// The masked interrupt status bit for CH1_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch1_err_int_st(&self) -> bool {
        (self.bits & IntSt::TX_CH1_ERR_INT_ST_MASK) != 0
    }

    /// Read **TX_CH0_ERR_INT_ST** `[4:4]`
    ///
    /// The masked interrupt status bit for CH0_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch0_err_int_st(&self) -> bool {
        (self.bits & IntSt::TX_CH0_ERR_INT_ST_MASK) != 0
    }

    /// Read **CH3_TX_END_INT_ST** `[3:3]`
    ///
    /// The masked interrupt status bit for CH3_TX_END_INT.
    #[inline(always)]
    pub const fn ch3_tx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH3_TX_END_INT_ST_MASK) != 0
    }

    /// Read **CH2_TX_END_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for CH2_TX_END_INT.
    #[inline(always)]
    pub const fn ch2_tx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH2_TX_END_INT_ST_MASK) != 0
    }

    /// Read **CH1_TX_END_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for CH1_TX_END_INT.
    #[inline(always)]
    pub const fn ch1_tx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH1_TX_END_INT_ST_MASK) != 0
    }

    /// Read **CH0_TX_END_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for CH0_TX_END_INT.
    #[inline(always)]
    pub const fn ch0_tx_end_int_st(&self) -> bool {
        (self.bits & IntSt::CH0_TX_END_INT_ST_MASK) != 0
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
/// Interrupt enable bits
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **RX_CH7_DMA_ACCESS_FAIL_INT_ENA** `[29]` (RW)
///
///   The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
///
/// - **TX_CH3_DMA_ACCESS_FAIL_INT_ENA** `[28]` (RW)
///
///   The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
///
/// - **CH7_RX_THR_EVENT_INT_ENA** `[27]` (RW)
///
///   The interrupt enable bit for CH7_RX_THR_EVENT_INT.
///
/// - **CH6_RX_THR_EVENT_INT_ENA** `[26]` (RW)
///
///   The interrupt enable bit for CH6_RX_THR_EVENT_INT.
///
/// - **CH5_RX_THR_EVENT_INT_ENA** `[25]` (RW)
///
///   The interrupt enable bit for CH5_RX_THR_EVENT_INT.
///
/// - **CH4_RX_THR_EVENT_INT_ENA** `[24]` (RW)
///
///   The interrupt enable bit for CH4_RX_THR_EVENT_INT.
///
/// - **CH7_ERR_INT_ENA** `[23]` (RW)
///
///   The interrupt enable bit for CH7_ERR_INT.
///
/// - **CH6_ERR_INT_ENA** `[22]` (RW)
///
///   The interrupt enable bit for CH6_ERR_INT.
///
/// - **CH5_ERR_INT_ENA** `[21]` (RW)
///
///   The interrupt enable bit for CH5_ERR_INT.
///
/// - **CH4_ERR_INT_ENA** `[20]` (RW)
///
///   The interrupt enable bit for CH4_ERR_INT.
///
/// - **CH7_RX_END_INT_ENA** `[19]` (RW)
///
///   The interrupt enable bit for CH7_RX_END_INT.
///
/// - **CH6_RX_END_INT_ENA** `[18]` (RW)
///
///   The interrupt enable bit for CH6_RX_END_INT.
///
/// - **CH5_RX_END_INT_ENA** `[17]` (RW)
///
///   The interrupt enable bit for CH5_RX_END_INT.
///
/// - **CH4_RX_END_INT_ENA** `[16]` (RW)
///
///   The interrupt enable bit for CH4_RX_END_INT.
///
/// - **CH3_TX_LOOP_INT_ENA** `[15]` (RW)
///
///   The interrupt enable bit for CH3_TX_LOOP_INT.
///
/// - **CH2_TX_LOOP_INT_ENA** `[14]` (RW)
///
///   The interrupt enable bit for CH2_TX_LOOP_INT.
///
/// - **CH1_TX_LOOP_INT_ENA** `[13]` (RW)
///
///   The interrupt enable bit for CH1_TX_LOOP_INT.
///
/// - **CH0_TX_LOOP_INT_ENA** `[12]` (RW)
///
///   The interrupt enable bit for CH0_TX_LOOP_INT.
///
/// - **CH3_TX_THR_EVENT_INT_ENA** `[11]` (RW)
///
///   The interrupt enable bit for CH3_TX_THR_EVENT_INT.
///
/// - **CH2_TX_THR_EVENT_INT_ENA** `[10]` (RW)
///
///   The interrupt enable bit for CH2_TX_THR_EVENT_INT.
///
/// - **CH1_TX_THR_EVENT_INT_ENA** `[9]` (RW)
///
///   The interrupt enable bit for CH1_TX_THR_EVENT_INT.
///
/// - **CH0_TX_THR_EVENT_INT_ENA** `[8]` (RW)
///
///   The interrupt enable bit for CH0_TX_THR_EVENT_INT.
///
/// - **TX_CH3_ERR_INT_ENA** `[7]` (RW)
///
///   The interrupt enable bit for CH3_ERR_INT.
///
/// - **TX_CH2_ERR_INT_ENA** `[6]` (RW)
///
///   The interrupt enable bit for CH2_ERR_INT.
///
/// - **TX_CH1_ERR_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for CH1_ERR_INT.
///
/// - **TX_CH0_ERR_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for CH0_ERR_INT.
///
/// - **CH3_TX_END_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for CH3_TX_END_INT.
///
/// - **CH2_TX_END_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for CH2_TX_END_INT.
///
/// - **CH1_TX_END_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for CH1_TX_END_INT.
///
/// - **CH0_TX_END_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for CH0_TX_END_INT.
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

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_ENA**
    ///
    /// The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ENA_OFFSET: u32 = 29;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK: u32 = 0x20000000;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_ENA: u32 = 1 << 29;

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_ENA**
    ///
    /// The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ENA_OFFSET: u32 = 28;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK: u32 = 0x10000000;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_ENA: u32 = 1 << 28;

    /// **CH7_RX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CH7_RX_THR_EVENT_INT_ENA_OFFSET: u32 = 27;
    pub const CH7_RX_THR_EVENT_INT_ENA_MASK: u32 = 0x08000000;
    pub const CH7_RX_THR_EVENT_INT_ENA: u32 = 1 << 27;

    /// **CH6_RX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CH6_RX_THR_EVENT_INT_ENA_OFFSET: u32 = 26;
    pub const CH6_RX_THR_EVENT_INT_ENA_MASK: u32 = 0x04000000;
    pub const CH6_RX_THR_EVENT_INT_ENA: u32 = 1 << 26;

    /// **CH5_RX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CH5_RX_THR_EVENT_INT_ENA_OFFSET: u32 = 25;
    pub const CH5_RX_THR_EVENT_INT_ENA_MASK: u32 = 0x02000000;
    pub const CH5_RX_THR_EVENT_INT_ENA: u32 = 1 << 25;

    /// **CH4_RX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CH4_RX_THR_EVENT_INT_ENA_OFFSET: u32 = 24;
    pub const CH4_RX_THR_EVENT_INT_ENA_MASK: u32 = 0x01000000;
    pub const CH4_RX_THR_EVENT_INT_ENA: u32 = 1 << 24;

    /// **CH7_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH7_ERR_INT.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CH7_ERR_INT_ENA_OFFSET: u32 = 23;
    pub const CH7_ERR_INT_ENA_MASK: u32 = 0x00800000;
    pub const CH7_ERR_INT_ENA: u32 = 1 << 23;

    /// **CH6_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH6_ERR_INT.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CH6_ERR_INT_ENA_OFFSET: u32 = 22;
    pub const CH6_ERR_INT_ENA_MASK: u32 = 0x00400000;
    pub const CH6_ERR_INT_ENA: u32 = 1 << 22;

    /// **CH5_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH5_ERR_INT.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CH5_ERR_INT_ENA_OFFSET: u32 = 21;
    pub const CH5_ERR_INT_ENA_MASK: u32 = 0x00200000;
    pub const CH5_ERR_INT_ENA: u32 = 1 << 21;

    /// **CH4_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH4_ERR_INT.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CH4_ERR_INT_ENA_OFFSET: u32 = 20;
    pub const CH4_ERR_INT_ENA_MASK: u32 = 0x00100000;
    pub const CH4_ERR_INT_ENA: u32 = 1 << 20;

    /// **CH7_RX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH7_RX_END_INT.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CH7_RX_END_INT_ENA_OFFSET: u32 = 19;
    pub const CH7_RX_END_INT_ENA_MASK: u32 = 0x00080000;
    pub const CH7_RX_END_INT_ENA: u32 = 1 << 19;

    /// **CH6_RX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH6_RX_END_INT.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CH6_RX_END_INT_ENA_OFFSET: u32 = 18;
    pub const CH6_RX_END_INT_ENA_MASK: u32 = 0x00040000;
    pub const CH6_RX_END_INT_ENA: u32 = 1 << 18;

    /// **CH5_RX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH5_RX_END_INT.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CH5_RX_END_INT_ENA_OFFSET: u32 = 17;
    pub const CH5_RX_END_INT_ENA_MASK: u32 = 0x00020000;
    pub const CH5_RX_END_INT_ENA: u32 = 1 << 17;

    /// **CH4_RX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH4_RX_END_INT.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CH4_RX_END_INT_ENA_OFFSET: u32 = 16;
    pub const CH4_RX_END_INT_ENA_MASK: u32 = 0x00010000;
    pub const CH4_RX_END_INT_ENA: u32 = 1 << 16;

    /// **CH3_TX_LOOP_INT_ENA**
    ///
    /// The interrupt enable bit for CH3_TX_LOOP_INT.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CH3_TX_LOOP_INT_ENA_OFFSET: u32 = 15;
    pub const CH3_TX_LOOP_INT_ENA_MASK: u32 = 0x00008000;
    pub const CH3_TX_LOOP_INT_ENA: u32 = 1 << 15;

    /// **CH2_TX_LOOP_INT_ENA**
    ///
    /// The interrupt enable bit for CH2_TX_LOOP_INT.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CH2_TX_LOOP_INT_ENA_OFFSET: u32 = 14;
    pub const CH2_TX_LOOP_INT_ENA_MASK: u32 = 0x00004000;
    pub const CH2_TX_LOOP_INT_ENA: u32 = 1 << 14;

    /// **CH1_TX_LOOP_INT_ENA**
    ///
    /// The interrupt enable bit for CH1_TX_LOOP_INT.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CH1_TX_LOOP_INT_ENA_OFFSET: u32 = 13;
    pub const CH1_TX_LOOP_INT_ENA_MASK: u32 = 0x00002000;
    pub const CH1_TX_LOOP_INT_ENA: u32 = 1 << 13;

    /// **CH0_TX_LOOP_INT_ENA**
    ///
    /// The interrupt enable bit for CH0_TX_LOOP_INT.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CH0_TX_LOOP_INT_ENA_OFFSET: u32 = 12;
    pub const CH0_TX_LOOP_INT_ENA_MASK: u32 = 0x00001000;
    pub const CH0_TX_LOOP_INT_ENA: u32 = 1 << 12;

    /// **CH3_TX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH3_TX_THR_EVENT_INT_ENA_OFFSET: u32 = 11;
    pub const CH3_TX_THR_EVENT_INT_ENA_MASK: u32 = 0x00000800;
    pub const CH3_TX_THR_EVENT_INT_ENA: u32 = 1 << 11;

    /// **CH2_TX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH2_TX_THR_EVENT_INT_ENA_OFFSET: u32 = 10;
    pub const CH2_TX_THR_EVENT_INT_ENA_MASK: u32 = 0x00000400;
    pub const CH2_TX_THR_EVENT_INT_ENA: u32 = 1 << 10;

    /// **CH1_TX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH1_TX_THR_EVENT_INT_ENA_OFFSET: u32 = 9;
    pub const CH1_TX_THR_EVENT_INT_ENA_MASK: u32 = 0x00000200;
    pub const CH1_TX_THR_EVENT_INT_ENA: u32 = 1 << 9;

    /// **CH0_TX_THR_EVENT_INT_ENA**
    ///
    /// The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH0_TX_THR_EVENT_INT_ENA_OFFSET: u32 = 8;
    pub const CH0_TX_THR_EVENT_INT_ENA_MASK: u32 = 0x00000100;
    pub const CH0_TX_THR_EVENT_INT_ENA: u32 = 1 << 8;

    /// **TX_CH3_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH3_ERR_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_CH3_ERR_INT_ENA_OFFSET: u32 = 7;
    pub const TX_CH3_ERR_INT_ENA_MASK: u32 = 0x00000080;
    pub const TX_CH3_ERR_INT_ENA: u32 = 1 << 7;

    /// **TX_CH2_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH2_ERR_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_CH2_ERR_INT_ENA_OFFSET: u32 = 6;
    pub const TX_CH2_ERR_INT_ENA_MASK: u32 = 0x00000040;
    pub const TX_CH2_ERR_INT_ENA: u32 = 1 << 6;

    /// **TX_CH1_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH1_ERR_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_CH1_ERR_INT_ENA_OFFSET: u32 = 5;
    pub const TX_CH1_ERR_INT_ENA_MASK: u32 = 0x00000020;
    pub const TX_CH1_ERR_INT_ENA: u32 = 1 << 5;

    /// **TX_CH0_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for CH0_ERR_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_CH0_ERR_INT_ENA_OFFSET: u32 = 4;
    pub const TX_CH0_ERR_INT_ENA_MASK: u32 = 0x00000010;
    pub const TX_CH0_ERR_INT_ENA: u32 = 1 << 4;

    /// **CH3_TX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH3_TX_END_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_TX_END_INT_ENA_OFFSET: u32 = 3;
    pub const CH3_TX_END_INT_ENA_MASK: u32 = 0x00000008;
    pub const CH3_TX_END_INT_ENA: u32 = 1 << 3;

    /// **CH2_TX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH2_TX_END_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_TX_END_INT_ENA_OFFSET: u32 = 2;
    pub const CH2_TX_END_INT_ENA_MASK: u32 = 0x00000004;
    pub const CH2_TX_END_INT_ENA: u32 = 1 << 2;

    /// **CH1_TX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH1_TX_END_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_TX_END_INT_ENA_OFFSET: u32 = 1;
    pub const CH1_TX_END_INT_ENA_MASK: u32 = 0x00000002;
    pub const CH1_TX_END_INT_ENA: u32 = 1 << 1;

    /// **CH0_TX_END_INT_ENA**
    ///
    /// The interrupt enable bit for CH0_TX_END_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_TX_END_INT_ENA_OFFSET: u32 = 0;
    pub const CH0_TX_END_INT_ENA_MASK: u32 = 0x00000001;
    pub const CH0_TX_END_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_ENA** `[29]` (RW)
    ///
    /// The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK) != 0
    }

    /// Set RX_CH7_DMA_ACCESS_FAIL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ch7_dma_access_fail_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK);
    }

    /// Clear RX_CH7_DMA_ACCESS_FAIL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ch7_dma_access_fail_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK);
    }

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_ENA** `[28]` (RW)
    ///
    /// The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK) != 0
    }

    /// Set TX_CH3_DMA_ACCESS_FAIL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch3_dma_access_fail_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK);
    }

    /// Clear TX_CH3_DMA_ACCESS_FAIL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch3_dma_access_fail_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK);
    }

    /// **CH7_RX_THR_EVENT_INT_ENA** `[27]` (RW)
    ///
    /// The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH7_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch7_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH7_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH7_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch7_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH7_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH6_RX_THR_EVENT_INT_ENA** `[26]` (RW)
    ///
    /// The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH6_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch6_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH6_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH6_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch6_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH6_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH5_RX_THR_EVENT_INT_ENA** `[25]` (RW)
    ///
    /// The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH5_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch5_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH5_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH5_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch5_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH5_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH4_RX_THR_EVENT_INT_ENA** `[24]` (RW)
    ///
    /// The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH4_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch4_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH4_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH4_RX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch4_rx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH4_RX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH7_ERR_INT_ENA** `[23]` (RW)
    ///
    /// The interrupt enable bit for CH7_ERR_INT.
    #[inline(always)]
    pub fn ch7_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH7_ERR_INT_ENA_MASK) != 0
    }

    /// Set CH7_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch7_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CH7_ERR_INT_ENA_MASK);
    }

    /// Clear CH7_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch7_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CH7_ERR_INT_ENA_MASK);
    }

    /// **CH6_ERR_INT_ENA** `[22]` (RW)
    ///
    /// The interrupt enable bit for CH6_ERR_INT.
    #[inline(always)]
    pub fn ch6_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH6_ERR_INT_ENA_MASK) != 0
    }

    /// Set CH6_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch6_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CH6_ERR_INT_ENA_MASK);
    }

    /// Clear CH6_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch6_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CH6_ERR_INT_ENA_MASK);
    }

    /// **CH5_ERR_INT_ENA** `[21]` (RW)
    ///
    /// The interrupt enable bit for CH5_ERR_INT.
    #[inline(always)]
    pub fn ch5_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH5_ERR_INT_ENA_MASK) != 0
    }

    /// Set CH5_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch5_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CH5_ERR_INT_ENA_MASK);
    }

    /// Clear CH5_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch5_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CH5_ERR_INT_ENA_MASK);
    }

    /// **CH4_ERR_INT_ENA** `[20]` (RW)
    ///
    /// The interrupt enable bit for CH4_ERR_INT.
    #[inline(always)]
    pub fn ch4_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH4_ERR_INT_ENA_MASK) != 0
    }

    /// Set CH4_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch4_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CH4_ERR_INT_ENA_MASK);
    }

    /// Clear CH4_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch4_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CH4_ERR_INT_ENA_MASK);
    }

    /// **CH7_RX_END_INT_ENA** `[19]` (RW)
    ///
    /// The interrupt enable bit for CH7_RX_END_INT.
    #[inline(always)]
    pub fn ch7_rx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH7_RX_END_INT_ENA_MASK) != 0
    }

    /// Set CH7_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch7_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH7_RX_END_INT_ENA_MASK);
    }

    /// Clear CH7_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch7_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH7_RX_END_INT_ENA_MASK);
    }

    /// **CH6_RX_END_INT_ENA** `[18]` (RW)
    ///
    /// The interrupt enable bit for CH6_RX_END_INT.
    #[inline(always)]
    pub fn ch6_rx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH6_RX_END_INT_ENA_MASK) != 0
    }

    /// Set CH6_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch6_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH6_RX_END_INT_ENA_MASK);
    }

    /// Clear CH6_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch6_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH6_RX_END_INT_ENA_MASK);
    }

    /// **CH5_RX_END_INT_ENA** `[17]` (RW)
    ///
    /// The interrupt enable bit for CH5_RX_END_INT.
    #[inline(always)]
    pub fn ch5_rx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH5_RX_END_INT_ENA_MASK) != 0
    }

    /// Set CH5_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch5_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH5_RX_END_INT_ENA_MASK);
    }

    /// Clear CH5_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch5_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH5_RX_END_INT_ENA_MASK);
    }

    /// **CH4_RX_END_INT_ENA** `[16]` (RW)
    ///
    /// The interrupt enable bit for CH4_RX_END_INT.
    #[inline(always)]
    pub fn ch4_rx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH4_RX_END_INT_ENA_MASK) != 0
    }

    /// Set CH4_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch4_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH4_RX_END_INT_ENA_MASK);
    }

    /// Clear CH4_RX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch4_rx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH4_RX_END_INT_ENA_MASK);
    }

    /// **CH3_TX_LOOP_INT_ENA** `[15]` (RW)
    ///
    /// The interrupt enable bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Set CH3_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_LOOP_INT_ENA_MASK);
    }

    /// Clear CH3_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_LOOP_INT_ENA_MASK);
    }

    /// **CH2_TX_LOOP_INT_ENA** `[14]` (RW)
    ///
    /// The interrupt enable bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Set CH2_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_LOOP_INT_ENA_MASK);
    }

    /// Clear CH2_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_LOOP_INT_ENA_MASK);
    }

    /// **CH1_TX_LOOP_INT_ENA** `[13]` (RW)
    ///
    /// The interrupt enable bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Set CH1_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_LOOP_INT_ENA_MASK);
    }

    /// Clear CH1_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_LOOP_INT_ENA_MASK);
    }

    /// **CH0_TX_LOOP_INT_ENA** `[12]` (RW)
    ///
    /// The interrupt enable bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Set CH0_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_LOOP_INT_ENA_MASK);
    }

    /// Clear CH0_TX_LOOP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_loop_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_LOOP_INT_ENA_MASK);
    }

    /// **CH3_TX_THR_EVENT_INT_ENA** `[11]` (RW)
    ///
    /// The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH3_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH3_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH2_TX_THR_EVENT_INT_ENA** `[10]` (RW)
    ///
    /// The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH2_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH2_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH1_TX_THR_EVENT_INT_ENA** `[9]` (RW)
    ///
    /// The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH1_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH1_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// **CH0_TX_THR_EVENT_INT_ENA** `[8]` (RW)
    ///
    /// The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Set CH0_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// Clear CH0_TX_THR_EVENT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_thr_event_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_THR_EVENT_INT_ENA_MASK);
    }

    /// **TX_CH3_ERR_INT_ENA** `[7]` (RW)
    ///
    /// The interrupt enable bit for CH3_ERR_INT.
    #[inline(always)]
    pub fn tx_ch3_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH3_ERR_INT_ENA_MASK) != 0
    }

    /// Set TX_CH3_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch3_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH3_ERR_INT_ENA_MASK);
    }

    /// Clear TX_CH3_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch3_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH3_ERR_INT_ENA_MASK);
    }

    /// **TX_CH2_ERR_INT_ENA** `[6]` (RW)
    ///
    /// The interrupt enable bit for CH2_ERR_INT.
    #[inline(always)]
    pub fn tx_ch2_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH2_ERR_INT_ENA_MASK) != 0
    }

    /// Set TX_CH2_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch2_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH2_ERR_INT_ENA_MASK);
    }

    /// Clear TX_CH2_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch2_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH2_ERR_INT_ENA_MASK);
    }

    /// **TX_CH1_ERR_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for CH1_ERR_INT.
    #[inline(always)]
    pub fn tx_ch1_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH1_ERR_INT_ENA_MASK) != 0
    }

    /// Set TX_CH1_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch1_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH1_ERR_INT_ENA_MASK);
    }

    /// Clear TX_CH1_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch1_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH1_ERR_INT_ENA_MASK);
    }

    /// **TX_CH0_ERR_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for CH0_ERR_INT.
    #[inline(always)]
    pub fn tx_ch0_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_CH0_ERR_INT_ENA_MASK) != 0
    }

    /// Set TX_CH0_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ch0_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CH0_ERR_INT_ENA_MASK);
    }

    /// Clear TX_CH0_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ch0_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CH0_ERR_INT_ENA_MASK);
    }

    /// **CH3_TX_END_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for CH3_TX_END_INT.
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH3_TX_END_INT_ENA_MASK) != 0
    }

    /// Set CH3_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH3_TX_END_INT_ENA_MASK);
    }

    /// Clear CH3_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH3_TX_END_INT_ENA_MASK);
    }

    /// **CH2_TX_END_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for CH2_TX_END_INT.
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH2_TX_END_INT_ENA_MASK) != 0
    }

    /// Set CH2_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH2_TX_END_INT_ENA_MASK);
    }

    /// Clear CH2_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH2_TX_END_INT_ENA_MASK);
    }

    /// **CH1_TX_END_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for CH1_TX_END_INT.
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH1_TX_END_INT_ENA_MASK) != 0
    }

    /// Set CH1_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH1_TX_END_INT_ENA_MASK);
    }

    /// Clear CH1_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH1_TX_END_INT_ENA_MASK);
    }

    /// **CH0_TX_END_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for CH0_TX_END_INT.
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CH0_TX_END_INT_ENA_MASK) != 0
    }

    /// Set CH0_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CH0_TX_END_INT_ENA_MASK);
    }

    /// Clear CH0_TX_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0_tx_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CH0_TX_END_INT_ENA_MASK);
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

    /// Read **RX_CH7_DMA_ACCESS_FAIL_INT_ENA** `[29:29]`
    ///
    /// The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn rx_ch7_dma_access_fail_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK) != 0
    }

    /// Read **TX_CH3_DMA_ACCESS_FAIL_INT_ENA** `[28:28]`
    ///
    /// The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn tx_ch3_dma_access_fail_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK) != 0
    }

    /// Read **CH7_RX_THR_EVENT_INT_ENA** `[27:27]`
    ///
    /// The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch7_rx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH7_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH6_RX_THR_EVENT_INT_ENA** `[26:26]`
    ///
    /// The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch6_rx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH6_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH5_RX_THR_EVENT_INT_ENA** `[25:25]`
    ///
    /// The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch5_rx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH5_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH4_RX_THR_EVENT_INT_ENA** `[24:24]`
    ///
    /// The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch4_rx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH4_RX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH7_ERR_INT_ENA** `[23:23]`
    ///
    /// The interrupt enable bit for CH7_ERR_INT.
    #[inline(always)]
    pub const fn ch7_err_int_ena(&self) -> bool {
        (self.bits & IntEna::CH7_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CH6_ERR_INT_ENA** `[22:22]`
    ///
    /// The interrupt enable bit for CH6_ERR_INT.
    #[inline(always)]
    pub const fn ch6_err_int_ena(&self) -> bool {
        (self.bits & IntEna::CH6_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CH5_ERR_INT_ENA** `[21:21]`
    ///
    /// The interrupt enable bit for CH5_ERR_INT.
    #[inline(always)]
    pub const fn ch5_err_int_ena(&self) -> bool {
        (self.bits & IntEna::CH5_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CH4_ERR_INT_ENA** `[20:20]`
    ///
    /// The interrupt enable bit for CH4_ERR_INT.
    #[inline(always)]
    pub const fn ch4_err_int_ena(&self) -> bool {
        (self.bits & IntEna::CH4_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CH7_RX_END_INT_ENA** `[19:19]`
    ///
    /// The interrupt enable bit for CH7_RX_END_INT.
    #[inline(always)]
    pub const fn ch7_rx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH7_RX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH6_RX_END_INT_ENA** `[18:18]`
    ///
    /// The interrupt enable bit for CH6_RX_END_INT.
    #[inline(always)]
    pub const fn ch6_rx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH6_RX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH5_RX_END_INT_ENA** `[17:17]`
    ///
    /// The interrupt enable bit for CH5_RX_END_INT.
    #[inline(always)]
    pub const fn ch5_rx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH5_RX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH4_RX_END_INT_ENA** `[16:16]`
    ///
    /// The interrupt enable bit for CH4_RX_END_INT.
    #[inline(always)]
    pub const fn ch4_rx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH4_RX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH3_TX_LOOP_INT_ENA** `[15:15]`
    ///
    /// The interrupt enable bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch3_tx_loop_int_ena(&self) -> bool {
        (self.bits & IntEna::CH3_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Read **CH2_TX_LOOP_INT_ENA** `[14:14]`
    ///
    /// The interrupt enable bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch2_tx_loop_int_ena(&self) -> bool {
        (self.bits & IntEna::CH2_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Read **CH1_TX_LOOP_INT_ENA** `[13:13]`
    ///
    /// The interrupt enable bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch1_tx_loop_int_ena(&self) -> bool {
        (self.bits & IntEna::CH1_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Read **CH0_TX_LOOP_INT_ENA** `[12:12]`
    ///
    /// The interrupt enable bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub const fn ch0_tx_loop_int_ena(&self) -> bool {
        (self.bits & IntEna::CH0_TX_LOOP_INT_ENA_MASK) != 0
    }

    /// Read **CH3_TX_THR_EVENT_INT_ENA** `[11:11]`
    ///
    /// The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch3_tx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH3_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH2_TX_THR_EVENT_INT_ENA** `[10:10]`
    ///
    /// The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch2_tx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH2_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH1_TX_THR_EVENT_INT_ENA** `[9:9]`
    ///
    /// The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch1_tx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH1_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **CH0_TX_THR_EVENT_INT_ENA** `[8:8]`
    ///
    /// The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn ch0_tx_thr_event_int_ena(&self) -> bool {
        (self.bits & IntEna::CH0_TX_THR_EVENT_INT_ENA_MASK) != 0
    }

    /// Read **TX_CH3_ERR_INT_ENA** `[7:7]`
    ///
    /// The interrupt enable bit for CH3_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch3_err_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_CH3_ERR_INT_ENA_MASK) != 0
    }

    /// Read **TX_CH2_ERR_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for CH2_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch2_err_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_CH2_ERR_INT_ENA_MASK) != 0
    }

    /// Read **TX_CH1_ERR_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for CH1_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch1_err_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_CH1_ERR_INT_ENA_MASK) != 0
    }

    /// Read **TX_CH0_ERR_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for CH0_ERR_INT.
    #[inline(always)]
    pub const fn tx_ch0_err_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_CH0_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CH3_TX_END_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for CH3_TX_END_INT.
    #[inline(always)]
    pub const fn ch3_tx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH3_TX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH2_TX_END_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for CH2_TX_END_INT.
    #[inline(always)]
    pub const fn ch2_tx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH2_TX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH1_TX_END_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for CH1_TX_END_INT.
    #[inline(always)]
    pub const fn ch1_tx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH1_TX_END_INT_ENA_MASK) != 0
    }

    /// Read **CH0_TX_END_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for CH0_TX_END_INT.
    #[inline(always)]
    pub const fn ch0_tx_end_int_ena(&self) -> bool {
        (self.bits & IntEna::CH0_TX_END_INT_ENA_MASK) != 0
    }

    /// Set **RX_CH7_DMA_ACCESS_FAIL_INT_ENA** `[29:29]`
    ///
    /// The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn with_rx_ch7_dma_access_fail_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_CH7_DMA_ACCESS_FAIL_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_CH3_DMA_ACCESS_FAIL_INT_ENA** `[28:28]`
    ///
    /// The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub const fn with_tx_ch3_dma_access_fail_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_CH3_DMA_ACCESS_FAIL_INT_ENA_MASK
            },
        }
    }

    /// Set **CH7_RX_THR_EVENT_INT_ENA** `[27:27]`
    ///
    /// The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch7_rx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH7_RX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH7_RX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH6_RX_THR_EVENT_INT_ENA** `[26:26]`
    ///
    /// The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch6_rx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH6_RX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH6_RX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH5_RX_THR_EVENT_INT_ENA** `[25:25]`
    ///
    /// The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch5_rx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH5_RX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH5_RX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH4_RX_THR_EVENT_INT_ENA** `[24:24]`
    ///
    /// The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch4_rx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH4_RX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH4_RX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH7_ERR_INT_ENA** `[23:23]`
    ///
    /// The interrupt enable bit for CH7_ERR_INT.
    #[inline(always)]
    pub const fn with_ch7_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH7_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH7_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CH6_ERR_INT_ENA** `[22:22]`
    ///
    /// The interrupt enable bit for CH6_ERR_INT.
    #[inline(always)]
    pub const fn with_ch6_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH6_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH6_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CH5_ERR_INT_ENA** `[21:21]`
    ///
    /// The interrupt enable bit for CH5_ERR_INT.
    #[inline(always)]
    pub const fn with_ch5_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH5_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH5_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CH4_ERR_INT_ENA** `[20:20]`
    ///
    /// The interrupt enable bit for CH4_ERR_INT.
    #[inline(always)]
    pub const fn with_ch4_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH4_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH4_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CH7_RX_END_INT_ENA** `[19:19]`
    ///
    /// The interrupt enable bit for CH7_RX_END_INT.
    #[inline(always)]
    pub const fn with_ch7_rx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH7_RX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH7_RX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH6_RX_END_INT_ENA** `[18:18]`
    ///
    /// The interrupt enable bit for CH6_RX_END_INT.
    #[inline(always)]
    pub const fn with_ch6_rx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH6_RX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH6_RX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH5_RX_END_INT_ENA** `[17:17]`
    ///
    /// The interrupt enable bit for CH5_RX_END_INT.
    #[inline(always)]
    pub const fn with_ch5_rx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH5_RX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH5_RX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH4_RX_END_INT_ENA** `[16:16]`
    ///
    /// The interrupt enable bit for CH4_RX_END_INT.
    #[inline(always)]
    pub const fn with_ch4_rx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH4_RX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH4_RX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH3_TX_LOOP_INT_ENA** `[15:15]`
    ///
    /// The interrupt enable bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub const fn with_ch3_tx_loop_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH3_TX_LOOP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH3_TX_LOOP_INT_ENA_MASK
            },
        }
    }

    /// Set **CH2_TX_LOOP_INT_ENA** `[14:14]`
    ///
    /// The interrupt enable bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub const fn with_ch2_tx_loop_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH2_TX_LOOP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH2_TX_LOOP_INT_ENA_MASK
            },
        }
    }

    /// Set **CH1_TX_LOOP_INT_ENA** `[13:13]`
    ///
    /// The interrupt enable bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub const fn with_ch1_tx_loop_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH1_TX_LOOP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH1_TX_LOOP_INT_ENA_MASK
            },
        }
    }

    /// Set **CH0_TX_LOOP_INT_ENA** `[12:12]`
    ///
    /// The interrupt enable bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub const fn with_ch0_tx_loop_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH0_TX_LOOP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH0_TX_LOOP_INT_ENA_MASK
            },
        }
    }

    /// Set **CH3_TX_THR_EVENT_INT_ENA** `[11:11]`
    ///
    /// The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch3_tx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH3_TX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH3_TX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH2_TX_THR_EVENT_INT_ENA** `[10:10]`
    ///
    /// The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch2_tx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH2_TX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH2_TX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH1_TX_THR_EVENT_INT_ENA** `[9:9]`
    ///
    /// The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch1_tx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH1_TX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH1_TX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **CH0_TX_THR_EVENT_INT_ENA** `[8:8]`
    ///
    /// The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub const fn with_ch0_tx_thr_event_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH0_TX_THR_EVENT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH0_TX_THR_EVENT_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_CH3_ERR_INT_ENA** `[7:7]`
    ///
    /// The interrupt enable bit for CH3_ERR_INT.
    #[inline(always)]
    pub const fn with_tx_ch3_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_CH3_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_CH3_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_CH2_ERR_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for CH2_ERR_INT.
    #[inline(always)]
    pub const fn with_tx_ch2_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_CH2_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_CH2_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_CH1_ERR_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for CH1_ERR_INT.
    #[inline(always)]
    pub const fn with_tx_ch1_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_CH1_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_CH1_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_CH0_ERR_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for CH0_ERR_INT.
    #[inline(always)]
    pub const fn with_tx_ch0_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_CH0_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_CH0_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CH3_TX_END_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for CH3_TX_END_INT.
    #[inline(always)]
    pub const fn with_ch3_tx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH3_TX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH3_TX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH2_TX_END_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for CH2_TX_END_INT.
    #[inline(always)]
    pub const fn with_ch2_tx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH2_TX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH2_TX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH1_TX_END_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for CH1_TX_END_INT.
    #[inline(always)]
    pub const fn with_ch1_tx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH1_TX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH1_TX_END_INT_ENA_MASK
            },
        }
    }

    /// Set **CH0_TX_END_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for CH0_TX_END_INT.
    #[inline(always)]
    pub const fn with_ch0_tx_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CH0_TX_END_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CH0_TX_END_INT_ENA_MASK
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
/// Interrupt clear bits
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **RX_CH7_DMA_ACCESS_FAIL_INT_CLR** `[29]` (WO)
///
///   Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
///
/// - **TX_CH3_DMA_ACCESS_FAIL_INT_CLR** `[28]` (WO)
///
///   Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
///
/// - **CH7_RX_THR_EVENT_INT_CLR** `[27]` (WO)
///
///   Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
///
/// - **CH6_RX_THR_EVENT_INT_CLR** `[26]` (WO)
///
///   Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
///
/// - **CH5_RX_THR_EVENT_INT_CLR** `[25]` (WO)
///
///   Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
///
/// - **CH4_RX_THR_EVENT_INT_CLR** `[24]` (WO)
///
///   Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
///
/// - **RX_CH7_ERR_INT_CLR** `[23]` (WO)
///
///   Set this bit to clear theCH7_ERR_INT interrupt.
///
/// - **RX_CH6_ERR_INT_CLR** `[22]` (WO)
///
///   Set this bit to clear theCH6_ERR_INT interrupt.
///
/// - **RX_CH5_ERR_INT_CLR** `[21]` (WO)
///
///   Set this bit to clear theCH5_ERR_INT interrupt.
///
/// - **RX_CH4_ERR_INT_CLR** `[20]` (WO)
///
///   Set this bit to clear theCH4_ERR_INT interrupt.
///
/// - **CH7_RX_END_INT_CLR** `[19]` (WO)
///
///   Set this bit to clear theCH7_RX_END_INT interrupt.
///
/// - **CH6_RX_END_INT_CLR** `[18]` (WO)
///
///   Set this bit to clear theCH6_RX_END_INT interrupt.
///
/// - **CH5_RX_END_INT_CLR** `[17]` (WO)
///
///   Set this bit to clear theCH5_RX_END_INT interrupt.
///
/// - **CH4_RX_END_INT_CLR** `[16]` (WO)
///
///   Set this bit to clear theCH4_RX_END_INT interrupt.
///
/// - **CH3_TX_LOOP_INT_CLR** `[15]` (WO)
///
///   Set this bit to clear theCH3_TX_LOOP_INT interrupt.
///
/// - **CH2_TX_LOOP_INT_CLR** `[14]` (WO)
///
///   Set this bit to clear theCH2_TX_LOOP_INT interrupt.
///
/// - **CH1_TX_LOOP_INT_CLR** `[13]` (WO)
///
///   Set this bit to clear theCH1_TX_LOOP_INT interrupt.
///
/// - **CH0_TX_LOOP_INT_CLR** `[12]` (WO)
///
///   Set this bit to clear theCH0_TX_LOOP_INT interrupt.
///
/// - **CH3_TX_THR_EVENT_INT_CLR** `[11]` (WO)
///
///   Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
///
/// - **CH2_TX_THR_EVENT_INT_CLR** `[10]` (WO)
///
///   Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
///
/// - **CH1_TX_THR_EVENT_INT_CLR** `[9]` (WO)
///
///   Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
///
/// - **CH0_TX_THR_EVENT_INT_CLR** `[8]` (WO)
///
///   Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
///
/// - **TX_CH3_ERR_INT_CLR** `[7]` (WO)
///
///   Set this bit to clear theCH3_ERR_INT interrupt.
///
/// - **TX_CH2_ERR_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear theCH2_ERR_INT interrupt.
///
/// - **TX_CH1_ERR_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear theCH1_ERR_INT interrupt.
///
/// - **TX_CH0_ERR_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear theCH0_ERR_INT interrupt.
///
/// - **CH3_TX_END_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear theCH3_TX_END_INT interrupt.
///
/// - **CH2_TX_END_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear theCH2_TX_END_INT interrupt.
///
/// - **CH1_TX_END_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear theCH1_TX_END_INT interrupt.
///
/// - **CH0_TX_END_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear theCH0_TX_END_INT interrupt.
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

    /// **RX_CH7_DMA_ACCESS_FAIL_INT_CLR**
    ///
    /// Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_CLR_OFFSET: u32 = 29;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_CLR_MASK: u32 = 0x20000000;
    pub const RX_CH7_DMA_ACCESS_FAIL_INT_CLR: u32 = 1 << 29;

    /// **TX_CH3_DMA_ACCESS_FAIL_INT_CLR**
    ///
    /// Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_CLR_OFFSET: u32 = 28;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_CLR_MASK: u32 = 0x10000000;
    pub const TX_CH3_DMA_ACCESS_FAIL_INT_CLR: u32 = 1 << 28;

    /// **CH7_RX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CH7_RX_THR_EVENT_INT_CLR_OFFSET: u32 = 27;
    pub const CH7_RX_THR_EVENT_INT_CLR_MASK: u32 = 0x08000000;
    pub const CH7_RX_THR_EVENT_INT_CLR: u32 = 1 << 27;

    /// **CH6_RX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CH6_RX_THR_EVENT_INT_CLR_OFFSET: u32 = 26;
    pub const CH6_RX_THR_EVENT_INT_CLR_MASK: u32 = 0x04000000;
    pub const CH6_RX_THR_EVENT_INT_CLR: u32 = 1 << 26;

    /// **CH5_RX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CH5_RX_THR_EVENT_INT_CLR_OFFSET: u32 = 25;
    pub const CH5_RX_THR_EVENT_INT_CLR_MASK: u32 = 0x02000000;
    pub const CH5_RX_THR_EVENT_INT_CLR: u32 = 1 << 25;

    /// **CH4_RX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CH4_RX_THR_EVENT_INT_CLR_OFFSET: u32 = 24;
    pub const CH4_RX_THR_EVENT_INT_CLR_MASK: u32 = 0x01000000;
    pub const CH4_RX_THR_EVENT_INT_CLR: u32 = 1 << 24;

    /// **RX_CH7_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH7_ERR_INT interrupt.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RX_CH7_ERR_INT_CLR_OFFSET: u32 = 23;
    pub const RX_CH7_ERR_INT_CLR_MASK: u32 = 0x00800000;
    pub const RX_CH7_ERR_INT_CLR: u32 = 1 << 23;

    /// **RX_CH6_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH6_ERR_INT interrupt.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const RX_CH6_ERR_INT_CLR_OFFSET: u32 = 22;
    pub const RX_CH6_ERR_INT_CLR_MASK: u32 = 0x00400000;
    pub const RX_CH6_ERR_INT_CLR: u32 = 1 << 22;

    /// **RX_CH5_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH5_ERR_INT interrupt.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const RX_CH5_ERR_INT_CLR_OFFSET: u32 = 21;
    pub const RX_CH5_ERR_INT_CLR_MASK: u32 = 0x00200000;
    pub const RX_CH5_ERR_INT_CLR: u32 = 1 << 21;

    /// **RX_CH4_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH4_ERR_INT interrupt.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_CH4_ERR_INT_CLR_OFFSET: u32 = 20;
    pub const RX_CH4_ERR_INT_CLR_MASK: u32 = 0x00100000;
    pub const RX_CH4_ERR_INT_CLR: u32 = 1 << 20;

    /// **CH7_RX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH7_RX_END_INT interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CH7_RX_END_INT_CLR_OFFSET: u32 = 19;
    pub const CH7_RX_END_INT_CLR_MASK: u32 = 0x00080000;
    pub const CH7_RX_END_INT_CLR: u32 = 1 << 19;

    /// **CH6_RX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH6_RX_END_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CH6_RX_END_INT_CLR_OFFSET: u32 = 18;
    pub const CH6_RX_END_INT_CLR_MASK: u32 = 0x00040000;
    pub const CH6_RX_END_INT_CLR: u32 = 1 << 18;

    /// **CH5_RX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH5_RX_END_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CH5_RX_END_INT_CLR_OFFSET: u32 = 17;
    pub const CH5_RX_END_INT_CLR_MASK: u32 = 0x00020000;
    pub const CH5_RX_END_INT_CLR: u32 = 1 << 17;

    /// **CH4_RX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH4_RX_END_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CH4_RX_END_INT_CLR_OFFSET: u32 = 16;
    pub const CH4_RX_END_INT_CLR_MASK: u32 = 0x00010000;
    pub const CH4_RX_END_INT_CLR: u32 = 1 << 16;

    /// **CH3_TX_LOOP_INT_CLR**
    ///
    /// Set this bit to clear theCH3_TX_LOOP_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CH3_TX_LOOP_INT_CLR_OFFSET: u32 = 15;
    pub const CH3_TX_LOOP_INT_CLR_MASK: u32 = 0x00008000;
    pub const CH3_TX_LOOP_INT_CLR: u32 = 1 << 15;

    /// **CH2_TX_LOOP_INT_CLR**
    ///
    /// Set this bit to clear theCH2_TX_LOOP_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CH2_TX_LOOP_INT_CLR_OFFSET: u32 = 14;
    pub const CH2_TX_LOOP_INT_CLR_MASK: u32 = 0x00004000;
    pub const CH2_TX_LOOP_INT_CLR: u32 = 1 << 14;

    /// **CH1_TX_LOOP_INT_CLR**
    ///
    /// Set this bit to clear theCH1_TX_LOOP_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CH1_TX_LOOP_INT_CLR_OFFSET: u32 = 13;
    pub const CH1_TX_LOOP_INT_CLR_MASK: u32 = 0x00002000;
    pub const CH1_TX_LOOP_INT_CLR: u32 = 1 << 13;

    /// **CH0_TX_LOOP_INT_CLR**
    ///
    /// Set this bit to clear theCH0_TX_LOOP_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CH0_TX_LOOP_INT_CLR_OFFSET: u32 = 12;
    pub const CH0_TX_LOOP_INT_CLR_MASK: u32 = 0x00001000;
    pub const CH0_TX_LOOP_INT_CLR: u32 = 1 << 12;

    /// **CH3_TX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH3_TX_THR_EVENT_INT_CLR_OFFSET: u32 = 11;
    pub const CH3_TX_THR_EVENT_INT_CLR_MASK: u32 = 0x00000800;
    pub const CH3_TX_THR_EVENT_INT_CLR: u32 = 1 << 11;

    /// **CH2_TX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH2_TX_THR_EVENT_INT_CLR_OFFSET: u32 = 10;
    pub const CH2_TX_THR_EVENT_INT_CLR_MASK: u32 = 0x00000400;
    pub const CH2_TX_THR_EVENT_INT_CLR: u32 = 1 << 10;

    /// **CH1_TX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH1_TX_THR_EVENT_INT_CLR_OFFSET: u32 = 9;
    pub const CH1_TX_THR_EVENT_INT_CLR_MASK: u32 = 0x00000200;
    pub const CH1_TX_THR_EVENT_INT_CLR: u32 = 1 << 9;

    /// **CH0_TX_THR_EVENT_INT_CLR**
    ///
    /// Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH0_TX_THR_EVENT_INT_CLR_OFFSET: u32 = 8;
    pub const CH0_TX_THR_EVENT_INT_CLR_MASK: u32 = 0x00000100;
    pub const CH0_TX_THR_EVENT_INT_CLR: u32 = 1 << 8;

    /// **TX_CH3_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH3_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_CH3_ERR_INT_CLR_OFFSET: u32 = 7;
    pub const TX_CH3_ERR_INT_CLR_MASK: u32 = 0x00000080;
    pub const TX_CH3_ERR_INT_CLR: u32 = 1 << 7;

    /// **TX_CH2_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH2_ERR_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_CH2_ERR_INT_CLR_OFFSET: u32 = 6;
    pub const TX_CH2_ERR_INT_CLR_MASK: u32 = 0x00000040;
    pub const TX_CH2_ERR_INT_CLR: u32 = 1 << 6;

    /// **TX_CH1_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH1_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_CH1_ERR_INT_CLR_OFFSET: u32 = 5;
    pub const TX_CH1_ERR_INT_CLR_MASK: u32 = 0x00000020;
    pub const TX_CH1_ERR_INT_CLR: u32 = 1 << 5;

    /// **TX_CH0_ERR_INT_CLR**
    ///
    /// Set this bit to clear theCH0_ERR_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_CH0_ERR_INT_CLR_OFFSET: u32 = 4;
    pub const TX_CH0_ERR_INT_CLR_MASK: u32 = 0x00000010;
    pub const TX_CH0_ERR_INT_CLR: u32 = 1 << 4;

    /// **CH3_TX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH3_TX_END_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_TX_END_INT_CLR_OFFSET: u32 = 3;
    pub const CH3_TX_END_INT_CLR_MASK: u32 = 0x00000008;
    pub const CH3_TX_END_INT_CLR: u32 = 1 << 3;

    /// **CH2_TX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH2_TX_END_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_TX_END_INT_CLR_OFFSET: u32 = 2;
    pub const CH2_TX_END_INT_CLR_MASK: u32 = 0x00000004;
    pub const CH2_TX_END_INT_CLR: u32 = 1 << 2;

    /// **CH1_TX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH1_TX_END_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_TX_END_INT_CLR_OFFSET: u32 = 1;
    pub const CH1_TX_END_INT_CLR_MASK: u32 = 0x00000002;
    pub const CH1_TX_END_INT_CLR: u32 = 1 << 1;

    /// **CH0_TX_END_INT_CLR**
    ///
    /// Set this bit to clear theCH0_TX_END_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_TX_END_INT_CLR_OFFSET: u32 = 0;
    pub const CH0_TX_END_INT_CLR_MASK: u32 = 0x00000001;
    pub const CH0_TX_END_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **RX_CH7_DMA_ACCESS_FAIL_INT_CLR** (WO)
    ///
    /// Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    pub fn trigger_rx_ch7_dma_access_fail_int_clr(&self) {
        self.value
            .write_bits(Self::RX_CH7_DMA_ACCESS_FAIL_INT_CLR_MASK);
    }

    /// Trigger **TX_CH3_DMA_ACCESS_FAIL_INT_CLR** (WO)
    ///
    /// Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    pub fn trigger_tx_ch3_dma_access_fail_int_clr(&self) {
        self.value
            .write_bits(Self::TX_CH3_DMA_ACCESS_FAIL_INT_CLR_MASK);
    }

    /// Trigger **CH7_RX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch7_rx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH7_RX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH6_RX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch6_rx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH6_RX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH5_RX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch5_rx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH5_RX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH4_RX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch4_rx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH4_RX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **RX_CH7_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH7_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_rx_ch7_err_int_clr(&self) {
        self.value.write_bits(Self::RX_CH7_ERR_INT_CLR_MASK);
    }

    /// Trigger **RX_CH6_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH6_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_rx_ch6_err_int_clr(&self) {
        self.value.write_bits(Self::RX_CH6_ERR_INT_CLR_MASK);
    }

    /// Trigger **RX_CH5_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH5_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_rx_ch5_err_int_clr(&self) {
        self.value.write_bits(Self::RX_CH5_ERR_INT_CLR_MASK);
    }

    /// Trigger **RX_CH4_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH4_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_rx_ch4_err_int_clr(&self) {
        self.value.write_bits(Self::RX_CH4_ERR_INT_CLR_MASK);
    }

    /// Trigger **CH7_RX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH7_RX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch7_rx_end_int_clr(&self) {
        self.value.write_bits(Self::CH7_RX_END_INT_CLR_MASK);
    }

    /// Trigger **CH6_RX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH6_RX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch6_rx_end_int_clr(&self) {
        self.value.write_bits(Self::CH6_RX_END_INT_CLR_MASK);
    }

    /// Trigger **CH5_RX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH5_RX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch5_rx_end_int_clr(&self) {
        self.value.write_bits(Self::CH5_RX_END_INT_CLR_MASK);
    }

    /// Trigger **CH4_RX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH4_RX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch4_rx_end_int_clr(&self) {
        self.value.write_bits(Self::CH4_RX_END_INT_CLR_MASK);
    }

    /// Trigger **CH3_TX_LOOP_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH3_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch3_tx_loop_int_clr(&self) {
        self.value.write_bits(Self::CH3_TX_LOOP_INT_CLR_MASK);
    }

    /// Trigger **CH2_TX_LOOP_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH2_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch2_tx_loop_int_clr(&self) {
        self.value.write_bits(Self::CH2_TX_LOOP_INT_CLR_MASK);
    }

    /// Trigger **CH1_TX_LOOP_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH1_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch1_tx_loop_int_clr(&self) {
        self.value.write_bits(Self::CH1_TX_LOOP_INT_CLR_MASK);
    }

    /// Trigger **CH0_TX_LOOP_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH0_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch0_tx_loop_int_clr(&self) {
        self.value.write_bits(Self::CH0_TX_LOOP_INT_CLR_MASK);
    }

    /// Trigger **CH3_TX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch3_tx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH3_TX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH2_TX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch2_tx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH2_TX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH1_TX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch1_tx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH1_TX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **CH0_TX_THR_EVENT_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch0_tx_thr_event_int_clr(&self) {
        self.value.write_bits(Self::CH0_TX_THR_EVENT_INT_CLR_MASK);
    }

    /// Trigger **TX_CH3_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH3_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_tx_ch3_err_int_clr(&self) {
        self.value.write_bits(Self::TX_CH3_ERR_INT_CLR_MASK);
    }

    /// Trigger **TX_CH2_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH2_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_tx_ch2_err_int_clr(&self) {
        self.value.write_bits(Self::TX_CH2_ERR_INT_CLR_MASK);
    }

    /// Trigger **TX_CH1_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH1_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_tx_ch1_err_int_clr(&self) {
        self.value.write_bits(Self::TX_CH1_ERR_INT_CLR_MASK);
    }

    /// Trigger **TX_CH0_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH0_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_tx_ch0_err_int_clr(&self) {
        self.value.write_bits(Self::TX_CH0_ERR_INT_CLR_MASK);
    }

    /// Trigger **CH3_TX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH3_TX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch3_tx_end_int_clr(&self) {
        self.value.write_bits(Self::CH3_TX_END_INT_CLR_MASK);
    }

    /// Trigger **CH2_TX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH2_TX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch2_tx_end_int_clr(&self) {
        self.value.write_bits(Self::CH2_TX_END_INT_CLR_MASK);
    }

    /// Trigger **CH1_TX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH1_TX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch1_tx_end_int_clr(&self) {
        self.value.write_bits(Self::CH1_TX_END_INT_CLR_MASK);
    }

    /// Trigger **CH0_TX_END_INT_CLR** (WO)
    ///
    /// Set this bit to clear theCH0_TX_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_ch0_tx_end_int_clr(&self) {
        self.value.write_bits(Self::CH0_TX_END_INT_CLR_MASK);
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

    /// Set **RX_CH7_DMA_ACCESS_FAIL_INT_CLR** `[29:29]`
    ///
    /// Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    pub const fn with_rx_ch7_dma_access_fail_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_CH7_DMA_ACCESS_FAIL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_CH7_DMA_ACCESS_FAIL_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_CH3_DMA_ACCESS_FAIL_INT_CLR** `[28:28]`
    ///
    /// Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    pub const fn with_tx_ch3_dma_access_fail_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_CH3_DMA_ACCESS_FAIL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_CH3_DMA_ACCESS_FAIL_INT_CLR_MASK
            },
        }
    }

    /// Set **CH7_RX_THR_EVENT_INT_CLR** `[27:27]`
    ///
    /// Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch7_rx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH7_RX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH7_RX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH6_RX_THR_EVENT_INT_CLR** `[26:26]`
    ///
    /// Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch6_rx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH6_RX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH6_RX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH5_RX_THR_EVENT_INT_CLR** `[25:25]`
    ///
    /// Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch5_rx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH5_RX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH5_RX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH4_RX_THR_EVENT_INT_CLR** `[24:24]`
    ///
    /// Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch4_rx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH4_RX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH4_RX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_CH7_ERR_INT_CLR** `[23:23]`
    ///
    /// Set this bit to clear theCH7_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_rx_ch7_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_CH7_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_CH7_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_CH6_ERR_INT_CLR** `[22:22]`
    ///
    /// Set this bit to clear theCH6_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_rx_ch6_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_CH6_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_CH6_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_CH5_ERR_INT_CLR** `[21:21]`
    ///
    /// Set this bit to clear theCH5_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_rx_ch5_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_CH5_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_CH5_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_CH4_ERR_INT_CLR** `[20:20]`
    ///
    /// Set this bit to clear theCH4_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_rx_ch4_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_CH4_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_CH4_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **CH7_RX_END_INT_CLR** `[19:19]`
    ///
    /// Set this bit to clear theCH7_RX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch7_rx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH7_RX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH7_RX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH6_RX_END_INT_CLR** `[18:18]`
    ///
    /// Set this bit to clear theCH6_RX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch6_rx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH6_RX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH6_RX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH5_RX_END_INT_CLR** `[17:17]`
    ///
    /// Set this bit to clear theCH5_RX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch5_rx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH5_RX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH5_RX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH4_RX_END_INT_CLR** `[16:16]`
    ///
    /// Set this bit to clear theCH4_RX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch4_rx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH4_RX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH4_RX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH3_TX_LOOP_INT_CLR** `[15:15]`
    ///
    /// Set this bit to clear theCH3_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub const fn with_ch3_tx_loop_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH3_TX_LOOP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH3_TX_LOOP_INT_CLR_MASK
            },
        }
    }

    /// Set **CH2_TX_LOOP_INT_CLR** `[14:14]`
    ///
    /// Set this bit to clear theCH2_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub const fn with_ch2_tx_loop_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH2_TX_LOOP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH2_TX_LOOP_INT_CLR_MASK
            },
        }
    }

    /// Set **CH1_TX_LOOP_INT_CLR** `[13:13]`
    ///
    /// Set this bit to clear theCH1_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub const fn with_ch1_tx_loop_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH1_TX_LOOP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH1_TX_LOOP_INT_CLR_MASK
            },
        }
    }

    /// Set **CH0_TX_LOOP_INT_CLR** `[12:12]`
    ///
    /// Set this bit to clear theCH0_TX_LOOP_INT interrupt.
    #[inline(always)]
    pub const fn with_ch0_tx_loop_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH0_TX_LOOP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH0_TX_LOOP_INT_CLR_MASK
            },
        }
    }

    /// Set **CH3_TX_THR_EVENT_INT_CLR** `[11:11]`
    ///
    /// Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch3_tx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH3_TX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH3_TX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH2_TX_THR_EVENT_INT_CLR** `[10:10]`
    ///
    /// Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch2_tx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH2_TX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH2_TX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH1_TX_THR_EVENT_INT_CLR** `[9:9]`
    ///
    /// Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch1_tx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH1_TX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH1_TX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **CH0_TX_THR_EVENT_INT_CLR** `[8:8]`
    ///
    /// Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    pub const fn with_ch0_tx_thr_event_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH0_TX_THR_EVENT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH0_TX_THR_EVENT_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_CH3_ERR_INT_CLR** `[7:7]`
    ///
    /// Set this bit to clear theCH3_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_tx_ch3_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_CH3_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_CH3_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_CH2_ERR_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear theCH2_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_tx_ch2_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_CH2_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_CH2_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_CH1_ERR_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear theCH1_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_tx_ch1_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_CH1_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_CH1_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_CH0_ERR_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear theCH0_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_tx_ch0_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_CH0_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_CH0_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **CH3_TX_END_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear theCH3_TX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch3_tx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH3_TX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH3_TX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH2_TX_END_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear theCH2_TX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch2_tx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH2_TX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH2_TX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH1_TX_END_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear theCH1_TX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch1_tx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH1_TX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH1_TX_END_INT_CLR_MASK
            },
        }
    }

    /// Set **CH0_TX_END_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear theCH0_TX_END_INT interrupt.
    #[inline(always)]
    pub const fn with_ch0_tx_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CH0_TX_END_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CH0_TX_END_INT_CLR_MASK
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

/// CH0CARRIER_DUTY Register
///
/// Channel 0 duty cycle configuration register
///
/// Address offset: 0x0080
/// Reset value: 0x00400040
///
/// ## Fields
///
/// - **CARRIER_HIGH_CH** `[31:16]` (RW)
///
///   This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
///
/// - **CARRIER_LOW_CH** `[15:0]` (RW)
///
///   This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
#[repr(transparent)]
pub struct Ch0carrierDuty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0carrierDuty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0carrierDuty {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_CH**
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_CH**
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_high_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_CH_MASK) >> Self::CARRIER_HIGH_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_high_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_CH_MASK)
                | ((value << Self::CARRIER_HIGH_CH_OFFSET) & Self::CARRIER_HIGH_CH_MASK)
        });
    }

    /// **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_low_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_CH_MASK) >> Self::CARRIER_LOW_CH_OFFSET
    }

    /// Write **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_low_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_CH_MASK)
                | ((value << Self::CARRIER_LOW_CH_OFFSET) & Self::CARRIER_LOW_CH_MASK)
        });
    }
}

/// CH0CARRIER_DUTY value type for batch read/modify/write operations
///
/// Use `ch0carrierduty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0carrierduty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0carrierDutyVal {
    bits: u32,
}

impl Ch0carrierDutyVal {
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
        Self { bits: 0x00400040 }
    }

    /// Read **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_high_ch(&self) -> u32 {
        (self.bits & Ch0carrierDuty::CARRIER_HIGH_CH_MASK) >> Ch0carrierDuty::CARRIER_HIGH_CH_OFFSET
    }

    /// Read **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_low_ch(&self) -> u32 {
        (self.bits & Ch0carrierDuty::CARRIER_LOW_CH_MASK) >> Ch0carrierDuty::CARRIER_LOW_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_high_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0carrierDuty::CARRIER_HIGH_CH_MASK)
                | ((value << Ch0carrierDuty::CARRIER_HIGH_CH_OFFSET)
                    & Ch0carrierDuty::CARRIER_HIGH_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_low_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0carrierDuty::CARRIER_LOW_CH_MASK)
                | ((value << Ch0carrierDuty::CARRIER_LOW_CH_OFFSET)
                    & Ch0carrierDuty::CARRIER_LOW_CH_MASK),
        }
    }
}

impl Default for Ch0carrierDutyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0carrierDuty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0carrierDutyVal {
        Ch0carrierDutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0carrierDutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0carrierDutyVal) -> Ch0carrierDutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1CARRIER_DUTY Register
///
/// Channel 1 duty cycle configuration register
///
/// Address offset: 0x0084
/// Reset value: 0x00400040
///
/// ## Fields
///
/// - **CARRIER_HIGH_CH** `[31:16]` (RW)
///
///   This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
///
/// - **CARRIER_LOW_CH** `[15:0]` (RW)
///
///   This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
#[repr(transparent)]
pub struct Ch1carrierDuty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1carrierDuty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1carrierDuty {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_CH**
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_CH**
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_high_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_CH_MASK) >> Self::CARRIER_HIGH_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_high_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_CH_MASK)
                | ((value << Self::CARRIER_HIGH_CH_OFFSET) & Self::CARRIER_HIGH_CH_MASK)
        });
    }

    /// **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_low_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_CH_MASK) >> Self::CARRIER_LOW_CH_OFFSET
    }

    /// Write **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_low_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_CH_MASK)
                | ((value << Self::CARRIER_LOW_CH_OFFSET) & Self::CARRIER_LOW_CH_MASK)
        });
    }
}

/// CH1CARRIER_DUTY value type for batch read/modify/write operations
///
/// Use `ch1carrierduty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1carrierduty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1carrierDutyVal {
    bits: u32,
}

impl Ch1carrierDutyVal {
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
        Self { bits: 0x00400040 }
    }

    /// Read **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_high_ch(&self) -> u32 {
        (self.bits & Ch1carrierDuty::CARRIER_HIGH_CH_MASK) >> Ch1carrierDuty::CARRIER_HIGH_CH_OFFSET
    }

    /// Read **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_low_ch(&self) -> u32 {
        (self.bits & Ch1carrierDuty::CARRIER_LOW_CH_MASK) >> Ch1carrierDuty::CARRIER_LOW_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_high_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1carrierDuty::CARRIER_HIGH_CH_MASK)
                | ((value << Ch1carrierDuty::CARRIER_HIGH_CH_OFFSET)
                    & Ch1carrierDuty::CARRIER_HIGH_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_low_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1carrierDuty::CARRIER_LOW_CH_MASK)
                | ((value << Ch1carrierDuty::CARRIER_LOW_CH_OFFSET)
                    & Ch1carrierDuty::CARRIER_LOW_CH_MASK),
        }
    }
}

impl Default for Ch1carrierDutyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1carrierDuty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1carrierDutyVal {
        Ch1carrierDutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1carrierDutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1carrierDutyVal) -> Ch1carrierDutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2CARRIER_DUTY Register
///
/// Channel 2 duty cycle configuration register
///
/// Address offset: 0x0088
/// Reset value: 0x00400040
///
/// ## Fields
///
/// - **CARRIER_HIGH_CH** `[31:16]` (RW)
///
///   This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
///
/// - **CARRIER_LOW_CH** `[15:0]` (RW)
///
///   This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
#[repr(transparent)]
pub struct Ch2carrierDuty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2carrierDuty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2carrierDuty {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_CH**
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_CH**
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_high_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_CH_MASK) >> Self::CARRIER_HIGH_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_high_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_CH_MASK)
                | ((value << Self::CARRIER_HIGH_CH_OFFSET) & Self::CARRIER_HIGH_CH_MASK)
        });
    }

    /// **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_low_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_CH_MASK) >> Self::CARRIER_LOW_CH_OFFSET
    }

    /// Write **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_low_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_CH_MASK)
                | ((value << Self::CARRIER_LOW_CH_OFFSET) & Self::CARRIER_LOW_CH_MASK)
        });
    }
}

/// CH2CARRIER_DUTY value type for batch read/modify/write operations
///
/// Use `ch2carrierduty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2carrierduty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2carrierDutyVal {
    bits: u32,
}

impl Ch2carrierDutyVal {
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
        Self { bits: 0x00400040 }
    }

    /// Read **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_high_ch(&self) -> u32 {
        (self.bits & Ch2carrierDuty::CARRIER_HIGH_CH_MASK) >> Ch2carrierDuty::CARRIER_HIGH_CH_OFFSET
    }

    /// Read **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_low_ch(&self) -> u32 {
        (self.bits & Ch2carrierDuty::CARRIER_LOW_CH_MASK) >> Ch2carrierDuty::CARRIER_LOW_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_high_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2carrierDuty::CARRIER_HIGH_CH_MASK)
                | ((value << Ch2carrierDuty::CARRIER_HIGH_CH_OFFSET)
                    & Ch2carrierDuty::CARRIER_HIGH_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_low_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2carrierDuty::CARRIER_LOW_CH_MASK)
                | ((value << Ch2carrierDuty::CARRIER_LOW_CH_OFFSET)
                    & Ch2carrierDuty::CARRIER_LOW_CH_MASK),
        }
    }
}

impl Default for Ch2carrierDutyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2carrierDuty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2carrierDutyVal {
        Ch2carrierDutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2carrierDutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2carrierDutyVal) -> Ch2carrierDutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3CARRIER_DUTY Register
///
/// Channel 3 duty cycle configuration register
///
/// Address offset: 0x008c
/// Reset value: 0x00400040
///
/// ## Fields
///
/// - **CARRIER_HIGH_CH** `[31:16]` (RW)
///
///   This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
///
/// - **CARRIER_LOW_CH** `[15:0]` (RW)
///
///   This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
#[repr(transparent)]
pub struct Ch3carrierDuty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3carrierDuty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3carrierDuty {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_CH**
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_CH**
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_high_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_CH_MASK) >> Self::CARRIER_HIGH_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_CH** `[31:16]` (RW)
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_high_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_CH_MASK)
                | ((value << Self::CARRIER_HIGH_CH_OFFSET) & Self::CARRIER_HIGH_CH_MASK)
        });
    }

    /// **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn carrier_low_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_CH_MASK) >> Self::CARRIER_LOW_CH_OFFSET
    }

    /// Write **CARRIER_LOW_CH** `[15:0]` (RW)
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub fn write_carrier_low_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_CH_MASK)
                | ((value << Self::CARRIER_LOW_CH_OFFSET) & Self::CARRIER_LOW_CH_MASK)
        });
    }
}

/// CH3CARRIER_DUTY value type for batch read/modify/write operations
///
/// Use `ch3carrierduty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3carrierduty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3carrierDutyVal {
    bits: u32,
}

impl Ch3carrierDutyVal {
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
        Self { bits: 0x00400040 }
    }

    /// Read **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_high_ch(&self) -> u32 {
        (self.bits & Ch3carrierDuty::CARRIER_HIGH_CH_MASK) >> Ch3carrierDuty::CARRIER_HIGH_CH_OFFSET
    }

    /// Read **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn carrier_low_ch(&self) -> u32 {
        (self.bits & Ch3carrierDuty::CARRIER_LOW_CH_MASK) >> Ch3carrierDuty::CARRIER_LOW_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_CH** `[31:16]`
    ///
    /// This register is used to configure carrier wave 's high level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_high_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3carrierDuty::CARRIER_HIGH_CH_MASK)
                | ((value << Ch3carrierDuty::CARRIER_HIGH_CH_OFFSET)
                    & Ch3carrierDuty::CARRIER_HIGH_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_CH** `[15:0]`
    ///
    /// This register is used to configure carrier wave 's low level clock period for CHANNEL%s.
    #[inline(always)]
    pub const fn with_carrier_low_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3carrierDuty::CARRIER_LOW_CH_MASK)
                | ((value << Ch3carrierDuty::CARRIER_LOW_CH_OFFSET)
                    & Ch3carrierDuty::CARRIER_LOW_CH_MASK),
        }
    }
}

impl Default for Ch3carrierDutyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3carrierDuty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3carrierDutyVal {
        Ch3carrierDutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3carrierDutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3carrierDutyVal) -> Ch3carrierDutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_RX_CARRIER_RM Register
///
/// Channel 0 carrier remove register
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
///
///   The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
///
/// - **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
///
///   The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
#[repr(transparent)]
pub struct Ch0RxCarrierRm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0RxCarrierRm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0RxCarrierRm {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_THRES_CH**
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_THRES_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_THRES_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_THRES_CH**
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_THRES_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_THRES_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_high_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_THRES_CH_MASK)
            >> Self::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_high_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Self::CARRIER_HIGH_THRES_CH_OFFSET) & Self::CARRIER_HIGH_THRES_CH_MASK)
        });
    }

    /// **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_low_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_THRES_CH_MASK)
            >> Self::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Write **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_low_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Self::CARRIER_LOW_THRES_CH_OFFSET) & Self::CARRIER_LOW_THRES_CH_MASK)
        });
    }
}

/// CH0_RX_CARRIER_RM value type for batch read/modify/write operations
///
/// Use `ch0rxcarrierrm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0rxcarrierrm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0RxCarrierRmVal {
    bits: u32,
}

impl Ch0RxCarrierRmVal {
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

    /// Read **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_high_thres_ch(&self) -> u32 {
        (self.bits & Ch0RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
            >> Ch0RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Read **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_low_thres_ch(&self) -> u32 {
        (self.bits & Ch0RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
            >> Ch0RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_high_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Ch0RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET)
                    & Ch0RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_low_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Ch0RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET)
                    & Ch0RxCarrierRm::CARRIER_LOW_THRES_CH_MASK),
        }
    }
}

impl Default for Ch0RxCarrierRmVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0RxCarrierRm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0RxCarrierRmVal {
        Ch0RxCarrierRmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0RxCarrierRmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0RxCarrierRmVal) -> Ch0RxCarrierRmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_RX_CARRIER_RM Register
///
/// Channel 1 carrier remove register
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
///
///   The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
///
/// - **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
///
///   The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
#[repr(transparent)]
pub struct Ch1RxCarrierRm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1RxCarrierRm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1RxCarrierRm {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_THRES_CH**
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_THRES_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_THRES_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_THRES_CH**
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_THRES_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_THRES_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_high_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_THRES_CH_MASK)
            >> Self::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_high_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Self::CARRIER_HIGH_THRES_CH_OFFSET) & Self::CARRIER_HIGH_THRES_CH_MASK)
        });
    }

    /// **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_low_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_THRES_CH_MASK)
            >> Self::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Write **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_low_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Self::CARRIER_LOW_THRES_CH_OFFSET) & Self::CARRIER_LOW_THRES_CH_MASK)
        });
    }
}

/// CH1_RX_CARRIER_RM value type for batch read/modify/write operations
///
/// Use `ch1rxcarrierrm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1rxcarrierrm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1RxCarrierRmVal {
    bits: u32,
}

impl Ch1RxCarrierRmVal {
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

    /// Read **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_high_thres_ch(&self) -> u32 {
        (self.bits & Ch1RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
            >> Ch1RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Read **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_low_thres_ch(&self) -> u32 {
        (self.bits & Ch1RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
            >> Ch1RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_high_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Ch1RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET)
                    & Ch1RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_low_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Ch1RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET)
                    & Ch1RxCarrierRm::CARRIER_LOW_THRES_CH_MASK),
        }
    }
}

impl Default for Ch1RxCarrierRmVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1RxCarrierRm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1RxCarrierRmVal {
        Ch1RxCarrierRmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1RxCarrierRmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1RxCarrierRmVal) -> Ch1RxCarrierRmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_RX_CARRIER_RM Register
///
/// Channel 2 carrier remove register
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
///
///   The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
///
/// - **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
///
///   The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
#[repr(transparent)]
pub struct Ch2RxCarrierRm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2RxCarrierRm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2RxCarrierRm {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_THRES_CH**
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_THRES_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_THRES_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_THRES_CH**
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_THRES_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_THRES_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_high_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_THRES_CH_MASK)
            >> Self::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_high_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Self::CARRIER_HIGH_THRES_CH_OFFSET) & Self::CARRIER_HIGH_THRES_CH_MASK)
        });
    }

    /// **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_low_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_THRES_CH_MASK)
            >> Self::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Write **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_low_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Self::CARRIER_LOW_THRES_CH_OFFSET) & Self::CARRIER_LOW_THRES_CH_MASK)
        });
    }
}

/// CH2_RX_CARRIER_RM value type for batch read/modify/write operations
///
/// Use `ch2rxcarrierrm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2rxcarrierrm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2RxCarrierRmVal {
    bits: u32,
}

impl Ch2RxCarrierRmVal {
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

    /// Read **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_high_thres_ch(&self) -> u32 {
        (self.bits & Ch2RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
            >> Ch2RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Read **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_low_thres_ch(&self) -> u32 {
        (self.bits & Ch2RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
            >> Ch2RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_high_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Ch2RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET)
                    & Ch2RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_low_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Ch2RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET)
                    & Ch2RxCarrierRm::CARRIER_LOW_THRES_CH_MASK),
        }
    }
}

impl Default for Ch2RxCarrierRmVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2RxCarrierRm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2RxCarrierRmVal {
        Ch2RxCarrierRmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2RxCarrierRmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2RxCarrierRmVal) -> Ch2RxCarrierRmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_RX_CARRIER_RM Register
///
/// Channel 3 carrier remove register
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
///
///   The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
///
/// - **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
///
///   The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
#[repr(transparent)]
pub struct Ch3RxCarrierRm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3RxCarrierRm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3RxCarrierRm {
    // ======== Field Constants ========

    /// **CARRIER_HIGH_THRES_CH**
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARRIER_HIGH_THRES_CH_OFFSET: u32 = 16;
    pub const CARRIER_HIGH_THRES_CH_MASK: u32 = 0xffff0000;

    /// **CARRIER_LOW_THRES_CH**
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CARRIER_LOW_THRES_CH_OFFSET: u32 = 0;
    pub const CARRIER_LOW_THRES_CH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_high_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_HIGH_THRES_CH_MASK)
            >> Self::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Write **CARRIER_HIGH_THRES_CH** `[31:16]` (RW)
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_high_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Self::CARRIER_HIGH_THRES_CH_OFFSET) & Self::CARRIER_HIGH_THRES_CH_MASK)
        });
    }

    /// **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn carrier_low_thres_ch(&self) -> u32 {
        (self.value.read_bits() & Self::CARRIER_LOW_THRES_CH_MASK)
            >> Self::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Write **CARRIER_LOW_THRES_CH** `[15:0]` (RW)
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub fn write_carrier_low_thres_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Self::CARRIER_LOW_THRES_CH_OFFSET) & Self::CARRIER_LOW_THRES_CH_MASK)
        });
    }
}

/// CH3_RX_CARRIER_RM value type for batch read/modify/write operations
///
/// Use `ch3rxcarrierrm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3rxcarrierrm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3RxCarrierRmVal {
    bits: u32,
}

impl Ch3RxCarrierRmVal {
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

    /// Read **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_high_thres_ch(&self) -> u32 {
        (self.bits & Ch3RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
            >> Ch3RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET
    }

    /// Read **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn carrier_low_thres_ch(&self) -> u32 {
        (self.bits & Ch3RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
            >> Ch3RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET
    }

    /// Set **CARRIER_HIGH_THRES_CH** `[31:16]`
    ///
    /// The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_high_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK)
                | ((value << Ch3RxCarrierRm::CARRIER_HIGH_THRES_CH_OFFSET)
                    & Ch3RxCarrierRm::CARRIER_HIGH_THRES_CH_MASK),
        }
    }

    /// Set **CARRIER_LOW_THRES_CH** `[15:0]`
    ///
    /// The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s.
    #[inline(always)]
    pub const fn with_carrier_low_thres_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3RxCarrierRm::CARRIER_LOW_THRES_CH_MASK)
                | ((value << Ch3RxCarrierRm::CARRIER_LOW_THRES_CH_OFFSET)
                    & Ch3RxCarrierRm::CARRIER_LOW_THRES_CH_MASK),
        }
    }
}

impl Default for Ch3RxCarrierRmVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3RxCarrierRm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3RxCarrierRmVal {
        Ch3RxCarrierRmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3RxCarrierRmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3RxCarrierRmVal) -> Ch3RxCarrierRmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_TX_LIM Register
///
/// Channel 0 Tx event configuration register
///
/// Address offset: 0x00a0
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **LOOP_STOP_EN_CH** `[21]` (RW)
///
///   This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
///
/// - **LOOP_COUNT_RESET_CH** `[20]` (WO)
///
///   This register is used to reset the loop count when tx_conti_mode is valid.
///
/// - **TX_LOOP_CNT_EN_CH** `[19]` (RW)
///
///   This register is the enabled bit for loop count.
///
/// - **TX_LOOP_NUM_CH** `[18:9]` (RW)
///
///   This register is used to configure the maximum loop count when tx_conti_mode is valid.
///
/// - **TX_LIM_CH** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can send out.
#[repr(transparent)]
pub struct Ch0TxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0TxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0TxLim {
    // ======== Field Constants ========

    /// **LOOP_STOP_EN_CH**
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LOOP_STOP_EN_CH_OFFSET: u32 = 21;
    pub const LOOP_STOP_EN_CH_MASK: u32 = 0x00200000;
    pub const LOOP_STOP_EN_CH: u32 = 1 << 21;

    /// **LOOP_COUNT_RESET_CH**
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LOOP_COUNT_RESET_CH_OFFSET: u32 = 20;
    pub const LOOP_COUNT_RESET_CH_MASK: u32 = 0x00100000;
    pub const LOOP_COUNT_RESET_CH: u32 = 1 << 20;

    /// **TX_LOOP_CNT_EN_CH**
    ///
    /// This register is the enabled bit for loop count.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TX_LOOP_CNT_EN_CH_OFFSET: u32 = 19;
    pub const TX_LOOP_CNT_EN_CH_MASK: u32 = 0x00080000;
    pub const TX_LOOP_CNT_EN_CH: u32 = 1 << 19;

    /// **TX_LOOP_NUM_CH**
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    ///
    /// Bits `[18:9]`, mask `0x0007fe00`
    pub const TX_LOOP_NUM_CH_OFFSET: u32 = 9;
    pub const TX_LOOP_NUM_CH_MASK: u32 = 0x0007fe00;

    /// **TX_LIM_CH**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TX_LIM_CH_OFFSET: u32 = 0;
    pub const TX_LIM_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **LOOP_STOP_EN_CH** `[21]` (RW)
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Set LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Clear LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Trigger **LOOP_COUNT_RESET_CH** (WO)
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn trigger_loop_count_reset_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::LOOP_COUNT_RESET_CH_MASK);
    }

    /// **TX_LOOP_CNT_EN_CH** `[19]` (RW)
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Set TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_loop_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// Clear TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_loop_cnt_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LOOP_NUM_CH_MASK) >> Self::TX_LOOP_NUM_CH_OFFSET
    }

    /// Write **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn write_tx_loop_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LOOP_NUM_CH_MASK)
                | ((value << Self::TX_LOOP_NUM_CH_OFFSET) & Self::TX_LOOP_NUM_CH_MASK)
        });
    }

    /// **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LIM_CH_MASK) >> Self::TX_LIM_CH_OFFSET
    }

    /// Write **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn write_tx_lim_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LIM_CH_MASK) | ((value << Self::TX_LIM_CH_OFFSET) & Self::TX_LIM_CH_MASK)
        });
    }
}

/// CH0_TX_LIM value type for batch read/modify/write operations
///
/// Use `ch0txlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0txlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0TxLimVal {
    bits: u32,
}

impl Ch0TxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn loop_stop_en_ch(&self) -> bool {
        (self.bits & Ch0TxLim::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.bits & Ch0TxLim::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn tx_loop_num_ch(&self) -> u32 {
        (self.bits & Ch0TxLim::TX_LOOP_NUM_CH_MASK) >> Ch0TxLim::TX_LOOP_NUM_CH_OFFSET
    }

    /// Read **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn tx_lim_ch(&self) -> u32 {
        (self.bits & Ch0TxLim::TX_LIM_CH_MASK) >> Ch0TxLim::TX_LIM_CH_OFFSET
    }

    /// Set **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn with_loop_stop_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0TxLim::LOOP_STOP_EN_CH_MASK
            } else {
                self.bits & !Ch0TxLim::LOOP_STOP_EN_CH_MASK
            },
        }
    }

    /// Set **LOOP_COUNT_RESET_CH** `[20:20]`
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_loop_count_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0TxLim::LOOP_COUNT_RESET_CH_MASK
            } else {
                self.bits & !Ch0TxLim::LOOP_COUNT_RESET_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn with_tx_loop_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0TxLim::TX_LOOP_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch0TxLim::TX_LOOP_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_tx_loop_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0TxLim::TX_LOOP_NUM_CH_MASK)
                | ((value << Ch0TxLim::TX_LOOP_NUM_CH_OFFSET) & Ch0TxLim::TX_LOOP_NUM_CH_MASK),
        }
    }

    /// Set **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn with_tx_lim_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0TxLim::TX_LIM_CH_MASK)
                | ((value << Ch0TxLim::TX_LIM_CH_OFFSET) & Ch0TxLim::TX_LIM_CH_MASK),
        }
    }
}

impl Default for Ch0TxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0TxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0TxLimVal {
        Ch0TxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0TxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0TxLimVal) -> Ch0TxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_TX_LIM Register
///
/// Channel 1 Tx event configuration register
///
/// Address offset: 0x00a4
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **LOOP_STOP_EN_CH** `[21]` (RW)
///
///   This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
///
/// - **LOOP_COUNT_RESET_CH** `[20]` (WO)
///
///   This register is used to reset the loop count when tx_conti_mode is valid.
///
/// - **TX_LOOP_CNT_EN_CH** `[19]` (RW)
///
///   This register is the enabled bit for loop count.
///
/// - **TX_LOOP_NUM_CH** `[18:9]` (RW)
///
///   This register is used to configure the maximum loop count when tx_conti_mode is valid.
///
/// - **TX_LIM_CH** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can send out.
#[repr(transparent)]
pub struct Ch1TxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1TxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1TxLim {
    // ======== Field Constants ========

    /// **LOOP_STOP_EN_CH**
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LOOP_STOP_EN_CH_OFFSET: u32 = 21;
    pub const LOOP_STOP_EN_CH_MASK: u32 = 0x00200000;
    pub const LOOP_STOP_EN_CH: u32 = 1 << 21;

    /// **LOOP_COUNT_RESET_CH**
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LOOP_COUNT_RESET_CH_OFFSET: u32 = 20;
    pub const LOOP_COUNT_RESET_CH_MASK: u32 = 0x00100000;
    pub const LOOP_COUNT_RESET_CH: u32 = 1 << 20;

    /// **TX_LOOP_CNT_EN_CH**
    ///
    /// This register is the enabled bit for loop count.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TX_LOOP_CNT_EN_CH_OFFSET: u32 = 19;
    pub const TX_LOOP_CNT_EN_CH_MASK: u32 = 0x00080000;
    pub const TX_LOOP_CNT_EN_CH: u32 = 1 << 19;

    /// **TX_LOOP_NUM_CH**
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    ///
    /// Bits `[18:9]`, mask `0x0007fe00`
    pub const TX_LOOP_NUM_CH_OFFSET: u32 = 9;
    pub const TX_LOOP_NUM_CH_MASK: u32 = 0x0007fe00;

    /// **TX_LIM_CH**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TX_LIM_CH_OFFSET: u32 = 0;
    pub const TX_LIM_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **LOOP_STOP_EN_CH** `[21]` (RW)
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Set LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Clear LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Trigger **LOOP_COUNT_RESET_CH** (WO)
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn trigger_loop_count_reset_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::LOOP_COUNT_RESET_CH_MASK);
    }

    /// **TX_LOOP_CNT_EN_CH** `[19]` (RW)
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Set TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_loop_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// Clear TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_loop_cnt_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LOOP_NUM_CH_MASK) >> Self::TX_LOOP_NUM_CH_OFFSET
    }

    /// Write **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn write_tx_loop_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LOOP_NUM_CH_MASK)
                | ((value << Self::TX_LOOP_NUM_CH_OFFSET) & Self::TX_LOOP_NUM_CH_MASK)
        });
    }

    /// **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LIM_CH_MASK) >> Self::TX_LIM_CH_OFFSET
    }

    /// Write **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn write_tx_lim_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LIM_CH_MASK) | ((value << Self::TX_LIM_CH_OFFSET) & Self::TX_LIM_CH_MASK)
        });
    }
}

/// CH1_TX_LIM value type for batch read/modify/write operations
///
/// Use `ch1txlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1txlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1TxLimVal {
    bits: u32,
}

impl Ch1TxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn loop_stop_en_ch(&self) -> bool {
        (self.bits & Ch1TxLim::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.bits & Ch1TxLim::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn tx_loop_num_ch(&self) -> u32 {
        (self.bits & Ch1TxLim::TX_LOOP_NUM_CH_MASK) >> Ch1TxLim::TX_LOOP_NUM_CH_OFFSET
    }

    /// Read **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn tx_lim_ch(&self) -> u32 {
        (self.bits & Ch1TxLim::TX_LIM_CH_MASK) >> Ch1TxLim::TX_LIM_CH_OFFSET
    }

    /// Set **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn with_loop_stop_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1TxLim::LOOP_STOP_EN_CH_MASK
            } else {
                self.bits & !Ch1TxLim::LOOP_STOP_EN_CH_MASK
            },
        }
    }

    /// Set **LOOP_COUNT_RESET_CH** `[20:20]`
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_loop_count_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1TxLim::LOOP_COUNT_RESET_CH_MASK
            } else {
                self.bits & !Ch1TxLim::LOOP_COUNT_RESET_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn with_tx_loop_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1TxLim::TX_LOOP_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch1TxLim::TX_LOOP_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_tx_loop_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1TxLim::TX_LOOP_NUM_CH_MASK)
                | ((value << Ch1TxLim::TX_LOOP_NUM_CH_OFFSET) & Ch1TxLim::TX_LOOP_NUM_CH_MASK),
        }
    }

    /// Set **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn with_tx_lim_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1TxLim::TX_LIM_CH_MASK)
                | ((value << Ch1TxLim::TX_LIM_CH_OFFSET) & Ch1TxLim::TX_LIM_CH_MASK),
        }
    }
}

impl Default for Ch1TxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1TxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1TxLimVal {
        Ch1TxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1TxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1TxLimVal) -> Ch1TxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_TX_LIM Register
///
/// Channel 2 Tx event configuration register
///
/// Address offset: 0x00a8
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **LOOP_STOP_EN_CH** `[21]` (RW)
///
///   This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
///
/// - **LOOP_COUNT_RESET_CH** `[20]` (WO)
///
///   This register is used to reset the loop count when tx_conti_mode is valid.
///
/// - **TX_LOOP_CNT_EN_CH** `[19]` (RW)
///
///   This register is the enabled bit for loop count.
///
/// - **TX_LOOP_NUM_CH** `[18:9]` (RW)
///
///   This register is used to configure the maximum loop count when tx_conti_mode is valid.
///
/// - **TX_LIM_CH** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can send out.
#[repr(transparent)]
pub struct Ch2TxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2TxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2TxLim {
    // ======== Field Constants ========

    /// **LOOP_STOP_EN_CH**
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LOOP_STOP_EN_CH_OFFSET: u32 = 21;
    pub const LOOP_STOP_EN_CH_MASK: u32 = 0x00200000;
    pub const LOOP_STOP_EN_CH: u32 = 1 << 21;

    /// **LOOP_COUNT_RESET_CH**
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LOOP_COUNT_RESET_CH_OFFSET: u32 = 20;
    pub const LOOP_COUNT_RESET_CH_MASK: u32 = 0x00100000;
    pub const LOOP_COUNT_RESET_CH: u32 = 1 << 20;

    /// **TX_LOOP_CNT_EN_CH**
    ///
    /// This register is the enabled bit for loop count.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TX_LOOP_CNT_EN_CH_OFFSET: u32 = 19;
    pub const TX_LOOP_CNT_EN_CH_MASK: u32 = 0x00080000;
    pub const TX_LOOP_CNT_EN_CH: u32 = 1 << 19;

    /// **TX_LOOP_NUM_CH**
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    ///
    /// Bits `[18:9]`, mask `0x0007fe00`
    pub const TX_LOOP_NUM_CH_OFFSET: u32 = 9;
    pub const TX_LOOP_NUM_CH_MASK: u32 = 0x0007fe00;

    /// **TX_LIM_CH**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TX_LIM_CH_OFFSET: u32 = 0;
    pub const TX_LIM_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **LOOP_STOP_EN_CH** `[21]` (RW)
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Set LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Clear LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Trigger **LOOP_COUNT_RESET_CH** (WO)
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn trigger_loop_count_reset_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::LOOP_COUNT_RESET_CH_MASK);
    }

    /// **TX_LOOP_CNT_EN_CH** `[19]` (RW)
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Set TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_loop_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// Clear TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_loop_cnt_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LOOP_NUM_CH_MASK) >> Self::TX_LOOP_NUM_CH_OFFSET
    }

    /// Write **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn write_tx_loop_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LOOP_NUM_CH_MASK)
                | ((value << Self::TX_LOOP_NUM_CH_OFFSET) & Self::TX_LOOP_NUM_CH_MASK)
        });
    }

    /// **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LIM_CH_MASK) >> Self::TX_LIM_CH_OFFSET
    }

    /// Write **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn write_tx_lim_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LIM_CH_MASK) | ((value << Self::TX_LIM_CH_OFFSET) & Self::TX_LIM_CH_MASK)
        });
    }
}

/// CH2_TX_LIM value type for batch read/modify/write operations
///
/// Use `ch2txlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2txlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2TxLimVal {
    bits: u32,
}

impl Ch2TxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn loop_stop_en_ch(&self) -> bool {
        (self.bits & Ch2TxLim::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.bits & Ch2TxLim::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn tx_loop_num_ch(&self) -> u32 {
        (self.bits & Ch2TxLim::TX_LOOP_NUM_CH_MASK) >> Ch2TxLim::TX_LOOP_NUM_CH_OFFSET
    }

    /// Read **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn tx_lim_ch(&self) -> u32 {
        (self.bits & Ch2TxLim::TX_LIM_CH_MASK) >> Ch2TxLim::TX_LIM_CH_OFFSET
    }

    /// Set **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn with_loop_stop_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2TxLim::LOOP_STOP_EN_CH_MASK
            } else {
                self.bits & !Ch2TxLim::LOOP_STOP_EN_CH_MASK
            },
        }
    }

    /// Set **LOOP_COUNT_RESET_CH** `[20:20]`
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_loop_count_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2TxLim::LOOP_COUNT_RESET_CH_MASK
            } else {
                self.bits & !Ch2TxLim::LOOP_COUNT_RESET_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn with_tx_loop_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2TxLim::TX_LOOP_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch2TxLim::TX_LOOP_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_tx_loop_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2TxLim::TX_LOOP_NUM_CH_MASK)
                | ((value << Ch2TxLim::TX_LOOP_NUM_CH_OFFSET) & Ch2TxLim::TX_LOOP_NUM_CH_MASK),
        }
    }

    /// Set **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn with_tx_lim_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2TxLim::TX_LIM_CH_MASK)
                | ((value << Ch2TxLim::TX_LIM_CH_OFFSET) & Ch2TxLim::TX_LIM_CH_MASK),
        }
    }
}

impl Default for Ch2TxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2TxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2TxLimVal {
        Ch2TxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2TxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2TxLimVal) -> Ch2TxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_TX_LIM Register
///
/// Channel 3 Tx event configuration register
///
/// Address offset: 0x00ac
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **LOOP_STOP_EN_CH** `[21]` (RW)
///
///   This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
///
/// - **LOOP_COUNT_RESET_CH** `[20]` (WO)
///
///   This register is used to reset the loop count when tx_conti_mode is valid.
///
/// - **TX_LOOP_CNT_EN_CH** `[19]` (RW)
///
///   This register is the enabled bit for loop count.
///
/// - **TX_LOOP_NUM_CH** `[18:9]` (RW)
///
///   This register is used to configure the maximum loop count when tx_conti_mode is valid.
///
/// - **TX_LIM_CH** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can send out.
#[repr(transparent)]
pub struct Ch3TxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3TxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3TxLim {
    // ======== Field Constants ========

    /// **LOOP_STOP_EN_CH**
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LOOP_STOP_EN_CH_OFFSET: u32 = 21;
    pub const LOOP_STOP_EN_CH_MASK: u32 = 0x00200000;
    pub const LOOP_STOP_EN_CH: u32 = 1 << 21;

    /// **LOOP_COUNT_RESET_CH**
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LOOP_COUNT_RESET_CH_OFFSET: u32 = 20;
    pub const LOOP_COUNT_RESET_CH_MASK: u32 = 0x00100000;
    pub const LOOP_COUNT_RESET_CH: u32 = 1 << 20;

    /// **TX_LOOP_CNT_EN_CH**
    ///
    /// This register is the enabled bit for loop count.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TX_LOOP_CNT_EN_CH_OFFSET: u32 = 19;
    pub const TX_LOOP_CNT_EN_CH_MASK: u32 = 0x00080000;
    pub const TX_LOOP_CNT_EN_CH: u32 = 1 << 19;

    /// **TX_LOOP_NUM_CH**
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    ///
    /// Bits `[18:9]`, mask `0x0007fe00`
    pub const TX_LOOP_NUM_CH_OFFSET: u32 = 9;
    pub const TX_LOOP_NUM_CH_MASK: u32 = 0x0007fe00;

    /// **TX_LIM_CH**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TX_LIM_CH_OFFSET: u32 = 0;
    pub const TX_LIM_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **LOOP_STOP_EN_CH** `[21]` (RW)
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Set LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Clear LOOP_STOP_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loop_stop_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::LOOP_STOP_EN_CH_MASK);
    }

    /// Trigger **LOOP_COUNT_RESET_CH** (WO)
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn trigger_loop_count_reset_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::LOOP_COUNT_RESET_CH_MASK);
    }

    /// **TX_LOOP_CNT_EN_CH** `[19]` (RW)
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Set TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_loop_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// Clear TX_LOOP_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_loop_cnt_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_LOOP_CNT_EN_CH_MASK);
    }

    /// **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LOOP_NUM_CH_MASK) >> Self::TX_LOOP_NUM_CH_OFFSET
    }

    /// Write **TX_LOOP_NUM_CH** `[18:9]` (RW)
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn write_tx_loop_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LOOP_NUM_CH_MASK)
                | ((value << Self::TX_LOOP_NUM_CH_OFFSET) & Self::TX_LOOP_NUM_CH_MASK)
        });
    }

    /// **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_LIM_CH_MASK) >> Self::TX_LIM_CH_OFFSET
    }

    /// Write **TX_LIM_CH** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn write_tx_lim_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_LIM_CH_MASK) | ((value << Self::TX_LIM_CH_OFFSET) & Self::TX_LIM_CH_MASK)
        });
    }
}

/// CH3_TX_LIM value type for batch read/modify/write operations
///
/// Use `ch3txlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3txlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3TxLimVal {
    bits: u32,
}

impl Ch3TxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn loop_stop_en_ch(&self) -> bool {
        (self.bits & Ch3TxLim::LOOP_STOP_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn tx_loop_cnt_en_ch(&self) -> bool {
        (self.bits & Ch3TxLim::TX_LOOP_CNT_EN_CH_MASK) != 0
    }

    /// Read **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn tx_loop_num_ch(&self) -> u32 {
        (self.bits & Ch3TxLim::TX_LOOP_NUM_CH_MASK) >> Ch3TxLim::TX_LOOP_NUM_CH_OFFSET
    }

    /// Read **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn tx_lim_ch(&self) -> u32 {
        (self.bits & Ch3TxLim::TX_LIM_CH_MASK) >> Ch3TxLim::TX_LIM_CH_OFFSET
    }

    /// Set **LOOP_STOP_EN_CH** `[21:21]`
    ///
    /// This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub const fn with_loop_stop_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3TxLim::LOOP_STOP_EN_CH_MASK
            } else {
                self.bits & !Ch3TxLim::LOOP_STOP_EN_CH_MASK
            },
        }
    }

    /// Set **LOOP_COUNT_RESET_CH** `[20:20]`
    ///
    /// This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_loop_count_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3TxLim::LOOP_COUNT_RESET_CH_MASK
            } else {
                self.bits & !Ch3TxLim::LOOP_COUNT_RESET_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_CNT_EN_CH** `[19:19]`
    ///
    /// This register is the enabled bit for loop count.
    #[inline(always)]
    pub const fn with_tx_loop_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3TxLim::TX_LOOP_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch3TxLim::TX_LOOP_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **TX_LOOP_NUM_CH** `[18:9]`
    ///
    /// This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub const fn with_tx_loop_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3TxLim::TX_LOOP_NUM_CH_MASK)
                | ((value << Ch3TxLim::TX_LOOP_NUM_CH_OFFSET) & Ch3TxLim::TX_LOOP_NUM_CH_MASK),
        }
    }

    /// Set **TX_LIM_CH** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub const fn with_tx_lim_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3TxLim::TX_LIM_CH_MASK)
                | ((value << Ch3TxLim::TX_LIM_CH_OFFSET) & Ch3TxLim::TX_LIM_CH_MASK),
        }
    }
}

impl Default for Ch3TxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3TxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3TxLimVal {
        Ch3TxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3TxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3TxLimVal) -> Ch3TxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_RX_LIM Register
///
/// Channel 0 Rx event configuration register
///
/// Address offset: 0x00b0
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **RX_LIM_CH4** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can receive.
#[repr(transparent)]
pub struct Ch0RxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0RxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0RxLim {
    // ======== Field Constants ========

    /// **RX_LIM_CH4**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_LIM_CH4_OFFSET: u32 = 0;
    pub const RX_LIM_CH4_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn rx_lim_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LIM_CH4_MASK) >> Self::RX_LIM_CH4_OFFSET
    }

    /// Write **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn write_rx_lim_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LIM_CH4_MASK)
                | ((value << Self::RX_LIM_CH4_OFFSET) & Self::RX_LIM_CH4_MASK)
        });
    }
}

/// CH0_RX_LIM value type for batch read/modify/write operations
///
/// Use `ch0rxlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0rxlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0RxLimVal {
    bits: u32,
}

impl Ch0RxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn rx_lim_ch4(&self) -> u32 {
        (self.bits & Ch0RxLim::RX_LIM_CH4_MASK) >> Ch0RxLim::RX_LIM_CH4_OFFSET
    }

    /// Set **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn with_rx_lim_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0RxLim::RX_LIM_CH4_MASK)
                | ((value << Ch0RxLim::RX_LIM_CH4_OFFSET) & Ch0RxLim::RX_LIM_CH4_MASK),
        }
    }
}

impl Default for Ch0RxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0RxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0RxLimVal {
        Ch0RxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0RxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0RxLimVal) -> Ch0RxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_RX_LIM Register
///
/// Channel 1 Rx event configuration register
///
/// Address offset: 0x00b4
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **RX_LIM_CH4** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can receive.
#[repr(transparent)]
pub struct Ch1RxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1RxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1RxLim {
    // ======== Field Constants ========

    /// **RX_LIM_CH4**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_LIM_CH4_OFFSET: u32 = 0;
    pub const RX_LIM_CH4_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn rx_lim_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LIM_CH4_MASK) >> Self::RX_LIM_CH4_OFFSET
    }

    /// Write **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn write_rx_lim_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LIM_CH4_MASK)
                | ((value << Self::RX_LIM_CH4_OFFSET) & Self::RX_LIM_CH4_MASK)
        });
    }
}

/// CH1_RX_LIM value type for batch read/modify/write operations
///
/// Use `ch1rxlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1rxlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1RxLimVal {
    bits: u32,
}

impl Ch1RxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn rx_lim_ch4(&self) -> u32 {
        (self.bits & Ch1RxLim::RX_LIM_CH4_MASK) >> Ch1RxLim::RX_LIM_CH4_OFFSET
    }

    /// Set **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn with_rx_lim_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1RxLim::RX_LIM_CH4_MASK)
                | ((value << Ch1RxLim::RX_LIM_CH4_OFFSET) & Ch1RxLim::RX_LIM_CH4_MASK),
        }
    }
}

impl Default for Ch1RxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1RxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1RxLimVal {
        Ch1RxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1RxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1RxLimVal) -> Ch1RxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_RX_LIM Register
///
/// Channel 2 Rx event configuration register
///
/// Address offset: 0x00b8
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **RX_LIM_CH4** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can receive.
#[repr(transparent)]
pub struct Ch2RxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2RxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2RxLim {
    // ======== Field Constants ========

    /// **RX_LIM_CH4**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_LIM_CH4_OFFSET: u32 = 0;
    pub const RX_LIM_CH4_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn rx_lim_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LIM_CH4_MASK) >> Self::RX_LIM_CH4_OFFSET
    }

    /// Write **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn write_rx_lim_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LIM_CH4_MASK)
                | ((value << Self::RX_LIM_CH4_OFFSET) & Self::RX_LIM_CH4_MASK)
        });
    }
}

/// CH2_RX_LIM value type for batch read/modify/write operations
///
/// Use `ch2rxlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2rxlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2RxLimVal {
    bits: u32,
}

impl Ch2RxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn rx_lim_ch4(&self) -> u32 {
        (self.bits & Ch2RxLim::RX_LIM_CH4_MASK) >> Ch2RxLim::RX_LIM_CH4_OFFSET
    }

    /// Set **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn with_rx_lim_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2RxLim::RX_LIM_CH4_MASK)
                | ((value << Ch2RxLim::RX_LIM_CH4_OFFSET) & Ch2RxLim::RX_LIM_CH4_MASK),
        }
    }
}

impl Default for Ch2RxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2RxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2RxLimVal {
        Ch2RxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2RxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2RxLimVal) -> Ch2RxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_RX_LIM Register
///
/// Channel 3 Rx event configuration register
///
/// Address offset: 0x00bc
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **RX_LIM_CH4** `[8:0]` (RW)
///
///   This register is used to configure the maximum entries that CHANNEL%s can receive.
#[repr(transparent)]
pub struct Ch3RxLim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3RxLim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3RxLim {
    // ======== Field Constants ========

    /// **RX_LIM_CH4**
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_LIM_CH4_OFFSET: u32 = 0;
    pub const RX_LIM_CH4_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn rx_lim_ch4(&self) -> u32 {
        (self.value.read_bits() & Self::RX_LIM_CH4_MASK) >> Self::RX_LIM_CH4_OFFSET
    }

    /// Write **RX_LIM_CH4** `[8:0]` (RW)
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub fn write_rx_lim_ch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_LIM_CH4_MASK)
                | ((value << Self::RX_LIM_CH4_OFFSET) & Self::RX_LIM_CH4_MASK)
        });
    }
}

/// CH3_RX_LIM value type for batch read/modify/write operations
///
/// Use `ch3rxlim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3rxlim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3RxLimVal {
    bits: u32,
}

impl Ch3RxLimVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn rx_lim_ch4(&self) -> u32 {
        (self.bits & Ch3RxLim::RX_LIM_CH4_MASK) >> Ch3RxLim::RX_LIM_CH4_OFFSET
    }

    /// Set **RX_LIM_CH4** `[8:0]`
    ///
    /// This register is used to configure the maximum entries that CHANNEL%s can receive.
    #[inline(always)]
    pub const fn with_rx_lim_ch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3RxLim::RX_LIM_CH4_MASK)
                | ((value << Ch3RxLim::RX_LIM_CH4_OFFSET) & Ch3RxLim::RX_LIM_CH4_MASK),
        }
    }
}

impl Default for Ch3RxLimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3RxLim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3RxLimVal {
        Ch3RxLimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3RxLimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3RxLimVal) -> Ch3RxLimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYS_CONF Register
///
/// RMT apb configuration register
///
/// Address offset: 0x00c0
/// Reset value: 0x05000010
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers
///
/// - **SCLK_ACTIVE** `[26]` (RW)
///
///   rmt_sclk switch
///
/// - **SCLK_SEL** `[25:24]` (RW)
///
///   choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
///
/// - **SCLK_DIV_B** `[23:18]` (RW)
///
///   the denominator of the fractional part of the fractional divisor
///
/// - **SCLK_DIV_A** `[17:12]` (RW)
///
///   the numerator of the fractional part of the fractional divisor
///
/// - **SCLK_DIV_NUM** `[11:4]` (RW)
///
///   the integral part of the fractional divisor
///
/// - **MEM_FORCE_PU** `[3]` (RW)
///
///   1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode.
///
/// - **MEM_FORCE_PD** `[2]` (RW)
///
///   Set this bit to power down RMT memory.
///
/// - **MEM_CLK_FORCE_ON** `[1]` (RW)
///
///   Set this bit to enable the clock for RMT memory.
///
/// - **APB_FIFO_MASK** `[0]` (RW)
///
///   1'h1: access memory directly. 1'h0: access memory by FIFO.
#[repr(transparent)]
pub struct SysConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SysConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SysConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **SCLK_ACTIVE**
    ///
    /// rmt_sclk switch
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SCLK_ACTIVE_OFFSET: u32 = 26;
    pub const SCLK_ACTIVE_MASK: u32 = 0x04000000;
    pub const SCLK_ACTIVE: u32 = 1 << 26;

    /// **SCLK_SEL**
    ///
    /// choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const SCLK_SEL_OFFSET: u32 = 24;
    pub const SCLK_SEL_MASK: u32 = 0x03000000;

    /// **SCLK_DIV_B**
    ///
    /// the denominator of the fractional part of the fractional divisor
    ///
    /// Bits `[23:18]`, mask `0x00fc0000`
    pub const SCLK_DIV_B_OFFSET: u32 = 18;
    pub const SCLK_DIV_B_MASK: u32 = 0x00fc0000;

    /// **SCLK_DIV_A**
    ///
    /// the numerator of the fractional part of the fractional divisor
    ///
    /// Bits `[17:12]`, mask `0x0003f000`
    pub const SCLK_DIV_A_OFFSET: u32 = 12;
    pub const SCLK_DIV_A_MASK: u32 = 0x0003f000;

    /// **SCLK_DIV_NUM**
    ///
    /// the integral part of the fractional divisor
    ///
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const SCLK_DIV_NUM_OFFSET: u32 = 4;
    pub const SCLK_DIV_NUM_MASK: u32 = 0x00000ff0;

    /// **MEM_FORCE_PU**
    ///
    /// 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MEM_FORCE_PU_OFFSET: u32 = 3;
    pub const MEM_FORCE_PU_MASK: u32 = 0x00000008;
    pub const MEM_FORCE_PU: u32 = 1 << 3;

    /// **MEM_FORCE_PD**
    ///
    /// Set this bit to power down RMT memory.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MEM_FORCE_PD_OFFSET: u32 = 2;
    pub const MEM_FORCE_PD_MASK: u32 = 0x00000004;
    pub const MEM_FORCE_PD: u32 = 1 << 2;

    /// **MEM_CLK_FORCE_ON**
    ///
    /// Set this bit to enable the clock for RMT memory.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_CLK_FORCE_ON_OFFSET: u32 = 1;
    pub const MEM_CLK_FORCE_ON_MASK: u32 = 0x00000002;
    pub const MEM_CLK_FORCE_ON: u32 = 1 << 1;

    /// **APB_FIFO_MASK**
    ///
    /// 1'h1: access memory directly. 1'h0: access memory by FIFO.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const APB_FIFO_MASK_OFFSET: u32 = 0;
    pub const APB_FIFO_MASK_MASK: u32 = 0x00000001;
    pub const APB_FIFO_MASK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers
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

    /// **SCLK_ACTIVE** `[26]` (RW)
    ///
    /// rmt_sclk switch
    #[inline(always)]
    pub fn sclk_active(&self) -> bool {
        (self.value.read_bits() & Self::SCLK_ACTIVE_MASK) != 0
    }

    /// Set SCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sclk_active(&self) {
        self.value.modify_bits(|r| r | Self::SCLK_ACTIVE_MASK);
    }

    /// Clear SCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sclk_active(&self) {
        self.value.modify_bits(|r| r & !Self::SCLK_ACTIVE_MASK);
    }

    /// **SCLK_SEL** `[25:24]` (RW)
    ///
    /// choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
    #[inline(always)]
    pub fn sclk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_SEL_MASK) >> Self::SCLK_SEL_OFFSET
    }

    /// Write **SCLK_SEL** `[25:24]` (RW)
    ///
    /// choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
    #[inline(always)]
    pub fn write_sclk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_SEL_MASK) | ((value << Self::SCLK_SEL_OFFSET) & Self::SCLK_SEL_MASK)
        });
    }

    /// **SCLK_DIV_B** `[23:18]` (RW)
    ///
    /// the denominator of the fractional part of the fractional divisor
    #[inline(always)]
    pub fn sclk_div_b(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_B_MASK) >> Self::SCLK_DIV_B_OFFSET
    }

    /// Write **SCLK_DIV_B** `[23:18]` (RW)
    ///
    /// the denominator of the fractional part of the fractional divisor
    #[inline(always)]
    pub fn write_sclk_div_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_B_MASK)
                | ((value << Self::SCLK_DIV_B_OFFSET) & Self::SCLK_DIV_B_MASK)
        });
    }

    /// **SCLK_DIV_A** `[17:12]` (RW)
    ///
    /// the numerator of the fractional part of the fractional divisor
    #[inline(always)]
    pub fn sclk_div_a(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_A_MASK) >> Self::SCLK_DIV_A_OFFSET
    }

    /// Write **SCLK_DIV_A** `[17:12]` (RW)
    ///
    /// the numerator of the fractional part of the fractional divisor
    #[inline(always)]
    pub fn write_sclk_div_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_A_MASK)
                | ((value << Self::SCLK_DIV_A_OFFSET) & Self::SCLK_DIV_A_MASK)
        });
    }

    /// **SCLK_DIV_NUM** `[11:4]` (RW)
    ///
    /// the integral part of the fractional divisor
    #[inline(always)]
    pub fn sclk_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_NUM_MASK) >> Self::SCLK_DIV_NUM_OFFSET
    }

    /// Write **SCLK_DIV_NUM** `[11:4]` (RW)
    ///
    /// the integral part of the fractional divisor
    #[inline(always)]
    pub fn write_sclk_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_NUM_MASK)
                | ((value << Self::SCLK_DIV_NUM_OFFSET) & Self::SCLK_DIV_NUM_MASK)
        });
    }

    /// **MEM_FORCE_PU** `[3]` (RW)
    ///
    /// 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode.
    #[inline(always)]
    pub fn mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PU_MASK) != 0
    }

    /// Set MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PU_MASK);
    }

    /// Clear MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PU_MASK);
    }

    /// **MEM_FORCE_PD** `[2]` (RW)
    ///
    /// Set this bit to power down RMT memory.
    #[inline(always)]
    pub fn mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PD_MASK) != 0
    }

    /// Set MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PD_MASK);
    }

    /// Clear MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PD_MASK);
    }

    /// **MEM_CLK_FORCE_ON** `[1]` (RW)
    ///
    /// Set this bit to enable the clock for RMT memory.
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// **APB_FIFO_MASK** `[0]` (RW)
    ///
    /// 1'h1: access memory directly. 1'h0: access memory by FIFO.
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> bool {
        (self.value.read_bits() & Self::APB_FIFO_MASK_MASK) != 0
    }

    /// Set APB_FIFO_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_apb_fifo_mask(&self) {
        self.value.modify_bits(|r| r | Self::APB_FIFO_MASK_MASK);
    }

    /// Clear APB_FIFO_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_apb_fifo_mask(&self) {
        self.value.modify_bits(|r| r & !Self::APB_FIFO_MASK_MASK);
    }
}

/// SYS_CONF value type for batch read/modify/write operations
///
/// Use `sysconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysConfVal {
    bits: u32,
}

impl SysConfVal {
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
        Self { bits: 0x05000010 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & SysConf::CLK_EN_MASK) != 0
    }

    /// Read **SCLK_ACTIVE** `[26:26]`
    ///
    /// rmt_sclk switch
    #[inline(always)]
    pub const fn sclk_active(&self) -> bool {
        (self.bits & SysConf::SCLK_ACTIVE_MASK) != 0
    }

    /// Read **SCLK_SEL** `[25:24]`
    ///
    /// choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
    #[inline(always)]
    pub const fn sclk_sel(&self) -> u32 {
        (self.bits & SysConf::SCLK_SEL_MASK) >> SysConf::SCLK_SEL_OFFSET
    }

    /// Read **SCLK_DIV_B** `[23:18]`
    ///
    /// the denominator of the fractional part of the fractional divisor
    #[inline(always)]
    pub const fn sclk_div_b(&self) -> u32 {
        (self.bits & SysConf::SCLK_DIV_B_MASK) >> SysConf::SCLK_DIV_B_OFFSET
    }

    /// Read **SCLK_DIV_A** `[17:12]`
    ///
    /// the numerator of the fractional part of the fractional divisor
    #[inline(always)]
    pub const fn sclk_div_a(&self) -> u32 {
        (self.bits & SysConf::SCLK_DIV_A_MASK) >> SysConf::SCLK_DIV_A_OFFSET
    }

    /// Read **SCLK_DIV_NUM** `[11:4]`
    ///
    /// the integral part of the fractional divisor
    #[inline(always)]
    pub const fn sclk_div_num(&self) -> u32 {
        (self.bits & SysConf::SCLK_DIV_NUM_MASK) >> SysConf::SCLK_DIV_NUM_OFFSET
    }

    /// Read **MEM_FORCE_PU** `[3:3]`
    ///
    /// 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode.
    #[inline(always)]
    pub const fn mem_force_pu(&self) -> bool {
        (self.bits & SysConf::MEM_FORCE_PU_MASK) != 0
    }

    /// Read **MEM_FORCE_PD** `[2:2]`
    ///
    /// Set this bit to power down RMT memory.
    #[inline(always)]
    pub const fn mem_force_pd(&self) -> bool {
        (self.bits & SysConf::MEM_FORCE_PD_MASK) != 0
    }

    /// Read **MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit to enable the clock for RMT memory.
    #[inline(always)]
    pub const fn mem_clk_force_on(&self) -> bool {
        (self.bits & SysConf::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **APB_FIFO_MASK** `[0:0]`
    ///
    /// 1'h1: access memory directly. 1'h0: access memory by FIFO.
    #[inline(always)]
    pub const fn apb_fifo_mask(&self) -> bool {
        (self.bits & SysConf::APB_FIFO_MASK_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::CLK_EN_MASK
            } else {
                self.bits & !SysConf::CLK_EN_MASK
            },
        }
    }

    /// Set **SCLK_ACTIVE** `[26:26]`
    ///
    /// rmt_sclk switch
    #[inline(always)]
    pub const fn with_sclk_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::SCLK_ACTIVE_MASK
            } else {
                self.bits & !SysConf::SCLK_ACTIVE_MASK
            },
        }
    }

    /// Set **SCLK_SEL** `[25:24]`
    ///
    /// choose the clock source of rmt_sclk. 1:CLK_80Mhz.2:CLK_8MHz.3:XTAL
    #[inline(always)]
    pub const fn with_sclk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysConf::SCLK_SEL_MASK)
                | ((value << SysConf::SCLK_SEL_OFFSET) & SysConf::SCLK_SEL_MASK),
        }
    }

    /// Set **SCLK_DIV_B** `[23:18]`
    ///
    /// the denominator of the fractional part of the fractional divisor
    #[inline(always)]
    pub const fn with_sclk_div_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysConf::SCLK_DIV_B_MASK)
                | ((value << SysConf::SCLK_DIV_B_OFFSET) & SysConf::SCLK_DIV_B_MASK),
        }
    }

    /// Set **SCLK_DIV_A** `[17:12]`
    ///
    /// the numerator of the fractional part of the fractional divisor
    #[inline(always)]
    pub const fn with_sclk_div_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysConf::SCLK_DIV_A_MASK)
                | ((value << SysConf::SCLK_DIV_A_OFFSET) & SysConf::SCLK_DIV_A_MASK),
        }
    }

    /// Set **SCLK_DIV_NUM** `[11:4]`
    ///
    /// the integral part of the fractional divisor
    #[inline(always)]
    pub const fn with_sclk_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysConf::SCLK_DIV_NUM_MASK)
                | ((value << SysConf::SCLK_DIV_NUM_OFFSET) & SysConf::SCLK_DIV_NUM_MASK),
        }
    }

    /// Set **MEM_FORCE_PU** `[3:3]`
    ///
    /// 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode.
    #[inline(always)]
    pub const fn with_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::MEM_FORCE_PU_MASK
            } else {
                self.bits & !SysConf::MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PD** `[2:2]`
    ///
    /// Set this bit to power down RMT memory.
    #[inline(always)]
    pub const fn with_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::MEM_FORCE_PD_MASK
            } else {
                self.bits & !SysConf::MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit to enable the clock for RMT memory.
    #[inline(always)]
    pub const fn with_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !SysConf::MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **APB_FIFO_MASK** `[0:0]`
    ///
    /// 1'h1: access memory directly. 1'h0: access memory by FIFO.
    #[inline(always)]
    pub const fn with_apb_fifo_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysConf::APB_FIFO_MASK_MASK
            } else {
                self.bits & !SysConf::APB_FIFO_MASK_MASK
            },
        }
    }
}

impl Default for SysConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SysConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysConfVal {
        SysConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysConfVal) -> SysConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_SIM Register
///
/// RMT TX synchronous register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **EN** `[4]` (RW)
///
///   This register is used to enable multiple of channels to start sending data synchronously.
///
/// - **CH3** `[3]` (RW)
///
///   Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels.
///
/// - **CH2** `[2]` (RW)
///
///   Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels.
///
/// - **CH1** `[1]` (RW)
///
///   Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels.
///
/// - **CH0** `[0]` (RW)
///
///   Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels.
#[repr(transparent)]
pub struct TxSim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxSim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxSim {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// This register is used to enable multiple of channels to start sending data synchronously.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EN_OFFSET: u32 = 4;
    pub const EN_MASK: u32 = 0x00000010;
    pub const EN: u32 = 1 << 4;

    /// **CH3**
    ///
    /// Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    ///
    /// Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    ///
    /// Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    ///
    /// Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EN** `[4]` (RW)
    ///
    /// This register is used to enable multiple of channels to start sending data synchronously.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }

    /// **CH3** `[3]` (RW)
    ///
    /// Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.modify_bits(|r| r | Self::CH3_MASK);
    }

    /// Clear CH3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.modify_bits(|r| r & !Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    ///
    /// Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.modify_bits(|r| r | Self::CH2_MASK);
    }

    /// Clear CH2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.modify_bits(|r| r & !Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    ///
    /// Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.modify_bits(|r| r | Self::CH1_MASK);
    }

    /// Clear CH1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.modify_bits(|r| r & !Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    ///
    /// Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.modify_bits(|r| r | Self::CH0_MASK);
    }

    /// Clear CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::CH0_MASK);
    }
}

/// TX_SIM value type for batch read/modify/write operations
///
/// Use `txsim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txsim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxSimVal {
    bits: u32,
}

impl TxSimVal {
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

    /// Read **EN** `[4:4]`
    ///
    /// This register is used to enable multiple of channels to start sending data synchronously.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & TxSim::EN_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    ///
    /// Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & TxSim::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    ///
    /// Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & TxSim::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    ///
    /// Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & TxSim::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    ///
    /// Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & TxSim::CH0_MASK) != 0
    }

    /// Set **EN** `[4:4]`
    ///
    /// This register is used to enable multiple of channels to start sending data synchronously.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSim::EN_MASK
            } else {
                self.bits & !TxSim::EN_MASK
            },
        }
    }

    /// Set **CH3** `[3:3]`
    ///
    /// Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSim::CH3_MASK
            } else {
                self.bits & !TxSim::CH3_MASK
            },
        }
    }

    /// Set **CH2** `[2:2]`
    ///
    /// Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSim::CH2_MASK
            } else {
                self.bits & !TxSim::CH2_MASK
            },
        }
    }

    /// Set **CH1** `[1:1]`
    ///
    /// Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSim::CH1_MASK
            } else {
                self.bits & !TxSim::CH1_MASK
            },
        }
    }

    /// Set **CH0** `[0:0]`
    ///
    /// Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels.
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSim::CH0_MASK
            } else {
                self.bits & !TxSim::CH0_MASK
            },
        }
    }
}

impl Default for TxSimVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxSim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxSimVal {
        TxSimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxSimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxSimVal) -> TxSimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REF_CNT_RST Register
///
/// RMT clock divider reset register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **RX_REF_CNT_RST_CH7** `[7]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL7.
///
/// - **RX_REF_CNT_RST_CH6** `[6]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL6.
///
/// - **RX_REF_CNT_RST_CH5** `[5]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL5.
///
/// - **RX_REF_CNT_RST_CH4** `[4]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL4.
///
/// - **TX_REF_CNT_RST_CH3** `[3]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL3.
///
/// - **TX_REF_CNT_RST_CH2** `[2]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL2.
///
/// - **TX_REF_CNT_RST_CH1** `[1]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL1.
///
/// - **TX_REF_CNT_RST_CH0** `[0]` (WO)
///
///   This register is used to reset the clock divider of CHANNEL0.
#[repr(transparent)]
pub struct RefCntRst {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RefCntRst {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RefCntRst {
    // ======== Field Constants ========

    /// **RX_REF_CNT_RST_CH7**
    ///
    /// This register is used to reset the clock divider of CHANNEL7.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_REF_CNT_RST_CH7_OFFSET: u32 = 7;
    pub const RX_REF_CNT_RST_CH7_MASK: u32 = 0x00000080;
    pub const RX_REF_CNT_RST_CH7: u32 = 1 << 7;

    /// **RX_REF_CNT_RST_CH6**
    ///
    /// This register is used to reset the clock divider of CHANNEL6.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_REF_CNT_RST_CH6_OFFSET: u32 = 6;
    pub const RX_REF_CNT_RST_CH6_MASK: u32 = 0x00000040;
    pub const RX_REF_CNT_RST_CH6: u32 = 1 << 6;

    /// **RX_REF_CNT_RST_CH5**
    ///
    /// This register is used to reset the clock divider of CHANNEL5.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_REF_CNT_RST_CH5_OFFSET: u32 = 5;
    pub const RX_REF_CNT_RST_CH5_MASK: u32 = 0x00000020;
    pub const RX_REF_CNT_RST_CH5: u32 = 1 << 5;

    /// **RX_REF_CNT_RST_CH4**
    ///
    /// This register is used to reset the clock divider of CHANNEL4.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_REF_CNT_RST_CH4_OFFSET: u32 = 4;
    pub const RX_REF_CNT_RST_CH4_MASK: u32 = 0x00000010;
    pub const RX_REF_CNT_RST_CH4: u32 = 1 << 4;

    /// **TX_REF_CNT_RST_CH3**
    ///
    /// This register is used to reset the clock divider of CHANNEL3.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_REF_CNT_RST_CH3_OFFSET: u32 = 3;
    pub const TX_REF_CNT_RST_CH3_MASK: u32 = 0x00000008;
    pub const TX_REF_CNT_RST_CH3: u32 = 1 << 3;

    /// **TX_REF_CNT_RST_CH2**
    ///
    /// This register is used to reset the clock divider of CHANNEL2.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_REF_CNT_RST_CH2_OFFSET: u32 = 2;
    pub const TX_REF_CNT_RST_CH2_MASK: u32 = 0x00000004;
    pub const TX_REF_CNT_RST_CH2: u32 = 1 << 2;

    /// **TX_REF_CNT_RST_CH1**
    ///
    /// This register is used to reset the clock divider of CHANNEL1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_REF_CNT_RST_CH1_OFFSET: u32 = 1;
    pub const TX_REF_CNT_RST_CH1_MASK: u32 = 0x00000002;
    pub const TX_REF_CNT_RST_CH1: u32 = 1 << 1;

    /// **TX_REF_CNT_RST_CH0**
    ///
    /// This register is used to reset the clock divider of CHANNEL0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_REF_CNT_RST_CH0_OFFSET: u32 = 0;
    pub const TX_REF_CNT_RST_CH0_MASK: u32 = 0x00000001;
    pub const TX_REF_CNT_RST_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **RX_REF_CNT_RST_CH7** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL7.
    #[inline(always)]
    pub fn trigger_rx_ref_cnt_rst_ch7(&self) {
        self.value.write_bits(Self::RX_REF_CNT_RST_CH7_MASK);
    }

    /// Trigger **RX_REF_CNT_RST_CH6** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL6.
    #[inline(always)]
    pub fn trigger_rx_ref_cnt_rst_ch6(&self) {
        self.value.write_bits(Self::RX_REF_CNT_RST_CH6_MASK);
    }

    /// Trigger **RX_REF_CNT_RST_CH5** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL5.
    #[inline(always)]
    pub fn trigger_rx_ref_cnt_rst_ch5(&self) {
        self.value.write_bits(Self::RX_REF_CNT_RST_CH5_MASK);
    }

    /// Trigger **RX_REF_CNT_RST_CH4** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL4.
    #[inline(always)]
    pub fn trigger_rx_ref_cnt_rst_ch4(&self) {
        self.value.write_bits(Self::RX_REF_CNT_RST_CH4_MASK);
    }

    /// Trigger **TX_REF_CNT_RST_CH3** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL3.
    #[inline(always)]
    pub fn trigger_tx_ref_cnt_rst_ch3(&self) {
        self.value.write_bits(Self::TX_REF_CNT_RST_CH3_MASK);
    }

    /// Trigger **TX_REF_CNT_RST_CH2** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL2.
    #[inline(always)]
    pub fn trigger_tx_ref_cnt_rst_ch2(&self) {
        self.value.write_bits(Self::TX_REF_CNT_RST_CH2_MASK);
    }

    /// Trigger **TX_REF_CNT_RST_CH1** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL1.
    #[inline(always)]
    pub fn trigger_tx_ref_cnt_rst_ch1(&self) {
        self.value.write_bits(Self::TX_REF_CNT_RST_CH1_MASK);
    }

    /// Trigger **TX_REF_CNT_RST_CH0** (WO)
    ///
    /// This register is used to reset the clock divider of CHANNEL0.
    #[inline(always)]
    pub fn trigger_tx_ref_cnt_rst_ch0(&self) {
        self.value.write_bits(Self::TX_REF_CNT_RST_CH0_MASK);
    }
}

/// REF_CNT_RST value type for batch read/modify/write operations
///
/// Use `refcntrst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `refcntrst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RefCntRstVal {
    bits: u32,
}

impl RefCntRstVal {
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

    /// Set **RX_REF_CNT_RST_CH7** `[7:7]`
    ///
    /// This register is used to reset the clock divider of CHANNEL7.
    #[inline(always)]
    pub const fn with_rx_ref_cnt_rst_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::RX_REF_CNT_RST_CH7_MASK
            } else {
                self.bits & !RefCntRst::RX_REF_CNT_RST_CH7_MASK
            },
        }
    }

    /// Set **RX_REF_CNT_RST_CH6** `[6:6]`
    ///
    /// This register is used to reset the clock divider of CHANNEL6.
    #[inline(always)]
    pub const fn with_rx_ref_cnt_rst_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::RX_REF_CNT_RST_CH6_MASK
            } else {
                self.bits & !RefCntRst::RX_REF_CNT_RST_CH6_MASK
            },
        }
    }

    /// Set **RX_REF_CNT_RST_CH5** `[5:5]`
    ///
    /// This register is used to reset the clock divider of CHANNEL5.
    #[inline(always)]
    pub const fn with_rx_ref_cnt_rst_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::RX_REF_CNT_RST_CH5_MASK
            } else {
                self.bits & !RefCntRst::RX_REF_CNT_RST_CH5_MASK
            },
        }
    }

    /// Set **RX_REF_CNT_RST_CH4** `[4:4]`
    ///
    /// This register is used to reset the clock divider of CHANNEL4.
    #[inline(always)]
    pub const fn with_rx_ref_cnt_rst_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::RX_REF_CNT_RST_CH4_MASK
            } else {
                self.bits & !RefCntRst::RX_REF_CNT_RST_CH4_MASK
            },
        }
    }

    /// Set **TX_REF_CNT_RST_CH3** `[3:3]`
    ///
    /// This register is used to reset the clock divider of CHANNEL3.
    #[inline(always)]
    pub const fn with_tx_ref_cnt_rst_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::TX_REF_CNT_RST_CH3_MASK
            } else {
                self.bits & !RefCntRst::TX_REF_CNT_RST_CH3_MASK
            },
        }
    }

    /// Set **TX_REF_CNT_RST_CH2** `[2:2]`
    ///
    /// This register is used to reset the clock divider of CHANNEL2.
    #[inline(always)]
    pub const fn with_tx_ref_cnt_rst_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::TX_REF_CNT_RST_CH2_MASK
            } else {
                self.bits & !RefCntRst::TX_REF_CNT_RST_CH2_MASK
            },
        }
    }

    /// Set **TX_REF_CNT_RST_CH1** `[1:1]`
    ///
    /// This register is used to reset the clock divider of CHANNEL1.
    #[inline(always)]
    pub const fn with_tx_ref_cnt_rst_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::TX_REF_CNT_RST_CH1_MASK
            } else {
                self.bits & !RefCntRst::TX_REF_CNT_RST_CH1_MASK
            },
        }
    }

    /// Set **TX_REF_CNT_RST_CH0** `[0:0]`
    ///
    /// This register is used to reset the clock divider of CHANNEL0.
    #[inline(always)]
    pub const fn with_tx_ref_cnt_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RefCntRst::TX_REF_CNT_RST_CH0_MASK
            } else {
                self.bits & !RefCntRst::TX_REF_CNT_RST_CH0_MASK
            },
        }
    }
}

impl Default for RefCntRstVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RefCntRst {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RefCntRstVal) {
        self.value.write_bits(val.bits())
    }
}

/// DATE Register
///
/// RMT version register
///
/// Address offset: 0x00cc
/// Reset value: 0x02201111
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   This is the version register.
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
    /// This is the version register.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// This is the version register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// This is the version register.
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
        Self { bits: 0x02201111 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// This is the version register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// This is the version register.
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
    /// 0x0000
    /// The read and write data register for CHANNEL0 by apb fifo access.
    pub tx_ch0data: TxCh0data,
    /// 0x0004
    /// The read and write data register for CHANNEL1 by apb fifo access.
    pub tx_ch1data: TxCh1data,
    /// 0x0008
    /// The read and write data register for CHANNEL2 by apb fifo access.
    pub tx_ch2data: TxCh2data,
    /// 0x000c
    /// The read and write data register for CHANNEL3 by apb fifo access.
    pub tx_ch3data: TxCh3data,
    /// 0x0010
    /// The read and write data register for CHANNEL$n by apb fifo access.
    pub rx_ch0data: RxCh0data,
    /// 0x0014
    /// The read and write data register for CHANNEL$n by apb fifo access.
    pub rx_ch1data: RxCh1data,
    /// 0x0018
    /// The read and write data register for CHANNEL$n by apb fifo access.
    pub rx_ch2data: RxCh2data,
    /// 0x001c
    /// The read and write data register for CHANNEL$n by apb fifo access.
    pub rx_ch3data: RxCh3data,
    /// 0x0020
    /// Channel 0 configure register 0
    pub tx_ch0conf0: TxCh0conf0,
    /// 0x0024
    /// Channel 1 configure register 0
    pub tx_ch1conf0: TxCh1conf0,
    /// 0x0028
    /// Channel 2 configure register 0
    pub tx_ch2conf0: TxCh2conf0,
    /// 0x002c
    /// Channel 3 configure register 0
    pub tx_ch3conf0: TxCh3conf0,
    /// 0x0030
    /// Channel 0 configure register 0
    pub rx_ch0conf0: RxCh0conf0,
    /// 0x0034
    /// Channel 0 configure register 1
    pub rx_ch0conf1: RxCh0conf1,
    /// 0x0038
    /// Channel 1 configure register 0
    pub rx_ch1conf0: RxCh1conf0,
    /// 0x003c
    /// Channel 1 configure register 1
    pub rx_ch1conf1: RxCh1conf1,
    /// 0x0040
    /// Channel 2 configure register 0
    pub rx_ch2conf0: RxCh2conf0,
    /// 0x0044
    /// Channel 2 configure register 1
    pub rx_ch2conf1: RxCh2conf1,
    /// 0x0048
    /// Channel 3 configure register 0
    pub rx_ch3conf0: RxCh3conf0,
    /// 0x004c
    /// Channel 3 configure register 1
    pub rx_ch3conf1: RxCh3conf1,
    /// 0x0050
    /// Channel 0 status register
    pub tx_ch0status: TxCh0status,
    /// 0x0054
    /// Channel 1 status register
    pub tx_ch1status: TxCh1status,
    /// 0x0058
    /// Channel 2 status register
    pub tx_ch2status: TxCh2status,
    /// 0x005c
    /// Channel 3 status register
    pub tx_ch3status: TxCh3status,
    /// 0x0060
    /// Channel 0 status register
    pub rx_ch0status: RxCh0status,
    /// 0x0064
    /// Channel 1 status register
    pub rx_ch1status: RxCh1status,
    /// 0x0068
    /// Channel 2 status register
    pub rx_ch2status: RxCh2status,
    /// 0x006c
    /// Channel 3 status register
    pub rx_ch3status: RxCh3status,
    /// 0x0070
    /// Raw interrupt status
    pub int_raw: IntRaw,
    /// 0x0074
    /// Masked interrupt status
    pub int_st: IntSt,
    /// 0x0078
    /// Interrupt enable bits
    pub int_ena: IntEna,
    /// 0x007c
    /// Interrupt clear bits
    pub int_clr: IntClr,
    /// 0x0080
    /// Channel 0 duty cycle configuration register
    pub ch0carrier_duty: Ch0carrierDuty,
    /// 0x0084
    /// Channel 1 duty cycle configuration register
    pub ch1carrier_duty: Ch1carrierDuty,
    /// 0x0088
    /// Channel 2 duty cycle configuration register
    pub ch2carrier_duty: Ch2carrierDuty,
    /// 0x008c
    /// Channel 3 duty cycle configuration register
    pub ch3carrier_duty: Ch3carrierDuty,
    /// 0x0090
    /// Channel 0 carrier remove register
    pub ch0_rx_carrier_rm: Ch0RxCarrierRm,
    /// 0x0094
    /// Channel 1 carrier remove register
    pub ch1_rx_carrier_rm: Ch1RxCarrierRm,
    /// 0x0098
    /// Channel 2 carrier remove register
    pub ch2_rx_carrier_rm: Ch2RxCarrierRm,
    /// 0x009c
    /// Channel 3 carrier remove register
    pub ch3_rx_carrier_rm: Ch3RxCarrierRm,
    /// 0x00a0
    /// Channel 0 Tx event configuration register
    pub ch0_tx_lim: Ch0TxLim,
    /// 0x00a4
    /// Channel 1 Tx event configuration register
    pub ch1_tx_lim: Ch1TxLim,
    /// 0x00a8
    /// Channel 2 Tx event configuration register
    pub ch2_tx_lim: Ch2TxLim,
    /// 0x00ac
    /// Channel 3 Tx event configuration register
    pub ch3_tx_lim: Ch3TxLim,
    /// 0x00b0
    /// Channel 0 Rx event configuration register
    pub ch0_rx_lim: Ch0RxLim,
    /// 0x00b4
    /// Channel 1 Rx event configuration register
    pub ch1_rx_lim: Ch1RxLim,
    /// 0x00b8
    /// Channel 2 Rx event configuration register
    pub ch2_rx_lim: Ch2RxLim,
    /// 0x00bc
    /// Channel 3 Rx event configuration register
    pub ch3_rx_lim: Ch3RxLim,
    /// 0x00c0
    /// RMT apb configuration register
    pub sys_conf: SysConf,
    /// 0x00c4
    /// RMT TX synchronous register
    pub tx_sim: TxSim,
    /// 0x00c8
    /// RMT clock divider reset register
    pub ref_cnt_rst: RefCntRst,
    /// 0x00cc
    /// RMT version register
    pub date: Date,
}
