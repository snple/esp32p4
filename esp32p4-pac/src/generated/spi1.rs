//! SPI1
//! SPI (Serial Peripheral Interface) Controller 1

#![allow(dead_code)]

pub const BASE: u32 = 0x5008d000;

/// SPI_MEM_CMD Register
///
/// SPI1 memory command register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SPI_MEM_FLASH_READ** `[31]` (RW)
///
///   Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_WREN** `[30]` (RW)
///
///   Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_WRDI** `[29]` (RW)
///
///   Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_RDID** `[28]` (RW)
///
///   Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_RDSR** `[27]` (RW)
///
///   Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_WRSR** `[26]` (RW)
///
///   Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_PP** `[25]` (RW)
///
///   Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_SE** `[24]` (RW)
///
///   Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_BE** `[23]` (RW)
///
///   Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_CE** `[22]` (RW)
///
///   Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_DP** `[21]` (RW)
///
///   Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_RES** `[20]` (RW)
///
///   This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_HPM** `[19]` (RW)
///
///   Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_USR** `[18]` (RW)
///
///   User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_PE** `[17]` (RW)
///
///   In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_SLV_ST** `[7:4]` (RO)
///
///   The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
///
/// - **SPI_MEM_MST_ST** `[3:0]` (RO)
///
///   The current status of SPI1 master FSM.
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

    /// **SPI_MEM_FLASH_READ**
    ///
    /// Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_FLASH_READ_OFFSET: u32 = 31;
    pub const SPI_MEM_FLASH_READ_MASK: u32 = 0x80000000;
    pub const SPI_MEM_FLASH_READ: u32 = 1 << 31;

    /// **SPI_MEM_FLASH_WREN**
    ///
    /// Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_FLASH_WREN_OFFSET: u32 = 30;
    pub const SPI_MEM_FLASH_WREN_MASK: u32 = 0x40000000;
    pub const SPI_MEM_FLASH_WREN: u32 = 1 << 30;

    /// **SPI_MEM_FLASH_WRDI**
    ///
    /// Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_FLASH_WRDI_OFFSET: u32 = 29;
    pub const SPI_MEM_FLASH_WRDI_MASK: u32 = 0x20000000;
    pub const SPI_MEM_FLASH_WRDI: u32 = 1 << 29;

    /// **SPI_MEM_FLASH_RDID**
    ///
    /// Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_FLASH_RDID_OFFSET: u32 = 28;
    pub const SPI_MEM_FLASH_RDID_MASK: u32 = 0x10000000;
    pub const SPI_MEM_FLASH_RDID: u32 = 1 << 28;

    /// **SPI_MEM_FLASH_RDSR**
    ///
    /// Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MEM_FLASH_RDSR_OFFSET: u32 = 27;
    pub const SPI_MEM_FLASH_RDSR_MASK: u32 = 0x08000000;
    pub const SPI_MEM_FLASH_RDSR: u32 = 1 << 27;

    /// **SPI_MEM_FLASH_WRSR**
    ///
    /// Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_FLASH_WRSR_OFFSET: u32 = 26;
    pub const SPI_MEM_FLASH_WRSR_MASK: u32 = 0x04000000;
    pub const SPI_MEM_FLASH_WRSR: u32 = 1 << 26;

    /// **SPI_MEM_FLASH_PP**
    ///
    /// Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SPI_MEM_FLASH_PP_OFFSET: u32 = 25;
    pub const SPI_MEM_FLASH_PP_MASK: u32 = 0x02000000;
    pub const SPI_MEM_FLASH_PP: u32 = 1 << 25;

    /// **SPI_MEM_FLASH_SE**
    ///
    /// Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_FLASH_SE_OFFSET: u32 = 24;
    pub const SPI_MEM_FLASH_SE_MASK: u32 = 0x01000000;
    pub const SPI_MEM_FLASH_SE: u32 = 1 << 24;

    /// **SPI_MEM_FLASH_BE**
    ///
    /// Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_MEM_FLASH_BE_OFFSET: u32 = 23;
    pub const SPI_MEM_FLASH_BE_MASK: u32 = 0x00800000;
    pub const SPI_MEM_FLASH_BE: u32 = 1 << 23;

    /// **SPI_MEM_FLASH_CE**
    ///
    /// Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SPI_MEM_FLASH_CE_OFFSET: u32 = 22;
    pub const SPI_MEM_FLASH_CE_MASK: u32 = 0x00400000;
    pub const SPI_MEM_FLASH_CE: u32 = 1 << 22;

    /// **SPI_MEM_FLASH_DP**
    ///
    /// Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SPI_MEM_FLASH_DP_OFFSET: u32 = 21;
    pub const SPI_MEM_FLASH_DP_MASK: u32 = 0x00200000;
    pub const SPI_MEM_FLASH_DP: u32 = 1 << 21;

    /// **SPI_MEM_FLASH_RES**
    ///
    /// This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SPI_MEM_FLASH_RES_OFFSET: u32 = 20;
    pub const SPI_MEM_FLASH_RES_MASK: u32 = 0x00100000;
    pub const SPI_MEM_FLASH_RES: u32 = 1 << 20;

    /// **SPI_MEM_FLASH_HPM**
    ///
    /// Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI_MEM_FLASH_HPM_OFFSET: u32 = 19;
    pub const SPI_MEM_FLASH_HPM_MASK: u32 = 0x00080000;
    pub const SPI_MEM_FLASH_HPM: u32 = 1 << 19;

    /// **SPI_MEM_USR**
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI_MEM_USR_OFFSET: u32 = 18;
    pub const SPI_MEM_USR_MASK: u32 = 0x00040000;
    pub const SPI_MEM_USR: u32 = 1 << 18;

    /// **SPI_MEM_FLASH_PE**
    ///
    /// In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SPI_MEM_FLASH_PE_OFFSET: u32 = 17;
    pub const SPI_MEM_FLASH_PE_MASK: u32 = 0x00020000;
    pub const SPI_MEM_FLASH_PE: u32 = 1 << 17;

    /// **SPI_MEM_SLV_ST**
    ///
    /// The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const SPI_MEM_SLV_ST_OFFSET: u32 = 4;
    pub const SPI_MEM_SLV_ST_MASK: u32 = 0x000000f0;

    /// **SPI_MEM_MST_ST**
    ///
    /// The current status of SPI1 master FSM.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SPI_MEM_MST_ST_OFFSET: u32 = 0;
    pub const SPI_MEM_MST_ST_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SPI_MEM_FLASH_READ** `[31]` (RW)
    ///
    /// Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_read(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_READ_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_READ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_read(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_READ_MASK);
    }

    /// Clear SPI_MEM_FLASH_READ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_read(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_READ_MASK);
    }

    /// **SPI_MEM_FLASH_WREN** `[30]` (RW)
    ///
    /// Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_wren(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_WREN_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_WREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_wren(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_WREN_MASK);
    }

    /// Clear SPI_MEM_FLASH_WREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_wren(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_WREN_MASK);
    }

    /// **SPI_MEM_FLASH_WRDI** `[29]` (RW)
    ///
    /// Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_wrdi(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_WRDI_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_WRDI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_wrdi(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_WRDI_MASK);
    }

    /// Clear SPI_MEM_FLASH_WRDI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_wrdi(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_WRDI_MASK);
    }

    /// **SPI_MEM_FLASH_RDID** `[28]` (RW)
    ///
    /// Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_rdid(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_RDID_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_RDID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_rdid(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_RDID_MASK);
    }

    /// Clear SPI_MEM_FLASH_RDID bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_rdid(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_RDID_MASK);
    }

    /// **SPI_MEM_FLASH_RDSR** `[27]` (RW)
    ///
    /// Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_rdsr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_RDSR_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_RDSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_rdsr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_RDSR_MASK);
    }

    /// Clear SPI_MEM_FLASH_RDSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_rdsr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_RDSR_MASK);
    }

    /// **SPI_MEM_FLASH_WRSR** `[26]` (RW)
    ///
    /// Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_wrsr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_WRSR_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_WRSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_wrsr(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_WRSR_MASK);
    }

    /// Clear SPI_MEM_FLASH_WRSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_wrsr(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_WRSR_MASK);
    }

    /// **SPI_MEM_FLASH_PP** `[25]` (RW)
    ///
    /// Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_pp(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PP_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pp(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_PP_MASK);
    }

    /// Clear SPI_MEM_FLASH_PP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pp(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_PP_MASK);
    }

    /// **SPI_MEM_FLASH_SE** `[24]` (RW)
    ///
    /// Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_se(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_SE_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_SE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_se(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_SE_MASK);
    }

    /// Clear SPI_MEM_FLASH_SE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_se(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_SE_MASK);
    }

    /// **SPI_MEM_FLASH_BE** `[23]` (RW)
    ///
    /// Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_be(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_BE_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_BE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_be(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_BE_MASK);
    }

    /// Clear SPI_MEM_FLASH_BE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_be(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_BE_MASK);
    }

    /// **SPI_MEM_FLASH_CE** `[22]` (RW)
    ///
    /// Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_ce(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_CE_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_ce(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_CE_MASK);
    }

    /// Clear SPI_MEM_FLASH_CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_ce(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_CE_MASK);
    }

    /// **SPI_MEM_FLASH_DP** `[21]` (RW)
    ///
    /// Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_dp(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_DP_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_dp(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_DP_MASK);
    }

    /// Clear SPI_MEM_FLASH_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_dp(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_DP_MASK);
    }

    /// **SPI_MEM_FLASH_RES** `[20]` (RW)
    ///
    /// This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_res(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_RES_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_RES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_res(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_RES_MASK);
    }

    /// Clear SPI_MEM_FLASH_RES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_res(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_RES_MASK);
    }

    /// **SPI_MEM_FLASH_HPM** `[19]` (RW)
    ///
    /// Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_hpm(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_HPM_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_HPM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_hpm(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_HPM_MASK);
    }

    /// Clear SPI_MEM_FLASH_HPM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_hpm(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_HPM_MASK);
    }

    /// **SPI_MEM_USR** `[18]` (RW)
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_usr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MASK) != 0
    }

    /// Set SPI_MEM_USR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_USR_MASK);
    }

    /// Clear SPI_MEM_USR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_USR_MASK);
    }

    /// **SPI_MEM_FLASH_PE** `[17]` (RW)
    ///
    /// In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_pe(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PE_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pe(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_PE_MASK);
    }

    /// Clear SPI_MEM_FLASH_PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pe(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_FLASH_PE_MASK);
    }

    /// **SPI_MEM_SLV_ST** `[7:4]` (RO)
    ///
    /// The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    #[inline(always)]
    pub fn spi_mem_slv_st(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SLV_ST_MASK) >> Self::SPI_MEM_SLV_ST_OFFSET
    }

    /// **SPI_MEM_MST_ST** `[3:0]` (RO)
    ///
    /// The current status of SPI1 master FSM.
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

    /// Read **SPI_MEM_FLASH_READ** `[31:31]`
    ///
    /// Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_read(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_READ_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_WREN** `[30:30]`
    ///
    /// Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_wren(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_WREN_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_WRDI** `[29:29]`
    ///
    /// Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_wrdi(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_WRDI_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_RDID** `[28:28]`
    ///
    /// Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_rdid(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_RDID_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_RDSR** `[27:27]`
    ///
    /// Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_rdsr(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_RDSR_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_WRSR** `[26:26]`
    ///
    /// Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_wrsr(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_WRSR_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PP** `[25:25]`
    ///
    /// Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_pp(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_PP_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_SE** `[24:24]`
    ///
    /// Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_se(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_SE_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_BE** `[23:23]`
    ///
    /// Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_be(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_BE_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_CE** `[22:22]`
    ///
    /// Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_ce(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_CE_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_DP** `[21:21]`
    ///
    /// Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_dp(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_DP_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_RES** `[20:20]`
    ///
    /// This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_res(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_RES_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_HPM** `[19:19]`
    ///
    /// Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_hpm(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_HPM_MASK) != 0
    }

    /// Read **SPI_MEM_USR** `[18:18]`
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_usr(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_USR_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PE** `[17:17]`
    ///
    /// In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_pe(&self) -> bool {
        (self.bits & SpiMemCmd::SPI_MEM_FLASH_PE_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST** `[7:4]`
    ///
    /// The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state.
    #[inline(always)]
    pub const fn spi_mem_slv_st(&self) -> u32 {
        (self.bits & SpiMemCmd::SPI_MEM_SLV_ST_MASK) >> SpiMemCmd::SPI_MEM_SLV_ST_OFFSET
    }

    /// Read **SPI_MEM_MST_ST** `[3:0]`
    ///
    /// The current status of SPI1 master FSM.
    #[inline(always)]
    pub const fn spi_mem_mst_st(&self) -> u32 {
        (self.bits & SpiMemCmd::SPI_MEM_MST_ST_MASK) >> SpiMemCmd::SPI_MEM_MST_ST_OFFSET
    }

    /// Set **SPI_MEM_FLASH_READ** `[31:31]`
    ///
    /// Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_read(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_READ_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_READ_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_WREN** `[30:30]`
    ///
    /// Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_wren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_WREN_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_WREN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_WRDI** `[29:29]`
    ///
    /// Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_wrdi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_WRDI_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_WRDI_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_RDID** `[28:28]`
    ///
    /// Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_rdid(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_RDID_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_RDID_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_RDSR** `[27:27]`
    ///
    /// Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_rdsr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_RDSR_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_RDSR_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_WRSR** `[26:26]`
    ///
    /// Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_wrsr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_WRSR_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_WRSR_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PP** `[25:25]`
    ///
    /// Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_PP_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_PP_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_SE** `[24:24]`
    ///
    /// Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_se(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_SE_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_SE_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_BE** `[23:23]`
    ///
    /// Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_be(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_BE_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_BE_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_CE** `[22:22]`
    ///
    /// Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_ce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_CE_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_CE_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_DP** `[21:21]`
    ///
    /// Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_DP_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_DP_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_RES** `[20:20]`
    ///
    /// This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_res(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_RES_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_RES_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_HPM** `[19:19]`
    ///
    /// Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_hpm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_HPM_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_HPM_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR** `[18:18]`
    ///
    /// User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_usr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_USR_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_USR_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PE** `[17:17]`
    ///
    /// In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCmd::SPI_MEM_FLASH_PE_MASK
            } else {
                self.bits & !SpiMemCmd::SPI_MEM_FLASH_PE_MASK
            },
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemCmdVal) -> SpiMemCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_ADDR Register
///
/// SPI1 address register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **SPI_MEM_USR_ADDR_VALUE** `[31:0]` (RW)
///
///   In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
#[repr(transparent)]
pub struct SpiMemAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemAddr {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_ADDR_VALUE**
    ///
    /// In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_USR_ADDR_VALUE_OFFSET: u32 = 0;
    pub const SPI_MEM_USR_ADDR_VALUE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_ADDR_VALUE** `[31:0]` (RW)
    ///
    /// In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_ADDR_VALUE_MASK)
            >> Self::SPI_MEM_USR_ADDR_VALUE_OFFSET
    }

    /// Write **SPI_MEM_USR_ADDR_VALUE** `[31:0]` (RW)
    ///
    /// In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
    #[inline(always)]
    pub fn write_spi_mem_usr_addr_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_ADDR_VALUE_MASK)
                | ((value << Self::SPI_MEM_USR_ADDR_VALUE_OFFSET)
                    & Self::SPI_MEM_USR_ADDR_VALUE_MASK)
        });
    }
}

/// SPI_MEM_ADDR value type for batch read/modify/write operations
///
/// Use `spimemaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemAddrVal {
    bits: u32,
}

impl SpiMemAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_USR_ADDR_VALUE** `[31:0]`
    ///
    /// In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
    #[inline(always)]
    pub const fn spi_mem_usr_addr_value(&self) -> u32 {
        (self.bits & SpiMemAddr::SPI_MEM_USR_ADDR_VALUE_MASK)
            >> SpiMemAddr::SPI_MEM_USR_ADDR_VALUE_OFFSET
    }

    /// Set **SPI_MEM_USR_ADDR_VALUE** `[31:0]`
    ///
    /// In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer.
    #[inline(always)]
    pub const fn with_spi_mem_usr_addr_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemAddr::SPI_MEM_USR_ADDR_VALUE_MASK)
                | ((value << SpiMemAddr::SPI_MEM_USR_ADDR_VALUE_OFFSET)
                    & SpiMemAddr::SPI_MEM_USR_ADDR_VALUE_MASK),
        }
    }
}

impl Default for SpiMemAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemAddrVal {
        SpiMemAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemAddrVal) -> SpiMemAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_CTRL Register
