//! USB_DEVICE
//! Full-speed USB Serial/JTAG Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x500d2000;

/// EP1 Register
///
/// FIFO access for the CDC-ACM data IN and OUT endpoints.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RDWR_BYTE** `[7:0]` (RO)
///
///   Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
#[repr(transparent)]
pub struct Ep1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ep1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ep1 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RDWR_BYTE**
    ///
    /// Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const USB_SERIAL_JTAG_RDWR_BYTE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RDWR_BYTE_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RDWR_BYTE** `[7:0]` (RO)
    ///
    /// Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
    #[inline(always)]
    pub fn usb_serial_jtag_rdwr_byte(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RDWR_BYTE_MASK)
            >> Self::USB_SERIAL_JTAG_RDWR_BYTE_OFFSET
    }
}

/// EP1 value type for batch read/modify/write operations
///
/// Use `ep1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ep1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ep1Val {
    bits: u32,
}

impl Ep1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_RDWR_BYTE** `[7:0]`
    ///
    /// Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
    #[inline(always)]
    pub const fn usb_serial_jtag_rdwr_byte(&self) -> u32 {
        (self.bits & Ep1::USB_SERIAL_JTAG_RDWR_BYTE_MASK) >> Ep1::USB_SERIAL_JTAG_RDWR_BYTE_OFFSET
    }
}

impl Default for Ep1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ep1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ep1Val {
        Ep1Val::from_bits(self.value.read_bits())
    }
}

/// EP1_CONF Register
///
/// Configuration and control registers for the CDC-ACM FIFOs.
///
/// Address offset: 0x0004
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL** `[2]` (RO)
///
///   1'b1: Indicate there is data in UART Rx FIFO.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE** `[1]` (RO)
///
///   1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host.
///
/// - **USB_SERIAL_JTAG_WR_DONE** `[0]` (WO)
///
///   Set this bit to indicate writing byte data to UART Tx FIFO is done.
#[repr(transparent)]
pub struct Ep1Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ep1Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ep1Conf {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL**
    ///
    /// 1'b1: Indicate there is data in UART Rx FIFO.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE**
    ///
    /// 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_WR_DONE**
    ///
    /// Set this bit to indicate writing byte data to UART Tx FIFO is done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_WR_DONE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_WR_DONE_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_WR_DONE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL** `[2]` (RO)
    ///
    /// 1'b1: Indicate there is data in UART Rx FIFO.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_ep_data_avail(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE** `[1]` (RO)
    ///
    /// 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_ep_data_free(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_MASK) != 0
    }

    /// Trigger **USB_SERIAL_JTAG_WR_DONE** (WO)
    ///
    /// Set this bit to indicate writing byte data to UART Tx FIFO is done.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_wr_done(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_WR_DONE_MASK);
    }
}

/// EP1_CONF value type for batch read/modify/write operations
///
/// Use `ep1conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ep1conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ep1ConfVal {
    bits: u32,
}

impl Ep1ConfVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL** `[2:2]`
    ///
    /// 1'b1: Indicate there is data in UART Rx FIFO.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_ep_data_avail(&self) -> bool {
        (self.bits & Ep1Conf::USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE** `[1:1]`
    ///
    /// 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_ep_data_free(&self) -> bool {
        (self.bits & Ep1Conf::USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_WR_DONE** `[0:0]`
    ///
    /// Set this bit to indicate writing byte data to UART Tx FIFO is done.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_wr_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ep1Conf::USB_SERIAL_JTAG_WR_DONE_MASK
            } else {
                self.bits & !Ep1Conf::USB_SERIAL_JTAG_WR_DONE_MASK
            },
        }
    }
}

impl Default for Ep1ConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ep1Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ep1ConfVal {
        Ep1ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ep1ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ep1ConfVal) -> Ep1ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Interrupt raw status register.
///
/// Address offset: 0x0008
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW** `[15]` (RW)
///
///   The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
///
/// - **USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW** `[14]` (RW)
///
///   The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
///
/// - **USB_SERIAL_JTAG_DTR_CHG_INT_RAW** `[13]` (RW)
///
///   The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
///
/// - **USB_SERIAL_JTAG_RTS_CHG_INT_RAW** `[12]` (RW)
///
///   The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW** `[11]` (RW)
///
///   The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW** `[10]` (RW)
///
///   The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
///
/// - **USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW** `[9]` (RW)
///
///   The raw interrupt bit turns to high level when usb bus reset is detected.
///
/// - **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW** `[8]` (RW)
///
///   The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
///
/// - **USB_SERIAL_JTAG_STUFF_ERR_INT_RAW** `[7]` (RW)
///
///   The raw interrupt bit turns to high level when stuff error is detected.
///
/// - **USB_SERIAL_JTAG_CRC16_ERR_INT_RAW** `[6]` (RW)
///
///   The raw interrupt bit turns to high level when CRC16 error is detected.
///
/// - **USB_SERIAL_JTAG_CRC5_ERR_INT_RAW** `[5]` (RW)
///
///   The raw interrupt bit turns to high level when CRC5 error is detected.
///
/// - **USB_SERIAL_JTAG_PID_ERR_INT_RAW** `[4]` (RW)
///
///   The raw interrupt bit turns to high level when pid error is detected.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
///
/// - **USB_SERIAL_JTAG_SOF_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when SOF frame is received.
///
/// - **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
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

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_OFFSET: u32 = 15;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK: u32 = 0x00008000;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW: u32 = 1 << 15;

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_OFFSET: u32 = 14;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK: u32 = 0x00004000;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW: u32 = 1 << 14;

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_RAW_OFFSET: u32 = 13;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK: u32 = 0x00002000;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_RAW: u32 = 1 << 13;

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_RAW_OFFSET: u32 = 12;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK: u32 = 0x00001000;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_RAW: u32 = 1 << 12;

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_OFFSET: u32 = 11;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK: u32 = 0x00000800;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW: u32 = 1 << 11;

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_OFFSET: u32 = 10;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK: u32 = 0x00000400;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW: u32 = 1 << 10;

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when usb bus reset is detected.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when stuff error is detected.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_RAW: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when CRC16 error is detected.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_RAW: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when CRC5 error is detected.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_RAW: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_PID_ERR_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when pid error is detected.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_PID_ERR_INT_RAW_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_RAW: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SOF_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when SOF frame is received.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SOF_INT_RAW_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SOF_INT_RAW_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SOF_INT_RAW: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW** `[15]` (RW)
    ///
    /// The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_set_line_code_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_set_line_code_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW** `[14]` (RW)
    ///
    /// The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_get_line_code_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_get_line_code_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_RAW** `[13]` (RW)
    ///
    /// The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DTR_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dtr_chg_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DTR_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dtr_chg_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_RAW** `[12]` (RW)
    ///
    /// The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_RTS_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_rts_chg_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_RTS_CHG_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_rts_chg_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW** `[11]` (RW)
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_out_ep2_zero_payload_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_out_ep2_zero_payload_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW** `[10]` (RW)
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_out_ep1_zero_payload_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_out_ep1_zero_payload_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW** `[9]` (RW)
    ///
    /// The raw interrupt bit turns to high level when usb bus reset is detected.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_bus_reset_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_bus_reset_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW** `[8]` (RW)
    ///
    /// The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_in_token_rec_in_ep1_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_in_token_rec_in_ep1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_RAW** `[7]` (RW)
    ///
    /// The raw interrupt bit turns to high level when stuff error is detected.
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_STUFF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_stuff_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_STUFF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_stuff_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_RAW** `[6]` (RW)
    ///
    /// The raw interrupt bit turns to high level when CRC16 error is detected.
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_CRC16_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_crc16_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_CRC16_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_crc16_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_RAW** `[5]` (RW)
    ///
    /// The raw interrupt bit turns to high level when CRC5 error is detected.
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_CRC5_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_crc5_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_CRC5_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_crc5_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_PID_ERR_INT_RAW** `[4]` (RW)
    ///
    /// The raw interrupt bit turns to high level when pid error is detected.
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_PID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_pid_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_PID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_pid_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_in_empty_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_in_empty_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_out_recv_pkt_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_out_recv_pkt_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_SOF_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when SOF frame is received.
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SOF_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_sof_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SOF_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_sof_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SOF_INT_RAW_MASK);
    }

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_jtag_in_flush_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK);
    }

    /// Clear USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_jtag_in_flush_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK);
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
        Self { bits: 0x00000008 }
    }

    /// Read **USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW** `[15:15]`
    ///
    /// The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    #[inline(always)]
    pub const fn usb_serial_jtag_set_line_code_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW** `[14:14]`
    ///
    /// The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_line_code_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DTR_CHG_INT_RAW** `[13:13]`
    ///
    /// The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    #[inline(always)]
    pub const fn usb_serial_jtag_dtr_chg_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RTS_CHG_INT_RAW** `[12:12]`
    ///
    /// The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    #[inline(always)]
    pub const fn usb_serial_jtag_rts_chg_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW** `[11:11]`
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_zero_payload_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW** `[10:10]`
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_zero_payload_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW** `[9:9]`
    ///
    /// The raw interrupt bit turns to high level when usb bus reset is detected.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_bus_reset_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW** `[8:8]`
    ///
    /// The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_token_rec_in_ep1_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_STUFF_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw interrupt bit turns to high level when stuff error is detected.
    #[inline(always)]
    pub const fn usb_serial_jtag_stuff_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC16_ERR_INT_RAW** `[6:6]`
    ///
    /// The raw interrupt bit turns to high level when CRC16 error is detected.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc16_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC5_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt bit turns to high level when CRC5 error is detected.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc5_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PID_ERR_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when pid error is detected.
    #[inline(always)]
    pub const fn usb_serial_jtag_pid_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_empty_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_recv_pkt_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SOF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when SOF frame is received.
    #[inline(always)]
    pub const fn usb_serial_jtag_sof_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_SOF_INT_RAW_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    #[inline(always)]
    pub const fn usb_serial_jtag_jtag_in_flush_int_raw(&self) -> bool {
        (self.bits & IntRaw::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW** `[15:15]`
    ///
    /// The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_set_line_code_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW** `[14:14]`
    ///
    /// The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_line_code_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DTR_CHG_INT_RAW** `[13:13]`
    ///
    /// The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dtr_chg_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_DTR_CHG_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_RTS_CHG_INT_RAW** `[12:12]`
    ///
    /// The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rts_chg_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_RTS_CHG_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW** `[11:11]`
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep2_zero_payload_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW** `[10:10]`
    ///
    /// The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep1_zero_payload_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW** `[9:9]`
    ///
    /// The raw interrupt bit turns to high level when usb bus reset is detected.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_bus_reset_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW** `[8:8]`
    ///
    /// The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_in_token_rec_in_ep1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_STUFF_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw interrupt bit turns to high level when stuff error is detected.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_stuff_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC16_ERR_INT_RAW** `[6:6]`
    ///
    /// The raw interrupt bit turns to high level when CRC16 error is detected.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc16_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC5_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt bit turns to high level when CRC5 error is detected.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc5_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PID_ERR_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit turns to high level when pid error is detected.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_pid_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_PID_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_in_empty_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_out_recv_pkt_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SOF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when SOF frame is received.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_sof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_SOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_SOF_INT_RAW_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_jtag_in_flush_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_MASK
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
/// Interrupt status register.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST** `[15]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST** `[14]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_DTR_CHG_INT_ST** `[13]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_RTS_CHG_INT_ST** `[12]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST** `[11]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST** `[10]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST** `[9]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
///
/// - **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST** `[8]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
///
/// - **USB_SERIAL_JTAG_STUFF_ERR_INT_ST** `[7]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC16_ERR_INT_ST** `[6]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC5_ERR_INT_ST** `[5]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_PID_ERR_INT_ST** `[4]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST** `[3]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SOF_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt.
///
/// - **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
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

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_OFFSET: u32 = 15;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_MASK: u32 = 0x00008000;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST: u32 = 1 << 15;

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_OFFSET: u32 = 14;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_MASK: u32 = 0x00004000;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST: u32 = 1 << 14;

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ST_OFFSET: u32 = 13;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ST_MASK: u32 = 0x00002000;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ST: u32 = 1 << 13;

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ST_OFFSET: u32 = 12;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ST_MASK: u32 = 0x00001000;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ST: u32 = 1 << 12;

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_OFFSET: u32 = 11;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_MASK: u32 = 0x00000800;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST: u32 = 1 << 11;

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_OFFSET: u32 = 10;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_MASK: u32 = 0x00000400;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST: u32 = 1 << 10;

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ST_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ST_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ST: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ST_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ST_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ST: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ST_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ST_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ST: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_PID_ERR_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ST_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ST_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ST: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SOF_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SOF_INT_ST_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SOF_INT_ST_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SOF_INT_ST: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST**
    ///
    /// The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST** `[15]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST** `[14]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_ST** `[13]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DTR_CHG_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_ST** `[12]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RTS_CHG_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST** `[11]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST** `[10]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST** `[9]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST** `[8]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_ST** `[7]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_STUFF_ERR_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_ST** `[6]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC16_ERR_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_ST** `[5]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC5_ERR_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_PID_ERR_INT_ST** `[4]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PID_ERR_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST** `[3]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SOF_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SOF_INT_ST_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_MASK) != 0
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

    /// Read **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST** `[15:15]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_set_line_code_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST** `[14:14]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_line_code_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DTR_CHG_INT_ST** `[13:13]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_dtr_chg_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_DTR_CHG_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RTS_CHG_INT_ST** `[12:12]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_rts_chg_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_RTS_CHG_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST** `[11:11]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_zero_payload_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST** `[10:10]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_zero_payload_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST** `[9:9]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_bus_reset_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST** `[8:8]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_token_rec_in_ep1_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_STUFF_ERR_INT_ST** `[7:7]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_stuff_err_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_STUFF_ERR_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC16_ERR_INT_ST** `[6:6]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc16_err_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_CRC16_ERR_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC5_ERR_INT_ST** `[5:5]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc5_err_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_CRC5_ERR_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PID_ERR_INT_ST** `[4:4]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_pid_err_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_PID_ERR_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST** `[3:3]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_empty_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_recv_pkt_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SOF_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_sof_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_SOF_INT_ST_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_jtag_in_flush_int_st(&self) -> bool {
        (self.bits & IntSt::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_MASK) != 0
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
/// Interrupt enable status register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA** `[15]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA** `[14]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_DTR_CHG_INT_ENA** `[13]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_RTS_CHG_INT_ENA** `[12]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA** `[11]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA** `[10]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA** `[9]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
///
/// - **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA** `[8]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
///
/// - **USB_SERIAL_JTAG_STUFF_ERR_INT_ENA** `[7]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC16_ERR_INT_ENA** `[6]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC5_ERR_INT_ENA** `[5]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_PID_ERR_INT_ENA** `[4]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SOF_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt.
///
/// - **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
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

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_OFFSET: u32 = 15;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK: u32 = 0x00008000;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA: u32 = 1 << 15;

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_OFFSET: u32 = 14;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK: u32 = 0x00004000;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA: u32 = 1 << 14;

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ENA_OFFSET: u32 = 13;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK: u32 = 0x00002000;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_ENA: u32 = 1 << 13;

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ENA_OFFSET: u32 = 12;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK: u32 = 0x00001000;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_ENA: u32 = 1 << 12;

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_OFFSET: u32 = 11;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK: u32 = 0x00000800;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA: u32 = 1 << 11;

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_OFFSET: u32 = 10;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK: u32 = 0x00000400;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA: u32 = 1 << 10;

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_ENA: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_ENA: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_ENA: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_PID_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ENA_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_ENA: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SOF_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SOF_INT_ENA_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SOF_INT_ENA_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SOF_INT_ENA: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA**
    ///
    /// The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA** `[15]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_set_line_code_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_set_line_code_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA** `[14]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_get_line_code_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_get_line_code_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_ENA** `[13]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DTR_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dtr_chg_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DTR_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dtr_chg_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_ENA** `[12]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_RTS_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_rts_chg_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_RTS_CHG_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_rts_chg_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA** `[11]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_out_ep2_zero_payload_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_out_ep2_zero_payload_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA** `[10]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_out_ep1_zero_payload_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_out_ep1_zero_payload_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA** `[9]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_bus_reset_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_bus_reset_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA** `[8]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_in_token_rec_in_ep1_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_in_token_rec_in_ep1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_ENA** `[7]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_STUFF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_stuff_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_STUFF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_stuff_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_ENA** `[6]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_CRC16_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_crc16_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_CRC16_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_crc16_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_ENA** `[5]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_CRC5_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_crc5_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_CRC5_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_crc5_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_PID_ERR_INT_ENA** `[4]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_PID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_pid_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_PID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_pid_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_in_empty_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_in_empty_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_out_recv_pkt_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_out_recv_pkt_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_SOF_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SOF_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_sof_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SOF_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_sof_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SOF_INT_ENA_MASK);
    }

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_jtag_in_flush_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK);
    }

    /// Clear USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_jtag_in_flush_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK);
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

    /// Read **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA** `[15:15]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_set_line_code_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA** `[14:14]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_line_code_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DTR_CHG_INT_ENA** `[13:13]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_dtr_chg_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RTS_CHG_INT_ENA** `[12:12]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_rts_chg_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA** `[11:11]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_zero_payload_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA** `[10:10]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_zero_payload_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA** `[9:9]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_bus_reset_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA** `[8:8]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_token_rec_in_ep1_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_STUFF_ERR_INT_ENA** `[7:7]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_stuff_err_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC16_ERR_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc16_err_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_CRC5_ERR_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_crc5_err_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PID_ERR_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_pid_err_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_empty_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_recv_pkt_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SOF_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_sof_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_SOF_INT_ENA_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub const fn usb_serial_jtag_jtag_in_flush_int_ena(&self) -> bool {
        (self.bits & IntEna::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA** `[15:15]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_set_line_code_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA** `[14:14]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_line_code_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DTR_CHG_INT_ENA** `[13:13]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dtr_chg_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_DTR_CHG_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_RTS_CHG_INT_ENA** `[12:12]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rts_chg_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_RTS_CHG_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA** `[11:11]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep2_zero_payload_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA** `[10:10]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep1_zero_payload_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA** `[9:9]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_bus_reset_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA** `[8:8]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_in_token_rec_in_ep1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_STUFF_ERR_INT_ENA** `[7:7]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_stuff_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC16_ERR_INT_ENA** `[6:6]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc16_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC5_ERR_INT_ENA** `[5:5]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc5_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PID_ERR_INT_ENA** `[4:4]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_pid_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_PID_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_in_empty_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_out_recv_pkt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SOF_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_sof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_SOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_SOF_INT_ENA_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_jtag_in_flush_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK
            } else {
                self.bits & !IntEna::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_MASK
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
/// Interrupt clear status register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR** `[15]` (WO)
///
///   Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR** `[14]` (WO)
///
///   Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt.
///
/// - **USB_SERIAL_JTAG_DTR_CHG_INT_CLR** `[13]` (WO)
///
///   Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_RTS_CHG_INT_CLR** `[12]` (WO)
///
///   Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR** `[11]` (WO)
///
///   Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR** `[10]` (WO)
///
///   Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
///
/// - **USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR** `[9]` (WO)
///
///   Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt.
///
/// - **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR** `[8]` (WO)
///
///   Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt.
///
/// - **USB_SERIAL_JTAG_STUFF_ERR_INT_CLR** `[7]` (WO)
///
///   Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC16_ERR_INT_CLR** `[6]` (WO)
///
///   Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_CRC5_ERR_INT_CLR** `[5]` (WO)
///
///   Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_PID_ERR_INT_CLR** `[4]` (WO)
///
///   Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
///
/// - **USB_SERIAL_JTAG_SOF_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt.
///
/// - **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
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

    /// **USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_OFFSET: u32 = 15;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_MASK: u32 = 0x00008000;
    pub const USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR: u32 = 1 << 15;

    /// **USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_OFFSET: u32 = 14;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_MASK: u32 = 0x00004000;
    pub const USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR: u32 = 1 << 14;

    /// **USB_SERIAL_JTAG_DTR_CHG_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_CLR_OFFSET: u32 = 13;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_CLR_MASK: u32 = 0x00002000;
    pub const USB_SERIAL_JTAG_DTR_CHG_INT_CLR: u32 = 1 << 13;

    /// **USB_SERIAL_JTAG_RTS_CHG_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_CLR_OFFSET: u32 = 12;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_CLR_MASK: u32 = 0x00001000;
    pub const USB_SERIAL_JTAG_RTS_CHG_INT_CLR: u32 = 1 << 12;

    /// **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_OFFSET: u32 = 11;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_MASK: u32 = 0x00000800;
    pub const USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR: u32 = 1 << 11;

    /// **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_OFFSET: u32 = 10;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_MASK: u32 = 0x00000400;
    pub const USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR: u32 = 1 << 10;

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_STUFF_ERR_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_STUFF_ERR_INT_CLR: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_CRC16_ERR_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_CRC16_ERR_INT_CLR: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_CRC5_ERR_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_CRC5_ERR_INT_CLR: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_PID_ERR_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_PID_ERR_INT_CLR_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_CLR_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_PID_ERR_INT_CLR: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SOF_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SOF_INT_CLR_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SOF_INT_CLR_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SOF_INT_CLR: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR**
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_set_line_code_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_get_line_code_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_DTR_CHG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_dtr_chg_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_DTR_CHG_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_RTS_CHG_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_rts_chg_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_RTS_CHG_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_out_ep2_zero_payload_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_out_ep1_zero_payload_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_usb_bus_reset_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_in_token_rec_in_ep1_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_STUFF_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_stuff_err_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_CRC16_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_crc16_err_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_CRC5_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_crc5_err_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_PID_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_pid_err_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_PID_ERR_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_serial_in_empty_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_serial_out_recv_pkt_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_SOF_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_sof_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_SOF_INT_CLR_MASK);
    }

    /// Trigger **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR** (WO)
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_jtag_in_flush_int_clr(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_MASK);
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

    /// Set **USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR** `[15:15]`
    ///
    /// Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_set_line_code_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR** `[14:14]`
    ///
    /// Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_line_code_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DTR_CHG_INT_CLR** `[13:13]`
    ///
    /// Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dtr_chg_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_DTR_CHG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_DTR_CHG_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_RTS_CHG_INT_CLR** `[12:12]`
    ///
    /// Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rts_chg_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_RTS_CHG_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_RTS_CHG_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR** `[11:11]`
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep2_zero_payload_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR** `[10:10]`
    ///
    /// Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_ep1_zero_payload_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR** `[9:9]`
    ///
    /// Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_bus_reset_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR** `[8:8]`
    ///
    /// Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_in_token_rec_in_ep1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_STUFF_ERR_INT_CLR** `[7:7]`
    ///
    /// Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_stuff_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC16_ERR_INT_CLR** `[6:6]`
    ///
    /// Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc16_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_CRC5_ERR_INT_CLR** `[5:5]`
    ///
    /// Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_crc5_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PID_ERR_INT_CLR** `[4:4]`
    ///
    /// Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_pid_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_PID_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_PID_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_in_empty_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_out_recv_pkt_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SOF_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_sof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_SOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_SOF_INT_CLR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_jtag_in_flush_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_MASK
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

