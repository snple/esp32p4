//! TWAI0
//! Two-Wire Automotive Interface

#![allow(dead_code)]

pub const BASE: u32 = 0x500d7000;

/// MODE Register
///
/// TWAI mode register.
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **ACCEPTANCE_FILTER_MODE** `[3]` (RW)
///
///   1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active).
///
/// - **SELF_TEST_MODE** `[2]` (RW)
///
///   1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission.
///
/// - **LISTEN_ONLY_MODE** `[1]` (RW)
///
///   1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal.
///
/// - **RESET_MODE** `[0]` (RW)
///
///   1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode.
#[repr(transparent)]
pub struct Mode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mode {
    // ======== Field Constants ========

    /// **ACCEPTANCE_FILTER_MODE**
    ///
    /// 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active).
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ACCEPTANCE_FILTER_MODE_OFFSET: u32 = 3;
    pub const ACCEPTANCE_FILTER_MODE_MASK: u32 = 0x00000008;
    pub const ACCEPTANCE_FILTER_MODE: u32 = 1 << 3;

    /// **SELF_TEST_MODE**
    ///
    /// 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SELF_TEST_MODE_OFFSET: u32 = 2;
    pub const SELF_TEST_MODE_MASK: u32 = 0x00000004;
    pub const SELF_TEST_MODE: u32 = 1 << 2;

    /// **LISTEN_ONLY_MODE**
    ///
    /// 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LISTEN_ONLY_MODE_OFFSET: u32 = 1;
    pub const LISTEN_ONLY_MODE_MASK: u32 = 0x00000002;
    pub const LISTEN_ONLY_MODE: u32 = 1 << 1;

    /// **RESET_MODE**
    ///
    /// 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RESET_MODE_OFFSET: u32 = 0;
    pub const RESET_MODE_MASK: u32 = 0x00000001;
    pub const RESET_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ACCEPTANCE_FILTER_MODE** `[3]` (RW)
    ///
    /// 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active).
    #[inline(always)]
    pub fn acceptance_filter_mode(&self) -> bool {
        (self.value.read_bits() & Self::ACCEPTANCE_FILTER_MODE_MASK) != 0
    }

    /// Set ACCEPTANCE_FILTER_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_acceptance_filter_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::ACCEPTANCE_FILTER_MODE_MASK);
    }

    /// Clear ACCEPTANCE_FILTER_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_acceptance_filter_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::ACCEPTANCE_FILTER_MODE_MASK);
    }

    /// **SELF_TEST_MODE** `[2]` (RW)
    ///
    /// 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission.
    #[inline(always)]
    pub fn self_test_mode(&self) -> bool {
        (self.value.read_bits() & Self::SELF_TEST_MODE_MASK) != 0
    }

    /// Set SELF_TEST_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_self_test_mode(&self) {
        self.value.modify_bits(|r| r | Self::SELF_TEST_MODE_MASK);
    }

    /// Clear SELF_TEST_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_self_test_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SELF_TEST_MODE_MASK);
    }

    /// **LISTEN_ONLY_MODE** `[1]` (RW)
    ///
    /// 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal.
    #[inline(always)]
    pub fn listen_only_mode(&self) -> bool {
        (self.value.read_bits() & Self::LISTEN_ONLY_MODE_MASK) != 0
    }

    /// Set LISTEN_ONLY_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_listen_only_mode(&self) {
        self.value.modify_bits(|r| r | Self::LISTEN_ONLY_MODE_MASK);
    }

    /// Clear LISTEN_ONLY_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_listen_only_mode(&self) {
        self.value.modify_bits(|r| r & !Self::LISTEN_ONLY_MODE_MASK);
    }

    /// **RESET_MODE** `[0]` (RW)
    ///
    /// 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode.
    #[inline(always)]
    pub fn reset_mode(&self) -> bool {
        (self.value.read_bits() & Self::RESET_MODE_MASK) != 0
    }

    /// Set RESET_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reset_mode(&self) {
        self.value.modify_bits(|r| r | Self::RESET_MODE_MASK);
    }

    /// Clear RESET_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reset_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RESET_MODE_MASK);
    }
}

/// MODE value type for batch read/modify/write operations
///
/// Use `mode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ModeVal {
    bits: u32,
}

impl ModeVal {
    /// Create from raw bits
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

    /// Read **ACCEPTANCE_FILTER_MODE** `[3:3]`
    ///
    /// 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active).
    #[inline(always)]
    pub const fn acceptance_filter_mode(&self) -> bool {
        (self.bits & Mode::ACCEPTANCE_FILTER_MODE_MASK) != 0
    }

    /// Read **SELF_TEST_MODE** `[2:2]`
    ///
    /// 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission.
    #[inline(always)]
    pub const fn self_test_mode(&self) -> bool {
        (self.bits & Mode::SELF_TEST_MODE_MASK) != 0
    }

    /// Read **LISTEN_ONLY_MODE** `[1:1]`
    ///
    /// 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal.
    #[inline(always)]
    pub const fn listen_only_mode(&self) -> bool {
        (self.bits & Mode::LISTEN_ONLY_MODE_MASK) != 0
    }

    /// Read **RESET_MODE** `[0:0]`
    ///
    /// 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode.
    #[inline(always)]
    pub const fn reset_mode(&self) -> bool {
        (self.bits & Mode::RESET_MODE_MASK) != 0
    }

    /// Set **ACCEPTANCE_FILTER_MODE** `[3:3]`
    ///
    /// 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active).
    #[inline(always)]
    pub const fn with_acceptance_filter_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mode::ACCEPTANCE_FILTER_MODE_MASK
            } else {
                self.bits & !Mode::ACCEPTANCE_FILTER_MODE_MASK
            },
        }
    }

    /// Set **SELF_TEST_MODE** `[2:2]`
    ///
    /// 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission.
    #[inline(always)]
    pub const fn with_self_test_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mode::SELF_TEST_MODE_MASK
            } else {
                self.bits & !Mode::SELF_TEST_MODE_MASK
            },
        }
    }

    /// Set **LISTEN_ONLY_MODE** `[1:1]`
    ///
    /// 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal.
    #[inline(always)]
    pub const fn with_listen_only_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mode::LISTEN_ONLY_MODE_MASK
            } else {
                self.bits & !Mode::LISTEN_ONLY_MODE_MASK
            },
        }
    }

    /// Set **RESET_MODE** `[0:0]`
    ///
    /// 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode.
    #[inline(always)]
    pub const fn with_reset_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mode::RESET_MODE_MASK
            } else {
                self.bits & !Mode::RESET_MODE_MASK
            },
        }
    }
}

impl Default for ModeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ModeVal {
        ModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ModeVal) -> ModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMD Register
///
/// TWAI command register.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **SELF_RX_REQUEST** `[4]` (WO)
///
///   1: present, a message shall be transmitted and received simultaneously. 0: absent.
///
/// - **CLEAR_DATA_OVERRUN** `[3]` (WO)
///
///   1: clear, the data overrun status bit is cleared. 0: no action.
///
/// - **RELEASE_BUFFER** `[2]` (WO)
///
///   1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
///
/// - **ABORT_TX** `[1]` (WO)
///
///   1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
///
/// - **TX_REQUEST** `[0]` (WO)
///
///   1: present, a message shall be transmitted. 0: absent
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

    /// **SELF_RX_REQUEST**
    ///
    /// 1: present, a message shall be transmitted and received simultaneously. 0: absent.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SELF_RX_REQUEST_OFFSET: u32 = 4;
    pub const SELF_RX_REQUEST_MASK: u32 = 0x00000010;
    pub const SELF_RX_REQUEST: u32 = 1 << 4;

    /// **CLEAR_DATA_OVERRUN**
    ///
    /// 1: clear, the data overrun status bit is cleared. 0: no action.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLEAR_DATA_OVERRUN_OFFSET: u32 = 3;
    pub const CLEAR_DATA_OVERRUN_MASK: u32 = 0x00000008;
    pub const CLEAR_DATA_OVERRUN: u32 = 1 << 3;

    /// **RELEASE_BUFFER**
    ///
    /// 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RELEASE_BUFFER_OFFSET: u32 = 2;
    pub const RELEASE_BUFFER_MASK: u32 = 0x00000004;
    pub const RELEASE_BUFFER: u32 = 1 << 2;

    /// **ABORT_TX**
    ///
    /// 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ABORT_TX_OFFSET: u32 = 1;
    pub const ABORT_TX_MASK: u32 = 0x00000002;
    pub const ABORT_TX: u32 = 1 << 1;

    /// **TX_REQUEST**
    ///
    /// 1: present, a message shall be transmitted. 0: absent
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_REQUEST_OFFSET: u32 = 0;
    pub const TX_REQUEST_MASK: u32 = 0x00000001;
    pub const TX_REQUEST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SELF_RX_REQUEST** (WO)
    ///
    /// 1: present, a message shall be transmitted and received simultaneously. 0: absent.
    #[inline(always)]
    pub fn trigger_self_rx_request(&self) {
        self.value.write_bits(Self::SELF_RX_REQUEST_MASK);
    }

    /// Trigger **CLEAR_DATA_OVERRUN** (WO)
    ///
    /// 1: clear, the data overrun status bit is cleared. 0: no action.
    #[inline(always)]
    pub fn trigger_clear_data_overrun(&self) {
        self.value.write_bits(Self::CLEAR_DATA_OVERRUN_MASK);
    }

    /// Trigger **RELEASE_BUFFER** (WO)
    ///
    /// 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
    #[inline(always)]
    pub fn trigger_release_buffer(&self) {
        self.value.write_bits(Self::RELEASE_BUFFER_MASK);
    }

    /// Trigger **ABORT_TX** (WO)
    ///
    /// 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
    #[inline(always)]
    pub fn trigger_abort_tx(&self) {
        self.value.write_bits(Self::ABORT_TX_MASK);
    }

    /// Trigger **TX_REQUEST** (WO)
    ///
    /// 1: present, a message shall be transmitted. 0: absent
    #[inline(always)]
    pub fn trigger_tx_request(&self) {
        self.value.write_bits(Self::TX_REQUEST_MASK);
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

    /// Set **SELF_RX_REQUEST** `[4:4]`
    ///
    /// 1: present, a message shall be transmitted and received simultaneously. 0: absent.
    #[inline(always)]
    pub const fn with_self_rx_request(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::SELF_RX_REQUEST_MASK
            } else {
                self.bits & !Cmd::SELF_RX_REQUEST_MASK
            },
        }
    }

    /// Set **CLEAR_DATA_OVERRUN** `[3:3]`
    ///
    /// 1: clear, the data overrun status bit is cleared. 0: no action.
    #[inline(always)]
    pub const fn with_clear_data_overrun(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::CLEAR_DATA_OVERRUN_MASK
            } else {
                self.bits & !Cmd::CLEAR_DATA_OVERRUN_MASK
            },
        }
    }

    /// Set **RELEASE_BUFFER** `[2:2]`
    ///
    /// 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
    #[inline(always)]
    pub const fn with_release_buffer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::RELEASE_BUFFER_MASK
            } else {
                self.bits & !Cmd::RELEASE_BUFFER_MASK
            },
        }
    }

    /// Set **ABORT_TX** `[1:1]`
    ///
    /// 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
    #[inline(always)]
    pub const fn with_abort_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::ABORT_TX_MASK
            } else {
                self.bits & !Cmd::ABORT_TX_MASK
            },
        }
    }

    /// Set **TX_REQUEST** `[0:0]`
    ///
    /// 1: present, a message shall be transmitted. 0: absent
    #[inline(always)]
    pub const fn with_tx_request(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::TX_REQUEST_MASK
            } else {
                self.bits & !Cmd::TX_REQUEST_MASK
            },
        }
    }
}

impl Default for CmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cmd {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATUS Register
///
/// TWAI status register.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **MISS** `[8]` (RO)
///
///   1: current message is destroyed because of FIFO overflow.
///
/// - **NODE_BUS_OFF** `[7]` (RO)
///
///   1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities
///
/// - **ERR** `[6]` (RO)
///
///   1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit
///
/// - **TRANSMIT** `[5]` (RO)
///
///   1: transmit, the TWAI controller is transmitting a message. 0: idle
///
/// - **RECEIVE** `[4]` (RO)
///
///   1: receive, the TWAI controller is receiving a message. 0: idle
///
/// - **TRANSMISSION_COMPLETE** `[3]` (RO)
///
///   1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed
///
/// - **TRANSMIT_BUFFER** `[2]` (RO)
///
///   1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted
///
/// - **OVERRUN** `[1]` (RO)
///
///   1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given
///
/// - **RECEIVE_BUFFER** `[0]` (RO)
///
///   1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available
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

    /// **MISS**
    ///
    /// 1: current message is destroyed because of FIFO overflow.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MISS_OFFSET: u32 = 8;
    pub const MISS_MASK: u32 = 0x00000100;
    pub const MISS: u32 = 1 << 8;

    /// **NODE_BUS_OFF**
    ///
    /// 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const NODE_BUS_OFF_OFFSET: u32 = 7;
    pub const NODE_BUS_OFF_MASK: u32 = 0x00000080;
    pub const NODE_BUS_OFF: u32 = 1 << 7;