///
/// SPI1 control register.
///
/// Address offset: 0x0008
/// Reset value: 0x002ca00c
///
/// ## Fields
///
/// - **SPI_MEM_FREAD_QIO** `[24]` (RW)
///
///   In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_FREAD_DIO** `[23]` (RW)
///
///   In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_WRSR_2B** `[22]` (RW)
///
///   two bytes data will be written to status register when it is set. 1: enable 0: disable.
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
/// - **SPI_MEM_RESANDRES** `[15]` (RW)
///
///   The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable.
///
/// - **SPI_MEM_FREAD_DUAL** `[14]` (RW)
///
///   In the read operations, read-data phase apply 2 signals. 1: enable 0: disable.
///
/// - **SPI_MEM_FASTRD_MODE** `[13]` (RW)
///
///   This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable.
///
/// - **SPI_MEM_TX_CRC_EN** `[11]` (RW)
///
///   For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable
///
/// - **SPI_MEM_FCS_CRC_EN** `[10]` (RW)
///
///   For SPI1, initialize crc32 module before writing encrypted data to flash. Active low.
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
///   In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
///
/// - **SPI_MEM_FDUMMY_RIN** `[2]` (RW)
///
///   In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
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

    /// **SPI_MEM_WRSR_2B**
    ///
    /// two bytes data will be written to status register when it is set. 1: enable 0: disable.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SPI_MEM_WRSR_2B_OFFSET: u32 = 22;
    pub const SPI_MEM_WRSR_2B_MASK: u32 = 0x00400000;
    pub const SPI_MEM_WRSR_2B: u32 = 1 << 22;

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

    /// **SPI_MEM_RESANDRES**
    ///
    /// The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_MEM_RESANDRES_OFFSET: u32 = 15;
    pub const SPI_MEM_RESANDRES_MASK: u32 = 0x00008000;
    pub const SPI_MEM_RESANDRES: u32 = 1 << 15;

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
    /// This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_MEM_FASTRD_MODE_OFFSET: u32 = 13;
    pub const SPI_MEM_FASTRD_MODE_MASK: u32 = 0x00002000;
    pub const SPI_MEM_FASTRD_MODE: u32 = 1 << 13;

    /// **SPI_MEM_TX_CRC_EN**
    ///
    /// For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPI_MEM_TX_CRC_EN_OFFSET: u32 = 11;
    pub const SPI_MEM_TX_CRC_EN_MASK: u32 = 0x00000800;
    pub const SPI_MEM_TX_CRC_EN: u32 = 1 << 11;

    /// **SPI_MEM_FCS_CRC_EN**
    ///
    /// For SPI1, initialize crc32 module before writing encrypted data to flash. Active low.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_FCS_CRC_EN_OFFSET: u32 = 10;
    pub const SPI_MEM_FCS_CRC_EN_MASK: u32 = 0x00000400;
    pub const SPI_MEM_FCS_CRC_EN: u32 = 1 << 10;

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
    /// In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FDUMMY_WOUT_OFFSET: u32 = 3;
    pub const SPI_MEM_FDUMMY_WOUT_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FDUMMY_WOUT: u32 = 1 << 3;

    /// **SPI_MEM_FDUMMY_RIN**
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_FDUMMY_RIN_OFFSET: u32 = 2;
    pub const SPI_MEM_FDUMMY_RIN_MASK: u32 = 0x00000004;
    pub const SPI_MEM_FDUMMY_RIN: u32 = 1 << 2;

    // ======== Field Accessors ========

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

    /// **SPI_MEM_WRSR_2B** `[22]` (RW)
    ///
    /// two bytes data will be written to status register when it is set. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_wrsr_2b(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WRSR_2B_MASK) != 0
    }

    /// Set SPI_MEM_WRSR_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wrsr_2b(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_WRSR_2B_MASK);
    }

    /// Clear SPI_MEM_WRSR_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wrsr_2b(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_WRSR_2B_MASK);
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

    /// **SPI_MEM_RESANDRES** `[15]` (RW)
    ///
    /// The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_resandres(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_RESANDRES_MASK) != 0
    }

    /// Set SPI_MEM_RESANDRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_resandres(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_RESANDRES_MASK);
    }

    /// Clear SPI_MEM_RESANDRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_resandres(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_RESANDRES_MASK);
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
    /// This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable.
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

    /// **SPI_MEM_TX_CRC_EN** `[11]` (RW)
    ///
    /// For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable
    #[inline(always)]
    pub fn spi_mem_tx_crc_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_TX_CRC_EN_MASK) != 0
    }

    /// Set SPI_MEM_TX_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_tx_crc_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_TX_CRC_EN_MASK);
    }

    /// Clear SPI_MEM_TX_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_tx_crc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_TX_CRC_EN_MASK);
    }

    /// **SPI_MEM_FCS_CRC_EN** `[10]` (RW)
    ///
    /// For SPI1, initialize crc32 module before writing encrypted data to flash. Active low.
    #[inline(always)]
    pub fn spi_mem_fcs_crc_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FCS_CRC_EN_MASK) != 0
    }

    /// Set SPI_MEM_FCS_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fcs_crc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FCS_CRC_EN_MASK);
    }

    /// Clear SPI_MEM_FCS_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fcs_crc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FCS_CRC_EN_MASK);
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
    /// In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
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
    /// In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
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
        Self { bits: 0x002ca00c }
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

    /// Read **SPI_MEM_WRSR_2B** `[22:22]`
    ///
    /// two bytes data will be written to status register when it is set. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_wrsr_2b(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_WRSR_2B_MASK) != 0
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

    /// Read **SPI_MEM_RESANDRES** `[15:15]`
    ///
    /// The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_resandres(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_RESANDRES_MASK) != 0
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
    /// This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_fastrd_mode(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FASTRD_MODE_MASK) != 0
    }

    /// Read **SPI_MEM_TX_CRC_EN** `[11:11]`
    ///
    /// For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable
    #[inline(always)]
    pub const fn spi_mem_tx_crc_en(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_TX_CRC_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FCS_CRC_EN** `[10:10]`
    ///
    /// For SPI1, initialize crc32 module before writing encrypted data to flash. Active low.
    #[inline(always)]
    pub const fn spi_mem_fcs_crc_en(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FCS_CRC_EN_MASK) != 0
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
    /// In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_fdummy_wout(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDUMMY_WOUT_MASK) != 0
    }

    /// Read **SPI_MEM_FDUMMY_RIN** `[2:2]`
    ///
    /// In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
    #[inline(always)]
    pub const fn spi_mem_fdummy_rin(&self) -> bool {
        (self.bits & SpiMemCtrl::SPI_MEM_FDUMMY_RIN_MASK) != 0
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

    /// Set **SPI_MEM_WRSR_2B** `[22:22]`
    ///
    /// two bytes data will be written to status register when it is set. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_wrsr_2b(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_WRSR_2B_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_WRSR_2B_MASK
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

    /// Set **SPI_MEM_RESANDRES** `[15:15]`
    ///
    /// The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_resandres(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_RESANDRES_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_RESANDRES_MASK
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
    /// This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable.
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

    /// Set **SPI_MEM_TX_CRC_EN** `[11:11]`
    ///
    /// For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable
    #[inline(always)]
    pub const fn with_spi_mem_tx_crc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_TX_CRC_EN_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_TX_CRC_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FCS_CRC_EN** `[10:10]`
    ///
    /// For SPI1, initialize crc32 module before writing encrypted data to flash. Active low.
    #[inline(always)]
    pub const fn with_spi_mem_fcs_crc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemCtrl::SPI_MEM_FCS_CRC_EN_MASK
            } else {
                self.bits & !SpiMemCtrl::SPI_MEM_FCS_CRC_EN_MASK
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
    /// In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
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
    /// In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller.
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
/// SPI1 control1 register.
///
/// Address offset: 0x000c
/// Reset value: 0x00000ffc
///
/// ## Fields
///
/// - **SPI_MEM_CS_HOLD_DLY_RES** `[11:2]` (RW)
///
///   After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
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

    /// **SPI_MEM_CS_HOLD_DLY_RES**
    ///
    /// After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
    ///
    /// Bits `[11:2]`, mask `0x00000ffc`
    pub const SPI_MEM_CS_HOLD_DLY_RES_OFFSET: u32 = 2;
    pub const SPI_MEM_CS_HOLD_DLY_RES_MASK: u32 = 0x00000ffc;

    /// **SPI_MEM_CLK_MODE**
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI_MEM_CLK_MODE_OFFSET: u32 = 0;
    pub const SPI_MEM_CLK_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SPI_MEM_CS_HOLD_DLY_RES** `[11:2]` (RW)
    ///
    /// After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_CS_HOLD_DLY_RES_MASK)
            >> Self::SPI_MEM_CS_HOLD_DLY_RES_OFFSET
    }

    /// Write **SPI_MEM_CS_HOLD_DLY_RES** `[11:2]` (RW)
    ///
    /// After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
    #[inline(always)]
    pub fn write_spi_mem_cs_hold_dly_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_CS_HOLD_DLY_RES_MASK)
                | ((value << Self::SPI_MEM_CS_HOLD_DLY_RES_OFFSET)
                    & Self::SPI_MEM_CS_HOLD_DLY_RES_MASK)
        });
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
        Self { bits: 0x00000ffc }
    }

    /// Read **SPI_MEM_CS_HOLD_DLY_RES** `[11:2]`
    ///
    /// After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
    #[inline(always)]
    pub const fn spi_mem_cs_hold_dly_res(&self) -> u32 {
        (self.bits & SpiMemCtrl1::SPI_MEM_CS_HOLD_DLY_RES_MASK)
            >> SpiMemCtrl1::SPI_MEM_CS_HOLD_DLY_RES_OFFSET
    }

    /// Read **SPI_MEM_CLK_MODE** `[1:0]`
    ///
    /// SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on.
    #[inline(always)]
    pub const fn spi_mem_clk_mode(&self) -> u32 {
        (self.bits & SpiMemCtrl1::SPI_MEM_CLK_MODE_MASK) >> SpiMemCtrl1::SPI_MEM_CLK_MODE_OFFSET
    }

    /// Set **SPI_MEM_CS_HOLD_DLY_RES** `[11:2]`
    ///
    /// After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 512) SPI_CLK cycles.
    #[inline(always)]
    pub const fn with_spi_mem_cs_hold_dly_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemCtrl1::SPI_MEM_CS_HOLD_DLY_RES_MASK)
                | ((value << SpiMemCtrl1::SPI_MEM_CS_HOLD_DLY_RES_OFFSET)
                    & SpiMemCtrl1::SPI_MEM_CS_HOLD_DLY_RES_MASK),
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
/// SPI1 control2 register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **SPI_MEM_SYNC_RESET** `[31]` (WO)
///
///   The FSM will be reset.
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
    /// The FSM will be reset.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_SYNC_RESET_OFFSET: u32 = 31;
    pub const SPI_MEM_SYNC_RESET_MASK: u32 = 0x80000000;
    pub const SPI_MEM_SYNC_RESET: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **SPI_MEM_SYNC_RESET** (WO)
    ///
    /// The FSM will be reset.
    #[inline(always)]
    pub fn trigger_spi_mem_sync_reset(&self) {
        self.value.write_bits(Self::SPI_MEM_SYNC_RESET_MASK);
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

    /// Set **SPI_MEM_SYNC_RESET** `[31:31]`
    ///
    /// The FSM will be reset.
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
}

impl Default for SpiMemCtrl2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemCtrl2 {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemCtrl2Val) {
        self.value.write_bits(val.bits())
    }
}

/// SPI_MEM_CLOCK Register
///
/// SPI1 clock division control register.
///
/// Address offset: 0x0014
/// Reset value: 0x00030103
///
/// ## Fields
///
/// - **SPI_MEM_CLK_EQU_SYSCLK** `[31]` (RW)
///
///   reserved
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
    /// reserved
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
    /// reserved
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
    /// reserved
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
    /// reserved
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
/// SPI1 user register.
///
/// Address offset: 0x0018
/// Reset value: 0x80000000
///
/// ## Fields
///
/// - **SPI_MEM_USR_COMMAND** `[31]` (RW)
///
///   This bit enable the command phase of an operation.
///
/// - **SPI_MEM_USR_ADDR** `[30]` (RW)
///
///   This bit enable the address phase of an operation.
///
/// - **SPI_MEM_USR_DUMMY** `[29]` (RW)
///
///   This bit enable the dummy phase of an operation.
///
/// - **SPI_MEM_USR_MISO** `[28]` (RW)
///
///   This bit enable the read-data phase of an operation.
///
/// - **SPI_MEM_USR_MOSI** `[27]` (RW)
///
///   This bit enable the write-data phase of an operation.
///
/// - **SPI_MEM_USR_DUMMY_IDLE** `[26]` (RW)
///
///   SPI clock is disable in dummy phase when the bit is enable.
///
/// - **SPI_MEM_USR_MOSI_HIGHPART** `[25]` (RW)
///
///   write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
///
/// - **SPI_MEM_USR_MISO_HIGHPART** `[24]` (RW)
///
///   read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
///
/// - **SPI_MEM_FWRITE_QIO** `[15]` (RW)
///
///   In the write operations address phase and read-data phase apply 4 signals.
///
/// - **SPI_MEM_FWRITE_DIO** `[14]` (RW)
///
///   In the write operations address phase and read-data phase apply 2 signals.
///
/// - **SPI_MEM_FWRITE_QUAD** `[13]` (RW)
///
///   In the write operations read-data phase apply 4 signals
///
/// - **SPI_MEM_FWRITE_DUAL** `[12]` (RW)
///
///   In the write operations read-data phase apply 2 signals
///
/// - **SPI_MEM_CK_OUT_EDGE** `[9]` (RW)
///
///   the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
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

    /// **SPI_MEM_USR_COMMAND**
    ///
    /// This bit enable the command phase of an operation.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_MEM_USR_COMMAND_OFFSET: u32 = 31;
    pub const SPI_MEM_USR_COMMAND_MASK: u32 = 0x80000000;
    pub const SPI_MEM_USR_COMMAND: u32 = 1 << 31;

    /// **SPI_MEM_USR_ADDR**
    ///
    /// This bit enable the address phase of an operation.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SPI_MEM_USR_ADDR_OFFSET: u32 = 30;
    pub const SPI_MEM_USR_ADDR_MASK: u32 = 0x40000000;
    pub const SPI_MEM_USR_ADDR: u32 = 1 << 30;

    /// **SPI_MEM_USR_DUMMY**
    ///
    /// This bit enable the dummy phase of an operation.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SPI_MEM_USR_DUMMY_OFFSET: u32 = 29;
    pub const SPI_MEM_USR_DUMMY_MASK: u32 = 0x20000000;
    pub const SPI_MEM_USR_DUMMY: u32 = 1 << 29;

    /// **SPI_MEM_USR_MISO**
    ///
    /// This bit enable the read-data phase of an operation.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SPI_MEM_USR_MISO_OFFSET: u32 = 28;
    pub const SPI_MEM_USR_MISO_MASK: u32 = 0x10000000;
    pub const SPI_MEM_USR_MISO: u32 = 1 << 28;

    /// **SPI_MEM_USR_MOSI**
    ///
    /// This bit enable the write-data phase of an operation.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SPI_MEM_USR_MOSI_OFFSET: u32 = 27;
    pub const SPI_MEM_USR_MOSI_MASK: u32 = 0x08000000;
    pub const SPI_MEM_USR_MOSI: u32 = 1 << 27;

    /// **SPI_MEM_USR_DUMMY_IDLE**
    ///
    /// SPI clock is disable in dummy phase when the bit is enable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SPI_MEM_USR_DUMMY_IDLE_OFFSET: u32 = 26;
    pub const SPI_MEM_USR_DUMMY_IDLE_MASK: u32 = 0x04000000;
    pub const SPI_MEM_USR_DUMMY_IDLE: u32 = 1 << 26;

    /// **SPI_MEM_USR_MOSI_HIGHPART**
    ///
    /// write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SPI_MEM_USR_MOSI_HIGHPART_OFFSET: u32 = 25;
    pub const SPI_MEM_USR_MOSI_HIGHPART_MASK: u32 = 0x02000000;
    pub const SPI_MEM_USR_MOSI_HIGHPART: u32 = 1 << 25;

    /// **SPI_MEM_USR_MISO_HIGHPART**
    ///
    /// read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_USR_MISO_HIGHPART_OFFSET: u32 = 24;
    pub const SPI_MEM_USR_MISO_HIGHPART_MASK: u32 = 0x01000000;
    pub const SPI_MEM_USR_MISO_HIGHPART: u32 = 1 << 24;

    /// **SPI_MEM_FWRITE_QIO**
    ///
    /// In the write operations address phase and read-data phase apply 4 signals.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_MEM_FWRITE_QIO_OFFSET: u32 = 15;
    pub const SPI_MEM_FWRITE_QIO_MASK: u32 = 0x00008000;
    pub const SPI_MEM_FWRITE_QIO: u32 = 1 << 15;

    /// **SPI_MEM_FWRITE_DIO**
    ///
    /// In the write operations address phase and read-data phase apply 2 signals.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SPI_MEM_FWRITE_DIO_OFFSET: u32 = 14;
    pub const SPI_MEM_FWRITE_DIO_MASK: u32 = 0x00004000;
    pub const SPI_MEM_FWRITE_DIO: u32 = 1 << 14;

    /// **SPI_MEM_FWRITE_QUAD**
    ///
    /// In the write operations read-data phase apply 4 signals
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_MEM_FWRITE_QUAD_OFFSET: u32 = 13;
    pub const SPI_MEM_FWRITE_QUAD_MASK: u32 = 0x00002000;
    pub const SPI_MEM_FWRITE_QUAD: u32 = 1 << 13;

    /// **SPI_MEM_FWRITE_DUAL**
    ///
    /// In the write operations read-data phase apply 2 signals
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI_MEM_FWRITE_DUAL_OFFSET: u32 = 12;
    pub const SPI_MEM_FWRITE_DUAL_MASK: u32 = 0x00001000;
    pub const SPI_MEM_FWRITE_DUAL: u32 = 1 << 12;

    /// **SPI_MEM_CK_OUT_EDGE**
    ///
    /// the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_CK_OUT_EDGE_OFFSET: u32 = 9;
    pub const SPI_MEM_CK_OUT_EDGE_MASK: u32 = 0x00000200;
    pub const SPI_MEM_CK_OUT_EDGE: u32 = 1 << 9;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_COMMAND** `[31]` (RW)
    ///
    /// This bit enable the command phase of an operation.
    #[inline(always)]
    pub fn spi_mem_usr_command(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_COMMAND_MASK) != 0
    }

    /// Set SPI_MEM_USR_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_command(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_COMMAND_MASK);
    }

    /// Clear SPI_MEM_USR_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_command(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_COMMAND_MASK);
    }

    /// **SPI_MEM_USR_ADDR** `[30]` (RW)
    ///
    /// This bit enable the address phase of an operation.
    #[inline(always)]
    pub fn spi_mem_usr_addr(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_ADDR_MASK) != 0
    }

    /// Set SPI_MEM_USR_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_addr(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_USR_ADDR_MASK);
    }

    /// Clear SPI_MEM_USR_ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_addr(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_USR_ADDR_MASK);
    }

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

    /// **SPI_MEM_USR_MISO** `[28]` (RW)
    ///
    /// This bit enable the read-data phase of an operation.
    #[inline(always)]
    pub fn spi_mem_usr_miso(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MISO_MASK) != 0
    }

    /// Set SPI_MEM_USR_MISO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_miso(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_USR_MISO_MASK);
    }

    /// Clear SPI_MEM_USR_MISO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_miso(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_USR_MISO_MASK);
    }

    /// **SPI_MEM_USR_MOSI** `[27]` (RW)
    ///
    /// This bit enable the write-data phase of an operation.
    #[inline(always)]
    pub fn spi_mem_usr_mosi(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MOSI_MASK) != 0
    }

    /// Set SPI_MEM_USR_MOSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_mosi(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_USR_MOSI_MASK);
    }

    /// Clear SPI_MEM_USR_MOSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_mosi(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_USR_MOSI_MASK);
    }

    /// **SPI_MEM_USR_DUMMY_IDLE** `[26]` (RW)
    ///
    /// SPI clock is disable in dummy phase when the bit is enable.
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

    /// **SPI_MEM_USR_MOSI_HIGHPART** `[25]` (RW)
    ///
    /// write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_usr_mosi_highpart(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MOSI_HIGHPART_MASK) != 0
    }

    /// Set SPI_MEM_USR_MOSI_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_mosi_highpart(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_MOSI_HIGHPART_MASK);
    }

    /// Clear SPI_MEM_USR_MOSI_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_mosi_highpart(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_MOSI_HIGHPART_MASK);
    }

    /// **SPI_MEM_USR_MISO_HIGHPART** `[24]` (RW)
    ///
    /// read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_usr_miso_highpart(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_USR_MISO_HIGHPART_MASK) != 0
    }

    /// Set SPI_MEM_USR_MISO_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_usr_miso_highpart(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_USR_MISO_HIGHPART_MASK);
    }

    /// Clear SPI_MEM_USR_MISO_HIGHPART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_usr_miso_highpart(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_USR_MISO_HIGHPART_MASK);
    }

    /// **SPI_MEM_FWRITE_QIO** `[15]` (RW)
    ///
    /// In the write operations address phase and read-data phase apply 4 signals.
    #[inline(always)]
    pub fn spi_mem_fwrite_qio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FWRITE_QIO_MASK) != 0
    }

    /// Set SPI_MEM_FWRITE_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fwrite_qio(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FWRITE_QIO_MASK);
    }

    /// Clear SPI_MEM_FWRITE_QIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fwrite_qio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FWRITE_QIO_MASK);
    }

    /// **SPI_MEM_FWRITE_DIO** `[14]` (RW)
    ///
    /// In the write operations address phase and read-data phase apply 2 signals.
    #[inline(always)]
    pub fn spi_mem_fwrite_dio(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FWRITE_DIO_MASK) != 0
    }

    /// Set SPI_MEM_FWRITE_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fwrite_dio(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FWRITE_DIO_MASK);
    }

    /// Clear SPI_MEM_FWRITE_DIO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fwrite_dio(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FWRITE_DIO_MASK);
    }

    /// **SPI_MEM_FWRITE_QUAD** `[13]` (RW)
    ///
    /// In the write operations read-data phase apply 4 signals
    #[inline(always)]
    pub fn spi_mem_fwrite_quad(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FWRITE_QUAD_MASK) != 0
    }

    /// Set SPI_MEM_FWRITE_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fwrite_quad(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FWRITE_QUAD_MASK);
    }

    /// Clear SPI_MEM_FWRITE_QUAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fwrite_quad(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FWRITE_QUAD_MASK);
    }

    /// **SPI_MEM_FWRITE_DUAL** `[12]` (RW)
    ///
    /// In the write operations read-data phase apply 2 signals
    #[inline(always)]
    pub fn spi_mem_fwrite_dual(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FWRITE_DUAL_MASK) != 0
    }

    /// Set SPI_MEM_FWRITE_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_fwrite_dual(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FWRITE_DUAL_MASK);
    }

    /// Clear SPI_MEM_FWRITE_DUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_fwrite_dual(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FWRITE_DUAL_MASK);
    }

    /// **SPI_MEM_CK_OUT_EDGE** `[9]` (RW)
    ///
    /// the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x80000000 }
    }

    /// Read **SPI_MEM_USR_COMMAND** `[31:31]`
    ///
    /// This bit enable the command phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_command(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_COMMAND_MASK) != 0
    }

    /// Read **SPI_MEM_USR_ADDR** `[30:30]`
    ///
    /// This bit enable the address phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_addr(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_ADDR_MASK) != 0
    }

    /// Read **SPI_MEM_USR_DUMMY** `[29:29]`
    ///
    /// This bit enable the dummy phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_dummy(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_DUMMY_MASK) != 0
    }

    /// Read **SPI_MEM_USR_MISO** `[28:28]`
    ///
    /// This bit enable the read-data phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_miso(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_MISO_MASK) != 0
    }

    /// Read **SPI_MEM_USR_MOSI** `[27:27]`
    ///
    /// This bit enable the write-data phase of an operation.
    #[inline(always)]
    pub const fn spi_mem_usr_mosi(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_MOSI_MASK) != 0
    }

    /// Read **SPI_MEM_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// SPI clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    pub const fn spi_mem_usr_dummy_idle(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_DUMMY_IDLE_MASK) != 0
    }

    /// Read **SPI_MEM_USR_MOSI_HIGHPART** `[25:25]`
    ///
    /// write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_usr_mosi_highpart(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_MOSI_HIGHPART_MASK) != 0
    }

    /// Read **SPI_MEM_USR_MISO_HIGHPART** `[24:24]`
    ///
    /// read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_usr_miso_highpart(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_USR_MISO_HIGHPART_MASK) != 0
    }

    /// Read **SPI_MEM_FWRITE_QIO** `[15:15]`
    ///
    /// In the write operations address phase and read-data phase apply 4 signals.
    #[inline(always)]
    pub const fn spi_mem_fwrite_qio(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_FWRITE_QIO_MASK) != 0
    }

    /// Read **SPI_MEM_FWRITE_DIO** `[14:14]`
    ///
    /// In the write operations address phase and read-data phase apply 2 signals.
    #[inline(always)]
    pub const fn spi_mem_fwrite_dio(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_FWRITE_DIO_MASK) != 0
    }

    /// Read **SPI_MEM_FWRITE_QUAD** `[13:13]`
    ///
    /// In the write operations read-data phase apply 4 signals
    #[inline(always)]
    pub const fn spi_mem_fwrite_quad(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_FWRITE_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FWRITE_DUAL** `[12:12]`
    ///
    /// In the write operations read-data phase apply 2 signals
    #[inline(always)]
    pub const fn spi_mem_fwrite_dual(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_FWRITE_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_CK_OUT_EDGE** `[9:9]`
    ///
    /// the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
    #[inline(always)]
    pub const fn spi_mem_ck_out_edge(&self) -> bool {
        (self.bits & SpiMemUser::SPI_MEM_CK_OUT_EDGE_MASK) != 0
    }

    /// Set **SPI_MEM_USR_COMMAND** `[31:31]`
    ///
    /// This bit enable the command phase of an operation.
    #[inline(always)]
    pub const fn with_spi_mem_usr_command(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_COMMAND_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_COMMAND_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_ADDR** `[30:30]`
    ///
    /// This bit enable the address phase of an operation.
    #[inline(always)]
    pub const fn with_spi_mem_usr_addr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_ADDR_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_ADDR_MASK
            },
        }
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

    /// Set **SPI_MEM_USR_MISO** `[28:28]`
    ///
    /// This bit enable the read-data phase of an operation.
    #[inline(always)]
    pub const fn with_spi_mem_usr_miso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_MISO_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_MISO_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_MOSI** `[27:27]`
    ///
    /// This bit enable the write-data phase of an operation.
    #[inline(always)]
    pub const fn with_spi_mem_usr_mosi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_MOSI_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_MOSI_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_DUMMY_IDLE** `[26:26]`
    ///
    /// SPI clock is disable in dummy phase when the bit is enable.
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

    /// Set **SPI_MEM_USR_MOSI_HIGHPART** `[25:25]`
    ///
    /// write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_usr_mosi_highpart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_MOSI_HIGHPART_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_MOSI_HIGHPART_MASK
            },
        }
    }

    /// Set **SPI_MEM_USR_MISO_HIGHPART** `[24:24]`
    ///
    /// read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_usr_miso_highpart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_USR_MISO_HIGHPART_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_USR_MISO_HIGHPART_MASK
            },
        }
    }

    /// Set **SPI_MEM_FWRITE_QIO** `[15:15]`
    ///
    /// In the write operations address phase and read-data phase apply 4 signals.
    #[inline(always)]
    pub const fn with_spi_mem_fwrite_qio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_FWRITE_QIO_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_FWRITE_QIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_FWRITE_DIO** `[14:14]`
    ///
    /// In the write operations address phase and read-data phase apply 2 signals.
    #[inline(always)]
    pub const fn with_spi_mem_fwrite_dio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_FWRITE_DIO_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_FWRITE_DIO_MASK
            },
        }
    }

    /// Set **SPI_MEM_FWRITE_QUAD** `[13:13]`
    ///
    /// In the write operations read-data phase apply 4 signals
    #[inline(always)]
    pub const fn with_spi_mem_fwrite_quad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_FWRITE_QUAD_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_FWRITE_QUAD_MASK
            },
        }
    }

    /// Set **SPI_MEM_FWRITE_DUAL** `[12:12]`
    ///
    /// In the write operations read-data phase apply 2 signals
    #[inline(always)]
    pub const fn with_spi_mem_fwrite_dual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemUser::SPI_MEM_FWRITE_DUAL_MASK
            } else {
                self.bits & !SpiMemUser::SPI_MEM_FWRITE_DUAL_MASK
            },
        }
    }

    /// Set **SPI_MEM_CK_OUT_EDGE** `[9:9]`
    ///
    /// the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
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
}