/// CONF0 Register
///
/// PHY hardware configuration.
///
/// Address offset: 0x0018
/// Reset value: 0x00004200
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN** `[15]` (RW)
///
///   Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix.
///
/// - **USB_SERIAL_JTAG_USB_PAD_ENABLE** `[14]` (RW)
///
///   Enable USB pad function.
///
/// - **USB_SERIAL_JTAG_PULLUP_VALUE** `[13]` (RW)
///
///   Control pull up value.
///
/// - **USB_SERIAL_JTAG_DM_PULLDOWN** `[12]` (RW)
///
///   Control USB D- pull down.
///
/// - **USB_SERIAL_JTAG_DM_PULLUP** `[11]` (RW)
///
///   Control USB D- pull up.
///
/// - **USB_SERIAL_JTAG_DP_PULLDOWN** `[10]` (RW)
///
///   Control USB D+ pull down.
///
/// - **USB_SERIAL_JTAG_DP_PULLUP** `[9]` (RW)
///
///   Control USB D+ pull up.
///
/// - **USB_SERIAL_JTAG_PAD_PULL_OVERRIDE** `[8]` (RW)
///
///   Enable software control USB D+ D- pullup pulldown
///
/// - **USB_SERIAL_JTAG_VREF_OVERRIDE** `[7]` (RW)
///
///   Enable software control input threshold
///
/// - **USB_SERIAL_JTAG_VREFL** `[6:5]` (RW)
///
///   Control single-end input low threshold,0.8V to 1.04V, step 80mV
///
/// - **USB_SERIAL_JTAG_VREFH** `[4:3]` (RW)
///
///   Control single-end input high threshold,1.76V to 2V, step 80mV
///
/// - **USB_SERIAL_JTAG_EXCHG_PINS** `[2]` (RW)
///
///   USB D+ D- exchange
///
/// - **USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE** `[1]` (RW)
///
///   Enable software control USB D+ D- exchange
///
/// - **USB_SERIAL_JTAG_PHY_SEL** `[0]` (RW)
///
///   Select internal/external PHY
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

    /// **USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN**
    ///
    /// Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_OFFSET: u32 = 15;
    pub const USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK: u32 = 0x00008000;
    pub const USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN: u32 = 1 << 15;

    /// **USB_SERIAL_JTAG_USB_PAD_ENABLE**
    ///
    /// Enable USB pad function.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USB_SERIAL_JTAG_USB_PAD_ENABLE_OFFSET: u32 = 14;
    pub const USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK: u32 = 0x00004000;
    pub const USB_SERIAL_JTAG_USB_PAD_ENABLE: u32 = 1 << 14;

    /// **USB_SERIAL_JTAG_PULLUP_VALUE**
    ///
    /// Control pull up value.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USB_SERIAL_JTAG_PULLUP_VALUE_OFFSET: u32 = 13;
    pub const USB_SERIAL_JTAG_PULLUP_VALUE_MASK: u32 = 0x00002000;
    pub const USB_SERIAL_JTAG_PULLUP_VALUE: u32 = 1 << 13;

    /// **USB_SERIAL_JTAG_DM_PULLDOWN**
    ///
    /// Control USB D- pull down.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USB_SERIAL_JTAG_DM_PULLDOWN_OFFSET: u32 = 12;
    pub const USB_SERIAL_JTAG_DM_PULLDOWN_MASK: u32 = 0x00001000;
    pub const USB_SERIAL_JTAG_DM_PULLDOWN: u32 = 1 << 12;

    /// **USB_SERIAL_JTAG_DM_PULLUP**
    ///
    /// Control USB D- pull up.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USB_SERIAL_JTAG_DM_PULLUP_OFFSET: u32 = 11;
    pub const USB_SERIAL_JTAG_DM_PULLUP_MASK: u32 = 0x00000800;
    pub const USB_SERIAL_JTAG_DM_PULLUP: u32 = 1 << 11;

    /// **USB_SERIAL_JTAG_DP_PULLDOWN**
    ///
    /// Control USB D+ pull down.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USB_SERIAL_JTAG_DP_PULLDOWN_OFFSET: u32 = 10;
    pub const USB_SERIAL_JTAG_DP_PULLDOWN_MASK: u32 = 0x00000400;
    pub const USB_SERIAL_JTAG_DP_PULLDOWN: u32 = 1 << 10;

    /// **USB_SERIAL_JTAG_DP_PULLUP**
    ///
    /// Control USB D+ pull up.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_DP_PULLUP_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_DP_PULLUP_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_DP_PULLUP: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_PAD_PULL_OVERRIDE**
    ///
    /// Enable software control USB D+ D- pullup pulldown
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_PAD_PULL_OVERRIDE: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_VREF_OVERRIDE**
    ///
    /// Enable software control input threshold
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_VREF_OVERRIDE_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_VREF_OVERRIDE_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_VREF_OVERRIDE: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_VREFL**
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const USB_SERIAL_JTAG_VREFL_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_VREFL_MASK: u32 = 0x00000060;

    /// **USB_SERIAL_JTAG_VREFH**
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const USB_SERIAL_JTAG_VREFH_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_VREFH_MASK: u32 = 0x00000018;

    /// **USB_SERIAL_JTAG_EXCHG_PINS**
    ///
    /// USB D+ D- exchange
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_EXCHG_PINS_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_EXCHG_PINS_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_EXCHG_PINS: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE**
    ///
    /// Enable software control USB D+ D- exchange
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_PHY_SEL**
    ///
    /// Select internal/external PHY
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_PHY_SEL_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_PHY_SEL_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_PHY_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN** `[15]` (RW)
    ///
    /// Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_jtag_bridge_en(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_jtag_bridge_en(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_jtag_bridge_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK);
    }

    /// **USB_SERIAL_JTAG_USB_PAD_ENABLE** `[14]` (RW)
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_pad_enable(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_PAD_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_pad_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_PAD_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_pad_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK);
    }

    /// **USB_SERIAL_JTAG_PULLUP_VALUE** `[13]` (RW)
    ///
    /// Control pull up value.
    #[inline(always)]
    pub fn usb_serial_jtag_pullup_value(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PULLUP_VALUE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_PULLUP_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_pullup_value(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_PULLUP_VALUE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_PULLUP_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_pullup_value(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_PULLUP_VALUE_MASK);
    }

    /// **USB_SERIAL_JTAG_DM_PULLDOWN** `[12]` (RW)
    ///
    /// Control USB D- pull down.
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pulldown(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DM_PULLDOWN_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DM_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dm_pulldown(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DM_PULLDOWN_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DM_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dm_pulldown(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DM_PULLDOWN_MASK);
    }

    /// **USB_SERIAL_JTAG_DM_PULLUP** `[11]` (RW)
    ///
    /// Control USB D- pull up.
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pullup(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DM_PULLUP_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DM_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dm_pullup(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DM_PULLUP_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DM_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dm_pullup(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DM_PULLUP_MASK);
    }

    /// **USB_SERIAL_JTAG_DP_PULLDOWN** `[10]` (RW)
    ///
    /// Control USB D+ pull down.
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pulldown(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DP_PULLDOWN_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DP_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dp_pulldown(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DP_PULLDOWN_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DP_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dp_pulldown(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DP_PULLDOWN_MASK);
    }

    /// **USB_SERIAL_JTAG_DP_PULLUP** `[9]` (RW)
    ///
    /// Control USB D+ pull up.
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pullup(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DP_PULLUP_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_DP_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_dp_pullup(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_DP_PULLUP_MASK);
    }

    /// Clear USB_SERIAL_JTAG_DP_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_dp_pullup(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_DP_PULLUP_MASK);
    }

    /// **USB_SERIAL_JTAG_PAD_PULL_OVERRIDE** `[8]` (RW)
    ///
    /// Enable software control USB D+ D- pullup pulldown
    #[inline(always)]
    pub fn usb_serial_jtag_pad_pull_override(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_PAD_PULL_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_pad_pull_override(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_PAD_PULL_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_pad_pull_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK);
    }

    /// **USB_SERIAL_JTAG_VREF_OVERRIDE** `[7]` (RW)
    ///
    /// Enable software control input threshold
    #[inline(always)]
    pub fn usb_serial_jtag_vref_override(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_VREF_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_vref_override(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_VREF_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_vref_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK);
    }

    /// **USB_SERIAL_JTAG_VREFL** `[6:5]` (RW)
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV
    #[inline(always)]
    pub fn usb_serial_jtag_vrefl(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_VREFL_MASK)
            >> Self::USB_SERIAL_JTAG_VREFL_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_VREFL** `[6:5]` (RW)
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV
    #[inline(always)]
    pub fn write_usb_serial_jtag_vrefl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_VREFL_MASK)
                | ((value << Self::USB_SERIAL_JTAG_VREFL_OFFSET) & Self::USB_SERIAL_JTAG_VREFL_MASK)
        });
    }

    /// **USB_SERIAL_JTAG_VREFH** `[4:3]` (RW)
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV
    #[inline(always)]
    pub fn usb_serial_jtag_vrefh(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_VREFH_MASK)
            >> Self::USB_SERIAL_JTAG_VREFH_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_VREFH** `[4:3]` (RW)
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV
    #[inline(always)]
    pub fn write_usb_serial_jtag_vrefh(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_VREFH_MASK)
                | ((value << Self::USB_SERIAL_JTAG_VREFH_OFFSET) & Self::USB_SERIAL_JTAG_VREFH_MASK)
        });
    }

    /// **USB_SERIAL_JTAG_EXCHG_PINS** `[2]` (RW)
    ///
    /// USB D+ D- exchange
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_EXCHG_PINS_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_EXCHG_PINS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_exchg_pins(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_EXCHG_PINS_MASK);
    }

    /// Clear USB_SERIAL_JTAG_EXCHG_PINS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_exchg_pins(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_EXCHG_PINS_MASK);
    }

    /// **USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE** `[1]` (RW)
    ///
    /// Enable software control USB D+ D- exchange
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins_override(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_exchg_pins_override(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_exchg_pins_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK);
    }

    /// **USB_SERIAL_JTAG_PHY_SEL** `[0]` (RW)
    ///
    /// Select internal/external PHY
    #[inline(always)]
    pub fn usb_serial_jtag_phy_sel(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_PHY_SEL_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_phy_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_PHY_SEL_MASK);
    }

    /// Clear USB_SERIAL_JTAG_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_phy_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_PHY_SEL_MASK);
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
        Self { bits: 0x00004200 }
    }

    /// Read **USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN** `[15:15]`
    ///
    /// Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_jtag_bridge_en(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_USB_PAD_ENABLE** `[14:14]`
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_pad_enable(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PULLUP_VALUE** `[13:13]`
    ///
    /// Control pull up value.
    #[inline(always)]
    pub const fn usb_serial_jtag_pullup_value(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_PULLUP_VALUE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DM_PULLDOWN** `[12:12]`
    ///
    /// Control USB D- pull down.
    #[inline(always)]
    pub const fn usb_serial_jtag_dm_pulldown(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_DM_PULLDOWN_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DM_PULLUP** `[11:11]`
    ///
    /// Control USB D- pull up.
    #[inline(always)]
    pub const fn usb_serial_jtag_dm_pullup(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_DM_PULLUP_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DP_PULLDOWN** `[10:10]`
    ///
    /// Control USB D+ pull down.
    #[inline(always)]
    pub const fn usb_serial_jtag_dp_pulldown(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_DP_PULLDOWN_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DP_PULLUP** `[9:9]`
    ///
    /// Control USB D+ pull up.
    #[inline(always)]
    pub const fn usb_serial_jtag_dp_pullup(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_DP_PULLUP_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PAD_PULL_OVERRIDE** `[8:8]`
    ///
    /// Enable software control USB D+ D- pullup pulldown
    #[inline(always)]
    pub const fn usb_serial_jtag_pad_pull_override(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_VREF_OVERRIDE** `[7:7]`
    ///
    /// Enable software control input threshold
    #[inline(always)]
    pub const fn usb_serial_jtag_vref_override(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_VREFL** `[6:5]`
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV
    #[inline(always)]
    pub const fn usb_serial_jtag_vrefl(&self) -> u32 {
        (self.bits & Conf0::USB_SERIAL_JTAG_VREFL_MASK) >> Conf0::USB_SERIAL_JTAG_VREFL_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_VREFH** `[4:3]`
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV
    #[inline(always)]
    pub const fn usb_serial_jtag_vrefh(&self) -> u32 {
        (self.bits & Conf0::USB_SERIAL_JTAG_VREFH_MASK) >> Conf0::USB_SERIAL_JTAG_VREFH_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_EXCHG_PINS** `[2:2]`
    ///
    /// USB D+ D- exchange
    #[inline(always)]
    pub const fn usb_serial_jtag_exchg_pins(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_EXCHG_PINS_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE** `[1:1]`
    ///
    /// Enable software control USB D+ D- exchange
    #[inline(always)]
    pub const fn usb_serial_jtag_exchg_pins_override(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_PHY_SEL** `[0:0]`
    ///
    /// Select internal/external PHY
    #[inline(always)]
    pub const fn usb_serial_jtag_phy_sel(&self) -> bool {
        (self.bits & Conf0::USB_SERIAL_JTAG_PHY_SEL_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN** `[15:15]`
    ///
    /// Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_jtag_bridge_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_USB_PAD_ENABLE** `[14:14]`
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_pad_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_USB_PAD_ENABLE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PULLUP_VALUE** `[13:13]`
    ///
    /// Control pull up value.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_pullup_value(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_PULLUP_VALUE_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_PULLUP_VALUE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DM_PULLDOWN** `[12:12]`
    ///
    /// Control USB D- pull down.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dm_pulldown(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_DM_PULLDOWN_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_DM_PULLDOWN_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DM_PULLUP** `[11:11]`
    ///
    /// Control USB D- pull up.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dm_pullup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_DM_PULLUP_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_DM_PULLUP_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DP_PULLDOWN** `[10:10]`
    ///
    /// Control USB D+ pull down.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dp_pulldown(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_DP_PULLDOWN_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_DP_PULLDOWN_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_DP_PULLUP** `[9:9]`
    ///
    /// Control USB D+ pull up.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_dp_pullup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_DP_PULLUP_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_DP_PULLUP_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PAD_PULL_OVERRIDE** `[8:8]`
    ///
    /// Enable software control USB D+ D- pullup pulldown
    #[inline(always)]
    pub const fn with_usb_serial_jtag_pad_pull_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_VREF_OVERRIDE** `[7:7]`
    ///
    /// Enable software control input threshold
    #[inline(always)]
    pub const fn with_usb_serial_jtag_vref_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_VREF_OVERRIDE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_VREFL** `[6:5]`
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV
    #[inline(always)]
    pub const fn with_usb_serial_jtag_vrefl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf0::USB_SERIAL_JTAG_VREFL_MASK)
                | ((value << Conf0::USB_SERIAL_JTAG_VREFL_OFFSET)
                    & Conf0::USB_SERIAL_JTAG_VREFL_MASK),
        }
    }

    /// Set **USB_SERIAL_JTAG_VREFH** `[4:3]`
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV
    #[inline(always)]
    pub const fn with_usb_serial_jtag_vrefh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf0::USB_SERIAL_JTAG_VREFH_MASK)
                | ((value << Conf0::USB_SERIAL_JTAG_VREFH_OFFSET)
                    & Conf0::USB_SERIAL_JTAG_VREFH_MASK),
        }
    }

    /// Set **USB_SERIAL_JTAG_EXCHG_PINS** `[2:2]`
    ///
    /// USB D+ D- exchange
    #[inline(always)]
    pub const fn with_usb_serial_jtag_exchg_pins(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_EXCHG_PINS_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_EXCHG_PINS_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE** `[1:1]`
    ///
    /// Enable software control USB D+ D- exchange
    #[inline(always)]
    pub const fn with_usb_serial_jtag_exchg_pins_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_PHY_SEL** `[0:0]`
    ///
    /// Select internal/external PHY
    #[inline(always)]
    pub const fn with_usb_serial_jtag_phy_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf0::USB_SERIAL_JTAG_PHY_SEL_MASK
            } else {
                self.bits & !Conf0::USB_SERIAL_JTAG_PHY_SEL_MASK
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

/// TEST Register
///
/// Registers used for debugging the PHY.
///
/// Address offset: 0x001c
/// Reset value: 0x00000030
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_TEST_RX_DM** `[6]` (RO)
///
///   USB D- rx value in test
///
/// - **USB_SERIAL_JTAG_TEST_RX_DP** `[5]` (RO)
///
///   USB D+ rx value in test
///
/// - **USB_SERIAL_JTAG_TEST_RX_RCV** `[4]` (RO)
///
///   USB RCV value in test
///
/// - **USB_SERIAL_JTAG_TEST_TX_DM** `[3]` (RW)
///
///   USB D- tx value in test
///
/// - **USB_SERIAL_JTAG_TEST_TX_DP** `[2]` (RW)
///
///   USB D+ tx value in test
///
/// - **USB_SERIAL_JTAG_TEST_USB_OE** `[1]` (RW)
///
///   USB pad oen in test
///
/// - **USB_SERIAL_JTAG_TEST_ENABLE** `[0]` (RW)
///
///   Enable test of the USB pad
#[repr(transparent)]
pub struct Test {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Test {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Test {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_TEST_RX_DM**
    ///
    /// USB D- rx value in test
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_TEST_RX_DM_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_TEST_RX_DM_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_TEST_RX_DM: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_TEST_RX_DP**
    ///
    /// USB D+ rx value in test
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_TEST_RX_DP_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_TEST_RX_DP_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_TEST_RX_DP: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_TEST_RX_RCV**
    ///
    /// USB RCV value in test
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_TEST_RX_RCV_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_TEST_RX_RCV_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_TEST_RX_RCV: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_TEST_TX_DM**
    ///
    /// USB D- tx value in test
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_TEST_TX_DM_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_TEST_TX_DM_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_TEST_TX_DM: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_TEST_TX_DP**
    ///
    /// USB D+ tx value in test
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_TEST_TX_DP_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_TEST_TX_DP_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_TEST_TX_DP: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_TEST_USB_OE**
    ///
    /// USB pad oen in test
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_TEST_USB_OE_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_TEST_USB_OE_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_TEST_USB_OE: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_TEST_ENABLE**
    ///
    /// Enable test of the USB pad
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_TEST_ENABLE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_TEST_ENABLE_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_TEST_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_TEST_RX_DM** `[6]` (RO)
    ///
    /// USB D- rx value in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_RX_DM_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_TEST_RX_DP** `[5]` (RO)
    ///
    /// USB D+ rx value in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_RX_DP_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_TEST_RX_RCV** `[4]` (RO)
    ///
    /// USB RCV value in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_rcv(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_RX_RCV_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_TEST_TX_DM** `[3]` (RW)
    ///
    /// USB D- tx value in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_TX_DM_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_TEST_TX_DM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_test_tx_dm(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_TEST_TX_DM_MASK);
    }

    /// Clear USB_SERIAL_JTAG_TEST_TX_DM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_test_tx_dm(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_TEST_TX_DM_MASK);
    }

    /// **USB_SERIAL_JTAG_TEST_TX_DP** `[2]` (RW)
    ///
    /// USB D+ tx value in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_TX_DP_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_TEST_TX_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_test_tx_dp(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_TEST_TX_DP_MASK);
    }

    /// Clear USB_SERIAL_JTAG_TEST_TX_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_test_tx_dp(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_TEST_TX_DP_MASK);
    }

    /// **USB_SERIAL_JTAG_TEST_USB_OE** `[1]` (RW)
    ///
    /// USB pad oen in test
    #[inline(always)]
    pub fn usb_serial_jtag_test_usb_oe(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_USB_OE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_TEST_USB_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_test_usb_oe(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_TEST_USB_OE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_TEST_USB_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_test_usb_oe(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_TEST_USB_OE_MASK);
    }

    /// **USB_SERIAL_JTAG_TEST_ENABLE** `[0]` (RW)
    ///
    /// Enable test of the USB pad
    #[inline(always)]
    pub fn usb_serial_jtag_test_enable(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_TEST_ENABLE_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_TEST_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_test_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_TEST_ENABLE_MASK);
    }

    /// Clear USB_SERIAL_JTAG_TEST_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_test_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_TEST_ENABLE_MASK);
    }
}

/// TEST value type for batch read/modify/write operations
///
/// Use `test.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `test.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TestVal {
    bits: u32,
}

impl TestVal {
    /// Create from raw bits
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
        Self { bits: 0x00000030 }
    }

    /// Read **USB_SERIAL_JTAG_TEST_RX_DM** `[6:6]`
    ///
    /// USB D- rx value in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_rx_dm(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_RX_DM_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_RX_DP** `[5:5]`
    ///
    /// USB D+ rx value in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_rx_dp(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_RX_DP_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_RX_RCV** `[4:4]`
    ///
    /// USB RCV value in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_rx_rcv(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_RX_RCV_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_TX_DM** `[3:3]`
    ///
    /// USB D- tx value in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_tx_dm(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_TX_DM_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_TX_DP** `[2:2]`
    ///
    /// USB D+ tx value in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_tx_dp(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_TX_DP_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_USB_OE** `[1:1]`
    ///
    /// USB pad oen in test
    #[inline(always)]
    pub const fn usb_serial_jtag_test_usb_oe(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_USB_OE_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_TEST_ENABLE** `[0:0]`
    ///
    /// Enable test of the USB pad
    #[inline(always)]
    pub const fn usb_serial_jtag_test_enable(&self) -> bool {
        (self.bits & Test::USB_SERIAL_JTAG_TEST_ENABLE_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_TEST_TX_DM** `[3:3]`
    ///
    /// USB D- tx value in test
    #[inline(always)]
    pub const fn with_usb_serial_jtag_test_tx_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Test::USB_SERIAL_JTAG_TEST_TX_DM_MASK
            } else {
                self.bits & !Test::USB_SERIAL_JTAG_TEST_TX_DM_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_TEST_TX_DP** `[2:2]`
    ///
    /// USB D+ tx value in test
    #[inline(always)]
    pub const fn with_usb_serial_jtag_test_tx_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Test::USB_SERIAL_JTAG_TEST_TX_DP_MASK
            } else {
                self.bits & !Test::USB_SERIAL_JTAG_TEST_TX_DP_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_TEST_USB_OE** `[1:1]`
    ///
    /// USB pad oen in test
    #[inline(always)]
    pub const fn with_usb_serial_jtag_test_usb_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Test::USB_SERIAL_JTAG_TEST_USB_OE_MASK
            } else {
                self.bits & !Test::USB_SERIAL_JTAG_TEST_USB_OE_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_TEST_ENABLE** `[0:0]`
    ///
    /// Enable test of the USB pad
    #[inline(always)]
    pub const fn with_usb_serial_jtag_test_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Test::USB_SERIAL_JTAG_TEST_ENABLE_MASK
            } else {
                self.bits & !Test::USB_SERIAL_JTAG_TEST_ENABLE_MASK
            },
        }
    }
}

impl Default for TestVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Test {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TestVal {
        TestVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TestVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TestVal) -> TestVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// JFIFO_ST Register
///
/// JTAG FIFO status and control registers.
///
/// Address offset: 0x0020
/// Reset value: 0x00000044
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_OUT_FIFO_RESET** `[9]` (RW)
///
///   Write 1 to reset JTAG out fifo.
///
/// - **USB_SERIAL_JTAG_IN_FIFO_RESET** `[8]` (RW)
///
///   Write 1 to reset JTAG in fifo.
///
/// - **USB_SERIAL_JTAG_OUT_FIFO_FULL** `[7]` (RO)
///
///   1: JTAG out fifo is full.
///
/// - **USB_SERIAL_JTAG_OUT_FIFO_EMPTY** `[6]` (RO)
///
///   1: JTAG out fifo is empty.
///
/// - **USB_SERIAL_JTAG_OUT_FIFO_CNT** `[5:4]` (RO)
///
///   JTAT out fifo counter.
///
/// - **USB_SERIAL_JTAG_IN_FIFO_FULL** `[3]` (RO)
///
///   1: JTAG in fifo is full.
///
/// - **USB_SERIAL_JTAG_IN_FIFO_EMPTY** `[2]` (RO)
///
///   1: JTAG in fifo is empty.
///
/// - **USB_SERIAL_JTAG_IN_FIFO_CNT** `[1:0]` (RO)
///
///   JTAT in fifo counter.
#[repr(transparent)]
pub struct JfifoSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for JfifoSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl JfifoSt {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_OUT_FIFO_RESET**
    ///
    /// Write 1 to reset JTAG out fifo.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USB_SERIAL_JTAG_OUT_FIFO_RESET_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK: u32 = 0x00000200;
    pub const USB_SERIAL_JTAG_OUT_FIFO_RESET: u32 = 1 << 9;

    /// **USB_SERIAL_JTAG_IN_FIFO_RESET**
    ///
    /// Write 1 to reset JTAG in fifo.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_SERIAL_JTAG_IN_FIFO_RESET_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_IN_FIFO_RESET_MASK: u32 = 0x00000100;
    pub const USB_SERIAL_JTAG_IN_FIFO_RESET: u32 = 1 << 8;

    /// **USB_SERIAL_JTAG_OUT_FIFO_FULL**
    ///
    /// 1: JTAG out fifo is full.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_SERIAL_JTAG_OUT_FIFO_FULL_OFFSET: u32 = 7;
    pub const USB_SERIAL_JTAG_OUT_FIFO_FULL_MASK: u32 = 0x00000080;
    pub const USB_SERIAL_JTAG_OUT_FIFO_FULL: u32 = 1 << 7;

    /// **USB_SERIAL_JTAG_OUT_FIFO_EMPTY**
    ///
    /// 1: JTAG out fifo is empty.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_SERIAL_JTAG_OUT_FIFO_EMPTY_OFFSET: u32 = 6;
    pub const USB_SERIAL_JTAG_OUT_FIFO_EMPTY_MASK: u32 = 0x00000040;
    pub const USB_SERIAL_JTAG_OUT_FIFO_EMPTY: u32 = 1 << 6;

    /// **USB_SERIAL_JTAG_OUT_FIFO_CNT**
    ///
    /// JTAT out fifo counter.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const USB_SERIAL_JTAG_OUT_FIFO_CNT_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_OUT_FIFO_CNT_MASK: u32 = 0x00000030;

    /// **USB_SERIAL_JTAG_IN_FIFO_FULL**
    ///
    /// 1: JTAG in fifo is full.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_IN_FIFO_FULL_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_IN_FIFO_FULL_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_IN_FIFO_FULL: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_IN_FIFO_EMPTY**
    ///
    /// 1: JTAG in fifo is empty.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_IN_FIFO_EMPTY_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_IN_FIFO_EMPTY_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_IN_FIFO_EMPTY: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_IN_FIFO_CNT**
    ///
    /// JTAT in fifo counter.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_IN_FIFO_CNT_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_IN_FIFO_CNT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_OUT_FIFO_RESET** `[9]` (RW)
    ///
    /// Write 1 to reset JTAG out fifo.
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_reset(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_OUT_FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_out_fifo_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK);
    }

    /// Clear USB_SERIAL_JTAG_OUT_FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_out_fifo_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK);
    }

    /// **USB_SERIAL_JTAG_IN_FIFO_RESET** `[8]` (RW)
    ///
    /// Write 1 to reset JTAG in fifo.
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_reset(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_IN_FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_in_fifo_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK);
    }

    /// Clear USB_SERIAL_JTAG_IN_FIFO_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_in_fifo_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK);
    }

    /// **USB_SERIAL_JTAG_OUT_FIFO_FULL** `[7]` (RO)
    ///
    /// 1: JTAG out fifo is full.
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_full(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_FIFO_FULL_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_OUT_FIFO_EMPTY** `[6]` (RO)
    ///
    /// 1: JTAG out fifo is empty.
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_FIFO_EMPTY_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_OUT_FIFO_CNT** `[5:4]` (RO)
    ///
    /// JTAT out fifo counter.
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_FIFO_CNT_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_FIFO_CNT_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_FIFO_FULL** `[3]` (RO)
    ///
    /// 1: JTAG in fifo is full.
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_full(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_FIFO_FULL_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_IN_FIFO_EMPTY** `[2]` (RO)
    ///
    /// 1: JTAG in fifo is empty.
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_FIFO_EMPTY_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_IN_FIFO_CNT** `[1:0]` (RO)
    ///
    /// JTAT in fifo counter.
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_FIFO_CNT_MASK)
            >> Self::USB_SERIAL_JTAG_IN_FIFO_CNT_OFFSET
    }
}

/// JFIFO_ST value type for batch read/modify/write operations
///
/// Use `jfifost.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `jfifost.write_val(val)`.
#[derive(Clone, Copy)]
pub struct JfifoStVal {
    bits: u32,
}

impl JfifoStVal {
    /// Create from raw bits
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
        Self { bits: 0x00000044 }
    }

    /// Read **USB_SERIAL_JTAG_OUT_FIFO_RESET** `[9:9]`
    ///
    /// Write 1 to reset JTAG out fifo.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_fifo_reset(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_FIFO_RESET** `[8:8]`
    ///
    /// Write 1 to reset JTAG in fifo.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_fifo_reset(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_FIFO_FULL** `[7:7]`
    ///
    /// 1: JTAG out fifo is full.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_fifo_full(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_FULL_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_FIFO_EMPTY** `[6:6]`
    ///
    /// 1: JTAG out fifo is empty.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_fifo_empty(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_EMPTY_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_OUT_FIFO_CNT** `[5:4]`
    ///
    /// JTAT out fifo counter.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_fifo_cnt(&self) -> u32 {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_CNT_MASK)
            >> JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_CNT_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_FIFO_FULL** `[3:3]`
    ///
    /// 1: JTAG in fifo is full.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_fifo_full(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_IN_FIFO_FULL_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_FIFO_EMPTY** `[2:2]`
    ///
    /// 1: JTAG in fifo is empty.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_fifo_empty(&self) -> bool {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_IN_FIFO_EMPTY_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_IN_FIFO_CNT** `[1:0]`
    ///
    /// JTAT in fifo counter.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_fifo_cnt(&self) -> u32 {
        (self.bits & JfifoSt::USB_SERIAL_JTAG_IN_FIFO_CNT_MASK)
            >> JfifoSt::USB_SERIAL_JTAG_IN_FIFO_CNT_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_OUT_FIFO_RESET** `[9:9]`
    ///
    /// Write 1 to reset JTAG out fifo.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_out_fifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK
            } else {
                self.bits & !JfifoSt::USB_SERIAL_JTAG_OUT_FIFO_RESET_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_IN_FIFO_RESET** `[8:8]`
    ///
    /// Write 1 to reset JTAG in fifo.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_in_fifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | JfifoSt::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK
            } else {
                self.bits & !JfifoSt::USB_SERIAL_JTAG_IN_FIFO_RESET_MASK
            },
        }
    }
}

