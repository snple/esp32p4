//! I2C0
//! I2C (Inter-Integrated Circuit) Controller 0

#![allow(dead_code)]

pub const BASE: u32 = 0x500c4000;

/// SCL_LOW_PERIOD Register
///
/// Configures the low level width of the SCL Clock.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SCL_LOW_PERIOD** `[8:0]` (RW)
///
///   Configures the low level width of the SCL Clock.
///   Measurement unit: i2c_sclk.
#[repr(transparent)]
pub struct SclLowPeriod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclLowPeriod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclLowPeriod {
    // ======== Field Constants ========

    /// **SCL_LOW_PERIOD**
    ///
    /// Configures the low level width of the SCL Clock.
    /// Measurement unit: i2c_sclk.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const SCL_LOW_PERIOD_OFFSET: u32 = 0;
    pub const SCL_LOW_PERIOD_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **SCL_LOW_PERIOD** `[8:0]` (RW)
    ///
    /// Configures the low level width of the SCL Clock.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn scl_low_period(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_LOW_PERIOD_MASK) >> Self::SCL_LOW_PERIOD_OFFSET
    }

    /// Write **SCL_LOW_PERIOD** `[8:0]` (RW)
    ///
    /// Configures the low level width of the SCL Clock.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn write_scl_low_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_LOW_PERIOD_MASK)
                | ((value << Self::SCL_LOW_PERIOD_OFFSET) & Self::SCL_LOW_PERIOD_MASK)
        });
    }
}

/// SCL_LOW_PERIOD value type for batch read/modify/write operations
///
/// Use `scllowperiod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scllowperiod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclLowPeriodVal {
    bits: u32,
}

impl SclLowPeriodVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SCL_LOW_PERIOD** `[8:0]`
    ///
    /// Configures the low level width of the SCL Clock.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn scl_low_period(&self) -> u32 {
        (self.bits & SclLowPeriod::SCL_LOW_PERIOD_MASK) >> SclLowPeriod::SCL_LOW_PERIOD_OFFSET
    }

    /// Set **SCL_LOW_PERIOD** `[8:0]`
    ///
    /// Configures the low level width of the SCL Clock.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn with_scl_low_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclLowPeriod::SCL_LOW_PERIOD_MASK)
                | ((value << SclLowPeriod::SCL_LOW_PERIOD_OFFSET)
                    & SclLowPeriod::SCL_LOW_PERIOD_MASK),
        }
    }
}

impl Default for SclLowPeriodVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SclLowPeriod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclLowPeriodVal {
        SclLowPeriodVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclLowPeriodVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclLowPeriodVal) -> SclLowPeriodVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTR Register
///
/// Transmission setting
///
/// Address offset: 0x0004
/// Reset value: 0x00000208
///
/// ## Fields
///
/// - **ADDR_BROADCASTING_EN** `[14]` (RW)
///
///   Configures to support the 7bit general call function.
///   0: Not support
///   1: Support
///
/// - **ADDR_10BIT_RW_CHECK_EN** `[13]` (RW)
///
///   Configures to check if the r/w bit of 10bit addressing consists with I2C protocol.
///   0: Not check
///   1: Check
///
/// - **SLV_TX_AUTO_START_EN** `[12]` (RW)
///
///   Configures to enable slave to send data automatically
///   0: Disable
///   1: Enable
///
/// - **CONF_UPGATE** `[11]` (WO)
///
///   Configures this bit for synchronization
///   0: No effect
///   1: Synchronize
///
/// - **FSM_RST** `[10]` (WO)
///
///   Configures to reset the SCL_FSM.
///   0: No effect
///   1: Reset
///
/// - **ARBITRATION_EN** `[9]` (RW)
///
///   Configures to enable I2C bus arbitration detection.
///   0: No effect
///   1: Enable
///
/// - **CLK_EN** `[8]` (RW)
///
///   Configures whether to gate clock signal for registers.
///   0: Force clock on for registers
///   1: Support clock only when registers are read or written to by software.
///
/// - **RX_LSB_FIRST** `[7]` (RW)
///
///   Configures to control the storage order for received data.
///   1: receive data from the least significant bit
///   0: receive data from the most significant bit.
///
/// - **TX_LSB_FIRST** `[6]` (RW)
///
///   Configures to control the sending order for data needing to be sent.
///   1: send data from the least significant bit,
///   0: send data from the most significant bit.
///
/// - **TRANS_START** `[5]` (WO)
///
///   Configures to start sending the data in txfifo for slave.
///   0: No effect
///   1: Start
///
/// - **MS_MODE** `[4]` (RW)
///
///   Configures the module as an I2C Master or Slave.
///   0: Slave
///   1: Master
///
/// - **RX_FULL_ACK_LEVEL** `[3]` (RW)
///
///   Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold.
///
/// - **SAMPLE_SCL_LEVEL** `[2]` (RW)
///
///   Configures the sample mode for SDA.
///   1: Sample SDA data on the SCL low level.
///   0: Sample SDA data on the SCL high level.
///
/// - **SCL_FORCE_OUT** `[1]` (RW)
///
///   Configures the SCL output mode
///   1: Direct output,
///   0: Open drain output.
///
/// - **SDA_FORCE_OUT** `[0]` (RW)
///
///   Configures the SDA output mode
///   1: Direct output,
///   0: Open drain output.
#[repr(transparent)]
pub struct Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctr {
    // ======== Field Constants ========

    /// **ADDR_BROADCASTING_EN**
    ///
    /// Configures to support the 7bit general call function.
    /// 0: Not support
    /// 1: Support
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ADDR_BROADCASTING_EN_OFFSET: u32 = 14;
    pub const ADDR_BROADCASTING_EN_MASK: u32 = 0x00004000;
    pub const ADDR_BROADCASTING_EN: u32 = 1 << 14;

    /// **ADDR_10BIT_RW_CHECK_EN**
    ///
    /// Configures to check if the r/w bit of 10bit addressing consists with I2C protocol.
    /// 0: Not check
    /// 1: Check
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ADDR_10BIT_RW_CHECK_EN_OFFSET: u32 = 13;
    pub const ADDR_10BIT_RW_CHECK_EN_MASK: u32 = 0x00002000;
    pub const ADDR_10BIT_RW_CHECK_EN: u32 = 1 << 13;

    /// **SLV_TX_AUTO_START_EN**
    ///
    /// Configures to enable slave to send data automatically
    /// 0: Disable
    /// 1: Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SLV_TX_AUTO_START_EN_OFFSET: u32 = 12;
    pub const SLV_TX_AUTO_START_EN_MASK: u32 = 0x00001000;
    pub const SLV_TX_AUTO_START_EN: u32 = 1 << 12;

    /// **CONF_UPGATE**
    ///
    /// Configures this bit for synchronization
    /// 0: No effect
    /// 1: Synchronize
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CONF_UPGATE_OFFSET: u32 = 11;
    pub const CONF_UPGATE_MASK: u32 = 0x00000800;
    pub const CONF_UPGATE: u32 = 1 << 11;

    /// **FSM_RST**
    ///
    /// Configures to reset the SCL_FSM.
    /// 0: No effect
    /// 1: Reset
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FSM_RST_OFFSET: u32 = 10;
    pub const FSM_RST_MASK: u32 = 0x00000400;
    pub const FSM_RST: u32 = 1 << 10;

    /// **ARBITRATION_EN**
    ///
    /// Configures to enable I2C bus arbitration detection.
    /// 0: No effect
    /// 1: Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ARBITRATION_EN_OFFSET: u32 = 9;
    pub const ARBITRATION_EN_MASK: u32 = 0x00000200;
    pub const ARBITRATION_EN: u32 = 1 << 9;

    /// **CLK_EN**
    ///
    /// Configures whether to gate clock signal for registers.
    /// 0: Force clock on for registers
    /// 1: Support clock only when registers are read or written to by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_EN_OFFSET: u32 = 8;
    pub const CLK_EN_MASK: u32 = 0x00000100;
    pub const CLK_EN: u32 = 1 << 8;

    /// **RX_LSB_FIRST**
    ///
    /// Configures to control the storage order for received data.
    /// 1: receive data from the least significant bit
    /// 0: receive data from the most significant bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_LSB_FIRST_OFFSET: u32 = 7;
    pub const RX_LSB_FIRST_MASK: u32 = 0x00000080;
    pub const RX_LSB_FIRST: u32 = 1 << 7;

    /// **TX_LSB_FIRST**
    ///
    /// Configures to control the sending order for data needing to be sent.
    /// 1: send data from the least significant bit,
    /// 0: send data from the most significant bit.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_LSB_FIRST_OFFSET: u32 = 6;
    pub const TX_LSB_FIRST_MASK: u32 = 0x00000040;
    pub const TX_LSB_FIRST: u32 = 1 << 6;

    /// **TRANS_START**
    ///
    /// Configures to start sending the data in txfifo for slave.
    /// 0: No effect
    /// 1: Start
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TRANS_START_OFFSET: u32 = 5;
    pub const TRANS_START_MASK: u32 = 0x00000020;
    pub const TRANS_START: u32 = 1 << 5;

    /// **MS_MODE**
    ///
    /// Configures the module as an I2C Master or Slave.
    /// 0: Slave
    /// 1: Master
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MS_MODE_OFFSET: u32 = 4;
    pub const MS_MODE_MASK: u32 = 0x00000010;
    pub const MS_MODE: u32 = 1 << 4;

    /// **RX_FULL_ACK_LEVEL**
    ///
    /// Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_FULL_ACK_LEVEL_OFFSET: u32 = 3;
    pub const RX_FULL_ACK_LEVEL_MASK: u32 = 0x00000008;
    pub const RX_FULL_ACK_LEVEL: u32 = 1 << 3;

    /// **SAMPLE_SCL_LEVEL**
    ///
    /// Configures the sample mode for SDA.
    /// 1: Sample SDA data on the SCL low level.
    /// 0: Sample SDA data on the SCL high level.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SAMPLE_SCL_LEVEL_OFFSET: u32 = 2;
    pub const SAMPLE_SCL_LEVEL_MASK: u32 = 0x00000004;
    pub const SAMPLE_SCL_LEVEL: u32 = 1 << 2;

    /// **SCL_FORCE_OUT**
    ///
    /// Configures the SCL output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCL_FORCE_OUT_OFFSET: u32 = 1;
    pub const SCL_FORCE_OUT_MASK: u32 = 0x00000002;
    pub const SCL_FORCE_OUT: u32 = 1 << 1;

    /// **SDA_FORCE_OUT**
    ///
    /// Configures the SDA output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SDA_FORCE_OUT_OFFSET: u32 = 0;
    pub const SDA_FORCE_OUT_MASK: u32 = 0x00000001;
    pub const SDA_FORCE_OUT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ADDR_BROADCASTING_EN** `[14]` (RW)
    ///
    /// Configures to support the 7bit general call function.
    /// 0: Not support
    /// 1: Support
    #[inline(always)]
    pub fn addr_broadcasting_en(&self) -> bool {
        (self.value.read_bits() & Self::ADDR_BROADCASTING_EN_MASK) != 0
    }

    /// Set ADDR_BROADCASTING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addr_broadcasting_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ADDR_BROADCASTING_EN_MASK);
    }

    /// Clear ADDR_BROADCASTING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addr_broadcasting_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ADDR_BROADCASTING_EN_MASK);
    }

    /// **ADDR_10BIT_RW_CHECK_EN** `[13]` (RW)
    ///
    /// Configures to check if the r/w bit of 10bit addressing consists with I2C protocol.
    /// 0: Not check
    /// 1: Check
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&self) -> bool {
        (self.value.read_bits() & Self::ADDR_10BIT_RW_CHECK_EN_MASK) != 0
    }

    /// Set ADDR_10BIT_RW_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addr_10bit_rw_check_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ADDR_10BIT_RW_CHECK_EN_MASK);
    }

    /// Clear ADDR_10BIT_RW_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addr_10bit_rw_check_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ADDR_10BIT_RW_CHECK_EN_MASK);
    }

    /// **SLV_TX_AUTO_START_EN** `[12]` (RW)
    ///
    /// Configures to enable slave to send data automatically
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&self) -> bool {
        (self.value.read_bits() & Self::SLV_TX_AUTO_START_EN_MASK) != 0
    }

    /// Set SLV_TX_AUTO_START_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slv_tx_auto_start_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SLV_TX_AUTO_START_EN_MASK);
    }

    /// Clear SLV_TX_AUTO_START_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slv_tx_auto_start_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLV_TX_AUTO_START_EN_MASK);
    }

    /// Trigger **CONF_UPGATE** (WO)
    ///
    /// Configures this bit for synchronization
    /// 0: No effect
    /// 1: Synchronize
    #[inline(always)]
    pub fn trigger_conf_upgate(&self) {
        self.value.modify_bits(|r| r | Self::CONF_UPGATE_MASK);
    }

    /// Trigger **FSM_RST** (WO)
    ///
    /// Configures to reset the SCL_FSM.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub fn trigger_fsm_rst(&self) {
        self.value.modify_bits(|r| r | Self::FSM_RST_MASK);
    }

    /// **ARBITRATION_EN** `[9]` (RW)
    ///
    /// Configures to enable I2C bus arbitration detection.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub fn arbitration_en(&self) -> bool {
        (self.value.read_bits() & Self::ARBITRATION_EN_MASK) != 0
    }

    /// Set ARBITRATION_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arbitration_en(&self) {
        self.value.modify_bits(|r| r | Self::ARBITRATION_EN_MASK);
    }

    /// Clear ARBITRATION_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arbitration_en(&self) {
        self.value.modify_bits(|r| r & !Self::ARBITRATION_EN_MASK);
    }

    /// **CLK_EN** `[8]` (RW)
    ///
    /// Configures whether to gate clock signal for registers.
    /// 0: Force clock on for registers
    /// 1: Support clock only when registers are read or written to by software.
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

    /// **RX_LSB_FIRST** `[7]` (RW)
    ///
    /// Configures to control the storage order for received data.
    /// 1: receive data from the least significant bit
    /// 0: receive data from the most significant bit.
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> bool {
        (self.value.read_bits() & Self::RX_LSB_FIRST_MASK) != 0
    }

    /// Set RX_LSB_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_lsb_first(&self) {
        self.value.modify_bits(|r| r | Self::RX_LSB_FIRST_MASK);
    }

    /// Clear RX_LSB_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_lsb_first(&self) {
        self.value.modify_bits(|r| r & !Self::RX_LSB_FIRST_MASK);
    }

    /// **TX_LSB_FIRST** `[6]` (RW)
    ///
    /// Configures to control the sending order for data needing to be sent.
    /// 1: send data from the least significant bit,
    /// 0: send data from the most significant bit.
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> bool {
        (self.value.read_bits() & Self::TX_LSB_FIRST_MASK) != 0
    }

    /// Set TX_LSB_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_lsb_first(&self) {
        self.value.modify_bits(|r| r | Self::TX_LSB_FIRST_MASK);
    }

    /// Clear TX_LSB_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_lsb_first(&self) {
        self.value.modify_bits(|r| r & !Self::TX_LSB_FIRST_MASK);
    }

    /// Trigger **TRANS_START** (WO)
    ///
    /// Configures to start sending the data in txfifo for slave.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub fn trigger_trans_start(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_START_MASK);
    }

    /// **MS_MODE** `[4]` (RW)
    ///
    /// Configures the module as an I2C Master or Slave.
    /// 0: Slave
    /// 1: Master
    #[inline(always)]
    pub fn ms_mode(&self) -> bool {
        (self.value.read_bits() & Self::MS_MODE_MASK) != 0
    }

    /// Set MS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ms_mode(&self) {
        self.value.modify_bits(|r| r | Self::MS_MODE_MASK);
    }

    /// Clear MS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ms_mode(&self) {
        self.value.modify_bits(|r| r & !Self::MS_MODE_MASK);
    }

    /// **RX_FULL_ACK_LEVEL** `[3]` (RW)
    ///
    /// Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold.
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> bool {
        (self.value.read_bits() & Self::RX_FULL_ACK_LEVEL_MASK) != 0
    }

    /// Set RX_FULL_ACK_LEVEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_full_ack_level(&self) {
        self.value.modify_bits(|r| r | Self::RX_FULL_ACK_LEVEL_MASK);
    }

    /// Clear RX_FULL_ACK_LEVEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_full_ack_level(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_FULL_ACK_LEVEL_MASK);
    }

    /// **SAMPLE_SCL_LEVEL** `[2]` (RW)
    ///
    /// Configures the sample mode for SDA.
    /// 1: Sample SDA data on the SCL low level.
    /// 0: Sample SDA data on the SCL high level.
    #[inline(always)]
    pub fn sample_scl_level(&self) -> bool {
        (self.value.read_bits() & Self::SAMPLE_SCL_LEVEL_MASK) != 0
    }

    /// Set SAMPLE_SCL_LEVEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sample_scl_level(&self) {
        self.value.modify_bits(|r| r | Self::SAMPLE_SCL_LEVEL_MASK);
    }

    /// Clear SAMPLE_SCL_LEVEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sample_scl_level(&self) {
        self.value.modify_bits(|r| r & !Self::SAMPLE_SCL_LEVEL_MASK);
    }

    /// **SCL_FORCE_OUT** `[1]` (RW)
    ///
    /// Configures the SCL output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub fn scl_force_out(&self) -> bool {
        (self.value.read_bits() & Self::SCL_FORCE_OUT_MASK) != 0
    }

    /// Set SCL_FORCE_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_force_out(&self) {
        self.value.modify_bits(|r| r | Self::SCL_FORCE_OUT_MASK);
    }

    /// Clear SCL_FORCE_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_force_out(&self) {
        self.value.modify_bits(|r| r & !Self::SCL_FORCE_OUT_MASK);
    }

    /// **SDA_FORCE_OUT** `[0]` (RW)
    ///
    /// Configures the SDA output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub fn sda_force_out(&self) -> bool {
        (self.value.read_bits() & Self::SDA_FORCE_OUT_MASK) != 0
    }

    /// Set SDA_FORCE_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sda_force_out(&self) {
        self.value.modify_bits(|r| r | Self::SDA_FORCE_OUT_MASK);
    }

    /// Clear SDA_FORCE_OUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sda_force_out(&self) {
        self.value.modify_bits(|r| r & !Self::SDA_FORCE_OUT_MASK);
    }
}

/// CTR value type for batch read/modify/write operations
///
/// Use `ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrVal {
    bits: u32,
}

impl CtrVal {
    /// Create from raw bits
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
        Self { bits: 0x00000208 }
    }

    /// Read **ADDR_BROADCASTING_EN** `[14:14]`
    ///
    /// Configures to support the 7bit general call function.
    /// 0: Not support
    /// 1: Support
    #[inline(always)]
    pub const fn addr_broadcasting_en(&self) -> bool {
        (self.bits & Ctr::ADDR_BROADCASTING_EN_MASK) != 0
    }

    /// Read **ADDR_10BIT_RW_CHECK_EN** `[13:13]`
    ///
    /// Configures to check if the r/w bit of 10bit addressing consists with I2C protocol.
    /// 0: Not check
    /// 1: Check
    #[inline(always)]
    pub const fn addr_10bit_rw_check_en(&self) -> bool {
        (self.bits & Ctr::ADDR_10BIT_RW_CHECK_EN_MASK) != 0
    }

    /// Read **SLV_TX_AUTO_START_EN** `[12:12]`
    ///
    /// Configures to enable slave to send data automatically
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn slv_tx_auto_start_en(&self) -> bool {
        (self.bits & Ctr::SLV_TX_AUTO_START_EN_MASK) != 0
    }

    /// Read **ARBITRATION_EN** `[9:9]`
    ///
    /// Configures to enable I2C bus arbitration detection.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn arbitration_en(&self) -> bool {
        (self.bits & Ctr::ARBITRATION_EN_MASK) != 0
    }

    /// Read **CLK_EN** `[8:8]`
    ///
    /// Configures whether to gate clock signal for registers.
    /// 0: Force clock on for registers
    /// 1: Support clock only when registers are read or written to by software.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Ctr::CLK_EN_MASK) != 0
    }

    /// Read **RX_LSB_FIRST** `[7:7]`
    ///
    /// Configures to control the storage order for received data.
    /// 1: receive data from the least significant bit
    /// 0: receive data from the most significant bit.
    #[inline(always)]
    pub const fn rx_lsb_first(&self) -> bool {
        (self.bits & Ctr::RX_LSB_FIRST_MASK) != 0
    }

    /// Read **TX_LSB_FIRST** `[6:6]`
    ///
    /// Configures to control the sending order for data needing to be sent.
    /// 1: send data from the least significant bit,
    /// 0: send data from the most significant bit.
    #[inline(always)]
    pub const fn tx_lsb_first(&self) -> bool {
        (self.bits & Ctr::TX_LSB_FIRST_MASK) != 0
    }

    /// Read **MS_MODE** `[4:4]`
    ///
    /// Configures the module as an I2C Master or Slave.
    /// 0: Slave
    /// 1: Master
    #[inline(always)]
    pub const fn ms_mode(&self) -> bool {
        (self.bits & Ctr::MS_MODE_MASK) != 0
    }

    /// Read **RX_FULL_ACK_LEVEL** `[3:3]`
    ///
    /// Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold.
    #[inline(always)]
    pub const fn rx_full_ack_level(&self) -> bool {
        (self.bits & Ctr::RX_FULL_ACK_LEVEL_MASK) != 0
    }

    /// Read **SAMPLE_SCL_LEVEL** `[2:2]`
    ///
    /// Configures the sample mode for SDA.
    /// 1: Sample SDA data on the SCL low level.
    /// 0: Sample SDA data on the SCL high level.
    #[inline(always)]
    pub const fn sample_scl_level(&self) -> bool {
        (self.bits & Ctr::SAMPLE_SCL_LEVEL_MASK) != 0
    }

    /// Read **SCL_FORCE_OUT** `[1:1]`
    ///
    /// Configures the SCL output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub const fn scl_force_out(&self) -> bool {
        (self.bits & Ctr::SCL_FORCE_OUT_MASK) != 0
    }

    /// Read **SDA_FORCE_OUT** `[0:0]`
    ///
    /// Configures the SDA output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub const fn sda_force_out(&self) -> bool {
        (self.bits & Ctr::SDA_FORCE_OUT_MASK) != 0
    }

    /// Set **ADDR_BROADCASTING_EN** `[14:14]`
    ///
    /// Configures to support the 7bit general call function.
    /// 0: Not support
    /// 1: Support
    #[inline(always)]
    pub const fn with_addr_broadcasting_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::ADDR_BROADCASTING_EN_MASK
            } else {
                self.bits & !Ctr::ADDR_BROADCASTING_EN_MASK
            },
        }
    }

    /// Set **ADDR_10BIT_RW_CHECK_EN** `[13:13]`
    ///
    /// Configures to check if the r/w bit of 10bit addressing consists with I2C protocol.
    /// 0: Not check
    /// 1: Check
    #[inline(always)]
    pub const fn with_addr_10bit_rw_check_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::ADDR_10BIT_RW_CHECK_EN_MASK
            } else {
                self.bits & !Ctr::ADDR_10BIT_RW_CHECK_EN_MASK
            },
        }
    }

    /// Set **SLV_TX_AUTO_START_EN** `[12:12]`
    ///
    /// Configures to enable slave to send data automatically
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn with_slv_tx_auto_start_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::SLV_TX_AUTO_START_EN_MASK
            } else {
                self.bits & !Ctr::SLV_TX_AUTO_START_EN_MASK
            },
        }
    }

    /// Set **CONF_UPGATE** `[11:11]`
    ///
    /// Configures this bit for synchronization
    /// 0: No effect
    /// 1: Synchronize
    #[inline(always)]
    pub const fn with_conf_upgate(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::CONF_UPGATE_MASK
            } else {
                self.bits & !Ctr::CONF_UPGATE_MASK
            },
        }
    }

    /// Set **FSM_RST** `[10:10]`
    ///
    /// Configures to reset the SCL_FSM.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub const fn with_fsm_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::FSM_RST_MASK
            } else {
                self.bits & !Ctr::FSM_RST_MASK
            },
        }
    }

    /// Set **ARBITRATION_EN** `[9:9]`
    ///
    /// Configures to enable I2C bus arbitration detection.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn with_arbitration_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::ARBITRATION_EN_MASK
            } else {
                self.bits & !Ctr::ARBITRATION_EN_MASK
            },
        }
    }

    /// Set **CLK_EN** `[8:8]`
    ///
    /// Configures whether to gate clock signal for registers.
    /// 0: Force clock on for registers
    /// 1: Support clock only when registers are read or written to by software.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::CLK_EN_MASK
            } else {
                self.bits & !Ctr::CLK_EN_MASK
            },
        }
    }

    /// Set **RX_LSB_FIRST** `[7:7]`
    ///
    /// Configures to control the storage order for received data.
    /// 1: receive data from the least significant bit
    /// 0: receive data from the most significant bit.
    #[inline(always)]
    pub const fn with_rx_lsb_first(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::RX_LSB_FIRST_MASK
            } else {
                self.bits & !Ctr::RX_LSB_FIRST_MASK
            },
        }
    }

    /// Set **TX_LSB_FIRST** `[6:6]`
    ///
    /// Configures to control the sending order for data needing to be sent.
    /// 1: send data from the least significant bit,
    /// 0: send data from the most significant bit.
    #[inline(always)]
    pub const fn with_tx_lsb_first(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::TX_LSB_FIRST_MASK
            } else {
                self.bits & !Ctr::TX_LSB_FIRST_MASK
            },
        }
    }

    /// Set **TRANS_START** `[5:5]`
    ///
    /// Configures to start sending the data in txfifo for slave.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub const fn with_trans_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::TRANS_START_MASK
            } else {
                self.bits & !Ctr::TRANS_START_MASK
            },
        }
    }

    /// Set **MS_MODE** `[4:4]`
    ///
    /// Configures the module as an I2C Master or Slave.
    /// 0: Slave
    /// 1: Master
    #[inline(always)]
    pub const fn with_ms_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::MS_MODE_MASK
            } else {
                self.bits & !Ctr::MS_MODE_MASK
            },
        }
    }

    /// Set **RX_FULL_ACK_LEVEL** `[3:3]`
    ///
    /// Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold.
    #[inline(always)]
    pub const fn with_rx_full_ack_level(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::RX_FULL_ACK_LEVEL_MASK
            } else {
                self.bits & !Ctr::RX_FULL_ACK_LEVEL_MASK
            },
        }
    }

    /// Set **SAMPLE_SCL_LEVEL** `[2:2]`
    ///
    /// Configures the sample mode for SDA.
    /// 1: Sample SDA data on the SCL low level.
    /// 0: Sample SDA data on the SCL high level.
    #[inline(always)]
    pub const fn with_sample_scl_level(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::SAMPLE_SCL_LEVEL_MASK
            } else {
                self.bits & !Ctr::SAMPLE_SCL_LEVEL_MASK
            },
        }
    }

    /// Set **SCL_FORCE_OUT** `[1:1]`
    ///
    /// Configures the SCL output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub const fn with_scl_force_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::SCL_FORCE_OUT_MASK
            } else {
                self.bits & !Ctr::SCL_FORCE_OUT_MASK
            },
        }
    }

    /// Set **SDA_FORCE_OUT** `[0:0]`
    ///
    /// Configures the SDA output mode
    /// 1: Direct output,
    /// 0: Open drain output.
    #[inline(always)]
    pub const fn with_sda_force_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctr::SDA_FORCE_OUT_MASK
            } else {
                self.bits & !Ctr::SDA_FORCE_OUT_MASK
            },
        }
    }
}

