//! SPI2
//! SPI (Serial Peripheral Interface) Controller 2

#![allow(dead_code)]

pub const BASE: u32 = 0x500d0000;

/// SPI_CMD Register
///
/// Command control register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SPI_USR** `[24]` (RW)
///
///   User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf.
///
/// - **SPI_UPDATE** `[23]` (WO)
///
///   Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode.
///
/// - **SPI_CONF_BITLEN** `[17:0]` (RW)
///
///   Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiCmd {
    // ======== Field Constants ========

    /// **SPI_USR**
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_USR_OFFSET: u32 = 24;
    pub const SPI_USR_MASK: u32 = 0x01000000;
    pub const SPI_USR: u32 = 1 << 24;

    /// **SPI_UPDATE**
    ///
    /// Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_UPDATE_OFFSET: u32 = 23;
    pub const SPI_UPDATE_MASK: u32 = 0x00800000;
    pub const SPI_UPDATE: u32 = 1 << 23;

    /// **SPI_CONF_BITLEN**
    ///
    /// Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const SPI_CONF_BITLEN_OFFSET: u32 = 0;
    pub const SPI_CONF_BITLEN_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **SPI_USR** `[24]` (RW)
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf.
    #[inline(always)]
    pub fn spi_usr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_MASK) != 0
    }

    /// Set SPI_USR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_MASK);
    }

    /// Clear SPI_USR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_MASK);
    }

    /// Trigger **SPI_UPDATE** (WO)
    ///
    /// Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode.
    #[inline(always)]
    pub fn trigger_spi_update(&self) {
        self.value.modify_bits(|r| r | Self::SPI_UPDATE_MASK);
    }

    /// **SPI_CONF_BITLEN** `[17:0]` (RW)
    ///
    /// Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_conf_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CONF_BITLEN_MASK) >> Self::SPI_CONF_BITLEN_OFFSET
    }

    /// Write **SPI_CONF_BITLEN** `[17:0]` (RW)
    ///
    /// Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_conf_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CONF_BITLEN_MASK)
                | ((value << Self::SPI_CONF_BITLEN_OFFSET) & Self::SPI_CONF_BITLEN_MASK)
        });
    }
}

/// SPI_CMD value type for batch read/modify/write operations
///
/// Use `spicmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spicmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiCmdVal {
    bits: u32,
}

impl SpiCmdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_USR** `[24:24]`
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf.
    #[inline(always)]
    pub const fn spi_usr(&self) -> bool {
        (self.bits & SpiCmd::SPI_USR_MASK) != 0
    }

    /// Read **SPI_CONF_BITLEN** `[17:0]`
    ///
    /// Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_conf_bitlen(&self) -> u32 {
        (self.bits & SpiCmd::SPI_CONF_BITLEN_MASK) >> SpiCmd::SPI_CONF_BITLEN_OFFSET
    }

    /// Set **SPI_USR** `[24:24]`
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf.
    #[inline(always)]
    pub const fn with_spi_usr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCmd::SPI_USR_MASK
            } else {
                self.bits & !SpiCmd::SPI_USR_MASK
            },
        }
    }

    /// Set **SPI_UPDATE** `[23:23]`
    ///
    /// Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode.
    #[inline(always)]
    pub const fn with_spi_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCmd::SPI_UPDATE_MASK
            } else {
                self.bits & !SpiCmd::SPI_UPDATE_MASK
            },
        }
    }

    /// Set **SPI_CONF_BITLEN** `[17:0]`
    ///
    /// Define the APB cycles of SPI_CONF state. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_conf_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiCmd::SPI_CONF_BITLEN_MASK)
                | ((value << SpiCmd::SPI_CONF_BITLEN_OFFSET) & SpiCmd::SPI_CONF_BITLEN_MASK),
        }
    }
}

impl Default for SpiCmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiCmdVal {
        SpiCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiCmdVal) -> SpiCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_ADDR Register
///
/// Address value register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **SPI_USR_ADDR_VALUE** `[31:0]` (RW)
///
///   Address to slave. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiAddr {
    // ======== Field Constants ========

    /// **SPI_USR_ADDR_VALUE**
    ///
    /// Address to slave. Can be configured in CONF state.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_USR_ADDR_VALUE_OFFSET: u32 = 0;
    pub const SPI_USR_ADDR_VALUE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_USR_ADDR_VALUE** `[31:0]` (RW)
    ///
    /// Address to slave. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_addr_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_USR_ADDR_VALUE_MASK) >> Self::SPI_USR_ADDR_VALUE_OFFSET
    }

    /// Write **SPI_USR_ADDR_VALUE** `[31:0]` (RW)
    ///
    /// Address to slave. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_usr_addr_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_USR_ADDR_VALUE_MASK)
                | ((value << Self::SPI_USR_ADDR_VALUE_OFFSET) & Self::SPI_USR_ADDR_VALUE_MASK)
        });
    }
}

/// SPI_ADDR value type for batch read/modify/write operations
///
/// Use `spiaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiAddrVal {
    bits: u32,
}

impl SpiAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_USR_ADDR_VALUE** `[31:0]`
    ///
    /// Address to slave. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_addr_value(&self) -> u32 {
        (self.bits & SpiAddr::SPI_USR_ADDR_VALUE_MASK) >> SpiAddr::SPI_USR_ADDR_VALUE_OFFSET
    }

    /// Set **SPI_USR_ADDR_VALUE** `[31:0]`
    ///
    /// Address to slave. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_addr_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiAddr::SPI_USR_ADDR_VALUE_MASK)
                | ((value << SpiAddr::SPI_USR_ADDR_VALUE_OFFSET)
                    & SpiAddr::SPI_USR_ADDR_VALUE_MASK),
        }
    }
}

impl Default for SpiAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiAddrVal {
        SpiAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiAddrVal) -> SpiAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_CTRL Register
///
/// SPI control register
///
/// Address offset: 0x0008
/// Reset value: 0x003c0000
///
/// ## Fields
///
/// - **SPI_WR_BIT_ORDER** `[26:25]` (RW)
///
///   In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
///
/// - **SPI_RD_BIT_ORDER** `[24:23]` (RW)
///
///   In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
///
/// - **SPI_WP_POL** `[21]` (RW)
///
///   Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
///
/// - **SPI_HOLD_POL** `[20]` (RW)
///
///   SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
///
/// - **SPI_D_POL** `[19]` (RW)
///
///   The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state.
///
/// - **SPI_Q_POL** `[18]` (RW)
///
///   The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state.
///
/// - **SPI_FREAD_OCT** `[16]` (RW)
///
///   In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FREAD_QUAD** `[15]` (RW)
///
///   In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FREAD_DUAL** `[14]` (RW)
///
///   In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FCMD_OCT** `[10]` (RW)
///
///   Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FCMD_QUAD** `[9]` (RW)
///
///   Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FCMD_DUAL** `[8]` (RW)
///
///   Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FADDR_OCT** `[7]` (RW)
///
///   Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FADDR_QUAD** `[6]` (RW)
///
///   Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_FADDR_DUAL** `[5]` (RW)
///
///   Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_DUMMY_OUT** `[3]` (RW)
///
///   0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiCtrl {
    // ======== Field Constants ========

    /// **SPI_WR_BIT_ORDER**
    ///
    /// In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
    ///
    /// Bits `[26:25]`, mask `0x06000000`
    pub const SPI_WR_BIT_ORDER_OFFSET: u32 = 25;
    pub const SPI_WR_BIT_ORDER_MASK: u32 = 0x06000000;

    /// **SPI_RD_BIT_ORDER**
    ///
    /// In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
    ///
    /// Bits `[24:23]`, mask `0x01800000`
    pub const SPI_RD_BIT_ORDER_OFFSET: u32 = 23;
    pub const SPI_RD_BIT_ORDER_MASK: u32 = 0x01800000;

    /// **SPI_WP_POL**
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_WP_POL_OFFSET: u32 = 21;
    pub const SPI_WP_POL_MASK: u32 = 0x00200000;
    pub const SPI_WP_POL: u32 = 1 << 21;

    /// **SPI_HOLD_POL**
    ///
    /// SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_HOLD_POL_OFFSET: u32 = 20;
    pub const SPI_HOLD_POL_MASK: u32 = 0x00100000;
    pub const SPI_HOLD_POL: u32 = 1 << 20;

    /// **SPI_D_POL**
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_D_POL_OFFSET: u32 = 19;
    pub const SPI_D_POL_MASK: u32 = 0x00080000;
    pub const SPI_D_POL: u32 = 1 << 19;

    /// **SPI_Q_POL**
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_Q_POL_OFFSET: u32 = 18;
    pub const SPI_Q_POL_MASK: u32 = 0x00040000;
    pub const SPI_Q_POL: u32 = 1 << 18;

    /// **SPI_FREAD_OCT**
    ///
    /// In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_FREAD_OCT_OFFSET: u32 = 16;
    pub const SPI_FREAD_OCT_MASK: u32 = 0x00010000;
    pub const SPI_FREAD_OCT: u32 = 1 << 16;

    /// **SPI_FREAD_QUAD**
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_FREAD_QUAD_OFFSET: u32 = 15;
    pub const SPI_FREAD_QUAD_MASK: u32 = 0x00008000;
    pub const SPI_FREAD_QUAD: u32 = 1 << 15;

    /// **SPI_FREAD_DUAL**
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_FREAD_DUAL_OFFSET: u32 = 14;
    pub const SPI_FREAD_DUAL_MASK: u32 = 0x00004000;
    pub const SPI_FREAD_DUAL: u32 = 1 << 14;

    /// **SPI_FCMD_OCT**
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_FCMD_OCT_OFFSET: u32 = 10;
    pub const SPI_FCMD_OCT_MASK: u32 = 0x00000400;
    pub const SPI_FCMD_OCT: u32 = 1 << 10;

    /// **SPI_FCMD_QUAD**
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_FCMD_QUAD_OFFSET: u32 = 9;
    pub const SPI_FCMD_QUAD_MASK: u32 = 0x00000200;
    pub const SPI_FCMD_QUAD: u32 = 1 << 9;

    /// **SPI_FCMD_DUAL**
    ///
    /// Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_FCMD_DUAL_OFFSET: u32 = 8;
    pub const SPI_FCMD_DUAL_MASK: u32 = 0x00000100;
    pub const SPI_FCMD_DUAL: u32 = 1 << 8;

    /// **SPI_FADDR_OCT**
    ///
    /// Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_FADDR_OCT_OFFSET: u32 = 7;
    pub const SPI_FADDR_OCT_MASK: u32 = 0x00000080;
    pub const SPI_FADDR_OCT: u32 = 1 << 7;

    /// **SPI_FADDR_QUAD**
    ///
    /// Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_FADDR_QUAD_OFFSET: u32 = 6;
    pub const SPI_FADDR_QUAD_MASK: u32 = 0x00000040;
    pub const SPI_FADDR_QUAD: u32 = 1 << 6;

    /// **SPI_FADDR_DUAL**
    ///
    /// Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_FADDR_DUAL_OFFSET: u32 = 5;
    pub const SPI_FADDR_DUAL_MASK: u32 = 0x00000020;
    pub const SPI_FADDR_DUAL: u32 = 1 << 5;

    /// **SPI_DUMMY_OUT**
    ///
    /// 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_DUMMY_OUT_OFFSET: u32 = 3;
    pub const SPI_DUMMY_OUT_MASK: u32 = 0x00000008;
    pub const SPI_DUMMY_OUT: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **SPI_WR_BIT_ORDER** `[26:25]` (RW)
    ///
    /// In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_WR_BIT_ORDER_MASK) >> Self::SPI_WR_BIT_ORDER_OFFSET
    }

    /// Write **SPI_WR_BIT_ORDER** `[26:25]` (RW)
    ///
    /// In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_wr_bit_order(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_WR_BIT_ORDER_MASK)
                | ((value << Self::SPI_WR_BIT_ORDER_OFFSET) & Self::SPI_WR_BIT_ORDER_MASK)
        });
    }

    /// **SPI_RD_BIT_ORDER** `[24:23]` (RW)
    ///
    /// In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_RD_BIT_ORDER_MASK) >> Self::SPI_RD_BIT_ORDER_OFFSET
    }

    /// Write **SPI_RD_BIT_ORDER** `[24:23]` (RW)
    ///
    /// In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_rd_bit_order(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_RD_BIT_ORDER_MASK)
                | ((value << Self::SPI_RD_BIT_ORDER_OFFSET) & Self::SPI_RD_BIT_ORDER_MASK)
        });
    }

    /// **SPI_WP_POL** `[21]` (RW)
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_wp_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_WP_POL_MASK) != 0
    }

    /// Set SPI_WP_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_wp_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_WP_POL_MASK);
    }

    /// Clear SPI_WP_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_wp_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_WP_POL_MASK);
    }

    /// **SPI_HOLD_POL** `[20]` (RW)
    ///
    /// SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_hold_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_HOLD_POL_MASK) != 0
    }

    /// Set SPI_HOLD_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_hold_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_HOLD_POL_MASK);
    }

    /// Clear SPI_HOLD_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_hold_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_HOLD_POL_MASK);
    }

    /// **SPI_D_POL** `[19]` (RW)
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_d_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_D_POL_MASK) != 0
    }

    /// Set SPI_D_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_d_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_D_POL_MASK);
    }

    /// Clear SPI_D_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_d_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_D_POL_MASK);
    }

    /// **SPI_Q_POL** `[18]` (RW)
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_q_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_Q_POL_MASK) != 0
    }

    /// Set SPI_Q_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_q_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_Q_POL_MASK);
    }

    /// Clear SPI_Q_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_q_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_Q_POL_MASK);
    }

    /// **SPI_FREAD_OCT** `[16]` (RW)
    ///
    /// In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fread_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FREAD_OCT_MASK) != 0
    }

    /// Set SPI_FREAD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fread_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FREAD_OCT_MASK);
    }

    /// Clear SPI_FREAD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fread_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FREAD_OCT_MASK);
    }

    /// **SPI_FREAD_QUAD** `[15]` (RW)
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fread_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FREAD_QUAD_MASK) != 0
    }

    /// Set SPI_FREAD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fread_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FREAD_QUAD_MASK);
    }

    /// Clear SPI_FREAD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fread_quad(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FREAD_QUAD_MASK);
    }

    /// **SPI_FREAD_DUAL** `[14]` (RW)
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fread_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FREAD_DUAL_MASK) != 0
    }

    /// Set SPI_FREAD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fread_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FREAD_DUAL_MASK);
    }

    /// Clear SPI_FREAD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fread_dual(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FREAD_DUAL_MASK);
    }

    /// **SPI_FCMD_OCT** `[10]` (RW)
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fcmd_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FCMD_OCT_MASK) != 0
    }

    /// Set SPI_FCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fcmd_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FCMD_OCT_MASK);
    }

    /// Clear SPI_FCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fcmd_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FCMD_OCT_MASK);
    }

    /// **SPI_FCMD_QUAD** `[9]` (RW)
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fcmd_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FCMD_QUAD_MASK) != 0
    }

    /// Set SPI_FCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fcmd_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FCMD_QUAD_MASK);
    }

    /// Clear SPI_FCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fcmd_quad(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FCMD_QUAD_MASK);
    }

    /// **SPI_FCMD_DUAL** `[8]` (RW)
    ///
    /// Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fcmd_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FCMD_DUAL_MASK) != 0
    }

    /// Set SPI_FCMD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fcmd_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FCMD_DUAL_MASK);
    }

    /// Clear SPI_FCMD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fcmd_dual(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FCMD_DUAL_MASK);
    }

    /// **SPI_FADDR_OCT** `[7]` (RW)
    ///
    /// Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_faddr_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FADDR_OCT_MASK) != 0
    }

    /// Set SPI_FADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_faddr_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FADDR_OCT_MASK);
    }

    /// Clear SPI_FADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_faddr_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FADDR_OCT_MASK);
    }

    /// **SPI_FADDR_QUAD** `[6]` (RW)
    ///
    /// Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_faddr_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FADDR_QUAD_MASK) != 0
    }

    /// Set SPI_FADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_faddr_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FADDR_QUAD_MASK);
    }

    /// Clear SPI_FADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_faddr_quad(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FADDR_QUAD_MASK);
    }

    /// **SPI_FADDR_DUAL** `[5]` (RW)
    ///
    /// Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_faddr_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FADDR_DUAL_MASK) != 0
    }

    /// Set SPI_FADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_faddr_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FADDR_DUAL_MASK);
    }

    /// Clear SPI_FADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_faddr_dual(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FADDR_DUAL_MASK);
    }

    /// **SPI_DUMMY_OUT** `[3]` (RW)
    ///
    /// 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dummy_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DUMMY_OUT_MASK) != 0
    }

    /// Set SPI_DUMMY_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dummy_out(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DUMMY_OUT_MASK);
    }

    /// Clear SPI_DUMMY_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dummy_out(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DUMMY_OUT_MASK);
    }
}

/// SPI_CTRL value type for batch read/modify/write operations
///
/// Use `spictrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spictrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiCtrlVal {
    bits: u32,
}

impl SpiCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x003c0000 }
    }

    /// Read **SPI_WR_BIT_ORDER** `[26:25]`
    ///
    /// In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_wr_bit_order(&self) -> u32 {
        (self.bits & SpiCtrl::SPI_WR_BIT_ORDER_MASK) >> SpiCtrl::SPI_WR_BIT_ORDER_OFFSET
    }

    /// Read **SPI_RD_BIT_ORDER** `[24:23]`
    ///
    /// In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_rd_bit_order(&self) -> u32 {
        (self.bits & SpiCtrl::SPI_RD_BIT_ORDER_MASK) >> SpiCtrl::SPI_RD_BIT_ORDER_OFFSET
    }

    /// Read **SPI_WP_POL** `[21:21]`
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_wp_pol(&self) -> bool {
        (self.bits & SpiCtrl::SPI_WP_POL_MASK) != 0
    }

    /// Read **SPI_HOLD_POL** `[20:20]`
    ///
    /// SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_hold_pol(&self) -> bool {
        (self.bits & SpiCtrl::SPI_HOLD_POL_MASK) != 0
    }

    /// Read **SPI_D_POL** `[19:19]`
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_d_pol(&self) -> bool {
        (self.bits & SpiCtrl::SPI_D_POL_MASK) != 0
    }

    /// Read **SPI_Q_POL** `[18:18]`
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_q_pol(&self) -> bool {
        (self.bits & SpiCtrl::SPI_Q_POL_MASK) != 0
    }

    /// Read **SPI_FREAD_OCT** `[16:16]`
    ///
    /// In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fread_oct(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FREAD_OCT_MASK) != 0
    }

    /// Read **SPI_FREAD_QUAD** `[15:15]`
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fread_quad(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FREAD_QUAD_MASK) != 0
    }

    /// Read **SPI_FREAD_DUAL** `[14:14]`
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fread_dual(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FREAD_DUAL_MASK) != 0
    }

    /// Read **SPI_FCMD_OCT** `[10:10]`
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fcmd_oct(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FCMD_OCT_MASK) != 0
    }

    /// Read **SPI_FCMD_QUAD** `[9:9]`
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fcmd_quad(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FCMD_QUAD_MASK) != 0
    }

    /// Read **SPI_FCMD_DUAL** `[8:8]`
    ///
    /// Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fcmd_dual(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FCMD_DUAL_MASK) != 0
    }

    /// Read **SPI_FADDR_OCT** `[7:7]`
    ///
    /// Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_faddr_oct(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FADDR_OCT_MASK) != 0
    }

    /// Read **SPI_FADDR_QUAD** `[6:6]`
    ///
    /// Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_faddr_quad(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FADDR_QUAD_MASK) != 0
    }

    /// Read **SPI_FADDR_DUAL** `[5:5]`
    ///
    /// Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_faddr_dual(&self) -> bool {
        (self.bits & SpiCtrl::SPI_FADDR_DUAL_MASK) != 0
    }

    /// Read **SPI_DUMMY_OUT** `[3:3]`
    ///
    /// 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dummy_out(&self) -> bool {
        (self.bits & SpiCtrl::SPI_DUMMY_OUT_MASK) != 0
    }

    /// Set **SPI_WR_BIT_ORDER** `[26:25]`
    ///
    /// In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_wr_bit_order(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiCtrl::SPI_WR_BIT_ORDER_MASK)
                | ((value << SpiCtrl::SPI_WR_BIT_ORDER_OFFSET) & SpiCtrl::SPI_WR_BIT_ORDER_MASK),
        }
    }

    /// Set **SPI_RD_BIT_ORDER** `[24:23]`
    ///
    /// In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_rd_bit_order(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiCtrl::SPI_RD_BIT_ORDER_MASK)
                | ((value << SpiCtrl::SPI_RD_BIT_ORDER_OFFSET) & SpiCtrl::SPI_RD_BIT_ORDER_MASK),
        }
    }

    /// Set **SPI_WP_POL** `[21:21]`
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_wp_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_WP_POL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_WP_POL_MASK
            },
        }
    }

    /// Set **SPI_HOLD_POL** `[20:20]`
    ///
    /// SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_hold_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_HOLD_POL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_HOLD_POL_MASK
            },
        }
    }

    /// Set **SPI_D_POL** `[19:19]`
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_d_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_D_POL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_D_POL_MASK
            },
        }
    }

    /// Set **SPI_Q_POL** `[18:18]`
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_q_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_Q_POL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_Q_POL_MASK
            },
        }
    }

    /// Set **SPI_FREAD_OCT** `[16:16]`
    ///
    /// In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fread_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FREAD_OCT_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FREAD_OCT_MASK
            },
        }
    }

    /// Set **SPI_FREAD_QUAD** `[15:15]`
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fread_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FREAD_QUAD_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FREAD_QUAD_MASK
            },
        }
    }

    /// Set **SPI_FREAD_DUAL** `[14:14]`
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fread_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FREAD_DUAL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FREAD_DUAL_MASK
            },
        }
    }

    /// Set **SPI_FCMD_OCT** `[10:10]`
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fcmd_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FCMD_OCT_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FCMD_OCT_MASK
            },
        }
    }

    /// Set **SPI_FCMD_QUAD** `[9:9]`
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fcmd_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FCMD_QUAD_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FCMD_QUAD_MASK
            },
        }
    }

    /// Set **SPI_FCMD_DUAL** `[8:8]`
    ///
    /// Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fcmd_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FCMD_DUAL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FCMD_DUAL_MASK
            },
        }
    }

    /// Set **SPI_FADDR_OCT** `[7:7]`
    ///
    /// Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_faddr_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FADDR_OCT_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FADDR_OCT_MASK
            },
        }
    }

    /// Set **SPI_FADDR_QUAD** `[6:6]`
    ///
    /// Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_faddr_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FADDR_QUAD_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FADDR_QUAD_MASK
            },
        }
    }

    /// Set **SPI_FADDR_DUAL** `[5:5]`
    ///
    /// Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_faddr_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_FADDR_DUAL_MASK
            } else {
                self.bits & !SpiCtrl::SPI_FADDR_DUAL_MASK
            },
        }
    }

    /// Set **SPI_DUMMY_OUT** `[3:3]`
    ///
    /// 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dummy_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiCtrl::SPI_DUMMY_OUT_MASK
            } else {
                self.bits & !SpiCtrl::SPI_DUMMY_OUT_MASK
            },
        }
    }
}

impl Default for SpiCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiCtrlVal {
        SpiCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiCtrlVal) -> SpiCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_CLOCK Register
///
/// SPI clock control register
///
/// Address offset: 0x000c
/// Reset value: 0x80003043
///
/// ## Fields
///
/// - **SPI_CLK_EQU_SYSCLK** `[31]` (RW)
///
///   In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state.
///
/// - **SPI_CLKDIV_PRE** `[21:18]` (RW)
///
///   In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
///
/// - **SPI_CLKCNT_N** `[17:12]` (RW)
///
///   In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
///
/// - **SPI_CLKCNT_H** `[11:6]` (RW)
///
///   In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
///
/// - **SPI_CLKCNT_L** `[5:0]` (RW)
///
///   In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiClock {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiClock {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiClock {
    // ======== Field Constants ========

    /// **SPI_CLK_EQU_SYSCLK**
    ///
    /// In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_CLK_EQU_SYSCLK_OFFSET: u32 = 31;
    pub const SPI_CLK_EQU_SYSCLK_MASK: u32 = 0x80000000;
    pub const SPI_CLK_EQU_SYSCLK: u32 = 1 << 31;

    /// **SPI_CLKDIV_PRE**
    ///
    /// In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[21:18]`, mask `0x003c0000`
    pub const SPI_CLKDIV_PRE_OFFSET: u32 = 18;
    pub const SPI_CLKDIV_PRE_MASK: u32 = 0x003c0000;

    /// **SPI_CLKCNT_N**
    ///
    /// In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
    ///
    /// Bits `[17:12]`, mask `0x0003f000`
    pub const SPI_CLKCNT_N_OFFSET: u32 = 12;
    pub const SPI_CLKCNT_N_MASK: u32 = 0x0003f000;

    /// **SPI_CLKCNT_H**
    ///
    /// In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
    ///
    /// Bits `[11:6]`, mask `0x00000fc0`
    pub const SPI_CLKCNT_H_OFFSET: u32 = 6;
    pub const SPI_CLKCNT_H_MASK: u32 = 0x00000fc0;

    /// **SPI_CLKCNT_L**
    ///
    /// In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SPI_CLKCNT_L_OFFSET: u32 = 0;
    pub const SPI_CLKCNT_L_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **SPI_CLK_EQU_SYSCLK** `[31]` (RW)
    ///
    /// In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Set SPI_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CLK_EQU_SYSCLK_MASK);
    }

    /// Clear SPI_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CLK_EQU_SYSCLK_MASK);
    }

    /// **SPI_CLKDIV_PRE** `[21:18]` (RW)
    ///
    /// In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clkdiv_pre(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CLKDIV_PRE_MASK) >> Self::SPI_CLKDIV_PRE_OFFSET
    }

    /// Write **SPI_CLKDIV_PRE** `[21:18]` (RW)
    ///
    /// In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_clkdiv_pre(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CLKDIV_PRE_MASK)
                | ((value << Self::SPI_CLKDIV_PRE_OFFSET) & Self::SPI_CLKDIV_PRE_MASK)
        });
    }

    /// **SPI_CLKCNT_N** `[17:12]` (RW)
    ///
    /// In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clkcnt_n(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CLKCNT_N_MASK) >> Self::SPI_CLKCNT_N_OFFSET
    }

    /// Write **SPI_CLKCNT_N** `[17:12]` (RW)
    ///
    /// In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_clkcnt_n(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CLKCNT_N_MASK)
                | ((value << Self::SPI_CLKCNT_N_OFFSET) & Self::SPI_CLKCNT_N_MASK)
        });
    }

    /// **SPI_CLKCNT_H** `[11:6]` (RW)
    ///
    /// In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clkcnt_h(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CLKCNT_H_MASK) >> Self::SPI_CLKCNT_H_OFFSET
    }

    /// Write **SPI_CLKCNT_H** `[11:6]` (RW)
    ///
    /// In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_clkcnt_h(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CLKCNT_H_MASK)
                | ((value << Self::SPI_CLKCNT_H_OFFSET) & Self::SPI_CLKCNT_H_MASK)
        });
    }

    /// **SPI_CLKCNT_L** `[5:0]` (RW)
    ///
    /// In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clkcnt_l(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CLKCNT_L_MASK) >> Self::SPI_CLKCNT_L_OFFSET
    }

    /// Write **SPI_CLKCNT_L** `[5:0]` (RW)
    ///
    /// In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_clkcnt_l(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CLKCNT_L_MASK)
                | ((value << Self::SPI_CLKCNT_L_OFFSET) & Self::SPI_CLKCNT_L_MASK)
        });
    }
}

/// SPI_CLOCK value type for batch read/modify/write operations
///
/// Use `spiclock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiclock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiClockVal {
    bits: u32,
}

impl SpiClockVal {
    /// Create from raw bits
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
        Self { bits: 0x80003043 }
    }

    /// Read **SPI_CLK_EQU_SYSCLK** `[31:31]`
    ///
    /// In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clk_equ_sysclk(&self) -> bool {
        (self.bits & SpiClock::SPI_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Read **SPI_CLKDIV_PRE** `[21:18]`
    ///
    /// In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clkdiv_pre(&self) -> u32 {
        (self.bits & SpiClock::SPI_CLKDIV_PRE_MASK) >> SpiClock::SPI_CLKDIV_PRE_OFFSET
    }

    /// Read **SPI_CLKCNT_N** `[17:12]`
    ///
    /// In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clkcnt_n(&self) -> u32 {
        (self.bits & SpiClock::SPI_CLKCNT_N_MASK) >> SpiClock::SPI_CLKCNT_N_OFFSET
    }

    /// Read **SPI_CLKCNT_H** `[11:6]`
    ///
    /// In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clkcnt_h(&self) -> u32 {
        (self.bits & SpiClock::SPI_CLKCNT_H_MASK) >> SpiClock::SPI_CLKCNT_H_OFFSET
    }

    /// Read **SPI_CLKCNT_L** `[5:0]`
    ///
    /// In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clkcnt_l(&self) -> u32 {
        (self.bits & SpiClock::SPI_CLKCNT_L_MASK) >> SpiClock::SPI_CLKCNT_L_OFFSET
    }

    /// Set **SPI_CLK_EQU_SYSCLK** `[31:31]`
    ///
    /// In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clk_equ_sysclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiClock::SPI_CLK_EQU_SYSCLK_MASK
            } else {
                self.bits & !SpiClock::SPI_CLK_EQU_SYSCLK_MASK
            },
        }
    }

    /// Set **SPI_CLKDIV_PRE** `[21:18]`
    ///
    /// In the master mode it is pre-divider of spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clkdiv_pre(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiClock::SPI_CLKDIV_PRE_MASK)
                | ((value << SpiClock::SPI_CLKDIV_PRE_OFFSET) & SpiClock::SPI_CLKDIV_PRE_MASK),
        }
    }

    /// Set **SPI_CLKCNT_N** `[17:12]`
    ///
    /// In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clkcnt_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiClock::SPI_CLKCNT_N_MASK)
                | ((value << SpiClock::SPI_CLKCNT_N_OFFSET) & SpiClock::SPI_CLKCNT_N_MASK),
        }
    }

    /// Set **SPI_CLKCNT_H** `[11:6]`
    ///
    /// In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clkcnt_h(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiClock::SPI_CLKCNT_H_MASK)
                | ((value << SpiClock::SPI_CLKCNT_H_OFFSET) & SpiClock::SPI_CLKCNT_H_MASK),
        }
    }

    /// Set **SPI_CLKCNT_L** `[5:0]`
    ///
    /// In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clkcnt_l(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiClock::SPI_CLKCNT_L_MASK)
                | ((value << SpiClock::SPI_CLKCNT_L_OFFSET) & SpiClock::SPI_CLKCNT_L_MASK),
        }
    }
}

impl Default for SpiClockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiClock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiClockVal {
        SpiClockVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiClockVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiClockVal) -> SpiClockVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_USER Register