impl Default for JfifoStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl JfifoSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> JfifoStVal {
        JfifoStVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: JfifoStVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(JfifoStVal) -> JfifoStVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FRAM_NUM Register
///
/// Last received SOF frame index register.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SOF_FRAME_INDEX** `[10:0]` (RO)
///
///   Frame index of received SOF frame.
#[repr(transparent)]
pub struct FramNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FramNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FramNum {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_SOF_FRAME_INDEX**
    ///
    /// Frame index of received SOF frame.
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const USB_SERIAL_JTAG_SOF_FRAME_INDEX_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_SOF_FRAME_INDEX_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SOF_FRAME_INDEX** `[10:0]` (RO)
    ///
    /// Frame index of received SOF frame.
    #[inline(always)]
    pub fn usb_serial_jtag_sof_frame_index(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SOF_FRAME_INDEX_MASK)
            >> Self::USB_SERIAL_JTAG_SOF_FRAME_INDEX_OFFSET
    }
}

/// FRAM_NUM value type for batch read/modify/write operations
///
/// Use `framnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `framnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FramNumVal {
    bits: u32,
}

impl FramNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_SOF_FRAME_INDEX** `[10:0]`
    ///
    /// Frame index of received SOF frame.
    #[inline(always)]
    pub const fn usb_serial_jtag_sof_frame_index(&self) -> u32 {
        (self.bits & FramNum::USB_SERIAL_JTAG_SOF_FRAME_INDEX_MASK)
            >> FramNum::USB_SERIAL_JTAG_SOF_FRAME_INDEX_OFFSET
    }
}