impl Default for CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrVal {
        CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrVal) -> CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SR Register
///
/// Describe I2C work status.
///
/// Address offset: 0x0008
/// Reset value: 0x0000c000
///
/// ## Fields
///
/// - **SCL_STATE_LAST** `[30:28]` (RO)
///
///   Represents the states of the state machine used to produce SCL.
///   0: Idle,
///   1: Start,
///   2: Negative edge,
///   3: Low,
///   4: Positive edge,
///   5: High,
///   6: Stop
///
/// - **SCL_MAIN_STATE_LAST** `[26:24]` (RO)
///
///   Represents the states of the I2C module state machine.
///   0: Idle,
///   1: Address shift,
///   2: ACK address,
///   3: Rx data,
///   4: Tx data,
///   5: Send ACK,
///   6: Wait ACK
///
/// - **TXFIFO_CNT** `[23:18]` (RO)
///
///   Represents the number of data bytes received in RAM.
///
/// - **STRETCH_CAUSE** `[15:14]` (RO)
///
///   Represents the cause of SCL clocking stretching in slave mode.
///   0: Stretching SCL low when the master starts to read data.
///   1: Stretching SCL low when I2C TX FIFO is empty in slave mode.
///   2: Stretching SCL low when I2C RX FIFO is full in slave mode.
///
/// - **RXFIFO_CNT** `[13:8]` (RO)
///
///   Represents the number of data bytes to be sent.
///
/// - **SLAVE_ADDRESSED** `[5]` (RO)
///
///   Represents whether the address sent by the master is equal to the address of the slave.
///   Valid only when the module is configured as an I2C Slave.
///   0: Not equal
///   1: Equal
///
/// - **BUS_BUSY** `[4]` (RO)
///
///   Represents the I2C bus state.
///   1: The I2C bus is busy transferring data,
///   0: The I2C bus is in idle state.
///
/// - **ARB_LOST** `[3]` (RO)
///
///   Represents whether the I2C controller loses control of SCL line.
///   0: No arbitration lost
///   1: Arbitration lost
///
/// - **SLAVE_RW** `[1]` (RO)
///
///   Represents the transfer direction in slave mode,.
///   1: Master reads from slave,
///   0: Master writes to slave.
///
/// - **RESP_REC** `[0]` (RO)
///
///   Represents the received ACK value in master mode or slave mode.
///   0: ACK,
///   1: NACK.
#[repr(transparent)]
pub struct Sr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sr {
    // ======== Field Constants ========

    /// **SCL_STATE_LAST**
    ///
    /// Represents the states of the state machine used to produce SCL.
    /// 0: Idle,
    /// 1: Start,
    /// 2: Negative edge,
    /// 3: Low,
    /// 4: Positive edge,
    /// 5: High,
    /// 6: Stop
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const SCL_STATE_LAST_OFFSET: u32 = 28;
    pub const SCL_STATE_LAST_MASK: u32 = 0x70000000;

    /// **SCL_MAIN_STATE_LAST**
    ///
    /// Represents the states of the I2C module state machine.
    /// 0: Idle,
    /// 1: Address shift,
    /// 2: ACK address,
    /// 3: Rx data,
    /// 4: Tx data,
    /// 5: Send ACK,
    /// 6: Wait ACK
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SCL_MAIN_STATE_LAST_OFFSET: u32 = 24;
    pub const SCL_MAIN_STATE_LAST_MASK: u32 = 0x07000000;

    /// **TXFIFO_CNT**
    ///
    /// Represents the number of data bytes received in RAM.
    ///
    /// Bits `[23:18]`, mask `0x00fc0000`
    pub const TXFIFO_CNT_OFFSET: u32 = 18;
    pub const TXFIFO_CNT_MASK: u32 = 0x00fc0000;

    /// **STRETCH_CAUSE**
    ///
    /// Represents the cause of SCL clocking stretching in slave mode.
    /// 0: Stretching SCL low when the master starts to read data.
    /// 1: Stretching SCL low when I2C TX FIFO is empty in slave mode.
    /// 2: Stretching SCL low when I2C RX FIFO is full in slave mode.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const STRETCH_CAUSE_OFFSET: u32 = 14;
    pub const STRETCH_CAUSE_MASK: u32 = 0x0000c000;

    /// **RXFIFO_CNT**
    ///
    /// Represents the number of data bytes to be sent.
    ///
    /// Bits `[13:8]`, mask `0x00003f00`
    pub const RXFIFO_CNT_OFFSET: u32 = 8;
    pub const RXFIFO_CNT_MASK: u32 = 0x00003f00;

    /// **SLAVE_ADDRESSED**
    ///
    /// Represents whether the address sent by the master is equal to the address of the slave.
    /// Valid only when the module is configured as an I2C Slave.
    /// 0: Not equal
    /// 1: Equal
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SLAVE_ADDRESSED_OFFSET: u32 = 5;
    pub const SLAVE_ADDRESSED_MASK: u32 = 0x00000020;
    pub const SLAVE_ADDRESSED: u32 = 1 << 5;

    /// **BUS_BUSY**
    ///
    /// Represents the I2C bus state.
    /// 1: The I2C bus is busy transferring data,
    /// 0: The I2C bus is in idle state.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BUS_BUSY_OFFSET: u32 = 4;
    pub const BUS_BUSY_MASK: u32 = 0x00000010;
    pub const BUS_BUSY: u32 = 1 << 4;

    /// **ARB_LOST**
    ///
    /// Represents whether the I2C controller loses control of SCL line.
    /// 0: No arbitration lost
    /// 1: Arbitration lost
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ARB_LOST_OFFSET: u32 = 3;
    pub const ARB_LOST_MASK: u32 = 0x00000008;
    pub const ARB_LOST: u32 = 1 << 3;

    /// **SLAVE_RW**
    ///
    /// Represents the transfer direction in slave mode,.
    /// 1: Master reads from slave,
    /// 0: Master writes to slave.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SLAVE_RW_OFFSET: u32 = 1;
    pub const SLAVE_RW_MASK: u32 = 0x00000002;
    pub const SLAVE_RW: u32 = 1 << 1;

    /// **RESP_REC**
    ///
    /// Represents the received ACK value in master mode or slave mode.
    /// 0: ACK,
    /// 1: NACK.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RESP_REC_OFFSET: u32 = 0;
    pub const RESP_REC_MASK: u32 = 0x00000001;
    pub const RESP_REC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SCL_STATE_LAST** `[30:28]` (RO)
    ///
    /// Represents the states of the state machine used to produce SCL.
    /// 0: Idle,
    /// 1: Start,
    /// 2: Negative edge,
    /// 3: Low,
    /// 4: Positive edge,
    /// 5: High,
    /// 6: Stop
    #[inline(always)]
    pub fn scl_state_last(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_STATE_LAST_MASK) >> Self::SCL_STATE_LAST_OFFSET
    }

    /// **SCL_MAIN_STATE_LAST** `[26:24]` (RO)
    ///
    /// Represents the states of the I2C module state machine.
    /// 0: Idle,
    /// 1: Address shift,
    /// 2: ACK address,
    /// 3: Rx data,
    /// 4: Tx data,
    /// 5: Send ACK,
    /// 6: Wait ACK
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_MAIN_STATE_LAST_MASK)
            >> Self::SCL_MAIN_STATE_LAST_OFFSET
    }

    /// **TXFIFO_CNT** `[23:18]` (RO)
    ///
    /// Represents the number of data bytes received in RAM.
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_CNT_MASK) >> Self::TXFIFO_CNT_OFFSET
    }

    /// **STRETCH_CAUSE** `[15:14]` (RO)
    ///
    /// Represents the cause of SCL clocking stretching in slave mode.
    /// 0: Stretching SCL low when the master starts to read data.
    /// 1: Stretching SCL low when I2C TX FIFO is empty in slave mode.
    /// 2: Stretching SCL low when I2C RX FIFO is full in slave mode.
    #[inline(always)]
    pub fn stretch_cause(&self) -> u32 {
        (self.value.read_bits() & Self::STRETCH_CAUSE_MASK) >> Self::STRETCH_CAUSE_OFFSET
    }

    /// **RXFIFO_CNT** `[13:8]` (RO)
    ///
    /// Represents the number of data bytes to be sent.
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_CNT_MASK) >> Self::RXFIFO_CNT_OFFSET
    }

    /// **SLAVE_ADDRESSED** `[5]` (RO)
    ///
    /// Represents whether the address sent by the master is equal to the address of the slave.
    /// Valid only when the module is configured as an I2C Slave.
    /// 0: Not equal
    /// 1: Equal
    #[inline(always)]
    pub fn slave_addressed(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_ADDRESSED_MASK) != 0
    }

    /// **BUS_BUSY** `[4]` (RO)
    ///
    /// Represents the I2C bus state.
    /// 1: The I2C bus is busy transferring data,
    /// 0: The I2C bus is in idle state.
    #[inline(always)]
    pub fn bus_busy(&self) -> bool {
        (self.value.read_bits() & Self::BUS_BUSY_MASK) != 0
    }

    /// **ARB_LOST** `[3]` (RO)
    ///
    /// Represents whether the I2C controller loses control of SCL line.
    /// 0: No arbitration lost
    /// 1: Arbitration lost
    #[inline(always)]
    pub fn arb_lost(&self) -> bool {
        (self.value.read_bits() & Self::ARB_LOST_MASK) != 0
    }

    /// **SLAVE_RW** `[1]` (RO)
    ///
    /// Represents the transfer direction in slave mode,.
    /// 1: Master reads from slave,
    /// 0: Master writes to slave.
    #[inline(always)]
    pub fn slave_rw(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_RW_MASK) != 0
    }

    /// **RESP_REC** `[0]` (RO)
    ///
    /// Represents the received ACK value in master mode or slave mode.
    /// 0: ACK,
    /// 1: NACK.
    #[inline(always)]
    pub fn resp_rec(&self) -> bool {
        (self.value.read_bits() & Self::RESP_REC_MASK) != 0
    }
}

/// SR value type for batch read/modify/write operations
///
/// Use `sr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrVal {
    bits: u32,
}

impl SrVal {
    /// Create from raw bits
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
        Self { bits: 0x0000c000 }
    }

    /// Read **SCL_STATE_LAST** `[30:28]`
    ///
    /// Represents the states of the state machine used to produce SCL.
    /// 0: Idle,
    /// 1: Start,
    /// 2: Negative edge,
    /// 3: Low,
    /// 4: Positive edge,
    /// 5: High,
    /// 6: Stop
    #[inline(always)]
    pub const fn scl_state_last(&self) -> u32 {
        (self.bits & Sr::SCL_STATE_LAST_MASK) >> Sr::SCL_STATE_LAST_OFFSET
    }

    /// Read **SCL_MAIN_STATE_LAST** `[26:24]`
    ///
    /// Represents the states of the I2C module state machine.
    /// 0: Idle,
    /// 1: Address shift,
    /// 2: ACK address,
    /// 3: Rx data,
    /// 4: Tx data,
    /// 5: Send ACK,
    /// 6: Wait ACK
    #[inline(always)]
    pub const fn scl_main_state_last(&self) -> u32 {
        (self.bits & Sr::SCL_MAIN_STATE_LAST_MASK) >> Sr::SCL_MAIN_STATE_LAST_OFFSET
    }

    /// Read **TXFIFO_CNT** `[23:18]`
    ///
    /// Represents the number of data bytes received in RAM.
    #[inline(always)]
    pub const fn txfifo_cnt(&self) -> u32 {
        (self.bits & Sr::TXFIFO_CNT_MASK) >> Sr::TXFIFO_CNT_OFFSET
    }

    /// Read **STRETCH_CAUSE** `[15:14]`
    ///
    /// Represents the cause of SCL clocking stretching in slave mode.
    /// 0: Stretching SCL low when the master starts to read data.
    /// 1: Stretching SCL low when I2C TX FIFO is empty in slave mode.
    /// 2: Stretching SCL low when I2C RX FIFO is full in slave mode.
    #[inline(always)]
    pub const fn stretch_cause(&self) -> u32 {
        (self.bits & Sr::STRETCH_CAUSE_MASK) >> Sr::STRETCH_CAUSE_OFFSET
    }

    /// Read **RXFIFO_CNT** `[13:8]`
    ///
    /// Represents the number of data bytes to be sent.
    #[inline(always)]
    pub const fn rxfifo_cnt(&self) -> u32 {
        (self.bits & Sr::RXFIFO_CNT_MASK) >> Sr::RXFIFO_CNT_OFFSET
    }

    /// Read **SLAVE_ADDRESSED** `[5:5]`
    ///
    /// Represents whether the address sent by the master is equal to the address of the slave.
    /// Valid only when the module is configured as an I2C Slave.
    /// 0: Not equal
    /// 1: Equal
    #[inline(always)]
    pub const fn slave_addressed(&self) -> bool {
        (self.bits & Sr::SLAVE_ADDRESSED_MASK) != 0
    }

    /// Read **BUS_BUSY** `[4:4]`
    ///
    /// Represents the I2C bus state.
    /// 1: The I2C bus is busy transferring data,
    /// 0: The I2C bus is in idle state.
    #[inline(always)]
    pub const fn bus_busy(&self) -> bool {
        (self.bits & Sr::BUS_BUSY_MASK) != 0
    }

    /// Read **ARB_LOST** `[3:3]`
    ///
    /// Represents whether the I2C controller loses control of SCL line.
    /// 0: No arbitration lost
    /// 1: Arbitration lost
    #[inline(always)]
    pub const fn arb_lost(&self) -> bool {
        (self.bits & Sr::ARB_LOST_MASK) != 0
    }

    /// Read **SLAVE_RW** `[1:1]`
    ///
    /// Represents the transfer direction in slave mode,.
    /// 1: Master reads from slave,
    /// 0: Master writes to slave.
    #[inline(always)]
    pub const fn slave_rw(&self) -> bool {
        (self.bits & Sr::SLAVE_RW_MASK) != 0
    }

    /// Read **RESP_REC** `[0:0]`
    ///
    /// Represents the received ACK value in master mode or slave mode.
    /// 0: ACK,
    /// 1: NACK.
    #[inline(always)]
    pub const fn resp_rec(&self) -> bool {
        (self.bits & Sr::RESP_REC_MASK) != 0
    }
}

impl Default for SrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrVal {
        SrVal::from_bits(self.value.read_bits())
    }
}

/// TO Register
///
/// Setting time out control for receiving data.
///
/// Address offset: 0x000c
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **TIME_OUT_EN** `[5]` (RW)
///
///   Configures to enable time out control.
///   0: No effect
///   1: Enable
///
/// - **TIME_OUT_VALUE** `[4:0]` (RW)
///
///   Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
///   Measurement unit: i2c_sclk.
#[repr(transparent)]
pub struct To {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for To {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl To {
    // ======== Field Constants ========

    /// **TIME_OUT_EN**
    ///
    /// Configures to enable time out control.
    /// 0: No effect
    /// 1: Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TIME_OUT_EN_OFFSET: u32 = 5;
    pub const TIME_OUT_EN_MASK: u32 = 0x00000020;
    pub const TIME_OUT_EN: u32 = 1 << 5;

    /// **TIME_OUT_VALUE**
    ///
    /// Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
    /// Measurement unit: i2c_sclk.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TIME_OUT_VALUE_OFFSET: u32 = 0;
    pub const TIME_OUT_VALUE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **TIME_OUT_EN** `[5]` (RW)
    ///
    /// Configures to enable time out control.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub fn time_out_en(&self) -> bool {
        (self.value.read_bits() & Self::TIME_OUT_EN_MASK) != 0
    }

    /// Set TIME_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_time_out_en(&self) {
        self.value.modify_bits(|r| r | Self::TIME_OUT_EN_MASK);
    }

    /// Clear TIME_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_time_out_en(&self) {
        self.value.modify_bits(|r| r & !Self::TIME_OUT_EN_MASK);
    }

    /// **TIME_OUT_VALUE** `[4:0]` (RW)
    ///
    /// Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn time_out_value(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_OUT_VALUE_MASK) >> Self::TIME_OUT_VALUE_OFFSET
    }

    /// Write **TIME_OUT_VALUE** `[4:0]` (RW)
    ///
    /// Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn write_time_out_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_OUT_VALUE_MASK)
                | ((value << Self::TIME_OUT_VALUE_OFFSET) & Self::TIME_OUT_VALUE_MASK)
        });
    }
}

/// TO value type for batch read/modify/write operations
///
/// Use `to.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `to.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ToVal {
    bits: u32,
}

impl ToVal {
    /// Create from raw bits
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

    /// Read **TIME_OUT_EN** `[5:5]`
    ///
    /// Configures to enable time out control.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn time_out_en(&self) -> bool {
        (self.bits & To::TIME_OUT_EN_MASK) != 0
    }

    /// Read **TIME_OUT_VALUE** `[4:0]`
    ///
    /// Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn time_out_value(&self) -> u32 {
        (self.bits & To::TIME_OUT_VALUE_MASK) >> To::TIME_OUT_VALUE_OFFSET
    }

    /// Set **TIME_OUT_EN** `[5:5]`
    ///
    /// Configures to enable time out control.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn with_time_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | To::TIME_OUT_EN_MASK
            } else {
                self.bits & !To::TIME_OUT_EN_MASK
            },
        }
    }

    /// Set **TIME_OUT_VALUE** `[4:0]`
    ///
    /// Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value).
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn with_time_out_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !To::TIME_OUT_VALUE_MASK)
                | ((value << To::TIME_OUT_VALUE_OFFSET) & To::TIME_OUT_VALUE_MASK),
        }
    }
}

impl Default for ToVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl To {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ToVal {
        ToVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ToVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ToVal) -> ToVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLAVE_ADDR Register
///
/// Local slave address setting
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **ADDR_10BIT_EN** `[31]` (RW)
///
///   Configures to enable the slave 10-bit addressing mode in master mode.
///   0: No effect
///   1: Enable
///
/// - **SLAVE_ADDR** `[14:0]` (RW)
///
///   Configure the slave address of I2C Slave.
#[repr(transparent)]
pub struct SlaveAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SlaveAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SlaveAddr {
    // ======== Field Constants ========

    /// **ADDR_10BIT_EN**
    ///
    /// Configures to enable the slave 10-bit addressing mode in master mode.
    /// 0: No effect
    /// 1: Enable
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDR_10BIT_EN_OFFSET: u32 = 31;
    pub const ADDR_10BIT_EN_MASK: u32 = 0x80000000;
    pub const ADDR_10BIT_EN: u32 = 1 << 31;

    /// **SLAVE_ADDR**
    ///
    /// Configure the slave address of I2C Slave.
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const SLAVE_ADDR_OFFSET: u32 = 0;
    pub const SLAVE_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **ADDR_10BIT_EN** `[31]` (RW)
    ///
    /// Configures to enable the slave 10-bit addressing mode in master mode.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub fn addr_10bit_en(&self) -> bool {
        (self.value.read_bits() & Self::ADDR_10BIT_EN_MASK) != 0
    }

    /// Set ADDR_10BIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addr_10bit_en(&self) {
        self.value.modify_bits(|r| r | Self::ADDR_10BIT_EN_MASK);
    }

    /// Clear ADDR_10BIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addr_10bit_en(&self) {
        self.value.modify_bits(|r| r & !Self::ADDR_10BIT_EN_MASK);
    }

    /// **SLAVE_ADDR** `[14:0]` (RW)
    ///
    /// Configure the slave address of I2C Slave.
    #[inline(always)]
    pub fn slave_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SLAVE_ADDR_MASK) >> Self::SLAVE_ADDR_OFFSET
    }

    /// Write **SLAVE_ADDR** `[14:0]` (RW)
    ///
    /// Configure the slave address of I2C Slave.
    #[inline(always)]
    pub fn write_slave_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLAVE_ADDR_MASK)
                | ((value << Self::SLAVE_ADDR_OFFSET) & Self::SLAVE_ADDR_MASK)
        });
    }
}

/// SLAVE_ADDR value type for batch read/modify/write operations
///
/// Use `slaveaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `slaveaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SlaveAddrVal {
    bits: u32,
}

impl SlaveAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ADDR_10BIT_EN** `[31:31]`
    ///
    /// Configures to enable the slave 10-bit addressing mode in master mode.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn addr_10bit_en(&self) -> bool {
        (self.bits & SlaveAddr::ADDR_10BIT_EN_MASK) != 0
    }

    /// Read **SLAVE_ADDR** `[14:0]`
    ///
    /// Configure the slave address of I2C Slave.
    #[inline(always)]
    pub const fn slave_addr(&self) -> u32 {
        (self.bits & SlaveAddr::SLAVE_ADDR_MASK) >> SlaveAddr::SLAVE_ADDR_OFFSET
    }

    /// Set **ADDR_10BIT_EN** `[31:31]`
    ///
    /// Configures to enable the slave 10-bit addressing mode in master mode.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn with_addr_10bit_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SlaveAddr::ADDR_10BIT_EN_MASK
            } else {
                self.bits & !SlaveAddr::ADDR_10BIT_EN_MASK
            },
        }
    }

    /// Set **SLAVE_ADDR** `[14:0]`
    ///
    /// Configure the slave address of I2C Slave.
    #[inline(always)]
    pub const fn with_slave_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SlaveAddr::SLAVE_ADDR_MASK)
                | ((value << SlaveAddr::SLAVE_ADDR_OFFSET) & SlaveAddr::SLAVE_ADDR_MASK),
        }
    }
}

impl Default for SlaveAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SlaveAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SlaveAddrVal {
        SlaveAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SlaveAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SlaveAddrVal) -> SlaveAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FIFO_ST Register
///
/// FIFO status register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **SLAVE_RW_POINT** `[29:22]` (RO)
///
///   Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode.
///
/// - **TXFIFO_WADDR** `[19:15]` (RO)
///
///   Represents the offset address of APB bus writing to TXFIFO.
///
/// - **TXFIFO_RADDR** `[14:10]` (RO)
///
///   Represents the offset address of i2c module reading from TXFIFO.
///
/// - **RXFIFO_WADDR** `[9:5]` (RO)
///
///   Represents the offset address of i2c module receiving data and writing to RXFIFO.
///
/// - **RXFIFO_RADDR** `[4:0]` (RO)
///
///   Represents the offset address of the APB reading from RXFIFO
#[repr(transparent)]
pub struct FifoSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoSt {
    // ======== Field Constants ========

    /// **SLAVE_RW_POINT**
    ///
    /// Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode.
    ///
    /// Bits `[29:22]`, mask `0x3fc00000`
    pub const SLAVE_RW_POINT_OFFSET: u32 = 22;
    pub const SLAVE_RW_POINT_MASK: u32 = 0x3fc00000;

    /// **TXFIFO_WADDR**
    ///
    /// Represents the offset address of APB bus writing to TXFIFO.
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const TXFIFO_WADDR_OFFSET: u32 = 15;
    pub const TXFIFO_WADDR_MASK: u32 = 0x000f8000;

    /// **TXFIFO_RADDR**
    ///
    /// Represents the offset address of i2c module reading from TXFIFO.
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const TXFIFO_RADDR_OFFSET: u32 = 10;
    pub const TXFIFO_RADDR_MASK: u32 = 0x00007c00;

    /// **RXFIFO_WADDR**
    ///
    /// Represents the offset address of i2c module receiving data and writing to RXFIFO.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const RXFIFO_WADDR_OFFSET: u32 = 5;
    pub const RXFIFO_WADDR_MASK: u32 = 0x000003e0;

    /// **RXFIFO_RADDR**
    ///
    /// Represents the offset address of the APB reading from RXFIFO
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const RXFIFO_RADDR_OFFSET: u32 = 0;
    pub const RXFIFO_RADDR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SLAVE_RW_POINT** `[29:22]` (RO)
    ///
    /// Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode.
    #[inline(always)]
    pub fn slave_rw_point(&self) -> u32 {
        (self.value.read_bits() & Self::SLAVE_RW_POINT_MASK) >> Self::SLAVE_RW_POINT_OFFSET
    }

    /// **TXFIFO_WADDR** `[19:15]` (RO)
    ///
    /// Represents the offset address of APB bus writing to TXFIFO.
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_WADDR_MASK) >> Self::TXFIFO_WADDR_OFFSET
    }

    /// **TXFIFO_RADDR** `[14:10]` (RO)
    ///
    /// Represents the offset address of i2c module reading from TXFIFO.
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_RADDR_MASK) >> Self::TXFIFO_RADDR_OFFSET
    }

    /// **RXFIFO_WADDR** `[9:5]` (RO)
    ///
    /// Represents the offset address of i2c module receiving data and writing to RXFIFO.
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_WADDR_MASK) >> Self::RXFIFO_WADDR_OFFSET
    }

    /// **RXFIFO_RADDR** `[4:0]` (RO)
    ///
    /// Represents the offset address of the APB reading from RXFIFO
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_RADDR_MASK) >> Self::RXFIFO_RADDR_OFFSET
    }
}

/// FIFO_ST value type for batch read/modify/write operations
///
/// Use `fifost.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifost.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoStVal {
    bits: u32,
}

impl FifoStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SLAVE_RW_POINT** `[29:22]`
    ///
    /// Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode.
    #[inline(always)]
    pub const fn slave_rw_point(&self) -> u32 {
        (self.bits & FifoSt::SLAVE_RW_POINT_MASK) >> FifoSt::SLAVE_RW_POINT_OFFSET
    }

    /// Read **TXFIFO_WADDR** `[19:15]`
    ///
    /// Represents the offset address of APB bus writing to TXFIFO.
    #[inline(always)]
    pub const fn txfifo_waddr(&self) -> u32 {
        (self.bits & FifoSt::TXFIFO_WADDR_MASK) >> FifoSt::TXFIFO_WADDR_OFFSET
    }

    /// Read **TXFIFO_RADDR** `[14:10]`
    ///
    /// Represents the offset address of i2c module reading from TXFIFO.
    #[inline(always)]
    pub const fn txfifo_raddr(&self) -> u32 {
        (self.bits & FifoSt::TXFIFO_RADDR_MASK) >> FifoSt::TXFIFO_RADDR_OFFSET
    }

    /// Read **RXFIFO_WADDR** `[9:5]`
    ///
    /// Represents the offset address of i2c module receiving data and writing to RXFIFO.
    #[inline(always)]
    pub const fn rxfifo_waddr(&self) -> u32 {
        (self.bits & FifoSt::RXFIFO_WADDR_MASK) >> FifoSt::RXFIFO_WADDR_OFFSET
    }

    /// Read **RXFIFO_RADDR** `[4:0]`
    ///
    /// Represents the offset address of the APB reading from RXFIFO
    #[inline(always)]
    pub const fn rxfifo_raddr(&self) -> u32 {
        (self.bits & FifoSt::RXFIFO_RADDR_MASK) >> FifoSt::RXFIFO_RADDR_OFFSET
    }
}

