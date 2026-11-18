//! LP_UART
//! Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x50121000;

/// FIFO Register
///
/// FIFO data register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **RXFIFO_RD_BYTE** `[7:0]` (RO)
///
///   UART 0 accesses FIFO via this register.
#[repr(transparent)]
pub struct Fifo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Fifo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fifo {
    // ======== Field Constants ========

    /// **RXFIFO_RD_BYTE**
    ///
    /// UART 0 accesses FIFO via this register.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RXFIFO_RD_BYTE_OFFSET: u32 = 0;
    pub const RXFIFO_RD_BYTE_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RXFIFO_RD_BYTE** `[7:0]` (RO)
    ///
    /// UART 0 accesses FIFO via this register.
    #[inline(always)]
    pub fn rxfifo_rd_byte(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_RD_BYTE_MASK) >> Self::RXFIFO_RD_BYTE_OFFSET
    }
}

/// FIFO value type for batch read/modify/write operations
///
/// Use `fifo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoVal {
    bits: u32,
}

impl FifoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RXFIFO_RD_BYTE** `[7:0]`
    ///
    /// UART 0 accesses FIFO via this register.
    #[inline(always)]
    pub const fn rxfifo_rd_byte(&self) -> u32 {
        (self.bits & Fifo::RXFIFO_RD_BYTE_MASK) >> Fifo::RXFIFO_RD_BYTE_OFFSET
    }
}

impl Default for FifoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Fifo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoVal {
        FifoVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// Raw interrupt status
///
/// Address offset: 0x0004
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **WAKEUP_INT_RAW** `[19]` (RW)
///
///   This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode.
///
/// - **AT_CMD_CHAR_DET_INT_RAW** `[18]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects the configured at_cmd char.
///
/// - **TX_DONE_INT_RAW** `[14]` (RW)
///
///   This interrupt raw bit turns to high level when transmitter has send out all data in FIFO.
///
/// - **TX_BRK_IDLE_DONE_INT_RAW** `[13]` (RW)
///
///   This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data.
///
/// - **TX_BRK_DONE_INT_RAW** `[12]` (RW)
///
///   This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent.
///
/// - **GLITCH_DET_INT_RAW** `[11]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit.
///
/// - **SW_XOFF_INT_RAW** `[10]` (RW)
///
///   This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1.
///
/// - **SW_XON_INT_RAW** `[9]` (RW)
///
///   This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1.
///
/// - **RXFIFO_TOUT_INT_RAW** `[8]` (RW)
///
///   This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte.
///
/// - **BRK_DET_INT_RAW** `[7]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit.
///
/// - **CTS_CHG_INT_RAW** `[6]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal.
///
/// - **DSR_CHG_INT_RAW** `[5]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal.
///
/// - **RXFIFO_OVF_INT_RAW** `[4]` (RW)
///
///   This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store.
///
/// - **FRM_ERR_INT_RAW** `[3]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects a data frame error .
///
/// - **PARITY_ERR_INT_RAW** `[2]` (RW)
///
///   This interrupt raw bit turns to high level when receiver detects a parity error in the data.
///
/// - **TXFIFO_EMPTY_INT_RAW** `[1]` (RW)
///
///   This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies .
///
/// - **RXFIFO_FULL_INT_RAW** `[0]` (RW)
///
///   This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies.
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

    /// **WAKEUP_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const WAKEUP_INT_RAW_OFFSET: u32 = 19;
    pub const WAKEUP_INT_RAW_MASK: u32 = 0x00080000;
    pub const WAKEUP_INT_RAW: u32 = 1 << 19;

    /// **AT_CMD_CHAR_DET_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects the configured at_cmd char.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const AT_CMD_CHAR_DET_INT_RAW_OFFSET: u32 = 18;
    pub const AT_CMD_CHAR_DET_INT_RAW_MASK: u32 = 0x00040000;
    pub const AT_CMD_CHAR_DET_INT_RAW: u32 = 1 << 18;

    /// **TX_DONE_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when transmitter has send out all data in FIFO.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_DONE_INT_RAW_OFFSET: u32 = 14;
    pub const TX_DONE_INT_RAW_MASK: u32 = 0x00004000;
    pub const TX_DONE_INT_RAW: u32 = 1 << 14;

    /// **TX_BRK_IDLE_DONE_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_BRK_IDLE_DONE_INT_RAW_OFFSET: u32 = 13;
    pub const TX_BRK_IDLE_DONE_INT_RAW_MASK: u32 = 0x00002000;
    pub const TX_BRK_IDLE_DONE_INT_RAW: u32 = 1 << 13;

    /// **TX_BRK_DONE_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_BRK_DONE_INT_RAW_OFFSET: u32 = 12;
    pub const TX_BRK_DONE_INT_RAW_MASK: u32 = 0x00001000;
    pub const TX_BRK_DONE_INT_RAW: u32 = 1 << 12;

    /// **GLITCH_DET_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GLITCH_DET_INT_RAW_OFFSET: u32 = 11;
    pub const GLITCH_DET_INT_RAW_MASK: u32 = 0x00000800;
    pub const GLITCH_DET_INT_RAW: u32 = 1 << 11;

    /// **SW_XOFF_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SW_XOFF_INT_RAW_OFFSET: u32 = 10;
    pub const SW_XOFF_INT_RAW_MASK: u32 = 0x00000400;
    pub const SW_XOFF_INT_RAW: u32 = 1 << 10;

    /// **SW_XON_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SW_XON_INT_RAW_OFFSET: u32 = 9;
    pub const SW_XON_INT_RAW_MASK: u32 = 0x00000200;
    pub const SW_XON_INT_RAW: u32 = 1 << 9;

    /// **RXFIFO_TOUT_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RXFIFO_TOUT_INT_RAW_OFFSET: u32 = 8;
    pub const RXFIFO_TOUT_INT_RAW_MASK: u32 = 0x00000100;
    pub const RXFIFO_TOUT_INT_RAW: u32 = 1 << 8;

    /// **BRK_DET_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BRK_DET_INT_RAW_OFFSET: u32 = 7;
    pub const BRK_DET_INT_RAW_MASK: u32 = 0x00000080;
    pub const BRK_DET_INT_RAW: u32 = 1 << 7;

    /// **CTS_CHG_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CTS_CHG_INT_RAW_OFFSET: u32 = 6;
    pub const CTS_CHG_INT_RAW_MASK: u32 = 0x00000040;
    pub const CTS_CHG_INT_RAW: u32 = 1 << 6;

    /// **DSR_CHG_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DSR_CHG_INT_RAW_OFFSET: u32 = 5;
    pub const DSR_CHG_INT_RAW_MASK: u32 = 0x00000020;
    pub const DSR_CHG_INT_RAW: u32 = 1 << 5;

    /// **RXFIFO_OVF_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXFIFO_OVF_INT_RAW_OFFSET: u32 = 4;
    pub const RXFIFO_OVF_INT_RAW_MASK: u32 = 0x00000010;
    pub const RXFIFO_OVF_INT_RAW: u32 = 1 << 4;

    /// **FRM_ERR_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects a data frame error .
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FRM_ERR_INT_RAW_OFFSET: u32 = 3;
    pub const FRM_ERR_INT_RAW_MASK: u32 = 0x00000008;
    pub const FRM_ERR_INT_RAW: u32 = 1 << 3;

    /// **PARITY_ERR_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver detects a parity error in the data.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PARITY_ERR_INT_RAW_OFFSET: u32 = 2;
    pub const PARITY_ERR_INT_RAW_MASK: u32 = 0x00000004;
    pub const PARITY_ERR_INT_RAW: u32 = 1 << 2;

    /// **TXFIFO_EMPTY_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies .
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_EMPTY_INT_RAW_OFFSET: u32 = 1;
    pub const TXFIFO_EMPTY_INT_RAW_MASK: u32 = 0x00000002;
    pub const TXFIFO_EMPTY_INT_RAW: u32 = 1 << 1;

