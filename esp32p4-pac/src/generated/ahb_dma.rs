//! AHB_DMA
//! AHB_DMA Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50085000;

/// IN_INT_RAW_CH0 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
///
/// - **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
///
/// - **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
///
///   The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
///
/// - **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
///
/// - **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
///
/// - **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
///
/// - **IN_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
#[repr(transparent)]
pub struct InIntRawCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntRawCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntRawCh0 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_RAW: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_RAW: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_RAW_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_RAW: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_RAW: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_RAW_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_RAW: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub fn in_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_RAW_MASK);
    }

    /// Clear IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_RAW_MASK);
    }
}

/// IN_INT_RAW_CH0 value type for batch read/modify/write operations
///
/// Use `inintrawch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintrawch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntRawCh0Val {
    bits: u32,
}

impl InIntRawCh0Val {
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

    /// Read **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn in_done_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh0::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::INFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::INFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::INFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::INFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::IN_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::IN_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::IN_ERR_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::IN_ERR_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::IN_SUC_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::IN_SUC_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh0::IN_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh0::IN_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for InIntRawCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntRawCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntRawCh0Val {
        InIntRawCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntRawCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntRawCh0Val) -> InIntRawCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_ST_CH0 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
///
///   The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntStCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntStCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntStCh0 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ST_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ST: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ST_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ST: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ST_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ST: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ST: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ST_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ST: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

/// IN_INT_ST_CH0 value type for batch read/modify/write operations
///
/// Use `inintstch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintstch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntStCh0Val {
    bits: u32,
}

impl InIntStCh0Val {
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

    /// Read **INFIFO_UDF_CH_INT_ST** `[6:6]`
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh0::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for InIntStCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntStCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntStCh0Val {
        InIntStCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_INT_ENA_CH0 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
///
///   The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntEnaCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntEnaCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntEnaCh0 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ENA: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ENA: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ENA_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ENA: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ENA: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ENA_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ENA: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_ENA_MASK);
    }

    /// Clear IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_ENA_MASK);
    }
}

/// IN_INT_ENA_CH0 value type for batch read/modify/write operations
///
/// Use `inintenach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintenach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntEnaCh0Val {
    bits: u32,
}

impl InIntEnaCh0Val {
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

    /// Read **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh0::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::INFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::INFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::INFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::INFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::IN_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::IN_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::IN_ERR_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::IN_ERR_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::IN_SUC_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::IN_SUC_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh0::IN_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh0::IN_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for InIntEnaCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntEnaCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntEnaCh0Val {
        InIntEnaCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntEnaCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntEnaCh0Val) -> InIntEnaCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_CLR_CH0 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntClrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntClrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntClrCh0 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_CLR: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_CLR: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_CLR_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_CLR: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_CLR: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_CLR_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_CLR: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **INFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_EMPTY_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_empty_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_EMPTY_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_ERR_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_err_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_ERR_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_SUC_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_suc_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_SUC_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_done_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DONE_CH_INT_CLR_MASK);
    }
}

/// IN_INT_CLR_CH0 value type for batch read/modify/write operations
///
/// Use `inintclrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintclrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntClrCh0Val {
    bits: u32,
}

impl InIntClrCh0Val {
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

    /// Set **INFIFO_UDF_CH_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::INFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::INFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::INFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::INFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::IN_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::IN_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::IN_ERR_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::IN_ERR_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::IN_SUC_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::IN_SUC_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh0::IN_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh0::IN_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for InIntClrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntClrCh0 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntClrCh0Val) {
        self.value.write_bits(val.bits())
    }
}

/// IN_INT_RAW_CH1 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
///
/// - **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
///
/// - **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
///
///   The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
///
/// - **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
///
/// - **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
///
/// - **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
///
/// - **IN_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
#[repr(transparent)]
pub struct InIntRawCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntRawCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntRawCh1 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_RAW: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_RAW: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_RAW_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_RAW: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_RAW: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_RAW_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_RAW: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub fn in_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_RAW_MASK);
    }

    /// Clear IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_RAW_MASK);
    }
}

/// IN_INT_RAW_CH1 value type for batch read/modify/write operations
///
/// Use `inintrawch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintrawch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntRawCh1Val {
    bits: u32,
}

impl InIntRawCh1Val {
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

    /// Read **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn in_done_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh1::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::INFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::INFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::INFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::INFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::IN_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::IN_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::IN_ERR_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::IN_ERR_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::IN_SUC_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::IN_SUC_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh1::IN_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh1::IN_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for InIntRawCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntRawCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntRawCh1Val {
        InIntRawCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntRawCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntRawCh1Val) -> InIntRawCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_ST_CH1 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
///
///   The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntStCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntStCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntStCh1 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ST_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ST: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ST_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ST: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ST_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ST: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ST: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ST_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ST: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

/// IN_INT_ST_CH1 value type for batch read/modify/write operations
///
/// Use `inintstch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintstch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntStCh1Val {
    bits: u32,
}

impl InIntStCh1Val {
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

    /// Read **INFIFO_UDF_CH_INT_ST** `[6:6]`
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh1::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for InIntStCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntStCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntStCh1Val {
        InIntStCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_INT_ENA_CH1 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
///
///   The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntEnaCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntEnaCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntEnaCh1 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ENA: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ENA: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ENA_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ENA: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ENA: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ENA_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ENA: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_ENA_MASK);
    }

    /// Clear IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_ENA_MASK);
    }
}

/// IN_INT_ENA_CH1 value type for batch read/modify/write operations
///
/// Use `inintenach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintenach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntEnaCh1Val {
    bits: u32,
}

impl InIntEnaCh1Val {
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

    /// Read **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh1::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::INFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::INFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::INFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::INFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::IN_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::IN_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::IN_ERR_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::IN_ERR_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::IN_SUC_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::IN_SUC_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh1::IN_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh1::IN_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for InIntEnaCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntEnaCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntEnaCh1Val {
        InIntEnaCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntEnaCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntEnaCh1Val) -> InIntEnaCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_CLR_CH1 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntClrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntClrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntClrCh1 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_CLR: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_CLR: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_CLR_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_CLR: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_CLR: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_CLR_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_CLR: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **INFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_EMPTY_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_empty_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_EMPTY_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_ERR_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_err_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_ERR_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_SUC_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_suc_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_SUC_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_done_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DONE_CH_INT_CLR_MASK);
    }
}

/// IN_INT_CLR_CH1 value type for batch read/modify/write operations
///
/// Use `inintclrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintclrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntClrCh1Val {
    bits: u32,
}

impl InIntClrCh1Val {
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

    /// Set **INFIFO_UDF_CH_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::INFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::INFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::INFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::INFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::IN_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::IN_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::IN_ERR_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::IN_ERR_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::IN_SUC_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::IN_SUC_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh1::IN_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh1::IN_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for InIntClrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntClrCh1 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntClrCh1Val) {
        self.value.write_bits(val.bits())
    }
}

/// IN_INT_RAW_CH2 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
///
/// - **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
///
/// - **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
///
///   The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
///
/// - **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
///
/// - **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
///
/// - **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
///
/// - **IN_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
#[repr(transparent)]
pub struct InIntRawCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntRawCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntRawCh2 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_RAW: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_RAW: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_RAW_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_RAW: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_RAW: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_RAW_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_RAW: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_RAW** `[6]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **INFIFO_OVF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_RAW** `[4]` (RW)
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_RAW_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_RAW_MASK);
    }

    /// **IN_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub fn in_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_RAW_MASK);
    }

    /// Clear IN_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_RAW_MASK);
    }
}

/// IN_INT_RAW_CH2 value type for batch read/modify/write operations
///
/// Use `inintrawch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintrawch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntRawCh2Val {
    bits: u32,
}

impl InIntRawCh2Val {
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

    /// Read **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::INFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::INFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::IN_DSCR_EMPTY_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::IN_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::IN_ERR_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::IN_SUC_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn in_done_ch_int_raw(&self) -> bool {
        (self.bits & InIntRawCh2::IN_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_RAW** `[6:6]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::INFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::INFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::INFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::INFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::IN_DSCR_EMPTY_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::IN_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::IN_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::IN_ERR_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::IN_ERR_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::IN_SUC_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::IN_SUC_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0.
    #[inline(always)]
    pub const fn with_in_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntRawCh2::IN_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !InIntRawCh2::IN_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for InIntRawCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntRawCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntRawCh2Val {
        InIntRawCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntRawCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntRawCh2Val) -> InIntRawCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_ST_CH2 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
///
///   The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntStCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntStCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntStCh2 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ST_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ST: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ST_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ST: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ST_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ST: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ST: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ST_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ST: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ST** `[6]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **INFIFO_OVF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_EMPTY_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// **IN_DSCR_ERR_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **IN_ERR_EOF_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_SUC_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// **IN_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

/// IN_INT_ST_CH2 value type for batch read/modify/write operations
///
/// Use `inintstch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintstch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntStCh2Val {
    bits: u32,
}

impl InIntStCh2Val {
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

    /// Read **INFIFO_UDF_CH_INT_ST** `[6:6]`
    ///
    /// The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::INFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::INFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::IN_DSCR_EMPTY_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::IN_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::IN_ERR_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::IN_SUC_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_st(&self) -> bool {
        (self.bits & InIntStCh2::IN_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for InIntStCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntStCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntStCh2Val {
        InIntStCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_INT_ENA_CH2 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
///
///   The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntEnaCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntEnaCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntEnaCh2 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_ENA: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_ENA: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_ENA_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_ENA: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_ENA: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_ENA_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_ENA: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INFIFO_UDF_CH_INT_ENA** `[6]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **INFIFO_OVF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear INFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_infifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::INFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_EMPTY_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_EMPTY_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_empty_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_EMPTY_CH_INT_ENA_MASK);
    }

    /// **IN_DSCR_ERR_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear IN_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **IN_ERR_EOF_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_ERR_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_err_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_ERR_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_SUC_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// Clear IN_SUC_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_suc_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_SUC_EOF_CH_INT_ENA_MASK);
    }

    /// **IN_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DONE_CH_INT_ENA_MASK);
    }

    /// Clear IN_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DONE_CH_INT_ENA_MASK);
    }
}

/// IN_INT_ENA_CH2 value type for batch read/modify/write operations
///
/// Use `inintenach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintenach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntEnaCh2Val {
    bits: u32,
}

impl InIntEnaCh2Val {
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

    /// Read **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::INFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn infifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::INFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_empty_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::IN_DSCR_EMPTY_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::IN_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_err_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::IN_ERR_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_suc_eof_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::IN_SUC_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn in_done_ch_int_ena(&self) -> bool {
        (self.bits & InIntEnaCh2::IN_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **INFIFO_UDF_CH_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::INFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::INFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::INFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::INFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::IN_DSCR_EMPTY_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::IN_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::IN_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::IN_ERR_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::IN_ERR_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::IN_SUC_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::IN_SUC_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntEnaCh2::IN_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !InIntEnaCh2::IN_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for InIntEnaCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntEnaCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InIntEnaCh2Val {
        InIntEnaCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntEnaCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InIntEnaCh2Val) -> InIntEnaCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_INT_CLR_CH2 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **INFIFO_UDF_CH_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
///
/// - **INFIFO_OVF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
///
/// - **IN_DSCR_EMPTY_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
///
/// - **IN_DSCR_ERR_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
///
/// - **IN_ERR_EOF_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
///
/// - **IN_SUC_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
///
/// - **IN_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the IN_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct InIntClrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InIntClrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InIntClrCh2 {
    // ======== Field Constants ========

    /// **INFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const INFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 6;
    pub const INFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000040;
    pub const INFIFO_UDF_CH_INT_CLR: u32 = 1 << 6;

    /// **INFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const INFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const INFIFO_OVF_CH_INT_CLR: u32 = 1 << 5;

    /// **IN_DSCR_EMPTY_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IN_DSCR_EMPTY_CH_INT_CLR_OFFSET: u32 = 4;
    pub const IN_DSCR_EMPTY_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const IN_DSCR_EMPTY_CH_INT_CLR: u32 = 1 << 4;

    /// **IN_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 3;
    pub const IN_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const IN_DSCR_ERR_CH_INT_CLR: u32 = 1 << 3;

    /// **IN_ERR_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IN_ERR_EOF_CH_INT_CLR_OFFSET: u32 = 2;
    pub const IN_ERR_EOF_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const IN_ERR_EOF_CH_INT_CLR: u32 = 1 << 2;

    /// **IN_SUC_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_SUC_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const IN_SUC_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const IN_SUC_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **IN_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const IN_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const IN_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **INFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_infifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::INFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_EMPTY_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_empty_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_EMPTY_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_ERR_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_err_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_ERR_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_SUC_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_suc_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_SUC_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **IN_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_in_done_ch_int_clr(&self) {
        self.value.write_bits(Self::IN_DONE_CH_INT_CLR_MASK);
    }
}

/// IN_INT_CLR_CH2 value type for batch read/modify/write operations
///
/// Use `inintclrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inintclrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InIntClrCh2Val {
    bits: u32,
}

impl InIntClrCh2Val {
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

    /// Set **INFIFO_UDF_CH_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::INFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::INFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **INFIFO_OVF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_infifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::INFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::INFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_EMPTY_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_empty_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::IN_DSCR_EMPTY_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DSCR_ERR_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::IN_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::IN_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_ERR_EOF_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_err_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::IN_ERR_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::IN_ERR_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_SUC_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_suc_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::IN_SUC_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::IN_SUC_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **IN_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_in_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InIntClrCh2::IN_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !InIntClrCh2::IN_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for InIntClrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InIntClrCh2 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InIntClrCh2Val) {
        self.value.write_bits(val.bits())
    }
}

/// OUT_INT_RAW_CH0 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
///
/// - **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
///
/// - **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
///
/// - **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
///
/// - **OUT_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
///
/// - **OUT_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
#[repr(transparent)]
pub struct OutIntRawCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntRawCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntRawCh0 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_RAW: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_RAW: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_RAW_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_RAW: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_RAW: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **OUT_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub fn out_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub fn out_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_RAW_MASK);
    }
}

/// OUT_INT_RAW_CH0 value type for batch read/modify/write operations
///
/// Use `outintrawch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintrawch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntRawCh0Val {
    bits: u32,
}

impl OutIntRawCh0Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn out_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn out_done_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh0::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUTFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUTFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUTFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUTFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUT_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUT_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUT_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUT_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh0::OUT_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh0::OUT_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for OutIntRawCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntRawCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntRawCh0Val {
        OutIntRawCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntRawCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntRawCh0Val) -> OutIntRawCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_ST_CH0 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntStCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntStCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntStCh0 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ST_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ST: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ST_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ST: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ST_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ST: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ST: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **OUT_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

/// OUT_INT_ST_CH0 value type for batch read/modify/write operations
///
/// Use `outintstch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintstch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntStCh0Val {
    bits: u32,
}

impl OutIntStCh0Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh0::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for OutIntStCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntStCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntStCh0Val {
        OutIntStCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_INT_ENA_CH0 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntEnaCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntEnaCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntEnaCh0 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ENA: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ENA: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ENA_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ENA: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ENA: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **OUT_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_ENA_MASK);
    }
}

/// OUT_INT_ENA_CH0 value type for batch read/modify/write operations
///
/// Use `outintenach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintenach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntEnaCh0Val {
    bits: u32,
}

impl OutIntEnaCh0Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh0::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUTFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUTFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUTFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUTFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUT_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUT_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUT_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUT_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh0::OUT_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh0::OUT_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for OutIntEnaCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntEnaCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntEnaCh0Val {
        OutIntEnaCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntEnaCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntEnaCh0Val) -> OutIntEnaCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_CLR_CH0 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntClrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntClrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntClrCh0 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_CLR: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_CLR: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_CLR_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_CLR: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_CLR: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUTFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_TOTAL_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_total_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_TOTAL_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_done_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DONE_CH_INT_CLR_MASK);
    }
}

/// OUT_INT_CLR_CH0 value type for batch read/modify/write operations
///
/// Use `outintclrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintclrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntClrCh0Val {
    bits: u32,
}

impl OutIntClrCh0Val {
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

    /// Set **OUTFIFO_UDF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUTFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUTFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUTFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUTFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUT_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUT_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUT_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUT_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh0::OUT_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh0::OUT_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for OutIntClrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntClrCh0 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntClrCh0Val) {
        self.value.write_bits(val.bits())
    }
}

/// OUT_INT_RAW_CH1 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
///
/// - **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
///
/// - **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
///
/// - **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
///
/// - **OUT_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
///
/// - **OUT_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
#[repr(transparent)]
pub struct OutIntRawCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntRawCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntRawCh1 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_RAW: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_RAW: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_RAW_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_RAW: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_RAW: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **OUT_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub fn out_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub fn out_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_RAW_MASK);
    }
}

/// OUT_INT_RAW_CH1 value type for batch read/modify/write operations
///
/// Use `outintrawch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintrawch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntRawCh1Val {
    bits: u32,
}

impl OutIntRawCh1Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn out_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn out_done_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh1::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUTFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUTFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUTFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUTFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUT_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUT_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUT_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUT_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh1::OUT_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh1::OUT_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for OutIntRawCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntRawCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntRawCh1Val {
        OutIntRawCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntRawCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntRawCh1Val) -> OutIntRawCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_ST_CH1 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntStCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntStCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntStCh1 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ST_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ST: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ST_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ST: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ST_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ST: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ST: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **OUT_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

/// OUT_INT_ST_CH1 value type for batch read/modify/write operations
///
/// Use `outintstch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintstch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntStCh1Val {
    bits: u32,
}

impl OutIntStCh1Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh1::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for OutIntStCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntStCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntStCh1Val {
        OutIntStCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_INT_ENA_CH1 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntEnaCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntEnaCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntEnaCh1 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ENA: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ENA: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ENA_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ENA: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ENA: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **OUT_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_ENA_MASK);
    }
}

/// OUT_INT_ENA_CH1 value type for batch read/modify/write operations
///
/// Use `outintenach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintenach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntEnaCh1Val {
    bits: u32,
}

impl OutIntEnaCh1Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh1::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUTFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUTFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUTFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUTFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUT_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUT_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUT_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUT_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh1::OUT_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh1::OUT_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for OutIntEnaCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntEnaCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntEnaCh1Val {
        OutIntEnaCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntEnaCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntEnaCh1Val) -> OutIntEnaCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_CLR_CH1 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntClrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntClrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntClrCh1 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_CLR: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_CLR: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_CLR_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_CLR: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_CLR: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUTFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_TOTAL_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_total_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_TOTAL_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_done_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DONE_CH_INT_CLR_MASK);
    }
}

/// OUT_INT_CLR_CH1 value type for batch read/modify/write operations
///
/// Use `outintclrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintclrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntClrCh1Val {
    bits: u32,
}

impl OutIntClrCh1Val {
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

    /// Set **OUTFIFO_UDF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUTFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUTFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUTFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUTFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUT_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUT_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUT_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUT_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh1::OUT_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh1::OUT_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for OutIntClrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntClrCh1 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntClrCh1Val) {
        self.value.write_bits(val.bits())
    }
}

/// OUT_INT_RAW_CH2 Register
///
/// Raw status interrupt of channel 0
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
///
/// - **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
///
///   This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
///
/// - **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
///
/// - **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
///
/// - **OUT_EOF_CH_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
///
/// - **OUT_DONE_CH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
#[repr(transparent)]
pub struct OutIntRawCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntRawCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntRawCh2 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_RAW_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_RAW_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_RAW: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_RAW_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_RAW_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_RAW: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_RAW_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_RAW_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_RAW: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_RAW_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_RAW_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_RAW: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_RAW_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_RAW_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_RAW: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_RAW_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_RAW_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_RAW** `[5]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_RAW_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_RAW** `[4]` (RW)
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_RAW_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_RAW_MASK);
    }

    /// **OUT_EOF_CH_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub fn out_eof_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// Clear OUT_EOF_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_RAW_MASK);
    }

    /// **OUT_DONE_CH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub fn out_done_ch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_RAW_MASK);
    }

    /// Clear OUT_DONE_CH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_RAW_MASK);
    }
}

/// OUT_INT_RAW_CH2 value type for batch read/modify/write operations
///
/// Use `outintrawch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintrawch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntRawCh2Val {
    bits: u32,
}

impl OutIntRawCh2Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUTFIFO_UDF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUTFIFO_OVF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUT_TOTAL_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUT_DSCR_ERR_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn out_eof_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUT_EOF_CH_INT_RAW_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn out_done_ch_int_raw(&self) -> bool {
        (self.bits & OutIntRawCh2::OUT_DONE_CH_INT_RAW_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_RAW** `[5:5]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUTFIFO_UDF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUTFIFO_UDF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_RAW** `[4:4]`
    ///
    /// This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUTFIFO_OVF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUTFIFO_OVF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUT_TOTAL_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUT_DSCR_ERR_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUT_DSCR_ERR_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUT_EOF_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUT_EOF_CH_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0.
    #[inline(always)]
    pub const fn with_out_done_ch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntRawCh2::OUT_DONE_CH_INT_RAW_MASK
            } else {
                self.bits & !OutIntRawCh2::OUT_DONE_CH_INT_RAW_MASK
            },
        }
    }
}

impl Default for OutIntRawCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntRawCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntRawCh2Val {
        OutIntRawCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntRawCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntRawCh2Val) -> OutIntRawCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_ST_CH2 Register
///
/// Masked interrupt of channel 0
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntStCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntStCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntStCh2 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ST_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ST_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ST: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ST_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ST_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ST: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ST_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ST_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ST: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ST_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ST_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ST: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ST_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ST_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ST: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ST**
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ST_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ST_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// **OUTFIFO_OVF_CH_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_TOTAL_EOF_CH_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DSCR_ERR_CH_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// **OUT_EOF_CH_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// **OUT_DONE_CH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

/// OUT_INT_ST_CH2 value type for batch read/modify/write operations
///
/// Use `outintstch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintstch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntStCh2Val {
    bits: u32,
}

impl OutIntStCh2Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUTFIFO_UDF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUTFIFO_OVF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUT_TOTAL_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUT_DSCR_ERR_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUT_EOF_CH_INT_ST_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_st(&self) -> bool {
        (self.bits & OutIntStCh2::OUT_DONE_CH_INT_ST_MASK) != 0
    }
}

impl Default for OutIntStCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntStCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntStCh2Val {
        OutIntStCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_INT_ENA_CH2 Register
///
/// Interrupt enable bits of channel 0
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntEnaCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntEnaCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntEnaCh2 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_ENA_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_ENA_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_ENA: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_ENA_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_ENA_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_ENA: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_ENA_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_ENA_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_ENA: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_ENA_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_ENA_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_ENA: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_ENA_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_ENA_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_ENA: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_ENA**
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_ENA_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_ENA_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTFIFO_UDF_CH_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_UDF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_udf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_UDF_CH_INT_ENA_MASK);
    }

    /// **OUTFIFO_OVF_CH_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// Clear OUTFIFO_OVF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outfifo_ovf_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTFIFO_OVF_CH_INT_ENA_MASK);
    }

    /// **OUT_TOTAL_EOF_CH_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_TOTAL_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_total_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_TOTAL_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DSCR_ERR_CH_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DSCR_ERR_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_dscr_err_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DSCR_ERR_CH_INT_ENA_MASK);
    }

    /// **OUT_EOF_CH_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// Clear OUT_EOF_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_EOF_CH_INT_ENA_MASK);
    }

    /// **OUT_DONE_CH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done_ch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DONE_CH_INT_ENA_MASK);
    }

    /// Clear OUT_DONE_CH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_done_ch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DONE_CH_INT_ENA_MASK);
    }
}

/// OUT_INT_ENA_CH2 value type for batch read/modify/write operations
///
/// Use `outintenach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintenach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntEnaCh2Val {
    bits: u32,
}

impl OutIntEnaCh2Val {
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

    /// Read **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_udf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUTFIFO_UDF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn outfifo_ovf_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUTFIFO_OVF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_total_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUT_TOTAL_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_dscr_err_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUT_DSCR_ERR_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_eof_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUT_EOF_CH_INT_ENA_MASK) != 0
    }

    /// Read **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn out_done_ch_int_ena(&self) -> bool {
        (self.bits & OutIntEnaCh2::OUT_DONE_CH_INT_ENA_MASK) != 0
    }

    /// Set **OUTFIFO_UDF_CH_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUTFIFO_UDF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUTFIFO_UDF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUTFIFO_OVF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUTFIFO_OVF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUT_TOTAL_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUT_DSCR_ERR_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUT_DSCR_ERR_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUT_EOF_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUT_EOF_CH_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntEnaCh2::OUT_DONE_CH_INT_ENA_MASK
            } else {
                self.bits & !OutIntEnaCh2::OUT_DONE_CH_INT_ENA_MASK
            },
        }
    }
}

impl Default for OutIntEnaCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntEnaCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutIntEnaCh2Val {
        OutIntEnaCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntEnaCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutIntEnaCh2Val) -> OutIntEnaCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_INT_CLR_CH2 Register
///
/// Interrupt clear bits of channel 0
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **OUTFIFO_UDF_CH_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
///
/// - **OUTFIFO_OVF_CH_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
///
/// - **OUT_TOTAL_EOF_CH_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
///
/// - **OUT_DSCR_ERR_CH_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
///
/// - **OUT_EOF_CH_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the OUT_EOF_CH_INT interrupt.
///
/// - **OUT_DONE_CH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the OUT_DONE_CH_INT interrupt.
#[repr(transparent)]
pub struct OutIntClrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutIntClrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutIntClrCh2 {
    // ======== Field Constants ========

    /// **OUTFIFO_UDF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUTFIFO_UDF_CH_INT_CLR_OFFSET: u32 = 5;
    pub const OUTFIFO_UDF_CH_INT_CLR_MASK: u32 = 0x00000020;
    pub const OUTFIFO_UDF_CH_INT_CLR: u32 = 1 << 5;

    /// **OUTFIFO_OVF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTFIFO_OVF_CH_INT_CLR_OFFSET: u32 = 4;
    pub const OUTFIFO_OVF_CH_INT_CLR_MASK: u32 = 0x00000010;
    pub const OUTFIFO_OVF_CH_INT_CLR: u32 = 1 << 4;

    /// **OUT_TOTAL_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_TOTAL_EOF_CH_INT_CLR_OFFSET: u32 = 3;
    pub const OUT_TOTAL_EOF_CH_INT_CLR_MASK: u32 = 0x00000008;
    pub const OUT_TOTAL_EOF_CH_INT_CLR: u32 = 1 << 3;

    /// **OUT_DSCR_ERR_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_DSCR_ERR_CH_INT_CLR_OFFSET: u32 = 2;
    pub const OUT_DSCR_ERR_CH_INT_CLR_MASK: u32 = 0x00000004;
    pub const OUT_DSCR_ERR_CH_INT_CLR: u32 = 1 << 2;

    /// **OUT_EOF_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_EOF_CH_INT_CLR_OFFSET: u32 = 1;
    pub const OUT_EOF_CH_INT_CLR_MASK: u32 = 0x00000002;
    pub const OUT_EOF_CH_INT_CLR: u32 = 1 << 1;

    /// **OUT_DONE_CH_INT_CLR**
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_DONE_CH_INT_CLR_OFFSET: u32 = 0;
    pub const OUT_DONE_CH_INT_CLR_MASK: u32 = 0x00000001;
    pub const OUT_DONE_CH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_UDF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_udf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_UDF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUTFIFO_OVF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_outfifo_ovf_ch_int_clr(&self) {
        self.value.write_bits(Self::OUTFIFO_OVF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_TOTAL_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_total_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_TOTAL_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DSCR_ERR_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_dscr_err_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DSCR_ERR_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_EOF_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_eof_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_EOF_CH_INT_CLR_MASK);
    }

    /// Trigger **OUT_DONE_CH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn trigger_out_done_ch_int_clr(&self) {
        self.value.write_bits(Self::OUT_DONE_CH_INT_CLR_MASK);
    }
}

/// OUT_INT_CLR_CH2 value type for batch read/modify/write operations
///
/// Use `outintclrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outintclrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutIntClrCh2Val {
    bits: u32,
}

impl OutIntClrCh2Val {
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

    /// Set **OUTFIFO_UDF_CH_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_udf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUTFIFO_UDF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUTFIFO_UDF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUTFIFO_OVF_CH_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_outfifo_ovf_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUTFIFO_OVF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUTFIFO_OVF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_TOTAL_EOF_CH_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_total_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUT_TOTAL_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DSCR_ERR_CH_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_dscr_err_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUT_DSCR_ERR_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUT_DSCR_ERR_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_EOF_CH_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_eof_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUT_EOF_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUT_EOF_CH_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_DONE_CH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub const fn with_out_done_ch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutIntClrCh2::OUT_DONE_CH_INT_CLR_MASK
            } else {
                self.bits & !OutIntClrCh2::OUT_DONE_CH_INT_CLR_MASK
            },
        }
    }
}

impl Default for OutIntClrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutIntClrCh2 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutIntClrCh2Val) {
        self.value.write_bits(val.bits())
    }
}

/// AHB_TEST Register
///
/// reserved
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **AHB_TESTADDR** `[5:4]` (RW)
///
///   reserved
///
/// - **AHB_TESTMODE** `[2:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct AhbTest {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbTest {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbTest {
    // ======== Field Constants ========

    /// **AHB_TESTADDR**
    ///
    /// reserved
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const AHB_TESTADDR_OFFSET: u32 = 4;
    pub const AHB_TESTADDR_MASK: u32 = 0x00000030;

    /// **AHB_TESTMODE**
    ///
    /// reserved
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const AHB_TESTMODE_OFFSET: u32 = 0;
    pub const AHB_TESTMODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **AHB_TESTADDR** `[5:4]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn ahb_testaddr(&self) -> u32 {
        (self.value.read_bits() & Self::AHB_TESTADDR_MASK) >> Self::AHB_TESTADDR_OFFSET
    }

    /// Write **AHB_TESTADDR** `[5:4]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_ahb_testaddr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AHB_TESTADDR_MASK)
                | ((value << Self::AHB_TESTADDR_OFFSET) & Self::AHB_TESTADDR_MASK)
        });
    }

    /// **AHB_TESTMODE** `[2:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn ahb_testmode(&self) -> u32 {
        (self.value.read_bits() & Self::AHB_TESTMODE_MASK) >> Self::AHB_TESTMODE_OFFSET
    }

    /// Write **AHB_TESTMODE** `[2:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_ahb_testmode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AHB_TESTMODE_MASK)
                | ((value << Self::AHB_TESTMODE_OFFSET) & Self::AHB_TESTMODE_MASK)
        });
    }
}

/// AHB_TEST value type for batch read/modify/write operations
///
/// Use `ahbtest.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbtest.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbTestVal {
    bits: u32,
}

impl AhbTestVal {
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

    /// Read **AHB_TESTADDR** `[5:4]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn ahb_testaddr(&self) -> u32 {
        (self.bits & AhbTest::AHB_TESTADDR_MASK) >> AhbTest::AHB_TESTADDR_OFFSET
    }

    /// Read **AHB_TESTMODE** `[2:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn ahb_testmode(&self) -> u32 {
        (self.bits & AhbTest::AHB_TESTMODE_MASK) >> AhbTest::AHB_TESTMODE_OFFSET
    }

    /// Set **AHB_TESTADDR** `[5:4]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_ahb_testaddr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbTest::AHB_TESTADDR_MASK)
                | ((value << AhbTest::AHB_TESTADDR_OFFSET) & AhbTest::AHB_TESTADDR_MASK),
        }
    }

    /// Set **AHB_TESTMODE** `[2:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_ahb_testmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbTest::AHB_TESTMODE_MASK)
                | ((value << AhbTest::AHB_TESTMODE_OFFSET) & AhbTest::AHB_TESTMODE_MASK),
        }
    }
}

impl Default for AhbTestVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbTest {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbTestVal {
        AhbTestVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbTestVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbTestVal) -> AhbTestVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MISC_CONF Register
///
/// MISC register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **CLK_EN** `[3]` (RW)
///
///   1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
///
/// - **ARB_PRI_DIS** `[2]` (RW)
///
///   Set this bit to disable priority arbitration function.
///
/// - **AHBM_RST_INTER** `[0]` (RW)
///
///   Set this bit then clear this bit to reset the internal ahb FSM.
#[repr(transparent)]
pub struct MiscConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MiscConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MiscConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_EN_OFFSET: u32 = 3;
    pub const CLK_EN_MASK: u32 = 0x00000008;
    pub const CLK_EN: u32 = 1 << 3;

    /// **ARB_PRI_DIS**
    ///
    /// Set this bit to disable priority arbitration function.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ARB_PRI_DIS_OFFSET: u32 = 2;
    pub const ARB_PRI_DIS_MASK: u32 = 0x00000004;
    pub const ARB_PRI_DIS: u32 = 1 << 2;

    /// **AHBM_RST_INTER**
    ///
    /// Set this bit then clear this bit to reset the internal ahb FSM.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const AHBM_RST_INTER_OFFSET: u32 = 0;
    pub const AHBM_RST_INTER_MASK: u32 = 0x00000001;
    pub const AHBM_RST_INTER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[3]` (RW)
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
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

    /// **ARB_PRI_DIS** `[2]` (RW)
    ///
    /// Set this bit to disable priority arbitration function.
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> bool {
        (self.value.read_bits() & Self::ARB_PRI_DIS_MASK) != 0
    }

    /// Set ARB_PRI_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_pri_dis(&self) {
        self.value.modify_bits(|r| r | Self::ARB_PRI_DIS_MASK);
    }

    /// Clear ARB_PRI_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_pri_dis(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_PRI_DIS_MASK);
    }

    /// **AHBM_RST_INTER** `[0]` (RW)
    ///
    /// Set this bit then clear this bit to reset the internal ahb FSM.
    #[inline(always)]
    pub fn ahbm_rst_inter(&self) -> bool {
        (self.value.read_bits() & Self::AHBM_RST_INTER_MASK) != 0
    }

    /// Set AHBM_RST_INTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ahbm_rst_inter(&self) {
        self.value.modify_bits(|r| r | Self::AHBM_RST_INTER_MASK);
    }

    /// Clear AHBM_RST_INTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ahbm_rst_inter(&self) {
        self.value.modify_bits(|r| r & !Self::AHBM_RST_INTER_MASK);
    }
}

/// MISC_CONF value type for batch read/modify/write operations
///
/// Use `miscconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `miscconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MiscConfVal {
    bits: u32,
}

impl MiscConfVal {
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

    /// Read **CLK_EN** `[3:3]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & MiscConf::CLK_EN_MASK) != 0
    }

    /// Read **ARB_PRI_DIS** `[2:2]`
    ///
    /// Set this bit to disable priority arbitration function.
    #[inline(always)]
    pub const fn arb_pri_dis(&self) -> bool {
        (self.bits & MiscConf::ARB_PRI_DIS_MASK) != 0
    }

    /// Read **AHBM_RST_INTER** `[0:0]`
    ///
    /// Set this bit then clear this bit to reset the internal ahb FSM.
    #[inline(always)]
    pub const fn ahbm_rst_inter(&self) -> bool {
        (self.bits & MiscConf::AHBM_RST_INTER_MASK) != 0
    }

    /// Set **CLK_EN** `[3:3]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MiscConf::CLK_EN_MASK
            } else {
                self.bits & !MiscConf::CLK_EN_MASK
            },
        }
    }

    /// Set **ARB_PRI_DIS** `[2:2]`
    ///
    /// Set this bit to disable priority arbitration function.
    #[inline(always)]
    pub const fn with_arb_pri_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MiscConf::ARB_PRI_DIS_MASK
            } else {
                self.bits & !MiscConf::ARB_PRI_DIS_MASK
            },
        }
    }

    /// Set **AHBM_RST_INTER** `[0:0]`
    ///
    /// Set this bit then clear this bit to reset the internal ahb FSM.
    #[inline(always)]
    pub const fn with_ahbm_rst_inter(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MiscConf::AHBM_RST_INTER_MASK
            } else {
                self.bits & !MiscConf::AHBM_RST_INTER_MASK
            },
        }
    }
}

impl Default for MiscConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MiscConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MiscConfVal {
        MiscConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MiscConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MiscConfVal) -> MiscConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Version control register
