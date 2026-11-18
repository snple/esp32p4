//! LP_I2S0
//! Low-power I2S (Inter-IC Sound) Controller 0

#![allow(dead_code)]

pub const BASE: u32 = 0x50125000;

/// VAD_CONF Register
///
/// I2S VAD Configure register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **VAD_FORCE_START** `[2]` (WO)
///
///   VAD force start register.
///
/// - **VAD_RESET** `[1]` (WO)
///
///   VAD reset register
///
/// - **VAD_EN** `[0]` (RW)
///
///   VAD enable register
#[repr(transparent)]
pub struct VadConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadConf {
    // ======== Field Constants ========

    /// **VAD_FORCE_START**
    ///
    /// VAD force start register.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const VAD_FORCE_START_OFFSET: u32 = 2;
    pub const VAD_FORCE_START_MASK: u32 = 0x00000004;
    pub const VAD_FORCE_START: u32 = 1 << 2;

    /// **VAD_RESET**
    ///
    /// VAD reset register
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VAD_RESET_OFFSET: u32 = 1;
    pub const VAD_RESET_MASK: u32 = 0x00000002;
    pub const VAD_RESET: u32 = 1 << 1;

    /// **VAD_EN**
    ///
    /// VAD enable register
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VAD_EN_OFFSET: u32 = 0;
    pub const VAD_EN_MASK: u32 = 0x00000001;
    pub const VAD_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **VAD_FORCE_START** (WO)
    ///
    /// VAD force start register.
    #[inline(always)]
    pub fn trigger_vad_force_start(&self) {
        self.value.modify_bits(|r| r | Self::VAD_FORCE_START_MASK);
    }

    /// Trigger **VAD_RESET** (WO)
    ///
    /// VAD reset register
    #[inline(always)]
    pub fn trigger_vad_reset(&self) {
        self.value.modify_bits(|r| r | Self::VAD_RESET_MASK);
    }

    /// **VAD_EN** `[0]` (RW)
    ///
    /// VAD enable register
    #[inline(always)]
    pub fn vad_en(&self) -> bool {
        (self.value.read_bits() & Self::VAD_EN_MASK) != 0
    }

    /// Set VAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vad_en(&self) {
        self.value.modify_bits(|r| r | Self::VAD_EN_MASK);
    }

    /// Clear VAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vad_en(&self) {
        self.value.modify_bits(|r| r & !Self::VAD_EN_MASK);
    }
}

/// VAD_CONF value type for batch read/modify/write operations
///
/// Use `vadconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadConfVal {
    bits: u32,
}

impl VadConfVal {
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

    /// Read **VAD_EN** `[0:0]`
    ///
    /// VAD enable register
    #[inline(always)]
    pub const fn vad_en(&self) -> bool {
        (self.bits & VadConf::VAD_EN_MASK) != 0
    }

    /// Set **VAD_FORCE_START** `[2:2]`
    ///
    /// VAD force start register.
    #[inline(always)]
    pub const fn with_vad_force_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VadConf::VAD_FORCE_START_MASK
            } else {
                self.bits & !VadConf::VAD_FORCE_START_MASK
            },
        }
    }

    /// Set **VAD_RESET** `[1:1]`
    ///
    /// VAD reset register
    #[inline(always)]
    pub const fn with_vad_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VadConf::VAD_RESET_MASK
            } else {
                self.bits & !VadConf::VAD_RESET_MASK
            },
        }
    }

    /// Set **VAD_EN** `[0:0]`
    ///
    /// VAD enable register
    #[inline(always)]
    pub const fn with_vad_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VadConf::VAD_EN_MASK
            } else {
                self.bits & !VadConf::VAD_EN_MASK
            },
        }
    }
}

impl Default for VadConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadConfVal {
        VadConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadConfVal) -> VadConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_RESULT Register
///
/// I2S VAD Result register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **ENERGY_ENOUGH** `[1]` (RO)
///
///   Reg energy enough observe signal
///
/// - **VAD_FLAG** `[0]` (RO)
///
///   Reg vad flag observe signal
#[repr(transparent)]
pub struct VadResult {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadResult {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadResult {
    // ======== Field Constants ========

    /// **ENERGY_ENOUGH**
    ///
    /// Reg energy enough observe signal
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ENERGY_ENOUGH_OFFSET: u32 = 1;
    pub const ENERGY_ENOUGH_MASK: u32 = 0x00000002;
    pub const ENERGY_ENOUGH: u32 = 1 << 1;

    /// **VAD_FLAG**
    ///
    /// Reg vad flag observe signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VAD_FLAG_OFFSET: u32 = 0;
    pub const VAD_FLAG_MASK: u32 = 0x00000001;
    pub const VAD_FLAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ENERGY_ENOUGH** `[1]` (RO)
    ///
    /// Reg energy enough observe signal
    #[inline(always)]
    pub fn energy_enough(&self) -> bool {
        (self.value.read_bits() & Self::ENERGY_ENOUGH_MASK) != 0
    }

    /// **VAD_FLAG** `[0]` (RO)
    ///
    /// Reg vad flag observe signal
    #[inline(always)]
    pub fn vad_flag(&self) -> bool {
        (self.value.read_bits() & Self::VAD_FLAG_MASK) != 0
    }
}

/// VAD_RESULT value type for batch read/modify/write operations
///
/// Use `vadresult.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadresult.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadResultVal {
    bits: u32,
}

impl VadResultVal {
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

    /// Read **ENERGY_ENOUGH** `[1:1]`
    ///
    /// Reg energy enough observe signal
    #[inline(always)]
    pub const fn energy_enough(&self) -> bool {
        (self.bits & VadResult::ENERGY_ENOUGH_MASK) != 0
    }

    /// Read **VAD_FLAG** `[0:0]`
    ///
    /// Reg vad flag observe signal
    #[inline(always)]
    pub const fn vad_flag(&self) -> bool {
        (self.bits & VadResult::VAD_FLAG_MASK) != 0
    }
}

impl Default for VadResultVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadResult {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadResultVal {
        VadResultVal::from_bits(self.value.read_bits())
    }
}

/// RX_MEM_CONF Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x0008
/// Reset value: 0x00007e00
///
/// ## Fields
///
/// - **RX_MEM_THRESHOLD** `[16:9]` (RW)
///
///   I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
///
/// - **RX_MEM_FIFO_CNT** `[8:0]` (RO)
///
///   The number of data in the rx mem
#[repr(transparent)]
pub struct RxMemConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxMemConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxMemConf {
    // ======== Field Constants ========

    /// **RX_MEM_THRESHOLD**
    ///
    /// I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
    ///
    /// Bits `[16:9]`, mask `0x0001fe00`
    pub const RX_MEM_THRESHOLD_OFFSET: u32 = 9;
    pub const RX_MEM_THRESHOLD_MASK: u32 = 0x0001fe00;

    /// **RX_MEM_FIFO_CNT**
    ///
    /// The number of data in the rx mem
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RX_MEM_FIFO_CNT_OFFSET: u32 = 0;
    pub const RX_MEM_FIFO_CNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RX_MEM_THRESHOLD** `[16:9]` (RW)
    ///
    /// I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> u32 {
        (self.value.read_bits() & Self::RX_MEM_THRESHOLD_MASK) >> Self::RX_MEM_THRESHOLD_OFFSET
    }

    /// Write **RX_MEM_THRESHOLD** `[16:9]` (RW)
    ///
    /// I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
    #[inline(always)]
    pub fn write_rx_mem_threshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_MEM_THRESHOLD_MASK)
                | ((value << Self::RX_MEM_THRESHOLD_OFFSET) & Self::RX_MEM_THRESHOLD_MASK)
        });
    }

    /// **RX_MEM_FIFO_CNT** `[8:0]` (RO)
    ///
    /// The number of data in the rx mem
    #[inline(always)]
    pub fn rx_mem_fifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_MEM_FIFO_CNT_MASK) >> Self::RX_MEM_FIFO_CNT_OFFSET
    }
}

/// RX_MEM_CONF value type for batch read/modify/write operations
///
/// Use `rxmemconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxmemconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxMemConfVal {
    bits: u32,
}

impl RxMemConfVal {
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
        Self { bits: 0x00007e00 }
    }

    /// Read **RX_MEM_THRESHOLD** `[16:9]`
    ///
    /// I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
    #[inline(always)]
    pub const fn rx_mem_threshold(&self) -> u32 {
        (self.bits & RxMemConf::RX_MEM_THRESHOLD_MASK) >> RxMemConf::RX_MEM_THRESHOLD_OFFSET
    }

    /// Read **RX_MEM_FIFO_CNT** `[8:0]`
    ///
    /// The number of data in the rx mem
    #[inline(always)]
    pub const fn rx_mem_fifo_cnt(&self) -> u32 {
        (self.bits & RxMemConf::RX_MEM_FIFO_CNT_MASK) >> RxMemConf::RX_MEM_FIFO_CNT_OFFSET
    }

    /// Set **RX_MEM_THRESHOLD** `[16:9]`
    ///
    /// I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold
    #[inline(always)]
    pub const fn with_rx_mem_threshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxMemConf::RX_MEM_THRESHOLD_MASK)
                | ((value << RxMemConf::RX_MEM_THRESHOLD_OFFSET)
                    & RxMemConf::RX_MEM_THRESHOLD_MASK),
        }
    }
}

impl Default for RxMemConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxMemConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxMemConfVal {
        RxMemConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxMemConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxMemConfVal) -> RxMemConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// I2S interrupt raw register, valid in level.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **RX_MEM_THRESHOLD_INT_RAW** `[5]` (RO)
