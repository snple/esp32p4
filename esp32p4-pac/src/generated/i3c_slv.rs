//! I3C_SLV
//! I3C Controller (Slave)

#![allow(dead_code)]

pub const BASE: u32 = 0x500db000;

/// CONFIG Register
///
/// NA
///
/// Address offset: 0x0004
/// Reset value: 0x002f0001
///
/// ## Fields
///
/// - **SADDR** `[31:25]` (RW)
///
///   If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
///
/// - **BAMATCH** `[23:16]` (RW)
///
///   Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
///
/// - **OFFLINE** `[9]` (RW)
///
///   NA
///
/// - **IDRAND** `[8]` (RW)
///
///   NA
///
/// - **DDROK** `[4]` (RW)
///
///   NA
///
/// - **S0IGNORE** `[3]` (RW)
///
///   If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR.
///
/// - **MATCHSS** `[2]` (RW)
///
///   1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave.
///
/// - **NACK** `[1]` (RW)
///
///   1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused.
///
/// - **SLVENA** `[0]` (RW)
///
///   1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master
#[repr(transparent)]
pub struct Config {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config {
    // ======== Field Constants ========

    /// **SADDR**
    ///
    /// If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
    ///
    /// Bits `[31:25]`, mask `0xfe000000`
    pub const SADDR_OFFSET: u32 = 25;
    pub const SADDR_MASK: u32 = 0xfe000000;

    /// **BAMATCH**
    ///
    /// Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const BAMATCH_OFFSET: u32 = 16;
    pub const BAMATCH_MASK: u32 = 0x00ff0000;

    /// **OFFLINE**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OFFLINE_OFFSET: u32 = 9;
    pub const OFFLINE_MASK: u32 = 0x00000200;
    pub const OFFLINE: u32 = 1 << 9;

    /// **IDRAND**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IDRAND_OFFSET: u32 = 8;
    pub const IDRAND_MASK: u32 = 0x00000100;
    pub const IDRAND: u32 = 1 << 8;

    /// **DDROK**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DDROK_OFFSET: u32 = 4;
    pub const DDROK_MASK: u32 = 0x00000010;
    pub const DDROK: u32 = 1 << 4;

    /// **S0IGNORE**
    ///
    /// If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const S0IGNORE_OFFSET: u32 = 3;
    pub const S0IGNORE_MASK: u32 = 0x00000008;
    pub const S0IGNORE: u32 = 1 << 3;

    /// **MATCHSS**
    ///
    /// 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MATCHSS_OFFSET: u32 = 2;
    pub const MATCHSS_MASK: u32 = 0x00000004;
    pub const MATCHSS: u32 = 1 << 2;

    /// **NACK**
    ///
    /// 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NACK_OFFSET: u32 = 1;
    pub const NACK_MASK: u32 = 0x00000002;
    pub const NACK: u32 = 1 << 1;

    /// **SLVENA**
    ///
    /// 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLVENA_OFFSET: u32 = 0;
    pub const SLVENA_MASK: u32 = 0x00000001;
    pub const SLVENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SADDR** `[31:25]` (RW)
    ///
    /// If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
    #[inline(always)]
    pub fn saddr(&self) -> u32 {
        (self.value.read_bits() & Self::SADDR_MASK) >> Self::SADDR_OFFSET
    }

    /// Write **SADDR** `[31:25]` (RW)
    ///
    /// If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
    #[inline(always)]
    pub fn write_saddr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SADDR_MASK) | ((value << Self::SADDR_OFFSET) & Self::SADDR_MASK)
        });
    }

    /// **BAMATCH** `[23:16]` (RW)
    ///
    /// Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
    #[inline(always)]
    pub fn bamatch(&self) -> u32 {
        (self.value.read_bits() & Self::BAMATCH_MASK) >> Self::BAMATCH_OFFSET
    }

    /// Write **BAMATCH** `[23:16]` (RW)
    ///
    /// Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
    #[inline(always)]
    pub fn write_bamatch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BAMATCH_MASK) | ((value << Self::BAMATCH_OFFSET) & Self::BAMATCH_MASK)
        });
    }

    /// **OFFLINE** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn offline(&self) -> bool {
        (self.value.read_bits() & Self::OFFLINE_MASK) != 0
    }

    /// Set OFFLINE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_offline(&self) {
        self.value.modify_bits(|r| r | Self::OFFLINE_MASK);
    }

    /// Clear OFFLINE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_offline(&self) {
        self.value.modify_bits(|r| r & !Self::OFFLINE_MASK);
    }

    /// **IDRAND** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn idrand(&self) -> bool {
        (self.value.read_bits() & Self::IDRAND_MASK) != 0
    }

    /// Set IDRAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idrand(&self) {
        self.value.modify_bits(|r| r | Self::IDRAND_MASK);
    }

    /// Clear IDRAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idrand(&self) {
        self.value.modify_bits(|r| r & !Self::IDRAND_MASK);
    }

    /// **DDROK** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ddrok(&self) -> bool {
        (self.value.read_bits() & Self::DDROK_MASK) != 0
    }

    /// Set DDROK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ddrok(&self) {
        self.value.modify_bits(|r| r | Self::DDROK_MASK);
    }

    /// Clear DDROK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ddrok(&self) {
        self.value.modify_bits(|r| r & !Self::DDROK_MASK);
    }

    /// **S0IGNORE** `[3]` (RW)
    ///
    /// If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR.
    #[inline(always)]
    pub fn s0ignore(&self) -> bool {
        (self.value.read_bits() & Self::S0IGNORE_MASK) != 0
    }

    /// Set S0IGNORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_s0ignore(&self) {
        self.value.modify_bits(|r| r | Self::S0IGNORE_MASK);
    }

    /// Clear S0IGNORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_s0ignore(&self) {
        self.value.modify_bits(|r| r & !Self::S0IGNORE_MASK);
    }

    /// **MATCHSS** `[2]` (RW)
    ///
    /// 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave.
    #[inline(always)]
    pub fn matchss(&self) -> bool {
        (self.value.read_bits() & Self::MATCHSS_MASK) != 0
    }

    /// Set MATCHSS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_matchss(&self) {
        self.value.modify_bits(|r| r | Self::MATCHSS_MASK);
    }

    /// Clear MATCHSS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_matchss(&self) {
        self.value.modify_bits(|r| r & !Self::MATCHSS_MASK);
    }

    /// **NACK** `[1]` (RW)
    ///
    /// 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused.
    #[inline(always)]
    pub fn nack(&self) -> bool {
        (self.value.read_bits() & Self::NACK_MASK) != 0
    }

    /// Set NACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nack(&self) {
        self.value.modify_bits(|r| r | Self::NACK_MASK);
    }

    /// Clear NACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nack(&self) {
        self.value.modify_bits(|r| r & !Self::NACK_MASK);
    }

    /// **SLVENA** `[0]` (RW)
    ///
    /// 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master
    #[inline(always)]
    pub fn slvena(&self) -> bool {
        (self.value.read_bits() & Self::SLVENA_MASK) != 0
    }

    /// Set SLVENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slvena(&self) {
        self.value.modify_bits(|r| r | Self::SLVENA_MASK);
    }

    /// Clear SLVENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slvena(&self) {
        self.value.modify_bits(|r| r & !Self::SLVENA_MASK);
    }
}

/// CONFIG value type for batch read/modify/write operations
///
/// Use `config.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfigVal {
    bits: u32,
}