    /// **RXFIFO_FULL_INT_RAW**
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_FULL_INT_RAW_OFFSET: u32 = 0;
    pub const RXFIFO_FULL_INT_RAW_MASK: u32 = 0x00000001;
    pub const RXFIFO_FULL_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WAKEUP_INT_RAW** `[19]` (RW)
    ///
    /// This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode.
    #[inline(always)]
    pub fn wakeup_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_INT_RAW_MASK) != 0
    }

    /// Set WAKEUP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wakeup_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::WAKEUP_INT_RAW_MASK);
    }

    /// Clear WAKEUP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wakeup_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::WAKEUP_INT_RAW_MASK);
    }

    /// **AT_CMD_CHAR_DET_INT_RAW** `[18]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects the configured at_cmd char.
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::AT_CMD_CHAR_DET_INT_RAW_MASK) != 0
    }

    /// Set AT_CMD_CHAR_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_at_cmd_char_det_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::AT_CMD_CHAR_DET_INT_RAW_MASK);
    }

    /// Clear AT_CMD_CHAR_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_at_cmd_char_det_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::AT_CMD_CHAR_DET_INT_RAW_MASK);
    }

    /// **TX_DONE_INT_RAW** `[14]` (RW)
    ///
    /// This interrupt raw bit turns to high level when transmitter has send out all data in FIFO.
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_RAW_MASK) != 0
    }

    /// Set TX_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TX_DONE_INT_RAW_MASK);
    }

    /// Clear TX_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_done_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TX_DONE_INT_RAW_MASK);
    }

    /// **TX_BRK_IDLE_DONE_INT_RAW** `[13]` (RW)
    ///
    /// This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data.
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_IDLE_DONE_INT_RAW_MASK) != 0
    }

    /// Set TX_BRK_IDLE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_brk_idle_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_BRK_IDLE_DONE_INT_RAW_MASK);
    }

    /// Clear TX_BRK_IDLE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_brk_idle_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_BRK_IDLE_DONE_INT_RAW_MASK);
    }

    /// **TX_BRK_DONE_INT_RAW** `[12]` (RW)
    ///
    /// This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent.
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_DONE_INT_RAW_MASK) != 0
    }

    /// Set TX_BRK_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_brk_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_BRK_DONE_INT_RAW_MASK);
    }

    /// Clear TX_BRK_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_brk_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_BRK_DONE_INT_RAW_MASK);
    }

    /// **GLITCH_DET_INT_RAW** `[11]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit.
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::GLITCH_DET_INT_RAW_MASK) != 0
    }

    /// Set GLITCH_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_glitch_det_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::GLITCH_DET_INT_RAW_MASK);
    }

    /// Clear GLITCH_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_glitch_det_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::GLITCH_DET_INT_RAW_MASK);
    }

    /// **SW_XOFF_INT_RAW** `[10]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SW_XOFF_INT_RAW_MASK) != 0
    }

    /// Set SW_XOFF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_xoff_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SW_XOFF_INT_RAW_MASK);
    }

    /// Clear SW_XOFF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_xoff_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::SW_XOFF_INT_RAW_MASK);
    }

    /// **SW_XON_INT_RAW** `[9]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SW_XON_INT_RAW_MASK) != 0
    }

    /// Set SW_XON_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_xon_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SW_XON_INT_RAW_MASK);
    }

    /// Clear SW_XON_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_xon_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::SW_XON_INT_RAW_MASK);
    }

    /// **RXFIFO_TOUT_INT_RAW** `[8]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte.
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_TOUT_INT_RAW_MASK) != 0
    }

    /// Set RXFIFO_TOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_tout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_TOUT_INT_RAW_MASK);
    }

    /// Clear RXFIFO_TOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_tout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_TOUT_INT_RAW_MASK);
    }

    /// **BRK_DET_INT_RAW** `[7]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit.
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::BRK_DET_INT_RAW_MASK) != 0
    }

    /// Set BRK_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_brk_det_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::BRK_DET_INT_RAW_MASK);
    }

    /// Clear BRK_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_brk_det_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::BRK_DET_INT_RAW_MASK);
    }

    /// **CTS_CHG_INT_RAW** `[6]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal.
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CTS_CHG_INT_RAW_MASK) != 0
    }

    /// Set CTS_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cts_chg_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::CTS_CHG_INT_RAW_MASK);
    }

    /// Clear CTS_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cts_chg_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::CTS_CHG_INT_RAW_MASK);
    }

    /// **DSR_CHG_INT_RAW** `[5]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal.
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DSR_CHG_INT_RAW_MASK) != 0
    }

    /// Set DSR_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsr_chg_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::DSR_CHG_INT_RAW_MASK);
    }

    /// Clear DSR_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsr_chg_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::DSR_CHG_INT_RAW_MASK);
    }

    /// **RXFIFO_OVF_INT_RAW** `[4]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store.
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Set RXFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_OVF_INT_RAW_MASK);
    }

    /// Clear RXFIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_OVF_INT_RAW_MASK);
    }

    /// **FRM_ERR_INT_RAW** `[3]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects a data frame error .
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::FRM_ERR_INT_RAW_MASK) != 0
    }

    /// Set FRM_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frm_err_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::FRM_ERR_INT_RAW_MASK);
    }

    /// Clear FRM_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frm_err_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::FRM_ERR_INT_RAW_MASK);
    }

    /// **PARITY_ERR_INT_RAW** `[2]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver detects a parity error in the data.
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::PARITY_ERR_INT_RAW_MASK) != 0
    }

    /// Set PARITY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_parity_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::PARITY_ERR_INT_RAW_MASK);
    }

    /// Clear PARITY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_parity_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::PARITY_ERR_INT_RAW_MASK);
    }

    /// **TXFIFO_EMPTY_INT_RAW** `[1]` (RW)
    ///
    /// This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies .
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_EMPTY_INT_RAW_MASK) != 0
    }

    /// Set TXFIFO_EMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_empty_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TXFIFO_EMPTY_INT_RAW_MASK);
    }

    /// Clear TXFIFO_EMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_empty_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TXFIFO_EMPTY_INT_RAW_MASK);
    }

    /// **RXFIFO_FULL_INT_RAW** `[0]` (RW)
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies.
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_FULL_INT_RAW_MASK) != 0
    }

    /// Set RXFIFO_FULL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_full_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_FULL_INT_RAW_MASK);
    }

    /// Clear RXFIFO_FULL_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_full_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_FULL_INT_RAW_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000002 }
    }

    /// Read **WAKEUP_INT_RAW** `[19:19]`
    ///
    /// This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode.
    #[inline(always)]
    pub const fn wakeup_int_raw(&self) -> bool {
        (self.bits & IntRaw::WAKEUP_INT_RAW_MASK) != 0
    }

    /// Read **AT_CMD_CHAR_DET_INT_RAW** `[18:18]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the configured at_cmd char.
    #[inline(always)]
    pub const fn at_cmd_char_det_int_raw(&self) -> bool {
        (self.bits & IntRaw::AT_CMD_CHAR_DET_INT_RAW_MASK) != 0
    }

    /// Read **TX_DONE_INT_RAW** `[14:14]`
    ///
    /// This interrupt raw bit turns to high level when transmitter has send out all data in FIFO.
    #[inline(always)]
    pub const fn tx_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_DONE_INT_RAW_MASK) != 0
    }

    /// Read **TX_BRK_IDLE_DONE_INT_RAW** `[13:13]`
    ///
    /// This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data.
    #[inline(always)]
    pub const fn tx_brk_idle_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_BRK_IDLE_DONE_INT_RAW_MASK) != 0
    }

    /// Read **TX_BRK_DONE_INT_RAW** `[12:12]`
    ///
    /// This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent.
    #[inline(always)]
    pub const fn tx_brk_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_BRK_DONE_INT_RAW_MASK) != 0
    }

    /// Read **GLITCH_DET_INT_RAW** `[11:11]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit.
    #[inline(always)]
    pub const fn glitch_det_int_raw(&self) -> bool {
        (self.bits & IntRaw::GLITCH_DET_INT_RAW_MASK) != 0
    }

    /// Read **SW_XOFF_INT_RAW** `[10:10]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub const fn sw_xoff_int_raw(&self) -> bool {
        (self.bits & IntRaw::SW_XOFF_INT_RAW_MASK) != 0
    }

    /// Read **SW_XON_INT_RAW** `[9:9]`
    ///
    /// This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub const fn sw_xon_int_raw(&self) -> bool {
        (self.bits & IntRaw::SW_XON_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_TOUT_INT_RAW** `[8:8]`
    ///
    /// This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte.
    #[inline(always)]
    pub const fn rxfifo_tout_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_TOUT_INT_RAW_MASK) != 0
    }

    /// Read **BRK_DET_INT_RAW** `[7:7]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit.
    #[inline(always)]
    pub const fn brk_det_int_raw(&self) -> bool {
        (self.bits & IntRaw::BRK_DET_INT_RAW_MASK) != 0
    }

    /// Read **CTS_CHG_INT_RAW** `[6:6]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal.
    #[inline(always)]
    pub const fn cts_chg_int_raw(&self) -> bool {
        (self.bits & IntRaw::CTS_CHG_INT_RAW_MASK) != 0
    }

    /// Read **DSR_CHG_INT_RAW** `[5:5]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal.
    #[inline(always)]
    pub const fn dsr_chg_int_raw(&self) -> bool {
        (self.bits & IntRaw::DSR_CHG_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_RAW** `[4:4]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **FRM_ERR_INT_RAW** `[3:3]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a data frame error .
    #[inline(always)]
    pub const fn frm_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::FRM_ERR_INT_RAW_MASK) != 0
    }

    /// Read **PARITY_ERR_INT_RAW** `[2:2]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a parity error in the data.
    #[inline(always)]
    pub const fn parity_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::PARITY_ERR_INT_RAW_MASK) != 0
    }

    /// Read **TXFIFO_EMPTY_INT_RAW** `[1:1]`
    ///
    /// This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies .
    #[inline(always)]
    pub const fn txfifo_empty_int_raw(&self) -> bool {
        (self.bits & IntRaw::TXFIFO_EMPTY_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_FULL_INT_RAW** `[0:0]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies.
    #[inline(always)]
    pub const fn rxfifo_full_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_FULL_INT_RAW_MASK) != 0
    }

    /// Set **WAKEUP_INT_RAW** `[19:19]`
    ///
    /// This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode.
    #[inline(always)]
    pub const fn with_wakeup_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::WAKEUP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::WAKEUP_INT_RAW_MASK
            },
        }
    }

    /// Set **AT_CMD_CHAR_DET_INT_RAW** `[18:18]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the configured at_cmd char.
    #[inline(always)]
    pub const fn with_at_cmd_char_det_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::AT_CMD_CHAR_DET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::AT_CMD_CHAR_DET_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_DONE_INT_RAW** `[14:14]`
    ///
    /// This interrupt raw bit turns to high level when transmitter has send out all data in FIFO.
    #[inline(always)]
    pub const fn with_tx_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_BRK_IDLE_DONE_INT_RAW** `[13:13]`
    ///
    /// This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data.
    #[inline(always)]
    pub const fn with_tx_brk_idle_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_BRK_IDLE_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_BRK_IDLE_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_BRK_DONE_INT_RAW** `[12:12]`
    ///
    /// This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent.
    #[inline(always)]
    pub const fn with_tx_brk_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_BRK_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_BRK_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **GLITCH_DET_INT_RAW** `[11:11]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit.
    #[inline(always)]
    pub const fn with_glitch_det_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::GLITCH_DET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::GLITCH_DET_INT_RAW_MASK
            },
        }
    }

    /// Set **SW_XOFF_INT_RAW** `[10:10]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub const fn with_sw_xoff_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SW_XOFF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SW_XOFF_INT_RAW_MASK
            },
        }
    }

    /// Set **SW_XON_INT_RAW** `[9:9]`
    ///
    /// This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1.
    #[inline(always)]
    pub const fn with_sw_xon_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SW_XON_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SW_XON_INT_RAW_MASK
            },
        }
    }

    /// Set **RXFIFO_TOUT_INT_RAW** `[8:8]`
    ///
    /// This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte.
    #[inline(always)]
    pub const fn with_rxfifo_tout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RXFIFO_TOUT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RXFIFO_TOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **BRK_DET_INT_RAW** `[7:7]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit.
    #[inline(always)]
    pub const fn with_brk_det_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::BRK_DET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::BRK_DET_INT_RAW_MASK
            },
        }
    }

    /// Set **CTS_CHG_INT_RAW** `[6:6]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal.
    #[inline(always)]
    pub const fn with_cts_chg_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CTS_CHG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CTS_CHG_INT_RAW_MASK
            },
        }
    }

    /// Set **DSR_CHG_INT_RAW** `[5:5]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal.
    #[inline(always)]
    pub const fn with_dsr_chg_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DSR_CHG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DSR_CHG_INT_RAW_MASK
            },
        }
    }

    /// Set **RXFIFO_OVF_INT_RAW** `[4:4]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store.
    #[inline(always)]
    pub const fn with_rxfifo_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RXFIFO_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RXFIFO_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **FRM_ERR_INT_RAW** `[3:3]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a data frame error .
    #[inline(always)]
    pub const fn with_frm_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::FRM_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::FRM_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **PARITY_ERR_INT_RAW** `[2:2]`
    ///
    /// This interrupt raw bit turns to high level when receiver detects a parity error in the data.
    #[inline(always)]
    pub const fn with_parity_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::PARITY_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::PARITY_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **TXFIFO_EMPTY_INT_RAW** `[1:1]`
    ///
    /// This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies .
    #[inline(always)]
    pub const fn with_txfifo_empty_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TXFIFO_EMPTY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TXFIFO_EMPTY_INT_RAW_MASK
            },
        }
    }

    /// Set **RXFIFO_FULL_INT_RAW** `[0:0]`
    ///
    /// This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies.
    #[inline(always)]
    pub const fn with_rxfifo_full_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RXFIFO_FULL_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RXFIFO_FULL_INT_RAW_MASK
            },
        }
    }
}

impl Default for IntRawVal {
    fn default() -> Self {
        Self::reset_val()
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
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **WAKEUP_INT_ST** `[19]` (RO)
///
///   This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1.
///
/// - **AT_CMD_CHAR_DET_INT_ST** `[18]` (RO)
///
///   This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1.
///
/// - **TX_DONE_INT_ST** `[14]` (RO)
///
///   This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1.
///
/// - **TX_BRK_IDLE_DONE_INT_ST** `[13]` (RO)
///
///   This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1.
///
/// - **TX_BRK_DONE_INT_ST** `[12]` (RO)
///
///   This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1.
///
/// - **GLITCH_DET_INT_ST** `[11]` (RO)
///
///   This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1.
///
/// - **SW_XOFF_INT_ST** `[10]` (RO)
///
///   This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1.
///
/// - **SW_XON_INT_ST** `[9]` (RO)
///
///   This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1.
///
/// - **RXFIFO_TOUT_INT_ST** `[8]` (RO)
///
///   This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1.
///
/// - **BRK_DET_INT_ST** `[7]` (RO)
///
///   This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1.
///
/// - **CTS_CHG_INT_ST** `[6]` (RO)
///
///   This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1.
///
/// - **DSR_CHG_INT_ST** `[5]` (RO)
///
///   This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1.
///
/// - **RXFIFO_OVF_INT_ST** `[4]` (RO)
///
///   This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1.
///
/// - **FRM_ERR_INT_ST** `[3]` (RO)
///
///   This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1.
///
/// - **PARITY_ERR_INT_ST** `[2]` (RO)
///
///   This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1.
///
/// - **TXFIFO_EMPTY_INT_ST** `[1]` (RO)
///
///   This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1.
///
/// - **RXFIFO_FULL_INT_ST** `[0]` (RO)
///
///   This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1.
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

    /// **WAKEUP_INT_ST**
    ///
    /// This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const WAKEUP_INT_ST_OFFSET: u32 = 19;
    pub const WAKEUP_INT_ST_MASK: u32 = 0x00080000;
    pub const WAKEUP_INT_ST: u32 = 1 << 19;

    /// **AT_CMD_CHAR_DET_INT_ST**
    ///
    /// This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const AT_CMD_CHAR_DET_INT_ST_OFFSET: u32 = 18;
    pub const AT_CMD_CHAR_DET_INT_ST_MASK: u32 = 0x00040000;
    pub const AT_CMD_CHAR_DET_INT_ST: u32 = 1 << 18;

    /// **TX_DONE_INT_ST**
    ///
    /// This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_DONE_INT_ST_OFFSET: u32 = 14;
    pub const TX_DONE_INT_ST_MASK: u32 = 0x00004000;
    pub const TX_DONE_INT_ST: u32 = 1 << 14;

    /// **TX_BRK_IDLE_DONE_INT_ST**
    ///
    /// This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_BRK_IDLE_DONE_INT_ST_OFFSET: u32 = 13;
    pub const TX_BRK_IDLE_DONE_INT_ST_MASK: u32 = 0x00002000;
    pub const TX_BRK_IDLE_DONE_INT_ST: u32 = 1 << 13;

    /// **TX_BRK_DONE_INT_ST**
    ///
    /// This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_BRK_DONE_INT_ST_OFFSET: u32 = 12;
    pub const TX_BRK_DONE_INT_ST_MASK: u32 = 0x00001000;
    pub const TX_BRK_DONE_INT_ST: u32 = 1 << 12;

    /// **GLITCH_DET_INT_ST**
    ///
    /// This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GLITCH_DET_INT_ST_OFFSET: u32 = 11;
    pub const GLITCH_DET_INT_ST_MASK: u32 = 0x00000800;
    pub const GLITCH_DET_INT_ST: u32 = 1 << 11;

    /// **SW_XOFF_INT_ST**
    ///
    /// This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SW_XOFF_INT_ST_OFFSET: u32 = 10;
    pub const SW_XOFF_INT_ST_MASK: u32 = 0x00000400;
    pub const SW_XOFF_INT_ST: u32 = 1 << 10;

    /// **SW_XON_INT_ST**
    ///
    /// This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SW_XON_INT_ST_OFFSET: u32 = 9;
    pub const SW_XON_INT_ST_MASK: u32 = 0x00000200;
    pub const SW_XON_INT_ST: u32 = 1 << 9;

    /// **RXFIFO_TOUT_INT_ST**
    ///
    /// This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RXFIFO_TOUT_INT_ST_OFFSET: u32 = 8;
    pub const RXFIFO_TOUT_INT_ST_MASK: u32 = 0x00000100;
    pub const RXFIFO_TOUT_INT_ST: u32 = 1 << 8;

    /// **BRK_DET_INT_ST**
    ///
    /// This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BRK_DET_INT_ST_OFFSET: u32 = 7;
    pub const BRK_DET_INT_ST_MASK: u32 = 0x00000080;
    pub const BRK_DET_INT_ST: u32 = 1 << 7;

    /// **CTS_CHG_INT_ST**
    ///
    /// This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CTS_CHG_INT_ST_OFFSET: u32 = 6;
    pub const CTS_CHG_INT_ST_MASK: u32 = 0x00000040;
    pub const CTS_CHG_INT_ST: u32 = 1 << 6;

    /// **DSR_CHG_INT_ST**
    ///
    /// This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DSR_CHG_INT_ST_OFFSET: u32 = 5;
    pub const DSR_CHG_INT_ST_MASK: u32 = 0x00000020;
    pub const DSR_CHG_INT_ST: u32 = 1 << 5;

    /// **RXFIFO_OVF_INT_ST**
    ///
    /// This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXFIFO_OVF_INT_ST_OFFSET: u32 = 4;
    pub const RXFIFO_OVF_INT_ST_MASK: u32 = 0x00000010;
    pub const RXFIFO_OVF_INT_ST: u32 = 1 << 4;

    /// **FRM_ERR_INT_ST**
    ///
    /// This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FRM_ERR_INT_ST_OFFSET: u32 = 3;
    pub const FRM_ERR_INT_ST_MASK: u32 = 0x00000008;
    pub const FRM_ERR_INT_ST: u32 = 1 << 3;

    /// **PARITY_ERR_INT_ST**
    ///
    /// This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PARITY_ERR_INT_ST_OFFSET: u32 = 2;
    pub const PARITY_ERR_INT_ST_MASK: u32 = 0x00000004;
    pub const PARITY_ERR_INT_ST: u32 = 1 << 2;

    /// **TXFIFO_EMPTY_INT_ST**
    ///
    /// This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_EMPTY_INT_ST_OFFSET: u32 = 1;
    pub const TXFIFO_EMPTY_INT_ST_MASK: u32 = 0x00000002;
    pub const TXFIFO_EMPTY_INT_ST: u32 = 1 << 1;

    /// **RXFIFO_FULL_INT_ST**
    ///
    /// This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_FULL_INT_ST_OFFSET: u32 = 0;
    pub const RXFIFO_FULL_INT_ST_MASK: u32 = 0x00000001;
    pub const RXFIFO_FULL_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WAKEUP_INT_ST** `[19]` (RO)
    ///
    /// This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1.
    #[inline(always)]
    pub fn wakeup_int_st(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_INT_ST_MASK) != 0
    }

    /// **AT_CMD_CHAR_DET_INT_ST** `[18]` (RO)
    ///
    /// This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1.
    #[inline(always)]
    pub fn at_cmd_char_det_int_st(&self) -> bool {
        (self.value.read_bits() & Self::AT_CMD_CHAR_DET_INT_ST_MASK) != 0
    }

    /// **TX_DONE_INT_ST** `[14]` (RO)
    ///
    /// This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1.
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_ST_MASK) != 0
    }

    /// **TX_BRK_IDLE_DONE_INT_ST** `[13]` (RO)
    ///
    /// This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1.
    #[inline(always)]
    pub fn tx_brk_idle_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_IDLE_DONE_INT_ST_MASK) != 0
    }

    /// **TX_BRK_DONE_INT_ST** `[12]` (RO)
    ///
    /// This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1.
    #[inline(always)]
    pub fn tx_brk_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_DONE_INT_ST_MASK) != 0
    }

    /// **GLITCH_DET_INT_ST** `[11]` (RO)
    ///
    /// This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1.
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> bool {
        (self.value.read_bits() & Self::GLITCH_DET_INT_ST_MASK) != 0
    }

    /// **SW_XOFF_INT_ST** `[10]` (RO)
    ///
    /// This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1.
    #[inline(always)]
    pub fn sw_xoff_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SW_XOFF_INT_ST_MASK) != 0
    }

    /// **SW_XON_INT_ST** `[9]` (RO)
    ///
    /// This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1.
    #[inline(always)]
    pub fn sw_xon_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SW_XON_INT_ST_MASK) != 0
    }

    /// **RXFIFO_TOUT_INT_ST** `[8]` (RO)
    ///
    /// This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1.
    #[inline(always)]
    pub fn rxfifo_tout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_TOUT_INT_ST_MASK) != 0
    }

    /// **BRK_DET_INT_ST** `[7]` (RO)
    ///
    /// This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1.
    #[inline(always)]
    pub fn brk_det_int_st(&self) -> bool {
        (self.value.read_bits() & Self::BRK_DET_INT_ST_MASK) != 0
    }

    /// **CTS_CHG_INT_ST** `[6]` (RO)
    ///
    /// This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1.
    #[inline(always)]
    pub fn cts_chg_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CTS_CHG_INT_ST_MASK) != 0
    }

    /// **DSR_CHG_INT_ST** `[5]` (RO)
    ///
    /// This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1.
    #[inline(always)]
    pub fn dsr_chg_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DSR_CHG_INT_ST_MASK) != 0
    }

    /// **RXFIFO_OVF_INT_ST** `[4]` (RO)
    ///
    /// This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1.
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// **FRM_ERR_INT_ST** `[3]` (RO)
    ///
    /// This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1.
    #[inline(always)]
    pub fn frm_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::FRM_ERR_INT_ST_MASK) != 0
    }

    /// **PARITY_ERR_INT_ST** `[2]` (RO)
    ///
    /// This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1.
    #[inline(always)]
    pub fn parity_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::PARITY_ERR_INT_ST_MASK) != 0
    }

    /// **TXFIFO_EMPTY_INT_ST** `[1]` (RO)
    ///
    /// This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1.
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_EMPTY_INT_ST_MASK) != 0
    }

    /// **RXFIFO_FULL_INT_ST** `[0]` (RO)
    ///
    /// This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1.
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_FULL_INT_ST_MASK) != 0
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

    /// Read **WAKEUP_INT_ST** `[19:19]`
    ///
    /// This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1.
    #[inline(always)]
    pub const fn wakeup_int_st(&self) -> bool {
        (self.bits & IntSt::WAKEUP_INT_ST_MASK) != 0
    }

    /// Read **AT_CMD_CHAR_DET_INT_ST** `[18:18]`
    ///
    /// This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1.
    #[inline(always)]
    pub const fn at_cmd_char_det_int_st(&self) -> bool {
        (self.bits & IntSt::AT_CMD_CHAR_DET_INT_ST_MASK) != 0
    }

    /// Read **TX_DONE_INT_ST** `[14:14]`
    ///
    /// This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1.
    #[inline(always)]
    pub const fn tx_done_int_st(&self) -> bool {
        (self.bits & IntSt::TX_DONE_INT_ST_MASK) != 0
    }

    /// Read **TX_BRK_IDLE_DONE_INT_ST** `[13:13]`
    ///
    /// This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1.
    #[inline(always)]
    pub const fn tx_brk_idle_done_int_st(&self) -> bool {
        (self.bits & IntSt::TX_BRK_IDLE_DONE_INT_ST_MASK) != 0
    }

    /// Read **TX_BRK_DONE_INT_ST** `[12:12]`
    ///
    /// This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1.
    #[inline(always)]
    pub const fn tx_brk_done_int_st(&self) -> bool {
        (self.bits & IntSt::TX_BRK_DONE_INT_ST_MASK) != 0
    }

    /// Read **GLITCH_DET_INT_ST** `[11:11]`
    ///
    /// This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1.
    #[inline(always)]
    pub const fn glitch_det_int_st(&self) -> bool {
        (self.bits & IntSt::GLITCH_DET_INT_ST_MASK) != 0
    }

    /// Read **SW_XOFF_INT_ST** `[10:10]`
    ///
    /// This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1.
    #[inline(always)]
    pub const fn sw_xoff_int_st(&self) -> bool {
        (self.bits & IntSt::SW_XOFF_INT_ST_MASK) != 0
    }

    /// Read **SW_XON_INT_ST** `[9:9]`
    ///
    /// This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1.
    #[inline(always)]
    pub const fn sw_xon_int_st(&self) -> bool {
        (self.bits & IntSt::SW_XON_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_TOUT_INT_ST** `[8:8]`
    ///
    /// This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1.
    #[inline(always)]
    pub const fn rxfifo_tout_int_st(&self) -> bool {
        (self.bits & IntSt::RXFIFO_TOUT_INT_ST_MASK) != 0
    }

    /// Read **BRK_DET_INT_ST** `[7:7]`
    ///
    /// This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1.
    #[inline(always)]
    pub const fn brk_det_int_st(&self) -> bool {
        (self.bits & IntSt::BRK_DET_INT_ST_MASK) != 0
    }

    /// Read **CTS_CHG_INT_ST** `[6:6]`
    ///
    /// This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1.
    #[inline(always)]
    pub const fn cts_chg_int_st(&self) -> bool {
        (self.bits & IntSt::CTS_CHG_INT_ST_MASK) != 0
    }

    /// Read **DSR_CHG_INT_ST** `[5:5]`
    ///
    /// This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1.
    #[inline(always)]
    pub const fn dsr_chg_int_st(&self) -> bool {
        (self.bits & IntSt::DSR_CHG_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_ST** `[4:4]`
    ///
    /// This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::RXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **FRM_ERR_INT_ST** `[3:3]`
    ///
    /// This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1.
    #[inline(always)]
    pub const fn frm_err_int_st(&self) -> bool {
        (self.bits & IntSt::FRM_ERR_INT_ST_MASK) != 0
    }

    /// Read **PARITY_ERR_INT_ST** `[2:2]`
    ///
    /// This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1.
    #[inline(always)]
    pub const fn parity_err_int_st(&self) -> bool {
        (self.bits & IntSt::PARITY_ERR_INT_ST_MASK) != 0
    }

    /// Read **TXFIFO_EMPTY_INT_ST** `[1:1]`
    ///
    /// This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1.
    #[inline(always)]
    pub const fn txfifo_empty_int_st(&self) -> bool {
        (self.bits & IntSt::TXFIFO_EMPTY_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_FULL_INT_ST** `[0:0]`
    ///
    /// This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1.
    #[inline(always)]
    pub const fn rxfifo_full_int_st(&self) -> bool {
        (self.bits & IntSt::RXFIFO_FULL_INT_ST_MASK) != 0
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
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **WAKEUP_INT_ENA** `[19]` (RW)
///
///   This is the enable bit for uart_wakeup_int_st register.
///
/// - **AT_CMD_CHAR_DET_INT_ENA** `[18]` (RW)
///
///   This is the enable bit for at_cmd_char_det_int_st register.
///
/// - **TX_DONE_INT_ENA** `[14]` (RW)
///
///   This is the enable bit for tx_done_int_st register.
///
/// - **TX_BRK_IDLE_DONE_INT_ENA** `[13]` (RW)
///
///   This is the enable bit for tx_brk_idle_done_int_st register.
///
/// - **TX_BRK_DONE_INT_ENA** `[12]` (RW)
///
///   This is the enable bit for tx_brk_done_int_st register.
///
/// - **GLITCH_DET_INT_ENA** `[11]` (RW)
///
///   This is the enable bit for glitch_det_int_st register.
///
/// - **SW_XOFF_INT_ENA** `[10]` (RW)
///
///   This is the enable bit for sw_xoff_int_st register.
///
/// - **SW_XON_INT_ENA** `[9]` (RW)
///
///   This is the enable bit for sw_xon_int_st register.
///
/// - **RXFIFO_TOUT_INT_ENA** `[8]` (RW)
///
///   This is the enable bit for rxfifo_tout_int_st register.
///
/// - **BRK_DET_INT_ENA** `[7]` (RW)
///
///   This is the enable bit for brk_det_int_st register.
///
/// - **CTS_CHG_INT_ENA** `[6]` (RW)
///
///   This is the enable bit for cts_chg_int_st register.
///
/// - **DSR_CHG_INT_ENA** `[5]` (RW)
///
///   This is the enable bit for dsr_chg_int_st register.
///
/// - **RXFIFO_OVF_INT_ENA** `[4]` (RW)
///
///   This is the enable bit for rxfifo_ovf_int_st register.
///
/// - **FRM_ERR_INT_ENA** `[3]` (RW)
///
///   This is the enable bit for frm_err_int_st register.
///
/// - **PARITY_ERR_INT_ENA** `[2]` (RW)
///
///   This is the enable bit for parity_err_int_st register.
///
/// - **TXFIFO_EMPTY_INT_ENA** `[1]` (RW)
///
///   This is the enable bit for txfifo_empty_int_st register.
///
/// - **RXFIFO_FULL_INT_ENA** `[0]` (RW)
///
///   This is the enable bit for rxfifo_full_int_st register.
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

    /// **WAKEUP_INT_ENA**
    ///
    /// This is the enable bit for uart_wakeup_int_st register.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const WAKEUP_INT_ENA_OFFSET: u32 = 19;
    pub const WAKEUP_INT_ENA_MASK: u32 = 0x00080000;
    pub const WAKEUP_INT_ENA: u32 = 1 << 19;

    /// **AT_CMD_CHAR_DET_INT_ENA**
    ///
    /// This is the enable bit for at_cmd_char_det_int_st register.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const AT_CMD_CHAR_DET_INT_ENA_OFFSET: u32 = 18;
    pub const AT_CMD_CHAR_DET_INT_ENA_MASK: u32 = 0x00040000;
    pub const AT_CMD_CHAR_DET_INT_ENA: u32 = 1 << 18;

    /// **TX_DONE_INT_ENA**
    ///
    /// This is the enable bit for tx_done_int_st register.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_DONE_INT_ENA_OFFSET: u32 = 14;
    pub const TX_DONE_INT_ENA_MASK: u32 = 0x00004000;
    pub const TX_DONE_INT_ENA: u32 = 1 << 14;

    /// **TX_BRK_IDLE_DONE_INT_ENA**
    ///
    /// This is the enable bit for tx_brk_idle_done_int_st register.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_BRK_IDLE_DONE_INT_ENA_OFFSET: u32 = 13;
    pub const TX_BRK_IDLE_DONE_INT_ENA_MASK: u32 = 0x00002000;
    pub const TX_BRK_IDLE_DONE_INT_ENA: u32 = 1 << 13;

    /// **TX_BRK_DONE_INT_ENA**
    ///
    /// This is the enable bit for tx_brk_done_int_st register.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_BRK_DONE_INT_ENA_OFFSET: u32 = 12;
    pub const TX_BRK_DONE_INT_ENA_MASK: u32 = 0x00001000;
    pub const TX_BRK_DONE_INT_ENA: u32 = 1 << 12;

    /// **GLITCH_DET_INT_ENA**
    ///
    /// This is the enable bit for glitch_det_int_st register.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GLITCH_DET_INT_ENA_OFFSET: u32 = 11;
    pub const GLITCH_DET_INT_ENA_MASK: u32 = 0x00000800;
    pub const GLITCH_DET_INT_ENA: u32 = 1 << 11;

    /// **SW_XOFF_INT_ENA**
    ///
    /// This is the enable bit for sw_xoff_int_st register.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SW_XOFF_INT_ENA_OFFSET: u32 = 10;
    pub const SW_XOFF_INT_ENA_MASK: u32 = 0x00000400;
    pub const SW_XOFF_INT_ENA: u32 = 1 << 10;

    /// **SW_XON_INT_ENA**
    ///
    /// This is the enable bit for sw_xon_int_st register.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SW_XON_INT_ENA_OFFSET: u32 = 9;
    pub const SW_XON_INT_ENA_MASK: u32 = 0x00000200;
    pub const SW_XON_INT_ENA: u32 = 1 << 9;

    /// **RXFIFO_TOUT_INT_ENA**
    ///
    /// This is the enable bit for rxfifo_tout_int_st register.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RXFIFO_TOUT_INT_ENA_OFFSET: u32 = 8;
    pub const RXFIFO_TOUT_INT_ENA_MASK: u32 = 0x00000100;
    pub const RXFIFO_TOUT_INT_ENA: u32 = 1 << 8;

    /// **BRK_DET_INT_ENA**
    ///
    /// This is the enable bit for brk_det_int_st register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BRK_DET_INT_ENA_OFFSET: u32 = 7;
    pub const BRK_DET_INT_ENA_MASK: u32 = 0x00000080;
    pub const BRK_DET_INT_ENA: u32 = 1 << 7;

    /// **CTS_CHG_INT_ENA**
    ///
    /// This is the enable bit for cts_chg_int_st register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CTS_CHG_INT_ENA_OFFSET: u32 = 6;
    pub const CTS_CHG_INT_ENA_MASK: u32 = 0x00000040;
    pub const CTS_CHG_INT_ENA: u32 = 1 << 6;

    /// **DSR_CHG_INT_ENA**
    ///
    /// This is the enable bit for dsr_chg_int_st register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DSR_CHG_INT_ENA_OFFSET: u32 = 5;
    pub const DSR_CHG_INT_ENA_MASK: u32 = 0x00000020;
    pub const DSR_CHG_INT_ENA: u32 = 1 << 5;

    /// **RXFIFO_OVF_INT_ENA**
    ///
    /// This is the enable bit for rxfifo_ovf_int_st register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXFIFO_OVF_INT_ENA_OFFSET: u32 = 4;
    pub const RXFIFO_OVF_INT_ENA_MASK: u32 = 0x00000010;
    pub const RXFIFO_OVF_INT_ENA: u32 = 1 << 4;

    /// **FRM_ERR_INT_ENA**
    ///
    /// This is the enable bit for frm_err_int_st register.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FRM_ERR_INT_ENA_OFFSET: u32 = 3;
    pub const FRM_ERR_INT_ENA_MASK: u32 = 0x00000008;
    pub const FRM_ERR_INT_ENA: u32 = 1 << 3;

    /// **PARITY_ERR_INT_ENA**
    ///
    /// This is the enable bit for parity_err_int_st register.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PARITY_ERR_INT_ENA_OFFSET: u32 = 2;
    pub const PARITY_ERR_INT_ENA_MASK: u32 = 0x00000004;
    pub const PARITY_ERR_INT_ENA: u32 = 1 << 2;

    /// **TXFIFO_EMPTY_INT_ENA**
    ///
    /// This is the enable bit for txfifo_empty_int_st register.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_EMPTY_INT_ENA_OFFSET: u32 = 1;
    pub const TXFIFO_EMPTY_INT_ENA_MASK: u32 = 0x00000002;
    pub const TXFIFO_EMPTY_INT_ENA: u32 = 1 << 1;

    /// **RXFIFO_FULL_INT_ENA**
    ///
    /// This is the enable bit for rxfifo_full_int_st register.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_FULL_INT_ENA_OFFSET: u32 = 0;
    pub const RXFIFO_FULL_INT_ENA_MASK: u32 = 0x00000001;
    pub const RXFIFO_FULL_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WAKEUP_INT_ENA** `[19]` (RW)
    ///
    /// This is the enable bit for uart_wakeup_int_st register.
    #[inline(always)]
    pub fn wakeup_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_INT_ENA_MASK) != 0
    }

    /// Set WAKEUP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wakeup_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::WAKEUP_INT_ENA_MASK);
    }

    /// Clear WAKEUP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wakeup_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::WAKEUP_INT_ENA_MASK);
    }

    /// **AT_CMD_CHAR_DET_INT_ENA** `[18]` (RW)
    ///
    /// This is the enable bit for at_cmd_char_det_int_st register.
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::AT_CMD_CHAR_DET_INT_ENA_MASK) != 0
    }

    /// Set AT_CMD_CHAR_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_at_cmd_char_det_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::AT_CMD_CHAR_DET_INT_ENA_MASK);
    }

    /// Clear AT_CMD_CHAR_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_at_cmd_char_det_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::AT_CMD_CHAR_DET_INT_ENA_MASK);
    }

    /// **TX_DONE_INT_ENA** `[14]` (RW)
    ///
    /// This is the enable bit for tx_done_int_st register.
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_DONE_INT_ENA_MASK) != 0
    }

    /// Set TX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_DONE_INT_ENA_MASK);
    }

    /// Clear TX_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_DONE_INT_ENA_MASK);
    }

    /// **TX_BRK_IDLE_DONE_INT_ENA** `[13]` (RW)
    ///
    /// This is the enable bit for tx_brk_idle_done_int_st register.
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_IDLE_DONE_INT_ENA_MASK) != 0
    }

    /// Set TX_BRK_IDLE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_brk_idle_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_BRK_IDLE_DONE_INT_ENA_MASK);
    }

    /// Clear TX_BRK_IDLE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_brk_idle_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_BRK_IDLE_DONE_INT_ENA_MASK);
    }

    /// **TX_BRK_DONE_INT_ENA** `[12]` (RW)
    ///
    /// This is the enable bit for tx_brk_done_int_st register.
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_BRK_DONE_INT_ENA_MASK) != 0
    }

    /// Set TX_BRK_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_brk_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_BRK_DONE_INT_ENA_MASK);
    }

    /// Clear TX_BRK_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_brk_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_BRK_DONE_INT_ENA_MASK);
    }

    /// **GLITCH_DET_INT_ENA** `[11]` (RW)
    ///
    /// This is the enable bit for glitch_det_int_st register.
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::GLITCH_DET_INT_ENA_MASK) != 0
    }

    /// Set GLITCH_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_glitch_det_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::GLITCH_DET_INT_ENA_MASK);
    }

    /// Clear GLITCH_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_glitch_det_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::GLITCH_DET_INT_ENA_MASK);
    }

    /// **SW_XOFF_INT_ENA** `[10]` (RW)
    ///
    /// This is the enable bit for sw_xoff_int_st register.
    #[inline(always)]
    pub fn sw_xoff_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SW_XOFF_INT_ENA_MASK) != 0
    }

    /// Set SW_XOFF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_xoff_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SW_XOFF_INT_ENA_MASK);
    }

    /// Clear SW_XOFF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_xoff_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SW_XOFF_INT_ENA_MASK);
    }

    /// **SW_XON_INT_ENA** `[9]` (RW)
    ///
    /// This is the enable bit for sw_xon_int_st register.
    #[inline(always)]
    pub fn sw_xon_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SW_XON_INT_ENA_MASK) != 0
    }

    /// Set SW_XON_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_xon_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SW_XON_INT_ENA_MASK);
    }

    /// Clear SW_XON_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_xon_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SW_XON_INT_ENA_MASK);
    }

    /// **RXFIFO_TOUT_INT_ENA** `[8]` (RW)
    ///
    /// This is the enable bit for rxfifo_tout_int_st register.
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_TOUT_INT_ENA_MASK) != 0
    }

    /// Set RXFIFO_TOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_tout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_TOUT_INT_ENA_MASK);
    }

    /// Clear RXFIFO_TOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_tout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_TOUT_INT_ENA_MASK);
    }

    /// **BRK_DET_INT_ENA** `[7]` (RW)
    ///
    /// This is the enable bit for brk_det_int_st register.
    #[inline(always)]
    pub fn brk_det_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::BRK_DET_INT_ENA_MASK) != 0
    }

    /// Set BRK_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_brk_det_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::BRK_DET_INT_ENA_MASK);
    }

    /// Clear BRK_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_brk_det_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::BRK_DET_INT_ENA_MASK);
    }

    /// **CTS_CHG_INT_ENA** `[6]` (RW)
    ///
    /// This is the enable bit for cts_chg_int_st register.
    #[inline(always)]
    pub fn cts_chg_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CTS_CHG_INT_ENA_MASK) != 0
    }

    /// Set CTS_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cts_chg_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CTS_CHG_INT_ENA_MASK);
    }

    /// Clear CTS_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cts_chg_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CTS_CHG_INT_ENA_MASK);
    }

    /// **DSR_CHG_INT_ENA** `[5]` (RW)
    ///
    /// This is the enable bit for dsr_chg_int_st register.
    #[inline(always)]
    pub fn dsr_chg_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DSR_CHG_INT_ENA_MASK) != 0
    }

    /// Set DSR_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsr_chg_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DSR_CHG_INT_ENA_MASK);
    }

    /// Clear DSR_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsr_chg_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DSR_CHG_INT_ENA_MASK);
    }

    /// **RXFIFO_OVF_INT_ENA** `[4]` (RW)
    ///
    /// This is the enable bit for rxfifo_ovf_int_st register.
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Set RXFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_OVF_INT_ENA_MASK);
    }

    /// Clear RXFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_OVF_INT_ENA_MASK);
    }

    /// **FRM_ERR_INT_ENA** `[3]` (RW)
    ///
    /// This is the enable bit for frm_err_int_st register.
    #[inline(always)]
    pub fn frm_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::FRM_ERR_INT_ENA_MASK) != 0
    }

    /// Set FRM_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frm_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::FRM_ERR_INT_ENA_MASK);
    }

    /// Clear FRM_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frm_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::FRM_ERR_INT_ENA_MASK);
    }

    /// **PARITY_ERR_INT_ENA** `[2]` (RW)
    ///
    /// This is the enable bit for parity_err_int_st register.
    #[inline(always)]
    pub fn parity_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::PARITY_ERR_INT_ENA_MASK) != 0
    }

    /// Set PARITY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_parity_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::PARITY_ERR_INT_ENA_MASK);
    }

    /// Clear PARITY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_parity_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::PARITY_ERR_INT_ENA_MASK);
    }

    /// **TXFIFO_EMPTY_INT_ENA** `[1]` (RW)
    ///
    /// This is the enable bit for txfifo_empty_int_st register.
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_EMPTY_INT_ENA_MASK) != 0
    }

    /// Set TXFIFO_EMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_empty_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TXFIFO_EMPTY_INT_ENA_MASK);
    }

    /// Clear TXFIFO_EMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_empty_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TXFIFO_EMPTY_INT_ENA_MASK);
    }

    /// **RXFIFO_FULL_INT_ENA** `[0]` (RW)
    ///
    /// This is the enable bit for rxfifo_full_int_st register.
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_FULL_INT_ENA_MASK) != 0
    }

    /// Set RXFIFO_FULL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_full_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_FULL_INT_ENA_MASK);
    }

    /// Clear RXFIFO_FULL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_full_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_FULL_INT_ENA_MASK);
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

    /// Read **WAKEUP_INT_ENA** `[19:19]`
    ///
    /// This is the enable bit for uart_wakeup_int_st register.
    #[inline(always)]
    pub const fn wakeup_int_ena(&self) -> bool {
        (self.bits & IntEna::WAKEUP_INT_ENA_MASK) != 0
    }

    /// Read **AT_CMD_CHAR_DET_INT_ENA** `[18:18]`
    ///
    /// This is the enable bit for at_cmd_char_det_int_st register.
    #[inline(always)]
    pub const fn at_cmd_char_det_int_ena(&self) -> bool {
        (self.bits & IntEna::AT_CMD_CHAR_DET_INT_ENA_MASK) != 0
    }

    /// Read **TX_DONE_INT_ENA** `[14:14]`
    ///
    /// This is the enable bit for tx_done_int_st register.
    #[inline(always)]
    pub const fn tx_done_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_DONE_INT_ENA_MASK) != 0
    }

    /// Read **TX_BRK_IDLE_DONE_INT_ENA** `[13:13]`
    ///
    /// This is the enable bit for tx_brk_idle_done_int_st register.
    #[inline(always)]
    pub const fn tx_brk_idle_done_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_BRK_IDLE_DONE_INT_ENA_MASK) != 0
    }

    /// Read **TX_BRK_DONE_INT_ENA** `[12:12]`
    ///
    /// This is the enable bit for tx_brk_done_int_st register.
    #[inline(always)]
    pub const fn tx_brk_done_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_BRK_DONE_INT_ENA_MASK) != 0
    }

    /// Read **GLITCH_DET_INT_ENA** `[11:11]`
    ///
    /// This is the enable bit for glitch_det_int_st register.
    #[inline(always)]
    pub const fn glitch_det_int_ena(&self) -> bool {
        (self.bits & IntEna::GLITCH_DET_INT_ENA_MASK) != 0
    }

    /// Read **SW_XOFF_INT_ENA** `[10:10]`
    ///
    /// This is the enable bit for sw_xoff_int_st register.
    #[inline(always)]
    pub const fn sw_xoff_int_ena(&self) -> bool {
        (self.bits & IntEna::SW_XOFF_INT_ENA_MASK) != 0
    }

    /// Read **SW_XON_INT_ENA** `[9:9]`
    ///
    /// This is the enable bit for sw_xon_int_st register.
    #[inline(always)]
    pub const fn sw_xon_int_ena(&self) -> bool {
        (self.bits & IntEna::SW_XON_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_TOUT_INT_ENA** `[8:8]`
    ///
    /// This is the enable bit for rxfifo_tout_int_st register.
    #[inline(always)]
    pub const fn rxfifo_tout_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_TOUT_INT_ENA_MASK) != 0
    }

    /// Read **BRK_DET_INT_ENA** `[7:7]`
    ///
    /// This is the enable bit for brk_det_int_st register.
    #[inline(always)]
    pub const fn brk_det_int_ena(&self) -> bool {
        (self.bits & IntEna::BRK_DET_INT_ENA_MASK) != 0
    }

    /// Read **CTS_CHG_INT_ENA** `[6:6]`
    ///
    /// This is the enable bit for cts_chg_int_st register.
    #[inline(always)]
    pub const fn cts_chg_int_ena(&self) -> bool {
        (self.bits & IntEna::CTS_CHG_INT_ENA_MASK) != 0
    }

    /// Read **DSR_CHG_INT_ENA** `[5:5]`
    ///
    /// This is the enable bit for dsr_chg_int_st register.
    #[inline(always)]
    pub const fn dsr_chg_int_ena(&self) -> bool {
        (self.bits & IntEna::DSR_CHG_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_ENA** `[4:4]`
    ///
    /// This is the enable bit for rxfifo_ovf_int_st register.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **FRM_ERR_INT_ENA** `[3:3]`
    ///
    /// This is the enable bit for frm_err_int_st register.
    #[inline(always)]
    pub const fn frm_err_int_ena(&self) -> bool {
        (self.bits & IntEna::FRM_ERR_INT_ENA_MASK) != 0
    }

    /// Read **PARITY_ERR_INT_ENA** `[2:2]`
    ///
    /// This is the enable bit for parity_err_int_st register.
    #[inline(always)]
    pub const fn parity_err_int_ena(&self) -> bool {
        (self.bits & IntEna::PARITY_ERR_INT_ENA_MASK) != 0
    }

    /// Read **TXFIFO_EMPTY_INT_ENA** `[1:1]`
    ///
    /// This is the enable bit for txfifo_empty_int_st register.
    #[inline(always)]
    pub const fn txfifo_empty_int_ena(&self) -> bool {
        (self.bits & IntEna::TXFIFO_EMPTY_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_FULL_INT_ENA** `[0:0]`
    ///
    /// This is the enable bit for rxfifo_full_int_st register.
    #[inline(always)]
    pub const fn rxfifo_full_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_FULL_INT_ENA_MASK) != 0
    }

    /// Set **WAKEUP_INT_ENA** `[19:19]`
    ///
    /// This is the enable bit for uart_wakeup_int_st register.
    #[inline(always)]
    pub const fn with_wakeup_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::WAKEUP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::WAKEUP_INT_ENA_MASK
            },
        }
    }

    /// Set **AT_CMD_CHAR_DET_INT_ENA** `[18:18]`
    ///
    /// This is the enable bit for at_cmd_char_det_int_st register.
    #[inline(always)]
    pub const fn with_at_cmd_char_det_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::AT_CMD_CHAR_DET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::AT_CMD_CHAR_DET_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_DONE_INT_ENA** `[14:14]`
    ///
    /// This is the enable bit for tx_done_int_st register.
    #[inline(always)]
    pub const fn with_tx_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_BRK_IDLE_DONE_INT_ENA** `[13:13]`
    ///
    /// This is the enable bit for tx_brk_idle_done_int_st register.
    #[inline(always)]
    pub const fn with_tx_brk_idle_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_BRK_IDLE_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_BRK_IDLE_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_BRK_DONE_INT_ENA** `[12:12]`
    ///
    /// This is the enable bit for tx_brk_done_int_st register.
    #[inline(always)]
    pub const fn with_tx_brk_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_BRK_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_BRK_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **GLITCH_DET_INT_ENA** `[11:11]`
    ///
    /// This is the enable bit for glitch_det_int_st register.
    #[inline(always)]
    pub const fn with_glitch_det_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::GLITCH_DET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::GLITCH_DET_INT_ENA_MASK
            },
        }
    }

    /// Set **SW_XOFF_INT_ENA** `[10:10]`
    ///
    /// This is the enable bit for sw_xoff_int_st register.
    #[inline(always)]
    pub const fn with_sw_xoff_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SW_XOFF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SW_XOFF_INT_ENA_MASK
            },
        }
    }

    /// Set **SW_XON_INT_ENA** `[9:9]`
    ///
    /// This is the enable bit for sw_xon_int_st register.
    #[inline(always)]
    pub const fn with_sw_xon_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SW_XON_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SW_XON_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_TOUT_INT_ENA** `[8:8]`
    ///
    /// This is the enable bit for rxfifo_tout_int_st register.
    #[inline(always)]
    pub const fn with_rxfifo_tout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RXFIFO_TOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RXFIFO_TOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **BRK_DET_INT_ENA** `[7:7]`
    ///
    /// This is the enable bit for brk_det_int_st register.
    #[inline(always)]
    pub const fn with_brk_det_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::BRK_DET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::BRK_DET_INT_ENA_MASK
            },
        }
    }

    /// Set **CTS_CHG_INT_ENA** `[6:6]`
    ///
    /// This is the enable bit for cts_chg_int_st register.
    #[inline(always)]
    pub const fn with_cts_chg_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CTS_CHG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CTS_CHG_INT_ENA_MASK
            },
        }
    }

    /// Set **DSR_CHG_INT_ENA** `[5:5]`
    ///
    /// This is the enable bit for dsr_chg_int_st register.
    #[inline(always)]
    pub const fn with_dsr_chg_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DSR_CHG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DSR_CHG_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_OVF_INT_ENA** `[4:4]`
    ///
    /// This is the enable bit for rxfifo_ovf_int_st register.
    #[inline(always)]
    pub const fn with_rxfifo_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RXFIFO_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RXFIFO_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **FRM_ERR_INT_ENA** `[3:3]`
    ///
    /// This is the enable bit for frm_err_int_st register.
    #[inline(always)]
    pub const fn with_frm_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::FRM_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::FRM_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **PARITY_ERR_INT_ENA** `[2:2]`
    ///
    /// This is the enable bit for parity_err_int_st register.
    #[inline(always)]
    pub const fn with_parity_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::PARITY_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::PARITY_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **TXFIFO_EMPTY_INT_ENA** `[1:1]`
    ///
    /// This is the enable bit for txfifo_empty_int_st register.
    #[inline(always)]
    pub const fn with_txfifo_empty_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TXFIFO_EMPTY_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TXFIFO_EMPTY_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_FULL_INT_ENA** `[0:0]`
    ///
    /// This is the enable bit for rxfifo_full_int_st register.
    #[inline(always)]
    pub const fn with_rxfifo_full_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RXFIFO_FULL_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RXFIFO_FULL_INT_ENA_MASK
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
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **WAKEUP_INT_CLR** `[19]` (WO)
///
///   Set this bit to clear the uart_wakeup_int_raw interrupt.
///
/// - **AT_CMD_CHAR_DET_INT_CLR** `[18]` (WO)
///
///   Set this bit to clear the at_cmd_char_det_int_raw interrupt.
///
/// - **TX_DONE_INT_CLR** `[14]` (WO)
///
///   Set this bit to clear the tx_done_int_raw interrupt.
///
/// - **TX_BRK_IDLE_DONE_INT_CLR** `[13]` (WO)
///
///   Set this bit to clear the tx_brk_idle_done_int_raw interrupt.
///
/// - **TX_BRK_DONE_INT_CLR** `[12]` (WO)
///
///   Set this bit to clear the tx_brk_done_int_raw interrupt..
///
/// - **GLITCH_DET_INT_CLR** `[11]` (WO)
///
///   Set this bit to clear the glitch_det_int_raw interrupt.
///
/// - **SW_XOFF_INT_CLR** `[10]` (WO)
///
///   Set this bit to clear the sw_xoff_int_raw interrupt.
///
/// - **SW_XON_INT_CLR** `[9]` (WO)
///
///   Set this bit to clear the sw_xon_int_raw interrupt.
///
/// - **RXFIFO_TOUT_INT_CLR** `[8]` (WO)
///
///   Set this bit to clear the rxfifo_tout_int_raw interrupt.
///
/// - **BRK_DET_INT_CLR** `[7]` (WO)
///
///   Set this bit to clear the brk_det_int_raw interrupt.
///
/// - **CTS_CHG_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the cts_chg_int_raw interrupt.
///
/// - **DSR_CHG_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the dsr_chg_int_raw interrupt.
///
/// - **RXFIFO_OVF_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear rxfifo_ovf_int_raw interrupt.
///
/// - **FRM_ERR_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear frm_err_int_raw interrupt.
///
/// - **PARITY_ERR_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear parity_err_int_raw interrupt.
///
/// - **TXFIFO_EMPTY_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear txfifo_empty_int_raw interrupt.
///
/// - **RXFIFO_FULL_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the rxfifo_full_int_raw interrupt.
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

    /// **WAKEUP_INT_CLR**
    ///
    /// Set this bit to clear the uart_wakeup_int_raw interrupt.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const WAKEUP_INT_CLR_OFFSET: u32 = 19;
    pub const WAKEUP_INT_CLR_MASK: u32 = 0x00080000;
    pub const WAKEUP_INT_CLR: u32 = 1 << 19;

    /// **AT_CMD_CHAR_DET_INT_CLR**
    ///
    /// Set this bit to clear the at_cmd_char_det_int_raw interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const AT_CMD_CHAR_DET_INT_CLR_OFFSET: u32 = 18;
    pub const AT_CMD_CHAR_DET_INT_CLR_MASK: u32 = 0x00040000;
    pub const AT_CMD_CHAR_DET_INT_CLR: u32 = 1 << 18;

    /// **TX_DONE_INT_CLR**
    ///
    /// Set this bit to clear the tx_done_int_raw interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_DONE_INT_CLR_OFFSET: u32 = 14;
    pub const TX_DONE_INT_CLR_MASK: u32 = 0x00004000;
    pub const TX_DONE_INT_CLR: u32 = 1 << 14;

    /// **TX_BRK_IDLE_DONE_INT_CLR**
    ///
    /// Set this bit to clear the tx_brk_idle_done_int_raw interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_BRK_IDLE_DONE_INT_CLR_OFFSET: u32 = 13;
    pub const TX_BRK_IDLE_DONE_INT_CLR_MASK: u32 = 0x00002000;
    pub const TX_BRK_IDLE_DONE_INT_CLR: u32 = 1 << 13;

    /// **TX_BRK_DONE_INT_CLR**
    ///
    /// Set this bit to clear the tx_brk_done_int_raw interrupt..
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TX_BRK_DONE_INT_CLR_OFFSET: u32 = 12;
    pub const TX_BRK_DONE_INT_CLR_MASK: u32 = 0x00001000;
    pub const TX_BRK_DONE_INT_CLR: u32 = 1 << 12;

    /// **GLITCH_DET_INT_CLR**
    ///
    /// Set this bit to clear the glitch_det_int_raw interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GLITCH_DET_INT_CLR_OFFSET: u32 = 11;
    pub const GLITCH_DET_INT_CLR_MASK: u32 = 0x00000800;
    pub const GLITCH_DET_INT_CLR: u32 = 1 << 11;

    /// **SW_XOFF_INT_CLR**
    ///
    /// Set this bit to clear the sw_xoff_int_raw interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SW_XOFF_INT_CLR_OFFSET: u32 = 10;
    pub const SW_XOFF_INT_CLR_MASK: u32 = 0x00000400;
    pub const SW_XOFF_INT_CLR: u32 = 1 << 10;

    /// **SW_XON_INT_CLR**
    ///
    /// Set this bit to clear the sw_xon_int_raw interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SW_XON_INT_CLR_OFFSET: u32 = 9;
    pub const SW_XON_INT_CLR_MASK: u32 = 0x00000200;
    pub const SW_XON_INT_CLR: u32 = 1 << 9;

    /// **RXFIFO_TOUT_INT_CLR**
    ///
    /// Set this bit to clear the rxfifo_tout_int_raw interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RXFIFO_TOUT_INT_CLR_OFFSET: u32 = 8;
    pub const RXFIFO_TOUT_INT_CLR_MASK: u32 = 0x00000100;
    pub const RXFIFO_TOUT_INT_CLR: u32 = 1 << 8;

    /// **BRK_DET_INT_CLR**
    ///
    /// Set this bit to clear the brk_det_int_raw interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BRK_DET_INT_CLR_OFFSET: u32 = 7;
    pub const BRK_DET_INT_CLR_MASK: u32 = 0x00000080;
    pub const BRK_DET_INT_CLR: u32 = 1 << 7;

    /// **CTS_CHG_INT_CLR**
    ///
    /// Set this bit to clear the cts_chg_int_raw interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CTS_CHG_INT_CLR_OFFSET: u32 = 6;
    pub const CTS_CHG_INT_CLR_MASK: u32 = 0x00000040;
    pub const CTS_CHG_INT_CLR: u32 = 1 << 6;

    /// **DSR_CHG_INT_CLR**
    ///
    /// Set this bit to clear the dsr_chg_int_raw interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DSR_CHG_INT_CLR_OFFSET: u32 = 5;
    pub const DSR_CHG_INT_CLR_MASK: u32 = 0x00000020;
    pub const DSR_CHG_INT_CLR: u32 = 1 << 5;

    /// **RXFIFO_OVF_INT_CLR**
    ///
    /// Set this bit to clear rxfifo_ovf_int_raw interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXFIFO_OVF_INT_CLR_OFFSET: u32 = 4;
    pub const RXFIFO_OVF_INT_CLR_MASK: u32 = 0x00000010;
    pub const RXFIFO_OVF_INT_CLR: u32 = 1 << 4;

    /// **FRM_ERR_INT_CLR**
    ///
    /// Set this bit to clear frm_err_int_raw interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FRM_ERR_INT_CLR_OFFSET: u32 = 3;
    pub const FRM_ERR_INT_CLR_MASK: u32 = 0x00000008;
    pub const FRM_ERR_INT_CLR: u32 = 1 << 3;

    /// **PARITY_ERR_INT_CLR**
    ///
    /// Set this bit to clear parity_err_int_raw interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PARITY_ERR_INT_CLR_OFFSET: u32 = 2;
    pub const PARITY_ERR_INT_CLR_MASK: u32 = 0x00000004;
    pub const PARITY_ERR_INT_CLR: u32 = 1 << 2;

    /// **TXFIFO_EMPTY_INT_CLR**
    ///
    /// Set this bit to clear txfifo_empty_int_raw interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_EMPTY_INT_CLR_OFFSET: u32 = 1;
    pub const TXFIFO_EMPTY_INT_CLR_MASK: u32 = 0x00000002;
    pub const TXFIFO_EMPTY_INT_CLR: u32 = 1 << 1;

    /// **RXFIFO_FULL_INT_CLR**
    ///
    /// Set this bit to clear the rxfifo_full_int_raw interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_FULL_INT_CLR_OFFSET: u32 = 0;
    pub const RXFIFO_FULL_INT_CLR_MASK: u32 = 0x00000001;
    pub const RXFIFO_FULL_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **WAKEUP_INT_CLR** (WO)
    ///
    /// Set this bit to clear the uart_wakeup_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_wakeup_int_clr(&self) {
        self.value.write_bits(Self::WAKEUP_INT_CLR_MASK);
    }

    /// Trigger **AT_CMD_CHAR_DET_INT_CLR** (WO)
    ///
    /// Set this bit to clear the at_cmd_char_det_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_at_cmd_char_det_int_clr(&self) {
        self.value.write_bits(Self::AT_CMD_CHAR_DET_INT_CLR_MASK);
    }

    /// Trigger **TX_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the tx_done_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_tx_done_int_clr(&self) {
        self.value.write_bits(Self::TX_DONE_INT_CLR_MASK);
    }

    /// Trigger **TX_BRK_IDLE_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the tx_brk_idle_done_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_tx_brk_idle_done_int_clr(&self) {
        self.value.write_bits(Self::TX_BRK_IDLE_DONE_INT_CLR_MASK);
    }

    /// Trigger **TX_BRK_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the tx_brk_done_int_raw interrupt..
    #[inline(always)]
    pub fn trigger_tx_brk_done_int_clr(&self) {
        self.value.write_bits(Self::TX_BRK_DONE_INT_CLR_MASK);
    }

    /// Trigger **GLITCH_DET_INT_CLR** (WO)
    ///
    /// Set this bit to clear the glitch_det_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_glitch_det_int_clr(&self) {
        self.value.write_bits(Self::GLITCH_DET_INT_CLR_MASK);
    }

    /// Trigger **SW_XOFF_INT_CLR** (WO)
    ///
    /// Set this bit to clear the sw_xoff_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_sw_xoff_int_clr(&self) {
        self.value.write_bits(Self::SW_XOFF_INT_CLR_MASK);
    }

    /// Trigger **SW_XON_INT_CLR** (WO)
    ///
    /// Set this bit to clear the sw_xon_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_sw_xon_int_clr(&self) {
        self.value.write_bits(Self::SW_XON_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_TOUT_INT_CLR** (WO)
    ///
    /// Set this bit to clear the rxfifo_tout_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_tout_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_TOUT_INT_CLR_MASK);
    }

    /// Trigger **BRK_DET_INT_CLR** (WO)
    ///
    /// Set this bit to clear the brk_det_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_brk_det_int_clr(&self) {
        self.value.write_bits(Self::BRK_DET_INT_CLR_MASK);
    }

    /// Trigger **CTS_CHG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the cts_chg_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_cts_chg_int_clr(&self) {
        self.value.write_bits(Self::CTS_CHG_INT_CLR_MASK);
    }

    /// Trigger **DSR_CHG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the dsr_chg_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_dsr_chg_int_clr(&self) {
        self.value.write_bits(Self::DSR_CHG_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_OVF_INT_CLR** (WO)
    ///
    /// Set this bit to clear rxfifo_ovf_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_ovf_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **FRM_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear frm_err_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_frm_err_int_clr(&self) {
        self.value.write_bits(Self::FRM_ERR_INT_CLR_MASK);
    }

    /// Trigger **PARITY_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear parity_err_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_parity_err_int_clr(&self) {
        self.value.write_bits(Self::PARITY_ERR_INT_CLR_MASK);
    }

    /// Trigger **TXFIFO_EMPTY_INT_CLR** (WO)
    ///
    /// Set this bit to clear txfifo_empty_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_txfifo_empty_int_clr(&self) {
        self.value.write_bits(Self::TXFIFO_EMPTY_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_FULL_INT_CLR** (WO)
    ///
    /// Set this bit to clear the rxfifo_full_int_raw interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_full_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_FULL_INT_CLR_MASK);
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

    /// Set **WAKEUP_INT_CLR** `[19:19]`
    ///
    /// Set this bit to clear the uart_wakeup_int_raw interrupt.
    #[inline(always)]
    pub const fn with_wakeup_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::WAKEUP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::WAKEUP_INT_CLR_MASK
            },
        }
    }

    /// Set **AT_CMD_CHAR_DET_INT_CLR** `[18:18]`
    ///
    /// Set this bit to clear the at_cmd_char_det_int_raw interrupt.
    #[inline(always)]
    pub const fn with_at_cmd_char_det_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::AT_CMD_CHAR_DET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::AT_CMD_CHAR_DET_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_DONE_INT_CLR** `[14:14]`
    ///
    /// Set this bit to clear the tx_done_int_raw interrupt.
    #[inline(always)]
    pub const fn with_tx_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_BRK_IDLE_DONE_INT_CLR** `[13:13]`
    ///
    /// Set this bit to clear the tx_brk_idle_done_int_raw interrupt.
    #[inline(always)]
    pub const fn with_tx_brk_idle_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_BRK_IDLE_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_BRK_IDLE_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_BRK_DONE_INT_CLR** `[12:12]`
    ///
    /// Set this bit to clear the tx_brk_done_int_raw interrupt..
    #[inline(always)]
    pub const fn with_tx_brk_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_BRK_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_BRK_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **GLITCH_DET_INT_CLR** `[11:11]`
    ///
    /// Set this bit to clear the glitch_det_int_raw interrupt.
    #[inline(always)]
    pub const fn with_glitch_det_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::GLITCH_DET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::GLITCH_DET_INT_CLR_MASK
            },
        }
    }

    /// Set **SW_XOFF_INT_CLR** `[10:10]`
    ///
    /// Set this bit to clear the sw_xoff_int_raw interrupt.
    #[inline(always)]
    pub const fn with_sw_xoff_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SW_XOFF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SW_XOFF_INT_CLR_MASK
            },
        }
    }

    /// Set **SW_XON_INT_CLR** `[9:9]`
    ///
    /// Set this bit to clear the sw_xon_int_raw interrupt.
    #[inline(always)]
    pub const fn with_sw_xon_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SW_XON_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SW_XON_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_TOUT_INT_CLR** `[8:8]`
    ///
    /// Set this bit to clear the rxfifo_tout_int_raw interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_tout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RXFIFO_TOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RXFIFO_TOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **BRK_DET_INT_CLR** `[7:7]`
    ///
    /// Set this bit to clear the brk_det_int_raw interrupt.
    #[inline(always)]
    pub const fn with_brk_det_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::BRK_DET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::BRK_DET_INT_CLR_MASK
            },
        }
    }

    /// Set **CTS_CHG_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the cts_chg_int_raw interrupt.
    #[inline(always)]
    pub const fn with_cts_chg_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CTS_CHG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CTS_CHG_INT_CLR_MASK
            },
        }
    }

    /// Set **DSR_CHG_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the dsr_chg_int_raw interrupt.
    #[inline(always)]
    pub const fn with_dsr_chg_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DSR_CHG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DSR_CHG_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_OVF_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear rxfifo_ovf_int_raw interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RXFIFO_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RXFIFO_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **FRM_ERR_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear frm_err_int_raw interrupt.
    #[inline(always)]
    pub const fn with_frm_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::FRM_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::FRM_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **PARITY_ERR_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear parity_err_int_raw interrupt.
    #[inline(always)]
    pub const fn with_parity_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::PARITY_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::PARITY_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **TXFIFO_EMPTY_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear txfifo_empty_int_raw interrupt.
    #[inline(always)]
    pub const fn with_txfifo_empty_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TXFIFO_EMPTY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TXFIFO_EMPTY_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_FULL_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the rxfifo_full_int_raw interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_full_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RXFIFO_FULL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RXFIFO_FULL_INT_CLR_MASK
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

/// CLKDIV_SYNC Register
///
/// Clock divider configuration
///
/// Address offset: 0x0014
/// Reset value: 0x000002b6
///
/// ## Fields
///
/// - **CLKDIV_FRAG** `[23:20]` (RW)
///
///   The decimal part of the frequency divider factor.
///
/// - **CLKDIV** `[11:0]` (RW)
///
///   The integral part of the frequency divider factor.
#[repr(transparent)]
pub struct ClkdivSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkdivSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkdivSync {
    // ======== Field Constants ========

    /// **CLKDIV_FRAG**
    ///
    /// The decimal part of the frequency divider factor.
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const CLKDIV_FRAG_OFFSET: u32 = 20;
    pub const CLKDIV_FRAG_MASK: u32 = 0x00f00000;

    /// **CLKDIV**
    ///
    /// The integral part of the frequency divider factor.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const CLKDIV_OFFSET: u32 = 0;
    pub const CLKDIV_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **CLKDIV_FRAG** `[23:20]` (RW)
    ///
    /// The decimal part of the frequency divider factor.
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> u32 {
        (self.value.read_bits() & Self::CLKDIV_FRAG_MASK) >> Self::CLKDIV_FRAG_OFFSET
    }

    /// Write **CLKDIV_FRAG** `[23:20]` (RW)
    ///
    /// The decimal part of the frequency divider factor.
    #[inline(always)]
    pub fn write_clkdiv_frag(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLKDIV_FRAG_MASK)
                | ((value << Self::CLKDIV_FRAG_OFFSET) & Self::CLKDIV_FRAG_MASK)
        });
    }

    /// **CLKDIV** `[11:0]` (RW)
    ///
    /// The integral part of the frequency divider factor.
    #[inline(always)]
    pub fn clkdiv(&self) -> u32 {
        (self.value.read_bits() & Self::CLKDIV_MASK) >> Self::CLKDIV_OFFSET
    }

    /// Write **CLKDIV** `[11:0]` (RW)
    ///
    /// The integral part of the frequency divider factor.
    #[inline(always)]
    pub fn write_clkdiv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLKDIV_MASK) | ((value << Self::CLKDIV_OFFSET) & Self::CLKDIV_MASK)
        });
    }
}

/// CLKDIV_SYNC value type for batch read/modify/write operations
///
/// Use `clkdivsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkdivsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkdivSyncVal {
    bits: u32,
}

