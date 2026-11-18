//! PARL_IO
//! Parallel IO Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x500cf000;

/// RX_MODE_CFG Register
///
/// Parallel RX Sampling mode configuration register.
///
/// Address offset: 0x0000
/// Reset value: 0x00e00000
///
/// ## Fields
///
/// - **RX_SMP_MODE_SEL** `[31:30]` (RW)
///
///   Configures the rxd sampling mode.
///   2'b00: external level enable mode
///   2'b01: external pulse enable mode
///   2'b10: internal software enable mode
///
/// - **RX_PULSE_SUBMODE_SEL** `[29:27]` (RW)
///
///   Configures the rxd pulse sampling submode.
///   4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
///   4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
///   4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
///   4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
///   4'd4: positive pulse start(data bit included) && length end
///   4'd5: positive pulse start(data bit excluded) && length end
///
/// - **RX_EXT_EN_INV** `[26]` (RW)
///
///   Set this bit to invert the external enable signal.
///
/// - **RX_SW_EN** `[25]` (RW)
///
///   Set this bit to enable data sampling by software.
///
/// - **RX_EXT_EN_SEL** `[24:21]` (RW)
///
///   Configures rx external enable signal selection from IO PAD.
#[repr(transparent)]
pub struct RxModeCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxModeCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxModeCfg {
    // ======== Field Constants ========

    /// **RX_SMP_MODE_SEL**
    ///
    /// Configures the rxd sampling mode.
    /// 2'b00: external level enable mode
    /// 2'b01: external pulse enable mode
    /// 2'b10: internal software enable mode
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const RX_SMP_MODE_SEL_OFFSET: u32 = 30;
    pub const RX_SMP_MODE_SEL_MASK: u32 = 0xc0000000;

    /// **RX_PULSE_SUBMODE_SEL**
    ///
    /// Configures the rxd pulse sampling submode.
    /// 4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
    /// 4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
    /// 4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
    /// 4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
    /// 4'd4: positive pulse start(data bit included) && length end
    /// 4'd5: positive pulse start(data bit excluded) && length end
    ///
    /// Bits `[29:27]`, mask `0x38000000`
    pub const RX_PULSE_SUBMODE_SEL_OFFSET: u32 = 27;
    pub const RX_PULSE_SUBMODE_SEL_MASK: u32 = 0x38000000;

    /// **RX_EXT_EN_INV**
    ///
    /// Set this bit to invert the external enable signal.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const RX_EXT_EN_INV_OFFSET: u32 = 26;
    pub const RX_EXT_EN_INV_MASK: u32 = 0x04000000;
    pub const RX_EXT_EN_INV: u32 = 1 << 26;

    /// **RX_SW_EN**
    ///
    /// Set this bit to enable data sampling by software.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const RX_SW_EN_OFFSET: u32 = 25;
    pub const RX_SW_EN_MASK: u32 = 0x02000000;
    pub const RX_SW_EN: u32 = 1 << 25;

    /// **RX_EXT_EN_SEL**
    ///
    /// Configures rx external enable signal selection from IO PAD.
    ///
    /// Bits `[24:21]`, mask `0x01e00000`
    pub const RX_EXT_EN_SEL_OFFSET: u32 = 21;
    pub const RX_EXT_EN_SEL_MASK: u32 = 0x01e00000;

    // ======== Field Accessors ========

    /// **RX_SMP_MODE_SEL** `[31:30]` (RW)
    ///
    /// Configures the rxd sampling mode.
    /// 2'b00: external level enable mode
    /// 2'b01: external pulse enable mode
    /// 2'b10: internal software enable mode
    #[inline(always)]
    pub fn rx_smp_mode_sel(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SMP_MODE_SEL_MASK) >> Self::RX_SMP_MODE_SEL_OFFSET
    }

    /// Write **RX_SMP_MODE_SEL** `[31:30]` (RW)
    ///
    /// Configures the rxd sampling mode.
    /// 2'b00: external level enable mode
    /// 2'b01: external pulse enable mode
    /// 2'b10: internal software enable mode
    #[inline(always)]
    pub fn write_rx_smp_mode_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_SMP_MODE_SEL_MASK)
                | ((value << Self::RX_SMP_MODE_SEL_OFFSET) & Self::RX_SMP_MODE_SEL_MASK)
        });
    }

    /// **RX_PULSE_SUBMODE_SEL** `[29:27]` (RW)
    ///
    /// Configures the rxd pulse sampling submode.
    /// 4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
    /// 4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
    /// 4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
    /// 4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
    /// 4'd4: positive pulse start(data bit included) && length end
    /// 4'd5: positive pulse start(data bit excluded) && length end
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&self) -> u32 {
        (self.value.read_bits() & Self::RX_PULSE_SUBMODE_SEL_MASK)
            >> Self::RX_PULSE_SUBMODE_SEL_OFFSET
    }

    /// Write **RX_PULSE_SUBMODE_SEL** `[29:27]` (RW)
    ///
    /// Configures the rxd pulse sampling submode.
    /// 4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
    /// 4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
    /// 4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
    /// 4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
    /// 4'd4: positive pulse start(data bit included) && length end
    /// 4'd5: positive pulse start(data bit excluded) && length end
    #[inline(always)]
    pub fn write_rx_pulse_submode_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_PULSE_SUBMODE_SEL_MASK)
                | ((value << Self::RX_PULSE_SUBMODE_SEL_OFFSET) & Self::RX_PULSE_SUBMODE_SEL_MASK)
        });
    }

    /// **RX_EXT_EN_INV** `[26]` (RW)
    ///
    /// Set this bit to invert the external enable signal.
    #[inline(always)]
    pub fn rx_ext_en_inv(&self) -> bool {
        (self.value.read_bits() & Self::RX_EXT_EN_INV_MASK) != 0
    }

    /// Set RX_EXT_EN_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_ext_en_inv(&self) {
        self.value.modify_bits(|r| r | Self::RX_EXT_EN_INV_MASK);
    }

    /// Clear RX_EXT_EN_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_ext_en_inv(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EXT_EN_INV_MASK);
    }

    /// **RX_SW_EN** `[25]` (RW)
    ///
    /// Set this bit to enable data sampling by software.
    #[inline(always)]
    pub fn rx_sw_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_SW_EN_MASK) != 0
    }

    /// Set RX_SW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_sw_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_SW_EN_MASK);
    }

    /// Clear RX_SW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_sw_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_SW_EN_MASK);
    }

    /// **RX_EXT_EN_SEL** `[24:21]` (RW)
    ///
    /// Configures rx external enable signal selection from IO PAD.
    #[inline(always)]
    pub fn rx_ext_en_sel(&self) -> u32 {
        (self.value.read_bits() & Self::RX_EXT_EN_SEL_MASK) >> Self::RX_EXT_EN_SEL_OFFSET
    }

    /// Write **RX_EXT_EN_SEL** `[24:21]` (RW)
    ///
    /// Configures rx external enable signal selection from IO PAD.
    #[inline(always)]
    pub fn write_rx_ext_en_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_EXT_EN_SEL_MASK)
                | ((value << Self::RX_EXT_EN_SEL_OFFSET) & Self::RX_EXT_EN_SEL_MASK)
        });
    }
}

/// RX_MODE_CFG value type for batch read/modify/write operations
///
/// Use `rxmodecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxmodecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxModeCfgVal {
    bits: u32,
}

impl RxModeCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00e00000 }
    }

    /// Read **RX_SMP_MODE_SEL** `[31:30]`
    ///
    /// Configures the rxd sampling mode.
    /// 2'b00: external level enable mode
    /// 2'b01: external pulse enable mode
    /// 2'b10: internal software enable mode
    #[inline(always)]
    pub const fn rx_smp_mode_sel(&self) -> u32 {
        (self.bits & RxModeCfg::RX_SMP_MODE_SEL_MASK) >> RxModeCfg::RX_SMP_MODE_SEL_OFFSET
    }

    /// Read **RX_PULSE_SUBMODE_SEL** `[29:27]`
    ///
    /// Configures the rxd pulse sampling submode.
    /// 4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
    /// 4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
    /// 4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
    /// 4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
    /// 4'd4: positive pulse start(data bit included) && length end
    /// 4'd5: positive pulse start(data bit excluded) && length end
    #[inline(always)]
    pub const fn rx_pulse_submode_sel(&self) -> u32 {
        (self.bits & RxModeCfg::RX_PULSE_SUBMODE_SEL_MASK) >> RxModeCfg::RX_PULSE_SUBMODE_SEL_OFFSET
    }

    /// Read **RX_EXT_EN_INV** `[26:26]`
    ///
    /// Set this bit to invert the external enable signal.
    #[inline(always)]
    pub const fn rx_ext_en_inv(&self) -> bool {
        (self.bits & RxModeCfg::RX_EXT_EN_INV_MASK) != 0
    }

    /// Read **RX_SW_EN** `[25:25]`
    ///
    /// Set this bit to enable data sampling by software.
    #[inline(always)]
    pub const fn rx_sw_en(&self) -> bool {
        (self.bits & RxModeCfg::RX_SW_EN_MASK) != 0
    }

    /// Read **RX_EXT_EN_SEL** `[24:21]`
    ///
    /// Configures rx external enable signal selection from IO PAD.
    #[inline(always)]
    pub const fn rx_ext_en_sel(&self) -> u32 {
        (self.bits & RxModeCfg::RX_EXT_EN_SEL_MASK) >> RxModeCfg::RX_EXT_EN_SEL_OFFSET
    }

    /// Set **RX_SMP_MODE_SEL** `[31:30]`
    ///
    /// Configures the rxd sampling mode.
    /// 2'b00: external level enable mode
    /// 2'b01: external pulse enable mode
    /// 2'b10: internal software enable mode
    #[inline(always)]
    pub const fn with_rx_smp_mode_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxModeCfg::RX_SMP_MODE_SEL_MASK)
                | ((value << RxModeCfg::RX_SMP_MODE_SEL_OFFSET) & RxModeCfg::RX_SMP_MODE_SEL_MASK),
        }
    }

    /// Set **RX_PULSE_SUBMODE_SEL** `[29:27]`
    ///
    /// Configures the rxd pulse sampling submode.
    /// 4'd0: positive pulse start(data bit included) && positive pulse end(data bit included)
    /// 4'd1: positive pulse start(data bit included) && positive pulse end (data bit excluded)
    /// 4'd2: positive pulse start(data bit excluded) && positive pulse end (data bit included)
    /// 4'd3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded)
    /// 4'd4: positive pulse start(data bit included) && length end
    /// 4'd5: positive pulse start(data bit excluded) && length end
    #[inline(always)]
    pub const fn with_rx_pulse_submode_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxModeCfg::RX_PULSE_SUBMODE_SEL_MASK)
                | ((value << RxModeCfg::RX_PULSE_SUBMODE_SEL_OFFSET)
                    & RxModeCfg::RX_PULSE_SUBMODE_SEL_MASK),
        }
    }

    /// Set **RX_EXT_EN_INV** `[26:26]`
    ///
    /// Set this bit to invert the external enable signal.
    #[inline(always)]
    pub const fn with_rx_ext_en_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxModeCfg::RX_EXT_EN_INV_MASK
            } else {
                self.bits & !RxModeCfg::RX_EXT_EN_INV_MASK
            },
        }
    }

    /// Set **RX_SW_EN** `[25:25]`
    ///
    /// Set this bit to enable data sampling by software.
    #[inline(always)]
    pub const fn with_rx_sw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxModeCfg::RX_SW_EN_MASK
            } else {
                self.bits & !RxModeCfg::RX_SW_EN_MASK
            },
        }
    }

    /// Set **RX_EXT_EN_SEL** `[24:21]`
    ///
    /// Configures rx external enable signal selection from IO PAD.
    #[inline(always)]
    pub const fn with_rx_ext_en_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxModeCfg::RX_EXT_EN_SEL_MASK)
                | ((value << RxModeCfg::RX_EXT_EN_SEL_OFFSET) & RxModeCfg::RX_EXT_EN_SEL_MASK),
        }
    }
}

