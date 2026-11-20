//! LEDC
//! LED Control PWM (Pulse Width Modulation)

#![allow(dead_code)]

pub const BASE: u32 = 0x500d3000;

/// CH0_CONF0 Register
///
/// Configuration register 0 for channel 0
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch0Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH0_CONF0 value type for batch read/modify/write operations
///
/// Use `ch0conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0Conf0Val {
    bits: u32,
}

impl Ch0Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch0Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch0Conf0::OVF_NUM_CH_MASK) >> Ch0Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch0Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch0Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch0Conf0::TIMER_SEL_CH_MASK) >> Ch0Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch0Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch0Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch0Conf0::OVF_NUM_CH_OFFSET) & Ch0Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch0Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch0Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch0Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch0Conf0::TIMER_SEL_CH_OFFSET) & Ch0Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch0Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0Conf0Val {
        Ch0Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0Conf0Val) -> Ch0Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_HPOINT Register
///
/// High point register for channel 0
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch0Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH0_HPOINT value type for batch read/modify/write operations
///
/// Use `ch0hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0HpointVal {
    bits: u32,
}

impl Ch0HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch0Hpoint::HPOINT_CH_MASK) >> Ch0Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Hpoint::HPOINT_CH_MASK)
                | ((value << Ch0Hpoint::HPOINT_CH_OFFSET) & Ch0Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch0HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0HpointVal {
        Ch0HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0HpointVal) -> Ch0HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_DUTY Register
///
/// Initial duty cycle register for channel 0
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch0Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH0_DUTY value type for batch read/modify/write operations
///
/// Use `ch0duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0DutyVal {
    bits: u32,
}

impl Ch0DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch0Duty::DUTY_CH_MASK) >> Ch0Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Duty::DUTY_CH_MASK)
                | ((value << Ch0Duty::DUTY_CH_OFFSET) & Ch0Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch0DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0DutyVal {
        Ch0DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0DutyVal) -> Ch0DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_CONF1 Register
///
/// Configuration register 1 for channel 0
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH0_CONF1 value type for batch read/modify/write operations
///
/// Use `ch0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0Conf1Val {
    bits: u32,
}

impl Ch0Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch0Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch0Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch0Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0Conf1Val {
        Ch0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0Conf1Val) -> Ch0Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH0_DUTY_R Register
///
/// Current duty cycle register for channel 0
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch0DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH0_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch0dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0DutyRVal {
    bits: u32,
}

impl Ch0DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch0DutyR::DUTY_CH_R_MASK) >> Ch0DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch0DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0DutyRVal {
        Ch0DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH1_CONF0 Register
///
/// Configuration register 0 for channel 1
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch1Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH1_CONF0 value type for batch read/modify/write operations
///
/// Use `ch1conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1Conf0Val {
    bits: u32,
}

impl Ch1Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch1Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch1Conf0::OVF_NUM_CH_MASK) >> Ch1Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch1Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch1Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch1Conf0::TIMER_SEL_CH_MASK) >> Ch1Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch1Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch1Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch1Conf0::OVF_NUM_CH_OFFSET) & Ch1Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch1Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch1Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch1Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch1Conf0::TIMER_SEL_CH_OFFSET) & Ch1Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch1Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1Conf0Val {
        Ch1Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1Conf0Val) -> Ch1Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_HPOINT Register
///
/// High point register for channel 1
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch1Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH1_HPOINT value type for batch read/modify/write operations
///
/// Use `ch1hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1HpointVal {
    bits: u32,
}

impl Ch1HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch1Hpoint::HPOINT_CH_MASK) >> Ch1Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Hpoint::HPOINT_CH_MASK)
                | ((value << Ch1Hpoint::HPOINT_CH_OFFSET) & Ch1Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch1HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1HpointVal {
        Ch1HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1HpointVal) -> Ch1HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_DUTY Register
///
/// Initial duty cycle register for channel 1
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch1Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH1_DUTY value type for batch read/modify/write operations
///
/// Use `ch1duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1DutyVal {
    bits: u32,
}

impl Ch1DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch1Duty::DUTY_CH_MASK) >> Ch1Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Duty::DUTY_CH_MASK)
                | ((value << Ch1Duty::DUTY_CH_OFFSET) & Ch1Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch1DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1DutyVal {
        Ch1DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1DutyVal) -> Ch1DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_CONF1 Register
///
/// Configuration register 1 for channel 1
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH1_CONF1 value type for batch read/modify/write operations
///
/// Use `ch1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1Conf1Val {
    bits: u32,
}

impl Ch1Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch1Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch1Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch1Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1Conf1Val {
        Ch1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1Conf1Val) -> Ch1Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_DUTY_R Register
///
/// Current duty cycle register for channel 1
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch1DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH1_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch1dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1DutyRVal {
    bits: u32,
}

impl Ch1DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch1DutyR::DUTY_CH_R_MASK) >> Ch1DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch1DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1DutyRVal {
        Ch1DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH2_CONF0 Register
///
/// Configuration register 0 for channel 2
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch2Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH2_CONF0 value type for batch read/modify/write operations
///
/// Use `ch2conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2Conf0Val {
    bits: u32,
}

impl Ch2Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch2Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch2Conf0::OVF_NUM_CH_MASK) >> Ch2Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch2Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch2Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch2Conf0::TIMER_SEL_CH_MASK) >> Ch2Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch2Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch2Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch2Conf0::OVF_NUM_CH_OFFSET) & Ch2Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch2Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch2Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch2Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch2Conf0::TIMER_SEL_CH_OFFSET) & Ch2Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch2Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2Conf0Val {
        Ch2Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2Conf0Val) -> Ch2Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_HPOINT Register
///
/// High point register for channel 2
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch2Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH2_HPOINT value type for batch read/modify/write operations
///
/// Use `ch2hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2HpointVal {
    bits: u32,
}

impl Ch2HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch2Hpoint::HPOINT_CH_MASK) >> Ch2Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Hpoint::HPOINT_CH_MASK)
                | ((value << Ch2Hpoint::HPOINT_CH_OFFSET) & Ch2Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch2HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2HpointVal {
        Ch2HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2HpointVal) -> Ch2HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_DUTY Register
///
/// Initial duty cycle register for channel 2
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch2Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH2_DUTY value type for batch read/modify/write operations
///
/// Use `ch2duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2DutyVal {
    bits: u32,
}

impl Ch2DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch2Duty::DUTY_CH_MASK) >> Ch2Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Duty::DUTY_CH_MASK)
                | ((value << Ch2Duty::DUTY_CH_OFFSET) & Ch2Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch2DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2DutyVal {
        Ch2DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2DutyVal) -> Ch2DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_CONF1 Register
///
/// Configuration register 1 for channel 2
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH2_CONF1 value type for batch read/modify/write operations
///
/// Use `ch2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2Conf1Val {
    bits: u32,
}

impl Ch2Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch2Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch2Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch2Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2Conf1Val {
        Ch2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2Conf1Val) -> Ch2Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_DUTY_R Register
///
/// Current duty cycle register for channel 2
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch2DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH2_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch2dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2DutyRVal {
    bits: u32,
}

impl Ch2DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch2DutyR::DUTY_CH_R_MASK) >> Ch2DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch2DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2DutyRVal {
        Ch2DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH3_CONF0 Register
///
/// Configuration register 0 for channel 3
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch3Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH3_CONF0 value type for batch read/modify/write operations
///
/// Use `ch3conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3Conf0Val {
    bits: u32,
}

impl Ch3Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch3Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch3Conf0::OVF_NUM_CH_MASK) >> Ch3Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch3Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch3Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch3Conf0::TIMER_SEL_CH_MASK) >> Ch3Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch3Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch3Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch3Conf0::OVF_NUM_CH_OFFSET) & Ch3Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch3Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch3Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch3Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch3Conf0::TIMER_SEL_CH_OFFSET) & Ch3Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch3Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3Conf0Val {
        Ch3Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3Conf0Val) -> Ch3Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_HPOINT Register
///
/// High point register for channel 3
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch3Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH3_HPOINT value type for batch read/modify/write operations
///
/// Use `ch3hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3HpointVal {
    bits: u32,
}

impl Ch3HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch3Hpoint::HPOINT_CH_MASK) >> Ch3Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Hpoint::HPOINT_CH_MASK)
                | ((value << Ch3Hpoint::HPOINT_CH_OFFSET) & Ch3Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch3HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3HpointVal {
        Ch3HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3HpointVal) -> Ch3HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_DUTY Register
///
/// Initial duty cycle register for channel 3
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch3Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH3_DUTY value type for batch read/modify/write operations
///
/// Use `ch3duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3DutyVal {
    bits: u32,
}

impl Ch3DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch3Duty::DUTY_CH_MASK) >> Ch3Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Duty::DUTY_CH_MASK)
                | ((value << Ch3Duty::DUTY_CH_OFFSET) & Ch3Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch3DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3DutyVal {
        Ch3DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3DutyVal) -> Ch3DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_CONF1 Register
///
/// Configuration register 1 for channel 3
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch3Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH3_CONF1 value type for batch read/modify/write operations
///
/// Use `ch3conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3Conf1Val {
    bits: u32,
}

impl Ch3Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch3Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch3Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch3Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3Conf1Val {
        Ch3Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3Conf1Val) -> Ch3Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_DUTY_R Register
///
/// Current duty cycle register for channel 3
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch3DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH3_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch3dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3DutyRVal {
    bits: u32,
}

impl Ch3DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch3DutyR::DUTY_CH_R_MASK) >> Ch3DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch3DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3DutyRVal {
        Ch3DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH4_CONF0 Register
///
/// Configuration register 0 for channel 4
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch4Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH4_CONF0 value type for batch read/modify/write operations
///
/// Use `ch4conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4Conf0Val {
    bits: u32,
}

impl Ch4Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch4Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch4Conf0::OVF_NUM_CH_MASK) >> Ch4Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch4Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch4Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch4Conf0::TIMER_SEL_CH_MASK) >> Ch4Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch4Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch4Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch4Conf0::OVF_NUM_CH_OFFSET) & Ch4Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch4Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch4Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch4Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch4Conf0::TIMER_SEL_CH_OFFSET) & Ch4Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch4Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4Conf0Val {
        Ch4Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4Conf0Val) -> Ch4Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4_HPOINT Register
///
/// High point register for channel 4
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch4Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH4_HPOINT value type for batch read/modify/write operations
///
/// Use `ch4hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4HpointVal {
    bits: u32,
}

impl Ch4HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch4Hpoint::HPOINT_CH_MASK) >> Ch4Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Hpoint::HPOINT_CH_MASK)
                | ((value << Ch4Hpoint::HPOINT_CH_OFFSET) & Ch4Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch4HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4HpointVal {
        Ch4HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4HpointVal) -> Ch4HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4_DUTY Register
///
/// Initial duty cycle register for channel 4
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch4Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH4_DUTY value type for batch read/modify/write operations
///
/// Use `ch4duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4DutyVal {
    bits: u32,
}

impl Ch4DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch4Duty::DUTY_CH_MASK) >> Ch4Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Duty::DUTY_CH_MASK)
                | ((value << Ch4Duty::DUTY_CH_OFFSET) & Ch4Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch4DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4DutyVal {
        Ch4DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4DutyVal) -> Ch4DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4_CONF1 Register
///
/// Configuration register 1 for channel 4
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch4Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH4_CONF1 value type for batch read/modify/write operations
///
/// Use `ch4conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4Conf1Val {
    bits: u32,
}

impl Ch4Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch4Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch4Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch4Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4Conf1Val {
        Ch4Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4Conf1Val) -> Ch4Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4_DUTY_R Register
///
/// Current duty cycle register for channel 4
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch4DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH4_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch4dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4DutyRVal {
    bits: u32,
}

impl Ch4DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch4DutyR::DUTY_CH_R_MASK) >> Ch4DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch4DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4DutyRVal {
        Ch4DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH5_CONF0 Register
///
/// Configuration register 0 for channel 5
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch5Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH5_CONF0 value type for batch read/modify/write operations
///
/// Use `ch5conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5Conf0Val {
    bits: u32,
}

impl Ch5Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch5Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch5Conf0::OVF_NUM_CH_MASK) >> Ch5Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch5Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch5Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch5Conf0::TIMER_SEL_CH_MASK) >> Ch5Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch5Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch5Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch5Conf0::OVF_NUM_CH_OFFSET) & Ch5Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch5Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch5Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch5Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch5Conf0::TIMER_SEL_CH_OFFSET) & Ch5Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch5Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5Conf0Val {
        Ch5Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5Conf0Val) -> Ch5Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH5_HPOINT Register
///
/// High point register for channel 5
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch5Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH5_HPOINT value type for batch read/modify/write operations
///
/// Use `ch5hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5HpointVal {
    bits: u32,
}

impl Ch5HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch5Hpoint::HPOINT_CH_MASK) >> Ch5Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Hpoint::HPOINT_CH_MASK)
                | ((value << Ch5Hpoint::HPOINT_CH_OFFSET) & Ch5Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch5HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5HpointVal {
        Ch5HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5HpointVal) -> Ch5HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH5_DUTY Register
///
/// Initial duty cycle register for channel 5
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch5Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH5_DUTY value type for batch read/modify/write operations
///
/// Use `ch5duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5DutyVal {
    bits: u32,
}

impl Ch5DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch5Duty::DUTY_CH_MASK) >> Ch5Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Duty::DUTY_CH_MASK)
                | ((value << Ch5Duty::DUTY_CH_OFFSET) & Ch5Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch5DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5DutyVal {
        Ch5DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5DutyVal) -> Ch5DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH5_CONF1 Register
///
/// Configuration register 1 for channel 5
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch5Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH5_CONF1 value type for batch read/modify/write operations
///
/// Use `ch5conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5Conf1Val {
    bits: u32,
}

impl Ch5Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch5Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch5Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch5Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5Conf1Val {
        Ch5Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5Conf1Val) -> Ch5Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH5_DUTY_R Register
///
/// Current duty cycle register for channel 5
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch5DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH5_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch5dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5DutyRVal {
    bits: u32,
}

impl Ch5DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch5DutyR::DUTY_CH_R_MASK) >> Ch5DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch5DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5DutyRVal {
        Ch5DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH6_CONF0 Register
///
/// Configuration register 0 for channel 6
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch6Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH6_CONF0 value type for batch read/modify/write operations
///
/// Use `ch6conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6Conf0Val {
    bits: u32,
}

impl Ch6Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch6Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch6Conf0::OVF_NUM_CH_MASK) >> Ch6Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch6Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch6Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch6Conf0::TIMER_SEL_CH_MASK) >> Ch6Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch6Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch6Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch6Conf0::OVF_NUM_CH_OFFSET) & Ch6Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch6Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch6Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch6Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch6Conf0::TIMER_SEL_CH_OFFSET) & Ch6Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch6Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6Conf0Val {
        Ch6Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6Conf0Val) -> Ch6Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH6_HPOINT Register
///
/// High point register for channel 6
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch6Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH6_HPOINT value type for batch read/modify/write operations
///
/// Use `ch6hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6HpointVal {
    bits: u32,
}

impl Ch6HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch6Hpoint::HPOINT_CH_MASK) >> Ch6Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Hpoint::HPOINT_CH_MASK)
                | ((value << Ch6Hpoint::HPOINT_CH_OFFSET) & Ch6Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch6HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6HpointVal {
        Ch6HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6HpointVal) -> Ch6HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH6_DUTY Register
///
/// Initial duty cycle register for channel 6
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch6Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH6_DUTY value type for batch read/modify/write operations
///
/// Use `ch6duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6DutyVal {
    bits: u32,
}

impl Ch6DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch6Duty::DUTY_CH_MASK) >> Ch6Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Duty::DUTY_CH_MASK)
                | ((value << Ch6Duty::DUTY_CH_OFFSET) & Ch6Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch6DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6DutyVal {
        Ch6DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6DutyVal) -> Ch6DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH6_CONF1 Register
///
/// Configuration register 1 for channel 6
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch6Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH6_CONF1 value type for batch read/modify/write operations
///
/// Use `ch6conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6Conf1Val {
    bits: u32,
}

impl Ch6Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch6Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch6Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch6Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6Conf1Val {
        Ch6Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6Conf1Val) -> Ch6Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH6_DUTY_R Register
///
/// Current duty cycle register for channel 6
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch6DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH6_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch6dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6DutyRVal {
    bits: u32,
}

impl Ch6DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch6DutyR::DUTY_CH_R_MASK) >> Ch6DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch6DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6DutyRVal {
        Ch6DutyRVal::from_bits(self.value.read_bits())
    }
}

/// CH7_CONF0 Register
///
/// Configuration register 0 for channel 7
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **OVF_CNT_RESET_CH** `[16]` (WO)
///
///   Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
///
/// - **OVF_CNT_EN_CH** `[15]` (RW)
///
///   Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
///
/// - **OVF_NUM_CH** `[14:5]` (RW)
///
///   Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
///
/// - **PARA_UP_CH** `[4]` (WO)
///
///   Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
///
/// - **IDLE_LV_CH** `[3]` (RW)
///
///   Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
///
/// - **SIG_OUT_EN_CH** `[2]` (RW)
///
///   Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
///
/// - **TIMER_SEL_CH** `[1:0]` (RW)
///
///   Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
#[repr(transparent)]
pub struct Ch7Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Conf0 {
    // ======== Field Constants ========

    /// **OVF_CNT_RESET_CH**
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_RESET_CH_OFFSET: u32 = 16;
    pub const OVF_CNT_RESET_CH_MASK: u32 = 0x00010000;
    pub const OVF_CNT_RESET_CH: u32 = 1 << 16;

    /// **OVF_CNT_EN_CH**
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_EN_CH_OFFSET: u32 = 15;
    pub const OVF_CNT_EN_CH_MASK: u32 = 0x00008000;
    pub const OVF_CNT_EN_CH: u32 = 1 << 15;

    /// **OVF_NUM_CH**
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    ///
    /// Bits `[14:5]`, mask `0x00007fe0`
    pub const OVF_NUM_CH_OFFSET: u32 = 5;
    pub const OVF_NUM_CH_MASK: u32 = 0x00007fe0;

    /// **PARA_UP_CH**
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PARA_UP_CH_OFFSET: u32 = 4;
    pub const PARA_UP_CH_MASK: u32 = 0x00000010;
    pub const PARA_UP_CH: u32 = 1 << 4;

    /// **IDLE_LV_CH**
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDLE_LV_CH_OFFSET: u32 = 3;
    pub const IDLE_LV_CH_MASK: u32 = 0x00000008;
    pub const IDLE_LV_CH: u32 = 1 << 3;

    /// **SIG_OUT_EN_CH**
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIG_OUT_EN_CH_OFFSET: u32 = 2;
    pub const SIG_OUT_EN_CH_MASK: u32 = 0x00000004;
    pub const SIG_OUT_EN_CH: u32 = 1 << 2;

    /// **TIMER_SEL_CH**
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const TIMER_SEL_CH_OFFSET: u32 = 0;
    pub const TIMER_SEL_CH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_RESET_CH** (WO)
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub fn trigger_ovf_cnt_reset_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_RESET_CH_MASK);
    }

    /// **OVF_CNT_EN_CH** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Set OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::OVF_CNT_EN_CH_MASK);
    }

    /// Clear OVF_CNT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::OVF_CNT_EN_CH_MASK);
    }

    /// **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> u32 {
        (self.value.read_bits() & Self::OVF_NUM_CH_MASK) >> Self::OVF_NUM_CH_OFFSET
    }

    /// Write **OVF_NUM_CH** `[14:5]` (RW)
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub fn write_ovf_num_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVF_NUM_CH_MASK)
                | ((value << Self::OVF_NUM_CH_OFFSET) & Self::OVF_NUM_CH_MASK)
        });
    }

    /// Trigger **PARA_UP_CH** (WO)
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_para_up_ch(&self) {
        self.value.modify_bits(|r| r | Self::PARA_UP_CH_MASK);
    }

    /// **IDLE_LV_CH** `[3]` (RW)
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_LV_CH_MASK) != 0
    }

    /// Set IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r | Self::IDLE_LV_CH_MASK);
    }

    /// Clear IDLE_LV_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idle_lv_ch(&self) {
        self.value.modify_bits(|r| r & !Self::IDLE_LV_CH_MASK);
    }

    /// **SIG_OUT_EN_CH** `[2]` (RW)
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> bool {
        (self.value.read_bits() & Self::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Set SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r | Self::SIG_OUT_EN_CH_MASK);
    }

    /// Clear SIG_OUT_EN_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sig_out_en_ch(&self) {
        self.value.modify_bits(|r| r & !Self::SIG_OUT_EN_CH_MASK);
    }

    /// **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_SEL_CH_MASK) >> Self::TIMER_SEL_CH_OFFSET
    }

    /// Write **TIMER_SEL_CH** `[1:0]` (RW)
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub fn write_timer_sel_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_SEL_CH_MASK)
                | ((value << Self::TIMER_SEL_CH_OFFSET) & Self::TIMER_SEL_CH_MASK)
        });
    }
}

/// CH7_CONF0 value type for batch read/modify/write operations
///
/// Use `ch7conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7Conf0Val {
    bits: u32,
}

impl Ch7Conf0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn ovf_cnt_en_ch(&self) -> bool {
        (self.bits & Ch7Conf0::OVF_CNT_EN_CH_MASK) != 0
    }

    /// Read **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn ovf_num_ch(&self) -> u32 {
        (self.bits & Ch7Conf0::OVF_NUM_CH_MASK) >> Ch7Conf0::OVF_NUM_CH_OFFSET
    }

    /// Read **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn idle_lv_ch(&self) -> bool {
        (self.bits & Ch7Conf0::IDLE_LV_CH_MASK) != 0
    }

    /// Read **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn sig_out_en_ch(&self) -> bool {
        (self.bits & Ch7Conf0::SIG_OUT_EN_CH_MASK) != 0
    }

    /// Read **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn timer_sel_ch(&self) -> u32 {
        (self.bits & Ch7Conf0::TIMER_SEL_CH_MASK) >> Ch7Conf0::TIMER_SEL_CH_OFFSET
    }

    /// Set **OVF_CNT_RESET_CH** `[16:16]`
    ///
    /// Configures whether or not to reset the ovf_cnt of channel %s.\\0: Invalid. No effect\\1: Reset the ovf_cnt
    #[inline(always)]
    pub const fn with_ovf_cnt_reset_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf0::OVF_CNT_RESET_CH_MASK
            } else {
                self.bits & !Ch7Conf0::OVF_CNT_RESET_CH_MASK
            },
        }
    }

    /// Set **OVF_CNT_EN_CH** `[15:15]`
    ///
    /// Configures whether or not to enable the ovf_cnt of channel %s.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_ovf_cnt_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf0::OVF_CNT_EN_CH_MASK
            } else {
                self.bits & !Ch7Conf0::OVF_CNT_EN_CH_MASK
            },
        }
    }

    /// Set **OVF_NUM_CH** `[14:5]`
    ///
    /// Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times.
    #[inline(always)]
    pub const fn with_ovf_num_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Conf0::OVF_NUM_CH_MASK)
                | ((value << Ch7Conf0::OVF_NUM_CH_OFFSET) & Ch7Conf0::OVF_NUM_CH_MASK),
        }
    }

    /// Set **PARA_UP_CH** `[4:4]`
    ///
    /// Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_DUTY_NUM_CH%s, LEDC_DUTY_CYCLE_CH%s, LEDC_DUTY_SCALE_CH%s, LEDC_DUTY_INC_CH%s, and LEDC_OVF_CNT_EN_CH%s fields for channel %s, and will be automatically cleared by hardware.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_para_up_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf0::PARA_UP_CH_MASK
            } else {
                self.bits & !Ch7Conf0::PARA_UP_CH_MASK
            },
        }
    }

    /// Set **IDLE_LV_CH** `[3:3]`
    ///
    /// Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\0: Output level is low\\1: Output level is high
    #[inline(always)]
    pub const fn with_idle_lv_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf0::IDLE_LV_CH_MASK
            } else {
                self.bits & !Ch7Conf0::IDLE_LV_CH_MASK
            },
        }
    }

    /// Set **SIG_OUT_EN_CH** `[2:2]`
    ///
    /// Configures whether or not to enable signal output on channel %s.\\0: Signal output disable\\1: Signal output enable
    #[inline(always)]
    pub const fn with_sig_out_en_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf0::SIG_OUT_EN_CH_MASK
            } else {
                self.bits & !Ch7Conf0::SIG_OUT_EN_CH_MASK
            },
        }
    }

    /// Set **TIMER_SEL_CH** `[1:0]`
    ///
    /// Configures which timer is channel %s selected.\\0: Select timer0\\1: Select timer1\\2: Select timer2\\3: Select timer3
    #[inline(always)]
    pub const fn with_timer_sel_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Conf0::TIMER_SEL_CH_MASK)
                | ((value << Ch7Conf0::TIMER_SEL_CH_OFFSET) & Ch7Conf0::TIMER_SEL_CH_MASK),
        }
    }
}

impl Default for Ch7Conf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7Conf0Val {
        Ch7Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7Conf0Val) -> Ch7Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH7_HPOINT Register
///
/// High point register for channel 7
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **HPOINT_CH** `[19:0]` (RW)
///
///   Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
#[repr(transparent)]
pub struct Ch7Hpoint {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Hpoint {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Hpoint {
    // ======== Field Constants ========

    /// **HPOINT_CH**
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const HPOINT_CH_OFFSET: u32 = 0;
    pub const HPOINT_CH_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint_ch(&self) -> u32 {
        (self.value.read_bits() & Self::HPOINT_CH_MASK) >> Self::HPOINT_CH_OFFSET
    }

    /// Write **HPOINT_CH** `[19:0]` (RW)
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn write_hpoint_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPOINT_CH_MASK) | ((value << Self::HPOINT_CH_OFFSET) & Self::HPOINT_CH_MASK)
        });
    }
}

/// CH7_HPOINT value type for batch read/modify/write operations
///
/// Use `ch7hpoint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7hpoint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7HpointVal {
    bits: u32,
}

impl Ch7HpointVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn hpoint_ch(&self) -> u32 {
        (self.bits & Ch7Hpoint::HPOINT_CH_MASK) >> Ch7Hpoint::HPOINT_CH_OFFSET
    }

    /// Set **HPOINT_CH** `[19:0]`
    ///
    /// Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub const fn with_hpoint_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Hpoint::HPOINT_CH_MASK)
                | ((value << Ch7Hpoint::HPOINT_CH_OFFSET) & Ch7Hpoint::HPOINT_CH_MASK),
        }
    }
}

impl Default for Ch7HpointVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7Hpoint {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7HpointVal {
        Ch7HpointVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7HpointVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7HpointVal) -> Ch7HpointVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH7_DUTY Register
///
/// Initial duty cycle register for channel 7
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **DUTY_CH** `[24:0]` (RW)
///
///   Configures the duty of signal output on channel %s.
#[repr(transparent)]
pub struct Ch7Duty {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Duty {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Duty {
    // ======== Field Constants ========

    /// **DUTY_CH**
    ///
    /// Configures the duty of signal output on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_OFFSET: u32 = 0;
    pub const DUTY_CH_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn duty_ch(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_MASK) >> Self::DUTY_CH_OFFSET
    }

    /// Write **DUTY_CH** `[24:0]` (RW)
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub fn write_duty_ch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DUTY_CH_MASK) | ((value << Self::DUTY_CH_OFFSET) & Self::DUTY_CH_MASK)
        });
    }
}

/// CH7_DUTY value type for batch read/modify/write operations
///
/// Use `ch7duty.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7duty.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7DutyVal {
    bits: u32,
}

impl Ch7DutyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn duty_ch(&self) -> u32 {
        (self.bits & Ch7Duty::DUTY_CH_MASK) >> Ch7Duty::DUTY_CH_OFFSET
    }

    /// Set **DUTY_CH** `[24:0]`
    ///
    /// Configures the duty of signal output on channel %s.
    #[inline(always)]
    pub const fn with_duty_ch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Duty::DUTY_CH_MASK)
                | ((value << Ch7Duty::DUTY_CH_OFFSET) & Ch7Duty::DUTY_CH_MASK),
        }
    }
}

impl Default for Ch7DutyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7Duty {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7DutyVal {
        Ch7DutyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7DutyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7DutyVal) -> Ch7DutyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH7_CONF1 Register
///
/// Configuration register 1 for channel 7
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **DUTY_START_CH** `[31]` (RW)
///
///   Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
#[repr(transparent)]
pub struct Ch7Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Conf1 {
    // ======== Field Constants ========

    /// **DUTY_START_CH**
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DUTY_START_CH_OFFSET: u32 = 31;
    pub const DUTY_START_CH_MASK: u32 = 0x80000000;
    pub const DUTY_START_CH: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **DUTY_START_CH** `[31]` (RW)
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub fn duty_start_ch(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_START_CH_MASK) != 0
    }

    /// Set DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_start_ch(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_START_CH_MASK);
    }

    /// Clear DUTY_START_CH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_start_ch(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_START_CH_MASK);
    }
}

/// CH7_CONF1 value type for batch read/modify/write operations
///
/// Use `ch7conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7Conf1Val {
    bits: u32,
}

impl Ch7Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn duty_start_ch(&self) -> bool {
        (self.bits & Ch7Conf1::DUTY_START_CH_MASK) != 0
    }

    /// Set **DUTY_START_CH** `[31:31]`
    ///
    /// Configures whether the duty cycle fading configurations take effect.\\0: Not take effect\\1: Take effect
    #[inline(always)]
    pub const fn with_duty_start_ch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Conf1::DUTY_START_CH_MASK
            } else {
                self.bits & !Ch7Conf1::DUTY_START_CH_MASK
            },
        }
    }
}

impl Default for Ch7Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7Conf1Val {
        Ch7Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7Conf1Val) -> Ch7Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH7_DUTY_R Register
///
/// Current duty cycle register for channel 7
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **DUTY_CH_R** `[24:0]` (RO)
///
///   Represents the current duty of output signal on channel %s.
#[repr(transparent)]
pub struct Ch7DutyR {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7DutyR {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7DutyR {
    // ======== Field Constants ========

    /// **DUTY_CH_R**
    ///
    /// Represents the current duty of output signal on channel %s.
    ///
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const DUTY_CH_R_OFFSET: u32 = 0;
    pub const DUTY_CH_R_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **DUTY_CH_R** `[24:0]` (RO)
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub fn duty_ch_r(&self) -> u32 {
        (self.value.read_bits() & Self::DUTY_CH_R_MASK) >> Self::DUTY_CH_R_OFFSET
    }
}

/// CH7_DUTY_R value type for batch read/modify/write operations
///
/// Use `ch7dutyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7dutyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7DutyRVal {
    bits: u32,
}

impl Ch7DutyRVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DUTY_CH_R** `[24:0]`
    ///
    /// Represents the current duty of output signal on channel %s.
    #[inline(always)]
    pub const fn duty_ch_r(&self) -> u32 {
        (self.bits & Ch7DutyR::DUTY_CH_R_MASK) >> Ch7DutyR::DUTY_CH_R_OFFSET
    }
}

impl Default for Ch7DutyRVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7DutyR {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7DutyRVal {
        Ch7DutyRVal::from_bits(self.value.read_bits())
    }
}

/// TIMER0_CONF Register
///
/// Timer 0 configuration register
///
/// Address offset: 0x00a0
/// Reset value: 0x01000000
///
/// ## Fields
///
/// - **TIMER_PARA_UP** `[26]` (WO)
///
///   Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
///
/// - **TICK_SEL_TIMER** `[25]` (RW)
///
///   Configures which clock is timer %s selected. Unused.
///
/// - **TIMER_RST** `[24]` (RW)
///
///   Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
///
/// - **TIMER_PAUSE** `[23]` (RW)
///
///   Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
///
/// - **CLK_DIV_TIMER** `[22:5]` (RW)
///
///   Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
///
/// - **TIMER_DUTY_RES** `[4:0]` (RW)
///
///   Configures the range of the counter in timer %s.
#[repr(transparent)]
pub struct Timer0Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer0Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Conf {
    // ======== Field Constants ========

    /// **TIMER_PARA_UP**
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TIMER_PARA_UP_OFFSET: u32 = 26;
    pub const TIMER_PARA_UP_MASK: u32 = 0x04000000;
    pub const TIMER_PARA_UP: u32 = 1 << 26;

    /// **TICK_SEL_TIMER**
    ///
    /// Configures which clock is timer %s selected. Unused.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TICK_SEL_TIMER_OFFSET: u32 = 25;
    pub const TICK_SEL_TIMER_MASK: u32 = 0x02000000;
    pub const TICK_SEL_TIMER: u32 = 1 << 25;

    /// **TIMER_RST**
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER_RST_OFFSET: u32 = 24;
    pub const TIMER_RST_MASK: u32 = 0x01000000;
    pub const TIMER_RST: u32 = 1 << 24;

    /// **TIMER_PAUSE**
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER_PAUSE_OFFSET: u32 = 23;
    pub const TIMER_PAUSE_MASK: u32 = 0x00800000;
    pub const TIMER_PAUSE: u32 = 1 << 23;

    /// **CLK_DIV_TIMER**
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    ///
    /// Bits `[22:5]`, mask `0x007fffe0`
    pub const CLK_DIV_TIMER_OFFSET: u32 = 5;
    pub const CLK_DIV_TIMER_MASK: u32 = 0x007fffe0;

