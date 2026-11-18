//! UHCI0
//! Universal Host Controller Interface 0

#![allow(dead_code)]

pub const BASE: u32 = 0x500df000;

/// CONF0 Register
///
/// UHCI Configuration Register0
///
/// Address offset: 0x0000
/// Reset value: 0x000006e0
///
/// ## Fields
///
/// - **UART_RX_BRK_EOF_EN** `[12]` (RW)
///
///   If this bit is set to 1 UHCI will end payload receive process when NULL frame is received by UART.
///
/// - **CLK_EN** `[11]` (RW)
///
///   1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers.
///
/// - **ENCODE_CRC_EN** `[10]` (RW)
///
///   Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload.
///
/// - **LEN_EOF_EN** `[9]` (RW)
///
///   If this bit is set to 1 UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0 UHCI decoder receiving payload data is end when 0xc0 is received.
///
/// - **UART_IDLE_EOF_EN** `[8]` (RW)
///
///   If this bit is set to 1 UHCI will end the payload receiving process when UART has been in idle state.
///
/// - **CRC_REC_EN** `[7]` (RW)
///
///   Set this bit to enable UHCI to receive the 16 bit CRC.
///
/// - **HEAD_EN** `[6]` (RW)
///
///   Set this bit to encode the data packet with a formatting header.
///
/// - **SEPER_EN** `[5]` (RW)
///
///   Set this bit to separate the data frame using a special char.
///
/// - **UART_SEL** `[4:2]` (RW)
///
///   Select which uart to connect with GDMA.
///
/// - **RX_RST** `[1]` (RW)
///
///   Write 1 then write 0 to this bit to reset encode state machine.
///
/// - **TX_RST** `[0]` (RW)
///
///   Write 1 then write 0 to this bit to reset decode state machine.
#[repr(transparent)]
pub struct Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf0 {
    // ======== Field Constants ========

    /// **UART_RX_BRK_EOF_EN**
    ///
    /// If this bit is set to 1 UHCI will end payload receive process when NULL frame is received by UART.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const UART_RX_BRK_EOF_EN_OFFSET: u32 = 12;
    pub const UART_RX_BRK_EOF_EN_MASK: u32 = 0x00001000;
    pub const UART_RX_BRK_EOF_EN: u32 = 1 << 12;

    /// **CLK_EN**
    ///
    /// 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_EN_OFFSET: u32 = 11;
    pub const CLK_EN_MASK: u32 = 0x00000800;
    pub const CLK_EN: u32 = 1 << 11;

    /// **ENCODE_CRC_EN**
    ///
    /// Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ENCODE_CRC_EN_OFFSET: u32 = 10;
    pub const ENCODE_CRC_EN_MASK: u32 = 0x00000400;
    pub const ENCODE_CRC_EN: u32 = 1 << 10;

    /// **LEN_EOF_EN**
    ///
    /// If this bit is set to 1 UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0 UHCI decoder receiving payload data is end when 0xc0 is received.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LEN_EOF_EN_OFFSET: u32 = 9;
    pub const LEN_EOF_EN_MASK: u32 = 0x00000200;
    pub const LEN_EOF_EN: u32 = 1 << 9;

    /// **UART_IDLE_EOF_EN**
    ///
    /// If this bit is set to 1 UHCI will end the payload receiving process when UART has been in idle state.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const UART_IDLE_EOF_EN_OFFSET: u32 = 8;
    pub const UART_IDLE_EOF_EN_MASK: u32 = 0x00000100;
    pub const UART_IDLE_EOF_EN: u32 = 1 << 8;

    /// **CRC_REC_EN**
    ///
    /// Set this bit to enable UHCI to receive the 16 bit CRC.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CRC_REC_EN_OFFSET: u32 = 7;
    pub const CRC_REC_EN_MASK: u32 = 0x00000080;
    pub const CRC_REC_EN: u32 = 1 << 7;

    /// **HEAD_EN**
    ///
    /// Set this bit to encode the data packet with a formatting header.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const HEAD_EN_OFFSET: u32 = 6;
    pub const HEAD_EN_MASK: u32 = 0x00000040;
    pub const HEAD_EN: u32 = 1 << 6;

    /// **SEPER_EN**
    ///
    /// Set this bit to separate the data frame using a special char.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SEPER_EN_OFFSET: u32 = 5;
    pub const SEPER_EN_MASK: u32 = 0x00000020;
    pub const SEPER_EN: u32 = 1 << 5;

    /// **UART_SEL**
    ///
    /// Select which uart to connect with GDMA.
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const UART_SEL_OFFSET: u32 = 2;
    pub const UART_SEL_MASK: u32 = 0x0000001c;

    /// **RX_RST**
    ///
    /// Write 1 then write 0 to this bit to reset encode state machine.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_RST_OFFSET: u32 = 1;
    pub const RX_RST_MASK: u32 = 0x00000002;
    pub const RX_RST: u32 = 1 << 1;

    /// **TX_RST**
    ///
    /// Write 1 then write 0 to this bit to reset decode state machine.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_RST_OFFSET: u32 = 0;
    pub const TX_RST_MASK: u32 = 0x00000001;
    pub const TX_RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UART_RX_BRK_EOF_EN** `[12]` (RW)
    ///
    /// If this bit is set to 1 UHCI will end payload receive process when NULL frame is received by UART.
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> bool {
        (self.value.read_bits() & Self::UART_RX_BRK_EOF_EN_MASK) != 0
    }

    /// Set UART_RX_BRK_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uart_rx_brk_eof_en(&self) {
        self.value
            .modify_bits(|r| r | Self::UART_RX_BRK_EOF_EN_MASK);
    }

    /// Clear UART_RX_BRK_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uart_rx_brk_eof_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::UART_RX_BRK_EOF_EN_MASK);
    }

    /// **CLK_EN** `[11]` (RW)
    ///
    /// 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers.
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

    /// **ENCODE_CRC_EN** `[10]` (RW)
    ///
    /// Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload.
    #[inline(always)]
    pub fn encode_crc_en(&self) -> bool {
        (self.value.read_bits() & Self::ENCODE_CRC_EN_MASK) != 0
    }

    /// Set ENCODE_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_encode_crc_en(&self) {
        self.value.modify_bits(|r| r | Self::ENCODE_CRC_EN_MASK);
    }

    /// Clear ENCODE_CRC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_encode_crc_en(&self) {
        self.value.modify_bits(|r| r & !Self::ENCODE_CRC_EN_MASK);
    }

    /// **LEN_EOF_EN** `[9]` (RW)
    ///
    /// If this bit is set to 1 UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0 UHCI decoder receiving payload data is end when 0xc0 is received.
    #[inline(always)]
    pub fn len_eof_en(&self) -> bool {
        (self.value.read_bits() & Self::LEN_EOF_EN_MASK) != 0
    }

    /// Set LEN_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_len_eof_en(&self) {
        self.value.modify_bits(|r| r | Self::LEN_EOF_EN_MASK);
    }

    /// Clear LEN_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_len_eof_en(&self) {
        self.value.modify_bits(|r| r & !Self::LEN_EOF_EN_MASK);
    }

    /// **UART_IDLE_EOF_EN** `[8]` (RW)
    ///
    /// If this bit is set to 1 UHCI will end the payload receiving process when UART has been in idle state.
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> bool {
        (self.value.read_bits() & Self::UART_IDLE_EOF_EN_MASK) != 0
    }

    /// Set UART_IDLE_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uart_idle_eof_en(&self) {
        self.value.modify_bits(|r| r | Self::UART_IDLE_EOF_EN_MASK);
    }

    /// Clear UART_IDLE_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uart_idle_eof_en(&self) {
        self.value.modify_bits(|r| r & !Self::UART_IDLE_EOF_EN_MASK);
    }

    /// **CRC_REC_EN** `[7]` (RW)
    ///
    /// Set this bit to enable UHCI to receive the 16 bit CRC.
    #[inline(always)]
    pub fn crc_rec_en(&self) -> bool {
        (self.value.read_bits() & Self::CRC_REC_EN_MASK) != 0
    }

    /// Set CRC_REC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crc_rec_en(&self) {
        self.value.modify_bits(|r| r | Self::CRC_REC_EN_MASK);
    }

    /// Clear CRC_REC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crc_rec_en(&self) {
        self.value.modify_bits(|r| r & !Self::CRC_REC_EN_MASK);
    }

    /// **HEAD_EN** `[6]` (RW)
    ///
    /// Set this bit to encode the data packet with a formatting header.
    #[inline(always)]
    pub fn head_en(&self) -> bool {
        (self.value.read_bits() & Self::HEAD_EN_MASK) != 0
    }

    /// Set HEAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_head_en(&self) {
        self.value.modify_bits(|r| r | Self::HEAD_EN_MASK);
    }

    /// Clear HEAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_head_en(&self) {
        self.value.modify_bits(|r| r & !Self::HEAD_EN_MASK);
    }

    /// **SEPER_EN** `[5]` (RW)
    ///
    /// Set this bit to separate the data frame using a special char.
    #[inline(always)]
    pub fn seper_en(&self) -> bool {
        (self.value.read_bits() & Self::SEPER_EN_MASK) != 0
    }

    /// Set SEPER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_seper_en(&self) {
        self.value.modify_bits(|r| r | Self::SEPER_EN_MASK);
    }

    /// Clear SEPER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_seper_en(&self) {
        self.value.modify_bits(|r| r & !Self::SEPER_EN_MASK);
    }

    /// **UART_SEL** `[4:2]` (RW)
    ///
    /// Select which uart to connect with GDMA.
    #[inline(always)]
    pub fn uart_sel(&self) -> u32 {
        (self.value.read_bits() & Self::UART_SEL_MASK) >> Self::UART_SEL_OFFSET
    }

    /// Write **UART_SEL** `[4:2]` (RW)
    ///
    /// Select which uart to connect with GDMA.
    #[inline(always)]
    pub fn write_uart_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::UART_SEL_MASK) | ((value << Self::UART_SEL_OFFSET) & Self::UART_SEL_MASK)
        });
    }

    /// **RX_RST** `[1]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset encode state machine.
    #[inline(always)]
    pub fn rx_rst(&self) -> bool {
        (self.value.read_bits() & Self::RX_RST_MASK) != 0
    }

    /// Set RX_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_rst(&self) {
        self.value.modify_bits(|r| r | Self::RX_RST_MASK);
    }

    /// Clear RX_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_rst(&self) {
        self.value.modify_bits(|r| r & !Self::RX_RST_MASK);
    }

    /// **TX_RST** `[0]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset decode state machine.
    #[inline(always)]
    pub fn tx_rst(&self) -> bool {
        (self.value.read_bits() & Self::TX_RST_MASK) != 0
    }

    /// Set TX_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_rst(&self) {
        self.value.modify_bits(|r| r | Self::TX_RST_MASK);
    }

    /// Clear TX_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TX_RST_MASK);
    }
}

/// CONF0 value type for batch read/modify/write operations
///
/// Use `conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Conf0Val {
    bits: u32,
}

impl Conf0Val {
    /// Create from raw bits
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
        Self { bits: 0x000006e0 }
    }

    /// Read **UART_RX_BRK_EOF_EN** `[12:12]`
    ///
    /// If this bit is set to 1 UHCI will end payload receive process when NULL frame is received by UART.
    #[inline(always)]
    pub const fn uart_rx_brk_eof_en(&self) -> bool {
        (self.bits & Conf0::UART_RX_BRK_EOF_EN_MASK) != 0
    }

    /// Read **CLK_EN** `[11:11]`
    ///
    /// 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Conf0::CLK_EN_MASK) != 0
    }

    /// Read **ENCODE_CRC_EN** `[10:10]`
    ///
    /// Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload.
    #[inline(always)]
    pub const fn encode_crc_en(&self) -> bool {
        (self.bits & Conf0::ENCODE_CRC_EN_MASK) != 0
    }

    /// Read **LEN_EOF_EN** `[9:9]`
    ///
    /// If this bit is set to 1 UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0 UHCI decoder receiving payload data is end when 0xc0 is received.
    #[inline(always)]
    pub const fn len_eof_en(&self) -> bool {
        (self.bits & Conf0::LEN_EOF_EN_MASK) != 0
    }

    /// Read **UART_IDLE_EOF_EN** `[8:8]`
    ///
    /// If this bit is set to 1 UHCI will end the payload receiving process when UART has been in idle state.
    #[inline(always)]
    pub const fn uart_idle_eof_en(&self) -> bool {
        (self.bits & Conf0::UART_IDLE_EOF_EN_MASK) != 0
    }

    /// Read **CRC_REC_EN** `[7:7]`
    ///
    /// Set this bit to enable UHCI to receive the 16 bit CRC.
    #[inline(always)]
    pub const fn crc_rec_en(&self) -> bool {
        (self.bits & Conf0::CRC_REC_EN_MASK) != 0
    }

    /// Read **HEAD_EN** `[6:6]`
    ///
    /// Set this bit to encode the data packet with a formatting header.
    #[inline(always)]
    pub const fn head_en(&self) -> bool {
        (self.bits & Conf0::HEAD_EN_MASK) != 0
    }

    /// Read **SEPER_EN** `[5:5]`
    ///
    /// Set this bit to separate the data frame using a special char.
    #[inline(always)]
    pub const fn seper_en(&self) -> bool {
        (self.bits & Conf0::SEPER_EN_MASK) != 0
    }

    /// Read **UART_SEL** `[4:2]`
    ///
    /// Select which uart to connect with GDMA.
    #[inline(always)]
    pub const fn uart_sel(&self) -> u32 {
        (self.bits & Conf0::UART_SEL_MASK) >> Conf0::UART_SEL_OFFSET
    }

    /// Read **RX_RST** `[1:1]`
    ///
    /// Write 1 then write 0 to this bit to reset encode state machine.
    #[inline(always)]
    pub const fn rx_rst(&self) -> bool {
        (self.bits & Conf0::RX_RST_MASK) != 0
    }

    /// Read **TX_RST** `[0:0]`
    ///
    /// Write 1 then write 0 to this bit to reset decode state machine.
    #[inline(always)]
    pub const fn tx_rst(&self) -> bool {
        (self.bits & Conf0::TX_RST_MASK) != 0
    }

    /// Set **UART_RX_BRK_EOF_EN** `[12:12]`
    ///
    /// If this bit is set to 1 UHCI will end payload receive process when NULL frame is received by UART.
    #[inline(always)]
    pub const fn with_uart_rx_brk_eof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::UART_RX_BRK_EOF_EN_MASK
            } else {
                self.bits & !Conf0::UART_RX_BRK_EOF_EN_MASK
            },
        }
    }

    /// Set **CLK_EN** `[11:11]`
    ///
    /// 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::CLK_EN_MASK
            } else {
                self.bits & !Conf0::CLK_EN_MASK
            },
        }
    }

    /// Set **ENCODE_CRC_EN** `[10:10]`
    ///
    /// Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload.
    #[inline(always)]
    pub const fn with_encode_crc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::ENCODE_CRC_EN_MASK
            } else {
                self.bits & !Conf0::ENCODE_CRC_EN_MASK
            },
        }
    }

    /// Set **LEN_EOF_EN** `[9:9]`
    ///
    /// If this bit is set to 1 UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0 UHCI decoder receiving payload data is end when 0xc0 is received.
    #[inline(always)]
    pub const fn with_len_eof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::LEN_EOF_EN_MASK
            } else {
                self.bits & !Conf0::LEN_EOF_EN_MASK
            },
        }
    }

    /// Set **UART_IDLE_EOF_EN** `[8:8]`
    ///
    /// If this bit is set to 1 UHCI will end the payload receiving process when UART has been in idle state.
    #[inline(always)]
    pub const fn with_uart_idle_eof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::UART_IDLE_EOF_EN_MASK
            } else {
                self.bits & !Conf0::UART_IDLE_EOF_EN_MASK
            },
        }
    }

    /// Set **CRC_REC_EN** `[7:7]`
    ///
    /// Set this bit to enable UHCI to receive the 16 bit CRC.
    #[inline(always)]
    pub const fn with_crc_rec_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::CRC_REC_EN_MASK
            } else {
                self.bits & !Conf0::CRC_REC_EN_MASK
            },
        }
    }

    /// Set **HEAD_EN** `[6:6]`
    ///
    /// Set this bit to encode the data packet with a formatting header.
    #[inline(always)]
    pub const fn with_head_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::HEAD_EN_MASK
            } else {
                self.bits & !Conf0::HEAD_EN_MASK
            },
        }
    }

    /// Set **SEPER_EN** `[5:5]`
    ///
    /// Set this bit to separate the data frame using a special char.
    #[inline(always)]
    pub const fn with_seper_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::SEPER_EN_MASK
            } else {
                self.bits & !Conf0::SEPER_EN_MASK
            },
        }
    }

    /// Set **UART_SEL** `[4:2]`
    ///
    /// Select which uart to connect with GDMA.
    #[inline(always)]
    pub const fn with_uart_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf0::UART_SEL_MASK)
                | ((value << Conf0::UART_SEL_OFFSET) & Conf0::UART_SEL_MASK),
        }
    }

    /// Set **RX_RST** `[1:1]`
    ///
    /// Write 1 then write 0 to this bit to reset encode state machine.
    #[inline(always)]
    pub const fn with_rx_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::RX_RST_MASK
            } else {
                self.bits & !Conf0::RX_RST_MASK
            },
        }
    }

    /// Set **TX_RST** `[0:0]`
    ///
    /// Write 1 then write 0 to this bit to reset decode state machine.
    #[inline(always)]
    pub const fn with_tx_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::TX_RST_MASK
            } else {
                self.bits & !Conf0::TX_RST_MASK
            },
        }
    }
}

impl Default for Conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Conf0Val {
        Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Conf0Val) -> Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// UHCI Interrupt Raw Register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **APP_CTRL1_INT_RAW** `[8]` (RW)