impl Default for RxModeCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxModeCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxModeCfgVal {
        RxModeCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxModeCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxModeCfgVal) -> RxModeCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_DATA_CFG Register
///
/// Parallel RX data configuration register.
///
/// Address offset: 0x0004
/// Reset value: 0x60000000
///
/// ## Fields
///
/// - **RX_BUS_WID_SEL** `[31:29]` (RW)
///
///   Configures the rxd bus width.
///   3'd0: bus width is 1.
///   3'd1: bus width is 2.
///   3'd2: bus width is 4.
///   3'd3: bus width is 8.
///
/// - **RX_DATA_ORDER_INV** `[28]` (RW)
///
///   Set this bit to invert bit order of one byte sent from RX_FIFO to DMA.
///
/// - **RX_BITLEN** `[27:9]` (RW)
///
///   Configures expected byte number of received data.
#[repr(transparent)]
pub struct RxDataCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxDataCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxDataCfg {
    // ======== Field Constants ========

    /// **RX_BUS_WID_SEL**
    ///
    /// Configures the rxd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const RX_BUS_WID_SEL_OFFSET: u32 = 29;
    pub const RX_BUS_WID_SEL_MASK: u32 = 0xe0000000;

    /// **RX_DATA_ORDER_INV**
    ///
    /// Set this bit to invert bit order of one byte sent from RX_FIFO to DMA.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const RX_DATA_ORDER_INV_OFFSET: u32 = 28;
    pub const RX_DATA_ORDER_INV_MASK: u32 = 0x10000000;
    pub const RX_DATA_ORDER_INV: u32 = 1 << 28;

    /// **RX_BITLEN**
    ///
    /// Configures expected byte number of received data.
    ///
    /// Bits `[27:9]`, mask `0x0ffffe00`
    pub const RX_BITLEN_OFFSET: u32 = 9;
    pub const RX_BITLEN_MASK: u32 = 0x0ffffe00;

    // ======== Field Accessors ========

    /// **RX_BUS_WID_SEL** `[31:29]` (RW)
    ///
    /// Configures the rxd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub fn rx_bus_wid_sel(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BUS_WID_SEL_MASK) >> Self::RX_BUS_WID_SEL_OFFSET
    }

    /// Write **RX_BUS_WID_SEL** `[31:29]` (RW)
    ///
    /// Configures the rxd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub fn write_rx_bus_wid_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BUS_WID_SEL_MASK)
                | ((value << Self::RX_BUS_WID_SEL_OFFSET) & Self::RX_BUS_WID_SEL_MASK)
        });
    }

    /// **RX_DATA_ORDER_INV** `[28]` (RW)
    ///
    /// Set this bit to invert bit order of one byte sent from RX_FIFO to DMA.
    #[inline(always)]
    pub fn rx_data_order_inv(&self) -> bool {
        (self.value.read_bits() & Self::RX_DATA_ORDER_INV_MASK) != 0
    }

    /// Set RX_DATA_ORDER_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_data_order_inv(&self) {
        self.value.modify_bits(|r| r | Self::RX_DATA_ORDER_INV_MASK);
    }

    /// Clear RX_DATA_ORDER_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_data_order_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_DATA_ORDER_INV_MASK);
    }

    /// **RX_BITLEN** `[27:9]` (RW)
    ///
    /// Configures expected byte number of received data.
    #[inline(always)]
    pub fn rx_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BITLEN_MASK) >> Self::RX_BITLEN_OFFSET
    }

    /// Write **RX_BITLEN** `[27:9]` (RW)
    ///
    /// Configures expected byte number of received data.
    #[inline(always)]
    pub fn write_rx_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_BITLEN_MASK) | ((value << Self::RX_BITLEN_OFFSET) & Self::RX_BITLEN_MASK)
        });
    }
}

/// RX_DATA_CFG value type for batch read/modify/write operations
///
/// Use `rxdatacfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxdatacfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxDataCfgVal {
    bits: u32,
}

impl RxDataCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x60000000 }
    }

    /// Read **RX_BUS_WID_SEL** `[31:29]`
    ///
    /// Configures the rxd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub const fn rx_bus_wid_sel(&self) -> u32 {
        (self.bits & RxDataCfg::RX_BUS_WID_SEL_MASK) >> RxDataCfg::RX_BUS_WID_SEL_OFFSET
    }

    /// Read **RX_DATA_ORDER_INV** `[28:28]`
    ///
    /// Set this bit to invert bit order of one byte sent from RX_FIFO to DMA.
    #[inline(always)]
    pub const fn rx_data_order_inv(&self) -> bool {
        (self.bits & RxDataCfg::RX_DATA_ORDER_INV_MASK) != 0
    }

    /// Read **RX_BITLEN** `[27:9]`
    ///
    /// Configures expected byte number of received data.
    #[inline(always)]
    pub const fn rx_bitlen(&self) -> u32 {
        (self.bits & RxDataCfg::RX_BITLEN_MASK) >> RxDataCfg::RX_BITLEN_OFFSET
    }

    /// Set **RX_BUS_WID_SEL** `[31:29]`
    ///
    /// Configures the rxd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub const fn with_rx_bus_wid_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxDataCfg::RX_BUS_WID_SEL_MASK)
                | ((value << RxDataCfg::RX_BUS_WID_SEL_OFFSET) & RxDataCfg::RX_BUS_WID_SEL_MASK),
        }
    }

    /// Set **RX_DATA_ORDER_INV** `[28:28]`
    ///
    /// Set this bit to invert bit order of one byte sent from RX_FIFO to DMA.
    #[inline(always)]
    pub const fn with_rx_data_order_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxDataCfg::RX_DATA_ORDER_INV_MASK
            } else {
                self.bits & !RxDataCfg::RX_DATA_ORDER_INV_MASK
            },
        }
    }

    /// Set **RX_BITLEN** `[27:9]`
    ///
    /// Configures expected byte number of received data.
    #[inline(always)]
    pub const fn with_rx_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxDataCfg::RX_BITLEN_MASK)
                | ((value << RxDataCfg::RX_BITLEN_OFFSET) & RxDataCfg::RX_BITLEN_MASK),
        }
    }
}

impl Default for RxDataCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxDataCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxDataCfgVal {
        RxDataCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxDataCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxDataCfgVal) -> RxDataCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_GENRL_CFG Register
///
/// Parallel RX general configuration register.
///
/// Address offset: 0x0008
/// Reset value: 0x21ffe000
///
/// ## Fields
///
/// - **RX_EOF_GEN_SEL** `[30]` (RW)
///
///   Configures the DMA eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by external enable signal.
///
/// - **RX_TIMEOUT_EN** `[29]` (RW)
///
///   Set this bit to enable timeout function to generate error eof.
///
/// - **RX_TIMEOUT_THRES** `[28:13]` (RW)
///
///   Configures threshold of timeout counter.
///
/// - **RX_GATING_EN** `[12]` (RW)
///
///   Set this bit to enable the clock gating of output rx clock.
#[repr(transparent)]
pub struct RxGenrlCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxGenrlCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxGenrlCfg {
    // ======== Field Constants ========

    /// **RX_EOF_GEN_SEL**
    ///
    /// Configures the DMA eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by external enable signal.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RX_EOF_GEN_SEL_OFFSET: u32 = 30;
    pub const RX_EOF_GEN_SEL_MASK: u32 = 0x40000000;
    pub const RX_EOF_GEN_SEL: u32 = 1 << 30;

    /// **RX_TIMEOUT_EN**
    ///
    /// Set this bit to enable timeout function to generate error eof.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RX_TIMEOUT_EN_OFFSET: u32 = 29;
    pub const RX_TIMEOUT_EN_MASK: u32 = 0x20000000;
    pub const RX_TIMEOUT_EN: u32 = 1 << 29;

    /// **RX_TIMEOUT_THRES**
    ///
    /// Configures threshold of timeout counter.
    ///
    /// Bits `[28:13]`, mask `0x1fffe000`
    pub const RX_TIMEOUT_THRES_OFFSET: u32 = 13;
    pub const RX_TIMEOUT_THRES_MASK: u32 = 0x1fffe000;