impl ConfigVal {
    /// Create from raw bits
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
        Self { bits: 0x002f0001 }
    }

    /// Read **SADDR** `[31:25]`
    ///
    /// If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        (self.bits & Config::SADDR_MASK) >> Config::SADDR_OFFSET
    }

    /// Read **BAMATCH** `[23:16]`
    ///
    /// Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
    #[inline(always)]
    pub const fn bamatch(&self) -> u32 {
        (self.bits & Config::BAMATCH_MASK) >> Config::BAMATCH_OFFSET
    }

    /// Read **OFFLINE** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn offline(&self) -> bool {
        (self.bits & Config::OFFLINE_MASK) != 0
    }

    /// Read **IDRAND** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn idrand(&self) -> bool {
        (self.bits & Config::IDRAND_MASK) != 0
    }

    /// Read **DDROK** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ddrok(&self) -> bool {
        (self.bits & Config::DDROK_MASK) != 0
    }

    /// Read **S0IGNORE** `[3:3]`
    ///
    /// If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR.
    #[inline(always)]
    pub const fn s0ignore(&self) -> bool {
        (self.bits & Config::S0IGNORE_MASK) != 0
    }

    /// Read **MATCHSS** `[2:2]`
    ///
    /// 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave.
    #[inline(always)]
    pub const fn matchss(&self) -> bool {
        (self.bits & Config::MATCHSS_MASK) != 0
    }

    /// Read **NACK** `[1:1]`
    ///
    /// 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused.
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        (self.bits & Config::NACK_MASK) != 0
    }

    /// Read **SLVENA** `[0:0]`
    ///
    /// 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master
    #[inline(always)]
    pub const fn slvena(&self) -> bool {
        (self.bits & Config::SLVENA_MASK) != 0
    }

    /// Set **SADDR** `[31:25]`
    ///
    /// If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well.
    #[inline(always)]
    pub const fn with_saddr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::SADDR_MASK)
                | ((value << Config::SADDR_OFFSET) & Config::SADDR_MASK),
        }
    }

    /// Set **BAMATCH** `[23:16]`
    ///
    /// Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS
    #[inline(always)]
    pub const fn with_bamatch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::BAMATCH_MASK)
                | ((value << Config::BAMATCH_OFFSET) & Config::BAMATCH_MASK),
        }
    }

    /// Set **OFFLINE** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_offline(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::OFFLINE_MASK
            } else {
                self.bits & !Config::OFFLINE_MASK
            },
        }
    }

    /// Set **IDRAND** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_idrand(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IDRAND_MASK
            } else {
                self.bits & !Config::IDRAND_MASK
            },
        }
    }

    /// Set **DDROK** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ddrok(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::DDROK_MASK
            } else {
                self.bits & !Config::DDROK_MASK
            },
        }
    }

    /// Set **S0IGNORE** `[3:3]`
    ///
    /// If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR.
    #[inline(always)]
    pub const fn with_s0ignore(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::S0IGNORE_MASK
            } else {
                self.bits & !Config::S0IGNORE_MASK
            },
        }
    }

    /// Set **MATCHSS** `[2:2]`
    ///
    /// 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave.
    #[inline(always)]
    pub const fn with_matchss(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::MATCHSS_MASK
            } else {
                self.bits & !Config::MATCHSS_MASK
            },
        }
    }

    /// Set **NACK** `[1:1]`
    ///
    /// 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused.
    #[inline(always)]
    pub const fn with_nack(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::NACK_MASK
            } else {
                self.bits & !Config::NACK_MASK
            },
        }
    }

    /// Set **SLVENA** `[0:0]`
    ///
    /// 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master
    #[inline(always)]
    pub const fn with_slvena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::SLVENA_MASK
            } else {
                self.bits & !Config::SLVENA_MASK
            },
        }
    }
}

impl Default for ConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfigVal {
        ConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfigVal) -> ConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **HDRMATCH** `[16]` (RW)
