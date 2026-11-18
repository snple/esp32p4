//! I3C_MST_MEM
//! I3C_MST_MEM Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x500da000;

/// COMMAND_BUF_PORT Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **REG_COMMAND** `[31:0]` (RW)
///
///   Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
#[repr(transparent)]
pub struct CommandBufPort {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CommandBufPort {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CommandBufPort {
    // ======== Field Constants ========

    /// **REG_COMMAND**
    ///
    /// Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_COMMAND_OFFSET: u32 = 0;
    pub const REG_COMMAND_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_COMMAND** `[31:0]` (RW)
    ///
    /// Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
    #[inline(always)]
    pub fn reg_command(&self) -> u32 {
        (self.value.read_bits() & Self::REG_COMMAND_MASK) >> Self::REG_COMMAND_OFFSET
    }

    /// Write **REG_COMMAND** `[31:0]` (RW)
    ///
    /// Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
    #[inline(always)]
    pub fn write_reg_command(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_COMMAND_MASK)
                | ((value << Self::REG_COMMAND_OFFSET) & Self::REG_COMMAND_MASK)
        });
    }
}

/// COMMAND_BUF_PORT value type for batch read/modify/write operations
///
/// Use `commandbufport.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `commandbufport.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CommandBufPortVal {
    bits: u32,
}

impl CommandBufPortVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_COMMAND** `[31:0]`
    ///
    /// Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
    #[inline(always)]
    pub const fn reg_command(&self) -> u32 {
        (self.bits & CommandBufPort::REG_COMMAND_MASK) >> CommandBufPort::REG_COMMAND_OFFSET
    }

    /// Set **REG_COMMAND** `[31:0]`
    ///
    /// Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus.
    #[inline(always)]
    pub const fn with_reg_command(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CommandBufPort::REG_COMMAND_MASK)
                | ((value << CommandBufPort::REG_COMMAND_OFFSET)
                    & CommandBufPort::REG_COMMAND_MASK),
        }
    }
}

impl Default for CommandBufPortVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CommandBufPort {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CommandBufPortVal {
        CommandBufPortVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CommandBufPortVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CommandBufPortVal) -> CommandBufPortVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESPONSE_BUF_PORT Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **RESPONSE** `[31:0]` (RO)
///
///   The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register.
#[repr(transparent)]
pub struct ResponseBufPort {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ResponseBufPort {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ResponseBufPort {
    // ======== Field Constants ========

    /// **RESPONSE**
    ///
    /// The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RESPONSE_OFFSET: u32 = 0;
    pub const RESPONSE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RESPONSE** `[31:0]` (RO)
    ///
    /// The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register.
    #[inline(always)]
    pub fn response(&self) -> u32 {
        (self.value.read_bits() & Self::RESPONSE_MASK) >> Self::RESPONSE_OFFSET
    }
}

/// RESPONSE_BUF_PORT value type for batch read/modify/write operations
///
/// Use `responsebufport.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `responsebufport.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResponseBufPortVal {
    bits: u32,
}

impl ResponseBufPortVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RESPONSE** `[31:0]`
    ///
    /// The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register.
    #[inline(always)]
    pub const fn response(&self) -> u32 {
        (self.bits & ResponseBufPort::RESPONSE_MASK) >> ResponseBufPort::RESPONSE_OFFSET
    }
}

impl Default for ResponseBufPortVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ResponseBufPort {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResponseBufPortVal {
        ResponseBufPortVal::from_bits(self.value.read_bits())
    }
}

/// RX_DATA_PORT Register
///
/// NA
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **RX_DATA_PORT** `[31:0]` (RO)
///
///   Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
#[repr(transparent)]
pub struct RxDataPort {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxDataPort {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxDataPort {
    // ======== Field Constants ========

    /// **RX_DATA_PORT**
    ///
    /// Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RX_DATA_PORT_OFFSET: u32 = 0;
    pub const RX_DATA_PORT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RX_DATA_PORT** `[31:0]` (RO)
    ///
    /// Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub fn rx_data_port(&self) -> u32 {
        (self.value.read_bits() & Self::RX_DATA_PORT_MASK) >> Self::RX_DATA_PORT_OFFSET
    }
}

/// RX_DATA_PORT value type for batch read/modify/write operations
///
/// Use `rxdataport.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxdataport.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxDataPortVal {
    bits: u32,
}

impl RxDataPortVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_DATA_PORT** `[31:0]`
    ///
    /// Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub const fn rx_data_port(&self) -> u32 {
        (self.bits & RxDataPort::RX_DATA_PORT_MASK) >> RxDataPort::RX_DATA_PORT_OFFSET
    }
}

impl Default for RxDataPortVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxDataPort {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxDataPortVal {
        RxDataPortVal::from_bits(self.value.read_bits())
    }
}

/// TX_DATA_PORT Register
///
/// NA
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **REG_TX_DATA_PORT** `[31:0]` (RW)
///
///   Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
#[repr(transparent)]
pub struct TxDataPort {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxDataPort {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxDataPort {
    // ======== Field Constants ========

    /// **REG_TX_DATA_PORT**
    ///
    /// Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_TX_DATA_PORT_OFFSET: u32 = 0;
    pub const REG_TX_DATA_PORT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_TX_DATA_PORT** `[31:0]` (RW)
    ///
    /// Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub fn reg_tx_data_port(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TX_DATA_PORT_MASK) >> Self::REG_TX_DATA_PORT_OFFSET
    }

    /// Write **REG_TX_DATA_PORT** `[31:0]` (RW)
    ///
    /// Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub fn write_reg_tx_data_port(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TX_DATA_PORT_MASK)
                | ((value << Self::REG_TX_DATA_PORT_OFFSET) & Self::REG_TX_DATA_PORT_MASK)
        });
    }
}

/// TX_DATA_PORT value type for batch read/modify/write operations
///
/// Use `txdataport.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txdataport.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxDataPortVal {
    bits: u32,
}

impl TxDataPortVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_TX_DATA_PORT** `[31:0]`
    ///
    /// Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub const fn reg_tx_data_port(&self) -> u32 {
        (self.bits & TxDataPort::REG_TX_DATA_PORT_MASK) >> TxDataPort::REG_TX_DATA_PORT_OFFSET
    }

    /// Set **REG_TX_DATA_PORT** `[31:0]`
    ///
    /// Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor.
    #[inline(always)]
    pub const fn with_reg_tx_data_port(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxDataPort::REG_TX_DATA_PORT_MASK)
                | ((value << TxDataPort::REG_TX_DATA_PORT_OFFSET)
                    & TxDataPort::REG_TX_DATA_PORT_MASK),
        }
    }
}

impl Default for TxDataPortVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxDataPort {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxDataPortVal {
        TxDataPortVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxDataPortVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxDataPortVal) -> TxDataPortVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IBI_STATUS_BUF Register
///
/// In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **IBI_STS** `[28]` (RO)
///
///   IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ).
///
/// - **IBI_ID** `[15:8]` (RO)
///
///   IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request.
///
/// - **DATA_LENGTH** `[7:0]` (RO)
///
///   This field represents the length of data received along with IBI, in bytes.
#[repr(transparent)]
pub struct IbiStatusBuf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IbiStatusBuf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IbiStatusBuf {
    // ======== Field Constants ========

    /// **IBI_STS**
    ///
    /// IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ).
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const IBI_STS_OFFSET: u32 = 28;
    pub const IBI_STS_MASK: u32 = 0x10000000;
    pub const IBI_STS: u32 = 1 << 28;

    /// **IBI_ID**
    ///
    /// IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const IBI_ID_OFFSET: u32 = 8;
    pub const IBI_ID_MASK: u32 = 0x0000ff00;

    /// **DATA_LENGTH**
    ///
    /// This field represents the length of data received along with IBI, in bytes.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_LENGTH_OFFSET: u32 = 0;
    pub const DATA_LENGTH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **IBI_STS** `[28]` (RO)
    ///
    /// IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ).
    #[inline(always)]
    pub fn ibi_sts(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STS_MASK) != 0
    }

    /// **IBI_ID** `[15:8]` (RO)
    ///
    /// IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request.
    #[inline(always)]
    pub fn ibi_id(&self) -> u32 {
        (self.value.read_bits() & Self::IBI_ID_MASK) >> Self::IBI_ID_OFFSET
    }

    /// **DATA_LENGTH** `[7:0]` (RO)
    ///
    /// This field represents the length of data received along with IBI, in bytes.
    #[inline(always)]
    pub fn data_length(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_LENGTH_MASK) >> Self::DATA_LENGTH_OFFSET
    }
}

/// IBI_STATUS_BUF value type for batch read/modify/write operations
///
/// Use `ibistatusbuf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ibistatusbuf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IbiStatusBufVal {
    bits: u32,
}

impl IbiStatusBufVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IBI_STS** `[28:28]`
    ///
    /// IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ).
    #[inline(always)]
    pub const fn ibi_sts(&self) -> bool {
        (self.bits & IbiStatusBuf::IBI_STS_MASK) != 0
    }

    /// Read **IBI_ID** `[15:8]`
    ///
    /// IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request.
    #[inline(always)]
    pub const fn ibi_id(&self) -> u32 {
        (self.bits & IbiStatusBuf::IBI_ID_MASK) >> IbiStatusBuf::IBI_ID_OFFSET
    }

    /// Read **DATA_LENGTH** `[7:0]`
    ///
    /// This field represents the length of data received along with IBI, in bytes.
    #[inline(always)]
    pub const fn data_length(&self) -> u32 {
        (self.bits & IbiStatusBuf::DATA_LENGTH_MASK) >> IbiStatusBuf::DATA_LENGTH_OFFSET
    }
}

impl Default for IbiStatusBufVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IbiStatusBuf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IbiStatusBufVal {
        IbiStatusBufVal::from_bits(self.value.read_bits())
    }
}

/// IBI_DATA_BUF Register
///
/// NA
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **IBI_DATA** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IbiDataBuf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IbiDataBuf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IbiDataBuf {
    // ======== Field Constants ========

    /// **IBI_DATA**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IBI_DATA_OFFSET: u32 = 0;
    pub const IBI_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IBI_DATA** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_data(&self) -> u32 {
        (self.value.read_bits() & Self::IBI_DATA_MASK) >> Self::IBI_DATA_OFFSET
    }
}

/// IBI_DATA_BUF value type for batch read/modify/write operations
///
/// Use `ibidatabuf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ibidatabuf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IbiDataBufVal {
    bits: u32,
}

impl IbiDataBufVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IBI_DATA** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_data(&self) -> u32 {
        (self.bits & IbiDataBuf::IBI_DATA_MASK) >> IbiDataBuf::IBI_DATA_OFFSET
    }
}