    /// **ERR**
    ///
    /// 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ERR_OFFSET: u32 = 6;
    pub const ERR_MASK: u32 = 0x00000040;
    pub const ERR: u32 = 1 << 6;

    /// **TRANSMIT**
    ///
    /// 1: transmit, the TWAI controller is transmitting a message. 0: idle
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TRANSMIT_OFFSET: u32 = 5;
    pub const TRANSMIT_MASK: u32 = 0x00000020;
    pub const TRANSMIT: u32 = 1 << 5;

    /// **RECEIVE**
    ///
    /// 1: receive, the TWAI controller is receiving a message. 0: idle
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RECEIVE_OFFSET: u32 = 4;
    pub const RECEIVE_MASK: u32 = 0x00000010;
    pub const RECEIVE: u32 = 1 << 4;

    /// **TRANSMISSION_COMPLETE**
    ///
    /// 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANSMISSION_COMPLETE_OFFSET: u32 = 3;
    pub const TRANSMISSION_COMPLETE_MASK: u32 = 0x00000008;
    pub const TRANSMISSION_COMPLETE: u32 = 1 << 3;

    /// **TRANSMIT_BUFFER**
    ///
    /// 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TRANSMIT_BUFFER_OFFSET: u32 = 2;
    pub const TRANSMIT_BUFFER_MASK: u32 = 0x00000004;
    pub const TRANSMIT_BUFFER: u32 = 1 << 2;

    /// **OVERRUN**
    ///
    /// 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OVERRUN_OFFSET: u32 = 1;
    pub const OVERRUN_MASK: u32 = 0x00000002;
    pub const OVERRUN: u32 = 1 << 1;

    /// **RECEIVE_BUFFER**
    ///
    /// 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RECEIVE_BUFFER_OFFSET: u32 = 0;
    pub const RECEIVE_BUFFER_MASK: u32 = 0x00000001;
    pub const RECEIVE_BUFFER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MISS** `[8]` (RO)
    ///
    /// 1: current message is destroyed because of FIFO overflow.
    #[inline(always)]
    pub fn miss(&self) -> bool {
        (self.value.read_bits() & Self::MISS_MASK) != 0
    }

    /// **NODE_BUS_OFF** `[7]` (RO)
    ///
    /// 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities
    #[inline(always)]
    pub fn node_bus_off(&self) -> bool {
        (self.value.read_bits() & Self::NODE_BUS_OFF_MASK) != 0
    }

    /// **ERR** `[6]` (RO)
    ///
    /// 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit
    #[inline(always)]
    pub fn err(&self) -> bool {
        (self.value.read_bits() & Self::ERR_MASK) != 0
    }

    /// **TRANSMIT** `[5]` (RO)
    ///
    /// 1: transmit, the TWAI controller is transmitting a message. 0: idle
    #[inline(always)]
    pub fn transmit(&self) -> bool {
        (self.value.read_bits() & Self::TRANSMIT_MASK) != 0
    }

    /// **RECEIVE** `[4]` (RO)
    ///
    /// 1: receive, the TWAI controller is receiving a message. 0: idle
    #[inline(always)]
    pub fn receive(&self) -> bool {
        (self.value.read_bits() & Self::RECEIVE_MASK) != 0
    }

    /// **TRANSMISSION_COMPLETE** `[3]` (RO)
    ///
    /// 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed
    #[inline(always)]
    pub fn transmission_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANSMISSION_COMPLETE_MASK) != 0
    }

    /// **TRANSMIT_BUFFER** `[2]` (RO)
    ///
    /// 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted
    #[inline(always)]
    pub fn transmit_buffer(&self) -> bool {
        (self.value.read_bits() & Self::TRANSMIT_BUFFER_MASK) != 0
    }

    /// **OVERRUN** `[1]` (RO)
    ///
    /// 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given
    #[inline(always)]
    pub fn overrun(&self) -> bool {
        (self.value.read_bits() & Self::OVERRUN_MASK) != 0
    }

    /// **RECEIVE_BUFFER** `[0]` (RO)
    ///
    /// 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available
    #[inline(always)]
    pub fn receive_buffer(&self) -> bool {
        (self.value.read_bits() & Self::RECEIVE_BUFFER_MASK) != 0
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

    /// Read **MISS** `[8:8]`
    ///
    /// 1: current message is destroyed because of FIFO overflow.
    #[inline(always)]
    pub const fn miss(&self) -> bool {
        (self.bits & Status::MISS_MASK) != 0
    }

    /// Read **NODE_BUS_OFF** `[7:7]`
    ///
    /// 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities
    #[inline(always)]
    pub const fn node_bus_off(&self) -> bool {
        (self.bits & Status::NODE_BUS_OFF_MASK) != 0
    }

    /// Read **ERR** `[6:6]`
    ///
    /// 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit
    #[inline(always)]
    pub const fn err(&self) -> bool {
        (self.bits & Status::ERR_MASK) != 0
    }

    /// Read **TRANSMIT** `[5:5]`
    ///
    /// 1: transmit, the TWAI controller is transmitting a message. 0: idle
    #[inline(always)]
    pub const fn transmit(&self) -> bool {
        (self.bits & Status::TRANSMIT_MASK) != 0
    }

    /// Read **RECEIVE** `[4:4]`
    ///
    /// 1: receive, the TWAI controller is receiving a message. 0: idle
    #[inline(always)]
    pub const fn receive(&self) -> bool {
        (self.bits & Status::RECEIVE_MASK) != 0
    }

    /// Read **TRANSMISSION_COMPLETE** `[3:3]`
    ///
    /// 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed
    #[inline(always)]
    pub const fn transmission_complete(&self) -> bool {
        (self.bits & Status::TRANSMISSION_COMPLETE_MASK) != 0
    }

    /// Read **TRANSMIT_BUFFER** `[2:2]`
    ///
    /// 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted
    #[inline(always)]
    pub const fn transmit_buffer(&self) -> bool {
        (self.bits & Status::TRANSMIT_BUFFER_MASK) != 0
    }

    /// Read **OVERRUN** `[1:1]`
    ///
    /// 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        (self.bits & Status::OVERRUN_MASK) != 0
    }

    /// Read **RECEIVE_BUFFER** `[0:0]`
    ///
    /// 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available
    #[inline(always)]
    pub const fn receive_buffer(&self) -> bool {
        (self.bits & Status::RECEIVE_BUFFER_MASK) != 0
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
    }
}

/// INTERRUPT Register
///
/// Interrupt signals' register.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **IDLE_INT_ST** `[8]` (RO)
///
///   1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset
///
/// - **BUS_ERR_INT_ST** `[7]` (RO)
///
///   1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset
///
/// - **ARBITRATION_LOST_INT_ST** `[6]` (RO)
///
///   1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset
///
/// - **ERR_PASSIVE_INT_ST** `[5]` (RO)
///
///   1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset
///
/// - **TS_COUNTER_OVFL_INT_ST** `[4]` (RO)
///
///   1: this bit is set then the timestamp counter reaches the maximum value and overflow.
///
/// - **DATA_OVERRUN_INT_ST** `[3]` (RO)
///
///   1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset
///
/// - **ERR_WARNING_INT_ST** `[2]` (RO)
///
///   1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset
///
/// - **TRANSMIT_INT_ST** `[1]` (RO)
///
///   1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset
///
/// - **RECEIVE_INT_ST** `[0]` (RO)
///
///   1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset
#[repr(transparent)]
pub struct Interrupt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interrupt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interrupt {
    // ======== Field Constants ========

    /// **IDLE_INT_ST**
    ///
    /// 1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IDLE_INT_ST_OFFSET: u32 = 8;
    pub const IDLE_INT_ST_MASK: u32 = 0x00000100;
    pub const IDLE_INT_ST: u32 = 1 << 8;

    /// **BUS_ERR_INT_ST**
    ///
    /// 1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUS_ERR_INT_ST_OFFSET: u32 = 7;
    pub const BUS_ERR_INT_ST_MASK: u32 = 0x00000080;
    pub const BUS_ERR_INT_ST: u32 = 1 << 7;

    /// **ARBITRATION_LOST_INT_ST**
    ///
    /// 1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ARBITRATION_LOST_INT_ST_OFFSET: u32 = 6;
    pub const ARBITRATION_LOST_INT_ST_MASK: u32 = 0x00000040;
    pub const ARBITRATION_LOST_INT_ST: u32 = 1 << 6;

    /// **ERR_PASSIVE_INT_ST**
    ///
    /// 1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERR_PASSIVE_INT_ST_OFFSET: u32 = 5;
    pub const ERR_PASSIVE_INT_ST_MASK: u32 = 0x00000020;
    pub const ERR_PASSIVE_INT_ST: u32 = 1 << 5;

    /// **TS_COUNTER_OVFL_INT_ST**
    ///
    /// 1: this bit is set then the timestamp counter reaches the maximum value and overflow.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TS_COUNTER_OVFL_INT_ST_OFFSET: u32 = 4;
    pub const TS_COUNTER_OVFL_INT_ST_MASK: u32 = 0x00000010;
    pub const TS_COUNTER_OVFL_INT_ST: u32 = 1 << 4;

    /// **DATA_OVERRUN_INT_ST**
    ///
    /// 1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DATA_OVERRUN_INT_ST_OFFSET: u32 = 3;
    pub const DATA_OVERRUN_INT_ST_MASK: u32 = 0x00000008;
    pub const DATA_OVERRUN_INT_ST: u32 = 1 << 3;

    /// **ERR_WARNING_INT_ST**
    ///
    /// 1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ERR_WARNING_INT_ST_OFFSET: u32 = 2;
    pub const ERR_WARNING_INT_ST_MASK: u32 = 0x00000004;
    pub const ERR_WARNING_INT_ST: u32 = 1 << 2;

    /// **TRANSMIT_INT_ST**
    ///
    /// 1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TRANSMIT_INT_ST_OFFSET: u32 = 1;
    pub const TRANSMIT_INT_ST_MASK: u32 = 0x00000002;
    pub const TRANSMIT_INT_ST: u32 = 1 << 1;

    /// **RECEIVE_INT_ST**
    ///
    /// 1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RECEIVE_INT_ST_OFFSET: u32 = 0;
    pub const RECEIVE_INT_ST_MASK: u32 = 0x00000001;
    pub const RECEIVE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IDLE_INT_ST** `[8]` (RO)
    ///
    /// 1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn idle_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_INT_ST_MASK) != 0
    }

    /// **BUS_ERR_INT_ST** `[7]` (RO)
    ///
    /// 1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn bus_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::BUS_ERR_INT_ST_MASK) != 0
    }

    /// **ARBITRATION_LOST_INT_ST** `[6]` (RO)
    ///
    /// 1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ARBITRATION_LOST_INT_ST_MASK) != 0
    }

    /// **ERR_PASSIVE_INT_ST** `[5]` (RO)
    ///
    /// 1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn err_passive_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ERR_PASSIVE_INT_ST_MASK) != 0
    }

    /// **TS_COUNTER_OVFL_INT_ST** `[4]` (RO)
    ///
    /// 1: this bit is set then the timestamp counter reaches the maximum value and overflow.
    #[inline(always)]
    pub fn ts_counter_ovfl_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TS_COUNTER_OVFL_INT_ST_MASK) != 0
    }

    /// **DATA_OVERRUN_INT_ST** `[3]` (RO)
    ///
    /// 1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn data_overrun_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DATA_OVERRUN_INT_ST_MASK) != 0
    }

    /// **ERR_WARNING_INT_ST** `[2]` (RO)
    ///
    /// 1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn err_warning_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ERR_WARNING_INT_ST_MASK) != 0
    }

    /// **TRANSMIT_INT_ST** `[1]` (RO)
    ///
    /// 1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn transmit_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TRANSMIT_INT_ST_MASK) != 0
    }

    /// **RECEIVE_INT_ST** `[0]` (RO)
    ///
    /// 1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub fn receive_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RECEIVE_INT_ST_MASK) != 0
    }
}

/// INTERRUPT value type for batch read/modify/write operations
///
/// Use `interrupt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interrupt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InterruptVal {
    bits: u32,
}