///
/// SPI USER control register
///
/// Address offset: 0x0010
/// Reset value: 0x800000c0
///
/// ## Fields
///
/// - **SPI_USR_COMMAND** `[31]` (RW)
///
///   This bit enable the command phase of an operation. Can be configured in CONF state.
///
/// - **SPI_USR_ADDR** `[30]` (RW)
///
///   This bit enable the address phase of an operation. Can be configured in CONF state.
///
/// - **SPI_USR_DUMMY** `[29]` (RW)
///
///   This bit enable the dummy phase of an operation. Can be configured in CONF state.
///
/// - **SPI_USR_MISO** `[28]` (RW)
///
///   This bit enable the read-data phase of an operation. Can be configured in CONF state.
///
/// - **SPI_USR_MOSI** `[27]` (RW)
///
///   This bit enable the write-data phase of an operation. Can be configured in CONF state.
///
/// - **SPI_USR_DUMMY_IDLE** `[26]` (RW)
///
///   spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state.
///
/// - **SPI_USR_MOSI_HIGHPART** `[25]` (RW)
///
///   write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_USR_MISO_HIGHPART** `[24]` (RW)
///
///   read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_SIO** `[17]` (RW)
///
///   Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_USR_CONF_NXT** `[15]` (RW)
///
///   1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state.
///
/// - **SPI_FWRITE_OCT** `[14]` (RW)
///
///   In the write operations read-data phase apply 8 signals. Can be configured in CONF state.
///
/// - **SPI_FWRITE_QUAD** `[13]` (RW)
///
///   In the write operations read-data phase apply 4 signals. Can be configured in CONF state.
///
/// - **SPI_FWRITE_DUAL** `[12]` (RW)
///
///   In the write operations read-data phase apply 2 signals. Can be configured in CONF state.
///
/// - **SPI_CK_OUT_EDGE** `[9]` (RW)
///
///   the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state.
///
/// - **SPI_RSCK_I_EDGE** `[8]` (RW)
///
///   In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i.
///
/// - **SPI_CS_SETUP** `[7]` (RW)
///
///   spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_CS_HOLD** `[6]` (RW)
///
///   spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state.
///
/// - **SPI_TSCK_I_EDGE** `[5]` (RW)
///
///   In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i.
///
/// - **SPI_OPI_MODE** `[4]` (RW)
///
///   Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state.
///
/// - **SPI_QPI_MODE** `[3]` (RW)
///
///   Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state.
///
/// - **SPI_DOUTDIN** `[0]` (RW)
///
///   Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiUser {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiUser {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiUser {
    // ======== Field Constants ========

    /// **SPI_USR_COMMAND**
    ///
    /// This bit enable the command phase of an operation. Can be configured in CONF state.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_USR_COMMAND_OFFSET: u32 = 31;
    pub const SPI_USR_COMMAND_MASK: u32 = 0x80000000;
    pub const SPI_USR_COMMAND: u32 = 1 << 31;

    /// **SPI_USR_ADDR**
    ///
    /// This bit enable the address phase of an operation. Can be configured in CONF state.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_USR_ADDR_OFFSET: u32 = 30;
    pub const SPI_USR_ADDR_MASK: u32 = 0x40000000;
    pub const SPI_USR_ADDR: u32 = 1 << 30;

    /// **SPI_USR_DUMMY**
    ///
    /// This bit enable the dummy phase of an operation. Can be configured in CONF state.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_USR_DUMMY_OFFSET: u32 = 29;
    pub const SPI_USR_DUMMY_MASK: u32 = 0x20000000;
    pub const SPI_USR_DUMMY: u32 = 1 << 29;

    /// **SPI_USR_MISO**
    ///
    /// This bit enable the read-data phase of an operation. Can be configured in CONF state.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_USR_MISO_OFFSET: u32 = 28;
    pub const SPI_USR_MISO_MASK: u32 = 0x10000000;
    pub const SPI_USR_MISO: u32 = 1 << 28;

    /// **SPI_USR_MOSI**
    ///
    /// This bit enable the write-data phase of an operation. Can be configured in CONF state.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_USR_MOSI_OFFSET: u32 = 27;
    pub const SPI_USR_MOSI_MASK: u32 = 0x08000000;
    pub const SPI_USR_MOSI: u32 = 1 << 27;

    /// **SPI_USR_DUMMY_IDLE**
    ///
    /// spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_USR_DUMMY_IDLE_OFFSET: u32 = 26;
    pub const SPI_USR_DUMMY_IDLE_MASK: u32 = 0x04000000;
    pub const SPI_USR_DUMMY_IDLE: u32 = 1 << 26;

    /// **SPI_USR_MOSI_HIGHPART**
    ///
    /// write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SPI_USR_MOSI_HIGHPART_OFFSET: u32 = 25;
    pub const SPI_USR_MOSI_HIGHPART_MASK: u32 = 0x02000000;
    pub const SPI_USR_MOSI_HIGHPART: u32 = 1 << 25;

    /// **SPI_USR_MISO_HIGHPART**
    ///
    /// read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_USR_MISO_HIGHPART_OFFSET: u32 = 24;
    pub const SPI_USR_MISO_HIGHPART_MASK: u32 = 0x01000000;
    pub const SPI_USR_MISO_HIGHPART: u32 = 1 << 24;

    /// **SPI_SIO**
    ///
    /// Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_SIO_OFFSET: u32 = 17;
    pub const SPI_SIO_MASK: u32 = 0x00020000;
    pub const SPI_SIO: u32 = 1 << 17;

    /// **SPI_USR_CONF_NXT**
    ///
    /// 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_USR_CONF_NXT_OFFSET: u32 = 15;
    pub const SPI_USR_CONF_NXT_MASK: u32 = 0x00008000;
    pub const SPI_USR_CONF_NXT: u32 = 1 << 15;

    /// **SPI_FWRITE_OCT**
    ///
    /// In the write operations read-data phase apply 8 signals. Can be configured in CONF state.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_FWRITE_OCT_OFFSET: u32 = 14;
    pub const SPI_FWRITE_OCT_MASK: u32 = 0x00004000;
    pub const SPI_FWRITE_OCT: u32 = 1 << 14;

    /// **SPI_FWRITE_QUAD**
    ///
    /// In the write operations read-data phase apply 4 signals. Can be configured in CONF state.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_FWRITE_QUAD_OFFSET: u32 = 13;
    pub const SPI_FWRITE_QUAD_MASK: u32 = 0x00002000;
    pub const SPI_FWRITE_QUAD: u32 = 1 << 13;

    /// **SPI_FWRITE_DUAL**
    ///
    /// In the write operations read-data phase apply 2 signals. Can be configured in CONF state.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_FWRITE_DUAL_OFFSET: u32 = 12;
    pub const SPI_FWRITE_DUAL_MASK: u32 = 0x00001000;
    pub const SPI_FWRITE_DUAL: u32 = 1 << 12;

    /// **SPI_CK_OUT_EDGE**
    ///
    /// the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_CK_OUT_EDGE_OFFSET: u32 = 9;
    pub const SPI_CK_OUT_EDGE_MASK: u32 = 0x00000200;
    pub const SPI_CK_OUT_EDGE: u32 = 1 << 9;

    /// **SPI_RSCK_I_EDGE**
    ///
    /// In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_RSCK_I_EDGE_OFFSET: u32 = 8;
    pub const SPI_RSCK_I_EDGE_MASK: u32 = 0x00000100;
    pub const SPI_RSCK_I_EDGE: u32 = 1 << 8;

    /// **SPI_CS_SETUP**
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_CS_SETUP_OFFSET: u32 = 7;
    pub const SPI_CS_SETUP_MASK: u32 = 0x00000080;
    pub const SPI_CS_SETUP: u32 = 1 << 7;

    /// **SPI_CS_HOLD**
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_CS_HOLD_OFFSET: u32 = 6;
    pub const SPI_CS_HOLD_MASK: u32 = 0x00000040;
    pub const SPI_CS_HOLD: u32 = 1 << 6;

    /// **SPI_TSCK_I_EDGE**
    ///
    /// In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_TSCK_I_EDGE_OFFSET: u32 = 5;
    pub const SPI_TSCK_I_EDGE_MASK: u32 = 0x00000020;
    pub const SPI_TSCK_I_EDGE: u32 = 1 << 5;

    /// **SPI_OPI_MODE**
    ///
    /// Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_OPI_MODE_OFFSET: u32 = 4;
    pub const SPI_OPI_MODE_MASK: u32 = 0x00000010;
    pub const SPI_OPI_MODE: u32 = 1 << 4;

    /// **SPI_QPI_MODE**
    ///
    /// Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_QPI_MODE_OFFSET: u32 = 3;
    pub const SPI_QPI_MODE_MASK: u32 = 0x00000008;
    pub const SPI_QPI_MODE: u32 = 1 << 3;

    /// **SPI_DOUTDIN**
    ///
    /// Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DOUTDIN_OFFSET: u32 = 0;
    pub const SPI_DOUTDIN_MASK: u32 = 0x00000001;
    pub const SPI_DOUTDIN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_USR_COMMAND** `[31]` (RW)
    ///
    /// This bit enable the command phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_command(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_COMMAND_MASK) != 0
    }

    /// Set SPI_USR_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_command(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_COMMAND_MASK);
    }

    /// Clear SPI_USR_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_command(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_COMMAND_MASK);
    }

    /// **SPI_USR_ADDR** `[30]` (RW)
    ///
    /// This bit enable the address phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_ADDR_MASK) != 0
    }

    /// Set SPI_USR_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_addr(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_ADDR_MASK);
    }

    /// Clear SPI_USR_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_addr(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_ADDR_MASK);
    }

    /// **SPI_USR_DUMMY** `[29]` (RW)
    ///
    /// This bit enable the dummy phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_DUMMY_MASK) != 0
    }

    /// Set SPI_USR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_dummy(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_DUMMY_MASK);
    }

    /// Clear SPI_USR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_dummy(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_DUMMY_MASK);
    }

    /// **SPI_USR_MISO** `[28]` (RW)
    ///
    /// This bit enable the read-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_MISO_MASK) != 0
    }

    /// Set SPI_USR_MISO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_miso(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_MISO_MASK);
    }

    /// Clear SPI_USR_MISO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_miso(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_MISO_MASK);
    }

    /// **SPI_USR_MOSI** `[27]` (RW)
    ///
    /// This bit enable the write-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_MOSI_MASK) != 0
    }

    /// Set SPI_USR_MOSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_mosi(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_MOSI_MASK);
    }

    /// Clear SPI_USR_MOSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_mosi(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_MOSI_MASK);
    }

    /// **SPI_USR_DUMMY_IDLE** `[26]` (RW)
    ///
    /// spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_DUMMY_IDLE_MASK) != 0
    }

    /// Set SPI_USR_DUMMY_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_dummy_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_USR_DUMMY_IDLE_MASK);
    }

    /// Clear SPI_USR_DUMMY_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_dummy_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_USR_DUMMY_IDLE_MASK);
    }

    /// **SPI_USR_MOSI_HIGHPART** `[25]` (RW)
    ///
    /// write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_MOSI_HIGHPART_MASK) != 0
    }

    /// Set SPI_USR_MOSI_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_mosi_highpart(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_USR_MOSI_HIGHPART_MASK);
    }

    /// Clear SPI_USR_MOSI_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_mosi_highpart(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_USR_MOSI_HIGHPART_MASK);
    }

    /// **SPI_USR_MISO_HIGHPART** `[24]` (RW)
    ///
    /// read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_MISO_HIGHPART_MASK) != 0
    }

    /// Set SPI_USR_MISO_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_miso_highpart(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_USR_MISO_HIGHPART_MASK);
    }

    /// Clear SPI_USR_MISO_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_miso_highpart(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_USR_MISO_HIGHPART_MASK);
    }

    /// **SPI_SIO** `[17]` (RW)
    ///
    /// Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_sio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SIO_MASK) != 0
    }

    /// Set SPI_SIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_sio(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SIO_MASK);
    }

    /// Clear SPI_SIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_sio(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SIO_MASK);
    }

    /// **SPI_USR_CONF_NXT** `[15]` (RW)
    ///
    /// 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_CONF_NXT_MASK) != 0
    }

    /// Set SPI_USR_CONF_NXT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_conf_nxt(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_CONF_NXT_MASK);
    }

    /// Clear SPI_USR_CONF_NXT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_conf_nxt(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_CONF_NXT_MASK);
    }

    /// **SPI_FWRITE_OCT** `[14]` (RW)
    ///
    /// In the write operations read-data phase apply 8 signals. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fwrite_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FWRITE_OCT_MASK) != 0
    }

    /// Set SPI_FWRITE_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fwrite_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FWRITE_OCT_MASK);
    }

    /// Clear SPI_FWRITE_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fwrite_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FWRITE_OCT_MASK);
    }

    /// **SPI_FWRITE_QUAD** `[13]` (RW)
    ///
    /// In the write operations read-data phase apply 4 signals. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FWRITE_QUAD_MASK) != 0
    }

    /// Set SPI_FWRITE_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fwrite_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FWRITE_QUAD_MASK);
    }

    /// Clear SPI_FWRITE_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fwrite_quad(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FWRITE_QUAD_MASK);
    }

    /// **SPI_FWRITE_DUAL** `[12]` (RW)
    ///
    /// In the write operations read-data phase apply 2 signals. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FWRITE_DUAL_MASK) != 0
    }

    /// Set SPI_FWRITE_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fwrite_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FWRITE_DUAL_MASK);
    }

    /// Clear SPI_FWRITE_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fwrite_dual(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FWRITE_DUAL_MASK);
    }

    /// **SPI_CK_OUT_EDGE** `[9]` (RW)
    ///
    /// the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_ck_out_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CK_OUT_EDGE_MASK) != 0
    }

    /// Set SPI_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_ck_out_edge(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CK_OUT_EDGE_MASK);
    }

    /// Clear SPI_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_ck_out_edge(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CK_OUT_EDGE_MASK);
    }

    /// **SPI_RSCK_I_EDGE** `[8]` (RW)
    ///
    /// In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i.
    #[inline(always)]
    pub fn spi_rsck_i_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_RSCK_I_EDGE_MASK) != 0
    }

    /// Set SPI_RSCK_I_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_rsck_i_edge(&self) {
        self.value.modify_bits(|r| r | Self::SPI_RSCK_I_EDGE_MASK);
    }

    /// Clear SPI_RSCK_I_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_rsck_i_edge(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_RSCK_I_EDGE_MASK);
    }

    /// **SPI_CS_SETUP** `[7]` (RW)
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS_SETUP_MASK) != 0
    }

    /// Set SPI_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs_setup(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS_SETUP_MASK);
    }

    /// Clear SPI_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs_setup(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS_SETUP_MASK);
    }

    /// **SPI_CS_HOLD** `[6]` (RW)
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS_HOLD_MASK) != 0
    }

    /// Set SPI_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs_hold(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS_HOLD_MASK);
    }

    /// Clear SPI_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs_hold(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS_HOLD_MASK);
    }

    /// **SPI_TSCK_I_EDGE** `[5]` (RW)
    ///
    /// In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i.
    #[inline(always)]
    pub fn spi_tsck_i_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_TSCK_I_EDGE_MASK) != 0
    }

    /// Set SPI_TSCK_I_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_tsck_i_edge(&self) {
        self.value.modify_bits(|r| r | Self::SPI_TSCK_I_EDGE_MASK);
    }

    /// Clear SPI_TSCK_I_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_tsck_i_edge(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_TSCK_I_EDGE_MASK);
    }

    /// **SPI_OPI_MODE** `[4]` (RW)
    ///
    /// Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_opi_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_OPI_MODE_MASK) != 0
    }

    /// Set SPI_OPI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_opi_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_OPI_MODE_MASK);
    }

    /// Clear SPI_OPI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_opi_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_OPI_MODE_MASK);
    }

    /// **SPI_QPI_MODE** `[3]` (RW)
    ///
    /// Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_qpi_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_QPI_MODE_MASK) != 0
    }

    /// Set SPI_QPI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_qpi_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_QPI_MODE_MASK);
    }

    /// Clear SPI_QPI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_qpi_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_QPI_MODE_MASK);
    }

    /// **SPI_DOUTDIN** `[0]` (RW)
    ///
    /// Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_doutdin(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUTDIN_MASK) != 0
    }

    /// Set SPI_DOUTDIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_doutdin(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUTDIN_MASK);
    }

    /// Clear SPI_DOUTDIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_doutdin(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUTDIN_MASK);
    }
}

/// SPI_USER value type for batch read/modify/write operations
///
/// Use `spiuser.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiuser.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiUserVal {
    bits: u32,
}

impl SpiUserVal {
    /// Create from raw bits
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
        Self { bits: 0x800000c0 }
    }

    /// Read **SPI_USR_COMMAND** `[31:31]`
    ///
    /// This bit enable the command phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_command(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_COMMAND_MASK) != 0
    }

    /// Read **SPI_USR_ADDR** `[30:30]`
    ///
    /// This bit enable the address phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_addr(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_ADDR_MASK) != 0
    }

    /// Read **SPI_USR_DUMMY** `[29:29]`
    ///
    /// This bit enable the dummy phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_dummy(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_DUMMY_MASK) != 0
    }

    /// Read **SPI_USR_MISO** `[28:28]`
    ///
    /// This bit enable the read-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_miso(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_MISO_MASK) != 0
    }

    /// Read **SPI_USR_MOSI** `[27:27]`
    ///
    /// This bit enable the write-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_mosi(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_MOSI_MASK) != 0
    }

    /// Read **SPI_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_dummy_idle(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_DUMMY_IDLE_MASK) != 0
    }

    /// Read **SPI_USR_MOSI_HIGHPART** `[25:25]`
    ///
    /// write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_mosi_highpart(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_MOSI_HIGHPART_MASK) != 0
    }

    /// Read **SPI_USR_MISO_HIGHPART** `[24:24]`
    ///
    /// read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_miso_highpart(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_MISO_HIGHPART_MASK) != 0
    }

    /// Read **SPI_SIO** `[17:17]`
    ///
    /// Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_sio(&self) -> bool {
        (self.bits & SpiUser::SPI_SIO_MASK) != 0
    }

    /// Read **SPI_USR_CONF_NXT** `[15:15]`
    ///
    /// 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_conf_nxt(&self) -> bool {
        (self.bits & SpiUser::SPI_USR_CONF_NXT_MASK) != 0
    }

    /// Read **SPI_FWRITE_OCT** `[14:14]`
    ///
    /// In the write operations read-data phase apply 8 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fwrite_oct(&self) -> bool {
        (self.bits & SpiUser::SPI_FWRITE_OCT_MASK) != 0
    }

    /// Read **SPI_FWRITE_QUAD** `[13:13]`
    ///
    /// In the write operations read-data phase apply 4 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fwrite_quad(&self) -> bool {
        (self.bits & SpiUser::SPI_FWRITE_QUAD_MASK) != 0
    }

    /// Read **SPI_FWRITE_DUAL** `[12:12]`
    ///
    /// In the write operations read-data phase apply 2 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_fwrite_dual(&self) -> bool {
        (self.bits & SpiUser::SPI_FWRITE_DUAL_MASK) != 0
    }

    /// Read **SPI_CK_OUT_EDGE** `[9:9]`
    ///
    /// the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_ck_out_edge(&self) -> bool {
        (self.bits & SpiUser::SPI_CK_OUT_EDGE_MASK) != 0
    }

    /// Read **SPI_RSCK_I_EDGE** `[8:8]`
    ///
    /// In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i.
    #[inline(always)]
    pub const fn spi_rsck_i_edge(&self) -> bool {
        (self.bits & SpiUser::SPI_RSCK_I_EDGE_MASK) != 0
    }

    /// Read **SPI_CS_SETUP** `[7:7]`
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs_setup(&self) -> bool {
        (self.bits & SpiUser::SPI_CS_SETUP_MASK) != 0
    }

    /// Read **SPI_CS_HOLD** `[6:6]`
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs_hold(&self) -> bool {
        (self.bits & SpiUser::SPI_CS_HOLD_MASK) != 0
    }

    /// Read **SPI_TSCK_I_EDGE** `[5:5]`
    ///
    /// In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i.
    #[inline(always)]
    pub const fn spi_tsck_i_edge(&self) -> bool {
        (self.bits & SpiUser::SPI_TSCK_I_EDGE_MASK) != 0
    }

    /// Read **SPI_OPI_MODE** `[4:4]`
    ///
    /// Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_opi_mode(&self) -> bool {
        (self.bits & SpiUser::SPI_OPI_MODE_MASK) != 0
    }

    /// Read **SPI_QPI_MODE** `[3:3]`
    ///
    /// Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_qpi_mode(&self) -> bool {
        (self.bits & SpiUser::SPI_QPI_MODE_MASK) != 0
    }

    /// Read **SPI_DOUTDIN** `[0:0]`
    ///
    /// Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_doutdin(&self) -> bool {
        (self.bits & SpiUser::SPI_DOUTDIN_MASK) != 0
    }

    /// Set **SPI_USR_COMMAND** `[31:31]`
    ///
    /// This bit enable the command phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_command(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_COMMAND_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_COMMAND_MASK
            },
        }
    }

    /// Set **SPI_USR_ADDR** `[30:30]`
    ///
    /// This bit enable the address phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_addr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_ADDR_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_ADDR_MASK
            },
        }
    }

    /// Set **SPI_USR_DUMMY** `[29:29]`
    ///
    /// This bit enable the dummy phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_DUMMY_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_USR_MISO** `[28:28]`
    ///
    /// This bit enable the read-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_miso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_MISO_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_MISO_MASK
            },
        }
    }

    /// Set **SPI_USR_MOSI** `[27:27]`
    ///
    /// This bit enable the write-data phase of an operation. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_mosi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_MOSI_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_MOSI_MASK
            },
        }
    }

    /// Set **SPI_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_dummy_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_DUMMY_IDLE_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_DUMMY_IDLE_MASK
            },
        }
    }

    /// Set **SPI_USR_MOSI_HIGHPART** `[25:25]`
    ///
    /// write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_mosi_highpart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_MOSI_HIGHPART_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_MOSI_HIGHPART_MASK
            },
        }
    }

    /// Set **SPI_USR_MISO_HIGHPART** `[24:24]`
    ///
    /// read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_miso_highpart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_MISO_HIGHPART_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_MISO_HIGHPART_MASK
            },
        }
    }

    /// Set **SPI_SIO** `[17:17]`
    ///
    /// Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_SIO_MASK
            } else {
                self.bits & !SpiUser::SPI_SIO_MASK
            },
        }
    }

    /// Set **SPI_USR_CONF_NXT** `[15:15]`
    ///
    /// 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_conf_nxt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_USR_CONF_NXT_MASK
            } else {
                self.bits & !SpiUser::SPI_USR_CONF_NXT_MASK
            },
        }
    }

    /// Set **SPI_FWRITE_OCT** `[14:14]`
    ///
    /// In the write operations read-data phase apply 8 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fwrite_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_FWRITE_OCT_MASK
            } else {
                self.bits & !SpiUser::SPI_FWRITE_OCT_MASK
            },
        }
    }

    /// Set **SPI_FWRITE_QUAD** `[13:13]`
    ///
    /// In the write operations read-data phase apply 4 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fwrite_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_FWRITE_QUAD_MASK
            } else {
                self.bits & !SpiUser::SPI_FWRITE_QUAD_MASK
            },
        }
    }

    /// Set **SPI_FWRITE_DUAL** `[12:12]`
    ///
    /// In the write operations read-data phase apply 2 signals. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_fwrite_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_FWRITE_DUAL_MASK
            } else {
                self.bits & !SpiUser::SPI_FWRITE_DUAL_MASK
            },
        }
    }

    /// Set **SPI_CK_OUT_EDGE** `[9:9]`
    ///
    /// the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_ck_out_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_CK_OUT_EDGE_MASK
            } else {
                self.bits & !SpiUser::SPI_CK_OUT_EDGE_MASK
            },
        }
    }

    /// Set **SPI_RSCK_I_EDGE** `[8:8]`
    ///
    /// In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i.
    #[inline(always)]
    pub const fn with_spi_rsck_i_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_RSCK_I_EDGE_MASK
            } else {
                self.bits & !SpiUser::SPI_RSCK_I_EDGE_MASK
            },
        }
    }

    /// Set **SPI_CS_SETUP** `[7:7]`
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_CS_SETUP_MASK
            } else {
                self.bits & !SpiUser::SPI_CS_SETUP_MASK
            },
        }
    }

    /// Set **SPI_CS_HOLD** `[6:6]`
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs_hold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_CS_HOLD_MASK
            } else {
                self.bits & !SpiUser::SPI_CS_HOLD_MASK
            },
        }
    }

    /// Set **SPI_TSCK_I_EDGE** `[5:5]`
    ///
    /// In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i.
    #[inline(always)]
    pub const fn with_spi_tsck_i_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_TSCK_I_EDGE_MASK
            } else {
                self.bits & !SpiUser::SPI_TSCK_I_EDGE_MASK
            },
        }
    }

    /// Set **SPI_OPI_MODE** `[4:4]`
    ///
    /// Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_opi_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_OPI_MODE_MASK
            } else {
                self.bits & !SpiUser::SPI_OPI_MODE_MASK
            },
        }
    }

    /// Set **SPI_QPI_MODE** `[3:3]`
    ///
    /// Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_qpi_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_QPI_MODE_MASK
            } else {
                self.bits & !SpiUser::SPI_QPI_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUTDIN** `[0:0]`
    ///
    /// Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_doutdin(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser::SPI_DOUTDIN_MASK
            } else {
                self.bits & !SpiUser::SPI_DOUTDIN_MASK
            },
        }
    }
}

impl Default for SpiUserVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiUser {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiUserVal {
        SpiUserVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiUserVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiUserVal) -> SpiUserVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_USER1 Register
///
/// SPI USER control register 1
///
/// Address offset: 0x0014
/// Reset value: 0xb8410007
///
/// ## Fields
///
/// - **SPI_USR_ADDR_BITLEN** `[31:27]` (RW)
///
///   The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
///
/// - **SPI_CS_HOLD_TIME** `[26:22]` (RW)
///
///   delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
///
/// - **SPI_CS_SETUP_TIME** `[21:17]` (RW)
///
///   (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
///
/// - **SPI_MST_WFULL_ERR_END_EN** `[16]` (RW)
///
///   1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode.
///
/// - **SPI_USR_DUMMY_CYCLELEN** `[7:0]` (RW)
///
///   The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiUser1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiUser1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiUser1 {
    // ======== Field Constants ========

    /// **SPI_USR_ADDR_BITLEN**
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const SPI_USR_ADDR_BITLEN_OFFSET: u32 = 27;
    pub const SPI_USR_ADDR_BITLEN_MASK: u32 = 0xf8000000;

    /// **SPI_CS_HOLD_TIME**
    ///
    /// delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
    ///
    /// Bits `[26:22]`, mask `0x07c00000`
    pub const SPI_CS_HOLD_TIME_OFFSET: u32 = 22;
    pub const SPI_CS_HOLD_TIME_MASK: u32 = 0x07c00000;

    /// **SPI_CS_SETUP_TIME**
    ///
    /// (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
    ///
    /// Bits `[21:17]`, mask `0x003e0000`
    pub const SPI_CS_SETUP_TIME_OFFSET: u32 = 17;
    pub const SPI_CS_SETUP_TIME_MASK: u32 = 0x003e0000;

    /// **SPI_MST_WFULL_ERR_END_EN**
    ///
    /// 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_MST_WFULL_ERR_END_EN_OFFSET: u32 = 16;
    pub const SPI_MST_WFULL_ERR_END_EN_MASK: u32 = 0x00010000;
    pub const SPI_MST_WFULL_ERR_END_EN: u32 = 1 << 16;

    /// **SPI_USR_DUMMY_CYCLELEN**
    ///
    /// The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SPI_USR_DUMMY_CYCLELEN_OFFSET: u32 = 0;
    pub const SPI_USR_DUMMY_CYCLELEN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SPI_USR_ADDR_BITLEN** `[31:27]` (RW)
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_addr_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_USR_ADDR_BITLEN_MASK)
            >> Self::SPI_USR_ADDR_BITLEN_OFFSET
    }

    /// Write **SPI_USR_ADDR_BITLEN** `[31:27]` (RW)
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_usr_addr_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_USR_ADDR_BITLEN_MASK)
                | ((value << Self::SPI_USR_ADDR_BITLEN_OFFSET) & Self::SPI_USR_ADDR_BITLEN_MASK)
        });
    }

    /// **SPI_CS_HOLD_TIME** `[26:22]` (RW)
    ///
    /// delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CS_HOLD_TIME_MASK) >> Self::SPI_CS_HOLD_TIME_OFFSET
    }

    /// Write **SPI_CS_HOLD_TIME** `[26:22]` (RW)
    ///
    /// delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_cs_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CS_HOLD_TIME_MASK)
                | ((value << Self::SPI_CS_HOLD_TIME_OFFSET) & Self::SPI_CS_HOLD_TIME_MASK)
        });
    }

    /// **SPI_CS_SETUP_TIME** `[21:17]` (RW)
    ///
    /// (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs_setup_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CS_SETUP_TIME_MASK) >> Self::SPI_CS_SETUP_TIME_OFFSET
    }

    /// Write **SPI_CS_SETUP_TIME** `[21:17]` (RW)
    ///
    /// (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_cs_setup_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CS_SETUP_TIME_MASK)
                | ((value << Self::SPI_CS_SETUP_TIME_OFFSET) & Self::SPI_CS_SETUP_TIME_MASK)
        });
    }

    /// **SPI_MST_WFULL_ERR_END_EN** `[16]` (RW)
    ///
    /// 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub fn spi_mst_wfull_err_end_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_WFULL_ERR_END_EN_MASK) != 0
    }

    /// Set SPI_MST_WFULL_ERR_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_wfull_err_end_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_WFULL_ERR_END_EN_MASK);
    }

    /// Clear SPI_MST_WFULL_ERR_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_wfull_err_end_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_WFULL_ERR_END_EN_MASK);
    }

    /// **SPI_USR_DUMMY_CYCLELEN** `[7:0]` (RW)
    ///
    /// The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_USR_DUMMY_CYCLELEN_MASK)
            >> Self::SPI_USR_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_USR_DUMMY_CYCLELEN** `[7:0]` (RW)
    ///
    /// The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_usr_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_USR_DUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_USR_DUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_USR_DUMMY_CYCLELEN_MASK)
        });
    }
}

/// SPI_USER1 value type for batch read/modify/write operations
///
/// Use `spiuser1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiuser1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiUser1Val {
    bits: u32,
}

impl SpiUser1Val {
    /// Create from raw bits
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
        Self { bits: 0xb8410007 }
    }

    /// Read **SPI_USR_ADDR_BITLEN** `[31:27]`
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_addr_bitlen(&self) -> u32 {
        (self.bits & SpiUser1::SPI_USR_ADDR_BITLEN_MASK) >> SpiUser1::SPI_USR_ADDR_BITLEN_OFFSET
    }

    /// Read **SPI_CS_HOLD_TIME** `[26:22]`
    ///
    /// delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs_hold_time(&self) -> u32 {
        (self.bits & SpiUser1::SPI_CS_HOLD_TIME_MASK) >> SpiUser1::SPI_CS_HOLD_TIME_OFFSET
    }

    /// Read **SPI_CS_SETUP_TIME** `[21:17]`
    ///
    /// (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs_setup_time(&self) -> u32 {
        (self.bits & SpiUser1::SPI_CS_SETUP_TIME_MASK) >> SpiUser1::SPI_CS_SETUP_TIME_OFFSET
    }

    /// Read **SPI_MST_WFULL_ERR_END_EN** `[16:16]`
    ///
    /// 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub const fn spi_mst_wfull_err_end_en(&self) -> bool {
        (self.bits & SpiUser1::SPI_MST_WFULL_ERR_END_EN_MASK) != 0
    }

    /// Read **SPI_USR_DUMMY_CYCLELEN** `[7:0]`
    ///
    /// The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_dummy_cyclelen(&self) -> u32 {
        (self.bits & SpiUser1::SPI_USR_DUMMY_CYCLELEN_MASK)
            >> SpiUser1::SPI_USR_DUMMY_CYCLELEN_OFFSET
    }

    /// Set **SPI_USR_ADDR_BITLEN** `[31:27]`
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_addr_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser1::SPI_USR_ADDR_BITLEN_MASK)
                | ((value << SpiUser1::SPI_USR_ADDR_BITLEN_OFFSET)
                    & SpiUser1::SPI_USR_ADDR_BITLEN_MASK),
        }
    }

    /// Set **SPI_CS_HOLD_TIME** `[26:22]`
    ///
    /// delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser1::SPI_CS_HOLD_TIME_MASK)
                | ((value << SpiUser1::SPI_CS_HOLD_TIME_OFFSET) & SpiUser1::SPI_CS_HOLD_TIME_MASK),
        }
    }

    /// Set **SPI_CS_SETUP_TIME** `[21:17]`
    ///
    /// (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs_setup_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser1::SPI_CS_SETUP_TIME_MASK)
                | ((value << SpiUser1::SPI_CS_SETUP_TIME_OFFSET)
                    & SpiUser1::SPI_CS_SETUP_TIME_MASK),
        }
    }

    /// Set **SPI_MST_WFULL_ERR_END_EN** `[16:16]`
    ///
    /// 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub const fn with_spi_mst_wfull_err_end_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser1::SPI_MST_WFULL_ERR_END_EN_MASK
            } else {
                self.bits & !SpiUser1::SPI_MST_WFULL_ERR_END_EN_MASK
            },
        }
    }

    /// Set **SPI_USR_DUMMY_CYCLELEN** `[7:0]`
    ///
    /// The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser1::SPI_USR_DUMMY_CYCLELEN_MASK)
                | ((value << SpiUser1::SPI_USR_DUMMY_CYCLELEN_OFFSET)
                    & SpiUser1::SPI_USR_DUMMY_CYCLELEN_MASK),
        }
    }
}

impl Default for SpiUser1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiUser1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiUser1Val {
        SpiUser1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiUser1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiUser1Val) -> SpiUser1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_USER2 Register
///
/// SPI USER control register 2
///
/// Address offset: 0x0018
/// Reset value: 0x78000000
///
/// ## Fields
///
/// - **SPI_USR_COMMAND_BITLEN** `[31:28]` (RW)
///
///   The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
///
/// - **SPI_MST_REMPTY_ERR_END_EN** `[27]` (RW)
///
///   1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode.
///
/// - **SPI_USR_COMMAND_VALUE** `[15:0]` (RW)
///
///   The value of command. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiUser2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiUser2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiUser2 {
    // ======== Field Constants ========

    /// **SPI_USR_COMMAND_BITLEN**
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const SPI_USR_COMMAND_BITLEN_OFFSET: u32 = 28;
    pub const SPI_USR_COMMAND_BITLEN_MASK: u32 = 0xf0000000;

    /// **SPI_MST_REMPTY_ERR_END_EN**
    ///
    /// 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MST_REMPTY_ERR_END_EN_OFFSET: u32 = 27;
    pub const SPI_MST_REMPTY_ERR_END_EN_MASK: u32 = 0x08000000;
    pub const SPI_MST_REMPTY_ERR_END_EN: u32 = 1 << 27;

    /// **SPI_USR_COMMAND_VALUE**
    ///
    /// The value of command. Can be configured in CONF state.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_USR_COMMAND_VALUE_OFFSET: u32 = 0;
    pub const SPI_USR_COMMAND_VALUE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SPI_USR_COMMAND_BITLEN** `[31:28]` (RW)
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_command_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_USR_COMMAND_BITLEN_MASK)
            >> Self::SPI_USR_COMMAND_BITLEN_OFFSET
    }

    /// Write **SPI_USR_COMMAND_BITLEN** `[31:28]` (RW)
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_usr_command_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_USR_COMMAND_BITLEN_MASK)
                | ((value << Self::SPI_USR_COMMAND_BITLEN_OFFSET)
                    & Self::SPI_USR_COMMAND_BITLEN_MASK)
        });
    }

    /// **SPI_MST_REMPTY_ERR_END_EN** `[27]` (RW)
    ///
    /// 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub fn spi_mst_rempty_err_end_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_REMPTY_ERR_END_EN_MASK) != 0
    }

    /// Set SPI_MST_REMPTY_ERR_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_rempty_err_end_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_REMPTY_ERR_END_EN_MASK);
    }

    /// Clear SPI_MST_REMPTY_ERR_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_rempty_err_end_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_REMPTY_ERR_END_EN_MASK);
    }

    /// **SPI_USR_COMMAND_VALUE** `[15:0]` (RW)
    ///
    /// The value of command. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_usr_command_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_USR_COMMAND_VALUE_MASK)
            >> Self::SPI_USR_COMMAND_VALUE_OFFSET
    }

    /// Write **SPI_USR_COMMAND_VALUE** `[15:0]` (RW)
    ///
    /// The value of command. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_usr_command_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_USR_COMMAND_VALUE_MASK)
                | ((value << Self::SPI_USR_COMMAND_VALUE_OFFSET) & Self::SPI_USR_COMMAND_VALUE_MASK)
        });
    }
}