    /// **TIMER_DUTY_RES**
    ///
    /// Configures the range of the counter in timer %s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TIMER_DUTY_RES_OFFSET: u32 = 0;
    pub const TIMER_DUTY_RES_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **TIMER_PARA_UP** (WO)
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_timer_para_up(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PARA_UP_MASK);
    }

    /// **TICK_SEL_TIMER** `[25]` (RW)
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> bool {
        (self.value.read_bits() & Self::TICK_SEL_TIMER_MASK) != 0
    }

    /// Set TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r | Self::TICK_SEL_TIMER_MASK);
    }

    /// Clear TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r & !Self::TICK_SEL_TIMER_MASK);
    }

    /// **TIMER_RST** `[24]` (RW)
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub fn timer_rst(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_RST_MASK) != 0
    }

    /// Set TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_rst(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_RST_MASK);
    }

    /// Clear TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_RST_MASK);
    }

    /// **TIMER_PAUSE** `[23]` (RW)
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub fn timer_pause(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_PAUSE_MASK) != 0
    }

    /// Set TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_pause(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PAUSE_MASK);
    }

    /// Clear TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_pause(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_PAUSE_MASK);
    }

    /// **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn clk_div_timer(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIV_TIMER_MASK) >> Self::CLK_DIV_TIMER_OFFSET
    }

    /// Write **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn write_clk_div_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIV_TIMER_MASK)
                | ((value << Self::CLK_DIV_TIMER_OFFSET) & Self::CLK_DIV_TIMER_MASK)
        });
    }

    /// **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn timer_duty_res(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_DUTY_RES_MASK) >> Self::TIMER_DUTY_RES_OFFSET
    }

    /// Write **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn write_timer_duty_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_DUTY_RES_MASK)
                | ((value << Self::TIMER_DUTY_RES_OFFSET) & Self::TIMER_DUTY_RES_MASK)
        });
    }
}

/// TIMER0_CONF value type for batch read/modify/write operations
///
/// Use `timer0conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0ConfVal {
    bits: u32,
}

impl Timer0ConfVal {
    /// Create from raw bits
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
        Self { bits: 0x01000000 }
    }

    /// Read **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn tick_sel_timer(&self) -> bool {
        (self.bits & Timer0Conf::TICK_SEL_TIMER_MASK) != 0
    }

    /// Read **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn timer_rst(&self) -> bool {
        (self.bits & Timer0Conf::TIMER_RST_MASK) != 0
    }

    /// Read **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn timer_pause(&self) -> bool {
        (self.bits & Timer0Conf::TIMER_PAUSE_MASK) != 0
    }

    /// Read **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn clk_div_timer(&self) -> u32 {
        (self.bits & Timer0Conf::CLK_DIV_TIMER_MASK) >> Timer0Conf::CLK_DIV_TIMER_OFFSET
    }

    /// Read **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn timer_duty_res(&self) -> u32 {
        (self.bits & Timer0Conf::TIMER_DUTY_RES_MASK) >> Timer0Conf::TIMER_DUTY_RES_OFFSET
    }

    /// Set **TIMER_PARA_UP** `[26:26]`
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_timer_para_up(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0Conf::TIMER_PARA_UP_MASK
            } else {
                self.bits & !Timer0Conf::TIMER_PARA_UP_MASK
            },
        }
    }

    /// Set **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn with_tick_sel_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0Conf::TICK_SEL_TIMER_MASK
            } else {
                self.bits & !Timer0Conf::TICK_SEL_TIMER_MASK
            },
        }
    }

    /// Set **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn with_timer_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0Conf::TIMER_RST_MASK
            } else {
                self.bits & !Timer0Conf::TIMER_RST_MASK
            },
        }
    }

    /// Set **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn with_timer_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0Conf::TIMER_PAUSE_MASK
            } else {
                self.bits & !Timer0Conf::TIMER_PAUSE_MASK
            },
        }
    }

    /// Set **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn with_clk_div_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer0Conf::CLK_DIV_TIMER_MASK)
                | ((value << Timer0Conf::CLK_DIV_TIMER_OFFSET) & Timer0Conf::CLK_DIV_TIMER_MASK),
        }
    }

    /// Set **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn with_timer_duty_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer0Conf::TIMER_DUTY_RES_MASK)
                | ((value << Timer0Conf::TIMER_DUTY_RES_OFFSET) & Timer0Conf::TIMER_DUTY_RES_MASK),
        }
    }
}

impl Default for Timer0ConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer0Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0ConfVal {
        Timer0ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer0ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer0ConfVal) -> Timer0ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0_VALUE Register
///
/// Timer 0 current counter value register
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **TIMER_CNT** `[19:0]` (RO)
///
///   Represents the current counter value of timer %s.
#[repr(transparent)]
pub struct Timer0Value {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer0Value {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Value {
    // ======== Field Constants ========

    /// **TIMER_CNT**
    ///
    /// Represents the current counter value of timer %s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_OFFSET: u32 = 0;
    pub const TIMER_CNT_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT** `[19:0]` (RO)
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub fn timer_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_MASK) >> Self::TIMER_CNT_OFFSET
    }
}

/// TIMER0_VALUE value type for batch read/modify/write operations
///
/// Use `timer0value.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0value.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0ValueVal {
    bits: u32,
}

impl Timer0ValueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT** `[19:0]`
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub const fn timer_cnt(&self) -> u32 {
        (self.bits & Timer0Value::TIMER_CNT_MASK) >> Timer0Value::TIMER_CNT_OFFSET
    }
}

impl Default for Timer0ValueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer0Value {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0ValueVal {
        Timer0ValueVal::from_bits(self.value.read_bits())
    }
}

/// TIMER1_CONF Register
///
/// Timer 1 configuration register
///
/// Address offset: 0x00a8
/// Reset value: 0x01000000
///
/// ## Fields
///
/// - **TIMER_PARA_UP** `[26]` (WO)
///
///   Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
///
/// - **TICK_SEL_TIMER** `[25]` (RW)
///
///   Configures which clock is timer %s selected. Unused.
///
/// - **TIMER_RST** `[24]` (RW)
///
///   Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
///
/// - **TIMER_PAUSE** `[23]` (RW)
///
///   Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
///
/// - **CLK_DIV_TIMER** `[22:5]` (RW)
///
///   Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
///
/// - **TIMER_DUTY_RES** `[4:0]` (RW)
///
///   Configures the range of the counter in timer %s.
#[repr(transparent)]
pub struct Timer1Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer1Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Conf {
    // ======== Field Constants ========

    /// **TIMER_PARA_UP**
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TIMER_PARA_UP_OFFSET: u32 = 26;
    pub const TIMER_PARA_UP_MASK: u32 = 0x04000000;
    pub const TIMER_PARA_UP: u32 = 1 << 26;

    /// **TICK_SEL_TIMER**
    ///
    /// Configures which clock is timer %s selected. Unused.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TICK_SEL_TIMER_OFFSET: u32 = 25;
    pub const TICK_SEL_TIMER_MASK: u32 = 0x02000000;
    pub const TICK_SEL_TIMER: u32 = 1 << 25;

    /// **TIMER_RST**
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER_RST_OFFSET: u32 = 24;
    pub const TIMER_RST_MASK: u32 = 0x01000000;
    pub const TIMER_RST: u32 = 1 << 24;

    /// **TIMER_PAUSE**
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER_PAUSE_OFFSET: u32 = 23;
    pub const TIMER_PAUSE_MASK: u32 = 0x00800000;
    pub const TIMER_PAUSE: u32 = 1 << 23;

    /// **CLK_DIV_TIMER**
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    ///
    /// Bits `[22:5]`, mask `0x007fffe0`
    pub const CLK_DIV_TIMER_OFFSET: u32 = 5;
    pub const CLK_DIV_TIMER_MASK: u32 = 0x007fffe0;

    /// **TIMER_DUTY_RES**
    ///
    /// Configures the range of the counter in timer %s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TIMER_DUTY_RES_OFFSET: u32 = 0;
    pub const TIMER_DUTY_RES_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **TIMER_PARA_UP** (WO)
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_timer_para_up(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PARA_UP_MASK);
    }

    /// **TICK_SEL_TIMER** `[25]` (RW)
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> bool {
        (self.value.read_bits() & Self::TICK_SEL_TIMER_MASK) != 0
    }

    /// Set TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r | Self::TICK_SEL_TIMER_MASK);
    }

    /// Clear TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r & !Self::TICK_SEL_TIMER_MASK);
    }

    /// **TIMER_RST** `[24]` (RW)
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub fn timer_rst(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_RST_MASK) != 0
    }

    /// Set TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_rst(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_RST_MASK);
    }

    /// Clear TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_RST_MASK);
    }

    /// **TIMER_PAUSE** `[23]` (RW)
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub fn timer_pause(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_PAUSE_MASK) != 0
    }

    /// Set TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_pause(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PAUSE_MASK);
    }

    /// Clear TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_pause(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_PAUSE_MASK);
    }

    /// **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn clk_div_timer(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIV_TIMER_MASK) >> Self::CLK_DIV_TIMER_OFFSET
    }

    /// Write **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn write_clk_div_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIV_TIMER_MASK)
                | ((value << Self::CLK_DIV_TIMER_OFFSET) & Self::CLK_DIV_TIMER_MASK)
        });
    }

    /// **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn timer_duty_res(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_DUTY_RES_MASK) >> Self::TIMER_DUTY_RES_OFFSET
    }

    /// Write **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn write_timer_duty_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_DUTY_RES_MASK)
                | ((value << Self::TIMER_DUTY_RES_OFFSET) & Self::TIMER_DUTY_RES_MASK)
        });
    }
}

/// TIMER1_CONF value type for batch read/modify/write operations
///
/// Use `timer1conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1ConfVal {
    bits: u32,
}

impl Timer1ConfVal {
    /// Create from raw bits
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
        Self { bits: 0x01000000 }
    }

    /// Read **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn tick_sel_timer(&self) -> bool {
        (self.bits & Timer1Conf::TICK_SEL_TIMER_MASK) != 0
    }

    /// Read **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn timer_rst(&self) -> bool {
        (self.bits & Timer1Conf::TIMER_RST_MASK) != 0
    }

    /// Read **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn timer_pause(&self) -> bool {
        (self.bits & Timer1Conf::TIMER_PAUSE_MASK) != 0
    }

    /// Read **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn clk_div_timer(&self) -> u32 {
        (self.bits & Timer1Conf::CLK_DIV_TIMER_MASK) >> Timer1Conf::CLK_DIV_TIMER_OFFSET
    }

    /// Read **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn timer_duty_res(&self) -> u32 {
        (self.bits & Timer1Conf::TIMER_DUTY_RES_MASK) >> Timer1Conf::TIMER_DUTY_RES_OFFSET
    }

    /// Set **TIMER_PARA_UP** `[26:26]`
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_timer_para_up(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1Conf::TIMER_PARA_UP_MASK
            } else {
                self.bits & !Timer1Conf::TIMER_PARA_UP_MASK
            },
        }
    }

    /// Set **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn with_tick_sel_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1Conf::TICK_SEL_TIMER_MASK
            } else {
                self.bits & !Timer1Conf::TICK_SEL_TIMER_MASK
            },
        }
    }

    /// Set **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn with_timer_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1Conf::TIMER_RST_MASK
            } else {
                self.bits & !Timer1Conf::TIMER_RST_MASK
            },
        }
    }

    /// Set **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn with_timer_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1Conf::TIMER_PAUSE_MASK
            } else {
                self.bits & !Timer1Conf::TIMER_PAUSE_MASK
            },
        }
    }

    /// Set **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn with_clk_div_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer1Conf::CLK_DIV_TIMER_MASK)
                | ((value << Timer1Conf::CLK_DIV_TIMER_OFFSET) & Timer1Conf::CLK_DIV_TIMER_MASK),
        }
    }

    /// Set **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn with_timer_duty_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer1Conf::TIMER_DUTY_RES_MASK)
                | ((value << Timer1Conf::TIMER_DUTY_RES_OFFSET) & Timer1Conf::TIMER_DUTY_RES_MASK),
        }
    }
}

impl Default for Timer1ConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer1Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1ConfVal {
        Timer1ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer1ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer1ConfVal) -> Timer1ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER1_VALUE Register
///
/// Timer 1 current counter value register
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **TIMER_CNT** `[19:0]` (RO)
///
///   Represents the current counter value of timer %s.
#[repr(transparent)]
pub struct Timer1Value {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer1Value {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Value {
    // ======== Field Constants ========

    /// **TIMER_CNT**
    ///
    /// Represents the current counter value of timer %s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_OFFSET: u32 = 0;
    pub const TIMER_CNT_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT** `[19:0]` (RO)
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub fn timer_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_MASK) >> Self::TIMER_CNT_OFFSET
    }
}

/// TIMER1_VALUE value type for batch read/modify/write operations
///
/// Use `timer1value.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1value.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1ValueVal {
    bits: u32,
}

impl Timer1ValueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT** `[19:0]`
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub const fn timer_cnt(&self) -> u32 {
        (self.bits & Timer1Value::TIMER_CNT_MASK) >> Timer1Value::TIMER_CNT_OFFSET
    }
}

impl Default for Timer1ValueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer1Value {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1ValueVal {
        Timer1ValueVal::from_bits(self.value.read_bits())
    }
}

/// TIMER2_CONF Register
///
/// Timer 2 configuration register
///
/// Address offset: 0x00b0
/// Reset value: 0x01000000
///
/// ## Fields
///
/// - **TIMER_PARA_UP** `[26]` (WO)
///
///   Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
///
/// - **TICK_SEL_TIMER** `[25]` (RW)
///
///   Configures which clock is timer %s selected. Unused.
///
/// - **TIMER_RST** `[24]` (RW)
///
///   Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
///
/// - **TIMER_PAUSE** `[23]` (RW)
///
///   Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
///
/// - **CLK_DIV_TIMER** `[22:5]` (RW)
///
///   Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
///
/// - **TIMER_DUTY_RES** `[4:0]` (RW)
///
///   Configures the range of the counter in timer %s.
#[repr(transparent)]
pub struct Timer2Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer2Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer2Conf {
    // ======== Field Constants ========

    /// **TIMER_PARA_UP**
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TIMER_PARA_UP_OFFSET: u32 = 26;
    pub const TIMER_PARA_UP_MASK: u32 = 0x04000000;
    pub const TIMER_PARA_UP: u32 = 1 << 26;

    /// **TICK_SEL_TIMER**
    ///
    /// Configures which clock is timer %s selected. Unused.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TICK_SEL_TIMER_OFFSET: u32 = 25;
    pub const TICK_SEL_TIMER_MASK: u32 = 0x02000000;
    pub const TICK_SEL_TIMER: u32 = 1 << 25;

    /// **TIMER_RST**
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER_RST_OFFSET: u32 = 24;
    pub const TIMER_RST_MASK: u32 = 0x01000000;
    pub const TIMER_RST: u32 = 1 << 24;

    /// **TIMER_PAUSE**
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER_PAUSE_OFFSET: u32 = 23;
    pub const TIMER_PAUSE_MASK: u32 = 0x00800000;
    pub const TIMER_PAUSE: u32 = 1 << 23;

    /// **CLK_DIV_TIMER**
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    ///
    /// Bits `[22:5]`, mask `0x007fffe0`
    pub const CLK_DIV_TIMER_OFFSET: u32 = 5;
    pub const CLK_DIV_TIMER_MASK: u32 = 0x007fffe0;

    /// **TIMER_DUTY_RES**
    ///
    /// Configures the range of the counter in timer %s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TIMER_DUTY_RES_OFFSET: u32 = 0;
    pub const TIMER_DUTY_RES_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **TIMER_PARA_UP** (WO)
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_timer_para_up(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PARA_UP_MASK);
    }

    /// **TICK_SEL_TIMER** `[25]` (RW)
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> bool {
        (self.value.read_bits() & Self::TICK_SEL_TIMER_MASK) != 0
    }

    /// Set TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r | Self::TICK_SEL_TIMER_MASK);
    }

    /// Clear TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r & !Self::TICK_SEL_TIMER_MASK);
    }

    /// **TIMER_RST** `[24]` (RW)
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub fn timer_rst(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_RST_MASK) != 0
    }

    /// Set TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_rst(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_RST_MASK);
    }

    /// Clear TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_RST_MASK);
    }

    /// **TIMER_PAUSE** `[23]` (RW)
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub fn timer_pause(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_PAUSE_MASK) != 0
    }

    /// Set TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_pause(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PAUSE_MASK);
    }

    /// Clear TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_pause(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_PAUSE_MASK);
    }

    /// **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn clk_div_timer(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIV_TIMER_MASK) >> Self::CLK_DIV_TIMER_OFFSET
    }

    /// Write **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn write_clk_div_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIV_TIMER_MASK)
                | ((value << Self::CLK_DIV_TIMER_OFFSET) & Self::CLK_DIV_TIMER_MASK)
        });
    }

    /// **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn timer_duty_res(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_DUTY_RES_MASK) >> Self::TIMER_DUTY_RES_OFFSET
    }

    /// Write **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn write_timer_duty_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_DUTY_RES_MASK)
                | ((value << Self::TIMER_DUTY_RES_OFFSET) & Self::TIMER_DUTY_RES_MASK)
        });
    }
}

/// TIMER2_CONF value type for batch read/modify/write operations
///
/// Use `timer2conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer2conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer2ConfVal {
    bits: u32,
}

impl Timer2ConfVal {
    /// Create from raw bits
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
        Self { bits: 0x01000000 }
    }

    /// Read **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn tick_sel_timer(&self) -> bool {
        (self.bits & Timer2Conf::TICK_SEL_TIMER_MASK) != 0
    }

    /// Read **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn timer_rst(&self) -> bool {
        (self.bits & Timer2Conf::TIMER_RST_MASK) != 0
    }

    /// Read **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn timer_pause(&self) -> bool {
        (self.bits & Timer2Conf::TIMER_PAUSE_MASK) != 0
    }

    /// Read **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn clk_div_timer(&self) -> u32 {
        (self.bits & Timer2Conf::CLK_DIV_TIMER_MASK) >> Timer2Conf::CLK_DIV_TIMER_OFFSET
    }

    /// Read **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn timer_duty_res(&self) -> u32 {
        (self.bits & Timer2Conf::TIMER_DUTY_RES_MASK) >> Timer2Conf::TIMER_DUTY_RES_OFFSET
    }

    /// Set **TIMER_PARA_UP** `[26:26]`
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_timer_para_up(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer2Conf::TIMER_PARA_UP_MASK
            } else {
                self.bits & !Timer2Conf::TIMER_PARA_UP_MASK
            },
        }
    }

    /// Set **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn with_tick_sel_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer2Conf::TICK_SEL_TIMER_MASK
            } else {
                self.bits & !Timer2Conf::TICK_SEL_TIMER_MASK
            },
        }
    }

    /// Set **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn with_timer_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer2Conf::TIMER_RST_MASK
            } else {
                self.bits & !Timer2Conf::TIMER_RST_MASK
            },
        }
    }

    /// Set **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn with_timer_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer2Conf::TIMER_PAUSE_MASK
            } else {
                self.bits & !Timer2Conf::TIMER_PAUSE_MASK
            },
        }
    }

    /// Set **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn with_clk_div_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer2Conf::CLK_DIV_TIMER_MASK)
                | ((value << Timer2Conf::CLK_DIV_TIMER_OFFSET) & Timer2Conf::CLK_DIV_TIMER_MASK),
        }
    }

    /// Set **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn with_timer_duty_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer2Conf::TIMER_DUTY_RES_MASK)
                | ((value << Timer2Conf::TIMER_DUTY_RES_OFFSET) & Timer2Conf::TIMER_DUTY_RES_MASK),
        }
    }
}

impl Default for Timer2ConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer2Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer2ConfVal {
        Timer2ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer2ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer2ConfVal) -> Timer2ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER2_VALUE Register
///
/// Timer 2 current counter value register
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **TIMER_CNT** `[19:0]` (RO)
///
///   Represents the current counter value of timer %s.
#[repr(transparent)]
pub struct Timer2Value {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer2Value {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer2Value {
    // ======== Field Constants ========

    /// **TIMER_CNT**
    ///
    /// Represents the current counter value of timer %s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_OFFSET: u32 = 0;
    pub const TIMER_CNT_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT** `[19:0]` (RO)
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub fn timer_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_MASK) >> Self::TIMER_CNT_OFFSET
    }
}

/// TIMER2_VALUE value type for batch read/modify/write operations
///
/// Use `timer2value.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer2value.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer2ValueVal {
    bits: u32,
}

impl Timer2ValueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT** `[19:0]`
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub const fn timer_cnt(&self) -> u32 {
        (self.bits & Timer2Value::TIMER_CNT_MASK) >> Timer2Value::TIMER_CNT_OFFSET
    }
}

impl Default for Timer2ValueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer2Value {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer2ValueVal {
        Timer2ValueVal::from_bits(self.value.read_bits())
    }
}

/// TIMER3_CONF Register
///
/// Timer 3 configuration register
///
/// Address offset: 0x00b8
/// Reset value: 0x01000000
///
/// ## Fields
///
/// - **TIMER_PARA_UP** `[26]` (WO)
///
///   Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
///
/// - **TICK_SEL_TIMER** `[25]` (RW)
///
///   Configures which clock is timer %s selected. Unused.
///
/// - **TIMER_RST** `[24]` (RW)
///
///   Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
///
/// - **TIMER_PAUSE** `[23]` (RW)
///
///   Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
///
/// - **CLK_DIV_TIMER** `[22:5]` (RW)
///
///   Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
///
/// - **TIMER_DUTY_RES** `[4:0]` (RW)
///
///   Configures the range of the counter in timer %s.
#[repr(transparent)]
pub struct Timer3Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer3Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer3Conf {
    // ======== Field Constants ========

    /// **TIMER_PARA_UP**
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TIMER_PARA_UP_OFFSET: u32 = 26;
    pub const TIMER_PARA_UP_MASK: u32 = 0x04000000;
    pub const TIMER_PARA_UP: u32 = 1 << 26;

    /// **TICK_SEL_TIMER**
    ///
    /// Configures which clock is timer %s selected. Unused.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TICK_SEL_TIMER_OFFSET: u32 = 25;
    pub const TICK_SEL_TIMER_MASK: u32 = 0x02000000;
    pub const TICK_SEL_TIMER: u32 = 1 << 25;

    /// **TIMER_RST**
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER_RST_OFFSET: u32 = 24;
    pub const TIMER_RST_MASK: u32 = 0x01000000;
    pub const TIMER_RST: u32 = 1 << 24;

    /// **TIMER_PAUSE**
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER_PAUSE_OFFSET: u32 = 23;
    pub const TIMER_PAUSE_MASK: u32 = 0x00800000;
    pub const TIMER_PAUSE: u32 = 1 << 23;

    /// **CLK_DIV_TIMER**
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    ///
    /// Bits `[22:5]`, mask `0x007fffe0`
    pub const CLK_DIV_TIMER_OFFSET: u32 = 5;
    pub const CLK_DIV_TIMER_MASK: u32 = 0x007fffe0;

    /// **TIMER_DUTY_RES**
    ///
    /// Configures the range of the counter in timer %s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TIMER_DUTY_RES_OFFSET: u32 = 0;
    pub const TIMER_DUTY_RES_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **TIMER_PARA_UP** (WO)
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub fn trigger_timer_para_up(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PARA_UP_MASK);
    }

    /// **TICK_SEL_TIMER** `[25]` (RW)
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> bool {
        (self.value.read_bits() & Self::TICK_SEL_TIMER_MASK) != 0
    }

    /// Set TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r | Self::TICK_SEL_TIMER_MASK);
    }

    /// Clear TICK_SEL_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tick_sel_timer(&self) {
        self.value.modify_bits(|r| r & !Self::TICK_SEL_TIMER_MASK);
    }

    /// **TIMER_RST** `[24]` (RW)
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub fn timer_rst(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_RST_MASK) != 0
    }

    /// Set TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_rst(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_RST_MASK);
    }

    /// Clear TIMER_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_RST_MASK);
    }

    /// **TIMER_PAUSE** `[23]` (RW)
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub fn timer_pause(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_PAUSE_MASK) != 0
    }

    /// Set TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_pause(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_PAUSE_MASK);
    }

    /// Clear TIMER_PAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_pause(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_PAUSE_MASK);
    }

    /// **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn clk_div_timer(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIV_TIMER_MASK) >> Self::CLK_DIV_TIMER_OFFSET
    }

    /// Write **CLK_DIV_TIMER** `[22:5]` (RW)
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn write_clk_div_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIV_TIMER_MASK)
                | ((value << Self::CLK_DIV_TIMER_OFFSET) & Self::CLK_DIV_TIMER_MASK)
        });
    }

    /// **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn timer_duty_res(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_DUTY_RES_MASK) >> Self::TIMER_DUTY_RES_OFFSET
    }

    /// Write **TIMER_DUTY_RES** `[4:0]` (RW)
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn write_timer_duty_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_DUTY_RES_MASK)
                | ((value << Self::TIMER_DUTY_RES_OFFSET) & Self::TIMER_DUTY_RES_MASK)
        });
    }
}

/// TIMER3_CONF value type for batch read/modify/write operations
///
/// Use `timer3conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer3conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer3ConfVal {
    bits: u32,
}

impl Timer3ConfVal {
    /// Create from raw bits
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
        Self { bits: 0x01000000 }
    }

    /// Read **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn tick_sel_timer(&self) -> bool {
        (self.bits & Timer3Conf::TICK_SEL_TIMER_MASK) != 0
    }

    /// Read **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn timer_rst(&self) -> bool {
        (self.bits & Timer3Conf::TIMER_RST_MASK) != 0
    }

    /// Read **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn timer_pause(&self) -> bool {
        (self.bits & Timer3Conf::TIMER_PAUSE_MASK) != 0
    }

    /// Read **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn clk_div_timer(&self) -> u32 {
        (self.bits & Timer3Conf::CLK_DIV_TIMER_MASK) >> Timer3Conf::CLK_DIV_TIMER_OFFSET
    }

    /// Read **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn timer_duty_res(&self) -> u32 {
        (self.bits & Timer3Conf::TIMER_DUTY_RES_MASK) >> Timer3Conf::TIMER_DUTY_RES_OFFSET
    }

    /// Set **TIMER_PARA_UP** `[26:26]`
    ///
    /// Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    pub const fn with_timer_para_up(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer3Conf::TIMER_PARA_UP_MASK
            } else {
                self.bits & !Timer3Conf::TIMER_PARA_UP_MASK
            },
        }
    }

    /// Set **TICK_SEL_TIMER** `[25:25]`
    ///
    /// Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub const fn with_tick_sel_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer3Conf::TICK_SEL_TIMER_MASK
            } else {
                self.bits & !Timer3Conf::TICK_SEL_TIMER_MASK
            },
        }
    }

    /// Set **TIMER_RST** `[24:24]`
    ///
    /// Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub const fn with_timer_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer3Conf::TIMER_RST_MASK
            } else {
                self.bits & !Timer3Conf::TIMER_RST_MASK
            },
        }
    }

    /// Set **TIMER_PAUSE** `[23:23]`
    ///
    /// Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub const fn with_timer_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer3Conf::TIMER_PAUSE_MASK
            } else {
                self.bits & !Timer3Conf::TIMER_PAUSE_MASK
            },
        }
    }

    /// Set **CLK_DIV_TIMER** `[22:5]`
    ///
    /// Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub const fn with_clk_div_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer3Conf::CLK_DIV_TIMER_MASK)
                | ((value << Timer3Conf::CLK_DIV_TIMER_OFFSET) & Timer3Conf::CLK_DIV_TIMER_MASK),
        }
    }

    /// Set **TIMER_DUTY_RES** `[4:0]`
    ///
    /// Configures the range of the counter in timer %s.
    #[inline(always)]
    pub const fn with_timer_duty_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer3Conf::TIMER_DUTY_RES_MASK)
                | ((value << Timer3Conf::TIMER_DUTY_RES_OFFSET) & Timer3Conf::TIMER_DUTY_RES_MASK),
        }
    }
}

impl Default for Timer3ConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer3Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer3ConfVal {
        Timer3ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer3ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer3ConfVal) -> Timer3ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER3_VALUE Register
///
/// Timer 3 current counter value register
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **TIMER_CNT** `[19:0]` (RO)
///
///   Represents the current counter value of timer %s.
#[repr(transparent)]
pub struct Timer3Value {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer3Value {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer3Value {
    // ======== Field Constants ========

    /// **TIMER_CNT**
    ///
    /// Represents the current counter value of timer %s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_OFFSET: u32 = 0;
    pub const TIMER_CNT_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT** `[19:0]` (RO)
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub fn timer_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_MASK) >> Self::TIMER_CNT_OFFSET
    }
}

/// TIMER3_VALUE value type for batch read/modify/write operations
///
/// Use `timer3value.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer3value.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer3ValueVal {
    bits: u32,
}

impl Timer3ValueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT** `[19:0]`
    ///
    /// Represents the current counter value of timer %s.
    #[inline(always)]
    pub const fn timer_cnt(&self) -> u32 {
        (self.bits & Timer3Value::TIMER_CNT_MASK) >> Timer3Value::TIMER_CNT_OFFSET
    }
}

impl Default for Timer3ValueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer3Value {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer3ValueVal {
        Timer3ValueVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// Interrupt raw status register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **OVF_CNT_CH7_INT_RAW** `[19]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH7_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7.
///
/// - **OVF_CNT_CH6_INT_RAW** `[18]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH6_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6.
///
/// - **OVF_CNT_CH5_INT_RAW** `[17]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5.
///
/// - **OVF_CNT_CH4_INT_RAW** `[16]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4.
///
/// - **OVF_CNT_CH3_INT_RAW** `[15]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3.
///
/// - **OVF_CNT_CH2_INT_RAW** `[14]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2.
///
/// - **OVF_CNT_CH1_INT_RAW** `[13]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1.
///
/// - **OVF_CNT_CH0_INT_RAW** `[12]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0.
///
/// - **DUTY_CHNG_END_CH7_INT_RAW** `[11]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH6_INT_RAW** `[10]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH5_INT_RAW** `[9]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH4_INT_RAW** `[8]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH3_INT_RAW** `[7]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH2_INT_RAW** `[6]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH1_INT_RAW** `[5]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished.
///
/// - **DUTY_CHNG_END_CH0_INT_RAW** `[4]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished.
///
/// - **TIMER3_OVF_INT_RAW** `[3]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value.
///
/// - **TIMER2_OVF_INT_RAW** `[2]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value.
///
/// - **TIMER1_OVF_INT_RAW** `[1]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value.
///
/// - **TIMER0_OVF_INT_RAW** `[0]` (RW)
///
///   Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value.
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

    /// **OVF_CNT_CH7_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH7_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OVF_CNT_CH7_INT_RAW_OFFSET: u32 = 19;
    pub const OVF_CNT_CH7_INT_RAW_MASK: u32 = 0x00080000;
    pub const OVF_CNT_CH7_INT_RAW: u32 = 1 << 19;

    /// **OVF_CNT_CH6_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH6_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const OVF_CNT_CH6_INT_RAW_OFFSET: u32 = 18;
    pub const OVF_CNT_CH6_INT_RAW_MASK: u32 = 0x00040000;
    pub const OVF_CNT_CH6_INT_RAW: u32 = 1 << 18;

    /// **OVF_CNT_CH5_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OVF_CNT_CH5_INT_RAW_OFFSET: u32 = 17;
    pub const OVF_CNT_CH5_INT_RAW_MASK: u32 = 0x00020000;
    pub const OVF_CNT_CH5_INT_RAW: u32 = 1 << 17;

    /// **OVF_CNT_CH4_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_CH4_INT_RAW_OFFSET: u32 = 16;
    pub const OVF_CNT_CH4_INT_RAW_MASK: u32 = 0x00010000;
    pub const OVF_CNT_CH4_INT_RAW: u32 = 1 << 16;

    /// **OVF_CNT_CH3_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_CH3_INT_RAW_OFFSET: u32 = 15;
    pub const OVF_CNT_CH3_INT_RAW_MASK: u32 = 0x00008000;
    pub const OVF_CNT_CH3_INT_RAW: u32 = 1 << 15;

    /// **OVF_CNT_CH2_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const OVF_CNT_CH2_INT_RAW_OFFSET: u32 = 14;
    pub const OVF_CNT_CH2_INT_RAW_MASK: u32 = 0x00004000;
    pub const OVF_CNT_CH2_INT_RAW: u32 = 1 << 14;

    /// **OVF_CNT_CH1_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OVF_CNT_CH1_INT_RAW_OFFSET: u32 = 13;
    pub const OVF_CNT_CH1_INT_RAW_MASK: u32 = 0x00002000;
    pub const OVF_CNT_CH1_INT_RAW: u32 = 1 << 13;

    /// **OVF_CNT_CH0_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OVF_CNT_CH0_INT_RAW_OFFSET: u32 = 12;
    pub const OVF_CNT_CH0_INT_RAW_MASK: u32 = 0x00001000;
    pub const OVF_CNT_CH0_INT_RAW: u32 = 1 << 12;

    /// **DUTY_CHNG_END_CH7_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DUTY_CHNG_END_CH7_INT_RAW_OFFSET: u32 = 11;
    pub const DUTY_CHNG_END_CH7_INT_RAW_MASK: u32 = 0x00000800;
    pub const DUTY_CHNG_END_CH7_INT_RAW: u32 = 1 << 11;

    /// **DUTY_CHNG_END_CH6_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DUTY_CHNG_END_CH6_INT_RAW_OFFSET: u32 = 10;
    pub const DUTY_CHNG_END_CH6_INT_RAW_MASK: u32 = 0x00000400;
    pub const DUTY_CHNG_END_CH6_INT_RAW: u32 = 1 << 10;

    /// **DUTY_CHNG_END_CH5_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DUTY_CHNG_END_CH5_INT_RAW_OFFSET: u32 = 9;
    pub const DUTY_CHNG_END_CH5_INT_RAW_MASK: u32 = 0x00000200;
    pub const DUTY_CHNG_END_CH5_INT_RAW: u32 = 1 << 9;