///
/// Address offset: 0x0068
/// Reset value: 0x02303140
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   register version.
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
    /// register version.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// register version.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// register version.
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
        Self { bits: 0x02303140 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// register version.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// register version.
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

/// IN_CONF0_CH0 Register
///
/// Configure 0 register of Rx channel 0
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **IN_ETM_EN_CH** `[5]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
///
/// - **MEM_TRANS_EN_CH** `[4]` (RW)
///
///   Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
///
/// - **IN_DATA_BURST_EN_CH** `[3]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
///
/// - **INDSCR_BURST_EN_CH** `[2]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
///
/// - **IN_LOOP_TEST_CH** `[1]` (RW)
///
///   reserved
///
/// - **IN_RST_CH** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
#[repr(transparent)]
pub struct InConf0Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf0Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf0Ch0 {
    // ======== Field Constants ========

    /// **IN_ETM_EN_CH**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IN_ETM_EN_CH_OFFSET: u32 = 5;
    pub const IN_ETM_EN_CH_MASK: u32 = 0x00000020;
    pub const IN_ETM_EN_CH: u32 = 1 << 5;

    /// **MEM_TRANS_EN_CH**
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TRANS_EN_CH_OFFSET: u32 = 4;
    pub const MEM_TRANS_EN_CH_MASK: u32 = 0x00000010;
    pub const MEM_TRANS_EN_CH: u32 = 1 << 4;

    /// **IN_DATA_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DATA_BURST_EN_CH_OFFSET: u32 = 3;
    pub const IN_DATA_BURST_EN_CH_MASK: u32 = 0x00000008;
    pub const IN_DATA_BURST_EN_CH: u32 = 1 << 3;

    /// **INDSCR_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INDSCR_BURST_EN_CH_OFFSET: u32 = 2;
    pub const INDSCR_BURST_EN_CH_MASK: u32 = 0x00000004;
    pub const INDSCR_BURST_EN_CH: u32 = 1 << 2;

    /// **IN_LOOP_TEST_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_LOOP_TEST_CH_OFFSET: u32 = 1;
    pub const IN_LOOP_TEST_CH_MASK: u32 = 0x00000002;
    pub const IN_LOOP_TEST_CH: u32 = 1 << 1;

    /// **IN_RST_CH**
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_RST_CH_OFFSET: u32 = 0;
    pub const IN_RST_CH_MASK: u32 = 0x00000001;
    pub const IN_RST_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_ETM_EN_CH** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_ETM_EN_CH_MASK) != 0
    }

    /// Set IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_ETM_EN_CH_MASK);
    }

    /// Clear IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_ETM_EN_CH_MASK);
    }

    /// **MEM_TRANS_EN_CH** `[4]` (RW)
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Set MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::MEM_TRANS_EN_CH_MASK);
    }

    /// Clear MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_TRANS_EN_CH_MASK);
    }

    /// **IN_DATA_BURST_EN_CH** `[3]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub fn in_data_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Set IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// Clear IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// **INDSCR_BURST_EN_CH** `[2]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Set INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// Clear INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// **IN_LOOP_TEST_CH** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Set IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_LOOP_TEST_CH_MASK);
    }

    /// Clear IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_LOOP_TEST_CH_MASK);
    }

    /// **IN_RST_CH** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub fn in_rst_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_RST_CH_MASK) != 0
    }

    /// Set IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_rst_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_RST_CH_MASK);
    }

    /// Clear IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_rst_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_RST_CH_MASK);
    }
}

/// IN_CONF0_CH0 value type for batch read/modify/write operations
///
/// Use `inconf0ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf0ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf0Ch0Val {
    bits: u32,
}

impl InConf0Ch0Val {
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

    /// Read **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn in_etm_en_ch(&self) -> bool {
        (self.bits & InConf0Ch0::IN_ETM_EN_CH_MASK) != 0
    }

    /// Read **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn mem_trans_en_ch(&self) -> bool {
        (self.bits & InConf0Ch0::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Read **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn in_data_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch0::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Read **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn indscr_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch0::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Read **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_loop_test_ch(&self) -> bool {
        (self.bits & InConf0Ch0::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Read **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn in_rst_ch(&self) -> bool {
        (self.bits & InConf0Ch0::IN_RST_CH_MASK) != 0
    }

    /// Set **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn with_in_etm_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::IN_ETM_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch0::IN_ETM_EN_CH_MASK
            },
        }
    }

    /// Set **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn with_mem_trans_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::MEM_TRANS_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch0::MEM_TRANS_EN_CH_MASK
            },
        }
    }

    /// Set **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_in_data_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::IN_DATA_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch0::IN_DATA_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_indscr_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::INDSCR_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch0::INDSCR_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_in_loop_test_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::IN_LOOP_TEST_CH_MASK
            } else {
                self.bits & !InConf0Ch0::IN_LOOP_TEST_CH_MASK
            },
        }
    }

    /// Set **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn with_in_rst_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch0::IN_RST_CH_MASK
            } else {
                self.bits & !InConf0Ch0::IN_RST_CH_MASK
            },
        }
    }
}

impl Default for InConf0Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf0Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf0Ch0Val {
        InConf0Ch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf0Ch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf0Ch0Val) -> InConf0Ch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CONF1_CH0 Register
///
/// Configure 1 register of Rx channel 0
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **IN_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct InConf1Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf1Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf1Ch0 {
    // ======== Field Constants ========

    /// **IN_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IN_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const IN_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const IN_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **IN_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_check_owner_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CHECK_OWNER_CH_MASK);
    }

    /// Clear IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_CHECK_OWNER_CH_MASK);
    }
}

/// IN_CONF1_CH0 value type for batch read/modify/write operations
///
/// Use `inconf1ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf1ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf1Ch0Val {
    bits: u32,
}

impl InConf1Ch0Val {
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

    /// Read **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn in_check_owner_ch(&self) -> bool {
        (self.bits & InConf1Ch0::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_in_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf1Ch0::IN_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !InConf1Ch0::IN_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for InConf1Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf1Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf1Ch0Val {
        InConf1Ch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf1Ch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf1Ch0Val) -> InConf1Ch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INFIFO_STATUS_CH0 Register
///
/// Receive FIFO status of Rx channel 0
///
/// Address offset: 0x0078
/// Reset value: 0x07800003
///
/// ## Fields
///
/// - **IN_BUF_HUNGRY_CH** `[27]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **INFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
///
/// - **INFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Rx FIFO empty signal for Rx channel 0.
///
/// - **INFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Rx FIFO full signal for Rx channel 0.
#[repr(transparent)]
pub struct InfifoStatusCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfifoStatusCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfifoStatusCh0 {
    // ======== Field Constants ========

    /// **IN_BUF_HUNGRY_CH**
    ///
    /// reserved
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const IN_BUF_HUNGRY_CH_OFFSET: u32 = 27;
    pub const IN_BUF_HUNGRY_CH_MASK: u32 = 0x08000000;
    pub const IN_BUF_HUNGRY_CH: u32 = 1 << 27;

    /// **IN_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const IN_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const IN_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const IN_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **IN_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const IN_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const IN_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const IN_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **IN_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const IN_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const IN_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const IN_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **IN_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const IN_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const IN_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const IN_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **INFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const INFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const INFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **INFIFO_EMPTY_CH**
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const INFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const INFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **INFIFO_FULL_CH**
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const INFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const INFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_BUF_HUNGRY_CH** `[27]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **INFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_CNT_CH_MASK) >> Self::INFIFO_CNT_CH_OFFSET
    }

    /// **INFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// **INFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_FULL_CH_MASK) != 0
    }
}

/// INFIFO_STATUS_CH0 value type for batch read/modify/write operations
///
/// Use `infifostatusch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `infifostatusch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InfifoStatusCh0Val {
    bits: u32,
}

impl InfifoStatusCh0Val {
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
        Self { bits: 0x07800003 }
    }

    /// Read **IN_BUF_HUNGRY_CH** `[27:27]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_buf_hungry_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_4b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_3b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_2b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_1b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **INFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_cnt_ch(&self) -> u32 {
        (self.bits & InfifoStatusCh0::INFIFO_CNT_CH_MASK) >> InfifoStatusCh0::INFIFO_CNT_CH_OFFSET
    }

    /// Read **INFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_empty_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **INFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_full_ch(&self) -> bool {
        (self.bits & InfifoStatusCh0::INFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for InfifoStatusCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InfifoStatusCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InfifoStatusCh0Val {
        InfifoStatusCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_POP_CH0 Register
///
/// Pop control register of Rx channel 0
///
/// Address offset: 0x007c
/// Reset value: 0x00000800
///
/// ## Fields
///
/// - **INFIFO_POP_CH** `[12]` (WO)
///
///   Set this bit to pop data from AHB_DMA FIFO.
///
/// - **INFIFO_RDATA_CH** `[11:0]` (RO)
///
///   This register stores the data popping from AHB_DMA FIFO.
#[repr(transparent)]
pub struct InPopCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPopCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPopCh0 {
    // ======== Field Constants ========

    /// **INFIFO_POP_CH**
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INFIFO_POP_CH_OFFSET: u32 = 12;
    pub const INFIFO_POP_CH_MASK: u32 = 0x00001000;
    pub const INFIFO_POP_CH: u32 = 1 << 12;

    /// **INFIFO_RDATA_CH**
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const INFIFO_RDATA_CH_OFFSET: u32 = 0;
    pub const INFIFO_RDATA_CH_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_POP_CH** (WO)
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_infifo_pop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INFIFO_POP_CH_MASK);
    }

    /// **INFIFO_RDATA_CH** `[11:0]` (RO)
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_RDATA_CH_MASK) >> Self::INFIFO_RDATA_CH_OFFSET
    }
}

/// IN_POP_CH0 value type for batch read/modify/write operations
///
/// Use `inpopch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inpopch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPopCh0Val {
    bits: u32,
}

impl InPopCh0Val {
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
        Self { bits: 0x00000800 }
    }

    /// Read **INFIFO_RDATA_CH** `[11:0]`
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn infifo_rdata_ch(&self) -> u32 {
        (self.bits & InPopCh0::INFIFO_RDATA_CH_MASK) >> InPopCh0::INFIFO_RDATA_CH_OFFSET
    }

    /// Set **INFIFO_POP_CH** `[12:12]`
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_infifo_pop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InPopCh0::INFIFO_POP_CH_MASK
            } else {
                self.bits & !InPopCh0::INFIFO_POP_CH_MASK
            },
        }
    }
}

impl Default for InPopCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPopCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPopCh0Val {
        InPopCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPopCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPopCh0Val) -> InPopCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_CH0 Register
///
/// Link descriptor configure and control register of Rx channel 0
///
/// Address offset: 0x0080
/// Reset value: 0x00000011
///
/// ## Fields
///
/// - **INLINK_PARK_CH** `[4]` (RO)
///
///   1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
///
/// - **INLINK_RESTART_CH** `[3]` (WO)
///
///   Set this bit to mount a new inlink descriptor.
///
/// - **INLINK_START_CH** `[2]` (WO)
///
///   Set this bit to start dealing with the inlink descriptors.
///
/// - **INLINK_STOP_CH** `[1]` (WO)
///
///   Set this bit to stop dealing with the inlink descriptors.
///
/// - **INLINK_AUTO_RET_CH** `[0]` (RW)
///
///   Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
#[repr(transparent)]
pub struct InLinkCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkCh0 {
    // ======== Field Constants ========

    /// **INLINK_PARK_CH**
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const INLINK_PARK_CH_OFFSET: u32 = 4;
    pub const INLINK_PARK_CH_MASK: u32 = 0x00000010;
    pub const INLINK_PARK_CH: u32 = 1 << 4;

    /// **INLINK_RESTART_CH**
    ///
    /// Set this bit to mount a new inlink descriptor.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INLINK_RESTART_CH_OFFSET: u32 = 3;
    pub const INLINK_RESTART_CH_MASK: u32 = 0x00000008;
    pub const INLINK_RESTART_CH: u32 = 1 << 3;

    /// **INLINK_START_CH**
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INLINK_START_CH_OFFSET: u32 = 2;
    pub const INLINK_START_CH_MASK: u32 = 0x00000004;
    pub const INLINK_START_CH: u32 = 1 << 2;

    /// **INLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INLINK_STOP_CH_OFFSET: u32 = 1;
    pub const INLINK_STOP_CH_MASK: u32 = 0x00000002;
    pub const INLINK_STOP_CH: u32 = 1 << 1;

    /// **INLINK_AUTO_RET_CH**
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INLINK_AUTO_RET_CH_OFFSET: u32 = 0;
    pub const INLINK_AUTO_RET_CH_MASK: u32 = 0x00000001;
    pub const INLINK_AUTO_RET_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INLINK_PARK_CH** `[4]` (RO)
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub fn inlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **INLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub fn trigger_inlink_restart_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_RESTART_CH_MASK);
    }

    /// Trigger **INLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_START_CH_MASK);
    }

    /// Trigger **INLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_STOP_CH_MASK);
    }

    /// **INLINK_AUTO_RET_CH** `[0]` (RW)
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INLINK_AUTO_RET_CH_MASK);
    }

    /// Clear INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INLINK_AUTO_RET_CH_MASK);
    }
}

/// IN_LINK_CH0 value type for batch read/modify/write operations
///
/// Use `inlinkch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkCh0Val {
    bits: u32,
}

impl InLinkCh0Val {
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
        Self { bits: 0x00000011 }
    }

    /// Read **INLINK_PARK_CH** `[4:4]`
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn inlink_park_ch(&self) -> bool {
        (self.bits & InLinkCh0::INLINK_PARK_CH_MASK) != 0
    }

    /// Read **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn inlink_auto_ret_ch(&self) -> bool {
        (self.bits & InLinkCh0::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set **INLINK_RESTART_CH** `[3:3]`
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub const fn with_inlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh0::INLINK_RESTART_CH_MASK
            } else {
                self.bits & !InLinkCh0::INLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **INLINK_START_CH** `[2:2]`
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh0::INLINK_START_CH_MASK
            } else {
                self.bits & !InLinkCh0::INLINK_START_CH_MASK
            },
        }
    }

    /// Set **INLINK_STOP_CH** `[1:1]`
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh0::INLINK_STOP_CH_MASK
            } else {
                self.bits & !InLinkCh0::INLINK_STOP_CH_MASK
            },
        }
    }

    /// Set **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn with_inlink_auto_ret_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh0::INLINK_AUTO_RET_CH_MASK
            } else {
                self.bits & !InLinkCh0::INLINK_AUTO_RET_CH_MASK
            },
        }
    }
}

impl Default for InLinkCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InLinkCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkCh0Val {
        InLinkCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkCh0Val) -> InLinkCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_STATE_CH0 Register
///
/// Receive status of Rx channel 0
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **IN_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **IN_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current inlink descriptor's address.
#[repr(transparent)]
pub struct InStateCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InStateCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InStateCh0 {
    // ======== Field Constants ========

    /// **IN_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const IN_STATE_CH_OFFSET: u32 = 20;
    pub const IN_STATE_CH_MASK: u32 = 0x00700000;

    /// **IN_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const IN_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const IN_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **INLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current inlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const INLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **IN_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_STATE_CH_MASK) >> Self::IN_STATE_CH_OFFSET
    }

    /// **IN_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_DSCR_STATE_CH_MASK) >> Self::IN_DSCR_STATE_CH_OFFSET
    }

    /// **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_ADDR_CH_MASK)
            >> Self::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// IN_STATE_CH0 value type for batch read/modify/write operations
///
/// Use `instatech0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instatech0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InStateCh0Val {
    bits: u32,
}

impl InStateCh0Val {
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

    /// Read **IN_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_state_ch(&self) -> u32 {
        (self.bits & InStateCh0::IN_STATE_CH_MASK) >> InStateCh0::IN_STATE_CH_OFFSET
    }

    /// Read **IN_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_dscr_state_ch(&self) -> u32 {
        (self.bits & InStateCh0::IN_DSCR_STATE_CH_MASK) >> InStateCh0::IN_DSCR_STATE_CH_OFFSET
    }

    /// Read **INLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & InStateCh0::INLINK_DSCR_ADDR_CH_MASK) >> InStateCh0::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for InStateCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InStateCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InStateCh0Val {
        InStateCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_SUC_EOF_DES_ADDR_CH0 Register
///
/// Inlink descriptor address when EOF occurs of Rx channel 0
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct InSucEofDesAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InSucEofDesAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InSucEofDesAddrCh0 {
    // ======== Field Constants ========

    /// **IN_SUC_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_SUC_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_SUC_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_SUC_EOF_DES_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `insuceofdesaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `insuceofdesaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InSucEofDesAddrCh0Val {
    bits: u32,
}

impl InSucEofDesAddrCh0Val {
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

    /// Read **IN_SUC_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InSucEofDesAddrCh0::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> InSucEofDesAddrCh0::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InSucEofDesAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InSucEofDesAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InSucEofDesAddrCh0Val {
        InSucEofDesAddrCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_ERR_EOF_DES_ADDR_CH0 Register
///
/// Inlink descriptor address when errors occur of Rx channel 0
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
#[repr(transparent)]
pub struct InErrEofDesAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InErrEofDesAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InErrEofDesAddrCh0 {
    // ======== Field Constants ========

    /// **IN_ERR_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_ERR_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_ERR_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_ERR_EOF_DES_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `inerreofdesaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inerreofdesaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InErrEofDesAddrCh0Val {
    bits: u32,
}

impl InErrEofDesAddrCh0Val {
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

    /// Read **IN_ERR_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InErrEofDesAddrCh0::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> InErrEofDesAddrCh0::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InErrEofDesAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InErrEofDesAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InErrEofDesAddrCh0Val {
        InErrEofDesAddrCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_CH0 Register
///
/// Current inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **INLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current inlink descriptor x.
#[repr(transparent)]
pub struct InDscrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrCh0 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_CH**
    ///
    /// The address of the current inlink descriptor x.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub fn inlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_CH_MASK) >> Self::INLINK_DSCR_CH_OFFSET
    }
}

/// IN_DSCR_CH0 value type for batch read/modify/write operations
///
/// Use `indscrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrCh0Val {
    bits: u32,
}

impl InDscrCh0Val {
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

    /// Read **INLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub const fn inlink_dscr_ch(&self) -> u32 {
        (self.bits & InDscrCh0::INLINK_DSCR_CH_MASK) >> InDscrCh0::INLINK_DSCR_CH_OFFSET
    }
}

impl Default for InDscrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrCh0Val {
        InDscrCh0Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF0_CH0 Register
///
/// The last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last inlink descriptor x-1.
#[repr(transparent)]
pub struct InDscrBf0Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf0Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf0Ch0 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF0_CH**
    ///
    /// The address of the last inlink descriptor x-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF0_CH_MASK) >> Self::INLINK_DSCR_BF0_CH_OFFSET
    }
}

/// IN_DSCR_BF0_CH0 value type for batch read/modify/write operations
///
/// Use `indscrbf0ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf0ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf0Ch0Val {
    bits: u32,
}

impl InDscrBf0Ch0Val {
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

    /// Read **INLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub const fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & InDscrBf0Ch0::INLINK_DSCR_BF0_CH_MASK)
            >> InDscrBf0Ch0::INLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for InDscrBf0Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf0Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf0Ch0Val {
        InDscrBf0Ch0Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF1_CH0 Register
///
/// The second-to-last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct InDscrBf1Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf1Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf1Ch0 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF1_CH_MASK) >> Self::INLINK_DSCR_BF1_CH_OFFSET
    }
}

/// IN_DSCR_BF1_CH0 value type for batch read/modify/write operations
///
/// Use `indscrbf1ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf1ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf1Ch0Val {
    bits: u32,
}

impl InDscrBf1Ch0Val {
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

    /// Read **INLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & InDscrBf1Ch0::INLINK_DSCR_BF1_CH_MASK)
            >> InDscrBf1Ch0::INLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for InDscrBf1Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf1Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf1Ch0Val {
        InDscrBf1Ch0Val::from_bits(self.value.read_bits())
    }
}

/// IN_PRI_CH0 Register
///
/// Priority register of Rx channel 0
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **RX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Rx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct InPriCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPriCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPriCh0 {
    // ======== Field Constants ========

    /// **RX_PRI_CH**
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_PRI_CH_OFFSET: u32 = 0;
    pub const RX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn rx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PRI_CH_MASK) >> Self::RX_PRI_CH_OFFSET
    }

    /// Write **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_rx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PRI_CH_MASK) | ((value << Self::RX_PRI_CH_OFFSET) & Self::RX_PRI_CH_MASK)
        });
    }
}

/// IN_PRI_CH0 value type for batch read/modify/write operations
///
/// Use `inprich0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inprich0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPriCh0Val {
    bits: u32,
}

impl InPriCh0Val {
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

    /// Read **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn rx_pri_ch(&self) -> u32 {
        (self.bits & InPriCh0::RX_PRI_CH_MASK) >> InPriCh0::RX_PRI_CH_OFFSET
    }

    /// Set **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_rx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPriCh0::RX_PRI_CH_MASK)
                | ((value << InPriCh0::RX_PRI_CH_OFFSET) & InPriCh0::RX_PRI_CH_MASK),
        }
    }
}

impl Default for InPriCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InPriCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPriCh0Val {
        InPriCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPriCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPriCh0Val) -> InPriCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_PERI_SEL_CH0 Register
///
/// Peripheral selection of Rx channel 0
///
/// Address offset: 0x00a0
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_IN_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct InPeriSelCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPeriSelCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPeriSelCh0 {
    // ======== Field Constants ========

    /// **PERI_IN_SEL_CH**
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_IN_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_IN_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_in_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_IN_SEL_CH_MASK) >> Self::PERI_IN_SEL_CH_OFFSET
    }

    /// Write **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_in_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_IN_SEL_CH_MASK)
                | ((value << Self::PERI_IN_SEL_CH_OFFSET) & Self::PERI_IN_SEL_CH_MASK)
        });
    }
}

/// IN_PERI_SEL_CH0 value type for batch read/modify/write operations
///
/// Use `inperiselch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inperiselch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPeriSelCh0Val {
    bits: u32,
}

impl InPeriSelCh0Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_in_sel_ch(&self) -> u32 {
        (self.bits & InPeriSelCh0::PERI_IN_SEL_CH_MASK) >> InPeriSelCh0::PERI_IN_SEL_CH_OFFSET
    }

    /// Set **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_in_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPeriSelCh0::PERI_IN_SEL_CH_MASK)
                | ((value << InPeriSelCh0::PERI_IN_SEL_CH_OFFSET)
                    & InPeriSelCh0::PERI_IN_SEL_CH_MASK),
        }
    }
}

impl Default for InPeriSelCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPeriSelCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPeriSelCh0Val {
        InPeriSelCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPeriSelCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPeriSelCh0Val) -> InPeriSelCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF0_CH0 Register
///
/// Configure 0 register of Tx channel 0
///
/// Address offset: 0x00d0
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUT_ETM_EN_CH0** `[6]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task.
///
/// - **OUT_DATA_BURST_EN_CH0** `[5]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM.
///
/// - **OUTDSCR_BURST_EN_CH0** `[4]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
///
/// - **OUT_EOF_MODE_CH0** `[3]` (RW)
///
///   EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA
///
/// - **OUT_AUTO_WRBACK_CH0** `[2]` (RW)
///
///   Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
///
/// - **OUT_LOOP_TEST_CH0** `[1]` (RW)
///
///   reserved
///
/// - **OUT_RST_CH0** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer.
#[repr(transparent)]
pub struct OutConf0Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf0Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf0Ch0 {
    // ======== Field Constants ========

    /// **OUT_ETM_EN_CH0**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUT_ETM_EN_CH0_OFFSET: u32 = 6;
    pub const OUT_ETM_EN_CH0_MASK: u32 = 0x00000040;
    pub const OUT_ETM_EN_CH0: u32 = 1 << 6;

    /// **OUT_DATA_BURST_EN_CH0**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUT_DATA_BURST_EN_CH0_OFFSET: u32 = 5;
    pub const OUT_DATA_BURST_EN_CH0_MASK: u32 = 0x00000020;
    pub const OUT_DATA_BURST_EN_CH0: u32 = 1 << 5;

    /// **OUTDSCR_BURST_EN_CH0**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTDSCR_BURST_EN_CH0_OFFSET: u32 = 4;
    pub const OUTDSCR_BURST_EN_CH0_MASK: u32 = 0x00000010;
    pub const OUTDSCR_BURST_EN_CH0: u32 = 1 << 4;

    /// **OUT_EOF_MODE_CH0**
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_EOF_MODE_CH0_OFFSET: u32 = 3;
    pub const OUT_EOF_MODE_CH0_MASK: u32 = 0x00000008;
    pub const OUT_EOF_MODE_CH0: u32 = 1 << 3;

    /// **OUT_AUTO_WRBACK_CH0**
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_AUTO_WRBACK_CH0_OFFSET: u32 = 2;
    pub const OUT_AUTO_WRBACK_CH0_MASK: u32 = 0x00000004;
    pub const OUT_AUTO_WRBACK_CH0: u32 = 1 << 2;

    /// **OUT_LOOP_TEST_CH0**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_LOOP_TEST_CH0_OFFSET: u32 = 1;
    pub const OUT_LOOP_TEST_CH0_MASK: u32 = 0x00000002;
    pub const OUT_LOOP_TEST_CH0: u32 = 1 << 1;

    /// **OUT_RST_CH0**
    ///
    /// This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_RST_CH0_OFFSET: u32 = 0;
    pub const OUT_RST_CH0_MASK: u32 = 0x00000001;
    pub const OUT_RST_CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_ETM_EN_CH0** `[6]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task.
    #[inline(always)]
    pub fn out_etm_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_ETM_EN_CH0_MASK) != 0
    }

    /// Set OUT_ETM_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_etm_en_ch0(&self) {
        self.value.modify_bits(|r| r | Self::OUT_ETM_EN_CH0_MASK);
    }

    /// Clear OUT_ETM_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_etm_en_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_ETM_EN_CH0_MASK);
    }

    /// **OUT_DATA_BURST_EN_CH0** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DATA_BURST_EN_CH0_MASK) != 0
    }

    /// Set OUT_DATA_BURST_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_data_burst_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DATA_BURST_EN_CH0_MASK);
    }

    /// Clear OUT_DATA_BURST_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_data_burst_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DATA_BURST_EN_CH0_MASK);
    }

    /// **OUTDSCR_BURST_EN_CH0** `[4]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUTDSCR_BURST_EN_CH0_MASK) != 0
    }

    /// Set OUTDSCR_BURST_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outdscr_burst_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTDSCR_BURST_EN_CH0_MASK);
    }

    /// Clear OUTDSCR_BURST_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outdscr_burst_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTDSCR_BURST_EN_CH0_MASK);
    }

    /// **OUT_EOF_MODE_CH0** `[3]` (RW)
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub fn out_eof_mode_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_MODE_CH0_MASK) != 0
    }

    /// Set OUT_EOF_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_mode_ch0(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_MODE_CH0_MASK);
    }

    /// Clear OUT_EOF_MODE_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_mode_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_MODE_CH0_MASK);
    }

    /// **OUT_AUTO_WRBACK_CH0** `[2]` (RW)
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_AUTO_WRBACK_CH0_MASK) != 0
    }

    /// Set OUT_AUTO_WRBACK_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_auto_wrback_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_AUTO_WRBACK_CH0_MASK);
    }

    /// Clear OUT_AUTO_WRBACK_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_auto_wrback_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_AUTO_WRBACK_CH0_MASK);
    }

    /// **OUT_LOOP_TEST_CH0** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_loop_test_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_LOOP_TEST_CH0_MASK) != 0
    }

    /// Set OUT_LOOP_TEST_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_loop_test_ch0(&self) {
        self.value.modify_bits(|r| r | Self::OUT_LOOP_TEST_CH0_MASK);
    }

    /// Clear OUT_LOOP_TEST_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_loop_test_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_LOOP_TEST_CH0_MASK);
    }

    /// **OUT_RST_CH0** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub fn out_rst_ch0(&self) -> bool {
        (self.value.read_bits() & Self::OUT_RST_CH0_MASK) != 0
    }

    /// Set OUT_RST_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_rst_ch0(&self) {
        self.value.modify_bits(|r| r | Self::OUT_RST_CH0_MASK);
    }

    /// Clear OUT_RST_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_rst_ch0(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_RST_CH0_MASK);
    }
}

/// OUT_CONF0_CH0 value type for batch read/modify/write operations
///
/// Use `outconf0ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf0ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf0Ch0Val {
    bits: u32,
}

impl OutConf0Ch0Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUT_ETM_EN_CH0** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn out_etm_en_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_ETM_EN_CH0_MASK) != 0
    }

    /// Read **OUT_DATA_BURST_EN_CH0** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn out_data_burst_en_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_DATA_BURST_EN_CH0_MASK) != 0
    }

    /// Read **OUTDSCR_BURST_EN_CH0** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn outdscr_burst_en_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUTDSCR_BURST_EN_CH0_MASK) != 0
    }

    /// Read **OUT_EOF_MODE_CH0** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn out_eof_mode_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_EOF_MODE_CH0_MASK) != 0
    }

    /// Read **OUT_AUTO_WRBACK_CH0** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn out_auto_wrback_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_AUTO_WRBACK_CH0_MASK) != 0
    }

    /// Read **OUT_LOOP_TEST_CH0** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_loop_test_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_LOOP_TEST_CH0_MASK) != 0
    }

    /// Read **OUT_RST_CH0** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn out_rst_ch0(&self) -> bool {
        (self.bits & OutConf0Ch0::OUT_RST_CH0_MASK) != 0
    }

    /// Set **OUT_ETM_EN_CH0** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn with_out_etm_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_ETM_EN_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_ETM_EN_CH0_MASK
            },
        }
    }

    /// Set **OUT_DATA_BURST_EN_CH0** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_out_data_burst_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_DATA_BURST_EN_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_DATA_BURST_EN_CH0_MASK
            },
        }
    }

    /// Set **OUTDSCR_BURST_EN_CH0** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_outdscr_burst_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUTDSCR_BURST_EN_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUTDSCR_BURST_EN_CH0_MASK
            },
        }
    }

    /// Set **OUT_EOF_MODE_CH0** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn with_out_eof_mode_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_EOF_MODE_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_EOF_MODE_CH0_MASK
            },
        }
    }

    /// Set **OUT_AUTO_WRBACK_CH0** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn with_out_auto_wrback_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_AUTO_WRBACK_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_AUTO_WRBACK_CH0_MASK
            },
        }
    }

    /// Set **OUT_LOOP_TEST_CH0** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_out_loop_test_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_LOOP_TEST_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_LOOP_TEST_CH0_MASK
            },
        }
    }

    /// Set **OUT_RST_CH0** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn with_out_rst_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch0::OUT_RST_CH0_MASK
            } else {
                self.bits & !OutConf0Ch0::OUT_RST_CH0_MASK
            },
        }
    }
}

impl Default for OutConf0Ch0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutConf0Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf0Ch0Val {
        OutConf0Ch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf0Ch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf0Ch0Val) -> OutConf0Ch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF1_CH0 Register
///
/// Configure 1 register of Tx channel 0
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **OUT_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct OutConf1Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf1Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf1Ch0 {
    // ======== Field Constants ========

    /// **OUT_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OUT_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const OUT_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const OUT_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **OUT_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_CHECK_OWNER_CH_MASK);
    }

    /// Clear OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_CHECK_OWNER_CH_MASK);
    }
}

/// OUT_CONF1_CH0 value type for batch read/modify/write operations
///
/// Use `outconf1ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf1ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf1Ch0Val {
    bits: u32,
}

impl OutConf1Ch0Val {
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

    /// Read **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn out_check_owner_ch(&self) -> bool {
        (self.bits & OutConf1Ch0::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_out_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf1Ch0::OUT_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !OutConf1Ch0::OUT_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for OutConf1Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutConf1Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf1Ch0Val {
        OutConf1Ch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf1Ch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf1Ch0Val) -> OutConf1Ch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUTFIFO_STATUS_CH0 Register
///
/// Transmit FIFO status of Tx channel 0
///
/// Address offset: 0x00d8
/// Reset value: 0x07800002
///
/// ## Fields
///
/// - **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **OUTFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
///
/// - **OUTFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Tx FIFO empty signal for Tx channel 0.
///
/// - **OUTFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Tx FIFO full signal for Tx channel 0.
#[repr(transparent)]
pub struct OutfifoStatusCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutfifoStatusCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutfifoStatusCh0 {
    // ======== Field Constants ========

    /// **OUT_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const OUT_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const OUT_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const OUT_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **OUT_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OUT_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const OUT_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const OUT_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **OUT_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OUT_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const OUT_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const OUT_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **OUT_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OUT_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const OUT_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const OUT_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **OUTFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const OUTFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const OUTFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **OUTFIFO_EMPTY_CH**
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const OUTFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const OUTFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **OUTFIFO_FULL_CH**
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const OUTFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **OUTFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_CNT_CH_MASK) >> Self::OUTFIFO_CNT_CH_OFFSET
    }

    /// **OUTFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// **OUTFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_FULL_CH_MASK) != 0
    }
}

/// OUTFIFO_STATUS_CH0 value type for batch read/modify/write operations
///
/// Use `outfifostatusch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outfifostatusch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutfifoStatusCh0Val {
    bits: u32,
}

impl OutfifoStatusCh0Val {
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
        Self { bits: 0x07800002 }
    }

    /// Read **OUT_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_4b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_3b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_2b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_1b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **OUTFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_cnt_ch(&self) -> u32 {
        (self.bits & OutfifoStatusCh0::OUTFIFO_CNT_CH_MASK)
            >> OutfifoStatusCh0::OUTFIFO_CNT_CH_OFFSET
    }

    /// Read **OUTFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_empty_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **OUTFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_full_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh0::OUTFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for OutfifoStatusCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutfifoStatusCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutfifoStatusCh0Val {
        OutfifoStatusCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PUSH_CH0 Register
///
/// Push control register of Rx channel 0
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **OUTFIFO_PUSH_CH** `[9]` (WO)
///
///   Set this bit to push data into AHB_DMA FIFO.
///
/// - **OUTFIFO_WDATA_CH** `[8:0]` (RW)
///
///   This register stores the data that need to be pushed into AHB_DMA FIFO.
#[repr(transparent)]
pub struct OutPushCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPushCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPushCh0 {
    // ======== Field Constants ========

    /// **OUTFIFO_PUSH_CH**
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUTFIFO_PUSH_CH_OFFSET: u32 = 9;
    pub const OUTFIFO_PUSH_CH_MASK: u32 = 0x00000200;
    pub const OUTFIFO_PUSH_CH: u32 = 1 << 9;

    /// **OUTFIFO_WDATA_CH**
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const OUTFIFO_WDATA_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_WDATA_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_PUSH_CH** (WO)
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_outfifo_push_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTFIFO_PUSH_CH_MASK);
    }

    /// **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn outfifo_wdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_WDATA_CH_MASK) >> Self::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Write **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn write_outfifo_wdata_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTFIFO_WDATA_CH_MASK)
                | ((value << Self::OUTFIFO_WDATA_CH_OFFSET) & Self::OUTFIFO_WDATA_CH_MASK)
        });
    }
}

/// OUT_PUSH_CH0 value type for batch read/modify/write operations
///
/// Use `outpushch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outpushch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPushCh0Val {
    bits: u32,
}

impl OutPushCh0Val {
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

    /// Read **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn outfifo_wdata_ch(&self) -> u32 {
        (self.bits & OutPushCh0::OUTFIFO_WDATA_CH_MASK) >> OutPushCh0::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Set **OUTFIFO_PUSH_CH** `[9:9]`
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_push_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutPushCh0::OUTFIFO_PUSH_CH_MASK
            } else {
                self.bits & !OutPushCh0::OUTFIFO_PUSH_CH_MASK
            },
        }
    }

    /// Set **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_wdata_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPushCh0::OUTFIFO_WDATA_CH_MASK)
                | ((value << OutPushCh0::OUTFIFO_WDATA_CH_OFFSET)
                    & OutPushCh0::OUTFIFO_WDATA_CH_MASK),
        }
    }
}

impl Default for OutPushCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPushCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPushCh0Val {
        OutPushCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPushCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPushCh0Val) -> OutPushCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_CH0 Register
///
/// Link descriptor configure and control register of Tx channel 0
///
/// Address offset: 0x00e0
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUTLINK_PARK_CH** `[3]` (RO)
///
///   1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
///
/// - **OUTLINK_RESTART_CH** `[2]` (WO)
///
///   Set this bit to restart a new outlink from the last address.
///
/// - **OUTLINK_START_CH** `[1]` (WO)
///
///   Set this bit to start dealing with the outlink descriptors.
///
/// - **OUTLINK_STOP_CH** `[0]` (WO)
///
///   Set this bit to stop dealing with the outlink descriptors.
#[repr(transparent)]
pub struct OutLinkCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkCh0 {
    // ======== Field Constants ========

    /// **OUTLINK_PARK_CH**
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUTLINK_PARK_CH_OFFSET: u32 = 3;
    pub const OUTLINK_PARK_CH_MASK: u32 = 0x00000008;
    pub const OUTLINK_PARK_CH: u32 = 1 << 3;

    /// **OUTLINK_RESTART_CH**
    ///
    /// Set this bit to restart a new outlink from the last address.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUTLINK_RESTART_CH_OFFSET: u32 = 2;
    pub const OUTLINK_RESTART_CH_MASK: u32 = 0x00000004;
    pub const OUTLINK_RESTART_CH: u32 = 1 << 2;

    /// **OUTLINK_START_CH**
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTLINK_START_CH_OFFSET: u32 = 1;
    pub const OUTLINK_START_CH_MASK: u32 = 0x00000002;
    pub const OUTLINK_START_CH: u32 = 1 << 1;

    /// **OUTLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTLINK_STOP_CH_OFFSET: u32 = 0;
    pub const OUTLINK_STOP_CH_MASK: u32 = 0x00000001;
    pub const OUTLINK_STOP_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTLINK_PARK_CH** `[3]` (RO)
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub fn outlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **OUTLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub fn trigger_outlink_restart_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTLINK_RESTART_CH_MASK);
    }

    /// Trigger **OUTLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_START_CH_MASK);
    }

    /// Trigger **OUTLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_STOP_CH_MASK);
    }
}

/// OUT_LINK_CH0 value type for batch read/modify/write operations
///
/// Use `outlinkch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkCh0Val {
    bits: u32,
}

impl OutLinkCh0Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUTLINK_PARK_CH** `[3:3]`
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn outlink_park_ch(&self) -> bool {
        (self.bits & OutLinkCh0::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Set **OUTLINK_RESTART_CH** `[2:2]`
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub const fn with_outlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh0::OUTLINK_RESTART_CH_MASK
            } else {
                self.bits & !OutLinkCh0::OUTLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_START_CH** `[1:1]`
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh0::OUTLINK_START_CH_MASK
            } else {
                self.bits & !OutLinkCh0::OUTLINK_START_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_STOP_CH** `[0:0]`
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh0::OUTLINK_STOP_CH_MASK
            } else {
                self.bits & !OutLinkCh0::OUTLINK_STOP_CH_MASK
            },
        }
    }
}