impl InterruptVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IDLE_INT_ST** `[8:8]`
    ///
    /// 1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn idle_int_st(&self) -> bool {
        (self.bits & Interrupt::IDLE_INT_ST_MASK) != 0
    }

    /// Read **BUS_ERR_INT_ST** `[7:7]`
    ///
    /// 1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn bus_err_int_st(&self) -> bool {
        (self.bits & Interrupt::BUS_ERR_INT_ST_MASK) != 0
    }

    /// Read **ARBITRATION_LOST_INT_ST** `[6:6]`
    ///
    /// 1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn arbitration_lost_int_st(&self) -> bool {
        (self.bits & Interrupt::ARBITRATION_LOST_INT_ST_MASK) != 0
    }

    /// Read **ERR_PASSIVE_INT_ST** `[5:5]`
    ///
    /// 1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn err_passive_int_st(&self) -> bool {
        (self.bits & Interrupt::ERR_PASSIVE_INT_ST_MASK) != 0
    }

    /// Read **TS_COUNTER_OVFL_INT_ST** `[4:4]`
    ///
    /// 1: this bit is set then the timestamp counter reaches the maximum value and overflow.
    #[inline(always)]
    pub const fn ts_counter_ovfl_int_st(&self) -> bool {
        (self.bits & Interrupt::TS_COUNTER_OVFL_INT_ST_MASK) != 0
    }

    /// Read **DATA_OVERRUN_INT_ST** `[3:3]`
    ///
    /// 1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn data_overrun_int_st(&self) -> bool {
        (self.bits & Interrupt::DATA_OVERRUN_INT_ST_MASK) != 0
    }

    /// Read **ERR_WARNING_INT_ST** `[2:2]`
    ///
    /// 1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn err_warning_int_st(&self) -> bool {
        (self.bits & Interrupt::ERR_WARNING_INT_ST_MASK) != 0
    }

    /// Read **TRANSMIT_INT_ST** `[1:1]`
    ///
    /// 1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn transmit_int_st(&self) -> bool {
        (self.bits & Interrupt::TRANSMIT_INT_ST_MASK) != 0
    }

    /// Read **RECEIVE_INT_ST** `[0:0]`
    ///
    /// 1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset
    #[inline(always)]
    pub const fn receive_int_st(&self) -> bool {
        (self.bits & Interrupt::RECEIVE_INT_ST_MASK) != 0
    }
}

impl Default for InterruptVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Interrupt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InterruptVal {
        InterruptVal::from_bits(self.value.read_bits())
    }
}

/// INTERRUPT_ENABLE Register
///
/// Interrupt enable register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **IDLE_INT_ENA** `[8]` (RO)
///
///   1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
///
/// - **BUS_ERR_INT_ENA** `[7]` (RW)
///
///   1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
///
/// - **ARBITRATION_LOST_INT_ENA** `[6]` (RW)
///
///   1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
///
/// - **ERR_PASSIVE_INT_ENA** `[5]` (RW)
///
///   1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
///
/// - **TS_COUNTER_OVFL_INT_ENA** `[4]` (RW)
///
///   enable the timestamp counter overflow interrupt request.
///
/// - **EXT_DATA_OVERRUN_INT_ENA** `[3]` (RW)
///
///   1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
///
/// - **EXT_ERR_WARNING_INT_ENA** `[2]` (RW)
///
///   1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
///
/// - **EXT_TRANSMIT_INT_ENA** `[1]` (RW)
///
///   1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
///
/// - **EXT_RECEIVE_INT_ENA** `[0]` (RW)
///
///   1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
#[repr(transparent)]
pub struct InterruptEnable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InterruptEnable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InterruptEnable {
    // ======== Field Constants ========

    /// **IDLE_INT_ENA**
    ///
    /// 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IDLE_INT_ENA_OFFSET: u32 = 8;
    pub const IDLE_INT_ENA_MASK: u32 = 0x00000100;
    pub const IDLE_INT_ENA: u32 = 1 << 8;

    /// **BUS_ERR_INT_ENA**
    ///
    /// 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUS_ERR_INT_ENA_OFFSET: u32 = 7;
    pub const BUS_ERR_INT_ENA_MASK: u32 = 0x00000080;
    pub const BUS_ERR_INT_ENA: u32 = 1 << 7;

    /// **ARBITRATION_LOST_INT_ENA**
    ///
    /// 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ARBITRATION_LOST_INT_ENA_OFFSET: u32 = 6;
    pub const ARBITRATION_LOST_INT_ENA_MASK: u32 = 0x00000040;
    pub const ARBITRATION_LOST_INT_ENA: u32 = 1 << 6;

    /// **ERR_PASSIVE_INT_ENA**
    ///
    /// 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERR_PASSIVE_INT_ENA_OFFSET: u32 = 5;
    pub const ERR_PASSIVE_INT_ENA_MASK: u32 = 0x00000020;
    pub const ERR_PASSIVE_INT_ENA: u32 = 1 << 5;

    /// **TS_COUNTER_OVFL_INT_ENA**
    ///
    /// enable the timestamp counter overflow interrupt request.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TS_COUNTER_OVFL_INT_ENA_OFFSET: u32 = 4;
    pub const TS_COUNTER_OVFL_INT_ENA_MASK: u32 = 0x00000010;
    pub const TS_COUNTER_OVFL_INT_ENA: u32 = 1 << 4;

    /// **EXT_DATA_OVERRUN_INT_ENA**
    ///
    /// 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EXT_DATA_OVERRUN_INT_ENA_OFFSET: u32 = 3;
    pub const EXT_DATA_OVERRUN_INT_ENA_MASK: u32 = 0x00000008;
    pub const EXT_DATA_OVERRUN_INT_ENA: u32 = 1 << 3;

    /// **EXT_ERR_WARNING_INT_ENA**
    ///
    /// 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EXT_ERR_WARNING_INT_ENA_OFFSET: u32 = 2;
    pub const EXT_ERR_WARNING_INT_ENA_MASK: u32 = 0x00000004;
    pub const EXT_ERR_WARNING_INT_ENA: u32 = 1 << 2;

    /// **EXT_TRANSMIT_INT_ENA**
    ///
    /// 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EXT_TRANSMIT_INT_ENA_OFFSET: u32 = 1;
    pub const EXT_TRANSMIT_INT_ENA_MASK: u32 = 0x00000002;
    pub const EXT_TRANSMIT_INT_ENA: u32 = 1 << 1;

    /// **EXT_RECEIVE_INT_ENA**
    ///
    /// 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EXT_RECEIVE_INT_ENA_OFFSET: u32 = 0;
    pub const EXT_RECEIVE_INT_ENA_MASK: u32 = 0x00000001;
    pub const EXT_RECEIVE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IDLE_INT_ENA** `[8]` (RO)
    ///
    /// 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn idle_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_INT_ENA_MASK) != 0
    }

    /// **BUS_ERR_INT_ENA** `[7]` (RW)
    ///
    /// 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::BUS_ERR_INT_ENA_MASK) != 0
    }

    /// Set BUS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bus_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::BUS_ERR_INT_ENA_MASK);
    }

    /// Clear BUS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bus_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::BUS_ERR_INT_ENA_MASK);
    }

    /// **ARBITRATION_LOST_INT_ENA** `[6]` (RW)
    ///
    /// 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::ARBITRATION_LOST_INT_ENA_MASK) != 0
    }

    /// Set ARBITRATION_LOST_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arbitration_lost_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::ARBITRATION_LOST_INT_ENA_MASK);
    }

    /// Clear ARBITRATION_LOST_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arbitration_lost_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::ARBITRATION_LOST_INT_ENA_MASK);
    }

    /// **ERR_PASSIVE_INT_ENA** `[5]` (RW)
    ///
    /// 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::ERR_PASSIVE_INT_ENA_MASK) != 0
    }

    /// Set ERR_PASSIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_err_passive_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::ERR_PASSIVE_INT_ENA_MASK);
    }

    /// Clear ERR_PASSIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_err_passive_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::ERR_PASSIVE_INT_ENA_MASK);
    }

    /// **TS_COUNTER_OVFL_INT_ENA** `[4]` (RW)
    ///
    /// enable the timestamp counter overflow interrupt request.
    #[inline(always)]
    pub fn ts_counter_ovfl_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TS_COUNTER_OVFL_INT_ENA_MASK) != 0
    }

    /// Set TS_COUNTER_OVFL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ts_counter_ovfl_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TS_COUNTER_OVFL_INT_ENA_MASK);
    }

    /// Clear TS_COUNTER_OVFL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ts_counter_ovfl_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TS_COUNTER_OVFL_INT_ENA_MASK);
    }

    /// **EXT_DATA_OVERRUN_INT_ENA** `[3]` (RW)
    ///
    /// 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_data_overrun_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EXT_DATA_OVERRUN_INT_ENA_MASK) != 0
    }

    /// Set EXT_DATA_OVERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ext_data_overrun_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EXT_DATA_OVERRUN_INT_ENA_MASK);
    }

    /// Clear EXT_DATA_OVERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ext_data_overrun_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EXT_DATA_OVERRUN_INT_ENA_MASK);
    }

    /// **EXT_ERR_WARNING_INT_ENA** `[2]` (RW)
    ///
    /// 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_err_warning_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EXT_ERR_WARNING_INT_ENA_MASK) != 0
    }

    /// Set EXT_ERR_WARNING_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ext_err_warning_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EXT_ERR_WARNING_INT_ENA_MASK);
    }

    /// Clear EXT_ERR_WARNING_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ext_err_warning_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EXT_ERR_WARNING_INT_ENA_MASK);
    }

    /// **EXT_TRANSMIT_INT_ENA** `[1]` (RW)
    ///
    /// 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_transmit_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EXT_TRANSMIT_INT_ENA_MASK) != 0
    }

    /// Set EXT_TRANSMIT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ext_transmit_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EXT_TRANSMIT_INT_ENA_MASK);
    }

    /// Clear EXT_TRANSMIT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ext_transmit_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EXT_TRANSMIT_INT_ENA_MASK);
    }

    /// **EXT_RECEIVE_INT_ENA** `[0]` (RW)
    ///
    /// 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_receive_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EXT_RECEIVE_INT_ENA_MASK) != 0
    }

    /// Set EXT_RECEIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ext_receive_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EXT_RECEIVE_INT_ENA_MASK);
    }

    /// Clear EXT_RECEIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ext_receive_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EXT_RECEIVE_INT_ENA_MASK);
    }
}

/// INTERRUPT_ENABLE value type for batch read/modify/write operations
///
/// Use `interruptenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interruptenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InterruptEnableVal {
    bits: u32,
}

impl InterruptEnableVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IDLE_INT_ENA** `[8:8]`
    ///
    /// 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn idle_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::IDLE_INT_ENA_MASK) != 0
    }

    /// Read **BUS_ERR_INT_ENA** `[7:7]`
    ///
    /// 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn bus_err_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::BUS_ERR_INT_ENA_MASK) != 0
    }

    /// Read **ARBITRATION_LOST_INT_ENA** `[6:6]`
    ///
    /// 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub const fn arbitration_lost_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::ARBITRATION_LOST_INT_ENA_MASK) != 0
    }

    /// Read **ERR_PASSIVE_INT_ENA** `[5:5]`
    ///
    /// 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub const fn err_passive_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::ERR_PASSIVE_INT_ENA_MASK) != 0
    }

    /// Read **TS_COUNTER_OVFL_INT_ENA** `[4:4]`
    ///
    /// enable the timestamp counter overflow interrupt request.
    #[inline(always)]
    pub const fn ts_counter_ovfl_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::TS_COUNTER_OVFL_INT_ENA_MASK) != 0
    }

    /// Read **EXT_DATA_OVERRUN_INT_ENA** `[3:3]`
    ///
    /// 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn ext_data_overrun_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::EXT_DATA_OVERRUN_INT_ENA_MASK) != 0
    }

    /// Read **EXT_ERR_WARNING_INT_ENA** `[2:2]`
    ///
    /// 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn ext_err_warning_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::EXT_ERR_WARNING_INT_ENA_MASK) != 0
    }

    /// Read **EXT_TRANSMIT_INT_ENA** `[1:1]`
    ///
    /// 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn ext_transmit_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::EXT_TRANSMIT_INT_ENA_MASK) != 0
    }

    /// Read **EXT_RECEIVE_INT_ENA** `[0:0]`
    ///
    /// 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn ext_receive_int_ena(&self) -> bool {
        (self.bits & InterruptEnable::EXT_RECEIVE_INT_ENA_MASK) != 0
    }

    /// Set **BUS_ERR_INT_ENA** `[7:7]`
    ///
    /// 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn with_bus_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::BUS_ERR_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::BUS_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **ARBITRATION_LOST_INT_ENA** `[6:6]`
    ///
    /// 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub const fn with_arbitration_lost_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::ARBITRATION_LOST_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::ARBITRATION_LOST_INT_ENA_MASK
            },
        }
    }

    /// Set **ERR_PASSIVE_INT_ENA** `[5:5]`
    ///
    /// 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub const fn with_err_passive_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::ERR_PASSIVE_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::ERR_PASSIVE_INT_ENA_MASK
            },
        }
    }

    /// Set **TS_COUNTER_OVFL_INT_ENA** `[4:4]`
    ///
    /// enable the timestamp counter overflow interrupt request.
    #[inline(always)]
    pub const fn with_ts_counter_ovfl_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::TS_COUNTER_OVFL_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::TS_COUNTER_OVFL_INT_ENA_MASK
            },
        }
    }

    /// Set **EXT_DATA_OVERRUN_INT_ENA** `[3:3]`
    ///
    /// 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn with_ext_data_overrun_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::EXT_DATA_OVERRUN_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::EXT_DATA_OVERRUN_INT_ENA_MASK
            },
        }
    }

    /// Set **EXT_ERR_WARNING_INT_ENA** `[2:2]`
    ///
    /// 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn with_ext_err_warning_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::EXT_ERR_WARNING_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::EXT_ERR_WARNING_INT_ENA_MASK
            },
        }
    }

    /// Set **EXT_TRANSMIT_INT_ENA** `[1:1]`
    ///
    /// 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn with_ext_transmit_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::EXT_TRANSMIT_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::EXT_TRANSMIT_INT_ENA_MASK
            },
        }
    }

    /// Set **EXT_RECEIVE_INT_ENA** `[0:0]`
    ///
    /// 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub const fn with_ext_receive_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | InterruptEnable::EXT_RECEIVE_INT_ENA_MASK
            } else {
                self.bits & !InterruptEnable::EXT_RECEIVE_INT_ENA_MASK
            },
        }
    }
}