impl Default for SpiMemUserVal {
    fn default() -> Self {
        Self::reset_val()
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
/// SPI1 user1 register.
///
/// Address offset: 0x001c
/// Reset value: 0x5c000007
///
/// ## Fields
///
/// - **SPI_MEM_USR_ADDR_BITLEN** `[31:26]` (RW)
///
///   The length in bits of address phase. The register value shall be (bit_num-1).
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
        Self { bits: 0x5c000007 }
    }

    /// Read **SPI_MEM_USR_ADDR_BITLEN** `[31:26]`
    ///
    /// The length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_usr_addr_bitlen(&self) -> u32 {
        (self.bits & SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_MASK)
            >> SpiMemUser1::SPI_MEM_USR_ADDR_BITLEN_OFFSET
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
/// SPI1 user2 register.
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

/// SPI_MEM_MOSI_DLEN Register
///
/// SPI1 send data bit length control register.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SPI_MEM_USR_MOSI_DBITLEN** `[9:0]` (RW)
///
///   The length in bits of write-data. The register value shall be (bit_num-1).
#[repr(transparent)]
pub struct SpiMemMosiDlen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMosiDlen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMosiDlen {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_MOSI_DBITLEN**
    ///
    /// The length in bits of write-data. The register value shall be (bit_num-1).
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const SPI_MEM_USR_MOSI_DBITLEN_OFFSET: u32 = 0;
    pub const SPI_MEM_USR_MOSI_DBITLEN_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_MOSI_DBITLEN** `[9:0]` (RW)
    ///
    /// The length in bits of write-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_usr_mosi_dbitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_MOSI_DBITLEN_MASK)
            >> Self::SPI_MEM_USR_MOSI_DBITLEN_OFFSET
    }

    /// Write **SPI_MEM_USR_MOSI_DBITLEN** `[9:0]` (RW)
    ///
    /// The length in bits of write-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_usr_mosi_dbitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_MOSI_DBITLEN_MASK)
                | ((value << Self::SPI_MEM_USR_MOSI_DBITLEN_OFFSET)
                    & Self::SPI_MEM_USR_MOSI_DBITLEN_MASK)
        });
    }
}

/// SPI_MEM_MOSI_DLEN value type for batch read/modify/write operations
///
/// Use `spimemmosidlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmosidlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMosiDlenVal {
    bits: u32,
}

impl SpiMemMosiDlenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_USR_MOSI_DBITLEN** `[9:0]`
    ///
    /// The length in bits of write-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_usr_mosi_dbitlen(&self) -> u32 {
        (self.bits & SpiMemMosiDlen::SPI_MEM_USR_MOSI_DBITLEN_MASK)
            >> SpiMemMosiDlen::SPI_MEM_USR_MOSI_DBITLEN_OFFSET
    }

    /// Set **SPI_MEM_USR_MOSI_DBITLEN** `[9:0]`
    ///
    /// The length in bits of write-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_usr_mosi_dbitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemMosiDlen::SPI_MEM_USR_MOSI_DBITLEN_MASK)
                | ((value << SpiMemMosiDlen::SPI_MEM_USR_MOSI_DBITLEN_OFFSET)
                    & SpiMemMosiDlen::SPI_MEM_USR_MOSI_DBITLEN_MASK),
        }
    }
}

impl Default for SpiMemMosiDlenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemMosiDlen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMosiDlenVal {
        SpiMemMosiDlenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMosiDlenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMosiDlenVal) -> SpiMemMosiDlenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_MISO_DLEN Register
///
/// SPI1 receive data bit length control register.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SPI_MEM_USR_MISO_DBITLEN** `[9:0]` (RW)
///
///   The length in bits of read-data. The register value shall be (bit_num-1).
#[repr(transparent)]
pub struct SpiMemMisoDlen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemMisoDlen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemMisoDlen {
    // ======== Field Constants ========

    /// **SPI_MEM_USR_MISO_DBITLEN**
    ///
    /// The length in bits of read-data. The register value shall be (bit_num-1).
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const SPI_MEM_USR_MISO_DBITLEN_OFFSET: u32 = 0;
    pub const SPI_MEM_USR_MISO_DBITLEN_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **SPI_MEM_USR_MISO_DBITLEN** `[9:0]` (RW)
    ///
    /// The length in bits of read-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn spi_mem_usr_miso_dbitlen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_USR_MISO_DBITLEN_MASK)
            >> Self::SPI_MEM_USR_MISO_DBITLEN_OFFSET
    }

    /// Write **SPI_MEM_USR_MISO_DBITLEN** `[9:0]` (RW)
    ///
    /// The length in bits of read-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn write_spi_mem_usr_miso_dbitlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_USR_MISO_DBITLEN_MASK)
                | ((value << Self::SPI_MEM_USR_MISO_DBITLEN_OFFSET)
                    & Self::SPI_MEM_USR_MISO_DBITLEN_MASK)
        });
    }
}

/// SPI_MEM_MISO_DLEN value type for batch read/modify/write operations
///
/// Use `spimemmisodlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemmisodlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemMisoDlenVal {
    bits: u32,
}

impl SpiMemMisoDlenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_USR_MISO_DBITLEN** `[9:0]`
    ///
    /// The length in bits of read-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn spi_mem_usr_miso_dbitlen(&self) -> u32 {
        (self.bits & SpiMemMisoDlen::SPI_MEM_USR_MISO_DBITLEN_MASK)
            >> SpiMemMisoDlen::SPI_MEM_USR_MISO_DBITLEN_OFFSET
    }

    /// Set **SPI_MEM_USR_MISO_DBITLEN** `[9:0]`
    ///
    /// The length in bits of read-data. The register value shall be (bit_num-1).
    #[inline(always)]
    pub const fn with_spi_mem_usr_miso_dbitlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemMisoDlen::SPI_MEM_USR_MISO_DBITLEN_MASK)
                | ((value << SpiMemMisoDlen::SPI_MEM_USR_MISO_DBITLEN_OFFSET)
                    & SpiMemMisoDlen::SPI_MEM_USR_MISO_DBITLEN_MASK),
        }
    }
}

impl Default for SpiMemMisoDlenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemMisoDlen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemMisoDlenVal {
        SpiMemMisoDlenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemMisoDlenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemMisoDlenVal) -> SpiMemMisoDlenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_RD_STATUS Register
///
/// SPI1 status register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SPI_MEM_WB_MODE** `[23:16]` (RW)
///
///   Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
///
/// - **SPI_MEM_STATUS** `[15:0]` (RW)
///
///   The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
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

    /// **SPI_MEM_STATUS**
    ///
    /// The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_MEM_STATUS_OFFSET: u32 = 0;
    pub const SPI_MEM_STATUS_MASK: u32 = 0x0000ffff;

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

    /// **SPI_MEM_STATUS** `[15:0]` (RW)
    ///
    /// The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
    #[inline(always)]
    pub fn spi_mem_status(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_STATUS_MASK) >> Self::SPI_MEM_STATUS_OFFSET
    }

    /// Write **SPI_MEM_STATUS** `[15:0]` (RW)
    ///
    /// The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
    #[inline(always)]
    pub fn write_spi_mem_status(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_STATUS_MASK)
                | ((value << Self::SPI_MEM_STATUS_OFFSET) & Self::SPI_MEM_STATUS_MASK)
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

    /// Read **SPI_MEM_STATUS** `[15:0]`
    ///
    /// The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
    #[inline(always)]
    pub const fn spi_mem_status(&self) -> u32 {
        (self.bits & SpiMemRdStatus::SPI_MEM_STATUS_MASK) >> SpiMemRdStatus::SPI_MEM_STATUS_OFFSET
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

    /// Set **SPI_MEM_STATUS** `[15:0]`
    ///
    /// The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit.
    #[inline(always)]
    pub const fn with_spi_mem_status(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemRdStatus::SPI_MEM_STATUS_MASK)
                | ((value << SpiMemRdStatus::SPI_MEM_STATUS_OFFSET)
                    & SpiMemRdStatus::SPI_MEM_STATUS_MASK),
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
/// SPI1 misc register
///
/// Address offset: 0x0034
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **SPI_MEM_CS_KEEP_ACTIVE** `[10]` (RW)
///
///   spi cs line keep low when the bit is set.
///
/// - **SPI_MEM_CK_IDLE_EDGE** `[9]` (RW)
///
///   1: spi clk line is high when idle 0: spi clk line is low when idle
///
/// - **SPI_MEM_CS1_DIS** `[1]` (RW)
///
///   SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on.
///
/// - **SPI_MEM_CS0_DIS** `[0]` (RW)
///
///   SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on.
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
    /// spi cs line keep low when the bit is set.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_CS_KEEP_ACTIVE_OFFSET: u32 = 10;
    pub const SPI_MEM_CS_KEEP_ACTIVE_MASK: u32 = 0x00000400;
    pub const SPI_MEM_CS_KEEP_ACTIVE: u32 = 1 << 10;

    /// **SPI_MEM_CK_IDLE_EDGE**
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_CK_IDLE_EDGE_OFFSET: u32 = 9;
    pub const SPI_MEM_CK_IDLE_EDGE_MASK: u32 = 0x00000200;
    pub const SPI_MEM_CK_IDLE_EDGE: u32 = 1 << 9;

    /// **SPI_MEM_CS1_DIS**
    ///
    /// SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_CS1_DIS_OFFSET: u32 = 1;
    pub const SPI_MEM_CS1_DIS_MASK: u32 = 0x00000002;
    pub const SPI_MEM_CS1_DIS: u32 = 1 << 1;

    /// **SPI_MEM_CS0_DIS**
    ///
    /// SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_CS0_DIS_OFFSET: u32 = 0;
    pub const SPI_MEM_CS0_DIS_MASK: u32 = 0x00000001;
    pub const SPI_MEM_CS0_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_CS_KEEP_ACTIVE** `[10]` (RW)
    ///
    /// spi cs line keep low when the bit is set.
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
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle
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

    /// **SPI_MEM_CS1_DIS** `[1]` (RW)
    ///
    /// SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub fn spi_mem_cs1_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CS1_DIS_MASK) != 0
    }

    /// Set SPI_MEM_CS1_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cs1_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_CS1_DIS_MASK);
    }

    /// Clear SPI_MEM_CS1_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cs1_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_CS1_DIS_MASK);
    }

    /// **SPI_MEM_CS0_DIS** `[0]` (RW)
    ///
    /// SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub fn spi_mem_cs0_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CS0_DIS_MASK) != 0
    }

    /// Set SPI_MEM_CS0_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_cs0_dis(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_CS0_DIS_MASK);
    }

    /// Clear SPI_MEM_CS0_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_cs0_dis(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_CS0_DIS_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000002 }
    }

    /// Read **SPI_MEM_CS_KEEP_ACTIVE** `[10:10]`
    ///
    /// spi cs line keep low when the bit is set.
    #[inline(always)]
    pub const fn spi_mem_cs_keep_active(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CS_KEEP_ACTIVE_MASK) != 0
    }

    /// Read **SPI_MEM_CK_IDLE_EDGE** `[9:9]`
    ///
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    pub const fn spi_mem_ck_idle_edge(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CK_IDLE_EDGE_MASK) != 0
    }

    /// Read **SPI_MEM_CS1_DIS** `[1:1]`
    ///
    /// SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub const fn spi_mem_cs1_dis(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CS1_DIS_MASK) != 0
    }

    /// Read **SPI_MEM_CS0_DIS** `[0:0]`
    ///
    /// SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub const fn spi_mem_cs0_dis(&self) -> bool {
        (self.bits & SpiMemMisc::SPI_MEM_CS0_DIS_MASK) != 0
    }

    /// Set **SPI_MEM_CS_KEEP_ACTIVE** `[10:10]`
    ///
    /// spi cs line keep low when the bit is set.
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
    /// 1: spi clk line is high when idle 0: spi clk line is low when idle
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

    /// Set **SPI_MEM_CS1_DIS** `[1:1]`
    ///
    /// SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub const fn with_spi_mem_cs1_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_CS1_DIS_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_CS1_DIS_MASK
            },
        }
    }

    /// Set **SPI_MEM_CS0_DIS** `[0:0]`
    ///
    /// SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on.
    #[inline(always)]
    pub const fn with_spi_mem_cs0_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemMisc::SPI_MEM_CS0_DIS_MASK
            } else {
                self.bits & !SpiMemMisc::SPI_MEM_CS0_DIS_MASK
            },
        }
    }
}

impl Default for SpiMemMiscVal {
    fn default() -> Self {
        Self::reset_val()
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

/// SPI_MEM_TX_CRC Register
///
/// SPI1 TX CRC data register.
///
/// Address offset: 0x0038
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **DATA** `[31:0]` (RO)
///
///   For SPI1, the value of crc32.
#[repr(transparent)]
pub struct SpiMemTxCrc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemTxCrc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemTxCrc {
    // ======== Field Constants ========

    /// **DATA**
    ///
    /// For SPI1, the value of crc32.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATA_OFFSET: u32 = 0;
    pub const DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATA** `[31:0]` (RO)
    ///
    /// For SPI1, the value of crc32.
    #[inline(always)]
    pub fn data(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_MASK) >> Self::DATA_OFFSET
    }
}

/// SPI_MEM_TX_CRC value type for batch read/modify/write operations
///
/// Use `spimemtxcrc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemtxcrc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemTxCrcVal {
    bits: u32,
}

impl SpiMemTxCrcVal {
    /// Create from raw bits
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

    /// Read **DATA** `[31:0]`
    ///
    /// For SPI1, the value of crc32.
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        (self.bits & SpiMemTxCrc::DATA_MASK) >> SpiMemTxCrc::DATA_OFFSET
    }
}

impl Default for SpiMemTxCrcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemTxCrc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemTxCrcVal {
        SpiMemTxCrcVal::from_bits(self.value.read_bits())
    }
}

/// SPI_MEM_CACHE_FCTRL Register
///
/// SPI1 bit mode control register.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **SPI_MEM_FADDR_QUAD** `[8]` (RW)
///
///   For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FDOUT_QUAD** `[7]` (RW)
///
///   For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FDIN_QUAD** `[6]` (RW)
///
///   For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
///
/// - **SPI_MEM_FADDR_DUAL** `[5]` (RW)
///
///   For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_FDOUT_DUAL** `[4]` (RW)
///
///   For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_FDIN_DUAL** `[3]` (RW)
///
///   For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
///
/// - **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1]` (RW)
///
///   For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable.
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

    /// **SPI_MEM_FADDR_QUAD**
    ///
    /// For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SPI_MEM_FADDR_QUAD_OFFSET: u32 = 8;
    pub const SPI_MEM_FADDR_QUAD_MASK: u32 = 0x00000100;
    pub const SPI_MEM_FADDR_QUAD: u32 = 1 << 8;

    /// **SPI_MEM_FDOUT_QUAD**
    ///
    /// For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_FDOUT_QUAD_OFFSET: u32 = 7;
    pub const SPI_MEM_FDOUT_QUAD_MASK: u32 = 0x00000080;
    pub const SPI_MEM_FDOUT_QUAD: u32 = 1 << 7;

    /// **SPI_MEM_FDIN_QUAD**
    ///
    /// For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_FDIN_QUAD_OFFSET: u32 = 6;
    pub const SPI_MEM_FDIN_QUAD_MASK: u32 = 0x00000040;
    pub const SPI_MEM_FDIN_QUAD: u32 = 1 << 6;

    /// **SPI_MEM_FADDR_DUAL**
    ///
    /// For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_FADDR_DUAL_OFFSET: u32 = 5;
    pub const SPI_MEM_FADDR_DUAL_MASK: u32 = 0x00000020;
    pub const SPI_MEM_FADDR_DUAL: u32 = 1 << 5;

    /// **SPI_MEM_FDOUT_DUAL**
    ///
    /// For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_FDOUT_DUAL_OFFSET: u32 = 4;
    pub const SPI_MEM_FDOUT_DUAL_MASK: u32 = 0x00000010;
    pub const SPI_MEM_FDOUT_DUAL: u32 = 1 << 4;

    /// **SPI_MEM_FDIN_DUAL**
    ///
    /// For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FDIN_DUAL_OFFSET: u32 = 3;
    pub const SPI_MEM_FDIN_DUAL_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FDIN_DUAL: u32 = 1 << 3;

    /// **SPI_MEM_CACHE_USR_ADDR_4BYTE**
    ///
    /// For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE_OFFSET: u32 = 1;
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK: u32 = 0x00000002;
    pub const SPI_MEM_CACHE_USR_ADDR_4BYTE: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **SPI_MEM_FADDR_QUAD** `[8]` (RW)
    ///
    /// For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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
    /// For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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
    /// For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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

    /// **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1]` (RW)
    ///
    /// For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable.
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

    /// Read **SPI_MEM_FADDR_QUAD** `[8:8]`
    ///
    /// For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_faddr_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FADDR_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FDOUT_QUAD** `[7:7]`
    ///
    /// For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_fdout_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDOUT_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FDIN_QUAD** `[6:6]`
    ///
    /// For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
    #[inline(always)]
    pub const fn spi_mem_fdin_quad(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDIN_QUAD_MASK) != 0
    }

    /// Read **SPI_MEM_FADDR_DUAL** `[5:5]`
    ///
    /// For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_faddr_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FADDR_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_FDOUT_DUAL** `[4:4]`
    ///
    /// For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_fdout_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDOUT_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_FDIN_DUAL** `[3:3]`
    ///
    /// For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
    #[inline(always)]
    pub const fn spi_mem_fdin_dual(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_FDIN_DUAL_MASK) != 0
    }

    /// Read **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1:1]`
    ///
    /// For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable.
    #[inline(always)]
    pub const fn spi_mem_cache_usr_addr_4byte(&self) -> bool {
        (self.bits & SpiMemCacheFctrl::SPI_MEM_CACHE_USR_ADDR_4BYTE_MASK) != 0
    }

    /// Set **SPI_MEM_FADDR_QUAD** `[8:8]`
    ///
    /// For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio.
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
    /// For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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
    /// For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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
    /// For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio.
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

    /// Set **SPI_MEM_CACHE_USR_ADDR_4BYTE** `[1:1]`
    ///
    /// For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable.
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
}