///
///   Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1.
///
/// - **APP_CTRL0_INT_RAW** `[7]` (RW)
///
///   Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1.
///
/// - **OUT_EOF_INT_RAW** `[6]` (RW)
///
///   Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF.
///
/// - **SEND_A_REG_Q_INT_RAW** `[5]` (RW)
///
///   Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode.
///
/// - **SEND_S_REG_Q_INT_RAW** `[4]` (RW)
///
///   Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode.
///
/// - **TX_HUNG_INT_RAW** `[3]` (RW)
///
///   Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value.
///
/// - **RX_HUNG_INT_RAW** `[2]` (RW)
///
///   Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value.
///
/// - **TX_START_INT_RAW** `[1]` (RW)
///
///   Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter.
///
/// - **RX_START_INT_RAW** `[0]` (RW)
///
///   Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully.
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

    /// **APP_CTRL1_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const APP_CTRL1_INT_RAW_OFFSET: u32 = 8;
    pub const APP_CTRL1_INT_RAW_MASK: u32 = 0x00000100;
    pub const APP_CTRL1_INT_RAW: u32 = 1 << 8;

    /// **APP_CTRL0_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const APP_CTRL0_INT_RAW_OFFSET: u32 = 7;
    pub const APP_CTRL0_INT_RAW_MASK: u32 = 0x00000080;
    pub const APP_CTRL0_INT_RAW: u32 = 1 << 7;

    /// **OUT_EOF_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUT_EOF_INT_RAW_OFFSET: u32 = 6;
    pub const OUT_EOF_INT_RAW_MASK: u32 = 0x00000040;
    pub const OUT_EOF_INT_RAW: u32 = 1 << 6;

    /// **SEND_A_REG_Q_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SEND_A_REG_Q_INT_RAW_OFFSET: u32 = 5;
    pub const SEND_A_REG_Q_INT_RAW_MASK: u32 = 0x00000020;
    pub const SEND_A_REG_Q_INT_RAW: u32 = 1 << 5;

    /// **SEND_S_REG_Q_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SEND_S_REG_Q_INT_RAW_OFFSET: u32 = 4;
    pub const SEND_S_REG_Q_INT_RAW_MASK: u32 = 0x00000010;
    pub const SEND_S_REG_Q_INT_RAW: u32 = 1 << 4;

    /// **TX_HUNG_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_RAW_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_RAW_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_RAW: u32 = 1 << 3;

    /// **RX_HUNG_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_RAW_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_RAW_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_RAW: u32 = 1 << 2;

    /// **TX_START_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_START_INT_RAW_OFFSET: u32 = 1;
    pub const TX_START_INT_RAW_MASK: u32 = 0x00000002;
    pub const TX_START_INT_RAW: u32 = 1 << 1;

    /// **RX_START_INT_RAW**
    ///
    /// Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_START_INT_RAW_OFFSET: u32 = 0;
    pub const RX_START_INT_RAW_MASK: u32 = 0x00000001;
    pub const RX_START_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APP_CTRL1_INT_RAW** `[8]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1.
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL1_INT_RAW_MASK) != 0
    }

    /// Set APP_CTRL1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_app_ctrl1_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::APP_CTRL1_INT_RAW_MASK);
    }

    /// Clear APP_CTRL1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_app_ctrl1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::APP_CTRL1_INT_RAW_MASK);
    }

    /// **APP_CTRL0_INT_RAW** `[7]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1.
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL0_INT_RAW_MASK) != 0
    }

    /// Set APP_CTRL0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_app_ctrl0_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::APP_CTRL0_INT_RAW_MASK);
    }

    /// Clear APP_CTRL0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_app_ctrl0_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::APP_CTRL0_INT_RAW_MASK);
    }

    /// **OUT_EOF_INT_RAW** `[6]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF.
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_INT_RAW_MASK) != 0
    }

    /// Set OUT_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_INT_RAW_MASK);
    }

    /// Clear OUT_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_INT_RAW_MASK);
    }

    /// **SEND_A_REG_Q_INT_RAW** `[5]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode.
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SEND_A_REG_Q_INT_RAW_MASK) != 0
    }

    /// Set SEND_A_REG_Q_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_a_reg_q_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_A_REG_Q_INT_RAW_MASK);
    }

    /// Clear SEND_A_REG_Q_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_a_reg_q_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_A_REG_Q_INT_RAW_MASK);
    }

    /// **SEND_S_REG_Q_INT_RAW** `[4]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode.
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SEND_S_REG_Q_INT_RAW_MASK) != 0
    }

    /// Set SEND_S_REG_Q_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_s_reg_q_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_S_REG_Q_INT_RAW_MASK);
    }

    /// Clear SEND_S_REG_Q_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_s_reg_q_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_S_REG_Q_INT_RAW_MASK);
    }

    /// **TX_HUNG_INT_RAW** `[3]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value.
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_RAW_MASK) != 0
    }

    /// Set TX_HUNG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_hung_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TX_HUNG_INT_RAW_MASK);
    }

    /// Clear TX_HUNG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_hung_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TX_HUNG_INT_RAW_MASK);
    }

    /// **RX_HUNG_INT_RAW** `[2]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value.
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_RAW_MASK) != 0
    }

    /// Set RX_HUNG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_hung_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::RX_HUNG_INT_RAW_MASK);
    }

    /// Clear RX_HUNG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_hung_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::RX_HUNG_INT_RAW_MASK);
    }

    /// **TX_START_INT_RAW** `[1]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter.
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_START_INT_RAW_MASK) != 0
    }

    /// Set TX_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_start_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_INT_RAW_MASK);
    }

    /// Clear TX_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_start_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TX_START_INT_RAW_MASK);
    }

    /// **RX_START_INT_RAW** `[0]` (RW)
    ///
    /// Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully.
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_START_INT_RAW_MASK) != 0
    }

    /// Set RX_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_start_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::RX_START_INT_RAW_MASK);
    }

    /// Clear RX_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_start_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::RX_START_INT_RAW_MASK);
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

    /// Read **APP_CTRL1_INT_RAW** `[8:8]`
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1.
    #[inline(always)]
    pub const fn app_ctrl1_int_raw(&self) -> bool {
        (self.bits & IntRaw::APP_CTRL1_INT_RAW_MASK) != 0
    }

    /// Read **APP_CTRL0_INT_RAW** `[7:7]`
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1.
    #[inline(always)]
    pub const fn app_ctrl0_int_raw(&self) -> bool {
        (self.bits & IntRaw::APP_CTRL0_INT_RAW_MASK) != 0
    }

    /// Read **OUT_EOF_INT_RAW** `[6:6]`
    ///
    /// Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF.
    #[inline(always)]
    pub const fn out_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::OUT_EOF_INT_RAW_MASK) != 0
    }

    /// Read **SEND_A_REG_Q_INT_RAW** `[5:5]`
    ///
    /// Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode.
    #[inline(always)]
    pub const fn send_a_reg_q_int_raw(&self) -> bool {
        (self.bits & IntRaw::SEND_A_REG_Q_INT_RAW_MASK) != 0
    }

    /// Read **SEND_S_REG_Q_INT_RAW** `[4:4]`
    ///
    /// Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode.
    #[inline(always)]
    pub const fn send_s_reg_q_int_raw(&self) -> bool {
        (self.bits & IntRaw::SEND_S_REG_Q_INT_RAW_MASK) != 0
    }

    /// Read **TX_HUNG_INT_RAW** `[3:3]`
    ///
    /// Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value.
    #[inline(always)]
    pub const fn tx_hung_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_HUNG_INT_RAW_MASK) != 0
    }

    /// Read **RX_HUNG_INT_RAW** `[2:2]`
    ///
    /// Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value.
    #[inline(always)]
    pub const fn rx_hung_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_HUNG_INT_RAW_MASK) != 0
    }

    /// Read **TX_START_INT_RAW** `[1:1]`
    ///
    /// Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter.
    #[inline(always)]
    pub const fn tx_start_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_START_INT_RAW_MASK) != 0
    }

    /// Read **RX_START_INT_RAW** `[0:0]`
    ///
    /// Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully.
    #[inline(always)]
    pub const fn rx_start_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_START_INT_RAW_MASK) != 0
    }

    /// Set **APP_CTRL1_INT_RAW** `[8:8]`
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1.
    #[inline(always)]
    pub const fn with_app_ctrl1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::APP_CTRL1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::APP_CTRL1_INT_RAW_MASK
            },
        }
    }

    /// Set **APP_CTRL0_INT_RAW** `[7:7]`
    ///
    /// Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1.
    #[inline(always)]
    pub const fn with_app_ctrl0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::APP_CTRL0_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::APP_CTRL0_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_EOF_INT_RAW** `[6:6]`
    ///
    /// Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF.
    #[inline(always)]
    pub const fn with_out_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OUT_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OUT_EOF_INT_RAW_MASK
            },
        }
    }

    /// Set **SEND_A_REG_Q_INT_RAW** `[5:5]`
    ///
    /// Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode.
    #[inline(always)]
    pub const fn with_send_a_reg_q_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SEND_A_REG_Q_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SEND_A_REG_Q_INT_RAW_MASK
            },
        }
    }

    /// Set **SEND_S_REG_Q_INT_RAW** `[4:4]`
    ///
    /// Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode.
    #[inline(always)]
    pub const fn with_send_s_reg_q_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SEND_S_REG_Q_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SEND_S_REG_Q_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_HUNG_INT_RAW** `[3:3]`
    ///
    /// Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value.
    #[inline(always)]
    pub const fn with_tx_hung_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_HUNG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_HUNG_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_RAW** `[2:2]`
    ///
    /// Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value.
    #[inline(always)]
    pub const fn with_rx_hung_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_HUNG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_HUNG_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_START_INT_RAW** `[1:1]`
    ///
    /// Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter.
    #[inline(always)]
    pub const fn with_tx_start_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_START_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_START_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_START_INT_RAW** `[0:0]`
    ///
    /// Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully.
    #[inline(always)]
    pub const fn with_rx_start_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_START_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_START_INT_RAW_MASK
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
/// UHCI Interrupt Status Register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **APP_CTRL1_INT_ST** `[8]` (RO)
///
///   Indicates the interrupt status of UHCI_APP_CTRL1_INT.
///
/// - **APP_CTRL0_INT_ST** `[7]` (RO)
///
///   Indicates the interrupt status of UHCI_APP_CTRL0_INT.
///
/// - **OUTLINK_EOF_ERR_INT_ST** `[6]` (RO)
///
///   Indicates the interrupt status of UHCI_OUT_EOF_INT.
///
/// - **SEND_A_REG_Q_INT_ST** `[5]` (RO)
///
///   Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT.
///
/// - **SEND_S_REG_Q_INT_ST** `[4]` (RO)
///
///   Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT.
///
/// - **TX_HUNG_INT_ST** `[3]` (RO)
///
///   Indicates the interrupt status of UHCI_TX_HUNG_INT.
///
/// - **RX_HUNG_INT_ST** `[2]` (RO)
///
///   Indicates the interrupt status of UHCI_RX_HUNG_INT.
///
/// - **TX_START_INT_ST** `[1]` (RO)
///
///   Indicates the interrupt status of UHCI_TX_START_INT.
///
/// - **RX_START_INT_ST** `[0]` (RO)
///
///   Indicates the interrupt status of UHCI_RX_START_INT.
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

    /// **APP_CTRL1_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL1_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const APP_CTRL1_INT_ST_OFFSET: u32 = 8;
    pub const APP_CTRL1_INT_ST_MASK: u32 = 0x00000100;
    pub const APP_CTRL1_INT_ST: u32 = 1 << 8;

    /// **APP_CTRL0_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL0_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const APP_CTRL0_INT_ST_OFFSET: u32 = 7;
    pub const APP_CTRL0_INT_ST_MASK: u32 = 0x00000080;
    pub const APP_CTRL0_INT_ST: u32 = 1 << 7;

    /// **OUTLINK_EOF_ERR_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_OUT_EOF_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUTLINK_EOF_ERR_INT_ST_OFFSET: u32 = 6;
    pub const OUTLINK_EOF_ERR_INT_ST_MASK: u32 = 0x00000040;
    pub const OUTLINK_EOF_ERR_INT_ST: u32 = 1 << 6;

    /// **SEND_A_REG_Q_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SEND_A_REG_Q_INT_ST_OFFSET: u32 = 5;
    pub const SEND_A_REG_Q_INT_ST_MASK: u32 = 0x00000020;
    pub const SEND_A_REG_Q_INT_ST: u32 = 1 << 5;

    /// **SEND_S_REG_Q_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SEND_S_REG_Q_INT_ST_OFFSET: u32 = 4;
    pub const SEND_S_REG_Q_INT_ST_MASK: u32 = 0x00000010;
    pub const SEND_S_REG_Q_INT_ST: u32 = 1 << 4;

    /// **TX_HUNG_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_TX_HUNG_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_ST_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_ST_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_ST: u32 = 1 << 3;

    /// **RX_HUNG_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_RX_HUNG_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_ST_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_ST_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_ST: u32 = 1 << 2;

    /// **TX_START_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_TX_START_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_START_INT_ST_OFFSET: u32 = 1;
    pub const TX_START_INT_ST_MASK: u32 = 0x00000002;
    pub const TX_START_INT_ST: u32 = 1 << 1;

    /// **RX_START_INT_ST**
    ///
    /// Indicates the interrupt status of UHCI_RX_START_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_START_INT_ST_OFFSET: u32 = 0;
    pub const RX_START_INT_ST_MASK: u32 = 0x00000001;
    pub const RX_START_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APP_CTRL1_INT_ST** `[8]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub fn app_ctrl1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL1_INT_ST_MASK) != 0
    }

    /// **APP_CTRL0_INT_ST** `[7]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub fn app_ctrl0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL0_INT_ST_MASK) != 0
    }

    /// **OUTLINK_EOF_ERR_INT_ST** `[6]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub fn outlink_eof_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUTLINK_EOF_ERR_INT_ST_MASK) != 0
    }

    /// **SEND_A_REG_Q_INT_ST** `[5]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub fn send_a_reg_q_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SEND_A_REG_Q_INT_ST_MASK) != 0
    }

    /// **SEND_S_REG_Q_INT_ST** `[4]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub fn send_s_reg_q_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SEND_S_REG_Q_INT_ST_MASK) != 0
    }

    /// **TX_HUNG_INT_ST** `[3]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_ST_MASK) != 0
    }

    /// **RX_HUNG_INT_ST** `[2]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_RX_HUNG_INT.
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_HUNG_INT_ST_MASK) != 0
    }

    /// **TX_START_INT_ST** `[1]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_TX_START_INT.
    #[inline(always)]
    pub fn tx_start_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_START_INT_ST_MASK) != 0
    }

    /// **RX_START_INT_ST** `[0]` (RO)
    ///
    /// Indicates the interrupt status of UHCI_RX_START_INT.
    #[inline(always)]
    pub fn rx_start_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_START_INT_ST_MASK) != 0
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

    /// Read **APP_CTRL1_INT_ST** `[8:8]`
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub const fn app_ctrl1_int_st(&self) -> bool {
        (self.bits & IntSt::APP_CTRL1_INT_ST_MASK) != 0
    }

    /// Read **APP_CTRL0_INT_ST** `[7:7]`
    ///
    /// Indicates the interrupt status of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub const fn app_ctrl0_int_st(&self) -> bool {
        (self.bits & IntSt::APP_CTRL0_INT_ST_MASK) != 0
    }

    /// Read **OUTLINK_EOF_ERR_INT_ST** `[6:6]`
    ///
    /// Indicates the interrupt status of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub const fn outlink_eof_err_int_st(&self) -> bool {
        (self.bits & IntSt::OUTLINK_EOF_ERR_INT_ST_MASK) != 0
    }

    /// Read **SEND_A_REG_Q_INT_ST** `[5:5]`
    ///
    /// Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub const fn send_a_reg_q_int_st(&self) -> bool {
        (self.bits & IntSt::SEND_A_REG_Q_INT_ST_MASK) != 0
    }

    /// Read **SEND_S_REG_Q_INT_ST** `[4:4]`
    ///
    /// Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub const fn send_s_reg_q_int_st(&self) -> bool {
        (self.bits & IntSt::SEND_S_REG_Q_INT_ST_MASK) != 0
    }

    /// Read **TX_HUNG_INT_ST** `[3:3]`
    ///
    /// Indicates the interrupt status of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub const fn tx_hung_int_st(&self) -> bool {
        (self.bits & IntSt::TX_HUNG_INT_ST_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ST** `[2:2]`
    ///
    /// Indicates the interrupt status of UHCI_RX_HUNG_INT.
    #[inline(always)]
    pub const fn rx_hung_int_st(&self) -> bool {
        (self.bits & IntSt::RX_HUNG_INT_ST_MASK) != 0
    }

    /// Read **TX_START_INT_ST** `[1:1]`
    ///
    /// Indicates the interrupt status of UHCI_TX_START_INT.
    #[inline(always)]
    pub const fn tx_start_int_st(&self) -> bool {
        (self.bits & IntSt::TX_START_INT_ST_MASK) != 0
    }

    /// Read **RX_START_INT_ST** `[0:0]`
    ///
    /// Indicates the interrupt status of UHCI_RX_START_INT.
    #[inline(always)]
    pub const fn rx_start_int_st(&self) -> bool {
        (self.bits & IntSt::RX_START_INT_ST_MASK) != 0
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
/// UHCI Interrupt Enable Register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **APP_CTRL1_INT_ENA** `[8]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
///
/// - **APP_CTRL0_INT_ENA** `[7]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
///
/// - **OUTLINK_EOF_ERR_INT_ENA** `[6]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
///
/// - **SEND_A_REG_Q_INT_ENA** `[5]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
///
/// - **SEND_S_REG_Q_INT_ENA** `[4]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
///
/// - **TX_HUNG_INT_ENA** `[3]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
///
/// - **RX_HUNG_INT_ENA** `[2]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
///
/// - **TX_START_INT_ENA** `[1]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_TX_START_INT.
///
/// - **RX_START_INT_ENA** `[0]` (RW)
///
///   Set this bit to enable the interrupt of UHCI_RX_START_INT.
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

    /// **APP_CTRL1_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const APP_CTRL1_INT_ENA_OFFSET: u32 = 8;
    pub const APP_CTRL1_INT_ENA_MASK: u32 = 0x00000100;
    pub const APP_CTRL1_INT_ENA: u32 = 1 << 8;

    /// **APP_CTRL0_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const APP_CTRL0_INT_ENA_OFFSET: u32 = 7;
    pub const APP_CTRL0_INT_ENA_MASK: u32 = 0x00000080;
    pub const APP_CTRL0_INT_ENA: u32 = 1 << 7;

    /// **OUTLINK_EOF_ERR_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUTLINK_EOF_ERR_INT_ENA_OFFSET: u32 = 6;
    pub const OUTLINK_EOF_ERR_INT_ENA_MASK: u32 = 0x00000040;
    pub const OUTLINK_EOF_ERR_INT_ENA: u32 = 1 << 6;

    /// **SEND_A_REG_Q_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SEND_A_REG_Q_INT_ENA_OFFSET: u32 = 5;
    pub const SEND_A_REG_Q_INT_ENA_MASK: u32 = 0x00000020;
    pub const SEND_A_REG_Q_INT_ENA: u32 = 1 << 5;

    /// **SEND_S_REG_Q_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SEND_S_REG_Q_INT_ENA_OFFSET: u32 = 4;
    pub const SEND_S_REG_Q_INT_ENA_MASK: u32 = 0x00000010;
    pub const SEND_S_REG_Q_INT_ENA: u32 = 1 << 4;

    /// **TX_HUNG_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_ENA_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_ENA_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_ENA: u32 = 1 << 3;

    /// **RX_HUNG_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_ENA_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_ENA_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_ENA: u32 = 1 << 2;

    /// **TX_START_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_START_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_START_INT_ENA_OFFSET: u32 = 1;
    pub const TX_START_INT_ENA_MASK: u32 = 0x00000002;
    pub const TX_START_INT_ENA: u32 = 1 << 1;

    /// **RX_START_INT_ENA**
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_START_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_START_INT_ENA_OFFSET: u32 = 0;
    pub const RX_START_INT_ENA_MASK: u32 = 0x00000001;
    pub const RX_START_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APP_CTRL1_INT_ENA** `[8]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL1_INT_ENA_MASK) != 0
    }

    /// Set APP_CTRL1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_app_ctrl1_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::APP_CTRL1_INT_ENA_MASK);
    }

    /// Clear APP_CTRL1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_app_ctrl1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::APP_CTRL1_INT_ENA_MASK);
    }

    /// **APP_CTRL0_INT_ENA** `[7]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::APP_CTRL0_INT_ENA_MASK) != 0
    }

    /// Set APP_CTRL0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_app_ctrl0_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::APP_CTRL0_INT_ENA_MASK);
    }

    /// Clear APP_CTRL0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_app_ctrl0_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::APP_CTRL0_INT_ENA_MASK);
    }

    /// **OUTLINK_EOF_ERR_INT_ENA** `[6]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUTLINK_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Set OUTLINK_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_outlink_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OUTLINK_EOF_ERR_INT_ENA_MASK);
    }

    /// Clear OUTLINK_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_outlink_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OUTLINK_EOF_ERR_INT_ENA_MASK);
    }

    /// **SEND_A_REG_Q_INT_ENA** `[5]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SEND_A_REG_Q_INT_ENA_MASK) != 0
    }

    /// Set SEND_A_REG_Q_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_a_reg_q_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_A_REG_Q_INT_ENA_MASK);
    }

    /// Clear SEND_A_REG_Q_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_a_reg_q_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_A_REG_Q_INT_ENA_MASK);
    }

    /// **SEND_S_REG_Q_INT_ENA** `[4]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SEND_S_REG_Q_INT_ENA_MASK) != 0
    }

    /// Set SEND_S_REG_Q_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_s_reg_q_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SEND_S_REG_Q_INT_ENA_MASK);
    }

    /// Clear SEND_S_REG_Q_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_s_reg_q_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SEND_S_REG_Q_INT_ENA_MASK);
    }

    /// **TX_HUNG_INT_ENA** `[3]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_HUNG_INT_ENA_MASK) != 0
    }

    /// Set TX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_HUNG_INT_ENA_MASK);
    }

    /// Clear TX_HUNG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_hung_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_HUNG_INT_ENA_MASK);
    }

    /// **RX_HUNG_INT_ENA** `[2]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
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

    /// **TX_START_INT_ENA** `[1]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_START_INT_ENA_MASK) != 0
    }

    /// Set TX_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_start_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TX_START_INT_ENA_MASK);
    }

    /// Clear TX_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_start_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TX_START_INT_ENA_MASK);
    }

    /// **RX_START_INT_ENA** `[0]` (RW)
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_START_INT_ENA_MASK) != 0
    }

    /// Set RX_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_start_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::RX_START_INT_ENA_MASK);
    }

    /// Clear RX_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_start_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RX_START_INT_ENA_MASK);
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

    /// Read **APP_CTRL1_INT_ENA** `[8:8]`
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub const fn app_ctrl1_int_ena(&self) -> bool {
        (self.bits & IntEna::APP_CTRL1_INT_ENA_MASK) != 0
    }

    /// Read **APP_CTRL0_INT_ENA** `[7:7]`
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub const fn app_ctrl0_int_ena(&self) -> bool {
        (self.bits & IntEna::APP_CTRL0_INT_ENA_MASK) != 0
    }

    /// Read **OUTLINK_EOF_ERR_INT_ENA** `[6:6]`
    ///
    /// Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub const fn outlink_eof_err_int_ena(&self) -> bool {
        (self.bits & IntEna::OUTLINK_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SEND_A_REG_Q_INT_ENA** `[5:5]`
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub const fn send_a_reg_q_int_ena(&self) -> bool {
        (self.bits & IntEna::SEND_A_REG_Q_INT_ENA_MASK) != 0
    }

    /// Read **SEND_S_REG_Q_INT_ENA** `[4:4]`
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub const fn send_s_reg_q_int_ena(&self) -> bool {
        (self.bits & IntEna::SEND_S_REG_Q_INT_ENA_MASK) != 0
    }

    /// Read **TX_HUNG_INT_ENA** `[3:3]`
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub const fn tx_hung_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_HUNG_INT_ENA_MASK) != 0
    }

    /// Read **RX_HUNG_INT_ENA** `[2:2]`
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
    #[inline(always)]
    pub const fn rx_hung_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_HUNG_INT_ENA_MASK) != 0
    }

    /// Read **TX_START_INT_ENA** `[1:1]`
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub const fn tx_start_int_ena(&self) -> bool {
        (self.bits & IntEna::TX_START_INT_ENA_MASK) != 0
    }

    /// Read **RX_START_INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub const fn rx_start_int_ena(&self) -> bool {
        (self.bits & IntEna::RX_START_INT_ENA_MASK) != 0
    }

    /// Set **APP_CTRL1_INT_ENA** `[8:8]`
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub const fn with_app_ctrl1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::APP_CTRL1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::APP_CTRL1_INT_ENA_MASK
            },
        }
    }

    /// Set **APP_CTRL0_INT_ENA** `[7:7]`
    ///
    /// Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub const fn with_app_ctrl0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::APP_CTRL0_INT_ENA_MASK
            } else {
                self.bits & !IntEna::APP_CTRL0_INT_ENA_MASK
            },
        }
    }

    /// Set **OUTLINK_EOF_ERR_INT_ENA** `[6:6]`
    ///
    /// Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub const fn with_outlink_eof_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OUTLINK_EOF_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OUTLINK_EOF_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SEND_A_REG_Q_INT_ENA** `[5:5]`
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub const fn with_send_a_reg_q_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SEND_A_REG_Q_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SEND_A_REG_Q_INT_ENA_MASK
            },
        }
    }

    /// Set **SEND_S_REG_Q_INT_ENA** `[4:4]`
    ///
    /// Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub const fn with_send_s_reg_q_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SEND_S_REG_Q_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SEND_S_REG_Q_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_HUNG_INT_ENA** `[3:3]`
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub const fn with_tx_hung_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_HUNG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_HUNG_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_ENA** `[2:2]`
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
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

    /// Set **TX_START_INT_ENA** `[1:1]`
    ///
    /// Set this bit to enable the interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub const fn with_tx_start_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TX_START_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TX_START_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_START_INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable the interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub const fn with_rx_start_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RX_START_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RX_START_INT_ENA_MASK
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
/// UHCI Interrupt Clear Register
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **APP_CTRL1_INT_CLR** `[8]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT.
///
/// - **APP_CTRL0_INT_CLR** `[7]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT.
///
/// - **OUTLINK_EOF_ERR_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT.
///
/// - **SEND_A_REG_Q_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT.
///
/// - **SEND_S_REG_Q_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT.
///
/// - **TX_HUNG_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT.
///
/// - **RX_HUNG_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT.
///
/// - **TX_START_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_TX_START_INT.
///
/// - **RX_START_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the raw interrupt of UHCI_RX_START_INT.
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

    /// **APP_CTRL1_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const APP_CTRL1_INT_CLR_OFFSET: u32 = 8;
    pub const APP_CTRL1_INT_CLR_MASK: u32 = 0x00000100;
    pub const APP_CTRL1_INT_CLR: u32 = 1 << 8;

    /// **APP_CTRL0_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const APP_CTRL0_INT_CLR_OFFSET: u32 = 7;
    pub const APP_CTRL0_INT_CLR_MASK: u32 = 0x00000080;
    pub const APP_CTRL0_INT_CLR: u32 = 1 << 7;

    /// **OUTLINK_EOF_ERR_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OUTLINK_EOF_ERR_INT_CLR_OFFSET: u32 = 6;
    pub const OUTLINK_EOF_ERR_INT_CLR_MASK: u32 = 0x00000040;
    pub const OUTLINK_EOF_ERR_INT_CLR: u32 = 1 << 6;

    /// **SEND_A_REG_Q_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SEND_A_REG_Q_INT_CLR_OFFSET: u32 = 5;
    pub const SEND_A_REG_Q_INT_CLR_MASK: u32 = 0x00000020;
    pub const SEND_A_REG_Q_INT_CLR: u32 = 1 << 5;

    /// **SEND_S_REG_Q_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SEND_S_REG_Q_INT_CLR_OFFSET: u32 = 4;
    pub const SEND_S_REG_Q_INT_CLR_MASK: u32 = 0x00000010;
    pub const SEND_S_REG_Q_INT_CLR: u32 = 1 << 4;

    /// **TX_HUNG_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_HUNG_INT_CLR_OFFSET: u32 = 3;
    pub const TX_HUNG_INT_CLR_MASK: u32 = 0x00000008;
    pub const TX_HUNG_INT_CLR: u32 = 1 << 3;

    /// **RX_HUNG_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_HUNG_INT_CLR_OFFSET: u32 = 2;
    pub const RX_HUNG_INT_CLR_MASK: u32 = 0x00000004;
    pub const RX_HUNG_INT_CLR: u32 = 1 << 2;

    /// **TX_START_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_START_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_START_INT_CLR_OFFSET: u32 = 1;
    pub const TX_START_INT_CLR_MASK: u32 = 0x00000002;
    pub const TX_START_INT_CLR: u32 = 1 << 1;

    /// **RX_START_INT_CLR**
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_START_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_START_INT_CLR_OFFSET: u32 = 0;
    pub const RX_START_INT_CLR_MASK: u32 = 0x00000001;
    pub const RX_START_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **APP_CTRL1_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub fn trigger_app_ctrl1_int_clr(&self) {
        self.value.write_bits(Self::APP_CTRL1_INT_CLR_MASK);
    }

    /// Trigger **APP_CTRL0_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub fn trigger_app_ctrl0_int_clr(&self) {
        self.value.write_bits(Self::APP_CTRL0_INT_CLR_MASK);
    }

    /// Trigger **OUTLINK_EOF_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub fn trigger_outlink_eof_err_int_clr(&self) {
        self.value.write_bits(Self::OUTLINK_EOF_ERR_INT_CLR_MASK);
    }

    /// Trigger **SEND_A_REG_Q_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub fn trigger_send_a_reg_q_int_clr(&self) {
        self.value.write_bits(Self::SEND_A_REG_Q_INT_CLR_MASK);
    }

    /// Trigger **SEND_S_REG_Q_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub fn trigger_send_s_reg_q_int_clr(&self) {
        self.value.write_bits(Self::SEND_S_REG_Q_INT_CLR_MASK);
    }

    /// Trigger **TX_HUNG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub fn trigger_tx_hung_int_clr(&self) {
        self.value.write_bits(Self::TX_HUNG_INT_CLR_MASK);
    }

    /// Trigger **RX_HUNG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT.
    #[inline(always)]
    pub fn trigger_rx_hung_int_clr(&self) {
        self.value.write_bits(Self::RX_HUNG_INT_CLR_MASK);
    }

    /// Trigger **TX_START_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub fn trigger_tx_start_int_clr(&self) {
        self.value.write_bits(Self::TX_START_INT_CLR_MASK);
    }

    /// Trigger **RX_START_INT_CLR** (WO)
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub fn trigger_rx_start_int_clr(&self) {
        self.value.write_bits(Self::RX_START_INT_CLR_MASK);
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

    /// Set **APP_CTRL1_INT_CLR** `[8:8]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub const fn with_app_ctrl1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::APP_CTRL1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::APP_CTRL1_INT_CLR_MASK
            },
        }
    }

    /// Set **APP_CTRL0_INT_CLR** `[7:7]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub const fn with_app_ctrl0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::APP_CTRL0_INT_CLR_MASK
            } else {
                self.bits & !IntClr::APP_CTRL0_INT_CLR_MASK
            },
        }
    }

    /// Set **OUTLINK_EOF_ERR_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub const fn with_outlink_eof_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OUTLINK_EOF_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OUTLINK_EOF_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SEND_A_REG_Q_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub const fn with_send_a_reg_q_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SEND_A_REG_Q_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SEND_A_REG_Q_INT_CLR_MASK
            },
        }
    }

    /// Set **SEND_S_REG_Q_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub const fn with_send_s_reg_q_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SEND_S_REG_Q_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SEND_S_REG_Q_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_HUNG_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub const fn with_tx_hung_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_HUNG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_HUNG_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_HUNG_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_HUNG_INT.
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

    /// Set **TX_START_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub const fn with_tx_start_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_START_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_START_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_START_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the raw interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub const fn with_rx_start_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_START_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_START_INT_CLR_MASK
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