impl Default for InterruptEnableVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InterruptEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InterruptEnableVal {
        InterruptEnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InterruptEnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InterruptEnableVal) -> InterruptEnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUS_TIMING_0 Register
///
/// Bit timing configuration register 0.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **SYNC_JUMP_WIDTH** `[15:14]` (RW)
///
///   The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
///
/// - **BAUD_PRESC** `[13:0]` (RW)
///
///   The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
#[repr(transparent)]
pub struct BusTiming0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusTiming0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusTiming0 {
    // ======== Field Constants ========

    /// **SYNC_JUMP_WIDTH**
    ///
    /// The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SYNC_JUMP_WIDTH_OFFSET: u32 = 14;
    pub const SYNC_JUMP_WIDTH_MASK: u32 = 0x0000c000;

    /// **BAUD_PRESC**
    ///
    /// The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const BAUD_PRESC_OFFSET: u32 = 0;
    pub const BAUD_PRESC_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **SYNC_JUMP_WIDTH** `[15:14]` (RW)
    ///
    /// The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn sync_jump_width(&self) -> u32 {
        (self.value.read_bits() & Self::SYNC_JUMP_WIDTH_MASK) >> Self::SYNC_JUMP_WIDTH_OFFSET
    }

    /// Write **SYNC_JUMP_WIDTH** `[15:14]` (RW)
    ///
    /// The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_sync_jump_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SYNC_JUMP_WIDTH_MASK)
                | ((value << Self::SYNC_JUMP_WIDTH_OFFSET) & Self::SYNC_JUMP_WIDTH_MASK)
        });
    }

    /// **BAUD_PRESC** `[13:0]` (RW)
    ///
    /// The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
    #[inline(always)]
    pub fn baud_presc(&self) -> u32 {
        (self.value.read_bits() & Self::BAUD_PRESC_MASK) >> Self::BAUD_PRESC_OFFSET
    }

    /// Write **BAUD_PRESC** `[13:0]` (RW)
    ///
    /// The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
    #[inline(always)]
    pub fn write_baud_presc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BAUD_PRESC_MASK)
                | ((value << Self::BAUD_PRESC_OFFSET) & Self::BAUD_PRESC_MASK)
        });
    }
}

/// BUS_TIMING_0 value type for batch read/modify/write operations
///
/// Use `bustiming0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bustiming0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusTiming0Val {
    bits: u32,
}

impl BusTiming0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYNC_JUMP_WIDTH** `[15:14]`
    ///
    /// The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn sync_jump_width(&self) -> u32 {
        (self.bits & BusTiming0::SYNC_JUMP_WIDTH_MASK) >> BusTiming0::SYNC_JUMP_WIDTH_OFFSET
    }

    /// Read **BAUD_PRESC** `[13:0]`
    ///
    /// The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
    #[inline(always)]
    pub const fn baud_presc(&self) -> u32 {
        (self.bits & BusTiming0::BAUD_PRESC_MASK) >> BusTiming0::BAUD_PRESC_OFFSET
    }

    /// Set **SYNC_JUMP_WIDTH** `[15:14]`
    ///
    /// The synchronization jump width defines the maximum number of clock cycles a bit period may be shortened or lengthened. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_sync_jump_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BusTiming0::SYNC_JUMP_WIDTH_MASK)
                | ((value << BusTiming0::SYNC_JUMP_WIDTH_OFFSET)
                    & BusTiming0::SYNC_JUMP_WIDTH_MASK),
        }
    }

    /// Set **BAUD_PRESC** `[13:0]`
    ///
    /// The period of the TWAI system clock is programmable and determines the individual bit timing. Software has R/W permission in reset mode and RO permission in operation mode.
    #[inline(always)]
    pub const fn with_baud_presc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BusTiming0::BAUD_PRESC_MASK)
                | ((value << BusTiming0::BAUD_PRESC_OFFSET) & BusTiming0::BAUD_PRESC_MASK),
        }
    }
}

impl Default for BusTiming0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BusTiming0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusTiming0Val {
        BusTiming0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusTiming0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusTiming0Val) -> BusTiming0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUS_TIMING_1 Register
///
/// Bit timing configuration register 1.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **TIME_SAMPLING** `[7]` (RW)
///
///   1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode.
///
/// - **TIME_SEGMENT2** `[6:4]` (RW)
///
///   The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
///
/// - **TIME_SEGMENT1** `[3:0]` (RW)
///
///   The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
#[repr(transparent)]
pub struct BusTiming1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusTiming1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusTiming1 {
    // ======== Field Constants ========

    /// **TIME_SAMPLING**
    ///
    /// 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TIME_SAMPLING_OFFSET: u32 = 7;
    pub const TIME_SAMPLING_MASK: u32 = 0x00000080;
    pub const TIME_SAMPLING: u32 = 1 << 7;

    /// **TIME_SEGMENT2**
    ///
    /// The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const TIME_SEGMENT2_OFFSET: u32 = 4;
    pub const TIME_SEGMENT2_MASK: u32 = 0x00000070;

    /// **TIME_SEGMENT1**
    ///
    /// The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TIME_SEGMENT1_OFFSET: u32 = 0;
    pub const TIME_SEGMENT1_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TIME_SAMPLING** `[7]` (RW)
    ///
    /// 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn time_sampling(&self) -> bool {
        (self.value.read_bits() & Self::TIME_SAMPLING_MASK) != 0
    }

    /// Set TIME_SAMPLING bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_time_sampling(&self) {
        self.value.modify_bits(|r| r | Self::TIME_SAMPLING_MASK);
    }

    /// Clear TIME_SAMPLING bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_time_sampling(&self) {
        self.value.modify_bits(|r| r & !Self::TIME_SAMPLING_MASK);
    }

    /// **TIME_SEGMENT2** `[6:4]` (RW)
    ///
    /// The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn time_segment2(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_SEGMENT2_MASK) >> Self::TIME_SEGMENT2_OFFSET
    }

    /// Write **TIME_SEGMENT2** `[6:4]` (RW)
    ///
    /// The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_time_segment2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_SEGMENT2_MASK)
                | ((value << Self::TIME_SEGMENT2_OFFSET) & Self::TIME_SEGMENT2_MASK)
        });
    }

    /// **TIME_SEGMENT1** `[3:0]` (RW)
    ///
    /// The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn time_segment1(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_SEGMENT1_MASK) >> Self::TIME_SEGMENT1_OFFSET
    }

    /// Write **TIME_SEGMENT1** `[3:0]` (RW)
    ///
    /// The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_time_segment1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_SEGMENT1_MASK)
                | ((value << Self::TIME_SEGMENT1_OFFSET) & Self::TIME_SEGMENT1_MASK)
        });
    }
}

/// BUS_TIMING_1 value type for batch read/modify/write operations
///
/// Use `bustiming1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bustiming1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusTiming1Val {
    bits: u32,
}

impl BusTiming1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIME_SAMPLING** `[7:7]`
    ///
    /// 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn time_sampling(&self) -> bool {
        (self.bits & BusTiming1::TIME_SAMPLING_MASK) != 0
    }

    /// Read **TIME_SEGMENT2** `[6:4]`
    ///
    /// The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn time_segment2(&self) -> u32 {
        (self.bits & BusTiming1::TIME_SEGMENT2_MASK) >> BusTiming1::TIME_SEGMENT2_OFFSET
    }

    /// Read **TIME_SEGMENT1** `[3:0]`
    ///
    /// The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn time_segment1(&self) -> u32 {
        (self.bits & BusTiming1::TIME_SEGMENT1_MASK) >> BusTiming1::TIME_SEGMENT1_OFFSET
    }

    /// Set **TIME_SAMPLING** `[7:7]`
    ///
    /// 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_time_sampling(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BusTiming1::TIME_SAMPLING_MASK
            } else {
                self.bits & !BusTiming1::TIME_SAMPLING_MASK
            },
        }
    }

    /// Set **TIME_SEGMENT2** `[6:4]`
    ///
    /// The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_time_segment2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BusTiming1::TIME_SEGMENT2_MASK)
                | ((value << BusTiming1::TIME_SEGMENT2_OFFSET) & BusTiming1::TIME_SEGMENT2_MASK),
        }
    }

    /// Set **TIME_SEGMENT1** `[3:0]`
    ///
    /// The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_time_segment1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BusTiming1::TIME_SEGMENT1_MASK)
                | ((value << BusTiming1::TIME_SEGMENT1_OFFSET) & BusTiming1::TIME_SEGMENT1_MASK),
        }
    }
}

impl Default for BusTiming1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BusTiming1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusTiming1Val {
        BusTiming1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusTiming1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusTiming1Val) -> BusTiming1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ARB_LOST_CAP Register
///
/// TWAI arbiter lost capture register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **ARBITRATION_LOST_CAPTURE** `[4:0]` (RO)
///
///   This register contains information about the bit position of losing arbitration.
#[repr(transparent)]
pub struct ArbLostCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ArbLostCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ArbLostCap {
    // ======== Field Constants ========

    /// **ARBITRATION_LOST_CAPTURE**
    ///
    /// This register contains information about the bit position of losing arbitration.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const ARBITRATION_LOST_CAPTURE_OFFSET: u32 = 0;
    pub const ARBITRATION_LOST_CAPTURE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **ARBITRATION_LOST_CAPTURE** `[4:0]` (RO)
    ///
    /// This register contains information about the bit position of losing arbitration.
    #[inline(always)]
    pub fn arbitration_lost_capture(&self) -> u32 {
        (self.value.read_bits() & Self::ARBITRATION_LOST_CAPTURE_MASK)
            >> Self::ARBITRATION_LOST_CAPTURE_OFFSET
    }
}

/// ARB_LOST_CAP value type for batch read/modify/write operations
///
/// Use `arblostcap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arblostcap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArbLostCapVal {
    bits: u32,
}

impl ArbLostCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ARBITRATION_LOST_CAPTURE** `[4:0]`
    ///
    /// This register contains information about the bit position of losing arbitration.
    #[inline(always)]
    pub const fn arbitration_lost_capture(&self) -> u32 {
        (self.bits & ArbLostCap::ARBITRATION_LOST_CAPTURE_MASK)
            >> ArbLostCap::ARBITRATION_LOST_CAPTURE_OFFSET
    }
}

impl Default for ArbLostCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ArbLostCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArbLostCapVal {
        ArbLostCapVal::from_bits(self.value.read_bits())
    }
}

/// ERR_CODE_CAP Register
///
/// TWAI error info capture register.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **ERR_CAPTURE_CODE_TYPE** `[7:6]` (RO)
///
///   00: bit error. 01: form error. 10:stuff error. 11:other type of error.
///
/// - **ERR_CAPTURE_CODE_DIRECTION** `[5]` (RO)
///
///   1: RX, error occurred during reception. 0: TX, error occurred during transmission.
///
/// - **ERR_CAPTURE_CODE_SEGMENT** `[4:0]` (RO)
///
///   This register contains information about the location of errors on the bus.
#[repr(transparent)]
pub struct ErrCodeCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ErrCodeCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ErrCodeCap {
    // ======== Field Constants ========

    /// **ERR_CAPTURE_CODE_TYPE**
    ///
    /// 00: bit error. 01: form error. 10:stuff error. 11:other type of error.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const ERR_CAPTURE_CODE_TYPE_OFFSET: u32 = 6;
    pub const ERR_CAPTURE_CODE_TYPE_MASK: u32 = 0x000000c0;

    /// **ERR_CAPTURE_CODE_DIRECTION**
    ///
    /// 1: RX, error occurred during reception. 0: TX, error occurred during transmission.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERR_CAPTURE_CODE_DIRECTION_OFFSET: u32 = 5;
    pub const ERR_CAPTURE_CODE_DIRECTION_MASK: u32 = 0x00000020;
    pub const ERR_CAPTURE_CODE_DIRECTION: u32 = 1 << 5;

    /// **ERR_CAPTURE_CODE_SEGMENT**
    ///
    /// This register contains information about the location of errors on the bus.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const ERR_CAPTURE_CODE_SEGMENT_OFFSET: u32 = 0;
    pub const ERR_CAPTURE_CODE_SEGMENT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **ERR_CAPTURE_CODE_TYPE** `[7:6]` (RO)
    ///
    /// 00: bit error. 01: form error. 10:stuff error. 11:other type of error.
    #[inline(always)]
    pub fn err_capture_code_type(&self) -> u32 {
        (self.value.read_bits() & Self::ERR_CAPTURE_CODE_TYPE_MASK)
            >> Self::ERR_CAPTURE_CODE_TYPE_OFFSET
    }