    /// **DUTY_CHNG_END_CH4_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DUTY_CHNG_END_CH4_INT_RAW_OFFSET: u32 = 8;
    pub const DUTY_CHNG_END_CH4_INT_RAW_MASK: u32 = 0x00000100;
    pub const DUTY_CHNG_END_CH4_INT_RAW: u32 = 1 << 8;

    /// **DUTY_CHNG_END_CH3_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DUTY_CHNG_END_CH3_INT_RAW_OFFSET: u32 = 7;
    pub const DUTY_CHNG_END_CH3_INT_RAW_MASK: u32 = 0x00000080;
    pub const DUTY_CHNG_END_CH3_INT_RAW: u32 = 1 << 7;

    /// **DUTY_CHNG_END_CH2_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DUTY_CHNG_END_CH2_INT_RAW_OFFSET: u32 = 6;
    pub const DUTY_CHNG_END_CH2_INT_RAW_MASK: u32 = 0x00000040;
    pub const DUTY_CHNG_END_CH2_INT_RAW: u32 = 1 << 6;

    /// **DUTY_CHNG_END_CH1_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DUTY_CHNG_END_CH1_INT_RAW_OFFSET: u32 = 5;
    pub const DUTY_CHNG_END_CH1_INT_RAW_MASK: u32 = 0x00000020;
    pub const DUTY_CHNG_END_CH1_INT_RAW: u32 = 1 << 5;

    /// **DUTY_CHNG_END_CH0_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DUTY_CHNG_END_CH0_INT_RAW_OFFSET: u32 = 4;
    pub const DUTY_CHNG_END_CH0_INT_RAW_MASK: u32 = 0x00000010;
    pub const DUTY_CHNG_END_CH0_INT_RAW: u32 = 1 << 4;

    /// **TIMER3_OVF_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TIMER3_OVF_INT_RAW_OFFSET: u32 = 3;
    pub const TIMER3_OVF_INT_RAW_MASK: u32 = 0x00000008;
    pub const TIMER3_OVF_INT_RAW: u32 = 1 << 3;

    /// **TIMER2_OVF_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TIMER2_OVF_INT_RAW_OFFSET: u32 = 2;
    pub const TIMER2_OVF_INT_RAW_MASK: u32 = 0x00000004;
    pub const TIMER2_OVF_INT_RAW: u32 = 1 << 2;

    /// **TIMER1_OVF_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER1_OVF_INT_RAW_OFFSET: u32 = 1;
    pub const TIMER1_OVF_INT_RAW_MASK: u32 = 0x00000002;
    pub const TIMER1_OVF_INT_RAW: u32 = 1 << 1;

    /// **TIMER0_OVF_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER0_OVF_INT_RAW_OFFSET: u32 = 0;
    pub const TIMER0_OVF_INT_RAW_MASK: u32 = 0x00000001;
    pub const TIMER0_OVF_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OVF_CNT_CH7_INT_RAW** `[19]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH7_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7.
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH7_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch7_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH7_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch7_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH7_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH6_INT_RAW** `[18]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH6_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6.
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH6_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH6_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch6_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH6_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH6_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch6_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH6_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH5_INT_RAW** `[17]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5.
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH5_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH5_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch5_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH5_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH5_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch5_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH5_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH4_INT_RAW** `[16]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4.
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH4_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH4_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch4_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH4_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH4_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch4_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH4_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH3_INT_RAW** `[15]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3.
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH3_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch3_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH3_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch3_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH3_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH2_INT_RAW** `[14]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2.
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH2_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch2_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH2_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch2_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH2_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH1_INT_RAW** `[13]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1.
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH1_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch1_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH1_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH1_INT_RAW_MASK);
    }

    /// **OVF_CNT_CH0_INT_RAW** `[12]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0.
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH0_INT_RAW_MASK) != 0
    }

    /// Set OVF_CNT_CH0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch0_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH0_INT_RAW_MASK);
    }

    /// Clear OVF_CNT_CH0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch0_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH0_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH7_INT_RAW** `[11]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH7_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch7_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH7_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH7_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch7_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH7_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH6_INT_RAW** `[10]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH6_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH6_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch6_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH6_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH6_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch6_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH6_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH5_INT_RAW** `[9]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH5_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH5_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch5_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH5_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH5_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch5_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH5_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH4_INT_RAW** `[8]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH4_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH4_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch4_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH4_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH4_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch4_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH4_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH3_INT_RAW** `[7]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH3_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch3_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH3_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch3_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH3_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH2_INT_RAW** `[6]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH2_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch2_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH2_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch2_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH2_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH1_INT_RAW** `[5]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH1_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch1_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH1_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH1_INT_RAW_MASK);
    }

    /// **DUTY_CHNG_END_CH0_INT_RAW** `[4]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH0_INT_RAW_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch0_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH0_INT_RAW_MASK);
    }

    /// Clear DUTY_CHNG_END_CH0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch0_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH0_INT_RAW_MASK);
    }

    /// **TIMER3_OVF_INT_RAW** `[3]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value.
    #[inline(always)]
    pub fn timer3_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIMER3_OVF_INT_RAW_MASK) != 0
    }

    /// Set TIMER3_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer3_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER3_OVF_INT_RAW_MASK);
    }

    /// Clear TIMER3_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer3_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER3_OVF_INT_RAW_MASK);
    }

    /// **TIMER2_OVF_INT_RAW** `[2]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value.
    #[inline(always)]
    pub fn timer2_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIMER2_OVF_INT_RAW_MASK) != 0
    }

    /// Set TIMER2_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer2_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER2_OVF_INT_RAW_MASK);
    }

    /// Clear TIMER2_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer2_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER2_OVF_INT_RAW_MASK);
    }

    /// **TIMER1_OVF_INT_RAW** `[1]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value.
    #[inline(always)]
    pub fn timer1_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_OVF_INT_RAW_MASK) != 0
    }

    /// Set TIMER1_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer1_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER1_OVF_INT_RAW_MASK);
    }

    /// Clear TIMER1_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer1_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER1_OVF_INT_RAW_MASK);
    }

    /// **TIMER0_OVF_INT_RAW** `[0]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value.
    #[inline(always)]
    pub fn timer0_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_OVF_INT_RAW_MASK) != 0
    }

    /// Set TIMER0_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer0_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER0_OVF_INT_RAW_MASK);
    }

    /// Clear TIMER0_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer0_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER0_OVF_INT_RAW_MASK);
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

    /// Read **OVF_CNT_CH7_INT_RAW** `[19:19]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH7_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7.
    #[inline(always)]
    pub const fn ovf_cnt_ch7_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH7_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH6_INT_RAW** `[18:18]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH6_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6.
    #[inline(always)]
    pub const fn ovf_cnt_ch6_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH6_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH5_INT_RAW** `[17:17]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5.
    #[inline(always)]
    pub const fn ovf_cnt_ch5_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH5_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH4_INT_RAW** `[16:16]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4.
    #[inline(always)]
    pub const fn ovf_cnt_ch4_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH4_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH3_INT_RAW** `[15:15]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3.
    #[inline(always)]
    pub const fn ovf_cnt_ch3_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH3_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH2_INT_RAW** `[14:14]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2.
    #[inline(always)]
    pub const fn ovf_cnt_ch2_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH2_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH1_INT_RAW** `[13:13]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1.
    #[inline(always)]
    pub const fn ovf_cnt_ch1_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH1_INT_RAW_MASK) != 0
    }

    /// Read **OVF_CNT_CH0_INT_RAW** `[12:12]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0.
    #[inline(always)]
    pub const fn ovf_cnt_ch0_int_raw(&self) -> bool {
        (self.bits & IntRaw::OVF_CNT_CH0_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH7_INT_RAW** `[11:11]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch7_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH7_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH6_INT_RAW** `[10:10]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch6_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH6_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH5_INT_RAW** `[9:9]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch5_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH5_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH4_INT_RAW** `[8:8]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch4_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH4_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH3_INT_RAW** `[7:7]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch3_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH3_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH2_INT_RAW** `[6:6]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch2_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH2_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH1_INT_RAW** `[5:5]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch1_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH1_INT_RAW_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH0_INT_RAW** `[4:4]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn duty_chng_end_ch0_int_raw(&self) -> bool {
        (self.bits & IntRaw::DUTY_CHNG_END_CH0_INT_RAW_MASK) != 0
    }

    /// Read **TIMER3_OVF_INT_RAW** `[3:3]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value.
    #[inline(always)]
    pub const fn timer3_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIMER3_OVF_INT_RAW_MASK) != 0
    }

    /// Read **TIMER2_OVF_INT_RAW** `[2:2]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value.
    #[inline(always)]
    pub const fn timer2_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIMER2_OVF_INT_RAW_MASK) != 0
    }

    /// Read **TIMER1_OVF_INT_RAW** `[1:1]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value.
    #[inline(always)]
    pub const fn timer1_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIMER1_OVF_INT_RAW_MASK) != 0
    }

    /// Read **TIMER0_OVF_INT_RAW** `[0:0]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value.
    #[inline(always)]
    pub const fn timer0_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIMER0_OVF_INT_RAW_MASK) != 0
    }

    /// Set **OVF_CNT_CH7_INT_RAW** `[19:19]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH7_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH7.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch7_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH7_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH7_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH6_INT_RAW** `[18:18]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH6_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH6.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch6_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH6_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH6_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH5_INT_RAW** `[17:17]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch5_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH5_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH5_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH4_INT_RAW** `[16:16]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch4_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH4_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH4_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH3_INT_RAW** `[15:15]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch3_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH3_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH3_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH2_INT_RAW** `[14:14]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH2_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH2_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH1_INT_RAW** `[13:13]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH1_INT_RAW_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH0_INT_RAW** `[12:12]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVF_CNT_CH0_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OVF_CNT_CH0_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH7_INT_RAW** `[11:11]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch7_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH7_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH7_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH6_INT_RAW** `[10:10]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch6_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH6_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH6_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH5_INT_RAW** `[9:9]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch5_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH5_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH5_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH4_INT_RAW** `[8:8]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch4_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH4_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH4_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH3_INT_RAW** `[7:7]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch3_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH3_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH3_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH2_INT_RAW** `[6:6]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH2_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH2_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH1_INT_RAW** `[5:5]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH1_INT_RAW_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH0_INT_RAW** `[4:4]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DUTY_CHNG_END_CH0_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DUTY_CHNG_END_CH0_INT_RAW_MASK
            },
        }
    }

    /// Set **TIMER3_OVF_INT_RAW** `[3:3]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value.
    #[inline(always)]
    pub const fn with_timer3_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TIMER3_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TIMER3_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **TIMER2_OVF_INT_RAW** `[2:2]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value.
    #[inline(always)]
    pub const fn with_timer2_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TIMER2_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TIMER2_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **TIMER1_OVF_INT_RAW** `[1:1]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value.
    #[inline(always)]
    pub const fn with_timer1_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TIMER1_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TIMER1_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **TIMER0_OVF_INT_RAW** `[0:0]`
    ///
    /// Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value.
    #[inline(always)]
    pub const fn with_timer0_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TIMER0_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TIMER0_OVF_INT_RAW_MASK
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
/// Interrupt masked status register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **OVF_CNT_CH7_INT_ST** `[19]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH7_INT. Valid only when LEDC_OVF_CNT_CH7_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH6_INT_ST** `[18]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH6_INT. Valid only when LEDC_OVF_CNT_CH6_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH5_INT_ST** `[17]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH4_INT_ST** `[16]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH3_INT_ST** `[15]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH2_INT_ST** `[14]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH1_INT_ST** `[13]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1.
///
/// - **OVF_CNT_CH0_INT_ST** `[12]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH7_INT_ST** `[11]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Valid only when LEDC_DUTY_CHNG_END_CH7_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH6_INT_ST** `[10]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Valid only when LEDC_DUTY_CHNG_END_CH6_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH5_INT_ST** `[9]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH4_INT_ST** `[8]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH3_INT_ST** `[7]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH2_INT_ST** `[6]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH1_INT_ST** `[5]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1.
///
/// - **DUTY_CHNG_END_CH0_INT_ST** `[4]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1.
///
/// - **TIMER3_OVF_INT_ST** `[3]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1.
///
/// - **TIMER2_OVF_INT_ST** `[2]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1.
///
/// - **TIMER1_OVF_INT_ST** `[1]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1.
///
/// - **TIMER0_OVF_INT_ST** `[0]` (RO)
///
///   Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1.
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

    /// **OVF_CNT_CH7_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH7_INT. Valid only when LEDC_OVF_CNT_CH7_INT_ENA is set to 1.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OVF_CNT_CH7_INT_ST_OFFSET: u32 = 19;
    pub const OVF_CNT_CH7_INT_ST_MASK: u32 = 0x00080000;
    pub const OVF_CNT_CH7_INT_ST: u32 = 1 << 19;

    /// **OVF_CNT_CH6_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH6_INT. Valid only when LEDC_OVF_CNT_CH6_INT_ENA is set to 1.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const OVF_CNT_CH6_INT_ST_OFFSET: u32 = 18;
    pub const OVF_CNT_CH6_INT_ST_MASK: u32 = 0x00040000;
    pub const OVF_CNT_CH6_INT_ST: u32 = 1 << 18;

    /// **OVF_CNT_CH5_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OVF_CNT_CH5_INT_ST_OFFSET: u32 = 17;
    pub const OVF_CNT_CH5_INT_ST_MASK: u32 = 0x00020000;
    pub const OVF_CNT_CH5_INT_ST: u32 = 1 << 17;

    /// **OVF_CNT_CH4_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_CH4_INT_ST_OFFSET: u32 = 16;
    pub const OVF_CNT_CH4_INT_ST_MASK: u32 = 0x00010000;
    pub const OVF_CNT_CH4_INT_ST: u32 = 1 << 16;

    /// **OVF_CNT_CH3_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_CH3_INT_ST_OFFSET: u32 = 15;
    pub const OVF_CNT_CH3_INT_ST_MASK: u32 = 0x00008000;
    pub const OVF_CNT_CH3_INT_ST: u32 = 1 << 15;

    /// **OVF_CNT_CH2_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const OVF_CNT_CH2_INT_ST_OFFSET: u32 = 14;
    pub const OVF_CNT_CH2_INT_ST_MASK: u32 = 0x00004000;
    pub const OVF_CNT_CH2_INT_ST: u32 = 1 << 14;

    /// **OVF_CNT_CH1_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OVF_CNT_CH1_INT_ST_OFFSET: u32 = 13;
    pub const OVF_CNT_CH1_INT_ST_MASK: u32 = 0x00002000;
    pub const OVF_CNT_CH1_INT_ST: u32 = 1 << 13;

    /// **OVF_CNT_CH0_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OVF_CNT_CH0_INT_ST_OFFSET: u32 = 12;
    pub const OVF_CNT_CH0_INT_ST_MASK: u32 = 0x00001000;
    pub const OVF_CNT_CH0_INT_ST: u32 = 1 << 12;

    /// **DUTY_CHNG_END_CH7_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Valid only when LEDC_DUTY_CHNG_END_CH7_INT_ENA is set to 1.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DUTY_CHNG_END_CH7_INT_ST_OFFSET: u32 = 11;
    pub const DUTY_CHNG_END_CH7_INT_ST_MASK: u32 = 0x00000800;
    pub const DUTY_CHNG_END_CH7_INT_ST: u32 = 1 << 11;

    /// **DUTY_CHNG_END_CH6_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Valid only when LEDC_DUTY_CHNG_END_CH6_INT_ENA is set to 1.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DUTY_CHNG_END_CH6_INT_ST_OFFSET: u32 = 10;
    pub const DUTY_CHNG_END_CH6_INT_ST_MASK: u32 = 0x00000400;
    pub const DUTY_CHNG_END_CH6_INT_ST: u32 = 1 << 10;

    /// **DUTY_CHNG_END_CH5_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DUTY_CHNG_END_CH5_INT_ST_OFFSET: u32 = 9;
    pub const DUTY_CHNG_END_CH5_INT_ST_MASK: u32 = 0x00000200;
    pub const DUTY_CHNG_END_CH5_INT_ST: u32 = 1 << 9;

    /// **DUTY_CHNG_END_CH4_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DUTY_CHNG_END_CH4_INT_ST_OFFSET: u32 = 8;
    pub const DUTY_CHNG_END_CH4_INT_ST_MASK: u32 = 0x00000100;
    pub const DUTY_CHNG_END_CH4_INT_ST: u32 = 1 << 8;

    /// **DUTY_CHNG_END_CH3_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DUTY_CHNG_END_CH3_INT_ST_OFFSET: u32 = 7;
    pub const DUTY_CHNG_END_CH3_INT_ST_MASK: u32 = 0x00000080;
    pub const DUTY_CHNG_END_CH3_INT_ST: u32 = 1 << 7;

    /// **DUTY_CHNG_END_CH2_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DUTY_CHNG_END_CH2_INT_ST_OFFSET: u32 = 6;
    pub const DUTY_CHNG_END_CH2_INT_ST_MASK: u32 = 0x00000040;
    pub const DUTY_CHNG_END_CH2_INT_ST: u32 = 1 << 6;

    /// **DUTY_CHNG_END_CH1_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DUTY_CHNG_END_CH1_INT_ST_OFFSET: u32 = 5;
    pub const DUTY_CHNG_END_CH1_INT_ST_MASK: u32 = 0x00000020;
    pub const DUTY_CHNG_END_CH1_INT_ST: u32 = 1 << 5;

    /// **DUTY_CHNG_END_CH0_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DUTY_CHNG_END_CH0_INT_ST_OFFSET: u32 = 4;
    pub const DUTY_CHNG_END_CH0_INT_ST_MASK: u32 = 0x00000010;
    pub const DUTY_CHNG_END_CH0_INT_ST: u32 = 1 << 4;

    /// **TIMER3_OVF_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TIMER3_OVF_INT_ST_OFFSET: u32 = 3;
    pub const TIMER3_OVF_INT_ST_MASK: u32 = 0x00000008;
    pub const TIMER3_OVF_INT_ST: u32 = 1 << 3;

    /// **TIMER2_OVF_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TIMER2_OVF_INT_ST_OFFSET: u32 = 2;
    pub const TIMER2_OVF_INT_ST_MASK: u32 = 0x00000004;
    pub const TIMER2_OVF_INT_ST: u32 = 1 << 2;

    /// **TIMER1_OVF_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER1_OVF_INT_ST_OFFSET: u32 = 1;
    pub const TIMER1_OVF_INT_ST_MASK: u32 = 0x00000002;
    pub const TIMER1_OVF_INT_ST: u32 = 1 << 1;

    /// **TIMER0_OVF_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER0_OVF_INT_ST_OFFSET: u32 = 0;
    pub const TIMER0_OVF_INT_ST_MASK: u32 = 0x00000001;
    pub const TIMER0_OVF_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OVF_CNT_CH7_INT_ST** `[19]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH7_INT. Valid only when LEDC_OVF_CNT_CH7_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH7_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH6_INT_ST** `[18]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH6_INT. Valid only when LEDC_OVF_CNT_CH6_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH6_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH5_INT_ST** `[17]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH5_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH4_INT_ST** `[16]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH4_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH3_INT_ST** `[15]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH3_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH2_INT_ST** `[14]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH2_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH1_INT_ST** `[13]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH1_INT_ST_MASK) != 0
    }

    /// **OVF_CNT_CH0_INT_ST** `[12]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1.
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH0_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH7_INT_ST** `[11]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Valid only when LEDC_DUTY_CHNG_END_CH7_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH7_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH6_INT_ST** `[10]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Valid only when LEDC_DUTY_CHNG_END_CH6_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH6_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH5_INT_ST** `[9]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH5_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH4_INT_ST** `[8]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH4_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH3_INT_ST** `[7]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH3_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH2_INT_ST** `[6]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH2_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH1_INT_ST** `[5]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH1_INT_ST_MASK) != 0
    }

    /// **DUTY_CHNG_END_CH0_INT_ST** `[4]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1.
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH0_INT_ST_MASK) != 0
    }

    /// **TIMER3_OVF_INT_ST** `[3]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub fn timer3_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIMER3_OVF_INT_ST_MASK) != 0
    }

    /// **TIMER2_OVF_INT_ST** `[2]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub fn timer2_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIMER2_OVF_INT_ST_MASK) != 0
    }

    /// **TIMER1_OVF_INT_ST** `[1]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub fn timer1_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_OVF_INT_ST_MASK) != 0
    }

    /// **TIMER0_OVF_INT_ST** `[0]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub fn timer0_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_OVF_INT_ST_MASK) != 0
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

    /// Read **OVF_CNT_CH7_INT_ST** `[19:19]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH7_INT. Valid only when LEDC_OVF_CNT_CH7_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch7_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH7_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH6_INT_ST** `[18:18]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH6_INT. Valid only when LEDC_OVF_CNT_CH6_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch6_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH6_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH5_INT_ST** `[17:17]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch5_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH5_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH4_INT_ST** `[16:16]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch4_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH4_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH3_INT_ST** `[15:15]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch3_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH3_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH2_INT_ST** `[14:14]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch2_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH2_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH1_INT_ST** `[13:13]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch1_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH1_INT_ST_MASK) != 0
    }

    /// Read **OVF_CNT_CH0_INT_ST** `[12:12]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn ovf_cnt_ch0_int_st(&self) -> bool {
        (self.bits & IntSt::OVF_CNT_CH0_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH7_INT_ST** `[11:11]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH7_INT. Valid only when LEDC_DUTY_CHNG_END_CH7_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch7_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH7_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH6_INT_ST** `[10:10]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH6_INT. Valid only when LEDC_DUTY_CHNG_END_CH6_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch6_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH6_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH5_INT_ST** `[9:9]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch5_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH5_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH4_INT_ST** `[8:8]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch4_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH4_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH3_INT_ST** `[7:7]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch3_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH3_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH2_INT_ST** `[6:6]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch2_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH2_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH1_INT_ST** `[5:5]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch1_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH1_INT_ST_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH0_INT_ST** `[4:4]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn duty_chng_end_ch0_int_st(&self) -> bool {
        (self.bits & IntSt::DUTY_CHNG_END_CH0_INT_ST_MASK) != 0
    }

    /// Read **TIMER3_OVF_INT_ST** `[3:3]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn timer3_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::TIMER3_OVF_INT_ST_MASK) != 0
    }

    /// Read **TIMER2_OVF_INT_ST** `[2:2]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn timer2_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::TIMER2_OVF_INT_ST_MASK) != 0
    }

    /// Read **TIMER1_OVF_INT_ST** `[1:1]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn timer1_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::TIMER1_OVF_INT_ST_MASK) != 0
    }

    /// Read **TIMER0_OVF_INT_ST** `[0:0]`
    ///
    /// Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn timer0_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::TIMER0_OVF_INT_ST_MASK) != 0
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
/// Interrupt enable register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **OVF_CNT_CH7_INT_ENA** `[19]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH7_INT.
///
/// - **OVF_CNT_CH6_INT_ENA** `[18]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH6_INT.
///
/// - **OVF_CNT_CH5_INT_ENA** `[17]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH5_INT.
///
/// - **OVF_CNT_CH4_INT_ENA** `[16]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH4_INT.
///
/// - **OVF_CNT_CH3_INT_ENA** `[15]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH3_INT.
///
/// - **OVF_CNT_CH2_INT_ENA** `[14]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH2_INT.
///
/// - **OVF_CNT_CH1_INT_ENA** `[13]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH1_INT.
///
/// - **OVF_CNT_CH0_INT_ENA** `[12]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_OVF_CNT_CH0_INT.
///
/// - **DUTY_CHNG_END_CH7_INT_ENA** `[11]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH7_INT.
///
/// - **DUTY_CHNG_END_CH6_INT_ENA** `[10]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH6_INT.
///
/// - **DUTY_CHNG_END_CH5_INT_ENA** `[9]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH5_INT.
///
/// - **DUTY_CHNG_END_CH4_INT_ENA** `[8]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH4_INT.
///
/// - **DUTY_CHNG_END_CH3_INT_ENA** `[7]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH3_INT.
///
/// - **DUTY_CHNG_END_CH2_INT_ENA** `[6]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH2_INT.
///
/// - **DUTY_CHNG_END_CH1_INT_ENA** `[5]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH1_INT.
///
/// - **DUTY_CHNG_END_CH0_INT_ENA** `[4]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH0_INT.
///
/// - **TIMER3_OVF_INT_ENA** `[3]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_TIMER3_OVF_INT.
///
/// - **TIMER2_OVF_INT_ENA** `[2]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_TIMER2_OVF_INT.
///
/// - **TIMER1_OVF_INT_ENA** `[1]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_TIMER1_OVF_INT.
///
/// - **TIMER0_OVF_INT_ENA** `[0]` (RW)
///
///   Enable bit: Write 1 to enable LEDC_TIMER0_OVF_INT.
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

    /// **OVF_CNT_CH7_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH7_INT.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OVF_CNT_CH7_INT_ENA_OFFSET: u32 = 19;
    pub const OVF_CNT_CH7_INT_ENA_MASK: u32 = 0x00080000;
    pub const OVF_CNT_CH7_INT_ENA: u32 = 1 << 19;

    /// **OVF_CNT_CH6_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH6_INT.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const OVF_CNT_CH6_INT_ENA_OFFSET: u32 = 18;
    pub const OVF_CNT_CH6_INT_ENA_MASK: u32 = 0x00040000;
    pub const OVF_CNT_CH6_INT_ENA: u32 = 1 << 18;

    /// **OVF_CNT_CH5_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH5_INT.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OVF_CNT_CH5_INT_ENA_OFFSET: u32 = 17;
    pub const OVF_CNT_CH5_INT_ENA_MASK: u32 = 0x00020000;
    pub const OVF_CNT_CH5_INT_ENA: u32 = 1 << 17;

    /// **OVF_CNT_CH4_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH4_INT.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_CH4_INT_ENA_OFFSET: u32 = 16;
    pub const OVF_CNT_CH4_INT_ENA_MASK: u32 = 0x00010000;
    pub const OVF_CNT_CH4_INT_ENA: u32 = 1 << 16;

    /// **OVF_CNT_CH3_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH3_INT.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_CH3_INT_ENA_OFFSET: u32 = 15;
    pub const OVF_CNT_CH3_INT_ENA_MASK: u32 = 0x00008000;
    pub const OVF_CNT_CH3_INT_ENA: u32 = 1 << 15;

    /// **OVF_CNT_CH2_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH2_INT.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const OVF_CNT_CH2_INT_ENA_OFFSET: u32 = 14;
    pub const OVF_CNT_CH2_INT_ENA_MASK: u32 = 0x00004000;
    pub const OVF_CNT_CH2_INT_ENA: u32 = 1 << 14;

    /// **OVF_CNT_CH1_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH1_INT.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OVF_CNT_CH1_INT_ENA_OFFSET: u32 = 13;
    pub const OVF_CNT_CH1_INT_ENA_MASK: u32 = 0x00002000;
    pub const OVF_CNT_CH1_INT_ENA: u32 = 1 << 13;

    /// **OVF_CNT_CH0_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH0_INT.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OVF_CNT_CH0_INT_ENA_OFFSET: u32 = 12;
    pub const OVF_CNT_CH0_INT_ENA_MASK: u32 = 0x00001000;
    pub const OVF_CNT_CH0_INT_ENA: u32 = 1 << 12;

    /// **DUTY_CHNG_END_CH7_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH7_INT.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DUTY_CHNG_END_CH7_INT_ENA_OFFSET: u32 = 11;
    pub const DUTY_CHNG_END_CH7_INT_ENA_MASK: u32 = 0x00000800;
    pub const DUTY_CHNG_END_CH7_INT_ENA: u32 = 1 << 11;

    /// **DUTY_CHNG_END_CH6_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH6_INT.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DUTY_CHNG_END_CH6_INT_ENA_OFFSET: u32 = 10;
    pub const DUTY_CHNG_END_CH6_INT_ENA_MASK: u32 = 0x00000400;
    pub const DUTY_CHNG_END_CH6_INT_ENA: u32 = 1 << 10;

    /// **DUTY_CHNG_END_CH5_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH5_INT.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DUTY_CHNG_END_CH5_INT_ENA_OFFSET: u32 = 9;
    pub const DUTY_CHNG_END_CH5_INT_ENA_MASK: u32 = 0x00000200;
    pub const DUTY_CHNG_END_CH5_INT_ENA: u32 = 1 << 9;

    /// **DUTY_CHNG_END_CH4_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH4_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DUTY_CHNG_END_CH4_INT_ENA_OFFSET: u32 = 8;
    pub const DUTY_CHNG_END_CH4_INT_ENA_MASK: u32 = 0x00000100;
    pub const DUTY_CHNG_END_CH4_INT_ENA: u32 = 1 << 8;

    /// **DUTY_CHNG_END_CH3_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH3_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DUTY_CHNG_END_CH3_INT_ENA_OFFSET: u32 = 7;
    pub const DUTY_CHNG_END_CH3_INT_ENA_MASK: u32 = 0x00000080;
    pub const DUTY_CHNG_END_CH3_INT_ENA: u32 = 1 << 7;

    /// **DUTY_CHNG_END_CH2_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH2_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DUTY_CHNG_END_CH2_INT_ENA_OFFSET: u32 = 6;
    pub const DUTY_CHNG_END_CH2_INT_ENA_MASK: u32 = 0x00000040;
    pub const DUTY_CHNG_END_CH2_INT_ENA: u32 = 1 << 6;

    /// **DUTY_CHNG_END_CH1_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH1_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DUTY_CHNG_END_CH1_INT_ENA_OFFSET: u32 = 5;
    pub const DUTY_CHNG_END_CH1_INT_ENA_MASK: u32 = 0x00000020;
    pub const DUTY_CHNG_END_CH1_INT_ENA: u32 = 1 << 5;

    /// **DUTY_CHNG_END_CH0_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH0_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DUTY_CHNG_END_CH0_INT_ENA_OFFSET: u32 = 4;
    pub const DUTY_CHNG_END_CH0_INT_ENA_MASK: u32 = 0x00000010;
    pub const DUTY_CHNG_END_CH0_INT_ENA: u32 = 1 << 4;

    /// **TIMER3_OVF_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER3_OVF_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TIMER3_OVF_INT_ENA_OFFSET: u32 = 3;
    pub const TIMER3_OVF_INT_ENA_MASK: u32 = 0x00000008;
    pub const TIMER3_OVF_INT_ENA: u32 = 1 << 3;

    /// **TIMER2_OVF_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER2_OVF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TIMER2_OVF_INT_ENA_OFFSET: u32 = 2;
    pub const TIMER2_OVF_INT_ENA_MASK: u32 = 0x00000004;
    pub const TIMER2_OVF_INT_ENA: u32 = 1 << 2;

    /// **TIMER1_OVF_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER1_OVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER1_OVF_INT_ENA_OFFSET: u32 = 1;
    pub const TIMER1_OVF_INT_ENA_MASK: u32 = 0x00000002;
    pub const TIMER1_OVF_INT_ENA: u32 = 1 << 1;

    /// **TIMER0_OVF_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER0_OVF_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER0_OVF_INT_ENA_OFFSET: u32 = 0;
    pub const TIMER0_OVF_INT_ENA_MASK: u32 = 0x00000001;
    pub const TIMER0_OVF_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OVF_CNT_CH7_INT_ENA** `[19]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH7_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH7_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch7_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH7_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch7_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH7_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH6_INT_ENA** `[18]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH6_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH6_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH6_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch6_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH6_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH6_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch6_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH6_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH5_INT_ENA** `[17]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH5_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH5_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH5_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch5_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH5_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH5_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch5_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH5_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH4_INT_ENA** `[16]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH4_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH4_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH4_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch4_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH4_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH4_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch4_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH4_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH3_INT_ENA** `[15]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH3_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH3_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch3_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH3_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch3_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH3_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH2_INT_ENA** `[14]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH2_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH2_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch2_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH2_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch2_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH2_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH1_INT_ENA** `[13]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH1_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH1_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch1_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH1_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH1_INT_ENA_MASK);
    }

    /// **OVF_CNT_CH0_INT_ENA** `[12]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH0_INT.
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVF_CNT_CH0_INT_ENA_MASK) != 0
    }

    /// Set OVF_CNT_CH0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovf_cnt_ch0_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::OVF_CNT_CH0_INT_ENA_MASK);
    }

    /// Clear OVF_CNT_CH0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovf_cnt_ch0_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::OVF_CNT_CH0_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH7_INT_ENA** `[11]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH7_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH7_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch7_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH7_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH7_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch7_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH7_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH6_INT_ENA** `[10]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH6_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH6_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH6_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch6_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH6_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH6_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch6_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH6_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH5_INT_ENA** `[9]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH5_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH5_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH5_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch5_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH5_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH5_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch5_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH5_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH4_INT_ENA** `[8]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH4_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH4_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH4_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch4_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH4_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH4_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch4_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH4_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH3_INT_ENA** `[7]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH3_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH3_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch3_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH3_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch3_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH3_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH2_INT_ENA** `[6]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH2_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH2_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch2_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH2_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch2_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH2_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH1_INT_ENA** `[5]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH1_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH1_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch1_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH1_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH1_INT_ENA_MASK);
    }

    /// **DUTY_CHNG_END_CH0_INT_ENA** `[4]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH0_INT.
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_CHNG_END_CH0_INT_ENA_MASK) != 0
    }

    /// Set DUTY_CHNG_END_CH0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty_chng_end_ch0_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DUTY_CHNG_END_CH0_INT_ENA_MASK);
    }

    /// Clear DUTY_CHNG_END_CH0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty_chng_end_ch0_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DUTY_CHNG_END_CH0_INT_ENA_MASK);
    }

    /// **TIMER3_OVF_INT_ENA** `[3]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER3_OVF_INT.
    #[inline(always)]
    pub fn timer3_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIMER3_OVF_INT_ENA_MASK) != 0
    }

    /// Set TIMER3_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer3_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER3_OVF_INT_ENA_MASK);
    }

    /// Clear TIMER3_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer3_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER3_OVF_INT_ENA_MASK);
    }

    /// **TIMER2_OVF_INT_ENA** `[2]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER2_OVF_INT.
    #[inline(always)]
    pub fn timer2_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIMER2_OVF_INT_ENA_MASK) != 0
    }

    /// Set TIMER2_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer2_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER2_OVF_INT_ENA_MASK);
    }

    /// Clear TIMER2_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer2_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER2_OVF_INT_ENA_MASK);
    }

    /// **TIMER1_OVF_INT_ENA** `[1]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER1_OVF_INT.
    #[inline(always)]
    pub fn timer1_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_OVF_INT_ENA_MASK) != 0
    }

    /// Set TIMER1_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer1_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER1_OVF_INT_ENA_MASK);
    }

    /// Clear TIMER1_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer1_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER1_OVF_INT_ENA_MASK);
    }

    /// **TIMER0_OVF_INT_ENA** `[0]` (RW)
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER0_OVF_INT.
    #[inline(always)]
    pub fn timer0_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_OVF_INT_ENA_MASK) != 0
    }

    /// Set TIMER0_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer0_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER0_OVF_INT_ENA_MASK);
    }

    /// Clear TIMER0_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer0_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER0_OVF_INT_ENA_MASK);
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

    /// Read **OVF_CNT_CH7_INT_ENA** `[19:19]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH7_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch7_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH7_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH6_INT_ENA** `[18:18]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH6_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch6_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH6_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH5_INT_ENA** `[17:17]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH5_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch5_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH5_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH4_INT_ENA** `[16:16]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH4_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch4_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH4_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH3_INT_ENA** `[15:15]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH3_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch3_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH3_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH2_INT_ENA** `[14:14]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH2_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch2_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH2_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH1_INT_ENA** `[13:13]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH1_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch1_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH1_INT_ENA_MASK) != 0
    }

    /// Read **OVF_CNT_CH0_INT_ENA** `[12:12]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH0_INT.
    #[inline(always)]
    pub const fn ovf_cnt_ch0_int_ena(&self) -> bool {
        (self.bits & IntEna::OVF_CNT_CH0_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH7_INT_ENA** `[11:11]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH7_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch7_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH7_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH6_INT_ENA** `[10:10]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH6_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch6_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH6_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH5_INT_ENA** `[9:9]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH5_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch5_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH5_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH4_INT_ENA** `[8:8]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH4_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch4_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH4_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH3_INT_ENA** `[7:7]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH3_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch3_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH3_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH2_INT_ENA** `[6:6]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH2_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch2_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH2_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH1_INT_ENA** `[5:5]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH1_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch1_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH1_INT_ENA_MASK) != 0
    }

    /// Read **DUTY_CHNG_END_CH0_INT_ENA** `[4:4]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH0_INT.
    #[inline(always)]
    pub const fn duty_chng_end_ch0_int_ena(&self) -> bool {
        (self.bits & IntEna::DUTY_CHNG_END_CH0_INT_ENA_MASK) != 0
    }

    /// Read **TIMER3_OVF_INT_ENA** `[3:3]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER3_OVF_INT.
    #[inline(always)]
    pub const fn timer3_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::TIMER3_OVF_INT_ENA_MASK) != 0
    }

    /// Read **TIMER2_OVF_INT_ENA** `[2:2]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER2_OVF_INT.
    #[inline(always)]
    pub const fn timer2_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::TIMER2_OVF_INT_ENA_MASK) != 0
    }

    /// Read **TIMER1_OVF_INT_ENA** `[1:1]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER1_OVF_INT.
    #[inline(always)]
    pub const fn timer1_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::TIMER1_OVF_INT_ENA_MASK) != 0
    }

    /// Read **TIMER0_OVF_INT_ENA** `[0:0]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER0_OVF_INT.
    #[inline(always)]
    pub const fn timer0_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::TIMER0_OVF_INT_ENA_MASK) != 0
    }

    /// Set **OVF_CNT_CH7_INT_ENA** `[19:19]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH7_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch7_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH7_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH7_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH6_INT_ENA** `[18:18]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH6_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch6_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH6_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH6_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH5_INT_ENA** `[17:17]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH5_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch5_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH5_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH5_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH4_INT_ENA** `[16:16]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH4_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch4_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH4_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH4_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH3_INT_ENA** `[15:15]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH3_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch3_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH3_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH3_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH2_INT_ENA** `[14:14]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH2_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH2_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH2_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH1_INT_ENA** `[13:13]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH1_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH1_INT_ENA_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH0_INT_ENA** `[12:12]`
    ///
    /// Enable bit: Write 1 to enable LEDC_OVF_CNT_CH0_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVF_CNT_CH0_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OVF_CNT_CH0_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH7_INT_ENA** `[11:11]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH7_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch7_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH7_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH7_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH6_INT_ENA** `[10:10]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH6_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch6_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH6_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH6_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH5_INT_ENA** `[9:9]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH5_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch5_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH5_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH5_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH4_INT_ENA** `[8:8]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH4_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch4_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH4_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH4_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH3_INT_ENA** `[7:7]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH3_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch3_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH3_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH3_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH2_INT_ENA** `[6:6]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH2_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH2_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH2_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH1_INT_ENA** `[5:5]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH1_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH1_INT_ENA_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH0_INT_ENA** `[4:4]`
    ///
    /// Enable bit: Write 1 to enable LEDC_DUTY_CHNG_END_CH0_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DUTY_CHNG_END_CH0_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DUTY_CHNG_END_CH0_INT_ENA_MASK
            },
        }
    }

    /// Set **TIMER3_OVF_INT_ENA** `[3:3]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER3_OVF_INT.
    #[inline(always)]
    pub const fn with_timer3_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIMER3_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIMER3_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **TIMER2_OVF_INT_ENA** `[2:2]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER2_OVF_INT.
    #[inline(always)]
    pub const fn with_timer2_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIMER2_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIMER2_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **TIMER1_OVF_INT_ENA** `[1:1]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER1_OVF_INT.
    #[inline(always)]
    pub const fn with_timer1_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIMER1_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIMER1_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **TIMER0_OVF_INT_ENA** `[0:0]`
    ///
    /// Enable bit: Write 1 to enable LEDC_TIMER0_OVF_INT.
    #[inline(always)]
    pub const fn with_timer0_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIMER0_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIMER0_OVF_INT_ENA_MASK
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
/// Interrupt clear register
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **OVF_CNT_CH7_INT_CLR** `[19]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT.
///
/// - **OVF_CNT_CH6_INT_CLR** `[18]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT.
///
/// - **OVF_CNT_CH5_INT_CLR** `[17]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT.
///
/// - **OVF_CNT_CH4_INT_CLR** `[16]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT.
///
/// - **OVF_CNT_CH3_INT_CLR** `[15]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT.
///
/// - **OVF_CNT_CH2_INT_CLR** `[14]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT.
///
/// - **OVF_CNT_CH1_INT_CLR** `[13]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT.
///
/// - **OVF_CNT_CH0_INT_CLR** `[12]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT.
///
/// - **DUTY_CHNG_END_CH7_INT_CLR** `[11]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT.
///
/// - **DUTY_CHNG_END_CH6_INT_CLR** `[10]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT.
///
/// - **DUTY_CHNG_END_CH5_INT_CLR** `[9]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT.
///
/// - **DUTY_CHNG_END_CH4_INT_CLR** `[8]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT.
///
/// - **DUTY_CHNG_END_CH3_INT_CLR** `[7]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT.
///
/// - **DUTY_CHNG_END_CH2_INT_CLR** `[6]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT.
///
/// - **DUTY_CHNG_END_CH1_INT_CLR** `[5]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT.
///
/// - **DUTY_CHNG_END_CH0_INT_CLR** `[4]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT.
///
/// - **TIMER3_OVF_INT_CLR** `[3]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT.
///
/// - **TIMER2_OVF_INT_CLR** `[2]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT.
///
/// - **TIMER1_OVF_INT_CLR** `[1]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT.
///
/// - **TIMER0_OVF_INT_CLR** `[0]` (WO)
///
///   Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT.
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

    /// **OVF_CNT_CH7_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OVF_CNT_CH7_INT_CLR_OFFSET: u32 = 19;
    pub const OVF_CNT_CH7_INT_CLR_MASK: u32 = 0x00080000;
    pub const OVF_CNT_CH7_INT_CLR: u32 = 1 << 19;

    /// **OVF_CNT_CH6_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const OVF_CNT_CH6_INT_CLR_OFFSET: u32 = 18;
    pub const OVF_CNT_CH6_INT_CLR_MASK: u32 = 0x00040000;
    pub const OVF_CNT_CH6_INT_CLR: u32 = 1 << 18;

    /// **OVF_CNT_CH5_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OVF_CNT_CH5_INT_CLR_OFFSET: u32 = 17;
    pub const OVF_CNT_CH5_INT_CLR_MASK: u32 = 0x00020000;
    pub const OVF_CNT_CH5_INT_CLR: u32 = 1 << 17;

    /// **OVF_CNT_CH4_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVF_CNT_CH4_INT_CLR_OFFSET: u32 = 16;
    pub const OVF_CNT_CH4_INT_CLR_MASK: u32 = 0x00010000;
    pub const OVF_CNT_CH4_INT_CLR: u32 = 1 << 16;

    /// **OVF_CNT_CH3_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OVF_CNT_CH3_INT_CLR_OFFSET: u32 = 15;
    pub const OVF_CNT_CH3_INT_CLR_MASK: u32 = 0x00008000;
    pub const OVF_CNT_CH3_INT_CLR: u32 = 1 << 15;

    /// **OVF_CNT_CH2_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const OVF_CNT_CH2_INT_CLR_OFFSET: u32 = 14;
    pub const OVF_CNT_CH2_INT_CLR_MASK: u32 = 0x00004000;
    pub const OVF_CNT_CH2_INT_CLR: u32 = 1 << 14;

    /// **OVF_CNT_CH1_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OVF_CNT_CH1_INT_CLR_OFFSET: u32 = 13;
    pub const OVF_CNT_CH1_INT_CLR_MASK: u32 = 0x00002000;
    pub const OVF_CNT_CH1_INT_CLR: u32 = 1 << 13;

    /// **OVF_CNT_CH0_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OVF_CNT_CH0_INT_CLR_OFFSET: u32 = 12;
    pub const OVF_CNT_CH0_INT_CLR_MASK: u32 = 0x00001000;
    pub const OVF_CNT_CH0_INT_CLR: u32 = 1 << 12;

    /// **DUTY_CHNG_END_CH7_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DUTY_CHNG_END_CH7_INT_CLR_OFFSET: u32 = 11;
    pub const DUTY_CHNG_END_CH7_INT_CLR_MASK: u32 = 0x00000800;
    pub const DUTY_CHNG_END_CH7_INT_CLR: u32 = 1 << 11;

    /// **DUTY_CHNG_END_CH6_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DUTY_CHNG_END_CH6_INT_CLR_OFFSET: u32 = 10;
    pub const DUTY_CHNG_END_CH6_INT_CLR_MASK: u32 = 0x00000400;
    pub const DUTY_CHNG_END_CH6_INT_CLR: u32 = 1 << 10;

    /// **DUTY_CHNG_END_CH5_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DUTY_CHNG_END_CH5_INT_CLR_OFFSET: u32 = 9;
    pub const DUTY_CHNG_END_CH5_INT_CLR_MASK: u32 = 0x00000200;
    pub const DUTY_CHNG_END_CH5_INT_CLR: u32 = 1 << 9;

    /// **DUTY_CHNG_END_CH4_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DUTY_CHNG_END_CH4_INT_CLR_OFFSET: u32 = 8;
    pub const DUTY_CHNG_END_CH4_INT_CLR_MASK: u32 = 0x00000100;
    pub const DUTY_CHNG_END_CH4_INT_CLR: u32 = 1 << 8;

    /// **DUTY_CHNG_END_CH3_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DUTY_CHNG_END_CH3_INT_CLR_OFFSET: u32 = 7;
    pub const DUTY_CHNG_END_CH3_INT_CLR_MASK: u32 = 0x00000080;
    pub const DUTY_CHNG_END_CH3_INT_CLR: u32 = 1 << 7;

    /// **DUTY_CHNG_END_CH2_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DUTY_CHNG_END_CH2_INT_CLR_OFFSET: u32 = 6;
    pub const DUTY_CHNG_END_CH2_INT_CLR_MASK: u32 = 0x00000040;
    pub const DUTY_CHNG_END_CH2_INT_CLR: u32 = 1 << 6;

    /// **DUTY_CHNG_END_CH1_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DUTY_CHNG_END_CH1_INT_CLR_OFFSET: u32 = 5;
    pub const DUTY_CHNG_END_CH1_INT_CLR_MASK: u32 = 0x00000020;
    pub const DUTY_CHNG_END_CH1_INT_CLR: u32 = 1 << 5;

    /// **DUTY_CHNG_END_CH0_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DUTY_CHNG_END_CH0_INT_CLR_OFFSET: u32 = 4;
    pub const DUTY_CHNG_END_CH0_INT_CLR_MASK: u32 = 0x00000010;
    pub const DUTY_CHNG_END_CH0_INT_CLR: u32 = 1 << 4;

    /// **TIMER3_OVF_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TIMER3_OVF_INT_CLR_OFFSET: u32 = 3;
    pub const TIMER3_OVF_INT_CLR_MASK: u32 = 0x00000008;
    pub const TIMER3_OVF_INT_CLR: u32 = 1 << 3;

    /// **TIMER2_OVF_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TIMER2_OVF_INT_CLR_OFFSET: u32 = 2;
    pub const TIMER2_OVF_INT_CLR_MASK: u32 = 0x00000004;
    pub const TIMER2_OVF_INT_CLR: u32 = 1 << 2;

    /// **TIMER1_OVF_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER1_OVF_INT_CLR_OFFSET: u32 = 1;
    pub const TIMER1_OVF_INT_CLR_MASK: u32 = 0x00000002;
    pub const TIMER1_OVF_INT_CLR: u32 = 1 << 1;

    /// **TIMER0_OVF_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER0_OVF_INT_CLR_OFFSET: u32 = 0;
    pub const TIMER0_OVF_INT_CLR_MASK: u32 = 0x00000001;
    pub const TIMER0_OVF_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **OVF_CNT_CH7_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch7_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH7_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH6_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch6_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH6_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH5_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch5_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH5_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH4_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch4_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH4_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH3_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch3_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH3_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH2_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch2_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH2_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH1_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch1_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH1_INT_CLR_MASK);
    }

    /// Trigger **OVF_CNT_CH0_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT.
    #[inline(always)]
    pub fn trigger_ovf_cnt_ch0_int_clr(&self) {
        self.value.write_bits(Self::OVF_CNT_CH0_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH7_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch7_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH7_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH6_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch6_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH6_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH5_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch5_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH5_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH4_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch4_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH4_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH3_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch3_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH3_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH2_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch2_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH2_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH1_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch1_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH1_INT_CLR_MASK);
    }

    /// Trigger **DUTY_CHNG_END_CH0_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT.
    #[inline(always)]
    pub fn trigger_duty_chng_end_ch0_int_clr(&self) {
        self.value.write_bits(Self::DUTY_CHNG_END_CH0_INT_CLR_MASK);
    }

    /// Trigger **TIMER3_OVF_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT.
    #[inline(always)]
    pub fn trigger_timer3_ovf_int_clr(&self) {
        self.value.write_bits(Self::TIMER3_OVF_INT_CLR_MASK);
    }

    /// Trigger **TIMER2_OVF_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT.
    #[inline(always)]
    pub fn trigger_timer2_ovf_int_clr(&self) {
        self.value.write_bits(Self::TIMER2_OVF_INT_CLR_MASK);
    }

    /// Trigger **TIMER1_OVF_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT.
    #[inline(always)]
    pub fn trigger_timer1_ovf_int_clr(&self) {
        self.value.write_bits(Self::TIMER1_OVF_INT_CLR_MASK);
    }

    /// Trigger **TIMER0_OVF_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT.
    #[inline(always)]
    pub fn trigger_timer0_ovf_int_clr(&self) {
        self.value.write_bits(Self::TIMER0_OVF_INT_CLR_MASK);
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

    /// Set **OVF_CNT_CH7_INT_CLR** `[19:19]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH7_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch7_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH7_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH7_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH6_INT_CLR** `[18:18]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH6_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch6_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH6_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH6_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH5_INT_CLR** `[17:17]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH5_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch5_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH5_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH5_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH4_INT_CLR** `[16:16]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH4_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch4_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH4_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH4_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH3_INT_CLR** `[15:15]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH3_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch3_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH3_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH3_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH2_INT_CLR** `[14:14]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH2_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH2_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH2_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH1_INT_CLR** `[13:13]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH1_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH1_INT_CLR_MASK
            },
        }
    }

    /// Set **OVF_CNT_CH0_INT_CLR** `[12:12]`
    ///
    /// Clear bit: Write 1 to clear LEDC_OVF_CNT_CH0_INT.
    #[inline(always)]
    pub const fn with_ovf_cnt_ch0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVF_CNT_CH0_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OVF_CNT_CH0_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH7_INT_CLR** `[11:11]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH7_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch7_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH7_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH7_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH6_INT_CLR** `[10:10]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH6_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch6_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH6_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH6_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH5_INT_CLR** `[9:9]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH5_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch5_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH5_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH5_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH4_INT_CLR** `[8:8]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH4_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch4_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH4_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH4_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH3_INT_CLR** `[7:7]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH3_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch3_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH3_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH3_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH2_INT_CLR** `[6:6]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH2_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH2_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH2_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH1_INT_CLR** `[5:5]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH1_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH1_INT_CLR_MASK
            },
        }
    }

    /// Set **DUTY_CHNG_END_CH0_INT_CLR** `[4:4]`
    ///
    /// Clear bit: Write 1 to clear LEDC_DUTY_CHNG_END_CH0_INT.
    #[inline(always)]
    pub const fn with_duty_chng_end_ch0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DUTY_CHNG_END_CH0_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DUTY_CHNG_END_CH0_INT_CLR_MASK
            },
        }
    }

    /// Set **TIMER3_OVF_INT_CLR** `[3:3]`
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER3_OVF_INT.
    #[inline(always)]
    pub const fn with_timer3_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIMER3_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIMER3_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **TIMER2_OVF_INT_CLR** `[2:2]`
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER2_OVF_INT.
    #[inline(always)]
    pub const fn with_timer2_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIMER2_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIMER2_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **TIMER1_OVF_INT_CLR** `[1:1]`
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER1_OVF_INT.
    #[inline(always)]
    pub const fn with_timer1_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIMER1_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIMER1_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **TIMER0_OVF_INT_CLR** `[0:0]`
    ///
    /// Clear bit: Write 1 to clear LEDC_TIMER0_OVF_INT.
    #[inline(always)]
    pub const fn with_timer0_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIMER0_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIMER0_OVF_INT_CLR_MASK
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

/// CH0_GAMMA_CONF Register
///
/// Ledc ch0 gamma config register.
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch0GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH0_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch0gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0GammaConfVal {
    bits: u32,
}

impl Ch0GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch0GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch0GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch0GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch0GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch0GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch0GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch0GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch0GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0GammaConfVal {
        Ch0GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0GammaConfVal) -> Ch0GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1_GAMMA_CONF Register
///
/// Ledc ch1 gamma config register.
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch1GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH1_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch1gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1GammaConfVal {
    bits: u32,
}

impl Ch1GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch1GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch1GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch1GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch1GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch1GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch1GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch1GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch1GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1GammaConfVal {
        Ch1GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1GammaConfVal) -> Ch1GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2_GAMMA_CONF Register
///
/// Ledc ch2 gamma config register.
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch2GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH2_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch2gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2GammaConfVal {
    bits: u32,
}

impl Ch2GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch2GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch2GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch2GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch2GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch2GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch2GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch2GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch2GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2GammaConfVal {
        Ch2GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2GammaConfVal) -> Ch2GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH3_GAMMA_CONF Register
///
/// Ledc ch3 gamma config register.
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch3GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH3_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch3gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3GammaConfVal {
    bits: u32,
}

impl Ch3GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch3GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch3GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch3GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch3GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch3GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch3GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch3GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch3GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3GammaConfVal {
        Ch3GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3GammaConfVal) -> Ch3GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4_GAMMA_CONF Register
///
/// Ledc ch4 gamma config register.
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch4GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH4_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch4gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4GammaConfVal {
    bits: u32,
}

impl Ch4GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch4GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch4GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch4GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch4GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch4GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch4GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch4GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch4GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4GammaConfVal {
        Ch4GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4GammaConfVal) -> Ch4GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH5_GAMMA_CONF Register
///
/// Ledc ch5 gamma config register.
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch5GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH5_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch5gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5GammaConfVal {
    bits: u32,
}

impl Ch5GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch5GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch5GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch5GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch5GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch5GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch5GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch5GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch5GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5GammaConfVal {
        Ch5GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5GammaConfVal) -> Ch5GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH6_GAMMA_CONF Register
///
/// Ledc ch6 gamma config register.
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch6GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH6_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch6gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6GammaConfVal {
    bits: u32,
}

impl Ch6GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch6GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch6GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch6GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch6GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch6GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch6GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch6GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch6GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6GammaConfVal {
        Ch6GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6GammaConfVal) -> Ch6GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH7_GAMMA_CONF Register
///
/// Ledc ch7 gamma config register.
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **CH_GAMMA_RESUME** `[6]` (WO)
///
///   Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
///
/// - **CH_GAMMA_PAUSE** `[5]` (WO)
///
///   Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
///
/// - **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
///
///   Configures the number of duty cycle fading rages for LEDC ch%s.
#[repr(transparent)]
pub struct Ch7GammaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaConf {
    // ======== Field Constants ========

    /// **CH_GAMMA_RESUME**
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH_GAMMA_RESUME_OFFSET: u32 = 6;
    pub const CH_GAMMA_RESUME_MASK: u32 = 0x00000040;
    pub const CH_GAMMA_RESUME: u32 = 1 << 6;

    /// **CH_GAMMA_PAUSE**
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH_GAMMA_PAUSE_OFFSET: u32 = 5;
    pub const CH_GAMMA_PAUSE_MASK: u32 = 0x00000020;
    pub const CH_GAMMA_PAUSE: u32 = 1 << 5;

    /// **CH_GAMMA_ENTRY_NUM**
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CH_GAMMA_ENTRY_NUM_OFFSET: u32 = 0;
    pub const CH_GAMMA_ENTRY_NUM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// Trigger **CH_GAMMA_RESUME** (WO)
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub fn trigger_ch_gamma_resume(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_RESUME_MASK);
    }

    /// Trigger **CH_GAMMA_PAUSE** (WO)
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub fn trigger_ch_gamma_pause(&self) {
        self.value.modify_bits(|r| r | Self::CH_GAMMA_PAUSE_MASK);
    }

    /// **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> u32 {
        (self.value.read_bits() & Self::CH_GAMMA_ENTRY_NUM_MASK) >> Self::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Write **CH_GAMMA_ENTRY_NUM** `[4:0]` (RW)
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub fn write_ch_gamma_entry_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Self::CH_GAMMA_ENTRY_NUM_OFFSET) & Self::CH_GAMMA_ENTRY_NUM_MASK)
        });
    }
}

/// CH7_GAMMA_CONF value type for batch read/modify/write operations
///
/// Use `ch7gammaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7gammaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7GammaConfVal {
    bits: u32,
}