impl Default for FifoStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FifoSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoStVal {
        FifoStVal::from_bits(self.value.read_bits())
    }
}

/// FIFO_CONF Register
///
/// FIFO configuration register.
///
/// Address offset: 0x0018
/// Reset value: 0x0000408b
///
/// ## Fields
///
/// - **FIFO_PRT_EN** `[14]` (RW)
///
///   Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.
///   0: No effect
///   1: Enable
///
/// - **TX_FIFO_RST** `[13]` (RW)
///
///   Configures to reset TXFIFO.
///   0: No effect
///   1: Reset
///
/// - **RX_FIFO_RST** `[12]` (RW)
///
///   Configures to reset RXFIFO.
///   0: No effect
///   1: Reset
///
/// - **FIFO_ADDR_CFG_EN** `[11]` (RW)
///
///   Configures to enable double addressing mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM.
///   0: Disable
///   1: Enable
///
/// - **NONFIFO_EN** `[10]` (RW)
///
///   Configures to enable APB nonfifo access.
///
/// - **TXFIFO_WM_THRHD** `[9:5]` (RW)
///
///   Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
///
/// - **RXFIFO_WM_THRHD** `[4:0]` (RW)
///
///   Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
#[repr(transparent)]
pub struct FifoConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoConf {
    // ======== Field Constants ========

    /// **FIFO_PRT_EN**
    ///
    /// Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.
    /// 0: No effect
    /// 1: Enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FIFO_PRT_EN_OFFSET: u32 = 14;
    pub const FIFO_PRT_EN_MASK: u32 = 0x00004000;
    pub const FIFO_PRT_EN: u32 = 1 << 14;

    /// **TX_FIFO_RST**
    ///
    /// Configures to reset TXFIFO.
    /// 0: No effect
    /// 1: Reset
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_FIFO_RST_OFFSET: u32 = 13;
    pub const TX_FIFO_RST_MASK: u32 = 0x00002000;
    pub const TX_FIFO_RST: u32 = 1 << 13;

    /// **RX_FIFO_RST**
    ///
    /// Configures to reset RXFIFO.
    /// 0: No effect
    /// 1: Reset
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_FIFO_RST_OFFSET: u32 = 12;
    pub const RX_FIFO_RST_MASK: u32 = 0x00001000;
    pub const RX_FIFO_RST: u32 = 1 << 12;

    /// **FIFO_ADDR_CFG_EN**
    ///
    /// Configures to enable double addressing mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM.
    /// 0: Disable
    /// 1: Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FIFO_ADDR_CFG_EN_OFFSET: u32 = 11;
    pub const FIFO_ADDR_CFG_EN_MASK: u32 = 0x00000800;
    pub const FIFO_ADDR_CFG_EN: u32 = 1 << 11;

    /// **NONFIFO_EN**
    ///
    /// Configures to enable APB nonfifo access.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const NONFIFO_EN_OFFSET: u32 = 10;
    pub const NONFIFO_EN_MASK: u32 = 0x00000400;
    pub const NONFIFO_EN: u32 = 1 << 10;

    /// **TXFIFO_WM_THRHD**
    ///
    /// Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const TXFIFO_WM_THRHD_OFFSET: u32 = 5;
    pub const TXFIFO_WM_THRHD_MASK: u32 = 0x000003e0;

    /// **RXFIFO_WM_THRHD**
    ///
    /// Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const RXFIFO_WM_THRHD_OFFSET: u32 = 0;
    pub const RXFIFO_WM_THRHD_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FIFO_PRT_EN** `[14]` (RW)
    ///
    /// Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_PRT_EN_MASK) != 0
    }

    /// Set FIFO_PRT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fifo_prt_en(&self) {
        self.value.modify_bits(|r| r | Self::FIFO_PRT_EN_MASK);
    }

    /// Clear FIFO_PRT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fifo_prt_en(&self) {
        self.value.modify_bits(|r| r & !Self::FIFO_PRT_EN_MASK);
    }

    /// **TX_FIFO_RST** `[13]` (RW)
    ///
    /// Configures to reset TXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> bool {
        (self.value.read_bits() & Self::TX_FIFO_RST_MASK) != 0
    }

    /// Set TX_FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_fifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::TX_FIFO_RST_MASK);
    }

    /// Clear TX_FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_fifo_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TX_FIFO_RST_MASK);
    }

    /// **RX_FIFO_RST** `[12]` (RW)
    ///
    /// Configures to reset RXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_RST_MASK) != 0
    }

    /// Set RX_FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_fifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::RX_FIFO_RST_MASK);
    }

    /// Clear RX_FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_fifo_rst(&self) {
        self.value.modify_bits(|r| r & !Self::RX_FIFO_RST_MASK);
    }

    /// **FIFO_ADDR_CFG_EN** `[11]` (RW)
    ///
    /// Configures to enable double addressing mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_ADDR_CFG_EN_MASK) != 0
    }

    /// Set FIFO_ADDR_CFG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fifo_addr_cfg_en(&self) {
        self.value.modify_bits(|r| r | Self::FIFO_ADDR_CFG_EN_MASK);
    }

    /// Clear FIFO_ADDR_CFG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fifo_addr_cfg_en(&self) {
        self.value.modify_bits(|r| r & !Self::FIFO_ADDR_CFG_EN_MASK);
    }

    /// **NONFIFO_EN** `[10]` (RW)
    ///
    /// Configures to enable APB nonfifo access.
    #[inline(always)]
    pub fn nonfifo_en(&self) -> bool {
        (self.value.read_bits() & Self::NONFIFO_EN_MASK) != 0
    }

    /// Set NONFIFO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nonfifo_en(&self) {
        self.value.modify_bits(|r| r | Self::NONFIFO_EN_MASK);
    }

    /// Clear NONFIFO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nonfifo_en(&self) {
        self.value.modify_bits(|r| r & !Self::NONFIFO_EN_MASK);
    }

    /// **TXFIFO_WM_THRHD** `[9:5]` (RW)
    ///
    /// Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_WM_THRHD_MASK) >> Self::TXFIFO_WM_THRHD_OFFSET
    }

    /// Write **TXFIFO_WM_THRHD** `[9:5]` (RW)
    ///
    /// Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub fn write_txfifo_wm_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXFIFO_WM_THRHD_MASK)
                | ((value << Self::TXFIFO_WM_THRHD_OFFSET) & Self::TXFIFO_WM_THRHD_MASK)
        });
    }

    /// **RXFIFO_WM_THRHD** `[4:0]` (RW)
    ///
    /// Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_WM_THRHD_MASK) >> Self::RXFIFO_WM_THRHD_OFFSET
    }

    /// Write **RXFIFO_WM_THRHD** `[4:0]` (RW)
    ///
    /// Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub fn write_rxfifo_wm_thrhd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXFIFO_WM_THRHD_MASK)
                | ((value << Self::RXFIFO_WM_THRHD_OFFSET) & Self::RXFIFO_WM_THRHD_MASK)
        });
    }
}

/// FIFO_CONF value type for batch read/modify/write operations
///
/// Use `fifoconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifoconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoConfVal {
    bits: u32,
}

impl FifoConfVal {
    /// Create from raw bits
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
        Self { bits: 0x0000408b }
    }

    /// Read **FIFO_PRT_EN** `[14:14]`
    ///
    /// Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn fifo_prt_en(&self) -> bool {
        (self.bits & FifoConf::FIFO_PRT_EN_MASK) != 0
    }

    /// Read **TX_FIFO_RST** `[13:13]`
    ///
    /// Configures to reset TXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub const fn tx_fifo_rst(&self) -> bool {
        (self.bits & FifoConf::TX_FIFO_RST_MASK) != 0
    }

    /// Read **RX_FIFO_RST** `[12:12]`
    ///
    /// Configures to reset RXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub const fn rx_fifo_rst(&self) -> bool {
        (self.bits & FifoConf::RX_FIFO_RST_MASK) != 0
    }

    /// Read **FIFO_ADDR_CFG_EN** `[11:11]`
    ///
    /// Configures to enable double addressing mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn fifo_addr_cfg_en(&self) -> bool {
        (self.bits & FifoConf::FIFO_ADDR_CFG_EN_MASK) != 0
    }

    /// Read **NONFIFO_EN** `[10:10]`
    ///
    /// Configures to enable APB nonfifo access.
    #[inline(always)]
    pub const fn nonfifo_en(&self) -> bool {
        (self.bits & FifoConf::NONFIFO_EN_MASK) != 0
    }

    /// Read **TXFIFO_WM_THRHD** `[9:5]`
    ///
    /// Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub const fn txfifo_wm_thrhd(&self) -> u32 {
        (self.bits & FifoConf::TXFIFO_WM_THRHD_MASK) >> FifoConf::TXFIFO_WM_THRHD_OFFSET
    }

    /// Read **RXFIFO_WM_THRHD** `[4:0]`
    ///
    /// Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub const fn rxfifo_wm_thrhd(&self) -> u32 {
        (self.bits & FifoConf::RXFIFO_WM_THRHD_MASK) >> FifoConf::RXFIFO_WM_THRHD_OFFSET
    }

    /// Set **FIFO_PRT_EN** `[14:14]`
    ///
    /// Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.
    /// 0: No effect
    /// 1: Enable
    #[inline(always)]
    pub const fn with_fifo_prt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoConf::FIFO_PRT_EN_MASK
            } else {
                self.bits & !FifoConf::FIFO_PRT_EN_MASK
            },
        }
    }

    /// Set **TX_FIFO_RST** `[13:13]`
    ///
    /// Configures to reset TXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub const fn with_tx_fifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoConf::TX_FIFO_RST_MASK
            } else {
                self.bits & !FifoConf::TX_FIFO_RST_MASK
            },
        }
    }

    /// Set **RX_FIFO_RST** `[12:12]`
    ///
    /// Configures to reset RXFIFO.
    /// 0: No effect
    /// 1: Reset
    #[inline(always)]
    pub const fn with_rx_fifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoConf::RX_FIFO_RST_MASK
            } else {
                self.bits & !FifoConf::RX_FIFO_RST_MASK
            },
        }
    }

    /// Set **FIFO_ADDR_CFG_EN** `[11:11]`
    ///
    /// Configures to enable double addressing mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn with_fifo_addr_cfg_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoConf::FIFO_ADDR_CFG_EN_MASK
            } else {
                self.bits & !FifoConf::FIFO_ADDR_CFG_EN_MASK
            },
        }
    }

    /// Set **NONFIFO_EN** `[10:10]`
    ///
    /// Configures to enable APB nonfifo access.
    #[inline(always)]
    pub const fn with_nonfifo_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoConf::NONFIFO_EN_MASK
            } else {
                self.bits & !FifoConf::NONFIFO_EN_MASK
            },
        }
    }

    /// Set **TXFIFO_WM_THRHD** `[9:5]`
    ///
    /// Configures the water mark threshold of TXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd[4:0], reg_txfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub const fn with_txfifo_wm_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FifoConf::TXFIFO_WM_THRHD_MASK)
                | ((value << FifoConf::TXFIFO_WM_THRHD_OFFSET) & FifoConf::TXFIFO_WM_THRHD_MASK),
        }
    }

    /// Set **RXFIFO_WM_THRHD** `[4:0]`
    ///
    /// Configures the water mark threshold of RXFIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd[4:0], reg_rxfifo_wm_int_raw bit will be valid.
    #[inline(always)]
    pub const fn with_rxfifo_wm_thrhd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FifoConf::RXFIFO_WM_THRHD_MASK)
                | ((value << FifoConf::RXFIFO_WM_THRHD_OFFSET) & FifoConf::RXFIFO_WM_THRHD_MASK),
        }
    }
}

impl Default for FifoConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FifoConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoConfVal {
        FifoConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoConfVal) -> FifoConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA Register
///
/// Rx FIFO read data.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **FIFO_RDATA** `[7:0]` (RO)
///
///   Represents the value of RXFIFO read data.
#[repr(transparent)]
pub struct Data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data {
    // ======== Field Constants ========

    /// **FIFO_RDATA**
    ///
    /// Represents the value of RXFIFO read data.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const FIFO_RDATA_OFFSET: u32 = 0;
    pub const FIFO_RDATA_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **FIFO_RDATA** `[7:0]` (RO)
    ///
    /// Represents the value of RXFIFO read data.
    #[inline(always)]
    pub fn fifo_rdata(&self) -> u32 {
        (self.value.read_bits() & Self::FIFO_RDATA_MASK) >> Self::FIFO_RDATA_OFFSET
    }
}

/// DATA value type for batch read/modify/write operations
///
/// Use `data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataVal {
    bits: u32,
}

impl DataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FIFO_RDATA** `[7:0]`
    ///
    /// Represents the value of RXFIFO read data.
    #[inline(always)]
    pub const fn fifo_rdata(&self) -> u32 {
        (self.bits & Data::FIFO_RDATA_MASK) >> Data::FIFO_RDATA_OFFSET
    }
}

impl Default for DataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataVal {
        DataVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// Raw interrupt status
///
/// Address offset: 0x0020
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **SLAVE_ADDR_UNMATCH_INT_RAW** `[18]` (RO)
///
///   The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
///
/// - **GENERAL_CALL_INT_RAW** `[17]` (RO)
///
///   The raw interrupt status of I2C_GENARAL_CALL_INT interrupt.
///
/// - **SLAVE_STRETCH_INT_RAW** `[16]` (RO)
///
///   The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **DET_START_INT_RAW** `[15]` (RO)
///
///   The raw interrupt status of I2C_DET_START_INT interrupt.
///
/// - **SCL_MAIN_ST_TO_INT_RAW** `[14]` (RO)
///
///   The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt.
///
/// - **SCL_ST_TO_INT_RAW** `[13]` (RO)
///
///   The raw interrupt status of I2C_SCL_ST_TO_INT interrupt.
///
/// - **RXFIFO_UDF_INT_RAW** `[12]` (RO)
///
///   The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt.
///
/// - **TXFIFO_OVF_INT_RAW** `[11]` (RO)
///
///   The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt.
///
/// - **NACK_INT_RAW** `[10]` (RO)
///
///   The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **TRANS_START_INT_RAW** `[9]` (RO)
///
///   The raw interrupt status of the I2C_TRANS_START_INT interrupt.
///
/// - **TIME_OUT_INT_RAW** `[8]` (RO)
///
///   The raw interrupt status of the I2C_TIME_OUT_INT interrupt.
///
/// - **TRANS_COMPLETE_INT_RAW** `[7]` (RO)
///
///   The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **MST_TXFIFO_UDF_INT_RAW** `[6]` (RO)
///
///   The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **ARBITRATION_LOST_INT_RAW** `[5]` (RO)
///
///   The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt.
///
/// - **BYTE_TRANS_DONE_INT_RAW** `[4]` (RO)
///
///   The raw interrupt status of the I2C_END_DETECT_INT interrupt.
///
/// - **END_DETECT_INT_RAW** `[3]` (RO)
///
///   The raw interrupt status of the I2C_END_DETECT_INT interrupt.
///
/// - **RXFIFO_OVF_INT_RAW** `[2]` (RO)
///
///   The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt.
///
/// - **TXFIFO_WM_INT_RAW** `[1]` (RO)
///
///   The raw interrupt status of I2C_TXFIFO_WM_INT interrupt.
///
/// - **RXFIFO_WM_INT_RAW** `[0]` (RO)
///
///   The raw interrupt status of I2C_RXFIFO_WM_INT interrupt.
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

    /// **SLAVE_ADDR_UNMATCH_INT_RAW**
    ///
    /// The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SLAVE_ADDR_UNMATCH_INT_RAW_OFFSET: u32 = 18;
    pub const SLAVE_ADDR_UNMATCH_INT_RAW_MASK: u32 = 0x00040000;
    pub const SLAVE_ADDR_UNMATCH_INT_RAW: u32 = 1 << 18;

    /// **GENERAL_CALL_INT_RAW**
    ///
    /// The raw interrupt status of I2C_GENARAL_CALL_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const GENERAL_CALL_INT_RAW_OFFSET: u32 = 17;
    pub const GENERAL_CALL_INT_RAW_MASK: u32 = 0x00020000;
    pub const GENERAL_CALL_INT_RAW: u32 = 1 << 17;

    /// **SLAVE_STRETCH_INT_RAW**
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SLAVE_STRETCH_INT_RAW_OFFSET: u32 = 16;
    pub const SLAVE_STRETCH_INT_RAW_MASK: u32 = 0x00010000;
    pub const SLAVE_STRETCH_INT_RAW: u32 = 1 << 16;

    /// **DET_START_INT_RAW**
    ///
    /// The raw interrupt status of I2C_DET_START_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DET_START_INT_RAW_OFFSET: u32 = 15;
    pub const DET_START_INT_RAW_MASK: u32 = 0x00008000;
    pub const DET_START_INT_RAW: u32 = 1 << 15;

    /// **SCL_MAIN_ST_TO_INT_RAW**
    ///
    /// The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCL_MAIN_ST_TO_INT_RAW_OFFSET: u32 = 14;
    pub const SCL_MAIN_ST_TO_INT_RAW_MASK: u32 = 0x00004000;
    pub const SCL_MAIN_ST_TO_INT_RAW: u32 = 1 << 14;

    /// **SCL_ST_TO_INT_RAW**
    ///
    /// The raw interrupt status of I2C_SCL_ST_TO_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCL_ST_TO_INT_RAW_OFFSET: u32 = 13;
    pub const SCL_ST_TO_INT_RAW_MASK: u32 = 0x00002000;
    pub const SCL_ST_TO_INT_RAW: u32 = 1 << 13;

    /// **RXFIFO_UDF_INT_RAW**
    ///
    /// The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RXFIFO_UDF_INT_RAW_OFFSET: u32 = 12;
    pub const RXFIFO_UDF_INT_RAW_MASK: u32 = 0x00001000;
    pub const RXFIFO_UDF_INT_RAW: u32 = 1 << 12;

    /// **TXFIFO_OVF_INT_RAW**
    ///
    /// The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXFIFO_OVF_INT_RAW_OFFSET: u32 = 11;
    pub const TXFIFO_OVF_INT_RAW_MASK: u32 = 0x00000800;
    pub const TXFIFO_OVF_INT_RAW: u32 = 1 << 11;

    /// **NACK_INT_RAW**
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const NACK_INT_RAW_OFFSET: u32 = 10;
    pub const NACK_INT_RAW_MASK: u32 = 0x00000400;
    pub const NACK_INT_RAW: u32 = 1 << 10;

    /// **TRANS_START_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_TRANS_START_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TRANS_START_INT_RAW_OFFSET: u32 = 9;
    pub const TRANS_START_INT_RAW_MASK: u32 = 0x00000200;
    pub const TRANS_START_INT_RAW: u32 = 1 << 9;

    /// **TIME_OUT_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_TIME_OUT_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TIME_OUT_INT_RAW_OFFSET: u32 = 8;
    pub const TIME_OUT_INT_RAW_MASK: u32 = 0x00000100;
    pub const TIME_OUT_INT_RAW: u32 = 1 << 8;

    /// **TRANS_COMPLETE_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANS_COMPLETE_INT_RAW_OFFSET: u32 = 7;
    pub const TRANS_COMPLETE_INT_RAW_MASK: u32 = 0x00000080;
    pub const TRANS_COMPLETE_INT_RAW: u32 = 1 << 7;

    /// **MST_TXFIFO_UDF_INT_RAW**
    ///
    /// The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MST_TXFIFO_UDF_INT_RAW_OFFSET: u32 = 6;
    pub const MST_TXFIFO_UDF_INT_RAW_MASK: u32 = 0x00000040;
    pub const MST_TXFIFO_UDF_INT_RAW: u32 = 1 << 6;

    /// **ARBITRATION_LOST_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ARBITRATION_LOST_INT_RAW_OFFSET: u32 = 5;
    pub const ARBITRATION_LOST_INT_RAW_MASK: u32 = 0x00000020;
    pub const ARBITRATION_LOST_INT_RAW: u32 = 1 << 5;

    /// **BYTE_TRANS_DONE_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BYTE_TRANS_DONE_INT_RAW_OFFSET: u32 = 4;
    pub const BYTE_TRANS_DONE_INT_RAW_MASK: u32 = 0x00000010;
    pub const BYTE_TRANS_DONE_INT_RAW: u32 = 1 << 4;

    /// **END_DETECT_INT_RAW**
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const END_DETECT_INT_RAW_OFFSET: u32 = 3;
    pub const END_DETECT_INT_RAW_MASK: u32 = 0x00000008;
    pub const END_DETECT_INT_RAW: u32 = 1 << 3;

    /// **RXFIFO_OVF_INT_RAW**
    ///
    /// The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXFIFO_OVF_INT_RAW_OFFSET: u32 = 2;
    pub const RXFIFO_OVF_INT_RAW_MASK: u32 = 0x00000004;
    pub const RXFIFO_OVF_INT_RAW: u32 = 1 << 2;

    /// **TXFIFO_WM_INT_RAW**
    ///
    /// The raw interrupt status of I2C_TXFIFO_WM_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_WM_INT_RAW_OFFSET: u32 = 1;
    pub const TXFIFO_WM_INT_RAW_MASK: u32 = 0x00000002;
    pub const TXFIFO_WM_INT_RAW: u32 = 1 << 1;

    /// **RXFIFO_WM_INT_RAW**
    ///
    /// The raw interrupt status of I2C_RXFIFO_WM_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_WM_INT_RAW_OFFSET: u32 = 0;
    pub const RXFIFO_WM_INT_RAW_MASK: u32 = 0x00000001;
    pub const RXFIFO_WM_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLAVE_ADDR_UNMATCH_INT_RAW** `[18]` (RO)
    ///
    /// The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    #[inline(always)]
    pub fn slave_addr_unmatch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_ADDR_UNMATCH_INT_RAW_MASK) != 0
    }

    /// **GENERAL_CALL_INT_RAW** `[17]` (RO)
    ///
    /// The raw interrupt status of I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub fn general_call_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::GENERAL_CALL_INT_RAW_MASK) != 0
    }

    /// **SLAVE_STRETCH_INT_RAW** `[16]` (RO)
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn slave_stretch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_STRETCH_INT_RAW_MASK) != 0
    }

    /// **DET_START_INT_RAW** `[15]` (RO)
    ///
    /// The raw interrupt status of I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub fn det_start_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DET_START_INT_RAW_MASK) != 0
    }

    /// **SCL_MAIN_ST_TO_INT_RAW** `[14]` (RO)
    ///
    /// The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_main_st_to_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SCL_MAIN_ST_TO_INT_RAW_MASK) != 0
    }

    /// **SCL_ST_TO_INT_RAW** `[13]` (RO)
    ///
    /// The raw interrupt status of I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_st_to_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SCL_ST_TO_INT_RAW_MASK) != 0
    }

    /// **RXFIFO_UDF_INT_RAW** `[12]` (RO)
    ///
    /// The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_udf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_UDF_INT_RAW_MASK) != 0
    }

    /// **TXFIFO_OVF_INT_RAW** `[11]` (RO)
    ///
    /// The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn txfifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// **NACK_INT_RAW** `[10]` (RO)
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn nack_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::NACK_INT_RAW_MASK) != 0
    }

    /// **TRANS_START_INT_RAW** `[9]` (RO)
    ///
    /// The raw interrupt status of the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub fn trans_start_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_START_INT_RAW_MASK) != 0
    }

    /// **TIME_OUT_INT_RAW** `[8]` (RO)
    ///
    /// The raw interrupt status of the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIME_OUT_INT_RAW_MASK) != 0
    }

    /// **TRANS_COMPLETE_INT_RAW** `[7]` (RO)
    ///
    /// The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_INT_RAW_MASK) != 0
    }

    /// **MST_TXFIFO_UDF_INT_RAW** `[6]` (RO)
    ///
    /// The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn mst_txfifo_udf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::MST_TXFIFO_UDF_INT_RAW_MASK) != 0
    }

    /// **ARBITRATION_LOST_INT_RAW** `[5]` (RO)
    ///
    /// The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::ARBITRATION_LOST_INT_RAW_MASK) != 0
    }

    /// **BYTE_TRANS_DONE_INT_RAW** `[4]` (RO)
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn byte_trans_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::BYTE_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// **END_DETECT_INT_RAW** `[3]` (RO)
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn end_detect_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::END_DETECT_INT_RAW_MASK) != 0
    }

    /// **RXFIFO_OVF_INT_RAW** `[2]` (RO)
    ///
    /// The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// **TXFIFO_WM_INT_RAW** `[1]` (RO)
    ///
    /// The raw interrupt status of I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn txfifo_wm_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_WM_INT_RAW_MASK) != 0
    }

    /// **RXFIFO_WM_INT_RAW** `[0]` (RO)
    ///
    /// The raw interrupt status of I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_wm_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_WM_INT_RAW_MASK) != 0
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

    /// Read **SLAVE_ADDR_UNMATCH_INT_RAW** `[18:18]`
    ///
    /// The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    #[inline(always)]
    pub const fn slave_addr_unmatch_int_raw(&self) -> bool {
        (self.bits & IntRaw::SLAVE_ADDR_UNMATCH_INT_RAW_MASK) != 0
    }

    /// Read **GENERAL_CALL_INT_RAW** `[17:17]`
    ///
    /// The raw interrupt status of I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub const fn general_call_int_raw(&self) -> bool {
        (self.bits & IntRaw::GENERAL_CALL_INT_RAW_MASK) != 0
    }

    /// Read **SLAVE_STRETCH_INT_RAW** `[16:16]`
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn slave_stretch_int_raw(&self) -> bool {
        (self.bits & IntRaw::SLAVE_STRETCH_INT_RAW_MASK) != 0
    }

    /// Read **DET_START_INT_RAW** `[15:15]`
    ///
    /// The raw interrupt status of I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub const fn det_start_int_raw(&self) -> bool {
        (self.bits & IntRaw::DET_START_INT_RAW_MASK) != 0
    }

    /// Read **SCL_MAIN_ST_TO_INT_RAW** `[14:14]`
    ///
    /// The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_main_st_to_int_raw(&self) -> bool {
        (self.bits & IntRaw::SCL_MAIN_ST_TO_INT_RAW_MASK) != 0
    }

    /// Read **SCL_ST_TO_INT_RAW** `[13:13]`
    ///
    /// The raw interrupt status of I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_st_to_int_raw(&self) -> bool {
        (self.bits & IntRaw::SCL_ST_TO_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_UDF_INT_RAW** `[12:12]`
    ///
    /// The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_udf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_UDF_INT_RAW_MASK) != 0
    }

    /// Read **TXFIFO_OVF_INT_RAW** `[11:11]`
    ///
    /// The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::TXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **NACK_INT_RAW** `[10:10]`
    ///
    /// The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn nack_int_raw(&self) -> bool {
        (self.bits & IntRaw::NACK_INT_RAW_MASK) != 0
    }

    /// Read **TRANS_START_INT_RAW** `[9:9]`
    ///
    /// The raw interrupt status of the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub const fn trans_start_int_raw(&self) -> bool {
        (self.bits & IntRaw::TRANS_START_INT_RAW_MASK) != 0
    }

    /// Read **TIME_OUT_INT_RAW** `[8:8]`
    ///
    /// The raw interrupt status of the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub const fn time_out_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIME_OUT_INT_RAW_MASK) != 0
    }

    /// Read **TRANS_COMPLETE_INT_RAW** `[7:7]`
    ///
    /// The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn trans_complete_int_raw(&self) -> bool {
        (self.bits & IntRaw::TRANS_COMPLETE_INT_RAW_MASK) != 0
    }

    /// Read **MST_TXFIFO_UDF_INT_RAW** `[6:6]`
    ///
    /// The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn mst_txfifo_udf_int_raw(&self) -> bool {
        (self.bits & IntRaw::MST_TXFIFO_UDF_INT_RAW_MASK) != 0
    }

    /// Read **ARBITRATION_LOST_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub const fn arbitration_lost_int_raw(&self) -> bool {
        (self.bits & IntRaw::ARBITRATION_LOST_INT_RAW_MASK) != 0
    }

    /// Read **BYTE_TRANS_DONE_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn byte_trans_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::BYTE_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Read **END_DETECT_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn end_detect_int_raw(&self) -> bool {
        (self.bits & IntRaw::END_DETECT_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **TXFIFO_WM_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status of I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_wm_int_raw(&self) -> bool {
        (self.bits & IntRaw::TXFIFO_WM_INT_RAW_MASK) != 0
    }

    /// Read **RXFIFO_WM_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status of I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_wm_int_raw(&self) -> bool {
        (self.bits & IntRaw::RXFIFO_WM_INT_RAW_MASK) != 0
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
}