/// CONF1 Register
///
/// UHCI Configuration Register1
///
/// Address offset: 0x0014
/// Reset value: 0x00000033
///
/// ## Fields
///
/// - **SW_START** `[8]` (WO)
///
///   Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT.
///
/// - **WAIT_SW_START** `[7]` (RW)
///
///   Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status.
///
/// - **TX_ACK_NUM_RE** `[5]` (RW)
///
///   Set this bit to encode data packet with ACK when reliable data packet is ready.
///
/// - **TX_CHECK_SUM_RE** `[4]` (RW)
///
///   Set this bit to encode data packet with checksum.
///
/// - **SAVE_HEAD** `[3]` (RW)
///
///   Set this bit to save data packet head when UHCI receive data.
///
/// - **CRC_DISABLE** `[2]` (RW)
///
///   Set this bit to support CRC calculation, and data integrity check bit should 1.
///
/// - **CHECK_SEQ_EN** `[1]` (RW)
///
///   Set this bit to enable sequence number check when receiving.
///
/// - **CHECK_SUM_EN** `[0]` (RW)
///
///   Set this bit to enable head checksum check when receiving.
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

    /// **SW_START**
    ///
    /// Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SW_START_OFFSET: u32 = 8;
    pub const SW_START_MASK: u32 = 0x00000100;
    pub const SW_START: u32 = 1 << 8;

    /// **WAIT_SW_START**
    ///
    /// Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const WAIT_SW_START_OFFSET: u32 = 7;
    pub const WAIT_SW_START_MASK: u32 = 0x00000080;
    pub const WAIT_SW_START: u32 = 1 << 7;

    /// **TX_ACK_NUM_RE**
    ///
    /// Set this bit to encode data packet with ACK when reliable data packet is ready.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_ACK_NUM_RE_OFFSET: u32 = 5;
    pub const TX_ACK_NUM_RE_MASK: u32 = 0x00000020;
    pub const TX_ACK_NUM_RE: u32 = 1 << 5;

    /// **TX_CHECK_SUM_RE**
    ///
    /// Set this bit to encode data packet with checksum.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_CHECK_SUM_RE_OFFSET: u32 = 4;
    pub const TX_CHECK_SUM_RE_MASK: u32 = 0x00000010;
    pub const TX_CHECK_SUM_RE: u32 = 1 << 4;

    /// **SAVE_HEAD**
    ///
    /// Set this bit to save data packet head when UHCI receive data.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SAVE_HEAD_OFFSET: u32 = 3;
    pub const SAVE_HEAD_MASK: u32 = 0x00000008;
    pub const SAVE_HEAD: u32 = 1 << 3;

    /// **CRC_DISABLE**
    ///
    /// Set this bit to support CRC calculation, and data integrity check bit should 1.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CRC_DISABLE_OFFSET: u32 = 2;
    pub const CRC_DISABLE_MASK: u32 = 0x00000004;
    pub const CRC_DISABLE: u32 = 1 << 2;

    /// **CHECK_SEQ_EN**
    ///
    /// Set this bit to enable sequence number check when receiving.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CHECK_SEQ_EN_OFFSET: u32 = 1;
    pub const CHECK_SEQ_EN_MASK: u32 = 0x00000002;
    pub const CHECK_SEQ_EN: u32 = 1 << 1;

    /// **CHECK_SUM_EN**
    ///
    /// Set this bit to enable head checksum check when receiving.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CHECK_SUM_EN_OFFSET: u32 = 0;
    pub const CHECK_SUM_EN_MASK: u32 = 0x00000001;
    pub const CHECK_SUM_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SW_START** (WO)
    ///
    /// Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT.
    #[inline(always)]
    pub fn trigger_sw_start(&self) {
        self.value.modify_bits(|r| r | Self::SW_START_MASK);
    }

    /// **WAIT_SW_START** `[7]` (RW)
    ///
    /// Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status.
    #[inline(always)]
    pub fn wait_sw_start(&self) -> bool {
        (self.value.read_bits() & Self::WAIT_SW_START_MASK) != 0
    }

    /// Set WAIT_SW_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wait_sw_start(&self) {
        self.value.modify_bits(|r| r | Self::WAIT_SW_START_MASK);
    }

    /// Clear WAIT_SW_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wait_sw_start(&self) {
        self.value.modify_bits(|r| r & !Self::WAIT_SW_START_MASK);
    }

    /// **TX_ACK_NUM_RE** `[5]` (RW)
    ///
    /// Set this bit to encode data packet with ACK when reliable data packet is ready.
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> bool {
        (self.value.read_bits() & Self::TX_ACK_NUM_RE_MASK) != 0
    }

    /// Set TX_ACK_NUM_RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_ack_num_re(&self) {
        self.value.modify_bits(|r| r | Self::TX_ACK_NUM_RE_MASK);
    }

    /// Clear TX_ACK_NUM_RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_ack_num_re(&self) {
        self.value.modify_bits(|r| r & !Self::TX_ACK_NUM_RE_MASK);
    }

    /// **TX_CHECK_SUM_RE** `[4]` (RW)
    ///
    /// Set this bit to encode data packet with checksum.
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> bool {
        (self.value.read_bits() & Self::TX_CHECK_SUM_RE_MASK) != 0
    }

    /// Set TX_CHECK_SUM_RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_check_sum_re(&self) {
        self.value.modify_bits(|r| r | Self::TX_CHECK_SUM_RE_MASK);
    }

    /// Clear TX_CHECK_SUM_RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_check_sum_re(&self) {
        self.value.modify_bits(|r| r & !Self::TX_CHECK_SUM_RE_MASK);
    }

    /// **SAVE_HEAD** `[3]` (RW)
    ///
    /// Set this bit to save data packet head when UHCI receive data.
    #[inline(always)]
    pub fn save_head(&self) -> bool {
        (self.value.read_bits() & Self::SAVE_HEAD_MASK) != 0
    }

    /// Set SAVE_HEAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_save_head(&self) {
        self.value.modify_bits(|r| r | Self::SAVE_HEAD_MASK);
    }

    /// Clear SAVE_HEAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_save_head(&self) {
        self.value.modify_bits(|r| r & !Self::SAVE_HEAD_MASK);
    }

    /// **CRC_DISABLE** `[2]` (RW)
    ///
    /// Set this bit to support CRC calculation, and data integrity check bit should 1.
    #[inline(always)]
    pub fn crc_disable(&self) -> bool {
        (self.value.read_bits() & Self::CRC_DISABLE_MASK) != 0
    }

    /// Set CRC_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crc_disable(&self) {
        self.value.modify_bits(|r| r | Self::CRC_DISABLE_MASK);
    }

    /// Clear CRC_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crc_disable(&self) {
        self.value.modify_bits(|r| r & !Self::CRC_DISABLE_MASK);
    }

    /// **CHECK_SEQ_EN** `[1]` (RW)
    ///
    /// Set this bit to enable sequence number check when receiving.
    #[inline(always)]
    pub fn check_seq_en(&self) -> bool {
        (self.value.read_bits() & Self::CHECK_SEQ_EN_MASK) != 0
    }

    /// Set CHECK_SEQ_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_check_seq_en(&self) {
        self.value.modify_bits(|r| r | Self::CHECK_SEQ_EN_MASK);
    }

    /// Clear CHECK_SEQ_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_check_seq_en(&self) {
        self.value.modify_bits(|r| r & !Self::CHECK_SEQ_EN_MASK);
    }

    /// **CHECK_SUM_EN** `[0]` (RW)
    ///
    /// Set this bit to enable head checksum check when receiving.
    #[inline(always)]
    pub fn check_sum_en(&self) -> bool {
        (self.value.read_bits() & Self::CHECK_SUM_EN_MASK) != 0
    }

    /// Set CHECK_SUM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_check_sum_en(&self) {
        self.value.modify_bits(|r| r | Self::CHECK_SUM_EN_MASK);
    }

    /// Clear CHECK_SUM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_check_sum_en(&self) {
        self.value.modify_bits(|r| r & !Self::CHECK_SUM_EN_MASK);
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
        Self { bits: 0x00000033 }
    }

    /// Read **WAIT_SW_START** `[7:7]`
    ///
    /// Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status.
    #[inline(always)]
    pub const fn wait_sw_start(&self) -> bool {
        (self.bits & Conf1::WAIT_SW_START_MASK) != 0
    }

    /// Read **TX_ACK_NUM_RE** `[5:5]`
    ///
    /// Set this bit to encode data packet with ACK when reliable data packet is ready.
    #[inline(always)]
    pub const fn tx_ack_num_re(&self) -> bool {
        (self.bits & Conf1::TX_ACK_NUM_RE_MASK) != 0
    }

    /// Read **TX_CHECK_SUM_RE** `[4:4]`
    ///
    /// Set this bit to encode data packet with checksum.
    #[inline(always)]
    pub const fn tx_check_sum_re(&self) -> bool {
        (self.bits & Conf1::TX_CHECK_SUM_RE_MASK) != 0
    }

    /// Read **SAVE_HEAD** `[3:3]`
    ///
    /// Set this bit to save data packet head when UHCI receive data.
    #[inline(always)]
    pub const fn save_head(&self) -> bool {
        (self.bits & Conf1::SAVE_HEAD_MASK) != 0
    }

    /// Read **CRC_DISABLE** `[2:2]`
    ///
    /// Set this bit to support CRC calculation, and data integrity check bit should 1.
    #[inline(always)]
    pub const fn crc_disable(&self) -> bool {
        (self.bits & Conf1::CRC_DISABLE_MASK) != 0
    }

    /// Read **CHECK_SEQ_EN** `[1:1]`
    ///
    /// Set this bit to enable sequence number check when receiving.
    #[inline(always)]
    pub const fn check_seq_en(&self) -> bool {
        (self.bits & Conf1::CHECK_SEQ_EN_MASK) != 0
    }

    /// Read **CHECK_SUM_EN** `[0:0]`
    ///
    /// Set this bit to enable head checksum check when receiving.
    #[inline(always)]
    pub const fn check_sum_en(&self) -> bool {
        (self.bits & Conf1::CHECK_SUM_EN_MASK) != 0
    }

    /// Set **SW_START** `[8:8]`
    ///
    /// Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT.
    #[inline(always)]
    pub const fn with_sw_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::SW_START_MASK
            } else {
                self.bits & !Conf1::SW_START_MASK
            },
        }
    }

    /// Set **WAIT_SW_START** `[7:7]`
    ///
    /// Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status.
    #[inline(always)]
    pub const fn with_wait_sw_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::WAIT_SW_START_MASK
            } else {
                self.bits & !Conf1::WAIT_SW_START_MASK
            },
        }
    }

    /// Set **TX_ACK_NUM_RE** `[5:5]`
    ///
    /// Set this bit to encode data packet with ACK when reliable data packet is ready.
    #[inline(always)]
    pub const fn with_tx_ack_num_re(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::TX_ACK_NUM_RE_MASK
            } else {
                self.bits & !Conf1::TX_ACK_NUM_RE_MASK
            },
        }
    }

    /// Set **TX_CHECK_SUM_RE** `[4:4]`
    ///
    /// Set this bit to encode data packet with checksum.
    #[inline(always)]
    pub const fn with_tx_check_sum_re(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::TX_CHECK_SUM_RE_MASK
            } else {
                self.bits & !Conf1::TX_CHECK_SUM_RE_MASK
            },
        }
    }

    /// Set **SAVE_HEAD** `[3:3]`
    ///
    /// Set this bit to save data packet head when UHCI receive data.
    #[inline(always)]
    pub const fn with_save_head(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::SAVE_HEAD_MASK
            } else {
                self.bits & !Conf1::SAVE_HEAD_MASK
            },
        }
    }

    /// Set **CRC_DISABLE** `[2:2]`
    ///
    /// Set this bit to support CRC calculation, and data integrity check bit should 1.
    #[inline(always)]
    pub const fn with_crc_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::CRC_DISABLE_MASK
            } else {
                self.bits & !Conf1::CRC_DISABLE_MASK
            },
        }
    }

    /// Set **CHECK_SEQ_EN** `[1:1]`
    ///
    /// Set this bit to enable sequence number check when receiving.
    #[inline(always)]
    pub const fn with_check_seq_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::CHECK_SEQ_EN_MASK
            } else {
                self.bits & !Conf1::CHECK_SEQ_EN_MASK
            },
        }
    }

    /// Set **CHECK_SUM_EN** `[0:0]`
    ///
    /// Set this bit to enable head checksum check when receiving.
    #[inline(always)]
    pub const fn with_check_sum_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf1::CHECK_SUM_EN_MASK
            } else {
                self.bits & !Conf1::CHECK_SUM_EN_MASK
            },
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