impl Default for OutLinkCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutLinkCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkCh0Val {
        OutLinkCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkCh0Val) -> OutLinkCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_STATE_CH0 Register
///
/// Transmit status of Tx channel 0
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **OUT_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **OUT_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current outlink descriptor's address.
#[repr(transparent)]
pub struct OutStateCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutStateCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutStateCh0 {
    // ======== Field Constants ========

    /// **OUT_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const OUT_STATE_CH_OFFSET: u32 = 20;
    pub const OUT_STATE_CH_MASK: u32 = 0x00700000;

    /// **OUT_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const OUT_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const OUT_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **OUTLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current outlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const OUTLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **OUT_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_STATE_CH_MASK) >> Self::OUT_STATE_CH_OFFSET
    }

    /// **OUT_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_DSCR_STATE_CH_MASK) >> Self::OUT_DSCR_STATE_CH_OFFSET
    }

    /// **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_ADDR_CH_MASK)
            >> Self::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// OUT_STATE_CH0 value type for batch read/modify/write operations
///
/// Use `outstatech0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outstatech0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutStateCh0Val {
    bits: u32,
}

impl OutStateCh0Val {
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

    /// Read **OUT_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_state_ch(&self) -> u32 {
        (self.bits & OutStateCh0::OUT_STATE_CH_MASK) >> OutStateCh0::OUT_STATE_CH_OFFSET
    }

    /// Read **OUT_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_dscr_state_ch(&self) -> u32 {
        (self.bits & OutStateCh0::OUT_DSCR_STATE_CH_MASK) >> OutStateCh0::OUT_DSCR_STATE_CH_OFFSET
    }

    /// Read **OUTLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & OutStateCh0::OUTLINK_DSCR_ADDR_CH_MASK)
            >> OutStateCh0::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for OutStateCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutStateCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutStateCh0Val {
        OutStateCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_DES_ADDR_CH0 Register
///
/// Outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct OutEofDesAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofDesAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofDesAddrCh0 {
    // ======== Field Constants ========

    /// **OUT_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn out_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_DES_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `outeofdesaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofdesaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofDesAddrCh0Val {
    bits: u32,
}

impl OutEofDesAddrCh0Val {
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

    /// Read **OUT_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofDesAddrCh0::OUT_EOF_DES_ADDR_CH_MASK)
            >> OutEofDesAddrCh0::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofDesAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofDesAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofDesAddrCh0Val {
        OutEofDesAddrCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH0 Register
///
/// The last outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor before the last outlink descriptor.
#[repr(transparent)]
pub struct OutEofBfrDesAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofBfrDesAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofBfrDesAddrCh0 {
    // ======== Field Constants ========

    /// **OUT_EOF_BFR_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_BFR_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_BFR_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `outeofbfrdesaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofbfrdesaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofBfrDesAddrCh0Val {
    bits: u32,
}

impl OutEofBfrDesAddrCh0Val {
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

    /// Read **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofBfrDesAddrCh0::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> OutEofBfrDesAddrCh0::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofBfrDesAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofBfrDesAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofBfrDesAddrCh0Val {
        OutEofBfrDesAddrCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_CH0 Register
///
/// Current inlink descriptor address of Tx channel 0
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **OUTLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current outlink descriptor y.
#[repr(transparent)]
pub struct OutDscrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrCh0 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_CH**
    ///
    /// The address of the current outlink descriptor y.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub fn outlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_CH_MASK) >> Self::OUTLINK_DSCR_CH_OFFSET
    }
}

/// OUT_DSCR_CH0 value type for batch read/modify/write operations
///
/// Use `outdscrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrCh0Val {
    bits: u32,
}

impl OutDscrCh0Val {
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

    /// Read **OUTLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub const fn outlink_dscr_ch(&self) -> u32 {
        (self.bits & OutDscrCh0::OUTLINK_DSCR_CH_MASK) >> OutDscrCh0::OUTLINK_DSCR_CH_OFFSET
    }
}

impl Default for OutDscrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrCh0Val {
        OutDscrCh0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF0_CH0 Register
///
/// The last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last outlink descriptor y-1.
#[repr(transparent)]
pub struct OutDscrBf0Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf0Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf0Ch0 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF0_CH**
    ///
    /// The address of the last outlink descriptor y-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF0_CH_MASK)
            >> Self::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

/// OUT_DSCR_BF0_CH0 value type for batch read/modify/write operations
///
/// Use `outdscrbf0ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf0ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf0Ch0Val {
    bits: u32,
}

impl OutDscrBf0Ch0Val {
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

    /// Read **OUTLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub const fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & OutDscrBf0Ch0::OUTLINK_DSCR_BF0_CH_MASK)
            >> OutDscrBf0Ch0::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for OutDscrBf0Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf0Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf0Ch0Val {
        OutDscrBf0Ch0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF1_CH0 Register
///
/// The second-to-last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct OutDscrBf1Ch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf1Ch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf1Ch0 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF1_CH_MASK)
            >> Self::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

/// OUT_DSCR_BF1_CH0 value type for batch read/modify/write operations
///
/// Use `outdscrbf1ch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf1ch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf1Ch0Val {
    bits: u32,
}

impl OutDscrBf1Ch0Val {
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

    /// Read **OUTLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & OutDscrBf1Ch0::OUTLINK_DSCR_BF1_CH_MASK)
            >> OutDscrBf1Ch0::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for OutDscrBf1Ch0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf1Ch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf1Ch0Val {
        OutDscrBf1Ch0Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PRI_CH0 Register
///
/// Priority register of Tx channel 0.
///
/// Address offset: 0x00fc
///
/// ## Fields
///
/// - **TX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Tx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct OutPriCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPriCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPriCh0 {
    // ======== Field Constants ========

    /// **TX_PRI_CH**
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_PRI_CH_OFFSET: u32 = 0;
    pub const TX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn tx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PRI_CH_MASK) >> Self::TX_PRI_CH_OFFSET
    }

    /// Write **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_tx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PRI_CH_MASK) | ((value << Self::TX_PRI_CH_OFFSET) & Self::TX_PRI_CH_MASK)
        });
    }
}

/// OUT_PRI_CH0 value type for batch read/modify/write operations
///
/// Use `outprich0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outprich0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPriCh0Val {
    bits: u32,
}

impl OutPriCh0Val {
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

    /// Read **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn tx_pri_ch(&self) -> u32 {
        (self.bits & OutPriCh0::TX_PRI_CH_MASK) >> OutPriCh0::TX_PRI_CH_OFFSET
    }

    /// Set **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_tx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPriCh0::TX_PRI_CH_MASK)
                | ((value << OutPriCh0::TX_PRI_CH_OFFSET) & OutPriCh0::TX_PRI_CH_MASK),
        }
    }
}

impl Default for OutPriCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPriCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPriCh0Val {
        OutPriCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPriCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPriCh0Val) -> OutPriCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_PERI_SEL_CH0 Register
///
/// Peripheral selection of Tx channel 0
///
/// Address offset: 0x0100
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_OUT_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct OutPeriSelCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPeriSelCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPeriSelCh0 {
    // ======== Field Constants ========

    /// **PERI_OUT_SEL_CH**
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_OUT_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_OUT_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_out_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_OUT_SEL_CH_MASK) >> Self::PERI_OUT_SEL_CH_OFFSET
    }

    /// Write **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_out_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_OUT_SEL_CH_MASK)
                | ((value << Self::PERI_OUT_SEL_CH_OFFSET) & Self::PERI_OUT_SEL_CH_MASK)
        });
    }
}

/// OUT_PERI_SEL_CH0 value type for batch read/modify/write operations
///
/// Use `outperiselch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outperiselch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPeriSelCh0Val {
    bits: u32,
}

impl OutPeriSelCh0Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_out_sel_ch(&self) -> u32 {
        (self.bits & OutPeriSelCh0::PERI_OUT_SEL_CH_MASK) >> OutPeriSelCh0::PERI_OUT_SEL_CH_OFFSET
    }

    /// Set **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_out_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPeriSelCh0::PERI_OUT_SEL_CH_MASK)
                | ((value << OutPeriSelCh0::PERI_OUT_SEL_CH_OFFSET)
                    & OutPeriSelCh0::PERI_OUT_SEL_CH_MASK),
        }
    }
}

impl Default for OutPeriSelCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutPeriSelCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPeriSelCh0Val {
        OutPeriSelCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPeriSelCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPeriSelCh0Val) -> OutPeriSelCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CONF0_CH1 Register
///
/// Configure 0 register of Rx channel 0
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **IN_ETM_EN_CH** `[5]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
///
/// - **MEM_TRANS_EN_CH** `[4]` (RW)
///
///   Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
///
/// - **IN_DATA_BURST_EN_CH** `[3]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
///
/// - **INDSCR_BURST_EN_CH** `[2]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
///
/// - **IN_LOOP_TEST_CH** `[1]` (RW)
///
///   reserved
///
/// - **IN_RST_CH** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
#[repr(transparent)]
pub struct InConf0Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf0Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf0Ch1 {
    // ======== Field Constants ========

    /// **IN_ETM_EN_CH**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IN_ETM_EN_CH_OFFSET: u32 = 5;
    pub const IN_ETM_EN_CH_MASK: u32 = 0x00000020;
    pub const IN_ETM_EN_CH: u32 = 1 << 5;

    /// **MEM_TRANS_EN_CH**
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TRANS_EN_CH_OFFSET: u32 = 4;
    pub const MEM_TRANS_EN_CH_MASK: u32 = 0x00000010;
    pub const MEM_TRANS_EN_CH: u32 = 1 << 4;

    /// **IN_DATA_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DATA_BURST_EN_CH_OFFSET: u32 = 3;
    pub const IN_DATA_BURST_EN_CH_MASK: u32 = 0x00000008;
    pub const IN_DATA_BURST_EN_CH: u32 = 1 << 3;

    /// **INDSCR_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INDSCR_BURST_EN_CH_OFFSET: u32 = 2;
    pub const INDSCR_BURST_EN_CH_MASK: u32 = 0x00000004;
    pub const INDSCR_BURST_EN_CH: u32 = 1 << 2;

    /// **IN_LOOP_TEST_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_LOOP_TEST_CH_OFFSET: u32 = 1;
    pub const IN_LOOP_TEST_CH_MASK: u32 = 0x00000002;
    pub const IN_LOOP_TEST_CH: u32 = 1 << 1;

    /// **IN_RST_CH**
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_RST_CH_OFFSET: u32 = 0;
    pub const IN_RST_CH_MASK: u32 = 0x00000001;
    pub const IN_RST_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_ETM_EN_CH** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_ETM_EN_CH_MASK) != 0
    }

    /// Set IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_ETM_EN_CH_MASK);
    }

    /// Clear IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_ETM_EN_CH_MASK);
    }

    /// **MEM_TRANS_EN_CH** `[4]` (RW)
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Set MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::MEM_TRANS_EN_CH_MASK);
    }

    /// Clear MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_TRANS_EN_CH_MASK);
    }

    /// **IN_DATA_BURST_EN_CH** `[3]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub fn in_data_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Set IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// Clear IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// **INDSCR_BURST_EN_CH** `[2]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Set INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// Clear INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// **IN_LOOP_TEST_CH** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Set IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_LOOP_TEST_CH_MASK);
    }

    /// Clear IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_LOOP_TEST_CH_MASK);
    }

    /// **IN_RST_CH** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub fn in_rst_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_RST_CH_MASK) != 0
    }

    /// Set IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_rst_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_RST_CH_MASK);
    }

    /// Clear IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_rst_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_RST_CH_MASK);
    }
}

/// IN_CONF0_CH1 value type for batch read/modify/write operations
///
/// Use `inconf0ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf0ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf0Ch1Val {
    bits: u32,
}

impl InConf0Ch1Val {
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

    /// Read **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn in_etm_en_ch(&self) -> bool {
        (self.bits & InConf0Ch1::IN_ETM_EN_CH_MASK) != 0
    }

    /// Read **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn mem_trans_en_ch(&self) -> bool {
        (self.bits & InConf0Ch1::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Read **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn in_data_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch1::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Read **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn indscr_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch1::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Read **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_loop_test_ch(&self) -> bool {
        (self.bits & InConf0Ch1::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Read **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn in_rst_ch(&self) -> bool {
        (self.bits & InConf0Ch1::IN_RST_CH_MASK) != 0
    }

    /// Set **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn with_in_etm_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::IN_ETM_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch1::IN_ETM_EN_CH_MASK
            },
        }
    }

    /// Set **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn with_mem_trans_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::MEM_TRANS_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch1::MEM_TRANS_EN_CH_MASK
            },
        }
    }

    /// Set **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_in_data_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::IN_DATA_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch1::IN_DATA_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_indscr_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::INDSCR_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch1::INDSCR_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_in_loop_test_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::IN_LOOP_TEST_CH_MASK
            } else {
                self.bits & !InConf0Ch1::IN_LOOP_TEST_CH_MASK
            },
        }
    }

    /// Set **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn with_in_rst_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch1::IN_RST_CH_MASK
            } else {
                self.bits & !InConf0Ch1::IN_RST_CH_MASK
            },
        }
    }
}

impl Default for InConf0Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf0Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf0Ch1Val {
        InConf0Ch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf0Ch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf0Ch1Val) -> InConf0Ch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CONF1_CH1 Register
///
/// Configure 1 register of Rx channel 0
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **IN_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct InConf1Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf1Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf1Ch1 {
    // ======== Field Constants ========

    /// **IN_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IN_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const IN_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const IN_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **IN_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_check_owner_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CHECK_OWNER_CH_MASK);
    }

    /// Clear IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_CHECK_OWNER_CH_MASK);
    }
}

/// IN_CONF1_CH1 value type for batch read/modify/write operations
///
/// Use `inconf1ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf1ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf1Ch1Val {
    bits: u32,
}

impl InConf1Ch1Val {
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

    /// Read **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn in_check_owner_ch(&self) -> bool {
        (self.bits & InConf1Ch1::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_in_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf1Ch1::IN_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !InConf1Ch1::IN_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for InConf1Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf1Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf1Ch1Val {
        InConf1Ch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf1Ch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf1Ch1Val) -> InConf1Ch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INFIFO_STATUS_CH1 Register
///
/// Receive FIFO status of Rx channel 0
///
/// Address offset: 0x0138
/// Reset value: 0x07800003
///
/// ## Fields
///
/// - **IN_BUF_HUNGRY_CH** `[27]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **INFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
///
/// - **INFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Rx FIFO empty signal for Rx channel 0.
///
/// - **INFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Rx FIFO full signal for Rx channel 0.
#[repr(transparent)]
pub struct InfifoStatusCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfifoStatusCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfifoStatusCh1 {
    // ======== Field Constants ========

    /// **IN_BUF_HUNGRY_CH**
    ///
    /// reserved
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const IN_BUF_HUNGRY_CH_OFFSET: u32 = 27;
    pub const IN_BUF_HUNGRY_CH_MASK: u32 = 0x08000000;
    pub const IN_BUF_HUNGRY_CH: u32 = 1 << 27;

    /// **IN_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const IN_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const IN_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const IN_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **IN_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const IN_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const IN_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const IN_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **IN_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const IN_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const IN_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const IN_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **IN_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const IN_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const IN_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const IN_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **INFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const INFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const INFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **INFIFO_EMPTY_CH**
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const INFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const INFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **INFIFO_FULL_CH**
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const INFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const INFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_BUF_HUNGRY_CH** `[27]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **INFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_CNT_CH_MASK) >> Self::INFIFO_CNT_CH_OFFSET
    }

    /// **INFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// **INFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_FULL_CH_MASK) != 0
    }
}

/// INFIFO_STATUS_CH1 value type for batch read/modify/write operations
///
/// Use `infifostatusch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `infifostatusch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InfifoStatusCh1Val {
    bits: u32,
}

impl InfifoStatusCh1Val {
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
        Self { bits: 0x07800003 }
    }

    /// Read **IN_BUF_HUNGRY_CH** `[27:27]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_buf_hungry_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_4b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_3b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_2b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_1b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **INFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_cnt_ch(&self) -> u32 {
        (self.bits & InfifoStatusCh1::INFIFO_CNT_CH_MASK) >> InfifoStatusCh1::INFIFO_CNT_CH_OFFSET
    }

    /// Read **INFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_empty_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **INFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_full_ch(&self) -> bool {
        (self.bits & InfifoStatusCh1::INFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for InfifoStatusCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InfifoStatusCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InfifoStatusCh1Val {
        InfifoStatusCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_POP_CH1 Register
///
/// Pop control register of Rx channel 0
///
/// Address offset: 0x013c
/// Reset value: 0x00000800
///
/// ## Fields
///
/// - **INFIFO_POP_CH** `[12]` (WO)
///
///   Set this bit to pop data from AHB_DMA FIFO.
///
/// - **INFIFO_RDATA_CH** `[11:0]` (RO)
///
///   This register stores the data popping from AHB_DMA FIFO.
#[repr(transparent)]
pub struct InPopCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPopCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPopCh1 {
    // ======== Field Constants ========

    /// **INFIFO_POP_CH**
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INFIFO_POP_CH_OFFSET: u32 = 12;
    pub const INFIFO_POP_CH_MASK: u32 = 0x00001000;
    pub const INFIFO_POP_CH: u32 = 1 << 12;

    /// **INFIFO_RDATA_CH**
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const INFIFO_RDATA_CH_OFFSET: u32 = 0;
    pub const INFIFO_RDATA_CH_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_POP_CH** (WO)
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_infifo_pop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INFIFO_POP_CH_MASK);
    }

    /// **INFIFO_RDATA_CH** `[11:0]` (RO)
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_RDATA_CH_MASK) >> Self::INFIFO_RDATA_CH_OFFSET
    }
}

/// IN_POP_CH1 value type for batch read/modify/write operations
///
/// Use `inpopch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inpopch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPopCh1Val {
    bits: u32,
}

impl InPopCh1Val {
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
        Self { bits: 0x00000800 }
    }

    /// Read **INFIFO_RDATA_CH** `[11:0]`
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn infifo_rdata_ch(&self) -> u32 {
        (self.bits & InPopCh1::INFIFO_RDATA_CH_MASK) >> InPopCh1::INFIFO_RDATA_CH_OFFSET
    }

    /// Set **INFIFO_POP_CH** `[12:12]`
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_infifo_pop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InPopCh1::INFIFO_POP_CH_MASK
            } else {
                self.bits & !InPopCh1::INFIFO_POP_CH_MASK
            },
        }
    }
}

impl Default for InPopCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPopCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPopCh1Val {
        InPopCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPopCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPopCh1Val) -> InPopCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_CH1 Register
///
/// Link descriptor configure and control register of Rx channel 0
///
/// Address offset: 0x0140
/// Reset value: 0x00000011
///
/// ## Fields
///
/// - **INLINK_PARK_CH** `[4]` (RO)
///
///   1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
///
/// - **INLINK_RESTART_CH** `[3]` (WO)
///
///   Set this bit to mount a new inlink descriptor.
///
/// - **INLINK_START_CH** `[2]` (WO)
///
///   Set this bit to start dealing with the inlink descriptors.
///
/// - **INLINK_STOP_CH** `[1]` (WO)
///
///   Set this bit to stop dealing with the inlink descriptors.
///
/// - **INLINK_AUTO_RET_CH** `[0]` (RW)
///
///   Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
#[repr(transparent)]
pub struct InLinkCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkCh1 {
    // ======== Field Constants ========

    /// **INLINK_PARK_CH**
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const INLINK_PARK_CH_OFFSET: u32 = 4;
    pub const INLINK_PARK_CH_MASK: u32 = 0x00000010;
    pub const INLINK_PARK_CH: u32 = 1 << 4;

    /// **INLINK_RESTART_CH**
    ///
    /// Set this bit to mount a new inlink descriptor.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INLINK_RESTART_CH_OFFSET: u32 = 3;
    pub const INLINK_RESTART_CH_MASK: u32 = 0x00000008;
    pub const INLINK_RESTART_CH: u32 = 1 << 3;

    /// **INLINK_START_CH**
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INLINK_START_CH_OFFSET: u32 = 2;
    pub const INLINK_START_CH_MASK: u32 = 0x00000004;
    pub const INLINK_START_CH: u32 = 1 << 2;

    /// **INLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INLINK_STOP_CH_OFFSET: u32 = 1;
    pub const INLINK_STOP_CH_MASK: u32 = 0x00000002;
    pub const INLINK_STOP_CH: u32 = 1 << 1;

    /// **INLINK_AUTO_RET_CH**
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INLINK_AUTO_RET_CH_OFFSET: u32 = 0;
    pub const INLINK_AUTO_RET_CH_MASK: u32 = 0x00000001;
    pub const INLINK_AUTO_RET_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INLINK_PARK_CH** `[4]` (RO)
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub fn inlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **INLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub fn trigger_inlink_restart_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_RESTART_CH_MASK);
    }

    /// Trigger **INLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_START_CH_MASK);
    }

    /// Trigger **INLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_STOP_CH_MASK);
    }

    /// **INLINK_AUTO_RET_CH** `[0]` (RW)
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INLINK_AUTO_RET_CH_MASK);
    }

    /// Clear INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INLINK_AUTO_RET_CH_MASK);
    }
}

/// IN_LINK_CH1 value type for batch read/modify/write operations
///
/// Use `inlinkch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkCh1Val {
    bits: u32,
}

impl InLinkCh1Val {
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
        Self { bits: 0x00000011 }
    }

    /// Read **INLINK_PARK_CH** `[4:4]`
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn inlink_park_ch(&self) -> bool {
        (self.bits & InLinkCh1::INLINK_PARK_CH_MASK) != 0
    }

    /// Read **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn inlink_auto_ret_ch(&self) -> bool {
        (self.bits & InLinkCh1::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set **INLINK_RESTART_CH** `[3:3]`
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub const fn with_inlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh1::INLINK_RESTART_CH_MASK
            } else {
                self.bits & !InLinkCh1::INLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **INLINK_START_CH** `[2:2]`
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh1::INLINK_START_CH_MASK
            } else {
                self.bits & !InLinkCh1::INLINK_START_CH_MASK
            },
        }
    }

    /// Set **INLINK_STOP_CH** `[1:1]`
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh1::INLINK_STOP_CH_MASK
            } else {
                self.bits & !InLinkCh1::INLINK_STOP_CH_MASK
            },
        }
    }

    /// Set **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn with_inlink_auto_ret_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh1::INLINK_AUTO_RET_CH_MASK
            } else {
                self.bits & !InLinkCh1::INLINK_AUTO_RET_CH_MASK
            },
        }
    }
}

impl Default for InLinkCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InLinkCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkCh1Val {
        InLinkCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkCh1Val) -> InLinkCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_STATE_CH1 Register
///
/// Receive status of Rx channel 0
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **IN_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **IN_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current inlink descriptor's address.
#[repr(transparent)]
pub struct InStateCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InStateCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InStateCh1 {
    // ======== Field Constants ========

    /// **IN_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const IN_STATE_CH_OFFSET: u32 = 20;
    pub const IN_STATE_CH_MASK: u32 = 0x00700000;

    /// **IN_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const IN_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const IN_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **INLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current inlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const INLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **IN_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_STATE_CH_MASK) >> Self::IN_STATE_CH_OFFSET
    }

    /// **IN_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_DSCR_STATE_CH_MASK) >> Self::IN_DSCR_STATE_CH_OFFSET
    }

    /// **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_ADDR_CH_MASK)
            >> Self::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// IN_STATE_CH1 value type for batch read/modify/write operations
///
/// Use `instatech1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instatech1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InStateCh1Val {
    bits: u32,
}

impl InStateCh1Val {
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

    /// Read **IN_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_state_ch(&self) -> u32 {
        (self.bits & InStateCh1::IN_STATE_CH_MASK) >> InStateCh1::IN_STATE_CH_OFFSET
    }

    /// Read **IN_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_dscr_state_ch(&self) -> u32 {
        (self.bits & InStateCh1::IN_DSCR_STATE_CH_MASK) >> InStateCh1::IN_DSCR_STATE_CH_OFFSET
    }

    /// Read **INLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & InStateCh1::INLINK_DSCR_ADDR_CH_MASK) >> InStateCh1::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for InStateCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InStateCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InStateCh1Val {
        InStateCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_SUC_EOF_DES_ADDR_CH1 Register
///
/// Inlink descriptor address when EOF occurs of Rx channel 0
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct InSucEofDesAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InSucEofDesAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InSucEofDesAddrCh1 {
    // ======== Field Constants ========

    /// **IN_SUC_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_SUC_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_SUC_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_SUC_EOF_DES_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `insuceofdesaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `insuceofdesaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InSucEofDesAddrCh1Val {
    bits: u32,
}

impl InSucEofDesAddrCh1Val {
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

    /// Read **IN_SUC_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InSucEofDesAddrCh1::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> InSucEofDesAddrCh1::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InSucEofDesAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InSucEofDesAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InSucEofDesAddrCh1Val {
        InSucEofDesAddrCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_ERR_EOF_DES_ADDR_CH1 Register
///
/// Inlink descriptor address when errors occur of Rx channel 0
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
#[repr(transparent)]
pub struct InErrEofDesAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InErrEofDesAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InErrEofDesAddrCh1 {
    // ======== Field Constants ========

    /// **IN_ERR_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_ERR_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_ERR_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_ERR_EOF_DES_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `inerreofdesaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inerreofdesaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InErrEofDesAddrCh1Val {
    bits: u32,
}

impl InErrEofDesAddrCh1Val {
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

    /// Read **IN_ERR_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InErrEofDesAddrCh1::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> InErrEofDesAddrCh1::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InErrEofDesAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InErrEofDesAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InErrEofDesAddrCh1Val {
        InErrEofDesAddrCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_CH1 Register
///
/// Current inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **INLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current inlink descriptor x.
#[repr(transparent)]
pub struct InDscrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrCh1 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_CH**
    ///
    /// The address of the current inlink descriptor x.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub fn inlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_CH_MASK) >> Self::INLINK_DSCR_CH_OFFSET
    }
}

/// IN_DSCR_CH1 value type for batch read/modify/write operations
///
/// Use `indscrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrCh1Val {
    bits: u32,
}

impl InDscrCh1Val {
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

    /// Read **INLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub const fn inlink_dscr_ch(&self) -> u32 {
        (self.bits & InDscrCh1::INLINK_DSCR_CH_MASK) >> InDscrCh1::INLINK_DSCR_CH_OFFSET
    }
}

impl Default for InDscrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrCh1Val {
        InDscrCh1Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF0_CH1 Register
///
/// The last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last inlink descriptor x-1.
#[repr(transparent)]
pub struct InDscrBf0Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf0Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf0Ch1 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF0_CH**
    ///
    /// The address of the last inlink descriptor x-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF0_CH_MASK) >> Self::INLINK_DSCR_BF0_CH_OFFSET
    }
}

/// IN_DSCR_BF0_CH1 value type for batch read/modify/write operations
///
/// Use `indscrbf0ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf0ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf0Ch1Val {
    bits: u32,
}

impl InDscrBf0Ch1Val {
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

    /// Read **INLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub const fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & InDscrBf0Ch1::INLINK_DSCR_BF0_CH_MASK)
            >> InDscrBf0Ch1::INLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for InDscrBf0Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf0Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf0Ch1Val {
        InDscrBf0Ch1Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF1_CH1 Register
///
/// The second-to-last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct InDscrBf1Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf1Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf1Ch1 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF1_CH_MASK) >> Self::INLINK_DSCR_BF1_CH_OFFSET
    }
}

/// IN_DSCR_BF1_CH1 value type for batch read/modify/write operations
///
/// Use `indscrbf1ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf1ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf1Ch1Val {
    bits: u32,
}

impl InDscrBf1Ch1Val {
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

    /// Read **INLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & InDscrBf1Ch1::INLINK_DSCR_BF1_CH_MASK)
            >> InDscrBf1Ch1::INLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for InDscrBf1Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf1Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf1Ch1Val {
        InDscrBf1Ch1Val::from_bits(self.value.read_bits())
    }
}

/// IN_PRI_CH1 Register
///
/// Priority register of Rx channel 0
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **RX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Rx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct InPriCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPriCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPriCh1 {
    // ======== Field Constants ========

    /// **RX_PRI_CH**
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_PRI_CH_OFFSET: u32 = 0;
    pub const RX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn rx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PRI_CH_MASK) >> Self::RX_PRI_CH_OFFSET
    }

    /// Write **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_rx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PRI_CH_MASK) | ((value << Self::RX_PRI_CH_OFFSET) & Self::RX_PRI_CH_MASK)
        });
    }
}

/// IN_PRI_CH1 value type for batch read/modify/write operations
///
/// Use `inprich1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inprich1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPriCh1Val {
    bits: u32,
}

impl InPriCh1Val {
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

    /// Read **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn rx_pri_ch(&self) -> u32 {
        (self.bits & InPriCh1::RX_PRI_CH_MASK) >> InPriCh1::RX_PRI_CH_OFFSET
    }

    /// Set **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_rx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPriCh1::RX_PRI_CH_MASK)
                | ((value << InPriCh1::RX_PRI_CH_OFFSET) & InPriCh1::RX_PRI_CH_MASK),
        }
    }
}

impl Default for InPriCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InPriCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPriCh1Val {
        InPriCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPriCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPriCh1Val) -> InPriCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_PERI_SEL_CH1 Register
///
/// Peripheral selection of Rx channel 0
///
/// Address offset: 0x0160
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_IN_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct InPeriSelCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPeriSelCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPeriSelCh1 {
    // ======== Field Constants ========

    /// **PERI_IN_SEL_CH**
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_IN_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_IN_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_in_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_IN_SEL_CH_MASK) >> Self::PERI_IN_SEL_CH_OFFSET
    }

    /// Write **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_in_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_IN_SEL_CH_MASK)
                | ((value << Self::PERI_IN_SEL_CH_OFFSET) & Self::PERI_IN_SEL_CH_MASK)
        });
    }
}

/// IN_PERI_SEL_CH1 value type for batch read/modify/write operations
///
/// Use `inperiselch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inperiselch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPeriSelCh1Val {
    bits: u32,
}

impl InPeriSelCh1Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_in_sel_ch(&self) -> u32 {
        (self.bits & InPeriSelCh1::PERI_IN_SEL_CH_MASK) >> InPeriSelCh1::PERI_IN_SEL_CH_OFFSET
    }

    /// Set **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_in_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPeriSelCh1::PERI_IN_SEL_CH_MASK)
                | ((value << InPeriSelCh1::PERI_IN_SEL_CH_OFFSET)
                    & InPeriSelCh1::PERI_IN_SEL_CH_MASK),
        }
    }
}

impl Default for InPeriSelCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPeriSelCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPeriSelCh1Val {
        InPeriSelCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPeriSelCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPeriSelCh1Val) -> InPeriSelCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF0_CH1 Register
///
/// Configure 0 register of Tx channel 1
///
/// Address offset: 0x0190
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUT_ETM_EN_CH** `[6]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
///
/// - **OUT_DATA_BURST_EN_CH** `[5]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
///
/// - **OUTDSCR_BURST_EN_CH** `[4]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
///
/// - **OUT_EOF_MODE_CH** `[3]` (RW)
///
///   EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
///
/// - **OUT_AUTO_WRBACK_CH** `[2]` (RW)
///
///   Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
///
/// - **OUT_LOOP_TEST_CH** `[1]` (RW)
///
///   reserved
///
/// - **OUT_RST_CH** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
#[repr(transparent)]
pub struct OutConf0Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf0Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf0Ch1 {
    // ======== Field Constants ========

    /// **OUT_ETM_EN_CH**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUT_ETM_EN_CH_OFFSET: u32 = 6;
    pub const OUT_ETM_EN_CH_MASK: u32 = 0x00000040;
    pub const OUT_ETM_EN_CH: u32 = 1 << 6;

    /// **OUT_DATA_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUT_DATA_BURST_EN_CH_OFFSET: u32 = 5;
    pub const OUT_DATA_BURST_EN_CH_MASK: u32 = 0x00000020;
    pub const OUT_DATA_BURST_EN_CH: u32 = 1 << 5;

    /// **OUTDSCR_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTDSCR_BURST_EN_CH_OFFSET: u32 = 4;
    pub const OUTDSCR_BURST_EN_CH_MASK: u32 = 0x00000010;
    pub const OUTDSCR_BURST_EN_CH: u32 = 1 << 4;

    /// **OUT_EOF_MODE_CH**
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_EOF_MODE_CH_OFFSET: u32 = 3;
    pub const OUT_EOF_MODE_CH_MASK: u32 = 0x00000008;
    pub const OUT_EOF_MODE_CH: u32 = 1 << 3;

    /// **OUT_AUTO_WRBACK_CH**
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_AUTO_WRBACK_CH_OFFSET: u32 = 2;
    pub const OUT_AUTO_WRBACK_CH_MASK: u32 = 0x00000004;
    pub const OUT_AUTO_WRBACK_CH: u32 = 1 << 2;

    /// **OUT_LOOP_TEST_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_LOOP_TEST_CH_OFFSET: u32 = 1;
    pub const OUT_LOOP_TEST_CH_MASK: u32 = 0x00000002;
    pub const OUT_LOOP_TEST_CH: u32 = 1 << 1;

    /// **OUT_RST_CH**
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_RST_CH_OFFSET: u32 = 0;
    pub const OUT_RST_CH_MASK: u32 = 0x00000001;
    pub const OUT_RST_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_ETM_EN_CH** `[6]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub fn out_etm_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_ETM_EN_CH_MASK) != 0
    }

    /// Set OUT_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_etm_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_ETM_EN_CH_MASK);
    }

    /// Clear OUT_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_etm_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_ETM_EN_CH_MASK);
    }

    /// **OUT_DATA_BURST_EN_CH** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub fn out_data_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Set OUT_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DATA_BURST_EN_CH_MASK);
    }

    /// Clear OUT_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DATA_BURST_EN_CH_MASK);
    }

    /// **OUTDSCR_BURST_EN_CH** `[4]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Set OUTDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outdscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTDSCR_BURST_EN_CH_MASK);
    }

    /// Clear OUTDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outdscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTDSCR_BURST_EN_CH_MASK);
    }

    /// **OUT_EOF_MODE_CH** `[3]` (RW)
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub fn out_eof_mode_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_MODE_CH_MASK) != 0
    }

    /// Set OUT_EOF_MODE_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_mode_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_MODE_CH_MASK);
    }

    /// Clear OUT_EOF_MODE_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_mode_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_MODE_CH_MASK);
    }

    /// **OUT_AUTO_WRBACK_CH** `[2]` (RW)
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub fn out_auto_wrback_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_AUTO_WRBACK_CH_MASK) != 0
    }

    /// Set OUT_AUTO_WRBACK_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_auto_wrback_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_AUTO_WRBACK_CH_MASK);
    }

    /// Clear OUT_AUTO_WRBACK_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_auto_wrback_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_AUTO_WRBACK_CH_MASK);
    }

    /// **OUT_LOOP_TEST_CH** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_loop_test_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_LOOP_TEST_CH_MASK) != 0
    }

    /// Set OUT_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_loop_test_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_LOOP_TEST_CH_MASK);
    }

    /// Clear OUT_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_loop_test_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_LOOP_TEST_CH_MASK);
    }

    /// **OUT_RST_CH** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub fn out_rst_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_RST_CH_MASK) != 0
    }

    /// Set OUT_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_rst_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_RST_CH_MASK);
    }

    /// Clear OUT_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_rst_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_RST_CH_MASK);
    }
}

/// OUT_CONF0_CH1 value type for batch read/modify/write operations
///
/// Use `outconf0ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf0ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf0Ch1Val {
    bits: u32,
}

impl OutConf0Ch1Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUT_ETM_EN_CH** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub const fn out_etm_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_ETM_EN_CH_MASK) != 0
    }

    /// Read **OUT_DATA_BURST_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn out_data_burst_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Read **OUTDSCR_BURST_EN_CH** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn outdscr_burst_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUTDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Read **OUT_EOF_MODE_CH** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn out_eof_mode_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_EOF_MODE_CH_MASK) != 0
    }

    /// Read **OUT_AUTO_WRBACK_CH** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn out_auto_wrback_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_AUTO_WRBACK_CH_MASK) != 0
    }

    /// Read **OUT_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_loop_test_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_LOOP_TEST_CH_MASK) != 0
    }

    /// Read **OUT_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn out_rst_ch(&self) -> bool {
        (self.bits & OutConf0Ch1::OUT_RST_CH_MASK) != 0
    }

    /// Set **OUT_ETM_EN_CH** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub const fn with_out_etm_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_ETM_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_ETM_EN_CH_MASK
            },
        }
    }

    /// Set **OUT_DATA_BURST_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_out_data_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_DATA_BURST_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_DATA_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **OUTDSCR_BURST_EN_CH** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_outdscr_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUTDSCR_BURST_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUTDSCR_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **OUT_EOF_MODE_CH** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn with_out_eof_mode_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_EOF_MODE_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_EOF_MODE_CH_MASK
            },
        }
    }

    /// Set **OUT_AUTO_WRBACK_CH** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn with_out_auto_wrback_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_AUTO_WRBACK_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_AUTO_WRBACK_CH_MASK
            },
        }
    }

    /// Set **OUT_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_out_loop_test_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_LOOP_TEST_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_LOOP_TEST_CH_MASK
            },
        }
    }

    /// Set **OUT_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn with_out_rst_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch1::OUT_RST_CH_MASK
            } else {
                self.bits & !OutConf0Ch1::OUT_RST_CH_MASK
            },
        }
    }
}

impl Default for OutConf0Ch1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutConf0Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf0Ch1Val {
        OutConf0Ch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf0Ch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf0Ch1Val) -> OutConf0Ch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF1_CH1 Register
///
/// Configure 1 register of Tx channel 0
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **OUT_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct OutConf1Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf1Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf1Ch1 {
    // ======== Field Constants ========

    /// **OUT_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OUT_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const OUT_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const OUT_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **OUT_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_CHECK_OWNER_CH_MASK);
    }

    /// Clear OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_CHECK_OWNER_CH_MASK);
    }
}