impl Default for SpiMemCacheFctrlVal {
    fn default() -> Self {
        Self { bits: 0 }
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

/// SPI_MEM_W0 Register
///
/// SPI1 memory data buffer0
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **SPI_MEM_BUF0** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW0 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF0**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF0_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF0** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf0(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF0_MASK) >> Self::SPI_MEM_BUF0_OFFSET
    }

    /// Write **SPI_MEM_BUF0** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF0_MASK)
                | ((value << Self::SPI_MEM_BUF0_OFFSET) & Self::SPI_MEM_BUF0_MASK)
        });
    }
}

/// SPI_MEM_W0 value type for batch read/modify/write operations
///
/// Use `spimemw0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW0Val {
    bits: u32,
}

impl SpiMemW0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF0** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf0(&self) -> u32 {
        (self.bits & SpiMemW0::SPI_MEM_BUF0_MASK) >> SpiMemW0::SPI_MEM_BUF0_OFFSET
    }

    /// Set **SPI_MEM_BUF0** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW0::SPI_MEM_BUF0_MASK)
                | ((value << SpiMemW0::SPI_MEM_BUF0_OFFSET) & SpiMemW0::SPI_MEM_BUF0_MASK),
        }
    }
}

impl Default for SpiMemW0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW0Val {
        SpiMemW0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW0Val) -> SpiMemW0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W1 Register
///
/// SPI1 memory data buffer1
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **SPI_MEM_BUF1** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW1 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF1**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF1_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF1** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf1(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF1_MASK) >> Self::SPI_MEM_BUF1_OFFSET
    }

    /// Write **SPI_MEM_BUF1** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF1_MASK)
                | ((value << Self::SPI_MEM_BUF1_OFFSET) & Self::SPI_MEM_BUF1_MASK)
        });
    }
}

/// SPI_MEM_W1 value type for batch read/modify/write operations
///
/// Use `spimemw1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW1Val {
    bits: u32,
}

impl SpiMemW1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF1** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf1(&self) -> u32 {
        (self.bits & SpiMemW1::SPI_MEM_BUF1_MASK) >> SpiMemW1::SPI_MEM_BUF1_OFFSET
    }

    /// Set **SPI_MEM_BUF1** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW1::SPI_MEM_BUF1_MASK)
                | ((value << SpiMemW1::SPI_MEM_BUF1_OFFSET) & SpiMemW1::SPI_MEM_BUF1_MASK),
        }
    }
}

impl Default for SpiMemW1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW1Val {
        SpiMemW1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW1Val) -> SpiMemW1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W2 Register
///
/// SPI1 memory data buffer2
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **SPI_MEM_BUF2** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW2 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF2**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF2_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF2** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf2(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF2_MASK) >> Self::SPI_MEM_BUF2_OFFSET
    }

    /// Write **SPI_MEM_BUF2** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF2_MASK)
                | ((value << Self::SPI_MEM_BUF2_OFFSET) & Self::SPI_MEM_BUF2_MASK)
        });
    }
}

/// SPI_MEM_W2 value type for batch read/modify/write operations
///
/// Use `spimemw2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW2Val {
    bits: u32,
}

impl SpiMemW2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF2** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf2(&self) -> u32 {
        (self.bits & SpiMemW2::SPI_MEM_BUF2_MASK) >> SpiMemW2::SPI_MEM_BUF2_OFFSET
    }

    /// Set **SPI_MEM_BUF2** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW2::SPI_MEM_BUF2_MASK)
                | ((value << SpiMemW2::SPI_MEM_BUF2_OFFSET) & SpiMemW2::SPI_MEM_BUF2_MASK),
        }
    }
}

impl Default for SpiMemW2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW2Val {
        SpiMemW2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW2Val) -> SpiMemW2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W3 Register
///
/// SPI1 memory data buffer3
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **SPI_MEM_BUF3** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW3 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF3**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF3_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF3** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf3(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF3_MASK) >> Self::SPI_MEM_BUF3_OFFSET
    }

    /// Write **SPI_MEM_BUF3** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF3_MASK)
                | ((value << Self::SPI_MEM_BUF3_OFFSET) & Self::SPI_MEM_BUF3_MASK)
        });
    }
}

/// SPI_MEM_W3 value type for batch read/modify/write operations
///
/// Use `spimemw3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW3Val {
    bits: u32,
}

impl SpiMemW3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF3** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf3(&self) -> u32 {
        (self.bits & SpiMemW3::SPI_MEM_BUF3_MASK) >> SpiMemW3::SPI_MEM_BUF3_OFFSET
    }

    /// Set **SPI_MEM_BUF3** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW3::SPI_MEM_BUF3_MASK)
                | ((value << SpiMemW3::SPI_MEM_BUF3_OFFSET) & SpiMemW3::SPI_MEM_BUF3_MASK),
        }
    }
}

impl Default for SpiMemW3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW3Val {
        SpiMemW3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW3Val) -> SpiMemW3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W4 Register
///
/// SPI1 memory data buffer4
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SPI_MEM_BUF4** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW4 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF4**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF4_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF4** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf4(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF4_MASK) >> Self::SPI_MEM_BUF4_OFFSET
    }

    /// Write **SPI_MEM_BUF4** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF4_MASK)
                | ((value << Self::SPI_MEM_BUF4_OFFSET) & Self::SPI_MEM_BUF4_MASK)
        });
    }
}

/// SPI_MEM_W4 value type for batch read/modify/write operations
///
/// Use `spimemw4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW4Val {
    bits: u32,
}

impl SpiMemW4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF4** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf4(&self) -> u32 {
        (self.bits & SpiMemW4::SPI_MEM_BUF4_MASK) >> SpiMemW4::SPI_MEM_BUF4_OFFSET
    }

    /// Set **SPI_MEM_BUF4** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW4::SPI_MEM_BUF4_MASK)
                | ((value << SpiMemW4::SPI_MEM_BUF4_OFFSET) & SpiMemW4::SPI_MEM_BUF4_MASK),
        }
    }
}

impl Default for SpiMemW4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW4Val {
        SpiMemW4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW4Val) -> SpiMemW4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W5 Register
///
/// SPI1 memory data buffer5
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **SPI_MEM_BUF5** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW5 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF5**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF5_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF5** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf5(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF5_MASK) >> Self::SPI_MEM_BUF5_OFFSET
    }

    /// Write **SPI_MEM_BUF5** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF5_MASK)
                | ((value << Self::SPI_MEM_BUF5_OFFSET) & Self::SPI_MEM_BUF5_MASK)
        });
    }
}

/// SPI_MEM_W5 value type for batch read/modify/write operations
///
/// Use `spimemw5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW5Val {
    bits: u32,
}

impl SpiMemW5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF5** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf5(&self) -> u32 {
        (self.bits & SpiMemW5::SPI_MEM_BUF5_MASK) >> SpiMemW5::SPI_MEM_BUF5_OFFSET
    }

    /// Set **SPI_MEM_BUF5** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW5::SPI_MEM_BUF5_MASK)
                | ((value << SpiMemW5::SPI_MEM_BUF5_OFFSET) & SpiMemW5::SPI_MEM_BUF5_MASK),
        }
    }
}

impl Default for SpiMemW5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW5Val {
        SpiMemW5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW5Val) -> SpiMemW5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W6 Register
///
/// SPI1 memory data buffer6
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **SPI_MEM_BUF6** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW6 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF6**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF6_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF6** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf6(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF6_MASK) >> Self::SPI_MEM_BUF6_OFFSET
    }

    /// Write **SPI_MEM_BUF6** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF6_MASK)
                | ((value << Self::SPI_MEM_BUF6_OFFSET) & Self::SPI_MEM_BUF6_MASK)
        });
    }
}

/// SPI_MEM_W6 value type for batch read/modify/write operations
///
/// Use `spimemw6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW6Val {
    bits: u32,
}

impl SpiMemW6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF6** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf6(&self) -> u32 {
        (self.bits & SpiMemW6::SPI_MEM_BUF6_MASK) >> SpiMemW6::SPI_MEM_BUF6_OFFSET
    }

    /// Set **SPI_MEM_BUF6** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW6::SPI_MEM_BUF6_MASK)
                | ((value << SpiMemW6::SPI_MEM_BUF6_OFFSET) & SpiMemW6::SPI_MEM_BUF6_MASK),
        }
    }
}

impl Default for SpiMemW6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW6Val {
        SpiMemW6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW6Val) -> SpiMemW6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W7 Register
///
/// SPI1 memory data buffer7
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **SPI_MEM_BUF7** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW7 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF7**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF7_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF7** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf7(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF7_MASK) >> Self::SPI_MEM_BUF7_OFFSET
    }

    /// Write **SPI_MEM_BUF7** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF7_MASK)
                | ((value << Self::SPI_MEM_BUF7_OFFSET) & Self::SPI_MEM_BUF7_MASK)
        });
    }
}

/// SPI_MEM_W7 value type for batch read/modify/write operations
///
/// Use `spimemw7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW7Val {
    bits: u32,
}

impl SpiMemW7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF7** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf7(&self) -> u32 {
        (self.bits & SpiMemW7::SPI_MEM_BUF7_MASK) >> SpiMemW7::SPI_MEM_BUF7_OFFSET
    }

    /// Set **SPI_MEM_BUF7** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW7::SPI_MEM_BUF7_MASK)
                | ((value << SpiMemW7::SPI_MEM_BUF7_OFFSET) & SpiMemW7::SPI_MEM_BUF7_MASK),
        }
    }
}

impl Default for SpiMemW7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW7Val {
        SpiMemW7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW7Val) -> SpiMemW7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W8 Register
///
/// SPI1 memory data buffer8
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **SPI_MEM_BUF8** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW8 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF8**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF8_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF8** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf8(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF8_MASK) >> Self::SPI_MEM_BUF8_OFFSET
    }

    /// Write **SPI_MEM_BUF8** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF8_MASK)
                | ((value << Self::SPI_MEM_BUF8_OFFSET) & Self::SPI_MEM_BUF8_MASK)
        });
    }
}

/// SPI_MEM_W8 value type for batch read/modify/write operations
///
/// Use `spimemw8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW8Val {
    bits: u32,
}

impl SpiMemW8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF8** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf8(&self) -> u32 {
        (self.bits & SpiMemW8::SPI_MEM_BUF8_MASK) >> SpiMemW8::SPI_MEM_BUF8_OFFSET
    }

    /// Set **SPI_MEM_BUF8** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW8::SPI_MEM_BUF8_MASK)
                | ((value << SpiMemW8::SPI_MEM_BUF8_OFFSET) & SpiMemW8::SPI_MEM_BUF8_MASK),
        }
    }
}

impl Default for SpiMemW8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW8Val {
        SpiMemW8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW8Val) -> SpiMemW8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W9 Register
///
/// SPI1 memory data buffer9
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **SPI_MEM_BUF9** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW9 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF9**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF9_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF9** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf9(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF9_MASK) >> Self::SPI_MEM_BUF9_OFFSET
    }

    /// Write **SPI_MEM_BUF9** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF9_MASK)
                | ((value << Self::SPI_MEM_BUF9_OFFSET) & Self::SPI_MEM_BUF9_MASK)
        });
    }
}

/// SPI_MEM_W9 value type for batch read/modify/write operations
///
/// Use `spimemw9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW9Val {
    bits: u32,
}

impl SpiMemW9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF9** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf9(&self) -> u32 {
        (self.bits & SpiMemW9::SPI_MEM_BUF9_MASK) >> SpiMemW9::SPI_MEM_BUF9_OFFSET
    }

    /// Set **SPI_MEM_BUF9** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW9::SPI_MEM_BUF9_MASK)
                | ((value << SpiMemW9::SPI_MEM_BUF9_OFFSET) & SpiMemW9::SPI_MEM_BUF9_MASK),
        }
    }
}

impl Default for SpiMemW9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW9Val {
        SpiMemW9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW9Val) -> SpiMemW9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W10 Register
///
/// SPI1 memory data buffer10
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **SPI_MEM_BUF10** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW10 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF10**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF10_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF10** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf10(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF10_MASK) >> Self::SPI_MEM_BUF10_OFFSET
    }

    /// Write **SPI_MEM_BUF10** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF10_MASK)
                | ((value << Self::SPI_MEM_BUF10_OFFSET) & Self::SPI_MEM_BUF10_MASK)
        });
    }
}

/// SPI_MEM_W10 value type for batch read/modify/write operations
///
/// Use `spimemw10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW10Val {
    bits: u32,
}

impl SpiMemW10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF10** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf10(&self) -> u32 {
        (self.bits & SpiMemW10::SPI_MEM_BUF10_MASK) >> SpiMemW10::SPI_MEM_BUF10_OFFSET
    }

    /// Set **SPI_MEM_BUF10** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW10::SPI_MEM_BUF10_MASK)
                | ((value << SpiMemW10::SPI_MEM_BUF10_OFFSET) & SpiMemW10::SPI_MEM_BUF10_MASK),
        }
    }
}

impl Default for SpiMemW10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW10Val {
        SpiMemW10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW10Val) -> SpiMemW10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W11 Register
///
/// SPI1 memory data buffer11
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **SPI_MEM_BUF11** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW11 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF11**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF11_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF11** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf11(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF11_MASK) >> Self::SPI_MEM_BUF11_OFFSET
    }

    /// Write **SPI_MEM_BUF11** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF11_MASK)
                | ((value << Self::SPI_MEM_BUF11_OFFSET) & Self::SPI_MEM_BUF11_MASK)
        });
    }
}

/// SPI_MEM_W11 value type for batch read/modify/write operations
///
/// Use `spimemw11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW11Val {
    bits: u32,
}

impl SpiMemW11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF11** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf11(&self) -> u32 {
        (self.bits & SpiMemW11::SPI_MEM_BUF11_MASK) >> SpiMemW11::SPI_MEM_BUF11_OFFSET
    }

    /// Set **SPI_MEM_BUF11** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW11::SPI_MEM_BUF11_MASK)
                | ((value << SpiMemW11::SPI_MEM_BUF11_OFFSET) & SpiMemW11::SPI_MEM_BUF11_MASK),
        }
    }
}

impl Default for SpiMemW11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW11Val {
        SpiMemW11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW11Val) -> SpiMemW11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W12 Register
///
/// SPI1 memory data buffer12
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **SPI_MEM_BUF12** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW12 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF12**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF12_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF12_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF12** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf12(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF12_MASK) >> Self::SPI_MEM_BUF12_OFFSET
    }

    /// Write **SPI_MEM_BUF12** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF12_MASK)
                | ((value << Self::SPI_MEM_BUF12_OFFSET) & Self::SPI_MEM_BUF12_MASK)
        });
    }
}

/// SPI_MEM_W12 value type for batch read/modify/write operations
///
/// Use `spimemw12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW12Val {
    bits: u32,
}

impl SpiMemW12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF12** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf12(&self) -> u32 {
        (self.bits & SpiMemW12::SPI_MEM_BUF12_MASK) >> SpiMemW12::SPI_MEM_BUF12_OFFSET
    }

    /// Set **SPI_MEM_BUF12** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW12::SPI_MEM_BUF12_MASK)
                | ((value << SpiMemW12::SPI_MEM_BUF12_OFFSET) & SpiMemW12::SPI_MEM_BUF12_MASK),
        }
    }
}

impl Default for SpiMemW12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW12Val {
        SpiMemW12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW12Val) -> SpiMemW12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W13 Register
///
/// SPI1 memory data buffer13
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **SPI_MEM_BUF13** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW13 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF13**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF13_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF13_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF13** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf13(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF13_MASK) >> Self::SPI_MEM_BUF13_OFFSET
    }

    /// Write **SPI_MEM_BUF13** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF13_MASK)
                | ((value << Self::SPI_MEM_BUF13_OFFSET) & Self::SPI_MEM_BUF13_MASK)
        });
    }
}

/// SPI_MEM_W13 value type for batch read/modify/write operations
///
/// Use `spimemw13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW13Val {
    bits: u32,
}

impl SpiMemW13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF13** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf13(&self) -> u32 {
        (self.bits & SpiMemW13::SPI_MEM_BUF13_MASK) >> SpiMemW13::SPI_MEM_BUF13_OFFSET
    }

    /// Set **SPI_MEM_BUF13** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW13::SPI_MEM_BUF13_MASK)
                | ((value << SpiMemW13::SPI_MEM_BUF13_OFFSET) & SpiMemW13::SPI_MEM_BUF13_MASK),
        }
    }
}

impl Default for SpiMemW13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW13Val {
        SpiMemW13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW13Val) -> SpiMemW13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W14 Register
///
/// SPI1 memory data buffer14
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **SPI_MEM_BUF14** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW14 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF14**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF14_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF14_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF14** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf14(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF14_MASK) >> Self::SPI_MEM_BUF14_OFFSET
    }

    /// Write **SPI_MEM_BUF14** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF14_MASK)
                | ((value << Self::SPI_MEM_BUF14_OFFSET) & Self::SPI_MEM_BUF14_MASK)
        });
    }
}

/// SPI_MEM_W14 value type for batch read/modify/write operations
///
/// Use `spimemw14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW14Val {
    bits: u32,
}

impl SpiMemW14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF14** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf14(&self) -> u32 {
        (self.bits & SpiMemW14::SPI_MEM_BUF14_MASK) >> SpiMemW14::SPI_MEM_BUF14_OFFSET
    }

    /// Set **SPI_MEM_BUF14** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW14::SPI_MEM_BUF14_MASK)
                | ((value << SpiMemW14::SPI_MEM_BUF14_OFFSET) & SpiMemW14::SPI_MEM_BUF14_MASK),
        }
    }
}

impl Default for SpiMemW14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW14Val {
        SpiMemW14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW14Val) -> SpiMemW14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_W15 Register
///
/// SPI1 memory data buffer15
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **SPI_MEM_BUF15** `[31:0]` (RW)
///
///   data buffer
#[repr(transparent)]
pub struct SpiMemW15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemW15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemW15 {
    // ======== Field Constants ========

    /// **SPI_MEM_BUF15**
    ///
    /// data buffer
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPI_MEM_BUF15_OFFSET: u32 = 0;
    pub const SPI_MEM_BUF15_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_BUF15** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn spi_mem_buf15(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_BUF15_MASK) >> Self::SPI_MEM_BUF15_OFFSET
    }

    /// Write **SPI_MEM_BUF15** `[31:0]` (RW)
    ///
    /// data buffer
    #[inline(always)]
    pub fn write_spi_mem_buf15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_BUF15_MASK)
                | ((value << Self::SPI_MEM_BUF15_OFFSET) & Self::SPI_MEM_BUF15_MASK)
        });
    }
}

/// SPI_MEM_W15 value type for batch read/modify/write operations
///
/// Use `spimemw15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemw15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemW15Val {
    bits: u32,
}