/// STATE0 Register
///
/// UHCI Receive Status Register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **DECODE_STATE** `[5:3]` (RO)
///
///   Indicates UHCI decoder status.
///
/// - **RX_ERR_CAUSE** `[2:0]` (RO)
///
///   Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error.
#[repr(transparent)]
pub struct State0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for State0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl State0 {
    // ======== Field Constants ========

    /// **DECODE_STATE**
    ///
    /// Indicates UHCI decoder status.
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const DECODE_STATE_OFFSET: u32 = 3;
    pub const DECODE_STATE_MASK: u32 = 0x00000038;

    /// **RX_ERR_CAUSE**
    ///
    /// Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const RX_ERR_CAUSE_OFFSET: u32 = 0;
    pub const RX_ERR_CAUSE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **DECODE_STATE** `[5:3]` (RO)
    ///
    /// Indicates UHCI decoder status.
    #[inline(always)]
    pub fn decode_state(&self) -> u32 {
        (self.value.read_bits() & Self::DECODE_STATE_MASK) >> Self::DECODE_STATE_OFFSET
    }

    /// **RX_ERR_CAUSE** `[2:0]` (RO)
    ///
    /// Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error.
    #[inline(always)]
    pub fn rx_err_cause(&self) -> u32 {
        (self.value.read_bits() & Self::RX_ERR_CAUSE_MASK) >> Self::RX_ERR_CAUSE_OFFSET
    }
}

/// STATE0 value type for batch read/modify/write operations
///
/// Use `state0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `state0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct State0Val {
    bits: u32,
}

impl State0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DECODE_STATE** `[5:3]`
    ///
    /// Indicates UHCI decoder status.
    #[inline(always)]
    pub const fn decode_state(&self) -> u32 {
        (self.bits & State0::DECODE_STATE_MASK) >> State0::DECODE_STATE_OFFSET
    }

    /// Read **RX_ERR_CAUSE** `[2:0]`
    ///
    /// Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error.
    #[inline(always)]
    pub const fn rx_err_cause(&self) -> u32 {
        (self.bits & State0::RX_ERR_CAUSE_MASK) >> State0::RX_ERR_CAUSE_OFFSET
    }
}

impl Default for State0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl State0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> State0Val {
        State0Val::from_bits(self.value.read_bits())
    }
}

/// STATE1 Register
///
/// UHCI Transmit Status Register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **ENCODE_STATE** `[2:0]` (RO)
///
///   Indicates UHCI encoder status.
#[repr(transparent)]
pub struct State1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for State1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl State1 {
    // ======== Field Constants ========

    /// **ENCODE_STATE**
    ///
    /// Indicates UHCI encoder status.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const ENCODE_STATE_OFFSET: u32 = 0;
    pub const ENCODE_STATE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **ENCODE_STATE** `[2:0]` (RO)
    ///
    /// Indicates UHCI encoder status.
    #[inline(always)]
    pub fn encode_state(&self) -> u32 {
        (self.value.read_bits() & Self::ENCODE_STATE_MASK) >> Self::ENCODE_STATE_OFFSET
    }
}

/// STATE1 value type for batch read/modify/write operations
///
/// Use `state1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `state1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct State1Val {
    bits: u32,
}

impl State1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ENCODE_STATE** `[2:0]`
    ///
    /// Indicates UHCI encoder status.
    #[inline(always)]
    pub const fn encode_state(&self) -> u32 {
        (self.bits & State1::ENCODE_STATE_MASK) >> State1::ENCODE_STATE_OFFSET
    }
}

impl Default for State1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl State1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> State1Val {
        State1Val::from_bits(self.value.read_bits())
    }
}

/// ESCAPE_CONF Register
///
/// UHCI Escapes Configuration Register0
///
/// Address offset: 0x0020
/// Reset value: 0x00000033
///
/// ## Fields
///
/// - **RX_13_ESC_EN** `[7]` (RW)
///
///   Set this bit to enable replacing 0x13 with special char when DMA receiving data.
///
/// - **RX_11_ESC_EN** `[6]` (RW)
///
///   Set this bit to enable replacing 0x11 with special char when DMA receiving data.
///
/// - **RX_DB_ESC_EN** `[5]` (RW)
///
///   Set this bit to enable replacing 0xDB with special char when DMA receiving data.
///
/// - **RX_C0_ESC_EN** `[4]` (RW)
///
///   Set this bit to enable replacing 0xC0 with special char when DMA receiving data.
///
/// - **TX_13_ESC_EN** `[3]` (RW)
///
///   Set this bit to enable resolve flow control char 0x13 when DMA receiving data.
///
/// - **TX_11_ESC_EN** `[2]` (RW)
///
///   Set this bit to enable resolve flow control char 0x11 when DMA receiving data.
///
/// - **TX_DB_ESC_EN** `[1]` (RW)
///
///   Set this bit to enable resolve char 0xDB when DMA receiving data.
///
/// - **TX_C0_ESC_EN** `[0]` (RW)
///
///   Set this bit to enable resolve char 0xC0 when DMA receiving data.
#[repr(transparent)]
pub struct EscapeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EscapeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EscapeConf {
    // ======== Field Constants ========

    /// **RX_13_ESC_EN**
    ///
    /// Set this bit to enable replacing 0x13 with special char when DMA receiving data.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_13_ESC_EN_OFFSET: u32 = 7;
    pub const RX_13_ESC_EN_MASK: u32 = 0x00000080;
    pub const RX_13_ESC_EN: u32 = 1 << 7;

    /// **RX_11_ESC_EN**
    ///
    /// Set this bit to enable replacing 0x11 with special char when DMA receiving data.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_11_ESC_EN_OFFSET: u32 = 6;
    pub const RX_11_ESC_EN_MASK: u32 = 0x00000040;
    pub const RX_11_ESC_EN: u32 = 1 << 6;

    /// **RX_DB_ESC_EN**
    ///
    /// Set this bit to enable replacing 0xDB with special char when DMA receiving data.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RX_DB_ESC_EN_OFFSET: u32 = 5;
    pub const RX_DB_ESC_EN_MASK: u32 = 0x00000020;
    pub const RX_DB_ESC_EN: u32 = 1 << 5;

    /// **RX_C0_ESC_EN**
    ///
    /// Set this bit to enable replacing 0xC0 with special char when DMA receiving data.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RX_C0_ESC_EN_OFFSET: u32 = 4;
    pub const RX_C0_ESC_EN_MASK: u32 = 0x00000010;
    pub const RX_C0_ESC_EN: u32 = 1 << 4;

    /// **TX_13_ESC_EN**
    ///
    /// Set this bit to enable resolve flow control char 0x13 when DMA receiving data.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_13_ESC_EN_OFFSET: u32 = 3;
    pub const TX_13_ESC_EN_MASK: u32 = 0x00000008;
    pub const TX_13_ESC_EN: u32 = 1 << 3;

    /// **TX_11_ESC_EN**
    ///
    /// Set this bit to enable resolve flow control char 0x11 when DMA receiving data.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_11_ESC_EN_OFFSET: u32 = 2;
    pub const TX_11_ESC_EN_MASK: u32 = 0x00000004;
    pub const TX_11_ESC_EN: u32 = 1 << 2;

    /// **TX_DB_ESC_EN**
    ///
    /// Set this bit to enable resolve char 0xDB when DMA receiving data.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DB_ESC_EN_OFFSET: u32 = 1;
    pub const TX_DB_ESC_EN_MASK: u32 = 0x00000002;
    pub const TX_DB_ESC_EN: u32 = 1 << 1;

    /// **TX_C0_ESC_EN**
    ///
    /// Set this bit to enable resolve char 0xC0 when DMA receiving data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_C0_ESC_EN_OFFSET: u32 = 0;
    pub const TX_C0_ESC_EN_MASK: u32 = 0x00000001;
    pub const TX_C0_ESC_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_13_ESC_EN** `[7]` (RW)
    ///
    /// Set this bit to enable replacing 0x13 with special char when DMA receiving data.
    #[inline(always)]
    pub fn rx_13_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_13_ESC_EN_MASK) != 0
    }

    /// Set RX_13_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_13_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_13_ESC_EN_MASK);
    }

    /// Clear RX_13_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_13_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_13_ESC_EN_MASK);
    }

    /// **RX_11_ESC_EN** `[6]` (RW)
    ///
    /// Set this bit to enable replacing 0x11 with special char when DMA receiving data.
    #[inline(always)]
    pub fn rx_11_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_11_ESC_EN_MASK) != 0
    }

    /// Set RX_11_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_11_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_11_ESC_EN_MASK);
    }

    /// Clear RX_11_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_11_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_11_ESC_EN_MASK);
    }

    /// **RX_DB_ESC_EN** `[5]` (RW)
    ///
    /// Set this bit to enable replacing 0xDB with special char when DMA receiving data.
    #[inline(always)]
    pub fn rx_db_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_DB_ESC_EN_MASK) != 0
    }

    /// Set RX_DB_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_db_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_DB_ESC_EN_MASK);
    }

    /// Clear RX_DB_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_db_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_DB_ESC_EN_MASK);
    }

    /// **RX_C0_ESC_EN** `[4]` (RW)
    ///
    /// Set this bit to enable replacing 0xC0 with special char when DMA receiving data.
    #[inline(always)]
    pub fn rx_c0_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_C0_ESC_EN_MASK) != 0
    }

    /// Set RX_C0_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_c0_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_C0_ESC_EN_MASK);
    }

    /// Clear RX_C0_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_c0_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::RX_C0_ESC_EN_MASK);
    }

    /// **TX_13_ESC_EN** `[3]` (RW)
    ///
    /// Set this bit to enable resolve flow control char 0x13 when DMA receiving data.
    #[inline(always)]
    pub fn tx_13_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_13_ESC_EN_MASK) != 0
    }

    /// Set TX_13_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_13_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_13_ESC_EN_MASK);
    }

    /// Clear TX_13_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_13_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_13_ESC_EN_MASK);
    }

    /// **TX_11_ESC_EN** `[2]` (RW)
    ///
    /// Set this bit to enable resolve flow control char 0x11 when DMA receiving data.
    #[inline(always)]
    pub fn tx_11_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_11_ESC_EN_MASK) != 0
    }

    /// Set TX_11_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_11_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_11_ESC_EN_MASK);
    }

    /// Clear TX_11_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_11_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_11_ESC_EN_MASK);
    }

    /// **TX_DB_ESC_EN** `[1]` (RW)
    ///
    /// Set this bit to enable resolve char 0xDB when DMA receiving data.
    #[inline(always)]
    pub fn tx_db_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DB_ESC_EN_MASK) != 0
    }

    /// Set TX_DB_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_db_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_DB_ESC_EN_MASK);
    }

    /// Clear TX_DB_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_db_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_DB_ESC_EN_MASK);
    }

    /// **TX_C0_ESC_EN** `[0]` (RW)
    ///
    /// Set this bit to enable resolve char 0xC0 when DMA receiving data.
    #[inline(always)]
    pub fn tx_c0_esc_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_C0_ESC_EN_MASK) != 0
    }

    /// Set TX_C0_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_c0_esc_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_C0_ESC_EN_MASK);
    }

    /// Clear TX_C0_ESC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_c0_esc_en(&self) {
        self.value.modify_bits(|r| r & !Self::TX_C0_ESC_EN_MASK);
    }
}