///
///   The raw interrupt status bit for the rx_mem_threshold_int interrupt
///
/// - **VAD_RESET_DONE_INT_RAW** `[4]` (RO)
///
///   The raw interrupt status bit for the vad_reset_done_int interrupt
///
/// - **VAD_DONE_INT_RAW** `[3]` (RO)
///
///   The raw interrupt status bit for the vad_done_int interrupt
///
/// - **RX_FIFOMEM_UDF_INT_RAW** `[2]` (RO)
///
///   The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
///
/// - **RX_HUNG_INT_RAW** `[1]` (RO)
///
///   The raw interrupt status bit for the i2s_rx_hung_int interrupt
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

    /// **RX_MEM_THRESHOLD_INT_RAW**
    ///
    /// The raw interrupt status bit for the rx_mem_threshold_int interrupt
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_MEM_THRESHOLD_INT_RAW_OFFSET: u32 = 5;
    pub const RX_MEM_THRESHOLD_INT_RAW_MASK: u32 = 0x00000020;
    pub const RX_MEM_THRESHOLD_INT_RAW: u32 = 1 << 5;

    /// **VAD_RESET_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the vad_reset_done_int interrupt
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const VAD_RESET_DONE_INT_RAW_OFFSET: u32 = 4;
    pub const VAD_RESET_DONE_INT_RAW_MASK: u32 = 0x00000010;
    pub const VAD_RESET_DONE_INT_RAW: u32 = 1 << 4;

    /// **VAD_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the vad_done_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const VAD_DONE_INT_RAW_OFFSET: u32 = 3;
    pub const VAD_DONE_INT_RAW_MASK: u32 = 0x00000008;
    pub const VAD_DONE_INT_RAW: u32 = 1 << 3;

    /// **RX_FIFOMEM_UDF_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_FIFOMEM_UDF_INT_RAW_OFFSET: u32 = 2;
    pub const RX_FIFOMEM_UDF_INT_RAW_MASK: u32 = 0x00000004;
    pub const RX_FIFOMEM_UDF_INT_RAW: u32 = 1 << 2;

    /// **RX_HUNG_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_HUNG_INT_RAW_OFFSET: u32 = 1;
    pub const RX_HUNG_INT_RAW_MASK: u32 = 0x00000002;
    pub const RX_HUNG_INT_RAW: u32 = 1 << 1;

    /// **RX_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const RX_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_MEM_THRESHOLD_INT_RAW** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub fn rx_mem_threshold_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_MEM_THRESHOLD_INT_RAW_MASK) != 0
    }

    /// **VAD_RESET_DONE_INT_RAW** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub fn vad_reset_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::VAD_RESET_DONE_INT_RAW_MASK) != 0
    }

    /// **VAD_DONE_INT_RAW** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the vad_done_int interrupt
    #[inline(always)]
    pub fn vad_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::VAD_DONE_INT_RAW_MASK) != 0
    }

    /// **RX_FIFOMEM_UDF_INT_RAW** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub fn rx_fifomem_udf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFOMEM_UDF_INT_RAW_MASK) != 0
    }

    /// **RX_HUNG_INT_RAW** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_RAW_MASK) != 0
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

    /// Read **RX_MEM_THRESHOLD_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt status bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub const fn rx_mem_threshold_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_MEM_THRESHOLD_INT_RAW_MASK) != 0
    }

    /// Read **VAD_RESET_DONE_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt status bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub const fn vad_reset_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::VAD_RESET_DONE_INT_RAW_MASK) != 0
    }

    /// Read **VAD_DONE_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt status bit for the vad_done_int interrupt
    #[inline(always)]
    pub const fn vad_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::VAD_DONE_INT_RAW_MASK) != 0
    }

    /// Read **RX_FIFOMEM_UDF_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub const fn rx_fifomem_udf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_FIFOMEM_UDF_INT_RAW_MASK) != 0
    }

    /// Read **RX_HUNG_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_HUNG_INT_RAW_MASK) != 0
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
/// - **RX_MEM_THRESHOLD_INT_ST** `[5]` (RO)
///
///   The masked interrupt status bit for the rx_mem_threshold_int interrupt
///
/// - **LP_VAD_RESET_DONE_INT_ST** `[4]` (RO)
///
///   The masked interrupt status bit for the vad_reset_done_int interrupt
///
/// - **LP_VAD_DONE_INT_ST** `[3]` (RO)
///
///   The masked interrupt status bit for the vad_done_int interrupt
///
/// - **RX_FIFOMEM_UDF_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
///
/// - **RX_HUNG_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for the i2s_rx_hung_int interrupt
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

    /// **RX_MEM_THRESHOLD_INT_ST**
    ///
    /// The masked interrupt status bit for the rx_mem_threshold_int interrupt
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_MEM_THRESHOLD_INT_ST_OFFSET: u32 = 5;
    pub const RX_MEM_THRESHOLD_INT_ST_MASK: u32 = 0x00000020;
    pub const RX_MEM_THRESHOLD_INT_ST: u32 = 1 << 5;

    /// **LP_VAD_RESET_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the vad_reset_done_int interrupt
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_VAD_RESET_DONE_INT_ST_OFFSET: u32 = 4;
    pub const LP_VAD_RESET_DONE_INT_ST_MASK: u32 = 0x00000010;
    pub const LP_VAD_RESET_DONE_INT_ST: u32 = 1 << 4;

    /// **LP_VAD_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the vad_done_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_VAD_DONE_INT_ST_OFFSET: u32 = 3;
    pub const LP_VAD_DONE_INT_ST_MASK: u32 = 0x00000008;
    pub const LP_VAD_DONE_INT_ST: u32 = 1 << 3;

    /// **RX_FIFOMEM_UDF_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_FIFOMEM_UDF_INT_ST_OFFSET: u32 = 2;
    pub const RX_FIFOMEM_UDF_INT_ST_MASK: u32 = 0x00000004;
    pub const RX_FIFOMEM_UDF_INT_ST: u32 = 1 << 2;

    /// **RX_HUNG_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_HUNG_INT_ST_OFFSET: u32 = 1;
    pub const RX_HUNG_INT_ST_MASK: u32 = 0x00000002;
    pub const RX_HUNG_INT_ST: u32 = 1 << 1;

    /// **RX_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_ST_OFFSET: u32 = 0;
    pub const RX_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_MEM_THRESHOLD_INT_ST** `[5]` (RO)
    ///
    /// The masked interrupt status bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub fn rx_mem_threshold_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_MEM_THRESHOLD_INT_ST_MASK) != 0
    }

    /// **LP_VAD_RESET_DONE_INT_ST** `[4]` (RO)
    ///
    /// The masked interrupt status bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub fn lp_vad_reset_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_VAD_RESET_DONE_INT_ST_MASK) != 0
    }

    /// **LP_VAD_DONE_INT_ST** `[3]` (RO)
    ///
    /// The masked interrupt status bit for the vad_done_int interrupt
    #[inline(always)]
    pub fn lp_vad_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_VAD_DONE_INT_ST_MASK) != 0
    }

    /// **RX_FIFOMEM_UDF_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub fn rx_fifomem_udf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFOMEM_UDF_INT_ST_MASK) != 0
    }

    /// **RX_HUNG_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_ST_MASK) != 0
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

    /// Read **RX_MEM_THRESHOLD_INT_ST** `[5:5]`
    ///
    /// The masked interrupt status bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub const fn rx_mem_threshold_int_st(&self) -> bool {
        (self.bits & IntSt::RX_MEM_THRESHOLD_INT_ST_MASK) != 0
    }

    /// Read **LP_VAD_RESET_DONE_INT_ST** `[4:4]`
    ///
    /// The masked interrupt status bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub const fn lp_vad_reset_done_int_st(&self) -> bool {
        (self.bits & IntSt::LP_VAD_RESET_DONE_INT_ST_MASK) != 0
    }

    /// Read **LP_VAD_DONE_INT_ST** `[3:3]`
    ///
    /// The masked interrupt status bit for the vad_done_int interrupt
    #[inline(always)]
    pub const fn lp_vad_done_int_st(&self) -> bool {
        (self.bits & IntSt::LP_VAD_DONE_INT_ST_MASK) != 0
    }

    /// Read **RX_FIFOMEM_UDF_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub const fn rx_fifomem_udf_int_st(&self) -> bool {
        (self.bits & IntSt::RX_FIFOMEM_UDF_INT_ST_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_st(&self) -> bool {
        (self.bits & IntSt::RX_HUNG_INT_ST_MASK) != 0
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
/// - **RX_MEM_THRESHOLD_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the rx_mem_threshold_int interrupt
///
/// - **LP_VAD_RESET_DONE_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the vad_reset_done_int interrupt
///
/// - **LP_VAD_DONE_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the vad_done_int interrupt
///
/// - **RX_FIFOMEM_UDF_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt
///
/// - **RX_HUNG_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the i2s_rx_hung_int interrupt
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

    /// **RX_MEM_THRESHOLD_INT_ENA**
    ///
    /// The interrupt enable bit for the rx_mem_threshold_int interrupt
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_MEM_THRESHOLD_INT_ENA_OFFSET: u32 = 5;
    pub const RX_MEM_THRESHOLD_INT_ENA_MASK: u32 = 0x00000020;
    pub const RX_MEM_THRESHOLD_INT_ENA: u32 = 1 << 5;

    /// **LP_VAD_RESET_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the vad_reset_done_int interrupt
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_VAD_RESET_DONE_INT_ENA_OFFSET: u32 = 4;
    pub const LP_VAD_RESET_DONE_INT_ENA_MASK: u32 = 0x00000010;
    pub const LP_VAD_RESET_DONE_INT_ENA: u32 = 1 << 4;

    /// **LP_VAD_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the vad_done_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_VAD_DONE_INT_ENA_OFFSET: u32 = 3;
    pub const LP_VAD_DONE_INT_ENA_MASK: u32 = 0x00000008;
    pub const LP_VAD_DONE_INT_ENA: u32 = 1 << 3;

    /// **RX_FIFOMEM_UDF_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_FIFOMEM_UDF_INT_ENA_OFFSET: u32 = 2;
    pub const RX_FIFOMEM_UDF_INT_ENA_MASK: u32 = 0x00000004;
    pub const RX_FIFOMEM_UDF_INT_ENA: u32 = 1 << 2;

    /// **RX_HUNG_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_HUNG_INT_ENA_OFFSET: u32 = 1;
    pub const RX_HUNG_INT_ENA_MASK: u32 = 0x00000002;
    pub const RX_HUNG_INT_ENA: u32 = 1 << 1;

    /// **RX_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const RX_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_MEM_THRESHOLD_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub fn rx_mem_threshold_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_MEM_THRESHOLD_INT_ENA_MASK) != 0
    }

    /// Set RX_MEM_THRESHOLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_mem_threshold_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_MEM_THRESHOLD_INT_ENA_MASK);
    }

    /// Clear RX_MEM_THRESHOLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_mem_threshold_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_MEM_THRESHOLD_INT_ENA_MASK);
    }

    /// **LP_VAD_RESET_DONE_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub fn lp_vad_reset_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_VAD_RESET_DONE_INT_ENA_MASK) != 0
    }

    /// Set LP_VAD_RESET_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vad_reset_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_VAD_RESET_DONE_INT_ENA_MASK);
    }

    /// Clear LP_VAD_RESET_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vad_reset_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_VAD_RESET_DONE_INT_ENA_MASK);
    }

    /// **LP_VAD_DONE_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the vad_done_int interrupt
    #[inline(always)]
    pub fn lp_vad_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_VAD_DONE_INT_ENA_MASK) != 0
    }

    /// Set LP_VAD_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vad_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_VAD_DONE_INT_ENA_MASK);
    }

    /// Clear LP_VAD_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vad_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_VAD_DONE_INT_ENA_MASK);
    }

    /// **RX_FIFOMEM_UDF_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub fn rx_fifomem_udf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFOMEM_UDF_INT_ENA_MASK) != 0
    }

    /// Set RX_FIFOMEM_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fifomem_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_FIFOMEM_UDF_INT_ENA_MASK);
    }

    /// Clear RX_FIFOMEM_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fifomem_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_FIFOMEM_UDF_INT_ENA_MASK);
    }

    /// **RX_HUNG_INT_ENA** `[1]` (RW)
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

    /// Read **RX_MEM_THRESHOLD_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub const fn rx_mem_threshold_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_MEM_THRESHOLD_INT_ENA_MASK) != 0
    }

    /// Read **LP_VAD_RESET_DONE_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub const fn lp_vad_reset_done_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_VAD_RESET_DONE_INT_ENA_MASK) != 0
    }

    /// Read **LP_VAD_DONE_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the vad_done_int interrupt
    #[inline(always)]
    pub const fn lp_vad_done_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_VAD_DONE_INT_ENA_MASK) != 0
    }

    /// Read **RX_FIFOMEM_UDF_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub const fn rx_fifomem_udf_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_FIFOMEM_UDF_INT_ENA_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub const fn rx_hung_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_HUNG_INT_ENA_MASK) != 0
    }

    /// Read **RX_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub const fn rx_done_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_DONE_INT_ENA_MASK) != 0
    }

    /// Set **RX_MEM_THRESHOLD_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub const fn with_rx_mem_threshold_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_MEM_THRESHOLD_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_MEM_THRESHOLD_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_VAD_RESET_DONE_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the vad_reset_done_int interrupt
    #[inline(always)]
    pub const fn with_lp_vad_reset_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_VAD_RESET_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_VAD_RESET_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_VAD_DONE_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the vad_done_int interrupt
    #[inline(always)]
    pub const fn with_lp_vad_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_VAD_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_VAD_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_FIFOMEM_UDF_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub const fn with_rx_fifomem_udf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_FIFOMEM_UDF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_FIFOMEM_UDF_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_ENA** `[1:1]`
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
/// - **RX_MEM_THRESHOLD_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the rx_mem_threshold_int interrupt
///
/// - **LP_VAD_RESET_DONE_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the vad_reset_done_int interrupt
///
/// - **LP_VAD_DONE_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the vad_done_int interrupt
///
/// - **RX_FIFOMEM_UDF_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the i2s_rx_fifomem_udf_int interrupt
///
/// - **RX_HUNG_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the i2s_rx_hung_int interrupt
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

    /// **RX_MEM_THRESHOLD_INT_CLR**
    ///
    /// Set this bit to clear the rx_mem_threshold_int interrupt
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_MEM_THRESHOLD_INT_CLR_OFFSET: u32 = 5;
    pub const RX_MEM_THRESHOLD_INT_CLR_MASK: u32 = 0x00000020;
    pub const RX_MEM_THRESHOLD_INT_CLR: u32 = 1 << 5;

    /// **LP_VAD_RESET_DONE_INT_CLR**
    ///
    /// Set this bit to clear the vad_reset_done_int interrupt
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_VAD_RESET_DONE_INT_CLR_OFFSET: u32 = 4;
    pub const LP_VAD_RESET_DONE_INT_CLR_MASK: u32 = 0x00000010;
    pub const LP_VAD_RESET_DONE_INT_CLR: u32 = 1 << 4;

    /// **LP_VAD_DONE_INT_CLR**
    ///
    /// Set this bit to clear the vad_done_int interrupt
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_VAD_DONE_INT_CLR_OFFSET: u32 = 3;
    pub const LP_VAD_DONE_INT_CLR_MASK: u32 = 0x00000008;
    pub const LP_VAD_DONE_INT_CLR: u32 = 1 << 3;

    /// **RX_FIFOMEM_UDF_INT_CLR**
    ///
    /// Set this bit to clear the i2s_rx_fifomem_udf_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_FIFOMEM_UDF_INT_CLR_OFFSET: u32 = 2;
    pub const RX_FIFOMEM_UDF_INT_CLR_MASK: u32 = 0x00000004;
    pub const RX_FIFOMEM_UDF_INT_CLR: u32 = 1 << 2;

    /// **RX_HUNG_INT_CLR**
    ///
    /// Set this bit to clear the i2s_rx_hung_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_HUNG_INT_CLR_OFFSET: u32 = 1;
    pub const RX_HUNG_INT_CLR_MASK: u32 = 0x00000002;
    pub const RX_HUNG_INT_CLR: u32 = 1 << 1;

    /// **RX_DONE_INT_CLR**
    ///
    /// Set this bit to clear the i2s_rx_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const RX_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const RX_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **RX_MEM_THRESHOLD_INT_CLR** (WO)
    ///
    /// Set this bit to clear the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub fn trigger_rx_mem_threshold_int_clr(&self) {
        self.value.write_bits(Self::RX_MEM_THRESHOLD_INT_CLR_MASK);
    }

    /// Trigger **LP_VAD_RESET_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the vad_reset_done_int interrupt
    #[inline(always)]
    pub fn trigger_lp_vad_reset_done_int_clr(&self) {
        self.value.write_bits(Self::LP_VAD_RESET_DONE_INT_CLR_MASK);
    }

    /// Trigger **LP_VAD_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the vad_done_int interrupt
    #[inline(always)]
    pub fn trigger_lp_vad_done_int_clr(&self) {
        self.value.write_bits(Self::LP_VAD_DONE_INT_CLR_MASK);
    }

    /// Trigger **RX_FIFOMEM_UDF_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub fn trigger_rx_fifomem_udf_int_clr(&self) {
        self.value.write_bits(Self::RX_FIFOMEM_UDF_INT_CLR_MASK);
    }

    /// Trigger **RX_HUNG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn trigger_rx_hung_int_clr(&self) {
        self.value.write_bits(Self::RX_HUNG_INT_CLR_MASK);
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

    /// Set **RX_MEM_THRESHOLD_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the rx_mem_threshold_int interrupt
    #[inline(always)]
    pub const fn with_rx_mem_threshold_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_MEM_THRESHOLD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_MEM_THRESHOLD_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_VAD_RESET_DONE_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the vad_reset_done_int interrupt
    #[inline(always)]
    pub const fn with_lp_vad_reset_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_VAD_RESET_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_VAD_RESET_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_VAD_DONE_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the vad_done_int interrupt
    #[inline(always)]
    pub const fn with_lp_vad_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_VAD_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_VAD_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_FIFOMEM_UDF_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the i2s_rx_fifomem_udf_int interrupt
    #[inline(always)]
    pub const fn with_rx_fifomem_udf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_FIFOMEM_UDF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_FIFOMEM_UDF_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_CLR** `[1:1]`
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
/// Reset value: 0x00009600
///
/// ## Fields
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
/// - **RX_STOP_MODE** `[14:13]` (RW)
///
///   0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
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
/// - **RX_MONO** `[5]` (RW)
///
///   Set this bit to enable receiver in mono mode
///
/// - **RX_FIFOMEM_RESET** `[4]` (WO)
///
///   Set this bit to reset Rx Syncfifomem
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

    /// **RX_STOP_MODE**
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    ///
    /// Bits `[14:13]`, mask `0x00006000`
    pub const RX_STOP_MODE_OFFSET: u32 = 13;
    pub const RX_STOP_MODE_MASK: u32 = 0x00006000;

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
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_MONO_OFFSET: u32 = 5;
    pub const RX_MONO_MASK: u32 = 0x00000020;
    pub const RX_MONO: u32 = 1 << 5;

    /// **RX_FIFOMEM_RESET**
    ///
    /// Set this bit to reset Rx Syncfifomem
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_FIFOMEM_RESET_OFFSET: u32 = 4;
    pub const RX_FIFOMEM_RESET_MASK: u32 = 0x00000010;
    pub const RX_FIFOMEM_RESET: u32 = 1 << 4;

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

    /// **RX_STOP_MODE** `[14:13]` (RW)
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> u32 {
        (self.value.read_bits() & Self::RX_STOP_MODE_MASK) >> Self::RX_STOP_MODE_OFFSET
    }

    /// Write **RX_STOP_MODE** `[14:13]` (RW)
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub fn write_rx_stop_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_STOP_MODE_MASK)
                | ((value << Self::RX_STOP_MODE_OFFSET) & Self::RX_STOP_MODE_MASK)
        });
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

    /// **RX_MONO** `[5]` (RW)
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

    /// Trigger **RX_FIFOMEM_RESET** (WO)
    ///
    /// Set this bit to reset Rx Syncfifomem
    #[inline(always)]
    pub fn trigger_rx_fifomem_reset(&self) {
        self.value.modify_bits(|r| r | Self::RX_FIFOMEM_RESET_MASK);
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
        Self { bits: 0x00009600 }
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

    /// Read **RX_STOP_MODE** `[14:13]`
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub const fn rx_stop_mode(&self) -> u32 {
        (self.bits & RxConf::RX_STOP_MODE_MASK) >> RxConf::RX_STOP_MODE_OFFSET
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

    /// Read **RX_MONO** `[5:5]`
    ///
    /// Set this bit to enable receiver in mono mode
    #[inline(always)]
    pub const fn rx_mono(&self) -> bool {
        (self.bits & RxConf::RX_MONO_MASK) != 0
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

    /// Set **RX_STOP_MODE** `[14:13]`
    ///
    /// 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full.
    #[inline(always)]
    pub const fn with_rx_stop_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf::RX_STOP_MODE_MASK)
                | ((value << RxConf::RX_STOP_MODE_OFFSET) & RxConf::RX_STOP_MODE_MASK),
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

    /// Set **RX_MONO** `[5:5]`
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

    /// Set **RX_FIFOMEM_RESET** `[4:4]`
    ///
    /// Set this bit to reset Rx Syncfifomem
    #[inline(always)]
    pub const fn with_rx_fifomem_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf::RX_FIFOMEM_RESET_MASK
            } else {
                self.bits & !RxConf::RX_FIFOMEM_RESET_MASK
            },
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

/// RX_CONF1 Register
///
/// I2S RX configure register 1
///
/// Address offset: 0x0028
/// Reset value: 0x2f3de300
///
/// ## Fields
///
/// - **RX_MSB_SHIFT** `[29]` (RW)
///
///   Set this bit to enable receiver in Phillips standard mode
///
/// - **RX_TDM_CHAN_BITS** `[28:24]` (RW)
///
///   The Rx bit number for each channel minus 1in TDM mode.
///
/// - **RX_HALF_SAMPLE_BITS** `[23:18]` (RW)
///
///   I2S Rx half sample bits -1.
///
/// - **RX_BITS_MOD** `[17:13]` (RW)
///
///   Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
///
/// - **RX_BCK_DIV_NUM** `[12:7]` (RW)
///
///   Bit clock configuration bits in receiver mode.
///
/// - **RX_TDM_WS_WIDTH** `[6:0]` (RW)
///
///   The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
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

    /// **RX_MSB_SHIFT**
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_MSB_SHIFT_OFFSET: u32 = 29;
    pub const RX_MSB_SHIFT_MASK: u32 = 0x20000000;
    pub const RX_MSB_SHIFT: u32 = 1 << 29;

    /// **RX_TDM_CHAN_BITS**
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const RX_TDM_CHAN_BITS_OFFSET: u32 = 24;
    pub const RX_TDM_CHAN_BITS_MASK: u32 = 0x1f000000;

    /// **RX_HALF_SAMPLE_BITS**
    ///
    /// I2S Rx half sample bits -1.
    ///
    /// Bits `[23:18]`, mask `0x00fc0000`
    pub const RX_HALF_SAMPLE_BITS_OFFSET: u32 = 18;
    pub const RX_HALF_SAMPLE_BITS_MASK: u32 = 0x00fc0000;

    /// **RX_BITS_MOD**
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const RX_BITS_MOD_OFFSET: u32 = 13;
    pub const RX_BITS_MOD_MASK: u32 = 0x0003e000;

    /// **RX_BCK_DIV_NUM**
    ///
    /// Bit clock configuration bits in receiver mode.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const RX_BCK_DIV_NUM_OFFSET: u32 = 7;
    pub const RX_BCK_DIV_NUM_MASK: u32 = 0x00001f80;

    /// **RX_TDM_WS_WIDTH**
    ///
    /// The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const RX_TDM_WS_WIDTH_OFFSET: u32 = 0;
    pub const RX_TDM_WS_WIDTH_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **RX_MSB_SHIFT** `[29]` (RW)
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

    /// **RX_TDM_CHAN_BITS** `[28:24]` (RW)
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TDM_CHAN_BITS_MASK) >> Self::RX_TDM_CHAN_BITS_OFFSET
    }

    /// Write **RX_TDM_CHAN_BITS** `[28:24]` (RW)
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub fn write_rx_tdm_chan_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TDM_CHAN_BITS_MASK)
                | ((value << Self::RX_TDM_CHAN_BITS_OFFSET) & Self::RX_TDM_CHAN_BITS_MASK)
        });
    }

    /// **RX_HALF_SAMPLE_BITS** `[23:18]` (RW)
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> u32 {
        (self.value.read_bits() & Self::RX_HALF_SAMPLE_BITS_MASK)
            >> Self::RX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Write **RX_HALF_SAMPLE_BITS** `[23:18]` (RW)
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub fn write_rx_half_sample_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_HALF_SAMPLE_BITS_MASK)
                | ((value << Self::RX_HALF_SAMPLE_BITS_OFFSET) & Self::RX_HALF_SAMPLE_BITS_MASK)
        });
    }

    /// **RX_BITS_MOD** `[17:13]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BITS_MOD_MASK) >> Self::RX_BITS_MOD_OFFSET
    }

    /// Write **RX_BITS_MOD** `[17:13]` (RW)
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub fn write_rx_bits_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BITS_MOD_MASK)
                | ((value << Self::RX_BITS_MOD_OFFSET) & Self::RX_BITS_MOD_MASK)
        });
    }

    /// **RX_BCK_DIV_NUM** `[12:7]` (RW)
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BCK_DIV_NUM_MASK) >> Self::RX_BCK_DIV_NUM_OFFSET
    }

    /// Write **RX_BCK_DIV_NUM** `[12:7]` (RW)
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub fn write_rx_bck_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BCK_DIV_NUM_MASK)
                | ((value << Self::RX_BCK_DIV_NUM_OFFSET) & Self::RX_BCK_DIV_NUM_MASK)
        });
    }

    /// **RX_TDM_WS_WIDTH** `[6:0]` (RW)
    ///
    /// The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TDM_WS_WIDTH_MASK) >> Self::RX_TDM_WS_WIDTH_OFFSET
    }

    /// Write **RX_TDM_WS_WIDTH** `[6:0]` (RW)
    ///
    /// The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
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
        Self { bits: 0x2f3de300 }
    }

    /// Read **RX_MSB_SHIFT** `[29:29]`
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    #[inline(always)]
    pub const fn rx_msb_shift(&self) -> bool {
        (self.bits & RxConf1::RX_MSB_SHIFT_MASK) != 0
    }

    /// Read **RX_TDM_CHAN_BITS** `[28:24]`
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn rx_tdm_chan_bits(&self) -> u32 {
        (self.bits & RxConf1::RX_TDM_CHAN_BITS_MASK) >> RxConf1::RX_TDM_CHAN_BITS_OFFSET
    }

    /// Read **RX_HALF_SAMPLE_BITS** `[23:18]`
    ///
    /// I2S Rx half sample bits -1.
    #[inline(always)]
    pub const fn rx_half_sample_bits(&self) -> u32 {
        (self.bits & RxConf1::RX_HALF_SAMPLE_BITS_MASK) >> RxConf1::RX_HALF_SAMPLE_BITS_OFFSET
    }

    /// Read **RX_BITS_MOD** `[17:13]`
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn rx_bits_mod(&self) -> u32 {
        (self.bits & RxConf1::RX_BITS_MOD_MASK) >> RxConf1::RX_BITS_MOD_OFFSET
    }

    /// Read **RX_BCK_DIV_NUM** `[12:7]`
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub const fn rx_bck_div_num(&self) -> u32 {
        (self.bits & RxConf1::RX_BCK_DIV_NUM_MASK) >> RxConf1::RX_BCK_DIV_NUM_OFFSET
    }

    /// Read **RX_TDM_WS_WIDTH** `[6:0]`
    ///
    /// The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
    #[inline(always)]
    pub const fn rx_tdm_ws_width(&self) -> u32 {
        (self.bits & RxConf1::RX_TDM_WS_WIDTH_MASK) >> RxConf1::RX_TDM_WS_WIDTH_OFFSET
    }

    /// Set **RX_MSB_SHIFT** `[29:29]`
    ///
    /// Set this bit to enable receiver in Phillips standard mode
    #[inline(always)]
    pub const fn with_rx_msb_shift(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxConf1::RX_MSB_SHIFT_MASK
            } else {
                self.bits & !RxConf1::RX_MSB_SHIFT_MASK
            },
        }
    }

    /// Set **RX_TDM_CHAN_BITS** `[28:24]`
    ///
    /// The Rx bit number for each channel minus 1in TDM mode.
    #[inline(always)]
    pub const fn with_rx_tdm_chan_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_TDM_CHAN_BITS_MASK)
                | ((value << RxConf1::RX_TDM_CHAN_BITS_OFFSET) & RxConf1::RX_TDM_CHAN_BITS_MASK),
        }
    }

    /// Set **RX_HALF_SAMPLE_BITS** `[23:18]`
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

    /// Set **RX_BITS_MOD** `[17:13]`
    ///
    /// Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode.
    #[inline(always)]
    pub const fn with_rx_bits_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_BITS_MOD_MASK)
                | ((value << RxConf1::RX_BITS_MOD_OFFSET) & RxConf1::RX_BITS_MOD_MASK),
        }
    }

    /// Set **RX_BCK_DIV_NUM** `[12:7]`
    ///
    /// Bit clock configuration bits in receiver mode.
    #[inline(always)]
    pub const fn with_rx_bck_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxConf1::RX_BCK_DIV_NUM_MASK)
                | ((value << RxConf1::RX_BCK_DIV_NUM_OFFSET) & RxConf1::RX_BCK_DIV_NUM_MASK),
        }
    }

    /// Set **RX_TDM_WS_WIDTH** `[6:0]`
    ///
    /// The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH[6:0] +1) * T_bck
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

