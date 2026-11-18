//! LP_I2C_ANA_MST
//! LP_I2C_ANA_MST Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50124000;

/// I2C0_CTRL Register
///
/// need des
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **I2C0_BUSY** `[25]` (RO)
///
///   need des
///
/// - **I2C0_CTRL** `[24:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c0Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c0Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c0Ctrl {
    // ======== Field Constants ========

    /// **I2C0_BUSY**
    ///
    /// need des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const I2C0_BUSY_OFFSET: u32 = 25;
    pub const I2C0_BUSY_MASK: u32 = 0x02000000;
    pub const I2C0_BUSY: u32 = 1 << 25;

    /// **I2C0_CTRL**
    ///
    /// need des
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const I2C0_CTRL_OFFSET: u32 = 0;
    pub const I2C0_CTRL_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **I2C0_BUSY** `[25]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_busy(&self) -> bool {
        (self.value.read_bits() & Self::I2C0_BUSY_MASK) != 0
    }

    /// **I2C0_CTRL** `[24:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::I2C0_CTRL_MASK) >> Self::I2C0_CTRL_OFFSET
    }

    /// Write **I2C0_CTRL** `[24:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c0_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C0_CTRL_MASK) | ((value << Self::I2C0_CTRL_OFFSET) & Self::I2C0_CTRL_MASK)
        });
    }
}

/// I2C0_CTRL value type for batch read/modify/write operations
///
/// Use `i2c0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c0CtrlVal {
    bits: u32,
}

impl I2c0CtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C0_BUSY** `[25:25]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_busy(&self) -> bool {
        (self.bits & I2c0Ctrl::I2C0_BUSY_MASK) != 0
    }

    /// Read **I2C0_CTRL** `[24:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_ctrl(&self) -> u32 {
        (self.bits & I2c0Ctrl::I2C0_CTRL_MASK) >> I2c0Ctrl::I2C0_CTRL_OFFSET
    }

    /// Set **I2C0_CTRL** `[24:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c0_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c0Ctrl::I2C0_CTRL_MASK)
                | ((value << I2c0Ctrl::I2C0_CTRL_OFFSET) & I2c0Ctrl::I2C0_CTRL_MASK),
        }
    }
}

impl Default for I2c0CtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c0CtrlVal {
        I2c0CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c0CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c0CtrlVal) -> I2c0CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C1_CTRL Register
///
/// need des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **I2C1_BUSY** `[25]` (RO)
///
///   need des
///
/// - **I2C1_CTRL** `[24:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c1Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c1Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c1Ctrl {
    // ======== Field Constants ========

    /// **I2C1_BUSY**
    ///
    /// need des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const I2C1_BUSY_OFFSET: u32 = 25;
    pub const I2C1_BUSY_MASK: u32 = 0x02000000;
    pub const I2C1_BUSY: u32 = 1 << 25;

    /// **I2C1_CTRL**
    ///
    /// need des
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const I2C1_CTRL_OFFSET: u32 = 0;
    pub const I2C1_CTRL_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **I2C1_BUSY** `[25]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_busy(&self) -> bool {
        (self.value.read_bits() & Self::I2C1_BUSY_MASK) != 0
    }

    /// **I2C1_CTRL** `[24:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1_CTRL_MASK) >> Self::I2C1_CTRL_OFFSET
    }

    /// Write **I2C1_CTRL** `[24:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c1_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C1_CTRL_MASK) | ((value << Self::I2C1_CTRL_OFFSET) & Self::I2C1_CTRL_MASK)
        });
    }
}

/// I2C1_CTRL value type for batch read/modify/write operations
///
/// Use `i2c1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c1CtrlVal {
    bits: u32,
}

impl I2c1CtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C1_BUSY** `[25:25]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_busy(&self) -> bool {
        (self.bits & I2c1Ctrl::I2C1_BUSY_MASK) != 0
    }

    /// Read **I2C1_CTRL** `[24:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_ctrl(&self) -> u32 {
        (self.bits & I2c1Ctrl::I2C1_CTRL_MASK) >> I2c1Ctrl::I2C1_CTRL_OFFSET
    }

    /// Set **I2C1_CTRL** `[24:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c1_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c1Ctrl::I2C1_CTRL_MASK)
                | ((value << I2c1Ctrl::I2C1_CTRL_OFFSET) & I2c1Ctrl::I2C1_CTRL_MASK),
        }
    }
}

impl Default for I2c1CtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c1CtrlVal {
        I2c1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c1CtrlVal) -> I2c1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C0_CONF Register
///
/// need des
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **I2C0_STATUS** `[31:24]` (RO)
///
///   need des
///
/// - **I2C0_CONF** `[23:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c0Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c0Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c0Conf {
    // ======== Field Constants ========

    /// **I2C0_STATUS**
    ///
    /// need des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const I2C0_STATUS_OFFSET: u32 = 24;
    pub const I2C0_STATUS_MASK: u32 = 0xff000000;

    /// **I2C0_CONF**
    ///
    /// need des
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const I2C0_CONF_OFFSET: u32 = 0;
    pub const I2C0_CONF_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **I2C0_STATUS** `[31:24]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_status(&self) -> u32 {
        (self.value.read_bits() & Self::I2C0_STATUS_MASK) >> Self::I2C0_STATUS_OFFSET
    }

    /// **I2C0_CONF** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_conf(&self) -> u32 {
        (self.value.read_bits() & Self::I2C0_CONF_MASK) >> Self::I2C0_CONF_OFFSET
    }

    /// Write **I2C0_CONF** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c0_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C0_CONF_MASK) | ((value << Self::I2C0_CONF_OFFSET) & Self::I2C0_CONF_MASK)
        });
    }
}

/// I2C0_CONF value type for batch read/modify/write operations
///
/// Use `i2c0conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c0conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c0ConfVal {
    bits: u32,
}