impl ClkdivSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x000002b6 }
    }

    /// Read **CLKDIV_FRAG** `[23:20]`
    ///
    /// The decimal part of the frequency divider factor.
    #[inline(always)]
    pub const fn clkdiv_frag(&self) -> u32 {
        (self.bits & ClkdivSync::CLKDIV_FRAG_MASK) >> ClkdivSync::CLKDIV_FRAG_OFFSET
    }

    /// Read **CLKDIV** `[11:0]`
    ///
    /// The integral part of the frequency divider factor.
    #[inline(always)]
    pub const fn clkdiv(&self) -> u32 {
        (self.bits & ClkdivSync::CLKDIV_MASK) >> ClkdivSync::CLKDIV_OFFSET
    }

    /// Set **CLKDIV_FRAG** `[23:20]`
    ///
    /// The decimal part of the frequency divider factor.
    #[inline(always)]
    pub const fn with_clkdiv_frag(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkdivSync::CLKDIV_FRAG_MASK)
                | ((value << ClkdivSync::CLKDIV_FRAG_OFFSET) & ClkdivSync::CLKDIV_FRAG_MASK),
        }
    }

    /// Set **CLKDIV** `[11:0]`
    ///
    /// The integral part of the frequency divider factor.
    #[inline(always)]
    pub const fn with_clkdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkdivSync::CLKDIV_MASK)
                | ((value << ClkdivSync::CLKDIV_OFFSET) & ClkdivSync::CLKDIV_MASK),
        }
    }
}

impl Default for ClkdivSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkdivSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkdivSyncVal {
        ClkdivSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkdivSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkdivSyncVal) -> ClkdivSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_FILT Register
///
/// Rx Filter configuration
///
/// Address offset: 0x0018
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **GLITCH_FILT_EN** `[8]` (RW)
///
///   Set this bit to enable Rx signal filter.
///
/// - **GLITCH_FILT** `[7:0]` (RW)
///
///   when input pulse width is lower than this value the pulse is ignored.
#[repr(transparent)]
pub struct RxFilt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxFilt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxFilt {
    // ======== Field Constants ========

    /// **GLITCH_FILT_EN**
    ///
    /// Set this bit to enable Rx signal filter.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GLITCH_FILT_EN_OFFSET: u32 = 8;
    pub const GLITCH_FILT_EN_MASK: u32 = 0x00000100;
    pub const GLITCH_FILT_EN: u32 = 1 << 8;

    /// **GLITCH_FILT**
    ///
    /// when input pulse width is lower than this value the pulse is ignored.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const GLITCH_FILT_OFFSET: u32 = 0;
    pub const GLITCH_FILT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **GLITCH_FILT_EN** `[8]` (RW)
    ///
    /// Set this bit to enable Rx signal filter.
    #[inline(always)]
    pub fn glitch_filt_en(&self) -> bool {
        (self.value.read_bits() & Self::GLITCH_FILT_EN_MASK) != 0
    }

    /// Set GLITCH_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_glitch_filt_en(&self) {
        self.value.modify_bits(|r| r | Self::GLITCH_FILT_EN_MASK);
    }

    /// Clear GLITCH_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_glitch_filt_en(&self) {
        self.value.modify_bits(|r| r & !Self::GLITCH_FILT_EN_MASK);
    }

    /// **GLITCH_FILT** `[7:0]` (RW)
    ///
    /// when input pulse width is lower than this value the pulse is ignored.
    #[inline(always)]
    pub fn glitch_filt(&self) -> u32 {
        (self.value.read_bits() & Self::GLITCH_FILT_MASK) >> Self::GLITCH_FILT_OFFSET
    }

    /// Write **GLITCH_FILT** `[7:0]` (RW)
    ///
    /// when input pulse width is lower than this value the pulse is ignored.
    #[inline(always)]
    pub fn write_glitch_filt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GLITCH_FILT_MASK)
                | ((value << Self::GLITCH_FILT_OFFSET) & Self::GLITCH_FILT_MASK)
        });
    }
}

/// RX_FILT value type for batch read/modify/write operations
///
/// Use `rxfilt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxfilt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxFiltVal {
    bits: u32,
}

impl RxFiltVal {
    /// Create from raw bits
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

    /// Read **GLITCH_FILT_EN** `[8:8]`
    ///
    /// Set this bit to enable Rx signal filter.
    #[inline(always)]
    pub const fn glitch_filt_en(&self) -> bool {
        (self.bits & RxFilt::GLITCH_FILT_EN_MASK) != 0
    }