/// SPI_USER2 value type for batch read/modify/write operations
///
/// Use `spiuser2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiuser2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiUser2Val {
    bits: u32,
}

impl SpiUser2Val {
    /// Create from raw bits
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
        Self { bits: 0x78000000 }
    }

    /// Read **SPI_USR_COMMAND_BITLEN** `[31:28]`
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_command_bitlen(&self) -> u32 {
        (self.bits & SpiUser2::SPI_USR_COMMAND_BITLEN_MASK)
            >> SpiUser2::SPI_USR_COMMAND_BITLEN_OFFSET
    }

    /// Read **SPI_MST_REMPTY_ERR_END_EN** `[27:27]`
    ///
    /// 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub const fn spi_mst_rempty_err_end_en(&self) -> bool {
        (self.bits & SpiUser2::SPI_MST_REMPTY_ERR_END_EN_MASK) != 0
    }

    /// Read **SPI_USR_COMMAND_VALUE** `[15:0]`
    ///
    /// The value of command. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_usr_command_value(&self) -> u32 {
        (self.bits & SpiUser2::SPI_USR_COMMAND_VALUE_MASK) >> SpiUser2::SPI_USR_COMMAND_VALUE_OFFSET
    }

    /// Set **SPI_USR_COMMAND_BITLEN** `[31:28]`
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_command_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser2::SPI_USR_COMMAND_BITLEN_MASK)
                | ((value << SpiUser2::SPI_USR_COMMAND_BITLEN_OFFSET)
                    & SpiUser2::SPI_USR_COMMAND_BITLEN_MASK),
        }
    }

    /// Set **SPI_MST_REMPTY_ERR_END_EN** `[27:27]`
    ///
    /// 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode.
    #[inline(always)]
    pub const fn with_spi_mst_rempty_err_end_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiUser2::SPI_MST_REMPTY_ERR_END_EN_MASK
            } else {
                self.bits & !SpiUser2::SPI_MST_REMPTY_ERR_END_EN_MASK
            },
        }
    }

    /// Set **SPI_USR_COMMAND_VALUE** `[15:0]`
    ///
    /// The value of command. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_usr_command_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiUser2::SPI_USR_COMMAND_VALUE_MASK)
                | ((value << SpiUser2::SPI_USR_COMMAND_VALUE_OFFSET)
                    & SpiUser2::SPI_USR_COMMAND_VALUE_MASK),
        }
    }
}

impl Default for SpiUser2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiUser2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiUser2Val {
        SpiUser2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiUser2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiUser2Val) -> SpiUser2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MS_DLEN Register
///
/// SPI data bit length control register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **SPI_MS_DATA_BITLEN** `[17:0]` (RW)
///
///   The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiMsDlen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMsDlen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMsDlen {
    // ======== Field Constants ========

    /// **SPI_MS_DATA_BITLEN**
    ///
    /// The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const SPI_MS_DATA_BITLEN_OFFSET: u32 = 0;
    pub const SPI_MS_DATA_BITLEN_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **SPI_MS_DATA_BITLEN** `[17:0]` (RW)
    ///
    /// The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MS_DATA_BITLEN_MASK) >> Self::SPI_MS_DATA_BITLEN_OFFSET
    }

    /// Write **SPI_MS_DATA_BITLEN** `[17:0]` (RW)
    ///
    /// The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_ms_data_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MS_DATA_BITLEN_MASK)
                | ((value << Self::SPI_MS_DATA_BITLEN_OFFSET) & Self::SPI_MS_DATA_BITLEN_MASK)
        });
    }
}

/// SPI_MS_DLEN value type for batch read/modify/write operations
///
/// Use `spimsdlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimsdlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMsDlenVal {
    bits: u32,
}

impl SpiMsDlenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MS_DATA_BITLEN** `[17:0]`
    ///
    /// The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_ms_data_bitlen(&self) -> u32 {
        (self.bits & SpiMsDlen::SPI_MS_DATA_BITLEN_MASK) >> SpiMsDlen::SPI_MS_DATA_BITLEN_OFFSET
    }

    /// Set **SPI_MS_DATA_BITLEN** `[17:0]`
    ///
    /// The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_ms_data_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMsDlen::SPI_MS_DATA_BITLEN_MASK)
                | ((value << SpiMsDlen::SPI_MS_DATA_BITLEN_OFFSET)
                    & SpiMsDlen::SPI_MS_DATA_BITLEN_MASK),
        }
    }
}

impl Default for SpiMsDlenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMsDlen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMsDlenVal {
        SpiMsDlenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMsDlenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMsDlenVal) -> SpiMsDlenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MISC Register
///
/// SPI misc register
///
/// Address offset: 0x0020
/// Reset value: 0x0000003e
///
/// ## Fields
///
/// - **SPI_QUAD_DIN_PIN_SWAP** `[31]` (RW)
///
///   1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state.
///
/// - **SPI_CS_KEEP_ACTIVE** `[30]` (RW)
///
///   spi cs line keep low when the bit is set. Can be configured in CONF state.
///
/// - **SPI_CK_IDLE_EDGE** `[29]` (RW)
///
///   1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state.
///
/// - **SPI_DQS_IDLE_EDGE** `[24]` (RW)
///
///   The default value of spi_dqs. Can be configured in CONF state.
///
/// - **SPI_SLAVE_CS_POL** `[23]` (RW)
///
///   spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state.
///
/// - **SPI_CMD_DTR_EN** `[19]` (RW)
///
///   1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state.
///
/// - **SPI_ADDR_DTR_EN** `[18]` (RW)
///
///   1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state.
///
/// - **SPI_DATA_DTR_EN** `[17]` (RW)
///
///   1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state.
///
/// - **SPI_CLK_DATA_DTR_EN** `[16]` (RW)
///
///   1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19.
///
/// - **SPI_MASTER_CS_POL** `[12:7]` (RW)
///
///   In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
///
/// - **SPI_CK_DIS** `[6]` (RW)
///
///   1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state.
///
/// - **SPI_CS5_DIS** `[5]` (RW)
///
///   SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state.
///
/// - **SPI_CS4_DIS** `[4]` (RW)
///
///   SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state.
///
/// - **SPI_CS3_DIS** `[3]` (RW)
///
///   SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state.
///
/// - **SPI_CS2_DIS** `[2]` (RW)
///
///   SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state.
///
/// - **SPI_CS1_DIS** `[1]` (RW)
///
///   SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state.
///
/// - **SPI_CS0_DIS** `[0]` (RW)
///
///   SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiMisc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMisc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMisc {
    // ======== Field Constants ========

    /// **SPI_QUAD_DIN_PIN_SWAP**
    ///
    /// 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_QUAD_DIN_PIN_SWAP_OFFSET: u32 = 31;
    pub const SPI_QUAD_DIN_PIN_SWAP_MASK: u32 = 0x80000000;
    pub const SPI_QUAD_DIN_PIN_SWAP: u32 = 1 << 31;

    /// **SPI_CS_KEEP_ACTIVE**
    ///
    /// spi cs line keep low when the bit is set. Can be configured in CONF state.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_CS_KEEP_ACTIVE_OFFSET: u32 = 30;
    pub const SPI_CS_KEEP_ACTIVE_MASK: u32 = 0x40000000;
    pub const SPI_CS_KEEP_ACTIVE: u32 = 1 << 30;

    /// **SPI_CK_IDLE_EDGE**
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_CK_IDLE_EDGE_OFFSET: u32 = 29;
    pub const SPI_CK_IDLE_EDGE_MASK: u32 = 0x20000000;
    pub const SPI_CK_IDLE_EDGE: u32 = 1 << 29;

    /// **SPI_DQS_IDLE_EDGE**
    ///
    /// The default value of spi_dqs. Can be configured in CONF state.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_DQS_IDLE_EDGE_OFFSET: u32 = 24;
    pub const SPI_DQS_IDLE_EDGE_MASK: u32 = 0x01000000;
    pub const SPI_DQS_IDLE_EDGE: u32 = 1 << 24;

    /// **SPI_SLAVE_CS_POL**
    ///
    /// spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_SLAVE_CS_POL_OFFSET: u32 = 23;
    pub const SPI_SLAVE_CS_POL_MASK: u32 = 0x00800000;
    pub const SPI_SLAVE_CS_POL: u32 = 1 << 23;

    /// **SPI_CMD_DTR_EN**
    ///
    /// 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_CMD_DTR_EN_OFFSET: u32 = 19;
    pub const SPI_CMD_DTR_EN_MASK: u32 = 0x00080000;
    pub const SPI_CMD_DTR_EN: u32 = 1 << 19;

    /// **SPI_ADDR_DTR_EN**
    ///
    /// 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_ADDR_DTR_EN_OFFSET: u32 = 18;
    pub const SPI_ADDR_DTR_EN_MASK: u32 = 0x00040000;
    pub const SPI_ADDR_DTR_EN: u32 = 1 << 18;

    /// **SPI_DATA_DTR_EN**
    ///
    /// 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_DATA_DTR_EN_OFFSET: u32 = 17;
    pub const SPI_DATA_DTR_EN_MASK: u32 = 0x00020000;
    pub const SPI_DATA_DTR_EN: u32 = 1 << 17;

    /// **SPI_CLK_DATA_DTR_EN**
    ///
    /// 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_CLK_DATA_DTR_EN_OFFSET: u32 = 16;
    pub const SPI_CLK_DATA_DTR_EN_MASK: u32 = 0x00010000;
    pub const SPI_CLK_DATA_DTR_EN: u32 = 1 << 16;

    /// **SPI_MASTER_CS_POL**
    ///
    /// In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const SPI_MASTER_CS_POL_OFFSET: u32 = 7;
    pub const SPI_MASTER_CS_POL_MASK: u32 = 0x00001f80;

    /// **SPI_CK_DIS**
    ///
    /// 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_CK_DIS_OFFSET: u32 = 6;
    pub const SPI_CK_DIS_MASK: u32 = 0x00000040;
    pub const SPI_CK_DIS: u32 = 1 << 6;

    /// **SPI_CS5_DIS**
    ///
    /// SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_CS5_DIS_OFFSET: u32 = 5;
    pub const SPI_CS5_DIS_MASK: u32 = 0x00000020;
    pub const SPI_CS5_DIS: u32 = 1 << 5;

    /// **SPI_CS4_DIS**
    ///
    /// SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_CS4_DIS_OFFSET: u32 = 4;
    pub const SPI_CS4_DIS_MASK: u32 = 0x00000010;
    pub const SPI_CS4_DIS: u32 = 1 << 4;

    /// **SPI_CS3_DIS**
    ///
    /// SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_CS3_DIS_OFFSET: u32 = 3;
    pub const SPI_CS3_DIS_MASK: u32 = 0x00000008;
    pub const SPI_CS3_DIS: u32 = 1 << 3;

    /// **SPI_CS2_DIS**
    ///
    /// SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_CS2_DIS_OFFSET: u32 = 2;
    pub const SPI_CS2_DIS_MASK: u32 = 0x00000004;
    pub const SPI_CS2_DIS: u32 = 1 << 2;

    /// **SPI_CS1_DIS**
    ///
    /// SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_CS1_DIS_OFFSET: u32 = 1;
    pub const SPI_CS1_DIS_MASK: u32 = 0x00000002;
    pub const SPI_CS1_DIS: u32 = 1 << 1;

    /// **SPI_CS0_DIS**
    ///
    /// SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_CS0_DIS_OFFSET: u32 = 0;
    pub const SPI_CS0_DIS_MASK: u32 = 0x00000001;
    pub const SPI_CS0_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_QUAD_DIN_PIN_SWAP** `[31]` (RW)
    ///
    /// 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&self) -> bool {
        (self.value.read_bits() & Self::SPI_QUAD_DIN_PIN_SWAP_MASK) != 0
    }

    /// Set SPI_QUAD_DIN_PIN_SWAP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_quad_din_pin_swap(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_QUAD_DIN_PIN_SWAP_MASK);
    }

    /// Clear SPI_QUAD_DIN_PIN_SWAP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_quad_din_pin_swap(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_QUAD_DIN_PIN_SWAP_MASK);
    }

    /// **SPI_CS_KEEP_ACTIVE** `[30]` (RW)
    ///
    /// spi cs line keep low when the bit is set. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs_keep_active(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS_KEEP_ACTIVE_MASK) != 0
    }

    /// Set SPI_CS_KEEP_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs_keep_active(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CS_KEEP_ACTIVE_MASK);
    }

    /// Clear SPI_CS_KEEP_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs_keep_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CS_KEEP_ACTIVE_MASK);
    }

    /// **SPI_CK_IDLE_EDGE** `[29]` (RW)
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CK_IDLE_EDGE_MASK) != 0
    }

    /// Set SPI_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_ck_idle_edge(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CK_IDLE_EDGE_MASK);
    }

    /// Clear SPI_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_ck_idle_edge(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CK_IDLE_EDGE_MASK);
    }

    /// **SPI_DQS_IDLE_EDGE** `[24]` (RW)
    ///
    /// The default value of spi_dqs. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dqs_idle_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DQS_IDLE_EDGE_MASK) != 0
    }

    /// Set SPI_DQS_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dqs_idle_edge(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DQS_IDLE_EDGE_MASK);
    }

    /// Clear SPI_DQS_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dqs_idle_edge(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DQS_IDLE_EDGE_MASK);
    }

    /// **SPI_SLAVE_CS_POL** `[23]` (RW)
    ///
    /// spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_slave_cs_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLAVE_CS_POL_MASK) != 0
    }

    /// Set SPI_SLAVE_CS_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slave_cs_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SLAVE_CS_POL_MASK);
    }

    /// Clear SPI_SLAVE_CS_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slave_cs_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SLAVE_CS_POL_MASK);
    }

    /// **SPI_CMD_DTR_EN** `[19]` (RW)
    ///
    /// 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cmd_dtr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CMD_DTR_EN_MASK) != 0
    }

    /// Set SPI_CMD_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cmd_dtr_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CMD_DTR_EN_MASK);
    }

    /// Clear SPI_CMD_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cmd_dtr_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CMD_DTR_EN_MASK);
    }

    /// **SPI_ADDR_DTR_EN** `[18]` (RW)
    ///
    /// 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_addr_dtr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_ADDR_DTR_EN_MASK) != 0
    }

    /// Set SPI_ADDR_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_addr_dtr_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_ADDR_DTR_EN_MASK);
    }

    /// Clear SPI_ADDR_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_addr_dtr_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_ADDR_DTR_EN_MASK);
    }

    /// **SPI_DATA_DTR_EN** `[17]` (RW)
    ///
    /// 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_data_dtr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DATA_DTR_EN_MASK) != 0
    }

    /// Set SPI_DATA_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_data_dtr_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DATA_DTR_EN_MASK);
    }

    /// Clear SPI_DATA_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_data_dtr_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DATA_DTR_EN_MASK);
    }

    /// **SPI_CLK_DATA_DTR_EN** `[16]` (RW)
    ///
    /// 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19.
    #[inline(always)]
    pub fn spi_clk_data_dtr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CLK_DATA_DTR_EN_MASK) != 0
    }

    /// Set SPI_CLK_DATA_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_clk_data_dtr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CLK_DATA_DTR_EN_MASK);
    }

    /// Clear SPI_CLK_DATA_DTR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_clk_data_dtr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CLK_DATA_DTR_EN_MASK);
    }

    /// **SPI_MASTER_CS_POL** `[12:7]` (RW)
    ///
    /// In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_master_cs_pol(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MASTER_CS_POL_MASK) >> Self::SPI_MASTER_CS_POL_OFFSET
    }

    /// Write **SPI_MASTER_CS_POL** `[12:7]` (RW)
    ///
    /// In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_master_cs_pol(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MASTER_CS_POL_MASK)
                | ((value << Self::SPI_MASTER_CS_POL_OFFSET) & Self::SPI_MASTER_CS_POL_MASK)
        });
    }

    /// **SPI_CK_DIS** `[6]` (RW)
    ///
    /// 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CK_DIS_MASK) != 0
    }

    /// Set SPI_CK_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_ck_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CK_DIS_MASK);
    }

    /// Clear SPI_CK_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_ck_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CK_DIS_MASK);
    }

    /// **SPI_CS5_DIS** `[5]` (RW)
    ///
    /// SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs5_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS5_DIS_MASK) != 0
    }

    /// Set SPI_CS5_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs5_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS5_DIS_MASK);
    }

    /// Clear SPI_CS5_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs5_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS5_DIS_MASK);
    }

    /// **SPI_CS4_DIS** `[4]` (RW)
    ///
    /// SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs4_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS4_DIS_MASK) != 0
    }

    /// Set SPI_CS4_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs4_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS4_DIS_MASK);
    }

    /// Clear SPI_CS4_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs4_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS4_DIS_MASK);
    }

    /// **SPI_CS3_DIS** `[3]` (RW)
    ///
    /// SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs3_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS3_DIS_MASK) != 0
    }

    /// Set SPI_CS3_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs3_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS3_DIS_MASK);
    }

    /// Clear SPI_CS3_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs3_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS3_DIS_MASK);
    }

    /// **SPI_CS2_DIS** `[2]` (RW)
    ///
    /// SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS2_DIS_MASK) != 0
    }

    /// Set SPI_CS2_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs2_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS2_DIS_MASK);
    }

    /// Clear SPI_CS2_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs2_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS2_DIS_MASK);
    }

    /// **SPI_CS1_DIS** `[1]` (RW)
    ///
    /// SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS1_DIS_MASK) != 0
    }

    /// Set SPI_CS1_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs1_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS1_DIS_MASK);
    }

    /// Clear SPI_CS1_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs1_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS1_DIS_MASK);
    }

    /// **SPI_CS0_DIS** `[0]` (RW)
    ///
    /// SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CS0_DIS_MASK) != 0
    }

    /// Set SPI_CS0_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_cs0_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CS0_DIS_MASK);
    }

    /// Clear SPI_CS0_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_cs0_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CS0_DIS_MASK);
    }
}

/// SPI_MISC value type for batch read/modify/write operations
///
/// Use `spimisc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimisc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMiscVal {
    bits: u32,
}

impl SpiMiscVal {
    /// Create from raw bits
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
        Self { bits: 0x0000003e }
    }

    /// Read **SPI_QUAD_DIN_PIN_SWAP** `[31:31]`
    ///
    /// 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_quad_din_pin_swap(&self) -> bool {
        (self.bits & SpiMisc::SPI_QUAD_DIN_PIN_SWAP_MASK) != 0
    }

    /// Read **SPI_CS_KEEP_ACTIVE** `[30:30]`
    ///
    /// spi cs line keep low when the bit is set. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs_keep_active(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS_KEEP_ACTIVE_MASK) != 0
    }

    /// Read **SPI_CK_IDLE_EDGE** `[29:29]`
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_ck_idle_edge(&self) -> bool {
        (self.bits & SpiMisc::SPI_CK_IDLE_EDGE_MASK) != 0
    }

    /// Read **SPI_DQS_IDLE_EDGE** `[24:24]`
    ///
    /// The default value of spi_dqs. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dqs_idle_edge(&self) -> bool {
        (self.bits & SpiMisc::SPI_DQS_IDLE_EDGE_MASK) != 0
    }

    /// Read **SPI_SLAVE_CS_POL** `[23:23]`
    ///
    /// spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_slave_cs_pol(&self) -> bool {
        (self.bits & SpiMisc::SPI_SLAVE_CS_POL_MASK) != 0
    }

    /// Read **SPI_CMD_DTR_EN** `[19:19]`
    ///
    /// 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cmd_dtr_en(&self) -> bool {
        (self.bits & SpiMisc::SPI_CMD_DTR_EN_MASK) != 0
    }

    /// Read **SPI_ADDR_DTR_EN** `[18:18]`
    ///
    /// 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_addr_dtr_en(&self) -> bool {
        (self.bits & SpiMisc::SPI_ADDR_DTR_EN_MASK) != 0
    }

    /// Read **SPI_DATA_DTR_EN** `[17:17]`
    ///
    /// 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_data_dtr_en(&self) -> bool {
        (self.bits & SpiMisc::SPI_DATA_DTR_EN_MASK) != 0
    }

    /// Read **SPI_CLK_DATA_DTR_EN** `[16:16]`
    ///
    /// 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19.
    #[inline(always)]
    pub const fn spi_clk_data_dtr_en(&self) -> bool {
        (self.bits & SpiMisc::SPI_CLK_DATA_DTR_EN_MASK) != 0
    }

    /// Read **SPI_MASTER_CS_POL** `[12:7]`
    ///
    /// In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_master_cs_pol(&self) -> u32 {
        (self.bits & SpiMisc::SPI_MASTER_CS_POL_MASK) >> SpiMisc::SPI_MASTER_CS_POL_OFFSET
    }

    /// Read **SPI_CK_DIS** `[6:6]`
    ///
    /// 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_ck_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CK_DIS_MASK) != 0
    }

    /// Read **SPI_CS5_DIS** `[5:5]`
    ///
    /// SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs5_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS5_DIS_MASK) != 0
    }

    /// Read **SPI_CS4_DIS** `[4:4]`
    ///
    /// SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs4_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS4_DIS_MASK) != 0
    }

    /// Read **SPI_CS3_DIS** `[3:3]`
    ///
    /// SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs3_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS3_DIS_MASK) != 0
    }

    /// Read **SPI_CS2_DIS** `[2:2]`
    ///
    /// SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs2_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS2_DIS_MASK) != 0
    }

    /// Read **SPI_CS1_DIS** `[1:1]`
    ///
    /// SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs1_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS1_DIS_MASK) != 0
    }

    /// Read **SPI_CS0_DIS** `[0:0]`
    ///
    /// SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_cs0_dis(&self) -> bool {
        (self.bits & SpiMisc::SPI_CS0_DIS_MASK) != 0
    }

    /// Set **SPI_QUAD_DIN_PIN_SWAP** `[31:31]`
    ///
    /// 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_quad_din_pin_swap(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_QUAD_DIN_PIN_SWAP_MASK
            } else {
                self.bits & !SpiMisc::SPI_QUAD_DIN_PIN_SWAP_MASK
            },
        }
    }

    /// Set **SPI_CS_KEEP_ACTIVE** `[30:30]`
    ///
    /// spi cs line keep low when the bit is set. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs_keep_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS_KEEP_ACTIVE_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS_KEEP_ACTIVE_MASK
            },
        }
    }

    /// Set **SPI_CK_IDLE_EDGE** `[29:29]`
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_ck_idle_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CK_IDLE_EDGE_MASK
            } else {
                self.bits & !SpiMisc::SPI_CK_IDLE_EDGE_MASK
            },
        }
    }

    /// Set **SPI_DQS_IDLE_EDGE** `[24:24]`
    ///
    /// The default value of spi_dqs. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dqs_idle_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_DQS_IDLE_EDGE_MASK
            } else {
                self.bits & !SpiMisc::SPI_DQS_IDLE_EDGE_MASK
            },
        }
    }

    /// Set **SPI_SLAVE_CS_POL** `[23:23]`
    ///
    /// spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_slave_cs_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_SLAVE_CS_POL_MASK
            } else {
                self.bits & !SpiMisc::SPI_SLAVE_CS_POL_MASK
            },
        }
    }

    /// Set **SPI_CMD_DTR_EN** `[19:19]`
    ///
    /// 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cmd_dtr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CMD_DTR_EN_MASK
            } else {
                self.bits & !SpiMisc::SPI_CMD_DTR_EN_MASK
            },
        }
    }

    /// Set **SPI_ADDR_DTR_EN** `[18:18]`
    ///
    /// 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_addr_dtr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_ADDR_DTR_EN_MASK
            } else {
                self.bits & !SpiMisc::SPI_ADDR_DTR_EN_MASK
            },
        }
    }

    /// Set **SPI_DATA_DTR_EN** `[17:17]`
    ///
    /// 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_data_dtr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_DATA_DTR_EN_MASK
            } else {
                self.bits & !SpiMisc::SPI_DATA_DTR_EN_MASK
            },
        }
    }

    /// Set **SPI_CLK_DATA_DTR_EN** `[16:16]`
    ///
    /// 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19.
    #[inline(always)]
    pub const fn with_spi_clk_data_dtr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CLK_DATA_DTR_EN_MASK
            } else {
                self.bits & !SpiMisc::SPI_CLK_DATA_DTR_EN_MASK
            },
        }
    }

    /// Set **SPI_MASTER_CS_POL** `[12:7]`
    ///
    /// In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_master_cs_pol(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMisc::SPI_MASTER_CS_POL_MASK)
                | ((value << SpiMisc::SPI_MASTER_CS_POL_OFFSET) & SpiMisc::SPI_MASTER_CS_POL_MASK),
        }
    }

    /// Set **SPI_CK_DIS** `[6:6]`
    ///
    /// 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_ck_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CK_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CK_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS5_DIS** `[5:5]`
    ///
    /// SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs5_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS5_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS5_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS4_DIS** `[4:4]`
    ///
    /// SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs4_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS4_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS4_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS3_DIS** `[3:3]`
    ///
    /// SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs3_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS3_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS3_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS2_DIS** `[2:2]`
    ///
    /// SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs2_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS2_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS2_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS1_DIS** `[1:1]`
    ///
    /// SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs1_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS1_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS1_DIS_MASK
            },
        }
    }

    /// Set **SPI_CS0_DIS** `[0:0]`
    ///
    /// SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_cs0_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMisc::SPI_CS0_DIS_MASK
            } else {
                self.bits & !SpiMisc::SPI_CS0_DIS_MASK
            },
        }
    }
}

impl Default for SpiMiscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMisc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMiscVal {
        SpiMiscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMiscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMiscVal) -> SpiMiscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DIN_MODE Register
///
/// SPI input delay mode configuration
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SPI_TIMING_HCLK_ACTIVE** `[16]` (RW)
///
///   1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state.
///
/// - **SPI_DIN7_MODE** `[15:14]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN6_MODE** `[13:12]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN5_MODE** `[11:10]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN4_MODE** `[9:8]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN3_MODE** `[7:6]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN2_MODE** `[5:4]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN1_MODE** `[3:2]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
///
/// - **SPI_DIN0_MODE** `[1:0]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiDinMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDinMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDinMode {
    // ======== Field Constants ========

    /// **SPI_TIMING_HCLK_ACTIVE**
    ///
    /// 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_TIMING_HCLK_ACTIVE_OFFSET: u32 = 16;
    pub const SPI_TIMING_HCLK_ACTIVE_MASK: u32 = 0x00010000;
    pub const SPI_TIMING_HCLK_ACTIVE: u32 = 1 << 16;

    /// **SPI_DIN7_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SPI_DIN7_MODE_OFFSET: u32 = 14;
    pub const SPI_DIN7_MODE_MASK: u32 = 0x0000c000;

    /// **SPI_DIN6_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const SPI_DIN6_MODE_OFFSET: u32 = 12;
    pub const SPI_DIN6_MODE_MASK: u32 = 0x00003000;

    /// **SPI_DIN5_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const SPI_DIN5_MODE_OFFSET: u32 = 10;
    pub const SPI_DIN5_MODE_MASK: u32 = 0x00000c00;

    /// **SPI_DIN4_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPI_DIN4_MODE_OFFSET: u32 = 8;
    pub const SPI_DIN4_MODE_MASK: u32 = 0x00000300;

    /// **SPI_DIN3_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SPI_DIN3_MODE_OFFSET: u32 = 6;
    pub const SPI_DIN3_MODE_MASK: u32 = 0x000000c0;

    /// **SPI_DIN2_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SPI_DIN2_MODE_OFFSET: u32 = 4;
    pub const SPI_DIN2_MODE_MASK: u32 = 0x00000030;

    /// **SPI_DIN1_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SPI_DIN1_MODE_OFFSET: u32 = 2;
    pub const SPI_DIN1_MODE_MASK: u32 = 0x0000000c;

    /// **SPI_DIN0_MODE**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_DIN0_MODE_OFFSET: u32 = 0;
    pub const SPI_DIN0_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_TIMING_HCLK_ACTIVE** `[16]` (RW)
    ///
    /// 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_timing_hclk_active(&self) -> bool {
        (self.value.read_bits() & Self::SPI_TIMING_HCLK_ACTIVE_MASK) != 0
    }

    /// Set SPI_TIMING_HCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_timing_hclk_active(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_TIMING_HCLK_ACTIVE_MASK);
    }

    /// Clear SPI_TIMING_HCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_timing_hclk_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_TIMING_HCLK_ACTIVE_MASK);
    }

    /// **SPI_DIN7_MODE** `[15:14]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din7_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN7_MODE_MASK) >> Self::SPI_DIN7_MODE_OFFSET
    }

    /// Write **SPI_DIN7_MODE** `[15:14]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din7_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN7_MODE_MASK)
                | ((value << Self::SPI_DIN7_MODE_OFFSET) & Self::SPI_DIN7_MODE_MASK)
        });
    }

    /// **SPI_DIN6_MODE** `[13:12]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din6_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN6_MODE_MASK) >> Self::SPI_DIN6_MODE_OFFSET
    }

    /// Write **SPI_DIN6_MODE** `[13:12]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din6_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN6_MODE_MASK)
                | ((value << Self::SPI_DIN6_MODE_OFFSET) & Self::SPI_DIN6_MODE_MASK)
        });
    }

    /// **SPI_DIN5_MODE** `[11:10]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din5_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN5_MODE_MASK) >> Self::SPI_DIN5_MODE_OFFSET
    }

    /// Write **SPI_DIN5_MODE** `[11:10]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din5_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN5_MODE_MASK)
                | ((value << Self::SPI_DIN5_MODE_OFFSET) & Self::SPI_DIN5_MODE_MASK)
        });
    }

    /// **SPI_DIN4_MODE** `[9:8]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din4_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN4_MODE_MASK) >> Self::SPI_DIN4_MODE_OFFSET
    }

    /// Write **SPI_DIN4_MODE** `[9:8]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din4_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN4_MODE_MASK)
                | ((value << Self::SPI_DIN4_MODE_OFFSET) & Self::SPI_DIN4_MODE_MASK)
        });
    }

    /// **SPI_DIN3_MODE** `[7:6]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din3_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN3_MODE_MASK) >> Self::SPI_DIN3_MODE_OFFSET
    }

    /// Write **SPI_DIN3_MODE** `[7:6]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din3_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN3_MODE_MASK)
                | ((value << Self::SPI_DIN3_MODE_OFFSET) & Self::SPI_DIN3_MODE_MASK)
        });
    }

    /// **SPI_DIN2_MODE** `[5:4]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din2_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN2_MODE_MASK) >> Self::SPI_DIN2_MODE_OFFSET
    }

    /// Write **SPI_DIN2_MODE** `[5:4]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din2_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN2_MODE_MASK)
                | ((value << Self::SPI_DIN2_MODE_OFFSET) & Self::SPI_DIN2_MODE_MASK)
        });
    }

    /// **SPI_DIN1_MODE** `[3:2]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din1_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN1_MODE_MASK) >> Self::SPI_DIN1_MODE_OFFSET
    }

    /// Write **SPI_DIN1_MODE** `[3:2]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din1_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN1_MODE_MASK)
                | ((value << Self::SPI_DIN1_MODE_OFFSET) & Self::SPI_DIN1_MODE_MASK)
        });
    }

    /// **SPI_DIN0_MODE** `[1:0]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din0_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN0_MODE_MASK) >> Self::SPI_DIN0_MODE_OFFSET
    }

    /// Write **SPI_DIN0_MODE** `[1:0]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din0_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN0_MODE_MASK)
                | ((value << Self::SPI_DIN0_MODE_OFFSET) & Self::SPI_DIN0_MODE_MASK)
        });
    }
}

/// SPI_DIN_MODE value type for batch read/modify/write operations
///
/// Use `spidinmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidinmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDinModeVal {
    bits: u32,
}

impl SpiDinModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_TIMING_HCLK_ACTIVE** `[16:16]`
    ///
    /// 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_timing_hclk_active(&self) -> bool {
        (self.bits & SpiDinMode::SPI_TIMING_HCLK_ACTIVE_MASK) != 0
    }

    /// Read **SPI_DIN7_MODE** `[15:14]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din7_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN7_MODE_MASK) >> SpiDinMode::SPI_DIN7_MODE_OFFSET
    }

    /// Read **SPI_DIN6_MODE** `[13:12]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din6_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN6_MODE_MASK) >> SpiDinMode::SPI_DIN6_MODE_OFFSET
    }

    /// Read **SPI_DIN5_MODE** `[11:10]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din5_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN5_MODE_MASK) >> SpiDinMode::SPI_DIN5_MODE_OFFSET
    }

    /// Read **SPI_DIN4_MODE** `[9:8]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din4_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN4_MODE_MASK) >> SpiDinMode::SPI_DIN4_MODE_OFFSET
    }

    /// Read **SPI_DIN3_MODE** `[7:6]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din3_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN3_MODE_MASK) >> SpiDinMode::SPI_DIN3_MODE_OFFSET
    }

    /// Read **SPI_DIN2_MODE** `[5:4]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din2_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN2_MODE_MASK) >> SpiDinMode::SPI_DIN2_MODE_OFFSET
    }

    /// Read **SPI_DIN1_MODE** `[3:2]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din1_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN1_MODE_MASK) >> SpiDinMode::SPI_DIN1_MODE_OFFSET
    }

    /// Read **SPI_DIN0_MODE** `[1:0]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din0_mode(&self) -> u32 {
        (self.bits & SpiDinMode::SPI_DIN0_MODE_MASK) >> SpiDinMode::SPI_DIN0_MODE_OFFSET
    }

    /// Set **SPI_TIMING_HCLK_ACTIVE** `[16:16]`
    ///
    /// 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_timing_hclk_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDinMode::SPI_TIMING_HCLK_ACTIVE_MASK
            } else {
                self.bits & !SpiDinMode::SPI_TIMING_HCLK_ACTIVE_MASK
            },
        }
    }

    /// Set **SPI_DIN7_MODE** `[15:14]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din7_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN7_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN7_MODE_OFFSET) & SpiDinMode::SPI_DIN7_MODE_MASK),
        }
    }

    /// Set **SPI_DIN6_MODE** `[13:12]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din6_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN6_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN6_MODE_OFFSET) & SpiDinMode::SPI_DIN6_MODE_MASK),
        }
    }

    /// Set **SPI_DIN5_MODE** `[11:10]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din5_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN5_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN5_MODE_OFFSET) & SpiDinMode::SPI_DIN5_MODE_MASK),
        }
    }

    /// Set **SPI_DIN4_MODE** `[9:8]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din4_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN4_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN4_MODE_OFFSET) & SpiDinMode::SPI_DIN4_MODE_MASK),
        }
    }

    /// Set **SPI_DIN3_MODE** `[7:6]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din3_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN3_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN3_MODE_OFFSET) & SpiDinMode::SPI_DIN3_MODE_MASK),
        }
    }

    /// Set **SPI_DIN2_MODE** `[5:4]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din2_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN2_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN2_MODE_OFFSET) & SpiDinMode::SPI_DIN2_MODE_MASK),
        }
    }

    /// Set **SPI_DIN1_MODE** `[3:2]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din1_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN1_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN1_MODE_OFFSET) & SpiDinMode::SPI_DIN1_MODE_MASK),
        }
    }

    /// Set **SPI_DIN0_MODE** `[1:0]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din0_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinMode::SPI_DIN0_MODE_MASK)
                | ((value << SpiDinMode::SPI_DIN0_MODE_OFFSET) & SpiDinMode::SPI_DIN0_MODE_MASK),
        }
    }
}