    /// **ERR_CAPTURE_CODE_DIRECTION** `[5]` (RO)
    ///
    /// 1: RX, error occurred during reception. 0: TX, error occurred during transmission.
    #[inline(always)]
    pub fn err_capture_code_direction(&self) -> bool {
        (self.value.read_bits() & Self::ERR_CAPTURE_CODE_DIRECTION_MASK) != 0
    }

    /// **ERR_CAPTURE_CODE_SEGMENT** `[4:0]` (RO)
    ///
    /// This register contains information about the location of errors on the bus.
    #[inline(always)]
    pub fn err_capture_code_segment(&self) -> u32 {
        (self.value.read_bits() & Self::ERR_CAPTURE_CODE_SEGMENT_MASK)
            >> Self::ERR_CAPTURE_CODE_SEGMENT_OFFSET
    }
}

/// ERR_CODE_CAP value type for batch read/modify/write operations
///
/// Use `errcodecap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `errcodecap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ErrCodeCapVal {
    bits: u32,
}

impl ErrCodeCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ERR_CAPTURE_CODE_TYPE** `[7:6]`
    ///
    /// 00: bit error. 01: form error. 10:stuff error. 11:other type of error.
    #[inline(always)]
    pub const fn err_capture_code_type(&self) -> u32 {
        (self.bits & ErrCodeCap::ERR_CAPTURE_CODE_TYPE_MASK)
            >> ErrCodeCap::ERR_CAPTURE_CODE_TYPE_OFFSET
    }

    /// Read **ERR_CAPTURE_CODE_DIRECTION** `[5:5]`
    ///
    /// 1: RX, error occurred during reception. 0: TX, error occurred during transmission.
    #[inline(always)]
    pub const fn err_capture_code_direction(&self) -> bool {
        (self.bits & ErrCodeCap::ERR_CAPTURE_CODE_DIRECTION_MASK) != 0
    }

    /// Read **ERR_CAPTURE_CODE_SEGMENT** `[4:0]`
    ///
    /// This register contains information about the location of errors on the bus.
    #[inline(always)]
    pub const fn err_capture_code_segment(&self) -> u32 {
        (self.bits & ErrCodeCap::ERR_CAPTURE_CODE_SEGMENT_MASK)
            >> ErrCodeCap::ERR_CAPTURE_CODE_SEGMENT_OFFSET
    }
}

impl Default for ErrCodeCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ErrCodeCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ErrCodeCapVal {
        ErrCodeCapVal::from_bits(self.value.read_bits())
    }
}

/// ERR_WARNING_LIMIT Register
///
/// TWAI error threshold configuration register.
///
/// Address offset: 0x0034
/// Reset value: 0x00000060
///
/// ## Fields
///
/// - **ERR_WARNING_LIMIT** `[7:0]` (RW)
///
///   The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
#[repr(transparent)]
pub struct ErrWarningLimit {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ErrWarningLimit {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ErrWarningLimit {
    // ======== Field Constants ========

    /// **ERR_WARNING_LIMIT**
    ///
    /// The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ERR_WARNING_LIMIT_OFFSET: u32 = 0;
    pub const ERR_WARNING_LIMIT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ERR_WARNING_LIMIT** `[7:0]` (RW)
    ///
    /// The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn err_warning_limit(&self) -> u32 {
        (self.value.read_bits() & Self::ERR_WARNING_LIMIT_MASK) >> Self::ERR_WARNING_LIMIT_OFFSET
    }

    /// Write **ERR_WARNING_LIMIT** `[7:0]` (RW)
    ///
    /// The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_err_warning_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ERR_WARNING_LIMIT_MASK)
                | ((value << Self::ERR_WARNING_LIMIT_OFFSET) & Self::ERR_WARNING_LIMIT_MASK)
        });
    }
}

/// ERR_WARNING_LIMIT value type for batch read/modify/write operations
///
/// Use `errwarninglimit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `errwarninglimit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ErrWarningLimitVal {
    bits: u32,
}

impl ErrWarningLimitVal {
    /// Create from raw bits
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
        Self { bits: 0x00000060 }
    }

    /// Read **ERR_WARNING_LIMIT** `[7:0]`
    ///
    /// The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn err_warning_limit(&self) -> u32 {
        (self.bits & ErrWarningLimit::ERR_WARNING_LIMIT_MASK)
            >> ErrWarningLimit::ERR_WARNING_LIMIT_OFFSET
    }

    /// Set **ERR_WARNING_LIMIT** `[7:0]`
    ///
    /// The threshold that trigger error warning interrupt when this interrupt is enabled. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_err_warning_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ErrWarningLimit::ERR_WARNING_LIMIT_MASK)
                | ((value << ErrWarningLimit::ERR_WARNING_LIMIT_OFFSET)
                    & ErrWarningLimit::ERR_WARNING_LIMIT_MASK),
        }
    }
}

impl Default for ErrWarningLimitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ErrWarningLimit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ErrWarningLimitVal {
        ErrWarningLimitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ErrWarningLimitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ErrWarningLimitVal) -> ErrWarningLimitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_ERR_CNT Register
///
/// Rx error counter register.
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **RX_ERR_CNT** `[7:0]` (RW)
///
///   The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
#[repr(transparent)]
pub struct RxErrCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxErrCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxErrCnt {
    // ======== Field Constants ========

    /// **RX_ERR_CNT**
    ///
    /// The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RX_ERR_CNT_OFFSET: u32 = 0;
    pub const RX_ERR_CNT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RX_ERR_CNT** `[7:0]` (RW)
    ///
    /// The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_ERR_CNT_MASK) >> Self::RX_ERR_CNT_OFFSET
    }

    /// Write **RX_ERR_CNT** `[7:0]` (RW)
    ///
    /// The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_rx_err_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_ERR_CNT_MASK)
                | ((value << Self::RX_ERR_CNT_OFFSET) & Self::RX_ERR_CNT_MASK)
        });
    }
}

/// RX_ERR_CNT value type for batch read/modify/write operations
///
/// Use `rxerrcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxerrcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxErrCntVal {
    bits: u32,
}

impl RxErrCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_ERR_CNT** `[7:0]`
    ///
    /// The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn rx_err_cnt(&self) -> u32 {
        (self.bits & RxErrCnt::RX_ERR_CNT_MASK) >> RxErrCnt::RX_ERR_CNT_OFFSET
    }

    /// Set **RX_ERR_CNT** `[7:0]`
    ///
    /// The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_rx_err_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RxErrCnt::RX_ERR_CNT_MASK)
                | ((value << RxErrCnt::RX_ERR_CNT_OFFSET) & RxErrCnt::RX_ERR_CNT_MASK),
        }
    }
}

impl Default for RxErrCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxErrCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxErrCntVal {
        RxErrCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RxErrCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RxErrCntVal) -> RxErrCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_ERR_CNT Register
///
/// Tx error counter register.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **TX_ERR_CNT** `[7:0]` (RW)
///
///   The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
#[repr(transparent)]
pub struct TxErrCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxErrCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxErrCnt {
    // ======== Field Constants ========

    /// **TX_ERR_CNT**
    ///
    /// The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_ERR_CNT_OFFSET: u32 = 0;
    pub const TX_ERR_CNT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_ERR_CNT** `[7:0]` (RW)
    ///
    /// The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_ERR_CNT_MASK) >> Self::TX_ERR_CNT_OFFSET
    }

    /// Write **TX_ERR_CNT** `[7:0]` (RW)
    ///
    /// The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn write_tx_err_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_ERR_CNT_MASK)
                | ((value << Self::TX_ERR_CNT_OFFSET) & Self::TX_ERR_CNT_MASK)
        });
    }
}

/// TX_ERR_CNT value type for batch read/modify/write operations
///
/// Use `txerrcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txerrcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxErrCntVal {
    bits: u32,
}

impl TxErrCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_ERR_CNT** `[7:0]`
    ///
    /// The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn tx_err_cnt(&self) -> u32 {
        (self.bits & TxErrCnt::TX_ERR_CNT_MASK) >> TxErrCnt::TX_ERR_CNT_OFFSET
    }

    /// Set **TX_ERR_CNT** `[7:0]`
    ///
    /// The TX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_tx_err_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxErrCnt::TX_ERR_CNT_MASK)
                | ((value << TxErrCnt::TX_ERR_CNT_OFFSET) & TxErrCnt::TX_ERR_CNT_MASK),
        }
    }
}

impl Default for TxErrCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxErrCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxErrCntVal {
        TxErrCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxErrCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxErrCntVal) -> TxErrCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_0 Register
///
/// Data register 0.
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **DATA_0** `[7:0]` (RW)
///
///   In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
#[repr(transparent)]
pub struct Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data0 {
    // ======== Field Constants ========

    /// **DATA_0**
    ///
    /// In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_0_OFFSET: u32 = 0;
    pub const DATA_0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_0** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
    #[inline(always)]
    pub fn data_0(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_0_MASK) >> Self::DATA_0_OFFSET
    }

    /// Write **DATA_0** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
    #[inline(always)]
    pub fn write_data_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_0_MASK) | ((value << Self::DATA_0_OFFSET) & Self::DATA_0_MASK)
        });
    }
}

/// DATA_0 value type for batch read/modify/write operations
///
/// Use `data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data0Val {
    bits: u32,
}

impl Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_0** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
    #[inline(always)]
    pub const fn data_0(&self) -> u32 {
        (self.bits & Data0::DATA_0_MASK) >> Data0::DATA_0_OFFSET
    }

    /// Set **DATA_0** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0.
    #[inline(always)]
    pub const fn with_data_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data0::DATA_0_MASK)
                | ((value << Data0::DATA_0_OFFSET) & Data0::DATA_0_MASK),
        }
    }
}

impl Default for Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data0Val {
        Data0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data0Val) -> Data0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_1 Register
///
/// Data register 1.
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DATA_1** `[7:0]` (RW)
///
///   In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
#[repr(transparent)]
pub struct Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data1 {
    // ======== Field Constants ========

    /// **DATA_1**
    ///
    /// In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_1_OFFSET: u32 = 0;
    pub const DATA_1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_1** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
    #[inline(always)]
    pub fn data_1(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_1_MASK) >> Self::DATA_1_OFFSET
    }

    /// Write **DATA_1** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
    #[inline(always)]
    pub fn write_data_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_1_MASK) | ((value << Self::DATA_1_OFFSET) & Self::DATA_1_MASK)
        });
    }
}

/// DATA_1 value type for batch read/modify/write operations
///
/// Use `data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data1Val {
    bits: u32,
}

impl Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_1** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
    #[inline(always)]
    pub const fn data_1(&self) -> u32 {
        (self.bits & Data1::DATA_1_MASK) >> Data1::DATA_1_OFFSET
    }

    /// Set **DATA_1** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1.
    #[inline(always)]
    pub const fn with_data_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data1::DATA_1_MASK)
                | ((value << Data1::DATA_1_OFFSET) & Data1::DATA_1_MASK),
        }
    }
}

impl Default for Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data1Val {
        Data1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data1Val) -> Data1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_2 Register
///
/// Data register 2.
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **DATA_2** `[7:0]` (RW)
///
///   In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
#[repr(transparent)]
pub struct Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data2 {
    // ======== Field Constants ========

    /// **DATA_2**
    ///
    /// In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_2_OFFSET: u32 = 0;
    pub const DATA_2_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_2** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    pub fn data_2(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_2_MASK) >> Self::DATA_2_OFFSET
    }

    /// Write **DATA_2** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    pub fn write_data_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_2_MASK) | ((value << Self::DATA_2_OFFSET) & Self::DATA_2_MASK)
        });
    }
}

/// DATA_2 value type for batch read/modify/write operations
///
/// Use `data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data2Val {
    bits: u32,
}

impl Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_2** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    pub const fn data_2(&self) -> u32 {
        (self.bits & Data2::DATA_2_MASK) >> Data2::DATA_2_OFFSET
    }

    /// Set **DATA_2** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    pub const fn with_data_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data2::DATA_2_MASK)
                | ((value << Data2::DATA_2_OFFSET) & Data2::DATA_2_MASK),
        }
    }
}

impl Default for Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data2Val {
        Data2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data2Val) -> Data2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_3 Register
///
/// Data register 3.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **DATA_3** `[7:0]` (RW)
///
///   In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
#[repr(transparent)]
pub struct Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data3 {
    // ======== Field Constants ========

    /// **DATA_3**
    ///
    /// In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_3_OFFSET: u32 = 0;
    pub const DATA_3_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_3** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
    #[inline(always)]
    pub fn data_3(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_3_MASK) >> Self::DATA_3_OFFSET
    }

    /// Write **DATA_3** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
    #[inline(always)]
    pub fn write_data_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_3_MASK) | ((value << Self::DATA_3_OFFSET) & Self::DATA_3_MASK)
        });
    }
}

/// DATA_3 value type for batch read/modify/write operations
///
/// Use `data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data3Val {
    bits: u32,
}

impl Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_3** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
    #[inline(always)]
    pub const fn data_3(&self) -> u32 {
        (self.bits & Data3::DATA_3_MASK) >> Data3::DATA_3_OFFSET
    }

    /// Set **DATA_3** `[7:0]`
    ///
    /// In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3.
    #[inline(always)]
    pub const fn with_data_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data3::DATA_3_MASK)
                | ((value << Data3::DATA_3_OFFSET) & Data3::DATA_3_MASK),
        }
    }
}