    /// **RX_GATING_EN**
    ///
    /// Set this bit to enable the clock gating of output rx clock.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_GATING_EN_OFFSET: u32 = 12;
    pub const RX_GATING_EN_MASK: u32 = 0x00001000;
    pub const RX_GATING_EN: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **RX_EOF_GEN_SEL** `[30]` (RW)
    ///
    /// Configures the DMA eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by external enable signal.
    #[inline(always)]
    pub fn rx_eof_gen_sel(&self) -> bool {
        (self.value.read_bits() & Self::RX_EOF_GEN_SEL_MASK) != 0
    }

    /// Set RX_EOF_GEN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_eof_gen_sel(&self) {
        self.value.modify_bits(|r| r | Self::RX_EOF_GEN_SEL_MASK);
    }

    /// Clear RX_EOF_GEN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_eof_gen_sel(&self) {
        self.value.modify_bits(|r| r & !Self::RX_EOF_GEN_SEL_MASK);
    }

    /// **RX_TIMEOUT_EN** `[29]` (RW)
    ///
    /// Set this bit to enable timeout function to generate error eof.
    #[inline(always)]
    pub fn rx_timeout_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TIMEOUT_EN_MASK) != 0
    }

    /// Set RX_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_timeout_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TIMEOUT_EN_MASK);
    }

    /// Clear RX_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_timeout_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TIMEOUT_EN_MASK);
    }

    /// **RX_TIMEOUT_THRES** `[28:13]` (RW)
    ///
    /// Configures threshold of timeout counter.
    #[inline(always)]
    pub fn rx_timeout_thres(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TIMEOUT_THRES_MASK) >> Self::RX_TIMEOUT_THRES_OFFSET
    }

    /// Write **RX_TIMEOUT_THRES** `[28:13]` (RW)
    ///
    /// Configures threshold of timeout counter.
    #[inline(always)]
    pub fn write_rx_timeout_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TIMEOUT_THRES_MASK)
                | ((value << Self::RX_TIMEOUT_THRES_OFFSET) & Self::RX_TIMEOUT_THRES_MASK)
        });
    }

    /// **RX_GATING_EN** `[12]` (RW)
    ///
    /// Set this bit to enable the clock gating of output rx clock.
    #[inline(always)]
    pub fn rx_gating_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_GATING_EN_MASK) != 0
    }

    /// Set RX_GATING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_gating_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_GATING_EN_MASK);
    }

    /// Clear RX_GATING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_gating_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_GATING_EN_MASK);
    }
}

/// RX_GENRL_CFG value type for batch read/modify/write operations
///
/// Use `rxgenrlcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxgenrlcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxGenrlCfgVal {
    bits: u32,
}

impl RxGenrlCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x21ffe000 }
    }

    /// Read **RX_EOF_GEN_SEL** `[30:30]`
    ///
    /// Configures the DMA eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by external enable signal.
    #[inline(always)]
    pub const fn rx_eof_gen_sel(&self) -> bool {
        (self.bits & RxGenrlCfg::RX_EOF_GEN_SEL_MASK) != 0
    }

    /// Read **RX_TIMEOUT_EN** `[29:29]`
    ///
    /// Set this bit to enable timeout function to generate error eof.
    #[inline(always)]
    pub const fn rx_timeout_en(&self) -> bool {
        (self.bits & RxGenrlCfg::RX_TIMEOUT_EN_MASK) != 0
    }

    /// Read **RX_TIMEOUT_THRES** `[28:13]`
    ///
    /// Configures threshold of timeout counter.
    #[inline(always)]
    pub const fn rx_timeout_thres(&self) -> u32 {
        (self.bits & RxGenrlCfg::RX_TIMEOUT_THRES_MASK) >> RxGenrlCfg::RX_TIMEOUT_THRES_OFFSET
    }

    /// Read **RX_GATING_EN** `[12:12]`
    ///
    /// Set this bit to enable the clock gating of output rx clock.
    #[inline(always)]
    pub const fn rx_gating_en(&self) -> bool {
        (self.bits & RxGenrlCfg::RX_GATING_EN_MASK) != 0
    }

    /// Set **RX_EOF_GEN_SEL** `[30:30]`
    ///
    /// Configures the DMA eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by external enable signal.
    #[inline(always)]
    pub const fn with_rx_eof_gen_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxGenrlCfg::RX_EOF_GEN_SEL_MASK
            } else {
                self.bits & !RxGenrlCfg::RX_EOF_GEN_SEL_MASK
            },
        }
    }

    /// Set **RX_TIMEOUT_EN** `[29:29]`
    ///
    /// Set this bit to enable timeout function to generate error eof.
    #[inline(always)]
    pub const fn with_rx_timeout_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxGenrlCfg::RX_TIMEOUT_EN_MASK
            } else {
                self.bits & !RxGenrlCfg::RX_TIMEOUT_EN_MASK
            },
        }
    }

    /// Set **RX_TIMEOUT_THRES** `[28:13]`
    ///
    /// Configures threshold of timeout counter.
    #[inline(always)]
    pub const fn with_rx_timeout_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxGenrlCfg::RX_TIMEOUT_THRES_MASK)
                | ((value << RxGenrlCfg::RX_TIMEOUT_THRES_OFFSET)
                    & RxGenrlCfg::RX_TIMEOUT_THRES_MASK),
        }
    }

    /// Set **RX_GATING_EN** `[12:12]`
    ///
    /// Set this bit to enable the clock gating of output rx clock.
    #[inline(always)]
    pub const fn with_rx_gating_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxGenrlCfg::RX_GATING_EN_MASK
            } else {
                self.bits & !RxGenrlCfg::RX_GATING_EN_MASK
            },
        }
    }
}

impl Default for RxGenrlCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxGenrlCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxGenrlCfgVal {
        RxGenrlCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxGenrlCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxGenrlCfgVal) -> RxGenrlCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_START_CFG Register
///
/// Parallel RX Start configuration register.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **RX_START** `[31]` (RW)
///
///   Set this bit to start rx data sampling.
#[repr(transparent)]
pub struct RxStartCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxStartCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxStartCfg {
    // ======== Field Constants ========

    /// **RX_START**
    ///
    /// Set this bit to start rx data sampling.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RX_START_OFFSET: u32 = 31;
    pub const RX_START_MASK: u32 = 0x80000000;
    pub const RX_START: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **RX_START** `[31]` (RW)
    ///
    /// Set this bit to start rx data sampling.
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
}

/// RX_START_CFG value type for batch read/modify/write operations
///
/// Use `rxstartcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxstartcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxStartCfgVal {
    bits: u32,
}

impl RxStartCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_START** `[31:31]`
    ///
    /// Set this bit to start rx data sampling.
    #[inline(always)]
    pub const fn rx_start(&self) -> bool {
        (self.bits & RxStartCfg::RX_START_MASK) != 0
    }

    /// Set **RX_START** `[31:31]`
    ///
    /// Set this bit to start rx data sampling.
    #[inline(always)]
    pub const fn with_rx_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxStartCfg::RX_START_MASK
            } else {
                self.bits & !RxStartCfg::RX_START_MASK
            },
        }
    }
}

impl Default for RxStartCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxStartCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxStartCfgVal {
        RxStartCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxStartCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxStartCfgVal) -> RxStartCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_DATA_CFG Register
///
/// Parallel TX data configuration register.
///
/// Address offset: 0x0010
/// Reset value: 0x60000000
///
/// ## Fields
///
/// - **TX_BUS_WID_SEL** `[31:29]` (RW)
///
///   Configures the txd bus width.
///   3'd0: bus width is 1.
///   3'd1: bus width is 2.
///   3'd2: bus width is 4.
///   3'd3: bus width is 8.
///
/// - **TX_DATA_ORDER_INV** `[28]` (RW)
///
///   Set this bit to invert bit order of one byte sent from TX_FIFO to IO data.
///
/// - **TX_BITLEN** `[27:9]` (RW)
///
///   Configures expected byte number of sent data.
#[repr(transparent)]
pub struct TxDataCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxDataCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxDataCfg {
    // ======== Field Constants ========

    /// **TX_BUS_WID_SEL**
    ///
    /// Configures the txd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const TX_BUS_WID_SEL_OFFSET: u32 = 29;
    pub const TX_BUS_WID_SEL_MASK: u32 = 0xe0000000;

    /// **TX_DATA_ORDER_INV**
    ///
    /// Set this bit to invert bit order of one byte sent from TX_FIFO to IO data.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TX_DATA_ORDER_INV_OFFSET: u32 = 28;
    pub const TX_DATA_ORDER_INV_MASK: u32 = 0x10000000;
    pub const TX_DATA_ORDER_INV: u32 = 1 << 28;

    /// **TX_BITLEN**
    ///
    /// Configures expected byte number of sent data.
    ///
    /// Bits `[27:9]`, mask `0x0ffffe00`
    pub const TX_BITLEN_OFFSET: u32 = 9;
    pub const TX_BITLEN_MASK: u32 = 0x0ffffe00;

    // ======== Field Accessors ========

    /// **TX_BUS_WID_SEL** `[31:29]` (RW)
    ///
    /// Configures the txd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub fn tx_bus_wid_sel(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BUS_WID_SEL_MASK) >> Self::TX_BUS_WID_SEL_OFFSET
    }

    /// Write **TX_BUS_WID_SEL** `[31:29]` (RW)
    ///
    /// Configures the txd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub fn write_tx_bus_wid_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BUS_WID_SEL_MASK)
                | ((value << Self::TX_BUS_WID_SEL_OFFSET) & Self::TX_BUS_WID_SEL_MASK)
        });
    }

    /// **TX_DATA_ORDER_INV** `[28]` (RW)
    ///
    /// Set this bit to invert bit order of one byte sent from TX_FIFO to IO data.
    #[inline(always)]
    pub fn tx_data_order_inv(&self) -> bool {
        (self.value.read_bits() & Self::TX_DATA_ORDER_INV_MASK) != 0
    }

    /// Set TX_DATA_ORDER_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_data_order_inv(&self) {
        self.value.modify_bits(|r| r | Self::TX_DATA_ORDER_INV_MASK);
    }

    /// Clear TX_DATA_ORDER_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_data_order_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_DATA_ORDER_INV_MASK);
    }

    /// **TX_BITLEN** `[27:9]` (RW)
    ///
    /// Configures expected byte number of sent data.
    #[inline(always)]
    pub fn tx_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BITLEN_MASK) >> Self::TX_BITLEN_OFFSET
    }

    /// Write **TX_BITLEN** `[27:9]` (RW)
    ///
    /// Configures expected byte number of sent data.
    #[inline(always)]
    pub fn write_tx_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BITLEN_MASK) | ((value << Self::TX_BITLEN_OFFSET) & Self::TX_BITLEN_MASK)
        });
    }
}