impl Ch7GammaConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn ch_gamma_entry_num(&self) -> u32 {
        (self.bits & Ch7GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
            >> Ch7GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET
    }

    /// Set **CH_GAMMA_RESUME** `[6:6]`
    ///
    /// Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Resume
    #[inline(always)]
    pub const fn with_ch_gamma_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7GammaConf::CH_GAMMA_RESUME_MASK
            } else {
                self.bits & !Ch7GammaConf::CH_GAMMA_RESUME_MASK
            },
        }
    }

    /// Set **CH_GAMMA_PAUSE** `[5:5]`
    ///
    /// Configures whether or not to pause duty cycle fading of LEDC ch%s.\\0: Invalid. No effect\\1: Pause
    #[inline(always)]
    pub const fn with_ch_gamma_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7GammaConf::CH_GAMMA_PAUSE_MASK
            } else {
                self.bits & !Ch7GammaConf::CH_GAMMA_PAUSE_MASK
            },
        }
    }

    /// Set **CH_GAMMA_ENTRY_NUM** `[4:0]`
    ///
    /// Configures the number of duty cycle fading rages for LEDC ch%s.
    #[inline(always)]
    pub const fn with_ch_gamma_entry_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7GammaConf::CH_GAMMA_ENTRY_NUM_MASK)
                | ((value << Ch7GammaConf::CH_GAMMA_ENTRY_NUM_OFFSET)
                    & Ch7GammaConf::CH_GAMMA_ENTRY_NUM_MASK),
        }
    }
}

impl Default for Ch7GammaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ch7GammaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7GammaConfVal {
        Ch7GammaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7GammaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7GammaConfVal) -> Ch7GammaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EVT_TASK_EN0 Register
///
/// Ledc event task enable bit register0.
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **TASK_DUTY_SCALE_UPDATE_CH7_EN** `[31]` (RW)
///
///   Configures whether or not to enable the ledc_ch7_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH6_EN** `[30]` (RW)
///
///   Configures whether or not to enable the ledc_ch6_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH5_EN** `[29]` (RW)
///
///   Configures whether or not to enable the ledc_ch5_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH4_EN** `[28]` (RW)
///
///   Configures whether or not to enable the ledc_ch4_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH3_EN** `[27]` (RW)
///
///   Configures whether or not to enable the ledc_ch3_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH2_EN** `[26]` (RW)
///
///   Configures whether or not to enable the ledc_ch2_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH1_EN** `[25]` (RW)
///
///   Configures whether or not to enable the ledc_ch1_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **TASK_DUTY_SCALE_UPDATE_CH0_EN** `[24]` (RW)
///
///   Configures whether or not to enable the ledc_ch0_duty_scale_update task.\\0: Disable\\1: Enable
///
/// - **EVT_TIME3_CMP_EN** `[23]` (RW)
///
///   Configures whether or not to enable the ledc_timer3_cmp event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME2_CMP_EN** `[22]` (RW)
///
///   Configures whether or not to enable the ledc_timer2_cmp event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME1_CMP_EN** `[21]` (RW)
///
///   Configures whether or not to enable the ledc_timer1_cmp event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME0_CMP_EN** `[20]` (RW)
///
///   Configures whether or not to enable the ledc_timer0_cmp event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME_OVF_TIMER3_EN** `[19]` (RW)
///
///   Configures whether or not to enable the ledc_timer3_ovf event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME_OVF_TIMER2_EN** `[18]` (RW)
///
///   Configures whether or not to enable the ledc_timer2_ovf event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME_OVF_TIMER1_EN** `[17]` (RW)
///
///   Configures whether or not to enable the ledc_timer1_ovf event.\\0: Disable\\1: Enable
///
/// - **EVT_TIME_OVF_TIMER0_EN** `[16]` (RW)
///
///   Configures whether or not to enable the ledc_timer0_ovf event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH7_EN** `[15]` (RW)
///
///   Configures whether or not to enable the ledc_ch7_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH6_EN** `[14]` (RW)
///
///   Configures whether or not to enable the ledc_ch6_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH5_EN** `[13]` (RW)
///
///   Configures whether or not to enable the ledc_ch5_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH4_EN** `[12]` (RW)
///
///   Configures whether or not to enable the ledc_ch4_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH3_EN** `[11]` (RW)
///
///   Configures whether or not to enable the ledc_ch3_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH2_EN** `[10]` (RW)
///
///   Configures whether or not to enable the ledc_ch2_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH1_EN** `[9]` (RW)
///
///   Configures whether or not to enable the ledc_ch1_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_OVF_CNT_PLS_CH0_EN** `[8]` (RW)
///
///   Configures whether or not to enable the ledc_ch0_ovf_cnt_pls event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH7_EN** `[7]` (RW)
///
///   Configures whether or not to enable the ledc_ch7_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH6_EN** `[6]` (RW)
///
///   Configures whether or not to enable the ledc_ch6_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH5_EN** `[5]` (RW)
///
///   Configures whether or not to enable the ledc_ch5_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH4_EN** `[4]` (RW)
///
///   Configures whether or not to enable the ledc_ch4_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH3_EN** `[3]` (RW)
///
///   Configures whether or not to enable the ledc_ch3_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH2_EN** `[2]` (RW)
///
///   Configures whether or not to enable the ledc_ch2_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH1_EN** `[1]` (RW)
///
///   Configures whether or not to enable the ledc_ch1_duty_chng_end event.\\0: Disable\\1: Enable
///
/// - **EVT_DUTY_CHNG_END_CH0_EN** `[0]` (RW)
///
///   Configures whether or not to enable the ledc_ch0_duty_chng_end event.\\0: Disable\\1: Enable
#[repr(transparent)]
pub struct EvtTaskEn0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EvtTaskEn0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EvtTaskEn0 {
    // ======== Field Constants ========

    /// **TASK_DUTY_SCALE_UPDATE_CH7_EN**
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH7_EN_OFFSET: u32 = 31;
    pub const TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK: u32 = 0x80000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH7_EN: u32 = 1 << 31;

    /// **TASK_DUTY_SCALE_UPDATE_CH6_EN**
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH6_EN_OFFSET: u32 = 30;
    pub const TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK: u32 = 0x40000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH6_EN: u32 = 1 << 30;

    /// **TASK_DUTY_SCALE_UPDATE_CH5_EN**
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH5_EN_OFFSET: u32 = 29;
    pub const TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK: u32 = 0x20000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH5_EN: u32 = 1 << 29;

    /// **TASK_DUTY_SCALE_UPDATE_CH4_EN**
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH4_EN_OFFSET: u32 = 28;
    pub const TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK: u32 = 0x10000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH4_EN: u32 = 1 << 28;

    /// **TASK_DUTY_SCALE_UPDATE_CH3_EN**
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH3_EN_OFFSET: u32 = 27;
    pub const TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK: u32 = 0x08000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH3_EN: u32 = 1 << 27;

    /// **TASK_DUTY_SCALE_UPDATE_CH2_EN**
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH2_EN_OFFSET: u32 = 26;
    pub const TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK: u32 = 0x04000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH2_EN: u32 = 1 << 26;

    /// **TASK_DUTY_SCALE_UPDATE_CH1_EN**
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH1_EN_OFFSET: u32 = 25;
    pub const TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK: u32 = 0x02000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH1_EN: u32 = 1 << 25;

    /// **TASK_DUTY_SCALE_UPDATE_CH0_EN**
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_scale_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TASK_DUTY_SCALE_UPDATE_CH0_EN_OFFSET: u32 = 24;
    pub const TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK: u32 = 0x01000000;
    pub const TASK_DUTY_SCALE_UPDATE_CH0_EN: u32 = 1 << 24;

    /// **EVT_TIME3_CMP_EN**
    ///
    /// Configures whether or not to enable the ledc_timer3_cmp event.\\0: Disable\\1: Enable
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EVT_TIME3_CMP_EN_OFFSET: u32 = 23;
    pub const EVT_TIME3_CMP_EN_MASK: u32 = 0x00800000;
    pub const EVT_TIME3_CMP_EN: u32 = 1 << 23;

    /// **EVT_TIME2_CMP_EN**
    ///
    /// Configures whether or not to enable the ledc_timer2_cmp event.\\0: Disable\\1: Enable
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EVT_TIME2_CMP_EN_OFFSET: u32 = 22;
    pub const EVT_TIME2_CMP_EN_MASK: u32 = 0x00400000;
    pub const EVT_TIME2_CMP_EN: u32 = 1 << 22;

    /// **EVT_TIME1_CMP_EN**
    ///
    /// Configures whether or not to enable the ledc_timer1_cmp event.\\0: Disable\\1: Enable
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EVT_TIME1_CMP_EN_OFFSET: u32 = 21;
    pub const EVT_TIME1_CMP_EN_MASK: u32 = 0x00200000;
    pub const EVT_TIME1_CMP_EN: u32 = 1 << 21;

    /// **EVT_TIME0_CMP_EN**
    ///
    /// Configures whether or not to enable the ledc_timer0_cmp event.\\0: Disable\\1: Enable
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EVT_TIME0_CMP_EN_OFFSET: u32 = 20;
    pub const EVT_TIME0_CMP_EN_MASK: u32 = 0x00100000;
    pub const EVT_TIME0_CMP_EN: u32 = 1 << 20;

    /// **EVT_TIME_OVF_TIMER3_EN**
    ///
    /// Configures whether or not to enable the ledc_timer3_ovf event.\\0: Disable\\1: Enable
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EVT_TIME_OVF_TIMER3_EN_OFFSET: u32 = 19;
    pub const EVT_TIME_OVF_TIMER3_EN_MASK: u32 = 0x00080000;
    pub const EVT_TIME_OVF_TIMER3_EN: u32 = 1 << 19;