/// OUT_CONF1_CH1 value type for batch read/modify/write operations
///
/// Use `outconf1ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf1ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf1Ch1Val {
    bits: u32,
}

impl OutConf1Ch1Val {
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

    /// Read **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn out_check_owner_ch(&self) -> bool {
        (self.bits & OutConf1Ch1::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_out_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf1Ch1::OUT_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !OutConf1Ch1::OUT_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for OutConf1Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutConf1Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf1Ch1Val {
        OutConf1Ch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf1Ch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf1Ch1Val) -> OutConf1Ch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUTFIFO_STATUS_CH1 Register
///
/// Transmit FIFO status of Tx channel 0
///
/// Address offset: 0x0198
/// Reset value: 0x07800002
///
/// ## Fields
///
/// - **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **OUTFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
///
/// - **OUTFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Tx FIFO empty signal for Tx channel 0.
///
/// - **OUTFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Tx FIFO full signal for Tx channel 0.
#[repr(transparent)]
pub struct OutfifoStatusCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutfifoStatusCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutfifoStatusCh1 {
    // ======== Field Constants ========

    /// **OUT_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const OUT_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const OUT_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const OUT_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **OUT_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OUT_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const OUT_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const OUT_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **OUT_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OUT_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const OUT_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const OUT_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **OUT_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OUT_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const OUT_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const OUT_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **OUTFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const OUTFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const OUTFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **OUTFIFO_EMPTY_CH**
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const OUTFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const OUTFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **OUTFIFO_FULL_CH**
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const OUTFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **OUTFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_CNT_CH_MASK) >> Self::OUTFIFO_CNT_CH_OFFSET
    }

    /// **OUTFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// **OUTFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_FULL_CH_MASK) != 0
    }
}

/// OUTFIFO_STATUS_CH1 value type for batch read/modify/write operations
///
/// Use `outfifostatusch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outfifostatusch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutfifoStatusCh1Val {
    bits: u32,
}

impl OutfifoStatusCh1Val {
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
        Self { bits: 0x07800002 }
    }

    /// Read **OUT_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_4b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_3b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_2b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_1b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **OUTFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_cnt_ch(&self) -> u32 {
        (self.bits & OutfifoStatusCh1::OUTFIFO_CNT_CH_MASK)
            >> OutfifoStatusCh1::OUTFIFO_CNT_CH_OFFSET
    }

    /// Read **OUTFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_empty_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **OUTFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_full_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh1::OUTFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for OutfifoStatusCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutfifoStatusCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutfifoStatusCh1Val {
        OutfifoStatusCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PUSH_CH1 Register
///
/// Push control register of Rx channel 0
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **OUTFIFO_PUSH_CH** `[9]` (WO)
///
///   Set this bit to push data into AHB_DMA FIFO.
///
/// - **OUTFIFO_WDATA_CH** `[8:0]` (RW)
///
///   This register stores the data that need to be pushed into AHB_DMA FIFO.
#[repr(transparent)]
pub struct OutPushCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPushCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPushCh1 {
    // ======== Field Constants ========

    /// **OUTFIFO_PUSH_CH**
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUTFIFO_PUSH_CH_OFFSET: u32 = 9;
    pub const OUTFIFO_PUSH_CH_MASK: u32 = 0x00000200;
    pub const OUTFIFO_PUSH_CH: u32 = 1 << 9;

    /// **OUTFIFO_WDATA_CH**
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const OUTFIFO_WDATA_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_WDATA_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_PUSH_CH** (WO)
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_outfifo_push_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTFIFO_PUSH_CH_MASK);
    }

    /// **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn outfifo_wdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_WDATA_CH_MASK) >> Self::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Write **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn write_outfifo_wdata_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTFIFO_WDATA_CH_MASK)
                | ((value << Self::OUTFIFO_WDATA_CH_OFFSET) & Self::OUTFIFO_WDATA_CH_MASK)
        });
    }
}

/// OUT_PUSH_CH1 value type for batch read/modify/write operations
///
/// Use `outpushch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outpushch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPushCh1Val {
    bits: u32,
}

impl OutPushCh1Val {
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

    /// Read **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn outfifo_wdata_ch(&self) -> u32 {
        (self.bits & OutPushCh1::OUTFIFO_WDATA_CH_MASK) >> OutPushCh1::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Set **OUTFIFO_PUSH_CH** `[9:9]`
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_push_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutPushCh1::OUTFIFO_PUSH_CH_MASK
            } else {
                self.bits & !OutPushCh1::OUTFIFO_PUSH_CH_MASK
            },
        }
    }

    /// Set **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_wdata_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPushCh1::OUTFIFO_WDATA_CH_MASK)
                | ((value << OutPushCh1::OUTFIFO_WDATA_CH_OFFSET)
                    & OutPushCh1::OUTFIFO_WDATA_CH_MASK),
        }
    }
}

impl Default for OutPushCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPushCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPushCh1Val {
        OutPushCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPushCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPushCh1Val) -> OutPushCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_CH1 Register
///
/// Link descriptor configure and control register of Tx channel 0
///
/// Address offset: 0x01a0
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUTLINK_PARK_CH** `[3]` (RO)
///
///   1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
///
/// - **OUTLINK_RESTART_CH** `[2]` (WO)
///
///   Set this bit to restart a new outlink from the last address.
///
/// - **OUTLINK_START_CH** `[1]` (WO)
///
///   Set this bit to start dealing with the outlink descriptors.
///
/// - **OUTLINK_STOP_CH** `[0]` (WO)
///
///   Set this bit to stop dealing with the outlink descriptors.
#[repr(transparent)]
pub struct OutLinkCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkCh1 {
    // ======== Field Constants ========

    /// **OUTLINK_PARK_CH**
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUTLINK_PARK_CH_OFFSET: u32 = 3;
    pub const OUTLINK_PARK_CH_MASK: u32 = 0x00000008;
    pub const OUTLINK_PARK_CH: u32 = 1 << 3;

    /// **OUTLINK_RESTART_CH**
    ///
    /// Set this bit to restart a new outlink from the last address.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUTLINK_RESTART_CH_OFFSET: u32 = 2;
    pub const OUTLINK_RESTART_CH_MASK: u32 = 0x00000004;
    pub const OUTLINK_RESTART_CH: u32 = 1 << 2;

    /// **OUTLINK_START_CH**
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTLINK_START_CH_OFFSET: u32 = 1;
    pub const OUTLINK_START_CH_MASK: u32 = 0x00000002;
    pub const OUTLINK_START_CH: u32 = 1 << 1;

    /// **OUTLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTLINK_STOP_CH_OFFSET: u32 = 0;
    pub const OUTLINK_STOP_CH_MASK: u32 = 0x00000001;
    pub const OUTLINK_STOP_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTLINK_PARK_CH** `[3]` (RO)
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub fn outlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **OUTLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub fn trigger_outlink_restart_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTLINK_RESTART_CH_MASK);
    }

    /// Trigger **OUTLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_START_CH_MASK);
    }

    /// Trigger **OUTLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_STOP_CH_MASK);
    }
}

/// OUT_LINK_CH1 value type for batch read/modify/write operations
///
/// Use `outlinkch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkCh1Val {
    bits: u32,
}

impl OutLinkCh1Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUTLINK_PARK_CH** `[3:3]`
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn outlink_park_ch(&self) -> bool {
        (self.bits & OutLinkCh1::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Set **OUTLINK_RESTART_CH** `[2:2]`
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub const fn with_outlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh1::OUTLINK_RESTART_CH_MASK
            } else {
                self.bits & !OutLinkCh1::OUTLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_START_CH** `[1:1]`
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh1::OUTLINK_START_CH_MASK
            } else {
                self.bits & !OutLinkCh1::OUTLINK_START_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_STOP_CH** `[0:0]`
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh1::OUTLINK_STOP_CH_MASK
            } else {
                self.bits & !OutLinkCh1::OUTLINK_STOP_CH_MASK
            },
        }
    }
}

impl Default for OutLinkCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutLinkCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkCh1Val {
        OutLinkCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkCh1Val) -> OutLinkCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_STATE_CH1 Register
///
/// Transmit status of Tx channel 0
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **OUT_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **OUT_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current outlink descriptor's address.
#[repr(transparent)]
pub struct OutStateCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutStateCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutStateCh1 {
    // ======== Field Constants ========

    /// **OUT_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const OUT_STATE_CH_OFFSET: u32 = 20;
    pub const OUT_STATE_CH_MASK: u32 = 0x00700000;

    /// **OUT_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const OUT_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const OUT_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **OUTLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current outlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const OUTLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **OUT_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_STATE_CH_MASK) >> Self::OUT_STATE_CH_OFFSET
    }

    /// **OUT_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_DSCR_STATE_CH_MASK) >> Self::OUT_DSCR_STATE_CH_OFFSET
    }

    /// **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_ADDR_CH_MASK)
            >> Self::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// OUT_STATE_CH1 value type for batch read/modify/write operations
///
/// Use `outstatech1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outstatech1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutStateCh1Val {
    bits: u32,
}

impl OutStateCh1Val {
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

    /// Read **OUT_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_state_ch(&self) -> u32 {
        (self.bits & OutStateCh1::OUT_STATE_CH_MASK) >> OutStateCh1::OUT_STATE_CH_OFFSET
    }

    /// Read **OUT_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_dscr_state_ch(&self) -> u32 {
        (self.bits & OutStateCh1::OUT_DSCR_STATE_CH_MASK) >> OutStateCh1::OUT_DSCR_STATE_CH_OFFSET
    }

    /// Read **OUTLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & OutStateCh1::OUTLINK_DSCR_ADDR_CH_MASK)
            >> OutStateCh1::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for OutStateCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutStateCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutStateCh1Val {
        OutStateCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_DES_ADDR_CH1 Register
///
/// Outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct OutEofDesAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofDesAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofDesAddrCh1 {
    // ======== Field Constants ========

    /// **OUT_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn out_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_DES_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `outeofdesaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofdesaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofDesAddrCh1Val {
    bits: u32,
}

impl OutEofDesAddrCh1Val {
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

    /// Read **OUT_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofDesAddrCh1::OUT_EOF_DES_ADDR_CH_MASK)
            >> OutEofDesAddrCh1::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofDesAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofDesAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofDesAddrCh1Val {
        OutEofDesAddrCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH1 Register
///
/// The last outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor before the last outlink descriptor.
#[repr(transparent)]
pub struct OutEofBfrDesAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofBfrDesAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofBfrDesAddrCh1 {
    // ======== Field Constants ========

    /// **OUT_EOF_BFR_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_BFR_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_BFR_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `outeofbfrdesaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofbfrdesaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofBfrDesAddrCh1Val {
    bits: u32,
}

impl OutEofBfrDesAddrCh1Val {
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

    /// Read **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofBfrDesAddrCh1::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> OutEofBfrDesAddrCh1::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofBfrDesAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofBfrDesAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofBfrDesAddrCh1Val {
        OutEofBfrDesAddrCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_CH1 Register
///
/// Current inlink descriptor address of Tx channel 0
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **OUTLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current outlink descriptor y.
#[repr(transparent)]
pub struct OutDscrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrCh1 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_CH**
    ///
    /// The address of the current outlink descriptor y.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub fn outlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_CH_MASK) >> Self::OUTLINK_DSCR_CH_OFFSET
    }
}

/// OUT_DSCR_CH1 value type for batch read/modify/write operations
///
/// Use `outdscrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrCh1Val {
    bits: u32,
}

impl OutDscrCh1Val {
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

    /// Read **OUTLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub const fn outlink_dscr_ch(&self) -> u32 {
        (self.bits & OutDscrCh1::OUTLINK_DSCR_CH_MASK) >> OutDscrCh1::OUTLINK_DSCR_CH_OFFSET
    }
}

impl Default for OutDscrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrCh1Val {
        OutDscrCh1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF0_CH1 Register
///
/// The last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last outlink descriptor y-1.
#[repr(transparent)]
pub struct OutDscrBf0Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf0Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf0Ch1 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF0_CH**
    ///
    /// The address of the last outlink descriptor y-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF0_CH_MASK)
            >> Self::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

/// OUT_DSCR_BF0_CH1 value type for batch read/modify/write operations
///
/// Use `outdscrbf0ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf0ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf0Ch1Val {
    bits: u32,
}

impl OutDscrBf0Ch1Val {
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

    /// Read **OUTLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub const fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & OutDscrBf0Ch1::OUTLINK_DSCR_BF0_CH_MASK)
            >> OutDscrBf0Ch1::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for OutDscrBf0Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf0Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf0Ch1Val {
        OutDscrBf0Ch1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF1_CH1 Register
///
/// The second-to-last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x01b8
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct OutDscrBf1Ch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf1Ch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf1Ch1 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF1_CH_MASK)
            >> Self::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

/// OUT_DSCR_BF1_CH1 value type for batch read/modify/write operations
///
/// Use `outdscrbf1ch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf1ch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf1Ch1Val {
    bits: u32,
}

impl OutDscrBf1Ch1Val {
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

    /// Read **OUTLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & OutDscrBf1Ch1::OUTLINK_DSCR_BF1_CH_MASK)
            >> OutDscrBf1Ch1::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for OutDscrBf1Ch1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf1Ch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf1Ch1Val {
        OutDscrBf1Ch1Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PRI_CH1 Register
///
/// Priority register of Tx channel 0.
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **TX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Tx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct OutPriCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPriCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPriCh1 {
    // ======== Field Constants ========

    /// **TX_PRI_CH**
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_PRI_CH_OFFSET: u32 = 0;
    pub const TX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn tx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PRI_CH_MASK) >> Self::TX_PRI_CH_OFFSET
    }

    /// Write **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_tx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PRI_CH_MASK) | ((value << Self::TX_PRI_CH_OFFSET) & Self::TX_PRI_CH_MASK)
        });
    }
}

/// OUT_PRI_CH1 value type for batch read/modify/write operations
///
/// Use `outprich1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outprich1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPriCh1Val {
    bits: u32,
}

impl OutPriCh1Val {
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

    /// Read **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn tx_pri_ch(&self) -> u32 {
        (self.bits & OutPriCh1::TX_PRI_CH_MASK) >> OutPriCh1::TX_PRI_CH_OFFSET
    }

    /// Set **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_tx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPriCh1::TX_PRI_CH_MASK)
                | ((value << OutPriCh1::TX_PRI_CH_OFFSET) & OutPriCh1::TX_PRI_CH_MASK),
        }
    }
}

impl Default for OutPriCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPriCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPriCh1Val {
        OutPriCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPriCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPriCh1Val) -> OutPriCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_PERI_SEL_CH1 Register
///
/// Peripheral selection of Tx channel 0
///
/// Address offset: 0x01c0
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_OUT_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct OutPeriSelCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPeriSelCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPeriSelCh1 {
    // ======== Field Constants ========

    /// **PERI_OUT_SEL_CH**
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_OUT_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_OUT_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_out_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_OUT_SEL_CH_MASK) >> Self::PERI_OUT_SEL_CH_OFFSET
    }

    /// Write **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_out_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_OUT_SEL_CH_MASK)
                | ((value << Self::PERI_OUT_SEL_CH_OFFSET) & Self::PERI_OUT_SEL_CH_MASK)
        });
    }
}

/// OUT_PERI_SEL_CH1 value type for batch read/modify/write operations
///
/// Use `outperiselch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outperiselch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPeriSelCh1Val {
    bits: u32,
}

impl OutPeriSelCh1Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_out_sel_ch(&self) -> u32 {
        (self.bits & OutPeriSelCh1::PERI_OUT_SEL_CH_MASK) >> OutPeriSelCh1::PERI_OUT_SEL_CH_OFFSET
    }

    /// Set **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_out_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPeriSelCh1::PERI_OUT_SEL_CH_MASK)
                | ((value << OutPeriSelCh1::PERI_OUT_SEL_CH_OFFSET)
                    & OutPeriSelCh1::PERI_OUT_SEL_CH_MASK),
        }
    }
}

impl Default for OutPeriSelCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutPeriSelCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPeriSelCh1Val {
        OutPeriSelCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPeriSelCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPeriSelCh1Val) -> OutPeriSelCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CONF0_CH2 Register
///
/// Configure 0 register of Rx channel 0
///
/// Address offset: 0x01f0
///
/// ## Fields
///
/// - **IN_ETM_EN_CH** `[5]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
///
/// - **MEM_TRANS_EN_CH** `[4]` (RW)
///
///   Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
///
/// - **IN_DATA_BURST_EN_CH** `[3]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
///
/// - **INDSCR_BURST_EN_CH** `[2]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
///
/// - **IN_LOOP_TEST_CH** `[1]` (RW)
///
///   reserved
///
/// - **IN_RST_CH** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
#[repr(transparent)]
pub struct InConf0Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf0Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf0Ch2 {
    // ======== Field Constants ========

    /// **IN_ETM_EN_CH**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IN_ETM_EN_CH_OFFSET: u32 = 5;
    pub const IN_ETM_EN_CH_MASK: u32 = 0x00000020;
    pub const IN_ETM_EN_CH: u32 = 1 << 5;

    /// **MEM_TRANS_EN_CH**
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MEM_TRANS_EN_CH_OFFSET: u32 = 4;
    pub const MEM_TRANS_EN_CH_MASK: u32 = 0x00000010;
    pub const MEM_TRANS_EN_CH: u32 = 1 << 4;

    /// **IN_DATA_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IN_DATA_BURST_EN_CH_OFFSET: u32 = 3;
    pub const IN_DATA_BURST_EN_CH_MASK: u32 = 0x00000008;
    pub const IN_DATA_BURST_EN_CH: u32 = 1 << 3;

    /// **INDSCR_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INDSCR_BURST_EN_CH_OFFSET: u32 = 2;
    pub const INDSCR_BURST_EN_CH_MASK: u32 = 0x00000004;
    pub const INDSCR_BURST_EN_CH: u32 = 1 << 2;

    /// **IN_LOOP_TEST_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IN_LOOP_TEST_CH_OFFSET: u32 = 1;
    pub const IN_LOOP_TEST_CH_MASK: u32 = 0x00000002;
    pub const IN_LOOP_TEST_CH: u32 = 1 << 1;

    /// **IN_RST_CH**
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_RST_CH_OFFSET: u32 = 0;
    pub const IN_RST_CH_MASK: u32 = 0x00000001;
    pub const IN_RST_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_ETM_EN_CH** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_ETM_EN_CH_MASK) != 0
    }

    /// Set IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_ETM_EN_CH_MASK);
    }

    /// Clear IN_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_etm_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_ETM_EN_CH_MASK);
    }

    /// **MEM_TRANS_EN_CH** `[4]` (RW)
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Set MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::MEM_TRANS_EN_CH_MASK);
    }

    /// Clear MEM_TRANS_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_trans_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_TRANS_EN_CH_MASK);
    }

    /// **IN_DATA_BURST_EN_CH** `[3]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub fn in_data_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Set IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// Clear IN_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_DATA_BURST_EN_CH_MASK);
    }

    /// **INDSCR_BURST_EN_CH** `[2]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Set INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// Clear INDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_indscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INDSCR_BURST_EN_CH_MASK);
    }

    /// **IN_LOOP_TEST_CH** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Set IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_LOOP_TEST_CH_MASK);
    }

    /// Clear IN_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_loop_test_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_LOOP_TEST_CH_MASK);
    }

    /// **IN_RST_CH** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub fn in_rst_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_RST_CH_MASK) != 0
    }

    /// Set IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_rst_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_RST_CH_MASK);
    }

    /// Clear IN_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_rst_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_RST_CH_MASK);
    }
}

/// IN_CONF0_CH2 value type for batch read/modify/write operations
///
/// Use `inconf0ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf0ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf0Ch2Val {
    bits: u32,
}

impl InConf0Ch2Val {
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

    /// Read **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn in_etm_en_ch(&self) -> bool {
        (self.bits & InConf0Ch2::IN_ETM_EN_CH_MASK) != 0
    }

    /// Read **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn mem_trans_en_ch(&self) -> bool {
        (self.bits & InConf0Ch2::MEM_TRANS_EN_CH_MASK) != 0
    }

    /// Read **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn in_data_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch2::IN_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Read **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn indscr_burst_en_ch(&self) -> bool {
        (self.bits & InConf0Ch2::INDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Read **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_loop_test_ch(&self) -> bool {
        (self.bits & InConf0Ch2::IN_LOOP_TEST_CH_MASK) != 0
    }

    /// Read **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn in_rst_ch(&self) -> bool {
        (self.bits & InConf0Ch2::IN_RST_CH_MASK) != 0
    }

    /// Set **IN_ETM_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task.
    #[inline(always)]
    pub const fn with_in_etm_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::IN_ETM_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch2::IN_ETM_EN_CH_MASK
            },
        }
    }

    /// Set **MEM_TRANS_EN_CH** `[4:4]`
    ///
    /// Set this bit 1 to enable automatic transmitting data from memory to memory via AHB_DMA.
    #[inline(always)]
    pub const fn with_mem_trans_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::MEM_TRANS_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch2::MEM_TRANS_EN_CH_MASK
            },
        }
    }

    /// Set **IN_DATA_BURST_EN_CH** `[3:3]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_in_data_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::IN_DATA_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch2::IN_DATA_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **INDSCR_BURST_EN_CH** `[2:2]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_indscr_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::INDSCR_BURST_EN_CH_MASK
            } else {
                self.bits & !InConf0Ch2::INDSCR_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **IN_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_in_loop_test_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::IN_LOOP_TEST_CH_MASK
            } else {
                self.bits & !InConf0Ch2::IN_LOOP_TEST_CH_MASK
            },
        }
    }

    /// Set **IN_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 0 Rx FSM and Rx FIFO pointer.
    #[inline(always)]
    pub const fn with_in_rst_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf0Ch2::IN_RST_CH_MASK
            } else {
                self.bits & !InConf0Ch2::IN_RST_CH_MASK
            },
        }
    }
}

impl Default for InConf0Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf0Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf0Ch2Val {
        InConf0Ch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf0Ch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf0Ch2Val) -> InConf0Ch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CONF1_CH2 Register
///
/// Configure 1 register of Rx channel 0
///
/// Address offset: 0x01f4
///
/// ## Fields
///
/// - **IN_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct InConf1Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InConf1Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InConf1Ch2 {
    // ======== Field Constants ========

    /// **IN_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IN_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const IN_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const IN_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **IN_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_check_owner_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CHECK_OWNER_CH_MASK);
    }

    /// Clear IN_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::IN_CHECK_OWNER_CH_MASK);
    }
}

/// IN_CONF1_CH2 value type for batch read/modify/write operations
///
/// Use `inconf1ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inconf1ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InConf1Ch2Val {
    bits: u32,
}

impl InConf1Ch2Val {
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

    /// Read **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn in_check_owner_ch(&self) -> bool {
        (self.bits & InConf1Ch2::IN_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **IN_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_in_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InConf1Ch2::IN_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !InConf1Ch2::IN_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for InConf1Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InConf1Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InConf1Ch2Val {
        InConf1Ch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InConf1Ch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InConf1Ch2Val) -> InConf1Ch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INFIFO_STATUS_CH2 Register
///
/// Receive FIFO status of Rx channel 0
///
/// Address offset: 0x01f8
/// Reset value: 0x07800003
///
/// ## Fields
///
/// - **IN_BUF_HUNGRY_CH** `[27]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **INFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
///
/// - **INFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Rx FIFO empty signal for Rx channel 0.
///
/// - **INFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Rx FIFO full signal for Rx channel 0.
#[repr(transparent)]
pub struct InfifoStatusCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfifoStatusCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfifoStatusCh2 {
    // ======== Field Constants ========

    /// **IN_BUF_HUNGRY_CH**
    ///
    /// reserved
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const IN_BUF_HUNGRY_CH_OFFSET: u32 = 27;
    pub const IN_BUF_HUNGRY_CH_MASK: u32 = 0x08000000;
    pub const IN_BUF_HUNGRY_CH: u32 = 1 << 27;

    /// **IN_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const IN_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const IN_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const IN_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **IN_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const IN_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const IN_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const IN_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **IN_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const IN_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const IN_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const IN_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **IN_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const IN_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const IN_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const IN_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **INFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const INFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const INFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **INFIFO_EMPTY_CH**
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const INFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const INFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **INFIFO_FULL_CH**
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const INFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const INFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_BUF_HUNGRY_CH** `[27]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **IN_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **INFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_CNT_CH_MASK) >> Self::INFIFO_CNT_CH_OFFSET
    }

    /// **INFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// **INFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::INFIFO_FULL_CH_MASK) != 0
    }
}

/// INFIFO_STATUS_CH2 value type for batch read/modify/write operations
///
/// Use `infifostatusch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `infifostatusch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InfifoStatusCh2Val {
    bits: u32,
}

impl InfifoStatusCh2Val {
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
        Self { bits: 0x07800003 }
    }

    /// Read **IN_BUF_HUNGRY_CH** `[27:27]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_buf_hungry_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::IN_BUF_HUNGRY_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_4b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::IN_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_3b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::IN_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_2b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::IN_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **IN_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_remain_under_1b_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::IN_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **INFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_cnt_ch(&self) -> u32 {
        (self.bits & InfifoStatusCh2::INFIFO_CNT_CH_MASK) >> InfifoStatusCh2::INFIFO_CNT_CH_OFFSET
    }

    /// Read **INFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_empty_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::INFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **INFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub const fn infifo_full_ch(&self) -> bool {
        (self.bits & InfifoStatusCh2::INFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for InfifoStatusCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InfifoStatusCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InfifoStatusCh2Val {
        InfifoStatusCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_POP_CH2 Register
///
/// Pop control register of Rx channel 0
///
/// Address offset: 0x01fc
/// Reset value: 0x00000800
///
/// ## Fields
///
/// - **INFIFO_POP_CH** `[12]` (WO)
///
///   Set this bit to pop data from AHB_DMA FIFO.
///
/// - **INFIFO_RDATA_CH** `[11:0]` (RO)
///
///   This register stores the data popping from AHB_DMA FIFO.
#[repr(transparent)]
pub struct InPopCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPopCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPopCh2 {
    // ======== Field Constants ========

    /// **INFIFO_POP_CH**
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INFIFO_POP_CH_OFFSET: u32 = 12;
    pub const INFIFO_POP_CH_MASK: u32 = 0x00001000;
    pub const INFIFO_POP_CH: u32 = 1 << 12;

    /// **INFIFO_RDATA_CH**
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const INFIFO_RDATA_CH_OFFSET: u32 = 0;
    pub const INFIFO_RDATA_CH_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// Trigger **INFIFO_POP_CH** (WO)
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_infifo_pop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INFIFO_POP_CH_MASK);
    }

    /// **INFIFO_RDATA_CH** `[11:0]` (RO)
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INFIFO_RDATA_CH_MASK) >> Self::INFIFO_RDATA_CH_OFFSET
    }
}

/// IN_POP_CH2 value type for batch read/modify/write operations
///
/// Use `inpopch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inpopch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPopCh2Val {
    bits: u32,
}

impl InPopCh2Val {
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
        Self { bits: 0x00000800 }
    }

    /// Read **INFIFO_RDATA_CH** `[11:0]`
    ///
    /// This register stores the data popping from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn infifo_rdata_ch(&self) -> u32 {
        (self.bits & InPopCh2::INFIFO_RDATA_CH_MASK) >> InPopCh2::INFIFO_RDATA_CH_OFFSET
    }

    /// Set **INFIFO_POP_CH** `[12:12]`
    ///
    /// Set this bit to pop data from AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_infifo_pop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InPopCh2::INFIFO_POP_CH_MASK
            } else {
                self.bits & !InPopCh2::INFIFO_POP_CH_MASK
            },
        }
    }
}

impl Default for InPopCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPopCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPopCh2Val {
        InPopCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPopCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPopCh2Val) -> InPopCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_CH2 Register
///
/// Link descriptor configure and control register of Rx channel 0
///
/// Address offset: 0x0200
/// Reset value: 0x00000011
///
/// ## Fields
///
/// - **INLINK_PARK_CH** `[4]` (RO)
///
///   1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
///
/// - **INLINK_RESTART_CH** `[3]` (WO)
///
///   Set this bit to mount a new inlink descriptor.
///
/// - **INLINK_START_CH** `[2]` (WO)
///
///   Set this bit to start dealing with the inlink descriptors.
///
/// - **INLINK_STOP_CH** `[1]` (WO)
///
///   Set this bit to stop dealing with the inlink descriptors.
///
/// - **INLINK_AUTO_RET_CH** `[0]` (RW)
///
///   Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
#[repr(transparent)]
pub struct InLinkCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkCh2 {
    // ======== Field Constants ========

    /// **INLINK_PARK_CH**
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const INLINK_PARK_CH_OFFSET: u32 = 4;
    pub const INLINK_PARK_CH_MASK: u32 = 0x00000010;
    pub const INLINK_PARK_CH: u32 = 1 << 4;

    /// **INLINK_RESTART_CH**
    ///
    /// Set this bit to mount a new inlink descriptor.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INLINK_RESTART_CH_OFFSET: u32 = 3;
    pub const INLINK_RESTART_CH_MASK: u32 = 0x00000008;
    pub const INLINK_RESTART_CH: u32 = 1 << 3;

    /// **INLINK_START_CH**
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INLINK_START_CH_OFFSET: u32 = 2;
    pub const INLINK_START_CH_MASK: u32 = 0x00000004;
    pub const INLINK_START_CH: u32 = 1 << 2;

    /// **INLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INLINK_STOP_CH_OFFSET: u32 = 1;
    pub const INLINK_STOP_CH_MASK: u32 = 0x00000002;
    pub const INLINK_STOP_CH: u32 = 1 << 1;

    /// **INLINK_AUTO_RET_CH**
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INLINK_AUTO_RET_CH_OFFSET: u32 = 0;
    pub const INLINK_AUTO_RET_CH_MASK: u32 = 0x00000001;
    pub const INLINK_AUTO_RET_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INLINK_PARK_CH** `[4]` (RO)
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub fn inlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **INLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub fn trigger_inlink_restart_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_RESTART_CH_MASK);
    }

    /// Trigger **INLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_START_CH_MASK);
    }

    /// Trigger **INLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub fn trigger_inlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::INLINK_STOP_CH_MASK);
    }

    /// **INLINK_AUTO_RET_CH** `[0]` (RW)
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&self) -> bool {
        (self.value.read_bits() & Self::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::INLINK_AUTO_RET_CH_MASK);
    }

    /// Clear INLINK_AUTO_RET_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inlink_auto_ret_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::INLINK_AUTO_RET_CH_MASK);
    }
}

/// IN_LINK_CH2 value type for batch read/modify/write operations
///
/// Use `inlinkch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkCh2Val {
    bits: u32,
}

impl InLinkCh2Val {
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
        Self { bits: 0x00000011 }
    }

    /// Read **INLINK_PARK_CH** `[4:4]`
    ///
    /// 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn inlink_park_ch(&self) -> bool {
        (self.bits & InLinkCh2::INLINK_PARK_CH_MASK) != 0
    }

    /// Read **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn inlink_auto_ret_ch(&self) -> bool {
        (self.bits & InLinkCh2::INLINK_AUTO_RET_CH_MASK) != 0
    }

    /// Set **INLINK_RESTART_CH** `[3:3]`
    ///
    /// Set this bit to mount a new inlink descriptor.
    #[inline(always)]
    pub const fn with_inlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh2::INLINK_RESTART_CH_MASK
            } else {
                self.bits & !InLinkCh2::INLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **INLINK_START_CH** `[2:2]`
    ///
    /// Set this bit to start dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh2::INLINK_START_CH_MASK
            } else {
                self.bits & !InLinkCh2::INLINK_START_CH_MASK
            },
        }
    }

    /// Set **INLINK_STOP_CH** `[1:1]`
    ///
    /// Set this bit to stop dealing with the inlink descriptors.
    #[inline(always)]
    pub const fn with_inlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh2::INLINK_STOP_CH_MASK
            } else {
                self.bits & !InLinkCh2::INLINK_STOP_CH_MASK
            },
        }
    }

    /// Set **INLINK_AUTO_RET_CH** `[0:0]`
    ///
    /// Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data.
    #[inline(always)]
    pub const fn with_inlink_auto_ret_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InLinkCh2::INLINK_AUTO_RET_CH_MASK
            } else {
                self.bits & !InLinkCh2::INLINK_AUTO_RET_CH_MASK
            },
        }
    }
}

impl Default for InLinkCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InLinkCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkCh2Val {
        InLinkCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkCh2Val) -> InLinkCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_STATE_CH2 Register
///
/// Receive status of Rx channel 0
///
/// Address offset: 0x0204
///
/// ## Fields
///
/// - **IN_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **IN_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current inlink descriptor's address.
#[repr(transparent)]
pub struct InStateCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InStateCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InStateCh2 {
    // ======== Field Constants ========

    /// **IN_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const IN_STATE_CH_OFFSET: u32 = 20;
    pub const IN_STATE_CH_MASK: u32 = 0x00700000;

    /// **IN_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const IN_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const IN_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **INLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current inlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const INLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **IN_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_STATE_CH_MASK) >> Self::IN_STATE_CH_OFFSET
    }

    /// **IN_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn in_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_DSCR_STATE_CH_MASK) >> Self::IN_DSCR_STATE_CH_OFFSET
    }

    /// **INLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_ADDR_CH_MASK)
            >> Self::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// IN_STATE_CH2 value type for batch read/modify/write operations
///
/// Use `instatech2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instatech2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InStateCh2Val {
    bits: u32,
}

impl InStateCh2Val {
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

    /// Read **IN_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_state_ch(&self) -> u32 {
        (self.bits & InStateCh2::IN_STATE_CH_MASK) >> InStateCh2::IN_STATE_CH_OFFSET
    }

    /// Read **IN_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn in_dscr_state_ch(&self) -> u32 {
        (self.bits & InStateCh2::IN_DSCR_STATE_CH_MASK) >> InStateCh2::IN_DSCR_STATE_CH_OFFSET
    }

    /// Read **INLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & InStateCh2::INLINK_DSCR_ADDR_CH_MASK) >> InStateCh2::INLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for InStateCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InStateCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InStateCh2Val {
        InStateCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_SUC_EOF_DES_ADDR_CH2 Register
///
/// Inlink descriptor address when EOF occurs of Rx channel 0
///
/// Address offset: 0x0208
///
/// ## Fields
///
/// - **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct InSucEofDesAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InSucEofDesAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InSucEofDesAddrCh2 {
    // ======== Field Constants ========

    /// **IN_SUC_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_SUC_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_SUC_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_SUC_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_SUC_EOF_DES_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `insuceofdesaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `insuceofdesaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InSucEofDesAddrCh2Val {
    bits: u32,
}

impl InSucEofDesAddrCh2Val {
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

    /// Read **IN_SUC_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InSucEofDesAddrCh2::IN_SUC_EOF_DES_ADDR_CH_MASK)
            >> InSucEofDesAddrCh2::IN_SUC_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InSucEofDesAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InSucEofDesAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InSucEofDesAddrCh2Val {
        InSucEofDesAddrCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_ERR_EOF_DES_ADDR_CH2 Register
///
/// Inlink descriptor address when errors occur of Rx channel 0
///
/// Address offset: 0x020c
///
/// ## Fields
///
/// - **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
#[repr(transparent)]
pub struct InErrEofDesAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InErrEofDesAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InErrEofDesAddrCh2 {
    // ======== Field Constants ========

    /// **IN_ERR_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_ERR_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const IN_ERR_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_ERR_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> Self::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

/// IN_ERR_EOF_DES_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `inerreofdesaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inerreofdesaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InErrEofDesAddrCh2Val {
    bits: u32,
}

impl InErrEofDesAddrCh2Val {
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

    /// Read **IN_ERR_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0.
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self) -> u32 {
        (self.bits & InErrEofDesAddrCh2::IN_ERR_EOF_DES_ADDR_CH_MASK)
            >> InErrEofDesAddrCh2::IN_ERR_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for InErrEofDesAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InErrEofDesAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InErrEofDesAddrCh2Val {
        InErrEofDesAddrCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_CH2 Register
///
/// Current inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0210
///
/// ## Fields
///
/// - **INLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current inlink descriptor x.
#[repr(transparent)]
pub struct InDscrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrCh2 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_CH**
    ///
    /// The address of the current inlink descriptor x.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub fn inlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_CH_MASK) >> Self::INLINK_DSCR_CH_OFFSET
    }
}

/// IN_DSCR_CH2 value type for batch read/modify/write operations
///
/// Use `indscrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrCh2Val {
    bits: u32,
}

impl InDscrCh2Val {
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

    /// Read **INLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current inlink descriptor x.
    #[inline(always)]
    pub const fn inlink_dscr_ch(&self) -> u32 {
        (self.bits & InDscrCh2::INLINK_DSCR_CH_MASK) >> InDscrCh2::INLINK_DSCR_CH_OFFSET
    }
}

impl Default for InDscrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrCh2Val {
        InDscrCh2Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF0_CH2 Register
///
/// The last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0214
///
/// ## Fields
///
/// - **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last inlink descriptor x-1.
#[repr(transparent)]
pub struct InDscrBf0Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf0Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf0Ch2 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF0_CH**
    ///
    /// The address of the last inlink descriptor x-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF0_CH_MASK) >> Self::INLINK_DSCR_BF0_CH_OFFSET
    }
}

/// IN_DSCR_BF0_CH2 value type for batch read/modify/write operations
///
/// Use `indscrbf0ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf0ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf0Ch2Val {
    bits: u32,
}

impl InDscrBf0Ch2Val {
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

    /// Read **INLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last inlink descriptor x-1.
    #[inline(always)]
    pub const fn inlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & InDscrBf0Ch2::INLINK_DSCR_BF0_CH_MASK)
            >> InDscrBf0Ch2::INLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for InDscrBf0Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf0Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf0Ch2Val {
        InDscrBf0Ch2Val::from_bits(self.value.read_bits())
    }
}