impl Default for SpiDinModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDinMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDinModeVal {
        SpiDinModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDinModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDinModeVal) -> SpiDinModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DIN_NUM Register
///
/// SPI input delay number configuration
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SPI_DIN7_NUM** `[15:14]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN6_NUM** `[13:12]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN5_NUM** `[11:10]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN4_NUM** `[9:8]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN3_NUM** `[7:6]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN2_NUM** `[5:4]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN1_NUM** `[3:2]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
///
/// - **SPI_DIN0_NUM** `[1:0]` (RW)
///
///   the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiDinNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDinNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDinNum {
    // ======== Field Constants ========

    /// **SPI_DIN7_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SPI_DIN7_NUM_OFFSET: u32 = 14;
    pub const SPI_DIN7_NUM_MASK: u32 = 0x0000c000;

    /// **SPI_DIN6_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const SPI_DIN6_NUM_OFFSET: u32 = 12;
    pub const SPI_DIN6_NUM_MASK: u32 = 0x00003000;

    /// **SPI_DIN5_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const SPI_DIN5_NUM_OFFSET: u32 = 10;
    pub const SPI_DIN5_NUM_MASK: u32 = 0x00000c00;

    /// **SPI_DIN4_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPI_DIN4_NUM_OFFSET: u32 = 8;
    pub const SPI_DIN4_NUM_MASK: u32 = 0x00000300;

    /// **SPI_DIN3_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SPI_DIN3_NUM_OFFSET: u32 = 6;
    pub const SPI_DIN3_NUM_MASK: u32 = 0x000000c0;

    /// **SPI_DIN2_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SPI_DIN2_NUM_OFFSET: u32 = 4;
    pub const SPI_DIN2_NUM_MASK: u32 = 0x00000030;

    /// **SPI_DIN1_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SPI_DIN1_NUM_OFFSET: u32 = 2;
    pub const SPI_DIN1_NUM_MASK: u32 = 0x0000000c;

    /// **SPI_DIN0_NUM**
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_DIN0_NUM_OFFSET: u32 = 0;
    pub const SPI_DIN0_NUM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din7_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN7_NUM_MASK) >> Self::SPI_DIN7_NUM_OFFSET
    }

    /// Write **SPI_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din7_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN7_NUM_MASK)
                | ((value << Self::SPI_DIN7_NUM_OFFSET) & Self::SPI_DIN7_NUM_MASK)
        });
    }

    /// **SPI_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din6_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN6_NUM_MASK) >> Self::SPI_DIN6_NUM_OFFSET
    }

    /// Write **SPI_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din6_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN6_NUM_MASK)
                | ((value << Self::SPI_DIN6_NUM_OFFSET) & Self::SPI_DIN6_NUM_MASK)
        });
    }

    /// **SPI_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din5_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN5_NUM_MASK) >> Self::SPI_DIN5_NUM_OFFSET
    }

    /// Write **SPI_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din5_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN5_NUM_MASK)
                | ((value << Self::SPI_DIN5_NUM_OFFSET) & Self::SPI_DIN5_NUM_MASK)
        });
    }

    /// **SPI_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din4_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN4_NUM_MASK) >> Self::SPI_DIN4_NUM_OFFSET
    }

    /// Write **SPI_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din4_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN4_NUM_MASK)
                | ((value << Self::SPI_DIN4_NUM_OFFSET) & Self::SPI_DIN4_NUM_MASK)
        });
    }

    /// **SPI_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din3_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN3_NUM_MASK) >> Self::SPI_DIN3_NUM_OFFSET
    }

    /// Write **SPI_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din3_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN3_NUM_MASK)
                | ((value << Self::SPI_DIN3_NUM_OFFSET) & Self::SPI_DIN3_NUM_MASK)
        });
    }

    /// **SPI_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din2_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN2_NUM_MASK) >> Self::SPI_DIN2_NUM_OFFSET
    }

    /// Write **SPI_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din2_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN2_NUM_MASK)
                | ((value << Self::SPI_DIN2_NUM_OFFSET) & Self::SPI_DIN2_NUM_MASK)
        });
    }

    /// **SPI_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din1_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN1_NUM_MASK) >> Self::SPI_DIN1_NUM_OFFSET
    }

    /// Write **SPI_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din1_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN1_NUM_MASK)
                | ((value << Self::SPI_DIN1_NUM_OFFSET) & Self::SPI_DIN1_NUM_MASK)
        });
    }

    /// **SPI_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_din0_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DIN0_NUM_MASK) >> Self::SPI_DIN0_NUM_OFFSET
    }

    /// Write **SPI_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_din0_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DIN0_NUM_MASK)
                | ((value << Self::SPI_DIN0_NUM_OFFSET) & Self::SPI_DIN0_NUM_MASK)
        });
    }
}

/// SPI_DIN_NUM value type for batch read/modify/write operations
///
/// Use `spidinnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidinnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDinNumVal {
    bits: u32,
}

impl SpiDinNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din7_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN7_NUM_MASK) >> SpiDinNum::SPI_DIN7_NUM_OFFSET
    }

    /// Read **SPI_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din6_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN6_NUM_MASK) >> SpiDinNum::SPI_DIN6_NUM_OFFSET
    }

    /// Read **SPI_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din5_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN5_NUM_MASK) >> SpiDinNum::SPI_DIN5_NUM_OFFSET
    }

    /// Read **SPI_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din4_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN4_NUM_MASK) >> SpiDinNum::SPI_DIN4_NUM_OFFSET
    }

    /// Read **SPI_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din3_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN3_NUM_MASK) >> SpiDinNum::SPI_DIN3_NUM_OFFSET
    }

    /// Read **SPI_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din2_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN2_NUM_MASK) >> SpiDinNum::SPI_DIN2_NUM_OFFSET
    }

    /// Read **SPI_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din1_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN1_NUM_MASK) >> SpiDinNum::SPI_DIN1_NUM_OFFSET
    }

    /// Read **SPI_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_din0_num(&self) -> u32 {
        (self.bits & SpiDinNum::SPI_DIN0_NUM_MASK) >> SpiDinNum::SPI_DIN0_NUM_OFFSET
    }

    /// Set **SPI_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din7_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN7_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN7_NUM_OFFSET) & SpiDinNum::SPI_DIN7_NUM_MASK),
        }
    }

    /// Set **SPI_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din6_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN6_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN6_NUM_OFFSET) & SpiDinNum::SPI_DIN6_NUM_MASK),
        }
    }

    /// Set **SPI_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din5_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN5_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN5_NUM_OFFSET) & SpiDinNum::SPI_DIN5_NUM_MASK),
        }
    }

    /// Set **SPI_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din4_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN4_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN4_NUM_OFFSET) & SpiDinNum::SPI_DIN4_NUM_MASK),
        }
    }

    /// Set **SPI_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din3_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN3_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN3_NUM_OFFSET) & SpiDinNum::SPI_DIN3_NUM_MASK),
        }
    }

    /// Set **SPI_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din2_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN2_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN2_NUM_OFFSET) & SpiDinNum::SPI_DIN2_NUM_MASK),
        }
    }

    /// Set **SPI_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din1_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN1_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN1_NUM_OFFSET) & SpiDinNum::SPI_DIN1_NUM_MASK),
        }
    }

    /// Set **SPI_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_din0_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDinNum::SPI_DIN0_NUM_MASK)
                | ((value << SpiDinNum::SPI_DIN0_NUM_OFFSET) & SpiDinNum::SPI_DIN0_NUM_MASK),
        }
    }
}

impl Default for SpiDinNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDinNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDinNumVal {
        SpiDinNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDinNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDinNumVal) -> SpiDinNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DOUT_MODE Register
///
/// SPI output delay mode configuration
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SPI_D_DQS_MODE** `[8]` (RW)
///
///   The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT7_MODE** `[7]` (RW)
///
///   The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT6_MODE** `[6]` (RW)
///
///   The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT5_MODE** `[5]` (RW)
///
///   The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT4_MODE** `[4]` (RW)
///
///   The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT3_MODE** `[3]` (RW)
///
///   The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT2_MODE** `[2]` (RW)
///
///   The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT1_MODE** `[1]` (RW)
///
///   The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
///
/// - **SPI_DOUT0_MODE** `[0]` (RW)
///
///   The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiDoutMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDoutMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDoutMode {
    // ======== Field Constants ========

    /// **SPI_D_DQS_MODE**
    ///
    /// The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_D_DQS_MODE_OFFSET: u32 = 8;
    pub const SPI_D_DQS_MODE_MASK: u32 = 0x00000100;
    pub const SPI_D_DQS_MODE: u32 = 1 << 8;

    /// **SPI_DOUT7_MODE**
    ///
    /// The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_DOUT7_MODE_OFFSET: u32 = 7;
    pub const SPI_DOUT7_MODE_MASK: u32 = 0x00000080;
    pub const SPI_DOUT7_MODE: u32 = 1 << 7;

    /// **SPI_DOUT6_MODE**
    ///
    /// The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_DOUT6_MODE_OFFSET: u32 = 6;
    pub const SPI_DOUT6_MODE_MASK: u32 = 0x00000040;
    pub const SPI_DOUT6_MODE: u32 = 1 << 6;

    /// **SPI_DOUT5_MODE**
    ///
    /// The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_DOUT5_MODE_OFFSET: u32 = 5;
    pub const SPI_DOUT5_MODE_MASK: u32 = 0x00000020;
    pub const SPI_DOUT5_MODE: u32 = 1 << 5;

    /// **SPI_DOUT4_MODE**
    ///
    /// The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_DOUT4_MODE_OFFSET: u32 = 4;
    pub const SPI_DOUT4_MODE_MASK: u32 = 0x00000010;
    pub const SPI_DOUT4_MODE: u32 = 1 << 4;

    /// **SPI_DOUT3_MODE**
    ///
    /// The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_DOUT3_MODE_OFFSET: u32 = 3;
    pub const SPI_DOUT3_MODE_MASK: u32 = 0x00000008;
    pub const SPI_DOUT3_MODE: u32 = 1 << 3;

    /// **SPI_DOUT2_MODE**
    ///
    /// The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_DOUT2_MODE_OFFSET: u32 = 2;
    pub const SPI_DOUT2_MODE_MASK: u32 = 0x00000004;
    pub const SPI_DOUT2_MODE: u32 = 1 << 2;

    /// **SPI_DOUT1_MODE**
    ///
    /// The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DOUT1_MODE_OFFSET: u32 = 1;
    pub const SPI_DOUT1_MODE_MASK: u32 = 0x00000002;
    pub const SPI_DOUT1_MODE: u32 = 1 << 1;

    /// **SPI_DOUT0_MODE**
    ///
    /// The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DOUT0_MODE_OFFSET: u32 = 0;
    pub const SPI_DOUT0_MODE_MASK: u32 = 0x00000001;
    pub const SPI_DOUT0_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_D_DQS_MODE** `[8]` (RW)
    ///
    /// The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_d_dqs_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_D_DQS_MODE_MASK) != 0
    }

    /// Set SPI_D_DQS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_d_dqs_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_D_DQS_MODE_MASK);
    }

    /// Clear SPI_D_DQS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_d_dqs_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_D_DQS_MODE_MASK);
    }

    /// **SPI_DOUT7_MODE** `[7]` (RW)
    ///
    /// The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout7_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT7_MODE_MASK) != 0
    }

    /// Set SPI_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout7_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT7_MODE_MASK);
    }

    /// Clear SPI_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout7_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT7_MODE_MASK);
    }

    /// **SPI_DOUT6_MODE** `[6]` (RW)
    ///
    /// The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout6_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT6_MODE_MASK) != 0
    }

    /// Set SPI_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout6_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT6_MODE_MASK);
    }

    /// Clear SPI_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout6_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT6_MODE_MASK);
    }

    /// **SPI_DOUT5_MODE** `[5]` (RW)
    ///
    /// The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout5_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT5_MODE_MASK) != 0
    }

    /// Set SPI_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout5_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT5_MODE_MASK);
    }

    /// Clear SPI_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout5_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT5_MODE_MASK);
    }

    /// **SPI_DOUT4_MODE** `[4]` (RW)
    ///
    /// The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout4_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT4_MODE_MASK) != 0
    }

    /// Set SPI_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout4_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT4_MODE_MASK);
    }

    /// Clear SPI_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout4_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT4_MODE_MASK);
    }

    /// **SPI_DOUT3_MODE** `[3]` (RW)
    ///
    /// The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout3_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT3_MODE_MASK) != 0
    }

    /// Set SPI_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout3_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT3_MODE_MASK);
    }

    /// Clear SPI_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout3_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT3_MODE_MASK);
    }

    /// **SPI_DOUT2_MODE** `[2]` (RW)
    ///
    /// The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout2_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT2_MODE_MASK) != 0
    }

    /// Set SPI_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout2_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT2_MODE_MASK);
    }

    /// Clear SPI_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout2_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT2_MODE_MASK);
    }

    /// **SPI_DOUT1_MODE** `[1]` (RW)
    ///
    /// The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout1_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT1_MODE_MASK) != 0
    }

    /// Set SPI_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout1_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT1_MODE_MASK);
    }

    /// Clear SPI_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout1_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT1_MODE_MASK);
    }

    /// **SPI_DOUT0_MODE** `[0]` (RW)
    ///
    /// The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_dout0_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOUT0_MODE_MASK) != 0
    }

    /// Set SPI_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dout0_mode(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DOUT0_MODE_MASK);
    }

    /// Clear SPI_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dout0_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DOUT0_MODE_MASK);
    }
}

/// SPI_DOUT_MODE value type for batch read/modify/write operations
///
/// Use `spidoutmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidoutmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDoutModeVal {
    bits: u32,
}

impl SpiDoutModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_D_DQS_MODE** `[8:8]`
    ///
    /// The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_d_dqs_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_D_DQS_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT7_MODE** `[7:7]`
    ///
    /// The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout7_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT7_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT6_MODE** `[6:6]`
    ///
    /// The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout6_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT6_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT5_MODE** `[5:5]`
    ///
    /// The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout5_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT5_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT4_MODE** `[4:4]`
    ///
    /// The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout4_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT4_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT3_MODE** `[3:3]`
    ///
    /// The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout3_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT3_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT2_MODE** `[2:2]`
    ///
    /// The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout2_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT2_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT1_MODE** `[1:1]`
    ///
    /// The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout1_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT1_MODE_MASK) != 0
    }

    /// Read **SPI_DOUT0_MODE** `[0:0]`
    ///
    /// The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_dout0_mode(&self) -> bool {
        (self.bits & SpiDoutMode::SPI_DOUT0_MODE_MASK) != 0
    }

    /// Set **SPI_D_DQS_MODE** `[8:8]`
    ///
    /// The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_d_dqs_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_D_DQS_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_D_DQS_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT7_MODE** `[7:7]`
    ///
    /// The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout7_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT7_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT7_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT6_MODE** `[6:6]`
    ///
    /// The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout6_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT6_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT6_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT5_MODE** `[5:5]`
    ///
    /// The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout5_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT5_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT5_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT4_MODE** `[4:4]`
    ///
    /// The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout4_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT4_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT4_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT3_MODE** `[3:3]`
    ///
    /// The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout3_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT3_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT3_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT2_MODE** `[2:2]`
    ///
    /// The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout2_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT2_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT2_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT1_MODE** `[1:1]`
    ///
    /// The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout1_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT1_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT1_MODE_MASK
            },
        }
    }

    /// Set **SPI_DOUT0_MODE** `[0:0]`
    ///
    /// The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_dout0_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDoutMode::SPI_DOUT0_MODE_MASK
            } else {
                self.bits & !SpiDoutMode::SPI_DOUT0_MODE_MASK
            },
        }
    }
}

impl Default for SpiDoutModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDoutMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDoutModeVal {
        SpiDoutModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDoutModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDoutModeVal) -> SpiDoutModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DMA_CONF Register
///
/// SPI DMA control register
///
/// Address offset: 0x0030
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_DMA_AFIFO_RST** `[31]` (WO)
///
///   Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer.
///
/// - **SPI_BUF_AFIFO_RST** `[30]` (WO)
///
///   Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer.
///
/// - **SPI_RX_AFIFO_RST** `[29]` (WO)
///
///   Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer.
///
/// - **SPI_DMA_TX_ENA** `[28]` (RW)
///
///   Set this bit to enable SPI DMA controlled send data mode.
///
/// - **SPI_DMA_RX_ENA** `[27]` (RW)
///
///   Set this bit to enable SPI DMA controlled receive data mode.
///
/// - **SPI_RX_EOF_EN** `[21]` (RW)
///
///   1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen[19:0] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans.
///
/// - **SPI_SLV_TX_SEG_TRANS_CLR_EN** `[20]` (RW)
///
///   1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done.
///
/// - **SPI_SLV_RX_SEG_TRANS_CLR_EN** `[19]` (RW)
///
///   1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done.
///
/// - **SPI_DMA_SLV_SEG_TRANS_EN** `[18]` (RW)
///
///   Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable.
///
/// - **SPI_DMA_INFIFO_FULL** `[1]` (RO)
///
///   Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data.
///
/// - **SPI_DMA_OUTFIFO_EMPTY** `[0]` (RO)
///
///   Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data.
#[repr(transparent)]
pub struct SpiDmaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaConf {
    // ======== Field Constants ========

    /// **SPI_DMA_AFIFO_RST**
    ///
    /// Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_DMA_AFIFO_RST_OFFSET: u32 = 31;
    pub const SPI_DMA_AFIFO_RST_MASK: u32 = 0x80000000;
    pub const SPI_DMA_AFIFO_RST: u32 = 1 << 31;

    /// **SPI_BUF_AFIFO_RST**
    ///
    /// Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_BUF_AFIFO_RST_OFFSET: u32 = 30;
    pub const SPI_BUF_AFIFO_RST_MASK: u32 = 0x40000000;
    pub const SPI_BUF_AFIFO_RST: u32 = 1 << 30;

    /// **SPI_RX_AFIFO_RST**
    ///
    /// Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_RX_AFIFO_RST_OFFSET: u32 = 29;
    pub const SPI_RX_AFIFO_RST_MASK: u32 = 0x20000000;
    pub const SPI_RX_AFIFO_RST: u32 = 1 << 29;

    /// **SPI_DMA_TX_ENA**
    ///
    /// Set this bit to enable SPI DMA controlled send data mode.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_DMA_TX_ENA_OFFSET: u32 = 28;
    pub const SPI_DMA_TX_ENA_MASK: u32 = 0x10000000;
    pub const SPI_DMA_TX_ENA: u32 = 1 << 28;

    /// **SPI_DMA_RX_ENA**
    ///
    /// Set this bit to enable SPI DMA controlled receive data mode.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_DMA_RX_ENA_OFFSET: u32 = 27;
    pub const SPI_DMA_RX_ENA_MASK: u32 = 0x08000000;
    pub const SPI_DMA_RX_ENA: u32 = 1 << 27;

    /// **SPI_RX_EOF_EN**
    ///
    /// 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen[19:0] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_RX_EOF_EN_OFFSET: u32 = 21;
    pub const SPI_RX_EOF_EN_MASK: u32 = 0x00200000;
    pub const SPI_RX_EOF_EN: u32 = 1 << 21;

    /// **SPI_SLV_TX_SEG_TRANS_CLR_EN**
    ///
    /// 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_SLV_TX_SEG_TRANS_CLR_EN_OFFSET: u32 = 20;
    pub const SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK: u32 = 0x00100000;
    pub const SPI_SLV_TX_SEG_TRANS_CLR_EN: u32 = 1 << 20;

    /// **SPI_SLV_RX_SEG_TRANS_CLR_EN**
    ///
    /// 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_SLV_RX_SEG_TRANS_CLR_EN_OFFSET: u32 = 19;
    pub const SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK: u32 = 0x00080000;
    pub const SPI_SLV_RX_SEG_TRANS_CLR_EN: u32 = 1 << 19;

    /// **SPI_DMA_SLV_SEG_TRANS_EN**
    ///
    /// Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_DMA_SLV_SEG_TRANS_EN_OFFSET: u32 = 18;
    pub const SPI_DMA_SLV_SEG_TRANS_EN_MASK: u32 = 0x00040000;
    pub const SPI_DMA_SLV_SEG_TRANS_EN: u32 = 1 << 18;

    /// **SPI_DMA_INFIFO_FULL**
    ///
    /// Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_INFIFO_FULL_OFFSET: u32 = 1;
    pub const SPI_DMA_INFIFO_FULL_MASK: u32 = 0x00000002;
    pub const SPI_DMA_INFIFO_FULL: u32 = 1 << 1;

    /// **SPI_DMA_OUTFIFO_EMPTY**
    ///
    /// Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_OUTFIFO_EMPTY_OFFSET: u32 = 0;
    pub const SPI_DMA_OUTFIFO_EMPTY_MASK: u32 = 0x00000001;
    pub const SPI_DMA_OUTFIFO_EMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_DMA_AFIFO_RST** (WO)
    ///
    /// Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer.
    #[inline(always)]
    pub fn trigger_spi_dma_afifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DMA_AFIFO_RST_MASK);
    }

    /// Trigger **SPI_BUF_AFIFO_RST** (WO)
    ///
    /// Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer.
    #[inline(always)]
    pub fn trigger_spi_buf_afifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::SPI_BUF_AFIFO_RST_MASK);
    }

    /// Trigger **SPI_RX_AFIFO_RST** (WO)
    ///
    /// Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer.
    #[inline(always)]
    pub fn trigger_spi_rx_afifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::SPI_RX_AFIFO_RST_MASK);
    }

    /// **SPI_DMA_TX_ENA** `[28]` (RW)
    ///
    /// Set this bit to enable SPI DMA controlled send data mode.
    #[inline(always)]
    pub fn spi_dma_tx_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_TX_ENA_MASK) != 0
    }

    /// Set SPI_DMA_TX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_tx_ena(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DMA_TX_ENA_MASK);
    }

    /// Clear SPI_DMA_TX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_tx_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DMA_TX_ENA_MASK);
    }

    /// **SPI_DMA_RX_ENA** `[27]` (RW)
    ///
    /// Set this bit to enable SPI DMA controlled receive data mode.
    #[inline(always)]
    pub fn spi_dma_rx_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_RX_ENA_MASK) != 0
    }

    /// Set SPI_DMA_RX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_rx_ena(&self) {
        self.value.modify_bits(|r| r | Self::SPI_DMA_RX_ENA_MASK);
    }

    /// Clear SPI_DMA_RX_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_rx_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_DMA_RX_ENA_MASK);
    }

    /// **SPI_RX_EOF_EN** `[21]` (RW)
    ///
    /// 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen[19:0] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans.
    #[inline(always)]
    pub fn spi_rx_eof_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_RX_EOF_EN_MASK) != 0
    }

    /// Set SPI_RX_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_rx_eof_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_RX_EOF_EN_MASK);
    }

    /// Clear SPI_RX_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_rx_eof_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_RX_EOF_EN_MASK);
    }

    /// **SPI_SLV_TX_SEG_TRANS_CLR_EN** `[20]` (RW)
    ///
    /// 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK) != 0
    }

    /// Set SPI_SLV_TX_SEG_TRANS_CLR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_tx_seg_trans_clr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK);
    }

    /// Clear SPI_SLV_TX_SEG_TRANS_CLR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_tx_seg_trans_clr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK);
    }

    /// **SPI_SLV_RX_SEG_TRANS_CLR_EN** `[19]` (RW)
    ///
    /// 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK) != 0
    }

    /// Set SPI_SLV_RX_SEG_TRANS_CLR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rx_seg_trans_clr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK);
    }

    /// Clear SPI_SLV_RX_SEG_TRANS_CLR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rx_seg_trans_clr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK);
    }

    /// **SPI_DMA_SLV_SEG_TRANS_EN** `[18]` (RW)
    ///
    /// Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable.
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_SLV_SEG_TRANS_EN_MASK) != 0
    }

    /// Set SPI_DMA_SLV_SEG_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_slv_seg_trans_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_SLV_SEG_TRANS_EN_MASK);
    }

    /// Clear SPI_DMA_SLV_SEG_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_slv_seg_trans_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_SLV_SEG_TRANS_EN_MASK);
    }

    /// **SPI_DMA_INFIFO_FULL** `[1]` (RO)
    ///
    /// Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data.
    #[inline(always)]
    pub fn spi_dma_infifo_full(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_INFIFO_FULL_MASK) != 0
    }

    /// **SPI_DMA_OUTFIFO_EMPTY** `[0]` (RO)
    ///
    /// Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data.
    #[inline(always)]
    pub fn spi_dma_outfifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_OUTFIFO_EMPTY_MASK) != 0
    }
}

/// SPI_DMA_CONF value type for batch read/modify/write operations
///
/// Use `spidmaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaConfVal {
    bits: u32,
}

impl SpiDmaConfVal {
    /// Create from raw bits
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

    /// Read **SPI_DMA_TX_ENA** `[28:28]`
    ///
    /// Set this bit to enable SPI DMA controlled send data mode.
    #[inline(always)]
    pub const fn spi_dma_tx_ena(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_DMA_TX_ENA_MASK) != 0
    }

    /// Read **SPI_DMA_RX_ENA** `[27:27]`
    ///
    /// Set this bit to enable SPI DMA controlled receive data mode.
    #[inline(always)]
    pub const fn spi_dma_rx_ena(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_DMA_RX_ENA_MASK) != 0
    }

    /// Read **SPI_RX_EOF_EN** `[21:21]`
    ///
    /// 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen[19:0] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans.
    #[inline(always)]
    pub const fn spi_rx_eof_en(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_RX_EOF_EN_MASK) != 0
    }

    /// Read **SPI_SLV_TX_SEG_TRANS_CLR_EN** `[20:20]`
    ///
    /// 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub const fn spi_slv_tx_seg_trans_clr_en(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK) != 0
    }

    /// Read **SPI_SLV_RX_SEG_TRANS_CLR_EN** `[19:19]`
    ///
    /// 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub const fn spi_slv_rx_seg_trans_clr_en(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK) != 0
    }

    /// Read **SPI_DMA_SLV_SEG_TRANS_EN** `[18:18]`
    ///
    /// Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable.
    #[inline(always)]
    pub const fn spi_dma_slv_seg_trans_en(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_DMA_SLV_SEG_TRANS_EN_MASK) != 0
    }

    /// Read **SPI_DMA_INFIFO_FULL** `[1:1]`
    ///
    /// Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data.
    #[inline(always)]
    pub const fn spi_dma_infifo_full(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_DMA_INFIFO_FULL_MASK) != 0
    }

    /// Read **SPI_DMA_OUTFIFO_EMPTY** `[0:0]`
    ///
    /// Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data.
    #[inline(always)]
    pub const fn spi_dma_outfifo_empty(&self) -> bool {
        (self.bits & SpiDmaConf::SPI_DMA_OUTFIFO_EMPTY_MASK) != 0
    }

    /// Set **SPI_DMA_AFIFO_RST** `[31:31]`
    ///
    /// Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer.
    #[inline(always)]
    pub const fn with_spi_dma_afifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_DMA_AFIFO_RST_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_DMA_AFIFO_RST_MASK
            },
        }
    }

    /// Set **SPI_BUF_AFIFO_RST** `[30:30]`
    ///
    /// Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer.
    #[inline(always)]
    pub const fn with_spi_buf_afifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_BUF_AFIFO_RST_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_BUF_AFIFO_RST_MASK
            },
        }
    }

    /// Set **SPI_RX_AFIFO_RST** `[29:29]`
    ///
    /// Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer.
    #[inline(always)]
    pub const fn with_spi_rx_afifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_RX_AFIFO_RST_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_RX_AFIFO_RST_MASK
            },
        }
    }

    /// Set **SPI_DMA_TX_ENA** `[28:28]`
    ///
    /// Set this bit to enable SPI DMA controlled send data mode.
    #[inline(always)]
    pub const fn with_spi_dma_tx_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_DMA_TX_ENA_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_DMA_TX_ENA_MASK
            },
        }
    }

    /// Set **SPI_DMA_RX_ENA** `[27:27]`
    ///
    /// Set this bit to enable SPI DMA controlled receive data mode.
    #[inline(always)]
    pub const fn with_spi_dma_rx_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_DMA_RX_ENA_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_DMA_RX_ENA_MASK
            },
        }
    }

    /// Set **SPI_RX_EOF_EN** `[21:21]`
    ///
    /// 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen[19:0] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans.
    #[inline(always)]
    pub const fn with_spi_rx_eof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_RX_EOF_EN_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_RX_EOF_EN_MASK
            },
        }
    }

    /// Set **SPI_SLV_TX_SEG_TRANS_CLR_EN** `[20:20]`
    ///
    /// 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub const fn with_spi_slv_tx_seg_trans_clr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_SLV_TX_SEG_TRANS_CLR_EN_MASK
            },
        }
    }

    /// Set **SPI_SLV_RX_SEG_TRANS_CLR_EN** `[19:19]`
    ///
    /// 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done.
    #[inline(always)]
    pub const fn with_spi_slv_rx_seg_trans_clr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_SLV_RX_SEG_TRANS_CLR_EN_MASK
            },
        }
    }

    /// Set **SPI_DMA_SLV_SEG_TRANS_EN** `[18:18]`
    ///
    /// Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable.
    #[inline(always)]
    pub const fn with_spi_dma_slv_seg_trans_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaConf::SPI_DMA_SLV_SEG_TRANS_EN_MASK
            } else {
                self.bits & !SpiDmaConf::SPI_DMA_SLV_SEG_TRANS_EN_MASK
            },
        }
    }
}

impl Default for SpiDmaConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiDmaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDmaConfVal {
        SpiDmaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDmaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDmaConfVal) -> SpiDmaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DMA_INT_ENA Register
///
/// SPI interrupt enable register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SPI_APP1_INT_ENA** `[20]` (RW)
///
///   The enable bit for SPI_APP1_INT interrupt.
///
/// - **SPI_APP2_INT_ENA** `[19]` (RW)
///
///   The enable bit for SPI_APP2_INT interrupt.
///
/// - **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA** `[18]` (RW)
///
///   The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
///
/// - **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA** `[17]` (RW)
///
///   The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
///
/// - **SPI_SLV_CMD_ERR_INT_ENA** `[16]` (RW)
///
///   The enable bit for SPI_SLV_CMD_ERR_INT interrupt.
///
/// - **SPI_SLV_BUF_ADDR_ERR_INT_ENA** `[15]` (RW)
///
///   The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
///
/// - **SPI_SEG_MAGIC_ERR_INT_ENA** `[14]` (RW)
///
///   The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt.
///
/// - **SPI_DMA_SEG_TRANS_DONE_INT_ENA** `[13]` (RW)
///
///   The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
///
/// - **SPI_TRANS_DONE_INT_ENA** `[12]` (RW)
///
///   The enable bit for SPI_TRANS_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_BUF_DONE_INT_ENA** `[11]` (RW)
///
///   The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_BUF_DONE_INT_ENA** `[10]` (RW)
///
///   The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_DMA_DONE_INT_ENA** `[9]` (RW)
///
///   The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_DMA_DONE_INT_ENA** `[8]` (RW)
///
///   The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_CMDA_INT_ENA** `[7]` (RW)
///
///   The enable bit for SPI slave CMDA interrupt.
///
/// - **SPI_SLV_CMD9_INT_ENA** `[6]` (RW)
///
///   The enable bit for SPI slave CMD9 interrupt.
///
/// - **SPI_SLV_CMD8_INT_ENA** `[5]` (RW)
///
///   The enable bit for SPI slave CMD8 interrupt.
///
/// - **SPI_SLV_CMD7_INT_ENA** `[4]` (RW)
///
///   The enable bit for SPI slave CMD7 interrupt.
///
/// - **SPI_SLV_EN_QPI_INT_ENA** `[3]` (RW)
///
///   The enable bit for SPI slave En_QPI interrupt.
///
/// - **SPI_SLV_EX_QPI_INT_ENA** `[2]` (RW)
///
///   The enable bit for SPI slave Ex_QPI interrupt.
///
/// - **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA** `[1]` (RW)
///
///   The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
///
/// - **SPI_DMA_INFIFO_FULL_ERR_INT_ENA** `[0]` (RW)
///
///   The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
#[repr(transparent)]
pub struct SpiDmaIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaIntEna {
    // ======== Field Constants ========

    /// **SPI_APP1_INT_ENA**
    ///
    /// The enable bit for SPI_APP1_INT interrupt.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_APP1_INT_ENA_OFFSET: u32 = 20;
    pub const SPI_APP1_INT_ENA_MASK: u32 = 0x00100000;
    pub const SPI_APP1_INT_ENA: u32 = 1 << 20;