impl Default for FramNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FramNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FramNumVal {
        FramNumVal::from_bits(self.value.read_bits())
    }
}

/// IN_EP0_ST Register
///
/// Control IN endpoint status information.
///
/// Address offset: 0x0028
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_IN_EP0_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of IN endpoint 0.
///
/// - **USB_SERIAL_JTAG_IN_EP0_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of IN endpoint 0.
///
/// - **USB_SERIAL_JTAG_IN_EP0_STATE** `[1:0]` (RO)
///
///   State of IN Endpoint 0.
#[repr(transparent)]
pub struct InEp0St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InEp0St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InEp0St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_IN_EP0_RD_ADDR**
    ///
    /// Read data address of IN endpoint 0.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_IN_EP0_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_IN_EP0_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_IN_EP0_WR_ADDR**
    ///
    /// Write data address of IN endpoint 0.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_IN_EP0_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_IN_EP0_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_IN_EP0_STATE**
    ///
    /// State of IN Endpoint 0.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_IN_EP0_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_IN_EP0_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_IN_EP0_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of IN endpoint 0.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP0_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP0_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP0_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of IN endpoint 0.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP0_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP0_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP0_STATE** `[1:0]` (RO)
    ///
    /// State of IN Endpoint 0.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP0_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP0_STATE_OFFSET
    }
}

/// IN_EP0_ST value type for batch read/modify/write operations
///
/// Use `inep0st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inep0st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InEp0StVal {
    bits: u32,
}

impl InEp0StVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_IN_EP0_RD_ADDR** `[15:9]`
    ///
    /// Read data address of IN endpoint 0.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep0_rd_addr(&self) -> u32 {
        (self.bits & InEp0St::USB_SERIAL_JTAG_IN_EP0_RD_ADDR_MASK)
            >> InEp0St::USB_SERIAL_JTAG_IN_EP0_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP0_WR_ADDR** `[8:2]`
    ///
    /// Write data address of IN endpoint 0.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep0_wr_addr(&self) -> u32 {
        (self.bits & InEp0St::USB_SERIAL_JTAG_IN_EP0_WR_ADDR_MASK)
            >> InEp0St::USB_SERIAL_JTAG_IN_EP0_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP0_STATE** `[1:0]`
    ///
    /// State of IN Endpoint 0.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep0_state(&self) -> u32 {
        (self.bits & InEp0St::USB_SERIAL_JTAG_IN_EP0_STATE_MASK)
            >> InEp0St::USB_SERIAL_JTAG_IN_EP0_STATE_OFFSET
    }
}

impl Default for InEp0StVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InEp0St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InEp0StVal {
        InEp0StVal::from_bits(self.value.read_bits())
    }
}

/// IN_EP1_ST Register
///
/// CDC-ACM IN endpoint status information.
///
/// Address offset: 0x002c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_IN_EP1_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of IN endpoint 1.
///
/// - **USB_SERIAL_JTAG_IN_EP1_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of IN endpoint 1.
///
/// - **USB_SERIAL_JTAG_IN_EP1_STATE** `[1:0]` (RO)
///
///   State of IN Endpoint 1.
#[repr(transparent)]
pub struct InEp1St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InEp1St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InEp1St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_IN_EP1_RD_ADDR**
    ///
    /// Read data address of IN endpoint 1.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_IN_EP1_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_IN_EP1_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_IN_EP1_WR_ADDR**
    ///
    /// Write data address of IN endpoint 1.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_IN_EP1_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_IN_EP1_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_IN_EP1_STATE**
    ///
    /// State of IN Endpoint 1.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_IN_EP1_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_IN_EP1_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_IN_EP1_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of IN endpoint 1.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep1_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP1_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP1_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP1_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of IN endpoint 1.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep1_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP1_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP1_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP1_STATE** `[1:0]` (RO)
    ///
    /// State of IN Endpoint 1.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep1_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP1_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP1_STATE_OFFSET
    }
}