///
///   NA
///
/// - **ERRWARN** `[15]` (RO)
///
///   NA
///
/// - **CCC** `[14]` (RW)
///
///   A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command.
///
/// - **DACHG** `[13]` (RW)
///
///   The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP.
///
/// - **TXNOTFULL** `[12]` (RO)
///
///   Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more.
///
/// - **RXPEND** `[11]` (RO)
///
///   Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)
///
/// - **STOP** `[10]` (RW)
///
///   NA
///
/// - **MATCHED** `[9]` (RW)
///
///   NA
///
/// - **START** `[8]` (RW)
///
///   NA
///
/// - **STHDR** `[6]` (RO)
///
///   NA
///
/// - **STDAA** `[5]` (RO)
///
///   NA
///
/// - **STREQWR** `[4]` (RO)
///
///   NA
///
/// - **STREQRD** `[3]` (RO)
///
///   1 if the req in process is an sdr read from this slave or an IBI is being pushed out,
///
/// - **STCCCH** `[2]` (RO)
///
///   Is 1 if a CCC message is being handled automatically.
///
/// - **STMSG** `[1]` (RO)
///
///   Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP.
///
/// - **STNOTSTOP** `[0]` (RO)
///
///   Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern.
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

    /// **HDRMATCH**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const HDRMATCH_OFFSET: u32 = 16;
    pub const HDRMATCH_MASK: u32 = 0x00010000;
    pub const HDRMATCH: u32 = 1 << 16;

    /// **ERRWARN**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ERRWARN_OFFSET: u32 = 15;
    pub const ERRWARN_MASK: u32 = 0x00008000;
    pub const ERRWARN: u32 = 1 << 15;

    /// **CCC**
    ///
    /// A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CCC_OFFSET: u32 = 14;
    pub const CCC_MASK: u32 = 0x00004000;
    pub const CCC: u32 = 1 << 14;

    /// **DACHG**
    ///
    /// The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DACHG_OFFSET: u32 = 13;
    pub const DACHG_MASK: u32 = 0x00002000;
    pub const DACHG: u32 = 1 << 13;

    /// **TXNOTFULL**
    ///
    /// Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TXNOTFULL_OFFSET: u32 = 12;
    pub const TXNOTFULL_MASK: u32 = 0x00001000;
    pub const TXNOTFULL: u32 = 1 << 12;

    /// **RXPEND**
    ///
    /// Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RXPEND_OFFSET: u32 = 11;
    pub const RXPEND_MASK: u32 = 0x00000800;
    pub const RXPEND: u32 = 1 << 11;

    /// **STOP**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STOP_OFFSET: u32 = 10;
    pub const STOP_MASK: u32 = 0x00000400;
    pub const STOP: u32 = 1 << 10;

    /// **MATCHED**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MATCHED_OFFSET: u32 = 9;
    pub const MATCHED_MASK: u32 = 0x00000200;
    pub const MATCHED: u32 = 1 << 9;

    /// **START**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const START_OFFSET: u32 = 8;
    pub const START_MASK: u32 = 0x00000100;
    pub const START: u32 = 1 << 8;

    /// **STHDR**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const STHDR_OFFSET: u32 = 6;
    pub const STHDR_MASK: u32 = 0x00000040;
    pub const STHDR: u32 = 1 << 6;

    /// **STDAA**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const STDAA_OFFSET: u32 = 5;
    pub const STDAA_MASK: u32 = 0x00000020;
    pub const STDAA: u32 = 1 << 5;

    /// **STREQWR**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const STREQWR_OFFSET: u32 = 4;
    pub const STREQWR_MASK: u32 = 0x00000010;
    pub const STREQWR: u32 = 1 << 4;

    /// **STREQRD**
    ///
    /// 1 if the req in process is an sdr read from this slave or an IBI is being pushed out,
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const STREQRD_OFFSET: u32 = 3;
    pub const STREQRD_MASK: u32 = 0x00000008;
    pub const STREQRD: u32 = 1 << 3;

    /// **STCCCH**
    ///
    /// Is 1 if a CCC message is being handled automatically.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STCCCH_OFFSET: u32 = 2;
    pub const STCCCH_MASK: u32 = 0x00000004;
    pub const STCCCH: u32 = 1 << 2;

    /// **STMSG**
    ///
    /// Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const STMSG_OFFSET: u32 = 1;
    pub const STMSG_MASK: u32 = 0x00000002;
    pub const STMSG: u32 = 1 << 1;

    /// **STNOTSTOP**
    ///
    /// Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const STNOTSTOP_OFFSET: u32 = 0;
    pub const STNOTSTOP_MASK: u32 = 0x00000001;
    pub const STNOTSTOP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HDRMATCH** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hdrmatch(&self) -> bool {
        (self.value.read_bits() & Self::HDRMATCH_MASK) != 0
    }

    /// Set HDRMATCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hdrmatch(&self) {
        self.value.modify_bits(|r| r | Self::HDRMATCH_MASK);
    }

    /// Clear HDRMATCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hdrmatch(&self) {
        self.value.modify_bits(|r| r & !Self::HDRMATCH_MASK);
    }

    /// **ERRWARN** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn errwarn(&self) -> bool {
        (self.value.read_bits() & Self::ERRWARN_MASK) != 0
    }

    /// **CCC** `[14]` (RW)
    ///
    /// A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command.
    #[inline(always)]
    pub fn ccc(&self) -> bool {
        (self.value.read_bits() & Self::CCC_MASK) != 0
    }

    /// Set CCC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ccc(&self) {
        self.value.modify_bits(|r| r | Self::CCC_MASK);
    }

    /// Clear CCC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ccc(&self) {
        self.value.modify_bits(|r| r & !Self::CCC_MASK);
    }

    /// **DACHG** `[13]` (RW)
    ///
    /// The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP.
    #[inline(always)]
    pub fn dachg(&self) -> bool {
        (self.value.read_bits() & Self::DACHG_MASK) != 0
    }

    /// Set DACHG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dachg(&self) {
        self.value.modify_bits(|r| r | Self::DACHG_MASK);
    }

    /// Clear DACHG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dachg(&self) {
        self.value.modify_bits(|r| r & !Self::DACHG_MASK);
    }

    /// **TXNOTFULL** `[12]` (RO)
    ///
    /// Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more.
    #[inline(always)]
    pub fn txnotfull(&self) -> bool {
        (self.value.read_bits() & Self::TXNOTFULL_MASK) != 0
    }

    /// **RXPEND** `[11]` (RO)
    ///
    /// Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)
    #[inline(always)]
    pub fn rxpend(&self) -> bool {
        (self.value.read_bits() & Self::RXPEND_MASK) != 0
    }

    /// **STOP** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn stop(&self) -> bool {
        (self.value.read_bits() & Self::STOP_MASK) != 0
    }

    /// Set STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stop(&self) {
        self.value.modify_bits(|r| r | Self::STOP_MASK);
    }

    /// Clear STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stop(&self) {
        self.value.modify_bits(|r| r & !Self::STOP_MASK);
    }

    /// **MATCHED** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn matched(&self) -> bool {
        (self.value.read_bits() & Self::MATCHED_MASK) != 0
    }

    /// Set MATCHED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_matched(&self) {
        self.value.modify_bits(|r| r | Self::MATCHED_MASK);
    }

    /// Clear MATCHED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_matched(&self) {
        self.value.modify_bits(|r| r & !Self::MATCHED_MASK);
    }

    /// **START** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn start(&self) -> bool {
        (self.value.read_bits() & Self::START_MASK) != 0
    }

    /// Set START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start(&self) {
        self.value.modify_bits(|r| r | Self::START_MASK);
    }

    /// Clear START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start(&self) {
        self.value.modify_bits(|r| r & !Self::START_MASK);
    }

    /// **STHDR** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn sthdr(&self) -> bool {
        (self.value.read_bits() & Self::STHDR_MASK) != 0
    }

    /// **STDAA** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn stdaa(&self) -> bool {
        (self.value.read_bits() & Self::STDAA_MASK) != 0
    }

    /// **STREQWR** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn streqwr(&self) -> bool {
        (self.value.read_bits() & Self::STREQWR_MASK) != 0
    }

    /// **STREQRD** `[3]` (RO)
    ///
    /// 1 if the req in process is an sdr read from this slave or an IBI is being pushed out,
    #[inline(always)]
    pub fn streqrd(&self) -> bool {
        (self.value.read_bits() & Self::STREQRD_MASK) != 0
    }

    /// **STCCCH** `[2]` (RO)
    ///
    /// Is 1 if a CCC message is being handled automatically.
    #[inline(always)]
    pub fn stccch(&self) -> bool {
        (self.value.read_bits() & Self::STCCCH_MASK) != 0
    }

    /// **STMSG** `[1]` (RO)
    ///
    /// Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP.
    #[inline(always)]
    pub fn stmsg(&self) -> bool {
        (self.value.read_bits() & Self::STMSG_MASK) != 0
    }

    /// **STNOTSTOP** `[0]` (RO)
    ///
    /// Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern.
    #[inline(always)]
    pub fn stnotstop(&self) -> bool {
        (self.value.read_bits() & Self::STNOTSTOP_MASK) != 0
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

    /// Read **HDRMATCH** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hdrmatch(&self) -> bool {
        (self.bits & Status::HDRMATCH_MASK) != 0
    }

    /// Read **ERRWARN** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        (self.bits & Status::ERRWARN_MASK) != 0
    }

    /// Read **CCC** `[14:14]`
    ///
    /// A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command.
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        (self.bits & Status::CCC_MASK) != 0
    }

    /// Read **DACHG** `[13:13]`
    ///
    /// The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP.
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        (self.bits & Status::DACHG_MASK) != 0
    }

    /// Read **TXNOTFULL** `[12:12]`
    ///
    /// Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more.
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        (self.bits & Status::TXNOTFULL_MASK) != 0
    }

    /// Read **RXPEND** `[11:11]`
    ///
    /// Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        (self.bits & Status::RXPEND_MASK) != 0
    }

    /// Read **STOP** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        (self.bits & Status::STOP_MASK) != 0
    }

    /// Read **MATCHED** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        (self.bits & Status::MATCHED_MASK) != 0
    }

    /// Read **START** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn start(&self) -> bool {
        (self.bits & Status::START_MASK) != 0
    }

    /// Read **STHDR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn sthdr(&self) -> bool {
        (self.bits & Status::STHDR_MASK) != 0
    }

    /// Read **STDAA** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn stdaa(&self) -> bool {
        (self.bits & Status::STDAA_MASK) != 0
    }

    /// Read **STREQWR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn streqwr(&self) -> bool {
        (self.bits & Status::STREQWR_MASK) != 0
    }

    /// Read **STREQRD** `[3:3]`
    ///
    /// 1 if the req in process is an sdr read from this slave or an IBI is being pushed out,
    #[inline(always)]
    pub const fn streqrd(&self) -> bool {
        (self.bits & Status::STREQRD_MASK) != 0
    }

    /// Read **STCCCH** `[2:2]`
    ///
    /// Is 1 if a CCC message is being handled automatically.
    #[inline(always)]
    pub const fn stccch(&self) -> bool {
        (self.bits & Status::STCCCH_MASK) != 0
    }

    /// Read **STMSG** `[1:1]`
    ///
    /// Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP.
    #[inline(always)]
    pub const fn stmsg(&self) -> bool {
        (self.bits & Status::STMSG_MASK) != 0
    }

    /// Read **STNOTSTOP** `[0:0]`
    ///
    /// Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern.
    #[inline(always)]
    pub const fn stnotstop(&self) -> bool {
        (self.bits & Status::STNOTSTOP_MASK) != 0
    }

    /// Set **HDRMATCH** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hdrmatch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::HDRMATCH_MASK
            } else {
                self.bits & !Status::HDRMATCH_MASK
            },
        }
    }

    /// Set **CCC** `[14:14]`
    ///
    /// A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command.
    #[inline(always)]
    pub const fn with_ccc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::CCC_MASK
            } else {
                self.bits & !Status::CCC_MASK
            },
        }
    }

    /// Set **DACHG** `[13:13]`
    ///
    /// The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP.
    #[inline(always)]
    pub const fn with_dachg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::DACHG_MASK
            } else {
                self.bits & !Status::DACHG_MASK
            },
        }
    }

    /// Set **STOP** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::STOP_MASK
            } else {
                self.bits & !Status::STOP_MASK
            },
        }
    }

    /// Set **MATCHED** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_matched(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::MATCHED_MASK
            } else {
                self.bits & !Status::MATCHED_MASK
            },
        }
    }

    /// Set **START** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::START_MASK
            } else {
                self.bits & !Status::START_MASK
            },
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatusVal) -> StatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTRL Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **VENDINFO** `[31:24]` (RW)
///
///   NA
///
/// - **ACTSTATE** `[21:20]` (RW)
///
///   NA
///
/// - **PENDINT** `[19:16]` (RW)
///
///   Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
///
/// - **IBIDATA** `[15:8]` (RW)
///
///   Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
///
/// - **MAPIDX** `[7:4]` (RW)
///
///   Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
///
/// - **EXTDATA** `[3]` (RW)
///
///   reserved
///
/// - **SLV_EVENT** `[1:0]` (RW)
///
///   If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
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

    /// **VENDINFO**
    ///
    /// NA
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const VENDINFO_OFFSET: u32 = 24;
    pub const VENDINFO_MASK: u32 = 0xff000000;

    /// **ACTSTATE**
    ///
    /// NA
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const ACTSTATE_OFFSET: u32 = 20;
    pub const ACTSTATE_MASK: u32 = 0x00300000;

    /// **PENDINT**
    ///
    /// Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const PENDINT_OFFSET: u32 = 16;
    pub const PENDINT_MASK: u32 = 0x000f0000;

    /// **IBIDATA**
    ///
    /// Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const IBIDATA_OFFSET: u32 = 8;
    pub const IBIDATA_MASK: u32 = 0x0000ff00;

    /// **MAPIDX**
    ///
    /// Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const MAPIDX_OFFSET: u32 = 4;
    pub const MAPIDX_MASK: u32 = 0x000000f0;

    /// **EXTDATA**
    ///
    /// reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EXTDATA_OFFSET: u32 = 3;
    pub const EXTDATA_MASK: u32 = 0x00000008;
    pub const EXTDATA: u32 = 1 << 3;

    /// **SLV_EVENT**
    ///
    /// If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SLV_EVENT_OFFSET: u32 = 0;
    pub const SLV_EVENT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **VENDINFO** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vendinfo(&self) -> u32 {
        (self.value.read_bits() & Self::VENDINFO_MASK) >> Self::VENDINFO_OFFSET
    }

    /// Write **VENDINFO** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vendinfo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VENDINFO_MASK) | ((value << Self::VENDINFO_OFFSET) & Self::VENDINFO_MASK)
        });
    }

    /// **ACTSTATE** `[21:20]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn actstate(&self) -> u32 {
        (self.value.read_bits() & Self::ACTSTATE_MASK) >> Self::ACTSTATE_OFFSET
    }

    /// Write **ACTSTATE** `[21:20]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_actstate(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ACTSTATE_MASK) | ((value << Self::ACTSTATE_OFFSET) & Self::ACTSTATE_MASK)
        });
    }

    /// **PENDINT** `[19:16]` (RW)
    ///
    /// Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
    #[inline(always)]
    pub fn pendint(&self) -> u32 {
        (self.value.read_bits() & Self::PENDINT_MASK) >> Self::PENDINT_OFFSET
    }

    /// Write **PENDINT** `[19:16]` (RW)
    ///
    /// Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
    #[inline(always)]
    pub fn write_pendint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PENDINT_MASK) | ((value << Self::PENDINT_OFFSET) & Self::PENDINT_MASK)
        });
    }

    /// **IBIDATA** `[15:8]` (RW)
    ///
    /// Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
    #[inline(always)]
    pub fn ibidata(&self) -> u32 {
        (self.value.read_bits() & Self::IBIDATA_MASK) >> Self::IBIDATA_OFFSET
    }

    /// Write **IBIDATA** `[15:8]` (RW)
    ///
    /// Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
    #[inline(always)]
    pub fn write_ibidata(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IBIDATA_MASK) | ((value << Self::IBIDATA_OFFSET) & Self::IBIDATA_MASK)
        });
    }

    /// **MAPIDX** `[7:4]` (RW)
    ///
    /// Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
    #[inline(always)]
    pub fn mapidx(&self) -> u32 {
        (self.value.read_bits() & Self::MAPIDX_MASK) >> Self::MAPIDX_OFFSET
    }

    /// Write **MAPIDX** `[7:4]` (RW)
    ///
    /// Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
    #[inline(always)]
    pub fn write_mapidx(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAPIDX_MASK) | ((value << Self::MAPIDX_OFFSET) & Self::MAPIDX_MASK)
        });
    }

    /// **EXTDATA** `[3]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn extdata(&self) -> bool {
        (self.value.read_bits() & Self::EXTDATA_MASK) != 0
    }

    /// Set EXTDATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_extdata(&self) {
        self.value.modify_bits(|r| r | Self::EXTDATA_MASK);
    }

    /// Clear EXTDATA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_extdata(&self) {
        self.value.modify_bits(|r| r & !Self::EXTDATA_MASK);
    }

    /// **SLV_EVENT** `[1:0]` (RW)
    ///
    /// If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
    #[inline(always)]
    pub fn slv_event(&self) -> u32 {
        (self.value.read_bits() & Self::SLV_EVENT_MASK) >> Self::SLV_EVENT_OFFSET
    }

    /// Write **SLV_EVENT** `[1:0]` (RW)
    ///
    /// If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
    #[inline(always)]
    pub fn write_slv_event(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLV_EVENT_MASK) | ((value << Self::SLV_EVENT_OFFSET) & Self::SLV_EVENT_MASK)
        });
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

    /// Read **VENDINFO** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vendinfo(&self) -> u32 {
        (self.bits & Ctrl::VENDINFO_MASK) >> Ctrl::VENDINFO_OFFSET
    }

    /// Read **ACTSTATE** `[21:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn actstate(&self) -> u32 {
        (self.bits & Ctrl::ACTSTATE_MASK) >> Ctrl::ACTSTATE_OFFSET
    }

    /// Read **PENDINT** `[19:16]`
    ///
    /// Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
    #[inline(always)]
    pub const fn pendint(&self) -> u32 {
        (self.bits & Ctrl::PENDINT_MASK) >> Ctrl::PENDINT_OFFSET
    }

    /// Read **IBIDATA** `[15:8]`
    ///
    /// Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
    #[inline(always)]
    pub const fn ibidata(&self) -> u32 {
        (self.bits & Ctrl::IBIDATA_MASK) >> Ctrl::IBIDATA_OFFSET
    }

    /// Read **MAPIDX** `[7:4]`
    ///
    /// Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
    #[inline(always)]
    pub const fn mapidx(&self) -> u32 {
        (self.bits & Ctrl::MAPIDX_MASK) >> Ctrl::MAPIDX_OFFSET
    }

    /// Read **EXTDATA** `[3:3]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn extdata(&self) -> bool {
        (self.bits & Ctrl::EXTDATA_MASK) != 0
    }

    /// Read **SLV_EVENT** `[1:0]`
    ///
    /// If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
    #[inline(always)]
    pub const fn slv_event(&self) -> u32 {
        (self.bits & Ctrl::SLV_EVENT_MASK) >> Ctrl::SLV_EVENT_OFFSET
    }

    /// Set **VENDINFO** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vendinfo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::VENDINFO_MASK)
                | ((value << Ctrl::VENDINFO_OFFSET) & Ctrl::VENDINFO_MASK),
        }
    }

    /// Set **ACTSTATE** `[21:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_actstate(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::ACTSTATE_MASK)
                | ((value << Ctrl::ACTSTATE_OFFSET) & Ctrl::ACTSTATE_MASK),
        }
    }

    /// Set **PENDINT** `[19:16]`
    ///
    /// Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise.
    #[inline(always)]
    pub const fn with_pendint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::PENDINT_MASK)
                | ((value << Ctrl::PENDINT_OFFSET) & Ctrl::PENDINT_MASK),
        }
    }

    /// Set **IBIDATA** `[15:8]`
    ///
    /// Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required.
    #[inline(always)]
    pub const fn with_ibidata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::IBIDATA_MASK)
                | ((value << Ctrl::IBIDATA_OFFSET) & Ctrl::IBIDATA_MASK),
        }
    }

    /// Set **MAPIDX** `[7:4]`
    ///
    /// Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index.
    #[inline(always)]
    pub const fn with_mapidx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::MAPIDX_MASK)
                | ((value << Ctrl::MAPIDX_OFFSET) & Ctrl::MAPIDX_MASK),
        }
    }

    /// Set **EXTDATA** `[3:3]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_extdata(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::EXTDATA_MASK
            } else {
                self.bits & !Ctrl::EXTDATA_MASK
            },
        }
    }

    /// Set **SLV_EVENT** `[1:0]`
    ///
    /// If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1.
    #[inline(always)]
    pub const fn with_slv_event(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SLV_EVENT_MASK)
                | ((value << Ctrl::SLV_EVENT_OFFSET) & Ctrl::SLV_EVENT_MASK),
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