/// RX_TDM_CTRL Register
///
/// I2S TX TDM mode control register
///
/// Address offset: 0x0050
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **RX_TDM_TOT_CHAN_NUM** `[19:16]` (RW)
///
///   The total channel number of I2S TX TDM mode.
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
        Self { bits: 0x00000003 }
    }

    /// Read **RX_TDM_TOT_CHAN_NUM** `[19:16]`
    ///
    /// The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub const fn rx_tdm_tot_chan_num(&self) -> u32 {
        (self.bits & RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_MASK) >> RxTdmCtrl::RX_TDM_TOT_CHAN_NUM_OFFSET
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

/// RX_PDM_CONF Register
///
/// I2S RX configure register
///
/// Address offset: 0x0070
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
pub struct RxPdmConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxPdmConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxPdmConf {
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

/// RX_PDM_CONF value type for batch read/modify/write operations
///
/// Use `rxpdmconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxpdmconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxPdmConfVal {
    bits: u32,
}

impl RxPdmConfVal {
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
        (self.bits & RxPdmConf::RX_IIR_HP_MULT12_0_MASK) >> RxPdmConf::RX_IIR_HP_MULT12_0_OFFSET
    }

    /// Read **RX_IIR_HP_MULT12_5** `[28:26]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn rx_iir_hp_mult12_5(&self) -> u32 {
        (self.bits & RxPdmConf::RX_IIR_HP_MULT12_5_MASK) >> RxPdmConf::RX_IIR_HP_MULT12_5_OFFSET
    }

    /// Read **RX_PDM_HP_BYPASS** `[25:25]`
    ///
    /// I2S PDM RX bypass hp filter or not.
    #[inline(always)]
    pub const fn rx_pdm_hp_bypass(&self) -> bool {
        (self.bits & RxPdmConf::RX_PDM_HP_BYPASS_MASK) != 0
    }

    /// Read **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]`
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub const fn rx_pdm2pcm_amplify_num(&self) -> u32 {
        (self.bits & RxPdmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK)
            >> RxPdmConf::RX_PDM2PCM_AMPLIFY_NUM_OFFSET
    }

    /// Read **RX_PDM_SINC_DSR_16_EN** `[20:20]`
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    #[inline(always)]
    pub const fn rx_pdm_sinc_dsr_16_en(&self) -> bool {
        (self.bits & RxPdmConf::RX_PDM_SINC_DSR_16_EN_MASK) != 0
    }

    /// Read **RX_PDM2PCM_EN** `[19:19]`
    ///
    /// 1: Enable PDM2PCM RX mode. 0: DIsable.
    #[inline(always)]
    pub const fn rx_pdm2pcm_en(&self) -> bool {
        (self.bits & RxPdmConf::RX_PDM2PCM_EN_MASK) != 0
    }

    /// Set **RX_IIR_HP_MULT12_0** `[31:29]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0[2:0])
    #[inline(always)]
    pub const fn with_rx_iir_hp_mult12_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdmConf::RX_IIR_HP_MULT12_0_MASK)
                | ((value << RxPdmConf::RX_IIR_HP_MULT12_0_OFFSET)
                    & RxPdmConf::RX_IIR_HP_MULT12_0_MASK),
        }
    }

    /// Set **RX_IIR_HP_MULT12_5** `[28:26]`
    ///
    /// The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5[2:0])
    #[inline(always)]
    pub const fn with_rx_iir_hp_mult12_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdmConf::RX_IIR_HP_MULT12_5_MASK)
                | ((value << RxPdmConf::RX_IIR_HP_MULT12_5_OFFSET)
                    & RxPdmConf::RX_IIR_HP_MULT12_5_MASK),
        }
    }

    /// Set **RX_PDM_HP_BYPASS** `[25:25]`
    ///
    /// I2S PDM RX bypass hp filter or not.
    #[inline(always)]
    pub const fn with_rx_pdm_hp_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxPdmConf::RX_PDM_HP_BYPASS_MASK
            } else {
                self.bits & !RxPdmConf::RX_PDM_HP_BYPASS_MASK
            },
        }
    }

    /// Set **RX_PDM2PCM_AMPLIFY_NUM** `[24:21]`
    ///
    /// Configure PDM RX amplify number.
    #[inline(always)]
    pub const fn with_rx_pdm2pcm_amplify_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxPdmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK)
                | ((value << RxPdmConf::RX_PDM2PCM_AMPLIFY_NUM_OFFSET)
                    & RxPdmConf::RX_PDM2PCM_AMPLIFY_NUM_MASK),
        }
    }

    /// Set **RX_PDM_SINC_DSR_16_EN** `[20:20]`
    ///
    /// Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64.
    #[inline(always)]
    pub const fn with_rx_pdm_sinc_dsr_16_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxPdmConf::RX_PDM_SINC_DSR_16_EN_MASK
            } else {
                self.bits & !RxPdmConf::RX_PDM_SINC_DSR_16_EN_MASK
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
                self.bits | RxPdmConf::RX_PDM2PCM_EN_MASK
            } else {
                self.bits & !RxPdmConf::RX_PDM2PCM_EN_MASK
            },
        }
    }
}