impl SpiMemW15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_MEM_BUF15** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn spi_mem_buf15(&self) -> u32 {
        (self.bits & SpiMemW15::SPI_MEM_BUF15_MASK) >> SpiMemW15::SPI_MEM_BUF15_OFFSET
    }

    /// Set **SPI_MEM_BUF15** `[31:0]`
    ///
    /// data buffer
    #[inline(always)]
    pub const fn with_spi_mem_buf15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemW15::SPI_MEM_BUF15_MASK)
                | ((value << SpiMemW15::SPI_MEM_BUF15_OFFSET) & SpiMemW15::SPI_MEM_BUF15_MASK),
        }
    }
}

impl Default for SpiMemW15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SpiMemW15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemW15Val {
        SpiMemW15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemW15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemW15Val) -> SpiMemW15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_FLASH_WAITI_CTRL Register
///
/// SPI1 wait idle control register
///
/// Address offset: 0x0098
/// Reset value: 0x00050001
///
/// ## Fields
///
/// - **SPI_MEM_WAITI_CMD** `[31:16]` (RW)
///
///   The command value to wait flash idle(RDSR).
///
/// - **SPI_MEM_WAITI_DUMMY_CYCLELEN** `[15:10]` (RW)
///
///   The dummy cycle length when wait flash idle(RDSR).
///
/// - **SPI_MEM_WAITI_CMD_2B** `[9]` (RW)
///
///   1:The wait idle command bit length is 16. 0: The wait idle command bit length is 8.
///
/// - **SPI_MEM_WAITI_ADDR_CYCLELEN** `[4:3]` (RW)
///
///   When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
///
/// - **SPI_MEM_WAITI_ADDR_EN** `[2]` (RW)
///
///   1: Output address 0 in RDSR or read SUS command transfer. 0: Do not send out address in RDSR or read SUS command transfer.
///
/// - **SPI_MEM_WAITI_DUMMY** `[1]` (RW)
///
///   The dummy phase enable when wait flash idle (RDSR)
///
/// - **SPI_MEM_WAITI_EN** `[0]` (RW)
///
///   1: The hardware will wait idle after SE/PP/WRSR automatically, and hardware auto Suspend/Resume can be enabled. 0: The functions of hardware wait idle and auto Suspend/Resume are not supported.
#[repr(transparent)]
pub struct SpiMemFlashWaitiCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemFlashWaitiCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemFlashWaitiCtrl {
    // ======== Field Constants ========

    /// **SPI_MEM_WAITI_CMD**
    ///
    /// The command value to wait flash idle(RDSR).
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SPI_MEM_WAITI_CMD_OFFSET: u32 = 16;
    pub const SPI_MEM_WAITI_CMD_MASK: u32 = 0xffff0000;

    /// **SPI_MEM_WAITI_DUMMY_CYCLELEN**
    ///
    /// The dummy cycle length when wait flash idle(RDSR).
    ///
    /// Bits `[15:10]`, mask `0x0000fc00`
    pub const SPI_MEM_WAITI_DUMMY_CYCLELEN_OFFSET: u32 = 10;
    pub const SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK: u32 = 0x0000fc00;

    /// **SPI_MEM_WAITI_CMD_2B**
    ///
    /// 1:The wait idle command bit length is 16. 0: The wait idle command bit length is 8.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPI_MEM_WAITI_CMD_2B_OFFSET: u32 = 9;
    pub const SPI_MEM_WAITI_CMD_2B_MASK: u32 = 0x00000200;
    pub const SPI_MEM_WAITI_CMD_2B: u32 = 1 << 9;

    /// **SPI_MEM_WAITI_ADDR_CYCLELEN**
    ///
    /// When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const SPI_MEM_WAITI_ADDR_CYCLELEN_OFFSET: u32 = 3;
    pub const SPI_MEM_WAITI_ADDR_CYCLELEN_MASK: u32 = 0x00000018;

    /// **SPI_MEM_WAITI_ADDR_EN**
    ///
    /// 1: Output address 0 in RDSR or read SUS command transfer. 0: Do not send out address in RDSR or read SUS command transfer.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_WAITI_ADDR_EN_OFFSET: u32 = 2;
    pub const SPI_MEM_WAITI_ADDR_EN_MASK: u32 = 0x00000004;
    pub const SPI_MEM_WAITI_ADDR_EN: u32 = 1 << 2;

    /// **SPI_MEM_WAITI_DUMMY**
    ///
    /// The dummy phase enable when wait flash idle (RDSR)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_WAITI_DUMMY_OFFSET: u32 = 1;
    pub const SPI_MEM_WAITI_DUMMY_MASK: u32 = 0x00000002;
    pub const SPI_MEM_WAITI_DUMMY: u32 = 1 << 1;

    /// **SPI_MEM_WAITI_EN**
    ///
    /// 1: The hardware will wait idle after SE/PP/WRSR automatically, and hardware auto Suspend/Resume can be enabled. 0: The functions of hardware wait idle and auto Suspend/Resume are not supported.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_WAITI_EN_OFFSET: u32 = 0;
    pub const SPI_MEM_WAITI_EN_MASK: u32 = 0x00000001;
    pub const SPI_MEM_WAITI_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_WAITI_CMD** `[31:16]` (RW)
    ///
    /// The command value to wait flash idle(RDSR).
    #[inline(always)]
    pub fn spi_mem_waiti_cmd(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_CMD_MASK) >> Self::SPI_MEM_WAITI_CMD_OFFSET
    }

    /// Write **SPI_MEM_WAITI_CMD** `[31:16]` (RW)
    ///
    /// The command value to wait flash idle(RDSR).
    #[inline(always)]
    pub fn write_spi_mem_waiti_cmd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_WAITI_CMD_MASK)
                | ((value << Self::SPI_MEM_WAITI_CMD_OFFSET) & Self::SPI_MEM_WAITI_CMD_MASK)
        });
    }

    /// **SPI_MEM_WAITI_DUMMY_CYCLELEN** `[15:10]` (RW)
    ///
    /// The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    pub fn spi_mem_waiti_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK)
            >> Self::SPI_MEM_WAITI_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_WAITI_DUMMY_CYCLELEN** `[15:10]` (RW)
    ///
    /// The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    pub fn write_spi_mem_waiti_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_WAITI_DUMMY_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK)
        });
    }

    /// **SPI_MEM_WAITI_CMD_2B** `[9]` (RW)
    ///
    /// 1:The wait idle command bit length is 16. 0: The wait idle command bit length is 8.
    #[inline(always)]
    pub fn spi_mem_waiti_cmd_2b(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_CMD_2B_MASK) != 0
    }

    /// Set SPI_MEM_WAITI_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_waiti_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WAITI_CMD_2B_MASK);
    }

    /// Clear SPI_MEM_WAITI_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_waiti_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WAITI_CMD_2B_MASK);
    }

    /// **SPI_MEM_WAITI_ADDR_CYCLELEN** `[4:3]` (RW)
    ///
    /// When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
    #[inline(always)]
    pub fn spi_mem_waiti_addr_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK)
            >> Self::SPI_MEM_WAITI_ADDR_CYCLELEN_OFFSET
    }

    /// Write **SPI_MEM_WAITI_ADDR_CYCLELEN** `[4:3]` (RW)
    ///
    /// When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
    #[inline(always)]
    pub fn write_spi_mem_waiti_addr_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK)
                | ((value << Self::SPI_MEM_WAITI_ADDR_CYCLELEN_OFFSET)
                    & Self::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK)
        });
    }

    /// **SPI_MEM_WAITI_ADDR_EN** `[2]` (RW)
    ///
    /// 1: Output address 0 in RDSR or read SUS command transfer. 0: Do not send out address in RDSR or read SUS command transfer.
    #[inline(always)]
    pub fn spi_mem_waiti_addr_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_ADDR_EN_MASK) != 0
    }

    /// Set SPI_MEM_WAITI_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_waiti_addr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WAITI_ADDR_EN_MASK);
    }

    /// Clear SPI_MEM_WAITI_ADDR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_waiti_addr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WAITI_ADDR_EN_MASK);
    }

    /// **SPI_MEM_WAITI_DUMMY** `[1]` (RW)
    ///
    /// The dummy phase enable when wait flash idle (RDSR)
    #[inline(always)]
    pub fn spi_mem_waiti_dummy(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_DUMMY_MASK) != 0
    }

    /// Set SPI_MEM_WAITI_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_waiti_dummy(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WAITI_DUMMY_MASK);
    }

    /// Clear SPI_MEM_WAITI_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_waiti_dummy(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WAITI_DUMMY_MASK);
    }

    /// **SPI_MEM_WAITI_EN** `[0]` (RW)
    ///
    /// 1: The hardware will wait idle after SE/PP/WRSR automatically, and hardware auto Suspend/Resume can be enabled. 0: The functions of hardware wait idle and auto Suspend/Resume are not supported.
    #[inline(always)]
    pub fn spi_mem_waiti_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WAITI_EN_MASK) != 0
    }

    /// Set SPI_MEM_WAITI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_waiti_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_WAITI_EN_MASK);
    }

    /// Clear SPI_MEM_WAITI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_waiti_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_WAITI_EN_MASK);
    }
}

/// SPI_MEM_FLASH_WAITI_CTRL value type for batch read/modify/write operations
///
/// Use `spimemflashwaitictrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemflashwaitictrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemFlashWaitiCtrlVal {
    bits: u32,
}

impl SpiMemFlashWaitiCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00050001 }
    }

    /// Read **SPI_MEM_WAITI_CMD** `[31:16]`
    ///
    /// The command value to wait flash idle(RDSR).
    #[inline(always)]
    pub const fn spi_mem_waiti_cmd(&self) -> u32 {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_MASK)
            >> SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_OFFSET
    }

    /// Read **SPI_MEM_WAITI_DUMMY_CYCLELEN** `[15:10]`
    ///
    /// The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    pub const fn spi_mem_waiti_dummy_cyclelen(&self) -> u32 {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK)
            >> SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_CYCLELEN_OFFSET
    }

    /// Read **SPI_MEM_WAITI_CMD_2B** `[9:9]`
    ///
    /// 1:The wait idle command bit length is 16. 0: The wait idle command bit length is 8.
    #[inline(always)]
    pub const fn spi_mem_waiti_cmd_2b(&self) -> bool {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_2B_MASK) != 0
    }

    /// Read **SPI_MEM_WAITI_ADDR_CYCLELEN** `[4:3]`
    ///
    /// When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
    #[inline(always)]
    pub const fn spi_mem_waiti_addr_cyclelen(&self) -> u32 {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK)
            >> SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_CYCLELEN_OFFSET
    }

    /// Read **SPI_MEM_WAITI_ADDR_EN** `[2:2]`
    ///
    /// 1: Output address 0 in RDSR or read SUS command transfer. 0: Do not send out address in RDSR or read SUS command transfer.
    #[inline(always)]
    pub const fn spi_mem_waiti_addr_en(&self) -> bool {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_EN_MASK) != 0
    }

    /// Read **SPI_MEM_WAITI_DUMMY** `[1:1]`
    ///
    /// The dummy phase enable when wait flash idle (RDSR)
    #[inline(always)]
    pub const fn spi_mem_waiti_dummy(&self) -> bool {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_MASK) != 0
    }

    /// Read **SPI_MEM_WAITI_EN** `[0:0]`
    ///
    /// 1: The hardware will wait idle after SE/PP/WRSR automatically, and hardware auto Suspend/Resume can be enabled. 0: The functions of hardware wait idle and auto Suspend/Resume are not supported.
    #[inline(always)]
    pub const fn spi_mem_waiti_en(&self) -> bool {
        (self.bits & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_EN_MASK) != 0
    }

    /// Set **SPI_MEM_WAITI_CMD** `[31:16]`
    ///
    /// The command value to wait flash idle(RDSR).
    #[inline(always)]
    pub const fn with_spi_mem_waiti_cmd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_MASK)
                | ((value << SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_OFFSET)
                    & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_MASK),
        }
    }

    /// Set **SPI_MEM_WAITI_DUMMY_CYCLELEN** `[15:10]`
    ///
    /// The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    pub const fn with_spi_mem_waiti_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK)
                | ((value << SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_CYCLELEN_OFFSET)
                    & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_MEM_WAITI_CMD_2B** `[9:9]`
    ///
    /// 1:The wait idle command bit length is 16. 0: The wait idle command bit length is 8.
    #[inline(always)]
    pub const fn with_spi_mem_waiti_cmd_2b(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_2B_MASK
            } else {
                self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_CMD_2B_MASK
            },
        }
    }

    /// Set **SPI_MEM_WAITI_ADDR_CYCLELEN** `[4:3]`
    ///
    /// When SPI_MEM_WAITI_ADDR_EN is set, the cycle length of sent out address is (SPI_MEM_WAITI_ADDR_CYCLELEN[1:0] + 1) SPI bus clock cycles. It is not active when SPI_MEM_WAITI_ADDR_EN is cleared.
    #[inline(always)]
    pub const fn with_spi_mem_waiti_addr_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK)
                | ((value << SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_CYCLELEN_OFFSET)
                    & SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_CYCLELEN_MASK),
        }
    }

    /// Set **SPI_MEM_WAITI_ADDR_EN** `[2:2]`
    ///
    /// 1: Output address 0 in RDSR or read SUS command transfer. 0: Do not send out address in RDSR or read SUS command transfer.
    #[inline(always)]
    pub const fn with_spi_mem_waiti_addr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_EN_MASK
            } else {
                self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_ADDR_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_WAITI_DUMMY** `[1:1]`
    ///
    /// The dummy phase enable when wait flash idle (RDSR)
    #[inline(always)]
    pub const fn with_spi_mem_waiti_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_MASK
            } else {
                self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_DUMMY_MASK
            },
        }
    }

    /// Set **SPI_MEM_WAITI_EN** `[0:0]`
    ///
    /// 1: The hardware will wait idle after SE/PP/WRSR automatically, and hardware auto Suspend/Resume can be enabled. 0: The functions of hardware wait idle and auto Suspend/Resume are not supported.
    #[inline(always)]
    pub const fn with_spi_mem_waiti_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_EN_MASK
            } else {
                self.bits & !SpiMemFlashWaitiCtrl::SPI_MEM_WAITI_EN_MASK
            },
        }
    }
}

impl Default for SpiMemFlashWaitiCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemFlashWaitiCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemFlashWaitiCtrlVal {
        SpiMemFlashWaitiCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemFlashWaitiCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemFlashWaitiCtrlVal) -> SpiMemFlashWaitiCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_FLASH_SUS_CTRL Register
///
/// SPI1 flash suspend control register
///
/// Address offset: 0x009c
/// Reset value: 0x08002000
///
/// ## Fields
///
/// - **SPI_MEM_SUS_TIMEOUT_CNT** `[31:25]` (RW)
///
///   When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
///
/// - **SPI_MEM_PES_END_EN** `[24]` (RW)
///
///   1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0.
///
/// - **SPI_MEM_PER_END_EN** `[23]` (RW)
///
///   1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0.
///
/// - **SPI_FMEM_RD_SUS_2B** `[22]` (RW)
///
///   1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit
///
/// - **SPI_MEM_PESR_END_MSK** `[21:6]` (RW)
///
///   The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
///
/// - **SPI_MEM_FLASH_PES_EN** `[5]` (RW)
///
///   Set this bit to enable Auto-suspending function.
///
/// - **SPI_MEM_PES_PER_EN** `[4]` (RW)
///
///   Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done.
///
/// - **SPI_MEM_FLASH_PES_WAIT_EN** `[3]` (RW)
///
///   1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent.
///
/// - **SPI_MEM_FLASH_PER_WAIT_EN** `[2]` (RW)
///
///   1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent.
///
/// - **SPI_MEM_FLASH_PES** `[1]` (RW)
///
///   program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
///
/// - **SPI_MEM_FLASH_PER** `[0]` (RW)
///
///   program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
#[repr(transparent)]
pub struct SpiMemFlashSusCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemFlashSusCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemFlashSusCtrl {
    // ======== Field Constants ========

    /// **SPI_MEM_SUS_TIMEOUT_CNT**
    ///
    /// When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
    ///
    /// Bits `[31:25]`, mask `0xfe000000`
    pub const SPI_MEM_SUS_TIMEOUT_CNT_OFFSET: u32 = 25;
    pub const SPI_MEM_SUS_TIMEOUT_CNT_MASK: u32 = 0xfe000000;

    /// **SPI_MEM_PES_END_EN**
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SPI_MEM_PES_END_EN_OFFSET: u32 = 24;
    pub const SPI_MEM_PES_END_EN_MASK: u32 = 0x01000000;
    pub const SPI_MEM_PES_END_EN: u32 = 1 << 24;

    /// **SPI_MEM_PER_END_EN**
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SPI_MEM_PER_END_EN_OFFSET: u32 = 23;
    pub const SPI_MEM_PER_END_EN_MASK: u32 = 0x00800000;
    pub const SPI_MEM_PER_END_EN: u32 = 1 << 23;

    /// **SPI_FMEM_RD_SUS_2B**
    ///
    /// 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SPI_FMEM_RD_SUS_2B_OFFSET: u32 = 22;
    pub const SPI_FMEM_RD_SUS_2B_MASK: u32 = 0x00400000;
    pub const SPI_FMEM_RD_SUS_2B: u32 = 1 << 22;

    /// **SPI_MEM_PESR_END_MSK**
    ///
    /// The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
    ///
    /// Bits `[21:6]`, mask `0x003fffc0`
    pub const SPI_MEM_PESR_END_MSK_OFFSET: u32 = 6;
    pub const SPI_MEM_PESR_END_MSK_MASK: u32 = 0x003fffc0;

    /// **SPI_MEM_FLASH_PES_EN**
    ///
    /// Set this bit to enable Auto-suspending function.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_FLASH_PES_EN_OFFSET: u32 = 5;
    pub const SPI_MEM_FLASH_PES_EN_MASK: u32 = 0x00000020;
    pub const SPI_MEM_FLASH_PES_EN: u32 = 1 << 5;

    /// **SPI_MEM_PES_PER_EN**
    ///
    /// Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_PES_PER_EN_OFFSET: u32 = 4;
    pub const SPI_MEM_PES_PER_EN_MASK: u32 = 0x00000010;
    pub const SPI_MEM_PES_PER_EN: u32 = 1 << 4;

    /// **SPI_MEM_FLASH_PES_WAIT_EN**
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FLASH_PES_WAIT_EN_OFFSET: u32 = 3;
    pub const SPI_MEM_FLASH_PES_WAIT_EN_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FLASH_PES_WAIT_EN: u32 = 1 << 3;

    /// **SPI_MEM_FLASH_PER_WAIT_EN**
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_FLASH_PER_WAIT_EN_OFFSET: u32 = 2;
    pub const SPI_MEM_FLASH_PER_WAIT_EN_MASK: u32 = 0x00000004;
    pub const SPI_MEM_FLASH_PER_WAIT_EN: u32 = 1 << 2;

    /// **SPI_MEM_FLASH_PES**
    ///
    /// program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_FLASH_PES_OFFSET: u32 = 1;
    pub const SPI_MEM_FLASH_PES_MASK: u32 = 0x00000002;
    pub const SPI_MEM_FLASH_PES: u32 = 1 << 1;