    /// **SPI_APP2_INT_ENA**
    ///
    /// The enable bit for SPI_APP2_INT interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_APP2_INT_ENA_OFFSET: u32 = 19;
    pub const SPI_APP2_INT_ENA_MASK: u32 = 0x00080000;
    pub const SPI_APP2_INT_ENA: u32 = 1 << 19;

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_OFFSET: u32 = 18;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK: u32 = 0x00040000;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA: u32 = 1 << 18;

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_OFFSET: u32 = 17;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK: u32 = 0x00020000;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA: u32 = 1 << 17;

    /// **SPI_SLV_CMD_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_CMD_ERR_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SLV_CMD_ERR_INT_ENA_OFFSET: u32 = 16;
    pub const SPI_SLV_CMD_ERR_INT_ENA_MASK: u32 = 0x00010000;
    pub const SPI_SLV_CMD_ERR_INT_ENA: u32 = 1 << 16;

    /// **SPI_SLV_BUF_ADDR_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ENA_OFFSET: u32 = 15;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK: u32 = 0x00008000;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ENA: u32 = 1 << 15;

    /// **SPI_SEG_MAGIC_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_SEG_MAGIC_ERR_INT_ENA_OFFSET: u32 = 14;
    pub const SPI_SEG_MAGIC_ERR_INT_ENA_MASK: u32 = 0x00004000;
    pub const SPI_SEG_MAGIC_ERR_INT_ENA: u32 = 1 << 14;

    /// **SPI_DMA_SEG_TRANS_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ENA_OFFSET: u32 = 13;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK: u32 = 0x00002000;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ENA: u32 = 1 << 13;

    /// **SPI_TRANS_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_TRANS_DONE_INT_ENA_OFFSET: u32 = 12;
    pub const SPI_TRANS_DONE_INT_ENA_MASK: u32 = 0x00001000;
    pub const SPI_TRANS_DONE_INT_ENA: u32 = 1 << 12;

    /// **SPI_SLV_WR_BUF_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WR_BUF_DONE_INT_ENA_OFFSET: u32 = 11;
    pub const SPI_SLV_WR_BUF_DONE_INT_ENA_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WR_BUF_DONE_INT_ENA: u32 = 1 << 11;

    /// **SPI_SLV_RD_BUF_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RD_BUF_DONE_INT_ENA_OFFSET: u32 = 10;
    pub const SPI_SLV_RD_BUF_DONE_INT_ENA_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RD_BUF_DONE_INT_ENA: u32 = 1 << 10;

    /// **SPI_SLV_WR_DMA_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WR_DMA_DONE_INT_ENA_OFFSET: u32 = 9;
    pub const SPI_SLV_WR_DMA_DONE_INT_ENA_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WR_DMA_DONE_INT_ENA: u32 = 1 << 9;

    /// **SPI_SLV_RD_DMA_DONE_INT_ENA**
    ///
    /// The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RD_DMA_DONE_INT_ENA_OFFSET: u32 = 8;
    pub const SPI_SLV_RD_DMA_DONE_INT_ENA_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RD_DMA_DONE_INT_ENA: u32 = 1 << 8;

    /// **SPI_SLV_CMDA_INT_ENA**
    ///
    /// The enable bit for SPI slave CMDA interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SLV_CMDA_INT_ENA_OFFSET: u32 = 7;
    pub const SPI_SLV_CMDA_INT_ENA_MASK: u32 = 0x00000080;
    pub const SPI_SLV_CMDA_INT_ENA: u32 = 1 << 7;

    /// **SPI_SLV_CMD9_INT_ENA**
    ///
    /// The enable bit for SPI slave CMD9 interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SLV_CMD9_INT_ENA_OFFSET: u32 = 6;
    pub const SPI_SLV_CMD9_INT_ENA_MASK: u32 = 0x00000040;
    pub const SPI_SLV_CMD9_INT_ENA: u32 = 1 << 6;

    /// **SPI_SLV_CMD8_INT_ENA**
    ///
    /// The enable bit for SPI slave CMD8 interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SLV_CMD8_INT_ENA_OFFSET: u32 = 5;
    pub const SPI_SLV_CMD8_INT_ENA_MASK: u32 = 0x00000020;
    pub const SPI_SLV_CMD8_INT_ENA: u32 = 1 << 5;

    /// **SPI_SLV_CMD7_INT_ENA**
    ///
    /// The enable bit for SPI slave CMD7 interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SLV_CMD7_INT_ENA_OFFSET: u32 = 4;
    pub const SPI_SLV_CMD7_INT_ENA_MASK: u32 = 0x00000010;
    pub const SPI_SLV_CMD7_INT_ENA: u32 = 1 << 4;

    /// **SPI_SLV_EN_QPI_INT_ENA**
    ///
    /// The enable bit for SPI slave En_QPI interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SLV_EN_QPI_INT_ENA_OFFSET: u32 = 3;
    pub const SPI_SLV_EN_QPI_INT_ENA_MASK: u32 = 0x00000008;
    pub const SPI_SLV_EN_QPI_INT_ENA: u32 = 1 << 3;

    /// **SPI_SLV_EX_QPI_INT_ENA**
    ///
    /// The enable bit for SPI slave Ex_QPI interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SLV_EX_QPI_INT_ENA_OFFSET: u32 = 2;
    pub const SPI_SLV_EX_QPI_INT_ENA_MASK: u32 = 0x00000004;
    pub const SPI_SLV_EX_QPI_INT_ENA: u32 = 1 << 2;

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_OFFSET: u32 = 1;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK: u32 = 0x00000002;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA: u32 = 1 << 1;

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ENA_OFFSET: u32 = 0;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK: u32 = 0x00000001;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_APP1_INT_ENA** `[20]` (RW)
    ///
    /// The enable bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub fn spi_app1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP1_INT_ENA_MASK) != 0
    }

    /// Set SPI_APP1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_app1_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SPI_APP1_INT_ENA_MASK);
    }

    /// Clear SPI_APP1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_app1_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_APP1_INT_ENA_MASK);
    }

    /// **SPI_APP2_INT_ENA** `[19]` (RW)
    ///
    /// The enable bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub fn spi_app2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP2_INT_ENA_MASK) != 0
    }

    /// Set SPI_APP2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_app2_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SPI_APP2_INT_ENA_MASK);
    }

    /// Clear SPI_APP2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_app2_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_APP2_INT_ENA_MASK);
    }

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA** `[18]` (RW)
    ///
    /// The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_tx_afifo_rempty_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_tx_afifo_rempty_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK);
    }

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA** `[17]` (RW)
    ///
    /// The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_rx_afifo_wfull_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_rx_afifo_wfull_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK);
    }

    /// **SPI_SLV_CMD_ERR_INT_ENA** `[16]` (RW)
    ///
    /// The enable bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_CMD_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_CMD_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD_ERR_INT_ENA_MASK);
    }

    /// **SPI_SLV_BUF_ADDR_ERR_INT_ENA** `[15]` (RW)
    ///
    /// The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_BUF_ADDR_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_buf_addr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_BUF_ADDR_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_buf_addr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK);
    }

    /// **SPI_SEG_MAGIC_ERR_INT_ENA** `[14]` (RW)
    ///
    /// The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SEG_MAGIC_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_SEG_MAGIC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_seg_magic_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SEG_MAGIC_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_SEG_MAGIC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_seg_magic_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SEG_MAGIC_ERR_INT_ENA_MASK);
    }

    /// **SPI_DMA_SEG_TRANS_DONE_INT_ENA** `[13]` (RW)
    ///
    /// The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_DMA_SEG_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_seg_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_DMA_SEG_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_seg_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK);
    }

    /// **SPI_TRANS_DONE_INT_ENA** `[12]` (RW)
    ///
    /// The enable bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_TRANS_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_TRANS_DONE_INT_ENA_MASK);
    }

    /// **SPI_SLV_WR_BUF_DONE_INT_ENA** `[11]` (RW)
    ///
    /// The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_WR_BUF_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wr_buf_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_WR_BUF_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wr_buf_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK);
    }

    /// **SPI_SLV_RD_BUF_DONE_INT_ENA** `[10]` (RW)
    ///
    /// The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_RD_BUF_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rd_buf_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_RD_BUF_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rd_buf_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK);
    }

    /// **SPI_SLV_WR_DMA_DONE_INT_ENA** `[9]` (RW)
    ///
    /// The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_WR_DMA_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wr_dma_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_WR_DMA_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wr_dma_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK);
    }

    /// **SPI_SLV_RD_DMA_DONE_INT_ENA** `[8]` (RW)
    ///
    /// The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_RD_DMA_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rd_dma_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_RD_DMA_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rd_dma_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK);
    }

    /// **SPI_SLV_CMDA_INT_ENA** `[7]` (RW)
    ///
    /// The enable bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMDA_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_CMDA_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmda_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMDA_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_CMDA_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmda_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMDA_INT_ENA_MASK);
    }

    /// **SPI_SLV_CMD9_INT_ENA** `[6]` (RW)
    ///
    /// The enable bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD9_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_CMD9_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd9_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD9_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_CMD9_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd9_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD9_INT_ENA_MASK);
    }

    /// **SPI_SLV_CMD8_INT_ENA** `[5]` (RW)
    ///
    /// The enable bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD8_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_CMD8_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd8_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD8_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_CMD8_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd8_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD8_INT_ENA_MASK);
    }

    /// **SPI_SLV_CMD7_INT_ENA** `[4]` (RW)
    ///
    /// The enable bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD7_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_CMD7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd7_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD7_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_CMD7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd7_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD7_INT_ENA_MASK);
    }

    /// **SPI_SLV_EN_QPI_INT_ENA** `[3]` (RW)
    ///
    /// The enable bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EN_QPI_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_EN_QPI_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_en_qpi_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_EN_QPI_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_EN_QPI_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_en_qpi_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_EN_QPI_INT_ENA_MASK);
    }

    /// **SPI_SLV_EX_QPI_INT_ENA** `[2]` (RW)
    ///
    /// The enable bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EX_QPI_INT_ENA_MASK) != 0
    }

    /// Set SPI_SLV_EX_QPI_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_ex_qpi_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_EX_QPI_INT_ENA_MASK);
    }

    /// Clear SPI_SLV_EX_QPI_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_ex_qpi_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_EX_QPI_INT_ENA_MASK);
    }

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA** `[1]` (RW)
    ///
    /// The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_outfifo_empty_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_outfifo_empty_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK);
    }

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_ENA** `[0]` (RW)
    ///
    /// The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_DMA_INFIFO_FULL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_infifo_full_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_DMA_INFIFO_FULL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_infifo_full_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK);
    }
}

/// SPI_DMA_INT_ENA value type for batch read/modify/write operations
///
/// Use `spidmaintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaIntEnaVal {
    bits: u32,
}

impl SpiDmaIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_APP1_INT_ENA** `[20:20]`
    ///
    /// The enable bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub const fn spi_app1_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_APP1_INT_ENA_MASK) != 0
    }

    /// Read **SPI_APP2_INT_ENA** `[19:19]`
    ///
    /// The enable bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub const fn spi_app2_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_APP2_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA** `[18:18]`
    ///
    /// The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mst_tx_afifo_rempty_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA** `[17:17]`
    ///
    /// The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mst_rx_afifo_wfull_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_CMD_ERR_INT_ENA** `[16:16]`
    ///
    /// The enable bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_CMD_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_BUF_ADDR_ERR_INT_ENA** `[15:15]`
    ///
    /// The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_buf_addr_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SEG_MAGIC_ERR_INT_ENA** `[14:14]`
    ///
    /// The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_seg_magic_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SEG_MAGIC_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_DMA_SEG_TRANS_DONE_INT_ENA** `[13:13]`
    ///
    /// The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_seg_trans_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_TRANS_DONE_INT_ENA** `[12:12]`
    ///
    /// The enable bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_trans_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_WR_BUF_DONE_INT_ENA** `[11:11]`
    ///
    /// The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_wr_buf_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_RD_BUF_DONE_INT_ENA** `[10:10]`
    ///
    /// The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_rd_buf_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_WR_DMA_DONE_INT_ENA** `[9:9]`
    ///
    /// The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_wr_dma_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_RD_DMA_DONE_INT_ENA** `[8:8]`
    ///
    /// The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_rd_dma_done_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_CMDA_INT_ENA** `[7:7]`
    ///
    /// The enable bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmda_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_CMDA_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_CMD9_INT_ENA** `[6:6]`
    ///
    /// The enable bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd9_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_CMD9_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_CMD8_INT_ENA** `[5:5]`
    ///
    /// The enable bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd8_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_CMD8_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_CMD7_INT_ENA** `[4:4]`
    ///
    /// The enable bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd7_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_CMD7_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_EN_QPI_INT_ENA** `[3:3]`
    ///
    /// The enable bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub const fn spi_slv_en_qpi_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_EN_QPI_INT_ENA_MASK) != 0
    }

    /// Read **SPI_SLV_EX_QPI_INT_ENA** `[2:2]`
    ///
    /// The enable bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub const fn spi_slv_ex_qpi_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_SLV_EX_QPI_INT_ENA_MASK) != 0
    }

    /// Read **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA** `[1:1]`
    ///
    /// The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_outfifo_empty_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_DMA_INFIFO_FULL_ERR_INT_ENA** `[0:0]`
    ///
    /// The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_infifo_full_err_int_ena(&self) -> bool {
        (self.bits & SpiDmaIntEna::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK) != 0
    }

    /// Set **SPI_APP1_INT_ENA** `[20:20]`
    ///
    /// The enable bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_APP1_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_APP1_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_APP2_INT_ENA** `[19:19]`
    ///
    /// The enable bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_APP2_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_APP2_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA** `[18:18]`
    ///
    /// The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_tx_afifo_rempty_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA** `[17:17]`
    ///
    /// The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_rx_afifo_wfull_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD_ERR_INT_ENA** `[16:16]`
    ///
    /// The enable bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_CMD_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_CMD_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_BUF_ADDR_ERR_INT_ENA** `[15:15]`
    ///
    /// The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_buf_addr_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_BUF_ADDR_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SEG_MAGIC_ERR_INT_ENA** `[14:14]`
    ///
    /// The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_seg_magic_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SEG_MAGIC_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SEG_MAGIC_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_DMA_SEG_TRANS_DONE_INT_ENA** `[13:13]`
    ///
    /// The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_seg_trans_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_DMA_SEG_TRANS_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_TRANS_DONE_INT_ENA** `[12:12]`
    ///
    /// The enable bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_trans_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_TRANS_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_TRANS_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_BUF_DONE_INT_ENA** `[11:11]`
    ///
    /// The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_buf_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_WR_BUF_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_BUF_DONE_INT_ENA** `[10:10]`
    ///
    /// The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_buf_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_RD_BUF_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_DMA_DONE_INT_ENA** `[9:9]`
    ///
    /// The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_dma_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_WR_DMA_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_DMA_DONE_INT_ENA** `[8:8]`
    ///
    /// The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_dma_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_RD_DMA_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMDA_INT_ENA** `[7:7]`
    ///
    /// The enable bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmda_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_CMDA_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_CMDA_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD9_INT_ENA** `[6:6]`
    ///
    /// The enable bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd9_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_CMD9_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_CMD9_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD8_INT_ENA** `[5:5]`
    ///
    /// The enable bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd8_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_CMD8_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_CMD8_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD7_INT_ENA** `[4:4]`
    ///
    /// The enable bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd7_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_CMD7_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_CMD7_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_EN_QPI_INT_ENA** `[3:3]`
    ///
    /// The enable bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_en_qpi_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_EN_QPI_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_EN_QPI_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_SLV_EX_QPI_INT_ENA** `[2:2]`
    ///
    /// The enable bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_ex_qpi_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_SLV_EX_QPI_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_SLV_EX_QPI_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA** `[1:1]`
    ///
    /// The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_outfifo_empty_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_DMA_INFIFO_FULL_ERR_INT_ENA** `[0:0]`
    ///
    /// The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_infifo_full_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntEna::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiDmaIntEna::SPI_DMA_INFIFO_FULL_ERR_INT_ENA_MASK
            },
        }
    }
}

impl Default for SpiDmaIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDmaIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDmaIntEnaVal {
        SpiDmaIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDmaIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDmaIntEnaVal) -> SpiDmaIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DMA_INT_CLR Register
///
/// SPI interrupt clear register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **SPI_APP1_INT_CLR** `[20]` (WO)
///
///   The clear bit for SPI_APP1_INT interrupt.
///
/// - **SPI_APP2_INT_CLR** `[19]` (WO)
///
///   The clear bit for SPI_APP2_INT interrupt.
///
/// - **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR** `[18]` (WO)
///
///   The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
///
/// - **SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR** `[17]` (WO)
///
///   The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
///
/// - **SPI_SLV_CMD_ERR_INT_CLR** `[16]` (WO)
///
///   The clear bit for SPI_SLV_CMD_ERR_INT interrupt.
///
/// - **SPI_SLV_BUF_ADDR_ERR_INT_CLR** `[15]` (WO)
///
///   The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
///
/// - **SPI_SEG_MAGIC_ERR_INT_CLR** `[14]` (WO)
///
///   The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt.
///
/// - **SPI_DMA_SEG_TRANS_DONE_INT_CLR** `[13]` (WO)
///
///   The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
///
/// - **SPI_TRANS_DONE_INT_CLR** `[12]` (WO)
///
///   The clear bit for SPI_TRANS_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_BUF_DONE_INT_CLR** `[11]` (WO)
///
///   The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_BUF_DONE_INT_CLR** `[10]` (WO)
///
///   The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_DMA_DONE_INT_CLR** `[9]` (WO)
///
///   The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_DMA_DONE_INT_CLR** `[8]` (WO)
///
///   The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_CMDA_INT_CLR** `[7]` (WO)
///
///   The clear bit for SPI slave CMDA interrupt.
///
/// - **SPI_SLV_CMD9_INT_CLR** `[6]` (WO)
///
///   The clear bit for SPI slave CMD9 interrupt.
///
/// - **SPI_SLV_CMD8_INT_CLR** `[5]` (WO)
///
///   The clear bit for SPI slave CMD8 interrupt.
///
/// - **SPI_SLV_CMD7_INT_CLR** `[4]` (WO)
///
///   The clear bit for SPI slave CMD7 interrupt.
///
/// - **SPI_SLV_EN_QPI_INT_CLR** `[3]` (WO)
///
///   The clear bit for SPI slave En_QPI interrupt.
///
/// - **SPI_SLV_EX_QPI_INT_CLR** `[2]` (WO)
///
///   The clear bit for SPI slave Ex_QPI interrupt.
///
/// - **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR** `[1]` (WO)
///
///   The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
///
/// - **SPI_DMA_INFIFO_FULL_ERR_INT_CLR** `[0]` (WO)
///
///   The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
#[repr(transparent)]
pub struct SpiDmaIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaIntClr {
    // ======== Field Constants ========

    /// **SPI_APP1_INT_CLR**
    ///
    /// The clear bit for SPI_APP1_INT interrupt.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_APP1_INT_CLR_OFFSET: u32 = 20;
    pub const SPI_APP1_INT_CLR_MASK: u32 = 0x00100000;
    pub const SPI_APP1_INT_CLR: u32 = 1 << 20;

    /// **SPI_APP2_INT_CLR**
    ///
    /// The clear bit for SPI_APP2_INT interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_APP2_INT_CLR_OFFSET: u32 = 19;
    pub const SPI_APP2_INT_CLR_MASK: u32 = 0x00080000;
    pub const SPI_APP2_INT_CLR: u32 = 1 << 19;

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_OFFSET: u32 = 18;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_MASK: u32 = 0x00040000;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR: u32 = 1 << 18;

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_OFFSET: u32 = 17;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_MASK: u32 = 0x00020000;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR: u32 = 1 << 17;

    /// **SPI_SLV_CMD_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_CMD_ERR_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SLV_CMD_ERR_INT_CLR_OFFSET: u32 = 16;
    pub const SPI_SLV_CMD_ERR_INT_CLR_MASK: u32 = 0x00010000;
    pub const SPI_SLV_CMD_ERR_INT_CLR: u32 = 1 << 16;

    /// **SPI_SLV_BUF_ADDR_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SLV_BUF_ADDR_ERR_INT_CLR_OFFSET: u32 = 15;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_CLR_MASK: u32 = 0x00008000;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_CLR: u32 = 1 << 15;

    /// **SPI_SEG_MAGIC_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_SEG_MAGIC_ERR_INT_CLR_OFFSET: u32 = 14;
    pub const SPI_SEG_MAGIC_ERR_INT_CLR_MASK: u32 = 0x00004000;
    pub const SPI_SEG_MAGIC_ERR_INT_CLR: u32 = 1 << 14;

    /// **SPI_DMA_SEG_TRANS_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DMA_SEG_TRANS_DONE_INT_CLR_OFFSET: u32 = 13;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_CLR_MASK: u32 = 0x00002000;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_CLR: u32 = 1 << 13;

    /// **SPI_TRANS_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_TRANS_DONE_INT_CLR_OFFSET: u32 = 12;
    pub const SPI_TRANS_DONE_INT_CLR_MASK: u32 = 0x00001000;
    pub const SPI_TRANS_DONE_INT_CLR: u32 = 1 << 12;

    /// **SPI_SLV_WR_BUF_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WR_BUF_DONE_INT_CLR_OFFSET: u32 = 11;
    pub const SPI_SLV_WR_BUF_DONE_INT_CLR_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WR_BUF_DONE_INT_CLR: u32 = 1 << 11;

    /// **SPI_SLV_RD_BUF_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RD_BUF_DONE_INT_CLR_OFFSET: u32 = 10;
    pub const SPI_SLV_RD_BUF_DONE_INT_CLR_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RD_BUF_DONE_INT_CLR: u32 = 1 << 10;

    /// **SPI_SLV_WR_DMA_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WR_DMA_DONE_INT_CLR_OFFSET: u32 = 9;
    pub const SPI_SLV_WR_DMA_DONE_INT_CLR_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WR_DMA_DONE_INT_CLR: u32 = 1 << 9;

    /// **SPI_SLV_RD_DMA_DONE_INT_CLR**
    ///
    /// The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RD_DMA_DONE_INT_CLR_OFFSET: u32 = 8;
    pub const SPI_SLV_RD_DMA_DONE_INT_CLR_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RD_DMA_DONE_INT_CLR: u32 = 1 << 8;

    /// **SPI_SLV_CMDA_INT_CLR**
    ///
    /// The clear bit for SPI slave CMDA interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SLV_CMDA_INT_CLR_OFFSET: u32 = 7;
    pub const SPI_SLV_CMDA_INT_CLR_MASK: u32 = 0x00000080;
    pub const SPI_SLV_CMDA_INT_CLR: u32 = 1 << 7;

    /// **SPI_SLV_CMD9_INT_CLR**
    ///
    /// The clear bit for SPI slave CMD9 interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SLV_CMD9_INT_CLR_OFFSET: u32 = 6;
    pub const SPI_SLV_CMD9_INT_CLR_MASK: u32 = 0x00000040;
    pub const SPI_SLV_CMD9_INT_CLR: u32 = 1 << 6;

    /// **SPI_SLV_CMD8_INT_CLR**
    ///
    /// The clear bit for SPI slave CMD8 interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SLV_CMD8_INT_CLR_OFFSET: u32 = 5;
    pub const SPI_SLV_CMD8_INT_CLR_MASK: u32 = 0x00000020;
    pub const SPI_SLV_CMD8_INT_CLR: u32 = 1 << 5;

    /// **SPI_SLV_CMD7_INT_CLR**
    ///
    /// The clear bit for SPI slave CMD7 interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SLV_CMD7_INT_CLR_OFFSET: u32 = 4;
    pub const SPI_SLV_CMD7_INT_CLR_MASK: u32 = 0x00000010;
    pub const SPI_SLV_CMD7_INT_CLR: u32 = 1 << 4;

    /// **SPI_SLV_EN_QPI_INT_CLR**
    ///
    /// The clear bit for SPI slave En_QPI interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SLV_EN_QPI_INT_CLR_OFFSET: u32 = 3;
    pub const SPI_SLV_EN_QPI_INT_CLR_MASK: u32 = 0x00000008;
    pub const SPI_SLV_EN_QPI_INT_CLR: u32 = 1 << 3;

    /// **SPI_SLV_EX_QPI_INT_CLR**
    ///
    /// The clear bit for SPI slave Ex_QPI interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SLV_EX_QPI_INT_CLR_OFFSET: u32 = 2;
    pub const SPI_SLV_EX_QPI_INT_CLR_MASK: u32 = 0x00000004;
    pub const SPI_SLV_EX_QPI_INT_CLR: u32 = 1 << 2;

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_OFFSET: u32 = 1;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_MASK: u32 = 0x00000002;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR: u32 = 1 << 1;

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_CLR_OFFSET: u32 = 0;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_CLR_MASK: u32 = 0x00000001;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_APP1_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_app1_int_clr(&self) {
        self.value.write_bits(Self::SPI_APP1_INT_CLR_MASK);
    }

    /// Trigger **SPI_APP2_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_app2_int_clr(&self) {
        self.value.write_bits(Self::SPI_APP2_INT_CLR_MASK);
    }

    /// Trigger **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mst_tx_afifo_rempty_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mst_rx_afifo_wfull_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_CMD_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd_err_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_BUF_ADDR_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_buf_addr_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_SLV_BUF_ADDR_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_SEG_MAGIC_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_seg_magic_err_int_clr(&self) {
        self.value.write_bits(Self::SPI_SEG_MAGIC_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_DMA_SEG_TRANS_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_seg_trans_done_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_DMA_SEG_TRANS_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_TRANS_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_trans_done_int_clr(&self) {
        self.value.write_bits(Self::SPI_TRANS_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_WR_BUF_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_wr_buf_done_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_SLV_WR_BUF_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_RD_BUF_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_rd_buf_done_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_SLV_RD_BUF_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_WR_DMA_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_wr_dma_done_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_SLV_WR_DMA_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_RD_DMA_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_rd_dma_done_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_SLV_RD_DMA_DONE_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_CMDA_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmda_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_CMDA_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_CMD9_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd9_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD9_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_CMD8_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd8_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD8_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_CMD7_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd7_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD7_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_EN_QPI_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_en_qpi_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_EN_QPI_INT_CLR_MASK);
    }

    /// Trigger **SPI_SLV_EX_QPI_INT_CLR** (WO)
    ///
    /// The clear bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_ex_qpi_int_clr(&self) {
        self.value.write_bits(Self::SPI_SLV_EX_QPI_INT_CLR_MASK);
    }

    /// Trigger **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_outfifo_empty_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_DMA_INFIFO_FULL_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_infifo_full_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_DMA_INFIFO_FULL_ERR_INT_CLR_MASK);
    }
}

/// SPI_DMA_INT_CLR value type for batch read/modify/write operations
///
/// Use `spidmaintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaIntClrVal {
    bits: u32,
}

impl SpiDmaIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_APP1_INT_CLR** `[20:20]`
    ///
    /// The clear bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_APP1_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_APP1_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_APP2_INT_CLR** `[19:19]`
    ///
    /// The clear bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_APP2_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_APP2_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR** `[18:18]`
    ///
    /// The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_tx_afifo_rempty_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR** `[17:17]`
    ///
    /// The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_rx_afifo_wfull_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD_ERR_INT_CLR** `[16:16]`
    ///
    /// The clear bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_CMD_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_CMD_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_BUF_ADDR_ERR_INT_CLR** `[15:15]`
    ///
    /// The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_buf_addr_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_BUF_ADDR_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_BUF_ADDR_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SEG_MAGIC_ERR_INT_CLR** `[14:14]`
    ///
    /// The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_seg_magic_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SEG_MAGIC_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SEG_MAGIC_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_DMA_SEG_TRANS_DONE_INT_CLR** `[13:13]`
    ///
    /// The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_seg_trans_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_DMA_SEG_TRANS_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_DMA_SEG_TRANS_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_TRANS_DONE_INT_CLR** `[12:12]`
    ///
    /// The clear bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_trans_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_TRANS_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_TRANS_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_BUF_DONE_INT_CLR** `[11:11]`
    ///
    /// The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_buf_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_WR_BUF_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_WR_BUF_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_BUF_DONE_INT_CLR** `[10:10]`
    ///
    /// The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_buf_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_RD_BUF_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_RD_BUF_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_DMA_DONE_INT_CLR** `[9:9]`
    ///
    /// The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_dma_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_WR_DMA_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_WR_DMA_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_DMA_DONE_INT_CLR** `[8:8]`
    ///
    /// The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_dma_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_RD_DMA_DONE_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_RD_DMA_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMDA_INT_CLR** `[7:7]`
    ///
    /// The clear bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmda_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_CMDA_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_CMDA_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD9_INT_CLR** `[6:6]`
    ///
    /// The clear bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd9_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_CMD9_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_CMD9_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD8_INT_CLR** `[5:5]`
    ///
    /// The clear bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd8_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_CMD8_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_CMD8_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD7_INT_CLR** `[4:4]`
    ///
    /// The clear bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd7_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_CMD7_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_CMD7_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_EN_QPI_INT_CLR** `[3:3]`
    ///
    /// The clear bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_en_qpi_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_EN_QPI_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_EN_QPI_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_SLV_EX_QPI_INT_CLR** `[2:2]`
    ///
    /// The clear bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_ex_qpi_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_SLV_EX_QPI_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_SLV_EX_QPI_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR** `[1:1]`
    ///
    /// The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_outfifo_empty_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_DMA_INFIFO_FULL_ERR_INT_CLR** `[0:0]`
    ///
    /// The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_infifo_full_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntClr::SPI_DMA_INFIFO_FULL_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiDmaIntClr::SPI_DMA_INFIFO_FULL_ERR_INT_CLR_MASK
            },
        }
    }
}

impl Default for SpiDmaIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDmaIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDmaIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_DMA_INT_RAW Register
///
/// SPI interrupt raw register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **SPI_APP1_INT_RAW** `[20]` (RW)
///
///   The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software.
///
/// - **SPI_APP2_INT_RAW** `[19]` (RW)
///
///   The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software.
///
/// - **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW** `[18]` (RW)
///
///   The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others.
///
/// - **SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW** `[17]` (RW)
///
///   The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others.
///
/// - **SPI_SLV_CMD_ERR_INT_RAW** `[16]` (RW)
///
///   The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others.
///
/// - **SPI_SLV_BUF_ADDR_ERR_INT_RAW** `[15]` (RW)
///
///   The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others.
///
/// - **SPI_SEG_MAGIC_ERR_INT_RAW** `[14]` (RW)
///
///   The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others.
///
/// - **SPI_DMA_SEG_TRANS_DONE_INT_RAW** `[13]` (RW)
///
///   The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred.
///
/// - **SPI_TRANS_DONE_INT_RAW** `[12]` (RW)
///
///   The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others.
///
/// - **SPI_SLV_WR_BUF_DONE_INT_RAW** `[11]` (RW)
///
///   The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others.
///
/// - **SPI_SLV_RD_BUF_DONE_INT_RAW** `[10]` (RW)
///
///   The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others.
///
/// - **SPI_SLV_WR_DMA_DONE_INT_RAW** `[9]` (RW)
///
///   The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others.
///
/// - **SPI_SLV_RD_DMA_DONE_INT_RAW** `[8]` (RW)
///
///   The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others.
///
/// - **SPI_SLV_CMDA_INT_RAW** `[7]` (RW)
///
///   The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others.
///
/// - **SPI_SLV_CMD9_INT_RAW** `[6]` (RW)
///
///   The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others.
///
/// - **SPI_SLV_CMD8_INT_RAW** `[5]` (RW)
///
///   The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others.
///
/// - **SPI_SLV_CMD7_INT_RAW** `[4]` (RW)
///
///   The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others.
///
/// - **SPI_SLV_EN_QPI_INT_RAW** `[3]` (RW)
///
///   The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others.
///
/// - **SPI_SLV_EX_QPI_INT_RAW** `[2]` (RW)
///
///   The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others.
///
/// - **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW** `[1]` (RW)
///
///   1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others.
///
/// - **SPI_DMA_INFIFO_FULL_ERR_INT_RAW** `[0]` (RW)
///
///   1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others.
#[repr(transparent)]
pub struct SpiDmaIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaIntRaw {
    // ======== Field Constants ========

    /// **SPI_APP1_INT_RAW**
    ///
    /// The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_APP1_INT_RAW_OFFSET: u32 = 20;
    pub const SPI_APP1_INT_RAW_MASK: u32 = 0x00100000;
    pub const SPI_APP1_INT_RAW: u32 = 1 << 20;

    /// **SPI_APP2_INT_RAW**
    ///
    /// The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_APP2_INT_RAW_OFFSET: u32 = 19;
    pub const SPI_APP2_INT_RAW_MASK: u32 = 0x00080000;
    pub const SPI_APP2_INT_RAW: u32 = 1 << 19;

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_OFFSET: u32 = 18;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK: u32 = 0x00040000;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW: u32 = 1 << 18;

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_OFFSET: u32 = 17;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK: u32 = 0x00020000;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW: u32 = 1 << 17;

    /// **SPI_SLV_CMD_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SLV_CMD_ERR_INT_RAW_OFFSET: u32 = 16;
    pub const SPI_SLV_CMD_ERR_INT_RAW_MASK: u32 = 0x00010000;
    pub const SPI_SLV_CMD_ERR_INT_RAW: u32 = 1 << 16;

    /// **SPI_SLV_BUF_ADDR_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SLV_BUF_ADDR_ERR_INT_RAW_OFFSET: u32 = 15;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK: u32 = 0x00008000;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_RAW: u32 = 1 << 15;

    /// **SPI_SEG_MAGIC_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_SEG_MAGIC_ERR_INT_RAW_OFFSET: u32 = 14;
    pub const SPI_SEG_MAGIC_ERR_INT_RAW_MASK: u32 = 0x00004000;
    pub const SPI_SEG_MAGIC_ERR_INT_RAW: u32 = 1 << 14;

    /// **SPI_DMA_SEG_TRANS_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DMA_SEG_TRANS_DONE_INT_RAW_OFFSET: u32 = 13;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK: u32 = 0x00002000;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_RAW: u32 = 1 << 13;