/// TX_DATA_CFG value type for batch read/modify/write operations
///
/// Use `txdatacfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txdatacfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxDataCfgVal {
    bits: u32,
}

impl TxDataCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x60000000 }
    }

    /// Read **TX_BUS_WID_SEL** `[31:29]`
    ///
    /// Configures the txd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub const fn tx_bus_wid_sel(&self) -> u32 {
        (self.bits & TxDataCfg::TX_BUS_WID_SEL_MASK) >> TxDataCfg::TX_BUS_WID_SEL_OFFSET
    }

    /// Read **TX_DATA_ORDER_INV** `[28:28]`
    ///
    /// Set this bit to invert bit order of one byte sent from TX_FIFO to IO data.
    #[inline(always)]
    pub const fn tx_data_order_inv(&self) -> bool {
        (self.bits & TxDataCfg::TX_DATA_ORDER_INV_MASK) != 0
    }

    /// Read **TX_BITLEN** `[27:9]`
    ///
    /// Configures expected byte number of sent data.
    #[inline(always)]
    pub const fn tx_bitlen(&self) -> u32 {
        (self.bits & TxDataCfg::TX_BITLEN_MASK) >> TxDataCfg::TX_BITLEN_OFFSET
    }

    /// Set **TX_BUS_WID_SEL** `[31:29]`
    ///
    /// Configures the txd bus width.
    /// 3'd0: bus width is 1.
    /// 3'd1: bus width is 2.
    /// 3'd2: bus width is 4.
    /// 3'd3: bus width is 8.
    #[inline(always)]
    pub const fn with_tx_bus_wid_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxDataCfg::TX_BUS_WID_SEL_MASK)
                | ((value << TxDataCfg::TX_BUS_WID_SEL_OFFSET) & TxDataCfg::TX_BUS_WID_SEL_MASK),
        }
    }

    /// Set **TX_DATA_ORDER_INV** `[28:28]`
    ///
    /// Set this bit to invert bit order of one byte sent from TX_FIFO to IO data.
    #[inline(always)]
    pub const fn with_tx_data_order_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxDataCfg::TX_DATA_ORDER_INV_MASK
            } else {
                self.bits & !TxDataCfg::TX_DATA_ORDER_INV_MASK
            },
        }
    }

    /// Set **TX_BITLEN** `[27:9]`
    ///
    /// Configures expected byte number of sent data.
    #[inline(always)]
    pub const fn with_tx_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxDataCfg::TX_BITLEN_MASK)
                | ((value << TxDataCfg::TX_BITLEN_OFFSET) & TxDataCfg::TX_BITLEN_MASK),
        }
    }
}

impl Default for TxDataCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxDataCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxDataCfgVal {
        TxDataCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxDataCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxDataCfgVal) -> TxDataCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_START_CFG Register
///
/// Parallel TX Start configuration register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **TX_START** `[31]` (RW)
///
///   Set this bit to start tx data transmit.
#[repr(transparent)]
pub struct TxStartCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxStartCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxStartCfg {
    // ======== Field Constants ========

    /// **TX_START**
    ///
    /// Set this bit to start tx data transmit.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_START_OFFSET: u32 = 31;
    pub const TX_START_MASK: u32 = 0x80000000;
    pub const TX_START: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **TX_START** `[31]` (RW)
    ///
    /// Set this bit to start tx data transmit.
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
}

/// TX_START_CFG value type for batch read/modify/write operations
///
/// Use `txstartcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txstartcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxStartCfgVal {
    bits: u32,
}

impl TxStartCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_START** `[31:31]`
    ///
    /// Set this bit to start tx data transmit.
    #[inline(always)]
    pub const fn tx_start(&self) -> bool {
        (self.bits & TxStartCfg::TX_START_MASK) != 0
    }

    /// Set **TX_START** `[31:31]`
    ///
    /// Set this bit to start tx data transmit.
    #[inline(always)]
    pub const fn with_tx_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxStartCfg::TX_START_MASK
            } else {
                self.bits & !TxStartCfg::TX_START_MASK
            },
        }
    }
}

impl Default for TxStartCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxStartCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxStartCfgVal {
        TxStartCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxStartCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxStartCfgVal) -> TxStartCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_GENRL_CFG Register
///
/// Parallel TX general configuration register.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **TX_VALID_OUTPUT_EN** `[31]` (RW)
///
///   Set this bit to enable the output of tx data valid signal.
///
/// - **TX_GATING_EN** `[30]` (RW)
///
///   Set this bit to enable the clock gating of output tx clock.
///
/// - **TX_IDLE_VALUE** `[29:14]` (RW)
///
///   Configures bus value of transmitter in IDLE state.
///
/// - **TX_EOF_GEN_SEL** `[13]` (RW)
///
///   Configures the tx eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by DMA eof.
#[repr(transparent)]
pub struct TxGenrlCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxGenrlCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxGenrlCfg {
    // ======== Field Constants ========

    /// **TX_VALID_OUTPUT_EN**
    ///
    /// Set this bit to enable the output of tx data valid signal.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_VALID_OUTPUT_EN_OFFSET: u32 = 31;
    pub const TX_VALID_OUTPUT_EN_MASK: u32 = 0x80000000;
    pub const TX_VALID_OUTPUT_EN: u32 = 1 << 31;

    /// **TX_GATING_EN**
    ///
    /// Set this bit to enable the clock gating of output tx clock.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TX_GATING_EN_OFFSET: u32 = 30;
    pub const TX_GATING_EN_MASK: u32 = 0x40000000;
    pub const TX_GATING_EN: u32 = 1 << 30;

    /// **TX_IDLE_VALUE**
    ///
    /// Configures bus value of transmitter in IDLE state.
    ///
    /// Bits `[29:14]`, mask `0x3fffc000`
    pub const TX_IDLE_VALUE_OFFSET: u32 = 14;
    pub const TX_IDLE_VALUE_MASK: u32 = 0x3fffc000;

    /// **TX_EOF_GEN_SEL**
    ///
    /// Configures the tx eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by DMA eof.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_EOF_GEN_SEL_OFFSET: u32 = 13;
    pub const TX_EOF_GEN_SEL_MASK: u32 = 0x00002000;
    pub const TX_EOF_GEN_SEL: u32 = 1 << 13;

    // ======== Field Accessors ========

    /// **TX_VALID_OUTPUT_EN** `[31]` (RW)
    ///
    /// Set this bit to enable the output of tx data valid signal.
    #[inline(always)]
    pub fn tx_valid_output_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_VALID_OUTPUT_EN_MASK) != 0
    }

    /// Set TX_VALID_OUTPUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_valid_output_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_VALID_OUTPUT_EN_MASK);
    }

    /// Clear TX_VALID_OUTPUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_valid_output_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_VALID_OUTPUT_EN_MASK);
    }

    /// **TX_GATING_EN** `[30]` (RW)
    ///
    /// Set this bit to enable the clock gating of output tx clock.
    #[inline(always)]
    pub fn tx_gating_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_GATING_EN_MASK) != 0
    }

    /// Set TX_GATING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_gating_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_GATING_EN_MASK);
    }

    /// Clear TX_GATING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_gating_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_GATING_EN_MASK);
    }

    /// **TX_IDLE_VALUE** `[29:14]` (RW)
    ///
    /// Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    pub fn tx_idle_value(&self) -> u32 {
        (self.value.read_bits() & Self::TX_IDLE_VALUE_MASK) >> Self::TX_IDLE_VALUE_OFFSET
    }

    /// Write **TX_IDLE_VALUE** `[29:14]` (RW)
    ///
    /// Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    pub fn write_tx_idle_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_IDLE_VALUE_MASK)
                | ((value << Self::TX_IDLE_VALUE_OFFSET) & Self::TX_IDLE_VALUE_MASK)
        });
    }

    /// **TX_EOF_GEN_SEL** `[13]` (RW)
    ///
    /// Configures the tx eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by DMA eof.
    #[inline(always)]
    pub fn tx_eof_gen_sel(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_GEN_SEL_MASK) != 0
    }

    /// Set TX_EOF_GEN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_eof_gen_sel(&self) {
        self.value.modify_bits(|r| r | Self::TX_EOF_GEN_SEL_MASK);
    }

    /// Clear TX_EOF_GEN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_eof_gen_sel(&self) {
        self.value.modify_bits(|r| r & !Self::TX_EOF_GEN_SEL_MASK);
    }
}

/// TX_GENRL_CFG value type for batch read/modify/write operations
///
/// Use `txgenrlcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txgenrlcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxGenrlCfgVal {
    bits: u32,
}