    /// **SPI_MEM_FLASH_PER**
    ///
    /// program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_FLASH_PER_OFFSET: u32 = 0;
    pub const SPI_MEM_FLASH_PER_MASK: u32 = 0x00000001;
    pub const SPI_MEM_FLASH_PER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_SUS_TIMEOUT_CNT** `[31:25]` (RW)
    ///
    /// When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
    #[inline(always)]
    pub fn spi_mem_sus_timeout_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_SUS_TIMEOUT_CNT_MASK)
            >> Self::SPI_MEM_SUS_TIMEOUT_CNT_OFFSET
    }

    /// Write **SPI_MEM_SUS_TIMEOUT_CNT** `[31:25]` (RW)
    ///
    /// When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
    #[inline(always)]
    pub fn write_spi_mem_sus_timeout_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_SUS_TIMEOUT_CNT_MASK)
                | ((value << Self::SPI_MEM_SUS_TIMEOUT_CNT_OFFSET)
                    & Self::SPI_MEM_SUS_TIMEOUT_CNT_MASK)
        });
    }

    /// **SPI_MEM_PES_END_EN** `[24]` (RW)
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub fn spi_mem_pes_end_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PES_END_EN_MASK) != 0
    }

    /// Set SPI_MEM_PES_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pes_end_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PES_END_EN_MASK);
    }

    /// Clear SPI_MEM_PES_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pes_end_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PES_END_EN_MASK);
    }

    /// **SPI_MEM_PER_END_EN** `[23]` (RW)
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub fn spi_mem_per_end_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PER_END_EN_MASK) != 0
    }

    /// Set SPI_MEM_PER_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_per_end_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PER_END_EN_MASK);
    }

    /// Clear SPI_MEM_PER_END_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_per_end_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PER_END_EN_MASK);
    }

    /// **SPI_FMEM_RD_SUS_2B** `[22]` (RW)
    ///
    /// 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit
    #[inline(always)]
    pub fn spi_fmem_rd_sus_2b(&self) -> bool {
        (self.value.read_bits() & Self::SPI_FMEM_RD_SUS_2B_MASK) != 0
    }

    /// Set SPI_FMEM_RD_SUS_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_fmem_rd_sus_2b(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_FMEM_RD_SUS_2B_MASK);
    }

    /// Clear SPI_FMEM_RD_SUS_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_fmem_rd_sus_2b(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_FMEM_RD_SUS_2B_MASK);
    }

    /// **SPI_MEM_PESR_END_MSK** `[21:6]` (RW)
    ///
    /// The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
    #[inline(always)]
    pub fn spi_mem_pesr_end_msk(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_PESR_END_MSK_MASK)
            >> Self::SPI_MEM_PESR_END_MSK_OFFSET
    }

    /// Write **SPI_MEM_PESR_END_MSK** `[21:6]` (RW)
    ///
    /// The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
    #[inline(always)]
    pub fn write_spi_mem_pesr_end_msk(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_PESR_END_MSK_MASK)
                | ((value << Self::SPI_MEM_PESR_END_MSK_OFFSET) & Self::SPI_MEM_PESR_END_MSK_MASK)
        });
    }

    /// **SPI_MEM_FLASH_PES_EN** `[5]` (RW)
    ///
    /// Set this bit to enable Auto-suspending function.
    #[inline(always)]
    pub fn spi_mem_flash_pes_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PES_EN_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PES_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pes_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PES_EN_MASK);
    }

    /// Clear SPI_MEM_FLASH_PES_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pes_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PES_EN_MASK);
    }

    /// **SPI_MEM_PES_PER_EN** `[4]` (RW)
    ///
    /// Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done.
    #[inline(always)]
    pub fn spi_mem_pes_per_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PES_PER_EN_MASK) != 0
    }

    /// Set SPI_MEM_PES_PER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pes_per_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PES_PER_EN_MASK);
    }

    /// Clear SPI_MEM_PES_PER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pes_per_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PES_PER_EN_MASK);
    }

    /// **SPI_MEM_FLASH_PES_WAIT_EN** `[3]` (RW)
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_pes_wait_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PES_WAIT_EN_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PES_WAIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pes_wait_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PES_WAIT_EN_MASK);
    }

    /// Clear SPI_MEM_FLASH_PES_WAIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pes_wait_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PES_WAIT_EN_MASK);
    }

    /// **SPI_MEM_FLASH_PER_WAIT_EN** `[2]` (RW)
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_per_wait_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PER_WAIT_EN_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PER_WAIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_per_wait_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PER_WAIT_EN_MASK);
    }

    /// Clear SPI_MEM_FLASH_PER_WAIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_per_wait_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PER_WAIT_EN_MASK);
    }

    /// **SPI_MEM_FLASH_PES** `[1]` (RW)
    ///
    /// program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_pes(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PES_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pes(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_PES_MASK);
    }

    /// Clear SPI_MEM_FLASH_PES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pes(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PES_MASK);
    }

    /// **SPI_MEM_FLASH_PER** `[0]` (RW)
    ///
    /// program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub fn spi_mem_flash_per(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PER_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_per(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_PER_MASK);
    }

    /// Clear SPI_MEM_FLASH_PER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_per(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PER_MASK);
    }
}

/// SPI_MEM_FLASH_SUS_CTRL value type for batch read/modify/write operations
///
/// Use `spimemflashsusctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemflashsusctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemFlashSusCtrlVal {
    bits: u32,
}

impl SpiMemFlashSusCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x08002000 }
    }

    /// Read **SPI_MEM_SUS_TIMEOUT_CNT** `[31:25]`
    ///
    /// When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
    #[inline(always)]
    pub const fn spi_mem_sus_timeout_cnt(&self) -> u32 {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_SUS_TIMEOUT_CNT_MASK)
            >> SpiMemFlashSusCtrl::SPI_MEM_SUS_TIMEOUT_CNT_OFFSET
    }

    /// Read **SPI_MEM_PES_END_EN** `[24:24]`
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub const fn spi_mem_pes_end_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_PES_END_EN_MASK) != 0
    }

    /// Read **SPI_MEM_PER_END_EN** `[23:23]`
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub const fn spi_mem_per_end_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_PER_END_EN_MASK) != 0
    }

    /// Read **SPI_FMEM_RD_SUS_2B** `[22:22]`
    ///
    /// 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit
    #[inline(always)]
    pub const fn spi_fmem_rd_sus_2b(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_FMEM_RD_SUS_2B_MASK) != 0
    }

    /// Read **SPI_MEM_PESR_END_MSK** `[21:6]`
    ///
    /// The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
    #[inline(always)]
    pub const fn spi_mem_pesr_end_msk(&self) -> u32 {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_PESR_END_MSK_MASK)
            >> SpiMemFlashSusCtrl::SPI_MEM_PESR_END_MSK_OFFSET
    }

    /// Read **SPI_MEM_FLASH_PES_EN** `[5:5]`
    ///
    /// Set this bit to enable Auto-suspending function.
    #[inline(always)]
    pub const fn spi_mem_flash_pes_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_EN_MASK) != 0
    }

    /// Read **SPI_MEM_PES_PER_EN** `[4:4]`
    ///
    /// Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done.
    #[inline(always)]
    pub const fn spi_mem_pes_per_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_PES_PER_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PES_WAIT_EN** `[3:3]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_pes_wait_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_WAIT_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PER_WAIT_EN** `[2:2]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_per_wait_en(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_WAIT_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PES** `[1:1]`
    ///
    /// program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_pes(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PER** `[0:0]`
    ///
    /// program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn spi_mem_flash_per(&self) -> bool {
        (self.bits & SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_MASK) != 0
    }

    /// Set **SPI_MEM_SUS_TIMEOUT_CNT** `[31:25]`
    ///
    /// When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT[6:0] times, it will be treated as check pass.
    #[inline(always)]
    pub const fn with_spi_mem_sus_timeout_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashSusCtrl::SPI_MEM_SUS_TIMEOUT_CNT_MASK)
                | ((value << SpiMemFlashSusCtrl::SPI_MEM_SUS_TIMEOUT_CNT_OFFSET)
                    & SpiMemFlashSusCtrl::SPI_MEM_SUS_TIMEOUT_CNT_MASK),
        }
    }

    /// Set **SPI_MEM_PES_END_EN** `[24:24]`
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub const fn with_spi_mem_pes_end_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_PES_END_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_PES_END_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_PER_END_EN** `[23:23]`
    ///
    /// 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0.
    #[inline(always)]
    pub const fn with_spi_mem_per_end_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_PER_END_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_PER_END_EN_MASK
            },
        }
    }

    /// Set **SPI_FMEM_RD_SUS_2B** `[22:22]`
    ///
    /// 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit
    #[inline(always)]
    pub const fn with_spi_fmem_rd_sus_2b(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_FMEM_RD_SUS_2B_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_FMEM_RD_SUS_2B_MASK
            },
        }
    }

    /// Set **SPI_MEM_PESR_END_MSK** `[21:6]`
    ///
    /// The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in[15:0](only status_in[7:0] is valid when only one byte of data is read out, status_in[15:0] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in[15:0]^ SPI_MEM_PESR_END_MSK[15:0].
    #[inline(always)]
    pub const fn with_spi_mem_pesr_end_msk(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashSusCtrl::SPI_MEM_PESR_END_MSK_MASK)
                | ((value << SpiMemFlashSusCtrl::SPI_MEM_PESR_END_MSK_OFFSET)
                    & SpiMemFlashSusCtrl::SPI_MEM_PESR_END_MSK_MASK),
        }
    }

    /// Set **SPI_MEM_FLASH_PES_EN** `[5:5]`
    ///
    /// Set this bit to enable Auto-suspending function.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pes_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_PES_PER_EN** `[4:4]`
    ///
    /// Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done.
    #[inline(always)]
    pub const fn with_spi_mem_pes_per_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_PES_PER_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_PES_PER_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PES_WAIT_EN** `[3:3]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pes_wait_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_WAIT_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_WAIT_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PER_WAIT_EN** `[2:2]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_per_wait_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_WAIT_EN_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_WAIT_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PES** `[1:1]`
    ///
    /// program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pes(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_FLASH_PES_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PER** `[0:0]`
    ///
    /// program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable.
    #[inline(always)]
    pub const fn with_spi_mem_flash_per(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_MASK
            } else {
                self.bits & !SpiMemFlashSusCtrl::SPI_MEM_FLASH_PER_MASK
            },
        }
    }
}

impl Default for SpiMemFlashSusCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemFlashSusCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemFlashSusCtrlVal {
        SpiMemFlashSusCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemFlashSusCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemFlashSusCtrlVal) -> SpiMemFlashSusCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_FLASH_SUS_CMD Register
///
/// SPI1 flash suspend command register
///
/// Address offset: 0x00a0
/// Reset value: 0x00057575
///
/// ## Fields
///
/// - **SPI_MEM_WAIT_PESR_COMMAND** `[31:16]` (RW)
///
///   Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
///
/// - **SPI_MEM_FLASH_PES_COMMAND** `[15:0]` (RW)
///
///   Program/Erase suspend command.
#[repr(transparent)]
pub struct SpiMemFlashSusCmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemFlashSusCmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemFlashSusCmd {
    // ======== Field Constants ========

    /// **SPI_MEM_WAIT_PESR_COMMAND**
    ///
    /// Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SPI_MEM_WAIT_PESR_COMMAND_OFFSET: u32 = 16;
    pub const SPI_MEM_WAIT_PESR_COMMAND_MASK: u32 = 0xffff0000;

    /// **SPI_MEM_FLASH_PES_COMMAND**
    ///
    /// Program/Erase suspend command.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SPI_MEM_FLASH_PES_COMMAND_OFFSET: u32 = 0;
    pub const SPI_MEM_FLASH_PES_COMMAND_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_WAIT_PESR_COMMAND** `[31:16]` (RW)
    ///
    /// Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
    #[inline(always)]
    pub fn spi_mem_wait_pesr_command(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_WAIT_PESR_COMMAND_MASK)
            >> Self::SPI_MEM_WAIT_PESR_COMMAND_OFFSET
    }

    /// Write **SPI_MEM_WAIT_PESR_COMMAND** `[31:16]` (RW)
    ///
    /// Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
    #[inline(always)]
    pub fn write_spi_mem_wait_pesr_command(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_WAIT_PESR_COMMAND_MASK)
                | ((value << Self::SPI_MEM_WAIT_PESR_COMMAND_OFFSET)
                    & Self::SPI_MEM_WAIT_PESR_COMMAND_MASK)
        });
    }

    /// **SPI_MEM_FLASH_PES_COMMAND** `[15:0]` (RW)
    ///
    /// Program/Erase suspend command.
    #[inline(always)]
    pub fn spi_mem_flash_pes_command(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PES_COMMAND_MASK)
            >> Self::SPI_MEM_FLASH_PES_COMMAND_OFFSET
    }

    /// Write **SPI_MEM_FLASH_PES_COMMAND** `[15:0]` (RW)
    ///
    /// Program/Erase suspend command.
    #[inline(always)]
    pub fn write_spi_mem_flash_pes_command(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_FLASH_PES_COMMAND_MASK)
                | ((value << Self::SPI_MEM_FLASH_PES_COMMAND_OFFSET)
                    & Self::SPI_MEM_FLASH_PES_COMMAND_MASK)
        });
    }
}

/// SPI_MEM_FLASH_SUS_CMD value type for batch read/modify/write operations
///
/// Use `spimemflashsuscmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemflashsuscmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemFlashSusCmdVal {
    bits: u32,
}

impl SpiMemFlashSusCmdVal {
    /// Create from raw bits
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
        Self { bits: 0x00057575 }
    }

    /// Read **SPI_MEM_WAIT_PESR_COMMAND** `[31:16]`
    ///
    /// Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
    #[inline(always)]
    pub const fn spi_mem_wait_pesr_command(&self) -> u32 {
        (self.bits & SpiMemFlashSusCmd::SPI_MEM_WAIT_PESR_COMMAND_MASK)
            >> SpiMemFlashSusCmd::SPI_MEM_WAIT_PESR_COMMAND_OFFSET
    }

    /// Read **SPI_MEM_FLASH_PES_COMMAND** `[15:0]`
    ///
    /// Program/Erase suspend command.
    #[inline(always)]
    pub const fn spi_mem_flash_pes_command(&self) -> u32 {
        (self.bits & SpiMemFlashSusCmd::SPI_MEM_FLASH_PES_COMMAND_MASK)
            >> SpiMemFlashSusCmd::SPI_MEM_FLASH_PES_COMMAND_OFFSET
    }

    /// Set **SPI_MEM_WAIT_PESR_COMMAND** `[31:16]`
    ///
    /// Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash.
    #[inline(always)]
    pub const fn with_spi_mem_wait_pesr_command(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashSusCmd::SPI_MEM_WAIT_PESR_COMMAND_MASK)
                | ((value << SpiMemFlashSusCmd::SPI_MEM_WAIT_PESR_COMMAND_OFFSET)
                    & SpiMemFlashSusCmd::SPI_MEM_WAIT_PESR_COMMAND_MASK),
        }
    }

    /// Set **SPI_MEM_FLASH_PES_COMMAND** `[15:0]`
    ///
    /// Program/Erase suspend command.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pes_command(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemFlashSusCmd::SPI_MEM_FLASH_PES_COMMAND_MASK)
                | ((value << SpiMemFlashSusCmd::SPI_MEM_FLASH_PES_COMMAND_OFFSET)
                    & SpiMemFlashSusCmd::SPI_MEM_FLASH_PES_COMMAND_MASK),
        }
    }
}

impl Default for SpiMemFlashSusCmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemFlashSusCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemFlashSusCmdVal {
        SpiMemFlashSusCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemFlashSusCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemFlashSusCmdVal) -> SpiMemFlashSusCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_SUS_STATUS Register
///
/// SPI1 flash suspend status register
///
/// Address offset: 0x00a4
/// Reset value: 0x7a7a0000
///
/// ## Fields
///
/// - **SPI_MEM_FLASH_PER_COMMAND** `[31:16]` (RW)
///
///   Program/Erase resume command.
///
/// - **SPI_MEM_FLASH_PESR_CMD_2B** `[15]` (RW)
///
///   1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8.
///
/// - **SPI_MEM_SPI0_LOCK_EN** `[7]` (RW)
///
///   1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
///
/// - **SPI_MEM_FLASH_PES_DLY_128** `[6]` (RW)
///
///   Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PES command is sent.
///
/// - **SPI_MEM_FLASH_PER_DLY_128** `[5]` (RW)
///
///   Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PER command is sent.
///
/// - **SPI_MEM_FLASH_DP_DLY_128** `[4]` (RW)
///
///   1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after DP command is sent.
///
/// - **SPI_MEM_FLASH_RES_DLY_128** `[3]` (RW)
///
///   1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after RES command is sent.
///
/// - **SPI_MEM_FLASH_HPM_DLY_128** `[2]` (RW)
///
///   1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after HPM command is sent.
///
/// - **SPI_MEM_WAIT_PESR_CMD_2B** `[1]` (RW)
///
///   1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[15:0] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[7:0] to check SUS/SUS1/SUS2 bit.
///
/// - **SPI_MEM_FLASH_SUS** `[0]` (RW)
///
///   The status of flash suspend, only used in SPI1.
#[repr(transparent)]
pub struct SpiMemSusStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpiMemSusStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpiMemSusStatus {
    // ======== Field Constants ========

    /// **SPI_MEM_FLASH_PER_COMMAND**
    ///
    /// Program/Erase resume command.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SPI_MEM_FLASH_PER_COMMAND_OFFSET: u32 = 16;
    pub const SPI_MEM_FLASH_PER_COMMAND_MASK: u32 = 0xffff0000;

    /// **SPI_MEM_FLASH_PESR_CMD_2B**
    ///
    /// 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SPI_MEM_FLASH_PESR_CMD_2B_OFFSET: u32 = 15;
    pub const SPI_MEM_FLASH_PESR_CMD_2B_MASK: u32 = 0x00008000;
    pub const SPI_MEM_FLASH_PESR_CMD_2B: u32 = 1 << 15;

    /// **SPI_MEM_SPI0_LOCK_EN**
    ///
    /// 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPI_MEM_SPI0_LOCK_EN_OFFSET: u32 = 7;
    pub const SPI_MEM_SPI0_LOCK_EN_MASK: u32 = 0x00000080;
    pub const SPI_MEM_SPI0_LOCK_EN: u32 = 1 << 7;

    /// **SPI_MEM_FLASH_PES_DLY_128**
    ///
    /// Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PES command is sent.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPI_MEM_FLASH_PES_DLY_128_OFFSET: u32 = 6;
    pub const SPI_MEM_FLASH_PES_DLY_128_MASK: u32 = 0x00000040;
    pub const SPI_MEM_FLASH_PES_DLY_128: u32 = 1 << 6;

    /// **SPI_MEM_FLASH_PER_DLY_128**
    ///
    /// Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PER command is sent.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SPI_MEM_FLASH_PER_DLY_128_OFFSET: u32 = 5;
    pub const SPI_MEM_FLASH_PER_DLY_128_MASK: u32 = 0x00000020;
    pub const SPI_MEM_FLASH_PER_DLY_128: u32 = 1 << 5;

    /// **SPI_MEM_FLASH_DP_DLY_128**
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after DP command is sent.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_FLASH_DP_DLY_128_OFFSET: u32 = 4;
    pub const SPI_MEM_FLASH_DP_DLY_128_MASK: u32 = 0x00000010;
    pub const SPI_MEM_FLASH_DP_DLY_128: u32 = 1 << 4;

    /// **SPI_MEM_FLASH_RES_DLY_128**
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after RES command is sent.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_FLASH_RES_DLY_128_OFFSET: u32 = 3;
    pub const SPI_MEM_FLASH_RES_DLY_128_MASK: u32 = 0x00000008;
    pub const SPI_MEM_FLASH_RES_DLY_128: u32 = 1 << 3;

    /// **SPI_MEM_FLASH_HPM_DLY_128**
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after HPM command is sent.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_FLASH_HPM_DLY_128_OFFSET: u32 = 2;
    pub const SPI_MEM_FLASH_HPM_DLY_128_MASK: u32 = 0x00000004;
    pub const SPI_MEM_FLASH_HPM_DLY_128: u32 = 1 << 2;

    /// **SPI_MEM_WAIT_PESR_CMD_2B**
    ///
    /// 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[15:0] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[7:0] to check SUS/SUS1/SUS2 bit.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_WAIT_PESR_CMD_2B_OFFSET: u32 = 1;
    pub const SPI_MEM_WAIT_PESR_CMD_2B_MASK: u32 = 0x00000002;
    pub const SPI_MEM_WAIT_PESR_CMD_2B: u32 = 1 << 1;