impl Default for RxPdmConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxPdmConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxPdmConfVal {
        RxPdmConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxPdmConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxPdmConfVal) -> RxPdmConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_LOW Register
///
/// I2S ECO register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RDN_ECO_LOW** `[31:0]` (RW)
///
///   logic low eco registers
#[repr(transparent)]
pub struct EcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoLow {
    // ======== Field Constants ========

    /// **RDN_ECO_LOW**
    ///
    /// logic low eco registers
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// logic low eco registers
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_LOW_MASK) >> Self::RDN_ECO_LOW_OFFSET
    }

    /// Write **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// logic low eco registers
    #[inline(always)]
    pub fn write_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_LOW_MASK)
                | ((value << Self::RDN_ECO_LOW_OFFSET) & Self::RDN_ECO_LOW_MASK)
        });
    }
}

/// ECO_LOW value type for batch read/modify/write operations
///
/// Use `ecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoLowVal {
    bits: u32,
}

impl EcoLowVal {
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

    /// Read **RDN_ECO_LOW** `[31:0]`
    ///
    /// logic low eco registers
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> u32 {
        (self.bits & EcoLow::RDN_ECO_LOW_MASK) >> EcoLow::RDN_ECO_LOW_OFFSET
    }

    /// Set **RDN_ECO_LOW** `[31:0]`
    ///
    /// logic low eco registers
    #[inline(always)]
    pub const fn with_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoLow::RDN_ECO_LOW_MASK)
                | ((value << EcoLow::RDN_ECO_LOW_OFFSET) & EcoLow::RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for EcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoLowVal {
        EcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoLowVal) -> EcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_HIGH Register
///
/// I2S ECO register
///
/// Address offset: 0x0078
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RDN_ECO_HIGH** `[31:0]` (RW)
///
///   logic high eco registers
#[repr(transparent)]
pub struct EcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoHigh {
    // ======== Field Constants ========

    /// **RDN_ECO_HIGH**
    ///
    /// logic high eco registers
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// logic high eco registers
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_HIGH_MASK) >> Self::RDN_ECO_HIGH_OFFSET
    }

    /// Write **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// logic high eco registers
    #[inline(always)]
    pub fn write_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_HIGH_MASK)
                | ((value << Self::RDN_ECO_HIGH_OFFSET) & Self::RDN_ECO_HIGH_MASK)
        });
    }
}

/// ECO_HIGH value type for batch read/modify/write operations
///
/// Use `ecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoHighVal {
    bits: u32,
}

impl EcoHighVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **RDN_ECO_HIGH** `[31:0]`
    ///
    /// logic high eco registers
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> u32 {
        (self.bits & EcoHigh::RDN_ECO_HIGH_MASK) >> EcoHigh::RDN_ECO_HIGH_OFFSET
    }

    /// Set **RDN_ECO_HIGH** `[31:0]`
    ///
    /// logic high eco registers
    #[inline(always)]
    pub const fn with_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoHigh::RDN_ECO_HIGH_MASK)
                | ((value << EcoHigh::RDN_ECO_HIGH_OFFSET) & EcoHigh::RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for EcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoHighVal {
        EcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoHighVal) -> EcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_CONF Register
///
/// I2S ECO register
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **RDN_RESULT** `[1]` (RO)
///
///   rdn result
///
/// - **RDN_ENA** `[0]` (RW)
///
///   enable rdn counter bit
#[repr(transparent)]
pub struct EcoConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoConf {
    // ======== Field Constants ========

    /// **RDN_RESULT**
    ///
    /// rdn result
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDN_RESULT_OFFSET: u32 = 1;
    pub const RDN_RESULT_MASK: u32 = 0x00000002;
    pub const RDN_RESULT: u32 = 1 << 1;

    /// **RDN_ENA**
    ///
    /// enable rdn counter bit
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDN_ENA_OFFSET: u32 = 0;
    pub const RDN_ENA_MASK: u32 = 0x00000001;
    pub const RDN_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RDN_RESULT** `[1]` (RO)
    ///
    /// rdn result
    #[inline(always)]
    pub fn rdn_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_RESULT_MASK) != 0
    }

    /// **RDN_ENA** `[0]` (RW)
    ///
    /// enable rdn counter bit
    #[inline(always)]
    pub fn rdn_ena(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ENA_MASK) != 0
    }

    /// Set RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rdn_ena(&self) {
        self.value.modify_bits(|r| r | Self::RDN_ENA_MASK);
    }

    /// Clear RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rdn_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RDN_ENA_MASK);
    }
}

/// ECO_CONF value type for batch read/modify/write operations
///
/// Use `ecoconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecoconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoConfVal {
    bits: u32,
}

impl EcoConfVal {
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

    /// Read **RDN_RESULT** `[1:1]`
    ///
    /// rdn result
    #[inline(always)]
    pub const fn rdn_result(&self) -> bool {
        (self.bits & EcoConf::RDN_RESULT_MASK) != 0
    }

    /// Read **RDN_ENA** `[0:0]`
    ///
    /// enable rdn counter bit
    #[inline(always)]
    pub const fn rdn_ena(&self) -> bool {
        (self.bits & EcoConf::RDN_ENA_MASK) != 0
    }

    /// Set **RDN_ENA** `[0:0]`
    ///
    /// enable rdn counter bit
    #[inline(always)]
    pub const fn with_rdn_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EcoConf::RDN_ENA_MASK
            } else {
                self.bits & !EcoConf::RDN_ENA_MASK
            },
        }
    }
}

impl Default for EcoConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoConfVal {
        EcoConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoConfVal) -> EcoConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM0 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0080
/// Reset value: 0x00c81388
///
/// ## Fields
///
/// - **PARAM_INIT_FRAME_NUM** `[24:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_MIN_ENERGY** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam0 {
    // ======== Field Constants ========

    /// **PARAM_INIT_FRAME_NUM**
    ///
    /// VAD parameter
    ///
    /// Bits `[24:16]`, mask `0x01ff0000`
    pub const PARAM_INIT_FRAME_NUM_OFFSET: u32 = 16;
    pub const PARAM_INIT_FRAME_NUM_MASK: u32 = 0x01ff0000;

    /// **PARAM_MIN_ENERGY**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_MIN_ENERGY_OFFSET: u32 = 0;
    pub const PARAM_MIN_ENERGY_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_INIT_FRAME_NUM** `[24:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_init_frame_num(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_INIT_FRAME_NUM_MASK)
            >> Self::PARAM_INIT_FRAME_NUM_OFFSET
    }

    /// Write **PARAM_INIT_FRAME_NUM** `[24:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_init_frame_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_INIT_FRAME_NUM_MASK)
                | ((value << Self::PARAM_INIT_FRAME_NUM_OFFSET) & Self::PARAM_INIT_FRAME_NUM_MASK)
        });
    }

    /// **PARAM_MIN_ENERGY** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_min_energy(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_MIN_ENERGY_MASK) >> Self::PARAM_MIN_ENERGY_OFFSET
    }

    /// Write **PARAM_MIN_ENERGY** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_min_energy(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_MIN_ENERGY_MASK)
                | ((value << Self::PARAM_MIN_ENERGY_OFFSET) & Self::PARAM_MIN_ENERGY_MASK)
        });
    }
}

/// VAD_PARAM0 value type for batch read/modify/write operations
///
/// Use `vadparam0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam0Val {
    bits: u32,
}

impl VadParam0Val {
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
        Self { bits: 0x00c81388 }
    }

    /// Read **PARAM_INIT_FRAME_NUM** `[24:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_init_frame_num(&self) -> u32 {
        (self.bits & VadParam0::PARAM_INIT_FRAME_NUM_MASK) >> VadParam0::PARAM_INIT_FRAME_NUM_OFFSET
    }

    /// Read **PARAM_MIN_ENERGY** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_min_energy(&self) -> u32 {
        (self.bits & VadParam0::PARAM_MIN_ENERGY_MASK) >> VadParam0::PARAM_MIN_ENERGY_OFFSET
    }

    /// Set **PARAM_INIT_FRAME_NUM** `[24:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_init_frame_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam0::PARAM_INIT_FRAME_NUM_MASK)
                | ((value << VadParam0::PARAM_INIT_FRAME_NUM_OFFSET)
                    & VadParam0::PARAM_INIT_FRAME_NUM_MASK),
        }
    }

    /// Set **PARAM_MIN_ENERGY** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_min_energy(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam0::PARAM_MIN_ENERGY_MASK)
                | ((value << VadParam0::PARAM_MIN_ENERGY_OFFSET)
                    & VadParam0::PARAM_MIN_ENERGY_MASK),
        }
    }
}

impl Default for VadParam0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam0Val {
        VadParam0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam0Val) -> VadParam0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM1 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0084
/// Reset value: 0x281e1e43
///
/// ## Fields
///
/// - **PARAM_SKIP_BAND_ENERGY** `[31]` (RW)
///
///   Set 1 to skip band energy check.
///
/// - **PARAM_MAX_OFFSET** `[30:24]` (RW)
///
///   VAD parameter
///
/// - **PARAM_HANGOVER_SILENT** `[23:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_HANGOVER_SPEECH** `[15:11]` (RW)
///
///   VAD parameter
///
/// - **PARAM_MAX_SPEECH_COUNT** `[10:4]` (RW)
///
///   VAD parameter
///
/// - **PARAM_MIN_SPEECH_COUNT** `[3:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam1 {
    // ======== Field Constants ========

    /// **PARAM_SKIP_BAND_ENERGY**
    ///
    /// Set 1 to skip band energy check.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const PARAM_SKIP_BAND_ENERGY_OFFSET: u32 = 31;
    pub const PARAM_SKIP_BAND_ENERGY_MASK: u32 = 0x80000000;
    pub const PARAM_SKIP_BAND_ENERGY: u32 = 1 << 31;

    /// **PARAM_MAX_OFFSET**
    ///
    /// VAD parameter
    ///
    /// Bits `[30:24]`, mask `0x7f000000`
    pub const PARAM_MAX_OFFSET_OFFSET: u32 = 24;
    pub const PARAM_MAX_OFFSET_MASK: u32 = 0x7f000000;

    /// **PARAM_HANGOVER_SILENT**
    ///
    /// VAD parameter
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PARAM_HANGOVER_SILENT_OFFSET: u32 = 16;
    pub const PARAM_HANGOVER_SILENT_MASK: u32 = 0x00ff0000;

    /// **PARAM_HANGOVER_SPEECH**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:11]`, mask `0x0000f800`
    pub const PARAM_HANGOVER_SPEECH_OFFSET: u32 = 11;
    pub const PARAM_HANGOVER_SPEECH_MASK: u32 = 0x0000f800;

    /// **PARAM_MAX_SPEECH_COUNT**
    ///
    /// VAD parameter
    ///
    /// Bits `[10:4]`, mask `0x000007f0`
    pub const PARAM_MAX_SPEECH_COUNT_OFFSET: u32 = 4;
    pub const PARAM_MAX_SPEECH_COUNT_MASK: u32 = 0x000007f0;

    /// **PARAM_MIN_SPEECH_COUNT**
    ///
    /// VAD parameter
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PARAM_MIN_SPEECH_COUNT_OFFSET: u32 = 0;
    pub const PARAM_MIN_SPEECH_COUNT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **PARAM_SKIP_BAND_ENERGY** `[31]` (RW)
    ///
    /// Set 1 to skip band energy check.
    #[inline(always)]
    pub fn param_skip_band_energy(&self) -> bool {
        (self.value.read_bits() & Self::PARAM_SKIP_BAND_ENERGY_MASK) != 0
    }

    /// Set PARAM_SKIP_BAND_ENERGY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_param_skip_band_energy(&self) {
        self.value
            .modify_bits(|r| r | Self::PARAM_SKIP_BAND_ENERGY_MASK);
    }

    /// Clear PARAM_SKIP_BAND_ENERGY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_param_skip_band_energy(&self) {
        self.value
            .modify_bits(|r| r & !Self::PARAM_SKIP_BAND_ENERGY_MASK);
    }

    /// **PARAM_MAX_OFFSET** `[30:24]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_max_offset(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_MAX_OFFSET_MASK) >> Self::PARAM_MAX_OFFSET_OFFSET
    }

    /// Write **PARAM_MAX_OFFSET** `[30:24]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_max_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_MAX_OFFSET_MASK)
                | ((value << Self::PARAM_MAX_OFFSET_OFFSET) & Self::PARAM_MAX_OFFSET_MASK)
        });
    }

    /// **PARAM_HANGOVER_SILENT** `[23:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_hangover_silent(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_HANGOVER_SILENT_MASK)
            >> Self::PARAM_HANGOVER_SILENT_OFFSET
    }

    /// Write **PARAM_HANGOVER_SILENT** `[23:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_hangover_silent(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_HANGOVER_SILENT_MASK)
                | ((value << Self::PARAM_HANGOVER_SILENT_OFFSET) & Self::PARAM_HANGOVER_SILENT_MASK)
        });
    }

    /// **PARAM_HANGOVER_SPEECH** `[15:11]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_hangover_speech(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_HANGOVER_SPEECH_MASK)
            >> Self::PARAM_HANGOVER_SPEECH_OFFSET
    }

    /// Write **PARAM_HANGOVER_SPEECH** `[15:11]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_hangover_speech(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_HANGOVER_SPEECH_MASK)
                | ((value << Self::PARAM_HANGOVER_SPEECH_OFFSET) & Self::PARAM_HANGOVER_SPEECH_MASK)
        });
    }

    /// **PARAM_MAX_SPEECH_COUNT** `[10:4]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_max_speech_count(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_MAX_SPEECH_COUNT_MASK)
            >> Self::PARAM_MAX_SPEECH_COUNT_OFFSET
    }

    /// Write **PARAM_MAX_SPEECH_COUNT** `[10:4]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_max_speech_count(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_MAX_SPEECH_COUNT_MASK)
                | ((value << Self::PARAM_MAX_SPEECH_COUNT_OFFSET)
                    & Self::PARAM_MAX_SPEECH_COUNT_MASK)
        });
    }

    /// **PARAM_MIN_SPEECH_COUNT** `[3:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_min_speech_count(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_MIN_SPEECH_COUNT_MASK)
            >> Self::PARAM_MIN_SPEECH_COUNT_OFFSET
    }

    /// Write **PARAM_MIN_SPEECH_COUNT** `[3:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_min_speech_count(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_MIN_SPEECH_COUNT_MASK)
                | ((value << Self::PARAM_MIN_SPEECH_COUNT_OFFSET)
                    & Self::PARAM_MIN_SPEECH_COUNT_MASK)
        });
    }
}