/// IN_DSCR_BF1_CH2 Register
///
/// The second-to-last inlink descriptor address of Rx channel 0
///
/// Address offset: 0x0218
///
/// ## Fields
///
/// - **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct InDscrBf1Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InDscrBf1Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InDscrBf1Ch2 {
    // ======== Field Constants ========

    /// **INLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const INLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_DSCR_BF1_CH_MASK) >> Self::INLINK_DSCR_BF1_CH_OFFSET
    }
}

/// IN_DSCR_BF1_CH2 value type for batch read/modify/write operations
///
/// Use `indscrbf1ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indscrbf1ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InDscrBf1Ch2Val {
    bits: u32,
}

impl InDscrBf1Ch2Val {
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

    /// Read **INLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn inlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & InDscrBf1Ch2::INLINK_DSCR_BF1_CH_MASK)
            >> InDscrBf1Ch2::INLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for InDscrBf1Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InDscrBf1Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InDscrBf1Ch2Val {
        InDscrBf1Ch2Val::from_bits(self.value.read_bits())
    }
}

/// IN_PRI_CH2 Register
///
/// Priority register of Rx channel 0
///
/// Address offset: 0x021c
///
/// ## Fields
///
/// - **RX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Rx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct InPriCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPriCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPriCh2 {
    // ======== Field Constants ========

    /// **RX_PRI_CH**
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_PRI_CH_OFFSET: u32 = 0;
    pub const RX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn rx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PRI_CH_MASK) >> Self::RX_PRI_CH_OFFSET
    }

    /// Write **RX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_rx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PRI_CH_MASK) | ((value << Self::RX_PRI_CH_OFFSET) & Self::RX_PRI_CH_MASK)
        });
    }
}

/// IN_PRI_CH2 value type for batch read/modify/write operations
///
/// Use `inprich2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inprich2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPriCh2Val {
    bits: u32,
}

impl InPriCh2Val {
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

    /// Read **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn rx_pri_ch(&self) -> u32 {
        (self.bits & InPriCh2::RX_PRI_CH_MASK) >> InPriCh2::RX_PRI_CH_OFFSET
    }

    /// Set **RX_PRI_CH** `[3:0]`
    ///
    /// The priority of Rx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_rx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPriCh2::RX_PRI_CH_MASK)
                | ((value << InPriCh2::RX_PRI_CH_OFFSET) & InPriCh2::RX_PRI_CH_MASK),
        }
    }
}

impl Default for InPriCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InPriCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPriCh2Val {
        InPriCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPriCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPriCh2Val) -> InPriCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_PERI_SEL_CH2 Register
///
/// Peripheral selection of Rx channel 0
///
/// Address offset: 0x0220
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_IN_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct InPeriSelCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InPeriSelCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InPeriSelCh2 {
    // ======== Field Constants ========

    /// **PERI_IN_SEL_CH**
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_IN_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_IN_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_in_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_IN_SEL_CH_MASK) >> Self::PERI_IN_SEL_CH_OFFSET
    }

    /// Write **PERI_IN_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_in_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_IN_SEL_CH_MASK)
                | ((value << Self::PERI_IN_SEL_CH_OFFSET) & Self::PERI_IN_SEL_CH_MASK)
        });
    }
}

/// IN_PERI_SEL_CH2 value type for batch read/modify/write operations
///
/// Use `inperiselch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inperiselch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InPeriSelCh2Val {
    bits: u32,
}

impl InPeriSelCh2Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_in_sel_ch(&self) -> u32 {
        (self.bits & InPeriSelCh2::PERI_IN_SEL_CH_MASK) >> InPeriSelCh2::PERI_IN_SEL_CH_OFFSET
    }

    /// Set **PERI_IN_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_in_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InPeriSelCh2::PERI_IN_SEL_CH_MASK)
                | ((value << InPeriSelCh2::PERI_IN_SEL_CH_OFFSET)
                    & InPeriSelCh2::PERI_IN_SEL_CH_MASK),
        }
    }
}

impl Default for InPeriSelCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InPeriSelCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InPeriSelCh2Val {
        InPeriSelCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InPeriSelCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InPeriSelCh2Val) -> InPeriSelCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF0_CH2 Register
///
/// Configure 0 register of Tx channel 1
///
/// Address offset: 0x0250
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUT_ETM_EN_CH** `[6]` (RW)
///
///   Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
///
/// - **OUT_DATA_BURST_EN_CH** `[5]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
///
/// - **OUTDSCR_BURST_EN_CH** `[4]` (RW)
///
///   Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
///
/// - **OUT_EOF_MODE_CH** `[3]` (RW)
///
///   EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
///
/// - **OUT_AUTO_WRBACK_CH** `[2]` (RW)
///
///   Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
///
/// - **OUT_LOOP_TEST_CH** `[1]` (RW)
///
///   reserved
///
/// - **OUT_RST_CH** `[0]` (RW)
///
///   This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
#[repr(transparent)]
pub struct OutConf0Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf0Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf0Ch2 {
    // ======== Field Constants ========

    /// **OUT_ETM_EN_CH**
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUT_ETM_EN_CH_OFFSET: u32 = 6;
    pub const OUT_ETM_EN_CH_MASK: u32 = 0x00000040;
    pub const OUT_ETM_EN_CH: u32 = 1 << 6;

    /// **OUT_DATA_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OUT_DATA_BURST_EN_CH_OFFSET: u32 = 5;
    pub const OUT_DATA_BURST_EN_CH_MASK: u32 = 0x00000020;
    pub const OUT_DATA_BURST_EN_CH: u32 = 1 << 5;

    /// **OUTDSCR_BURST_EN_CH**
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OUTDSCR_BURST_EN_CH_OFFSET: u32 = 4;
    pub const OUTDSCR_BURST_EN_CH_MASK: u32 = 0x00000010;
    pub const OUTDSCR_BURST_EN_CH: u32 = 1 << 4;

    /// **OUT_EOF_MODE_CH**
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUT_EOF_MODE_CH_OFFSET: u32 = 3;
    pub const OUT_EOF_MODE_CH_MASK: u32 = 0x00000008;
    pub const OUT_EOF_MODE_CH: u32 = 1 << 3;

    /// **OUT_AUTO_WRBACK_CH**
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUT_AUTO_WRBACK_CH_OFFSET: u32 = 2;
    pub const OUT_AUTO_WRBACK_CH_MASK: u32 = 0x00000004;
    pub const OUT_AUTO_WRBACK_CH: u32 = 1 << 2;

    /// **OUT_LOOP_TEST_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUT_LOOP_TEST_CH_OFFSET: u32 = 1;
    pub const OUT_LOOP_TEST_CH_MASK: u32 = 0x00000002;
    pub const OUT_LOOP_TEST_CH: u32 = 1 << 1;

    /// **OUT_RST_CH**
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_RST_CH_OFFSET: u32 = 0;
    pub const OUT_RST_CH_MASK: u32 = 0x00000001;
    pub const OUT_RST_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_ETM_EN_CH** `[6]` (RW)
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub fn out_etm_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_ETM_EN_CH_MASK) != 0
    }

    /// Set OUT_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_etm_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_ETM_EN_CH_MASK);
    }

    /// Clear OUT_ETM_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_etm_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_ETM_EN_CH_MASK);
    }

    /// **OUT_DATA_BURST_EN_CH** `[5]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub fn out_data_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Set OUT_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_DATA_BURST_EN_CH_MASK);
    }

    /// Clear OUT_DATA_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_data_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_DATA_BURST_EN_CH_MASK);
    }

    /// **OUTDSCR_BURST_EN_CH** `[4]` (RW)
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Set OUTDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outdscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTDSCR_BURST_EN_CH_MASK);
    }

    /// Clear OUTDSCR_BURST_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outdscr_burst_en_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTDSCR_BURST_EN_CH_MASK);
    }

    /// **OUT_EOF_MODE_CH** `[3]` (RW)
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub fn out_eof_mode_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_MODE_CH_MASK) != 0
    }

    /// Set OUT_EOF_MODE_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_mode_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_MODE_CH_MASK);
    }

    /// Clear OUT_EOF_MODE_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_mode_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_MODE_CH_MASK);
    }

    /// **OUT_AUTO_WRBACK_CH** `[2]` (RW)
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub fn out_auto_wrback_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_AUTO_WRBACK_CH_MASK) != 0
    }

    /// Set OUT_AUTO_WRBACK_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_auto_wrback_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_AUTO_WRBACK_CH_MASK);
    }

    /// Clear OUT_AUTO_WRBACK_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_auto_wrback_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_AUTO_WRBACK_CH_MASK);
    }

    /// **OUT_LOOP_TEST_CH** `[1]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_loop_test_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_LOOP_TEST_CH_MASK) != 0
    }

    /// Set OUT_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_loop_test_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_LOOP_TEST_CH_MASK);
    }

    /// Clear OUT_LOOP_TEST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_loop_test_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_LOOP_TEST_CH_MASK);
    }

    /// **OUT_RST_CH** `[0]` (RW)
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub fn out_rst_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_RST_CH_MASK) != 0
    }

    /// Set OUT_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_rst_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_RST_CH_MASK);
    }

    /// Clear OUT_RST_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_rst_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_RST_CH_MASK);
    }
}

/// OUT_CONF0_CH2 value type for batch read/modify/write operations
///
/// Use `outconf0ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf0ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf0Ch2Val {
    bits: u32,
}

impl OutConf0Ch2Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUT_ETM_EN_CH** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub const fn out_etm_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_ETM_EN_CH_MASK) != 0
    }

    /// Read **OUT_DATA_BURST_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn out_data_burst_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_DATA_BURST_EN_CH_MASK) != 0
    }

    /// Read **OUTDSCR_BURST_EN_CH** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn outdscr_burst_en_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUTDSCR_BURST_EN_CH_MASK) != 0
    }

    /// Read **OUT_EOF_MODE_CH** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn out_eof_mode_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_EOF_MODE_CH_MASK) != 0
    }

    /// Read **OUT_AUTO_WRBACK_CH** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn out_auto_wrback_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_AUTO_WRBACK_CH_MASK) != 0
    }

    /// Read **OUT_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_loop_test_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_LOOP_TEST_CH_MASK) != 0
    }

    /// Read **OUT_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn out_rst_ch(&self) -> bool {
        (self.bits & OutConf0Ch2::OUT_RST_CH_MASK) != 0
    }

    /// Set **OUT_ETM_EN_CH** `[6:6]`
    ///
    /// Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task.
    #[inline(always)]
    pub const fn with_out_etm_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_ETM_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_ETM_EN_CH_MASK
            },
        }
    }

    /// Set **OUT_DATA_BURST_EN_CH** `[5:5]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_out_data_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_DATA_BURST_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_DATA_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **OUTDSCR_BURST_EN_CH** `[4:4]`
    ///
    /// Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM.
    #[inline(always)]
    pub const fn with_outdscr_burst_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUTDSCR_BURST_EN_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUTDSCR_BURST_EN_CH_MASK
            },
        }
    }

    /// Set **OUT_EOF_MODE_CH** `[3:3]`
    ///
    /// EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA
    #[inline(always)]
    pub const fn with_out_eof_mode_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_EOF_MODE_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_EOF_MODE_CH_MASK
            },
        }
    }

    /// Set **OUT_AUTO_WRBACK_CH** `[2:2]`
    ///
    /// Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted.
    #[inline(always)]
    pub const fn with_out_auto_wrback_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_AUTO_WRBACK_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_AUTO_WRBACK_CH_MASK
            },
        }
    }

    /// Set **OUT_LOOP_TEST_CH** `[1:1]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_out_loop_test_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_LOOP_TEST_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_LOOP_TEST_CH_MASK
            },
        }
    }

    /// Set **OUT_RST_CH** `[0:0]`
    ///
    /// This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer.
    #[inline(always)]
    pub const fn with_out_rst_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf0Ch2::OUT_RST_CH_MASK
            } else {
                self.bits & !OutConf0Ch2::OUT_RST_CH_MASK
            },
        }
    }
}

impl Default for OutConf0Ch2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutConf0Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf0Ch2Val {
        OutConf0Ch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf0Ch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf0Ch2Val) -> OutConf0Ch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CONF1_CH2 Register
///
/// Configure 1 register of Tx channel 0
///
/// Address offset: 0x0254
///
/// ## Fields
///
/// - **OUT_CHECK_OWNER_CH** `[12]` (RW)
///
///   Set this bit to enable checking the owner attribute of the link descriptor.
#[repr(transparent)]
pub struct OutConf1Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutConf1Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutConf1Ch2 {
    // ======== Field Constants ========

    /// **OUT_CHECK_OWNER_CH**
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OUT_CHECK_OWNER_CH_OFFSET: u32 = 12;
    pub const OUT_CHECK_OWNER_CH_MASK: u32 = 0x00001000;
    pub const OUT_CHECK_OWNER_CH: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **OUT_CHECK_OWNER_CH** `[12]` (RW)
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUT_CHECK_OWNER_CH_MASK);
    }

    /// Clear OUT_CHECK_OWNER_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_check_owner_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUT_CHECK_OWNER_CH_MASK);
    }
}

/// OUT_CONF1_CH2 value type for batch read/modify/write operations
///
/// Use `outconf1ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outconf1ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutConf1Ch2Val {
    bits: u32,
}

impl OutConf1Ch2Val {
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

    /// Read **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn out_check_owner_ch(&self) -> bool {
        (self.bits & OutConf1Ch2::OUT_CHECK_OWNER_CH_MASK) != 0
    }

    /// Set **OUT_CHECK_OWNER_CH** `[12:12]`
    ///
    /// Set this bit to enable checking the owner attribute of the link descriptor.
    #[inline(always)]
    pub const fn with_out_check_owner_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutConf1Ch2::OUT_CHECK_OWNER_CH_MASK
            } else {
                self.bits & !OutConf1Ch2::OUT_CHECK_OWNER_CH_MASK
            },
        }
    }
}

impl Default for OutConf1Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutConf1Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutConf1Ch2Val {
        OutConf1Ch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutConf1Ch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutConf1Ch2Val) -> OutConf1Ch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUTFIFO_STATUS_CH2 Register
///
/// Transmit FIFO status of Tx channel 0
///
/// Address offset: 0x0258
/// Reset value: 0x07800002
///
/// ## Fields
///
/// - **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
///
///   reserved
///
/// - **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
///
///   reserved
///
/// - **OUTFIFO_CNT_CH** `[7:2]` (RO)
///
///   The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
///
/// - **OUTFIFO_EMPTY_CH** `[1]` (RO)
///
///   L1 Tx FIFO empty signal for Tx channel 0.
///
/// - **OUTFIFO_FULL_CH** `[0]` (RO)
///
///   L1 Tx FIFO full signal for Tx channel 0.
#[repr(transparent)]
pub struct OutfifoStatusCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutfifoStatusCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutfifoStatusCh2 {
    // ======== Field Constants ========

    /// **OUT_REMAIN_UNDER_4B_CH**
    ///
    /// reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const OUT_REMAIN_UNDER_4B_CH_OFFSET: u32 = 26;
    pub const OUT_REMAIN_UNDER_4B_CH_MASK: u32 = 0x04000000;
    pub const OUT_REMAIN_UNDER_4B_CH: u32 = 1 << 26;

    /// **OUT_REMAIN_UNDER_3B_CH**
    ///
    /// reserved
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OUT_REMAIN_UNDER_3B_CH_OFFSET: u32 = 25;
    pub const OUT_REMAIN_UNDER_3B_CH_MASK: u32 = 0x02000000;
    pub const OUT_REMAIN_UNDER_3B_CH: u32 = 1 << 25;

    /// **OUT_REMAIN_UNDER_2B_CH**
    ///
    /// reserved
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OUT_REMAIN_UNDER_2B_CH_OFFSET: u32 = 24;
    pub const OUT_REMAIN_UNDER_2B_CH_MASK: u32 = 0x01000000;
    pub const OUT_REMAIN_UNDER_2B_CH: u32 = 1 << 24;

    /// **OUT_REMAIN_UNDER_1B_CH**
    ///
    /// reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OUT_REMAIN_UNDER_1B_CH_OFFSET: u32 = 23;
    pub const OUT_REMAIN_UNDER_1B_CH_MASK: u32 = 0x00800000;
    pub const OUT_REMAIN_UNDER_1B_CH: u32 = 1 << 23;

    /// **OUTFIFO_CNT_CH**
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const OUTFIFO_CNT_CH_OFFSET: u32 = 2;
    pub const OUTFIFO_CNT_CH_MASK: u32 = 0x000000fc;

    /// **OUTFIFO_EMPTY_CH**
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTFIFO_EMPTY_CH_OFFSET: u32 = 1;
    pub const OUTFIFO_EMPTY_CH_MASK: u32 = 0x00000002;
    pub const OUTFIFO_EMPTY_CH: u32 = 1 << 1;

    /// **OUTFIFO_FULL_CH**
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTFIFO_FULL_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_FULL_CH_MASK: u32 = 0x00000001;
    pub const OUTFIFO_FULL_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_REMAIN_UNDER_4B_CH** `[26]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_3B_CH** `[25]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_2B_CH** `[24]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// **OUT_REMAIN_UNDER_1B_CH** `[23]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// **OUTFIFO_CNT_CH** `[7:2]` (RO)
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_CNT_CH_MASK) >> Self::OUTFIFO_CNT_CH_OFFSET
    }

    /// **OUTFIFO_EMPTY_CH** `[1]` (RO)
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// **OUTFIFO_FULL_CH** `[0]` (RO)
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTFIFO_FULL_CH_MASK) != 0
    }
}

/// OUTFIFO_STATUS_CH2 value type for batch read/modify/write operations
///
/// Use `outfifostatusch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outfifostatusch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutfifoStatusCh2Val {
    bits: u32,
}

impl OutfifoStatusCh2Val {
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
        Self { bits: 0x07800002 }
    }

    /// Read **OUT_REMAIN_UNDER_4B_CH** `[26:26]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_4b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUT_REMAIN_UNDER_4B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_3B_CH** `[25:25]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_3b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUT_REMAIN_UNDER_3B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_2B_CH** `[24:24]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_2b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUT_REMAIN_UNDER_2B_CH_MASK) != 0
    }

    /// Read **OUT_REMAIN_UNDER_1B_CH** `[23:23]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_remain_under_1b_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUT_REMAIN_UNDER_1B_CH_MASK) != 0
    }

    /// Read **OUTFIFO_CNT_CH** `[7:2]`
    ///
    /// The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_cnt_ch(&self) -> u32 {
        (self.bits & OutfifoStatusCh2::OUTFIFO_CNT_CH_MASK)
            >> OutfifoStatusCh2::OUTFIFO_CNT_CH_OFFSET
    }

    /// Read **OUTFIFO_EMPTY_CH** `[1:1]`
    ///
    /// L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_empty_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUTFIFO_EMPTY_CH_MASK) != 0
    }

    /// Read **OUTFIFO_FULL_CH** `[0:0]`
    ///
    /// L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub const fn outfifo_full_ch(&self) -> bool {
        (self.bits & OutfifoStatusCh2::OUTFIFO_FULL_CH_MASK) != 0
    }
}

impl Default for OutfifoStatusCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutfifoStatusCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutfifoStatusCh2Val {
        OutfifoStatusCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PUSH_CH2 Register
///
/// Push control register of Rx channel 0
///
/// Address offset: 0x025c
///
/// ## Fields
///
/// - **OUTFIFO_PUSH_CH** `[9]` (WO)
///
///   Set this bit to push data into AHB_DMA FIFO.
///
/// - **OUTFIFO_WDATA_CH** `[8:0]` (RW)
///
///   This register stores the data that need to be pushed into AHB_DMA FIFO.
#[repr(transparent)]
pub struct OutPushCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPushCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPushCh2 {
    // ======== Field Constants ========

    /// **OUTFIFO_PUSH_CH**
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUTFIFO_PUSH_CH_OFFSET: u32 = 9;
    pub const OUTFIFO_PUSH_CH_MASK: u32 = 0x00000200;
    pub const OUTFIFO_PUSH_CH: u32 = 1 << 9;

    /// **OUTFIFO_WDATA_CH**
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const OUTFIFO_WDATA_CH_OFFSET: u32 = 0;
    pub const OUTFIFO_WDATA_CH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// Trigger **OUTFIFO_PUSH_CH** (WO)
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub fn trigger_outfifo_push_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTFIFO_PUSH_CH_MASK);
    }

    /// **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn outfifo_wdata_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTFIFO_WDATA_CH_MASK) >> Self::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Write **OUTFIFO_WDATA_CH** `[8:0]` (RW)
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub fn write_outfifo_wdata_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTFIFO_WDATA_CH_MASK)
                | ((value << Self::OUTFIFO_WDATA_CH_OFFSET) & Self::OUTFIFO_WDATA_CH_MASK)
        });
    }
}

/// OUT_PUSH_CH2 value type for batch read/modify/write operations
///
/// Use `outpushch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outpushch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPushCh2Val {
    bits: u32,
}

impl OutPushCh2Val {
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

    /// Read **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn outfifo_wdata_ch(&self) -> u32 {
        (self.bits & OutPushCh2::OUTFIFO_WDATA_CH_MASK) >> OutPushCh2::OUTFIFO_WDATA_CH_OFFSET
    }

    /// Set **OUTFIFO_PUSH_CH** `[9:9]`
    ///
    /// Set this bit to push data into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_push_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutPushCh2::OUTFIFO_PUSH_CH_MASK
            } else {
                self.bits & !OutPushCh2::OUTFIFO_PUSH_CH_MASK
            },
        }
    }

    /// Set **OUTFIFO_WDATA_CH** `[8:0]`
    ///
    /// This register stores the data that need to be pushed into AHB_DMA FIFO.
    #[inline(always)]
    pub const fn with_outfifo_wdata_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPushCh2::OUTFIFO_WDATA_CH_MASK)
                | ((value << OutPushCh2::OUTFIFO_WDATA_CH_OFFSET)
                    & OutPushCh2::OUTFIFO_WDATA_CH_MASK),
        }
    }
}

impl Default for OutPushCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPushCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPushCh2Val {
        OutPushCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPushCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPushCh2Val) -> OutPushCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_CH2 Register
///
/// Link descriptor configure and control register of Tx channel 0
///
/// Address offset: 0x0260
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **OUTLINK_PARK_CH** `[3]` (RO)
///
///   1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
///
/// - **OUTLINK_RESTART_CH** `[2]` (WO)
///
///   Set this bit to restart a new outlink from the last address.
///
/// - **OUTLINK_START_CH** `[1]` (WO)
///
///   Set this bit to start dealing with the outlink descriptors.
///
/// - **OUTLINK_STOP_CH** `[0]` (WO)
///
///   Set this bit to stop dealing with the outlink descriptors.
#[repr(transparent)]
pub struct OutLinkCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkCh2 {
    // ======== Field Constants ========

    /// **OUTLINK_PARK_CH**
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OUTLINK_PARK_CH_OFFSET: u32 = 3;
    pub const OUTLINK_PARK_CH_MASK: u32 = 0x00000008;
    pub const OUTLINK_PARK_CH: u32 = 1 << 3;

    /// **OUTLINK_RESTART_CH**
    ///
    /// Set this bit to restart a new outlink from the last address.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OUTLINK_RESTART_CH_OFFSET: u32 = 2;
    pub const OUTLINK_RESTART_CH_MASK: u32 = 0x00000004;
    pub const OUTLINK_RESTART_CH: u32 = 1 << 2;

    /// **OUTLINK_START_CH**
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OUTLINK_START_CH_OFFSET: u32 = 1;
    pub const OUTLINK_START_CH_MASK: u32 = 0x00000002;
    pub const OUTLINK_START_CH: u32 = 1 << 1;

    /// **OUTLINK_STOP_CH**
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUTLINK_STOP_CH_OFFSET: u32 = 0;
    pub const OUTLINK_STOP_CH_MASK: u32 = 0x00000001;
    pub const OUTLINK_STOP_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUTLINK_PARK_CH** `[3]` (RO)
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub fn outlink_park_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Trigger **OUTLINK_RESTART_CH** (WO)
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub fn trigger_outlink_restart_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTLINK_RESTART_CH_MASK);
    }

    /// Trigger **OUTLINK_START_CH** (WO)
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_START_CH_MASK);
    }

    /// Trigger **OUTLINK_STOP_CH** (WO)
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub fn trigger_outlink_stop_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUTLINK_STOP_CH_MASK);
    }
}

/// OUT_LINK_CH2 value type for batch read/modify/write operations
///
/// Use `outlinkch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkCh2Val {
    bits: u32,
}

impl OutLinkCh2Val {
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
        Self { bits: 0x00000008 }
    }

    /// Read **OUTLINK_PARK_CH** `[3:3]`
    ///
    /// 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working.
    #[inline(always)]
    pub const fn outlink_park_ch(&self) -> bool {
        (self.bits & OutLinkCh2::OUTLINK_PARK_CH_MASK) != 0
    }

    /// Set **OUTLINK_RESTART_CH** `[2:2]`
    ///
    /// Set this bit to restart a new outlink from the last address.
    #[inline(always)]
    pub const fn with_outlink_restart_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh2::OUTLINK_RESTART_CH_MASK
            } else {
                self.bits & !OutLinkCh2::OUTLINK_RESTART_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_START_CH** `[1:1]`
    ///
    /// Set this bit to start dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh2::OUTLINK_START_CH_MASK
            } else {
                self.bits & !OutLinkCh2::OUTLINK_START_CH_MASK
            },
        }
    }

    /// Set **OUTLINK_STOP_CH** `[0:0]`
    ///
    /// Set this bit to stop dealing with the outlink descriptors.
    #[inline(always)]
    pub const fn with_outlink_stop_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutLinkCh2::OUTLINK_STOP_CH_MASK
            } else {
                self.bits & !OutLinkCh2::OUTLINK_STOP_CH_MASK
            },
        }
    }
}

impl Default for OutLinkCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutLinkCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkCh2Val {
        OutLinkCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkCh2Val) -> OutLinkCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_STATE_CH2 Register
///
/// Transmit status of Tx channel 0
///
/// Address offset: 0x0264
///
/// ## Fields
///
/// - **OUT_STATE_CH** `[22:20]` (RO)
///
///   reserved
///
/// - **OUT_DSCR_STATE_CH** `[19:18]` (RO)
///
///   reserved
///
/// - **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
///
///   This register stores the current outlink descriptor's address.
#[repr(transparent)]
pub struct OutStateCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutStateCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutStateCh2 {
    // ======== Field Constants ========

    /// **OUT_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const OUT_STATE_CH_OFFSET: u32 = 20;
    pub const OUT_STATE_CH_MASK: u32 = 0x00700000;

    /// **OUT_DSCR_STATE_CH**
    ///
    /// reserved
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const OUT_DSCR_STATE_CH_OFFSET: u32 = 18;
    pub const OUT_DSCR_STATE_CH_MASK: u32 = 0x000c0000;

    /// **OUTLINK_DSCR_ADDR_CH**
    ///
    /// This register stores the current outlink descriptor's address.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const OUTLINK_DSCR_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_ADDR_CH_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **OUT_STATE_CH** `[22:20]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_STATE_CH_MASK) >> Self::OUT_STATE_CH_OFFSET
    }

    /// **OUT_DSCR_STATE_CH** `[19:18]` (RO)
    ///
    /// reserved
    #[inline(always)]
    pub fn out_dscr_state_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_DSCR_STATE_CH_MASK) >> Self::OUT_DSCR_STATE_CH_OFFSET
    }

    /// **OUTLINK_DSCR_ADDR_CH** `[17:0]` (RO)
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_ADDR_CH_MASK)
            >> Self::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

/// OUT_STATE_CH2 value type for batch read/modify/write operations
///
/// Use `outstatech2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outstatech2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutStateCh2Val {
    bits: u32,
}

impl OutStateCh2Val {
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

    /// Read **OUT_STATE_CH** `[22:20]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_state_ch(&self) -> u32 {
        (self.bits & OutStateCh2::OUT_STATE_CH_MASK) >> OutStateCh2::OUT_STATE_CH_OFFSET
    }

    /// Read **OUT_DSCR_STATE_CH** `[19:18]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn out_dscr_state_ch(&self) -> u32 {
        (self.bits & OutStateCh2::OUT_DSCR_STATE_CH_MASK) >> OutStateCh2::OUT_DSCR_STATE_CH_OFFSET
    }

    /// Read **OUTLINK_DSCR_ADDR_CH** `[17:0]`
    ///
    /// This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_dscr_addr_ch(&self) -> u32 {
        (self.bits & OutStateCh2::OUTLINK_DSCR_ADDR_CH_MASK)
            >> OutStateCh2::OUTLINK_DSCR_ADDR_CH_OFFSET
    }
}

impl Default for OutStateCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutStateCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutStateCh2Val {
        OutStateCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_DES_ADDR_CH2 Register
///
/// Outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x0268
///
/// ## Fields
///
/// - **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
#[repr(transparent)]
pub struct OutEofDesAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofDesAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofDesAddrCh2 {
    // ======== Field Constants ========

    /// **OUT_EOF_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub fn out_eof_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_DES_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `outeofdesaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofdesaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofDesAddrCh2Val {
    bits: u32,
}

impl OutEofDesAddrCh2Val {
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

    /// Read **OUT_EOF_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1.
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofDesAddrCh2::OUT_EOF_DES_ADDR_CH_MASK)
            >> OutEofDesAddrCh2::OUT_EOF_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofDesAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofDesAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofDesAddrCh2Val {
        OutEofDesAddrCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH2 Register
///
/// The last outlink descriptor address when EOF occurs of Tx channel 0
///
/// Address offset: 0x026c
///
/// ## Fields
///
/// - **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
///
///   This register stores the address of the outlink descriptor before the last outlink descriptor.
#[repr(transparent)]
pub struct OutEofBfrDesAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEofBfrDesAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEofBfrDesAddrCh2 {
    // ======== Field Constants ========

    /// **OUT_EOF_BFR_DES_ADDR_CH**
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_EOF_BFR_DES_ADDR_CH_OFFSET: u32 = 0;
    pub const OUT_EOF_BFR_DES_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]` (RO)
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> Self::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

/// OUT_EOF_BFR_DES_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `outeofbfrdesaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outeofbfrdesaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEofBfrDesAddrCh2Val {
    bits: u32,
}

impl OutEofBfrDesAddrCh2Val {
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

    /// Read **OUT_EOF_BFR_DES_ADDR_CH** `[31:0]`
    ///
    /// This register stores the address of the outlink descriptor before the last outlink descriptor.
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self) -> u32 {
        (self.bits & OutEofBfrDesAddrCh2::OUT_EOF_BFR_DES_ADDR_CH_MASK)
            >> OutEofBfrDesAddrCh2::OUT_EOF_BFR_DES_ADDR_CH_OFFSET
    }
}

impl Default for OutEofBfrDesAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEofBfrDesAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEofBfrDesAddrCh2Val {
        OutEofBfrDesAddrCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_CH2 Register
///
/// Current inlink descriptor address of Tx channel 0
///
/// Address offset: 0x0270
///
/// ## Fields
///
/// - **OUTLINK_DSCR_CH** `[31:0]` (RO)
///
///   The address of the current outlink descriptor y.
#[repr(transparent)]
pub struct OutDscrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrCh2 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_CH**
    ///
    /// The address of the current outlink descriptor y.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_CH** `[31:0]` (RO)
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub fn outlink_dscr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_CH_MASK) >> Self::OUTLINK_DSCR_CH_OFFSET
    }
}

/// OUT_DSCR_CH2 value type for batch read/modify/write operations
///
/// Use `outdscrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrCh2Val {
    bits: u32,
}

impl OutDscrCh2Val {
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

    /// Read **OUTLINK_DSCR_CH** `[31:0]`
    ///
    /// The address of the current outlink descriptor y.
    #[inline(always)]
    pub const fn outlink_dscr_ch(&self) -> u32 {
        (self.bits & OutDscrCh2::OUTLINK_DSCR_CH_MASK) >> OutDscrCh2::OUTLINK_DSCR_CH_OFFSET
    }
}

impl Default for OutDscrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrCh2Val {
        OutDscrCh2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF0_CH2 Register
///
/// The last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x0274
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
///
///   The address of the last outlink descriptor y-1.
#[repr(transparent)]
pub struct OutDscrBf0Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf0Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf0Ch2 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF0_CH**
    ///
    /// The address of the last outlink descriptor y-1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF0_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF0_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF0_CH** `[31:0]` (RO)
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF0_CH_MASK)
            >> Self::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

/// OUT_DSCR_BF0_CH2 value type for batch read/modify/write operations
///
/// Use `outdscrbf0ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf0ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf0Ch2Val {
    bits: u32,
}

impl OutDscrBf0Ch2Val {
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

    /// Read **OUTLINK_DSCR_BF0_CH** `[31:0]`
    ///
    /// The address of the last outlink descriptor y-1.
    #[inline(always)]
    pub const fn outlink_dscr_bf0_ch(&self) -> u32 {
        (self.bits & OutDscrBf0Ch2::OUTLINK_DSCR_BF0_CH_MASK)
            >> OutDscrBf0Ch2::OUTLINK_DSCR_BF0_CH_OFFSET
    }
}

impl Default for OutDscrBf0Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf0Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf0Ch2Val {
        OutDscrBf0Ch2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_DSCR_BF1_CH2 Register
///
/// The second-to-last inlink descriptor address of Tx channel 0
///
/// Address offset: 0x0278
///
/// ## Fields
///
/// - **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
///
///   The address of the second-to-last inlink descriptor x-2.
#[repr(transparent)]
pub struct OutDscrBf1Ch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutDscrBf1Ch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutDscrBf1Ch2 {
    // ======== Field Constants ========

    /// **OUTLINK_DSCR_BF1_CH**
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_DSCR_BF1_CH_OFFSET: u32 = 0;
    pub const OUTLINK_DSCR_BF1_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_DSCR_BF1_CH** `[31:0]` (RO)
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_DSCR_BF1_CH_MASK)
            >> Self::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

/// OUT_DSCR_BF1_CH2 value type for batch read/modify/write operations
///
/// Use `outdscrbf1ch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdscrbf1ch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutDscrBf1Ch2Val {
    bits: u32,
}

impl OutDscrBf1Ch2Val {
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

    /// Read **OUTLINK_DSCR_BF1_CH** `[31:0]`
    ///
    /// The address of the second-to-last inlink descriptor x-2.
    #[inline(always)]
    pub const fn outlink_dscr_bf1_ch(&self) -> u32 {
        (self.bits & OutDscrBf1Ch2::OUTLINK_DSCR_BF1_CH_MASK)
            >> OutDscrBf1Ch2::OUTLINK_DSCR_BF1_CH_OFFSET
    }
}

impl Default for OutDscrBf1Ch2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutDscrBf1Ch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutDscrBf1Ch2Val {
        OutDscrBf1Ch2Val::from_bits(self.value.read_bits())
    }
}

/// OUT_PRI_CH2 Register
///
/// Priority register of Tx channel 0.
///
/// Address offset: 0x027c
///
/// ## Fields
///
/// - **TX_PRI_CH** `[3:0]` (RW)
///
///   The priority of Tx channel 0. The larger of the value the higher of the priority.
#[repr(transparent)]
pub struct OutPriCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPriCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPriCh2 {
    // ======== Field Constants ========

    /// **TX_PRI_CH**
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_PRI_CH_OFFSET: u32 = 0;
    pub const TX_PRI_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn tx_pri_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_PRI_CH_MASK) >> Self::TX_PRI_CH_OFFSET
    }

    /// Write **TX_PRI_CH** `[3:0]` (RW)
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub fn write_tx_pri_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_PRI_CH_MASK) | ((value << Self::TX_PRI_CH_OFFSET) & Self::TX_PRI_CH_MASK)
        });
    }
}

/// OUT_PRI_CH2 value type for batch read/modify/write operations
///
/// Use `outprich2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outprich2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPriCh2Val {
    bits: u32,
}

impl OutPriCh2Val {
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

    /// Read **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn tx_pri_ch(&self) -> u32 {
        (self.bits & OutPriCh2::TX_PRI_CH_MASK) >> OutPriCh2::TX_PRI_CH_OFFSET
    }

    /// Set **TX_PRI_CH** `[3:0]`
    ///
    /// The priority of Tx channel 0. The larger of the value the higher of the priority.
    #[inline(always)]
    pub const fn with_tx_pri_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPriCh2::TX_PRI_CH_MASK)
                | ((value << OutPriCh2::TX_PRI_CH_OFFSET) & OutPriCh2::TX_PRI_CH_MASK),
        }
    }
}

impl Default for OutPriCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutPriCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPriCh2Val {
        OutPriCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPriCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPriCh2Val) -> OutPriCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_PERI_SEL_CH2 Register
///
/// Peripheral selection of Tx channel 0
///
/// Address offset: 0x0280
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **PERI_OUT_SEL_CH** `[5:0]` (RW)
///
///   This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
#[repr(transparent)]
pub struct OutPeriSelCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutPeriSelCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutPeriSelCh2 {
    // ======== Field Constants ========

    /// **PERI_OUT_SEL_CH**
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PERI_OUT_SEL_CH_OFFSET: u32 = 0;
    pub const PERI_OUT_SEL_CH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_out_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::PERI_OUT_SEL_CH_MASK) >> Self::PERI_OUT_SEL_CH_OFFSET
    }

    /// Write **PERI_OUT_SEL_CH** `[5:0]` (RW)
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn write_peri_out_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PERI_OUT_SEL_CH_MASK)
                | ((value << Self::PERI_OUT_SEL_CH_OFFSET) & Self::PERI_OUT_SEL_CH_MASK)
        });
    }
}