    /// Read **GLITCH_FILT** `[7:0]`
    ///
    /// when input pulse width is lower than this value the pulse is ignored.
    #[inline(always)]
    pub const fn glitch_filt(&self) -> u32 {
        (self.bits & RxFilt::GLITCH_FILT_MASK) >> RxFilt::GLITCH_FILT_OFFSET
    }

    /// Set **GLITCH_FILT_EN** `[8:8]`
    ///
    /// Set this bit to enable Rx signal filter.
    #[inline(always)]
    pub const fn with_glitch_filt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RxFilt::GLITCH_FILT_EN_MASK
            } else {
                self.bits & !RxFilt::GLITCH_FILT_EN_MASK
            },
        }
    }

    /// Set **GLITCH_FILT** `[7:0]`
    ///
    /// when input pulse width is lower than this value the pulse is ignored.
    #[inline(always)]
    pub const fn with_glitch_filt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxFilt::GLITCH_FILT_MASK)
                | ((value << RxFilt::GLITCH_FILT_OFFSET) & RxFilt::GLITCH_FILT_MASK),
        }
    }
}

impl Default for RxFiltVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxFilt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxFiltVal {
        RxFiltVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxFiltVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxFiltVal) -> RxFiltVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS Register
///
/// UART status register
///
/// Address offset: 0x001c
/// Reset value: 0xe000c000
///
/// ## Fields
///
/// - **TXD** `[31]` (RO)
///
///   This bit represents the level of the internal uart txd signal.
///
/// - **RTSN** `[30]` (RO)
///
///   This bit represents the level of the internal uart rts signal.
///
/// - **DTRN** `[29]` (RO)
///
///   This bit represents the level of the internal uart dtr signal.
///
/// - **TXFIFO_CNT** `[23:19]` (RO)
///
///   Stores the byte number of data in Tx-FIFO.
///
/// - **RXD** `[15]` (RO)
///
///   This register represent the level value of the internal uart rxd signal.
///
/// - **CTSN** `[14]` (RO)
///
///   This register represent the level value of the internal uart cts signal.
///
/// - **DSRN** `[13]` (RO)
///
///   The register represent the level value of the internal uart dsr signal.
///
/// - **RXFIFO_CNT** `[7:3]` (RO)
///
///   Stores the byte number of valid data in Rx-FIFO.
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

    /// **TXD**
    ///
    /// This bit represents the level of the internal uart txd signal.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TXD_OFFSET: u32 = 31;
    pub const TXD_MASK: u32 = 0x80000000;
    pub const TXD: u32 = 1 << 31;

    /// **RTSN**
    ///
    /// This bit represents the level of the internal uart rts signal.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RTSN_OFFSET: u32 = 30;
    pub const RTSN_MASK: u32 = 0x40000000;
    pub const RTSN: u32 = 1 << 30;

    /// **DTRN**
    ///
    /// This bit represents the level of the internal uart dtr signal.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const DTRN_OFFSET: u32 = 29;
    pub const DTRN_MASK: u32 = 0x20000000;
    pub const DTRN: u32 = 1 << 29;

    /// **TXFIFO_CNT**
    ///
    /// Stores the byte number of data in Tx-FIFO.
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const TXFIFO_CNT_OFFSET: u32 = 19;
    pub const TXFIFO_CNT_MASK: u32 = 0x00f80000;

    /// **RXD**
    ///
    /// This register represent the level value of the internal uart rxd signal.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RXD_OFFSET: u32 = 15;
    pub const RXD_MASK: u32 = 0x00008000;
    pub const RXD: u32 = 1 << 15;

    /// **CTSN**
    ///
    /// This register represent the level value of the internal uart cts signal.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CTSN_OFFSET: u32 = 14;
    pub const CTSN_MASK: u32 = 0x00004000;
    pub const CTSN: u32 = 1 << 14;

    /// **DSRN**
    ///
    /// The register represent the level value of the internal uart dsr signal.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DSRN_OFFSET: u32 = 13;
    pub const DSRN_MASK: u32 = 0x00002000;
    pub const DSRN: u32 = 1 << 13;

    /// **RXFIFO_CNT**
    ///
    /// Stores the byte number of valid data in Rx-FIFO.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const RXFIFO_CNT_OFFSET: u32 = 3;
    pub const RXFIFO_CNT_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **TXD** `[31]` (RO)
    ///
    /// This bit represents the level of the internal uart txd signal.
    #[inline(always)]
    pub fn txd(&self) -> bool {
        (self.value.read_bits() & Self::TXD_MASK) != 0
    }

    /// **RTSN** `[30]` (RO)
    ///
    /// This bit represents the level of the internal uart rts signal.
    #[inline(always)]
    pub fn rtsn(&self) -> bool {
        (self.value.read_bits() & Self::RTSN_MASK) != 0
    }

    /// **DTRN** `[29]` (RO)
    ///
    /// This bit represents the level of the internal uart dtr signal.
    #[inline(always)]
    pub fn dtrn(&self) -> bool {
        (self.value.read_bits() & Self::DTRN_MASK) != 0
    }

    /// **TXFIFO_CNT** `[23:19]` (RO)
    ///
    /// Stores the byte number of data in Tx-FIFO.
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_CNT_MASK) >> Self::TXFIFO_CNT_OFFSET
    }

    /// **RXD** `[15]` (RO)
    ///
    /// This register represent the level value of the internal uart rxd signal.
    #[inline(always)]
    pub fn rxd(&self) -> bool {
        (self.value.read_bits() & Self::RXD_MASK) != 0
    }

    /// **CTSN** `[14]` (RO)
    ///
    /// This register represent the level value of the internal uart cts signal.
    #[inline(always)]
    pub fn ctsn(&self) -> bool {
        (self.value.read_bits() & Self::CTSN_MASK) != 0
    }

    /// **DSRN** `[13]` (RO)
    ///
    /// The register represent the level value of the internal uart dsr signal.
    #[inline(always)]
    pub fn dsrn(&self) -> bool {
        (self.value.read_bits() & Self::DSRN_MASK) != 0
    }

    /// **RXFIFO_CNT** `[7:3]` (RO)
    ///
    /// Stores the byte number of valid data in Rx-FIFO.
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_CNT_MASK) >> Self::RXFIFO_CNT_OFFSET
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
        Self { bits: 0xe000c000 }
    }

    /// Read **TXD** `[31:31]`
    ///
    /// This bit represents the level of the internal uart txd signal.
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        (self.bits & Status::TXD_MASK) != 0
    }

    /// Read **RTSN** `[30:30]`
    ///
    /// This bit represents the level of the internal uart rts signal.
    #[inline(always)]
    pub const fn rtsn(&self) -> bool {
        (self.bits & Status::RTSN_MASK) != 0
    }

    /// Read **DTRN** `[29:29]`
    ///
    /// This bit represents the level of the internal uart dtr signal.
    #[inline(always)]
    pub const fn dtrn(&self) -> bool {
        (self.bits & Status::DTRN_MASK) != 0
    }

    /// Read **TXFIFO_CNT** `[23:19]`
    ///
    /// Stores the byte number of data in Tx-FIFO.
    #[inline(always)]
    pub const fn txfifo_cnt(&self) -> u32 {
        (self.bits & Status::TXFIFO_CNT_MASK) >> Status::TXFIFO_CNT_OFFSET
    }

    /// Read **RXD** `[15:15]`
    ///
    /// This register represent the level value of the internal uart rxd signal.
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        (self.bits & Status::RXD_MASK) != 0
    }

    /// Read **CTSN** `[14:14]`
    ///
    /// This register represent the level value of the internal uart cts signal.
    #[inline(always)]
    pub const fn ctsn(&self) -> bool {
        (self.bits & Status::CTSN_MASK) != 0
    }

    /// Read **DSRN** `[13:13]`
    ///
    /// The register represent the level value of the internal uart dsr signal.
    #[inline(always)]
    pub const fn dsrn(&self) -> bool {
        (self.bits & Status::DSRN_MASK) != 0
    }

    /// Read **RXFIFO_CNT** `[7:3]`
    ///
    /// Stores the byte number of valid data in Rx-FIFO.
    #[inline(always)]
    pub const fn rxfifo_cnt(&self) -> u32 {
        (self.bits & Status::RXFIFO_CNT_MASK) >> Status::RXFIFO_CNT_OFFSET
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

/// CONF0_SYNC Register
///
/// Configuration register 0
///
/// Address offset: 0x0020
/// Reset value: 0x0000001c
///
/// ## Fields
///
/// - **TXFIFO_RST** `[23]` (RW)
///
///   Set this bit to reset the uart transmit-FIFO.
///
/// - **RXFIFO_RST** `[22]` (RW)
///
///   Set this bit to reset the uart receive-FIFO.
///
/// - **SW_RTS** `[21]` (RW)
///
///   This register is used to configure the software rts signal which is used in software flow control.
///
/// - **MEM_CLK_EN** `[20]` (RW)
///
///   UART memory clock gate enable signal.
///
/// - **ERR_WR_MASK** `[18]` (RW)
///
///   1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
///
/// - **DIS_RX_DAT_OVF** `[17]` (RW)
///
///   Disable UART Rx data overflow detect.
///
/// - **TXD_INV** `[16]` (RW)
///
///   Set this bit to inverse the level value of uart txd signal.
///
/// - **RXD_INV** `[15]` (RW)
///
///   Set this bit to inverse the level value of uart rxd signal.
///
/// - **TX_FLOW_EN** `[13]` (RW)
///
///   Set this bit to enable flow control function for transmitter.
///
/// - **LOOPBACK** `[12]` (RW)
///
///   Set this bit to enable uart loopback test mode.
///
/// - **TXD_BRK** `[6]` (RW)
///
///   Set this bit to enbale transmitter to send NULL when the process of sending data is done.
///
/// - **STOP_BIT_NUM** `[5:4]` (RW)
///
///   This register is used to set the length of stop bit.
///
/// - **BIT_NUM** `[3:2]` (RW)
///
///   This register is used to set the length of data.
///
/// - **PARITY_EN** `[1]` (RW)
///
///   Set this bit to enable uart parity check.
///
/// - **PARITY** `[0]` (RW)
///
///   This register is used to configure the parity check mode.
#[repr(transparent)]
pub struct Conf0Sync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf0Sync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf0Sync {
    // ======== Field Constants ========

    /// **TXFIFO_RST**
    ///
    /// Set this bit to reset the uart transmit-FIFO.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TXFIFO_RST_OFFSET: u32 = 23;
    pub const TXFIFO_RST_MASK: u32 = 0x00800000;
    pub const TXFIFO_RST: u32 = 1 << 23;

    /// **RXFIFO_RST**
    ///
    /// Set this bit to reset the uart receive-FIFO.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const RXFIFO_RST_OFFSET: u32 = 22;
    pub const RXFIFO_RST_MASK: u32 = 0x00400000;
    pub const RXFIFO_RST: u32 = 1 << 22;

    /// **SW_RTS**
    ///
    /// This register is used to configure the software rts signal which is used in software flow control.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SW_RTS_OFFSET: u32 = 21;
    pub const SW_RTS_MASK: u32 = 0x00200000;
    pub const SW_RTS: u32 = 1 << 21;

    /// **MEM_CLK_EN**
    ///
    /// UART memory clock gate enable signal.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const MEM_CLK_EN_OFFSET: u32 = 20;
    pub const MEM_CLK_EN_MASK: u32 = 0x00100000;
    pub const MEM_CLK_EN: u32 = 1 << 20;

    /// **ERR_WR_MASK**
    ///
    /// 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ERR_WR_MASK_OFFSET: u32 = 18;
    pub const ERR_WR_MASK_MASK: u32 = 0x00040000;
    pub const ERR_WR_MASK: u32 = 1 << 18;

    /// **DIS_RX_DAT_OVF**
    ///
    /// Disable UART Rx data overflow detect.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DIS_RX_DAT_OVF_OFFSET: u32 = 17;
    pub const DIS_RX_DAT_OVF_MASK: u32 = 0x00020000;
    pub const DIS_RX_DAT_OVF: u32 = 1 << 17;

    /// **TXD_INV**
    ///
    /// Set this bit to inverse the level value of uart txd signal.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TXD_INV_OFFSET: u32 = 16;
    pub const TXD_INV_MASK: u32 = 0x00010000;
    pub const TXD_INV: u32 = 1 << 16;

    /// **RXD_INV**
    ///
    /// Set this bit to inverse the level value of uart rxd signal.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RXD_INV_OFFSET: u32 = 15;
    pub const RXD_INV_MASK: u32 = 0x00008000;
    pub const RXD_INV: u32 = 1 << 15;

    /// **TX_FLOW_EN**
    ///
    /// Set this bit to enable flow control function for transmitter.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_FLOW_EN_OFFSET: u32 = 13;
    pub const TX_FLOW_EN_MASK: u32 = 0x00002000;
    pub const TX_FLOW_EN: u32 = 1 << 13;

    /// **LOOPBACK**
    ///
    /// Set this bit to enable uart loopback test mode.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LOOPBACK_OFFSET: u32 = 12;
    pub const LOOPBACK_MASK: u32 = 0x00001000;
    pub const LOOPBACK: u32 = 1 << 12;

    /// **TXD_BRK**
    ///
    /// Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TXD_BRK_OFFSET: u32 = 6;
    pub const TXD_BRK_MASK: u32 = 0x00000040;
    pub const TXD_BRK: u32 = 1 << 6;

    /// **STOP_BIT_NUM**
    ///
    /// This register is used to set the length of stop bit.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const STOP_BIT_NUM_OFFSET: u32 = 4;
    pub const STOP_BIT_NUM_MASK: u32 = 0x00000030;

    /// **BIT_NUM**
    ///
    /// This register is used to set the length of data.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const BIT_NUM_OFFSET: u32 = 2;
    pub const BIT_NUM_MASK: u32 = 0x0000000c;

    /// **PARITY_EN**
    ///
    /// Set this bit to enable uart parity check.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PARITY_EN_OFFSET: u32 = 1;
    pub const PARITY_EN_MASK: u32 = 0x00000002;
    pub const PARITY_EN: u32 = 1 << 1;

    /// **PARITY**
    ///
    /// This register is used to configure the parity check mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PARITY_OFFSET: u32 = 0;
    pub const PARITY_MASK: u32 = 0x00000001;
    pub const PARITY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXFIFO_RST** `[23]` (RW)
    ///
    /// Set this bit to reset the uart transmit-FIFO.
    #[inline(always)]
    pub fn txfifo_rst(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_RST_MASK) != 0
    }

    /// Set TXFIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::TXFIFO_RST_MASK);
    }

    /// Clear TXFIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TXFIFO_RST_MASK);
    }

    /// **RXFIFO_RST** `[22]` (RW)
    ///
    /// Set this bit to reset the uart receive-FIFO.
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_RST_MASK) != 0
    }

    /// Set RXFIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::RXFIFO_RST_MASK);
    }

    /// Clear RXFIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_rst(&self) {
        self.value.modify_bits(|r| r & !Self::RXFIFO_RST_MASK);
    }

    /// **SW_RTS** `[21]` (RW)
    ///
    /// This register is used to configure the software rts signal which is used in software flow control.
    #[inline(always)]
    pub fn sw_rts(&self) -> bool {
        (self.value.read_bits() & Self::SW_RTS_MASK) != 0
    }

    /// Set SW_RTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_rts(&self) {
        self.value.modify_bits(|r| r | Self::SW_RTS_MASK);
    }

    /// Clear SW_RTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_rts(&self) {
        self.value.modify_bits(|r| r & !Self::SW_RTS_MASK);
    }

    /// **MEM_CLK_EN** `[20]` (RW)
    ///
    /// UART memory clock gate enable signal.
    #[inline(always)]
    pub fn mem_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CLK_EN_MASK) != 0
    }

    /// Set MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::MEM_CLK_EN_MASK);
    }

    /// Clear MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_CLK_EN_MASK);
    }

    /// **ERR_WR_MASK** `[18]` (RW)
    ///
    /// 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    #[inline(always)]
    pub fn err_wr_mask(&self) -> bool {
        (self.value.read_bits() & Self::ERR_WR_MASK_MASK) != 0
    }

    /// Set ERR_WR_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_err_wr_mask(&self) {
        self.value.modify_bits(|r| r | Self::ERR_WR_MASK_MASK);
    }

    /// Clear ERR_WR_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_err_wr_mask(&self) {
        self.value.modify_bits(|r| r & !Self::ERR_WR_MASK_MASK);
    }

    /// **DIS_RX_DAT_OVF** `[17]` (RW)
    ///
    /// Disable UART Rx data overflow detect.
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> bool {
        (self.value.read_bits() & Self::DIS_RX_DAT_OVF_MASK) != 0
    }

    /// Set DIS_RX_DAT_OVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dis_rx_dat_ovf(&self) {
        self.value.modify_bits(|r| r | Self::DIS_RX_DAT_OVF_MASK);
    }

    /// Clear DIS_RX_DAT_OVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dis_rx_dat_ovf(&self) {
        self.value.modify_bits(|r| r & !Self::DIS_RX_DAT_OVF_MASK);
    }

    /// **TXD_INV** `[16]` (RW)
    ///
    /// Set this bit to inverse the level value of uart txd signal.
    #[inline(always)]
    pub fn txd_inv(&self) -> bool {
        (self.value.read_bits() & Self::TXD_INV_MASK) != 0
    }

    /// Set TXD_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txd_inv(&self) {
        self.value.modify_bits(|r| r | Self::TXD_INV_MASK);
    }

    /// Clear TXD_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txd_inv(&self) {
        self.value.modify_bits(|r| r & !Self::TXD_INV_MASK);
    }

    /// **RXD_INV** `[15]` (RW)
    ///
    /// Set this bit to inverse the level value of uart rxd signal.
    #[inline(always)]
    pub fn rxd_inv(&self) -> bool {
        (self.value.read_bits() & Self::RXD_INV_MASK) != 0
    }

    /// Set RXD_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxd_inv(&self) {
        self.value.modify_bits(|r| r | Self::RXD_INV_MASK);
    }

    /// Clear RXD_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxd_inv(&self) {
        self.value.modify_bits(|r| r & !Self::RXD_INV_MASK);
    }

    /// **TX_FLOW_EN** `[13]` (RW)
    ///
    /// Set this bit to enable flow control function for transmitter.
    #[inline(always)]
    pub fn tx_flow_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_FLOW_EN_MASK) != 0
    }

    /// Set TX_FLOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_flow_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_FLOW_EN_MASK);
    }

    /// Clear TX_FLOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_flow_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_FLOW_EN_MASK);
    }

    /// **LOOPBACK** `[12]` (RW)
    ///
    /// Set this bit to enable uart loopback test mode.
    #[inline(always)]
    pub fn loopback(&self) -> bool {
        (self.value.read_bits() & Self::LOOPBACK_MASK) != 0
    }

    /// Set LOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loopback(&self) {
        self.value.modify_bits(|r| r | Self::LOOPBACK_MASK);
    }

    /// Clear LOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loopback(&self) {
        self.value.modify_bits(|r| r & !Self::LOOPBACK_MASK);
    }

    /// **TXD_BRK** `[6]` (RW)
    ///
    /// Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    #[inline(always)]
    pub fn txd_brk(&self) -> bool {
        (self.value.read_bits() & Self::TXD_BRK_MASK) != 0
    }

    /// Set TXD_BRK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txd_brk(&self) {
        self.value.modify_bits(|r| r | Self::TXD_BRK_MASK);
    }

    /// Clear TXD_BRK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txd_brk(&self) {
        self.value.modify_bits(|r| r & !Self::TXD_BRK_MASK);
    }

    /// **STOP_BIT_NUM** `[5:4]` (RW)
    ///
    /// This register is used to set the length of stop bit.
    #[inline(always)]
    pub fn stop_bit_num(&self) -> u32 {
        (self.value.read_bits() & Self::STOP_BIT_NUM_MASK) >> Self::STOP_BIT_NUM_OFFSET
    }

    /// Write **STOP_BIT_NUM** `[5:4]` (RW)
    ///
    /// This register is used to set the length of stop bit.
    #[inline(always)]
    pub fn write_stop_bit_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::STOP_BIT_NUM_MASK)
                | ((value << Self::STOP_BIT_NUM_OFFSET) & Self::STOP_BIT_NUM_MASK)
        });
    }

    /// **BIT_NUM** `[3:2]` (RW)
    ///
    /// This register is used to set the length of data.
    #[inline(always)]
    pub fn bit_num(&self) -> u32 {
        (self.value.read_bits() & Self::BIT_NUM_MASK) >> Self::BIT_NUM_OFFSET
    }

    /// Write **BIT_NUM** `[3:2]` (RW)
    ///
    /// This register is used to set the length of data.
    #[inline(always)]
    pub fn write_bit_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BIT_NUM_MASK) | ((value << Self::BIT_NUM_OFFSET) & Self::BIT_NUM_MASK)
        });
    }

    /// **PARITY_EN** `[1]` (RW)
    ///
    /// Set this bit to enable uart parity check.
    #[inline(always)]
    pub fn parity_en(&self) -> bool {
        (self.value.read_bits() & Self::PARITY_EN_MASK) != 0
    }

    /// Set PARITY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_parity_en(&self) {
        self.value.modify_bits(|r| r | Self::PARITY_EN_MASK);
    }

    /// Clear PARITY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_parity_en(&self) {
        self.value.modify_bits(|r| r & !Self::PARITY_EN_MASK);
    }

    /// **PARITY** `[0]` (RW)
    ///
    /// This register is used to configure the parity check mode.
    #[inline(always)]
    pub fn parity(&self) -> bool {
        (self.value.read_bits() & Self::PARITY_MASK) != 0
    }

    /// Set PARITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_parity(&self) {
        self.value.modify_bits(|r| r | Self::PARITY_MASK);
    }

    /// Clear PARITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_parity(&self) {
        self.value.modify_bits(|r| r & !Self::PARITY_MASK);
    }
}

/// CONF0_SYNC value type for batch read/modify/write operations
///
/// Use `conf0sync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf0sync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Conf0SyncVal {
    bits: u32,
}

impl Conf0SyncVal {
    /// Create from raw bits
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

    /// Read **TXFIFO_RST** `[23:23]`
    ///
    /// Set this bit to reset the uart transmit-FIFO.
    #[inline(always)]
    pub const fn txfifo_rst(&self) -> bool {
        (self.bits & Conf0Sync::TXFIFO_RST_MASK) != 0
    }

    /// Read **RXFIFO_RST** `[22:22]`
    ///
    /// Set this bit to reset the uart receive-FIFO.
    #[inline(always)]
    pub const fn rxfifo_rst(&self) -> bool {
        (self.bits & Conf0Sync::RXFIFO_RST_MASK) != 0
    }

    /// Read **SW_RTS** `[21:21]`
    ///
    /// This register is used to configure the software rts signal which is used in software flow control.
    #[inline(always)]
    pub const fn sw_rts(&self) -> bool {
        (self.bits & Conf0Sync::SW_RTS_MASK) != 0
    }

    /// Read **MEM_CLK_EN** `[20:20]`
    ///
    /// UART memory clock gate enable signal.
    #[inline(always)]
    pub const fn mem_clk_en(&self) -> bool {
        (self.bits & Conf0Sync::MEM_CLK_EN_MASK) != 0
    }

    /// Read **ERR_WR_MASK** `[18:18]`
    ///
    /// 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    #[inline(always)]
    pub const fn err_wr_mask(&self) -> bool {
        (self.bits & Conf0Sync::ERR_WR_MASK_MASK) != 0
    }

    /// Read **DIS_RX_DAT_OVF** `[17:17]`
    ///
    /// Disable UART Rx data overflow detect.
    #[inline(always)]
    pub const fn dis_rx_dat_ovf(&self) -> bool {
        (self.bits & Conf0Sync::DIS_RX_DAT_OVF_MASK) != 0
    }

    /// Read **TXD_INV** `[16:16]`
    ///
    /// Set this bit to inverse the level value of uart txd signal.
    #[inline(always)]
    pub const fn txd_inv(&self) -> bool {
        (self.bits & Conf0Sync::TXD_INV_MASK) != 0
    }

    /// Read **RXD_INV** `[15:15]`
    ///
    /// Set this bit to inverse the level value of uart rxd signal.
    #[inline(always)]
    pub const fn rxd_inv(&self) -> bool {
        (self.bits & Conf0Sync::RXD_INV_MASK) != 0
    }

    /// Read **TX_FLOW_EN** `[13:13]`
    ///
    /// Set this bit to enable flow control function for transmitter.
    #[inline(always)]
    pub const fn tx_flow_en(&self) -> bool {
        (self.bits & Conf0Sync::TX_FLOW_EN_MASK) != 0
    }

    /// Read **LOOPBACK** `[12:12]`
    ///
    /// Set this bit to enable uart loopback test mode.
    #[inline(always)]
    pub const fn loopback(&self) -> bool {
        (self.bits & Conf0Sync::LOOPBACK_MASK) != 0
    }

    /// Read **TXD_BRK** `[6:6]`
    ///
    /// Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    #[inline(always)]
    pub const fn txd_brk(&self) -> bool {
        (self.bits & Conf0Sync::TXD_BRK_MASK) != 0
    }

    /// Read **STOP_BIT_NUM** `[5:4]`
    ///
    /// This register is used to set the length of stop bit.
    #[inline(always)]
    pub const fn stop_bit_num(&self) -> u32 {
        (self.bits & Conf0Sync::STOP_BIT_NUM_MASK) >> Conf0Sync::STOP_BIT_NUM_OFFSET
    }

    /// Read **BIT_NUM** `[3:2]`
    ///
    /// This register is used to set the length of data.
    #[inline(always)]
    pub const fn bit_num(&self) -> u32 {
        (self.bits & Conf0Sync::BIT_NUM_MASK) >> Conf0Sync::BIT_NUM_OFFSET
    }

    /// Read **PARITY_EN** `[1:1]`
    ///
    /// Set this bit to enable uart parity check.
    #[inline(always)]
    pub const fn parity_en(&self) -> bool {
        (self.bits & Conf0Sync::PARITY_EN_MASK) != 0
    }

    /// Read **PARITY** `[0:0]`
    ///
    /// This register is used to configure the parity check mode.
    #[inline(always)]
    pub const fn parity(&self) -> bool {
        (self.bits & Conf0Sync::PARITY_MASK) != 0
    }