impl Default for Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data3Val {
        Data3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data3Val) -> Data3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_4 Register
///
/// Data register 4.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **DATA_4** `[7:0]` (RW)
///
///   In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
#[repr(transparent)]
pub struct Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data4 {
    // ======== Field Constants ========

    /// **DATA_4**
    ///
    /// In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_4_OFFSET: u32 = 0;
    pub const DATA_4_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_4** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
    #[inline(always)]
    pub fn data_4(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_4_MASK) >> Self::DATA_4_OFFSET
    }

    /// Write **DATA_4** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
    #[inline(always)]
    pub fn write_data_4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_4_MASK) | ((value << Self::DATA_4_OFFSET) & Self::DATA_4_MASK)
        });
    }
}

/// DATA_4 value type for batch read/modify/write operations
///
/// Use `data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data4Val {
    bits: u32,
}

impl Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_4** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
    #[inline(always)]
    pub const fn data_4(&self) -> u32 {
        (self.bits & Data4::DATA_4_MASK) >> Data4::DATA_4_OFFSET
    }

    /// Set **DATA_4** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4.
    #[inline(always)]
    pub const fn with_data_4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data4::DATA_4_MASK)
                | ((value << Data4::DATA_4_OFFSET) & Data4::DATA_4_MASK),
        }
    }
}

impl Default for Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data4Val {
        Data4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data4Val) -> Data4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_5 Register
///
/// Data register 5.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **DATA_5** `[7:0]` (RW)
///
///   In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
#[repr(transparent)]
pub struct Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data5 {
    // ======== Field Constants ========

    /// **DATA_5**
    ///
    /// In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_5_OFFSET: u32 = 0;
    pub const DATA_5_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_5** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
    #[inline(always)]
    pub fn data_5(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_5_MASK) >> Self::DATA_5_OFFSET
    }

    /// Write **DATA_5** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
    #[inline(always)]
    pub fn write_data_5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_5_MASK) | ((value << Self::DATA_5_OFFSET) & Self::DATA_5_MASK)
        });
    }
}

/// DATA_5 value type for batch read/modify/write operations
///
/// Use `data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data5Val {
    bits: u32,
}

impl Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_5** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
    #[inline(always)]
    pub const fn data_5(&self) -> u32 {
        (self.bits & Data5::DATA_5_MASK) >> Data5::DATA_5_OFFSET
    }

    /// Set **DATA_5** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5.
    #[inline(always)]
    pub const fn with_data_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data5::DATA_5_MASK)
                | ((value << Data5::DATA_5_OFFSET) & Data5::DATA_5_MASK),
        }
    }
}

impl Default for Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data5Val {
        Data5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data5Val) -> Data5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_6 Register
///
/// Data register 6.
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **DATA_6** `[7:0]` (RW)
///
///   In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
#[repr(transparent)]
pub struct Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data6 {
    // ======== Field Constants ========

    /// **DATA_6**
    ///
    /// In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_6_OFFSET: u32 = 0;
    pub const DATA_6_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_6** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    pub fn data_6(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_6_MASK) >> Self::DATA_6_OFFSET
    }

    /// Write **DATA_6** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    pub fn write_data_6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_6_MASK) | ((value << Self::DATA_6_OFFSET) & Self::DATA_6_MASK)
        });
    }
}

/// DATA_6 value type for batch read/modify/write operations
///
/// Use `data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data6Val {
    bits: u32,
}

impl Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_6** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    pub const fn data_6(&self) -> u32 {
        (self.bits & Data6::DATA_6_MASK) >> Data6::DATA_6_OFFSET
    }

    /// Set **DATA_6** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    pub const fn with_data_6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data6::DATA_6_MASK)
                | ((value << Data6::DATA_6_OFFSET) & Data6::DATA_6_MASK),
        }
    }
}

impl Default for Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data6Val {
        Data6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data6Val) -> Data6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_7 Register
///
/// Data register 7.
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **DATA_7** `[7:0]` (RW)
///
///   In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
#[repr(transparent)]
pub struct Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data7 {
    // ======== Field Constants ========

    /// **DATA_7**
    ///
    /// In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_7_OFFSET: u32 = 0;
    pub const DATA_7_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_7** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
    #[inline(always)]
    pub fn data_7(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_7_MASK) >> Self::DATA_7_OFFSET
    }

    /// Write **DATA_7** `[7:0]` (RW)
    ///
    /// In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
    #[inline(always)]
    pub fn write_data_7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_7_MASK) | ((value << Self::DATA_7_OFFSET) & Self::DATA_7_MASK)
        });
    }
}

/// DATA_7 value type for batch read/modify/write operations
///
/// Use `data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data7Val {
    bits: u32,
}

impl Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_7** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
    #[inline(always)]
    pub const fn data_7(&self) -> u32 {
        (self.bits & Data7::DATA_7_MASK) >> Data7::DATA_7_OFFSET
    }

    /// Set **DATA_7** `[7:0]`
    ///
    /// In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7.
    #[inline(always)]
    pub const fn with_data_7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data7::DATA_7_MASK)
                | ((value << Data7::DATA_7_OFFSET) & Data7::DATA_7_MASK),
        }
    }
}

impl Default for Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data7Val {
        Data7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data7Val) -> Data7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_8 Register
///
/// Data register 8.
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **DATA_8** `[7:0]` (RW)
///
///   In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
#[repr(transparent)]
pub struct Data8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data8 {
    // ======== Field Constants ========

    /// **DATA_8**
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_8_OFFSET: u32 = 0;
    pub const DATA_8_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_8** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
    #[inline(always)]
    pub fn data_8(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_8_MASK) >> Self::DATA_8_OFFSET
    }

    /// Write **DATA_8** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
    #[inline(always)]
    pub fn write_data_8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_8_MASK) | ((value << Self::DATA_8_OFFSET) & Self::DATA_8_MASK)
        });
    }
}

/// DATA_8 value type for batch read/modify/write operations
///
/// Use `data8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data8Val {
    bits: u32,
}

impl Data8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_8** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
    #[inline(always)]
    pub const fn data_8(&self) -> u32 {
        (self.bits & Data8::DATA_8_MASK) >> Data8::DATA_8_OFFSET
    }

    /// Set **DATA_8** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8.
    #[inline(always)]
    pub const fn with_data_8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data8::DATA_8_MASK)
                | ((value << Data8::DATA_8_OFFSET) & Data8::DATA_8_MASK),
        }
    }
}

impl Default for Data8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data8Val {
        Data8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data8Val) -> Data8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_9 Register
///
/// Data register 9.
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **DATA_9** `[7:0]` (RW)
///
///   In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
#[repr(transparent)]
pub struct Data9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data9 {
    // ======== Field Constants ========

    /// **DATA_9**
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_9_OFFSET: u32 = 0;
    pub const DATA_9_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_9** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
    #[inline(always)]
    pub fn data_9(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_9_MASK) >> Self::DATA_9_OFFSET
    }

    /// Write **DATA_9** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
    #[inline(always)]
    pub fn write_data_9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_9_MASK) | ((value << Self::DATA_9_OFFSET) & Self::DATA_9_MASK)
        });
    }
}

/// DATA_9 value type for batch read/modify/write operations
///
/// Use `data9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data9Val {
    bits: u32,
}

impl Data9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_9** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
    #[inline(always)]
    pub const fn data_9(&self) -> u32 {
        (self.bits & Data9::DATA_9_MASK) >> Data9::DATA_9_OFFSET
    }

    /// Set **DATA_9** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9.
    #[inline(always)]
    pub const fn with_data_9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data9::DATA_9_MASK)
                | ((value << Data9::DATA_9_OFFSET) & Data9::DATA_9_MASK),
        }
    }
}

impl Default for Data9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data9Val {
        Data9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data9Val) -> Data9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_10 Register
///
/// Data register 10.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **DATA_10** `[7:0]` (RW)
///
///   In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
#[repr(transparent)]
pub struct Data10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data10 {
    // ======== Field Constants ========

    /// **DATA_10**
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_10_OFFSET: u32 = 0;
    pub const DATA_10_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_10** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
    #[inline(always)]
    pub fn data_10(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_10_MASK) >> Self::DATA_10_OFFSET
    }

    /// Write **DATA_10** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
    #[inline(always)]
    pub fn write_data_10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_10_MASK) | ((value << Self::DATA_10_OFFSET) & Self::DATA_10_MASK)
        });
    }
}

/// DATA_10 value type for batch read/modify/write operations
///
/// Use `data10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data10Val {
    bits: u32,
}

impl Data10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_10** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
    #[inline(always)]
    pub const fn data_10(&self) -> u32 {
        (self.bits & Data10::DATA_10_MASK) >> Data10::DATA_10_OFFSET
    }

    /// Set **DATA_10** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10.
    #[inline(always)]
    pub const fn with_data_10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data10::DATA_10_MASK)
                | ((value << Data10::DATA_10_OFFSET) & Data10::DATA_10_MASK),
        }
    }
}

impl Default for Data10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data10Val {
        Data10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data10Val) -> Data10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_11 Register
///
/// Data register 11.
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **DATA_11** `[7:0]` (RW)
///
///   In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
#[repr(transparent)]
pub struct Data11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data11 {
    // ======== Field Constants ========

    /// **DATA_11**
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_11_OFFSET: u32 = 0;
    pub const DATA_11_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_11** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
    #[inline(always)]
    pub fn data_11(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_11_MASK) >> Self::DATA_11_OFFSET
    }

    /// Write **DATA_11** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
    #[inline(always)]
    pub fn write_data_11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_11_MASK) | ((value << Self::DATA_11_OFFSET) & Self::DATA_11_MASK)
        });
    }
}

/// DATA_11 value type for batch read/modify/write operations
///
/// Use `data11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data11Val {
    bits: u32,
}

impl Data11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_11** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
    #[inline(always)]
    pub const fn data_11(&self) -> u32 {
        (self.bits & Data11::DATA_11_MASK) >> Data11::DATA_11_OFFSET
    }

    /// Set **DATA_11** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 11 and when software initiate read operation, it is rx data register 11.
    #[inline(always)]
    pub const fn with_data_11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data11::DATA_11_MASK)
                | ((value << Data11::DATA_11_OFFSET) & Data11::DATA_11_MASK),
        }
    }
}

impl Default for Data11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data11Val {
        Data11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data11Val) -> Data11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_12 Register
///
/// Data register 12.
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **DATA_12** `[7:0]` (RW)
///
///   In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
#[repr(transparent)]
pub struct Data12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data12 {
    // ======== Field Constants ========

    /// **DATA_12**
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_12_OFFSET: u32 = 0;
    pub const DATA_12_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_12** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    pub fn data_12(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_12_MASK) >> Self::DATA_12_OFFSET
    }

    /// Write **DATA_12** `[7:0]` (RW)
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    pub fn write_data_12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_12_MASK) | ((value << Self::DATA_12_OFFSET) & Self::DATA_12_MASK)
        });
    }
}

/// DATA_12 value type for batch read/modify/write operations
///
/// Use `data12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Data12Val {
    bits: u32,
}

impl Data12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_12** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    pub const fn data_12(&self) -> u32 {
        (self.bits & Data12::DATA_12_MASK) >> Data12::DATA_12_OFFSET
    }

    /// Set **DATA_12** `[7:0]`
    ///
    /// In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    pub const fn with_data_12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data12::DATA_12_MASK)
                | ((value << Data12::DATA_12_OFFSET) & Data12::DATA_12_MASK),
        }
    }
}

impl Default for Data12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Data12Val {
        Data12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Data12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Data12Val) -> Data12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RX_MESSAGE_COUNTER Register
///
/// Received message counter register.
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RX_MESSAGE_COUNTER** `[6:0]` (RO)
///
///   Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
#[repr(transparent)]
pub struct RxMessageCounter {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxMessageCounter {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxMessageCounter {
    // ======== Field Constants ========

    /// **RX_MESSAGE_COUNTER**
    ///
    /// Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const RX_MESSAGE_COUNTER_OFFSET: u32 = 0;
    pub const RX_MESSAGE_COUNTER_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **RX_MESSAGE_COUNTER** `[6:0]` (RO)
    ///
    /// Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
    #[inline(always)]
    pub fn rx_message_counter(&self) -> u32 {
        (self.value.read_bits() & Self::RX_MESSAGE_COUNTER_MASK) >> Self::RX_MESSAGE_COUNTER_OFFSET
    }
}

/// RX_MESSAGE_COUNTER value type for batch read/modify/write operations
///
/// Use `rxmessagecounter.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxmessagecounter.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxMessageCounterVal {
    bits: u32,
}

impl RxMessageCounterVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RX_MESSAGE_COUNTER** `[6:0]`
    ///
    /// Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
    #[inline(always)]
    pub const fn rx_message_counter(&self) -> u32 {
        (self.bits & RxMessageCounter::RX_MESSAGE_COUNTER_MASK)
            >> RxMessageCounter::RX_MESSAGE_COUNTER_OFFSET
    }
}

impl Default for RxMessageCounterVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxMessageCounter {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxMessageCounterVal {
        RxMessageCounterVal::from_bits(self.value.read_bits())
    }
}