impl I2c0ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C0_STATUS** `[31:24]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_status(&self) -> u32 {
        (self.bits & I2c0Conf::I2C0_STATUS_MASK) >> I2c0Conf::I2C0_STATUS_OFFSET
    }

    /// Read **I2C0_CONF** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> u32 {
        (self.bits & I2c0Conf::I2C0_CONF_MASK) >> I2c0Conf::I2C0_CONF_OFFSET
    }

    /// Set **I2C0_CONF** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c0_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c0Conf::I2C0_CONF_MASK)
                | ((value << I2c0Conf::I2C0_CONF_OFFSET) & I2c0Conf::I2C0_CONF_MASK),
        }
    }
}

impl Default for I2c0ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c0Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c0ConfVal {
        I2c0ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c0ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c0ConfVal) -> I2c0ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C1_CONF Register
///
/// need des
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **I2C1_STATUS** `[31:24]` (RO)
///
///   need des
///
/// - **I2C1_CONF** `[23:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c1Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c1Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c1Conf {
    // ======== Field Constants ========

    /// **I2C1_STATUS**
    ///
    /// need des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const I2C1_STATUS_OFFSET: u32 = 24;
    pub const I2C1_STATUS_MASK: u32 = 0xff000000;

    /// **I2C1_CONF**
    ///
    /// need des
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const I2C1_CONF_OFFSET: u32 = 0;
    pub const I2C1_CONF_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **I2C1_STATUS** `[31:24]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_status(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1_STATUS_MASK) >> Self::I2C1_STATUS_OFFSET
    }

    /// **I2C1_CONF** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_conf(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1_CONF_MASK) >> Self::I2C1_CONF_OFFSET
    }

    /// Write **I2C1_CONF** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c1_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C1_CONF_MASK) | ((value << Self::I2C1_CONF_OFFSET) & Self::I2C1_CONF_MASK)
        });
    }
}

/// I2C1_CONF value type for batch read/modify/write operations
///
/// Use `i2c1conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c1conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c1ConfVal {
    bits: u32,
}

impl I2c1ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C1_STATUS** `[31:24]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_status(&self) -> u32 {
        (self.bits & I2c1Conf::I2C1_STATUS_MASK) >> I2c1Conf::I2C1_STATUS_OFFSET
    }

    /// Read **I2C1_CONF** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> u32 {
        (self.bits & I2c1Conf::I2C1_CONF_MASK) >> I2c1Conf::I2C1_CONF_OFFSET
    }

    /// Set **I2C1_CONF** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c1_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c1Conf::I2C1_CONF_MASK)
                | ((value << I2c1Conf::I2C1_CONF_OFFSET) & I2c1Conf::I2C1_CONF_MASK),
        }
    }
}

impl Default for I2c1ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c1Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c1ConfVal {
        I2c1ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c1ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c1ConfVal) -> I2c1ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C_BURST_CONF Register
///
/// need des
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **I2C_MST_BURST_CTRL** `[31:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2cBurstConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2cBurstConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2cBurstConf {
    // ======== Field Constants ========

    /// **I2C_MST_BURST_CTRL**
    ///
    /// need des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const I2C_MST_BURST_CTRL_OFFSET: u32 = 0;
    pub const I2C_MST_BURST_CTRL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **I2C_MST_BURST_CTRL** `[31:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst_burst_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::I2C_MST_BURST_CTRL_MASK) >> Self::I2C_MST_BURST_CTRL_OFFSET
    }

    /// Write **I2C_MST_BURST_CTRL** `[31:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c_mst_burst_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C_MST_BURST_CTRL_MASK)
                | ((value << Self::I2C_MST_BURST_CTRL_OFFSET) & Self::I2C_MST_BURST_CTRL_MASK)
        });
    }
}

/// I2C_BURST_CONF value type for batch read/modify/write operations
///
/// Use `i2cburstconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2cburstconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2cBurstConfVal {
    bits: u32,
}

impl I2cBurstConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C_MST_BURST_CTRL** `[31:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst_burst_ctrl(&self) -> u32 {
        (self.bits & I2cBurstConf::I2C_MST_BURST_CTRL_MASK)
            >> I2cBurstConf::I2C_MST_BURST_CTRL_OFFSET
    }

    /// Set **I2C_MST_BURST_CTRL** `[31:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c_mst_burst_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2cBurstConf::I2C_MST_BURST_CTRL_MASK)
                | ((value << I2cBurstConf::I2C_MST_BURST_CTRL_OFFSET)
                    & I2cBurstConf::I2C_MST_BURST_CTRL_MASK),
        }
    }
}

impl Default for I2cBurstConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2cBurstConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2cBurstConfVal {
        I2cBurstConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2cBurstConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2cBurstConfVal) -> I2cBurstConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C_BURST_STATUS Register
