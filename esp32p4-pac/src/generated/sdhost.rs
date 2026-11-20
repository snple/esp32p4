//! SDHOST
//! SD/MMC Host Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x50083000;

/// CTRL Register
///
/// Control register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **USE_INTERNAL_DMA** `[25]` (RW)
///
///   0: the data FIFO is accessed through the APB interface. 1: the internal DMA engine moves the FIFO.
///
/// - **CEATA_DEVICE_INTERRUPT_STATUS** `[11]` (RW)
///
///   Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit.
///
/// - **SEND_AUTO_STOP_CCSD** `[10]` (RW)
///
///   Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit.
///
/// - **SEND_CCSD** `[9]` (RW)
///
///   When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked.
///   NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS.
///
/// - **ABORT_READ_DATA** `[8]` (RW)
///
///   After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle.
///
/// - **SEND_IRQ_RESPONSE** `[7]` (RW)
///
///   Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state.
///
/// - **READ_WAIT** `[6]` (RW)
///
///   For sending read-wait to SDIO cards.
///
/// - **DMA_ENABLE** `[5]` (RW)
///
///   Enable the internal DMA (IDMAC) to move the data FIFO. 0: CPU reads and writes SDHOST_BUFFIFO_REG itself.
///
/// - **INT_ENABLE** `[4]` (RW)
///
///   Global interrupt enable/disable bit. 0: Disable; 1: Enable.
///
/// - **DMA_RESET** `[2]` (RW)
///
///   To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
///
/// - **FIFO_RESET** `[1]` (RW)
///
///   To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
///   Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared.
///
/// - **CONTROLLER_RESET** `[0]` (RW)
///
///   To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles.
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **USE_INTERNAL_DMA**
    ///
    /// 0: the data FIFO is accessed through the APB interface. 1: the internal DMA engine moves the FIFO.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USE_INTERNAL_DMA_OFFSET: u32 = 25;
    pub const USE_INTERNAL_DMA_MASK: u32 = 0x02000000;
    pub const USE_INTERNAL_DMA: u32 = 1 << 25;

    /// **CEATA_DEVICE_INTERRUPT_STATUS**
    ///
    /// Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CEATA_DEVICE_INTERRUPT_STATUS_OFFSET: u32 = 11;
    pub const CEATA_DEVICE_INTERRUPT_STATUS_MASK: u32 = 0x00000800;
    pub const CEATA_DEVICE_INTERRUPT_STATUS: u32 = 1 << 11;

    /// **SEND_AUTO_STOP_CCSD**
    ///
    /// Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SEND_AUTO_STOP_CCSD_OFFSET: u32 = 10;
    pub const SEND_AUTO_STOP_CCSD_MASK: u32 = 0x00000400;
    pub const SEND_AUTO_STOP_CCSD: u32 = 1 << 10;

    /// **SEND_CCSD**
    ///
    /// When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked.
    /// NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SEND_CCSD_OFFSET: u32 = 9;
    pub const SEND_CCSD_MASK: u32 = 0x00000200;
    pub const SEND_CCSD: u32 = 1 << 9;

    /// **ABORT_READ_DATA**
    ///
    /// After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ABORT_READ_DATA_OFFSET: u32 = 8;
    pub const ABORT_READ_DATA_MASK: u32 = 0x00000100;
    pub const ABORT_READ_DATA: u32 = 1 << 8;

    /// **SEND_IRQ_RESPONSE**
    ///
    /// Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SEND_IRQ_RESPONSE_OFFSET: u32 = 7;
    pub const SEND_IRQ_RESPONSE_MASK: u32 = 0x00000080;
    pub const SEND_IRQ_RESPONSE: u32 = 1 << 7;

    /// **READ_WAIT**
    ///
    /// For sending read-wait to SDIO cards.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const READ_WAIT_OFFSET: u32 = 6;
    pub const READ_WAIT_MASK: u32 = 0x00000040;
    pub const READ_WAIT: u32 = 1 << 6;

    /// **DMA_ENABLE**
    ///
    /// Enable the internal DMA (IDMAC) to move the data FIFO. 0: CPU reads and writes SDHOST_BUFFIFO_REG itself.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMA_ENABLE_OFFSET: u32 = 5;
    pub const DMA_ENABLE_MASK: u32 = 0x00000020;
    pub const DMA_ENABLE: u32 = 1 << 5;

    /// **INT_ENABLE**
    ///
    /// Global interrupt enable/disable bit. 0: Disable; 1: Enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const INT_ENABLE_OFFSET: u32 = 4;
    pub const INT_ENABLE_MASK: u32 = 0x00000010;
    pub const INT_ENABLE: u32 = 1 << 4;

    /// **DMA_RESET**
    ///
    /// To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMA_RESET_OFFSET: u32 = 2;
    pub const DMA_RESET_MASK: u32 = 0x00000004;
    pub const DMA_RESET: u32 = 1 << 2;

    /// **FIFO_RESET**
    ///
    /// To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
    /// Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FIFO_RESET_OFFSET: u32 = 1;
    pub const FIFO_RESET_MASK: u32 = 0x00000002;
    pub const FIFO_RESET: u32 = 1 << 1;

    /// **CONTROLLER_RESET**
    ///
    /// To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CONTROLLER_RESET_OFFSET: u32 = 0;
    pub const CONTROLLER_RESET_MASK: u32 = 0x00000001;
    pub const CONTROLLER_RESET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USE_INTERNAL_DMA** `[25]` (RW)
    ///
    /// 0: the data FIFO is accessed through the APB interface. 1: the internal DMA engine moves the FIFO.
    #[inline(always)]
    pub fn use_internal_dma(&self) -> bool {
        (self.value.read_bits() & Self::USE_INTERNAL_DMA_MASK) != 0
    }

    /// Set USE_INTERNAL_DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_use_internal_dma(&self) {
        self.value.modify_bits(|r| r | Self::USE_INTERNAL_DMA_MASK);
    }

    /// Clear USE_INTERNAL_DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_use_internal_dma(&self) {
        self.value.modify_bits(|r| r & !Self::USE_INTERNAL_DMA_MASK);
    }

    /// **CEATA_DEVICE_INTERRUPT_STATUS** `[11]` (RW)
    ///
    /// Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit.
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> bool {
        (self.value.read_bits() & Self::CEATA_DEVICE_INTERRUPT_STATUS_MASK) != 0
    }

    /// Set CEATA_DEVICE_INTERRUPT_STATUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ceata_device_interrupt_status(&self) {
        self.value
            .modify_bits(|r| r | Self::CEATA_DEVICE_INTERRUPT_STATUS_MASK);
    }

    /// Clear CEATA_DEVICE_INTERRUPT_STATUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ceata_device_interrupt_status(&self) {
        self.value
            .modify_bits(|r| r & !Self::CEATA_DEVICE_INTERRUPT_STATUS_MASK);
    }

    /// **SEND_AUTO_STOP_CCSD** `[10]` (RW)
    ///
    /// Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit.
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> bool {
        (self.value.read_bits() & Self::SEND_AUTO_STOP_CCSD_MASK) != 0
    }

    /// Set SEND_AUTO_STOP_CCSD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_auto_stop_ccsd(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_AUTO_STOP_CCSD_MASK);
    }

    /// Clear SEND_AUTO_STOP_CCSD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_auto_stop_ccsd(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_AUTO_STOP_CCSD_MASK);
    }

    /// **SEND_CCSD** `[9]` (RW)
    ///
    /// When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked.
    /// NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS.
    #[inline(always)]
    pub fn send_ccsd(&self) -> bool {
        (self.value.read_bits() & Self::SEND_CCSD_MASK) != 0
    }

    /// Set SEND_CCSD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_ccsd(&self) {
        self.value.modify_bits(|r| r | Self::SEND_CCSD_MASK);
    }

    /// Clear SEND_CCSD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_ccsd(&self) {
        self.value.modify_bits(|r| r & !Self::SEND_CCSD_MASK);
    }

    /// **ABORT_READ_DATA** `[8]` (RW)
    ///
    /// After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle.
    #[inline(always)]
    pub fn abort_read_data(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_READ_DATA_MASK) != 0
    }

    /// Set ABORT_READ_DATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_abort_read_data(&self) {
        self.value.modify_bits(|r| r | Self::ABORT_READ_DATA_MASK);
    }

    /// Clear ABORT_READ_DATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_abort_read_data(&self) {
        self.value.modify_bits(|r| r & !Self::ABORT_READ_DATA_MASK);
    }

    /// **SEND_IRQ_RESPONSE** `[7]` (RW)
    ///
    /// Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state.
    #[inline(always)]
    pub fn send_irq_response(&self) -> bool {
        (self.value.read_bits() & Self::SEND_IRQ_RESPONSE_MASK) != 0
    }

    /// Set SEND_IRQ_RESPONSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_irq_response(&self) {
        self.value.modify_bits(|r| r | Self::SEND_IRQ_RESPONSE_MASK);
    }

    /// Clear SEND_IRQ_RESPONSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_irq_response(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_IRQ_RESPONSE_MASK);
    }

    /// **READ_WAIT** `[6]` (RW)
    ///
    /// For sending read-wait to SDIO cards.
    #[inline(always)]
    pub fn read_wait(&self) -> bool {
        (self.value.read_bits() & Self::READ_WAIT_MASK) != 0
    }

    /// Set READ_WAIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_read_wait(&self) {
        self.value.modify_bits(|r| r | Self::READ_WAIT_MASK);
    }

    /// Clear READ_WAIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_read_wait(&self) {
        self.value.modify_bits(|r| r & !Self::READ_WAIT_MASK);
    }

    /// **DMA_ENABLE** `[5]` (RW)
    ///
    /// Enable the internal DMA (IDMAC) to move the data FIFO. 0: CPU reads and writes SDHOST_BUFFIFO_REG itself.
    #[inline(always)]
    pub fn dma_enable(&self) -> bool {
        (self.value.read_bits() & Self::DMA_ENABLE_MASK) != 0
    }

    /// Set DMA_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_enable(&self) {
        self.value.modify_bits(|r| r | Self::DMA_ENABLE_MASK);
    }

    /// Clear DMA_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_enable(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_ENABLE_MASK);
    }

    /// **INT_ENABLE** `[4]` (RW)
    ///
    /// Global interrupt enable/disable bit. 0: Disable; 1: Enable.
    #[inline(always)]
    pub fn int_enable(&self) -> bool {
        (self.value.read_bits() & Self::INT_ENABLE_MASK) != 0
    }

    /// Set INT_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_int_enable(&self) {
        self.value.modify_bits(|r| r | Self::INT_ENABLE_MASK);
    }

    /// Clear INT_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_int_enable(&self) {
        self.value.modify_bits(|r| r & !Self::INT_ENABLE_MASK);
    }

    /// **DMA_RESET** `[2]` (RW)
    ///
    /// To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
    #[inline(always)]
    pub fn dma_reset(&self) -> bool {
        (self.value.read_bits() & Self::DMA_RESET_MASK) != 0
    }

    /// Set DMA_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_reset(&self) {
        self.value.modify_bits(|r| r | Self::DMA_RESET_MASK);
    }

    /// Clear DMA_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_reset(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_RESET_MASK);
    }

    /// **FIFO_RESET** `[1]` (RW)
    ///
    /// To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
    /// Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared.
    #[inline(always)]
    pub fn fifo_reset(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_RESET_MASK) != 0
    }

    /// Set FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fifo_reset(&self) {
        self.value.modify_bits(|r| r | Self::FIFO_RESET_MASK);
    }

    /// Clear FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fifo_reset(&self) {
        self.value.modify_bits(|r| r & !Self::FIFO_RESET_MASK);
    }

    /// **CONTROLLER_RESET** `[0]` (RW)
    ///
    /// To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles.
    #[inline(always)]
    pub fn controller_reset(&self) -> bool {
        (self.value.read_bits() & Self::CONTROLLER_RESET_MASK) != 0
    }

    /// Set CONTROLLER_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_controller_reset(&self) {
        self.value.modify_bits(|r| r | Self::CONTROLLER_RESET_MASK);
    }

    /// Clear CONTROLLER_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_controller_reset(&self) {
        self.value.modify_bits(|r| r & !Self::CONTROLLER_RESET_MASK);
    }
}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USE_INTERNAL_DMA** `[25:25]`
    ///
    /// 0: the data FIFO is accessed through the APB interface. 1: the internal DMA engine moves the FIFO.
    #[inline(always)]
    pub const fn use_internal_dma(&self) -> bool {
        (self.bits & Ctrl::USE_INTERNAL_DMA_MASK) != 0
    }

    /// Read **CEATA_DEVICE_INTERRUPT_STATUS** `[11:11]`
    ///
    /// Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit.
    #[inline(always)]
    pub const fn ceata_device_interrupt_status(&self) -> bool {
        (self.bits & Ctrl::CEATA_DEVICE_INTERRUPT_STATUS_MASK) != 0
    }

    /// Read **SEND_AUTO_STOP_CCSD** `[10:10]`
    ///
    /// Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit.
    #[inline(always)]
    pub const fn send_auto_stop_ccsd(&self) -> bool {
        (self.bits & Ctrl::SEND_AUTO_STOP_CCSD_MASK) != 0
    }

    /// Read **SEND_CCSD** `[9:9]`
    ///
    /// When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked.
    /// NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS.
    #[inline(always)]
    pub const fn send_ccsd(&self) -> bool {
        (self.bits & Ctrl::SEND_CCSD_MASK) != 0
    }

    /// Read **ABORT_READ_DATA** `[8:8]`
    ///
    /// After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle.
    #[inline(always)]
    pub const fn abort_read_data(&self) -> bool {
        (self.bits & Ctrl::ABORT_READ_DATA_MASK) != 0
    }

    /// Read **SEND_IRQ_RESPONSE** `[7:7]`
    ///
    /// Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state.
    #[inline(always)]
    pub const fn send_irq_response(&self) -> bool {
        (self.bits & Ctrl::SEND_IRQ_RESPONSE_MASK) != 0
    }

    /// Read **READ_WAIT** `[6:6]`
    ///
    /// For sending read-wait to SDIO cards.
    #[inline(always)]
    pub const fn read_wait(&self) -> bool {
        (self.bits & Ctrl::READ_WAIT_MASK) != 0
    }

    /// Read **DMA_ENABLE** `[5:5]`
    ///
    /// Enable the internal DMA (IDMAC) to move the data FIFO. 0: CPU reads and writes SDHOST_BUFFIFO_REG itself.
    #[inline(always)]
    pub const fn dma_enable(&self) -> bool {
        (self.bits & Ctrl::DMA_ENABLE_MASK) != 0
    }

    /// Read **INT_ENABLE** `[4:4]`
    ///
    /// Global interrupt enable/disable bit. 0: Disable; 1: Enable.
    #[inline(always)]
    pub const fn int_enable(&self) -> bool {
        (self.bits & Ctrl::INT_ENABLE_MASK) != 0
    }

    /// Read **DMA_RESET** `[2:2]`
    ///
    /// To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
    #[inline(always)]
    pub const fn dma_reset(&self) -> bool {
        (self.bits & Ctrl::DMA_RESET_MASK) != 0
    }

    /// Read **FIFO_RESET** `[1:1]`
    ///
    /// To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
    /// Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared.
    #[inline(always)]
    pub const fn fifo_reset(&self) -> bool {
        (self.bits & Ctrl::FIFO_RESET_MASK) != 0
    }

    /// Read **CONTROLLER_RESET** `[0:0]`
    ///
    /// To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles.
    #[inline(always)]
    pub const fn controller_reset(&self) -> bool {
        (self.bits & Ctrl::CONTROLLER_RESET_MASK) != 0
    }

    /// Set **USE_INTERNAL_DMA** `[25:25]`
    ///
    /// 0: the data FIFO is accessed through the APB interface. 1: the internal DMA engine moves the FIFO.
    #[inline(always)]
    pub const fn with_use_internal_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::USE_INTERNAL_DMA_MASK
            } else {
                self.bits & !Ctrl::USE_INTERNAL_DMA_MASK
            },
        }
    }

    /// Set **CEATA_DEVICE_INTERRUPT_STATUS** `[11:11]`
    ///
    /// Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit.
    #[inline(always)]
    pub const fn with_ceata_device_interrupt_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CEATA_DEVICE_INTERRUPT_STATUS_MASK
            } else {
                self.bits & !Ctrl::CEATA_DEVICE_INTERRUPT_STATUS_MASK
            },
        }
    }

    /// Set **SEND_AUTO_STOP_CCSD** `[10:10]`
    ///
    /// Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit.
    #[inline(always)]
    pub const fn with_send_auto_stop_ccsd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SEND_AUTO_STOP_CCSD_MASK
            } else {
                self.bits & !Ctrl::SEND_AUTO_STOP_CCSD_MASK
            },
        }
    }

    /// Set **SEND_CCSD** `[9:9]`
    ///
    /// When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked.
    /// NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS.
    #[inline(always)]
    pub const fn with_send_ccsd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SEND_CCSD_MASK
            } else {
                self.bits & !Ctrl::SEND_CCSD_MASK
            },
        }
    }

    /// Set **ABORT_READ_DATA** `[8:8]`
    ///
    /// After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle.
    #[inline(always)]
    pub const fn with_abort_read_data(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::ABORT_READ_DATA_MASK
            } else {
                self.bits & !Ctrl::ABORT_READ_DATA_MASK
            },
        }
    }

    /// Set **SEND_IRQ_RESPONSE** `[7:7]`
    ///
    /// Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state.
    #[inline(always)]
    pub const fn with_send_irq_response(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SEND_IRQ_RESPONSE_MASK
            } else {
                self.bits & !Ctrl::SEND_IRQ_RESPONSE_MASK
            },
        }
    }

    /// Set **READ_WAIT** `[6:6]`
    ///
    /// For sending read-wait to SDIO cards.
    #[inline(always)]
    pub const fn with_read_wait(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::READ_WAIT_MASK
            } else {
                self.bits & !Ctrl::READ_WAIT_MASK
            },
        }
    }

    /// Set **DMA_ENABLE** `[5:5]`
    ///
    /// Enable the internal DMA (IDMAC) to move the data FIFO. 0: CPU reads and writes SDHOST_BUFFIFO_REG itself.
    #[inline(always)]
    pub const fn with_dma_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DMA_ENABLE_MASK
            } else {
                self.bits & !Ctrl::DMA_ENABLE_MASK
            },
        }
    }

    /// Set **INT_ENABLE** `[4:4]`
    ///
    /// Global interrupt enable/disable bit. 0: Disable; 1: Enable.
    #[inline(always)]
    pub const fn with_int_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::INT_ENABLE_MASK
            } else {
                self.bits & !Ctrl::INT_ENABLE_MASK
            },
        }
    }

    /// Set **DMA_RESET** `[2:2]`
    ///
    /// To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
    #[inline(always)]
    pub const fn with_dma_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DMA_RESET_MASK
            } else {
                self.bits & !Ctrl::DMA_RESET_MASK
            },
        }
    }

    /// Set **FIFO_RESET** `[1:1]`
    ///
    /// To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
    /// Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared.
    #[inline(always)]
    pub const fn with_fifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::FIFO_RESET_MASK
            } else {
                self.bits & !Ctrl::FIFO_RESET_MASK
            },
        }
    }

    /// Set **CONTROLLER_RESET** `[0:0]`
    ///
    /// To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles.
    #[inline(always)]
    pub const fn with_controller_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CONTROLLER_RESET_MASK
            } else {
                self.bits & !Ctrl::CONTROLLER_RESET_MASK
            },
        }
    }
}

impl Default for CtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLKDIV Register
///
/// Clock divider configuration register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CLK_DIVIDER3** `[31:24]` (RW)
///
///   Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
///
/// - **CLK_DIVIDER2** `[23:16]` (RW)
///
///   Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
///
/// - **CLK_DIVIDER1** `[15:8]` (RW)
///
///   Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
///
/// - **CLK_DIVIDER0** `[7:0]` (RW)
///
///   Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
#[repr(transparent)]
pub struct Clkdiv {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clkdiv {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clkdiv {
    // ======== Field Constants ========

    /// **CLK_DIVIDER3**
    ///
    /// Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const CLK_DIVIDER3_OFFSET: u32 = 24;
    pub const CLK_DIVIDER3_MASK: u32 = 0xff000000;

    /// **CLK_DIVIDER2**
    ///
    /// Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const CLK_DIVIDER2_OFFSET: u32 = 16;
    pub const CLK_DIVIDER2_MASK: u32 = 0x00ff0000;

    /// **CLK_DIVIDER1**
    ///
    /// Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const CLK_DIVIDER1_OFFSET: u32 = 8;
    pub const CLK_DIVIDER1_MASK: u32 = 0x0000ff00;

    /// **CLK_DIVIDER0**
    ///
    /// Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CLK_DIVIDER0_OFFSET: u32 = 0;
    pub const CLK_DIVIDER0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CLK_DIVIDER3** `[31:24]` (RW)
    ///
    /// Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn clk_divider3(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIVIDER3_MASK) >> Self::CLK_DIVIDER3_OFFSET
    }

    /// Write **CLK_DIVIDER3** `[31:24]` (RW)
    ///
    /// Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn write_clk_divider3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIVIDER3_MASK)
                | ((value << Self::CLK_DIVIDER3_OFFSET) & Self::CLK_DIVIDER3_MASK)
        });
    }

    /// **CLK_DIVIDER2** `[23:16]` (RW)
    ///
    /// Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn clk_divider2(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIVIDER2_MASK) >> Self::CLK_DIVIDER2_OFFSET
    }

    /// Write **CLK_DIVIDER2** `[23:16]` (RW)
    ///
    /// Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn write_clk_divider2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIVIDER2_MASK)
                | ((value << Self::CLK_DIVIDER2_OFFSET) & Self::CLK_DIVIDER2_MASK)
        });
    }

    /// **CLK_DIVIDER1** `[15:8]` (RW)
    ///
    /// Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn clk_divider1(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIVIDER1_MASK) >> Self::CLK_DIVIDER1_OFFSET
    }

    /// Write **CLK_DIVIDER1** `[15:8]` (RW)
    ///
    /// Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn write_clk_divider1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIVIDER1_MASK)
                | ((value << Self::CLK_DIVIDER1_OFFSET) & Self::CLK_DIVIDER1_MASK)
        });
    }

    /// **CLK_DIVIDER0** `[7:0]` (RW)
    ///
    /// Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn clk_divider0(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIVIDER0_MASK) >> Self::CLK_DIVIDER0_OFFSET
    }

    /// Write **CLK_DIVIDER0** `[7:0]` (RW)
    ///
    /// Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub fn write_clk_divider0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIVIDER0_MASK)
                | ((value << Self::CLK_DIVIDER0_OFFSET) & Self::CLK_DIVIDER0_MASK)
        });
    }
}

/// CLKDIV value type for batch read/modify/write operations
///
/// Use `clkdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkdivVal {
    bits: u32,
}

impl ClkdivVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_DIVIDER3** `[31:24]`
    ///
    /// Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn clk_divider3(&self) -> u32 {
        (self.bits & Clkdiv::CLK_DIVIDER3_MASK) >> Clkdiv::CLK_DIVIDER3_OFFSET
    }

    /// Read **CLK_DIVIDER2** `[23:16]`
    ///
    /// Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn clk_divider2(&self) -> u32 {
        (self.bits & Clkdiv::CLK_DIVIDER2_MASK) >> Clkdiv::CLK_DIVIDER2_OFFSET
    }

    /// Read **CLK_DIVIDER1** `[15:8]`
    ///
    /// Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn clk_divider1(&self) -> u32 {
        (self.bits & Clkdiv::CLK_DIVIDER1_MASK) >> Clkdiv::CLK_DIVIDER1_OFFSET
    }

    /// Read **CLK_DIVIDER0** `[7:0]`
    ///
    /// Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn clk_divider0(&self) -> u32 {
        (self.bits & Clkdiv::CLK_DIVIDER0_MASK) >> Clkdiv::CLK_DIVIDER0_OFFSET
    }

    /// Set **CLK_DIVIDER3** `[31:24]`
    ///
    /// Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn with_clk_divider3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkdiv::CLK_DIVIDER3_MASK)
                | ((value << Clkdiv::CLK_DIVIDER3_OFFSET) & Clkdiv::CLK_DIVIDER3_MASK),
        }
    }

    /// Set **CLK_DIVIDER2** `[23:16]`
    ///
    /// Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn with_clk_divider2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkdiv::CLK_DIVIDER2_MASK)
                | ((value << Clkdiv::CLK_DIVIDER2_OFFSET) & Clkdiv::CLK_DIVIDER2_MASK),
        }
    }

    /// Set **CLK_DIVIDER1** `[15:8]`
    ///
    /// Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn with_clk_divider1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkdiv::CLK_DIVIDER1_MASK)
                | ((value << Clkdiv::CLK_DIVIDER1_OFFSET) & Clkdiv::CLK_DIVIDER1_MASK),
        }
    }

    /// Set **CLK_DIVIDER0** `[7:0]`
    ///
    /// Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on.
    #[inline(always)]
    pub const fn with_clk_divider0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkdiv::CLK_DIVIDER0_MASK)
                | ((value << Clkdiv::CLK_DIVIDER0_OFFSET) & Clkdiv::CLK_DIVIDER0_MASK),
        }
    }
}

impl Default for ClkdivVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clkdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkdivVal {
        ClkdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkdivVal) -> ClkdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLKSRC Register
///
/// Clock source selection register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **CLKSRC** `[3:0]` (RW)
///
///   Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
///   00 : Clock divider 0;
///   01 : Clock divider 1;
///   10 : Clock divider 2;
///   11 : Clock divider 3.
#[repr(transparent)]
pub struct Clksrc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clksrc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clksrc {
    // ======== Field Constants ========

    /// **CLKSRC**
    ///
    /// Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
    /// 00 : Clock divider 0;
    /// 01 : Clock divider 1;
    /// 10 : Clock divider 2;
    /// 11 : Clock divider 3.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const CLKSRC_OFFSET: u32 = 0;
    pub const CLKSRC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **CLKSRC** `[3:0]` (RW)
    ///
    /// Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
    /// 00 : Clock divider 0;
    /// 01 : Clock divider 1;
    /// 10 : Clock divider 2;
    /// 11 : Clock divider 3.
    #[inline(always)]
    pub fn clksrc(&self) -> u32 {
        (self.value.read_bits() & Self::CLKSRC_MASK) >> Self::CLKSRC_OFFSET
    }

    /// Write **CLKSRC** `[3:0]` (RW)
    ///
    /// Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
    /// 00 : Clock divider 0;
    /// 01 : Clock divider 1;
    /// 10 : Clock divider 2;
    /// 11 : Clock divider 3.
    #[inline(always)]
    pub fn write_clksrc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLKSRC_MASK) | ((value << Self::CLKSRC_OFFSET) & Self::CLKSRC_MASK)
        });
    }
}

/// CLKSRC value type for batch read/modify/write operations
///
/// Use `clksrc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksrc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClksrcVal {
    bits: u32,
}

impl ClksrcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLKSRC** `[3:0]`
    ///
    /// Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
    /// 00 : Clock divider 0;
    /// 01 : Clock divider 1;
    /// 10 : Clock divider 2;
    /// 11 : Clock divider 3.
    #[inline(always)]
    pub const fn clksrc(&self) -> u32 {
        (self.bits & Clksrc::CLKSRC_MASK) >> Clksrc::CLKSRC_OFFSET
    }

    /// Set **CLKSRC** `[3:0]`
    ///
    /// Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit[1:0] are assigned for card 0, bit[3:2] are assigned for card 1. Card 0 maps and internally routes clock divider[0:3] outputs to cclk_out[1:0] pins, depending on bit value.
    /// 00 : Clock divider 0;
    /// 01 : Clock divider 1;
    /// 10 : Clock divider 2;
    /// 11 : Clock divider 3.
    #[inline(always)]
    pub const fn with_clksrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clksrc::CLKSRC_MASK)
                | ((value << Clksrc::CLKSRC_OFFSET) & Clksrc::CLKSRC_MASK),
        }
    }
}

impl Default for ClksrcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clksrc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClksrcVal {
        ClksrcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClksrcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClksrcVal) -> ClksrcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLKENA Register
///
/// Clock enable register
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **LP_ENABLE** `[17:16]` (RW)
///
///   Disable clock when the card is in IDLE state. One bit per card.
///   0: clock disabled;
///   1: clock enabled.
///
/// - **CCLK_ENABLE** `[1:0]` (RW)
///
///   Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
///   0: Clock disabled;
///   1: Clock enabled.
#[repr(transparent)]
pub struct Clkena {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clkena {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clkena {
    // ======== Field Constants ========

    /// **LP_ENABLE**
    ///
    /// Disable clock when the card is in IDLE state. One bit per card.
    /// 0: clock disabled;
    /// 1: clock enabled.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const LP_ENABLE_OFFSET: u32 = 16;
    pub const LP_ENABLE_MASK: u32 = 0x00030000;