/// IN_EP1_ST value type for batch read/modify/write operations
///
/// Use `inep1st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inep1st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InEp1StVal {
    bits: u32,
}

impl InEp1StVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_IN_EP1_RD_ADDR** `[15:9]`
    ///
    /// Read data address of IN endpoint 1.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep1_rd_addr(&self) -> u32 {
        (self.bits & InEp1St::USB_SERIAL_JTAG_IN_EP1_RD_ADDR_MASK)
            >> InEp1St::USB_SERIAL_JTAG_IN_EP1_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP1_WR_ADDR** `[8:2]`
    ///
    /// Write data address of IN endpoint 1.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep1_wr_addr(&self) -> u32 {
        (self.bits & InEp1St::USB_SERIAL_JTAG_IN_EP1_WR_ADDR_MASK)
            >> InEp1St::USB_SERIAL_JTAG_IN_EP1_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP1_STATE** `[1:0]`
    ///
    /// State of IN Endpoint 1.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep1_state(&self) -> u32 {
        (self.bits & InEp1St::USB_SERIAL_JTAG_IN_EP1_STATE_MASK)
            >> InEp1St::USB_SERIAL_JTAG_IN_EP1_STATE_OFFSET
    }
}

impl Default for InEp1StVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InEp1St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InEp1StVal {
        InEp1StVal::from_bits(self.value.read_bits())
    }
}

/// IN_EP2_ST Register
///
/// CDC-ACM interrupt IN endpoint status information.
///
/// Address offset: 0x0030
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_IN_EP2_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of IN endpoint 2.
///
/// - **USB_SERIAL_JTAG_IN_EP2_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of IN endpoint 2.
///
/// - **USB_SERIAL_JTAG_IN_EP2_STATE** `[1:0]` (RO)
///
///   State of IN Endpoint 2.
#[repr(transparent)]
pub struct InEp2St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InEp2St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InEp2St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_IN_EP2_RD_ADDR**
    ///
    /// Read data address of IN endpoint 2.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_IN_EP2_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_IN_EP2_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_IN_EP2_WR_ADDR**
    ///
    /// Write data address of IN endpoint 2.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_IN_EP2_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_IN_EP2_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_IN_EP2_STATE**
    ///
    /// State of IN Endpoint 2.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_IN_EP2_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_IN_EP2_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_IN_EP2_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of IN endpoint 2.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep2_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP2_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP2_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP2_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of IN endpoint 2.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep2_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP2_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP2_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP2_STATE** `[1:0]` (RO)
    ///
    /// State of IN Endpoint 2.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep2_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP2_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP2_STATE_OFFSET
    }
}

/// IN_EP2_ST value type for batch read/modify/write operations
///
/// Use `inep2st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inep2st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InEp2StVal {
    bits: u32,
}

impl InEp2StVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_IN_EP2_RD_ADDR** `[15:9]`
    ///
    /// Read data address of IN endpoint 2.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep2_rd_addr(&self) -> u32 {
        (self.bits & InEp2St::USB_SERIAL_JTAG_IN_EP2_RD_ADDR_MASK)
            >> InEp2St::USB_SERIAL_JTAG_IN_EP2_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP2_WR_ADDR** `[8:2]`
    ///
    /// Write data address of IN endpoint 2.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep2_wr_addr(&self) -> u32 {
        (self.bits & InEp2St::USB_SERIAL_JTAG_IN_EP2_WR_ADDR_MASK)
            >> InEp2St::USB_SERIAL_JTAG_IN_EP2_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP2_STATE** `[1:0]`
    ///
    /// State of IN Endpoint 2.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep2_state(&self) -> u32 {
        (self.bits & InEp2St::USB_SERIAL_JTAG_IN_EP2_STATE_MASK)
            >> InEp2St::USB_SERIAL_JTAG_IN_EP2_STATE_OFFSET
    }
}

impl Default for InEp2StVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InEp2St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InEp2StVal {
        InEp2StVal::from_bits(self.value.read_bits())
    }
}

/// IN_EP3_ST Register
///
/// JTAG IN endpoint status information.
///
/// Address offset: 0x0034
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_IN_EP3_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of IN endpoint 3.
///
/// - **USB_SERIAL_JTAG_IN_EP3_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of IN endpoint 3.
///
/// - **USB_SERIAL_JTAG_IN_EP3_STATE** `[1:0]` (RO)
///
///   State of IN Endpoint 3.
#[repr(transparent)]
pub struct InEp3St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InEp3St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InEp3St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_IN_EP3_RD_ADDR**
    ///
    /// Read data address of IN endpoint 3.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_IN_EP3_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_IN_EP3_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_IN_EP3_WR_ADDR**
    ///
    /// Write data address of IN endpoint 3.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_IN_EP3_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_IN_EP3_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_IN_EP3_STATE**
    ///
    /// State of IN Endpoint 3.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_IN_EP3_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_IN_EP3_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_IN_EP3_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of IN endpoint 3.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep3_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP3_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP3_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP3_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of IN endpoint 3.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep3_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP3_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP3_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_IN_EP3_STATE** `[1:0]` (RO)
    ///
    /// State of IN Endpoint 3.
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep3_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_IN_EP3_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_IN_EP3_STATE_OFFSET
    }
}

/// IN_EP3_ST value type for batch read/modify/write operations
///
/// Use `inep3st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inep3st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InEp3StVal {
    bits: u32,
}

impl InEp3StVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_IN_EP3_RD_ADDR** `[15:9]`
    ///
    /// Read data address of IN endpoint 3.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep3_rd_addr(&self) -> u32 {
        (self.bits & InEp3St::USB_SERIAL_JTAG_IN_EP3_RD_ADDR_MASK)
            >> InEp3St::USB_SERIAL_JTAG_IN_EP3_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP3_WR_ADDR** `[8:2]`
    ///
    /// Write data address of IN endpoint 3.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep3_wr_addr(&self) -> u32 {
        (self.bits & InEp3St::USB_SERIAL_JTAG_IN_EP3_WR_ADDR_MASK)
            >> InEp3St::USB_SERIAL_JTAG_IN_EP3_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_IN_EP3_STATE** `[1:0]`
    ///
    /// State of IN Endpoint 3.
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep3_state(&self) -> u32 {
        (self.bits & InEp3St::USB_SERIAL_JTAG_IN_EP3_STATE_MASK)
            >> InEp3St::USB_SERIAL_JTAG_IN_EP3_STATE_OFFSET
    }
}

impl Default for InEp3StVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InEp3St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InEp3StVal {
        InEp3StVal::from_bits(self.value.read_bits())
    }
}

/// OUT_EP0_ST Register
///
/// Control OUT endpoint status information.
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_OUT_EP0_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of OUT endpoint 0.
///
/// - **USB_SERIAL_JTAG_OUT_EP0_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0.
///
/// - **USB_SERIAL_JTAG_OUT_EP0_STATE** `[1:0]` (RO)
///
///   State of OUT Endpoint 0.
#[repr(transparent)]
pub struct OutEp0St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEp0St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEp0St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_OUT_EP0_RD_ADDR**
    ///
    /// Read data address of OUT endpoint 0.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_OUT_EP0_WR_ADDR**
    ///
    /// Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_OUT_EP0_STATE**
    ///
    /// State of OUT Endpoint 0.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_OUT_EP0_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_OUT_EP0_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_OUT_EP0_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of OUT endpoint 0.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep0_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP0_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep0_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP0_STATE** `[1:0]` (RO)
    ///
    /// State of OUT Endpoint 0.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep0_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP0_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP0_STATE_OFFSET
    }
}

/// OUT_EP0_ST value type for batch read/modify/write operations
///
/// Use `outep0st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outep0st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEp0StVal {
    bits: u32,
}

impl OutEp0StVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP0_RD_ADDR** `[15:9]`
    ///
    /// Read data address of OUT endpoint 0.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep0_rd_addr(&self) -> u32 {
        (self.bits & OutEp0St::USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_MASK)
            >> OutEp0St::USB_SERIAL_JTAG_OUT_EP0_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP0_WR_ADDR** `[8:2]`
    ///
    /// Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep0_wr_addr(&self) -> u32 {
        (self.bits & OutEp0St::USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_MASK)
            >> OutEp0St::USB_SERIAL_JTAG_OUT_EP0_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP0_STATE** `[1:0]`
    ///
    /// State of OUT Endpoint 0.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep0_state(&self) -> u32 {
        (self.bits & OutEp0St::USB_SERIAL_JTAG_OUT_EP0_STATE_MASK)
            >> OutEp0St::USB_SERIAL_JTAG_OUT_EP0_STATE_OFFSET
    }
}

impl Default for OutEp0StVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEp0St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEp0StVal {
        OutEp0StVal::from_bits(self.value.read_bits())
    }
}

/// OUT_EP1_ST Register
///
/// CDC-ACM OUT endpoint status information.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT** `[22:16]` (RO)
///
///   Data count in OUT endpoint 1 when one packet is received.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of OUT endpoint 1.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1.
///
/// - **USB_SERIAL_JTAG_OUT_EP1_STATE** `[1:0]` (RO)
///
///   State of OUT Endpoint 1.
#[repr(transparent)]
pub struct OutEp1St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEp1St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEp1St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT**
    ///
    /// Data count in OUT endpoint 1 when one packet is received.
    ///
    /// Bits `[22:16]`, mask `0x007f0000`
    pub const USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_OFFSET: u32 = 16;
    pub const USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_MASK: u32 = 0x007f0000;

    /// **USB_SERIAL_JTAG_OUT_EP1_RD_ADDR**
    ///
    /// Read data address of OUT endpoint 1.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_OUT_EP1_WR_ADDR**
    ///
    /// Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_OUT_EP1_STATE**
    ///
    /// State of OUT Endpoint 1.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_OUT_EP1_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_OUT_EP1_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT** `[22:16]` (RO)
    ///
    /// Data count in OUT endpoint 1 when one packet is received.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_rec_data_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of OUT endpoint 1.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP1_STATE** `[1:0]` (RO)
    ///
    /// State of OUT Endpoint 1.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP1_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP1_STATE_OFFSET
    }
}

/// OUT_EP1_ST value type for batch read/modify/write operations
///
/// Use `outep1st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outep1st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEp1StVal {
    bits: u32,
}

impl OutEp1StVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT** `[22:16]`
    ///
    /// Data count in OUT endpoint 1 when one packet is received.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_rec_data_cnt(&self) -> u32 {
        (self.bits & OutEp1St::USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_MASK)
            >> OutEp1St::USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_RD_ADDR** `[15:9]`
    ///
    /// Read data address of OUT endpoint 1.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_rd_addr(&self) -> u32 {
        (self.bits & OutEp1St::USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_MASK)
            >> OutEp1St::USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_WR_ADDR** `[8:2]`
    ///
    /// Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_wr_addr(&self) -> u32 {
        (self.bits & OutEp1St::USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_MASK)
            >> OutEp1St::USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP1_STATE** `[1:0]`
    ///
    /// State of OUT Endpoint 1.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_state(&self) -> u32 {
        (self.bits & OutEp1St::USB_SERIAL_JTAG_OUT_EP1_STATE_MASK)
            >> OutEp1St::USB_SERIAL_JTAG_OUT_EP1_STATE_OFFSET
    }
}

impl Default for OutEp1StVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEp1St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEp1StVal {
        OutEp1StVal::from_bits(self.value.read_bits())
    }
}

/// OUT_EP2_ST Register
///
/// JTAG OUT endpoint status information.
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_OUT_EP2_RD_ADDR** `[15:9]` (RO)
///
///   Read data address of OUT endpoint 2.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_WR_ADDR** `[8:2]` (RO)
///
///   Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2.
///
/// - **USB_SERIAL_JTAG_OUT_EP2_STATE** `[1:0]` (RO)
///
///   State of OUT Endpoint 2.
#[repr(transparent)]
pub struct OutEp2St {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutEp2St {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutEp2St {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_OUT_EP2_RD_ADDR**
    ///
    /// Read data address of OUT endpoint 2.
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_OFFSET: u32 = 9;
    pub const USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_MASK: u32 = 0x0000fe00;

    /// **USB_SERIAL_JTAG_OUT_EP2_WR_ADDR**
    ///
    /// Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2.
    ///
    /// Bits `[8:2]`, mask `0x000001fc`
    pub const USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_MASK: u32 = 0x000001fc;

    /// **USB_SERIAL_JTAG_OUT_EP2_STATE**
    ///
    /// State of OUT Endpoint 2.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const USB_SERIAL_JTAG_OUT_EP2_STATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_OUT_EP2_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_OUT_EP2_RD_ADDR** `[15:9]` (RO)
    ///
    /// Read data address of OUT endpoint 2.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_rd_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP2_WR_ADDR** `[8:2]` (RO)
    ///
    /// Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_wr_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_OFFSET
    }

    /// **USB_SERIAL_JTAG_OUT_EP2_STATE** `[1:0]` (RO)
    ///
    /// State of OUT Endpoint 2.
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_state(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_OUT_EP2_STATE_MASK)
            >> Self::USB_SERIAL_JTAG_OUT_EP2_STATE_OFFSET
    }
}

/// OUT_EP2_ST value type for batch read/modify/write operations
///
/// Use `outep2st.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outep2st.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutEp2StVal {
    bits: u32,
}

impl OutEp2StVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_RD_ADDR** `[15:9]`
    ///
    /// Read data address of OUT endpoint 2.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_rd_addr(&self) -> u32 {
        (self.bits & OutEp2St::USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_MASK)
            >> OutEp2St::USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_WR_ADDR** `[8:2]`
    ///
    /// Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_wr_addr(&self) -> u32 {
        (self.bits & OutEp2St::USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_MASK)
            >> OutEp2St::USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_OUT_EP2_STATE** `[1:0]`
    ///
    /// State of OUT Endpoint 2.
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_state(&self) -> u32 {
        (self.bits & OutEp2St::USB_SERIAL_JTAG_OUT_EP2_STATE_MASK)
            >> OutEp2St::USB_SERIAL_JTAG_OUT_EP2_STATE_OFFSET
    }
}

impl Default for OutEp2StVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutEp2St {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutEp2StVal {
        OutEp2StVal::from_bits(self.value.read_bits())
    }
}