/// VAD_PARAM1 value type for batch read/modify/write operations
///
/// Use `vadparam1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam1Val {
    bits: u32,
}

impl VadParam1Val {
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
        Self { bits: 0x281e1e43 }
    }

    /// Read **PARAM_SKIP_BAND_ENERGY** `[31:31]`
    ///
    /// Set 1 to skip band energy check.
    #[inline(always)]
    pub const fn param_skip_band_energy(&self) -> bool {
        (self.bits & VadParam1::PARAM_SKIP_BAND_ENERGY_MASK) != 0
    }

    /// Read **PARAM_MAX_OFFSET** `[30:24]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_max_offset(&self) -> u32 {
        (self.bits & VadParam1::PARAM_MAX_OFFSET_MASK) >> VadParam1::PARAM_MAX_OFFSET_OFFSET
    }

    /// Read **PARAM_HANGOVER_SILENT** `[23:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_hangover_silent(&self) -> u32 {
        (self.bits & VadParam1::PARAM_HANGOVER_SILENT_MASK)
            >> VadParam1::PARAM_HANGOVER_SILENT_OFFSET
    }

    /// Read **PARAM_HANGOVER_SPEECH** `[15:11]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_hangover_speech(&self) -> u32 {
        (self.bits & VadParam1::PARAM_HANGOVER_SPEECH_MASK)
            >> VadParam1::PARAM_HANGOVER_SPEECH_OFFSET
    }

    /// Read **PARAM_MAX_SPEECH_COUNT** `[10:4]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_max_speech_count(&self) -> u32 {
        (self.bits & VadParam1::PARAM_MAX_SPEECH_COUNT_MASK)
            >> VadParam1::PARAM_MAX_SPEECH_COUNT_OFFSET
    }

    /// Read **PARAM_MIN_SPEECH_COUNT** `[3:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_min_speech_count(&self) -> u32 {
        (self.bits & VadParam1::PARAM_MIN_SPEECH_COUNT_MASK)
            >> VadParam1::PARAM_MIN_SPEECH_COUNT_OFFSET
    }

    /// Set **PARAM_SKIP_BAND_ENERGY** `[31:31]`
    ///
    /// Set 1 to skip band energy check.
    #[inline(always)]
    pub const fn with_param_skip_band_energy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VadParam1::PARAM_SKIP_BAND_ENERGY_MASK
            } else {
                self.bits & !VadParam1::PARAM_SKIP_BAND_ENERGY_MASK
            },
        }
    }

    /// Set **PARAM_MAX_OFFSET** `[30:24]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_max_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam1::PARAM_MAX_OFFSET_MASK)
                | ((value << VadParam1::PARAM_MAX_OFFSET_OFFSET)
                    & VadParam1::PARAM_MAX_OFFSET_MASK),
        }
    }

    /// Set **PARAM_HANGOVER_SILENT** `[23:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_hangover_silent(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam1::PARAM_HANGOVER_SILENT_MASK)
                | ((value << VadParam1::PARAM_HANGOVER_SILENT_OFFSET)
                    & VadParam1::PARAM_HANGOVER_SILENT_MASK),
        }
    }

    /// Set **PARAM_HANGOVER_SPEECH** `[15:11]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_hangover_speech(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam1::PARAM_HANGOVER_SPEECH_MASK)
                | ((value << VadParam1::PARAM_HANGOVER_SPEECH_OFFSET)
                    & VadParam1::PARAM_HANGOVER_SPEECH_MASK),
        }
    }

    /// Set **PARAM_MAX_SPEECH_COUNT** `[10:4]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_max_speech_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam1::PARAM_MAX_SPEECH_COUNT_MASK)
                | ((value << VadParam1::PARAM_MAX_SPEECH_COUNT_OFFSET)
                    & VadParam1::PARAM_MAX_SPEECH_COUNT_MASK),
        }
    }

    /// Set **PARAM_MIN_SPEECH_COUNT** `[3:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_min_speech_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam1::PARAM_MIN_SPEECH_COUNT_MASK)
                | ((value << VadParam1::PARAM_MIN_SPEECH_COUNT_OFFSET)
                    & VadParam1::PARAM_MIN_SPEECH_COUNT_MASK),
        }
    }
}

impl Default for VadParam1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam1Val {
        VadParam1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam1Val) -> VadParam1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM2 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0088
/// Reset value: 0x7eb86666
///
/// ## Fields
///
/// - **PARAM_NOISE_AMP_UP** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_NOISE_AMP_DOWN** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam2 {
    // ======== Field Constants ========

    /// **PARAM_NOISE_AMP_UP**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_NOISE_AMP_UP_OFFSET: u32 = 16;
    pub const PARAM_NOISE_AMP_UP_MASK: u32 = 0xffff0000;

    /// **PARAM_NOISE_AMP_DOWN**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_NOISE_AMP_DOWN_OFFSET: u32 = 0;
    pub const PARAM_NOISE_AMP_DOWN_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_NOISE_AMP_UP** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_amp_up(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_AMP_UP_MASK) >> Self::PARAM_NOISE_AMP_UP_OFFSET
    }

    /// Write **PARAM_NOISE_AMP_UP** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_amp_up(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_AMP_UP_MASK)
                | ((value << Self::PARAM_NOISE_AMP_UP_OFFSET) & Self::PARAM_NOISE_AMP_UP_MASK)
        });
    }

    /// **PARAM_NOISE_AMP_DOWN** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_amp_down(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_AMP_DOWN_MASK)
            >> Self::PARAM_NOISE_AMP_DOWN_OFFSET
    }

    /// Write **PARAM_NOISE_AMP_DOWN** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_amp_down(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_AMP_DOWN_MASK)
                | ((value << Self::PARAM_NOISE_AMP_DOWN_OFFSET) & Self::PARAM_NOISE_AMP_DOWN_MASK)
        });
    }
}

/// VAD_PARAM2 value type for batch read/modify/write operations
///
/// Use `vadparam2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam2Val {
    bits: u32,
}

impl VadParam2Val {
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
        Self { bits: 0x7eb86666 }
    }

    /// Read **PARAM_NOISE_AMP_UP** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_amp_up(&self) -> u32 {
        (self.bits & VadParam2::PARAM_NOISE_AMP_UP_MASK) >> VadParam2::PARAM_NOISE_AMP_UP_OFFSET
    }

    /// Read **PARAM_NOISE_AMP_DOWN** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_amp_down(&self) -> u32 {
        (self.bits & VadParam2::PARAM_NOISE_AMP_DOWN_MASK) >> VadParam2::PARAM_NOISE_AMP_DOWN_OFFSET
    }

    /// Set **PARAM_NOISE_AMP_UP** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_amp_up(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam2::PARAM_NOISE_AMP_UP_MASK)
                | ((value << VadParam2::PARAM_NOISE_AMP_UP_OFFSET)
                    & VadParam2::PARAM_NOISE_AMP_UP_MASK),
        }
    }

    /// Set **PARAM_NOISE_AMP_DOWN** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_amp_down(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam2::PARAM_NOISE_AMP_DOWN_MASK)
                | ((value << VadParam2::PARAM_NOISE_AMP_DOWN_OFFSET)
                    & VadParam2::PARAM_NOISE_AMP_DOWN_MASK),
        }
    }
}

impl Default for VadParam2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam2Val {
        VadParam2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam2Val) -> VadParam2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM3 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x008c
/// Reset value: 0x7d717fdf
///
/// ## Fields
///
/// - **PARAM_NOISE_SPE_UP1** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_NOISE_SPE_UP0** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam3 {
    // ======== Field Constants ========

    /// **PARAM_NOISE_SPE_UP1**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_NOISE_SPE_UP1_OFFSET: u32 = 16;
    pub const PARAM_NOISE_SPE_UP1_MASK: u32 = 0xffff0000;

    /// **PARAM_NOISE_SPE_UP0**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_NOISE_SPE_UP0_OFFSET: u32 = 0;
    pub const PARAM_NOISE_SPE_UP0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_NOISE_SPE_UP1** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_spe_up1(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_SPE_UP1_MASK)
            >> Self::PARAM_NOISE_SPE_UP1_OFFSET
    }

    /// Write **PARAM_NOISE_SPE_UP1** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_spe_up1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_SPE_UP1_MASK)
                | ((value << Self::PARAM_NOISE_SPE_UP1_OFFSET) & Self::PARAM_NOISE_SPE_UP1_MASK)
        });
    }

    /// **PARAM_NOISE_SPE_UP0** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_spe_up0(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_SPE_UP0_MASK)
            >> Self::PARAM_NOISE_SPE_UP0_OFFSET
    }

    /// Write **PARAM_NOISE_SPE_UP0** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_spe_up0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_SPE_UP0_MASK)
                | ((value << Self::PARAM_NOISE_SPE_UP0_OFFSET) & Self::PARAM_NOISE_SPE_UP0_MASK)
        });
    }
}

/// VAD_PARAM3 value type for batch read/modify/write operations
///
/// Use `vadparam3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam3Val {
    bits: u32,
}

impl VadParam3Val {
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
        Self { bits: 0x7d717fdf }
    }

    /// Read **PARAM_NOISE_SPE_UP1** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_spe_up1(&self) -> u32 {
        (self.bits & VadParam3::PARAM_NOISE_SPE_UP1_MASK) >> VadParam3::PARAM_NOISE_SPE_UP1_OFFSET
    }

    /// Read **PARAM_NOISE_SPE_UP0** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_spe_up0(&self) -> u32 {
        (self.bits & VadParam3::PARAM_NOISE_SPE_UP0_MASK) >> VadParam3::PARAM_NOISE_SPE_UP0_OFFSET
    }

    /// Set **PARAM_NOISE_SPE_UP1** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_spe_up1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam3::PARAM_NOISE_SPE_UP1_MASK)
                | ((value << VadParam3::PARAM_NOISE_SPE_UP1_OFFSET)
                    & VadParam3::PARAM_NOISE_SPE_UP1_MASK),
        }
    }

    /// Set **PARAM_NOISE_SPE_UP0** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_spe_up0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam3::PARAM_NOISE_SPE_UP0_MASK)
                | ((value << VadParam3::PARAM_NOISE_SPE_UP0_OFFSET)
                    & VadParam3::PARAM_NOISE_SPE_UP0_MASK),
        }
    }
}

impl Default for VadParam3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam3Val {
        VadParam3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam3Val) -> VadParam3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM4 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0090
/// Reset value: 0x799a6666
///
/// ## Fields
///
/// - **PARAM_NOISE_MEAN_DOWN** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_NOISE_SPE_DOWN** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam4 {
    // ======== Field Constants ========

    /// **PARAM_NOISE_MEAN_DOWN**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_NOISE_MEAN_DOWN_OFFSET: u32 = 16;
    pub const PARAM_NOISE_MEAN_DOWN_MASK: u32 = 0xffff0000;

    /// **PARAM_NOISE_SPE_DOWN**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_NOISE_SPE_DOWN_OFFSET: u32 = 0;
    pub const PARAM_NOISE_SPE_DOWN_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_NOISE_MEAN_DOWN** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_mean_down(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_MEAN_DOWN_MASK)
            >> Self::PARAM_NOISE_MEAN_DOWN_OFFSET
    }

    /// Write **PARAM_NOISE_MEAN_DOWN** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_mean_down(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_MEAN_DOWN_MASK)
                | ((value << Self::PARAM_NOISE_MEAN_DOWN_OFFSET) & Self::PARAM_NOISE_MEAN_DOWN_MASK)
        });
    }

    /// **PARAM_NOISE_SPE_DOWN** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_spe_down(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_SPE_DOWN_MASK)
            >> Self::PARAM_NOISE_SPE_DOWN_OFFSET
    }

    /// Write **PARAM_NOISE_SPE_DOWN** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_spe_down(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_SPE_DOWN_MASK)
                | ((value << Self::PARAM_NOISE_SPE_DOWN_OFFSET) & Self::PARAM_NOISE_SPE_DOWN_MASK)
        });
    }
}