    /// **CCLK_ENABLE**
    ///
    /// Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
    /// 0: Clock disabled;
    /// 1: Clock enabled.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CCLK_ENABLE_OFFSET: u32 = 0;
    pub const CCLK_ENABLE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LP_ENABLE** `[17:16]` (RW)
    ///
    /// Disable clock when the card is in IDLE state. One bit per card.
    /// 0: clock disabled;
    /// 1: clock enabled.
    #[inline(always)]
    pub fn lp_enable(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ENABLE_MASK) >> Self::LP_ENABLE_OFFSET
    }

    /// Write **LP_ENABLE** `[17:16]` (RW)
    ///
    /// Disable clock when the card is in IDLE state. One bit per card.
    /// 0: clock disabled;
    /// 1: clock enabled.
    #[inline(always)]
    pub fn write_lp_enable(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ENABLE_MASK) | ((value << Self::LP_ENABLE_OFFSET) & Self::LP_ENABLE_MASK)
        });
    }

    /// **CCLK_ENABLE** `[1:0]` (RW)
    ///
    /// Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
    /// 0: Clock disabled;
    /// 1: Clock enabled.
    #[inline(always)]
    pub fn cclk_enable(&self) -> u32 {
        (self.value.read_bits() & Self::CCLK_ENABLE_MASK) >> Self::CCLK_ENABLE_OFFSET
    }

    /// Write **CCLK_ENABLE** `[1:0]` (RW)
    ///
    /// Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
    /// 0: Clock disabled;
    /// 1: Clock enabled.
    #[inline(always)]
    pub fn write_cclk_enable(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLK_ENABLE_MASK)
                | ((value << Self::CCLK_ENABLE_OFFSET) & Self::CCLK_ENABLE_MASK)
        });
    }
}

/// CLKENA value type for batch read/modify/write operations
///
/// Use `clkena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkenaVal {
    bits: u32,
}

impl ClkenaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ENABLE** `[17:16]`
    ///
    /// Disable clock when the card is in IDLE state. One bit per card.
    /// 0: clock disabled;
    /// 1: clock enabled.
    #[inline(always)]
    pub const fn lp_enable(&self) -> u32 {
        (self.bits & Clkena::LP_ENABLE_MASK) >> Clkena::LP_ENABLE_OFFSET
    }

    /// Read **CCLK_ENABLE** `[1:0]`
    ///
    /// Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
    /// 0: Clock disabled;
    /// 1: Clock enabled.
    #[inline(always)]
    pub const fn cclk_enable(&self) -> u32 {
        (self.bits & Clkena::CCLK_ENABLE_MASK) >> Clkena::CCLK_ENABLE_OFFSET
    }

    /// Set **LP_ENABLE** `[17:16]`
    ///
    /// Disable clock when the card is in IDLE state. One bit per card.
    /// 0: clock disabled;
    /// 1: clock enabled.
    #[inline(always)]
    pub const fn with_lp_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkena::LP_ENABLE_MASK)
                | ((value << Clkena::LP_ENABLE_OFFSET) & Clkena::LP_ENABLE_MASK),
        }
    }

    /// Set **CCLK_ENABLE** `[1:0]`
    ///
    /// Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card.
    /// 0: Clock disabled;
    /// 1: Clock enabled.
    #[inline(always)]
    pub const fn with_cclk_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Clkena::CCLK_ENABLE_MASK)
                | ((value << Clkena::CCLK_ENABLE_OFFSET) & Clkena::CCLK_ENABLE_MASK),
        }
    }
}

impl Default for ClkenaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clkena {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkenaVal {
        ClkenaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkenaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkenaVal) -> ClkenaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TMOUT Register
///
/// Data and response timeout configuration register
///
/// Address offset: 0x0014
/// Reset value: 0xffffff40
///
/// ## Fields
///
/// - **DATA_TIMEOUT** `[31:8]` (RW)
///
///   Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
///   NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
///
/// - **RESPONSE_TIMEOUT** `[7:0]` (RW)
///
///   Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
#[repr(transparent)]
pub struct Tmout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tmout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tmout {
    // ======== Field Constants ========

    /// **DATA_TIMEOUT**
    ///
    /// Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
    /// NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
    ///
    /// Bits `[31:8]`, mask `0xffffff00`
    pub const DATA_TIMEOUT_OFFSET: u32 = 8;
    pub const DATA_TIMEOUT_MASK: u32 = 0xffffff00;

    /// **RESPONSE_TIMEOUT**
    ///
    /// Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RESPONSE_TIMEOUT_OFFSET: u32 = 0;
    pub const RESPONSE_TIMEOUT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_TIMEOUT** `[31:8]` (RW)
    ///
    /// Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
    /// NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
    #[inline(always)]
    pub fn data_timeout(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_TIMEOUT_MASK) >> Self::DATA_TIMEOUT_OFFSET
    }

    /// Write **DATA_TIMEOUT** `[31:8]` (RW)
    ///
    /// Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
    /// NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
    #[inline(always)]
    pub fn write_data_timeout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_TIMEOUT_MASK)
                | ((value << Self::DATA_TIMEOUT_OFFSET) & Self::DATA_TIMEOUT_MASK)
        });
    }

    /// **RESPONSE_TIMEOUT** `[7:0]` (RW)
    ///
    /// Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
    #[inline(always)]
    pub fn response_timeout(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE_TIMEOUT_MASK) >> Self::RESPONSE_TIMEOUT_OFFSET
    }

    /// Write **RESPONSE_TIMEOUT** `[7:0]` (RW)
    ///
    /// Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
    #[inline(always)]
    pub fn write_response_timeout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RESPONSE_TIMEOUT_MASK)
                | ((value << Self::RESPONSE_TIMEOUT_OFFSET) & Self::RESPONSE_TIMEOUT_MASK)
        });
    }
}

/// TMOUT value type for batch read/modify/write operations
///
/// Use `tmout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmoutVal {
    bits: u32,
}

impl TmoutVal {
    /// Create from raw bits
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
        Self { bits: 0xffffff40 }
    }

    /// Read **DATA_TIMEOUT** `[31:8]`
    ///
    /// Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
    /// NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
    #[inline(always)]
    pub const fn data_timeout(&self) -> u32 {
        (self.bits & Tmout::DATA_TIMEOUT_MASK) >> Tmout::DATA_TIMEOUT_OFFSET
    }

    /// Read **RESPONSE_TIMEOUT** `[7:0]`
    ///
    /// Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
    #[inline(always)]
    pub const fn response_timeout(&self) -> u32 {
        (self.bits & Tmout::RESPONSE_TIMEOUT_MASK) >> Tmout::RESPONSE_TIMEOUT_OFFSET
    }

    /// Set **DATA_TIMEOUT** `[31:8]`
    ///
    /// Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card.
    /// NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled.
    #[inline(always)]
    pub const fn with_data_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tmout::DATA_TIMEOUT_MASK)
                | ((value << Tmout::DATA_TIMEOUT_OFFSET) & Tmout::DATA_TIMEOUT_MASK),
        }
    }

    /// Set **RESPONSE_TIMEOUT** `[7:0]`
    ///
    /// Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out.
    #[inline(always)]
    pub const fn with_response_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tmout::RESPONSE_TIMEOUT_MASK)
                | ((value << Tmout::RESPONSE_TIMEOUT_OFFSET) & Tmout::RESPONSE_TIMEOUT_MASK),
        }
    }
}

impl Default for TmoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Tmout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmoutVal {
        TmoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmoutVal) -> TmoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTYPE Register
///
/// Card bus width configuration register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CARD_WIDTH8** `[17:16]` (RW)
///
///   One bit per card indicates if card is in 8-bit mode.
///   0: Non 8-bit mode;
///   1: 8-bit mode.
///   Bit[17:16] correspond to card[1:0] respectively.
///
/// - **CARD_WIDTH4** `[1:0]` (RW)
///
///   One bit per card indicates if card is 1-bit or 4-bit mode.
///   0: 1-bit mode;
///   1: 4-bit mode.
///   Bit[1:0] correspond to card[1:0] respectively.
#[repr(transparent)]
pub struct Ctype {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctype {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctype {
    // ======== Field Constants ========

    /// **CARD_WIDTH8**
    ///
    /// One bit per card indicates if card is in 8-bit mode.
    /// 0: Non 8-bit mode;
    /// 1: 8-bit mode.
    /// Bit[17:16] correspond to card[1:0] respectively.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CARD_WIDTH8_OFFSET: u32 = 16;
    pub const CARD_WIDTH8_MASK: u32 = 0x00030000;

    /// **CARD_WIDTH4**
    ///
    /// One bit per card indicates if card is 1-bit or 4-bit mode.
    /// 0: 1-bit mode;
    /// 1: 4-bit mode.
    /// Bit[1:0] correspond to card[1:0] respectively.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CARD_WIDTH4_OFFSET: u32 = 0;
    pub const CARD_WIDTH4_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CARD_WIDTH8** `[17:16]` (RW)
    ///
    /// One bit per card indicates if card is in 8-bit mode.
    /// 0: Non 8-bit mode;
    /// 1: 8-bit mode.
    /// Bit[17:16] correspond to card[1:0] respectively.
    #[inline(always)]
    pub fn card_width8(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_WIDTH8_MASK) >> Self::CARD_WIDTH8_OFFSET
    }

    /// Write **CARD_WIDTH8** `[17:16]` (RW)
    ///
    /// One bit per card indicates if card is in 8-bit mode.
    /// 0: Non 8-bit mode;
    /// 1: 8-bit mode.
    /// Bit[17:16] correspond to card[1:0] respectively.
    #[inline(always)]
    pub fn write_card_width8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARD_WIDTH8_MASK)
                | ((value << Self::CARD_WIDTH8_OFFSET) & Self::CARD_WIDTH8_MASK)
        });
    }

    /// **CARD_WIDTH4** `[1:0]` (RW)
    ///
    /// One bit per card indicates if card is 1-bit or 4-bit mode.
    /// 0: 1-bit mode;
    /// 1: 4-bit mode.
    /// Bit[1:0] correspond to card[1:0] respectively.
    #[inline(always)]
    pub fn card_width4(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_WIDTH4_MASK) >> Self::CARD_WIDTH4_OFFSET
    }

    /// Write **CARD_WIDTH4** `[1:0]` (RW)
    ///
    /// One bit per card indicates if card is 1-bit or 4-bit mode.
    /// 0: 1-bit mode;
    /// 1: 4-bit mode.
    /// Bit[1:0] correspond to card[1:0] respectively.
    #[inline(always)]
    pub fn write_card_width4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARD_WIDTH4_MASK)
                | ((value << Self::CARD_WIDTH4_OFFSET) & Self::CARD_WIDTH4_MASK)
        });
    }
}

/// CTYPE value type for batch read/modify/write operations
///
/// Use `ctype.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctype.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtypeVal {
    bits: u32,
}

impl CtypeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CARD_WIDTH8** `[17:16]`
    ///
    /// One bit per card indicates if card is in 8-bit mode.
    /// 0: Non 8-bit mode;
    /// 1: 8-bit mode.
    /// Bit[17:16] correspond to card[1:0] respectively.
    #[inline(always)]
    pub const fn card_width8(&self) -> u32 {
        (self.bits & Ctype::CARD_WIDTH8_MASK) >> Ctype::CARD_WIDTH8_OFFSET
    }

    /// Read **CARD_WIDTH4** `[1:0]`
    ///
    /// One bit per card indicates if card is 1-bit or 4-bit mode.
    /// 0: 1-bit mode;
    /// 1: 4-bit mode.
    /// Bit[1:0] correspond to card[1:0] respectively.
    #[inline(always)]
    pub const fn card_width4(&self) -> u32 {
        (self.bits & Ctype::CARD_WIDTH4_MASK) >> Ctype::CARD_WIDTH4_OFFSET
    }

    /// Set **CARD_WIDTH8** `[17:16]`
    ///
    /// One bit per card indicates if card is in 8-bit mode.
    /// 0: Non 8-bit mode;
    /// 1: 8-bit mode.
    /// Bit[17:16] correspond to card[1:0] respectively.
    #[inline(always)]
    pub const fn with_card_width8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctype::CARD_WIDTH8_MASK)
                | ((value << Ctype::CARD_WIDTH8_OFFSET) & Ctype::CARD_WIDTH8_MASK),
        }
    }

    /// Set **CARD_WIDTH4** `[1:0]`
    ///
    /// One bit per card indicates if card is 1-bit or 4-bit mode.
    /// 0: 1-bit mode;
    /// 1: 4-bit mode.
    /// Bit[1:0] correspond to card[1:0] respectively.
    #[inline(always)]
    pub const fn with_card_width4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctype::CARD_WIDTH4_MASK)
                | ((value << Ctype::CARD_WIDTH4_OFFSET) & Ctype::CARD_WIDTH4_MASK),
        }
    }
}

impl Default for CtypeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ctype {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtypeVal {
        CtypeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtypeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtypeVal) -> CtypeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLKSIZ Register
///
/// Card data block size configuration register
///
/// Address offset: 0x001c
/// Reset value: 0x00000200
///
/// ## Fields
///
/// - **BLOCK_SIZE** `[15:0]` (RW)
///
///   Block size.
#[repr(transparent)]
pub struct Blksiz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Blksiz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Blksiz {
    // ======== Field Constants ========

    /// **BLOCK_SIZE**
    ///
    /// Block size.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BLOCK_SIZE_OFFSET: u32 = 0;
    pub const BLOCK_SIZE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BLOCK_SIZE** `[15:0]` (RW)
    ///
    /// Block size.
    #[inline(always)]
    pub fn block_size(&self) -> u32 {
        (self.value.read_bits() & Self::BLOCK_SIZE_MASK) >> Self::BLOCK_SIZE_OFFSET
    }

    /// Write **BLOCK_SIZE** `[15:0]` (RW)
    ///
    /// Block size.
    #[inline(always)]
    pub fn write_block_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLOCK_SIZE_MASK)
                | ((value << Self::BLOCK_SIZE_OFFSET) & Self::BLOCK_SIZE_MASK)
        });
    }
}

/// BLKSIZ value type for batch read/modify/write operations
///
/// Use `blksiz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blksiz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlksizVal {
    bits: u32,
}

impl BlksizVal {
    /// Create from raw bits
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

    /// Read **BLOCK_SIZE** `[15:0]`
    ///
    /// Block size.
    #[inline(always)]
    pub const fn block_size(&self) -> u32 {
        (self.bits & Blksiz::BLOCK_SIZE_MASK) >> Blksiz::BLOCK_SIZE_OFFSET
    }

    /// Set **BLOCK_SIZE** `[15:0]`
    ///
    /// Block size.
    #[inline(always)]
    pub const fn with_block_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Blksiz::BLOCK_SIZE_MASK)
                | ((value << Blksiz::BLOCK_SIZE_OFFSET) & Blksiz::BLOCK_SIZE_MASK),
        }
    }
}

impl Default for BlksizVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Blksiz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlksizVal {
        BlksizVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlksizVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlksizVal) -> BlksizVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BYTCNT Register
///
/// Data transfer length configuration register
///
/// Address offset: 0x0020
/// Reset value: 0x00000200
///
/// ## Fields
///
/// - **BYTE_COUNT** `[31:0]` (RW)
///
///   Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
#[repr(transparent)]
pub struct Bytcnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bytcnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bytcnt {
    // ======== Field Constants ========

    /// **BYTE_COUNT**
    ///
    /// Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BYTE_COUNT_OFFSET: u32 = 0;
    pub const BYTE_COUNT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BYTE_COUNT** `[31:0]` (RW)
    ///
    /// Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
    #[inline(always)]
    pub fn byte_count(&self) -> u32 {
        (self.value.read_bits() & Self::BYTE_COUNT_MASK) >> Self::BYTE_COUNT_OFFSET
    }

    /// Write **BYTE_COUNT** `[31:0]` (RW)
    ///
    /// Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
    #[inline(always)]
    pub fn write_byte_count(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BYTE_COUNT_MASK)
                | ((value << Self::BYTE_COUNT_OFFSET) & Self::BYTE_COUNT_MASK)
        });
    }
}

/// BYTCNT value type for batch read/modify/write operations
///
/// Use `bytcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bytcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BytcntVal {
    bits: u32,
}

impl BytcntVal {
    /// Create from raw bits
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

    /// Read **BYTE_COUNT** `[31:0]`
    ///
    /// Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
    #[inline(always)]
    pub const fn byte_count(&self) -> u32 {
        (self.bits & Bytcnt::BYTE_COUNT_MASK) >> Bytcnt::BYTE_COUNT_OFFSET
    }

    /// Set **BYTE_COUNT** `[31:0]`
    ///
    /// Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer.
    #[inline(always)]
    pub const fn with_byte_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bytcnt::BYTE_COUNT_MASK)
                | ((value << Bytcnt::BYTE_COUNT_OFFSET) & Bytcnt::BYTE_COUNT_MASK),
        }
    }
}

impl Default for BytcntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bytcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BytcntVal {
        BytcntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BytcntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BytcntVal) -> BytcntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTMASK Register
///
/// SDIO interrupt mask register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SDIO_INT_MASK** `[17:16]` (RW)
///
///   SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
///
/// - **INT_MASK** `[15:0]` (RW)
///
///   These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
///   Bit 15 (EBE): End-bit error/no CRC error;
///   Bit 14 (ACD): Auto command done;
///   Bit 13 (SBE/BCI): Rx Start Bit Error;
///   Bit 12 (HLE): Hardware locked write error;
///   Bit 11 (FRUN): FIFO underrun/overrun error;
///   Bit 10 (HTO): Data starvation-by-host timeout;
///   Bit 9 (DRTO): Data read timeout;
///   Bit 8 (RTO): Response timeout;
///   Bit 7 (DCRC): Data CRC error;
///   Bit 6 (RCRC): Response CRC error;
///   Bit 5 (RXDR): Receive FIFO data request;
///   Bit 4 (TXDR): Transmit FIFO data request;
///   Bit 3 (DTO): Data transfer over;
///   Bit 2 (CD): Command done;
///   Bit 1 (RE): Response error;
///   Bit 0 (CD): Card detect.
#[repr(transparent)]
pub struct Intmask {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intmask {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intmask {
    // ======== Field Constants ========

    /// **SDIO_INT_MASK**
    ///
    /// SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SDIO_INT_MASK_OFFSET: u32 = 16;
    pub const SDIO_INT_MASK_MASK: u32 = 0x00030000;

    /// **INT_MASK**
    ///
    /// These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): Rx Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation-by-host timeout;
    /// Bit 9 (DRTO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INT_MASK_OFFSET: u32 = 0;
    pub const INT_MASK_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SDIO_INT_MASK** `[17:16]` (RW)
    ///
    /// SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> u32 {
        (self.value.read_bits() & Self::SDIO_INT_MASK_MASK) >> Self::SDIO_INT_MASK_OFFSET
    }

    /// Write **SDIO_INT_MASK** `[17:16]` (RW)
    ///
    /// SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
    #[inline(always)]
    pub fn write_sdio_int_mask(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SDIO_INT_MASK_MASK)
                | ((value << Self::SDIO_INT_MASK_OFFSET) & Self::SDIO_INT_MASK_MASK)
        });
    }

    /// **INT_MASK** `[15:0]` (RW)
    ///
    /// These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): Rx Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation-by-host timeout;
    /// Bit 9 (DRTO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn int_mask(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK_MASK) >> Self::INT_MASK_OFFSET
    }

    /// Write **INT_MASK** `[15:0]` (RW)
    ///
    /// These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): Rx Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation-by-host timeout;
    /// Bit 9 (DRTO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn write_int_mask(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INT_MASK_MASK) | ((value << Self::INT_MASK_OFFSET) & Self::INT_MASK_MASK)
        });
    }
}

/// INTMASK value type for batch read/modify/write operations
///
/// Use `intmask.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmask.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntmaskVal {
    bits: u32,
}

impl IntmaskVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SDIO_INT_MASK** `[17:16]`
    ///
    /// SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
    #[inline(always)]
    pub const fn sdio_int_mask(&self) -> u32 {
        (self.bits & Intmask::SDIO_INT_MASK_MASK) >> Intmask::SDIO_INT_MASK_OFFSET
    }

    /// Read **INT_MASK** `[15:0]`
    ///
    /// These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): Rx Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation-by-host timeout;
    /// Bit 9 (DRTO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub const fn int_mask(&self) -> u32 {
        (self.bits & Intmask::INT_MASK_MASK) >> Intmask::INT_MASK_OFFSET
    }

    /// Set **SDIO_INT_MASK** `[17:16]`
    ///
    /// SDIO interrupt mask, one bit for each card. Bit[17:16] correspond to card[15:0] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt.
    #[inline(always)]
    pub const fn with_sdio_int_mask(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Intmask::SDIO_INT_MASK_MASK)
                | ((value << Intmask::SDIO_INT_MASK_OFFSET) & Intmask::SDIO_INT_MASK_MASK),
        }
    }

    /// Set **INT_MASK** `[15:0]`
    ///
    /// These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): Rx Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation-by-host timeout;
    /// Bit 9 (DRTO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub const fn with_int_mask(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Intmask::INT_MASK_MASK)
                | ((value << Intmask::INT_MASK_OFFSET) & Intmask::INT_MASK_MASK),
        }
    }
}

impl Default for IntmaskVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Intmask {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntmaskVal {
        IntmaskVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntmaskVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntmaskVal) -> IntmaskVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMDARG Register
///
/// Command argument data register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **CMDARG** `[31:0]` (RW)
///
///   Value indicates command argument to be passed to the card.
#[repr(transparent)]
pub struct Cmdarg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cmdarg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cmdarg {
    // ======== Field Constants ========

    /// **CMDARG**
    ///
    /// Value indicates command argument to be passed to the card.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CMDARG_OFFSET: u32 = 0;
    pub const CMDARG_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CMDARG** `[31:0]` (RW)
    ///
    /// Value indicates command argument to be passed to the card.
    #[inline(always)]
    pub fn cmdarg(&self) -> u32 {
        (self.value.read_bits() & Self::CMDARG_MASK) >> Self::CMDARG_OFFSET
    }

    /// Write **CMDARG** `[31:0]` (RW)
    ///
    /// Value indicates command argument to be passed to the card.
    #[inline(always)]
    pub fn write_cmdarg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CMDARG_MASK) | ((value << Self::CMDARG_OFFSET) & Self::CMDARG_MASK)
        });
    }
}

/// CMDARG value type for batch read/modify/write operations
///
/// Use `cmdarg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmdarg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdargVal {
    bits: u32,
}

impl CmdargVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CMDARG** `[31:0]`
    ///
    /// Value indicates command argument to be passed to the card.
    #[inline(always)]
    pub const fn cmdarg(&self) -> u32 {
        (self.bits & Cmdarg::CMDARG_MASK) >> Cmdarg::CMDARG_OFFSET
    }

    /// Set **CMDARG** `[31:0]`
    ///
    /// Value indicates command argument to be passed to the card.
    #[inline(always)]
    pub const fn with_cmdarg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmdarg::CMDARG_MASK)
                | ((value << Cmdarg::CMDARG_OFFSET) & Cmdarg::CMDARG_MASK),
        }
    }
}

impl Default for CmdargVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cmdarg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdargVal {
        CmdargVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdargVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmdargVal) -> CmdargVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMD Register
///
/// Command and boot configuration register
///
/// Address offset: 0x002c
/// Reset value: 0x20000000
///
/// ## Fields
///
/// - **START_CMD** `[31]` (RW)
///
///   Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register.
///
/// - **USE_HOLE** `[29]` (RW)
///
///   Use Hold Register.
///   0: CMD and DATA sent to card bypassing HOLD Register;
///   1: CMD and DATA sent to card through the HOLD Register.
///
/// - **CCS_EXPECTED** `[23]` (RW)
///
///   Expected Command Completion Signal (CCS) configuration.
///   0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device;
///   1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device.
///   If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked.
///
/// - **READ_CEATA_DEVICE** `[22]` (RW)
///
///   Read access flag.
///   0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device;
///   1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
///   Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device.
///
/// - **UPDATE_CLOCK_REGISTERS_ONLY** `[21]` (RW)
///
///   0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain.
///   Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA.
///   Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards.
///
/// - **CARD_NUMBER** `[20:16]` (RW)
///
///   Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
///
/// - **SEND_INITIALIZATION** `[15]` (RW)
///
///   0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command.
///   After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card.
///
/// - **STOP_ABORT_CMD** `[14]` (RW)
///
///   0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress.
///   When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state.
///
/// - **WAIT_PRVDATA_COMPLETE** `[13]` (RW)
///
///   0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command.
///   The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command.
///
/// - **SEND_AUTO_STOP** `[12]` (RW)
///
///   0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer.
///
/// - **TRANSFER_MODE** `[11]` (RW)
///
///   0: Block data transfer command; 1: Stream data transfer command.
///   Don't care if no data expected.
///
/// - **READ_WRITE** `[10]` (RW)
///
///   0: Read from card; 1: Write to card.
///   Don't care if no data is expected from card.
///
/// - **DATA_EXPECTED** `[9]` (RW)
///
///   0: No data transfer expected; 1: Data transfer expected.
///
/// - **CHECK_RESPONSE_CRC** `[8]` (RW)
///
///   0: Do not check; 1: Check response CRC.
///   Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller.
///
/// - **RESPONSE_LENGTH** `[7]` (RW)
///
///   0: Short response expected from card; 1: Long response expected from card.
///
/// - **RESPONSE_EXPECT** `[6]` (RW)
///
///   0: No response expected from card; 1: Response expected from card.
///
/// - **INDEX** `[5:0]` (RW)
///
///   Command index.
#[repr(transparent)]
pub struct Cmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cmd {
    // ======== Field Constants ========

    /// **START_CMD**
    ///
    /// Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const START_CMD_OFFSET: u32 = 31;
    pub const START_CMD_MASK: u32 = 0x80000000;
    pub const START_CMD: u32 = 1 << 31;

    /// **USE_HOLE**
    ///
    /// Use Hold Register.
    /// 0: CMD and DATA sent to card bypassing HOLD Register;
    /// 1: CMD and DATA sent to card through the HOLD Register.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const USE_HOLE_OFFSET: u32 = 29;
    pub const USE_HOLE_MASK: u32 = 0x20000000;
    pub const USE_HOLE: u32 = 1 << 29;

    /// **CCS_EXPECTED**
    ///
    /// Expected Command Completion Signal (CCS) configuration.
    /// 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device;
    /// 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device.
    /// If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CCS_EXPECTED_OFFSET: u32 = 23;
    pub const CCS_EXPECTED_MASK: u32 = 0x00800000;
    pub const CCS_EXPECTED: u32 = 1 << 23;

    /// **READ_CEATA_DEVICE**
    ///
    /// Read access flag.
    /// 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device;
    /// 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
    /// Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const READ_CEATA_DEVICE_OFFSET: u32 = 22;
    pub const READ_CEATA_DEVICE_MASK: u32 = 0x00400000;
    pub const READ_CEATA_DEVICE: u32 = 1 << 22;

    /// **UPDATE_CLOCK_REGISTERS_ONLY**
    ///
    /// 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain.
    /// Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA.
    /// Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const UPDATE_CLOCK_REGISTERS_ONLY_OFFSET: u32 = 21;
    pub const UPDATE_CLOCK_REGISTERS_ONLY_MASK: u32 = 0x00200000;
    pub const UPDATE_CLOCK_REGISTERS_ONLY: u32 = 1 << 21;

    /// **CARD_NUMBER**
    ///
    /// Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
    ///
    /// Bits `[20:16]`, mask `0x001f0000`
    pub const CARD_NUMBER_OFFSET: u32 = 16;
    pub const CARD_NUMBER_MASK: u32 = 0x001f0000;

    /// **SEND_INITIALIZATION**
    ///
    /// 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command.
    /// After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SEND_INITIALIZATION_OFFSET: u32 = 15;
    pub const SEND_INITIALIZATION_MASK: u32 = 0x00008000;
    pub const SEND_INITIALIZATION: u32 = 1 << 15;

    /// **STOP_ABORT_CMD**
    ///
    /// 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress.
    /// When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const STOP_ABORT_CMD_OFFSET: u32 = 14;
    pub const STOP_ABORT_CMD_MASK: u32 = 0x00004000;
    pub const STOP_ABORT_CMD: u32 = 1 << 14;

    /// **WAIT_PRVDATA_COMPLETE**
    ///
    /// 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command.
    /// The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const WAIT_PRVDATA_COMPLETE_OFFSET: u32 = 13;
    pub const WAIT_PRVDATA_COMPLETE_MASK: u32 = 0x00002000;
    pub const WAIT_PRVDATA_COMPLETE: u32 = 1 << 13;