/// ESCAPE_CONF value type for batch read/modify/write operations
///
/// Use `escapeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `escapeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EscapeConfVal {
    bits: u32,
}

impl EscapeConfVal {
    /// Create from raw bits
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
        Self { bits: 0x00000033 }
    }

    /// Read **RX_13_ESC_EN** `[7:7]`
    ///
    /// Set this bit to enable replacing 0x13 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn rx_13_esc_en(&self) -> bool {
        (self.bits & EscapeConf::RX_13_ESC_EN_MASK) != 0
    }

    /// Read **RX_11_ESC_EN** `[6:6]`
    ///
    /// Set this bit to enable replacing 0x11 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn rx_11_esc_en(&self) -> bool {
        (self.bits & EscapeConf::RX_11_ESC_EN_MASK) != 0
    }

    /// Read **RX_DB_ESC_EN** `[5:5]`
    ///
    /// Set this bit to enable replacing 0xDB with special char when DMA receiving data.
    #[inline(always)]
    pub const fn rx_db_esc_en(&self) -> bool {
        (self.bits & EscapeConf::RX_DB_ESC_EN_MASK) != 0
    }

    /// Read **RX_C0_ESC_EN** `[4:4]`
    ///
    /// Set this bit to enable replacing 0xC0 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn rx_c0_esc_en(&self) -> bool {
        (self.bits & EscapeConf::RX_C0_ESC_EN_MASK) != 0
    }

    /// Read **TX_13_ESC_EN** `[3:3]`
    ///
    /// Set this bit to enable resolve flow control char 0x13 when DMA receiving data.
    #[inline(always)]
    pub const fn tx_13_esc_en(&self) -> bool {
        (self.bits & EscapeConf::TX_13_ESC_EN_MASK) != 0
    }

    /// Read **TX_11_ESC_EN** `[2:2]`
    ///
    /// Set this bit to enable resolve flow control char 0x11 when DMA receiving data.
    #[inline(always)]
    pub const fn tx_11_esc_en(&self) -> bool {
        (self.bits & EscapeConf::TX_11_ESC_EN_MASK) != 0
    }

    /// Read **TX_DB_ESC_EN** `[1:1]`
    ///
    /// Set this bit to enable resolve char 0xDB when DMA receiving data.
    #[inline(always)]
    pub const fn tx_db_esc_en(&self) -> bool {
        (self.bits & EscapeConf::TX_DB_ESC_EN_MASK) != 0
    }

    /// Read **TX_C0_ESC_EN** `[0:0]`
    ///
    /// Set this bit to enable resolve char 0xC0 when DMA receiving data.
    #[inline(always)]
    pub const fn tx_c0_esc_en(&self) -> bool {
        (self.bits & EscapeConf::TX_C0_ESC_EN_MASK) != 0
    }

    /// Set **RX_13_ESC_EN** `[7:7]`
    ///
    /// Set this bit to enable replacing 0x13 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn with_rx_13_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::RX_13_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::RX_13_ESC_EN_MASK
            },
        }
    }

    /// Set **RX_11_ESC_EN** `[6:6]`
    ///
    /// Set this bit to enable replacing 0x11 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn with_rx_11_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::RX_11_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::RX_11_ESC_EN_MASK
            },
        }
    }

    /// Set **RX_DB_ESC_EN** `[5:5]`
    ///
    /// Set this bit to enable replacing 0xDB with special char when DMA receiving data.
    #[inline(always)]
    pub const fn with_rx_db_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::RX_DB_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::RX_DB_ESC_EN_MASK
            },
        }
    }

    /// Set **RX_C0_ESC_EN** `[4:4]`
    ///
    /// Set this bit to enable replacing 0xC0 with special char when DMA receiving data.
    #[inline(always)]
    pub const fn with_rx_c0_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::RX_C0_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::RX_C0_ESC_EN_MASK
            },
        }
    }

    /// Set **TX_13_ESC_EN** `[3:3]`
    ///
    /// Set this bit to enable resolve flow control char 0x13 when DMA receiving data.
    #[inline(always)]
    pub const fn with_tx_13_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::TX_13_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::TX_13_ESC_EN_MASK
            },
        }
    }

    /// Set **TX_11_ESC_EN** `[2:2]`
    ///
    /// Set this bit to enable resolve flow control char 0x11 when DMA receiving data.
    #[inline(always)]
    pub const fn with_tx_11_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::TX_11_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::TX_11_ESC_EN_MASK
            },
        }
    }

    /// Set **TX_DB_ESC_EN** `[1:1]`
    ///
    /// Set this bit to enable resolve char 0xDB when DMA receiving data.
    #[inline(always)]
    pub const fn with_tx_db_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::TX_DB_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::TX_DB_ESC_EN_MASK
            },
        }
    }

    /// Set **TX_C0_ESC_EN** `[0:0]`
    ///
    /// Set this bit to enable resolve char 0xC0 when DMA receiving data.
    #[inline(always)]
    pub const fn with_tx_c0_esc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EscapeConf::TX_C0_ESC_EN_MASK
            } else {
                self.bits & !EscapeConf::TX_C0_ESC_EN_MASK
            },
        }
    }
}

impl Default for EscapeConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EscapeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EscapeConfVal {
        EscapeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EscapeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EscapeConfVal) -> EscapeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HUNG_CONF Register
///
/// UHCI Hung Configuration Register0
///
/// Address offset: 0x0024
/// Reset value: 0x00810810
///
/// ## Fields
///
/// - **RXFIFO_TIMEOUT_ENA** `[23]` (RW)
///
///   Set this bit to enable TX FIFO timeout when DMA sending data.
///
/// - **RXFIFO_TIMEOUT_SHIFT** `[22:20]` (RW)
///
///   Configures the maximum counter value.
///
/// - **RXFIFO_TIMEOUT** `[19:12]` (RW)
///
///   Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
///
/// - **TXFIFO_TIMEOUT_ENA** `[11]` (RW)
///
///   Set this bit to enable TX FIFO timeout when receiving.
///
/// - **TXFIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
///
///   Configures the maximum counter value.
///
/// - **TXFIFO_TIMEOUT** `[7:0]` (RW)
///
///   Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
#[repr(transparent)]
pub struct HungConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HungConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HungConf {
    // ======== Field Constants ========

    /// **RXFIFO_TIMEOUT_ENA**
    ///
    /// Set this bit to enable TX FIFO timeout when DMA sending data.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RXFIFO_TIMEOUT_ENA_OFFSET: u32 = 23;
    pub const RXFIFO_TIMEOUT_ENA_MASK: u32 = 0x00800000;
    pub const RXFIFO_TIMEOUT_ENA: u32 = 1 << 23;

    /// **RXFIFO_TIMEOUT_SHIFT**
    ///
    /// Configures the maximum counter value.
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const RXFIFO_TIMEOUT_SHIFT_OFFSET: u32 = 20;
    pub const RXFIFO_TIMEOUT_SHIFT_MASK: u32 = 0x00700000;

    /// **RXFIFO_TIMEOUT**
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
    ///
    /// Bits `[19:12]`, mask `0x000ff000`
    pub const RXFIFO_TIMEOUT_OFFSET: u32 = 12;
    pub const RXFIFO_TIMEOUT_MASK: u32 = 0x000ff000;

    /// **TXFIFO_TIMEOUT_ENA**
    ///
    /// Set this bit to enable TX FIFO timeout when receiving.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXFIFO_TIMEOUT_ENA_OFFSET: u32 = 11;
    pub const TXFIFO_TIMEOUT_ENA_MASK: u32 = 0x00000800;
    pub const TXFIFO_TIMEOUT_ENA: u32 = 1 << 11;

    /// **TXFIFO_TIMEOUT_SHIFT**
    ///
    /// Configures the maximum counter value.
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const TXFIFO_TIMEOUT_SHIFT_OFFSET: u32 = 8;
    pub const TXFIFO_TIMEOUT_SHIFT_MASK: u32 = 0x00000700;

    /// **TXFIFO_TIMEOUT**
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TXFIFO_TIMEOUT_OFFSET: u32 = 0;
    pub const TXFIFO_TIMEOUT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RXFIFO_TIMEOUT_ENA** `[23]` (RW)
    ///
    /// Set this bit to enable TX FIFO timeout when DMA sending data.
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Set RXFIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_TIMEOUT_ENA_MASK);
    }

    /// Clear RXFIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_TIMEOUT_ENA_MASK);
    }

    /// **RXFIFO_TIMEOUT_SHIFT** `[22:20]` (RW)
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_TIMEOUT_SHIFT_MASK)
            >> Self::RXFIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Write **RXFIFO_TIMEOUT_SHIFT** `[22:20]` (RW)
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub fn write_rxfifo_timeout_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXFIFO_TIMEOUT_SHIFT_MASK)
                | ((value << Self::RXFIFO_TIMEOUT_SHIFT_OFFSET) & Self::RXFIFO_TIMEOUT_SHIFT_MASK)
        });
    }

    /// **RXFIFO_TIMEOUT** `[19:12]` (RW)
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_TIMEOUT_MASK) >> Self::RXFIFO_TIMEOUT_OFFSET
    }

    /// Write **RXFIFO_TIMEOUT** `[19:12]` (RW)
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
    #[inline(always)]
    pub fn write_rxfifo_timeout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXFIFO_TIMEOUT_MASK)
                | ((value << Self::RXFIFO_TIMEOUT_OFFSET) & Self::RXFIFO_TIMEOUT_MASK)
        });
    }

    /// **TXFIFO_TIMEOUT_ENA** `[11]` (RW)
    ///
    /// Set this bit to enable TX FIFO timeout when receiving.
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Set TXFIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TXFIFO_TIMEOUT_ENA_MASK);
    }

    /// Clear TXFIFO_TIMEOUT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_timeout_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TXFIFO_TIMEOUT_ENA_MASK);
    }

    /// **TXFIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_TIMEOUT_SHIFT_MASK)
            >> Self::TXFIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Write **TXFIFO_TIMEOUT_SHIFT** `[10:8]` (RW)
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub fn write_txfifo_timeout_shift(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXFIFO_TIMEOUT_SHIFT_MASK)
                | ((value << Self::TXFIFO_TIMEOUT_SHIFT_OFFSET) & Self::TXFIFO_TIMEOUT_SHIFT_MASK)
        });
    }

    /// **TXFIFO_TIMEOUT** `[7:0]` (RW)
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_TIMEOUT_MASK) >> Self::TXFIFO_TIMEOUT_OFFSET
    }

    /// Write **TXFIFO_TIMEOUT** `[7:0]` (RW)
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
    #[inline(always)]
    pub fn write_txfifo_timeout(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXFIFO_TIMEOUT_MASK)
                | ((value << Self::TXFIFO_TIMEOUT_OFFSET) & Self::TXFIFO_TIMEOUT_MASK)
        });
    }
}

/// HUNG_CONF value type for batch read/modify/write operations
///
/// Use `hungconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hungconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HungConfVal {
    bits: u32,
}

impl HungConfVal {
    /// Create from raw bits
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
        Self { bits: 0x00810810 }
    }

    /// Read **RXFIFO_TIMEOUT_ENA** `[23:23]`
    ///
    /// Set this bit to enable TX FIFO timeout when DMA sending data.
    #[inline(always)]
    pub const fn rxfifo_timeout_ena(&self) -> bool {
        (self.bits & HungConf::RXFIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_TIMEOUT_SHIFT** `[22:20]`
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub const fn rxfifo_timeout_shift(&self) -> u32 {
        (self.bits & HungConf::RXFIFO_TIMEOUT_SHIFT_MASK) >> HungConf::RXFIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Read **RXFIFO_TIMEOUT** `[19:12]`
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
    #[inline(always)]
    pub const fn rxfifo_timeout(&self) -> u32 {
        (self.bits & HungConf::RXFIFO_TIMEOUT_MASK) >> HungConf::RXFIFO_TIMEOUT_OFFSET
    }

    /// Read **TXFIFO_TIMEOUT_ENA** `[11:11]`
    ///
    /// Set this bit to enable TX FIFO timeout when receiving.
    #[inline(always)]
    pub const fn txfifo_timeout_ena(&self) -> bool {
        (self.bits & HungConf::TXFIFO_TIMEOUT_ENA_MASK) != 0
    }

    /// Read **TXFIFO_TIMEOUT_SHIFT** `[10:8]`
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub const fn txfifo_timeout_shift(&self) -> u32 {
        (self.bits & HungConf::TXFIFO_TIMEOUT_SHIFT_MASK) >> HungConf::TXFIFO_TIMEOUT_SHIFT_OFFSET
    }

    /// Read **TXFIFO_TIMEOUT** `[7:0]`
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
    #[inline(always)]
    pub const fn txfifo_timeout(&self) -> u32 {
        (self.bits & HungConf::TXFIFO_TIMEOUT_MASK) >> HungConf::TXFIFO_TIMEOUT_OFFSET
    }

    /// Set **RXFIFO_TIMEOUT_ENA** `[23:23]`
    ///
    /// Set this bit to enable TX FIFO timeout when DMA sending data.
    #[inline(always)]
    pub const fn with_rxfifo_timeout_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HungConf::RXFIFO_TIMEOUT_ENA_MASK
            } else {
                self.bits & !HungConf::RXFIFO_TIMEOUT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_TIMEOUT_SHIFT** `[22:20]`
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub const fn with_rxfifo_timeout_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HungConf::RXFIFO_TIMEOUT_SHIFT_MASK)
                | ((value << HungConf::RXFIFO_TIMEOUT_SHIFT_OFFSET)
                    & HungConf::RXFIFO_TIMEOUT_SHIFT_MASK),
        }
    }

    /// Set **RXFIFO_TIMEOUT** `[19:12]`
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data.
    #[inline(always)]
    pub const fn with_rxfifo_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HungConf::RXFIFO_TIMEOUT_MASK)
                | ((value << HungConf::RXFIFO_TIMEOUT_OFFSET) & HungConf::RXFIFO_TIMEOUT_MASK),
        }
    }

    /// Set **TXFIFO_TIMEOUT_ENA** `[11:11]`
    ///
    /// Set this bit to enable TX FIFO timeout when receiving.
    #[inline(always)]
    pub const fn with_txfifo_timeout_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HungConf::TXFIFO_TIMEOUT_ENA_MASK
            } else {
                self.bits & !HungConf::TXFIFO_TIMEOUT_ENA_MASK
            },
        }
    }

    /// Set **TXFIFO_TIMEOUT_SHIFT** `[10:8]`
    ///
    /// Configures the maximum counter value.
    #[inline(always)]
    pub const fn with_txfifo_timeout_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HungConf::TXFIFO_TIMEOUT_SHIFT_MASK)
                | ((value << HungConf::TXFIFO_TIMEOUT_SHIFT_OFFSET)
                    & HungConf::TXFIFO_TIMEOUT_SHIFT_MASK),
        }
    }

    /// Set **TXFIFO_TIMEOUT** `[7:0]`
    ///
    /// Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data.
    #[inline(always)]
    pub const fn with_txfifo_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HungConf::TXFIFO_TIMEOUT_MASK)
                | ((value << HungConf::TXFIFO_TIMEOUT_OFFSET) & HungConf::TXFIFO_TIMEOUT_MASK),
        }
    }
}

impl Default for HungConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HungConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HungConfVal {
        HungConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HungConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HungConfVal) -> HungConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ACK_NUM Register
///
/// UHCI Ack Value Configuration Register0
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **LOAD** `[3]` (WO)
///
///   Set this bit to load the ACK value of UHCI_ACK_NUM.
///
/// - **ACK_NUM** `[2:0]` (RW)
///
///   Indicates the ACK number during software flow control.
#[repr(transparent)]
pub struct AckNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AckNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AckNum {
    // ======== Field Constants ========

    /// **LOAD**
    ///
    /// Set this bit to load the ACK value of UHCI_ACK_NUM.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LOAD_OFFSET: u32 = 3;
    pub const LOAD_MASK: u32 = 0x00000008;
    pub const LOAD: u32 = 1 << 3;

    /// **ACK_NUM**
    ///
    /// Indicates the ACK number during software flow control.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const ACK_NUM_OFFSET: u32 = 0;
    pub const ACK_NUM_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// Trigger **LOAD** (WO)
    ///
    /// Set this bit to load the ACK value of UHCI_ACK_NUM.
    #[inline(always)]
    pub fn trigger_load(&self) {
        self.value.modify_bits(|r| r | Self::LOAD_MASK);
    }

    /// **ACK_NUM** `[2:0]` (RW)
    ///
    /// Indicates the ACK number during software flow control.
    #[inline(always)]
    pub fn ack_num(&self) -> u32 {
        (self.value.read_bits() & Self::ACK_NUM_MASK) >> Self::ACK_NUM_OFFSET
    }

    /// Write **ACK_NUM** `[2:0]` (RW)
    ///
    /// Indicates the ACK number during software flow control.
    #[inline(always)]
    pub fn write_ack_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ACK_NUM_MASK) | ((value << Self::ACK_NUM_OFFSET) & Self::ACK_NUM_MASK)
        });
    }
}

/// ACK_NUM value type for batch read/modify/write operations
///
/// Use `acknum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `acknum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AckNumVal {
    bits: u32,
}