    /// **EVT_TIME_OVF_TIMER2_EN**
    ///
    /// Configures whether or not to enable the ledc_timer2_ovf event.\\0: Disable\\1: Enable
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EVT_TIME_OVF_TIMER2_EN_OFFSET: u32 = 18;
    pub const EVT_TIME_OVF_TIMER2_EN_MASK: u32 = 0x00040000;
    pub const EVT_TIME_OVF_TIMER2_EN: u32 = 1 << 18;

    /// **EVT_TIME_OVF_TIMER1_EN**
    ///
    /// Configures whether or not to enable the ledc_timer1_ovf event.\\0: Disable\\1: Enable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EVT_TIME_OVF_TIMER1_EN_OFFSET: u32 = 17;
    pub const EVT_TIME_OVF_TIMER1_EN_MASK: u32 = 0x00020000;
    pub const EVT_TIME_OVF_TIMER1_EN: u32 = 1 << 17;

    /// **EVT_TIME_OVF_TIMER0_EN**
    ///
    /// Configures whether or not to enable the ledc_timer0_ovf event.\\0: Disable\\1: Enable
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EVT_TIME_OVF_TIMER0_EN_OFFSET: u32 = 16;
    pub const EVT_TIME_OVF_TIMER0_EN_MASK: u32 = 0x00010000;
    pub const EVT_TIME_OVF_TIMER0_EN: u32 = 1 << 16;

    /// **EVT_OVF_CNT_PLS_CH7_EN**
    ///
    /// Configures whether or not to enable the ledc_ch7_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EVT_OVF_CNT_PLS_CH7_EN_OFFSET: u32 = 15;
    pub const EVT_OVF_CNT_PLS_CH7_EN_MASK: u32 = 0x00008000;
    pub const EVT_OVF_CNT_PLS_CH7_EN: u32 = 1 << 15;

    /// **EVT_OVF_CNT_PLS_CH6_EN**
    ///
    /// Configures whether or not to enable the ledc_ch6_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EVT_OVF_CNT_PLS_CH6_EN_OFFSET: u32 = 14;
    pub const EVT_OVF_CNT_PLS_CH6_EN_MASK: u32 = 0x00004000;
    pub const EVT_OVF_CNT_PLS_CH6_EN: u32 = 1 << 14;

    /// **EVT_OVF_CNT_PLS_CH5_EN**
    ///
    /// Configures whether or not to enable the ledc_ch5_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EVT_OVF_CNT_PLS_CH5_EN_OFFSET: u32 = 13;
    pub const EVT_OVF_CNT_PLS_CH5_EN_MASK: u32 = 0x00002000;
    pub const EVT_OVF_CNT_PLS_CH5_EN: u32 = 1 << 13;

    /// **EVT_OVF_CNT_PLS_CH4_EN**
    ///
    /// Configures whether or not to enable the ledc_ch4_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EVT_OVF_CNT_PLS_CH4_EN_OFFSET: u32 = 12;
    pub const EVT_OVF_CNT_PLS_CH4_EN_MASK: u32 = 0x00001000;
    pub const EVT_OVF_CNT_PLS_CH4_EN: u32 = 1 << 12;

    /// **EVT_OVF_CNT_PLS_CH3_EN**
    ///
    /// Configures whether or not to enable the ledc_ch3_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EVT_OVF_CNT_PLS_CH3_EN_OFFSET: u32 = 11;
    pub const EVT_OVF_CNT_PLS_CH3_EN_MASK: u32 = 0x00000800;
    pub const EVT_OVF_CNT_PLS_CH3_EN: u32 = 1 << 11;

    /// **EVT_OVF_CNT_PLS_CH2_EN**
    ///
    /// Configures whether or not to enable the ledc_ch2_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EVT_OVF_CNT_PLS_CH2_EN_OFFSET: u32 = 10;
    pub const EVT_OVF_CNT_PLS_CH2_EN_MASK: u32 = 0x00000400;
    pub const EVT_OVF_CNT_PLS_CH2_EN: u32 = 1 << 10;

    /// **EVT_OVF_CNT_PLS_CH1_EN**
    ///
    /// Configures whether or not to enable the ledc_ch1_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EVT_OVF_CNT_PLS_CH1_EN_OFFSET: u32 = 9;
    pub const EVT_OVF_CNT_PLS_CH1_EN_MASK: u32 = 0x00000200;
    pub const EVT_OVF_CNT_PLS_CH1_EN: u32 = 1 << 9;

    /// **EVT_OVF_CNT_PLS_CH0_EN**
    ///
    /// Configures whether or not to enable the ledc_ch0_ovf_cnt_pls event.\\0: Disable\\1: Enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EVT_OVF_CNT_PLS_CH0_EN_OFFSET: u32 = 8;
    pub const EVT_OVF_CNT_PLS_CH0_EN_MASK: u32 = 0x00000100;
    pub const EVT_OVF_CNT_PLS_CH0_EN: u32 = 1 << 8;

    /// **EVT_DUTY_CHNG_END_CH7_EN**
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EVT_DUTY_CHNG_END_CH7_EN_OFFSET: u32 = 7;
    pub const EVT_DUTY_CHNG_END_CH7_EN_MASK: u32 = 0x00000080;
    pub const EVT_DUTY_CHNG_END_CH7_EN: u32 = 1 << 7;

    /// **EVT_DUTY_CHNG_END_CH6_EN**
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EVT_DUTY_CHNG_END_CH6_EN_OFFSET: u32 = 6;
    pub const EVT_DUTY_CHNG_END_CH6_EN_MASK: u32 = 0x00000040;
    pub const EVT_DUTY_CHNG_END_CH6_EN: u32 = 1 << 6;

    /// **EVT_DUTY_CHNG_END_CH5_EN**
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EVT_DUTY_CHNG_END_CH5_EN_OFFSET: u32 = 5;
    pub const EVT_DUTY_CHNG_END_CH5_EN_MASK: u32 = 0x00000020;
    pub const EVT_DUTY_CHNG_END_CH5_EN: u32 = 1 << 5;

    /// **EVT_DUTY_CHNG_END_CH4_EN**
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EVT_DUTY_CHNG_END_CH4_EN_OFFSET: u32 = 4;
    pub const EVT_DUTY_CHNG_END_CH4_EN_MASK: u32 = 0x00000010;
    pub const EVT_DUTY_CHNG_END_CH4_EN: u32 = 1 << 4;

    /// **EVT_DUTY_CHNG_END_CH3_EN**
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EVT_DUTY_CHNG_END_CH3_EN_OFFSET: u32 = 3;
    pub const EVT_DUTY_CHNG_END_CH3_EN_MASK: u32 = 0x00000008;
    pub const EVT_DUTY_CHNG_END_CH3_EN: u32 = 1 << 3;

    /// **EVT_DUTY_CHNG_END_CH2_EN**
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EVT_DUTY_CHNG_END_CH2_EN_OFFSET: u32 = 2;
    pub const EVT_DUTY_CHNG_END_CH2_EN_MASK: u32 = 0x00000004;
    pub const EVT_DUTY_CHNG_END_CH2_EN: u32 = 1 << 2;

    /// **EVT_DUTY_CHNG_END_CH1_EN**
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EVT_DUTY_CHNG_END_CH1_EN_OFFSET: u32 = 1;
    pub const EVT_DUTY_CHNG_END_CH1_EN_MASK: u32 = 0x00000002;
    pub const EVT_DUTY_CHNG_END_CH1_EN: u32 = 1 << 1;

    /// **EVT_DUTY_CHNG_END_CH0_EN**
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_chng_end event.\\0: Disable\\1: Enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EVT_DUTY_CHNG_END_CH0_EN_OFFSET: u32 = 0;
    pub const EVT_DUTY_CHNG_END_CH0_EN_MASK: u32 = 0x00000001;
    pub const EVT_DUTY_CHNG_END_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TASK_DUTY_SCALE_UPDATE_CH7_EN** `[31]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH6_EN** `[30]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH5_EN** `[29]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH4_EN** `[28]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH3_EN** `[27]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH2_EN** `[26]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH1_EN** `[25]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK);
    }

    /// **TASK_DUTY_SCALE_UPDATE_CH0_EN** `[24]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_duty_scale_update_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK) != 0
    }

    /// Set TASK_DUTY_SCALE_UPDATE_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_duty_scale_update_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK);
    }

    /// Clear TASK_DUTY_SCALE_UPDATE_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_duty_scale_update_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK);
    }

    /// **EVT_TIME3_CMP_EN** `[23]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer3_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time3_cmp_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME3_CMP_EN_MASK) != 0
    }

    /// Set EVT_TIME3_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time3_cmp_en(&self) {
        self.value.modify_bits(|r| r | Self::EVT_TIME3_CMP_EN_MASK);
    }

    /// Clear EVT_TIME3_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time3_cmp_en(&self) {
        self.value.modify_bits(|r| r & !Self::EVT_TIME3_CMP_EN_MASK);
    }

    /// **EVT_TIME2_CMP_EN** `[22]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer2_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time2_cmp_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME2_CMP_EN_MASK) != 0
    }

    /// Set EVT_TIME2_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time2_cmp_en(&self) {
        self.value.modify_bits(|r| r | Self::EVT_TIME2_CMP_EN_MASK);
    }

    /// Clear EVT_TIME2_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time2_cmp_en(&self) {
        self.value.modify_bits(|r| r & !Self::EVT_TIME2_CMP_EN_MASK);
    }

    /// **EVT_TIME1_CMP_EN** `[21]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer1_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time1_cmp_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME1_CMP_EN_MASK) != 0
    }

    /// Set EVT_TIME1_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time1_cmp_en(&self) {
        self.value.modify_bits(|r| r | Self::EVT_TIME1_CMP_EN_MASK);
    }

    /// Clear EVT_TIME1_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time1_cmp_en(&self) {
        self.value.modify_bits(|r| r & !Self::EVT_TIME1_CMP_EN_MASK);
    }

    /// **EVT_TIME0_CMP_EN** `[20]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer0_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time0_cmp_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME0_CMP_EN_MASK) != 0
    }

    /// Set EVT_TIME0_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time0_cmp_en(&self) {
        self.value.modify_bits(|r| r | Self::EVT_TIME0_CMP_EN_MASK);
    }

    /// Clear EVT_TIME0_CMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time0_cmp_en(&self) {
        self.value.modify_bits(|r| r & !Self::EVT_TIME0_CMP_EN_MASK);
    }

    /// **EVT_TIME_OVF_TIMER3_EN** `[19]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer3_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time_ovf_timer3_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME_OVF_TIMER3_EN_MASK) != 0
    }

    /// Set EVT_TIME_OVF_TIMER3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time_ovf_timer3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_TIME_OVF_TIMER3_EN_MASK);
    }

    /// Clear EVT_TIME_OVF_TIMER3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time_ovf_timer3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_TIME_OVF_TIMER3_EN_MASK);
    }

    /// **EVT_TIME_OVF_TIMER2_EN** `[18]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer2_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time_ovf_timer2_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME_OVF_TIMER2_EN_MASK) != 0
    }

    /// Set EVT_TIME_OVF_TIMER2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time_ovf_timer2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_TIME_OVF_TIMER2_EN_MASK);
    }

    /// Clear EVT_TIME_OVF_TIMER2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time_ovf_timer2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_TIME_OVF_TIMER2_EN_MASK);
    }

    /// **EVT_TIME_OVF_TIMER1_EN** `[17]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer1_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time_ovf_timer1_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME_OVF_TIMER1_EN_MASK) != 0
    }

    /// Set EVT_TIME_OVF_TIMER1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time_ovf_timer1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_TIME_OVF_TIMER1_EN_MASK);
    }

    /// Clear EVT_TIME_OVF_TIMER1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time_ovf_timer1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_TIME_OVF_TIMER1_EN_MASK);
    }

    /// **EVT_TIME_OVF_TIMER0_EN** `[16]` (RW)
    ///
    /// Configures whether or not to enable the ledc_timer0_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_time_ovf_timer0_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_TIME_OVF_TIMER0_EN_MASK) != 0
    }

    /// Set EVT_TIME_OVF_TIMER0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_time_ovf_timer0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_TIME_OVF_TIMER0_EN_MASK);
    }

    /// Clear EVT_TIME_OVF_TIMER0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_time_ovf_timer0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_TIME_OVF_TIMER0_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH7_EN** `[15]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch7_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH7_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH7_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH7_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH6_EN** `[14]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch6_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH6_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH6_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH6_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH5_EN** `[13]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch5_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH5_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH5_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH5_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH4_EN** `[12]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch4_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH4_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH4_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH4_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH3_EN** `[11]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch3_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH3_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH3_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH3_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH2_EN** `[10]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch2_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH2_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH2_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH2_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH1_EN** `[9]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch1_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH1_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH1_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH1_EN_MASK);
    }

    /// **EVT_OVF_CNT_PLS_CH0_EN** `[8]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch0_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_OVF_CNT_PLS_CH0_EN_MASK) != 0
    }

    /// Set EVT_OVF_CNT_PLS_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_ovf_cnt_pls_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_OVF_CNT_PLS_CH0_EN_MASK);
    }

    /// Clear EVT_OVF_CNT_PLS_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_ovf_cnt_pls_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_OVF_CNT_PLS_CH0_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH7_EN** `[7]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH7_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH7_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH7_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH6_EN** `[6]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH6_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH6_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH6_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH5_EN** `[5]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH5_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH5_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH5_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH4_EN** `[4]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH4_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH4_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH4_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH3_EN** `[3]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH3_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH3_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH3_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH2_EN** `[2]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH2_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH2_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH2_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH1_EN** `[1]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH1_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH1_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH1_EN_MASK);
    }

    /// **EVT_DUTY_CHNG_END_CH0_EN** `[0]` (RW)
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn evt_duty_chng_end_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::EVT_DUTY_CHNG_END_CH0_EN_MASK) != 0
    }

    /// Set EVT_DUTY_CHNG_END_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_evt_duty_chng_end_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EVT_DUTY_CHNG_END_CH0_EN_MASK);
    }

    /// Clear EVT_DUTY_CHNG_END_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_evt_duty_chng_end_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EVT_DUTY_CHNG_END_CH0_EN_MASK);
    }
}

/// EVT_TASK_EN0 value type for batch read/modify/write operations
///
/// Use `evttasken0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `evttasken0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EvtTaskEn0Val {
    bits: u32,
}

impl EvtTaskEn0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH7_EN** `[31:31]`
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH6_EN** `[30:30]`
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH5_EN** `[29:29]`
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH4_EN** `[28:28]`
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH3_EN** `[27:27]`
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH2_EN** `[26:26]`
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH1_EN** `[25:25]`
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK) != 0
    }

    /// Read **TASK_DUTY_SCALE_UPDATE_CH0_EN** `[24:24]`
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_duty_scale_update_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK) != 0
    }

    /// Read **EVT_TIME3_CMP_EN** `[23:23]`
    ///
    /// Configures whether or not to enable the ledc_timer3_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time3_cmp_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME3_CMP_EN_MASK) != 0
    }

    /// Read **EVT_TIME2_CMP_EN** `[22:22]`
    ///
    /// Configures whether or not to enable the ledc_timer2_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time2_cmp_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME2_CMP_EN_MASK) != 0
    }

    /// Read **EVT_TIME1_CMP_EN** `[21:21]`
    ///
    /// Configures whether or not to enable the ledc_timer1_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time1_cmp_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME1_CMP_EN_MASK) != 0
    }

    /// Read **EVT_TIME0_CMP_EN** `[20:20]`
    ///
    /// Configures whether or not to enable the ledc_timer0_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time0_cmp_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME0_CMP_EN_MASK) != 0
    }

    /// Read **EVT_TIME_OVF_TIMER3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable the ledc_timer3_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time_ovf_timer3_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME_OVF_TIMER3_EN_MASK) != 0
    }

    /// Read **EVT_TIME_OVF_TIMER2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable the ledc_timer2_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time_ovf_timer2_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME_OVF_TIMER2_EN_MASK) != 0
    }

    /// Read **EVT_TIME_OVF_TIMER1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable the ledc_timer1_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time_ovf_timer1_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME_OVF_TIMER1_EN_MASK) != 0
    }

    /// Read **EVT_TIME_OVF_TIMER0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable the ledc_timer0_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_time_ovf_timer0_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_TIME_OVF_TIMER0_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable the ledc_ch7_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH7_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable the ledc_ch6_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH6_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable the ledc_ch5_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH5_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable the ledc_ch4_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH4_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable the ledc_ch3_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH3_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable the ledc_ch2_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH2_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable the ledc_ch1_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH1_EN_MASK) != 0
    }

    /// Read **EVT_OVF_CNT_PLS_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable the ledc_ch0_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_ovf_cnt_pls_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_OVF_CNT_PLS_CH0_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH7_EN** `[7:7]`
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH7_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH6_EN** `[6:6]`
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH6_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH5_EN** `[5:5]`
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH5_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH4_EN** `[4:4]`
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH4_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH3_EN** `[3:3]`
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH3_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH2_EN** `[2:2]`
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH2_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH1_EN** `[1:1]`
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH1_EN_MASK) != 0
    }

    /// Read **EVT_DUTY_CHNG_END_CH0_EN** `[0:0]`
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn evt_duty_chng_end_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn0::EVT_DUTY_CHNG_END_CH0_EN_MASK) != 0
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH7_EN** `[31:31]`
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH6_EN** `[30:30]`
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH5_EN** `[29:29]`
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH4_EN** `[28:28]`
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH3_EN** `[27:27]`
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH2_EN** `[26:26]`
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH1_EN** `[25:25]`
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_DUTY_SCALE_UPDATE_CH0_EN** `[24:24]`
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_scale_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_duty_scale_update_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::TASK_DUTY_SCALE_UPDATE_CH0_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME3_CMP_EN** `[23:23]`
    ///
    /// Configures whether or not to enable the ledc_timer3_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time3_cmp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME3_CMP_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME3_CMP_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME2_CMP_EN** `[22:22]`
    ///
    /// Configures whether or not to enable the ledc_timer2_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time2_cmp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME2_CMP_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME2_CMP_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME1_CMP_EN** `[21:21]`
    ///
    /// Configures whether or not to enable the ledc_timer1_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time1_cmp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME1_CMP_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME1_CMP_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME0_CMP_EN** `[20:20]`
    ///
    /// Configures whether or not to enable the ledc_timer0_cmp event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time0_cmp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME0_CMP_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME0_CMP_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME_OVF_TIMER3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable the ledc_timer3_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time_ovf_timer3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME_OVF_TIMER3_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME_OVF_TIMER3_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME_OVF_TIMER2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable the ledc_timer2_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time_ovf_timer2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME_OVF_TIMER2_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME_OVF_TIMER2_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME_OVF_TIMER1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable the ledc_timer1_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time_ovf_timer1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME_OVF_TIMER1_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME_OVF_TIMER1_EN_MASK
            },
        }
    }

    /// Set **EVT_TIME_OVF_TIMER0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable the ledc_timer0_ovf event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_time_ovf_timer0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_TIME_OVF_TIMER0_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_TIME_OVF_TIMER0_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable the ledc_ch7_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH7_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable the ledc_ch6_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH6_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable the ledc_ch5_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH5_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable the ledc_ch4_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH4_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable the ledc_ch3_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH3_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable the ledc_ch2_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH2_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable the ledc_ch1_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH1_EN_MASK
            },
        }
    }

    /// Set **EVT_OVF_CNT_PLS_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable the ledc_ch0_ovf_cnt_pls event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_ovf_cnt_pls_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_OVF_CNT_PLS_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_OVF_CNT_PLS_CH0_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH7_EN** `[7:7]`
    ///
    /// Configures whether or not to enable the ledc_ch7_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH7_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH6_EN** `[6:6]`
    ///
    /// Configures whether or not to enable the ledc_ch6_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH6_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH5_EN** `[5:5]`
    ///
    /// Configures whether or not to enable the ledc_ch5_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH5_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH4_EN** `[4:4]`
    ///
    /// Configures whether or not to enable the ledc_ch4_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH4_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH3_EN** `[3:3]`
    ///
    /// Configures whether or not to enable the ledc_ch3_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH3_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH2_EN** `[2:2]`
    ///
    /// Configures whether or not to enable the ledc_ch2_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH2_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH1_EN** `[1:1]`
    ///
    /// Configures whether or not to enable the ledc_ch1_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH1_EN_MASK
            },
        }
    }

    /// Set **EVT_DUTY_CHNG_END_CH0_EN** `[0:0]`
    ///
    /// Configures whether or not to enable the ledc_ch0_duty_chng_end event.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_evt_duty_chng_end_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn0::EVT_DUTY_CHNG_END_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn0::EVT_DUTY_CHNG_END_CH0_EN_MASK
            },
        }
    }
}

impl Default for EvtTaskEn0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EvtTaskEn0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EvtTaskEn0Val {
        EvtTaskEn0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EvtTaskEn0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EvtTaskEn0Val) -> EvtTaskEn0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EVT_TASK_EN1 Register
///
/// Ledc event task enable bit register1.
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **TASK_TIMER3_PAUSE_RESUME_EN** `[31]` (RW)
///
///   Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER2_PAUSE_RESUME_EN** `[30]` (RW)
///
///   Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER1_PAUSE_RESUME_EN** `[29]` (RW)
///
///   Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER0_PAUSE_RESUME_EN** `[28]` (RW)
///
///   Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER3_RST_EN** `[27]` (RW)
///
///   Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER2_RST_EN** `[26]` (RW)
///
///   Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER1_RST_EN** `[25]` (RW)
///
///   Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER0_RST_EN** `[24]` (RW)
///
///   Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH7_EN** `[23]` (RW)
///
///   Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH6_EN** `[22]` (RW)
///
///   Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH5_EN** `[21]` (RW)
///
///   Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH4_EN** `[20]` (RW)
///
///   Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH3_EN** `[19]` (RW)
///
///   Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH2_EN** `[18]` (RW)
///
///   Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH1_EN** `[17]` (RW)
///
///   Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_OVF_CNT_RST_CH0_EN** `[16]` (RW)
///
///   Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH7_EN** `[15]` (RW)
///
///   Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH6_EN** `[14]` (RW)
///
///   Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH5_EN** `[13]` (RW)
///
///   Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH4_EN** `[12]` (RW)
///
///   Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH3_EN** `[11]` (RW)
///
///   Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH2_EN** `[10]` (RW)
///
///   Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH1_EN** `[9]` (RW)
///
///   Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_SIG_OUT_DIS_CH0_EN** `[8]` (RW)
///
///   Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER3_CAP_EN** `[7]` (RW)
///
///   Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER2_CAP_EN** `[6]` (RW)
///
///   Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER1_CAP_EN** `[5]` (RW)
///
///   Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER0_CAP_EN** `[4]` (RW)
///
///   Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER3_RES_UPDATE_EN** `[3]` (RW)
///
///   Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER2_RES_UPDATE_EN** `[2]` (RW)
///
///   Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER1_RES_UPDATE_EN** `[1]` (RW)
///
///   Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
///
/// - **TASK_TIMER0_RES_UPDATE_EN** `[0]` (RW)
///
///   Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
#[repr(transparent)]
pub struct EvtTaskEn1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EvtTaskEn1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EvtTaskEn1 {
    // ======== Field Constants ========

    /// **TASK_TIMER3_PAUSE_RESUME_EN**
    ///
    /// Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TASK_TIMER3_PAUSE_RESUME_EN_OFFSET: u32 = 31;
    pub const TASK_TIMER3_PAUSE_RESUME_EN_MASK: u32 = 0x80000000;
    pub const TASK_TIMER3_PAUSE_RESUME_EN: u32 = 1 << 31;

    /// **TASK_TIMER2_PAUSE_RESUME_EN**
    ///
    /// Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TASK_TIMER2_PAUSE_RESUME_EN_OFFSET: u32 = 30;
    pub const TASK_TIMER2_PAUSE_RESUME_EN_MASK: u32 = 0x40000000;
    pub const TASK_TIMER2_PAUSE_RESUME_EN: u32 = 1 << 30;

    /// **TASK_TIMER1_PAUSE_RESUME_EN**
    ///
    /// Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TASK_TIMER1_PAUSE_RESUME_EN_OFFSET: u32 = 29;
    pub const TASK_TIMER1_PAUSE_RESUME_EN_MASK: u32 = 0x20000000;
    pub const TASK_TIMER1_PAUSE_RESUME_EN: u32 = 1 << 29;

    /// **TASK_TIMER0_PAUSE_RESUME_EN**
    ///
    /// Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TASK_TIMER0_PAUSE_RESUME_EN_OFFSET: u32 = 28;
    pub const TASK_TIMER0_PAUSE_RESUME_EN_MASK: u32 = 0x10000000;
    pub const TASK_TIMER0_PAUSE_RESUME_EN: u32 = 1 << 28;

    /// **TASK_TIMER3_RST_EN**
    ///
    /// Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TASK_TIMER3_RST_EN_OFFSET: u32 = 27;
    pub const TASK_TIMER3_RST_EN_MASK: u32 = 0x08000000;
    pub const TASK_TIMER3_RST_EN: u32 = 1 << 27;

    /// **TASK_TIMER2_RST_EN**
    ///
    /// Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TASK_TIMER2_RST_EN_OFFSET: u32 = 26;
    pub const TASK_TIMER2_RST_EN_MASK: u32 = 0x04000000;
    pub const TASK_TIMER2_RST_EN: u32 = 1 << 26;

    /// **TASK_TIMER1_RST_EN**
    ///
    /// Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TASK_TIMER1_RST_EN_OFFSET: u32 = 25;
    pub const TASK_TIMER1_RST_EN_MASK: u32 = 0x02000000;
    pub const TASK_TIMER1_RST_EN: u32 = 1 << 25;

    /// **TASK_TIMER0_RST_EN**
    ///
    /// Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TASK_TIMER0_RST_EN_OFFSET: u32 = 24;
    pub const TASK_TIMER0_RST_EN_MASK: u32 = 0x01000000;
    pub const TASK_TIMER0_RST_EN: u32 = 1 << 24;

    /// **TASK_OVF_CNT_RST_CH7_EN**
    ///
    /// Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TASK_OVF_CNT_RST_CH7_EN_OFFSET: u32 = 23;
    pub const TASK_OVF_CNT_RST_CH7_EN_MASK: u32 = 0x00800000;
    pub const TASK_OVF_CNT_RST_CH7_EN: u32 = 1 << 23;

    /// **TASK_OVF_CNT_RST_CH6_EN**
    ///
    /// Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TASK_OVF_CNT_RST_CH6_EN_OFFSET: u32 = 22;
    pub const TASK_OVF_CNT_RST_CH6_EN_MASK: u32 = 0x00400000;
    pub const TASK_OVF_CNT_RST_CH6_EN: u32 = 1 << 22;

    /// **TASK_OVF_CNT_RST_CH5_EN**
    ///
    /// Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TASK_OVF_CNT_RST_CH5_EN_OFFSET: u32 = 21;
    pub const TASK_OVF_CNT_RST_CH5_EN_MASK: u32 = 0x00200000;
    pub const TASK_OVF_CNT_RST_CH5_EN: u32 = 1 << 21;

    /// **TASK_OVF_CNT_RST_CH4_EN**
    ///
    /// Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TASK_OVF_CNT_RST_CH4_EN_OFFSET: u32 = 20;
    pub const TASK_OVF_CNT_RST_CH4_EN_MASK: u32 = 0x00100000;
    pub const TASK_OVF_CNT_RST_CH4_EN: u32 = 1 << 20;

    /// **TASK_OVF_CNT_RST_CH3_EN**
    ///
    /// Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TASK_OVF_CNT_RST_CH3_EN_OFFSET: u32 = 19;
    pub const TASK_OVF_CNT_RST_CH3_EN_MASK: u32 = 0x00080000;
    pub const TASK_OVF_CNT_RST_CH3_EN: u32 = 1 << 19;

    /// **TASK_OVF_CNT_RST_CH2_EN**
    ///
    /// Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TASK_OVF_CNT_RST_CH2_EN_OFFSET: u32 = 18;
    pub const TASK_OVF_CNT_RST_CH2_EN_MASK: u32 = 0x00040000;
    pub const TASK_OVF_CNT_RST_CH2_EN: u32 = 1 << 18;

    /// **TASK_OVF_CNT_RST_CH1_EN**
    ///
    /// Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TASK_OVF_CNT_RST_CH1_EN_OFFSET: u32 = 17;
    pub const TASK_OVF_CNT_RST_CH1_EN_MASK: u32 = 0x00020000;
    pub const TASK_OVF_CNT_RST_CH1_EN: u32 = 1 << 17;

    /// **TASK_OVF_CNT_RST_CH0_EN**
    ///
    /// Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TASK_OVF_CNT_RST_CH0_EN_OFFSET: u32 = 16;
    pub const TASK_OVF_CNT_RST_CH0_EN_MASK: u32 = 0x00010000;
    pub const TASK_OVF_CNT_RST_CH0_EN: u32 = 1 << 16;

    /// **TASK_SIG_OUT_DIS_CH7_EN**
    ///
    /// Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TASK_SIG_OUT_DIS_CH7_EN_OFFSET: u32 = 15;
    pub const TASK_SIG_OUT_DIS_CH7_EN_MASK: u32 = 0x00008000;
    pub const TASK_SIG_OUT_DIS_CH7_EN: u32 = 1 << 15;

    /// **TASK_SIG_OUT_DIS_CH6_EN**
    ///
    /// Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TASK_SIG_OUT_DIS_CH6_EN_OFFSET: u32 = 14;
    pub const TASK_SIG_OUT_DIS_CH6_EN_MASK: u32 = 0x00004000;
    pub const TASK_SIG_OUT_DIS_CH6_EN: u32 = 1 << 14;

    /// **TASK_SIG_OUT_DIS_CH5_EN**
    ///
    /// Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TASK_SIG_OUT_DIS_CH5_EN_OFFSET: u32 = 13;
    pub const TASK_SIG_OUT_DIS_CH5_EN_MASK: u32 = 0x00002000;
    pub const TASK_SIG_OUT_DIS_CH5_EN: u32 = 1 << 13;

    /// **TASK_SIG_OUT_DIS_CH4_EN**
    ///
    /// Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TASK_SIG_OUT_DIS_CH4_EN_OFFSET: u32 = 12;
    pub const TASK_SIG_OUT_DIS_CH4_EN_MASK: u32 = 0x00001000;
    pub const TASK_SIG_OUT_DIS_CH4_EN: u32 = 1 << 12;

    /// **TASK_SIG_OUT_DIS_CH3_EN**
    ///
    /// Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TASK_SIG_OUT_DIS_CH3_EN_OFFSET: u32 = 11;
    pub const TASK_SIG_OUT_DIS_CH3_EN_MASK: u32 = 0x00000800;
    pub const TASK_SIG_OUT_DIS_CH3_EN: u32 = 1 << 11;

    /// **TASK_SIG_OUT_DIS_CH2_EN**
    ///
    /// Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TASK_SIG_OUT_DIS_CH2_EN_OFFSET: u32 = 10;
    pub const TASK_SIG_OUT_DIS_CH2_EN_MASK: u32 = 0x00000400;
    pub const TASK_SIG_OUT_DIS_CH2_EN: u32 = 1 << 10;

    /// **TASK_SIG_OUT_DIS_CH1_EN**
    ///
    /// Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TASK_SIG_OUT_DIS_CH1_EN_OFFSET: u32 = 9;
    pub const TASK_SIG_OUT_DIS_CH1_EN_MASK: u32 = 0x00000200;
    pub const TASK_SIG_OUT_DIS_CH1_EN: u32 = 1 << 9;

    /// **TASK_SIG_OUT_DIS_CH0_EN**
    ///
    /// Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TASK_SIG_OUT_DIS_CH0_EN_OFFSET: u32 = 8;
    pub const TASK_SIG_OUT_DIS_CH0_EN_MASK: u32 = 0x00000100;
    pub const TASK_SIG_OUT_DIS_CH0_EN: u32 = 1 << 8;

    /// **TASK_TIMER3_CAP_EN**
    ///
    /// Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TASK_TIMER3_CAP_EN_OFFSET: u32 = 7;
    pub const TASK_TIMER3_CAP_EN_MASK: u32 = 0x00000080;
    pub const TASK_TIMER3_CAP_EN: u32 = 1 << 7;

    /// **TASK_TIMER2_CAP_EN**
    ///
    /// Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TASK_TIMER2_CAP_EN_OFFSET: u32 = 6;
    pub const TASK_TIMER2_CAP_EN_MASK: u32 = 0x00000040;
    pub const TASK_TIMER2_CAP_EN: u32 = 1 << 6;

    /// **TASK_TIMER1_CAP_EN**
    ///
    /// Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TASK_TIMER1_CAP_EN_OFFSET: u32 = 5;
    pub const TASK_TIMER1_CAP_EN_MASK: u32 = 0x00000020;
    pub const TASK_TIMER1_CAP_EN: u32 = 1 << 5;

    /// **TASK_TIMER0_CAP_EN**
    ///
    /// Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TASK_TIMER0_CAP_EN_OFFSET: u32 = 4;
    pub const TASK_TIMER0_CAP_EN_MASK: u32 = 0x00000010;
    pub const TASK_TIMER0_CAP_EN: u32 = 1 << 4;

    /// **TASK_TIMER3_RES_UPDATE_EN**
    ///
    /// Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TASK_TIMER3_RES_UPDATE_EN_OFFSET: u32 = 3;
    pub const TASK_TIMER3_RES_UPDATE_EN_MASK: u32 = 0x00000008;
    pub const TASK_TIMER3_RES_UPDATE_EN: u32 = 1 << 3;