    /// **SEND_AUTO_STOP**
    ///
    /// 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SEND_AUTO_STOP_OFFSET: u32 = 12;
    pub const SEND_AUTO_STOP_MASK: u32 = 0x00001000;
    pub const SEND_AUTO_STOP: u32 = 1 << 12;

    /// **TRANSFER_MODE**
    ///
    /// 0: Block data transfer command; 1: Stream data transfer command.
    /// Don't care if no data expected.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TRANSFER_MODE_OFFSET: u32 = 11;
    pub const TRANSFER_MODE_MASK: u32 = 0x00000800;
    pub const TRANSFER_MODE: u32 = 1 << 11;

    /// **READ_WRITE**
    ///
    /// 0: Read from card; 1: Write to card.
    /// Don't care if no data is expected from card.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const READ_WRITE_OFFSET: u32 = 10;
    pub const READ_WRITE_MASK: u32 = 0x00000400;
    pub const READ_WRITE: u32 = 1 << 10;

    /// **DATA_EXPECTED**
    ///
    /// 0: No data transfer expected; 1: Data transfer expected.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DATA_EXPECTED_OFFSET: u32 = 9;
    pub const DATA_EXPECTED_MASK: u32 = 0x00000200;
    pub const DATA_EXPECTED: u32 = 1 << 9;

    /// **CHECK_RESPONSE_CRC**
    ///
    /// 0: Do not check; 1: Check response CRC.
    /// Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CHECK_RESPONSE_CRC_OFFSET: u32 = 8;
    pub const CHECK_RESPONSE_CRC_MASK: u32 = 0x00000100;
    pub const CHECK_RESPONSE_CRC: u32 = 1 << 8;

    /// **RESPONSE_LENGTH**
    ///
    /// 0: Short response expected from card; 1: Long response expected from card.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RESPONSE_LENGTH_OFFSET: u32 = 7;
    pub const RESPONSE_LENGTH_MASK: u32 = 0x00000080;
    pub const RESPONSE_LENGTH: u32 = 1 << 7;

    /// **RESPONSE_EXPECT**
    ///
    /// 0: No response expected from card; 1: Response expected from card.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RESPONSE_EXPECT_OFFSET: u32 = 6;
    pub const RESPONSE_EXPECT_MASK: u32 = 0x00000040;
    pub const RESPONSE_EXPECT: u32 = 1 << 6;

    /// **INDEX**
    ///
    /// Command index.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const INDEX_OFFSET: u32 = 0;
    pub const INDEX_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **START_CMD** `[31]` (RW)
    ///
    /// Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register.
    #[inline(always)]
    pub fn start_cmd(&self) -> bool {
        (self.value.read_bits() & Self::START_CMD_MASK) != 0
    }

    /// Set START_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start_cmd(&self) {
        self.value.modify_bits(|r| r | Self::START_CMD_MASK);
    }

    /// Clear START_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start_cmd(&self) {
        self.value.modify_bits(|r| r & !Self::START_CMD_MASK);
    }

    /// **USE_HOLE** `[29]` (RW)
    ///
    /// Use Hold Register.
    /// 0: CMD and DATA sent to card bypassing HOLD Register;
    /// 1: CMD and DATA sent to card through the HOLD Register.
    #[inline(always)]
    pub fn use_hole(&self) -> bool {
        (self.value.read_bits() & Self::USE_HOLE_MASK) != 0
    }

    /// Set USE_HOLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_use_hole(&self) {
        self.value.modify_bits(|r| r | Self::USE_HOLE_MASK);
    }

    /// Clear USE_HOLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_use_hole(&self) {
        self.value.modify_bits(|r| r & !Self::USE_HOLE_MASK);
    }

    /// **CCS_EXPECTED** `[23]` (RW)
    ///
    /// Expected Command Completion Signal (CCS) configuration.
    /// 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device;
    /// 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device.
    /// If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked.
    #[inline(always)]
    pub fn ccs_expected(&self) -> bool {
        (self.value.read_bits() & Self::CCS_EXPECTED_MASK) != 0
    }

    /// Set CCS_EXPECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ccs_expected(&self) {
        self.value.modify_bits(|r| r | Self::CCS_EXPECTED_MASK);
    }

    /// Clear CCS_EXPECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ccs_expected(&self) {
        self.value.modify_bits(|r| r & !Self::CCS_EXPECTED_MASK);
    }

    /// **READ_CEATA_DEVICE** `[22]` (RW)
    ///
    /// Read access flag.
    /// 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device;
    /// 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
    /// Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device.
    #[inline(always)]
    pub fn read_ceata_device(&self) -> bool {
        (self.value.read_bits() & Self::READ_CEATA_DEVICE_MASK) != 0
    }

    /// Set READ_CEATA_DEVICE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_read_ceata_device(&self) {
        self.value.modify_bits(|r| r | Self::READ_CEATA_DEVICE_MASK);
    }

    /// Clear READ_CEATA_DEVICE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_read_ceata_device(&self) {
        self.value
            .modify_bits(|r| r & !Self::READ_CEATA_DEVICE_MASK);
    }

    /// **UPDATE_CLOCK_REGISTERS_ONLY** `[21]` (RW)
    ///
    /// 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain.
    /// Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA.
    /// Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards.
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> bool {
        (self.value.read_bits() & Self::UPDATE_CLOCK_REGISTERS_ONLY_MASK) != 0
    }

    /// Set UPDATE_CLOCK_REGISTERS_ONLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_update_clock_registers_only(&self) {
        self.value
            .modify_bits(|r| r | Self::UPDATE_CLOCK_REGISTERS_ONLY_MASK);
    }

    /// Clear UPDATE_CLOCK_REGISTERS_ONLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_update_clock_registers_only(&self) {
        self.value
            .modify_bits(|r| r & !Self::UPDATE_CLOCK_REGISTERS_ONLY_MASK);
    }

    /// **CARD_NUMBER** `[20:16]` (RW)
    ///
    /// Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
    #[inline(always)]
    pub fn card_number(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_NUMBER_MASK) >> Self::CARD_NUMBER_OFFSET
    }

    /// Write **CARD_NUMBER** `[20:16]` (RW)
    ///
    /// Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
    #[inline(always)]
    pub fn write_card_number(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARD_NUMBER_MASK)
                | ((value << Self::CARD_NUMBER_OFFSET) & Self::CARD_NUMBER_MASK)
        });
    }

    /// **SEND_INITIALIZATION** `[15]` (RW)
    ///
    /// 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command.
    /// After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card.
    #[inline(always)]
    pub fn send_initialization(&self) -> bool {
        (self.value.read_bits() & Self::SEND_INITIALIZATION_MASK) != 0
    }

    /// Set SEND_INITIALIZATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_initialization(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_INITIALIZATION_MASK);
    }

    /// Clear SEND_INITIALIZATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_initialization(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_INITIALIZATION_MASK);
    }

    /// **STOP_ABORT_CMD** `[14]` (RW)
    ///
    /// 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress.
    /// When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state.
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> bool {
        (self.value.read_bits() & Self::STOP_ABORT_CMD_MASK) != 0
    }

    /// Set STOP_ABORT_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stop_abort_cmd(&self) {
        self.value.modify_bits(|r| r | Self::STOP_ABORT_CMD_MASK);
    }

    /// Clear STOP_ABORT_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stop_abort_cmd(&self) {
        self.value.modify_bits(|r| r & !Self::STOP_ABORT_CMD_MASK);
    }

    /// **WAIT_PRVDATA_COMPLETE** `[13]` (RW)
    ///
    /// 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command.
    /// The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command.
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> bool {
        (self.value.read_bits() & Self::WAIT_PRVDATA_COMPLETE_MASK) != 0
    }

    /// Set WAIT_PRVDATA_COMPLETE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wait_prvdata_complete(&self) {
        self.value
            .modify_bits(|r| r | Self::WAIT_PRVDATA_COMPLETE_MASK);
    }

    /// Clear WAIT_PRVDATA_COMPLETE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wait_prvdata_complete(&self) {
        self.value
            .modify_bits(|r| r & !Self::WAIT_PRVDATA_COMPLETE_MASK);
    }

    /// **SEND_AUTO_STOP** `[12]` (RW)
    ///
    /// 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer.
    #[inline(always)]
    pub fn send_auto_stop(&self) -> bool {
        (self.value.read_bits() & Self::SEND_AUTO_STOP_MASK) != 0
    }

    /// Set SEND_AUTO_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_auto_stop(&self) {
        self.value.modify_bits(|r| r | Self::SEND_AUTO_STOP_MASK);
    }

    /// Clear SEND_AUTO_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_auto_stop(&self) {
        self.value.modify_bits(|r| r & !Self::SEND_AUTO_STOP_MASK);
    }

    /// **TRANSFER_MODE** `[11]` (RW)
    ///
    /// 0: Block data transfer command; 1: Stream data transfer command.
    /// Don't care if no data expected.
    #[inline(always)]
    pub fn transfer_mode(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_MODE_MASK) != 0
    }

    /// Set TRANSFER_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_transfer_mode(&self) {
        self.value.modify_bits(|r| r | Self::TRANSFER_MODE_MASK);
    }

    /// Clear TRANSFER_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_transfer_mode(&self) {
        self.value.modify_bits(|r| r & !Self::TRANSFER_MODE_MASK);
    }

    /// **READ_WRITE** `[10]` (RW)
    ///
    /// 0: Read from card; 1: Write to card.
    /// Don't care if no data is expected from card.
    #[inline(always)]
    pub fn read_write(&self) -> bool {
        (self.value.read_bits() & Self::READ_WRITE_MASK) != 0
    }

    /// Set READ_WRITE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_read_write(&self) {
        self.value.modify_bits(|r| r | Self::READ_WRITE_MASK);
    }

    /// Clear READ_WRITE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_read_write(&self) {
        self.value.modify_bits(|r| r & !Self::READ_WRITE_MASK);
    }

    /// **DATA_EXPECTED** `[9]` (RW)
    ///
    /// 0: No data transfer expected; 1: Data transfer expected.
    #[inline(always)]
    pub fn data_expected(&self) -> bool {
        (self.value.read_bits() & Self::DATA_EXPECTED_MASK) != 0
    }

    /// Set DATA_EXPECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_data_expected(&self) {
        self.value.modify_bits(|r| r | Self::DATA_EXPECTED_MASK);
    }

    /// Clear DATA_EXPECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_data_expected(&self) {
        self.value.modify_bits(|r| r & !Self::DATA_EXPECTED_MASK);
    }

    /// **CHECK_RESPONSE_CRC** `[8]` (RW)
    ///
    /// 0: Do not check; 1: Check response CRC.
    /// Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller.
    #[inline(always)]
    pub fn check_response_crc(&self) -> bool {
        (self.value.read_bits() & Self::CHECK_RESPONSE_CRC_MASK) != 0
    }

    /// Set CHECK_RESPONSE_CRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_check_response_crc(&self) {
        self.value
            .modify_bits(|r| r | Self::CHECK_RESPONSE_CRC_MASK);
    }

    /// Clear CHECK_RESPONSE_CRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_check_response_crc(&self) {
        self.value
            .modify_bits(|r| r & !Self::CHECK_RESPONSE_CRC_MASK);
    }

    /// **RESPONSE_LENGTH** `[7]` (RW)
    ///
    /// 0: Short response expected from card; 1: Long response expected from card.
    #[inline(always)]
    pub fn response_length(&self) -> bool {
        (self.value.read_bits() & Self::RESPONSE_LENGTH_MASK) != 0
    }

    /// Set RESPONSE_LENGTH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_response_length(&self) {
        self.value.modify_bits(|r| r | Self::RESPONSE_LENGTH_MASK);
    }

    /// Clear RESPONSE_LENGTH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_response_length(&self) {
        self.value.modify_bits(|r| r & !Self::RESPONSE_LENGTH_MASK);
    }

    /// **RESPONSE_EXPECT** `[6]` (RW)
    ///
    /// 0: No response expected from card; 1: Response expected from card.
    #[inline(always)]
    pub fn response_expect(&self) -> bool {
        (self.value.read_bits() & Self::RESPONSE_EXPECT_MASK) != 0
    }

    /// Set RESPONSE_EXPECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_response_expect(&self) {
        self.value.modify_bits(|r| r | Self::RESPONSE_EXPECT_MASK);
    }

    /// Clear RESPONSE_EXPECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_response_expect(&self) {
        self.value.modify_bits(|r| r & !Self::RESPONSE_EXPECT_MASK);
    }

    /// **INDEX** `[5:0]` (RW)
    ///
    /// Command index.
    #[inline(always)]
    pub fn index(&self) -> u32 {
        (self.value.read_bits() & Self::INDEX_MASK) >> Self::INDEX_OFFSET
    }

    /// Write **INDEX** `[5:0]` (RW)
    ///
    /// Command index.
    #[inline(always)]
    pub fn write_index(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INDEX_MASK) | ((value << Self::INDEX_OFFSET) & Self::INDEX_MASK)
        });
    }
}

/// CMD value type for batch read/modify/write operations
///
/// Use `cmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdVal {
    bits: u32,
}

impl CmdVal {
    /// Create from raw bits
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
        Self { bits: 0x20000000 }
    }

    /// Read **START_CMD** `[31:31]`
    ///
    /// Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register.
    #[inline(always)]
    pub const fn start_cmd(&self) -> bool {
        (self.bits & Cmd::START_CMD_MASK) != 0
    }

    /// Read **USE_HOLE** `[29:29]`
    ///
    /// Use Hold Register.
    /// 0: CMD and DATA sent to card bypassing HOLD Register;
    /// 1: CMD and DATA sent to card through the HOLD Register.
    #[inline(always)]
    pub const fn use_hole(&self) -> bool {
        (self.bits & Cmd::USE_HOLE_MASK) != 0
    }

    /// Read **CCS_EXPECTED** `[23:23]`
    ///
    /// Expected Command Completion Signal (CCS) configuration.
    /// 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device;
    /// 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device.
    /// If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked.
    #[inline(always)]
    pub const fn ccs_expected(&self) -> bool {
        (self.bits & Cmd::CCS_EXPECTED_MASK) != 0
    }

    /// Read **READ_CEATA_DEVICE** `[22:22]`
    ///
    /// Read access flag.
    /// 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device;
    /// 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
    /// Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device.
    #[inline(always)]
    pub const fn read_ceata_device(&self) -> bool {
        (self.bits & Cmd::READ_CEATA_DEVICE_MASK) != 0
    }

    /// Read **UPDATE_CLOCK_REGISTERS_ONLY** `[21:21]`
    ///
    /// 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain.
    /// Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA.
    /// Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards.
    #[inline(always)]
    pub const fn update_clock_registers_only(&self) -> bool {
        (self.bits & Cmd::UPDATE_CLOCK_REGISTERS_ONLY_MASK) != 0
    }

    /// Read **CARD_NUMBER** `[20:16]`
    ///
    /// Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
    #[inline(always)]
    pub const fn card_number(&self) -> u32 {
        (self.bits & Cmd::CARD_NUMBER_MASK) >> Cmd::CARD_NUMBER_OFFSET
    }

    /// Read **SEND_INITIALIZATION** `[15:15]`
    ///
    /// 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command.
    /// After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card.
    #[inline(always)]
    pub const fn send_initialization(&self) -> bool {
        (self.bits & Cmd::SEND_INITIALIZATION_MASK) != 0
    }

    /// Read **STOP_ABORT_CMD** `[14:14]`
    ///
    /// 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress.
    /// When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state.
    #[inline(always)]
    pub const fn stop_abort_cmd(&self) -> bool {
        (self.bits & Cmd::STOP_ABORT_CMD_MASK) != 0
    }

    /// Read **WAIT_PRVDATA_COMPLETE** `[13:13]`
    ///
    /// 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command.
    /// The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command.
    #[inline(always)]
    pub const fn wait_prvdata_complete(&self) -> bool {
        (self.bits & Cmd::WAIT_PRVDATA_COMPLETE_MASK) != 0
    }

    /// Read **SEND_AUTO_STOP** `[12:12]`
    ///
    /// 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer.
    #[inline(always)]
    pub const fn send_auto_stop(&self) -> bool {
        (self.bits & Cmd::SEND_AUTO_STOP_MASK) != 0
    }

    /// Read **TRANSFER_MODE** `[11:11]`
    ///
    /// 0: Block data transfer command; 1: Stream data transfer command.
    /// Don't care if no data expected.
    #[inline(always)]
    pub const fn transfer_mode(&self) -> bool {
        (self.bits & Cmd::TRANSFER_MODE_MASK) != 0
    }

    /// Read **READ_WRITE** `[10:10]`
    ///
    /// 0: Read from card; 1: Write to card.
    /// Don't care if no data is expected from card.
    #[inline(always)]
    pub const fn read_write(&self) -> bool {
        (self.bits & Cmd::READ_WRITE_MASK) != 0
    }

    /// Read **DATA_EXPECTED** `[9:9]`
    ///
    /// 0: No data transfer expected; 1: Data transfer expected.
    #[inline(always)]
    pub const fn data_expected(&self) -> bool {
        (self.bits & Cmd::DATA_EXPECTED_MASK) != 0
    }

    /// Read **CHECK_RESPONSE_CRC** `[8:8]`
    ///
    /// 0: Do not check; 1: Check response CRC.
    /// Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller.
    #[inline(always)]
    pub const fn check_response_crc(&self) -> bool {
        (self.bits & Cmd::CHECK_RESPONSE_CRC_MASK) != 0
    }

    /// Read **RESPONSE_LENGTH** `[7:7]`
    ///
    /// 0: Short response expected from card; 1: Long response expected from card.
    #[inline(always)]
    pub const fn response_length(&self) -> bool {
        (self.bits & Cmd::RESPONSE_LENGTH_MASK) != 0
    }

    /// Read **RESPONSE_EXPECT** `[6:6]`
    ///
    /// 0: No response expected from card; 1: Response expected from card.
    #[inline(always)]
    pub const fn response_expect(&self) -> bool {
        (self.bits & Cmd::RESPONSE_EXPECT_MASK) != 0
    }

    /// Read **INDEX** `[5:0]`
    ///
    /// Command index.
    #[inline(always)]
    pub const fn index(&self) -> u32 {
        (self.bits & Cmd::INDEX_MASK) >> Cmd::INDEX_OFFSET
    }

    /// Set **START_CMD** `[31:31]`
    ///
    /// Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register.
    #[inline(always)]
    pub const fn with_start_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::START_CMD_MASK
            } else {
                self.bits & !Cmd::START_CMD_MASK
            },
        }
    }

    /// Set **USE_HOLE** `[29:29]`
    ///
    /// Use Hold Register.
    /// 0: CMD and DATA sent to card bypassing HOLD Register;
    /// 1: CMD and DATA sent to card through the HOLD Register.
    #[inline(always)]
    pub const fn with_use_hole(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::USE_HOLE_MASK
            } else {
                self.bits & !Cmd::USE_HOLE_MASK
            },
        }
    }

    /// Set **CCS_EXPECTED** `[23:23]`
    ///
    /// Expected Command Completion Signal (CCS) configuration.
    /// 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device;
    /// 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device.
    /// If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked.
    #[inline(always)]
    pub const fn with_ccs_expected(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::CCS_EXPECTED_MASK
            } else {
                self.bits & !Cmd::CCS_EXPECTED_MASK
            },
        }
    }

    /// Set **READ_CEATA_DEVICE** `[22:22]`
    ///
    /// Read access flag.
    /// 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device;
    /// 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
    /// Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device.
    #[inline(always)]
    pub const fn with_read_ceata_device(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::READ_CEATA_DEVICE_MASK
            } else {
                self.bits & !Cmd::READ_CEATA_DEVICE_MASK
            },
        }
    }

    /// Set **UPDATE_CLOCK_REGISTERS_ONLY** `[21:21]`
    ///
    /// 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain.
    /// Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA.
    /// Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards.
    #[inline(always)]
    pub const fn with_update_clock_registers_only(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::UPDATE_CLOCK_REGISTERS_ONLY_MASK
            } else {
                self.bits & !Cmd::UPDATE_CLOCK_REGISTERS_ONLY_MASK
            },
        }
    }

    /// Set **CARD_NUMBER** `[20:16]`
    ///
    /// Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported.
    #[inline(always)]
    pub const fn with_card_number(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmd::CARD_NUMBER_MASK)
                | ((value << Cmd::CARD_NUMBER_OFFSET) & Cmd::CARD_NUMBER_MASK),
        }
    }

    /// Set **SEND_INITIALIZATION** `[15:15]`
    ///
    /// 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command.
    /// After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card.
    #[inline(always)]
    pub const fn with_send_initialization(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::SEND_INITIALIZATION_MASK
            } else {
                self.bits & !Cmd::SEND_INITIALIZATION_MASK
            },
        }
    }

    /// Set **STOP_ABORT_CMD** `[14:14]`
    ///
    /// 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress.
    /// When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state.
    #[inline(always)]
    pub const fn with_stop_abort_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::STOP_ABORT_CMD_MASK
            } else {
                self.bits & !Cmd::STOP_ABORT_CMD_MASK
            },
        }
    }

    /// Set **WAIT_PRVDATA_COMPLETE** `[13:13]`
    ///
    /// 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command.
    /// The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command.
    #[inline(always)]
    pub const fn with_wait_prvdata_complete(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::WAIT_PRVDATA_COMPLETE_MASK
            } else {
                self.bits & !Cmd::WAIT_PRVDATA_COMPLETE_MASK
            },
        }
    }

    /// Set **SEND_AUTO_STOP** `[12:12]`
    ///
    /// 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer.
    #[inline(always)]
    pub const fn with_send_auto_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::SEND_AUTO_STOP_MASK
            } else {
                self.bits & !Cmd::SEND_AUTO_STOP_MASK
            },
        }
    }

    /// Set **TRANSFER_MODE** `[11:11]`
    ///
    /// 0: Block data transfer command; 1: Stream data transfer command.
    /// Don't care if no data expected.
    #[inline(always)]
    pub const fn with_transfer_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::TRANSFER_MODE_MASK
            } else {
                self.bits & !Cmd::TRANSFER_MODE_MASK
            },
        }
    }

    /// Set **READ_WRITE** `[10:10]`
    ///
    /// 0: Read from card; 1: Write to card.
    /// Don't care if no data is expected from card.
    #[inline(always)]
    pub const fn with_read_write(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::READ_WRITE_MASK
            } else {
                self.bits & !Cmd::READ_WRITE_MASK
            },
        }
    }

    /// Set **DATA_EXPECTED** `[9:9]`
    ///
    /// 0: No data transfer expected; 1: Data transfer expected.
    #[inline(always)]
    pub const fn with_data_expected(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::DATA_EXPECTED_MASK
            } else {
                self.bits & !Cmd::DATA_EXPECTED_MASK
            },
        }
    }

    /// Set **CHECK_RESPONSE_CRC** `[8:8]`
    ///
    /// 0: Do not check; 1: Check response CRC.
    /// Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller.
    #[inline(always)]
    pub const fn with_check_response_crc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::CHECK_RESPONSE_CRC_MASK
            } else {
                self.bits & !Cmd::CHECK_RESPONSE_CRC_MASK
            },
        }
    }

    /// Set **RESPONSE_LENGTH** `[7:7]`
    ///
    /// 0: Short response expected from card; 1: Long response expected from card.
    #[inline(always)]
    pub const fn with_response_length(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::RESPONSE_LENGTH_MASK
            } else {
                self.bits & !Cmd::RESPONSE_LENGTH_MASK
            },
        }
    }

    /// Set **RESPONSE_EXPECT** `[6:6]`
    ///
    /// 0: No response expected from card; 1: Response expected from card.
    #[inline(always)]
    pub const fn with_response_expect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::RESPONSE_EXPECT_MASK
            } else {
                self.bits & !Cmd::RESPONSE_EXPECT_MASK
            },
        }
    }

    /// Set **INDEX** `[5:0]`
    ///
    /// Command index.
    #[inline(always)]
    pub const fn with_index(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmd::INDEX_MASK) | ((value << Cmd::INDEX_OFFSET) & Cmd::INDEX_MASK),
        }
    }
}

impl Default for CmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdVal {
        CmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmdVal) -> CmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESP0 Register
///
/// Response data register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **RESPONSE0** `[31:0]` (RO)
///
///   Bit[31:0] of response.
#[repr(transparent)]
pub struct Resp0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Resp0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Resp0 {
    // ======== Field Constants ========

    /// **RESPONSE0**
    ///
    /// Bit[31:0] of response.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RESPONSE0_OFFSET: u32 = 0;
    pub const RESPONSE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RESPONSE0** `[31:0]` (RO)
    ///
    /// Bit[31:0] of response.
    #[inline(always)]
    pub fn response0(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE0_MASK) >> Self::RESPONSE0_OFFSET
    }
}

/// RESP0 value type for batch read/modify/write operations
///
/// Use `resp0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resp0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Resp0Val {
    bits: u32,
}

impl Resp0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RESPONSE0** `[31:0]`
    ///
    /// Bit[31:0] of response.
    #[inline(always)]
    pub const fn response0(&self) -> u32 {
        (self.bits & Resp0::RESPONSE0_MASK) >> Resp0::RESPONSE0_OFFSET
    }
}

impl Default for Resp0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Resp0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Resp0Val {
        Resp0Val::from_bits(self.value.read_bits())
    }
}

/// RESP1 Register
///
/// Long response data register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **RESPONSE1** `[31:0]` (RO)
///
///   Bit[63:32] of long response.
#[repr(transparent)]
pub struct Resp1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Resp1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Resp1 {
    // ======== Field Constants ========

    /// **RESPONSE1**
    ///
    /// Bit[63:32] of long response.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RESPONSE1_OFFSET: u32 = 0;
    pub const RESPONSE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RESPONSE1** `[31:0]` (RO)
    ///
    /// Bit[63:32] of long response.
    #[inline(always)]
    pub fn response1(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE1_MASK) >> Self::RESPONSE1_OFFSET
    }
}

/// RESP1 value type for batch read/modify/write operations
///
/// Use `resp1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resp1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Resp1Val {
    bits: u32,
}

impl Resp1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RESPONSE1** `[31:0]`
    ///
    /// Bit[63:32] of long response.
    #[inline(always)]
    pub const fn response1(&self) -> u32 {
        (self.bits & Resp1::RESPONSE1_MASK) >> Resp1::RESPONSE1_OFFSET
    }
}

impl Default for Resp1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Resp1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Resp1Val {
        Resp1Val::from_bits(self.value.read_bits())
    }
}

/// RESP2 Register
///
/// Long response data register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **RESPONSE2** `[31:0]` (RO)
///
///   Bit[95:64] of long response.
#[repr(transparent)]
pub struct Resp2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Resp2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Resp2 {
    // ======== Field Constants ========

    /// **RESPONSE2**
    ///
    /// Bit[95:64] of long response.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RESPONSE2_OFFSET: u32 = 0;
    pub const RESPONSE2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RESPONSE2** `[31:0]` (RO)
    ///
    /// Bit[95:64] of long response.
    #[inline(always)]
    pub fn response2(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE2_MASK) >> Self::RESPONSE2_OFFSET
    }
}

/// RESP2 value type for batch read/modify/write operations
///
/// Use `resp2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resp2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Resp2Val {
    bits: u32,
}

impl Resp2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RESPONSE2** `[31:0]`
    ///
    /// Bit[95:64] of long response.
    #[inline(always)]
    pub const fn response2(&self) -> u32 {
        (self.bits & Resp2::RESPONSE2_MASK) >> Resp2::RESPONSE2_OFFSET
    }
}

impl Default for Resp2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Resp2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Resp2Val {
        Resp2Val::from_bits(self.value.read_bits())
    }
}

/// RESP3 Register
///
/// Long response data register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **RESPONSE3** `[31:0]` (RO)
///
///   Bit[127:96] of long response.
#[repr(transparent)]
pub struct Resp3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Resp3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Resp3 {
    // ======== Field Constants ========

    /// **RESPONSE3**
    ///
    /// Bit[127:96] of long response.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RESPONSE3_OFFSET: u32 = 0;
    pub const RESPONSE3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RESPONSE3** `[31:0]` (RO)
    ///
    /// Bit[127:96] of long response.
    #[inline(always)]
    pub fn response3(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE3_MASK) >> Self::RESPONSE3_OFFSET
    }
}

/// RESP3 value type for batch read/modify/write operations
///
/// Use `resp3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resp3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Resp3Val {
    bits: u32,
}

impl Resp3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RESPONSE3** `[31:0]`
    ///
    /// Bit[127:96] of long response.
    #[inline(always)]
    pub const fn response3(&self) -> u32 {
        (self.bits & Resp3::RESPONSE3_MASK) >> Resp3::RESPONSE3_OFFSET
    }
}

impl Default for Resp3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Resp3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Resp3Val {
        Resp3Val::from_bits(self.value.read_bits())
    }
}