    /// **SPI_TRANS_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_TRANS_DONE_INT_RAW_OFFSET: u32 = 12;
    pub const SPI_TRANS_DONE_INT_RAW_MASK: u32 = 0x00001000;
    pub const SPI_TRANS_DONE_INT_RAW: u32 = 1 << 12;

    /// **SPI_SLV_WR_BUF_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WR_BUF_DONE_INT_RAW_OFFSET: u32 = 11;
    pub const SPI_SLV_WR_BUF_DONE_INT_RAW_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WR_BUF_DONE_INT_RAW: u32 = 1 << 11;

    /// **SPI_SLV_RD_BUF_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RD_BUF_DONE_INT_RAW_OFFSET: u32 = 10;
    pub const SPI_SLV_RD_BUF_DONE_INT_RAW_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RD_BUF_DONE_INT_RAW: u32 = 1 << 10;

    /// **SPI_SLV_WR_DMA_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WR_DMA_DONE_INT_RAW_OFFSET: u32 = 9;
    pub const SPI_SLV_WR_DMA_DONE_INT_RAW_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WR_DMA_DONE_INT_RAW: u32 = 1 << 9;

    /// **SPI_SLV_RD_DMA_DONE_INT_RAW**
    ///
    /// The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RD_DMA_DONE_INT_RAW_OFFSET: u32 = 8;
    pub const SPI_SLV_RD_DMA_DONE_INT_RAW_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RD_DMA_DONE_INT_RAW: u32 = 1 << 8;

    /// **SPI_SLV_CMDA_INT_RAW**
    ///
    /// The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SLV_CMDA_INT_RAW_OFFSET: u32 = 7;
    pub const SPI_SLV_CMDA_INT_RAW_MASK: u32 = 0x00000080;
    pub const SPI_SLV_CMDA_INT_RAW: u32 = 1 << 7;

    /// **SPI_SLV_CMD9_INT_RAW**
    ///
    /// The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SLV_CMD9_INT_RAW_OFFSET: u32 = 6;
    pub const SPI_SLV_CMD9_INT_RAW_MASK: u32 = 0x00000040;
    pub const SPI_SLV_CMD9_INT_RAW: u32 = 1 << 6;

    /// **SPI_SLV_CMD8_INT_RAW**
    ///
    /// The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SLV_CMD8_INT_RAW_OFFSET: u32 = 5;
    pub const SPI_SLV_CMD8_INT_RAW_MASK: u32 = 0x00000020;
    pub const SPI_SLV_CMD8_INT_RAW: u32 = 1 << 5;

    /// **SPI_SLV_CMD7_INT_RAW**
    ///
    /// The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SLV_CMD7_INT_RAW_OFFSET: u32 = 4;
    pub const SPI_SLV_CMD7_INT_RAW_MASK: u32 = 0x00000010;
    pub const SPI_SLV_CMD7_INT_RAW: u32 = 1 << 4;

    /// **SPI_SLV_EN_QPI_INT_RAW**
    ///
    /// The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SLV_EN_QPI_INT_RAW_OFFSET: u32 = 3;
    pub const SPI_SLV_EN_QPI_INT_RAW_MASK: u32 = 0x00000008;
    pub const SPI_SLV_EN_QPI_INT_RAW: u32 = 1 << 3;

    /// **SPI_SLV_EX_QPI_INT_RAW**
    ///
    /// The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SLV_EX_QPI_INT_RAW_OFFSET: u32 = 2;
    pub const SPI_SLV_EX_QPI_INT_RAW_MASK: u32 = 0x00000004;
    pub const SPI_SLV_EX_QPI_INT_RAW: u32 = 1 << 2;

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW**
    ///
    /// 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_OFFSET: u32 = 1;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK: u32 = 0x00000002;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW: u32 = 1 << 1;

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_RAW**
    ///
    /// 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_RAW_OFFSET: u32 = 0;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK: u32 = 0x00000001;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_APP1_INT_RAW** `[20]` (RW)
    ///
    /// The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub fn spi_app1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP1_INT_RAW_MASK) != 0
    }

    /// Set SPI_APP1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_app1_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SPI_APP1_INT_RAW_MASK);
    }

    /// Clear SPI_APP1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_app1_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_APP1_INT_RAW_MASK);
    }

    /// **SPI_APP2_INT_RAW** `[19]` (RW)
    ///
    /// The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub fn spi_app2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP2_INT_RAW_MASK) != 0
    }

    /// Set SPI_APP2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_app2_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SPI_APP2_INT_RAW_MASK);
    }

    /// Clear SPI_APP2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_app2_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_APP2_INT_RAW_MASK);
    }

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW** `[18]` (RW)
    ///
    /// The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others.
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_tx_afifo_rempty_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_tx_afifo_rempty_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK);
    }

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW** `[17]` (RW)
    ///
    /// The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others.
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_rx_afifo_wfull_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_rx_afifo_wfull_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK);
    }

    /// **SPI_SLV_CMD_ERR_INT_RAW** `[16]` (RW)
    ///
    /// The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others.
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_CMD_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_CMD_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD_ERR_INT_RAW_MASK);
    }

    /// **SPI_SLV_BUF_ADDR_ERR_INT_RAW** `[15]` (RW)
    ///
    /// The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others.
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_BUF_ADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_buf_addr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_BUF_ADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_buf_addr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK);
    }

    /// **SPI_SEG_MAGIC_ERR_INT_RAW** `[14]` (RW)
    ///
    /// The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others.
    #[inline(always)]
    pub fn spi_seg_magic_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SEG_MAGIC_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_SEG_MAGIC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_seg_magic_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SEG_MAGIC_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_SEG_MAGIC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_seg_magic_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SEG_MAGIC_ERR_INT_RAW_MASK);
    }

    /// **SPI_DMA_SEG_TRANS_DONE_INT_RAW** `[13]` (RW)
    ///
    /// The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred.
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_DMA_SEG_TRANS_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_seg_trans_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_DMA_SEG_TRANS_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_seg_trans_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK);
    }

    /// **SPI_TRANS_DONE_INT_RAW** `[12]` (RW)
    ///
    /// The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others.
    #[inline(always)]
    pub fn spi_trans_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_TRANS_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_trans_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_TRANS_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_TRANS_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_trans_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_TRANS_DONE_INT_RAW_MASK);
    }

    /// **SPI_SLV_WR_BUF_DONE_INT_RAW** `[11]` (RW)
    ///
    /// The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_WR_BUF_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wr_buf_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_WR_BUF_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wr_buf_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK);
    }

    /// **SPI_SLV_RD_BUF_DONE_INT_RAW** `[10]` (RW)
    ///
    /// The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_RD_BUF_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rd_buf_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_RD_BUF_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rd_buf_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK);
    }

    /// **SPI_SLV_WR_DMA_DONE_INT_RAW** `[9]` (RW)
    ///
    /// The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_WR_DMA_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wr_dma_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_WR_DMA_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wr_dma_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK);
    }

    /// **SPI_SLV_RD_DMA_DONE_INT_RAW** `[8]` (RW)
    ///
    /// The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_RD_DMA_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rd_dma_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_RD_DMA_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rd_dma_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK);
    }

    /// **SPI_SLV_CMDA_INT_RAW** `[7]` (RW)
    ///
    /// The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_cmda_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMDA_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_CMDA_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmda_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMDA_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_CMDA_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmda_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMDA_INT_RAW_MASK);
    }

    /// **SPI_SLV_CMD9_INT_RAW** `[6]` (RW)
    ///
    /// The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_cmd9_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD9_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_CMD9_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd9_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD9_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_CMD9_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd9_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD9_INT_RAW_MASK);
    }

    /// **SPI_SLV_CMD8_INT_RAW** `[5]` (RW)
    ///
    /// The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_cmd8_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD8_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_CMD8_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd8_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD8_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_CMD8_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd8_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD8_INT_RAW_MASK);
    }

    /// **SPI_SLV_CMD7_INT_RAW** `[4]` (RW)
    ///
    /// The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_cmd7_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD7_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_CMD7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_cmd7_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_CMD7_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_CMD7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_cmd7_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_CMD7_INT_RAW_MASK);
    }

    /// **SPI_SLV_EN_QPI_INT_RAW** `[3]` (RW)
    ///
    /// The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EN_QPI_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_EN_QPI_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_en_qpi_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_EN_QPI_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_EN_QPI_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_en_qpi_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_EN_QPI_INT_RAW_MASK);
    }

    /// **SPI_SLV_EX_QPI_INT_RAW** `[2]` (RW)
    ///
    /// The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EX_QPI_INT_RAW_MASK) != 0
    }

    /// Set SPI_SLV_EX_QPI_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_ex_qpi_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_EX_QPI_INT_RAW_MASK);
    }

    /// Clear SPI_SLV_EX_QPI_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_ex_qpi_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_EX_QPI_INT_RAW_MASK);
    }

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW** `[1]` (RW)
    ///
    /// 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others.
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_outfifo_empty_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_outfifo_empty_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK);
    }

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_RAW** `[0]` (RW)
    ///
    /// 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others.
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_DMA_INFIFO_FULL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_dma_infifo_full_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_DMA_INFIFO_FULL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_dma_infifo_full_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK);
    }
}

/// SPI_DMA_INT_RAW value type for batch read/modify/write operations
///
/// Use `spidmaintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaIntRawVal {
    bits: u32,
}

impl SpiDmaIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_APP1_INT_RAW** `[20:20]`
    ///
    /// The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub const fn spi_app1_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_APP1_INT_RAW_MASK) != 0
    }

    /// Read **SPI_APP2_INT_RAW** `[19:19]`
    ///
    /// The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub const fn spi_app2_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_APP2_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW** `[18:18]`
    ///
    /// The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others.
    #[inline(always)]
    pub const fn spi_mst_tx_afifo_rempty_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW** `[17:17]`
    ///
    /// The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others.
    #[inline(always)]
    pub const fn spi_mst_rx_afifo_wfull_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_CMD_ERR_INT_RAW** `[16:16]`
    ///
    /// The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_cmd_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_CMD_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_BUF_ADDR_ERR_INT_RAW** `[15:15]`
    ///
    /// The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_buf_addr_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SEG_MAGIC_ERR_INT_RAW** `[14:14]`
    ///
    /// The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others.
    #[inline(always)]
    pub const fn spi_seg_magic_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SEG_MAGIC_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_DMA_SEG_TRANS_DONE_INT_RAW** `[13:13]`
    ///
    /// The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred.
    #[inline(always)]
    pub const fn spi_dma_seg_trans_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_TRANS_DONE_INT_RAW** `[12:12]`
    ///
    /// The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others.
    #[inline(always)]
    pub const fn spi_trans_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_WR_BUF_DONE_INT_RAW** `[11:11]`
    ///
    /// The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_wr_buf_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_RD_BUF_DONE_INT_RAW** `[10:10]`
    ///
    /// The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_rd_buf_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_WR_DMA_DONE_INT_RAW** `[9:9]`
    ///
    /// The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_wr_dma_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_RD_DMA_DONE_INT_RAW** `[8:8]`
    ///
    /// The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_rd_dma_done_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_CMDA_INT_RAW** `[7:7]`
    ///
    /// The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_cmda_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_CMDA_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_CMD9_INT_RAW** `[6:6]`
    ///
    /// The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_cmd9_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_CMD9_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_CMD8_INT_RAW** `[5:5]`
    ///
    /// The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_cmd8_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_CMD8_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_CMD7_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_cmd7_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_CMD7_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_EN_QPI_INT_RAW** `[3:3]`
    ///
    /// The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_en_qpi_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_EN_QPI_INT_RAW_MASK) != 0
    }

    /// Read **SPI_SLV_EX_QPI_INT_RAW** `[2:2]`
    ///
    /// The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn spi_slv_ex_qpi_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_SLV_EX_QPI_INT_RAW_MASK) != 0
    }

    /// Read **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW** `[1:1]`
    ///
    /// 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others.
    #[inline(always)]
    pub const fn spi_dma_outfifo_empty_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_DMA_INFIFO_FULL_ERR_INT_RAW** `[0:0]`
    ///
    /// 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others.
    #[inline(always)]
    pub const fn spi_dma_infifo_full_err_int_raw(&self) -> bool {
        (self.bits & SpiDmaIntRaw::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK) != 0
    }

    /// Set **SPI_APP1_INT_RAW** `[20:20]`
    ///
    /// The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub const fn with_spi_app1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_APP1_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_APP1_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_APP2_INT_RAW** `[19:19]`
    ///
    /// The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software.
    #[inline(always)]
    pub const fn with_spi_app2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_APP2_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_APP2_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW** `[18:18]`
    ///
    /// The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mst_tx_afifo_rempty_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW** `[17:17]`
    ///
    /// The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mst_rx_afifo_wfull_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD_ERR_INT_RAW** `[16:16]`
    ///
    /// The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_cmd_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_CMD_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_CMD_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_BUF_ADDR_ERR_INT_RAW** `[15:15]`
    ///
    /// The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_buf_addr_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_BUF_ADDR_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SEG_MAGIC_ERR_INT_RAW** `[14:14]`
    ///
    /// The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others.
    #[inline(always)]
    pub const fn with_spi_seg_magic_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SEG_MAGIC_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SEG_MAGIC_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_DMA_SEG_TRANS_DONE_INT_RAW** `[13:13]`
    ///
    /// The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred.
    #[inline(always)]
    pub const fn with_spi_dma_seg_trans_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_DMA_SEG_TRANS_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_TRANS_DONE_INT_RAW** `[12:12]`
    ///
    /// The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others.
    #[inline(always)]
    pub const fn with_spi_trans_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_TRANS_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_TRANS_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_BUF_DONE_INT_RAW** `[11:11]`
    ///
    /// The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_wr_buf_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_WR_BUF_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_BUF_DONE_INT_RAW** `[10:10]`
    ///
    /// The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_rd_buf_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_RD_BUF_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_DMA_DONE_INT_RAW** `[9:9]`
    ///
    /// The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_wr_dma_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_WR_DMA_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_DMA_DONE_INT_RAW** `[8:8]`
    ///
    /// The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_rd_dma_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_RD_DMA_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMDA_INT_RAW** `[7:7]`
    ///
    /// The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_cmda_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_CMDA_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_CMDA_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD9_INT_RAW** `[6:6]`
    ///
    /// The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_cmd9_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_CMD9_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_CMD9_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD8_INT_RAW** `[5:5]`
    ///
    /// The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_cmd8_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_CMD8_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_CMD8_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD7_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_cmd7_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_CMD7_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_CMD7_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_EN_QPI_INT_RAW** `[3:3]`
    ///
    /// The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_en_qpi_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_EN_QPI_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_EN_QPI_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_SLV_EX_QPI_INT_RAW** `[2:2]`
    ///
    /// The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others.
    #[inline(always)]
    pub const fn with_spi_slv_ex_qpi_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_SLV_EX_QPI_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_SLV_EX_QPI_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW** `[1:1]`
    ///
    /// 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others.
    #[inline(always)]
    pub const fn with_spi_dma_outfifo_empty_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_DMA_INFIFO_FULL_ERR_INT_RAW** `[0:0]`
    ///
    /// 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others.
    #[inline(always)]
    pub const fn with_spi_dma_infifo_full_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntRaw::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiDmaIntRaw::SPI_DMA_INFIFO_FULL_ERR_INT_RAW_MASK
            },
        }
    }
}

impl Default for SpiDmaIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDmaIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDmaIntRawVal {
        SpiDmaIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDmaIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDmaIntRawVal) -> SpiDmaIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DMA_INT_ST Register
///
/// SPI interrupt status register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **SPI_APP1_INT_ST** `[20]` (RO)
///
///   The status bit for SPI_APP1_INT interrupt.
///
/// - **SPI_APP2_INT_ST** `[19]` (RO)
///
///   The status bit for SPI_APP2_INT interrupt.
///
/// - **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST** `[18]` (RO)
///
///   The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
///
/// - **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST** `[17]` (RO)
///
///   The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
///
/// - **SPI_SLV_CMD_ERR_INT_ST** `[16]` (RO)
///
///   The status bit for SPI_SLV_CMD_ERR_INT interrupt.
///
/// - **SPI_SLV_BUF_ADDR_ERR_INT_ST** `[15]` (RO)
///
///   The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
///
/// - **SPI_SEG_MAGIC_ERR_INT_ST** `[14]` (RO)
///
///   The status bit for SPI_SEG_MAGIC_ERR_INT interrupt.
///
/// - **SPI_DMA_SEG_TRANS_DONE_INT_ST** `[13]` (RO)
///
///   The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
///
/// - **SPI_TRANS_DONE_INT_ST** `[12]` (RO)
///
///   The status bit for SPI_TRANS_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_BUF_DONE_INT_ST** `[11]` (RO)
///
///   The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_BUF_DONE_INT_ST** `[10]` (RO)
///
///   The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_DMA_DONE_INT_ST** `[9]` (RO)
///
///   The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_DMA_DONE_INT_ST** `[8]` (RO)
///
///   The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_CMDA_INT_ST** `[7]` (RO)
///
///   The status bit for SPI slave CMDA interrupt.
///
/// - **SPI_SLV_CMD9_INT_ST** `[6]` (RO)
///
///   The status bit for SPI slave CMD9 interrupt.
///
/// - **SPI_SLV_CMD8_INT_ST** `[5]` (RO)
///
///   The status bit for SPI slave CMD8 interrupt.
///
/// - **SPI_SLV_CMD7_INT_ST** `[4]` (RO)
///
///   The status bit for SPI slave CMD7 interrupt.
///
/// - **SPI_SLV_EN_QPI_INT_ST** `[3]` (RO)
///
///   The status bit for SPI slave En_QPI interrupt.
///
/// - **SPI_SLV_EX_QPI_INT_ST** `[2]` (RO)
///
///   The status bit for SPI slave Ex_QPI interrupt.
///
/// - **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST** `[1]` (RO)
///
///   The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
///
/// - **SPI_DMA_INFIFO_FULL_ERR_INT_ST** `[0]` (RO)
///
///   The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
#[repr(transparent)]
pub struct SpiDmaIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaIntSt {
    // ======== Field Constants ========

    /// **SPI_APP1_INT_ST**
    ///
    /// The status bit for SPI_APP1_INT interrupt.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_APP1_INT_ST_OFFSET: u32 = 20;
    pub const SPI_APP1_INT_ST_MASK: u32 = 0x00100000;
    pub const SPI_APP1_INT_ST: u32 = 1 << 20;

    /// **SPI_APP2_INT_ST**
    ///
    /// The status bit for SPI_APP2_INT interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_APP2_INT_ST_OFFSET: u32 = 19;
    pub const SPI_APP2_INT_ST_MASK: u32 = 0x00080000;
    pub const SPI_APP2_INT_ST: u32 = 1 << 19;

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST**
    ///
    /// The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_OFFSET: u32 = 18;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_MASK: u32 = 0x00040000;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST: u32 = 1 << 18;

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST**
    ///
    /// The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_OFFSET: u32 = 17;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_MASK: u32 = 0x00020000;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST: u32 = 1 << 17;

    /// **SPI_SLV_CMD_ERR_INT_ST**
    ///
    /// The status bit for SPI_SLV_CMD_ERR_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SLV_CMD_ERR_INT_ST_OFFSET: u32 = 16;
    pub const SPI_SLV_CMD_ERR_INT_ST_MASK: u32 = 0x00010000;
    pub const SPI_SLV_CMD_ERR_INT_ST: u32 = 1 << 16;

    /// **SPI_SLV_BUF_ADDR_ERR_INT_ST**
    ///
    /// The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ST_OFFSET: u32 = 15;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ST_MASK: u32 = 0x00008000;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_ST: u32 = 1 << 15;

    /// **SPI_SEG_MAGIC_ERR_INT_ST**
    ///
    /// The status bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_SEG_MAGIC_ERR_INT_ST_OFFSET: u32 = 14;
    pub const SPI_SEG_MAGIC_ERR_INT_ST_MASK: u32 = 0x00004000;
    pub const SPI_SEG_MAGIC_ERR_INT_ST: u32 = 1 << 14;

    /// **SPI_DMA_SEG_TRANS_DONE_INT_ST**
    ///
    /// The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ST_OFFSET: u32 = 13;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ST_MASK: u32 = 0x00002000;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_ST: u32 = 1 << 13;

    /// **SPI_TRANS_DONE_INT_ST**
    ///
    /// The status bit for SPI_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_TRANS_DONE_INT_ST_OFFSET: u32 = 12;
    pub const SPI_TRANS_DONE_INT_ST_MASK: u32 = 0x00001000;
    pub const SPI_TRANS_DONE_INT_ST: u32 = 1 << 12;

    /// **SPI_SLV_WR_BUF_DONE_INT_ST**
    ///
    /// The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WR_BUF_DONE_INT_ST_OFFSET: u32 = 11;
    pub const SPI_SLV_WR_BUF_DONE_INT_ST_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WR_BUF_DONE_INT_ST: u32 = 1 << 11;

    /// **SPI_SLV_RD_BUF_DONE_INT_ST**
    ///
    /// The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RD_BUF_DONE_INT_ST_OFFSET: u32 = 10;
    pub const SPI_SLV_RD_BUF_DONE_INT_ST_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RD_BUF_DONE_INT_ST: u32 = 1 << 10;

    /// **SPI_SLV_WR_DMA_DONE_INT_ST**
    ///
    /// The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WR_DMA_DONE_INT_ST_OFFSET: u32 = 9;
    pub const SPI_SLV_WR_DMA_DONE_INT_ST_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WR_DMA_DONE_INT_ST: u32 = 1 << 9;

    /// **SPI_SLV_RD_DMA_DONE_INT_ST**
    ///
    /// The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RD_DMA_DONE_INT_ST_OFFSET: u32 = 8;
    pub const SPI_SLV_RD_DMA_DONE_INT_ST_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RD_DMA_DONE_INT_ST: u32 = 1 << 8;

    /// **SPI_SLV_CMDA_INT_ST**
    ///
    /// The status bit for SPI slave CMDA interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SLV_CMDA_INT_ST_OFFSET: u32 = 7;
    pub const SPI_SLV_CMDA_INT_ST_MASK: u32 = 0x00000080;
    pub const SPI_SLV_CMDA_INT_ST: u32 = 1 << 7;

    /// **SPI_SLV_CMD9_INT_ST**
    ///
    /// The status bit for SPI slave CMD9 interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SLV_CMD9_INT_ST_OFFSET: u32 = 6;
    pub const SPI_SLV_CMD9_INT_ST_MASK: u32 = 0x00000040;
    pub const SPI_SLV_CMD9_INT_ST: u32 = 1 << 6;

    /// **SPI_SLV_CMD8_INT_ST**
    ///
    /// The status bit for SPI slave CMD8 interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SLV_CMD8_INT_ST_OFFSET: u32 = 5;
    pub const SPI_SLV_CMD8_INT_ST_MASK: u32 = 0x00000020;
    pub const SPI_SLV_CMD8_INT_ST: u32 = 1 << 5;

    /// **SPI_SLV_CMD7_INT_ST**
    ///
    /// The status bit for SPI slave CMD7 interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SLV_CMD7_INT_ST_OFFSET: u32 = 4;
    pub const SPI_SLV_CMD7_INT_ST_MASK: u32 = 0x00000010;
    pub const SPI_SLV_CMD7_INT_ST: u32 = 1 << 4;

    /// **SPI_SLV_EN_QPI_INT_ST**
    ///
    /// The status bit for SPI slave En_QPI interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SLV_EN_QPI_INT_ST_OFFSET: u32 = 3;
    pub const SPI_SLV_EN_QPI_INT_ST_MASK: u32 = 0x00000008;
    pub const SPI_SLV_EN_QPI_INT_ST: u32 = 1 << 3;

    /// **SPI_SLV_EX_QPI_INT_ST**
    ///
    /// The status bit for SPI slave Ex_QPI interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SLV_EX_QPI_INT_ST_OFFSET: u32 = 2;
    pub const SPI_SLV_EX_QPI_INT_ST_MASK: u32 = 0x00000004;
    pub const SPI_SLV_EX_QPI_INT_ST: u32 = 1 << 2;

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST**
    ///
    /// The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_OFFSET: u32 = 1;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_MASK: u32 = 0x00000002;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST: u32 = 1 << 1;

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_ST**
    ///
    /// The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ST_OFFSET: u32 = 0;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ST_MASK: u32 = 0x00000001;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_APP1_INT_ST** `[20]` (RO)
    ///
    /// The status bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub fn spi_app1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP1_INT_ST_MASK) != 0
    }

    /// **SPI_APP2_INT_ST** `[19]` (RO)
    ///
    /// The status bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub fn spi_app2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_APP2_INT_ST_MASK) != 0
    }

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST** `[18]` (RO)
    ///
    /// The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST** `[17]` (RO)
    ///
    /// The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_CMD_ERR_INT_ST** `[16]` (RO)
    ///
    /// The status bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_BUF_ADDR_ERR_INT_ST** `[15]` (RO)
    ///
    /// The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_BUF_ADDR_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_SEG_MAGIC_ERR_INT_ST** `[14]` (RO)
    ///
    /// The status bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_seg_magic_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SEG_MAGIC_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_DMA_SEG_TRANS_DONE_INT_ST** `[13]` (RO)
    ///
    /// The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_SEG_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_TRANS_DONE_INT_ST** `[12]` (RO)
    ///
    /// The status bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_trans_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_WR_BUF_DONE_INT_ST** `[11]` (RO)
    ///
    /// The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_BUF_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_RD_BUF_DONE_INT_ST** `[10]` (RO)
    ///
    /// The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_BUF_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_WR_DMA_DONE_INT_ST** `[9]` (RO)
    ///
    /// The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WR_DMA_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_RD_DMA_DONE_INT_ST** `[8]` (RO)
    ///
    /// The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RD_DMA_DONE_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_CMDA_INT_ST** `[7]` (RO)
    ///
    /// The status bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub fn spi_slv_cmda_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMDA_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_CMD9_INT_ST** `[6]` (RO)
    ///
    /// The status bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd9_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD9_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_CMD8_INT_ST** `[5]` (RO)
    ///
    /// The status bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd8_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD8_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_CMD7_INT_ST** `[4]` (RO)
    ///
    /// The status bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub fn spi_slv_cmd7_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_CMD7_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_EN_QPI_INT_ST** `[3]` (RO)
    ///
    /// The status bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EN_QPI_INT_ST_MASK) != 0
    }

    /// **SPI_SLV_EX_QPI_INT_ST** `[2]` (RO)
    ///
    /// The status bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_EX_QPI_INT_ST_MASK) != 0
    }

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST** `[1]` (RO)
    ///
    /// The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_ST** `[0]` (RO)
    ///
    /// The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DMA_INFIFO_FULL_ERR_INT_ST_MASK) != 0
    }
}

/// SPI_DMA_INT_ST value type for batch read/modify/write operations
///
/// Use `spidmaintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaIntStVal {
    bits: u32,
}

impl SpiDmaIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_APP1_INT_ST** `[20:20]`
    ///
    /// The status bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub const fn spi_app1_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_APP1_INT_ST_MASK) != 0
    }

    /// Read **SPI_APP2_INT_ST** `[19:19]`
    ///
    /// The status bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub const fn spi_app2_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_APP2_INT_ST_MASK) != 0
    }

    /// Read **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST** `[18:18]`
    ///
    /// The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mst_tx_afifo_rempty_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST** `[17:17]`
    ///
    /// The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mst_rx_afifo_wfull_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_CMD_ERR_INT_ST** `[16:16]`
    ///
    /// The status bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_CMD_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_BUF_ADDR_ERR_INT_ST** `[15:15]`
    ///
    /// The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_buf_addr_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_BUF_ADDR_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_SEG_MAGIC_ERR_INT_ST** `[14:14]`
    ///
    /// The status bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_seg_magic_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SEG_MAGIC_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_DMA_SEG_TRANS_DONE_INT_ST** `[13:13]`
    ///
    /// The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_seg_trans_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_DMA_SEG_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_TRANS_DONE_INT_ST** `[12:12]`
    ///
    /// The status bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_trans_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_WR_BUF_DONE_INT_ST** `[11:11]`
    ///
    /// The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_wr_buf_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_WR_BUF_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_RD_BUF_DONE_INT_ST** `[10:10]`
    ///
    /// The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_rd_buf_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_RD_BUF_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_WR_DMA_DONE_INT_ST** `[9:9]`
    ///
    /// The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_wr_dma_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_WR_DMA_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_RD_DMA_DONE_INT_ST** `[8:8]`
    ///
    /// The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn spi_slv_rd_dma_done_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_RD_DMA_DONE_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_CMDA_INT_ST** `[7:7]`
    ///
    /// The status bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmda_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_CMDA_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_CMD9_INT_ST** `[6:6]`
    ///
    /// The status bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd9_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_CMD9_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_CMD8_INT_ST** `[5:5]`
    ///
    /// The status bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd8_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_CMD8_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_CMD7_INT_ST** `[4:4]`
    ///
    /// The status bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub const fn spi_slv_cmd7_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_CMD7_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_EN_QPI_INT_ST** `[3:3]`
    ///
    /// The status bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub const fn spi_slv_en_qpi_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_EN_QPI_INT_ST_MASK) != 0
    }

    /// Read **SPI_SLV_EX_QPI_INT_ST** `[2:2]`
    ///
    /// The status bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub const fn spi_slv_ex_qpi_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_SLV_EX_QPI_INT_ST_MASK) != 0
    }

    /// Read **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST** `[1:1]`
    ///
    /// The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_outfifo_empty_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_DMA_INFIFO_FULL_ERR_INT_ST** `[0:0]`
    ///
    /// The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_dma_infifo_full_err_int_st(&self) -> bool {
        (self.bits & SpiDmaIntSt::SPI_DMA_INFIFO_FULL_ERR_INT_ST_MASK) != 0
    }
}

impl Default for SpiDmaIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDmaIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDmaIntStVal {
        SpiDmaIntStVal::from_bits(self.value.read_bits())
    }
}

/// SPI_DMA_INT_SET Register
///
/// SPI interrupt software set register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **SPI_APP1_INT_SET** `[20]` (WO)
///
///   The software set bit for SPI_APP1_INT interrupt.
///
/// - **SPI_APP2_INT_SET** `[19]` (WO)
///
///   The software set bit for SPI_APP2_INT interrupt.
///
/// - **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET** `[18]` (WO)
///
///   The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
///
/// - **SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET** `[17]` (WO)
///
///   The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
///
/// - **SPI_SLV_CMD_ERR_INT_SET** `[16]` (WO)
///
///   The software set bit for SPI_SLV_CMD_ERR_INT interrupt.
///
/// - **SPI_SLV_BUF_ADDR_ERR_INT_SET** `[15]` (WO)
///
///   The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
///
/// - **SPI_SEG_MAGIC_ERR_INT_SET** `[14]` (WO)
///
///   The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt.
///
/// - **SPI_DMA_SEG_TRANS_DONE_INT_SET** `[13]` (WO)
///
///   The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
///
/// - **SPI_TRANS_DONE_INT_SET** `[12]` (WO)
///
///   The software set bit for SPI_TRANS_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_BUF_DONE_INT_SET** `[11]` (WO)
///
///   The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_BUF_DONE_INT_SET** `[10]` (WO)
///
///   The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
///
/// - **SPI_SLV_WR_DMA_DONE_INT_SET** `[9]` (WO)
///
///   The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_RD_DMA_DONE_INT_SET** `[8]` (WO)
///
///   The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
///
/// - **SPI_SLV_CMDA_INT_SET** `[7]` (WO)
///
///   The software set bit for SPI slave CMDA interrupt.
///
/// - **SPI_SLV_CMD9_INT_SET** `[6]` (WO)
///
///   The software set bit for SPI slave CMD9 interrupt.
///
/// - **SPI_SLV_CMD8_INT_SET** `[5]` (WO)
///
///   The software set bit for SPI slave CMD8 interrupt.
///
/// - **SPI_SLV_CMD7_INT_SET** `[4]` (WO)
///
///   The software set bit for SPI slave CMD7 interrupt.
///
/// - **SPI_SLV_EN_QPI_INT_SET** `[3]` (WO)
///
///   The software set bit for SPI slave En_QPI interrupt.
///
/// - **SPI_SLV_EX_QPI_INT_SET** `[2]` (WO)
///
///   The software set bit for SPI slave Ex_QPI interrupt.
///
/// - **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET** `[1]` (WO)
///
///   The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
///
/// - **SPI_DMA_INFIFO_FULL_ERR_INT_SET** `[0]` (WO)
///
///   The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
#[repr(transparent)]
pub struct SpiDmaIntSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDmaIntSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDmaIntSet {
    // ======== Field Constants ========

    /// **SPI_APP1_INT_SET**
    ///
    /// The software set bit for SPI_APP1_INT interrupt.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_APP1_INT_SET_OFFSET: u32 = 20;
    pub const SPI_APP1_INT_SET_MASK: u32 = 0x00100000;
    pub const SPI_APP1_INT_SET: u32 = 1 << 20;

    /// **SPI_APP2_INT_SET**
    ///
    /// The software set bit for SPI_APP2_INT interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_APP2_INT_SET_OFFSET: u32 = 19;
    pub const SPI_APP2_INT_SET_MASK: u32 = 0x00080000;
    pub const SPI_APP2_INT_SET: u32 = 1 << 19;

    /// **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET**
    ///
    /// The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_OFFSET: u32 = 18;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_MASK: u32 = 0x00040000;
    pub const SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET: u32 = 1 << 18;

    /// **SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET**
    ///
    /// The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_OFFSET: u32 = 17;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_MASK: u32 = 0x00020000;
    pub const SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET: u32 = 1 << 17;

    /// **SPI_SLV_CMD_ERR_INT_SET**
    ///
    /// The software set bit for SPI_SLV_CMD_ERR_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SLV_CMD_ERR_INT_SET_OFFSET: u32 = 16;
    pub const SPI_SLV_CMD_ERR_INT_SET_MASK: u32 = 0x00010000;
    pub const SPI_SLV_CMD_ERR_INT_SET: u32 = 1 << 16;