/// CLOCK_DIVIDER Register
///
/// Clock divider register.
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **CLOCK_OFF** `[8]` (RW)
///
///   1: Disable the external CLKOUT pin. 0: Enable the external CLKOUT pin. Software has R/W permission in reset mode and RO in operation mode.
///
/// - **CD** `[7:0]` (RW)
///
///   These bits are used to define the frequency at the external CLKOUT pin.
#[repr(transparent)]
pub struct ClockDivider {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClockDivider {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClockDivider {
    // ======== Field Constants ========

    /// **CLOCK_OFF**
    ///
    /// 1: Disable the external CLKOUT pin. 0: Enable the external CLKOUT pin. Software has R/W permission in reset mode and RO in operation mode.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLOCK_OFF_OFFSET: u32 = 8;
    pub const CLOCK_OFF_MASK: u32 = 0x00000100;
    pub const CLOCK_OFF: u32 = 1 << 8;

    /// **CD**
    ///
    /// These bits are used to define the frequency at the external CLKOUT pin.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CD_OFFSET: u32 = 0;
    pub const CD_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CLOCK_OFF** `[8]` (RW)
    ///
    /// 1: Disable the external CLKOUT pin. 0: Enable the external CLKOUT pin. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub fn clock_off(&self) -> bool {
        (self.value.read_bits() & Self::CLOCK_OFF_MASK) != 0
    }

    /// Set CLOCK_OFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clock_off(&self) {
        self.value.modify_bits(|r| r | Self::CLOCK_OFF_MASK);
    }

    /// Clear CLOCK_OFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clock_off(&self) {
        self.value.modify_bits(|r| r & !Self::CLOCK_OFF_MASK);
    }

    /// **CD** `[7:0]` (RW)
    ///
    /// These bits are used to define the frequency at the external CLKOUT pin.
    #[inline(always)]
    pub fn cd(&self) -> u32 {
        (self.value.read_bits() & Self::CD_MASK) >> Self::CD_OFFSET
    }

    /// Write **CD** `[7:0]` (RW)
    ///
    /// These bits are used to define the frequency at the external CLKOUT pin.
    #[inline(always)]
    pub fn write_cd(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::CD_MASK) | ((value << Self::CD_OFFSET) & Self::CD_MASK));
    }
}

/// CLOCK_DIVIDER value type for batch read/modify/write operations
///
/// Use `clockdivider.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clockdivider.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClockDividerVal {
    bits: u32,
}

impl ClockDividerVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLOCK_OFF** `[8:8]`
    ///
    /// 1: Disable the external CLKOUT pin. 0: Enable the external CLKOUT pin. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn clock_off(&self) -> bool {
        (self.bits & ClockDivider::CLOCK_OFF_MASK) != 0
    }

    /// Read **CD** `[7:0]`
    ///
    /// These bits are used to define the frequency at the external CLKOUT pin.
    #[inline(always)]
    pub const fn cd(&self) -> u32 {
        (self.bits & ClockDivider::CD_MASK) >> ClockDivider::CD_OFFSET
    }

    /// Set **CLOCK_OFF** `[8:8]`
    ///
    /// 1: Disable the external CLKOUT pin. 0: Enable the external CLKOUT pin. Software has R/W permission in reset mode and RO in operation mode.
    #[inline(always)]
    pub const fn with_clock_off(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClockDivider::CLOCK_OFF_MASK
            } else {
                self.bits & !ClockDivider::CLOCK_OFF_MASK
            },
        }
    }

    /// Set **CD** `[7:0]`
    ///
    /// These bits are used to define the frequency at the external CLKOUT pin.
    #[inline(always)]
    pub const fn with_cd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClockDivider::CD_MASK)
                | ((value << ClockDivider::CD_OFFSET) & ClockDivider::CD_MASK),
        }
    }
}

impl Default for ClockDividerVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClockDivider {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClockDividerVal {
        ClockDividerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClockDividerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClockDividerVal) -> ClockDividerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SW_STANDBY_CFG Register
///
/// Software configure standby pin directly.
///
/// Address offset: 0x0080
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **SW_STANDBY_CLR** `[1]` (RW)
///
///   Clear standby pin.
///
/// - **SW_STANDBY_EN** `[0]` (RW)
///
///   Enable standby pin.
#[repr(transparent)]
pub struct SwStandbyCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwStandbyCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwStandbyCfg {
    // ======== Field Constants ========

    /// **SW_STANDBY_CLR**
    ///
    /// Clear standby pin.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SW_STANDBY_CLR_OFFSET: u32 = 1;
    pub const SW_STANDBY_CLR_MASK: u32 = 0x00000002;
    pub const SW_STANDBY_CLR: u32 = 1 << 1;

    /// **SW_STANDBY_EN**
    ///
    /// Enable standby pin.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SW_STANDBY_EN_OFFSET: u32 = 0;
    pub const SW_STANDBY_EN_MASK: u32 = 0x00000001;
    pub const SW_STANDBY_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SW_STANDBY_CLR** `[1]` (RW)
    ///
    /// Clear standby pin.
    #[inline(always)]
    pub fn sw_standby_clr(&self) -> bool {
        (self.value.read_bits() & Self::SW_STANDBY_CLR_MASK) != 0
    }

    /// Set SW_STANDBY_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_standby_clr(&self) {
        self.value.modify_bits(|r| r | Self::SW_STANDBY_CLR_MASK);
    }

    /// Clear SW_STANDBY_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_standby_clr(&self) {
        self.value.modify_bits(|r| r & !Self::SW_STANDBY_CLR_MASK);
    }

    /// **SW_STANDBY_EN** `[0]` (RW)
    ///
    /// Enable standby pin.
    #[inline(always)]
    pub fn sw_standby_en(&self) -> bool {
        (self.value.read_bits() & Self::SW_STANDBY_EN_MASK) != 0
    }

    /// Set SW_STANDBY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_standby_en(&self) {
        self.value.modify_bits(|r| r | Self::SW_STANDBY_EN_MASK);
    }

    /// Clear SW_STANDBY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_standby_en(&self) {
        self.value.modify_bits(|r| r & !Self::SW_STANDBY_EN_MASK);
    }
}

/// SW_STANDBY_CFG value type for batch read/modify/write operations
///
/// Use `swstandbycfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swstandbycfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwStandbyCfgVal {
    bits: u32,
}

impl SwStandbyCfgVal {
    /// Create from raw bits
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

    /// Read **SW_STANDBY_CLR** `[1:1]`
    ///
    /// Clear standby pin.
    #[inline(always)]
    pub const fn sw_standby_clr(&self) -> bool {
        (self.bits & SwStandbyCfg::SW_STANDBY_CLR_MASK) != 0
    }

    /// Read **SW_STANDBY_EN** `[0:0]`
    ///
    /// Enable standby pin.
    #[inline(always)]
    pub const fn sw_standby_en(&self) -> bool {
        (self.bits & SwStandbyCfg::SW_STANDBY_EN_MASK) != 0
    }

    /// Set **SW_STANDBY_CLR** `[1:1]`
    ///
    /// Clear standby pin.
    #[inline(always)]
    pub const fn with_sw_standby_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwStandbyCfg::SW_STANDBY_CLR_MASK
            } else {
                self.bits & !SwStandbyCfg::SW_STANDBY_CLR_MASK
            },
        }
    }

    /// Set **SW_STANDBY_EN** `[0:0]`
    ///
    /// Enable standby pin.
    #[inline(always)]
    pub const fn with_sw_standby_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwStandbyCfg::SW_STANDBY_EN_MASK
            } else {
                self.bits & !SwStandbyCfg::SW_STANDBY_EN_MASK
            },
        }
    }
}

impl Default for SwStandbyCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwStandbyCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwStandbyCfgVal {
        SwStandbyCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwStandbyCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwStandbyCfgVal) -> SwStandbyCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HW_CFG Register
///
/// Hardware configure standby pin.
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **HW_STANDBY_EN** `[0]` (RW)
///
///   Enable function that hardware control standby pin.
#[repr(transparent)]
pub struct HwCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HwCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HwCfg {
    // ======== Field Constants ========

    /// **HW_STANDBY_EN**
    ///
    /// Enable function that hardware control standby pin.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HW_STANDBY_EN_OFFSET: u32 = 0;
    pub const HW_STANDBY_EN_MASK: u32 = 0x00000001;
    pub const HW_STANDBY_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HW_STANDBY_EN** `[0]` (RW)
    ///
    /// Enable function that hardware control standby pin.
    #[inline(always)]
    pub fn hw_standby_en(&self) -> bool {
        (self.value.read_bits() & Self::HW_STANDBY_EN_MASK) != 0
    }

    /// Set HW_STANDBY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hw_standby_en(&self) {
        self.value.modify_bits(|r| r | Self::HW_STANDBY_EN_MASK);
    }

    /// Clear HW_STANDBY_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hw_standby_en(&self) {
        self.value.modify_bits(|r| r & !Self::HW_STANDBY_EN_MASK);
    }
}

/// HW_CFG value type for batch read/modify/write operations
///
/// Use `hwcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hwcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HwCfgVal {
    bits: u32,
}

impl HwCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HW_STANDBY_EN** `[0:0]`
    ///
    /// Enable function that hardware control standby pin.
    #[inline(always)]
    pub const fn hw_standby_en(&self) -> bool {
        (self.bits & HwCfg::HW_STANDBY_EN_MASK) != 0
    }

    /// Set **HW_STANDBY_EN** `[0:0]`
    ///
    /// Enable function that hardware control standby pin.
    #[inline(always)]
    pub const fn with_hw_standby_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HwCfg::HW_STANDBY_EN_MASK
            } else {
                self.bits & !HwCfg::HW_STANDBY_EN_MASK
            },
        }
    }
}

impl Default for HwCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HwCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HwCfgVal {
        HwCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HwCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HwCfgVal) -> HwCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HW_STANDBY_CNT Register
///
/// Configure standby counter.
///
/// Address offset: 0x0088
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **STANDBY_WAIT_CNT** `[31:0]` (RW)
///
///   Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
#[repr(transparent)]
pub struct HwStandbyCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HwStandbyCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HwStandbyCnt {
    // ======== Field Constants ========

    /// **STANDBY_WAIT_CNT**
    ///
    /// Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const STANDBY_WAIT_CNT_OFFSET: u32 = 0;
    pub const STANDBY_WAIT_CNT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **STANDBY_WAIT_CNT** `[31:0]` (RW)
    ///
    /// Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    pub fn standby_wait_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::STANDBY_WAIT_CNT_MASK) >> Self::STANDBY_WAIT_CNT_OFFSET
    }

    /// Write **STANDBY_WAIT_CNT** `[31:0]` (RW)
    ///
    /// Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    pub fn write_standby_wait_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::STANDBY_WAIT_CNT_MASK)
                | ((value << Self::STANDBY_WAIT_CNT_OFFSET) & Self::STANDBY_WAIT_CNT_MASK)
        });
    }
}

/// HW_STANDBY_CNT value type for batch read/modify/write operations
///
/// Use `hwstandbycnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hwstandbycnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HwStandbyCntVal {
    bits: u32,
}

impl HwStandbyCntVal {
    /// Create from raw bits
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

    /// Read **STANDBY_WAIT_CNT** `[31:0]`
    ///
    /// Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    pub const fn standby_wait_cnt(&self) -> u32 {
        (self.bits & HwStandbyCnt::STANDBY_WAIT_CNT_MASK) >> HwStandbyCnt::STANDBY_WAIT_CNT_OFFSET
    }

    /// Set **STANDBY_WAIT_CNT** `[31:0]`
    ///
    /// Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    pub const fn with_standby_wait_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HwStandbyCnt::STANDBY_WAIT_CNT_MASK)
                | ((value << HwStandbyCnt::STANDBY_WAIT_CNT_OFFSET)
                    & HwStandbyCnt::STANDBY_WAIT_CNT_MASK),
        }
    }
}

impl Default for HwStandbyCntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HwStandbyCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HwStandbyCntVal {
        HwStandbyCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HwStandbyCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HwStandbyCntVal) -> HwStandbyCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDLE_INTR_CNT Register
///
/// Configure idle interrupt counter.
///
/// Address offset: 0x008c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **IDLE_INTR_CNT** `[31:0]` (RW)
///
///   Configure the number of cycles before triggering idle interrupt.
#[repr(transparent)]
pub struct IdleIntrCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IdleIntrCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IdleIntrCnt {
    // ======== Field Constants ========

    /// **IDLE_INTR_CNT**
    ///
    /// Configure the number of cycles before triggering idle interrupt.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IDLE_INTR_CNT_OFFSET: u32 = 0;
    pub const IDLE_INTR_CNT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IDLE_INTR_CNT** `[31:0]` (RW)
    ///
    /// Configure the number of cycles before triggering idle interrupt.
    #[inline(always)]
    pub fn idle_intr_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::IDLE_INTR_CNT_MASK) >> Self::IDLE_INTR_CNT_OFFSET
    }

    /// Write **IDLE_INTR_CNT** `[31:0]` (RW)
    ///
    /// Configure the number of cycles before triggering idle interrupt.
    #[inline(always)]
    pub fn write_idle_intr_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDLE_INTR_CNT_MASK)
                | ((value << Self::IDLE_INTR_CNT_OFFSET) & Self::IDLE_INTR_CNT_MASK)
        });
    }
}