/// MINTSTS Register
///
/// Masked interrupt status register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **SDIO_INTERRUPT_MSK** `[17:16]` (RO)
///
///   Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt).
///
/// - **INT_STATUS_MSK** `[15:0]` (RO)
///
///   Interrupt enabled only if corresponding bit in interrupt mask register is set.
///   Bit 15 (EBE): End-bit error/no CRC error;
///   Bit 14 (ACD): Auto command done;
///   Bit 13 (SBE/BCI): RX Start Bit Error;
///   Bit 12 (HLE): Hardware locked write error;
///   Bit 11 (FRUN): FIFO underrun/overrun error;
///   Bit 10 (HTO): Data starvation by host timeout (HTO);
///   Bit 9 (DTRO): Data read timeout;
///   Bit 8 (RTO): Response timeout;
///   Bit 7 (DCRC): Data CRC error;
///   Bit 6 (RCRC): Response CRC error;
///   Bit 5 (RXDR): Receive FIFO data request;
///   Bit 4 (TXDR): Transmit FIFO data request;
///   Bit 3 (DTO): Data transfer over;
///   Bit 2 (CD): Command done;
///   Bit 1 (RE): Response error;
///   Bit 0 (CD): Card detect.
#[repr(transparent)]
pub struct Mintsts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mintsts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mintsts {
    // ======== Field Constants ========

    /// **SDIO_INTERRUPT_MSK**
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt).
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SDIO_INTERRUPT_MSK_OFFSET: u32 = 16;
    pub const SDIO_INTERRUPT_MSK_MASK: u32 = 0x00030000;

    /// **INT_STATUS_MSK**
    ///
    /// Interrupt enabled only if corresponding bit in interrupt mask register is set.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INT_STATUS_MSK_OFFSET: u32 = 0;
    pub const INT_STATUS_MSK_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SDIO_INTERRUPT_MSK** `[17:16]` (RO)
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt).
    #[inline(always)]
    pub fn sdio_interrupt_msk(&self) -> u32 {
        (self.value.read_bits() & Self::SDIO_INTERRUPT_MSK_MASK) >> Self::SDIO_INTERRUPT_MSK_OFFSET
    }

    /// **INT_STATUS_MSK** `[15:0]` (RO)
    ///
    /// Interrupt enabled only if corresponding bit in interrupt mask register is set.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn int_status_msk(&self) -> u32 {
        (self.value.read_bits() & Self::INT_STATUS_MSK_MASK) >> Self::INT_STATUS_MSK_OFFSET
    }
}

/// MINTSTS value type for batch read/modify/write operations
///
/// Use `mintsts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mintsts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MintstsVal {
    bits: u32,
}

impl MintstsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SDIO_INTERRUPT_MSK** `[17:16]`
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt).
    #[inline(always)]
    pub const fn sdio_interrupt_msk(&self) -> u32 {
        (self.bits & Mintsts::SDIO_INTERRUPT_MSK_MASK) >> Mintsts::SDIO_INTERRUPT_MSK_OFFSET
    }

    /// Read **INT_STATUS_MSK** `[15:0]`
    ///
    /// Interrupt enabled only if corresponding bit in interrupt mask register is set.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub const fn int_status_msk(&self) -> u32 {
        (self.bits & Mintsts::INT_STATUS_MSK_MASK) >> Mintsts::INT_STATUS_MSK_OFFSET
    }
}

impl Default for MintstsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Mintsts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MintstsVal {
        MintstsVal::from_bits(self.value.read_bits())
    }
}

/// RINTSTS Register
///
/// Raw interrupt status register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **SDIO_INTERRUPT_RAW** `[17:16]` (RW)
///
///   Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
///   0: No SDIO interrupt from card;
///   1: SDIO interrupt from card.
///
/// - **INT_STATUS_RAW** `[15:0]` (RW)
///
///   Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
///   Bit 15 (EBE): End-bit error/no CRC error;
///   Bit 14 (ACD): Auto command done;
///   Bit 13 (SBE/BCI): RX Start Bit Error;
///   Bit 12 (HLE): Hardware locked write error;
///   Bit 11 (FRUN): FIFO underrun/overrun error;
///   Bit 10 (HTO): Data starvation by host timeout (HTO);
///   Bit 9 (DTRO): Data read timeout;
///   Bit 8 (RTO): Response timeout;
///   Bit 7 (DCRC): Data CRC error;
///   Bit 6 (RCRC): Response CRC error;
///   Bit 5 (RXDR): Receive FIFO data request;
///   Bit 4 (TXDR): Transmit FIFO data request;
///   Bit 3 (DTO): Data transfer over;
///   Bit 2 (CD): Command done;
///   Bit 1 (RE): Response error;
///   Bit 0 (CD): Card detect.
#[repr(transparent)]
pub struct Rintsts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rintsts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rintsts {
    // ======== Field Constants ========

    /// **SDIO_INTERRUPT_RAW**
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
    /// 0: No SDIO interrupt from card;
    /// 1: SDIO interrupt from card.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SDIO_INTERRUPT_RAW_OFFSET: u32 = 16;
    pub const SDIO_INTERRUPT_RAW_MASK: u32 = 0x00030000;

    /// **INT_STATUS_RAW**
    ///
    /// Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INT_STATUS_RAW_OFFSET: u32 = 0;
    pub const INT_STATUS_RAW_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SDIO_INTERRUPT_RAW** `[17:16]` (RW)
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
    /// 0: No SDIO interrupt from card;
    /// 1: SDIO interrupt from card.
    #[inline(always)]
    pub fn sdio_interrupt_raw(&self) -> u32 {
        (self.value.read_bits() & Self::SDIO_INTERRUPT_RAW_MASK) >> Self::SDIO_INTERRUPT_RAW_OFFSET
    }

    /// Write **SDIO_INTERRUPT_RAW** `[17:16]` (RW)
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
    /// 0: No SDIO interrupt from card;
    /// 1: SDIO interrupt from card.
    #[inline(always)]
    pub fn write_sdio_interrupt_raw(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SDIO_INTERRUPT_RAW_MASK)
                | ((value << Self::SDIO_INTERRUPT_RAW_OFFSET) & Self::SDIO_INTERRUPT_RAW_MASK)
        });
    }

    /// **INT_STATUS_RAW** `[15:0]` (RW)
    ///
    /// Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn int_status_raw(&self) -> u32 {
        (self.value.read_bits() & Self::INT_STATUS_RAW_MASK) >> Self::INT_STATUS_RAW_OFFSET
    }

    /// Write **INT_STATUS_RAW** `[15:0]` (RW)
    ///
    /// Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn write_int_status_raw(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INT_STATUS_RAW_MASK)
                | ((value << Self::INT_STATUS_RAW_OFFSET) & Self::INT_STATUS_RAW_MASK)
        });
    }
}

/// RINTSTS value type for batch read/modify/write operations
///
/// Use `rintsts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rintsts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RintstsVal {
    bits: u32,
}

impl RintstsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SDIO_INTERRUPT_RAW** `[17:16]`
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
    /// 0: No SDIO interrupt from card;
    /// 1: SDIO interrupt from card.
    #[inline(always)]
    pub const fn sdio_interrupt_raw(&self) -> u32 {
        (self.bits & Rintsts::SDIO_INTERRUPT_RAW_MASK) >> Rintsts::SDIO_INTERRUPT_RAW_OFFSET
    }

    /// Read **INT_STATUS_RAW** `[15:0]`
    ///
    /// Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub const fn int_status_raw(&self) -> u32 {
        (self.bits & Rintsts::INT_STATUS_RAW_MASK) >> Rintsts::INT_STATUS_RAW_OFFSET
    }

    /// Set **SDIO_INTERRUPT_RAW** `[17:16]`
    ///
    /// Interrupt from SDIO card, one bit for each card. Bit[17:16] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect.
    /// 0: No SDIO interrupt from card;
    /// 1: SDIO interrupt from card.
    #[inline(always)]
    pub const fn with_sdio_interrupt_raw(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rintsts::SDIO_INTERRUPT_RAW_MASK)
                | ((value << Rintsts::SDIO_INTERRUPT_RAW_OFFSET)
                    & Rintsts::SDIO_INTERRUPT_RAW_MASK),
        }
    }

    /// Set **INT_STATUS_RAW** `[15:0]`
    ///
    /// Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status.
    /// Bit 15 (EBE): End-bit error/no CRC error;
    /// Bit 14 (ACD): Auto command done;
    /// Bit 13 (SBE/BCI): RX Start Bit Error;
    /// Bit 12 (HLE): Hardware locked write error;
    /// Bit 11 (FRUN): FIFO underrun/overrun error;
    /// Bit 10 (HTO): Data starvation by host timeout (HTO);
    /// Bit 9 (DTRO): Data read timeout;
    /// Bit 8 (RTO): Response timeout;
    /// Bit 7 (DCRC): Data CRC error;
    /// Bit 6 (RCRC): Response CRC error;
    /// Bit 5 (RXDR): Receive FIFO data request;
    /// Bit 4 (TXDR): Transmit FIFO data request;
    /// Bit 3 (DTO): Data transfer over;
    /// Bit 2 (CD): Command done;
    /// Bit 1 (RE): Response error;
    /// Bit 0 (CD): Card detect.
    #[inline(always)]
    pub const fn with_int_status_raw(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rintsts::INT_STATUS_RAW_MASK)
                | ((value << Rintsts::INT_STATUS_RAW_OFFSET) & Rintsts::INT_STATUS_RAW_MASK),
        }
    }
}

impl Default for RintstsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rintsts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RintstsVal {
        RintstsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RintstsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RintstsVal) -> RintstsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS Register
///
/// SD/MMC status register
///
/// Address offset: 0x0048
/// Reset value: 0x00000716
///
/// ## Fields
///
/// - **FIFO_COUNT** `[29:17]` (RO)
///
///   FIFO count, number of filled locations in FIFO.
///
/// - **RESPONSE_INDEX** `[16:11]` (RO)
///
///   Index of previous response, including any auto-stop sent by core.
///
/// - **DATA_STATE_MC_BUSY** `[10]` (RO)
///
///   Data transmit or receive state-machine is busy.
///
/// - **DATA_BUSY** `[9]` (RO)
///
///   Inverted version of raw selected sdhost_card_data[0].
///   0: Card data not busy;
///   1: Card data busy.
///
/// - **DATA_3_STATUS** `[8]` (RO)
///
///   Raw selected sdhost_card_data[3], checks whether card is present.
///   0: card not present;
///   1: card present.
///
/// - **COMMAND_FSM_STATES** `[7:4]` (RO)
///
///   Command FSM states.
///   0: Idle;
///   1: Send init sequence;
///   2: Send cmd start bit;
///   3: Send cmd tx bit;
///   4: Send cmd index + arg;
///   5: Send cmd crc7;
///   6: Send cmd end bit;
///   7: Receive resp start bit;
///   8: Receive resp IRQ response;
///   9: Receive resp tx bit;
///   10: Receive resp cmd idx;
///   11: Receive resp data;
///   12: Receive resp crc7;
///   13: Receive resp end bit;
///   14: Cmd path wait NCC;
///   15: Wait, cmd-to-response turnaround.
///
/// - **FIFO_FULL** `[3]` (RO)
///
///   FIFO is full status.
///
/// - **FIFO_EMPTY** `[2]` (RO)
///
///   FIFO is empty status.
///
/// - **FIFO_TX_WATERMARK** `[1]` (RO)
///
///   FIFO reached Transmit watermark level, not qualified with data transfer.
///
/// - **FIFO_RX_WATERMARK** `[0]` (RO)
///
///   FIFO reached Receive watermark level, not qualified with data transfer.
#[repr(transparent)]
pub struct Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status {
    // ======== Field Constants ========

    /// **FIFO_COUNT**
    ///
    /// FIFO count, number of filled locations in FIFO.
    ///
    /// Bits `[29:17]`, mask `0x3ffe0000`
    pub const FIFO_COUNT_OFFSET: u32 = 17;
    pub const FIFO_COUNT_MASK: u32 = 0x3ffe0000;

    /// **RESPONSE_INDEX**
    ///
    /// Index of previous response, including any auto-stop sent by core.
    ///
    /// Bits `[16:11]`, mask `0x0001f800`
    pub const RESPONSE_INDEX_OFFSET: u32 = 11;
    pub const RESPONSE_INDEX_MASK: u32 = 0x0001f800;

    /// **DATA_STATE_MC_BUSY**
    ///
    /// Data transmit or receive state-machine is busy.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DATA_STATE_MC_BUSY_OFFSET: u32 = 10;
    pub const DATA_STATE_MC_BUSY_MASK: u32 = 0x00000400;
    pub const DATA_STATE_MC_BUSY: u32 = 1 << 10;

    /// **DATA_BUSY**
    ///
    /// Inverted version of raw selected sdhost_card_data[0].
    /// 0: Card data not busy;
    /// 1: Card data busy.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DATA_BUSY_OFFSET: u32 = 9;
    pub const DATA_BUSY_MASK: u32 = 0x00000200;
    pub const DATA_BUSY: u32 = 1 << 9;

    /// **DATA_3_STATUS**
    ///
    /// Raw selected sdhost_card_data[3], checks whether card is present.
    /// 0: card not present;
    /// 1: card present.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DATA_3_STATUS_OFFSET: u32 = 8;
    pub const DATA_3_STATUS_MASK: u32 = 0x00000100;
    pub const DATA_3_STATUS: u32 = 1 << 8;

    /// **COMMAND_FSM_STATES**
    ///
    /// Command FSM states.
    /// 0: Idle;
    /// 1: Send init sequence;
    /// 2: Send cmd start bit;
    /// 3: Send cmd tx bit;
    /// 4: Send cmd index + arg;
    /// 5: Send cmd crc7;
    /// 6: Send cmd end bit;
    /// 7: Receive resp start bit;
    /// 8: Receive resp IRQ response;
    /// 9: Receive resp tx bit;
    /// 10: Receive resp cmd idx;
    /// 11: Receive resp data;
    /// 12: Receive resp crc7;
    /// 13: Receive resp end bit;
    /// 14: Cmd path wait NCC;
    /// 15: Wait, cmd-to-response turnaround.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const COMMAND_FSM_STATES_OFFSET: u32 = 4;
    pub const COMMAND_FSM_STATES_MASK: u32 = 0x000000f0;

    /// **FIFO_FULL**
    ///
    /// FIFO is full status.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FIFO_FULL_OFFSET: u32 = 3;
    pub const FIFO_FULL_MASK: u32 = 0x00000008;
    pub const FIFO_FULL: u32 = 1 << 3;

    /// **FIFO_EMPTY**
    ///
    /// FIFO is empty status.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FIFO_EMPTY_OFFSET: u32 = 2;
    pub const FIFO_EMPTY_MASK: u32 = 0x00000004;
    pub const FIFO_EMPTY: u32 = 1 << 2;

    /// **FIFO_TX_WATERMARK**
    ///
    /// FIFO reached Transmit watermark level, not qualified with data transfer.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FIFO_TX_WATERMARK_OFFSET: u32 = 1;
    pub const FIFO_TX_WATERMARK_MASK: u32 = 0x00000002;
    pub const FIFO_TX_WATERMARK: u32 = 1 << 1;

    /// **FIFO_RX_WATERMARK**
    ///
    /// FIFO reached Receive watermark level, not qualified with data transfer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FIFO_RX_WATERMARK_OFFSET: u32 = 0;
    pub const FIFO_RX_WATERMARK_MASK: u32 = 0x00000001;
    pub const FIFO_RX_WATERMARK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FIFO_COUNT** `[29:17]` (RO)
    ///
    /// FIFO count, number of filled locations in FIFO.
    #[inline(always)]
    pub fn fifo_count(&self) -> u32 {
        (self.value.read_bits() & Self::FIFO_COUNT_MASK) >> Self::FIFO_COUNT_OFFSET
    }

    /// **RESPONSE_INDEX** `[16:11]` (RO)
    ///
    /// Index of previous response, including any auto-stop sent by core.
    #[inline(always)]
    pub fn response_index(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE_INDEX_MASK) >> Self::RESPONSE_INDEX_OFFSET
    }

    /// **DATA_STATE_MC_BUSY** `[10]` (RO)
    ///
    /// Data transmit or receive state-machine is busy.
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> bool {
        (self.value.read_bits() & Self::DATA_STATE_MC_BUSY_MASK) != 0
    }

    /// **DATA_BUSY** `[9]` (RO)
    ///
    /// Inverted version of raw selected sdhost_card_data[0].
    /// 0: Card data not busy;
    /// 1: Card data busy.
    #[inline(always)]
    pub fn data_busy(&self) -> bool {
        (self.value.read_bits() & Self::DATA_BUSY_MASK) != 0
    }

    /// **DATA_3_STATUS** `[8]` (RO)
    ///
    /// Raw selected sdhost_card_data[3], checks whether card is present.
    /// 0: card not present;
    /// 1: card present.
    #[inline(always)]
    pub fn data_3_status(&self) -> bool {
        (self.value.read_bits() & Self::DATA_3_STATUS_MASK) != 0
    }

    /// **COMMAND_FSM_STATES** `[7:4]` (RO)
    ///
    /// Command FSM states.
    /// 0: Idle;
    /// 1: Send init sequence;
    /// 2: Send cmd start bit;
    /// 3: Send cmd tx bit;
    /// 4: Send cmd index + arg;
    /// 5: Send cmd crc7;
    /// 6: Send cmd end bit;
    /// 7: Receive resp start bit;
    /// 8: Receive resp IRQ response;
    /// 9: Receive resp tx bit;
    /// 10: Receive resp cmd idx;
    /// 11: Receive resp data;
    /// 12: Receive resp crc7;
    /// 13: Receive resp end bit;
    /// 14: Cmd path wait NCC;
    /// 15: Wait, cmd-to-response turnaround.
    #[inline(always)]
    pub fn command_fsm_states(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND_FSM_STATES_MASK) >> Self::COMMAND_FSM_STATES_OFFSET
    }

    /// **FIFO_FULL** `[3]` (RO)
    ///
    /// FIFO is full status.
    #[inline(always)]
    pub fn fifo_full(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_FULL_MASK) != 0
    }

    /// **FIFO_EMPTY** `[2]` (RO)
    ///
    /// FIFO is empty status.
    #[inline(always)]
    pub fn fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_EMPTY_MASK) != 0
    }

    /// **FIFO_TX_WATERMARK** `[1]` (RO)
    ///
    /// FIFO reached Transmit watermark level, not qualified with data transfer.
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_TX_WATERMARK_MASK) != 0
    }

    /// **FIFO_RX_WATERMARK** `[0]` (RO)
    ///
    /// FIFO reached Receive watermark level, not qualified with data transfer.
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_RX_WATERMARK_MASK) != 0
    }
}

/// STATUS value type for batch read/modify/write operations
///
/// Use `status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusVal {
    bits: u32,
}

impl StatusVal {
    /// Create from raw bits
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
        Self { bits: 0x00000716 }
    }

    /// Read **FIFO_COUNT** `[29:17]`
    ///
    /// FIFO count, number of filled locations in FIFO.
    #[inline(always)]
    pub const fn fifo_count(&self) -> u32 {
        (self.bits & Status::FIFO_COUNT_MASK) >> Status::FIFO_COUNT_OFFSET
    }

    /// Read **RESPONSE_INDEX** `[16:11]`
    ///
    /// Index of previous response, including any auto-stop sent by core.
    #[inline(always)]
    pub const fn response_index(&self) -> u32 {
        (self.bits & Status::RESPONSE_INDEX_MASK) >> Status::RESPONSE_INDEX_OFFSET
    }

    /// Read **DATA_STATE_MC_BUSY** `[10:10]`
    ///
    /// Data transmit or receive state-machine is busy.
    #[inline(always)]
    pub const fn data_state_mc_busy(&self) -> bool {
        (self.bits & Status::DATA_STATE_MC_BUSY_MASK) != 0
    }

    /// Read **DATA_BUSY** `[9:9]`
    ///
    /// Inverted version of raw selected sdhost_card_data[0].
    /// 0: Card data not busy;
    /// 1: Card data busy.
    #[inline(always)]
    pub const fn data_busy(&self) -> bool {
        (self.bits & Status::DATA_BUSY_MASK) != 0
    }

    /// Read **DATA_3_STATUS** `[8:8]`
    ///
    /// Raw selected sdhost_card_data[3], checks whether card is present.
    /// 0: card not present;
    /// 1: card present.
    #[inline(always)]
    pub const fn data_3_status(&self) -> bool {
        (self.bits & Status::DATA_3_STATUS_MASK) != 0
    }

    /// Read **COMMAND_FSM_STATES** `[7:4]`
    ///
    /// Command FSM states.
    /// 0: Idle;
    /// 1: Send init sequence;
    /// 2: Send cmd start bit;
    /// 3: Send cmd tx bit;
    /// 4: Send cmd index + arg;
    /// 5: Send cmd crc7;
    /// 6: Send cmd end bit;
    /// 7: Receive resp start bit;
    /// 8: Receive resp IRQ response;
    /// 9: Receive resp tx bit;
    /// 10: Receive resp cmd idx;
    /// 11: Receive resp data;
    /// 12: Receive resp crc7;
    /// 13: Receive resp end bit;
    /// 14: Cmd path wait NCC;
    /// 15: Wait, cmd-to-response turnaround.
    #[inline(always)]
    pub const fn command_fsm_states(&self) -> u32 {
        (self.bits & Status::COMMAND_FSM_STATES_MASK) >> Status::COMMAND_FSM_STATES_OFFSET
    }

    /// Read **FIFO_FULL** `[3:3]`
    ///
    /// FIFO is full status.
    #[inline(always)]
    pub const fn fifo_full(&self) -> bool {
        (self.bits & Status::FIFO_FULL_MASK) != 0
    }

    /// Read **FIFO_EMPTY** `[2:2]`
    ///
    /// FIFO is empty status.
    #[inline(always)]
    pub const fn fifo_empty(&self) -> bool {
        (self.bits & Status::FIFO_EMPTY_MASK) != 0
    }

    /// Read **FIFO_TX_WATERMARK** `[1:1]`
    ///
    /// FIFO reached Transmit watermark level, not qualified with data transfer.
    #[inline(always)]
    pub const fn fifo_tx_watermark(&self) -> bool {
        (self.bits & Status::FIFO_TX_WATERMARK_MASK) != 0
    }

    /// Read **FIFO_RX_WATERMARK** `[0:0]`
    ///
    /// FIFO reached Receive watermark level, not qualified with data transfer.
    #[inline(always)]
    pub const fn fifo_rx_watermark(&self) -> bool {
        (self.bits & Status::FIFO_RX_WATERMARK_MASK) != 0
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
    }
}

/// FIFOTH Register
///
/// FIFO configuration register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **DMA_MULTIPLE_TRANSACTION_SIZE** `[30:28]` (RW)
///
///   Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
///   000: 1-byte transfer;
///   001: 4-byte transfer;
///   010: 8-byte transfer;
///   011: 16-byte transfer;
///   100: 32-byte transfer;
///   101: 64-byte transfer;
///   110: 128-byte transfer;
///   111: 256-byte transfer.
///
/// - **RX_WMARK** `[26:16]` (RW)
///
///   FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
///
/// - **TX_WMARK** `[11:0]` (RW)
///
///   FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
#[repr(transparent)]
pub struct Fifoth {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Fifoth {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fifoth {
    // ======== Field Constants ========

    /// **DMA_MULTIPLE_TRANSACTION_SIZE**
    ///
    /// Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const DMA_MULTIPLE_TRANSACTION_SIZE_OFFSET: u32 = 28;
    pub const DMA_MULTIPLE_TRANSACTION_SIZE_MASK: u32 = 0x70000000;

    /// **RX_WMARK**
    ///
    /// FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
    ///
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const RX_WMARK_OFFSET: u32 = 16;
    pub const RX_WMARK_MASK: u32 = 0x07ff0000;

    /// **TX_WMARK**
    ///
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const TX_WMARK_OFFSET: u32 = 0;
    pub const TX_WMARK_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **DMA_MULTIPLE_TRANSACTION_SIZE** `[30:28]` (RW)
    ///
    /// Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    #[inline(always)]
    pub fn dma_multiple_transaction_size(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_MULTIPLE_TRANSACTION_SIZE_MASK)
            >> Self::DMA_MULTIPLE_TRANSACTION_SIZE_OFFSET
    }

    /// Write **DMA_MULTIPLE_TRANSACTION_SIZE** `[30:28]` (RW)
    ///
    /// Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    #[inline(always)]
    pub fn write_dma_multiple_transaction_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_MULTIPLE_TRANSACTION_SIZE_MASK)
                | ((value << Self::DMA_MULTIPLE_TRANSACTION_SIZE_OFFSET)
                    & Self::DMA_MULTIPLE_TRANSACTION_SIZE_MASK)
        });
    }

    /// **RX_WMARK** `[26:16]` (RW)
    ///
    /// FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
    #[inline(always)]
    pub fn rx_wmark(&self) -> u32 {
        (self.value.read_bits() & Self::RX_WMARK_MASK) >> Self::RX_WMARK_OFFSET
    }

    /// Write **RX_WMARK** `[26:16]` (RW)
    ///
    /// FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
    #[inline(always)]
    pub fn write_rx_wmark(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_WMARK_MASK) | ((value << Self::RX_WMARK_OFFSET) & Self::RX_WMARK_MASK)
        });
    }

    /// **TX_WMARK** `[11:0]` (RW)
    ///
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
    #[inline(always)]
    pub fn tx_wmark(&self) -> u32 {
        (self.value.read_bits() & Self::TX_WMARK_MASK) >> Self::TX_WMARK_OFFSET
    }

    /// Write **TX_WMARK** `[11:0]` (RW)
    ///
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
    #[inline(always)]
    pub fn write_tx_wmark(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_WMARK_MASK) | ((value << Self::TX_WMARK_OFFSET) & Self::TX_WMARK_MASK)
        });
    }
}

/// FIFOTH value type for batch read/modify/write operations
///
/// Use `fifoth.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifoth.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifothVal {
    bits: u32,
}

impl FifothVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DMA_MULTIPLE_TRANSACTION_SIZE** `[30:28]`
    ///
    /// Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    #[inline(always)]
    pub const fn dma_multiple_transaction_size(&self) -> u32 {
        (self.bits & Fifoth::DMA_MULTIPLE_TRANSACTION_SIZE_MASK)
            >> Fifoth::DMA_MULTIPLE_TRANSACTION_SIZE_OFFSET
    }

    /// Read **RX_WMARK** `[26:16]`
    ///
    /// FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
    #[inline(always)]
    pub const fn rx_wmark(&self) -> u32 {
        (self.bits & Fifoth::RX_WMARK_MASK) >> Fifoth::RX_WMARK_OFFSET
    }

    /// Read **TX_WMARK** `[11:0]`
    ///
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
    #[inline(always)]
    pub const fn tx_wmark(&self) -> u32 {
        (self.bits & Fifoth::TX_WMARK_MASK) >> Fifoth::TX_WMARK_OFFSET
    }

    /// Set **DMA_MULTIPLE_TRANSACTION_SIZE** `[30:28]`
    ///
    /// Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE.
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    #[inline(always)]
    pub const fn with_dma_multiple_transaction_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fifoth::DMA_MULTIPLE_TRANSACTION_SIZE_MASK)
                | ((value << Fifoth::DMA_MULTIPLE_TRANSACTION_SIZE_OFFSET)
                    & Fifoth::DMA_MULTIPLE_TRANSACTION_SIZE_MASK),
        }
    }

    /// Set **RX_WMARK** `[26:16]`
    ///
    /// FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set.
    #[inline(always)]
    pub const fn with_rx_wmark(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fifoth::RX_WMARK_MASK)
                | ((value << Fifoth::RX_WMARK_OFFSET) & Fifoth::RX_WMARK_MASK),
        }
    }

    /// Set **TX_WMARK** `[11:0]`
    ///
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred.
    #[inline(always)]
    pub const fn with_tx_wmark(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fifoth::TX_WMARK_MASK)
                | ((value << Fifoth::TX_WMARK_OFFSET) & Fifoth::TX_WMARK_MASK),
        }
    }
}

impl Default for FifothVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Fifoth {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifothVal {
        FifothVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifothVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifothVal) -> FifothVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CDETECT Register
///
/// Card detect register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **CARD_DETECT_N** `[1:0]` (RO)
///
///   Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented.
#[repr(transparent)]
pub struct Cdetect {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cdetect {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cdetect {
    // ======== Field Constants ========