///
/// need des
///
/// Address offset: 0x0014
/// Reset value: 0x40000000
///
/// ## Fields
///
/// - **I2C_MST_BURST_TIMEOUT_CNT** `[31:20]` (RW)
///
///   need des
///
/// - **I2C_MST1_BURST_ERR_FLAG** `[2]` (RO)
///
///   need des
///
/// - **I2C_MST0_BURST_ERR_FLAG** `[1]` (RO)
///
///   need des
///
/// - **I2C_MST_BURST_DONE** `[0]` (RO)
///
///   need des
#[repr(transparent)]
pub struct I2cBurstStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2cBurstStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2cBurstStatus {
    // ======== Field Constants ========

    /// **I2C_MST_BURST_TIMEOUT_CNT**
    ///
    /// need des
    ///
    /// Bits `[31:20]`, mask `0xfff00000`
    pub const I2C_MST_BURST_TIMEOUT_CNT_OFFSET: u32 = 20;
    pub const I2C_MST_BURST_TIMEOUT_CNT_MASK: u32 = 0xfff00000;

    /// **I2C_MST1_BURST_ERR_FLAG**
    ///
    /// need des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const I2C_MST1_BURST_ERR_FLAG_OFFSET: u32 = 2;
    pub const I2C_MST1_BURST_ERR_FLAG_MASK: u32 = 0x00000004;
    pub const I2C_MST1_BURST_ERR_FLAG: u32 = 1 << 2;

    /// **I2C_MST0_BURST_ERR_FLAG**
    ///
    /// need des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const I2C_MST0_BURST_ERR_FLAG_OFFSET: u32 = 1;
    pub const I2C_MST0_BURST_ERR_FLAG_MASK: u32 = 0x00000002;
    pub const I2C_MST0_BURST_ERR_FLAG: u32 = 1 << 1;

    /// **I2C_MST_BURST_DONE**
    ///
    /// need des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const I2C_MST_BURST_DONE_OFFSET: u32 = 0;
    pub const I2C_MST_BURST_DONE_MASK: u32 = 0x00000001;
    pub const I2C_MST_BURST_DONE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **I2C_MST_BURST_TIMEOUT_CNT** `[31:20]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst_burst_timeout_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::I2C_MST_BURST_TIMEOUT_CNT_MASK)
            >> Self::I2C_MST_BURST_TIMEOUT_CNT_OFFSET
    }

    /// Write **I2C_MST_BURST_TIMEOUT_CNT** `[31:20]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c_mst_burst_timeout_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C_MST_BURST_TIMEOUT_CNT_MASK)
                | ((value << Self::I2C_MST_BURST_TIMEOUT_CNT_OFFSET)
                    & Self::I2C_MST_BURST_TIMEOUT_CNT_MASK)
        });
    }

    /// **I2C_MST1_BURST_ERR_FLAG** `[2]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst1_burst_err_flag(&self) -> bool {
        (self.value.read_bits() & Self::I2C_MST1_BURST_ERR_FLAG_MASK) != 0
    }

    /// **I2C_MST0_BURST_ERR_FLAG** `[1]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst0_burst_err_flag(&self) -> bool {
        (self.value.read_bits() & Self::I2C_MST0_BURST_ERR_FLAG_MASK) != 0
    }

    /// **I2C_MST_BURST_DONE** `[0]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst_burst_done(&self) -> bool {
        (self.value.read_bits() & Self::I2C_MST_BURST_DONE_MASK) != 0
    }
}

/// I2C_BURST_STATUS value type for batch read/modify/write operations
///
/// Use `i2cburststatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2cburststatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2cBurstStatusVal {
    bits: u32,
}

impl I2cBurstStatusVal {
    /// Create from raw bits
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
        Self { bits: 0x40000000 }
    }

    /// Read **I2C_MST_BURST_TIMEOUT_CNT** `[31:20]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst_burst_timeout_cnt(&self) -> u32 {
        (self.bits & I2cBurstStatus::I2C_MST_BURST_TIMEOUT_CNT_MASK)
            >> I2cBurstStatus::I2C_MST_BURST_TIMEOUT_CNT_OFFSET
    }

    /// Read **I2C_MST1_BURST_ERR_FLAG** `[2:2]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst1_burst_err_flag(&self) -> bool {
        (self.bits & I2cBurstStatus::I2C_MST1_BURST_ERR_FLAG_MASK) != 0
    }

    /// Read **I2C_MST0_BURST_ERR_FLAG** `[1:1]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst0_burst_err_flag(&self) -> bool {
        (self.bits & I2cBurstStatus::I2C_MST0_BURST_ERR_FLAG_MASK) != 0
    }

    /// Read **I2C_MST_BURST_DONE** `[0:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst_burst_done(&self) -> bool {
        (self.bits & I2cBurstStatus::I2C_MST_BURST_DONE_MASK) != 0
    }

    /// Set **I2C_MST_BURST_TIMEOUT_CNT** `[31:20]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c_mst_burst_timeout_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2cBurstStatus::I2C_MST_BURST_TIMEOUT_CNT_MASK)
                | ((value << I2cBurstStatus::I2C_MST_BURST_TIMEOUT_CNT_OFFSET)
                    & I2cBurstStatus::I2C_MST_BURST_TIMEOUT_CNT_MASK),
        }
    }
}