impl TxGenrlCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_VALID_OUTPUT_EN** `[31:31]`
    ///
    /// Set this bit to enable the output of tx data valid signal.
    #[inline(always)]
    pub const fn tx_valid_output_en(&self) -> bool {
        (self.bits & TxGenrlCfg::TX_VALID_OUTPUT_EN_MASK) != 0
    }

    /// Read **TX_GATING_EN** `[30:30]`
    ///
    /// Set this bit to enable the clock gating of output tx clock.
    #[inline(always)]
    pub const fn tx_gating_en(&self) -> bool {
        (self.bits & TxGenrlCfg::TX_GATING_EN_MASK) != 0
    }

    /// Read **TX_IDLE_VALUE** `[29:14]`
    ///
    /// Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    pub const fn tx_idle_value(&self) -> u32 {
        (self.bits & TxGenrlCfg::TX_IDLE_VALUE_MASK) >> TxGenrlCfg::TX_IDLE_VALUE_OFFSET
    }

    /// Read **TX_EOF_GEN_SEL** `[13:13]`
    ///
    /// Configures the tx eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by DMA eof.
    #[inline(always)]
    pub const fn tx_eof_gen_sel(&self) -> bool {
        (self.bits & TxGenrlCfg::TX_EOF_GEN_SEL_MASK) != 0
    }

    /// Set **TX_VALID_OUTPUT_EN** `[31:31]`
    ///
    /// Set this bit to enable the output of tx data valid signal.
    #[inline(always)]
    pub const fn with_tx_valid_output_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxGenrlCfg::TX_VALID_OUTPUT_EN_MASK
            } else {
                self.bits & !TxGenrlCfg::TX_VALID_OUTPUT_EN_MASK
            },
        }
    }

    /// Set **TX_GATING_EN** `[30:30]`
    ///
    /// Set this bit to enable the clock gating of output tx clock.
    #[inline(always)]
    pub const fn with_tx_gating_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxGenrlCfg::TX_GATING_EN_MASK
            } else {
                self.bits & !TxGenrlCfg::TX_GATING_EN_MASK
            },
        }
    }

    /// Set **TX_IDLE_VALUE** `[29:14]`
    ///
    /// Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    pub const fn with_tx_idle_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxGenrlCfg::TX_IDLE_VALUE_MASK)
                | ((value << TxGenrlCfg::TX_IDLE_VALUE_OFFSET) & TxGenrlCfg::TX_IDLE_VALUE_MASK),
        }
    }

    /// Set **TX_EOF_GEN_SEL** `[13:13]`
    ///
    /// Configures the tx eof generated mechanism. 1'b0: eof generated by data bit length. 1'b1: eof generated by DMA eof.
    #[inline(always)]
    pub const fn with_tx_eof_gen_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxGenrlCfg::TX_EOF_GEN_SEL_MASK
            } else {
                self.bits & !TxGenrlCfg::TX_EOF_GEN_SEL_MASK
            },
        }
    }
}

impl Default for TxGenrlCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxGenrlCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxGenrlCfgVal {
        TxGenrlCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxGenrlCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxGenrlCfgVal) -> TxGenrlCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FIFO_CFG Register
///
/// Parallel IO FIFO configuration register.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **RX_FIFO_SRST** `[31]` (RW)
///
///   Set this bit to reset async fifo in rx module.
///
/// - **TX_FIFO_SRST** `[30]` (RW)
///
///   Set this bit to reset async fifo in tx module.
#[repr(transparent)]
pub struct FifoCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoCfg {
    // ======== Field Constants ========

    /// **RX_FIFO_SRST**
    ///
    /// Set this bit to reset async fifo in rx module.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RX_FIFO_SRST_OFFSET: u32 = 31;
    pub const RX_FIFO_SRST_MASK: u32 = 0x80000000;
    pub const RX_FIFO_SRST: u32 = 1 << 31;

    /// **TX_FIFO_SRST**
    ///
    /// Set this bit to reset async fifo in tx module.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TX_FIFO_SRST_OFFSET: u32 = 30;
    pub const TX_FIFO_SRST_MASK: u32 = 0x40000000;
    pub const TX_FIFO_SRST: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **RX_FIFO_SRST** `[31]` (RW)
    ///
    /// Set this bit to reset async fifo in rx module.
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_SRST_MASK) != 0
    }

    /// Set RX_FIFO_SRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fifo_srst(&self) {
        self.value.modify_bits(|r| r | Self::RX_FIFO_SRST_MASK);
    }

    /// Clear RX_FIFO_SRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fifo_srst(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FIFO_SRST_MASK);
    }

    /// **TX_FIFO_SRST** `[30]` (RW)
    ///
    /// Set this bit to reset async fifo in tx module.
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_SRST_MASK) != 0
    }

    /// Set TX_FIFO_SRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_fifo_srst(&self) {
        self.value.modify_bits(|r| r | Self::TX_FIFO_SRST_MASK);
    }

    /// Clear TX_FIFO_SRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_fifo_srst(&self) {
        self.value.modify_bits(|r| r & !Self::TX_FIFO_SRST_MASK);
    }
}

/// FIFO_CFG value type for batch read/modify/write operations
///
/// Use `fifocfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifocfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoCfgVal {
    bits: u32,
}

impl FifoCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_FIFO_SRST** `[31:31]`
    ///
    /// Set this bit to reset async fifo in rx module.
    #[inline(always)]
    pub const fn rx_fifo_srst(&self) -> bool {
        (self.bits & FifoCfg::RX_FIFO_SRST_MASK) != 0
    }

    /// Read **TX_FIFO_SRST** `[30:30]`
    ///
    /// Set this bit to reset async fifo in tx module.
    #[inline(always)]
    pub const fn tx_fifo_srst(&self) -> bool {
        (self.bits & FifoCfg::TX_FIFO_SRST_MASK) != 0
    }

    /// Set **RX_FIFO_SRST** `[31:31]`
    ///
    /// Set this bit to reset async fifo in rx module.
    #[inline(always)]
    pub const fn with_rx_fifo_srst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoCfg::RX_FIFO_SRST_MASK
            } else {
                self.bits & !FifoCfg::RX_FIFO_SRST_MASK
            },
        }
    }

    /// Set **TX_FIFO_SRST** `[30:30]`
    ///
    /// Set this bit to reset async fifo in tx module.
    #[inline(always)]
    pub const fn with_tx_fifo_srst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoCfg::TX_FIFO_SRST_MASK
            } else {
                self.bits & !FifoCfg::TX_FIFO_SRST_MASK
            },
        }
    }
}

impl Default for FifoCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FifoCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoCfgVal {
        FifoCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoCfgVal) -> FifoCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_UPDATE Register
///
/// Parallel IO FIFO configuration register.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **RX_REG_UPDATE** `[31]` (WO)
///
///   Set this bit to update rx register configuration.
#[repr(transparent)]
pub struct RegUpdate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegUpdate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegUpdate {
    // ======== Field Constants ========

    /// **RX_REG_UPDATE**
    ///
    /// Set this bit to update rx register configuration.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RX_REG_UPDATE_OFFSET: u32 = 31;
    pub const RX_REG_UPDATE_MASK: u32 = 0x80000000;
    pub const RX_REG_UPDATE: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **RX_REG_UPDATE** (WO)
    ///
    /// Set this bit to update rx register configuration.
    #[inline(always)]
    pub fn trigger_rx_reg_update(&self) {
        self.value.write_bits(Self::RX_REG_UPDATE_MASK);
    }
}

/// REG_UPDATE value type for batch read/modify/write operations
///
/// Use `regupdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regupdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegUpdateVal {
    bits: u32,
}

impl RegUpdateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **RX_REG_UPDATE** `[31:31]`
    ///
    /// Set this bit to update rx register configuration.
    #[inline(always)]
    pub const fn with_rx_reg_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegUpdate::RX_REG_UPDATE_MASK
            } else {
                self.bits & !RegUpdate::RX_REG_UPDATE_MASK
            },
        }
    }
}

impl Default for RegUpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegUpdate {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegUpdateVal) {
        self.value.write_bits(val.bits())
    }
}

/// ST Register
///
/// Parallel IO module status register0.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **TX_READY** `[31]` (RO)
///
///   Represents the status that tx is ready to transmit.
#[repr(transparent)]
pub struct St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl St {
    // ======== Field Constants ========

    /// **TX_READY**
    ///
    /// Represents the status that tx is ready to transmit.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_READY_OFFSET: u32 = 31;
    pub const TX_READY_MASK: u32 = 0x80000000;
    pub const TX_READY: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **TX_READY** `[31]` (RO)
    ///
    /// Represents the status that tx is ready to transmit.
    #[inline(always)]
    pub fn tx_ready(&self) -> bool {
        (self.value.read_bits() & Self::TX_READY_MASK) != 0
    }
}

/// ST value type for batch read/modify/write operations
///
/// Use `st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StVal {
    bits: u32,
}

impl StVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_READY** `[31:31]`
    ///
    /// Represents the status that tx is ready to transmit.
    #[inline(always)]
    pub const fn tx_ready(&self) -> bool {
        (self.bits & St::TX_READY_MASK) != 0
    }
}

impl Default for StVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StVal {
        StVal::from_bits(self.value.read_bits())
    }
}