/// INTSET Register
///
/// INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **TXSEND_ENA** `[12]` (RW)
///
///   NA
///
/// - **RXPEND_ENA** `[11]` (RW)
///
///   Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
///
/// - **STOP_ENA** `[10]` (RW)
///
///   Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
#[repr(transparent)]
pub struct Intset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intset {
    // ======== Field Constants ========

    /// **TXSEND_ENA**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TXSEND_ENA_OFFSET: u32 = 12;
    pub const TXSEND_ENA_MASK: u32 = 0x00001000;
    pub const TXSEND_ENA: u32 = 1 << 12;

    /// **RXPEND_ENA**
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RXPEND_ENA_OFFSET: u32 = 11;
    pub const RXPEND_ENA_MASK: u32 = 0x00000800;
    pub const RXPEND_ENA: u32 = 1 << 11;

    /// **STOP_ENA**
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STOP_ENA_OFFSET: u32 = 10;
    pub const STOP_ENA_MASK: u32 = 0x00000400;
    pub const STOP_ENA: u32 = 1 << 10;

    // ======== Field Accessors ========

    /// **TXSEND_ENA** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn txsend_ena(&self) -> bool {
        (self.value.read_bits() & Self::TXSEND_ENA_MASK) != 0
    }

    /// Set TXSEND_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txsend_ena(&self) {
        self.value.modify_bits(|r| r | Self::TXSEND_ENA_MASK);
    }

    /// Clear TXSEND_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txsend_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TXSEND_ENA_MASK);
    }

    /// **RXPEND_ENA** `[11]` (RW)
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub fn rxpend_ena(&self) -> bool {
        (self.value.read_bits() & Self::RXPEND_ENA_MASK) != 0
    }

    /// Set RXPEND_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxpend_ena(&self) {
        self.value.modify_bits(|r| r | Self::RXPEND_ENA_MASK);
    }

    /// Clear RXPEND_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxpend_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RXPEND_ENA_MASK);
    }

    /// **STOP_ENA** `[10]` (RW)
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub fn stop_ena(&self) -> bool {
        (self.value.read_bits() & Self::STOP_ENA_MASK) != 0
    }

    /// Set STOP_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stop_ena(&self) {
        self.value.modify_bits(|r| r | Self::STOP_ENA_MASK);
    }

    /// Clear STOP_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stop_ena(&self) {
        self.value.modify_bits(|r| r & !Self::STOP_ENA_MASK);
    }
}