impl Default for I2cBurstStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl I2cBurstStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2cBurstStatusVal {
        I2cBurstStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2cBurstStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2cBurstStatusVal) -> I2cBurstStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ANA_CONF0 Register
///
/// need des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **ANA_STATUS0** `[31:24]` (RO)
///
///   need des
///
/// - **ANA_CONF0** `[23:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct AnaConf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AnaConf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AnaConf0 {
    // ======== Field Constants ========

    /// **ANA_STATUS0**
    ///
    /// need des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const ANA_STATUS0_OFFSET: u32 = 24;
    pub const ANA_STATUS0_MASK: u32 = 0xff000000;

    /// **ANA_CONF0**
    ///
    /// need des
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const ANA_CONF0_OFFSET: u32 = 0;
    pub const ANA_CONF0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **ANA_STATUS0** `[31:24]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_status0(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_STATUS0_MASK) >> Self::ANA_STATUS0_OFFSET
    }

    /// **ANA_CONF0** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_conf0(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_CONF0_MASK) >> Self::ANA_CONF0_OFFSET
    }

    /// Write **ANA_CONF0** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_ana_conf0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ANA_CONF0_MASK) | ((value << Self::ANA_CONF0_OFFSET) & Self::ANA_CONF0_MASK)
        });
    }
}

/// ANA_CONF0 value type for batch read/modify/write operations
///
/// Use `anaconf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `anaconf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AnaConf0Val {
    bits: u32,
}

impl AnaConf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ANA_STATUS0** `[31:24]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_status0(&self) -> u32 {
        (self.bits & AnaConf0::ANA_STATUS0_MASK) >> AnaConf0::ANA_STATUS0_OFFSET
    }

    /// Read **ANA_CONF0** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_conf0(&self) -> u32 {
        (self.bits & AnaConf0::ANA_CONF0_MASK) >> AnaConf0::ANA_CONF0_OFFSET
    }

    /// Set **ANA_CONF0** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_ana_conf0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AnaConf0::ANA_CONF0_MASK)
                | ((value << AnaConf0::ANA_CONF0_OFFSET) & AnaConf0::ANA_CONF0_MASK),
        }
    }
}

impl Default for AnaConf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AnaConf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AnaConf0Val {
        AnaConf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AnaConf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AnaConf0Val) -> AnaConf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ANA_CONF1 Register
///
/// need des
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **ANA_STATUS1** `[31:24]` (RO)
///
///   need des
///
/// - **ANA_CONF1** `[23:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct AnaConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AnaConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AnaConf1 {
    // ======== Field Constants ========

    /// **ANA_STATUS1**
    ///
    /// need des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const ANA_STATUS1_OFFSET: u32 = 24;
    pub const ANA_STATUS1_MASK: u32 = 0xff000000;

    /// **ANA_CONF1**
    ///
    /// need des
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const ANA_CONF1_OFFSET: u32 = 0;
    pub const ANA_CONF1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **ANA_STATUS1** `[31:24]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_status1(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_STATUS1_MASK) >> Self::ANA_STATUS1_OFFSET
    }

    /// **ANA_CONF1** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_conf1(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_CONF1_MASK) >> Self::ANA_CONF1_OFFSET
    }

    /// Write **ANA_CONF1** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_ana_conf1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ANA_CONF1_MASK) | ((value << Self::ANA_CONF1_OFFSET) & Self::ANA_CONF1_MASK)
        });
    }
}

/// ANA_CONF1 value type for batch read/modify/write operations
///
/// Use `anaconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `anaconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AnaConf1Val {
    bits: u32,
}

impl AnaConf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ANA_STATUS1** `[31:24]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_status1(&self) -> u32 {
        (self.bits & AnaConf1::ANA_STATUS1_MASK) >> AnaConf1::ANA_STATUS1_OFFSET
    }

    /// Read **ANA_CONF1** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_conf1(&self) -> u32 {
        (self.bits & AnaConf1::ANA_CONF1_MASK) >> AnaConf1::ANA_CONF1_OFFSET
    }

    /// Set **ANA_CONF1** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_ana_conf1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AnaConf1::ANA_CONF1_MASK)
                | ((value << AnaConf1::ANA_CONF1_OFFSET) & AnaConf1::ANA_CONF1_MASK),
        }
    }
}

impl Default for AnaConf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AnaConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AnaConf1Val {
        AnaConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AnaConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AnaConf1Val) -> AnaConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ANA_CONF2 Register
///
/// need des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **ANA_STATUS2** `[31:24]` (RO)
///
///   need des
///
/// - **ANA_CONF2** `[23:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct AnaConf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AnaConf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AnaConf2 {
    // ======== Field Constants ========

    /// **ANA_STATUS2**
    ///
    /// need des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const ANA_STATUS2_OFFSET: u32 = 24;
    pub const ANA_STATUS2_MASK: u32 = 0xff000000;

    /// **ANA_CONF2**
    ///
    /// need des
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const ANA_CONF2_OFFSET: u32 = 0;
    pub const ANA_CONF2_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **ANA_STATUS2** `[31:24]` (RO)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_status2(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_STATUS2_MASK) >> Self::ANA_STATUS2_OFFSET
    }

    /// **ANA_CONF2** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn ana_conf2(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_CONF2_MASK) >> Self::ANA_CONF2_OFFSET
    }

    /// Write **ANA_CONF2** `[23:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_ana_conf2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ANA_CONF2_MASK) | ((value << Self::ANA_CONF2_OFFSET) & Self::ANA_CONF2_MASK)
        });
    }
}