/// MISC_CONF Register
///
/// Clock enable control
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_CLK_EN** `[0]` (RW)
///
///   1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
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

    /// **USB_SERIAL_JTAG_CLK_EN**
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_CLK_EN_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_CLK_EN_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_CLK_EN** `[0]` (RW)
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub fn usb_serial_jtag_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_CLK_EN_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_CLK_EN_MASK);
    }

    /// Clear USB_SERIAL_JTAG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_CLK_EN_MASK);
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

    /// Read **USB_SERIAL_JTAG_CLK_EN** `[0:0]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn usb_serial_jtag_clk_en(&self) -> bool {
        (self.bits & MiscConf::USB_SERIAL_JTAG_CLK_EN_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_CLK_EN** `[0:0]`
    ///
    /// 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MiscConf::USB_SERIAL_JTAG_CLK_EN_MASK
            } else {
                self.bits & !MiscConf::USB_SERIAL_JTAG_CLK_EN_MASK
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

/// MEM_CONF Register
///
/// Memory power control
///
/// Address offset: 0x0048
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_USB_MEM_CLK_EN** `[1]` (RW)
///
///   1: Force clock on for usb memory.
///
/// - **USB_SERIAL_JTAG_USB_MEM_PD** `[0]` (RW)
///
///   1: power down usb memory.
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

    /// **USB_SERIAL_JTAG_USB_MEM_CLK_EN**
    ///
    /// 1: Force clock on for usb memory.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_USB_MEM_CLK_EN_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_USB_MEM_CLK_EN: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_USB_MEM_PD**
    ///
    /// 1: power down usb memory.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_USB_MEM_PD_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_USB_MEM_PD_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_USB_MEM_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_USB_MEM_CLK_EN** `[1]` (RW)
    ///
    /// 1: Force clock on for usb memory.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_mem_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_mem_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_MEM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_mem_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK);
    }

    /// **USB_SERIAL_JTAG_USB_MEM_PD** `[0]` (RW)
    ///
    /// 1: power down usb memory.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_MEM_PD_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_mem_pd(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_MEM_PD_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_mem_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_MEM_PD_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000002 }
    }

    /// Read **USB_SERIAL_JTAG_USB_MEM_CLK_EN** `[1:1]`
    ///
    /// 1: Force clock on for usb memory.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_mem_clk_en(&self) -> bool {
        (self.bits & MemConf::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_USB_MEM_PD** `[0:0]`
    ///
    /// 1: power down usb memory.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_mem_pd(&self) -> bool {
        (self.bits & MemConf::USB_SERIAL_JTAG_USB_MEM_PD_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_USB_MEM_CLK_EN** `[1:1]`
    ///
    /// 1: Force clock on for usb memory.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_mem_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemConf::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK
            } else {
                self.bits & !MemConf::USB_SERIAL_JTAG_USB_MEM_CLK_EN_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_USB_MEM_PD** `[0:0]`
    ///
    /// 1: power down usb memory.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemConf::USB_SERIAL_JTAG_USB_MEM_PD_MASK
            } else {
                self.bits & !MemConf::USB_SERIAL_JTAG_USB_MEM_PD_MASK
            },
        }
    }
}

impl Default for MemConfVal {
    fn default() -> Self {
        Self::reset_val()
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

/// CHIP_RST Register
///
/// CDC-ACM chip reset control.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS** `[2]` (RW)
///
///   Set this bit to disable chip reset from usb serial channel to reset chip.
///
/// - **USB_SERIAL_JTAG_DTR** `[1]` (RO)
///
///   1: Chip reset is detected from usb jtag channel. Software write 1 to clear it.
///
/// - **USB_SERIAL_JTAG_RTS** `[0]` (RO)
///
///   1: Chip reset is detected from usb serial channel. Software write 1 to clear it.
#[repr(transparent)]
pub struct ChipRst {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ChipRst {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ChipRst {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS**
    ///
    /// Set this bit to disable chip reset from usb serial channel to reset chip.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_DTR**
    ///
    /// 1: Chip reset is detected from usb jtag channel. Software write 1 to clear it.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_DTR_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_DTR_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_DTR: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_RTS**
    ///
    /// 1: Chip reset is detected from usb serial channel. Software write 1 to clear it.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_RTS_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RTS_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_RTS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS** `[2]` (RW)
    ///
    /// Set this bit to disable chip reset from usb serial channel to reset chip.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_uart_chip_rst_dis(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_usb_uart_chip_rst_dis(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK);
    }

    /// Clear USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_usb_uart_chip_rst_dis(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK);
    }

    /// **USB_SERIAL_JTAG_DTR** `[1]` (RO)
    ///
    /// 1: Chip reset is detected from usb jtag channel. Software write 1 to clear it.
    #[inline(always)]
    pub fn usb_serial_jtag_dtr(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DTR_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_RTS** `[0]` (RO)
    ///
    /// 1: Chip reset is detected from usb serial channel. Software write 1 to clear it.
    #[inline(always)]
    pub fn usb_serial_jtag_rts(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RTS_MASK) != 0
    }
}

/// CHIP_RST value type for batch read/modify/write operations
///
/// Use `chiprst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chiprst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ChipRstVal {
    bits: u32,
}

impl ChipRstVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS** `[2:2]`
    ///
    /// Set this bit to disable chip reset from usb serial channel to reset chip.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_uart_chip_rst_dis(&self) -> bool {
        (self.bits & ChipRst::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_DTR** `[1:1]`
    ///
    /// 1: Chip reset is detected from usb jtag channel. Software write 1 to clear it.
    #[inline(always)]
    pub const fn usb_serial_jtag_dtr(&self) -> bool {
        (self.bits & ChipRst::USB_SERIAL_JTAG_DTR_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RTS** `[0:0]`
    ///
    /// 1: Chip reset is detected from usb serial channel. Software write 1 to clear it.
    #[inline(always)]
    pub const fn usb_serial_jtag_rts(&self) -> bool {
        (self.bits & ChipRst::USB_SERIAL_JTAG_RTS_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS** `[2:2]`
    ///
    /// Set this bit to disable chip reset from usb serial channel to reset chip.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_usb_uart_chip_rst_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ChipRst::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK
            } else {
                self.bits & !ChipRst::USB_SERIAL_JTAG_USB_UART_CHIP_RST_DIS_MASK
            },
        }
    }
}

impl Default for ChipRstVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ChipRst {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ChipRstVal {
        ChipRstVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ChipRstVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ChipRstVal) -> ChipRstVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SET_LINE_CODE_W0 Register
///
/// W0 of SET_LINE_CODING command.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_DW_DTE_RATE** `[31:0]` (RO)
///
///   The value of dwDTERate set by host through SET_LINE_CODING command.
#[repr(transparent)]
pub struct SetLineCodeW0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetLineCodeW0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetLineCodeW0 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_DW_DTE_RATE**
    ///
    /// The value of dwDTERate set by host through SET_LINE_CODING command.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_DW_DTE_RATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_DW_DTE_RATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_DW_DTE_RATE** `[31:0]` (RO)
    ///
    /// The value of dwDTERate set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_dw_dte_rate(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DW_DTE_RATE_MASK)
            >> Self::USB_SERIAL_JTAG_DW_DTE_RATE_OFFSET
    }
}

/// SET_LINE_CODE_W0 value type for batch read/modify/write operations
///
/// Use `setlinecodew0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setlinecodew0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetLineCodeW0Val {
    bits: u32,
}

impl SetLineCodeW0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_DW_DTE_RATE** `[31:0]`
    ///
    /// The value of dwDTERate set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_dw_dte_rate(&self) -> u32 {
        (self.bits & SetLineCodeW0::USB_SERIAL_JTAG_DW_DTE_RATE_MASK)
            >> SetLineCodeW0::USB_SERIAL_JTAG_DW_DTE_RATE_OFFSET
    }
}

impl Default for SetLineCodeW0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetLineCodeW0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetLineCodeW0Val {
        SetLineCodeW0Val::from_bits(self.value.read_bits())
    }
}

/// SET_LINE_CODE_W1 Register
///
/// W1 of SET_LINE_CODING command.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_BDATA_BITS** `[23:16]` (RO)
///
///   The value of bDataBits set by host through SET_LINE_CODING command.
///
/// - **USB_SERIAL_JTAG_BPARITY_TYPE** `[15:8]` (RO)
///
///   The value of bParityTpye set by host through SET_LINE_CODING command.
///
/// - **USB_SERIAL_JTAG_BCHAR_FORMAT** `[7:0]` (RO)
///
///   The value of bCharFormat set by host through SET_LINE_CODING command.
#[repr(transparent)]
pub struct SetLineCodeW1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetLineCodeW1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetLineCodeW1 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_BDATA_BITS**
    ///
    /// The value of bDataBits set by host through SET_LINE_CODING command.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const USB_SERIAL_JTAG_BDATA_BITS_OFFSET: u32 = 16;
    pub const USB_SERIAL_JTAG_BDATA_BITS_MASK: u32 = 0x00ff0000;

    /// **USB_SERIAL_JTAG_BPARITY_TYPE**
    ///
    /// The value of bParityTpye set by host through SET_LINE_CODING command.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const USB_SERIAL_JTAG_BPARITY_TYPE_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_BPARITY_TYPE_MASK: u32 = 0x0000ff00;

    /// **USB_SERIAL_JTAG_BCHAR_FORMAT**
    ///
    /// The value of bCharFormat set by host through SET_LINE_CODING command.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const USB_SERIAL_JTAG_BCHAR_FORMAT_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_BCHAR_FORMAT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_BDATA_BITS** `[23:16]` (RO)
    ///
    /// The value of bDataBits set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_bdata_bits(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_BDATA_BITS_MASK)
            >> Self::USB_SERIAL_JTAG_BDATA_BITS_OFFSET
    }

    /// **USB_SERIAL_JTAG_BPARITY_TYPE** `[15:8]` (RO)
    ///
    /// The value of bParityTpye set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_bparity_type(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_BPARITY_TYPE_MASK)
            >> Self::USB_SERIAL_JTAG_BPARITY_TYPE_OFFSET
    }

    /// **USB_SERIAL_JTAG_BCHAR_FORMAT** `[7:0]` (RO)
    ///
    /// The value of bCharFormat set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_bchar_format(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_BCHAR_FORMAT_MASK)
            >> Self::USB_SERIAL_JTAG_BCHAR_FORMAT_OFFSET
    }
}

/// SET_LINE_CODE_W1 value type for batch read/modify/write operations
///
/// Use `setlinecodew1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setlinecodew1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetLineCodeW1Val {
    bits: u32,
}

impl SetLineCodeW1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_BDATA_BITS** `[23:16]`
    ///
    /// The value of bDataBits set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_bdata_bits(&self) -> u32 {
        (self.bits & SetLineCodeW1::USB_SERIAL_JTAG_BDATA_BITS_MASK)
            >> SetLineCodeW1::USB_SERIAL_JTAG_BDATA_BITS_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_BPARITY_TYPE** `[15:8]`
    ///
    /// The value of bParityTpye set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_bparity_type(&self) -> u32 {
        (self.bits & SetLineCodeW1::USB_SERIAL_JTAG_BPARITY_TYPE_MASK)
            >> SetLineCodeW1::USB_SERIAL_JTAG_BPARITY_TYPE_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_BCHAR_FORMAT** `[7:0]`
    ///
    /// The value of bCharFormat set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_bchar_format(&self) -> u32 {
        (self.bits & SetLineCodeW1::USB_SERIAL_JTAG_BCHAR_FORMAT_MASK)
            >> SetLineCodeW1::USB_SERIAL_JTAG_BCHAR_FORMAT_OFFSET
    }
}

impl Default for SetLineCodeW1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetLineCodeW1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetLineCodeW1Val {
        SetLineCodeW1Val::from_bits(self.value.read_bits())
    }
}

/// GET_LINE_CODE_W0 Register
///
/// W0 of GET_LINE_CODING command.
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_GET_DW_DTE_RATE** `[31:0]` (RW)
///
///   The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
#[repr(transparent)]
pub struct GetLineCodeW0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GetLineCodeW0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GetLineCodeW0 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_GET_DW_DTE_RATE**
    ///
    /// The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_GET_DW_DTE_RATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_GET_DW_DTE_RATE** `[31:0]` (RW)
    ///
    /// The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_get_dw_dte_rate(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK)
            >> Self::USB_SERIAL_JTAG_GET_DW_DTE_RATE_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_GET_DW_DTE_RATE** `[31:0]` (RW)
    ///
    /// The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn write_usb_serial_jtag_get_dw_dte_rate(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK)
                | ((value << Self::USB_SERIAL_JTAG_GET_DW_DTE_RATE_OFFSET)
                    & Self::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK)
        });
    }
}

/// GET_LINE_CODE_W0 value type for batch read/modify/write operations
///
/// Use `getlinecodew0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `getlinecodew0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GetLineCodeW0Val {
    bits: u32,
}

impl GetLineCodeW0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_GET_DW_DTE_RATE** `[31:0]`
    ///
    /// The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_dw_dte_rate(&self) -> u32 {
        (self.bits & GetLineCodeW0::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK)
            >> GetLineCodeW0::USB_SERIAL_JTAG_GET_DW_DTE_RATE_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_GET_DW_DTE_RATE** `[31:0]`
    ///
    /// The value of dwDTERate set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_dw_dte_rate(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GetLineCodeW0::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK)
                | ((value << GetLineCodeW0::USB_SERIAL_JTAG_GET_DW_DTE_RATE_OFFSET)
                    & GetLineCodeW0::USB_SERIAL_JTAG_GET_DW_DTE_RATE_MASK),
        }
    }
}

impl Default for GetLineCodeW0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GetLineCodeW0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GetLineCodeW0Val {
        GetLineCodeW0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GetLineCodeW0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GetLineCodeW0Val) -> GetLineCodeW0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GET_LINE_CODE_W1 Register
///
/// W1 of GET_LINE_CODING command.
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_GET_BCHAR_FORMAT** `[23:16]` (RW)
///
///   The value of bDataBits set by software which is requested by GET_LINE_CODING command.
///
/// - **USB_SERIAL_JTAG_GET_BPARITY_TYPE** `[15:8]` (RW)
///
///   The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
///
/// - **USB_SERIAL_JTAG_GET_BDATA_BITS** `[7:0]` (RW)
///
///   The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
#[repr(transparent)]
pub struct GetLineCodeW1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GetLineCodeW1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GetLineCodeW1 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_GET_BCHAR_FORMAT**
    ///
    /// The value of bDataBits set by software which is requested by GET_LINE_CODING command.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const USB_SERIAL_JTAG_GET_BCHAR_FORMAT_OFFSET: u32 = 16;
    pub const USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK: u32 = 0x00ff0000;

    /// **USB_SERIAL_JTAG_GET_BPARITY_TYPE**
    ///
    /// The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const USB_SERIAL_JTAG_GET_BPARITY_TYPE_OFFSET: u32 = 8;
    pub const USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK: u32 = 0x0000ff00;

    /// **USB_SERIAL_JTAG_GET_BDATA_BITS**
    ///
    /// The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const USB_SERIAL_JTAG_GET_BDATA_BITS_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_GET_BDATA_BITS_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_GET_BCHAR_FORMAT** `[23:16]` (RW)
    ///
    /// The value of bDataBits set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_get_bchar_format(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK)
            >> Self::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_GET_BCHAR_FORMAT** `[23:16]` (RW)
    ///
    /// The value of bDataBits set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn write_usb_serial_jtag_get_bchar_format(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK)
                | ((value << Self::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_OFFSET)
                    & Self::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK)
        });
    }

    /// **USB_SERIAL_JTAG_GET_BPARITY_TYPE** `[15:8]` (RW)
    ///
    /// The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_get_bparity_type(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK)
            >> Self::USB_SERIAL_JTAG_GET_BPARITY_TYPE_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_GET_BPARITY_TYPE** `[15:8]` (RW)
    ///
    /// The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn write_usb_serial_jtag_get_bparity_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK)
                | ((value << Self::USB_SERIAL_JTAG_GET_BPARITY_TYPE_OFFSET)
                    & Self::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK)
        });
    }

    /// **USB_SERIAL_JTAG_GET_BDATA_BITS** `[7:0]` (RW)
    ///
    /// The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn usb_serial_jtag_get_bdata_bits(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK)
            >> Self::USB_SERIAL_JTAG_GET_BDATA_BITS_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_GET_BDATA_BITS** `[7:0]` (RW)
    ///
    /// The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub fn write_usb_serial_jtag_get_bdata_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK)
                | ((value << Self::USB_SERIAL_JTAG_GET_BDATA_BITS_OFFSET)
                    & Self::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK)
        });
    }
}