    /// Set **TXFIFO_RST** `[23:23]`
    ///
    /// Set this bit to reset the uart transmit-FIFO.
    #[inline(always)]
    pub const fn with_txfifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::TXFIFO_RST_MASK
            } else {
                self.bits & !Conf0Sync::TXFIFO_RST_MASK
            },
        }
    }

    /// Set **RXFIFO_RST** `[22:22]`
    ///
    /// Set this bit to reset the uart receive-FIFO.
    #[inline(always)]
    pub const fn with_rxfifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::RXFIFO_RST_MASK
            } else {
                self.bits & !Conf0Sync::RXFIFO_RST_MASK
            },
        }
    }

    /// Set **SW_RTS** `[21:21]`
    ///
    /// This register is used to configure the software rts signal which is used in software flow control.
    #[inline(always)]
    pub const fn with_sw_rts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::SW_RTS_MASK
            } else {
                self.bits & !Conf0Sync::SW_RTS_MASK
            },
        }
    }

    /// Set **MEM_CLK_EN** `[20:20]`
    ///
    /// UART memory clock gate enable signal.
    #[inline(always)]
    pub const fn with_mem_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::MEM_CLK_EN_MASK
            } else {
                self.bits & !Conf0Sync::MEM_CLK_EN_MASK
            },
        }
    }

    /// Set **ERR_WR_MASK** `[18:18]`
    ///
    /// 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    #[inline(always)]
    pub const fn with_err_wr_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::ERR_WR_MASK_MASK
            } else {
                self.bits & !Conf0Sync::ERR_WR_MASK_MASK
            },
        }
    }

    /// Set **DIS_RX_DAT_OVF** `[17:17]`
    ///
    /// Disable UART Rx data overflow detect.
    #[inline(always)]
    pub const fn with_dis_rx_dat_ovf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::DIS_RX_DAT_OVF_MASK
            } else {
                self.bits & !Conf0Sync::DIS_RX_DAT_OVF_MASK
            },
        }
    }

    /// Set **TXD_INV** `[16:16]`
    ///
    /// Set this bit to inverse the level value of uart txd signal.
    #[inline(always)]
    pub const fn with_txd_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::TXD_INV_MASK
            } else {
                self.bits & !Conf0Sync::TXD_INV_MASK
            },
        }
    }

    /// Set **RXD_INV** `[15:15]`
    ///
    /// Set this bit to inverse the level value of uart rxd signal.
    #[inline(always)]
    pub const fn with_rxd_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::RXD_INV_MASK
            } else {
                self.bits & !Conf0Sync::RXD_INV_MASK
            },
        }
    }

    /// Set **TX_FLOW_EN** `[13:13]`
    ///
    /// Set this bit to enable flow control function for transmitter.
    #[inline(always)]
    pub const fn with_tx_flow_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::TX_FLOW_EN_MASK
            } else {
                self.bits & !Conf0Sync::TX_FLOW_EN_MASK
            },
        }
    }

    /// Set **LOOPBACK** `[12:12]`
    ///
    /// Set this bit to enable uart loopback test mode.
    #[inline(always)]
    pub const fn with_loopback(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::LOOPBACK_MASK
            } else {
                self.bits & !Conf0Sync::LOOPBACK_MASK
            },
        }
    }

    /// Set **TXD_BRK** `[6:6]`
    ///
    /// Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    #[inline(always)]
    pub const fn with_txd_brk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::TXD_BRK_MASK
            } else {
                self.bits & !Conf0Sync::TXD_BRK_MASK
            },
        }
    }

    /// Set **STOP_BIT_NUM** `[5:4]`
    ///
    /// This register is used to set the length of stop bit.
    #[inline(always)]
    pub const fn with_stop_bit_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf0Sync::STOP_BIT_NUM_MASK)
                | ((value << Conf0Sync::STOP_BIT_NUM_OFFSET) & Conf0Sync::STOP_BIT_NUM_MASK),
        }
    }

    /// Set **BIT_NUM** `[3:2]`
    ///
    /// This register is used to set the length of data.
    #[inline(always)]
    pub const fn with_bit_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf0Sync::BIT_NUM_MASK)
                | ((value << Conf0Sync::BIT_NUM_OFFSET) & Conf0Sync::BIT_NUM_MASK),
        }
    }

    /// Set **PARITY_EN** `[1:1]`
    ///
    /// Set this bit to enable uart parity check.
    #[inline(always)]
    pub const fn with_parity_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::PARITY_EN_MASK
            } else {
                self.bits & !Conf0Sync::PARITY_EN_MASK
            },
        }
    }

    /// Set **PARITY** `[0:0]`
    ///
    /// This register is used to configure the parity check mode.
    #[inline(always)]
    pub const fn with_parity(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0Sync::PARITY_MASK
            } else {
                self.bits & !Conf0Sync::PARITY_MASK
            },
        }
    }
}

impl Default for Conf0SyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Conf0Sync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Conf0SyncVal {
        Conf0SyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Conf0SyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Conf0SyncVal) -> Conf0SyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONF1 Register
///
/// Configuration register 1
///
/// Address offset: 0x0024
/// Reset value: 0x00006060
///
/// ## Fields
///
/// - **CLK_EN** `[21]` (RW)
///
///   1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
///
/// - **SW_DTR** `[20]` (RW)
///
///   This register is used to configure the software dtr signal which is used in software flow control.
///
/// - **DTR_INV** `[19]` (RW)
///
///   Set this bit to inverse the level value of uart dtr signal.
///
/// - **RTS_INV** `[18]` (RW)
///
///   Set this bit to inverse the level value of uart rts signal.
///
/// - **DSR_INV** `[17]` (RW)
///
///   Set this bit to inverse the level value of uart dsr signal.
///
/// - **CTS_INV** `[16]` (RW)
///
///   Set this bit to inverse the level value of uart cts signal.
///
/// - **TXFIFO_EMPTY_THRHD** `[15:11]` (RW)
///
///   It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
///
/// - **RXFIFO_FULL_THRHD** `[7:3]` (RW)
///
///   It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
#[repr(transparent)]
pub struct Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf1 {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_EN_OFFSET: u32 = 21;
    pub const CLK_EN_MASK: u32 = 0x00200000;
    pub const CLK_EN: u32 = 1 << 21;

    /// **SW_DTR**
    ///
    /// This register is used to configure the software dtr signal which is used in software flow control.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SW_DTR_OFFSET: u32 = 20;
    pub const SW_DTR_MASK: u32 = 0x00100000;
    pub const SW_DTR: u32 = 1 << 20;

    /// **DTR_INV**
    ///
    /// Set this bit to inverse the level value of uart dtr signal.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DTR_INV_OFFSET: u32 = 19;
    pub const DTR_INV_MASK: u32 = 0x00080000;
    pub const DTR_INV: u32 = 1 << 19;

    /// **RTS_INV**
    ///
    /// Set this bit to inverse the level value of uart rts signal.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const RTS_INV_OFFSET: u32 = 18;
    pub const RTS_INV_MASK: u32 = 0x00040000;
    pub const RTS_INV: u32 = 1 << 18;

    /// **DSR_INV**
    ///
    /// Set this bit to inverse the level value of uart dsr signal.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DSR_INV_OFFSET: u32 = 17;
    pub const DSR_INV_MASK: u32 = 0x00020000;
    pub const DSR_INV: u32 = 1 << 17;

    /// **CTS_INV**
    ///
    /// Set this bit to inverse the level value of uart cts signal.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CTS_INV_OFFSET: u32 = 16;
    pub const CTS_INV_MASK: u32 = 0x00010000;
    pub const CTS_INV: u32 = 1 << 16;

    /// **TXFIFO_EMPTY_THRHD**
    ///
    /// It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
    ///
    /// Bits `[15:11]`, mask `0x0000f800`
    pub const TXFIFO_EMPTY_THRHD_OFFSET: u32 = 11;
    pub const TXFIFO_EMPTY_THRHD_MASK: u32 = 0x0000f800;

    /// **RXFIFO_FULL_THRHD**
    ///
    /// It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const RXFIFO_FULL_THRHD_OFFSET: u32 = 3;
    pub const RXFIFO_FULL_THRHD_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **CLK_EN** `[21]` (RW)
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

    /// **SW_DTR** `[20]` (RW)
    ///
    /// This register is used to configure the software dtr signal which is used in software flow control.
    #[inline(always)]
    pub fn sw_dtr(&self) -> bool {
        (self.value.read_bits() & Self::SW_DTR_MASK) != 0
    }

    /// Set SW_DTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_dtr(&self) {
        self.value.modify_bits(|r| r | Self::SW_DTR_MASK);
    }

    /// Clear SW_DTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_dtr(&self) {
        self.value.modify_bits(|r| r & !Self::SW_DTR_MASK);
    }

    /// **DTR_INV** `[19]` (RW)
    ///
    /// Set this bit to inverse the level value of uart dtr signal.
    #[inline(always)]
    pub fn dtr_inv(&self) -> bool {
        (self.value.read_bits() & Self::DTR_INV_MASK) != 0
    }

    /// Set DTR_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dtr_inv(&self) {
        self.value.modify_bits(|r| r | Self::DTR_INV_MASK);
    }

    /// Clear DTR_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dtr_inv(&self) {
        self.value.modify_bits(|r| r & !Self::DTR_INV_MASK);
    }

    /// **RTS_INV** `[18]` (RW)
    ///
    /// Set this bit to inverse the level value of uart rts signal.
    #[inline(always)]
    pub fn rts_inv(&self) -> bool {
        (self.value.read_bits() & Self::RTS_INV_MASK) != 0
    }

    /// Set RTS_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rts_inv(&self) {
        self.value.modify_bits(|r| r | Self::RTS_INV_MASK);
    }

    /// Clear RTS_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rts_inv(&self) {
        self.value.modify_bits(|r| r & !Self::RTS_INV_MASK);
    }

    /// **DSR_INV** `[17]` (RW)
    ///
    /// Set this bit to inverse the level value of uart dsr signal.
    #[inline(always)]
    pub fn dsr_inv(&self) -> bool {
        (self.value.read_bits() & Self::DSR_INV_MASK) != 0
    }

    /// Set DSR_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsr_inv(&self) {
        self.value.modify_bits(|r| r | Self::DSR_INV_MASK);
    }

    /// Clear DSR_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsr_inv(&self) {
        self.value.modify_bits(|r| r & !Self::DSR_INV_MASK);
    }

    /// **CTS_INV** `[16]` (RW)
    ///
    /// Set this bit to inverse the level value of uart cts signal.
    #[inline(always)]
    pub fn cts_inv(&self) -> bool {
        (self.value.read_bits() & Self::CTS_INV_MASK) != 0
    }

    /// Set CTS_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cts_inv(&self) {
        self.value.modify_bits(|r| r | Self::CTS_INV_MASK);
    }

    /// Clear CTS_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cts_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CTS_INV_MASK);
    }

    /// **TXFIFO_EMPTY_THRHD** `[15:11]` (RW)
    ///
    /// It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_EMPTY_THRHD_MASK) >> Self::TXFIFO_EMPTY_THRHD_OFFSET
    }

    /// Write **TXFIFO_EMPTY_THRHD** `[15:11]` (RW)
    ///
    /// It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
    #[inline(always)]
    pub fn write_txfifo_empty_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXFIFO_EMPTY_THRHD_MASK)
                | ((value << Self::TXFIFO_EMPTY_THRHD_OFFSET) & Self::TXFIFO_EMPTY_THRHD_MASK)
        });
    }

    /// **RXFIFO_FULL_THRHD** `[7:3]` (RW)
    ///
    /// It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_FULL_THRHD_MASK) >> Self::RXFIFO_FULL_THRHD_OFFSET
    }

    /// Write **RXFIFO_FULL_THRHD** `[7:3]` (RW)
    ///
    /// It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
    #[inline(always)]
    pub fn write_rxfifo_full_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXFIFO_FULL_THRHD_MASK)
                | ((value << Self::RXFIFO_FULL_THRHD_OFFSET) & Self::RXFIFO_FULL_THRHD_MASK)
        });
    }
}

/// CONF1 value type for batch read/modify/write operations
///
/// Use `conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Conf1Val {
    bits: u32,
}

impl Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00006060 }
    }

    /// Read **CLK_EN** `[21:21]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Conf1::CLK_EN_MASK) != 0
    }

    /// Read **SW_DTR** `[20:20]`
    ///
    /// This register is used to configure the software dtr signal which is used in software flow control.
    #[inline(always)]
    pub const fn sw_dtr(&self) -> bool {
        (self.bits & Conf1::SW_DTR_MASK) != 0
    }

    /// Read **DTR_INV** `[19:19]`
    ///
    /// Set this bit to inverse the level value of uart dtr signal.
    #[inline(always)]
    pub const fn dtr_inv(&self) -> bool {
        (self.bits & Conf1::DTR_INV_MASK) != 0
    }

    /// Read **RTS_INV** `[18:18]`
    ///
    /// Set this bit to inverse the level value of uart rts signal.
    #[inline(always)]
    pub const fn rts_inv(&self) -> bool {
        (self.bits & Conf1::RTS_INV_MASK) != 0
    }

    /// Read **DSR_INV** `[17:17]`
    ///
    /// Set this bit to inverse the level value of uart dsr signal.
    #[inline(always)]
    pub const fn dsr_inv(&self) -> bool {
        (self.bits & Conf1::DSR_INV_MASK) != 0
    }

    /// Read **CTS_INV** `[16:16]`
    ///
    /// Set this bit to inverse the level value of uart cts signal.
    #[inline(always)]
    pub const fn cts_inv(&self) -> bool {
        (self.bits & Conf1::CTS_INV_MASK) != 0
    }

    /// Read **TXFIFO_EMPTY_THRHD** `[15:11]`
    ///
    /// It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
    #[inline(always)]
    pub const fn txfifo_empty_thrhd(&self) -> u32 {
        (self.bits & Conf1::TXFIFO_EMPTY_THRHD_MASK) >> Conf1::TXFIFO_EMPTY_THRHD_OFFSET
    }

    /// Read **RXFIFO_FULL_THRHD** `[7:3]`
    ///
    /// It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
    #[inline(always)]
    pub const fn rxfifo_full_thrhd(&self) -> u32 {
        (self.bits & Conf1::RXFIFO_FULL_THRHD_MASK) >> Conf1::RXFIFO_FULL_THRHD_OFFSET
    }

    /// Set **CLK_EN** `[21:21]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::CLK_EN_MASK
            } else {
                self.bits & !Conf1::CLK_EN_MASK
            },
        }
    }

    /// Set **SW_DTR** `[20:20]`
    ///
    /// This register is used to configure the software dtr signal which is used in software flow control.
    #[inline(always)]
    pub const fn with_sw_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::SW_DTR_MASK
            } else {
                self.bits & !Conf1::SW_DTR_MASK
            },
        }
    }

    /// Set **DTR_INV** `[19:19]`
    ///
    /// Set this bit to inverse the level value of uart dtr signal.
    #[inline(always)]
    pub const fn with_dtr_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::DTR_INV_MASK
            } else {
                self.bits & !Conf1::DTR_INV_MASK
            },
        }
    }

    /// Set **RTS_INV** `[18:18]`
    ///
    /// Set this bit to inverse the level value of uart rts signal.
    #[inline(always)]
    pub const fn with_rts_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::RTS_INV_MASK
            } else {
                self.bits & !Conf1::RTS_INV_MASK
            },
        }
    }

    /// Set **DSR_INV** `[17:17]`
    ///
    /// Set this bit to inverse the level value of uart dsr signal.
    #[inline(always)]
    pub const fn with_dsr_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::DSR_INV_MASK
            } else {
                self.bits & !Conf1::DSR_INV_MASK
            },
        }
    }

    /// Set **CTS_INV** `[16:16]`
    ///
    /// Set this bit to inverse the level value of uart cts signal.
    #[inline(always)]
    pub const fn with_cts_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::CTS_INV_MASK
            } else {
                self.bits & !Conf1::CTS_INV_MASK
            },
        }
    }

    /// Set **TXFIFO_EMPTY_THRHD** `[15:11]`
    ///
    /// It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value.
    #[inline(always)]
    pub const fn with_txfifo_empty_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf1::TXFIFO_EMPTY_THRHD_MASK)
                | ((value << Conf1::TXFIFO_EMPTY_THRHD_OFFSET) & Conf1::TXFIFO_EMPTY_THRHD_MASK),
        }
    }

    /// Set **RXFIFO_FULL_THRHD** `[7:3]`
    ///
    /// It will produce rxfifo_full_int interrupt when receiver receives more data than this register value.
    #[inline(always)]
    pub const fn with_rxfifo_full_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf1::RXFIFO_FULL_THRHD_MASK)
                | ((value << Conf1::RXFIFO_FULL_THRHD_OFFSET) & Conf1::RXFIFO_FULL_THRHD_MASK),
        }
    }
}

impl Default for Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Conf1Val {
        Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Conf1Val) -> Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HWFC_CONF_SYNC Register
///
/// Hardware flow-control configuration
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **RX_FLOW_EN** `[8]` (RW)
///
///   This is the flow enable bit for UART receiver.
///
/// - **RX_FLOW_THRHD** `[7:3]` (RW)
///
///   This register is used to configure the maximum amount of data that can be received when hardware flow control works.
#[repr(transparent)]
pub struct HwfcConfSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HwfcConfSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HwfcConfSync {
    // ======== Field Constants ========

    /// **RX_FLOW_EN**
    ///
    /// This is the flow enable bit for UART receiver.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RX_FLOW_EN_OFFSET: u32 = 8;
    pub const RX_FLOW_EN_MASK: u32 = 0x00000100;
    pub const RX_FLOW_EN: u32 = 1 << 8;

    /// **RX_FLOW_THRHD**
    ///
    /// This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const RX_FLOW_THRHD_OFFSET: u32 = 3;
    pub const RX_FLOW_THRHD_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **RX_FLOW_EN** `[8]` (RW)
    ///
    /// This is the flow enable bit for UART receiver.
    #[inline(always)]
    pub fn rx_flow_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_FLOW_EN_MASK) != 0
    }

    /// Set RX_FLOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_flow_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_FLOW_EN_MASK);
    }

    /// Clear RX_FLOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_flow_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FLOW_EN_MASK);
    }

    /// **RX_FLOW_THRHD** `[7:3]` (RW)
    ///
    /// This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RX_FLOW_THRHD_MASK) >> Self::RX_FLOW_THRHD_OFFSET
    }

    /// Write **RX_FLOW_THRHD** `[7:3]` (RW)
    ///
    /// This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    pub fn write_rx_flow_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_FLOW_THRHD_MASK)
                | ((value << Self::RX_FLOW_THRHD_OFFSET) & Self::RX_FLOW_THRHD_MASK)
        });
    }
}

/// HWFC_CONF_SYNC value type for batch read/modify/write operations
///
/// Use `hwfcconfsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hwfcconfsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HwfcConfSyncVal {
    bits: u32,
}

impl HwfcConfSyncVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_FLOW_EN** `[8:8]`
    ///
    /// This is the flow enable bit for UART receiver.
    #[inline(always)]
    pub const fn rx_flow_en(&self) -> bool {
        (self.bits & HwfcConfSync::RX_FLOW_EN_MASK) != 0
    }

    /// Read **RX_FLOW_THRHD** `[7:3]`
    ///
    /// This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    pub const fn rx_flow_thrhd(&self) -> u32 {
        (self.bits & HwfcConfSync::RX_FLOW_THRHD_MASK) >> HwfcConfSync::RX_FLOW_THRHD_OFFSET
    }

    /// Set **RX_FLOW_EN** `[8:8]`
    ///
    /// This is the flow enable bit for UART receiver.
    #[inline(always)]
    pub const fn with_rx_flow_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HwfcConfSync::RX_FLOW_EN_MASK
            } else {
                self.bits & !HwfcConfSync::RX_FLOW_EN_MASK
            },
        }
    }

    /// Set **RX_FLOW_THRHD** `[7:3]`
    ///
    /// This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    pub const fn with_rx_flow_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HwfcConfSync::RX_FLOW_THRHD_MASK)
                | ((value << HwfcConfSync::RX_FLOW_THRHD_OFFSET)
                    & HwfcConfSync::RX_FLOW_THRHD_MASK),
        }
    }
}

impl Default for HwfcConfSyncVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HwfcConfSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HwfcConfSyncVal {
        HwfcConfSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HwfcConfSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HwfcConfSyncVal) -> HwfcConfSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLEEP_CONF0 Register
///
/// UART sleep configure register 0
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **WK_CHAR4** `[31:24]` (RW)
///
///   This register restores the specified wake up char4 to wake up
///
/// - **WK_CHAR3** `[23:16]` (RW)
///
///   This register restores the specified wake up char3 to wake up
///
/// - **WK_CHAR2** `[15:8]` (RW)
///
///   This register restores the specified wake up char2 to wake up
///
/// - **WK_CHAR1** `[7:0]` (RW)
///
///   This register restores the specified wake up char1 to wake up
#[repr(transparent)]
pub struct SleepConf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SleepConf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SleepConf0 {
    // ======== Field Constants ========

    /// **WK_CHAR4**
    ///
    /// This register restores the specified wake up char4 to wake up
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const WK_CHAR4_OFFSET: u32 = 24;
    pub const WK_CHAR4_MASK: u32 = 0xff000000;

    /// **WK_CHAR3**
    ///
    /// This register restores the specified wake up char3 to wake up
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const WK_CHAR3_OFFSET: u32 = 16;
    pub const WK_CHAR3_MASK: u32 = 0x00ff0000;

    /// **WK_CHAR2**
    ///
    /// This register restores the specified wake up char2 to wake up
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const WK_CHAR2_OFFSET: u32 = 8;
    pub const WK_CHAR2_MASK: u32 = 0x0000ff00;

    /// **WK_CHAR1**
    ///
    /// This register restores the specified wake up char1 to wake up
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WK_CHAR1_OFFSET: u32 = 0;
    pub const WK_CHAR1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **WK_CHAR4** `[31:24]` (RW)
    ///
    /// This register restores the specified wake up char4 to wake up
    #[inline(always)]
    pub fn wk_char4(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR4_MASK) >> Self::WK_CHAR4_OFFSET
    }

    /// Write **WK_CHAR4** `[31:24]` (RW)
    ///
    /// This register restores the specified wake up char4 to wake up
    #[inline(always)]
    pub fn write_wk_char4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR4_MASK) | ((value << Self::WK_CHAR4_OFFSET) & Self::WK_CHAR4_MASK)
        });
    }

    /// **WK_CHAR3** `[23:16]` (RW)
    ///
    /// This register restores the specified wake up char3 to wake up
    #[inline(always)]
    pub fn wk_char3(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR3_MASK) >> Self::WK_CHAR3_OFFSET
    }

    /// Write **WK_CHAR3** `[23:16]` (RW)
    ///
    /// This register restores the specified wake up char3 to wake up
    #[inline(always)]
    pub fn write_wk_char3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR3_MASK) | ((value << Self::WK_CHAR3_OFFSET) & Self::WK_CHAR3_MASK)
        });
    }

    /// **WK_CHAR2** `[15:8]` (RW)
    ///
    /// This register restores the specified wake up char2 to wake up
    #[inline(always)]
    pub fn wk_char2(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR2_MASK) >> Self::WK_CHAR2_OFFSET
    }

    /// Write **WK_CHAR2** `[15:8]` (RW)
    ///
    /// This register restores the specified wake up char2 to wake up
    #[inline(always)]
    pub fn write_wk_char2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR2_MASK) | ((value << Self::WK_CHAR2_OFFSET) & Self::WK_CHAR2_MASK)
        });
    }

    /// **WK_CHAR1** `[7:0]` (RW)
    ///
    /// This register restores the specified wake up char1 to wake up
    #[inline(always)]
    pub fn wk_char1(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR1_MASK) >> Self::WK_CHAR1_OFFSET
    }

    /// Write **WK_CHAR1** `[7:0]` (RW)
    ///
    /// This register restores the specified wake up char1 to wake up
    #[inline(always)]
    pub fn write_wk_char1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR1_MASK) | ((value << Self::WK_CHAR1_OFFSET) & Self::WK_CHAR1_MASK)
        });
    }
}

/// SLEEP_CONF0 value type for batch read/modify/write operations
///
/// Use `sleepconf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepconf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepConf0Val {
    bits: u32,
}

impl SleepConf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WK_CHAR4** `[31:24]`
    ///
    /// This register restores the specified wake up char4 to wake up
    #[inline(always)]
    pub const fn wk_char4(&self) -> u32 {
        (self.bits & SleepConf0::WK_CHAR4_MASK) >> SleepConf0::WK_CHAR4_OFFSET
    }

    /// Read **WK_CHAR3** `[23:16]`
    ///
    /// This register restores the specified wake up char3 to wake up
    #[inline(always)]
    pub const fn wk_char3(&self) -> u32 {
        (self.bits & SleepConf0::WK_CHAR3_MASK) >> SleepConf0::WK_CHAR3_OFFSET
    }

    /// Read **WK_CHAR2** `[15:8]`
    ///
    /// This register restores the specified wake up char2 to wake up
    #[inline(always)]
    pub const fn wk_char2(&self) -> u32 {
        (self.bits & SleepConf0::WK_CHAR2_MASK) >> SleepConf0::WK_CHAR2_OFFSET
    }

    /// Read **WK_CHAR1** `[7:0]`
    ///
    /// This register restores the specified wake up char1 to wake up
    #[inline(always)]
    pub const fn wk_char1(&self) -> u32 {
        (self.bits & SleepConf0::WK_CHAR1_MASK) >> SleepConf0::WK_CHAR1_OFFSET
    }

    /// Set **WK_CHAR4** `[31:24]`
    ///
    /// This register restores the specified wake up char4 to wake up
    #[inline(always)]
    pub const fn with_wk_char4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf0::WK_CHAR4_MASK)
                | ((value << SleepConf0::WK_CHAR4_OFFSET) & SleepConf0::WK_CHAR4_MASK),
        }
    }

    /// Set **WK_CHAR3** `[23:16]`
    ///
    /// This register restores the specified wake up char3 to wake up
    #[inline(always)]
    pub const fn with_wk_char3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf0::WK_CHAR3_MASK)
                | ((value << SleepConf0::WK_CHAR3_OFFSET) & SleepConf0::WK_CHAR3_MASK),
        }
    }

    /// Set **WK_CHAR2** `[15:8]`
    ///
    /// This register restores the specified wake up char2 to wake up
    #[inline(always)]
    pub const fn with_wk_char2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf0::WK_CHAR2_MASK)
                | ((value << SleepConf0::WK_CHAR2_OFFSET) & SleepConf0::WK_CHAR2_MASK),
        }
    }

    /// Set **WK_CHAR1** `[7:0]`
    ///
    /// This register restores the specified wake up char1 to wake up
    #[inline(always)]
    pub const fn with_wk_char1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf0::WK_CHAR1_MASK)
                | ((value << SleepConf0::WK_CHAR1_OFFSET) & SleepConf0::WK_CHAR1_MASK),
        }
    }
}

impl Default for SleepConf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SleepConf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepConf0Val {
        SleepConf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepConf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepConf0Val) -> SleepConf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLEEP_CONF1 Register
///
/// UART sleep configure register 1
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **WK_CHAR0** `[7:0]` (RW)
///
///   This register restores the specified char0 to wake up
#[repr(transparent)]
pub struct SleepConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SleepConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SleepConf1 {
    // ======== Field Constants ========

    /// **WK_CHAR0**
    ///
    /// This register restores the specified char0 to wake up
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WK_CHAR0_OFFSET: u32 = 0;
    pub const WK_CHAR0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **WK_CHAR0** `[7:0]` (RW)
    ///
    /// This register restores the specified char0 to wake up
    #[inline(always)]
    pub fn wk_char0(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR0_MASK) >> Self::WK_CHAR0_OFFSET
    }

    /// Write **WK_CHAR0** `[7:0]` (RW)
    ///
    /// This register restores the specified char0 to wake up
    #[inline(always)]
    pub fn write_wk_char0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR0_MASK) | ((value << Self::WK_CHAR0_OFFSET) & Self::WK_CHAR0_MASK)
        });
    }
}

/// SLEEP_CONF1 value type for batch read/modify/write operations
///
/// Use `sleepconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepConf1Val {
    bits: u32,
}

impl SleepConf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WK_CHAR0** `[7:0]`
    ///
    /// This register restores the specified char0 to wake up
    #[inline(always)]
    pub const fn wk_char0(&self) -> u32 {
        (self.bits & SleepConf1::WK_CHAR0_MASK) >> SleepConf1::WK_CHAR0_OFFSET
    }

    /// Set **WK_CHAR0** `[7:0]`
    ///
    /// This register restores the specified char0 to wake up
    #[inline(always)]
    pub const fn with_wk_char0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf1::WK_CHAR0_MASK)
                | ((value << SleepConf1::WK_CHAR0_OFFSET) & SleepConf1::WK_CHAR0_MASK),
        }
    }
}

impl Default for SleepConf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SleepConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepConf1Val {
        SleepConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepConf1Val) -> SleepConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLEEP_CONF2 Register
///
/// UART sleep configure register 2
///
/// Address offset: 0x0038
/// Reset value: 0x001420f0
///
/// ## Fields
///
/// - **WK_MODE_SEL** `[27:26]` (RW)
///
///   This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
///
/// - **WK_CHAR_MASK** `[25:21]` (RW)
///
///   This register is used to mask wake up char.
///
/// - **WK_CHAR_NUM** `[20:18]` (RW)
///
///   This register is used to select number of wake up char.
///
/// - **RX_WAKE_UP_THRHD** `[17:13]` (RW)
///
///   In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
///
/// - **ACTIVE_THRESHOLD** `[9:0]` (RW)
///
///   The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
#[repr(transparent)]
pub struct SleepConf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SleepConf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SleepConf2 {
    // ======== Field Constants ========