impl AckNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ACK_NUM** `[2:0]`
    ///
    /// Indicates the ACK number during software flow control.
    #[inline(always)]
    pub const fn ack_num(&self) -> u32 {
        (self.bits & AckNum::ACK_NUM_MASK) >> AckNum::ACK_NUM_OFFSET
    }

    /// Set **LOAD** `[3:3]`
    ///
    /// Set this bit to load the ACK value of UHCI_ACK_NUM.
    #[inline(always)]
    pub const fn with_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AckNum::LOAD_MASK
            } else {
                self.bits & !AckNum::LOAD_MASK
            },
        }
    }

    /// Set **ACK_NUM** `[2:0]`
    ///
    /// Indicates the ACK number during software flow control.
    #[inline(always)]
    pub const fn with_ack_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AckNum::ACK_NUM_MASK)
                | ((value << AckNum::ACK_NUM_OFFSET) & AckNum::ACK_NUM_MASK),
        }
    }
}

impl Default for AckNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AckNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AckNumVal {
        AckNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AckNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AckNumVal) -> AckNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_HEAD Register
///
/// UHCI Head Register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **RX_HEAD** `[31:0]` (RO)
///
///   Stores the head of received packet.
#[repr(transparent)]
pub struct RxHead {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxHead {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxHead {
    // ======== Field Constants ========

    /// **RX_HEAD**
    ///
    /// Stores the head of received packet.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_HEAD_OFFSET: u32 = 0;
    pub const RX_HEAD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_HEAD** `[31:0]` (RO)
    ///
    /// Stores the head of received packet.
    #[inline(always)]
    pub fn rx_head(&self) -> u32 {
        (self.value.read_bits() & Self::RX_HEAD_MASK) >> Self::RX_HEAD_OFFSET
    }
}

/// RX_HEAD value type for batch read/modify/write operations
///
/// Use `rxhead.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxhead.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxHeadVal {
    bits: u32,
}

impl RxHeadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_HEAD** `[31:0]`
    ///
    /// Stores the head of received packet.
    #[inline(always)]
    pub const fn rx_head(&self) -> u32 {
        (self.bits & RxHead::RX_HEAD_MASK) >> RxHead::RX_HEAD_OFFSET
    }
}

impl Default for RxHeadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxHead {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxHeadVal {
        RxHeadVal::from_bits(self.value.read_bits())
    }
}

/// QUICK_SENT Register
///
/// UCHI Quick send Register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **ALWAYS_SEND_EN** `[7]` (RW)
///
///   Set this bit to enable sending short packet with always_send mode.
///
/// - **ALWAYS_SEND_NUM** `[6:4]` (RW)
///
///   Configures always_send mode.
///
/// - **SINGLE_SEND_EN** `[3]` (WO)
///
///   Set this bit to enable sending short packet with single_send mode.
///
/// - **SINGLE_SEND_NUM** `[2:0]` (RW)
///
///   Configures single_send mode.
#[repr(transparent)]
pub struct QuickSent {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QuickSent {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QuickSent {
    // ======== Field Constants ========

    /// **ALWAYS_SEND_EN**
    ///
    /// Set this bit to enable sending short packet with always_send mode.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ALWAYS_SEND_EN_OFFSET: u32 = 7;
    pub const ALWAYS_SEND_EN_MASK: u32 = 0x00000080;
    pub const ALWAYS_SEND_EN: u32 = 1 << 7;

    /// **ALWAYS_SEND_NUM**
    ///
    /// Configures always_send mode.
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const ALWAYS_SEND_NUM_OFFSET: u32 = 4;
    pub const ALWAYS_SEND_NUM_MASK: u32 = 0x00000070;

    /// **SINGLE_SEND_EN**
    ///
    /// Set this bit to enable sending short packet with single_send mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SINGLE_SEND_EN_OFFSET: u32 = 3;
    pub const SINGLE_SEND_EN_MASK: u32 = 0x00000008;
    pub const SINGLE_SEND_EN: u32 = 1 << 3;

    /// **SINGLE_SEND_NUM**
    ///
    /// Configures single_send mode.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SINGLE_SEND_NUM_OFFSET: u32 = 0;
    pub const SINGLE_SEND_NUM_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **ALWAYS_SEND_EN** `[7]` (RW)
    ///
    /// Set this bit to enable sending short packet with always_send mode.
    #[inline(always)]
    pub fn always_send_en(&self) -> bool {
        (self.value.read_bits() & Self::ALWAYS_SEND_EN_MASK) != 0
    }

    /// Set ALWAYS_SEND_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_always_send_en(&self) {
        self.value.modify_bits(|r| r | Self::ALWAYS_SEND_EN_MASK);
    }

    /// Clear ALWAYS_SEND_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_always_send_en(&self) {
        self.value.modify_bits(|r| r & !Self::ALWAYS_SEND_EN_MASK);
    }

    /// **ALWAYS_SEND_NUM** `[6:4]` (RW)
    ///
    /// Configures always_send mode.
    #[inline(always)]
    pub fn always_send_num(&self) -> u32 {
        (self.value.read_bits() & Self::ALWAYS_SEND_NUM_MASK) >> Self::ALWAYS_SEND_NUM_OFFSET
    }

    /// Write **ALWAYS_SEND_NUM** `[6:4]` (RW)
    ///
    /// Configures always_send mode.
    #[inline(always)]
    pub fn write_always_send_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALWAYS_SEND_NUM_MASK)
                | ((value << Self::ALWAYS_SEND_NUM_OFFSET) & Self::ALWAYS_SEND_NUM_MASK)
        });
    }

    /// Trigger **SINGLE_SEND_EN** (WO)
    ///
    /// Set this bit to enable sending short packet with single_send mode.
    #[inline(always)]
    pub fn trigger_single_send_en(&self) {
        self.value.modify_bits(|r| r | Self::SINGLE_SEND_EN_MASK);
    }

    /// **SINGLE_SEND_NUM** `[2:0]` (RW)
    ///
    /// Configures single_send mode.
    #[inline(always)]
    pub fn single_send_num(&self) -> u32 {
        (self.value.read_bits() & Self::SINGLE_SEND_NUM_MASK) >> Self::SINGLE_SEND_NUM_OFFSET
    }

    /// Write **SINGLE_SEND_NUM** `[2:0]` (RW)
    ///
    /// Configures single_send mode.
    #[inline(always)]
    pub fn write_single_send_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SINGLE_SEND_NUM_MASK)
                | ((value << Self::SINGLE_SEND_NUM_OFFSET) & Self::SINGLE_SEND_NUM_MASK)
        });
    }
}

/// QUICK_SENT value type for batch read/modify/write operations
///
/// Use `quicksent.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `quicksent.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QuickSentVal {
    bits: u32,
}

impl QuickSentVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALWAYS_SEND_EN** `[7:7]`
    ///
    /// Set this bit to enable sending short packet with always_send mode.
    #[inline(always)]
    pub const fn always_send_en(&self) -> bool {
        (self.bits & QuickSent::ALWAYS_SEND_EN_MASK) != 0
    }

    /// Read **ALWAYS_SEND_NUM** `[6:4]`
    ///
    /// Configures always_send mode.
    #[inline(always)]
    pub const fn always_send_num(&self) -> u32 {
        (self.bits & QuickSent::ALWAYS_SEND_NUM_MASK) >> QuickSent::ALWAYS_SEND_NUM_OFFSET
    }

    /// Read **SINGLE_SEND_NUM** `[2:0]`
    ///
    /// Configures single_send mode.
    #[inline(always)]
    pub const fn single_send_num(&self) -> u32 {
        (self.bits & QuickSent::SINGLE_SEND_NUM_MASK) >> QuickSent::SINGLE_SEND_NUM_OFFSET
    }

    /// Set **ALWAYS_SEND_EN** `[7:7]`
    ///
    /// Set this bit to enable sending short packet with always_send mode.
    #[inline(always)]
    pub const fn with_always_send_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | QuickSent::ALWAYS_SEND_EN_MASK
            } else {
                self.bits & !QuickSent::ALWAYS_SEND_EN_MASK
            },
        }
    }

    /// Set **ALWAYS_SEND_NUM** `[6:4]`
    ///
    /// Configures always_send mode.
    #[inline(always)]
    pub const fn with_always_send_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !QuickSent::ALWAYS_SEND_NUM_MASK)
                | ((value << QuickSent::ALWAYS_SEND_NUM_OFFSET) & QuickSent::ALWAYS_SEND_NUM_MASK),
        }
    }

    /// Set **SINGLE_SEND_EN** `[3:3]`
    ///
    /// Set this bit to enable sending short packet with single_send mode.
    #[inline(always)]
    pub const fn with_single_send_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | QuickSent::SINGLE_SEND_EN_MASK
            } else {
                self.bits & !QuickSent::SINGLE_SEND_EN_MASK
            },
        }
    }

    /// Set **SINGLE_SEND_NUM** `[2:0]`
    ///
    /// Configures single_send mode.
    #[inline(always)]
    pub const fn with_single_send_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !QuickSent::SINGLE_SEND_NUM_MASK)
                | ((value << QuickSent::SINGLE_SEND_NUM_OFFSET) & QuickSent::SINGLE_SEND_NUM_MASK),
        }
    }
}

impl Default for QuickSentVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QuickSent {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QuickSentVal {
        QuickSentVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: QuickSentVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(QuickSentVal) -> QuickSentVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q0_WORD0 Register
///
/// UHCI Q0_WORD0 Quick Send Register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SEND_Q0_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ0Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ0Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ0Word0 {
    // ======== Field Constants ========

    /// **SEND_Q0_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q0_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q0_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q0_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q0_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q0_WORD0_MASK) >> Self::SEND_Q0_WORD0_OFFSET
    }

    /// Write **SEND_Q0_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q0_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q0_WORD0_MASK)
                | ((value << Self::SEND_Q0_WORD0_OFFSET) & Self::SEND_Q0_WORD0_MASK)
        });
    }
}

/// REG_Q0_WORD0 value type for batch read/modify/write operations
///
/// Use `regq0word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq0word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ0Word0Val {
    bits: u32,
}

impl RegQ0Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q0_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q0_word0(&self) -> u32 {
        (self.bits & RegQ0Word0::SEND_Q0_WORD0_MASK) >> RegQ0Word0::SEND_Q0_WORD0_OFFSET
    }

    /// Set **SEND_Q0_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q0_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ0Word0::SEND_Q0_WORD0_MASK)
                | ((value << RegQ0Word0::SEND_Q0_WORD0_OFFSET) & RegQ0Word0::SEND_Q0_WORD0_MASK),
        }
    }
}

impl Default for RegQ0Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ0Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ0Word0Val {
        RegQ0Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ0Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ0Word0Val) -> RegQ0Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q0_WORD1 Register
///
/// UHCI Q0_WORD1 Quick Send Register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **SEND_Q0_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ0Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ0Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ0Word1 {
    // ======== Field Constants ========

    /// **SEND_Q0_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q0_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q0_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q0_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q0_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q0_WORD1_MASK) >> Self::SEND_Q0_WORD1_OFFSET
    }

    /// Write **SEND_Q0_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q0_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q0_WORD1_MASK)
                | ((value << Self::SEND_Q0_WORD1_OFFSET) & Self::SEND_Q0_WORD1_MASK)
        });
    }
}

/// REG_Q0_WORD1 value type for batch read/modify/write operations
///
/// Use `regq0word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq0word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ0Word1Val {
    bits: u32,
}

impl RegQ0Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q0_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q0_word1(&self) -> u32 {
        (self.bits & RegQ0Word1::SEND_Q0_WORD1_MASK) >> RegQ0Word1::SEND_Q0_WORD1_OFFSET
    }

    /// Set **SEND_Q0_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q0_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ0Word1::SEND_Q0_WORD1_MASK)
                | ((value << RegQ0Word1::SEND_Q0_WORD1_OFFSET) & RegQ0Word1::SEND_Q0_WORD1_MASK),
        }
    }
}

impl Default for RegQ0Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ0Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ0Word1Val {
        RegQ0Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ0Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ0Word1Val) -> RegQ0Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q1_WORD0 Register
///
/// UHCI Q1_WORD0 Quick Send Register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **SEND_Q1_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ1Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ1Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ1Word0 {
    // ======== Field Constants ========

    /// **SEND_Q1_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q1_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q1_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q1_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q1_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q1_WORD0_MASK) >> Self::SEND_Q1_WORD0_OFFSET
    }

    /// Write **SEND_Q1_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q1_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q1_WORD0_MASK)
                | ((value << Self::SEND_Q1_WORD0_OFFSET) & Self::SEND_Q1_WORD0_MASK)
        });
    }
}

/// REG_Q1_WORD0 value type for batch read/modify/write operations
///
/// Use `regq1word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq1word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ1Word0Val {
    bits: u32,
}

impl RegQ1Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q1_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q1_word0(&self) -> u32 {
        (self.bits & RegQ1Word0::SEND_Q1_WORD0_MASK) >> RegQ1Word0::SEND_Q1_WORD0_OFFSET
    }

    /// Set **SEND_Q1_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q1_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ1Word0::SEND_Q1_WORD0_MASK)
                | ((value << RegQ1Word0::SEND_Q1_WORD0_OFFSET) & RegQ1Word0::SEND_Q1_WORD0_MASK),
        }
    }
}

impl Default for RegQ1Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ1Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ1Word0Val {
        RegQ1Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ1Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ1Word0Val) -> RegQ1Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q1_WORD1 Register
///
/// UHCI Q1_WORD1 Quick Send Register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **SEND_Q1_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ1Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ1Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ1Word1 {
    // ======== Field Constants ========

    /// **SEND_Q1_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q1_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q1_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q1_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q1_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q1_WORD1_MASK) >> Self::SEND_Q1_WORD1_OFFSET
    }

    /// Write **SEND_Q1_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q1_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q1_WORD1_MASK)
                | ((value << Self::SEND_Q1_WORD1_OFFSET) & Self::SEND_Q1_WORD1_MASK)
        });
    }
}

/// REG_Q1_WORD1 value type for batch read/modify/write operations
///
/// Use `regq1word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq1word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ1Word1Val {
    bits: u32,
}

impl RegQ1Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q1_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q1_word1(&self) -> u32 {
        (self.bits & RegQ1Word1::SEND_Q1_WORD1_MASK) >> RegQ1Word1::SEND_Q1_WORD1_OFFSET
    }

    /// Set **SEND_Q1_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q1_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ1Word1::SEND_Q1_WORD1_MASK)
                | ((value << RegQ1Word1::SEND_Q1_WORD1_OFFSET) & RegQ1Word1::SEND_Q1_WORD1_MASK),
        }
    }
}

impl Default for RegQ1Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ1Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ1Word1Val {
        RegQ1Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ1Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ1Word1Val) -> RegQ1Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q2_WORD0 Register
///
/// UHCI Q2_WORD0 Quick Send Register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **SEND_Q2_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ2Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ2Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ2Word0 {
    // ======== Field Constants ========

    /// **SEND_Q2_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q2_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q2_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q2_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q2_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q2_WORD0_MASK) >> Self::SEND_Q2_WORD0_OFFSET
    }

    /// Write **SEND_Q2_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q2_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q2_WORD0_MASK)
                | ((value << Self::SEND_Q2_WORD0_OFFSET) & Self::SEND_Q2_WORD0_MASK)
        });
    }
}

/// REG_Q2_WORD0 value type for batch read/modify/write operations
///
/// Use `regq2word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq2word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ2Word0Val {
    bits: u32,
}

impl RegQ2Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q2_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q2_word0(&self) -> u32 {
        (self.bits & RegQ2Word0::SEND_Q2_WORD0_MASK) >> RegQ2Word0::SEND_Q2_WORD0_OFFSET
    }

    /// Set **SEND_Q2_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q2_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ2Word0::SEND_Q2_WORD0_MASK)
                | ((value << RegQ2Word0::SEND_Q2_WORD0_OFFSET) & RegQ2Word0::SEND_Q2_WORD0_MASK),
        }
    }
}

impl Default for RegQ2Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ2Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ2Word0Val {
        RegQ2Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ2Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ2Word0Val) -> RegQ2Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q2_WORD1 Register
///
/// UHCI Q2_WORD1 Quick Send Register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **SEND_Q2_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ2Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ2Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ2Word1 {
    // ======== Field Constants ========

    /// **SEND_Q2_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q2_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q2_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q2_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q2_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q2_WORD1_MASK) >> Self::SEND_Q2_WORD1_OFFSET
    }

    /// Write **SEND_Q2_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q2_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q2_WORD1_MASK)
                | ((value << Self::SEND_Q2_WORD1_OFFSET) & Self::SEND_Q2_WORD1_MASK)
        });
    }
}

/// REG_Q2_WORD1 value type for batch read/modify/write operations
///
/// Use `regq2word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq2word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ2Word1Val {
    bits: u32,
}

impl RegQ2Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q2_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q2_word1(&self) -> u32 {
        (self.bits & RegQ2Word1::SEND_Q2_WORD1_MASK) >> RegQ2Word1::SEND_Q2_WORD1_OFFSET
    }

    /// Set **SEND_Q2_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q2_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ2Word1::SEND_Q2_WORD1_MASK)
                | ((value << RegQ2Word1::SEND_Q2_WORD1_OFFSET) & RegQ2Word1::SEND_Q2_WORD1_MASK),
        }
    }
}

impl Default for RegQ2Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ2Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ2Word1Val {
        RegQ2Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ2Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ2Word1Val) -> RegQ2Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q3_WORD0 Register
///
/// UHCI Q3_WORD0 Quick Send Register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **SEND_Q3_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ3Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ3Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ3Word0 {
    // ======== Field Constants ========

    /// **SEND_Q3_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q3_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q3_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q3_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q3_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q3_WORD0_MASK) >> Self::SEND_Q3_WORD0_OFFSET
    }

    /// Write **SEND_Q3_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q3_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q3_WORD0_MASK)
                | ((value << Self::SEND_Q3_WORD0_OFFSET) & Self::SEND_Q3_WORD0_MASK)
        });
    }
}

/// REG_Q3_WORD0 value type for batch read/modify/write operations
///
/// Use `regq3word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq3word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ3Word0Val {
    bits: u32,
}

impl RegQ3Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q3_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q3_word0(&self) -> u32 {
        (self.bits & RegQ3Word0::SEND_Q3_WORD0_MASK) >> RegQ3Word0::SEND_Q3_WORD0_OFFSET
    }

    /// Set **SEND_Q3_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q3_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ3Word0::SEND_Q3_WORD0_MASK)
                | ((value << RegQ3Word0::SEND_Q3_WORD0_OFFSET) & RegQ3Word0::SEND_Q3_WORD0_MASK),
        }
    }
}

impl Default for RegQ3Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ3Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ3Word0Val {
        RegQ3Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ3Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ3Word0Val) -> RegQ3Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q3_WORD1 Register