/// OUT_PERI_SEL_CH2 value type for batch read/modify/write operations
///
/// Use `outperiselch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outperiselch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutPeriSelCh2Val {
    bits: u32,
}

impl OutPeriSelCh2Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn peri_out_sel_ch(&self) -> u32 {
        (self.bits & OutPeriSelCh2::PERI_OUT_SEL_CH_MASK) >> OutPeriSelCh2::PERI_OUT_SEL_CH_OFFSET
    }

    /// Set **PERI_OUT_SEL_CH** `[5:0]`
    ///
    /// This register is used to select peripheral for Tx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub const fn with_peri_out_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutPeriSelCh2::PERI_OUT_SEL_CH_MASK)
                | ((value << OutPeriSelCh2::PERI_OUT_SEL_CH_OFFSET)
                    & OutPeriSelCh2::PERI_OUT_SEL_CH_MASK),
        }
    }
}

impl Default for OutPeriSelCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutPeriSelCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutPeriSelCh2Val {
        OutPeriSelCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutPeriSelCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutPeriSelCh2Val) -> OutPeriSelCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_INIT_DATA_CH0 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x02bc
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of tx crc initial value
#[repr(transparent)]
pub struct OutCrcInitDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcInitDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcInitDataCh0 {
    // ======== Field Constants ========

    /// **OUT_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of tx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn out_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_INIT_DATA_CH_MASK)
            >> Self::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn write_out_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::OUT_CRC_INIT_DATA_CH_OFFSET) & Self::OUT_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// OUT_CRC_INIT_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `outcrcinitdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcinitdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcInitDataCh0Val {
    bits: u32,
}

impl OutCrcInitDataCh0Val {
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

    /// Read **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self) -> u32 {
        (self.bits & OutCrcInitDataCh0::OUT_CRC_INIT_DATA_CH_MASK)
            >> OutCrcInitDataCh0::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn with_out_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutCrcInitDataCh0::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << OutCrcInitDataCh0::OUT_CRC_INIT_DATA_CH_OFFSET)
                    & OutCrcInitDataCh0::OUT_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for OutCrcInitDataCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutCrcInitDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcInitDataCh0Val {
        OutCrcInitDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcInitDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcInitDataCh0Val) -> OutCrcInitDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_WIDTH_CH0 Register
///
/// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x02c0
///
/// ## Fields
///
/// - **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **TX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcWidthCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcWidthCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcWidthCh0 {
    // ======== Field Constants ========

    /// **TX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const TX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const TX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **TX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const TX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_WIDTH_CH_MASK) >> Self::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_WIDTH_CH_MASK)
                | ((value << Self::TX_CRC_WIDTH_CH_OFFSET) & Self::TX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// TX_CRC_WIDTH_CH0 value type for batch read/modify/write operations
///
/// Use `txcrcwidthch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcwidthch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcWidthCh0Val {
    bits: u32,
}

impl TxCrcWidthCh0Val {
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

    /// Read **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & TxCrcWidthCh0::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self) -> u32 {
        (self.bits & TxCrcWidthCh0::TX_CRC_WIDTH_CH_MASK) >> TxCrcWidthCh0::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCrcWidthCh0::TX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !TxCrcWidthCh0::TX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcWidthCh0::TX_CRC_WIDTH_CH_MASK)
                | ((value << TxCrcWidthCh0::TX_CRC_WIDTH_CH_OFFSET)
                    & TxCrcWidthCh0::TX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for TxCrcWidthCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcWidthCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcWidthCh0Val {
        TxCrcWidthCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcWidthCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcWidthCh0Val) -> TxCrcWidthCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_CLEAR_CH0 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x02c4
///
/// ## Fields
///
/// - **OUT_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of tx crc result
#[repr(transparent)]
pub struct OutCrcClearCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcClearCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcClearCh0 {
    // ======== Field Constants ========

    /// **OUT_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of tx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const OUT_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub fn out_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_CRC_CLEAR_CH_MASK);
    }

    /// Clear OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_CRC_CLEAR_CH_MASK);
    }
}

/// OUT_CRC_CLEAR_CH0 value type for batch read/modify/write operations
///
/// Use `outcrcclearch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcclearch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcClearCh0Val {
    bits: u32,
}

impl OutCrcClearCh0Val {
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

    /// Read **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self) -> bool {
        (self.bits & OutCrcClearCh0::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn with_out_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutCrcClearCh0::OUT_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !OutCrcClearCh0::OUT_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for OutCrcClearCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcClearCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcClearCh0Val {
        OutCrcClearCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcClearCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcClearCh0Val) -> OutCrcClearCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_FINAL_RESULT_CH0 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x02c8
///
/// ## Fields
///
/// - **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of tx
#[repr(transparent)]
pub struct OutCrcFinalResultCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcFinalResultCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcFinalResultCh0 {
    // ======== Field Constants ========

    /// **OUT_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of tx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub fn out_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> Self::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// OUT_CRC_FINAL_RESULT_CH0 value type for batch read/modify/write operations
///
/// Use `outcrcfinalresultch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcfinalresultch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcFinalResultCh0Val {
    bits: u32,
}

impl OutCrcFinalResultCh0Val {
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

    /// Read **OUT_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self) -> u32 {
        (self.bits & OutCrcFinalResultCh0::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> OutCrcFinalResultCh0::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for OutCrcFinalResultCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcFinalResultCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcFinalResultCh0Val {
        OutCrcFinalResultCh0Val::from_bits(self.value.read_bits())
    }
}

/// TX_CRC_EN_WR_DATA_CH0 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x02cc
///
/// ## Fields
///
/// - **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable tx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct TxCrcEnWrDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnWrDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnWrDataCh0 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_tx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_EN_WR_DATA_CH_OFFSET) & Self::TX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_EN_WR_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `txcrcenwrdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenwrdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnWrDataCh0Val {
    bits: u32,
}

impl TxCrcEnWrDataCh0Val {
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

    /// Read **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcEnWrDataCh0::TX_CRC_EN_WR_DATA_CH_MASK)
            >> TxCrcEnWrDataCh0::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_tx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnWrDataCh0::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcEnWrDataCh0::TX_CRC_EN_WR_DATA_CH_OFFSET)
                    & TxCrcEnWrDataCh0::TX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcEnWrDataCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnWrDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnWrDataCh0Val {
        TxCrcEnWrDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnWrDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnWrDataCh0Val) -> TxCrcEnWrDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_EN_ADDR_CH0 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x02d0
///
/// ## Fields
///
/// - **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcEnAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnAddrCh0 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_ADDR_CH_MASK) >> Self::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_EN_ADDR_CH_OFFSET) & Self::TX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_EN_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `txcrcenaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnAddrCh0Val {
    bits: u32,
}

impl TxCrcEnAddrCh0Val {
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

    /// Read **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcEnAddrCh0::TX_CRC_EN_ADDR_CH_MASK)
            >> TxCrcEnAddrCh0::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnAddrCh0::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << TxCrcEnAddrCh0::TX_CRC_EN_ADDR_CH_OFFSET)
                    & TxCrcEnAddrCh0::TX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcEnAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnAddrCh0Val {
        TxCrcEnAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnAddrCh0Val) -> TxCrcEnAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH0 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x02d4
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnWrDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnWrDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnWrDataCh0 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `txcrcdataenwrdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenwrdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnWrDataCh0Val {
    bits: u32,
}

impl TxCrcDataEnWrDataCh0Val {
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

    /// Read **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnWrDataCh0::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> TxCrcDataEnWrDataCh0::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnWrDataCh0::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcDataEnWrDataCh0::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & TxCrcDataEnWrDataCh0::TX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnWrDataCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnWrDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnWrDataCh0Val {
        TxCrcDataEnWrDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnWrDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnWrDataCh0Val) -> TxCrcDataEnWrDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_ADDR_CH0 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x02d8
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnAddrCh0 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `txcrcdataenaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnAddrCh0Val {
    bits: u32,
}

impl TxCrcDataEnAddrCh0Val {
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

    /// Read **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnAddrCh0::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> TxCrcDataEnAddrCh0::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnAddrCh0::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << TxCrcDataEnAddrCh0::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & TxCrcDataEnAddrCh0::TX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnAddrCh0Val {
        TxCrcDataEnAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnAddrCh0Val) -> TxCrcDataEnAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH_ARB_WEIGH_CH0 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x02dc
///
/// ## Fields
///
/// - **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxChArbWeighCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxChArbWeighCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxChArbWeighCh0 {
    // ======== Field Constants ========

    /// **TX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const TX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CH_ARB_WEIGH_CH_MASK) >> Self::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::TX_CH_ARB_WEIGH_CH_OFFSET) & Self::TX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// TX_CH_ARB_WEIGH_CH0 value type for batch read/modify/write operations
///
/// Use `txcharbweighch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcharbweighch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxChArbWeighCh0Val {
    bits: u32,
}

impl TxChArbWeighCh0Val {
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

    /// Read **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & TxChArbWeighCh0::TX_CH_ARB_WEIGH_CH_MASK)
            >> TxChArbWeighCh0::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxChArbWeighCh0::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << TxChArbWeighCh0::TX_CH_ARB_WEIGH_CH_OFFSET)
                    & TxChArbWeighCh0::TX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for TxChArbWeighCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxChArbWeighCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxChArbWeighCh0Val {
        TxChArbWeighCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxChArbWeighCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxChArbWeighCh0Val) -> TxChArbWeighCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH0 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x02e0
///
/// ## Fields
///
/// - **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxArbWeighOptDirCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxArbWeighOptDirCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxArbWeighOptDirCh0 {
    // ======== Field Constants ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const TX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const TX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH0 value type for batch read/modify/write operations
///
/// Use `txarbweighoptdirch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txarbweighoptdirch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxArbWeighOptDirCh0Val {
    bits: u32,
}

impl TxArbWeighOptDirCh0Val {
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

    /// Read **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & TxArbWeighOptDirCh0::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxArbWeighOptDirCh0::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !TxArbWeighOptDirCh0::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for TxArbWeighOptDirCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxArbWeighOptDirCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxArbWeighOptDirCh0Val {
        TxArbWeighOptDirCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxArbWeighOptDirCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxArbWeighOptDirCh0Val) -> TxArbWeighOptDirCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_INIT_DATA_CH1 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x02e4
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of tx crc initial value
#[repr(transparent)]
pub struct OutCrcInitDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcInitDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcInitDataCh1 {
    // ======== Field Constants ========

    /// **OUT_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of tx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn out_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_INIT_DATA_CH_MASK)
            >> Self::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn write_out_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::OUT_CRC_INIT_DATA_CH_OFFSET) & Self::OUT_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// OUT_CRC_INIT_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `outcrcinitdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcinitdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcInitDataCh1Val {
    bits: u32,
}

impl OutCrcInitDataCh1Val {
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

    /// Read **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self) -> u32 {
        (self.bits & OutCrcInitDataCh1::OUT_CRC_INIT_DATA_CH_MASK)
            >> OutCrcInitDataCh1::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn with_out_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutCrcInitDataCh1::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << OutCrcInitDataCh1::OUT_CRC_INIT_DATA_CH_OFFSET)
                    & OutCrcInitDataCh1::OUT_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for OutCrcInitDataCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutCrcInitDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcInitDataCh1Val {
        OutCrcInitDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcInitDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcInitDataCh1Val) -> OutCrcInitDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_WIDTH_CH1 Register
///
/// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x02e8
///
/// ## Fields
///
/// - **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **TX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcWidthCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcWidthCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcWidthCh1 {
    // ======== Field Constants ========

    /// **TX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const TX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const TX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **TX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const TX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_WIDTH_CH_MASK) >> Self::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_WIDTH_CH_MASK)
                | ((value << Self::TX_CRC_WIDTH_CH_OFFSET) & Self::TX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// TX_CRC_WIDTH_CH1 value type for batch read/modify/write operations
///
/// Use `txcrcwidthch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcwidthch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcWidthCh1Val {
    bits: u32,
}

impl TxCrcWidthCh1Val {
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

    /// Read **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & TxCrcWidthCh1::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self) -> u32 {
        (self.bits & TxCrcWidthCh1::TX_CRC_WIDTH_CH_MASK) >> TxCrcWidthCh1::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCrcWidthCh1::TX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !TxCrcWidthCh1::TX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcWidthCh1::TX_CRC_WIDTH_CH_MASK)
                | ((value << TxCrcWidthCh1::TX_CRC_WIDTH_CH_OFFSET)
                    & TxCrcWidthCh1::TX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for TxCrcWidthCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcWidthCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcWidthCh1Val {
        TxCrcWidthCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcWidthCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcWidthCh1Val) -> TxCrcWidthCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_CLEAR_CH1 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x02ec
///
/// ## Fields
///
/// - **OUT_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of tx crc result
#[repr(transparent)]
pub struct OutCrcClearCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcClearCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcClearCh1 {
    // ======== Field Constants ========

    /// **OUT_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of tx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const OUT_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub fn out_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_CRC_CLEAR_CH_MASK);
    }

    /// Clear OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_CRC_CLEAR_CH_MASK);
    }
}

/// OUT_CRC_CLEAR_CH1 value type for batch read/modify/write operations
///
/// Use `outcrcclearch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcclearch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcClearCh1Val {
    bits: u32,
}

impl OutCrcClearCh1Val {
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

    /// Read **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self) -> bool {
        (self.bits & OutCrcClearCh1::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn with_out_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutCrcClearCh1::OUT_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !OutCrcClearCh1::OUT_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for OutCrcClearCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcClearCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcClearCh1Val {
        OutCrcClearCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcClearCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcClearCh1Val) -> OutCrcClearCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_FINAL_RESULT_CH1 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x02f0
///
/// ## Fields
///
/// - **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of tx
#[repr(transparent)]
pub struct OutCrcFinalResultCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcFinalResultCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcFinalResultCh1 {
    // ======== Field Constants ========

    /// **OUT_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of tx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub fn out_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> Self::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// OUT_CRC_FINAL_RESULT_CH1 value type for batch read/modify/write operations
///
/// Use `outcrcfinalresultch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcfinalresultch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcFinalResultCh1Val {
    bits: u32,
}

impl OutCrcFinalResultCh1Val {
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

    /// Read **OUT_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self) -> u32 {
        (self.bits & OutCrcFinalResultCh1::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> OutCrcFinalResultCh1::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for OutCrcFinalResultCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcFinalResultCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcFinalResultCh1Val {
        OutCrcFinalResultCh1Val::from_bits(self.value.read_bits())
    }
}

/// TX_CRC_EN_WR_DATA_CH1 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x02f4
///
/// ## Fields
///
/// - **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable tx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct TxCrcEnWrDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnWrDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnWrDataCh1 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_tx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_EN_WR_DATA_CH_OFFSET) & Self::TX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_EN_WR_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `txcrcenwrdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenwrdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnWrDataCh1Val {
    bits: u32,
}

impl TxCrcEnWrDataCh1Val {
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

    /// Read **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcEnWrDataCh1::TX_CRC_EN_WR_DATA_CH_MASK)
            >> TxCrcEnWrDataCh1::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_tx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnWrDataCh1::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcEnWrDataCh1::TX_CRC_EN_WR_DATA_CH_OFFSET)
                    & TxCrcEnWrDataCh1::TX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcEnWrDataCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnWrDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnWrDataCh1Val {
        TxCrcEnWrDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnWrDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnWrDataCh1Val) -> TxCrcEnWrDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_EN_ADDR_CH1 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x02f8
///
/// ## Fields
///
/// - **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcEnAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnAddrCh1 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_ADDR_CH_MASK) >> Self::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_EN_ADDR_CH_OFFSET) & Self::TX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_EN_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `txcrcenaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnAddrCh1Val {
    bits: u32,
}

impl TxCrcEnAddrCh1Val {
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

    /// Read **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcEnAddrCh1::TX_CRC_EN_ADDR_CH_MASK)
            >> TxCrcEnAddrCh1::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnAddrCh1::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << TxCrcEnAddrCh1::TX_CRC_EN_ADDR_CH_OFFSET)
                    & TxCrcEnAddrCh1::TX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcEnAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnAddrCh1Val {
        TxCrcEnAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnAddrCh1Val) -> TxCrcEnAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH1 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x02fc
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnWrDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnWrDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnWrDataCh1 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `txcrcdataenwrdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenwrdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnWrDataCh1Val {
    bits: u32,
}

impl TxCrcDataEnWrDataCh1Val {
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

    /// Read **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnWrDataCh1::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> TxCrcDataEnWrDataCh1::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnWrDataCh1::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcDataEnWrDataCh1::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & TxCrcDataEnWrDataCh1::TX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnWrDataCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnWrDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnWrDataCh1Val {
        TxCrcDataEnWrDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnWrDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnWrDataCh1Val) -> TxCrcDataEnWrDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_ADDR_CH1 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x0300
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnAddrCh1 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `txcrcdataenaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnAddrCh1Val {
    bits: u32,
}

impl TxCrcDataEnAddrCh1Val {
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

    /// Read **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnAddrCh1::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> TxCrcDataEnAddrCh1::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnAddrCh1::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << TxCrcDataEnAddrCh1::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & TxCrcDataEnAddrCh1::TX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnAddrCh1Val {
        TxCrcDataEnAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnAddrCh1Val) -> TxCrcDataEnAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH_ARB_WEIGH_CH1 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x0304
///
/// ## Fields
///
/// - **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxChArbWeighCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxChArbWeighCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxChArbWeighCh1 {
    // ======== Field Constants ========

    /// **TX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const TX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CH_ARB_WEIGH_CH_MASK) >> Self::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::TX_CH_ARB_WEIGH_CH_OFFSET) & Self::TX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// TX_CH_ARB_WEIGH_CH1 value type for batch read/modify/write operations
///
/// Use `txcharbweighch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcharbweighch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxChArbWeighCh1Val {
    bits: u32,
}

impl TxChArbWeighCh1Val {
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

    /// Read **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & TxChArbWeighCh1::TX_CH_ARB_WEIGH_CH_MASK)
            >> TxChArbWeighCh1::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxChArbWeighCh1::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << TxChArbWeighCh1::TX_CH_ARB_WEIGH_CH_OFFSET)
                    & TxChArbWeighCh1::TX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for TxChArbWeighCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxChArbWeighCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxChArbWeighCh1Val {
        TxChArbWeighCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxChArbWeighCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxChArbWeighCh1Val) -> TxChArbWeighCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH1 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x0308
///
/// ## Fields
///
/// - **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxArbWeighOptDirCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxArbWeighOptDirCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxArbWeighOptDirCh1 {
    // ======== Field Constants ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const TX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const TX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH1 value type for batch read/modify/write operations
///
/// Use `txarbweighoptdirch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txarbweighoptdirch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxArbWeighOptDirCh1Val {
    bits: u32,
}

impl TxArbWeighOptDirCh1Val {
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

    /// Read **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & TxArbWeighOptDirCh1::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxArbWeighOptDirCh1::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !TxArbWeighOptDirCh1::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for TxArbWeighOptDirCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxArbWeighOptDirCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxArbWeighOptDirCh1Val {
        TxArbWeighOptDirCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxArbWeighOptDirCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxArbWeighOptDirCh1Val) -> TxArbWeighOptDirCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_INIT_DATA_CH2 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x030c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of tx crc initial value
#[repr(transparent)]
pub struct OutCrcInitDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcInitDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcInitDataCh2 {
    // ======== Field Constants ========

    /// **OUT_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of tx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn out_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_INIT_DATA_CH_MASK)
            >> Self::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **OUT_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn write_out_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::OUT_CRC_INIT_DATA_CH_OFFSET) & Self::OUT_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// OUT_CRC_INIT_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `outcrcinitdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcinitdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcInitDataCh2Val {
    bits: u32,
}

impl OutCrcInitDataCh2Val {
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

    /// Read **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self) -> u32 {
        (self.bits & OutCrcInitDataCh2::OUT_CRC_INIT_DATA_CH_MASK)
            >> OutCrcInitDataCh2::OUT_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **OUT_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub const fn with_out_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutCrcInitDataCh2::OUT_CRC_INIT_DATA_CH_MASK)
                | ((value << OutCrcInitDataCh2::OUT_CRC_INIT_DATA_CH_OFFSET)
                    & OutCrcInitDataCh2::OUT_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for OutCrcInitDataCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OutCrcInitDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcInitDataCh2Val {
        OutCrcInitDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcInitDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcInitDataCh2Val) -> OutCrcInitDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_WIDTH_CH2 Register
///
/// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x0310
///
/// ## Fields
///
/// - **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **TX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcWidthCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcWidthCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcWidthCh2 {
    // ======== Field Constants ========

    /// **TX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const TX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const TX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **TX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const TX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear TX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_WIDTH_CH_MASK) >> Self::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **TX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_WIDTH_CH_MASK)
                | ((value << Self::TX_CRC_WIDTH_CH_OFFSET) & Self::TX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// TX_CRC_WIDTH_CH2 value type for batch read/modify/write operations
///
/// Use `txcrcwidthch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcwidthch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcWidthCh2Val {
    bits: u32,
}

impl TxCrcWidthCh2Val {
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

    /// Read **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & TxCrcWidthCh2::TX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self) -> u32 {
        (self.bits & TxCrcWidthCh2::TX_CRC_WIDTH_CH_MASK) >> TxCrcWidthCh2::TX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **TX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxCrcWidthCh2::TX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !TxCrcWidthCh2::TX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **TX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcWidthCh2::TX_CRC_WIDTH_CH_MASK)
                | ((value << TxCrcWidthCh2::TX_CRC_WIDTH_CH_OFFSET)
                    & TxCrcWidthCh2::TX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for TxCrcWidthCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcWidthCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcWidthCh2Val {
        TxCrcWidthCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcWidthCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcWidthCh2Val) -> TxCrcWidthCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_CLEAR_CH2 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x0314
///
/// ## Fields
///
/// - **OUT_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of tx crc result
#[repr(transparent)]
pub struct OutCrcClearCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcClearCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcClearCh2 {
    // ======== Field Constants ========

    /// **OUT_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of tx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OUT_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const OUT_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OUT_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub fn out_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::OUT_CRC_CLEAR_CH_MASK);
    }

    /// Clear OUT_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_CRC_CLEAR_CH_MASK);
    }
}

/// OUT_CRC_CLEAR_CH2 value type for batch read/modify/write operations
///
/// Use `outcrcclearch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcclearch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcClearCh2Val {
    bits: u32,
}

impl OutCrcClearCh2Val {
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

    /// Read **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self) -> bool {
        (self.bits & OutCrcClearCh2::OUT_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **OUT_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of tx crc result
    #[inline(always)]
    pub const fn with_out_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OutCrcClearCh2::OUT_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !OutCrcClearCh2::OUT_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for OutCrcClearCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcClearCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcClearCh2Val {
        OutCrcClearCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutCrcClearCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutCrcClearCh2Val) -> OutCrcClearCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_CRC_FINAL_RESULT_CH2 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x0318
///
/// ## Fields
///
/// - **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of tx
#[repr(transparent)]
pub struct OutCrcFinalResultCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutCrcFinalResultCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutCrcFinalResultCh2 {
    // ======== Field Constants ========

    /// **OUT_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of tx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUT_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const OUT_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUT_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub fn out_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> Self::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// OUT_CRC_FINAL_RESULT_CH2 value type for batch read/modify/write operations
///
/// Use `outcrcfinalresultch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outcrcfinalresultch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutCrcFinalResultCh2Val {
    bits: u32,
}

impl OutCrcFinalResultCh2Val {
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

    /// Read **OUT_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of tx
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self) -> u32 {
        (self.bits & OutCrcFinalResultCh2::OUT_CRC_FINAL_RESULT_CH_MASK)
            >> OutCrcFinalResultCh2::OUT_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for OutCrcFinalResultCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutCrcFinalResultCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutCrcFinalResultCh2Val {
        OutCrcFinalResultCh2Val::from_bits(self.value.read_bits())
    }
}

/// TX_CRC_EN_WR_DATA_CH2 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x031c
///
/// ## Fields
///
/// - **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable tx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct TxCrcEnWrDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnWrDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnWrDataCh2 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_tx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_EN_WR_DATA_CH_OFFSET) & Self::TX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_EN_WR_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `txcrcenwrdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenwrdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnWrDataCh2Val {
    bits: u32,
}

impl TxCrcEnWrDataCh2Val {
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

    /// Read **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcEnWrDataCh2::TX_CRC_EN_WR_DATA_CH_MASK)
            >> TxCrcEnWrDataCh2::TX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable tx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_tx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnWrDataCh2::TX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcEnWrDataCh2::TX_CRC_EN_WR_DATA_CH_OFFSET)
                    & TxCrcEnWrDataCh2::TX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcEnWrDataCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnWrDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnWrDataCh2Val {
        TxCrcEnWrDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnWrDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnWrDataCh2Val) -> TxCrcEnWrDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_EN_ADDR_CH2 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x0320
///
/// ## Fields
///
/// - **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcEnAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcEnAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcEnAddrCh2 {
    // ======== Field Constants ========

    /// **TX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_EN_ADDR_CH_MASK) >> Self::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_EN_ADDR_CH_OFFSET) & Self::TX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_EN_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `txcrcenaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcenaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcEnAddrCh2Val {
    bits: u32,
}

impl TxCrcEnAddrCh2Val {
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

    /// Read **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcEnAddrCh2::TX_CRC_EN_ADDR_CH_MASK)
            >> TxCrcEnAddrCh2::TX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcEnAddrCh2::TX_CRC_EN_ADDR_CH_MASK)
                | ((value << TxCrcEnAddrCh2::TX_CRC_EN_ADDR_CH_OFFSET)
                    & TxCrcEnAddrCh2::TX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcEnAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcEnAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcEnAddrCh2Val {
        TxCrcEnAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcEnAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcEnAddrCh2Val) -> TxCrcEnAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH2 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x0324
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnWrDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnWrDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnWrDataCh2 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_WR_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `txcrcdataenwrdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenwrdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnWrDataCh2Val {
    bits: u32,
}

impl TxCrcDataEnWrDataCh2Val {
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

    /// Read **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnWrDataCh2::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> TxCrcDataEnWrDataCh2::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnWrDataCh2::TX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << TxCrcDataEnWrDataCh2::TX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & TxCrcDataEnWrDataCh2::TX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnWrDataCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnWrDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnWrDataCh2Val {
        TxCrcDataEnWrDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnWrDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnWrDataCh2Val) -> TxCrcDataEnWrDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CRC_DATA_EN_ADDR_CH2 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x0328
///
/// ## Fields
///
/// - **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxCrcDataEnAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxCrcDataEnAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxCrcDataEnAddrCh2 {
    // ======== Field Constants ========

    /// **TX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const TX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **TX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::TX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// TX_CRC_DATA_EN_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `txcrcdataenaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcrcdataenaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxCrcDataEnAddrCh2Val {
    bits: u32,
}

impl TxCrcDataEnAddrCh2Val {
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

    /// Read **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & TxCrcDataEnAddrCh2::TX_CRC_DATA_EN_ADDR_CH_MASK)
            >> TxCrcDataEnAddrCh2::TX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **TX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxCrcDataEnAddrCh2::TX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << TxCrcDataEnAddrCh2::TX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & TxCrcDataEnAddrCh2::TX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for TxCrcDataEnAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxCrcDataEnAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxCrcDataEnAddrCh2Val {
        TxCrcDataEnAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxCrcDataEnAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxCrcDataEnAddrCh2Val) -> TxCrcDataEnAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CH_ARB_WEIGH_CH2 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x032c
///
/// ## Fields
///
/// - **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxChArbWeighCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxChArbWeighCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxChArbWeighCh2 {
    // ======== Field Constants ========

    /// **TX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const TX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CH_ARB_WEIGH_CH_MASK) >> Self::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **TX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_tx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::TX_CH_ARB_WEIGH_CH_OFFSET) & Self::TX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// TX_CH_ARB_WEIGH_CH2 value type for batch read/modify/write operations
///
/// Use `txcharbweighch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txcharbweighch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxChArbWeighCh2Val {
    bits: u32,
}

impl TxChArbWeighCh2Val {
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

    /// Read **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & TxChArbWeighCh2::TX_CH_ARB_WEIGH_CH_MASK)
            >> TxChArbWeighCh2::TX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **TX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxChArbWeighCh2::TX_CH_ARB_WEIGH_CH_MASK)
                | ((value << TxChArbWeighCh2::TX_CH_ARB_WEIGH_CH_OFFSET)
                    & TxChArbWeighCh2::TX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for TxChArbWeighCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxChArbWeighCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxChArbWeighCh2Val {
        TxChArbWeighCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxChArbWeighCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxChArbWeighCh2Val) -> TxChArbWeighCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH2 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x0330
///
/// ## Fields
///
/// - **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct TxArbWeighOptDirCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxArbWeighOptDirCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxArbWeighOptDirCh2 {
    // ======== Field Constants ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const TX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const TX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear TX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// TX_ARB_WEIGH_OPT_DIR_CH2 value type for batch read/modify/write operations
///
/// Use `txarbweighoptdirch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txarbweighoptdirch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxArbWeighOptDirCh2Val {
    bits: u32,
}

impl TxArbWeighOptDirCh2Val {
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

    /// Read **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & TxArbWeighOptDirCh2::TX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **TX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_tx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxArbWeighOptDirCh2::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !TxArbWeighOptDirCh2::TX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for TxArbWeighOptDirCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxArbWeighOptDirCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxArbWeighOptDirCh2Val {
        TxArbWeighOptDirCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxArbWeighOptDirCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxArbWeighOptDirCh2Val) -> TxArbWeighOptDirCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_INIT_DATA_CH0 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x0334
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of rx crc initial value
#[repr(transparent)]
pub struct InCrcInitDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcInitDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcInitDataCh0 {
    // ======== Field Constants ========

    /// **IN_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of rx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const IN_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn in_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_INIT_DATA_CH_MASK)
            >> Self::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn write_in_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::IN_CRC_INIT_DATA_CH_OFFSET) & Self::IN_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// IN_CRC_INIT_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `incrcinitdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcinitdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcInitDataCh0Val {
    bits: u32,
}

impl InCrcInitDataCh0Val {
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

    /// Read **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self) -> u32 {
        (self.bits & InCrcInitDataCh0::IN_CRC_INIT_DATA_CH_MASK)
            >> InCrcInitDataCh0::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn with_in_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InCrcInitDataCh0::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << InCrcInitDataCh0::IN_CRC_INIT_DATA_CH_OFFSET)
                    & InCrcInitDataCh0::IN_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for InCrcInitDataCh0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InCrcInitDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcInitDataCh0Val {
        InCrcInitDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcInitDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcInitDataCh0Val) -> InCrcInitDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_WIDTH_CH0 Register
///
/// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x0338
///
/// ## Fields
///
/// - **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **RX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcWidthCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcWidthCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcWidthCh0 {
    // ======== Field Constants ========

    /// **RX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const RX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const RX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **RX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const RX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const RX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_WIDTH_CH_MASK) >> Self::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_WIDTH_CH_MASK)
                | ((value << Self::RX_CRC_WIDTH_CH_OFFSET) & Self::RX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// RX_CRC_WIDTH_CH0 value type for batch read/modify/write operations
///
/// Use `rxcrcwidthch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcwidthch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcWidthCh0Val {
    bits: u32,
}

impl RxCrcWidthCh0Val {
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

    /// Read **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & RxCrcWidthCh0::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self) -> u32 {
        (self.bits & RxCrcWidthCh0::RX_CRC_WIDTH_CH_MASK) >> RxCrcWidthCh0::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCrcWidthCh0::RX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !RxCrcWidthCh0::RX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcWidthCh0::RX_CRC_WIDTH_CH_MASK)
                | ((value << RxCrcWidthCh0::RX_CRC_WIDTH_CH_OFFSET)
                    & RxCrcWidthCh0::RX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for RxCrcWidthCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcWidthCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcWidthCh0Val {
        RxCrcWidthCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcWidthCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcWidthCh0Val) -> RxCrcWidthCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_CLEAR_CH0 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x033c
///
/// ## Fields
///
/// - **IN_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of rx crc result
#[repr(transparent)]
pub struct InCrcClearCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcClearCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcClearCh0 {
    // ======== Field Constants ========

    /// **IN_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of rx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const IN_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const IN_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub fn in_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CRC_CLEAR_CH_MASK);
    }

    /// Clear IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_CRC_CLEAR_CH_MASK);
    }
}

/// IN_CRC_CLEAR_CH0 value type for batch read/modify/write operations
///
/// Use `incrcclearch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcclearch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcClearCh0Val {
    bits: u32,
}

impl InCrcClearCh0Val {
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

    /// Read **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self) -> bool {
        (self.bits & InCrcClearCh0::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn with_in_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InCrcClearCh0::IN_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !InCrcClearCh0::IN_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for InCrcClearCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcClearCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcClearCh0Val {
        InCrcClearCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcClearCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcClearCh0Val) -> InCrcClearCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_FINAL_RESULT_CH0 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x0340
///
/// ## Fields
///
/// - **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of rx
#[repr(transparent)]
pub struct InCrcFinalResultCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcFinalResultCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcFinalResultCh0 {
    // ======== Field Constants ========

    /// **IN_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of rx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const IN_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub fn in_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_FINAL_RESULT_CH_MASK)
            >> Self::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// IN_CRC_FINAL_RESULT_CH0 value type for batch read/modify/write operations
///
/// Use `incrcfinalresultch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcfinalresultch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcFinalResultCh0Val {
    bits: u32,
}

impl InCrcFinalResultCh0Val {
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

    /// Read **IN_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self) -> u32 {
        (self.bits & InCrcFinalResultCh0::IN_CRC_FINAL_RESULT_CH_MASK)
            >> InCrcFinalResultCh0::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for InCrcFinalResultCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcFinalResultCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcFinalResultCh0Val {
        InCrcFinalResultCh0Val::from_bits(self.value.read_bits())
    }
}

/// RX_CRC_EN_WR_DATA_CH0 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x0344
///
/// ## Fields
///
/// - **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable rx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct RxCrcEnWrDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnWrDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnWrDataCh0 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_rx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_EN_WR_DATA_CH_OFFSET) & Self::RX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_EN_WR_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `rxcrcenwrdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenwrdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnWrDataCh0Val {
    bits: u32,
}

impl RxCrcEnWrDataCh0Val {
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

    /// Read **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcEnWrDataCh0::RX_CRC_EN_WR_DATA_CH_MASK)
            >> RxCrcEnWrDataCh0::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_rx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnWrDataCh0::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcEnWrDataCh0::RX_CRC_EN_WR_DATA_CH_OFFSET)
                    & RxCrcEnWrDataCh0::RX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcEnWrDataCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnWrDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnWrDataCh0Val {
        RxCrcEnWrDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnWrDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnWrDataCh0Val) -> RxCrcEnWrDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_EN_ADDR_CH0 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x0348
///
/// ## Fields
///
/// - **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcEnAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnAddrCh0 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_ADDR_CH_MASK) >> Self::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_EN_ADDR_CH_OFFSET) & Self::RX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_EN_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `rxcrcenaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnAddrCh0Val {
    bits: u32,
}

impl RxCrcEnAddrCh0Val {
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

    /// Read **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcEnAddrCh0::RX_CRC_EN_ADDR_CH_MASK)
            >> RxCrcEnAddrCh0::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnAddrCh0::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << RxCrcEnAddrCh0::RX_CRC_EN_ADDR_CH_OFFSET)
                    & RxCrcEnAddrCh0::RX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcEnAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnAddrCh0Val {
        RxCrcEnAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnAddrCh0Val) -> RxCrcEnAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH0 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x034c
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnWrDataCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnWrDataCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnWrDataCh0 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH0 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenwrdatach0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenwrdatach0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnWrDataCh0Val {
    bits: u32,
}

impl RxCrcDataEnWrDataCh0Val {
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

    /// Read **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnWrDataCh0::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> RxCrcDataEnWrDataCh0::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnWrDataCh0::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcDataEnWrDataCh0::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & RxCrcDataEnWrDataCh0::RX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnWrDataCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnWrDataCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnWrDataCh0Val {
        RxCrcDataEnWrDataCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnWrDataCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnWrDataCh0Val) -> RxCrcDataEnWrDataCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_ADDR_CH0 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x0350
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnAddrCh0 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnAddrCh0Val {
    bits: u32,
}

impl RxCrcDataEnAddrCh0Val {
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

    /// Read **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnAddrCh0::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> RxCrcDataEnAddrCh0::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnAddrCh0::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << RxCrcDataEnAddrCh0::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & RxCrcDataEnAddrCh0::RX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnAddrCh0Val {
        RxCrcDataEnAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnAddrCh0Val) -> RxCrcDataEnAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH_ARB_WEIGH_CH0 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x0354
///
/// ## Fields
///
/// - **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxChArbWeighCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxChArbWeighCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxChArbWeighCh0 {
    // ======== Field Constants ========

    /// **RX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const RX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CH_ARB_WEIGH_CH_MASK) >> Self::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::RX_CH_ARB_WEIGH_CH_OFFSET) & Self::RX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// RX_CH_ARB_WEIGH_CH0 value type for batch read/modify/write operations
///
/// Use `rxcharbweighch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcharbweighch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxChArbWeighCh0Val {
    bits: u32,
}

impl RxChArbWeighCh0Val {
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

    /// Read **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & RxChArbWeighCh0::RX_CH_ARB_WEIGH_CH_MASK)
            >> RxChArbWeighCh0::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxChArbWeighCh0::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << RxChArbWeighCh0::RX_CH_ARB_WEIGH_CH_OFFSET)
                    & RxChArbWeighCh0::RX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for RxChArbWeighCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxChArbWeighCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxChArbWeighCh0Val {
        RxChArbWeighCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxChArbWeighCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxChArbWeighCh0Val) -> RxChArbWeighCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH0 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x0358
///
/// ## Fields
///
/// - **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxArbWeighOptDirCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxArbWeighOptDirCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxArbWeighOptDirCh0 {
    // ======== Field Constants ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const RX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const RX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH0 value type for batch read/modify/write operations