    /// **WK_MODE_SEL**
    ///
    /// This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const WK_MODE_SEL_OFFSET: u32 = 26;
    pub const WK_MODE_SEL_MASK: u32 = 0x0c000000;

    /// **WK_CHAR_MASK**
    ///
    /// This register is used to mask wake up char.
    ///
    /// Bits `[25:21]`, mask `0x03e00000`
    pub const WK_CHAR_MASK_OFFSET: u32 = 21;
    pub const WK_CHAR_MASK_MASK: u32 = 0x03e00000;

    /// **WK_CHAR_NUM**
    ///
    /// This register is used to select number of wake up char.
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const WK_CHAR_NUM_OFFSET: u32 = 18;
    pub const WK_CHAR_NUM_MASK: u32 = 0x001c0000;

    /// **RX_WAKE_UP_THRHD**
    ///
    /// In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const RX_WAKE_UP_THRHD_OFFSET: u32 = 13;
    pub const RX_WAKE_UP_THRHD_MASK: u32 = 0x0003e000;

    /// **ACTIVE_THRESHOLD**
    ///
    /// The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const ACTIVE_THRESHOLD_OFFSET: u32 = 0;
    pub const ACTIVE_THRESHOLD_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **WK_MODE_SEL** `[27:26]` (RW)
    ///
    /// This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
    #[inline(always)]
    pub fn wk_mode_sel(&self) -> u32 {
        (self.value.read_bits() & Self::WK_MODE_SEL_MASK) >> Self::WK_MODE_SEL_OFFSET
    }

    /// Write **WK_MODE_SEL** `[27:26]` (RW)
    ///
    /// This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
    #[inline(always)]
    pub fn write_wk_mode_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_MODE_SEL_MASK)
                | ((value << Self::WK_MODE_SEL_OFFSET) & Self::WK_MODE_SEL_MASK)
        });
    }

    /// **WK_CHAR_MASK** `[25:21]` (RW)
    ///
    /// This register is used to mask wake up char.
    #[inline(always)]
    pub fn wk_char_mask(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR_MASK_MASK) >> Self::WK_CHAR_MASK_OFFSET
    }

    /// Write **WK_CHAR_MASK** `[25:21]` (RW)
    ///
    /// This register is used to mask wake up char.
    #[inline(always)]
    pub fn write_wk_char_mask(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR_MASK_MASK)
                | ((value << Self::WK_CHAR_MASK_OFFSET) & Self::WK_CHAR_MASK_MASK)
        });
    }

    /// **WK_CHAR_NUM** `[20:18]` (RW)
    ///
    /// This register is used to select number of wake up char.
    #[inline(always)]
    pub fn wk_char_num(&self) -> u32 {
        (self.value.read_bits() & Self::WK_CHAR_NUM_MASK) >> Self::WK_CHAR_NUM_OFFSET
    }

    /// Write **WK_CHAR_NUM** `[20:18]` (RW)
    ///
    /// This register is used to select number of wake up char.
    #[inline(always)]
    pub fn write_wk_char_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WK_CHAR_NUM_MASK)
                | ((value << Self::WK_CHAR_NUM_OFFSET) & Self::WK_CHAR_NUM_MASK)
        });
    }

    /// **RX_WAKE_UP_THRHD** `[17:13]` (RW)
    ///
    /// In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RX_WAKE_UP_THRHD_MASK) >> Self::RX_WAKE_UP_THRHD_OFFSET
    }

    /// Write **RX_WAKE_UP_THRHD** `[17:13]` (RW)
    ///
    /// In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
    #[inline(always)]
    pub fn write_rx_wake_up_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_WAKE_UP_THRHD_MASK)
                | ((value << Self::RX_WAKE_UP_THRHD_OFFSET) & Self::RX_WAKE_UP_THRHD_MASK)
        });
    }

    /// **ACTIVE_THRESHOLD** `[9:0]` (RW)
    ///
    /// The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
    #[inline(always)]
    pub fn active_threshold(&self) -> u32 {
        (self.value.read_bits() & Self::ACTIVE_THRESHOLD_MASK) >> Self::ACTIVE_THRESHOLD_OFFSET
    }

    /// Write **ACTIVE_THRESHOLD** `[9:0]` (RW)
    ///
    /// The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
    #[inline(always)]
    pub fn write_active_threshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ACTIVE_THRESHOLD_MASK)
                | ((value << Self::ACTIVE_THRESHOLD_OFFSET) & Self::ACTIVE_THRESHOLD_MASK)
        });
    }
}

/// SLEEP_CONF2 value type for batch read/modify/write operations
///
/// Use `sleepconf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepconf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepConf2Val {
    bits: u32,
}

impl SleepConf2Val {
    /// Create from raw bits
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
        Self { bits: 0x001420f0 }
    }

    /// Read **WK_MODE_SEL** `[27:26]`
    ///
    /// This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
    #[inline(always)]
    pub const fn wk_mode_sel(&self) -> u32 {
        (self.bits & SleepConf2::WK_MODE_SEL_MASK) >> SleepConf2::WK_MODE_SEL_OFFSET
    }

    /// Read **WK_CHAR_MASK** `[25:21]`
    ///
    /// This register is used to mask wake up char.
    #[inline(always)]
    pub const fn wk_char_mask(&self) -> u32 {
        (self.bits & SleepConf2::WK_CHAR_MASK_MASK) >> SleepConf2::WK_CHAR_MASK_OFFSET
    }

    /// Read **WK_CHAR_NUM** `[20:18]`
    ///
    /// This register is used to select number of wake up char.
    #[inline(always)]
    pub const fn wk_char_num(&self) -> u32 {
        (self.bits & SleepConf2::WK_CHAR_NUM_MASK) >> SleepConf2::WK_CHAR_NUM_OFFSET
    }

    /// Read **RX_WAKE_UP_THRHD** `[17:13]`
    ///
    /// In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
    #[inline(always)]
    pub const fn rx_wake_up_thrhd(&self) -> u32 {
        (self.bits & SleepConf2::RX_WAKE_UP_THRHD_MASK) >> SleepConf2::RX_WAKE_UP_THRHD_OFFSET
    }

    /// Read **ACTIVE_THRESHOLD** `[9:0]`
    ///
    /// The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
    #[inline(always)]
    pub const fn active_threshold(&self) -> u32 {
        (self.bits & SleepConf2::ACTIVE_THRESHOLD_MASK) >> SleepConf2::ACTIVE_THRESHOLD_OFFSET
    }

    /// Set **WK_MODE_SEL** `[27:26]`
    ///
    /// This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than
    #[inline(always)]
    pub const fn with_wk_mode_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf2::WK_MODE_SEL_MASK)
                | ((value << SleepConf2::WK_MODE_SEL_OFFSET) & SleepConf2::WK_MODE_SEL_MASK),
        }
    }

    /// Set **WK_CHAR_MASK** `[25:21]`
    ///
    /// This register is used to mask wake up char.
    #[inline(always)]
    pub const fn with_wk_char_mask(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf2::WK_CHAR_MASK_MASK)
                | ((value << SleepConf2::WK_CHAR_MASK_OFFSET) & SleepConf2::WK_CHAR_MASK_MASK),
        }
    }

    /// Set **WK_CHAR_NUM** `[20:18]`
    ///
    /// This register is used to select number of wake up char.
    #[inline(always)]
    pub const fn with_wk_char_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf2::WK_CHAR_NUM_MASK)
                | ((value << SleepConf2::WK_CHAR_NUM_OFFSET) & SleepConf2::WK_CHAR_NUM_MASK),
        }
    }

    /// Set **RX_WAKE_UP_THRHD** `[17:13]`
    ///
    /// In wake up mode 1 this field is used to set the received data number threshold to wake up chip.
    #[inline(always)]
    pub const fn with_rx_wake_up_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf2::RX_WAKE_UP_THRHD_MASK)
                | ((value << SleepConf2::RX_WAKE_UP_THRHD_OFFSET)
                    & SleepConf2::RX_WAKE_UP_THRHD_MASK),
        }
    }

    /// Set **ACTIVE_THRESHOLD** `[9:0]`
    ///
    /// The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value.
    #[inline(always)]
    pub const fn with_active_threshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SleepConf2::ACTIVE_THRESHOLD_MASK)
                | ((value << SleepConf2::ACTIVE_THRESHOLD_OFFSET)
                    & SleepConf2::ACTIVE_THRESHOLD_MASK),
        }
    }
}

impl Default for SleepConf2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SleepConf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepConf2Val {
        SleepConf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepConf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepConf2Val) -> SleepConf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SWFC_CONF0_SYNC Register
///
/// Software flow-control character configuration
///
/// Address offset: 0x003c
/// Reset value: 0x00001311
///
/// ## Fields
///
/// - **SEND_XOFF** `[22]` (RW)
///
///   Set this bit to send Xoff char. It is cleared by hardware automatically.
///
/// - **SEND_XON** `[21]` (RW)
///
///   Set this bit to send Xon char. It is cleared by hardware automatically.
///
/// - **FORCE_XOFF** `[20]` (RW)
///
///   Set this bit to stop the transmitter from sending data.
///
/// - **FORCE_XON** `[19]` (RW)
///
///   Set this bit to enable the transmitter to go on sending data.
///
/// - **XONOFF_DEL** `[18]` (RW)
///
///   Set this bit to remove flow control char from the received data.
///
/// - **SW_FLOW_CON_EN** `[17]` (RW)
///
///   Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff.
///
/// - **XON_XOFF_STILL_SEND** `[16]` (RW)
///
///   In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled.
///
/// - **XOFF_CHAR** `[15:8]` (RW)
///
///   This register stores the Xoff flow control char.
///
/// - **XON_CHAR** `[7:0]` (RW)
///
///   This register stores the Xon flow control char.
#[repr(transparent)]
pub struct SwfcConf0Sync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwfcConf0Sync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwfcConf0Sync {
    // ======== Field Constants ========

    /// **SEND_XOFF**
    ///
    /// Set this bit to send Xoff char. It is cleared by hardware automatically.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SEND_XOFF_OFFSET: u32 = 22;
    pub const SEND_XOFF_MASK: u32 = 0x00400000;
    pub const SEND_XOFF: u32 = 1 << 22;

    /// **SEND_XON**
    ///
    /// Set this bit to send Xon char. It is cleared by hardware automatically.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SEND_XON_OFFSET: u32 = 21;
    pub const SEND_XON_MASK: u32 = 0x00200000;
    pub const SEND_XON: u32 = 1 << 21;

    /// **FORCE_XOFF**
    ///
    /// Set this bit to stop the transmitter from sending data.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const FORCE_XOFF_OFFSET: u32 = 20;
    pub const FORCE_XOFF_MASK: u32 = 0x00100000;
    pub const FORCE_XOFF: u32 = 1 << 20;

    /// **FORCE_XON**
    ///
    /// Set this bit to enable the transmitter to go on sending data.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const FORCE_XON_OFFSET: u32 = 19;
    pub const FORCE_XON_MASK: u32 = 0x00080000;
    pub const FORCE_XON: u32 = 1 << 19;

    /// **XONOFF_DEL**
    ///
    /// Set this bit to remove flow control char from the received data.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const XONOFF_DEL_OFFSET: u32 = 18;
    pub const XONOFF_DEL_MASK: u32 = 0x00040000;
    pub const XONOFF_DEL: u32 = 1 << 18;

    /// **SW_FLOW_CON_EN**
    ///
    /// Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SW_FLOW_CON_EN_OFFSET: u32 = 17;
    pub const SW_FLOW_CON_EN_MASK: u32 = 0x00020000;
    pub const SW_FLOW_CON_EN: u32 = 1 << 17;

    /// **XON_XOFF_STILL_SEND**
    ///
    /// In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const XON_XOFF_STILL_SEND_OFFSET: u32 = 16;
    pub const XON_XOFF_STILL_SEND_MASK: u32 = 0x00010000;
    pub const XON_XOFF_STILL_SEND: u32 = 1 << 16;

    /// **XOFF_CHAR**
    ///
    /// This register stores the Xoff flow control char.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const XOFF_CHAR_OFFSET: u32 = 8;
    pub const XOFF_CHAR_MASK: u32 = 0x0000ff00;

    /// **XON_CHAR**
    ///
    /// This register stores the Xon flow control char.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const XON_CHAR_OFFSET: u32 = 0;
    pub const XON_CHAR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SEND_XOFF** `[22]` (RW)
    ///
    /// Set this bit to send Xoff char. It is cleared by hardware automatically.
    #[inline(always)]
    pub fn send_xoff(&self) -> bool {
        (self.value.read_bits() & Self::SEND_XOFF_MASK) != 0
    }

    /// Set SEND_XOFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_xoff(&self) {
        self.value.modify_bits(|r| r | Self::SEND_XOFF_MASK);
    }

    /// Clear SEND_XOFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_xoff(&self) {
        self.value.modify_bits(|r| r & !Self::SEND_XOFF_MASK);
    }

    /// **SEND_XON** `[21]` (RW)
    ///
    /// Set this bit to send Xon char. It is cleared by hardware automatically.
    #[inline(always)]
    pub fn send_xon(&self) -> bool {
        (self.value.read_bits() & Self::SEND_XON_MASK) != 0
    }

    /// Set SEND_XON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_xon(&self) {
        self.value.modify_bits(|r| r | Self::SEND_XON_MASK);
    }

    /// Clear SEND_XON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_xon(&self) {
        self.value.modify_bits(|r| r & !Self::SEND_XON_MASK);
    }

    /// **FORCE_XOFF** `[20]` (RW)
    ///
    /// Set this bit to stop the transmitter from sending data.
    #[inline(always)]
    pub fn force_xoff(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_XOFF_MASK) != 0
    }

    /// Set FORCE_XOFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_xoff(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_XOFF_MASK);
    }

    /// Clear FORCE_XOFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_xoff(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_XOFF_MASK);
    }

    /// **FORCE_XON** `[19]` (RW)
    ///
    /// Set this bit to enable the transmitter to go on sending data.
    #[inline(always)]
    pub fn force_xon(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_XON_MASK) != 0
    }

    /// Set FORCE_XON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_xon(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_XON_MASK);
    }

    /// Clear FORCE_XON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_xon(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_XON_MASK);
    }

    /// **XONOFF_DEL** `[18]` (RW)
    ///
    /// Set this bit to remove flow control char from the received data.
    #[inline(always)]
    pub fn xonoff_del(&self) -> bool {
        (self.value.read_bits() & Self::XONOFF_DEL_MASK) != 0
    }

    /// Set XONOFF_DEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xonoff_del(&self) {
        self.value.modify_bits(|r| r | Self::XONOFF_DEL_MASK);
    }

    /// Clear XONOFF_DEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xonoff_del(&self) {
        self.value.modify_bits(|r| r & !Self::XONOFF_DEL_MASK);
    }

    /// **SW_FLOW_CON_EN** `[17]` (RW)
    ///
    /// Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff.
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> bool {
        (self.value.read_bits() & Self::SW_FLOW_CON_EN_MASK) != 0
    }

    /// Set SW_FLOW_CON_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_flow_con_en(&self) {
        self.value.modify_bits(|r| r | Self::SW_FLOW_CON_EN_MASK);
    }

    /// Clear SW_FLOW_CON_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_flow_con_en(&self) {
        self.value.modify_bits(|r| r & !Self::SW_FLOW_CON_EN_MASK);
    }

    /// **XON_XOFF_STILL_SEND** `[16]` (RW)
    ///
    /// In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled.
    #[inline(always)]
    pub fn xon_xoff_still_send(&self) -> bool {
        (self.value.read_bits() & Self::XON_XOFF_STILL_SEND_MASK) != 0
    }

    /// Set XON_XOFF_STILL_SEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xon_xoff_still_send(&self) {
        self.value
            .modify_bits(|r| r | Self::XON_XOFF_STILL_SEND_MASK);
    }

    /// Clear XON_XOFF_STILL_SEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xon_xoff_still_send(&self) {
        self.value
            .modify_bits(|r| r & !Self::XON_XOFF_STILL_SEND_MASK);
    }

    /// **XOFF_CHAR** `[15:8]` (RW)
    ///
    /// This register stores the Xoff flow control char.
    #[inline(always)]
    pub fn xoff_char(&self) -> u32 {
        (self.value.read_bits() & Self::XOFF_CHAR_MASK) >> Self::XOFF_CHAR_OFFSET
    }

    /// Write **XOFF_CHAR** `[15:8]` (RW)
    ///
    /// This register stores the Xoff flow control char.
    #[inline(always)]
    pub fn write_xoff_char(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XOFF_CHAR_MASK) | ((value << Self::XOFF_CHAR_OFFSET) & Self::XOFF_CHAR_MASK)
        });
    }

    /// **XON_CHAR** `[7:0]` (RW)
    ///
    /// This register stores the Xon flow control char.
    #[inline(always)]
    pub fn xon_char(&self) -> u32 {
        (self.value.read_bits() & Self::XON_CHAR_MASK) >> Self::XON_CHAR_OFFSET
    }

    /// Write **XON_CHAR** `[7:0]` (RW)
    ///
    /// This register stores the Xon flow control char.
    #[inline(always)]
    pub fn write_xon_char(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XON_CHAR_MASK) | ((value << Self::XON_CHAR_OFFSET) & Self::XON_CHAR_MASK)
        });
    }
}

/// SWFC_CONF0_SYNC value type for batch read/modify/write operations
///
/// Use `swfcconf0sync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swfcconf0sync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwfcConf0SyncVal {
    bits: u32,
}

impl SwfcConf0SyncVal {
    /// Create from raw bits
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
        Self { bits: 0x00001311 }
    }

    /// Read **SEND_XOFF** `[22:22]`
    ///
    /// Set this bit to send Xoff char. It is cleared by hardware automatically.
    #[inline(always)]
    pub const fn send_xoff(&self) -> bool {
        (self.bits & SwfcConf0Sync::SEND_XOFF_MASK) != 0
    }

    /// Read **SEND_XON** `[21:21]`
    ///
    /// Set this bit to send Xon char. It is cleared by hardware automatically.
    #[inline(always)]
    pub const fn send_xon(&self) -> bool {
        (self.bits & SwfcConf0Sync::SEND_XON_MASK) != 0
    }

    /// Read **FORCE_XOFF** `[20:20]`
    ///
    /// Set this bit to stop the transmitter from sending data.
    #[inline(always)]
    pub const fn force_xoff(&self) -> bool {
        (self.bits & SwfcConf0Sync::FORCE_XOFF_MASK) != 0
    }

    /// Read **FORCE_XON** `[19:19]`
    ///
    /// Set this bit to enable the transmitter to go on sending data.
    #[inline(always)]
    pub const fn force_xon(&self) -> bool {
        (self.bits & SwfcConf0Sync::FORCE_XON_MASK) != 0
    }

    /// Read **XONOFF_DEL** `[18:18]`
    ///
    /// Set this bit to remove flow control char from the received data.
    #[inline(always)]
    pub const fn xonoff_del(&self) -> bool {
        (self.bits & SwfcConf0Sync::XONOFF_DEL_MASK) != 0
    }

    /// Read **SW_FLOW_CON_EN** `[17:17]`
    ///
    /// Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff.
    #[inline(always)]
    pub const fn sw_flow_con_en(&self) -> bool {
        (self.bits & SwfcConf0Sync::SW_FLOW_CON_EN_MASK) != 0
    }

    /// Read **XON_XOFF_STILL_SEND** `[16:16]`
    ///
    /// In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled.
    #[inline(always)]
    pub const fn xon_xoff_still_send(&self) -> bool {
        (self.bits & SwfcConf0Sync::XON_XOFF_STILL_SEND_MASK) != 0
    }

    /// Read **XOFF_CHAR** `[15:8]`
    ///
    /// This register stores the Xoff flow control char.
    #[inline(always)]
    pub const fn xoff_char(&self) -> u32 {
        (self.bits & SwfcConf0Sync::XOFF_CHAR_MASK) >> SwfcConf0Sync::XOFF_CHAR_OFFSET
    }

    /// Read **XON_CHAR** `[7:0]`
    ///
    /// This register stores the Xon flow control char.
    #[inline(always)]
    pub const fn xon_char(&self) -> u32 {
        (self.bits & SwfcConf0Sync::XON_CHAR_MASK) >> SwfcConf0Sync::XON_CHAR_OFFSET
    }

    /// Set **SEND_XOFF** `[22:22]`
    ///
    /// Set this bit to send Xoff char. It is cleared by hardware automatically.
    #[inline(always)]
    pub const fn with_send_xoff(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::SEND_XOFF_MASK
            } else {
                self.bits & !SwfcConf0Sync::SEND_XOFF_MASK
            },
        }
    }

    /// Set **SEND_XON** `[21:21]`
    ///
    /// Set this bit to send Xon char. It is cleared by hardware automatically.
    #[inline(always)]
    pub const fn with_send_xon(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::SEND_XON_MASK
            } else {
                self.bits & !SwfcConf0Sync::SEND_XON_MASK
            },
        }
    }

    /// Set **FORCE_XOFF** `[20:20]`
    ///
    /// Set this bit to stop the transmitter from sending data.
    #[inline(always)]
    pub const fn with_force_xoff(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::FORCE_XOFF_MASK
            } else {
                self.bits & !SwfcConf0Sync::FORCE_XOFF_MASK
            },
        }
    }

    /// Set **FORCE_XON** `[19:19]`
    ///
    /// Set this bit to enable the transmitter to go on sending data.
    #[inline(always)]
    pub const fn with_force_xon(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::FORCE_XON_MASK
            } else {
                self.bits & !SwfcConf0Sync::FORCE_XON_MASK
            },
        }
    }

    /// Set **XONOFF_DEL** `[18:18]`
    ///
    /// Set this bit to remove flow control char from the received data.
    #[inline(always)]
    pub const fn with_xonoff_del(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::XONOFF_DEL_MASK
            } else {
                self.bits & !SwfcConf0Sync::XONOFF_DEL_MASK
            },
        }
    }

    /// Set **SW_FLOW_CON_EN** `[17:17]`
    ///
    /// Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff.
    #[inline(always)]
    pub const fn with_sw_flow_con_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::SW_FLOW_CON_EN_MASK
            } else {
                self.bits & !SwfcConf0Sync::SW_FLOW_CON_EN_MASK
            },
        }
    }

    /// Set **XON_XOFF_STILL_SEND** `[16:16]`
    ///
    /// In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled.
    #[inline(always)]
    pub const fn with_xon_xoff_still_send(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwfcConf0Sync::XON_XOFF_STILL_SEND_MASK
            } else {
                self.bits & !SwfcConf0Sync::XON_XOFF_STILL_SEND_MASK
            },
        }
    }

    /// Set **XOFF_CHAR** `[15:8]`
    ///
    /// This register stores the Xoff flow control char.
    #[inline(always)]
    pub const fn with_xoff_char(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwfcConf0Sync::XOFF_CHAR_MASK)
                | ((value << SwfcConf0Sync::XOFF_CHAR_OFFSET) & SwfcConf0Sync::XOFF_CHAR_MASK),
        }
    }

    /// Set **XON_CHAR** `[7:0]`
    ///
    /// This register stores the Xon flow control char.
    #[inline(always)]
    pub const fn with_xon_char(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwfcConf0Sync::XON_CHAR_MASK)
                | ((value << SwfcConf0Sync::XON_CHAR_OFFSET) & SwfcConf0Sync::XON_CHAR_MASK),
        }
    }
}

impl Default for SwfcConf0SyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwfcConf0Sync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwfcConf0SyncVal {
        SwfcConf0SyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwfcConf0SyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwfcConf0SyncVal) -> SwfcConf0SyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SWFC_CONF1 Register
///
/// Software flow-control character configuration
///
/// Address offset: 0x0040
/// Reset value: 0x00006000
///
/// ## Fields
///
/// - **XOFF_THRESHOLD** `[15:11]` (RW)
///
///   When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
///
/// - **XON_THRESHOLD** `[7:3]` (RW)
///
///   When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
#[repr(transparent)]
pub struct SwfcConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwfcConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwfcConf1 {
    // ======== Field Constants ========

    /// **XOFF_THRESHOLD**
    ///
    /// When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
    ///
    /// Bits `[15:11]`, mask `0x0000f800`
    pub const XOFF_THRESHOLD_OFFSET: u32 = 11;
    pub const XOFF_THRESHOLD_MASK: u32 = 0x0000f800;

    /// **XON_THRESHOLD**
    ///
    /// When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const XON_THRESHOLD_OFFSET: u32 = 3;
    pub const XON_THRESHOLD_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **XOFF_THRESHOLD** `[15:11]` (RW)
    ///
    /// When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
    #[inline(always)]
    pub fn xoff_threshold(&self) -> u32 {
        (self.value.read_bits() & Self::XOFF_THRESHOLD_MASK) >> Self::XOFF_THRESHOLD_OFFSET
    }

    /// Write **XOFF_THRESHOLD** `[15:11]` (RW)
    ///
    /// When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
    #[inline(always)]
    pub fn write_xoff_threshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XOFF_THRESHOLD_MASK)
                | ((value << Self::XOFF_THRESHOLD_OFFSET) & Self::XOFF_THRESHOLD_MASK)
        });
    }

    /// **XON_THRESHOLD** `[7:3]` (RW)
    ///
    /// When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
    #[inline(always)]
    pub fn xon_threshold(&self) -> u32 {
        (self.value.read_bits() & Self::XON_THRESHOLD_MASK) >> Self::XON_THRESHOLD_OFFSET
    }

    /// Write **XON_THRESHOLD** `[7:3]` (RW)
    ///
    /// When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
    #[inline(always)]
    pub fn write_xon_threshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XON_THRESHOLD_MASK)
                | ((value << Self::XON_THRESHOLD_OFFSET) & Self::XON_THRESHOLD_MASK)
        });
    }
}

/// SWFC_CONF1 value type for batch read/modify/write operations
///
/// Use `swfcconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swfcconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwfcConf1Val {
    bits: u32,
}

impl SwfcConf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00006000 }
    }

    /// Read **XOFF_THRESHOLD** `[15:11]`
    ///
    /// When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
    #[inline(always)]
    pub const fn xoff_threshold(&self) -> u32 {
        (self.bits & SwfcConf1::XOFF_THRESHOLD_MASK) >> SwfcConf1::XOFF_THRESHOLD_OFFSET
    }

    /// Read **XON_THRESHOLD** `[7:3]`
    ///
    /// When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
    #[inline(always)]
    pub const fn xon_threshold(&self) -> u32 {
        (self.bits & SwfcConf1::XON_THRESHOLD_MASK) >> SwfcConf1::XON_THRESHOLD_OFFSET
    }

    /// Set **XOFF_THRESHOLD** `[15:11]`
    ///
    /// When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char.
    #[inline(always)]
    pub const fn with_xoff_threshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwfcConf1::XOFF_THRESHOLD_MASK)
                | ((value << SwfcConf1::XOFF_THRESHOLD_OFFSET) & SwfcConf1::XOFF_THRESHOLD_MASK),
        }
    }

    /// Set **XON_THRESHOLD** `[7:3]`
    ///
    /// When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char.
    #[inline(always)]
    pub const fn with_xon_threshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwfcConf1::XON_THRESHOLD_MASK)
                | ((value << SwfcConf1::XON_THRESHOLD_OFFSET) & SwfcConf1::XON_THRESHOLD_MASK),
        }
    }
}

impl Default for SwfcConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwfcConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwfcConf1Val {
        SwfcConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwfcConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwfcConf1Val) -> SwfcConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TXBRK_CONF_SYNC Register
///
/// Tx Break character configuration
///
/// Address offset: 0x0044
/// Reset value: 0x0000000a
///
/// ## Fields
///
/// - **TX_BRK_NUM** `[7:0]` (RW)
///
///   This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
#[repr(transparent)]
pub struct TxbrkConfSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxbrkConfSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxbrkConfSync {
    // ======== Field Constants ========