/// GET_LINE_CODE_W1 value type for batch read/modify/write operations
///
/// Use `getlinecodew1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `getlinecodew1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GetLineCodeW1Val {
    bits: u32,
}

impl GetLineCodeW1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_GET_BCHAR_FORMAT** `[23:16]`
    ///
    /// The value of bDataBits set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_bchar_format(&self) -> u32 {
        (self.bits & GetLineCodeW1::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK)
            >> GetLineCodeW1::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_GET_BPARITY_TYPE** `[15:8]`
    ///
    /// The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_bparity_type(&self) -> u32 {
        (self.bits & GetLineCodeW1::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK)
            >> GetLineCodeW1::USB_SERIAL_JTAG_GET_BPARITY_TYPE_OFFSET
    }

    /// Read **USB_SERIAL_JTAG_GET_BDATA_BITS** `[7:0]`
    ///
    /// The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn usb_serial_jtag_get_bdata_bits(&self) -> u32 {
        (self.bits & GetLineCodeW1::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK)
            >> GetLineCodeW1::USB_SERIAL_JTAG_GET_BDATA_BITS_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_GET_BCHAR_FORMAT** `[23:16]`
    ///
    /// The value of bDataBits set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_bchar_format(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GetLineCodeW1::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK)
                | ((value << GetLineCodeW1::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_OFFSET)
                    & GetLineCodeW1::USB_SERIAL_JTAG_GET_BCHAR_FORMAT_MASK),
        }
    }

    /// Set **USB_SERIAL_JTAG_GET_BPARITY_TYPE** `[15:8]`
    ///
    /// The value of bParityTpye set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_bparity_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GetLineCodeW1::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK)
                | ((value << GetLineCodeW1::USB_SERIAL_JTAG_GET_BPARITY_TYPE_OFFSET)
                    & GetLineCodeW1::USB_SERIAL_JTAG_GET_BPARITY_TYPE_MASK),
        }
    }

    /// Set **USB_SERIAL_JTAG_GET_BDATA_BITS** `[7:0]`
    ///
    /// The value of bCharFormat set by software which is requested by GET_LINE_CODING command.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_get_bdata_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GetLineCodeW1::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK)
                | ((value << GetLineCodeW1::USB_SERIAL_JTAG_GET_BDATA_BITS_OFFSET)
                    & GetLineCodeW1::USB_SERIAL_JTAG_GET_BDATA_BITS_MASK),
        }
    }
}

impl Default for GetLineCodeW1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GetLineCodeW1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GetLineCodeW1Val {
        GetLineCodeW1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GetLineCodeW1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GetLineCodeW1Val) -> GetLineCodeW1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG_UPDATE Register
///
/// Configuration registers' value update
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_CONFIG_UPDATE** `[0]` (WO)
///
///   Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain.
#[repr(transparent)]
pub struct ConfigUpdate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ConfigUpdate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ConfigUpdate {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_CONFIG_UPDATE**
    ///
    /// Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_CONFIG_UPDATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_CONFIG_UPDATE_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_CONFIG_UPDATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **USB_SERIAL_JTAG_CONFIG_UPDATE** (WO)
    ///
    /// Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain.
    #[inline(always)]
    pub fn trigger_usb_serial_jtag_config_update(&self) {
        self.value
            .write_bits(Self::USB_SERIAL_JTAG_CONFIG_UPDATE_MASK);
    }
}

/// CONFIG_UPDATE value type for batch read/modify/write operations
///
/// Use `configupdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `configupdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfigUpdateVal {
    bits: u32,
}

impl ConfigUpdateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **USB_SERIAL_JTAG_CONFIG_UPDATE** `[0:0]`
    ///
    /// Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_config_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ConfigUpdate::USB_SERIAL_JTAG_CONFIG_UPDATE_MASK
            } else {
                self.bits & !ConfigUpdate::USB_SERIAL_JTAG_CONFIG_UPDATE_MASK
            },
        }
    }
}

impl Default for ConfigUpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ConfigUpdate {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfigUpdateVal) {
        self.value.write_bits(val.bits())
    }
}

/// SER_AFIFO_CONFIG Register
///
/// Serial AFIFO configure register
///
/// Address offset: 0x0064
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL** `[5]` (RO)
///
///   CDC_ACM OUT IN async FIFO empty signal in write clock domain.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY** `[4]` (RO)
///
///   CDC_ACM OUTOUT async FIFO empty signal in read clock domain.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD** `[3]` (RW)
///
///   Write 1 to reset CDC_ACM OUT async FIFO read clock domain.
///
/// - **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR** `[2]` (RW)
///
///   Write 1 to reset CDC_ACM OUT async FIFO write clock domain.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD** `[1]` (RW)
///
///   Write 1 to reset CDC_ACM IN async FIFO read clock domain.
///
/// - **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR** `[0]` (RW)
///
///   Write 1 to reset CDC_ACM IN async FIFO write clock domain.
#[repr(transparent)]
pub struct SerAfifoConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SerAfifoConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SerAfifoConfig {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL**
    ///
    /// CDC_ACM OUT IN async FIFO empty signal in write clock domain.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_OFFSET: u32 = 5;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_MASK: u32 = 0x00000020;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL: u32 = 1 << 5;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY**
    ///
    /// CDC_ACM OUTOUT async FIFO empty signal in read clock domain.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_OFFSET: u32 = 4;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_MASK: u32 = 0x00000010;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY: u32 = 1 << 4;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD**
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO read clock domain.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_OFFSET: u32 = 3;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK: u32 = 0x00000008;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD: u32 = 1 << 3;

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR**
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO write clock domain.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_OFFSET: u32 = 2;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK: u32 = 0x00000004;
    pub const USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR: u32 = 1 << 2;

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD**
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO read clock domain.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR**
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO write clock domain.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL** `[5]` (RO)
    ///
    /// CDC_ACM OUT IN async FIFO empty signal in write clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_wfull(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY** `[4]` (RO)
    ///
    /// CDC_ACM OUTOUT async FIFO empty signal in read clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_rempty(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_MASK) != 0
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD** `[3]` (RW)
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO read clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_rd(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_out_afifo_reset_rd(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_out_afifo_reset_rd(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR** `[2]` (RW)
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO write clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_wr(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_out_afifo_reset_wr(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_out_afifo_reset_wr(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD** `[1]` (RW)
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO read clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_rd(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_in_afifo_reset_rd(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_in_afifo_reset_rd(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK);
    }

    /// **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR** `[0]` (RW)
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO write clock domain.
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_wr(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_serial_in_afifo_reset_wr(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK);
    }

    /// Clear USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_serial_in_afifo_reset_wr(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK);
    }
}

/// SER_AFIFO_CONFIG value type for batch read/modify/write operations
///
/// Use `serafifoconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `serafifoconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SerAfifoConfigVal {
    bits: u32,
}

impl SerAfifoConfigVal {
    /// Create from raw bits
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
        Self { bits: 0x00000010 }
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL** `[5:5]`
    ///
    /// CDC_ACM OUT IN async FIFO empty signal in write clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_afifo_wfull(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY** `[4:4]`
    ///
    /// CDC_ACM OUTOUT async FIFO empty signal in read clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_afifo_rempty(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD** `[3:3]`
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO read clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_afifo_reset_rd(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR** `[2:2]`
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO write clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_out_afifo_reset_wr(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD** `[1:1]`
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO read clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_afifo_reset_rd(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR** `[0:0]`
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO write clock domain.
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_in_afifo_reset_wr(&self) -> bool {
        (self.bits & SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD** `[3:3]`
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO read clock domain.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_out_afifo_reset_rd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK
            } else {
                self.bits & !SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR** `[2:2]`
    ///
    /// Write 1 to reset CDC_ACM OUT async FIFO write clock domain.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_out_afifo_reset_wr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK
            } else {
                self.bits & !SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD** `[1:1]`
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO read clock domain.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_in_afifo_reset_rd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK
            } else {
                self.bits & !SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_MASK
            },
        }
    }

    /// Set **USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR** `[0:0]`
    ///
    /// Write 1 to reset CDC_ACM IN async FIFO write clock domain.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_serial_in_afifo_reset_wr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK
            } else {
                self.bits & !SerAfifoConfig::USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_MASK
            },
        }
    }
}

impl Default for SerAfifoConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SerAfifoConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SerAfifoConfigVal {
        SerAfifoConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SerAfifoConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SerAfifoConfigVal) -> SerAfifoConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUS_RESET_ST Register
///
/// USB Bus reset status register
///
/// Address offset: 0x0068
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_USB_BUS_RESET_ST** `[0]` (RO)
///
///   USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released.
#[repr(transparent)]
pub struct BusResetSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusResetSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusResetSt {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_ST**
    ///
    /// USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_ST_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_ST_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_USB_BUS_RESET_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_USB_BUS_RESET_ST** `[0]` (RO)
    ///
    /// USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released.
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_st(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_USB_BUS_RESET_ST_MASK) != 0
    }
}

/// BUS_RESET_ST value type for batch read/modify/write operations
///
/// Use `busresetst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `busresetst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusResetStVal {
    bits: u32,
}

impl BusResetStVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_USB_BUS_RESET_ST** `[0:0]`
    ///
    /// USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released.
    #[inline(always)]
    pub const fn usb_serial_jtag_usb_bus_reset_st(&self) -> bool {
        (self.bits & BusResetSt::USB_SERIAL_JTAG_USB_BUS_RESET_ST_MASK) != 0
    }
}

impl Default for BusResetStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BusResetSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusResetStVal {
        BusResetStVal::from_bits(self.value.read_bits())
    }
}

/// ECO_LOW_48 Register
///
/// Reserved.
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RND_ECO_LOW_48** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoLow48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoLow48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoLow48 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RND_ECO_LOW_48**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_RND_ECO_LOW_48_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RND_ECO_LOW_48** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rnd_eco_low_48(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK)
            >> Self::USB_SERIAL_JTAG_RND_ECO_LOW_48_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_RND_ECO_LOW_48** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_usb_serial_jtag_rnd_eco_low_48(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK)
                | ((value << Self::USB_SERIAL_JTAG_RND_ECO_LOW_48_OFFSET)
                    & Self::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK)
        });
    }
}

/// ECO_LOW_48 value type for batch read/modify/write operations
///
/// Use `ecolow48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecolow48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoLow48Val {
    bits: u32,
}

impl EcoLow48Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_RND_ECO_LOW_48** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rnd_eco_low_48(&self) -> u32 {
        (self.bits & EcoLow48::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK)
            >> EcoLow48::USB_SERIAL_JTAG_RND_ECO_LOW_48_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_RND_ECO_LOW_48** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rnd_eco_low_48(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoLow48::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK)
                | ((value << EcoLow48::USB_SERIAL_JTAG_RND_ECO_LOW_48_OFFSET)
                    & EcoLow48::USB_SERIAL_JTAG_RND_ECO_LOW_48_MASK),
        }
    }
}

impl Default for EcoLow48Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoLow48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoLow48Val {
        EcoLow48Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoLow48Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoLow48Val) -> EcoLow48Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_HIGH_48 Register
///
/// Reserved.
///
/// Address offset: 0x0070
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RND_ECO_HIGH_48** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoHigh48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoHigh48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoHigh48 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RND_ECO_HIGH_48**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_RND_ECO_HIGH_48_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RND_ECO_HIGH_48** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rnd_eco_high_48(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK)
            >> Self::USB_SERIAL_JTAG_RND_ECO_HIGH_48_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_RND_ECO_HIGH_48** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_usb_serial_jtag_rnd_eco_high_48(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK)
                | ((value << Self::USB_SERIAL_JTAG_RND_ECO_HIGH_48_OFFSET)
                    & Self::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK)
        });
    }
}

/// ECO_HIGH_48 value type for batch read/modify/write operations
///
/// Use `ecohigh48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecohigh48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoHigh48Val {
    bits: u32,
}

impl EcoHigh48Val {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_RND_ECO_HIGH_48** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rnd_eco_high_48(&self) -> u32 {
        (self.bits & EcoHigh48::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK)
            >> EcoHigh48::USB_SERIAL_JTAG_RND_ECO_HIGH_48_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_RND_ECO_HIGH_48** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rnd_eco_high_48(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoHigh48::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK)
                | ((value << EcoHigh48::USB_SERIAL_JTAG_RND_ECO_HIGH_48_OFFSET)
                    & EcoHigh48::USB_SERIAL_JTAG_RND_ECO_HIGH_48_MASK),
        }
    }
}

impl Default for EcoHigh48Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EcoHigh48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoHigh48Val {
        EcoHigh48Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoHigh48Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoHigh48Val) -> EcoHigh48Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_CELL_CTRL_48 Register
///
/// Reserved.
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RDN_ENA_48** `[1]` (RW)
///
///   Reserved.
///
/// - **USB_SERIAL_JTAG_RDN_RESULT_48** `[0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoCellCtrl48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoCellCtrl48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoCellCtrl48 {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RDN_ENA_48**
    ///
    /// Reserved.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_RDN_ENA_48_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_RDN_ENA_48_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_RDN_ENA_48: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_RDN_RESULT_48**
    ///
    /// Reserved.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_RDN_RESULT_48_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RDN_RESULT_48_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_RDN_RESULT_48: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RDN_ENA_48** `[1]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_ena_48(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RDN_ENA_48_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_RDN_ENA_48 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_rdn_ena_48(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_RDN_ENA_48_MASK);
    }

    /// Clear USB_SERIAL_JTAG_RDN_ENA_48 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_rdn_ena_48(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_RDN_ENA_48_MASK);
    }

    /// **USB_SERIAL_JTAG_RDN_RESULT_48** `[0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_result_48(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RDN_RESULT_48_MASK) != 0
    }
}