/// INTSET value type for batch read/modify/write operations
///
/// Use `intset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntsetVal {
    bits: u32,
}

impl IntsetVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TXSEND_ENA** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn txsend_ena(&self) -> bool {
        (self.bits & Intset::TXSEND_ENA_MASK) != 0
    }

    /// Read **RXPEND_ENA** `[11:11]`
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub const fn rxpend_ena(&self) -> bool {
        (self.bits & Intset::RXPEND_ENA_MASK) != 0
    }

    /// Read **STOP_ENA** `[10:10]`
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub const fn stop_ena(&self) -> bool {
        (self.bits & Intset::STOP_ENA_MASK) != 0
    }

    /// Set **TXSEND_ENA** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_txsend_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intset::TXSEND_ENA_MASK
            } else {
                self.bits & !Intset::TXSEND_ENA_MASK
            },
        }
    }

    /// Set **RXPEND_ENA** `[11:11]`
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub const fn with_rxpend_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intset::RXPEND_ENA_MASK
            } else {
                self.bits & !Intset::RXPEND_ENA_MASK
            },
        }
    }

    /// Set **STOP_ENA** `[10:10]`
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub const fn with_stop_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intset::STOP_ENA_MASK
            } else {
                self.bits & !Intset::STOP_ENA_MASK
            },
        }
    }
}

impl Default for IntsetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Intset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntsetVal {
        IntsetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntsetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntsetVal) -> IntsetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTCLR Register
///
/// NA
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **TXSEND_CLR** `[12]` (WO)
///
///   NA
///
/// - **RXPEND_CLR** `[11]` (WO)
///
///   Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
///
/// - **STOP_CLR** `[10]` (WO)
///
///   Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
#[repr(transparent)]
pub struct Intclr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intclr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intclr {
    // ======== Field Constants ========

    /// **TXSEND_CLR**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TXSEND_CLR_OFFSET: u32 = 12;
    pub const TXSEND_CLR_MASK: u32 = 0x00001000;
    pub const TXSEND_CLR: u32 = 1 << 12;

    /// **RXPEND_CLR**
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RXPEND_CLR_OFFSET: u32 = 11;
    pub const RXPEND_CLR_MASK: u32 = 0x00000800;
    pub const RXPEND_CLR: u32 = 1 << 11;

    /// **STOP_CLR**
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STOP_CLR_OFFSET: u32 = 10;
    pub const STOP_CLR_MASK: u32 = 0x00000400;
    pub const STOP_CLR: u32 = 1 << 10;

    // ======== Field Accessors ========

    /// Trigger **TXSEND_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_txsend_clr(&self) {
        self.value.write_bits(Self::TXSEND_CLR_MASK);
    }

    /// Trigger **RXPEND_CLR** (WO)
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub fn trigger_rxpend_clr(&self) {
        self.value.write_bits(Self::RXPEND_CLR_MASK);
    }

    /// Trigger **STOP_CLR** (WO)
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub fn trigger_stop_clr(&self) {
        self.value.write_bits(Self::STOP_CLR_MASK);
    }
}

/// INTCLR value type for batch read/modify/write operations
///
/// Use `intclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntclrVal {
    bits: u32,
}

impl IntclrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TXSEND_CLR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_txsend_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intclr::TXSEND_CLR_MASK
            } else {
                self.bits & !Intclr::TXSEND_CLR_MASK
            },
        }
    }

    /// Set **RXPEND_CLR** `[11:11]`
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub const fn with_rxpend_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intclr::RXPEND_CLR_MASK
            } else {
                self.bits & !Intclr::RXPEND_CLR_MASK
            },
        }
    }

    /// Set **STOP_CLR** `[10:10]`
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub const fn with_stop_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intclr::STOP_CLR_MASK
            } else {
                self.bits & !Intclr::STOP_CLR_MASK
            },
        }
    }
}

impl Default for IntclrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Intclr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntclrVal) {
        self.value.write_bits(val.bits())
    }
}

/// INTMASKED Register
///
/// NA
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **TXSEND_MASK** `[12]` (RO)
///
///   NA
///
/// - **RXPEND_MASK** `[11]` (RO)
///
///   Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
///
/// - **STOP_MASK** `[10]` (RO)
///
///   Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
#[repr(transparent)]
pub struct Intmasked {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intmasked {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intmasked {
    // ======== Field Constants ========

    /// **TXSEND_MASK**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TXSEND_MASK_OFFSET: u32 = 12;
    pub const TXSEND_MASK_MASK: u32 = 0x00001000;
    pub const TXSEND_MASK: u32 = 1 << 12;