    /// **TASK_TIMER2_RES_UPDATE_EN**
    ///
    /// Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TASK_TIMER2_RES_UPDATE_EN_OFFSET: u32 = 2;
    pub const TASK_TIMER2_RES_UPDATE_EN_MASK: u32 = 0x00000004;
    pub const TASK_TIMER2_RES_UPDATE_EN: u32 = 1 << 2;

    /// **TASK_TIMER1_RES_UPDATE_EN**
    ///
    /// Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TASK_TIMER1_RES_UPDATE_EN_OFFSET: u32 = 1;
    pub const TASK_TIMER1_RES_UPDATE_EN_MASK: u32 = 0x00000002;
    pub const TASK_TIMER1_RES_UPDATE_EN: u32 = 1 << 1;

    /// **TASK_TIMER0_RES_UPDATE_EN**
    ///
    /// Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TASK_TIMER0_RES_UPDATE_EN_OFFSET: u32 = 0;
    pub const TASK_TIMER0_RES_UPDATE_EN_MASK: u32 = 0x00000001;
    pub const TASK_TIMER0_RES_UPDATE_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TASK_TIMER3_PAUSE_RESUME_EN** `[31]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_pause_resume_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER3_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Set TASK_TIMER3_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer3_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER3_PAUSE_RESUME_EN_MASK);
    }

    /// Clear TASK_TIMER3_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer3_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER3_PAUSE_RESUME_EN_MASK);
    }

    /// **TASK_TIMER2_PAUSE_RESUME_EN** `[30]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_pause_resume_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER2_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Set TASK_TIMER2_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer2_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER2_PAUSE_RESUME_EN_MASK);
    }

    /// Clear TASK_TIMER2_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer2_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER2_PAUSE_RESUME_EN_MASK);
    }

    /// **TASK_TIMER1_PAUSE_RESUME_EN** `[29]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_pause_resume_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER1_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Set TASK_TIMER1_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer1_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER1_PAUSE_RESUME_EN_MASK);
    }

    /// Clear TASK_TIMER1_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer1_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER1_PAUSE_RESUME_EN_MASK);
    }

    /// **TASK_TIMER0_PAUSE_RESUME_EN** `[28]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_pause_resume_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER0_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Set TASK_TIMER0_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer0_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER0_PAUSE_RESUME_EN_MASK);
    }

    /// Clear TASK_TIMER0_PAUSE_RESUME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer0_pause_resume_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER0_PAUSE_RESUME_EN_MASK);
    }

    /// **TASK_TIMER3_RST_EN** `[27]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_rst_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER3_RST_EN_MASK) != 0
    }

    /// Set TASK_TIMER3_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer3_rst_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER3_RST_EN_MASK);
    }

    /// Clear TASK_TIMER3_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer3_rst_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER3_RST_EN_MASK);
    }

    /// **TASK_TIMER2_RST_EN** `[26]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_rst_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER2_RST_EN_MASK) != 0
    }

    /// Set TASK_TIMER2_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer2_rst_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER2_RST_EN_MASK);
    }

    /// Clear TASK_TIMER2_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer2_rst_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER2_RST_EN_MASK);
    }

    /// **TASK_TIMER1_RST_EN** `[25]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_rst_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER1_RST_EN_MASK) != 0
    }

    /// Set TASK_TIMER1_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer1_rst_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER1_RST_EN_MASK);
    }

    /// Clear TASK_TIMER1_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer1_rst_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER1_RST_EN_MASK);
    }

    /// **TASK_TIMER0_RST_EN** `[24]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_rst_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER0_RST_EN_MASK) != 0
    }

    /// Set TASK_TIMER0_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer0_rst_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER0_RST_EN_MASK);
    }

    /// Clear TASK_TIMER0_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer0_rst_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER0_RST_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH7_EN** `[23]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH7_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH7_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH7_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH6_EN** `[22]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH6_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH6_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH6_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH5_EN** `[21]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH5_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH5_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH5_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH4_EN** `[20]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH4_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH4_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH4_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH3_EN** `[19]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH3_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH3_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH3_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH2_EN** `[18]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH2_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH2_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH2_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH1_EN** `[17]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH1_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH1_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH1_EN_MASK);
    }

    /// **TASK_OVF_CNT_RST_CH0_EN** `[16]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_OVF_CNT_RST_CH0_EN_MASK) != 0
    }

    /// Set TASK_OVF_CNT_RST_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_ovf_cnt_rst_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_OVF_CNT_RST_CH0_EN_MASK);
    }

    /// Clear TASK_OVF_CNT_RST_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_ovf_cnt_rst_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_OVF_CNT_RST_CH0_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH7_EN** `[15]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH7_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH7_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH7_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH6_EN** `[14]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH6_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH6_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH6_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH5_EN** `[13]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH5_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH5_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH5_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH4_EN** `[12]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH4_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH4_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH4_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH3_EN** `[11]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH3_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH3_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH3_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH2_EN** `[10]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH2_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH2_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH2_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH1_EN** `[9]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH1_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH1_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH1_EN_MASK);
    }

    /// **TASK_SIG_OUT_DIS_CH0_EN** `[8]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_SIG_OUT_DIS_CH0_EN_MASK) != 0
    }

    /// Set TASK_SIG_OUT_DIS_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_sig_out_dis_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_SIG_OUT_DIS_CH0_EN_MASK);
    }

    /// Clear TASK_SIG_OUT_DIS_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_sig_out_dis_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_SIG_OUT_DIS_CH0_EN_MASK);
    }

    /// **TASK_TIMER3_CAP_EN** `[7]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_cap_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER3_CAP_EN_MASK) != 0
    }

    /// Set TASK_TIMER3_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer3_cap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER3_CAP_EN_MASK);
    }

    /// Clear TASK_TIMER3_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer3_cap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER3_CAP_EN_MASK);
    }

    /// **TASK_TIMER2_CAP_EN** `[6]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_cap_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER2_CAP_EN_MASK) != 0
    }

    /// Set TASK_TIMER2_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer2_cap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER2_CAP_EN_MASK);
    }

    /// Clear TASK_TIMER2_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer2_cap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER2_CAP_EN_MASK);
    }

    /// **TASK_TIMER1_CAP_EN** `[5]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_cap_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER1_CAP_EN_MASK) != 0
    }

    /// Set TASK_TIMER1_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer1_cap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER1_CAP_EN_MASK);
    }

    /// Clear TASK_TIMER1_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer1_cap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER1_CAP_EN_MASK);
    }

    /// **TASK_TIMER0_CAP_EN** `[4]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_cap_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER0_CAP_EN_MASK) != 0
    }

    /// Set TASK_TIMER0_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer0_cap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER0_CAP_EN_MASK);
    }

    /// Clear TASK_TIMER0_CAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer0_cap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER0_CAP_EN_MASK);
    }

    /// **TASK_TIMER3_RES_UPDATE_EN** `[3]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_res_update_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER3_RES_UPDATE_EN_MASK) != 0
    }

    /// Set TASK_TIMER3_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer3_res_update_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER3_RES_UPDATE_EN_MASK);
    }

    /// Clear TASK_TIMER3_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer3_res_update_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER3_RES_UPDATE_EN_MASK);
    }

    /// **TASK_TIMER2_RES_UPDATE_EN** `[2]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_res_update_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER2_RES_UPDATE_EN_MASK) != 0
    }

    /// Set TASK_TIMER2_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer2_res_update_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER2_RES_UPDATE_EN_MASK);
    }

    /// Clear TASK_TIMER2_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer2_res_update_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER2_RES_UPDATE_EN_MASK);
    }

    /// **TASK_TIMER1_RES_UPDATE_EN** `[1]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_res_update_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER1_RES_UPDATE_EN_MASK) != 0
    }

    /// Set TASK_TIMER1_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer1_res_update_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER1_RES_UPDATE_EN_MASK);
    }

    /// Clear TASK_TIMER1_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer1_res_update_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER1_RES_UPDATE_EN_MASK);
    }

    /// **TASK_TIMER0_RES_UPDATE_EN** `[0]` (RW)
    ///
    /// Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_res_update_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_TIMER0_RES_UPDATE_EN_MASK) != 0
    }

    /// Set TASK_TIMER0_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_timer0_res_update_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_TIMER0_RES_UPDATE_EN_MASK);
    }

    /// Clear TASK_TIMER0_RES_UPDATE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_timer0_res_update_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_TIMER0_RES_UPDATE_EN_MASK);
    }
}

/// EVT_TASK_EN1 value type for batch read/modify/write operations
///
/// Use `evttasken1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `evttasken1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EvtTaskEn1Val {
    bits: u32,
}

impl EvtTaskEn1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TASK_TIMER3_PAUSE_RESUME_EN** `[31:31]`
    ///
    /// Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer3_pause_resume_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER3_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Read **TASK_TIMER2_PAUSE_RESUME_EN** `[30:30]`
    ///
    /// Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer2_pause_resume_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER2_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Read **TASK_TIMER1_PAUSE_RESUME_EN** `[29:29]`
    ///
    /// Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer1_pause_resume_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER1_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Read **TASK_TIMER0_PAUSE_RESUME_EN** `[28:28]`
    ///
    /// Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer0_pause_resume_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER0_PAUSE_RESUME_EN_MASK) != 0
    }

    /// Read **TASK_TIMER3_RST_EN** `[27:27]`
    ///
    /// Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer3_rst_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER3_RST_EN_MASK) != 0
    }

    /// Read **TASK_TIMER2_RST_EN** `[26:26]`
    ///
    /// Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer2_rst_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER2_RST_EN_MASK) != 0
    }

    /// Read **TASK_TIMER1_RST_EN** `[25:25]`
    ///
    /// Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer1_rst_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER1_RST_EN_MASK) != 0
    }

    /// Read **TASK_TIMER0_RST_EN** `[24:24]`
    ///
    /// Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer0_rst_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER0_RST_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH7_EN** `[23:23]`
    ///
    /// Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH7_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH6_EN** `[22:22]`
    ///
    /// Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH6_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH5_EN** `[21:21]`
    ///
    /// Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH5_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH4_EN** `[20:20]`
    ///
    /// Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH4_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH3_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH2_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH1_EN_MASK) != 0
    }

    /// Read **TASK_OVF_CNT_RST_CH0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_ovf_cnt_rst_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_OVF_CNT_RST_CH0_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH7_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH6_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH5_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH4_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH3_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH2_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH1_EN_MASK) != 0
    }

    /// Read **TASK_SIG_OUT_DIS_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_sig_out_dis_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_SIG_OUT_DIS_CH0_EN_MASK) != 0
    }

    /// Read **TASK_TIMER3_CAP_EN** `[7:7]`
    ///
    /// Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer3_cap_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER3_CAP_EN_MASK) != 0
    }

    /// Read **TASK_TIMER2_CAP_EN** `[6:6]`
    ///
    /// Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer2_cap_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER2_CAP_EN_MASK) != 0
    }

    /// Read **TASK_TIMER1_CAP_EN** `[5:5]`
    ///
    /// Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer1_cap_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER1_CAP_EN_MASK) != 0
    }

    /// Read **TASK_TIMER0_CAP_EN** `[4:4]`
    ///
    /// Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer0_cap_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER0_CAP_EN_MASK) != 0
    }

    /// Read **TASK_TIMER3_RES_UPDATE_EN** `[3:3]`
    ///
    /// Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer3_res_update_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER3_RES_UPDATE_EN_MASK) != 0
    }

    /// Read **TASK_TIMER2_RES_UPDATE_EN** `[2:2]`
    ///
    /// Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer2_res_update_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER2_RES_UPDATE_EN_MASK) != 0
    }

    /// Read **TASK_TIMER1_RES_UPDATE_EN** `[1:1]`
    ///
    /// Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer1_res_update_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER1_RES_UPDATE_EN_MASK) != 0
    }

    /// Read **TASK_TIMER0_RES_UPDATE_EN** `[0:0]`
    ///
    /// Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_timer0_res_update_en(&self) -> bool {
        (self.bits & EvtTaskEn1::TASK_TIMER0_RES_UPDATE_EN_MASK) != 0
    }

    /// Set **TASK_TIMER3_PAUSE_RESUME_EN** `[31:31]`
    ///
    /// Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer3_pause_resume_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER3_PAUSE_RESUME_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER3_PAUSE_RESUME_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER2_PAUSE_RESUME_EN** `[30:30]`
    ///
    /// Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer2_pause_resume_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER2_PAUSE_RESUME_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER2_PAUSE_RESUME_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER1_PAUSE_RESUME_EN** `[29:29]`
    ///
    /// Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer1_pause_resume_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER1_PAUSE_RESUME_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER1_PAUSE_RESUME_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER0_PAUSE_RESUME_EN** `[28:28]`
    ///
    /// Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer0_pause_resume_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER0_PAUSE_RESUME_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER0_PAUSE_RESUME_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER3_RST_EN** `[27:27]`
    ///
    /// Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer3_rst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER3_RST_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER3_RST_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER2_RST_EN** `[26:26]`
    ///
    /// Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer2_rst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER2_RST_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER2_RST_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER1_RST_EN** `[25:25]`
    ///
    /// Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer1_rst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER1_RST_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER1_RST_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER0_RST_EN** `[24:24]`
    ///
    /// Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer0_rst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER0_RST_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER0_RST_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH7_EN** `[23:23]`
    ///
    /// Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH6_EN** `[22:22]`
    ///
    /// Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH5_EN** `[21:21]`
    ///
    /// Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH4_EN** `[20:20]`
    ///
    /// Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_OVF_CNT_RST_CH0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_ovf_cnt_rst_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_OVF_CNT_RST_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_OVF_CNT_RST_CH0_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_SIG_OUT_DIS_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_sig_out_dis_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_SIG_OUT_DIS_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_SIG_OUT_DIS_CH0_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER3_CAP_EN** `[7:7]`
    ///
    /// Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer3_cap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER3_CAP_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER3_CAP_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER2_CAP_EN** `[6:6]`
    ///
    /// Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer2_cap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER2_CAP_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER2_CAP_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER1_CAP_EN** `[5:5]`
    ///
    /// Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer1_cap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER1_CAP_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER1_CAP_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER0_CAP_EN** `[4:4]`
    ///
    /// Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer0_cap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER0_CAP_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER0_CAP_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER3_RES_UPDATE_EN** `[3:3]`
    ///
    /// Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer3_res_update_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER3_RES_UPDATE_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER3_RES_UPDATE_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER2_RES_UPDATE_EN** `[2:2]`
    ///
    /// Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer2_res_update_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER2_RES_UPDATE_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER2_RES_UPDATE_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER1_RES_UPDATE_EN** `[1:1]`
    ///
    /// Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer1_res_update_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER1_RES_UPDATE_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER1_RES_UPDATE_EN_MASK
            },
        }
    }

    /// Set **TASK_TIMER0_RES_UPDATE_EN** `[0:0]`
    ///
    /// Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_timer0_res_update_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn1::TASK_TIMER0_RES_UPDATE_EN_MASK
            } else {
                self.bits & !EvtTaskEn1::TASK_TIMER0_RES_UPDATE_EN_MASK
            },
        }
    }
}

impl Default for EvtTaskEn1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EvtTaskEn1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EvtTaskEn1Val {
        EvtTaskEn1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EvtTaskEn1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EvtTaskEn1Val) -> EvtTaskEn1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EVT_TASK_EN2 Register
///
/// Ledc event task enable bit register2.
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **TASK_GAMMA_RESUME_CH7_EN** `[23]` (RW)
///
///   Configures whether or not to enable ledc_ch7_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH6_EN** `[22]` (RW)
///
///   Configures whether or not to enable ledc_ch6_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH5_EN** `[21]` (RW)
///
///   Configures whether or not to enable ledc_ch5_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH4_EN** `[20]` (RW)
///
///   Configures whether or not to enable ledc_ch4_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH3_EN** `[19]` (RW)
///
///   Configures whether or not to enable ledc_ch3_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH2_EN** `[18]` (RW)
///
///   Configures whether or not to enable ledc_ch2_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH1_EN** `[17]` (RW)
///
///   Configures whether or not to enable ledc_ch1_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESUME_CH0_EN** `[16]` (RW)
///
///   Configures whether or not to enable ledc_ch0_gamma_resume task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH7_EN** `[15]` (RW)
///
///   Configures whether or not to enable ledc_ch7_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH6_EN** `[14]` (RW)
///
///   Configures whether or not to enable ledc_ch6_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH5_EN** `[13]` (RW)
///
///   Configures whether or not to enable ledc_ch5_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH4_EN** `[12]` (RW)
///
///   Configures whether or not to enable ledc_ch4_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH3_EN** `[11]` (RW)
///
///   Configures whether or not to enable ledc_ch3_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH2_EN** `[10]` (RW)
///
///   Configures whether or not to enable ledc_ch2_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH1_EN** `[9]` (RW)
///
///   Configures whether or not to enable ledc_ch1_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_PAUSE_CH0_EN** `[8]` (RW)
///
///   Configures whether or not to enable ledc_ch0_gamma_pause task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH7_EN** `[7]` (RW)
///
///   Configures whether or not to enable ledc_ch7_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH6_EN** `[6]` (RW)
///
///   Configures whether or not to enable ledc_ch6_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH5_EN** `[5]` (RW)
///
///   Configures whether or not to enable ledc_ch5_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH4_EN** `[4]` (RW)
///
///   Configures whether or not to enable ledc_ch4_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH3_EN** `[3]` (RW)
///
///   Configures whether or not to enable ledc_ch3_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH2_EN** `[2]` (RW)
///
///   Configures whether or not to enable ledc_ch2_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH1_EN** `[1]` (RW)
///
///   Configures whether or not to enable ledc_ch1_gamma_restart task.\\0: Disable\\1: Enable
///
/// - **TASK_GAMMA_RESTART_CH0_EN** `[0]` (RW)
///
///   Configures whether or not to enable ledc_ch0_gamma_restart task.\\0: Disable\\1: Enable
#[repr(transparent)]
pub struct EvtTaskEn2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EvtTaskEn2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EvtTaskEn2 {
    // ======== Field Constants ========

    /// **TASK_GAMMA_RESUME_CH7_EN**
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TASK_GAMMA_RESUME_CH7_EN_OFFSET: u32 = 23;
    pub const TASK_GAMMA_RESUME_CH7_EN_MASK: u32 = 0x00800000;
    pub const TASK_GAMMA_RESUME_CH7_EN: u32 = 1 << 23;

    /// **TASK_GAMMA_RESUME_CH6_EN**
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TASK_GAMMA_RESUME_CH6_EN_OFFSET: u32 = 22;
    pub const TASK_GAMMA_RESUME_CH6_EN_MASK: u32 = 0x00400000;
    pub const TASK_GAMMA_RESUME_CH6_EN: u32 = 1 << 22;

    /// **TASK_GAMMA_RESUME_CH5_EN**
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TASK_GAMMA_RESUME_CH5_EN_OFFSET: u32 = 21;
    pub const TASK_GAMMA_RESUME_CH5_EN_MASK: u32 = 0x00200000;
    pub const TASK_GAMMA_RESUME_CH5_EN: u32 = 1 << 21;

    /// **TASK_GAMMA_RESUME_CH4_EN**
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TASK_GAMMA_RESUME_CH4_EN_OFFSET: u32 = 20;
    pub const TASK_GAMMA_RESUME_CH4_EN_MASK: u32 = 0x00100000;
    pub const TASK_GAMMA_RESUME_CH4_EN: u32 = 1 << 20;

    /// **TASK_GAMMA_RESUME_CH3_EN**
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TASK_GAMMA_RESUME_CH3_EN_OFFSET: u32 = 19;
    pub const TASK_GAMMA_RESUME_CH3_EN_MASK: u32 = 0x00080000;
    pub const TASK_GAMMA_RESUME_CH3_EN: u32 = 1 << 19;

    /// **TASK_GAMMA_RESUME_CH2_EN**
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TASK_GAMMA_RESUME_CH2_EN_OFFSET: u32 = 18;
    pub const TASK_GAMMA_RESUME_CH2_EN_MASK: u32 = 0x00040000;
    pub const TASK_GAMMA_RESUME_CH2_EN: u32 = 1 << 18;

    /// **TASK_GAMMA_RESUME_CH1_EN**
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TASK_GAMMA_RESUME_CH1_EN_OFFSET: u32 = 17;
    pub const TASK_GAMMA_RESUME_CH1_EN_MASK: u32 = 0x00020000;
    pub const TASK_GAMMA_RESUME_CH1_EN: u32 = 1 << 17;

    /// **TASK_GAMMA_RESUME_CH0_EN**
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_resume task.\\0: Disable\\1: Enable
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TASK_GAMMA_RESUME_CH0_EN_OFFSET: u32 = 16;
    pub const TASK_GAMMA_RESUME_CH0_EN_MASK: u32 = 0x00010000;
    pub const TASK_GAMMA_RESUME_CH0_EN: u32 = 1 << 16;

    /// **TASK_GAMMA_PAUSE_CH7_EN**
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TASK_GAMMA_PAUSE_CH7_EN_OFFSET: u32 = 15;
    pub const TASK_GAMMA_PAUSE_CH7_EN_MASK: u32 = 0x00008000;
    pub const TASK_GAMMA_PAUSE_CH7_EN: u32 = 1 << 15;

    /// **TASK_GAMMA_PAUSE_CH6_EN**
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TASK_GAMMA_PAUSE_CH6_EN_OFFSET: u32 = 14;
    pub const TASK_GAMMA_PAUSE_CH6_EN_MASK: u32 = 0x00004000;
    pub const TASK_GAMMA_PAUSE_CH6_EN: u32 = 1 << 14;

    /// **TASK_GAMMA_PAUSE_CH5_EN**
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TASK_GAMMA_PAUSE_CH5_EN_OFFSET: u32 = 13;
    pub const TASK_GAMMA_PAUSE_CH5_EN_MASK: u32 = 0x00002000;
    pub const TASK_GAMMA_PAUSE_CH5_EN: u32 = 1 << 13;

    /// **TASK_GAMMA_PAUSE_CH4_EN**
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TASK_GAMMA_PAUSE_CH4_EN_OFFSET: u32 = 12;
    pub const TASK_GAMMA_PAUSE_CH4_EN_MASK: u32 = 0x00001000;
    pub const TASK_GAMMA_PAUSE_CH4_EN: u32 = 1 << 12;

    /// **TASK_GAMMA_PAUSE_CH3_EN**
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TASK_GAMMA_PAUSE_CH3_EN_OFFSET: u32 = 11;
    pub const TASK_GAMMA_PAUSE_CH3_EN_MASK: u32 = 0x00000800;
    pub const TASK_GAMMA_PAUSE_CH3_EN: u32 = 1 << 11;

    /// **TASK_GAMMA_PAUSE_CH2_EN**
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TASK_GAMMA_PAUSE_CH2_EN_OFFSET: u32 = 10;
    pub const TASK_GAMMA_PAUSE_CH2_EN_MASK: u32 = 0x00000400;
    pub const TASK_GAMMA_PAUSE_CH2_EN: u32 = 1 << 10;

    /// **TASK_GAMMA_PAUSE_CH1_EN**
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TASK_GAMMA_PAUSE_CH1_EN_OFFSET: u32 = 9;
    pub const TASK_GAMMA_PAUSE_CH1_EN_MASK: u32 = 0x00000200;
    pub const TASK_GAMMA_PAUSE_CH1_EN: u32 = 1 << 9;

    /// **TASK_GAMMA_PAUSE_CH0_EN**
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_pause task.\\0: Disable\\1: Enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TASK_GAMMA_PAUSE_CH0_EN_OFFSET: u32 = 8;
    pub const TASK_GAMMA_PAUSE_CH0_EN_MASK: u32 = 0x00000100;
    pub const TASK_GAMMA_PAUSE_CH0_EN: u32 = 1 << 8;

    /// **TASK_GAMMA_RESTART_CH7_EN**
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TASK_GAMMA_RESTART_CH7_EN_OFFSET: u32 = 7;
    pub const TASK_GAMMA_RESTART_CH7_EN_MASK: u32 = 0x00000080;
    pub const TASK_GAMMA_RESTART_CH7_EN: u32 = 1 << 7;

    /// **TASK_GAMMA_RESTART_CH6_EN**
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TASK_GAMMA_RESTART_CH6_EN_OFFSET: u32 = 6;
    pub const TASK_GAMMA_RESTART_CH6_EN_MASK: u32 = 0x00000040;
    pub const TASK_GAMMA_RESTART_CH6_EN: u32 = 1 << 6;

    /// **TASK_GAMMA_RESTART_CH5_EN**
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TASK_GAMMA_RESTART_CH5_EN_OFFSET: u32 = 5;
    pub const TASK_GAMMA_RESTART_CH5_EN_MASK: u32 = 0x00000020;
    pub const TASK_GAMMA_RESTART_CH5_EN: u32 = 1 << 5;

    /// **TASK_GAMMA_RESTART_CH4_EN**
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TASK_GAMMA_RESTART_CH4_EN_OFFSET: u32 = 4;
    pub const TASK_GAMMA_RESTART_CH4_EN_MASK: u32 = 0x00000010;
    pub const TASK_GAMMA_RESTART_CH4_EN: u32 = 1 << 4;

    /// **TASK_GAMMA_RESTART_CH3_EN**
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TASK_GAMMA_RESTART_CH3_EN_OFFSET: u32 = 3;
    pub const TASK_GAMMA_RESTART_CH3_EN_MASK: u32 = 0x00000008;
    pub const TASK_GAMMA_RESTART_CH3_EN: u32 = 1 << 3;

    /// **TASK_GAMMA_RESTART_CH2_EN**
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TASK_GAMMA_RESTART_CH2_EN_OFFSET: u32 = 2;
    pub const TASK_GAMMA_RESTART_CH2_EN_MASK: u32 = 0x00000004;
    pub const TASK_GAMMA_RESTART_CH2_EN: u32 = 1 << 2;

    /// **TASK_GAMMA_RESTART_CH1_EN**
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TASK_GAMMA_RESTART_CH1_EN_OFFSET: u32 = 1;
    pub const TASK_GAMMA_RESTART_CH1_EN_MASK: u32 = 0x00000002;
    pub const TASK_GAMMA_RESTART_CH1_EN: u32 = 1 << 1;

    /// **TASK_GAMMA_RESTART_CH0_EN**
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_restart task.\\0: Disable\\1: Enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TASK_GAMMA_RESTART_CH0_EN_OFFSET: u32 = 0;
    pub const TASK_GAMMA_RESTART_CH0_EN_MASK: u32 = 0x00000001;
    pub const TASK_GAMMA_RESTART_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TASK_GAMMA_RESUME_CH7_EN** `[23]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH7_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH7_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH7_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH6_EN** `[22]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH6_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH6_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH6_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH5_EN** `[21]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH5_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH5_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH5_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH4_EN** `[20]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH4_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH4_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH4_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH3_EN** `[19]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH3_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH3_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH3_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH2_EN** `[18]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH2_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH2_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH2_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH1_EN** `[17]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH1_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH1_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH1_EN_MASK);
    }

    /// **TASK_GAMMA_RESUME_CH0_EN** `[16]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_resume_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESUME_CH0_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESUME_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_resume_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESUME_CH0_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESUME_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_resume_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESUME_CH0_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH7_EN** `[15]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH7_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH7_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH7_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH6_EN** `[14]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH6_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH6_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH6_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH5_EN** `[13]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH5_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH5_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH5_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH4_EN** `[12]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH4_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH4_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH4_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH3_EN** `[11]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH3_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH3_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH3_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH2_EN** `[10]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH2_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH2_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH2_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH1_EN** `[9]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH1_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH1_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH1_EN_MASK);
    }

    /// **TASK_GAMMA_PAUSE_CH0_EN** `[8]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_pause_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_PAUSE_CH0_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_PAUSE_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_pause_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_PAUSE_CH0_EN_MASK);
    }

    /// Clear TASK_GAMMA_PAUSE_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_pause_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_PAUSE_CH0_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH7_EN** `[7]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch7_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH7_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch7_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH7_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH7_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH6_EN** `[6]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch6_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH6_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch6_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH6_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH6_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH5_EN** `[5]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch5_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH5_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch5_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH5_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH5_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH4_EN** `[4]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch4_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH4_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch4_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH4_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH4_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH3_EN** `[3]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch3_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH3_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch3_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH3_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH3_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH2_EN** `[2]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch2_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH2_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch2_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH2_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH2_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH1_EN** `[1]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch1_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH1_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch1_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH1_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH1_EN_MASK);
    }

    /// **TASK_GAMMA_RESTART_CH0_EN** `[0]` (RW)
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_gamma_restart_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::TASK_GAMMA_RESTART_CH0_EN_MASK) != 0
    }

    /// Set TASK_GAMMA_RESTART_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_task_gamma_restart_ch0_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TASK_GAMMA_RESTART_CH0_EN_MASK);
    }

    /// Clear TASK_GAMMA_RESTART_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_task_gamma_restart_ch0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TASK_GAMMA_RESTART_CH0_EN_MASK);
    }
}

/// EVT_TASK_EN2 value type for batch read/modify/write operations
///
/// Use `evttasken2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `evttasken2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EvtTaskEn2Val {
    bits: u32,
}

impl EvtTaskEn2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TASK_GAMMA_RESUME_CH7_EN** `[23:23]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH7_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH6_EN** `[22:22]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH6_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH5_EN** `[21:21]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH5_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH4_EN** `[20:20]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH4_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH3_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH2_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH1_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESUME_CH0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_resume_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESUME_CH0_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH7_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH6_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH5_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH4_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH3_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH2_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH1_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_PAUSE_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_pause_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_PAUSE_CH0_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH7_EN** `[7:7]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch7_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH7_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH6_EN** `[6:6]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch6_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH6_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH5_EN** `[5:5]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch5_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH5_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH4_EN** `[4:4]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch4_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH4_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH3_EN** `[3:3]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch3_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH3_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH2_EN** `[2:2]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch2_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH2_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH1_EN** `[1:1]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch1_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH1_EN_MASK) != 0
    }

    /// Read **TASK_GAMMA_RESTART_CH0_EN** `[0:0]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn task_gamma_restart_ch0_en(&self) -> bool {
        (self.bits & EvtTaskEn2::TASK_GAMMA_RESTART_CH0_EN_MASK) != 0
    }

    /// Set **TASK_GAMMA_RESUME_CH7_EN** `[23:23]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH6_EN** `[22:22]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH5_EN** `[21:21]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH4_EN** `[20:20]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH3_EN** `[19:19]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH2_EN** `[18:18]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH1_EN** `[17:17]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESUME_CH0_EN** `[16:16]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_resume_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESUME_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESUME_CH0_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH7_EN** `[15:15]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH6_EN** `[14:14]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH5_EN** `[13:13]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH4_EN** `[12:12]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH3_EN** `[11:11]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH2_EN** `[10:10]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH1_EN** `[9:9]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_PAUSE_CH0_EN** `[8:8]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_pause task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_pause_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_PAUSE_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_PAUSE_CH0_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH7_EN** `[7:7]`
    ///
    /// Configures whether or not to enable ledc_ch7_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH7_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH7_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH6_EN** `[6:6]`
    ///
    /// Configures whether or not to enable ledc_ch6_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH6_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH6_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH5_EN** `[5:5]`
    ///
    /// Configures whether or not to enable ledc_ch5_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH5_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH5_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH4_EN** `[4:4]`
    ///
    /// Configures whether or not to enable ledc_ch4_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH4_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH4_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH3_EN** `[3:3]`
    ///
    /// Configures whether or not to enable ledc_ch3_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH3_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH3_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH2_EN** `[2:2]`
    ///
    /// Configures whether or not to enable ledc_ch2_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH2_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH2_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH1_EN** `[1:1]`
    ///
    /// Configures whether or not to enable ledc_ch1_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH1_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH1_EN_MASK
            },
        }
    }

    /// Set **TASK_GAMMA_RESTART_CH0_EN** `[0:0]`
    ///
    /// Configures whether or not to enable ledc_ch0_gamma_restart task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub const fn with_task_gamma_restart_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EvtTaskEn2::TASK_GAMMA_RESTART_CH0_EN_MASK
            } else {
                self.bits & !EvtTaskEn2::TASK_GAMMA_RESTART_CH0_EN_MASK
            },
        }
    }
}

impl Default for EvtTaskEn2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EvtTaskEn2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EvtTaskEn2Val {
        EvtTaskEn2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EvtTaskEn2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EvtTaskEn2Val) -> EvtTaskEn2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0_CMP Register
///
/// Ledc timer0 compare value register.
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **TIMER_CMP** `[19:0]` (RW)
///
///   Configures the comparison value for LEDC timer%s.
#[repr(transparent)]
pub struct Timer0Cmp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer0Cmp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Cmp {
    // ======== Field Constants ========

    /// **TIMER_CMP**
    ///
    /// Configures the comparison value for LEDC timer%s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CMP_OFFSET: u32 = 0;
    pub const TIMER_CMP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn timer_cmp(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CMP_MASK) >> Self::TIMER_CMP_OFFSET
    }

    /// Write **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn write_timer_cmp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_CMP_MASK) | ((value << Self::TIMER_CMP_OFFSET) & Self::TIMER_CMP_MASK)
        });
    }
}

/// TIMER0_CMP value type for batch read/modify/write operations
///
/// Use `timer0cmp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0cmp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0CmpVal {
    bits: u32,
}

impl Timer0CmpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn timer_cmp(&self) -> u32 {
        (self.bits & Timer0Cmp::TIMER_CMP_MASK) >> Timer0Cmp::TIMER_CMP_OFFSET
    }

    /// Set **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn with_timer_cmp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer0Cmp::TIMER_CMP_MASK)
                | ((value << Timer0Cmp::TIMER_CMP_OFFSET) & Timer0Cmp::TIMER_CMP_MASK),
        }
    }
}