impl Default for IbiDataBufVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IbiDataBuf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IbiDataBufVal {
        IbiDataBufVal::from_bits(self.value.read_bits())
    }
}

/// DEV_ADDR_TABLE1_LOC Register
///
/// NA
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **REG_DAT_DEV1_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV1_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV1_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV1_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable1Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable1Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable1Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV1_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV1_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV1_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV1_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV1_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV1_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV1_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV1_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV1_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV1_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV1_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV1_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV1_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV1_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev1_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV1_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV1_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev1_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV1_I2C_MASK);
    }

    /// Clear REG_DAT_DEV1_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev1_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV1_I2C_MASK);
    }

    /// **REG_DAT_DEV1_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev1_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV1_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV1_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV1_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev1_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV1_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV1_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV1_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV1_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev1_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV1_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV1_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV1_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev1_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV1_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV1_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV1_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV1_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev1_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV1_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV1_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV1_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev1_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV1_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV1_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV1_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE1_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable1loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable1loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable1LocVal {
    bits: u32,
}

impl DevAddrTable1LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV1_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev1_i2c(&self) -> bool {
        (self.bits & DevAddrTable1Loc::REG_DAT_DEV1_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV1_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev1_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable1Loc::REG_DAT_DEV1_NACK_RETRY_CNT_MASK)
            >> DevAddrTable1Loc::REG_DAT_DEV1_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV1_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev1_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable1Loc::REG_DAT_DEV1_DYNAMIC_ADDR_MASK)
            >> DevAddrTable1Loc::REG_DAT_DEV1_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV1_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev1_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable1Loc::REG_DAT_DEV1_STATIC_ADDR_MASK)
            >> DevAddrTable1Loc::REG_DAT_DEV1_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV1_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev1_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable1Loc::REG_DAT_DEV1_I2C_MASK
            } else {
                self.bits & !DevAddrTable1Loc::REG_DAT_DEV1_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV1_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev1_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable1Loc::REG_DAT_DEV1_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable1Loc::REG_DAT_DEV1_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable1Loc::REG_DAT_DEV1_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV1_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev1_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable1Loc::REG_DAT_DEV1_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable1Loc::REG_DAT_DEV1_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable1Loc::REG_DAT_DEV1_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV1_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev1_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable1Loc::REG_DAT_DEV1_STATIC_ADDR_MASK)
                | ((value << DevAddrTable1Loc::REG_DAT_DEV1_STATIC_ADDR_OFFSET)
                    & DevAddrTable1Loc::REG_DAT_DEV1_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable1LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable1Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable1LocVal {
        DevAddrTable1LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable1LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable1LocVal) -> DevAddrTable1LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE2_LOC Register
///
/// NA
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **REG_DAT_DEV2_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV2_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV2_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV2_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable2Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable2Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable2Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV2_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV2_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV2_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV2_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV2_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV2_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV2_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV2_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV2_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV2_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV2_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV2_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV2_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV2_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev2_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV2_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV2_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev2_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV2_I2C_MASK);
    }

    /// Clear REG_DAT_DEV2_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev2_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV2_I2C_MASK);
    }

    /// **REG_DAT_DEV2_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev2_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV2_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV2_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV2_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev2_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV2_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV2_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV2_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV2_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev2_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV2_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV2_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV2_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev2_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV2_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV2_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV2_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV2_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev2_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV2_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV2_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV2_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev2_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV2_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV2_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV2_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE2_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable2loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable2loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable2LocVal {
    bits: u32,
}

impl DevAddrTable2LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV2_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev2_i2c(&self) -> bool {
        (self.bits & DevAddrTable2Loc::REG_DAT_DEV2_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV2_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev2_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable2Loc::REG_DAT_DEV2_NACK_RETRY_CNT_MASK)
            >> DevAddrTable2Loc::REG_DAT_DEV2_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV2_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev2_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable2Loc::REG_DAT_DEV2_DYNAMIC_ADDR_MASK)
            >> DevAddrTable2Loc::REG_DAT_DEV2_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV2_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev2_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable2Loc::REG_DAT_DEV2_STATIC_ADDR_MASK)
            >> DevAddrTable2Loc::REG_DAT_DEV2_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV2_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev2_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable2Loc::REG_DAT_DEV2_I2C_MASK
            } else {
                self.bits & !DevAddrTable2Loc::REG_DAT_DEV2_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV2_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev2_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable2Loc::REG_DAT_DEV2_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable2Loc::REG_DAT_DEV2_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable2Loc::REG_DAT_DEV2_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV2_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev2_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable2Loc::REG_DAT_DEV2_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable2Loc::REG_DAT_DEV2_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable2Loc::REG_DAT_DEV2_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV2_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev2_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable2Loc::REG_DAT_DEV2_STATIC_ADDR_MASK)
                | ((value << DevAddrTable2Loc::REG_DAT_DEV2_STATIC_ADDR_OFFSET)
                    & DevAddrTable2Loc::REG_DAT_DEV2_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable2LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable2Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable2LocVal {
        DevAddrTable2LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable2LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable2LocVal) -> DevAddrTable2LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE3_LOC Register
///
/// NA
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **REG_DAT_DEV3_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV3_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV3_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV3_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable3Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable3Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable3Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV3_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV3_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV3_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV3_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV3_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV3_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV3_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV3_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV3_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV3_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV3_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV3_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV3_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV3_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev3_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV3_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV3_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev3_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV3_I2C_MASK);
    }

    /// Clear REG_DAT_DEV3_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev3_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV3_I2C_MASK);
    }

    /// **REG_DAT_DEV3_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev3_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV3_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV3_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV3_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev3_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV3_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV3_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV3_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV3_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev3_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV3_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV3_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV3_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev3_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV3_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV3_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV3_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV3_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev3_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV3_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV3_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV3_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev3_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV3_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV3_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV3_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE3_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable3loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable3loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable3LocVal {
    bits: u32,
}

impl DevAddrTable3LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV3_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev3_i2c(&self) -> bool {
        (self.bits & DevAddrTable3Loc::REG_DAT_DEV3_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV3_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev3_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable3Loc::REG_DAT_DEV3_NACK_RETRY_CNT_MASK)
            >> DevAddrTable3Loc::REG_DAT_DEV3_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV3_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev3_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable3Loc::REG_DAT_DEV3_DYNAMIC_ADDR_MASK)
            >> DevAddrTable3Loc::REG_DAT_DEV3_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV3_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev3_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable3Loc::REG_DAT_DEV3_STATIC_ADDR_MASK)
            >> DevAddrTable3Loc::REG_DAT_DEV3_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV3_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev3_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable3Loc::REG_DAT_DEV3_I2C_MASK
            } else {
                self.bits & !DevAddrTable3Loc::REG_DAT_DEV3_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV3_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev3_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable3Loc::REG_DAT_DEV3_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable3Loc::REG_DAT_DEV3_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable3Loc::REG_DAT_DEV3_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV3_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev3_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable3Loc::REG_DAT_DEV3_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable3Loc::REG_DAT_DEV3_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable3Loc::REG_DAT_DEV3_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV3_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev3_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable3Loc::REG_DAT_DEV3_STATIC_ADDR_MASK)
                | ((value << DevAddrTable3Loc::REG_DAT_DEV3_STATIC_ADDR_OFFSET)
                    & DevAddrTable3Loc::REG_DAT_DEV3_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable3LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable3Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable3LocVal {
        DevAddrTable3LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable3LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable3LocVal) -> DevAddrTable3LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE4_LOC Register
///
/// NA
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **REG_DAT_DEV4_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV4_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV4_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV4_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable4Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable4Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable4Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV4_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV4_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV4_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV4_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV4_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV4_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV4_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV4_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV4_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV4_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV4_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV4_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV4_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV4_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev4_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV4_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV4_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev4_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV4_I2C_MASK);
    }

    /// Clear REG_DAT_DEV4_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev4_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV4_I2C_MASK);
    }

    /// **REG_DAT_DEV4_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev4_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV4_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV4_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV4_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev4_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV4_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV4_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV4_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV4_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev4_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV4_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV4_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV4_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev4_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV4_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV4_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV4_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV4_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev4_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV4_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV4_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV4_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev4_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV4_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV4_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV4_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE4_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable4loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable4loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable4LocVal {
    bits: u32,
}

impl DevAddrTable4LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV4_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev4_i2c(&self) -> bool {
        (self.bits & DevAddrTable4Loc::REG_DAT_DEV4_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV4_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev4_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable4Loc::REG_DAT_DEV4_NACK_RETRY_CNT_MASK)
            >> DevAddrTable4Loc::REG_DAT_DEV4_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV4_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev4_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable4Loc::REG_DAT_DEV4_DYNAMIC_ADDR_MASK)
            >> DevAddrTable4Loc::REG_DAT_DEV4_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV4_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev4_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable4Loc::REG_DAT_DEV4_STATIC_ADDR_MASK)
            >> DevAddrTable4Loc::REG_DAT_DEV4_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV4_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev4_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable4Loc::REG_DAT_DEV4_I2C_MASK
            } else {
                self.bits & !DevAddrTable4Loc::REG_DAT_DEV4_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV4_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev4_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable4Loc::REG_DAT_DEV4_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable4Loc::REG_DAT_DEV4_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable4Loc::REG_DAT_DEV4_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV4_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev4_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable4Loc::REG_DAT_DEV4_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable4Loc::REG_DAT_DEV4_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable4Loc::REG_DAT_DEV4_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV4_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev4_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable4Loc::REG_DAT_DEV4_STATIC_ADDR_MASK)
                | ((value << DevAddrTable4Loc::REG_DAT_DEV4_STATIC_ADDR_OFFSET)
                    & DevAddrTable4Loc::REG_DAT_DEV4_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable4LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable4Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable4LocVal {
        DevAddrTable4LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable4LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable4LocVal) -> DevAddrTable4LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE5_LOC Register
///
/// NA
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **REG_DAT_DEV5_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV5_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV5_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV5_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable5Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable5Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable5Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV5_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV5_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV5_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV5_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV5_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV5_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV5_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV5_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV5_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV5_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV5_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV5_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV5_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV5_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev5_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV5_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV5_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev5_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV5_I2C_MASK);
    }

    /// Clear REG_DAT_DEV5_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev5_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV5_I2C_MASK);
    }

    /// **REG_DAT_DEV5_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev5_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV5_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV5_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV5_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev5_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV5_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV5_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV5_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV5_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev5_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV5_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV5_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV5_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev5_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV5_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV5_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV5_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV5_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev5_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV5_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV5_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV5_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev5_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV5_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV5_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV5_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE5_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable5loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable5loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable5LocVal {
    bits: u32,
}