    /// **SPI_SLV_BUF_ADDR_ERR_INT_SET**
    ///
    /// The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SLV_BUF_ADDR_ERR_INT_SET_OFFSET: u32 = 15;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_SET_MASK: u32 = 0x00008000;
    pub const SPI_SLV_BUF_ADDR_ERR_INT_SET: u32 = 1 << 15;

    /// **SPI_SEG_MAGIC_ERR_INT_SET**
    ///
    /// The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_SEG_MAGIC_ERR_INT_SET_OFFSET: u32 = 14;
    pub const SPI_SEG_MAGIC_ERR_INT_SET_MASK: u32 = 0x00004000;
    pub const SPI_SEG_MAGIC_ERR_INT_SET: u32 = 1 << 14;

    /// **SPI_DMA_SEG_TRANS_DONE_INT_SET**
    ///
    /// The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DMA_SEG_TRANS_DONE_INT_SET_OFFSET: u32 = 13;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_SET_MASK: u32 = 0x00002000;
    pub const SPI_DMA_SEG_TRANS_DONE_INT_SET: u32 = 1 << 13;

    /// **SPI_TRANS_DONE_INT_SET**
    ///
    /// The software set bit for SPI_TRANS_DONE_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_TRANS_DONE_INT_SET_OFFSET: u32 = 12;
    pub const SPI_TRANS_DONE_INT_SET_MASK: u32 = 0x00001000;
    pub const SPI_TRANS_DONE_INT_SET: u32 = 1 << 12;

    /// **SPI_SLV_WR_BUF_DONE_INT_SET**
    ///
    /// The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WR_BUF_DONE_INT_SET_OFFSET: u32 = 11;
    pub const SPI_SLV_WR_BUF_DONE_INT_SET_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WR_BUF_DONE_INT_SET: u32 = 1 << 11;

    /// **SPI_SLV_RD_BUF_DONE_INT_SET**
    ///
    /// The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RD_BUF_DONE_INT_SET_OFFSET: u32 = 10;
    pub const SPI_SLV_RD_BUF_DONE_INT_SET_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RD_BUF_DONE_INT_SET: u32 = 1 << 10;

    /// **SPI_SLV_WR_DMA_DONE_INT_SET**
    ///
    /// The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WR_DMA_DONE_INT_SET_OFFSET: u32 = 9;
    pub const SPI_SLV_WR_DMA_DONE_INT_SET_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WR_DMA_DONE_INT_SET: u32 = 1 << 9;

    /// **SPI_SLV_RD_DMA_DONE_INT_SET**
    ///
    /// The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RD_DMA_DONE_INT_SET_OFFSET: u32 = 8;
    pub const SPI_SLV_RD_DMA_DONE_INT_SET_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RD_DMA_DONE_INT_SET: u32 = 1 << 8;

    /// **SPI_SLV_CMDA_INT_SET**
    ///
    /// The software set bit for SPI slave CMDA interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SLV_CMDA_INT_SET_OFFSET: u32 = 7;
    pub const SPI_SLV_CMDA_INT_SET_MASK: u32 = 0x00000080;
    pub const SPI_SLV_CMDA_INT_SET: u32 = 1 << 7;

    /// **SPI_SLV_CMD9_INT_SET**
    ///
    /// The software set bit for SPI slave CMD9 interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SLV_CMD9_INT_SET_OFFSET: u32 = 6;
    pub const SPI_SLV_CMD9_INT_SET_MASK: u32 = 0x00000040;
    pub const SPI_SLV_CMD9_INT_SET: u32 = 1 << 6;

    /// **SPI_SLV_CMD8_INT_SET**
    ///
    /// The software set bit for SPI slave CMD8 interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SLV_CMD8_INT_SET_OFFSET: u32 = 5;
    pub const SPI_SLV_CMD8_INT_SET_MASK: u32 = 0x00000020;
    pub const SPI_SLV_CMD8_INT_SET: u32 = 1 << 5;

    /// **SPI_SLV_CMD7_INT_SET**
    ///
    /// The software set bit for SPI slave CMD7 interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SLV_CMD7_INT_SET_OFFSET: u32 = 4;
    pub const SPI_SLV_CMD7_INT_SET_MASK: u32 = 0x00000010;
    pub const SPI_SLV_CMD7_INT_SET: u32 = 1 << 4;

    /// **SPI_SLV_EN_QPI_INT_SET**
    ///
    /// The software set bit for SPI slave En_QPI interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SLV_EN_QPI_INT_SET_OFFSET: u32 = 3;
    pub const SPI_SLV_EN_QPI_INT_SET_MASK: u32 = 0x00000008;
    pub const SPI_SLV_EN_QPI_INT_SET: u32 = 1 << 3;

    /// **SPI_SLV_EX_QPI_INT_SET**
    ///
    /// The software set bit for SPI slave Ex_QPI interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SLV_EX_QPI_INT_SET_OFFSET: u32 = 2;
    pub const SPI_SLV_EX_QPI_INT_SET_MASK: u32 = 0x00000004;
    pub const SPI_SLV_EX_QPI_INT_SET: u32 = 1 << 2;

    /// **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET**
    ///
    /// The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_OFFSET: u32 = 1;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_MASK: u32 = 0x00000002;
    pub const SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET: u32 = 1 << 1;

    /// **SPI_DMA_INFIFO_FULL_ERR_INT_SET**
    ///
    /// The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_SET_OFFSET: u32 = 0;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_SET_MASK: u32 = 0x00000001;
    pub const SPI_DMA_INFIFO_FULL_ERR_INT_SET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_APP1_INT_SET** (WO)
    ///
    /// The software set bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_app1_int_set(&self) {
        self.value.write_bits(Self::SPI_APP1_INT_SET_MASK);
    }

    /// Trigger **SPI_APP2_INT_SET** (WO)
    ///
    /// The software set bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_app2_int_set(&self) {
        self.value.write_bits(Self::SPI_APP2_INT_SET_MASK);
    }

    /// Trigger **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mst_tx_afifo_rempty_err_int_set(&self) {
        self.value
            .write_bits(Self::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mst_rx_afifo_wfull_err_int_set(&self) {
        self.value
            .write_bits(Self::SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_CMD_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd_err_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_BUF_ADDR_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_buf_addr_err_int_set(&self) {
        self.value
            .write_bits(Self::SPI_SLV_BUF_ADDR_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_SEG_MAGIC_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_seg_magic_err_int_set(&self) {
        self.value.write_bits(Self::SPI_SEG_MAGIC_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_DMA_SEG_TRANS_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_seg_trans_done_int_set(&self) {
        self.value
            .write_bits(Self::SPI_DMA_SEG_TRANS_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_TRANS_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_trans_done_int_set(&self) {
        self.value.write_bits(Self::SPI_TRANS_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_WR_BUF_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_wr_buf_done_int_set(&self) {
        self.value
            .write_bits(Self::SPI_SLV_WR_BUF_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_RD_BUF_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_rd_buf_done_int_set(&self) {
        self.value
            .write_bits(Self::SPI_SLV_RD_BUF_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_WR_DMA_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_wr_dma_done_int_set(&self) {
        self.value
            .write_bits(Self::SPI_SLV_WR_DMA_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_RD_DMA_DONE_INT_SET** (WO)
    ///
    /// The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_rd_dma_done_int_set(&self) {
        self.value
            .write_bits(Self::SPI_SLV_RD_DMA_DONE_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_CMDA_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmda_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_CMDA_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_CMD9_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd9_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD9_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_CMD8_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd8_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD8_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_CMD7_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_cmd7_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_CMD7_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_EN_QPI_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_en_qpi_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_EN_QPI_INT_SET_MASK);
    }

    /// Trigger **SPI_SLV_EX_QPI_INT_SET** (WO)
    ///
    /// The software set bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub fn trigger_spi_slv_ex_qpi_int_set(&self) {
        self.value.write_bits(Self::SPI_SLV_EX_QPI_INT_SET_MASK);
    }

    /// Trigger **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_outfifo_empty_err_int_set(&self) {
        self.value
            .write_bits(Self::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_MASK);
    }

    /// Trigger **SPI_DMA_INFIFO_FULL_ERR_INT_SET** (WO)
    ///
    /// The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_dma_infifo_full_err_int_set(&self) {
        self.value
            .write_bits(Self::SPI_DMA_INFIFO_FULL_ERR_INT_SET_MASK);
    }
}

/// SPI_DMA_INT_SET value type for batch read/modify/write operations
///
/// Use `spidmaintset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidmaintset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDmaIntSetVal {
    bits: u32,
}

impl SpiDmaIntSetVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_APP1_INT_SET** `[20:20]`
    ///
    /// The software set bit for SPI_APP1_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app1_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_APP1_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_APP1_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_APP2_INT_SET** `[19:19]`
    ///
    /// The software set bit for SPI_APP2_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_app2_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_APP2_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_APP2_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET** `[18:18]`
    ///
    /// The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_tx_afifo_rempty_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET** `[17:17]`
    ///
    /// The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mst_rx_afifo_wfull_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD_ERR_INT_SET** `[16:16]`
    ///
    /// The software set bit for SPI_SLV_CMD_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_CMD_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_CMD_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_BUF_ADDR_ERR_INT_SET** `[15:15]`
    ///
    /// The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_buf_addr_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_BUF_ADDR_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_BUF_ADDR_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SEG_MAGIC_ERR_INT_SET** `[14:14]`
    ///
    /// The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_seg_magic_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SEG_MAGIC_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SEG_MAGIC_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_DMA_SEG_TRANS_DONE_INT_SET** `[13:13]`
    ///
    /// The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_seg_trans_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_DMA_SEG_TRANS_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_DMA_SEG_TRANS_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_TRANS_DONE_INT_SET** `[12:12]`
    ///
    /// The software set bit for SPI_TRANS_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_trans_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_TRANS_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_TRANS_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_BUF_DONE_INT_SET** `[11:11]`
    ///
    /// The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_buf_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_WR_BUF_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_WR_BUF_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_BUF_DONE_INT_SET** `[10:10]`
    ///
    /// The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_buf_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_RD_BUF_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_RD_BUF_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_WR_DMA_DONE_INT_SET** `[9:9]`
    ///
    /// The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_wr_dma_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_WR_DMA_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_WR_DMA_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_RD_DMA_DONE_INT_SET** `[8:8]`
    ///
    /// The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_rd_dma_done_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_RD_DMA_DONE_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_RD_DMA_DONE_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMDA_INT_SET** `[7:7]`
    ///
    /// The software set bit for SPI slave CMDA interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmda_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_CMDA_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_CMDA_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD9_INT_SET** `[6:6]`
    ///
    /// The software set bit for SPI slave CMD9 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd9_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_CMD9_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_CMD9_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD8_INT_SET** `[5:5]`
    ///
    /// The software set bit for SPI slave CMD8 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd8_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_CMD8_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_CMD8_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_CMD7_INT_SET** `[4:4]`
    ///
    /// The software set bit for SPI slave CMD7 interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_cmd7_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_CMD7_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_CMD7_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_EN_QPI_INT_SET** `[3:3]`
    ///
    /// The software set bit for SPI slave En_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_en_qpi_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_EN_QPI_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_EN_QPI_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_SLV_EX_QPI_INT_SET** `[2:2]`
    ///
    /// The software set bit for SPI slave Ex_QPI interrupt.
    #[inline(always)]
    pub const fn with_spi_slv_ex_qpi_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_SLV_EX_QPI_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_SLV_EX_QPI_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET** `[1:1]`
    ///
    /// The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_outfifo_empty_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_MASK
            },
        }
    }

    /// Set **SPI_DMA_INFIFO_FULL_ERR_INT_SET** `[0:0]`
    ///
    /// The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_dma_infifo_full_err_int_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiDmaIntSet::SPI_DMA_INFIFO_FULL_ERR_INT_SET_MASK
            } else {
                self.bits & !SpiDmaIntSet::SPI_DMA_INFIFO_FULL_ERR_INT_SET_MASK
            },
        }
    }
}

impl Default for SpiDmaIntSetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiDmaIntSet {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDmaIntSetVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_W0 Register
///
/// SPI CPU-controlled buffer0
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **SPI_BUF0** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW0 {
    // ======== Field Constants ========

    /// **SPI_BUF0**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF0_OFFSET: u32 = 0;
    pub const SPI_BUF0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF0** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf0(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF0_MASK) >> Self::SPI_BUF0_OFFSET
    }

    /// Write **SPI_BUF0** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF0_MASK) | ((value << Self::SPI_BUF0_OFFSET) & Self::SPI_BUF0_MASK)
        });
    }
}

/// SPI_W0 value type for batch read/modify/write operations
///
/// Use `spiw0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW0Val {
    bits: u32,
}

impl SpiW0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF0** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf0(&self) -> u32 {
        (self.bits & SpiW0::SPI_BUF0_MASK) >> SpiW0::SPI_BUF0_OFFSET
    }

    /// Set **SPI_BUF0** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW0::SPI_BUF0_MASK)
                | ((value << SpiW0::SPI_BUF0_OFFSET) & SpiW0::SPI_BUF0_MASK),
        }
    }
}

impl Default for SpiW0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW0Val {
        SpiW0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW0Val) -> SpiW0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W1 Register
///
/// SPI CPU-controlled buffer1
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **SPI_BUF1** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW1 {
    // ======== Field Constants ========

    /// **SPI_BUF1**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF1_OFFSET: u32 = 0;
    pub const SPI_BUF1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF1** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf1(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF1_MASK) >> Self::SPI_BUF1_OFFSET
    }

    /// Write **SPI_BUF1** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF1_MASK) | ((value << Self::SPI_BUF1_OFFSET) & Self::SPI_BUF1_MASK)
        });
    }
}

/// SPI_W1 value type for batch read/modify/write operations
///
/// Use `spiw1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW1Val {
    bits: u32,
}

impl SpiW1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF1** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf1(&self) -> u32 {
        (self.bits & SpiW1::SPI_BUF1_MASK) >> SpiW1::SPI_BUF1_OFFSET
    }

    /// Set **SPI_BUF1** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW1::SPI_BUF1_MASK)
                | ((value << SpiW1::SPI_BUF1_OFFSET) & SpiW1::SPI_BUF1_MASK),
        }
    }
}

impl Default for SpiW1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW1Val {
        SpiW1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW1Val) -> SpiW1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W2 Register
///
/// SPI CPU-controlled buffer2
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **SPI_BUF2** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW2 {
    // ======== Field Constants ========

    /// **SPI_BUF2**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF2_OFFSET: u32 = 0;
    pub const SPI_BUF2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF2** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf2(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF2_MASK) >> Self::SPI_BUF2_OFFSET
    }

    /// Write **SPI_BUF2** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF2_MASK) | ((value << Self::SPI_BUF2_OFFSET) & Self::SPI_BUF2_MASK)
        });
    }
}

/// SPI_W2 value type for batch read/modify/write operations
///
/// Use `spiw2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW2Val {
    bits: u32,
}

impl SpiW2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF2** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf2(&self) -> u32 {
        (self.bits & SpiW2::SPI_BUF2_MASK) >> SpiW2::SPI_BUF2_OFFSET
    }

    /// Set **SPI_BUF2** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW2::SPI_BUF2_MASK)
                | ((value << SpiW2::SPI_BUF2_OFFSET) & SpiW2::SPI_BUF2_MASK),
        }
    }
}

impl Default for SpiW2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW2Val {
        SpiW2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW2Val) -> SpiW2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W3 Register
///
/// SPI CPU-controlled buffer3
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **SPI_BUF3** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW3 {
    // ======== Field Constants ========

    /// **SPI_BUF3**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF3_OFFSET: u32 = 0;
    pub const SPI_BUF3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF3** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf3(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF3_MASK) >> Self::SPI_BUF3_OFFSET
    }

    /// Write **SPI_BUF3** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF3_MASK) | ((value << Self::SPI_BUF3_OFFSET) & Self::SPI_BUF3_MASK)
        });
    }
}

/// SPI_W3 value type for batch read/modify/write operations
///
/// Use `spiw3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW3Val {
    bits: u32,
}

impl SpiW3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF3** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf3(&self) -> u32 {
        (self.bits & SpiW3::SPI_BUF3_MASK) >> SpiW3::SPI_BUF3_OFFSET
    }

    /// Set **SPI_BUF3** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW3::SPI_BUF3_MASK)
                | ((value << SpiW3::SPI_BUF3_OFFSET) & SpiW3::SPI_BUF3_MASK),
        }
    }
}

impl Default for SpiW3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW3Val {
        SpiW3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW3Val) -> SpiW3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W4 Register
///
/// SPI CPU-controlled buffer4
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **SPI_BUF4** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW4 {
    // ======== Field Constants ========

    /// **SPI_BUF4**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF4_OFFSET: u32 = 0;
    pub const SPI_BUF4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF4** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf4(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF4_MASK) >> Self::SPI_BUF4_OFFSET
    }

    /// Write **SPI_BUF4** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF4_MASK) | ((value << Self::SPI_BUF4_OFFSET) & Self::SPI_BUF4_MASK)
        });
    }
}

/// SPI_W4 value type for batch read/modify/write operations
///
/// Use `spiw4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW4Val {
    bits: u32,
}

impl SpiW4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF4** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf4(&self) -> u32 {
        (self.bits & SpiW4::SPI_BUF4_MASK) >> SpiW4::SPI_BUF4_OFFSET
    }

    /// Set **SPI_BUF4** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW4::SPI_BUF4_MASK)
                | ((value << SpiW4::SPI_BUF4_OFFSET) & SpiW4::SPI_BUF4_MASK),
        }
    }
}

impl Default for SpiW4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW4Val {
        SpiW4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW4Val) -> SpiW4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W5 Register
///
/// SPI CPU-controlled buffer5
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **SPI_BUF5** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW5 {
    // ======== Field Constants ========

    /// **SPI_BUF5**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF5_OFFSET: u32 = 0;
    pub const SPI_BUF5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF5** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf5(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF5_MASK) >> Self::SPI_BUF5_OFFSET
    }

    /// Write **SPI_BUF5** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF5_MASK) | ((value << Self::SPI_BUF5_OFFSET) & Self::SPI_BUF5_MASK)
        });
    }
}

/// SPI_W5 value type for batch read/modify/write operations
///
/// Use `spiw5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW5Val {
    bits: u32,
}

impl SpiW5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF5** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf5(&self) -> u32 {
        (self.bits & SpiW5::SPI_BUF5_MASK) >> SpiW5::SPI_BUF5_OFFSET
    }

    /// Set **SPI_BUF5** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW5::SPI_BUF5_MASK)
                | ((value << SpiW5::SPI_BUF5_OFFSET) & SpiW5::SPI_BUF5_MASK),
        }
    }
}

impl Default for SpiW5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW5Val {
        SpiW5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW5Val) -> SpiW5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W6 Register
///
/// SPI CPU-controlled buffer6
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **SPI_BUF6** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW6 {
    // ======== Field Constants ========

    /// **SPI_BUF6**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF6_OFFSET: u32 = 0;
    pub const SPI_BUF6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF6** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf6(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF6_MASK) >> Self::SPI_BUF6_OFFSET
    }

    /// Write **SPI_BUF6** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF6_MASK) | ((value << Self::SPI_BUF6_OFFSET) & Self::SPI_BUF6_MASK)
        });
    }
}

/// SPI_W6 value type for batch read/modify/write operations
///
/// Use `spiw6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW6Val {
    bits: u32,
}

impl SpiW6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF6** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf6(&self) -> u32 {
        (self.bits & SpiW6::SPI_BUF6_MASK) >> SpiW6::SPI_BUF6_OFFSET
    }

    /// Set **SPI_BUF6** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW6::SPI_BUF6_MASK)
                | ((value << SpiW6::SPI_BUF6_OFFSET) & SpiW6::SPI_BUF6_MASK),
        }
    }
}

impl Default for SpiW6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW6Val {
        SpiW6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW6Val) -> SpiW6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W7 Register
///
/// SPI CPU-controlled buffer7
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **SPI_BUF7** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW7 {
    // ======== Field Constants ========

    /// **SPI_BUF7**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF7_OFFSET: u32 = 0;
    pub const SPI_BUF7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF7** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf7(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF7_MASK) >> Self::SPI_BUF7_OFFSET
    }

    /// Write **SPI_BUF7** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF7_MASK) | ((value << Self::SPI_BUF7_OFFSET) & Self::SPI_BUF7_MASK)
        });
    }
}

/// SPI_W7 value type for batch read/modify/write operations
///
/// Use `spiw7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW7Val {
    bits: u32,
}

impl SpiW7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF7** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf7(&self) -> u32 {
        (self.bits & SpiW7::SPI_BUF7_MASK) >> SpiW7::SPI_BUF7_OFFSET
    }

    /// Set **SPI_BUF7** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW7::SPI_BUF7_MASK)
                | ((value << SpiW7::SPI_BUF7_OFFSET) & SpiW7::SPI_BUF7_MASK),
        }
    }
}

impl Default for SpiW7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW7Val {
        SpiW7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW7Val) -> SpiW7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W8 Register
///
/// SPI CPU-controlled buffer8
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **SPI_BUF8** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW8 {
    // ======== Field Constants ========

    /// **SPI_BUF8**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF8_OFFSET: u32 = 0;
    pub const SPI_BUF8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF8** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf8(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF8_MASK) >> Self::SPI_BUF8_OFFSET
    }

    /// Write **SPI_BUF8** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF8_MASK) | ((value << Self::SPI_BUF8_OFFSET) & Self::SPI_BUF8_MASK)
        });
    }
}

/// SPI_W8 value type for batch read/modify/write operations
///
/// Use `spiw8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW8Val {
    bits: u32,
}

impl SpiW8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF8** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf8(&self) -> u32 {
        (self.bits & SpiW8::SPI_BUF8_MASK) >> SpiW8::SPI_BUF8_OFFSET
    }

    /// Set **SPI_BUF8** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW8::SPI_BUF8_MASK)
                | ((value << SpiW8::SPI_BUF8_OFFSET) & SpiW8::SPI_BUF8_MASK),
        }
    }
}

impl Default for SpiW8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW8Val {
        SpiW8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW8Val) -> SpiW8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W9 Register
///
/// SPI CPU-controlled buffer9
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **SPI_BUF9** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW9 {
    // ======== Field Constants ========

    /// **SPI_BUF9**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF9_OFFSET: u32 = 0;
    pub const SPI_BUF9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF9** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf9(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF9_MASK) >> Self::SPI_BUF9_OFFSET
    }

    /// Write **SPI_BUF9** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF9_MASK) | ((value << Self::SPI_BUF9_OFFSET) & Self::SPI_BUF9_MASK)
        });
    }
}

/// SPI_W9 value type for batch read/modify/write operations
///
/// Use `spiw9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW9Val {
    bits: u32,
}

impl SpiW9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF9** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf9(&self) -> u32 {
        (self.bits & SpiW9::SPI_BUF9_MASK) >> SpiW9::SPI_BUF9_OFFSET
    }

    /// Set **SPI_BUF9** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW9::SPI_BUF9_MASK)
                | ((value << SpiW9::SPI_BUF9_OFFSET) & SpiW9::SPI_BUF9_MASK),
        }
    }
}

impl Default for SpiW9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW9Val {
        SpiW9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW9Val) -> SpiW9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W10 Register
///
/// SPI CPU-controlled buffer10
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **SPI_BUF10** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW10 {
    // ======== Field Constants ========

    /// **SPI_BUF10**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF10_OFFSET: u32 = 0;
    pub const SPI_BUF10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF10** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf10(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF10_MASK) >> Self::SPI_BUF10_OFFSET
    }

    /// Write **SPI_BUF10** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF10_MASK) | ((value << Self::SPI_BUF10_OFFSET) & Self::SPI_BUF10_MASK)
        });
    }
}

/// SPI_W10 value type for batch read/modify/write operations
///
/// Use `spiw10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW10Val {
    bits: u32,
}

impl SpiW10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF10** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf10(&self) -> u32 {
        (self.bits & SpiW10::SPI_BUF10_MASK) >> SpiW10::SPI_BUF10_OFFSET
    }

    /// Set **SPI_BUF10** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW10::SPI_BUF10_MASK)
                | ((value << SpiW10::SPI_BUF10_OFFSET) & SpiW10::SPI_BUF10_MASK),
        }
    }
}

impl Default for SpiW10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW10Val {
        SpiW10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW10Val) -> SpiW10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W11 Register
///
/// SPI CPU-controlled buffer11
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **SPI_BUF11** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW11 {
    // ======== Field Constants ========

    /// **SPI_BUF11**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF11_OFFSET: u32 = 0;
    pub const SPI_BUF11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF11** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf11(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF11_MASK) >> Self::SPI_BUF11_OFFSET
    }

    /// Write **SPI_BUF11** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF11_MASK) | ((value << Self::SPI_BUF11_OFFSET) & Self::SPI_BUF11_MASK)
        });
    }
}

/// SPI_W11 value type for batch read/modify/write operations
///
/// Use `spiw11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW11Val {
    bits: u32,
}

impl SpiW11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF11** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf11(&self) -> u32 {
        (self.bits & SpiW11::SPI_BUF11_MASK) >> SpiW11::SPI_BUF11_OFFSET
    }

    /// Set **SPI_BUF11** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW11::SPI_BUF11_MASK)
                | ((value << SpiW11::SPI_BUF11_OFFSET) & SpiW11::SPI_BUF11_MASK),
        }
    }
}

impl Default for SpiW11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW11Val {
        SpiW11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW11Val) -> SpiW11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W12 Register
///
/// SPI CPU-controlled buffer12
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **SPI_BUF12** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW12 {
    // ======== Field Constants ========

    /// **SPI_BUF12**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF12_OFFSET: u32 = 0;
    pub const SPI_BUF12_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF12** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf12(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF12_MASK) >> Self::SPI_BUF12_OFFSET
    }

    /// Write **SPI_BUF12** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF12_MASK) | ((value << Self::SPI_BUF12_OFFSET) & Self::SPI_BUF12_MASK)
        });
    }
}

/// SPI_W12 value type for batch read/modify/write operations
///
/// Use `spiw12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW12Val {
    bits: u32,
}

impl SpiW12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF12** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf12(&self) -> u32 {
        (self.bits & SpiW12::SPI_BUF12_MASK) >> SpiW12::SPI_BUF12_OFFSET
    }

    /// Set **SPI_BUF12** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW12::SPI_BUF12_MASK)
                | ((value << SpiW12::SPI_BUF12_OFFSET) & SpiW12::SPI_BUF12_MASK),
        }
    }
}

impl Default for SpiW12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW12Val {
        SpiW12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW12Val) -> SpiW12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W13 Register
///
/// SPI CPU-controlled buffer13
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **SPI_BUF13** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW13 {
    // ======== Field Constants ========

    /// **SPI_BUF13**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF13_OFFSET: u32 = 0;
    pub const SPI_BUF13_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF13** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf13(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF13_MASK) >> Self::SPI_BUF13_OFFSET
    }

    /// Write **SPI_BUF13** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF13_MASK) | ((value << Self::SPI_BUF13_OFFSET) & Self::SPI_BUF13_MASK)
        });
    }
}

/// SPI_W13 value type for batch read/modify/write operations
///
/// Use `spiw13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW13Val {
    bits: u32,
}

impl SpiW13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF13** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf13(&self) -> u32 {
        (self.bits & SpiW13::SPI_BUF13_MASK) >> SpiW13::SPI_BUF13_OFFSET
    }

    /// Set **SPI_BUF13** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW13::SPI_BUF13_MASK)
                | ((value << SpiW13::SPI_BUF13_OFFSET) & SpiW13::SPI_BUF13_MASK),
        }
    }
}

impl Default for SpiW13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW13Val {
        SpiW13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW13Val) -> SpiW13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W14 Register
///
/// SPI CPU-controlled buffer14
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **SPI_BUF14** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW14 {
    // ======== Field Constants ========

    /// **SPI_BUF14**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF14_OFFSET: u32 = 0;
    pub const SPI_BUF14_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF14** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf14(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF14_MASK) >> Self::SPI_BUF14_OFFSET
    }

    /// Write **SPI_BUF14** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF14_MASK) | ((value << Self::SPI_BUF14_OFFSET) & Self::SPI_BUF14_MASK)
        });
    }
}

/// SPI_W14 value type for batch read/modify/write operations
///
/// Use `spiw14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW14Val {
    bits: u32,
}

impl SpiW14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF14** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf14(&self) -> u32 {
        (self.bits & SpiW14::SPI_BUF14_MASK) >> SpiW14::SPI_BUF14_OFFSET
    }

    /// Set **SPI_BUF14** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW14::SPI_BUF14_MASK)
                | ((value << SpiW14::SPI_BUF14_OFFSET) & SpiW14::SPI_BUF14_MASK),
        }
    }
}

impl Default for SpiW14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW14Val {
        SpiW14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW14Val) -> SpiW14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_W15 Register
///
/// SPI CPU-controlled buffer15
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **SPI_BUF15** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiW15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiW15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiW15 {
    // ======== Field Constants ========

    /// **SPI_BUF15**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_BUF15_OFFSET: u32 = 0;
    pub const SPI_BUF15_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_BUF15** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_buf15(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BUF15_MASK) >> Self::SPI_BUF15_OFFSET
    }

    /// Write **SPI_BUF15** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_buf15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_BUF15_MASK) | ((value << Self::SPI_BUF15_OFFSET) & Self::SPI_BUF15_MASK)
        });
    }
}

/// SPI_W15 value type for batch read/modify/write operations
///
/// Use `spiw15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiw15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiW15Val {
    bits: u32,
}

impl SpiW15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_BUF15** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_buf15(&self) -> u32 {
        (self.bits & SpiW15::SPI_BUF15_MASK) >> SpiW15::SPI_BUF15_OFFSET
    }

    /// Set **SPI_BUF15** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_buf15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiW15::SPI_BUF15_MASK)
                | ((value << SpiW15::SPI_BUF15_OFFSET) & SpiW15::SPI_BUF15_MASK),
        }
    }
}

impl Default for SpiW15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiW15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiW15Val {
        SpiW15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiW15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiW15Val) -> SpiW15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SLAVE Register
///
/// SPI slave control register
///
/// Address offset: 0x00e0
/// Reset value: 0x02800000
///
/// ## Fields
///
/// - **SPI_MST_FD_WAIT_DMA_TX_DATA** `[29]` (RW)
///
///   In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer.
///
/// - **SPI_USR_CONF** `[28]` (RW)
///
///   1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode.
///
/// - **SPI_SOFT_RESET** `[27]` (WO)
///
///   Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
///
/// - **MODE** `[26]` (RW)
///
///   Set SPI work mode. 1: slave mode 0: master mode.
///
/// - **SPI_DMA_SEG_MAGIC_VALUE** `[25:22]` (RW)
///
///   The magic value of BM table in master DMA seg-trans.
///
/// - **SPI_SLV_LAST_BYTE_STRB** `[19:12]` (RO)
///
///   Represents the effective bit of the last received data byte in SPI slave FD and HD mode.
///
/// - **SPI_SLV_WRBUF_BITLEN_EN** `[11]` (RW)
///
///   1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others
///
/// - **SPI_SLV_RDBUF_BITLEN_EN** `[10]` (RW)
///
///   1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others
///
/// - **SPI_SLV_WRDMA_BITLEN_EN** `[9]` (RW)
///
///   1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others
///
/// - **SPI_SLV_RDDMA_BITLEN_EN** `[8]` (RW)
///
///   1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others
///
/// - **SPI_RSCK_DATA_OUT** `[3]` (RW)
///
///   It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge
///
/// - **SPI_CLK_MODE_13** `[2]` (RW)
///
///   {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B[0]/B[7]. 0: support spi clk mode 0 and 2, first edge output data B[1]/B[6].
///
/// - **SPI_CLK_MODE** `[1:0]` (RW)
///
///   SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
#[repr(transparent)]
pub struct SpiSlave {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSlave {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSlave {
    // ======== Field Constants ========

    /// **SPI_MST_FD_WAIT_DMA_TX_DATA**
    ///
    /// In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MST_FD_WAIT_DMA_TX_DATA_OFFSET: u32 = 29;
    pub const SPI_MST_FD_WAIT_DMA_TX_DATA_MASK: u32 = 0x20000000;
    pub const SPI_MST_FD_WAIT_DMA_TX_DATA: u32 = 1 << 29;

    /// **SPI_USR_CONF**
    ///
    /// 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_USR_CONF_OFFSET: u32 = 28;
    pub const SPI_USR_CONF_MASK: u32 = 0x10000000;
    pub const SPI_USR_CONF: u32 = 1 << 28;

    /// **SPI_SOFT_RESET**
    ///
    /// Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_SOFT_RESET_OFFSET: u32 = 27;
    pub const SPI_SOFT_RESET_MASK: u32 = 0x08000000;
    pub const SPI_SOFT_RESET: u32 = 1 << 27;

    /// **MODE**
    ///
    /// Set SPI work mode. 1: slave mode 0: master mode.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MODE_OFFSET: u32 = 26;
    pub const MODE_MASK: u32 = 0x04000000;
    pub const MODE: u32 = 1 << 26;

    /// **SPI_DMA_SEG_MAGIC_VALUE**
    ///
    /// The magic value of BM table in master DMA seg-trans.
    ///
    /// Bits `[25:22]`, mask `0x03c00000`
    pub const SPI_DMA_SEG_MAGIC_VALUE_OFFSET: u32 = 22;
    pub const SPI_DMA_SEG_MAGIC_VALUE_MASK: u32 = 0x03c00000;

    /// **SPI_SLV_LAST_BYTE_STRB**
    ///
    /// Represents the effective bit of the last received data byte in SPI slave FD and HD mode.
    ///
    /// Bits `[19:12]`, mask `0x000ff000`
    pub const SPI_SLV_LAST_BYTE_STRB_OFFSET: u32 = 12;
    pub const SPI_SLV_LAST_BYTE_STRB_MASK: u32 = 0x000ff000;

    /// **SPI_SLV_WRBUF_BITLEN_EN**
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_SLV_WRBUF_BITLEN_EN_OFFSET: u32 = 11;
    pub const SPI_SLV_WRBUF_BITLEN_EN_MASK: u32 = 0x00000800;
    pub const SPI_SLV_WRBUF_BITLEN_EN: u32 = 1 << 11;