/// INT_CLR Register
///
/// Interrupt clear bits
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SLAVE_ADDR_UNMATCH_INT_CLR** `[18]` (WO)
///
///   Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
///
/// - **GENERAL_CALL_INT_CLR** `[17]` (WO)
///
///   Write 1 to clear I2C_GENARAL_CALL_INT interrupt.
///
/// - **SLAVE_STRETCH_INT_CLR** `[16]` (WO)
///
///   Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **DET_START_INT_CLR** `[15]` (WO)
///
///   Write 1 to clear I2C_DET_START_INT interrupt.
///
/// - **SCL_MAIN_ST_TO_INT_CLR** `[14]` (WO)
///
///   Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt.
///
/// - **SCL_ST_TO_INT_CLR** `[13]` (WO)
///
///   Write 1 to clear I2C_SCL_ST_TO_INT interrupt.
///
/// - **RXFIFO_UDF_INT_CLR** `[12]` (WO)
///
///   Write 1 to clear I2C_RXFIFO_UDF_INT interrupt.
///
/// - **TXFIFO_OVF_INT_CLR** `[11]` (WO)
///
///   Write 1 to clear I2C_TXFIFO_OVF_INT interrupt.
///
/// - **NACK_INT_CLR** `[10]` (WO)
///
///   Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **TRANS_START_INT_CLR** `[9]` (WO)
///
///   Write 1 to clear the I2C_TRANS_START_INT interrupt.
///
/// - **TIME_OUT_INT_CLR** `[8]` (WO)
///
///   Write 1 to clear the I2C_TIME_OUT_INT interrupt.
///
/// - **TRANS_COMPLETE_INT_CLR** `[7]` (WO)
///
///   Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **MST_TXFIFO_UDF_INT_CLR** `[6]` (WO)
///
///   Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **ARBITRATION_LOST_INT_CLR** `[5]` (WO)
///
///   Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt.
///
/// - **BYTE_TRANS_DONE_INT_CLR** `[4]` (WO)
///
///   Write 1 to clear the I2C_END_DETECT_INT interrupt.
///
/// - **END_DETECT_INT_CLR** `[3]` (WO)
///
///   Write 1 to clear the I2C_END_DETECT_INT interrupt.
///
/// - **RXFIFO_OVF_INT_CLR** `[2]` (WO)
///
///   Write 1 to clear I2C_RXFIFO_OVF_INT interrupt.
///
/// - **TXFIFO_WM_INT_CLR** `[1]` (WO)
///
///   Write 1 to clear I2C_TXFIFO_WM_INT interrupt.
///
/// - **RXFIFO_WM_INT_CLR** `[0]` (WO)
///
///   Write 1 to clear I2C_RXFIFO_WM_INT interrupt.
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

    /// **SLAVE_ADDR_UNMATCH_INT_CLR**
    ///
    /// Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SLAVE_ADDR_UNMATCH_INT_CLR_OFFSET: u32 = 18;
    pub const SLAVE_ADDR_UNMATCH_INT_CLR_MASK: u32 = 0x00040000;
    pub const SLAVE_ADDR_UNMATCH_INT_CLR: u32 = 1 << 18;

    /// **GENERAL_CALL_INT_CLR**
    ///
    /// Write 1 to clear I2C_GENARAL_CALL_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const GENERAL_CALL_INT_CLR_OFFSET: u32 = 17;
    pub const GENERAL_CALL_INT_CLR_MASK: u32 = 0x00020000;
    pub const GENERAL_CALL_INT_CLR: u32 = 1 << 17;

    /// **SLAVE_STRETCH_INT_CLR**
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SLAVE_STRETCH_INT_CLR_OFFSET: u32 = 16;
    pub const SLAVE_STRETCH_INT_CLR_MASK: u32 = 0x00010000;
    pub const SLAVE_STRETCH_INT_CLR: u32 = 1 << 16;

    /// **DET_START_INT_CLR**
    ///
    /// Write 1 to clear I2C_DET_START_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DET_START_INT_CLR_OFFSET: u32 = 15;
    pub const DET_START_INT_CLR_MASK: u32 = 0x00008000;
    pub const DET_START_INT_CLR: u32 = 1 << 15;

    /// **SCL_MAIN_ST_TO_INT_CLR**
    ///
    /// Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCL_MAIN_ST_TO_INT_CLR_OFFSET: u32 = 14;
    pub const SCL_MAIN_ST_TO_INT_CLR_MASK: u32 = 0x00004000;
    pub const SCL_MAIN_ST_TO_INT_CLR: u32 = 1 << 14;

    /// **SCL_ST_TO_INT_CLR**
    ///
    /// Write 1 to clear I2C_SCL_ST_TO_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCL_ST_TO_INT_CLR_OFFSET: u32 = 13;
    pub const SCL_ST_TO_INT_CLR_MASK: u32 = 0x00002000;
    pub const SCL_ST_TO_INT_CLR: u32 = 1 << 13;

    /// **RXFIFO_UDF_INT_CLR**
    ///
    /// Write 1 to clear I2C_RXFIFO_UDF_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RXFIFO_UDF_INT_CLR_OFFSET: u32 = 12;
    pub const RXFIFO_UDF_INT_CLR_MASK: u32 = 0x00001000;
    pub const RXFIFO_UDF_INT_CLR: u32 = 1 << 12;

    /// **TXFIFO_OVF_INT_CLR**
    ///
    /// Write 1 to clear I2C_TXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXFIFO_OVF_INT_CLR_OFFSET: u32 = 11;
    pub const TXFIFO_OVF_INT_CLR_MASK: u32 = 0x00000800;
    pub const TXFIFO_OVF_INT_CLR: u32 = 1 << 11;

    /// **NACK_INT_CLR**
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const NACK_INT_CLR_OFFSET: u32 = 10;
    pub const NACK_INT_CLR_MASK: u32 = 0x00000400;
    pub const NACK_INT_CLR: u32 = 1 << 10;

    /// **TRANS_START_INT_CLR**
    ///
    /// Write 1 to clear the I2C_TRANS_START_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TRANS_START_INT_CLR_OFFSET: u32 = 9;
    pub const TRANS_START_INT_CLR_MASK: u32 = 0x00000200;
    pub const TRANS_START_INT_CLR: u32 = 1 << 9;

    /// **TIME_OUT_INT_CLR**
    ///
    /// Write 1 to clear the I2C_TIME_OUT_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TIME_OUT_INT_CLR_OFFSET: u32 = 8;
    pub const TIME_OUT_INT_CLR_MASK: u32 = 0x00000100;
    pub const TIME_OUT_INT_CLR: u32 = 1 << 8;

    /// **TRANS_COMPLETE_INT_CLR**
    ///
    /// Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANS_COMPLETE_INT_CLR_OFFSET: u32 = 7;
    pub const TRANS_COMPLETE_INT_CLR_MASK: u32 = 0x00000080;
    pub const TRANS_COMPLETE_INT_CLR: u32 = 1 << 7;

    /// **MST_TXFIFO_UDF_INT_CLR**
    ///
    /// Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MST_TXFIFO_UDF_INT_CLR_OFFSET: u32 = 6;
    pub const MST_TXFIFO_UDF_INT_CLR_MASK: u32 = 0x00000040;
    pub const MST_TXFIFO_UDF_INT_CLR: u32 = 1 << 6;

    /// **ARBITRATION_LOST_INT_CLR**
    ///
    /// Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ARBITRATION_LOST_INT_CLR_OFFSET: u32 = 5;
    pub const ARBITRATION_LOST_INT_CLR_MASK: u32 = 0x00000020;
    pub const ARBITRATION_LOST_INT_CLR: u32 = 1 << 5;

    /// **BYTE_TRANS_DONE_INT_CLR**
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BYTE_TRANS_DONE_INT_CLR_OFFSET: u32 = 4;
    pub const BYTE_TRANS_DONE_INT_CLR_MASK: u32 = 0x00000010;
    pub const BYTE_TRANS_DONE_INT_CLR: u32 = 1 << 4;

    /// **END_DETECT_INT_CLR**
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const END_DETECT_INT_CLR_OFFSET: u32 = 3;
    pub const END_DETECT_INT_CLR_MASK: u32 = 0x00000008;
    pub const END_DETECT_INT_CLR: u32 = 1 << 3;

    /// **RXFIFO_OVF_INT_CLR**
    ///
    /// Write 1 to clear I2C_RXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXFIFO_OVF_INT_CLR_OFFSET: u32 = 2;
    pub const RXFIFO_OVF_INT_CLR_MASK: u32 = 0x00000004;
    pub const RXFIFO_OVF_INT_CLR: u32 = 1 << 2;

    /// **TXFIFO_WM_INT_CLR**
    ///
    /// Write 1 to clear I2C_TXFIFO_WM_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_WM_INT_CLR_OFFSET: u32 = 1;
    pub const TXFIFO_WM_INT_CLR_MASK: u32 = 0x00000002;
    pub const TXFIFO_WM_INT_CLR: u32 = 1 << 1;

    /// **RXFIFO_WM_INT_CLR**
    ///
    /// Write 1 to clear I2C_RXFIFO_WM_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_WM_INT_CLR_OFFSET: u32 = 0;
    pub const RXFIFO_WM_INT_CLR_MASK: u32 = 0x00000001;
    pub const RXFIFO_WM_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SLAVE_ADDR_UNMATCH_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    #[inline(always)]
    pub fn trigger_slave_addr_unmatch_int_clr(&self) {
        self.value.write_bits(Self::SLAVE_ADDR_UNMATCH_INT_CLR_MASK);
    }

    /// Trigger **GENERAL_CALL_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub fn trigger_general_call_int_clr(&self) {
        self.value.write_bits(Self::GENERAL_CALL_INT_CLR_MASK);
    }

    /// Trigger **SLAVE_STRETCH_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn trigger_slave_stretch_int_clr(&self) {
        self.value.write_bits(Self::SLAVE_STRETCH_INT_CLR_MASK);
    }

    /// Trigger **DET_START_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub fn trigger_det_start_int_clr(&self) {
        self.value.write_bits(Self::DET_START_INT_CLR_MASK);
    }

    /// Trigger **SCL_MAIN_ST_TO_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn trigger_scl_main_st_to_int_clr(&self) {
        self.value.write_bits(Self::SCL_MAIN_ST_TO_INT_CLR_MASK);
    }

    /// Trigger **SCL_ST_TO_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn trigger_scl_st_to_int_clr(&self) {
        self.value.write_bits(Self::SCL_ST_TO_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_UDF_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_udf_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_UDF_INT_CLR_MASK);
    }

    /// Trigger **TXFIFO_OVF_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn trigger_txfifo_ovf_int_clr(&self) {
        self.value.write_bits(Self::TXFIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **NACK_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn trigger_nack_int_clr(&self) {
        self.value.write_bits(Self::NACK_INT_CLR_MASK);
    }

    /// Trigger **TRANS_START_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub fn trigger_trans_start_int_clr(&self) {
        self.value.write_bits(Self::TRANS_START_INT_CLR_MASK);
    }

    /// Trigger **TIME_OUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub fn trigger_time_out_int_clr(&self) {
        self.value.write_bits(Self::TIME_OUT_INT_CLR_MASK);
    }

    /// Trigger **TRANS_COMPLETE_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trigger_trans_complete_int_clr(&self) {
        self.value.write_bits(Self::TRANS_COMPLETE_INT_CLR_MASK);
    }

    /// Trigger **MST_TXFIFO_UDF_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trigger_mst_txfifo_udf_int_clr(&self) {
        self.value.write_bits(Self::MST_TXFIFO_UDF_INT_CLR_MASK);
    }

    /// Trigger **ARBITRATION_LOST_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub fn trigger_arbitration_lost_int_clr(&self) {
        self.value.write_bits(Self::ARBITRATION_LOST_INT_CLR_MASK);
    }

    /// Trigger **BYTE_TRANS_DONE_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn trigger_byte_trans_done_int_clr(&self) {
        self.value.write_bits(Self::BYTE_TRANS_DONE_INT_CLR_MASK);
    }

    /// Trigger **END_DETECT_INT_CLR** (WO)
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn trigger_end_detect_int_clr(&self) {
        self.value.write_bits(Self::END_DETECT_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_OVF_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_ovf_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **TXFIFO_WM_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn trigger_txfifo_wm_int_clr(&self) {
        self.value.write_bits(Self::TXFIFO_WM_INT_CLR_MASK);
    }

    /// Trigger **RXFIFO_WM_INT_CLR** (WO)
    ///
    /// Write 1 to clear I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn trigger_rxfifo_wm_int_clr(&self) {
        self.value.write_bits(Self::RXFIFO_WM_INT_CLR_MASK);
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

    /// Set **SLAVE_ADDR_UNMATCH_INT_CLR** `[18:18]`
    ///
    /// Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt.
    #[inline(always)]
    pub const fn with_slave_addr_unmatch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SLAVE_ADDR_UNMATCH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SLAVE_ADDR_UNMATCH_INT_CLR_MASK
            },
        }
    }

    /// Set **GENERAL_CALL_INT_CLR** `[17:17]`
    ///
    /// Write 1 to clear I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub const fn with_general_call_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::GENERAL_CALL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::GENERAL_CALL_INT_CLR_MASK
            },
        }
    }

    /// Set **SLAVE_STRETCH_INT_CLR** `[16:16]`
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn with_slave_stretch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SLAVE_STRETCH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SLAVE_STRETCH_INT_CLR_MASK
            },
        }
    }

    /// Set **DET_START_INT_CLR** `[15:15]`
    ///
    /// Write 1 to clear I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub const fn with_det_start_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DET_START_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DET_START_INT_CLR_MASK
            },
        }
    }

    /// Set **SCL_MAIN_ST_TO_INT_CLR** `[14:14]`
    ///
    /// Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn with_scl_main_st_to_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SCL_MAIN_ST_TO_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SCL_MAIN_ST_TO_INT_CLR_MASK
            },
        }
    }

    /// Set **SCL_ST_TO_INT_CLR** `[13:13]`
    ///
    /// Write 1 to clear I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn with_scl_st_to_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SCL_ST_TO_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SCL_ST_TO_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_UDF_INT_CLR** `[12:12]`
    ///
    /// Write 1 to clear I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_udf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RXFIFO_UDF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RXFIFO_UDF_INT_CLR_MASK
            },
        }
    }

    /// Set **TXFIFO_OVF_INT_CLR** `[11:11]`
    ///
    /// Write 1 to clear I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_txfifo_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TXFIFO_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TXFIFO_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **NACK_INT_CLR** `[10:10]`
    ///
    /// Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn with_nack_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::NACK_INT_CLR_MASK
            } else {
                self.bits & !IntClr::NACK_INT_CLR_MASK
            },
        }
    }

    /// Set **TRANS_START_INT_CLR** `[9:9]`
    ///
    /// Write 1 to clear the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub const fn with_trans_start_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TRANS_START_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TRANS_START_INT_CLR_MASK
            },
        }
    }

    /// Set **TIME_OUT_INT_CLR** `[8:8]`
    ///
    /// Write 1 to clear the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub const fn with_time_out_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIME_OUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIME_OUT_INT_CLR_MASK
            },
        }
    }

    /// Set **TRANS_COMPLETE_INT_CLR** `[7:7]`
    ///
    /// Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn with_trans_complete_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TRANS_COMPLETE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TRANS_COMPLETE_INT_CLR_MASK
            },
        }
    }

    /// Set **MST_TXFIFO_UDF_INT_CLR** `[6:6]`
    ///
    /// Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn with_mst_txfifo_udf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::MST_TXFIFO_UDF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::MST_TXFIFO_UDF_INT_CLR_MASK
            },
        }
    }

    /// Set **ARBITRATION_LOST_INT_CLR** `[5:5]`
    ///
    /// Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub const fn with_arbitration_lost_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::ARBITRATION_LOST_INT_CLR_MASK
            } else {
                self.bits & !IntClr::ARBITRATION_LOST_INT_CLR_MASK
            },
        }
    }

    /// Set **BYTE_TRANS_DONE_INT_CLR** `[4:4]`
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn with_byte_trans_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::BYTE_TRANS_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::BYTE_TRANS_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **END_DETECT_INT_CLR** `[3:3]`
    ///
    /// Write 1 to clear the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn with_end_detect_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::END_DETECT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::END_DETECT_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_OVF_INT_CLR** `[2:2]`
    ///
    /// Write 1 to clear I2C_RXFIFO_OVF_INT interrupt.
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

    /// Set **TXFIFO_WM_INT_CLR** `[1:1]`
    ///
    /// Write 1 to clear I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn with_txfifo_wm_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TXFIFO_WM_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TXFIFO_WM_INT_CLR_MASK
            },
        }
    }

    /// Set **RXFIFO_WM_INT_CLR** `[0:0]`
    ///
    /// Write 1 to clear I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_wm_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RXFIFO_WM_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RXFIFO_WM_INT_CLR_MASK
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