/// VAD_PARAM4 value type for batch read/modify/write operations
///
/// Use `vadparam4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam4Val {
    bits: u32,
}

impl VadParam4Val {
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
        Self { bits: 0x799a6666 }
    }

    /// Read **PARAM_NOISE_MEAN_DOWN** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_mean_down(&self) -> u32 {
        (self.bits & VadParam4::PARAM_NOISE_MEAN_DOWN_MASK)
            >> VadParam4::PARAM_NOISE_MEAN_DOWN_OFFSET
    }

    /// Read **PARAM_NOISE_SPE_DOWN** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_spe_down(&self) -> u32 {
        (self.bits & VadParam4::PARAM_NOISE_SPE_DOWN_MASK) >> VadParam4::PARAM_NOISE_SPE_DOWN_OFFSET
    }

    /// Set **PARAM_NOISE_MEAN_DOWN** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_mean_down(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam4::PARAM_NOISE_MEAN_DOWN_MASK)
                | ((value << VadParam4::PARAM_NOISE_MEAN_DOWN_OFFSET)
                    & VadParam4::PARAM_NOISE_MEAN_DOWN_MASK),
        }
    }

    /// Set **PARAM_NOISE_SPE_DOWN** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_spe_down(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam4::PARAM_NOISE_SPE_DOWN_MASK)
                | ((value << VadParam4::PARAM_NOISE_SPE_DOWN_OFFSET)
                    & VadParam4::PARAM_NOISE_SPE_DOWN_MASK),
        }
    }
}

impl Default for VadParam4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam4Val {
        VadParam4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam4Val) -> VadParam4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM5 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0094
/// Reset value: 0x7c287d71
///
/// ## Fields
///
/// - **PARAM_NOISE_MEAN_UP1** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_NOISE_MEAN_UP0** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam5 {
    // ======== Field Constants ========

    /// **PARAM_NOISE_MEAN_UP1**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_NOISE_MEAN_UP1_OFFSET: u32 = 16;
    pub const PARAM_NOISE_MEAN_UP1_MASK: u32 = 0xffff0000;

    /// **PARAM_NOISE_MEAN_UP0**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_NOISE_MEAN_UP0_OFFSET: u32 = 0;
    pub const PARAM_NOISE_MEAN_UP0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_NOISE_MEAN_UP1** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_mean_up1(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_MEAN_UP1_MASK)
            >> Self::PARAM_NOISE_MEAN_UP1_OFFSET
    }

    /// Write **PARAM_NOISE_MEAN_UP1** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_mean_up1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_MEAN_UP1_MASK)
                | ((value << Self::PARAM_NOISE_MEAN_UP1_OFFSET) & Self::PARAM_NOISE_MEAN_UP1_MASK)
        });
    }

    /// **PARAM_NOISE_MEAN_UP0** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_noise_mean_up0(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_MEAN_UP0_MASK)
            >> Self::PARAM_NOISE_MEAN_UP0_OFFSET
    }

    /// Write **PARAM_NOISE_MEAN_UP0** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_noise_mean_up0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_MEAN_UP0_MASK)
                | ((value << Self::PARAM_NOISE_MEAN_UP0_OFFSET) & Self::PARAM_NOISE_MEAN_UP0_MASK)
        });
    }
}

/// VAD_PARAM5 value type for batch read/modify/write operations
///
/// Use `vadparam5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam5Val {
    bits: u32,
}

impl VadParam5Val {
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
        Self { bits: 0x7c287d71 }
    }

    /// Read **PARAM_NOISE_MEAN_UP1** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_mean_up1(&self) -> u32 {
        (self.bits & VadParam5::PARAM_NOISE_MEAN_UP1_MASK) >> VadParam5::PARAM_NOISE_MEAN_UP1_OFFSET
    }

    /// Read **PARAM_NOISE_MEAN_UP0** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_noise_mean_up0(&self) -> u32 {
        (self.bits & VadParam5::PARAM_NOISE_MEAN_UP0_MASK) >> VadParam5::PARAM_NOISE_MEAN_UP0_OFFSET
    }

    /// Set **PARAM_NOISE_MEAN_UP1** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_mean_up1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam5::PARAM_NOISE_MEAN_UP1_MASK)
                | ((value << VadParam5::PARAM_NOISE_MEAN_UP1_OFFSET)
                    & VadParam5::PARAM_NOISE_MEAN_UP1_MASK),
        }
    }

    /// Set **PARAM_NOISE_MEAN_UP0** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_noise_mean_up0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam5::PARAM_NOISE_MEAN_UP0_MASK)
                | ((value << VadParam5::PARAM_NOISE_MEAN_UP0_OFFSET)
                    & VadParam5::PARAM_NOISE_MEAN_UP0_MASK),
        }
    }
}

impl Default for VadParam5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam5Val {
        VadParam5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam5Val) -> VadParam5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM6 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x0098
/// Reset value: 0xb4007d00
///
/// ## Fields
///
/// - **PARAM_NOISE_STD_FS_THSH** `[31:16]` (RW)
///
///   Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
///
/// - **PARAM_NOISE_STD_FS_THSL** `[15:0]` (RW)
///
///   Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
#[repr(transparent)]
pub struct VadParam6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam6 {
    // ======== Field Constants ========

    /// **PARAM_NOISE_STD_FS_THSH**
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_NOISE_STD_FS_THSH_OFFSET: u32 = 16;
    pub const PARAM_NOISE_STD_FS_THSH_MASK: u32 = 0xffff0000;

    /// **PARAM_NOISE_STD_FS_THSL**
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_NOISE_STD_FS_THSL_OFFSET: u32 = 0;
    pub const PARAM_NOISE_STD_FS_THSL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_NOISE_STD_FS_THSH** `[31:16]` (RW)
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn param_noise_std_fs_thsh(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_STD_FS_THSH_MASK)
            >> Self::PARAM_NOISE_STD_FS_THSH_OFFSET
    }

    /// Write **PARAM_NOISE_STD_FS_THSH** `[31:16]` (RW)
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn write_param_noise_std_fs_thsh(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_STD_FS_THSH_MASK)
                | ((value << Self::PARAM_NOISE_STD_FS_THSH_OFFSET)
                    & Self::PARAM_NOISE_STD_FS_THSH_MASK)
        });
    }

    /// **PARAM_NOISE_STD_FS_THSL** `[15:0]` (RW)
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn param_noise_std_fs_thsl(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_NOISE_STD_FS_THSL_MASK)
            >> Self::PARAM_NOISE_STD_FS_THSL_OFFSET
    }

    /// Write **PARAM_NOISE_STD_FS_THSL** `[15:0]` (RW)
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn write_param_noise_std_fs_thsl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_NOISE_STD_FS_THSL_MASK)
                | ((value << Self::PARAM_NOISE_STD_FS_THSL_OFFSET)
                    & Self::PARAM_NOISE_STD_FS_THSL_MASK)
        });
    }
}

/// VAD_PARAM6 value type for batch read/modify/write operations
///
/// Use `vadparam6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam6Val {
    bits: u32,
}

impl VadParam6Val {
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
        Self { bits: 0xb4007d00 }
    }

    /// Read **PARAM_NOISE_STD_FS_THSH** `[31:16]`
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub const fn param_noise_std_fs_thsh(&self) -> u32 {
        (self.bits & VadParam6::PARAM_NOISE_STD_FS_THSH_MASK)
            >> VadParam6::PARAM_NOISE_STD_FS_THSH_OFFSET
    }

    /// Read **PARAM_NOISE_STD_FS_THSL** `[15:0]`
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub const fn param_noise_std_fs_thsl(&self) -> u32 {
        (self.bits & VadParam6::PARAM_NOISE_STD_FS_THSL_MASK)
            >> VadParam6::PARAM_NOISE_STD_FS_THSL_OFFSET
    }

    /// Set **PARAM_NOISE_STD_FS_THSH** `[31:16]`
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub const fn with_param_noise_std_fs_thsh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam6::PARAM_NOISE_STD_FS_THSH_MASK)
                | ((value << VadParam6::PARAM_NOISE_STD_FS_THSH_OFFSET)
                    & VadParam6::PARAM_NOISE_STD_FS_THSH_MASK),
        }
    }

    /// Set **PARAM_NOISE_STD_FS_THSL** `[15:0]`
    ///
    /// Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub const fn with_param_noise_std_fs_thsl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam6::PARAM_NOISE_STD_FS_THSL_MASK)
                | ((value << VadParam6::PARAM_NOISE_STD_FS_THSL_OFFSET)
                    & VadParam6::PARAM_NOISE_STD_FS_THSL_MASK),
        }
    }
}

impl Default for VadParam6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam6Val {
        VadParam6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam6Val) -> VadParam6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM7 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x009c
/// Reset value: 0x01487eb8
///
/// ## Fields
///
/// - **PARAM_THRES_UPD_VARY** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_THRES_UPD_BASE** `[15:0]` (RW)
///
///   VAD parameter
#[repr(transparent)]
pub struct VadParam7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam7 {
    // ======== Field Constants ========

    /// **PARAM_THRES_UPD_VARY**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_THRES_UPD_VARY_OFFSET: u32 = 16;
    pub const PARAM_THRES_UPD_VARY_MASK: u32 = 0xffff0000;

    /// **PARAM_THRES_UPD_BASE**
    ///
    /// VAD parameter
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PARAM_THRES_UPD_BASE_OFFSET: u32 = 0;
    pub const PARAM_THRES_UPD_BASE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PARAM_THRES_UPD_VARY** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_thres_upd_vary(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_THRES_UPD_VARY_MASK)
            >> Self::PARAM_THRES_UPD_VARY_OFFSET
    }

    /// Write **PARAM_THRES_UPD_VARY** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_thres_upd_vary(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_THRES_UPD_VARY_MASK)
                | ((value << Self::PARAM_THRES_UPD_VARY_OFFSET) & Self::PARAM_THRES_UPD_VARY_MASK)
        });
    }

    /// **PARAM_THRES_UPD_BASE** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_thres_upd_base(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_THRES_UPD_BASE_MASK)
            >> Self::PARAM_THRES_UPD_BASE_OFFSET
    }

    /// Write **PARAM_THRES_UPD_BASE** `[15:0]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_thres_upd_base(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_THRES_UPD_BASE_MASK)
                | ((value << Self::PARAM_THRES_UPD_BASE_OFFSET) & Self::PARAM_THRES_UPD_BASE_MASK)
        });
    }
}

/// VAD_PARAM7 value type for batch read/modify/write operations
///
/// Use `vadparam7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam7Val {
    bits: u32,
}

impl VadParam7Val {
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
        Self { bits: 0x01487eb8 }
    }

    /// Read **PARAM_THRES_UPD_VARY** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_thres_upd_vary(&self) -> u32 {
        (self.bits & VadParam7::PARAM_THRES_UPD_VARY_MASK) >> VadParam7::PARAM_THRES_UPD_VARY_OFFSET
    }

    /// Read **PARAM_THRES_UPD_BASE** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_thres_upd_base(&self) -> u32 {
        (self.bits & VadParam7::PARAM_THRES_UPD_BASE_MASK) >> VadParam7::PARAM_THRES_UPD_BASE_OFFSET
    }

    /// Set **PARAM_THRES_UPD_VARY** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_thres_upd_vary(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam7::PARAM_THRES_UPD_VARY_MASK)
                | ((value << VadParam7::PARAM_THRES_UPD_VARY_OFFSET)
                    & VadParam7::PARAM_THRES_UPD_VARY_MASK),
        }
    }

    /// Set **PARAM_THRES_UPD_BASE** `[15:0]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_thres_upd_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam7::PARAM_THRES_UPD_BASE_MASK)
                | ((value << VadParam7::PARAM_THRES_UPD_BASE_OFFSET)
                    & VadParam7::PARAM_THRES_UPD_BASE_MASK),
        }
    }
}

impl Default for VadParam7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam7Val {
        VadParam7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam7Val) -> VadParam7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_PARAM8 Register
///
/// I2S VAD Parameter register
///
/// Address offset: 0x00a0
/// Reset value: 0x20005040
///
/// ## Fields
///
/// - **PARAM_FEATURE_BURST** `[31:16]` (RW)
///
///   VAD parameter
///
/// - **PARAM_THRES_UPD_BDH** `[15:8]` (RW)
///
///   Noise_std boundary high when updating threshold.
///
/// - **PARAM_THRES_UPD_BDL** `[7:0]` (RW)
///
///   Noise_std boundary low when updating threshold.
#[repr(transparent)]
pub struct VadParam8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadParam8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadParam8 {
    // ======== Field Constants ========

    /// **PARAM_FEATURE_BURST**
    ///
    /// VAD parameter
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PARAM_FEATURE_BURST_OFFSET: u32 = 16;
    pub const PARAM_FEATURE_BURST_MASK: u32 = 0xffff0000;

    /// **PARAM_THRES_UPD_BDH**
    ///
    /// Noise_std boundary high when updating threshold.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const PARAM_THRES_UPD_BDH_OFFSET: u32 = 8;
    pub const PARAM_THRES_UPD_BDH_MASK: u32 = 0x0000ff00;