/// IDLE_INTR_CNT value type for batch read/modify/write operations
///
/// Use `idleintrcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idleintrcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdleIntrCntVal {
    bits: u32,
}

impl IdleIntrCntVal {
    /// Create from raw bits
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

    /// Read **IDLE_INTR_CNT** `[31:0]`
    ///
    /// Configure the number of cycles before triggering idle interrupt.
    #[inline(always)]
    pub const fn idle_intr_cnt(&self) -> u32 {
        (self.bits & IdleIntrCnt::IDLE_INTR_CNT_MASK) >> IdleIntrCnt::IDLE_INTR_CNT_OFFSET
    }

    /// Set **IDLE_INTR_CNT** `[31:0]`
    ///
    /// Configure the number of cycles before triggering idle interrupt.
    #[inline(always)]
    pub const fn with_idle_intr_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IdleIntrCnt::IDLE_INTR_CNT_MASK)
                | ((value << IdleIntrCnt::IDLE_INTR_CNT_OFFSET) & IdleIntrCnt::IDLE_INTR_CNT_MASK),
        }
    }
}

impl Default for IdleIntrCntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IdleIntrCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdleIntrCntVal {
        IdleIntrCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdleIntrCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdleIntrCntVal) -> IdleIntrCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_CFG Register
///
/// ECO configuration register.
///
/// Address offset: 0x0090
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **RDN_RESULT** `[1]` (RO)
///
///   Output of eco module.
///
/// - **RDN_ENA** `[0]` (RW)
///
///   Enable eco module.
#[repr(transparent)]
pub struct EcoCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoCfg {
    // ======== Field Constants ========

    /// **RDN_RESULT**
    ///
    /// Output of eco module.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDN_RESULT_OFFSET: u32 = 1;
    pub const RDN_RESULT_MASK: u32 = 0x00000002;
    pub const RDN_RESULT: u32 = 1 << 1;

    /// **RDN_ENA**
    ///
    /// Enable eco module.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDN_ENA_OFFSET: u32 = 0;
    pub const RDN_ENA_MASK: u32 = 0x00000001;
    pub const RDN_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RDN_RESULT** `[1]` (RO)
    ///
    /// Output of eco module.
    #[inline(always)]
    pub fn rdn_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_RESULT_MASK) != 0
    }

    /// **RDN_ENA** `[0]` (RW)
    ///
    /// Enable eco module.
    #[inline(always)]
    pub fn rdn_ena(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ENA_MASK) != 0
    }

    /// Set RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rdn_ena(&self) {
        self.value.modify_bits(|r| r | Self::RDN_ENA_MASK);
    }

    /// Clear RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rdn_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RDN_ENA_MASK);
    }
}

/// ECO_CFG value type for batch read/modify/write operations
///
/// Use `ecocfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecocfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoCfgVal {
    bits: u32,
}

impl EcoCfgVal {
    /// Create from raw bits
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

    /// Read **RDN_RESULT** `[1:1]`
    ///
    /// Output of eco module.
    #[inline(always)]
    pub const fn rdn_result(&self) -> bool {
        (self.bits & EcoCfg::RDN_RESULT_MASK) != 0
    }

    /// Read **RDN_ENA** `[0:0]`
    ///
    /// Enable eco module.
    #[inline(always)]
    pub const fn rdn_ena(&self) -> bool {
        (self.bits & EcoCfg::RDN_ENA_MASK) != 0
    }

    /// Set **RDN_ENA** `[0:0]`
    ///
    /// Enable eco module.
    #[inline(always)]
    pub const fn with_rdn_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EcoCfg::RDN_ENA_MASK
            } else {
                self.bits & !EcoCfg::RDN_ENA_MASK
            },
        }
    }
}

impl Default for EcoCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EcoCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoCfgVal {
        EcoCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoCfgVal) -> EcoCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMESTAMP_DATA Register
///
/// Timestamp data register
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **TIMESTAMP_DATA** `[31:0]` (RO)
///
///   Data of timestamp of a CAN frame.
#[repr(transparent)]
pub struct TimestampData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TimestampData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TimestampData {
    // ======== Field Constants ========

    /// **TIMESTAMP_DATA**
    ///
    /// Data of timestamp of a CAN frame.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMESTAMP_DATA_OFFSET: u32 = 0;
    pub const TIMESTAMP_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMESTAMP_DATA** `[31:0]` (RO)
    ///
    /// Data of timestamp of a CAN frame.
    #[inline(always)]
    pub fn timestamp_data(&self) -> u32 {
        (self.value.read_bits() & Self::TIMESTAMP_DATA_MASK) >> Self::TIMESTAMP_DATA_OFFSET
    }
}

/// TIMESTAMP_DATA value type for batch read/modify/write operations
///
/// Use `timestampdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timestampdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimestampDataVal {
    bits: u32,
}

impl TimestampDataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMESTAMP_DATA** `[31:0]`
    ///
    /// Data of timestamp of a CAN frame.
    #[inline(always)]
    pub const fn timestamp_data(&self) -> u32 {
        (self.bits & TimestampData::TIMESTAMP_DATA_MASK) >> TimestampData::TIMESTAMP_DATA_OFFSET
    }
}

impl Default for TimestampDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TimestampData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimestampDataVal {
        TimestampDataVal::from_bits(self.value.read_bits())
    }
}

/// TIMESTAMP_PRESCALER Register
///
/// Timestamp configuration register
///
/// Address offset: 0x0098
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **TS_DIV_NUM** `[15:0]` (RW)
///
///   Configures the clock division number of timestamp counter.
#[repr(transparent)]
pub struct TimestampPrescaler {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TimestampPrescaler {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TimestampPrescaler {
    // ======== Field Constants ========

    /// **TS_DIV_NUM**
    ///
    /// Configures the clock division number of timestamp counter.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const TS_DIV_NUM_OFFSET: u32 = 0;
    pub const TS_DIV_NUM_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **TS_DIV_NUM** `[15:0]` (RW)
    ///
    /// Configures the clock division number of timestamp counter.
    #[inline(always)]
    pub fn ts_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::TS_DIV_NUM_MASK) >> Self::TS_DIV_NUM_OFFSET
    }

    /// Write **TS_DIV_NUM** `[15:0]` (RW)
    ///
    /// Configures the clock division number of timestamp counter.
    #[inline(always)]
    pub fn write_ts_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TS_DIV_NUM_MASK)
                | ((value << Self::TS_DIV_NUM_OFFSET) & Self::TS_DIV_NUM_MASK)
        });
    }
}

/// TIMESTAMP_PRESCALER value type for batch read/modify/write operations
///
/// Use `timestampprescaler.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timestampprescaler.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimestampPrescalerVal {
    bits: u32,
}

impl TimestampPrescalerVal {
    /// Create from raw bits
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
        Self { bits: 0x0000001f }
    }

    /// Read **TS_DIV_NUM** `[15:0]`
    ///
    /// Configures the clock division number of timestamp counter.
    #[inline(always)]
    pub const fn ts_div_num(&self) -> u32 {
        (self.bits & TimestampPrescaler::TS_DIV_NUM_MASK) >> TimestampPrescaler::TS_DIV_NUM_OFFSET
    }

    /// Set **TS_DIV_NUM** `[15:0]`
    ///
    /// Configures the clock division number of timestamp counter.
    #[inline(always)]
    pub const fn with_ts_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TimestampPrescaler::TS_DIV_NUM_MASK)
                | ((value << TimestampPrescaler::TS_DIV_NUM_OFFSET)
                    & TimestampPrescaler::TS_DIV_NUM_MASK),
        }
    }
}

impl Default for TimestampPrescalerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TimestampPrescaler {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimestampPrescalerVal {
        TimestampPrescalerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimestampPrescalerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TimestampPrescalerVal) -> TimestampPrescalerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMESTAMP_CFG Register
///
/// Timestamp configuration register
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **TS_ENABLE** `[0]` (RW)
///
///   enable the timestamp collection function.
#[repr(transparent)]
pub struct TimestampCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TimestampCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TimestampCfg {
    // ======== Field Constants ========

    /// **TS_ENABLE**
    ///
    /// enable the timestamp collection function.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TS_ENABLE_OFFSET: u32 = 0;
    pub const TS_ENABLE_MASK: u32 = 0x00000001;
    pub const TS_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TS_ENABLE** `[0]` (RW)
    ///
    /// enable the timestamp collection function.
    #[inline(always)]
    pub fn ts_enable(&self) -> bool {
        (self.value.read_bits() & Self::TS_ENABLE_MASK) != 0
    }

    /// Set TS_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ts_enable(&self) {
        self.value.modify_bits(|r| r | Self::TS_ENABLE_MASK);
    }

    /// Clear TS_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ts_enable(&self) {
        self.value.modify_bits(|r| r & !Self::TS_ENABLE_MASK);
    }
}

/// TIMESTAMP_CFG value type for batch read/modify/write operations
///
/// Use `timestampcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timestampcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimestampCfgVal {
    bits: u32,
}

impl TimestampCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TS_ENABLE** `[0:0]`
    ///
    /// enable the timestamp collection function.
    #[inline(always)]
    pub const fn ts_enable(&self) -> bool {
        (self.bits & TimestampCfg::TS_ENABLE_MASK) != 0
    }

    /// Set **TS_ENABLE** `[0:0]`
    ///
    /// enable the timestamp collection function.
    #[inline(always)]
    pub const fn with_ts_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TimestampCfg::TS_ENABLE_MASK
            } else {
                self.bits & !TimestampCfg::TS_ENABLE_MASK
            },
        }
    }
}

impl Default for TimestampCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TimestampCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimestampCfgVal {
        TimestampCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimestampCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TimestampCfgVal) -> TimestampCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// TWAI mode register.
    pub mode: Mode,
    /// 0x0004
    /// TWAI command register.
    pub cmd: Cmd,
    /// 0x0008
    /// TWAI status register.
    pub status: Status,
    /// 0x000c
    /// Interrupt signals' register.
    pub interrupt: Interrupt,
    /// 0x0010
    /// Interrupt enable register.
    pub interrupt_enable: InterruptEnable,
    _reserved0: [u8; 4],
    /// 0x0018
    /// Bit timing configuration register 0.
    pub bus_timing0: BusTiming0,
    /// 0x001c
    /// Bit timing configuration register 1.
    pub bus_timing1: BusTiming1,
    _reserved1: [u8; 12],
    /// 0x002c
    /// TWAI arbiter lost capture register.
    pub arb_lost_cap: ArbLostCap,
    /// 0x0030
    /// TWAI error info capture register.
    pub err_code_cap: ErrCodeCap,
    /// 0x0034
    /// TWAI error threshold configuration register.
    pub err_warning_limit: ErrWarningLimit,
    /// 0x0038
    /// Rx error counter register.
    pub rx_err_cnt: RxErrCnt,
    /// 0x003c
    /// Tx error counter register.
    pub tx_err_cnt: TxErrCnt,
    /// 0x0040
    /// Data register 0.
    pub data0: Data0,
    /// 0x0044
    /// Data register 1.
    pub data1: Data1,
    /// 0x0048
    /// Data register 2.
    pub data2: Data2,
    /// 0x004c
    /// Data register 3.
    pub data3: Data3,
    /// 0x0050
    /// Data register 4.
    pub data4: Data4,
    /// 0x0054
    /// Data register 5.
    pub data5: Data5,
    /// 0x0058
    /// Data register 6.
    pub data6: Data6,
    /// 0x005c
    /// Data register 7.
    pub data7: Data7,
    /// 0x0060
    /// Data register 8.
    pub data8: Data8,
    /// 0x0064
    /// Data register 9.
    pub data9: Data9,
    /// 0x0068
    /// Data register 10.
    pub data10: Data10,
    /// 0x006c
    /// Data register 11.
    pub data11: Data11,
    /// 0x0070
    /// Data register 12.
    pub data12: Data12,
    /// 0x0074
    /// Received message counter register.
    pub rx_message_counter: RxMessageCounter,
    _reserved2: [u8; 4],
    /// 0x007c
    /// Clock divider register.
    pub clock_divider: ClockDivider,
    /// 0x0080
    /// Software configure standby pin directly.
    pub sw_standby_cfg: SwStandbyCfg,
    /// 0x0084
    /// Hardware configure standby pin.
    pub hw_cfg: HwCfg,
    /// 0x0088
    /// Configure standby counter.
    pub hw_standby_cnt: HwStandbyCnt,
    /// 0x008c
    /// Configure idle interrupt counter.
    pub idle_intr_cnt: IdleIntrCnt,
    /// 0x0090
    /// ECO configuration register.
    pub eco_cfg: EcoCfg,
    /// 0x0094
    /// Timestamp data register
    pub timestamp_data: TimestampData,
    /// 0x0098
    /// Timestamp configuration register
    pub timestamp_prescaler: TimestampPrescaler,
    /// 0x009c
    /// Timestamp configuration register
    pub timestamp_cfg: TimestampCfg,
}