    /// **TX_BRK_NUM**
    ///
    /// This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_BRK_NUM_OFFSET: u32 = 0;
    pub const TX_BRK_NUM_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_BRK_NUM** `[7:0]` (RW)
    ///
    /// This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
    #[inline(always)]
    pub fn tx_brk_num(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BRK_NUM_MASK) >> Self::TX_BRK_NUM_OFFSET
    }

    /// Write **TX_BRK_NUM** `[7:0]` (RW)
    ///
    /// This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
    #[inline(always)]
    pub fn write_tx_brk_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_BRK_NUM_MASK)
                | ((value << Self::TX_BRK_NUM_OFFSET) & Self::TX_BRK_NUM_MASK)
        });
    }
}

/// TXBRK_CONF_SYNC value type for batch read/modify/write operations
///
/// Use `txbrkconfsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txbrkconfsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxbrkConfSyncVal {
    bits: u32,
}

impl TxbrkConfSyncVal {
    /// Create from raw bits
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

    /// Read **TX_BRK_NUM** `[7:0]`
    ///
    /// This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
    #[inline(always)]
    pub const fn tx_brk_num(&self) -> u32 {
        (self.bits & TxbrkConfSync::TX_BRK_NUM_MASK) >> TxbrkConfSync::TX_BRK_NUM_OFFSET
    }

    /// Set **TX_BRK_NUM** `[7:0]`
    ///
    /// This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1.
    #[inline(always)]
    pub const fn with_tx_brk_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxbrkConfSync::TX_BRK_NUM_MASK)
                | ((value << TxbrkConfSync::TX_BRK_NUM_OFFSET) & TxbrkConfSync::TX_BRK_NUM_MASK),
        }
    }
}

impl Default for TxbrkConfSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxbrkConfSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxbrkConfSyncVal {
        TxbrkConfSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxbrkConfSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxbrkConfSyncVal) -> TxbrkConfSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDLE_CONF_SYNC Register
///
/// Frame-end idle configuration
///
/// Address offset: 0x0048
/// Reset value: 0x00040100
///
/// ## Fields
///
/// - **TX_IDLE_NUM** `[19:10]` (RW)
///
///   This register is used to configure the duration time between transfers.
///
/// - **RX_IDLE_THRHD** `[9:0]` (RW)
///
///   It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
#[repr(transparent)]
pub struct IdleConfSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IdleConfSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IdleConfSync {
    // ======== Field Constants ========

    /// **TX_IDLE_NUM**
    ///
    /// This register is used to configure the duration time between transfers.
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const TX_IDLE_NUM_OFFSET: u32 = 10;
    pub const TX_IDLE_NUM_MASK: u32 = 0x000ffc00;

    /// **RX_IDLE_THRHD**
    ///
    /// It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const RX_IDLE_THRHD_OFFSET: u32 = 0;
    pub const RX_IDLE_THRHD_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **TX_IDLE_NUM** `[19:10]` (RW)
    ///
    /// This register is used to configure the duration time between transfers.
    #[inline(always)]
    pub fn tx_idle_num(&self) -> u32 {
        (self.value.read_bits() & Self::TX_IDLE_NUM_MASK) >> Self::TX_IDLE_NUM_OFFSET
    }

    /// Write **TX_IDLE_NUM** `[19:10]` (RW)
    ///
    /// This register is used to configure the duration time between transfers.
    #[inline(always)]
    pub fn write_tx_idle_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_IDLE_NUM_MASK)
                | ((value << Self::TX_IDLE_NUM_OFFSET) & Self::TX_IDLE_NUM_MASK)
        });
    }

    /// **RX_IDLE_THRHD** `[9:0]` (RW)
    ///
    /// It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RX_IDLE_THRHD_MASK) >> Self::RX_IDLE_THRHD_OFFSET
    }

    /// Write **RX_IDLE_THRHD** `[9:0]` (RW)
    ///
    /// It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
    #[inline(always)]
    pub fn write_rx_idle_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_IDLE_THRHD_MASK)
                | ((value << Self::RX_IDLE_THRHD_OFFSET) & Self::RX_IDLE_THRHD_MASK)
        });
    }
}

/// IDLE_CONF_SYNC value type for batch read/modify/write operations
///
/// Use `idleconfsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idleconfsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdleConfSyncVal {
    bits: u32,
}

impl IdleConfSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x00040100 }
    }

    /// Read **TX_IDLE_NUM** `[19:10]`
    ///
    /// This register is used to configure the duration time between transfers.
    #[inline(always)]
    pub const fn tx_idle_num(&self) -> u32 {
        (self.bits & IdleConfSync::TX_IDLE_NUM_MASK) >> IdleConfSync::TX_IDLE_NUM_OFFSET
    }

    /// Read **RX_IDLE_THRHD** `[9:0]`
    ///
    /// It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
    #[inline(always)]
    pub const fn rx_idle_thrhd(&self) -> u32 {
        (self.bits & IdleConfSync::RX_IDLE_THRHD_MASK) >> IdleConfSync::RX_IDLE_THRHD_OFFSET
    }

    /// Set **TX_IDLE_NUM** `[19:10]`
    ///
    /// This register is used to configure the duration time between transfers.
    #[inline(always)]
    pub const fn with_tx_idle_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IdleConfSync::TX_IDLE_NUM_MASK)
                | ((value << IdleConfSync::TX_IDLE_NUM_OFFSET) & IdleConfSync::TX_IDLE_NUM_MASK),
        }
    }

    /// Set **RX_IDLE_THRHD** `[9:0]`
    ///
    /// It will produce frame end signal when receiver takes more time to receive one byte data than this register value.
    #[inline(always)]
    pub const fn with_rx_idle_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IdleConfSync::RX_IDLE_THRHD_MASK)
                | ((value << IdleConfSync::RX_IDLE_THRHD_OFFSET)
                    & IdleConfSync::RX_IDLE_THRHD_MASK),
        }
    }
}

impl Default for IdleConfSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IdleConfSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdleConfSyncVal {
        IdleConfSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdleConfSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdleConfSyncVal) -> IdleConfSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RS485_CONF_SYNC Register
///
/// RS485 mode configuration
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **DL1_EN** `[2]` (RW)
///
///   Set this bit to delay the stop bit by 1 bit.
///
/// - **DL0_EN** `[1]` (RW)
///
///   Set this bit to delay the stop bit by 1 bit.
#[repr(transparent)]
pub struct Rs485ConfSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rs485ConfSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rs485ConfSync {
    // ======== Field Constants ========

    /// **DL1_EN**
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DL1_EN_OFFSET: u32 = 2;
    pub const DL1_EN_MASK: u32 = 0x00000004;
    pub const DL1_EN: u32 = 1 << 2;

    /// **DL0_EN**
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DL0_EN_OFFSET: u32 = 1;
    pub const DL0_EN_MASK: u32 = 0x00000002;
    pub const DL0_EN: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **DL1_EN** `[2]` (RW)
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl1_en(&self) -> bool {
        (self.value.read_bits() & Self::DL1_EN_MASK) != 0
    }

    /// Set DL1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dl1_en(&self) {
        self.value.modify_bits(|r| r | Self::DL1_EN_MASK);
    }

    /// Clear DL1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dl1_en(&self) {
        self.value.modify_bits(|r| r & !Self::DL1_EN_MASK);
    }

    /// **DL0_EN** `[1]` (RW)
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl0_en(&self) -> bool {
        (self.value.read_bits() & Self::DL0_EN_MASK) != 0
    }

    /// Set DL0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dl0_en(&self) {
        self.value.modify_bits(|r| r | Self::DL0_EN_MASK);
    }

    /// Clear DL0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dl0_en(&self) {
        self.value.modify_bits(|r| r & !Self::DL0_EN_MASK);
    }
}

/// RS485_CONF_SYNC value type for batch read/modify/write operations
///
/// Use `rs485confsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rs485confsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rs485ConfSyncVal {
    bits: u32,
}

impl Rs485ConfSyncVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DL1_EN** `[2:2]`
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub const fn dl1_en(&self) -> bool {
        (self.bits & Rs485ConfSync::DL1_EN_MASK) != 0
    }

    /// Read **DL0_EN** `[1:1]`
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub const fn dl0_en(&self) -> bool {
        (self.bits & Rs485ConfSync::DL0_EN_MASK) != 0
    }

    /// Set **DL1_EN** `[2:2]`
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub const fn with_dl1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rs485ConfSync::DL1_EN_MASK
            } else {
                self.bits & !Rs485ConfSync::DL1_EN_MASK
            },
        }
    }

    /// Set **DL0_EN** `[1:1]`
    ///
    /// Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub const fn with_dl0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rs485ConfSync::DL0_EN_MASK
            } else {
                self.bits & !Rs485ConfSync::DL0_EN_MASK
            },
        }
    }
}

impl Default for Rs485ConfSyncVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rs485ConfSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rs485ConfSyncVal {
        Rs485ConfSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rs485ConfSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rs485ConfSyncVal) -> Rs485ConfSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AT_CMD_PRECNT_SYNC Register
///
/// Pre-sequence timing configuration
///
/// Address offset: 0x0050
/// Reset value: 0x00000901
///
/// ## Fields
///
/// - **PRE_IDLE_NUM** `[15:0]` (RW)
///
///   This register is used to configure the idle duration time before the first at_cmd is received by receiver.
#[repr(transparent)]
pub struct AtCmdPrecntSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AtCmdPrecntSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AtCmdPrecntSync {
    // ======== Field Constants ========

    /// **PRE_IDLE_NUM**
    ///
    /// This register is used to configure the idle duration time before the first at_cmd is received by receiver.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PRE_IDLE_NUM_OFFSET: u32 = 0;
    pub const PRE_IDLE_NUM_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PRE_IDLE_NUM** `[15:0]` (RW)
    ///
    /// This register is used to configure the idle duration time before the first at_cmd is received by receiver.
    #[inline(always)]
    pub fn pre_idle_num(&self) -> u32 {
        (self.value.read_bits() & Self::PRE_IDLE_NUM_MASK) >> Self::PRE_IDLE_NUM_OFFSET
    }

    /// Write **PRE_IDLE_NUM** `[15:0]` (RW)
    ///
    /// This register is used to configure the idle duration time before the first at_cmd is received by receiver.
    #[inline(always)]
    pub fn write_pre_idle_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PRE_IDLE_NUM_MASK)
                | ((value << Self::PRE_IDLE_NUM_OFFSET) & Self::PRE_IDLE_NUM_MASK)
        });
    }
}

/// AT_CMD_PRECNT_SYNC value type for batch read/modify/write operations
///
/// Use `atcmdprecntsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atcmdprecntsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AtCmdPrecntSyncVal {
    bits: u32,
}

impl AtCmdPrecntSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x00000901 }
    }

    /// Read **PRE_IDLE_NUM** `[15:0]`
    ///
    /// This register is used to configure the idle duration time before the first at_cmd is received by receiver.
    #[inline(always)]
    pub const fn pre_idle_num(&self) -> u32 {
        (self.bits & AtCmdPrecntSync::PRE_IDLE_NUM_MASK) >> AtCmdPrecntSync::PRE_IDLE_NUM_OFFSET
    }

    /// Set **PRE_IDLE_NUM** `[15:0]`
    ///
    /// This register is used to configure the idle duration time before the first at_cmd is received by receiver.
    #[inline(always)]
    pub const fn with_pre_idle_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AtCmdPrecntSync::PRE_IDLE_NUM_MASK)
                | ((value << AtCmdPrecntSync::PRE_IDLE_NUM_OFFSET)
                    & AtCmdPrecntSync::PRE_IDLE_NUM_MASK),
        }
    }
}

impl Default for AtCmdPrecntSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AtCmdPrecntSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AtCmdPrecntSyncVal {
        AtCmdPrecntSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AtCmdPrecntSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AtCmdPrecntSyncVal) -> AtCmdPrecntSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AT_CMD_POSTCNT_SYNC Register
///
/// Post-sequence timing configuration
///
/// Address offset: 0x0054
/// Reset value: 0x00000901
///
/// ## Fields
///
/// - **POST_IDLE_NUM** `[15:0]` (RW)
///
///   This register is used to configure the duration time between the last at_cmd and the next data.
#[repr(transparent)]
pub struct AtCmdPostcntSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AtCmdPostcntSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AtCmdPostcntSync {
    // ======== Field Constants ========

    /// **POST_IDLE_NUM**
    ///
    /// This register is used to configure the duration time between the last at_cmd and the next data.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const POST_IDLE_NUM_OFFSET: u32 = 0;
    pub const POST_IDLE_NUM_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **POST_IDLE_NUM** `[15:0]` (RW)
    ///
    /// This register is used to configure the duration time between the last at_cmd and the next data.
    #[inline(always)]
    pub fn post_idle_num(&self) -> u32 {
        (self.value.read_bits() & Self::POST_IDLE_NUM_MASK) >> Self::POST_IDLE_NUM_OFFSET
    }

    /// Write **POST_IDLE_NUM** `[15:0]` (RW)
    ///
    /// This register is used to configure the duration time between the last at_cmd and the next data.
    #[inline(always)]
    pub fn write_post_idle_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::POST_IDLE_NUM_MASK)
                | ((value << Self::POST_IDLE_NUM_OFFSET) & Self::POST_IDLE_NUM_MASK)
        });
    }
}

/// AT_CMD_POSTCNT_SYNC value type for batch read/modify/write operations
///
/// Use `atcmdpostcntsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atcmdpostcntsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AtCmdPostcntSyncVal {
    bits: u32,
}

impl AtCmdPostcntSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x00000901 }
    }

    /// Read **POST_IDLE_NUM** `[15:0]`
    ///
    /// This register is used to configure the duration time between the last at_cmd and the next data.
    #[inline(always)]
    pub const fn post_idle_num(&self) -> u32 {
        (self.bits & AtCmdPostcntSync::POST_IDLE_NUM_MASK) >> AtCmdPostcntSync::POST_IDLE_NUM_OFFSET
    }

    /// Set **POST_IDLE_NUM** `[15:0]`
    ///
    /// This register is used to configure the duration time between the last at_cmd and the next data.
    #[inline(always)]
    pub const fn with_post_idle_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AtCmdPostcntSync::POST_IDLE_NUM_MASK)
                | ((value << AtCmdPostcntSync::POST_IDLE_NUM_OFFSET)
                    & AtCmdPostcntSync::POST_IDLE_NUM_MASK),
        }
    }
}

impl Default for AtCmdPostcntSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AtCmdPostcntSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AtCmdPostcntSyncVal {
        AtCmdPostcntSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AtCmdPostcntSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AtCmdPostcntSyncVal) -> AtCmdPostcntSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AT_CMD_GAPTOUT_SYNC Register
///
/// Timeout configuration
///
/// Address offset: 0x0058
/// Reset value: 0x0000000b
///
/// ## Fields
///
/// - **RX_GAP_TOUT** `[15:0]` (RW)
///
///   This register is used to configure the duration time between the at_cmd chars.
#[repr(transparent)]
pub struct AtCmdGaptoutSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AtCmdGaptoutSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AtCmdGaptoutSync {
    // ======== Field Constants ========

    /// **RX_GAP_TOUT**
    ///
    /// This register is used to configure the duration time between the at_cmd chars.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RX_GAP_TOUT_OFFSET: u32 = 0;
    pub const RX_GAP_TOUT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RX_GAP_TOUT** `[15:0]` (RW)
    ///
    /// This register is used to configure the duration time between the at_cmd chars.
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> u32 {
        (self.value.read_bits() & Self::RX_GAP_TOUT_MASK) >> Self::RX_GAP_TOUT_OFFSET
    }

    /// Write **RX_GAP_TOUT** `[15:0]` (RW)
    ///
    /// This register is used to configure the duration time between the at_cmd chars.
    #[inline(always)]
    pub fn write_rx_gap_tout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_GAP_TOUT_MASK)
                | ((value << Self::RX_GAP_TOUT_OFFSET) & Self::RX_GAP_TOUT_MASK)
        });
    }
}

/// AT_CMD_GAPTOUT_SYNC value type for batch read/modify/write operations
///
/// Use `atcmdgaptoutsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atcmdgaptoutsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AtCmdGaptoutSyncVal {
    bits: u32,
}

impl AtCmdGaptoutSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x0000000b }
    }

    /// Read **RX_GAP_TOUT** `[15:0]`
    ///
    /// This register is used to configure the duration time between the at_cmd chars.
    #[inline(always)]
    pub const fn rx_gap_tout(&self) -> u32 {
        (self.bits & AtCmdGaptoutSync::RX_GAP_TOUT_MASK) >> AtCmdGaptoutSync::RX_GAP_TOUT_OFFSET
    }

    /// Set **RX_GAP_TOUT** `[15:0]`
    ///
    /// This register is used to configure the duration time between the at_cmd chars.
    #[inline(always)]
    pub const fn with_rx_gap_tout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AtCmdGaptoutSync::RX_GAP_TOUT_MASK)
                | ((value << AtCmdGaptoutSync::RX_GAP_TOUT_OFFSET)
                    & AtCmdGaptoutSync::RX_GAP_TOUT_MASK),
        }
    }
}

impl Default for AtCmdGaptoutSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AtCmdGaptoutSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AtCmdGaptoutSyncVal {
        AtCmdGaptoutSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AtCmdGaptoutSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AtCmdGaptoutSyncVal) -> AtCmdGaptoutSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AT_CMD_CHAR_SYNC Register
///
/// AT escape sequence detection configuration
///
/// Address offset: 0x005c
/// Reset value: 0x0000032b
///
/// ## Fields
///
/// - **CHAR_NUM** `[15:8]` (RW)
///
///   This register is used to configure the num of continuous at_cmd chars received by receiver.
///
/// - **AT_CMD_CHAR** `[7:0]` (RW)
///
///   This register is used to configure the content of at_cmd char.
#[repr(transparent)]
pub struct AtCmdCharSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AtCmdCharSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AtCmdCharSync {
    // ======== Field Constants ========

    /// **CHAR_NUM**
    ///
    /// This register is used to configure the num of continuous at_cmd chars received by receiver.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const CHAR_NUM_OFFSET: u32 = 8;
    pub const CHAR_NUM_MASK: u32 = 0x0000ff00;

    /// **AT_CMD_CHAR**
    ///
    /// This register is used to configure the content of at_cmd char.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const AT_CMD_CHAR_OFFSET: u32 = 0;
    pub const AT_CMD_CHAR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CHAR_NUM** `[15:8]` (RW)
    ///
    /// This register is used to configure the num of continuous at_cmd chars received by receiver.
    #[inline(always)]
    pub fn char_num(&self) -> u32 {
        (self.value.read_bits() & Self::CHAR_NUM_MASK) >> Self::CHAR_NUM_OFFSET
    }

    /// Write **CHAR_NUM** `[15:8]` (RW)
    ///
    /// This register is used to configure the num of continuous at_cmd chars received by receiver.
    #[inline(always)]
    pub fn write_char_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CHAR_NUM_MASK) | ((value << Self::CHAR_NUM_OFFSET) & Self::CHAR_NUM_MASK)
        });
    }

    /// **AT_CMD_CHAR** `[7:0]` (RW)
    ///
    /// This register is used to configure the content of at_cmd char.
    #[inline(always)]
    pub fn at_cmd_char(&self) -> u32 {
        (self.value.read_bits() & Self::AT_CMD_CHAR_MASK) >> Self::AT_CMD_CHAR_OFFSET
    }

    /// Write **AT_CMD_CHAR** `[7:0]` (RW)
    ///
    /// This register is used to configure the content of at_cmd char.
    #[inline(always)]
    pub fn write_at_cmd_char(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AT_CMD_CHAR_MASK)
                | ((value << Self::AT_CMD_CHAR_OFFSET) & Self::AT_CMD_CHAR_MASK)
        });
    }
}

/// AT_CMD_CHAR_SYNC value type for batch read/modify/write operations
///
/// Use `atcmdcharsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atcmdcharsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AtCmdCharSyncVal {
    bits: u32,
}

impl AtCmdCharSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x0000032b }
    }

    /// Read **CHAR_NUM** `[15:8]`
    ///
    /// This register is used to configure the num of continuous at_cmd chars received by receiver.
    #[inline(always)]
    pub const fn char_num(&self) -> u32 {
        (self.bits & AtCmdCharSync::CHAR_NUM_MASK) >> AtCmdCharSync::CHAR_NUM_OFFSET
    }

    /// Read **AT_CMD_CHAR** `[7:0]`
    ///
    /// This register is used to configure the content of at_cmd char.
    #[inline(always)]
    pub const fn at_cmd_char(&self) -> u32 {
        (self.bits & AtCmdCharSync::AT_CMD_CHAR_MASK) >> AtCmdCharSync::AT_CMD_CHAR_OFFSET
    }

    /// Set **CHAR_NUM** `[15:8]`
    ///
    /// This register is used to configure the num of continuous at_cmd chars received by receiver.
    #[inline(always)]
    pub const fn with_char_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AtCmdCharSync::CHAR_NUM_MASK)
                | ((value << AtCmdCharSync::CHAR_NUM_OFFSET) & AtCmdCharSync::CHAR_NUM_MASK),
        }
    }

    /// Set **AT_CMD_CHAR** `[7:0]`
    ///
    /// This register is used to configure the content of at_cmd char.
    #[inline(always)]
    pub const fn with_at_cmd_char(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AtCmdCharSync::AT_CMD_CHAR_MASK)
                | ((value << AtCmdCharSync::AT_CMD_CHAR_OFFSET) & AtCmdCharSync::AT_CMD_CHAR_MASK),
        }
    }
}

impl Default for AtCmdCharSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AtCmdCharSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AtCmdCharSyncVal {
        AtCmdCharSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AtCmdCharSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AtCmdCharSyncVal) -> AtCmdCharSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_CONF Register
///
/// UART memory power configuration
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **MEM_FORCE_PU** `[26]` (RW)
///
///   Set this bit to force power up UART memory.
///
/// - **MEM_FORCE_PD** `[25]` (RW)
///
///   Set this bit to force power down UART memory.
#[repr(transparent)]
pub struct MemConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemConf {
    // ======== Field Constants ========

    /// **MEM_FORCE_PU**
    ///
    /// Set this bit to force power up UART memory.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MEM_FORCE_PU_OFFSET: u32 = 26;
    pub const MEM_FORCE_PU_MASK: u32 = 0x04000000;
    pub const MEM_FORCE_PU: u32 = 1 << 26;

    /// **MEM_FORCE_PD**
    ///
    /// Set this bit to force power down UART memory.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MEM_FORCE_PD_OFFSET: u32 = 25;
    pub const MEM_FORCE_PD_MASK: u32 = 0x02000000;
    pub const MEM_FORCE_PD: u32 = 1 << 25;

    // ======== Field Accessors ========

    /// **MEM_FORCE_PU** `[26]` (RW)
    ///
    /// Set this bit to force power up UART memory.
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

    /// **MEM_FORCE_PD** `[25]` (RW)
    ///
    /// Set this bit to force power down UART memory.
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
}

/// MEM_CONF value type for batch read/modify/write operations
///
/// Use `memconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemConfVal {
    bits: u32,
}

impl MemConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MEM_FORCE_PU** `[26:26]`
    ///
    /// Set this bit to force power up UART memory.
    #[inline(always)]
    pub const fn mem_force_pu(&self) -> bool {
        (self.bits & MemConf::MEM_FORCE_PU_MASK) != 0
    }

    /// Read **MEM_FORCE_PD** `[25:25]`
    ///
    /// Set this bit to force power down UART memory.
    #[inline(always)]
    pub const fn mem_force_pd(&self) -> bool {
        (self.bits & MemConf::MEM_FORCE_PD_MASK) != 0
    }

    /// Set **MEM_FORCE_PU** `[26:26]`
    ///
    /// Set this bit to force power up UART memory.
    #[inline(always)]
    pub const fn with_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemConf::MEM_FORCE_PU_MASK
            } else {
                self.bits & !MemConf::MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PD** `[25:25]`
    ///
    /// Set this bit to force power down UART memory.
    #[inline(always)]
    pub const fn with_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemConf::MEM_FORCE_PD_MASK
            } else {
                self.bits & !MemConf::MEM_FORCE_PD_MASK
            },
        }
    }
}

impl Default for MemConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemConfVal {
        MemConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemConfVal) -> MemConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TOUT_CONF_SYNC Register
///
/// UART threshold and allocation configuration
///
/// Address offset: 0x0064
/// Reset value: 0x00000028
///
/// ## Fields
///
/// - **RX_TOUT_THRHD** `[11:2]` (RW)
///
///   This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
///
/// - **RX_TOUT_FLOW_DIS** `[1]` (RW)
///
///   Set this bit to stop accumulating idle_cnt when hardware flow control works.
///
/// - **RX_TOUT_EN** `[0]` (RW)
///
///   This is the enble bit for uart receiver's timeout function.
#[repr(transparent)]
pub struct ToutConfSync {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ToutConfSync {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ToutConfSync {
    // ======== Field Constants ========

    /// **RX_TOUT_THRHD**
    ///
    /// This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    ///
    /// Bits `[11:2]`, mask `0x00000ffc`
    pub const RX_TOUT_THRHD_OFFSET: u32 = 2;
    pub const RX_TOUT_THRHD_MASK: u32 = 0x00000ffc;

    /// **RX_TOUT_FLOW_DIS**
    ///
    /// Set this bit to stop accumulating idle_cnt when hardware flow control works.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_TOUT_FLOW_DIS_OFFSET: u32 = 1;
    pub const RX_TOUT_FLOW_DIS_MASK: u32 = 0x00000002;
    pub const RX_TOUT_FLOW_DIS: u32 = 1 << 1;

    /// **RX_TOUT_EN**
    ///
    /// This is the enble bit for uart receiver's timeout function.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_TOUT_EN_OFFSET: u32 = 0;
    pub const RX_TOUT_EN_MASK: u32 = 0x00000001;
    pub const RX_TOUT_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_TOUT_THRHD** `[11:2]` (RW)
    ///
    /// This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TOUT_THRHD_MASK) >> Self::RX_TOUT_THRHD_OFFSET
    }

    /// Write **RX_TOUT_THRHD** `[11:2]` (RW)
    ///
    /// This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    pub fn write_rx_tout_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TOUT_THRHD_MASK)
                | ((value << Self::RX_TOUT_THRHD_OFFSET) & Self::RX_TOUT_THRHD_MASK)
        });
    }

    /// **RX_TOUT_FLOW_DIS** `[1]` (RW)
    ///
    /// Set this bit to stop accumulating idle_cnt when hardware flow control works.
    #[inline(always)]
    pub fn rx_tout_flow_dis(&self) -> bool {
        (self.value.read_bits() & Self::RX_TOUT_FLOW_DIS_MASK) != 0
    }

    /// Set RX_TOUT_FLOW_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tout_flow_dis(&self) {
        self.value.modify_bits(|r| r | Self::RX_TOUT_FLOW_DIS_MASK);
    }

    /// Clear RX_TOUT_FLOW_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tout_flow_dis(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TOUT_FLOW_DIS_MASK);
    }

    /// **RX_TOUT_EN** `[0]` (RW)
    ///
    /// This is the enble bit for uart receiver's timeout function.
    #[inline(always)]
    pub fn rx_tout_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TOUT_EN_MASK) != 0
    }

    /// Set RX_TOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_tout_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TOUT_EN_MASK);
    }

    /// Clear RX_TOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_tout_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_TOUT_EN_MASK);
    }
}

/// TOUT_CONF_SYNC value type for batch read/modify/write operations
///
/// Use `toutconfsync.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `toutconfsync.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ToutConfSyncVal {
    bits: u32,
}