    /// **RXPEND_MASK**
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RXPEND_MASK_OFFSET: u32 = 11;
    pub const RXPEND_MASK_MASK: u32 = 0x00000800;
    pub const RXPEND_MASK: u32 = 1 << 11;

    /// **STOP_MASK**
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STOP_MASK_OFFSET: u32 = 10;
    pub const STOP_MASK_MASK: u32 = 0x00000400;
    pub const STOP_MASK: u32 = 1 << 10;

    // ======== Field Accessors ========

    /// **TXSEND_MASK** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn txsend_mask(&self) -> bool {
        (self.value.read_bits() & Self::TXSEND_MASK_MASK) != 0
    }

    /// **RXPEND_MASK** `[11]` (RO)
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub fn rxpend_mask(&self) -> bool {
        (self.value.read_bits() & Self::RXPEND_MASK_MASK) != 0
    }

    /// **STOP_MASK** `[10]` (RO)
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub fn stop_mask(&self) -> bool {
        (self.value.read_bits() & Self::STOP_MASK_MASK) != 0
    }
}

/// INTMASKED value type for batch read/modify/write operations
///
/// Use `intmasked.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmasked.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntmaskedVal {
    bits: u32,
}

impl IntmaskedVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TXSEND_MASK** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn txsend_mask(&self) -> bool {
        (self.bits & Intmasked::TXSEND_MASK_MASK) != 0
    }

    /// Read **RXPEND_MASK** `[11:11]`
    ///
    /// Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end.
    #[inline(always)]
    pub const fn rxpend_mask(&self) -> bool {
        (self.bits & Intmasked::RXPEND_MASK_MASK) != 0
    }

    /// Read **STOP_MASK** `[10:10]`
    ///
    /// Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped.
    #[inline(always)]
    pub const fn stop_mask(&self) -> bool {
        (self.bits & Intmasked::STOP_MASK_MASK) != 0
    }
}

impl Default for IntmaskedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Intmasked {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntmaskedVal {
        IntmaskedVal::from_bits(self.value.read_bits())
    }
}

/// DATACTRL Register
///
/// NA
///
/// Address offset: 0x002c
/// Reset value: 0x000000b0
///
/// ## Fields
///
/// - **RXEMPTY** `[31]` (RO)
///
///   NA
///
/// - **TXFULL** `[30]` (RO)
///
///   NA
///
/// - **RXCOUNT** `[28:24]` (RO)
///
///   NA
///
/// - **TXCOUNT** `[20:16]` (RO)
///
///   NA
///
/// - **RXTRIG** `[7:6]` (RW)
///
///   Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
///
/// - **TXTRIG** `[5:4]` (RW)
///
///   Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
///
/// - **UNLOCK** `[3]` (WO)
///
///   If this bit is not written 1, the register bits from 7 to 4 are not changed on write.
///
/// - **FLUSHFB** `[1]` (WO)
///
///   Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely
///
/// - **FLUSHTB** `[0]` (WO)
///
///   Flushes the from-bus buffer/FIFO. Not normally used
#[repr(transparent)]
pub struct Datactrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Datactrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Datactrl {
    // ======== Field Constants ========

    /// **RXEMPTY**
    ///
    /// NA
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RXEMPTY_OFFSET: u32 = 31;
    pub const RXEMPTY_MASK: u32 = 0x80000000;
    pub const RXEMPTY: u32 = 1 << 31;

    /// **TXFULL**
    ///
    /// NA
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TXFULL_OFFSET: u32 = 30;
    pub const TXFULL_MASK: u32 = 0x40000000;
    pub const TXFULL: u32 = 1 << 30;

    /// **RXCOUNT**
    ///
    /// NA
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const RXCOUNT_OFFSET: u32 = 24;
    pub const RXCOUNT_MASK: u32 = 0x1f000000;

    /// **TXCOUNT**
    ///
    /// NA
    ///
    /// Bits `[20:16]`, mask `0x001f0000`
    pub const TXCOUNT_OFFSET: u32 = 16;
    pub const TXCOUNT_MASK: u32 = 0x001f0000;

    /// **RXTRIG**
    ///
    /// Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const RXTRIG_OFFSET: u32 = 6;
    pub const RXTRIG_MASK: u32 = 0x000000c0;

    /// **TXTRIG**
    ///
    /// Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const TXTRIG_OFFSET: u32 = 4;
    pub const TXTRIG_MASK: u32 = 0x00000030;

    /// **UNLOCK**
    ///
    /// If this bit is not written 1, the register bits from 7 to 4 are not changed on write.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const UNLOCK_OFFSET: u32 = 3;
    pub const UNLOCK_MASK: u32 = 0x00000008;
    pub const UNLOCK: u32 = 1 << 3;

    /// **FLUSHFB**
    ///
    /// Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FLUSHFB_OFFSET: u32 = 1;
    pub const FLUSHFB_MASK: u32 = 0x00000002;
    pub const FLUSHFB: u32 = 1 << 1;

    /// **FLUSHTB**
    ///
    /// Flushes the from-bus buffer/FIFO. Not normally used
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FLUSHTB_OFFSET: u32 = 0;
    pub const FLUSHTB_MASK: u32 = 0x00000001;
    pub const FLUSHTB: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RXEMPTY** `[31]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn rxempty(&self) -> bool {
        (self.value.read_bits() & Self::RXEMPTY_MASK) != 0
    }

    /// **TXFULL** `[30]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn txfull(&self) -> bool {
        (self.value.read_bits() & Self::TXFULL_MASK) != 0
    }

    /// **RXCOUNT** `[28:24]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn rxcount(&self) -> u32 {
        (self.value.read_bits() & Self::RXCOUNT_MASK) >> Self::RXCOUNT_OFFSET
    }

    /// **TXCOUNT** `[20:16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn txcount(&self) -> u32 {
        (self.value.read_bits() & Self::TXCOUNT_MASK) >> Self::TXCOUNT_OFFSET
    }

    /// **RXTRIG** `[7:6]` (RW)
    ///
    /// Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub fn rxtrig(&self) -> u32 {
        (self.value.read_bits() & Self::RXTRIG_MASK) >> Self::RXTRIG_OFFSET
    }

    /// Write **RXTRIG** `[7:6]` (RW)
    ///
    /// Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub fn write_rxtrig(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXTRIG_MASK) | ((value << Self::RXTRIG_OFFSET) & Self::RXTRIG_MASK)
        });
    }

    /// **TXTRIG** `[5:4]` (RW)
    ///
    /// Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub fn txtrig(&self) -> u32 {
        (self.value.read_bits() & Self::TXTRIG_MASK) >> Self::TXTRIG_OFFSET
    }

    /// Write **TXTRIG** `[5:4]` (RW)
    ///
    /// Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub fn write_txtrig(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXTRIG_MASK) | ((value << Self::TXTRIG_OFFSET) & Self::TXTRIG_MASK)
        });
    }

    /// Trigger **UNLOCK** (WO)
    ///
    /// If this bit is not written 1, the register bits from 7 to 4 are not changed on write.
    #[inline(always)]
    pub fn trigger_unlock(&self) {
        self.value.modify_bits(|r| r | Self::UNLOCK_MASK);
    }

    /// Trigger **FLUSHFB** (WO)
    ///
    /// Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely
    #[inline(always)]
    pub fn trigger_flushfb(&self) {
        self.value.modify_bits(|r| r | Self::FLUSHFB_MASK);
    }

    /// Trigger **FLUSHTB** (WO)
    ///
    /// Flushes the from-bus buffer/FIFO. Not normally used
    #[inline(always)]
    pub fn trigger_flushtb(&self) {
        self.value.modify_bits(|r| r | Self::FLUSHTB_MASK);
    }
}

/// DATACTRL value type for batch read/modify/write operations
///
/// Use `datactrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `datactrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DatactrlVal {
    bits: u32,
}