/// ANA_CONF2 value type for batch read/modify/write operations
///
/// Use `anaconf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `anaconf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AnaConf2Val {
    bits: u32,
}

impl AnaConf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ANA_STATUS2** `[31:24]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_status2(&self) -> u32 {
        (self.bits & AnaConf2::ANA_STATUS2_MASK) >> AnaConf2::ANA_STATUS2_OFFSET
    }

    /// Read **ANA_CONF2** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn ana_conf2(&self) -> u32 {
        (self.bits & AnaConf2::ANA_CONF2_MASK) >> AnaConf2::ANA_CONF2_OFFSET
    }

    /// Set **ANA_CONF2** `[23:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_ana_conf2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AnaConf2::ANA_CONF2_MASK)
                | ((value << AnaConf2::ANA_CONF2_OFFSET) & AnaConf2::ANA_CONF2_MASK),
        }
    }
}

impl Default for AnaConf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AnaConf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AnaConf2Val {
        AnaConf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AnaConf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AnaConf2Val) -> AnaConf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C0_CTRL1 Register
///
/// need des
///
/// Address offset: 0x0024
/// Reset value: 0x00000042
///
/// ## Fields
///
/// - **I2C0_SDA_SIDE_GUARD** `[10:6]` (RW)
///
///   need des
///
/// - **I2C0_SCL_PULSE_DUR** `[5:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c0Ctrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c0Ctrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c0Ctrl1 {
    // ======== Field Constants ========

    /// **I2C0_SDA_SIDE_GUARD**
    ///
    /// need des
    ///
    /// Bits `[10:6]`, mask `0x000007c0`
    pub const I2C0_SDA_SIDE_GUARD_OFFSET: u32 = 6;
    pub const I2C0_SDA_SIDE_GUARD_MASK: u32 = 0x000007c0;

    /// **I2C0_SCL_PULSE_DUR**
    ///
    /// need des
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const I2C0_SCL_PULSE_DUR_OFFSET: u32 = 0;
    pub const I2C0_SCL_PULSE_DUR_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **I2C0_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_sda_side_guard(&self) -> u32 {
        (self.value.read_bits() & Self::I2C0_SDA_SIDE_GUARD_MASK)
            >> Self::I2C0_SDA_SIDE_GUARD_OFFSET
    }

    /// Write **I2C0_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c0_sda_side_guard(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C0_SDA_SIDE_GUARD_MASK)
                | ((value << Self::I2C0_SDA_SIDE_GUARD_OFFSET) & Self::I2C0_SDA_SIDE_GUARD_MASK)
        });
    }

    /// **I2C0_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c0_scl_pulse_dur(&self) -> u32 {
        (self.value.read_bits() & Self::I2C0_SCL_PULSE_DUR_MASK) >> Self::I2C0_SCL_PULSE_DUR_OFFSET
    }

    /// Write **I2C0_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c0_scl_pulse_dur(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C0_SCL_PULSE_DUR_MASK)
                | ((value << Self::I2C0_SCL_PULSE_DUR_OFFSET) & Self::I2C0_SCL_PULSE_DUR_MASK)
        });
    }
}

/// I2C0_CTRL1 value type for batch read/modify/write operations
///
/// Use `i2c0ctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c0ctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c0Ctrl1Val {
    bits: u32,
}

impl I2c0Ctrl1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000042 }
    }

    /// Read **I2C0_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_sda_side_guard(&self) -> u32 {
        (self.bits & I2c0Ctrl1::I2C0_SDA_SIDE_GUARD_MASK) >> I2c0Ctrl1::I2C0_SDA_SIDE_GUARD_OFFSET
    }

    /// Read **I2C0_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c0_scl_pulse_dur(&self) -> u32 {
        (self.bits & I2c0Ctrl1::I2C0_SCL_PULSE_DUR_MASK) >> I2c0Ctrl1::I2C0_SCL_PULSE_DUR_OFFSET
    }

    /// Set **I2C0_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c0_sda_side_guard(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c0Ctrl1::I2C0_SDA_SIDE_GUARD_MASK)
                | ((value << I2c0Ctrl1::I2C0_SDA_SIDE_GUARD_OFFSET)
                    & I2c0Ctrl1::I2C0_SDA_SIDE_GUARD_MASK),
        }
    }

    /// Set **I2C0_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c0_scl_pulse_dur(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c0Ctrl1::I2C0_SCL_PULSE_DUR_MASK)
                | ((value << I2c0Ctrl1::I2C0_SCL_PULSE_DUR_OFFSET)
                    & I2c0Ctrl1::I2C0_SCL_PULSE_DUR_MASK),
        }
    }
}

impl Default for I2c0Ctrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl I2c0Ctrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c0Ctrl1Val {
        I2c0Ctrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c0Ctrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c0Ctrl1Val) -> I2c0Ctrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C1_CTRL1 Register