impl Default for Timer0CmpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer0Cmp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0CmpVal {
        Timer0CmpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer0CmpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer0CmpVal) -> Timer0CmpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER1_CMP Register
///
/// Ledc timer1 compare value register.
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **TIMER_CMP** `[19:0]` (RW)
///
///   Configures the comparison value for LEDC timer%s.
#[repr(transparent)]
pub struct Timer1Cmp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer1Cmp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Cmp {
    // ======== Field Constants ========

    /// **TIMER_CMP**
    ///
    /// Configures the comparison value for LEDC timer%s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CMP_OFFSET: u32 = 0;
    pub const TIMER_CMP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn timer_cmp(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CMP_MASK) >> Self::TIMER_CMP_OFFSET
    }

    /// Write **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn write_timer_cmp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_CMP_MASK) | ((value << Self::TIMER_CMP_OFFSET) & Self::TIMER_CMP_MASK)
        });
    }
}

/// TIMER1_CMP value type for batch read/modify/write operations
///
/// Use `timer1cmp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1cmp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1CmpVal {
    bits: u32,
}

impl Timer1CmpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn timer_cmp(&self) -> u32 {
        (self.bits & Timer1Cmp::TIMER_CMP_MASK) >> Timer1Cmp::TIMER_CMP_OFFSET
    }

    /// Set **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn with_timer_cmp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer1Cmp::TIMER_CMP_MASK)
                | ((value << Timer1Cmp::TIMER_CMP_OFFSET) & Timer1Cmp::TIMER_CMP_MASK),
        }
    }
}

impl Default for Timer1CmpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer1Cmp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1CmpVal {
        Timer1CmpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer1CmpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer1CmpVal) -> Timer1CmpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER2_CMP Register
///
/// Ledc timer2 compare value register.
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **TIMER_CMP** `[19:0]` (RW)
///
///   Configures the comparison value for LEDC timer%s.
#[repr(transparent)]
pub struct Timer2Cmp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer2Cmp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer2Cmp {
    // ======== Field Constants ========

    /// **TIMER_CMP**
    ///
    /// Configures the comparison value for LEDC timer%s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CMP_OFFSET: u32 = 0;
    pub const TIMER_CMP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn timer_cmp(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CMP_MASK) >> Self::TIMER_CMP_OFFSET
    }

    /// Write **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn write_timer_cmp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_CMP_MASK) | ((value << Self::TIMER_CMP_OFFSET) & Self::TIMER_CMP_MASK)
        });
    }
}

/// TIMER2_CMP value type for batch read/modify/write operations
///
/// Use `timer2cmp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer2cmp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer2CmpVal {
    bits: u32,
}

impl Timer2CmpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn timer_cmp(&self) -> u32 {
        (self.bits & Timer2Cmp::TIMER_CMP_MASK) >> Timer2Cmp::TIMER_CMP_OFFSET
    }

    /// Set **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn with_timer_cmp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer2Cmp::TIMER_CMP_MASK)
                | ((value << Timer2Cmp::TIMER_CMP_OFFSET) & Timer2Cmp::TIMER_CMP_MASK),
        }
    }
}

impl Default for Timer2CmpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer2Cmp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer2CmpVal {
        Timer2CmpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer2CmpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer2CmpVal) -> Timer2CmpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER3_CMP Register
///
/// Ledc timer3 compare value register.
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **TIMER_CMP** `[19:0]` (RW)
///
///   Configures the comparison value for LEDC timer%s.
#[repr(transparent)]
pub struct Timer3Cmp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer3Cmp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer3Cmp {
    // ======== Field Constants ========

    /// **TIMER_CMP**
    ///
    /// Configures the comparison value for LEDC timer%s.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CMP_OFFSET: u32 = 0;
    pub const TIMER_CMP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn timer_cmp(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CMP_MASK) >> Self::TIMER_CMP_OFFSET
    }

    /// Write **TIMER_CMP** `[19:0]` (RW)
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub fn write_timer_cmp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_CMP_MASK) | ((value << Self::TIMER_CMP_OFFSET) & Self::TIMER_CMP_MASK)
        });
    }
}

/// TIMER3_CMP value type for batch read/modify/write operations
///
/// Use `timer3cmp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer3cmp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer3CmpVal {
    bits: u32,
}

impl Timer3CmpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn timer_cmp(&self) -> u32 {
        (self.bits & Timer3Cmp::TIMER_CMP_MASK) >> Timer3Cmp::TIMER_CMP_OFFSET
    }

    /// Set **TIMER_CMP** `[19:0]`
    ///
    /// Configures the comparison value for LEDC timer%s.
    #[inline(always)]
    pub const fn with_timer_cmp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer3Cmp::TIMER_CMP_MASK)
                | ((value << Timer3Cmp::TIMER_CMP_OFFSET) & Timer3Cmp::TIMER_CMP_MASK),
        }
    }
}

impl Default for Timer3CmpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer3Cmp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer3CmpVal {
        Timer3CmpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer3CmpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer3CmpVal) -> Timer3CmpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0_CNT_CAP Register
///
/// Ledc timer0 captured count value register.
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **TIMER_CNT_CAP** `[19:0]` (RO)
///
///   Represents the captured LEDC timer%s count value.
#[repr(transparent)]
pub struct Timer0CntCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer0CntCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0CntCap {
    // ======== Field Constants ========

    /// **TIMER_CNT_CAP**
    ///
    /// Represents the captured LEDC timer%s count value.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_CAP_OFFSET: u32 = 0;
    pub const TIMER_CNT_CAP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT_CAP** `[19:0]` (RO)
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub fn timer_cnt_cap(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_CAP_MASK) >> Self::TIMER_CNT_CAP_OFFSET
    }
}

/// TIMER0_CNT_CAP value type for batch read/modify/write operations
///
/// Use `timer0cntcap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0cntcap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0CntCapVal {
    bits: u32,
}

impl Timer0CntCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT_CAP** `[19:0]`
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub const fn timer_cnt_cap(&self) -> u32 {
        (self.bits & Timer0CntCap::TIMER_CNT_CAP_MASK) >> Timer0CntCap::TIMER_CNT_CAP_OFFSET
    }
}

impl Default for Timer0CntCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer0CntCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0CntCapVal {
        Timer0CntCapVal::from_bits(self.value.read_bits())
    }
}

/// TIMER1_CNT_CAP Register
///
/// Ledc timer1 captured count value register.
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **TIMER_CNT_CAP** `[19:0]` (RO)
///
///   Represents the captured LEDC timer%s count value.
#[repr(transparent)]
pub struct Timer1CntCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer1CntCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1CntCap {
    // ======== Field Constants ========

    /// **TIMER_CNT_CAP**
    ///
    /// Represents the captured LEDC timer%s count value.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_CAP_OFFSET: u32 = 0;
    pub const TIMER_CNT_CAP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT_CAP** `[19:0]` (RO)
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub fn timer_cnt_cap(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_CAP_MASK) >> Self::TIMER_CNT_CAP_OFFSET
    }
}

/// TIMER1_CNT_CAP value type for batch read/modify/write operations
///
/// Use `timer1cntcap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1cntcap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1CntCapVal {
    bits: u32,
}

impl Timer1CntCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT_CAP** `[19:0]`
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub const fn timer_cnt_cap(&self) -> u32 {
        (self.bits & Timer1CntCap::TIMER_CNT_CAP_MASK) >> Timer1CntCap::TIMER_CNT_CAP_OFFSET
    }
}

impl Default for Timer1CntCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer1CntCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1CntCapVal {
        Timer1CntCapVal::from_bits(self.value.read_bits())
    }
}

/// TIMER2_CNT_CAP Register
///
/// Ledc timer2 captured count value register.
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **TIMER_CNT_CAP** `[19:0]` (RO)
///
///   Represents the captured LEDC timer%s count value.
#[repr(transparent)]
pub struct Timer2CntCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer2CntCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer2CntCap {
    // ======== Field Constants ========

    /// **TIMER_CNT_CAP**
    ///
    /// Represents the captured LEDC timer%s count value.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_CAP_OFFSET: u32 = 0;
    pub const TIMER_CNT_CAP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT_CAP** `[19:0]` (RO)
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub fn timer_cnt_cap(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_CAP_MASK) >> Self::TIMER_CNT_CAP_OFFSET
    }
}

/// TIMER2_CNT_CAP value type for batch read/modify/write operations
///
/// Use `timer2cntcap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer2cntcap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer2CntCapVal {
    bits: u32,
}

impl Timer2CntCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT_CAP** `[19:0]`
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub const fn timer_cnt_cap(&self) -> u32 {
        (self.bits & Timer2CntCap::TIMER_CNT_CAP_MASK) >> Timer2CntCap::TIMER_CNT_CAP_OFFSET
    }
}

impl Default for Timer2CntCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer2CntCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer2CntCapVal {
        Timer2CntCapVal::from_bits(self.value.read_bits())
    }
}

/// TIMER3_CNT_CAP Register
///
/// Ledc timer3 captured count value register.
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **TIMER_CNT_CAP** `[19:0]` (RO)
///
///   Represents the captured LEDC timer%s count value.
#[repr(transparent)]
pub struct Timer3CntCap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timer3CntCap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer3CntCap {
    // ======== Field Constants ========

    /// **TIMER_CNT_CAP**
    ///
    /// Represents the captured LEDC timer%s count value.
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_CNT_CAP_OFFSET: u32 = 0;
    pub const TIMER_CNT_CAP_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_CNT_CAP** `[19:0]` (RO)
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub fn timer_cnt_cap(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_CNT_CAP_MASK) >> Self::TIMER_CNT_CAP_OFFSET
    }
}

/// TIMER3_CNT_CAP value type for batch read/modify/write operations
///
/// Use `timer3cntcap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer3cntcap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer3CntCapVal {
    bits: u32,
}

impl Timer3CntCapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_CNT_CAP** `[19:0]`
    ///
    /// Represents the captured LEDC timer%s count value.
    #[inline(always)]
    pub const fn timer_cnt_cap(&self) -> u32 {
        (self.bits & Timer3CntCap::TIMER_CNT_CAP_MASK) >> Timer3CntCap::TIMER_CNT_CAP_OFFSET
    }
}

impl Default for Timer3CntCapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer3CntCap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer3CntCapVal {
        Timer3CntCapVal::from_bits(self.value.read_bits())
    }
}

/// CONF Register
///
/// LEDC global configuration register
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
///
/// - **GAMMA_RAM_CLK_EN_CH7** `[9]` (RW)
///
///   Configures whether or not to open LEDC ch7 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\1: Force open the clock gate for LEDC ch7 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH6** `[8]` (RW)
///
///   Configures whether or not to open LEDC ch6 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\1: Force open the clock gate for LEDC ch6 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH5** `[7]` (RW)
///
///   Configures whether or not to open LEDC ch5 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\1: Force open the clock gate for LEDC ch5 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH4** `[6]` (RW)
///
///   Configures whether or not to open LEDC ch4 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\1: Force open the clock gate for LEDC ch4 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH3** `[5]` (RW)
///
///   Configures whether or not to open LEDC ch3 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\1: Force open the clock gate for LEDC ch3 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH2** `[4]` (RW)
///
///   Configures whether or not to open LEDC ch2 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\1: Force open the clock gate for LEDC ch2 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH1** `[3]` (RW)
///
///   Configures whether or not to open LEDC ch1 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\1: Force open the clock gate for LEDC ch1 gamma ram
///
/// - **GAMMA_RAM_CLK_EN_CH0** `[2]` (RW)
///
///   Configures whether or not to open LEDC ch0 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\1: Force open the clock gate for LEDC ch0 gamma ram
///
/// - **APB_CLK_SEL** `[1:0]` (RW)
///
///   Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
#[repr(transparent)]
pub struct Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **GAMMA_RAM_CLK_EN_CH7**
    ///
    /// Configures whether or not to open LEDC ch7 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\1: Force open the clock gate for LEDC ch7 gamma ram
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const GAMMA_RAM_CLK_EN_CH7_OFFSET: u32 = 9;
    pub const GAMMA_RAM_CLK_EN_CH7_MASK: u32 = 0x00000200;
    pub const GAMMA_RAM_CLK_EN_CH7: u32 = 1 << 9;

    /// **GAMMA_RAM_CLK_EN_CH6**
    ///
    /// Configures whether or not to open LEDC ch6 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\1: Force open the clock gate for LEDC ch6 gamma ram
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GAMMA_RAM_CLK_EN_CH6_OFFSET: u32 = 8;
    pub const GAMMA_RAM_CLK_EN_CH6_MASK: u32 = 0x00000100;
    pub const GAMMA_RAM_CLK_EN_CH6: u32 = 1 << 8;

    /// **GAMMA_RAM_CLK_EN_CH5**
    ///
    /// Configures whether or not to open LEDC ch5 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\1: Force open the clock gate for LEDC ch5 gamma ram
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const GAMMA_RAM_CLK_EN_CH5_OFFSET: u32 = 7;
    pub const GAMMA_RAM_CLK_EN_CH5_MASK: u32 = 0x00000080;
    pub const GAMMA_RAM_CLK_EN_CH5: u32 = 1 << 7;

    /// **GAMMA_RAM_CLK_EN_CH4**
    ///
    /// Configures whether or not to open LEDC ch4 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\1: Force open the clock gate for LEDC ch4 gamma ram
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const GAMMA_RAM_CLK_EN_CH4_OFFSET: u32 = 6;
    pub const GAMMA_RAM_CLK_EN_CH4_MASK: u32 = 0x00000040;
    pub const GAMMA_RAM_CLK_EN_CH4: u32 = 1 << 6;

    /// **GAMMA_RAM_CLK_EN_CH3**
    ///
    /// Configures whether or not to open LEDC ch3 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\1: Force open the clock gate for LEDC ch3 gamma ram
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const GAMMA_RAM_CLK_EN_CH3_OFFSET: u32 = 5;
    pub const GAMMA_RAM_CLK_EN_CH3_MASK: u32 = 0x00000020;
    pub const GAMMA_RAM_CLK_EN_CH3: u32 = 1 << 5;

    /// **GAMMA_RAM_CLK_EN_CH2**
    ///
    /// Configures whether or not to open LEDC ch2 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\1: Force open the clock gate for LEDC ch2 gamma ram
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const GAMMA_RAM_CLK_EN_CH2_OFFSET: u32 = 4;
    pub const GAMMA_RAM_CLK_EN_CH2_MASK: u32 = 0x00000010;
    pub const GAMMA_RAM_CLK_EN_CH2: u32 = 1 << 4;

    /// **GAMMA_RAM_CLK_EN_CH1**
    ///
    /// Configures whether or not to open LEDC ch1 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\1: Force open the clock gate for LEDC ch1 gamma ram
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const GAMMA_RAM_CLK_EN_CH1_OFFSET: u32 = 3;
    pub const GAMMA_RAM_CLK_EN_CH1_MASK: u32 = 0x00000008;
    pub const GAMMA_RAM_CLK_EN_CH1: u32 = 1 << 3;

    /// **GAMMA_RAM_CLK_EN_CH0**
    ///
    /// Configures whether or not to open LEDC ch0 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\1: Force open the clock gate for LEDC ch0 gamma ram
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const GAMMA_RAM_CLK_EN_CH0_OFFSET: u32 = 2;
    pub const GAMMA_RAM_CLK_EN_CH0_MASK: u32 = 0x00000004;
    pub const GAMMA_RAM_CLK_EN_CH0: u32 = 1 << 2;

    /// **APB_CLK_SEL**
    ///
    /// Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const APB_CLK_SEL_OFFSET: u32 = 0;
    pub const APB_CLK_SEL_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
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

    /// **GAMMA_RAM_CLK_EN_CH7** `[9]` (RW)
    ///
    /// Configures whether or not to open LEDC ch7 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\1: Force open the clock gate for LEDC ch7 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch7(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH7_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch7(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH7_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch7(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH7_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH6** `[8]` (RW)
    ///
    /// Configures whether or not to open LEDC ch6 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\1: Force open the clock gate for LEDC ch6 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch6(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH6_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch6(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH6_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch6(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH6_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH5** `[7]` (RW)
    ///
    /// Configures whether or not to open LEDC ch5 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\1: Force open the clock gate for LEDC ch5 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH5_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch5(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH5_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch5(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH5_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH4** `[6]` (RW)
    ///
    /// Configures whether or not to open LEDC ch4 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\1: Force open the clock gate for LEDC ch4 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH4_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch4(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH4_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch4(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH4_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH3** `[5]` (RW)
    ///
    /// Configures whether or not to open LEDC ch3 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\1: Force open the clock gate for LEDC ch3 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH3_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch3(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH3_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch3(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH3_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH2** `[4]` (RW)
    ///
    /// Configures whether or not to open LEDC ch2 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\1: Force open the clock gate for LEDC ch2 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH2_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch2(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH2_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch2(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH2_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH1** `[3]` (RW)
    ///
    /// Configures whether or not to open LEDC ch1 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\1: Force open the clock gate for LEDC ch1 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH1_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch1(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH1_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch1(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH1_MASK);
    }

    /// **GAMMA_RAM_CLK_EN_CH0** `[2]` (RW)
    ///
    /// Configures whether or not to open LEDC ch0 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\1: Force open the clock gate for LEDC ch0 gamma ram
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&self) -> bool {
        (self.value.read_bits() & Self::GAMMA_RAM_CLK_EN_CH0_MASK) != 0
    }

    /// Set GAMMA_RAM_CLK_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gamma_ram_clk_en_ch0(&self) {
        self.value
            .modify_bits(|r| r | Self::GAMMA_RAM_CLK_EN_CH0_MASK);
    }

    /// Clear GAMMA_RAM_CLK_EN_CH0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gamma_ram_clk_en_ch0(&self) {
        self.value
            .modify_bits(|r| r & !Self::GAMMA_RAM_CLK_EN_CH0_MASK);
    }

    /// **APB_CLK_SEL** `[1:0]` (RW)
    ///
    /// Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::APB_CLK_SEL_MASK) >> Self::APB_CLK_SEL_OFFSET
    }

    /// Write **APB_CLK_SEL** `[1:0]` (RW)
    ///
    /// Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
    #[inline(always)]
    pub fn write_apb_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::APB_CLK_SEL_MASK)
                | ((value << Self::APB_CLK_SEL_OFFSET) & Self::APB_CLK_SEL_MASK)
        });
    }
}

/// CONF value type for batch read/modify/write operations
///
/// Use `conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfVal {
    bits: u32,
}

impl ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Conf::CLK_EN_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH7** `[9:9]`
    ///
    /// Configures whether or not to open LEDC ch7 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\1: Force open the clock gate for LEDC ch7 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch7(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH7_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH6** `[8:8]`
    ///
    /// Configures whether or not to open LEDC ch6 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\1: Force open the clock gate for LEDC ch6 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch6(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH6_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH5** `[7:7]`
    ///
    /// Configures whether or not to open LEDC ch5 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\1: Force open the clock gate for LEDC ch5 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch5(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH5_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH4** `[6:6]`
    ///
    /// Configures whether or not to open LEDC ch4 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\1: Force open the clock gate for LEDC ch4 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch4(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH4_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH3** `[5:5]`
    ///
    /// Configures whether or not to open LEDC ch3 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\1: Force open the clock gate for LEDC ch3 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch3(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH3_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH2** `[4:4]`
    ///
    /// Configures whether or not to open LEDC ch2 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\1: Force open the clock gate for LEDC ch2 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch2(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH2_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH1** `[3:3]`
    ///
    /// Configures whether or not to open LEDC ch1 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\1: Force open the clock gate for LEDC ch1 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch1(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH1_MASK) != 0
    }

    /// Read **GAMMA_RAM_CLK_EN_CH0** `[2:2]`
    ///
    /// Configures whether or not to open LEDC ch0 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\1: Force open the clock gate for LEDC ch0 gamma ram
    #[inline(always)]
    pub const fn gamma_ram_clk_en_ch0(&self) -> bool {
        (self.bits & Conf::GAMMA_RAM_CLK_EN_CH0_MASK) != 0
    }

    /// Read **APB_CLK_SEL** `[1:0]`
    ///
    /// Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
    #[inline(always)]
    pub const fn apb_clk_sel(&self) -> u32 {
        (self.bits & Conf::APB_CLK_SEL_MASK) >> Conf::APB_CLK_SEL_OFFSET
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CLK_EN_MASK
            } else {
                self.bits & !Conf::CLK_EN_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH7** `[9:9]`
    ///
    /// Configures whether or not to open LEDC ch7 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\1: Force open the clock gate for LEDC ch7 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH7_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH7_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH6** `[8:8]`
    ///
    /// Configures whether or not to open LEDC ch6 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\1: Force open the clock gate for LEDC ch6 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH6_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH6_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH5** `[7:7]`
    ///
    /// Configures whether or not to open LEDC ch5 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\1: Force open the clock gate for LEDC ch5 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH5_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH5_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH4** `[6:6]`
    ///
    /// Configures whether or not to open LEDC ch4 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\1: Force open the clock gate for LEDC ch4 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH4_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH4_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH3** `[5:5]`
    ///
    /// Configures whether or not to open LEDC ch3 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\1: Force open the clock gate for LEDC ch3 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH3_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH3_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH2** `[4:4]`
    ///
    /// Configures whether or not to open LEDC ch2 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\1: Force open the clock gate for LEDC ch2 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH2_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH2_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH1** `[3:3]`
    ///
    /// Configures whether or not to open LEDC ch1 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\1: Force open the clock gate for LEDC ch1 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH1_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH1_MASK
            },
        }
    }

    /// Set **GAMMA_RAM_CLK_EN_CH0** `[2:2]`
    ///
    /// Configures whether or not to open LEDC ch0 gamma ram clock gate.\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\1: Force open the clock gate for LEDC ch0 gamma ram
    #[inline(always)]
    pub const fn with_gamma_ram_clk_en_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::GAMMA_RAM_CLK_EN_CH0_MASK
            } else {
                self.bits & !Conf::GAMMA_RAM_CLK_EN_CH0_MASK
            },
        }
    }

    /// Set **APB_CLK_SEL** `[1:0]`
    ///
    /// Configures the clock source for the four timers.\\0: APB_CLK\\1: RC_FAST_CLK\\2: XTAL_CLK\\3: Invalid. No clock
    #[inline(always)]
    pub const fn with_apb_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf::APB_CLK_SEL_MASK)
                | ((value << Conf::APB_CLK_SEL_OFFSET) & Conf::APB_CLK_SEL_MASK),
        }
    }
}

impl Default for ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfVal {
        ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfVal) -> ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Version control register
///
/// Address offset: 0x0174
/// Reset value: 0x02303070
///
/// ## Fields
///
/// - **LEDC_DATE** `[27:0]` (RW)
///
///   Configures the version.
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

    /// **LEDC_DATE**
    ///
    /// Configures the version.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const LEDC_DATE_OFFSET: u32 = 0;
    pub const LEDC_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **LEDC_DATE** `[27:0]` (RW)
    ///
    /// Configures the version.
    #[inline(always)]
    pub fn ledc_date(&self) -> u32 {
        (self.value.read_bits() & Self::LEDC_DATE_MASK) >> Self::LEDC_DATE_OFFSET
    }

    /// Write **LEDC_DATE** `[27:0]` (RW)
    ///
    /// Configures the version.
    #[inline(always)]
    pub fn write_ledc_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LEDC_DATE_MASK) | ((value << Self::LEDC_DATE_OFFSET) & Self::LEDC_DATE_MASK)
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
        Self { bits: 0x02303070 }
    }

    /// Read **LEDC_DATE** `[27:0]`
    ///
    /// Configures the version.
    #[inline(always)]
    pub const fn ledc_date(&self) -> u32 {
        (self.bits & Date::LEDC_DATE_MASK) >> Date::LEDC_DATE_OFFSET
    }

    /// Set **LEDC_DATE** `[27:0]`
    ///
    /// Configures the version.
    #[inline(always)]
    pub const fn with_ledc_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::LEDC_DATE_MASK)
                | ((value << Date::LEDC_DATE_OFFSET) & Date::LEDC_DATE_MASK),
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

/// CH0_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 0
///
/// Address offset: 0x0400
#[repr(transparent)]
pub struct Ch0GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem0 {}

/// CH0_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 0
///
/// Address offset: 0x0404
#[repr(transparent)]
pub struct Ch0GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem1 {}

/// CH0_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 0
///
/// Address offset: 0x0408
#[repr(transparent)]
pub struct Ch0GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem2 {}

/// CH0_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 0
///
/// Address offset: 0x040c
#[repr(transparent)]
pub struct Ch0GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem3 {}

/// CH0_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 0
///
/// Address offset: 0x0410
#[repr(transparent)]
pub struct Ch0GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem4 {}

/// CH0_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 0
///
/// Address offset: 0x0414
#[repr(transparent)]
pub struct Ch0GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem5 {}

/// CH0_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 0
///
/// Address offset: 0x0418
#[repr(transparent)]
pub struct Ch0GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem6 {}

/// CH0_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 0
///
/// Address offset: 0x041c
#[repr(transparent)]
pub struct Ch0GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem7 {}

/// CH0_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 0
///
/// Address offset: 0x0420
#[repr(transparent)]
pub struct Ch0GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem8 {}

/// CH0_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 0
///
/// Address offset: 0x0424
#[repr(transparent)]
pub struct Ch0GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem9 {}

/// CH0_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 0
///
/// Address offset: 0x0428
#[repr(transparent)]
pub struct Ch0GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem10 {}

/// CH0_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 0
///
/// Address offset: 0x042c
#[repr(transparent)]
pub struct Ch0GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem11 {}

/// CH0_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 0
///
/// Address offset: 0x0430
#[repr(transparent)]
pub struct Ch0GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem12 {}

/// CH0_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 0
///
/// Address offset: 0x0434
#[repr(transparent)]
pub struct Ch0GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem13 {}

/// CH0_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 0
///
/// Address offset: 0x0438
#[repr(transparent)]
pub struct Ch0GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem14 {}

/// CH0_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 0
///
/// Address offset: 0x043c
#[repr(transparent)]
pub struct Ch0GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0GammaMem15 {}

/// CH1_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 1
///
/// Address offset: 0x0440
#[repr(transparent)]
pub struct Ch1GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem0 {}

/// CH1_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 1
///
/// Address offset: 0x0444
#[repr(transparent)]
pub struct Ch1GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem1 {}

/// CH1_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 1
///
/// Address offset: 0x0448
#[repr(transparent)]
pub struct Ch1GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem2 {}

/// CH1_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 1
///
/// Address offset: 0x044c
#[repr(transparent)]
pub struct Ch1GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem3 {}

/// CH1_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 1
///
/// Address offset: 0x0450
#[repr(transparent)]
pub struct Ch1GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem4 {}

/// CH1_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 1
///
/// Address offset: 0x0454
#[repr(transparent)]
pub struct Ch1GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem5 {}

/// CH1_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 1
///
/// Address offset: 0x0458
#[repr(transparent)]
pub struct Ch1GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem6 {}

/// CH1_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 1
///
/// Address offset: 0x045c
#[repr(transparent)]
pub struct Ch1GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem7 {}

/// CH1_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 1
///
/// Address offset: 0x0460
#[repr(transparent)]
pub struct Ch1GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem8 {}

/// CH1_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 1
///
/// Address offset: 0x0464
#[repr(transparent)]
pub struct Ch1GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem9 {}

/// CH1_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 1
///
/// Address offset: 0x0468
#[repr(transparent)]
pub struct Ch1GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem10 {}

/// CH1_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 1
///
/// Address offset: 0x046c
#[repr(transparent)]
pub struct Ch1GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem11 {}

/// CH1_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 1
///
/// Address offset: 0x0470
#[repr(transparent)]
pub struct Ch1GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem12 {}

/// CH1_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 1
///
/// Address offset: 0x0474
#[repr(transparent)]
pub struct Ch1GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem13 {}

/// CH1_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 1
///
/// Address offset: 0x0478
#[repr(transparent)]
pub struct Ch1GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem14 {}

/// CH1_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 1
///
/// Address offset: 0x047c
#[repr(transparent)]
pub struct Ch1GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1GammaMem15 {}

/// CH2_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 2
///
/// Address offset: 0x0480
#[repr(transparent)]
pub struct Ch2GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem0 {}

/// CH2_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 2
///
/// Address offset: 0x0484
#[repr(transparent)]
pub struct Ch2GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem1 {}

/// CH2_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 2
///
/// Address offset: 0x0488
#[repr(transparent)]
pub struct Ch2GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem2 {}

/// CH2_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 2
///
/// Address offset: 0x048c
#[repr(transparent)]
pub struct Ch2GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem3 {}

/// CH2_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 2
///
/// Address offset: 0x0490
#[repr(transparent)]
pub struct Ch2GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem4 {}

/// CH2_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 2
///
/// Address offset: 0x0494
#[repr(transparent)]
pub struct Ch2GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem5 {}

/// CH2_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 2
///
/// Address offset: 0x0498
#[repr(transparent)]
pub struct Ch2GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem6 {}

/// CH2_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 2
///
/// Address offset: 0x049c
#[repr(transparent)]
pub struct Ch2GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem7 {}

/// CH2_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 2
///
/// Address offset: 0x04a0
#[repr(transparent)]
pub struct Ch2GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem8 {}

/// CH2_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 2
///
/// Address offset: 0x04a4
#[repr(transparent)]
pub struct Ch2GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem9 {}

/// CH2_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 2
///
/// Address offset: 0x04a8
#[repr(transparent)]
pub struct Ch2GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem10 {}

/// CH2_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 2
///
/// Address offset: 0x04ac
#[repr(transparent)]
pub struct Ch2GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem11 {}

/// CH2_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 2
///
/// Address offset: 0x04b0
#[repr(transparent)]
pub struct Ch2GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem12 {}

/// CH2_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 2
///
/// Address offset: 0x04b4
#[repr(transparent)]
pub struct Ch2GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem13 {}

/// CH2_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 2
///
/// Address offset: 0x04b8
#[repr(transparent)]
pub struct Ch2GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem14 {}

/// CH2_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 2
///
/// Address offset: 0x04bc
#[repr(transparent)]
pub struct Ch2GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2GammaMem15 {}

/// CH3_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 3
///
/// Address offset: 0x04c0
#[repr(transparent)]
pub struct Ch3GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem0 {}

/// CH3_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 3
///
/// Address offset: 0x04c4
#[repr(transparent)]
pub struct Ch3GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem1 {}

/// CH3_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 3
///
/// Address offset: 0x04c8
#[repr(transparent)]
pub struct Ch3GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem2 {}

/// CH3_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 3
///
/// Address offset: 0x04cc
#[repr(transparent)]
pub struct Ch3GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem3 {}

/// CH3_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 3
///
/// Address offset: 0x04d0
#[repr(transparent)]
pub struct Ch3GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem4 {}

/// CH3_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 3
///
/// Address offset: 0x04d4
#[repr(transparent)]
pub struct Ch3GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem5 {}

/// CH3_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 3
///
/// Address offset: 0x04d8
#[repr(transparent)]
pub struct Ch3GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem6 {}

/// CH3_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 3
///
/// Address offset: 0x04dc
#[repr(transparent)]
pub struct Ch3GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem7 {}

/// CH3_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 3
///
/// Address offset: 0x04e0
#[repr(transparent)]
pub struct Ch3GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem8 {}

/// CH3_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 3
///
/// Address offset: 0x04e4
#[repr(transparent)]
pub struct Ch3GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem9 {}

/// CH3_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 3
///
/// Address offset: 0x04e8
#[repr(transparent)]
pub struct Ch3GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem10 {}

/// CH3_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 3
///
/// Address offset: 0x04ec
#[repr(transparent)]
pub struct Ch3GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem11 {}

/// CH3_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 3
///
/// Address offset: 0x04f0
#[repr(transparent)]
pub struct Ch3GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem12 {}

/// CH3_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 3
///
/// Address offset: 0x04f4
#[repr(transparent)]
pub struct Ch3GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem13 {}

/// CH3_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 3
///
/// Address offset: 0x04f8
#[repr(transparent)]
pub struct Ch3GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem14 {}

/// CH3_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 3
///
/// Address offset: 0x04fc
#[repr(transparent)]
pub struct Ch3GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3GammaMem15 {}

/// CH4_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 4
///
/// Address offset: 0x0500
#[repr(transparent)]
pub struct Ch4GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem0 {}

/// CH4_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 4
///
/// Address offset: 0x0504
#[repr(transparent)]
pub struct Ch4GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem1 {}

/// CH4_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 4
///
/// Address offset: 0x0508
#[repr(transparent)]
pub struct Ch4GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem2 {}

/// CH4_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 4
///
/// Address offset: 0x050c
#[repr(transparent)]
pub struct Ch4GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem3 {}

/// CH4_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 4
///
/// Address offset: 0x0510
#[repr(transparent)]
pub struct Ch4GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem4 {}

/// CH4_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 4
///
/// Address offset: 0x0514
#[repr(transparent)]
pub struct Ch4GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem5 {}

/// CH4_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 4
///
/// Address offset: 0x0518
#[repr(transparent)]
pub struct Ch4GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem6 {}

/// CH4_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 4
///
/// Address offset: 0x051c
#[repr(transparent)]
pub struct Ch4GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem7 {}

/// CH4_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 4
///
/// Address offset: 0x0520
#[repr(transparent)]
pub struct Ch4GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem8 {}

/// CH4_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 4
///
/// Address offset: 0x0524
#[repr(transparent)]
pub struct Ch4GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem9 {}