impl DatactrlVal {
    /// Create from raw bits
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
        Self { bits: 0x000000b0 }
    }

    /// Read **RXEMPTY** `[31:31]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        (self.bits & Datactrl::RXEMPTY_MASK) != 0
    }

    /// Read **TXFULL** `[30:30]`
    ///
    /// NA
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        (self.bits & Datactrl::TXFULL_MASK) != 0
    }

    /// Read **RXCOUNT** `[28:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rxcount(&self) -> u32 {
        (self.bits & Datactrl::RXCOUNT_MASK) >> Datactrl::RXCOUNT_OFFSET
    }

    /// Read **TXCOUNT** `[20:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn txcount(&self) -> u32 {
        (self.bits & Datactrl::TXCOUNT_MASK) >> Datactrl::TXCOUNT_OFFSET
    }

    /// Read **RXTRIG** `[7:6]`
    ///
    /// Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub const fn rxtrig(&self) -> u32 {
        (self.bits & Datactrl::RXTRIG_MASK) >> Datactrl::RXTRIG_OFFSET
    }

    /// Read **TXTRIG** `[5:4]`
    ///
    /// Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub const fn txtrig(&self) -> u32 {
        (self.bits & Datactrl::TXTRIG_MASK) >> Datactrl::TXTRIG_OFFSET
    }

    /// Set **RXTRIG** `[7:6]`
    ///
    /// Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub const fn with_rxtrig(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Datactrl::RXTRIG_MASK)
                | ((value << Datactrl::RXTRIG_OFFSET) & Datactrl::RXTRIG_MASK),
        }
    }

    /// Set **TXTRIG** `[5:4]`
    ///
    /// Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3
    #[inline(always)]
    pub const fn with_txtrig(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Datactrl::TXTRIG_MASK)
                | ((value << Datactrl::TXTRIG_OFFSET) & Datactrl::TXTRIG_MASK),
        }
    }

    /// Set **UNLOCK** `[3:3]`
    ///
    /// If this bit is not written 1, the register bits from 7 to 4 are not changed on write.
    #[inline(always)]
    pub const fn with_unlock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Datactrl::UNLOCK_MASK
            } else {
                self.bits & !Datactrl::UNLOCK_MASK
            },
        }
    }

    /// Set **FLUSHFB** `[1:1]`
    ///
    /// Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely
    #[inline(always)]
    pub const fn with_flushfb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Datactrl::FLUSHFB_MASK
            } else {
                self.bits & !Datactrl::FLUSHFB_MASK
            },
        }
    }

    /// Set **FLUSHTB** `[0:0]`
    ///
    /// Flushes the from-bus buffer/FIFO. Not normally used
    #[inline(always)]
    pub const fn with_flushtb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Datactrl::FLUSHTB_MASK
            } else {
                self.bits & !Datactrl::FLUSHTB_MASK
            },
        }
    }
}

impl Default for DatactrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Datactrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DatactrlVal {
        DatactrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DatactrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DatactrlVal) -> DatactrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDATAB Register
///
/// NA
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **WDATA_END** `[8]` (WO)
///
///   NA
///
/// - **WDATAB** `[7:0]` (WO)
///
///   NA
#[repr(transparent)]
pub struct Wdatab {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdatab {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdatab {
    // ======== Field Constants ========

    /// **WDATA_END**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const WDATA_END_OFFSET: u32 = 8;
    pub const WDATA_END_MASK: u32 = 0x00000100;
    pub const WDATA_END: u32 = 1 << 8;

    /// **WDATAB**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WDATAB_OFFSET: u32 = 0;
    pub const WDATAB_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// Trigger **WDATA_END** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_wdata_end(&self) {
        self.value.write_bits(Self::WDATA_END_MASK);
    }

    /// Write **WDATAB** `[7:0]` (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn write_wdatab(&self, value: u32) {
        self.value
            .write_bits((value << Self::WDATAB_OFFSET) & Self::WDATAB_MASK);
    }
}

/// WDATAB value type for batch read/modify/write operations
///
/// Use `wdatab.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdatab.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdatabVal {
    bits: u32,
}

impl WdatabVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **WDATA_END** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_wdata_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdatab::WDATA_END_MASK
            } else {
                self.bits & !Wdatab::WDATA_END_MASK
            },
        }
    }

    /// Set **WDATAB** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_wdatab(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdatab::WDATAB_MASK)
                | ((value << Wdatab::WDATAB_OFFSET) & Wdatab::WDATAB_MASK),
        }
    }
}

impl Default for WdatabVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Wdatab {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdatabVal) {
        self.value.write_bits(val.bits())
    }
}

/// WDATABE Register
///
/// NA
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **WDATABE** `[7:0]` (WO)
///
///   NA
#[repr(transparent)]
pub struct Wdatabe {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdatabe {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdatabe {
    // ======== Field Constants ========

    /// **WDATABE**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WDATABE_OFFSET: u32 = 0;
    pub const WDATABE_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// Write **WDATABE** `[7:0]` (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn write_wdatabe(&self, value: u32) {
        self.value
            .write_bits((value << Self::WDATABE_OFFSET) & Self::WDATABE_MASK);
    }
}

/// WDATABE value type for batch read/modify/write operations
///
/// Use `wdatabe.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdatabe.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdatabeVal {
    bits: u32,
}

impl WdatabeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **WDATABE** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_wdatabe(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdatabe::WDATABE_MASK)
                | ((value << Wdatabe::WDATABE_OFFSET) & Wdatabe::WDATABE_MASK),
        }
    }
}

impl Default for WdatabeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Wdatabe {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdatabeVal) {
        self.value.write_bits(val.bits())
    }
}

/// RDARAB Register
///
/// Read Byte Data (from-bus) register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **DATA0** `[7:0]` (RO)
///
///   This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register
#[repr(transparent)]
pub struct Rdarab {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rdarab {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rdarab {
    // ======== Field Constants ========

    /// **DATA0**
    ///
    /// This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA0_OFFSET: u32 = 0;
    pub const DATA0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA0** `[7:0]` (RO)
    ///
    /// This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register
    #[inline(always)]
    pub fn data0(&self) -> u32 {
        (self.value.read_bits() & Self::DATA0_MASK) >> Self::DATA0_OFFSET
    }
}

/// RDARAB value type for batch read/modify/write operations
///
/// Use `rdarab.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdarab.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdarabVal {
    bits: u32,
}

impl RdarabVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA0** `[7:0]`
    ///
    /// This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register
    #[inline(always)]
    pub const fn data0(&self) -> u32 {
        (self.bits & Rdarab::DATA0_MASK) >> Rdarab::DATA0_OFFSET
    }
}

impl Default for RdarabVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rdarab {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdarabVal {
        RdarabVal::from_bits(self.value.read_bits())
    }
}

/// RDATAH Register
///
/// Read Half-word Data (from-bus) register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **DATA_MSB** `[15:8]` (RO)
///
///   This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
///
/// - **DATA_LSB** `[7:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct Rdatah {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rdatah {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rdatah {
    // ======== Field Constants ========

    /// **DATA_MSB**
    ///
    /// This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DATA_MSB_OFFSET: u32 = 8;
    pub const DATA_MSB_MASK: u32 = 0x0000ff00;

    /// **DATA_LSB**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_LSB_OFFSET: u32 = 0;
    pub const DATA_LSB_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_MSB** `[15:8]` (RO)
    ///
    /// This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
    #[inline(always)]
    pub fn data_msb(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_MSB_MASK) >> Self::DATA_MSB_OFFSET
    }

    /// **DATA_LSB** `[7:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn data_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_LSB_MASK) >> Self::DATA_LSB_OFFSET
    }
}

/// RDATAH value type for batch read/modify/write operations
///
/// Use `rdatah.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdatah.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdatahVal {
    bits: u32,
}

impl RdatahVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_MSB** `[15:8]`
    ///
    /// This register allows reading a Half-word (byte pair) from the bus unless external FIFO is used. A Half-word should not be read unless there is at least 2 bytes of data waiting, as indicated by the RX FIFO level trigger or RXCOUNT available space in the DATACTRL register
    #[inline(always)]
    pub const fn data_msb(&self) -> u32 {
        (self.bits & Rdatah::DATA_MSB_MASK) >> Rdatah::DATA_MSB_OFFSET
    }

    /// Read **DATA_LSB** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn data_lsb(&self) -> u32 {
        (self.bits & Rdatah::DATA_LSB_MASK) >> Rdatah::DATA_LSB_OFFSET
    }
}