impl DevAddrTable5LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV5_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev5_i2c(&self) -> bool {
        (self.bits & DevAddrTable5Loc::REG_DAT_DEV5_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV5_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev5_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable5Loc::REG_DAT_DEV5_NACK_RETRY_CNT_MASK)
            >> DevAddrTable5Loc::REG_DAT_DEV5_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV5_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev5_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable5Loc::REG_DAT_DEV5_DYNAMIC_ADDR_MASK)
            >> DevAddrTable5Loc::REG_DAT_DEV5_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV5_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev5_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable5Loc::REG_DAT_DEV5_STATIC_ADDR_MASK)
            >> DevAddrTable5Loc::REG_DAT_DEV5_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV5_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev5_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable5Loc::REG_DAT_DEV5_I2C_MASK
            } else {
                self.bits & !DevAddrTable5Loc::REG_DAT_DEV5_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV5_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev5_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable5Loc::REG_DAT_DEV5_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable5Loc::REG_DAT_DEV5_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable5Loc::REG_DAT_DEV5_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV5_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev5_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable5Loc::REG_DAT_DEV5_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable5Loc::REG_DAT_DEV5_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable5Loc::REG_DAT_DEV5_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV5_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev5_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable5Loc::REG_DAT_DEV5_STATIC_ADDR_MASK)
                | ((value << DevAddrTable5Loc::REG_DAT_DEV5_STATIC_ADDR_OFFSET)
                    & DevAddrTable5Loc::REG_DAT_DEV5_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable5LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable5Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable5LocVal {
        DevAddrTable5LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable5LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable5LocVal) -> DevAddrTable5LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE6_LOC Register
///
/// NA
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **REG_DAT_DEV6_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV6_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV6_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV6_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable6Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable6Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable6Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV6_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV6_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV6_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV6_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV6_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV6_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV6_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV6_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV6_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV6_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV6_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV6_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV6_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV6_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev6_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV6_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV6_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev6_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV6_I2C_MASK);
    }

    /// Clear REG_DAT_DEV6_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev6_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV6_I2C_MASK);
    }

    /// **REG_DAT_DEV6_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev6_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV6_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV6_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV6_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev6_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV6_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV6_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV6_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV6_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev6_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV6_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV6_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV6_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev6_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV6_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV6_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV6_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV6_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev6_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV6_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV6_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV6_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev6_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV6_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV6_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV6_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE6_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable6loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable6loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable6LocVal {
    bits: u32,
}

impl DevAddrTable6LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV6_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev6_i2c(&self) -> bool {
        (self.bits & DevAddrTable6Loc::REG_DAT_DEV6_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV6_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev6_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable6Loc::REG_DAT_DEV6_NACK_RETRY_CNT_MASK)
            >> DevAddrTable6Loc::REG_DAT_DEV6_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV6_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev6_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable6Loc::REG_DAT_DEV6_DYNAMIC_ADDR_MASK)
            >> DevAddrTable6Loc::REG_DAT_DEV6_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV6_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev6_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable6Loc::REG_DAT_DEV6_STATIC_ADDR_MASK)
            >> DevAddrTable6Loc::REG_DAT_DEV6_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV6_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev6_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable6Loc::REG_DAT_DEV6_I2C_MASK
            } else {
                self.bits & !DevAddrTable6Loc::REG_DAT_DEV6_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV6_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev6_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable6Loc::REG_DAT_DEV6_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable6Loc::REG_DAT_DEV6_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable6Loc::REG_DAT_DEV6_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV6_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev6_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable6Loc::REG_DAT_DEV6_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable6Loc::REG_DAT_DEV6_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable6Loc::REG_DAT_DEV6_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV6_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev6_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable6Loc::REG_DAT_DEV6_STATIC_ADDR_MASK)
                | ((value << DevAddrTable6Loc::REG_DAT_DEV6_STATIC_ADDR_OFFSET)
                    & DevAddrTable6Loc::REG_DAT_DEV6_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable6LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable6Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable6LocVal {
        DevAddrTable6LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable6LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable6LocVal) -> DevAddrTable6LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE7_LOC Register
///
/// NA
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **REG_DAT_DEV7_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV7_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV7_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV7_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable7Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable7Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable7Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV7_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV7_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV7_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV7_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV7_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV7_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV7_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV7_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV7_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV7_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV7_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV7_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV7_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV7_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev7_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV7_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV7_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev7_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV7_I2C_MASK);
    }

    /// Clear REG_DAT_DEV7_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev7_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV7_I2C_MASK);
    }

    /// **REG_DAT_DEV7_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev7_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV7_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV7_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV7_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev7_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV7_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV7_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV7_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV7_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev7_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV7_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV7_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV7_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev7_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV7_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV7_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV7_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV7_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev7_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV7_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV7_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV7_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev7_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV7_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV7_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV7_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE7_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable7loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable7loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable7LocVal {
    bits: u32,
}

impl DevAddrTable7LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV7_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev7_i2c(&self) -> bool {
        (self.bits & DevAddrTable7Loc::REG_DAT_DEV7_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV7_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev7_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable7Loc::REG_DAT_DEV7_NACK_RETRY_CNT_MASK)
            >> DevAddrTable7Loc::REG_DAT_DEV7_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV7_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev7_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable7Loc::REG_DAT_DEV7_DYNAMIC_ADDR_MASK)
            >> DevAddrTable7Loc::REG_DAT_DEV7_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV7_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev7_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable7Loc::REG_DAT_DEV7_STATIC_ADDR_MASK)
            >> DevAddrTable7Loc::REG_DAT_DEV7_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV7_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev7_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable7Loc::REG_DAT_DEV7_I2C_MASK
            } else {
                self.bits & !DevAddrTable7Loc::REG_DAT_DEV7_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV7_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev7_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable7Loc::REG_DAT_DEV7_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable7Loc::REG_DAT_DEV7_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable7Loc::REG_DAT_DEV7_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV7_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev7_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable7Loc::REG_DAT_DEV7_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable7Loc::REG_DAT_DEV7_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable7Loc::REG_DAT_DEV7_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV7_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev7_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable7Loc::REG_DAT_DEV7_STATIC_ADDR_MASK)
                | ((value << DevAddrTable7Loc::REG_DAT_DEV7_STATIC_ADDR_OFFSET)
                    & DevAddrTable7Loc::REG_DAT_DEV7_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable7LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable7Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable7LocVal {
        DevAddrTable7LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable7LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable7LocVal) -> DevAddrTable7LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE8_LOC Register
///
/// NA
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **REG_DAT_DEV8_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV8_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV8_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV8_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable8Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable8Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable8Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV8_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV8_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV8_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV8_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV8_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV8_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV8_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV8_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV8_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV8_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV8_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV8_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV8_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV8_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev8_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV8_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV8_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev8_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV8_I2C_MASK);
    }

    /// Clear REG_DAT_DEV8_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev8_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV8_I2C_MASK);
    }

    /// **REG_DAT_DEV8_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev8_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV8_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV8_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV8_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev8_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV8_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV8_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV8_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV8_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev8_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV8_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV8_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV8_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev8_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV8_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV8_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV8_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV8_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev8_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV8_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV8_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV8_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev8_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV8_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV8_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV8_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE8_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable8loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable8loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable8LocVal {
    bits: u32,
}

impl DevAddrTable8LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV8_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev8_i2c(&self) -> bool {
        (self.bits & DevAddrTable8Loc::REG_DAT_DEV8_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV8_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev8_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable8Loc::REG_DAT_DEV8_NACK_RETRY_CNT_MASK)
            >> DevAddrTable8Loc::REG_DAT_DEV8_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV8_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev8_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable8Loc::REG_DAT_DEV8_DYNAMIC_ADDR_MASK)
            >> DevAddrTable8Loc::REG_DAT_DEV8_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV8_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev8_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable8Loc::REG_DAT_DEV8_STATIC_ADDR_MASK)
            >> DevAddrTable8Loc::REG_DAT_DEV8_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV8_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev8_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable8Loc::REG_DAT_DEV8_I2C_MASK
            } else {
                self.bits & !DevAddrTable8Loc::REG_DAT_DEV8_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV8_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev8_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable8Loc::REG_DAT_DEV8_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable8Loc::REG_DAT_DEV8_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable8Loc::REG_DAT_DEV8_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV8_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev8_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable8Loc::REG_DAT_DEV8_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable8Loc::REG_DAT_DEV8_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable8Loc::REG_DAT_DEV8_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV8_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev8_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable8Loc::REG_DAT_DEV8_STATIC_ADDR_MASK)
                | ((value << DevAddrTable8Loc::REG_DAT_DEV8_STATIC_ADDR_OFFSET)
                    & DevAddrTable8Loc::REG_DAT_DEV8_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable8LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable8Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable8LocVal {
        DevAddrTable8LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable8LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable8LocVal) -> DevAddrTable8LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE9_LOC Register
///
/// NA
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **REG_DAT_DEV9_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV9_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV9_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV9_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable9Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable9Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable9Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV9_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV9_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV9_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV9_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV9_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV9_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV9_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV9_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV9_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV9_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV9_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV9_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV9_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV9_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev9_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV9_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV9_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev9_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV9_I2C_MASK);
    }

    /// Clear REG_DAT_DEV9_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev9_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DAT_DEV9_I2C_MASK);
    }

    /// **REG_DAT_DEV9_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev9_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV9_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV9_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV9_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev9_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV9_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV9_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV9_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV9_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev9_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV9_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV9_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV9_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev9_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV9_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV9_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV9_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV9_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev9_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV9_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV9_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV9_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev9_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV9_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV9_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV9_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE9_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable9loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable9loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable9LocVal {
    bits: u32,
}