    /// **PARAM_THRES_UPD_BDL**
    ///
    /// Noise_std boundary low when updating threshold.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PARAM_THRES_UPD_BDL_OFFSET: u32 = 0;
    pub const PARAM_THRES_UPD_BDL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PARAM_FEATURE_BURST** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn param_feature_burst(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_FEATURE_BURST_MASK)
            >> Self::PARAM_FEATURE_BURST_OFFSET
    }

    /// Write **PARAM_FEATURE_BURST** `[31:16]` (RW)
    ///
    /// VAD parameter
    #[inline(always)]
    pub fn write_param_feature_burst(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_FEATURE_BURST_MASK)
                | ((value << Self::PARAM_FEATURE_BURST_OFFSET) & Self::PARAM_FEATURE_BURST_MASK)
        });
    }

    /// **PARAM_THRES_UPD_BDH** `[15:8]` (RW)
    ///
    /// Noise_std boundary high when updating threshold.
    #[inline(always)]
    pub fn param_thres_upd_bdh(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_THRES_UPD_BDH_MASK)
            >> Self::PARAM_THRES_UPD_BDH_OFFSET
    }

    /// Write **PARAM_THRES_UPD_BDH** `[15:8]` (RW)
    ///
    /// Noise_std boundary high when updating threshold.
    #[inline(always)]
    pub fn write_param_thres_upd_bdh(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_THRES_UPD_BDH_MASK)
                | ((value << Self::PARAM_THRES_UPD_BDH_OFFSET) & Self::PARAM_THRES_UPD_BDH_MASK)
        });
    }

    /// **PARAM_THRES_UPD_BDL** `[7:0]` (RW)
    ///
    /// Noise_std boundary low when updating threshold.
    #[inline(always)]
    pub fn param_thres_upd_bdl(&self) -> u32 {
        (self.value.read_bits() & Self::PARAM_THRES_UPD_BDL_MASK)
            >> Self::PARAM_THRES_UPD_BDL_OFFSET
    }

    /// Write **PARAM_THRES_UPD_BDL** `[7:0]` (RW)
    ///
    /// Noise_std boundary low when updating threshold.
    #[inline(always)]
    pub fn write_param_thres_upd_bdl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARAM_THRES_UPD_BDL_MASK)
                | ((value << Self::PARAM_THRES_UPD_BDL_OFFSET) & Self::PARAM_THRES_UPD_BDL_MASK)
        });
    }
}

/// VAD_PARAM8 value type for batch read/modify/write operations
///
/// Use `vadparam8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadparam8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadParam8Val {
    bits: u32,
}

impl VadParam8Val {
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
        Self { bits: 0x20005040 }
    }

    /// Read **PARAM_FEATURE_BURST** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn param_feature_burst(&self) -> u32 {
        (self.bits & VadParam8::PARAM_FEATURE_BURST_MASK) >> VadParam8::PARAM_FEATURE_BURST_OFFSET
    }

    /// Read **PARAM_THRES_UPD_BDH** `[15:8]`
    ///
    /// Noise_std boundary high when updating threshold.
    #[inline(always)]
    pub const fn param_thres_upd_bdh(&self) -> u32 {
        (self.bits & VadParam8::PARAM_THRES_UPD_BDH_MASK) >> VadParam8::PARAM_THRES_UPD_BDH_OFFSET
    }

    /// Read **PARAM_THRES_UPD_BDL** `[7:0]`
    ///
    /// Noise_std boundary low when updating threshold.
    #[inline(always)]
    pub const fn param_thres_upd_bdl(&self) -> u32 {
        (self.bits & VadParam8::PARAM_THRES_UPD_BDL_MASK) >> VadParam8::PARAM_THRES_UPD_BDL_OFFSET
    }

    /// Set **PARAM_FEATURE_BURST** `[31:16]`
    ///
    /// VAD parameter
    #[inline(always)]
    pub const fn with_param_feature_burst(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam8::PARAM_FEATURE_BURST_MASK)
                | ((value << VadParam8::PARAM_FEATURE_BURST_OFFSET)
                    & VadParam8::PARAM_FEATURE_BURST_MASK),
        }
    }

    /// Set **PARAM_THRES_UPD_BDH** `[15:8]`
    ///
    /// Noise_std boundary high when updating threshold.
    #[inline(always)]
    pub const fn with_param_thres_upd_bdh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam8::PARAM_THRES_UPD_BDH_MASK)
                | ((value << VadParam8::PARAM_THRES_UPD_BDH_OFFSET)
                    & VadParam8::PARAM_THRES_UPD_BDH_MASK),
        }
    }

    /// Set **PARAM_THRES_UPD_BDL** `[7:0]`
    ///
    /// Noise_std boundary low when updating threshold.
    #[inline(always)]
    pub const fn with_param_thres_upd_bdl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VadParam8::PARAM_THRES_UPD_BDL_MASK)
                | ((value << VadParam8::PARAM_THRES_UPD_BDL_OFFSET)
                    & VadParam8::PARAM_THRES_UPD_BDL_MASK),
        }
    }
}

impl Default for VadParam8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VadParam8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadParam8Val {
        VadParam8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VadParam8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VadParam8Val) -> VadParam8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VAD_OB0 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **MAX_SIGNAL0_OB** `[31:16]` (RO)
///
///   Reg max signal0 observe
///
/// - **SILENT_COUNT_OB** `[15:8]` (RO)
///
///   Reg speech count observe
///
/// - **SPEECH_COUNT_OB** `[7:0]` (RO)
///
///   Reg silent count observe
#[repr(transparent)]
pub struct VadOb0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb0 {
    // ======== Field Constants ========

    /// **MAX_SIGNAL0_OB**
    ///
    /// Reg max signal0 observe
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const MAX_SIGNAL0_OB_OFFSET: u32 = 16;
    pub const MAX_SIGNAL0_OB_MASK: u32 = 0xffff0000;

    /// **SILENT_COUNT_OB**
    ///
    /// Reg speech count observe
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SILENT_COUNT_OB_OFFSET: u32 = 8;
    pub const SILENT_COUNT_OB_MASK: u32 = 0x0000ff00;

    /// **SPEECH_COUNT_OB**
    ///
    /// Reg silent count observe
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SPEECH_COUNT_OB_OFFSET: u32 = 0;
    pub const SPEECH_COUNT_OB_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **MAX_SIGNAL0_OB** `[31:16]` (RO)
    ///
    /// Reg max signal0 observe
    #[inline(always)]
    pub fn max_signal0_ob(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SIGNAL0_OB_MASK) >> Self::MAX_SIGNAL0_OB_OFFSET
    }

    /// **SILENT_COUNT_OB** `[15:8]` (RO)
    ///
    /// Reg speech count observe
    #[inline(always)]
    pub fn silent_count_ob(&self) -> u32 {
        (self.value.read_bits() & Self::SILENT_COUNT_OB_MASK) >> Self::SILENT_COUNT_OB_OFFSET
    }

    /// **SPEECH_COUNT_OB** `[7:0]` (RO)
    ///
    /// Reg silent count observe
    #[inline(always)]
    pub fn speech_count_ob(&self) -> u32 {
        (self.value.read_bits() & Self::SPEECH_COUNT_OB_MASK) >> Self::SPEECH_COUNT_OB_OFFSET
    }
}

/// VAD_OB0 value type for batch read/modify/write operations
///
/// Use `vadob0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb0Val {
    bits: u32,
}

impl VadOb0Val {
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

    /// Read **MAX_SIGNAL0_OB** `[31:16]`
    ///
    /// Reg max signal0 observe
    #[inline(always)]
    pub const fn max_signal0_ob(&self) -> u32 {
        (self.bits & VadOb0::MAX_SIGNAL0_OB_MASK) >> VadOb0::MAX_SIGNAL0_OB_OFFSET
    }

    /// Read **SILENT_COUNT_OB** `[15:8]`
    ///
    /// Reg speech count observe
    #[inline(always)]
    pub const fn silent_count_ob(&self) -> u32 {
        (self.bits & VadOb0::SILENT_COUNT_OB_MASK) >> VadOb0::SILENT_COUNT_OB_OFFSET
    }

    /// Read **SPEECH_COUNT_OB** `[7:0]`
    ///
    /// Reg silent count observe
    #[inline(always)]
    pub const fn speech_count_ob(&self) -> u32 {
        (self.bits & VadOb0::SPEECH_COUNT_OB_MASK) >> VadOb0::SPEECH_COUNT_OB_OFFSET
    }
}

impl Default for VadOb0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb0Val {
        VadOb0Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB1 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **MAX_SIGNAL2_OB** `[31:16]` (RO)
///
///   Reg max signal2 observe
///
/// - **MAX_SIGNAL1_OB** `[15:0]` (RO)
///
///   Reg max signal1 observe
#[repr(transparent)]
pub struct VadOb1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb1 {
    // ======== Field Constants ========

    /// **MAX_SIGNAL2_OB**
    ///
    /// Reg max signal2 observe
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const MAX_SIGNAL2_OB_OFFSET: u32 = 16;
    pub const MAX_SIGNAL2_OB_MASK: u32 = 0xffff0000;

    /// **MAX_SIGNAL1_OB**
    ///
    /// Reg max signal1 observe
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAX_SIGNAL1_OB_OFFSET: u32 = 0;
    pub const MAX_SIGNAL1_OB_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **MAX_SIGNAL2_OB** `[31:16]` (RO)
    ///
    /// Reg max signal2 observe
    #[inline(always)]
    pub fn max_signal2_ob(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SIGNAL2_OB_MASK) >> Self::MAX_SIGNAL2_OB_OFFSET
    }

    /// **MAX_SIGNAL1_OB** `[15:0]` (RO)
    ///
    /// Reg max signal1 observe
    #[inline(always)]
    pub fn max_signal1_ob(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SIGNAL1_OB_MASK) >> Self::MAX_SIGNAL1_OB_OFFSET
    }
}

/// VAD_OB1 value type for batch read/modify/write operations
///
/// Use `vadob1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb1Val {
    bits: u32,
}

impl VadOb1Val {
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

    /// Read **MAX_SIGNAL2_OB** `[31:16]`
    ///
    /// Reg max signal2 observe
    #[inline(always)]
    pub const fn max_signal2_ob(&self) -> u32 {
        (self.bits & VadOb1::MAX_SIGNAL2_OB_MASK) >> VadOb1::MAX_SIGNAL2_OB_OFFSET
    }

    /// Read **MAX_SIGNAL1_OB** `[15:0]`
    ///
    /// Reg max signal1 observe
    #[inline(always)]
    pub const fn max_signal1_ob(&self) -> u32 {
        (self.bits & VadOb1::MAX_SIGNAL1_OB_MASK) >> VadOb1::MAX_SIGNAL1_OB_OFFSET
    }
}

impl Default for VadOb1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb1Val {
        VadOb1Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB2 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **NOISE_AMP_OB** `[31:0]` (RO)
///
///   Reg noise_amp observe signal
#[repr(transparent)]
pub struct VadOb2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb2 {
    // ======== Field Constants ========

    /// **NOISE_AMP_OB**
    ///
    /// Reg noise_amp observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const NOISE_AMP_OB_OFFSET: u32 = 0;
    pub const NOISE_AMP_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **NOISE_AMP_OB** `[31:0]` (RO)
    ///
    /// Reg noise_amp observe signal
    #[inline(always)]
    pub fn noise_amp_ob(&self) -> u32 {
        (self.value.read_bits() & Self::NOISE_AMP_OB_MASK) >> Self::NOISE_AMP_OB_OFFSET
    }
}

/// VAD_OB2 value type for batch read/modify/write operations
///
/// Use `vadob2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb2Val {
    bits: u32,
}

impl VadOb2Val {
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

    /// Read **NOISE_AMP_OB** `[31:0]`
    ///
    /// Reg noise_amp observe signal
    #[inline(always)]
    pub const fn noise_amp_ob(&self) -> u32 {
        (self.bits & VadOb2::NOISE_AMP_OB_MASK) >> VadOb2::NOISE_AMP_OB_OFFSET
    }
}

impl Default for VadOb2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb2Val {
        VadOb2Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB3 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **NOISE_MEAN_OB** `[31:0]` (RO)
///
///   Reg noise_mean observe signal
#[repr(transparent)]
pub struct VadOb3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb3 {
    // ======== Field Constants ========

    /// **NOISE_MEAN_OB**
    ///
    /// Reg noise_mean observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const NOISE_MEAN_OB_OFFSET: u32 = 0;
    pub const NOISE_MEAN_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **NOISE_MEAN_OB** `[31:0]` (RO)
    ///
    /// Reg noise_mean observe signal
    #[inline(always)]
    pub fn noise_mean_ob(&self) -> u32 {
        (self.value.read_bits() & Self::NOISE_MEAN_OB_MASK) >> Self::NOISE_MEAN_OB_OFFSET
    }
}

/// VAD_OB3 value type for batch read/modify/write operations
///
/// Use `vadob3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb3Val {
    bits: u32,
}

impl VadOb3Val {
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

    /// Read **NOISE_MEAN_OB** `[31:0]`
    ///
    /// Reg noise_mean observe signal
    #[inline(always)]
    pub const fn noise_mean_ob(&self) -> u32 {
        (self.bits & VadOb3::NOISE_MEAN_OB_MASK) >> VadOb3::NOISE_MEAN_OB_OFFSET
    }
}

impl Default for VadOb3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb3Val {
        VadOb3Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB4 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **NOISE_STD_OB** `[31:0]` (RO)
///
///   Reg noise_std observe signal
#[repr(transparent)]
pub struct VadOb4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb4 {
    // ======== Field Constants ========

    /// **NOISE_STD_OB**
    ///
    /// Reg noise_std observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const NOISE_STD_OB_OFFSET: u32 = 0;
    pub const NOISE_STD_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **NOISE_STD_OB** `[31:0]` (RO)
    ///
    /// Reg noise_std observe signal
    #[inline(always)]
    pub fn noise_std_ob(&self) -> u32 {
        (self.value.read_bits() & Self::NOISE_STD_OB_MASK) >> Self::NOISE_STD_OB_OFFSET
    }
}

/// VAD_OB4 value type for batch read/modify/write operations
///
/// Use `vadob4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb4Val {
    bits: u32,
}

impl VadOb4Val {
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

    /// Read **NOISE_STD_OB** `[31:0]`
    ///
    /// Reg noise_std observe signal
    #[inline(always)]
    pub const fn noise_std_ob(&self) -> u32 {
        (self.bits & VadOb4::NOISE_STD_OB_MASK) >> VadOb4::NOISE_STD_OB_OFFSET
    }
}

impl Default for VadOb4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb4Val {
        VadOb4Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB5 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **OFFSET_OB** `[31:0]` (RO)
///
///   Reg offset observe signal
#[repr(transparent)]
pub struct VadOb5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb5 {
    // ======== Field Constants ========