///
/// need des
///
/// Address offset: 0x0028
/// Reset value: 0x00000042
///
/// ## Fields
///
/// - **I2C1_SDA_SIDE_GUARD** `[10:6]` (RW)
///
///   need des
///
/// - **I2C1_SCL_PULSE_DUR** `[5:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct I2c1Ctrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c1Ctrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c1Ctrl1 {
    // ======== Field Constants ========

    /// **I2C1_SDA_SIDE_GUARD**
    ///
    /// need des
    ///
    /// Bits `[10:6]`, mask `0x000007c0`
    pub const I2C1_SDA_SIDE_GUARD_OFFSET: u32 = 6;
    pub const I2C1_SDA_SIDE_GUARD_MASK: u32 = 0x000007c0;

    /// **I2C1_SCL_PULSE_DUR**
    ///
    /// need des
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const I2C1_SCL_PULSE_DUR_OFFSET: u32 = 0;
    pub const I2C1_SCL_PULSE_DUR_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **I2C1_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_sda_side_guard(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1_SDA_SIDE_GUARD_MASK)
            >> Self::I2C1_SDA_SIDE_GUARD_OFFSET
    }

    /// Write **I2C1_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c1_sda_side_guard(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C1_SDA_SIDE_GUARD_MASK)
                | ((value << Self::I2C1_SDA_SIDE_GUARD_OFFSET) & Self::I2C1_SDA_SIDE_GUARD_MASK)
        });
    }

    /// **I2C1_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c1_scl_pulse_dur(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1_SCL_PULSE_DUR_MASK) >> Self::I2C1_SCL_PULSE_DUR_OFFSET
    }

    /// Write **I2C1_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c1_scl_pulse_dur(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C1_SCL_PULSE_DUR_MASK)
                | ((value << Self::I2C1_SCL_PULSE_DUR_OFFSET) & Self::I2C1_SCL_PULSE_DUR_MASK)
        });
    }
}

/// I2C1_CTRL1 value type for batch read/modify/write operations
///
/// Use `i2c1ctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c1ctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c1Ctrl1Val {
    bits: u32,
}

impl I2c1Ctrl1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000042 }
    }

    /// Read **I2C1_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_sda_side_guard(&self) -> u32 {
        (self.bits & I2c1Ctrl1::I2C1_SDA_SIDE_GUARD_MASK) >> I2c1Ctrl1::I2C1_SDA_SIDE_GUARD_OFFSET
    }

    /// Read **I2C1_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c1_scl_pulse_dur(&self) -> u32 {
        (self.bits & I2c1Ctrl1::I2C1_SCL_PULSE_DUR_MASK) >> I2c1Ctrl1::I2C1_SCL_PULSE_DUR_OFFSET
    }

    /// Set **I2C1_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c1_sda_side_guard(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c1Ctrl1::I2C1_SDA_SIDE_GUARD_MASK)
                | ((value << I2c1Ctrl1::I2C1_SDA_SIDE_GUARD_OFFSET)
                    & I2c1Ctrl1::I2C1_SDA_SIDE_GUARD_MASK),
        }
    }

    /// Set **I2C1_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c1_scl_pulse_dur(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c1Ctrl1::I2C1_SCL_PULSE_DUR_MASK)
                | ((value << I2c1Ctrl1::I2C1_SCL_PULSE_DUR_OFFSET)
                    & I2c1Ctrl1::I2C1_SCL_PULSE_DUR_MASK),
        }
    }
}

impl Default for I2c1Ctrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl I2c1Ctrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c1Ctrl1Val {
        I2c1Ctrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c1Ctrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c1Ctrl1Val) -> I2c1Ctrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HW_I2C_CTRL Register
///
/// need des
///
/// Address offset: 0x002c
/// Reset value: 0x00000042
///
/// ## Fields
///
/// - **ARBITER_DIS** `[11]` (RW)
///
///   need des
///
/// - **HW_I2C_SDA_SIDE_GUARD** `[10:6]` (RW)
///
///   need des
///
/// - **HW_I2C_SCL_PULSE_DUR** `[5:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct HwI2cCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HwI2cCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HwI2cCtrl {
    // ======== Field Constants ========

    /// **ARBITER_DIS**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ARBITER_DIS_OFFSET: u32 = 11;
    pub const ARBITER_DIS_MASK: u32 = 0x00000800;
    pub const ARBITER_DIS: u32 = 1 << 11;

    /// **HW_I2C_SDA_SIDE_GUARD**
    ///
    /// need des
    ///
    /// Bits `[10:6]`, mask `0x000007c0`
    pub const HW_I2C_SDA_SIDE_GUARD_OFFSET: u32 = 6;
    pub const HW_I2C_SDA_SIDE_GUARD_MASK: u32 = 0x000007c0;

    /// **HW_I2C_SCL_PULSE_DUR**
    ///
    /// need des
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const HW_I2C_SCL_PULSE_DUR_OFFSET: u32 = 0;
    pub const HW_I2C_SCL_PULSE_DUR_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ARBITER_DIS** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn arbiter_dis(&self) -> bool {
        (self.value.read_bits() & Self::ARBITER_DIS_MASK) != 0
    }

    /// Set ARBITER_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arbiter_dis(&self) {
        self.value.modify_bits(|r| r | Self::ARBITER_DIS_MASK);
    }

    /// Clear ARBITER_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arbiter_dis(&self) {
        self.value.modify_bits(|r| r & !Self::ARBITER_DIS_MASK);
    }

    /// **HW_I2C_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn hw_i2c_sda_side_guard(&self) -> u32 {
        (self.value.read_bits() & Self::HW_I2C_SDA_SIDE_GUARD_MASK)
            >> Self::HW_I2C_SDA_SIDE_GUARD_OFFSET
    }

    /// Write **HW_I2C_SDA_SIDE_GUARD** `[10:6]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_hw_i2c_sda_side_guard(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HW_I2C_SDA_SIDE_GUARD_MASK)
                | ((value << Self::HW_I2C_SDA_SIDE_GUARD_OFFSET) & Self::HW_I2C_SDA_SIDE_GUARD_MASK)
        });
    }

    /// **HW_I2C_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn hw_i2c_scl_pulse_dur(&self) -> u32 {
        (self.value.read_bits() & Self::HW_I2C_SCL_PULSE_DUR_MASK)
            >> Self::HW_I2C_SCL_PULSE_DUR_OFFSET
    }

    /// Write **HW_I2C_SCL_PULSE_DUR** `[5:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_hw_i2c_scl_pulse_dur(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HW_I2C_SCL_PULSE_DUR_MASK)
                | ((value << Self::HW_I2C_SCL_PULSE_DUR_OFFSET) & Self::HW_I2C_SCL_PULSE_DUR_MASK)
        });
    }
}