impl DevAddrTable9LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV9_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev9_i2c(&self) -> bool {
        (self.bits & DevAddrTable9Loc::REG_DAT_DEV9_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV9_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev9_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable9Loc::REG_DAT_DEV9_NACK_RETRY_CNT_MASK)
            >> DevAddrTable9Loc::REG_DAT_DEV9_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV9_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev9_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable9Loc::REG_DAT_DEV9_DYNAMIC_ADDR_MASK)
            >> DevAddrTable9Loc::REG_DAT_DEV9_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV9_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev9_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable9Loc::REG_DAT_DEV9_STATIC_ADDR_MASK)
            >> DevAddrTable9Loc::REG_DAT_DEV9_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV9_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev9_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable9Loc::REG_DAT_DEV9_I2C_MASK
            } else {
                self.bits & !DevAddrTable9Loc::REG_DAT_DEV9_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV9_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev9_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable9Loc::REG_DAT_DEV9_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable9Loc::REG_DAT_DEV9_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable9Loc::REG_DAT_DEV9_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV9_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev9_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable9Loc::REG_DAT_DEV9_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable9Loc::REG_DAT_DEV9_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable9Loc::REG_DAT_DEV9_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV9_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev9_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable9Loc::REG_DAT_DEV9_STATIC_ADDR_MASK)
                | ((value << DevAddrTable9Loc::REG_DAT_DEV9_STATIC_ADDR_OFFSET)
                    & DevAddrTable9Loc::REG_DAT_DEV9_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable9LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable9Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable9LocVal {
        DevAddrTable9LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable9LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable9LocVal) -> DevAddrTable9LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE10_LOC Register
///
/// NA
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **REG_DAT_DEV10_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV10_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV10_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV10_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable10Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable10Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable10Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV10_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV10_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV10_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV10_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV10_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV10_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV10_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV10_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV10_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV10_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV10_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV10_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV10_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV10_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev10_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV10_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV10_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev10_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV10_I2C_MASK);
    }

    /// Clear REG_DAT_DEV10_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev10_i2c(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_DAT_DEV10_I2C_MASK);
    }

    /// **REG_DAT_DEV10_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev10_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV10_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV10_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV10_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev10_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV10_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV10_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV10_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV10_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev10_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV10_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV10_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV10_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev10_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV10_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV10_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV10_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV10_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev10_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV10_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV10_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV10_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev10_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV10_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV10_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV10_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE10_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable10loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable10loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable10LocVal {
    bits: u32,
}

impl DevAddrTable10LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV10_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev10_i2c(&self) -> bool {
        (self.bits & DevAddrTable10Loc::REG_DAT_DEV10_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV10_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev10_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable10Loc::REG_DAT_DEV10_NACK_RETRY_CNT_MASK)
            >> DevAddrTable10Loc::REG_DAT_DEV10_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV10_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev10_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable10Loc::REG_DAT_DEV10_DYNAMIC_ADDR_MASK)
            >> DevAddrTable10Loc::REG_DAT_DEV10_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV10_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev10_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable10Loc::REG_DAT_DEV10_STATIC_ADDR_MASK)
            >> DevAddrTable10Loc::REG_DAT_DEV10_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV10_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev10_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable10Loc::REG_DAT_DEV10_I2C_MASK
            } else {
                self.bits & !DevAddrTable10Loc::REG_DAT_DEV10_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV10_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev10_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable10Loc::REG_DAT_DEV10_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable10Loc::REG_DAT_DEV10_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable10Loc::REG_DAT_DEV10_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV10_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev10_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable10Loc::REG_DAT_DEV10_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable10Loc::REG_DAT_DEV10_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable10Loc::REG_DAT_DEV10_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV10_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev10_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable10Loc::REG_DAT_DEV10_STATIC_ADDR_MASK)
                | ((value << DevAddrTable10Loc::REG_DAT_DEV10_STATIC_ADDR_OFFSET)
                    & DevAddrTable10Loc::REG_DAT_DEV10_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable10LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable10Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable10LocVal {
        DevAddrTable10LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable10LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable10LocVal) -> DevAddrTable10LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE11_LOC Register
///
/// NA
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **REG_DAT_DEV11_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV11_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV11_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV11_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable11Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable11Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable11Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV11_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV11_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV11_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV11_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV11_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV11_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV11_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV11_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV11_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV11_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV11_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV11_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV11_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV11_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev11_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV11_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV11_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev11_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV11_I2C_MASK);
    }

    /// Clear REG_DAT_DEV11_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev11_i2c(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_DAT_DEV11_I2C_MASK);
    }

    /// **REG_DAT_DEV11_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev11_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV11_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV11_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV11_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev11_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV11_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV11_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV11_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV11_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev11_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV11_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV11_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV11_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev11_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV11_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV11_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV11_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV11_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev11_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV11_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV11_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV11_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev11_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV11_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV11_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV11_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE11_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable11loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable11loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable11LocVal {
    bits: u32,
}

impl DevAddrTable11LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV11_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev11_i2c(&self) -> bool {
        (self.bits & DevAddrTable11Loc::REG_DAT_DEV11_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV11_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev11_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable11Loc::REG_DAT_DEV11_NACK_RETRY_CNT_MASK)
            >> DevAddrTable11Loc::REG_DAT_DEV11_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV11_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev11_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable11Loc::REG_DAT_DEV11_DYNAMIC_ADDR_MASK)
            >> DevAddrTable11Loc::REG_DAT_DEV11_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV11_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev11_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable11Loc::REG_DAT_DEV11_STATIC_ADDR_MASK)
            >> DevAddrTable11Loc::REG_DAT_DEV11_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV11_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev11_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable11Loc::REG_DAT_DEV11_I2C_MASK
            } else {
                self.bits & !DevAddrTable11Loc::REG_DAT_DEV11_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV11_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev11_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable11Loc::REG_DAT_DEV11_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable11Loc::REG_DAT_DEV11_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable11Loc::REG_DAT_DEV11_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV11_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev11_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable11Loc::REG_DAT_DEV11_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable11Loc::REG_DAT_DEV11_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable11Loc::REG_DAT_DEV11_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV11_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev11_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable11Loc::REG_DAT_DEV11_STATIC_ADDR_MASK)
                | ((value << DevAddrTable11Loc::REG_DAT_DEV11_STATIC_ADDR_OFFSET)
                    & DevAddrTable11Loc::REG_DAT_DEV11_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable11LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable11Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable11LocVal {
        DevAddrTable11LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable11LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable11LocVal) -> DevAddrTable11LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR_TABLE12_LOC Register
///
/// NA
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **REG_DAT_DEV12_I2C** `[31]` (RW)
///
///   Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
///
/// - **REG_DAT_DEV12_NACK_RETRY_CNT** `[30:29]` (RW)
///
///   This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
///
/// - **REG_DAT_DEV12_DYNAMIC_ADDR** `[23:16]` (RW)
///
///   Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
///
/// - **REG_DAT_DEV12_STATIC_ADDR** `[6:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DevAddrTable12Loc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddrTable12Loc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddrTable12Loc {
    // ======== Field Constants ========

    /// **REG_DAT_DEV12_I2C**
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_DAT_DEV12_I2C_OFFSET: u32 = 31;
    pub const REG_DAT_DEV12_I2C_MASK: u32 = 0x80000000;
    pub const REG_DAT_DEV12_I2C: u32 = 1 << 31;

    /// **REG_DAT_DEV12_NACK_RETRY_CNT**
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const REG_DAT_DEV12_NACK_RETRY_CNT_OFFSET: u32 = 29;
    pub const REG_DAT_DEV12_NACK_RETRY_CNT_MASK: u32 = 0x60000000;

    /// **REG_DAT_DEV12_DYNAMIC_ADDR**
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_DAT_DEV12_DYNAMIC_ADDR_OFFSET: u32 = 16;
    pub const REG_DAT_DEV12_DYNAMIC_ADDR_MASK: u32 = 0x00ff0000;

    /// **REG_DAT_DEV12_STATIC_ADDR**
    ///
    /// NA
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REG_DAT_DEV12_STATIC_ADDR_OFFSET: u32 = 0;
    pub const REG_DAT_DEV12_STATIC_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REG_DAT_DEV12_I2C** `[31]` (RW)
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev12_i2c(&self) -> bool {
        (self.value.read_bits() & Self::REG_DAT_DEV12_I2C_MASK) != 0
    }

    /// Set REG_DAT_DEV12_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dat_dev12_i2c(&self) {
        self.value.modify_bits(|r| r | Self::REG_DAT_DEV12_I2C_MASK);
    }

    /// Clear REG_DAT_DEV12_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dat_dev12_i2c(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_DAT_DEV12_I2C_MASK);
    }

    /// **REG_DAT_DEV12_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev12_nack_retry_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV12_NACK_RETRY_CNT_MASK)
            >> Self::REG_DAT_DEV12_NACK_RETRY_CNT_OFFSET
    }

    /// Write **REG_DAT_DEV12_NACK_RETRY_CNT** `[30:29]` (RW)
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn write_reg_dat_dev12_nack_retry_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV12_NACK_RETRY_CNT_MASK)
                | ((value << Self::REG_DAT_DEV12_NACK_RETRY_CNT_OFFSET)
                    & Self::REG_DAT_DEV12_NACK_RETRY_CNT_MASK)
        });
    }

    /// **REG_DAT_DEV12_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev12_dynamic_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV12_DYNAMIC_ADDR_MASK)
            >> Self::REG_DAT_DEV12_DYNAMIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV12_DYNAMIC_ADDR** `[23:16]` (RW)
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn write_reg_dat_dev12_dynamic_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV12_DYNAMIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV12_DYNAMIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV12_DYNAMIC_ADDR_MASK)
        });
    }

    /// **REG_DAT_DEV12_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_dev12_static_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DEV12_STATIC_ADDR_MASK)
            >> Self::REG_DAT_DEV12_STATIC_ADDR_OFFSET
    }

    /// Write **REG_DAT_DEV12_STATIC_ADDR** `[6:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_dev12_static_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DEV12_STATIC_ADDR_MASK)
                | ((value << Self::REG_DAT_DEV12_STATIC_ADDR_OFFSET)
                    & Self::REG_DAT_DEV12_STATIC_ADDR_MASK)
        });
    }
}

/// DEV_ADDR_TABLE12_LOC value type for batch read/modify/write operations
///
/// Use `devaddrtable12loc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddrtable12loc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrTable12LocVal {
    bits: u32,
}