    /// **CARD_DETECT_N**
    ///
    /// Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CARD_DETECT_N_OFFSET: u32 = 0;
    pub const CARD_DETECT_N_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CARD_DETECT_N** `[1:0]` (RO)
    ///
    /// Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented.
    #[inline(always)]
    pub fn card_detect_n(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_DETECT_N_MASK) >> Self::CARD_DETECT_N_OFFSET
    }
}

/// CDETECT value type for batch read/modify/write operations
///
/// Use `cdetect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cdetect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CdetectVal {
    bits: u32,
}

impl CdetectVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CARD_DETECT_N** `[1:0]`
    ///
    /// Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented.
    #[inline(always)]
    pub const fn card_detect_n(&self) -> u32 {
        (self.bits & Cdetect::CARD_DETECT_N_MASK) >> Cdetect::CARD_DETECT_N_OFFSET
    }
}

impl Default for CdetectVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cdetect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CdetectVal {
        CdetectVal::from_bits(self.value.read_bits())
    }
}

/// WRTPRT Register
///
/// Card write protection (WP) status register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **WRITE_PROTECT** `[1:0]` (RO)
///
///   Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
#[repr(transparent)]
pub struct Wrtprt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wrtprt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wrtprt {
    // ======== Field Constants ========

    /// **WRITE_PROTECT**
    ///
    /// Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const WRITE_PROTECT_OFFSET: u32 = 0;
    pub const WRITE_PROTECT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **WRITE_PROTECT** `[1:0]` (RO)
    ///
    /// Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
    #[inline(always)]
    pub fn write_protect(&self) -> u32 {
        (self.value.read_bits() & Self::WRITE_PROTECT_MASK) >> Self::WRITE_PROTECT_OFFSET
    }
}

/// WRTPRT value type for batch read/modify/write operations
///
/// Use `wrtprt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wrtprt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WrtprtVal {
    bits: u32,
}

impl WrtprtVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WRITE_PROTECT** `[1:0]`
    ///
    /// Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
    #[inline(always)]
    pub const fn write_protect(&self) -> u32 {
        (self.bits & Wrtprt::WRITE_PROTECT_MASK) >> Wrtprt::WRITE_PROTECT_OFFSET
    }
}

impl Default for WrtprtVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Wrtprt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WrtprtVal {
        WrtprtVal::from_bits(self.value.read_bits())
    }
}

/// TCBCNT Register
///
/// Transferred byte count register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **TCBCNT** `[31:0]` (RO)
///
///   Number of bytes transferred by CIU unit to card.
#[repr(transparent)]
pub struct Tcbcnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tcbcnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tcbcnt {
    // ======== Field Constants ========

    /// **TCBCNT**
    ///
    /// Number of bytes transferred by CIU unit to card.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TCBCNT_OFFSET: u32 = 0;
    pub const TCBCNT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TCBCNT** `[31:0]` (RO)
    ///
    /// Number of bytes transferred by CIU unit to card.
    #[inline(always)]
    pub fn tcbcnt(&self) -> u32 {
        (self.value.read_bits() & Self::TCBCNT_MASK) >> Self::TCBCNT_OFFSET
    }
}

/// TCBCNT value type for batch read/modify/write operations
///
/// Use `tcbcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcbcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcbcntVal {
    bits: u32,
}

impl TcbcntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TCBCNT** `[31:0]`
    ///
    /// Number of bytes transferred by CIU unit to card.
    #[inline(always)]
    pub const fn tcbcnt(&self) -> u32 {
        (self.bits & Tcbcnt::TCBCNT_MASK) >> Tcbcnt::TCBCNT_OFFSET
    }
}

impl Default for TcbcntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tcbcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcbcntVal {
        TcbcntVal::from_bits(self.value.read_bits())
    }
}

/// TBBCNT Register
///
/// Transferred byte count register
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **TBBCNT** `[31:0]` (RO)
///
///   Number of bytes transferred between Host/DMA memory and BIU FIFO.
#[repr(transparent)]
pub struct Tbbcnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tbbcnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tbbcnt {
    // ======== Field Constants ========

    /// **TBBCNT**
    ///
    /// Number of bytes transferred between Host/DMA memory and BIU FIFO.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TBBCNT_OFFSET: u32 = 0;
    pub const TBBCNT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TBBCNT** `[31:0]` (RO)
    ///
    /// Number of bytes transferred between Host/DMA memory and BIU FIFO.
    #[inline(always)]
    pub fn tbbcnt(&self) -> u32 {
        (self.value.read_bits() & Self::TBBCNT_MASK) >> Self::TBBCNT_OFFSET
    }
}

/// TBBCNT value type for batch read/modify/write operations
///
/// Use `tbbcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tbbcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TbbcntVal {
    bits: u32,
}

impl TbbcntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TBBCNT** `[31:0]`
    ///
    /// Number of bytes transferred between Host/DMA memory and BIU FIFO.
    #[inline(always)]
    pub const fn tbbcnt(&self) -> u32 {
        (self.bits & Tbbcnt::TBBCNT_MASK) >> Tbbcnt::TBBCNT_OFFSET
    }
}

impl Default for TbbcntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tbbcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TbbcntVal {
        TbbcntVal::from_bits(self.value.read_bits())
    }
}

/// DEBNCE Register
///
/// Debounce filter time configuration register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **DEBOUNCE_COUNT** `[23:0]` (RW)
///
///   Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
#[repr(transparent)]
pub struct Debnce {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Debnce {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Debnce {
    // ======== Field Constants ========

    /// **DEBOUNCE_COUNT**
    ///
    /// Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const DEBOUNCE_COUNT_OFFSET: u32 = 0;
    pub const DEBOUNCE_COUNT_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **DEBOUNCE_COUNT** `[23:0]` (RW)
    ///
    /// Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
    #[inline(always)]
    pub fn debounce_count(&self) -> u32 {
        (self.value.read_bits() & Self::DEBOUNCE_COUNT_MASK) >> Self::DEBOUNCE_COUNT_OFFSET
    }

    /// Write **DEBOUNCE_COUNT** `[23:0]` (RW)
    ///
    /// Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
    #[inline(always)]
    pub fn write_debounce_count(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DEBOUNCE_COUNT_MASK)
                | ((value << Self::DEBOUNCE_COUNT_OFFSET) & Self::DEBOUNCE_COUNT_MASK)
        });
    }
}

/// DEBNCE value type for batch read/modify/write operations
///
/// Use `debnce.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debnce.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebnceVal {
    bits: u32,
}

impl DebnceVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DEBOUNCE_COUNT** `[23:0]`
    ///
    /// Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
    #[inline(always)]
    pub const fn debounce_count(&self) -> u32 {
        (self.bits & Debnce::DEBOUNCE_COUNT_MASK) >> Debnce::DEBOUNCE_COUNT_OFFSET
    }

    /// Set **DEBOUNCE_COUNT** `[23:0]`
    ///
    /// Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \verb+~+ 25 ms to prevent the card instability when the card is inserted or removed.
    #[inline(always)]
    pub const fn with_debounce_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Debnce::DEBOUNCE_COUNT_MASK)
                | ((value << Debnce::DEBOUNCE_COUNT_OFFSET) & Debnce::DEBOUNCE_COUNT_MASK),
        }
    }
}

impl Default for DebnceVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Debnce {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DebnceVal {
        DebnceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DebnceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DebnceVal) -> DebnceVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USRID Register
///
/// User ID (scratchpad) register
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **USRID** `[31:0]` (RW)
///
///   User identification register, value set by user. Can also be used as a scratchpad register by user.
#[repr(transparent)]
pub struct Usrid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Usrid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Usrid {
    // ======== Field Constants ========

    /// **USRID**
    ///
    /// User identification register, value set by user. Can also be used as a scratchpad register by user.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USRID_OFFSET: u32 = 0;
    pub const USRID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USRID** `[31:0]` (RW)
    ///
    /// User identification register, value set by user. Can also be used as a scratchpad register by user.
    #[inline(always)]
    pub fn usrid(&self) -> u32 {
        (self.value.read_bits() & Self::USRID_MASK) >> Self::USRID_OFFSET
    }

    /// Write **USRID** `[31:0]` (RW)
    ///
    /// User identification register, value set by user. Can also be used as a scratchpad register by user.
    #[inline(always)]
    pub fn write_usrid(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USRID_MASK) | ((value << Self::USRID_OFFSET) & Self::USRID_MASK)
        });
    }
}

/// USRID value type for batch read/modify/write operations
///
/// Use `usrid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usrid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsridVal {
    bits: u32,
}

impl UsridVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USRID** `[31:0]`
    ///
    /// User identification register, value set by user. Can also be used as a scratchpad register by user.
    #[inline(always)]
    pub const fn usrid(&self) -> u32 {
        (self.bits & Usrid::USRID_MASK) >> Usrid::USRID_OFFSET
    }

    /// Set **USRID** `[31:0]`
    ///
    /// User identification register, value set by user. Can also be used as a scratchpad register by user.
    #[inline(always)]
    pub const fn with_usrid(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Usrid::USRID_MASK)
                | ((value << Usrid::USRID_OFFSET) & Usrid::USRID_MASK),
        }
    }
}

impl Default for UsridVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Usrid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsridVal {
        UsridVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsridVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsridVal) -> UsridVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VERID Register
///
/// Version ID (scratchpad) register
///
/// Address offset: 0x006c
/// Reset value: 0x5432270a
///
/// ## Fields
///
/// - **VERSIONID** `[31:0]` (RO)
///
///   Hardware version register. Can also be read by fireware.
#[repr(transparent)]
pub struct Verid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Verid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Verid {
    // ======== Field Constants ========

    /// **VERSIONID**
    ///
    /// Hardware version register. Can also be read by fireware.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const VERSIONID_OFFSET: u32 = 0;
    pub const VERSIONID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **VERSIONID** `[31:0]` (RO)
    ///
    /// Hardware version register. Can also be read by fireware.
    #[inline(always)]
    pub fn versionid(&self) -> u32 {
        (self.value.read_bits() & Self::VERSIONID_MASK) >> Self::VERSIONID_OFFSET
    }
}

/// VERID value type for batch read/modify/write operations
///
/// Use `verid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `verid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VeridVal {
    bits: u32,
}

impl VeridVal {
    /// Create from raw bits
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
        Self { bits: 0x5432270a }
    }

    /// Read **VERSIONID** `[31:0]`
    ///
    /// Hardware version register. Can also be read by fireware.
    #[inline(always)]
    pub const fn versionid(&self) -> u32 {
        (self.bits & Verid::VERSIONID_MASK) >> Verid::VERSIONID_OFFSET
    }
}

impl Default for VeridVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Verid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VeridVal {
        VeridVal::from_bits(self.value.read_bits())
    }
}

/// HCON Register
///
/// Hardware feature register
///
/// Address offset: 0x0070
/// Reset value: 0x03444cc3
///
/// ## Fields
///
/// - **NUM_CLK_DIV** `[25:24]` (RO)
///
///   Have 4 clk divider in design .
///
/// - **HOLD** `[22]` (RO)
///
///   Have a hold regiser in data path .
///
/// - **RAM_INDISE** `[21]` (RO)
///
///   Inside RAM in SDMMC module.
///
/// - **DMA_WIDTH** `[20:18]` (RO)
///
///   DMA data witdth is 32.
///
/// - **ADDR_WIDTH** `[15:10]` (RO)
///
///   Register address width is 32.
///
/// - **DATA_WIDTH** `[9:7]` (RO)
///
///   Regisger data widht is 32.
///
/// - **BUS_TYPE** `[6]` (RO)
///
///   Register config is APB bus.
///
/// - **CARD_NUM** `[5:1]` (RO)
///
///   Support card number is 2.
///
/// - **CARD_TYPE** `[0]` (RO)
///
///   Hardware support SDIO and MMC.
#[repr(transparent)]
pub struct Hcon {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Hcon {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Hcon {
    // ======== Field Constants ========

    /// **NUM_CLK_DIV**
    ///
    /// Have 4 clk divider in design .
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const NUM_CLK_DIV_OFFSET: u32 = 24;
    pub const NUM_CLK_DIV_MASK: u32 = 0x03000000;

    /// **HOLD**
    ///
    /// Have a hold regiser in data path .
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const HOLD_OFFSET: u32 = 22;
    pub const HOLD_MASK: u32 = 0x00400000;
    pub const HOLD: u32 = 1 << 22;

    /// **RAM_INDISE**
    ///
    /// Inside RAM in SDMMC module.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const RAM_INDISE_OFFSET: u32 = 21;
    pub const RAM_INDISE_MASK: u32 = 0x00200000;
    pub const RAM_INDISE: u32 = 1 << 21;

    /// **DMA_WIDTH**
    ///
    /// DMA data witdth is 32.
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const DMA_WIDTH_OFFSET: u32 = 18;
    pub const DMA_WIDTH_MASK: u32 = 0x001c0000;

    /// **ADDR_WIDTH**
    ///
    /// Register address width is 32.
    ///
    /// Bits `[15:10]`, mask `0x0000fc00`
    pub const ADDR_WIDTH_OFFSET: u32 = 10;
    pub const ADDR_WIDTH_MASK: u32 = 0x0000fc00;

    /// **DATA_WIDTH**
    ///
    /// Regisger data widht is 32.
    ///
    /// Bits `[9:7]`, mask `0x00000380`
    pub const DATA_WIDTH_OFFSET: u32 = 7;
    pub const DATA_WIDTH_MASK: u32 = 0x00000380;

    /// **BUS_TYPE**
    ///
    /// Register config is APB bus.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BUS_TYPE_OFFSET: u32 = 6;
    pub const BUS_TYPE_MASK: u32 = 0x00000040;
    pub const BUS_TYPE: u32 = 1 << 6;

    /// **CARD_NUM**
    ///
    /// Support card number is 2.
    ///
    /// Bits `[5:1]`, mask `0x0000003e`
    pub const CARD_NUM_OFFSET: u32 = 1;
    pub const CARD_NUM_MASK: u32 = 0x0000003e;

    /// **CARD_TYPE**
    ///
    /// Hardware support SDIO and MMC.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CARD_TYPE_OFFSET: u32 = 0;
    pub const CARD_TYPE_MASK: u32 = 0x00000001;
    pub const CARD_TYPE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **NUM_CLK_DIV** `[25:24]` (RO)
    ///
    /// Have 4 clk divider in design .
    #[inline(always)]
    pub fn num_clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::NUM_CLK_DIV_MASK) >> Self::NUM_CLK_DIV_OFFSET
    }

    /// **HOLD** `[22]` (RO)
    ///
    /// Have a hold regiser in data path .
    #[inline(always)]
    pub fn hold(&self) -> bool {
        (self.value.read_bits() & Self::HOLD_MASK) != 0
    }

    /// **RAM_INDISE** `[21]` (RO)
    ///
    /// Inside RAM in SDMMC module.
    #[inline(always)]
    pub fn ram_indise(&self) -> bool {
        (self.value.read_bits() & Self::RAM_INDISE_MASK) != 0
    }

    /// **DMA_WIDTH** `[20:18]` (RO)
    ///
    /// DMA data witdth is 32.
    #[inline(always)]
    pub fn dma_width(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_WIDTH_MASK) >> Self::DMA_WIDTH_OFFSET
    }

    /// **ADDR_WIDTH** `[15:10]` (RO)
    ///
    /// Register address width is 32.
    #[inline(always)]
    pub fn addr_width(&self) -> u32 {
        (self.value.read_bits() & Self::ADDR_WIDTH_MASK) >> Self::ADDR_WIDTH_OFFSET
    }

    /// **DATA_WIDTH** `[9:7]` (RO)
    ///
    /// Regisger data widht is 32.
    #[inline(always)]
    pub fn data_width(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_WIDTH_MASK) >> Self::DATA_WIDTH_OFFSET
    }

    /// **BUS_TYPE** `[6]` (RO)
    ///
    /// Register config is APB bus.
    #[inline(always)]
    pub fn bus_type(&self) -> bool {
        (self.value.read_bits() & Self::BUS_TYPE_MASK) != 0
    }

    /// **CARD_NUM** `[5:1]` (RO)
    ///
    /// Support card number is 2.
    #[inline(always)]
    pub fn card_num(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_NUM_MASK) >> Self::CARD_NUM_OFFSET
    }

    /// **CARD_TYPE** `[0]` (RO)
    ///
    /// Hardware support SDIO and MMC.
    #[inline(always)]
    pub fn card_type(&self) -> bool {
        (self.value.read_bits() & Self::CARD_TYPE_MASK) != 0
    }
}

/// HCON value type for batch read/modify/write operations
///
/// Use `hcon.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hcon.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HconVal {
    bits: u32,
}

impl HconVal {
    /// Create from raw bits
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
        Self { bits: 0x03444cc3 }
    }

    /// Read **NUM_CLK_DIV** `[25:24]`
    ///
    /// Have 4 clk divider in design .
    #[inline(always)]
    pub const fn num_clk_div(&self) -> u32 {
        (self.bits & Hcon::NUM_CLK_DIV_MASK) >> Hcon::NUM_CLK_DIV_OFFSET
    }

    /// Read **HOLD** `[22:22]`
    ///
    /// Have a hold regiser in data path .
    #[inline(always)]
    pub const fn hold(&self) -> bool {
        (self.bits & Hcon::HOLD_MASK) != 0
    }

    /// Read **RAM_INDISE** `[21:21]`
    ///
    /// Inside RAM in SDMMC module.
    #[inline(always)]
    pub const fn ram_indise(&self) -> bool {
        (self.bits & Hcon::RAM_INDISE_MASK) != 0
    }

    /// Read **DMA_WIDTH** `[20:18]`
    ///
    /// DMA data witdth is 32.
    #[inline(always)]
    pub const fn dma_width(&self) -> u32 {
        (self.bits & Hcon::DMA_WIDTH_MASK) >> Hcon::DMA_WIDTH_OFFSET
    }

    /// Read **ADDR_WIDTH** `[15:10]`
    ///
    /// Register address width is 32.
    #[inline(always)]
    pub const fn addr_width(&self) -> u32 {
        (self.bits & Hcon::ADDR_WIDTH_MASK) >> Hcon::ADDR_WIDTH_OFFSET
    }

    /// Read **DATA_WIDTH** `[9:7]`
    ///
    /// Regisger data widht is 32.
    #[inline(always)]
    pub const fn data_width(&self) -> u32 {
        (self.bits & Hcon::DATA_WIDTH_MASK) >> Hcon::DATA_WIDTH_OFFSET
    }

    /// Read **BUS_TYPE** `[6:6]`
    ///
    /// Register config is APB bus.
    #[inline(always)]
    pub const fn bus_type(&self) -> bool {
        (self.bits & Hcon::BUS_TYPE_MASK) != 0
    }

    /// Read **CARD_NUM** `[5:1]`
    ///
    /// Support card number is 2.
    #[inline(always)]
    pub const fn card_num(&self) -> u32 {
        (self.bits & Hcon::CARD_NUM_MASK) >> Hcon::CARD_NUM_OFFSET
    }

    /// Read **CARD_TYPE** `[0:0]`
    ///
    /// Hardware support SDIO and MMC.
    #[inline(always)]
    pub const fn card_type(&self) -> bool {
        (self.bits & Hcon::CARD_TYPE_MASK) != 0
    }
}

impl Default for HconVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Hcon {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HconVal {
        HconVal::from_bits(self.value.read_bits())
    }
}

/// UHS Register
///
/// UHS-1 register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **DDR** `[17:16]` (RW)
///
///   DDR mode selecton,1 bit for each card.
///   0-Non-DDR mdoe.
///   1-DDR mdoe.
#[repr(transparent)]
pub struct Uhs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uhs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uhs {
    // ======== Field Constants ========

    /// **DDR**
    ///
    /// DDR mode selecton,1 bit for each card.
    /// 0-Non-DDR mdoe.
    /// 1-DDR mdoe.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const DDR_OFFSET: u32 = 16;
    pub const DDR_MASK: u32 = 0x00030000;

    // ======== Field Accessors ========

    /// **DDR** `[17:16]` (RW)
    ///
    /// DDR mode selecton,1 bit for each card.
    /// 0-Non-DDR mdoe.
    /// 1-DDR mdoe.
    #[inline(always)]
    pub fn ddr(&self) -> u32 {
        (self.value.read_bits() & Self::DDR_MASK) >> Self::DDR_OFFSET
    }

    /// Write **DDR** `[17:16]` (RW)
    ///
    /// DDR mode selecton,1 bit for each card.
    /// 0-Non-DDR mdoe.
    /// 1-DDR mdoe.
    #[inline(always)]
    pub fn write_ddr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DDR_MASK) | ((value << Self::DDR_OFFSET) & Self::DDR_MASK)
        });
    }
}

/// UHS value type for batch read/modify/write operations
///
/// Use `uhs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uhs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UhsVal {
    bits: u32,
}

impl UhsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DDR** `[17:16]`
    ///
    /// DDR mode selecton,1 bit for each card.
    /// 0-Non-DDR mdoe.
    /// 1-DDR mdoe.
    #[inline(always)]
    pub const fn ddr(&self) -> u32 {
        (self.bits & Uhs::DDR_MASK) >> Uhs::DDR_OFFSET
    }

    /// Set **DDR** `[17:16]`
    ///
    /// DDR mode selecton,1 bit for each card.
    /// 0-Non-DDR mdoe.
    /// 1-DDR mdoe.
    #[inline(always)]
    pub const fn with_ddr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uhs::DDR_MASK) | ((value << Uhs::DDR_OFFSET) & Uhs::DDR_MASK),
        }
    }
}

impl Default for UhsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uhs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UhsVal {
        UhsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UhsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UhsVal) -> UhsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RST_N Register
///
/// Card reset register
///
/// Address offset: 0x0078
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CARD_RESET** `[1:0]` (RW)
///
///   Hardware reset.
///   1: Active mode;
///   0: Reset.
///   These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
#[repr(transparent)]
pub struct RstN {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RstN {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RstN {
    // ======== Field Constants ========

    /// **CARD_RESET**
    ///
    /// Hardware reset.
    /// 1: Active mode;
    /// 0: Reset.
    /// These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CARD_RESET_OFFSET: u32 = 0;
    pub const CARD_RESET_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CARD_RESET** `[1:0]` (RW)
    ///
    /// Hardware reset.
    /// 1: Active mode;
    /// 0: Reset.
    /// These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
    #[inline(always)]
    pub fn card_reset(&self) -> u32 {
        (self.value.read_bits() & Self::CARD_RESET_MASK) >> Self::CARD_RESET_OFFSET
    }

    /// Write **CARD_RESET** `[1:0]` (RW)
    ///
    /// Hardware reset.
    /// 1: Active mode;
    /// 0: Reset.
    /// These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
    #[inline(always)]
    pub fn write_card_reset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARD_RESET_MASK)
                | ((value << Self::CARD_RESET_OFFSET) & Self::CARD_RESET_MASK)
        });
    }
}

/// RST_N value type for batch read/modify/write operations
///
/// Use `rstn.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rstn.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RstNVal {
    bits: u32,
}

impl RstNVal {
    /// Create from raw bits
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

    /// Read **CARD_RESET** `[1:0]`
    ///
    /// Hardware reset.
    /// 1: Active mode;
    /// 0: Reset.
    /// These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
    #[inline(always)]
    pub const fn card_reset(&self) -> u32 {
        (self.bits & RstN::CARD_RESET_MASK) >> RstN::CARD_RESET_OFFSET
    }

    /// Set **CARD_RESET** `[1:0]`
    ///
    /// Hardware reset.
    /// 1: Active mode;
    /// 0: Reset.
    /// These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET[0] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET[1] should be set to 1'b0 to reset card1.
    #[inline(always)]
    pub const fn with_card_reset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RstN::CARD_RESET_MASK)
                | ((value << RstN::CARD_RESET_OFFSET) & RstN::CARD_RESET_MASK),
        }
    }
}

impl Default for RstNVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RstN {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RstNVal {
        RstNVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RstNVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RstNVal) -> RstNVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BMOD Register
///
/// Burst mode transfer configuration register
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **PBL** `[10:8]` (RW)
///
///   Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
///   000: 1-byte transfer;
///   001: 4-byte transfer;
///   010: 8-byte transfer;
///   011: 16-byte transfer;
///   100: 32-byte transfer;
///   101: 64-byte transfer;
///   110: 128-byte transfer;
///   111: 256-byte transfer.
///   PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
///
/// - **DE** `[7]` (RW)
///
///   IDMAC Enable. When set, the IDMAC is enabled.
///
/// - **FB** `[1]` (RW)
///
///   Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations.
///
/// - **SWR** `[0]` (RW)
///
///   Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle.
#[repr(transparent)]
pub struct Bmod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bmod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bmod {
    // ======== Field Constants ========

    /// **PBL**
    ///
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    /// PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const PBL_OFFSET: u32 = 8;
    pub const PBL_MASK: u32 = 0x00000700;

    /// **DE**
    ///
    /// IDMAC Enable. When set, the IDMAC is enabled.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DE_OFFSET: u32 = 7;
    pub const DE_MASK: u32 = 0x00000080;
    pub const DE: u32 = 1 << 7;

    /// **FB**
    ///
    /// Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FB_OFFSET: u32 = 1;
    pub const FB_MASK: u32 = 0x00000002;
    pub const FB: u32 = 1 << 1;

    /// **SWR**
    ///
    /// Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SWR_OFFSET: u32 = 0;
    pub const SWR_MASK: u32 = 0x00000001;
    pub const SWR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PBL** `[10:8]` (RW)
    ///
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    /// PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
    #[inline(always)]
    pub fn pbl(&self) -> u32 {
        (self.value.read_bits() & Self::PBL_MASK) >> Self::PBL_OFFSET
    }

    /// Write **PBL** `[10:8]` (RW)
    ///
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    /// PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
    #[inline(always)]
    pub fn write_pbl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PBL_MASK) | ((value << Self::PBL_OFFSET) & Self::PBL_MASK)
        });
    }

    /// **DE** `[7]` (RW)
    ///
    /// IDMAC Enable. When set, the IDMAC is enabled.
    #[inline(always)]
    pub fn de(&self) -> bool {
        (self.value.read_bits() & Self::DE_MASK) != 0
    }

    /// Set DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_de(&self) {
        self.value.modify_bits(|r| r | Self::DE_MASK);
    }

    /// Clear DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_de(&self) {
        self.value.modify_bits(|r| r & !Self::DE_MASK);
    }

    /// **FB** `[1]` (RW)
    ///
    /// Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations.
    #[inline(always)]
    pub fn fb(&self) -> bool {
        (self.value.read_bits() & Self::FB_MASK) != 0
    }

    /// Set FB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fb(&self) {
        self.value.modify_bits(|r| r | Self::FB_MASK);
    }

    /// Clear FB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fb(&self) {
        self.value.modify_bits(|r| r & !Self::FB_MASK);
    }

    /// **SWR** `[0]` (RW)
    ///
    /// Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle.
    #[inline(always)]
    pub fn swr(&self) -> bool {
        (self.value.read_bits() & Self::SWR_MASK) != 0
    }

    /// Set SWR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swr(&self) {
        self.value.modify_bits(|r| r | Self::SWR_MASK);
    }

    /// Clear SWR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swr(&self) {
        self.value.modify_bits(|r| r & !Self::SWR_MASK);
    }
}

/// BMOD value type for batch read/modify/write operations
///
/// Use `bmod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bmod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BmodVal {
    bits: u32,
}

impl BmodVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PBL** `[10:8]`
    ///
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    /// PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
    #[inline(always)]
    pub const fn pbl(&self) -> u32 {
        (self.bits & Bmod::PBL_MASK) >> Bmod::PBL_OFFSET
    }

    /// Read **DE** `[7:7]`
    ///
    /// IDMAC Enable. When set, the IDMAC is enabled.
    #[inline(always)]
    pub const fn de(&self) -> bool {
        (self.bits & Bmod::DE_MASK) != 0
    }

    /// Read **FB** `[1:1]`
    ///
    /// Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations.
    #[inline(always)]
    pub const fn fb(&self) -> bool {
        (self.bits & Bmod::FB_MASK) != 0
    }

    /// Read **SWR** `[0:0]`
    ///
    /// Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle.
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        (self.bits & Bmod::SWR_MASK) != 0
    }

    /// Set **PBL** `[10:8]`
    ///
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows:
    /// 000: 1-byte transfer;
    /// 001: 4-byte transfer;
    /// 010: 8-byte transfer;
    /// 011: 16-byte transfer;
    /// 100: 32-byte transfer;
    /// 101: 64-byte transfer;
    /// 110: 128-byte transfer;
    /// 111: 256-byte transfer.
    /// PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access.
    #[inline(always)]
    pub const fn with_pbl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bmod::PBL_MASK) | ((value << Bmod::PBL_OFFSET) & Bmod::PBL_MASK),
        }
    }

    /// Set **DE** `[7:7]`
    ///
    /// IDMAC Enable. When set, the IDMAC is enabled.
    #[inline(always)]
    pub const fn with_de(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bmod::DE_MASK
            } else {
                self.bits & !Bmod::DE_MASK
            },
        }
    }

    /// Set **FB** `[1:1]`
    ///
    /// Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations.
    #[inline(always)]
    pub const fn with_fb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bmod::FB_MASK
            } else {
                self.bits & !Bmod::FB_MASK
            },
        }
    }

    /// Set **SWR** `[0:0]`
    ///
    /// Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle.
    #[inline(always)]
    pub const fn with_swr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bmod::SWR_MASK
            } else {
                self.bits & !Bmod::SWR_MASK
            },
        }
    }
}