///
/// Use `rxarbweighoptdirch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxarbweighoptdirch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxArbWeighOptDirCh0Val {
    bits: u32,
}

impl RxArbWeighOptDirCh0Val {
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

    /// Read **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & RxArbWeighOptDirCh0::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxArbWeighOptDirCh0::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !RxArbWeighOptDirCh0::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for RxArbWeighOptDirCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxArbWeighOptDirCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxArbWeighOptDirCh0Val {
        RxArbWeighOptDirCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxArbWeighOptDirCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxArbWeighOptDirCh0Val) -> RxArbWeighOptDirCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_INIT_DATA_CH1 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x035c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of rx crc initial value
#[repr(transparent)]
pub struct InCrcInitDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcInitDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcInitDataCh1 {
    // ======== Field Constants ========

    /// **IN_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of rx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const IN_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn in_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_INIT_DATA_CH_MASK)
            >> Self::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn write_in_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::IN_CRC_INIT_DATA_CH_OFFSET) & Self::IN_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// IN_CRC_INIT_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `incrcinitdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcinitdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcInitDataCh1Val {
    bits: u32,
}

impl InCrcInitDataCh1Val {
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

    /// Read **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self) -> u32 {
        (self.bits & InCrcInitDataCh1::IN_CRC_INIT_DATA_CH_MASK)
            >> InCrcInitDataCh1::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn with_in_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InCrcInitDataCh1::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << InCrcInitDataCh1::IN_CRC_INIT_DATA_CH_OFFSET)
                    & InCrcInitDataCh1::IN_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for InCrcInitDataCh1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InCrcInitDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcInitDataCh1Val {
        InCrcInitDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcInitDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcInitDataCh1Val) -> InCrcInitDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_WIDTH_CH1 Register
///
/// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x0360
///
/// ## Fields
///
/// - **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **RX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcWidthCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcWidthCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcWidthCh1 {
    // ======== Field Constants ========

    /// **RX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const RX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const RX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **RX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const RX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const RX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_WIDTH_CH_MASK) >> Self::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_WIDTH_CH_MASK)
                | ((value << Self::RX_CRC_WIDTH_CH_OFFSET) & Self::RX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// RX_CRC_WIDTH_CH1 value type for batch read/modify/write operations
///
/// Use `rxcrcwidthch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcwidthch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcWidthCh1Val {
    bits: u32,
}

impl RxCrcWidthCh1Val {
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

    /// Read **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & RxCrcWidthCh1::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self) -> u32 {
        (self.bits & RxCrcWidthCh1::RX_CRC_WIDTH_CH_MASK) >> RxCrcWidthCh1::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCrcWidthCh1::RX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !RxCrcWidthCh1::RX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcWidthCh1::RX_CRC_WIDTH_CH_MASK)
                | ((value << RxCrcWidthCh1::RX_CRC_WIDTH_CH_OFFSET)
                    & RxCrcWidthCh1::RX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for RxCrcWidthCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcWidthCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcWidthCh1Val {
        RxCrcWidthCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcWidthCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcWidthCh1Val) -> RxCrcWidthCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_CLEAR_CH1 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x0364
///
/// ## Fields
///
/// - **IN_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of rx crc result
#[repr(transparent)]
pub struct InCrcClearCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcClearCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcClearCh1 {
    // ======== Field Constants ========

    /// **IN_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of rx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const IN_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const IN_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub fn in_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CRC_CLEAR_CH_MASK);
    }

    /// Clear IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_CRC_CLEAR_CH_MASK);
    }
}

/// IN_CRC_CLEAR_CH1 value type for batch read/modify/write operations
///
/// Use `incrcclearch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcclearch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcClearCh1Val {
    bits: u32,
}

impl InCrcClearCh1Val {
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

    /// Read **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self) -> bool {
        (self.bits & InCrcClearCh1::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn with_in_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InCrcClearCh1::IN_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !InCrcClearCh1::IN_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for InCrcClearCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcClearCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcClearCh1Val {
        InCrcClearCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcClearCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcClearCh1Val) -> InCrcClearCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_FINAL_RESULT_CH1 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x0368
///
/// ## Fields
///
/// - **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of rx
#[repr(transparent)]
pub struct InCrcFinalResultCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcFinalResultCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcFinalResultCh1 {
    // ======== Field Constants ========

    /// **IN_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of rx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const IN_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub fn in_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_FINAL_RESULT_CH_MASK)
            >> Self::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// IN_CRC_FINAL_RESULT_CH1 value type for batch read/modify/write operations
///
/// Use `incrcfinalresultch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcfinalresultch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcFinalResultCh1Val {
    bits: u32,
}

impl InCrcFinalResultCh1Val {
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

    /// Read **IN_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self) -> u32 {
        (self.bits & InCrcFinalResultCh1::IN_CRC_FINAL_RESULT_CH_MASK)
            >> InCrcFinalResultCh1::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for InCrcFinalResultCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcFinalResultCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcFinalResultCh1Val {
        InCrcFinalResultCh1Val::from_bits(self.value.read_bits())
    }
}

/// RX_CRC_EN_WR_DATA_CH1 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x036c
///
/// ## Fields
///
/// - **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable rx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct RxCrcEnWrDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnWrDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnWrDataCh1 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_rx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_EN_WR_DATA_CH_OFFSET) & Self::RX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_EN_WR_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `rxcrcenwrdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenwrdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnWrDataCh1Val {
    bits: u32,
}

impl RxCrcEnWrDataCh1Val {
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

    /// Read **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcEnWrDataCh1::RX_CRC_EN_WR_DATA_CH_MASK)
            >> RxCrcEnWrDataCh1::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_rx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnWrDataCh1::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcEnWrDataCh1::RX_CRC_EN_WR_DATA_CH_OFFSET)
                    & RxCrcEnWrDataCh1::RX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcEnWrDataCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnWrDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnWrDataCh1Val {
        RxCrcEnWrDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnWrDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnWrDataCh1Val) -> RxCrcEnWrDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_EN_ADDR_CH1 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x0370
///
/// ## Fields
///
/// - **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcEnAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnAddrCh1 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_ADDR_CH_MASK) >> Self::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_EN_ADDR_CH_OFFSET) & Self::RX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_EN_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `rxcrcenaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnAddrCh1Val {
    bits: u32,
}

impl RxCrcEnAddrCh1Val {
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

    /// Read **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcEnAddrCh1::RX_CRC_EN_ADDR_CH_MASK)
            >> RxCrcEnAddrCh1::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnAddrCh1::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << RxCrcEnAddrCh1::RX_CRC_EN_ADDR_CH_OFFSET)
                    & RxCrcEnAddrCh1::RX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcEnAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnAddrCh1Val {
        RxCrcEnAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnAddrCh1Val) -> RxCrcEnAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH1 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x0374
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnWrDataCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnWrDataCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnWrDataCh1 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH1 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenwrdatach1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenwrdatach1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnWrDataCh1Val {
    bits: u32,
}

impl RxCrcDataEnWrDataCh1Val {
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

    /// Read **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnWrDataCh1::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> RxCrcDataEnWrDataCh1::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnWrDataCh1::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcDataEnWrDataCh1::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & RxCrcDataEnWrDataCh1::RX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnWrDataCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnWrDataCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnWrDataCh1Val {
        RxCrcDataEnWrDataCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnWrDataCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnWrDataCh1Val) -> RxCrcDataEnWrDataCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_ADDR_CH1 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x0378
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnAddrCh1 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnAddrCh1Val {
    bits: u32,
}

impl RxCrcDataEnAddrCh1Val {
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

    /// Read **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnAddrCh1::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> RxCrcDataEnAddrCh1::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnAddrCh1::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << RxCrcDataEnAddrCh1::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & RxCrcDataEnAddrCh1::RX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnAddrCh1Val {
        RxCrcDataEnAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnAddrCh1Val) -> RxCrcDataEnAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH_ARB_WEIGH_CH1 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x037c
///
/// ## Fields
///
/// - **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxChArbWeighCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxChArbWeighCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxChArbWeighCh1 {
    // ======== Field Constants ========

    /// **RX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const RX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CH_ARB_WEIGH_CH_MASK) >> Self::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::RX_CH_ARB_WEIGH_CH_OFFSET) & Self::RX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// RX_CH_ARB_WEIGH_CH1 value type for batch read/modify/write operations
///
/// Use `rxcharbweighch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcharbweighch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxChArbWeighCh1Val {
    bits: u32,
}

impl RxChArbWeighCh1Val {
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

    /// Read **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & RxChArbWeighCh1::RX_CH_ARB_WEIGH_CH_MASK)
            >> RxChArbWeighCh1::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxChArbWeighCh1::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << RxChArbWeighCh1::RX_CH_ARB_WEIGH_CH_OFFSET)
                    & RxChArbWeighCh1::RX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for RxChArbWeighCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxChArbWeighCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxChArbWeighCh1Val {
        RxChArbWeighCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxChArbWeighCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxChArbWeighCh1Val) -> RxChArbWeighCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH1 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x0380
///
/// ## Fields
///
/// - **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxArbWeighOptDirCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxArbWeighOptDirCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxArbWeighOptDirCh1 {
    // ======== Field Constants ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const RX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const RX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH1 value type for batch read/modify/write operations
///
/// Use `rxarbweighoptdirch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxarbweighoptdirch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxArbWeighOptDirCh1Val {
    bits: u32,
}

impl RxArbWeighOptDirCh1Val {
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

    /// Read **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & RxArbWeighOptDirCh1::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxArbWeighOptDirCh1::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !RxArbWeighOptDirCh1::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for RxArbWeighOptDirCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxArbWeighOptDirCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxArbWeighOptDirCh1Val {
        RxArbWeighOptDirCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxArbWeighOptDirCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxArbWeighOptDirCh1Val) -> RxArbWeighOptDirCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_INIT_DATA_CH2 Register
///
/// This register is used to config ch0 crc initial data(max 32 bit)
///
/// Address offset: 0x0384
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
///
///   This register is used to config ch0 of rx crc initial value
#[repr(transparent)]
pub struct InCrcInitDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcInitDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcInitDataCh2 {
    // ======== Field Constants ========

    /// **IN_CRC_INIT_DATA_CH**
    ///
    /// This register is used to config ch0 of rx crc initial value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_INIT_DATA_CH_OFFSET: u32 = 0;
    pub const IN_CRC_INIT_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn in_crc_init_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_INIT_DATA_CH_MASK)
            >> Self::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Write **IN_CRC_INIT_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub fn write_in_crc_init_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << Self::IN_CRC_INIT_DATA_CH_OFFSET) & Self::IN_CRC_INIT_DATA_CH_MASK)
        });
    }
}

/// IN_CRC_INIT_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `incrcinitdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcinitdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcInitDataCh2Val {
    bits: u32,
}

impl InCrcInitDataCh2Val {
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

    /// Read **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self) -> u32 {
        (self.bits & InCrcInitDataCh2::IN_CRC_INIT_DATA_CH_MASK)
            >> InCrcInitDataCh2::IN_CRC_INIT_DATA_CH_OFFSET
    }

    /// Set **IN_CRC_INIT_DATA_CH** `[31:0]`
    ///
    /// This register is used to config ch0 of rx crc initial value
    #[inline(always)]
    pub const fn with_in_crc_init_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InCrcInitDataCh2::IN_CRC_INIT_DATA_CH_MASK)
                | ((value << InCrcInitDataCh2::IN_CRC_INIT_DATA_CH_OFFSET)
                    & InCrcInitDataCh2::IN_CRC_INIT_DATA_CH_MASK),
        }
    }
}

impl Default for InCrcInitDataCh2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InCrcInitDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcInitDataCh2Val {
        InCrcInitDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcInitDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcInitDataCh2Val) -> InCrcInitDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_WIDTH_CH2 Register
///
/// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
///
/// Address offset: 0x0388
///
/// ## Fields
///
/// - **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
///
///   reserved
///
/// - **RX_CRC_WIDTH_CH** `[1:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcWidthCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcWidthCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcWidthCh2 {
    // ======== Field Constants ========

    /// **RX_CRC_LAUTCH_FLGA_CH**
    ///
    /// reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_CRC_LAUTCH_FLGA_CH_OFFSET: u32 = 2;
    pub const RX_CRC_LAUTCH_FLGA_CH_MASK: u32 = 0x00000004;
    pub const RX_CRC_LAUTCH_FLGA_CH: u32 = 1 << 2;

    /// **RX_CRC_WIDTH_CH**
    ///
    /// reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const RX_CRC_WIDTH_CH_OFFSET: u32 = 0;
    pub const RX_CRC_WIDTH_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **RX_CRC_LAUTCH_FLGA_CH** `[2]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Set RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// Clear RX_CRC_LAUTCH_FLGA_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_crc_lautch_flga_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_CRC_LAUTCH_FLGA_CH_MASK);
    }

    /// **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_width_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_WIDTH_CH_MASK) >> Self::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Write **RX_CRC_WIDTH_CH** `[1:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_width_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_WIDTH_CH_MASK)
                | ((value << Self::RX_CRC_WIDTH_CH_OFFSET) & Self::RX_CRC_WIDTH_CH_MASK)
        });
    }
}

/// RX_CRC_WIDTH_CH2 value type for batch read/modify/write operations
///
/// Use `rxcrcwidthch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcwidthch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcWidthCh2Val {
    bits: u32,
}

impl RxCrcWidthCh2Val {
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

    /// Read **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_lautch_flga_ch(&self) -> bool {
        (self.bits & RxCrcWidthCh2::RX_CRC_LAUTCH_FLGA_CH_MASK) != 0
    }

    /// Read **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self) -> u32 {
        (self.bits & RxCrcWidthCh2::RX_CRC_WIDTH_CH_MASK) >> RxCrcWidthCh2::RX_CRC_WIDTH_CH_OFFSET
    }

    /// Set **RX_CRC_LAUTCH_FLGA_CH** `[2:2]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_lautch_flga_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxCrcWidthCh2::RX_CRC_LAUTCH_FLGA_CH_MASK
            } else {
                self.bits & !RxCrcWidthCh2::RX_CRC_LAUTCH_FLGA_CH_MASK
            },
        }
    }

    /// Set **RX_CRC_WIDTH_CH** `[1:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_width_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcWidthCh2::RX_CRC_WIDTH_CH_MASK)
                | ((value << RxCrcWidthCh2::RX_CRC_WIDTH_CH_OFFSET)
                    & RxCrcWidthCh2::RX_CRC_WIDTH_CH_MASK),
        }
    }
}

impl Default for RxCrcWidthCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcWidthCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcWidthCh2Val {
        RxCrcWidthCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcWidthCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcWidthCh2Val) -> RxCrcWidthCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_CLEAR_CH2 Register
///
/// This register is used to clear ch0 crc result
///
/// Address offset: 0x038c
///
/// ## Fields
///
/// - **IN_CRC_CLEAR_CH** `[0]` (RW)
///
///   This register is used to clear ch0 of rx crc result
#[repr(transparent)]
pub struct InCrcClearCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcClearCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcClearCh2 {
    // ======== Field Constants ========

    /// **IN_CRC_CLEAR_CH**
    ///
    /// This register is used to clear ch0 of rx crc result
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IN_CRC_CLEAR_CH_OFFSET: u32 = 0;
    pub const IN_CRC_CLEAR_CH_MASK: u32 = 0x00000001;
    pub const IN_CRC_CLEAR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IN_CRC_CLEAR_CH** `[0]` (RW)
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub fn in_crc_clear_ch(&self) -> bool {
        (self.value.read_bits() & Self::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r | Self::IN_CRC_CLEAR_CH_MASK);
    }

    /// Clear IN_CRC_CLEAR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_crc_clear_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IN_CRC_CLEAR_CH_MASK);
    }
}

/// IN_CRC_CLEAR_CH2 value type for batch read/modify/write operations
///
/// Use `incrcclearch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcclearch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcClearCh2Val {
    bits: u32,
}

impl InCrcClearCh2Val {
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

    /// Read **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self) -> bool {
        (self.bits & InCrcClearCh2::IN_CRC_CLEAR_CH_MASK) != 0
    }

    /// Set **IN_CRC_CLEAR_CH** `[0:0]`
    ///
    /// This register is used to clear ch0 of rx crc result
    #[inline(always)]
    pub const fn with_in_crc_clear_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InCrcClearCh2::IN_CRC_CLEAR_CH_MASK
            } else {
                self.bits & !InCrcClearCh2::IN_CRC_CLEAR_CH_MASK
            },
        }
    }
}

impl Default for InCrcClearCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcClearCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcClearCh2Val {
        InCrcClearCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InCrcClearCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InCrcClearCh2Val) -> InCrcClearCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_CRC_FINAL_RESULT_CH2 Register
///
/// This register is used to store ch0 crc result
///
/// Address offset: 0x0390
///
/// ## Fields
///
/// - **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
///
///   This register is used to store result ch0 of rx
#[repr(transparent)]
pub struct InCrcFinalResultCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InCrcFinalResultCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InCrcFinalResultCh2 {
    // ======== Field Constants ========

    /// **IN_CRC_FINAL_RESULT_CH**
    ///
    /// This register is used to store result ch0 of rx
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IN_CRC_FINAL_RESULT_CH_OFFSET: u32 = 0;
    pub const IN_CRC_FINAL_RESULT_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IN_CRC_FINAL_RESULT_CH** `[31:0]` (RO)
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub fn in_crc_final_result_ch(&self) -> u32 {
        (self.value.read_bits() & Self::IN_CRC_FINAL_RESULT_CH_MASK)
            >> Self::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

/// IN_CRC_FINAL_RESULT_CH2 value type for batch read/modify/write operations
///
/// Use `incrcfinalresultch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incrcfinalresultch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InCrcFinalResultCh2Val {
    bits: u32,
}

impl InCrcFinalResultCh2Val {
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

    /// Read **IN_CRC_FINAL_RESULT_CH** `[31:0]`
    ///
    /// This register is used to store result ch0 of rx
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self) -> u32 {
        (self.bits & InCrcFinalResultCh2::IN_CRC_FINAL_RESULT_CH_MASK)
            >> InCrcFinalResultCh2::IN_CRC_FINAL_RESULT_CH_OFFSET
    }
}

impl Default for InCrcFinalResultCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InCrcFinalResultCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InCrcFinalResultCh2Val {
        InCrcFinalResultCh2Val::from_bits(self.value.read_bits())
    }
}

/// RX_CRC_EN_WR_DATA_CH2 Register
///
/// This resister is used to config ch0 crc en for every bit
///
/// Address offset: 0x0394
///
/// ## Fields
///
/// - **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
///
///   This register is used to enable rx ch0 crc 32bit on/off
#[repr(transparent)]
pub struct RxCrcEnWrDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnWrDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnWrDataCh2 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_WR_DATA_CH**
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_WR_DATA_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_EN_WR_DATA_CH** `[31:0]` (RW)
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub fn write_rx_crc_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_EN_WR_DATA_CH_OFFSET) & Self::RX_CRC_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_EN_WR_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `rxcrcenwrdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenwrdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnWrDataCh2Val {
    bits: u32,
}

impl RxCrcEnWrDataCh2Val {
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

    /// Read **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcEnWrDataCh2::RX_CRC_EN_WR_DATA_CH_MASK)
            >> RxCrcEnWrDataCh2::RX_CRC_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_EN_WR_DATA_CH** `[31:0]`
    ///
    /// This register is used to enable rx ch0 crc 32bit on/off
    #[inline(always)]
    pub const fn with_rx_crc_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnWrDataCh2::RX_CRC_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcEnWrDataCh2::RX_CRC_EN_WR_DATA_CH_OFFSET)
                    & RxCrcEnWrDataCh2::RX_CRC_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcEnWrDataCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnWrDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnWrDataCh2Val {
        RxCrcEnWrDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnWrDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnWrDataCh2Val) -> RxCrcEnWrDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_EN_ADDR_CH2 Register
///
/// This register is used to config ch0 crc en addr
///
/// Address offset: 0x0398
///
/// ## Fields
///
/// - **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcEnAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcEnAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcEnAddrCh2 {
    // ======== Field Constants ========

    /// **RX_CRC_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_EN_ADDR_CH_MASK) >> Self::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_EN_ADDR_CH_OFFSET) & Self::RX_CRC_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_EN_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `rxcrcenaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcenaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcEnAddrCh2Val {
    bits: u32,
}

impl RxCrcEnAddrCh2Val {
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

    /// Read **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcEnAddrCh2::RX_CRC_EN_ADDR_CH_MASK)
            >> RxCrcEnAddrCh2::RX_CRC_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcEnAddrCh2::RX_CRC_EN_ADDR_CH_MASK)
                | ((value << RxCrcEnAddrCh2::RX_CRC_EN_ADDR_CH_OFFSET)
                    & RxCrcEnAddrCh2::RX_CRC_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcEnAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcEnAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcEnAddrCh2Val {
        RxCrcEnAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcEnAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcEnAddrCh2Val) -> RxCrcEnAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH2 Register
///
/// This register is used to config crc data_8bit en
///
/// Address offset: 0x039c
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnWrDataCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnWrDataCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnWrDataCh2 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH**
    ///
    /// reserved
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RX_CRC_DATA_EN_WR_DATA_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_WR_DATA_CH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_wr_data_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_WR_DATA_CH2 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenwrdatach2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenwrdatach2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnWrDataCh2Val {
    bits: u32,
}

impl RxCrcDataEnWrDataCh2Val {
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

    /// Read **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnWrDataCh2::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
            >> RxCrcDataEnWrDataCh2::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_WR_DATA_CH** `[7:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_wr_data_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnWrDataCh2::RX_CRC_DATA_EN_WR_DATA_CH_MASK)
                | ((value << RxCrcDataEnWrDataCh2::RX_CRC_DATA_EN_WR_DATA_CH_OFFSET)
                    & RxCrcDataEnWrDataCh2::RX_CRC_DATA_EN_WR_DATA_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnWrDataCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnWrDataCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnWrDataCh2Val {
        RxCrcDataEnWrDataCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnWrDataCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnWrDataCh2Val) -> RxCrcDataEnWrDataCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CRC_DATA_EN_ADDR_CH2 Register
///
/// This register is used to config addr of crc data_8bit en
///
/// Address offset: 0x03a0
///
/// ## Fields
///
/// - **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxCrcDataEnAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxCrcDataEnAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxCrcDataEnAddrCh2 {
    // ======== Field Constants ========

    /// **RX_CRC_DATA_EN_ADDR_CH**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_CRC_DATA_EN_ADDR_CH_OFFSET: u32 = 0;
    pub const RX_CRC_DATA_EN_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Write **RX_CRC_DATA_EN_ADDR_CH** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_crc_data_en_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << Self::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & Self::RX_CRC_DATA_EN_ADDR_CH_MASK)
        });
    }
}

/// RX_CRC_DATA_EN_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `rxcrcdataenaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcrcdataenaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxCrcDataEnAddrCh2Val {
    bits: u32,
}

impl RxCrcDataEnAddrCh2Val {
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

    /// Read **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_crc_data_en_addr_ch(&self) -> u32 {
        (self.bits & RxCrcDataEnAddrCh2::RX_CRC_DATA_EN_ADDR_CH_MASK)
            >> RxCrcDataEnAddrCh2::RX_CRC_DATA_EN_ADDR_CH_OFFSET
    }

    /// Set **RX_CRC_DATA_EN_ADDR_CH** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_crc_data_en_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxCrcDataEnAddrCh2::RX_CRC_DATA_EN_ADDR_CH_MASK)
                | ((value << RxCrcDataEnAddrCh2::RX_CRC_DATA_EN_ADDR_CH_OFFSET)
                    & RxCrcDataEnAddrCh2::RX_CRC_DATA_EN_ADDR_CH_MASK),
        }
    }
}

impl Default for RxCrcDataEnAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxCrcDataEnAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxCrcDataEnAddrCh2Val {
        RxCrcDataEnAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxCrcDataEnAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxCrcDataEnAddrCh2Val) -> RxCrcDataEnAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_CH_ARB_WEIGH_CH2 Register
///
/// This register is used to config ch0 arbiter weigh
///
/// Address offset: 0x03a4
///
/// ## Fields
///
/// - **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxChArbWeighCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxChArbWeighCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxChArbWeighCh2 {
    // ======== Field Constants ========

    /// **RX_CH_ARB_WEIGH_CH**
    ///
    /// reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RX_CH_ARB_WEIGH_CH_OFFSET: u32 = 0;
    pub const RX_CH_ARB_WEIGH_CH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CH_ARB_WEIGH_CH_MASK) >> Self::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Write **RX_CH_ARB_WEIGH_CH** `[3:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_rx_ch_arb_weigh_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << Self::RX_CH_ARB_WEIGH_CH_OFFSET) & Self::RX_CH_ARB_WEIGH_CH_MASK)
        });
    }
}

/// RX_CH_ARB_WEIGH_CH2 value type for batch read/modify/write operations
///
/// Use `rxcharbweighch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxcharbweighch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxChArbWeighCh2Val {
    bits: u32,
}

impl RxChArbWeighCh2Val {
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

    /// Read **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self) -> u32 {
        (self.bits & RxChArbWeighCh2::RX_CH_ARB_WEIGH_CH_MASK)
            >> RxChArbWeighCh2::RX_CH_ARB_WEIGH_CH_OFFSET
    }

    /// Set **RX_CH_ARB_WEIGH_CH** `[3:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_ch_arb_weigh_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxChArbWeighCh2::RX_CH_ARB_WEIGH_CH_MASK)
                | ((value << RxChArbWeighCh2::RX_CH_ARB_WEIGH_CH_OFFSET)
                    & RxChArbWeighCh2::RX_CH_ARB_WEIGH_CH_MASK),
        }
    }
}

impl Default for RxChArbWeighCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxChArbWeighCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxChArbWeighCh2Val {
        RxChArbWeighCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxChArbWeighCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxChArbWeighCh2Val) -> RxChArbWeighCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH2 Register
///
/// This register is used to config off or on weigh optimization
///
/// Address offset: 0x03a8
///
/// ## Fields
///
/// - **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct RxArbWeighOptDirCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxArbWeighOptDirCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxArbWeighOptDirCh2 {
    // ======== Field Constants ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_ARB_WEIGH_OPT_DIR_CH_OFFSET: u32 = 0;
    pub const RX_ARB_WEIGH_OPT_DIR_CH_MASK: u32 = 0x00000001;
    pub const RX_ARB_WEIGH_OPT_DIR_CH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_ARB_WEIGH_OPT_DIR_CH** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.value.read_bits() & Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }

    /// Clear RX_ARB_WEIGH_OPT_DIR_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_arb_weigh_opt_dir_ch(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_ARB_WEIGH_OPT_DIR_CH_MASK);
    }
}

/// RX_ARB_WEIGH_OPT_DIR_CH2 value type for batch read/modify/write operations
///
/// Use `rxarbweighoptdirch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxarbweighoptdirch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxArbWeighOptDirCh2Val {
    bits: u32,
}

impl RxArbWeighOptDirCh2Val {
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

    /// Read **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self) -> bool {
        (self.bits & RxArbWeighOptDirCh2::RX_ARB_WEIGH_OPT_DIR_CH_MASK) != 0
    }

    /// Set **RX_ARB_WEIGH_OPT_DIR_CH** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_rx_arb_weigh_opt_dir_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxArbWeighOptDirCh2::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            } else {
                self.bits & !RxArbWeighOptDirCh2::RX_ARB_WEIGH_OPT_DIR_CH_MASK
            },
        }
    }
}

impl Default for RxArbWeighOptDirCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxArbWeighOptDirCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxArbWeighOptDirCh2Val {
        RxArbWeighOptDirCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxArbWeighOptDirCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxArbWeighOptDirCh2Val) -> RxArbWeighOptDirCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_ADDR_CH0 Register
///
/// Link descriptor configure of Rx channel 0
///
/// Address offset: 0x03ac
///
/// ## Fields
///
/// - **INLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first inlink descriptor's address.
#[repr(transparent)]
pub struct InLinkAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkAddrCh0 {
    // ======== Field Constants ========

    /// **INLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_ADDR_CH_MASK) >> Self::INLINK_ADDR_CH_OFFSET
    }

    /// Write **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn write_inlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INLINK_ADDR_CH_MASK)
                | ((value << Self::INLINK_ADDR_CH_OFFSET) & Self::INLINK_ADDR_CH_MASK)
        });
    }
}

/// IN_LINK_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `inlinkaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkAddrCh0Val {
    bits: u32,
}

impl InLinkAddrCh0Val {
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

    /// Read **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_addr_ch(&self) -> u32 {
        (self.bits & InLinkAddrCh0::INLINK_ADDR_CH_MASK) >> InLinkAddrCh0::INLINK_ADDR_CH_OFFSET
    }

    /// Set **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn with_inlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InLinkAddrCh0::INLINK_ADDR_CH_MASK)
                | ((value << InLinkAddrCh0::INLINK_ADDR_CH_OFFSET)
                    & InLinkAddrCh0::INLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for InLinkAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InLinkAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkAddrCh0Val {
        InLinkAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkAddrCh0Val) -> InLinkAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_ADDR_CH1 Register
///
/// Link descriptor configure of Rx channel 0
///
/// Address offset: 0x03b0
///
/// ## Fields
///
/// - **INLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first inlink descriptor's address.
#[repr(transparent)]
pub struct InLinkAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkAddrCh1 {
    // ======== Field Constants ========

    /// **INLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_ADDR_CH_MASK) >> Self::INLINK_ADDR_CH_OFFSET
    }

    /// Write **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn write_inlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INLINK_ADDR_CH_MASK)
                | ((value << Self::INLINK_ADDR_CH_OFFSET) & Self::INLINK_ADDR_CH_MASK)
        });
    }
}

/// IN_LINK_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `inlinkaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkAddrCh1Val {
    bits: u32,
}

impl InLinkAddrCh1Val {
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

    /// Read **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_addr_ch(&self) -> u32 {
        (self.bits & InLinkAddrCh1::INLINK_ADDR_CH_MASK) >> InLinkAddrCh1::INLINK_ADDR_CH_OFFSET
    }

    /// Set **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn with_inlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InLinkAddrCh1::INLINK_ADDR_CH_MASK)
                | ((value << InLinkAddrCh1::INLINK_ADDR_CH_OFFSET)
                    & InLinkAddrCh1::INLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for InLinkAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InLinkAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkAddrCh1Val {
        InLinkAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkAddrCh1Val) -> InLinkAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IN_LINK_ADDR_CH2 Register
///
/// Link descriptor configure of Rx channel 0
///
/// Address offset: 0x03b4
///
/// ## Fields
///
/// - **INLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first inlink descriptor's address.
#[repr(transparent)]
pub struct InLinkAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InLinkAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InLinkAddrCh2 {
    // ======== Field Constants ========

    /// **INLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const INLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::INLINK_ADDR_CH_MASK) >> Self::INLINK_ADDR_CH_OFFSET
    }

    /// Write **INLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn write_inlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INLINK_ADDR_CH_MASK)
                | ((value << Self::INLINK_ADDR_CH_OFFSET) & Self::INLINK_ADDR_CH_MASK)
        });
    }
}

/// IN_LINK_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `inlinkaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inlinkaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InLinkAddrCh2Val {
    bits: u32,
}

impl InLinkAddrCh2Val {
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

    /// Read **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn inlink_addr_ch(&self) -> u32 {
        (self.bits & InLinkAddrCh2::INLINK_ADDR_CH_MASK) >> InLinkAddrCh2::INLINK_ADDR_CH_OFFSET
    }

    /// Set **INLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub const fn with_inlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InLinkAddrCh2::INLINK_ADDR_CH_MASK)
                | ((value << InLinkAddrCh2::INLINK_ADDR_CH_OFFSET)
                    & InLinkAddrCh2::INLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for InLinkAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InLinkAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InLinkAddrCh2Val {
        InLinkAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InLinkAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InLinkAddrCh2Val) -> InLinkAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_ADDR_CH0 Register
///
/// Link descriptor configure of Tx channel 0
///
/// Address offset: 0x03b8
///
/// ## Fields
///
/// - **OUTLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first outlink descriptor's address.
#[repr(transparent)]
pub struct OutLinkAddrCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkAddrCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkAddrCh0 {
    // ======== Field Constants ========

    /// **OUTLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_ADDR_CH_MASK) >> Self::OUTLINK_ADDR_CH_OFFSET
    }

    /// Write **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn write_outlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTLINK_ADDR_CH_MASK)
                | ((value << Self::OUTLINK_ADDR_CH_OFFSET) & Self::OUTLINK_ADDR_CH_MASK)
        });
    }
}

/// OUT_LINK_ADDR_CH0 value type for batch read/modify/write operations
///
/// Use `outlinkaddrch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkaddrch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkAddrCh0Val {
    bits: u32,
}

impl OutLinkAddrCh0Val {
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

    /// Read **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_addr_ch(&self) -> u32 {
        (self.bits & OutLinkAddrCh0::OUTLINK_ADDR_CH_MASK) >> OutLinkAddrCh0::OUTLINK_ADDR_CH_OFFSET
    }

    /// Set **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn with_outlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutLinkAddrCh0::OUTLINK_ADDR_CH_MASK)
                | ((value << OutLinkAddrCh0::OUTLINK_ADDR_CH_OFFSET)
                    & OutLinkAddrCh0::OUTLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for OutLinkAddrCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutLinkAddrCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkAddrCh0Val {
        OutLinkAddrCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkAddrCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkAddrCh0Val) -> OutLinkAddrCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_ADDR_CH1 Register
///
/// Link descriptor configure of Tx channel 0
///
/// Address offset: 0x03bc
///
/// ## Fields
///
/// - **OUTLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first outlink descriptor's address.
#[repr(transparent)]
pub struct OutLinkAddrCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkAddrCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkAddrCh1 {
    // ======== Field Constants ========

    /// **OUTLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_ADDR_CH_MASK) >> Self::OUTLINK_ADDR_CH_OFFSET
    }

    /// Write **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn write_outlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTLINK_ADDR_CH_MASK)
                | ((value << Self::OUTLINK_ADDR_CH_OFFSET) & Self::OUTLINK_ADDR_CH_MASK)
        });
    }
}

/// OUT_LINK_ADDR_CH1 value type for batch read/modify/write operations
///
/// Use `outlinkaddrch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkaddrch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkAddrCh1Val {
    bits: u32,
}

impl OutLinkAddrCh1Val {
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

    /// Read **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_addr_ch(&self) -> u32 {
        (self.bits & OutLinkAddrCh1::OUTLINK_ADDR_CH_MASK) >> OutLinkAddrCh1::OUTLINK_ADDR_CH_OFFSET
    }

    /// Set **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn with_outlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutLinkAddrCh1::OUTLINK_ADDR_CH_MASK)
                | ((value << OutLinkAddrCh1::OUTLINK_ADDR_CH_OFFSET)
                    & OutLinkAddrCh1::OUTLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for OutLinkAddrCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutLinkAddrCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkAddrCh1Val {
        OutLinkAddrCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkAddrCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkAddrCh1Val) -> OutLinkAddrCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_LINK_ADDR_CH2 Register
///
/// Link descriptor configure of Tx channel 0
///
/// Address offset: 0x03c0
///
/// ## Fields
///
/// - **OUTLINK_ADDR_CH** `[31:0]` (RW)
///
///   This register stores the 32 least significant bits of the first outlink descriptor's address.
#[repr(transparent)]
pub struct OutLinkAddrCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutLinkAddrCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutLinkAddrCh2 {
    // ======== Field Constants ========

    /// **OUTLINK_ADDR_CH**
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OUTLINK_ADDR_CH_OFFSET: u32 = 0;
    pub const OUTLINK_ADDR_CH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OUTLINK_ADDR_CH_MASK) >> Self::OUTLINK_ADDR_CH_OFFSET
    }

    /// Write **OUTLINK_ADDR_CH** `[31:0]` (RW)
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub fn write_outlink_addr_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTLINK_ADDR_CH_MASK)
                | ((value << Self::OUTLINK_ADDR_CH_OFFSET) & Self::OUTLINK_ADDR_CH_MASK)
        });
    }
}

/// OUT_LINK_ADDR_CH2 value type for batch read/modify/write operations
///
/// Use `outlinkaddrch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outlinkaddrch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutLinkAddrCh2Val {
    bits: u32,
}

impl OutLinkAddrCh2Val {
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

    /// Read **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn outlink_addr_ch(&self) -> u32 {
        (self.bits & OutLinkAddrCh2::OUTLINK_ADDR_CH_MASK) >> OutLinkAddrCh2::OUTLINK_ADDR_CH_OFFSET
    }

    /// Set **OUTLINK_ADDR_CH** `[31:0]`
    ///
    /// This register stores the 32 least significant bits of the first outlink descriptor's address.
    #[inline(always)]
    pub const fn with_outlink_addr_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutLinkAddrCh2::OUTLINK_ADDR_CH_MASK)
                | ((value << OutLinkAddrCh2::OUTLINK_ADDR_CH_OFFSET)
                    & OutLinkAddrCh2::OUTLINK_ADDR_CH_MASK),
        }
    }
}

impl Default for OutLinkAddrCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutLinkAddrCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutLinkAddrCh2Val {
        OutLinkAddrCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutLinkAddrCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutLinkAddrCh2Val) -> OutLinkAddrCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR_MEM_START_ADDR Register
///
/// The start address of accessible address space.
///
/// Address offset: 0x03c4
///
/// ## Fields
///
/// - **ACCESS_INTR_MEM_START_ADDR** `[31:0]` (RW)
///
///   The start address of accessible address space.
#[repr(transparent)]
pub struct IntrMemStartAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrMemStartAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrMemStartAddr {
    // ======== Field Constants ========