///
/// UHCI Q3_WORD1 Quick Send Register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **SEND_Q3_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ3Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ3Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ3Word1 {
    // ======== Field Constants ========

    /// **SEND_Q3_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q3_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q3_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q3_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q3_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q3_WORD1_MASK) >> Self::SEND_Q3_WORD1_OFFSET
    }

    /// Write **SEND_Q3_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q3_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q3_WORD1_MASK)
                | ((value << Self::SEND_Q3_WORD1_OFFSET) & Self::SEND_Q3_WORD1_MASK)
        });
    }
}

/// REG_Q3_WORD1 value type for batch read/modify/write operations
///
/// Use `regq3word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq3word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ3Word1Val {
    bits: u32,
}

impl RegQ3Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q3_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q3_word1(&self) -> u32 {
        (self.bits & RegQ3Word1::SEND_Q3_WORD1_MASK) >> RegQ3Word1::SEND_Q3_WORD1_OFFSET
    }

    /// Set **SEND_Q3_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q3_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ3Word1::SEND_Q3_WORD1_MASK)
                | ((value << RegQ3Word1::SEND_Q3_WORD1_OFFSET) & RegQ3Word1::SEND_Q3_WORD1_MASK),
        }
    }
}

impl Default for RegQ3Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ3Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ3Word1Val {
        RegQ3Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ3Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ3Word1Val) -> RegQ3Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q4_WORD0 Register
///
/// UHCI Q4_WORD0 Quick Send Register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **SEND_Q4_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ4Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ4Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ4Word0 {
    // ======== Field Constants ========

    /// **SEND_Q4_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q4_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q4_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q4_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q4_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q4_WORD0_MASK) >> Self::SEND_Q4_WORD0_OFFSET
    }

    /// Write **SEND_Q4_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q4_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q4_WORD0_MASK)
                | ((value << Self::SEND_Q4_WORD0_OFFSET) & Self::SEND_Q4_WORD0_MASK)
        });
    }
}

/// REG_Q4_WORD0 value type for batch read/modify/write operations
///
/// Use `regq4word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq4word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ4Word0Val {
    bits: u32,
}

impl RegQ4Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q4_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q4_word0(&self) -> u32 {
        (self.bits & RegQ4Word0::SEND_Q4_WORD0_MASK) >> RegQ4Word0::SEND_Q4_WORD0_OFFSET
    }

    /// Set **SEND_Q4_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q4_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ4Word0::SEND_Q4_WORD0_MASK)
                | ((value << RegQ4Word0::SEND_Q4_WORD0_OFFSET) & RegQ4Word0::SEND_Q4_WORD0_MASK),
        }
    }
}

impl Default for RegQ4Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ4Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ4Word0Val {
        RegQ4Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ4Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ4Word0Val) -> RegQ4Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q4_WORD1 Register
///
/// UHCI Q4_WORD1 Quick Send Register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **SEND_Q4_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ4Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ4Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ4Word1 {
    // ======== Field Constants ========

    /// **SEND_Q4_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q4_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q4_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q4_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q4_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q4_WORD1_MASK) >> Self::SEND_Q4_WORD1_OFFSET
    }

    /// Write **SEND_Q4_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q4_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q4_WORD1_MASK)
                | ((value << Self::SEND_Q4_WORD1_OFFSET) & Self::SEND_Q4_WORD1_MASK)
        });
    }
}

/// REG_Q4_WORD1 value type for batch read/modify/write operations
///
/// Use `regq4word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq4word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ4Word1Val {
    bits: u32,
}

impl RegQ4Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q4_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q4_word1(&self) -> u32 {
        (self.bits & RegQ4Word1::SEND_Q4_WORD1_MASK) >> RegQ4Word1::SEND_Q4_WORD1_OFFSET
    }

    /// Set **SEND_Q4_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q4_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ4Word1::SEND_Q4_WORD1_MASK)
                | ((value << RegQ4Word1::SEND_Q4_WORD1_OFFSET) & RegQ4Word1::SEND_Q4_WORD1_MASK),
        }
    }
}

impl Default for RegQ4Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ4Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ4Word1Val {
        RegQ4Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ4Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ4Word1Val) -> RegQ4Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q5_WORD0 Register
///
/// UHCI Q5_WORD0 Quick Send Register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **SEND_Q5_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ5Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ5Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ5Word0 {
    // ======== Field Constants ========

    /// **SEND_Q5_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q5_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q5_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q5_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q5_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q5_WORD0_MASK) >> Self::SEND_Q5_WORD0_OFFSET
    }

    /// Write **SEND_Q5_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q5_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q5_WORD0_MASK)
                | ((value << Self::SEND_Q5_WORD0_OFFSET) & Self::SEND_Q5_WORD0_MASK)
        });
    }
}

/// REG_Q5_WORD0 value type for batch read/modify/write operations
///
/// Use `regq5word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq5word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ5Word0Val {
    bits: u32,
}

impl RegQ5Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q5_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q5_word0(&self) -> u32 {
        (self.bits & RegQ5Word0::SEND_Q5_WORD0_MASK) >> RegQ5Word0::SEND_Q5_WORD0_OFFSET
    }

    /// Set **SEND_Q5_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q5_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ5Word0::SEND_Q5_WORD0_MASK)
                | ((value << RegQ5Word0::SEND_Q5_WORD0_OFFSET) & RegQ5Word0::SEND_Q5_WORD0_MASK),
        }
    }
}

impl Default for RegQ5Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ5Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ5Word0Val {
        RegQ5Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ5Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ5Word0Val) -> RegQ5Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q5_WORD1 Register
///
/// UHCI Q5_WORD1 Quick Send Register
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **SEND_Q5_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ5Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ5Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ5Word1 {
    // ======== Field Constants ========

    /// **SEND_Q5_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q5_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q5_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q5_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q5_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q5_WORD1_MASK) >> Self::SEND_Q5_WORD1_OFFSET
    }

    /// Write **SEND_Q5_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q5_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q5_WORD1_MASK)
                | ((value << Self::SEND_Q5_WORD1_OFFSET) & Self::SEND_Q5_WORD1_MASK)
        });
    }
}

/// REG_Q5_WORD1 value type for batch read/modify/write operations
///
/// Use `regq5word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq5word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ5Word1Val {
    bits: u32,
}

impl RegQ5Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q5_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q5_word1(&self) -> u32 {
        (self.bits & RegQ5Word1::SEND_Q5_WORD1_MASK) >> RegQ5Word1::SEND_Q5_WORD1_OFFSET
    }

    /// Set **SEND_Q5_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q5_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ5Word1::SEND_Q5_WORD1_MASK)
                | ((value << RegQ5Word1::SEND_Q5_WORD1_OFFSET) & RegQ5Word1::SEND_Q5_WORD1_MASK),
        }
    }
}

impl Default for RegQ5Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ5Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ5Word1Val {
        RegQ5Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ5Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ5Word1Val) -> RegQ5Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q6_WORD0 Register
///
/// UHCI Q6_WORD0 Quick Send Register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **SEND_Q6_WORD0** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ6Word0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ6Word0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ6Word0 {
    // ======== Field Constants ========

    /// **SEND_Q6_WORD0**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q6_WORD0_OFFSET: u32 = 0;
    pub const SEND_Q6_WORD0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q6_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q6_word0(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q6_WORD0_MASK) >> Self::SEND_Q6_WORD0_OFFSET
    }

    /// Write **SEND_Q6_WORD0** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q6_word0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q6_WORD0_MASK)
                | ((value << Self::SEND_Q6_WORD0_OFFSET) & Self::SEND_Q6_WORD0_MASK)
        });
    }
}

/// REG_Q6_WORD0 value type for batch read/modify/write operations
///
/// Use `regq6word0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq6word0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ6Word0Val {
    bits: u32,
}

impl RegQ6Word0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q6_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q6_word0(&self) -> u32 {
        (self.bits & RegQ6Word0::SEND_Q6_WORD0_MASK) >> RegQ6Word0::SEND_Q6_WORD0_OFFSET
    }

    /// Set **SEND_Q6_WORD0** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q6_word0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ6Word0::SEND_Q6_WORD0_MASK)
                | ((value << RegQ6Word0::SEND_Q6_WORD0_OFFSET) & RegQ6Word0::SEND_Q6_WORD0_MASK),
        }
    }
}

impl Default for RegQ6Word0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ6Word0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ6Word0Val {
        RegQ6Word0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ6Word0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ6Word0Val) -> RegQ6Word0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_Q6_WORD1 Register
///
/// UHCI Q6_WORD1 Quick Send Register
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SEND_Q6_WORD1** `[31:0]` (RW)
///
///   Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
#[repr(transparent)]
pub struct RegQ6Word1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegQ6Word1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegQ6Word1 {
    // ======== Field Constants ========

    /// **SEND_Q6_WORD1**
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEND_Q6_WORD1_OFFSET: u32 = 0;
    pub const SEND_Q6_WORD1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEND_Q6_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn send_q6_word1(&self) -> u32 {
        (self.value.read_bits() & Self::SEND_Q6_WORD1_MASK) >> Self::SEND_Q6_WORD1_OFFSET
    }

    /// Write **SEND_Q6_WORD1** `[31:0]` (RW)
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub fn write_send_q6_word1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEND_Q6_WORD1_MASK)
                | ((value << Self::SEND_Q6_WORD1_OFFSET) & Self::SEND_Q6_WORD1_MASK)
        });
    }
}

/// REG_Q6_WORD1 value type for batch read/modify/write operations
///
/// Use `regq6word1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regq6word1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegQ6Word1Val {
    bits: u32,
}

impl RegQ6Word1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_Q6_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn send_q6_word1(&self) -> u32 {
        (self.bits & RegQ6Word1::SEND_Q6_WORD1_MASK) >> RegQ6Word1::SEND_Q6_WORD1_OFFSET
    }

    /// Set **SEND_Q6_WORD1** `[31:0]`
    ///
    /// Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM.
    #[inline(always)]
    pub const fn with_send_q6_word1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegQ6Word1::SEND_Q6_WORD1_MASK)
                | ((value << RegQ6Word1::SEND_Q6_WORD1_OFFSET) & RegQ6Word1::SEND_Q6_WORD1_MASK),
        }
    }
}

impl Default for RegQ6Word1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegQ6Word1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegQ6Word1Val {
        RegQ6Word1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegQ6Word1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegQ6Word1Val) -> RegQ6Word1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ESC_CONF0 Register
///
/// UHCI Escapes Sequence Configuration Register0
///
/// Address offset: 0x006c
/// Reset value: 0x00dcdbc0
///
/// ## Fields
///
/// - **SEPER_ESC_CHAR1** `[23:16]` (RW)
///
///   Configures the second char of SLIP escape character, default value is 0xDC.
///
/// - **SEPER_ESC_CHAR0** `[15:8]` (RW)
///
///   Configures the first char of SLIP escape character, default value is 0xDB.
///
/// - **SEPER_CHAR** `[7:0]` (RW)
///
///   Configures the delimiter for encoding, default value is 0xC0.
#[repr(transparent)]
pub struct EscConf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EscConf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EscConf0 {
    // ======== Field Constants ========

    /// **SEPER_ESC_CHAR1**
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDC.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const SEPER_ESC_CHAR1_OFFSET: u32 = 16;
    pub const SEPER_ESC_CHAR1_MASK: u32 = 0x00ff0000;

    /// **SEPER_ESC_CHAR0**
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SEPER_ESC_CHAR0_OFFSET: u32 = 8;
    pub const SEPER_ESC_CHAR0_MASK: u32 = 0x0000ff00;

    /// **SEPER_CHAR**
    ///
    /// Configures the delimiter for encoding, default value is 0xC0.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SEPER_CHAR_OFFSET: u32 = 0;
    pub const SEPER_CHAR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SEPER_ESC_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDC.
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> u32 {
        (self.value.read_bits() & Self::SEPER_ESC_CHAR1_MASK) >> Self::SEPER_ESC_CHAR1_OFFSET
    }

    /// Write **SEPER_ESC_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDC.
    #[inline(always)]
    pub fn write_seper_esc_char1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEPER_ESC_CHAR1_MASK)
                | ((value << Self::SEPER_ESC_CHAR1_OFFSET) & Self::SEPER_ESC_CHAR1_MASK)
        });
    }

    /// **SEPER_ESC_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> u32 {
        (self.value.read_bits() & Self::SEPER_ESC_CHAR0_MASK) >> Self::SEPER_ESC_CHAR0_OFFSET
    }

    /// Write **SEPER_ESC_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn write_seper_esc_char0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEPER_ESC_CHAR0_MASK)
                | ((value << Self::SEPER_ESC_CHAR0_OFFSET) & Self::SEPER_ESC_CHAR0_MASK)
        });
    }

    /// **SEPER_CHAR** `[7:0]` (RW)
    ///
    /// Configures the delimiter for encoding, default value is 0xC0.
    #[inline(always)]
    pub fn seper_char(&self) -> u32 {
        (self.value.read_bits() & Self::SEPER_CHAR_MASK) >> Self::SEPER_CHAR_OFFSET
    }

    /// Write **SEPER_CHAR** `[7:0]` (RW)
    ///
    /// Configures the delimiter for encoding, default value is 0xC0.
    #[inline(always)]
    pub fn write_seper_char(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEPER_CHAR_MASK)
                | ((value << Self::SEPER_CHAR_OFFSET) & Self::SEPER_CHAR_MASK)
        });
    }
}

/// ESC_CONF0 value type for batch read/modify/write operations
///
/// Use `escconf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `escconf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EscConf0Val {
    bits: u32,
}

impl EscConf0Val {
    /// Create from raw bits
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
        Self { bits: 0x00dcdbc0 }
    }

    /// Read **SEPER_ESC_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDC.
    #[inline(always)]
    pub const fn seper_esc_char1(&self) -> u32 {
        (self.bits & EscConf0::SEPER_ESC_CHAR1_MASK) >> EscConf0::SEPER_ESC_CHAR1_OFFSET
    }

    /// Read **SEPER_ESC_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn seper_esc_char0(&self) -> u32 {
        (self.bits & EscConf0::SEPER_ESC_CHAR0_MASK) >> EscConf0::SEPER_ESC_CHAR0_OFFSET
    }

    /// Read **SEPER_CHAR** `[7:0]`
    ///
    /// Configures the delimiter for encoding, default value is 0xC0.
    #[inline(always)]
    pub const fn seper_char(&self) -> u32 {
        (self.bits & EscConf0::SEPER_CHAR_MASK) >> EscConf0::SEPER_CHAR_OFFSET
    }

    /// Set **SEPER_ESC_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDC.
    #[inline(always)]
    pub const fn with_seper_esc_char1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf0::SEPER_ESC_CHAR1_MASK)
                | ((value << EscConf0::SEPER_ESC_CHAR1_OFFSET) & EscConf0::SEPER_ESC_CHAR1_MASK),
        }
    }

    /// Set **SEPER_ESC_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn with_seper_esc_char0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf0::SEPER_ESC_CHAR0_MASK)
                | ((value << EscConf0::SEPER_ESC_CHAR0_OFFSET) & EscConf0::SEPER_ESC_CHAR0_MASK),
        }
    }

    /// Set **SEPER_CHAR** `[7:0]`
    ///
    /// Configures the delimiter for encoding, default value is 0xC0.
    #[inline(always)]
    pub const fn with_seper_char(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf0::SEPER_CHAR_MASK)
                | ((value << EscConf0::SEPER_CHAR_OFFSET) & EscConf0::SEPER_CHAR_MASK),
        }
    }
}

impl Default for EscConf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EscConf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EscConf0Val {
        EscConf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EscConf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EscConf0Val) -> EscConf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ESC_CONF1 Register
///
/// UHCI Escapes Sequence Configuration Register1
///
/// Address offset: 0x0070
/// Reset value: 0x00dddbdb
///
/// ## Fields
///
/// - **ESC_SEQ0_CHAR1** `[23:16]` (RW)
///
///   Configures the second char of SLIP escape character, default value is 0xDD.
///
/// - **ESC_SEQ0_CHAR0** `[15:8]` (RW)
///
///   Configures the first char of SLIP escape character, default value is 0xDB.
///
/// - **ESC_SEQ0** `[7:0]` (RW)
///
///   Configures the char needing encoding, which is 0xDB as flow control char by default.
#[repr(transparent)]
pub struct EscConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EscConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EscConf1 {
    // ======== Field Constants ========

    /// **ESC_SEQ0_CHAR1**
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDD.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ESC_SEQ0_CHAR1_OFFSET: u32 = 16;
    pub const ESC_SEQ0_CHAR1_MASK: u32 = 0x00ff0000;

    /// **ESC_SEQ0_CHAR0**
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const ESC_SEQ0_CHAR0_OFFSET: u32 = 8;
    pub const ESC_SEQ0_CHAR0_MASK: u32 = 0x0000ff00;

    /// **ESC_SEQ0**
    ///
    /// Configures the char needing encoding, which is 0xDB as flow control char by default.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ESC_SEQ0_OFFSET: u32 = 0;
    pub const ESC_SEQ0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ESC_SEQ0_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDD.
    #[inline(always)]
    pub fn esc_seq0_char1(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ0_CHAR1_MASK) >> Self::ESC_SEQ0_CHAR1_OFFSET
    }

    /// Write **ESC_SEQ0_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDD.
    #[inline(always)]
    pub fn write_esc_seq0_char1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ0_CHAR1_MASK)
                | ((value << Self::ESC_SEQ0_CHAR1_OFFSET) & Self::ESC_SEQ0_CHAR1_MASK)
        });
    }

    /// **ESC_SEQ0_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn esc_seq0_char0(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ0_CHAR0_MASK) >> Self::ESC_SEQ0_CHAR0_OFFSET
    }

    /// Write **ESC_SEQ0_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn write_esc_seq0_char0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ0_CHAR0_MASK)
                | ((value << Self::ESC_SEQ0_CHAR0_OFFSET) & Self::ESC_SEQ0_CHAR0_MASK)
        });
    }

    /// **ESC_SEQ0** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0xDB as flow control char by default.
    #[inline(always)]
    pub fn esc_seq0(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ0_MASK) >> Self::ESC_SEQ0_OFFSET
    }

    /// Write **ESC_SEQ0** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0xDB as flow control char by default.
    #[inline(always)]
    pub fn write_esc_seq0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ0_MASK) | ((value << Self::ESC_SEQ0_OFFSET) & Self::ESC_SEQ0_MASK)
        });
    }
}

/// ESC_CONF1 value type for batch read/modify/write operations
///
/// Use `escconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `escconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EscConf1Val {
    bits: u32,
}