impl DevAddrTable12LocVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_DAT_DEV12_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn reg_dat_dev12_i2c(&self) -> bool {
        (self.bits & DevAddrTable12Loc::REG_DAT_DEV12_I2C_MASK) != 0
    }

    /// Read **REG_DAT_DEV12_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn reg_dat_dev12_nack_retry_cnt(&self) -> u32 {
        (self.bits & DevAddrTable12Loc::REG_DAT_DEV12_NACK_RETRY_CNT_MASK)
            >> DevAddrTable12Loc::REG_DAT_DEV12_NACK_RETRY_CNT_OFFSET
    }

    /// Read **REG_DAT_DEV12_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn reg_dat_dev12_dynamic_addr(&self) -> u32 {
        (self.bits & DevAddrTable12Loc::REG_DAT_DEV12_DYNAMIC_ADDR_MASK)
            >> DevAddrTable12Loc::REG_DAT_DEV12_DYNAMIC_ADDR_OFFSET
    }

    /// Read **REG_DAT_DEV12_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_dev12_static_addr(&self) -> u32 {
        (self.bits & DevAddrTable12Loc::REG_DAT_DEV12_STATIC_ADDR_MASK)
            >> DevAddrTable12Loc::REG_DAT_DEV12_STATIC_ADDR_OFFSET
    }

    /// Set **REG_DAT_DEV12_I2C** `[31:31]`
    ///
    /// Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub const fn with_reg_dat_dev12_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevAddrTable12Loc::REG_DAT_DEV12_I2C_MASK
            } else {
                self.bits & !DevAddrTable12Loc::REG_DAT_DEV12_I2C_MASK
            },
        }
    }

    /// Set **REG_DAT_DEV12_NACK_RETRY_CNT** `[30:29]`
    ///
    /// This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub const fn with_reg_dat_dev12_nack_retry_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable12Loc::REG_DAT_DEV12_NACK_RETRY_CNT_MASK)
                | ((value << DevAddrTable12Loc::REG_DAT_DEV12_NACK_RETRY_CNT_OFFSET)
                    & DevAddrTable12Loc::REG_DAT_DEV12_NACK_RETRY_CNT_MASK),
        }
    }

    /// Set **REG_DAT_DEV12_DYNAMIC_ADDR** `[23:16]`
    ///
    /// Device Dynamic Address with parity, The MSB,bit[23], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub const fn with_reg_dat_dev12_dynamic_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable12Loc::REG_DAT_DEV12_DYNAMIC_ADDR_MASK)
                | ((value << DevAddrTable12Loc::REG_DAT_DEV12_DYNAMIC_ADDR_OFFSET)
                    & DevAddrTable12Loc::REG_DAT_DEV12_DYNAMIC_ADDR_MASK),
        }
    }

    /// Set **REG_DAT_DEV12_STATIC_ADDR** `[6:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_dev12_static_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddrTable12Loc::REG_DAT_DEV12_STATIC_ADDR_MASK)
                | ((value << DevAddrTable12Loc::REG_DAT_DEV12_STATIC_ADDR_OFFSET)
                    & DevAddrTable12Loc::REG_DAT_DEV12_STATIC_ADDR_MASK),
        }
    }
}

impl Default for DevAddrTable12LocVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevAddrTable12Loc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrTable12LocVal {
        DevAddrTable12LocVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrTable12LocVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrTable12LocVal) -> DevAddrTable12LocVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_CHAR_TABLE1_LOC1 Register
///
/// NA
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **DCT_DEV1_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable1Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable1Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable1Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV1_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV1_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV1_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV1_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev1_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV1_LOC1_MASK) >> Self::DCT_DEV1_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE1_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable1loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable1loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable1Loc1Val {
    bits: u32,
}

impl DevCharTable1Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV1_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev1_loc1(&self) -> u32 {
        (self.bits & DevCharTable1Loc1::DCT_DEV1_LOC1_MASK)
            >> DevCharTable1Loc1::DCT_DEV1_LOC1_OFFSET
    }
}

impl Default for DevCharTable1Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable1Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable1Loc1Val {
        DevCharTable1Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE1_LOC2 Register
///
/// NA
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **DCT_DEV1_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable1Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable1Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable1Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV1_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV1_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV1_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV1_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev1_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV1_LOC2_MASK) >> Self::DCT_DEV1_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE1_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable1loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable1loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable1Loc2Val {
    bits: u32,
}

impl DevCharTable1Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV1_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev1_loc2(&self) -> u32 {
        (self.bits & DevCharTable1Loc2::DCT_DEV1_LOC2_MASK)
            >> DevCharTable1Loc2::DCT_DEV1_LOC2_OFFSET
    }
}

impl Default for DevCharTable1Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable1Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable1Loc2Val {
        DevCharTable1Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE1_LOC3 Register
///
/// NA
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **DCT_DEV1_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable1Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable1Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable1Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV1_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV1_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV1_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV1_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev1_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV1_LOC3_MASK) >> Self::DCT_DEV1_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE1_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable1loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable1loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable1Loc3Val {
    bits: u32,
}

impl DevCharTable1Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV1_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev1_loc3(&self) -> u32 {
        (self.bits & DevCharTable1Loc3::DCT_DEV1_LOC3_MASK)
            >> DevCharTable1Loc3::DCT_DEV1_LOC3_OFFSET
    }
}

impl Default for DevCharTable1Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable1Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable1Loc3Val {
        DevCharTable1Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE1_LOC4 Register
///
/// NA
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **DCT_DEV1_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable1Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable1Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable1Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV1_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV1_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV1_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV1_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev1_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV1_LOC4_MASK) >> Self::DCT_DEV1_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE1_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable1loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable1loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable1Loc4Val {
    bits: u32,
}

impl DevCharTable1Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV1_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev1_loc4(&self) -> u32 {
        (self.bits & DevCharTable1Loc4::DCT_DEV1_LOC4_MASK)
            >> DevCharTable1Loc4::DCT_DEV1_LOC4_OFFSET
    }
}

impl Default for DevCharTable1Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable1Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable1Loc4Val {
        DevCharTable1Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE2_LOC1 Register
///
/// NA
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **DCT_DEV2_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable2Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable2Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable2Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV2_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV2_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV2_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV2_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev2_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV2_LOC1_MASK) >> Self::DCT_DEV2_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE2_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable2loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable2loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable2Loc1Val {
    bits: u32,
}

impl DevCharTable2Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV2_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev2_loc1(&self) -> u32 {
        (self.bits & DevCharTable2Loc1::DCT_DEV2_LOC1_MASK)
            >> DevCharTable2Loc1::DCT_DEV2_LOC1_OFFSET
    }
}

impl Default for DevCharTable2Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable2Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable2Loc1Val {
        DevCharTable2Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE2_LOC2 Register
///
/// NA
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **DCT_DEV2_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable2Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable2Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable2Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV2_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV2_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV2_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV2_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev2_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV2_LOC2_MASK) >> Self::DCT_DEV2_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE2_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable2loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable2loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable2Loc2Val {
    bits: u32,
}

impl DevCharTable2Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV2_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev2_loc2(&self) -> u32 {
        (self.bits & DevCharTable2Loc2::DCT_DEV2_LOC2_MASK)
            >> DevCharTable2Loc2::DCT_DEV2_LOC2_OFFSET
    }
}

impl Default for DevCharTable2Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable2Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable2Loc2Val {
        DevCharTable2Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE2_LOC3 Register
///
/// NA
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **DCT_DEV2_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable2Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable2Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable2Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV2_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV2_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV2_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV2_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev2_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV2_LOC3_MASK) >> Self::DCT_DEV2_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE2_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable2loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable2loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable2Loc3Val {
    bits: u32,
}

impl DevCharTable2Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV2_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev2_loc3(&self) -> u32 {
        (self.bits & DevCharTable2Loc3::DCT_DEV2_LOC3_MASK)
            >> DevCharTable2Loc3::DCT_DEV2_LOC3_OFFSET
    }
}

impl Default for DevCharTable2Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable2Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable2Loc3Val {
        DevCharTable2Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE2_LOC4 Register
///
/// NA
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **DCT_DEV2_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable2Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable2Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable2Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV2_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV2_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV2_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV2_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev2_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV2_LOC4_MASK) >> Self::DCT_DEV2_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE2_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable2loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable2loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable2Loc4Val {
    bits: u32,
}

impl DevCharTable2Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV2_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev2_loc4(&self) -> u32 {
        (self.bits & DevCharTable2Loc4::DCT_DEV2_LOC4_MASK)
            >> DevCharTable2Loc4::DCT_DEV2_LOC4_OFFSET
    }
}

impl Default for DevCharTable2Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable2Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable2Loc4Val {
        DevCharTable2Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE3_LOC1 Register
///
/// NA
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **DCT_DEV3_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable3Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable3Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable3Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV3_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV3_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV3_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV3_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev3_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV3_LOC1_MASK) >> Self::DCT_DEV3_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE3_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable3loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable3loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable3Loc1Val {
    bits: u32,
}

impl DevCharTable3Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV3_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev3_loc1(&self) -> u32 {
        (self.bits & DevCharTable3Loc1::DCT_DEV3_LOC1_MASK)
            >> DevCharTable3Loc1::DCT_DEV3_LOC1_OFFSET
    }
}

impl Default for DevCharTable3Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable3Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable3Loc1Val {
        DevCharTable3Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE3_LOC2 Register
///
/// NA
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **DCT_DEV3_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable3Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable3Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable3Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV3_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV3_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV3_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV3_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev3_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV3_LOC2_MASK) >> Self::DCT_DEV3_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE3_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable3loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable3loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable3Loc2Val {
    bits: u32,
}

impl DevCharTable3Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV3_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev3_loc2(&self) -> u32 {
        (self.bits & DevCharTable3Loc2::DCT_DEV3_LOC2_MASK)
            >> DevCharTable3Loc2::DCT_DEV3_LOC2_OFFSET
    }
}

impl Default for DevCharTable3Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable3Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable3Loc2Val {
        DevCharTable3Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE3_LOC3 Register
///
/// NA
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **DCT_DEV3_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable3Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable3Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable3Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV3_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV3_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV3_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV3_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev3_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV3_LOC3_MASK) >> Self::DCT_DEV3_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE3_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable3loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable3loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable3Loc3Val {
    bits: u32,
}

impl DevCharTable3Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV3_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev3_loc3(&self) -> u32 {
        (self.bits & DevCharTable3Loc3::DCT_DEV3_LOC3_MASK)
            >> DevCharTable3Loc3::DCT_DEV3_LOC3_OFFSET
    }
}

impl Default for DevCharTable3Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable3Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable3Loc3Val {
        DevCharTable3Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE3_LOC4 Register
///
/// NA
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **DCT_DEV3_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable3Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable3Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable3Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV3_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV3_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV3_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV3_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev3_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV3_LOC4_MASK) >> Self::DCT_DEV3_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE3_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable3loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable3loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable3Loc4Val {
    bits: u32,
}

impl DevCharTable3Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV3_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev3_loc4(&self) -> u32 {
        (self.bits & DevCharTable3Loc4::DCT_DEV3_LOC4_MASK)
            >> DevCharTable3Loc4::DCT_DEV3_LOC4_OFFSET
    }
}

impl Default for DevCharTable3Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable3Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable3Loc4Val {
        DevCharTable3Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE4_LOC1 Register
///
/// NA
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **DCT_DEV4_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable4Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable4Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable4Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV4_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV4_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV4_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV4_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev4_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV4_LOC1_MASK) >> Self::DCT_DEV4_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE4_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable4loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable4loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable4Loc1Val {
    bits: u32,
}

impl DevCharTable4Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV4_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev4_loc1(&self) -> u32 {
        (self.bits & DevCharTable4Loc1::DCT_DEV4_LOC1_MASK)
            >> DevCharTable4Loc1::DCT_DEV4_LOC1_OFFSET
    }
}