    /// **SPI_MEM_FLASH_SUS**
    ///
    /// The status of flash suspend, only used in SPI1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_FLASH_SUS_OFFSET: u32 = 0;
    pub const SPI_MEM_FLASH_SUS_MASK: u32 = 0x00000001;
    pub const SPI_MEM_FLASH_SUS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_FLASH_PER_COMMAND** `[31:16]` (RW)
    ///
    /// Program/Erase resume command.
    #[inline(always)]
    pub fn spi_mem_flash_per_command(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PER_COMMAND_MASK)
            >> Self::SPI_MEM_FLASH_PER_COMMAND_OFFSET
    }

    /// Write **SPI_MEM_FLASH_PER_COMMAND** `[31:16]` (RW)
    ///
    /// Program/Erase resume command.
    #[inline(always)]
    pub fn write_spi_mem_flash_per_command(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI_MEM_FLASH_PER_COMMAND_MASK)
                | ((value << Self::SPI_MEM_FLASH_PER_COMMAND_OFFSET)
                    & Self::SPI_MEM_FLASH_PER_COMMAND_MASK)
        });
    }

    /// **SPI_MEM_FLASH_PESR_CMD_2B** `[15]` (RW)
    ///
    /// 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8.
    #[inline(always)]
    pub fn spi_mem_flash_pesr_cmd_2b(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PESR_CMD_2B_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PESR_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pesr_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PESR_CMD_2B_MASK);
    }

    /// Clear SPI_MEM_FLASH_PESR_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pesr_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PESR_CMD_2B_MASK);
    }

    /// **SPI_MEM_SPI0_LOCK_EN** `[7]` (RW)
    ///
    /// 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    #[inline(always)]
    pub fn spi_mem_spi0_lock_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_SPI0_LOCK_EN_MASK) != 0
    }

    /// Set SPI_MEM_SPI0_LOCK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_spi0_lock_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_SPI0_LOCK_EN_MASK);
    }

    /// Clear SPI_MEM_SPI0_LOCK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_spi0_lock_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_SPI0_LOCK_EN_MASK);
    }

    /// **SPI_MEM_FLASH_PES_DLY_128** `[6]` (RW)
    ///
    /// Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PES command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_pes_dly_128(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PES_DLY_128_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PES_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_pes_dly_128(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PES_DLY_128_MASK);
    }

    /// Clear SPI_MEM_FLASH_PES_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_pes_dly_128(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PES_DLY_128_MASK);
    }

    /// **SPI_MEM_FLASH_PER_DLY_128** `[5]` (RW)
    ///
    /// Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PER command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_per_dly_128(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_PER_DLY_128_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_PER_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_per_dly_128(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_PER_DLY_128_MASK);
    }

    /// Clear SPI_MEM_FLASH_PER_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_per_dly_128(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_PER_DLY_128_MASK);
    }

    /// **SPI_MEM_FLASH_DP_DLY_128** `[4]` (RW)
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after DP command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_dp_dly_128(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_DP_DLY_128_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_DP_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_dp_dly_128(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_DP_DLY_128_MASK);
    }

    /// Clear SPI_MEM_FLASH_DP_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_dp_dly_128(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_DP_DLY_128_MASK);
    }

    /// **SPI_MEM_FLASH_RES_DLY_128** `[3]` (RW)
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after RES command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_res_dly_128(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_RES_DLY_128_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_RES_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_res_dly_128(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_RES_DLY_128_MASK);
    }

    /// Clear SPI_MEM_FLASH_RES_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_res_dly_128(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_RES_DLY_128_MASK);
    }

    /// **SPI_MEM_FLASH_HPM_DLY_128** `[2]` (RW)
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after HPM command is sent.
    #[inline(always)]
    pub fn spi_mem_flash_hpm_dly_128(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_HPM_DLY_128_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_HPM_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_hpm_dly_128(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_FLASH_HPM_DLY_128_MASK);
    }

    /// Clear SPI_MEM_FLASH_HPM_DLY_128 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_hpm_dly_128(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_HPM_DLY_128_MASK);
    }

    /// **SPI_MEM_WAIT_PESR_CMD_2B** `[1]` (RW)
    ///
    /// 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[15:0] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[7:0] to check SUS/SUS1/SUS2 bit.
    #[inline(always)]
    pub fn spi_mem_wait_pesr_cmd_2b(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WAIT_PESR_CMD_2B_MASK) != 0
    }

    /// Set SPI_MEM_WAIT_PESR_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wait_pesr_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WAIT_PESR_CMD_2B_MASK);
    }

    /// Clear SPI_MEM_WAIT_PESR_CMD_2B bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wait_pesr_cmd_2b(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WAIT_PESR_CMD_2B_MASK);
    }

    /// **SPI_MEM_FLASH_SUS** `[0]` (RW)
    ///
    /// The status of flash suspend, only used in SPI1.
    #[inline(always)]
    pub fn spi_mem_flash_sus(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_FLASH_SUS_MASK) != 0
    }

    /// Set SPI_MEM_FLASH_SUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_flash_sus(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_FLASH_SUS_MASK);
    }

    /// Clear SPI_MEM_FLASH_SUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_flash_sus(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_FLASH_SUS_MASK);
    }
}

/// SPI_MEM_SUS_STATUS value type for batch read/modify/write operations
///
/// Use `spimemsusstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spimemsusstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpiMemSusStatusVal {
    bits: u32,
}

impl SpiMemSusStatusVal {
    /// Create from raw bits
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
        Self { bits: 0x7a7a0000 }
    }

    /// Read **SPI_MEM_FLASH_PER_COMMAND** `[31:16]`
    ///
    /// Program/Erase resume command.
    #[inline(always)]
    pub const fn spi_mem_flash_per_command(&self) -> u32 {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_PER_COMMAND_MASK)
            >> SpiMemSusStatus::SPI_MEM_FLASH_PER_COMMAND_OFFSET
    }

    /// Read **SPI_MEM_FLASH_PESR_CMD_2B** `[15:15]`
    ///
    /// 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8.
    #[inline(always)]
    pub const fn spi_mem_flash_pesr_cmd_2b(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_PESR_CMD_2B_MASK) != 0
    }

    /// Read **SPI_MEM_SPI0_LOCK_EN** `[7:7]`
    ///
    /// 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    #[inline(always)]
    pub const fn spi_mem_spi0_lock_en(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_SPI0_LOCK_EN_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PES_DLY_128** `[6:6]`
    ///
    /// Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PES command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_pes_dly_128(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_PES_DLY_128_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_PER_DLY_128** `[5:5]`
    ///
    /// Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PER command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_per_dly_128(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_PER_DLY_128_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_DP_DLY_128** `[4:4]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after DP command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_dp_dly_128(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_DP_DLY_128_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_RES_DLY_128** `[3:3]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after RES command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_res_dly_128(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_RES_DLY_128_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_HPM_DLY_128** `[2:2]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after HPM command is sent.
    #[inline(always)]
    pub const fn spi_mem_flash_hpm_dly_128(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_HPM_DLY_128_MASK) != 0
    }

    /// Read **SPI_MEM_WAIT_PESR_CMD_2B** `[1:1]`
    ///
    /// 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[15:0] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[7:0] to check SUS/SUS1/SUS2 bit.
    #[inline(always)]
    pub const fn spi_mem_wait_pesr_cmd_2b(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_WAIT_PESR_CMD_2B_MASK) != 0
    }

    /// Read **SPI_MEM_FLASH_SUS** `[0:0]`
    ///
    /// The status of flash suspend, only used in SPI1.
    #[inline(always)]
    pub const fn spi_mem_flash_sus(&self) -> bool {
        (self.bits & SpiMemSusStatus::SPI_MEM_FLASH_SUS_MASK) != 0
    }

    /// Set **SPI_MEM_FLASH_PER_COMMAND** `[31:16]`
    ///
    /// Program/Erase resume command.
    #[inline(always)]
    pub const fn with_spi_mem_flash_per_command(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_PER_COMMAND_MASK)
                | ((value << SpiMemSusStatus::SPI_MEM_FLASH_PER_COMMAND_OFFSET)
                    & SpiMemSusStatus::SPI_MEM_FLASH_PER_COMMAND_MASK),
        }
    }

    /// Set **SPI_MEM_FLASH_PESR_CMD_2B** `[15:15]`
    ///
    /// 1: The bit length of Program/Erase Suspend/Resume command is 16. 0: The bit length of Program/Erase Suspend/Resume command is 8.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pesr_cmd_2b(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_PESR_CMD_2B_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_PESR_CMD_2B_MASK
            },
        }
    }

    /// Set **SPI_MEM_SPI0_LOCK_EN** `[7:7]`
    ///
    /// 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    #[inline(always)]
    pub const fn with_spi_mem_spi0_lock_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_SPI0_LOCK_EN_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_SPI0_LOCK_EN_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PES_DLY_128** `[6:6]`
    ///
    /// Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PES command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_pes_dly_128(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_PES_DLY_128_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_PES_DLY_128_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_PER_DLY_128** `[5:5]`
    ///
    /// Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after PER command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_per_dly_128(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_PER_DLY_128_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_PER_DLY_128_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_DP_DLY_128** `[4:4]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after DP command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_dp_dly_128(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_DP_DLY_128_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_DP_DLY_128_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_RES_DLY_128** `[3:3]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after RES command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_res_dly_128(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_RES_DLY_128_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_RES_DLY_128_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_HPM_DLY_128** `[2:2]`
    ///
    /// 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES[9:0] * 4) SPI_CLK cycles after HPM command is sent.
    #[inline(always)]
    pub const fn with_spi_mem_flash_hpm_dly_128(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_HPM_DLY_128_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_HPM_DLY_128_MASK
            },
        }
    }

    /// Set **SPI_MEM_WAIT_PESR_CMD_2B** `[1:1]`
    ///
    /// 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[15:0] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND[7:0] to check SUS/SUS1/SUS2 bit.
    #[inline(always)]
    pub const fn with_spi_mem_wait_pesr_cmd_2b(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_WAIT_PESR_CMD_2B_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_WAIT_PESR_CMD_2B_MASK
            },
        }
    }

    /// Set **SPI_MEM_FLASH_SUS** `[0:0]`
    ///
    /// The status of flash suspend, only used in SPI1.
    #[inline(always)]
    pub const fn with_spi_mem_flash_sus(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemSusStatus::SPI_MEM_FLASH_SUS_MASK
            } else {
                self.bits & !SpiMemSusStatus::SPI_MEM_FLASH_SUS_MASK
            },
        }
    }
}

impl Default for SpiMemSusStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpiMemSusStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpiMemSusStatusVal {
        SpiMemSusStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpiMemSusStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpiMemSusStatusVal) -> SpiMemSusStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI_MEM_INT_ENA Register