    /// **ACCESS_INTR_MEM_START_ADDR**
    ///
    /// The start address of accessible address space.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ACCESS_INTR_MEM_START_ADDR_OFFSET: u32 = 0;
    pub const ACCESS_INTR_MEM_START_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ACCESS_INTR_MEM_START_ADDR** `[31:0]` (RW)
    ///
    /// The start address of accessible address space.
    #[inline(always)]
    pub fn access_intr_mem_start_addr(&self) -> u32 {
        (self.value.read_bits() & Self::ACCESS_INTR_MEM_START_ADDR_MASK)
            >> Self::ACCESS_INTR_MEM_START_ADDR_OFFSET
    }

    /// Write **ACCESS_INTR_MEM_START_ADDR** `[31:0]` (RW)
    ///
    /// The start address of accessible address space.
    #[inline(always)]
    pub fn write_access_intr_mem_start_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ACCESS_INTR_MEM_START_ADDR_MASK)
                | ((value << Self::ACCESS_INTR_MEM_START_ADDR_OFFSET)
                    & Self::ACCESS_INTR_MEM_START_ADDR_MASK)
        });
    }
}

/// INTR_MEM_START_ADDR value type for batch read/modify/write operations
///
/// Use `intrmemstartaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrmemstartaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrMemStartAddrVal {
    bits: u32,
}

impl IntrMemStartAddrVal {
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

    /// Read **ACCESS_INTR_MEM_START_ADDR** `[31:0]`
    ///
    /// The start address of accessible address space.
    #[inline(always)]
    pub const fn access_intr_mem_start_addr(&self) -> u32 {
        (self.bits & IntrMemStartAddr::ACCESS_INTR_MEM_START_ADDR_MASK)
            >> IntrMemStartAddr::ACCESS_INTR_MEM_START_ADDR_OFFSET
    }

    /// Set **ACCESS_INTR_MEM_START_ADDR** `[31:0]`
    ///
    /// The start address of accessible address space.
    #[inline(always)]
    pub const fn with_access_intr_mem_start_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IntrMemStartAddr::ACCESS_INTR_MEM_START_ADDR_MASK)
                | ((value << IntrMemStartAddr::ACCESS_INTR_MEM_START_ADDR_OFFSET)
                    & IntrMemStartAddr::ACCESS_INTR_MEM_START_ADDR_MASK),
        }
    }
}

impl Default for IntrMemStartAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrMemStartAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrMemStartAddrVal {
        IntrMemStartAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrMemStartAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntrMemStartAddrVal) -> IntrMemStartAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR_MEM_END_ADDR Register
///
/// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
///
/// Address offset: 0x03c8
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ACCESS_INTR_MEM_END_ADDR** `[31:0]` (RW)
///
///   The end address of accessible address space. The access address beyond this range would lead to descriptor error.
#[repr(transparent)]
pub struct IntrMemEndAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrMemEndAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrMemEndAddr {
    // ======== Field Constants ========

    /// **ACCESS_INTR_MEM_END_ADDR**
    ///
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ACCESS_INTR_MEM_END_ADDR_OFFSET: u32 = 0;
    pub const ACCESS_INTR_MEM_END_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ACCESS_INTR_MEM_END_ADDR** `[31:0]` (RW)
    ///
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    #[inline(always)]
    pub fn access_intr_mem_end_addr(&self) -> u32 {
        (self.value.read_bits() & Self::ACCESS_INTR_MEM_END_ADDR_MASK)
            >> Self::ACCESS_INTR_MEM_END_ADDR_OFFSET
    }

    /// Write **ACCESS_INTR_MEM_END_ADDR** `[31:0]` (RW)
    ///
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    #[inline(always)]
    pub fn write_access_intr_mem_end_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ACCESS_INTR_MEM_END_ADDR_MASK)
                | ((value << Self::ACCESS_INTR_MEM_END_ADDR_OFFSET)
                    & Self::ACCESS_INTR_MEM_END_ADDR_MASK)
        });
    }
}

/// INTR_MEM_END_ADDR value type for batch read/modify/write operations
///
/// Use `intrmemendaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrmemendaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrMemEndAddrVal {
    bits: u32,
}

impl IntrMemEndAddrVal {
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

    /// Read **ACCESS_INTR_MEM_END_ADDR** `[31:0]`
    ///
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    #[inline(always)]
    pub const fn access_intr_mem_end_addr(&self) -> u32 {
        (self.bits & IntrMemEndAddr::ACCESS_INTR_MEM_END_ADDR_MASK)
            >> IntrMemEndAddr::ACCESS_INTR_MEM_END_ADDR_OFFSET
    }

    /// Set **ACCESS_INTR_MEM_END_ADDR** `[31:0]`
    ///
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    #[inline(always)]
    pub const fn with_access_intr_mem_end_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IntrMemEndAddr::ACCESS_INTR_MEM_END_ADDR_MASK)
                | ((value << IntrMemEndAddr::ACCESS_INTR_MEM_END_ADDR_OFFSET)
                    & IntrMemEndAddr::ACCESS_INTR_MEM_END_ADDR_MASK),
        }
    }
}

impl Default for IntrMemEndAddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IntrMemEndAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrMemEndAddrVal {
        IntrMemEndAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrMemEndAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntrMemEndAddrVal) -> IntrMemEndAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ARB_TIMEOUT_TX Register
///
/// This retister is used to config arbiter time slice for tx dir
///
/// Address offset: 0x03cc
///
/// ## Fields
///
/// - **ARB_TIMEOUT_TX** `[15:0]` (RW)
///
///   This register is used to config arbiter time out value
#[repr(transparent)]
pub struct ArbTimeoutTx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ArbTimeoutTx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ArbTimeoutTx {
    // ======== Field Constants ========

    /// **ARB_TIMEOUT_TX**
    ///
    /// This register is used to config arbiter time out value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ARB_TIMEOUT_TX_OFFSET: u32 = 0;
    pub const ARB_TIMEOUT_TX_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ARB_TIMEOUT_TX** `[15:0]` (RW)
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub fn arb_timeout_tx(&self) -> u32 {
        (self.value.read_bits() & Self::ARB_TIMEOUT_TX_MASK) >> Self::ARB_TIMEOUT_TX_OFFSET
    }

    /// Write **ARB_TIMEOUT_TX** `[15:0]` (RW)
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub fn write_arb_timeout_tx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARB_TIMEOUT_TX_MASK)
                | ((value << Self::ARB_TIMEOUT_TX_OFFSET) & Self::ARB_TIMEOUT_TX_MASK)
        });
    }
}

/// ARB_TIMEOUT_TX value type for batch read/modify/write operations
///
/// Use `arbtimeouttx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arbtimeouttx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArbTimeoutTxVal {
    bits: u32,
}

impl ArbTimeoutTxVal {
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

    /// Read **ARB_TIMEOUT_TX** `[15:0]`
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> u32 {
        (self.bits & ArbTimeoutTx::ARB_TIMEOUT_TX_MASK) >> ArbTimeoutTx::ARB_TIMEOUT_TX_OFFSET
    }

    /// Set **ARB_TIMEOUT_TX** `[15:0]`
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub const fn with_arb_timeout_tx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ArbTimeoutTx::ARB_TIMEOUT_TX_MASK)
                | ((value << ArbTimeoutTx::ARB_TIMEOUT_TX_OFFSET)
                    & ArbTimeoutTx::ARB_TIMEOUT_TX_MASK),
        }
    }
}

impl Default for ArbTimeoutTxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ArbTimeoutTx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArbTimeoutTxVal {
        ArbTimeoutTxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ArbTimeoutTxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ArbTimeoutTxVal) -> ArbTimeoutTxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ARB_TIMEOUT_RX Register
///
/// This retister is used to config arbiter time slice for rx dir
///
/// Address offset: 0x03d0
///
/// ## Fields
///
/// - **ARB_TIMEOUT_RX** `[15:0]` (RW)
///
///   This register is used to config arbiter time out value
#[repr(transparent)]
pub struct ArbTimeoutRx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ArbTimeoutRx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ArbTimeoutRx {
    // ======== Field Constants ========

    /// **ARB_TIMEOUT_RX**
    ///
    /// This register is used to config arbiter time out value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ARB_TIMEOUT_RX_OFFSET: u32 = 0;
    pub const ARB_TIMEOUT_RX_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ARB_TIMEOUT_RX** `[15:0]` (RW)
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub fn arb_timeout_rx(&self) -> u32 {
        (self.value.read_bits() & Self::ARB_TIMEOUT_RX_MASK) >> Self::ARB_TIMEOUT_RX_OFFSET
    }

    /// Write **ARB_TIMEOUT_RX** `[15:0]` (RW)
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub fn write_arb_timeout_rx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARB_TIMEOUT_RX_MASK)
                | ((value << Self::ARB_TIMEOUT_RX_OFFSET) & Self::ARB_TIMEOUT_RX_MASK)
        });
    }
}

/// ARB_TIMEOUT_RX value type for batch read/modify/write operations
///
/// Use `arbtimeoutrx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arbtimeoutrx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArbTimeoutRxVal {
    bits: u32,
}

impl ArbTimeoutRxVal {
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

    /// Read **ARB_TIMEOUT_RX** `[15:0]`
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> u32 {
        (self.bits & ArbTimeoutRx::ARB_TIMEOUT_RX_MASK) >> ArbTimeoutRx::ARB_TIMEOUT_RX_OFFSET
    }

    /// Set **ARB_TIMEOUT_RX** `[15:0]`
    ///
    /// This register is used to config arbiter time out value
    #[inline(always)]
    pub const fn with_arb_timeout_rx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ArbTimeoutRx::ARB_TIMEOUT_RX_MASK)
                | ((value << ArbTimeoutRx::ARB_TIMEOUT_RX_OFFSET)
                    & ArbTimeoutRx::ARB_TIMEOUT_RX_MASK),
        }
    }
}

impl Default for ArbTimeoutRxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ArbTimeoutRx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArbTimeoutRxVal {
        ArbTimeoutRxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ArbTimeoutRxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ArbTimeoutRxVal) -> ArbTimeoutRxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WEIGHT_EN_TX Register
///
/// This register is used to config arbiter weigh function to on or off for tx dir
///
/// Address offset: 0x03d4
///
/// ## Fields
///
/// - **WEIGHT_EN_TX** `[0]` (RW)
///
///   This register is used to config arbiter weight function off/on
#[repr(transparent)]
pub struct WeightEnTx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WeightEnTx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WeightEnTx {
    // ======== Field Constants ========

    /// **WEIGHT_EN_TX**
    ///
    /// This register is used to config arbiter weight function off/on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const WEIGHT_EN_TX_OFFSET: u32 = 0;
    pub const WEIGHT_EN_TX_MASK: u32 = 0x00000001;
    pub const WEIGHT_EN_TX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WEIGHT_EN_TX** `[0]` (RW)
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub fn weight_en_tx(&self) -> bool {
        (self.value.read_bits() & Self::WEIGHT_EN_TX_MASK) != 0
    }

    /// Set WEIGHT_EN_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_weight_en_tx(&self) {
        self.value.modify_bits(|r| r | Self::WEIGHT_EN_TX_MASK);
    }

    /// Clear WEIGHT_EN_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_weight_en_tx(&self) {
        self.value.modify_bits(|r| r & !Self::WEIGHT_EN_TX_MASK);
    }
}

/// WEIGHT_EN_TX value type for batch read/modify/write operations
///
/// Use `weightentx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `weightentx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WeightEnTxVal {
    bits: u32,
}

impl WeightEnTxVal {
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

    /// Read **WEIGHT_EN_TX** `[0:0]`
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> bool {
        (self.bits & WeightEnTx::WEIGHT_EN_TX_MASK) != 0
    }

    /// Set **WEIGHT_EN_TX** `[0:0]`
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub const fn with_weight_en_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WeightEnTx::WEIGHT_EN_TX_MASK
            } else {
                self.bits & !WeightEnTx::WEIGHT_EN_TX_MASK
            },
        }
    }
}

impl Default for WeightEnTxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl WeightEnTx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WeightEnTxVal {
        WeightEnTxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WeightEnTxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WeightEnTxVal) -> WeightEnTxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WEIGHT_EN_RX Register
///
/// This register is used to config arbiter weigh function to on or off for rx dir
///
/// Address offset: 0x03d8
///
/// ## Fields
///
/// - **WEIGHT_EN_RX** `[0]` (RW)
///
///   This register is used to config arbiter weight function off/on
#[repr(transparent)]
pub struct WeightEnRx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WeightEnRx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WeightEnRx {
    // ======== Field Constants ========

    /// **WEIGHT_EN_RX**
    ///
    /// This register is used to config arbiter weight function off/on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const WEIGHT_EN_RX_OFFSET: u32 = 0;
    pub const WEIGHT_EN_RX_MASK: u32 = 0x00000001;
    pub const WEIGHT_EN_RX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WEIGHT_EN_RX** `[0]` (RW)
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub fn weight_en_rx(&self) -> bool {
        (self.value.read_bits() & Self::WEIGHT_EN_RX_MASK) != 0
    }

    /// Set WEIGHT_EN_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_weight_en_rx(&self) {
        self.value.modify_bits(|r| r | Self::WEIGHT_EN_RX_MASK);
    }

    /// Clear WEIGHT_EN_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_weight_en_rx(&self) {
        self.value.modify_bits(|r| r & !Self::WEIGHT_EN_RX_MASK);
    }
}

/// WEIGHT_EN_RX value type for batch read/modify/write operations
///
/// Use `weightenrx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `weightenrx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WeightEnRxVal {
    bits: u32,
}

impl WeightEnRxVal {
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

    /// Read **WEIGHT_EN_RX** `[0:0]`
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> bool {
        (self.bits & WeightEnRx::WEIGHT_EN_RX_MASK) != 0
    }

    /// Set **WEIGHT_EN_RX** `[0:0]`
    ///
    /// This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub const fn with_weight_en_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WeightEnRx::WEIGHT_EN_RX_MASK
            } else {
                self.bits & !WeightEnRx::WEIGHT_EN_RX_MASK
            },
        }
    }
}

impl Default for WeightEnRxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl WeightEnRx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WeightEnRxVal {
        WeightEnRxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WeightEnRxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WeightEnRxVal) -> WeightEnRxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Raw status interrupt of channel 0
    pub in_int_raw_ch0: InIntRawCh0,
    /// 0x0004
    /// Masked interrupt of channel 0
    pub in_int_st_ch0: InIntStCh0,
    /// 0x0008
    /// Interrupt enable bits of channel 0
    pub in_int_ena_ch0: InIntEnaCh0,
    /// 0x000c
    /// Interrupt clear bits of channel 0
    pub in_int_clr_ch0: InIntClrCh0,
    /// 0x0010
    /// Raw status interrupt of channel 0
    pub in_int_raw_ch1: InIntRawCh1,
    /// 0x0014
    /// Masked interrupt of channel 0
    pub in_int_st_ch1: InIntStCh1,
    /// 0x0018
    /// Interrupt enable bits of channel 0
    pub in_int_ena_ch1: InIntEnaCh1,
    /// 0x001c
    /// Interrupt clear bits of channel 0
    pub in_int_clr_ch1: InIntClrCh1,
    /// 0x0020
    /// Raw status interrupt of channel 0
    pub in_int_raw_ch2: InIntRawCh2,
    /// 0x0024
    /// Masked interrupt of channel 0
    pub in_int_st_ch2: InIntStCh2,
    /// 0x0028
    /// Interrupt enable bits of channel 0
    pub in_int_ena_ch2: InIntEnaCh2,
    /// 0x002c
    /// Interrupt clear bits of channel 0
    pub in_int_clr_ch2: InIntClrCh2,
    /// 0x0030
    /// Raw status interrupt of channel 0
    pub out_int_raw_ch0: OutIntRawCh0,
    /// 0x0034
    /// Masked interrupt of channel 0
    pub out_int_st_ch0: OutIntStCh0,
    /// 0x0038
    /// Interrupt enable bits of channel 0
    pub out_int_ena_ch0: OutIntEnaCh0,
    /// 0x003c
    /// Interrupt clear bits of channel 0
    pub out_int_clr_ch0: OutIntClrCh0,
    /// 0x0040
    /// Raw status interrupt of channel 0
    pub out_int_raw_ch1: OutIntRawCh1,
    /// 0x0044
    /// Masked interrupt of channel 0
    pub out_int_st_ch1: OutIntStCh1,
    /// 0x0048
    /// Interrupt enable bits of channel 0
    pub out_int_ena_ch1: OutIntEnaCh1,
    /// 0x004c
    /// Interrupt clear bits of channel 0
    pub out_int_clr_ch1: OutIntClrCh1,
    /// 0x0050
    /// Raw status interrupt of channel 0
    pub out_int_raw_ch2: OutIntRawCh2,
    /// 0x0054
    /// Masked interrupt of channel 0
    pub out_int_st_ch2: OutIntStCh2,
    /// 0x0058
    /// Interrupt enable bits of channel 0
    pub out_int_ena_ch2: OutIntEnaCh2,
    /// 0x005c
    /// Interrupt clear bits of channel 0
    pub out_int_clr_ch2: OutIntClrCh2,
    /// 0x0060
    /// reserved
    pub ahb_test: AhbTest,
    /// 0x0064
    /// MISC register
    pub misc_conf: MiscConf,
    /// 0x0068
    /// Version control register
    pub date: Date,
    _reserved0: [u8; 4],
    /// 0x0070
    /// Configure 0 register of Rx channel 0
    pub in_conf0_ch0: InConf0Ch0,
    /// 0x0074
    /// Configure 1 register of Rx channel 0
    pub in_conf1_ch0: InConf1Ch0,
    /// 0x0078
    /// Receive FIFO status of Rx channel 0
    pub infifo_status_ch0: InfifoStatusCh0,
    /// 0x007c
    /// Pop control register of Rx channel 0
    pub in_pop_ch0: InPopCh0,
    /// 0x0080
    /// Link descriptor configure and control register of Rx channel 0
    pub in_link_ch0: InLinkCh0,
    /// 0x0084
    /// Receive status of Rx channel 0
    pub in_state_ch0: InStateCh0,
    /// 0x0088
    /// Inlink descriptor address when EOF occurs of Rx channel 0
    pub in_suc_eof_des_addr_ch0: InSucEofDesAddrCh0,
    /// 0x008c
    /// Inlink descriptor address when errors occur of Rx channel 0
    pub in_err_eof_des_addr_ch0: InErrEofDesAddrCh0,
    /// 0x0090
    /// Current inlink descriptor address of Rx channel 0
    pub in_dscr_ch0: InDscrCh0,
    /// 0x0094
    /// The last inlink descriptor address of Rx channel 0
    pub in_dscr_bf0_ch0: InDscrBf0Ch0,
    /// 0x0098
    /// The second-to-last inlink descriptor address of Rx channel 0
    pub in_dscr_bf1_ch0: InDscrBf1Ch0,
    /// 0x009c
    /// Priority register of Rx channel 0
    pub in_pri_ch0: InPriCh0,
    /// 0x00a0
    /// Peripheral selection of Rx channel 0
    pub in_peri_sel_ch0: InPeriSelCh0,
    _reserved1: [u8; 44],
    /// 0x00d0
    /// Configure 0 register of Tx channel 0
    pub out_conf0_ch0: OutConf0Ch0,
    /// 0x00d4
    /// Configure 1 register of Tx channel 0
    pub out_conf1_ch0: OutConf1Ch0,
    /// 0x00d8
    /// Transmit FIFO status of Tx channel 0
    pub outfifo_status_ch0: OutfifoStatusCh0,
    /// 0x00dc
    /// Push control register of Rx channel 0
    pub out_push_ch0: OutPushCh0,
    /// 0x00e0
    /// Link descriptor configure and control register of Tx channel 0
    pub out_link_ch0: OutLinkCh0,
    /// 0x00e4
    /// Transmit status of Tx channel 0
    pub out_state_ch0: OutStateCh0,
    /// 0x00e8
    /// Outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_des_addr_ch0: OutEofDesAddrCh0,
    /// 0x00ec
    /// The last outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_bfr_des_addr_ch0: OutEofBfrDesAddrCh0,
    /// 0x00f0
    /// Current inlink descriptor address of Tx channel 0
    pub out_dscr_ch0: OutDscrCh0,
    /// 0x00f4
    /// The last inlink descriptor address of Tx channel 0
    pub out_dscr_bf0_ch0: OutDscrBf0Ch0,
    /// 0x00f8
    /// The second-to-last inlink descriptor address of Tx channel 0
    pub out_dscr_bf1_ch0: OutDscrBf1Ch0,
    /// 0x00fc
    /// Priority register of Tx channel 0.
    pub out_pri_ch0: OutPriCh0,
    /// 0x0100
    /// Peripheral selection of Tx channel 0
    pub out_peri_sel_ch0: OutPeriSelCh0,
    _reserved2: [u8; 44],
    /// 0x0130
    /// Configure 0 register of Rx channel 0
    pub in_conf0_ch1: InConf0Ch1,
    /// 0x0134
    /// Configure 1 register of Rx channel 0
    pub in_conf1_ch1: InConf1Ch1,
    /// 0x0138
    /// Receive FIFO status of Rx channel 0
    pub infifo_status_ch1: InfifoStatusCh1,
    /// 0x013c
    /// Pop control register of Rx channel 0
    pub in_pop_ch1: InPopCh1,
    /// 0x0140
    /// Link descriptor configure and control register of Rx channel 0
    pub in_link_ch1: InLinkCh1,
    /// 0x0144
    /// Receive status of Rx channel 0
    pub in_state_ch1: InStateCh1,
    /// 0x0148
    /// Inlink descriptor address when EOF occurs of Rx channel 0
    pub in_suc_eof_des_addr_ch1: InSucEofDesAddrCh1,
    /// 0x014c
    /// Inlink descriptor address when errors occur of Rx channel 0
    pub in_err_eof_des_addr_ch1: InErrEofDesAddrCh1,
    /// 0x0150
    /// Current inlink descriptor address of Rx channel 0
    pub in_dscr_ch1: InDscrCh1,
    /// 0x0154
    /// The last inlink descriptor address of Rx channel 0
    pub in_dscr_bf0_ch1: InDscrBf0Ch1,
    /// 0x0158
    /// The second-to-last inlink descriptor address of Rx channel 0
    pub in_dscr_bf1_ch1: InDscrBf1Ch1,
    /// 0x015c
    /// Priority register of Rx channel 0
    pub in_pri_ch1: InPriCh1,
    /// 0x0160
    /// Peripheral selection of Rx channel 0
    pub in_peri_sel_ch1: InPeriSelCh1,
    _reserved3: [u8; 44],
    /// 0x0190
    /// Configure 0 register of Tx channel 1
    pub out_conf0_ch1: OutConf0Ch1,
    /// 0x0194
    /// Configure 1 register of Tx channel 0
    pub out_conf1_ch1: OutConf1Ch1,
    /// 0x0198
    /// Transmit FIFO status of Tx channel 0
    pub outfifo_status_ch1: OutfifoStatusCh1,
    /// 0x019c
    /// Push control register of Rx channel 0
    pub out_push_ch1: OutPushCh1,
    /// 0x01a0
    /// Link descriptor configure and control register of Tx channel 0
    pub out_link_ch1: OutLinkCh1,
    /// 0x01a4
    /// Transmit status of Tx channel 0
    pub out_state_ch1: OutStateCh1,
    /// 0x01a8
    /// Outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_des_addr_ch1: OutEofDesAddrCh1,
    /// 0x01ac
    /// The last outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_bfr_des_addr_ch1: OutEofBfrDesAddrCh1,
    /// 0x01b0
    /// Current inlink descriptor address of Tx channel 0
    pub out_dscr_ch1: OutDscrCh1,
    /// 0x01b4
    /// The last inlink descriptor address of Tx channel 0
    pub out_dscr_bf0_ch1: OutDscrBf0Ch1,
    /// 0x01b8
    /// The second-to-last inlink descriptor address of Tx channel 0
    pub out_dscr_bf1_ch1: OutDscrBf1Ch1,
    /// 0x01bc
    /// Priority register of Tx channel 0.
    pub out_pri_ch1: OutPriCh1,
    /// 0x01c0
    /// Peripheral selection of Tx channel 0
    pub out_peri_sel_ch1: OutPeriSelCh1,
    _reserved4: [u8; 44],
    /// 0x01f0
    /// Configure 0 register of Rx channel 0
    pub in_conf0_ch2: InConf0Ch2,
    /// 0x01f4
    /// Configure 1 register of Rx channel 0
    pub in_conf1_ch2: InConf1Ch2,
    /// 0x01f8
    /// Receive FIFO status of Rx channel 0
    pub infifo_status_ch2: InfifoStatusCh2,
    /// 0x01fc
    /// Pop control register of Rx channel 0
    pub in_pop_ch2: InPopCh2,
    /// 0x0200
    /// Link descriptor configure and control register of Rx channel 0
    pub in_link_ch2: InLinkCh2,
    /// 0x0204
    /// Receive status of Rx channel 0
    pub in_state_ch2: InStateCh2,
    /// 0x0208
    /// Inlink descriptor address when EOF occurs of Rx channel 0
    pub in_suc_eof_des_addr_ch2: InSucEofDesAddrCh2,
    /// 0x020c
    /// Inlink descriptor address when errors occur of Rx channel 0
    pub in_err_eof_des_addr_ch2: InErrEofDesAddrCh2,
    /// 0x0210
    /// Current inlink descriptor address of Rx channel 0
    pub in_dscr_ch2: InDscrCh2,
    /// 0x0214
    /// The last inlink descriptor address of Rx channel 0
    pub in_dscr_bf0_ch2: InDscrBf0Ch2,
    /// 0x0218
    /// The second-to-last inlink descriptor address of Rx channel 0
    pub in_dscr_bf1_ch2: InDscrBf1Ch2,
    /// 0x021c
    /// Priority register of Rx channel 0
    pub in_pri_ch2: InPriCh2,
    /// 0x0220
    /// Peripheral selection of Rx channel 0
    pub in_peri_sel_ch2: InPeriSelCh2,
    _reserved5: [u8; 44],
    /// 0x0250
    /// Configure 0 register of Tx channel 1
    pub out_conf0_ch2: OutConf0Ch2,
    /// 0x0254
    /// Configure 1 register of Tx channel 0
    pub out_conf1_ch2: OutConf1Ch2,
    /// 0x0258
    /// Transmit FIFO status of Tx channel 0
    pub outfifo_status_ch2: OutfifoStatusCh2,
    /// 0x025c
    /// Push control register of Rx channel 0
    pub out_push_ch2: OutPushCh2,
    /// 0x0260
    /// Link descriptor configure and control register of Tx channel 0
    pub out_link_ch2: OutLinkCh2,
    /// 0x0264
    /// Transmit status of Tx channel 0
    pub out_state_ch2: OutStateCh2,
    /// 0x0268
    /// Outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_des_addr_ch2: OutEofDesAddrCh2,
    /// 0x026c
    /// The last outlink descriptor address when EOF occurs of Tx channel 0
    pub out_eof_bfr_des_addr_ch2: OutEofBfrDesAddrCh2,
    /// 0x0270
    /// Current inlink descriptor address of Tx channel 0
    pub out_dscr_ch2: OutDscrCh2,
    /// 0x0274
    /// The last inlink descriptor address of Tx channel 0
    pub out_dscr_bf0_ch2: OutDscrBf0Ch2,
    /// 0x0278
    /// The second-to-last inlink descriptor address of Tx channel 0
    pub out_dscr_bf1_ch2: OutDscrBf1Ch2,
    /// 0x027c
    /// Priority register of Tx channel 0.
    pub out_pri_ch2: OutPriCh2,
    /// 0x0280
    /// Peripheral selection of Tx channel 0
    pub out_peri_sel_ch2: OutPeriSelCh2,
    _reserved6: [u8; 56],
    /// 0x02bc
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub out_crc_init_data_ch0: OutCrcInitDataCh0,
    /// 0x02c0
    /// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub tx_crc_width_ch0: TxCrcWidthCh0,
    /// 0x02c4
    /// This register is used to clear ch0 crc result
    pub out_crc_clear_ch0: OutCrcClearCh0,
    /// 0x02c8
    /// This register is used to store ch0 crc result
    pub out_crc_final_result_ch0: OutCrcFinalResultCh0,
    /// 0x02cc
    /// This resister is used to config ch0 crc en for every bit
    pub tx_crc_en_wr_data_ch0: TxCrcEnWrDataCh0,
    /// 0x02d0
    /// This register is used to config ch0 crc en addr
    pub tx_crc_en_addr_ch0: TxCrcEnAddrCh0,
    /// 0x02d4
    /// This register is used to config crc data_8bit en
    pub tx_crc_data_en_wr_data_ch0: TxCrcDataEnWrDataCh0,
    /// 0x02d8
    /// This register is used to config addr of crc data_8bit en
    pub tx_crc_data_en_addr_ch0: TxCrcDataEnAddrCh0,
    /// 0x02dc
    /// This register is used to config ch0 arbiter weigh
    pub tx_ch_arb_weigh_ch0: TxChArbWeighCh0,
    /// 0x02e0
    /// This register is used to config off or on weigh optimization
    pub tx_arb_weigh_opt_dir_ch0: TxArbWeighOptDirCh0,
    /// 0x02e4
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub out_crc_init_data_ch1: OutCrcInitDataCh1,
    /// 0x02e8
    /// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub tx_crc_width_ch1: TxCrcWidthCh1,
    /// 0x02ec
    /// This register is used to clear ch0 crc result
    pub out_crc_clear_ch1: OutCrcClearCh1,
    /// 0x02f0
    /// This register is used to store ch0 crc result
    pub out_crc_final_result_ch1: OutCrcFinalResultCh1,
    /// 0x02f4
    /// This resister is used to config ch0 crc en for every bit
    pub tx_crc_en_wr_data_ch1: TxCrcEnWrDataCh1,
    /// 0x02f8
    /// This register is used to config ch0 crc en addr
    pub tx_crc_en_addr_ch1: TxCrcEnAddrCh1,
    /// 0x02fc
    /// This register is used to config crc data_8bit en
    pub tx_crc_data_en_wr_data_ch1: TxCrcDataEnWrDataCh1,
    /// 0x0300
    /// This register is used to config addr of crc data_8bit en
    pub tx_crc_data_en_addr_ch1: TxCrcDataEnAddrCh1,
    /// 0x0304
    /// This register is used to config ch0 arbiter weigh
    pub tx_ch_arb_weigh_ch1: TxChArbWeighCh1,
    /// 0x0308
    /// This register is used to config off or on weigh optimization
    pub tx_arb_weigh_opt_dir_ch1: TxArbWeighOptDirCh1,
    /// 0x030c
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub out_crc_init_data_ch2: OutCrcInitDataCh2,
    /// 0x0310
    /// This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub tx_crc_width_ch2: TxCrcWidthCh2,
    /// 0x0314
    /// This register is used to clear ch0 crc result
    pub out_crc_clear_ch2: OutCrcClearCh2,
    /// 0x0318
    /// This register is used to store ch0 crc result
    pub out_crc_final_result_ch2: OutCrcFinalResultCh2,
    /// 0x031c
    /// This resister is used to config ch0 crc en for every bit
    pub tx_crc_en_wr_data_ch2: TxCrcEnWrDataCh2,
    /// 0x0320
    /// This register is used to config ch0 crc en addr
    pub tx_crc_en_addr_ch2: TxCrcEnAddrCh2,
    /// 0x0324
    /// This register is used to config crc data_8bit en
    pub tx_crc_data_en_wr_data_ch2: TxCrcDataEnWrDataCh2,
    /// 0x0328
    /// This register is used to config addr of crc data_8bit en
    pub tx_crc_data_en_addr_ch2: TxCrcDataEnAddrCh2,
    /// 0x032c
    /// This register is used to config ch0 arbiter weigh
    pub tx_ch_arb_weigh_ch2: TxChArbWeighCh2,
    /// 0x0330
    /// This register is used to config off or on weigh optimization
    pub tx_arb_weigh_opt_dir_ch2: TxArbWeighOptDirCh2,
    /// 0x0334
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub in_crc_init_data_ch0: InCrcInitDataCh0,
    /// 0x0338
    /// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub rx_crc_width_ch0: RxCrcWidthCh0,
    /// 0x033c
    /// This register is used to clear ch0 crc result
    pub in_crc_clear_ch0: InCrcClearCh0,
    /// 0x0340
    /// This register is used to store ch0 crc result
    pub in_crc_final_result_ch0: InCrcFinalResultCh0,
    /// 0x0344
    /// This resister is used to config ch0 crc en for every bit
    pub rx_crc_en_wr_data_ch0: RxCrcEnWrDataCh0,
    /// 0x0348
    /// This register is used to config ch0 crc en addr
    pub rx_crc_en_addr_ch0: RxCrcEnAddrCh0,
    /// 0x034c
    /// This register is used to config crc data_8bit en
    pub rx_crc_data_en_wr_data_ch0: RxCrcDataEnWrDataCh0,
    /// 0x0350
    /// This register is used to config addr of crc data_8bit en
    pub rx_crc_data_en_addr_ch0: RxCrcDataEnAddrCh0,
    /// 0x0354
    /// This register is used to config ch0 arbiter weigh
    pub rx_ch_arb_weigh_ch0: RxChArbWeighCh0,
    /// 0x0358
    /// This register is used to config off or on weigh optimization
    pub rx_arb_weigh_opt_dir_ch0: RxArbWeighOptDirCh0,
    /// 0x035c
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub in_crc_init_data_ch1: InCrcInitDataCh1,
    /// 0x0360
    /// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub rx_crc_width_ch1: RxCrcWidthCh1,
    /// 0x0364
    /// This register is used to clear ch0 crc result
    pub in_crc_clear_ch1: InCrcClearCh1,
    /// 0x0368
    /// This register is used to store ch0 crc result
    pub in_crc_final_result_ch1: InCrcFinalResultCh1,
    /// 0x036c
    /// This resister is used to config ch0 crc en for every bit
    pub rx_crc_en_wr_data_ch1: RxCrcEnWrDataCh1,
    /// 0x0370
    /// This register is used to config ch0 crc en addr
    pub rx_crc_en_addr_ch1: RxCrcEnAddrCh1,
    /// 0x0374
    /// This register is used to config crc data_8bit en
    pub rx_crc_data_en_wr_data_ch1: RxCrcDataEnWrDataCh1,
    /// 0x0378
    /// This register is used to config addr of crc data_8bit en
    pub rx_crc_data_en_addr_ch1: RxCrcDataEnAddrCh1,
    /// 0x037c
    /// This register is used to config ch0 arbiter weigh
    pub rx_ch_arb_weigh_ch1: RxChArbWeighCh1,
    /// 0x0380
    /// This register is used to config off or on weigh optimization
    pub rx_arb_weigh_opt_dir_ch1: RxArbWeighOptDirCh1,
    /// 0x0384
    /// This register is used to config ch0 crc initial data(max 32 bit)
    pub in_crc_init_data_ch2: InCrcInitDataCh2,
    /// 0x0388
    /// This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32
    pub rx_crc_width_ch2: RxCrcWidthCh2,
    /// 0x038c
    /// This register is used to clear ch0 crc result
    pub in_crc_clear_ch2: InCrcClearCh2,
    /// 0x0390
    /// This register is used to store ch0 crc result
    pub in_crc_final_result_ch2: InCrcFinalResultCh2,
    /// 0x0394
    /// This resister is used to config ch0 crc en for every bit
    pub rx_crc_en_wr_data_ch2: RxCrcEnWrDataCh2,
    /// 0x0398
    /// This register is used to config ch0 crc en addr
    pub rx_crc_en_addr_ch2: RxCrcEnAddrCh2,
    /// 0x039c
    /// This register is used to config crc data_8bit en
    pub rx_crc_data_en_wr_data_ch2: RxCrcDataEnWrDataCh2,
    /// 0x03a0
    /// This register is used to config addr of crc data_8bit en
    pub rx_crc_data_en_addr_ch2: RxCrcDataEnAddrCh2,
    /// 0x03a4
    /// This register is used to config ch0 arbiter weigh
    pub rx_ch_arb_weigh_ch2: RxChArbWeighCh2,
    /// 0x03a8
    /// This register is used to config off or on weigh optimization
    pub rx_arb_weigh_opt_dir_ch2: RxArbWeighOptDirCh2,
    /// 0x03ac
    /// Link descriptor configure of Rx channel 0
    pub in_link_addr_ch0: InLinkAddrCh0,
    /// 0x03b0
    /// Link descriptor configure of Rx channel 0
    pub in_link_addr_ch1: InLinkAddrCh1,
    /// 0x03b4
    /// Link descriptor configure of Rx channel 0
    pub in_link_addr_ch2: InLinkAddrCh2,
    /// 0x03b8
    /// Link descriptor configure of Tx channel 0
    pub out_link_addr_ch0: OutLinkAddrCh0,
    /// 0x03bc
    /// Link descriptor configure of Tx channel 0
    pub out_link_addr_ch1: OutLinkAddrCh1,
    /// 0x03c0
    /// Link descriptor configure of Tx channel 0
    pub out_link_addr_ch2: OutLinkAddrCh2,
    /// 0x03c4
    /// The start address of accessible address space.
    pub intr_mem_start_addr: IntrMemStartAddr,
    /// 0x03c8
    /// The end address of accessible address space. The access address beyond this range would lead to descriptor error.
    pub intr_mem_end_addr: IntrMemEndAddr,
    /// 0x03cc
    /// This retister is used to config arbiter time slice for tx dir
    pub arb_timeout_tx: ArbTimeoutTx,
    /// 0x03d0
    /// This retister is used to config arbiter time slice for rx dir
    pub arb_timeout_rx: ArbTimeoutRx,
    /// 0x03d4
    /// This register is used to config arbiter weigh function to on or off for tx dir
    pub weight_en_tx: WeightEnTx,
    /// 0x03d8
    /// This register is used to config arbiter weigh function to on or off for rx dir
    pub weight_en_rx: WeightEnRx,
}