/// INT_ENA Register
///
/// Parallel IO interrupt enable singal configuration register.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **TX_EOF_INT_ENA** `[2]` (RW)
///
///   Set this bit to enable TX_EOF_INT.
///
/// - **RX_FIFO_WOVF_INT_ENA** `[1]` (RW)
///
///   Set this bit to enable RX_FIFO_WOVF_INT.
///
/// - **TX_FIFO_REMPTY_INT_ENA** `[0]` (RW)
///
///   Set this bit to enable TX_FIFO_REMPTY_INT.
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

    /// **TX_EOF_INT_ENA**
    ///
    /// Set this bit to enable TX_EOF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_EOF_INT_ENA_OFFSET: u32 = 2;
    pub const TX_EOF_INT_ENA_MASK: u32 = 0x00000004;
    pub const TX_EOF_INT_ENA: u32 = 1 << 2;

    /// **RX_FIFO_WOVF_INT_ENA**
    ///
    /// Set this bit to enable RX_FIFO_WOVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_FIFO_WOVF_INT_ENA_OFFSET: u32 = 1;
    pub const RX_FIFO_WOVF_INT_ENA_MASK: u32 = 0x00000002;
    pub const RX_FIFO_WOVF_INT_ENA: u32 = 1 << 1;

    /// **TX_FIFO_REMPTY_INT_ENA**
    ///
    /// Set this bit to enable TX_FIFO_REMPTY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_FIFO_REMPTY_INT_ENA_OFFSET: u32 = 0;
    pub const TX_FIFO_REMPTY_INT_ENA_MASK: u32 = 0x00000001;
    pub const TX_FIFO_REMPTY_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_EOF_INT_ENA** `[2]` (RW)
    ///
    /// Set this bit to enable TX_EOF_INT.
    #[inline(always)]
    pub fn tx_eof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_INT_ENA_MASK) != 0
    }

    /// Set TX_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_eof_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_EOF_INT_ENA_MASK);
    }

    /// Clear TX_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_eof_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_EOF_INT_ENA_MASK);
    }

    /// **RX_FIFO_WOVF_INT_ENA** `[1]` (RW)
    ///
    /// Set this bit to enable RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub fn rx_fifo_wovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_WOVF_INT_ENA_MASK) != 0
    }

    /// Set RX_FIFO_WOVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fifo_wovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_FIFO_WOVF_INT_ENA_MASK);
    }

    /// Clear RX_FIFO_WOVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fifo_wovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_FIFO_WOVF_INT_ENA_MASK);
    }

    /// **TX_FIFO_REMPTY_INT_ENA** `[0]` (RW)
    ///
    /// Set this bit to enable TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub fn tx_fifo_rempty_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_REMPTY_INT_ENA_MASK) != 0
    }

    /// Set TX_FIFO_REMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_fifo_rempty_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_FIFO_REMPTY_INT_ENA_MASK);
    }

    /// Clear TX_FIFO_REMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_fifo_rempty_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_FIFO_REMPTY_INT_ENA_MASK);
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

    /// Read **TX_EOF_INT_ENA** `[2:2]`
    ///
    /// Set this bit to enable TX_EOF_INT.
    #[inline(always)]
    pub const fn tx_eof_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_EOF_INT_ENA_MASK) != 0
    }

    /// Read **RX_FIFO_WOVF_INT_ENA** `[1:1]`
    ///
    /// Set this bit to enable RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn rx_fifo_wovf_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_FIFO_WOVF_INT_ENA_MASK) != 0
    }

    /// Read **TX_FIFO_REMPTY_INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn tx_fifo_rempty_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_FIFO_REMPTY_INT_ENA_MASK) != 0
    }

    /// Set **TX_EOF_INT_ENA** `[2:2]`
    ///
    /// Set this bit to enable TX_EOF_INT.
    #[inline(always)]
    pub const fn with_tx_eof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_EOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_EOF_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_FIFO_WOVF_INT_ENA** `[1:1]`
    ///
    /// Set this bit to enable RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn with_rx_fifo_wovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_FIFO_WOVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_FIFO_WOVF_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_FIFO_REMPTY_INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn with_tx_fifo_rempty_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_FIFO_REMPTY_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_FIFO_REMPTY_INT_ENA_MASK
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

/// INT_RAW Register
///
/// Parallel IO interrupt raw singal status register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **TX_EOF_INT_RAW** `[2]` (RW)
///
///   The raw interrupt status of TX_EOF_INT.
///
/// - **RX_FIFO_WOVF_INT_RAW** `[1]` (RW)
///
///   The raw interrupt status of RX_FIFO_WOVF_INT.
///
/// - **TX_FIFO_REMPTY_INT_RAW** `[0]` (RW)
///
///   The raw interrupt status of TX_FIFO_REMPTY_INT.
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

    /// **TX_EOF_INT_RAW**
    ///
    /// The raw interrupt status of TX_EOF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_EOF_INT_RAW_OFFSET: u32 = 2;
    pub const TX_EOF_INT_RAW_MASK: u32 = 0x00000004;
    pub const TX_EOF_INT_RAW: u32 = 1 << 2;

    /// **RX_FIFO_WOVF_INT_RAW**
    ///
    /// The raw interrupt status of RX_FIFO_WOVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_FIFO_WOVF_INT_RAW_OFFSET: u32 = 1;
    pub const RX_FIFO_WOVF_INT_RAW_MASK: u32 = 0x00000002;
    pub const RX_FIFO_WOVF_INT_RAW: u32 = 1 << 1;

    /// **TX_FIFO_REMPTY_INT_RAW**
    ///
    /// The raw interrupt status of TX_FIFO_REMPTY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_FIFO_REMPTY_INT_RAW_OFFSET: u32 = 0;
    pub const TX_FIFO_REMPTY_INT_RAW_MASK: u32 = 0x00000001;
    pub const TX_FIFO_REMPTY_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_EOF_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt status of TX_EOF_INT.
    #[inline(always)]
    pub fn tx_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_INT_RAW_MASK) != 0
    }

    /// Set TX_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_eof_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TX_EOF_INT_RAW_MASK);
    }

    /// Clear TX_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_eof_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TX_EOF_INT_RAW_MASK);
    }

    /// **RX_FIFO_WOVF_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt status of RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub fn rx_fifo_wovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_WOVF_INT_RAW_MASK) != 0
    }

    /// Set RX_FIFO_WOVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fifo_wovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_FIFO_WOVF_INT_RAW_MASK);
    }

    /// Clear RX_FIFO_WOVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fifo_wovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_FIFO_WOVF_INT_RAW_MASK);
    }

    /// **TX_FIFO_REMPTY_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt status of TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub fn tx_fifo_rempty_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_REMPTY_INT_RAW_MASK) != 0
    }

    /// Set TX_FIFO_REMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_fifo_rempty_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_FIFO_REMPTY_INT_RAW_MASK);
    }

    /// Clear TX_FIFO_REMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_fifo_rempty_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_FIFO_REMPTY_INT_RAW_MASK);
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

    /// Read **TX_EOF_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status of TX_EOF_INT.
    #[inline(always)]
    pub const fn tx_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_EOF_INT_RAW_MASK) != 0
    }

    /// Read **RX_FIFO_WOVF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status of RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn rx_fifo_wovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_FIFO_WOVF_INT_RAW_MASK) != 0
    }

    /// Read **TX_FIFO_REMPTY_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status of TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn tx_fifo_rempty_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_FIFO_REMPTY_INT_RAW_MASK) != 0
    }

    /// Set **TX_EOF_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status of TX_EOF_INT.
    #[inline(always)]
    pub const fn with_tx_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_EOF_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_FIFO_WOVF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status of RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn with_rx_fifo_wovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_FIFO_WOVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_FIFO_WOVF_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_FIFO_REMPTY_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status of TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn with_tx_fifo_rempty_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_FIFO_REMPTY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_FIFO_REMPTY_INT_RAW_MASK
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
/// Parallel IO interrupt singal status register.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **TX_EOF_INT_ST** `[2]` (RO)
///
///   The masked interrupt status of TX_EOF_INT.
///
/// - **RX_FIFO_WOVF_INT_ST** `[1]` (RO)
///
///   The masked interrupt status of RX_FIFO_WOVF_INT.
///
/// - **TX_FIFO_REMPTY_INT_ST** `[0]` (RO)
///
///   The masked interrupt status of TX_FIFO_REMPTY_INT.
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

    /// **TX_EOF_INT_ST**
    ///
    /// The masked interrupt status of TX_EOF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_EOF_INT_ST_OFFSET: u32 = 2;
    pub const TX_EOF_INT_ST_MASK: u32 = 0x00000004;
    pub const TX_EOF_INT_ST: u32 = 1 << 2;

    /// **RX_FIFO_WOVF_INT_ST**
    ///
    /// The masked interrupt status of RX_FIFO_WOVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_FIFO_WOVF_INT_ST_OFFSET: u32 = 1;
    pub const RX_FIFO_WOVF_INT_ST_MASK: u32 = 0x00000002;
    pub const RX_FIFO_WOVF_INT_ST: u32 = 1 << 1;

    /// **TX_FIFO_REMPTY_INT_ST**
    ///
    /// The masked interrupt status of TX_FIFO_REMPTY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_FIFO_REMPTY_INT_ST_OFFSET: u32 = 0;
    pub const TX_FIFO_REMPTY_INT_ST_MASK: u32 = 0x00000001;
    pub const TX_FIFO_REMPTY_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_EOF_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status of TX_EOF_INT.
    #[inline(always)]
    pub fn tx_eof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_EOF_INT_ST_MASK) != 0
    }

    /// **RX_FIFO_WOVF_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status of RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub fn rx_fifo_wovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_WOVF_INT_ST_MASK) != 0
    }

    /// **TX_FIFO_REMPTY_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status of TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub fn tx_fifo_rempty_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_REMPTY_INT_ST_MASK) != 0
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

    /// Read **TX_EOF_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status of TX_EOF_INT.
    #[inline(always)]
    pub const fn tx_eof_int_st(&self) -> bool {
        (self.bits & IntSt::TX_EOF_INT_ST_MASK) != 0
    }

    /// Read **RX_FIFO_WOVF_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status of RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn rx_fifo_wovf_int_st(&self) -> bool {
        (self.bits & IntSt::RX_FIFO_WOVF_INT_ST_MASK) != 0
    }

    /// Read **TX_FIFO_REMPTY_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status of TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn tx_fifo_rempty_int_st(&self) -> bool {
        (self.bits & IntSt::TX_FIFO_REMPTY_INT_ST_MASK) != 0
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

/// INT_CLR Register
///
/// Parallel IO interrupt clear singal configuration register.
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **TX_EOF_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear TX_EOF_INT.
///
/// - **RX_FIFO_WOVF_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear RX_FIFO_WOVF_INT.
///
/// - **TX_FIFO_REMPTY_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear TX_FIFO_REMPTY_INT.
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

    /// **TX_EOF_INT_CLR**
    ///
    /// Set this bit to clear TX_EOF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_EOF_INT_CLR_OFFSET: u32 = 2;
    pub const TX_EOF_INT_CLR_MASK: u32 = 0x00000004;
    pub const TX_EOF_INT_CLR: u32 = 1 << 2;

    /// **RX_FIFO_WOVF_INT_CLR**
    ///
    /// Set this bit to clear RX_FIFO_WOVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_FIFO_WOVF_INT_CLR_OFFSET: u32 = 1;
    pub const RX_FIFO_WOVF_INT_CLR_MASK: u32 = 0x00000002;
    pub const RX_FIFO_WOVF_INT_CLR: u32 = 1 << 1;

    /// **TX_FIFO_REMPTY_INT_CLR**
    ///
    /// Set this bit to clear TX_FIFO_REMPTY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_FIFO_REMPTY_INT_CLR_OFFSET: u32 = 0;
    pub const TX_FIFO_REMPTY_INT_CLR_MASK: u32 = 0x00000001;
    pub const TX_FIFO_REMPTY_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TX_EOF_INT_CLR** (WO)
    ///
    /// Set this bit to clear TX_EOF_INT.
    #[inline(always)]
    pub fn trigger_tx_eof_int_clr(&self) {
        self.value.write_bits(Self::TX_EOF_INT_CLR_MASK);
    }

    /// Trigger **RX_FIFO_WOVF_INT_CLR** (WO)
    ///
    /// Set this bit to clear RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub fn trigger_rx_fifo_wovf_int_clr(&self) {
        self.value.write_bits(Self::RX_FIFO_WOVF_INT_CLR_MASK);
    }

    /// Trigger **TX_FIFO_REMPTY_INT_CLR** (WO)
    ///
    /// Set this bit to clear TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub fn trigger_tx_fifo_rempty_int_clr(&self) {
        self.value.write_bits(Self::TX_FIFO_REMPTY_INT_CLR_MASK);
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

    /// Set **TX_EOF_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear TX_EOF_INT.
    #[inline(always)]
    pub const fn with_tx_eof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_EOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_EOF_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_FIFO_WOVF_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub const fn with_rx_fifo_wovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_FIFO_WOVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_FIFO_WOVF_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_FIFO_REMPTY_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub const fn with_tx_fifo_rempty_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_FIFO_REMPTY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_FIFO_REMPTY_INT_CLR_MASK
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