impl Default for DevCharTable4Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable4Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable4Loc1Val {
        DevCharTable4Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE4_LOC2 Register
///
/// NA
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **DCT_DEV4_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable4Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable4Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable4Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV4_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV4_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV4_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV4_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev4_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV4_LOC2_MASK) >> Self::DCT_DEV4_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE4_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable4loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable4loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable4Loc2Val {
    bits: u32,
}

impl DevCharTable4Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV4_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev4_loc2(&self) -> u32 {
        (self.bits & DevCharTable4Loc2::DCT_DEV4_LOC2_MASK)
            >> DevCharTable4Loc2::DCT_DEV4_LOC2_OFFSET
    }
}

impl Default for DevCharTable4Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable4Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable4Loc2Val {
        DevCharTable4Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE4_LOC3 Register
///
/// NA
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **DCT_DEV4_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable4Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable4Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable4Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV4_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV4_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV4_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV4_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev4_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV4_LOC3_MASK) >> Self::DCT_DEV4_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE4_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable4loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable4loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable4Loc3Val {
    bits: u32,
}

impl DevCharTable4Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV4_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev4_loc3(&self) -> u32 {
        (self.bits & DevCharTable4Loc3::DCT_DEV4_LOC3_MASK)
            >> DevCharTable4Loc3::DCT_DEV4_LOC3_OFFSET
    }
}

impl Default for DevCharTable4Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable4Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable4Loc3Val {
        DevCharTable4Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE4_LOC4 Register
///
/// NA
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **DCT_DEV4_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable4Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable4Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable4Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV4_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV4_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV4_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV4_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev4_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV4_LOC4_MASK) >> Self::DCT_DEV4_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE4_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable4loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable4loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable4Loc4Val {
    bits: u32,
}

impl DevCharTable4Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV4_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev4_loc4(&self) -> u32 {
        (self.bits & DevCharTable4Loc4::DCT_DEV4_LOC4_MASK)
            >> DevCharTable4Loc4::DCT_DEV4_LOC4_OFFSET
    }
}

impl Default for DevCharTable4Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable4Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable4Loc4Val {
        DevCharTable4Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE5_LOC1 Register
///
/// NA
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **DCT_DEV5_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable5Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable5Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable5Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV5_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV5_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV5_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV5_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev5_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV5_LOC1_MASK) >> Self::DCT_DEV5_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE5_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable5loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable5loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable5Loc1Val {
    bits: u32,
}

impl DevCharTable5Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV5_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev5_loc1(&self) -> u32 {
        (self.bits & DevCharTable5Loc1::DCT_DEV5_LOC1_MASK)
            >> DevCharTable5Loc1::DCT_DEV5_LOC1_OFFSET
    }
}

impl Default for DevCharTable5Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable5Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable5Loc1Val {
        DevCharTable5Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE5_LOC2 Register
///
/// NA
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **DCT_DEV5_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable5Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable5Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable5Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV5_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV5_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV5_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV5_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev5_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV5_LOC2_MASK) >> Self::DCT_DEV5_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE5_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable5loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable5loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable5Loc2Val {
    bits: u32,
}

impl DevCharTable5Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV5_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev5_loc2(&self) -> u32 {
        (self.bits & DevCharTable5Loc2::DCT_DEV5_LOC2_MASK)
            >> DevCharTable5Loc2::DCT_DEV5_LOC2_OFFSET
    }
}

impl Default for DevCharTable5Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable5Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable5Loc2Val {
        DevCharTable5Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE5_LOC3 Register
///
/// NA
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **DCT_DEV5_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable5Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable5Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable5Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV5_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV5_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV5_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV5_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev5_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV5_LOC3_MASK) >> Self::DCT_DEV5_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE5_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable5loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable5loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable5Loc3Val {
    bits: u32,
}

impl DevCharTable5Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV5_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev5_loc3(&self) -> u32 {
        (self.bits & DevCharTable5Loc3::DCT_DEV5_LOC3_MASK)
            >> DevCharTable5Loc3::DCT_DEV5_LOC3_OFFSET
    }
}

impl Default for DevCharTable5Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable5Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable5Loc3Val {
        DevCharTable5Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE5_LOC4 Register
///
/// NA
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **DCT_DEV5_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable5Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable5Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable5Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV5_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV5_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV5_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV5_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev5_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV5_LOC4_MASK) >> Self::DCT_DEV5_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE5_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable5loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable5loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable5Loc4Val {
    bits: u32,
}

impl DevCharTable5Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV5_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev5_loc4(&self) -> u32 {
        (self.bits & DevCharTable5Loc4::DCT_DEV5_LOC4_MASK)
            >> DevCharTable5Loc4::DCT_DEV5_LOC4_OFFSET
    }
}

impl Default for DevCharTable5Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable5Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable5Loc4Val {
        DevCharTable5Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE6_LOC1 Register
///
/// NA
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **DCT_DEV6_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable6Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable6Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable6Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV6_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV6_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV6_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV6_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev6_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV6_LOC1_MASK) >> Self::DCT_DEV6_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE6_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable6loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable6loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable6Loc1Val {
    bits: u32,
}

impl DevCharTable6Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV6_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev6_loc1(&self) -> u32 {
        (self.bits & DevCharTable6Loc1::DCT_DEV6_LOC1_MASK)
            >> DevCharTable6Loc1::DCT_DEV6_LOC1_OFFSET
    }
}

impl Default for DevCharTable6Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable6Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable6Loc1Val {
        DevCharTable6Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE6_LOC2 Register
///
/// NA
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **DCT_DEV6_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable6Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable6Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable6Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV6_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV6_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV6_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV6_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev6_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV6_LOC2_MASK) >> Self::DCT_DEV6_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE6_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable6loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable6loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable6Loc2Val {
    bits: u32,
}

impl DevCharTable6Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV6_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev6_loc2(&self) -> u32 {
        (self.bits & DevCharTable6Loc2::DCT_DEV6_LOC2_MASK)
            >> DevCharTable6Loc2::DCT_DEV6_LOC2_OFFSET
    }
}

impl Default for DevCharTable6Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable6Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable6Loc2Val {
        DevCharTable6Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE6_LOC3 Register
///
/// NA
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **DCT_DEV6_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable6Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable6Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable6Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV6_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV6_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV6_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV6_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev6_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV6_LOC3_MASK) >> Self::DCT_DEV6_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE6_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable6loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable6loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable6Loc3Val {
    bits: u32,
}

impl DevCharTable6Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV6_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev6_loc3(&self) -> u32 {
        (self.bits & DevCharTable6Loc3::DCT_DEV6_LOC3_MASK)
            >> DevCharTable6Loc3::DCT_DEV6_LOC3_OFFSET
    }
}

impl Default for DevCharTable6Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable6Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable6Loc3Val {
        DevCharTable6Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE6_LOC4 Register
///
/// NA
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **DCT_DEV6_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable6Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable6Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable6Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV6_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV6_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV6_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV6_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev6_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV6_LOC4_MASK) >> Self::DCT_DEV6_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE6_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable6loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable6loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable6Loc4Val {
    bits: u32,
}

impl DevCharTable6Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV6_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev6_loc4(&self) -> u32 {
        (self.bits & DevCharTable6Loc4::DCT_DEV6_LOC4_MASK)
            >> DevCharTable6Loc4::DCT_DEV6_LOC4_OFFSET
    }
}

impl Default for DevCharTable6Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable6Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable6Loc4Val {
        DevCharTable6Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE7_LOC1 Register
///
/// NA
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **DCT_DEV7_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable7Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable7Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable7Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV7_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV7_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV7_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV7_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev7_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV7_LOC1_MASK) >> Self::DCT_DEV7_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE7_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable7loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable7loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable7Loc1Val {
    bits: u32,
}

impl DevCharTable7Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV7_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev7_loc1(&self) -> u32 {
        (self.bits & DevCharTable7Loc1::DCT_DEV7_LOC1_MASK)
            >> DevCharTable7Loc1::DCT_DEV7_LOC1_OFFSET
    }
}

impl Default for DevCharTable7Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable7Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable7Loc1Val {
        DevCharTable7Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE7_LOC2 Register
///
/// NA
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **DCT_DEV7_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable7Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable7Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable7Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV7_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV7_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV7_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV7_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev7_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV7_LOC2_MASK) >> Self::DCT_DEV7_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE7_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable7loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable7loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable7Loc2Val {
    bits: u32,
}

impl DevCharTable7Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV7_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev7_loc2(&self) -> u32 {
        (self.bits & DevCharTable7Loc2::DCT_DEV7_LOC2_MASK)
            >> DevCharTable7Loc2::DCT_DEV7_LOC2_OFFSET
    }
}

impl Default for DevCharTable7Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable7Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable7Loc2Val {
        DevCharTable7Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE7_LOC3 Register
///
/// NA
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **DCT_DEV7_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable7Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable7Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable7Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV7_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV7_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV7_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV7_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev7_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV7_LOC3_MASK) >> Self::DCT_DEV7_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE7_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable7loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable7loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable7Loc3Val {
    bits: u32,
}

impl DevCharTable7Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV7_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev7_loc3(&self) -> u32 {
        (self.bits & DevCharTable7Loc3::DCT_DEV7_LOC3_MASK)
            >> DevCharTable7Loc3::DCT_DEV7_LOC3_OFFSET
    }
}

impl Default for DevCharTable7Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable7Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable7Loc3Val {
        DevCharTable7Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE7_LOC4 Register
///
/// NA
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **DCT_DEV7_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable7Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable7Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable7Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV7_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV7_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV7_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV7_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev7_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV7_LOC4_MASK) >> Self::DCT_DEV7_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE7_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable7loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable7loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable7Loc4Val {
    bits: u32,
}

impl DevCharTable7Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV7_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev7_loc4(&self) -> u32 {
        (self.bits & DevCharTable7Loc4::DCT_DEV7_LOC4_MASK)
            >> DevCharTable7Loc4::DCT_DEV7_LOC4_OFFSET
    }
}

impl Default for DevCharTable7Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable7Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable7Loc4Val {
        DevCharTable7Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE8_LOC1 Register
///
/// NA
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **DCT_DEV8_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable8Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable8Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable8Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV8_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV8_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV8_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV8_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev8_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV8_LOC1_MASK) >> Self::DCT_DEV8_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE8_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable8loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable8loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable8Loc1Val {
    bits: u32,
}

impl DevCharTable8Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV8_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev8_loc1(&self) -> u32 {
        (self.bits & DevCharTable8Loc1::DCT_DEV8_LOC1_MASK)
            >> DevCharTable8Loc1::DCT_DEV8_LOC1_OFFSET
    }
}

impl Default for DevCharTable8Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable8Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable8Loc1Val {
        DevCharTable8Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE8_LOC2 Register
///
/// NA
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **DCT_DEV8_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable8Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable8Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable8Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV8_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV8_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV8_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV8_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev8_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV8_LOC2_MASK) >> Self::DCT_DEV8_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE8_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable8loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable8loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable8Loc2Val {
    bits: u32,
}