///
/// SPI1 interrupt enable register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **SPI_MEM_BROWN_OUT_INT_ENA** `[10]` (RW)
///
///   The enable bit for SPI_MEM_BROWN_OUT_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_ENA** `[4]` (RW)
///
///   The enable bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_ENA** `[3]` (RW)
///
///   The enable bit for SPI_MEM_SLV_ST_END_INT interrupt.
///
/// - **SPI_MEM_WPE_END_INT_ENA** `[2]` (RW)
///
///   The enable bit for SPI_MEM_WPE_END_INT interrupt.
///
/// - **SPI_MEM_PES_END_INT_ENA** `[1]` (RW)
///
///   The enable bit for SPI_MEM_PES_END_INT interrupt.
///
/// - **SPI_MEM_PER_END_INT_ENA** `[0]` (RW)
///
///   The enable bit for SPI_MEM_PER_END_INT interrupt.
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

    /// **SPI_MEM_BROWN_OUT_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_BROWN_OUT_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_BROWN_OUT_INT_ENA_OFFSET: u32 = 10;
    pub const SPI_MEM_BROWN_OUT_INT_ENA_MASK: u32 = 0x00000400;
    pub const SPI_MEM_BROWN_OUT_INT_ENA: u32 = 1 << 10;

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

    /// **SPI_MEM_WPE_END_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_WPE_END_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_WPE_END_INT_ENA_OFFSET: u32 = 2;
    pub const SPI_MEM_WPE_END_INT_ENA_MASK: u32 = 0x00000004;
    pub const SPI_MEM_WPE_END_INT_ENA: u32 = 1 << 2;

    /// **SPI_MEM_PES_END_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_PES_END_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_PES_END_INT_ENA_OFFSET: u32 = 1;
    pub const SPI_MEM_PES_END_INT_ENA_MASK: u32 = 0x00000002;
    pub const SPI_MEM_PES_END_INT_ENA: u32 = 1 << 1;

    /// **SPI_MEM_PER_END_INT_ENA**
    ///
    /// The enable bit for SPI_MEM_PER_END_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_PER_END_INT_ENA_OFFSET: u32 = 0;
    pub const SPI_MEM_PER_END_INT_ENA_MASK: u32 = 0x00000001;
    pub const SPI_MEM_PER_END_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_BROWN_OUT_INT_ENA** `[10]` (RW)
    ///
    /// The enable bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_brown_out_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BROWN_OUT_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_BROWN_OUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_brown_out_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BROWN_OUT_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_BROWN_OUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_brown_out_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BROWN_OUT_INT_ENA_MASK);
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

    /// **SPI_MEM_WPE_END_INT_ENA** `[2]` (RW)
    ///
    /// The enable bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WPE_END_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_WPE_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wpe_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WPE_END_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_WPE_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wpe_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WPE_END_INT_ENA_MASK);
    }

    /// **SPI_MEM_PES_END_INT_ENA** `[1]` (RW)
    ///
    /// The enable bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_pes_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PES_END_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_PES_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pes_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PES_END_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_PES_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pes_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PES_END_INT_ENA_MASK);
    }

    /// **SPI_MEM_PER_END_INT_ENA** `[0]` (RW)
    ///
    /// The enable bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_per_end_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PER_END_INT_ENA_MASK) != 0
    }

    /// Set SPI_MEM_PER_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_per_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PER_END_INT_ENA_MASK);
    }

    /// Clear SPI_MEM_PER_END_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_per_end_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PER_END_INT_ENA_MASK);
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

    /// Read **SPI_MEM_BROWN_OUT_INT_ENA** `[10:10]`
    ///
    /// The enable bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_brown_out_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_BROWN_OUT_INT_ENA_MASK) != 0
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

    /// Read **SPI_MEM_WPE_END_INT_ENA** `[2:2]`
    ///
    /// The enable bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_wpe_end_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_WPE_END_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_PES_END_INT_ENA** `[1:1]`
    ///
    /// The enable bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_pes_end_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_PES_END_INT_ENA_MASK) != 0
    }

    /// Read **SPI_MEM_PER_END_INT_ENA** `[0:0]`
    ///
    /// The enable bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_per_end_int_ena(&self) -> bool {
        (self.bits & SpiMemIntEna::SPI_MEM_PER_END_INT_ENA_MASK) != 0
    }

    /// Set **SPI_MEM_BROWN_OUT_INT_ENA** `[10:10]`
    ///
    /// The enable bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_brown_out_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_BROWN_OUT_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_BROWN_OUT_INT_ENA_MASK
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

    /// Set **SPI_MEM_WPE_END_INT_ENA** `[2:2]`
    ///
    /// The enable bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_wpe_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_WPE_END_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_WPE_END_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_PES_END_INT_ENA** `[1:1]`
    ///
    /// The enable bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_pes_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_PES_END_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_PES_END_INT_ENA_MASK
            },
        }
    }

    /// Set **SPI_MEM_PER_END_INT_ENA** `[0:0]`
    ///
    /// The enable bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_per_end_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntEna::SPI_MEM_PER_END_INT_ENA_MASK
            } else {
                self.bits & !SpiMemIntEna::SPI_MEM_PER_END_INT_ENA_MASK
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
/// SPI1 interrupt clear register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **SPI_MEM_BROWN_OUT_INT_CLR** `[10]` (WO)
///
///   The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_CLR** `[4]` (WO)
///
///   The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_CLR** `[3]` (WO)
///
///   The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
///
/// - **SPI_MEM_WPE_END_INT_CLR** `[2]` (WO)
///
///   The clear bit for SPI_MEM_WPE_END_INT interrupt.
///
/// - **SPI_MEM_PES_END_INT_CLR** `[1]` (WO)
///
///   The clear bit for SPI_MEM_PES_END_INT interrupt.
///
/// - **SPI_MEM_PER_END_INT_CLR** `[0]` (WO)
///
///   The clear bit for SPI_MEM_PER_END_INT interrupt.
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

    /// **SPI_MEM_BROWN_OUT_INT_CLR**
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_BROWN_OUT_INT_CLR_OFFSET: u32 = 10;
    pub const SPI_MEM_BROWN_OUT_INT_CLR_MASK: u32 = 0x00000400;
    pub const SPI_MEM_BROWN_OUT_INT_CLR: u32 = 1 << 10;

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

    /// **SPI_MEM_WPE_END_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_WPE_END_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_WPE_END_INT_CLR_OFFSET: u32 = 2;
    pub const SPI_MEM_WPE_END_INT_CLR_MASK: u32 = 0x00000004;
    pub const SPI_MEM_WPE_END_INT_CLR: u32 = 1 << 2;

    /// **SPI_MEM_PES_END_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_PES_END_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_PES_END_INT_CLR_OFFSET: u32 = 1;
    pub const SPI_MEM_PES_END_INT_CLR_MASK: u32 = 0x00000002;
    pub const SPI_MEM_PES_END_INT_CLR: u32 = 1 << 1;

    /// **SPI_MEM_PER_END_INT_CLR**
    ///
    /// The clear bit for SPI_MEM_PER_END_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_PER_END_INT_CLR_OFFSET: u32 = 0;
    pub const SPI_MEM_PER_END_INT_CLR_MASK: u32 = 0x00000001;
    pub const SPI_MEM_PER_END_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SPI_MEM_BROWN_OUT_INT_CLR** (WO)
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_brown_out_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_BROWN_OUT_INT_CLR_MASK);
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

    /// Trigger **SPI_MEM_WPE_END_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_wpe_end_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_WPE_END_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_PES_END_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_pes_end_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_PES_END_INT_CLR_MASK);
    }

    /// Trigger **SPI_MEM_PER_END_INT_CLR** (WO)
    ///
    /// The clear bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub fn trigger_spi_mem_per_end_int_clr(&self) {
        self.value.write_bits(Self::SPI_MEM_PER_END_INT_CLR_MASK);
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

    /// Set **SPI_MEM_BROWN_OUT_INT_CLR** `[10:10]`
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_brown_out_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_BROWN_OUT_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_BROWN_OUT_INT_CLR_MASK
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

    /// Set **SPI_MEM_WPE_END_INT_CLR** `[2:2]`
    ///
    /// The clear bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_wpe_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_WPE_END_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_WPE_END_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_PES_END_INT_CLR** `[1:1]`
    ///
    /// The clear bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_pes_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_PES_END_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_PES_END_INT_CLR_MASK
            },
        }
    }

    /// Set **SPI_MEM_PER_END_INT_CLR** `[0:0]`
    ///
    /// The clear bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub const fn with_spi_mem_per_end_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntClr::SPI_MEM_PER_END_INT_CLR_MASK
            } else {
                self.bits & !SpiMemIntClr::SPI_MEM_PER_END_INT_CLR_MASK
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
/// SPI1 interrupt raw register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **SPI_MEM_BROWN_OUT_INT_RAW** `[10]` (RW)
///
///   The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others.
///
/// - **SPI_MEM_MST_ST_END_INT_RAW** `[4]` (RW)
///
///   The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others.
///
/// - **SPI_MEM_SLV_ST_END_INT_RAW** `[3]` (RW)
///
///   The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
///
/// - **SPI_MEM_WPE_END_INT_RAW** `[2]` (RW)
///
///   The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others.
///
/// - **SPI_MEM_PES_END_INT_RAW** `[1]` (RW)
///
///   The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others.
///
/// - **SPI_MEM_PER_END_INT_RAW** `[0]` (RW)
///
///   The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others.
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

    /// **SPI_MEM_BROWN_OUT_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_BROWN_OUT_INT_RAW_OFFSET: u32 = 10;
    pub const SPI_MEM_BROWN_OUT_INT_RAW_MASK: u32 = 0x00000400;
    pub const SPI_MEM_BROWN_OUT_INT_RAW: u32 = 1 << 10;

    /// **SPI_MEM_MST_ST_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_MEM_MST_ST_END_INT_RAW_OFFSET: u32 = 4;
    pub const SPI_MEM_MST_ST_END_INT_RAW_MASK: u32 = 0x00000010;
    pub const SPI_MEM_MST_ST_END_INT_RAW: u32 = 1 << 4;

    /// **SPI_MEM_SLV_ST_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_MEM_SLV_ST_END_INT_RAW_OFFSET: u32 = 3;
    pub const SPI_MEM_SLV_ST_END_INT_RAW_MASK: u32 = 0x00000008;
    pub const SPI_MEM_SLV_ST_END_INT_RAW: u32 = 1 << 3;

    /// **SPI_MEM_WPE_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_WPE_END_INT_RAW_OFFSET: u32 = 2;
    pub const SPI_MEM_WPE_END_INT_RAW_MASK: u32 = 0x00000004;
    pub const SPI_MEM_WPE_END_INT_RAW: u32 = 1 << 2;

    /// **SPI_MEM_PES_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_PES_END_INT_RAW_OFFSET: u32 = 1;
    pub const SPI_MEM_PES_END_INT_RAW_MASK: u32 = 0x00000002;
    pub const SPI_MEM_PES_END_INT_RAW: u32 = 1 << 1;

    /// **SPI_MEM_PER_END_INT_RAW**
    ///
    /// The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_PER_END_INT_RAW_OFFSET: u32 = 0;
    pub const SPI_MEM_PER_END_INT_RAW_MASK: u32 = 0x00000001;
    pub const SPI_MEM_PER_END_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_BROWN_OUT_INT_RAW** `[10]` (RW)
    ///
    /// The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others.
    #[inline(always)]
    pub fn spi_mem_brown_out_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BROWN_OUT_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_BROWN_OUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_brown_out_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_BROWN_OUT_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_BROWN_OUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_brown_out_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_BROWN_OUT_INT_RAW_MASK);
    }

    /// **SPI_MEM_MST_ST_END_INT_RAW** `[4]` (RW)
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others.
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
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
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

    /// **SPI_MEM_WPE_END_INT_RAW** `[2]` (RW)
    ///
    /// The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others.
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WPE_END_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_WPE_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_wpe_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_WPE_END_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_WPE_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_wpe_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_WPE_END_INT_RAW_MASK);
    }

    /// **SPI_MEM_PES_END_INT_RAW** `[1]` (RW)
    ///
    /// The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others.
    #[inline(always)]
    pub fn spi_mem_pes_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PES_END_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_PES_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_pes_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PES_END_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_PES_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_pes_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PES_END_INT_RAW_MASK);
    }

    /// **SPI_MEM_PER_END_INT_RAW** `[0]` (RW)
    ///
    /// The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others.
    #[inline(always)]
    pub fn spi_mem_per_end_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PER_END_INT_RAW_MASK) != 0
    }

    /// Set SPI_MEM_PER_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_per_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SPI_MEM_PER_END_INT_RAW_MASK);
    }

    /// Clear SPI_MEM_PER_END_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_per_end_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SPI_MEM_PER_END_INT_RAW_MASK);
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

    /// Read **SPI_MEM_BROWN_OUT_INT_RAW** `[10:10]`
    ///
    /// The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_brown_out_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_BROWN_OUT_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_MST_ST_END_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_mst_st_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_MST_ST_END_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_SLV_ST_END_INT_RAW** `[3:3]`
    ///
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    pub const fn spi_mem_slv_st_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_SLV_ST_END_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_WPE_END_INT_RAW** `[2:2]`
    ///
    /// The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_wpe_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_WPE_END_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_PES_END_INT_RAW** `[1:1]`
    ///
    /// The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_pes_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_PES_END_INT_RAW_MASK) != 0
    }

    /// Read **SPI_MEM_PER_END_INT_RAW** `[0:0]`
    ///
    /// The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others.
    #[inline(always)]
    pub const fn spi_mem_per_end_int_raw(&self) -> bool {
        (self.bits & SpiMemIntRaw::SPI_MEM_PER_END_INT_RAW_MASK) != 0
    }

    /// Set **SPI_MEM_BROWN_OUT_INT_RAW** `[10:10]`
    ///
    /// The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_brown_out_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_BROWN_OUT_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_BROWN_OUT_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_MST_ST_END_INT_RAW** `[4:4]`
    ///
    /// The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others.
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
    /// The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
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

    /// Set **SPI_MEM_WPE_END_INT_RAW** `[2:2]`
    ///
    /// The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_wpe_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_WPE_END_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_WPE_END_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_PES_END_INT_RAW** `[1:1]`
    ///
    /// The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_pes_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_PES_END_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_PES_END_INT_RAW_MASK
            },
        }
    }

    /// Set **SPI_MEM_PER_END_INT_RAW** `[0:0]`
    ///
    /// The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others.
    #[inline(always)]
    pub const fn with_spi_mem_per_end_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemIntRaw::SPI_MEM_PER_END_INT_RAW_MASK
            } else {
                self.bits & !SpiMemIntRaw::SPI_MEM_PER_END_INT_RAW_MASK
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
/// SPI1 interrupt status register
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **SPI_MEM_BROWN_OUT_INT_ST** `[10]` (RO)
///
///   The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
///
/// - **SPI_MEM_MST_ST_END_INT_ST** `[4]` (RO)
///
///   The status bit for SPI_MEM_MST_ST_END_INT interrupt.
///
/// - **SPI_MEM_SLV_ST_END_INT_ST** `[3]` (RO)
///
///   The status bit for SPI_MEM_SLV_ST_END_INT interrupt.
///
/// - **SPI_MEM_WPE_END_INT_ST** `[2]` (RO)
///
///   The status bit for SPI_MEM_WPE_END_INT interrupt.
///
/// - **SPI_MEM_PES_END_INT_ST** `[1]` (RO)
///
///   The status bit for SPI_MEM_PES_END_INT interrupt.
///
/// - **SPI_MEM_PER_END_INT_ST** `[0]` (RO)
///
///   The status bit for SPI_MEM_PER_END_INT interrupt.
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

    /// **SPI_MEM_BROWN_OUT_INT_ST**
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SPI_MEM_BROWN_OUT_INT_ST_OFFSET: u32 = 10;
    pub const SPI_MEM_BROWN_OUT_INT_ST_MASK: u32 = 0x00000400;
    pub const SPI_MEM_BROWN_OUT_INT_ST: u32 = 1 << 10;

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

    /// **SPI_MEM_WPE_END_INT_ST**
    ///
    /// The status bit for SPI_MEM_WPE_END_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_MEM_WPE_END_INT_ST_OFFSET: u32 = 2;
    pub const SPI_MEM_WPE_END_INT_ST_MASK: u32 = 0x00000004;
    pub const SPI_MEM_WPE_END_INT_ST: u32 = 1 << 2;

    /// **SPI_MEM_PES_END_INT_ST**
    ///
    /// The status bit for SPI_MEM_PES_END_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_MEM_PES_END_INT_ST_OFFSET: u32 = 1;
    pub const SPI_MEM_PES_END_INT_ST_MASK: u32 = 0x00000002;
    pub const SPI_MEM_PES_END_INT_ST: u32 = 1 << 1;

    /// **SPI_MEM_PER_END_INT_ST**
    ///
    /// The status bit for SPI_MEM_PER_END_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_PER_END_INT_ST_OFFSET: u32 = 0;
    pub const SPI_MEM_PER_END_INT_ST_MASK: u32 = 0x00000001;
    pub const SPI_MEM_PER_END_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_BROWN_OUT_INT_ST** `[10]` (RO)
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_brown_out_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_BROWN_OUT_INT_ST_MASK) != 0
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

    /// **SPI_MEM_WPE_END_INT_ST** `[2]` (RO)
    ///
    /// The status bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_WPE_END_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_PES_END_INT_ST** `[1]` (RO)
    ///
    /// The status bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_pes_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PES_END_INT_ST_MASK) != 0
    }

    /// **SPI_MEM_PER_END_INT_ST** `[0]` (RO)
    ///
    /// The status bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub fn spi_mem_per_end_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_PER_END_INT_ST_MASK) != 0
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

    /// Read **SPI_MEM_BROWN_OUT_INT_ST** `[10:10]`
    ///
    /// The status bit for SPI_MEM_BROWN_OUT_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_brown_out_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_BROWN_OUT_INT_ST_MASK) != 0
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

    /// Read **SPI_MEM_WPE_END_INT_ST** `[2:2]`
    ///
    /// The status bit for SPI_MEM_WPE_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_wpe_end_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_WPE_END_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_PES_END_INT_ST** `[1:1]`
    ///
    /// The status bit for SPI_MEM_PES_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_pes_end_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_PES_END_INT_ST_MASK) != 0
    }

    /// Read **SPI_MEM_PER_END_INT_ST** `[0:0]`
    ///
    /// The status bit for SPI_MEM_PER_END_INT interrupt.
    #[inline(always)]
    pub const fn spi_mem_per_end_int_st(&self) -> bool {
        (self.bits & SpiMemIntSt::SPI_MEM_PER_END_INT_ST_MASK) != 0
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
/// SPI1 DDR control register
///
/// Address offset: 0x00d4
/// Reset value: 0x00000020
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
/// - **SPI_FMEM_OUTMINBYTELEN** `[11:5]` (RW)
///
///   It is the minimum output data length in the panda device.
///
/// - **SPI_FMEM_DDR_CMD_DIS** `[4]` (RW)
///
///   the bit is used to disable dual edge in command phase when ddr mode.
///
/// - **SPI_FMEM_DDR_WDAT_SWP** `[3]` (RW)
///
///   Set the bit to reorder tx data of the word in spi ddr mode.
///
/// - **SPI_FMEM_DDR_RDAT_SWP** `[2]` (RW)
///
///   Set the bit to reorder rx data of the word in spi ddr mode.
///
/// - **SPI_FMEM_VAR_DUMMY** `[1]` (RW)
///
///   Set the bit to enable variable dummy cycle in spi ddr mode.
///
/// - **SPI_FMEM_DDR_EN** `[0]` (RW)
///
///   1: in ddr mode, 0 in sdr mode
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

    /// **SPI_FMEM_OUTMINBYTELEN**
    ///
    /// It is the minimum output data length in the panda device.
    ///
    /// Bits `[11:5]`, mask `0x00000fe0`
    pub const SPI_FMEM_OUTMINBYTELEN_OFFSET: u32 = 5;
    pub const SPI_FMEM_OUTMINBYTELEN_MASK: u32 = 0x00000fe0;

    /// **SPI_FMEM_DDR_CMD_DIS**
    ///
    /// the bit is used to disable dual edge in command phase when ddr mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SPI_FMEM_DDR_CMD_DIS_OFFSET: u32 = 4;
    pub const SPI_FMEM_DDR_CMD_DIS_MASK: u32 = 0x00000010;
    pub const SPI_FMEM_DDR_CMD_DIS: u32 = 1 << 4;

    /// **SPI_FMEM_DDR_WDAT_SWP**
    ///
    /// Set the bit to reorder tx data of the word in spi ddr mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SPI_FMEM_DDR_WDAT_SWP_OFFSET: u32 = 3;
    pub const SPI_FMEM_DDR_WDAT_SWP_MASK: u32 = 0x00000008;
    pub const SPI_FMEM_DDR_WDAT_SWP: u32 = 1 << 3;

    /// **SPI_FMEM_DDR_RDAT_SWP**
    ///
    /// Set the bit to reorder rx data of the word in spi ddr mode.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SPI_FMEM_DDR_RDAT_SWP_OFFSET: u32 = 2;
    pub const SPI_FMEM_DDR_RDAT_SWP_MASK: u32 = 0x00000004;
    pub const SPI_FMEM_DDR_RDAT_SWP: u32 = 1 << 2;

    /// **SPI_FMEM_VAR_DUMMY**
    ///
    /// Set the bit to enable variable dummy cycle in spi ddr mode.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SPI_FMEM_VAR_DUMMY_OFFSET: u32 = 1;
    pub const SPI_FMEM_VAR_DUMMY_MASK: u32 = 0x00000002;
    pub const SPI_FMEM_VAR_DUMMY: u32 = 1 << 1;

    /// **SPI_FMEM_DDR_EN**
    ///
    /// 1: in ddr mode, 0 in sdr mode
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
    /// the bit is used to disable dual edge in command phase when ddr mode.
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
    /// Set the bit to reorder tx data of the word in spi ddr mode.
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
    /// Set the bit to reorder rx data of the word in spi ddr mode.
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
    /// Set the bit to enable variable dummy cycle in spi ddr mode.
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
    /// 1: in ddr mode, 0 in sdr mode
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
        Self { bits: 0x00000020 }
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
    /// the bit is used to disable dual edge in command phase when ddr mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_cmd_dis(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_CMD_DIS_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_WDAT_SWP** `[3:3]`
    ///
    /// Set the bit to reorder tx data of the word in spi ddr mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_wdat_swp(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_WDAT_SWP_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_RDAT_SWP** `[2:2]`
    ///
    /// Set the bit to reorder rx data of the word in spi ddr mode.
    #[inline(always)]
    pub const fn spi_fmem_ddr_rdat_swp(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_DDR_RDAT_SWP_MASK) != 0
    }

    /// Read **SPI_FMEM_VAR_DUMMY** `[1:1]`
    ///
    /// Set the bit to enable variable dummy cycle in spi ddr mode.
    #[inline(always)]
    pub const fn spi_fmem_var_dummy(&self) -> bool {
        (self.bits & SpiMemDdr::SPI_FMEM_VAR_DUMMY_MASK) != 0
    }

    /// Read **SPI_FMEM_DDR_EN** `[0:0]`
    ///
    /// 1: in ddr mode, 0 in sdr mode
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
    /// the bit is used to disable dual edge in command phase when ddr mode.
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
    /// Set the bit to reorder tx data of the word in spi ddr mode.
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
    /// Set the bit to reorder rx data of the word in spi ddr mode.
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
    /// Set the bit to enable variable dummy cycle in spi ddr mode.
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
    /// 1: in ddr mode, 0 in sdr mode
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

/// SPI_MEM_TIMING_CALI Register
///
/// SPI1 timing control register
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **SPI_MEM_EXTRA_DUMMY_CYCLELEN** `[4:2]` (RW)
///
///   add extra dummy spi clock cycle length for spi clock calibration.
///
/// - **SPI_MEM_TIMING_CALI** `[1]` (RW)
///
///   The bit is used to enable timing auto-calibration for all reading operations.
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

    // ======== Field Accessors ========

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
}

impl Default for SpiMemTimingCaliVal {
    fn default() -> Self {
        Self { bits: 0 }
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

/// SPI_MEM_CLOCK_GATE Register
///
/// SPI1 clk_gate register
///
/// Address offset: 0x0200
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **SPI_MEM_CLK_EN** `[0]` (RW)
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

    /// **SPI_MEM_CLK_EN**
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SPI_MEM_CLK_EN_OFFSET: u32 = 0;
    pub const SPI_MEM_CLK_EN_MASK: u32 = 0x00000001;
    pub const SPI_MEM_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPI_MEM_CLK_EN** `[0]` (RW)
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    #[inline(always)]
    pub fn spi_mem_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::SPI_MEM_CLK_EN_MASK) != 0
    }

    /// Set SPI_MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_mem_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::SPI_MEM_CLK_EN_MASK);
    }

    /// Clear SPI_MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_mem_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_MEM_CLK_EN_MASK);
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

    /// Read **SPI_MEM_CLK_EN** `[0:0]`
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    #[inline(always)]
    pub const fn spi_mem_clk_en(&self) -> bool {
        (self.bits & SpiMemClockGate::SPI_MEM_CLK_EN_MASK) != 0
    }

    /// Set **SPI_MEM_CLK_EN** `[0:0]`
    ///
    /// Register clock gate enable signal. 1: Enable. 0: Disable.
    #[inline(always)]
    pub const fn with_spi_mem_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SpiMemClockGate::SPI_MEM_CLK_EN_MASK
            } else {
                self.bits & !SpiMemClockGate::SPI_MEM_CLK_EN_MASK
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

/// SPI_MEM_DATE Register
///
/// Version control register
///
/// Address offset: 0x03fc
/// Reset value: 0x02111240
///
/// ## Fields
///
/// - **SPI_MEM_DATE** `[27:0]` (RW)
///
///   Version control register
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
    /// Version control register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const SPI_MEM_DATE_OFFSET: u32 = 0;
    pub const SPI_MEM_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **SPI_MEM_DATE** `[27:0]` (RW)
    ///
    /// Version control register
    #[inline(always)]
    pub fn spi_mem_date(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_MEM_DATE_MASK) >> Self::SPI_MEM_DATE_OFFSET
    }

    /// Write **SPI_MEM_DATE** `[27:0]` (RW)
    ///
    /// Version control register
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
        Self { bits: 0x02111240 }
    }

    /// Read **SPI_MEM_DATE** `[27:0]`
    ///
    /// Version control register
    #[inline(always)]
    pub const fn spi_mem_date(&self) -> u32 {
        (self.bits & SpiMemDate::SPI_MEM_DATE_MASK) >> SpiMemDate::SPI_MEM_DATE_OFFSET
    }

    /// Set **SPI_MEM_DATE** `[27:0]`
    ///
    /// Version control register
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
    /// SPI1 memory command register
    pub spi_mem_cmd: SpiMemCmd,
    /// 0x0004
    /// SPI1 address register
    pub spi_mem_addr: SpiMemAddr,
    /// 0x0008
    /// SPI1 control register.
    pub spi_mem_ctrl: SpiMemCtrl,
    /// 0x000c
    /// SPI1 control1 register.
    pub spi_mem_ctrl1: SpiMemCtrl1,
    /// 0x0010
    /// SPI1 control2 register.
    pub spi_mem_ctrl2: SpiMemCtrl2,
    /// 0x0014
    /// SPI1 clock division control register.
    pub spi_mem_clock: SpiMemClock,
    /// 0x0018
    /// SPI1 user register.
    pub spi_mem_user: SpiMemUser,
    /// 0x001c
    /// SPI1 user1 register.
    pub spi_mem_user1: SpiMemUser1,
    /// 0x0020
    /// SPI1 user2 register.
    pub spi_mem_user2: SpiMemUser2,
    /// 0x0024
    /// SPI1 send data bit length control register.
    pub spi_mem_mosi_dlen: SpiMemMosiDlen,
    /// 0x0028
    /// SPI1 receive data bit length control register.
    pub spi_mem_miso_dlen: SpiMemMisoDlen,
    /// 0x002c
    /// SPI1 status register.
    pub spi_mem_rd_status: SpiMemRdStatus,
    _reserved0: [u8; 4],
    /// 0x0034
    /// SPI1 misc register
    pub spi_mem_misc: SpiMemMisc,
    /// 0x0038
    /// SPI1 TX CRC data register.
    pub spi_mem_tx_crc: SpiMemTxCrc,
    /// 0x003c
    /// SPI1 bit mode control register.
    pub spi_mem_cache_fctrl: SpiMemCacheFctrl,
    _reserved1: [u8; 24],
    /// 0x0058
    /// SPI1 memory data buffer0
    pub spi_mem_w0: SpiMemW0,
    /// 0x005c
    /// SPI1 memory data buffer1
    pub spi_mem_w1: SpiMemW1,
    /// 0x0060
    /// SPI1 memory data buffer2
    pub spi_mem_w2: SpiMemW2,
    /// 0x0064
    /// SPI1 memory data buffer3
    pub spi_mem_w3: SpiMemW3,
    /// 0x0068
    /// SPI1 memory data buffer4
    pub spi_mem_w4: SpiMemW4,
    /// 0x006c
    /// SPI1 memory data buffer5
    pub spi_mem_w5: SpiMemW5,
    /// 0x0070
    /// SPI1 memory data buffer6
    pub spi_mem_w6: SpiMemW6,
    /// 0x0074
    /// SPI1 memory data buffer7
    pub spi_mem_w7: SpiMemW7,
    /// 0x0078
    /// SPI1 memory data buffer8
    pub spi_mem_w8: SpiMemW8,
    /// 0x007c
    /// SPI1 memory data buffer9
    pub spi_mem_w9: SpiMemW9,
    /// 0x0080
    /// SPI1 memory data buffer10
    pub spi_mem_w10: SpiMemW10,
    /// 0x0084
    /// SPI1 memory data buffer11
    pub spi_mem_w11: SpiMemW11,
    /// 0x0088
    /// SPI1 memory data buffer12
    pub spi_mem_w12: SpiMemW12,
    /// 0x008c
    /// SPI1 memory data buffer13
    pub spi_mem_w13: SpiMemW13,
    /// 0x0090
    /// SPI1 memory data buffer14
    pub spi_mem_w14: SpiMemW14,
    /// 0x0094
    /// SPI1 memory data buffer15
    pub spi_mem_w15: SpiMemW15,
    /// 0x0098
    /// SPI1 wait idle control register
    pub spi_mem_flash_waiti_ctrl: SpiMemFlashWaitiCtrl,
    /// 0x009c
    /// SPI1 flash suspend control register
    pub spi_mem_flash_sus_ctrl: SpiMemFlashSusCtrl,
    /// 0x00a0
    /// SPI1 flash suspend command register
    pub spi_mem_flash_sus_cmd: SpiMemFlashSusCmd,
    /// 0x00a4
    /// SPI1 flash suspend status register
    pub spi_mem_sus_status: SpiMemSusStatus,
    _reserved2: [u8; 24],
    /// 0x00c0
    /// SPI1 interrupt enable register
    pub spi_mem_int_ena: SpiMemIntEna,
    /// 0x00c4
    /// SPI1 interrupt clear register
    pub spi_mem_int_clr: SpiMemIntClr,
    /// 0x00c8
    /// SPI1 interrupt raw register
    pub spi_mem_int_raw: SpiMemIntRaw,
    /// 0x00cc
    /// SPI1 interrupt status register
    pub spi_mem_int_st: SpiMemIntSt,
    _reserved3: [u8; 4],
    /// 0x00d4
    /// SPI1 DDR control register
    pub spi_mem_ddr: SpiMemDdr,
    _reserved4: [u8; 168],
    /// 0x0180
    /// SPI1 timing control register
    pub spi_mem_timing_cali: SpiMemTimingCali,
    _reserved5: [u8; 124],
    /// 0x0200
    /// SPI1 clk_gate register
    pub spi_mem_clock_gate: SpiMemClockGate,
    _reserved6: [u8; 504],
    /// 0x03fc
    /// Version control register
    pub spi_mem_date: SpiMemDate,
}