/// INT_ENA Register
///
/// Interrupt enable bits
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SLAVE_ADDR_UNMATCH_INT_ENA** `[18]` (RW)
///
///   Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
///
/// - **GENERAL_CALL_INT_ENA** `[17]` (RW)
///
///   Write 1 to enable I2C_GENARAL_CALL_INT interrupt.
///
/// - **SLAVE_STRETCH_INT_ENA** `[16]` (RW)
///
///   Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **DET_START_INT_ENA** `[15]` (RW)
///
///   Write 1 to enable I2C_DET_START_INT interrupt.
///
/// - **SCL_MAIN_ST_TO_INT_ENA** `[14]` (RW)
///
///   Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt.
///
/// - **SCL_ST_TO_INT_ENA** `[13]` (RW)
///
///   Write 1 to enable I2C_SCL_ST_TO_INT interrupt.
///
/// - **RXFIFO_UDF_INT_ENA** `[12]` (RW)
///
///   Write 1 to enable I2C_RXFIFO_UDF_INT interrupt.
///
/// - **TXFIFO_OVF_INT_ENA** `[11]` (RW)
///
///   Write 1 to enable I2C_TXFIFO_OVF_INT interrupt.
///
/// - **NACK_INT_ENA** `[10]` (RW)
///
///   Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **TRANS_START_INT_ENA** `[9]` (RW)
///
///   Write 1 to enable the I2C_TRANS_START_INT interrupt.
///
/// - **TIME_OUT_INT_ENA** `[8]` (RW)
///
///   Write 1 to enable the I2C_TIME_OUT_INT interrupt.
///
/// - **TRANS_COMPLETE_INT_ENA** `[7]` (RW)
///
///   Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **MST_TXFIFO_UDF_INT_ENA** `[6]` (RW)
///
///   Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **ARBITRATION_LOST_INT_ENA** `[5]` (RW)
///
///   Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt.
///
/// - **BYTE_TRANS_DONE_INT_ENA** `[4]` (RW)
///
///   Write 1 to enable the I2C_END_DETECT_INT interrupt.
///
/// - **END_DETECT_INT_ENA** `[3]` (RW)
///
///   Write 1 to enable the I2C_END_DETECT_INT interrupt.
///
/// - **RXFIFO_OVF_INT_ENA** `[2]` (RW)
///
///   Write 1 to enable I2C_RXFIFO_OVF_INT interrupt.
///
/// - **TXFIFO_WM_INT_ENA** `[1]` (RW)
///
///   Write 1 to enable I2C_TXFIFO_WM_INT interrupt.
///
/// - **RXFIFO_WM_INT_ENA** `[0]` (RW)
///
///   Write 1 to enable I2C_RXFIFO_WM_INT interrupt.
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

    /// **SLAVE_ADDR_UNMATCH_INT_ENA**
    ///
    /// Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SLAVE_ADDR_UNMATCH_INT_ENA_OFFSET: u32 = 18;
    pub const SLAVE_ADDR_UNMATCH_INT_ENA_MASK: u32 = 0x00040000;
    pub const SLAVE_ADDR_UNMATCH_INT_ENA: u32 = 1 << 18;

    /// **GENERAL_CALL_INT_ENA**
    ///
    /// Write 1 to enable I2C_GENARAL_CALL_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const GENERAL_CALL_INT_ENA_OFFSET: u32 = 17;
    pub const GENERAL_CALL_INT_ENA_MASK: u32 = 0x00020000;
    pub const GENERAL_CALL_INT_ENA: u32 = 1 << 17;

    /// **SLAVE_STRETCH_INT_ENA**
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SLAVE_STRETCH_INT_ENA_OFFSET: u32 = 16;
    pub const SLAVE_STRETCH_INT_ENA_MASK: u32 = 0x00010000;
    pub const SLAVE_STRETCH_INT_ENA: u32 = 1 << 16;

    /// **DET_START_INT_ENA**
    ///
    /// Write 1 to enable I2C_DET_START_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DET_START_INT_ENA_OFFSET: u32 = 15;
    pub const DET_START_INT_ENA_MASK: u32 = 0x00008000;
    pub const DET_START_INT_ENA: u32 = 1 << 15;

    /// **SCL_MAIN_ST_TO_INT_ENA**
    ///
    /// Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCL_MAIN_ST_TO_INT_ENA_OFFSET: u32 = 14;
    pub const SCL_MAIN_ST_TO_INT_ENA_MASK: u32 = 0x00004000;
    pub const SCL_MAIN_ST_TO_INT_ENA: u32 = 1 << 14;

    /// **SCL_ST_TO_INT_ENA**
    ///
    /// Write 1 to enable I2C_SCL_ST_TO_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCL_ST_TO_INT_ENA_OFFSET: u32 = 13;
    pub const SCL_ST_TO_INT_ENA_MASK: u32 = 0x00002000;
    pub const SCL_ST_TO_INT_ENA: u32 = 1 << 13;

    /// **RXFIFO_UDF_INT_ENA**
    ///
    /// Write 1 to enable I2C_RXFIFO_UDF_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RXFIFO_UDF_INT_ENA_OFFSET: u32 = 12;
    pub const RXFIFO_UDF_INT_ENA_MASK: u32 = 0x00001000;
    pub const RXFIFO_UDF_INT_ENA: u32 = 1 << 12;

    /// **TXFIFO_OVF_INT_ENA**
    ///
    /// Write 1 to enable I2C_TXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXFIFO_OVF_INT_ENA_OFFSET: u32 = 11;
    pub const TXFIFO_OVF_INT_ENA_MASK: u32 = 0x00000800;
    pub const TXFIFO_OVF_INT_ENA: u32 = 1 << 11;

    /// **NACK_INT_ENA**
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const NACK_INT_ENA_OFFSET: u32 = 10;
    pub const NACK_INT_ENA_MASK: u32 = 0x00000400;
    pub const NACK_INT_ENA: u32 = 1 << 10;

    /// **TRANS_START_INT_ENA**
    ///
    /// Write 1 to enable the I2C_TRANS_START_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TRANS_START_INT_ENA_OFFSET: u32 = 9;
    pub const TRANS_START_INT_ENA_MASK: u32 = 0x00000200;
    pub const TRANS_START_INT_ENA: u32 = 1 << 9;

    /// **TIME_OUT_INT_ENA**
    ///
    /// Write 1 to enable the I2C_TIME_OUT_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TIME_OUT_INT_ENA_OFFSET: u32 = 8;
    pub const TIME_OUT_INT_ENA_MASK: u32 = 0x00000100;
    pub const TIME_OUT_INT_ENA: u32 = 1 << 8;

    /// **TRANS_COMPLETE_INT_ENA**
    ///
    /// Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANS_COMPLETE_INT_ENA_OFFSET: u32 = 7;
    pub const TRANS_COMPLETE_INT_ENA_MASK: u32 = 0x00000080;
    pub const TRANS_COMPLETE_INT_ENA: u32 = 1 << 7;

    /// **MST_TXFIFO_UDF_INT_ENA**
    ///
    /// Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MST_TXFIFO_UDF_INT_ENA_OFFSET: u32 = 6;
    pub const MST_TXFIFO_UDF_INT_ENA_MASK: u32 = 0x00000040;
    pub const MST_TXFIFO_UDF_INT_ENA: u32 = 1 << 6;

    /// **ARBITRATION_LOST_INT_ENA**
    ///
    /// Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ARBITRATION_LOST_INT_ENA_OFFSET: u32 = 5;
    pub const ARBITRATION_LOST_INT_ENA_MASK: u32 = 0x00000020;
    pub const ARBITRATION_LOST_INT_ENA: u32 = 1 << 5;

    /// **BYTE_TRANS_DONE_INT_ENA**
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BYTE_TRANS_DONE_INT_ENA_OFFSET: u32 = 4;
    pub const BYTE_TRANS_DONE_INT_ENA_MASK: u32 = 0x00000010;
    pub const BYTE_TRANS_DONE_INT_ENA: u32 = 1 << 4;

    /// **END_DETECT_INT_ENA**
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const END_DETECT_INT_ENA_OFFSET: u32 = 3;
    pub const END_DETECT_INT_ENA_MASK: u32 = 0x00000008;
    pub const END_DETECT_INT_ENA: u32 = 1 << 3;

    /// **RXFIFO_OVF_INT_ENA**
    ///
    /// Write 1 to enable I2C_RXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXFIFO_OVF_INT_ENA_OFFSET: u32 = 2;
    pub const RXFIFO_OVF_INT_ENA_MASK: u32 = 0x00000004;
    pub const RXFIFO_OVF_INT_ENA: u32 = 1 << 2;

    /// **TXFIFO_WM_INT_ENA**
    ///
    /// Write 1 to enable I2C_TXFIFO_WM_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_WM_INT_ENA_OFFSET: u32 = 1;
    pub const TXFIFO_WM_INT_ENA_MASK: u32 = 0x00000002;
    pub const TXFIFO_WM_INT_ENA: u32 = 1 << 1;

    /// **RXFIFO_WM_INT_ENA**
    ///
    /// Write 1 to enable I2C_RXFIFO_WM_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_WM_INT_ENA_OFFSET: u32 = 0;
    pub const RXFIFO_WM_INT_ENA_MASK: u32 = 0x00000001;
    pub const RXFIFO_WM_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLAVE_ADDR_UNMATCH_INT_ENA** `[18]` (RW)
    ///
    /// Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    #[inline(always)]
    pub fn slave_addr_unmatch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_ADDR_UNMATCH_INT_ENA_MASK) != 0
    }

    /// Set SLAVE_ADDR_UNMATCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slave_addr_unmatch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_ADDR_UNMATCH_INT_ENA_MASK);
    }

    /// Clear SLAVE_ADDR_UNMATCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slave_addr_unmatch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLAVE_ADDR_UNMATCH_INT_ENA_MASK);
    }

    /// **GENERAL_CALL_INT_ENA** `[17]` (RW)
    ///
    /// Write 1 to enable I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub fn general_call_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::GENERAL_CALL_INT_ENA_MASK) != 0
    }

    /// Set GENERAL_CALL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_general_call_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::GENERAL_CALL_INT_ENA_MASK);
    }

    /// Clear GENERAL_CALL_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_general_call_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::GENERAL_CALL_INT_ENA_MASK);
    }

    /// **SLAVE_STRETCH_INT_ENA** `[16]` (RW)
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn slave_stretch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_STRETCH_INT_ENA_MASK) != 0
    }

    /// Set SLAVE_STRETCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slave_stretch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_STRETCH_INT_ENA_MASK);
    }

    /// Clear SLAVE_STRETCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slave_stretch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLAVE_STRETCH_INT_ENA_MASK);
    }

    /// **DET_START_INT_ENA** `[15]` (RW)
    ///
    /// Write 1 to enable I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub fn det_start_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DET_START_INT_ENA_MASK) != 0
    }

    /// Set DET_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_det_start_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DET_START_INT_ENA_MASK);
    }

    /// Clear DET_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_det_start_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DET_START_INT_ENA_MASK);
    }

    /// **SCL_MAIN_ST_TO_INT_ENA** `[14]` (RW)
    ///
    /// Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SCL_MAIN_ST_TO_INT_ENA_MASK) != 0
    }

    /// Set SCL_MAIN_ST_TO_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_main_st_to_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SCL_MAIN_ST_TO_INT_ENA_MASK);
    }

    /// Clear SCL_MAIN_ST_TO_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_main_st_to_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCL_MAIN_ST_TO_INT_ENA_MASK);
    }

    /// **SCL_ST_TO_INT_ENA** `[13]` (RW)
    ///
    /// Write 1 to enable I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_st_to_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SCL_ST_TO_INT_ENA_MASK) != 0
    }

    /// Set SCL_ST_TO_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_st_to_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SCL_ST_TO_INT_ENA_MASK);
    }

    /// Clear SCL_ST_TO_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_st_to_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCL_ST_TO_INT_ENA_MASK);
    }

    /// **RXFIFO_UDF_INT_ENA** `[12]` (RW)
    ///
    /// Write 1 to enable I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_UDF_INT_ENA_MASK) != 0
    }

    /// Set RXFIFO_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RXFIFO_UDF_INT_ENA_MASK);
    }

    /// Clear RXFIFO_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_UDF_INT_ENA_MASK);
    }

    /// **TXFIFO_OVF_INT_ENA** `[11]` (RW)
    ///
    /// Write 1 to enable I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Set TXFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TXFIFO_OVF_INT_ENA_MASK);
    }

    /// Clear TXFIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TXFIFO_OVF_INT_ENA_MASK);
    }

    /// **NACK_INT_ENA** `[10]` (RW)
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn nack_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::NACK_INT_ENA_MASK) != 0
    }

    /// Set NACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nack_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::NACK_INT_ENA_MASK);
    }

    /// Clear NACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nack_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::NACK_INT_ENA_MASK);
    }

    /// **TRANS_START_INT_ENA** `[9]` (RW)
    ///
    /// Write 1 to enable the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub fn trans_start_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_START_INT_ENA_MASK) != 0
    }

    /// Set TRANS_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_start_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANS_START_INT_ENA_MASK);
    }

    /// Clear TRANS_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_start_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANS_START_INT_ENA_MASK);
    }

    /// **TIME_OUT_INT_ENA** `[8]` (RW)
    ///
    /// Write 1 to enable the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIME_OUT_INT_ENA_MASK) != 0
    }

    /// Set TIME_OUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_time_out_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TIME_OUT_INT_ENA_MASK);
    }

    /// Clear TIME_OUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_time_out_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TIME_OUT_INT_ENA_MASK);
    }

    /// **TRANS_COMPLETE_INT_ENA** `[7]` (RW)
    ///
    /// Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_INT_ENA_MASK) != 0
    }

    /// Set TRANS_COMPLETE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_complete_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANS_COMPLETE_INT_ENA_MASK);
    }

    /// Clear TRANS_COMPLETE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_complete_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANS_COMPLETE_INT_ENA_MASK);
    }

    /// **MST_TXFIFO_UDF_INT_ENA** `[6]` (RW)
    ///
    /// Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::MST_TXFIFO_UDF_INT_ENA_MASK) != 0
    }

    /// Set MST_TXFIFO_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mst_txfifo_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::MST_TXFIFO_UDF_INT_ENA_MASK);
    }

    /// Clear MST_TXFIFO_UDF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mst_txfifo_udf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MST_TXFIFO_UDF_INT_ENA_MASK);
    }

    /// **ARBITRATION_LOST_INT_ENA** `[5]` (RW)
    ///
    /// Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt.
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

    /// **BYTE_TRANS_DONE_INT_ENA** `[4]` (RW)
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::BYTE_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Set BYTE_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_byte_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::BYTE_TRANS_DONE_INT_ENA_MASK);
    }

    /// Clear BYTE_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_byte_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::BYTE_TRANS_DONE_INT_ENA_MASK);
    }

    /// **END_DETECT_INT_ENA** `[3]` (RW)
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn end_detect_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::END_DETECT_INT_ENA_MASK) != 0
    }

    /// Set END_DETECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_end_detect_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::END_DETECT_INT_ENA_MASK);
    }

    /// Clear END_DETECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_end_detect_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::END_DETECT_INT_ENA_MASK);
    }

    /// **RXFIFO_OVF_INT_ENA** `[2]` (RW)
    ///
    /// Write 1 to enable I2C_RXFIFO_OVF_INT interrupt.
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

    /// **TXFIFO_WM_INT_ENA** `[1]` (RW)
    ///
    /// Write 1 to enable I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_WM_INT_ENA_MASK) != 0
    }

    /// Set TXFIFO_WM_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txfifo_wm_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TXFIFO_WM_INT_ENA_MASK);
    }

    /// Clear TXFIFO_WM_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txfifo_wm_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TXFIFO_WM_INT_ENA_MASK);
    }

    /// **RXFIFO_WM_INT_ENA** `[0]` (RW)
    ///
    /// Write 1 to enable I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_WM_INT_ENA_MASK) != 0
    }

    /// Set RXFIFO_WM_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxfifo_wm_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::RXFIFO_WM_INT_ENA_MASK);
    }

    /// Clear RXFIFO_WM_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxfifo_wm_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RXFIFO_WM_INT_ENA_MASK);
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

    /// Read **SLAVE_ADDR_UNMATCH_INT_ENA** `[18:18]`
    ///
    /// Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    #[inline(always)]
    pub const fn slave_addr_unmatch_int_ena(&self) -> bool {
        (self.bits & IntEna::SLAVE_ADDR_UNMATCH_INT_ENA_MASK) != 0
    }

    /// Read **GENERAL_CALL_INT_ENA** `[17:17]`
    ///
    /// Write 1 to enable I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub const fn general_call_int_ena(&self) -> bool {
        (self.bits & IntEna::GENERAL_CALL_INT_ENA_MASK) != 0
    }

    /// Read **SLAVE_STRETCH_INT_ENA** `[16:16]`
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn slave_stretch_int_ena(&self) -> bool {
        (self.bits & IntEna::SLAVE_STRETCH_INT_ENA_MASK) != 0
    }

    /// Read **DET_START_INT_ENA** `[15:15]`
    ///
    /// Write 1 to enable I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub const fn det_start_int_ena(&self) -> bool {
        (self.bits & IntEna::DET_START_INT_ENA_MASK) != 0
    }

    /// Read **SCL_MAIN_ST_TO_INT_ENA** `[14:14]`
    ///
    /// Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_main_st_to_int_ena(&self) -> bool {
        (self.bits & IntEna::SCL_MAIN_ST_TO_INT_ENA_MASK) != 0
    }

    /// Read **SCL_ST_TO_INT_ENA** `[13:13]`
    ///
    /// Write 1 to enable I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_st_to_int_ena(&self) -> bool {
        (self.bits & IntEna::SCL_ST_TO_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_UDF_INT_ENA** `[12:12]`
    ///
    /// Write 1 to enable I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_udf_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_UDF_INT_ENA_MASK) != 0
    }

    /// Read **TXFIFO_OVF_INT_ENA** `[11:11]`
    ///
    /// Write 1 to enable I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::TXFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **NACK_INT_ENA** `[10:10]`
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn nack_int_ena(&self) -> bool {
        (self.bits & IntEna::NACK_INT_ENA_MASK) != 0
    }

    /// Read **TRANS_START_INT_ENA** `[9:9]`
    ///
    /// Write 1 to enable the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub const fn trans_start_int_ena(&self) -> bool {
        (self.bits & IntEna::TRANS_START_INT_ENA_MASK) != 0
    }

    /// Read **TIME_OUT_INT_ENA** `[8:8]`
    ///
    /// Write 1 to enable the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub const fn time_out_int_ena(&self) -> bool {
        (self.bits & IntEna::TIME_OUT_INT_ENA_MASK) != 0
    }

    /// Read **TRANS_COMPLETE_INT_ENA** `[7:7]`
    ///
    /// Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn trans_complete_int_ena(&self) -> bool {
        (self.bits & IntEna::TRANS_COMPLETE_INT_ENA_MASK) != 0
    }

    /// Read **MST_TXFIFO_UDF_INT_ENA** `[6:6]`
    ///
    /// Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn mst_txfifo_udf_int_ena(&self) -> bool {
        (self.bits & IntEna::MST_TXFIFO_UDF_INT_ENA_MASK) != 0
    }

    /// Read **ARBITRATION_LOST_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub const fn arbitration_lost_int_ena(&self) -> bool {
        (self.bits & IntEna::ARBITRATION_LOST_INT_ENA_MASK) != 0
    }

    /// Read **BYTE_TRANS_DONE_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn byte_trans_done_int_ena(&self) -> bool {
        (self.bits & IntEna::BYTE_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Read **END_DETECT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn end_detect_int_ena(&self) -> bool {
        (self.bits & IntEna::END_DETECT_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **TXFIFO_WM_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_wm_int_ena(&self) -> bool {
        (self.bits & IntEna::TXFIFO_WM_INT_ENA_MASK) != 0
    }

    /// Read **RXFIFO_WM_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_wm_int_ena(&self) -> bool {
        (self.bits & IntEna::RXFIFO_WM_INT_ENA_MASK) != 0
    }

    /// Set **SLAVE_ADDR_UNMATCH_INT_ENA** `[18:18]`
    ///
    /// Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    #[inline(always)]
    pub const fn with_slave_addr_unmatch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SLAVE_ADDR_UNMATCH_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SLAVE_ADDR_UNMATCH_INT_ENA_MASK
            },
        }
    }

    /// Set **GENERAL_CALL_INT_ENA** `[17:17]`
    ///
    /// Write 1 to enable I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub const fn with_general_call_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::GENERAL_CALL_INT_ENA_MASK
            } else {
                self.bits & !IntEna::GENERAL_CALL_INT_ENA_MASK
            },
        }
    }

    /// Set **SLAVE_STRETCH_INT_ENA** `[16:16]`
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn with_slave_stretch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SLAVE_STRETCH_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SLAVE_STRETCH_INT_ENA_MASK
            },
        }
    }

    /// Set **DET_START_INT_ENA** `[15:15]`
    ///
    /// Write 1 to enable I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub const fn with_det_start_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DET_START_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DET_START_INT_ENA_MASK
            },
        }
    }

    /// Set **SCL_MAIN_ST_TO_INT_ENA** `[14:14]`
    ///
    /// Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn with_scl_main_st_to_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SCL_MAIN_ST_TO_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SCL_MAIN_ST_TO_INT_ENA_MASK
            },
        }
    }

    /// Set **SCL_ST_TO_INT_ENA** `[13:13]`
    ///
    /// Write 1 to enable I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn with_scl_st_to_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SCL_ST_TO_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SCL_ST_TO_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_UDF_INT_ENA** `[12:12]`
    ///
    /// Write 1 to enable I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_udf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RXFIFO_UDF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RXFIFO_UDF_INT_ENA_MASK
            },
        }
    }

    /// Set **TXFIFO_OVF_INT_ENA** `[11:11]`
    ///
    /// Write 1 to enable I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn with_txfifo_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TXFIFO_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TXFIFO_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **NACK_INT_ENA** `[10:10]`
    ///
    /// Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn with_nack_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::NACK_INT_ENA_MASK
            } else {
                self.bits & !IntEna::NACK_INT_ENA_MASK
            },
        }
    }

    /// Set **TRANS_START_INT_ENA** `[9:9]`
    ///
    /// Write 1 to enable the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub const fn with_trans_start_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TRANS_START_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TRANS_START_INT_ENA_MASK
            },
        }
    }

    /// Set **TIME_OUT_INT_ENA** `[8:8]`
    ///
    /// Write 1 to enable the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub const fn with_time_out_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIME_OUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIME_OUT_INT_ENA_MASK
            },
        }
    }

    /// Set **TRANS_COMPLETE_INT_ENA** `[7:7]`
    ///
    /// Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn with_trans_complete_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TRANS_COMPLETE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TRANS_COMPLETE_INT_ENA_MASK
            },
        }
    }

    /// Set **MST_TXFIFO_UDF_INT_ENA** `[6:6]`
    ///
    /// Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn with_mst_txfifo_udf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::MST_TXFIFO_UDF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::MST_TXFIFO_UDF_INT_ENA_MASK
            },
        }
    }

    /// Set **ARBITRATION_LOST_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub const fn with_arbitration_lost_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::ARBITRATION_LOST_INT_ENA_MASK
            } else {
                self.bits & !IntEna::ARBITRATION_LOST_INT_ENA_MASK
            },
        }
    }

    /// Set **BYTE_TRANS_DONE_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn with_byte_trans_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::BYTE_TRANS_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::BYTE_TRANS_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **END_DETECT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn with_end_detect_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::END_DETECT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::END_DETECT_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_OVF_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable I2C_RXFIFO_OVF_INT interrupt.
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

    /// Set **TXFIFO_WM_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn with_txfifo_wm_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TXFIFO_WM_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TXFIFO_WM_INT_ENA_MASK
            },
        }
    }

    /// Set **RXFIFO_WM_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn with_rxfifo_wm_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RXFIFO_WM_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RXFIFO_WM_INT_ENA_MASK
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

/// INT_STATUS Register
///
/// Status of captured I2C communication events
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SLAVE_ADDR_UNMATCH_INT_ST** `[18]` (RO)
///
///   The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
///
/// - **GENERAL_CALL_INT_ST** `[17]` (RO)
///
///   The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt.
///
/// - **SLAVE_STRETCH_INT_ST** `[16]` (RO)
///
///   The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **DET_START_INT_ST** `[15]` (RO)
///
///   The masked interrupt status status of I2C_DET_START_INT interrupt.
///
/// - **SCL_MAIN_ST_TO_INT_ST** `[14]` (RO)
///
///   The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt.
///
/// - **SCL_ST_TO_INT_ST** `[13]` (RO)
///
///   The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt.
///
/// - **RXFIFO_UDF_INT_ST** `[12]` (RO)
///
///   The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt.
///
/// - **TXFIFO_OVF_INT_ST** `[11]` (RO)
///
///   The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt.
///
/// - **NACK_INT_ST** `[10]` (RO)
///
///   The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
///
/// - **TRANS_START_INT_ST** `[9]` (RO)
///
///   The masked interrupt status status of the I2C_TRANS_START_INT interrupt.
///
/// - **TIME_OUT_INT_ST** `[8]` (RO)
///
///   The masked interrupt status status of the I2C_TIME_OUT_INT interrupt.
///
/// - **TRANS_COMPLETE_INT_ST** `[7]` (RO)
///
///   The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **MST_TXFIFO_UDF_INT_ST** `[6]` (RO)
///
///   The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt.
///
/// - **ARBITRATION_LOST_INT_ST** `[5]` (RO)
///
///   The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt.
///
/// - **BYTE_TRANS_DONE_INT_ST** `[4]` (RO)
///
///   The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
///
/// - **END_DETECT_INT_ST** `[3]` (RO)
///
///   The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
///
/// - **RXFIFO_OVF_INT_ST** `[2]` (RO)
///
///   The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt.
///
/// - **TXFIFO_WM_INT_ST** `[1]` (RO)
///
///   The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt.
///
/// - **RXFIFO_WM_INT_ST** `[0]` (RO)
///
///   The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt.
#[repr(transparent)]
pub struct IntStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStatus {
    // ======== Field Constants ========

    /// **SLAVE_ADDR_UNMATCH_INT_ST**
    ///
    /// The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SLAVE_ADDR_UNMATCH_INT_ST_OFFSET: u32 = 18;
    pub const SLAVE_ADDR_UNMATCH_INT_ST_MASK: u32 = 0x00040000;
    pub const SLAVE_ADDR_UNMATCH_INT_ST: u32 = 1 << 18;

    /// **GENERAL_CALL_INT_ST**
    ///
    /// The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const GENERAL_CALL_INT_ST_OFFSET: u32 = 17;
    pub const GENERAL_CALL_INT_ST_MASK: u32 = 0x00020000;
    pub const GENERAL_CALL_INT_ST: u32 = 1 << 17;

    /// **SLAVE_STRETCH_INT_ST**
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SLAVE_STRETCH_INT_ST_OFFSET: u32 = 16;
    pub const SLAVE_STRETCH_INT_ST_MASK: u32 = 0x00010000;
    pub const SLAVE_STRETCH_INT_ST: u32 = 1 << 16;

    /// **DET_START_INT_ST**
    ///
    /// The masked interrupt status status of I2C_DET_START_INT interrupt.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DET_START_INT_ST_OFFSET: u32 = 15;
    pub const DET_START_INT_ST_MASK: u32 = 0x00008000;
    pub const DET_START_INT_ST: u32 = 1 << 15;

    /// **SCL_MAIN_ST_TO_INT_ST**
    ///
    /// The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCL_MAIN_ST_TO_INT_ST_OFFSET: u32 = 14;
    pub const SCL_MAIN_ST_TO_INT_ST_MASK: u32 = 0x00004000;
    pub const SCL_MAIN_ST_TO_INT_ST: u32 = 1 << 14;

    /// **SCL_ST_TO_INT_ST**
    ///
    /// The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCL_ST_TO_INT_ST_OFFSET: u32 = 13;
    pub const SCL_ST_TO_INT_ST_MASK: u32 = 0x00002000;
    pub const SCL_ST_TO_INT_ST: u32 = 1 << 13;

    /// **RXFIFO_UDF_INT_ST**
    ///
    /// The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RXFIFO_UDF_INT_ST_OFFSET: u32 = 12;
    pub const RXFIFO_UDF_INT_ST_MASK: u32 = 0x00001000;
    pub const RXFIFO_UDF_INT_ST: u32 = 1 << 12;

    /// **TXFIFO_OVF_INT_ST**
    ///
    /// The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXFIFO_OVF_INT_ST_OFFSET: u32 = 11;
    pub const TXFIFO_OVF_INT_ST_MASK: u32 = 0x00000800;
    pub const TXFIFO_OVF_INT_ST: u32 = 1 << 11;

    /// **NACK_INT_ST**
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const NACK_INT_ST_OFFSET: u32 = 10;
    pub const NACK_INT_ST_MASK: u32 = 0x00000400;
    pub const NACK_INT_ST: u32 = 1 << 10;

    /// **TRANS_START_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_TRANS_START_INT interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TRANS_START_INT_ST_OFFSET: u32 = 9;
    pub const TRANS_START_INT_ST_MASK: u32 = 0x00000200;
    pub const TRANS_START_INT_ST: u32 = 1 << 9;

    /// **TIME_OUT_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_TIME_OUT_INT interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TIME_OUT_INT_ST_OFFSET: u32 = 8;
    pub const TIME_OUT_INT_ST_MASK: u32 = 0x00000100;
    pub const TIME_OUT_INT_ST: u32 = 1 << 8;

    /// **TRANS_COMPLETE_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANS_COMPLETE_INT_ST_OFFSET: u32 = 7;
    pub const TRANS_COMPLETE_INT_ST_MASK: u32 = 0x00000080;
    pub const TRANS_COMPLETE_INT_ST: u32 = 1 << 7;

    /// **MST_TXFIFO_UDF_INT_ST**
    ///
    /// The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MST_TXFIFO_UDF_INT_ST_OFFSET: u32 = 6;
    pub const MST_TXFIFO_UDF_INT_ST_MASK: u32 = 0x00000040;
    pub const MST_TXFIFO_UDF_INT_ST: u32 = 1 << 6;

    /// **ARBITRATION_LOST_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ARBITRATION_LOST_INT_ST_OFFSET: u32 = 5;
    pub const ARBITRATION_LOST_INT_ST_MASK: u32 = 0x00000020;
    pub const ARBITRATION_LOST_INT_ST: u32 = 1 << 5;

    /// **BYTE_TRANS_DONE_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BYTE_TRANS_DONE_INT_ST_OFFSET: u32 = 4;
    pub const BYTE_TRANS_DONE_INT_ST_MASK: u32 = 0x00000010;
    pub const BYTE_TRANS_DONE_INT_ST: u32 = 1 << 4;

    /// **END_DETECT_INT_ST**
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const END_DETECT_INT_ST_OFFSET: u32 = 3;
    pub const END_DETECT_INT_ST_MASK: u32 = 0x00000008;
    pub const END_DETECT_INT_ST: u32 = 1 << 3;

    /// **RXFIFO_OVF_INT_ST**
    ///
    /// The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXFIFO_OVF_INT_ST_OFFSET: u32 = 2;
    pub const RXFIFO_OVF_INT_ST_MASK: u32 = 0x00000004;
    pub const RXFIFO_OVF_INT_ST: u32 = 1 << 2;

    /// **TXFIFO_WM_INT_ST**
    ///
    /// The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXFIFO_WM_INT_ST_OFFSET: u32 = 1;
    pub const TXFIFO_WM_INT_ST_MASK: u32 = 0x00000002;
    pub const TXFIFO_WM_INT_ST: u32 = 1 << 1;

    /// **RXFIFO_WM_INT_ST**
    ///
    /// The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXFIFO_WM_INT_ST_OFFSET: u32 = 0;
    pub const RXFIFO_WM_INT_ST_MASK: u32 = 0x00000001;
    pub const RXFIFO_WM_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLAVE_ADDR_UNMATCH_INT_ST** `[18]` (RO)
    ///
    /// The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    #[inline(always)]
    pub fn slave_addr_unmatch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_ADDR_UNMATCH_INT_ST_MASK) != 0
    }

    /// **GENERAL_CALL_INT_ST** `[17]` (RO)
    ///
    /// The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub fn general_call_int_st(&self) -> bool {
        (self.value.read_bits() & Self::GENERAL_CALL_INT_ST_MASK) != 0
    }

    /// **SLAVE_STRETCH_INT_ST** `[16]` (RO)
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn slave_stretch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_STRETCH_INT_ST_MASK) != 0
    }

    /// **DET_START_INT_ST** `[15]` (RO)
    ///
    /// The masked interrupt status status of I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub fn det_start_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DET_START_INT_ST_MASK) != 0
    }

    /// **SCL_MAIN_ST_TO_INT_ST** `[14]` (RO)
    ///
    /// The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_main_st_to_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SCL_MAIN_ST_TO_INT_ST_MASK) != 0
    }

    /// **SCL_ST_TO_INT_ST** `[13]` (RO)
    ///
    /// The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_st_to_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SCL_ST_TO_INT_ST_MASK) != 0
    }

    /// **RXFIFO_UDF_INT_ST** `[12]` (RO)
    ///
    /// The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_udf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_UDF_INT_ST_MASK) != 0
    }

    /// **TXFIFO_OVF_INT_ST** `[11]` (RO)
    ///
    /// The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn txfifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// **NACK_INT_ST** `[10]` (RO)
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn nack_int_st(&self) -> bool {
        (self.value.read_bits() & Self::NACK_INT_ST_MASK) != 0
    }

    /// **TRANS_START_INT_ST** `[9]` (RO)
    ///
    /// The masked interrupt status status of the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_START_INT_ST_MASK) != 0
    }

    /// **TIME_OUT_INT_ST** `[8]` (RO)
    ///
    /// The masked interrupt status status of the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub fn time_out_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIME_OUT_INT_ST_MASK) != 0
    }

    /// **TRANS_COMPLETE_INT_ST** `[7]` (RO)
    ///
    /// The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_INT_ST_MASK) != 0
    }

    /// **MST_TXFIFO_UDF_INT_ST** `[6]` (RO)
    ///
    /// The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn mst_txfifo_udf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::MST_TXFIFO_UDF_INT_ST_MASK) != 0
    }

    /// **ARBITRATION_LOST_INT_ST** `[5]` (RO)
    ///
    /// The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ARBITRATION_LOST_INT_ST_MASK) != 0
    }

    /// **BYTE_TRANS_DONE_INT_ST** `[4]` (RO)
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn byte_trans_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::BYTE_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// **END_DETECT_INT_ST** `[3]` (RO)
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> bool {
        (self.value.read_bits() & Self::END_DETECT_INT_ST_MASK) != 0
    }

    /// **RXFIFO_OVF_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// **TXFIFO_WM_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn txfifo_wm_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TXFIFO_WM_INT_ST_MASK) != 0
    }

    /// **RXFIFO_WM_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_wm_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RXFIFO_WM_INT_ST_MASK) != 0
    }
}

/// INT_STATUS value type for batch read/modify/write operations
///
/// Use `intstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStatusVal {
    bits: u32,
}