impl Default for BmodVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bmod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BmodVal {
        BmodVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BmodVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BmodVal) -> BmodVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PLDMND Register
///
/// Poll demand configuration register
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **PD** `[31:0]` (WO)
///
///   Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only .
#[repr(transparent)]
pub struct Pldmnd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pldmnd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pldmnd {
    // ======== Field Constants ========

    /// **PD**
    ///
    /// Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only .
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PD_OFFSET: u32 = 0;
    pub const PD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **PD** `[31:0]` (WO)
    ///
    /// Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only .
    #[inline(always)]
    pub fn write_pd(&self, value: u32) {
        self.value
            .write_bits((value << Self::PD_OFFSET) & Self::PD_MASK);
    }
}

/// PLDMND value type for batch read/modify/write operations
///
/// Use `pldmnd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pldmnd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PldmndVal {
    bits: u32,
}

impl PldmndVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **PD** `[31:0]`
    ///
    /// Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only .
    #[inline(always)]
    pub const fn with_pd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pldmnd::PD_MASK) | ((value << Pldmnd::PD_OFFSET) & Pldmnd::PD_MASK),
        }
    }
}

impl Default for PldmndVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pldmnd {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PldmndVal) {
        self.value.write_bits(val.bits())
    }
}

/// DBADDR Register
///
/// Descriptor base address register
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **DBADDR** `[31:0]` (RW)
///
///   Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
#[repr(transparent)]
pub struct Dbaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dbaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dbaddr {
    // ======== Field Constants ========

    /// **DBADDR**
    ///
    /// Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DBADDR_OFFSET: u32 = 0;
    pub const DBADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DBADDR** `[31:0]` (RW)
    ///
    /// Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
    #[inline(always)]
    pub fn dbaddr(&self) -> u32 {
        (self.value.read_bits() & Self::DBADDR_MASK) >> Self::DBADDR_OFFSET
    }

    /// Write **DBADDR** `[31:0]` (RW)
    ///
    /// Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
    #[inline(always)]
    pub fn write_dbaddr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBADDR_MASK) | ((value << Self::DBADDR_OFFSET) & Self::DBADDR_MASK)
        });
    }
}

/// DBADDR value type for batch read/modify/write operations
///
/// Use `dbaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbaddrVal {
    bits: u32,
}

impl DbaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBADDR** `[31:0]`
    ///
    /// Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
    #[inline(always)]
    pub const fn dbaddr(&self) -> u32 {
        (self.bits & Dbaddr::DBADDR_MASK) >> Dbaddr::DBADDR_OFFSET
    }

    /// Set **DBADDR** `[31:0]`
    ///
    /// Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits [1:0] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only.
    #[inline(always)]
    pub const fn with_dbaddr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dbaddr::DBADDR_MASK)
                | ((value << Dbaddr::DBADDR_OFFSET) & Dbaddr::DBADDR_MASK),
        }
    }
}

impl Default for DbaddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dbaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbaddrVal {
        DbaddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbaddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbaddrVal) -> DbaddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDSTS Register
///
/// IDMAC status register
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **FSM** `[16:13]` (RW)
///
///   DMAC FSM present state.
///   0: DMA_IDLE (idle state);
///   1: DMA_SUSPEND (suspend state);
///   2: DESC_RD (descriptor reading state);
///   3: DESC_CHK (descriptor checking state);
///   4: DMA_RD_REQ_WAIT (read-data request waiting state);
///   5: DMA_WR_REQ_WAIT (write-data request waiting state);
///   6: DMA_RD (data-read state);
///   7: DMA_WR (data-write state);
///   8: DESC_CLOSE (descriptor close state).
///
/// - **FBE_CODE** `[12:10]` (RW)
///
///   Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
///   001: Host Abort received during transmission;
///   010: Host Abort received during reception;
///   Others: Reserved.
///
/// - **AIS** `[9]` (RW)
///
///   Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] : Fatal Bus Interrupt, IDSTS[4] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit.
///
/// - **NIS** `[8]` (RW)
///
///   Normal Interrupt Summary. Logical OR of the following: IDSTS[0] : Transmit Interrupt, IDSTS[1] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit.
///
/// - **CES** `[5]` (RW)
///
///   Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits:
///   EBE : End Bit Error;
///   RTO : Response Timeout/Boot Ack Timeout;
///   RCRC : Response CRC;
///   SBE : Start Bit Error;
///   DRTO : Data Read Timeout/BDS timeout;
///   DCRC : Data CRC for Receive;
///   RE : Response Error.
///   Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error.
///
/// - **DU** `[4]` (RW)
///
///   Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0[31] = 0). Writing 1 clears this bit.
///
/// - **FBE** `[2]` (RW)
///
///   Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit.
///
/// - **RI** `[1]` (RW)
///
///   Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit.
///
/// - **TI** `[0]` (RW)
///
///   Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit.
#[repr(transparent)]
pub struct Idsts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idsts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idsts {
    // ======== Field Constants ========

    /// **FSM**
    ///
    /// DMAC FSM present state.
    /// 0: DMA_IDLE (idle state);
    /// 1: DMA_SUSPEND (suspend state);
    /// 2: DESC_RD (descriptor reading state);
    /// 3: DESC_CHK (descriptor checking state);
    /// 4: DMA_RD_REQ_WAIT (read-data request waiting state);
    /// 5: DMA_WR_REQ_WAIT (write-data request waiting state);
    /// 6: DMA_RD (data-read state);
    /// 7: DMA_WR (data-write state);
    /// 8: DESC_CLOSE (descriptor close state).
    ///
    /// Bits `[16:13]`, mask `0x0001e000`
    pub const FSM_OFFSET: u32 = 13;
    pub const FSM_MASK: u32 = 0x0001e000;

    /// **FBE_CODE**
    ///
    /// Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
    /// 001: Host Abort received during transmission;
    /// 010: Host Abort received during reception;
    /// Others: Reserved.
    ///
    /// Bits `[12:10]`, mask `0x00001c00`
    pub const FBE_CODE_OFFSET: u32 = 10;
    pub const FBE_CODE_MASK: u32 = 0x00001c00;

    /// **AIS**
    ///
    /// Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] : Fatal Bus Interrupt, IDSTS[4] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const AIS_OFFSET: u32 = 9;
    pub const AIS_MASK: u32 = 0x00000200;
    pub const AIS: u32 = 1 << 9;

    /// **NIS**
    ///
    /// Normal Interrupt Summary. Logical OR of the following: IDSTS[0] : Transmit Interrupt, IDSTS[1] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const NIS_OFFSET: u32 = 8;
    pub const NIS_MASK: u32 = 0x00000100;
    pub const NIS: u32 = 1 << 8;

    /// **CES**
    ///
    /// Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits:
    /// EBE : End Bit Error;
    /// RTO : Response Timeout/Boot Ack Timeout;
    /// RCRC : Response CRC;
    /// SBE : Start Bit Error;
    /// DRTO : Data Read Timeout/BDS timeout;
    /// DCRC : Data CRC for Receive;
    /// RE : Response Error.
    /// Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CES_OFFSET: u32 = 5;
    pub const CES_MASK: u32 = 0x00000020;
    pub const CES: u32 = 1 << 5;

    /// **DU**
    ///
    /// Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0[31] = 0). Writing 1 clears this bit.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DU_OFFSET: u32 = 4;
    pub const DU_MASK: u32 = 0x00000010;
    pub const DU: u32 = 1 << 4;

    /// **FBE**
    ///
    /// Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FBE_OFFSET: u32 = 2;
    pub const FBE_MASK: u32 = 0x00000004;
    pub const FBE: u32 = 1 << 2;

    /// **RI**
    ///
    /// Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RI_OFFSET: u32 = 1;
    pub const RI_MASK: u32 = 0x00000002;
    pub const RI: u32 = 1 << 1;

    /// **TI**
    ///
    /// Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TI_OFFSET: u32 = 0;
    pub const TI_MASK: u32 = 0x00000001;
    pub const TI: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FSM** `[16:13]` (RW)
    ///
    /// DMAC FSM present state.
    /// 0: DMA_IDLE (idle state);
    /// 1: DMA_SUSPEND (suspend state);
    /// 2: DESC_RD (descriptor reading state);
    /// 3: DESC_CHK (descriptor checking state);
    /// 4: DMA_RD_REQ_WAIT (read-data request waiting state);
    /// 5: DMA_WR_REQ_WAIT (write-data request waiting state);
    /// 6: DMA_RD (data-read state);
    /// 7: DMA_WR (data-write state);
    /// 8: DESC_CLOSE (descriptor close state).
    #[inline(always)]
    pub fn fsm(&self) -> u32 {
        (self.value.read_bits() & Self::FSM_MASK) >> Self::FSM_OFFSET
    }

    /// Write **FSM** `[16:13]` (RW)
    ///
    /// DMAC FSM present state.
    /// 0: DMA_IDLE (idle state);
    /// 1: DMA_SUSPEND (suspend state);
    /// 2: DESC_RD (descriptor reading state);
    /// 3: DESC_CHK (descriptor checking state);
    /// 4: DMA_RD_REQ_WAIT (read-data request waiting state);
    /// 5: DMA_WR_REQ_WAIT (write-data request waiting state);
    /// 6: DMA_RD (data-read state);
    /// 7: DMA_WR (data-write state);
    /// 8: DESC_CLOSE (descriptor close state).
    #[inline(always)]
    pub fn write_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FSM_MASK) | ((value << Self::FSM_OFFSET) & Self::FSM_MASK)
        });
    }

    /// **FBE_CODE** `[12:10]` (RW)
    ///
    /// Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
    /// 001: Host Abort received during transmission;
    /// 010: Host Abort received during reception;
    /// Others: Reserved.
    #[inline(always)]
    pub fn fbe_code(&self) -> u32 {
        (self.value.read_bits() & Self::FBE_CODE_MASK) >> Self::FBE_CODE_OFFSET
    }

    /// Write **FBE_CODE** `[12:10]` (RW)
    ///
    /// Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
    /// 001: Host Abort received during transmission;
    /// 010: Host Abort received during reception;
    /// Others: Reserved.
    #[inline(always)]
    pub fn write_fbe_code(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FBE_CODE_MASK) | ((value << Self::FBE_CODE_OFFSET) & Self::FBE_CODE_MASK)
        });
    }

    /// **AIS** `[9]` (RW)
    ///
    /// Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] : Fatal Bus Interrupt, IDSTS[4] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub fn ais(&self) -> bool {
        (self.value.read_bits() & Self::AIS_MASK) != 0
    }

    /// Set AIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ais(&self) {
        self.value.modify_bits(|r| r | Self::AIS_MASK);
    }

    /// Clear AIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ais(&self) {
        self.value.modify_bits(|r| r & !Self::AIS_MASK);
    }

    /// **NIS** `[8]` (RW)
    ///
    /// Normal Interrupt Summary. Logical OR of the following: IDSTS[0] : Transmit Interrupt, IDSTS[1] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub fn nis(&self) -> bool {
        (self.value.read_bits() & Self::NIS_MASK) != 0
    }

    /// Set NIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nis(&self) {
        self.value.modify_bits(|r| r | Self::NIS_MASK);
    }

    /// Clear NIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nis(&self) {
        self.value.modify_bits(|r| r & !Self::NIS_MASK);
    }

    /// **CES** `[5]` (RW)
    ///
    /// Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits:
    /// EBE : End Bit Error;
    /// RTO : Response Timeout/Boot Ack Timeout;
    /// RCRC : Response CRC;
    /// SBE : Start Bit Error;
    /// DRTO : Data Read Timeout/BDS timeout;
    /// DCRC : Data CRC for Receive;
    /// RE : Response Error.
    /// Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error.
    #[inline(always)]
    pub fn ces(&self) -> bool {
        (self.value.read_bits() & Self::CES_MASK) != 0
    }

    /// Set CES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ces(&self) {
        self.value.modify_bits(|r| r | Self::CES_MASK);
    }

    /// Clear CES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ces(&self) {
        self.value.modify_bits(|r| r & !Self::CES_MASK);
    }

    /// **DU** `[4]` (RW)
    ///
    /// Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0[31] = 0). Writing 1 clears this bit.
    #[inline(always)]
    pub fn du(&self) -> bool {
        (self.value.read_bits() & Self::DU_MASK) != 0
    }

    /// Set DU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_du(&self) {
        self.value.modify_bits(|r| r | Self::DU_MASK);
    }

    /// Clear DU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_du(&self) {
        self.value.modify_bits(|r| r & !Self::DU_MASK);
    }

    /// **FBE** `[2]` (RW)
    ///
    /// Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit.
    #[inline(always)]
    pub fn fbe(&self) -> bool {
        (self.value.read_bits() & Self::FBE_MASK) != 0
    }

    /// Set FBE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fbe(&self) {
        self.value.modify_bits(|r| r | Self::FBE_MASK);
    }

    /// Clear FBE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fbe(&self) {
        self.value.modify_bits(|r| r & !Self::FBE_MASK);
    }

    /// **RI** `[1]` (RW)
    ///
    /// Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub fn ri(&self) -> bool {
        (self.value.read_bits() & Self::RI_MASK) != 0
    }

    /// Set RI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ri(&self) {
        self.value.modify_bits(|r| r | Self::RI_MASK);
    }

    /// Clear RI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ri(&self) {
        self.value.modify_bits(|r| r & !Self::RI_MASK);
    }

    /// **TI** `[0]` (RW)
    ///
    /// Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub fn ti(&self) -> bool {
        (self.value.read_bits() & Self::TI_MASK) != 0
    }

    /// Set TI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ti(&self) {
        self.value.modify_bits(|r| r | Self::TI_MASK);
    }

    /// Clear TI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ti(&self) {
        self.value.modify_bits(|r| r & !Self::TI_MASK);
    }
}

/// IDSTS value type for batch read/modify/write operations
///
/// Use `idsts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idsts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdstsVal {
    bits: u32,
}

impl IdstsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FSM** `[16:13]`
    ///
    /// DMAC FSM present state.
    /// 0: DMA_IDLE (idle state);
    /// 1: DMA_SUSPEND (suspend state);
    /// 2: DESC_RD (descriptor reading state);
    /// 3: DESC_CHK (descriptor checking state);
    /// 4: DMA_RD_REQ_WAIT (read-data request waiting state);
    /// 5: DMA_WR_REQ_WAIT (write-data request waiting state);
    /// 6: DMA_RD (data-read state);
    /// 7: DMA_WR (data-write state);
    /// 8: DESC_CLOSE (descriptor close state).
    #[inline(always)]
    pub const fn fsm(&self) -> u32 {
        (self.bits & Idsts::FSM_MASK) >> Idsts::FSM_OFFSET
    }

    /// Read **FBE_CODE** `[12:10]`
    ///
    /// Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
    /// 001: Host Abort received during transmission;
    /// 010: Host Abort received during reception;
    /// Others: Reserved.
    #[inline(always)]
    pub const fn fbe_code(&self) -> u32 {
        (self.bits & Idsts::FBE_CODE_MASK) >> Idsts::FBE_CODE_OFFSET
    }

    /// Read **AIS** `[9:9]`
    ///
    /// Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] : Fatal Bus Interrupt, IDSTS[4] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        (self.bits & Idsts::AIS_MASK) != 0
    }

    /// Read **NIS** `[8:8]`
    ///
    /// Normal Interrupt Summary. Logical OR of the following: IDSTS[0] : Transmit Interrupt, IDSTS[1] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        (self.bits & Idsts::NIS_MASK) != 0
    }

    /// Read **CES** `[5:5]`
    ///
    /// Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits:
    /// EBE : End Bit Error;
    /// RTO : Response Timeout/Boot Ack Timeout;
    /// RCRC : Response CRC;
    /// SBE : Start Bit Error;
    /// DRTO : Data Read Timeout/BDS timeout;
    /// DCRC : Data CRC for Receive;
    /// RE : Response Error.
    /// Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error.
    #[inline(always)]
    pub const fn ces(&self) -> bool {
        (self.bits & Idsts::CES_MASK) != 0
    }

    /// Read **DU** `[4:4]`
    ///
    /// Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0[31] = 0). Writing 1 clears this bit.
    #[inline(always)]
    pub const fn du(&self) -> bool {
        (self.bits & Idsts::DU_MASK) != 0
    }

    /// Read **FBE** `[2:2]`
    ///
    /// Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        (self.bits & Idsts::FBE_MASK) != 0
    }

    /// Read **RI** `[1:1]`
    ///
    /// Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        (self.bits & Idsts::RI_MASK) != 0
    }

    /// Read **TI** `[0:0]`
    ///
    /// Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        (self.bits & Idsts::TI_MASK) != 0
    }

    /// Set **FSM** `[16:13]`
    ///
    /// DMAC FSM present state.
    /// 0: DMA_IDLE (idle state);
    /// 1: DMA_SUSPEND (suspend state);
    /// 2: DESC_RD (descriptor reading state);
    /// 3: DESC_CHK (descriptor checking state);
    /// 4: DMA_RD_REQ_WAIT (read-data request waiting state);
    /// 5: DMA_WR_REQ_WAIT (write-data request waiting state);
    /// 6: DMA_RD (data-read state);
    /// 7: DMA_WR (data-write state);
    /// 8: DESC_CLOSE (descriptor close state).
    #[inline(always)]
    pub const fn with_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idsts::FSM_MASK) | ((value << Idsts::FSM_OFFSET) & Idsts::FSM_MASK),
        }
    }

    /// Set **FBE_CODE** `[12:10]`
    ///
    /// Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS[2] is set. This field does not generate an interrupt.
    /// 001: Host Abort received during transmission;
    /// 010: Host Abort received during reception;
    /// Others: Reserved.
    #[inline(always)]
    pub const fn with_fbe_code(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idsts::FBE_CODE_MASK)
                | ((value << Idsts::FBE_CODE_OFFSET) & Idsts::FBE_CODE_MASK),
        }
    }

    /// Set **AIS** `[9:9]`
    ///
    /// Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] : Fatal Bus Interrupt, IDSTS[4] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_ais(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::AIS_MASK
            } else {
                self.bits & !Idsts::AIS_MASK
            },
        }
    }

    /// Set **NIS** `[8:8]`
    ///
    /// Normal Interrupt Summary. Logical OR of the following: IDSTS[0] : Transmit Interrupt, IDSTS[1] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_nis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::NIS_MASK
            } else {
                self.bits & !Idsts::NIS_MASK
            },
        }
    }

    /// Set **CES** `[5:5]`
    ///
    /// Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits:
    /// EBE : End Bit Error;
    /// RTO : Response Timeout/Boot Ack Timeout;
    /// RCRC : Response CRC;
    /// SBE : Start Bit Error;
    /// DRTO : Data Read Timeout/BDS timeout;
    /// DCRC : Data CRC for Receive;
    /// RE : Response Error.
    /// Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error.
    #[inline(always)]
    pub const fn with_ces(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::CES_MASK
            } else {
                self.bits & !Idsts::CES_MASK
            },
        }
    }

    /// Set **DU** `[4:4]`
    ///
    /// Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0[31] = 0). Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_du(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::DU_MASK
            } else {
                self.bits & !Idsts::DU_MASK
            },
        }
    }

    /// Set **FBE** `[2:2]`
    ///
    /// Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_fbe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::FBE_MASK
            } else {
                self.bits & !Idsts::FBE_MASK
            },
        }
    }

    /// Set **RI** `[1:1]`
    ///
    /// Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_ri(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::RI_MASK
            } else {
                self.bits & !Idsts::RI_MASK
            },
        }
    }

    /// Set **TI** `[0:0]`
    ///
    /// Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit.
    #[inline(always)]
    pub const fn with_ti(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idsts::TI_MASK
            } else {
                self.bits & !Idsts::TI_MASK
            },
        }
    }
}

impl Default for IdstsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Idsts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdstsVal {
        IdstsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdstsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdstsVal) -> IdstsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDINTEN Register
///
/// IDMAC interrupt enable register
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **AI** `[9]` (RW)
///
///   Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits:
///   IDINTEN[2]: Fatal Bus Error Interrupt;
///   IDINTEN[4]: DU Interrupt.
///
/// - **NI** `[8]` (RW)
///
///   Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits:
///   IDINTEN[0]: Transmit Interrupt;
///   IDINTEN[1]: Receive Interrupt.
///
/// - **CES** `[5]` (RW)
///
///   Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary.
///
/// - **DU** `[4]` (RW)
///
///   Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled.
///
/// - **FBE** `[2]` (RW)
///
///   Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled.
///
/// - **RI** `[1]` (RW)
///
///   Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled.
///
/// - **TI** `[0]` (RW)
///
///   Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled.
#[repr(transparent)]
pub struct Idinten {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idinten {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idinten {
    // ======== Field Constants ========

    /// **AI**
    ///
    /// Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits:
    /// IDINTEN[2]: Fatal Bus Error Interrupt;
    /// IDINTEN[4]: DU Interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const AI_OFFSET: u32 = 9;
    pub const AI_MASK: u32 = 0x00000200;
    pub const AI: u32 = 1 << 9;

    /// **NI**
    ///
    /// Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits:
    /// IDINTEN[0]: Transmit Interrupt;
    /// IDINTEN[1]: Receive Interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const NI_OFFSET: u32 = 8;
    pub const NI_MASK: u32 = 0x00000100;
    pub const NI: u32 = 1 << 8;

    /// **CES**
    ///
    /// Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CES_OFFSET: u32 = 5;
    pub const CES_MASK: u32 = 0x00000020;
    pub const CES: u32 = 1 << 5;

    /// **DU**
    ///
    /// Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DU_OFFSET: u32 = 4;
    pub const DU_MASK: u32 = 0x00000010;
    pub const DU: u32 = 1 << 4;

    /// **FBE**
    ///
    /// Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FBE_OFFSET: u32 = 2;
    pub const FBE_MASK: u32 = 0x00000004;
    pub const FBE: u32 = 1 << 2;

    /// **RI**
    ///
    /// Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RI_OFFSET: u32 = 1;
    pub const RI_MASK: u32 = 0x00000002;
    pub const RI: u32 = 1 << 1;

    /// **TI**
    ///
    /// Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TI_OFFSET: u32 = 0;
    pub const TI_MASK: u32 = 0x00000001;
    pub const TI: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **AI** `[9]` (RW)
    ///
    /// Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits:
    /// IDINTEN[2]: Fatal Bus Error Interrupt;
    /// IDINTEN[4]: DU Interrupt.
    #[inline(always)]
    pub fn ai(&self) -> bool {
        (self.value.read_bits() & Self::AI_MASK) != 0
    }

    /// Set AI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ai(&self) {
        self.value.modify_bits(|r| r | Self::AI_MASK);
    }

    /// Clear AI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ai(&self) {
        self.value.modify_bits(|r| r & !Self::AI_MASK);
    }

    /// **NI** `[8]` (RW)
    ///
    /// Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits:
    /// IDINTEN[0]: Transmit Interrupt;
    /// IDINTEN[1]: Receive Interrupt.
    #[inline(always)]
    pub fn ni(&self) -> bool {
        (self.value.read_bits() & Self::NI_MASK) != 0
    }

    /// Set NI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ni(&self) {
        self.value.modify_bits(|r| r | Self::NI_MASK);
    }

    /// Clear NI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ni(&self) {
        self.value.modify_bits(|r| r & !Self::NI_MASK);
    }

    /// **CES** `[5]` (RW)
    ///
    /// Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary.
    #[inline(always)]
    pub fn ces(&self) -> bool {
        (self.value.read_bits() & Self::CES_MASK) != 0
    }

    /// Set CES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ces(&self) {
        self.value.modify_bits(|r| r | Self::CES_MASK);
    }

    /// Clear CES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ces(&self) {
        self.value.modify_bits(|r| r & !Self::CES_MASK);
    }

    /// **DU** `[4]` (RW)
    ///
    /// Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled.
    #[inline(always)]
    pub fn du(&self) -> bool {
        (self.value.read_bits() & Self::DU_MASK) != 0
    }

    /// Set DU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_du(&self) {
        self.value.modify_bits(|r| r | Self::DU_MASK);
    }

    /// Clear DU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_du(&self) {
        self.value.modify_bits(|r| r & !Self::DU_MASK);
    }

    /// **FBE** `[2]` (RW)
    ///
    /// Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled.
    #[inline(always)]
    pub fn fbe(&self) -> bool {
        (self.value.read_bits() & Self::FBE_MASK) != 0
    }

    /// Set FBE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fbe(&self) {
        self.value.modify_bits(|r| r | Self::FBE_MASK);
    }

    /// Clear FBE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fbe(&self) {
        self.value.modify_bits(|r| r & !Self::FBE_MASK);
    }

    /// **RI** `[1]` (RW)
    ///
    /// Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled.
    #[inline(always)]
    pub fn ri(&self) -> bool {
        (self.value.read_bits() & Self::RI_MASK) != 0
    }

    /// Set RI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ri(&self) {
        self.value.modify_bits(|r| r | Self::RI_MASK);
    }

    /// Clear RI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ri(&self) {
        self.value.modify_bits(|r| r & !Self::RI_MASK);
    }

    /// **TI** `[0]` (RW)
    ///
    /// Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled.
    #[inline(always)]
    pub fn ti(&self) -> bool {
        (self.value.read_bits() & Self::TI_MASK) != 0
    }

    /// Set TI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ti(&self) {
        self.value.modify_bits(|r| r | Self::TI_MASK);
    }

    /// Clear TI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ti(&self) {
        self.value.modify_bits(|r| r & !Self::TI_MASK);
    }
}

/// IDINTEN value type for batch read/modify/write operations
///
/// Use `idinten.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idinten.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdintenVal {
    bits: u32,
}

impl IdintenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **AI** `[9:9]`
    ///
    /// Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits:
    /// IDINTEN[2]: Fatal Bus Error Interrupt;
    /// IDINTEN[4]: DU Interrupt.
    #[inline(always)]
    pub const fn ai(&self) -> bool {
        (self.bits & Idinten::AI_MASK) != 0
    }

    /// Read **NI** `[8:8]`
    ///
    /// Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits:
    /// IDINTEN[0]: Transmit Interrupt;
    /// IDINTEN[1]: Receive Interrupt.
    #[inline(always)]
    pub const fn ni(&self) -> bool {
        (self.bits & Idinten::NI_MASK) != 0
    }

    /// Read **CES** `[5:5]`
    ///
    /// Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary.
    #[inline(always)]
    pub const fn ces(&self) -> bool {
        (self.bits & Idinten::CES_MASK) != 0
    }

    /// Read **DU** `[4:4]`
    ///
    /// Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled.
    #[inline(always)]
    pub const fn du(&self) -> bool {
        (self.bits & Idinten::DU_MASK) != 0
    }

    /// Read **FBE** `[2:2]`
    ///
    /// Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled.
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        (self.bits & Idinten::FBE_MASK) != 0
    }

    /// Read **RI** `[1:1]`
    ///
    /// Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled.
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        (self.bits & Idinten::RI_MASK) != 0
    }

    /// Read **TI** `[0:0]`
    ///
    /// Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled.
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        (self.bits & Idinten::TI_MASK) != 0
    }

    /// Set **AI** `[9:9]`
    ///
    /// Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits:
    /// IDINTEN[2]: Fatal Bus Error Interrupt;
    /// IDINTEN[4]: DU Interrupt.
    #[inline(always)]
    pub const fn with_ai(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::AI_MASK
            } else {
                self.bits & !Idinten::AI_MASK
            },
        }
    }

    /// Set **NI** `[8:8]`
    ///
    /// Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits:
    /// IDINTEN[0]: Transmit Interrupt;
    /// IDINTEN[1]: Receive Interrupt.
    #[inline(always)]
    pub const fn with_ni(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::NI_MASK
            } else {
                self.bits & !Idinten::NI_MASK
            },
        }
    }

    /// Set **CES** `[5:5]`
    ///
    /// Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary.
    #[inline(always)]
    pub const fn with_ces(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::CES_MASK
            } else {
                self.bits & !Idinten::CES_MASK
            },
        }
    }

    /// Set **DU** `[4:4]`
    ///
    /// Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled.
    #[inline(always)]
    pub const fn with_du(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::DU_MASK
            } else {
                self.bits & !Idinten::DU_MASK
            },
        }
    }

    /// Set **FBE** `[2:2]`
    ///
    /// Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled.
    #[inline(always)]
    pub const fn with_fbe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::FBE_MASK
            } else {
                self.bits & !Idinten::FBE_MASK
            },
        }
    }

    /// Set **RI** `[1:1]`
    ///
    /// Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled.
    #[inline(always)]
    pub const fn with_ri(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::RI_MASK
            } else {
                self.bits & !Idinten::RI_MASK
            },
        }
    }

    /// Set **TI** `[0:0]`
    ///
    /// Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled.
    #[inline(always)]
    pub const fn with_ti(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Idinten::TI_MASK
            } else {
                self.bits & !Idinten::TI_MASK
            },
        }
    }
}