/// CH4_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 4
///
/// Address offset: 0x0528
#[repr(transparent)]
pub struct Ch4GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem10 {}

/// CH4_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 4
///
/// Address offset: 0x052c
#[repr(transparent)]
pub struct Ch4GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem11 {}

/// CH4_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 4
///
/// Address offset: 0x0530
#[repr(transparent)]
pub struct Ch4GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem12 {}

/// CH4_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 4
///
/// Address offset: 0x0534
#[repr(transparent)]
pub struct Ch4GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem13 {}

/// CH4_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 4
///
/// Address offset: 0x0538
#[repr(transparent)]
pub struct Ch4GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem14 {}

/// CH4_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 4
///
/// Address offset: 0x053c
#[repr(transparent)]
pub struct Ch4GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4GammaMem15 {}

/// CH5_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 5
///
/// Address offset: 0x0540
#[repr(transparent)]
pub struct Ch5GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem0 {}

/// CH5_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 5
///
/// Address offset: 0x0544
#[repr(transparent)]
pub struct Ch5GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem1 {}

/// CH5_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 5
///
/// Address offset: 0x0548
#[repr(transparent)]
pub struct Ch5GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem2 {}

/// CH5_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 5
///
/// Address offset: 0x054c
#[repr(transparent)]
pub struct Ch5GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem3 {}

/// CH5_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 5
///
/// Address offset: 0x0550
#[repr(transparent)]
pub struct Ch5GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem4 {}

/// CH5_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 5
///
/// Address offset: 0x0554
#[repr(transparent)]
pub struct Ch5GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem5 {}

/// CH5_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 5
///
/// Address offset: 0x0558
#[repr(transparent)]
pub struct Ch5GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem6 {}

/// CH5_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 5
///
/// Address offset: 0x055c
#[repr(transparent)]
pub struct Ch5GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem7 {}

/// CH5_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 5
///
/// Address offset: 0x0560
#[repr(transparent)]
pub struct Ch5GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem8 {}

/// CH5_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 5
///
/// Address offset: 0x0564
#[repr(transparent)]
pub struct Ch5GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem9 {}

/// CH5_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 5
///
/// Address offset: 0x0568
#[repr(transparent)]
pub struct Ch5GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem10 {}

/// CH5_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 5
///
/// Address offset: 0x056c
#[repr(transparent)]
pub struct Ch5GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem11 {}

/// CH5_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 5
///
/// Address offset: 0x0570
#[repr(transparent)]
pub struct Ch5GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem12 {}

/// CH5_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 5
///
/// Address offset: 0x0574
#[repr(transparent)]
pub struct Ch5GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem13 {}

/// CH5_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 5
///
/// Address offset: 0x0578
#[repr(transparent)]
pub struct Ch5GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem14 {}

/// CH5_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 5
///
/// Address offset: 0x057c
#[repr(transparent)]
pub struct Ch5GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5GammaMem15 {}

/// CH6_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 6
///
/// Address offset: 0x0580
#[repr(transparent)]
pub struct Ch6GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem0 {}

/// CH6_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 6
///
/// Address offset: 0x0584
#[repr(transparent)]
pub struct Ch6GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem1 {}

/// CH6_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 6
///
/// Address offset: 0x0588
#[repr(transparent)]
pub struct Ch6GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem2 {}

/// CH6_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 6
///
/// Address offset: 0x058c
#[repr(transparent)]
pub struct Ch6GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem3 {}

/// CH6_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 6
///
/// Address offset: 0x0590
#[repr(transparent)]
pub struct Ch6GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem4 {}

/// CH6_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 6
///
/// Address offset: 0x0594
#[repr(transparent)]
pub struct Ch6GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem5 {}

/// CH6_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 6
///
/// Address offset: 0x0598
#[repr(transparent)]
pub struct Ch6GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem6 {}

/// CH6_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 6
///
/// Address offset: 0x059c
#[repr(transparent)]
pub struct Ch6GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem7 {}

/// CH6_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 6
///
/// Address offset: 0x05a0
#[repr(transparent)]
pub struct Ch6GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem8 {}

/// CH6_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 6
///
/// Address offset: 0x05a4
#[repr(transparent)]
pub struct Ch6GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem9 {}

/// CH6_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 6
///
/// Address offset: 0x05a8
#[repr(transparent)]
pub struct Ch6GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem10 {}

/// CH6_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 6
///
/// Address offset: 0x05ac
#[repr(transparent)]
pub struct Ch6GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem11 {}

/// CH6_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 6
///
/// Address offset: 0x05b0
#[repr(transparent)]
pub struct Ch6GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem12 {}

/// CH6_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 6
///
/// Address offset: 0x05b4
#[repr(transparent)]
pub struct Ch6GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem13 {}

/// CH6_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 6
///
/// Address offset: 0x05b8
#[repr(transparent)]
pub struct Ch6GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem14 {}

/// CH6_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 6
///
/// Address offset: 0x05bc
#[repr(transparent)]
pub struct Ch6GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6GammaMem15 {}

/// CH7_GAMMA_MEM[0] Register
///
/// Gamma fade range 0 of channel 7
///
/// Address offset: 0x05c0
#[repr(transparent)]
pub struct Ch7GammaMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem0 {}

/// CH7_GAMMA_MEM[1] Register
///
/// Gamma fade range 1 of channel 7
///
/// Address offset: 0x05c4
#[repr(transparent)]
pub struct Ch7GammaMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem1 {}

/// CH7_GAMMA_MEM[2] Register
///
/// Gamma fade range 2 of channel 7
///
/// Address offset: 0x05c8
#[repr(transparent)]
pub struct Ch7GammaMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem2 {}

/// CH7_GAMMA_MEM[3] Register
///
/// Gamma fade range 3 of channel 7
///
/// Address offset: 0x05cc
#[repr(transparent)]
pub struct Ch7GammaMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem3 {}

/// CH7_GAMMA_MEM[4] Register
///
/// Gamma fade range 4 of channel 7
///
/// Address offset: 0x05d0
#[repr(transparent)]
pub struct Ch7GammaMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem4 {}

/// CH7_GAMMA_MEM[5] Register
///
/// Gamma fade range 5 of channel 7
///
/// Address offset: 0x05d4
#[repr(transparent)]
pub struct Ch7GammaMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem5 {}

/// CH7_GAMMA_MEM[6] Register
///
/// Gamma fade range 6 of channel 7
///
/// Address offset: 0x05d8
#[repr(transparent)]
pub struct Ch7GammaMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem6 {}

/// CH7_GAMMA_MEM[7] Register
///
/// Gamma fade range 7 of channel 7
///
/// Address offset: 0x05dc
#[repr(transparent)]
pub struct Ch7GammaMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem7 {}

/// CH7_GAMMA_MEM[8] Register
///
/// Gamma fade range 8 of channel 7
///
/// Address offset: 0x05e0
#[repr(transparent)]
pub struct Ch7GammaMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem8 {}

/// CH7_GAMMA_MEM[9] Register
///
/// Gamma fade range 9 of channel 7
///
/// Address offset: 0x05e4
#[repr(transparent)]
pub struct Ch7GammaMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem9 {}

/// CH7_GAMMA_MEM[10] Register
///
/// Gamma fade range 10 of channel 7
///
/// Address offset: 0x05e8
#[repr(transparent)]
pub struct Ch7GammaMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem10 {}

/// CH7_GAMMA_MEM[11] Register
///
/// Gamma fade range 11 of channel 7
///
/// Address offset: 0x05ec
#[repr(transparent)]
pub struct Ch7GammaMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem11 {}

/// CH7_GAMMA_MEM[12] Register
///
/// Gamma fade range 12 of channel 7
///
/// Address offset: 0x05f0
#[repr(transparent)]
pub struct Ch7GammaMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem12 {}

/// CH7_GAMMA_MEM[13] Register
///
/// Gamma fade range 13 of channel 7
///
/// Address offset: 0x05f4
#[repr(transparent)]
pub struct Ch7GammaMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem13 {}

/// CH7_GAMMA_MEM[14] Register
///
/// Gamma fade range 14 of channel 7
///
/// Address offset: 0x05f8
#[repr(transparent)]
pub struct Ch7GammaMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem14 {}

/// CH7_GAMMA_MEM[15] Register
///
/// Gamma fade range 15 of channel 7
///
/// Address offset: 0x05fc
#[repr(transparent)]
pub struct Ch7GammaMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7GammaMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7GammaMem15 {}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Configuration register 0 for channel 0
    pub ch0_conf0: Ch0Conf0,
    /// 0x0004
    /// High point register for channel 0
    pub ch0_hpoint: Ch0Hpoint,
    /// 0x0008
    /// Initial duty cycle register for channel 0
    pub ch0_duty: Ch0Duty,
    /// 0x000c
    /// Configuration register 1 for channel 0
    pub ch0_conf1: Ch0Conf1,
    /// 0x0010
    /// Current duty cycle register for channel 0
    pub ch0_duty_r: Ch0DutyR,
    /// 0x0014
    /// Configuration register 0 for channel 1
    pub ch1_conf0: Ch1Conf0,
    /// 0x0018
    /// High point register for channel 1
    pub ch1_hpoint: Ch1Hpoint,
    /// 0x001c
    /// Initial duty cycle register for channel 1
    pub ch1_duty: Ch1Duty,
    /// 0x0020
    /// Configuration register 1 for channel 1
    pub ch1_conf1: Ch1Conf1,
    /// 0x0024
    /// Current duty cycle register for channel 1
    pub ch1_duty_r: Ch1DutyR,
    /// 0x0028
    /// Configuration register 0 for channel 2
    pub ch2_conf0: Ch2Conf0,
    /// 0x002c
    /// High point register for channel 2
    pub ch2_hpoint: Ch2Hpoint,
    /// 0x0030
    /// Initial duty cycle register for channel 2
    pub ch2_duty: Ch2Duty,
    /// 0x0034
    /// Configuration register 1 for channel 2
    pub ch2_conf1: Ch2Conf1,
    /// 0x0038
    /// Current duty cycle register for channel 2
    pub ch2_duty_r: Ch2DutyR,
    /// 0x003c
    /// Configuration register 0 for channel 3
    pub ch3_conf0: Ch3Conf0,
    /// 0x0040
    /// High point register for channel 3
    pub ch3_hpoint: Ch3Hpoint,
    /// 0x0044
    /// Initial duty cycle register for channel 3
    pub ch3_duty: Ch3Duty,
    /// 0x0048
    /// Configuration register 1 for channel 3
    pub ch3_conf1: Ch3Conf1,
    /// 0x004c
    /// Current duty cycle register for channel 3
    pub ch3_duty_r: Ch3DutyR,
    /// 0x0050
    /// Configuration register 0 for channel 4
    pub ch4_conf0: Ch4Conf0,
    /// 0x0054
    /// High point register for channel 4
    pub ch4_hpoint: Ch4Hpoint,
    /// 0x0058
    /// Initial duty cycle register for channel 4
    pub ch4_duty: Ch4Duty,
    /// 0x005c
    /// Configuration register 1 for channel 4
    pub ch4_conf1: Ch4Conf1,
    /// 0x0060
    /// Current duty cycle register for channel 4
    pub ch4_duty_r: Ch4DutyR,
    /// 0x0064
    /// Configuration register 0 for channel 5
    pub ch5_conf0: Ch5Conf0,
    /// 0x0068
    /// High point register for channel 5
    pub ch5_hpoint: Ch5Hpoint,
    /// 0x006c
    /// Initial duty cycle register for channel 5
    pub ch5_duty: Ch5Duty,
    /// 0x0070
    /// Configuration register 1 for channel 5
    pub ch5_conf1: Ch5Conf1,
    /// 0x0074
    /// Current duty cycle register for channel 5
    pub ch5_duty_r: Ch5DutyR,
    /// 0x0078
    /// Configuration register 0 for channel 6
    pub ch6_conf0: Ch6Conf0,
    /// 0x007c
    /// High point register for channel 6
    pub ch6_hpoint: Ch6Hpoint,
    /// 0x0080
    /// Initial duty cycle register for channel 6
    pub ch6_duty: Ch6Duty,
    /// 0x0084
    /// Configuration register 1 for channel 6
    pub ch6_conf1: Ch6Conf1,
    /// 0x0088
    /// Current duty cycle register for channel 6
    pub ch6_duty_r: Ch6DutyR,
    /// 0x008c
    /// Configuration register 0 for channel 7
    pub ch7_conf0: Ch7Conf0,
    /// 0x0090
    /// High point register for channel 7
    pub ch7_hpoint: Ch7Hpoint,
    /// 0x0094
    /// Initial duty cycle register for channel 7
    pub ch7_duty: Ch7Duty,
    /// 0x0098
    /// Configuration register 1 for channel 7
    pub ch7_conf1: Ch7Conf1,
    /// 0x009c
    /// Current duty cycle register for channel 7
    pub ch7_duty_r: Ch7DutyR,
    /// 0x00a0
    /// Timer 0 configuration register
    pub timer0_conf: Timer0Conf,
    /// 0x00a4
    /// Timer 0 current counter value register
    pub timer0_value: Timer0Value,
    /// 0x00a8
    /// Timer 1 configuration register
    pub timer1_conf: Timer1Conf,
    /// 0x00ac
    /// Timer 1 current counter value register
    pub timer1_value: Timer1Value,
    /// 0x00b0
    /// Timer 2 configuration register
    pub timer2_conf: Timer2Conf,
    /// 0x00b4
    /// Timer 2 current counter value register
    pub timer2_value: Timer2Value,
    /// 0x00b8
    /// Timer 3 configuration register
    pub timer3_conf: Timer3Conf,
    /// 0x00bc
    /// Timer 3 current counter value register
    pub timer3_value: Timer3Value,
    /// 0x00c0
    /// Interrupt raw status register
    pub int_raw: IntRaw,
    /// 0x00c4
    /// Interrupt masked status register
    pub int_st: IntSt,
    /// 0x00c8
    /// Interrupt enable register
    pub int_ena: IntEna,
    /// 0x00cc
    /// Interrupt clear register
    pub int_clr: IntClr,
    _reserved0: [u8; 48],
    /// 0x0100
    /// Ledc ch0 gamma config register.
    pub ch0_gamma_conf: Ch0GammaConf,
    /// 0x0104
    /// Ledc ch1 gamma config register.
    pub ch1_gamma_conf: Ch1GammaConf,
    /// 0x0108
    /// Ledc ch2 gamma config register.
    pub ch2_gamma_conf: Ch2GammaConf,
    /// 0x010c
    /// Ledc ch3 gamma config register.
    pub ch3_gamma_conf: Ch3GammaConf,
    /// 0x0110
    /// Ledc ch4 gamma config register.
    pub ch4_gamma_conf: Ch4GammaConf,
    /// 0x0114
    /// Ledc ch5 gamma config register.
    pub ch5_gamma_conf: Ch5GammaConf,
    /// 0x0118
    /// Ledc ch6 gamma config register.
    pub ch6_gamma_conf: Ch6GammaConf,
    /// 0x011c
    /// Ledc ch7 gamma config register.
    pub ch7_gamma_conf: Ch7GammaConf,
    /// 0x0120
    /// Ledc event task enable bit register0.
    pub evt_task_en0: EvtTaskEn0,
    /// 0x0124
    /// Ledc event task enable bit register1.
    pub evt_task_en1: EvtTaskEn1,
    /// 0x0128
    /// Ledc event task enable bit register2.
    pub evt_task_en2: EvtTaskEn2,
    _reserved1: [u8; 20],
    /// 0x0140
    /// Ledc timer0 compare value register.
    pub timer0_cmp: Timer0Cmp,
    /// 0x0144
    /// Ledc timer1 compare value register.
    pub timer1_cmp: Timer1Cmp,
    /// 0x0148
    /// Ledc timer2 compare value register.
    pub timer2_cmp: Timer2Cmp,
    /// 0x014c
    /// Ledc timer3 compare value register.
    pub timer3_cmp: Timer3Cmp,
    /// 0x0150
    /// Ledc timer0 captured count value register.
    pub timer0_cnt_cap: Timer0CntCap,
    /// 0x0154
    /// Ledc timer1 captured count value register.
    pub timer1_cnt_cap: Timer1CntCap,
    /// 0x0158
    /// Ledc timer2 captured count value register.
    pub timer2_cnt_cap: Timer2CntCap,
    /// 0x015c
    /// Ledc timer3 captured count value register.
    pub timer3_cnt_cap: Timer3CntCap,
    _reserved2: [u8; 16],
    /// 0x0170
    /// LEDC global configuration register
    pub conf: Conf,
    /// 0x0174
    /// Version control register
    pub date: Date,
    _reserved3: [u8; 648],
    /// 0x0400
    /// Gamma fade range 0 of channel 0
    pub ch0_gamma_mem0: Ch0GammaMem0,
    /// 0x0404
    /// Gamma fade range 1 of channel 0
    pub ch0_gamma_mem1: Ch0GammaMem1,
    /// 0x0408
    /// Gamma fade range 2 of channel 0
    pub ch0_gamma_mem2: Ch0GammaMem2,
    /// 0x040c
    /// Gamma fade range 3 of channel 0
    pub ch0_gamma_mem3: Ch0GammaMem3,
    /// 0x0410
    /// Gamma fade range 4 of channel 0
    pub ch0_gamma_mem4: Ch0GammaMem4,
    /// 0x0414
    /// Gamma fade range 5 of channel 0
    pub ch0_gamma_mem5: Ch0GammaMem5,
    /// 0x0418
    /// Gamma fade range 6 of channel 0
    pub ch0_gamma_mem6: Ch0GammaMem6,
    /// 0x041c
    /// Gamma fade range 7 of channel 0
    pub ch0_gamma_mem7: Ch0GammaMem7,
    /// 0x0420
    /// Gamma fade range 8 of channel 0
    pub ch0_gamma_mem8: Ch0GammaMem8,
    /// 0x0424
    /// Gamma fade range 9 of channel 0
    pub ch0_gamma_mem9: Ch0GammaMem9,
    /// 0x0428
    /// Gamma fade range 10 of channel 0
    pub ch0_gamma_mem10: Ch0GammaMem10,
    /// 0x042c
    /// Gamma fade range 11 of channel 0
    pub ch0_gamma_mem11: Ch0GammaMem11,
    /// 0x0430
    /// Gamma fade range 12 of channel 0
    pub ch0_gamma_mem12: Ch0GammaMem12,
    /// 0x0434
    /// Gamma fade range 13 of channel 0
    pub ch0_gamma_mem13: Ch0GammaMem13,
    /// 0x0438
    /// Gamma fade range 14 of channel 0
    pub ch0_gamma_mem14: Ch0GammaMem14,
    /// 0x043c
    /// Gamma fade range 15 of channel 0
    pub ch0_gamma_mem15: Ch0GammaMem15,
    /// 0x0440
    /// Gamma fade range 0 of channel 1
    pub ch1_gamma_mem0: Ch1GammaMem0,
    /// 0x0444
    /// Gamma fade range 1 of channel 1
    pub ch1_gamma_mem1: Ch1GammaMem1,
    /// 0x0448
    /// Gamma fade range 2 of channel 1
    pub ch1_gamma_mem2: Ch1GammaMem2,
    /// 0x044c
    /// Gamma fade range 3 of channel 1
    pub ch1_gamma_mem3: Ch1GammaMem3,
    /// 0x0450
    /// Gamma fade range 4 of channel 1
    pub ch1_gamma_mem4: Ch1GammaMem4,
    /// 0x0454
    /// Gamma fade range 5 of channel 1
    pub ch1_gamma_mem5: Ch1GammaMem5,
    /// 0x0458
    /// Gamma fade range 6 of channel 1
    pub ch1_gamma_mem6: Ch1GammaMem6,
    /// 0x045c
    /// Gamma fade range 7 of channel 1
    pub ch1_gamma_mem7: Ch1GammaMem7,
    /// 0x0460
    /// Gamma fade range 8 of channel 1
    pub ch1_gamma_mem8: Ch1GammaMem8,
    /// 0x0464
    /// Gamma fade range 9 of channel 1
    pub ch1_gamma_mem9: Ch1GammaMem9,
    /// 0x0468
    /// Gamma fade range 10 of channel 1
    pub ch1_gamma_mem10: Ch1GammaMem10,
    /// 0x046c
    /// Gamma fade range 11 of channel 1
    pub ch1_gamma_mem11: Ch1GammaMem11,
    /// 0x0470
    /// Gamma fade range 12 of channel 1
    pub ch1_gamma_mem12: Ch1GammaMem12,
    /// 0x0474
    /// Gamma fade range 13 of channel 1
    pub ch1_gamma_mem13: Ch1GammaMem13,
    /// 0x0478
    /// Gamma fade range 14 of channel 1
    pub ch1_gamma_mem14: Ch1GammaMem14,
    /// 0x047c
    /// Gamma fade range 15 of channel 1
    pub ch1_gamma_mem15: Ch1GammaMem15,
    /// 0x0480
    /// Gamma fade range 0 of channel 2
    pub ch2_gamma_mem0: Ch2GammaMem0,
    /// 0x0484
    /// Gamma fade range 1 of channel 2
    pub ch2_gamma_mem1: Ch2GammaMem1,
    /// 0x0488
    /// Gamma fade range 2 of channel 2
    pub ch2_gamma_mem2: Ch2GammaMem2,
    /// 0x048c
    /// Gamma fade range 3 of channel 2
    pub ch2_gamma_mem3: Ch2GammaMem3,
    /// 0x0490
    /// Gamma fade range 4 of channel 2
    pub ch2_gamma_mem4: Ch2GammaMem4,
    /// 0x0494
    /// Gamma fade range 5 of channel 2
    pub ch2_gamma_mem5: Ch2GammaMem5,
    /// 0x0498
    /// Gamma fade range 6 of channel 2
    pub ch2_gamma_mem6: Ch2GammaMem6,
    /// 0x049c
    /// Gamma fade range 7 of channel 2
    pub ch2_gamma_mem7: Ch2GammaMem7,
    /// 0x04a0
    /// Gamma fade range 8 of channel 2
    pub ch2_gamma_mem8: Ch2GammaMem8,
    /// 0x04a4
    /// Gamma fade range 9 of channel 2
    pub ch2_gamma_mem9: Ch2GammaMem9,
    /// 0x04a8
    /// Gamma fade range 10 of channel 2
    pub ch2_gamma_mem10: Ch2GammaMem10,
    /// 0x04ac
    /// Gamma fade range 11 of channel 2
    pub ch2_gamma_mem11: Ch2GammaMem11,
    /// 0x04b0
    /// Gamma fade range 12 of channel 2
    pub ch2_gamma_mem12: Ch2GammaMem12,
    /// 0x04b4
    /// Gamma fade range 13 of channel 2
    pub ch2_gamma_mem13: Ch2GammaMem13,
    /// 0x04b8
    /// Gamma fade range 14 of channel 2
    pub ch2_gamma_mem14: Ch2GammaMem14,
    /// 0x04bc
    /// Gamma fade range 15 of channel 2
    pub ch2_gamma_mem15: Ch2GammaMem15,
    /// 0x04c0
    /// Gamma fade range 0 of channel 3
    pub ch3_gamma_mem0: Ch3GammaMem0,
    /// 0x04c4
    /// Gamma fade range 1 of channel 3
    pub ch3_gamma_mem1: Ch3GammaMem1,
    /// 0x04c8
    /// Gamma fade range 2 of channel 3
    pub ch3_gamma_mem2: Ch3GammaMem2,
    /// 0x04cc
    /// Gamma fade range 3 of channel 3
    pub ch3_gamma_mem3: Ch3GammaMem3,
    /// 0x04d0
    /// Gamma fade range 4 of channel 3
    pub ch3_gamma_mem4: Ch3GammaMem4,
    /// 0x04d4
    /// Gamma fade range 5 of channel 3
    pub ch3_gamma_mem5: Ch3GammaMem5,
    /// 0x04d8
    /// Gamma fade range 6 of channel 3
    pub ch3_gamma_mem6: Ch3GammaMem6,
    /// 0x04dc
    /// Gamma fade range 7 of channel 3
    pub ch3_gamma_mem7: Ch3GammaMem7,
    /// 0x04e0
    /// Gamma fade range 8 of channel 3
    pub ch3_gamma_mem8: Ch3GammaMem8,
    /// 0x04e4
    /// Gamma fade range 9 of channel 3
    pub ch3_gamma_mem9: Ch3GammaMem9,
    /// 0x04e8
    /// Gamma fade range 10 of channel 3
    pub ch3_gamma_mem10: Ch3GammaMem10,
    /// 0x04ec
    /// Gamma fade range 11 of channel 3
    pub ch3_gamma_mem11: Ch3GammaMem11,
    /// 0x04f0
    /// Gamma fade range 12 of channel 3
    pub ch3_gamma_mem12: Ch3GammaMem12,
    /// 0x04f4
    /// Gamma fade range 13 of channel 3
    pub ch3_gamma_mem13: Ch3GammaMem13,
    /// 0x04f8
    /// Gamma fade range 14 of channel 3
    pub ch3_gamma_mem14: Ch3GammaMem14,
    /// 0x04fc
    /// Gamma fade range 15 of channel 3
    pub ch3_gamma_mem15: Ch3GammaMem15,
    /// 0x0500
    /// Gamma fade range 0 of channel 4
    pub ch4_gamma_mem0: Ch4GammaMem0,
    /// 0x0504
    /// Gamma fade range 1 of channel 4
    pub ch4_gamma_mem1: Ch4GammaMem1,
    /// 0x0508
    /// Gamma fade range 2 of channel 4
    pub ch4_gamma_mem2: Ch4GammaMem2,
    /// 0x050c
    /// Gamma fade range 3 of channel 4
    pub ch4_gamma_mem3: Ch4GammaMem3,
    /// 0x0510
    /// Gamma fade range 4 of channel 4
    pub ch4_gamma_mem4: Ch4GammaMem4,
    /// 0x0514
    /// Gamma fade range 5 of channel 4
    pub ch4_gamma_mem5: Ch4GammaMem5,
    /// 0x0518
    /// Gamma fade range 6 of channel 4
    pub ch4_gamma_mem6: Ch4GammaMem6,
    /// 0x051c
    /// Gamma fade range 7 of channel 4
    pub ch4_gamma_mem7: Ch4GammaMem7,
    /// 0x0520
    /// Gamma fade range 8 of channel 4
    pub ch4_gamma_mem8: Ch4GammaMem8,
    /// 0x0524
    /// Gamma fade range 9 of channel 4
    pub ch4_gamma_mem9: Ch4GammaMem9,
    /// 0x0528
    /// Gamma fade range 10 of channel 4
    pub ch4_gamma_mem10: Ch4GammaMem10,
    /// 0x052c
    /// Gamma fade range 11 of channel 4
    pub ch4_gamma_mem11: Ch4GammaMem11,
    /// 0x0530
    /// Gamma fade range 12 of channel 4
    pub ch4_gamma_mem12: Ch4GammaMem12,
    /// 0x0534
    /// Gamma fade range 13 of channel 4
    pub ch4_gamma_mem13: Ch4GammaMem13,
    /// 0x0538
    /// Gamma fade range 14 of channel 4
    pub ch4_gamma_mem14: Ch4GammaMem14,
    /// 0x053c
    /// Gamma fade range 15 of channel 4
    pub ch4_gamma_mem15: Ch4GammaMem15,
    /// 0x0540
    /// Gamma fade range 0 of channel 5
    pub ch5_gamma_mem0: Ch5GammaMem0,
    /// 0x0544
    /// Gamma fade range 1 of channel 5
    pub ch5_gamma_mem1: Ch5GammaMem1,
    /// 0x0548
    /// Gamma fade range 2 of channel 5
    pub ch5_gamma_mem2: Ch5GammaMem2,
    /// 0x054c
    /// Gamma fade range 3 of channel 5
    pub ch5_gamma_mem3: Ch5GammaMem3,
    /// 0x0550
    /// Gamma fade range 4 of channel 5
    pub ch5_gamma_mem4: Ch5GammaMem4,
    /// 0x0554
    /// Gamma fade range 5 of channel 5
    pub ch5_gamma_mem5: Ch5GammaMem5,
    /// 0x0558
    /// Gamma fade range 6 of channel 5
    pub ch5_gamma_mem6: Ch5GammaMem6,
    /// 0x055c
    /// Gamma fade range 7 of channel 5
    pub ch5_gamma_mem7: Ch5GammaMem7,
    /// 0x0560
    /// Gamma fade range 8 of channel 5
    pub ch5_gamma_mem8: Ch5GammaMem8,
    /// 0x0564
    /// Gamma fade range 9 of channel 5
    pub ch5_gamma_mem9: Ch5GammaMem9,
    /// 0x0568
    /// Gamma fade range 10 of channel 5
    pub ch5_gamma_mem10: Ch5GammaMem10,
    /// 0x056c
    /// Gamma fade range 11 of channel 5
    pub ch5_gamma_mem11: Ch5GammaMem11,
    /// 0x0570
    /// Gamma fade range 12 of channel 5
    pub ch5_gamma_mem12: Ch5GammaMem12,
    /// 0x0574
    /// Gamma fade range 13 of channel 5
    pub ch5_gamma_mem13: Ch5GammaMem13,
    /// 0x0578
    /// Gamma fade range 14 of channel 5
    pub ch5_gamma_mem14: Ch5GammaMem14,
    /// 0x057c
    /// Gamma fade range 15 of channel 5
    pub ch5_gamma_mem15: Ch5GammaMem15,
    /// 0x0580
    /// Gamma fade range 0 of channel 6
    pub ch6_gamma_mem0: Ch6GammaMem0,
    /// 0x0584
    /// Gamma fade range 1 of channel 6
    pub ch6_gamma_mem1: Ch6GammaMem1,
    /// 0x0588
    /// Gamma fade range 2 of channel 6
    pub ch6_gamma_mem2: Ch6GammaMem2,
    /// 0x058c
    /// Gamma fade range 3 of channel 6
    pub ch6_gamma_mem3: Ch6GammaMem3,
    /// 0x0590
    /// Gamma fade range 4 of channel 6
    pub ch6_gamma_mem4: Ch6GammaMem4,
    /// 0x0594
    /// Gamma fade range 5 of channel 6
    pub ch6_gamma_mem5: Ch6GammaMem5,
    /// 0x0598
    /// Gamma fade range 6 of channel 6
    pub ch6_gamma_mem6: Ch6GammaMem6,
    /// 0x059c
    /// Gamma fade range 7 of channel 6
    pub ch6_gamma_mem7: Ch6GammaMem7,
    /// 0x05a0
    /// Gamma fade range 8 of channel 6
    pub ch6_gamma_mem8: Ch6GammaMem8,
    /// 0x05a4
    /// Gamma fade range 9 of channel 6
    pub ch6_gamma_mem9: Ch6GammaMem9,
    /// 0x05a8
    /// Gamma fade range 10 of channel 6
    pub ch6_gamma_mem10: Ch6GammaMem10,
    /// 0x05ac
    /// Gamma fade range 11 of channel 6
    pub ch6_gamma_mem11: Ch6GammaMem11,
    /// 0x05b0
    /// Gamma fade range 12 of channel 6
    pub ch6_gamma_mem12: Ch6GammaMem12,
    /// 0x05b4
    /// Gamma fade range 13 of channel 6
    pub ch6_gamma_mem13: Ch6GammaMem13,
    /// 0x05b8
    /// Gamma fade range 14 of channel 6
    pub ch6_gamma_mem14: Ch6GammaMem14,
    /// 0x05bc
    /// Gamma fade range 15 of channel 6
    pub ch6_gamma_mem15: Ch6GammaMem15,
    /// 0x05c0
    /// Gamma fade range 0 of channel 7
    pub ch7_gamma_mem0: Ch7GammaMem0,
    /// 0x05c4
    /// Gamma fade range 1 of channel 7
    pub ch7_gamma_mem1: Ch7GammaMem1,
    /// 0x05c8
    /// Gamma fade range 2 of channel 7
    pub ch7_gamma_mem2: Ch7GammaMem2,
    /// 0x05cc
    /// Gamma fade range 3 of channel 7
    pub ch7_gamma_mem3: Ch7GammaMem3,
    /// 0x05d0
    /// Gamma fade range 4 of channel 7
    pub ch7_gamma_mem4: Ch7GammaMem4,
    /// 0x05d4
    /// Gamma fade range 5 of channel 7
    pub ch7_gamma_mem5: Ch7GammaMem5,
    /// 0x05d8
    /// Gamma fade range 6 of channel 7
    pub ch7_gamma_mem6: Ch7GammaMem6,
    /// 0x05dc
    /// Gamma fade range 7 of channel 7
    pub ch7_gamma_mem7: Ch7GammaMem7,
    /// 0x05e0
    /// Gamma fade range 8 of channel 7
    pub ch7_gamma_mem8: Ch7GammaMem8,
    /// 0x05e4
    /// Gamma fade range 9 of channel 7
    pub ch7_gamma_mem9: Ch7GammaMem9,
    /// 0x05e8
    /// Gamma fade range 10 of channel 7
    pub ch7_gamma_mem10: Ch7GammaMem10,
    /// 0x05ec
    /// Gamma fade range 11 of channel 7
    pub ch7_gamma_mem11: Ch7GammaMem11,
    /// 0x05f0
    /// Gamma fade range 12 of channel 7
    pub ch7_gamma_mem12: Ch7GammaMem12,
    /// 0x05f4
    /// Gamma fade range 13 of channel 7
    pub ch7_gamma_mem13: Ch7GammaMem13,
    /// 0x05f8
    /// Gamma fade range 14 of channel 7
    pub ch7_gamma_mem14: Ch7GammaMem14,
    /// 0x05fc
    /// Gamma fade range 15 of channel 7
    pub ch7_gamma_mem15: Ch7GammaMem15,
}