impl IntStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SLAVE_ADDR_UNMATCH_INT_ST** `[18:18]`
    ///
    /// The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt.
    #[inline(always)]
    pub const fn slave_addr_unmatch_int_st(&self) -> bool {
        (self.bits & IntStatus::SLAVE_ADDR_UNMATCH_INT_ST_MASK) != 0
    }

    /// Read **GENERAL_CALL_INT_ST** `[17:17]`
    ///
    /// The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt.
    #[inline(always)]
    pub const fn general_call_int_st(&self) -> bool {
        (self.bits & IntStatus::GENERAL_CALL_INT_ST_MASK) != 0
    }

    /// Read **SLAVE_STRETCH_INT_ST** `[16:16]`
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn slave_stretch_int_st(&self) -> bool {
        (self.bits & IntStatus::SLAVE_STRETCH_INT_ST_MASK) != 0
    }

    /// Read **DET_START_INT_ST** `[15:15]`
    ///
    /// The masked interrupt status status of I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub const fn det_start_int_st(&self) -> bool {
        (self.bits & IntStatus::DET_START_INT_ST_MASK) != 0
    }

    /// Read **SCL_MAIN_ST_TO_INT_ST** `[14:14]`
    ///
    /// The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_main_st_to_int_st(&self) -> bool {
        (self.bits & IntStatus::SCL_MAIN_ST_TO_INT_ST_MASK) != 0
    }

    /// Read **SCL_ST_TO_INT_ST** `[13:13]`
    ///
    /// The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub const fn scl_st_to_int_st(&self) -> bool {
        (self.bits & IntStatus::SCL_ST_TO_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_UDF_INT_ST** `[12:12]`
    ///
    /// The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_udf_int_st(&self) -> bool {
        (self.bits & IntStatus::RXFIFO_UDF_INT_ST_MASK) != 0
    }

    /// Read **TXFIFO_OVF_INT_ST** `[11:11]`
    ///
    /// The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_ovf_int_st(&self) -> bool {
        (self.bits & IntStatus::TXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **NACK_INT_ST** `[10:10]`
    ///
    /// The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub const fn nack_int_st(&self) -> bool {
        (self.bits & IntStatus::NACK_INT_ST_MASK) != 0
    }

    /// Read **TRANS_START_INT_ST** `[9:9]`
    ///
    /// The masked interrupt status status of the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub const fn trans_start_int_st(&self) -> bool {
        (self.bits & IntStatus::TRANS_START_INT_ST_MASK) != 0
    }

    /// Read **TIME_OUT_INT_ST** `[8:8]`
    ///
    /// The masked interrupt status status of the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub const fn time_out_int_st(&self) -> bool {
        (self.bits & IntStatus::TIME_OUT_INT_ST_MASK) != 0
    }

    /// Read **TRANS_COMPLETE_INT_ST** `[7:7]`
    ///
    /// The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn trans_complete_int_st(&self) -> bool {
        (self.bits & IntStatus::TRANS_COMPLETE_INT_ST_MASK) != 0
    }

    /// Read **MST_TXFIFO_UDF_INT_ST** `[6:6]`
    ///
    /// The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub const fn mst_txfifo_udf_int_st(&self) -> bool {
        (self.bits & IntStatus::MST_TXFIFO_UDF_INT_ST_MASK) != 0
    }

    /// Read **ARBITRATION_LOST_INT_ST** `[5:5]`
    ///
    /// The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub const fn arbitration_lost_int_st(&self) -> bool {
        (self.bits & IntStatus::ARBITRATION_LOST_INT_ST_MASK) != 0
    }

    /// Read **BYTE_TRANS_DONE_INT_ST** `[4:4]`
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn byte_trans_done_int_st(&self) -> bool {
        (self.bits & IntStatus::BYTE_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// Read **END_DETECT_INT_ST** `[3:3]`
    ///
    /// The masked interrupt status status of the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub const fn end_detect_int_st(&self) -> bool {
        (self.bits & IntStatus::END_DETECT_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_OVF_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_ovf_int_st(&self) -> bool {
        (self.bits & IntStatus::RXFIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **TXFIFO_WM_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn txfifo_wm_int_st(&self) -> bool {
        (self.bits & IntStatus::TXFIFO_WM_INT_ST_MASK) != 0
    }

    /// Read **RXFIFO_WM_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub const fn rxfifo_wm_int_st(&self) -> bool {
        (self.bits & IntStatus::RXFIFO_WM_INT_ST_MASK) != 0
    }
}

impl Default for IntStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStatusVal {
        IntStatusVal::from_bits(self.value.read_bits())
    }
}

/// SDA_HOLD Register
///
/// Configures the hold time after a negative SCL edge.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the time to hold the data after the falling edge of SCL.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SdaHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SdaHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SdaHold {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the time to hold the data after the falling edge of SCL.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the time to hold the data after the falling edge of SCL.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the time to hold the data after the falling edge of SCL.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SDA_HOLD value type for batch read/modify/write operations
///
/// Use `sdahold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdahold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SdaHoldVal {
    bits: u32,
}

impl SdaHoldVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the time to hold the data after the falling edge of SCL.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SdaHold::TIME_MASK) >> SdaHold::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the time to hold the data after the falling edge of SCL.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaHold::TIME_MASK)
                | ((value << SdaHold::TIME_OFFSET) & SdaHold::TIME_MASK),
        }
    }
}

impl Default for SdaHoldVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SdaHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SdaHoldVal {
        SdaHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SdaHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SdaHoldVal) -> SdaHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SDA_SAMPLE Register
///
/// Configures the sample time after a positive SCL edge.
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the sample time after a positive SCL edge.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SdaSample {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SdaSample {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SdaSample {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the sample time after a positive SCL edge.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the sample time after a positive SCL edge.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the sample time after a positive SCL edge.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SDA_SAMPLE value type for batch read/modify/write operations
///
/// Use `sdasample.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdasample.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SdaSampleVal {
    bits: u32,
}

impl SdaSampleVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the sample time after a positive SCL edge.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SdaSample::TIME_MASK) >> SdaSample::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the sample time after a positive SCL edge.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaSample::TIME_MASK)
                | ((value << SdaSample::TIME_OFFSET) & SdaSample::TIME_MASK),
        }
    }
}

impl Default for SdaSampleVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SdaSample {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SdaSampleVal {
        SdaSampleVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SdaSampleVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SdaSampleVal) -> SdaSampleVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_HIGH_PERIOD Register
///
/// Configures the high level width of SCL
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **SCL_WAIT_HIGH_PERIOD** `[15:9]` (RW)
///
///   Configures the SCL_FSM's waiting period for SCL high level in master mode.
///   Measurement unit: i2c_sclk
///
/// - **SCL_HIGH_PERIOD** `[8:0]` (RW)
///
///   Configures for how long SCL remains high in master mode.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclHighPeriod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclHighPeriod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclHighPeriod {
    // ======== Field Constants ========

    /// **SCL_WAIT_HIGH_PERIOD**
    ///
    /// Configures the SCL_FSM's waiting period for SCL high level in master mode.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[15:9]`, mask `0x0000fe00`
    pub const SCL_WAIT_HIGH_PERIOD_OFFSET: u32 = 9;
    pub const SCL_WAIT_HIGH_PERIOD_MASK: u32 = 0x0000fe00;

    /// **SCL_HIGH_PERIOD**
    ///
    /// Configures for how long SCL remains high in master mode.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const SCL_HIGH_PERIOD_OFFSET: u32 = 0;
    pub const SCL_HIGH_PERIOD_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **SCL_WAIT_HIGH_PERIOD** `[15:9]` (RW)
    ///
    /// Configures the SCL_FSM's waiting period for SCL high level in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_wait_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_WAIT_HIGH_PERIOD_MASK)
            >> Self::SCL_WAIT_HIGH_PERIOD_OFFSET
    }

    /// Write **SCL_WAIT_HIGH_PERIOD** `[15:9]` (RW)
    ///
    /// Configures the SCL_FSM's waiting period for SCL high level in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_wait_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_WAIT_HIGH_PERIOD_MASK)
                | ((value << Self::SCL_WAIT_HIGH_PERIOD_OFFSET) & Self::SCL_WAIT_HIGH_PERIOD_MASK)
        });
    }

    /// **SCL_HIGH_PERIOD** `[8:0]` (RW)
    ///
    /// Configures for how long SCL remains high in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_HIGH_PERIOD_MASK) >> Self::SCL_HIGH_PERIOD_OFFSET
    }

    /// Write **SCL_HIGH_PERIOD** `[8:0]` (RW)
    ///
    /// Configures for how long SCL remains high in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_HIGH_PERIOD_MASK)
                | ((value << Self::SCL_HIGH_PERIOD_OFFSET) & Self::SCL_HIGH_PERIOD_MASK)
        });
    }
}

/// SCL_HIGH_PERIOD value type for batch read/modify/write operations
///
/// Use `sclhighperiod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclhighperiod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclHighPeriodVal {
    bits: u32,
}

impl SclHighPeriodVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SCL_WAIT_HIGH_PERIOD** `[15:9]`
    ///
    /// Configures the SCL_FSM's waiting period for SCL high level in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_wait_high_period(&self) -> u32 {
        (self.bits & SclHighPeriod::SCL_WAIT_HIGH_PERIOD_MASK)
            >> SclHighPeriod::SCL_WAIT_HIGH_PERIOD_OFFSET
    }

    /// Read **SCL_HIGH_PERIOD** `[8:0]`
    ///
    /// Configures for how long SCL remains high in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_high_period(&self) -> u32 {
        (self.bits & SclHighPeriod::SCL_HIGH_PERIOD_MASK) >> SclHighPeriod::SCL_HIGH_PERIOD_OFFSET
    }

    /// Set **SCL_WAIT_HIGH_PERIOD** `[15:9]`
    ///
    /// Configures the SCL_FSM's waiting period for SCL high level in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_wait_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclHighPeriod::SCL_WAIT_HIGH_PERIOD_MASK)
                | ((value << SclHighPeriod::SCL_WAIT_HIGH_PERIOD_OFFSET)
                    & SclHighPeriod::SCL_WAIT_HIGH_PERIOD_MASK),
        }
    }

    /// Set **SCL_HIGH_PERIOD** `[8:0]`
    ///
    /// Configures for how long SCL remains high in master mode.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclHighPeriod::SCL_HIGH_PERIOD_MASK)
                | ((value << SclHighPeriod::SCL_HIGH_PERIOD_OFFSET)
                    & SclHighPeriod::SCL_HIGH_PERIOD_MASK),
        }
    }
}

impl Default for SclHighPeriodVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SclHighPeriod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclHighPeriodVal {
        SclHighPeriodVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclHighPeriodVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclHighPeriodVal) -> SclHighPeriodVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_START_HOLD Register
///
/// Configures the delay between the SDA and SCL negative edge for a start condition
///
/// Address offset: 0x0040
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
///   Measurement unit: i2c_sclk.
#[repr(transparent)]
pub struct SclStartHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStartHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStartHold {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
    /// Measurement unit: i2c_sclk.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SCL_START_HOLD value type for batch read/modify/write operations
///
/// Use `sclstarthold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstarthold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStartHoldVal {
    bits: u32,
}

impl SclStartHoldVal {
    /// Create from raw bits
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

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SclStartHold::TIME_MASK) >> SclStartHold::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition.
    /// Measurement unit: i2c_sclk.
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStartHold::TIME_MASK)
                | ((value << SclStartHold::TIME_OFFSET) & SclStartHold::TIME_MASK),
        }
    }
}

impl Default for SclStartHoldVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStartHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStartHoldVal {
        SclStartHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStartHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStartHoldVal) -> SclStartHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_RSTART_SETUP Register
///
/// Configures the delay between the positive edge of SCL and the negative edge of SDA
///
/// Address offset: 0x0044
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclRstartSetup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclRstartSetup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclRstartSetup {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SCL_RSTART_SETUP value type for batch read/modify/write operations
///
/// Use `sclrstartsetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclrstartsetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclRstartSetupVal {
    bits: u32,
}

impl SclRstartSetupVal {
    /// Create from raw bits
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

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SclRstartSetup::TIME_MASK) >> SclRstartSetup::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the time between the positive edge of SCL and the negative edge of SDA for a RESTART condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclRstartSetup::TIME_MASK)
                | ((value << SclRstartSetup::TIME_OFFSET) & SclRstartSetup::TIME_MASK),
        }
    }
}

impl Default for SclRstartSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclRstartSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclRstartSetupVal {
        SclRstartSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclRstartSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclRstartSetupVal) -> SclRstartSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_STOP_HOLD Register
///
/// Configures the delay after the SCL clock edge for a stop condition
///
/// Address offset: 0x0048
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the delay after the STOP condition.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclStopHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStopHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStopHold {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the delay after the STOP condition.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the delay after the STOP condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the delay after the STOP condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SCL_STOP_HOLD value type for batch read/modify/write operations
///
/// Use `sclstophold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstophold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStopHoldVal {
    bits: u32,
}

impl SclStopHoldVal {
    /// Create from raw bits
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

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the delay after the STOP condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SclStopHold::TIME_MASK) >> SclStopHold::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the delay after the STOP condition.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStopHold::TIME_MASK)
                | ((value << SclStopHold::TIME_OFFSET) & SclStopHold::TIME_MASK),
        }
    }
}

impl Default for SclStopHoldVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStopHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStopHoldVal {
        SclStopHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStopHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStopHoldVal) -> SclStopHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_STOP_SETUP Register
///
/// Configures the delay between the SDA and SCL rising edge for a stop condition.
/// Measurement unit: i2c_sclk
///
/// Address offset: 0x004c
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **TIME** `[8:0]` (RW)
///
///   Configures the time between the rising edge of SCL and the rising edge of SDA.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclStopSetup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStopSetup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStopSetup {
    // ======== Field Constants ========

    /// **TIME**
    ///
    /// Configures the time between the rising edge of SCL and the rising edge of SDA.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the rising edge of SCL and the rising edge of SDA.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

    /// Write **TIME** `[8:0]` (RW)
    ///
    /// Configures the time between the rising edge of SCL and the rising edge of SDA.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIME_MASK) | ((value << Self::TIME_OFFSET) & Self::TIME_MASK)
        });
    }
}

/// SCL_STOP_SETUP value type for batch read/modify/write operations
///
/// Use `sclstopsetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstopsetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStopSetupVal {
    bits: u32,
}

impl SclStopSetupVal {
    /// Create from raw bits
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

    /// Read **TIME** `[8:0]`
    ///
    /// Configures the time between the rising edge of SCL and the rising edge of SDA.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & SclStopSetup::TIME_MASK) >> SclStopSetup::TIME_OFFSET
    }

    /// Set **TIME** `[8:0]`
    ///
    /// Configures the time between the rising edge of SCL and the rising edge of SDA.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStopSetup::TIME_MASK)
                | ((value << SclStopSetup::TIME_OFFSET) & SclStopSetup::TIME_MASK),
        }
    }
}

impl Default for SclStopSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStopSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStopSetupVal {
        SclStopSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStopSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStopSetupVal) -> SclStopSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_CFG Register
///
/// SCL and SDA filter configuration register
///
/// Address offset: 0x0050
/// Reset value: 0x00000300
///
/// ## Fields
///
/// - **SDA_FILTER_EN** `[9]` (RW)
///
///   Configures to enable the filter function for SDA.
///
/// - **SCL_FILTER_EN** `[8]` (RW)
///
///   Configures to enable the filter function for SCL.
///
/// - **SDA_FILTER_THRES** `[7:4]` (RW)
///
///   Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
///   Measurement unit: i2c_sclk
///
/// - **SCL_FILTER_THRES** `[3:0]` (RW)
///
///   Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct FilterCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterCfg {
    // ======== Field Constants ========

    /// **SDA_FILTER_EN**
    ///
    /// Configures to enable the filter function for SDA.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SDA_FILTER_EN_OFFSET: u32 = 9;
    pub const SDA_FILTER_EN_MASK: u32 = 0x00000200;
    pub const SDA_FILTER_EN: u32 = 1 << 9;

    /// **SCL_FILTER_EN**
    ///
    /// Configures to enable the filter function for SCL.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SCL_FILTER_EN_OFFSET: u32 = 8;
    pub const SCL_FILTER_EN_MASK: u32 = 0x00000100;
    pub const SCL_FILTER_EN: u32 = 1 << 8;

    /// **SDA_FILTER_THRES**
    ///
    /// Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const SDA_FILTER_THRES_OFFSET: u32 = 4;
    pub const SDA_FILTER_THRES_MASK: u32 = 0x000000f0;

    /// **SCL_FILTER_THRES**
    ///
    /// Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SCL_FILTER_THRES_OFFSET: u32 = 0;
    pub const SCL_FILTER_THRES_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SDA_FILTER_EN** `[9]` (RW)
    ///
    /// Configures to enable the filter function for SDA.
    #[inline(always)]
    pub fn sda_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::SDA_FILTER_EN_MASK) != 0
    }

    /// Set SDA_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sda_filter_en(&self) {
        self.value.modify_bits(|r| r | Self::SDA_FILTER_EN_MASK);
    }

    /// Clear SDA_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sda_filter_en(&self) {
        self.value.modify_bits(|r| r & !Self::SDA_FILTER_EN_MASK);
    }

    /// **SCL_FILTER_EN** `[8]` (RW)
    ///
    /// Configures to enable the filter function for SCL.
    #[inline(always)]
    pub fn scl_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::SCL_FILTER_EN_MASK) != 0
    }

    /// Set SCL_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_filter_en(&self) {
        self.value.modify_bits(|r| r | Self::SCL_FILTER_EN_MASK);
    }

    /// Clear SCL_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_filter_en(&self) {
        self.value.modify_bits(|r| r & !Self::SCL_FILTER_EN_MASK);
    }

    /// **SDA_FILTER_THRES** `[7:4]` (RW)
    ///
    /// Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> u32 {
        (self.value.read_bits() & Self::SDA_FILTER_THRES_MASK) >> Self::SDA_FILTER_THRES_OFFSET
    }

    /// Write **SDA_FILTER_THRES** `[7:4]` (RW)
    ///
    /// Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_sda_filter_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SDA_FILTER_THRES_MASK)
                | ((value << Self::SDA_FILTER_THRES_OFFSET) & Self::SDA_FILTER_THRES_MASK)
        });
    }

    /// **SCL_FILTER_THRES** `[3:0]` (RW)
    ///
    /// Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_FILTER_THRES_MASK) >> Self::SCL_FILTER_THRES_OFFSET
    }

    /// Write **SCL_FILTER_THRES** `[3:0]` (RW)
    ///
    /// Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_filter_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_FILTER_THRES_MASK)
                | ((value << Self::SCL_FILTER_THRES_OFFSET) & Self::SCL_FILTER_THRES_MASK)
        });
    }
}

/// FILTER_CFG value type for batch read/modify/write operations
///
/// Use `filtercfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filtercfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterCfgVal {
    bits: u32,
}

impl FilterCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000300 }
    }

    /// Read **SDA_FILTER_EN** `[9:9]`
    ///
    /// Configures to enable the filter function for SDA.
    #[inline(always)]
    pub const fn sda_filter_en(&self) -> bool {
        (self.bits & FilterCfg::SDA_FILTER_EN_MASK) != 0
    }

    /// Read **SCL_FILTER_EN** `[8:8]`
    ///
    /// Configures to enable the filter function for SCL.
    #[inline(always)]
    pub const fn scl_filter_en(&self) -> bool {
        (self.bits & FilterCfg::SCL_FILTER_EN_MASK) != 0
    }

    /// Read **SDA_FILTER_THRES** `[7:4]`
    ///
    /// Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn sda_filter_thres(&self) -> u32 {
        (self.bits & FilterCfg::SDA_FILTER_THRES_MASK) >> FilterCfg::SDA_FILTER_THRES_OFFSET
    }

    /// Read **SCL_FILTER_THRES** `[3:0]`
    ///
    /// Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_filter_thres(&self) -> u32 {
        (self.bits & FilterCfg::SCL_FILTER_THRES_MASK) >> FilterCfg::SCL_FILTER_THRES_OFFSET
    }

    /// Set **SDA_FILTER_EN** `[9:9]`
    ///
    /// Configures to enable the filter function for SDA.
    #[inline(always)]
    pub const fn with_sda_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterCfg::SDA_FILTER_EN_MASK
            } else {
                self.bits & !FilterCfg::SDA_FILTER_EN_MASK
            },
        }
    }

    /// Set **SCL_FILTER_EN** `[8:8]`
    ///
    /// Configures to enable the filter function for SCL.
    #[inline(always)]
    pub const fn with_scl_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterCfg::SCL_FILTER_EN_MASK
            } else {
                self.bits & !FilterCfg::SCL_FILTER_EN_MASK
            },
        }
    }

    /// Set **SDA_FILTER_THRES** `[7:4]`
    ///
    /// Configures the threshold pulse width to be filtered on SDA. When a pulse on the SDA input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_sda_filter_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCfg::SDA_FILTER_THRES_MASK)
                | ((value << FilterCfg::SDA_FILTER_THRES_OFFSET)
                    & FilterCfg::SDA_FILTER_THRES_MASK),
        }
    }

    /// Set **SCL_FILTER_THRES** `[3:0]`
    ///
    /// Configures the threshold pulse width to be filtered on SCL. When a pulse on the SCL input has smaller width than this register value, the I2C controller will ignore that pulse.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_filter_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCfg::SCL_FILTER_THRES_MASK)
                | ((value << FilterCfg::SCL_FILTER_THRES_OFFSET)
                    & FilterCfg::SCL_FILTER_THRES_MASK),
        }
    }
}

impl Default for FilterCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FilterCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterCfgVal {
        FilterCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterCfgVal) -> FilterCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_CONF Register
///
/// I2C CLK configuration register
///
/// Address offset: 0x0054
/// Reset value: 0x00200000
///
/// ## Fields
///
/// - **SCLK_ACTIVE** `[21]` (RW)
///
///   The clock switch for i2c module
///
/// - **SCLK_SEL** `[20]` (RW)
///
///   The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
///
/// - **SCLK_DIV_B** `[19:14]` (RW)
///
///   the denominator of the fractional part of the fractional divisor for i2c module
///
/// - **SCLK_DIV_A** `[13:8]` (RW)
///
///   the numerator of the fractional part of the fractional divisor for i2c module
///
/// - **SCLK_DIV_NUM** `[7:0]` (RW)
///
///   the integral part of the fractional divisor for i2c module
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

    /// **SCLK_ACTIVE**
    ///
    /// The clock switch for i2c module
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SCLK_ACTIVE_OFFSET: u32 = 21;
    pub const SCLK_ACTIVE_MASK: u32 = 0x00200000;
    pub const SCLK_ACTIVE: u32 = 1 << 21;

    /// **SCLK_SEL**
    ///
    /// The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SCLK_SEL_OFFSET: u32 = 20;
    pub const SCLK_SEL_MASK: u32 = 0x00100000;
    pub const SCLK_SEL: u32 = 1 << 20;

    /// **SCLK_DIV_B**
    ///
    /// the denominator of the fractional part of the fractional divisor for i2c module
    ///
    /// Bits `[19:14]`, mask `0x000fc000`
    pub const SCLK_DIV_B_OFFSET: u32 = 14;
    pub const SCLK_DIV_B_MASK: u32 = 0x000fc000;

    /// **SCLK_DIV_A**
    ///
    /// the numerator of the fractional part of the fractional divisor for i2c module
    ///
    /// Bits `[13:8]`, mask `0x00003f00`
    pub const SCLK_DIV_A_OFFSET: u32 = 8;
    pub const SCLK_DIV_A_MASK: u32 = 0x00003f00;

    /// **SCLK_DIV_NUM**
    ///
    /// the integral part of the fractional divisor for i2c module
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SCLK_DIV_NUM_OFFSET: u32 = 0;
    pub const SCLK_DIV_NUM_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SCLK_ACTIVE** `[21]` (RW)
    ///
    /// The clock switch for i2c module
    #[inline(always)]
    pub fn sclk_active(&self) -> bool {
        (self.value.read_bits() & Self::SCLK_ACTIVE_MASK) != 0
    }

    /// Set SCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sclk_active(&self) {
        self.value.modify_bits(|r| r | Self::SCLK_ACTIVE_MASK);
    }

    /// Clear SCLK_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sclk_active(&self) {
        self.value.modify_bits(|r| r & !Self::SCLK_ACTIVE_MASK);
    }

    /// **SCLK_SEL** `[20]` (RW)
    ///
    /// The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    #[inline(always)]
    pub fn sclk_sel(&self) -> bool {
        (self.value.read_bits() & Self::SCLK_SEL_MASK) != 0
    }

    /// Set SCLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sclk_sel(&self) {
        self.value.modify_bits(|r| r | Self::SCLK_SEL_MASK);
    }

    /// Clear SCLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sclk_sel(&self) {
        self.value.modify_bits(|r| r & !Self::SCLK_SEL_MASK);
    }

    /// **SCLK_DIV_B** `[19:14]` (RW)
    ///
    /// the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_b(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_B_MASK) >> Self::SCLK_DIV_B_OFFSET
    }

    /// Write **SCLK_DIV_B** `[19:14]` (RW)
    ///
    /// the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn write_sclk_div_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_B_MASK)
                | ((value << Self::SCLK_DIV_B_OFFSET) & Self::SCLK_DIV_B_MASK)
        });
    }

    /// **SCLK_DIV_A** `[13:8]` (RW)
    ///
    /// the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_a(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_A_MASK) >> Self::SCLK_DIV_A_OFFSET
    }

    /// Write **SCLK_DIV_A** `[13:8]` (RW)
    ///
    /// the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn write_sclk_div_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_A_MASK)
                | ((value << Self::SCLK_DIV_A_OFFSET) & Self::SCLK_DIV_A_MASK)
        });
    }

    /// **SCLK_DIV_NUM** `[7:0]` (RW)
    ///
    /// the integral part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::SCLK_DIV_NUM_MASK) >> Self::SCLK_DIV_NUM_OFFSET
    }

    /// Write **SCLK_DIV_NUM** `[7:0]` (RW)
    ///
    /// the integral part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn write_sclk_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCLK_DIV_NUM_MASK)
                | ((value << Self::SCLK_DIV_NUM_OFFSET) & Self::SCLK_DIV_NUM_MASK)
        });
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
        Self { bits: 0x00200000 }
    }

    /// Read **SCLK_ACTIVE** `[21:21]`
    ///
    /// The clock switch for i2c module
    #[inline(always)]
    pub const fn sclk_active(&self) -> bool {
        (self.bits & ClkConf::SCLK_ACTIVE_MASK) != 0
    }

    /// Read **SCLK_SEL** `[20:20]`
    ///
    /// The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    #[inline(always)]
    pub const fn sclk_sel(&self) -> bool {
        (self.bits & ClkConf::SCLK_SEL_MASK) != 0
    }

    /// Read **SCLK_DIV_B** `[19:14]`
    ///
    /// the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn sclk_div_b(&self) -> u32 {
        (self.bits & ClkConf::SCLK_DIV_B_MASK) >> ClkConf::SCLK_DIV_B_OFFSET
    }

    /// Read **SCLK_DIV_A** `[13:8]`
    ///
    /// the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn sclk_div_a(&self) -> u32 {
        (self.bits & ClkConf::SCLK_DIV_A_MASK) >> ClkConf::SCLK_DIV_A_OFFSET
    }

    /// Read **SCLK_DIV_NUM** `[7:0]`
    ///
    /// the integral part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn sclk_div_num(&self) -> u32 {
        (self.bits & ClkConf::SCLK_DIV_NUM_MASK) >> ClkConf::SCLK_DIV_NUM_OFFSET
    }

    /// Set **SCLK_ACTIVE** `[21:21]`
    ///
    /// The clock switch for i2c module
    #[inline(always)]
    pub const fn with_sclk_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::SCLK_ACTIVE_MASK
            } else {
                self.bits & !ClkConf::SCLK_ACTIVE_MASK
            },
        }
    }

    /// Set **SCLK_SEL** `[20:20]`
    ///
    /// The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    #[inline(always)]
    pub const fn with_sclk_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::SCLK_SEL_MASK
            } else {
                self.bits & !ClkConf::SCLK_SEL_MASK
            },
        }
    }

    /// Set **SCLK_DIV_B** `[19:14]`
    ///
    /// the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn with_sclk_div_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkConf::SCLK_DIV_B_MASK)
                | ((value << ClkConf::SCLK_DIV_B_OFFSET) & ClkConf::SCLK_DIV_B_MASK),
        }
    }

    /// Set **SCLK_DIV_A** `[13:8]`
    ///
    /// the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn with_sclk_div_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkConf::SCLK_DIV_A_MASK)
                | ((value << ClkConf::SCLK_DIV_A_OFFSET) & ClkConf::SCLK_DIV_A_MASK),
        }
    }

    /// Set **SCLK_DIV_NUM** `[7:0]`
    ///
    /// the integral part of the fractional divisor for i2c module
    #[inline(always)]
    pub const fn with_sclk_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkConf::SCLK_DIV_NUM_MASK)
                | ((value << ClkConf::SCLK_DIV_NUM_OFFSET) & ClkConf::SCLK_DIV_NUM_MASK),
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

