//! PSRAM_MSPI0
//! PSRAM MSPI controller 0 (AXI cache / MMU path)

#![allow(dead_code)]

pub const BASE: u32 = 0x5008e000;

/// SPI_MEM_CMD Register
///
/// SPI0 FSM status register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SPI_MEM_USR** `[18]` (RO)
///
///   SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_SLV_ST** `[7:4]` (RO)
///
///   The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
///
/// - **SPI_MEM_MST_ST** `[3:0]` (RO)
///
///   The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state.
#[repr(transparent)]
pub struct SpiMemCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCmd {
    // ======== Field Constants ========

    /// **SPI_MEM_USR**
    ///
    /// SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MEM_USR_OFFSET: u32 = 18;
    pub const SPI_MEM_USR_MASK: u32 = 0x00040000;
    pub const SPI_MEM_USR: u32 = 1 << 18;

    /// **SPI_MEM_SLV_ST**
    ///
    /// The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const SPI_MEM_SLV_ST_OFFSET: u32 = 4;
    pub const SPI_MEM_SLV_ST_MASK: u32 = 0x000000f0;

    /// **SPI_MEM_MST_ST**
    ///
    /// The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SPI_MEM_MST_ST_OFFSET: u32 = 0;
    pub const SPI_MEM_MST_ST_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR** `[18]` (RO)
    ///
    /// SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_usr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MASK) != 0
    }

    /// **SPI_MEM_SLV_ST** `[7:4]` (RO)
    ///
    /// The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    #[inline(always)]
    pub fn spi_mem_slv_st(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SLV_ST_MASK) >> Self::SPI_MEM_SLV_ST_OFFSET
    }

    /// **SPI_MEM_MST_ST** `[3:0]` (RO)
    ///
    /// The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state.
    #[inline(always)]
    pub fn spi_mem_mst_st(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_MST_ST_MASK) >> Self::SPI_MEM_MST_ST_OFFSET
    }
}

/// SPI_MEM_CMD value type for batch read/modify/write operations
///
/// Use `spimemcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCmdVal {
    bits: u32,
}

impl SpiMemCmdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_USR** `[18:18]`
    ///
    /// SPI0 USR_CMD start bit, only used when SPI_MEM_AXI_REQ_EN is cleared. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_usr(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_USR_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST** `[7:4]`
    ///
    /// The current status of SPI0 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    #[inline(always)]
    pub const fn spi_mem_slv_st(&self) -> u32 {
        (self.bits & SpiMemCmd::SPI_MEM_SLV_ST_MASK) >> SpiMemCmd::SPI_MEM_SLV_ST_OFFSET
    }

    /// Read **SPI_MEM_MST_ST** `[3:0]`
    ///
    /// The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:SPI0_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state.
    #[inline(always)]
    pub const fn spi_mem_mst_st(&self) -> u32 {
        (self.bits & SpiMemCmd::SPI_MEM_MST_ST_MASK) >> SpiMemCmd::SPI_MEM_MST_ST_OFFSET
    }
}

impl Default for SpiMemCmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCmdVal {
        SpiMemCmdVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_CTRL Register
///
/// SPI0 control register.
///
/// Address offset: 0x0008
/// Reset value: 0x802c200c
///
/// ## Fields
///
/// - **SPI_MEM_DATA_IE_ALWAYS_ON** `[31]` (RW)
///
///   When accesses to flash, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
///
/// - **SPI_MEM_DQS_IE_ALWAYS_ON** `[30]` (RW)
///
///   When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
///
/// - **SPI_MEM_FREAD_QIO** `[24]` (RW)
///
///   In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_FREAD_DIO** `[23]` (RW)
///
///   In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_WP** `[21]` (RW)
///
///   Write protect signal output when SPI is idle. 1: output high, 0: output low.
///
/// - **SPI_MEM_FREAD_QUAD** `[20]` (RW)
///
///   In the read operations read-data phase apply 4 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_D_POL** `[19]` (RW)
///
///   The bit is used to set MOSI line polarity, 1: high 0, low
///
/// - **SPI_MEM_Q_POL** `[18]` (RW)
///
///   The bit is used to set MISO line polarity, 1: high 0, low
///
/// - **SPI_MEM_FREAD_DUAL** `[14]` (RW)
///
///   In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_FASTRD_MODE** `[13]` (RW)
///
///   This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable.
///
/// - **SPI_MEM_FCMD_OCT** `[9]` (RW)
///
///   Apply 8 signals during command phase 1:enable 0: disable
///
/// - **SPI_MEM_FCMD_QUAD** `[8]` (RW)
///
///   Apply 4 signals during command phase 1:enable 0: disable
///
/// - **SPI_MEM_FADDR_OCT** `[6]` (RW)
///
///   Apply 8 signals during address phase 1:enable 0: disable
///
/// - **SPI_MEM_FDIN_OCT** `[5]` (RW)
///
///   Apply 8 signals during read-data phase 1:enable 0: disable
///
/// - **SPI_MEM_FDOUT_OCT** `[4]` (RW)
///
///   Apply 8 signals during write-data phase 1:enable 0: disable
///
/// - **SPI_MEM_FDUMMY_WOUT** `[3]` (RW)
///
///   In an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash.
///
/// - **SPI_MEM_FDUMMY_RIN** `[2]` (RW)
///
///   In an MSPI read data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase.
///
/// - **SPI_MEM_WDUMMY_ALWAYS_OUT** `[1]` (RW)
///
///   In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller.
///
/// - **SPI_MEM_WDUMMY_DQS_ALWAYS_OUT** `[0]` (RW)
///
///   In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller.
#[repr(transparent)]
pub struct SpiMemCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCtrl {
    // ======== Field Constants ========

    /// **SPI_MEM_DATA_IE_ALWAYS_ON**
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_DATA_IE_ALWAYS_ON_OFFSET: u32 = 31;
    pub const SPI_MEM_DATA_IE_ALWAYS_ON_MASK: u32 = 0x80000000;
    pub const SPI_MEM_DATA_IE_ALWAYS_ON: u32 = 1 << 31;

    /// **SPI_MEM_DQS_IE_ALWAYS_ON**
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_DQS_IE_ALWAYS_ON_OFFSET: u32 = 30;
    pub const SPI_MEM_DQS_IE_ALWAYS_ON_MASK: u32 = 0x40000000;
    pub const SPI_MEM_DQS_IE_ALWAYS_ON: u32 = 1 << 30;

    /// **SPI_MEM_FREAD_QIO**
    ///
    /// In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_FREAD_QIO_OFFSET: u32 = 24;
    pub const SPI_MEM_FREAD_QIO_MASK: u32 = 0x01000000;
    pub const SPI_MEM_FREAD_QIO: u32 = 1 << 24;

    /// **SPI_MEM_FREAD_DIO**
    ///
    /// In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_MEM_FREAD_DIO_OFFSET: u32 = 23;
    pub const SPI_MEM_FREAD_DIO_MASK: u32 = 0x00800000;
    pub const SPI_MEM_FREAD_DIO: u32 = 1 << 23;

    /// **SPI_MEM_WP**
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_MEM_WP_OFFSET: u32 = 21;
    pub const SPI_MEM_WP_MASK: u32 = 0x00200000;
    pub const SPI_MEM_WP: u32 = 1 << 21;

    /// **SPI_MEM_FREAD_QUAD**
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_MEM_FREAD_QUAD_OFFSET: u32 = 20;
    pub const SPI_MEM_FREAD_QUAD_MASK: u32 = 0x00100000;
    pub const SPI_MEM_FREAD_QUAD: u32 = 1 << 20;

    /// **SPI_MEM_D_POL**
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_MEM_D_POL_OFFSET: u32 = 19;
    pub const SPI_MEM_D_POL_MASK: u32 = 0x00080000;
    pub const SPI_MEM_D_POL: u32 = 1 << 19;

    /// **SPI_MEM_Q_POL**
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MEM_Q_POL_OFFSET: u32 = 18;
    pub const SPI_MEM_Q_POL_MASK: u32 = 0x00040000;
    pub const SPI_MEM_Q_POL: u32 = 1 << 18;

    /// **SPI_MEM_FREAD_DUAL**
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_MEM_FREAD_DUAL_OFFSET: u32 = 14;
    pub const SPI_MEM_FREAD_DUAL_MASK: u32 = 0x00004000;
    pub const SPI_MEM_FREAD_DUAL: u32 = 1 << 14;

    /// **SPI_MEM_FASTRD_MODE**
    ///
    /// This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_MEM_FASTRD_MODE_OFFSET: u32 = 13;
    pub const SPI_MEM_FASTRD_MODE_MASK: u32 = 0x00002000;
    pub const SPI_MEM_FASTRD_MODE: u32 = 1 << 13;

    /// **SPI_MEM_FCMD_OCT**
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_FCMD_OCT_OFFSET: u32 = 9;
    pub const SPI_MEM_FCMD_OCT_MASK: u32 = 0x00000200;
    pub const SPI_MEM_FCMD_OCT: u32 = 1 << 9;

    /// **SPI_MEM_FCMD_QUAD**
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_FCMD_QUAD_OFFSET: u32 = 8;
    pub const SPI_MEM_FCMD_QUAD_MASK: u32 = 0x00000100;
    pub const SPI_MEM_FCMD_QUAD: u32 = 1 << 8;

    /// **SPI_MEM_FADDR_OCT**
    ///
    /// Apply 8 signals during address phase 1:enable 0: disable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_FADDR_OCT_OFFSET: u32 = 6;
    pub const SPI_MEM_FADDR_OCT_MASK: u32 = 0x00000040;
    pub const SPI_MEM_FADDR_OCT: u32 = 1 << 6;

    /// **SPI_MEM_FDIN_OCT**
    ///
    /// Apply 8 signals during read-data phase 1:enable 0: disable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_FDIN_OCT_OFFSET: u32 = 5;
    pub const SPI_MEM_FDIN_OCT_MASK: u32 = 0x00000020;
    pub const SPI_MEM_FDIN_OCT: u32 = 1 << 5;

    /// **SPI_MEM_FDOUT_OCT**
    ///
    /// Apply 8 signals during write-data phase 1:enable 0: disable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_FDOUT_OCT_OFFSET: u32 = 4;
    pub const SPI_MEM_FDOUT_OCT_MASK: u32 = 0x00000010;
    pub const SPI_MEM_FDOUT_OCT: u32 = 1 << 4;

    /// **SPI_MEM_FDUMMY_WOUT**
    ///
    /// In an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FDUMMY_WOUT_OFFSET: u32 = 3;
    pub const SPI_MEM_FDUMMY_WOUT_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FDUMMY_WOUT: u32 = 1 << 3;

    /// **SPI_MEM_FDUMMY_RIN**
    ///
    /// In an MSPI read data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_FDUMMY_RIN_OFFSET: u32 = 2;
    pub const SPI_MEM_FDUMMY_RIN_MASK: u32 = 0x00000004;
    pub const SPI_MEM_FDUMMY_RIN: u32 = 1 << 2;

    /// **SPI_MEM_WDUMMY_ALWAYS_OUT**
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_WDUMMY_ALWAYS_OUT_OFFSET: u32 = 1;
    pub const SPI_MEM_WDUMMY_ALWAYS_OUT_MASK: u32 = 0x00000002;
    pub const SPI_MEM_WDUMMY_ALWAYS_OUT: u32 = 1 << 1;

    /// **SPI_MEM_WDUMMY_DQS_ALWAYS_OUT**
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_OFFSET: u32 = 0;
    pub const SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK: u32 = 0x00000001;
    pub const SPI_MEM_WDUMMY_DQS_ALWAYS_OUT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_DATA_IE_ALWAYS_ON** `[31]` (RW)
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub fn spi_mem_data_ie_always_on(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DATA_IE_ALWAYS_ON_MASK) != 0
    }

    /// Set SPI_MEM_DATA_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_data_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DATA_IE_ALWAYS_ON_MASK);
    }

    /// Clear SPI_MEM_DATA_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_data_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DATA_IE_ALWAYS_ON_MASK);
    }

    /// **SPI_MEM_DQS_IE_ALWAYS_ON** `[30]` (RW)
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub fn spi_mem_dqs_ie_always_on(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS_IE_ALWAYS_ON_MASK) != 0
    }

    /// Set SPI_MEM_DQS_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dqs_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DQS_IE_ALWAYS_ON_MASK);
    }

    /// Clear SPI_MEM_DQS_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dqs_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DQS_IE_ALWAYS_ON_MASK);
    }

    /// **SPI_MEM_FREAD_QIO** `[24]` (RW)
    ///
    /// In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_fread_qio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FREAD_QIO_MASK) != 0
    }

    /// Set SPI_MEM_FREAD_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fread_qio(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FREAD_QIO_MASK);
    }

    /// Clear SPI_MEM_FREAD_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fread_qio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FREAD_QIO_MASK);
    }

    /// **SPI_MEM_FREAD_DIO** `[23]` (RW)
    ///
    /// In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_fread_dio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FREAD_DIO_MASK) != 0
    }

    /// Set SPI_MEM_FREAD_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fread_dio(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FREAD_DIO_MASK);
    }

    /// Clear SPI_MEM_FREAD_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fread_dio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FREAD_DIO_MASK);
    }

    /// **SPI_MEM_WP** `[21]` (RW)
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low.
    #[inline(always)]
    pub fn spi_mem_wp(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WP_MASK) != 0
    }

    /// Set SPI_MEM_WP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wp(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_WP_MASK);
    }

    /// Clear SPI_MEM_WP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wp(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_WP_MASK);
    }

    /// **SPI_MEM_FREAD_QUAD** `[20]` (RW)
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_fread_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FREAD_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FREAD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fread_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FREAD_QUAD_MASK);
    }

    /// Clear SPI_MEM_FREAD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fread_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FREAD_QUAD_MASK);
    }

    /// **SPI_MEM_D_POL** `[19]` (RW)
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low
    #[inline(always)]
    pub fn spi_mem_d_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_D_POL_MASK) != 0
    }

    /// Set SPI_MEM_D_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_d_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_D_POL_MASK);
    }

    /// Clear SPI_MEM_D_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_d_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_D_POL_MASK);
    }

    /// **SPI_MEM_Q_POL** `[18]` (RW)
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low
    #[inline(always)]
    pub fn spi_mem_q_pol(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_Q_POL_MASK) != 0
    }

    /// Set SPI_MEM_Q_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_q_pol(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_Q_POL_MASK);
    }

    /// Clear SPI_MEM_Q_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_q_pol(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_Q_POL_MASK);
    }

    /// **SPI_MEM_FREAD_DUAL** `[14]` (RW)
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_fread_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FREAD_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_FREAD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fread_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FREAD_DUAL_MASK);
    }

    /// Clear SPI_MEM_FREAD_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fread_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FREAD_DUAL_MASK);
    }

    /// **SPI_MEM_FASTRD_MODE** `[13]` (RW)
    ///
    /// This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_fastrd_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FASTRD_MODE_MASK) != 0
    }

    /// Set SPI_MEM_FASTRD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fastrd_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FASTRD_MODE_MASK);
    }

    /// Clear SPI_MEM_FASTRD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fastrd_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FASTRD_MODE_MASK);
    }

    /// **SPI_MEM_FCMD_OCT** `[9]` (RW)
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub fn spi_mem_fcmd_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FCMD_OCT_MASK) != 0
    }

    /// Set SPI_MEM_FCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fcmd_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FCMD_OCT_MASK);
    }

    /// Clear SPI_MEM_FCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fcmd_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FCMD_OCT_MASK);
    }

    /// **SPI_MEM_FCMD_QUAD** `[8]` (RW)
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub fn spi_mem_fcmd_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FCMD_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fcmd_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FCMD_QUAD_MASK);
    }

    /// Clear SPI_MEM_FCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fcmd_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FCMD_QUAD_MASK);
    }

    /// **SPI_MEM_FADDR_OCT** `[6]` (RW)
    ///
    /// Apply 8 signals during address phase 1:enable 0: disable
    #[inline(always)]
    pub fn spi_mem_faddr_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FADDR_OCT_MASK) != 0
    }

    /// Set SPI_MEM_FADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_faddr_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FADDR_OCT_MASK);
    }

    /// Clear SPI_MEM_FADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_faddr_oct(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FADDR_OCT_MASK);
    }

    /// **SPI_MEM_FDIN_OCT** `[5]` (RW)
    ///
    /// Apply 8 signals during read-data phase 1:enable 0: disable
    #[inline(always)]
    pub fn spi_mem_fdin_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDIN_OCT_MASK) != 0
    }

    /// Set SPI_MEM_FDIN_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdin_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FDIN_OCT_MASK);
    }

    /// Clear SPI_MEM_FDIN_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdin_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FDIN_OCT_MASK);
    }

    /// **SPI_MEM_FDOUT_OCT** `[4]` (RW)
    ///
    /// Apply 8 signals during write-data phase 1:enable 0: disable
    #[inline(always)]
    pub fn spi_mem_fdout_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDOUT_OCT_MASK) != 0
    }

    /// Set SPI_MEM_FDOUT_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdout_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FDOUT_OCT_MASK);
    }

    /// Clear SPI_MEM_FDOUT_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdout_oct(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDOUT_OCT_MASK);
    }

    /// **SPI_MEM_FDUMMY_WOUT** `[3]` (RW)
    ///
    /// In an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash.
    #[inline(always)]
    pub fn spi_mem_fdummy_wout(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDUMMY_WOUT_MASK) != 0
    }

    /// Set SPI_MEM_FDUMMY_WOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdummy_wout(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FDUMMY_WOUT_MASK);
    }

    /// Clear SPI_MEM_FDUMMY_WOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdummy_wout(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDUMMY_WOUT_MASK);
    }

    /// **SPI_MEM_FDUMMY_RIN** `[2]` (RW)
    ///
    /// In an MSPI read data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase.
    #[inline(always)]
    pub fn spi_mem_fdummy_rin(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDUMMY_RIN_MASK) != 0
    }

    /// Set SPI_MEM_FDUMMY_RIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdummy_rin(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FDUMMY_RIN_MASK);
    }

    /// Clear SPI_MEM_FDUMMY_RIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdummy_rin(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDUMMY_RIN_MASK);
    }

    /// **SPI_MEM_WDUMMY_ALWAYS_OUT** `[1]` (RW)
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_mem_wdummy_always_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK) != 0
    }

    /// Set SPI_MEM_WDUMMY_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wdummy_always_out(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK);
    }

    /// Clear SPI_MEM_WDUMMY_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wdummy_always_out(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK);
    }

    /// **SPI_MEM_WDUMMY_DQS_ALWAYS_OUT** `[0]` (RW)
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_mem_wdummy_dqs_always_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK) != 0
    }

    /// Set SPI_MEM_WDUMMY_DQS_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wdummy_dqs_always_out(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK);
    }

    /// Clear SPI_MEM_WDUMMY_DQS_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wdummy_dqs_always_out(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK);
    }
}

/// SPI_MEM_CTRL value type for batch read/modify/write operations
///
/// Use `spimemctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCtrlVal {
    bits: u32,
}

impl SpiMemCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x802c200c }
    }

    /// Read **SPI_MEM_DATA_IE_ALWAYS_ON** `[31:31]`
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_data_ie_always_on(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_DATA_IE_ALWAYS_ON_MASK) != 0
    }

    /// Read **SPI_MEM_DQS_IE_ALWAYS_ON** `[30:30]`
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_dqs_ie_always_on(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_DQS_IE_ALWAYS_ON_MASK) != 0
    }

    /// Read **SPI_MEM_FREAD_QIO** `[24:24]`
    ///
    /// In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fread_qio(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FREAD_QIO_MASK) != 0
    }

    /// Read **SPI_MEM_FREAD_DIO** `[23:23]`
    ///
    /// In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fread_dio(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FREAD_DIO_MASK) != 0
    }

    /// Read **SPI_MEM_WP** `[21:21]`
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low.
    #[inline(always)]
    pub const fn spi_mem_wp(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_WP_MASK) != 0
    }

    /// Read **SPI_MEM_FREAD_QUAD** `[20:20]`
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fread_quad(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FREAD_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_D_POL** `[19:19]`
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low
    #[inline(always)]
    pub const fn spi_mem_d_pol(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_D_POL_MASK) != 0
    }

    /// Read **SPI_MEM_Q_POL** `[18:18]`
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low
    #[inline(always)]
    pub const fn spi_mem_q_pol(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_Q_POL_MASK) != 0
    }

    /// Read **SPI_MEM_FREAD_DUAL** `[14:14]`
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fread_dual(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FREAD_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_FASTRD_MODE** `[13:13]`
    ///
    /// This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fastrd_mode(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FASTRD_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_FCMD_OCT** `[9:9]`
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub const fn spi_mem_fcmd_oct(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FCMD_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_FCMD_QUAD** `[8:8]`
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub const fn spi_mem_fcmd_quad(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FCMD_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FADDR_OCT** `[6:6]`
    ///
    /// Apply 8 signals during address phase 1:enable 0: disable
    #[inline(always)]
    pub const fn spi_mem_faddr_oct(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FADDR_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_FDIN_OCT** `[5:5]`
    ///
    /// Apply 8 signals during read-data phase 1:enable 0: disable
    #[inline(always)]
    pub const fn spi_mem_fdin_oct(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDIN_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_FDOUT_OCT** `[4:4]`
    ///
    /// Apply 8 signals during write-data phase 1:enable 0: disable
    #[inline(always)]
    pub const fn spi_mem_fdout_oct(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDOUT_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_FDUMMY_WOUT** `[3:3]`
    ///
    /// In an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash.
    #[inline(always)]
    pub const fn spi_mem_fdummy_wout(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDUMMY_WOUT_MASK) != 0
    }

    /// Read **SPI_MEM_FDUMMY_RIN** `[2:2]`
    ///
    /// In an MSPI read data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase.
    #[inline(always)]
    pub const fn spi_mem_fdummy_rin(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDUMMY_RIN_MASK) != 0
    }

    /// Read **SPI_MEM_WDUMMY_ALWAYS_OUT** `[1:1]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_wdummy_always_out(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK) != 0
    }

    /// Read **SPI_MEM_WDUMMY_DQS_ALWAYS_OUT** `[0:0]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_wdummy_dqs_always_out(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK) != 0
    }

    /// Set **SPI_MEM_DATA_IE_ALWAYS_ON** `[31:31]`
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_data_ie_always_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_DATA_IE_ALWAYS_ON_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_DATA_IE_ALWAYS_ON_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS_IE_ALWAYS_ON** `[30:30]`
    ///
    /// When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_dqs_ie_always_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_DQS_IE_ALWAYS_ON_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_DQS_IE_ALWAYS_ON_MASK
            },
        }
    }

    /// Set **SPI_MEM_FREAD_QIO** `[24:24]`
    ///
    /// In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_fread_qio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FREAD_QIO_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FREAD_QIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_FREAD_DIO** `[23:23]`
    ///
    /// In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_fread_dio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FREAD_DIO_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FREAD_DIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_WP** `[21:21]`
    ///
    /// Write protect signal output when SPI is idle. 1: output high, 0: output low.
    #[inline(always)]
    pub const fn with_spi_mem_wp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_WP_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_WP_MASK
            },
        }
    }

    /// Set **SPI_MEM_FREAD_QUAD** `[20:20]`
    ///
    /// In the read operations read-data phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_fread_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FREAD_QUAD_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FREAD_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_D_POL** `[19:19]`
    ///
    /// The bit is used to set MOSI line polarity, 1: high 0, low
    #[inline(always)]
    pub const fn with_spi_mem_d_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_D_POL_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_D_POL_MASK
            },
        }
    }

    /// Set **SPI_MEM_Q_POL** `[18:18]`
    ///
    /// The bit is used to set MISO line polarity, 1: high 0, low
    #[inline(always)]
    pub const fn with_spi_mem_q_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_Q_POL_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_Q_POL_MASK
            },
        }
    }

    /// Set **SPI_MEM_FREAD_DUAL** `[14:14]`
    ///
    /// In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_fread_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FREAD_DUAL_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FREAD_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_FASTRD_MODE** `[13:13]`
    ///
    /// This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_fastrd_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FASTRD_MODE_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FASTRD_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_FCMD_OCT** `[9:9]`
    ///
    /// Apply 8 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub const fn with_spi_mem_fcmd_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FCMD_OCT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FCMD_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_FCMD_QUAD** `[8:8]`
    ///
    /// Apply 4 signals during command phase 1:enable 0: disable
    #[inline(always)]
    pub const fn with_spi_mem_fcmd_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FCMD_QUAD_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FCMD_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_FADDR_OCT** `[6:6]`
    ///
    /// Apply 8 signals during address phase 1:enable 0: disable
    #[inline(always)]
    pub const fn with_spi_mem_faddr_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FADDR_OCT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FADDR_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDIN_OCT** `[5:5]`
    ///
    /// Apply 8 signals during read-data phase 1:enable 0: disable
    #[inline(always)]
    pub const fn with_spi_mem_fdin_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FDIN_OCT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FDIN_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDOUT_OCT** `[4:4]`
    ///
    /// Apply 8 signals during write-data phase 1:enable 0: disable
    #[inline(always)]
    pub const fn with_spi_mem_fdout_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FDOUT_OCT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FDOUT_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDUMMY_WOUT** `[3:3]`
    ///
    /// In an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash.
    #[inline(always)]
    pub const fn with_spi_mem_fdummy_wout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FDUMMY_WOUT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FDUMMY_WOUT_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDUMMY_RIN** `[2:2]`
    ///
    /// In an MSPI read data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase.
    #[inline(always)]
    pub const fn with_spi_mem_fdummy_rin(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FDUMMY_RIN_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FDUMMY_RIN_MASK
            },
        }
    }

    /// Set **SPI_MEM_WDUMMY_ALWAYS_OUT** `[1:1]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_mem_wdummy_always_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_WDUMMY_ALWAYS_OUT_MASK
            },
        }
    }

    /// Set **SPI_MEM_WDUMMY_DQS_ALWAYS_OUT** `[0:0]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_mem_wdummy_dqs_always_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_MASK
            },
        }
    }
}

impl Default for SpiMemCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCtrlVal {
        SpiMemCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCtrlVal) -> SpiMemCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CTRL1 Register
///
/// SPI0 control1 register.
///
/// Address offset: 0x000c
/// Reset value: 0x28e00000
///
/// ## Fields
///
/// - **SPI_MEM_TXFIFO_RST** `[31]` (WO)
///
///   The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO.
///
/// - **SPI_MEM_RXFIFO_RST** `[30]` (WO)
///
///   The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO.
///
/// - **SPI_MEM_FAST_WRITE_EN** `[29]` (RW)
///
///   Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2.
///
/// - **SPI_MEM_DUAL_RAM_EN** `[28]` (RO)
///
///   Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
///
/// - **SPI_MEM_RAM0_EN** `[27]` (RO)
///
///   When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
///
/// - **SPI_MEM_AW_SPLICE_EN** `[26]` (RW)
///
///   Set this bit to enable AXI Write Splice-transfer.
///
/// - **SPI_MEM_AR_SPLICE_EN** `[25]` (RW)
///
///   Set this bit to enable AXI Read Splice-transfer.
///
/// - **SPI_MEM_RRESP_ECC_ERR_EN** `[24]` (RW)
///
///   1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG.
///
/// - **SPI_AXI_RDATA_BACK_FAST** `[23]` (RW)
///
///   1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available.
///
/// - **SPI_AW_SIZE0_1_SUPPORT_EN** `[22]` (RW)
///
///   1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR.
///
/// - **SPI_AR_SIZE0_1_SUPPORT_EN** `[21]` (RW)
///
///   1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR.
///
/// - **SPI_MEM_CLK_MODE** `[1:0]` (RW)
///
///   SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
#[repr(transparent)]
pub struct SpiMemCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCtrl1 {
    // ======== Field Constants ========

    /// **SPI_MEM_TXFIFO_RST**
    ///
    /// The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_TXFIFO_RST_OFFSET: u32 = 31;
    pub const SPI_MEM_TXFIFO_RST_MASK: u32 = 0x80000000;
    pub const SPI_MEM_TXFIFO_RST: u32 = 1 << 31;

    /// **SPI_MEM_RXFIFO_RST**
    ///
    /// The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_RXFIFO_RST_OFFSET: u32 = 30;
    pub const SPI_MEM_RXFIFO_RST_MASK: u32 = 0x40000000;
    pub const SPI_MEM_RXFIFO_RST: u32 = 1 << 30;

    /// **SPI_MEM_FAST_WRITE_EN**
    ///
    /// Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_FAST_WRITE_EN_OFFSET: u32 = 29;
    pub const SPI_MEM_FAST_WRITE_EN_MASK: u32 = 0x20000000;
    pub const SPI_MEM_FAST_WRITE_EN: u32 = 1 << 29;

    /// **SPI_MEM_DUAL_RAM_EN**
    ///
    /// Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_DUAL_RAM_EN_OFFSET: u32 = 28;
    pub const SPI_MEM_DUAL_RAM_EN_MASK: u32 = 0x10000000;
    pub const SPI_MEM_DUAL_RAM_EN: u32 = 1 << 28;

    /// **SPI_MEM_RAM0_EN**
    ///
    /// When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MEM_RAM0_EN_OFFSET: u32 = 27;
    pub const SPI_MEM_RAM0_EN_MASK: u32 = 0x08000000;
    pub const SPI_MEM_RAM0_EN: u32 = 1 << 27;

    /// **SPI_MEM_AW_SPLICE_EN**
    ///
    /// Set this bit to enable AXI Write Splice-transfer.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_AW_SPLICE_EN_OFFSET: u32 = 26;
    pub const SPI_MEM_AW_SPLICE_EN_MASK: u32 = 0x04000000;
    pub const SPI_MEM_AW_SPLICE_EN: u32 = 1 << 26;

    /// **SPI_MEM_AR_SPLICE_EN**
    ///
    /// Set this bit to enable AXI Read Splice-transfer.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SPI_MEM_AR_SPLICE_EN_OFFSET: u32 = 25;
    pub const SPI_MEM_AR_SPLICE_EN_MASK: u32 = 0x02000000;
    pub const SPI_MEM_AR_SPLICE_EN: u32 = 1 << 25;

    /// **SPI_MEM_RRESP_ECC_ERR_EN**
    ///
    /// 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_RRESP_ECC_ERR_EN_OFFSET: u32 = 24;
    pub const SPI_MEM_RRESP_ECC_ERR_EN_MASK: u32 = 0x01000000;
    pub const SPI_MEM_RRESP_ECC_ERR_EN: u32 = 1 << 24;

    /// **SPI_AXI_RDATA_BACK_FAST**
    ///
    /// 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_AXI_RDATA_BACK_FAST_OFFSET: u32 = 23;
    pub const SPI_AXI_RDATA_BACK_FAST_MASK: u32 = 0x00800000;
    pub const SPI_AXI_RDATA_BACK_FAST: u32 = 1 << 23;

    /// **SPI_AW_SIZE0_1_SUPPORT_EN**
    ///
    /// 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SPI_AW_SIZE0_1_SUPPORT_EN_OFFSET: u32 = 22;
    pub const SPI_AW_SIZE0_1_SUPPORT_EN_MASK: u32 = 0x00400000;
    pub const SPI_AW_SIZE0_1_SUPPORT_EN: u32 = 1 << 22;

    /// **SPI_AR_SIZE0_1_SUPPORT_EN**
    ///
    /// 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_AR_SIZE0_1_SUPPORT_EN_OFFSET: u32 = 21;
    pub const SPI_AR_SIZE0_1_SUPPORT_EN_MASK: u32 = 0x00200000;
    pub const SPI_AR_SIZE0_1_SUPPORT_EN: u32 = 1 << 21;

    /// **SPI_MEM_CLK_MODE**
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_MEM_CLK_MODE_OFFSET: u32 = 0;
    pub const SPI_MEM_CLK_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **SPI_MEM_TXFIFO_RST** (WO)
    ///
    /// The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO.
    #[inline(always)]
    pub fn trigger_spi_mem_txfifo_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_TXFIFO_RST_MASK);
    }

    /// Trigger **SPI_MEM_RXFIFO_RST** (WO)
    ///
    /// The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO.
    #[inline(always)]
    pub fn trigger_spi_mem_rxfifo_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_RXFIFO_RST_MASK);
    }

    /// **SPI_MEM_FAST_WRITE_EN** `[29]` (RW)
    ///
    /// Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2.
    #[inline(always)]
    pub fn spi_mem_fast_write_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FAST_WRITE_EN_MASK) != 0
    }

    /// Set SPI_MEM_FAST_WRITE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fast_write_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FAST_WRITE_EN_MASK);
    }

    /// Clear SPI_MEM_FAST_WRITE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fast_write_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FAST_WRITE_EN_MASK);
    }

    /// **SPI_MEM_DUAL_RAM_EN** `[28]` (RO)
    ///
    /// Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    #[inline(always)]
    pub fn spi_mem_dual_ram_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DUAL_RAM_EN_MASK) != 0
    }

    /// **SPI_MEM_RAM0_EN** `[27]` (RO)
    ///
    /// When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    #[inline(always)]
    pub fn spi_mem_ram0_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_RAM0_EN_MASK) != 0
    }

    /// **SPI_MEM_AW_SPLICE_EN** `[26]` (RW)
    ///
    /// Set this bit to enable AXI Write Splice-transfer.
    #[inline(always)]
    pub fn spi_mem_aw_splice_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_SPLICE_EN_MASK) != 0
    }

    /// Set SPI_MEM_AW_SPLICE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_splice_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_SPLICE_EN_MASK);
    }

    /// Clear SPI_MEM_AW_SPLICE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_splice_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_SPLICE_EN_MASK);
    }

    /// **SPI_MEM_AR_SPLICE_EN** `[25]` (RW)
    ///
    /// Set this bit to enable AXI Read Splice-transfer.
    #[inline(always)]
    pub fn spi_mem_ar_splice_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_SPLICE_EN_MASK) != 0
    }

    /// Set SPI_MEM_AR_SPLICE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_splice_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_SPLICE_EN_MASK);
    }

    /// Clear SPI_MEM_AR_SPLICE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_splice_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_SPLICE_EN_MASK);
    }

    /// **SPI_MEM_RRESP_ECC_ERR_EN** `[24]` (RW)
    ///
    /// 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG.
    #[inline(always)]
    pub fn spi_mem_rresp_ecc_err_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_RRESP_ECC_ERR_EN_MASK) != 0
    }

    /// Set SPI_MEM_RRESP_ECC_ERR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_rresp_ecc_err_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_RRESP_ECC_ERR_EN_MASK);
    }

    /// Clear SPI_MEM_RRESP_ECC_ERR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_rresp_ecc_err_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_RRESP_ECC_ERR_EN_MASK);
    }

    /// **SPI_AXI_RDATA_BACK_FAST** `[23]` (RW)
    ///
    /// 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available.
    #[inline(always)]
    pub fn spi_axi_rdata_back_fast(&self) -> bool {
        (self.value.read_bits() & Self::SPI_AXI_RDATA_BACK_FAST_MASK) != 0
    }

    /// Set SPI_AXI_RDATA_BACK_FAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_axi_rdata_back_fast(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_AXI_RDATA_BACK_FAST_MASK);
    }

    /// Clear SPI_AXI_RDATA_BACK_FAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_axi_rdata_back_fast(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_AXI_RDATA_BACK_FAST_MASK);
    }

    /// **SPI_AW_SIZE0_1_SUPPORT_EN** `[22]` (RW)
    ///
    /// 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub fn spi_aw_size0_1_support_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_AW_SIZE0_1_SUPPORT_EN_MASK) != 0
    }

    /// Set SPI_AW_SIZE0_1_SUPPORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_aw_size0_1_support_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_AW_SIZE0_1_SUPPORT_EN_MASK);
    }

    /// Clear SPI_AW_SIZE0_1_SUPPORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_aw_size0_1_support_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_AW_SIZE0_1_SUPPORT_EN_MASK);
    }

    /// **SPI_AR_SIZE0_1_SUPPORT_EN** `[21]` (RW)
    ///
    /// 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub fn spi_ar_size0_1_support_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_AR_SIZE0_1_SUPPORT_EN_MASK) != 0
    }

    /// Set SPI_AR_SIZE0_1_SUPPORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_ar_size0_1_support_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_AR_SIZE0_1_SUPPORT_EN_MASK);
    }

    /// Clear SPI_AR_SIZE0_1_SUPPORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_ar_size0_1_support_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_AR_SIZE0_1_SUPPORT_EN_MASK);
    }

    /// **SPI_MEM_CLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CLK_MODE_MASK) >> Self::SPI_MEM_CLK_MODE_OFFSET
    }

    /// Write **SPI_MEM_CLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    #[inline(always)]
    pub fn write_spi_mem_clk_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CLK_MODE_MASK)
                | ((value << Self::SPI_MEM_CLK_MODE_OFFSET) & Self::SPI_MEM_CLK_MODE_MASK)
        });
    }
}

/// SPI_MEM_CTRL1 value type for batch read/modify/write operations
///
/// Use `spimemctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCtrl1Val {
    bits: u32,
}

impl SpiMemCtrl1Val {
    /// Create from raw bits
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
        Self { bits: 0x28e00000 }
    }

    /// Read **SPI_MEM_FAST_WRITE_EN** `[29:29]`
    ///
    /// Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2.
    #[inline(always)]
    pub const fn spi_mem_fast_write_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_FAST_WRITE_EN_MASK) != 0
    }

    /// Read **SPI_MEM_DUAL_RAM_EN** `[28:28]`
    ///
    /// Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    #[inline(always)]
    pub const fn spi_mem_dual_ram_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_DUAL_RAM_EN_MASK) != 0
    }

    /// Read **SPI_MEM_RAM0_EN** `[27:27]`
    ///
    /// When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time.
    #[inline(always)]
    pub const fn spi_mem_ram0_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_RAM0_EN_MASK) != 0
    }

    /// Read **SPI_MEM_AW_SPLICE_EN** `[26:26]`
    ///
    /// Set this bit to enable AXI Write Splice-transfer.
    #[inline(always)]
    pub const fn spi_mem_aw_splice_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_AW_SPLICE_EN_MASK) != 0
    }

    /// Read **SPI_MEM_AR_SPLICE_EN** `[25:25]`
    ///
    /// Set this bit to enable AXI Read Splice-transfer.
    #[inline(always)]
    pub const fn spi_mem_ar_splice_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_AR_SPLICE_EN_MASK) != 0
    }

    /// Read **SPI_MEM_RRESP_ECC_ERR_EN** `[24:24]`
    ///
    /// 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG.
    #[inline(always)]
    pub const fn spi_mem_rresp_ecc_err_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_MEM_RRESP_ECC_ERR_EN_MASK) != 0
    }

    /// Read **SPI_AXI_RDATA_BACK_FAST** `[23:23]`
    ///
    /// 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available.
    #[inline(always)]
    pub const fn spi_axi_rdata_back_fast(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_AXI_RDATA_BACK_FAST_MASK) != 0
    }

    /// Read **SPI_AW_SIZE0_1_SUPPORT_EN** `[22:22]`
    ///
    /// 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub const fn spi_aw_size0_1_support_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_AW_SIZE0_1_SUPPORT_EN_MASK) != 0
    }

    /// Read **SPI_AR_SIZE0_1_SUPPORT_EN** `[21:21]`
    ///
    /// 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub const fn spi_ar_size0_1_support_en(&self) -> bool {
        (self.bits & SpiMemCtrl1::SPI_AR_SIZE0_1_SUPPORT_EN_MASK) != 0
    }

    /// Read **SPI_MEM_CLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    #[inline(always)]
    pub const fn spi_mem_clk_mode(&self) -> u32 {
        (self.bits & SpiMemCtrl1::SPI_MEM_CLK_MODE_MASK) >> SpiMemCtrl1::SPI_MEM_CLK_MODE_OFFSET
    }

    /// Set **SPI_MEM_TXFIFO_RST** `[31:31]`
    ///
    /// The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO.
    #[inline(always)]
    pub const fn with_spi_mem_txfifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_TXFIFO_RST_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_TXFIFO_RST_MASK
            },
        }
    }

    /// Set **SPI_MEM_RXFIFO_RST** `[30:30]`
    ///
    /// The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO.
    #[inline(always)]
    pub const fn with_spi_mem_rxfifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_RXFIFO_RST_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_RXFIFO_RST_MASK
            },
        }
    }

    /// Set **SPI_MEM_FAST_WRITE_EN** `[29:29]`
    ///
    /// Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2.
    #[inline(always)]
    pub const fn with_spi_mem_fast_write_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_FAST_WRITE_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_FAST_WRITE_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_SPLICE_EN** `[26:26]`
    ///
    /// Set this bit to enable AXI Write Splice-transfer.
    #[inline(always)]
    pub const fn with_spi_mem_aw_splice_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_AW_SPLICE_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_AW_SPLICE_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_AR_SPLICE_EN** `[25:25]`
    ///
    /// Set this bit to enable AXI Read Splice-transfer.
    #[inline(always)]
    pub const fn with_spi_mem_ar_splice_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_AR_SPLICE_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_AR_SPLICE_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_RRESP_ECC_ERR_EN** `[24:24]`
    ///
    /// 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG.
    #[inline(always)]
    pub const fn with_spi_mem_rresp_ecc_err_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_MEM_RRESP_ECC_ERR_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_MEM_RRESP_ECC_ERR_EN_MASK
            },
        }
    }

    /// Set **SPI_AXI_RDATA_BACK_FAST** `[23:23]`
    ///
    /// 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available.
    #[inline(always)]
    pub const fn with_spi_axi_rdata_back_fast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_AXI_RDATA_BACK_FAST_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_AXI_RDATA_BACK_FAST_MASK
            },
        }
    }

    /// Set **SPI_AW_SIZE0_1_SUPPORT_EN** `[22:22]`
    ///
    /// 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub const fn with_spi_aw_size0_1_support_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_AW_SIZE0_1_SUPPORT_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_AW_SIZE0_1_SUPPORT_EN_MASK
            },
        }
    }

    /// Set **SPI_AR_SIZE0_1_SUPPORT_EN** `[21:21]`
    ///
    /// 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR.
    #[inline(always)]
    pub const fn with_spi_ar_size0_1_support_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl1::SPI_AR_SIZE0_1_SUPPORT_EN_MASK
            } else {
                self.bits & !SpiMemCtrl1::SPI_AR_SIZE0_1_SUPPORT_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_CLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    #[inline(always)]
    pub const fn with_spi_mem_clk_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl1::SPI_MEM_CLK_MODE_MASK)
                | ((value << SpiMemCtrl1::SPI_MEM_CLK_MODE_OFFSET)
                    & SpiMemCtrl1::SPI_MEM_CLK_MODE_MASK),
        }
    }
}

impl Default for SpiMemCtrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCtrl1Val {
        SpiMemCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCtrl1Val) -> SpiMemCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CTRL2 Register
///
/// SPI0 control2 register.
///
/// Address offset: 0x0010
/// Reset value: 0x01002c21
///
/// ## Fields
///
/// - **SPI_MEM_SYNC_RESET** `[31]` (WO)
///
///   The spi0_mst_st and spi0_slv_st will be reset.
///
/// - **SPI_MEM_CS_HOLD_DELAY** `[30:25]` (RW)
///
///   These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
///
/// - **SPI_MEM_SPLIT_TRANS_EN** `[24]` (RW)
///
///   Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not.
///
/// - **SPI_MEM_ECC_16TO18_BYTE_EN** `[14]` (RW)
///
///   Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash.
///
/// - **SPI_MEM_ECC_SKIP_PAGE_CORNER** `[13]` (RW)
///
///   1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash.
///
/// - **SPI_MEM_ECC_CS_HOLD_TIME** `[12:10]` (RW)
///
///   SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
///
/// - **SPI_MEM_CS_HOLD_TIME** `[9:5]` (RW)
///
///   SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
///
/// - **SPI_MEM_CS_SETUP_TIME** `[4:0]` (RW)
///
///   (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
#[repr(transparent)]
pub struct SpiMemCtrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCtrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCtrl2 {
    // ======== Field Constants ========

    /// **SPI_MEM_SYNC_RESET**
    ///
    /// The spi0_mst_st and spi0_slv_st will be reset.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_SYNC_RESET_OFFSET: u32 = 31;
    pub const SPI_MEM_SYNC_RESET_MASK: u32 = 0x80000000;
    pub const SPI_MEM_SYNC_RESET: u32 = 1 << 31;

    /// **SPI_MEM_CS_HOLD_DELAY**
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    ///
    /// Bits `[30:25]`, mask `0x7e000000`
    pub const SPI_MEM_CS_HOLD_DELAY_OFFSET: u32 = 25;
    pub const SPI_MEM_CS_HOLD_DELAY_MASK: u32 = 0x7e000000;

    /// **SPI_MEM_SPLIT_TRANS_EN**
    ///
    /// Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_SPLIT_TRANS_EN_OFFSET: u32 = 24;
    pub const SPI_MEM_SPLIT_TRANS_EN_MASK: u32 = 0x01000000;
    pub const SPI_MEM_SPLIT_TRANS_EN: u32 = 1 << 24;

    /// **SPI_MEM_ECC_16TO18_BYTE_EN**
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_MEM_ECC_16TO18_BYTE_EN_OFFSET: u32 = 14;
    pub const SPI_MEM_ECC_16TO18_BYTE_EN_MASK: u32 = 0x00004000;
    pub const SPI_MEM_ECC_16TO18_BYTE_EN: u32 = 1 << 14;

    /// **SPI_MEM_ECC_SKIP_PAGE_CORNER**
    ///
    /// 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_MEM_ECC_SKIP_PAGE_CORNER_OFFSET: u32 = 13;
    pub const SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK: u32 = 0x00002000;
    pub const SPI_MEM_ECC_SKIP_PAGE_CORNER: u32 = 1 << 13;

    /// **SPI_MEM_ECC_CS_HOLD_TIME**
    ///
    /// SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
    ///
    /// Bits `[12:10]`, mask `0x00001c00`
    pub const SPI_MEM_ECC_CS_HOLD_TIME_OFFSET: u32 = 10;
    pub const SPI_MEM_ECC_CS_HOLD_TIME_MASK: u32 = 0x00001c00;

    /// **SPI_MEM_CS_HOLD_TIME**
    ///
    /// SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SPI_MEM_CS_HOLD_TIME_OFFSET: u32 = 5;
    pub const SPI_MEM_CS_HOLD_TIME_MASK: u32 = 0x000003e0;

    /// **SPI_MEM_CS_SETUP_TIME**
    ///
    /// (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SPI_MEM_CS_SETUP_TIME_OFFSET: u32 = 0;
    pub const SPI_MEM_CS_SETUP_TIME_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **SPI_MEM_SYNC_RESET** (WO)
    ///
    /// The spi0_mst_st and spi0_slv_st will be reset.
    #[inline(always)]
    pub fn trigger_spi_mem_sync_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SYNC_RESET_MASK);
    }

    /// **SPI_MEM_CS_HOLD_DELAY** `[30:25]` (RW)
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CS_HOLD_DELAY_MASK)
            >> Self::SPI_MEM_CS_HOLD_DELAY_OFFSET
    }

    /// Write **SPI_MEM_CS_HOLD_DELAY** `[30:25]` (RW)
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub fn write_spi_mem_cs_hold_delay(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CS_HOLD_DELAY_MASK)
                | ((value << Self::SPI_MEM_CS_HOLD_DELAY_OFFSET) & Self::SPI_MEM_CS_HOLD_DELAY_MASK)
        });
    }

    /// **SPI_MEM_SPLIT_TRANS_EN** `[24]` (RW)
    ///
    /// Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub fn spi_mem_split_trans_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SPLIT_TRANS_EN_MASK) != 0
    }

    /// Set SPI_MEM_SPLIT_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_split_trans_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SPLIT_TRANS_EN_MASK);
    }

    /// Clear SPI_MEM_SPLIT_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_split_trans_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SPLIT_TRANS_EN_MASK);
    }

    /// **SPI_MEM_ECC_16TO18_BYTE_EN** `[14]` (RW)
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash.
    #[inline(always)]
    pub fn spi_mem_ecc_16to18_byte_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_16TO18_BYTE_EN_MASK) != 0
    }

    /// Set SPI_MEM_ECC_16TO18_BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ecc_16to18_byte_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_ECC_16TO18_BYTE_EN_MASK);
    }

    /// Clear SPI_MEM_ECC_16TO18_BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ecc_16to18_byte_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_ECC_16TO18_BYTE_EN_MASK);
    }

    /// **SPI_MEM_ECC_SKIP_PAGE_CORNER** `[13]` (RW)
    ///
    /// 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash.
    #[inline(always)]
    pub fn spi_mem_ecc_skip_page_corner(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK) != 0
    }

    /// Set SPI_MEM_ECC_SKIP_PAGE_CORNER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ecc_skip_page_corner(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK);
    }

    /// Clear SPI_MEM_ECC_SKIP_PAGE_CORNER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ecc_skip_page_corner(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK);
    }

    /// **SPI_MEM_ECC_CS_HOLD_TIME** `[12:10]` (RW)
    ///
    /// SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
    #[inline(always)]
    pub fn spi_mem_ecc_cs_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_ECC_CS_HOLD_TIME_MASK)
            >> Self::SPI_MEM_ECC_CS_HOLD_TIME_OFFSET
    }

    /// Write **SPI_MEM_ECC_CS_HOLD_TIME** `[12:10]` (RW)
    ///
    /// SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
    #[inline(always)]
    pub fn write_spi_mem_ecc_cs_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_ECC_CS_HOLD_TIME_MASK)
                | ((value << Self::SPI_MEM_ECC_CS_HOLD_TIME_OFFSET)
                    & Self::SPI_MEM_ECC_CS_HOLD_TIME_MASK)
        });
    }

    /// **SPI_MEM_CS_HOLD_TIME** `[9:5]` (RW)
    ///
    /// SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CS_HOLD_TIME_MASK)
            >> Self::SPI_MEM_CS_HOLD_TIME_OFFSET
    }

    /// Write **SPI_MEM_CS_HOLD_TIME** `[9:5]` (RW)
    ///
    /// SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
    #[inline(always)]
    pub fn write_spi_mem_cs_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CS_HOLD_TIME_MASK)
                | ((value << Self::SPI_MEM_CS_HOLD_TIME_OFFSET) & Self::SPI_MEM_CS_HOLD_TIME_MASK)
        });
    }

    /// **SPI_MEM_CS_SETUP_TIME** `[4:0]` (RW)
    ///
    /// (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CS_SETUP_TIME_MASK)
            >> Self::SPI_MEM_CS_SETUP_TIME_OFFSET
    }

    /// Write **SPI_MEM_CS_SETUP_TIME** `[4:0]` (RW)
    ///
    /// (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
    #[inline(always)]
    pub fn write_spi_mem_cs_setup_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CS_SETUP_TIME_MASK)
                | ((value << Self::SPI_MEM_CS_SETUP_TIME_OFFSET) & Self::SPI_MEM_CS_SETUP_TIME_MASK)
        });
    }
}

/// SPI_MEM_CTRL2 value type for batch read/modify/write operations
///
/// Use `spimemctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCtrl2Val {
    bits: u32,
}

impl SpiMemCtrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x01002c21 }
    }

    /// Read **SPI_MEM_CS_HOLD_DELAY** `[30:25]`
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub const fn spi_mem_cs_hold_delay(&self) -> u32 {
        (self.bits & SpiMemCtrl2::SPI_MEM_CS_HOLD_DELAY_MASK)
            >> SpiMemCtrl2::SPI_MEM_CS_HOLD_DELAY_OFFSET
    }

    /// Read **SPI_MEM_SPLIT_TRANS_EN** `[24:24]`
    ///
    /// Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub const fn spi_mem_split_trans_en(&self) -> bool {
        (self.bits & SpiMemCtrl2::SPI_MEM_SPLIT_TRANS_EN_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_16TO18_BYTE_EN** `[14:14]`
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash.
    #[inline(always)]
    pub const fn spi_mem_ecc_16to18_byte_en(&self) -> bool {
        (self.bits & SpiMemCtrl2::SPI_MEM_ECC_16TO18_BYTE_EN_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_SKIP_PAGE_CORNER** `[13:13]`
    ///
    /// 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash.
    #[inline(always)]
    pub const fn spi_mem_ecc_skip_page_corner(&self) -> bool {
        (self.bits & SpiMemCtrl2::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_CS_HOLD_TIME** `[12:10]`
    ///
    /// SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
    #[inline(always)]
    pub const fn spi_mem_ecc_cs_hold_time(&self) -> u32 {
        (self.bits & SpiMemCtrl2::SPI_MEM_ECC_CS_HOLD_TIME_MASK)
            >> SpiMemCtrl2::SPI_MEM_ECC_CS_HOLD_TIME_OFFSET
    }

    /// Read **SPI_MEM_CS_HOLD_TIME** `[9:5]`
    ///
    /// SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
    #[inline(always)]
    pub const fn spi_mem_cs_hold_time(&self) -> u32 {
        (self.bits & SpiMemCtrl2::SPI_MEM_CS_HOLD_TIME_MASK)
            >> SpiMemCtrl2::SPI_MEM_CS_HOLD_TIME_OFFSET
    }

    /// Read **SPI_MEM_CS_SETUP_TIME** `[4:0]`
    ///
    /// (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
    #[inline(always)]
    pub const fn spi_mem_cs_setup_time(&self) -> u32 {
        (self.bits & SpiMemCtrl2::SPI_MEM_CS_SETUP_TIME_MASK)
            >> SpiMemCtrl2::SPI_MEM_CS_SETUP_TIME_OFFSET
    }

    /// Set **SPI_MEM_SYNC_RESET** `[31:31]`
    ///
    /// The spi0_mst_st and spi0_slv_st will be reset.
    #[inline(always)]
    pub const fn with_spi_mem_sync_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl2::SPI_MEM_SYNC_RESET_MASK
            } else {
                self.bits & !SpiMemCtrl2::SPI_MEM_SYNC_RESET_MASK
            },
        }
    }

    /// Set **SPI_MEM_CS_HOLD_DELAY** `[30:25]`
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub const fn with_spi_mem_cs_hold_delay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl2::SPI_MEM_CS_HOLD_DELAY_MASK)
                | ((value << SpiMemCtrl2::SPI_MEM_CS_HOLD_DELAY_OFFSET)
                    & SpiMemCtrl2::SPI_MEM_CS_HOLD_DELAY_MASK),
        }
    }

    /// Set **SPI_MEM_SPLIT_TRANS_EN** `[24:24]`
    ///
    /// Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub const fn with_spi_mem_split_trans_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl2::SPI_MEM_SPLIT_TRANS_EN_MASK
            } else {
                self.bits & !SpiMemCtrl2::SPI_MEM_SPLIT_TRANS_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_16TO18_BYTE_EN** `[14:14]`
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_16to18_byte_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl2::SPI_MEM_ECC_16TO18_BYTE_EN_MASK
            } else {
                self.bits & !SpiMemCtrl2::SPI_MEM_ECC_16TO18_BYTE_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_SKIP_PAGE_CORNER** `[13:13]`
    ///
    /// 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_skip_page_corner(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl2::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK
            } else {
                self.bits & !SpiMemCtrl2::SPI_MEM_ECC_SKIP_PAGE_CORNER_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_CS_HOLD_TIME** `[12:10]`
    ///
    /// SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_cs_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl2::SPI_MEM_ECC_CS_HOLD_TIME_MASK)
                | ((value << SpiMemCtrl2::SPI_MEM_ECC_CS_HOLD_TIME_OFFSET)
                    & SpiMemCtrl2::SPI_MEM_ECC_CS_HOLD_TIME_MASK),
        }
    }

    /// Set **SPI_MEM_CS_HOLD_TIME** `[9:5]`
    ///
    /// SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit.
    #[inline(always)]
    pub const fn with_spi_mem_cs_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl2::SPI_MEM_CS_HOLD_TIME_MASK)
                | ((value << SpiMemCtrl2::SPI_MEM_CS_HOLD_TIME_OFFSET)
                    & SpiMemCtrl2::SPI_MEM_CS_HOLD_TIME_MASK),
        }
    }

    /// Set **SPI_MEM_CS_SETUP_TIME** `[4:0]`
    ///
    /// (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit.
    #[inline(always)]
    pub const fn with_spi_mem_cs_setup_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl2::SPI_MEM_CS_SETUP_TIME_MASK)
                | ((value << SpiMemCtrl2::SPI_MEM_CS_SETUP_TIME_OFFSET)
                    & SpiMemCtrl2::SPI_MEM_CS_SETUP_TIME_MASK),
        }
    }
}

impl Default for SpiMemCtrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemCtrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCtrl2Val {
        SpiMemCtrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCtrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCtrl2Val) -> SpiMemCtrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CLOCK Register
///
/// SPI clock division control register.
///
/// Address offset: 0x0014
/// Reset value: 0x00030103
///
/// ## Fields
///
/// - **SPI_MEM_CLK_EQU_SYSCLK** `[31]` (RW)
///
///   1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock.
///
/// - **SPI_MEM_CLKCNT_N** `[23:16]` (RW)
///
///   In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
///
/// - **SPI_MEM_CLKCNT_H** `[15:8]` (RW)
///
///   In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
///
/// - **SPI_MEM_CLKCNT_L** `[7:0]` (RW)
///
///   In the master mode it must be equal to spi_mem_clkcnt_N.
#[repr(transparent)]
pub struct SpiMemClock {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemClock {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemClock {
    // ======== Field Constants ========

    /// **SPI_MEM_CLK_EQU_SYSCLK**
    ///
    /// 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_CLK_EQU_SYSCLK_OFFSET: u32 = 31;
    pub const SPI_MEM_CLK_EQU_SYSCLK_MASK: u32 = 0x80000000;
    pub const SPI_MEM_CLK_EQU_SYSCLK: u32 = 1 << 31;

    /// **SPI_MEM_CLKCNT_N**
    ///
    /// In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const SPI_MEM_CLKCNT_N_OFFSET: u32 = 16;
    pub const SPI_MEM_CLKCNT_N_MASK: u32 = 0x00ff0000;

    /// **SPI_MEM_CLKCNT_H**
    ///
    /// In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SPI_MEM_CLKCNT_H_OFFSET: u32 = 8;
    pub const SPI_MEM_CLKCNT_H_MASK: u32 = 0x0000ff00;

    /// **SPI_MEM_CLKCNT_L**
    ///
    /// In the master mode it must be equal to spi_mem_clkcnt_N.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SPI_MEM_CLKCNT_L_OFFSET: u32 = 0;
    pub const SPI_MEM_CLKCNT_L_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SPI_MEM_CLK_EQU_SYSCLK** `[31]` (RW)
    ///
    /// 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock.
    #[inline(always)]
    pub fn spi_mem_clk_equ_sysclk(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Set SPI_MEM_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CLK_EQU_SYSCLK_MASK);
    }

    /// Clear SPI_MEM_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CLK_EQU_SYSCLK_MASK);
    }

    /// **SPI_MEM_CLKCNT_N** `[23:16]` (RW)
    ///
    /// In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub fn spi_mem_clkcnt_n(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CLKCNT_N_MASK) >> Self::SPI_MEM_CLKCNT_N_OFFSET
    }

    /// Write **SPI_MEM_CLKCNT_N** `[23:16]` (RW)
    ///
    /// In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub fn write_spi_mem_clkcnt_n(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CLKCNT_N_MASK)
                | ((value << Self::SPI_MEM_CLKCNT_N_OFFSET) & Self::SPI_MEM_CLKCNT_N_MASK)
        });
    }

    /// **SPI_MEM_CLKCNT_H** `[15:8]` (RW)
    ///
    /// In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub fn spi_mem_clkcnt_h(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CLKCNT_H_MASK) >> Self::SPI_MEM_CLKCNT_H_OFFSET
    }

    /// Write **SPI_MEM_CLKCNT_H** `[15:8]` (RW)
    ///
    /// In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub fn write_spi_mem_clkcnt_h(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CLKCNT_H_MASK)
                | ((value << Self::SPI_MEM_CLKCNT_H_OFFSET) & Self::SPI_MEM_CLKCNT_H_MASK)
        });
    }

    /// **SPI_MEM_CLKCNT_L** `[7:0]` (RW)
    ///
    /// In the master mode it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub fn spi_mem_clkcnt_l(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CLKCNT_L_MASK) >> Self::SPI_MEM_CLKCNT_L_OFFSET
    }

    /// Write **SPI_MEM_CLKCNT_L** `[7:0]` (RW)
    ///
    /// In the master mode it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub fn write_spi_mem_clkcnt_l(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CLKCNT_L_MASK)
                | ((value << Self::SPI_MEM_CLKCNT_L_OFFSET) & Self::SPI_MEM_CLKCNT_L_MASK)
        });
    }
}

/// SPI_MEM_CLOCK value type for batch read/modify/write operations
///
/// Use `spimemclock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemclock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemClockVal {
    bits: u32,
}

impl SpiMemClockVal {
    /// Create from raw bits
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
        Self { bits: 0x00030103 }
    }

    /// Read **SPI_MEM_CLK_EQU_SYSCLK** `[31:31]`
    ///
    /// 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock.
    #[inline(always)]
    pub const fn spi_mem_clk_equ_sysclk(&self) -> bool {
        (self.bits & SpiMemClock::SPI_MEM_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Read **SPI_MEM_CLKCNT_N** `[23:16]`
    ///
    /// In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub const fn spi_mem_clkcnt_n(&self) -> u32 {
        (self.bits & SpiMemClock::SPI_MEM_CLKCNT_N_MASK) >> SpiMemClock::SPI_MEM_CLKCNT_N_OFFSET
    }

    /// Read **SPI_MEM_CLKCNT_H** `[15:8]`
    ///
    /// In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub const fn spi_mem_clkcnt_h(&self) -> u32 {
        (self.bits & SpiMemClock::SPI_MEM_CLKCNT_H_MASK) >> SpiMemClock::SPI_MEM_CLKCNT_H_OFFSET
    }

    /// Read **SPI_MEM_CLKCNT_L** `[7:0]`
    ///
    /// In the master mode it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub const fn spi_mem_clkcnt_l(&self) -> u32 {
        (self.bits & SpiMemClock::SPI_MEM_CLKCNT_L_MASK) >> SpiMemClock::SPI_MEM_CLKCNT_L_OFFSET
    }

    /// Set **SPI_MEM_CLK_EQU_SYSCLK** `[31:31]`
    ///
    /// 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock.
    #[inline(always)]
    pub const fn with_spi_mem_clk_equ_sysclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemClock::SPI_MEM_CLK_EQU_SYSCLK_MASK
            } else {
                self.bits & !SpiMemClock::SPI_MEM_CLK_EQU_SYSCLK_MASK
            },
        }
    }

    /// Set **SPI_MEM_CLKCNT_N** `[23:16]`
    ///
    /// In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub const fn with_spi_mem_clkcnt_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemClock::SPI_MEM_CLKCNT_N_MASK)
                | ((value << SpiMemClock::SPI_MEM_CLKCNT_N_OFFSET)
                    & SpiMemClock::SPI_MEM_CLKCNT_N_MASK),
        }
    }

    /// Set **SPI_MEM_CLKCNT_H** `[15:8]`
    ///
    /// In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub const fn with_spi_mem_clkcnt_h(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemClock::SPI_MEM_CLKCNT_H_MASK)
                | ((value << SpiMemClock::SPI_MEM_CLKCNT_H_OFFSET)
                    & SpiMemClock::SPI_MEM_CLKCNT_H_MASK),
        }
    }

    /// Set **SPI_MEM_CLKCNT_L** `[7:0]`
    ///
    /// In the master mode it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub const fn with_spi_mem_clkcnt_l(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemClock::SPI_MEM_CLKCNT_L_MASK)
                | ((value << SpiMemClock::SPI_MEM_CLKCNT_L_OFFSET)
                    & SpiMemClock::SPI_MEM_CLKCNT_L_MASK),
        }
    }
}

impl Default for SpiMemClockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemClock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemClockVal {
        SpiMemClockVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemClockVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemClockVal) -> SpiMemClockVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_USER Register
///
/// SPI0 user register.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **SPI_MEM_USR_DUMMY** `[29]` (RW)
///
///   This bit enable the dummy phase of an operation.
///
/// - **SPI_MEM_USR_DUMMY_IDLE** `[26]` (RW)
///
///   spi clock is disable in dummy phase when the bit is enable.
///
/// - **SPI_MEM_CK_OUT_EDGE** `[9]` (RW)
///
///   The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3.
///
/// - **SPI_MEM_CS_SETUP** `[7]` (RW)
///
///   spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
///
/// - **SPI_MEM_CS_HOLD** `[6]` (RW)
///
///   spi cs keep low when spi is in done phase. 1: enable 0: disable.
#[repr(transparent)]
pub struct SpiMemUser {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemUser {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemUser {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_DUMMY**
    ///
    /// This bit enable the dummy phase of an operation.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_USR_DUMMY_OFFSET: u32 = 29;
    pub const SPI_MEM_USR_DUMMY_MASK: u32 = 0x20000000;
    pub const SPI_MEM_USR_DUMMY: u32 = 1 << 29;

    /// **SPI_MEM_USR_DUMMY_IDLE**
    ///
    /// spi clock is disable in dummy phase when the bit is enable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_USR_DUMMY_IDLE_OFFSET: u32 = 26;
    pub const SPI_MEM_USR_DUMMY_IDLE_MASK: u32 = 0x04000000;
    pub const SPI_MEM_USR_DUMMY_IDLE: u32 = 1 << 26;

    /// **SPI_MEM_CK_OUT_EDGE**
    ///
    /// The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_CK_OUT_EDGE_OFFSET: u32 = 9;
    pub const SPI_MEM_CK_OUT_EDGE_MASK: u32 = 0x00000200;
    pub const SPI_MEM_CK_OUT_EDGE: u32 = 1 << 9;

    /// **SPI_MEM_CS_SETUP**
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_CS_SETUP_OFFSET: u32 = 7;
    pub const SPI_MEM_CS_SETUP_MASK: u32 = 0x00000080;
    pub const SPI_MEM_CS_SETUP: u32 = 1 << 7;

    /// **SPI_MEM_CS_HOLD**
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_CS_HOLD_OFFSET: u32 = 6;
    pub const SPI_MEM_CS_HOLD_MASK: u32 = 0x00000040;
    pub const SPI_MEM_CS_HOLD: u32 = 1 << 6;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_DUMMY** `[29]` (RW)
    ///
    /// This bit enable the dummy phase of an operation.
    #[inline(always)]
    pub fn spi_mem_usr_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_DUMMY_MASK) != 0
    }

    /// Set SPI_MEM_USR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_dummy(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_USR_DUMMY_MASK);
    }

    /// Clear SPI_MEM_USR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_DUMMY_MASK);
    }

    /// **SPI_MEM_USR_DUMMY_IDLE** `[26]` (RW)
    ///
    /// spi clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    pub fn spi_mem_usr_dummy_idle(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_DUMMY_IDLE_MASK) != 0
    }

    /// Set SPI_MEM_USR_DUMMY_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_dummy_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_DUMMY_IDLE_MASK);
    }

    /// Clear SPI_MEM_USR_DUMMY_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_dummy_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_DUMMY_IDLE_MASK);
    }

    /// **SPI_MEM_CK_OUT_EDGE** `[9]` (RW)
    ///
    /// The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3.
    #[inline(always)]
    pub fn spi_mem_ck_out_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CK_OUT_EDGE_MASK) != 0
    }

    /// Set SPI_MEM_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ck_out_edge(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CK_OUT_EDGE_MASK);
    }

    /// Clear SPI_MEM_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ck_out_edge(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CK_OUT_EDGE_MASK);
    }

    /// **SPI_MEM_CS_SETUP** `[7]` (RW)
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_cs_setup(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CS_SETUP_MASK) != 0
    }

    /// Set SPI_MEM_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cs_setup(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_CS_SETUP_MASK);
    }

    /// Clear SPI_MEM_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cs_setup(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_CS_SETUP_MASK);
    }

    /// **SPI_MEM_CS_HOLD** `[6]` (RW)
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_cs_hold(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CS_HOLD_MASK) != 0
    }

    /// Set SPI_MEM_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cs_hold(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_CS_HOLD_MASK);
    }

    /// Clear SPI_MEM_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cs_hold(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_CS_HOLD_MASK);
    }
}

/// SPI_MEM_USER value type for batch read/modify/write operations
///
/// Use `spimemuser.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemuser.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemUserVal {
    bits: u32,
}

impl SpiMemUserVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_USR_DUMMY** `[29:29]`
    ///
    /// This bit enable the dummy phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_dummy(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_DUMMY_MASK) != 0
    }

    /// Read **SPI_MEM_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// spi clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    pub const fn spi_mem_usr_dummy_idle(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_DUMMY_IDLE_MASK) != 0
    }

    /// Read **SPI_MEM_CK_OUT_EDGE** `[9:9]`
    ///
    /// The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3.
    #[inline(always)]
    pub const fn spi_mem_ck_out_edge(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_CK_OUT_EDGE_MASK) != 0
    }

    /// Read **SPI_MEM_CS_SETUP** `[7:7]`
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_cs_setup(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_CS_SETUP_MASK) != 0
    }

    /// Read **SPI_MEM_CS_HOLD** `[6:6]`
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_cs_hold(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_CS_HOLD_MASK) != 0
    }

    /// Set **SPI_MEM_USR_DUMMY** `[29:29]`
    ///
    /// This bit enable the dummy phase of an operation.
    #[inline(always)]
    pub const fn with_spi_mem_usr_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_DUMMY_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// spi clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    pub const fn with_spi_mem_usr_dummy_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_DUMMY_IDLE_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_DUMMY_IDLE_MASK
            },
        }
    }

    /// Set **SPI_MEM_CK_OUT_EDGE** `[9:9]`
    ///
    /// The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3.
    #[inline(always)]
    pub const fn with_spi_mem_ck_out_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_CK_OUT_EDGE_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_CK_OUT_EDGE_MASK
            },
        }
    }

    /// Set **SPI_MEM_CS_SETUP** `[7:7]`
    ///
    /// spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_cs_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_CS_SETUP_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_CS_SETUP_MASK
            },
        }
    }

    /// Set **SPI_MEM_CS_HOLD** `[6:6]`
    ///
    /// spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_cs_hold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_CS_HOLD_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_CS_HOLD_MASK
            },
        }
    }
}

impl Default for SpiMemUserVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemUser {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemUserVal {
        SpiMemUserVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemUserVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemUserVal) -> SpiMemUserVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_USER1 Register
///
/// SPI0 user1 register.
///
/// Address offset: 0x001c
/// Reset value: 0x5c000047
///
/// ## Fields
///
/// - **SPI_MEM_USR_ADDR_BITLEN** `[31:26]` (RW)
///
///   The length in bits of address phase. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_USR_DBYTELEN** `[8:6]` (RO)
///
///   SPI0 USR_CMD read or write data byte length -1
///
/// - **SPI_MEM_USR_DUMMY_CYCLELEN** `[5:0]` (RW)
///
///   The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
#[repr(transparent)]
pub struct SpiMemUser1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemUser1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemUser1 {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_ADDR_BITLEN**
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    ///
    /// Bits `[31:26]`, mask `0xfc000000`
    pub const SPI_MEM_USR_ADDR_BITLEN_OFFSET: u32 = 26;
    pub const SPI_MEM_USR_ADDR_BITLEN_MASK: u32 = 0xfc000000;

    /// **SPI_MEM_USR_DBYTELEN**
    ///
    /// SPI0 USR_CMD read or write data byte length -1
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SPI_MEM_USR_DBYTELEN_OFFSET: u32 = 6;
    pub const SPI_MEM_USR_DBYTELEN_MASK: u32 = 0x000001c0;

    /// **SPI_MEM_USR_DUMMY_CYCLELEN**
    ///
    /// The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SPI_MEM_USR_DUMMY_CYCLELEN_OFFSET: u32 = 0;
    pub const SPI_MEM_USR_DUMMY_CYCLELEN_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_ADDR_BITLEN** `[31:26]` (RW)
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_ADDR_BITLEN_MASK)
            >> Self::SPI_MEM_USR_ADDR_BITLEN_OFFSET
    }

    /// Write **SPI_MEM_USR_ADDR_BITLEN** `[31:26]` (RW)
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_usr_addr_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_ADDR_BITLEN_MASK)
                | ((value << Self::SPI_MEM_USR_ADDR_BITLEN_OFFSET)
                    & Self::SPI_MEM_USR_ADDR_BITLEN_MASK)
        });
    }

    /// **SPI_MEM_USR_DBYTELEN** `[8:6]` (RO)
    ///
    /// SPI0 USR_CMD read or write data byte length -1
    #[inline(always)]
    pub fn spi_mem_usr_dbytelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_DBYTELEN_MASK)
            >> Self::SPI_MEM_USR_DBYTELEN_OFFSET
    }

    /// **SPI_MEM_USR_DUMMY_CYCLELEN** `[5:0]` (RW)
    ///
    /// The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_DUMMY_CYCLELEN_MASK)
            >> Self::SPI_MEM_USR_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_USR_DUMMY_CYCLELEN** `[5:0]` (RW)
    ///
    /// The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn write_spi_mem_usr_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_DUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_USR_DUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_USR_DUMMY_CYCLELEN_MASK)
        });
    }
}

/// SPI_MEM_USER1 value type for batch read/modify/write operations
///
/// Use `spimemuser1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemuser1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemUser1Val {
    bits: u32,
}

impl SpiMemUser1Val {
    /// Create from raw bits
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
        Self { bits: 0x5c000047 }
    }

    /// Read **SPI_MEM_USR_ADDR_BITLEN** `[31:26]`
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_usr_addr_bitlen(&self) -> u32 {
        (self.bits & SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_MASK)
            >> SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_OFFSET
    }

    /// Read **SPI_MEM_USR_DBYTELEN** `[8:6]`
    ///
    /// SPI0 USR_CMD read or write data byte length -1
    #[inline(always)]
    pub const fn spi_mem_usr_dbytelen(&self) -> u32 {
        (self.bits & SpiMemUser1::SPI_MEM_USR_DBYTELEN_MASK)
            >> SpiMemUser1::SPI_MEM_USR_DBYTELEN_OFFSET
    }

    /// Read **SPI_MEM_USR_DUMMY_CYCLELEN** `[5:0]`
    ///
    /// The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub const fn spi_mem_usr_dummy_cyclelen(&self) -> u32 {
        (self.bits & SpiMemUser1::SPI_MEM_USR_DUMMY_CYCLELEN_MASK)
            >> SpiMemUser1::SPI_MEM_USR_DUMMY_CYCLELEN_OFFSET
    }

    /// Set **SPI_MEM_USR_ADDR_BITLEN** `[31:26]`
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_usr_addr_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_MASK)
                | ((value << SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_OFFSET)
                    & SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_MASK),
        }
    }

    /// Set **SPI_MEM_USR_DUMMY_CYCLELEN** `[5:0]`
    ///
    /// The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_usr_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemUser1::SPI_MEM_USR_DUMMY_CYCLELEN_MASK)
                | ((value << SpiMemUser1::SPI_MEM_USR_DUMMY_CYCLELEN_OFFSET)
                    & SpiMemUser1::SPI_MEM_USR_DUMMY_CYCLELEN_MASK),
        }
    }
}

impl Default for SpiMemUser1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemUser1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemUser1Val {
        SpiMemUser1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemUser1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemUser1Val) -> SpiMemUser1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_USER2 Register
///
/// SPI0 user2 register.
///
/// Address offset: 0x0020
/// Reset value: 0x70000000
///
/// ## Fields
///
/// - **SPI_MEM_USR_COMMAND_BITLEN** `[31:28]` (RW)
///
///   The length in bits of command phase. The register value shall be (bit_num-1)
///
/// - **SPI_MEM_USR_COMMAND_VALUE** `[15:0]` (RW)
///
///   The value of command.
#[repr(transparent)]
pub struct SpiMemUser2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemUser2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemUser2 {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_COMMAND_BITLEN**
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1)
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const SPI_MEM_USR_COMMAND_BITLEN_OFFSET: u32 = 28;
    pub const SPI_MEM_USR_COMMAND_BITLEN_MASK: u32 = 0xf0000000;

    /// **SPI_MEM_USR_COMMAND_VALUE**
    ///
    /// The value of command.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_MEM_USR_COMMAND_VALUE_OFFSET: u32 = 0;
    pub const SPI_MEM_USR_COMMAND_VALUE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_COMMAND_BITLEN** `[31:28]` (RW)
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1)
    #[inline(always)]
    pub fn spi_mem_usr_command_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_COMMAND_BITLEN_MASK)
            >> Self::SPI_MEM_USR_COMMAND_BITLEN_OFFSET
    }

    /// Write **SPI_MEM_USR_COMMAND_BITLEN** `[31:28]` (RW)
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1)
    #[inline(always)]
    pub fn write_spi_mem_usr_command_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_COMMAND_BITLEN_MASK)
                | ((value << Self::SPI_MEM_USR_COMMAND_BITLEN_OFFSET)
                    & Self::SPI_MEM_USR_COMMAND_BITLEN_MASK)
        });
    }

    /// **SPI_MEM_USR_COMMAND_VALUE** `[15:0]` (RW)
    ///
    /// The value of command.
    #[inline(always)]
    pub fn spi_mem_usr_command_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_COMMAND_VALUE_MASK)
            >> Self::SPI_MEM_USR_COMMAND_VALUE_OFFSET
    }

    /// Write **SPI_MEM_USR_COMMAND_VALUE** `[15:0]` (RW)
    ///
    /// The value of command.
    #[inline(always)]
    pub fn write_spi_mem_usr_command_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_COMMAND_VALUE_MASK)
                | ((value << Self::SPI_MEM_USR_COMMAND_VALUE_OFFSET)
                    & Self::SPI_MEM_USR_COMMAND_VALUE_MASK)
        });
    }
}

/// SPI_MEM_USER2 value type for batch read/modify/write operations
///
/// Use `spimemuser2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemuser2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemUser2Val {
    bits: u32,
}

impl SpiMemUser2Val {
    /// Create from raw bits
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
        Self { bits: 0x70000000 }
    }

    /// Read **SPI_MEM_USR_COMMAND_BITLEN** `[31:28]`
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1)
    #[inline(always)]
    pub const fn spi_mem_usr_command_bitlen(&self) -> u32 {
        (self.bits & SpiMemUser2::SPI_MEM_USR_COMMAND_BITLEN_MASK)
            >> SpiMemUser2::SPI_MEM_USR_COMMAND_BITLEN_OFFSET
    }

    /// Read **SPI_MEM_USR_COMMAND_VALUE** `[15:0]`
    ///
    /// The value of command.
    #[inline(always)]
    pub const fn spi_mem_usr_command_value(&self) -> u32 {
        (self.bits & SpiMemUser2::SPI_MEM_USR_COMMAND_VALUE_MASK)
            >> SpiMemUser2::SPI_MEM_USR_COMMAND_VALUE_OFFSET
    }

    /// Set **SPI_MEM_USR_COMMAND_BITLEN** `[31:28]`
    ///
    /// The length in bits of command phase. The register value shall be (bit_num-1)
    #[inline(always)]
    pub const fn with_spi_mem_usr_command_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemUser2::SPI_MEM_USR_COMMAND_BITLEN_MASK)
                | ((value << SpiMemUser2::SPI_MEM_USR_COMMAND_BITLEN_OFFSET)
                    & SpiMemUser2::SPI_MEM_USR_COMMAND_BITLEN_MASK),
        }
    }

    /// Set **SPI_MEM_USR_COMMAND_VALUE** `[15:0]`
    ///
    /// The value of command.
    #[inline(always)]
    pub const fn with_spi_mem_usr_command_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemUser2::SPI_MEM_USR_COMMAND_VALUE_MASK)
                | ((value << SpiMemUser2::SPI_MEM_USR_COMMAND_VALUE_OFFSET)
                    & SpiMemUser2::SPI_MEM_USR_COMMAND_VALUE_MASK),
        }
    }
}

impl Default for SpiMemUser2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemUser2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemUser2Val {
        SpiMemUser2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemUser2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemUser2Val) -> SpiMemUser2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_RD_STATUS Register
///
/// SPI0 read control register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SPI_MEM_WB_MODE** `[23:16]` (RW)
///
///   Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
#[repr(transparent)]
pub struct SpiMemRdStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemRdStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemRdStatus {
    // ======== Field Constants ========

    /// **SPI_MEM_WB_MODE**
    ///
    /// Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const SPI_MEM_WB_MODE_OFFSET: u32 = 16;
    pub const SPI_MEM_WB_MODE_MASK: u32 = 0x00ff0000;

    // ======== Field Accessors ========

    /// **SPI_MEM_WB_MODE** `[23:16]` (RW)
    ///
    /// Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub fn spi_mem_wb_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_WB_MODE_MASK) >> Self::SPI_MEM_WB_MODE_OFFSET
    }

    /// Write **SPI_MEM_WB_MODE** `[23:16]` (RW)
    ///
    /// Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub fn write_spi_mem_wb_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_WB_MODE_MASK)
                | ((value << Self::SPI_MEM_WB_MODE_OFFSET) & Self::SPI_MEM_WB_MODE_MASK)
        });
    }
}

/// SPI_MEM_RD_STATUS value type for batch read/modify/write operations
///
/// Use `spimemrdstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemrdstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemRdStatusVal {
    bits: u32,
}

impl SpiMemRdStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_WB_MODE** `[23:16]`
    ///
    /// Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub const fn spi_mem_wb_mode(&self) -> u32 {
        (self.bits & SpiMemRdStatus::SPI_MEM_WB_MODE_MASK) >> SpiMemRdStatus::SPI_MEM_WB_MODE_OFFSET
    }

    /// Set **SPI_MEM_WB_MODE** `[23:16]`
    ///
    /// Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub const fn with_spi_mem_wb_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemRdStatus::SPI_MEM_WB_MODE_MASK)
                | ((value << SpiMemRdStatus::SPI_MEM_WB_MODE_OFFSET)
                    & SpiMemRdStatus::SPI_MEM_WB_MODE_MASK),
        }
    }
}

impl Default for SpiMemRdStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemRdStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemRdStatusVal {
        SpiMemRdStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemRdStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemRdStatusVal) -> SpiMemRdStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_MISC Register
///
/// SPI0 misc register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SPI_MEM_CS_KEEP_ACTIVE** `[10]` (RW)
///
///   SPI_CS line keep low when the bit is set.
///
/// - **SPI_MEM_CK_IDLE_EDGE** `[9]` (RW)
///
///   1: SPI_CLK line is high when idle 0: spi clk line is low when idle
///
/// - **SPI_MEM_SSUB_PIN** `[8]` (RW)
///
///   For SPI0, sram is connected to SUBPINs.
///
/// - **SPI_MEM_FSUB_PIN** `[7]` (RW)
///
///   For SPI0, flash is connected to SUBPINs.
#[repr(transparent)]
pub struct SpiMemMisc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMisc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMisc {
    // ======== Field Constants ========

    /// **SPI_MEM_CS_KEEP_ACTIVE**
    ///
    /// SPI_CS line keep low when the bit is set.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_CS_KEEP_ACTIVE_OFFSET: u32 = 10;
    pub const SPI_MEM_CS_KEEP_ACTIVE_MASK: u32 = 0x00000400;
    pub const SPI_MEM_CS_KEEP_ACTIVE: u32 = 1 << 10;

    /// **SPI_MEM_CK_IDLE_EDGE**
    ///
    /// 1: SPI_CLK line is high when idle 0: spi clk line is low when idle
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_CK_IDLE_EDGE_OFFSET: u32 = 9;
    pub const SPI_MEM_CK_IDLE_EDGE_MASK: u32 = 0x00000200;
    pub const SPI_MEM_CK_IDLE_EDGE: u32 = 1 << 9;

    /// **SPI_MEM_SSUB_PIN**
    ///
    /// For SPI0, sram is connected to SUBPINs.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_SSUB_PIN_OFFSET: u32 = 8;
    pub const SPI_MEM_SSUB_PIN_MASK: u32 = 0x00000100;
    pub const SPI_MEM_SSUB_PIN: u32 = 1 << 8;

    /// **SPI_MEM_FSUB_PIN**
    ///
    /// For SPI0, flash is connected to SUBPINs.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_FSUB_PIN_OFFSET: u32 = 7;
    pub const SPI_MEM_FSUB_PIN_MASK: u32 = 0x00000080;
    pub const SPI_MEM_FSUB_PIN: u32 = 1 << 7;

    // ======== Field Accessors ========

    /// **SPI_MEM_CS_KEEP_ACTIVE** `[10]` (RW)
    ///
    /// SPI_CS line keep low when the bit is set.
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CS_KEEP_ACTIVE_MASK) != 0
    }

    /// Set SPI_MEM_CS_KEEP_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cs_keep_active(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CS_KEEP_ACTIVE_MASK);
    }

    /// Clear SPI_MEM_CS_KEEP_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cs_keep_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CS_KEEP_ACTIVE_MASK);
    }

    /// **SPI_MEM_CK_IDLE_EDGE** `[9]` (RW)
    ///
    /// 1: SPI_CLK line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CK_IDLE_EDGE_MASK) != 0
    }

    /// Set SPI_MEM_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ck_idle_edge(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CK_IDLE_EDGE_MASK);
    }

    /// Clear SPI_MEM_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ck_idle_edge(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CK_IDLE_EDGE_MASK);
    }

    /// **SPI_MEM_SSUB_PIN** `[8]` (RW)
    ///
    /// For SPI0, sram is connected to SUBPINs.
    #[inline(always)]
    pub fn spi_mem_ssub_pin(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SSUB_PIN_MASK) != 0
    }

    /// Set SPI_MEM_SSUB_PIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ssub_pin(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SSUB_PIN_MASK);
    }

    /// Clear SPI_MEM_SSUB_PIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ssub_pin(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_SSUB_PIN_MASK);
    }

    /// **SPI_MEM_FSUB_PIN** `[7]` (RW)
    ///
    /// For SPI0, flash is connected to SUBPINs.
    #[inline(always)]
    pub fn spi_mem_fsub_pin(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FSUB_PIN_MASK) != 0
    }

    /// Set SPI_MEM_FSUB_PIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fsub_pin(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FSUB_PIN_MASK);
    }

    /// Clear SPI_MEM_FSUB_PIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fsub_pin(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FSUB_PIN_MASK);
    }
}

/// SPI_MEM_MISC value type for batch read/modify/write operations
///
/// Use `spimemmisc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmisc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMiscVal {
    bits: u32,
}

impl SpiMemMiscVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_CS_KEEP_ACTIVE** `[10:10]`
    ///
    /// SPI_CS line keep low when the bit is set.
    #[inline(always)]
    pub const fn spi_mem_cs_keep_active(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CS_KEEP_ACTIVE_MASK) != 0
    }

    /// Read **SPI_MEM_CK_IDLE_EDGE** `[9:9]`
    ///
    /// 1: SPI_CLK line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    pub const fn spi_mem_ck_idle_edge(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CK_IDLE_EDGE_MASK) != 0
    }

    /// Read **SPI_MEM_SSUB_PIN** `[8:8]`
    ///
    /// For SPI0, sram is connected to SUBPINs.
    #[inline(always)]
    pub const fn spi_mem_ssub_pin(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_SSUB_PIN_MASK) != 0
    }

    /// Read **SPI_MEM_FSUB_PIN** `[7:7]`
    ///
    /// For SPI0, flash is connected to SUBPINs.
    #[inline(always)]
    pub const fn spi_mem_fsub_pin(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_FSUB_PIN_MASK) != 0
    }

    /// Set **SPI_MEM_CS_KEEP_ACTIVE** `[10:10]`
    ///
    /// SPI_CS line keep low when the bit is set.
    #[inline(always)]
    pub const fn with_spi_mem_cs_keep_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_CS_KEEP_ACTIVE_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_CS_KEEP_ACTIVE_MASK
            },
        }
    }

    /// Set **SPI_MEM_CK_IDLE_EDGE** `[9:9]`
    ///
    /// 1: SPI_CLK line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    pub const fn with_spi_mem_ck_idle_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_CK_IDLE_EDGE_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_CK_IDLE_EDGE_MASK
            },
        }
    }

    /// Set **SPI_MEM_SSUB_PIN** `[8:8]`
    ///
    /// For SPI0, sram is connected to SUBPINs.
    #[inline(always)]
    pub const fn with_spi_mem_ssub_pin(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_SSUB_PIN_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_SSUB_PIN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FSUB_PIN** `[7:7]`
    ///
    /// For SPI0, flash is connected to SUBPINs.
    #[inline(always)]
    pub const fn with_spi_mem_fsub_pin(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_FSUB_PIN_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_FSUB_PIN_MASK
            },
        }
    }
}

impl Default for SpiMemMiscVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemMisc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMiscVal {
        SpiMemMiscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMiscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMiscVal) -> SpiMemMiscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CACHE_FCTRL Register
///
/// SPI0 bit mode control register.
///
/// Address offset: 0x003c
/// Reset value: 0xc0000000
///
/// ## Fields
///
/// - **SPI_CLOSE_AXI_INF_EN** `[31]` (RW)
///
///   Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP.
///
/// - **SPI_SAME_AW_AR_ADDR_CHK_EN** `[30]` (RW)
///
///   Set this bit to check AXI read/write the same address region.
///
/// - **SPI_MEM_FADDR_QUAD** `[8]` (RW)
///
///   For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FDOUT_QUAD** `[7]` (RW)
///
///   For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FDIN_QUAD** `[6]` (RW)
///
///   For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FADDR_DUAL** `[5]` (RW)
///
///   For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_FDOUT_DUAL** `[4]` (RW)
///
///   For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_FDIN_DUAL** `[3]` (RW)
///
///   For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_CACHE_FLASH_USR_CMD** `[2]` (RW)
///
///   For SPI0, cache read flash for user define command, 1: enable, 0:disable.
///
/// - **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1]` (RW)
///
///   For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable.
///
/// - **SPI_MEM_AXI_REQ_EN** `[0]` (RW)
///
///   For SPI0, AXI master access enable, 1: enable, 0:disable.
#[repr(transparent)]
pub struct SpiMemCacheFctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCacheFctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCacheFctrl {
    // ======== Field Constants ========

    /// **SPI_CLOSE_AXI_INF_EN**
    ///
    /// Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_CLOSE_AXI_INF_EN_OFFSET: u32 = 31;
    pub const SPI_CLOSE_AXI_INF_EN_MASK: u32 = 0x80000000;
    pub const SPI_CLOSE_AXI_INF_EN: u32 = 1 << 31;

    /// **SPI_SAME_AW_AR_ADDR_CHK_EN**
    ///
    /// Set this bit to check AXI read/write the same address region.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_SAME_AW_AR_ADDR_CHK_EN_OFFSET: u32 = 30;
    pub const SPI_SAME_AW_AR_ADDR_CHK_EN_MASK: u32 = 0x40000000;
    pub const SPI_SAME_AW_AR_ADDR_CHK_EN: u32 = 1 << 30;

    /// **SPI_MEM_FADDR_QUAD**
    ///
    /// For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_FADDR_QUAD_OFFSET: u32 = 8;
    pub const SPI_MEM_FADDR_QUAD_MASK: u32 = 0x00000100;
    pub const SPI_MEM_FADDR_QUAD: u32 = 1 << 8;

    /// **SPI_MEM_FDOUT_QUAD**
    ///
    /// For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_FDOUT_QUAD_OFFSET: u32 = 7;
    pub const SPI_MEM_FDOUT_QUAD_MASK: u32 = 0x00000080;
    pub const SPI_MEM_FDOUT_QUAD: u32 = 1 << 7;

    /// **SPI_MEM_FDIN_QUAD**
    ///
    /// For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_FDIN_QUAD_OFFSET: u32 = 6;
    pub const SPI_MEM_FDIN_QUAD_MASK: u32 = 0x00000040;
    pub const SPI_MEM_FDIN_QUAD: u32 = 1 << 6;

    /// **SPI_MEM_FADDR_DUAL**
    ///
    /// For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_FADDR_DUAL_OFFSET: u32 = 5;
    pub const SPI_MEM_FADDR_DUAL_MASK: u32 = 0x00000020;
    pub const SPI_MEM_FADDR_DUAL: u32 = 1 << 5;

    /// **SPI_MEM_FDOUT_DUAL**
    ///
    /// For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_FDOUT_DUAL_OFFSET: u32 = 4;
    pub const SPI_MEM_FDOUT_DUAL_MASK: u32 = 0x00000010;
    pub const SPI_MEM_FDOUT_DUAL: u32 = 1 << 4;

    /// **SPI_MEM_FDIN_DUAL**
    ///
    /// For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FDIN_DUAL_OFFSET: u32 = 3;
    pub const SPI_MEM_FDIN_DUAL_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FDIN_DUAL: u32 = 1 << 3;

    /// **SPI_MEM_CACHE_FLASH_USR_CMD**
    ///
    /// For SPI0, cache read flash for user define command, 1: enable, 0:disable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_CACHE_FLASH_USR_CMD_OFFSET: u32 = 2;
    pub const SPI_MEM_CACHE_FLASH_USR_CMD_MASK: u32 = 0x00000004;
    pub const SPI_MEM_CACHE_FLASH_USR_CMD: u32 = 1 << 2;

    /// **SPI_MEM_CACHE_USR_ADDR_4BYTE**
    ///
    /// For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE_OFFSET: u32 = 1;
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK: u32 = 0x00000002;
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE: u32 = 1 << 1;

    /// **SPI_MEM_AXI_REQ_EN**
    ///
    /// For SPI0, AXI master access enable, 1: enable, 0:disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_AXI_REQ_EN_OFFSET: u32 = 0;
    pub const SPI_MEM_AXI_REQ_EN_MASK: u32 = 0x00000001;
    pub const SPI_MEM_AXI_REQ_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_CLOSE_AXI_INF_EN** `[31]` (RW)
    ///
    /// Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP.
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CLOSE_AXI_INF_EN_MASK) != 0
    }

    /// Set SPI_CLOSE_AXI_INF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_close_axi_inf_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CLOSE_AXI_INF_EN_MASK);
    }

    /// Clear SPI_CLOSE_AXI_INF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_close_axi_inf_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CLOSE_AXI_INF_EN_MASK);
    }

    /// **SPI_SAME_AW_AR_ADDR_CHK_EN** `[30]` (RW)
    ///
    /// Set this bit to check AXI read/write the same address region.
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK) != 0
    }

    /// Set SPI_SAME_AW_AR_ADDR_CHK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_same_aw_ar_addr_chk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK);
    }

    /// Clear SPI_SAME_AW_AR_ADDR_CHK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_same_aw_ar_addr_chk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK);
    }

    /// **SPI_MEM_FADDR_QUAD** `[8]` (RW)
    ///
    /// For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FADDR_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_faddr_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FADDR_QUAD_MASK);
    }

    /// Clear SPI_MEM_FADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_faddr_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FADDR_QUAD_MASK);
    }

    /// **SPI_MEM_FDOUT_QUAD** `[7]` (RW)
    ///
    /// For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDOUT_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FDOUT_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdout_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FDOUT_QUAD_MASK);
    }

    /// Clear SPI_MEM_FDOUT_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdout_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDOUT_QUAD_MASK);
    }

    /// **SPI_MEM_FDIN_QUAD** `[6]` (RW)
    ///
    /// For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDIN_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FDIN_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdin_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FDIN_QUAD_MASK);
    }

    /// Clear SPI_MEM_FDIN_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdin_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDIN_QUAD_MASK);
    }

    /// **SPI_MEM_FADDR_DUAL** `[5]` (RW)
    ///
    /// For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FADDR_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_FADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_faddr_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FADDR_DUAL_MASK);
    }

    /// Clear SPI_MEM_FADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_faddr_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FADDR_DUAL_MASK);
    }

    /// **SPI_MEM_FDOUT_DUAL** `[4]` (RW)
    ///
    /// For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDOUT_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_FDOUT_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdout_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FDOUT_DUAL_MASK);
    }

    /// Clear SPI_MEM_FDOUT_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdout_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDOUT_DUAL_MASK);
    }

    /// **SPI_MEM_FDIN_DUAL** `[3]` (RW)
    ///
    /// For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FDIN_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_FDIN_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fdin_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FDIN_DUAL_MASK);
    }

    /// Clear SPI_MEM_FDIN_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fdin_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FDIN_DUAL_MASK);
    }

    /// **SPI_MEM_CACHE_FLASH_USR_CMD** `[2]` (RW)
    ///
    /// For SPI0, cache read flash for user define command, 1: enable, 0:disable.
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_FLASH_USR_CMD_MASK) != 0
    }

    /// Set SPI_MEM_CACHE_FLASH_USR_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cache_flash_usr_cmd(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CACHE_FLASH_USR_CMD_MASK);
    }

    /// Clear SPI_MEM_CACHE_FLASH_USR_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cache_flash_usr_cmd(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CACHE_FLASH_USR_CMD_MASK);
    }

    /// **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1]` (RW)
    ///
    /// For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable.
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK) != 0
    }

    /// Set SPI_MEM_CACHE_USR_ADDR_4BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cache_usr_addr_4byte(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK);
    }

    /// Clear SPI_MEM_CACHE_USR_ADDR_4BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cache_usr_addr_4byte(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK);
    }

    /// **SPI_MEM_AXI_REQ_EN** `[0]` (RW)
    ///
    /// For SPI0, AXI master access enable, 1: enable, 0:disable.
    #[inline(always)]
    pub fn spi_mem_axi_req_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_REQ_EN_MASK) != 0
    }

    /// Set SPI_MEM_AXI_REQ_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_req_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_REQ_EN_MASK);
    }

    /// Clear SPI_MEM_AXI_REQ_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_req_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_REQ_EN_MASK);
    }
}

/// SPI_MEM_CACHE_FCTRL value type for batch read/modify/write operations
///
/// Use `spimemcachefctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemcachefctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCacheFctrlVal {
    bits: u32,
}

impl SpiMemCacheFctrlVal {
    /// Create from raw bits
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
        Self { bits: 0xc0000000 }
    }

    /// Read **SPI_CLOSE_AXI_INF_EN** `[31:31]`
    ///
    /// Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP.
    #[inline(always)]
    pub const fn spi_close_axi_inf_en(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_CLOSE_AXI_INF_EN_MASK) != 0
    }

    /// Read **SPI_SAME_AW_AR_ADDR_CHK_EN** `[30:30]`
    ///
    /// Set this bit to check AXI read/write the same address region.
    #[inline(always)]
    pub const fn spi_same_aw_ar_addr_chk_en(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FADDR_QUAD** `[8:8]`
    ///
    /// For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_faddr_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FADDR_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FDOUT_QUAD** `[7:7]`
    ///
    /// For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_fdout_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDOUT_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FDIN_QUAD** `[6:6]`
    ///
    /// For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_fdin_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDIN_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FADDR_DUAL** `[5:5]`
    ///
    /// For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_faddr_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FADDR_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_FDOUT_DUAL** `[4:4]`
    ///
    /// For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_fdout_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDOUT_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_FDIN_DUAL** `[3:3]`
    ///
    /// For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_fdin_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDIN_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_CACHE_FLASH_USR_CMD** `[2:2]`
    ///
    /// For SPI0, cache read flash for user define command, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn spi_mem_cache_flash_usr_cmd(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_CACHE_FLASH_USR_CMD_MASK) != 0
    }

    /// Read **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1:1]`
    ///
    /// For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn spi_mem_cache_usr_addr_4byte(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_REQ_EN** `[0:0]`
    ///
    /// For SPI0, AXI master access enable, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn spi_mem_axi_req_en(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_AXI_REQ_EN_MASK) != 0
    }

    /// Set **SPI_CLOSE_AXI_INF_EN** `[31:31]`
    ///
    /// Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP.
    #[inline(always)]
    pub const fn with_spi_close_axi_inf_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_CLOSE_AXI_INF_EN_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_CLOSE_AXI_INF_EN_MASK
            },
        }
    }

    /// Set **SPI_SAME_AW_AR_ADDR_CHK_EN** `[30:30]`
    ///
    /// Set this bit to check AXI read/write the same address region.
    #[inline(always)]
    pub const fn with_spi_same_aw_ar_addr_chk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_SAME_AW_AR_ADDR_CHK_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FADDR_QUAD** `[8:8]`
    ///
    /// For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn with_spi_mem_faddr_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FADDR_QUAD_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FADDR_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDOUT_QUAD** `[7:7]`
    ///
    /// For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn with_spi_mem_fdout_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FDOUT_QUAD_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FDOUT_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDIN_QUAD** `[6:6]`
    ///
    /// For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn with_spi_mem_fdin_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FDIN_QUAD_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FDIN_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_FADDR_DUAL** `[5:5]`
    ///
    /// For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn with_spi_mem_faddr_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FADDR_DUAL_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FADDR_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDOUT_DUAL** `[4:4]`
    ///
    /// For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn with_spi_mem_fdout_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FDOUT_DUAL_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FDOUT_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_FDIN_DUAL** `[3:3]`
    ///
    /// For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn with_spi_mem_fdin_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_FDIN_DUAL_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_FDIN_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_CACHE_FLASH_USR_CMD** `[2:2]`
    ///
    /// For SPI0, cache read flash for user define command, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn with_spi_mem_cache_flash_usr_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_CACHE_FLASH_USR_CMD_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_CACHE_FLASH_USR_CMD_MASK
            },
        }
    }

    /// Set **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1:1]`
    ///
    /// For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn with_spi_mem_cache_usr_addr_4byte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_REQ_EN** `[0:0]`
    ///
    /// For SPI0, AXI master access enable, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn with_spi_mem_axi_req_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheFctrl::SPI_MEM_AXI_REQ_EN_MASK
            } else {
                self.bits & !SpiMemCacheFctrl::SPI_MEM_AXI_REQ_EN_MASK
            },
        }
    }
}

impl Default for SpiMemCacheFctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemCacheFctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCacheFctrlVal {
        SpiMemCacheFctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCacheFctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCacheFctrlVal) -> SpiMemCacheFctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CACHE_SCTRL Register
///
/// SPI0 external RAM control register
///
/// Address offset: 0x0040
/// Reset value: 0x0055c070
///
/// ## Fields
///
/// - **SPI_MEM_SRAM_WDUMMY_CYCLELEN** `[27:22]` (RW)
///
///   For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_SRAM_OCT** `[21]` (RW)
///
///   reserved
///
/// - **SPI_MEM_CACHE_SRAM_USR_WCMD** `[20]` (RW)
///
///   For SPI0, In the external RAM mode cache write sram for user define command
///
/// - **SPI_MEM_SRAM_ADDR_BITLEN** `[19:14]` (RW)
///
///   For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_SRAM_RDUMMY_CYCLELEN** `[11:6]` (RW)
///
///   For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_CACHE_SRAM_USR_RCMD** `[5]` (RW)
///
///   For SPI0, In the external RAM mode cache read external RAM for user define command.
///
/// - **SPI_MEM_USR_RD_SRAM_DUMMY** `[4]` (RW)
///
///   For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
///
/// - **SPI_MEM_USR_WR_SRAM_DUMMY** `[3]` (RW)
///
///   For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
///
/// - **SPI_MEM_USR_SRAM_QIO** `[2]` (RW)
///
///   For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
///
/// - **SPI_MEM_USR_SRAM_DIO** `[1]` (RW)
///
///   For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
///
/// - **SPI_MEM_CACHE_USR_SADDR_4BYTE** `[0]` (RW)
///
///   For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
#[repr(transparent)]
pub struct SpiMemCacheSctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemCacheSctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemCacheSctrl {
    // ======== Field Constants ========

    /// **SPI_MEM_SRAM_WDUMMY_CYCLELEN**
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    ///
    /// Bits `[27:22]`, mask `0x0fc00000`
    pub const SPI_MEM_SRAM_WDUMMY_CYCLELEN_OFFSET: u32 = 22;
    pub const SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK: u32 = 0x0fc00000;

    /// **SPI_MEM_SRAM_OCT**
    ///
    /// reserved
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_MEM_SRAM_OCT_OFFSET: u32 = 21;
    pub const SPI_MEM_SRAM_OCT_MASK: u32 = 0x00200000;
    pub const SPI_MEM_SRAM_OCT: u32 = 1 << 21;

    /// **SPI_MEM_CACHE_SRAM_USR_WCMD**
    ///
    /// For SPI0, In the external RAM mode cache write sram for user define command
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_MEM_CACHE_SRAM_USR_WCMD_OFFSET: u32 = 20;
    pub const SPI_MEM_CACHE_SRAM_USR_WCMD_MASK: u32 = 0x00100000;
    pub const SPI_MEM_CACHE_SRAM_USR_WCMD: u32 = 1 << 20;

    /// **SPI_MEM_SRAM_ADDR_BITLEN**
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    ///
    /// Bits `[19:14]`, mask `0x000fc000`
    pub const SPI_MEM_SRAM_ADDR_BITLEN_OFFSET: u32 = 14;
    pub const SPI_MEM_SRAM_ADDR_BITLEN_MASK: u32 = 0x000fc000;

    /// **SPI_MEM_SRAM_RDUMMY_CYCLELEN**
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    ///
    /// Bits `[11:6]`, mask `0x00000fc0`
    pub const SPI_MEM_SRAM_RDUMMY_CYCLELEN_OFFSET: u32 = 6;
    pub const SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK: u32 = 0x00000fc0;

    /// **SPI_MEM_CACHE_SRAM_USR_RCMD**
    ///
    /// For SPI0, In the external RAM mode cache read external RAM for user define command.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_CACHE_SRAM_USR_RCMD_OFFSET: u32 = 5;
    pub const SPI_MEM_CACHE_SRAM_USR_RCMD_MASK: u32 = 0x00000020;
    pub const SPI_MEM_CACHE_SRAM_USR_RCMD: u32 = 1 << 5;

    /// **SPI_MEM_USR_RD_SRAM_DUMMY**
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_USR_RD_SRAM_DUMMY_OFFSET: u32 = 4;
    pub const SPI_MEM_USR_RD_SRAM_DUMMY_MASK: u32 = 0x00000010;
    pub const SPI_MEM_USR_RD_SRAM_DUMMY: u32 = 1 << 4;

    /// **SPI_MEM_USR_WR_SRAM_DUMMY**
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_USR_WR_SRAM_DUMMY_OFFSET: u32 = 3;
    pub const SPI_MEM_USR_WR_SRAM_DUMMY_MASK: u32 = 0x00000008;
    pub const SPI_MEM_USR_WR_SRAM_DUMMY: u32 = 1 << 3;

    /// **SPI_MEM_USR_SRAM_QIO**
    ///
    /// For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_USR_SRAM_QIO_OFFSET: u32 = 2;
    pub const SPI_MEM_USR_SRAM_QIO_MASK: u32 = 0x00000004;
    pub const SPI_MEM_USR_SRAM_QIO: u32 = 1 << 2;

    /// **SPI_MEM_USR_SRAM_DIO**
    ///
    /// For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_USR_SRAM_DIO_OFFSET: u32 = 1;
    pub const SPI_MEM_USR_SRAM_DIO_MASK: u32 = 0x00000002;
    pub const SPI_MEM_USR_SRAM_DIO: u32 = 1 << 1;

    /// **SPI_MEM_CACHE_USR_SADDR_4BYTE**
    ///
    /// For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_CACHE_USR_SADDR_4BYTE_OFFSET: u32 = 0;
    pub const SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK: u32 = 0x00000001;
    pub const SPI_MEM_CACHE_USR_SADDR_4BYTE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_SRAM_WDUMMY_CYCLELEN** `[27:22]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_sram_wdummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK)
            >> Self::SPI_MEM_SRAM_WDUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_SRAM_WDUMMY_CYCLELEN** `[27:22]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_sram_wdummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_SRAM_WDUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK)
        });
    }

    /// **SPI_MEM_SRAM_OCT** `[21]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn spi_mem_sram_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SRAM_OCT_MASK) != 0
    }

    /// Set SPI_MEM_SRAM_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sram_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SRAM_OCT_MASK);
    }

    /// Clear SPI_MEM_SRAM_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sram_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_SRAM_OCT_MASK);
    }

    /// **SPI_MEM_CACHE_SRAM_USR_WCMD** `[20]` (RW)
    ///
    /// For SPI0, In the external RAM mode cache write sram for user define command
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wcmd(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK) != 0
    }

    /// Set SPI_MEM_CACHE_SRAM_USR_WCMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cache_sram_usr_wcmd(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK);
    }

    /// Clear SPI_MEM_CACHE_SRAM_USR_WCMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cache_sram_usr_wcmd(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK);
    }

    /// **SPI_MEM_SRAM_ADDR_BITLEN** `[19:14]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_sram_addr_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SRAM_ADDR_BITLEN_MASK)
            >> Self::SPI_MEM_SRAM_ADDR_BITLEN_OFFSET
    }

    /// Write **SPI_MEM_SRAM_ADDR_BITLEN** `[19:14]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_sram_addr_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SRAM_ADDR_BITLEN_MASK)
                | ((value << Self::SPI_MEM_SRAM_ADDR_BITLEN_OFFSET)
                    & Self::SPI_MEM_SRAM_ADDR_BITLEN_MASK)
        });
    }

    /// **SPI_MEM_SRAM_RDUMMY_CYCLELEN** `[11:6]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_sram_rdummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK)
            >> Self::SPI_MEM_SRAM_RDUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_SRAM_RDUMMY_CYCLELEN** `[11:6]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_sram_rdummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_SRAM_RDUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK)
        });
    }

    /// **SPI_MEM_CACHE_SRAM_USR_RCMD** `[5]` (RW)
    ///
    /// For SPI0, In the external RAM mode cache read external RAM for user define command.
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_rcmd(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK) != 0
    }

    /// Set SPI_MEM_CACHE_SRAM_USR_RCMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cache_sram_usr_rcmd(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK);
    }

    /// Clear SPI_MEM_CACHE_SRAM_USR_RCMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cache_sram_usr_rcmd(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK);
    }

    /// **SPI_MEM_USR_RD_SRAM_DUMMY** `[4]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    #[inline(always)]
    pub fn spi_mem_usr_rd_sram_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_RD_SRAM_DUMMY_MASK) != 0
    }

    /// Set SPI_MEM_USR_RD_SRAM_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_rd_sram_dummy(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_RD_SRAM_DUMMY_MASK);
    }

    /// Clear SPI_MEM_USR_RD_SRAM_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_rd_sram_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_RD_SRAM_DUMMY_MASK);
    }

    /// **SPI_MEM_USR_WR_SRAM_DUMMY** `[3]` (RW)
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    #[inline(always)]
    pub fn spi_mem_usr_wr_sram_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_WR_SRAM_DUMMY_MASK) != 0
    }

    /// Set SPI_MEM_USR_WR_SRAM_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_wr_sram_dummy(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_WR_SRAM_DUMMY_MASK);
    }

    /// Clear SPI_MEM_USR_WR_SRAM_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_wr_sram_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_WR_SRAM_DUMMY_MASK);
    }

    /// **SPI_MEM_USR_SRAM_QIO** `[2]` (RW)
    ///
    /// For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub fn spi_mem_usr_sram_qio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_SRAM_QIO_MASK) != 0
    }

    /// Set SPI_MEM_USR_SRAM_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_sram_qio(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_SRAM_QIO_MASK);
    }

    /// Clear SPI_MEM_USR_SRAM_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_sram_qio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_SRAM_QIO_MASK);
    }

    /// **SPI_MEM_USR_SRAM_DIO** `[1]` (RW)
    ///
    /// For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub fn spi_mem_usr_sram_dio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_SRAM_DIO_MASK) != 0
    }

    /// Set SPI_MEM_USR_SRAM_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_sram_dio(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_SRAM_DIO_MASK);
    }

    /// Clear SPI_MEM_USR_SRAM_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_sram_dio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_SRAM_DIO_MASK);
    }

    /// **SPI_MEM_CACHE_USR_SADDR_4BYTE** `[0]` (RW)
    ///
    /// For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    #[inline(always)]
    pub fn spi_mem_cache_usr_saddr_4byte(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK) != 0
    }

    /// Set SPI_MEM_CACHE_USR_SADDR_4BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cache_usr_saddr_4byte(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK);
    }

    /// Clear SPI_MEM_CACHE_USR_SADDR_4BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cache_usr_saddr_4byte(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK);
    }
}

/// SPI_MEM_CACHE_SCTRL value type for batch read/modify/write operations
///
/// Use `spimemcachesctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemcachesctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemCacheSctrlVal {
    bits: u32,
}

impl SpiMemCacheSctrlVal {
    /// Create from raw bits
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
        Self { bits: 0x0055c070 }
    }

    /// Read **SPI_MEM_SRAM_WDUMMY_CYCLELEN** `[27:22]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_sram_wdummy_cyclelen(&self) -> u32 {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK)
            >> SpiMemCacheSctrl::SPI_MEM_SRAM_WDUMMY_CYCLELEN_OFFSET
    }

    /// Read **SPI_MEM_SRAM_OCT** `[21:21]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn spi_mem_sram_oct(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_SRAM_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_WCMD** `[20:20]`
    ///
    /// For SPI0, In the external RAM mode cache write sram for user define command
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_wcmd(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK) != 0
    }

    /// Read **SPI_MEM_SRAM_ADDR_BITLEN** `[19:14]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_sram_addr_bitlen(&self) -> u32 {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_SRAM_ADDR_BITLEN_MASK)
            >> SpiMemCacheSctrl::SPI_MEM_SRAM_ADDR_BITLEN_OFFSET
    }

    /// Read **SPI_MEM_SRAM_RDUMMY_CYCLELEN** `[11:6]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_sram_rdummy_cyclelen(&self) -> u32 {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK)
            >> SpiMemCacheSctrl::SPI_MEM_SRAM_RDUMMY_CYCLELEN_OFFSET
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_RCMD** `[5:5]`
    ///
    /// For SPI0, In the external RAM mode cache read external RAM for user define command.
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_rcmd(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK) != 0
    }

    /// Read **SPI_MEM_USR_RD_SRAM_DUMMY** `[4:4]`
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    #[inline(always)]
    pub const fn spi_mem_usr_rd_sram_dummy(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_USR_RD_SRAM_DUMMY_MASK) != 0
    }

    /// Read **SPI_MEM_USR_WR_SRAM_DUMMY** `[3:3]`
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    #[inline(always)]
    pub const fn spi_mem_usr_wr_sram_dummy(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_USR_WR_SRAM_DUMMY_MASK) != 0
    }

    /// Read **SPI_MEM_USR_SRAM_QIO** `[2:2]`
    ///
    /// For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub const fn spi_mem_usr_sram_qio(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_USR_SRAM_QIO_MASK) != 0
    }

    /// Read **SPI_MEM_USR_SRAM_DIO** `[1:1]`
    ///
    /// For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub const fn spi_mem_usr_sram_dio(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_USR_SRAM_DIO_MASK) != 0
    }

    /// Read **SPI_MEM_CACHE_USR_SADDR_4BYTE** `[0:0]`
    ///
    /// For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn spi_mem_cache_usr_saddr_4byte(&self) -> bool {
        (self.bits & SpiMemCacheSctrl::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK) != 0
    }

    /// Set **SPI_MEM_SRAM_WDUMMY_CYCLELEN** `[27:22]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_sram_wdummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCacheSctrl::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK)
                | ((value << SpiMemCacheSctrl::SPI_MEM_SRAM_WDUMMY_CYCLELEN_OFFSET)
                    & SpiMemCacheSctrl::SPI_MEM_SRAM_WDUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_MEM_SRAM_OCT** `[21:21]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_spi_mem_sram_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_SRAM_OCT_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_SRAM_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_WCMD** `[20:20]`
    ///
    /// For SPI0, In the external RAM mode cache write sram for user define command
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_wcmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_WCMD_MASK
            },
        }
    }

    /// Set **SPI_MEM_SRAM_ADDR_BITLEN** `[19:14]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_sram_addr_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCacheSctrl::SPI_MEM_SRAM_ADDR_BITLEN_MASK)
                | ((value << SpiMemCacheSctrl::SPI_MEM_SRAM_ADDR_BITLEN_OFFSET)
                    & SpiMemCacheSctrl::SPI_MEM_SRAM_ADDR_BITLEN_MASK),
        }
    }

    /// Set **SPI_MEM_SRAM_RDUMMY_CYCLELEN** `[11:6]`
    ///
    /// For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_sram_rdummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCacheSctrl::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK)
                | ((value << SpiMemCacheSctrl::SPI_MEM_SRAM_RDUMMY_CYCLELEN_OFFSET)
                    & SpiMemCacheSctrl::SPI_MEM_SRAM_RDUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_RCMD** `[5:5]`
    ///
    /// For SPI0, In the external RAM mode cache read external RAM for user define command.
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_rcmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_CACHE_SRAM_USR_RCMD_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_RD_SRAM_DUMMY** `[4:4]`
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    #[inline(always)]
    pub const fn with_spi_mem_usr_rd_sram_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_USR_RD_SRAM_DUMMY_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_USR_RD_SRAM_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_WR_SRAM_DUMMY** `[3:3]`
    ///
    /// For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    #[inline(always)]
    pub const fn with_spi_mem_usr_wr_sram_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_USR_WR_SRAM_DUMMY_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_USR_WR_SRAM_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_SRAM_QIO** `[2:2]`
    ///
    /// For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub const fn with_spi_mem_usr_sram_qio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_USR_SRAM_QIO_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_USR_SRAM_QIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_SRAM_DIO** `[1:1]`
    ///
    /// For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub const fn with_spi_mem_usr_sram_dio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_USR_SRAM_DIO_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_USR_SRAM_DIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_CACHE_USR_SADDR_4BYTE** `[0:0]`
    ///
    /// For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn with_spi_mem_cache_usr_saddr_4byte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCacheSctrl::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK
            } else {
                self.bits & !SpiMemCacheSctrl::SPI_MEM_CACHE_USR_SADDR_4BYTE_MASK
            },
        }
    }
}

impl Default for SpiMemCacheSctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemCacheSctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemCacheSctrlVal {
        SpiMemCacheSctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCacheSctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCacheSctrlVal) -> SpiMemCacheSctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_SRAM_CMD Register
///
/// SPI0 external RAM mode control register
///
/// Address offset: 0x0044
/// Reset value: 0x80c00000
///
/// ## Fields
///
/// - **SPI_SMEM_DATA_IE_ALWAYS_ON** `[31]` (RW)
///
///   When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
///
/// - **SPI_SMEM_DQS_IE_ALWAYS_ON** `[30]` (RW)
///
///   When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
///
/// - **SPI_MEM_SDOUT_HEX** `[27]` (RW)
///
///   For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_SDIN_HEX** `[26]` (RW)
///
///   For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable.
///
/// - **SPI_SMEM_WDUMMY_ALWAYS_OUT** `[25]` (RW)
///
///   In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO[7:0] is output by the MSPI controller.
///
/// - **SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT** `[24]` (RW)
///
///   In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller.
///
/// - **SPI_MEM_SDUMMY_WOUT** `[23]` (RW)
///
///   In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
///
/// - **SPI_MEM_SDUMMY_RIN** `[22]` (RW)
///
///   In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
///
/// - **SPI_MEM_SCMD_OCT** `[21]` (RW)
///
///   For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_SADDR_OCT** `[20]` (RW)
///
///   For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_SDOUT_OCT** `[19]` (RW)
///
///   For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_SDIN_OCT** `[18]` (RW)
///
///   For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_SCMD_QUAD** `[17]` (RW)
///
///   For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
///
/// - **SPI_MEM_SADDR_QUAD** `[16]` (RW)
///
///   For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
///
/// - **SPI_MEM_SDOUT_QUAD** `[15]` (RW)
///
///   For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
///
/// - **SPI_MEM_SDIN_QUAD** `[14]` (RW)
///
///   For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
///
/// - **SPI_MEM_SADDR_DUAL** `[12]` (RW)
///
///   For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
///
/// - **SPI_MEM_SDOUT_DUAL** `[11]` (RW)
///
///   For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
///
/// - **SPI_MEM_SDIN_DUAL** `[10]` (RW)
///
///   For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
///
/// - **SPI_MEM_SWB_MODE** `[9:2]` (RW)
///
///   Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
///
/// - **SPI_MEM_SCLK_MODE** `[1:0]` (RW)
///
///   SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
#[repr(transparent)]
pub struct SpiMemSramCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemSramCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemSramCmd {
    // ======== Field Constants ========

    /// **SPI_SMEM_DATA_IE_ALWAYS_ON**
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_SMEM_DATA_IE_ALWAYS_ON_OFFSET: u32 = 31;
    pub const SPI_SMEM_DATA_IE_ALWAYS_ON_MASK: u32 = 0x80000000;
    pub const SPI_SMEM_DATA_IE_ALWAYS_ON: u32 = 1 << 31;

    /// **SPI_SMEM_DQS_IE_ALWAYS_ON**
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_SMEM_DQS_IE_ALWAYS_ON_OFFSET: u32 = 30;
    pub const SPI_SMEM_DQS_IE_ALWAYS_ON_MASK: u32 = 0x40000000;
    pub const SPI_SMEM_DQS_IE_ALWAYS_ON: u32 = 1 << 30;

    /// **SPI_MEM_SDOUT_HEX**
    ///
    /// For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MEM_SDOUT_HEX_OFFSET: u32 = 27;
    pub const SPI_MEM_SDOUT_HEX_MASK: u32 = 0x08000000;
    pub const SPI_MEM_SDOUT_HEX: u32 = 1 << 27;

    /// **SPI_MEM_SDIN_HEX**
    ///
    /// For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_SDIN_HEX_OFFSET: u32 = 26;
    pub const SPI_MEM_SDIN_HEX_MASK: u32 = 0x04000000;
    pub const SPI_MEM_SDIN_HEX: u32 = 1 << 26;

    /// **SPI_SMEM_WDUMMY_ALWAYS_OUT**
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO[7:0] is output by the MSPI controller.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SPI_SMEM_WDUMMY_ALWAYS_OUT_OFFSET: u32 = 25;
    pub const SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK: u32 = 0x02000000;
    pub const SPI_SMEM_WDUMMY_ALWAYS_OUT: u32 = 1 << 25;

    /// **SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT**
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_OFFSET: u32 = 24;
    pub const SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK: u32 = 0x01000000;
    pub const SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT: u32 = 1 << 24;

    /// **SPI_MEM_SDUMMY_WOUT**
    ///
    /// In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_MEM_SDUMMY_WOUT_OFFSET: u32 = 23;
    pub const SPI_MEM_SDUMMY_WOUT_MASK: u32 = 0x00800000;
    pub const SPI_MEM_SDUMMY_WOUT: u32 = 1 << 23;

    /// **SPI_MEM_SDUMMY_RIN**
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SPI_MEM_SDUMMY_RIN_OFFSET: u32 = 22;
    pub const SPI_MEM_SDUMMY_RIN_MASK: u32 = 0x00400000;
    pub const SPI_MEM_SDUMMY_RIN: u32 = 1 << 22;

    /// **SPI_MEM_SCMD_OCT**
    ///
    /// For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_MEM_SCMD_OCT_OFFSET: u32 = 21;
    pub const SPI_MEM_SCMD_OCT_MASK: u32 = 0x00200000;
    pub const SPI_MEM_SCMD_OCT: u32 = 1 << 21;

    /// **SPI_MEM_SADDR_OCT**
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_MEM_SADDR_OCT_OFFSET: u32 = 20;
    pub const SPI_MEM_SADDR_OCT_MASK: u32 = 0x00100000;
    pub const SPI_MEM_SADDR_OCT: u32 = 1 << 20;

    /// **SPI_MEM_SDOUT_OCT**
    ///
    /// For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_MEM_SDOUT_OCT_OFFSET: u32 = 19;
    pub const SPI_MEM_SDOUT_OCT_MASK: u32 = 0x00080000;
    pub const SPI_MEM_SDOUT_OCT: u32 = 1 << 19;

    /// **SPI_MEM_SDIN_OCT**
    ///
    /// For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MEM_SDIN_OCT_OFFSET: u32 = 18;
    pub const SPI_MEM_SDIN_OCT_MASK: u32 = 0x00040000;
    pub const SPI_MEM_SDIN_OCT: u32 = 1 << 18;

    /// **SPI_MEM_SCMD_QUAD**
    ///
    /// For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MEM_SCMD_QUAD_OFFSET: u32 = 17;
    pub const SPI_MEM_SCMD_QUAD_MASK: u32 = 0x00020000;
    pub const SPI_MEM_SCMD_QUAD: u32 = 1 << 17;

    /// **SPI_MEM_SADDR_QUAD**
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_MEM_SADDR_QUAD_OFFSET: u32 = 16;
    pub const SPI_MEM_SADDR_QUAD_MASK: u32 = 0x00010000;
    pub const SPI_MEM_SADDR_QUAD: u32 = 1 << 16;

    /// **SPI_MEM_SDOUT_QUAD**
    ///
    /// For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_MEM_SDOUT_QUAD_OFFSET: u32 = 15;
    pub const SPI_MEM_SDOUT_QUAD_MASK: u32 = 0x00008000;
    pub const SPI_MEM_SDOUT_QUAD: u32 = 1 << 15;

    /// **SPI_MEM_SDIN_QUAD**
    ///
    /// For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_MEM_SDIN_QUAD_OFFSET: u32 = 14;
    pub const SPI_MEM_SDIN_QUAD_MASK: u32 = 0x00004000;
    pub const SPI_MEM_SDIN_QUAD: u32 = 1 << 14;

    /// **SPI_MEM_SADDR_DUAL**
    ///
    /// For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_MEM_SADDR_DUAL_OFFSET: u32 = 12;
    pub const SPI_MEM_SADDR_DUAL_MASK: u32 = 0x00001000;
    pub const SPI_MEM_SADDR_DUAL: u32 = 1 << 12;

    /// **SPI_MEM_SDOUT_DUAL**
    ///
    /// For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_MEM_SDOUT_DUAL_OFFSET: u32 = 11;
    pub const SPI_MEM_SDOUT_DUAL_MASK: u32 = 0x00000800;
    pub const SPI_MEM_SDOUT_DUAL: u32 = 1 << 11;

    /// **SPI_MEM_SDIN_DUAL**
    ///
    /// For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_SDIN_DUAL_OFFSET: u32 = 10;
    pub const SPI_MEM_SDIN_DUAL_MASK: u32 = 0x00000400;
    pub const SPI_MEM_SDIN_DUAL: u32 = 1 << 10;

    /// **SPI_MEM_SWB_MODE**
    ///
    /// Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const SPI_MEM_SWB_MODE_OFFSET: u32 = 2;
    pub const SPI_MEM_SWB_MODE_MASK: u32 = 0x000003fc;

    /// **SPI_MEM_SCLK_MODE**
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_MEM_SCLK_MODE_OFFSET: u32 = 0;
    pub const SPI_MEM_SCLK_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DATA_IE_ALWAYS_ON** `[31]` (RW)
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub fn spi_smem_data_ie_always_on(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK) != 0
    }

    /// Set SPI_SMEM_DATA_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_data_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK);
    }

    /// Clear SPI_SMEM_DATA_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_data_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK);
    }

    /// **SPI_SMEM_DQS_IE_ALWAYS_ON** `[30]` (RW)
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub fn spi_smem_dqs_ie_always_on(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK) != 0
    }

    /// Set SPI_SMEM_DQS_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dqs_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK);
    }

    /// Clear SPI_SMEM_DQS_IE_ALWAYS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dqs_ie_always_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK);
    }

    /// **SPI_MEM_SDOUT_HEX** `[27]` (RW)
    ///
    /// For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_sdout_hex(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDOUT_HEX_MASK) != 0
    }

    /// Set SPI_MEM_SDOUT_HEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdout_hex(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDOUT_HEX_MASK);
    }

    /// Clear SPI_MEM_SDOUT_HEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdout_hex(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDOUT_HEX_MASK);
    }

    /// **SPI_MEM_SDIN_HEX** `[26]` (RW)
    ///
    /// For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_sdin_hex(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDIN_HEX_MASK) != 0
    }

    /// Set SPI_MEM_SDIN_HEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdin_hex(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDIN_HEX_MASK);
    }

    /// Clear SPI_MEM_SDIN_HEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdin_hex(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_SDIN_HEX_MASK);
    }

    /// **SPI_SMEM_WDUMMY_ALWAYS_OUT** `[25]` (RW)
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_smem_wdummy_always_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK) != 0
    }

    /// Set SPI_SMEM_WDUMMY_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_wdummy_always_out(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK);
    }

    /// Clear SPI_SMEM_WDUMMY_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_wdummy_always_out(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK);
    }

    /// **SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT** `[24]` (RW)
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_smem_wdummy_dqs_always_out(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK) != 0
    }

    /// Set SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_wdummy_dqs_always_out(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK);
    }

    /// Clear SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_wdummy_dqs_always_out(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK);
    }

    /// **SPI_MEM_SDUMMY_WOUT** `[23]` (RW)
    ///
    /// In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_mem_sdummy_wout(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDUMMY_WOUT_MASK) != 0
    }

    /// Set SPI_MEM_SDUMMY_WOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdummy_wout(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SDUMMY_WOUT_MASK);
    }

    /// Clear SPI_MEM_SDUMMY_WOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdummy_wout(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDUMMY_WOUT_MASK);
    }

    /// **SPI_MEM_SDUMMY_RIN** `[22]` (RW)
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub fn spi_mem_sdummy_rin(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDUMMY_RIN_MASK) != 0
    }

    /// Set SPI_MEM_SDUMMY_RIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdummy_rin(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SDUMMY_RIN_MASK);
    }

    /// Clear SPI_MEM_SDUMMY_RIN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdummy_rin(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDUMMY_RIN_MASK);
    }

    /// **SPI_MEM_SCMD_OCT** `[21]` (RW)
    ///
    /// For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_scmd_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SCMD_OCT_MASK) != 0
    }

    /// Set SPI_MEM_SCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_scmd_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SCMD_OCT_MASK);
    }

    /// Clear SPI_MEM_SCMD_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_scmd_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_SCMD_OCT_MASK);
    }

    /// **SPI_MEM_SADDR_OCT** `[20]` (RW)
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_saddr_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SADDR_OCT_MASK) != 0
    }

    /// Set SPI_MEM_SADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_saddr_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SADDR_OCT_MASK);
    }

    /// Clear SPI_MEM_SADDR_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_saddr_oct(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SADDR_OCT_MASK);
    }

    /// **SPI_MEM_SDOUT_OCT** `[19]` (RW)
    ///
    /// For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_sdout_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDOUT_OCT_MASK) != 0
    }

    /// Set SPI_MEM_SDOUT_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdout_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDOUT_OCT_MASK);
    }

    /// Clear SPI_MEM_SDOUT_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdout_oct(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDOUT_OCT_MASK);
    }

    /// **SPI_MEM_SDIN_OCT** `[18]` (RW)
    ///
    /// For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_sdin_oct(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDIN_OCT_MASK) != 0
    }

    /// Set SPI_MEM_SDIN_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdin_oct(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDIN_OCT_MASK);
    }

    /// Clear SPI_MEM_SDIN_OCT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdin_oct(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_SDIN_OCT_MASK);
    }

    /// **SPI_MEM_SCMD_QUAD** `[17]` (RW)
    ///
    /// For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub fn spi_mem_scmd_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SCMD_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_SCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_scmd_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SCMD_QUAD_MASK);
    }

    /// Clear SPI_MEM_SCMD_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_scmd_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SCMD_QUAD_MASK);
    }

    /// **SPI_MEM_SADDR_QUAD** `[16]` (RW)
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub fn spi_mem_saddr_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SADDR_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_SADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_saddr_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SADDR_QUAD_MASK);
    }

    /// Clear SPI_MEM_SADDR_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_saddr_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SADDR_QUAD_MASK);
    }

    /// **SPI_MEM_SDOUT_QUAD** `[15]` (RW)
    ///
    /// For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub fn spi_mem_sdout_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDOUT_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_SDOUT_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdout_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SDOUT_QUAD_MASK);
    }

    /// Clear SPI_MEM_SDOUT_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdout_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDOUT_QUAD_MASK);
    }

    /// **SPI_MEM_SDIN_QUAD** `[14]` (RW)
    ///
    /// For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub fn spi_mem_sdin_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDIN_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_SDIN_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdin_quad(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDIN_QUAD_MASK);
    }

    /// Clear SPI_MEM_SDIN_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdin_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDIN_QUAD_MASK);
    }

    /// **SPI_MEM_SADDR_DUAL** `[12]` (RW)
    ///
    /// For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub fn spi_mem_saddr_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SADDR_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_SADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_saddr_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SADDR_DUAL_MASK);
    }

    /// Clear SPI_MEM_SADDR_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_saddr_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SADDR_DUAL_MASK);
    }

    /// **SPI_MEM_SDOUT_DUAL** `[11]` (RW)
    ///
    /// For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub fn spi_mem_sdout_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDOUT_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_SDOUT_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdout_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SDOUT_DUAL_MASK);
    }

    /// Clear SPI_MEM_SDOUT_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdout_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDOUT_DUAL_MASK);
    }

    /// **SPI_MEM_SDIN_DUAL** `[10]` (RW)
    ///
    /// For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub fn spi_mem_sdin_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SDIN_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_SDIN_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sdin_dual(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_SDIN_DUAL_MASK);
    }

    /// Clear SPI_MEM_SDIN_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sdin_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SDIN_DUAL_MASK);
    }

    /// **SPI_MEM_SWB_MODE** `[9:2]` (RW)
    ///
    /// Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub fn spi_mem_swb_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SWB_MODE_MASK) >> Self::SPI_MEM_SWB_MODE_OFFSET
    }

    /// Write **SPI_MEM_SWB_MODE** `[9:2]` (RW)
    ///
    /// Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub fn write_spi_mem_swb_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SWB_MODE_MASK)
                | ((value << Self::SPI_MEM_SWB_MODE_OFFSET) & Self::SPI_MEM_SWB_MODE_MASK)
        });
    }

    /// **SPI_MEM_SCLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
    #[inline(always)]
    pub fn spi_mem_sclk_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SCLK_MODE_MASK) >> Self::SPI_MEM_SCLK_MODE_OFFSET
    }

    /// Write **SPI_MEM_SCLK_MODE** `[1:0]` (RW)
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
    #[inline(always)]
    pub fn write_spi_mem_sclk_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SCLK_MODE_MASK)
                | ((value << Self::SPI_MEM_SCLK_MODE_OFFSET) & Self::SPI_MEM_SCLK_MODE_MASK)
        });
    }
}

/// SPI_MEM_SRAM_CMD value type for batch read/modify/write operations
///
/// Use `spimemsramcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemsramcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemSramCmdVal {
    bits: u32,
}

impl SpiMemSramCmdVal {
    /// Create from raw bits
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
        Self { bits: 0x80c00000 }
    }

    /// Read **SPI_SMEM_DATA_IE_ALWAYS_ON** `[31:31]`
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub const fn spi_smem_data_ie_always_on(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK) != 0
    }

    /// Read **SPI_SMEM_DQS_IE_ALWAYS_ON** `[30:30]`
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub const fn spi_smem_dqs_ie_always_on(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK) != 0
    }

    /// Read **SPI_MEM_SDOUT_HEX** `[27:27]`
    ///
    /// For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_sdout_hex(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDOUT_HEX_MASK) != 0
    }

    /// Read **SPI_MEM_SDIN_HEX** `[26:26]`
    ///
    /// For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_sdin_hex(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDIN_HEX_MASK) != 0
    }

    /// Read **SPI_SMEM_WDUMMY_ALWAYS_OUT** `[25:25]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_smem_wdummy_always_out(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK) != 0
    }

    /// Read **SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT** `[24:24]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_smem_wdummy_dqs_always_out(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK) != 0
    }

    /// Read **SPI_MEM_SDUMMY_WOUT** `[23:23]`
    ///
    /// In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_sdummy_wout(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDUMMY_WOUT_MASK) != 0
    }

    /// Read **SPI_MEM_SDUMMY_RIN** `[22:22]`
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_sdummy_rin(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDUMMY_RIN_MASK) != 0
    }

    /// Read **SPI_MEM_SCMD_OCT** `[21:21]`
    ///
    /// For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_scmd_oct(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SCMD_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_SADDR_OCT** `[20:20]`
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_saddr_oct(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SADDR_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_SDOUT_OCT** `[19:19]`
    ///
    /// For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_sdout_oct(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDOUT_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_SDIN_OCT** `[18:18]`
    ///
    /// For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_sdin_oct(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDIN_OCT_MASK) != 0
    }

    /// Read **SPI_MEM_SCMD_QUAD** `[17:17]`
    ///
    /// For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn spi_mem_scmd_quad(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SCMD_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_SADDR_QUAD** `[16:16]`
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn spi_mem_saddr_quad(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SADDR_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_SDOUT_QUAD** `[15:15]`
    ///
    /// For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn spi_mem_sdout_quad(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDOUT_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_SDIN_QUAD** `[14:14]`
    ///
    /// For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn spi_mem_sdin_quad(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDIN_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_SADDR_DUAL** `[12:12]`
    ///
    /// For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn spi_mem_saddr_dual(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SADDR_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_SDOUT_DUAL** `[11:11]`
    ///
    /// For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn spi_mem_sdout_dual(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDOUT_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_SDIN_DUAL** `[10:10]`
    ///
    /// For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn spi_mem_sdin_dual(&self) -> bool {
        (self.bits & SpiMemSramCmd::SPI_MEM_SDIN_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_SWB_MODE** `[9:2]`
    ///
    /// Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub const fn spi_mem_swb_mode(&self) -> u32 {
        (self.bits & SpiMemSramCmd::SPI_MEM_SWB_MODE_MASK) >> SpiMemSramCmd::SPI_MEM_SWB_MODE_OFFSET
    }

    /// Read **SPI_MEM_SCLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
    #[inline(always)]
    pub const fn spi_mem_sclk_mode(&self) -> u32 {
        (self.bits & SpiMemSramCmd::SPI_MEM_SCLK_MODE_MASK)
            >> SpiMemSramCmd::SPI_MEM_SCLK_MODE_OFFSET
    }

    /// Set **SPI_SMEM_DATA_IE_ALWAYS_ON** `[31:31]`
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO[7:0] are always 1. 0: Others.
    #[inline(always)]
    pub const fn with_spi_smem_data_ie_always_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_SMEM_DATA_IE_ALWAYS_ON_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DQS_IE_ALWAYS_ON** `[30:30]`
    ///
    /// When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others.
    #[inline(always)]
    pub const fn with_spi_smem_dqs_ie_always_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_SMEM_DQS_IE_ALWAYS_ON_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDOUT_HEX** `[27:27]`
    ///
    /// For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_sdout_hex(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDOUT_HEX_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDOUT_HEX_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDIN_HEX** `[26:26]`
    ///
    /// For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_sdin_hex(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDIN_HEX_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDIN_HEX_MASK
            },
        }
    }

    /// Set **SPI_SMEM_WDUMMY_ALWAYS_OUT** `[25:25]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO[7:0] is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_smem_wdummy_always_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_SMEM_WDUMMY_ALWAYS_OUT_MASK
            },
        }
    }

    /// Set **SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT** `[24:24]`
    ///
    /// In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_smem_wdummy_dqs_always_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDUMMY_WOUT** `[23:23]`
    ///
    /// In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_mem_sdummy_wout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDUMMY_WOUT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDUMMY_WOUT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDUMMY_RIN** `[22:22]`
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn with_spi_mem_sdummy_rin(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDUMMY_RIN_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDUMMY_RIN_MASK
            },
        }
    }

    /// Set **SPI_MEM_SCMD_OCT** `[21:21]`
    ///
    /// For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_scmd_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SCMD_OCT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SCMD_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SADDR_OCT** `[20:20]`
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_saddr_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SADDR_OCT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SADDR_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDOUT_OCT** `[19:19]`
    ///
    /// For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_sdout_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDOUT_OCT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDOUT_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDIN_OCT** `[18:18]`
    ///
    /// For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_sdin_oct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDIN_OCT_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDIN_OCT_MASK
            },
        }
    }

    /// Set **SPI_MEM_SCMD_QUAD** `[17:17]`
    ///
    /// For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn with_spi_mem_scmd_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SCMD_QUAD_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SCMD_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_SADDR_QUAD** `[16:16]`
    ///
    /// For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn with_spi_mem_saddr_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SADDR_QUAD_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SADDR_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDOUT_QUAD** `[15:15]`
    ///
    /// For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn with_spi_mem_sdout_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDOUT_QUAD_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDOUT_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDIN_QUAD** `[14:14]`
    ///
    /// For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio.
    #[inline(always)]
    pub const fn with_spi_mem_sdin_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDIN_QUAD_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDIN_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_SADDR_DUAL** `[12:12]`
    ///
    /// For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn with_spi_mem_saddr_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SADDR_DUAL_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SADDR_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDOUT_DUAL** `[11:11]`
    ///
    /// For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn with_spi_mem_sdout_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDOUT_DUAL_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDOUT_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_SDIN_DUAL** `[10:10]`
    ///
    /// For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio.
    #[inline(always)]
    pub const fn with_spi_mem_sdin_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramCmd::SPI_MEM_SDIN_DUAL_MASK
            } else {
                self.bits & !SpiMemSramCmd::SPI_MEM_SDIN_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_SWB_MODE** `[9:2]`
    ///
    /// Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub const fn with_spi_mem_swb_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramCmd::SPI_MEM_SWB_MODE_MASK)
                | ((value << SpiMemSramCmd::SPI_MEM_SWB_MODE_OFFSET)
                    & SpiMemSramCmd::SPI_MEM_SWB_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_SCLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on.
    #[inline(always)]
    pub const fn with_spi_mem_sclk_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramCmd::SPI_MEM_SCLK_MODE_MASK)
                | ((value << SpiMemSramCmd::SPI_MEM_SCLK_MODE_OFFSET)
                    & SpiMemSramCmd::SPI_MEM_SCLK_MODE_MASK),
        }
    }
}

impl Default for SpiMemSramCmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemSramCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemSramCmdVal {
        SpiMemSramCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemSramCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemSramCmdVal) -> SpiMemSramCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_SRAM_DRD_CMD Register
///
/// SPI0 external RAM DDR read command control register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN** `[31:28]` (RW)
///
///   For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE** `[15:0]` (RW)
///
///   For SPI0,When cache mode is enable it is the read command value of command phase for sram.
#[repr(transparent)]
pub struct SpiMemSramDrdCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemSramDrdCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemSramDrdCmd {
    // ======== Field Constants ========

    /// **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN**
    ///
    /// For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_OFFSET: u32 = 28;
    pub const SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK: u32 = 0xf0000000;

    /// **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE**
    ///
    /// For SPI0,When cache mode is enable it is the read command value of command phase for sram.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_OFFSET: u32 = 0;
    pub const SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN** `[31:28]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_rd_cmd_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK)
            >> Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_OFFSET
    }

    /// Write **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN** `[31:28]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_cache_sram_usr_rd_cmd_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK)
                | ((value << Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_OFFSET)
                    & Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK)
        });
    }

    /// **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE** `[15:0]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the read command value of command phase for sram.
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_rd_cmd_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK)
            >> Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_OFFSET
    }

    /// Write **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE** `[15:0]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the read command value of command phase for sram.
    #[inline(always)]
    pub fn write_spi_mem_cache_sram_usr_rd_cmd_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK)
                | ((value << Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_OFFSET)
                    & Self::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK)
        });
    }
}

/// SPI_MEM_SRAM_DRD_CMD value type for batch read/modify/write operations
///
/// Use `spimemsramdrdcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemsramdrdcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemSramDrdCmdVal {
    bits: u32,
}

impl SpiMemSramDrdCmdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN** `[31:28]`
    ///
    /// For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_rd_cmd_bitlen(&self) -> u32 {
        (self.bits & SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK)
            >> SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_OFFSET
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE** `[15:0]`
    ///
    /// For SPI0,When cache mode is enable it is the read command value of command phase for sram.
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_rd_cmd_value(&self) -> u32 {
        (self.bits & SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK)
            >> SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_OFFSET
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN** `[31:28]`
    ///
    /// For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_rd_cmd_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK)
                | ((value << SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_OFFSET)
                    & SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_BITLEN_MASK),
        }
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE** `[15:0]`
    ///
    /// For SPI0,When cache mode is enable it is the read command value of command phase for sram.
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_rd_cmd_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK)
                | ((value << SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_OFFSET)
                    & SpiMemSramDrdCmd::SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE_MASK),
        }
    }
}

impl Default for SpiMemSramDrdCmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemSramDrdCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemSramDrdCmdVal {
        SpiMemSramDrdCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemSramDrdCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemSramDrdCmdVal) -> SpiMemSramDrdCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_SRAM_DWR_CMD Register
///
/// SPI0 external RAM DDR write command control register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN** `[31:28]` (RW)
///
///   For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
///
/// - **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE** `[15:0]` (RW)
///
///   For SPI0,When cache mode is enable it is the write command value of command phase for sram.
#[repr(transparent)]
pub struct SpiMemSramDwrCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemSramDwrCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemSramDwrCmd {
    // ======== Field Constants ========

    /// **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN**
    ///
    /// For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_OFFSET: u32 = 28;
    pub const SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK: u32 = 0xf0000000;

    /// **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE**
    ///
    /// For SPI0,When cache mode is enable it is the write command value of command phase for sram.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_OFFSET: u32 = 0;
    pub const SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN** `[31:28]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wr_cmd_bitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK)
            >> Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_OFFSET
    }

    /// Write **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN** `[31:28]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_cache_sram_usr_wr_cmd_bitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK)
                | ((value << Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_OFFSET)
                    & Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK)
        });
    }

    /// **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE** `[15:0]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the write command value of command phase for sram.
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wr_cmd_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK)
            >> Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_OFFSET
    }

    /// Write **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE** `[15:0]` (RW)
    ///
    /// For SPI0,When cache mode is enable it is the write command value of command phase for sram.
    #[inline(always)]
    pub fn write_spi_mem_cache_sram_usr_wr_cmd_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK)
                | ((value << Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_OFFSET)
                    & Self::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK)
        });
    }
}

/// SPI_MEM_SRAM_DWR_CMD value type for batch read/modify/write operations
///
/// Use `spimemsramdwrcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemsramdwrcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemSramDwrCmdVal {
    bits: u32,
}

impl SpiMemSramDwrCmdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN** `[31:28]`
    ///
    /// For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_wr_cmd_bitlen(&self) -> u32 {
        (self.bits & SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK)
            >> SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_OFFSET
    }

    /// Read **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE** `[15:0]`
    ///
    /// For SPI0,When cache mode is enable it is the write command value of command phase for sram.
    #[inline(always)]
    pub const fn spi_mem_cache_sram_usr_wr_cmd_value(&self) -> u32 {
        (self.bits & SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK)
            >> SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_OFFSET
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN** `[31:28]`
    ///
    /// For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_wr_cmd_bitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK)
                | ((value << SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_OFFSET)
                    & SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_MASK),
        }
    }

    /// Set **SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE** `[15:0]`
    ///
    /// For SPI0,When cache mode is enable it is the write command value of command phase for sram.
    #[inline(always)]
    pub const fn with_spi_mem_cache_sram_usr_wr_cmd_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK)
                | ((value << SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_OFFSET)
                    & SpiMemSramDwrCmd::SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_MASK),
        }
    }
}

impl Default for SpiMemSramDwrCmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemSramDwrCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemSramDwrCmdVal {
        SpiMemSramDwrCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemSramDwrCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemSramDwrCmdVal) -> SpiMemSramDwrCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_SRAM_CLK Register
///
/// SPI0 external RAM clock control register
///
/// Address offset: 0x0050
/// Reset value: 0x00030103
///
/// ## Fields
///
/// - **SPI_MEM_SCLK_EQU_SYSCLK** `[31]` (RW)
///
///   For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock.
///
/// - **SPI_MEM_SCLKCNT_N** `[23:16]` (RW)
///
///   For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
///
/// - **SPI_MEM_SCLKCNT_H** `[15:8]` (RW)
///
///   For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
///
/// - **SPI_MEM_SCLKCNT_L** `[7:0]` (RW)
///
///   For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
#[repr(transparent)]
pub struct SpiMemSramClk {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemSramClk {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemSramClk {
    // ======== Field Constants ========

    /// **SPI_MEM_SCLK_EQU_SYSCLK**
    ///
    /// For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_SCLK_EQU_SYSCLK_OFFSET: u32 = 31;
    pub const SPI_MEM_SCLK_EQU_SYSCLK_MASK: u32 = 0x80000000;
    pub const SPI_MEM_SCLK_EQU_SYSCLK: u32 = 1 << 31;

    /// **SPI_MEM_SCLKCNT_N**
    ///
    /// For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const SPI_MEM_SCLKCNT_N_OFFSET: u32 = 16;
    pub const SPI_MEM_SCLKCNT_N_MASK: u32 = 0x00ff0000;

    /// **SPI_MEM_SCLKCNT_H**
    ///
    /// For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SPI_MEM_SCLKCNT_H_OFFSET: u32 = 8;
    pub const SPI_MEM_SCLKCNT_H_MASK: u32 = 0x0000ff00;

    /// **SPI_MEM_SCLKCNT_L**
    ///
    /// For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SPI_MEM_SCLKCNT_L_OFFSET: u32 = 0;
    pub const SPI_MEM_SCLKCNT_L_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SPI_MEM_SCLK_EQU_SYSCLK** `[31]` (RW)
    ///
    /// For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock.
    #[inline(always)]
    pub fn spi_mem_sclk_equ_sysclk(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SCLK_EQU_SYSCLK_MASK) != 0
    }

    /// Set SPI_MEM_SCLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_sclk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SCLK_EQU_SYSCLK_MASK);
    }

    /// Clear SPI_MEM_SCLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_sclk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SCLK_EQU_SYSCLK_MASK);
    }

    /// **SPI_MEM_SCLKCNT_N** `[23:16]` (RW)
    ///
    /// For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub fn spi_mem_sclkcnt_n(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SCLKCNT_N_MASK) >> Self::SPI_MEM_SCLKCNT_N_OFFSET
    }

    /// Write **SPI_MEM_SCLKCNT_N** `[23:16]` (RW)
    ///
    /// For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub fn write_spi_mem_sclkcnt_n(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SCLKCNT_N_MASK)
                | ((value << Self::SPI_MEM_SCLKCNT_N_OFFSET) & Self::SPI_MEM_SCLKCNT_N_MASK)
        });
    }

    /// **SPI_MEM_SCLKCNT_H** `[15:8]` (RW)
    ///
    /// For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub fn spi_mem_sclkcnt_h(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SCLKCNT_H_MASK) >> Self::SPI_MEM_SCLKCNT_H_OFFSET
    }

    /// Write **SPI_MEM_SCLKCNT_H** `[15:8]` (RW)
    ///
    /// For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub fn write_spi_mem_sclkcnt_h(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SCLKCNT_H_MASK)
                | ((value << Self::SPI_MEM_SCLKCNT_H_OFFSET) & Self::SPI_MEM_SCLKCNT_H_MASK)
        });
    }

    /// **SPI_MEM_SCLKCNT_L** `[7:0]` (RW)
    ///
    /// For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub fn spi_mem_sclkcnt_l(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SCLKCNT_L_MASK) >> Self::SPI_MEM_SCLKCNT_L_OFFSET
    }

    /// Write **SPI_MEM_SCLKCNT_L** `[7:0]` (RW)
    ///
    /// For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub fn write_spi_mem_sclkcnt_l(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SCLKCNT_L_MASK)
                | ((value << Self::SPI_MEM_SCLKCNT_L_OFFSET) & Self::SPI_MEM_SCLKCNT_L_MASK)
        });
    }
}

/// SPI_MEM_SRAM_CLK value type for batch read/modify/write operations
///
/// Use `spimemsramclk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemsramclk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemSramClkVal {
    bits: u32,
}

impl SpiMemSramClkVal {
    /// Create from raw bits
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
        Self { bits: 0x00030103 }
    }

    /// Read **SPI_MEM_SCLK_EQU_SYSCLK** `[31:31]`
    ///
    /// For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock.
    #[inline(always)]
    pub const fn spi_mem_sclk_equ_sysclk(&self) -> bool {
        (self.bits & SpiMemSramClk::SPI_MEM_SCLK_EQU_SYSCLK_MASK) != 0
    }

    /// Read **SPI_MEM_SCLKCNT_N** `[23:16]`
    ///
    /// For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub const fn spi_mem_sclkcnt_n(&self) -> u32 {
        (self.bits & SpiMemSramClk::SPI_MEM_SCLKCNT_N_MASK)
            >> SpiMemSramClk::SPI_MEM_SCLKCNT_N_OFFSET
    }

    /// Read **SPI_MEM_SCLKCNT_H** `[15:8]`
    ///
    /// For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub const fn spi_mem_sclkcnt_h(&self) -> u32 {
        (self.bits & SpiMemSramClk::SPI_MEM_SCLKCNT_H_MASK)
            >> SpiMemSramClk::SPI_MEM_SCLKCNT_H_OFFSET
    }

    /// Read **SPI_MEM_SCLKCNT_L** `[7:0]`
    ///
    /// For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub const fn spi_mem_sclkcnt_l(&self) -> u32 {
        (self.bits & SpiMemSramClk::SPI_MEM_SCLKCNT_L_MASK)
            >> SpiMemSramClk::SPI_MEM_SCLKCNT_L_OFFSET
    }

    /// Set **SPI_MEM_SCLK_EQU_SYSCLK** `[31:31]`
    ///
    /// For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock.
    #[inline(always)]
    pub const fn with_spi_mem_sclk_equ_sysclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSramClk::SPI_MEM_SCLK_EQU_SYSCLK_MASK
            } else {
                self.bits & !SpiMemSramClk::SPI_MEM_SCLK_EQU_SYSCLK_MASK
            },
        }
    }

    /// Set **SPI_MEM_SCLKCNT_N** `[23:16]`
    ///
    /// For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)
    #[inline(always)]
    pub const fn with_spi_mem_sclkcnt_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramClk::SPI_MEM_SCLKCNT_N_MASK)
                | ((value << SpiMemSramClk::SPI_MEM_SCLKCNT_N_OFFSET)
                    & SpiMemSramClk::SPI_MEM_SCLKCNT_N_MASK),
        }
    }

    /// Set **SPI_MEM_SCLKCNT_H** `[15:8]`
    ///
    /// For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1).
    #[inline(always)]
    pub const fn with_spi_mem_sclkcnt_h(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramClk::SPI_MEM_SCLKCNT_H_MASK)
                | ((value << SpiMemSramClk::SPI_MEM_SCLKCNT_H_OFFSET)
                    & SpiMemSramClk::SPI_MEM_SCLKCNT_H_MASK),
        }
    }

    /// Set **SPI_MEM_SCLKCNT_L** `[7:0]`
    ///
    /// For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N.
    #[inline(always)]
    pub const fn with_spi_mem_sclkcnt_l(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSramClk::SPI_MEM_SCLKCNT_L_MASK)
                | ((value << SpiMemSramClk::SPI_MEM_SCLKCNT_L_OFFSET)
                    & SpiMemSramClk::SPI_MEM_SCLKCNT_L_MASK),
        }
    }
}

impl Default for SpiMemSramClkVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemSramClk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemSramClkVal {
        SpiMemSramClkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemSramClkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemSramClkVal) -> SpiMemSramClkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_FSM Register
///
/// SPI0 FSM status register
///
/// Address offset: 0x0054
/// Reset value: 0x00000200
///
/// ## Fields
///
/// - **SPI_MEM_LOCK_DELAY_TIME** `[11:7]` (RW)
///
///   The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
#[repr(transparent)]
pub struct SpiMemFsm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemFsm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemFsm {
    // ======== Field Constants ========

    /// **SPI_MEM_LOCK_DELAY_TIME**
    ///
    /// The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const SPI_MEM_LOCK_DELAY_TIME_OFFSET: u32 = 7;
    pub const SPI_MEM_LOCK_DELAY_TIME_MASK: u32 = 0x00000f80;

    // ======== Field Accessors ========

    /// **SPI_MEM_LOCK_DELAY_TIME** `[11:7]` (RW)
    ///
    /// The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
    #[inline(always)]
    pub fn spi_mem_lock_delay_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_LOCK_DELAY_TIME_MASK)
            >> Self::SPI_MEM_LOCK_DELAY_TIME_OFFSET
    }

    /// Write **SPI_MEM_LOCK_DELAY_TIME** `[11:7]` (RW)
    ///
    /// The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
    #[inline(always)]
    pub fn write_spi_mem_lock_delay_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_LOCK_DELAY_TIME_MASK)
                | ((value << Self::SPI_MEM_LOCK_DELAY_TIME_OFFSET)
                    & Self::SPI_MEM_LOCK_DELAY_TIME_MASK)
        });
    }
}

/// SPI_MEM_FSM value type for batch read/modify/write operations
///
/// Use `spimemfsm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemfsm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemFsmVal {
    bits: u32,
}

impl SpiMemFsmVal {
    /// Create from raw bits
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
        Self { bits: 0x00000200 }
    }

    /// Read **SPI_MEM_LOCK_DELAY_TIME** `[11:7]`
    ///
    /// The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
    #[inline(always)]
    pub const fn spi_mem_lock_delay_time(&self) -> u32 {
        (self.bits & SpiMemFsm::SPI_MEM_LOCK_DELAY_TIME_MASK)
            >> SpiMemFsm::SPI_MEM_LOCK_DELAY_TIME_OFFSET
    }

    /// Set **SPI_MEM_LOCK_DELAY_TIME** `[11:7]`
    ///
    /// The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1.
    #[inline(always)]
    pub const fn with_spi_mem_lock_delay_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFsm::SPI_MEM_LOCK_DELAY_TIME_MASK)
                | ((value << SpiMemFsm::SPI_MEM_LOCK_DELAY_TIME_OFFSET)
                    & SpiMemFsm::SPI_MEM_LOCK_DELAY_TIME_MASK),
        }
    }
}

impl Default for SpiMemFsmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemFsm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemFsmVal {
        SpiMemFsmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemFsmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemFsmVal) -> SpiMemFsmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_INT_ENA Register
///
/// SPI0 interrupt enable register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **SPI_MEM_BUS_FIFO0_UDF_INT_ENA** `[31]` (RW)
///
///   The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
///
/// - **SPI_MEM_BUS_FIFO1_UDF_INT_ENA** `[30]` (RW)
///
///   The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
///
/// - **SPI_MEM_DQS1_AFIFO_OVF_INT_ENA** `[29]` (RW)
///
///   The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_DQS0_AFIFO_OVF_INT_ENA** `[28]` (RW)
///
///   The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_AXI_WADDR_ERR_INT__ENA** `[9]` (RW)
///
///   The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA** `[8]` (RW)
///
///   The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_RADDR_ERR_INT_ENA** `[7]` (RW)
///
///   The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_PMS_REJECT_INT_ENA** `[6]` (RW)
///
///   The enable bit for SPI_MEM_PMS_REJECT_INT interrupt.
///
/// - **SPI_MEM_ECC_ERR_INT_ENA** `[5]` (RW)
///
///   The enable bit for SPI_MEM_ECC_ERR_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_ENA** `[4]` (RW)
///
///   The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_ENA** `[3]` (RW)
///
///   The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
#[repr(transparent)]
pub struct SpiMemIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemIntEna {
    // ======== Field Constants ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ENA_OFFSET: u32 = 31;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK: u32 = 0x80000000;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ENA: u32 = 1 << 31;

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ENA_OFFSET: u32 = 30;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK: u32 = 0x40000000;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ENA: u32 = 1 << 30;

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_OFFSET: u32 = 29;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK: u32 = 0x20000000;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ENA: u32 = 1 << 29;

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_OFFSET: u32 = 28;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK: u32 = 0x10000000;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ENA: u32 = 1 << 28;

    /// **SPI_MEM_AXI_WADDR_ERR_INT__ENA**
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ENA_OFFSET: u32 = 9;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK: u32 = 0x00000200;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ENA: u32 = 1 << 9;

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_OFFSET: u32 = 8;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK: u32 = 0x00000100;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA: u32 = 1 << 8;

    /// **SPI_MEM_AXI_RADDR_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ENA_OFFSET: u32 = 7;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK: u32 = 0x00000080;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ENA: u32 = 1 << 7;

    /// **SPI_MEM_PMS_REJECT_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_PMS_REJECT_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_PMS_REJECT_INT_ENA_OFFSET: u32 = 6;
    pub const SPI_MEM_PMS_REJECT_INT_ENA_MASK: u32 = 0x00000040;
    pub const SPI_MEM_PMS_REJECT_INT_ENA: u32 = 1 << 6;

    /// **SPI_MEM_ECC_ERR_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_ECC_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_ECC_ERR_INT_ENA_OFFSET: u32 = 5;
    pub const SPI_MEM_ECC_ERR_INT_ENA_MASK: u32 = 0x00000020;
    pub const SPI_MEM_ECC_ERR_INT_ENA: u32 = 1 << 5;

    /// **SPI_MEM_MST_ST_END_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_MST_ST_END_INT_ENA_OFFSET: u32 = 4;
    pub const SPI_MEM_MST_ST_END_INT_ENA_MASK: u32 = 0x00000010;
    pub const SPI_MEM_MST_ST_END_INT_ENA: u32 = 1 << 4;

    /// **SPI_MEM_SLV_ST_END_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_SLV_ST_END_INT_ENA_OFFSET: u32 = 3;
    pub const SPI_MEM_SLV_ST_END_INT_ENA_MASK: u32 = 0x00000008;
    pub const SPI_MEM_SLV_ST_END_INT_ENA: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_ENA** `[31]` (RW)
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_bus_fifo0_udf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_BUS_FIFO0_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_bus_fifo0_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_BUS_FIFO0_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_bus_fifo0_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK);
    }

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_ENA** `[30]` (RW)
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_bus_fifo1_udf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_BUS_FIFO1_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_bus_fifo1_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_BUS_FIFO1_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_bus_fifo1_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK);
    }

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_ENA** `[29]` (RW)
    ///
    /// The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_dqs1_afifo_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_DQS1_AFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dqs1_afifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_DQS1_AFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dqs1_afifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK);
    }

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_ENA** `[28]` (RW)
    ///
    /// The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_dqs0_afifo_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_DQS0_AFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dqs0_afifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_DQS0_AFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dqs0_afifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK);
    }

    /// **SPI_MEM_AXI_WADDR_ERR_INT__ENA** `[9]` (RW)
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_AXI_WADDR_ERR_INT__ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_waddr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_AXI_WADDR_ERR_INT__ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_waddr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK);
    }

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA** `[8]` (RW)
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_wr_flash_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_wr_flash_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK);
    }

    /// **SPI_MEM_AXI_RADDR_ERR_INT_ENA** `[7]` (RW)
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_AXI_RADDR_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_raddr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_AXI_RADDR_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_raddr_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK);
    }

    /// **SPI_MEM_PMS_REJECT_INT_ENA** `[6]` (RW)
    ///
    /// The enable bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_REJECT_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_PMS_REJECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pms_reject_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PMS_REJECT_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_PMS_REJECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pms_reject_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PMS_REJECT_INT_ENA_MASK);
    }

    /// **SPI_MEM_ECC_ERR_INT_ENA** `[5]` (RW)
    ///
    /// The enable bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_ECC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ecc_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_ECC_ERR_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_ECC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ecc_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_ECC_ERR_INT_ENA_MASK);
    }

    /// **SPI_MEM_MST_ST_END_INT_ENA** `[4]` (RW)
    ///
    /// The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_MST_ST_END_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_MST_ST_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_mst_st_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_MST_ST_END_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_MST_ST_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_mst_st_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_MST_ST_END_INT_ENA_MASK);
    }

    /// **SPI_MEM_SLV_ST_END_INT_ENA** `[3]` (RW)
    ///
    /// The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SLV_ST_END_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_SLV_ST_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_slv_st_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SLV_ST_END_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_SLV_ST_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_slv_st_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SLV_ST_END_INT_ENA_MASK);
    }
}

/// SPI_MEM_INT_ENA value type for batch read/modify/write operations
///
/// Use `spimemintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemIntEnaVal {
    bits: u32,
}

impl SpiMemIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUS_FIFO0_UDF_INT_ENA** `[31:31]`
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo0_udf_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_BUS_FIFO1_UDF_INT_ENA** `[30:30]`
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo1_udf_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_DQS1_AFIFO_OVF_INT_ENA** `[29:29]`
    ///
    /// The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_dqs1_afifo_ovf_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_DQS0_AFIFO_OVF_INT_ENA** `[28:28]`
    ///
    /// The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_dqs0_afifo_ovf_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WADDR_ERR_INT__ENA** `[9:9]`
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_waddr_err_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA** `[8:8]`
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_wr_flash_err_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_RADDR_ERR_INT_ENA** `[7:7]`
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_raddr_err_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_REJECT_INT_ENA** `[6:6]`
    ///
    /// The enable bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_pms_reject_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_PMS_REJECT_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_ERR_INT_ENA** `[5:5]`
    ///
    /// The enable bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_ecc_err_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_ECC_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_MST_ST_END_INT_ENA** `[4:4]`
    ///
    /// The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_mst_st_end_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_MST_ST_END_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST_END_INT_ENA** `[3:3]`
    ///
    /// The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_slv_st_end_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_SLV_ST_END_INT_ENA_MASK) != 0
    }

    /// Set **SPI_MEM_BUS_FIFO0_UDF_INT_ENA** `[31:31]`
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo0_udf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_BUS_FIFO0_UDF_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_BUS_FIFO1_UDF_INT_ENA** `[30:30]`
    ///
    /// The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo1_udf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_BUS_FIFO1_UDF_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS1_AFIFO_OVF_INT_ENA** `[29:29]`
    ///
    /// The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_dqs1_afifo_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_DQS1_AFIFO_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS0_AFIFO_OVF_INT_ENA** `[28:28]`
    ///
    /// The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_dqs0_afifo_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_DQS0_AFIFO_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WADDR_ERR_INT__ENA** `[9:9]`
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_waddr_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_AXI_WADDR_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA** `[8:8]`
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_wr_flash_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_RADDR_ERR_INT_ENA** `[7:7]`
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_raddr_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_AXI_RADDR_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_PMS_REJECT_INT_ENA** `[6:6]`
    ///
    /// The enable bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_pms_reject_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_PMS_REJECT_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_PMS_REJECT_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_ERR_INT_ENA** `[5:5]`
    ///
    /// The enable bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_ECC_ERR_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_ECC_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_MST_ST_END_INT_ENA** `[4:4]`
    ///
    /// The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_mst_st_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_MST_ST_END_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_MST_ST_END_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_SLV_ST_END_INT_ENA** `[3:3]`
    ///
    /// The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_slv_st_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_SLV_ST_END_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_SLV_ST_END_INT_ENA_MASK
            },
        }
    }
}

impl Default for SpiMemIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemIntEnaVal {
        SpiMemIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemIntEnaVal) -> SpiMemIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_INT_CLR Register
///
/// SPI0 interrupt clear register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **SPI_MEM_BUS_FIFO0_UDF_INT_CLR** `[31]` (WO)
///
///   The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
///
/// - **SPI_MEM_BUS_FIFO1_UDF_INT_CLR** `[30]` (WO)
///
///   The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
///
/// - **SPI_MEM_DQS1_AFIFO_OVF_INT_CLR** `[29]` (WO)
///
///   The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_DQS0_AFIFO_OVF_INT_CLR** `[28]` (WO)
///
///   The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_AXI_WADDR_ERR_INT_CLR** `[9]` (WO)
///
///   The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR** `[8]` (WO)
///
///   The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_RADDR_ERR_INT_CLR** `[7]` (WO)
///
///   The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_PMS_REJECT_INT_CLR** `[6]` (WO)
///
///   The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
///
/// - **SPI_MEM_ECC_ERR_INT_CLR** `[5]` (WO)
///
///   The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_CLR** `[4]` (WO)
///
///   The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_CLR** `[3]` (WO)
///
///   The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
#[repr(transparent)]
pub struct SpiMemIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemIntClr {
    // ======== Field Constants ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_CLR_OFFSET: u32 = 31;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_CLR_MASK: u32 = 0x80000000;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_CLR: u32 = 1 << 31;

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_CLR_OFFSET: u32 = 30;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_CLR_MASK: u32 = 0x40000000;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_CLR: u32 = 1 << 30;

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_CLR_OFFSET: u32 = 29;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_CLR_MASK: u32 = 0x20000000;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_CLR: u32 = 1 << 29;

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_CLR_OFFSET: u32 = 28;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_CLR_MASK: u32 = 0x10000000;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_CLR: u32 = 1 << 28;

    /// **SPI_MEM_AXI_WADDR_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_AXI_WADDR_ERR_INT_CLR_OFFSET: u32 = 9;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_CLR_MASK: u32 = 0x00000200;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_CLR: u32 = 1 << 9;

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_OFFSET: u32 = 8;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_MASK: u32 = 0x00000100;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR: u32 = 1 << 8;

    /// **SPI_MEM_AXI_RADDR_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_AXI_RADDR_ERR_INT_CLR_OFFSET: u32 = 7;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_CLR_MASK: u32 = 0x00000080;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_CLR: u32 = 1 << 7;

    /// **SPI_MEM_PMS_REJECT_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_PMS_REJECT_INT_CLR_OFFSET: u32 = 6;
    pub const SPI_MEM_PMS_REJECT_INT_CLR_MASK: u32 = 0x00000040;
    pub const SPI_MEM_PMS_REJECT_INT_CLR: u32 = 1 << 6;

    /// **SPI_MEM_ECC_ERR_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_ECC_ERR_INT_CLR_OFFSET: u32 = 5;
    pub const SPI_MEM_ECC_ERR_INT_CLR_MASK: u32 = 0x00000020;
    pub const SPI_MEM_ECC_ERR_INT_CLR: u32 = 1 << 5;

    /// **SPI_MEM_MST_ST_END_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_MST_ST_END_INT_CLR_OFFSET: u32 = 4;
    pub const SPI_MEM_MST_ST_END_INT_CLR_MASK: u32 = 0x00000010;
    pub const SPI_MEM_MST_ST_END_INT_CLR: u32 = 1 << 4;

    /// **SPI_MEM_SLV_ST_END_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_SLV_ST_END_INT_CLR_OFFSET: u32 = 3;
    pub const SPI_MEM_SLV_ST_END_INT_CLR_MASK: u32 = 0x00000008;
    pub const SPI_MEM_SLV_ST_END_INT_CLR: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// Trigger **SPI_MEM_BUS_FIFO0_UDF_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_bus_fifo0_udf_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_BUS_FIFO0_UDF_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_BUS_FIFO1_UDF_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_bus_fifo1_udf_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_BUS_FIFO1_UDF_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_DQS1_AFIFO_OVF_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_dqs1_afifo_ovf_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_DQS1_AFIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_DQS0_AFIFO_OVF_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_dqs0_afifo_ovf_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_DQS0_AFIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_AXI_WADDR_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_axi_waddr_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_AXI_WADDR_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_axi_wr_flash_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_AXI_RADDR_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_axi_raddr_err_int_clr(&self) {
        self.value
            .write_bits(Self::SPI_MEM_AXI_RADDR_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_PMS_REJECT_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_pms_reject_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_PMS_REJECT_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_ECC_ERR_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_ecc_err_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_ECC_ERR_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_MST_ST_END_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_mst_st_end_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_MST_ST_END_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_SLV_ST_END_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_slv_st_end_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_SLV_ST_END_INT_CLR_MASK);
    }
}

/// SPI_MEM_INT_CLR value type for batch read/modify/write operations
///
/// Use `spimemintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemIntClrVal {
    bits: u32,
}

impl SpiMemIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_MEM_BUS_FIFO0_UDF_INT_CLR** `[31:31]`
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo0_udf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_BUS_FIFO0_UDF_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_BUS_FIFO0_UDF_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_BUS_FIFO1_UDF_INT_CLR** `[30:30]`
    ///
    /// The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo1_udf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_BUS_FIFO1_UDF_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_BUS_FIFO1_UDF_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS1_AFIFO_OVF_INT_CLR** `[29:29]`
    ///
    /// The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_dqs1_afifo_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_DQS1_AFIFO_OVF_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_DQS1_AFIFO_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS0_AFIFO_OVF_INT_CLR** `[28:28]`
    ///
    /// The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_dqs0_afifo_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_DQS0_AFIFO_OVF_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_DQS0_AFIFO_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WADDR_ERR_INT_CLR** `[9:9]`
    ///
    /// The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_waddr_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_AXI_WADDR_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_AXI_WADDR_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR** `[8:8]`
    ///
    /// The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_wr_flash_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_RADDR_ERR_INT_CLR** `[7:7]`
    ///
    /// The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_axi_raddr_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_AXI_RADDR_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_AXI_RADDR_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_PMS_REJECT_INT_CLR** `[6:6]`
    ///
    /// The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_pms_reject_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_PMS_REJECT_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_PMS_REJECT_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_ERR_INT_CLR** `[5:5]`
    ///
    /// The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_ECC_ERR_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_ECC_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_MST_ST_END_INT_CLR** `[4:4]`
    ///
    /// The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_mst_st_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_MST_ST_END_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_MST_ST_END_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_SLV_ST_END_INT_CLR** `[3:3]`
    ///
    /// The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_slv_st_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_SLV_ST_END_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_SLV_ST_END_INT_CLR_MASK
            },
        }
    }
}

impl Default for SpiMemIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_MEM_INT_RAW Register
///
/// SPI0 interrupt raw register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **SPI_MEM_BUS_FIFO0_UDF_INT_RAW** `[31]` (RW)
///
///   The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
///
/// - **SPI_MEM_BUS_FIFO1_UDF_INT_RAW** `[30]` (RW)
///
///   The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
///
/// - **SPI_MEM_DQS1_AFIFO_OVF_INT_RAW** `[29]` (RW)
///
///   The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
///
/// - **SPI_MEM_DQS0_AFIFO_OVF_INT_RAW** `[28]` (RW)
///
///   The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
///
/// - **SPI_MEM_AXI_WADDR_ERR_INT_RAW** `[9]` (RW)
///
///   The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
///
/// - **SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW** `[8]` (RW)
///
///   The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
///
/// - **SPI_MEM_AXI_RADDR_ERR_INT_RAW** `[7]` (RW)
///
///   The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
///
/// - **SPI_MEM_PMS_REJECT_INT_RAW** `[6]` (RW)
///
///   The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
///
/// - **SPI_MEM_ECC_ERR_INT_RAW** `[5]` (RW)
///
///   The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
///
/// - **SPI_MEM_MST_ST_END_INT_RAW** `[4]` (RW)
///
///   The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
///
/// - **SPI_MEM_SLV_ST_END_INT_RAW** `[3]` (RW)
///
///   The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
#[repr(transparent)]
pub struct SpiMemIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemIntRaw {
    // ======== Field Constants ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_RAW_OFFSET: u32 = 31;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK: u32 = 0x80000000;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_RAW: u32 = 1 << 31;

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_RAW_OFFSET: u32 = 30;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK: u32 = 0x40000000;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_RAW: u32 = 1 << 30;

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_OFFSET: u32 = 29;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK: u32 = 0x20000000;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_RAW: u32 = 1 << 29;

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_OFFSET: u32 = 28;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK: u32 = 0x10000000;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_RAW: u32 = 1 << 28;

    /// **SPI_MEM_AXI_WADDR_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_AXI_WADDR_ERR_INT_RAW_OFFSET: u32 = 9;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK: u32 = 0x00000200;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_RAW: u32 = 1 << 9;

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_OFFSET: u32 = 8;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK: u32 = 0x00000100;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW: u32 = 1 << 8;

    /// **SPI_MEM_AXI_RADDR_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_AXI_RADDR_ERR_INT_RAW_OFFSET: u32 = 7;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK: u32 = 0x00000080;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_RAW: u32 = 1 << 7;

    /// **SPI_MEM_PMS_REJECT_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_PMS_REJECT_INT_RAW_OFFSET: u32 = 6;
    pub const SPI_MEM_PMS_REJECT_INT_RAW_MASK: u32 = 0x00000040;
    pub const SPI_MEM_PMS_REJECT_INT_RAW: u32 = 1 << 6;

    /// **SPI_MEM_ECC_ERR_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_ECC_ERR_INT_RAW_OFFSET: u32 = 5;
    pub const SPI_MEM_ECC_ERR_INT_RAW_MASK: u32 = 0x00000020;
    pub const SPI_MEM_ECC_ERR_INT_RAW: u32 = 1 << 5;

    /// **SPI_MEM_MST_ST_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_MST_ST_END_INT_RAW_OFFSET: u32 = 4;
    pub const SPI_MEM_MST_ST_END_INT_RAW_MASK: u32 = 0x00000010;
    pub const SPI_MEM_MST_ST_END_INT_RAW: u32 = 1 << 4;

    /// **SPI_MEM_SLV_ST_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_SLV_ST_END_INT_RAW_OFFSET: u32 = 3;
    pub const SPI_MEM_SLV_ST_END_INT_RAW_MASK: u32 = 0x00000008;
    pub const SPI_MEM_SLV_ST_END_INT_RAW: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_RAW** `[31]` (RW)
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    #[inline(always)]
    pub fn spi_mem_bus_fifo0_udf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_BUS_FIFO0_UDF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_bus_fifo0_udf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_BUS_FIFO0_UDF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_bus_fifo0_udf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK);
    }

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_RAW** `[30]` (RW)
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    #[inline(always)]
    pub fn spi_mem_bus_fifo1_udf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_BUS_FIFO1_UDF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_bus_fifo1_udf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_BUS_FIFO1_UDF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_bus_fifo1_udf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK);
    }

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_RAW** `[29]` (RW)
    ///
    /// The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    #[inline(always)]
    pub fn spi_mem_dqs1_afifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_DQS1_AFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dqs1_afifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_DQS1_AFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dqs1_afifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK);
    }

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_RAW** `[28]` (RW)
    ///
    /// The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    #[inline(always)]
    pub fn spi_mem_dqs0_afifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_DQS0_AFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dqs0_afifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_DQS0_AFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dqs0_afifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK);
    }

    /// **SPI_MEM_AXI_WADDR_ERR_INT_RAW** `[9]` (RW)
    ///
    /// The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_AXI_WADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_waddr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_AXI_WADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_waddr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK);
    }

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW** `[8]` (RW)
    ///
    /// The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_wr_flash_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_wr_flash_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK);
    }

    /// **SPI_MEM_AXI_RADDR_ERR_INT_RAW** `[7]` (RW)
    ///
    /// The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_AXI_RADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_axi_raddr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_AXI_RADDR_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_axi_raddr_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK);
    }

    /// **SPI_MEM_PMS_REJECT_INT_RAW** `[6]` (RW)
    ///
    /// The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_REJECT_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_PMS_REJECT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pms_reject_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PMS_REJECT_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_PMS_REJECT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pms_reject_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PMS_REJECT_INT_RAW_MASK);
    }

    /// **SPI_MEM_ECC_ERR_INT_RAW** `[5]` (RW)
    ///
    /// The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_ECC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ecc_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_ECC_ERR_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_ECC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ecc_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_ECC_ERR_INT_RAW_MASK);
    }

    /// **SPI_MEM_MST_ST_END_INT_RAW** `[4]` (RW)
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_MST_ST_END_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_MST_ST_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_mst_st_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_MST_ST_END_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_MST_ST_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_mst_st_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_MST_ST_END_INT_RAW_MASK);
    }

    /// **SPI_MEM_SLV_ST_END_INT_RAW** `[3]` (RW)
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SLV_ST_END_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_SLV_ST_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_slv_st_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SLV_ST_END_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_SLV_ST_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_slv_st_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SLV_ST_END_INT_RAW_MASK);
    }
}

/// SPI_MEM_INT_RAW value type for batch read/modify/write operations
///
/// Use `spimemintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemIntRawVal {
    bits: u32,
}

impl SpiMemIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUS_FIFO0_UDF_INT_RAW** `[31:31]`
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo0_udf_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_BUS_FIFO1_UDF_INT_RAW** `[30:30]`
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo1_udf_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_DQS1_AFIFO_OVF_INT_RAW** `[29:29]`
    ///
    /// The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    #[inline(always)]
    pub const fn spi_mem_dqs1_afifo_ovf_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_DQS0_AFIFO_OVF_INT_RAW** `[28:28]`
    ///
    /// The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    #[inline(always)]
    pub const fn spi_mem_dqs0_afifo_ovf_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WADDR_ERR_INT_RAW** `[9:9]`
    ///
    /// The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_axi_waddr_err_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW** `[8:8]`
    ///
    /// The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_axi_wr_flash_err_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_RADDR_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_axi_raddr_err_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_REJECT_INT_RAW** `[6:6]`
    ///
    /// The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_pms_reject_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_PMS_REJECT_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    #[inline(always)]
    pub const fn spi_mem_ecc_err_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_ECC_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_MST_ST_END_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_mst_st_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_MST_ST_END_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST_END_INT_RAW** `[3:3]`
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    pub const fn spi_mem_slv_st_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_SLV_ST_END_INT_RAW_MASK) != 0
    }

    /// Set **SPI_MEM_BUS_FIFO0_UDF_INT_RAW** `[31:31]`
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo0_udf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_BUS_FIFO0_UDF_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_BUS_FIFO1_UDF_INT_RAW** `[30:30]`
    ///
    /// The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    #[inline(always)]
    pub const fn with_spi_mem_bus_fifo1_udf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_BUS_FIFO1_UDF_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS1_AFIFO_OVF_INT_RAW** `[29:29]`
    ///
    /// The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    #[inline(always)]
    pub const fn with_spi_mem_dqs1_afifo_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_DQS0_AFIFO_OVF_INT_RAW** `[28:28]`
    ///
    /// The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    #[inline(always)]
    pub const fn with_spi_mem_dqs0_afifo_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WADDR_ERR_INT_RAW** `[9:9]`
    ///
    /// The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_axi_waddr_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_AXI_WADDR_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW** `[8:8]`
    ///
    /// The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_axi_wr_flash_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_AXI_RADDR_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_axi_raddr_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_AXI_RADDR_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_PMS_REJECT_INT_RAW** `[6:6]`
    ///
    /// The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_pms_reject_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_PMS_REJECT_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_PMS_REJECT_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_ECC_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_ECC_ERR_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_ECC_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_MST_ST_END_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_mst_st_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_MST_ST_END_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_MST_ST_END_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_SLV_ST_END_INT_RAW** `[3:3]`
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    pub const fn with_spi_mem_slv_st_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_SLV_ST_END_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_SLV_ST_END_INT_RAW_MASK
            },
        }
    }
}

impl Default for SpiMemIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemIntRawVal {
        SpiMemIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemIntRawVal) -> SpiMemIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_INT_ST Register
///
/// SPI0 interrupt status register
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **SPI_MEM_BUS_FIFO0_UDF_INT_ST** `[31]` (RO)
///
///   The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
///
/// - **SPI_MEM_BUS_FIFO1_UDF_INT_ST** `[30]` (RO)
///
///   The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
///
/// - **SPI_MEM_DQS1_AFIFO_OVF_INT_ST** `[29]` (RO)
///
///   The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_DQS0_AFIFO_OVF_INT_ST** `[28]` (RO)
///
///   The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
///
/// - **SPI_MEM_AXI_WADDR_ERR_INT_ST** `[9]` (RO)
///
///   The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_WR_FLASH_ERR_INT_ST** `[8]` (RO)
///
///   The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
///
/// - **SPI_MEM_AXI_RADDR_ERR_INT_ST** `[7]` (RO)
///
///   The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
///
/// - **SPI_MEM_PMS_REJECT_INT_ST** `[6]` (RO)
///
///   The status bit for SPI_MEM_PMS_REJECT_INT interrupt.
///
/// - **SPI_MEM_ECC_ERR_INT_ST** `[5]` (RO)
///
///   The status bit for SPI_MEM_ECC_ERR_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_ST** `[4]` (RO)
///
///   The status bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_ST** `[3]` (RO)
///
///   The status bit for SPI_MEM_SLV_ST_END_INT interrupt.
#[repr(transparent)]
pub struct SpiMemIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemIntSt {
    // ======== Field Constants ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_ST**
    ///
    /// The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ST_OFFSET: u32 = 31;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ST_MASK: u32 = 0x80000000;
    pub const SPI_MEM_BUS_FIFO0_UDF_INT_ST: u32 = 1 << 31;

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_ST**
    ///
    /// The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ST_OFFSET: u32 = 30;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ST_MASK: u32 = 0x40000000;
    pub const SPI_MEM_BUS_FIFO1_UDF_INT_ST: u32 = 1 << 30;

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_ST**
    ///
    /// The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ST_OFFSET: u32 = 29;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ST_MASK: u32 = 0x20000000;
    pub const SPI_MEM_DQS1_AFIFO_OVF_INT_ST: u32 = 1 << 29;

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_ST**
    ///
    /// The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ST_OFFSET: u32 = 28;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ST_MASK: u32 = 0x10000000;
    pub const SPI_MEM_DQS0_AFIFO_OVF_INT_ST: u32 = 1 << 28;

    /// **SPI_MEM_AXI_WADDR_ERR_INT_ST**
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ST_OFFSET: u32 = 9;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ST_MASK: u32 = 0x00000200;
    pub const SPI_MEM_AXI_WADDR_ERR_INT_ST: u32 = 1 << 9;

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_ST**
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_OFFSET: u32 = 8;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_MASK: u32 = 0x00000100;
    pub const SPI_MEM_AXI_WR_FLASH_ERR_INT_ST: u32 = 1 << 8;

    /// **SPI_MEM_AXI_RADDR_ERR_INT_ST**
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ST_OFFSET: u32 = 7;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ST_MASK: u32 = 0x00000080;
    pub const SPI_MEM_AXI_RADDR_ERR_INT_ST: u32 = 1 << 7;

    /// **SPI_MEM_PMS_REJECT_INT_ST**
    ///
    /// The status bit for SPI_MEM_PMS_REJECT_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_PMS_REJECT_INT_ST_OFFSET: u32 = 6;
    pub const SPI_MEM_PMS_REJECT_INT_ST_MASK: u32 = 0x00000040;
    pub const SPI_MEM_PMS_REJECT_INT_ST: u32 = 1 << 6;

    /// **SPI_MEM_ECC_ERR_INT_ST**
    ///
    /// The status bit for SPI_MEM_ECC_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_ECC_ERR_INT_ST_OFFSET: u32 = 5;
    pub const SPI_MEM_ECC_ERR_INT_ST_MASK: u32 = 0x00000020;
    pub const SPI_MEM_ECC_ERR_INT_ST: u32 = 1 << 5;

    /// **SPI_MEM_MST_ST_END_INT_ST**
    ///
    /// The status bit for SPI_MEM_MST_ST_END_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_MST_ST_END_INT_ST_OFFSET: u32 = 4;
    pub const SPI_MEM_MST_ST_END_INT_ST_MASK: u32 = 0x00000010;
    pub const SPI_MEM_MST_ST_END_INT_ST: u32 = 1 << 4;

    /// **SPI_MEM_SLV_ST_END_INT_ST**
    ///
    /// The status bit for SPI_MEM_SLV_ST_END_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_SLV_ST_END_INT_ST_OFFSET: u32 = 3;
    pub const SPI_MEM_SLV_ST_END_INT_ST_MASK: u32 = 0x00000008;
    pub const SPI_MEM_SLV_ST_END_INT_ST: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUS_FIFO0_UDF_INT_ST** `[31]` (RO)
    ///
    /// The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_bus_fifo0_udf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO0_UDF_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_BUS_FIFO1_UDF_INT_ST** `[30]` (RO)
    ///
    /// The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_bus_fifo1_udf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BUS_FIFO1_UDF_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_DQS1_AFIFO_OVF_INT_ST** `[29]` (RO)
    ///
    /// The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_dqs1_afifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS1_AFIFO_OVF_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_DQS0_AFIFO_OVF_INT_ST** `[28]` (RO)
    ///
    /// The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_dqs0_afifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DQS0_AFIFO_OVF_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_AXI_WADDR_ERR_INT_ST** `[9]` (RO)
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WADDR_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_AXI_WR_FLASH_ERR_INT_ST** `[8]` (RO)
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_AXI_RADDR_ERR_INT_ST** `[7]` (RO)
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AXI_RADDR_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_PMS_REJECT_INT_ST** `[6]` (RO)
    ///
    /// The status bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_REJECT_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_ECC_ERR_INT_ST** `[5]` (RO)
    ///
    /// The status bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_MST_ST_END_INT_ST** `[4]` (RO)
    ///
    /// The status bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_MST_ST_END_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_SLV_ST_END_INT_ST** `[3]` (RO)
    ///
    /// The status bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SLV_ST_END_INT_ST_MASK) != 0
    }
}

/// SPI_MEM_INT_ST value type for batch read/modify/write operations
///
/// Use `spimemintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemIntStVal {
    bits: u32,
}

impl SpiMemIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUS_FIFO0_UDF_INT_ST** `[31:31]`
    ///
    /// The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo0_udf_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_BUS_FIFO0_UDF_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_BUS_FIFO1_UDF_INT_ST** `[30:30]`
    ///
    /// The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_bus_fifo1_udf_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_BUS_FIFO1_UDF_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_DQS1_AFIFO_OVF_INT_ST** `[29:29]`
    ///
    /// The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_dqs1_afifo_ovf_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_DQS1_AFIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_DQS0_AFIFO_OVF_INT_ST** `[28:28]`
    ///
    /// The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_dqs0_afifo_ovf_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_DQS0_AFIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WADDR_ERR_INT_ST** `[9:9]`
    ///
    /// The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_waddr_err_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_AXI_WADDR_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_WR_FLASH_ERR_INT_ST** `[8:8]`
    ///
    /// The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_wr_flash_err_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_AXI_RADDR_ERR_INT_ST** `[7:7]`
    ///
    /// The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_axi_raddr_err_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_AXI_RADDR_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_REJECT_INT_ST** `[6:6]`
    ///
    /// The status bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_pms_reject_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_PMS_REJECT_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_ECC_ERR_INT_ST** `[5:5]`
    ///
    /// The status bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_ecc_err_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_ECC_ERR_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_MST_ST_END_INT_ST** `[4:4]`
    ///
    /// The status bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_mst_st_end_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_MST_ST_END_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST_END_INT_ST** `[3:3]`
    ///
    /// The status bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_slv_st_end_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_SLV_ST_END_INT_ST_MASK) != 0
    }
}

impl Default for SpiMemIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemIntStVal {
        SpiMemIntStVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_DDR Register
///
/// SPI0 flash DDR mode control register
///
/// Address offset: 0x00d4
/// Reset value: 0x00003020
///
/// ## Fields
///
/// - **SPI_FMEM_HYPERBUS_CA** `[30]` (RW)
///
///   Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
///
/// - **SPI_FMEM_OCTA_RAM_ADDR** `[29]` (RW)
///
///   Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
///
/// - **SPI_FMEM_CLK_DIFF_INV** `[28]` (RW)
///
///   Set this bit to invert SPI_DIFF when accesses to flash. .
///
/// - **SPI_FMEM_HYPERBUS_DUMMY_2X** `[27]` (RW)
///
///   Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
///
/// - **SPI_FMEM_DQS_CA_IN** `[26]` (RW)
///
///   Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
///
/// - **SPI_FMEM_CLK_DIFF_EN** `[24]` (RW)
///
///   Set this bit to enable the differential SPI_CLK#.
///
/// - **SPI_FMEM_DDR_DQS_LOOP** `[21]` (RW)
///
///   1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
///
/// - **SPI_FMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
///
///   The delay number of data strobe which from memory based on SPI clock.
///
/// - **SPI_FMEM_RX_DDR_MSK_EN** `[13]` (RW)
///
///   Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to flash.
///
/// - **SPI_FMEM_TX_DDR_MSK_EN** `[12]` (RW)
///
///   Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to flash.
///
/// - **SPI_FMEM_OUTMINBYTELEN** `[11:5]` (RW)
///
///   It is the minimum output data length in the panda device.
///
/// - **SPI_FMEM_DDR_CMD_DIS** `[4]` (RW)
///
///   the bit is used to disable dual edge in command phase when DDR mode.
///
/// - **SPI_FMEM_DDR_WDAT_SWP** `[3]` (RW)
///
///   Set the bit to reorder tx data of the word in spi DDR mode.
///
/// - **SPI_FMEM_DDR_RDAT_SWP** `[2]` (RW)
///
///   Set the bit to reorder rx data of the word in spi DDR mode.
///
/// - **SPI_FMEM_VAR_DUMMY** `[1]` (RW)
///
///   Set the bit to enable variable dummy cycle in spi DDR mode.
///
/// - **SPI_FMEM_DDR_EN** `[0]` (RW)
///
///   1: in DDR mode, 0 in SDR mode
#[repr(transparent)]
pub struct SpiMemDdr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDdr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDdr {
    // ======== Field Constants ========

    /// **SPI_FMEM_HYPERBUS_CA**
    ///
    /// Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_FMEM_HYPERBUS_CA_OFFSET: u32 = 30;
    pub const SPI_FMEM_HYPERBUS_CA_MASK: u32 = 0x40000000;
    pub const SPI_FMEM_HYPERBUS_CA: u32 = 1 << 30;

    /// **SPI_FMEM_OCTA_RAM_ADDR**
    ///
    /// Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_FMEM_OCTA_RAM_ADDR_OFFSET: u32 = 29;
    pub const SPI_FMEM_OCTA_RAM_ADDR_MASK: u32 = 0x20000000;
    pub const SPI_FMEM_OCTA_RAM_ADDR: u32 = 1 << 29;

    /// **SPI_FMEM_CLK_DIFF_INV**
    ///
    /// Set this bit to invert SPI_DIFF when accesses to flash. .
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_FMEM_CLK_DIFF_INV_OFFSET: u32 = 28;
    pub const SPI_FMEM_CLK_DIFF_INV_MASK: u32 = 0x10000000;
    pub const SPI_FMEM_CLK_DIFF_INV: u32 = 1 << 28;

    /// **SPI_FMEM_HYPERBUS_DUMMY_2X**
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_FMEM_HYPERBUS_DUMMY_2X_OFFSET: u32 = 27;
    pub const SPI_FMEM_HYPERBUS_DUMMY_2X_MASK: u32 = 0x08000000;
    pub const SPI_FMEM_HYPERBUS_DUMMY_2X: u32 = 1 << 27;

    /// **SPI_FMEM_DQS_CA_IN**
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_FMEM_DQS_CA_IN_OFFSET: u32 = 26;
    pub const SPI_FMEM_DQS_CA_IN_MASK: u32 = 0x04000000;
    pub const SPI_FMEM_DQS_CA_IN: u32 = 1 << 26;

    /// **SPI_FMEM_CLK_DIFF_EN**
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_FMEM_CLK_DIFF_EN_OFFSET: u32 = 24;
    pub const SPI_FMEM_CLK_DIFF_EN_MASK: u32 = 0x01000000;
    pub const SPI_FMEM_CLK_DIFF_EN: u32 = 1 << 24;

    /// **SPI_FMEM_DDR_DQS_LOOP**
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_FMEM_DDR_DQS_LOOP_OFFSET: u32 = 21;
    pub const SPI_FMEM_DDR_DQS_LOOP_MASK: u32 = 0x00200000;
    pub const SPI_FMEM_DDR_DQS_LOOP: u32 = 1 << 21;

    /// **SPI_FMEM_USR_DDR_DQS_THD**
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const SPI_FMEM_USR_DDR_DQS_THD_OFFSET: u32 = 14;
    pub const SPI_FMEM_USR_DDR_DQS_THD_MASK: u32 = 0x001fc000;

    /// **SPI_FMEM_RX_DDR_MSK_EN**
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to flash.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_FMEM_RX_DDR_MSK_EN_OFFSET: u32 = 13;
    pub const SPI_FMEM_RX_DDR_MSK_EN_MASK: u32 = 0x00002000;
    pub const SPI_FMEM_RX_DDR_MSK_EN: u32 = 1 << 13;

    /// **SPI_FMEM_TX_DDR_MSK_EN**
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to flash.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_FMEM_TX_DDR_MSK_EN_OFFSET: u32 = 12;
    pub const SPI_FMEM_TX_DDR_MSK_EN_MASK: u32 = 0x00001000;
    pub const SPI_FMEM_TX_DDR_MSK_EN: u32 = 1 << 12;

    /// **SPI_FMEM_OUTMINBYTELEN**
    ///
    /// It is the minimum output data length in the panda device.
    ///
    /// Bits `[11:5]`, mask `0x00000fe0`
    pub const SPI_FMEM_OUTMINBYTELEN_OFFSET: u32 = 5;
    pub const SPI_FMEM_OUTMINBYTELEN_MASK: u32 = 0x00000fe0;

    /// **SPI_FMEM_DDR_CMD_DIS**
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_FMEM_DDR_CMD_DIS_OFFSET: u32 = 4;
    pub const SPI_FMEM_DDR_CMD_DIS_MASK: u32 = 0x00000010;
    pub const SPI_FMEM_DDR_CMD_DIS: u32 = 1 << 4;

    /// **SPI_FMEM_DDR_WDAT_SWP**
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_FMEM_DDR_WDAT_SWP_OFFSET: u32 = 3;
    pub const SPI_FMEM_DDR_WDAT_SWP_MASK: u32 = 0x00000008;
    pub const SPI_FMEM_DDR_WDAT_SWP: u32 = 1 << 3;

    /// **SPI_FMEM_DDR_RDAT_SWP**
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_DDR_RDAT_SWP_OFFSET: u32 = 2;
    pub const SPI_FMEM_DDR_RDAT_SWP_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_DDR_RDAT_SWP: u32 = 1 << 2;

    /// **SPI_FMEM_VAR_DUMMY**
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_VAR_DUMMY_OFFSET: u32 = 1;
    pub const SPI_FMEM_VAR_DUMMY_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_VAR_DUMMY: u32 = 1 << 1;

    /// **SPI_FMEM_DDR_EN**
    ///
    /// 1: in DDR mode, 0 in SDR mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_FMEM_DDR_EN_OFFSET: u32 = 0;
    pub const SPI_FMEM_DDR_EN_MASK: u32 = 0x00000001;
    pub const SPI_FMEM_DDR_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_FMEM_HYPERBUS_CA** `[30]` (RW)
    ///
    /// Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub fn spi_fmem_hyperbus_ca(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_HYPERBUS_CA_MASK) != 0
    }

    /// Set SPI_FMEM_HYPERBUS_CA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_hyperbus_ca(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_HYPERBUS_CA_MASK);
    }

    /// Clear SPI_FMEM_HYPERBUS_CA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_hyperbus_ca(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_HYPERBUS_CA_MASK);
    }

    /// **SPI_FMEM_OCTA_RAM_ADDR** `[29]` (RW)
    ///
    /// Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub fn spi_fmem_octa_ram_addr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_OCTA_RAM_ADDR_MASK) != 0
    }

    /// Set SPI_FMEM_OCTA_RAM_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_octa_ram_addr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_OCTA_RAM_ADDR_MASK);
    }

    /// Clear SPI_FMEM_OCTA_RAM_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_octa_ram_addr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_OCTA_RAM_ADDR_MASK);
    }

    /// **SPI_FMEM_CLK_DIFF_INV** `[28]` (RW)
    ///
    /// Set this bit to invert SPI_DIFF when accesses to flash. .
    #[inline(always)]
    pub fn spi_fmem_clk_diff_inv(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_CLK_DIFF_INV_MASK) != 0
    }

    /// Set SPI_FMEM_CLK_DIFF_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_clk_diff_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_CLK_DIFF_INV_MASK);
    }

    /// Clear SPI_FMEM_CLK_DIFF_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_clk_diff_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_CLK_DIFF_INV_MASK);
    }

    /// **SPI_FMEM_HYPERBUS_DUMMY_2X** `[27]` (RW)
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub fn spi_fmem_hyperbus_dummy_2x(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK) != 0
    }

    /// Set SPI_FMEM_HYPERBUS_DUMMY_2X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_hyperbus_dummy_2x(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK);
    }

    /// Clear SPI_FMEM_HYPERBUS_DUMMY_2X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_hyperbus_dummy_2x(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK);
    }

    /// **SPI_FMEM_DQS_CA_IN** `[26]` (RW)
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub fn spi_fmem_dqs_ca_in(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DQS_CA_IN_MASK) != 0
    }

    /// Set SPI_FMEM_DQS_CA_IN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_dqs_ca_in(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_DQS_CA_IN_MASK);
    }

    /// Clear SPI_FMEM_DQS_CA_IN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_dqs_ca_in(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_DQS_CA_IN_MASK);
    }

    /// **SPI_FMEM_CLK_DIFF_EN** `[24]` (RW)
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub fn spi_fmem_clk_diff_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_CLK_DIFF_EN_MASK) != 0
    }

    /// Set SPI_FMEM_CLK_DIFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_clk_diff_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_CLK_DIFF_EN_MASK);
    }

    /// Clear SPI_FMEM_CLK_DIFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_clk_diff_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_CLK_DIFF_EN_MASK);
    }

    /// **SPI_FMEM_DDR_DQS_LOOP** `[21]` (RW)
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DDR_DQS_LOOP_MASK) != 0
    }

    /// Set SPI_FMEM_DDR_DQS_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ddr_dqs_loop(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_DDR_DQS_LOOP_MASK);
    }

    /// Clear SPI_FMEM_DDR_DQS_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ddr_dqs_loop(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_DDR_DQS_LOOP_MASK);
    }

    /// **SPI_FMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub fn spi_fmem_usr_ddr_dqs_thd(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_USR_DDR_DQS_THD_MASK)
            >> Self::SPI_FMEM_USR_DDR_DQS_THD_OFFSET
    }

    /// Write **SPI_FMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub fn write_spi_fmem_usr_ddr_dqs_thd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_USR_DDR_DQS_THD_MASK)
                | ((value << Self::SPI_FMEM_USR_DDR_DQS_THD_OFFSET)
                    & Self::SPI_FMEM_USR_DDR_DQS_THD_MASK)
        });
    }

    /// **SPI_FMEM_RX_DDR_MSK_EN** `[13]` (RW)
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to flash.
    #[inline(always)]
    pub fn spi_fmem_rx_ddr_msk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_RX_DDR_MSK_EN_MASK) != 0
    }

    /// Set SPI_FMEM_RX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_rx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_RX_DDR_MSK_EN_MASK);
    }

    /// Clear SPI_FMEM_RX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_rx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_RX_DDR_MSK_EN_MASK);
    }

    /// **SPI_FMEM_TX_DDR_MSK_EN** `[12]` (RW)
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to flash.
    #[inline(always)]
    pub fn spi_fmem_tx_ddr_msk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_TX_DDR_MSK_EN_MASK) != 0
    }

    /// Set SPI_FMEM_TX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_tx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_TX_DDR_MSK_EN_MASK);
    }

    /// Clear SPI_FMEM_TX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_tx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_TX_DDR_MSK_EN_MASK);
    }

    /// **SPI_FMEM_OUTMINBYTELEN** `[11:5]` (RW)
    ///
    /// It is the minimum output data length in the panda device.
    #[inline(always)]
    pub fn spi_fmem_outminbytelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_OUTMINBYTELEN_MASK)
            >> Self::SPI_FMEM_OUTMINBYTELEN_OFFSET
    }

    /// Write **SPI_FMEM_OUTMINBYTELEN** `[11:5]` (RW)
    ///
    /// It is the minimum output data length in the panda device.
    #[inline(always)]
    pub fn write_spi_fmem_outminbytelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_OUTMINBYTELEN_MASK)
                | ((value << Self::SPI_FMEM_OUTMINBYTELEN_OFFSET)
                    & Self::SPI_FMEM_OUTMINBYTELEN_MASK)
        });
    }

    /// **SPI_FMEM_DDR_CMD_DIS** `[4]` (RW)
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub fn spi_fmem_ddr_cmd_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DDR_CMD_DIS_MASK) != 0
    }

    /// Set SPI_FMEM_DDR_CMD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ddr_cmd_dis(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_DDR_CMD_DIS_MASK);
    }

    /// Clear SPI_FMEM_DDR_CMD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ddr_cmd_dis(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_DDR_CMD_DIS_MASK);
    }

    /// **SPI_FMEM_DDR_WDAT_SWP** `[3]` (RW)
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub fn spi_fmem_ddr_wdat_swp(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DDR_WDAT_SWP_MASK) != 0
    }

    /// Set SPI_FMEM_DDR_WDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ddr_wdat_swp(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_DDR_WDAT_SWP_MASK);
    }

    /// Clear SPI_FMEM_DDR_WDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ddr_wdat_swp(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_DDR_WDAT_SWP_MASK);
    }

    /// **SPI_FMEM_DDR_RDAT_SWP** `[2]` (RW)
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub fn spi_fmem_ddr_rdat_swp(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DDR_RDAT_SWP_MASK) != 0
    }

    /// Set SPI_FMEM_DDR_RDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ddr_rdat_swp(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_DDR_RDAT_SWP_MASK);
    }

    /// Clear SPI_FMEM_DDR_RDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ddr_rdat_swp(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_DDR_RDAT_SWP_MASK);
    }

    /// **SPI_FMEM_VAR_DUMMY** `[1]` (RW)
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub fn spi_fmem_var_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_VAR_DUMMY_MASK) != 0
    }

    /// Set SPI_FMEM_VAR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_var_dummy(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_VAR_DUMMY_MASK);
    }

    /// Clear SPI_FMEM_VAR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_var_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_VAR_DUMMY_MASK);
    }

    /// **SPI_FMEM_DDR_EN** `[0]` (RW)
    ///
    /// 1: in DDR mode, 0 in SDR mode
    #[inline(always)]
    pub fn spi_fmem_ddr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_DDR_EN_MASK) != 0
    }

    /// Set SPI_FMEM_DDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ddr_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FMEM_DDR_EN_MASK);
    }

    /// Clear SPI_FMEM_DDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ddr_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FMEM_DDR_EN_MASK);
    }
}

/// SPI_MEM_DDR value type for batch read/modify/write operations
///
/// Use `spimemddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDdrVal {
    bits: u32,
}

impl SpiMemDdrVal {
    /// Create from raw bits
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
        Self { bits: 0x00003020 }
    }

    /// Read **SPI_FMEM_HYPERBUS_CA** `[30:30]`
    ///
    /// Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub const fn spi_fmem_hyperbus_ca(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_HYPERBUS_CA_MASK) != 0
    }

    /// Read **SPI_FMEM_OCTA_RAM_ADDR** `[29:29]`
    ///
    /// Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub const fn spi_fmem_octa_ram_addr(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_OCTA_RAM_ADDR_MASK) != 0
    }

    /// Read **SPI_FMEM_CLK_DIFF_INV** `[28:28]`
    ///
    /// Set this bit to invert SPI_DIFF when accesses to flash. .
    #[inline(always)]
    pub const fn spi_fmem_clk_diff_inv(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_CLK_DIFF_INV_MASK) != 0
    }

    /// Read **SPI_FMEM_HYPERBUS_DUMMY_2X** `[27:27]`
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub const fn spi_fmem_hyperbus_dummy_2x(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK) != 0
    }

    /// Read **SPI_FMEM_DQS_CA_IN** `[26:26]`
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub const fn spi_fmem_dqs_ca_in(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DQS_CA_IN_MASK) != 0
    }

    /// Read **SPI_FMEM_CLK_DIFF_EN** `[24:24]`
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub const fn spi_fmem_clk_diff_en(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_CLK_DIFF_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_DQS_LOOP** `[21:21]`
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub const fn spi_fmem_ddr_dqs_loop(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_DQS_LOOP_MASK) != 0
    }

    /// Read **SPI_FMEM_USR_DDR_DQS_THD** `[20:14]`
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub const fn spi_fmem_usr_ddr_dqs_thd(&self) -> u32 {
        (self.bits & SpiMemDdr::SPI_FMEM_USR_DDR_DQS_THD_MASK)
            >> SpiMemDdr::SPI_FMEM_USR_DDR_DQS_THD_OFFSET
    }

    /// Read **SPI_FMEM_RX_DDR_MSK_EN** `[13:13]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to flash.
    #[inline(always)]
    pub const fn spi_fmem_rx_ddr_msk_en(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_RX_DDR_MSK_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_TX_DDR_MSK_EN** `[12:12]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to flash.
    #[inline(always)]
    pub const fn spi_fmem_tx_ddr_msk_en(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_TX_DDR_MSK_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_OUTMINBYTELEN** `[11:5]`
    ///
    /// It is the minimum output data length in the panda device.
    #[inline(always)]
    pub const fn spi_fmem_outminbytelen(&self) -> u32 {
        (self.bits & SpiMemDdr::SPI_FMEM_OUTMINBYTELEN_MASK)
            >> SpiMemDdr::SPI_FMEM_OUTMINBYTELEN_OFFSET
    }

    /// Read **SPI_FMEM_DDR_CMD_DIS** `[4:4]`
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_cmd_dis(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_CMD_DIS_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_WDAT_SWP** `[3:3]`
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_wdat_swp(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_WDAT_SWP_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_RDAT_SWP** `[2:2]`
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_rdat_swp(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_RDAT_SWP_MASK) != 0
    }

    /// Read **SPI_FMEM_VAR_DUMMY** `[1:1]`
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub const fn spi_fmem_var_dummy(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_VAR_DUMMY_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_EN** `[0:0]`
    ///
    /// 1: in DDR mode, 0 in SDR mode
    #[inline(always)]
    pub const fn spi_fmem_ddr_en(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_EN_MASK) != 0
    }

    /// Set **SPI_FMEM_HYPERBUS_CA** `[30:30]`
    ///
    /// Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub const fn with_spi_fmem_hyperbus_ca(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_HYPERBUS_CA_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_HYPERBUS_CA_MASK
            },
        }
    }

    /// Set **SPI_FMEM_OCTA_RAM_ADDR** `[29:29]`
    ///
    /// Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub const fn with_spi_fmem_octa_ram_addr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_OCTA_RAM_ADDR_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_OCTA_RAM_ADDR_MASK
            },
        }
    }

    /// Set **SPI_FMEM_CLK_DIFF_INV** `[28:28]`
    ///
    /// Set this bit to invert SPI_DIFF when accesses to flash. .
    #[inline(always)]
    pub const fn with_spi_fmem_clk_diff_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_CLK_DIFF_INV_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_CLK_DIFF_INV_MASK
            },
        }
    }

    /// Set **SPI_FMEM_HYPERBUS_DUMMY_2X** `[27:27]`
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub const fn with_spi_fmem_hyperbus_dummy_2x(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_HYPERBUS_DUMMY_2X_MASK
            },
        }
    }

    /// Set **SPI_FMEM_DQS_CA_IN** `[26:26]`
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub const fn with_spi_fmem_dqs_ca_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DQS_CA_IN_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DQS_CA_IN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_CLK_DIFF_EN** `[24:24]`
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub const fn with_spi_fmem_clk_diff_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_CLK_DIFF_EN_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_CLK_DIFF_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_DDR_DQS_LOOP** `[21:21]`
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub const fn with_spi_fmem_ddr_dqs_loop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DDR_DQS_LOOP_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DDR_DQS_LOOP_MASK
            },
        }
    }

    /// Set **SPI_FMEM_USR_DDR_DQS_THD** `[20:14]`
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub const fn with_spi_fmem_usr_ddr_dqs_thd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDdr::SPI_FMEM_USR_DDR_DQS_THD_MASK)
                | ((value << SpiMemDdr::SPI_FMEM_USR_DDR_DQS_THD_OFFSET)
                    & SpiMemDdr::SPI_FMEM_USR_DDR_DQS_THD_MASK),
        }
    }

    /// Set **SPI_FMEM_RX_DDR_MSK_EN** `[13:13]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to flash.
    #[inline(always)]
    pub const fn with_spi_fmem_rx_ddr_msk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_RX_DDR_MSK_EN_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_RX_DDR_MSK_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_TX_DDR_MSK_EN** `[12:12]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to flash.
    #[inline(always)]
    pub const fn with_spi_fmem_tx_ddr_msk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_TX_DDR_MSK_EN_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_TX_DDR_MSK_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_OUTMINBYTELEN** `[11:5]`
    ///
    /// It is the minimum output data length in the panda device.
    #[inline(always)]
    pub const fn with_spi_fmem_outminbytelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDdr::SPI_FMEM_OUTMINBYTELEN_MASK)
                | ((value << SpiMemDdr::SPI_FMEM_OUTMINBYTELEN_OFFSET)
                    & SpiMemDdr::SPI_FMEM_OUTMINBYTELEN_MASK),
        }
    }

    /// Set **SPI_FMEM_DDR_CMD_DIS** `[4:4]`
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub const fn with_spi_fmem_ddr_cmd_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DDR_CMD_DIS_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DDR_CMD_DIS_MASK
            },
        }
    }

    /// Set **SPI_FMEM_DDR_WDAT_SWP** `[3:3]`
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn with_spi_fmem_ddr_wdat_swp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DDR_WDAT_SWP_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DDR_WDAT_SWP_MASK
            },
        }
    }

    /// Set **SPI_FMEM_DDR_RDAT_SWP** `[2:2]`
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn with_spi_fmem_ddr_rdat_swp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DDR_RDAT_SWP_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DDR_RDAT_SWP_MASK
            },
        }
    }

    /// Set **SPI_FMEM_VAR_DUMMY** `[1:1]`
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub const fn with_spi_fmem_var_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_VAR_DUMMY_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_VAR_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_FMEM_DDR_EN** `[0:0]`
    ///
    /// 1: in DDR mode, 0 in SDR mode
    #[inline(always)]
    pub const fn with_spi_fmem_ddr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDdr::SPI_FMEM_DDR_EN_MASK
            } else {
                self.bits & !SpiMemDdr::SPI_FMEM_DDR_EN_MASK
            },
        }
    }
}

impl Default for SpiMemDdrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemDdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDdrVal {
        SpiMemDdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDdrVal) -> SpiMemDdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DDR Register
///
/// SPI0 external RAM DDR mode control register
///
/// Address offset: 0x00d8
/// Reset value: 0x00003020
///
/// ## Fields
///
/// - **SPI_SMEM_HYPERBUS_CA** `[30]` (RW)
///
///   Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
///
/// - **SPI_SMEM_OCTA_RAM_ADDR** `[29]` (RW)
///
///   Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
///
/// - **SPI_SMEM_CLK_DIFF_INV** `[28]` (RW)
///
///   Set this bit to invert SPI_DIFF when accesses to external RAM. .
///
/// - **SPI_SMEM_HYPERBUS_DUMMY_2X** `[27]` (RW)
///
///   Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
///
/// - **SPI_SMEM_DQS_CA_IN** `[26]` (RW)
///
///   Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
///
/// - **SPI_SMEM_CLK_DIFF_EN** `[24]` (RW)
///
///   Set this bit to enable the differential SPI_CLK#.
///
/// - **DQS_LOOP** `[21]` (RW)
///
///   1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
///
/// - **SPI_SMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
///
///   The delay number of data strobe which from memory based on SPI clock.
///
/// - **SPI_SMEM_RX_DDR_MSK_EN** `[13]` (RW)
///
///   Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM.
///
/// - **SPI_SMEM_TX_DDR_MSK_EN** `[12]` (RW)
///
///   Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM.
///
/// - **SPI_SMEM_OUTMINBYTELEN** `[11:5]` (RW)
///
///   It is the minimum output data length in the DDR psram.
///
/// - **CMD_DIS** `[4]` (RW)
///
///   the bit is used to disable dual edge in command phase when DDR mode.
///
/// - **WDAT_SWP** `[3]` (RW)
///
///   Set the bit to reorder tx data of the word in spi DDR mode.
///
/// - **RDAT_SWP** `[2]` (RW)
///
///   Set the bit to reorder rx data of the word in spi DDR mode.
///
/// - **SPI_SMEM_VAR_DUMMY** `[1]` (RW)
///
///   Set the bit to enable variable dummy cycle in spi DDR mode.
///
/// - **EN** `[0]` (RW)
///
///   1: in DDR mode, 0 in SDR mode
#[repr(transparent)]
pub struct SpiSmemDdr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDdr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDdr {
    // ======== Field Constants ========

    /// **SPI_SMEM_HYPERBUS_CA**
    ///
    /// Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_SMEM_HYPERBUS_CA_OFFSET: u32 = 30;
    pub const SPI_SMEM_HYPERBUS_CA_MASK: u32 = 0x40000000;
    pub const SPI_SMEM_HYPERBUS_CA: u32 = 1 << 30;

    /// **SPI_SMEM_OCTA_RAM_ADDR**
    ///
    /// Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_SMEM_OCTA_RAM_ADDR_OFFSET: u32 = 29;
    pub const SPI_SMEM_OCTA_RAM_ADDR_MASK: u32 = 0x20000000;
    pub const SPI_SMEM_OCTA_RAM_ADDR: u32 = 1 << 29;

    /// **SPI_SMEM_CLK_DIFF_INV**
    ///
    /// Set this bit to invert SPI_DIFF when accesses to external RAM. .
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_SMEM_CLK_DIFF_INV_OFFSET: u32 = 28;
    pub const SPI_SMEM_CLK_DIFF_INV_MASK: u32 = 0x10000000;
    pub const SPI_SMEM_CLK_DIFF_INV: u32 = 1 << 28;

    /// **SPI_SMEM_HYPERBUS_DUMMY_2X**
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_SMEM_HYPERBUS_DUMMY_2X_OFFSET: u32 = 27;
    pub const SPI_SMEM_HYPERBUS_DUMMY_2X_MASK: u32 = 0x08000000;
    pub const SPI_SMEM_HYPERBUS_DUMMY_2X: u32 = 1 << 27;

    /// **SPI_SMEM_DQS_CA_IN**
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_SMEM_DQS_CA_IN_OFFSET: u32 = 26;
    pub const SPI_SMEM_DQS_CA_IN_MASK: u32 = 0x04000000;
    pub const SPI_SMEM_DQS_CA_IN: u32 = 1 << 26;

    /// **SPI_SMEM_CLK_DIFF_EN**
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_SMEM_CLK_DIFF_EN_OFFSET: u32 = 24;
    pub const SPI_SMEM_CLK_DIFF_EN_MASK: u32 = 0x01000000;
    pub const SPI_SMEM_CLK_DIFF_EN: u32 = 1 << 24;

    /// **DQS_LOOP**
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const DQS_LOOP_OFFSET: u32 = 21;
    pub const DQS_LOOP_MASK: u32 = 0x00200000;
    pub const DQS_LOOP: u32 = 1 << 21;

    /// **SPI_SMEM_USR_DDR_DQS_THD**
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const SPI_SMEM_USR_DDR_DQS_THD_OFFSET: u32 = 14;
    pub const SPI_SMEM_USR_DDR_DQS_THD_MASK: u32 = 0x001fc000;

    /// **SPI_SMEM_RX_DDR_MSK_EN**
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_SMEM_RX_DDR_MSK_EN_OFFSET: u32 = 13;
    pub const SPI_SMEM_RX_DDR_MSK_EN_MASK: u32 = 0x00002000;
    pub const SPI_SMEM_RX_DDR_MSK_EN: u32 = 1 << 13;

    /// **SPI_SMEM_TX_DDR_MSK_EN**
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_SMEM_TX_DDR_MSK_EN_OFFSET: u32 = 12;
    pub const SPI_SMEM_TX_DDR_MSK_EN_MASK: u32 = 0x00001000;
    pub const SPI_SMEM_TX_DDR_MSK_EN: u32 = 1 << 12;

    /// **SPI_SMEM_OUTMINBYTELEN**
    ///
    /// It is the minimum output data length in the DDR psram.
    ///
    /// Bits `[11:5]`, mask `0x00000fe0`
    pub const SPI_SMEM_OUTMINBYTELEN_OFFSET: u32 = 5;
    pub const SPI_SMEM_OUTMINBYTELEN_MASK: u32 = 0x00000fe0;

    /// **CMD_DIS**
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CMD_DIS_OFFSET: u32 = 4;
    pub const CMD_DIS_MASK: u32 = 0x00000010;
    pub const CMD_DIS: u32 = 1 << 4;

    /// **WDAT_SWP**
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const WDAT_SWP_OFFSET: u32 = 3;
    pub const WDAT_SWP_MASK: u32 = 0x00000008;
    pub const WDAT_SWP: u32 = 1 << 3;

    /// **RDAT_SWP**
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RDAT_SWP_OFFSET: u32 = 2;
    pub const RDAT_SWP_MASK: u32 = 0x00000004;
    pub const RDAT_SWP: u32 = 1 << 2;

    /// **SPI_SMEM_VAR_DUMMY**
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_VAR_DUMMY_OFFSET: u32 = 1;
    pub const SPI_SMEM_VAR_DUMMY_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_VAR_DUMMY: u32 = 1 << 1;

    /// **EN**
    ///
    /// 1: in DDR mode, 0 in SDR mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_HYPERBUS_CA** `[30]` (RW)
    ///
    /// Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub fn spi_smem_hyperbus_ca(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_HYPERBUS_CA_MASK) != 0
    }

    /// Set SPI_SMEM_HYPERBUS_CA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_hyperbus_ca(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_HYPERBUS_CA_MASK);
    }

    /// Clear SPI_SMEM_HYPERBUS_CA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_hyperbus_ca(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_HYPERBUS_CA_MASK);
    }

    /// **SPI_SMEM_OCTA_RAM_ADDR** `[29]` (RW)
    ///
    /// Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub fn spi_smem_octa_ram_addr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_OCTA_RAM_ADDR_MASK) != 0
    }

    /// Set SPI_SMEM_OCTA_RAM_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_octa_ram_addr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_OCTA_RAM_ADDR_MASK);
    }

    /// Clear SPI_SMEM_OCTA_RAM_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_octa_ram_addr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_OCTA_RAM_ADDR_MASK);
    }

    /// **SPI_SMEM_CLK_DIFF_INV** `[28]` (RW)
    ///
    /// Set this bit to invert SPI_DIFF when accesses to external RAM. .
    #[inline(always)]
    pub fn spi_smem_clk_diff_inv(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_CLK_DIFF_INV_MASK) != 0
    }

    /// Set SPI_SMEM_CLK_DIFF_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_clk_diff_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_CLK_DIFF_INV_MASK);
    }

    /// Clear SPI_SMEM_CLK_DIFF_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_clk_diff_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_CLK_DIFF_INV_MASK);
    }

    /// **SPI_SMEM_HYPERBUS_DUMMY_2X** `[27]` (RW)
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub fn spi_smem_hyperbus_dummy_2x(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK) != 0
    }

    /// Set SPI_SMEM_HYPERBUS_DUMMY_2X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_hyperbus_dummy_2x(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK);
    }

    /// Clear SPI_SMEM_HYPERBUS_DUMMY_2X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_hyperbus_dummy_2x(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK);
    }

    /// **SPI_SMEM_DQS_CA_IN** `[26]` (RW)
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub fn spi_smem_dqs_ca_in(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DQS_CA_IN_MASK) != 0
    }

    /// Set SPI_SMEM_DQS_CA_IN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dqs_ca_in(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DQS_CA_IN_MASK);
    }

    /// Clear SPI_SMEM_DQS_CA_IN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dqs_ca_in(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DQS_CA_IN_MASK);
    }

    /// **SPI_SMEM_CLK_DIFF_EN** `[24]` (RW)
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub fn spi_smem_clk_diff_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_CLK_DIFF_EN_MASK) != 0
    }

    /// Set SPI_SMEM_CLK_DIFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_clk_diff_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_CLK_DIFF_EN_MASK);
    }

    /// Clear SPI_SMEM_CLK_DIFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_clk_diff_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_CLK_DIFF_EN_MASK);
    }

    /// **DQS_LOOP** `[21]` (RW)
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub fn dqs_loop(&self) -> bool {
        (self.value.read_bits() & Self::DQS_LOOP_MASK) != 0
    }

    /// Set DQS_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dqs_loop(&self) {
        self.value.modify_bits(|r| r | Self::DQS_LOOP_MASK);
    }

    /// Clear DQS_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dqs_loop(&self) {
        self.value.modify_bits(|r| r & !Self::DQS_LOOP_MASK);
    }

    /// **SPI_SMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub fn spi_smem_usr_ddr_dqs_thd(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_USR_DDR_DQS_THD_MASK)
            >> Self::SPI_SMEM_USR_DDR_DQS_THD_OFFSET
    }

    /// Write **SPI_SMEM_USR_DDR_DQS_THD** `[20:14]` (RW)
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub fn write_spi_smem_usr_ddr_dqs_thd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_USR_DDR_DQS_THD_MASK)
                | ((value << Self::SPI_SMEM_USR_DDR_DQS_THD_OFFSET)
                    & Self::SPI_SMEM_USR_DDR_DQS_THD_MASK)
        });
    }

    /// **SPI_SMEM_RX_DDR_MSK_EN** `[13]` (RW)
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM.
    #[inline(always)]
    pub fn spi_smem_rx_ddr_msk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_RX_DDR_MSK_EN_MASK) != 0
    }

    /// Set SPI_SMEM_RX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_rx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_RX_DDR_MSK_EN_MASK);
    }

    /// Clear SPI_SMEM_RX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_rx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_RX_DDR_MSK_EN_MASK);
    }

    /// **SPI_SMEM_TX_DDR_MSK_EN** `[12]` (RW)
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM.
    #[inline(always)]
    pub fn spi_smem_tx_ddr_msk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_TX_DDR_MSK_EN_MASK) != 0
    }

    /// Set SPI_SMEM_TX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_tx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_TX_DDR_MSK_EN_MASK);
    }

    /// Clear SPI_SMEM_TX_DDR_MSK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_tx_ddr_msk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_TX_DDR_MSK_EN_MASK);
    }

    /// **SPI_SMEM_OUTMINBYTELEN** `[11:5]` (RW)
    ///
    /// It is the minimum output data length in the DDR psram.
    #[inline(always)]
    pub fn spi_smem_outminbytelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_OUTMINBYTELEN_MASK)
            >> Self::SPI_SMEM_OUTMINBYTELEN_OFFSET
    }

    /// Write **SPI_SMEM_OUTMINBYTELEN** `[11:5]` (RW)
    ///
    /// It is the minimum output data length in the DDR psram.
    #[inline(always)]
    pub fn write_spi_smem_outminbytelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_OUTMINBYTELEN_MASK)
                | ((value << Self::SPI_SMEM_OUTMINBYTELEN_OFFSET)
                    & Self::SPI_SMEM_OUTMINBYTELEN_MASK)
        });
    }

    /// **CMD_DIS** `[4]` (RW)
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub fn cmd_dis(&self) -> bool {
        (self.value.read_bits() & Self::CMD_DIS_MASK) != 0
    }

    /// Set CMD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_dis(&self) {
        self.value.modify_bits(|r| r | Self::CMD_DIS_MASK);
    }

    /// Clear CMD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_dis(&self) {
        self.value.modify_bits(|r| r & !Self::CMD_DIS_MASK);
    }

    /// **WDAT_SWP** `[3]` (RW)
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub fn wdat_swp(&self) -> bool {
        (self.value.read_bits() & Self::WDAT_SWP_MASK) != 0
    }

    /// Set WDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdat_swp(&self) {
        self.value.modify_bits(|r| r | Self::WDAT_SWP_MASK);
    }

    /// Clear WDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdat_swp(&self) {
        self.value.modify_bits(|r| r & !Self::WDAT_SWP_MASK);
    }

    /// **RDAT_SWP** `[2]` (RW)
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub fn rdat_swp(&self) -> bool {
        (self.value.read_bits() & Self::RDAT_SWP_MASK) != 0
    }

    /// Set RDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rdat_swp(&self) {
        self.value.modify_bits(|r| r | Self::RDAT_SWP_MASK);
    }

    /// Clear RDAT_SWP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rdat_swp(&self) {
        self.value.modify_bits(|r| r & !Self::RDAT_SWP_MASK);
    }

    /// **SPI_SMEM_VAR_DUMMY** `[1]` (RW)
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub fn spi_smem_var_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_VAR_DUMMY_MASK) != 0
    }

    /// Set SPI_SMEM_VAR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_var_dummy(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_VAR_DUMMY_MASK);
    }

    /// Clear SPI_SMEM_VAR_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_var_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_VAR_DUMMY_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// 1: in DDR mode, 0 in SDR mode
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

/// SPI_SMEM_DDR value type for batch read/modify/write operations
///
/// Use `spismemddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDdrVal {
    bits: u32,
}

impl SpiSmemDdrVal {
    /// Create from raw bits
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
        Self { bits: 0x00003020 }
    }

    /// Read **SPI_SMEM_HYPERBUS_CA** `[30:30]`
    ///
    /// Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub const fn spi_smem_hyperbus_ca(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_HYPERBUS_CA_MASK) != 0
    }

    /// Read **SPI_SMEM_OCTA_RAM_ADDR** `[29:29]`
    ///
    /// Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub const fn spi_smem_octa_ram_addr(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_OCTA_RAM_ADDR_MASK) != 0
    }

    /// Read **SPI_SMEM_CLK_DIFF_INV** `[28:28]`
    ///
    /// Set this bit to invert SPI_DIFF when accesses to external RAM. .
    #[inline(always)]
    pub const fn spi_smem_clk_diff_inv(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_CLK_DIFF_INV_MASK) != 0
    }

    /// Read **SPI_SMEM_HYPERBUS_DUMMY_2X** `[27:27]`
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub const fn spi_smem_hyperbus_dummy_2x(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK) != 0
    }

    /// Read **SPI_SMEM_DQS_CA_IN** `[26:26]`
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub const fn spi_smem_dqs_ca_in(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_DQS_CA_IN_MASK) != 0
    }

    /// Read **SPI_SMEM_CLK_DIFF_EN** `[24:24]`
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub const fn spi_smem_clk_diff_en(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_CLK_DIFF_EN_MASK) != 0
    }

    /// Read **DQS_LOOP** `[21:21]`
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub const fn dqs_loop(&self) -> bool {
        (self.bits & SpiSmemDdr::DQS_LOOP_MASK) != 0
    }

    /// Read **SPI_SMEM_USR_DDR_DQS_THD** `[20:14]`
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub const fn spi_smem_usr_ddr_dqs_thd(&self) -> u32 {
        (self.bits & SpiSmemDdr::SPI_SMEM_USR_DDR_DQS_THD_MASK)
            >> SpiSmemDdr::SPI_SMEM_USR_DDR_DQS_THD_OFFSET
    }

    /// Read **SPI_SMEM_RX_DDR_MSK_EN** `[13:13]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM.
    #[inline(always)]
    pub const fn spi_smem_rx_ddr_msk_en(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_RX_DDR_MSK_EN_MASK) != 0
    }

    /// Read **SPI_SMEM_TX_DDR_MSK_EN** `[12:12]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM.
    #[inline(always)]
    pub const fn spi_smem_tx_ddr_msk_en(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_TX_DDR_MSK_EN_MASK) != 0
    }

    /// Read **SPI_SMEM_OUTMINBYTELEN** `[11:5]`
    ///
    /// It is the minimum output data length in the DDR psram.
    #[inline(always)]
    pub const fn spi_smem_outminbytelen(&self) -> u32 {
        (self.bits & SpiSmemDdr::SPI_SMEM_OUTMINBYTELEN_MASK)
            >> SpiSmemDdr::SPI_SMEM_OUTMINBYTELEN_OFFSET
    }

    /// Read **CMD_DIS** `[4:4]`
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub const fn cmd_dis(&self) -> bool {
        (self.bits & SpiSmemDdr::CMD_DIS_MASK) != 0
    }

    /// Read **WDAT_SWP** `[3:3]`
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn wdat_swp(&self) -> bool {
        (self.bits & SpiSmemDdr::WDAT_SWP_MASK) != 0
    }

    /// Read **RDAT_SWP** `[2:2]`
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn rdat_swp(&self) -> bool {
        (self.bits & SpiSmemDdr::RDAT_SWP_MASK) != 0
    }

    /// Read **SPI_SMEM_VAR_DUMMY** `[1:1]`
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub const fn spi_smem_var_dummy(&self) -> bool {
        (self.bits & SpiSmemDdr::SPI_SMEM_VAR_DUMMY_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// 1: in DDR mode, 0 in SDR mode
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & SpiSmemDdr::EN_MASK) != 0
    }

    /// Set **SPI_SMEM_HYPERBUS_CA** `[30:30]`
    ///
    /// Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[19:4], 13'd0, spi_usr_addr_value[3:1]}.
    #[inline(always)]
    pub const fn with_spi_smem_hyperbus_ca(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_HYPERBUS_CA_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_HYPERBUS_CA_MASK
            },
        }
    }

    /// Set **SPI_SMEM_OCTA_RAM_ADDR** `[29:29]`
    ///
    /// Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT[31:0] = {spi_usr_addr_value[25:4], 6'd0, spi_usr_addr_value[3:1], 1'b0}.
    #[inline(always)]
    pub const fn with_spi_smem_octa_ram_addr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_OCTA_RAM_ADDR_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_OCTA_RAM_ADDR_MASK
            },
        }
    }

    /// Set **SPI_SMEM_CLK_DIFF_INV** `[28:28]`
    ///
    /// Set this bit to invert SPI_DIFF when accesses to external RAM. .
    #[inline(always)]
    pub const fn with_spi_smem_clk_diff_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_CLK_DIFF_INV_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_CLK_DIFF_INV_MASK
            },
        }
    }

    /// Set **SPI_SMEM_HYPERBUS_DUMMY_2X** `[27:27]`
    ///
    /// Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub const fn with_spi_smem_hyperbus_dummy_2x(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_HYPERBUS_DUMMY_2X_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DQS_CA_IN** `[26:26]`
    ///
    /// Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub const fn with_spi_smem_dqs_ca_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_DQS_CA_IN_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_DQS_CA_IN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_CLK_DIFF_EN** `[24:24]`
    ///
    /// Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub const fn with_spi_smem_clk_diff_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_CLK_DIFF_EN_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_CLK_DIFF_EN_MASK
            },
        }
    }

    /// Set **DQS_LOOP** `[21:21]`
    ///
    /// 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS.
    #[inline(always)]
    pub const fn with_dqs_loop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::DQS_LOOP_MASK
            } else {
                self.bits & !SpiSmemDdr::DQS_LOOP_MASK
            },
        }
    }

    /// Set **SPI_SMEM_USR_DDR_DQS_THD** `[20:14]`
    ///
    /// The delay number of data strobe which from memory based on SPI clock.
    #[inline(always)]
    pub const fn with_spi_smem_usr_ddr_dqs_thd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDdr::SPI_SMEM_USR_DDR_DQS_THD_MASK)
                | ((value << SpiSmemDdr::SPI_SMEM_USR_DDR_DQS_THD_OFFSET)
                    & SpiSmemDdr::SPI_SMEM_USR_DDR_DQS_THD_MASK),
        }
    }

    /// Set **SPI_SMEM_RX_DDR_MSK_EN** `[13:13]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_rx_ddr_msk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_RX_DDR_MSK_EN_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_RX_DDR_MSK_EN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_TX_DDR_MSK_EN** `[12:12]`
    ///
    /// Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_tx_ddr_msk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_TX_DDR_MSK_EN_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_TX_DDR_MSK_EN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_OUTMINBYTELEN** `[11:5]`
    ///
    /// It is the minimum output data length in the DDR psram.
    #[inline(always)]
    pub const fn with_spi_smem_outminbytelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDdr::SPI_SMEM_OUTMINBYTELEN_MASK)
                | ((value << SpiSmemDdr::SPI_SMEM_OUTMINBYTELEN_OFFSET)
                    & SpiSmemDdr::SPI_SMEM_OUTMINBYTELEN_MASK),
        }
    }

    /// Set **CMD_DIS** `[4:4]`
    ///
    /// the bit is used to disable dual edge in command phase when DDR mode.
    #[inline(always)]
    pub const fn with_cmd_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::CMD_DIS_MASK
            } else {
                self.bits & !SpiSmemDdr::CMD_DIS_MASK
            },
        }
    }

    /// Set **WDAT_SWP** `[3:3]`
    ///
    /// Set the bit to reorder tx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn with_wdat_swp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::WDAT_SWP_MASK
            } else {
                self.bits & !SpiSmemDdr::WDAT_SWP_MASK
            },
        }
    }

    /// Set **RDAT_SWP** `[2:2]`
    ///
    /// Set the bit to reorder rx data of the word in spi DDR mode.
    #[inline(always)]
    pub const fn with_rdat_swp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::RDAT_SWP_MASK
            } else {
                self.bits & !SpiSmemDdr::RDAT_SWP_MASK
            },
        }
    }

    /// Set **SPI_SMEM_VAR_DUMMY** `[1:1]`
    ///
    /// Set the bit to enable variable dummy cycle in spi DDR mode.
    #[inline(always)]
    pub const fn with_spi_smem_var_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::SPI_SMEM_VAR_DUMMY_MASK
            } else {
                self.bits & !SpiSmemDdr::SPI_SMEM_VAR_DUMMY_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// 1: in DDR mode, 0 in SDR mode
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDdr::EN_MASK
            } else {
                self.bits & !SpiSmemDdr::EN_MASK
            },
        }
    }
}

impl Default for SpiSmemDdrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemDdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDdrVal {
        SpiSmemDdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDdrVal) -> SpiSmemDdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS0_ATTR Register
///
/// MSPI flash PMS section 0 attribute register
///
/// Address offset: 0x0100
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
///
/// - **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiFmemPms0Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms0Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms0Attr {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_ECC**
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_FMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_FMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_FMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_FMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_FMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_FMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_fmem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_FMEM_PMS0_ATTR value type for batch read/modify/write operations
///
/// Use `spifmempms0attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms0attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms0AttrVal {
    bits: u32,
}

impl SpiFmemPms0AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_fmem_pms_ecc(&self) -> bool {
        (self.bits & SpiFmemPms0Attr::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiFmemPms0Attr::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiFmemPms0Attr::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms0Attr::SPI_FMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiFmemPms0Attr::SPI_FMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms0Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms0Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms0Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms0Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiFmemPms0AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms0Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms0AttrVal {
        SpiFmemPms0AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms0AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms0AttrVal) -> SpiFmemPms0AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS1_ATTR Register
///
/// MSPI flash PMS section 1 attribute register
///
/// Address offset: 0x0104
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
///
/// - **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiFmemPms1Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms1Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms1Attr {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_ECC**
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_FMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_FMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_FMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_FMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_FMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_FMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_fmem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_FMEM_PMS1_ATTR value type for batch read/modify/write operations
///
/// Use `spifmempms1attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms1attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms1AttrVal {
    bits: u32,
}

impl SpiFmemPms1AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_fmem_pms_ecc(&self) -> bool {
        (self.bits & SpiFmemPms1Attr::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiFmemPms1Attr::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiFmemPms1Attr::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms1Attr::SPI_FMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiFmemPms1Attr::SPI_FMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms1Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms1Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms1Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms1Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiFmemPms1AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms1Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms1AttrVal {
        SpiFmemPms1AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms1AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms1AttrVal) -> SpiFmemPms1AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS2_ATTR Register
///
/// MSPI flash PMS section 2 attribute register
///
/// Address offset: 0x0108
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
///
/// - **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiFmemPms2Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms2Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms2Attr {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_ECC**
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_FMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_FMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_FMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_FMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_FMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_FMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_fmem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_FMEM_PMS2_ATTR value type for batch read/modify/write operations
///
/// Use `spifmempms2attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms2attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms2AttrVal {
    bits: u32,
}

impl SpiFmemPms2AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_fmem_pms_ecc(&self) -> bool {
        (self.bits & SpiFmemPms2Attr::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiFmemPms2Attr::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiFmemPms2Attr::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms2Attr::SPI_FMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiFmemPms2Attr::SPI_FMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms2Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms2Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms2Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms2Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiFmemPms2AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms2Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms2AttrVal {
        SpiFmemPms2AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms2AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms2AttrVal) -> SpiFmemPms2AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS3_ATTR Register
///
/// MSPI flash PMS section 3 attribute register
///
/// Address offset: 0x010c
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
///
/// - **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiFmemPms3Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms3Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms3Attr {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_ECC**
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_FMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_FMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_FMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_FMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_FMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_FMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_fmem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_FMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_FMEM_PMS_ECC_MASK);
    }

    /// **SPI_FMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_FMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_FMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_FMEM_PMS3_ATTR value type for batch read/modify/write operations
///
/// Use `spifmempms3attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms3attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms3AttrVal {
    bits: u32,
}

impl SpiFmemPms3AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_fmem_pms_ecc(&self) -> bool {
        (self.bits & SpiFmemPms3Attr::SPI_FMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiFmemPms3Attr::SPI_FMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_fmem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiFmemPms3Attr::SPI_FMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_FMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 flash PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms3Attr::SPI_FMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiFmemPms3Attr::SPI_FMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 flash PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms3Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms3Attr::SPI_FMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 flash PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_fmem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiFmemPms3Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiFmemPms3Attr::SPI_FMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiFmemPms3AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms3Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms3AttrVal {
        SpiFmemPms3AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms3AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms3AttrVal) -> SpiFmemPms3AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS0_ADDR Register
///
/// SPI1 flash PMS section 0 start address register
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 flash PMS section %s start address value
#[repr(transparent)]
pub struct SpiFmemPms0Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms0Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms0Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 flash PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_FMEM_PMS0_ADDR value type for batch read/modify/write operations
///
/// Use `spifmempms0addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms0addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms0AddrVal {
    bits: u32,
}

impl SpiFmemPms0AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiFmemPms0Addr::S_MASK) >> SpiFmemPms0Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms0Addr::S_MASK)
                | ((value << SpiFmemPms0Addr::S_OFFSET) & SpiFmemPms0Addr::S_MASK),
        }
    }
}

impl Default for SpiFmemPms0AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiFmemPms0Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms0AddrVal {
        SpiFmemPms0AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms0AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms0AddrVal) -> SpiFmemPms0AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS1_ADDR Register
///
/// SPI1 flash PMS section 1 start address register
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 flash PMS section %s start address value
#[repr(transparent)]
pub struct SpiFmemPms1Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms1Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms1Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 flash PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_FMEM_PMS1_ADDR value type for batch read/modify/write operations
///
/// Use `spifmempms1addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms1addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms1AddrVal {
    bits: u32,
}

impl SpiFmemPms1AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiFmemPms1Addr::S_MASK) >> SpiFmemPms1Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms1Addr::S_MASK)
                | ((value << SpiFmemPms1Addr::S_OFFSET) & SpiFmemPms1Addr::S_MASK),
        }
    }
}

impl Default for SpiFmemPms1AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiFmemPms1Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms1AddrVal {
        SpiFmemPms1AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms1AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms1AddrVal) -> SpiFmemPms1AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS2_ADDR Register
///
/// SPI1 flash PMS section 2 start address register
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 flash PMS section %s start address value
#[repr(transparent)]
pub struct SpiFmemPms2Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms2Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms2Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 flash PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_FMEM_PMS2_ADDR value type for batch read/modify/write operations
///
/// Use `spifmempms2addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms2addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms2AddrVal {
    bits: u32,
}

impl SpiFmemPms2AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiFmemPms2Addr::S_MASK) >> SpiFmemPms2Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms2Addr::S_MASK)
                | ((value << SpiFmemPms2Addr::S_OFFSET) & SpiFmemPms2Addr::S_MASK),
        }
    }
}

impl Default for SpiFmemPms2AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiFmemPms2Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms2AddrVal {
        SpiFmemPms2AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms2AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms2AddrVal) -> SpiFmemPms2AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS3_ADDR Register
///
/// SPI1 flash PMS section 3 start address register
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 flash PMS section %s start address value
#[repr(transparent)]
pub struct SpiFmemPms3Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms3Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms3Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 flash PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_FMEM_PMS3_ADDR value type for batch read/modify/write operations
///
/// Use `spifmempms3addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms3addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms3AddrVal {
    bits: u32,
}

impl SpiFmemPms3AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiFmemPms3Addr::S_MASK) >> SpiFmemPms3Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms3Addr::S_MASK)
                | ((value << SpiFmemPms3Addr::S_OFFSET) & SpiFmemPms3Addr::S_MASK),
        }
    }
}

impl Default for SpiFmemPms3AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiFmemPms3Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms3AddrVal {
        SpiFmemPms3AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms3AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms3AddrVal) -> SpiFmemPms3AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS0_SIZE Register
///
/// SPI1 flash PMS section 0 start address register
///
/// Address offset: 0x0120
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiFmemPms0Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms0Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms0Size {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_SIZE**
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_FMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_fmem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_SIZE_MASK) >> Self::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_fmem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_FMEM_PMS_SIZE_OFFSET) & Self::SPI_FMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_FMEM_PMS0_SIZE value type for batch read/modify/write operations
///
/// Use `spifmempms0size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms0size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms0SizeVal {
    bits: u32,
}

impl SpiFmemPms0SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self) -> u32 {
        (self.bits & SpiFmemPms0Size::SPI_FMEM_PMS_SIZE_MASK)
            >> SpiFmemPms0Size::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_fmem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms0Size::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << SpiFmemPms0Size::SPI_FMEM_PMS_SIZE_OFFSET)
                    & SpiFmemPms0Size::SPI_FMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiFmemPms0SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms0Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms0SizeVal {
        SpiFmemPms0SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms0SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms0SizeVal) -> SpiFmemPms0SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS1_SIZE Register
///
/// SPI1 flash PMS section 1 start address register
///
/// Address offset: 0x0124
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiFmemPms1Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms1Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms1Size {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_SIZE**
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_FMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_fmem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_SIZE_MASK) >> Self::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_fmem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_FMEM_PMS_SIZE_OFFSET) & Self::SPI_FMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_FMEM_PMS1_SIZE value type for batch read/modify/write operations
///
/// Use `spifmempms1size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms1size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms1SizeVal {
    bits: u32,
}

impl SpiFmemPms1SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self) -> u32 {
        (self.bits & SpiFmemPms1Size::SPI_FMEM_PMS_SIZE_MASK)
            >> SpiFmemPms1Size::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_fmem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms1Size::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << SpiFmemPms1Size::SPI_FMEM_PMS_SIZE_OFFSET)
                    & SpiFmemPms1Size::SPI_FMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiFmemPms1SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms1Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms1SizeVal {
        SpiFmemPms1SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms1SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms1SizeVal) -> SpiFmemPms1SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS2_SIZE Register
///
/// SPI1 flash PMS section 2 start address register
///
/// Address offset: 0x0128
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiFmemPms2Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms2Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms2Size {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_SIZE**
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_FMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_fmem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_SIZE_MASK) >> Self::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_fmem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_FMEM_PMS_SIZE_OFFSET) & Self::SPI_FMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_FMEM_PMS2_SIZE value type for batch read/modify/write operations
///
/// Use `spifmempms2size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms2size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms2SizeVal {
    bits: u32,
}

impl SpiFmemPms2SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self) -> u32 {
        (self.bits & SpiFmemPms2Size::SPI_FMEM_PMS_SIZE_MASK)
            >> SpiFmemPms2Size::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_fmem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms2Size::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << SpiFmemPms2Size::SPI_FMEM_PMS_SIZE_OFFSET)
                    & SpiFmemPms2Size::SPI_FMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiFmemPms2SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms2Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms2SizeVal {
        SpiFmemPms2SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms2SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms2SizeVal) -> SpiFmemPms2SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_FMEM_PMS3_SIZE Register
///
/// SPI1 flash PMS section 3 start address register
///
/// Address offset: 0x012c
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiFmemPms3Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiFmemPms3Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiFmemPms3Size {
    // ======== Field Constants ========

    /// **SPI_FMEM_PMS_SIZE**
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_FMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_FMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_fmem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_PMS_SIZE_MASK) >> Self::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_FMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_fmem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_FMEM_PMS_SIZE_OFFSET) & Self::SPI_FMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_FMEM_PMS3_SIZE value type for batch read/modify/write operations
///
/// Use `spifmempms3size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spifmempms3size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiFmemPms3SizeVal {
    bits: u32,
}

impl SpiFmemPms3SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self) -> u32 {
        (self.bits & SpiFmemPms3Size::SPI_FMEM_PMS_SIZE_MASK)
            >> SpiFmemPms3Size::SPI_FMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_FMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_fmem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiFmemPms3Size::SPI_FMEM_PMS_SIZE_MASK)
                | ((value << SpiFmemPms3Size::SPI_FMEM_PMS_SIZE_OFFSET)
                    & SpiFmemPms3Size::SPI_FMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiFmemPms3SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiFmemPms3Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiFmemPms3SizeVal {
        SpiFmemPms3SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiFmemPms3SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiFmemPms3SizeVal) -> SpiFmemPms3SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS0_ATTR Register
///
/// SPI1 flash PMS section 0 start address register
///
/// Address offset: 0x0130
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
///
/// - **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiSmemPms0Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms0Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms0Attr {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_ECC**
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_SMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_SMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_SMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_SMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_smem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_SMEM_PMS0_ATTR value type for batch read/modify/write operations
///
/// Use `spismempms0attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms0attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms0AttrVal {
    bits: u32,
}

impl SpiSmemPms0AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_smem_pms_ecc(&self) -> bool {
        (self.bits & SpiSmemPms0Attr::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiSmemPms0Attr::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiSmemPms0Attr::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_smem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms0Attr::SPI_SMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiSmemPms0Attr::SPI_SMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms0Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms0Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms0Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms0Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiSmemPms0AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms0Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms0AttrVal {
        SpiSmemPms0AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms0AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms0AttrVal) -> SpiSmemPms0AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS1_ATTR Register
///
/// SPI1 flash PMS section 1 start address register
///
/// Address offset: 0x0134
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
///
/// - **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiSmemPms1Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms1Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms1Attr {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_ECC**
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_SMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_SMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_SMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_SMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_smem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_SMEM_PMS1_ATTR value type for batch read/modify/write operations
///
/// Use `spismempms1attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms1attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms1AttrVal {
    bits: u32,
}

impl SpiSmemPms1AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_smem_pms_ecc(&self) -> bool {
        (self.bits & SpiSmemPms1Attr::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiSmemPms1Attr::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiSmemPms1Attr::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_smem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms1Attr::SPI_SMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiSmemPms1Attr::SPI_SMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms1Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms1Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms1Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms1Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiSmemPms1AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms1Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms1AttrVal {
        SpiSmemPms1AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms1AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms1AttrVal) -> SpiSmemPms1AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS2_ATTR Register
///
/// SPI1 flash PMS section 2 start address register
///
/// Address offset: 0x0138
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
///
/// - **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiSmemPms2Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms2Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms2Attr {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_ECC**
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_SMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_SMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_SMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_SMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_smem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_SMEM_PMS2_ATTR value type for batch read/modify/write operations
///
/// Use `spismempms2attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms2attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms2AttrVal {
    bits: u32,
}

impl SpiSmemPms2AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_smem_pms_ecc(&self) -> bool {
        (self.bits & SpiSmemPms2Attr::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiSmemPms2Attr::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiSmemPms2Attr::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_smem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms2Attr::SPI_SMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiSmemPms2Attr::SPI_SMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms2Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms2Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms2Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms2Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiSmemPms2AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms2Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms2AttrVal {
        SpiSmemPms2AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms2AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms2AttrVal) -> SpiSmemPms2AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS3_ATTR Register
///
/// SPI1 flash PMS section 3 start address register
///
/// Address offset: 0x013c
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_ECC** `[2]` (RW)
///
///   SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
///
/// - **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
///
///   1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
///
/// - **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
///
///   1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
#[repr(transparent)]
pub struct SpiSmemPms3Attr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms3Attr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms3Attr {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_ECC**
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_PMS_ECC_OFFSET: u32 = 2;
    pub const SPI_SMEM_PMS_ECC_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_PMS_ECC: u32 = 1 << 2;

    /// **SPI_SMEM_PMS_WR_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_PMS_WR_ATTR_OFFSET: u32 = 1;
    pub const SPI_SMEM_PMS_WR_ATTR_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_PMS_WR_ATTR: u32 = 1 << 1;

    /// **SPI_SMEM_PMS_RD_ATTR**
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_PMS_RD_ATTR_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_RD_ATTR_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_PMS_RD_ATTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_ECC** `[2]` (RW)
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub fn spi_smem_pms_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// Clear SPI_SMEM_PMS_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_ecc(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SMEM_PMS_ECC_MASK);
    }

    /// **SPI_SMEM_PMS_WR_ATTR** `[1]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_WR_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_wr_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_WR_ATTR_MASK);
    }

    /// **SPI_SMEM_PMS_RD_ATTR** `[0]` (RW)
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }

    /// Clear SPI_SMEM_PMS_RD_ATTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_pms_rd_attr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_PMS_RD_ATTR_MASK);
    }
}

/// SPI_SMEM_PMS3_ATTR value type for batch read/modify/write operations
///
/// Use `spismempms3attr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms3attr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms3AttrVal {
    bits: u32,
}

impl SpiSmemPms3AttrVal {
    /// Create from raw bits
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

    /// Read **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn spi_smem_pms_ecc(&self) -> bool {
        (self.bits & SpiSmemPms3Attr::SPI_SMEM_PMS_ECC_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_wr_attr(&self) -> bool {
        (self.bits & SpiSmemPms3Attr::SPI_SMEM_PMS_WR_ATTR_MASK) != 0
    }

    /// Read **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn spi_smem_pms_rd_attr(&self) -> bool {
        (self.bits & SpiSmemPms3Attr::SPI_SMEM_PMS_RD_ATTR_MASK) != 0
    }

    /// Set **SPI_SMEM_PMS_ECC** `[2:2]`
    ///
    /// SPI1 external RAM PMS section %s ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section %s is configured by registers SPI_SMEM_PMS%s_ADDR_REG and SPI_SMEM_PMS%s_SIZE_REG.
    #[inline(always)]
    pub const fn with_spi_smem_pms_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms3Attr::SPI_SMEM_PMS_ECC_MASK
            } else {
                self.bits & !SpiSmemPms3Attr::SPI_SMEM_PMS_ECC_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_WR_ATTR** `[1:1]`
    ///
    /// 1: SPI1 external RAM PMS section %s write accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_wr_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms3Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms3Attr::SPI_SMEM_PMS_WR_ATTR_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PMS_RD_ATTR** `[0:0]`
    ///
    /// 1: SPI1 external RAM PMS section %s read accessible. 0: Not allowed.
    #[inline(always)]
    pub const fn with_spi_smem_pms_rd_attr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemPms3Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            } else {
                self.bits & !SpiSmemPms3Attr::SPI_SMEM_PMS_RD_ATTR_MASK
            },
        }
    }
}

impl Default for SpiSmemPms3AttrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms3Attr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms3AttrVal {
        SpiSmemPms3AttrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms3AttrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms3AttrVal) -> SpiSmemPms3AttrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS0_ADDR Register
///
/// SPI1 external RAM PMS section 0 start address register
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 external RAM PMS section %s start address value
#[repr(transparent)]
pub struct SpiSmemPms0Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms0Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms0Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 external RAM PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_SMEM_PMS0_ADDR value type for batch read/modify/write operations
///
/// Use `spismempms0addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms0addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms0AddrVal {
    bits: u32,
}

impl SpiSmemPms0AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiSmemPms0Addr::S_MASK) >> SpiSmemPms0Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms0Addr::S_MASK)
                | ((value << SpiSmemPms0Addr::S_OFFSET) & SpiSmemPms0Addr::S_MASK),
        }
    }
}

impl Default for SpiSmemPms0AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemPms0Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms0AddrVal {
        SpiSmemPms0AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms0AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms0AddrVal) -> SpiSmemPms0AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS1_ADDR Register
///
/// SPI1 external RAM PMS section 1 start address register
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 external RAM PMS section %s start address value
#[repr(transparent)]
pub struct SpiSmemPms1Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms1Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms1Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 external RAM PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_SMEM_PMS1_ADDR value type for batch read/modify/write operations
///
/// Use `spismempms1addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms1addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms1AddrVal {
    bits: u32,
}

impl SpiSmemPms1AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiSmemPms1Addr::S_MASK) >> SpiSmemPms1Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms1Addr::S_MASK)
                | ((value << SpiSmemPms1Addr::S_OFFSET) & SpiSmemPms1Addr::S_MASK),
        }
    }
}

impl Default for SpiSmemPms1AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemPms1Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms1AddrVal {
        SpiSmemPms1AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms1AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms1AddrVal) -> SpiSmemPms1AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS2_ADDR Register
///
/// SPI1 external RAM PMS section 2 start address register
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 external RAM PMS section %s start address value
#[repr(transparent)]
pub struct SpiSmemPms2Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms2Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms2Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 external RAM PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_SMEM_PMS2_ADDR value type for batch read/modify/write operations
///
/// Use `spismempms2addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms2addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms2AddrVal {
    bits: u32,
}

impl SpiSmemPms2AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiSmemPms2Addr::S_MASK) >> SpiSmemPms2Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms2Addr::S_MASK)
                | ((value << SpiSmemPms2Addr::S_OFFSET) & SpiSmemPms2Addr::S_MASK),
        }
    }
}

impl Default for SpiSmemPms2AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemPms2Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms2AddrVal {
        SpiSmemPms2AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms2AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms2AddrVal) -> SpiSmemPms2AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS3_ADDR Register
///
/// SPI1 external RAM PMS section 3 start address register
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **S** `[26:0]` (RW)
///
///   SPI1 external RAM PMS section %s start address value
#[repr(transparent)]
pub struct SpiSmemPms3Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms3Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms3Addr {
    // ======== Field Constants ========

    /// **S**
    ///
    /// SPI1 external RAM PMS section %s start address value
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const S_OFFSET: u32 = 0;
    pub const S_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> u32 {
        (self.value.read_bits() & Self::S_MASK) >> Self::S_OFFSET
    }

    /// Write **S** `[26:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub fn write_s(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::S_MASK) | ((value << Self::S_OFFSET) & Self::S_MASK));
    }
}

/// SPI_SMEM_PMS3_ADDR value type for batch read/modify/write operations
///
/// Use `spismempms3addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms3addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms3AddrVal {
    bits: u32,
}

impl SpiSmemPms3AddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn s(&self) -> u32 {
        (self.bits & SpiSmemPms3Addr::S_MASK) >> SpiSmemPms3Addr::S_OFFSET
    }

    /// Set **S** `[26:0]`
    ///
    /// SPI1 external RAM PMS section %s start address value
    #[inline(always)]
    pub const fn with_s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms3Addr::S_MASK)
                | ((value << SpiSmemPms3Addr::S_OFFSET) & SpiSmemPms3Addr::S_MASK),
        }
    }
}

impl Default for SpiSmemPms3AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemPms3Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms3AddrVal {
        SpiSmemPms3AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms3AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms3AddrVal) -> SpiSmemPms3AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS0_SIZE Register
///
/// SPI1 external RAM PMS section 0 start address register
///
/// Address offset: 0x0150
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiSmemPms0Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms0Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms0Size {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_SIZE**
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_SMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_smem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_SIZE_MASK) >> Self::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_smem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_SMEM_PMS_SIZE_OFFSET) & Self::SPI_SMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_SMEM_PMS0_SIZE value type for batch read/modify/write operations
///
/// Use `spismempms0size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms0size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms0SizeVal {
    bits: u32,
}

impl SpiSmemPms0SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self) -> u32 {
        (self.bits & SpiSmemPms0Size::SPI_SMEM_PMS_SIZE_MASK)
            >> SpiSmemPms0Size::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_smem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms0Size::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << SpiSmemPms0Size::SPI_SMEM_PMS_SIZE_OFFSET)
                    & SpiSmemPms0Size::SPI_SMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiSmemPms0SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms0Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms0SizeVal {
        SpiSmemPms0SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms0SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms0SizeVal) -> SpiSmemPms0SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS1_SIZE Register
///
/// SPI1 external RAM PMS section 1 start address register
///
/// Address offset: 0x0154
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiSmemPms1Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms1Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms1Size {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_SIZE**
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_SMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_smem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_SIZE_MASK) >> Self::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_smem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_SMEM_PMS_SIZE_OFFSET) & Self::SPI_SMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_SMEM_PMS1_SIZE value type for batch read/modify/write operations
///
/// Use `spismempms1size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms1size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms1SizeVal {
    bits: u32,
}

impl SpiSmemPms1SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self) -> u32 {
        (self.bits & SpiSmemPms1Size::SPI_SMEM_PMS_SIZE_MASK)
            >> SpiSmemPms1Size::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_smem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms1Size::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << SpiSmemPms1Size::SPI_SMEM_PMS_SIZE_OFFSET)
                    & SpiSmemPms1Size::SPI_SMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiSmemPms1SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms1Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms1SizeVal {
        SpiSmemPms1SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms1SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms1SizeVal) -> SpiSmemPms1SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS2_SIZE Register
///
/// SPI1 external RAM PMS section 2 start address register
///
/// Address offset: 0x0158
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiSmemPms2Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms2Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms2Size {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_SIZE**
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_SMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_smem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_SIZE_MASK) >> Self::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_smem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_SMEM_PMS_SIZE_OFFSET) & Self::SPI_SMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_SMEM_PMS2_SIZE value type for batch read/modify/write operations
///
/// Use `spismempms2size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms2size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms2SizeVal {
    bits: u32,
}

impl SpiSmemPms2SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self) -> u32 {
        (self.bits & SpiSmemPms2Size::SPI_SMEM_PMS_SIZE_MASK)
            >> SpiSmemPms2Size::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_smem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms2Size::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << SpiSmemPms2Size::SPI_SMEM_PMS_SIZE_OFFSET)
                    & SpiSmemPms2Size::SPI_SMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiSmemPms2SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms2Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms2SizeVal {
        SpiSmemPms2SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms2SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms2SizeVal) -> SpiSmemPms2SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_PMS3_SIZE Register
///
/// SPI1 external RAM PMS section 3 start address register
///
/// Address offset: 0x015c
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
///
///   SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
#[repr(transparent)]
pub struct SpiSmemPms3Size {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemPms3Size {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemPms3Size {
    // ======== Field Constants ========

    /// **SPI_SMEM_PMS_SIZE**
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SPI_SMEM_PMS_SIZE_OFFSET: u32 = 0;
    pub const SPI_SMEM_PMS_SIZE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_smem_pms_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_PMS_SIZE_MASK) >> Self::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Write **SPI_SMEM_PMS_SIZE** `[14:0]` (RW)
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn write_spi_smem_pms_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << Self::SPI_SMEM_PMS_SIZE_OFFSET) & Self::SPI_SMEM_PMS_SIZE_MASK)
        });
    }
}

/// SPI_SMEM_PMS3_SIZE value type for batch read/modify/write operations
///
/// Use `spismempms3size.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismempms3size.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemPms3SizeVal {
    bits: u32,
}

impl SpiSmemPms3SizeVal {
    /// Create from raw bits
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
        Self { bits: 0x00001000 }
    }

    /// Read **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self) -> u32 {
        (self.bits & SpiSmemPms3Size::SPI_SMEM_PMS_SIZE_MASK)
            >> SpiSmemPms3Size::SPI_SMEM_PMS_SIZE_OFFSET
    }

    /// Set **SPI_SMEM_PMS_SIZE** `[14:0]`
    ///
    /// SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)
    #[inline(always)]
    pub const fn with_spi_smem_pms_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemPms3Size::SPI_SMEM_PMS_SIZE_MASK)
                | ((value << SpiSmemPms3Size::SPI_SMEM_PMS_SIZE_OFFSET)
                    & SpiSmemPms3Size::SPI_SMEM_PMS_SIZE_MASK),
        }
    }
}

impl Default for SpiSmemPms3SizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemPms3Size {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemPms3SizeVal {
        SpiSmemPms3SizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemPms3SizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemPms3SizeVal) -> SpiSmemPms3SizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_PMS_REJECT Register
///
/// SPI1 access reject register
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **SPI_MEM_PMS_IVD** `[31]` (RO)
///
///   1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
///
/// - **SPI_MEM_PMS_MULTI_HIT** `[30]` (RO)
///
///   1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
///
/// - **SPI_MEM_PMS_ST** `[29]` (RO)
///
///   1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
///
/// - **SPI_MEM_PMS_LD** `[28]` (RO)
///
///   1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
///
/// - **SPI_MEM_PM_EN** `[27]` (RW)
///
///   Set this bit to enable SPI0/1 transfer permission control function.
///
/// - **SPI_MEM_REJECT_ADDR** `[26:0]` (RO)
///
///   This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
#[repr(transparent)]
pub struct SpiMemPmsReject {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemPmsReject {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemPmsReject {
    // ======== Field Constants ========

    /// **SPI_MEM_PMS_IVD**
    ///
    /// 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_PMS_IVD_OFFSET: u32 = 31;
    pub const SPI_MEM_PMS_IVD_MASK: u32 = 0x80000000;
    pub const SPI_MEM_PMS_IVD: u32 = 1 << 31;

    /// **SPI_MEM_PMS_MULTI_HIT**
    ///
    /// 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_PMS_MULTI_HIT_OFFSET: u32 = 30;
    pub const SPI_MEM_PMS_MULTI_HIT_MASK: u32 = 0x40000000;
    pub const SPI_MEM_PMS_MULTI_HIT: u32 = 1 << 30;

    /// **SPI_MEM_PMS_ST**
    ///
    /// 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_PMS_ST_OFFSET: u32 = 29;
    pub const SPI_MEM_PMS_ST_MASK: u32 = 0x20000000;
    pub const SPI_MEM_PMS_ST: u32 = 1 << 29;

    /// **SPI_MEM_PMS_LD**
    ///
    /// 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_PMS_LD_OFFSET: u32 = 28;
    pub const SPI_MEM_PMS_LD_MASK: u32 = 0x10000000;
    pub const SPI_MEM_PMS_LD: u32 = 1 << 28;

    /// **SPI_MEM_PM_EN**
    ///
    /// Set this bit to enable SPI0/1 transfer permission control function.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MEM_PM_EN_OFFSET: u32 = 27;
    pub const SPI_MEM_PM_EN_MASK: u32 = 0x08000000;
    pub const SPI_MEM_PM_EN: u32 = 1 << 27;

    /// **SPI_MEM_REJECT_ADDR**
    ///
    /// This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const SPI_MEM_REJECT_ADDR_OFFSET: u32 = 0;
    pub const SPI_MEM_REJECT_ADDR_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_PMS_IVD** `[31]` (RO)
    ///
    /// 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_pms_ivd(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_IVD_MASK) != 0
    }

    /// **SPI_MEM_PMS_MULTI_HIT** `[30]` (RO)
    ///
    /// 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_pms_multi_hit(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_MULTI_HIT_MASK) != 0
    }

    /// **SPI_MEM_PMS_ST** `[29]` (RO)
    ///
    /// 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_pms_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_ST_MASK) != 0
    }

    /// **SPI_MEM_PMS_LD** `[28]` (RO)
    ///
    /// 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_pms_ld(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PMS_LD_MASK) != 0
    }

    /// **SPI_MEM_PM_EN** `[27]` (RW)
    ///
    /// Set this bit to enable SPI0/1 transfer permission control function.
    #[inline(always)]
    pub fn spi_mem_pm_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PM_EN_MASK) != 0
    }

    /// Set SPI_MEM_PM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pm_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_PM_EN_MASK);
    }

    /// Clear SPI_MEM_PM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pm_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_PM_EN_MASK);
    }

    /// **SPI_MEM_REJECT_ADDR** `[26:0]` (RO)
    ///
    /// This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_REJECT_ADDR_MASK)
            >> Self::SPI_MEM_REJECT_ADDR_OFFSET
    }
}

/// SPI_MEM_PMS_REJECT value type for batch read/modify/write operations
///
/// Use `spimempmsreject.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimempmsreject.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemPmsRejectVal {
    bits: u32,
}

impl SpiMemPmsRejectVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_PMS_IVD** `[31:31]`
    ///
    /// 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_pms_ivd(&self) -> bool {
        (self.bits & SpiMemPmsReject::SPI_MEM_PMS_IVD_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_MULTI_HIT** `[30:30]`
    ///
    /// 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_pms_multi_hit(&self) -> bool {
        (self.bits & SpiMemPmsReject::SPI_MEM_PMS_MULTI_HIT_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_ST** `[29:29]`
    ///
    /// 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_pms_st(&self) -> bool {
        (self.bits & SpiMemPmsReject::SPI_MEM_PMS_ST_MASK) != 0
    }

    /// Read **SPI_MEM_PMS_LD** `[28:28]`
    ///
    /// 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_pms_ld(&self) -> bool {
        (self.bits & SpiMemPmsReject::SPI_MEM_PMS_LD_MASK) != 0
    }

    /// Read **SPI_MEM_PM_EN** `[27:27]`
    ///
    /// Set this bit to enable SPI0/1 transfer permission control function.
    #[inline(always)]
    pub const fn spi_mem_pm_en(&self) -> bool {
        (self.bits & SpiMemPmsReject::SPI_MEM_PM_EN_MASK) != 0
    }

    /// Read **SPI_MEM_REJECT_ADDR** `[26:0]`
    ///
    /// This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_reject_addr(&self) -> u32 {
        (self.bits & SpiMemPmsReject::SPI_MEM_REJECT_ADDR_MASK)
            >> SpiMemPmsReject::SPI_MEM_REJECT_ADDR_OFFSET
    }

    /// Set **SPI_MEM_PM_EN** `[27:27]`
    ///
    /// Set this bit to enable SPI0/1 transfer permission control function.
    #[inline(always)]
    pub const fn with_spi_mem_pm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemPmsReject::SPI_MEM_PM_EN_MASK
            } else {
                self.bits & !SpiMemPmsReject::SPI_MEM_PM_EN_MASK
            },
        }
    }
}

impl Default for SpiMemPmsRejectVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemPmsReject {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemPmsRejectVal {
        SpiMemPmsRejectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemPmsRejectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemPmsRejectVal) -> SpiMemPmsRejectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_ECC_CTRL Register
///
/// MSPI ECC control register
///
/// Address offset: 0x0168
/// Reset value: 0x01005000
///
/// ## Fields
///
/// - **SPI_MEM_ECC_ERR_BITS** `[31:25]` (RO)
///
///   Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)
///
/// - **SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN** `[24]` (RW)
///
///   1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information.
///
/// - **SPI_MEM_USR_ECC_ADDR_EN** `[21]` (RW)
///
///   Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer.
///
/// - **SPI_FMEM_ECC_ADDR_EN** `[20]` (RW)
///
///   Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1.
///
/// - **SPI_FMEM_PAGE_SIZE** `[19:18]` (RW)
///
///   Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
///
/// - **SPI_FMEM_ECC_ERR_INT_EN** `[17]` (RW)
///
///   Set this bit to calculate the error times of MSPI ECC read when accesses to flash.
///
/// - **SPI_FMEM_ECC_ERR_INT_NUM** `[16:11]` (RW)
///
///   Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
///
/// - **SPI_MEM_ECC_ERR_CNT** `[10:5]` (RO)
///
///   This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
#[repr(transparent)]
pub struct SpiMemEccCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemEccCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemEccCtrl {
    // ======== Field Constants ========

    /// **SPI_MEM_ECC_ERR_BITS**
    ///
    /// Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)
    ///
    /// Bits `[31:25]`, mask `0xfe000000`
    pub const SPI_MEM_ECC_ERR_BITS_OFFSET: u32 = 25;
    pub const SPI_MEM_ECC_ERR_BITS_MASK: u32 = 0xfe000000;

    /// **SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN**
    ///
    /// 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_OFFSET: u32 = 24;
    pub const SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK: u32 = 0x01000000;
    pub const SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN: u32 = 1 << 24;

    /// **SPI_MEM_USR_ECC_ADDR_EN**
    ///
    /// Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_MEM_USR_ECC_ADDR_EN_OFFSET: u32 = 21;
    pub const SPI_MEM_USR_ECC_ADDR_EN_MASK: u32 = 0x00200000;
    pub const SPI_MEM_USR_ECC_ADDR_EN: u32 = 1 << 21;

    /// **SPI_FMEM_ECC_ADDR_EN**
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_FMEM_ECC_ADDR_EN_OFFSET: u32 = 20;
    pub const SPI_FMEM_ECC_ADDR_EN_MASK: u32 = 0x00100000;
    pub const SPI_FMEM_ECC_ADDR_EN: u32 = 1 << 20;

    /// **SPI_FMEM_PAGE_SIZE**
    ///
    /// Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const SPI_FMEM_PAGE_SIZE_OFFSET: u32 = 18;
    pub const SPI_FMEM_PAGE_SIZE_MASK: u32 = 0x000c0000;

    /// **SPI_FMEM_ECC_ERR_INT_EN**
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to flash.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_FMEM_ECC_ERR_INT_EN_OFFSET: u32 = 17;
    pub const SPI_FMEM_ECC_ERR_INT_EN_MASK: u32 = 0x00020000;
    pub const SPI_FMEM_ECC_ERR_INT_EN: u32 = 1 << 17;

    /// **SPI_FMEM_ECC_ERR_INT_NUM**
    ///
    /// Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
    ///
    /// Bits `[16:11]`, mask `0x0001f800`
    pub const SPI_FMEM_ECC_ERR_INT_NUM_OFFSET: u32 = 11;
    pub const SPI_FMEM_ECC_ERR_INT_NUM_MASK: u32 = 0x0001f800;

    /// **SPI_MEM_ECC_ERR_CNT**
    ///
    /// This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    ///
    /// Bits `[10:5]`, mask `0x000007e0`
    pub const SPI_MEM_ECC_ERR_CNT_OFFSET: u32 = 5;
    pub const SPI_MEM_ECC_ERR_CNT_MASK: u32 = 0x000007e0;

    // ======== Field Accessors ========

    /// **SPI_MEM_ECC_ERR_BITS** `[31:25]` (RO)
    ///
    /// Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)
    #[inline(always)]
    pub fn spi_mem_ecc_err_bits(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_BITS_MASK)
            >> Self::SPI_MEM_ECC_ERR_BITS_OFFSET
    }

    /// **SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN** `[24]` (RW)
    ///
    /// 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information.
    #[inline(always)]
    pub fn spi_mem_ecc_continue_record_err_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK) != 0
    }

    /// Set SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ecc_continue_record_err_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK);
    }

    /// Clear SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ecc_continue_record_err_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK);
    }

    /// **SPI_MEM_USR_ECC_ADDR_EN** `[21]` (RW)
    ///
    /// Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer.
    #[inline(always)]
    pub fn spi_mem_usr_ecc_addr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_ECC_ADDR_EN_MASK) != 0
    }

    /// Set SPI_MEM_USR_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_ECC_ADDR_EN_MASK);
    }

    /// Clear SPI_MEM_USR_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_ECC_ADDR_EN_MASK);
    }

    /// **SPI_FMEM_ECC_ADDR_EN** `[20]` (RW)
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub fn spi_fmem_ecc_addr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_ECC_ADDR_EN_MASK) != 0
    }

    /// Set SPI_FMEM_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_ECC_ADDR_EN_MASK);
    }

    /// Clear SPI_FMEM_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_ECC_ADDR_EN_MASK);
    }

    /// **SPI_FMEM_PAGE_SIZE** `[19:18]` (RW)
    ///
    /// Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub fn spi_fmem_page_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_PAGE_SIZE_MASK) >> Self::SPI_FMEM_PAGE_SIZE_OFFSET
    }

    /// Write **SPI_FMEM_PAGE_SIZE** `[19:18]` (RW)
    ///
    /// Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub fn write_spi_fmem_page_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_PAGE_SIZE_MASK)
                | ((value << Self::SPI_FMEM_PAGE_SIZE_OFFSET) & Self::SPI_FMEM_PAGE_SIZE_MASK)
        });
    }

    /// **SPI_FMEM_ECC_ERR_INT_EN** `[17]` (RW)
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to flash.
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_ECC_ERR_INT_EN_MASK) != 0
    }

    /// Set SPI_FMEM_ECC_ERR_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_ecc_err_int_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_ECC_ERR_INT_EN_MASK);
    }

    /// Clear SPI_FMEM_ECC_ERR_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_ecc_err_int_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_ECC_ERR_INT_EN_MASK);
    }

    /// **SPI_FMEM_ECC_ERR_INT_NUM** `[16:11]` (RW)
    ///
    /// Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_FMEM_ECC_ERR_INT_NUM_MASK)
            >> Self::SPI_FMEM_ECC_ERR_INT_NUM_OFFSET
    }

    /// Write **SPI_FMEM_ECC_ERR_INT_NUM** `[16:11]` (RW)
    ///
    /// Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub fn write_spi_fmem_ecc_err_int_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_FMEM_ECC_ERR_INT_NUM_MASK)
                | ((value << Self::SPI_FMEM_ECC_ERR_INT_NUM_OFFSET)
                    & Self::SPI_FMEM_ECC_ERR_INT_NUM_MASK)
        });
    }

    /// **SPI_MEM_ECC_ERR_CNT** `[10:5]` (RO)
    ///
    /// This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_ecc_err_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_CNT_MASK)
            >> Self::SPI_MEM_ECC_ERR_CNT_OFFSET
    }
}

/// SPI_MEM_ECC_CTRL value type for batch read/modify/write operations
///
/// Use `spimemeccctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemeccctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemEccCtrlVal {
    bits: u32,
}

impl SpiMemEccCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x01005000 }
    }

    /// Read **SPI_MEM_ECC_ERR_BITS** `[31:25]`
    ///
    /// Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)
    #[inline(always)]
    pub const fn spi_mem_ecc_err_bits(&self) -> u32 {
        (self.bits & SpiMemEccCtrl::SPI_MEM_ECC_ERR_BITS_MASK)
            >> SpiMemEccCtrl::SPI_MEM_ECC_ERR_BITS_OFFSET
    }

    /// Read **SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN** `[24:24]`
    ///
    /// 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information.
    #[inline(always)]
    pub const fn spi_mem_ecc_continue_record_err_en(&self) -> bool {
        (self.bits & SpiMemEccCtrl::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK) != 0
    }

    /// Read **SPI_MEM_USR_ECC_ADDR_EN** `[21:21]`
    ///
    /// Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer.
    #[inline(always)]
    pub const fn spi_mem_usr_ecc_addr_en(&self) -> bool {
        (self.bits & SpiMemEccCtrl::SPI_MEM_USR_ECC_ADDR_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_ECC_ADDR_EN** `[20:20]`
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub const fn spi_fmem_ecc_addr_en(&self) -> bool {
        (self.bits & SpiMemEccCtrl::SPI_FMEM_ECC_ADDR_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_PAGE_SIZE** `[19:18]`
    ///
    /// Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub const fn spi_fmem_page_size(&self) -> u32 {
        (self.bits & SpiMemEccCtrl::SPI_FMEM_PAGE_SIZE_MASK)
            >> SpiMemEccCtrl::SPI_FMEM_PAGE_SIZE_OFFSET
    }

    /// Read **SPI_FMEM_ECC_ERR_INT_EN** `[17:17]`
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to flash.
    #[inline(always)]
    pub const fn spi_fmem_ecc_err_int_en(&self) -> bool {
        (self.bits & SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_ECC_ERR_INT_NUM** `[16:11]`
    ///
    /// Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn spi_fmem_ecc_err_int_num(&self) -> u32 {
        (self.bits & SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_NUM_MASK)
            >> SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_NUM_OFFSET
    }

    /// Read **SPI_MEM_ECC_ERR_CNT** `[10:5]`
    ///
    /// This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_ecc_err_cnt(&self) -> u32 {
        (self.bits & SpiMemEccCtrl::SPI_MEM_ECC_ERR_CNT_MASK)
            >> SpiMemEccCtrl::SPI_MEM_ECC_ERR_CNT_OFFSET
    }

    /// Set **SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN** `[24:24]`
    ///
    /// 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information.
    #[inline(always)]
    pub const fn with_spi_mem_ecc_continue_record_err_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemEccCtrl::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK
            } else {
                self.bits & !SpiMemEccCtrl::SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_ECC_ADDR_EN** `[21:21]`
    ///
    /// Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer.
    #[inline(always)]
    pub const fn with_spi_mem_usr_ecc_addr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemEccCtrl::SPI_MEM_USR_ECC_ADDR_EN_MASK
            } else {
                self.bits & !SpiMemEccCtrl::SPI_MEM_USR_ECC_ADDR_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_ECC_ADDR_EN** `[20:20]`
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub const fn with_spi_fmem_ecc_addr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemEccCtrl::SPI_FMEM_ECC_ADDR_EN_MASK
            } else {
                self.bits & !SpiMemEccCtrl::SPI_FMEM_ECC_ADDR_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_PAGE_SIZE** `[19:18]`
    ///
    /// Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub const fn with_spi_fmem_page_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemEccCtrl::SPI_FMEM_PAGE_SIZE_MASK)
                | ((value << SpiMemEccCtrl::SPI_FMEM_PAGE_SIZE_OFFSET)
                    & SpiMemEccCtrl::SPI_FMEM_PAGE_SIZE_MASK),
        }
    }

    /// Set **SPI_FMEM_ECC_ERR_INT_EN** `[17:17]`
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to flash.
    #[inline(always)]
    pub const fn with_spi_fmem_ecc_err_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_EN_MASK
            } else {
                self.bits & !SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_ECC_ERR_INT_NUM** `[16:11]`
    ///
    /// Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_fmem_ecc_err_int_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_NUM_MASK)
                | ((value << SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_NUM_OFFSET)
                    & SpiMemEccCtrl::SPI_FMEM_ECC_ERR_INT_NUM_MASK),
        }
    }
}

impl Default for SpiMemEccCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemEccCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemEccCtrlVal {
        SpiMemEccCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemEccCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemEccCtrlVal) -> SpiMemEccCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_ECC_ERR_ADDR Register
///
/// MSPI ECC error address register
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **SPI_MEM_ECC_ERR_ADDR** `[26:0]` (RO)
///
///   This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
#[repr(transparent)]
pub struct SpiMemEccErrAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemEccErrAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemEccErrAddr {
    // ======== Field Constants ========

    /// **SPI_MEM_ECC_ERR_ADDR**
    ///
    /// This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const SPI_MEM_ECC_ERR_ADDR_OFFSET: u32 = 0;
    pub const SPI_MEM_ECC_ERR_ADDR_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_ECC_ERR_ADDR** `[26:0]` (RO)
    ///
    /// This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_ecc_err_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_ECC_ERR_ADDR_MASK)
            >> Self::SPI_MEM_ECC_ERR_ADDR_OFFSET
    }
}

/// SPI_MEM_ECC_ERR_ADDR value type for batch read/modify/write operations
///
/// Use `spimemeccerraddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemeccerraddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemEccErrAddrVal {
    bits: u32,
}

impl SpiMemEccErrAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_ECC_ERR_ADDR** `[26:0]`
    ///
    /// This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_ecc_err_addr(&self) -> u32 {
        (self.bits & SpiMemEccErrAddr::SPI_MEM_ECC_ERR_ADDR_MASK)
            >> SpiMemEccErrAddr::SPI_MEM_ECC_ERR_ADDR_OFFSET
    }
}

impl Default for SpiMemEccErrAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemEccErrAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemEccErrAddrVal {
        SpiMemEccErrAddrVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_AXI_ERR_ADDR Register
///
/// SPI0 AXI request error address.
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **SPI_MEM_AXI_ERR_ADDR** `[26:0]` (RO)
///
///   This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set.
#[repr(transparent)]
pub struct SpiMemAxiErrAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemAxiErrAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemAxiErrAddr {
    // ======== Field Constants ========

    /// **SPI_MEM_AXI_ERR_ADDR**
    ///
    /// This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set.
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const SPI_MEM_AXI_ERR_ADDR_OFFSET: u32 = 0;
    pub const SPI_MEM_AXI_ERR_ADDR_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_AXI_ERR_ADDR** `[26:0]` (RO)
    ///
    /// This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set.
    #[inline(always)]
    pub fn spi_mem_axi_err_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_AXI_ERR_ADDR_MASK)
            >> Self::SPI_MEM_AXI_ERR_ADDR_OFFSET
    }
}

/// SPI_MEM_AXI_ERR_ADDR value type for batch read/modify/write operations
///
/// Use `spimemaxierraddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemaxierraddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemAxiErrAddrVal {
    bits: u32,
}

impl SpiMemAxiErrAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_AXI_ERR_ADDR** `[26:0]`
    ///
    /// This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set.
    #[inline(always)]
    pub const fn spi_mem_axi_err_addr(&self) -> u32 {
        (self.bits & SpiMemAxiErrAddr::SPI_MEM_AXI_ERR_ADDR_MASK)
            >> SpiMemAxiErrAddr::SPI_MEM_AXI_ERR_ADDR_OFFSET
    }
}

impl Default for SpiMemAxiErrAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemAxiErrAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemAxiErrAddrVal {
        SpiMemAxiErrAddrVal::from_bits(self.value.read_bits())
    }
}

/// SPI_SMEM_ECC_CTRL Register
///
/// MSPI ECC control register
///
/// Address offset: 0x0174
/// Reset value: 0x00080000
///
/// ## Fields
///
/// - **SPI_SMEM_ECC_ADDR_EN** `[20]` (RW)
///
///   Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1.
///
/// - **SPI_SMEM_PAGE_SIZE** `[19:18]` (RW)
///
///   Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
///
/// - **SPI_SMEM_ECC_ERR_INT_EN** `[17]` (RW)
///
///   Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM.
#[repr(transparent)]
pub struct SpiSmemEccCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemEccCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemEccCtrl {
    // ======== Field Constants ========

    /// **SPI_SMEM_ECC_ADDR_EN**
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_SMEM_ECC_ADDR_EN_OFFSET: u32 = 20;
    pub const SPI_SMEM_ECC_ADDR_EN_MASK: u32 = 0x00100000;
    pub const SPI_SMEM_ECC_ADDR_EN: u32 = 1 << 20;

    /// **SPI_SMEM_PAGE_SIZE**
    ///
    /// Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const SPI_SMEM_PAGE_SIZE_OFFSET: u32 = 18;
    pub const SPI_SMEM_PAGE_SIZE_MASK: u32 = 0x000c0000;

    /// **SPI_SMEM_ECC_ERR_INT_EN**
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_SMEM_ECC_ERR_INT_EN_OFFSET: u32 = 17;
    pub const SPI_SMEM_ECC_ERR_INT_EN_MASK: u32 = 0x00020000;
    pub const SPI_SMEM_ECC_ERR_INT_EN: u32 = 1 << 17;

    // ======== Field Accessors ========

    /// **SPI_SMEM_ECC_ADDR_EN** `[20]` (RW)
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub fn spi_smem_ecc_addr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_ECC_ADDR_EN_MASK) != 0
    }

    /// Set SPI_SMEM_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_ECC_ADDR_EN_MASK);
    }

    /// Clear SPI_SMEM_ECC_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_ecc_addr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_ECC_ADDR_EN_MASK);
    }

    /// **SPI_SMEM_PAGE_SIZE** `[19:18]` (RW)
    ///
    /// Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub fn spi_smem_page_size(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_PAGE_SIZE_MASK) >> Self::SPI_SMEM_PAGE_SIZE_OFFSET
    }

    /// Write **SPI_SMEM_PAGE_SIZE** `[19:18]` (RW)
    ///
    /// Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub fn write_spi_smem_page_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_PAGE_SIZE_MASK)
                | ((value << Self::SPI_SMEM_PAGE_SIZE_OFFSET) & Self::SPI_SMEM_PAGE_SIZE_MASK)
        });
    }

    /// **SPI_SMEM_ECC_ERR_INT_EN** `[17]` (RW)
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM.
    #[inline(always)]
    pub fn spi_smem_ecc_err_int_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_ECC_ERR_INT_EN_MASK) != 0
    }

    /// Set SPI_SMEM_ECC_ERR_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_ecc_err_int_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_ECC_ERR_INT_EN_MASK);
    }

    /// Clear SPI_SMEM_ECC_ERR_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_ecc_err_int_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_ECC_ERR_INT_EN_MASK);
    }
}

/// SPI_SMEM_ECC_CTRL value type for batch read/modify/write operations
///
/// Use `spismemeccctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemeccctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemEccCtrlVal {
    bits: u32,
}

impl SpiSmemEccCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00080000 }
    }

    /// Read **SPI_SMEM_ECC_ADDR_EN** `[20:20]`
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub const fn spi_smem_ecc_addr_en(&self) -> bool {
        (self.bits & SpiSmemEccCtrl::SPI_SMEM_ECC_ADDR_EN_MASK) != 0
    }

    /// Read **SPI_SMEM_PAGE_SIZE** `[19:18]`
    ///
    /// Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub const fn spi_smem_page_size(&self) -> u32 {
        (self.bits & SpiSmemEccCtrl::SPI_SMEM_PAGE_SIZE_MASK)
            >> SpiSmemEccCtrl::SPI_SMEM_PAGE_SIZE_OFFSET
    }

    /// Read **SPI_SMEM_ECC_ERR_INT_EN** `[17:17]`
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM.
    #[inline(always)]
    pub const fn spi_smem_ecc_err_int_en(&self) -> bool {
        (self.bits & SpiSmemEccCtrl::SPI_SMEM_ECC_ERR_INT_EN_MASK) != 0
    }

    /// Set **SPI_SMEM_ECC_ADDR_EN** `[20:20]`
    ///
    /// Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1.
    #[inline(always)]
    pub const fn with_spi_smem_ecc_addr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemEccCtrl::SPI_SMEM_ECC_ADDR_EN_MASK
            } else {
                self.bits & !SpiSmemEccCtrl::SPI_SMEM_ECC_ADDR_EN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_PAGE_SIZE** `[19:18]`
    ///
    /// Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes.
    #[inline(always)]
    pub const fn with_spi_smem_page_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemEccCtrl::SPI_SMEM_PAGE_SIZE_MASK)
                | ((value << SpiSmemEccCtrl::SPI_SMEM_PAGE_SIZE_OFFSET)
                    & SpiSmemEccCtrl::SPI_SMEM_PAGE_SIZE_MASK),
        }
    }

    /// Set **SPI_SMEM_ECC_ERR_INT_EN** `[17:17]`
    ///
    /// Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_ecc_err_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemEccCtrl::SPI_SMEM_ECC_ERR_INT_EN_MASK
            } else {
                self.bits & !SpiSmemEccCtrl::SPI_SMEM_ECC_ERR_INT_EN_MASK
            },
        }
    }
}

impl Default for SpiSmemEccCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemEccCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemEccCtrlVal {
        SpiSmemEccCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemEccCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemEccCtrlVal) -> SpiSmemEccCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_AXI_ADDR_CTRL Register
///
/// SPI0 AXI address control register
///
/// Address offset: 0x0178
/// Reset value: 0xfc000000
///
/// ## Fields
///
/// - **SPI_ALL_AXI_TRANS_AFIFO_EMPTY** `[31]` (RO)
///
///   This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE.
///
/// - **SPI_WBLEN_AFIFO_REMPTY** `[30]` (RO)
///
///   1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending.
///
/// - **SPI_WDATA_AFIFO_REMPTY** `[29]` (RO)
///
///   1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending.
///
/// - **SPI_RADDR_AFIFO_REMPTY** `[28]` (RO)
///
///   1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending.
///
/// - **SPI_RDATA_AFIFO_REMPTY** `[27]` (RO)
///
///   1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending.
///
/// - **SPI_MEM_ALL_FIFO_EMPTY** `[26]` (RO)
///
///   The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others.
#[repr(transparent)]
pub struct SpiSmemAxiAddrCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemAxiAddrCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemAxiAddrCtrl {
    // ======== Field Constants ========

    /// **SPI_ALL_AXI_TRANS_AFIFO_EMPTY**
    ///
    /// This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_ALL_AXI_TRANS_AFIFO_EMPTY_OFFSET: u32 = 31;
    pub const SPI_ALL_AXI_TRANS_AFIFO_EMPTY_MASK: u32 = 0x80000000;
    pub const SPI_ALL_AXI_TRANS_AFIFO_EMPTY: u32 = 1 << 31;

    /// **SPI_WBLEN_AFIFO_REMPTY**
    ///
    /// 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_WBLEN_AFIFO_REMPTY_OFFSET: u32 = 30;
    pub const SPI_WBLEN_AFIFO_REMPTY_MASK: u32 = 0x40000000;
    pub const SPI_WBLEN_AFIFO_REMPTY: u32 = 1 << 30;

    /// **SPI_WDATA_AFIFO_REMPTY**
    ///
    /// 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_WDATA_AFIFO_REMPTY_OFFSET: u32 = 29;
    pub const SPI_WDATA_AFIFO_REMPTY_MASK: u32 = 0x20000000;
    pub const SPI_WDATA_AFIFO_REMPTY: u32 = 1 << 29;

    /// **SPI_RADDR_AFIFO_REMPTY**
    ///
    /// 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_RADDR_AFIFO_REMPTY_OFFSET: u32 = 28;
    pub const SPI_RADDR_AFIFO_REMPTY_MASK: u32 = 0x10000000;
    pub const SPI_RADDR_AFIFO_REMPTY: u32 = 1 << 28;

    /// **SPI_RDATA_AFIFO_REMPTY**
    ///
    /// 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_RDATA_AFIFO_REMPTY_OFFSET: u32 = 27;
    pub const SPI_RDATA_AFIFO_REMPTY_MASK: u32 = 0x08000000;
    pub const SPI_RDATA_AFIFO_REMPTY: u32 = 1 << 27;

    /// **SPI_MEM_ALL_FIFO_EMPTY**
    ///
    /// The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_ALL_FIFO_EMPTY_OFFSET: u32 = 26;
    pub const SPI_MEM_ALL_FIFO_EMPTY_MASK: u32 = 0x04000000;
    pub const SPI_MEM_ALL_FIFO_EMPTY: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// **SPI_ALL_AXI_TRANS_AFIFO_EMPTY** `[31]` (RO)
    ///
    /// This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE.
    #[inline(always)]
    pub fn spi_all_axi_trans_afifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_ALL_AXI_TRANS_AFIFO_EMPTY_MASK) != 0
    }

    /// **SPI_WBLEN_AFIFO_REMPTY** `[30]` (RO)
    ///
    /// 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending.
    #[inline(always)]
    pub fn spi_wblen_afifo_rempty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_WBLEN_AFIFO_REMPTY_MASK) != 0
    }

    /// **SPI_WDATA_AFIFO_REMPTY** `[29]` (RO)
    ///
    /// 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending.
    #[inline(always)]
    pub fn spi_wdata_afifo_rempty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_WDATA_AFIFO_REMPTY_MASK) != 0
    }

    /// **SPI_RADDR_AFIFO_REMPTY** `[28]` (RO)
    ///
    /// 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending.
    #[inline(always)]
    pub fn spi_raddr_afifo_rempty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_RADDR_AFIFO_REMPTY_MASK) != 0
    }

    /// **SPI_RDATA_AFIFO_REMPTY** `[27]` (RO)
    ///
    /// 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending.
    #[inline(always)]
    pub fn spi_rdata_afifo_rempty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_RDATA_AFIFO_REMPTY_MASK) != 0
    }

    /// **SPI_MEM_ALL_FIFO_EMPTY** `[26]` (RO)
    ///
    /// The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others.
    #[inline(always)]
    pub fn spi_mem_all_fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_ALL_FIFO_EMPTY_MASK) != 0
    }
}

/// SPI_SMEM_AXI_ADDR_CTRL value type for batch read/modify/write operations
///
/// Use `spismemaxiaddrctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemaxiaddrctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemAxiAddrCtrlVal {
    bits: u32,
}

impl SpiSmemAxiAddrCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0xfc000000 }
    }

    /// Read **SPI_ALL_AXI_TRANS_AFIFO_EMPTY** `[31:31]`
    ///
    /// This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE.
    #[inline(always)]
    pub const fn spi_all_axi_trans_afifo_empty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_ALL_AXI_TRANS_AFIFO_EMPTY_MASK) != 0
    }

    /// Read **SPI_WBLEN_AFIFO_REMPTY** `[30:30]`
    ///
    /// 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending.
    #[inline(always)]
    pub const fn spi_wblen_afifo_rempty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_WBLEN_AFIFO_REMPTY_MASK) != 0
    }

    /// Read **SPI_WDATA_AFIFO_REMPTY** `[29:29]`
    ///
    /// 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending.
    #[inline(always)]
    pub const fn spi_wdata_afifo_rempty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_WDATA_AFIFO_REMPTY_MASK) != 0
    }

    /// Read **SPI_RADDR_AFIFO_REMPTY** `[28:28]`
    ///
    /// 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending.
    #[inline(always)]
    pub const fn spi_raddr_afifo_rempty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_RADDR_AFIFO_REMPTY_MASK) != 0
    }

    /// Read **SPI_RDATA_AFIFO_REMPTY** `[27:27]`
    ///
    /// 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending.
    #[inline(always)]
    pub const fn spi_rdata_afifo_rempty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_RDATA_AFIFO_REMPTY_MASK) != 0
    }

    /// Read **SPI_MEM_ALL_FIFO_EMPTY** `[26:26]`
    ///
    /// The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_all_fifo_empty(&self) -> bool {
        (self.bits & SpiSmemAxiAddrCtrl::SPI_MEM_ALL_FIFO_EMPTY_MASK) != 0
    }
}

impl Default for SpiSmemAxiAddrCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemAxiAddrCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemAxiAddrCtrlVal {
        SpiSmemAxiAddrCtrlVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_AXI_ERR_RESP_EN Register
///
/// SPI0 AXI error response enable register
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **SPI_MEM_AR_RESP_EN_AXI_SIZE** `[11]` (RW)
///
///   Set this bit to enable AXI response function for axi size err in axi read trans.
///
/// - **SPI_MEM_AR_RESP_EN_MMU_SENS** `[10]` (RW)
///
///   Set this bit to enable AXI response function for mmu sensitive err in axi read trans.
///
/// - **SPI_MEM_AR_RESP_EN_MMU_ECC** `[9]` (RW)
///
///   Set this bit to enable AXI response function for mmu ecc err in axi read trans.
///
/// - **SPI_MEM_AR_RESP_EN_MMU_GID** `[8]` (RW)
///
///   Set this bit to enable AXI response function for mmu gid err in axi read trans.
///
/// - **SPI_MEM_AR_RESP_EN_MMU_VLD** `[7]` (RW)
///
///   Set this bit to enable AXI response function for mmu valid err in axi read trans.
///
/// - **SPI_MEM_AW_RESP_EN_AXI_WSTRB** `[6]` (RW)
///
///   Set this bit to enable AXI response function for axi wstrb err in axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_MMU_SENS** `[5]` (RW)
///
///   Set this bit to enable AXI response function for mmu sens in err axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_MMU_ECC** `[4]` (RW)
///
///   Set this bit to enable AXI response function for mmu ecc err in axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_AXI_FLASH** `[3]` (RW)
///
///   Set this bit to enable AXI response function for axi flash err in axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_AXI_SIZE** `[2]` (RW)
///
///   Set this bit to enable AXI response function for axi size err in axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_MMU_GID** `[1]` (RW)
///
///   Set this bit to enable AXI response function for mmu gid err in axi write trans.
///
/// - **SPI_MEM_AW_RESP_EN_MMU_VLD** `[0]` (RW)
///
///   Set this bit to enable AXI response function for mmu valid err in axi write trans.
#[repr(transparent)]
pub struct SpiMemAxiErrRespEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemAxiErrRespEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemAxiErrRespEn {
    // ======== Field Constants ========

    /// **SPI_MEM_AR_RESP_EN_AXI_SIZE**
    ///
    /// Set this bit to enable AXI response function for axi size err in axi read trans.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_MEM_AR_RESP_EN_AXI_SIZE_OFFSET: u32 = 11;
    pub const SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK: u32 = 0x00000800;
    pub const SPI_MEM_AR_RESP_EN_AXI_SIZE: u32 = 1 << 11;

    /// **SPI_MEM_AR_RESP_EN_MMU_SENS**
    ///
    /// Set this bit to enable AXI response function for mmu sensitive err in axi read trans.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_AR_RESP_EN_MMU_SENS_OFFSET: u32 = 10;
    pub const SPI_MEM_AR_RESP_EN_MMU_SENS_MASK: u32 = 0x00000400;
    pub const SPI_MEM_AR_RESP_EN_MMU_SENS: u32 = 1 << 10;

    /// **SPI_MEM_AR_RESP_EN_MMU_ECC**
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi read trans.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_AR_RESP_EN_MMU_ECC_OFFSET: u32 = 9;
    pub const SPI_MEM_AR_RESP_EN_MMU_ECC_MASK: u32 = 0x00000200;
    pub const SPI_MEM_AR_RESP_EN_MMU_ECC: u32 = 1 << 9;

    /// **SPI_MEM_AR_RESP_EN_MMU_GID**
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi read trans.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_AR_RESP_EN_MMU_GID_OFFSET: u32 = 8;
    pub const SPI_MEM_AR_RESP_EN_MMU_GID_MASK: u32 = 0x00000100;
    pub const SPI_MEM_AR_RESP_EN_MMU_GID: u32 = 1 << 8;

    /// **SPI_MEM_AR_RESP_EN_MMU_VLD**
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi read trans.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_AR_RESP_EN_MMU_VLD_OFFSET: u32 = 7;
    pub const SPI_MEM_AR_RESP_EN_MMU_VLD_MASK: u32 = 0x00000080;
    pub const SPI_MEM_AR_RESP_EN_MMU_VLD: u32 = 1 << 7;

    /// **SPI_MEM_AW_RESP_EN_AXI_WSTRB**
    ///
    /// Set this bit to enable AXI response function for axi wstrb err in axi write trans.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_AW_RESP_EN_AXI_WSTRB_OFFSET: u32 = 6;
    pub const SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK: u32 = 0x00000040;
    pub const SPI_MEM_AW_RESP_EN_AXI_WSTRB: u32 = 1 << 6;

    /// **SPI_MEM_AW_RESP_EN_MMU_SENS**
    ///
    /// Set this bit to enable AXI response function for mmu sens in err axi write trans.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_AW_RESP_EN_MMU_SENS_OFFSET: u32 = 5;
    pub const SPI_MEM_AW_RESP_EN_MMU_SENS_MASK: u32 = 0x00000020;
    pub const SPI_MEM_AW_RESP_EN_MMU_SENS: u32 = 1 << 5;

    /// **SPI_MEM_AW_RESP_EN_MMU_ECC**
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi write trans.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_AW_RESP_EN_MMU_ECC_OFFSET: u32 = 4;
    pub const SPI_MEM_AW_RESP_EN_MMU_ECC_MASK: u32 = 0x00000010;
    pub const SPI_MEM_AW_RESP_EN_MMU_ECC: u32 = 1 << 4;

    /// **SPI_MEM_AW_RESP_EN_AXI_FLASH**
    ///
    /// Set this bit to enable AXI response function for axi flash err in axi write trans.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_AW_RESP_EN_AXI_FLASH_OFFSET: u32 = 3;
    pub const SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK: u32 = 0x00000008;
    pub const SPI_MEM_AW_RESP_EN_AXI_FLASH: u32 = 1 << 3;

    /// **SPI_MEM_AW_RESP_EN_AXI_SIZE**
    ///
    /// Set this bit to enable AXI response function for axi size err in axi write trans.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_AW_RESP_EN_AXI_SIZE_OFFSET: u32 = 2;
    pub const SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK: u32 = 0x00000004;
    pub const SPI_MEM_AW_RESP_EN_AXI_SIZE: u32 = 1 << 2;

    /// **SPI_MEM_AW_RESP_EN_MMU_GID**
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi write trans.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_AW_RESP_EN_MMU_GID_OFFSET: u32 = 1;
    pub const SPI_MEM_AW_RESP_EN_MMU_GID_MASK: u32 = 0x00000002;
    pub const SPI_MEM_AW_RESP_EN_MMU_GID: u32 = 1 << 1;

    /// **SPI_MEM_AW_RESP_EN_MMU_VLD**
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi write trans.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_AW_RESP_EN_MMU_VLD_OFFSET: u32 = 0;
    pub const SPI_MEM_AW_RESP_EN_MMU_VLD_MASK: u32 = 0x00000001;
    pub const SPI_MEM_AW_RESP_EN_MMU_VLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_AR_RESP_EN_AXI_SIZE** `[11]` (RW)
    ///
    /// Set this bit to enable AXI response function for axi size err in axi read trans.
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_axi_size(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK) != 0
    }

    /// Set SPI_MEM_AR_RESP_EN_AXI_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_resp_en_axi_size(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK);
    }

    /// Clear SPI_MEM_AR_RESP_EN_AXI_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_resp_en_axi_size(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK);
    }

    /// **SPI_MEM_AR_RESP_EN_MMU_SENS** `[10]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu sensitive err in axi read trans.
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_sens(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK) != 0
    }

    /// Set SPI_MEM_AR_RESP_EN_MMU_SENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_resp_en_mmu_sens(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK);
    }

    /// Clear SPI_MEM_AR_RESP_EN_MMU_SENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_resp_en_mmu_sens(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK);
    }

    /// **SPI_MEM_AR_RESP_EN_MMU_ECC** `[9]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi read trans.
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK) != 0
    }

    /// Set SPI_MEM_AR_RESP_EN_MMU_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_resp_en_mmu_ecc(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK);
    }

    /// Clear SPI_MEM_AR_RESP_EN_MMU_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_resp_en_mmu_ecc(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK);
    }

    /// **SPI_MEM_AR_RESP_EN_MMU_GID** `[8]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi read trans.
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_gid(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_RESP_EN_MMU_GID_MASK) != 0
    }

    /// Set SPI_MEM_AR_RESP_EN_MMU_GID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_resp_en_mmu_gid(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_RESP_EN_MMU_GID_MASK);
    }

    /// Clear SPI_MEM_AR_RESP_EN_MMU_GID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_resp_en_mmu_gid(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_RESP_EN_MMU_GID_MASK);
    }

    /// **SPI_MEM_AR_RESP_EN_MMU_VLD** `[7]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi read trans.
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_vld(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK) != 0
    }

    /// Set SPI_MEM_AR_RESP_EN_MMU_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_ar_resp_en_mmu_vld(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK);
    }

    /// Clear SPI_MEM_AR_RESP_EN_MMU_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_ar_resp_en_mmu_vld(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_AXI_WSTRB** `[6]` (RW)
    ///
    /// Set this bit to enable AXI response function for axi wstrb err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_wstrb(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_AXI_WSTRB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_axi_wstrb(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_AXI_WSTRB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_axi_wstrb(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_MMU_SENS** `[5]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu sens in err axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_sens(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_MMU_SENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_mmu_sens(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_MMU_SENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_mmu_sens(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_MMU_ECC** `[4]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_ecc(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_MMU_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_mmu_ecc(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_MMU_ECC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_mmu_ecc(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_AXI_FLASH** `[3]` (RW)
    ///
    /// Set this bit to enable AXI response function for axi flash err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_flash(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_AXI_FLASH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_axi_flash(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_AXI_FLASH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_axi_flash(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_AXI_SIZE** `[2]` (RW)
    ///
    /// Set this bit to enable AXI response function for axi size err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_size(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_AXI_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_axi_size(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_AXI_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_axi_size(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_MMU_GID** `[1]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_gid(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_MMU_GID_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_MMU_GID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_mmu_gid(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_MMU_GID_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_MMU_GID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_mmu_gid(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_MMU_GID_MASK);
    }

    /// **SPI_MEM_AW_RESP_EN_MMU_VLD** `[0]` (RW)
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi write trans.
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_vld(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK) != 0
    }

    /// Set SPI_MEM_AW_RESP_EN_MMU_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_aw_resp_en_mmu_vld(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK);
    }

    /// Clear SPI_MEM_AW_RESP_EN_MMU_VLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_aw_resp_en_mmu_vld(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK);
    }
}

/// SPI_MEM_AXI_ERR_RESP_EN value type for batch read/modify/write operations
///
/// Use `spimemaxierrrespen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemaxierrrespen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemAxiErrRespEnVal {
    bits: u32,
}

impl SpiMemAxiErrRespEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_AR_RESP_EN_AXI_SIZE** `[11:11]`
    ///
    /// Set this bit to enable AXI response function for axi size err in axi read trans.
    #[inline(always)]
    pub const fn spi_mem_ar_resp_en_axi_size(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK) != 0
    }

    /// Read **SPI_MEM_AR_RESP_EN_MMU_SENS** `[10:10]`
    ///
    /// Set this bit to enable AXI response function for mmu sensitive err in axi read trans.
    #[inline(always)]
    pub const fn spi_mem_ar_resp_en_mmu_sens(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK) != 0
    }

    /// Read **SPI_MEM_AR_RESP_EN_MMU_ECC** `[9:9]`
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi read trans.
    #[inline(always)]
    pub const fn spi_mem_ar_resp_en_mmu_ecc(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK) != 0
    }

    /// Read **SPI_MEM_AR_RESP_EN_MMU_GID** `[8:8]`
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi read trans.
    #[inline(always)]
    pub const fn spi_mem_ar_resp_en_mmu_gid(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_GID_MASK) != 0
    }

    /// Read **SPI_MEM_AR_RESP_EN_MMU_VLD** `[7:7]`
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi read trans.
    #[inline(always)]
    pub const fn spi_mem_ar_resp_en_mmu_vld(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_AXI_WSTRB** `[6:6]`
    ///
    /// Set this bit to enable AXI response function for axi wstrb err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_axi_wstrb(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_MMU_SENS** `[5:5]`
    ///
    /// Set this bit to enable AXI response function for mmu sens in err axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_mmu_sens(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_MMU_ECC** `[4:4]`
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_mmu_ecc(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_AXI_FLASH** `[3:3]`
    ///
    /// Set this bit to enable AXI response function for axi flash err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_axi_flash(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_AXI_SIZE** `[2:2]`
    ///
    /// Set this bit to enable AXI response function for axi size err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_axi_size(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_MMU_GID** `[1:1]`
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_mmu_gid(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_GID_MASK) != 0
    }

    /// Read **SPI_MEM_AW_RESP_EN_MMU_VLD** `[0:0]`
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi write trans.
    #[inline(always)]
    pub const fn spi_mem_aw_resp_en_mmu_vld(&self) -> bool {
        (self.bits & SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK) != 0
    }

    /// Set **SPI_MEM_AR_RESP_EN_AXI_SIZE** `[11:11]`
    ///
    /// Set this bit to enable AXI response function for axi size err in axi read trans.
    #[inline(always)]
    pub const fn with_spi_mem_ar_resp_en_axi_size(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_AXI_SIZE_MASK
            },
        }
    }

    /// Set **SPI_MEM_AR_RESP_EN_MMU_SENS** `[10:10]`
    ///
    /// Set this bit to enable AXI response function for mmu sensitive err in axi read trans.
    #[inline(always)]
    pub const fn with_spi_mem_ar_resp_en_mmu_sens(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_SENS_MASK
            },
        }
    }

    /// Set **SPI_MEM_AR_RESP_EN_MMU_ECC** `[9:9]`
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi read trans.
    #[inline(always)]
    pub const fn with_spi_mem_ar_resp_en_mmu_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_ECC_MASK
            },
        }
    }

    /// Set **SPI_MEM_AR_RESP_EN_MMU_GID** `[8:8]`
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi read trans.
    #[inline(always)]
    pub const fn with_spi_mem_ar_resp_en_mmu_gid(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_GID_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_GID_MASK
            },
        }
    }

    /// Set **SPI_MEM_AR_RESP_EN_MMU_VLD** `[7:7]`
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi read trans.
    #[inline(always)]
    pub const fn with_spi_mem_ar_resp_en_mmu_vld(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AR_RESP_EN_MMU_VLD_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_AXI_WSTRB** `[6:6]`
    ///
    /// Set this bit to enable AXI response function for axi wstrb err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_axi_wstrb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_WSTRB_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_MMU_SENS** `[5:5]`
    ///
    /// Set this bit to enable AXI response function for mmu sens in err axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_mmu_sens(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_SENS_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_MMU_ECC** `[4:4]`
    ///
    /// Set this bit to enable AXI response function for mmu ecc err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_mmu_ecc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_ECC_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_AXI_FLASH** `[3:3]`
    ///
    /// Set this bit to enable AXI response function for axi flash err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_axi_flash(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_FLASH_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_AXI_SIZE** `[2:2]`
    ///
    /// Set this bit to enable AXI response function for axi size err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_axi_size(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_AXI_SIZE_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_MMU_GID** `[1:1]`
    ///
    /// Set this bit to enable AXI response function for mmu gid err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_mmu_gid(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_GID_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_GID_MASK
            },
        }
    }

    /// Set **SPI_MEM_AW_RESP_EN_MMU_VLD** `[0:0]`
    ///
    /// Set this bit to enable AXI response function for mmu valid err in axi write trans.
    #[inline(always)]
    pub const fn with_spi_mem_aw_resp_en_mmu_vld(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK
            } else {
                self.bits & !SpiMemAxiErrRespEn::SPI_MEM_AW_RESP_EN_MMU_VLD_MASK
            },
        }
    }
}

impl Default for SpiMemAxiErrRespEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemAxiErrRespEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemAxiErrRespEnVal {
        SpiMemAxiErrRespEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemAxiErrRespEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemAxiErrRespEnVal) -> SpiMemAxiErrRespEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_TIMING_CALI Register
///
/// SPI0 flash timing calibration register
///
/// Address offset: 0x0180
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **UPDATE** `[6]` (WO)
///
///   Set this bit to update delay mode, delay num and extra dummy in MSPI.
///
/// - **SPI_MEM_DLL_TIMING_CALI** `[5]` (RW)
///
///   Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash.
///
/// - **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
///
///   add extra dummy spi clock cycle length for spi clock calibration.
///
/// - **SPI_MEM_TIMING_CALI** `[1]` (RW)
///
///   The bit is used to enable timing auto-calibration for all reading operations.
///
/// - **SPI_MEM_TIMING_CLK_ENA** `[0]` (RW)
///
///   The bit is used to enable timing adjust clock for all reading operations.
#[repr(transparent)]
pub struct SpiMemTimingCali {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemTimingCali {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemTimingCali {
    // ======== Field Constants ========

    /// **UPDATE**
    ///
    /// Set this bit to update delay mode, delay num and extra dummy in MSPI.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const UPDATE_OFFSET: u32 = 6;
    pub const UPDATE_MASK: u32 = 0x00000040;
    pub const UPDATE: u32 = 1 << 6;

    /// **SPI_MEM_DLL_TIMING_CALI**
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_DLL_TIMING_CALI_OFFSET: u32 = 5;
    pub const SPI_MEM_DLL_TIMING_CALI_MASK: u32 = 0x00000020;
    pub const SPI_MEM_DLL_TIMING_CALI: u32 = 1 << 5;

    /// **SPI_MEM_EXTRA_DUMMY_CYCLELEN**
    ///
    /// add extra dummy spi clock cycle length for spi clock calibration.
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const SPI_MEM_EXTRA_DUMMY_CYCLELEN_OFFSET: u32 = 2;
    pub const SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK: u32 = 0x0000001c;

    /// **SPI_MEM_TIMING_CALI**
    ///
    /// The bit is used to enable timing auto-calibration for all reading operations.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_TIMING_CALI_OFFSET: u32 = 1;
    pub const SPI_MEM_TIMING_CALI_MASK: u32 = 0x00000002;
    pub const SPI_MEM_TIMING_CALI: u32 = 1 << 1;

    /// **SPI_MEM_TIMING_CLK_ENA**
    ///
    /// The bit is used to enable timing adjust clock for all reading operations.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_TIMING_CLK_ENA_OFFSET: u32 = 0;
    pub const SPI_MEM_TIMING_CLK_ENA_MASK: u32 = 0x00000001;
    pub const SPI_MEM_TIMING_CLK_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **UPDATE** (WO)
    ///
    /// Set this bit to update delay mode, delay num and extra dummy in MSPI.
    #[inline(always)]
    pub fn trigger_update(&self) {
        self.value.modify_bits(|r| r | Self::UPDATE_MASK);
    }

    /// **SPI_MEM_DLL_TIMING_CALI** `[5]` (RW)
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash.
    #[inline(always)]
    pub fn spi_mem_dll_timing_cali(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DLL_TIMING_CALI_MASK) != 0
    }

    /// Set SPI_MEM_DLL_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dll_timing_cali(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DLL_TIMING_CALI_MASK);
    }

    /// Clear SPI_MEM_DLL_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dll_timing_cali(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DLL_TIMING_CALI_MASK);
    }

    /// **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
    ///
    /// add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK)
            >> Self::SPI_MEM_EXTRA_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
    ///
    /// add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub fn write_spi_mem_extra_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_EXTRA_DUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK)
        });
    }

    /// **SPI_MEM_TIMING_CALI** `[1]` (RW)
    ///
    /// The bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub fn spi_mem_timing_cali(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_TIMING_CALI_MASK) != 0
    }

    /// Set SPI_MEM_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_timing_cali(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_TIMING_CALI_MASK);
    }

    /// Clear SPI_MEM_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_timing_cali(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_TIMING_CALI_MASK);
    }

    /// **SPI_MEM_TIMING_CLK_ENA** `[0]` (RW)
    ///
    /// The bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_TIMING_CLK_ENA_MASK) != 0
    }

    /// Set SPI_MEM_TIMING_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_timing_clk_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_TIMING_CLK_ENA_MASK);
    }

    /// Clear SPI_MEM_TIMING_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_timing_clk_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_TIMING_CLK_ENA_MASK);
    }
}

/// SPI_MEM_TIMING_CALI value type for batch read/modify/write operations
///
/// Use `spimemtimingcali.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemtimingcali.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemTimingCaliVal {
    bits: u32,
}

impl SpiMemTimingCaliVal {
    /// Create from raw bits
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

    /// Read **SPI_MEM_DLL_TIMING_CALI** `[5:5]`
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash.
    #[inline(always)]
    pub const fn spi_mem_dll_timing_cali(&self) -> bool {
        (self.bits & SpiMemTimingCali::SPI_MEM_DLL_TIMING_CALI_MASK) != 0
    }

    /// Read **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]`
    ///
    /// add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub const fn spi_mem_extra_dummy_cyclelen(&self) -> u32 {
        (self.bits & SpiMemTimingCali::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK)
            >> SpiMemTimingCali::SPI_MEM_EXTRA_DUMMY_CYCLELEN_OFFSET
    }

    /// Read **SPI_MEM_TIMING_CALI** `[1:1]`
    ///
    /// The bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub const fn spi_mem_timing_cali(&self) -> bool {
        (self.bits & SpiMemTimingCali::SPI_MEM_TIMING_CALI_MASK) != 0
    }

    /// Read **SPI_MEM_TIMING_CLK_ENA** `[0:0]`
    ///
    /// The bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub const fn spi_mem_timing_clk_ena(&self) -> bool {
        (self.bits & SpiMemTimingCali::SPI_MEM_TIMING_CLK_ENA_MASK) != 0
    }

    /// Set **UPDATE** `[6:6]`
    ///
    /// Set this bit to update delay mode, delay num and extra dummy in MSPI.
    #[inline(always)]
    pub const fn with_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemTimingCali::UPDATE_MASK
            } else {
                self.bits & !SpiMemTimingCali::UPDATE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DLL_TIMING_CALI** `[5:5]`
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash.
    #[inline(always)]
    pub const fn with_spi_mem_dll_timing_cali(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemTimingCali::SPI_MEM_DLL_TIMING_CALI_MASK
            } else {
                self.bits & !SpiMemTimingCali::SPI_MEM_DLL_TIMING_CALI_MASK
            },
        }
    }

    /// Set **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]`
    ///
    /// add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub const fn with_spi_mem_extra_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemTimingCali::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK)
                | ((value << SpiMemTimingCali::SPI_MEM_EXTRA_DUMMY_CYCLELEN_OFFSET)
                    & SpiMemTimingCali::SPI_MEM_EXTRA_DUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_MEM_TIMING_CALI** `[1:1]`
    ///
    /// The bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub const fn with_spi_mem_timing_cali(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemTimingCali::SPI_MEM_TIMING_CALI_MASK
            } else {
                self.bits & !SpiMemTimingCali::SPI_MEM_TIMING_CALI_MASK
            },
        }
    }

    /// Set **SPI_MEM_TIMING_CLK_ENA** `[0:0]`
    ///
    /// The bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub const fn with_spi_mem_timing_clk_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemTimingCali::SPI_MEM_TIMING_CLK_ENA_MASK
            } else {
                self.bits & !SpiMemTimingCali::SPI_MEM_TIMING_CLK_ENA_MASK
            },
        }
    }
}

impl Default for SpiMemTimingCaliVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemTimingCali {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemTimingCaliVal {
        SpiMemTimingCaliVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemTimingCaliVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemTimingCaliVal) -> SpiMemTimingCaliVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_DIN_MODE Register
///
/// MSPI flash input timing delay mode control register
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **SPI_MEM_DINS_MODE** `[26:24]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
///
/// - **SPI_MEM_DIN7_MODE** `[23:21]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
///
/// - **SPI_MEM_DIN6_MODE** `[20:18]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
///
/// - **SPI_MEM_DIN5_MODE** `[17:15]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
///
/// - **SPI_MEM_DIN4_MODE** `[14:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
///
/// - **SPI_MEM_DIN3_MODE** `[11:9]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_MEM_DIN2_MODE** `[8:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_MEM_DIN1_MODE** `[5:3]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_MEM_DIN0_MODE** `[2:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
#[repr(transparent)]
pub struct SpiMemDinMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDinMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDinMode {
    // ======== Field Constants ========

    /// **SPI_MEM_DINS_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SPI_MEM_DINS_MODE_OFFSET: u32 = 24;
    pub const SPI_MEM_DINS_MODE_MASK: u32 = 0x07000000;

    /// **SPI_MEM_DIN7_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const SPI_MEM_DIN7_MODE_OFFSET: u32 = 21;
    pub const SPI_MEM_DIN7_MODE_MASK: u32 = 0x00e00000;

    /// **SPI_MEM_DIN6_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SPI_MEM_DIN6_MODE_OFFSET: u32 = 18;
    pub const SPI_MEM_DIN6_MODE_MASK: u32 = 0x001c0000;

    /// **SPI_MEM_DIN5_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const SPI_MEM_DIN5_MODE_OFFSET: u32 = 15;
    pub const SPI_MEM_DIN5_MODE_MASK: u32 = 0x00038000;

    /// **SPI_MEM_DIN4_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SPI_MEM_DIN4_MODE_OFFSET: u32 = 12;
    pub const SPI_MEM_DIN4_MODE_MASK: u32 = 0x00007000;

    /// **SPI_MEM_DIN3_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const SPI_MEM_DIN3_MODE_OFFSET: u32 = 9;
    pub const SPI_MEM_DIN3_MODE_MASK: u32 = 0x00000e00;

    /// **SPI_MEM_DIN2_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SPI_MEM_DIN2_MODE_OFFSET: u32 = 6;
    pub const SPI_MEM_DIN2_MODE_MASK: u32 = 0x000001c0;

    /// **SPI_MEM_DIN1_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const SPI_MEM_DIN1_MODE_OFFSET: u32 = 3;
    pub const SPI_MEM_DIN1_MODE_MASK: u32 = 0x00000038;

    /// **SPI_MEM_DIN0_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SPI_MEM_DIN0_MODE_OFFSET: u32 = 0;
    pub const SPI_MEM_DIN0_MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SPI_MEM_DINS_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn spi_mem_dins_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DINS_MODE_MASK) >> Self::SPI_MEM_DINS_MODE_OFFSET
    }

    /// Write **SPI_MEM_DINS_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn write_spi_mem_dins_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DINS_MODE_MASK)
                | ((value << Self::SPI_MEM_DINS_MODE_OFFSET) & Self::SPI_MEM_DINS_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN7_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn spi_mem_din7_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN7_MODE_MASK) >> Self::SPI_MEM_DIN7_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN7_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn write_spi_mem_din7_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN7_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN7_MODE_OFFSET) & Self::SPI_MEM_DIN7_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN6_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn spi_mem_din6_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN6_MODE_MASK) >> Self::SPI_MEM_DIN6_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN6_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn write_spi_mem_din6_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN6_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN6_MODE_OFFSET) & Self::SPI_MEM_DIN6_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN5_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn spi_mem_din5_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN5_MODE_MASK) >> Self::SPI_MEM_DIN5_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN5_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn write_spi_mem_din5_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN5_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN5_MODE_OFFSET) & Self::SPI_MEM_DIN5_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN4_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn spi_mem_din4_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN4_MODE_MASK) >> Self::SPI_MEM_DIN4_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN4_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub fn write_spi_mem_din4_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN4_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN4_MODE_OFFSET) & Self::SPI_MEM_DIN4_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN3_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_din3_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN3_MODE_MASK) >> Self::SPI_MEM_DIN3_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN3_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_mem_din3_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN3_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN3_MODE_OFFSET) & Self::SPI_MEM_DIN3_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN2_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_din2_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN2_MODE_MASK) >> Self::SPI_MEM_DIN2_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN2_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_mem_din2_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN2_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN2_MODE_OFFSET) & Self::SPI_MEM_DIN2_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN1_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_din1_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN1_MODE_MASK) >> Self::SPI_MEM_DIN1_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN1_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_mem_din1_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN1_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN1_MODE_OFFSET) & Self::SPI_MEM_DIN1_MODE_MASK)
        });
    }

    /// **SPI_MEM_DIN0_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_din0_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN0_MODE_MASK) >> Self::SPI_MEM_DIN0_MODE_OFFSET
    }

    /// Write **SPI_MEM_DIN0_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_mem_din0_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN0_MODE_MASK)
                | ((value << Self::SPI_MEM_DIN0_MODE_OFFSET) & Self::SPI_MEM_DIN0_MODE_MASK)
        });
    }
}

/// SPI_MEM_DIN_MODE value type for batch read/modify/write operations
///
/// Use `spimemdinmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemdinmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDinModeVal {
    bits: u32,
}

impl SpiMemDinModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_DINS_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_dins_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DINS_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DINS_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN7_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_din7_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN7_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN7_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN6_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_din6_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN6_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN6_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN5_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_din5_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN5_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN5_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN4_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_din4_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN4_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN4_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN3_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_din3_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN3_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN3_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN2_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_din2_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN2_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN2_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN1_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_din1_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN1_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN1_MODE_OFFSET
    }

    /// Read **SPI_MEM_DIN0_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_din0_mode(&self) -> u32 {
        (self.bits & SpiMemDinMode::SPI_MEM_DIN0_MODE_MASK)
            >> SpiMemDinMode::SPI_MEM_DIN0_MODE_OFFSET
    }

    /// Set **SPI_MEM_DINS_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_dins_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DINS_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DINS_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DINS_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN7_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_din7_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN7_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN7_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN7_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN6_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_din6_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN6_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN6_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN6_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN5_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_din5_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN5_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN5_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN5_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN4_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_din4_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN4_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN4_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN4_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN3_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_din3_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN3_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN3_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN3_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN2_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_din2_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN2_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN2_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN2_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN1_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_din1_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN1_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN1_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN1_MODE_MASK),
        }
    }

    /// Set **SPI_MEM_DIN0_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_din0_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinMode::SPI_MEM_DIN0_MODE_MASK)
                | ((value << SpiMemDinMode::SPI_MEM_DIN0_MODE_OFFSET)
                    & SpiMemDinMode::SPI_MEM_DIN0_MODE_MASK),
        }
    }
}

impl Default for SpiMemDinModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemDinMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDinModeVal {
        SpiMemDinModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDinModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDinModeVal) -> SpiMemDinModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_DIN_NUM Register
///
/// MSPI flash input timing delay number control register
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **SPI_MEM_DINS_NUM** `[17:16]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN7_NUM** `[15:14]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN6_NUM** `[13:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN5_NUM** `[11:10]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN4_NUM** `[9:8]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN3_NUM** `[7:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN2_NUM** `[5:4]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN1_NUM** `[3:2]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_MEM_DIN0_NUM** `[1:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
#[repr(transparent)]
pub struct SpiMemDinNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDinNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDinNum {
    // ======== Field Constants ========

    /// **SPI_MEM_DINS_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SPI_MEM_DINS_NUM_OFFSET: u32 = 16;
    pub const SPI_MEM_DINS_NUM_MASK: u32 = 0x00030000;

    /// **SPI_MEM_DIN7_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SPI_MEM_DIN7_NUM_OFFSET: u32 = 14;
    pub const SPI_MEM_DIN7_NUM_MASK: u32 = 0x0000c000;

    /// **SPI_MEM_DIN6_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const SPI_MEM_DIN6_NUM_OFFSET: u32 = 12;
    pub const SPI_MEM_DIN6_NUM_MASK: u32 = 0x00003000;

    /// **SPI_MEM_DIN5_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const SPI_MEM_DIN5_NUM_OFFSET: u32 = 10;
    pub const SPI_MEM_DIN5_NUM_MASK: u32 = 0x00000c00;

    /// **SPI_MEM_DIN4_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPI_MEM_DIN4_NUM_OFFSET: u32 = 8;
    pub const SPI_MEM_DIN4_NUM_MASK: u32 = 0x00000300;

    /// **SPI_MEM_DIN3_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SPI_MEM_DIN3_NUM_OFFSET: u32 = 6;
    pub const SPI_MEM_DIN3_NUM_MASK: u32 = 0x000000c0;

    /// **SPI_MEM_DIN2_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SPI_MEM_DIN2_NUM_OFFSET: u32 = 4;
    pub const SPI_MEM_DIN2_NUM_MASK: u32 = 0x00000030;

    /// **SPI_MEM_DIN1_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SPI_MEM_DIN1_NUM_OFFSET: u32 = 2;
    pub const SPI_MEM_DIN1_NUM_MASK: u32 = 0x0000000c;

    /// **SPI_MEM_DIN0_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_MEM_DIN0_NUM_OFFSET: u32 = 0;
    pub const SPI_MEM_DIN0_NUM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_MEM_DINS_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_dins_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DINS_NUM_MASK) >> Self::SPI_MEM_DINS_NUM_OFFSET
    }

    /// Write **SPI_MEM_DINS_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_dins_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DINS_NUM_MASK)
                | ((value << Self::SPI_MEM_DINS_NUM_OFFSET) & Self::SPI_MEM_DINS_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din7_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN7_NUM_MASK) >> Self::SPI_MEM_DIN7_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din7_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN7_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN7_NUM_OFFSET) & Self::SPI_MEM_DIN7_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din6_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN6_NUM_MASK) >> Self::SPI_MEM_DIN6_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din6_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN6_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN6_NUM_OFFSET) & Self::SPI_MEM_DIN6_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din5_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN5_NUM_MASK) >> Self::SPI_MEM_DIN5_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din5_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN5_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN5_NUM_OFFSET) & Self::SPI_MEM_DIN5_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din4_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN4_NUM_MASK) >> Self::SPI_MEM_DIN4_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din4_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN4_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN4_NUM_OFFSET) & Self::SPI_MEM_DIN4_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din3_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN3_NUM_MASK) >> Self::SPI_MEM_DIN3_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din3_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN3_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN3_NUM_OFFSET) & Self::SPI_MEM_DIN3_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din2_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN2_NUM_MASK) >> Self::SPI_MEM_DIN2_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din2_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN2_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN2_NUM_OFFSET) & Self::SPI_MEM_DIN2_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din1_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN1_NUM_MASK) >> Self::SPI_MEM_DIN1_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din1_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN1_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN1_NUM_OFFSET) & Self::SPI_MEM_DIN1_NUM_MASK)
        });
    }

    /// **SPI_MEM_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_mem_din0_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DIN0_NUM_MASK) >> Self::SPI_MEM_DIN0_NUM_OFFSET
    }

    /// Write **SPI_MEM_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_mem_din0_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DIN0_NUM_MASK)
                | ((value << Self::SPI_MEM_DIN0_NUM_OFFSET) & Self::SPI_MEM_DIN0_NUM_MASK)
        });
    }
}

/// SPI_MEM_DIN_NUM value type for batch read/modify/write operations
///
/// Use `spimemdinnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemdinnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDinNumVal {
    bits: u32,
}

impl SpiMemDinNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_DINS_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_dins_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DINS_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DINS_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din7_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN7_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN7_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din6_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN6_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN6_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din5_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN5_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN5_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din4_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN4_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN4_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din3_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN3_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN3_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din2_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN2_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN2_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din1_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN1_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN1_NUM_OFFSET
    }

    /// Read **SPI_MEM_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_mem_din0_num(&self) -> u32 {
        (self.bits & SpiMemDinNum::SPI_MEM_DIN0_NUM_MASK) >> SpiMemDinNum::SPI_MEM_DIN0_NUM_OFFSET
    }

    /// Set **SPI_MEM_DINS_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_dins_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DINS_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DINS_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DINS_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din7_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN7_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN7_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN7_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din6_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN6_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN6_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN6_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din5_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN5_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN5_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN5_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din4_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN4_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN4_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN4_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din3_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN3_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN3_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN3_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din2_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN2_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN2_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN2_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din1_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN1_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN1_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN1_NUM_MASK),
        }
    }

    /// Set **SPI_MEM_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_mem_din0_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDinNum::SPI_MEM_DIN0_NUM_MASK)
                | ((value << SpiMemDinNum::SPI_MEM_DIN0_NUM_OFFSET)
                    & SpiMemDinNum::SPI_MEM_DIN0_NUM_MASK),
        }
    }
}

impl Default for SpiMemDinNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemDinNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDinNumVal {
        SpiMemDinNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDinNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDinNumVal) -> SpiMemDinNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_DOUT_MODE Register
///
/// MSPI flash output timing adjustment control register
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **SPI_MEM_DOUTS_MODE** `[8]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
///
/// - **SPI_MEM_DOUT7_MODE** `[7]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
///
/// - **SPI_MEM_DOUT6_MODE** `[6]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
///
/// - **SPI_MEM_DOUT5_MODE** `[5]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
///
/// - **SPI_MEM_DOUT4_MODE** `[4]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
///
/// - **SPI_MEM_DOUT3_MODE** `[3]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_MEM_DOUT2_MODE** `[2]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_MEM_DOUT1_MODE** `[1]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_MEM_DOUT0_MODE** `[0]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
#[repr(transparent)]
pub struct SpiMemDoutMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDoutMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDoutMode {
    // ======== Field Constants ========

    /// **SPI_MEM_DOUTS_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_DOUTS_MODE_OFFSET: u32 = 8;
    pub const SPI_MEM_DOUTS_MODE_MASK: u32 = 0x00000100;
    pub const SPI_MEM_DOUTS_MODE: u32 = 1 << 8;

    /// **SPI_MEM_DOUT7_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_DOUT7_MODE_OFFSET: u32 = 7;
    pub const SPI_MEM_DOUT7_MODE_MASK: u32 = 0x00000080;
    pub const SPI_MEM_DOUT7_MODE: u32 = 1 << 7;

    /// **SPI_MEM_DOUT6_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_DOUT6_MODE_OFFSET: u32 = 6;
    pub const SPI_MEM_DOUT6_MODE_MASK: u32 = 0x00000040;
    pub const SPI_MEM_DOUT6_MODE: u32 = 1 << 6;

    /// **SPI_MEM_DOUT5_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_DOUT5_MODE_OFFSET: u32 = 5;
    pub const SPI_MEM_DOUT5_MODE_MASK: u32 = 0x00000020;
    pub const SPI_MEM_DOUT5_MODE: u32 = 1 << 5;

    /// **SPI_MEM_DOUT4_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_DOUT4_MODE_OFFSET: u32 = 4;
    pub const SPI_MEM_DOUT4_MODE_MASK: u32 = 0x00000010;
    pub const SPI_MEM_DOUT4_MODE: u32 = 1 << 4;

    /// **SPI_MEM_DOUT3_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_DOUT3_MODE_OFFSET: u32 = 3;
    pub const SPI_MEM_DOUT3_MODE_MASK: u32 = 0x00000008;
    pub const SPI_MEM_DOUT3_MODE: u32 = 1 << 3;

    /// **SPI_MEM_DOUT2_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_DOUT2_MODE_OFFSET: u32 = 2;
    pub const SPI_MEM_DOUT2_MODE_MASK: u32 = 0x00000004;
    pub const SPI_MEM_DOUT2_MODE: u32 = 1 << 2;

    /// **SPI_MEM_DOUT1_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_DOUT1_MODE_OFFSET: u32 = 1;
    pub const SPI_MEM_DOUT1_MODE_MASK: u32 = 0x00000002;
    pub const SPI_MEM_DOUT1_MODE: u32 = 1 << 1;

    /// **SPI_MEM_DOUT0_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_DOUT0_MODE_OFFSET: u32 = 0;
    pub const SPI_MEM_DOUT0_MODE_MASK: u32 = 0x00000001;
    pub const SPI_MEM_DOUT0_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_DOUTS_MODE** `[8]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub fn spi_mem_douts_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUTS_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUTS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_douts_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUTS_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUTS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_douts_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUTS_MODE_MASK);
    }

    /// **SPI_MEM_DOUT7_MODE** `[7]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub fn spi_mem_dout7_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT7_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout7_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT7_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout7_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT7_MODE_MASK);
    }

    /// **SPI_MEM_DOUT6_MODE** `[6]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub fn spi_mem_dout6_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT6_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout6_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT6_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout6_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT6_MODE_MASK);
    }

    /// **SPI_MEM_DOUT5_MODE** `[5]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub fn spi_mem_dout5_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT5_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout5_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT5_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout5_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT5_MODE_MASK);
    }

    /// **SPI_MEM_DOUT4_MODE** `[4]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub fn spi_mem_dout4_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT4_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout4_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT4_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout4_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT4_MODE_MASK);
    }

    /// **SPI_MEM_DOUT3_MODE** `[3]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_dout3_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT3_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout3_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT3_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout3_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT3_MODE_MASK);
    }

    /// **SPI_MEM_DOUT2_MODE** `[2]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_dout2_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT2_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout2_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT2_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout2_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT2_MODE_MASK);
    }

    /// **SPI_MEM_DOUT1_MODE** `[1]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_dout1_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT1_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout1_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT1_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout1_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT1_MODE_MASK);
    }

    /// **SPI_MEM_DOUT0_MODE** `[0]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_mem_dout0_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_DOUT0_MODE_MASK) != 0
    }

    /// Set SPI_MEM_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_dout0_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_DOUT0_MODE_MASK);
    }

    /// Clear SPI_MEM_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_dout0_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_DOUT0_MODE_MASK);
    }
}

/// SPI_MEM_DOUT_MODE value type for batch read/modify/write operations
///
/// Use `spimemdoutmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemdoutmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDoutModeVal {
    bits: u32,
}

impl SpiMemDoutModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_DOUTS_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_douts_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUTS_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT7_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_dout7_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT7_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT6_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_dout6_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT6_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT5_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_dout5_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT5_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT4_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn spi_mem_dout4_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT4_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT3_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_dout3_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT3_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT2_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_dout2_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT2_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT1_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_dout1_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT1_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_DOUT0_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_mem_dout0_mode(&self) -> bool {
        (self.bits & SpiMemDoutMode::SPI_MEM_DOUT0_MODE_MASK) != 0
    }

    /// Set **SPI_MEM_DOUTS_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_douts_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUTS_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUTS_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT7_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_dout7_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT7_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT7_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT6_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_dout6_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT6_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT6_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT5_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_dout5_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT5_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT5_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT4_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk
    #[inline(always)]
    pub const fn with_spi_mem_dout4_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT4_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT4_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT3_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_dout3_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT3_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT3_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT2_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_dout2_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT2_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT2_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT1_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_dout1_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT1_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT1_MODE_MASK
            },
        }
    }

    /// Set **SPI_MEM_DOUT0_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_mem_dout0_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDoutMode::SPI_MEM_DOUT0_MODE_MASK
            } else {
                self.bits & !SpiMemDoutMode::SPI_MEM_DOUT0_MODE_MASK
            },
        }
    }
}

impl Default for SpiMemDoutModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemDoutMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDoutModeVal {
        SpiMemDoutModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDoutModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDoutModeVal) -> SpiMemDoutModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_TIMING_CALI Register
///
/// MSPI external RAM timing calibration register
///
/// Address offset: 0x0190
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **SPI_SMEM_DLL_TIMING_CALI** `[5]` (RW)
///
///   Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM.
///
/// - **SPI_SMEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
///
///   For sram, add extra dummy spi clock cycle length for spi clock calibration.
///
/// - **SPI_SMEM_TIMING_CALI** `[1]` (RW)
///
///   For sram, the bit is used to enable timing auto-calibration for all reading operations.
///
/// - **SPI_SMEM_TIMING_CLK_ENA** `[0]` (RW)
///
///   For sram, the bit is used to enable timing adjust clock for all reading operations.
#[repr(transparent)]
pub struct SpiSmemTimingCali {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemTimingCali {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemTimingCali {
    // ======== Field Constants ========

    /// **SPI_SMEM_DLL_TIMING_CALI**
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SMEM_DLL_TIMING_CALI_OFFSET: u32 = 5;
    pub const SPI_SMEM_DLL_TIMING_CALI_MASK: u32 = 0x00000020;
    pub const SPI_SMEM_DLL_TIMING_CALI: u32 = 1 << 5;

    /// **SPI_SMEM_EXTRA_DUMMY_CYCLELEN**
    ///
    /// For sram, add extra dummy spi clock cycle length for spi clock calibration.
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const SPI_SMEM_EXTRA_DUMMY_CYCLELEN_OFFSET: u32 = 2;
    pub const SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK: u32 = 0x0000001c;

    /// **SPI_SMEM_TIMING_CALI**
    ///
    /// For sram, the bit is used to enable timing auto-calibration for all reading operations.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_TIMING_CALI_OFFSET: u32 = 1;
    pub const SPI_SMEM_TIMING_CALI_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_TIMING_CALI: u32 = 1 << 1;

    /// **SPI_SMEM_TIMING_CLK_ENA**
    ///
    /// For sram, the bit is used to enable timing adjust clock for all reading operations.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_TIMING_CLK_ENA_OFFSET: u32 = 0;
    pub const SPI_SMEM_TIMING_CLK_ENA_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_TIMING_CLK_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DLL_TIMING_CALI** `[5]` (RW)
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM.
    #[inline(always)]
    pub fn spi_smem_dll_timing_cali(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DLL_TIMING_CALI_MASK) != 0
    }

    /// Set SPI_SMEM_DLL_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dll_timing_cali(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DLL_TIMING_CALI_MASK);
    }

    /// Clear SPI_SMEM_DLL_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dll_timing_cali(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DLL_TIMING_CALI_MASK);
    }

    /// **SPI_SMEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
    ///
    /// For sram, add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK)
            >> Self::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_SMEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
    ///
    /// For sram, add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub fn write_spi_smem_extra_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK)
        });
    }

    /// **SPI_SMEM_TIMING_CALI** `[1]` (RW)
    ///
    /// For sram, the bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_TIMING_CALI_MASK) != 0
    }

    /// Set SPI_SMEM_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_timing_cali(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_TIMING_CALI_MASK);
    }

    /// Clear SPI_SMEM_TIMING_CALI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_timing_cali(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_TIMING_CALI_MASK);
    }

    /// **SPI_SMEM_TIMING_CLK_ENA** `[0]` (RW)
    ///
    /// For sram, the bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_TIMING_CLK_ENA_MASK) != 0
    }

    /// Set SPI_SMEM_TIMING_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_timing_clk_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_TIMING_CLK_ENA_MASK);
    }

    /// Clear SPI_SMEM_TIMING_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_timing_clk_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_TIMING_CLK_ENA_MASK);
    }
}

/// SPI_SMEM_TIMING_CALI value type for batch read/modify/write operations
///
/// Use `spismemtimingcali.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemtimingcali.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemTimingCaliVal {
    bits: u32,
}

impl SpiSmemTimingCaliVal {
    /// Create from raw bits
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

    /// Read **SPI_SMEM_DLL_TIMING_CALI** `[5:5]`
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM.
    #[inline(always)]
    pub const fn spi_smem_dll_timing_cali(&self) -> bool {
        (self.bits & SpiSmemTimingCali::SPI_SMEM_DLL_TIMING_CALI_MASK) != 0
    }

    /// Read **SPI_SMEM_EXTRA_DUMMY_CYCLELEN** `[4:2]`
    ///
    /// For sram, add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub const fn spi_smem_extra_dummy_cyclelen(&self) -> u32 {
        (self.bits & SpiSmemTimingCali::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK)
            >> SpiSmemTimingCali::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_OFFSET
    }

    /// Read **SPI_SMEM_TIMING_CALI** `[1:1]`
    ///
    /// For sram, the bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> bool {
        (self.bits & SpiSmemTimingCali::SPI_SMEM_TIMING_CALI_MASK) != 0
    }

    /// Read **SPI_SMEM_TIMING_CLK_ENA** `[0:0]`
    ///
    /// For sram, the bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub const fn spi_smem_timing_clk_ena(&self) -> bool {
        (self.bits & SpiSmemTimingCali::SPI_SMEM_TIMING_CLK_ENA_MASK) != 0
    }

    /// Set **SPI_SMEM_DLL_TIMING_CALI** `[5:5]`
    ///
    /// Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM.
    #[inline(always)]
    pub const fn with_spi_smem_dll_timing_cali(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemTimingCali::SPI_SMEM_DLL_TIMING_CALI_MASK
            } else {
                self.bits & !SpiSmemTimingCali::SPI_SMEM_DLL_TIMING_CALI_MASK
            },
        }
    }

    /// Set **SPI_SMEM_EXTRA_DUMMY_CYCLELEN** `[4:2]`
    ///
    /// For sram, add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub const fn with_spi_smem_extra_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemTimingCali::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK)
                | ((value << SpiSmemTimingCali::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_OFFSET)
                    & SpiSmemTimingCali::SPI_SMEM_EXTRA_DUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_SMEM_TIMING_CALI** `[1:1]`
    ///
    /// For sram, the bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub const fn with_spi_smem_timing_cali(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemTimingCali::SPI_SMEM_TIMING_CALI_MASK
            } else {
                self.bits & !SpiSmemTimingCali::SPI_SMEM_TIMING_CALI_MASK
            },
        }
    }

    /// Set **SPI_SMEM_TIMING_CLK_ENA** `[0:0]`
    ///
    /// For sram, the bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub const fn with_spi_smem_timing_clk_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemTimingCali::SPI_SMEM_TIMING_CLK_ENA_MASK
            } else {
                self.bits & !SpiSmemTimingCali::SPI_SMEM_TIMING_CLK_ENA_MASK
            },
        }
    }
}

impl Default for SpiSmemTimingCaliVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemTimingCali {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemTimingCaliVal {
        SpiSmemTimingCaliVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemTimingCaliVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemTimingCaliVal) -> SpiSmemTimingCaliVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DIN_MODE Register
///
/// MSPI external RAM input timing delay mode control register
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **SPI_SMEM_DINS_MODE** `[26:24]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN7_MODE** `[23:21]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN6_MODE** `[20:18]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN5_MODE** `[17:15]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN4_MODE** `[14:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN3_MODE** `[11:9]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN2_MODE** `[8:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN1_MODE** `[5:3]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN0_MODE** `[2:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
#[repr(transparent)]
pub struct SpiSmemDinMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDinMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDinMode {
    // ======== Field Constants ========

    /// **SPI_SMEM_DINS_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SPI_SMEM_DINS_MODE_OFFSET: u32 = 24;
    pub const SPI_SMEM_DINS_MODE_MASK: u32 = 0x07000000;

    /// **SPI_SMEM_DIN7_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const SPI_SMEM_DIN7_MODE_OFFSET: u32 = 21;
    pub const SPI_SMEM_DIN7_MODE_MASK: u32 = 0x00e00000;

    /// **SPI_SMEM_DIN6_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SPI_SMEM_DIN6_MODE_OFFSET: u32 = 18;
    pub const SPI_SMEM_DIN6_MODE_MASK: u32 = 0x001c0000;

    /// **SPI_SMEM_DIN5_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const SPI_SMEM_DIN5_MODE_OFFSET: u32 = 15;
    pub const SPI_SMEM_DIN5_MODE_MASK: u32 = 0x00038000;

    /// **SPI_SMEM_DIN4_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SPI_SMEM_DIN4_MODE_OFFSET: u32 = 12;
    pub const SPI_SMEM_DIN4_MODE_MASK: u32 = 0x00007000;

    /// **SPI_SMEM_DIN3_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const SPI_SMEM_DIN3_MODE_OFFSET: u32 = 9;
    pub const SPI_SMEM_DIN3_MODE_MASK: u32 = 0x00000e00;

    /// **SPI_SMEM_DIN2_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SPI_SMEM_DIN2_MODE_OFFSET: u32 = 6;
    pub const SPI_SMEM_DIN2_MODE_MASK: u32 = 0x000001c0;

    /// **SPI_SMEM_DIN1_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const SPI_SMEM_DIN1_MODE_OFFSET: u32 = 3;
    pub const SPI_SMEM_DIN1_MODE_MASK: u32 = 0x00000038;

    /// **SPI_SMEM_DIN0_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SPI_SMEM_DIN0_MODE_OFFSET: u32 = 0;
    pub const SPI_SMEM_DIN0_MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DINS_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dins_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DINS_MODE_MASK) >> Self::SPI_SMEM_DINS_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DINS_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_dins_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DINS_MODE_MASK)
                | ((value << Self::SPI_SMEM_DINS_MODE_OFFSET) & Self::SPI_SMEM_DINS_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN7_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din7_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN7_MODE_MASK) >> Self::SPI_SMEM_DIN7_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN7_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din7_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN7_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN7_MODE_OFFSET) & Self::SPI_SMEM_DIN7_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN6_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din6_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN6_MODE_MASK) >> Self::SPI_SMEM_DIN6_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN6_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din6_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN6_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN6_MODE_OFFSET) & Self::SPI_SMEM_DIN6_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN5_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din5_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN5_MODE_MASK) >> Self::SPI_SMEM_DIN5_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN5_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din5_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN5_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN5_MODE_OFFSET) & Self::SPI_SMEM_DIN5_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN4_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din4_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN4_MODE_MASK) >> Self::SPI_SMEM_DIN4_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN4_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din4_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN4_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN4_MODE_OFFSET) & Self::SPI_SMEM_DIN4_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN3_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din3_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN3_MODE_MASK) >> Self::SPI_SMEM_DIN3_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN3_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din3_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN3_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN3_MODE_OFFSET) & Self::SPI_SMEM_DIN3_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN2_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din2_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN2_MODE_MASK) >> Self::SPI_SMEM_DIN2_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN2_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din2_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN2_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN2_MODE_OFFSET) & Self::SPI_SMEM_DIN2_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN1_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din1_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN1_MODE_MASK) >> Self::SPI_SMEM_DIN1_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN1_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din1_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN1_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN1_MODE_OFFSET) & Self::SPI_SMEM_DIN1_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN0_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din0_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN0_MODE_MASK) >> Self::SPI_SMEM_DIN0_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN0_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din0_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN0_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN0_MODE_OFFSET) & Self::SPI_SMEM_DIN0_MODE_MASK)
        });
    }
}

/// SPI_SMEM_DIN_MODE value type for batch read/modify/write operations
///
/// Use `spismemdinmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdinmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDinModeVal {
    bits: u32,
}

impl SpiSmemDinModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DINS_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dins_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DINS_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DINS_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN7_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din7_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN7_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN7_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN6_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din6_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN6_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN6_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN5_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din5_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN5_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN5_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN4_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din4_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN4_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN4_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN3_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din3_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN3_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN3_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN2_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din2_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN2_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN2_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN1_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din1_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN1_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN1_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN0_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din0_mode(&self) -> u32 {
        (self.bits & SpiSmemDinMode::SPI_SMEM_DIN0_MODE_MASK)
            >> SpiSmemDinMode::SPI_SMEM_DIN0_MODE_OFFSET
    }

    /// Set **SPI_SMEM_DINS_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dins_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DINS_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DINS_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DINS_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN7_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din7_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN7_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN7_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN7_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN6_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din6_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN6_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN6_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN6_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN5_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din5_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN5_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN5_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN5_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN4_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din4_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN4_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN4_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN4_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN3_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din3_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN3_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN3_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN3_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN2_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din2_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN2_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN2_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN2_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN1_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din1_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN1_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN1_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN1_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN0_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din0_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinMode::SPI_SMEM_DIN0_MODE_MASK)
                | ((value << SpiSmemDinMode::SPI_SMEM_DIN0_MODE_OFFSET)
                    & SpiSmemDinMode::SPI_SMEM_DIN0_MODE_MASK),
        }
    }
}

impl Default for SpiSmemDinModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDinMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDinModeVal {
        SpiSmemDinModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDinModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDinModeVal) -> SpiSmemDinModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DIN_NUM Register
///
/// MSPI external RAM input timing delay number control register
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **SPI_SMEM_DINS_NUM** `[17:16]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN7_NUM** `[15:14]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN6_NUM** `[13:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN5_NUM** `[11:10]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN4_NUM** `[9:8]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN3_NUM** `[7:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN2_NUM** `[5:4]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN1_NUM** `[3:2]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN0_NUM** `[1:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
#[repr(transparent)]
pub struct SpiSmemDinNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDinNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDinNum {
    // ======== Field Constants ========

    /// **SPI_SMEM_DINS_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SPI_SMEM_DINS_NUM_OFFSET: u32 = 16;
    pub const SPI_SMEM_DINS_NUM_MASK: u32 = 0x00030000;

    /// **SPI_SMEM_DIN7_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SPI_SMEM_DIN7_NUM_OFFSET: u32 = 14;
    pub const SPI_SMEM_DIN7_NUM_MASK: u32 = 0x0000c000;

    /// **SPI_SMEM_DIN6_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const SPI_SMEM_DIN6_NUM_OFFSET: u32 = 12;
    pub const SPI_SMEM_DIN6_NUM_MASK: u32 = 0x00003000;

    /// **SPI_SMEM_DIN5_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const SPI_SMEM_DIN5_NUM_OFFSET: u32 = 10;
    pub const SPI_SMEM_DIN5_NUM_MASK: u32 = 0x00000c00;

    /// **SPI_SMEM_DIN4_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPI_SMEM_DIN4_NUM_OFFSET: u32 = 8;
    pub const SPI_SMEM_DIN4_NUM_MASK: u32 = 0x00000300;

    /// **SPI_SMEM_DIN3_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SPI_SMEM_DIN3_NUM_OFFSET: u32 = 6;
    pub const SPI_SMEM_DIN3_NUM_MASK: u32 = 0x000000c0;

    /// **SPI_SMEM_DIN2_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SPI_SMEM_DIN2_NUM_OFFSET: u32 = 4;
    pub const SPI_SMEM_DIN2_NUM_MASK: u32 = 0x00000030;

    /// **SPI_SMEM_DIN1_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SPI_SMEM_DIN1_NUM_OFFSET: u32 = 2;
    pub const SPI_SMEM_DIN1_NUM_MASK: u32 = 0x0000000c;

    /// **SPI_SMEM_DIN0_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_SMEM_DIN0_NUM_OFFSET: u32 = 0;
    pub const SPI_SMEM_DIN0_NUM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DINS_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_dins_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DINS_NUM_MASK) >> Self::SPI_SMEM_DINS_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DINS_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_dins_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DINS_NUM_MASK)
                | ((value << Self::SPI_SMEM_DINS_NUM_OFFSET) & Self::SPI_SMEM_DINS_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din7_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN7_NUM_MASK) >> Self::SPI_SMEM_DIN7_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN7_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din7_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN7_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN7_NUM_OFFSET) & Self::SPI_SMEM_DIN7_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din6_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN6_NUM_MASK) >> Self::SPI_SMEM_DIN6_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN6_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din6_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN6_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN6_NUM_OFFSET) & Self::SPI_SMEM_DIN6_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din5_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN5_NUM_MASK) >> Self::SPI_SMEM_DIN5_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN5_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din5_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN5_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN5_NUM_OFFSET) & Self::SPI_SMEM_DIN5_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din4_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN4_NUM_MASK) >> Self::SPI_SMEM_DIN4_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN4_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din4_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN4_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN4_NUM_OFFSET) & Self::SPI_SMEM_DIN4_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din3_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN3_NUM_MASK) >> Self::SPI_SMEM_DIN3_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN3_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din3_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN3_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN3_NUM_OFFSET) & Self::SPI_SMEM_DIN3_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din2_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN2_NUM_MASK) >> Self::SPI_SMEM_DIN2_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN2_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din2_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN2_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN2_NUM_OFFSET) & Self::SPI_SMEM_DIN2_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din1_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN1_NUM_MASK) >> Self::SPI_SMEM_DIN1_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN1_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din1_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN1_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN1_NUM_OFFSET) & Self::SPI_SMEM_DIN1_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din0_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN0_NUM_MASK) >> Self::SPI_SMEM_DIN0_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN0_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din0_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN0_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN0_NUM_OFFSET) & Self::SPI_SMEM_DIN0_NUM_MASK)
        });
    }
}

/// SPI_SMEM_DIN_NUM value type for batch read/modify/write operations
///
/// Use `spismemdinnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdinnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDinNumVal {
    bits: u32,
}

impl SpiSmemDinNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DINS_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_dins_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DINS_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DINS_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din7_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN7_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN7_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din6_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN6_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN6_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din5_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN5_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN5_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din4_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN4_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN4_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din3_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN3_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN3_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din2_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN2_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN2_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din1_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN1_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN1_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din0_num(&self) -> u32 {
        (self.bits & SpiSmemDinNum::SPI_SMEM_DIN0_NUM_MASK)
            >> SpiSmemDinNum::SPI_SMEM_DIN0_NUM_OFFSET
    }

    /// Set **SPI_SMEM_DINS_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_dins_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DINS_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DINS_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DINS_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN7_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din7_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN7_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN7_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN7_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN6_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din6_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN6_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN6_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN6_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN5_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din5_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN5_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN5_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN5_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN4_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din4_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN4_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN4_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN4_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN3_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din3_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN3_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN3_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN3_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN2_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din2_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN2_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN2_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN2_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN1_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din1_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN1_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN1_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN1_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN0_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din0_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinNum::SPI_SMEM_DIN0_NUM_MASK)
                | ((value << SpiSmemDinNum::SPI_SMEM_DIN0_NUM_OFFSET)
                    & SpiSmemDinNum::SPI_SMEM_DIN0_NUM_MASK),
        }
    }
}

impl Default for SpiSmemDinNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDinNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDinNumVal {
        SpiSmemDinNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDinNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDinNumVal) -> SpiSmemDinNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DOUT_MODE Register
///
/// MSPI external RAM output timing adjustment control register
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **SPI_SMEM_DOUTS_MODE** `[8]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT7_MODE** `[7]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT6_MODE** `[6]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT5_MODE** `[5]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT4_MODE** `[4]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT3_MODE** `[3]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT2_MODE** `[2]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT1_MODE** `[1]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT0_MODE** `[0]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
#[repr(transparent)]
pub struct SpiSmemDoutMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDoutMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDoutMode {
    // ======== Field Constants ========

    /// **SPI_SMEM_DOUTS_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SMEM_DOUTS_MODE_OFFSET: u32 = 8;
    pub const SPI_SMEM_DOUTS_MODE_MASK: u32 = 0x00000100;
    pub const SPI_SMEM_DOUTS_MODE: u32 = 1 << 8;

    /// **SPI_SMEM_DOUT7_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SMEM_DOUT7_MODE_OFFSET: u32 = 7;
    pub const SPI_SMEM_DOUT7_MODE_MASK: u32 = 0x00000080;
    pub const SPI_SMEM_DOUT7_MODE: u32 = 1 << 7;

    /// **SPI_SMEM_DOUT6_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SMEM_DOUT6_MODE_OFFSET: u32 = 6;
    pub const SPI_SMEM_DOUT6_MODE_MASK: u32 = 0x00000040;
    pub const SPI_SMEM_DOUT6_MODE: u32 = 1 << 6;

    /// **SPI_SMEM_DOUT5_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SMEM_DOUT5_MODE_OFFSET: u32 = 5;
    pub const SPI_SMEM_DOUT5_MODE_MASK: u32 = 0x00000020;
    pub const SPI_SMEM_DOUT5_MODE: u32 = 1 << 5;

    /// **SPI_SMEM_DOUT4_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SMEM_DOUT4_MODE_OFFSET: u32 = 4;
    pub const SPI_SMEM_DOUT4_MODE_MASK: u32 = 0x00000010;
    pub const SPI_SMEM_DOUT4_MODE: u32 = 1 << 4;

    /// **SPI_SMEM_DOUT3_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SMEM_DOUT3_MODE_OFFSET: u32 = 3;
    pub const SPI_SMEM_DOUT3_MODE_MASK: u32 = 0x00000008;
    pub const SPI_SMEM_DOUT3_MODE: u32 = 1 << 3;

    /// **SPI_SMEM_DOUT2_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_DOUT2_MODE_OFFSET: u32 = 2;
    pub const SPI_SMEM_DOUT2_MODE_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_DOUT2_MODE: u32 = 1 << 2;

    /// **SPI_SMEM_DOUT1_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_DOUT1_MODE_OFFSET: u32 = 1;
    pub const SPI_SMEM_DOUT1_MODE_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_DOUT1_MODE: u32 = 1 << 1;

    /// **SPI_SMEM_DOUT0_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_DOUT0_MODE_OFFSET: u32 = 0;
    pub const SPI_SMEM_DOUT0_MODE_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_DOUT0_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DOUTS_MODE** `[8]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_douts_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUTS_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUTS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_douts_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUTS_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUTS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_douts_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUTS_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT7_MODE** `[7]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout7_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT7_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout7_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT7_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT7_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout7_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT7_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT6_MODE** `[6]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout6_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT6_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout6_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT6_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT6_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout6_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT6_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT5_MODE** `[5]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout5_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT5_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout5_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT5_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT5_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout5_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT5_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT4_MODE** `[4]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout4_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT4_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout4_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT4_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT4_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout4_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT4_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT3_MODE** `[3]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout3_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT3_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout3_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT3_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT3_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout3_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT3_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT2_MODE** `[2]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout2_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT2_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout2_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT2_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT2_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout2_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT2_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT1_MODE** `[1]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout1_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT1_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout1_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT1_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT1_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout1_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT1_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT0_MODE** `[0]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout0_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT0_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout0_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT0_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT0_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout0_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT0_MODE_MASK);
    }
}

/// SPI_SMEM_DOUT_MODE value type for batch read/modify/write operations
///
/// Use `spismemdoutmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdoutmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDoutModeVal {
    bits: u32,
}

impl SpiSmemDoutModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DOUTS_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_douts_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUTS_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT7_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout7_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT7_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT6_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout6_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT6_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT5_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout5_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT5_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT4_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout4_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT4_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT3_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout3_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT3_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT2_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout2_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT2_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT1_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout1_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT1_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT0_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout0_mode(&self) -> bool {
        (self.bits & SpiSmemDoutMode::SPI_SMEM_DOUT0_MODE_MASK) != 0
    }

    /// Set **SPI_SMEM_DOUTS_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_douts_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUTS_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUTS_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT7_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout7_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT7_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT7_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT6_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout6_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT6_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT6_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT5_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout5_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT5_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT5_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT4_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout4_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT4_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT4_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT3_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout3_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT3_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT3_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT2_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout2_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT2_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT2_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT1_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout1_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT1_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT1_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT0_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout0_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutMode::SPI_SMEM_DOUT0_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutMode::SPI_SMEM_DOUT0_MODE_MASK
            },
        }
    }
}

impl Default for SpiSmemDoutModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDoutMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDoutModeVal {
        SpiSmemDoutModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDoutModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDoutModeVal) -> SpiSmemDoutModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_AC Register
///
/// MSPI external RAM ECC and SPI CS timing control register
///
/// Address offset: 0x01a0
/// Reset value: 0x8000b084
///
/// ## Fields
///
/// - **SPI_SMEM_SPLIT_TRANS_EN** `[31]` (RW)
///
///   Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not.
///
/// - **SPI_SMEM_CS_HOLD_DELAY** `[30:25]` (RW)
///
///   These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
///
/// - **SPI_SMEM_ECC_16TO18_BYTE_EN** `[16]` (RW)
///
///   Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM.
///
/// - **SPI_SMEM_ECC_SKIP_PAGE_CORNER** `[15]` (RW)
///
///   1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM.
///
/// - **SPI_SMEM_ECC_CS_HOLD_TIME** `[14:12]` (RW)
///
///   SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
///
/// - **SPI_SMEM_CS_HOLD_TIME** `[11:7]` (RW)
///
///   For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
///
/// - **SPI_SMEM_CS_SETUP_TIME** `[6:2]` (RW)
///
///   For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
///
/// - **SPI_SMEM_CS_HOLD** `[1]` (RW)
///
///   For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable.
///
/// - **SPI_SMEM_CS_SETUP** `[0]` (RW)
///
///   For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
#[repr(transparent)]
pub struct SpiSmemAc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemAc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemAc {
    // ======== Field Constants ========

    /// **SPI_SMEM_SPLIT_TRANS_EN**
    ///
    /// Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_SMEM_SPLIT_TRANS_EN_OFFSET: u32 = 31;
    pub const SPI_SMEM_SPLIT_TRANS_EN_MASK: u32 = 0x80000000;
    pub const SPI_SMEM_SPLIT_TRANS_EN: u32 = 1 << 31;

    /// **SPI_SMEM_CS_HOLD_DELAY**
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    ///
    /// Bits `[30:25]`, mask `0x7e000000`
    pub const SPI_SMEM_CS_HOLD_DELAY_OFFSET: u32 = 25;
    pub const SPI_SMEM_CS_HOLD_DELAY_MASK: u32 = 0x7e000000;

    /// **SPI_SMEM_ECC_16TO18_BYTE_EN**
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SPI_SMEM_ECC_16TO18_BYTE_EN_OFFSET: u32 = 16;
    pub const SPI_SMEM_ECC_16TO18_BYTE_EN_MASK: u32 = 0x00010000;
    pub const SPI_SMEM_ECC_16TO18_BYTE_EN: u32 = 1 << 16;

    /// **SPI_SMEM_ECC_SKIP_PAGE_CORNER**
    ///
    /// 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_SMEM_ECC_SKIP_PAGE_CORNER_OFFSET: u32 = 15;
    pub const SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK: u32 = 0x00008000;
    pub const SPI_SMEM_ECC_SKIP_PAGE_CORNER: u32 = 1 << 15;

    /// **SPI_SMEM_ECC_CS_HOLD_TIME**
    ///
    /// SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SPI_SMEM_ECC_CS_HOLD_TIME_OFFSET: u32 = 12;
    pub const SPI_SMEM_ECC_CS_HOLD_TIME_MASK: u32 = 0x00007000;

    /// **SPI_SMEM_CS_HOLD_TIME**
    ///
    /// For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const SPI_SMEM_CS_HOLD_TIME_OFFSET: u32 = 7;
    pub const SPI_SMEM_CS_HOLD_TIME_MASK: u32 = 0x00000f80;

    /// **SPI_SMEM_CS_SETUP_TIME**
    ///
    /// For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
    ///
    /// Bits `[6:2]`, mask `0x0000007c`
    pub const SPI_SMEM_CS_SETUP_TIME_OFFSET: u32 = 2;
    pub const SPI_SMEM_CS_SETUP_TIME_MASK: u32 = 0x0000007c;

    /// **SPI_SMEM_CS_HOLD**
    ///
    /// For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_CS_HOLD_OFFSET: u32 = 1;
    pub const SPI_SMEM_CS_HOLD_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_CS_HOLD: u32 = 1 << 1;

    /// **SPI_SMEM_CS_SETUP**
    ///
    /// For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_CS_SETUP_OFFSET: u32 = 0;
    pub const SPI_SMEM_CS_SETUP_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_CS_SETUP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_SPLIT_TRANS_EN** `[31]` (RW)
    ///
    /// Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub fn spi_smem_split_trans_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_SPLIT_TRANS_EN_MASK) != 0
    }

    /// Set SPI_SMEM_SPLIT_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_split_trans_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_SPLIT_TRANS_EN_MASK);
    }

    /// Clear SPI_SMEM_SPLIT_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_split_trans_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_SPLIT_TRANS_EN_MASK);
    }

    /// **SPI_SMEM_CS_HOLD_DELAY** `[30:25]` (RW)
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub fn spi_smem_cs_hold_delay(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_CS_HOLD_DELAY_MASK)
            >> Self::SPI_SMEM_CS_HOLD_DELAY_OFFSET
    }

    /// Write **SPI_SMEM_CS_HOLD_DELAY** `[30:25]` (RW)
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub fn write_spi_smem_cs_hold_delay(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_CS_HOLD_DELAY_MASK)
                | ((value << Self::SPI_SMEM_CS_HOLD_DELAY_OFFSET)
                    & Self::SPI_SMEM_CS_HOLD_DELAY_MASK)
        });
    }

    /// **SPI_SMEM_ECC_16TO18_BYTE_EN** `[16]` (RW)
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM.
    #[inline(always)]
    pub fn spi_smem_ecc_16to18_byte_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK) != 0
    }

    /// Set SPI_SMEM_ECC_16TO18_BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_ecc_16to18_byte_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK);
    }

    /// Clear SPI_SMEM_ECC_16TO18_BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_ecc_16to18_byte_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK);
    }

    /// **SPI_SMEM_ECC_SKIP_PAGE_CORNER** `[15]` (RW)
    ///
    /// 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM.
    #[inline(always)]
    pub fn spi_smem_ecc_skip_page_corner(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK) != 0
    }

    /// Set SPI_SMEM_ECC_SKIP_PAGE_CORNER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_ecc_skip_page_corner(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK);
    }

    /// Clear SPI_SMEM_ECC_SKIP_PAGE_CORNER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_ecc_skip_page_corner(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK);
    }

    /// **SPI_SMEM_ECC_CS_HOLD_TIME** `[14:12]` (RW)
    ///
    /// SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
    #[inline(always)]
    pub fn spi_smem_ecc_cs_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_ECC_CS_HOLD_TIME_MASK)
            >> Self::SPI_SMEM_ECC_CS_HOLD_TIME_OFFSET
    }

    /// Write **SPI_SMEM_ECC_CS_HOLD_TIME** `[14:12]` (RW)
    ///
    /// SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
    #[inline(always)]
    pub fn write_spi_smem_ecc_cs_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_ECC_CS_HOLD_TIME_MASK)
                | ((value << Self::SPI_SMEM_ECC_CS_HOLD_TIME_OFFSET)
                    & Self::SPI_SMEM_ECC_CS_HOLD_TIME_MASK)
        });
    }

    /// **SPI_SMEM_CS_HOLD_TIME** `[11:7]` (RW)
    ///
    /// For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_CS_HOLD_TIME_MASK)
            >> Self::SPI_SMEM_CS_HOLD_TIME_OFFSET
    }

    /// Write **SPI_SMEM_CS_HOLD_TIME** `[11:7]` (RW)
    ///
    /// For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
    #[inline(always)]
    pub fn write_spi_smem_cs_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_CS_HOLD_TIME_MASK)
                | ((value << Self::SPI_SMEM_CS_HOLD_TIME_OFFSET) & Self::SPI_SMEM_CS_HOLD_TIME_MASK)
        });
    }

    /// **SPI_SMEM_CS_SETUP_TIME** `[6:2]` (RW)
    ///
    /// For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_CS_SETUP_TIME_MASK)
            >> Self::SPI_SMEM_CS_SETUP_TIME_OFFSET
    }

    /// Write **SPI_SMEM_CS_SETUP_TIME** `[6:2]` (RW)
    ///
    /// For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
    #[inline(always)]
    pub fn write_spi_smem_cs_setup_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_CS_SETUP_TIME_MASK)
                | ((value << Self::SPI_SMEM_CS_SETUP_TIME_OFFSET)
                    & Self::SPI_SMEM_CS_SETUP_TIME_MASK)
        });
    }

    /// **SPI_SMEM_CS_HOLD** `[1]` (RW)
    ///
    /// For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_smem_cs_hold(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_CS_HOLD_MASK) != 0
    }

    /// Set SPI_SMEM_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_cs_hold(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_CS_HOLD_MASK);
    }

    /// Clear SPI_SMEM_CS_HOLD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_cs_hold(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SMEM_CS_HOLD_MASK);
    }

    /// **SPI_SMEM_CS_SETUP** `[0]` (RW)
    ///
    /// For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_smem_cs_setup(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_CS_SETUP_MASK) != 0
    }

    /// Set SPI_SMEM_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_cs_setup(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SMEM_CS_SETUP_MASK);
    }

    /// Clear SPI_SMEM_CS_SETUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_cs_setup(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_CS_SETUP_MASK);
    }
}

/// SPI_SMEM_AC value type for batch read/modify/write operations
///
/// Use `spismemac.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemac.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemAcVal {
    bits: u32,
}

impl SpiSmemAcVal {
    /// Create from raw bits
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
        Self { bits: 0x8000b084 }
    }

    /// Read **SPI_SMEM_SPLIT_TRANS_EN** `[31:31]`
    ///
    /// Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub const fn spi_smem_split_trans_en(&self) -> bool {
        (self.bits & SpiSmemAc::SPI_SMEM_SPLIT_TRANS_EN_MASK) != 0
    }

    /// Read **SPI_SMEM_CS_HOLD_DELAY** `[30:25]`
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub const fn spi_smem_cs_hold_delay(&self) -> u32 {
        (self.bits & SpiSmemAc::SPI_SMEM_CS_HOLD_DELAY_MASK)
            >> SpiSmemAc::SPI_SMEM_CS_HOLD_DELAY_OFFSET
    }

    /// Read **SPI_SMEM_ECC_16TO18_BYTE_EN** `[16:16]`
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM.
    #[inline(always)]
    pub const fn spi_smem_ecc_16to18_byte_en(&self) -> bool {
        (self.bits & SpiSmemAc::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK) != 0
    }

    /// Read **SPI_SMEM_ECC_SKIP_PAGE_CORNER** `[15:15]`
    ///
    /// 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM.
    #[inline(always)]
    pub const fn spi_smem_ecc_skip_page_corner(&self) -> bool {
        (self.bits & SpiSmemAc::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK) != 0
    }

    /// Read **SPI_SMEM_ECC_CS_HOLD_TIME** `[14:12]`
    ///
    /// SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
    #[inline(always)]
    pub const fn spi_smem_ecc_cs_hold_time(&self) -> u32 {
        (self.bits & SpiSmemAc::SPI_SMEM_ECC_CS_HOLD_TIME_MASK)
            >> SpiSmemAc::SPI_SMEM_ECC_CS_HOLD_TIME_OFFSET
    }

    /// Read **SPI_SMEM_CS_HOLD_TIME** `[11:7]`
    ///
    /// For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
    #[inline(always)]
    pub const fn spi_smem_cs_hold_time(&self) -> u32 {
        (self.bits & SpiSmemAc::SPI_SMEM_CS_HOLD_TIME_MASK)
            >> SpiSmemAc::SPI_SMEM_CS_HOLD_TIME_OFFSET
    }

    /// Read **SPI_SMEM_CS_SETUP_TIME** `[6:2]`
    ///
    /// For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
    #[inline(always)]
    pub const fn spi_smem_cs_setup_time(&self) -> u32 {
        (self.bits & SpiSmemAc::SPI_SMEM_CS_SETUP_TIME_MASK)
            >> SpiSmemAc::SPI_SMEM_CS_SETUP_TIME_OFFSET
    }

    /// Read **SPI_SMEM_CS_HOLD** `[1:1]`
    ///
    /// For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_smem_cs_hold(&self) -> bool {
        (self.bits & SpiSmemAc::SPI_SMEM_CS_HOLD_MASK) != 0
    }

    /// Read **SPI_SMEM_CS_SETUP** `[0:0]`
    ///
    /// For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_smem_cs_setup(&self) -> bool {
        (self.bits & SpiSmemAc::SPI_SMEM_CS_SETUP_MASK) != 0
    }

    /// Set **SPI_SMEM_SPLIT_TRANS_EN** `[31:31]`
    ///
    /// Set this bit to enable SPI0 split one AXI accesses EXT_RAM transfer into two SPI transfers when one transfer will cross flash/EXT_RAM page corner, valid no matter whether there is an ECC region or not.
    #[inline(always)]
    pub const fn with_spi_smem_split_trans_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemAc::SPI_SMEM_SPLIT_TRANS_EN_MASK
            } else {
                self.bits & !SpiSmemAc::SPI_SMEM_SPLIT_TRANS_EN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_CS_HOLD_DELAY** `[30:25]`
    ///
    /// These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY[5:0] + 1) MSPI core clock cycles.
    #[inline(always)]
    pub const fn with_spi_smem_cs_hold_delay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemAc::SPI_SMEM_CS_HOLD_DELAY_MASK)
                | ((value << SpiSmemAc::SPI_SMEM_CS_HOLD_DELAY_OFFSET)
                    & SpiSmemAc::SPI_SMEM_CS_HOLD_DELAY_MASK),
        }
    }

    /// Set **SPI_SMEM_ECC_16TO18_BYTE_EN** `[16:16]`
    ///
    /// Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_ecc_16to18_byte_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemAc::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK
            } else {
                self.bits & !SpiSmemAc::SPI_SMEM_ECC_16TO18_BYTE_EN_MASK
            },
        }
    }

    /// Set **SPI_SMEM_ECC_SKIP_PAGE_CORNER** `[15:15]`
    ///
    /// 1: SPI0 skips page corner when accesses external RAM. 0: Not skip page corner when accesses external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_ecc_skip_page_corner(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemAc::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK
            } else {
                self.bits & !SpiSmemAc::SPI_SMEM_ECC_SKIP_PAGE_CORNER_MASK
            },
        }
    }

    /// Set **SPI_SMEM_ECC_CS_HOLD_TIME** `[14:12]`
    ///
    /// SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the SPI0 and SPI1 CS hold cycles in ECC mode when accessed external RAM.
    #[inline(always)]
    pub const fn with_spi_smem_ecc_cs_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemAc::SPI_SMEM_ECC_CS_HOLD_TIME_MASK)
                | ((value << SpiSmemAc::SPI_SMEM_ECC_CS_HOLD_TIME_OFFSET)
                    & SpiSmemAc::SPI_SMEM_ECC_CS_HOLD_TIME_MASK),
        }
    }

    /// Set **SPI_SMEM_CS_HOLD_TIME** `[11:7]`
    ///
    /// For SPI0 and SPI1, spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit.
    #[inline(always)]
    pub const fn with_spi_smem_cs_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemAc::SPI_SMEM_CS_HOLD_TIME_MASK)
                | ((value << SpiSmemAc::SPI_SMEM_CS_HOLD_TIME_OFFSET)
                    & SpiSmemAc::SPI_SMEM_CS_HOLD_TIME_MASK),
        }
    }

    /// Set **SPI_SMEM_CS_SETUP_TIME** `[6:2]`
    ///
    /// For spi0, (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit.
    #[inline(always)]
    pub const fn with_spi_smem_cs_setup_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemAc::SPI_SMEM_CS_SETUP_TIME_MASK)
                | ((value << SpiSmemAc::SPI_SMEM_CS_SETUP_TIME_OFFSET)
                    & SpiSmemAc::SPI_SMEM_CS_SETUP_TIME_MASK),
        }
    }

    /// Set **SPI_SMEM_CS_HOLD** `[1:1]`
    ///
    /// For SPI0 and SPI1, spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_smem_cs_hold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemAc::SPI_SMEM_CS_HOLD_MASK
            } else {
                self.bits & !SpiSmemAc::SPI_SMEM_CS_HOLD_MASK
            },
        }
    }

    /// Set **SPI_SMEM_CS_SETUP** `[0:0]`
    ///
    /// For SPI0 and SPI1, spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_smem_cs_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemAc::SPI_SMEM_CS_SETUP_MASK
            } else {
                self.bits & !SpiSmemAc::SPI_SMEM_CS_SETUP_MASK
            },
        }
    }
}

impl Default for SpiSmemAcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiSmemAc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemAcVal {
        SpiSmemAcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemAcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemAcVal) -> SpiSmemAcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DIN_HEX_MODE Register
///
/// MSPI 16x external RAM input timing delay mode control register
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **SPI_SMEM_DINS_HEX_MODE** `[26:24]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN15_MODE** `[23:21]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN14_MODE** `[20:18]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN13_MODE** `[17:15]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN12_MODE** `[14:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN11_MODE** `[11:9]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN10_MODE** `[8:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN09_MODE** `[5:3]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
///
/// - **SPI_SMEM_DIN08_MODE** `[2:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
#[repr(transparent)]
pub struct SpiSmemDinHexMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDinHexMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDinHexMode {
    // ======== Field Constants ========

    /// **SPI_SMEM_DINS_HEX_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SPI_SMEM_DINS_HEX_MODE_OFFSET: u32 = 24;
    pub const SPI_SMEM_DINS_HEX_MODE_MASK: u32 = 0x07000000;

    /// **SPI_SMEM_DIN15_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const SPI_SMEM_DIN15_MODE_OFFSET: u32 = 21;
    pub const SPI_SMEM_DIN15_MODE_MASK: u32 = 0x00e00000;

    /// **SPI_SMEM_DIN14_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SPI_SMEM_DIN14_MODE_OFFSET: u32 = 18;
    pub const SPI_SMEM_DIN14_MODE_MASK: u32 = 0x001c0000;

    /// **SPI_SMEM_DIN13_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const SPI_SMEM_DIN13_MODE_OFFSET: u32 = 15;
    pub const SPI_SMEM_DIN13_MODE_MASK: u32 = 0x00038000;

    /// **SPI_SMEM_DIN12_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SPI_SMEM_DIN12_MODE_OFFSET: u32 = 12;
    pub const SPI_SMEM_DIN12_MODE_MASK: u32 = 0x00007000;

    /// **SPI_SMEM_DIN11_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const SPI_SMEM_DIN11_MODE_OFFSET: u32 = 9;
    pub const SPI_SMEM_DIN11_MODE_MASK: u32 = 0x00000e00;

    /// **SPI_SMEM_DIN10_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SPI_SMEM_DIN10_MODE_OFFSET: u32 = 6;
    pub const SPI_SMEM_DIN10_MODE_MASK: u32 = 0x000001c0;

    /// **SPI_SMEM_DIN09_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const SPI_SMEM_DIN09_MODE_OFFSET: u32 = 3;
    pub const SPI_SMEM_DIN09_MODE_MASK: u32 = 0x00000038;

    /// **SPI_SMEM_DIN08_MODE**
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SPI_SMEM_DIN08_MODE_OFFSET: u32 = 0;
    pub const SPI_SMEM_DIN08_MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DINS_HEX_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dins_hex_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DINS_HEX_MODE_MASK)
            >> Self::SPI_SMEM_DINS_HEX_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DINS_HEX_MODE** `[26:24]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_dins_hex_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DINS_HEX_MODE_MASK)
                | ((value << Self::SPI_SMEM_DINS_HEX_MODE_OFFSET)
                    & Self::SPI_SMEM_DINS_HEX_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN15_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din15_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN15_MODE_MASK)
            >> Self::SPI_SMEM_DIN15_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN15_MODE** `[23:21]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din15_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN15_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN15_MODE_OFFSET) & Self::SPI_SMEM_DIN15_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN14_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din14_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN14_MODE_MASK)
            >> Self::SPI_SMEM_DIN14_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN14_MODE** `[20:18]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din14_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN14_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN14_MODE_OFFSET) & Self::SPI_SMEM_DIN14_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN13_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din13_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN13_MODE_MASK)
            >> Self::SPI_SMEM_DIN13_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN13_MODE** `[17:15]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din13_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN13_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN13_MODE_OFFSET) & Self::SPI_SMEM_DIN13_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN12_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din12_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN12_MODE_MASK)
            >> Self::SPI_SMEM_DIN12_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN12_MODE** `[14:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din12_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN12_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN12_MODE_OFFSET) & Self::SPI_SMEM_DIN12_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN11_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din11_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN11_MODE_MASK)
            >> Self::SPI_SMEM_DIN11_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN11_MODE** `[11:9]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din11_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN11_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN11_MODE_OFFSET) & Self::SPI_SMEM_DIN11_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN10_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din10_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN10_MODE_MASK)
            >> Self::SPI_SMEM_DIN10_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN10_MODE** `[8:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din10_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN10_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN10_MODE_OFFSET) & Self::SPI_SMEM_DIN10_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN09_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din09_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN09_MODE_MASK)
            >> Self::SPI_SMEM_DIN09_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN09_MODE** `[5:3]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din09_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN09_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN09_MODE_OFFSET) & Self::SPI_SMEM_DIN09_MODE_MASK)
        });
    }

    /// **SPI_SMEM_DIN08_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_din08_mode(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN08_MODE_MASK)
            >> Self::SPI_SMEM_DIN08_MODE_OFFSET
    }

    /// Write **SPI_SMEM_DIN08_MODE** `[2:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn write_spi_smem_din08_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN08_MODE_MASK)
                | ((value << Self::SPI_SMEM_DIN08_MODE_OFFSET) & Self::SPI_SMEM_DIN08_MODE_MASK)
        });
    }
}

/// SPI_SMEM_DIN_HEX_MODE value type for batch read/modify/write operations
///
/// Use `spismemdinhexmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdinhexmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDinHexModeVal {
    bits: u32,
}

impl SpiSmemDinHexModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DINS_HEX_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dins_hex_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DINS_HEX_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DINS_HEX_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN15_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din15_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN15_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN15_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN14_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din14_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN14_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN14_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN13_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din13_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN13_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN13_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN12_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din12_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN12_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN12_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN11_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din11_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN11_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN11_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN10_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din10_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN10_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN10_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN09_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din09_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN09_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN09_MODE_OFFSET
    }

    /// Read **SPI_SMEM_DIN08_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_din08_mode(&self) -> u32 {
        (self.bits & SpiSmemDinHexMode::SPI_SMEM_DIN08_MODE_MASK)
            >> SpiSmemDinHexMode::SPI_SMEM_DIN08_MODE_OFFSET
    }

    /// Set **SPI_SMEM_DINS_HEX_MODE** `[26:24]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dins_hex_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DINS_HEX_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DINS_HEX_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DINS_HEX_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN15_MODE** `[23:21]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din15_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN15_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN15_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN15_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN14_MODE** `[20:18]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din14_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN14_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN14_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN14_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN13_MODE** `[17:15]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din13_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN13_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN13_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN13_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN12_MODE** `[14:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din12_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN12_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN12_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN12_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN11_MODE** `[11:9]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din11_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN11_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN11_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN11_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN10_MODE** `[8:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din10_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN10_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN10_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN10_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN09_MODE** `[5:3]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din09_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN09_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN09_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN09_MODE_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN08_MODE** `[2:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_din08_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexMode::SPI_SMEM_DIN08_MODE_MASK)
                | ((value << SpiSmemDinHexMode::SPI_SMEM_DIN08_MODE_OFFSET)
                    & SpiSmemDinHexMode::SPI_SMEM_DIN08_MODE_MASK),
        }
    }
}

impl Default for SpiSmemDinHexModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDinHexMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDinHexModeVal {
        SpiSmemDinHexModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDinHexModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDinHexModeVal) -> SpiSmemDinHexModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DIN_HEX_NUM Register
///
/// MSPI 16x external RAM input timing delay number control register
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **SPI_SMEM_DINS_HEX_NUM** `[17:16]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN15_NUM** `[15:14]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN14_NUM** `[13:12]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN13_NUM** `[11:10]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN12_NUM** `[9:8]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN11_NUM** `[7:6]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN10_NUM** `[5:4]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN09_NUM** `[3:2]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
///
/// - **SPI_SMEM_DIN08_NUM** `[1:0]` (RW)
///
///   the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
#[repr(transparent)]
pub struct SpiSmemDinHexNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDinHexNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDinHexNum {
    // ======== Field Constants ========

    /// **SPI_SMEM_DINS_HEX_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SPI_SMEM_DINS_HEX_NUM_OFFSET: u32 = 16;
    pub const SPI_SMEM_DINS_HEX_NUM_MASK: u32 = 0x00030000;

    /// **SPI_SMEM_DIN15_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SPI_SMEM_DIN15_NUM_OFFSET: u32 = 14;
    pub const SPI_SMEM_DIN15_NUM_MASK: u32 = 0x0000c000;

    /// **SPI_SMEM_DIN14_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const SPI_SMEM_DIN14_NUM_OFFSET: u32 = 12;
    pub const SPI_SMEM_DIN14_NUM_MASK: u32 = 0x00003000;

    /// **SPI_SMEM_DIN13_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const SPI_SMEM_DIN13_NUM_OFFSET: u32 = 10;
    pub const SPI_SMEM_DIN13_NUM_MASK: u32 = 0x00000c00;

    /// **SPI_SMEM_DIN12_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPI_SMEM_DIN12_NUM_OFFSET: u32 = 8;
    pub const SPI_SMEM_DIN12_NUM_MASK: u32 = 0x00000300;

    /// **SPI_SMEM_DIN11_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SPI_SMEM_DIN11_NUM_OFFSET: u32 = 6;
    pub const SPI_SMEM_DIN11_NUM_MASK: u32 = 0x000000c0;

    /// **SPI_SMEM_DIN10_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SPI_SMEM_DIN10_NUM_OFFSET: u32 = 4;
    pub const SPI_SMEM_DIN10_NUM_MASK: u32 = 0x00000030;

    /// **SPI_SMEM_DIN09_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SPI_SMEM_DIN09_NUM_OFFSET: u32 = 2;
    pub const SPI_SMEM_DIN09_NUM_MASK: u32 = 0x0000000c;

    /// **SPI_SMEM_DIN08_NUM**
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_SMEM_DIN08_NUM_OFFSET: u32 = 0;
    pub const SPI_SMEM_DIN08_NUM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DINS_HEX_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_dins_hex_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DINS_HEX_NUM_MASK)
            >> Self::SPI_SMEM_DINS_HEX_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DINS_HEX_NUM** `[17:16]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_dins_hex_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DINS_HEX_NUM_MASK)
                | ((value << Self::SPI_SMEM_DINS_HEX_NUM_OFFSET) & Self::SPI_SMEM_DINS_HEX_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN15_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din15_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN15_NUM_MASK) >> Self::SPI_SMEM_DIN15_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN15_NUM** `[15:14]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din15_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN15_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN15_NUM_OFFSET) & Self::SPI_SMEM_DIN15_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN14_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din14_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN14_NUM_MASK) >> Self::SPI_SMEM_DIN14_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN14_NUM** `[13:12]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din14_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN14_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN14_NUM_OFFSET) & Self::SPI_SMEM_DIN14_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN13_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din13_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN13_NUM_MASK) >> Self::SPI_SMEM_DIN13_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN13_NUM** `[11:10]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din13_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN13_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN13_NUM_OFFSET) & Self::SPI_SMEM_DIN13_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN12_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din12_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN12_NUM_MASK) >> Self::SPI_SMEM_DIN12_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN12_NUM** `[9:8]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din12_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN12_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN12_NUM_OFFSET) & Self::SPI_SMEM_DIN12_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN11_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din11_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN11_NUM_MASK) >> Self::SPI_SMEM_DIN11_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN11_NUM** `[7:6]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din11_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN11_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN11_NUM_OFFSET) & Self::SPI_SMEM_DIN11_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN10_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din10_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN10_NUM_MASK) >> Self::SPI_SMEM_DIN10_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN10_NUM** `[5:4]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din10_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN10_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN10_NUM_OFFSET) & Self::SPI_SMEM_DIN10_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN09_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din09_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN09_NUM_MASK) >> Self::SPI_SMEM_DIN09_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN09_NUM** `[3:2]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din09_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN09_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN09_NUM_OFFSET) & Self::SPI_SMEM_DIN09_NUM_MASK)
        });
    }

    /// **SPI_SMEM_DIN08_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn spi_smem_din08_num(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_SMEM_DIN08_NUM_MASK) >> Self::SPI_SMEM_DIN08_NUM_OFFSET
    }

    /// Write **SPI_SMEM_DIN08_NUM** `[1:0]` (RW)
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn write_spi_smem_din08_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_SMEM_DIN08_NUM_MASK)
                | ((value << Self::SPI_SMEM_DIN08_NUM_OFFSET) & Self::SPI_SMEM_DIN08_NUM_MASK)
        });
    }
}

/// SPI_SMEM_DIN_HEX_NUM value type for batch read/modify/write operations
///
/// Use `spismemdinhexnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdinhexnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDinHexNumVal {
    bits: u32,
}

impl SpiSmemDinHexNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DINS_HEX_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_dins_hex_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DINS_HEX_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DINS_HEX_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN15_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din15_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN15_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN15_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN14_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din14_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN14_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN14_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN13_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din13_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN13_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN13_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN12_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din12_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN12_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN12_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN11_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din11_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN11_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN11_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN10_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din10_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN10_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN10_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN09_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din09_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN09_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN09_NUM_OFFSET
    }

    /// Read **SPI_SMEM_DIN08_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn spi_smem_din08_num(&self) -> u32 {
        (self.bits & SpiSmemDinHexNum::SPI_SMEM_DIN08_NUM_MASK)
            >> SpiSmemDinHexNum::SPI_SMEM_DIN08_NUM_OFFSET
    }

    /// Set **SPI_SMEM_DINS_HEX_NUM** `[17:16]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_dins_hex_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DINS_HEX_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DINS_HEX_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DINS_HEX_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN15_NUM** `[15:14]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din15_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN15_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN15_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN15_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN14_NUM** `[13:12]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din14_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN14_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN14_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN14_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN13_NUM** `[11:10]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din13_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN13_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN13_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN13_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN12_NUM** `[9:8]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din12_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN12_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN12_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN12_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN11_NUM** `[7:6]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din11_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN11_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN11_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN11_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN10_NUM** `[5:4]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din10_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN10_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN10_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN10_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN09_NUM** `[3:2]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din09_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN09_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN09_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN09_NUM_MASK),
        }
    }

    /// Set **SPI_SMEM_DIN08_NUM** `[1:0]`
    ///
    /// the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub const fn with_spi_smem_din08_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiSmemDinHexNum::SPI_SMEM_DIN08_NUM_MASK)
                | ((value << SpiSmemDinHexNum::SPI_SMEM_DIN08_NUM_OFFSET)
                    & SpiSmemDinHexNum::SPI_SMEM_DIN08_NUM_MASK),
        }
    }
}

impl Default for SpiSmemDinHexNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDinHexNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDinHexNumVal {
        SpiSmemDinHexNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDinHexNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDinHexNumVal) -> SpiSmemDinHexNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_SMEM_DOUT_HEX_MODE Register
///
/// MSPI 16x external RAM output timing adjustment control register
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **SPI_SMEM_DOUTS_HEX_MODE** `[8]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT15_MODE** `[7]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT14_MODE** `[6]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT13_MODE** `[5]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT12_MODE** `[4]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT11_MODE** `[3]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT10_MODE** `[2]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT09_MODE** `[1]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
///
/// - **SPI_SMEM_DOUT08_MODE** `[0]` (RW)
///
///   the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
#[repr(transparent)]
pub struct SpiSmemDoutHexMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiSmemDoutHexMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiSmemDoutHexMode {
    // ======== Field Constants ========

    /// **SPI_SMEM_DOUTS_HEX_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_SMEM_DOUTS_HEX_MODE_OFFSET: u32 = 8;
    pub const SPI_SMEM_DOUTS_HEX_MODE_MASK: u32 = 0x00000100;
    pub const SPI_SMEM_DOUTS_HEX_MODE: u32 = 1 << 8;

    /// **SPI_SMEM_DOUT15_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_SMEM_DOUT15_MODE_OFFSET: u32 = 7;
    pub const SPI_SMEM_DOUT15_MODE_MASK: u32 = 0x00000080;
    pub const SPI_SMEM_DOUT15_MODE: u32 = 1 << 7;

    /// **SPI_SMEM_DOUT14_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_SMEM_DOUT14_MODE_OFFSET: u32 = 6;
    pub const SPI_SMEM_DOUT14_MODE_MASK: u32 = 0x00000040;
    pub const SPI_SMEM_DOUT14_MODE: u32 = 1 << 6;

    /// **SPI_SMEM_DOUT13_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_SMEM_DOUT13_MODE_OFFSET: u32 = 5;
    pub const SPI_SMEM_DOUT13_MODE_MASK: u32 = 0x00000020;
    pub const SPI_SMEM_DOUT13_MODE: u32 = 1 << 5;

    /// **SPI_SMEM_DOUT12_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_SMEM_DOUT12_MODE_OFFSET: u32 = 4;
    pub const SPI_SMEM_DOUT12_MODE_MASK: u32 = 0x00000010;
    pub const SPI_SMEM_DOUT12_MODE: u32 = 1 << 4;

    /// **SPI_SMEM_DOUT11_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_SMEM_DOUT11_MODE_OFFSET: u32 = 3;
    pub const SPI_SMEM_DOUT11_MODE_MASK: u32 = 0x00000008;
    pub const SPI_SMEM_DOUT11_MODE: u32 = 1 << 3;

    /// **SPI_SMEM_DOUT10_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_SMEM_DOUT10_MODE_OFFSET: u32 = 2;
    pub const SPI_SMEM_DOUT10_MODE_MASK: u32 = 0x00000004;
    pub const SPI_SMEM_DOUT10_MODE: u32 = 1 << 2;

    /// **SPI_SMEM_DOUT09_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_SMEM_DOUT09_MODE_OFFSET: u32 = 1;
    pub const SPI_SMEM_DOUT09_MODE_MASK: u32 = 0x00000002;
    pub const SPI_SMEM_DOUT09_MODE: u32 = 1 << 1;

    /// **SPI_SMEM_DOUT08_MODE**
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_SMEM_DOUT08_MODE_OFFSET: u32 = 0;
    pub const SPI_SMEM_DOUT08_MODE_MASK: u32 = 0x00000001;
    pub const SPI_SMEM_DOUT08_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_SMEM_DOUTS_HEX_MODE** `[8]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_douts_hex_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUTS_HEX_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUTS_HEX_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_douts_hex_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUTS_HEX_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUTS_HEX_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_douts_hex_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUTS_HEX_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT15_MODE** `[7]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout15_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT15_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT15_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout15_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT15_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT15_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout15_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT15_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT14_MODE** `[6]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout14_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT14_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT14_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout14_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT14_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT14_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout14_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT14_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT13_MODE** `[5]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout13_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT13_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT13_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout13_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT13_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT13_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout13_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT13_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT12_MODE** `[4]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout12_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT12_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT12_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout12_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT12_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT12_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout12_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT12_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT11_MODE** `[3]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout11_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT11_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT11_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout11_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT11_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT11_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout11_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT11_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT10_MODE** `[2]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout10_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT10_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT10_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout10_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT10_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT10_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout10_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT10_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT09_MODE** `[1]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout09_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT09_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT09_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout09_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT09_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT09_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout09_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT09_MODE_MASK);
    }

    /// **SPI_SMEM_DOUT08_MODE** `[0]` (RW)
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub fn spi_smem_dout08_mode(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SMEM_DOUT08_MODE_MASK) != 0
    }

    /// Set SPI_SMEM_DOUT08_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_smem_dout08_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_SMEM_DOUT08_MODE_MASK);
    }

    /// Clear SPI_SMEM_DOUT08_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_smem_dout08_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_SMEM_DOUT08_MODE_MASK);
    }
}

/// SPI_SMEM_DOUT_HEX_MODE value type for batch read/modify/write operations
///
/// Use `spismemdouthexmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spismemdouthexmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiSmemDoutHexModeVal {
    bits: u32,
}

impl SpiSmemDoutHexModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SMEM_DOUTS_HEX_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_douts_hex_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUTS_HEX_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT15_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout15_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT15_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT14_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout14_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT14_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT13_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout13_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT13_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT12_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout12_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT12_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT11_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout11_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT11_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT10_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout10_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT10_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT09_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout09_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT09_MODE_MASK) != 0
    }

    /// Read **SPI_SMEM_DOUT08_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn spi_smem_dout08_mode(&self) -> bool {
        (self.bits & SpiSmemDoutHexMode::SPI_SMEM_DOUT08_MODE_MASK) != 0
    }

    /// Set **SPI_SMEM_DOUTS_HEX_MODE** `[8:8]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_douts_hex_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUTS_HEX_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUTS_HEX_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT15_MODE** `[7:7]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout15_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT15_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT15_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT14_MODE** `[6:6]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout14_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT14_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT14_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT13_MODE** `[5:5]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout13_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT13_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT13_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT12_MODE** `[4:4]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout12_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT12_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT12_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT11_MODE** `[3:3]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout11_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT11_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT11_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT10_MODE** `[2:2]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout10_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT10_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT10_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT09_MODE** `[1:1]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout09_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT09_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT09_MODE_MASK
            },
        }
    }

    /// Set **SPI_SMEM_DOUT08_MODE** `[0:0]`
    ///
    /// the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge
    #[inline(always)]
    pub const fn with_spi_smem_dout08_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiSmemDoutHexMode::SPI_SMEM_DOUT08_MODE_MASK
            } else {
                self.bits & !SpiSmemDoutHexMode::SPI_SMEM_DOUT08_MODE_MASK
            },
        }
    }
}

impl Default for SpiSmemDoutHexModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiSmemDoutHexMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiSmemDoutHexModeVal {
        SpiSmemDoutHexModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiSmemDoutHexModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiSmemDoutHexModeVal) -> SpiSmemDoutHexModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CLOCK_GATE Register
///
/// SPI0 clock gate register
///
/// Address offset: 0x0200
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **SPI_CLK_EN** `[0]` (RW)
///
///   Register clock gate enable signal. 1: Enable. 0: Disable.
#[repr(transparent)]
pub struct SpiMemClockGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemClockGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemClockGate {
    // ======== Field Constants ========

    /// **SPI_CLK_EN**
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_CLK_EN_OFFSET: u32 = 0;
    pub const SPI_CLK_EN_MASK: u32 = 0x00000001;
    pub const SPI_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_CLK_EN** `[0]` (RW)
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
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

/// SPI_MEM_CLOCK_GATE value type for batch read/modify/write operations
///
/// Use `spimemclockgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemclockgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemClockGateVal {
    bits: u32,
}

impl SpiMemClockGateVal {
    /// Create from raw bits
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

    /// Read **SPI_CLK_EN** `[0:0]`
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    #[inline(always)]
    pub const fn spi_clk_en(&self) -> bool {
        (self.bits & SpiMemClockGate::SPI_CLK_EN_MASK) != 0
    }

    /// Set **SPI_CLK_EN** `[0:0]`
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    #[inline(always)]
    pub const fn with_spi_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemClockGate::SPI_CLK_EN_MASK
            } else {
                self.bits & !SpiMemClockGate::SPI_CLK_EN_MASK
            },
        }
    }
}

impl Default for SpiMemClockGateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemClockGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemClockGateVal {
        SpiMemClockGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemClockGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemClockGateVal) -> SpiMemClockGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_XTS_PLAIN_BASE Register
///
/// The base address of the memory that stores plaintext in Manual Encryption
///
/// Address offset: 0x0300
///
/// ## Fields
///
/// - **SPI_XTS_PLAIN** `[31:0]` (RW)
///
///   This field is only used to generate include file in c case. This field is useless. Please do not use this field.
#[repr(transparent)]
pub struct SpiMemXtsPlainBase {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsPlainBase {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsPlainBase {
    // ======== Field Constants ========

    /// **SPI_XTS_PLAIN**
    ///
    /// This field is only used to generate include file in c case. This field is useless. Please do not use this field.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_XTS_PLAIN_OFFSET: u32 = 0;
    pub const SPI_XTS_PLAIN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_XTS_PLAIN** `[31:0]` (RW)
    ///
    /// This field is only used to generate include file in c case. This field is useless. Please do not use this field.
    #[inline(always)]
    pub fn spi_xts_plain(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_XTS_PLAIN_MASK) >> Self::SPI_XTS_PLAIN_OFFSET
    }

    /// Write **SPI_XTS_PLAIN** `[31:0]` (RW)
    ///
    /// This field is only used to generate include file in c case. This field is useless. Please do not use this field.
    #[inline(always)]
    pub fn write_spi_xts_plain(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_XTS_PLAIN_MASK)
                | ((value << Self::SPI_XTS_PLAIN_OFFSET) & Self::SPI_XTS_PLAIN_MASK)
        });
    }
}

/// SPI_MEM_XTS_PLAIN_BASE value type for batch read/modify/write operations
///
/// Use `spimemxtsplainbase.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsplainbase.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsPlainBaseVal {
    bits: u32,
}

impl SpiMemXtsPlainBaseVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_XTS_PLAIN** `[31:0]`
    ///
    /// This field is only used to generate include file in c case. This field is useless. Please do not use this field.
    #[inline(always)]
    pub const fn spi_xts_plain(&self) -> u32 {
        (self.bits & SpiMemXtsPlainBase::SPI_XTS_PLAIN_MASK)
            >> SpiMemXtsPlainBase::SPI_XTS_PLAIN_OFFSET
    }

    /// Set **SPI_XTS_PLAIN** `[31:0]`
    ///
    /// This field is only used to generate include file in c case. This field is useless. Please do not use this field.
    #[inline(always)]
    pub const fn with_spi_xts_plain(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemXtsPlainBase::SPI_XTS_PLAIN_MASK)
                | ((value << SpiMemXtsPlainBase::SPI_XTS_PLAIN_OFFSET)
                    & SpiMemXtsPlainBase::SPI_XTS_PLAIN_MASK),
        }
    }
}

impl Default for SpiMemXtsPlainBaseVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsPlainBase {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsPlainBaseVal {
        SpiMemXtsPlainBaseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsPlainBaseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemXtsPlainBaseVal) -> SpiMemXtsPlainBaseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_XTS_LINESIZE Register
///
/// Manual Encryption Line-Size register
///
/// Address offset: 0x0340
///
/// ## Fields
///
/// - **SPI_XTS_LINESIZE** `[1:0]` (RW)
///
///   This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
#[repr(transparent)]
pub struct SpiMemXtsLinesize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsLinesize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsLinesize {
    // ======== Field Constants ========

    /// **SPI_XTS_LINESIZE**
    ///
    /// This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_XTS_LINESIZE_OFFSET: u32 = 0;
    pub const SPI_XTS_LINESIZE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_XTS_LINESIZE** `[1:0]` (RW)
    ///
    /// This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
    #[inline(always)]
    pub fn spi_xts_linesize(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_XTS_LINESIZE_MASK) >> Self::SPI_XTS_LINESIZE_OFFSET
    }

    /// Write **SPI_XTS_LINESIZE** `[1:0]` (RW)
    ///
    /// This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
    #[inline(always)]
    pub fn write_spi_xts_linesize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_XTS_LINESIZE_MASK)
                | ((value << Self::SPI_XTS_LINESIZE_OFFSET) & Self::SPI_XTS_LINESIZE_MASK)
        });
    }
}

/// SPI_MEM_XTS_LINESIZE value type for batch read/modify/write operations
///
/// Use `spimemxtslinesize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtslinesize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsLinesizeVal {
    bits: u32,
}

impl SpiMemXtsLinesizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_XTS_LINESIZE** `[1:0]`
    ///
    /// This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
    #[inline(always)]
    pub const fn spi_xts_linesize(&self) -> u32 {
        (self.bits & SpiMemXtsLinesize::SPI_XTS_LINESIZE_MASK)
            >> SpiMemXtsLinesize::SPI_XTS_LINESIZE_OFFSET
    }

    /// Set **SPI_XTS_LINESIZE** `[1:0]`
    ///
    /// This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved.
    #[inline(always)]
    pub const fn with_spi_xts_linesize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemXtsLinesize::SPI_XTS_LINESIZE_MASK)
                | ((value << SpiMemXtsLinesize::SPI_XTS_LINESIZE_OFFSET)
                    & SpiMemXtsLinesize::SPI_XTS_LINESIZE_MASK),
        }
    }
}

impl Default for SpiMemXtsLinesizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsLinesize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsLinesizeVal {
        SpiMemXtsLinesizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsLinesizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemXtsLinesizeVal) -> SpiMemXtsLinesizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_XTS_DESTINATION Register
///
/// Manual Encryption destination register
///
/// Address offset: 0x0344
///
/// ## Fields
///
/// - **SPI_XTS_DESTINATION** `[0]` (RW)
///
///   This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used.
#[repr(transparent)]
pub struct SpiMemXtsDestination {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsDestination {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsDestination {
    // ======== Field Constants ========

    /// **SPI_XTS_DESTINATION**
    ///
    /// This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_XTS_DESTINATION_OFFSET: u32 = 0;
    pub const SPI_XTS_DESTINATION_MASK: u32 = 0x00000001;
    pub const SPI_XTS_DESTINATION: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_XTS_DESTINATION** `[0]` (RW)
    ///
    /// This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used.
    #[inline(always)]
    pub fn spi_xts_destination(&self) -> bool {
        (self.value.read_bits() & Self::SPI_XTS_DESTINATION_MASK) != 0
    }

    /// Set SPI_XTS_DESTINATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_xts_destination(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_XTS_DESTINATION_MASK);
    }

    /// Clear SPI_XTS_DESTINATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_xts_destination(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_XTS_DESTINATION_MASK);
    }
}

/// SPI_MEM_XTS_DESTINATION value type for batch read/modify/write operations
///
/// Use `spimemxtsdestination.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsdestination.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsDestinationVal {
    bits: u32,
}

impl SpiMemXtsDestinationVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_XTS_DESTINATION** `[0:0]`
    ///
    /// This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used.
    #[inline(always)]
    pub const fn spi_xts_destination(&self) -> bool {
        (self.bits & SpiMemXtsDestination::SPI_XTS_DESTINATION_MASK) != 0
    }

    /// Set **SPI_XTS_DESTINATION** `[0:0]`
    ///
    /// This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used.
    #[inline(always)]
    pub const fn with_spi_xts_destination(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemXtsDestination::SPI_XTS_DESTINATION_MASK
            } else {
                self.bits & !SpiMemXtsDestination::SPI_XTS_DESTINATION_MASK
            },
        }
    }
}

impl Default for SpiMemXtsDestinationVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsDestination {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsDestinationVal {
        SpiMemXtsDestinationVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsDestinationVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemXtsDestinationVal) -> SpiMemXtsDestinationVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_XTS_PHYSICAL_ADDRESS Register
///
/// Manual Encryption physical address register
///
/// Address offset: 0x0348
///
/// ## Fields
///
/// - **SPI_XTS_PHYSICAL_ADDRESS** `[25:0]` (RW)
///
///   This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
#[repr(transparent)]
pub struct SpiMemXtsPhysicalAddress {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsPhysicalAddress {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsPhysicalAddress {
    // ======== Field Constants ========

    /// **SPI_XTS_PHYSICAL_ADDRESS**
    ///
    /// This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const SPI_XTS_PHYSICAL_ADDRESS_OFFSET: u32 = 0;
    pub const SPI_XTS_PHYSICAL_ADDRESS_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **SPI_XTS_PHYSICAL_ADDRESS** `[25:0]` (RW)
    ///
    /// This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    pub fn spi_xts_physical_address(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_XTS_PHYSICAL_ADDRESS_MASK)
            >> Self::SPI_XTS_PHYSICAL_ADDRESS_OFFSET
    }

    /// Write **SPI_XTS_PHYSICAL_ADDRESS** `[25:0]` (RW)
    ///
    /// This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    pub fn write_spi_xts_physical_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_XTS_PHYSICAL_ADDRESS_MASK)
                | ((value << Self::SPI_XTS_PHYSICAL_ADDRESS_OFFSET)
                    & Self::SPI_XTS_PHYSICAL_ADDRESS_MASK)
        });
    }
}

/// SPI_MEM_XTS_PHYSICAL_ADDRESS value type for batch read/modify/write operations
///
/// Use `spimemxtsphysicaladdress.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsphysicaladdress.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsPhysicalAddressVal {
    bits: u32,
}

impl SpiMemXtsPhysicalAddressVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_XTS_PHYSICAL_ADDRESS** `[25:0]`
    ///
    /// This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    pub const fn spi_xts_physical_address(&self) -> u32 {
        (self.bits & SpiMemXtsPhysicalAddress::SPI_XTS_PHYSICAL_ADDRESS_MASK)
            >> SpiMemXtsPhysicalAddress::SPI_XTS_PHYSICAL_ADDRESS_OFFSET
    }

    /// Set **SPI_XTS_PHYSICAL_ADDRESS** `[25:0]`
    ///
    /// This bits stores the physical-address parameter which will be used in manual encryption calculation. This value should aligned with byte number decided by line-size parameter.
    #[inline(always)]
    pub const fn with_spi_xts_physical_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemXtsPhysicalAddress::SPI_XTS_PHYSICAL_ADDRESS_MASK)
                | ((value << SpiMemXtsPhysicalAddress::SPI_XTS_PHYSICAL_ADDRESS_OFFSET)
                    & SpiMemXtsPhysicalAddress::SPI_XTS_PHYSICAL_ADDRESS_MASK),
        }
    }
}

impl Default for SpiMemXtsPhysicalAddressVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsPhysicalAddress {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsPhysicalAddressVal {
        SpiMemXtsPhysicalAddressVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsPhysicalAddressVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemXtsPhysicalAddressVal) -> SpiMemXtsPhysicalAddressVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_XTS_TRIGGER Register
///
/// Manual Encryption physical address register
///
/// Address offset: 0x034c
///
/// ## Fields
///
/// - **SPI_XTS_TRIGGER** `[0]` (WO)
///
///   Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
#[repr(transparent)]
pub struct SpiMemXtsTrigger {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsTrigger {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsTrigger {
    // ======== Field Constants ========

    /// **SPI_XTS_TRIGGER**
    ///
    /// Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_XTS_TRIGGER_OFFSET: u32 = 0;
    pub const SPI_XTS_TRIGGER_MASK: u32 = 0x00000001;
    pub const SPI_XTS_TRIGGER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_XTS_TRIGGER** (WO)
    ///
    /// Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
    #[inline(always)]
    pub fn trigger_spi_xts_trigger(&self) {
        self.value.write_bits(Self::SPI_XTS_TRIGGER_MASK);
    }
}

/// SPI_MEM_XTS_TRIGGER value type for batch read/modify/write operations
///
/// Use `spimemxtstrigger.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtstrigger.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsTriggerVal {
    bits: u32,
}

impl SpiMemXtsTriggerVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_XTS_TRIGGER** `[0:0]`
    ///
    /// Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2.
    #[inline(always)]
    pub const fn with_spi_xts_trigger(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemXtsTrigger::SPI_XTS_TRIGGER_MASK
            } else {
                self.bits & !SpiMemXtsTrigger::SPI_XTS_TRIGGER_MASK
            },
        }
    }
}

impl Default for SpiMemXtsTriggerVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsTrigger {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsTriggerVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_MEM_XTS_RELEASE Register
///
/// Manual Encryption physical address register
///
/// Address offset: 0x0350
///
/// ## Fields
///
/// - **SPI_XTS_RELEASE** `[0]` (WO)
///
///   Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3.
#[repr(transparent)]
pub struct SpiMemXtsRelease {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsRelease {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsRelease {
    // ======== Field Constants ========

    /// **SPI_XTS_RELEASE**
    ///
    /// Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_XTS_RELEASE_OFFSET: u32 = 0;
    pub const SPI_XTS_RELEASE_MASK: u32 = 0x00000001;
    pub const SPI_XTS_RELEASE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_XTS_RELEASE** (WO)
    ///
    /// Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3.
    #[inline(always)]
    pub fn trigger_spi_xts_release(&self) {
        self.value.write_bits(Self::SPI_XTS_RELEASE_MASK);
    }
}

/// SPI_MEM_XTS_RELEASE value type for batch read/modify/write operations
///
/// Use `spimemxtsrelease.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsrelease.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsReleaseVal {
    bits: u32,
}

impl SpiMemXtsReleaseVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_XTS_RELEASE** `[0:0]`
    ///
    /// Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3.
    #[inline(always)]
    pub const fn with_spi_xts_release(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemXtsRelease::SPI_XTS_RELEASE_MASK
            } else {
                self.bits & !SpiMemXtsRelease::SPI_XTS_RELEASE_MASK
            },
        }
    }
}

impl Default for SpiMemXtsReleaseVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsRelease {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsReleaseVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_MEM_XTS_DESTROY Register
///
/// Manual Encryption physical address register
///
/// Address offset: 0x0354
///
/// ## Fields
///
/// - **SPI_XTS_DESTROY** `[0]` (WO)
///
///   Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0.
#[repr(transparent)]
pub struct SpiMemXtsDestroy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsDestroy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsDestroy {
    // ======== Field Constants ========

    /// **SPI_XTS_DESTROY**
    ///
    /// Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_XTS_DESTROY_OFFSET: u32 = 0;
    pub const SPI_XTS_DESTROY_MASK: u32 = 0x00000001;
    pub const SPI_XTS_DESTROY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_XTS_DESTROY** (WO)
    ///
    /// Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0.
    #[inline(always)]
    pub fn trigger_spi_xts_destroy(&self) {
        self.value.write_bits(Self::SPI_XTS_DESTROY_MASK);
    }
}

/// SPI_MEM_XTS_DESTROY value type for batch read/modify/write operations
///
/// Use `spimemxtsdestroy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsdestroy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsDestroyVal {
    bits: u32,
}

impl SpiMemXtsDestroyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SPI_XTS_DESTROY** `[0:0]`
    ///
    /// Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0.
    #[inline(always)]
    pub const fn with_spi_xts_destroy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemXtsDestroy::SPI_XTS_DESTROY_MASK
            } else {
                self.bits & !SpiMemXtsDestroy::SPI_XTS_DESTROY_MASK
            },
        }
    }
}

impl Default for SpiMemXtsDestroyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsDestroy {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsDestroyVal) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_MEM_XTS_STATE Register
///
/// Manual Encryption physical address register
///
/// Address offset: 0x0358
///
/// ## Fields
///
/// - **SPI_XTS_STATE** `[1:0]` (RO)
///
///   This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi.
#[repr(transparent)]
pub struct SpiMemXtsState {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsState {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsState {
    // ======== Field Constants ========

    /// **SPI_XTS_STATE**
    ///
    /// This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_XTS_STATE_OFFSET: u32 = 0;
    pub const SPI_XTS_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_XTS_STATE** `[1:0]` (RO)
    ///
    /// This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi.
    #[inline(always)]
    pub fn spi_xts_state(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_XTS_STATE_MASK) >> Self::SPI_XTS_STATE_OFFSET
    }
}

/// SPI_MEM_XTS_STATE value type for batch read/modify/write operations
///
/// Use `spimemxtsstate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsstate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsStateVal {
    bits: u32,
}

impl SpiMemXtsStateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_XTS_STATE** `[1:0]`
    ///
    /// This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi.
    #[inline(always)]
    pub const fn spi_xts_state(&self) -> u32 {
        (self.bits & SpiMemXtsState::SPI_XTS_STATE_MASK) >> SpiMemXtsState::SPI_XTS_STATE_OFFSET
    }
}

impl Default for SpiMemXtsStateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemXtsState {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsStateVal {
        SpiMemXtsStateVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_XTS_DATE Register
///
/// Manual Encryption version register
///
/// Address offset: 0x035c
/// Reset value: 0x20201010
///
/// ## Fields
///
/// - **SPI_XTS_DATE** `[29:0]` (RW)
///
///   This bits stores the last modified-time of manual encryption feature.
#[repr(transparent)]
pub struct SpiMemXtsDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemXtsDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemXtsDate {
    // ======== Field Constants ========

    /// **SPI_XTS_DATE**
    ///
    /// This bits stores the last modified-time of manual encryption feature.
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const SPI_XTS_DATE_OFFSET: u32 = 0;
    pub const SPI_XTS_DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **SPI_XTS_DATE** `[29:0]` (RW)
    ///
    /// This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    pub fn spi_xts_date(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_XTS_DATE_MASK) >> Self::SPI_XTS_DATE_OFFSET
    }

    /// Write **SPI_XTS_DATE** `[29:0]` (RW)
    ///
    /// This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    pub fn write_spi_xts_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_XTS_DATE_MASK)
                | ((value << Self::SPI_XTS_DATE_OFFSET) & Self::SPI_XTS_DATE_MASK)
        });
    }
}

/// SPI_MEM_XTS_DATE value type for batch read/modify/write operations
///
/// Use `spimemxtsdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemxtsdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemXtsDateVal {
    bits: u32,
}

impl SpiMemXtsDateVal {
    /// Create from raw bits
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
        Self { bits: 0x20201010 }
    }

    /// Read **SPI_XTS_DATE** `[29:0]`
    ///
    /// This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    pub const fn spi_xts_date(&self) -> u32 {
        (self.bits & SpiMemXtsDate::SPI_XTS_DATE_MASK) >> SpiMemXtsDate::SPI_XTS_DATE_OFFSET
    }

    /// Set **SPI_XTS_DATE** `[29:0]`
    ///
    /// This bits stores the last modified-time of manual encryption feature.
    #[inline(always)]
    pub const fn with_spi_xts_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemXtsDate::SPI_XTS_DATE_MASK)
                | ((value << SpiMemXtsDate::SPI_XTS_DATE_OFFSET)
                    & SpiMemXtsDate::SPI_XTS_DATE_MASK),
        }
    }
}

impl Default for SpiMemXtsDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemXtsDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemXtsDateVal {
        SpiMemXtsDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemXtsDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemXtsDateVal) -> SpiMemXtsDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_MMU_ITEM_CONTENT Register
///
/// MSPI-MMU item content register
///
/// Address offset: 0x037c
/// Reset value: 0x0000037c
///
/// ## Fields
///
/// - **SPI_MMU_ITEM_CONTENT** `[31:0]` (RW)
///
///   MSPI-MMU item content
#[repr(transparent)]
pub struct SpiMemMmuItemContent {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMmuItemContent {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMmuItemContent {
    // ======== Field Constants ========

    /// **SPI_MMU_ITEM_CONTENT**
    ///
    /// MSPI-MMU item content
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MMU_ITEM_CONTENT_OFFSET: u32 = 0;
    pub const SPI_MMU_ITEM_CONTENT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MMU_ITEM_CONTENT** `[31:0]` (RW)
    ///
    /// MSPI-MMU item content
    #[inline(always)]
    pub fn spi_mmu_item_content(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MMU_ITEM_CONTENT_MASK)
            >> Self::SPI_MMU_ITEM_CONTENT_OFFSET
    }

    /// Write **SPI_MMU_ITEM_CONTENT** `[31:0]` (RW)
    ///
    /// MSPI-MMU item content
    #[inline(always)]
    pub fn write_spi_mmu_item_content(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MMU_ITEM_CONTENT_MASK)
                | ((value << Self::SPI_MMU_ITEM_CONTENT_OFFSET) & Self::SPI_MMU_ITEM_CONTENT_MASK)
        });
    }
}

/// SPI_MEM_MMU_ITEM_CONTENT value type for batch read/modify/write operations
///
/// Use `spimemmmuitemcontent.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmmuitemcontent.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMmuItemContentVal {
    bits: u32,
}

impl SpiMemMmuItemContentVal {
    /// Create from raw bits
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
        Self { bits: 0x0000037c }
    }

    /// Read **SPI_MMU_ITEM_CONTENT** `[31:0]`
    ///
    /// MSPI-MMU item content
    #[inline(always)]
    pub const fn spi_mmu_item_content(&self) -> u32 {
        (self.bits & SpiMemMmuItemContent::SPI_MMU_ITEM_CONTENT_MASK)
            >> SpiMemMmuItemContent::SPI_MMU_ITEM_CONTENT_OFFSET
    }

    /// Set **SPI_MMU_ITEM_CONTENT** `[31:0]`
    ///
    /// MSPI-MMU item content
    #[inline(always)]
    pub const fn with_spi_mmu_item_content(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemMmuItemContent::SPI_MMU_ITEM_CONTENT_MASK)
                | ((value << SpiMemMmuItemContent::SPI_MMU_ITEM_CONTENT_OFFSET)
                    & SpiMemMmuItemContent::SPI_MMU_ITEM_CONTENT_MASK),
        }
    }
}

impl Default for SpiMemMmuItemContentVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemMmuItemContent {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMmuItemContentVal {
        SpiMemMmuItemContentVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMmuItemContentVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMmuItemContentVal) -> SpiMemMmuItemContentVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_MMU_ITEM_INDEX Register
///
/// MSPI-MMU item index register
///
/// Address offset: 0x0380
///
/// ## Fields
///
/// - **SPI_MMU_ITEM_INDEX** `[31:0]` (RW)
///
///   MSPI-MMU item index
#[repr(transparent)]
pub struct SpiMemMmuItemIndex {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMmuItemIndex {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMmuItemIndex {
    // ======== Field Constants ========

    /// **SPI_MMU_ITEM_INDEX**
    ///
    /// MSPI-MMU item index
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MMU_ITEM_INDEX_OFFSET: u32 = 0;
    pub const SPI_MMU_ITEM_INDEX_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MMU_ITEM_INDEX** `[31:0]` (RW)
    ///
    /// MSPI-MMU item index
    #[inline(always)]
    pub fn spi_mmu_item_index(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MMU_ITEM_INDEX_MASK) >> Self::SPI_MMU_ITEM_INDEX_OFFSET
    }

    /// Write **SPI_MMU_ITEM_INDEX** `[31:0]` (RW)
    ///
    /// MSPI-MMU item index
    #[inline(always)]
    pub fn write_spi_mmu_item_index(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MMU_ITEM_INDEX_MASK)
                | ((value << Self::SPI_MMU_ITEM_INDEX_OFFSET) & Self::SPI_MMU_ITEM_INDEX_MASK)
        });
    }
}

/// SPI_MEM_MMU_ITEM_INDEX value type for batch read/modify/write operations
///
/// Use `spimemmmuitemindex.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmmuitemindex.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMmuItemIndexVal {
    bits: u32,
}

impl SpiMemMmuItemIndexVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MMU_ITEM_INDEX** `[31:0]`
    ///
    /// MSPI-MMU item index
    #[inline(always)]
    pub const fn spi_mmu_item_index(&self) -> u32 {
        (self.bits & SpiMemMmuItemIndex::SPI_MMU_ITEM_INDEX_MASK)
            >> SpiMemMmuItemIndex::SPI_MMU_ITEM_INDEX_OFFSET
    }

    /// Set **SPI_MMU_ITEM_INDEX** `[31:0]`
    ///
    /// MSPI-MMU item index
    #[inline(always)]
    pub const fn with_spi_mmu_item_index(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemMmuItemIndex::SPI_MMU_ITEM_INDEX_MASK)
                | ((value << SpiMemMmuItemIndex::SPI_MMU_ITEM_INDEX_OFFSET)
                    & SpiMemMmuItemIndex::SPI_MMU_ITEM_INDEX_MASK),
        }
    }
}

impl Default for SpiMemMmuItemIndexVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemMmuItemIndex {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMmuItemIndexVal {
        SpiMemMmuItemIndexVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMmuItemIndexVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMmuItemIndexVal) -> SpiMemMmuItemIndexVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_MMU_POWER_CTRL Register
///
/// MSPI MMU power control register
///
/// Address offset: 0x0384
/// Reset value: 0x13200004
///
/// ## Fields
///
/// - **SPI_MEM_RDN_RESULT** `[31]` (RO)
///
///   MSPI module clock domain and AXI clock domain ECO register result register
///
/// - **SPI_MEM_RDN_ENA** `[30]` (RW)
///
///   ECO register enable bit
///
/// - **SPI_MEM_AUX_CTRL** `[29:16]` (RW)
///
///   MMU PSRAM aux control register
///
/// - **SPI_MMU_MEM_FORCE_PU** `[2]` (RW)
///
///   Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc.
///
/// - **SPI_MMU_MEM_FORCE_PD** `[1]` (RW)
///
///   Set this bit to force mmu-memory powerdown
///
/// - **SPI_MMU_MEM_FORCE_ON** `[0]` (RW)
///
///   Set this bit to enable mmu-memory clock force on
#[repr(transparent)]
pub struct SpiMemMmuPowerCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMmuPowerCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMmuPowerCtrl {
    // ======== Field Constants ========

    /// **SPI_MEM_RDN_RESULT**
    ///
    /// MSPI module clock domain and AXI clock domain ECO register result register
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_RDN_RESULT_OFFSET: u32 = 31;
    pub const SPI_MEM_RDN_RESULT_MASK: u32 = 0x80000000;
    pub const SPI_MEM_RDN_RESULT: u32 = 1 << 31;

    /// **SPI_MEM_RDN_ENA**
    ///
    /// ECO register enable bit
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_RDN_ENA_OFFSET: u32 = 30;
    pub const SPI_MEM_RDN_ENA_MASK: u32 = 0x40000000;
    pub const SPI_MEM_RDN_ENA: u32 = 1 << 30;

    /// **SPI_MEM_AUX_CTRL**
    ///
    /// MMU PSRAM aux control register
    ///
    /// Bits `[29:16]`, mask `0x3fff0000`
    pub const SPI_MEM_AUX_CTRL_OFFSET: u32 = 16;
    pub const SPI_MEM_AUX_CTRL_MASK: u32 = 0x3fff0000;

    /// **SPI_MMU_MEM_FORCE_PU**
    ///
    /// Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MMU_MEM_FORCE_PU_OFFSET: u32 = 2;
    pub const SPI_MMU_MEM_FORCE_PU_MASK: u32 = 0x00000004;
    pub const SPI_MMU_MEM_FORCE_PU: u32 = 1 << 2;

    /// **SPI_MMU_MEM_FORCE_PD**
    ///
    /// Set this bit to force mmu-memory powerdown
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MMU_MEM_FORCE_PD_OFFSET: u32 = 1;
    pub const SPI_MMU_MEM_FORCE_PD_MASK: u32 = 0x00000002;
    pub const SPI_MMU_MEM_FORCE_PD: u32 = 1 << 1;

    /// **SPI_MMU_MEM_FORCE_ON**
    ///
    /// Set this bit to enable mmu-memory clock force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MMU_MEM_FORCE_ON_OFFSET: u32 = 0;
    pub const SPI_MMU_MEM_FORCE_ON_MASK: u32 = 0x00000001;
    pub const SPI_MMU_MEM_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_RDN_RESULT** `[31]` (RO)
    ///
    /// MSPI module clock domain and AXI clock domain ECO register result register
    #[inline(always)]
    pub fn spi_mem_rdn_result(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_RDN_RESULT_MASK) != 0
    }

    /// **SPI_MEM_RDN_ENA** `[30]` (RW)
    ///
    /// ECO register enable bit
    #[inline(always)]
    pub fn spi_mem_rdn_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_RDN_ENA_MASK) != 0
    }

    /// Set SPI_MEM_RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_rdn_ena(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_RDN_ENA_MASK);
    }

    /// Clear SPI_MEM_RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_rdn_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_RDN_ENA_MASK);
    }

    /// **SPI_MEM_AUX_CTRL** `[29:16]` (RW)
    ///
    /// MMU PSRAM aux control register
    #[inline(always)]
    pub fn spi_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_AUX_CTRL_MASK) >> Self::SPI_MEM_AUX_CTRL_OFFSET
    }

    /// Write **SPI_MEM_AUX_CTRL** `[29:16]` (RW)
    ///
    /// MMU PSRAM aux control register
    #[inline(always)]
    pub fn write_spi_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_AUX_CTRL_MASK)
                | ((value << Self::SPI_MEM_AUX_CTRL_OFFSET) & Self::SPI_MEM_AUX_CTRL_MASK)
        });
    }

    /// **SPI_MMU_MEM_FORCE_PU** `[2]` (RW)
    ///
    /// Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc.
    #[inline(always)]
    pub fn spi_mmu_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MMU_MEM_FORCE_PU_MASK) != 0
    }

    /// Set SPI_MMU_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mmu_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MMU_MEM_FORCE_PU_MASK);
    }

    /// Clear SPI_MMU_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mmu_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MMU_MEM_FORCE_PU_MASK);
    }

    /// **SPI_MMU_MEM_FORCE_PD** `[1]` (RW)
    ///
    /// Set this bit to force mmu-memory powerdown
    #[inline(always)]
    pub fn spi_mmu_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MMU_MEM_FORCE_PD_MASK) != 0
    }

    /// Set SPI_MMU_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mmu_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MMU_MEM_FORCE_PD_MASK);
    }

    /// Clear SPI_MMU_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mmu_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MMU_MEM_FORCE_PD_MASK);
    }

    /// **SPI_MMU_MEM_FORCE_ON** `[0]` (RW)
    ///
    /// Set this bit to enable mmu-memory clock force on
    #[inline(always)]
    pub fn spi_mmu_mem_force_on(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MMU_MEM_FORCE_ON_MASK) != 0
    }

    /// Set SPI_MMU_MEM_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mmu_mem_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MMU_MEM_FORCE_ON_MASK);
    }

    /// Clear SPI_MMU_MEM_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mmu_mem_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MMU_MEM_FORCE_ON_MASK);
    }
}

/// SPI_MEM_MMU_POWER_CTRL value type for batch read/modify/write operations
///
/// Use `spimemmmupowerctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmmupowerctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMmuPowerCtrlVal {
    bits: u32,
}

impl SpiMemMmuPowerCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x13200004 }
    }

    /// Read **SPI_MEM_RDN_RESULT** `[31:31]`
    ///
    /// MSPI module clock domain and AXI clock domain ECO register result register
    #[inline(always)]
    pub const fn spi_mem_rdn_result(&self) -> bool {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MEM_RDN_RESULT_MASK) != 0
    }

    /// Read **SPI_MEM_RDN_ENA** `[30:30]`
    ///
    /// ECO register enable bit
    #[inline(always)]
    pub const fn spi_mem_rdn_ena(&self) -> bool {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MEM_RDN_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_AUX_CTRL** `[29:16]`
    ///
    /// MMU PSRAM aux control register
    #[inline(always)]
    pub const fn spi_mem_aux_ctrl(&self) -> u32 {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MEM_AUX_CTRL_MASK)
            >> SpiMemMmuPowerCtrl::SPI_MEM_AUX_CTRL_OFFSET
    }

    /// Read **SPI_MMU_MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc.
    #[inline(always)]
    pub const fn spi_mmu_mem_force_pu(&self) -> bool {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **SPI_MMU_MEM_FORCE_PD** `[1:1]`
    ///
    /// Set this bit to force mmu-memory powerdown
    #[inline(always)]
    pub const fn spi_mmu_mem_force_pd(&self) -> bool {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PD_MASK) != 0
    }

    /// Read **SPI_MMU_MEM_FORCE_ON** `[0:0]`
    ///
    /// Set this bit to enable mmu-memory clock force on
    #[inline(always)]
    pub const fn spi_mmu_mem_force_on(&self) -> bool {
        (self.bits & SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_ON_MASK) != 0
    }

    /// Set **SPI_MEM_RDN_ENA** `[30:30]`
    ///
    /// ECO register enable bit
    #[inline(always)]
    pub const fn with_spi_mem_rdn_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMmuPowerCtrl::SPI_MEM_RDN_ENA_MASK
            } else {
                self.bits & !SpiMemMmuPowerCtrl::SPI_MEM_RDN_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_AUX_CTRL** `[29:16]`
    ///
    /// MMU PSRAM aux control register
    #[inline(always)]
    pub const fn with_spi_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemMmuPowerCtrl::SPI_MEM_AUX_CTRL_MASK)
                | ((value << SpiMemMmuPowerCtrl::SPI_MEM_AUX_CTRL_OFFSET)
                    & SpiMemMmuPowerCtrl::SPI_MEM_AUX_CTRL_MASK),
        }
    }

    /// Set **SPI_MMU_MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc.
    #[inline(always)]
    pub const fn with_spi_mmu_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PU_MASK
            } else {
                self.bits & !SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **SPI_MMU_MEM_FORCE_PD** `[1:1]`
    ///
    /// Set this bit to force mmu-memory powerdown
    #[inline(always)]
    pub const fn with_spi_mmu_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PD_MASK
            } else {
                self.bits & !SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **SPI_MMU_MEM_FORCE_ON** `[0:0]`
    ///
    /// Set this bit to enable mmu-memory clock force on
    #[inline(always)]
    pub const fn with_spi_mmu_mem_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_ON_MASK
            } else {
                self.bits & !SpiMemMmuPowerCtrl::SPI_MMU_MEM_FORCE_ON_MASK
            },
        }
    }
}

impl Default for SpiMemMmuPowerCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemMmuPowerCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMmuPowerCtrlVal {
        SpiMemMmuPowerCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMmuPowerCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMmuPowerCtrlVal) -> SpiMemMmuPowerCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_DPA_CTRL Register
///
/// SPI memory cryption DPA register
///
/// Address offset: 0x0388
/// Reset value: 0x0000000f
///
/// ## Fields
///
/// - **SPI_CRYPT_DPA_SELECT_REGISTER** `[4]` (RW)
///
///   1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits.
///
/// - **SPI_CRYPT_CALC_D_DPA_EN** `[3]` (RW)
///
///   Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1.
///
/// - **SPI_CRYPT_SECURITY_LEVEL** `[2:0]` (RW)
///
///   Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
#[repr(transparent)]
pub struct SpiMemDpaCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDpaCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDpaCtrl {
    // ======== Field Constants ========

    /// **SPI_CRYPT_DPA_SELECT_REGISTER**
    ///
    /// 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_CRYPT_DPA_SELECT_REGISTER_OFFSET: u32 = 4;
    pub const SPI_CRYPT_DPA_SELECT_REGISTER_MASK: u32 = 0x00000010;
    pub const SPI_CRYPT_DPA_SELECT_REGISTER: u32 = 1 << 4;

    /// **SPI_CRYPT_CALC_D_DPA_EN**
    ///
    /// Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_CRYPT_CALC_D_DPA_EN_OFFSET: u32 = 3;
    pub const SPI_CRYPT_CALC_D_DPA_EN_MASK: u32 = 0x00000008;
    pub const SPI_CRYPT_CALC_D_DPA_EN: u32 = 1 << 3;

    /// **SPI_CRYPT_SECURITY_LEVEL**
    ///
    /// Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SPI_CRYPT_SECURITY_LEVEL_OFFSET: u32 = 0;
    pub const SPI_CRYPT_SECURITY_LEVEL_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SPI_CRYPT_DPA_SELECT_REGISTER** `[4]` (RW)
    ///
    /// 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits.
    #[inline(always)]
    pub fn spi_crypt_dpa_select_register(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CRYPT_DPA_SELECT_REGISTER_MASK) != 0
    }

    /// Set SPI_CRYPT_DPA_SELECT_REGISTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_crypt_dpa_select_register(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CRYPT_DPA_SELECT_REGISTER_MASK);
    }

    /// Clear SPI_CRYPT_DPA_SELECT_REGISTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_crypt_dpa_select_register(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CRYPT_DPA_SELECT_REGISTER_MASK);
    }

    /// **SPI_CRYPT_CALC_D_DPA_EN** `[3]` (RW)
    ///
    /// Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1.
    #[inline(always)]
    pub fn spi_crypt_calc_d_dpa_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_CRYPT_CALC_D_DPA_EN_MASK) != 0
    }

    /// Set SPI_CRYPT_CALC_D_DPA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_crypt_calc_d_dpa_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_CRYPT_CALC_D_DPA_EN_MASK);
    }

    /// Clear SPI_CRYPT_CALC_D_DPA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_crypt_calc_d_dpa_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_CRYPT_CALC_D_DPA_EN_MASK);
    }

    /// **SPI_CRYPT_SECURITY_LEVEL** `[2:0]` (RW)
    ///
    /// Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
    #[inline(always)]
    pub fn spi_crypt_security_level(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_CRYPT_SECURITY_LEVEL_MASK)
            >> Self::SPI_CRYPT_SECURITY_LEVEL_OFFSET
    }

    /// Write **SPI_CRYPT_SECURITY_LEVEL** `[2:0]` (RW)
    ///
    /// Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
    #[inline(always)]
    pub fn write_spi_crypt_security_level(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_CRYPT_SECURITY_LEVEL_MASK)
                | ((value << Self::SPI_CRYPT_SECURITY_LEVEL_OFFSET)
                    & Self::SPI_CRYPT_SECURITY_LEVEL_MASK)
        });
    }
}

/// SPI_MEM_DPA_CTRL value type for batch read/modify/write operations
///
/// Use `spimemdpactrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemdpactrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDpaCtrlVal {
    bits: u32,
}

impl SpiMemDpaCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x0000000f }
    }

    /// Read **SPI_CRYPT_DPA_SELECT_REGISTER** `[4:4]`
    ///
    /// 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits.
    #[inline(always)]
    pub const fn spi_crypt_dpa_select_register(&self) -> bool {
        (self.bits & SpiMemDpaCtrl::SPI_CRYPT_DPA_SELECT_REGISTER_MASK) != 0
    }

    /// Read **SPI_CRYPT_CALC_D_DPA_EN** `[3:3]`
    ///
    /// Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1.
    #[inline(always)]
    pub const fn spi_crypt_calc_d_dpa_en(&self) -> bool {
        (self.bits & SpiMemDpaCtrl::SPI_CRYPT_CALC_D_DPA_EN_MASK) != 0
    }

    /// Read **SPI_CRYPT_SECURITY_LEVEL** `[2:0]`
    ///
    /// Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
    #[inline(always)]
    pub const fn spi_crypt_security_level(&self) -> u32 {
        (self.bits & SpiMemDpaCtrl::SPI_CRYPT_SECURITY_LEVEL_MASK)
            >> SpiMemDpaCtrl::SPI_CRYPT_SECURITY_LEVEL_OFFSET
    }

    /// Set **SPI_CRYPT_DPA_SELECT_REGISTER** `[4:4]`
    ///
    /// 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits.
    #[inline(always)]
    pub const fn with_spi_crypt_dpa_select_register(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDpaCtrl::SPI_CRYPT_DPA_SELECT_REGISTER_MASK
            } else {
                self.bits & !SpiMemDpaCtrl::SPI_CRYPT_DPA_SELECT_REGISTER_MASK
            },
        }
    }

    /// Set **SPI_CRYPT_CALC_D_DPA_EN** `[3:3]`
    ///
    /// Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1.
    #[inline(always)]
    pub const fn with_spi_crypt_calc_d_dpa_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemDpaCtrl::SPI_CRYPT_CALC_D_DPA_EN_MASK
            } else {
                self.bits & !SpiMemDpaCtrl::SPI_CRYPT_CALC_D_DPA_EN_MASK
            },
        }
    }

    /// Set **SPI_CRYPT_SECURITY_LEVEL** `[2:0]`
    ///
    /// Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)
    #[inline(always)]
    pub const fn with_spi_crypt_security_level(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDpaCtrl::SPI_CRYPT_SECURITY_LEVEL_MASK)
                | ((value << SpiMemDpaCtrl::SPI_CRYPT_SECURITY_LEVEL_OFFSET)
                    & SpiMemDpaCtrl::SPI_CRYPT_SECURITY_LEVEL_MASK),
        }
    }
}

impl Default for SpiMemDpaCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemDpaCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDpaCtrlVal {
        SpiMemDpaCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDpaCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDpaCtrlVal) -> SpiMemDpaCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_REGISTERRND_ECO_HIGH Register
///
/// MSPI ECO high register
///
/// Address offset: 0x03f0
/// Reset value: 0x0000037c
///
/// ## Fields
///
/// - **SPI_MEM_REGISTERRND_ECO_HIGH** `[31:0]` (RW)
///
///   ECO high register
#[repr(transparent)]
pub struct SpiMemRegisterrndEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemRegisterrndEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemRegisterrndEcoHigh {
    // ======== Field Constants ========

    /// **SPI_MEM_REGISTERRND_ECO_HIGH**
    ///
    /// ECO high register
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_REGISTERRND_ECO_HIGH_OFFSET: u32 = 0;
    pub const SPI_MEM_REGISTERRND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_REGISTERRND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// ECO high register
    #[inline(always)]
    pub fn spi_mem_registerrnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_REGISTERRND_ECO_HIGH_MASK)
            >> Self::SPI_MEM_REGISTERRND_ECO_HIGH_OFFSET
    }

    /// Write **SPI_MEM_REGISTERRND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// ECO high register
    #[inline(always)]
    pub fn write_spi_mem_registerrnd_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_REGISTERRND_ECO_HIGH_MASK)
                | ((value << Self::SPI_MEM_REGISTERRND_ECO_HIGH_OFFSET)
                    & Self::SPI_MEM_REGISTERRND_ECO_HIGH_MASK)
        });
    }
}

/// SPI_MEM_REGISTERRND_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `spimemregisterrndecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemregisterrndecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemRegisterrndEcoHighVal {
    bits: u32,
}

impl SpiMemRegisterrndEcoHighVal {
    /// Create from raw bits
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
        Self { bits: 0x0000037c }
    }

    /// Read **SPI_MEM_REGISTERRND_ECO_HIGH** `[31:0]`
    ///
    /// ECO high register
    #[inline(always)]
    pub const fn spi_mem_registerrnd_eco_high(&self) -> u32 {
        (self.bits & SpiMemRegisterrndEcoHigh::SPI_MEM_REGISTERRND_ECO_HIGH_MASK)
            >> SpiMemRegisterrndEcoHigh::SPI_MEM_REGISTERRND_ECO_HIGH_OFFSET
    }

    /// Set **SPI_MEM_REGISTERRND_ECO_HIGH** `[31:0]`
    ///
    /// ECO high register
    #[inline(always)]
    pub const fn with_spi_mem_registerrnd_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemRegisterrndEcoHigh::SPI_MEM_REGISTERRND_ECO_HIGH_MASK)
                | ((value << SpiMemRegisterrndEcoHigh::SPI_MEM_REGISTERRND_ECO_HIGH_OFFSET)
                    & SpiMemRegisterrndEcoHigh::SPI_MEM_REGISTERRND_ECO_HIGH_MASK),
        }
    }
}

impl Default for SpiMemRegisterrndEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemRegisterrndEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemRegisterrndEcoHighVal {
        SpiMemRegisterrndEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemRegisterrndEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemRegisterrndEcoHighVal) -> SpiMemRegisterrndEcoHighVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_REGISTERRND_ECO_LOW Register
///
/// MSPI ECO low register
///
/// Address offset: 0x03f4
/// Reset value: 0x0000037c
///
/// ## Fields
///
/// - **SPI_MEM_REGISTERRND_ECO_LOW** `[31:0]` (RW)
///
///   ECO low register
#[repr(transparent)]
pub struct SpiMemRegisterrndEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemRegisterrndEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemRegisterrndEcoLow {
    // ======== Field Constants ========

    /// **SPI_MEM_REGISTERRND_ECO_LOW**
    ///
    /// ECO low register
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_REGISTERRND_ECO_LOW_OFFSET: u32 = 0;
    pub const SPI_MEM_REGISTERRND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_REGISTERRND_ECO_LOW** `[31:0]` (RW)
    ///
    /// ECO low register
    #[inline(always)]
    pub fn spi_mem_registerrnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_REGISTERRND_ECO_LOW_MASK)
            >> Self::SPI_MEM_REGISTERRND_ECO_LOW_OFFSET
    }

    /// Write **SPI_MEM_REGISTERRND_ECO_LOW** `[31:0]` (RW)
    ///
    /// ECO low register
    #[inline(always)]
    pub fn write_spi_mem_registerrnd_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_REGISTERRND_ECO_LOW_MASK)
                | ((value << Self::SPI_MEM_REGISTERRND_ECO_LOW_OFFSET)
                    & Self::SPI_MEM_REGISTERRND_ECO_LOW_MASK)
        });
    }
}

/// SPI_MEM_REGISTERRND_ECO_LOW value type for batch read/modify/write operations
///
/// Use `spimemregisterrndecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemregisterrndecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemRegisterrndEcoLowVal {
    bits: u32,
}

impl SpiMemRegisterrndEcoLowVal {
    /// Create from raw bits
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
        Self { bits: 0x0000037c }
    }

    /// Read **SPI_MEM_REGISTERRND_ECO_LOW** `[31:0]`
    ///
    /// ECO low register
    #[inline(always)]
    pub const fn spi_mem_registerrnd_eco_low(&self) -> u32 {
        (self.bits & SpiMemRegisterrndEcoLow::SPI_MEM_REGISTERRND_ECO_LOW_MASK)
            >> SpiMemRegisterrndEcoLow::SPI_MEM_REGISTERRND_ECO_LOW_OFFSET
    }

    /// Set **SPI_MEM_REGISTERRND_ECO_LOW** `[31:0]`
    ///
    /// ECO low register
    #[inline(always)]
    pub const fn with_spi_mem_registerrnd_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemRegisterrndEcoLow::SPI_MEM_REGISTERRND_ECO_LOW_MASK)
                | ((value << SpiMemRegisterrndEcoLow::SPI_MEM_REGISTERRND_ECO_LOW_OFFSET)
                    & SpiMemRegisterrndEcoLow::SPI_MEM_REGISTERRND_ECO_LOW_MASK),
        }
    }
}

impl Default for SpiMemRegisterrndEcoLowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemRegisterrndEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemRegisterrndEcoLowVal {
        SpiMemRegisterrndEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemRegisterrndEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemRegisterrndEcoLowVal) -> SpiMemRegisterrndEcoLowVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_DATE Register
///
/// SPI0 version control register
///
/// Address offset: 0x03fc
/// Reset value: 0x02303100
///
/// ## Fields
///
/// - **SPI_MEM_DATE** `[27:0]` (RW)
///
///   SPI0 register version.
#[repr(transparent)]
pub struct SpiMemDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemDate {
    // ======== Field Constants ========

    /// **SPI_MEM_DATE**
    ///
    /// SPI0 register version.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const SPI_MEM_DATE_OFFSET: u32 = 0;
    pub const SPI_MEM_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_DATE** `[27:0]` (RW)
    ///
    /// SPI0 register version.
    #[inline(always)]
    pub fn spi_mem_date(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DATE_MASK) >> Self::SPI_MEM_DATE_OFFSET
    }

    /// Write **SPI_MEM_DATE** `[27:0]` (RW)
    ///
    /// SPI0 register version.
    #[inline(always)]
    pub fn write_spi_mem_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_DATE_MASK)
                | ((value << Self::SPI_MEM_DATE_OFFSET) & Self::SPI_MEM_DATE_MASK)
        });
    }
}

/// SPI_MEM_DATE value type for batch read/modify/write operations
///
/// Use `spimemdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemDateVal {
    bits: u32,
}

impl SpiMemDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02303100 }
    }

    /// Read **SPI_MEM_DATE** `[27:0]`
    ///
    /// SPI0 register version.
    #[inline(always)]
    pub const fn spi_mem_date(&self) -> u32 {
        (self.bits & SpiMemDate::SPI_MEM_DATE_MASK) >> SpiMemDate::SPI_MEM_DATE_OFFSET
    }

    /// Set **SPI_MEM_DATE** `[27:0]`
    ///
    /// SPI0 register version.
    #[inline(always)]
    pub const fn with_spi_mem_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemDate::SPI_MEM_DATE_MASK)
                | ((value << SpiMemDate::SPI_MEM_DATE_OFFSET) & SpiMemDate::SPI_MEM_DATE_MASK),
        }
    }
}

impl Default for SpiMemDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemDateVal {
        SpiMemDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemDateVal) -> SpiMemDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// SPI0 FSM status register
    pub spi_mem_cmd: SpiMemCmd,
    _reserved0: [u8; 4],
    /// 0x0008
    /// SPI0 control register.
    pub spi_mem_ctrl: SpiMemCtrl,
    /// 0x000c
    /// SPI0 control1 register.
    pub spi_mem_ctrl1: SpiMemCtrl1,
    /// 0x0010
    /// SPI0 control2 register.
    pub spi_mem_ctrl2: SpiMemCtrl2,
    /// 0x0014
    /// SPI clock division control register.
    pub spi_mem_clock: SpiMemClock,
    /// 0x0018
    /// SPI0 user register.
    pub spi_mem_user: SpiMemUser,
    /// 0x001c
    /// SPI0 user1 register.
    pub spi_mem_user1: SpiMemUser1,
    /// 0x0020
    /// SPI0 user2 register.
    pub spi_mem_user2: SpiMemUser2,
    _reserved1: [u8; 8],
    /// 0x002c
    /// SPI0 read control register.
    pub spi_mem_rd_status: SpiMemRdStatus,
    _reserved2: [u8; 4],
    /// 0x0034
    /// SPI0 misc register
    pub spi_mem_misc: SpiMemMisc,
    _reserved3: [u8; 4],
    /// 0x003c
    /// SPI0 bit mode control register.
    pub spi_mem_cache_fctrl: SpiMemCacheFctrl,
    /// 0x0040
    /// SPI0 external RAM control register
    pub spi_mem_cache_sctrl: SpiMemCacheSctrl,
    /// 0x0044
    /// SPI0 external RAM mode control register
    pub spi_mem_sram_cmd: SpiMemSramCmd,
    /// 0x0048
    /// SPI0 external RAM DDR read command control register
    pub spi_mem_sram_drd_cmd: SpiMemSramDrdCmd,
    /// 0x004c
    /// SPI0 external RAM DDR write command control register
    pub spi_mem_sram_dwr_cmd: SpiMemSramDwrCmd,
    /// 0x0050
    /// SPI0 external RAM clock control register
    pub spi_mem_sram_clk: SpiMemSramClk,
    /// 0x0054
    /// SPI0 FSM status register
    pub spi_mem_fsm: SpiMemFsm,
    _reserved4: [u8; 104],
    /// 0x00c0
    /// SPI0 interrupt enable register
    pub spi_mem_int_ena: SpiMemIntEna,
    /// 0x00c4
    /// SPI0 interrupt clear register
    pub spi_mem_int_clr: SpiMemIntClr,
    /// 0x00c8
    /// SPI0 interrupt raw register
    pub spi_mem_int_raw: SpiMemIntRaw,
    /// 0x00cc
    /// SPI0 interrupt status register
    pub spi_mem_int_st: SpiMemIntSt,
    _reserved5: [u8; 4],
    /// 0x00d4
    /// SPI0 flash DDR mode control register
    pub spi_mem_ddr: SpiMemDdr,
    /// 0x00d8
    /// SPI0 external RAM DDR mode control register
    pub spi_smem_ddr: SpiSmemDdr,
    _reserved6: [u8; 36],
    /// 0x0100
    /// MSPI flash PMS section 0 attribute register
    pub spi_fmem_pms0_attr: SpiFmemPms0Attr,
    /// 0x0104
    /// MSPI flash PMS section 1 attribute register
    pub spi_fmem_pms1_attr: SpiFmemPms1Attr,
    /// 0x0108
    /// MSPI flash PMS section 2 attribute register
    pub spi_fmem_pms2_attr: SpiFmemPms2Attr,
    /// 0x010c
    /// MSPI flash PMS section 3 attribute register
    pub spi_fmem_pms3_attr: SpiFmemPms3Attr,
    /// 0x0110
    /// SPI1 flash PMS section 0 start address register
    pub spi_fmem_pms0_addr: SpiFmemPms0Addr,
    /// 0x0114
    /// SPI1 flash PMS section 1 start address register
    pub spi_fmem_pms1_addr: SpiFmemPms1Addr,
    /// 0x0118
    /// SPI1 flash PMS section 2 start address register
    pub spi_fmem_pms2_addr: SpiFmemPms2Addr,
    /// 0x011c
    /// SPI1 flash PMS section 3 start address register
    pub spi_fmem_pms3_addr: SpiFmemPms3Addr,
    /// 0x0120
    /// SPI1 flash PMS section 0 start address register
    pub spi_fmem_pms0_size: SpiFmemPms0Size,
    /// 0x0124
    /// SPI1 flash PMS section 1 start address register
    pub spi_fmem_pms1_size: SpiFmemPms1Size,
    /// 0x0128
    /// SPI1 flash PMS section 2 start address register
    pub spi_fmem_pms2_size: SpiFmemPms2Size,
    /// 0x012c
    /// SPI1 flash PMS section 3 start address register
    pub spi_fmem_pms3_size: SpiFmemPms3Size,
    /// 0x0130
    /// SPI1 flash PMS section 0 start address register
    pub spi_smem_pms0_attr: SpiSmemPms0Attr,
    /// 0x0134
    /// SPI1 flash PMS section 1 start address register
    pub spi_smem_pms1_attr: SpiSmemPms1Attr,
    /// 0x0138
    /// SPI1 flash PMS section 2 start address register
    pub spi_smem_pms2_attr: SpiSmemPms2Attr,
    /// 0x013c
    /// SPI1 flash PMS section 3 start address register
    pub spi_smem_pms3_attr: SpiSmemPms3Attr,
    /// 0x0140
    /// SPI1 external RAM PMS section 0 start address register
    pub spi_smem_pms0_addr: SpiSmemPms0Addr,
    /// 0x0144
    /// SPI1 external RAM PMS section 1 start address register
    pub spi_smem_pms1_addr: SpiSmemPms1Addr,
    /// 0x0148
    /// SPI1 external RAM PMS section 2 start address register
    pub spi_smem_pms2_addr: SpiSmemPms2Addr,
    /// 0x014c
    /// SPI1 external RAM PMS section 3 start address register
    pub spi_smem_pms3_addr: SpiSmemPms3Addr,
    /// 0x0150
    /// SPI1 external RAM PMS section 0 start address register
    pub spi_smem_pms0_size: SpiSmemPms0Size,
    /// 0x0154
    /// SPI1 external RAM PMS section 1 start address register
    pub spi_smem_pms1_size: SpiSmemPms1Size,
    /// 0x0158
    /// SPI1 external RAM PMS section 2 start address register
    pub spi_smem_pms2_size: SpiSmemPms2Size,
    /// 0x015c
    /// SPI1 external RAM PMS section 3 start address register
    pub spi_smem_pms3_size: SpiSmemPms3Size,
    _reserved7: [u8; 4],
    /// 0x0164
    /// SPI1 access reject register
    pub spi_mem_pms_reject: SpiMemPmsReject,
    /// 0x0168
    /// MSPI ECC control register
    pub spi_mem_ecc_ctrl: SpiMemEccCtrl,
    /// 0x016c
    /// MSPI ECC error address register
    pub spi_mem_ecc_err_addr: SpiMemEccErrAddr,
    /// 0x0170
    /// SPI0 AXI request error address.
    pub spi_mem_axi_err_addr: SpiMemAxiErrAddr,
    /// 0x0174
    /// MSPI ECC control register
    pub spi_smem_ecc_ctrl: SpiSmemEccCtrl,
    /// 0x0178
    /// SPI0 AXI address control register
    pub spi_smem_axi_addr_ctrl: SpiSmemAxiAddrCtrl,
    /// 0x017c
    /// SPI0 AXI error response enable register
    pub spi_mem_axi_err_resp_en: SpiMemAxiErrRespEn,
    /// 0x0180
    /// SPI0 flash timing calibration register
    pub spi_mem_timing_cali: SpiMemTimingCali,
    /// 0x0184
    /// MSPI flash input timing delay mode control register
    pub spi_mem_din_mode: SpiMemDinMode,
    /// 0x0188
    /// MSPI flash input timing delay number control register
    pub spi_mem_din_num: SpiMemDinNum,
    /// 0x018c
    /// MSPI flash output timing adjustment control register
    pub spi_mem_dout_mode: SpiMemDoutMode,
    /// 0x0190
    /// MSPI external RAM timing calibration register
    pub spi_smem_timing_cali: SpiSmemTimingCali,
    /// 0x0194
    /// MSPI external RAM input timing delay mode control register
    pub spi_smem_din_mode: SpiSmemDinMode,
    /// 0x0198
    /// MSPI external RAM input timing delay number control register
    pub spi_smem_din_num: SpiSmemDinNum,
    /// 0x019c
    /// MSPI external RAM output timing adjustment control register
    pub spi_smem_dout_mode: SpiSmemDoutMode,
    /// 0x01a0
    /// MSPI external RAM ECC and SPI CS timing control register
    pub spi_smem_ac: SpiSmemAc,
    /// 0x01a4
    /// MSPI 16x external RAM input timing delay mode control register
    pub spi_smem_din_hex_mode: SpiSmemDinHexMode,
    /// 0x01a8
    /// MSPI 16x external RAM input timing delay number control register
    pub spi_smem_din_hex_num: SpiSmemDinHexNum,
    /// 0x01ac
    /// MSPI 16x external RAM output timing adjustment control register
    pub spi_smem_dout_hex_mode: SpiSmemDoutHexMode,
    _reserved8: [u8; 80],
    /// 0x0200
    /// SPI0 clock gate register
    pub spi_mem_clock_gate: SpiMemClockGate,
    _reserved9: [u8; 252],
    /// 0x0300
    /// The base address of the memory that stores plaintext in Manual Encryption
    pub spi_mem_xts_plain_base: SpiMemXtsPlainBase,
    _reserved10: [u8; 60],
    /// 0x0340
    /// Manual Encryption Line-Size register
    pub spi_mem_xts_linesize: SpiMemXtsLinesize,
    /// 0x0344
    /// Manual Encryption destination register
    pub spi_mem_xts_destination: SpiMemXtsDestination,
    /// 0x0348
    /// Manual Encryption physical address register
    pub spi_mem_xts_physical_address: SpiMemXtsPhysicalAddress,
    /// 0x034c
    /// Manual Encryption physical address register
    pub spi_mem_xts_trigger: SpiMemXtsTrigger,
    /// 0x0350
    /// Manual Encryption physical address register
    pub spi_mem_xts_release: SpiMemXtsRelease,
    /// 0x0354
    /// Manual Encryption physical address register
    pub spi_mem_xts_destroy: SpiMemXtsDestroy,
    /// 0x0358
    /// Manual Encryption physical address register
    pub spi_mem_xts_state: SpiMemXtsState,
    /// 0x035c
    /// Manual Encryption version register
    pub spi_mem_xts_date: SpiMemXtsDate,
    _reserved11: [u8; 28],
    /// 0x037c
    /// MSPI-MMU item content register
    pub spi_mem_mmu_item_content: SpiMemMmuItemContent,
    /// 0x0380
    /// MSPI-MMU item index register
    pub spi_mem_mmu_item_index: SpiMemMmuItemIndex,
    /// 0x0384
    /// MSPI MMU power control register
    pub spi_mem_mmu_power_ctrl: SpiMemMmuPowerCtrl,
    /// 0x0388
    /// SPI memory cryption DPA register
    pub spi_mem_dpa_ctrl: SpiMemDpaCtrl,
    _reserved12: [u8; 100],
    /// 0x03f0
    /// MSPI ECO high register
    pub spi_mem_registerrnd_eco_high: SpiMemRegisterrndEcoHigh,
    /// 0x03f4
    /// MSPI ECO low register
    pub spi_mem_registerrnd_eco_low: SpiMemRegisterrndEcoLow,
    _reserved13: [u8; 4],
    /// 0x03fc
    /// SPI0 version control register
    pub spi_mem_date: SpiMemDate,
}