impl Default for IdintenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Idinten {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdintenVal {
        IdintenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdintenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdintenVal) -> IdintenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DSCADDR Register
///
/// Host descriptor address pointer
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **DSCADDR** `[31:0]` (RO)
///
///   Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC.
#[repr(transparent)]
pub struct Dscaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dscaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dscaddr {
    // ======== Field Constants ========

    /// **DSCADDR**
    ///
    /// Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DSCADDR_OFFSET: u32 = 0;
    pub const DSCADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DSCADDR** `[31:0]` (RO)
    ///
    /// Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC.
    #[inline(always)]
    pub fn dscaddr(&self) -> u32 {
        (self.value.read_bits() & Self::DSCADDR_MASK) >> Self::DSCADDR_OFFSET
    }
}

/// DSCADDR value type for batch read/modify/write operations
///
/// Use `dscaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dscaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DscaddrVal {
    bits: u32,
}

impl DscaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DSCADDR** `[31:0]`
    ///
    /// Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC.
    #[inline(always)]
    pub const fn dscaddr(&self) -> u32 {
        (self.bits & Dscaddr::DSCADDR_MASK) >> Dscaddr::DSCADDR_OFFSET
    }
}

impl Default for DscaddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dscaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DscaddrVal {
        DscaddrVal::from_bits(self.value.read_bits())
    }
}

/// BUFADDR Register
///
/// Host buffer address pointer register
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **BUFADDR** `[31:0]` (RO)
///
///   Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC.
#[repr(transparent)]
pub struct Bufaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bufaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bufaddr {
    // ======== Field Constants ========

    /// **BUFADDR**
    ///
    /// Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BUFADDR_OFFSET: u32 = 0;
    pub const BUFADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BUFADDR** `[31:0]` (RO)
    ///
    /// Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC.
    #[inline(always)]
    pub fn bufaddr(&self) -> u32 {
        (self.value.read_bits() & Self::BUFADDR_MASK) >> Self::BUFADDR_OFFSET
    }
}

/// BUFADDR value type for batch read/modify/write operations
///
/// Use `bufaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bufaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BufaddrVal {
    bits: u32,
}

impl BufaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BUFADDR** `[31:0]`
    ///
    /// Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC.
    #[inline(always)]
    pub const fn bufaddr(&self) -> u32 {
        (self.bits & Bufaddr::BUFADDR_MASK) >> Bufaddr::BUFADDR_OFFSET
    }
}

impl Default for BufaddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bufaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BufaddrVal {
        BufaddrVal::from_bits(self.value.read_bits())
    }
}

/// CARDTHRCTL Register
///
/// Card Threshold Control register
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **CARDTHRESHOLD** `[31:16]` (RW)
///
///   The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
///
/// - **CARDWRTHREN** `[2]` (RW)
///
///   Applicable when HS400 mode is enabled.
///   1'b0-Card write Threshold disabled.
///   1'b1-Card write Threshold enabled.
///
/// - **CARDCLRINTEN** `[1]` (RW)
///
///   Busy clear interrupt generation:
///   1'b0-Busy clear interrypt disabled.
///   1'b1-Busy clear interrypt enabled.
///
/// - **CARDRDTHREN** `[0]` (RW)
///
///   Card read threshold enable.
///   1'b0-Card read threshold disabled.
///   1'b1-Card read threshold enabled.
#[repr(transparent)]
pub struct Cardthrctl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cardthrctl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cardthrctl {
    // ======== Field Constants ========

    /// **CARDTHRESHOLD**
    ///
    /// The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CARDTHRESHOLD_OFFSET: u32 = 16;
    pub const CARDTHRESHOLD_MASK: u32 = 0xffff0000;

    /// **CARDWRTHREN**
    ///
    /// Applicable when HS400 mode is enabled.
    /// 1'b0-Card write Threshold disabled.
    /// 1'b1-Card write Threshold enabled.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CARDWRTHREN_OFFSET: u32 = 2;
    pub const CARDWRTHREN_MASK: u32 = 0x00000004;
    pub const CARDWRTHREN: u32 = 1 << 2;

    /// **CARDCLRINTEN**
    ///
    /// Busy clear interrupt generation:
    /// 1'b0-Busy clear interrypt disabled.
    /// 1'b1-Busy clear interrypt enabled.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CARDCLRINTEN_OFFSET: u32 = 1;
    pub const CARDCLRINTEN_MASK: u32 = 0x00000002;
    pub const CARDCLRINTEN: u32 = 1 << 1;

    /// **CARDRDTHREN**
    ///
    /// Card read threshold enable.
    /// 1'b0-Card read threshold disabled.
    /// 1'b1-Card read threshold enabled.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CARDRDTHREN_OFFSET: u32 = 0;
    pub const CARDRDTHREN_MASK: u32 = 0x00000001;
    pub const CARDRDTHREN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CARDTHRESHOLD** `[31:16]` (RW)
    ///
    /// The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    pub fn cardthreshold(&self) -> u32 {
        (self.value.read_bits() & Self::CARDTHRESHOLD_MASK) >> Self::CARDTHRESHOLD_OFFSET
    }

    /// Write **CARDTHRESHOLD** `[31:16]` (RW)
    ///
    /// The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    pub fn write_cardthreshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CARDTHRESHOLD_MASK)
                | ((value << Self::CARDTHRESHOLD_OFFSET) & Self::CARDTHRESHOLD_MASK)
        });
    }

    /// **CARDWRTHREN** `[2]` (RW)
    ///
    /// Applicable when HS400 mode is enabled.
    /// 1'b0-Card write Threshold disabled.
    /// 1'b1-Card write Threshold enabled.
    #[inline(always)]
    pub fn cardwrthren(&self) -> bool {
        (self.value.read_bits() & Self::CARDWRTHREN_MASK) != 0
    }

    /// Set CARDWRTHREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cardwrthren(&self) {
        self.value.modify_bits(|r| r | Self::CARDWRTHREN_MASK);
    }

    /// Clear CARDWRTHREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cardwrthren(&self) {
        self.value.modify_bits(|r| r & !Self::CARDWRTHREN_MASK);
    }

    /// **CARDCLRINTEN** `[1]` (RW)
    ///
    /// Busy clear interrupt generation:
    /// 1'b0-Busy clear interrypt disabled.
    /// 1'b1-Busy clear interrypt enabled.
    #[inline(always)]
    pub fn cardclrinten(&self) -> bool {
        (self.value.read_bits() & Self::CARDCLRINTEN_MASK) != 0
    }

    /// Set CARDCLRINTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cardclrinten(&self) {
        self.value.modify_bits(|r| r | Self::CARDCLRINTEN_MASK);
    }

    /// Clear CARDCLRINTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cardclrinten(&self) {
        self.value.modify_bits(|r| r & !Self::CARDCLRINTEN_MASK);
    }

    /// **CARDRDTHREN** `[0]` (RW)
    ///
    /// Card read threshold enable.
    /// 1'b0-Card read threshold disabled.
    /// 1'b1-Card read threshold enabled.
    #[inline(always)]
    pub fn cardrdthren(&self) -> bool {
        (self.value.read_bits() & Self::CARDRDTHREN_MASK) != 0
    }

    /// Set CARDRDTHREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cardrdthren(&self) {
        self.value.modify_bits(|r| r | Self::CARDRDTHREN_MASK);
    }

    /// Clear CARDRDTHREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cardrdthren(&self) {
        self.value.modify_bits(|r| r & !Self::CARDRDTHREN_MASK);
    }
}

/// CARDTHRCTL value type for batch read/modify/write operations
///
/// Use `cardthrctl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cardthrctl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CardthrctlVal {
    bits: u32,
}

impl CardthrctlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CARDTHRESHOLD** `[31:16]`
    ///
    /// The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    pub const fn cardthreshold(&self) -> u32 {
        (self.bits & Cardthrctl::CARDTHRESHOLD_MASK) >> Cardthrctl::CARDTHRESHOLD_OFFSET
    }

    /// Read **CARDWRTHREN** `[2:2]`
    ///
    /// Applicable when HS400 mode is enabled.
    /// 1'b0-Card write Threshold disabled.
    /// 1'b1-Card write Threshold enabled.
    #[inline(always)]
    pub const fn cardwrthren(&self) -> bool {
        (self.bits & Cardthrctl::CARDWRTHREN_MASK) != 0
    }

    /// Read **CARDCLRINTEN** `[1:1]`
    ///
    /// Busy clear interrupt generation:
    /// 1'b0-Busy clear interrypt disabled.
    /// 1'b1-Busy clear interrypt enabled.
    #[inline(always)]
    pub const fn cardclrinten(&self) -> bool {
        (self.bits & Cardthrctl::CARDCLRINTEN_MASK) != 0
    }

    /// Read **CARDRDTHREN** `[0:0]`
    ///
    /// Card read threshold enable.
    /// 1'b0-Card read threshold disabled.
    /// 1'b1-Card read threshold enabled.
    #[inline(always)]
    pub const fn cardrdthren(&self) -> bool {
        (self.bits & Cardthrctl::CARDRDTHREN_MASK) != 0
    }

    /// Set **CARDTHRESHOLD** `[31:16]`
    ///
    /// The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    pub const fn with_cardthreshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cardthrctl::CARDTHRESHOLD_MASK)
                | ((value << Cardthrctl::CARDTHRESHOLD_OFFSET) & Cardthrctl::CARDTHRESHOLD_MASK),
        }
    }

    /// Set **CARDWRTHREN** `[2:2]`
    ///
    /// Applicable when HS400 mode is enabled.
    /// 1'b0-Card write Threshold disabled.
    /// 1'b1-Card write Threshold enabled.
    #[inline(always)]
    pub const fn with_cardwrthren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cardthrctl::CARDWRTHREN_MASK
            } else {
                self.bits & !Cardthrctl::CARDWRTHREN_MASK
            },
        }
    }

    /// Set **CARDCLRINTEN** `[1:1]`
    ///
    /// Busy clear interrupt generation:
    /// 1'b0-Busy clear interrypt disabled.
    /// 1'b1-Busy clear interrypt enabled.
    #[inline(always)]
    pub const fn with_cardclrinten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cardthrctl::CARDCLRINTEN_MASK
            } else {
                self.bits & !Cardthrctl::CARDCLRINTEN_MASK
            },
        }
    }

    /// Set **CARDRDTHREN** `[0:0]`
    ///
    /// Card read threshold enable.
    /// 1'b0-Card read threshold disabled.
    /// 1'b1-Card read threshold enabled.
    #[inline(always)]
    pub const fn with_cardrdthren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cardthrctl::CARDRDTHREN_MASK
            } else {
                self.bits & !Cardthrctl::CARDRDTHREN_MASK
            },
        }
    }
}

impl Default for CardthrctlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cardthrctl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CardthrctlVal {
        CardthrctlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CardthrctlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CardthrctlVal) -> CardthrctlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMMCDDR Register
///
/// eMMC DDR register
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **HS400_MODE** `[31]` (RW)
///
///   Set 1 to enable HS400 mode.
///
/// - **HALFSTARTBIT** `[1:0]` (RW)
///
///   Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
///   1'b0-Full cycle.
///   1'b1-less than one full cycle.
#[repr(transparent)]
pub struct Emmcddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emmcddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emmcddr {
    // ======== Field Constants ========

    /// **HS400_MODE**
    ///
    /// Set 1 to enable HS400 mode.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const HS400_MODE_OFFSET: u32 = 31;
    pub const HS400_MODE_MASK: u32 = 0x80000000;
    pub const HS400_MODE: u32 = 1 << 31;

    /// **HALFSTARTBIT**
    ///
    /// Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
    /// 1'b0-Full cycle.
    /// 1'b1-less than one full cycle.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const HALFSTARTBIT_OFFSET: u32 = 0;
    pub const HALFSTARTBIT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **HS400_MODE** `[31]` (RW)
    ///
    /// Set 1 to enable HS400 mode.
    #[inline(always)]
    pub fn hs400_mode(&self) -> bool {
        (self.value.read_bits() & Self::HS400_MODE_MASK) != 0
    }

    /// Set HS400_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hs400_mode(&self) {
        self.value.modify_bits(|r| r | Self::HS400_MODE_MASK);
    }

    /// Clear HS400_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hs400_mode(&self) {
        self.value.modify_bits(|r| r & !Self::HS400_MODE_MASK);
    }

    /// **HALFSTARTBIT** `[1:0]` (RW)
    ///
    /// Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
    /// 1'b0-Full cycle.
    /// 1'b1-less than one full cycle.
    #[inline(always)]
    pub fn halfstartbit(&self) -> u32 {
        (self.value.read_bits() & Self::HALFSTARTBIT_MASK) >> Self::HALFSTARTBIT_OFFSET
    }

    /// Write **HALFSTARTBIT** `[1:0]` (RW)
    ///
    /// Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
    /// 1'b0-Full cycle.
    /// 1'b1-less than one full cycle.
    #[inline(always)]
    pub fn write_halfstartbit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HALFSTARTBIT_MASK)
                | ((value << Self::HALFSTARTBIT_OFFSET) & Self::HALFSTARTBIT_MASK)
        });
    }
}

/// EMMCDDR value type for batch read/modify/write operations
///
/// Use `emmcddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emmcddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmmcddrVal {
    bits: u32,
}

impl EmmcddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HS400_MODE** `[31:31]`
    ///
    /// Set 1 to enable HS400 mode.
    #[inline(always)]
    pub const fn hs400_mode(&self) -> bool {
        (self.bits & Emmcddr::HS400_MODE_MASK) != 0
    }

    /// Read **HALFSTARTBIT** `[1:0]`
    ///
    /// Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
    /// 1'b0-Full cycle.
    /// 1'b1-less than one full cycle.
    #[inline(always)]
    pub const fn halfstartbit(&self) -> u32 {
        (self.bits & Emmcddr::HALFSTARTBIT_MASK) >> Emmcddr::HALFSTARTBIT_OFFSET
    }

    /// Set **HS400_MODE** `[31:31]`
    ///
    /// Set 1 to enable HS400 mode.
    #[inline(always)]
    pub const fn with_hs400_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emmcddr::HS400_MODE_MASK
            } else {
                self.bits & !Emmcddr::HS400_MODE_MASK
            },
        }
    }

    /// Set **HALFSTARTBIT** `[1:0]`
    ///
    /// Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be:
    /// 1'b0-Full cycle.
    /// 1'b1-less than one full cycle.
    #[inline(always)]
    pub const fn with_halfstartbit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emmcddr::HALFSTARTBIT_MASK)
                | ((value << Emmcddr::HALFSTARTBIT_OFFSET) & Emmcddr::HALFSTARTBIT_MASK),
        }
    }
}

impl Default for EmmcddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Emmcddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmmcddrVal {
        EmmcddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmmcddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmmcddrVal) -> EmmcddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ENSHIFT Register
///
/// Enable Phase Shift register
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **ENABLE_SHIFT** `[3:0]` (RW)
///
///   Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
///   2'b00-Default phase shift.
///   2'b01-Enables shifted to next immediate positive edge.
///   2'b10-Enables shifted to next immediate negative edge.
///   2'b11-Reserved.
#[repr(transparent)]
pub struct Enshift {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Enshift {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Enshift {
    // ======== Field Constants ========

    /// **ENABLE_SHIFT**
    ///
    /// Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
    /// 2'b00-Default phase shift.
    /// 2'b01-Enables shifted to next immediate positive edge.
    /// 2'b10-Enables shifted to next immediate negative edge.
    /// 2'b11-Reserved.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const ENABLE_SHIFT_OFFSET: u32 = 0;
    pub const ENABLE_SHIFT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **ENABLE_SHIFT** `[3:0]` (RW)
    ///
    /// Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
    /// 2'b00-Default phase shift.
    /// 2'b01-Enables shifted to next immediate positive edge.
    /// 2'b10-Enables shifted to next immediate negative edge.
    /// 2'b11-Reserved.
    #[inline(always)]
    pub fn enable_shift(&self) -> u32 {
        (self.value.read_bits() & Self::ENABLE_SHIFT_MASK) >> Self::ENABLE_SHIFT_OFFSET
    }

    /// Write **ENABLE_SHIFT** `[3:0]` (RW)
    ///
    /// Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
    /// 2'b00-Default phase shift.
    /// 2'b01-Enables shifted to next immediate positive edge.
    /// 2'b10-Enables shifted to next immediate negative edge.
    /// 2'b11-Reserved.
    #[inline(always)]
    pub fn write_enable_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENABLE_SHIFT_MASK)
                | ((value << Self::ENABLE_SHIFT_OFFSET) & Self::ENABLE_SHIFT_MASK)
        });
    }
}

/// ENSHIFT value type for batch read/modify/write operations
///
/// Use `enshift.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enshift.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnshiftVal {
    bits: u32,
}

impl EnshiftVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ENABLE_SHIFT** `[3:0]`
    ///
    /// Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
    /// 2'b00-Default phase shift.
    /// 2'b01-Enables shifted to next immediate positive edge.
    /// 2'b10-Enables shifted to next immediate negative edge.
    /// 2'b11-Reserved.
    #[inline(always)]
    pub const fn enable_shift(&self) -> u32 {
        (self.bits & Enshift::ENABLE_SHIFT_MASK) >> Enshift::ENABLE_SHIFT_OFFSET
    }

    /// Set **ENABLE_SHIFT** `[3:0]`
    ///
    /// Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card.
    /// 2'b00-Default phase shift.
    /// 2'b01-Enables shifted to next immediate positive edge.
    /// 2'b10-Enables shifted to next immediate negative edge.
    /// 2'b11-Reserved.
    #[inline(always)]
    pub const fn with_enable_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Enshift::ENABLE_SHIFT_MASK)
                | ((value << Enshift::ENABLE_SHIFT_OFFSET) & Enshift::ENABLE_SHIFT_MASK),
        }
    }
}

impl Default for EnshiftVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Enshift {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EnshiftVal {
        EnshiftVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnshiftVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EnshiftVal) -> EnshiftVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUFFIFO Register
///
/// CPU write and read transmit data by FIFO
///
/// Address offset: 0x0200
///
/// ## Fields
///
/// - **BUFFIFO** `[31:0]` (RW)
///
///   CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
#[repr(transparent)]
pub struct Buffifo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Buffifo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Buffifo {
    // ======== Field Constants ========

    /// **BUFFIFO**
    ///
    /// CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BUFFIFO_OFFSET: u32 = 0;
    pub const BUFFIFO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BUFFIFO** `[31:0]` (RW)
    ///
    /// CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
    #[inline(always)]
    pub fn buffifo(&self) -> u32 {
        (self.value.read_bits() & Self::BUFFIFO_MASK) >> Self::BUFFIFO_OFFSET
    }

    /// Write **BUFFIFO** `[31:0]` (RW)
    ///
    /// CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
    #[inline(always)]
    pub fn write_buffifo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BUFFIFO_MASK) | ((value << Self::BUFFIFO_OFFSET) & Self::BUFFIFO_MASK)
        });
    }
}

/// BUFFIFO value type for batch read/modify/write operations
///
/// Use `buffifo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `buffifo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BuffifoVal {
    bits: u32,
}

impl BuffifoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BUFFIFO** `[31:0]`
    ///
    /// CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
    #[inline(always)]
    pub const fn buffifo(&self) -> u32 {
        (self.bits & Buffifo::BUFFIFO_MASK) >> Buffifo::BUFFIFO_OFFSET
    }

    /// Set **BUFFIFO** `[31:0]`
    ///
    /// CPU write and read transmit data by FIFO. This register points to the current Data FIFO .
    #[inline(always)]
    pub const fn with_buffifo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Buffifo::BUFFIFO_MASK)
                | ((value << Buffifo::BUFFIFO_OFFSET) & Buffifo::BUFFIFO_MASK),
        }
    }
}

impl Default for BuffifoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Buffifo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BuffifoVal {
        BuffifoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BuffifoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BuffifoVal) -> BuffifoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_EDGE_SEL Register
///
/// SDIO control register.
///
/// Address offset: 0x0800
/// Reset value: 0x00820200
///
/// ## Fields
///
/// - **ULTRA_HIGH_SPEED_MODE** `[24]` (RW)
///
///   Enable ultra high speed mode, use dll to generate clk.
///
/// - **CCLK_EN** `[23]` (RW)
///
///   Sdio clock enable.
///
/// - **ESD_MODE** `[22]` (RW)
///
///   Enable esd mode.
///
/// - **ESDIO_MODE** `[21]` (RW)
///
///   Enable esdio mode.
///
/// - **CCLLKIN_EDGE_N** `[20:17]` (RW)
///
///   The clock division of cclk_in.
///
/// - **CCLLKIN_EDGE_L** `[16:13]` (RW)
///
///   The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
///
/// - **CCLLKIN_EDGE_H** `[12:9]` (RW)
///
///   The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
///
/// - **CCLKIN_EDGE_SLF_SEL** `[8:6]` (RW)
///
///   It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
///
/// - **CCLKIN_EDGE_SAM_SEL** `[5:3]` (RW)
///
///   It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
///
/// - **CCLKIN_EDGE_DRV_SEL** `[2:0]` (RW)
///
///   It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
#[repr(transparent)]
pub struct ClkEdgeSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkEdgeSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkEdgeSel {
    // ======== Field Constants ========

    /// **ULTRA_HIGH_SPEED_MODE**
    ///
    /// Enable ultra high speed mode, use dll to generate clk.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ULTRA_HIGH_SPEED_MODE_OFFSET: u32 = 24;
    pub const ULTRA_HIGH_SPEED_MODE_MASK: u32 = 0x01000000;
    pub const ULTRA_HIGH_SPEED_MODE: u32 = 1 << 24;

    /// **CCLK_EN**
    ///
    /// Sdio clock enable.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CCLK_EN_OFFSET: u32 = 23;
    pub const CCLK_EN_MASK: u32 = 0x00800000;
    pub const CCLK_EN: u32 = 1 << 23;

    /// **ESD_MODE**
    ///
    /// Enable esd mode.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ESD_MODE_OFFSET: u32 = 22;
    pub const ESD_MODE_MASK: u32 = 0x00400000;
    pub const ESD_MODE: u32 = 1 << 22;

    /// **ESDIO_MODE**
    ///
    /// Enable esdio mode.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ESDIO_MODE_OFFSET: u32 = 21;
    pub const ESDIO_MODE_MASK: u32 = 0x00200000;
    pub const ESDIO_MODE: u32 = 1 << 21;

    /// **CCLLKIN_EDGE_N**
    ///
    /// The clock division of cclk_in.
    ///
    /// Bits `[20:17]`, mask `0x001e0000`
    pub const CCLLKIN_EDGE_N_OFFSET: u32 = 17;
    pub const CCLLKIN_EDGE_N_MASK: u32 = 0x001e0000;

    /// **CCLLKIN_EDGE_L**
    ///
    /// The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
    ///
    /// Bits `[16:13]`, mask `0x0001e000`
    pub const CCLLKIN_EDGE_L_OFFSET: u32 = 13;
    pub const CCLLKIN_EDGE_L_MASK: u32 = 0x0001e000;

    /// **CCLLKIN_EDGE_H**
    ///
    /// The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const CCLLKIN_EDGE_H_OFFSET: u32 = 9;
    pub const CCLLKIN_EDGE_H_MASK: u32 = 0x00001e00;

    /// **CCLKIN_EDGE_SLF_SEL**
    ///
    /// It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const CCLKIN_EDGE_SLF_SEL_OFFSET: u32 = 6;
    pub const CCLKIN_EDGE_SLF_SEL_MASK: u32 = 0x000001c0;

    /// **CCLKIN_EDGE_SAM_SEL**
    ///
    /// It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const CCLKIN_EDGE_SAM_SEL_OFFSET: u32 = 3;
    pub const CCLKIN_EDGE_SAM_SEL_MASK: u32 = 0x00000038;

    /// **CCLKIN_EDGE_DRV_SEL**
    ///
    /// It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const CCLKIN_EDGE_DRV_SEL_OFFSET: u32 = 0;
    pub const CCLKIN_EDGE_DRV_SEL_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **ULTRA_HIGH_SPEED_MODE** `[24]` (RW)
    ///
    /// Enable ultra high speed mode, use dll to generate clk.
    #[inline(always)]
    pub fn ultra_high_speed_mode(&self) -> bool {
        (self.value.read_bits() & Self::ULTRA_HIGH_SPEED_MODE_MASK) != 0
    }

    /// Set ULTRA_HIGH_SPEED_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ultra_high_speed_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::ULTRA_HIGH_SPEED_MODE_MASK);
    }

    /// Clear ULTRA_HIGH_SPEED_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ultra_high_speed_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::ULTRA_HIGH_SPEED_MODE_MASK);
    }

    /// **CCLK_EN** `[23]` (RW)
    ///
    /// Sdio clock enable.
    #[inline(always)]
    pub fn cclk_en(&self) -> bool {
        (self.value.read_bits() & Self::CCLK_EN_MASK) != 0
    }

    /// Set CCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cclk_en(&self) {
        self.value.modify_bits(|r| r | Self::CCLK_EN_MASK);
    }

    /// Clear CCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cclk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CCLK_EN_MASK);
    }

    /// **ESD_MODE** `[22]` (RW)
    ///
    /// Enable esd mode.
    #[inline(always)]
    pub fn esd_mode(&self) -> bool {
        (self.value.read_bits() & Self::ESD_MODE_MASK) != 0
    }

    /// Set ESD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_esd_mode(&self) {
        self.value.modify_bits(|r| r | Self::ESD_MODE_MASK);
    }

    /// Clear ESD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_esd_mode(&self) {
        self.value.modify_bits(|r| r & !Self::ESD_MODE_MASK);
    }

    /// **ESDIO_MODE** `[21]` (RW)
    ///
    /// Enable esdio mode.
    #[inline(always)]
    pub fn esdio_mode(&self) -> bool {
        (self.value.read_bits() & Self::ESDIO_MODE_MASK) != 0
    }

    /// Set ESDIO_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_esdio_mode(&self) {
        self.value.modify_bits(|r| r | Self::ESDIO_MODE_MASK);
    }

    /// Clear ESDIO_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_esdio_mode(&self) {
        self.value.modify_bits(|r| r & !Self::ESDIO_MODE_MASK);
    }

    /// **CCLLKIN_EDGE_N** `[20:17]` (RW)
    ///
    /// The clock division of cclk_in.
    #[inline(always)]
    pub fn ccllkin_edge_n(&self) -> u32 {
        (self.value.read_bits() & Self::CCLLKIN_EDGE_N_MASK) >> Self::CCLLKIN_EDGE_N_OFFSET
    }

    /// Write **CCLLKIN_EDGE_N** `[20:17]` (RW)
    ///
    /// The clock division of cclk_in.
    #[inline(always)]
    pub fn write_ccllkin_edge_n(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLLKIN_EDGE_N_MASK)
                | ((value << Self::CCLLKIN_EDGE_N_OFFSET) & Self::CCLLKIN_EDGE_N_MASK)
        });
    }

    /// **CCLLKIN_EDGE_L** `[16:13]` (RW)
    ///
    /// The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
    #[inline(always)]
    pub fn ccllkin_edge_l(&self) -> u32 {
        (self.value.read_bits() & Self::CCLLKIN_EDGE_L_MASK) >> Self::CCLLKIN_EDGE_L_OFFSET
    }

    /// Write **CCLLKIN_EDGE_L** `[16:13]` (RW)
    ///
    /// The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
    #[inline(always)]
    pub fn write_ccllkin_edge_l(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLLKIN_EDGE_L_MASK)
                | ((value << Self::CCLLKIN_EDGE_L_OFFSET) & Self::CCLLKIN_EDGE_L_MASK)
        });
    }

    /// **CCLLKIN_EDGE_H** `[12:9]` (RW)
    ///
    /// The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
    #[inline(always)]
    pub fn ccllkin_edge_h(&self) -> u32 {
        (self.value.read_bits() & Self::CCLLKIN_EDGE_H_MASK) >> Self::CCLLKIN_EDGE_H_OFFSET
    }

    /// Write **CCLLKIN_EDGE_H** `[12:9]` (RW)
    ///
    /// The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
    #[inline(always)]
    pub fn write_ccllkin_edge_h(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLLKIN_EDGE_H_MASK)
                | ((value << Self::CCLLKIN_EDGE_H_OFFSET) & Self::CCLLKIN_EDGE_H_MASK)
        });
    }

    /// **CCLKIN_EDGE_SLF_SEL** `[8:6]` (RW)
    ///
    /// It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&self) -> u32 {
        (self.value.read_bits() & Self::CCLKIN_EDGE_SLF_SEL_MASK)
            >> Self::CCLKIN_EDGE_SLF_SEL_OFFSET
    }

    /// Write **CCLKIN_EDGE_SLF_SEL** `[8:6]` (RW)
    ///
    /// It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn write_cclkin_edge_slf_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLKIN_EDGE_SLF_SEL_MASK)
                | ((value << Self::CCLKIN_EDGE_SLF_SEL_OFFSET) & Self::CCLKIN_EDGE_SLF_SEL_MASK)
        });
    }

    /// **CCLKIN_EDGE_SAM_SEL** `[5:3]` (RW)
    ///
    /// It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&self) -> u32 {
        (self.value.read_bits() & Self::CCLKIN_EDGE_SAM_SEL_MASK)
            >> Self::CCLKIN_EDGE_SAM_SEL_OFFSET
    }

    /// Write **CCLKIN_EDGE_SAM_SEL** `[5:3]` (RW)
    ///
    /// It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn write_cclkin_edge_sam_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLKIN_EDGE_SAM_SEL_MASK)
                | ((value << Self::CCLKIN_EDGE_SAM_SEL_OFFSET) & Self::CCLKIN_EDGE_SAM_SEL_MASK)
        });
    }

    /// **CCLKIN_EDGE_DRV_SEL** `[2:0]` (RW)
    ///
    /// It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&self) -> u32 {
        (self.value.read_bits() & Self::CCLKIN_EDGE_DRV_SEL_MASK)
            >> Self::CCLKIN_EDGE_DRV_SEL_OFFSET
    }

    /// Write **CCLKIN_EDGE_DRV_SEL** `[2:0]` (RW)
    ///
    /// It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub fn write_cclkin_edge_drv_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCLKIN_EDGE_DRV_SEL_MASK)
                | ((value << Self::CCLKIN_EDGE_DRV_SEL_OFFSET) & Self::CCLKIN_EDGE_DRV_SEL_MASK)
        });
    }
}