/// COMD0 Register
///
/// I2C command register 0
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **COMMAND0_DONE** `[31]` (RW)
///
///   Represents whether command 0 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND0** `[13:0]` (RW)
///
///   Configures command 0. It consists of three parts:
///   op_code is the command,
///   0: RSTART,
///   1: WRITE,
///   2: READ,
///   3: STOP,
///   4: END.
///   Byte_num represents the number of bytes that need to be sent or received.
///   ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
#[repr(transparent)]
pub struct Comd0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd0 {
    // ======== Field Constants ========

    /// **COMMAND0_DONE**
    ///
    /// Represents whether command 0 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND0_DONE_OFFSET: u32 = 31;
    pub const COMMAND0_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND0_DONE: u32 = 1 << 31;

    /// **COMMAND0**
    ///
    /// Configures command 0. It consists of three parts:
    /// op_code is the command,
    /// 0: RSTART,
    /// 1: WRITE,
    /// 2: READ,
    /// 3: STOP,
    /// 4: END.
    /// Byte_num represents the number of bytes that need to be sent or received.
    /// ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND0_OFFSET: u32 = 0;
    pub const COMMAND0_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND0_DONE** `[31]` (RW)
    ///
    /// Represents whether command 0 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command0_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND0_DONE_MASK) != 0
    }

    /// Set COMMAND0_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command0_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND0_DONE_MASK);
    }

    /// Clear COMMAND0_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command0_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND0_DONE_MASK);
    }

    /// **COMMAND0** `[13:0]` (RW)
    ///
    /// Configures command 0. It consists of three parts:
    /// op_code is the command,
    /// 0: RSTART,
    /// 1: WRITE,
    /// 2: READ,
    /// 3: STOP,
    /// 4: END.
    /// Byte_num represents the number of bytes that need to be sent or received.
    /// ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
    #[inline(always)]
    pub fn command0(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND0_MASK) >> Self::COMMAND0_OFFSET
    }

    /// Write **COMMAND0** `[13:0]` (RW)
    ///
    /// Configures command 0. It consists of three parts:
    /// op_code is the command,
    /// 0: RSTART,
    /// 1: WRITE,
    /// 2: READ,
    /// 3: STOP,
    /// 4: END.
    /// Byte_num represents the number of bytes that need to be sent or received.
    /// ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
    #[inline(always)]
    pub fn write_command0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND0_MASK) | ((value << Self::COMMAND0_OFFSET) & Self::COMMAND0_MASK)
        });
    }
}

/// COMD0 value type for batch read/modify/write operations
///
/// Use `comd0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd0Val {
    bits: u32,
}

impl Comd0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND0_DONE** `[31:31]`
    ///
    /// Represents whether command 0 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command0_done(&self) -> bool {
        (self.bits & Comd0::COMMAND0_DONE_MASK) != 0
    }

    /// Read **COMMAND0** `[13:0]`
    ///
    /// Configures command 0. It consists of three parts:
    /// op_code is the command,
    /// 0: RSTART,
    /// 1: WRITE,
    /// 2: READ,
    /// 3: STOP,
    /// 4: END.
    /// Byte_num represents the number of bytes that need to be sent or received.
    /// ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
    #[inline(always)]
    pub const fn command0(&self) -> u32 {
        (self.bits & Comd0::COMMAND0_MASK) >> Comd0::COMMAND0_OFFSET
    }

    /// Set **COMMAND0_DONE** `[31:31]`
    ///
    /// Represents whether command 0 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command0_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd0::COMMAND0_DONE_MASK
            } else {
                self.bits & !Comd0::COMMAND0_DONE_MASK
            },
        }
    }

    /// Set **COMMAND0** `[13:0]`
    ///
    /// Configures command 0. It consists of three parts:
    /// op_code is the command,
    /// 0: RSTART,
    /// 1: WRITE,
    /// 2: READ,
    /// 3: STOP,
    /// 4: END.
    /// Byte_num represents the number of bytes that need to be sent or received.
    /// ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information.
    #[inline(always)]
    pub const fn with_command0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd0::COMMAND0_MASK)
                | ((value << Comd0::COMMAND0_OFFSET) & Comd0::COMMAND0_MASK),
        }
    }
}

impl Default for Comd0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd0Val {
        Comd0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd0Val) -> Comd0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD1 Register
///
/// I2C command register 1
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **COMMAND1_DONE** `[31]` (RW)
///
///   Represents whether command 1 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND1** `[13:0]` (RW)
///
///   Configures command 1. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd1 {
    // ======== Field Constants ========

    /// **COMMAND1_DONE**
    ///
    /// Represents whether command 1 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND1_DONE_OFFSET: u32 = 31;
    pub const COMMAND1_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND1_DONE: u32 = 1 << 31;

    /// **COMMAND1**
    ///
    /// Configures command 1. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND1_OFFSET: u32 = 0;
    pub const COMMAND1_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND1_DONE** `[31]` (RW)
    ///
    /// Represents whether command 1 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command1_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND1_DONE_MASK) != 0
    }

    /// Set COMMAND1_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command1_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND1_DONE_MASK);
    }

    /// Clear COMMAND1_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command1_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND1_DONE_MASK);
    }

    /// **COMMAND1** `[13:0]` (RW)
    ///
    /// Configures command 1. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command1(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND1_MASK) >> Self::COMMAND1_OFFSET
    }

    /// Write **COMMAND1** `[13:0]` (RW)
    ///
    /// Configures command 1. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND1_MASK) | ((value << Self::COMMAND1_OFFSET) & Self::COMMAND1_MASK)
        });
    }
}

/// COMD1 value type for batch read/modify/write operations
///
/// Use `comd1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd1Val {
    bits: u32,
}

impl Comd1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND1_DONE** `[31:31]`
    ///
    /// Represents whether command 1 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command1_done(&self) -> bool {
        (self.bits & Comd1::COMMAND1_DONE_MASK) != 0
    }

    /// Read **COMMAND1** `[13:0]`
    ///
    /// Configures command 1. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command1(&self) -> u32 {
        (self.bits & Comd1::COMMAND1_MASK) >> Comd1::COMMAND1_OFFSET
    }

    /// Set **COMMAND1_DONE** `[31:31]`
    ///
    /// Represents whether command 1 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command1_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd1::COMMAND1_DONE_MASK
            } else {
                self.bits & !Comd1::COMMAND1_DONE_MASK
            },
        }
    }

    /// Set **COMMAND1** `[13:0]`
    ///
    /// Configures command 1. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd1::COMMAND1_MASK)
                | ((value << Comd1::COMMAND1_OFFSET) & Comd1::COMMAND1_MASK),
        }
    }
}

impl Default for Comd1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd1Val {
        Comd1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd1Val) -> Comd1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD2 Register
///
/// I2C command register 2
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **COMMAND2_DONE** `[31]` (RW)
///
///   Represents whether command 2 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND2** `[13:0]` (RW)
///
///   Configures command 2. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd2 {
    // ======== Field Constants ========

    /// **COMMAND2_DONE**
    ///
    /// Represents whether command 2 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND2_DONE_OFFSET: u32 = 31;
    pub const COMMAND2_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND2_DONE: u32 = 1 << 31;

    /// **COMMAND2**
    ///
    /// Configures command 2. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND2_OFFSET: u32 = 0;
    pub const COMMAND2_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND2_DONE** `[31]` (RW)
    ///
    /// Represents whether command 2 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command2_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND2_DONE_MASK) != 0
    }

    /// Set COMMAND2_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command2_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND2_DONE_MASK);
    }

    /// Clear COMMAND2_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command2_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND2_DONE_MASK);
    }

    /// **COMMAND2** `[13:0]` (RW)
    ///
    /// Configures command 2. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command2(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND2_MASK) >> Self::COMMAND2_OFFSET
    }

    /// Write **COMMAND2** `[13:0]` (RW)
    ///
    /// Configures command 2. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND2_MASK) | ((value << Self::COMMAND2_OFFSET) & Self::COMMAND2_MASK)
        });
    }
}

/// COMD2 value type for batch read/modify/write operations
///
/// Use `comd2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd2Val {
    bits: u32,
}

impl Comd2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND2_DONE** `[31:31]`
    ///
    /// Represents whether command 2 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command2_done(&self) -> bool {
        (self.bits & Comd2::COMMAND2_DONE_MASK) != 0
    }

    /// Read **COMMAND2** `[13:0]`
    ///
    /// Configures command 2. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command2(&self) -> u32 {
        (self.bits & Comd2::COMMAND2_MASK) >> Comd2::COMMAND2_OFFSET
    }

    /// Set **COMMAND2_DONE** `[31:31]`
    ///
    /// Represents whether command 2 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command2_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd2::COMMAND2_DONE_MASK
            } else {
                self.bits & !Comd2::COMMAND2_DONE_MASK
            },
        }
    }

    /// Set **COMMAND2** `[13:0]`
    ///
    /// Configures command 2. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd2::COMMAND2_MASK)
                | ((value << Comd2::COMMAND2_OFFSET) & Comd2::COMMAND2_MASK),
        }
    }
}

impl Default for Comd2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd2Val {
        Comd2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd2Val) -> Comd2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD3 Register
///
/// I2C command register 3
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **COMMAND3_DONE** `[31]` (RW)
///
///   Represents whether command 3 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND3** `[13:0]` (RW)
///
///   Configures command 3. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd3 {
    // ======== Field Constants ========

    /// **COMMAND3_DONE**
    ///
    /// Represents whether command 3 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND3_DONE_OFFSET: u32 = 31;
    pub const COMMAND3_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND3_DONE: u32 = 1 << 31;

    /// **COMMAND3**
    ///
    /// Configures command 3. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND3_OFFSET: u32 = 0;
    pub const COMMAND3_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND3_DONE** `[31]` (RW)
    ///
    /// Represents whether command 3 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command3_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND3_DONE_MASK) != 0
    }

    /// Set COMMAND3_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command3_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND3_DONE_MASK);
    }

    /// Clear COMMAND3_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command3_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND3_DONE_MASK);
    }

    /// **COMMAND3** `[13:0]` (RW)
    ///
    /// Configures command 3. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command3(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND3_MASK) >> Self::COMMAND3_OFFSET
    }

    /// Write **COMMAND3** `[13:0]` (RW)
    ///
    /// Configures command 3. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND3_MASK) | ((value << Self::COMMAND3_OFFSET) & Self::COMMAND3_MASK)
        });
    }
}

/// COMD3 value type for batch read/modify/write operations
///
/// Use `comd3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd3Val {
    bits: u32,
}

impl Comd3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND3_DONE** `[31:31]`
    ///
    /// Represents whether command 3 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command3_done(&self) -> bool {
        (self.bits & Comd3::COMMAND3_DONE_MASK) != 0
    }

    /// Read **COMMAND3** `[13:0]`
    ///
    /// Configures command 3. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command3(&self) -> u32 {
        (self.bits & Comd3::COMMAND3_MASK) >> Comd3::COMMAND3_OFFSET
    }

    /// Set **COMMAND3_DONE** `[31:31]`
    ///
    /// Represents whether command 3 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command3_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd3::COMMAND3_DONE_MASK
            } else {
                self.bits & !Comd3::COMMAND3_DONE_MASK
            },
        }
    }

    /// Set **COMMAND3** `[13:0]`
    ///
    /// Configures command 3. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd3::COMMAND3_MASK)
                | ((value << Comd3::COMMAND3_OFFSET) & Comd3::COMMAND3_MASK),
        }
    }
}

impl Default for Comd3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd3Val {
        Comd3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd3Val) -> Comd3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD4 Register
///
/// I2C command register 4
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **COMMAND4_DONE** `[31]` (RW)
///
///   Represents whether command 4 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND4** `[13:0]` (RW)
///
///   Configures command 4. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd4 {
    // ======== Field Constants ========

    /// **COMMAND4_DONE**
    ///
    /// Represents whether command 4 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND4_DONE_OFFSET: u32 = 31;
    pub const COMMAND4_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND4_DONE: u32 = 1 << 31;

    /// **COMMAND4**
    ///
    /// Configures command 4. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND4_OFFSET: u32 = 0;
    pub const COMMAND4_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND4_DONE** `[31]` (RW)
    ///
    /// Represents whether command 4 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command4_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND4_DONE_MASK) != 0
    }

    /// Set COMMAND4_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command4_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND4_DONE_MASK);
    }

    /// Clear COMMAND4_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command4_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND4_DONE_MASK);
    }

    /// **COMMAND4** `[13:0]` (RW)
    ///
    /// Configures command 4. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command4(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND4_MASK) >> Self::COMMAND4_OFFSET
    }

    /// Write **COMMAND4** `[13:0]` (RW)
    ///
    /// Configures command 4. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND4_MASK) | ((value << Self::COMMAND4_OFFSET) & Self::COMMAND4_MASK)
        });
    }
}

/// COMD4 value type for batch read/modify/write operations
///
/// Use `comd4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd4Val {
    bits: u32,
}

impl Comd4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND4_DONE** `[31:31]`
    ///
    /// Represents whether command 4 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command4_done(&self) -> bool {
        (self.bits & Comd4::COMMAND4_DONE_MASK) != 0
    }

    /// Read **COMMAND4** `[13:0]`
    ///
    /// Configures command 4. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command4(&self) -> u32 {
        (self.bits & Comd4::COMMAND4_MASK) >> Comd4::COMMAND4_OFFSET
    }

    /// Set **COMMAND4_DONE** `[31:31]`
    ///
    /// Represents whether command 4 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command4_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd4::COMMAND4_DONE_MASK
            } else {
                self.bits & !Comd4::COMMAND4_DONE_MASK
            },
        }
    }

    /// Set **COMMAND4** `[13:0]`
    ///
    /// Configures command 4. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd4::COMMAND4_MASK)
                | ((value << Comd4::COMMAND4_OFFSET) & Comd4::COMMAND4_MASK),
        }
    }
}

impl Default for Comd4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd4Val {
        Comd4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd4Val) -> Comd4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD5 Register
///
/// I2C command register 5
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **COMMAND5_DONE** `[31]` (RW)
///
///   Represents whether command 5 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND5** `[13:0]` (RW)
///
///   Configures command 5. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd5 {
    // ======== Field Constants ========

    /// **COMMAND5_DONE**
    ///
    /// Represents whether command 5 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND5_DONE_OFFSET: u32 = 31;
    pub const COMMAND5_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND5_DONE: u32 = 1 << 31;

    /// **COMMAND5**
    ///
    /// Configures command 5. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND5_OFFSET: u32 = 0;
    pub const COMMAND5_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND5_DONE** `[31]` (RW)
    ///
    /// Represents whether command 5 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command5_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND5_DONE_MASK) != 0
    }

    /// Set COMMAND5_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command5_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND5_DONE_MASK);
    }

    /// Clear COMMAND5_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command5_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND5_DONE_MASK);
    }

    /// **COMMAND5** `[13:0]` (RW)
    ///
    /// Configures command 5. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command5(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND5_MASK) >> Self::COMMAND5_OFFSET
    }

    /// Write **COMMAND5** `[13:0]` (RW)
    ///
    /// Configures command 5. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND5_MASK) | ((value << Self::COMMAND5_OFFSET) & Self::COMMAND5_MASK)
        });
    }
}

/// COMD5 value type for batch read/modify/write operations
///
/// Use `comd5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd5Val {
    bits: u32,
}

impl Comd5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND5_DONE** `[31:31]`
    ///
    /// Represents whether command 5 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command5_done(&self) -> bool {
        (self.bits & Comd5::COMMAND5_DONE_MASK) != 0
    }

    /// Read **COMMAND5** `[13:0]`
    ///
    /// Configures command 5. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command5(&self) -> u32 {
        (self.bits & Comd5::COMMAND5_MASK) >> Comd5::COMMAND5_OFFSET
    }

    /// Set **COMMAND5_DONE** `[31:31]`
    ///
    /// Represents whether command 5 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command5_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd5::COMMAND5_DONE_MASK
            } else {
                self.bits & !Comd5::COMMAND5_DONE_MASK
            },
        }
    }

    /// Set **COMMAND5** `[13:0]`
    ///
    /// Configures command 5. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd5::COMMAND5_MASK)
                | ((value << Comd5::COMMAND5_OFFSET) & Comd5::COMMAND5_MASK),
        }
    }
}

impl Default for Comd5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd5Val {
        Comd5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd5Val) -> Comd5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD6 Register
///
/// I2C command register 6
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **COMMAND6_DONE** `[31]` (RW)
///
///   Represents whether command 6 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND6** `[13:0]` (RW)
///
///   Configures command 6. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd6 {
    // ======== Field Constants ========

    /// **COMMAND6_DONE**
    ///
    /// Represents whether command 6 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND6_DONE_OFFSET: u32 = 31;
    pub const COMMAND6_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND6_DONE: u32 = 1 << 31;

    /// **COMMAND6**
    ///
    /// Configures command 6. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND6_OFFSET: u32 = 0;
    pub const COMMAND6_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND6_DONE** `[31]` (RW)
    ///
    /// Represents whether command 6 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command6_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND6_DONE_MASK) != 0
    }

    /// Set COMMAND6_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command6_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND6_DONE_MASK);
    }

    /// Clear COMMAND6_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command6_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND6_DONE_MASK);
    }

    /// **COMMAND6** `[13:0]` (RW)
    ///
    /// Configures command 6. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command6(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND6_MASK) >> Self::COMMAND6_OFFSET
    }

    /// Write **COMMAND6** `[13:0]` (RW)
    ///
    /// Configures command 6. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND6_MASK) | ((value << Self::COMMAND6_OFFSET) & Self::COMMAND6_MASK)
        });
    }
}

/// COMD6 value type for batch read/modify/write operations
///
/// Use `comd6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd6Val {
    bits: u32,
}

impl Comd6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND6_DONE** `[31:31]`
    ///
    /// Represents whether command 6 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command6_done(&self) -> bool {
        (self.bits & Comd6::COMMAND6_DONE_MASK) != 0
    }

    /// Read **COMMAND6** `[13:0]`
    ///
    /// Configures command 6. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command6(&self) -> u32 {
        (self.bits & Comd6::COMMAND6_MASK) >> Comd6::COMMAND6_OFFSET
    }

    /// Set **COMMAND6_DONE** `[31:31]`
    ///
    /// Represents whether command 6 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command6_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd6::COMMAND6_DONE_MASK
            } else {
                self.bits & !Comd6::COMMAND6_DONE_MASK
            },
        }
    }

    /// Set **COMMAND6** `[13:0]`
    ///
    /// Configures command 6. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd6::COMMAND6_MASK)
                | ((value << Comd6::COMMAND6_OFFSET) & Comd6::COMMAND6_MASK),
        }
    }
}

impl Default for Comd6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd6Val {
        Comd6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd6Val) -> Comd6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMD7 Register
///
/// I2C command register 7
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **COMMAND7_DONE** `[31]` (RW)
///
///   Represents whether command 7 is done in I2C Master mode.
///   0: Not done
///   1: Done
///
/// - **COMMAND7** `[13:0]` (RW)
///
///   Configures command 7. See details in I2C_CMD0_REG[13:0].
#[repr(transparent)]
pub struct Comd7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comd7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comd7 {
    // ======== Field Constants ========

    /// **COMMAND7_DONE**
    ///
    /// Represents whether command 7 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const COMMAND7_DONE_OFFSET: u32 = 31;
    pub const COMMAND7_DONE_MASK: u32 = 0x80000000;
    pub const COMMAND7_DONE: u32 = 1 << 31;

    /// **COMMAND7**
    ///
    /// Configures command 7. See details in I2C_CMD0_REG[13:0].
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const COMMAND7_OFFSET: u32 = 0;
    pub const COMMAND7_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **COMMAND7_DONE** `[31]` (RW)
    ///
    /// Represents whether command 7 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub fn command7_done(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND7_DONE_MASK) != 0
    }

    /// Set COMMAND7_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command7_done(&self) {
        self.value.modify_bits(|r| r | Self::COMMAND7_DONE_MASK);
    }

    /// Clear COMMAND7_DONE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command7_done(&self) {
        self.value.modify_bits(|r| r & !Self::COMMAND7_DONE_MASK);
    }

    /// **COMMAND7** `[13:0]` (RW)
    ///
    /// Configures command 7. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn command7(&self) -> u32 {
        (self.value.read_bits() & Self::COMMAND7_MASK) >> Self::COMMAND7_OFFSET
    }

    /// Write **COMMAND7** `[13:0]` (RW)
    ///
    /// Configures command 7. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub fn write_command7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMMAND7_MASK) | ((value << Self::COMMAND7_OFFSET) & Self::COMMAND7_MASK)
        });
    }
}

/// COMD7 value type for batch read/modify/write operations
///
/// Use `comd7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comd7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comd7Val {
    bits: u32,
}

impl Comd7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COMMAND7_DONE** `[31:31]`
    ///
    /// Represents whether command 7 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn command7_done(&self) -> bool {
        (self.bits & Comd7::COMMAND7_DONE_MASK) != 0
    }

    /// Read **COMMAND7** `[13:0]`
    ///
    /// Configures command 7. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn command7(&self) -> u32 {
        (self.bits & Comd7::COMMAND7_MASK) >> Comd7::COMMAND7_OFFSET
    }

    /// Set **COMMAND7_DONE** `[31:31]`
    ///
    /// Represents whether command 7 is done in I2C Master mode.
    /// 0: Not done
    /// 1: Done
    #[inline(always)]
    pub const fn with_command7_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comd7::COMMAND7_DONE_MASK
            } else {
                self.bits & !Comd7::COMMAND7_DONE_MASK
            },
        }
    }

    /// Set **COMMAND7** `[13:0]`
    ///
    /// Configures command 7. See details in I2C_CMD0_REG[13:0].
    #[inline(always)]
    pub const fn with_command7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Comd7::COMMAND7_MASK)
                | ((value << Comd7::COMMAND7_OFFSET) & Comd7::COMMAND7_MASK),
        }
    }
}

impl Default for Comd7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comd7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Comd7Val {
        Comd7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comd7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Comd7Val) -> Comd7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_ST_TIME_OUT Register
///
/// SCL status time out register
///
/// Address offset: 0x0078
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **SCL_ST_TO_I2C** `[4:0]` (RW)
///
///   Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclStTimeOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStTimeOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStTimeOut {
    // ======== Field Constants ========

    /// **SCL_ST_TO_I2C**
    ///
    /// Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SCL_ST_TO_I2C_OFFSET: u32 = 0;
    pub const SCL_ST_TO_I2C_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SCL_ST_TO_I2C** `[4:0]` (RW)
    ///
    /// Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_st_to_i2c(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_ST_TO_I2C_MASK) >> Self::SCL_ST_TO_I2C_OFFSET
    }

    /// Write **SCL_ST_TO_I2C** `[4:0]` (RW)
    ///
    /// Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_st_to_i2c(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_ST_TO_I2C_MASK)
                | ((value << Self::SCL_ST_TO_I2C_OFFSET) & Self::SCL_ST_TO_I2C_MASK)
        });
    }
}

/// SCL_ST_TIME_OUT value type for batch read/modify/write operations
///
/// Use `sclsttimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclsttimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStTimeOutVal {
    bits: u32,
}

impl SclStTimeOutVal {
    /// Create from raw bits
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

    /// Read **SCL_ST_TO_I2C** `[4:0]`
    ///
    /// Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_st_to_i2c(&self) -> u32 {
        (self.bits & SclStTimeOut::SCL_ST_TO_I2C_MASK) >> SclStTimeOut::SCL_ST_TO_I2C_OFFSET
    }

    /// Set **SCL_ST_TO_I2C** `[4:0]`
    ///
    /// Configures the threshold value of SCL_FSM state unchanged period. It should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_st_to_i2c(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStTimeOut::SCL_ST_TO_I2C_MASK)
                | ((value << SclStTimeOut::SCL_ST_TO_I2C_OFFSET)
                    & SclStTimeOut::SCL_ST_TO_I2C_MASK),
        }
    }
}

impl Default for SclStTimeOutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStTimeOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStTimeOutVal {
        SclStTimeOutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStTimeOutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStTimeOutVal) -> SclStTimeOutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_MAIN_ST_TIME_OUT Register