    /// **SPI_SLV_RDBUF_BITLEN_EN**
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_SLV_RDBUF_BITLEN_EN_OFFSET: u32 = 10;
    pub const SPI_SLV_RDBUF_BITLEN_EN_MASK: u32 = 0x00000400;
    pub const SPI_SLV_RDBUF_BITLEN_EN: u32 = 1 << 10;

    /// **SPI_SLV_WRDMA_BITLEN_EN**
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_SLV_WRDMA_BITLEN_EN_OFFSET: u32 = 9;
    pub const SPI_SLV_WRDMA_BITLEN_EN_MASK: u32 = 0x00000200;
    pub const SPI_SLV_WRDMA_BITLEN_EN: u32 = 1 << 9;

    /// **SPI_SLV_RDDMA_BITLEN_EN**
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SLV_RDDMA_BITLEN_EN_OFFSET: u32 = 8;
    pub const SPI_SLV_RDDMA_BITLEN_EN_MASK: u32 = 0x00000100;
    pub const SPI_SLV_RDDMA_BITLEN_EN: u32 = 1 << 8;

    /// **SPI_RSCK_DATA_OUT**
    ///
    /// It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_RSCK_DATA_OUT_OFFSET: u32 = 3;
    pub const SPI_RSCK_DATA_OUT_MASK: u32 = 0x00000008;
    pub const SPI_RSCK_DATA_OUT: u32 = 1 << 3;

    /// **SPI_CLK_MODE_13**
    ///
    /// {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B[0]/B[7]. 0: support spi clk mode 0 and 2, first edge output data B[1]/B[6].
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_CLK_MODE_13_OFFSET: u32 = 2;
    pub const SPI_CLK_MODE_13_MASK: u32 = 0x00000004;
    pub const SPI_CLK_MODE_13: u32 = 1 << 2;

    /// **SPI_CLK_MODE**
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_CLK_MODE_OFFSET: u32 = 0;
    pub const SPI_CLK_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_MST_FD_WAIT_DMA_TX_DATA** `[29]` (RW)
    ///
    /// In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer.
    #[inline(always)]
    pub fn spi_mst_fd_wait_dma_tx_data(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK) != 0
    }

    /// Set SPI_MST_FD_WAIT_DMA_TX_DATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_fd_wait_dma_tx_data(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK);
    }

    /// Clear SPI_MST_FD_WAIT_DMA_TX_DATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_fd_wait_dma_tx_data(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK);
    }

    /// **SPI_USR_CONF** `[28]` (RW)
    ///
    /// 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode.
    #[inline(always)]
    pub fn spi_usr_conf(&self) -> bool {
        (self.value.read_bits() & Self::SPI_USR_CONF_MASK) != 0
    }

    /// Set SPI_USR_CONF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_usr_conf(&self) {
        self.value.modify_bits(|r| r | Self::SPI_USR_CONF_MASK);
    }

    /// Clear SPI_USR_CONF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_usr_conf(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_USR_CONF_MASK);
    }

    /// Trigger **SPI_SOFT_RESET** (WO)
    ///
    /// Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
    #[inline(always)]
    pub fn trigger_spi_soft_reset(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SOFT_RESET_MASK);
    }

    /// **MODE** `[26]` (RW)
    ///
    /// Set SPI work mode. 1: slave mode 0: master mode.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode(&self) {
        self.value.modify_bits(|r| r | Self::MODE_MASK);
    }

    /// Clear MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.value.modify_bits(|r| r & !Self::MODE_MASK);
    }

    /// **SPI_DMA_SEG_MAGIC_VALUE** `[25:22]` (RW)
    ///
    /// The magic value of BM table in master DMA seg-trans.
    #[inline(always)]
    pub fn spi_dma_seg_magic_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DMA_SEG_MAGIC_VALUE_MASK)
            >> Self::SPI_DMA_SEG_MAGIC_VALUE_OFFSET
    }

    /// Write **SPI_DMA_SEG_MAGIC_VALUE** `[25:22]` (RW)
    ///
    /// The magic value of BM table in master DMA seg-trans.
    #[inline(always)]
    pub fn write_spi_dma_seg_magic_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DMA_SEG_MAGIC_VALUE_MASK)
                | ((value << Self::SPI_DMA_SEG_MAGIC_VALUE_OFFSET)
                    & Self::SPI_DMA_SEG_MAGIC_VALUE_MASK)
        });
    }

    /// **SPI_SLV_LAST_BYTE_STRB** `[19:12]` (RO)
    ///
    /// Represents the effective bit of the last received data byte in SPI slave FD and HD mode.
    #[inline(always)]
    pub fn spi_slv_last_byte_strb(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SLV_LAST_BYTE_STRB_MASK)
            >> Self::SPI_SLV_LAST_BYTE_STRB_OFFSET
    }

    /// **SPI_SLV_WRBUF_BITLEN_EN** `[11]` (RW)
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others
    #[inline(always)]
    pub fn spi_slv_wrbuf_bitlen_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WRBUF_BITLEN_EN_MASK) != 0
    }

    /// Set SPI_SLV_WRBUF_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wrbuf_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WRBUF_BITLEN_EN_MASK);
    }

    /// Clear SPI_SLV_WRBUF_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wrbuf_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WRBUF_BITLEN_EN_MASK);
    }

    /// **SPI_SLV_RDBUF_BITLEN_EN** `[10]` (RW)
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others
    #[inline(always)]
    pub fn spi_slv_rdbuf_bitlen_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RDBUF_BITLEN_EN_MASK) != 0
    }

    /// Set SPI_SLV_RDBUF_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rdbuf_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RDBUF_BITLEN_EN_MASK);
    }

    /// Clear SPI_SLV_RDBUF_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rdbuf_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RDBUF_BITLEN_EN_MASK);
    }

    /// **SPI_SLV_WRDMA_BITLEN_EN** `[9]` (RW)
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others
    #[inline(always)]
    pub fn spi_slv_wrdma_bitlen_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_WRDMA_BITLEN_EN_MASK) != 0
    }

    /// Set SPI_SLV_WRDMA_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_wrdma_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_WRDMA_BITLEN_EN_MASK);
    }

    /// Clear SPI_SLV_WRDMA_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_wrdma_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_WRDMA_BITLEN_EN_MASK);
    }

    /// **SPI_SLV_RDDMA_BITLEN_EN** `[8]` (RW)
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others
    #[inline(always)]
    pub fn spi_slv_rddma_bitlen_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SLV_RDDMA_BITLEN_EN_MASK) != 0
    }

    /// Set SPI_SLV_RDDMA_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_slv_rddma_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SLV_RDDMA_BITLEN_EN_MASK);
    }

    /// Clear SPI_SLV_RDDMA_BITLEN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_slv_rddma_bitlen_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SLV_RDDMA_BITLEN_EN_MASK);
    }

    /// **SPI_RSCK_DATA_OUT** `[3]` (RW)
    ///
    /// It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge
    #[inline(always)]
    pub fn spi_rsck_data_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_RSCK_DATA_OUT_MASK) != 0
    }

    /// Set SPI_RSCK_DATA_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_rsck_data_out(&self) {
        self.value.modify_bits(|r| r | Self::SPI_RSCK_DATA_OUT_MASK);
    }

    /// Clear SPI_RSCK_DATA_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_rsck_data_out(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_RSCK_DATA_OUT_MASK);
    }

    /// **SPI_CLK_MODE_13** `[2]` (RW)
    ///
    /// {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B[0]/B[7]. 0: support spi clk mode 0 and 2, first edge output data B[1]/B[6].
    #[inline(always)]
    pub fn spi_clk_mode_13(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CLK_MODE_13_MASK) != 0
    }

    /// Set SPI_CLK_MODE_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_clk_mode_13(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CLK_MODE_13_MASK);
    }

    /// Clear SPI_CLK_MODE_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_clk_mode_13(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CLK_MODE_13_MASK);
    }

    /// **SPI_CLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
    #[inline(always)]
    pub fn spi_clk_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CLK_MODE_MASK) >> Self::SPI_CLK_MODE_OFFSET
    }

    /// Write **SPI_CLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
    #[inline(always)]
    pub fn write_spi_clk_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CLK_MODE_MASK)
                | ((value << Self::SPI_CLK_MODE_OFFSET) & Self::SPI_CLK_MODE_MASK)
        });
    }
}

/// SPI_SLAVE value type for batch read/modify/write operations
///
/// Use `spislave.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spislave.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSlaveVal {
    bits: u32,
}

impl SpiSlaveVal {
    /// Create from raw bits
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
        Self { bits: 0x02800000 }
    }

    /// Read **SPI_MST_FD_WAIT_DMA_TX_DATA** `[29:29]`
    ///
    /// In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer.
    #[inline(always)]
    pub const fn spi_mst_fd_wait_dma_tx_data(&self) -> bool {
        (self.bits & SpiSlave::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK) != 0
    }

    /// Read **SPI_USR_CONF** `[28:28]`
    ///
    /// 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode.
    #[inline(always)]
    pub const fn spi_usr_conf(&self) -> bool {
        (self.bits & SpiSlave::SPI_USR_CONF_MASK) != 0
    }

    /// Read **MODE** `[26:26]`
    ///
    /// Set SPI work mode. 1: slave mode 0: master mode.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & SpiSlave::MODE_MASK) != 0
    }

    /// Read **SPI_DMA_SEG_MAGIC_VALUE** `[25:22]`
    ///
    /// The magic value of BM table in master DMA seg-trans.
    #[inline(always)]
    pub const fn spi_dma_seg_magic_value(&self) -> u32 {
        (self.bits & SpiSlave::SPI_DMA_SEG_MAGIC_VALUE_MASK)
            >> SpiSlave::SPI_DMA_SEG_MAGIC_VALUE_OFFSET
    }

    /// Read **SPI_SLV_LAST_BYTE_STRB** `[19:12]`
    ///
    /// Represents the effective bit of the last received data byte in SPI slave FD and HD mode.
    #[inline(always)]
    pub const fn spi_slv_last_byte_strb(&self) -> u32 {
        (self.bits & SpiSlave::SPI_SLV_LAST_BYTE_STRB_MASK)
            >> SpiSlave::SPI_SLV_LAST_BYTE_STRB_OFFSET
    }

    /// Read **SPI_SLV_WRBUF_BITLEN_EN** `[11:11]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others
    #[inline(always)]
    pub const fn spi_slv_wrbuf_bitlen_en(&self) -> bool {
        (self.bits & SpiSlave::SPI_SLV_WRBUF_BITLEN_EN_MASK) != 0
    }

    /// Read **SPI_SLV_RDBUF_BITLEN_EN** `[10:10]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others
    #[inline(always)]
    pub const fn spi_slv_rdbuf_bitlen_en(&self) -> bool {
        (self.bits & SpiSlave::SPI_SLV_RDBUF_BITLEN_EN_MASK) != 0
    }

    /// Read **SPI_SLV_WRDMA_BITLEN_EN** `[9:9]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others
    #[inline(always)]
    pub const fn spi_slv_wrdma_bitlen_en(&self) -> bool {
        (self.bits & SpiSlave::SPI_SLV_WRDMA_BITLEN_EN_MASK) != 0
    }

    /// Read **SPI_SLV_RDDMA_BITLEN_EN** `[8:8]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others
    #[inline(always)]
    pub const fn spi_slv_rddma_bitlen_en(&self) -> bool {
        (self.bits & SpiSlave::SPI_SLV_RDDMA_BITLEN_EN_MASK) != 0
    }

    /// Read **SPI_RSCK_DATA_OUT** `[3:3]`
    ///
    /// It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge
    #[inline(always)]
    pub const fn spi_rsck_data_out(&self) -> bool {
        (self.bits & SpiSlave::SPI_RSCK_DATA_OUT_MASK) != 0
    }

    /// Read **SPI_CLK_MODE_13** `[2:2]`
    ///
    /// {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B[0]/B[7]. 0: support spi clk mode 0 and 2, first edge output data B[1]/B[6].
    #[inline(always)]
    pub const fn spi_clk_mode_13(&self) -> bool {
        (self.bits & SpiSlave::SPI_CLK_MODE_13_MASK) != 0
    }

    /// Read **SPI_CLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
    #[inline(always)]
    pub const fn spi_clk_mode(&self) -> u32 {
        (self.bits & SpiSlave::SPI_CLK_MODE_MASK) >> SpiSlave::SPI_CLK_MODE_OFFSET
    }

    /// Set **SPI_MST_FD_WAIT_DMA_TX_DATA** `[29:29]`
    ///
    /// In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer.
    #[inline(always)]
    pub const fn with_spi_mst_fd_wait_dma_tx_data(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK
            } else {
                self.bits & !SpiSlave::SPI_MST_FD_WAIT_DMA_TX_DATA_MASK
            },
        }
    }

    /// Set **SPI_USR_CONF** `[28:28]`
    ///
    /// 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode.
    #[inline(always)]
    pub const fn with_spi_usr_conf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_USR_CONF_MASK
            } else {
                self.bits & !SpiSlave::SPI_USR_CONF_MASK
            },
        }
    }

    /// Set **SPI_SOFT_RESET** `[27:27]`
    ///
    /// Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_soft_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_SOFT_RESET_MASK
            } else {
                self.bits & !SpiSlave::SPI_SOFT_RESET_MASK
            },
        }
    }

    /// Set **MODE** `[26:26]`
    ///
    /// Set SPI work mode. 1: slave mode 0: master mode.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::MODE_MASK
            } else {
                self.bits & !SpiSlave::MODE_MASK
            },
        }
    }

    /// Set **SPI_DMA_SEG_MAGIC_VALUE** `[25:22]`
    ///
    /// The magic value of BM table in master DMA seg-trans.
    #[inline(always)]
    pub const fn with_spi_dma_seg_magic_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSlave::SPI_DMA_SEG_MAGIC_VALUE_MASK)
                | ((value << SpiSlave::SPI_DMA_SEG_MAGIC_VALUE_OFFSET)
                    & SpiSlave::SPI_DMA_SEG_MAGIC_VALUE_MASK),
        }
    }

    /// Set **SPI_SLV_WRBUF_BITLEN_EN** `[11:11]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others
    #[inline(always)]
    pub const fn with_spi_slv_wrbuf_bitlen_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_SLV_WRBUF_BITLEN_EN_MASK
            } else {
                self.bits & !SpiSlave::SPI_SLV_WRBUF_BITLEN_EN_MASK
            },
        }
    }

    /// Set **SPI_SLV_RDBUF_BITLEN_EN** `[10:10]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others
    #[inline(always)]
    pub const fn with_spi_slv_rdbuf_bitlen_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_SLV_RDBUF_BITLEN_EN_MASK
            } else {
                self.bits & !SpiSlave::SPI_SLV_RDBUF_BITLEN_EN_MASK
            },
        }
    }

    /// Set **SPI_SLV_WRDMA_BITLEN_EN** `[9:9]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others
    #[inline(always)]
    pub const fn with_spi_slv_wrdma_bitlen_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_SLV_WRDMA_BITLEN_EN_MASK
            } else {
                self.bits & !SpiSlave::SPI_SLV_WRDMA_BITLEN_EN_MASK
            },
        }
    }

    /// Set **SPI_SLV_RDDMA_BITLEN_EN** `[8:8]`
    ///
    /// 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others
    #[inline(always)]
    pub const fn with_spi_slv_rddma_bitlen_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_SLV_RDDMA_BITLEN_EN_MASK
            } else {
                self.bits & !SpiSlave::SPI_SLV_RDDMA_BITLEN_EN_MASK
            },
        }
    }

    /// Set **SPI_RSCK_DATA_OUT** `[3:3]`
    ///
    /// It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge
    #[inline(always)]
    pub const fn with_spi_rsck_data_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_RSCK_DATA_OUT_MASK
            } else {
                self.bits & !SpiSlave::SPI_RSCK_DATA_OUT_MASK
            },
        }
    }

    /// Set **SPI_CLK_MODE_13** `[2:2]`
    ///
    /// {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B[0]/B[7]. 0: support spi clk mode 0 and 2, first edge output data B[1]/B[6].
    #[inline(always)]
    pub const fn with_spi_clk_mode_13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSlave::SPI_CLK_MODE_13_MASK
            } else {
                self.bits & !SpiSlave::SPI_CLK_MODE_13_MASK
            },
        }
    }

    /// Set **SPI_CLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state.
    #[inline(always)]
    pub const fn with_spi_clk_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSlave::SPI_CLK_MODE_MASK)
                | ((value << SpiSlave::SPI_CLK_MODE_OFFSET) & SpiSlave::SPI_CLK_MODE_MASK),
        }
    }
}

impl Default for SpiSlaveVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSlave {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSlaveVal {
        SpiSlaveVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSlaveVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSlaveVal) -> SpiSlaveVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SLAVE1 Register
///
/// SPI slave control register 1
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **SPI_SLV_LAST_ADDR** `[31:26]` (RW)
///
///   In the slave mode it is the value of address.
///
/// - **SPI_SLV_LAST_COMMAND** `[25:18]` (RW)
///
///   In the slave mode it is the value of command.
///
/// - **SPI_SLV_DATA_BITLEN** `[17:0]` (RW)
///
///   The transferred data bit length in SPI slave FD and HD mode.
#[repr(transparent)]
pub struct SpiSlave1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSlave1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSlave1 {
    // ======== Field Constants ========

    /// **SPI_SLV_LAST_ADDR**
    ///
    /// In the slave mode it is the value of address.
    ///
    /// Bits `[31:26]`, mask `0xfc000000`
    pub const SPI_SLV_LAST_ADDR_OFFSET: u32 = 26;
    pub const SPI_SLV_LAST_ADDR_MASK: u32 = 0xfc000000;

    /// **SPI_SLV_LAST_COMMAND**
    ///
    /// In the slave mode it is the value of command.
    ///
    /// Bits `[25:18]`, mask `0x03fc0000`
    pub const SPI_SLV_LAST_COMMAND_OFFSET: u32 = 18;
    pub const SPI_SLV_LAST_COMMAND_MASK: u32 = 0x03fc0000;

    /// **SPI_SLV_DATA_BITLEN**
    ///
    /// The transferred data bit length in SPI slave FD and HD mode.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const SPI_SLV_DATA_BITLEN_OFFSET: u32 = 0;
    pub const SPI_SLV_DATA_BITLEN_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **SPI_SLV_LAST_ADDR** `[31:26]` (RW)
    ///
    /// In the slave mode it is the value of address.
    #[inline(always)]
    pub fn spi_slv_last_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SLV_LAST_ADDR_MASK) >> Self::SPI_SLV_LAST_ADDR_OFFSET
    }

    /// Write **SPI_SLV_LAST_ADDR** `[31:26]` (RW)
    ///
    /// In the slave mode it is the value of address.
    #[inline(always)]
    pub fn write_spi_slv_last_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SLV_LAST_ADDR_MASK)
                | ((value << Self::SPI_SLV_LAST_ADDR_OFFSET) & Self::SPI_SLV_LAST_ADDR_MASK)
        });
    }

    /// **SPI_SLV_LAST_COMMAND** `[25:18]` (RW)
    ///
    /// In the slave mode it is the value of command.
    #[inline(always)]
    pub fn spi_slv_last_command(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SLV_LAST_COMMAND_MASK)
            >> Self::SPI_SLV_LAST_COMMAND_OFFSET
    }

    /// Write **SPI_SLV_LAST_COMMAND** `[25:18]` (RW)
    ///
    /// In the slave mode it is the value of command.
    #[inline(always)]
    pub fn write_spi_slv_last_command(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SLV_LAST_COMMAND_MASK)
                | ((value << Self::SPI_SLV_LAST_COMMAND_OFFSET) & Self::SPI_SLV_LAST_COMMAND_MASK)
        });
    }

    /// **SPI_SLV_DATA_BITLEN** `[17:0]` (RW)
    ///
    /// The transferred data bit length in SPI slave FD and HD mode.
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SLV_DATA_BITLEN_MASK)
            >> Self::SPI_SLV_DATA_BITLEN_OFFSET
    }

    /// Write **SPI_SLV_DATA_BITLEN** `[17:0]` (RW)
    ///
    /// The transferred data bit length in SPI slave FD and HD mode.
    #[inline(always)]
    pub fn write_spi_slv_data_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SLV_DATA_BITLEN_MASK)
                | ((value << Self::SPI_SLV_DATA_BITLEN_OFFSET) & Self::SPI_SLV_DATA_BITLEN_MASK)
        });
    }
}

/// SPI_SLAVE1 value type for batch read/modify/write operations
///
/// Use `spislave1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spislave1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSlave1Val {
    bits: u32,
}

impl SpiSlave1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SLV_LAST_ADDR** `[31:26]`
    ///
    /// In the slave mode it is the value of address.
    #[inline(always)]
    pub const fn spi_slv_last_addr(&self) -> u32 {
        (self.bits & SpiSlave1::SPI_SLV_LAST_ADDR_MASK) >> SpiSlave1::SPI_SLV_LAST_ADDR_OFFSET
    }

    /// Read **SPI_SLV_LAST_COMMAND** `[25:18]`
    ///
    /// In the slave mode it is the value of command.
    #[inline(always)]
    pub const fn spi_slv_last_command(&self) -> u32 {
        (self.bits & SpiSlave1::SPI_SLV_LAST_COMMAND_MASK) >> SpiSlave1::SPI_SLV_LAST_COMMAND_OFFSET
    }

    /// Read **SPI_SLV_DATA_BITLEN** `[17:0]`
    ///
    /// The transferred data bit length in SPI slave FD and HD mode.
    #[inline(always)]
    pub const fn spi_slv_data_bitlen(&self) -> u32 {
        (self.bits & SpiSlave1::SPI_SLV_DATA_BITLEN_MASK) >> SpiSlave1::SPI_SLV_DATA_BITLEN_OFFSET
    }

    /// Set **SPI_SLV_LAST_ADDR** `[31:26]`
    ///
    /// In the slave mode it is the value of address.
    #[inline(always)]
    pub const fn with_spi_slv_last_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSlave1::SPI_SLV_LAST_ADDR_MASK)
                | ((value << SpiSlave1::SPI_SLV_LAST_ADDR_OFFSET)
                    & SpiSlave1::SPI_SLV_LAST_ADDR_MASK),
        }
    }

    /// Set **SPI_SLV_LAST_COMMAND** `[25:18]`
    ///
    /// In the slave mode it is the value of command.
    #[inline(always)]
    pub const fn with_spi_slv_last_command(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSlave1::SPI_SLV_LAST_COMMAND_MASK)
                | ((value << SpiSlave1::SPI_SLV_LAST_COMMAND_OFFSET)
                    & SpiSlave1::SPI_SLV_LAST_COMMAND_MASK),
        }
    }

    /// Set **SPI_SLV_DATA_BITLEN** `[17:0]`
    ///
    /// The transferred data bit length in SPI slave FD and HD mode.
    #[inline(always)]
    pub const fn with_spi_slv_data_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSlave1::SPI_SLV_DATA_BITLEN_MASK)
                | ((value << SpiSlave1::SPI_SLV_DATA_BITLEN_OFFSET)
                    & SpiSlave1::SPI_SLV_DATA_BITLEN_MASK),
        }
    }
}

impl Default for SpiSlave1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSlave1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSlave1Val {
        SpiSlave1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSlave1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSlave1Val) -> SpiSlave1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_CLK_GATE Register
///
/// SPI module clock and register clock control
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **SPI_MST_CLK_SEL** `[2]` (RW)
///
///   This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK.
///
/// - **SPI_MST_CLK_ACTIVE** `[1]` (RW)
///
///   Set this bit to power on the SPI module clock.
///
/// - **SPI_CLK_EN** `[0]` (RW)
///
///   Set this bit to enable clk gate
#[repr(transparent)]
pub struct SpiClkGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiClkGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiClkGate {
    // ======== Field Constants ========

    /// **SPI_MST_CLK_SEL**
    ///
    /// This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MST_CLK_SEL_OFFSET: u32 = 2;
    pub const SPI_MST_CLK_SEL_MASK: u32 = 0x00000004;
    pub const SPI_MST_CLK_SEL: u32 = 1 << 2;

    /// **SPI_MST_CLK_ACTIVE**
    ///
    /// Set this bit to power on the SPI module clock.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MST_CLK_ACTIVE_OFFSET: u32 = 1;
    pub const SPI_MST_CLK_ACTIVE_MASK: u32 = 0x00000002;
    pub const SPI_MST_CLK_ACTIVE: u32 = 1 << 1;

    /// **SPI_CLK_EN**
    ///
    /// Set this bit to enable clk gate
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_CLK_EN_OFFSET: u32 = 0;
    pub const SPI_CLK_EN_MASK: u32 = 0x00000001;
    pub const SPI_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MST_CLK_SEL** `[2]` (RW)
    ///
    /// This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK.
    #[inline(always)]
    pub fn spi_mst_clk_sel(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_CLK_SEL_MASK) != 0
    }

    /// Set SPI_MST_CLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_clk_sel(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MST_CLK_SEL_MASK);
    }

    /// Clear SPI_MST_CLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_clk_sel(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MST_CLK_SEL_MASK);
    }

    /// **SPI_MST_CLK_ACTIVE** `[1]` (RW)
    ///
    /// Set this bit to power on the SPI module clock.
    #[inline(always)]
    pub fn spi_mst_clk_active(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MST_CLK_ACTIVE_MASK) != 0
    }

    /// Set SPI_MST_CLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mst_clk_active(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MST_CLK_ACTIVE_MASK);
    }

    /// Clear SPI_MST_CLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mst_clk_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MST_CLK_ACTIVE_MASK);
    }

    /// **SPI_CLK_EN** `[0]` (RW)
    ///
    /// Set this bit to enable clk gate
    #[inline(always)]
    pub fn spi_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CLK_EN_MASK) != 0
    }

    /// Set SPI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_CLK_EN_MASK);
    }

    /// Clear SPI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_CLK_EN_MASK);
    }
}

/// SPI_CLK_GATE value type for batch read/modify/write operations
///
/// Use `spiclkgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spiclkgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiClkGateVal {
    bits: u32,
}

impl SpiClkGateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MST_CLK_SEL** `[2:2]`
    ///
    /// This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK.
    #[inline(always)]
    pub const fn spi_mst_clk_sel(&self) -> bool {
        (self.bits & SpiClkGate::SPI_MST_CLK_SEL_MASK) != 0
    }

    /// Read **SPI_MST_CLK_ACTIVE** `[1:1]`
    ///
    /// Set this bit to power on the SPI module clock.
    #[inline(always)]
    pub const fn spi_mst_clk_active(&self) -> bool {
        (self.bits & SpiClkGate::SPI_MST_CLK_ACTIVE_MASK) != 0
    }

    /// Read **SPI_CLK_EN** `[0:0]`
    ///
    /// Set this bit to enable clk gate
    #[inline(always)]
    pub const fn spi_clk_en(&self) -> bool {
        (self.bits & SpiClkGate::SPI_CLK_EN_MASK) != 0
    }

    /// Set **SPI_MST_CLK_SEL** `[2:2]`
    ///
    /// This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK.
    #[inline(always)]
    pub const fn with_spi_mst_clk_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiClkGate::SPI_MST_CLK_SEL_MASK
            } else {
                self.bits & !SpiClkGate::SPI_MST_CLK_SEL_MASK
            },
        }
    }

    /// Set **SPI_MST_CLK_ACTIVE** `[1:1]`
    ///
    /// Set this bit to power on the SPI module clock.
    #[inline(always)]
    pub const fn with_spi_mst_clk_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiClkGate::SPI_MST_CLK_ACTIVE_MASK
            } else {
                self.bits & !SpiClkGate::SPI_MST_CLK_ACTIVE_MASK
            },
        }
    }

    /// Set **SPI_CLK_EN** `[0:0]`
    ///
    /// Set this bit to enable clk gate
    #[inline(always)]
    pub const fn with_spi_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiClkGate::SPI_CLK_EN_MASK
            } else {
                self.bits & !SpiClkGate::SPI_CLK_EN_MASK
            },
        }
    }
}

impl Default for SpiClkGateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiClkGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiClkGateVal {
        SpiClkGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiClkGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiClkGateVal) -> SpiClkGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_DATE Register
///
/// Version control
///
/// Address offset: 0x00f0
/// Reset value: 0x02207202
///
/// ## Fields
///
/// - **SPI_DATE** `[27:0]` (RW)
///
///   SPI register version.
#[repr(transparent)]
pub struct SpiDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiDate {
    // ======== Field Constants ========

    /// **SPI_DATE**
    ///
    /// SPI register version.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const SPI_DATE_OFFSET: u32 = 0;
    pub const SPI_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **SPI_DATE** `[27:0]` (RW)
    ///
    /// SPI register version.
    #[inline(always)]
    pub fn spi_date(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_DATE_MASK) >> Self::SPI_DATE_OFFSET
    }

    /// Write **SPI_DATE** `[27:0]` (RW)
    ///
    /// SPI register version.
    #[inline(always)]
    pub fn write_spi_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_DATE_MASK) | ((value << Self::SPI_DATE_OFFSET) & Self::SPI_DATE_MASK)
        });
    }
}

/// SPI_DATE value type for batch read/modify/write operations
///
/// Use `spidate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spidate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiDateVal {
    bits: u32,
}

impl SpiDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02207202 }
    }

    /// Read **SPI_DATE** `[27:0]`
    ///
    /// SPI register version.
    #[inline(always)]
    pub const fn spi_date(&self) -> u32 {
        (self.bits & SpiDate::SPI_DATE_MASK) >> SpiDate::SPI_DATE_OFFSET
    }

    /// Set **SPI_DATE** `[27:0]`
    ///
    /// SPI register version.
    #[inline(always)]
    pub const fn with_spi_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiDate::SPI_DATE_MASK)
                | ((value << SpiDate::SPI_DATE_OFFSET) & SpiDate::SPI_DATE_MASK),
        }
    }
}

impl Default for SpiDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiDateVal {
        SpiDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiDateVal) -> SpiDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Command control register
    pub spi_cmd: SpiCmd,
    /// 0x0004
    /// Address value register
    pub spi_addr: SpiAddr,
    /// 0x0008
    /// SPI control register
    pub spi_ctrl: SpiCtrl,
    /// 0x000c
    /// SPI clock control register
    pub spi_clock: SpiClock,
    /// 0x0010
    /// SPI USER control register
    pub spi_user: SpiUser,
    /// 0x0014
    /// SPI USER control register 1
    pub spi_user1: SpiUser1,
    /// 0x0018
    /// SPI USER control register 2
    pub spi_user2: SpiUser2,
    /// 0x001c
    /// SPI data bit length control register
    pub spi_ms_dlen: SpiMsDlen,
    /// 0x0020
    /// SPI misc register
    pub spi_misc: SpiMisc,
    /// 0x0024
    /// SPI input delay mode configuration
    pub spi_din_mode: SpiDinMode,
    /// 0x0028
    /// SPI input delay number configuration
    pub spi_din_num: SpiDinNum,
    /// 0x002c
    /// SPI output delay mode configuration
    pub spi_dout_mode: SpiDoutMode,
    /// 0x0030
    /// SPI DMA control register
    pub spi_dma_conf: SpiDmaConf,
    /// 0x0034
    /// SPI interrupt enable register
    pub spi_dma_int_ena: SpiDmaIntEna,
    /// 0x0038
    /// SPI interrupt clear register
    pub spi_dma_int_clr: SpiDmaIntClr,
    /// 0x003c
    /// SPI interrupt raw register
    pub spi_dma_int_raw: SpiDmaIntRaw,
    /// 0x0040
    /// SPI interrupt status register
    pub spi_dma_int_st: SpiDmaIntSt,
    /// 0x0044
    /// SPI interrupt software set register
    pub spi_dma_int_set: SpiDmaIntSet,
    _reserved0: [u8; 80],
    /// 0x0098
    /// SPI CPU-controlled buffer0
    pub spi_w0: SpiW0,
    /// 0x009c
    /// SPI CPU-controlled buffer1
    pub spi_w1: SpiW1,
    /// 0x00a0
    /// SPI CPU-controlled buffer2
    pub spi_w2: SpiW2,
    /// 0x00a4
    /// SPI CPU-controlled buffer3
    pub spi_w3: SpiW3,
    /// 0x00a8
    /// SPI CPU-controlled buffer4
    pub spi_w4: SpiW4,
    /// 0x00ac
    /// SPI CPU-controlled buffer5
    pub spi_w5: SpiW5,
    /// 0x00b0
    /// SPI CPU-controlled buffer6
    pub spi_w6: SpiW6,
    /// 0x00b4
    /// SPI CPU-controlled buffer7
    pub spi_w7: SpiW7,
    /// 0x00b8
    /// SPI CPU-controlled buffer8
    pub spi_w8: SpiW8,
    /// 0x00bc
    /// SPI CPU-controlled buffer9
    pub spi_w9: SpiW9,
    /// 0x00c0
    /// SPI CPU-controlled buffer10
    pub spi_w10: SpiW10,
    /// 0x00c4
    /// SPI CPU-controlled buffer11
    pub spi_w11: SpiW11,
    /// 0x00c8
    /// SPI CPU-controlled buffer12
    pub spi_w12: SpiW12,
    /// 0x00cc
    /// SPI CPU-controlled buffer13
    pub spi_w13: SpiW13,
    /// 0x00d0
    /// SPI CPU-controlled buffer14
    pub spi_w14: SpiW14,
    /// 0x00d4
    /// SPI CPU-controlled buffer15
    pub spi_w15: SpiW15,
    _reserved1: [u8; 8],
    /// 0x00e0
    /// SPI slave control register
    pub spi_slave: SpiSlave,
    /// 0x00e4
    /// SPI slave control register 1
    pub spi_slave1: SpiSlave1,
    /// 0x00e8
    /// SPI module clock and register clock control
    pub spi_clk_gate: SpiClkGate,
    _reserved2: [u8; 4],
    /// 0x00f0
    /// Version control
    pub spi_date: SpiDate,
}