impl DevCharTable8Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV8_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev8_loc2(&self) -> u32 {
        (self.bits & DevCharTable8Loc2::DCT_DEV8_LOC2_MASK)
            >> DevCharTable8Loc2::DCT_DEV8_LOC2_OFFSET
    }
}

impl Default for DevCharTable8Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable8Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable8Loc2Val {
        DevCharTable8Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE8_LOC3 Register
///
/// NA
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **DCT_DEV8_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable8Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable8Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable8Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV8_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV8_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV8_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV8_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev8_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV8_LOC3_MASK) >> Self::DCT_DEV8_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE8_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable8loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable8loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable8Loc3Val {
    bits: u32,
}

impl DevCharTable8Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV8_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev8_loc3(&self) -> u32 {
        (self.bits & DevCharTable8Loc3::DCT_DEV8_LOC3_MASK)
            >> DevCharTable8Loc3::DCT_DEV8_LOC3_OFFSET
    }
}

impl Default for DevCharTable8Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable8Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable8Loc3Val {
        DevCharTable8Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE8_LOC4 Register
///
/// NA
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **DCT_DEV8_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable8Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable8Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable8Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV8_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV8_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV8_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV8_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev8_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV8_LOC4_MASK) >> Self::DCT_DEV8_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE8_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable8loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable8loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable8Loc4Val {
    bits: u32,
}

impl DevCharTable8Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV8_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev8_loc4(&self) -> u32 {
        (self.bits & DevCharTable8Loc4::DCT_DEV8_LOC4_MASK)
            >> DevCharTable8Loc4::DCT_DEV8_LOC4_OFFSET
    }
}

impl Default for DevCharTable8Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable8Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable8Loc4Val {
        DevCharTable8Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE9_LOC1 Register
///
/// NA
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **DCT_DEV9_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable9Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable9Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable9Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV9_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV9_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV9_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV9_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev9_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV9_LOC1_MASK) >> Self::DCT_DEV9_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE9_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable9loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable9loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable9Loc1Val {
    bits: u32,
}

impl DevCharTable9Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV9_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev9_loc1(&self) -> u32 {
        (self.bits & DevCharTable9Loc1::DCT_DEV9_LOC1_MASK)
            >> DevCharTable9Loc1::DCT_DEV9_LOC1_OFFSET
    }
}

impl Default for DevCharTable9Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable9Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable9Loc1Val {
        DevCharTable9Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE9_LOC2 Register
///
/// NA
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **DCT_DEV9_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable9Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable9Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable9Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV9_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV9_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV9_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV9_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev9_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV9_LOC2_MASK) >> Self::DCT_DEV9_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE9_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable9loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable9loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable9Loc2Val {
    bits: u32,
}

impl DevCharTable9Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV9_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev9_loc2(&self) -> u32 {
        (self.bits & DevCharTable9Loc2::DCT_DEV9_LOC2_MASK)
            >> DevCharTable9Loc2::DCT_DEV9_LOC2_OFFSET
    }
}

impl Default for DevCharTable9Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable9Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable9Loc2Val {
        DevCharTable9Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE9_LOC3 Register
///
/// NA
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **DCT_DEV9_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable9Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable9Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable9Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV9_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV9_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV9_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV9_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev9_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV9_LOC3_MASK) >> Self::DCT_DEV9_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE9_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable9loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable9loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable9Loc3Val {
    bits: u32,
}

impl DevCharTable9Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV9_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev9_loc3(&self) -> u32 {
        (self.bits & DevCharTable9Loc3::DCT_DEV9_LOC3_MASK)
            >> DevCharTable9Loc3::DCT_DEV9_LOC3_OFFSET
    }
}

impl Default for DevCharTable9Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable9Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable9Loc3Val {
        DevCharTable9Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE9_LOC4 Register
///
/// NA
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **DCT_DEV9_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable9Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable9Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable9Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV9_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV9_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV9_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV9_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev9_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV9_LOC4_MASK) >> Self::DCT_DEV9_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE9_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable9loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable9loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable9Loc4Val {
    bits: u32,
}

impl DevCharTable9Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV9_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev9_loc4(&self) -> u32 {
        (self.bits & DevCharTable9Loc4::DCT_DEV9_LOC4_MASK)
            >> DevCharTable9Loc4::DCT_DEV9_LOC4_OFFSET
    }
}

impl Default for DevCharTable9Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable9Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable9Loc4Val {
        DevCharTable9Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE10_LOC1 Register
///
/// NA
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **DCT_DEV10_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable10Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable10Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable10Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV10_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV10_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV10_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV10_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev10_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV10_LOC1_MASK) >> Self::DCT_DEV10_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE10_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable10loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable10loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable10Loc1Val {
    bits: u32,
}

impl DevCharTable10Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV10_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev10_loc1(&self) -> u32 {
        (self.bits & DevCharTable10Loc1::DCT_DEV10_LOC1_MASK)
            >> DevCharTable10Loc1::DCT_DEV10_LOC1_OFFSET
    }
}

impl Default for DevCharTable10Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable10Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable10Loc1Val {
        DevCharTable10Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE10_LOC2 Register
///
/// NA
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **DCT_DEV10_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable10Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable10Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable10Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV10_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV10_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV10_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV10_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev10_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV10_LOC2_MASK) >> Self::DCT_DEV10_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE10_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable10loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable10loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable10Loc2Val {
    bits: u32,
}

impl DevCharTable10Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV10_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev10_loc2(&self) -> u32 {
        (self.bits & DevCharTable10Loc2::DCT_DEV10_LOC2_MASK)
            >> DevCharTable10Loc2::DCT_DEV10_LOC2_OFFSET
    }
}

impl Default for DevCharTable10Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable10Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable10Loc2Val {
        DevCharTable10Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE10_LOC3 Register
///
/// NA
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **DCT_DEV10_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable10Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable10Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable10Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV10_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV10_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV10_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV10_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev10_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV10_LOC3_MASK) >> Self::DCT_DEV10_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE10_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable10loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable10loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable10Loc3Val {
    bits: u32,
}

impl DevCharTable10Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV10_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev10_loc3(&self) -> u32 {
        (self.bits & DevCharTable10Loc3::DCT_DEV10_LOC3_MASK)
            >> DevCharTable10Loc3::DCT_DEV10_LOC3_OFFSET
    }
}

impl Default for DevCharTable10Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable10Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable10Loc3Val {
        DevCharTable10Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE10_LOC4 Register
///
/// NA
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **DCT_DEV10_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable10Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable10Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable10Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV10_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV10_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV10_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV10_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev10_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV10_LOC4_MASK) >> Self::DCT_DEV10_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE10_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable10loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable10loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable10Loc4Val {
    bits: u32,
}

impl DevCharTable10Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV10_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev10_loc4(&self) -> u32 {
        (self.bits & DevCharTable10Loc4::DCT_DEV10_LOC4_MASK)
            >> DevCharTable10Loc4::DCT_DEV10_LOC4_OFFSET
    }
}

impl Default for DevCharTable10Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable10Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable10Loc4Val {
        DevCharTable10Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE11_LOC1 Register
///
/// NA
///
/// Address offset: 0x01a0
///
/// ## Fields
///
/// - **DCT_DEV11_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable11Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable11Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable11Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV11_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV11_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV11_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV11_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev11_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV11_LOC1_MASK) >> Self::DCT_DEV11_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE11_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable11loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable11loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable11Loc1Val {
    bits: u32,
}

impl DevCharTable11Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV11_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev11_loc1(&self) -> u32 {
        (self.bits & DevCharTable11Loc1::DCT_DEV11_LOC1_MASK)
            >> DevCharTable11Loc1::DCT_DEV11_LOC1_OFFSET
    }
}

impl Default for DevCharTable11Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable11Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable11Loc1Val {
        DevCharTable11Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE11_LOC2 Register
///
/// NA
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **DCT_DEV11_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable11Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable11Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable11Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV11_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV11_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV11_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV11_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev11_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV11_LOC2_MASK) >> Self::DCT_DEV11_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE11_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable11loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable11loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable11Loc2Val {
    bits: u32,
}

impl DevCharTable11Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV11_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev11_loc2(&self) -> u32 {
        (self.bits & DevCharTable11Loc2::DCT_DEV11_LOC2_MASK)
            >> DevCharTable11Loc2::DCT_DEV11_LOC2_OFFSET
    }
}

impl Default for DevCharTable11Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable11Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable11Loc2Val {
        DevCharTable11Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE11_LOC3 Register
///
/// NA
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **DCT_DEV11_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable11Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable11Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable11Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV11_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV11_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV11_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV11_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev11_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV11_LOC3_MASK) >> Self::DCT_DEV11_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE11_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable11loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable11loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable11Loc3Val {
    bits: u32,
}

impl DevCharTable11Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV11_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev11_loc3(&self) -> u32 {
        (self.bits & DevCharTable11Loc3::DCT_DEV11_LOC3_MASK)
            >> DevCharTable11Loc3::DCT_DEV11_LOC3_OFFSET
    }
}

impl Default for DevCharTable11Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable11Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable11Loc3Val {
        DevCharTable11Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE11_LOC4 Register
///
/// NA
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **DCT_DEV11_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable11Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable11Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable11Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV11_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV11_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV11_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV11_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev11_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV11_LOC4_MASK) >> Self::DCT_DEV11_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE11_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable11loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable11loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable11Loc4Val {
    bits: u32,
}

impl DevCharTable11Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV11_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev11_loc4(&self) -> u32 {
        (self.bits & DevCharTable11Loc4::DCT_DEV11_LOC4_MASK)
            >> DevCharTable11Loc4::DCT_DEV11_LOC4_OFFSET
    }
}

impl Default for DevCharTable11Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable11Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable11Loc4Val {
        DevCharTable11Loc4Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE12_LOC1 Register
///
/// NA
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **DCT_DEV12_LOC1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable12Loc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable12Loc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable12Loc1 {
    // ======== Field Constants ========

    /// **DCT_DEV12_LOC1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV12_LOC1_OFFSET: u32 = 0;
    pub const DCT_DEV12_LOC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV12_LOC1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev12_loc1(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV12_LOC1_MASK) >> Self::DCT_DEV12_LOC1_OFFSET
    }
}

/// DEV_CHAR_TABLE12_LOC1 value type for batch read/modify/write operations
///
/// Use `devchartable12loc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable12loc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable12Loc1Val {
    bits: u32,
}

impl DevCharTable12Loc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV12_LOC1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev12_loc1(&self) -> u32 {
        (self.bits & DevCharTable12Loc1::DCT_DEV12_LOC1_MASK)
            >> DevCharTable12Loc1::DCT_DEV12_LOC1_OFFSET
    }
}