///
/// SCL main status time out register
///
/// Address offset: 0x007c
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **SCL_MAIN_ST_TO_I2C** `[4:0]` (RW)
///
///   Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclMainStTimeOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclMainStTimeOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclMainStTimeOut {
    // ======== Field Constants ========

    /// **SCL_MAIN_ST_TO_I2C**
    ///
    /// Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SCL_MAIN_ST_TO_I2C_OFFSET: u32 = 0;
    pub const SCL_MAIN_ST_TO_I2C_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SCL_MAIN_ST_TO_I2C** `[4:0]` (RW)
    ///
    /// Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_main_st_to_i2c(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_MAIN_ST_TO_I2C_MASK) >> Self::SCL_MAIN_ST_TO_I2C_OFFSET
    }

    /// Write **SCL_MAIN_ST_TO_I2C** `[4:0]` (RW)
    ///
    /// Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_main_st_to_i2c(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_MAIN_ST_TO_I2C_MASK)
                | ((value << Self::SCL_MAIN_ST_TO_I2C_OFFSET) & Self::SCL_MAIN_ST_TO_I2C_MASK)
        });
    }
}

/// SCL_MAIN_ST_TIME_OUT value type for batch read/modify/write operations
///
/// Use `sclmainsttimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclmainsttimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclMainStTimeOutVal {
    bits: u32,
}

impl SclMainStTimeOutVal {
    /// Create from raw bits
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

    /// Read **SCL_MAIN_ST_TO_I2C** `[4:0]`
    ///
    /// Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_main_st_to_i2c(&self) -> u32 {
        (self.bits & SclMainStTimeOut::SCL_MAIN_ST_TO_I2C_MASK)
            >> SclMainStTimeOut::SCL_MAIN_ST_TO_I2C_OFFSET
    }

    /// Set **SCL_MAIN_ST_TO_I2C** `[4:0]`
    ///
    /// Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_main_st_to_i2c(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclMainStTimeOut::SCL_MAIN_ST_TO_I2C_MASK)
                | ((value << SclMainStTimeOut::SCL_MAIN_ST_TO_I2C_OFFSET)
                    & SclMainStTimeOut::SCL_MAIN_ST_TO_I2C_MASK),
        }
    }
}

impl Default for SclMainStTimeOutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclMainStTimeOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclMainStTimeOutVal {
        SclMainStTimeOutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclMainStTimeOutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclMainStTimeOutVal) -> SclMainStTimeOutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_SP_CONF Register
///
/// Power configuration register
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **SDA_PD_EN** `[7]` (RW)
///
///   Configures to power down the I2C output SDA line.
///   0: Not power down.
///   1: Power down.
///   Valid only when reg_sda_force_out is 1.
///
/// - **SCL_PD_EN** `[6]` (RW)
///
///   Configures to power down the I2C output SCL line.
///   0: Not power down.
///   1: Power down.
///   Valid only when reg_scl_force_out is 1.
///
/// - **SCL_RST_SLV_NUM** `[5:1]` (RW)
///
///   Configure the pulses of SCL generated in I2C master mode.
///   Valid when reg_scl_rst_slv_en is 1.
///   Measurement unit: i2c_sclk
///
/// - **SCL_RST_SLV_EN** `[0]` (RW)
///
///   Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to reg_scl_rst_slv_num[4:0].
#[repr(transparent)]
pub struct SclSpConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclSpConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclSpConf {
    // ======== Field Constants ========

    /// **SDA_PD_EN**
    ///
    /// Configures to power down the I2C output SDA line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_sda_force_out is 1.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SDA_PD_EN_OFFSET: u32 = 7;
    pub const SDA_PD_EN_MASK: u32 = 0x00000080;
    pub const SDA_PD_EN: u32 = 1 << 7;

    /// **SCL_PD_EN**
    ///
    /// Configures to power down the I2C output SCL line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_scl_force_out is 1.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SCL_PD_EN_OFFSET: u32 = 6;
    pub const SCL_PD_EN_MASK: u32 = 0x00000040;
    pub const SCL_PD_EN: u32 = 1 << 6;

    /// **SCL_RST_SLV_NUM**
    ///
    /// Configure the pulses of SCL generated in I2C master mode.
    /// Valid when reg_scl_rst_slv_en is 1.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[5:1]`, mask `0x0000003e`
    pub const SCL_RST_SLV_NUM_OFFSET: u32 = 1;
    pub const SCL_RST_SLV_NUM_MASK: u32 = 0x0000003e;

    /// **SCL_RST_SLV_EN**
    ///
    /// Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to reg_scl_rst_slv_num[4:0].
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCL_RST_SLV_EN_OFFSET: u32 = 0;
    pub const SCL_RST_SLV_EN_MASK: u32 = 0x00000001;
    pub const SCL_RST_SLV_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SDA_PD_EN** `[7]` (RW)
    ///
    /// Configures to power down the I2C output SDA line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_sda_force_out is 1.
    #[inline(always)]
    pub fn sda_pd_en(&self) -> bool {
        (self.value.read_bits() & Self::SDA_PD_EN_MASK) != 0
    }

    /// Set SDA_PD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sda_pd_en(&self) {
        self.value.modify_bits(|r| r | Self::SDA_PD_EN_MASK);
    }

    /// Clear SDA_PD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sda_pd_en(&self) {
        self.value.modify_bits(|r| r & !Self::SDA_PD_EN_MASK);
    }

    /// **SCL_PD_EN** `[6]` (RW)
    ///
    /// Configures to power down the I2C output SCL line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_scl_force_out is 1.
    #[inline(always)]
    pub fn scl_pd_en(&self) -> bool {
        (self.value.read_bits() & Self::SCL_PD_EN_MASK) != 0
    }

    /// Set SCL_PD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_pd_en(&self) {
        self.value.modify_bits(|r| r | Self::SCL_PD_EN_MASK);
    }

    /// Clear SCL_PD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_pd_en(&self) {
        self.value.modify_bits(|r| r & !Self::SCL_PD_EN_MASK);
    }

    /// **SCL_RST_SLV_NUM** `[5:1]` (RW)
    ///
    /// Configure the pulses of SCL generated in I2C master mode.
    /// Valid when reg_scl_rst_slv_en is 1.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_RST_SLV_NUM_MASK) >> Self::SCL_RST_SLV_NUM_OFFSET
    }

    /// Write **SCL_RST_SLV_NUM** `[5:1]` (RW)
    ///
    /// Configure the pulses of SCL generated in I2C master mode.
    /// Valid when reg_scl_rst_slv_en is 1.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_scl_rst_slv_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCL_RST_SLV_NUM_MASK)
                | ((value << Self::SCL_RST_SLV_NUM_OFFSET) & Self::SCL_RST_SLV_NUM_MASK)
        });
    }

    /// **SCL_RST_SLV_EN** `[0]` (RW)
    ///
    /// Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to reg_scl_rst_slv_num[4:0].
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> bool {
        (self.value.read_bits() & Self::SCL_RST_SLV_EN_MASK) != 0
    }

    /// Set SCL_RST_SLV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scl_rst_slv_en(&self) {
        self.value.modify_bits(|r| r | Self::SCL_RST_SLV_EN_MASK);
    }

    /// Clear SCL_RST_SLV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scl_rst_slv_en(&self) {
        self.value.modify_bits(|r| r & !Self::SCL_RST_SLV_EN_MASK);
    }
}

/// SCL_SP_CONF value type for batch read/modify/write operations
///
/// Use `sclspconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclspconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclSpConfVal {
    bits: u32,
}

impl SclSpConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SDA_PD_EN** `[7:7]`
    ///
    /// Configures to power down the I2C output SDA line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_sda_force_out is 1.
    #[inline(always)]
    pub const fn sda_pd_en(&self) -> bool {
        (self.bits & SclSpConf::SDA_PD_EN_MASK) != 0
    }

    /// Read **SCL_PD_EN** `[6:6]`
    ///
    /// Configures to power down the I2C output SCL line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_scl_force_out is 1.
    #[inline(always)]
    pub const fn scl_pd_en(&self) -> bool {
        (self.bits & SclSpConf::SCL_PD_EN_MASK) != 0
    }

    /// Read **SCL_RST_SLV_NUM** `[5:1]`
    ///
    /// Configure the pulses of SCL generated in I2C master mode.
    /// Valid when reg_scl_rst_slv_en is 1.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn scl_rst_slv_num(&self) -> u32 {
        (self.bits & SclSpConf::SCL_RST_SLV_NUM_MASK) >> SclSpConf::SCL_RST_SLV_NUM_OFFSET
    }

    /// Read **SCL_RST_SLV_EN** `[0:0]`
    ///
    /// Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to reg_scl_rst_slv_num[4:0].
    #[inline(always)]
    pub const fn scl_rst_slv_en(&self) -> bool {
        (self.bits & SclSpConf::SCL_RST_SLV_EN_MASK) != 0
    }

    /// Set **SDA_PD_EN** `[7:7]`
    ///
    /// Configures to power down the I2C output SDA line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_sda_force_out is 1.
    #[inline(always)]
    pub const fn with_sda_pd_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclSpConf::SDA_PD_EN_MASK
            } else {
                self.bits & !SclSpConf::SDA_PD_EN_MASK
            },
        }
    }

    /// Set **SCL_PD_EN** `[6:6]`
    ///
    /// Configures to power down the I2C output SCL line.
    /// 0: Not power down.
    /// 1: Power down.
    /// Valid only when reg_scl_force_out is 1.
    #[inline(always)]
    pub const fn with_scl_pd_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclSpConf::SCL_PD_EN_MASK
            } else {
                self.bits & !SclSpConf::SCL_PD_EN_MASK
            },
        }
    }

    /// Set **SCL_RST_SLV_NUM** `[5:1]`
    ///
    /// Configure the pulses of SCL generated in I2C master mode.
    /// Valid when reg_scl_rst_slv_en is 1.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_scl_rst_slv_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclSpConf::SCL_RST_SLV_NUM_MASK)
                | ((value << SclSpConf::SCL_RST_SLV_NUM_OFFSET) & SclSpConf::SCL_RST_SLV_NUM_MASK),
        }
    }

    /// Set **SCL_RST_SLV_EN** `[0:0]`
    ///
    /// Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to reg_scl_rst_slv_num[4:0].
    #[inline(always)]
    pub const fn with_scl_rst_slv_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclSpConf::SCL_RST_SLV_EN_MASK
            } else {
                self.bits & !SclSpConf::SCL_RST_SLV_EN_MASK
            },
        }
    }
}

impl Default for SclSpConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SclSpConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclSpConfVal {
        SclSpConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclSpConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclSpConfVal) -> SclSpConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_STRETCH_CONF Register
///
/// Set SCL stretch of I2C slave
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **SLAVE_BYTE_ACK_LVL** `[13]` (RW)
///
///   Set the ACK level when slave controlling ACK level function enables.
///   0: Low level
///   1: High level
///
/// - **SLAVE_BYTE_ACK_CTL_EN** `[12]` (RW)
///
///   Configures to enable the function for slave to control ACK level.
///   0: Disable
///   1: Enable
///
/// - **SLAVE_SCL_STRETCH_CLR** `[11]` (WO)
///
///   Configures to clear the I2C slave SCL stretch function.
///   0: No effect
///   1: Clear
///
/// - **SLAVE_SCL_STRETCH_EN** `[10]` (RW)
///
///   Configures to enable slave SCL stretch function.
///   0: Disable
///   1: Enable
///   The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause.
///
/// - **STRETCH_PROTECT_NUM** `[9:0]` (RW)
///
///   Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
///   Measurement unit: i2c_sclk
#[repr(transparent)]
pub struct SclStretchConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStretchConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStretchConf {
    // ======== Field Constants ========

    /// **SLAVE_BYTE_ACK_LVL**
    ///
    /// Set the ACK level when slave controlling ACK level function enables.
    /// 0: Low level
    /// 1: High level
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SLAVE_BYTE_ACK_LVL_OFFSET: u32 = 13;
    pub const SLAVE_BYTE_ACK_LVL_MASK: u32 = 0x00002000;
    pub const SLAVE_BYTE_ACK_LVL: u32 = 1 << 13;

    /// **SLAVE_BYTE_ACK_CTL_EN**
    ///
    /// Configures to enable the function for slave to control ACK level.
    /// 0: Disable
    /// 1: Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SLAVE_BYTE_ACK_CTL_EN_OFFSET: u32 = 12;
    pub const SLAVE_BYTE_ACK_CTL_EN_MASK: u32 = 0x00001000;
    pub const SLAVE_BYTE_ACK_CTL_EN: u32 = 1 << 12;

    /// **SLAVE_SCL_STRETCH_CLR**
    ///
    /// Configures to clear the I2C slave SCL stretch function.
    /// 0: No effect
    /// 1: Clear
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SLAVE_SCL_STRETCH_CLR_OFFSET: u32 = 11;
    pub const SLAVE_SCL_STRETCH_CLR_MASK: u32 = 0x00000800;
    pub const SLAVE_SCL_STRETCH_CLR: u32 = 1 << 11;

    /// **SLAVE_SCL_STRETCH_EN**
    ///
    /// Configures to enable slave SCL stretch function.
    /// 0: Disable
    /// 1: Enable
    /// The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SLAVE_SCL_STRETCH_EN_OFFSET: u32 = 10;
    pub const SLAVE_SCL_STRETCH_EN_MASK: u32 = 0x00000400;
    pub const SLAVE_SCL_STRETCH_EN: u32 = 1 << 10;

    /// **STRETCH_PROTECT_NUM**
    ///
    /// Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
    /// Measurement unit: i2c_sclk
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const STRETCH_PROTECT_NUM_OFFSET: u32 = 0;
    pub const STRETCH_PROTECT_NUM_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **SLAVE_BYTE_ACK_LVL** `[13]` (RW)
    ///
    /// Set the ACK level when slave controlling ACK level function enables.
    /// 0: Low level
    /// 1: High level
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_BYTE_ACK_LVL_MASK) != 0
    }

    /// Set SLAVE_BYTE_ACK_LVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slave_byte_ack_lvl(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_BYTE_ACK_LVL_MASK);
    }

    /// Clear SLAVE_BYTE_ACK_LVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slave_byte_ack_lvl(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLAVE_BYTE_ACK_LVL_MASK);
    }

    /// **SLAVE_BYTE_ACK_CTL_EN** `[12]` (RW)
    ///
    /// Configures to enable the function for slave to control ACK level.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_BYTE_ACK_CTL_EN_MASK) != 0
    }

    /// Set SLAVE_BYTE_ACK_CTL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slave_byte_ack_ctl_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_BYTE_ACK_CTL_EN_MASK);
    }

    /// Clear SLAVE_BYTE_ACK_CTL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slave_byte_ack_ctl_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLAVE_BYTE_ACK_CTL_EN_MASK);
    }

    /// Trigger **SLAVE_SCL_STRETCH_CLR** (WO)
    ///
    /// Configures to clear the I2C slave SCL stretch function.
    /// 0: No effect
    /// 1: Clear
    #[inline(always)]
    pub fn trigger_slave_scl_stretch_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_SCL_STRETCH_CLR_MASK);
    }

    /// **SLAVE_SCL_STRETCH_EN** `[10]` (RW)
    ///
    /// Configures to enable slave SCL stretch function.
    /// 0: Disable
    /// 1: Enable
    /// The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause.
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> bool {
        (self.value.read_bits() & Self::SLAVE_SCL_STRETCH_EN_MASK) != 0
    }

    /// Set SLAVE_SCL_STRETCH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slave_scl_stretch_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SLAVE_SCL_STRETCH_EN_MASK);
    }

    /// Clear SLAVE_SCL_STRETCH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slave_scl_stretch_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLAVE_SCL_STRETCH_EN_MASK);
    }

    /// **STRETCH_PROTECT_NUM** `[9:0]` (RW)
    ///
    /// Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> u32 {
        (self.value.read_bits() & Self::STRETCH_PROTECT_NUM_MASK)
            >> Self::STRETCH_PROTECT_NUM_OFFSET
    }

    /// Write **STRETCH_PROTECT_NUM** `[9:0]` (RW)
    ///
    /// Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn write_stretch_protect_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::STRETCH_PROTECT_NUM_MASK)
                | ((value << Self::STRETCH_PROTECT_NUM_OFFSET) & Self::STRETCH_PROTECT_NUM_MASK)
        });
    }
}

/// SCL_STRETCH_CONF value type for batch read/modify/write operations
///
/// Use `sclstretchconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstretchconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStretchConfVal {
    bits: u32,
}

impl SclStretchConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SLAVE_BYTE_ACK_LVL** `[13:13]`
    ///
    /// Set the ACK level when slave controlling ACK level function enables.
    /// 0: Low level
    /// 1: High level
    #[inline(always)]
    pub const fn slave_byte_ack_lvl(&self) -> bool {
        (self.bits & SclStretchConf::SLAVE_BYTE_ACK_LVL_MASK) != 0
    }

    /// Read **SLAVE_BYTE_ACK_CTL_EN** `[12:12]`
    ///
    /// Configures to enable the function for slave to control ACK level.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn slave_byte_ack_ctl_en(&self) -> bool {
        (self.bits & SclStretchConf::SLAVE_BYTE_ACK_CTL_EN_MASK) != 0
    }

    /// Read **SLAVE_SCL_STRETCH_EN** `[10:10]`
    ///
    /// Configures to enable slave SCL stretch function.
    /// 0: Disable
    /// 1: Enable
    /// The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause.
    #[inline(always)]
    pub const fn slave_scl_stretch_en(&self) -> bool {
        (self.bits & SclStretchConf::SLAVE_SCL_STRETCH_EN_MASK) != 0
    }

    /// Read **STRETCH_PROTECT_NUM** `[9:0]`
    ///
    /// Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn stretch_protect_num(&self) -> u32 {
        (self.bits & SclStretchConf::STRETCH_PROTECT_NUM_MASK)
            >> SclStretchConf::STRETCH_PROTECT_NUM_OFFSET
    }

    /// Set **SLAVE_BYTE_ACK_LVL** `[13:13]`
    ///
    /// Set the ACK level when slave controlling ACK level function enables.
    /// 0: Low level
    /// 1: High level
    #[inline(always)]
    pub const fn with_slave_byte_ack_lvl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclStretchConf::SLAVE_BYTE_ACK_LVL_MASK
            } else {
                self.bits & !SclStretchConf::SLAVE_BYTE_ACK_LVL_MASK
            },
        }
    }

    /// Set **SLAVE_BYTE_ACK_CTL_EN** `[12:12]`
    ///
    /// Configures to enable the function for slave to control ACK level.
    /// 0: Disable
    /// 1: Enable
    #[inline(always)]
    pub const fn with_slave_byte_ack_ctl_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclStretchConf::SLAVE_BYTE_ACK_CTL_EN_MASK
            } else {
                self.bits & !SclStretchConf::SLAVE_BYTE_ACK_CTL_EN_MASK
            },
        }
    }

    /// Set **SLAVE_SCL_STRETCH_CLR** `[11:11]`
    ///
    /// Configures to clear the I2C slave SCL stretch function.
    /// 0: No effect
    /// 1: Clear
    #[inline(always)]
    pub const fn with_slave_scl_stretch_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclStretchConf::SLAVE_SCL_STRETCH_CLR_MASK
            } else {
                self.bits & !SclStretchConf::SLAVE_SCL_STRETCH_CLR_MASK
            },
        }
    }

    /// Set **SLAVE_SCL_STRETCH_EN** `[10:10]`
    ///
    /// Configures to enable slave SCL stretch function.
    /// 0: Disable
    /// 1: Enable
    /// The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause.
    #[inline(always)]
    pub const fn with_slave_scl_stretch_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SclStretchConf::SLAVE_SCL_STRETCH_EN_MASK
            } else {
                self.bits & !SclStretchConf::SLAVE_SCL_STRETCH_EN_MASK
            },
        }
    }

    /// Set **STRETCH_PROTECT_NUM** `[9:0]`
    ///
    /// Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.
    /// Measurement unit: i2c_sclk
    #[inline(always)]
    pub const fn with_stretch_protect_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStretchConf::STRETCH_PROTECT_NUM_MASK)
                | ((value << SclStretchConf::STRETCH_PROTECT_NUM_OFFSET)
                    & SclStretchConf::STRETCH_PROTECT_NUM_MASK),
        }
    }
}

impl Default for SclStretchConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SclStretchConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStretchConfVal {
        SclStretchConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStretchConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStretchConfVal) -> SclStretchConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Version register
///
/// Address offset: 0x00f8
/// Reset value: 0x02201172
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   Version control register.
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
    /// Version control register.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// Version control register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// Version control register.
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
        Self { bits: 0x02201172 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// Version control register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// Version control register.
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

/// TXFIFO_START_ADDR Register
///
/// I2C TXFIFO base address register
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **TXFIFO_START_ADDR** `[31:0]` (RO)
///
///   Represents the I2C txfifo first address.
#[repr(transparent)]
pub struct TxfifoStartAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxfifoStartAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxfifoStartAddr {
    // ======== Field Constants ========

    /// **TXFIFO_START_ADDR**
    ///
    /// Represents the I2C txfifo first address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TXFIFO_START_ADDR_OFFSET: u32 = 0;
    pub const TXFIFO_START_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TXFIFO_START_ADDR** `[31:0]` (RO)
    ///
    /// Represents the I2C txfifo first address.
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> u32 {
        (self.value.read_bits() & Self::TXFIFO_START_ADDR_MASK) >> Self::TXFIFO_START_ADDR_OFFSET
    }
}

/// TXFIFO_START_ADDR value type for batch read/modify/write operations
///
/// Use `txfifostartaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txfifostartaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxfifoStartAddrVal {
    bits: u32,
}

impl TxfifoStartAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TXFIFO_START_ADDR** `[31:0]`
    ///
    /// Represents the I2C txfifo first address.
    #[inline(always)]
    pub const fn txfifo_start_addr(&self) -> u32 {
        (self.bits & TxfifoStartAddr::TXFIFO_START_ADDR_MASK)
            >> TxfifoStartAddr::TXFIFO_START_ADDR_OFFSET
    }
}

impl Default for TxfifoStartAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TxfifoStartAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxfifoStartAddrVal {
        TxfifoStartAddrVal::from_bits(self.value.read_bits())
    }
}

/// RXFIFO_START_ADDR Register
///
/// I2C RXFIFO base address register
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **RXFIFO_START_ADDR** `[31:0]` (RO)
///
///   Represents the I2C rxfifo first address.
#[repr(transparent)]
pub struct RxfifoStartAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxfifoStartAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxfifoStartAddr {
    // ======== Field Constants ========

    /// **RXFIFO_START_ADDR**
    ///
    /// Represents the I2C rxfifo first address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXFIFO_START_ADDR_OFFSET: u32 = 0;
    pub const RXFIFO_START_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXFIFO_START_ADDR** `[31:0]` (RO)
    ///
    /// Represents the I2C rxfifo first address.
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> u32 {
        (self.value.read_bits() & Self::RXFIFO_START_ADDR_MASK) >> Self::RXFIFO_START_ADDR_OFFSET
    }
}

/// RXFIFO_START_ADDR value type for batch read/modify/write operations
///
/// Use `rxfifostartaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxfifostartaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxfifoStartAddrVal {
    bits: u32,
}

impl RxfifoStartAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RXFIFO_START_ADDR** `[31:0]`
    ///
    /// Represents the I2C rxfifo first address.
    #[inline(always)]
    pub const fn rxfifo_start_addr(&self) -> u32 {
        (self.bits & RxfifoStartAddr::RXFIFO_START_ADDR_MASK)
            >> RxfifoStartAddr::RXFIFO_START_ADDR_OFFSET
    }
}

impl Default for RxfifoStartAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RxfifoStartAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxfifoStartAddrVal {
        RxfifoStartAddrVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Configures the low level width of the SCL Clock.
    pub scl_low_period: SclLowPeriod,
    /// 0x0004
    /// Transmission setting
    pub ctr: Ctr,
    /// 0x0008
    /// Describe I2C work status.
    pub sr: Sr,
    /// 0x000c
    /// Setting time out control for receiving data.
    pub to: To,
    /// 0x0010
    /// Local slave address setting
    pub slave_addr: SlaveAddr,
    /// 0x0014
    /// FIFO status register.
    pub fifo_st: FifoSt,
    /// 0x0018
    /// FIFO configuration register.
    pub fifo_conf: FifoConf,
    /// 0x001c
    /// Rx FIFO read data.
    pub data: Data,
    /// 0x0020
    /// Raw interrupt status
    pub int_raw: IntRaw,
    /// 0x0024
    /// Interrupt clear bits
    pub int_clr: IntClr,
    /// 0x0028
    /// Interrupt enable bits
    pub int_ena: IntEna,
    /// 0x002c
    /// Status of captured I2C communication events
    pub int_status: IntStatus,
    /// 0x0030
    /// Configures the hold time after a negative SCL edge.
    pub sda_hold: SdaHold,
    /// 0x0034
    /// Configures the sample time after a positive SCL edge.
    pub sda_sample: SdaSample,
    /// 0x0038
    /// Configures the high level width of SCL
    pub scl_high_period: SclHighPeriod,
    _reserved0: [u8; 4],
    /// 0x0040
    /// Configures the delay between the SDA and SCL negative edge for a start condition
    pub scl_start_hold: SclStartHold,
    /// 0x0044
    /// Configures the delay between the positive edge of SCL and the negative edge of SDA
    pub scl_rstart_setup: SclRstartSetup,
    /// 0x0048
    /// Configures the delay after the SCL clock edge for a stop condition
    pub scl_stop_hold: SclStopHold,
    /// 0x004c
    /// Configures the delay between the SDA and SCL rising edge for a stop condition.
    /// Measurement unit: i2c_sclk
    pub scl_stop_setup: SclStopSetup,
    /// 0x0050
    /// SCL and SDA filter configuration register
    pub filter_cfg: FilterCfg,
    /// 0x0054
    /// I2C CLK configuration register
    pub clk_conf: ClkConf,
    /// 0x0058
    /// I2C command register 0
    pub comd0: Comd0,
    /// 0x005c
    /// I2C command register 1
    pub comd1: Comd1,
    /// 0x0060
    /// I2C command register 2
    pub comd2: Comd2,
    /// 0x0064
    /// I2C command register 3
    pub comd3: Comd3,
    /// 0x0068
    /// I2C command register 4
    pub comd4: Comd4,
    /// 0x006c
    /// I2C command register 5
    pub comd5: Comd5,
    /// 0x0070
    /// I2C command register 6
    pub comd6: Comd6,
    /// 0x0074
    /// I2C command register 7
    pub comd7: Comd7,
    /// 0x0078
    /// SCL status time out register
    pub scl_st_time_out: SclStTimeOut,
    /// 0x007c
    /// SCL main status time out register
    pub scl_main_st_time_out: SclMainStTimeOut,
    /// 0x0080
    /// Power configuration register
    pub scl_sp_conf: SclSpConf,
    /// 0x0084
    /// Set SCL stretch of I2C slave
    pub scl_stretch_conf: SclStretchConf,
    _reserved1: [u8; 112],
    /// 0x00f8
    /// Version register
    pub date: Date,
    _reserved2: [u8; 4],
    /// 0x0100
    /// I2C TXFIFO base address register
    pub txfifo_start_addr: TxfifoStartAddr,
    _reserved3: [u8; 124],
    /// 0x0180
    /// I2C RXFIFO base address register
    pub rxfifo_start_addr: RxfifoStartAddr,
}