impl ToutConfSyncVal {
    /// Create from raw bits
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
        Self { bits: 0x00000028 }
    }

    /// Read **RX_TOUT_THRHD** `[11:2]`
    ///
    /// This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    pub const fn rx_tout_thrhd(&self) -> u32 {
        (self.bits & ToutConfSync::RX_TOUT_THRHD_MASK) >> ToutConfSync::RX_TOUT_THRHD_OFFSET
    }

    /// Read **RX_TOUT_FLOW_DIS** `[1:1]`
    ///
    /// Set this bit to stop accumulating idle_cnt when hardware flow control works.
    #[inline(always)]
    pub const fn rx_tout_flow_dis(&self) -> bool {
        (self.bits & ToutConfSync::RX_TOUT_FLOW_DIS_MASK) != 0
    }

    /// Read **RX_TOUT_EN** `[0:0]`
    ///
    /// This is the enble bit for uart receiver's timeout function.
    #[inline(always)]
    pub const fn rx_tout_en(&self) -> bool {
        (self.bits & ToutConfSync::RX_TOUT_EN_MASK) != 0
    }

    /// Set **RX_TOUT_THRHD** `[11:2]`
    ///
    /// This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    pub const fn with_rx_tout_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ToutConfSync::RX_TOUT_THRHD_MASK)
                | ((value << ToutConfSync::RX_TOUT_THRHD_OFFSET)
                    & ToutConfSync::RX_TOUT_THRHD_MASK),
        }
    }

    /// Set **RX_TOUT_FLOW_DIS** `[1:1]`
    ///
    /// Set this bit to stop accumulating idle_cnt when hardware flow control works.
    #[inline(always)]
    pub const fn with_rx_tout_flow_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ToutConfSync::RX_TOUT_FLOW_DIS_MASK
            } else {
                self.bits & !ToutConfSync::RX_TOUT_FLOW_DIS_MASK
            },
        }
    }

    /// Set **RX_TOUT_EN** `[0:0]`
    ///
    /// This is the enble bit for uart receiver's timeout function.
    #[inline(always)]
    pub const fn with_rx_tout_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ToutConfSync::RX_TOUT_EN_MASK
            } else {
                self.bits & !ToutConfSync::RX_TOUT_EN_MASK
            },
        }
    }
}

impl Default for ToutConfSyncVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ToutConfSync {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ToutConfSyncVal {
        ToutConfSyncVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ToutConfSyncVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ToutConfSyncVal) -> ToutConfSyncVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_TX_STATUS Register
///
/// Tx-SRAM write and read offset address.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **TX_SRAM_RADDR** `[16:12]` (RO)
///
///   This register stores the offset read address in Tx-SRAM.
///
/// - **TX_SRAM_WADDR** `[7:3]` (RO)
///
///   This register stores the offset write address in Tx-SRAM.
#[repr(transparent)]
pub struct MemTxStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemTxStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemTxStatus {
    // ======== Field Constants ========

    /// **TX_SRAM_RADDR**
    ///
    /// This register stores the offset read address in Tx-SRAM.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const TX_SRAM_RADDR_OFFSET: u32 = 12;
    pub const TX_SRAM_RADDR_MASK: u32 = 0x0001f000;

    /// **TX_SRAM_WADDR**
    ///
    /// This register stores the offset write address in Tx-SRAM.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const TX_SRAM_WADDR_OFFSET: u32 = 3;
    pub const TX_SRAM_WADDR_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **TX_SRAM_RADDR** `[16:12]` (RO)
    ///
    /// This register stores the offset read address in Tx-SRAM.
    #[inline(always)]
    pub fn tx_sram_raddr(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SRAM_RADDR_MASK) >> Self::TX_SRAM_RADDR_OFFSET
    }

    /// **TX_SRAM_WADDR** `[7:3]` (RO)
    ///
    /// This register stores the offset write address in Tx-SRAM.
    #[inline(always)]
    pub fn tx_sram_waddr(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SRAM_WADDR_MASK) >> Self::TX_SRAM_WADDR_OFFSET
    }
}

/// MEM_TX_STATUS value type for batch read/modify/write operations
///
/// Use `memtxstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memtxstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemTxStatusVal {
    bits: u32,
}

impl MemTxStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_SRAM_RADDR** `[16:12]`
    ///
    /// This register stores the offset read address in Tx-SRAM.
    #[inline(always)]
    pub const fn tx_sram_raddr(&self) -> u32 {
        (self.bits & MemTxStatus::TX_SRAM_RADDR_MASK) >> MemTxStatus::TX_SRAM_RADDR_OFFSET
    }

    /// Read **TX_SRAM_WADDR** `[7:3]`
    ///
    /// This register stores the offset write address in Tx-SRAM.
    #[inline(always)]
    pub const fn tx_sram_waddr(&self) -> u32 {
        (self.bits & MemTxStatus::TX_SRAM_WADDR_MASK) >> MemTxStatus::TX_SRAM_WADDR_OFFSET
    }
}

impl Default for MemTxStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemTxStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemTxStatusVal {
        MemTxStatusVal::from_bits(self.value.read_bits())
    }
}

/// MEM_RX_STATUS Register
///
/// Rx-SRAM write and read offset address.
///
/// Address offset: 0x006c
/// Reset value: 0x00010080
///
/// ## Fields
///
/// - **RX_SRAM_WADDR** `[16:12]` (RO)
///
///   This register stores the offset write address in Rx-SRAM.
///
/// - **RX_SRAM_RADDR** `[7:3]` (RO)
///
///   This register stores the offset read address in RX-SRAM.
#[repr(transparent)]
pub struct MemRxStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemRxStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemRxStatus {
    // ======== Field Constants ========

    /// **RX_SRAM_WADDR**
    ///
    /// This register stores the offset write address in Rx-SRAM.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const RX_SRAM_WADDR_OFFSET: u32 = 12;
    pub const RX_SRAM_WADDR_MASK: u32 = 0x0001f000;

    /// **RX_SRAM_RADDR**
    ///
    /// This register stores the offset read address in RX-SRAM.
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const RX_SRAM_RADDR_OFFSET: u32 = 3;
    pub const RX_SRAM_RADDR_MASK: u32 = 0x000000f8;

    // ======== Field Accessors ========

    /// **RX_SRAM_WADDR** `[16:12]` (RO)
    ///
    /// This register stores the offset write address in Rx-SRAM.
    #[inline(always)]
    pub fn rx_sram_waddr(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SRAM_WADDR_MASK) >> Self::RX_SRAM_WADDR_OFFSET
    }

    /// **RX_SRAM_RADDR** `[7:3]` (RO)
    ///
    /// This register stores the offset read address in RX-SRAM.
    #[inline(always)]
    pub fn rx_sram_raddr(&self) -> u32 {
        (self.value.read_bits() & Self::RX_SRAM_RADDR_MASK) >> Self::RX_SRAM_RADDR_OFFSET
    }
}

/// MEM_RX_STATUS value type for batch read/modify/write operations
///
/// Use `memrxstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memrxstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemRxStatusVal {
    bits: u32,
}

impl MemRxStatusVal {
    /// Create from raw bits
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
        Self { bits: 0x00010080 }
    }

    /// Read **RX_SRAM_WADDR** `[16:12]`
    ///
    /// This register stores the offset write address in Rx-SRAM.
    #[inline(always)]
    pub const fn rx_sram_waddr(&self) -> u32 {
        (self.bits & MemRxStatus::RX_SRAM_WADDR_MASK) >> MemRxStatus::RX_SRAM_WADDR_OFFSET
    }

    /// Read **RX_SRAM_RADDR** `[7:3]`
    ///
    /// This register stores the offset read address in RX-SRAM.
    #[inline(always)]
    pub const fn rx_sram_raddr(&self) -> u32 {
        (self.bits & MemRxStatus::RX_SRAM_RADDR_MASK) >> MemRxStatus::RX_SRAM_RADDR_OFFSET
    }
}

impl Default for MemRxStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MemRxStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemRxStatusVal {
        MemRxStatusVal::from_bits(self.value.read_bits())
    }
}

/// FSM_STATUS Register
///
/// UART transmit and receive status.
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **ST_UTX_OUT** `[7:4]` (RO)
///
///   This is the status register of transmitter.
///
/// - **ST_URX_OUT** `[3:0]` (RO)
///
///   This is the status register of receiver.
#[repr(transparent)]
pub struct FsmStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FsmStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FsmStatus {
    // ======== Field Constants ========

    /// **ST_UTX_OUT**
    ///
    /// This is the status register of transmitter.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const ST_UTX_OUT_OFFSET: u32 = 4;
    pub const ST_UTX_OUT_MASK: u32 = 0x000000f0;

    /// **ST_URX_OUT**
    ///
    /// This is the status register of receiver.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const ST_URX_OUT_OFFSET: u32 = 0;
    pub const ST_URX_OUT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **ST_UTX_OUT** `[7:4]` (RO)
    ///
    /// This is the status register of transmitter.
    #[inline(always)]
    pub fn st_utx_out(&self) -> u32 {
        (self.value.read_bits() & Self::ST_UTX_OUT_MASK) >> Self::ST_UTX_OUT_OFFSET
    }

    /// **ST_URX_OUT** `[3:0]` (RO)
    ///
    /// This is the status register of receiver.
    #[inline(always)]
    pub fn st_urx_out(&self) -> u32 {
        (self.value.read_bits() & Self::ST_URX_OUT_MASK) >> Self::ST_URX_OUT_OFFSET
    }
}

/// FSM_STATUS value type for batch read/modify/write operations
///
/// Use `fsmstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fsmstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FsmStatusVal {
    bits: u32,
}

impl FsmStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ST_UTX_OUT** `[7:4]`
    ///
    /// This is the status register of transmitter.
    #[inline(always)]
    pub const fn st_utx_out(&self) -> u32 {
        (self.bits & FsmStatus::ST_UTX_OUT_MASK) >> FsmStatus::ST_UTX_OUT_OFFSET
    }

    /// Read **ST_URX_OUT** `[3:0]`
    ///
    /// This is the status register of receiver.
    #[inline(always)]
    pub const fn st_urx_out(&self) -> u32 {
        (self.bits & FsmStatus::ST_URX_OUT_MASK) >> FsmStatus::ST_URX_OUT_OFFSET
    }
}

impl Default for FsmStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FsmStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FsmStatusVal {
        FsmStatusVal::from_bits(self.value.read_bits())
    }
}

/// CLK_CONF Register
///
/// UART core clock configuration
///
/// Address offset: 0x0088
/// Reset value: 0x03000000
///
/// ## Fields
///
/// - **RX_RST_CORE** `[27]` (RW)
///
///   Write 1 then write 0 to this bit to reset UART Rx.
///
/// - **TX_RST_CORE** `[26]` (RW)
///
///   Write 1 then write 0 to this bit to reset UART Tx.
///
/// - **RX_SCLK_EN** `[25]` (RW)
///
///   Set this bit to enable UART Rx clock.
///
/// - **TX_SCLK_EN** `[24]` (RW)
///
///   Set this bit to enable UART Tx clock.
#[repr(transparent)]
pub struct ClkConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkConf {
    // ======== Field Constants ========

    /// **RX_RST_CORE**
    ///
    /// Write 1 then write 0 to this bit to reset UART Rx.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const RX_RST_CORE_OFFSET: u32 = 27;
    pub const RX_RST_CORE_MASK: u32 = 0x08000000;
    pub const RX_RST_CORE: u32 = 1 << 27;

    /// **TX_RST_CORE**
    ///
    /// Write 1 then write 0 to this bit to reset UART Tx.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TX_RST_CORE_OFFSET: u32 = 26;
    pub const TX_RST_CORE_MASK: u32 = 0x04000000;
    pub const TX_RST_CORE: u32 = 1 << 26;

    /// **RX_SCLK_EN**
    ///
    /// Set this bit to enable UART Rx clock.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const RX_SCLK_EN_OFFSET: u32 = 25;
    pub const RX_SCLK_EN_MASK: u32 = 0x02000000;
    pub const RX_SCLK_EN: u32 = 1 << 25;

    /// **TX_SCLK_EN**
    ///
    /// Set this bit to enable UART Tx clock.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TX_SCLK_EN_OFFSET: u32 = 24;
    pub const TX_SCLK_EN_MASK: u32 = 0x01000000;
    pub const TX_SCLK_EN: u32 = 1 << 24;

    // ======== Field Accessors ========

    /// **RX_RST_CORE** `[27]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset UART Rx.
    #[inline(always)]
    pub fn rx_rst_core(&self) -> bool {
        (self.value.read_bits() & Self::RX_RST_CORE_MASK) != 0
    }

    /// Set RX_RST_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_rst_core(&self) {
        self.value.modify_bits(|r| r | Self::RX_RST_CORE_MASK);
    }

    /// Clear RX_RST_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_rst_core(&self) {
        self.value.modify_bits(|r| r & !Self::RX_RST_CORE_MASK);
    }

    /// **TX_RST_CORE** `[26]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset UART Tx.
    #[inline(always)]
    pub fn tx_rst_core(&self) -> bool {
        (self.value.read_bits() & Self::TX_RST_CORE_MASK) != 0
    }

    /// Set TX_RST_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_rst_core(&self) {
        self.value.modify_bits(|r| r | Self::TX_RST_CORE_MASK);
    }

    /// Clear TX_RST_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_rst_core(&self) {
        self.value.modify_bits(|r| r & !Self::TX_RST_CORE_MASK);
    }

    /// **RX_SCLK_EN** `[25]` (RW)
    ///
    /// Set this bit to enable UART Rx clock.
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_SCLK_EN_MASK) != 0
    }

    /// Set RX_SCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_sclk_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_SCLK_EN_MASK);
    }

    /// Clear RX_SCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_sclk_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_SCLK_EN_MASK);
    }

    /// **TX_SCLK_EN** `[24]` (RW)
    ///
    /// Set this bit to enable UART Tx clock.
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_SCLK_EN_MASK) != 0
    }

    /// Set TX_SCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_sclk_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_SCLK_EN_MASK);
    }

    /// Clear TX_SCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_sclk_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_SCLK_EN_MASK);
    }
}

/// CLK_CONF value type for batch read/modify/write operations
///
/// Use `clkconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkConfVal {
    bits: u32,
}

impl ClkConfVal {
    /// Create from raw bits
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
        Self { bits: 0x03000000 }
    }

    /// Read **RX_RST_CORE** `[27:27]`
    ///
    /// Write 1 then write 0 to this bit to reset UART Rx.
    #[inline(always)]
    pub const fn rx_rst_core(&self) -> bool {
        (self.bits & ClkConf::RX_RST_CORE_MASK) != 0
    }

    /// Read **TX_RST_CORE** `[26:26]`
    ///
    /// Write 1 then write 0 to this bit to reset UART Tx.
    #[inline(always)]
    pub const fn tx_rst_core(&self) -> bool {
        (self.bits & ClkConf::TX_RST_CORE_MASK) != 0
    }

    /// Read **RX_SCLK_EN** `[25:25]`
    ///
    /// Set this bit to enable UART Rx clock.
    #[inline(always)]
    pub const fn rx_sclk_en(&self) -> bool {
        (self.bits & ClkConf::RX_SCLK_EN_MASK) != 0
    }

    /// Read **TX_SCLK_EN** `[24:24]`
    ///
    /// Set this bit to enable UART Tx clock.
    #[inline(always)]
    pub const fn tx_sclk_en(&self) -> bool {
        (self.bits & ClkConf::TX_SCLK_EN_MASK) != 0
    }

    /// Set **RX_RST_CORE** `[27:27]`
    ///
    /// Write 1 then write 0 to this bit to reset UART Rx.
    #[inline(always)]
    pub const fn with_rx_rst_core(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::RX_RST_CORE_MASK
            } else {
                self.bits & !ClkConf::RX_RST_CORE_MASK
            },
        }
    }

    /// Set **TX_RST_CORE** `[26:26]`
    ///
    /// Write 1 then write 0 to this bit to reset UART Tx.
    #[inline(always)]
    pub const fn with_tx_rst_core(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::TX_RST_CORE_MASK
            } else {
                self.bits & !ClkConf::TX_RST_CORE_MASK
            },
        }
    }

    /// Set **RX_SCLK_EN** `[25:25]`
    ///
    /// Set this bit to enable UART Rx clock.
    #[inline(always)]
    pub const fn with_rx_sclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::RX_SCLK_EN_MASK
            } else {
                self.bits & !ClkConf::RX_SCLK_EN_MASK
            },
        }
    }

    /// Set **TX_SCLK_EN** `[24:24]`
    ///
    /// Set this bit to enable UART Tx clock.
    #[inline(always)]
    pub const fn with_tx_sclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::TX_SCLK_EN_MASK
            } else {
                self.bits & !ClkConf::TX_SCLK_EN_MASK
            },
        }
    }
}

impl Default for ClkConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkConfVal {
        ClkConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkConfVal) -> ClkConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// UART Version register
///
/// Address offset: 0x008c
/// Reset value: 0x02305050
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
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
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// This is the version register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
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
        Self { bits: 0x02305050 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// This is the version register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
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

/// AFIFO_STATUS Register
///
/// UART AFIFO Status
///
/// Address offset: 0x0090
/// Reset value: 0x0000000a
///
/// ## Fields
///
/// - **RX_AFIFO_EMPTY** `[3]` (RO)
///
///   Empty signal of APB RX AFIFO.
///
/// - **RX_AFIFO_FULL** `[2]` (RO)
///
///   Full signal of APB RX AFIFO.
///
/// - **TX_AFIFO_EMPTY** `[1]` (RO)
///
///   Empty signal of APB TX AFIFO.
///
/// - **TX_AFIFO_FULL** `[0]` (RO)
///
///   Full signal of APB TX AFIFO.
#[repr(transparent)]
pub struct AfifoStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AfifoStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AfifoStatus {
    // ======== Field Constants ========

    /// **RX_AFIFO_EMPTY**
    ///
    /// Empty signal of APB RX AFIFO.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_AFIFO_EMPTY_OFFSET: u32 = 3;
    pub const RX_AFIFO_EMPTY_MASK: u32 = 0x00000008;
    pub const RX_AFIFO_EMPTY: u32 = 1 << 3;

    /// **RX_AFIFO_FULL**
    ///
    /// Full signal of APB RX AFIFO.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_AFIFO_FULL_OFFSET: u32 = 2;
    pub const RX_AFIFO_FULL_MASK: u32 = 0x00000004;
    pub const RX_AFIFO_FULL: u32 = 1 << 2;

    /// **TX_AFIFO_EMPTY**
    ///
    /// Empty signal of APB TX AFIFO.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_AFIFO_EMPTY_OFFSET: u32 = 1;
    pub const TX_AFIFO_EMPTY_MASK: u32 = 0x00000002;
    pub const TX_AFIFO_EMPTY: u32 = 1 << 1;

    /// **TX_AFIFO_FULL**
    ///
    /// Full signal of APB TX AFIFO.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_AFIFO_FULL_OFFSET: u32 = 0;
    pub const TX_AFIFO_FULL_MASK: u32 = 0x00000001;
    pub const TX_AFIFO_FULL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_AFIFO_EMPTY** `[3]` (RO)
    ///
    /// Empty signal of APB RX AFIFO.
    #[inline(always)]
    pub fn rx_afifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::RX_AFIFO_EMPTY_MASK) != 0
    }

    /// **RX_AFIFO_FULL** `[2]` (RO)
    ///
    /// Full signal of APB RX AFIFO.
    #[inline(always)]
    pub fn rx_afifo_full(&self) -> bool {
        (self.value.read_bits() & Self::RX_AFIFO_FULL_MASK) != 0
    }

    /// **TX_AFIFO_EMPTY** `[1]` (RO)
    ///
    /// Empty signal of APB TX AFIFO.
    #[inline(always)]
    pub fn tx_afifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::TX_AFIFO_EMPTY_MASK) != 0
    }

    /// **TX_AFIFO_FULL** `[0]` (RO)
    ///
    /// Full signal of APB TX AFIFO.
    #[inline(always)]
    pub fn tx_afifo_full(&self) -> bool {
        (self.value.read_bits() & Self::TX_AFIFO_FULL_MASK) != 0
    }
}

/// AFIFO_STATUS value type for batch read/modify/write operations
///
/// Use `afifostatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `afifostatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AfifoStatusVal {
    bits: u32,
}

impl AfifoStatusVal {
    /// Create from raw bits
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

    /// Read **RX_AFIFO_EMPTY** `[3:3]`
    ///
    /// Empty signal of APB RX AFIFO.
    #[inline(always)]
    pub const fn rx_afifo_empty(&self) -> bool {
        (self.bits & AfifoStatus::RX_AFIFO_EMPTY_MASK) != 0
    }

    /// Read **RX_AFIFO_FULL** `[2:2]`
    ///
    /// Full signal of APB RX AFIFO.
    #[inline(always)]
    pub const fn rx_afifo_full(&self) -> bool {
        (self.bits & AfifoStatus::RX_AFIFO_FULL_MASK) != 0
    }

    /// Read **TX_AFIFO_EMPTY** `[1:1]`
    ///
    /// Empty signal of APB TX AFIFO.
    #[inline(always)]
    pub const fn tx_afifo_empty(&self) -> bool {
        (self.bits & AfifoStatus::TX_AFIFO_EMPTY_MASK) != 0
    }

    /// Read **TX_AFIFO_FULL** `[0:0]`
    ///
    /// Full signal of APB TX AFIFO.
    #[inline(always)]
    pub const fn tx_afifo_full(&self) -> bool {
        (self.bits & AfifoStatus::TX_AFIFO_FULL_MASK) != 0
    }
}

impl Default for AfifoStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AfifoStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AfifoStatusVal {
        AfifoStatusVal::from_bits(self.value.read_bits())
    }
}

/// REG_UPDATE Register
///
/// UART Registers Configuration Update register
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **REG_UPDATE** `[0]` (RW)
///
///   Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done.
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

    /// **REG_UPDATE**
    ///
    /// Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_UPDATE_OFFSET: u32 = 0;
    pub const REG_UPDATE_MASK: u32 = 0x00000001;
    pub const REG_UPDATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_UPDATE** `[0]` (RW)
    ///
    /// Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done.
    #[inline(always)]
    pub fn reg_update(&self) -> bool {
        (self.value.read_bits() & Self::REG_UPDATE_MASK) != 0
    }

    /// Set REG_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_update(&self) {
        self.value.modify_bits(|r| r | Self::REG_UPDATE_MASK);
    }

    /// Clear REG_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_update(&self) {
        self.value.modify_bits(|r| r & !Self::REG_UPDATE_MASK);
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

    /// Read **REG_UPDATE** `[0:0]`
    ///
    /// Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done.
    #[inline(always)]
    pub const fn reg_update(&self) -> bool {
        (self.bits & RegUpdate::REG_UPDATE_MASK) != 0
    }

    /// Set **REG_UPDATE** `[0:0]`
    ///
    /// Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done.
    #[inline(always)]
    pub const fn with_reg_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegUpdate::REG_UPDATE_MASK
            } else {
                self.bits & !RegUpdate::REG_UPDATE_MASK
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
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegUpdateVal {
        RegUpdateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegUpdateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegUpdateVal) -> RegUpdateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ID Register
///
/// UART ID register
///
/// Address offset: 0x009c
/// Reset value: 0x00000500
///
/// ## Fields
///
/// - **ID** `[31:0]` (RW)
///
///   This register is used to configure the uart_id.
#[repr(transparent)]
pub struct Id {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Id {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Id {
    // ======== Field Constants ========

    /// **ID**
    ///
    /// This register is used to configure the uart_id.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ID_OFFSET: u32 = 0;
    pub const ID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ID** `[31:0]` (RW)
    ///
    /// This register is used to configure the uart_id.
    #[inline(always)]
    pub fn id(&self) -> u32 {
        (self.value.read_bits() & Self::ID_MASK) >> Self::ID_OFFSET
    }

    /// Write **ID** `[31:0]` (RW)
    ///
    /// This register is used to configure the uart_id.
    #[inline(always)]
    pub fn write_id(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::ID_MASK) | ((value << Self::ID_OFFSET) & Self::ID_MASK));
    }
}

/// ID value type for batch read/modify/write operations
///
/// Use `id.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `id.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdVal {
    bits: u32,
}

impl IdVal {
    /// Create from raw bits
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
        Self { bits: 0x00000500 }
    }

    /// Read **ID** `[31:0]`
    ///
    /// This register is used to configure the uart_id.
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        (self.bits & Id::ID_MASK) >> Id::ID_OFFSET
    }

    /// Set **ID** `[31:0]`
    ///
    /// This register is used to configure the uart_id.
    #[inline(always)]
    pub const fn with_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Id::ID_MASK) | ((value << Id::ID_OFFSET) & Id::ID_MASK),
        }
    }
}

impl Default for IdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Id {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdVal {
        IdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdVal) -> IdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// FIFO data register
    pub fifo: Fifo,
    /// 0x0004
    /// Raw interrupt status
    pub int_raw: IntRaw,
    /// 0x0008
    /// Masked interrupt status
    pub int_st: IntSt,
    /// 0x000c
    /// Interrupt enable bits
    pub int_ena: IntEna,
    /// 0x0010
    /// Interrupt clear bits
    pub int_clr: IntClr,
    /// 0x0014
    /// Clock divider configuration
    pub clkdiv_sync: ClkdivSync,
    /// 0x0018
    /// Rx Filter configuration
    pub rx_filt: RxFilt,
    /// 0x001c
    /// UART status register
    pub status: Status,
    /// 0x0020
    /// Configuration register 0
    pub conf0_sync: Conf0Sync,
    /// 0x0024
    /// Configuration register 1
    pub conf1: Conf1,
    _reserved0: [u8; 4],
    /// 0x002c
    /// Hardware flow-control configuration
    pub hwfc_conf_sync: HwfcConfSync,
    /// 0x0030
    /// UART sleep configure register 0
    pub sleep_conf0: SleepConf0,
    /// 0x0034
    /// UART sleep configure register 1
    pub sleep_conf1: SleepConf1,
    /// 0x0038
    /// UART sleep configure register 2
    pub sleep_conf2: SleepConf2,
    /// 0x003c
    /// Software flow-control character configuration
    pub swfc_conf0_sync: SwfcConf0Sync,
    /// 0x0040
    /// Software flow-control character configuration
    pub swfc_conf1: SwfcConf1,
    /// 0x0044
    /// Tx Break character configuration
    pub txbrk_conf_sync: TxbrkConfSync,
    /// 0x0048
    /// Frame-end idle configuration
    pub idle_conf_sync: IdleConfSync,
    /// 0x004c
    /// RS485 mode configuration
    pub rs485_conf_sync: Rs485ConfSync,
    /// 0x0050
    /// Pre-sequence timing configuration
    pub at_cmd_precnt_sync: AtCmdPrecntSync,
    /// 0x0054
    /// Post-sequence timing configuration
    pub at_cmd_postcnt_sync: AtCmdPostcntSync,
    /// 0x0058
    /// Timeout configuration
    pub at_cmd_gaptout_sync: AtCmdGaptoutSync,
    /// 0x005c
    /// AT escape sequence detection configuration
    pub at_cmd_char_sync: AtCmdCharSync,
    /// 0x0060
    /// UART memory power configuration
    pub mem_conf: MemConf,
    /// 0x0064
    /// UART threshold and allocation configuration
    pub tout_conf_sync: ToutConfSync,
    /// 0x0068
    /// Tx-SRAM write and read offset address.
    pub mem_tx_status: MemTxStatus,
    /// 0x006c
    /// Rx-SRAM write and read offset address.
    pub mem_rx_status: MemRxStatus,
    /// 0x0070
    /// UART transmit and receive status.
    pub fsm_status: FsmStatus,
    _reserved1: [u8; 20],
    /// 0x0088
    /// UART core clock configuration
    pub clk_conf: ClkConf,
    /// 0x008c
    /// UART Version register
    pub date: Date,
    /// 0x0090
    /// UART AFIFO Status
    pub afifo_status: AfifoStatus,
    _reserved2: [u8; 4],
    /// 0x0098
    /// UART Registers Configuration Update register
    pub reg_update: RegUpdate,
    /// 0x009c
    /// UART ID register
    pub id: Id,
}