impl Default for DevCharTable12Loc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable12Loc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable12Loc1Val {
        DevCharTable12Loc1Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE12_LOC2 Register
///
/// NA
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **DCT_DEV12_LOC2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable12Loc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable12Loc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable12Loc2 {
    // ======== Field Constants ========

    /// **DCT_DEV12_LOC2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV12_LOC2_OFFSET: u32 = 0;
    pub const DCT_DEV12_LOC2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV12_LOC2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev12_loc2(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV12_LOC2_MASK) >> Self::DCT_DEV12_LOC2_OFFSET
    }
}

/// DEV_CHAR_TABLE12_LOC2 value type for batch read/modify/write operations
///
/// Use `devchartable12loc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable12loc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable12Loc2Val {
    bits: u32,
}

impl DevCharTable12Loc2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV12_LOC2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev12_loc2(&self) -> u32 {
        (self.bits & DevCharTable12Loc2::DCT_DEV12_LOC2_MASK)
            >> DevCharTable12Loc2::DCT_DEV12_LOC2_OFFSET
    }
}

impl Default for DevCharTable12Loc2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable12Loc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable12Loc2Val {
        DevCharTable12Loc2Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE12_LOC3 Register
///
/// NA
///
/// Address offset: 0x01b8
///
/// ## Fields
///
/// - **DCT_DEV12_LOC3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable12Loc3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable12Loc3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable12Loc3 {
    // ======== Field Constants ========

    /// **DCT_DEV12_LOC3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV12_LOC3_OFFSET: u32 = 0;
    pub const DCT_DEV12_LOC3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV12_LOC3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev12_loc3(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV12_LOC3_MASK) >> Self::DCT_DEV12_LOC3_OFFSET
    }
}

/// DEV_CHAR_TABLE12_LOC3 value type for batch read/modify/write operations
///
/// Use `devchartable12loc3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable12loc3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable12Loc3Val {
    bits: u32,
}

impl DevCharTable12Loc3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV12_LOC3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev12_loc3(&self) -> u32 {
        (self.bits & DevCharTable12Loc3::DCT_DEV12_LOC3_MASK)
            >> DevCharTable12Loc3::DCT_DEV12_LOC3_OFFSET
    }
}

impl Default for DevCharTable12Loc3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable12Loc3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable12Loc3Val {
        DevCharTable12Loc3Val::from_bits(self.value.read_bits())
    }
}

/// DEV_CHAR_TABLE12_LOC4 Register
///
/// NA
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **DCT_DEV12_LOC4** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DevCharTable12Loc4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevCharTable12Loc4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevCharTable12Loc4 {
    // ======== Field Constants ========

    /// **DCT_DEV12_LOC4**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DCT_DEV12_LOC4_OFFSET: u32 = 0;
    pub const DCT_DEV12_LOC4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DCT_DEV12_LOC4** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dct_dev12_loc4(&self) -> u32 {
        (self.value.read_bits() & Self::DCT_DEV12_LOC4_MASK) >> Self::DCT_DEV12_LOC4_OFFSET
    }
}

/// DEV_CHAR_TABLE12_LOC4 value type for batch read/modify/write operations
///
/// Use `devchartable12loc4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devchartable12loc4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevCharTable12Loc4Val {
    bits: u32,
}

impl DevCharTable12Loc4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCT_DEV12_LOC4** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dct_dev12_loc4(&self) -> u32 {
        (self.bits & DevCharTable12Loc4::DCT_DEV12_LOC4_MASK)
            >> DevCharTable12Loc4::DCT_DEV12_LOC4_OFFSET
    }
}

impl Default for DevCharTable12Loc4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DevCharTable12Loc4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevCharTable12Loc4Val {
        DevCharTable12Loc4Val::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8],
    /// 0x0008
    /// NA
    pub command_buf_port: CommandBufPort,
    /// 0x000c
    /// NA
    pub response_buf_port: ResponseBufPort,
    /// 0x0010
    /// NA
    pub rx_data_port: RxDataPort,
    /// 0x0014
    /// NA
    pub tx_data_port: TxDataPort,
    /// 0x0018
    /// In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)
    pub ibi_status_buf: IbiStatusBuf,
    _reserved1: [u8; 36],
    /// 0x0040
    /// NA
    pub ibi_data_buf: IbiDataBuf,
    _reserved2: [u8; 124],
    /// 0x00c0
    /// NA
    pub dev_addr_table1_loc: DevAddrTable1Loc,
    /// 0x00c4
    /// NA
    pub dev_addr_table2_loc: DevAddrTable2Loc,
    /// 0x00c8
    /// NA
    pub dev_addr_table3_loc: DevAddrTable3Loc,
    /// 0x00cc
    /// NA
    pub dev_addr_table4_loc: DevAddrTable4Loc,
    /// 0x00d0
    /// NA
    pub dev_addr_table5_loc: DevAddrTable5Loc,
    /// 0x00d4
    /// NA
    pub dev_addr_table6_loc: DevAddrTable6Loc,
    /// 0x00d8
    /// NA
    pub dev_addr_table7_loc: DevAddrTable7Loc,
    /// 0x00dc
    /// NA
    pub dev_addr_table8_loc: DevAddrTable8Loc,
    /// 0x00e0
    /// NA
    pub dev_addr_table9_loc: DevAddrTable9Loc,
    /// 0x00e4
    /// NA
    pub dev_addr_table10_loc: DevAddrTable10Loc,
    /// 0x00e8
    /// NA
    pub dev_addr_table11_loc: DevAddrTable11Loc,
    /// 0x00ec
    /// NA
    pub dev_addr_table12_loc: DevAddrTable12Loc,
    _reserved3: [u8; 16],
    /// 0x0100
    /// NA
    pub dev_char_table1_loc1: DevCharTable1Loc1,
    /// 0x0104
    /// NA
    pub dev_char_table1_loc2: DevCharTable1Loc2,
    /// 0x0108
    /// NA
    pub dev_char_table1_loc3: DevCharTable1Loc3,
    /// 0x010c
    /// NA
    pub dev_char_table1_loc4: DevCharTable1Loc4,
    /// 0x0110
    /// NA
    pub dev_char_table2_loc1: DevCharTable2Loc1,
    /// 0x0114
    /// NA
    pub dev_char_table2_loc2: DevCharTable2Loc2,
    /// 0x0118
    /// NA
    pub dev_char_table2_loc3: DevCharTable2Loc3,
    /// 0x011c
    /// NA
    pub dev_char_table2_loc4: DevCharTable2Loc4,
    /// 0x0120
    /// NA
    pub dev_char_table3_loc1: DevCharTable3Loc1,
    /// 0x0124
    /// NA
    pub dev_char_table3_loc2: DevCharTable3Loc2,
    /// 0x0128
    /// NA
    pub dev_char_table3_loc3: DevCharTable3Loc3,
    /// 0x012c
    /// NA
    pub dev_char_table3_loc4: DevCharTable3Loc4,
    /// 0x0130
    /// NA
    pub dev_char_table4_loc1: DevCharTable4Loc1,
    /// 0x0134
    /// NA
    pub dev_char_table4_loc2: DevCharTable4Loc2,
    /// 0x0138
    /// NA
    pub dev_char_table4_loc3: DevCharTable4Loc3,
    /// 0x013c
    /// NA
    pub dev_char_table4_loc4: DevCharTable4Loc4,
    /// 0x0140
    /// NA
    pub dev_char_table5_loc1: DevCharTable5Loc1,
    /// 0x0144
    /// NA
    pub dev_char_table5_loc2: DevCharTable5Loc2,
    /// 0x0148
    /// NA
    pub dev_char_table5_loc3: DevCharTable5Loc3,
    /// 0x014c
    /// NA
    pub dev_char_table5_loc4: DevCharTable5Loc4,
    /// 0x0150
    /// NA
    pub dev_char_table6_loc1: DevCharTable6Loc1,
    /// 0x0154
    /// NA
    pub dev_char_table6_loc2: DevCharTable6Loc2,
    /// 0x0158
    /// NA
    pub dev_char_table6_loc3: DevCharTable6Loc3,
    /// 0x015c
    /// NA
    pub dev_char_table6_loc4: DevCharTable6Loc4,
    /// 0x0160
    /// NA
    pub dev_char_table7_loc1: DevCharTable7Loc1,
    /// 0x0164
    /// NA
    pub dev_char_table7_loc2: DevCharTable7Loc2,
    /// 0x0168
    /// NA
    pub dev_char_table7_loc3: DevCharTable7Loc3,
    /// 0x016c
    /// NA
    pub dev_char_table7_loc4: DevCharTable7Loc4,
    /// 0x0170
    /// NA
    pub dev_char_table8_loc1: DevCharTable8Loc1,
    /// 0x0174
    /// NA
    pub dev_char_table8_loc2: DevCharTable8Loc2,
    /// 0x0178
    /// NA
    pub dev_char_table8_loc3: DevCharTable8Loc3,
    /// 0x017c
    /// NA
    pub dev_char_table8_loc4: DevCharTable8Loc4,
    /// 0x0180
    /// NA
    pub dev_char_table9_loc1: DevCharTable9Loc1,
    /// 0x0184
    /// NA
    pub dev_char_table9_loc2: DevCharTable9Loc2,
    /// 0x0188
    /// NA
    pub dev_char_table9_loc3: DevCharTable9Loc3,
    /// 0x018c
    /// NA
    pub dev_char_table9_loc4: DevCharTable9Loc4,
    /// 0x0190
    /// NA
    pub dev_char_table10_loc1: DevCharTable10Loc1,
    /// 0x0194
    /// NA
    pub dev_char_table10_loc2: DevCharTable10Loc2,
    /// 0x0198
    /// NA
    pub dev_char_table10_loc3: DevCharTable10Loc3,
    /// 0x019c
    /// NA
    pub dev_char_table10_loc4: DevCharTable10Loc4,
    /// 0x01a0
    /// NA
    pub dev_char_table11_loc1: DevCharTable11Loc1,
    /// 0x01a4
    /// NA
    pub dev_char_table11_loc2: DevCharTable11Loc2,
    /// 0x01a8
    /// NA
    pub dev_char_table11_loc3: DevCharTable11Loc3,
    /// 0x01ac
    /// NA
    pub dev_char_table11_loc4: DevCharTable11Loc4,
    /// 0x01b0
    /// NA
    pub dev_char_table12_loc1: DevCharTable12Loc1,
    /// 0x01b4
    /// NA
    pub dev_char_table12_loc2: DevCharTable12Loc2,
    /// 0x01b8
    /// NA
    pub dev_char_table12_loc3: DevCharTable12Loc3,
    /// 0x01bc
    /// NA
    pub dev_char_table12_loc4: DevCharTable12Loc4,
}