/// CLK_EDGE_SEL value type for batch read/modify/write operations
///
/// Use `clkedgesel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkedgesel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkEdgeSelVal {
    bits: u32,
}

impl ClkEdgeSelVal {
    /// Create from raw bits
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
        Self { bits: 0x00820200 }
    }

    /// Read **ULTRA_HIGH_SPEED_MODE** `[24:24]`
    ///
    /// Enable ultra high speed mode, use dll to generate clk.
    #[inline(always)]
    pub const fn ultra_high_speed_mode(&self) -> bool {
        (self.bits & ClkEdgeSel::ULTRA_HIGH_SPEED_MODE_MASK) != 0
    }

    /// Read **CCLK_EN** `[23:23]`
    ///
    /// Sdio clock enable.
    #[inline(always)]
    pub const fn cclk_en(&self) -> bool {
        (self.bits & ClkEdgeSel::CCLK_EN_MASK) != 0
    }

    /// Read **ESD_MODE** `[22:22]`
    ///
    /// Enable esd mode.
    #[inline(always)]
    pub const fn esd_mode(&self) -> bool {
        (self.bits & ClkEdgeSel::ESD_MODE_MASK) != 0
    }

    /// Read **ESDIO_MODE** `[21:21]`
    ///
    /// Enable esdio mode.
    #[inline(always)]
    pub const fn esdio_mode(&self) -> bool {
        (self.bits & ClkEdgeSel::ESDIO_MODE_MASK) != 0
    }

    /// Read **CCLLKIN_EDGE_N** `[20:17]`
    ///
    /// The clock division of cclk_in.
    #[inline(always)]
    pub const fn ccllkin_edge_n(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLLKIN_EDGE_N_MASK) >> ClkEdgeSel::CCLLKIN_EDGE_N_OFFSET
    }

    /// Read **CCLLKIN_EDGE_L** `[16:13]`
    ///
    /// The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
    #[inline(always)]
    pub const fn ccllkin_edge_l(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLLKIN_EDGE_L_MASK) >> ClkEdgeSel::CCLLKIN_EDGE_L_OFFSET
    }

    /// Read **CCLLKIN_EDGE_H** `[12:9]`
    ///
    /// The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
    #[inline(always)]
    pub const fn ccllkin_edge_h(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLLKIN_EDGE_H_MASK) >> ClkEdgeSel::CCLLKIN_EDGE_H_OFFSET
    }

    /// Read **CCLKIN_EDGE_SLF_SEL** `[8:6]`
    ///
    /// It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn cclkin_edge_slf_sel(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLKIN_EDGE_SLF_SEL_MASK) >> ClkEdgeSel::CCLKIN_EDGE_SLF_SEL_OFFSET
    }

    /// Read **CCLKIN_EDGE_SAM_SEL** `[5:3]`
    ///
    /// It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn cclkin_edge_sam_sel(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLKIN_EDGE_SAM_SEL_MASK) >> ClkEdgeSel::CCLKIN_EDGE_SAM_SEL_OFFSET
    }

    /// Read **CCLKIN_EDGE_DRV_SEL** `[2:0]`
    ///
    /// It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn cclkin_edge_drv_sel(&self) -> u32 {
        (self.bits & ClkEdgeSel::CCLKIN_EDGE_DRV_SEL_MASK) >> ClkEdgeSel::CCLKIN_EDGE_DRV_SEL_OFFSET
    }

    /// Set **ULTRA_HIGH_SPEED_MODE** `[24:24]`
    ///
    /// Enable ultra high speed mode, use dll to generate clk.
    #[inline(always)]
    pub const fn with_ultra_high_speed_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEdgeSel::ULTRA_HIGH_SPEED_MODE_MASK
            } else {
                self.bits & !ClkEdgeSel::ULTRA_HIGH_SPEED_MODE_MASK
            },
        }
    }

    /// Set **CCLK_EN** `[23:23]`
    ///
    /// Sdio clock enable.
    #[inline(always)]
    pub const fn with_cclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEdgeSel::CCLK_EN_MASK
            } else {
                self.bits & !ClkEdgeSel::CCLK_EN_MASK
            },
        }
    }

    /// Set **ESD_MODE** `[22:22]`
    ///
    /// Enable esd mode.
    #[inline(always)]
    pub const fn with_esd_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEdgeSel::ESD_MODE_MASK
            } else {
                self.bits & !ClkEdgeSel::ESD_MODE_MASK
            },
        }
    }

    /// Set **ESDIO_MODE** `[21:21]`
    ///
    /// Enable esdio mode.
    #[inline(always)]
    pub const fn with_esdio_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEdgeSel::ESDIO_MODE_MASK
            } else {
                self.bits & !ClkEdgeSel::ESDIO_MODE_MASK
            },
        }
    }

    /// Set **CCLLKIN_EDGE_N** `[20:17]`
    ///
    /// The clock division of cclk_in.
    #[inline(always)]
    pub const fn with_ccllkin_edge_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLLKIN_EDGE_N_MASK)
                | ((value << ClkEdgeSel::CCLLKIN_EDGE_N_OFFSET) & ClkEdgeSel::CCLLKIN_EDGE_N_MASK),
        }
    }

    /// Set **CCLLKIN_EDGE_L** `[16:13]`
    ///
    /// The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H.
    #[inline(always)]
    pub const fn with_ccllkin_edge_l(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLLKIN_EDGE_L_MASK)
                | ((value << ClkEdgeSel::CCLLKIN_EDGE_L_OFFSET) & ClkEdgeSel::CCLLKIN_EDGE_L_MASK),
        }
    }

    /// Set **CCLLKIN_EDGE_H** `[12:9]`
    ///
    /// The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L.
    #[inline(always)]
    pub const fn with_ccllkin_edge_h(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLLKIN_EDGE_H_MASK)
                | ((value << ClkEdgeSel::CCLLKIN_EDGE_H_OFFSET) & ClkEdgeSel::CCLLKIN_EDGE_H_MASK),
        }
    }

    /// Set **CCLKIN_EDGE_SLF_SEL** `[8:6]`
    ///
    /// It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn with_cclkin_edge_slf_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLKIN_EDGE_SLF_SEL_MASK)
                | ((value << ClkEdgeSel::CCLKIN_EDGE_SLF_SEL_OFFSET)
                    & ClkEdgeSel::CCLKIN_EDGE_SLF_SEL_MASK),
        }
    }

    /// Set **CCLKIN_EDGE_SAM_SEL** `[5:3]`
    ///
    /// It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn with_cclkin_edge_sam_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLKIN_EDGE_SAM_SEL_MASK)
                | ((value << ClkEdgeSel::CCLKIN_EDGE_SAM_SEL_OFFSET)
                    & ClkEdgeSel::CCLKIN_EDGE_SAM_SEL_MASK),
        }
    }

    /// Set **CCLKIN_EDGE_DRV_SEL** `[2:0]`
    ///
    /// It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270.
    #[inline(always)]
    pub const fn with_cclkin_edge_drv_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkEdgeSel::CCLKIN_EDGE_DRV_SEL_MASK)
                | ((value << ClkEdgeSel::CCLKIN_EDGE_DRV_SEL_OFFSET)
                    & ClkEdgeSel::CCLKIN_EDGE_DRV_SEL_MASK),
        }
    }
}

impl Default for ClkEdgeSelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkEdgeSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkEdgeSelVal {
        ClkEdgeSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkEdgeSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkEdgeSelVal) -> ClkEdgeSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RAW_INTS Register
///
/// SDIO raw ints register.
///
/// Address offset: 0x0804
///
/// ## Fields
///
/// - **RAW_INTS** `[31:0]` (RO)
///
///   It indicates raw ints.
#[repr(transparent)]
pub struct RawInts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RawInts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RawInts {
    // ======== Field Constants ========

    /// **RAW_INTS**
    ///
    /// It indicates raw ints.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RAW_INTS_OFFSET: u32 = 0;
    pub const RAW_INTS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RAW_INTS** `[31:0]` (RO)
    ///
    /// It indicates raw ints.
    #[inline(always)]
    pub fn raw_ints(&self) -> u32 {
        (self.value.read_bits() & Self::RAW_INTS_MASK) >> Self::RAW_INTS_OFFSET
    }
}

/// RAW_INTS value type for batch read/modify/write operations
///
/// Use `rawints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rawints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RawIntsVal {
    bits: u32,
}

impl RawIntsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RAW_INTS** `[31:0]`
    ///
    /// It indicates raw ints.
    #[inline(always)]
    pub const fn raw_ints(&self) -> u32 {
        (self.bits & RawInts::RAW_INTS_MASK) >> RawInts::RAW_INTS_OFFSET
    }
}

impl Default for RawIntsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RawInts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RawIntsVal {
        RawIntsVal::from_bits(self.value.read_bits())
    }
}

/// DLL_CLK_CONF Register
///
/// SDIO DLL clock control register.
///
/// Address offset: 0x0808
///
/// ## Fields
///
/// - **DLL_CCLK_IN_SAM_PHASE** `[20:15]` (RW)
///
///   It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
///
/// - **DLL_CCLK_IN_DRV_PHASE** `[14:9]` (RW)
///
///   It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
///
/// - **DLL_CCLK_IN_SLF_PHASE** `[8:3]` (RW)
///
///   It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
///
/// - **DLL_CCLK_IN_SAM_EN** `[2]` (RW)
///
///   Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
///
/// - **DLL_CCLK_IN_DRV_EN** `[1]` (RW)
///
///   Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
///
/// - **DLL_CCLK_IN_SLF_EN** `[0]` (RW)
///
///   Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
#[repr(transparent)]
pub struct DllClkConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DllClkConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DllClkConf {
    // ======== Field Constants ========

    /// **DLL_CCLK_IN_SAM_PHASE**
    ///
    /// It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[20:15]`, mask `0x001f8000`
    pub const DLL_CCLK_IN_SAM_PHASE_OFFSET: u32 = 15;
    pub const DLL_CCLK_IN_SAM_PHASE_MASK: u32 = 0x001f8000;

    /// **DLL_CCLK_IN_DRV_PHASE**
    ///
    /// It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[14:9]`, mask `0x00007e00`
    pub const DLL_CCLK_IN_DRV_PHASE_OFFSET: u32 = 9;
    pub const DLL_CCLK_IN_DRV_PHASE_MASK: u32 = 0x00007e00;

    /// **DLL_CCLK_IN_SLF_PHASE**
    ///
    /// It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const DLL_CCLK_IN_SLF_PHASE_OFFSET: u32 = 3;
    pub const DLL_CCLK_IN_SLF_PHASE_MASK: u32 = 0x000001f8;

    /// **DLL_CCLK_IN_SAM_EN**
    ///
    /// Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DLL_CCLK_IN_SAM_EN_OFFSET: u32 = 2;
    pub const DLL_CCLK_IN_SAM_EN_MASK: u32 = 0x00000004;
    pub const DLL_CCLK_IN_SAM_EN: u32 = 1 << 2;

    /// **DLL_CCLK_IN_DRV_EN**
    ///
    /// Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DLL_CCLK_IN_DRV_EN_OFFSET: u32 = 1;
    pub const DLL_CCLK_IN_DRV_EN_MASK: u32 = 0x00000002;
    pub const DLL_CCLK_IN_DRV_EN: u32 = 1 << 1;

    /// **DLL_CCLK_IN_SLF_EN**
    ///
    /// Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DLL_CCLK_IN_SLF_EN_OFFSET: u32 = 0;
    pub const DLL_CCLK_IN_SLF_EN_MASK: u32 = 0x00000001;
    pub const DLL_CCLK_IN_SLF_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DLL_CCLK_IN_SAM_PHASE** `[20:15]` (RW)
    ///
    /// It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_sam_phase(&self) -> u32 {
        (self.value.read_bits() & Self::DLL_CCLK_IN_SAM_PHASE_MASK)
            >> Self::DLL_CCLK_IN_SAM_PHASE_OFFSET
    }

    /// Write **DLL_CCLK_IN_SAM_PHASE** `[20:15]` (RW)
    ///
    /// It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn write_dll_cclk_in_sam_phase(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DLL_CCLK_IN_SAM_PHASE_MASK)
                | ((value << Self::DLL_CCLK_IN_SAM_PHASE_OFFSET) & Self::DLL_CCLK_IN_SAM_PHASE_MASK)
        });
    }

    /// **DLL_CCLK_IN_DRV_PHASE** `[14:9]` (RW)
    ///
    /// It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_drv_phase(&self) -> u32 {
        (self.value.read_bits() & Self::DLL_CCLK_IN_DRV_PHASE_MASK)
            >> Self::DLL_CCLK_IN_DRV_PHASE_OFFSET
    }

    /// Write **DLL_CCLK_IN_DRV_PHASE** `[14:9]` (RW)
    ///
    /// It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn write_dll_cclk_in_drv_phase(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DLL_CCLK_IN_DRV_PHASE_MASK)
                | ((value << Self::DLL_CCLK_IN_DRV_PHASE_OFFSET) & Self::DLL_CCLK_IN_DRV_PHASE_MASK)
        });
    }

    /// **DLL_CCLK_IN_SLF_PHASE** `[8:3]` (RW)
    ///
    /// It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_slf_phase(&self) -> u32 {
        (self.value.read_bits() & Self::DLL_CCLK_IN_SLF_PHASE_MASK)
            >> Self::DLL_CCLK_IN_SLF_PHASE_OFFSET
    }

    /// Write **DLL_CCLK_IN_SLF_PHASE** `[8:3]` (RW)
    ///
    /// It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn write_dll_cclk_in_slf_phase(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DLL_CCLK_IN_SLF_PHASE_MASK)
                | ((value << Self::DLL_CCLK_IN_SLF_PHASE_OFFSET) & Self::DLL_CCLK_IN_SLF_PHASE_MASK)
        });
    }

    /// **DLL_CCLK_IN_SAM_EN** `[2]` (RW)
    ///
    /// Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_sam_en(&self) -> bool {
        (self.value.read_bits() & Self::DLL_CCLK_IN_SAM_EN_MASK) != 0
    }

    /// Set DLL_CCLK_IN_SAM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dll_cclk_in_sam_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DLL_CCLK_IN_SAM_EN_MASK);
    }

    /// Clear DLL_CCLK_IN_SAM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dll_cclk_in_sam_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DLL_CCLK_IN_SAM_EN_MASK);
    }

    /// **DLL_CCLK_IN_DRV_EN** `[1]` (RW)
    ///
    /// Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_drv_en(&self) -> bool {
        (self.value.read_bits() & Self::DLL_CCLK_IN_DRV_EN_MASK) != 0
    }

    /// Set DLL_CCLK_IN_DRV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dll_cclk_in_drv_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DLL_CCLK_IN_DRV_EN_MASK);
    }

    /// Clear DLL_CCLK_IN_DRV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dll_cclk_in_drv_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DLL_CCLK_IN_DRV_EN_MASK);
    }

    /// **DLL_CCLK_IN_SLF_EN** `[0]` (RW)
    ///
    /// Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub fn dll_cclk_in_slf_en(&self) -> bool {
        (self.value.read_bits() & Self::DLL_CCLK_IN_SLF_EN_MASK) != 0
    }

    /// Set DLL_CCLK_IN_SLF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dll_cclk_in_slf_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DLL_CCLK_IN_SLF_EN_MASK);
    }

    /// Clear DLL_CCLK_IN_SLF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dll_cclk_in_slf_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DLL_CCLK_IN_SLF_EN_MASK);
    }
}

/// DLL_CLK_CONF value type for batch read/modify/write operations
///
/// Use `dllclkconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dllclkconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DllClkConfVal {
    bits: u32,
}

impl DllClkConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DLL_CCLK_IN_SAM_PHASE** `[20:15]`
    ///
    /// It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_sam_phase(&self) -> u32 {
        (self.bits & DllClkConf::DLL_CCLK_IN_SAM_PHASE_MASK)
            >> DllClkConf::DLL_CCLK_IN_SAM_PHASE_OFFSET
    }

    /// Read **DLL_CCLK_IN_DRV_PHASE** `[14:9]`
    ///
    /// It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_drv_phase(&self) -> u32 {
        (self.bits & DllClkConf::DLL_CCLK_IN_DRV_PHASE_MASK)
            >> DllClkConf::DLL_CCLK_IN_DRV_PHASE_OFFSET
    }

    /// Read **DLL_CCLK_IN_SLF_PHASE** `[8:3]`
    ///
    /// It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_slf_phase(&self) -> u32 {
        (self.bits & DllClkConf::DLL_CCLK_IN_SLF_PHASE_MASK)
            >> DllClkConf::DLL_CCLK_IN_SLF_PHASE_OFFSET
    }

    /// Read **DLL_CCLK_IN_SAM_EN** `[2:2]`
    ///
    /// Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_sam_en(&self) -> bool {
        (self.bits & DllClkConf::DLL_CCLK_IN_SAM_EN_MASK) != 0
    }

    /// Read **DLL_CCLK_IN_DRV_EN** `[1:1]`
    ///
    /// Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_drv_en(&self) -> bool {
        (self.bits & DllClkConf::DLL_CCLK_IN_DRV_EN_MASK) != 0
    }

    /// Read **DLL_CCLK_IN_SLF_EN** `[0:0]`
    ///
    /// Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn dll_cclk_in_slf_en(&self) -> bool {
        (self.bits & DllClkConf::DLL_CCLK_IN_SLF_EN_MASK) != 0
    }

    /// Set **DLL_CCLK_IN_SAM_PHASE** `[20:15]`
    ///
    /// It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_sam_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DllClkConf::DLL_CCLK_IN_SAM_PHASE_MASK)
                | ((value << DllClkConf::DLL_CCLK_IN_SAM_PHASE_OFFSET)
                    & DllClkConf::DLL_CCLK_IN_SAM_PHASE_MASK),
        }
    }

    /// Set **DLL_CCLK_IN_DRV_PHASE** `[14:9]`
    ///
    /// It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_drv_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DllClkConf::DLL_CCLK_IN_DRV_PHASE_MASK)
                | ((value << DllClkConf::DLL_CCLK_IN_DRV_PHASE_OFFSET)
                    & DllClkConf::DLL_CCLK_IN_DRV_PHASE_MASK),
        }
    }

    /// Set **DLL_CCLK_IN_SLF_PHASE** `[8:3]`
    ///
    /// It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_slf_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DllClkConf::DLL_CCLK_IN_SLF_PHASE_MASK)
                | ((value << DllClkConf::DLL_CCLK_IN_SLF_PHASE_OFFSET)
                    & DllClkConf::DLL_CCLK_IN_SLF_PHASE_MASK),
        }
    }

    /// Set **DLL_CCLK_IN_SAM_EN** `[2:2]`
    ///
    /// Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_sam_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DllClkConf::DLL_CCLK_IN_SAM_EN_MASK
            } else {
                self.bits & !DllClkConf::DLL_CCLK_IN_SAM_EN_MASK
            },
        }
    }

    /// Set **DLL_CCLK_IN_DRV_EN** `[1:1]`
    ///
    /// Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_drv_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DllClkConf::DLL_CCLK_IN_DRV_EN_MASK
            } else {
                self.bits & !DllClkConf::DLL_CCLK_IN_DRV_EN_MASK
            },
        }
    }

    /// Set **DLL_CCLK_IN_SLF_EN** `[0:0]`
    ///
    /// Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1.
    #[inline(always)]
    pub const fn with_dll_cclk_in_slf_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DllClkConf::DLL_CCLK_IN_SLF_EN_MASK
            } else {
                self.bits & !DllClkConf::DLL_CCLK_IN_SLF_EN_MASK
            },
        }
    }
}

impl Default for DllClkConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DllClkConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DllClkConfVal {
        DllClkConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DllClkConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DllClkConfVal) -> DllClkConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DLL_CONF Register
///
/// SDIO DLL configuration register.
///
/// Address offset: 0x080c
///
/// ## Fields
///
/// - **DLL_CAL_END** `[1]` (RO)
///
///   1 means calibration finished.
///
/// - **DLL_CAL_STOP** `[0]` (RW)
///
///   Set 1 to stop calibration.
#[repr(transparent)]
pub struct DllConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DllConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DllConf {
    // ======== Field Constants ========

    /// **DLL_CAL_END**
    ///
    /// 1 means calibration finished.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DLL_CAL_END_OFFSET: u32 = 1;
    pub const DLL_CAL_END_MASK: u32 = 0x00000002;
    pub const DLL_CAL_END: u32 = 1 << 1;

    /// **DLL_CAL_STOP**
    ///
    /// Set 1 to stop calibration.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DLL_CAL_STOP_OFFSET: u32 = 0;
    pub const DLL_CAL_STOP_MASK: u32 = 0x00000001;
    pub const DLL_CAL_STOP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DLL_CAL_END** `[1]` (RO)
    ///
    /// 1 means calibration finished.
    #[inline(always)]
    pub fn dll_cal_end(&self) -> bool {
        (self.value.read_bits() & Self::DLL_CAL_END_MASK) != 0
    }

    /// **DLL_CAL_STOP** `[0]` (RW)
    ///
    /// Set 1 to stop calibration.
    #[inline(always)]
    pub fn dll_cal_stop(&self) -> bool {
        (self.value.read_bits() & Self::DLL_CAL_STOP_MASK) != 0
    }

    /// Set DLL_CAL_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dll_cal_stop(&self) {
        self.value.modify_bits(|r| r | Self::DLL_CAL_STOP_MASK);
    }

    /// Clear DLL_CAL_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dll_cal_stop(&self) {
        self.value.modify_bits(|r| r & !Self::DLL_CAL_STOP_MASK);
    }
}

/// DLL_CONF value type for batch read/modify/write operations
///
/// Use `dllconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dllconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DllConfVal {
    bits: u32,
}

impl DllConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DLL_CAL_END** `[1:1]`
    ///
    /// 1 means calibration finished.
    #[inline(always)]
    pub const fn dll_cal_end(&self) -> bool {
        (self.bits & DllConf::DLL_CAL_END_MASK) != 0
    }

    /// Read **DLL_CAL_STOP** `[0:0]`
    ///
    /// Set 1 to stop calibration.
    #[inline(always)]
    pub const fn dll_cal_stop(&self) -> bool {
        (self.bits & DllConf::DLL_CAL_STOP_MASK) != 0
    }

    /// Set **DLL_CAL_STOP** `[0:0]`
    ///
    /// Set 1 to stop calibration.
    #[inline(always)]
    pub const fn with_dll_cal_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DllConf::DLL_CAL_STOP_MASK
            } else {
                self.bits & !DllConf::DLL_CAL_STOP_MASK
            },
        }
    }
}

impl Default for DllConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DllConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DllConfVal {
        DllConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DllConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DllConfVal) -> DllConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control register
    pub ctrl: Ctrl,
    _reserved0: [u8; 4],
    /// 0x0008
    /// Clock divider configuration register
    pub clkdiv: Clkdiv,
    /// 0x000c
    /// Clock source selection register
    pub clksrc: Clksrc,
    /// 0x0010
    /// Clock enable register
    pub clkena: Clkena,
    /// 0x0014
    /// Data and response timeout configuration register
    pub tmout: Tmout,
    /// 0x0018
    /// Card bus width configuration register
    pub ctype: Ctype,
    /// 0x001c
    /// Card data block size configuration register
    pub blksiz: Blksiz,
    /// 0x0020
    /// Data transfer length configuration register
    pub bytcnt: Bytcnt,
    /// 0x0024
    /// SDIO interrupt mask register
    pub intmask: Intmask,
    /// 0x0028
    /// Command argument data register
    pub cmdarg: Cmdarg,
    /// 0x002c
    /// Command and boot configuration register
    pub cmd: Cmd,
    /// 0x0030
    /// Response data register
    pub resp0: Resp0,
    /// 0x0034
    /// Long response data register
    pub resp1: Resp1,
    /// 0x0038
    /// Long response data register
    pub resp2: Resp2,
    /// 0x003c
    /// Long response data register
    pub resp3: Resp3,
    /// 0x0040
    /// Masked interrupt status register
    pub mintsts: Mintsts,
    /// 0x0044
    /// Raw interrupt status register
    pub rintsts: Rintsts,
    /// 0x0048
    /// SD/MMC status register
    pub status: Status,
    /// 0x004c
    /// FIFO configuration register
    pub fifoth: Fifoth,
    /// 0x0050
    /// Card detect register
    pub cdetect: Cdetect,
    /// 0x0054
    /// Card write protection (WP) status register
    pub wrtprt: Wrtprt,
    _reserved1: [u8; 4],
    /// 0x005c
    /// Transferred byte count register
    pub tcbcnt: Tcbcnt,
    /// 0x0060
    /// Transferred byte count register
    pub tbbcnt: Tbbcnt,
    /// 0x0064
    /// Debounce filter time configuration register
    pub debnce: Debnce,
    /// 0x0068
    /// User ID (scratchpad) register
    pub usrid: Usrid,
    /// 0x006c
    /// Version ID (scratchpad) register
    pub verid: Verid,
    /// 0x0070
    /// Hardware feature register
    pub hcon: Hcon,
    /// 0x0074
    /// UHS-1 register
    pub uhs: Uhs,
    /// 0x0078
    /// Card reset register
    pub rst_n: RstN,
    _reserved2: [u8; 4],
    /// 0x0080
    /// Burst mode transfer configuration register
    pub bmod: Bmod,
    /// 0x0084
    /// Poll demand configuration register
    pub pldmnd: Pldmnd,
    /// 0x0088
    /// Descriptor base address register
    pub dbaddr: Dbaddr,
    /// 0x008c
    /// IDMAC status register
    pub idsts: Idsts,
    /// 0x0090
    /// IDMAC interrupt enable register
    pub idinten: Idinten,
    /// 0x0094
    /// Host descriptor address pointer
    pub dscaddr: Dscaddr,
    /// 0x0098
    /// Host buffer address pointer register
    pub bufaddr: Bufaddr,
    _reserved3: [u8; 100],
    /// 0x0100
    /// Card Threshold Control register
    pub cardthrctl: Cardthrctl,
    _reserved4: [u8; 8],
    /// 0x010c
    /// eMMC DDR register
    pub emmcddr: Emmcddr,
    /// 0x0110
    /// Enable Phase Shift register
    pub enshift: Enshift,
    _reserved5: [u8; 236],
    /// 0x0200
    /// CPU write and read transmit data by FIFO
    pub buffifo: Buffifo,
    _reserved6: [u8; 1532],
    /// 0x0800
    /// SDIO control register.
    pub clk_edge_sel: ClkEdgeSel,
    /// 0x0804
    /// SDIO raw ints register.
    pub raw_ints: RawInts,
    /// 0x0808
    /// SDIO DLL clock control register.
    pub dll_clk_conf: DllClkConf,
    /// 0x080c
    /// SDIO DLL configuration register.
    pub dll_conf: DllConf,
}