/// HW_I2C_CTRL value type for batch read/modify/write operations
///
/// Use `hwi2cctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hwi2cctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HwI2cCtrlVal {
    bits: u32,
}

impl HwI2cCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000042 }
    }

    /// Read **ARBITER_DIS** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn arbiter_dis(&self) -> bool {
        (self.bits & HwI2cCtrl::ARBITER_DIS_MASK) != 0
    }

    /// Read **HW_I2C_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn hw_i2c_sda_side_guard(&self) -> u32 {
        (self.bits & HwI2cCtrl::HW_I2C_SDA_SIDE_GUARD_MASK)
            >> HwI2cCtrl::HW_I2C_SDA_SIDE_GUARD_OFFSET
    }

    /// Read **HW_I2C_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn hw_i2c_scl_pulse_dur(&self) -> u32 {
        (self.bits & HwI2cCtrl::HW_I2C_SCL_PULSE_DUR_MASK) >> HwI2cCtrl::HW_I2C_SCL_PULSE_DUR_OFFSET
    }

    /// Set **ARBITER_DIS** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_arbiter_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HwI2cCtrl::ARBITER_DIS_MASK
            } else {
                self.bits & !HwI2cCtrl::ARBITER_DIS_MASK
            },
        }
    }

    /// Set **HW_I2C_SDA_SIDE_GUARD** `[10:6]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_hw_i2c_sda_side_guard(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HwI2cCtrl::HW_I2C_SDA_SIDE_GUARD_MASK)
                | ((value << HwI2cCtrl::HW_I2C_SDA_SIDE_GUARD_OFFSET)
                    & HwI2cCtrl::HW_I2C_SDA_SIDE_GUARD_MASK),
        }
    }

    /// Set **HW_I2C_SCL_PULSE_DUR** `[5:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_hw_i2c_scl_pulse_dur(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HwI2cCtrl::HW_I2C_SCL_PULSE_DUR_MASK)
                | ((value << HwI2cCtrl::HW_I2C_SCL_PULSE_DUR_OFFSET)
                    & HwI2cCtrl::HW_I2C_SCL_PULSE_DUR_MASK),
        }
    }
}

impl Default for HwI2cCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HwI2cCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HwI2cCtrlVal {
        HwI2cCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HwI2cCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HwI2cCtrlVal) -> HwI2cCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// NOUSE Register
///
/// need des
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **I2C_MST_NOUSE** `[31:0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct Nouse {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Nouse {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Nouse {
    // ======== Field Constants ========

    /// **I2C_MST_NOUSE**
    ///
    /// need des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const I2C_MST_NOUSE_OFFSET: u32 = 0;
    pub const I2C_MST_NOUSE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **I2C_MST_NOUSE** `[31:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst_nouse(&self) -> u32 {
        (self.value.read_bits() & Self::I2C_MST_NOUSE_MASK) >> Self::I2C_MST_NOUSE_OFFSET
    }

    /// Write **I2C_MST_NOUSE** `[31:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn write_i2c_mst_nouse(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C_MST_NOUSE_MASK)
                | ((value << Self::I2C_MST_NOUSE_OFFSET) & Self::I2C_MST_NOUSE_MASK)
        });
    }
}

/// NOUSE value type for batch read/modify/write operations
///
/// Use `nouse.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `nouse.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NouseVal {
    bits: u32,
}

impl NouseVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **I2C_MST_NOUSE** `[31:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst_nouse(&self) -> u32 {
        (self.bits & Nouse::I2C_MST_NOUSE_MASK) >> Nouse::I2C_MST_NOUSE_OFFSET
    }

    /// Set **I2C_MST_NOUSE** `[31:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c_mst_nouse(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Nouse::I2C_MST_NOUSE_MASK)
                | ((value << Nouse::I2C_MST_NOUSE_OFFSET) & Nouse::I2C_MST_NOUSE_MASK),
        }
    }
}