/// RX_ST0 Register
///
/// Parallel IO RX status register0
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **RX_FIFO_WR_BIT_CNT** `[31:13]` (RO)
///
///   Indicates the current written bit number into Rx FIFO.
///
/// - **RX_CNT** `[12:8]` (RO)
///
///   Indicates the cycle number of reading Rx FIFO.
#[repr(transparent)]
pub struct RxSt0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxSt0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxSt0 {
    // ======== Field Constants ========

    /// **RX_FIFO_WR_BIT_CNT**
    ///
    /// Indicates the current written bit number into Rx FIFO.
    ///
    /// Bits `[31:13]`, mask `0xffffe000`
    pub const RX_FIFO_WR_BIT_CNT_OFFSET: u32 = 13;
    pub const RX_FIFO_WR_BIT_CNT_MASK: u32 = 0xffffe000;

    /// **RX_CNT**
    ///
    /// Indicates the cycle number of reading Rx FIFO.
    ///
    /// Bits `[12:8]`, mask `0x00001f00`
    pub const RX_CNT_OFFSET: u32 = 8;
    pub const RX_CNT_MASK: u32 = 0x00001f00;

    // ======== Field Accessors ========

    /// **RX_FIFO_WR_BIT_CNT** `[31:13]` (RO)
    ///
    /// Indicates the current written bit number into Rx FIFO.
    #[inline(always)]
    pub fn rx_fifo_wr_bit_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FIFO_WR_BIT_CNT_MASK) >> Self::RX_FIFO_WR_BIT_CNT_OFFSET
    }

    /// **RX_CNT** `[12:8]` (RO)
    ///
    /// Indicates the cycle number of reading Rx FIFO.
    #[inline(always)]
    pub fn rx_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_CNT_MASK) >> Self::RX_CNT_OFFSET
    }
}

/// RX_ST0 value type for batch read/modify/write operations
///
/// Use `rxst0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxst0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxSt0Val {
    bits: u32,
}

impl RxSt0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_FIFO_WR_BIT_CNT** `[31:13]`
    ///
    /// Indicates the current written bit number into Rx FIFO.
    #[inline(always)]
    pub const fn rx_fifo_wr_bit_cnt(&self) -> u32 {
        (self.bits & RxSt0::RX_FIFO_WR_BIT_CNT_MASK) >> RxSt0::RX_FIFO_WR_BIT_CNT_OFFSET
    }

    /// Read **RX_CNT** `[12:8]`
    ///
    /// Indicates the cycle number of reading Rx FIFO.
    #[inline(always)]
    pub const fn rx_cnt(&self) -> u32 {
        (self.bits & RxSt0::RX_CNT_MASK) >> RxSt0::RX_CNT_OFFSET
    }
}

impl Default for RxSt0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxSt0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxSt0Val {
        RxSt0Val::from_bits(self.value.read_bits())
    }
}

/// RX_ST1 Register
///
/// Parallel IO RX status register1
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **RX_FIFO_RD_BIT_CNT** `[31:13]` (RO)
///
///   Indicates the current read bit number from Rx FIFO.
#[repr(transparent)]
pub struct RxSt1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxSt1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxSt1 {
    // ======== Field Constants ========

    /// **RX_FIFO_RD_BIT_CNT**
    ///
    /// Indicates the current read bit number from Rx FIFO.
    ///
    /// Bits `[31:13]`, mask `0xffffe000`
    pub const RX_FIFO_RD_BIT_CNT_OFFSET: u32 = 13;
    pub const RX_FIFO_RD_BIT_CNT_MASK: u32 = 0xffffe000;

    // ======== Field Accessors ========

    /// **RX_FIFO_RD_BIT_CNT** `[31:13]` (RO)
    ///
    /// Indicates the current read bit number from Rx FIFO.
    #[inline(always)]
    pub fn rx_fifo_rd_bit_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FIFO_RD_BIT_CNT_MASK) >> Self::RX_FIFO_RD_BIT_CNT_OFFSET
    }
}

/// RX_ST1 value type for batch read/modify/write operations
///
/// Use `rxst1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxst1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxSt1Val {
    bits: u32,
}

impl RxSt1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_FIFO_RD_BIT_CNT** `[31:13]`
    ///
    /// Indicates the current read bit number from Rx FIFO.
    #[inline(always)]
    pub const fn rx_fifo_rd_bit_cnt(&self) -> u32 {
        (self.bits & RxSt1::RX_FIFO_RD_BIT_CNT_MASK) >> RxSt1::RX_FIFO_RD_BIT_CNT_OFFSET
    }
}

impl Default for RxSt1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxSt1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxSt1Val {
        RxSt1Val::from_bits(self.value.read_bits())
    }
}

/// TX_ST0 Register
///
/// Parallel IO TX status register0
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **TX_FIFO_RD_BIT_CNT** `[31:13]` (RO)
///
///   Indicates the current read bit number from Tx FIFO.
///
/// - **TX_CNT** `[12:6]` (RO)
///
///   Indicates the cycle number of reading Tx FIFO.
#[repr(transparent)]
pub struct TxSt0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxSt0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxSt0 {
    // ======== Field Constants ========

    /// **TX_FIFO_RD_BIT_CNT**
    ///
    /// Indicates the current read bit number from Tx FIFO.
    ///
    /// Bits `[31:13]`, mask `0xffffe000`
    pub const TX_FIFO_RD_BIT_CNT_OFFSET: u32 = 13;
    pub const TX_FIFO_RD_BIT_CNT_MASK: u32 = 0xffffe000;

    /// **TX_CNT**
    ///
    /// Indicates the cycle number of reading Tx FIFO.
    ///
    /// Bits `[12:6]`, mask `0x00001fc0`
    pub const TX_CNT_OFFSET: u32 = 6;
    pub const TX_CNT_MASK: u32 = 0x00001fc0;

    // ======== Field Accessors ========

    /// **TX_FIFO_RD_BIT_CNT** `[31:13]` (RO)
    ///
    /// Indicates the current read bit number from Tx FIFO.
    #[inline(always)]
    pub fn tx_fifo_rd_bit_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_FIFO_RD_BIT_CNT_MASK) >> Self::TX_FIFO_RD_BIT_CNT_OFFSET
    }

    /// **TX_CNT** `[12:6]` (RO)
    ///
    /// Indicates the cycle number of reading Tx FIFO.
    #[inline(always)]
    pub fn tx_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_CNT_MASK) >> Self::TX_CNT_OFFSET
    }
}

/// TX_ST0 value type for batch read/modify/write operations
///
/// Use `txst0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txst0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxSt0Val {
    bits: u32,
}

impl TxSt0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_FIFO_RD_BIT_CNT** `[31:13]`
    ///
    /// Indicates the current read bit number from Tx FIFO.
    #[inline(always)]
    pub const fn tx_fifo_rd_bit_cnt(&self) -> u32 {
        (self.bits & TxSt0::TX_FIFO_RD_BIT_CNT_MASK) >> TxSt0::TX_FIFO_RD_BIT_CNT_OFFSET
    }

    /// Read **TX_CNT** `[12:6]`
    ///
    /// Indicates the cycle number of reading Tx FIFO.
    #[inline(always)]
    pub const fn tx_cnt(&self) -> u32 {
        (self.bits & TxSt0::TX_CNT_MASK) >> TxSt0::TX_CNT_OFFSET
    }
}

impl Default for TxSt0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxSt0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxSt0Val {
        TxSt0Val::from_bits(self.value.read_bits())
    }
}

/// RX_CLK_CFG Register
///
/// Parallel IO RX clk configuration register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **RX_CLK_O_INV** `[31]` (RW)
///
///   Set this bit to invert the output Rx core clock.
///
/// - **RX_CLK_I_INV** `[30]` (RW)
///
///   Set this bit to invert the input Rx core clock.
#[repr(transparent)]
pub struct RxClkCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxClkCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxClkCfg {
    // ======== Field Constants ========

    /// **RX_CLK_O_INV**
    ///
    /// Set this bit to invert the output Rx core clock.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RX_CLK_O_INV_OFFSET: u32 = 31;
    pub const RX_CLK_O_INV_MASK: u32 = 0x80000000;
    pub const RX_CLK_O_INV: u32 = 1 << 31;

    /// **RX_CLK_I_INV**
    ///
    /// Set this bit to invert the input Rx core clock.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RX_CLK_I_INV_OFFSET: u32 = 30;
    pub const RX_CLK_I_INV_MASK: u32 = 0x40000000;
    pub const RX_CLK_I_INV: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **RX_CLK_O_INV** `[31]` (RW)
    ///
    /// Set this bit to invert the output Rx core clock.
    #[inline(always)]
    pub fn rx_clk_o_inv(&self) -> bool {
        (self.value.read_bits() & Self::RX_CLK_O_INV_MASK) != 0
    }