impl EscConf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00dddbdb }
    }

    /// Read **ESC_SEQ0_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDD.
    #[inline(always)]
    pub const fn esc_seq0_char1(&self) -> u32 {
        (self.bits & EscConf1::ESC_SEQ0_CHAR1_MASK) >> EscConf1::ESC_SEQ0_CHAR1_OFFSET
    }

    /// Read **ESC_SEQ0_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn esc_seq0_char0(&self) -> u32 {
        (self.bits & EscConf1::ESC_SEQ0_CHAR0_MASK) >> EscConf1::ESC_SEQ0_CHAR0_OFFSET
    }

    /// Read **ESC_SEQ0** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0xDB as flow control char by default.
    #[inline(always)]
    pub const fn esc_seq0(&self) -> u32 {
        (self.bits & EscConf1::ESC_SEQ0_MASK) >> EscConf1::ESC_SEQ0_OFFSET
    }

    /// Set **ESC_SEQ0_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDD.
    #[inline(always)]
    pub const fn with_esc_seq0_char1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf1::ESC_SEQ0_CHAR1_MASK)
                | ((value << EscConf1::ESC_SEQ0_CHAR1_OFFSET) & EscConf1::ESC_SEQ0_CHAR1_MASK),
        }
    }

    /// Set **ESC_SEQ0_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn with_esc_seq0_char0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf1::ESC_SEQ0_CHAR0_MASK)
                | ((value << EscConf1::ESC_SEQ0_CHAR0_OFFSET) & EscConf1::ESC_SEQ0_CHAR0_MASK),
        }
    }

    /// Set **ESC_SEQ0** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0xDB as flow control char by default.
    #[inline(always)]
    pub const fn with_esc_seq0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf1::ESC_SEQ0_MASK)
                | ((value << EscConf1::ESC_SEQ0_OFFSET) & EscConf1::ESC_SEQ0_MASK),
        }
    }
}

impl Default for EscConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EscConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EscConf1Val {
        EscConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EscConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EscConf1Val) -> EscConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ESC_CONF2 Register
///
/// UHCI Escapes Sequence Configuration Register2
///
/// Address offset: 0x0074
/// Reset value: 0x00dedb11
///
/// ## Fields
///
/// - **ESC_SEQ1_CHAR1** `[23:16]` (RW)
///
///   Configures the second char of SLIP escape character, default value is 0xDE.
///
/// - **ESC_SEQ1_CHAR0** `[15:8]` (RW)
///
///   Configures the first char of SLIP escape character, default value is 0xDB.
///
/// - **ESC_SEQ1** `[7:0]` (RW)
///
///   Configures the char needing encoding, which is 0x11 as flow control char by default.
#[repr(transparent)]
pub struct EscConf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EscConf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EscConf2 {
    // ======== Field Constants ========

    /// **ESC_SEQ1_CHAR1**
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDE.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ESC_SEQ1_CHAR1_OFFSET: u32 = 16;
    pub const ESC_SEQ1_CHAR1_MASK: u32 = 0x00ff0000;

    /// **ESC_SEQ1_CHAR0**
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const ESC_SEQ1_CHAR0_OFFSET: u32 = 8;
    pub const ESC_SEQ1_CHAR0_MASK: u32 = 0x0000ff00;

    /// **ESC_SEQ1**
    ///
    /// Configures the char needing encoding, which is 0x11 as flow control char by default.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ESC_SEQ1_OFFSET: u32 = 0;
    pub const ESC_SEQ1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ESC_SEQ1_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDE.
    #[inline(always)]
    pub fn esc_seq1_char1(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ1_CHAR1_MASK) >> Self::ESC_SEQ1_CHAR1_OFFSET
    }

    /// Write **ESC_SEQ1_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDE.
    #[inline(always)]
    pub fn write_esc_seq1_char1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ1_CHAR1_MASK)
                | ((value << Self::ESC_SEQ1_CHAR1_OFFSET) & Self::ESC_SEQ1_CHAR1_MASK)
        });
    }

    /// **ESC_SEQ1_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn esc_seq1_char0(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ1_CHAR0_MASK) >> Self::ESC_SEQ1_CHAR0_OFFSET
    }

    /// Write **ESC_SEQ1_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn write_esc_seq1_char0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ1_CHAR0_MASK)
                | ((value << Self::ESC_SEQ1_CHAR0_OFFSET) & Self::ESC_SEQ1_CHAR0_MASK)
        });
    }

    /// **ESC_SEQ1** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0x11 as flow control char by default.
    #[inline(always)]
    pub fn esc_seq1(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ1_MASK) >> Self::ESC_SEQ1_OFFSET
    }

    /// Write **ESC_SEQ1** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0x11 as flow control char by default.
    #[inline(always)]
    pub fn write_esc_seq1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ1_MASK) | ((value << Self::ESC_SEQ1_OFFSET) & Self::ESC_SEQ1_MASK)
        });
    }
}

/// ESC_CONF2 value type for batch read/modify/write operations
///
/// Use `escconf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `escconf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EscConf2Val {
    bits: u32,
}

impl EscConf2Val {
    /// Create from raw bits
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
        Self { bits: 0x00dedb11 }
    }

    /// Read **ESC_SEQ1_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDE.
    #[inline(always)]
    pub const fn esc_seq1_char1(&self) -> u32 {
        (self.bits & EscConf2::ESC_SEQ1_CHAR1_MASK) >> EscConf2::ESC_SEQ1_CHAR1_OFFSET
    }

    /// Read **ESC_SEQ1_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn esc_seq1_char0(&self) -> u32 {
        (self.bits & EscConf2::ESC_SEQ1_CHAR0_MASK) >> EscConf2::ESC_SEQ1_CHAR0_OFFSET
    }

    /// Read **ESC_SEQ1** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0x11 as flow control char by default.
    #[inline(always)]
    pub const fn esc_seq1(&self) -> u32 {
        (self.bits & EscConf2::ESC_SEQ1_MASK) >> EscConf2::ESC_SEQ1_OFFSET
    }

    /// Set **ESC_SEQ1_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDE.
    #[inline(always)]
    pub const fn with_esc_seq1_char1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf2::ESC_SEQ1_CHAR1_MASK)
                | ((value << EscConf2::ESC_SEQ1_CHAR1_OFFSET) & EscConf2::ESC_SEQ1_CHAR1_MASK),
        }
    }

    /// Set **ESC_SEQ1_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn with_esc_seq1_char0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf2::ESC_SEQ1_CHAR0_MASK)
                | ((value << EscConf2::ESC_SEQ1_CHAR0_OFFSET) & EscConf2::ESC_SEQ1_CHAR0_MASK),
        }
    }

    /// Set **ESC_SEQ1** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0x11 as flow control char by default.
    #[inline(always)]
    pub const fn with_esc_seq1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf2::ESC_SEQ1_MASK)
                | ((value << EscConf2::ESC_SEQ1_OFFSET) & EscConf2::ESC_SEQ1_MASK),
        }
    }
}

impl Default for EscConf2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EscConf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EscConf2Val {
        EscConf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EscConf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EscConf2Val) -> EscConf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ESC_CONF3 Register
///
/// UHCI Escapes Sequence Configuration Register3
///
/// Address offset: 0x0078
/// Reset value: 0x00dfdb13
///
/// ## Fields
///
/// - **ESC_SEQ2_CHAR1** `[23:16]` (RW)
///
///   Configures the second char of SLIP escape character, default value is 0xDF.
///
/// - **ESC_SEQ2_CHAR0** `[15:8]` (RW)
///
///   Configures the first char of SLIP escape character, default value is 0xDB.
///
/// - **ESC_SEQ2** `[7:0]` (RW)
///
///   Configures the char needing encoding, which is 0x13 as flow control char by default.
#[repr(transparent)]
pub struct EscConf3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EscConf3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EscConf3 {
    // ======== Field Constants ========

    /// **ESC_SEQ2_CHAR1**
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDF.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ESC_SEQ2_CHAR1_OFFSET: u32 = 16;
    pub const ESC_SEQ2_CHAR1_MASK: u32 = 0x00ff0000;

    /// **ESC_SEQ2_CHAR0**
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const ESC_SEQ2_CHAR0_OFFSET: u32 = 8;
    pub const ESC_SEQ2_CHAR0_MASK: u32 = 0x0000ff00;

    /// **ESC_SEQ2**
    ///
    /// Configures the char needing encoding, which is 0x13 as flow control char by default.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ESC_SEQ2_OFFSET: u32 = 0;
    pub const ESC_SEQ2_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ESC_SEQ2_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDF.
    #[inline(always)]
    pub fn esc_seq2_char1(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ2_CHAR1_MASK) >> Self::ESC_SEQ2_CHAR1_OFFSET
    }

    /// Write **ESC_SEQ2_CHAR1** `[23:16]` (RW)
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDF.
    #[inline(always)]
    pub fn write_esc_seq2_char1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ2_CHAR1_MASK)
                | ((value << Self::ESC_SEQ2_CHAR1_OFFSET) & Self::ESC_SEQ2_CHAR1_MASK)
        });
    }

    /// **ESC_SEQ2_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn esc_seq2_char0(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ2_CHAR0_MASK) >> Self::ESC_SEQ2_CHAR0_OFFSET
    }

    /// Write **ESC_SEQ2_CHAR0** `[15:8]` (RW)
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub fn write_esc_seq2_char0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ2_CHAR0_MASK)
                | ((value << Self::ESC_SEQ2_CHAR0_OFFSET) & Self::ESC_SEQ2_CHAR0_MASK)
        });
    }

    /// **ESC_SEQ2** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0x13 as flow control char by default.
    #[inline(always)]
    pub fn esc_seq2(&self) -> u32 {
        (self.value.read_bits() & Self::ESC_SEQ2_MASK) >> Self::ESC_SEQ2_OFFSET
    }

    /// Write **ESC_SEQ2** `[7:0]` (RW)
    ///
    /// Configures the char needing encoding, which is 0x13 as flow control char by default.
    #[inline(always)]
    pub fn write_esc_seq2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ESC_SEQ2_MASK) | ((value << Self::ESC_SEQ2_OFFSET) & Self::ESC_SEQ2_MASK)
        });
    }
}

/// ESC_CONF3 value type for batch read/modify/write operations
///
/// Use `escconf3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `escconf3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EscConf3Val {
    bits: u32,
}

impl EscConf3Val {
    /// Create from raw bits
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
        Self { bits: 0x00dfdb13 }
    }

    /// Read **ESC_SEQ2_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDF.
    #[inline(always)]
    pub const fn esc_seq2_char1(&self) -> u32 {
        (self.bits & EscConf3::ESC_SEQ2_CHAR1_MASK) >> EscConf3::ESC_SEQ2_CHAR1_OFFSET
    }

    /// Read **ESC_SEQ2_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn esc_seq2_char0(&self) -> u32 {
        (self.bits & EscConf3::ESC_SEQ2_CHAR0_MASK) >> EscConf3::ESC_SEQ2_CHAR0_OFFSET
    }

    /// Read **ESC_SEQ2** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0x13 as flow control char by default.
    #[inline(always)]
    pub const fn esc_seq2(&self) -> u32 {
        (self.bits & EscConf3::ESC_SEQ2_MASK) >> EscConf3::ESC_SEQ2_OFFSET
    }

    /// Set **ESC_SEQ2_CHAR1** `[23:16]`
    ///
    /// Configures the second char of SLIP escape character, default value is 0xDF.
    #[inline(always)]
    pub const fn with_esc_seq2_char1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf3::ESC_SEQ2_CHAR1_MASK)
                | ((value << EscConf3::ESC_SEQ2_CHAR1_OFFSET) & EscConf3::ESC_SEQ2_CHAR1_MASK),
        }
    }

    /// Set **ESC_SEQ2_CHAR0** `[15:8]`
    ///
    /// Configures the first char of SLIP escape character, default value is 0xDB.
    #[inline(always)]
    pub const fn with_esc_seq2_char0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf3::ESC_SEQ2_CHAR0_MASK)
                | ((value << EscConf3::ESC_SEQ2_CHAR0_OFFSET) & EscConf3::ESC_SEQ2_CHAR0_MASK),
        }
    }

    /// Set **ESC_SEQ2** `[7:0]`
    ///
    /// Configures the char needing encoding, which is 0x13 as flow control char by default.
    #[inline(always)]
    pub const fn with_esc_seq2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EscConf3::ESC_SEQ2_MASK)
                | ((value << EscConf3::ESC_SEQ2_OFFSET) & EscConf3::ESC_SEQ2_MASK),
        }
    }
}

impl Default for EscConf3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EscConf3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EscConf3Val {
        EscConf3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EscConf3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EscConf3Val) -> EscConf3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PKT_THRES Register
///
/// UCHI Packet Length Configuration Register
///
/// Address offset: 0x007c
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **PKT_THRS** `[12:0]` (RW)
///
///   Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
#[repr(transparent)]
pub struct PktThres {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PktThres {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PktThres {
    // ======== Field Constants ========

    /// **PKT_THRS**
    ///
    /// Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const PKT_THRS_OFFSET: u32 = 0;
    pub const PKT_THRS_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **PKT_THRS** `[12:0]` (RW)
    ///
    /// Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
    #[inline(always)]
    pub fn pkt_thrs(&self) -> u32 {
        (self.value.read_bits() & Self::PKT_THRS_MASK) >> Self::PKT_THRS_OFFSET
    }

    /// Write **PKT_THRS** `[12:0]` (RW)
    ///
    /// Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
    #[inline(always)]
    pub fn write_pkt_thrs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PKT_THRS_MASK) | ((value << Self::PKT_THRS_OFFSET) & Self::PKT_THRS_MASK)
        });
    }
}

/// PKT_THRES value type for batch read/modify/write operations
///
/// Use `pktthres.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pktthres.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PktThresVal {
    bits: u32,
}

impl PktThresVal {
    /// Create from raw bits
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

    /// Read **PKT_THRS** `[12:0]`
    ///
    /// Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
    #[inline(always)]
    pub const fn pkt_thrs(&self) -> u32 {
        (self.bits & PktThres::PKT_THRS_MASK) >> PktThres::PKT_THRS_OFFSET
    }

    /// Set **PKT_THRS** `[12:0]`
    ///
    /// Configures the data packet's maximum length when UHCI_HEAD_EN is 0.
    #[inline(always)]
    pub const fn with_pkt_thrs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PktThres::PKT_THRS_MASK)
                | ((value << PktThres::PKT_THRS_OFFSET) & PktThres::PKT_THRS_MASK),
        }
    }
}

impl Default for PktThresVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PktThres {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PktThresVal {
        PktThresVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PktThresVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PktThresVal) -> PktThresVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// UHCI Version Register
///
/// Address offset: 0x0080
/// Reset value: 0x02201100
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   Configures version.
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
    /// Configures version.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// Configures version.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// Configures version.
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
        Self { bits: 0x02201100 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// Configures version.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// Configures version.
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
    /// UHCI Configuration Register0
    pub conf0: Conf0,
    /// 0x0004
    /// UHCI Interrupt Raw Register
    pub int_raw: IntRaw,
    /// 0x0008
    /// UHCI Interrupt Status Register
    pub int_st: IntSt,
    /// 0x000c
    /// UHCI Interrupt Enable Register
    pub int_ena: IntEna,
    /// 0x0010
    /// UHCI Interrupt Clear Register
    pub int_clr: IntClr,
    /// 0x0014
    /// UHCI Configuration Register1
    pub conf1: Conf1,
    /// 0x0018
    /// UHCI Receive Status Register
    pub state0: State0,
    /// 0x001c
    /// UHCI Transmit Status Register
    pub state1: State1,
    /// 0x0020
    /// UHCI Escapes Configuration Register0
    pub escape_conf: EscapeConf,
    /// 0x0024
    /// UHCI Hung Configuration Register0
    pub hung_conf: HungConf,
    /// 0x0028
    /// UHCI Ack Value Configuration Register0
    pub ack_num: AckNum,
    /// 0x002c
    /// UHCI Head Register
    pub rx_head: RxHead,
    /// 0x0030
    /// UCHI Quick send Register
    pub quick_sent: QuickSent,
    /// 0x0034
    /// UHCI Q0_WORD0 Quick Send Register
    pub reg_q0_word0: RegQ0Word0,
    /// 0x0038
    /// UHCI Q0_WORD1 Quick Send Register
    pub reg_q0_word1: RegQ0Word1,
    /// 0x003c
    /// UHCI Q1_WORD0 Quick Send Register
    pub reg_q1_word0: RegQ1Word0,
    /// 0x0040
    /// UHCI Q1_WORD1 Quick Send Register
    pub reg_q1_word1: RegQ1Word1,
    /// 0x0044
    /// UHCI Q2_WORD0 Quick Send Register
    pub reg_q2_word0: RegQ2Word0,
    /// 0x0048
    /// UHCI Q2_WORD1 Quick Send Register
    pub reg_q2_word1: RegQ2Word1,
    /// 0x004c
    /// UHCI Q3_WORD0 Quick Send Register
    pub reg_q3_word0: RegQ3Word0,
    /// 0x0050
    /// UHCI Q3_WORD1 Quick Send Register
    pub reg_q3_word1: RegQ3Word1,
    /// 0x0054
    /// UHCI Q4_WORD0 Quick Send Register
    pub reg_q4_word0: RegQ4Word0,
    /// 0x0058
    /// UHCI Q4_WORD1 Quick Send Register
    pub reg_q4_word1: RegQ4Word1,
    /// 0x005c
    /// UHCI Q5_WORD0 Quick Send Register
    pub reg_q5_word0: RegQ5Word0,
    /// 0x0060
    /// UHCI Q5_WORD1 Quick Send Register
    pub reg_q5_word1: RegQ5Word1,
    /// 0x0064
    /// UHCI Q6_WORD0 Quick Send Register
    pub reg_q6_word0: RegQ6Word0,
    /// 0x0068
    /// UHCI Q6_WORD1 Quick Send Register
    pub reg_q6_word1: RegQ6Word1,
    /// 0x006c
    /// UHCI Escapes Sequence Configuration Register0
    pub esc_conf0: EscConf0,
    /// 0x0070
    /// UHCI Escapes Sequence Configuration Register1
    pub esc_conf1: EscConf1,
    /// 0x0074
    /// UHCI Escapes Sequence Configuration Register2
    pub esc_conf2: EscConf2,
    /// 0x0078
    /// UHCI Escapes Sequence Configuration Register3
    pub esc_conf3: EscConf3,
    /// 0x007c
    /// UCHI Packet Length Configuration Register
    pub pkt_thres: PktThres,
    /// 0x0080
    /// UHCI Version Register
    pub date: Date,
}