impl Default for NouseVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Nouse {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NouseVal {
        NouseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NouseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NouseVal) -> NouseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK160M Register
///
/// need des
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **CLK_I2C_MST_SEL_160M** `[0]` (RW)
///
///   need des
#[repr(transparent)]
pub struct Clk160m {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clk160m {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clk160m {
    // ======== Field Constants ========

    /// **CLK_I2C_MST_SEL_160M**
    ///
    /// need des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_I2C_MST_SEL_160M_OFFSET: u32 = 0;
    pub const CLK_I2C_MST_SEL_160M_MASK: u32 = 0x00000001;
    pub const CLK_I2C_MST_SEL_160M: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_I2C_MST_SEL_160M** `[0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&self) -> bool {
        (self.value.read_bits() & Self::CLK_I2C_MST_SEL_160M_MASK) != 0
    }

    /// Set CLK_I2C_MST_SEL_160M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clk_i2c_mst_sel_160m(&self) {
        self.value
            .modify_bits(|r| r | Self::CLK_I2C_MST_SEL_160M_MASK);
    }

    /// Clear CLK_I2C_MST_SEL_160M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clk_i2c_mst_sel_160m(&self) {
        self.value
            .modify_bits(|r| r & !Self::CLK_I2C_MST_SEL_160M_MASK);
    }
}

/// CLK160M value type for batch read/modify/write operations
///
/// Use `clk160m.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clk160m.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Clk160mVal {
    bits: u32,
}

impl Clk160mVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_I2C_MST_SEL_160M** `[0:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn clk_i2c_mst_sel_160m(&self) -> bool {
        (self.bits & Clk160m::CLK_I2C_MST_SEL_160M_MASK) != 0
    }

    /// Set **CLK_I2C_MST_SEL_160M** `[0:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_clk_i2c_mst_sel_160m(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk160m::CLK_I2C_MST_SEL_160M_MASK
            } else {
                self.bits & !Clk160m::CLK_I2C_MST_SEL_160M_MASK
            },
        }
    }
}

impl Default for Clk160mVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clk160m {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Clk160mVal {
        Clk160mVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Clk160mVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Clk160mVal) -> Clk160mVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// need des
///
/// Address offset: 0x0038
/// Reset value: 0x02201300
///
/// ## Fields
///
/// - **I2C_MST_CLK_EN** `[28]` (RW)
///
///   need des
///
/// - **DATE** `[27:0]` (RW)
///
///   need des
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

    /// **I2C_MST_CLK_EN**
    ///
    /// need des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const I2C_MST_CLK_EN_OFFSET: u32 = 28;
    pub const I2C_MST_CLK_EN_MASK: u32 = 0x10000000;
    pub const I2C_MST_CLK_EN: u32 = 1 << 28;

    /// **DATE**
    ///
    /// need des
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **I2C_MST_CLK_EN** `[28]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn i2c_mst_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::I2C_MST_CLK_EN_MASK) != 0
    }

    /// Set I2C_MST_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_i2c_mst_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::I2C_MST_CLK_EN_MASK);
    }

    /// Clear I2C_MST_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_i2c_mst_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::I2C_MST_CLK_EN_MASK);
    }

    /// **DATE** `[27:0]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// need des
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
        Self { bits: 0x02201300 }
    }

    /// Read **I2C_MST_CLK_EN** `[28:28]`
    ///
    /// need des
    #[inline(always)]
    pub const fn i2c_mst_clk_en(&self) -> bool {
        (self.bits & Date::I2C_MST_CLK_EN_MASK) != 0
    }

    /// Read **DATE** `[27:0]`
    ///
    /// need des
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **I2C_MST_CLK_EN** `[28:28]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_i2c_mst_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Date::I2C_MST_CLK_EN_MASK
            } else {
                self.bits & !Date::I2C_MST_CLK_EN_MASK
            },
        }
    }

    /// Set **DATE** `[27:0]`
    ///
    /// need des
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
    /// need des
    pub i2c0_ctrl: I2c0Ctrl,
    /// 0x0004
    /// need des
    pub i2c1_ctrl: I2c1Ctrl,
    /// 0x0008
    /// need des
    pub i2c0_conf: I2c0Conf,
    /// 0x000c
    /// need des
    pub i2c1_conf: I2c1Conf,
    /// 0x0010
    /// need des
    pub i2c_burst_conf: I2cBurstConf,
    /// 0x0014
    /// need des
    pub i2c_burst_status: I2cBurstStatus,
    /// 0x0018
    /// need des
    pub ana_conf0: AnaConf0,
    /// 0x001c
    /// need des
    pub ana_conf1: AnaConf1,
    /// 0x0020
    /// need des
    pub ana_conf2: AnaConf2,
    /// 0x0024
    /// need des
    pub i2c0_ctrl1: I2c0Ctrl1,
    /// 0x0028
    /// need des
    pub i2c1_ctrl1: I2c1Ctrl1,
    /// 0x002c
    /// need des
    pub hw_i2c_ctrl: HwI2cCtrl,
    /// 0x0030
    /// need des
    pub nouse: Nouse,
    /// 0x0034
    /// need des
    pub clk160m: Clk160m,
    /// 0x0038
    /// need des
    pub date: Date,
}