    /// Set RX_CLK_O_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_clk_o_inv(&self) {
        self.value.modify_bits(|r| r | Self::RX_CLK_O_INV_MASK);
    }

    /// Clear RX_CLK_O_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_clk_o_inv(&self) {
        self.value.modify_bits(|r| r & !Self::RX_CLK_O_INV_MASK);
    }

    /// **RX_CLK_I_INV** `[30]` (RW)
    ///
    /// Set this bit to invert the input Rx core clock.
    #[inline(always)]
    pub fn rx_clk_i_inv(&self) -> bool {
        (self.value.read_bits() & Self::RX_CLK_I_INV_MASK) != 0
    }

    /// Set RX_CLK_I_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_clk_i_inv(&self) {
        self.value.modify_bits(|r| r | Self::RX_CLK_I_INV_MASK);
    }

    /// Clear RX_CLK_I_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_clk_i_inv(&self) {
        self.value.modify_bits(|r| r & !Self::RX_CLK_I_INV_MASK);
    }
}

/// RX_CLK_CFG value type for batch read/modify/write operations
///
/// Use `rxclkcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxclkcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxClkCfgVal {
    bits: u32,
}

impl RxClkCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_CLK_O_INV** `[31:31]`
    ///
    /// Set this bit to invert the output Rx core clock.
    #[inline(always)]
    pub const fn rx_clk_o_inv(&self) -> bool {
        (self.bits & RxClkCfg::RX_CLK_O_INV_MASK) != 0
    }

    /// Read **RX_CLK_I_INV** `[30:30]`
    ///
    /// Set this bit to invert the input Rx core clock.
    #[inline(always)]
    pub const fn rx_clk_i_inv(&self) -> bool {
        (self.bits & RxClkCfg::RX_CLK_I_INV_MASK) != 0
    }

    /// Set **RX_CLK_O_INV** `[31:31]`
    ///
    /// Set this bit to invert the output Rx core clock.
    #[inline(always)]
    pub const fn with_rx_clk_o_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxClkCfg::RX_CLK_O_INV_MASK
            } else {
                self.bits & !RxClkCfg::RX_CLK_O_INV_MASK
            },
        }
    }

    /// Set **RX_CLK_I_INV** `[30:30]`
    ///
    /// Set this bit to invert the input Rx core clock.
    #[inline(always)]
    pub const fn with_rx_clk_i_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxClkCfg::RX_CLK_I_INV_MASK
            } else {
                self.bits & !RxClkCfg::RX_CLK_I_INV_MASK
            },
        }
    }
}

impl Default for RxClkCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxClkCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxClkCfgVal {
        RxClkCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxClkCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxClkCfgVal) -> RxClkCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_CLK_CFG Register
///
/// Parallel IO TX clk configuration register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **TX_CLK_O_INV** `[31]` (RW)
///
///   Set this bit to invert the output Tx core clock.
///
/// - **TX_CLK_I_INV** `[30]` (RW)
///
///   Set this bit to invert the input Tx core clock.
#[repr(transparent)]
pub struct TxClkCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxClkCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxClkCfg {
    // ======== Field Constants ========

    /// **TX_CLK_O_INV**
    ///
    /// Set this bit to invert the output Tx core clock.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TX_CLK_O_INV_OFFSET: u32 = 31;
    pub const TX_CLK_O_INV_MASK: u32 = 0x80000000;
    pub const TX_CLK_O_INV: u32 = 1 << 31;

    /// **TX_CLK_I_INV**
    ///
    /// Set this bit to invert the input Tx core clock.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TX_CLK_I_INV_OFFSET: u32 = 30;
    pub const TX_CLK_I_INV_MASK: u32 = 0x40000000;
    pub const TX_CLK_I_INV: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **TX_CLK_O_INV** `[31]` (RW)
    ///
    /// Set this bit to invert the output Tx core clock.
    #[inline(always)]
    pub fn tx_clk_o_inv(&self) -> bool {
        (self.value.read_bits() & Self::TX_CLK_O_INV_MASK) != 0
    }

    /// Set TX_CLK_O_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_clk_o_inv(&self) {
        self.value.modify_bits(|r| r | Self::TX_CLK_O_INV_MASK);
    }

    /// Clear TX_CLK_O_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_clk_o_inv(&self) {
        self.value.modify_bits(|r| r & !Self::TX_CLK_O_INV_MASK);
    }

    /// **TX_CLK_I_INV** `[30]` (RW)
    ///
    /// Set this bit to invert the input Tx core clock.
    #[inline(always)]
    pub fn tx_clk_i_inv(&self) -> bool {
        (self.value.read_bits() & Self::TX_CLK_I_INV_MASK) != 0
    }

    /// Set TX_CLK_I_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_clk_i_inv(&self) {
        self.value.modify_bits(|r| r | Self::TX_CLK_I_INV_MASK);
    }

    /// Clear TX_CLK_I_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_clk_i_inv(&self) {
        self.value.modify_bits(|r| r & !Self::TX_CLK_I_INV_MASK);
    }
}

/// TX_CLK_CFG value type for batch read/modify/write operations
///
/// Use `txclkcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txclkcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxClkCfgVal {
    bits: u32,
}

impl TxClkCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_CLK_O_INV** `[31:31]`
    ///
    /// Set this bit to invert the output Tx core clock.
    #[inline(always)]
    pub const fn tx_clk_o_inv(&self) -> bool {
        (self.bits & TxClkCfg::TX_CLK_O_INV_MASK) != 0
    }

    /// Read **TX_CLK_I_INV** `[30:30]`
    ///
    /// Set this bit to invert the input Tx core clock.
    #[inline(always)]
    pub const fn tx_clk_i_inv(&self) -> bool {
        (self.bits & TxClkCfg::TX_CLK_I_INV_MASK) != 0
    }

    /// Set **TX_CLK_O_INV** `[31:31]`
    ///
    /// Set this bit to invert the output Tx core clock.
    #[inline(always)]
    pub const fn with_tx_clk_o_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxClkCfg::TX_CLK_O_INV_MASK
            } else {
                self.bits & !TxClkCfg::TX_CLK_O_INV_MASK
            },
        }
    }

    /// Set **TX_CLK_I_INV** `[30:30]`
    ///
    /// Set this bit to invert the input Tx core clock.
    #[inline(always)]
    pub const fn with_tx_clk_i_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxClkCfg::TX_CLK_I_INV_MASK
            } else {
                self.bits & !TxClkCfg::TX_CLK_I_INV_MASK
            },
        }
    }
}

impl Default for TxClkCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxClkCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxClkCfgVal {
        TxClkCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxClkCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxClkCfgVal) -> TxClkCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK Register
///
/// Parallel IO clk configuration register
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **EN** `[31]` (RW)
///
///   Force clock on for this register file
#[repr(transparent)]
pub struct Clk {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clk {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clk {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Force clock on for this register file
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EN_OFFSET: u32 = 31;
    pub const EN_MASK: u32 = 0x80000000;
    pub const EN: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **EN** `[31]` (RW)
    ///
    /// Force clock on for this register file
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
}

/// CLK value type for batch read/modify/write operations
///
/// Use `clk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkVal {
    bits: u32,
}

impl ClkVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **EN** `[31:31]`
    ///
    /// Force clock on for this register file
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Clk::EN_MASK) != 0
    }

    /// Set **EN** `[31:31]`
    ///
    /// Force clock on for this register file
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::EN_MASK
            } else {
                self.bits & !Clk::EN_MASK
            },
        }
    }
}

impl Default for ClkVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkVal {
        ClkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkVal) -> ClkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VERSION Register
///
/// Version register.
///
/// Address offset: 0x03fc
/// Reset value: 0x02212260
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   Version of this register file
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

    /// **DATE**
    ///
    /// Version of this register file
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// Version of this register file
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// Version of this register file
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
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
        Self { bits: 0x02212260 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// Version of this register file
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Version::DATE_MASK) >> Version::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// Version of this register file
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Version::DATE_MASK)
                | ((value << Version::DATE_OFFSET) & Version::DATE_MASK),
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
    /// Parallel RX Sampling mode configuration register.
    pub rx_mode_cfg: RxModeCfg,
    /// 0x0004
    /// Parallel RX data configuration register.
    pub rx_data_cfg: RxDataCfg,
    /// 0x0008
    /// Parallel RX general configuration register.
    pub rx_genrl_cfg: RxGenrlCfg,
    /// 0x000c
    /// Parallel RX Start configuration register.
    pub rx_start_cfg: RxStartCfg,
    /// 0x0010
    /// Parallel TX data configuration register.
    pub tx_data_cfg: TxDataCfg,
    /// 0x0014
    /// Parallel TX Start configuration register.
    pub tx_start_cfg: TxStartCfg,
    /// 0x0018
    /// Parallel TX general configuration register.
    pub tx_genrl_cfg: TxGenrlCfg,
    /// 0x001c
    /// Parallel IO FIFO configuration register.
    pub fifo_cfg: FifoCfg,
    /// 0x0020
    /// Parallel IO FIFO configuration register.
    pub reg_update: RegUpdate,
    /// 0x0024
    /// Parallel IO module status register0.
    pub st: St,
    /// 0x0028
    /// Parallel IO interrupt enable singal configuration register.
    pub int_ena: IntEna,
    /// 0x002c
    /// Parallel IO interrupt raw singal status register.
    pub int_raw: IntRaw,
    /// 0x0030
    /// Parallel IO interrupt singal status register.
    pub int_st: IntSt,
    /// 0x0034
    /// Parallel IO interrupt clear singal configuration register.
    pub int_clr: IntClr,
    /// 0x0038
    /// Parallel IO RX status register0
    pub rx_st0: RxSt0,
    /// 0x003c
    /// Parallel IO RX status register1
    pub rx_st1: RxSt1,
    /// 0x0040
    /// Parallel IO TX status register0
    pub tx_st0: TxSt0,
    /// 0x0044
    /// Parallel IO RX clk configuration register
    pub rx_clk_cfg: RxClkCfg,
    /// 0x0048
    /// Parallel IO TX clk configuration register
    pub tx_clk_cfg: TxClkCfg,
    _reserved0: [u8; 212],
    /// 0x0120
    /// Parallel IO clk configuration register
    pub clk: Clk,
    _reserved1: [u8; 728],
    /// 0x03fc
    /// Version register.
    pub version: Version,
}