impl Default for RdatahVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rdatah {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdatahVal {
        RdatahVal::from_bits(self.value.read_bits())
    }
}

/// CAPABILITIES2 Register
///
/// NA
///
/// Address offset: 0x005c
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **CAPABLITIES2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct Capabilities2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Capabilities2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Capabilities2 {
    // ======== Field Constants ========

    /// **CAPABLITIES2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CAPABLITIES2_OFFSET: u32 = 0;
    pub const CAPABLITIES2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CAPABLITIES2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn capablities2(&self) -> u32 {
        (self.value.read_bits() & Self::CAPABLITIES2_MASK) >> Self::CAPABLITIES2_OFFSET
    }
}

/// CAPABILITIES2 value type for batch read/modify/write operations
///
/// Use `capabilities2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `capabilities2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Capabilities2Val {
    bits: u32,
}

impl Capabilities2Val {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **CAPABLITIES2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn capablities2(&self) -> u32 {
        (self.bits & Capabilities2::CAPABLITIES2_MASK) >> Capabilities2::CAPABLITIES2_OFFSET
    }
}

impl Default for Capabilities2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Capabilities2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Capabilities2Val {
        Capabilities2Val::from_bits(self.value.read_bits())
    }
}

/// CAPABILITIES Register
///
/// NA
///
/// Address offset: 0x0060
/// Reset value: 0x7c13fc1c
///
/// ## Fields
///
/// - **CAPABLITIES** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct Capabilities {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Capabilities {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Capabilities {
    // ======== Field Constants ========

    /// **CAPABLITIES**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CAPABLITIES_OFFSET: u32 = 0;
    pub const CAPABLITIES_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CAPABLITIES** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn capablities(&self) -> u32 {
        (self.value.read_bits() & Self::CAPABLITIES_MASK) >> Self::CAPABLITIES_OFFSET
    }
}

/// CAPABILITIES value type for batch read/modify/write operations
///
/// Use `capabilities.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `capabilities.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CapabilitiesVal {
    bits: u32,
}

impl CapabilitiesVal {
    /// Create from raw bits
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
        Self { bits: 0x7c13fc1c }
    }

    /// Read **CAPABLITIES** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn capablities(&self) -> u32 {
        (self.bits & Capabilities::CAPABLITIES_MASK) >> Capabilities::CAPABLITIES_OFFSET
    }
}

impl Default for CapabilitiesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Capabilities {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CapabilitiesVal {
        CapabilitiesVal::from_bits(self.value.read_bits())
    }
}

/// IDPARTNO Register
///
/// NA
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **PARTNO** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Idpartno {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idpartno {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idpartno {
    // ======== Field Constants ========

    /// **PARTNO**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PARTNO_OFFSET: u32 = 0;
    pub const PARTNO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PARTNO** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn partno(&self) -> u32 {
        (self.value.read_bits() & Self::PARTNO_MASK) >> Self::PARTNO_OFFSET
    }

    /// Write **PARTNO** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_partno(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PARTNO_MASK) | ((value << Self::PARTNO_OFFSET) & Self::PARTNO_MASK)
        });
    }
}

/// IDPARTNO value type for batch read/modify/write operations
///
/// Use `idpartno.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idpartno.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdpartnoVal {
    bits: u32,
}

impl IdpartnoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PARTNO** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn partno(&self) -> u32 {
        (self.bits & Idpartno::PARTNO_MASK) >> Idpartno::PARTNO_OFFSET
    }

    /// Set **PARTNO** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_partno(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idpartno::PARTNO_MASK)
                | ((value << Idpartno::PARTNO_OFFSET) & Idpartno::PARTNO_MASK),
        }
    }
}

impl Default for IdpartnoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Idpartno {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdpartnoVal {
        IdpartnoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdpartnoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdpartnoVal) -> IdpartnoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDEXT Register
///
/// NA
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **IDEXT** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Idext {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idext {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idext {
    // ======== Field Constants ========

    /// **IDEXT**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IDEXT_OFFSET: u32 = 0;
    pub const IDEXT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IDEXT** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn idext(&self) -> u32 {
        (self.value.read_bits() & Self::IDEXT_MASK) >> Self::IDEXT_OFFSET
    }

    /// Write **IDEXT** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_idext(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IDEXT_MASK) | ((value << Self::IDEXT_OFFSET) & Self::IDEXT_MASK)
        });
    }
}

/// IDEXT value type for batch read/modify/write operations
///
/// Use `idext.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idext.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdextVal {
    bits: u32,
}

impl IdextVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IDEXT** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn idext(&self) -> u32 {
        (self.bits & Idext::IDEXT_MASK) >> Idext::IDEXT_OFFSET
    }

    /// Set **IDEXT** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_idext(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idext::IDEXT_MASK)
                | ((value << Idext::IDEXT_OFFSET) & Idext::IDEXT_MASK),
        }
    }
}

impl Default for IdextVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Idext {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdextVal {
        IdextVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IdextVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IdextVal) -> IdextVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VENDORID Register
///
/// NA
///
/// Address offset: 0x0074
/// Reset value: 0x00005550
///
/// ## Fields
///
/// - **VID** `[14:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Vendorid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Vendorid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Vendorid {
    // ======== Field Constants ========

    /// **VID**
    ///
    /// NA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const VID_OFFSET: u32 = 0;
    pub const VID_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **VID** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid(&self) -> u32 {
        (self.value.read_bits() & Self::VID_MASK) >> Self::VID_OFFSET
    }

    /// Write **VID** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_MASK) | ((value << Self::VID_OFFSET) & Self::VID_MASK)
        });
    }
}

/// VENDORID value type for batch read/modify/write operations
///
/// Use `vendorid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vendorid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VendoridVal {
    bits: u32,
}

impl VendoridVal {
    /// Create from raw bits
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
        Self { bits: 0x00005550 }
    }

    /// Read **VID** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid(&self) -> u32 {
        (self.bits & Vendorid::VID_MASK) >> Vendorid::VID_OFFSET
    }

    /// Set **VID** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Vendorid::VID_MASK)
                | ((value << Vendorid::VID_OFFSET) & Vendorid::VID_MASK),
        }
    }
}

impl Default for VendoridVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Vendorid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VendoridVal {
        VendoridVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VendoridVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VendoridVal) -> VendoridVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4],
    /// 0x0004
    /// NA
    pub config: Config,
    /// 0x0008
    /// NA
    pub status: Status,
    /// 0x000c
    /// NA
    pub ctrl: Ctrl,
    /// 0x0010
    /// INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)
    pub intset: Intset,
    /// 0x0014
    /// NA
    pub intclr: Intclr,
    /// 0x0018
    /// NA
    pub intmasked: Intmasked,
    _reserved1: [u8; 16],
    /// 0x002c
    /// NA
    pub datactrl: Datactrl,
    /// 0x0030
    /// NA
    pub wdatab: Wdatab,
    /// 0x0034
    /// NA
    pub wdatabe: Wdatabe,
    _reserved2: [u8; 8],
    /// 0x0040
    /// Read Byte Data (from-bus) register
    pub rdarab: Rdarab,
    _reserved3: [u8; 4],
    /// 0x0048
    /// Read Half-word Data (from-bus) register
    pub rdatah: Rdatah,
    _reserved4: [u8; 16],
    /// 0x005c
    /// NA
    pub capabilities2: Capabilities2,
    /// 0x0060
    /// NA
    pub capabilities: Capabilities,
    _reserved5: [u8; 8],
    /// 0x006c
    /// NA
    pub idpartno: Idpartno,
    /// 0x0070
    /// NA
    pub idext: Idext,
    /// 0x0074
    /// NA
    pub vendorid: Vendorid,
}