    /// **OFFSET_OB**
    ///
    /// Reg offset observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OFFSET_OB_OFFSET: u32 = 0;
    pub const OFFSET_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OFFSET_OB** `[31:0]` (RO)
    ///
    /// Reg offset observe signal
    #[inline(always)]
    pub fn offset_ob(&self) -> u32 {
        (self.value.read_bits() & Self::OFFSET_OB_MASK) >> Self::OFFSET_OB_OFFSET
    }
}

/// VAD_OB5 value type for batch read/modify/write operations
///
/// Use `vadob5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb5Val {
    bits: u32,
}

impl VadOb5Val {
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

    /// Read **OFFSET_OB** `[31:0]`
    ///
    /// Reg offset observe signal
    #[inline(always)]
    pub const fn offset_ob(&self) -> u32 {
        (self.bits & VadOb5::OFFSET_OB_MASK) >> VadOb5::OFFSET_OB_OFFSET
    }
}

impl Default for VadOb5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb5Val {
        VadOb5Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB6 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **THRESHOLD_OB** `[31:0]` (RO)
///
///   Reg threshold observe signal
#[repr(transparent)]
pub struct VadOb6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb6 {
    // ======== Field Constants ========

    /// **THRESHOLD_OB**
    ///
    /// Reg threshold observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const THRESHOLD_OB_OFFSET: u32 = 0;
    pub const THRESHOLD_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **THRESHOLD_OB** `[31:0]` (RO)
    ///
    /// Reg threshold observe signal
    #[inline(always)]
    pub fn threshold_ob(&self) -> u32 {
        (self.value.read_bits() & Self::THRESHOLD_OB_MASK) >> Self::THRESHOLD_OB_OFFSET
    }
}

/// VAD_OB6 value type for batch read/modify/write operations
///
/// Use `vadob6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb6Val {
    bits: u32,
}

impl VadOb6Val {
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

    /// Read **THRESHOLD_OB** `[31:0]`
    ///
    /// Reg threshold observe signal
    #[inline(always)]
    pub const fn threshold_ob(&self) -> u32 {
        (self.bits & VadOb6::THRESHOLD_OB_MASK) >> VadOb6::THRESHOLD_OB_OFFSET
    }
}

impl Default for VadOb6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb6Val {
        VadOb6Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB7 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **ENERGY_LOW_OB** `[31:0]` (RO)
///
///   Reg energy bit 31~0 observe signal
#[repr(transparent)]
pub struct VadOb7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb7 {
    // ======== Field Constants ========

    /// **ENERGY_LOW_OB**
    ///
    /// Reg energy bit 31~0 observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ENERGY_LOW_OB_OFFSET: u32 = 0;
    pub const ENERGY_LOW_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ENERGY_LOW_OB** `[31:0]` (RO)
    ///
    /// Reg energy bit 31~0 observe signal
    #[inline(always)]
    pub fn energy_low_ob(&self) -> u32 {
        (self.value.read_bits() & Self::ENERGY_LOW_OB_MASK) >> Self::ENERGY_LOW_OB_OFFSET
    }
}

/// VAD_OB7 value type for batch read/modify/write operations
///
/// Use `vadob7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb7Val {
    bits: u32,
}

impl VadOb7Val {
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

    /// Read **ENERGY_LOW_OB** `[31:0]`
    ///
    /// Reg energy bit 31~0 observe signal
    #[inline(always)]
    pub const fn energy_low_ob(&self) -> u32 {
        (self.bits & VadOb7::ENERGY_LOW_OB_MASK) >> VadOb7::ENERGY_LOW_OB_OFFSET
    }
}

impl Default for VadOb7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb7Val {
        VadOb7Val::from_bits(self.value.read_bits())
    }
}

/// VAD_OB8 Register
///
/// I2S VAD Observe register
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **ENERGY_HIGH_OB** `[31:0]` (RO)
///
///   Reg energy bit 63~32 observe signal
#[repr(transparent)]
pub struct VadOb8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VadOb8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VadOb8 {
    // ======== Field Constants ========

    /// **ENERGY_HIGH_OB**
    ///
    /// Reg energy bit 63~32 observe signal
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ENERGY_HIGH_OB_OFFSET: u32 = 0;
    pub const ENERGY_HIGH_OB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ENERGY_HIGH_OB** `[31:0]` (RO)
    ///
    /// Reg energy bit 63~32 observe signal
    #[inline(always)]
    pub fn energy_high_ob(&self) -> u32 {
        (self.value.read_bits() & Self::ENERGY_HIGH_OB_MASK) >> Self::ENERGY_HIGH_OB_OFFSET
    }
}

/// VAD_OB8 value type for batch read/modify/write operations
///
/// Use `vadob8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vadob8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VadOb8Val {
    bits: u32,
}

impl VadOb8Val {
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

    /// Read **ENERGY_HIGH_OB** `[31:0]`
    ///
    /// Reg energy bit 63~32 observe signal
    #[inline(always)]
    pub const fn energy_high_ob(&self) -> u32 {
        (self.bits & VadOb8::ENERGY_HIGH_OB_MASK) >> VadOb8::ENERGY_HIGH_OB_OFFSET
    }
}

impl Default for VadOb8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VadOb8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VadOb8Val {
        VadOb8Val::from_bits(self.value.read_bits())
    }
}

/// CLK_GATE Register
///
/// Clock gate register
///
/// Address offset: 0x00f8
/// Reset value: 0x0000000a
///
/// ## Fields
///
/// - **RX_REG_CG_FORCE_ON** `[3]` (RW)
///
///   I2S rx reg clock gate force on register
///
/// - **RX_MEM_CG_FORCE_ON** `[2]` (RW)
///
///   I2S rx mem clock gate force on register
///
/// - **VAD_CG_FORCE_ON** `[1]` (RW)
///
///   VAD clock gate force on register
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

    /// **RX_REG_CG_FORCE_ON**
    ///
    /// I2S rx reg clock gate force on register
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_REG_CG_FORCE_ON_OFFSET: u32 = 3;
    pub const RX_REG_CG_FORCE_ON_MASK: u32 = 0x00000008;
    pub const RX_REG_CG_FORCE_ON: u32 = 1 << 3;

    /// **RX_MEM_CG_FORCE_ON**
    ///
    /// I2S rx mem clock gate force on register
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_MEM_CG_FORCE_ON_OFFSET: u32 = 2;
    pub const RX_MEM_CG_FORCE_ON_MASK: u32 = 0x00000004;
    pub const RX_MEM_CG_FORCE_ON: u32 = 1 << 2;

    /// **VAD_CG_FORCE_ON**
    ///
    /// VAD clock gate force on register
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VAD_CG_FORCE_ON_OFFSET: u32 = 1;
    pub const VAD_CG_FORCE_ON_MASK: u32 = 0x00000002;
    pub const VAD_CG_FORCE_ON: u32 = 1 << 1;

    /// **CLK_EN**
    ///
    /// set this bit to enable clock gate
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_REG_CG_FORCE_ON** `[3]` (RW)
    ///
    /// I2S rx reg clock gate force on register
    #[inline(always)]
    pub fn rx_reg_cg_force_on(&self) -> bool {
        (self.value.read_bits() & Self::RX_REG_CG_FORCE_ON_MASK) != 0
    }

    /// Set RX_REG_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_reg_cg_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_REG_CG_FORCE_ON_MASK);
    }

    /// Clear RX_REG_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_reg_cg_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_REG_CG_FORCE_ON_MASK);
    }

    /// **RX_MEM_CG_FORCE_ON** `[2]` (RW)
    ///
    /// I2S rx mem clock gate force on register
    #[inline(always)]
    pub fn rx_mem_cg_force_on(&self) -> bool {
        (self.value.read_bits() & Self::RX_MEM_CG_FORCE_ON_MASK) != 0
    }

    /// Set RX_MEM_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_mem_cg_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_MEM_CG_FORCE_ON_MASK);
    }

    /// Clear RX_MEM_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_mem_cg_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_MEM_CG_FORCE_ON_MASK);
    }

    /// **VAD_CG_FORCE_ON** `[1]` (RW)
    ///
    /// VAD clock gate force on register
    #[inline(always)]
    pub fn vad_cg_force_on(&self) -> bool {
        (self.value.read_bits() & Self::VAD_CG_FORCE_ON_MASK) != 0
    }

    /// Set VAD_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vad_cg_force_on(&self) {
        self.value.modify_bits(|r| r | Self::VAD_CG_FORCE_ON_MASK);
    }

    /// Clear VAD_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vad_cg_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::VAD_CG_FORCE_ON_MASK);
    }

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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000000a }
    }

    /// Read **RX_REG_CG_FORCE_ON** `[3:3]`
    ///
    /// I2S rx reg clock gate force on register
    #[inline(always)]
    pub const fn rx_reg_cg_force_on(&self) -> bool {
        (self.bits & ClkGate::RX_REG_CG_FORCE_ON_MASK) != 0
    }

    /// Read **RX_MEM_CG_FORCE_ON** `[2:2]`
    ///
    /// I2S rx mem clock gate force on register
    #[inline(always)]
    pub const fn rx_mem_cg_force_on(&self) -> bool {
        (self.bits & ClkGate::RX_MEM_CG_FORCE_ON_MASK) != 0
    }

    /// Read **VAD_CG_FORCE_ON** `[1:1]`
    ///
    /// VAD clock gate force on register
    #[inline(always)]
    pub const fn vad_cg_force_on(&self) -> bool {
        (self.bits & ClkGate::VAD_CG_FORCE_ON_MASK) != 0
    }

    /// Read **CLK_EN** `[0:0]`
    ///
    /// set this bit to enable clock gate
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClkGate::CLK_EN_MASK) != 0
    }

    /// Set **RX_REG_CG_FORCE_ON** `[3:3]`
    ///
    /// I2S rx reg clock gate force on register
    #[inline(always)]
    pub const fn with_rx_reg_cg_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGate::RX_REG_CG_FORCE_ON_MASK
            } else {
                self.bits & !ClkGate::RX_REG_CG_FORCE_ON_MASK
            },
        }
    }

    /// Set **RX_MEM_CG_FORCE_ON** `[2:2]`
    ///
    /// I2S rx mem clock gate force on register
    #[inline(always)]
    pub const fn with_rx_mem_cg_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGate::RX_MEM_CG_FORCE_ON_MASK
            } else {
                self.bits & !ClkGate::RX_MEM_CG_FORCE_ON_MASK
            },
        }
    }

    /// Set **VAD_CG_FORCE_ON** `[1:1]`
    ///
    /// VAD clock gate force on register
    #[inline(always)]
    pub const fn with_vad_cg_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGate::VAD_CG_FORCE_ON_MASK
            } else {
                self.bits & !ClkGate::VAD_CG_FORCE_ON_MASK
            },
        }
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
        Self::reset_val()
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
/// Address offset: 0x00fc
/// Reset value: 0x02305040
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
        Self { bits: 0x02305040 }
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
    /// 0x0000
    /// I2S VAD Configure register
    pub vad_conf: VadConf,
    /// 0x0004
    /// I2S VAD Result register
    pub vad_result: VadResult,
    /// 0x0008
    /// I2S VAD Observe register
    pub rx_mem_conf: RxMemConf,
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
    _reserved0: [u8; 4],
    /// 0x0020
    /// I2S RX configure register
    pub rx_conf: RxConf,
    _reserved1: [u8; 4],
    /// 0x0028
    /// I2S RX configure register 1
    pub rx_conf1: RxConf1,
    _reserved2: [u8; 36],
    /// 0x0050
    /// I2S TX TDM mode control register
    pub rx_tdm_ctrl: RxTdmCtrl,
    _reserved3: [u8; 4],
    /// 0x0058
    /// I2S RX timing control register
    pub rx_timing: RxTiming,
    _reserved4: [u8; 4],
    /// 0x0060
    /// I2S HUNG configure register.
    pub lc_hung_conf: LcHungConf,
    /// 0x0064
    /// I2S RX data number control register.
    pub rxeof_num: RxeofNum,
    /// 0x0068
    /// I2S signal data register
    pub conf_sigle_data: ConfSigleData,
    _reserved5: [u8; 4],
    /// 0x0070
    /// I2S RX configure register
    pub rx_pdm_conf: RxPdmConf,
    /// 0x0074
    /// I2S ECO register
    pub eco_low: EcoLow,
    /// 0x0078
    /// I2S ECO register
    pub eco_high: EcoHigh,
    /// 0x007c
    /// I2S ECO register
    pub eco_conf: EcoConf,
    /// 0x0080
    /// I2S VAD Parameter register
    pub vad_param0: VadParam0,
    /// 0x0084
    /// I2S VAD Parameter register
    pub vad_param1: VadParam1,
    /// 0x0088
    /// I2S VAD Parameter register
    pub vad_param2: VadParam2,
    /// 0x008c
    /// I2S VAD Parameter register
    pub vad_param3: VadParam3,
    /// 0x0090
    /// I2S VAD Parameter register
    pub vad_param4: VadParam4,
    /// 0x0094
    /// I2S VAD Parameter register
    pub vad_param5: VadParam5,
    /// 0x0098
    /// I2S VAD Parameter register
    pub vad_param6: VadParam6,
    /// 0x009c
    /// I2S VAD Parameter register
    pub vad_param7: VadParam7,
    /// 0x00a0
    /// I2S VAD Parameter register
    pub vad_param8: VadParam8,
    _reserved6: [u8; 12],
    /// 0x00b0
    /// I2S VAD Observe register
    pub vad_ob0: VadOb0,
    /// 0x00b4
    /// I2S VAD Observe register
    pub vad_ob1: VadOb1,
    /// 0x00b8
    /// I2S VAD Observe register
    pub vad_ob2: VadOb2,
    /// 0x00bc
    /// I2S VAD Observe register
    pub vad_ob3: VadOb3,
    /// 0x00c0
    /// I2S VAD Observe register
    pub vad_ob4: VadOb4,
    /// 0x00c4
    /// I2S VAD Observe register
    pub vad_ob5: VadOb5,
    /// 0x00c8
    /// I2S VAD Observe register
    pub vad_ob6: VadOb6,
    /// 0x00cc
    /// I2S VAD Observe register
    pub vad_ob7: VadOb7,
    /// 0x00d0
    /// I2S VAD Observe register
    pub vad_ob8: VadOb8,
    _reserved7: [u8; 36],
    /// 0x00f8
    /// Clock gate register
    pub clk_gate: ClkGate,
    /// 0x00fc
    /// Version control register
    pub date: Date,
}