/// ECO_CELL_CTRL_48 value type for batch read/modify/write operations
///
/// Use `ecocellctrl48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecocellctrl48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoCellCtrl48Val {
    bits: u32,
}

impl EcoCellCtrl48Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_RDN_ENA_48** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rdn_ena_48(&self) -> bool {
        (self.bits & EcoCellCtrl48::USB_SERIAL_JTAG_RDN_ENA_48_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RDN_RESULT_48** `[0:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rdn_result_48(&self) -> bool {
        (self.bits & EcoCellCtrl48::USB_SERIAL_JTAG_RDN_RESULT_48_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_RDN_ENA_48** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rdn_ena_48(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EcoCellCtrl48::USB_SERIAL_JTAG_RDN_ENA_48_MASK
            } else {
                self.bits & !EcoCellCtrl48::USB_SERIAL_JTAG_RDN_ENA_48_MASK
            },
        }
    }
}

impl Default for EcoCellCtrl48Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoCellCtrl48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoCellCtrl48Val {
        EcoCellCtrl48Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoCellCtrl48Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoCellCtrl48Val) -> EcoCellCtrl48Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_LOW_APB Register
///
/// Reserved.
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RND_ECO_LOW_APB** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoLowApb {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoLowApb {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoLowApb {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RND_ECO_LOW_APB**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_RND_ECO_LOW_APB_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RND_ECO_LOW_APB** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rnd_eco_low_apb(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK)
            >> Self::USB_SERIAL_JTAG_RND_ECO_LOW_APB_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_RND_ECO_LOW_APB** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_usb_serial_jtag_rnd_eco_low_apb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK)
                | ((value << Self::USB_SERIAL_JTAG_RND_ECO_LOW_APB_OFFSET)
                    & Self::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK)
        });
    }
}

/// ECO_LOW_APB value type for batch read/modify/write operations
///
/// Use `ecolowapb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecolowapb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoLowApbVal {
    bits: u32,
}

impl EcoLowApbVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_RND_ECO_LOW_APB** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rnd_eco_low_apb(&self) -> u32 {
        (self.bits & EcoLowApb::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK)
            >> EcoLowApb::USB_SERIAL_JTAG_RND_ECO_LOW_APB_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_RND_ECO_LOW_APB** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rnd_eco_low_apb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoLowApb::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK)
                | ((value << EcoLowApb::USB_SERIAL_JTAG_RND_ECO_LOW_APB_OFFSET)
                    & EcoLowApb::USB_SERIAL_JTAG_RND_ECO_LOW_APB_MASK),
        }
    }
}

impl Default for EcoLowApbVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoLowApb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoLowApbVal {
        EcoLowApbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoLowApbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoLowApbVal) -> EcoLowApbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_HIGH_APB Register
///
/// Reserved.
///
/// Address offset: 0x007c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RND_ECO_HIGH_APB** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoHighApb {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoHighApb {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoHighApb {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RND_ECO_HIGH_APB**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_RND_ECO_HIGH_APB_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RND_ECO_HIGH_APB** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rnd_eco_high_apb(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK)
            >> Self::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_RND_ECO_HIGH_APB** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_usb_serial_jtag_rnd_eco_high_apb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK)
                | ((value << Self::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_OFFSET)
                    & Self::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK)
        });
    }
}

/// ECO_HIGH_APB value type for batch read/modify/write operations
///
/// Use `ecohighapb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecohighapb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoHighApbVal {
    bits: u32,
}

impl EcoHighApbVal {
    /// Create from raw bits
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

    /// Read **USB_SERIAL_JTAG_RND_ECO_HIGH_APB** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rnd_eco_high_apb(&self) -> u32 {
        (self.bits & EcoHighApb::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK)
            >> EcoHighApb::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_RND_ECO_HIGH_APB** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rnd_eco_high_apb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoHighApb::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK)
                | ((value << EcoHighApb::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_OFFSET)
                    & EcoHighApb::USB_SERIAL_JTAG_RND_ECO_HIGH_APB_MASK),
        }
    }
}

impl Default for EcoHighApbVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EcoHighApb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoHighApbVal {
        EcoHighApbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoHighApbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoHighApbVal) -> EcoHighApbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_CELL_CTRL_APB Register
///
/// Reserved.
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_RDN_ENA_APB** `[1]` (RW)
///
///   Reserved.
///
/// - **USB_SERIAL_JTAG_RDN_RESULT_APB** `[0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoCellCtrlApb {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoCellCtrlApb {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoCellCtrlApb {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_RDN_ENA_APB**
    ///
    /// Reserved.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const USB_SERIAL_JTAG_RDN_ENA_APB_OFFSET: u32 = 1;
    pub const USB_SERIAL_JTAG_RDN_ENA_APB_MASK: u32 = 0x00000002;
    pub const USB_SERIAL_JTAG_RDN_ENA_APB: u32 = 1 << 1;

    /// **USB_SERIAL_JTAG_RDN_RESULT_APB**
    ///
    /// Reserved.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const USB_SERIAL_JTAG_RDN_RESULT_APB_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_RDN_RESULT_APB_MASK: u32 = 0x00000001;
    pub const USB_SERIAL_JTAG_RDN_RESULT_APB: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_RDN_ENA_APB** `[1]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_ena_apb(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RDN_ENA_APB_MASK) != 0
    }

    /// Set USB_SERIAL_JTAG_RDN_ENA_APB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_serial_jtag_rdn_ena_apb(&self) {
        self.value
            .modify_bits(|r| r | Self::USB_SERIAL_JTAG_RDN_ENA_APB_MASK);
    }

    /// Clear USB_SERIAL_JTAG_RDN_ENA_APB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_serial_jtag_rdn_ena_apb(&self) {
        self.value
            .modify_bits(|r| r & !Self::USB_SERIAL_JTAG_RDN_ENA_APB_MASK);
    }

    /// **USB_SERIAL_JTAG_RDN_RESULT_APB** `[0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_result_apb(&self) -> bool {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_RDN_RESULT_APB_MASK) != 0
    }
}

/// ECO_CELL_CTRL_APB value type for batch read/modify/write operations
///
/// Use `ecocellctrlapb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecocellctrlapb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoCellCtrlApbVal {
    bits: u32,
}

impl EcoCellCtrlApbVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_SERIAL_JTAG_RDN_ENA_APB** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rdn_ena_apb(&self) -> bool {
        (self.bits & EcoCellCtrlApb::USB_SERIAL_JTAG_RDN_ENA_APB_MASK) != 0
    }

    /// Read **USB_SERIAL_JTAG_RDN_RESULT_APB** `[0:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn usb_serial_jtag_rdn_result_apb(&self) -> bool {
        (self.bits & EcoCellCtrlApb::USB_SERIAL_JTAG_RDN_RESULT_APB_MASK) != 0
    }

    /// Set **USB_SERIAL_JTAG_RDN_ENA_APB** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_rdn_ena_apb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EcoCellCtrlApb::USB_SERIAL_JTAG_RDN_ENA_APB_MASK
            } else {
                self.bits & !EcoCellCtrlApb::USB_SERIAL_JTAG_RDN_ENA_APB_MASK
            },
        }
    }
}

impl Default for EcoCellCtrlApbVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoCellCtrlApb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoCellCtrlApbVal {
        EcoCellCtrlApbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoCellCtrlApbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoCellCtrlApbVal) -> EcoCellCtrlApbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM_CTRL Register
///
/// PPA SRAM Control Register
///
/// Address offset: 0x0084
/// Reset value: 0x00001320
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_MEM_AUX_CTRL** `[13:0]` (RW)
///
///   Control signals
#[repr(transparent)]
pub struct SramCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SramCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SramCtrl {
    // ======== Field Constants ========

    /// **USB_SERIAL_JTAG_MEM_AUX_CTRL**
    ///
    /// Control signals
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const USB_SERIAL_JTAG_MEM_AUX_CTRL_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// Control signals
    #[inline(always)]
    pub fn usb_serial_jtag_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK)
            >> Self::USB_SERIAL_JTAG_MEM_AUX_CTRL_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// Control signals
    #[inline(always)]
    pub fn write_usb_serial_jtag_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK)
                | ((value << Self::USB_SERIAL_JTAG_MEM_AUX_CTRL_OFFSET)
                    & Self::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK)
        });
    }
}

/// SRAM_CTRL value type for batch read/modify/write operations
///
/// Use `sramctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sramctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SramCtrlVal {
    bits: u32,
}

impl SramCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00001320 }
    }

    /// Read **USB_SERIAL_JTAG_MEM_AUX_CTRL** `[13:0]`
    ///
    /// Control signals
    #[inline(always)]
    pub const fn usb_serial_jtag_mem_aux_ctrl(&self) -> u32 {
        (self.bits & SramCtrl::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK)
            >> SramCtrl::USB_SERIAL_JTAG_MEM_AUX_CTRL_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_MEM_AUX_CTRL** `[13:0]`
    ///
    /// Control signals
    #[inline(always)]
    pub const fn with_usb_serial_jtag_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SramCtrl::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK)
                | ((value << SramCtrl::USB_SERIAL_JTAG_MEM_AUX_CTRL_OFFSET)
                    & SramCtrl::USB_SERIAL_JTAG_MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for SramCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SramCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SramCtrlVal {
        SramCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SramCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SramCtrlVal) -> SramCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Date register
///
/// Address offset: 0x0088
/// Reset value: 0x02112010
///
/// ## Fields
///
/// - **USB_SERIAL_JTAG_DATE** `[31:0]` (RW)
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

    /// **USB_SERIAL_JTAG_DATE**
    ///
    /// register version.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_SERIAL_JTAG_DATE_OFFSET: u32 = 0;
    pub const USB_SERIAL_JTAG_DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_SERIAL_JTAG_DATE** `[31:0]` (RW)
    ///
    /// register version.
    #[inline(always)]
    pub fn usb_serial_jtag_date(&self) -> u32 {
        (self.value.read_bits() & Self::USB_SERIAL_JTAG_DATE_MASK)
            >> Self::USB_SERIAL_JTAG_DATE_OFFSET
    }

    /// Write **USB_SERIAL_JTAG_DATE** `[31:0]` (RW)
    ///
    /// register version.
    #[inline(always)]
    pub fn write_usb_serial_jtag_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_SERIAL_JTAG_DATE_MASK)
                | ((value << Self::USB_SERIAL_JTAG_DATE_OFFSET) & Self::USB_SERIAL_JTAG_DATE_MASK)
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
        Self { bits: 0x02112010 }
    }

    /// Read **USB_SERIAL_JTAG_DATE** `[31:0]`
    ///
    /// register version.
    #[inline(always)]
    pub const fn usb_serial_jtag_date(&self) -> u32 {
        (self.bits & Date::USB_SERIAL_JTAG_DATE_MASK) >> Date::USB_SERIAL_JTAG_DATE_OFFSET
    }

    /// Set **USB_SERIAL_JTAG_DATE** `[31:0]`
    ///
    /// register version.
    #[inline(always)]
    pub const fn with_usb_serial_jtag_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::USB_SERIAL_JTAG_DATE_MASK)
                | ((value << Date::USB_SERIAL_JTAG_DATE_OFFSET) & Date::USB_SERIAL_JTAG_DATE_MASK),
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
    /// FIFO access for the CDC-ACM data IN and OUT endpoints.
    pub ep1: Ep1,
    /// 0x0004
    /// Configuration and control registers for the CDC-ACM FIFOs.
    pub ep1_conf: Ep1Conf,
    /// 0x0008
    /// Interrupt raw status register.
    pub int_raw: IntRaw,
    /// 0x000c
    /// Interrupt status register.
    pub int_st: IntSt,
    /// 0x0010
    /// Interrupt enable status register.
    pub int_ena: IntEna,
    /// 0x0014
    /// Interrupt clear status register.
    pub int_clr: IntClr,
    /// 0x0018
    /// PHY hardware configuration.
    pub conf0: Conf0,
    /// 0x001c
    /// Registers used for debugging the PHY.
    pub test: Test,
    /// 0x0020
    /// JTAG FIFO status and control registers.
    pub jfifo_st: JfifoSt,
    /// 0x0024
    /// Last received SOF frame index register.
    pub fram_num: FramNum,
    /// 0x0028
    /// Control IN endpoint status information.
    pub in_ep0_st: InEp0St,
    /// 0x002c
    /// CDC-ACM IN endpoint status information.
    pub in_ep1_st: InEp1St,
    /// 0x0030
    /// CDC-ACM interrupt IN endpoint status information.
    pub in_ep2_st: InEp2St,
    /// 0x0034
    /// JTAG IN endpoint status information.
    pub in_ep3_st: InEp3St,
    /// 0x0038
    /// Control OUT endpoint status information.
    pub out_ep0_st: OutEp0St,
    /// 0x003c
    /// CDC-ACM OUT endpoint status information.
    pub out_ep1_st: OutEp1St,
    /// 0x0040
    /// JTAG OUT endpoint status information.
    pub out_ep2_st: OutEp2St,
    /// 0x0044
    /// Clock enable control
    pub misc_conf: MiscConf,
    /// 0x0048
    /// Memory power control
    pub mem_conf: MemConf,
    /// 0x004c
    /// CDC-ACM chip reset control.
    pub chip_rst: ChipRst,
    /// 0x0050
    /// W0 of SET_LINE_CODING command.
    pub set_line_code_w0: SetLineCodeW0,
    /// 0x0054
    /// W1 of SET_LINE_CODING command.
    pub set_line_code_w1: SetLineCodeW1,
    /// 0x0058
    /// W0 of GET_LINE_CODING command.
    pub get_line_code_w0: GetLineCodeW0,
    /// 0x005c
    /// W1 of GET_LINE_CODING command.
    pub get_line_code_w1: GetLineCodeW1,
    /// 0x0060
    /// Configuration registers' value update
    pub config_update: ConfigUpdate,
    /// 0x0064
    /// Serial AFIFO configure register
    pub ser_afifo_config: SerAfifoConfig,
    /// 0x0068
    /// USB Bus reset status register
    pub bus_reset_st: BusResetSt,
    /// 0x006c
    /// Reserved.
    pub eco_low48: EcoLow48,
    /// 0x0070
    /// Reserved.
    pub eco_high48: EcoHigh48,
    /// 0x0074
    /// Reserved.
    pub eco_cell_ctrl48: EcoCellCtrl48,
    /// 0x0078
    /// Reserved.
    pub eco_low_apb: EcoLowApb,
    /// 0x007c
    /// Reserved.
    pub eco_high_apb: EcoHighApb,
    /// 0x0080
    /// Reserved.
    pub eco_cell_ctrl_apb: EcoCellCtrlApb,
    /// 0x0084
    /// PPA SRAM Control Register
    pub sram_ctrl: SramCtrl,
    /// 0x0088
    /// Date register
    pub date: Date,
}
