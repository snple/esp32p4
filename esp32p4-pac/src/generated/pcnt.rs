//! PCNT
//! Pulse Count Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x500c9000;

/// U0_CONF0 Register
///
/// Configuration register 0 for unit 0
///
/// Address offset: 0x0000
/// Reset value: 0x00003c10
///
/// ## Fields
///
/// - **CH1_LCTRL_MODE_U** `[31:30]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_HCTRL_MODE_U** `[29:28]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_POS_MODE_U** `[27:26]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a positive edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH1_NEG_MODE_U** `[25:24]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a negative edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH0_LCTRL_MODE_U** `[23:22]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_HCTRL_MODE_U** `[21:20]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_POS_MODE_U** `[19:18]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a positive edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **CH0_NEG_MODE_U** `[17:16]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a negative edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **THR_THRES1_EN_U** `[15]` (RW)
///
///   This is the enable bit for unit %s's thres1 comparator.
///
/// - **THR_THRES0_EN_U** `[14]` (RW)
///
///   This is the enable bit for unit %s's thres0 comparator.
///
/// - **THR_L_LIM_EN_U** `[13]` (RW)
///
///   This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
///
/// - **THR_H_LIM_EN_U** `[12]` (RW)
///
///   This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
///
/// - **THR_ZERO_EN_U** `[11]` (RW)
///
///   This is the enable bit for unit %s's zero comparator.
///
/// - **FILTER_EN_U** `[10]` (RW)
///
///   This is the enable bit for unit %s's input filter.
///
/// - **FILTER_THRES_U** `[9:0]` (RW)
///
///   This sets the maximum threshold, in APB_CLK cycles, for the filter.
///   Any pulses with width less than this will be ignored when the filter is enabled.
#[repr(transparent)]
pub struct U0Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0Conf0 {
    // ======== Field Constants ========

    /// **CH1_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CH1_LCTRL_MODE_U_OFFSET: u32 = 30;
    pub const CH1_LCTRL_MODE_U_MASK: u32 = 0xc0000000;

    /// **CH1_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const CH1_HCTRL_MODE_U_OFFSET: u32 = 28;
    pub const CH1_HCTRL_MODE_U_MASK: u32 = 0x30000000;

    /// **CH1_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CH1_POS_MODE_U_OFFSET: u32 = 26;
    pub const CH1_POS_MODE_U_MASK: u32 = 0x0c000000;

    /// **CH1_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const CH1_NEG_MODE_U_OFFSET: u32 = 24;
    pub const CH1_NEG_MODE_U_MASK: u32 = 0x03000000;

    /// **CH0_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CH0_LCTRL_MODE_U_OFFSET: u32 = 22;
    pub const CH0_LCTRL_MODE_U_MASK: u32 = 0x00c00000;

    /// **CH0_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const CH0_HCTRL_MODE_U_OFFSET: u32 = 20;
    pub const CH0_HCTRL_MODE_U_MASK: u32 = 0x00300000;

    /// **CH0_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CH0_POS_MODE_U_OFFSET: u32 = 18;
    pub const CH0_POS_MODE_U_MASK: u32 = 0x000c0000;

    /// **CH0_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CH0_NEG_MODE_U_OFFSET: u32 = 16;
    pub const CH0_NEG_MODE_U_MASK: u32 = 0x00030000;

    /// **THR_THRES1_EN_U**
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const THR_THRES1_EN_U_OFFSET: u32 = 15;
    pub const THR_THRES1_EN_U_MASK: u32 = 0x00008000;
    pub const THR_THRES1_EN_U: u32 = 1 << 15;

    /// **THR_THRES0_EN_U**
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const THR_THRES0_EN_U_OFFSET: u32 = 14;
    pub const THR_THRES0_EN_U_MASK: u32 = 0x00004000;
    pub const THR_THRES0_EN_U: u32 = 1 << 14;

    /// **THR_L_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const THR_L_LIM_EN_U_OFFSET: u32 = 13;
    pub const THR_L_LIM_EN_U_MASK: u32 = 0x00002000;
    pub const THR_L_LIM_EN_U: u32 = 1 << 13;

    /// **THR_H_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const THR_H_LIM_EN_U_OFFSET: u32 = 12;
    pub const THR_H_LIM_EN_U_MASK: u32 = 0x00001000;
    pub const THR_H_LIM_EN_U: u32 = 1 << 12;

    /// **THR_ZERO_EN_U**
    ///
    /// This is the enable bit for unit %s's zero comparator.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const THR_ZERO_EN_U_OFFSET: u32 = 11;
    pub const THR_ZERO_EN_U_MASK: u32 = 0x00000800;
    pub const THR_ZERO_EN_U: u32 = 1 << 11;

    /// **FILTER_EN_U**
    ///
    /// This is the enable bit for unit %s's input filter.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FILTER_EN_U_OFFSET: u32 = 10;
    pub const FILTER_EN_U_MASK: u32 = 0x00000400;
    pub const FILTER_EN_U: u32 = 1 << 10;

    /// **FILTER_THRES_U**
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const FILTER_THRES_U_OFFSET: u32 = 0;
    pub const FILTER_THRES_U_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_LCTRL_MODE_U_MASK) >> Self::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_LCTRL_MODE_U_MASK)
                | ((value << Self::CH1_LCTRL_MODE_U_OFFSET) & Self::CH1_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_HCTRL_MODE_U_MASK) >> Self::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_HCTRL_MODE_U_MASK)
                | ((value << Self::CH1_HCTRL_MODE_U_OFFSET) & Self::CH1_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_POS_MODE_U_MASK) >> Self::CH1_POS_MODE_U_OFFSET
    }

    /// Write **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_POS_MODE_U_MASK)
                | ((value << Self::CH1_POS_MODE_U_OFFSET) & Self::CH1_POS_MODE_U_MASK)
        });
    }

    /// **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_NEG_MODE_U_MASK) >> Self::CH1_NEG_MODE_U_OFFSET
    }

    /// Write **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_NEG_MODE_U_MASK)
                | ((value << Self::CH1_NEG_MODE_U_OFFSET) & Self::CH1_NEG_MODE_U_MASK)
        });
    }

    /// **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_LCTRL_MODE_U_MASK) >> Self::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_LCTRL_MODE_U_MASK)
                | ((value << Self::CH0_LCTRL_MODE_U_OFFSET) & Self::CH0_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_HCTRL_MODE_U_MASK) >> Self::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_HCTRL_MODE_U_MASK)
                | ((value << Self::CH0_HCTRL_MODE_U_OFFSET) & Self::CH0_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_POS_MODE_U_MASK) >> Self::CH0_POS_MODE_U_OFFSET
    }

    /// Write **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_POS_MODE_U_MASK)
                | ((value << Self::CH0_POS_MODE_U_OFFSET) & Self::CH0_POS_MODE_U_MASK)
        });
    }

    /// **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_NEG_MODE_U_MASK) >> Self::CH0_NEG_MODE_U_OFFSET
    }

    /// Write **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_NEG_MODE_U_MASK)
                | ((value << Self::CH0_NEG_MODE_U_OFFSET) & Self::CH0_NEG_MODE_U_MASK)
        });
    }

    /// **THR_THRES1_EN_U** `[15]` (RW)
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES1_EN_U_MASK) != 0
    }

    /// Set THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES1_EN_U_MASK);
    }

    /// Clear THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES1_EN_U_MASK);
    }

    /// **THR_THRES0_EN_U** `[14]` (RW)
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES0_EN_U_MASK) != 0
    }

    /// Set THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES0_EN_U_MASK);
    }

    /// Clear THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES0_EN_U_MASK);
    }

    /// **THR_L_LIM_EN_U** `[13]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Set THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_L_LIM_EN_U_MASK);
    }

    /// Clear THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_L_LIM_EN_U_MASK);
    }

    /// **THR_H_LIM_EN_U** `[12]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Set THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_H_LIM_EN_U_MASK);
    }

    /// Clear THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_H_LIM_EN_U_MASK);
    }

    /// **THR_ZERO_EN_U** `[11]` (RW)
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_ZERO_EN_U_MASK) != 0
    }

    /// Set THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_ZERO_EN_U_MASK);
    }

    /// Clear THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_ZERO_EN_U_MASK);
    }

    /// **FILTER_EN_U** `[10]` (RW)
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub fn filter_en_u(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_EN_U_MASK) != 0
    }

    /// Set FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_en_u(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_EN_U_MASK);
    }

    /// Clear FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_EN_U_MASK);
    }

    /// **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn filter_thres_u(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_THRES_U_MASK) >> Self::FILTER_THRES_U_OFFSET
    }

    /// Write **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn write_filter_thres_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_THRES_U_MASK)
                | ((value << Self::FILTER_THRES_U_OFFSET) & Self::FILTER_THRES_U_MASK)
        });
    }
}

/// U0_CONF0 value type for batch read/modify/write operations
///
/// Use `u0conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0Conf0Val {
    bits: u32,
}

impl U0Conf0Val {
    /// Create from raw bits
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
        Self { bits: 0x00003c10 }
    }

    /// Read **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH1_LCTRL_MODE_U_MASK) >> U0Conf0::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH1_HCTRL_MODE_U_MASK) >> U0Conf0::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_pos_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH1_POS_MODE_U_MASK) >> U0Conf0::CH1_POS_MODE_U_OFFSET
    }

    /// Read **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_neg_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH1_NEG_MODE_U_MASK) >> U0Conf0::CH1_NEG_MODE_U_OFFSET
    }

    /// Read **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH0_LCTRL_MODE_U_MASK) >> U0Conf0::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH0_HCTRL_MODE_U_MASK) >> U0Conf0::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_pos_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH0_POS_MODE_U_MASK) >> U0Conf0::CH0_POS_MODE_U_OFFSET
    }

    /// Read **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_neg_mode_u(&self) -> u32 {
        (self.bits & U0Conf0::CH0_NEG_MODE_U_MASK) >> U0Conf0::CH0_NEG_MODE_U_OFFSET
    }

    /// Read **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn thr_thres1_en_u(&self) -> bool {
        (self.bits & U0Conf0::THR_THRES1_EN_U_MASK) != 0
    }

    /// Read **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn thr_thres0_en_u(&self) -> bool {
        (self.bits & U0Conf0::THR_THRES0_EN_U_MASK) != 0
    }

    /// Read **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn thr_l_lim_en_u(&self) -> bool {
        (self.bits & U0Conf0::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn thr_h_lim_en_u(&self) -> bool {
        (self.bits & U0Conf0::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn thr_zero_en_u(&self) -> bool {
        (self.bits & U0Conf0::THR_ZERO_EN_U_MASK) != 0
    }

    /// Read **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn filter_en_u(&self) -> bool {
        (self.bits & U0Conf0::FILTER_EN_U_MASK) != 0
    }

    /// Read **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn filter_thres_u(&self) -> u32 {
        (self.bits & U0Conf0::FILTER_THRES_U_MASK) >> U0Conf0::FILTER_THRES_U_OFFSET
    }

    /// Set **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH1_LCTRL_MODE_U_MASK)
                | ((value << U0Conf0::CH1_LCTRL_MODE_U_OFFSET) & U0Conf0::CH1_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH1_HCTRL_MODE_U_MASK)
                | ((value << U0Conf0::CH1_HCTRL_MODE_U_OFFSET) & U0Conf0::CH1_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH1_POS_MODE_U_MASK)
                | ((value << U0Conf0::CH1_POS_MODE_U_OFFSET) & U0Conf0::CH1_POS_MODE_U_MASK),
        }
    }

    /// Set **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH1_NEG_MODE_U_MASK)
                | ((value << U0Conf0::CH1_NEG_MODE_U_OFFSET) & U0Conf0::CH1_NEG_MODE_U_MASK),
        }
    }

    /// Set **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH0_LCTRL_MODE_U_MASK)
                | ((value << U0Conf0::CH0_LCTRL_MODE_U_OFFSET) & U0Conf0::CH0_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH0_HCTRL_MODE_U_MASK)
                | ((value << U0Conf0::CH0_HCTRL_MODE_U_OFFSET) & U0Conf0::CH0_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH0_POS_MODE_U_MASK)
                | ((value << U0Conf0::CH0_POS_MODE_U_OFFSET) & U0Conf0::CH0_POS_MODE_U_MASK),
        }
    }

    /// Set **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::CH0_NEG_MODE_U_MASK)
                | ((value << U0Conf0::CH0_NEG_MODE_U_OFFSET) & U0Conf0::CH0_NEG_MODE_U_MASK),
        }
    }

    /// Set **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn with_thr_thres1_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::THR_THRES1_EN_U_MASK
            } else {
                self.bits & !U0Conf0::THR_THRES1_EN_U_MASK
            },
        }
    }

    /// Set **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn with_thr_thres0_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::THR_THRES0_EN_U_MASK
            } else {
                self.bits & !U0Conf0::THR_THRES0_EN_U_MASK
            },
        }
    }

    /// Set **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn with_thr_l_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::THR_L_LIM_EN_U_MASK
            } else {
                self.bits & !U0Conf0::THR_L_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn with_thr_h_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::THR_H_LIM_EN_U_MASK
            } else {
                self.bits & !U0Conf0::THR_H_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn with_thr_zero_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::THR_ZERO_EN_U_MASK
            } else {
                self.bits & !U0Conf0::THR_ZERO_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn with_filter_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U0Conf0::FILTER_EN_U_MASK
            } else {
                self.bits & !U0Conf0::FILTER_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn with_filter_thres_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf0::FILTER_THRES_U_MASK)
                | ((value << U0Conf0::FILTER_THRES_U_OFFSET) & U0Conf0::FILTER_THRES_U_MASK),
        }
    }
}

impl Default for U0Conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl U0Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0Conf0Val {
        U0Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U0Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U0Conf0Val) -> U0Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U0_CONF1 Register
///
/// Configuration register 1 for unit 0
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CNT_THRES1_U** `[31:16]` (RW)
///
///   This register is used to configure the thres1 value for unit %s.
///
/// - **CNT_THRES0_U** `[15:0]` (RW)
///
///   This register is used to configure the thres0 value for unit %s.
#[repr(transparent)]
pub struct U0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0Conf1 {
    // ======== Field Constants ========

    /// **CNT_THRES1_U**
    ///
    /// This register is used to configure the thres1 value for unit %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_THRES1_U_OFFSET: u32 = 16;
    pub const CNT_THRES1_U_MASK: u32 = 0xffff0000;

    /// **CNT_THRES0_U**
    ///
    /// This register is used to configure the thres0 value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_THRES0_U_OFFSET: u32 = 0;
    pub const CNT_THRES0_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres1_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES1_U_MASK) >> Self::CNT_THRES1_U_OFFSET
    }

    /// Write **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres1_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES1_U_MASK)
                | ((value << Self::CNT_THRES1_U_OFFSET) & Self::CNT_THRES1_U_MASK)
        });
    }

    /// **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres0_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES0_U_MASK) >> Self::CNT_THRES0_U_OFFSET
    }

    /// Write **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres0_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES0_U_MASK)
                | ((value << Self::CNT_THRES0_U_OFFSET) & Self::CNT_THRES0_U_MASK)
        });
    }
}

/// U0_CONF1 value type for batch read/modify/write operations
///
/// Use `u0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0Conf1Val {
    bits: u32,
}

impl U0Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres1_u(&self) -> u32 {
        (self.bits & U0Conf1::CNT_THRES1_U_MASK) >> U0Conf1::CNT_THRES1_U_OFFSET
    }

    /// Read **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres0_u(&self) -> u32 {
        (self.bits & U0Conf1::CNT_THRES0_U_MASK) >> U0Conf1::CNT_THRES0_U_OFFSET
    }

    /// Set **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres1_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf1::CNT_THRES1_U_MASK)
                | ((value << U0Conf1::CNT_THRES1_U_OFFSET) & U0Conf1::CNT_THRES1_U_MASK),
        }
    }

    /// Set **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres0_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf1::CNT_THRES0_U_MASK)
                | ((value << U0Conf1::CNT_THRES0_U_OFFSET) & U0Conf1::CNT_THRES0_U_MASK),
        }
    }
}

impl Default for U0Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0Conf1Val {
        U0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U0Conf1Val) -> U0Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U0_CONF2 Register
///
/// Configuration register 2 for unit 0
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CNT_L_LIM_U** `[31:16]` (RW)
///
///   This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
///
/// - **CNT_H_LIM_U** `[15:0]` (RW)
///
///   This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
#[repr(transparent)]
pub struct U0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0Conf2 {
    // ======== Field Constants ========

    /// **CNT_L_LIM_U**
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_L_LIM_U_OFFSET: u32 = 16;
    pub const CNT_L_LIM_U_MASK: u32 = 0xffff0000;

    /// **CNT_H_LIM_U**
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_H_LIM_U_OFFSET: u32 = 0;
    pub const CNT_H_LIM_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_L_LIM_U_MASK) >> Self::CNT_L_LIM_U_OFFSET
    }

    /// Write **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_l_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_L_LIM_U_MASK)
                | ((value << Self::CNT_L_LIM_U_OFFSET) & Self::CNT_L_LIM_U_MASK)
        });
    }

    /// **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_H_LIM_U_MASK) >> Self::CNT_H_LIM_U_OFFSET
    }

    /// Write **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_h_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_H_LIM_U_MASK)
                | ((value << Self::CNT_H_LIM_U_OFFSET) & Self::CNT_H_LIM_U_MASK)
        });
    }
}

/// U0_CONF2 value type for batch read/modify/write operations
///
/// Use `u0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0Conf2Val {
    bits: u32,
}

impl U0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_l_lim_u(&self) -> u32 {
        (self.bits & U0Conf2::CNT_L_LIM_U_MASK) >> U0Conf2::CNT_L_LIM_U_OFFSET
    }

    /// Read **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_h_lim_u(&self) -> u32 {
        (self.bits & U0Conf2::CNT_H_LIM_U_MASK) >> U0Conf2::CNT_H_LIM_U_OFFSET
    }

    /// Set **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_l_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf2::CNT_L_LIM_U_MASK)
                | ((value << U0Conf2::CNT_L_LIM_U_OFFSET) & U0Conf2::CNT_L_LIM_U_MASK),
        }
    }

    /// Set **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_h_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0Conf2::CNT_H_LIM_U_MASK)
                | ((value << U0Conf2::CNT_H_LIM_U_OFFSET) & U0Conf2::CNT_H_LIM_U_MASK),
        }
    }
}

impl Default for U0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0Conf2Val {
        U0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U0Conf2Val) -> U0Conf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U1_CONF0 Register
///
/// Configuration register 0 for unit 1
///
/// Address offset: 0x000c
/// Reset value: 0x00003c10
///
/// ## Fields
///
/// - **CH1_LCTRL_MODE_U** `[31:30]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_HCTRL_MODE_U** `[29:28]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_POS_MODE_U** `[27:26]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a positive edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH1_NEG_MODE_U** `[25:24]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a negative edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH0_LCTRL_MODE_U** `[23:22]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_HCTRL_MODE_U** `[21:20]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_POS_MODE_U** `[19:18]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a positive edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **CH0_NEG_MODE_U** `[17:16]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a negative edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **THR_THRES1_EN_U** `[15]` (RW)
///
///   This is the enable bit for unit %s's thres1 comparator.
///
/// - **THR_THRES0_EN_U** `[14]` (RW)
///
///   This is the enable bit for unit %s's thres0 comparator.
///
/// - **THR_L_LIM_EN_U** `[13]` (RW)
///
///   This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
///
/// - **THR_H_LIM_EN_U** `[12]` (RW)
///
///   This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
///
/// - **THR_ZERO_EN_U** `[11]` (RW)
///
///   This is the enable bit for unit %s's zero comparator.
///
/// - **FILTER_EN_U** `[10]` (RW)
///
///   This is the enable bit for unit %s's input filter.
///
/// - **FILTER_THRES_U** `[9:0]` (RW)
///
///   This sets the maximum threshold, in APB_CLK cycles, for the filter.
///   Any pulses with width less than this will be ignored when the filter is enabled.
#[repr(transparent)]
pub struct U1Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1Conf0 {
    // ======== Field Constants ========

    /// **CH1_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CH1_LCTRL_MODE_U_OFFSET: u32 = 30;
    pub const CH1_LCTRL_MODE_U_MASK: u32 = 0xc0000000;

    /// **CH1_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const CH1_HCTRL_MODE_U_OFFSET: u32 = 28;
    pub const CH1_HCTRL_MODE_U_MASK: u32 = 0x30000000;

    /// **CH1_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CH1_POS_MODE_U_OFFSET: u32 = 26;
    pub const CH1_POS_MODE_U_MASK: u32 = 0x0c000000;

    /// **CH1_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const CH1_NEG_MODE_U_OFFSET: u32 = 24;
    pub const CH1_NEG_MODE_U_MASK: u32 = 0x03000000;

    /// **CH0_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CH0_LCTRL_MODE_U_OFFSET: u32 = 22;
    pub const CH0_LCTRL_MODE_U_MASK: u32 = 0x00c00000;

    /// **CH0_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const CH0_HCTRL_MODE_U_OFFSET: u32 = 20;
    pub const CH0_HCTRL_MODE_U_MASK: u32 = 0x00300000;

    /// **CH0_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CH0_POS_MODE_U_OFFSET: u32 = 18;
    pub const CH0_POS_MODE_U_MASK: u32 = 0x000c0000;

    /// **CH0_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CH0_NEG_MODE_U_OFFSET: u32 = 16;
    pub const CH0_NEG_MODE_U_MASK: u32 = 0x00030000;

    /// **THR_THRES1_EN_U**
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const THR_THRES1_EN_U_OFFSET: u32 = 15;
    pub const THR_THRES1_EN_U_MASK: u32 = 0x00008000;
    pub const THR_THRES1_EN_U: u32 = 1 << 15;

    /// **THR_THRES0_EN_U**
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const THR_THRES0_EN_U_OFFSET: u32 = 14;
    pub const THR_THRES0_EN_U_MASK: u32 = 0x00004000;
    pub const THR_THRES0_EN_U: u32 = 1 << 14;

    /// **THR_L_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const THR_L_LIM_EN_U_OFFSET: u32 = 13;
    pub const THR_L_LIM_EN_U_MASK: u32 = 0x00002000;
    pub const THR_L_LIM_EN_U: u32 = 1 << 13;

    /// **THR_H_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const THR_H_LIM_EN_U_OFFSET: u32 = 12;
    pub const THR_H_LIM_EN_U_MASK: u32 = 0x00001000;
    pub const THR_H_LIM_EN_U: u32 = 1 << 12;

    /// **THR_ZERO_EN_U**
    ///
    /// This is the enable bit for unit %s's zero comparator.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const THR_ZERO_EN_U_OFFSET: u32 = 11;
    pub const THR_ZERO_EN_U_MASK: u32 = 0x00000800;
    pub const THR_ZERO_EN_U: u32 = 1 << 11;

    /// **FILTER_EN_U**
    ///
    /// This is the enable bit for unit %s's input filter.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FILTER_EN_U_OFFSET: u32 = 10;
    pub const FILTER_EN_U_MASK: u32 = 0x00000400;
    pub const FILTER_EN_U: u32 = 1 << 10;

    /// **FILTER_THRES_U**
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const FILTER_THRES_U_OFFSET: u32 = 0;
    pub const FILTER_THRES_U_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_LCTRL_MODE_U_MASK) >> Self::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_LCTRL_MODE_U_MASK)
                | ((value << Self::CH1_LCTRL_MODE_U_OFFSET) & Self::CH1_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_HCTRL_MODE_U_MASK) >> Self::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_HCTRL_MODE_U_MASK)
                | ((value << Self::CH1_HCTRL_MODE_U_OFFSET) & Self::CH1_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_POS_MODE_U_MASK) >> Self::CH1_POS_MODE_U_OFFSET
    }

    /// Write **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_POS_MODE_U_MASK)
                | ((value << Self::CH1_POS_MODE_U_OFFSET) & Self::CH1_POS_MODE_U_MASK)
        });
    }

    /// **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_NEG_MODE_U_MASK) >> Self::CH1_NEG_MODE_U_OFFSET
    }

    /// Write **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_NEG_MODE_U_MASK)
                | ((value << Self::CH1_NEG_MODE_U_OFFSET) & Self::CH1_NEG_MODE_U_MASK)
        });
    }

    /// **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_LCTRL_MODE_U_MASK) >> Self::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_LCTRL_MODE_U_MASK)
                | ((value << Self::CH0_LCTRL_MODE_U_OFFSET) & Self::CH0_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_HCTRL_MODE_U_MASK) >> Self::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_HCTRL_MODE_U_MASK)
                | ((value << Self::CH0_HCTRL_MODE_U_OFFSET) & Self::CH0_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_POS_MODE_U_MASK) >> Self::CH0_POS_MODE_U_OFFSET
    }

    /// Write **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_POS_MODE_U_MASK)
                | ((value << Self::CH0_POS_MODE_U_OFFSET) & Self::CH0_POS_MODE_U_MASK)
        });
    }

    /// **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_NEG_MODE_U_MASK) >> Self::CH0_NEG_MODE_U_OFFSET
    }

    /// Write **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_NEG_MODE_U_MASK)
                | ((value << Self::CH0_NEG_MODE_U_OFFSET) & Self::CH0_NEG_MODE_U_MASK)
        });
    }

    /// **THR_THRES1_EN_U** `[15]` (RW)
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES1_EN_U_MASK) != 0
    }

    /// Set THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES1_EN_U_MASK);
    }

    /// Clear THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES1_EN_U_MASK);
    }

    /// **THR_THRES0_EN_U** `[14]` (RW)
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES0_EN_U_MASK) != 0
    }

    /// Set THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES0_EN_U_MASK);
    }

    /// Clear THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES0_EN_U_MASK);
    }

    /// **THR_L_LIM_EN_U** `[13]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Set THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_L_LIM_EN_U_MASK);
    }

    /// Clear THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_L_LIM_EN_U_MASK);
    }

    /// **THR_H_LIM_EN_U** `[12]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Set THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_H_LIM_EN_U_MASK);
    }

    /// Clear THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_H_LIM_EN_U_MASK);
    }

    /// **THR_ZERO_EN_U** `[11]` (RW)
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_ZERO_EN_U_MASK) != 0
    }

    /// Set THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_ZERO_EN_U_MASK);
    }

    /// Clear THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_ZERO_EN_U_MASK);
    }

    /// **FILTER_EN_U** `[10]` (RW)
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub fn filter_en_u(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_EN_U_MASK) != 0
    }

    /// Set FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_en_u(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_EN_U_MASK);
    }

    /// Clear FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_EN_U_MASK);
    }

    /// **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn filter_thres_u(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_THRES_U_MASK) >> Self::FILTER_THRES_U_OFFSET
    }

    /// Write **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn write_filter_thres_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_THRES_U_MASK)
                | ((value << Self::FILTER_THRES_U_OFFSET) & Self::FILTER_THRES_U_MASK)
        });
    }
}

/// U1_CONF0 value type for batch read/modify/write operations
///
/// Use `u1conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1Conf0Val {
    bits: u32,
}

impl U1Conf0Val {
    /// Create from raw bits
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
        Self { bits: 0x00003c10 }
    }

    /// Read **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH1_LCTRL_MODE_U_MASK) >> U1Conf0::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH1_HCTRL_MODE_U_MASK) >> U1Conf0::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_pos_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH1_POS_MODE_U_MASK) >> U1Conf0::CH1_POS_MODE_U_OFFSET
    }

    /// Read **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_neg_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH1_NEG_MODE_U_MASK) >> U1Conf0::CH1_NEG_MODE_U_OFFSET
    }

    /// Read **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH0_LCTRL_MODE_U_MASK) >> U1Conf0::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH0_HCTRL_MODE_U_MASK) >> U1Conf0::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_pos_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH0_POS_MODE_U_MASK) >> U1Conf0::CH0_POS_MODE_U_OFFSET
    }

    /// Read **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_neg_mode_u(&self) -> u32 {
        (self.bits & U1Conf0::CH0_NEG_MODE_U_MASK) >> U1Conf0::CH0_NEG_MODE_U_OFFSET
    }

    /// Read **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn thr_thres1_en_u(&self) -> bool {
        (self.bits & U1Conf0::THR_THRES1_EN_U_MASK) != 0
    }

    /// Read **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn thr_thres0_en_u(&self) -> bool {
        (self.bits & U1Conf0::THR_THRES0_EN_U_MASK) != 0
    }

    /// Read **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn thr_l_lim_en_u(&self) -> bool {
        (self.bits & U1Conf0::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn thr_h_lim_en_u(&self) -> bool {
        (self.bits & U1Conf0::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn thr_zero_en_u(&self) -> bool {
        (self.bits & U1Conf0::THR_ZERO_EN_U_MASK) != 0
    }

    /// Read **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn filter_en_u(&self) -> bool {
        (self.bits & U1Conf0::FILTER_EN_U_MASK) != 0
    }

    /// Read **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn filter_thres_u(&self) -> u32 {
        (self.bits & U1Conf0::FILTER_THRES_U_MASK) >> U1Conf0::FILTER_THRES_U_OFFSET
    }

    /// Set **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH1_LCTRL_MODE_U_MASK)
                | ((value << U1Conf0::CH1_LCTRL_MODE_U_OFFSET) & U1Conf0::CH1_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH1_HCTRL_MODE_U_MASK)
                | ((value << U1Conf0::CH1_HCTRL_MODE_U_OFFSET) & U1Conf0::CH1_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH1_POS_MODE_U_MASK)
                | ((value << U1Conf0::CH1_POS_MODE_U_OFFSET) & U1Conf0::CH1_POS_MODE_U_MASK),
        }
    }

    /// Set **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH1_NEG_MODE_U_MASK)
                | ((value << U1Conf0::CH1_NEG_MODE_U_OFFSET) & U1Conf0::CH1_NEG_MODE_U_MASK),
        }
    }

    /// Set **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH0_LCTRL_MODE_U_MASK)
                | ((value << U1Conf0::CH0_LCTRL_MODE_U_OFFSET) & U1Conf0::CH0_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH0_HCTRL_MODE_U_MASK)
                | ((value << U1Conf0::CH0_HCTRL_MODE_U_OFFSET) & U1Conf0::CH0_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH0_POS_MODE_U_MASK)
                | ((value << U1Conf0::CH0_POS_MODE_U_OFFSET) & U1Conf0::CH0_POS_MODE_U_MASK),
        }
    }

    /// Set **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::CH0_NEG_MODE_U_MASK)
                | ((value << U1Conf0::CH0_NEG_MODE_U_OFFSET) & U1Conf0::CH0_NEG_MODE_U_MASK),
        }
    }

    /// Set **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn with_thr_thres1_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::THR_THRES1_EN_U_MASK
            } else {
                self.bits & !U1Conf0::THR_THRES1_EN_U_MASK
            },
        }
    }

    /// Set **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn with_thr_thres0_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::THR_THRES0_EN_U_MASK
            } else {
                self.bits & !U1Conf0::THR_THRES0_EN_U_MASK
            },
        }
    }

    /// Set **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn with_thr_l_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::THR_L_LIM_EN_U_MASK
            } else {
                self.bits & !U1Conf0::THR_L_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn with_thr_h_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::THR_H_LIM_EN_U_MASK
            } else {
                self.bits & !U1Conf0::THR_H_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn with_thr_zero_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::THR_ZERO_EN_U_MASK
            } else {
                self.bits & !U1Conf0::THR_ZERO_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn with_filter_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U1Conf0::FILTER_EN_U_MASK
            } else {
                self.bits & !U1Conf0::FILTER_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn with_filter_thres_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf0::FILTER_THRES_U_MASK)
                | ((value << U1Conf0::FILTER_THRES_U_OFFSET) & U1Conf0::FILTER_THRES_U_MASK),
        }
    }
}

impl Default for U1Conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl U1Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1Conf0Val {
        U1Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U1Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U1Conf0Val) -> U1Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U1_CONF1 Register
///
/// Configuration register 1 for unit 1
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **CNT_THRES1_U** `[31:16]` (RW)
///
///   This register is used to configure the thres1 value for unit %s.
///
/// - **CNT_THRES0_U** `[15:0]` (RW)
///
///   This register is used to configure the thres0 value for unit %s.
#[repr(transparent)]
pub struct U1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1Conf1 {
    // ======== Field Constants ========

    /// **CNT_THRES1_U**
    ///
    /// This register is used to configure the thres1 value for unit %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_THRES1_U_OFFSET: u32 = 16;
    pub const CNT_THRES1_U_MASK: u32 = 0xffff0000;

    /// **CNT_THRES0_U**
    ///
    /// This register is used to configure the thres0 value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_THRES0_U_OFFSET: u32 = 0;
    pub const CNT_THRES0_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres1_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES1_U_MASK) >> Self::CNT_THRES1_U_OFFSET
    }

    /// Write **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres1_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES1_U_MASK)
                | ((value << Self::CNT_THRES1_U_OFFSET) & Self::CNT_THRES1_U_MASK)
        });
    }

    /// **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres0_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES0_U_MASK) >> Self::CNT_THRES0_U_OFFSET
    }

    /// Write **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres0_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES0_U_MASK)
                | ((value << Self::CNT_THRES0_U_OFFSET) & Self::CNT_THRES0_U_MASK)
        });
    }
}

/// U1_CONF1 value type for batch read/modify/write operations
///
/// Use `u1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1Conf1Val {
    bits: u32,
}

impl U1Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres1_u(&self) -> u32 {
        (self.bits & U1Conf1::CNT_THRES1_U_MASK) >> U1Conf1::CNT_THRES1_U_OFFSET
    }

    /// Read **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres0_u(&self) -> u32 {
        (self.bits & U1Conf1::CNT_THRES0_U_MASK) >> U1Conf1::CNT_THRES0_U_OFFSET
    }

    /// Set **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres1_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf1::CNT_THRES1_U_MASK)
                | ((value << U1Conf1::CNT_THRES1_U_OFFSET) & U1Conf1::CNT_THRES1_U_MASK),
        }
    }

    /// Set **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres0_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf1::CNT_THRES0_U_MASK)
                | ((value << U1Conf1::CNT_THRES0_U_OFFSET) & U1Conf1::CNT_THRES0_U_MASK),
        }
    }
}

impl Default for U1Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1Conf1Val {
        U1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U1Conf1Val) -> U1Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U1_CONF2 Register
///
/// Configuration register 2 for unit 1
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CNT_L_LIM_U** `[31:16]` (RW)
///
///   This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
///
/// - **CNT_H_LIM_U** `[15:0]` (RW)
///
///   This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
#[repr(transparent)]
pub struct U1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1Conf2 {
    // ======== Field Constants ========

    /// **CNT_L_LIM_U**
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_L_LIM_U_OFFSET: u32 = 16;
    pub const CNT_L_LIM_U_MASK: u32 = 0xffff0000;

    /// **CNT_H_LIM_U**
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_H_LIM_U_OFFSET: u32 = 0;
    pub const CNT_H_LIM_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_L_LIM_U_MASK) >> Self::CNT_L_LIM_U_OFFSET
    }

    /// Write **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_l_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_L_LIM_U_MASK)
                | ((value << Self::CNT_L_LIM_U_OFFSET) & Self::CNT_L_LIM_U_MASK)
        });
    }

    /// **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_H_LIM_U_MASK) >> Self::CNT_H_LIM_U_OFFSET
    }

    /// Write **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_h_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_H_LIM_U_MASK)
                | ((value << Self::CNT_H_LIM_U_OFFSET) & Self::CNT_H_LIM_U_MASK)
        });
    }
}

/// U1_CONF2 value type for batch read/modify/write operations
///
/// Use `u1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1Conf2Val {
    bits: u32,
}

impl U1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_l_lim_u(&self) -> u32 {
        (self.bits & U1Conf2::CNT_L_LIM_U_MASK) >> U1Conf2::CNT_L_LIM_U_OFFSET
    }

    /// Read **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_h_lim_u(&self) -> u32 {
        (self.bits & U1Conf2::CNT_H_LIM_U_MASK) >> U1Conf2::CNT_H_LIM_U_OFFSET
    }

    /// Set **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_l_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf2::CNT_L_LIM_U_MASK)
                | ((value << U1Conf2::CNT_L_LIM_U_OFFSET) & U1Conf2::CNT_L_LIM_U_MASK),
        }
    }

    /// Set **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_h_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1Conf2::CNT_H_LIM_U_MASK)
                | ((value << U1Conf2::CNT_H_LIM_U_OFFSET) & U1Conf2::CNT_H_LIM_U_MASK),
        }
    }
}

impl Default for U1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1Conf2Val {
        U1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U1Conf2Val) -> U1Conf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U2_CONF0 Register
///
/// Configuration register 0 for unit 2
///
/// Address offset: 0x0018
/// Reset value: 0x00003c10
///
/// ## Fields
///
/// - **CH1_LCTRL_MODE_U** `[31:30]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_HCTRL_MODE_U** `[29:28]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_POS_MODE_U** `[27:26]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a positive edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH1_NEG_MODE_U** `[25:24]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a negative edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH0_LCTRL_MODE_U** `[23:22]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_HCTRL_MODE_U** `[21:20]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_POS_MODE_U** `[19:18]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a positive edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **CH0_NEG_MODE_U** `[17:16]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a negative edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **THR_THRES1_EN_U** `[15]` (RW)
///
///   This is the enable bit for unit %s's thres1 comparator.
///
/// - **THR_THRES0_EN_U** `[14]` (RW)
///
///   This is the enable bit for unit %s's thres0 comparator.
///
/// - **THR_L_LIM_EN_U** `[13]` (RW)
///
///   This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
///
/// - **THR_H_LIM_EN_U** `[12]` (RW)
///
///   This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
///
/// - **THR_ZERO_EN_U** `[11]` (RW)
///
///   This is the enable bit for unit %s's zero comparator.
///
/// - **FILTER_EN_U** `[10]` (RW)
///
///   This is the enable bit for unit %s's input filter.
///
/// - **FILTER_THRES_U** `[9:0]` (RW)
///
///   This sets the maximum threshold, in APB_CLK cycles, for the filter.
///   Any pulses with width less than this will be ignored when the filter is enabled.
#[repr(transparent)]
pub struct U2Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2Conf0 {
    // ======== Field Constants ========

    /// **CH1_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CH1_LCTRL_MODE_U_OFFSET: u32 = 30;
    pub const CH1_LCTRL_MODE_U_MASK: u32 = 0xc0000000;

    /// **CH1_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const CH1_HCTRL_MODE_U_OFFSET: u32 = 28;
    pub const CH1_HCTRL_MODE_U_MASK: u32 = 0x30000000;

    /// **CH1_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CH1_POS_MODE_U_OFFSET: u32 = 26;
    pub const CH1_POS_MODE_U_MASK: u32 = 0x0c000000;

    /// **CH1_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const CH1_NEG_MODE_U_OFFSET: u32 = 24;
    pub const CH1_NEG_MODE_U_MASK: u32 = 0x03000000;

    /// **CH0_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CH0_LCTRL_MODE_U_OFFSET: u32 = 22;
    pub const CH0_LCTRL_MODE_U_MASK: u32 = 0x00c00000;

    /// **CH0_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const CH0_HCTRL_MODE_U_OFFSET: u32 = 20;
    pub const CH0_HCTRL_MODE_U_MASK: u32 = 0x00300000;

    /// **CH0_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CH0_POS_MODE_U_OFFSET: u32 = 18;
    pub const CH0_POS_MODE_U_MASK: u32 = 0x000c0000;

    /// **CH0_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CH0_NEG_MODE_U_OFFSET: u32 = 16;
    pub const CH0_NEG_MODE_U_MASK: u32 = 0x00030000;

    /// **THR_THRES1_EN_U**
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const THR_THRES1_EN_U_OFFSET: u32 = 15;
    pub const THR_THRES1_EN_U_MASK: u32 = 0x00008000;
    pub const THR_THRES1_EN_U: u32 = 1 << 15;

    /// **THR_THRES0_EN_U**
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const THR_THRES0_EN_U_OFFSET: u32 = 14;
    pub const THR_THRES0_EN_U_MASK: u32 = 0x00004000;
    pub const THR_THRES0_EN_U: u32 = 1 << 14;

    /// **THR_L_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const THR_L_LIM_EN_U_OFFSET: u32 = 13;
    pub const THR_L_LIM_EN_U_MASK: u32 = 0x00002000;
    pub const THR_L_LIM_EN_U: u32 = 1 << 13;

    /// **THR_H_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const THR_H_LIM_EN_U_OFFSET: u32 = 12;
    pub const THR_H_LIM_EN_U_MASK: u32 = 0x00001000;
    pub const THR_H_LIM_EN_U: u32 = 1 << 12;

    /// **THR_ZERO_EN_U**
    ///
    /// This is the enable bit for unit %s's zero comparator.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const THR_ZERO_EN_U_OFFSET: u32 = 11;
    pub const THR_ZERO_EN_U_MASK: u32 = 0x00000800;
    pub const THR_ZERO_EN_U: u32 = 1 << 11;

    /// **FILTER_EN_U**
    ///
    /// This is the enable bit for unit %s's input filter.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FILTER_EN_U_OFFSET: u32 = 10;
    pub const FILTER_EN_U_MASK: u32 = 0x00000400;
    pub const FILTER_EN_U: u32 = 1 << 10;

    /// **FILTER_THRES_U**
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const FILTER_THRES_U_OFFSET: u32 = 0;
    pub const FILTER_THRES_U_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_LCTRL_MODE_U_MASK) >> Self::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_LCTRL_MODE_U_MASK)
                | ((value << Self::CH1_LCTRL_MODE_U_OFFSET) & Self::CH1_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_HCTRL_MODE_U_MASK) >> Self::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_HCTRL_MODE_U_MASK)
                | ((value << Self::CH1_HCTRL_MODE_U_OFFSET) & Self::CH1_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_POS_MODE_U_MASK) >> Self::CH1_POS_MODE_U_OFFSET
    }

    /// Write **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_POS_MODE_U_MASK)
                | ((value << Self::CH1_POS_MODE_U_OFFSET) & Self::CH1_POS_MODE_U_MASK)
        });
    }

    /// **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_NEG_MODE_U_MASK) >> Self::CH1_NEG_MODE_U_OFFSET
    }

    /// Write **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_NEG_MODE_U_MASK)
                | ((value << Self::CH1_NEG_MODE_U_OFFSET) & Self::CH1_NEG_MODE_U_MASK)
        });
    }

    /// **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_LCTRL_MODE_U_MASK) >> Self::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_LCTRL_MODE_U_MASK)
                | ((value << Self::CH0_LCTRL_MODE_U_OFFSET) & Self::CH0_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_HCTRL_MODE_U_MASK) >> Self::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_HCTRL_MODE_U_MASK)
                | ((value << Self::CH0_HCTRL_MODE_U_OFFSET) & Self::CH0_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_POS_MODE_U_MASK) >> Self::CH0_POS_MODE_U_OFFSET
    }

    /// Write **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_POS_MODE_U_MASK)
                | ((value << Self::CH0_POS_MODE_U_OFFSET) & Self::CH0_POS_MODE_U_MASK)
        });
    }

    /// **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_NEG_MODE_U_MASK) >> Self::CH0_NEG_MODE_U_OFFSET
    }

    /// Write **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_NEG_MODE_U_MASK)
                | ((value << Self::CH0_NEG_MODE_U_OFFSET) & Self::CH0_NEG_MODE_U_MASK)
        });
    }

    /// **THR_THRES1_EN_U** `[15]` (RW)
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES1_EN_U_MASK) != 0
    }

    /// Set THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES1_EN_U_MASK);
    }

    /// Clear THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES1_EN_U_MASK);
    }

    /// **THR_THRES0_EN_U** `[14]` (RW)
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES0_EN_U_MASK) != 0
    }

    /// Set THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES0_EN_U_MASK);
    }

    /// Clear THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES0_EN_U_MASK);
    }

    /// **THR_L_LIM_EN_U** `[13]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Set THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_L_LIM_EN_U_MASK);
    }

    /// Clear THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_L_LIM_EN_U_MASK);
    }

    /// **THR_H_LIM_EN_U** `[12]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Set THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_H_LIM_EN_U_MASK);
    }

    /// Clear THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_H_LIM_EN_U_MASK);
    }

    /// **THR_ZERO_EN_U** `[11]` (RW)
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_ZERO_EN_U_MASK) != 0
    }

    /// Set THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_ZERO_EN_U_MASK);
    }

    /// Clear THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_ZERO_EN_U_MASK);
    }

    /// **FILTER_EN_U** `[10]` (RW)
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub fn filter_en_u(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_EN_U_MASK) != 0
    }

    /// Set FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_en_u(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_EN_U_MASK);
    }

    /// Clear FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_EN_U_MASK);
    }

    /// **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn filter_thres_u(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_THRES_U_MASK) >> Self::FILTER_THRES_U_OFFSET
    }

    /// Write **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn write_filter_thres_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_THRES_U_MASK)
                | ((value << Self::FILTER_THRES_U_OFFSET) & Self::FILTER_THRES_U_MASK)
        });
    }
}

/// U2_CONF0 value type for batch read/modify/write operations
///
/// Use `u2conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2Conf0Val {
    bits: u32,
}

impl U2Conf0Val {
    /// Create from raw bits
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
        Self { bits: 0x00003c10 }
    }

    /// Read **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH1_LCTRL_MODE_U_MASK) >> U2Conf0::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH1_HCTRL_MODE_U_MASK) >> U2Conf0::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_pos_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH1_POS_MODE_U_MASK) >> U2Conf0::CH1_POS_MODE_U_OFFSET
    }

    /// Read **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_neg_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH1_NEG_MODE_U_MASK) >> U2Conf0::CH1_NEG_MODE_U_OFFSET
    }

    /// Read **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH0_LCTRL_MODE_U_MASK) >> U2Conf0::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH0_HCTRL_MODE_U_MASK) >> U2Conf0::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_pos_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH0_POS_MODE_U_MASK) >> U2Conf0::CH0_POS_MODE_U_OFFSET
    }

    /// Read **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_neg_mode_u(&self) -> u32 {
        (self.bits & U2Conf0::CH0_NEG_MODE_U_MASK) >> U2Conf0::CH0_NEG_MODE_U_OFFSET
    }

    /// Read **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn thr_thres1_en_u(&self) -> bool {
        (self.bits & U2Conf0::THR_THRES1_EN_U_MASK) != 0
    }

    /// Read **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn thr_thres0_en_u(&self) -> bool {
        (self.bits & U2Conf0::THR_THRES0_EN_U_MASK) != 0
    }

    /// Read **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn thr_l_lim_en_u(&self) -> bool {
        (self.bits & U2Conf0::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn thr_h_lim_en_u(&self) -> bool {
        (self.bits & U2Conf0::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn thr_zero_en_u(&self) -> bool {
        (self.bits & U2Conf0::THR_ZERO_EN_U_MASK) != 0
    }

    /// Read **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn filter_en_u(&self) -> bool {
        (self.bits & U2Conf0::FILTER_EN_U_MASK) != 0
    }

    /// Read **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn filter_thres_u(&self) -> u32 {
        (self.bits & U2Conf0::FILTER_THRES_U_MASK) >> U2Conf0::FILTER_THRES_U_OFFSET
    }

    /// Set **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH1_LCTRL_MODE_U_MASK)
                | ((value << U2Conf0::CH1_LCTRL_MODE_U_OFFSET) & U2Conf0::CH1_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH1_HCTRL_MODE_U_MASK)
                | ((value << U2Conf0::CH1_HCTRL_MODE_U_OFFSET) & U2Conf0::CH1_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH1_POS_MODE_U_MASK)
                | ((value << U2Conf0::CH1_POS_MODE_U_OFFSET) & U2Conf0::CH1_POS_MODE_U_MASK),
        }
    }

    /// Set **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH1_NEG_MODE_U_MASK)
                | ((value << U2Conf0::CH1_NEG_MODE_U_OFFSET) & U2Conf0::CH1_NEG_MODE_U_MASK),
        }
    }

    /// Set **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH0_LCTRL_MODE_U_MASK)
                | ((value << U2Conf0::CH0_LCTRL_MODE_U_OFFSET) & U2Conf0::CH0_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH0_HCTRL_MODE_U_MASK)
                | ((value << U2Conf0::CH0_HCTRL_MODE_U_OFFSET) & U2Conf0::CH0_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH0_POS_MODE_U_MASK)
                | ((value << U2Conf0::CH0_POS_MODE_U_OFFSET) & U2Conf0::CH0_POS_MODE_U_MASK),
        }
    }

    /// Set **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::CH0_NEG_MODE_U_MASK)
                | ((value << U2Conf0::CH0_NEG_MODE_U_OFFSET) & U2Conf0::CH0_NEG_MODE_U_MASK),
        }
    }

    /// Set **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn with_thr_thres1_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::THR_THRES1_EN_U_MASK
            } else {
                self.bits & !U2Conf0::THR_THRES1_EN_U_MASK
            },
        }
    }

    /// Set **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn with_thr_thres0_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::THR_THRES0_EN_U_MASK
            } else {
                self.bits & !U2Conf0::THR_THRES0_EN_U_MASK
            },
        }
    }

    /// Set **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn with_thr_l_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::THR_L_LIM_EN_U_MASK
            } else {
                self.bits & !U2Conf0::THR_L_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn with_thr_h_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::THR_H_LIM_EN_U_MASK
            } else {
                self.bits & !U2Conf0::THR_H_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn with_thr_zero_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::THR_ZERO_EN_U_MASK
            } else {
                self.bits & !U2Conf0::THR_ZERO_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn with_filter_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U2Conf0::FILTER_EN_U_MASK
            } else {
                self.bits & !U2Conf0::FILTER_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn with_filter_thres_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf0::FILTER_THRES_U_MASK)
                | ((value << U2Conf0::FILTER_THRES_U_OFFSET) & U2Conf0::FILTER_THRES_U_MASK),
        }
    }
}

impl Default for U2Conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl U2Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2Conf0Val {
        U2Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U2Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U2Conf0Val) -> U2Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U2_CONF1 Register
///
/// Configuration register 1 for unit 2
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **CNT_THRES1_U** `[31:16]` (RW)
///
///   This register is used to configure the thres1 value for unit %s.
///
/// - **CNT_THRES0_U** `[15:0]` (RW)
///
///   This register is used to configure the thres0 value for unit %s.
#[repr(transparent)]
pub struct U2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2Conf1 {
    // ======== Field Constants ========

    /// **CNT_THRES1_U**
    ///
    /// This register is used to configure the thres1 value for unit %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_THRES1_U_OFFSET: u32 = 16;
    pub const CNT_THRES1_U_MASK: u32 = 0xffff0000;

    /// **CNT_THRES0_U**
    ///
    /// This register is used to configure the thres0 value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_THRES0_U_OFFSET: u32 = 0;
    pub const CNT_THRES0_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres1_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES1_U_MASK) >> Self::CNT_THRES1_U_OFFSET
    }

    /// Write **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres1_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES1_U_MASK)
                | ((value << Self::CNT_THRES1_U_OFFSET) & Self::CNT_THRES1_U_MASK)
        });
    }

    /// **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres0_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES0_U_MASK) >> Self::CNT_THRES0_U_OFFSET
    }

    /// Write **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres0_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES0_U_MASK)
                | ((value << Self::CNT_THRES0_U_OFFSET) & Self::CNT_THRES0_U_MASK)
        });
    }
}

/// U2_CONF1 value type for batch read/modify/write operations
///
/// Use `u2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2Conf1Val {
    bits: u32,
}

impl U2Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres1_u(&self) -> u32 {
        (self.bits & U2Conf1::CNT_THRES1_U_MASK) >> U2Conf1::CNT_THRES1_U_OFFSET
    }

    /// Read **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres0_u(&self) -> u32 {
        (self.bits & U2Conf1::CNT_THRES0_U_MASK) >> U2Conf1::CNT_THRES0_U_OFFSET
    }

    /// Set **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres1_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf1::CNT_THRES1_U_MASK)
                | ((value << U2Conf1::CNT_THRES1_U_OFFSET) & U2Conf1::CNT_THRES1_U_MASK),
        }
    }

    /// Set **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres0_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf1::CNT_THRES0_U_MASK)
                | ((value << U2Conf1::CNT_THRES0_U_OFFSET) & U2Conf1::CNT_THRES0_U_MASK),
        }
    }
}

impl Default for U2Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2Conf1Val {
        U2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U2Conf1Val) -> U2Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U2_CONF2 Register
///
/// Configuration register 2 for unit 2
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **CNT_L_LIM_U** `[31:16]` (RW)
///
///   This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
///
/// - **CNT_H_LIM_U** `[15:0]` (RW)
///
///   This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
#[repr(transparent)]
pub struct U2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2Conf2 {
    // ======== Field Constants ========

    /// **CNT_L_LIM_U**
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_L_LIM_U_OFFSET: u32 = 16;
    pub const CNT_L_LIM_U_MASK: u32 = 0xffff0000;

    /// **CNT_H_LIM_U**
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_H_LIM_U_OFFSET: u32 = 0;
    pub const CNT_H_LIM_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_L_LIM_U_MASK) >> Self::CNT_L_LIM_U_OFFSET
    }

    /// Write **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_l_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_L_LIM_U_MASK)
                | ((value << Self::CNT_L_LIM_U_OFFSET) & Self::CNT_L_LIM_U_MASK)
        });
    }

    /// **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_H_LIM_U_MASK) >> Self::CNT_H_LIM_U_OFFSET
    }

    /// Write **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_h_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_H_LIM_U_MASK)
                | ((value << Self::CNT_H_LIM_U_OFFSET) & Self::CNT_H_LIM_U_MASK)
        });
    }
}

/// U2_CONF2 value type for batch read/modify/write operations
///
/// Use `u2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2Conf2Val {
    bits: u32,
}

impl U2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_l_lim_u(&self) -> u32 {
        (self.bits & U2Conf2::CNT_L_LIM_U_MASK) >> U2Conf2::CNT_L_LIM_U_OFFSET
    }

    /// Read **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_h_lim_u(&self) -> u32 {
        (self.bits & U2Conf2::CNT_H_LIM_U_MASK) >> U2Conf2::CNT_H_LIM_U_OFFSET
    }

    /// Set **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_l_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf2::CNT_L_LIM_U_MASK)
                | ((value << U2Conf2::CNT_L_LIM_U_OFFSET) & U2Conf2::CNT_L_LIM_U_MASK),
        }
    }

    /// Set **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_h_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2Conf2::CNT_H_LIM_U_MASK)
                | ((value << U2Conf2::CNT_H_LIM_U_OFFSET) & U2Conf2::CNT_H_LIM_U_MASK),
        }
    }
}

impl Default for U2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2Conf2Val {
        U2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U2Conf2Val) -> U2Conf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U3_CONF0 Register
///
/// Configuration register 0 for unit 3
///
/// Address offset: 0x0024
/// Reset value: 0x00003c10
///
/// ## Fields
///
/// - **CH1_LCTRL_MODE_U** `[31:30]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_HCTRL_MODE_U** `[29:28]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH1_POS_MODE_U** `[27:26]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a positive edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH1_NEG_MODE_U** `[25:24]` (RW)
///
///   This register sets the behavior when the signal input of channel 1 detects a negative edge.
///   1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
///
/// - **CH0_LCTRL_MODE_U** `[23:22]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_HCTRL_MODE_U** `[21:20]` (RW)
///
///   This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
///   0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
///
/// - **CH0_POS_MODE_U** `[19:18]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a positive edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **CH0_NEG_MODE_U** `[17:16]` (RW)
///
///   This register sets the behavior when the signal input of channel 0 detects a negative edge.
///   1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
///
/// - **THR_THRES1_EN_U** `[15]` (RW)
///
///   This is the enable bit for unit %s's thres1 comparator.
///
/// - **THR_THRES0_EN_U** `[14]` (RW)
///
///   This is the enable bit for unit %s's thres0 comparator.
///
/// - **THR_L_LIM_EN_U** `[13]` (RW)
///
///   This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
///
/// - **THR_H_LIM_EN_U** `[12]` (RW)
///
///   This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
///
/// - **THR_ZERO_EN_U** `[11]` (RW)
///
///   This is the enable bit for unit %s's zero comparator.
///
/// - **FILTER_EN_U** `[10]` (RW)
///
///   This is the enable bit for unit %s's input filter.
///
/// - **FILTER_THRES_U** `[9:0]` (RW)
///
///   This sets the maximum threshold, in APB_CLK cycles, for the filter.
///   Any pulses with width less than this will be ignored when the filter is enabled.
#[repr(transparent)]
pub struct U3Conf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3Conf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3Conf0 {
    // ======== Field Constants ========

    /// **CH1_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CH1_LCTRL_MODE_U_OFFSET: u32 = 30;
    pub const CH1_LCTRL_MODE_U_MASK: u32 = 0xc0000000;

    /// **CH1_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const CH1_HCTRL_MODE_U_OFFSET: u32 = 28;
    pub const CH1_HCTRL_MODE_U_MASK: u32 = 0x30000000;

    /// **CH1_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CH1_POS_MODE_U_OFFSET: u32 = 26;
    pub const CH1_POS_MODE_U_MASK: u32 = 0x0c000000;

    /// **CH1_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const CH1_NEG_MODE_U_OFFSET: u32 = 24;
    pub const CH1_NEG_MODE_U_MASK: u32 = 0x03000000;

    /// **CH0_LCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CH0_LCTRL_MODE_U_OFFSET: u32 = 22;
    pub const CH0_LCTRL_MODE_U_MASK: u32 = 0x00c00000;

    /// **CH0_HCTRL_MODE_U**
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const CH0_HCTRL_MODE_U_OFFSET: u32 = 20;
    pub const CH0_HCTRL_MODE_U_MASK: u32 = 0x00300000;

    /// **CH0_POS_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CH0_POS_MODE_U_OFFSET: u32 = 18;
    pub const CH0_POS_MODE_U_MASK: u32 = 0x000c0000;

    /// **CH0_NEG_MODE_U**
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CH0_NEG_MODE_U_OFFSET: u32 = 16;
    pub const CH0_NEG_MODE_U_MASK: u32 = 0x00030000;

    /// **THR_THRES1_EN_U**
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const THR_THRES1_EN_U_OFFSET: u32 = 15;
    pub const THR_THRES1_EN_U_MASK: u32 = 0x00008000;
    pub const THR_THRES1_EN_U: u32 = 1 << 15;

    /// **THR_THRES0_EN_U**
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const THR_THRES0_EN_U_OFFSET: u32 = 14;
    pub const THR_THRES0_EN_U_MASK: u32 = 0x00004000;
    pub const THR_THRES0_EN_U: u32 = 1 << 14;

    /// **THR_L_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const THR_L_LIM_EN_U_OFFSET: u32 = 13;
    pub const THR_L_LIM_EN_U_MASK: u32 = 0x00002000;
    pub const THR_L_LIM_EN_U: u32 = 1 << 13;

    /// **THR_H_LIM_EN_U**
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const THR_H_LIM_EN_U_OFFSET: u32 = 12;
    pub const THR_H_LIM_EN_U_MASK: u32 = 0x00001000;
    pub const THR_H_LIM_EN_U: u32 = 1 << 12;

    /// **THR_ZERO_EN_U**
    ///
    /// This is the enable bit for unit %s's zero comparator.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const THR_ZERO_EN_U_OFFSET: u32 = 11;
    pub const THR_ZERO_EN_U_MASK: u32 = 0x00000800;
    pub const THR_ZERO_EN_U: u32 = 1 << 11;

    /// **FILTER_EN_U**
    ///
    /// This is the enable bit for unit %s's input filter.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FILTER_EN_U_OFFSET: u32 = 10;
    pub const FILTER_EN_U_MASK: u32 = 0x00000400;
    pub const FILTER_EN_U: u32 = 1 << 10;

    /// **FILTER_THRES_U**
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const FILTER_THRES_U_OFFSET: u32 = 0;
    pub const FILTER_THRES_U_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_LCTRL_MODE_U_MASK) >> Self::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_LCTRL_MODE_U** `[31:30]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_LCTRL_MODE_U_MASK)
                | ((value << Self::CH1_LCTRL_MODE_U_OFFSET) & Self::CH1_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_HCTRL_MODE_U_MASK) >> Self::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH1_HCTRL_MODE_U** `[29:28]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch1_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_HCTRL_MODE_U_MASK)
                | ((value << Self::CH1_HCTRL_MODE_U_OFFSET) & Self::CH1_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_POS_MODE_U_MASK) >> Self::CH1_POS_MODE_U_OFFSET
    }

    /// Write **CH1_POS_MODE_U** `[27:26]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_POS_MODE_U_MASK)
                | ((value << Self::CH1_POS_MODE_U_OFFSET) & Self::CH1_POS_MODE_U_MASK)
        });
    }

    /// **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_NEG_MODE_U_MASK) >> Self::CH1_NEG_MODE_U_OFFSET
    }

    /// Write **CH1_NEG_MODE_U** `[25:24]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch1_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1_NEG_MODE_U_MASK)
                | ((value << Self::CH1_NEG_MODE_U_OFFSET) & Self::CH1_NEG_MODE_U_MASK)
        });
    }

    /// **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_LCTRL_MODE_U_MASK) >> Self::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_LCTRL_MODE_U** `[23:22]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_lctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_LCTRL_MODE_U_MASK)
                | ((value << Self::CH0_LCTRL_MODE_U_OFFSET) & Self::CH0_LCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_HCTRL_MODE_U_MASK) >> Self::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Write **CH0_HCTRL_MODE_U** `[21:20]` (RW)
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub fn write_ch0_hctrl_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_HCTRL_MODE_U_MASK)
                | ((value << Self::CH0_HCTRL_MODE_U_OFFSET) & Self::CH0_HCTRL_MODE_U_MASK)
        });
    }

    /// **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_POS_MODE_U_MASK) >> Self::CH0_POS_MODE_U_OFFSET
    }

    /// Write **CH0_POS_MODE_U** `[19:18]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_pos_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_POS_MODE_U_MASK)
                | ((value << Self::CH0_POS_MODE_U_OFFSET) & Self::CH0_POS_MODE_U_MASK)
        });
    }

    /// **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_NEG_MODE_U_MASK) >> Self::CH0_NEG_MODE_U_OFFSET
    }

    /// Write **CH0_NEG_MODE_U** `[17:16]` (RW)
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub fn write_ch0_neg_mode_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH0_NEG_MODE_U_MASK)
                | ((value << Self::CH0_NEG_MODE_U_OFFSET) & Self::CH0_NEG_MODE_U_MASK)
        });
    }

    /// **THR_THRES1_EN_U** `[15]` (RW)
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES1_EN_U_MASK) != 0
    }

    /// Set THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES1_EN_U_MASK);
    }

    /// Clear THR_THRES1_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres1_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES1_EN_U_MASK);
    }

    /// **THR_THRES0_EN_U** `[14]` (RW)
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_THRES0_EN_U_MASK) != 0
    }

    /// Set THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_THRES0_EN_U_MASK);
    }

    /// Clear THR_THRES0_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_thres0_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_THRES0_EN_U_MASK);
    }

    /// **THR_L_LIM_EN_U** `[13]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Set THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_L_LIM_EN_U_MASK);
    }

    /// Clear THR_L_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_l_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_L_LIM_EN_U_MASK);
    }

    /// **THR_H_LIM_EN_U** `[12]` (RW)
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Set THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_H_LIM_EN_U_MASK);
    }

    /// Clear THR_H_LIM_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_h_lim_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_H_LIM_EN_U_MASK);
    }

    /// **THR_ZERO_EN_U** `[11]` (RW)
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> bool {
        (self.value.read_bits() & Self::THR_ZERO_EN_U_MASK) != 0
    }

    /// Set THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r | Self::THR_ZERO_EN_U_MASK);
    }

    /// Clear THR_ZERO_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thr_zero_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::THR_ZERO_EN_U_MASK);
    }

    /// **FILTER_EN_U** `[10]` (RW)
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub fn filter_en_u(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_EN_U_MASK) != 0
    }

    /// Set FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_en_u(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_EN_U_MASK);
    }

    /// Clear FILTER_EN_U bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_en_u(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_EN_U_MASK);
    }

    /// **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn filter_thres_u(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_THRES_U_MASK) >> Self::FILTER_THRES_U_OFFSET
    }

    /// Write **FILTER_THRES_U** `[9:0]` (RW)
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub fn write_filter_thres_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_THRES_U_MASK)
                | ((value << Self::FILTER_THRES_U_OFFSET) & Self::FILTER_THRES_U_MASK)
        });
    }
}

/// U3_CONF0 value type for batch read/modify/write operations
///
/// Use `u3conf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3conf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3Conf0Val {
    bits: u32,
}

impl U3Conf0Val {
    /// Create from raw bits
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
        Self { bits: 0x00003c10 }
    }

    /// Read **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_lctrl_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH1_LCTRL_MODE_U_MASK) >> U3Conf0::CH1_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch1_hctrl_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH1_HCTRL_MODE_U_MASK) >> U3Conf0::CH1_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_pos_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH1_POS_MODE_U_MASK) >> U3Conf0::CH1_POS_MODE_U_OFFSET
    }

    /// Read **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch1_neg_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH1_NEG_MODE_U_MASK) >> U3Conf0::CH1_NEG_MODE_U_OFFSET
    }

    /// Read **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_lctrl_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH0_LCTRL_MODE_U_MASK) >> U3Conf0::CH0_LCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn ch0_hctrl_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH0_HCTRL_MODE_U_MASK) >> U3Conf0::CH0_HCTRL_MODE_U_OFFSET
    }

    /// Read **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_pos_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH0_POS_MODE_U_MASK) >> U3Conf0::CH0_POS_MODE_U_OFFSET
    }

    /// Read **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn ch0_neg_mode_u(&self) -> u32 {
        (self.bits & U3Conf0::CH0_NEG_MODE_U_MASK) >> U3Conf0::CH0_NEG_MODE_U_OFFSET
    }

    /// Read **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn thr_thres1_en_u(&self) -> bool {
        (self.bits & U3Conf0::THR_THRES1_EN_U_MASK) != 0
    }

    /// Read **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn thr_thres0_en_u(&self) -> bool {
        (self.bits & U3Conf0::THR_THRES0_EN_U_MASK) != 0
    }

    /// Read **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn thr_l_lim_en_u(&self) -> bool {
        (self.bits & U3Conf0::THR_L_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn thr_h_lim_en_u(&self) -> bool {
        (self.bits & U3Conf0::THR_H_LIM_EN_U_MASK) != 0
    }

    /// Read **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn thr_zero_en_u(&self) -> bool {
        (self.bits & U3Conf0::THR_ZERO_EN_U_MASK) != 0
    }

    /// Read **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn filter_en_u(&self) -> bool {
        (self.bits & U3Conf0::FILTER_EN_U_MASK) != 0
    }

    /// Read **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn filter_thres_u(&self) -> u32 {
        (self.bits & U3Conf0::FILTER_THRES_U_MASK) >> U3Conf0::FILTER_THRES_U_OFFSET
    }

    /// Set **CH1_LCTRL_MODE_U** `[31:30]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH1_LCTRL_MODE_U_MASK)
                | ((value << U3Conf0::CH1_LCTRL_MODE_U_OFFSET) & U3Conf0::CH1_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_HCTRL_MODE_U** `[29:28]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch1_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH1_HCTRL_MODE_U_MASK)
                | ((value << U3Conf0::CH1_HCTRL_MODE_U_OFFSET) & U3Conf0::CH1_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH1_POS_MODE_U** `[27:26]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a positive edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH1_POS_MODE_U_MASK)
                | ((value << U3Conf0::CH1_POS_MODE_U_OFFSET) & U3Conf0::CH1_POS_MODE_U_MASK),
        }
    }

    /// Set **CH1_NEG_MODE_U** `[25:24]`
    ///
    /// This register sets the behavior when the signal input of channel 1 detects a negative edge.
    /// 1: Increment the counter.2: Decrement the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch1_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH1_NEG_MODE_U_MASK)
                | ((value << U3Conf0::CH1_NEG_MODE_U_OFFSET) & U3Conf0::CH1_NEG_MODE_U_MASK),
        }
    }

    /// Set **CH0_LCTRL_MODE_U** `[23:22]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_lctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH0_LCTRL_MODE_U_MASK)
                | ((value << U3Conf0::CH0_LCTRL_MODE_U_OFFSET) & U3Conf0::CH0_LCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_HCTRL_MODE_U** `[21:20]`
    ///
    /// This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high.
    /// 0: No modification.1: Invert behavior (increase -> decrease, decrease -> increase).2, 3: Inhibit counter modification
    #[inline(always)]
    pub const fn with_ch0_hctrl_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH0_HCTRL_MODE_U_MASK)
                | ((value << U3Conf0::CH0_HCTRL_MODE_U_OFFSET) & U3Conf0::CH0_HCTRL_MODE_U_MASK),
        }
    }

    /// Set **CH0_POS_MODE_U** `[19:18]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a positive edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_pos_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH0_POS_MODE_U_MASK)
                | ((value << U3Conf0::CH0_POS_MODE_U_OFFSET) & U3Conf0::CH0_POS_MODE_U_MASK),
        }
    }

    /// Set **CH0_NEG_MODE_U** `[17:16]`
    ///
    /// This register sets the behavior when the signal input of channel 0 detects a negative edge.
    /// 1: Increase the counter.2: Decrease the counter.0, 3: No effect on counter
    #[inline(always)]
    pub const fn with_ch0_neg_mode_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::CH0_NEG_MODE_U_MASK)
                | ((value << U3Conf0::CH0_NEG_MODE_U_OFFSET) & U3Conf0::CH0_NEG_MODE_U_MASK),
        }
    }

    /// Set **THR_THRES1_EN_U** `[15:15]`
    ///
    /// This is the enable bit for unit %s's thres1 comparator.
    #[inline(always)]
    pub const fn with_thr_thres1_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::THR_THRES1_EN_U_MASK
            } else {
                self.bits & !U3Conf0::THR_THRES1_EN_U_MASK
            },
        }
    }

    /// Set **THR_THRES0_EN_U** `[14:14]`
    ///
    /// This is the enable bit for unit %s's thres0 comparator.
    #[inline(always)]
    pub const fn with_thr_thres0_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::THR_THRES0_EN_U_MASK
            } else {
                self.bits & !U3Conf0::THR_THRES0_EN_U_MASK
            },
        }
    }

    /// Set **THR_L_LIM_EN_U** `[13:13]`
    ///
    /// This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt.
    #[inline(always)]
    pub const fn with_thr_l_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::THR_L_LIM_EN_U_MASK
            } else {
                self.bits & !U3Conf0::THR_L_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_H_LIM_EN_U** `[12:12]`
    ///
    /// This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt.
    #[inline(always)]
    pub const fn with_thr_h_lim_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::THR_H_LIM_EN_U_MASK
            } else {
                self.bits & !U3Conf0::THR_H_LIM_EN_U_MASK
            },
        }
    }

    /// Set **THR_ZERO_EN_U** `[11:11]`
    ///
    /// This is the enable bit for unit %s's zero comparator.
    #[inline(always)]
    pub const fn with_thr_zero_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::THR_ZERO_EN_U_MASK
            } else {
                self.bits & !U3Conf0::THR_ZERO_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_EN_U** `[10:10]`
    ///
    /// This is the enable bit for unit %s's input filter.
    #[inline(always)]
    pub const fn with_filter_en_u(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | U3Conf0::FILTER_EN_U_MASK
            } else {
                self.bits & !U3Conf0::FILTER_EN_U_MASK
            },
        }
    }

    /// Set **FILTER_THRES_U** `[9:0]`
    ///
    /// This sets the maximum threshold, in APB_CLK cycles, for the filter.
    /// Any pulses with width less than this will be ignored when the filter is enabled.
    #[inline(always)]
    pub const fn with_filter_thres_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf0::FILTER_THRES_U_MASK)
                | ((value << U3Conf0::FILTER_THRES_U_OFFSET) & U3Conf0::FILTER_THRES_U_MASK),
        }
    }
}

impl Default for U3Conf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl U3Conf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3Conf0Val {
        U3Conf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U3Conf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U3Conf0Val) -> U3Conf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U3_CONF1 Register
///
/// Configuration register 1 for unit 3
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **CNT_THRES1_U** `[31:16]` (RW)
///
///   This register is used to configure the thres1 value for unit %s.
///
/// - **CNT_THRES0_U** `[15:0]` (RW)
///
///   This register is used to configure the thres0 value for unit %s.
#[repr(transparent)]
pub struct U3Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3Conf1 {
    // ======== Field Constants ========

    /// **CNT_THRES1_U**
    ///
    /// This register is used to configure the thres1 value for unit %s.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_THRES1_U_OFFSET: u32 = 16;
    pub const CNT_THRES1_U_MASK: u32 = 0xffff0000;

    /// **CNT_THRES0_U**
    ///
    /// This register is used to configure the thres0 value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_THRES0_U_OFFSET: u32 = 0;
    pub const CNT_THRES0_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres1_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES1_U_MASK) >> Self::CNT_THRES1_U_OFFSET
    }

    /// Write **CNT_THRES1_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres1_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES1_U_MASK)
                | ((value << Self::CNT_THRES1_U_OFFSET) & Self::CNT_THRES1_U_MASK)
        });
    }

    /// **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn cnt_thres0_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THRES0_U_MASK) >> Self::CNT_THRES0_U_OFFSET
    }

    /// Write **CNT_THRES0_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub fn write_cnt_thres0_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_THRES0_U_MASK)
                | ((value << Self::CNT_THRES0_U_OFFSET) & Self::CNT_THRES0_U_MASK)
        });
    }
}

/// U3_CONF1 value type for batch read/modify/write operations
///
/// Use `u3conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3Conf1Val {
    bits: u32,
}

impl U3Conf1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres1_u(&self) -> u32 {
        (self.bits & U3Conf1::CNT_THRES1_U_MASK) >> U3Conf1::CNT_THRES1_U_OFFSET
    }

    /// Read **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn cnt_thres0_u(&self) -> u32 {
        (self.bits & U3Conf1::CNT_THRES0_U_MASK) >> U3Conf1::CNT_THRES0_U_OFFSET
    }

    /// Set **CNT_THRES1_U** `[31:16]`
    ///
    /// This register is used to configure the thres1 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres1_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf1::CNT_THRES1_U_MASK)
                | ((value << U3Conf1::CNT_THRES1_U_OFFSET) & U3Conf1::CNT_THRES1_U_MASK),
        }
    }

    /// Set **CNT_THRES0_U** `[15:0]`
    ///
    /// This register is used to configure the thres0 value for unit %s.
    #[inline(always)]
    pub const fn with_cnt_thres0_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf1::CNT_THRES0_U_MASK)
                | ((value << U3Conf1::CNT_THRES0_U_OFFSET) & U3Conf1::CNT_THRES0_U_MASK),
        }
    }
}

impl Default for U3Conf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U3Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3Conf1Val {
        U3Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U3Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U3Conf1Val) -> U3Conf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U3_CONF2 Register
///
/// Configuration register 2 for unit 3
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **CNT_L_LIM_U** `[31:16]` (RW)
///
///   This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
///
/// - **CNT_H_LIM_U** `[15:0]` (RW)
///
///   This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
#[repr(transparent)]
pub struct U3Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3Conf2 {
    // ======== Field Constants ========

    /// **CNT_L_LIM_U**
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_L_LIM_U_OFFSET: u32 = 16;
    pub const CNT_L_LIM_U_MASK: u32 = 0xffff0000;

    /// **CNT_H_LIM_U**
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_H_LIM_U_OFFSET: u32 = 0;
    pub const CNT_H_LIM_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_L_LIM_U_MASK) >> Self::CNT_L_LIM_U_OFFSET
    }

    /// Write **CNT_L_LIM_U** `[31:16]` (RW)
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_l_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_L_LIM_U_MASK)
                | ((value << Self::CNT_L_LIM_U_OFFSET) & Self::CNT_L_LIM_U_MASK)
        });
    }

    /// **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_H_LIM_U_MASK) >> Self::CNT_H_LIM_U_OFFSET
    }

    /// Write **CNT_H_LIM_U** `[15:0]` (RW)
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub fn write_cnt_h_lim_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_H_LIM_U_MASK)
                | ((value << Self::CNT_H_LIM_U_OFFSET) & Self::CNT_H_LIM_U_MASK)
        });
    }
}

/// U3_CONF2 value type for batch read/modify/write operations
///
/// Use `u3conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3Conf2Val {
    bits: u32,
}

impl U3Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_l_lim_u(&self) -> u32 {
        (self.bits & U3Conf2::CNT_L_LIM_U_MASK) >> U3Conf2::CNT_L_LIM_U_OFFSET
    }

    /// Read **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn cnt_h_lim_u(&self) -> u32 {
        (self.bits & U3Conf2::CNT_H_LIM_U_MASK) >> U3Conf2::CNT_H_LIM_U_OFFSET
    }

    /// Set **CNT_L_LIM_U** `[31:16]`
    ///
    /// This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_l_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf2::CNT_L_LIM_U_MASK)
                | ((value << U3Conf2::CNT_L_LIM_U_OFFSET) & U3Conf2::CNT_L_LIM_U_MASK),
        }
    }

    /// Set **CNT_H_LIM_U** `[15:0]`
    ///
    /// This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0.
    #[inline(always)]
    pub const fn with_cnt_h_lim_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3Conf2::CNT_H_LIM_U_MASK)
                | ((value << U3Conf2::CNT_H_LIM_U_OFFSET) & U3Conf2::CNT_H_LIM_U_MASK),
        }
    }
}

impl Default for U3Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U3Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3Conf2Val {
        U3Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U3Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U3Conf2Val) -> U3Conf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U0_CNT Register
///
/// Counter value for unit 0
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **PULSE_CNT_U** `[15:0]` (RO)
///
///   This register stores the current pulse count value for unit %s.
#[repr(transparent)]
pub struct U0Cnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0Cnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0Cnt {
    // ======== Field Constants ========

    /// **PULSE_CNT_U**
    ///
    /// This register stores the current pulse count value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PULSE_CNT_U_OFFSET: u32 = 0;
    pub const PULSE_CNT_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PULSE_CNT_U** `[15:0]` (RO)
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> u32 {
        (self.value.read_bits() & Self::PULSE_CNT_U_MASK) >> Self::PULSE_CNT_U_OFFSET
    }
}

/// U0_CNT value type for batch read/modify/write operations
///
/// Use `u0cnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0cnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0CntVal {
    bits: u32,
}

impl U0CntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PULSE_CNT_U** `[15:0]`
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub const fn pulse_cnt_u(&self) -> u32 {
        (self.bits & U0Cnt::PULSE_CNT_U_MASK) >> U0Cnt::PULSE_CNT_U_OFFSET
    }
}

impl Default for U0CntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U0Cnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0CntVal {
        U0CntVal::from_bits(self.value.read_bits())
    }
}

/// U1_CNT Register
///
/// Counter value for unit 1
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **PULSE_CNT_U** `[15:0]` (RO)
///
///   This register stores the current pulse count value for unit %s.
#[repr(transparent)]
pub struct U1Cnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1Cnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1Cnt {
    // ======== Field Constants ========

    /// **PULSE_CNT_U**
    ///
    /// This register stores the current pulse count value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PULSE_CNT_U_OFFSET: u32 = 0;
    pub const PULSE_CNT_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PULSE_CNT_U** `[15:0]` (RO)
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> u32 {
        (self.value.read_bits() & Self::PULSE_CNT_U_MASK) >> Self::PULSE_CNT_U_OFFSET
    }
}

/// U1_CNT value type for batch read/modify/write operations
///
/// Use `u1cnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1cnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1CntVal {
    bits: u32,
}

impl U1CntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PULSE_CNT_U** `[15:0]`
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub const fn pulse_cnt_u(&self) -> u32 {
        (self.bits & U1Cnt::PULSE_CNT_U_MASK) >> U1Cnt::PULSE_CNT_U_OFFSET
    }
}

impl Default for U1CntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U1Cnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1CntVal {
        U1CntVal::from_bits(self.value.read_bits())
    }
}

/// U2_CNT Register
///
/// Counter value for unit 2
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **PULSE_CNT_U** `[15:0]` (RO)
///
///   This register stores the current pulse count value for unit %s.
#[repr(transparent)]
pub struct U2Cnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2Cnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2Cnt {
    // ======== Field Constants ========

    /// **PULSE_CNT_U**
    ///
    /// This register stores the current pulse count value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PULSE_CNT_U_OFFSET: u32 = 0;
    pub const PULSE_CNT_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PULSE_CNT_U** `[15:0]` (RO)
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> u32 {
        (self.value.read_bits() & Self::PULSE_CNT_U_MASK) >> Self::PULSE_CNT_U_OFFSET
    }
}

/// U2_CNT value type for batch read/modify/write operations
///
/// Use `u2cnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2cnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2CntVal {
    bits: u32,
}

impl U2CntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PULSE_CNT_U** `[15:0]`
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub const fn pulse_cnt_u(&self) -> u32 {
        (self.bits & U2Cnt::PULSE_CNT_U_MASK) >> U2Cnt::PULSE_CNT_U_OFFSET
    }
}

impl Default for U2CntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U2Cnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2CntVal {
        U2CntVal::from_bits(self.value.read_bits())
    }
}

/// U3_CNT Register
///
/// Counter value for unit 3
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **PULSE_CNT_U** `[15:0]` (RO)
///
///   This register stores the current pulse count value for unit %s.
#[repr(transparent)]
pub struct U3Cnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3Cnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3Cnt {
    // ======== Field Constants ========

    /// **PULSE_CNT_U**
    ///
    /// This register stores the current pulse count value for unit %s.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PULSE_CNT_U_OFFSET: u32 = 0;
    pub const PULSE_CNT_U_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PULSE_CNT_U** `[15:0]` (RO)
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> u32 {
        (self.value.read_bits() & Self::PULSE_CNT_U_MASK) >> Self::PULSE_CNT_U_OFFSET
    }
}

/// U3_CNT value type for batch read/modify/write operations
///
/// Use `u3cnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3cnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3CntVal {
    bits: u32,
}

impl U3CntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PULSE_CNT_U** `[15:0]`
    ///
    /// This register stores the current pulse count value for unit %s.
    #[inline(always)]
    pub const fn pulse_cnt_u(&self) -> u32 {
        (self.bits & U3Cnt::PULSE_CNT_U_MASK) >> U3Cnt::PULSE_CNT_U_OFFSET
    }
}

impl Default for U3CntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U3Cnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3CntVal {
        U3CntVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// Interrupt raw status register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **CNT_THR_EVENT_U3_INT_RAW** `[3]` (RW)
///
///   The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
///
/// - **CNT_THR_EVENT_U2_INT_RAW** `[2]` (RW)
///
///   The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
///
/// - **CNT_THR_EVENT_U1_INT_RAW** `[1]` (RW)
///
///   The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
///
/// - **CNT_THR_EVENT_U0_INT_RAW** `[0]` (RW)
///
///   The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
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

    /// **CNT_THR_EVENT_U3_INT_RAW**
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_EVENT_U3_INT_RAW_OFFSET: u32 = 3;
    pub const CNT_THR_EVENT_U3_INT_RAW_MASK: u32 = 0x00000008;
    pub const CNT_THR_EVENT_U3_INT_RAW: u32 = 1 << 3;

    /// **CNT_THR_EVENT_U2_INT_RAW**
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_EVENT_U2_INT_RAW_OFFSET: u32 = 2;
    pub const CNT_THR_EVENT_U2_INT_RAW_MASK: u32 = 0x00000004;
    pub const CNT_THR_EVENT_U2_INT_RAW: u32 = 1 << 2;

    /// **CNT_THR_EVENT_U1_INT_RAW**
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CNT_THR_EVENT_U1_INT_RAW_OFFSET: u32 = 1;
    pub const CNT_THR_EVENT_U1_INT_RAW_MASK: u32 = 0x00000002;
    pub const CNT_THR_EVENT_U1_INT_RAW: u32 = 1 << 1;

    /// **CNT_THR_EVENT_U0_INT_RAW**
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CNT_THR_EVENT_U0_INT_RAW_OFFSET: u32 = 0;
    pub const CNT_THR_EVENT_U0_INT_RAW_MASK: u32 = 0x00000001;
    pub const CNT_THR_EVENT_U0_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CNT_THR_EVENT_U3_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U3_INT_RAW_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u3_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U3_INT_RAW_MASK);
    }

    /// Clear CNT_THR_EVENT_U3_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u3_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U3_INT_RAW_MASK);
    }

    /// **CNT_THR_EVENT_U2_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U2_INT_RAW_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u2_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U2_INT_RAW_MASK);
    }

    /// Clear CNT_THR_EVENT_U2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u2_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U2_INT_RAW_MASK);
    }

    /// **CNT_THR_EVENT_U1_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U1_INT_RAW_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u1_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U1_INT_RAW_MASK);
    }

    /// Clear CNT_THR_EVENT_U1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U1_INT_RAW_MASK);
    }

    /// **CNT_THR_EVENT_U0_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U0_INT_RAW_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u0_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U0_INT_RAW_MASK);
    }

    /// Clear CNT_THR_EVENT_U0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u0_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U0_INT_RAW_MASK);
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

    /// Read **CNT_THR_EVENT_U3_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u3_int_raw(&self) -> bool {
        (self.bits & IntRaw::CNT_THR_EVENT_U3_INT_RAW_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U2_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u2_int_raw(&self) -> bool {
        (self.bits & IntRaw::CNT_THR_EVENT_U2_INT_RAW_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U1_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u1_int_raw(&self) -> bool {
        (self.bits & IntRaw::CNT_THR_EVENT_U1_INT_RAW_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U0_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u0_int_raw(&self) -> bool {
        (self.bits & IntRaw::CNT_THR_EVENT_U0_INT_RAW_MASK) != 0
    }

    /// Set **CNT_THR_EVENT_U3_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u3_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CNT_THR_EVENT_U3_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CNT_THR_EVENT_U3_INT_RAW_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U2_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CNT_THR_EVENT_U2_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CNT_THR_EVENT_U2_INT_RAW_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U1_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CNT_THR_EVENT_U1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CNT_THR_EVENT_U1_INT_RAW_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U0_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CNT_THR_EVENT_U0_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CNT_THR_EVENT_U0_INT_RAW_MASK
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
/// Interrupt status register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **CNT_THR_EVENT_U3_INT_ST** `[3]` (RO)
///
///   The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
///
/// - **CNT_THR_EVENT_U2_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
///
/// - **CNT_THR_EVENT_U1_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
///
/// - **CNT_THR_EVENT_U0_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
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

    /// **CNT_THR_EVENT_U3_INT_ST**
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_EVENT_U3_INT_ST_OFFSET: u32 = 3;
    pub const CNT_THR_EVENT_U3_INT_ST_MASK: u32 = 0x00000008;
    pub const CNT_THR_EVENT_U3_INT_ST: u32 = 1 << 3;

    /// **CNT_THR_EVENT_U2_INT_ST**
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_EVENT_U2_INT_ST_OFFSET: u32 = 2;
    pub const CNT_THR_EVENT_U2_INT_ST_MASK: u32 = 0x00000004;
    pub const CNT_THR_EVENT_U2_INT_ST: u32 = 1 << 2;

    /// **CNT_THR_EVENT_U1_INT_ST**
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CNT_THR_EVENT_U1_INT_ST_OFFSET: u32 = 1;
    pub const CNT_THR_EVENT_U1_INT_ST_MASK: u32 = 0x00000002;
    pub const CNT_THR_EVENT_U1_INT_ST: u32 = 1 << 1;

    /// **CNT_THR_EVENT_U0_INT_ST**
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CNT_THR_EVENT_U0_INT_ST_OFFSET: u32 = 0;
    pub const CNT_THR_EVENT_U0_INT_ST_MASK: u32 = 0x00000001;
    pub const CNT_THR_EVENT_U0_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CNT_THR_EVENT_U3_INT_ST** `[3]` (RO)
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U3_INT_ST_MASK) != 0
    }

    /// **CNT_THR_EVENT_U2_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U2_INT_ST_MASK) != 0
    }

    /// **CNT_THR_EVENT_U1_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U1_INT_ST_MASK) != 0
    }

    /// **CNT_THR_EVENT_U0_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U0_INT_ST_MASK) != 0
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

    /// Read **CNT_THR_EVENT_U3_INT_ST** `[3:3]`
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u3_int_st(&self) -> bool {
        (self.bits & IntSt::CNT_THR_EVENT_U3_INT_ST_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U2_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u2_int_st(&self) -> bool {
        (self.bits & IntSt::CNT_THR_EVENT_U2_INT_ST_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U1_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u1_int_st(&self) -> bool {
        (self.bits & IntSt::CNT_THR_EVENT_U1_INT_ST_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U0_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u0_int_st(&self) -> bool {
        (self.bits & IntSt::CNT_THR_EVENT_U0_INT_ST_MASK) != 0
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
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **CNT_THR_EVENT_U3_INT_ENA** `[3]` (RW)
///
///   The interrupt enable bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
///
/// - **CNT_THR_EVENT_U2_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
///
/// - **CNT_THR_EVENT_U1_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
///
/// - **CNT_THR_EVENT_U0_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
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

    /// **CNT_THR_EVENT_U3_INT_ENA**
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_EVENT_U3_INT_ENA_OFFSET: u32 = 3;
    pub const CNT_THR_EVENT_U3_INT_ENA_MASK: u32 = 0x00000008;
    pub const CNT_THR_EVENT_U3_INT_ENA: u32 = 1 << 3;

    /// **CNT_THR_EVENT_U2_INT_ENA**
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_EVENT_U2_INT_ENA_OFFSET: u32 = 2;
    pub const CNT_THR_EVENT_U2_INT_ENA_MASK: u32 = 0x00000004;
    pub const CNT_THR_EVENT_U2_INT_ENA: u32 = 1 << 2;

    /// **CNT_THR_EVENT_U1_INT_ENA**
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CNT_THR_EVENT_U1_INT_ENA_OFFSET: u32 = 1;
    pub const CNT_THR_EVENT_U1_INT_ENA_MASK: u32 = 0x00000002;
    pub const CNT_THR_EVENT_U1_INT_ENA: u32 = 1 << 1;

    /// **CNT_THR_EVENT_U0_INT_ENA**
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CNT_THR_EVENT_U0_INT_ENA_OFFSET: u32 = 0;
    pub const CNT_THR_EVENT_U0_INT_ENA_MASK: u32 = 0x00000001;
    pub const CNT_THR_EVENT_U0_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CNT_THR_EVENT_U3_INT_ENA** `[3]` (RW)
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U3_INT_ENA_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u3_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U3_INT_ENA_MASK);
    }

    /// Clear CNT_THR_EVENT_U3_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u3_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U3_INT_ENA_MASK);
    }

    /// **CNT_THR_EVENT_U2_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U2_INT_ENA_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u2_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U2_INT_ENA_MASK);
    }

    /// Clear CNT_THR_EVENT_U2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u2_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U2_INT_ENA_MASK);
    }

    /// **CNT_THR_EVENT_U1_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U1_INT_ENA_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u1_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U1_INT_ENA_MASK);
    }

    /// Clear CNT_THR_EVENT_U1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U1_INT_ENA_MASK);
    }

    /// **CNT_THR_EVENT_U0_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_EVENT_U0_INT_ENA_MASK) != 0
    }

    /// Set CNT_THR_EVENT_U0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_thr_event_u0_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CNT_THR_EVENT_U0_INT_ENA_MASK);
    }

    /// Clear CNT_THR_EVENT_U0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_thr_event_u0_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CNT_THR_EVENT_U0_INT_ENA_MASK);
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

    /// Read **CNT_THR_EVENT_U3_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u3_int_ena(&self) -> bool {
        (self.bits & IntEna::CNT_THR_EVENT_U3_INT_ENA_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U2_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u2_int_ena(&self) -> bool {
        (self.bits & IntEna::CNT_THR_EVENT_U2_INT_ENA_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U1_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u1_int_ena(&self) -> bool {
        (self.bits & IntEna::CNT_THR_EVENT_U1_INT_ENA_MASK) != 0
    }

    /// Read **CNT_THR_EVENT_U0_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn cnt_thr_event_u0_int_ena(&self) -> bool {
        (self.bits & IntEna::CNT_THR_EVENT_U0_INT_ENA_MASK) != 0
    }

    /// Set **CNT_THR_EVENT_U3_INT_ENA** `[3:3]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u3_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CNT_THR_EVENT_U3_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CNT_THR_EVENT_U3_INT_ENA_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U2_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CNT_THR_EVENT_U2_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CNT_THR_EVENT_U2_INT_ENA_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U1_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CNT_THR_EVENT_U1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CNT_THR_EVENT_U1_INT_ENA_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U0_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CNT_THR_EVENT_U0_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CNT_THR_EVENT_U0_INT_ENA_MASK
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
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **CNT_THR_EVENT_U3_INT_CLR** `[3]` (WO)
///
///   Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt.
///
/// - **CNT_THR_EVENT_U2_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt.
///
/// - **CNT_THR_EVENT_U1_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt.
///
/// - **CNT_THR_EVENT_U0_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt.
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

    /// **CNT_THR_EVENT_U3_INT_CLR**
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_EVENT_U3_INT_CLR_OFFSET: u32 = 3;
    pub const CNT_THR_EVENT_U3_INT_CLR_MASK: u32 = 0x00000008;
    pub const CNT_THR_EVENT_U3_INT_CLR: u32 = 1 << 3;

    /// **CNT_THR_EVENT_U2_INT_CLR**
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_EVENT_U2_INT_CLR_OFFSET: u32 = 2;
    pub const CNT_THR_EVENT_U2_INT_CLR_MASK: u32 = 0x00000004;
    pub const CNT_THR_EVENT_U2_INT_CLR: u32 = 1 << 2;

    /// **CNT_THR_EVENT_U1_INT_CLR**
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CNT_THR_EVENT_U1_INT_CLR_OFFSET: u32 = 1;
    pub const CNT_THR_EVENT_U1_INT_CLR_MASK: u32 = 0x00000002;
    pub const CNT_THR_EVENT_U1_INT_CLR: u32 = 1 << 1;

    /// **CNT_THR_EVENT_U0_INT_CLR**
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CNT_THR_EVENT_U0_INT_CLR_OFFSET: u32 = 0;
    pub const CNT_THR_EVENT_U0_INT_CLR_MASK: u32 = 0x00000001;
    pub const CNT_THR_EVENT_U0_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CNT_THR_EVENT_U3_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub fn trigger_cnt_thr_event_u3_int_clr(&self) {
        self.value.write_bits(Self::CNT_THR_EVENT_U3_INT_CLR_MASK);
    }

    /// Trigger **CNT_THR_EVENT_U2_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub fn trigger_cnt_thr_event_u2_int_clr(&self) {
        self.value.write_bits(Self::CNT_THR_EVENT_U2_INT_CLR_MASK);
    }

    /// Trigger **CNT_THR_EVENT_U1_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub fn trigger_cnt_thr_event_u1_int_clr(&self) {
        self.value.write_bits(Self::CNT_THR_EVENT_U1_INT_CLR_MASK);
    }

    /// Trigger **CNT_THR_EVENT_U0_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub fn trigger_cnt_thr_event_u0_int_clr(&self) {
        self.value.write_bits(Self::CNT_THR_EVENT_U0_INT_CLR_MASK);
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

    /// Set **CNT_THR_EVENT_U3_INT_CLR** `[3:3]`
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u3_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CNT_THR_EVENT_U3_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CNT_THR_EVENT_U3_INT_CLR_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U2_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CNT_THR_EVENT_U2_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CNT_THR_EVENT_U2_INT_CLR_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U1_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CNT_THR_EVENT_U1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CNT_THR_EVENT_U1_INT_CLR_MASK
            },
        }
    }

    /// Set **CNT_THR_EVENT_U0_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub const fn with_cnt_thr_event_u0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CNT_THR_EVENT_U0_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CNT_THR_EVENT_U0_INT_CLR_MASK
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

/// U0_STATUS Register
///
/// PNCT UNIT0 status register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **CNT_THR_ZERO_LAT_U** `[6]` (RO)
///
///   The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
///
/// - **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
///
///   The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
///
/// - **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
///
///   The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
///
/// - **CNT_THR_THRES0_LAT_U** `[3]` (RO)
///
///   The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
///
/// - **CNT_THR_THRES1_LAT_U** `[2]` (RO)
///
///   The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
///
/// - **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
///
///   The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
#[repr(transparent)]
pub struct U0Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0Status {
    // ======== Field Constants ========

    /// **CNT_THR_ZERO_LAT_U**
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CNT_THR_ZERO_LAT_U_OFFSET: u32 = 6;
    pub const CNT_THR_ZERO_LAT_U_MASK: u32 = 0x00000040;
    pub const CNT_THR_ZERO_LAT_U: u32 = 1 << 6;

    /// **CNT_THR_H_LIM_LAT_U**
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CNT_THR_H_LIM_LAT_U_OFFSET: u32 = 5;
    pub const CNT_THR_H_LIM_LAT_U_MASK: u32 = 0x00000020;
    pub const CNT_THR_H_LIM_LAT_U: u32 = 1 << 5;

    /// **CNT_THR_L_LIM_LAT_U**
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CNT_THR_L_LIM_LAT_U_OFFSET: u32 = 4;
    pub const CNT_THR_L_LIM_LAT_U_MASK: u32 = 0x00000010;
    pub const CNT_THR_L_LIM_LAT_U: u32 = 1 << 4;

    /// **CNT_THR_THRES0_LAT_U**
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_THRES0_LAT_U_OFFSET: u32 = 3;
    pub const CNT_THR_THRES0_LAT_U_MASK: u32 = 0x00000008;
    pub const CNT_THR_THRES0_LAT_U: u32 = 1 << 3;

    /// **CNT_THR_THRES1_LAT_U**
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_THRES1_LAT_U_OFFSET: u32 = 2;
    pub const CNT_THR_THRES1_LAT_U_MASK: u32 = 0x00000004;
    pub const CNT_THR_THRES1_LAT_U: u32 = 1 << 2;

    /// **CNT_THR_ZERO_MODE_U**
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CNT_THR_ZERO_MODE_U_OFFSET: u32 = 0;
    pub const CNT_THR_ZERO_MODE_U_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNT_THR_ZERO_LAT_U** `[6]` (RO)
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES0_LAT_U** `[3]` (RO)
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES1_LAT_U** `[2]` (RO)
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THR_ZERO_MODE_U_MASK)
            >> Self::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

/// U0_STATUS value type for batch read/modify/write operations
///
/// Use `u0status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0StatusVal {
    bits: u32,
}

impl U0StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THR_ZERO_LAT_U** `[6:6]`
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.bits & U0Status::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_H_LIM_LAT_U** `[5:5]`
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.bits & U0Status::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_L_LIM_LAT_U** `[4:4]`
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.bits & U0Status::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES0_LAT_U** `[3:3]`
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.bits & U0Status::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES1_LAT_U** `[2:2]`
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.bits & U0Status::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_ZERO_MODE_U** `[1:0]`
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub const fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.bits & U0Status::CNT_THR_ZERO_MODE_U_MASK) >> U0Status::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

impl Default for U0StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U0Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0StatusVal {
        U0StatusVal::from_bits(self.value.read_bits())
    }
}

/// U1_STATUS Register
///
/// PNCT UNIT1 status register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **CNT_THR_ZERO_LAT_U** `[6]` (RO)
///
///   The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
///
/// - **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
///
///   The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
///
/// - **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
///
///   The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
///
/// - **CNT_THR_THRES0_LAT_U** `[3]` (RO)
///
///   The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
///
/// - **CNT_THR_THRES1_LAT_U** `[2]` (RO)
///
///   The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
///
/// - **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
///
///   The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
#[repr(transparent)]
pub struct U1Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1Status {
    // ======== Field Constants ========

    /// **CNT_THR_ZERO_LAT_U**
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CNT_THR_ZERO_LAT_U_OFFSET: u32 = 6;
    pub const CNT_THR_ZERO_LAT_U_MASK: u32 = 0x00000040;
    pub const CNT_THR_ZERO_LAT_U: u32 = 1 << 6;

    /// **CNT_THR_H_LIM_LAT_U**
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CNT_THR_H_LIM_LAT_U_OFFSET: u32 = 5;
    pub const CNT_THR_H_LIM_LAT_U_MASK: u32 = 0x00000020;
    pub const CNT_THR_H_LIM_LAT_U: u32 = 1 << 5;

    /// **CNT_THR_L_LIM_LAT_U**
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CNT_THR_L_LIM_LAT_U_OFFSET: u32 = 4;
    pub const CNT_THR_L_LIM_LAT_U_MASK: u32 = 0x00000010;
    pub const CNT_THR_L_LIM_LAT_U: u32 = 1 << 4;

    /// **CNT_THR_THRES0_LAT_U**
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_THRES0_LAT_U_OFFSET: u32 = 3;
    pub const CNT_THR_THRES0_LAT_U_MASK: u32 = 0x00000008;
    pub const CNT_THR_THRES0_LAT_U: u32 = 1 << 3;

    /// **CNT_THR_THRES1_LAT_U**
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_THRES1_LAT_U_OFFSET: u32 = 2;
    pub const CNT_THR_THRES1_LAT_U_MASK: u32 = 0x00000004;
    pub const CNT_THR_THRES1_LAT_U: u32 = 1 << 2;

    /// **CNT_THR_ZERO_MODE_U**
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CNT_THR_ZERO_MODE_U_OFFSET: u32 = 0;
    pub const CNT_THR_ZERO_MODE_U_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNT_THR_ZERO_LAT_U** `[6]` (RO)
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES0_LAT_U** `[3]` (RO)
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES1_LAT_U** `[2]` (RO)
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THR_ZERO_MODE_U_MASK)
            >> Self::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

/// U1_STATUS value type for batch read/modify/write operations
///
/// Use `u1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1StatusVal {
    bits: u32,
}

impl U1StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THR_ZERO_LAT_U** `[6:6]`
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.bits & U1Status::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_H_LIM_LAT_U** `[5:5]`
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.bits & U1Status::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_L_LIM_LAT_U** `[4:4]`
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.bits & U1Status::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES0_LAT_U** `[3:3]`
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.bits & U1Status::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES1_LAT_U** `[2:2]`
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.bits & U1Status::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_ZERO_MODE_U** `[1:0]`
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub const fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.bits & U1Status::CNT_THR_ZERO_MODE_U_MASK) >> U1Status::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

impl Default for U1StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U1Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1StatusVal {
        U1StatusVal::from_bits(self.value.read_bits())
    }
}

/// U2_STATUS Register
///
/// PNCT UNIT2 status register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **CNT_THR_ZERO_LAT_U** `[6]` (RO)
///
///   The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
///
/// - **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
///
///   The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
///
/// - **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
///
///   The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
///
/// - **CNT_THR_THRES0_LAT_U** `[3]` (RO)
///
///   The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
///
/// - **CNT_THR_THRES1_LAT_U** `[2]` (RO)
///
///   The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
///
/// - **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
///
///   The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
#[repr(transparent)]
pub struct U2Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2Status {
    // ======== Field Constants ========

    /// **CNT_THR_ZERO_LAT_U**
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CNT_THR_ZERO_LAT_U_OFFSET: u32 = 6;
    pub const CNT_THR_ZERO_LAT_U_MASK: u32 = 0x00000040;
    pub const CNT_THR_ZERO_LAT_U: u32 = 1 << 6;

    /// **CNT_THR_H_LIM_LAT_U**
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CNT_THR_H_LIM_LAT_U_OFFSET: u32 = 5;
    pub const CNT_THR_H_LIM_LAT_U_MASK: u32 = 0x00000020;
    pub const CNT_THR_H_LIM_LAT_U: u32 = 1 << 5;

    /// **CNT_THR_L_LIM_LAT_U**
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CNT_THR_L_LIM_LAT_U_OFFSET: u32 = 4;
    pub const CNT_THR_L_LIM_LAT_U_MASK: u32 = 0x00000010;
    pub const CNT_THR_L_LIM_LAT_U: u32 = 1 << 4;

    /// **CNT_THR_THRES0_LAT_U**
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_THRES0_LAT_U_OFFSET: u32 = 3;
    pub const CNT_THR_THRES0_LAT_U_MASK: u32 = 0x00000008;
    pub const CNT_THR_THRES0_LAT_U: u32 = 1 << 3;

    /// **CNT_THR_THRES1_LAT_U**
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_THRES1_LAT_U_OFFSET: u32 = 2;
    pub const CNT_THR_THRES1_LAT_U_MASK: u32 = 0x00000004;
    pub const CNT_THR_THRES1_LAT_U: u32 = 1 << 2;

    /// **CNT_THR_ZERO_MODE_U**
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CNT_THR_ZERO_MODE_U_OFFSET: u32 = 0;
    pub const CNT_THR_ZERO_MODE_U_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNT_THR_ZERO_LAT_U** `[6]` (RO)
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES0_LAT_U** `[3]` (RO)
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES1_LAT_U** `[2]` (RO)
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THR_ZERO_MODE_U_MASK)
            >> Self::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

/// U2_STATUS value type for batch read/modify/write operations
///
/// Use `u2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2StatusVal {
    bits: u32,
}

impl U2StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THR_ZERO_LAT_U** `[6:6]`
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.bits & U2Status::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_H_LIM_LAT_U** `[5:5]`
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.bits & U2Status::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_L_LIM_LAT_U** `[4:4]`
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.bits & U2Status::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES0_LAT_U** `[3:3]`
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.bits & U2Status::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES1_LAT_U** `[2:2]`
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.bits & U2Status::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_ZERO_MODE_U** `[1:0]`
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub const fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.bits & U2Status::CNT_THR_ZERO_MODE_U_MASK) >> U2Status::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

impl Default for U2StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U2Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2StatusVal {
        U2StatusVal::from_bits(self.value.read_bits())
    }
}

/// U3_STATUS Register
///
/// PNCT UNIT3 status register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **CNT_THR_ZERO_LAT_U** `[6]` (RO)
///
///   The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
///
/// - **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
///
///   The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
///
/// - **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
///
///   The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
///
/// - **CNT_THR_THRES0_LAT_U** `[3]` (RO)
///
///   The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
///
/// - **CNT_THR_THRES1_LAT_U** `[2]` (RO)
///
///   The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
///
/// - **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
///
///   The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
#[repr(transparent)]
pub struct U3Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3Status {
    // ======== Field Constants ========

    /// **CNT_THR_ZERO_LAT_U**
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CNT_THR_ZERO_LAT_U_OFFSET: u32 = 6;
    pub const CNT_THR_ZERO_LAT_U_MASK: u32 = 0x00000040;
    pub const CNT_THR_ZERO_LAT_U: u32 = 1 << 6;

    /// **CNT_THR_H_LIM_LAT_U**
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CNT_THR_H_LIM_LAT_U_OFFSET: u32 = 5;
    pub const CNT_THR_H_LIM_LAT_U_MASK: u32 = 0x00000020;
    pub const CNT_THR_H_LIM_LAT_U: u32 = 1 << 5;

    /// **CNT_THR_L_LIM_LAT_U**
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CNT_THR_L_LIM_LAT_U_OFFSET: u32 = 4;
    pub const CNT_THR_L_LIM_LAT_U_MASK: u32 = 0x00000010;
    pub const CNT_THR_L_LIM_LAT_U: u32 = 1 << 4;

    /// **CNT_THR_THRES0_LAT_U**
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_THR_THRES0_LAT_U_OFFSET: u32 = 3;
    pub const CNT_THR_THRES0_LAT_U_MASK: u32 = 0x00000008;
    pub const CNT_THR_THRES0_LAT_U: u32 = 1 << 3;

    /// **CNT_THR_THRES1_LAT_U**
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CNT_THR_THRES1_LAT_U_OFFSET: u32 = 2;
    pub const CNT_THR_THRES1_LAT_U_MASK: u32 = 0x00000004;
    pub const CNT_THR_THRES1_LAT_U: u32 = 1 << 2;

    /// **CNT_THR_ZERO_MODE_U**
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CNT_THR_ZERO_MODE_U_OFFSET: u32 = 0;
    pub const CNT_THR_ZERO_MODE_U_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNT_THR_ZERO_LAT_U** `[6]` (RO)
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// **CNT_THR_H_LIM_LAT_U** `[5]` (RO)
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_L_LIM_LAT_U** `[4]` (RO)
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES0_LAT_U** `[3]` (RO)
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// **CNT_THR_THRES1_LAT_U** `[2]` (RO)
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.value.read_bits() & Self::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// **CNT_THR_ZERO_MODE_U** `[1:0]` (RO)
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_THR_ZERO_MODE_U_MASK)
            >> Self::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

/// U3_STATUS value type for batch read/modify/write operations
///
/// Use `u3status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3StatusVal {
    bits: u32,
}

impl U3StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_THR_ZERO_LAT_U** `[6:6]`
    ///
    /// The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_zero_lat_u(&self) -> bool {
        (self.bits & U3Status::CNT_THR_ZERO_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_H_LIM_LAT_U** `[5:5]`
    ///
    /// The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_h_lim_lat_u(&self) -> bool {
        (self.bits & U3Status::CNT_THR_H_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_L_LIM_LAT_U** `[4:4]`
    ///
    /// The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_l_lim_lat_u(&self) -> bool {
        (self.bits & U3Status::CNT_THR_L_LIM_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES0_LAT_U** `[3:3]`
    ///
    /// The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres0_lat_u(&self) -> bool {
        (self.bits & U3Status::CNT_THR_THRES0_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_THRES1_LAT_U** `[2:2]`
    ///
    /// The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others
    #[inline(always)]
    pub const fn cnt_thr_thres1_lat_u(&self) -> bool {
        (self.bits & U3Status::CNT_THR_THRES1_LAT_U_MASK) != 0
    }

    /// Read **CNT_THR_ZERO_MODE_U** `[1:0]`
    ///
    /// The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub const fn cnt_thr_zero_mode_u(&self) -> u32 {
        (self.bits & U3Status::CNT_THR_ZERO_MODE_U_MASK) >> U3Status::CNT_THR_ZERO_MODE_U_OFFSET
    }
}

impl Default for U3StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U3Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3StatusVal {
        U3StatusVal::from_bits(self.value.read_bits())
    }
}

/// CTRL Register
///
/// Control register for all counters
///
/// Address offset: 0x0060
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_EN** `[16]` (RW)
///
///   The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application
///
/// - **DALTA_CHANGE_EN_U3** `[11]` (RW)
///
///   Configures this bit to enable unit 3's step comparator.
///
/// - **DALTA_CHANGE_EN_U2** `[10]` (RW)
///
///   Configures this bit to enable unit 2's step comparator.
///
/// - **DALTA_CHANGE_EN_U1** `[9]` (RW)
///
///   Configures this bit to enable unit 1's step comparator.
///
/// - **DALTA_CHANGE_EN_U0** `[8]` (RW)
///
///   Configures this bit to enable unit 0's step comparator.
///
/// - **CNT_PAUSE_U3** `[7]` (RW)
///
///   Set this bit to freeze unit 3's counter.
///
/// - **PULSE_CNT_RST_U3** `[6]` (RW)
///
///   Set this bit to clear unit 3's counter.
///
/// - **CNT_PAUSE_U2** `[5]` (RW)
///
///   Set this bit to freeze unit 2's counter.
///
/// - **PULSE_CNT_RST_U2** `[4]` (RW)
///
///   Set this bit to clear unit 2's counter.
///
/// - **CNT_PAUSE_U1** `[3]` (RW)
///
///   Set this bit to freeze unit 1's counter.
///
/// - **PULSE_CNT_RST_U1** `[2]` (RW)
///
///   Set this bit to clear unit 1's counter.
///
/// - **CNT_PAUSE_U0** `[1]` (RW)
///
///   Set this bit to freeze unit 0's counter.
///
/// - **PULSE_CNT_RST_U0** `[0]` (RW)
///
///   Set this bit to clear unit 0's counter.
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

    /// **CLK_EN**
    ///
    /// The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_EN_OFFSET: u32 = 16;
    pub const CLK_EN_MASK: u32 = 0x00010000;
    pub const CLK_EN: u32 = 1 << 16;

    /// **DALTA_CHANGE_EN_U3**
    ///
    /// Configures this bit to enable unit 3's step comparator.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DALTA_CHANGE_EN_U3_OFFSET: u32 = 11;
    pub const DALTA_CHANGE_EN_U3_MASK: u32 = 0x00000800;
    pub const DALTA_CHANGE_EN_U3: u32 = 1 << 11;

    /// **DALTA_CHANGE_EN_U2**
    ///
    /// Configures this bit to enable unit 2's step comparator.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DALTA_CHANGE_EN_U2_OFFSET: u32 = 10;
    pub const DALTA_CHANGE_EN_U2_MASK: u32 = 0x00000400;
    pub const DALTA_CHANGE_EN_U2: u32 = 1 << 10;

    /// **DALTA_CHANGE_EN_U1**
    ///
    /// Configures this bit to enable unit 1's step comparator.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DALTA_CHANGE_EN_U1_OFFSET: u32 = 9;
    pub const DALTA_CHANGE_EN_U1_MASK: u32 = 0x00000200;
    pub const DALTA_CHANGE_EN_U1: u32 = 1 << 9;

    /// **DALTA_CHANGE_EN_U0**
    ///
    /// Configures this bit to enable unit 0's step comparator.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DALTA_CHANGE_EN_U0_OFFSET: u32 = 8;
    pub const DALTA_CHANGE_EN_U0_MASK: u32 = 0x00000100;
    pub const DALTA_CHANGE_EN_U0: u32 = 1 << 8;

    /// **CNT_PAUSE_U3**
    ///
    /// Set this bit to freeze unit 3's counter.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CNT_PAUSE_U3_OFFSET: u32 = 7;
    pub const CNT_PAUSE_U3_MASK: u32 = 0x00000080;
    pub const CNT_PAUSE_U3: u32 = 1 << 7;

    /// **PULSE_CNT_RST_U3**
    ///
    /// Set this bit to clear unit 3's counter.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PULSE_CNT_RST_U3_OFFSET: u32 = 6;
    pub const PULSE_CNT_RST_U3_MASK: u32 = 0x00000040;
    pub const PULSE_CNT_RST_U3: u32 = 1 << 6;

    /// **CNT_PAUSE_U2**
    ///
    /// Set this bit to freeze unit 2's counter.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CNT_PAUSE_U2_OFFSET: u32 = 5;
    pub const CNT_PAUSE_U2_MASK: u32 = 0x00000020;
    pub const CNT_PAUSE_U2: u32 = 1 << 5;

    /// **PULSE_CNT_RST_U2**
    ///
    /// Set this bit to clear unit 2's counter.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PULSE_CNT_RST_U2_OFFSET: u32 = 4;
    pub const PULSE_CNT_RST_U2_MASK: u32 = 0x00000010;
    pub const PULSE_CNT_RST_U2: u32 = 1 << 4;

    /// **CNT_PAUSE_U1**
    ///
    /// Set this bit to freeze unit 1's counter.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CNT_PAUSE_U1_OFFSET: u32 = 3;
    pub const CNT_PAUSE_U1_MASK: u32 = 0x00000008;
    pub const CNT_PAUSE_U1: u32 = 1 << 3;

    /// **PULSE_CNT_RST_U1**
    ///
    /// Set this bit to clear unit 1's counter.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PULSE_CNT_RST_U1_OFFSET: u32 = 2;
    pub const PULSE_CNT_RST_U1_MASK: u32 = 0x00000004;
    pub const PULSE_CNT_RST_U1: u32 = 1 << 2;

    /// **CNT_PAUSE_U0**
    ///
    /// Set this bit to freeze unit 0's counter.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CNT_PAUSE_U0_OFFSET: u32 = 1;
    pub const CNT_PAUSE_U0_MASK: u32 = 0x00000002;
    pub const CNT_PAUSE_U0: u32 = 1 << 1;

    /// **PULSE_CNT_RST_U0**
    ///
    /// Set this bit to clear unit 0's counter.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PULSE_CNT_RST_U0_OFFSET: u32 = 0;
    pub const PULSE_CNT_RST_U0_MASK: u32 = 0x00000001;
    pub const PULSE_CNT_RST_U0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[16]` (RW)
    ///
    /// The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application
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

    /// **DALTA_CHANGE_EN_U3** `[11]` (RW)
    ///
    /// Configures this bit to enable unit 3's step comparator.
    #[inline(always)]
    pub fn dalta_change_en_u3(&self) -> bool {
        (self.value.read_bits() & Self::DALTA_CHANGE_EN_U3_MASK) != 0
    }

    /// Set DALTA_CHANGE_EN_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dalta_change_en_u3(&self) {
        self.value
            .modify_bits(|r| r | Self::DALTA_CHANGE_EN_U3_MASK);
    }

    /// Clear DALTA_CHANGE_EN_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dalta_change_en_u3(&self) {
        self.value
            .modify_bits(|r| r & !Self::DALTA_CHANGE_EN_U3_MASK);
    }

    /// **DALTA_CHANGE_EN_U2** `[10]` (RW)
    ///
    /// Configures this bit to enable unit 2's step comparator.
    #[inline(always)]
    pub fn dalta_change_en_u2(&self) -> bool {
        (self.value.read_bits() & Self::DALTA_CHANGE_EN_U2_MASK) != 0
    }

    /// Set DALTA_CHANGE_EN_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dalta_change_en_u2(&self) {
        self.value
            .modify_bits(|r| r | Self::DALTA_CHANGE_EN_U2_MASK);
    }

    /// Clear DALTA_CHANGE_EN_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dalta_change_en_u2(&self) {
        self.value
            .modify_bits(|r| r & !Self::DALTA_CHANGE_EN_U2_MASK);
    }

    /// **DALTA_CHANGE_EN_U1** `[9]` (RW)
    ///
    /// Configures this bit to enable unit 1's step comparator.
    #[inline(always)]
    pub fn dalta_change_en_u1(&self) -> bool {
        (self.value.read_bits() & Self::DALTA_CHANGE_EN_U1_MASK) != 0
    }

    /// Set DALTA_CHANGE_EN_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dalta_change_en_u1(&self) {
        self.value
            .modify_bits(|r| r | Self::DALTA_CHANGE_EN_U1_MASK);
    }

    /// Clear DALTA_CHANGE_EN_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dalta_change_en_u1(&self) {
        self.value
            .modify_bits(|r| r & !Self::DALTA_CHANGE_EN_U1_MASK);
    }

    /// **DALTA_CHANGE_EN_U0** `[8]` (RW)
    ///
    /// Configures this bit to enable unit 0's step comparator.
    #[inline(always)]
    pub fn dalta_change_en_u0(&self) -> bool {
        (self.value.read_bits() & Self::DALTA_CHANGE_EN_U0_MASK) != 0
    }

    /// Set DALTA_CHANGE_EN_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dalta_change_en_u0(&self) {
        self.value
            .modify_bits(|r| r | Self::DALTA_CHANGE_EN_U0_MASK);
    }

    /// Clear DALTA_CHANGE_EN_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dalta_change_en_u0(&self) {
        self.value
            .modify_bits(|r| r & !Self::DALTA_CHANGE_EN_U0_MASK);
    }

    /// **CNT_PAUSE_U3** `[7]` (RW)
    ///
    /// Set this bit to freeze unit 3's counter.
    #[inline(always)]
    pub fn cnt_pause_u3(&self) -> bool {
        (self.value.read_bits() & Self::CNT_PAUSE_U3_MASK) != 0
    }

    /// Set CNT_PAUSE_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_pause_u3(&self) {
        self.value.modify_bits(|r| r | Self::CNT_PAUSE_U3_MASK);
    }

    /// Clear CNT_PAUSE_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_pause_u3(&self) {
        self.value.modify_bits(|r| r & !Self::CNT_PAUSE_U3_MASK);
    }

    /// **PULSE_CNT_RST_U3** `[6]` (RW)
    ///
    /// Set this bit to clear unit 3's counter.
    #[inline(always)]
    pub fn pulse_cnt_rst_u3(&self) -> bool {
        (self.value.read_bits() & Self::PULSE_CNT_RST_U3_MASK) != 0
    }

    /// Set PULSE_CNT_RST_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pulse_cnt_rst_u3(&self) {
        self.value.modify_bits(|r| r | Self::PULSE_CNT_RST_U3_MASK);
    }

    /// Clear PULSE_CNT_RST_U3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pulse_cnt_rst_u3(&self) {
        self.value.modify_bits(|r| r & !Self::PULSE_CNT_RST_U3_MASK);
    }

    /// **CNT_PAUSE_U2** `[5]` (RW)
    ///
    /// Set this bit to freeze unit 2's counter.
    #[inline(always)]
    pub fn cnt_pause_u2(&self) -> bool {
        (self.value.read_bits() & Self::CNT_PAUSE_U2_MASK) != 0
    }

    /// Set CNT_PAUSE_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_pause_u2(&self) {
        self.value.modify_bits(|r| r | Self::CNT_PAUSE_U2_MASK);
    }

    /// Clear CNT_PAUSE_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_pause_u2(&self) {
        self.value.modify_bits(|r| r & !Self::CNT_PAUSE_U2_MASK);
    }

    /// **PULSE_CNT_RST_U2** `[4]` (RW)
    ///
    /// Set this bit to clear unit 2's counter.
    #[inline(always)]
    pub fn pulse_cnt_rst_u2(&self) -> bool {
        (self.value.read_bits() & Self::PULSE_CNT_RST_U2_MASK) != 0
    }

    /// Set PULSE_CNT_RST_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pulse_cnt_rst_u2(&self) {
        self.value.modify_bits(|r| r | Self::PULSE_CNT_RST_U2_MASK);
    }

    /// Clear PULSE_CNT_RST_U2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pulse_cnt_rst_u2(&self) {
        self.value.modify_bits(|r| r & !Self::PULSE_CNT_RST_U2_MASK);
    }

    /// **CNT_PAUSE_U1** `[3]` (RW)
    ///
    /// Set this bit to freeze unit 1's counter.
    #[inline(always)]
    pub fn cnt_pause_u1(&self) -> bool {
        (self.value.read_bits() & Self::CNT_PAUSE_U1_MASK) != 0
    }

    /// Set CNT_PAUSE_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_pause_u1(&self) {
        self.value.modify_bits(|r| r | Self::CNT_PAUSE_U1_MASK);
    }

    /// Clear CNT_PAUSE_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_pause_u1(&self) {
        self.value.modify_bits(|r| r & !Self::CNT_PAUSE_U1_MASK);
    }

    /// **PULSE_CNT_RST_U1** `[2]` (RW)
    ///
    /// Set this bit to clear unit 1's counter.
    #[inline(always)]
    pub fn pulse_cnt_rst_u1(&self) -> bool {
        (self.value.read_bits() & Self::PULSE_CNT_RST_U1_MASK) != 0
    }

    /// Set PULSE_CNT_RST_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pulse_cnt_rst_u1(&self) {
        self.value.modify_bits(|r| r | Self::PULSE_CNT_RST_U1_MASK);
    }

    /// Clear PULSE_CNT_RST_U1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pulse_cnt_rst_u1(&self) {
        self.value.modify_bits(|r| r & !Self::PULSE_CNT_RST_U1_MASK);
    }

    /// **CNT_PAUSE_U0** `[1]` (RW)
    ///
    /// Set this bit to freeze unit 0's counter.
    #[inline(always)]
    pub fn cnt_pause_u0(&self) -> bool {
        (self.value.read_bits() & Self::CNT_PAUSE_U0_MASK) != 0
    }

    /// Set CNT_PAUSE_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cnt_pause_u0(&self) {
        self.value.modify_bits(|r| r | Self::CNT_PAUSE_U0_MASK);
    }

    /// Clear CNT_PAUSE_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cnt_pause_u0(&self) {
        self.value.modify_bits(|r| r & !Self::CNT_PAUSE_U0_MASK);
    }

    /// **PULSE_CNT_RST_U0** `[0]` (RW)
    ///
    /// Set this bit to clear unit 0's counter.
    #[inline(always)]
    pub fn pulse_cnt_rst_u0(&self) -> bool {
        (self.value.read_bits() & Self::PULSE_CNT_RST_U0_MASK) != 0
    }

    /// Set PULSE_CNT_RST_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pulse_cnt_rst_u0(&self) {
        self.value.modify_bits(|r| r | Self::PULSE_CNT_RST_U0_MASK);
    }

    /// Clear PULSE_CNT_RST_U0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pulse_cnt_rst_u0(&self) {
        self.value.modify_bits(|r| r & !Self::PULSE_CNT_RST_U0_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_EN** `[16:16]`
    ///
    /// The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Ctrl::CLK_EN_MASK) != 0
    }

    /// Read **DALTA_CHANGE_EN_U3** `[11:11]`
    ///
    /// Configures this bit to enable unit 3's step comparator.
    #[inline(always)]
    pub const fn dalta_change_en_u3(&self) -> bool {
        (self.bits & Ctrl::DALTA_CHANGE_EN_U3_MASK) != 0
    }

    /// Read **DALTA_CHANGE_EN_U2** `[10:10]`
    ///
    /// Configures this bit to enable unit 2's step comparator.
    #[inline(always)]
    pub const fn dalta_change_en_u2(&self) -> bool {
        (self.bits & Ctrl::DALTA_CHANGE_EN_U2_MASK) != 0
    }

    /// Read **DALTA_CHANGE_EN_U1** `[9:9]`
    ///
    /// Configures this bit to enable unit 1's step comparator.
    #[inline(always)]
    pub const fn dalta_change_en_u1(&self) -> bool {
        (self.bits & Ctrl::DALTA_CHANGE_EN_U1_MASK) != 0
    }

    /// Read **DALTA_CHANGE_EN_U0** `[8:8]`
    ///
    /// Configures this bit to enable unit 0's step comparator.
    #[inline(always)]
    pub const fn dalta_change_en_u0(&self) -> bool {
        (self.bits & Ctrl::DALTA_CHANGE_EN_U0_MASK) != 0
    }

    /// Read **CNT_PAUSE_U3** `[7:7]`
    ///
    /// Set this bit to freeze unit 3's counter.
    #[inline(always)]
    pub const fn cnt_pause_u3(&self) -> bool {
        (self.bits & Ctrl::CNT_PAUSE_U3_MASK) != 0
    }

    /// Read **PULSE_CNT_RST_U3** `[6:6]`
    ///
    /// Set this bit to clear unit 3's counter.
    #[inline(always)]
    pub const fn pulse_cnt_rst_u3(&self) -> bool {
        (self.bits & Ctrl::PULSE_CNT_RST_U3_MASK) != 0
    }

    /// Read **CNT_PAUSE_U2** `[5:5]`
    ///
    /// Set this bit to freeze unit 2's counter.
    #[inline(always)]
    pub const fn cnt_pause_u2(&self) -> bool {
        (self.bits & Ctrl::CNT_PAUSE_U2_MASK) != 0
    }

    /// Read **PULSE_CNT_RST_U2** `[4:4]`
    ///
    /// Set this bit to clear unit 2's counter.
    #[inline(always)]
    pub const fn pulse_cnt_rst_u2(&self) -> bool {
        (self.bits & Ctrl::PULSE_CNT_RST_U2_MASK) != 0
    }

    /// Read **CNT_PAUSE_U1** `[3:3]`
    ///
    /// Set this bit to freeze unit 1's counter.
    #[inline(always)]
    pub const fn cnt_pause_u1(&self) -> bool {
        (self.bits & Ctrl::CNT_PAUSE_U1_MASK) != 0
    }

    /// Read **PULSE_CNT_RST_U1** `[2:2]`
    ///
    /// Set this bit to clear unit 1's counter.
    #[inline(always)]
    pub const fn pulse_cnt_rst_u1(&self) -> bool {
        (self.bits & Ctrl::PULSE_CNT_RST_U1_MASK) != 0
    }

    /// Read **CNT_PAUSE_U0** `[1:1]`
    ///
    /// Set this bit to freeze unit 0's counter.
    #[inline(always)]
    pub const fn cnt_pause_u0(&self) -> bool {
        (self.bits & Ctrl::CNT_PAUSE_U0_MASK) != 0
    }

    /// Read **PULSE_CNT_RST_U0** `[0:0]`
    ///
    /// Set this bit to clear unit 0's counter.
    #[inline(always)]
    pub const fn pulse_cnt_rst_u0(&self) -> bool {
        (self.bits & Ctrl::PULSE_CNT_RST_U0_MASK) != 0
    }

    /// Set **CLK_EN** `[16:16]`
    ///
    /// The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CLK_EN_MASK
            } else {
                self.bits & !Ctrl::CLK_EN_MASK
            },
        }
    }

    /// Set **DALTA_CHANGE_EN_U3** `[11:11]`
    ///
    /// Configures this bit to enable unit 3's step comparator.
    #[inline(always)]
    pub const fn with_dalta_change_en_u3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DALTA_CHANGE_EN_U3_MASK
            } else {
                self.bits & !Ctrl::DALTA_CHANGE_EN_U3_MASK
            },
        }
    }

    /// Set **DALTA_CHANGE_EN_U2** `[10:10]`
    ///
    /// Configures this bit to enable unit 2's step comparator.
    #[inline(always)]
    pub const fn with_dalta_change_en_u2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DALTA_CHANGE_EN_U2_MASK
            } else {
                self.bits & !Ctrl::DALTA_CHANGE_EN_U2_MASK
            },
        }
    }

    /// Set **DALTA_CHANGE_EN_U1** `[9:9]`
    ///
    /// Configures this bit to enable unit 1's step comparator.
    #[inline(always)]
    pub const fn with_dalta_change_en_u1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DALTA_CHANGE_EN_U1_MASK
            } else {
                self.bits & !Ctrl::DALTA_CHANGE_EN_U1_MASK
            },
        }
    }

    /// Set **DALTA_CHANGE_EN_U0** `[8:8]`
    ///
    /// Configures this bit to enable unit 0's step comparator.
    #[inline(always)]
    pub const fn with_dalta_change_en_u0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DALTA_CHANGE_EN_U0_MASK
            } else {
                self.bits & !Ctrl::DALTA_CHANGE_EN_U0_MASK
            },
        }
    }

    /// Set **CNT_PAUSE_U3** `[7:7]`
    ///
    /// Set this bit to freeze unit 3's counter.
    #[inline(always)]
    pub const fn with_cnt_pause_u3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CNT_PAUSE_U3_MASK
            } else {
                self.bits & !Ctrl::CNT_PAUSE_U3_MASK
            },
        }
    }

    /// Set **PULSE_CNT_RST_U3** `[6:6]`
    ///
    /// Set this bit to clear unit 3's counter.
    #[inline(always)]
    pub const fn with_pulse_cnt_rst_u3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PULSE_CNT_RST_U3_MASK
            } else {
                self.bits & !Ctrl::PULSE_CNT_RST_U3_MASK
            },
        }
    }

    /// Set **CNT_PAUSE_U2** `[5:5]`
    ///
    /// Set this bit to freeze unit 2's counter.
    #[inline(always)]
    pub const fn with_cnt_pause_u2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CNT_PAUSE_U2_MASK
            } else {
                self.bits & !Ctrl::CNT_PAUSE_U2_MASK
            },
        }
    }

    /// Set **PULSE_CNT_RST_U2** `[4:4]`
    ///
    /// Set this bit to clear unit 2's counter.
    #[inline(always)]
    pub const fn with_pulse_cnt_rst_u2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PULSE_CNT_RST_U2_MASK
            } else {
                self.bits & !Ctrl::PULSE_CNT_RST_U2_MASK
            },
        }
    }

    /// Set **CNT_PAUSE_U1** `[3:3]`
    ///
    /// Set this bit to freeze unit 1's counter.
    #[inline(always)]
    pub const fn with_cnt_pause_u1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CNT_PAUSE_U1_MASK
            } else {
                self.bits & !Ctrl::CNT_PAUSE_U1_MASK
            },
        }
    }

    /// Set **PULSE_CNT_RST_U1** `[2:2]`
    ///
    /// Set this bit to clear unit 1's counter.
    #[inline(always)]
    pub const fn with_pulse_cnt_rst_u1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PULSE_CNT_RST_U1_MASK
            } else {
                self.bits & !Ctrl::PULSE_CNT_RST_U1_MASK
            },
        }
    }

    /// Set **CNT_PAUSE_U0** `[1:1]`
    ///
    /// Set this bit to freeze unit 0's counter.
    #[inline(always)]
    pub const fn with_cnt_pause_u0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::CNT_PAUSE_U0_MASK
            } else {
                self.bits & !Ctrl::CNT_PAUSE_U0_MASK
            },
        }
    }

    /// Set **PULSE_CNT_RST_U0** `[0:0]`
    ///
    /// Set this bit to clear unit 0's counter.
    #[inline(always)]
    pub const fn with_pulse_cnt_rst_u0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PULSE_CNT_RST_U0_MASK
            } else {
                self.bits & !Ctrl::PULSE_CNT_RST_U0_MASK
            },
        }
    }
}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
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

/// U3_CHANGE_CONF Register
///
/// Configuration register for unit $n's step value.
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **CNT_STEP_LIM_U3** `[31:16]` (RW)
///
///   Configures the step limit value for unit 3.
///
/// - **CNT_STEP_U3** `[15:0]` (RW)
///
///   Configures the step value for unit 3.
#[repr(transparent)]
pub struct U3ChangeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U3ChangeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U3ChangeConf {
    // ======== Field Constants ========

    /// **CNT_STEP_LIM_U3**
    ///
    /// Configures the step limit value for unit 3.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_STEP_LIM_U3_OFFSET: u32 = 16;
    pub const CNT_STEP_LIM_U3_MASK: u32 = 0xffff0000;

    /// **CNT_STEP_U3**
    ///
    /// Configures the step value for unit 3.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_STEP_U3_OFFSET: u32 = 0;
    pub const CNT_STEP_U3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_STEP_LIM_U3** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 3.
    #[inline(always)]
    pub fn cnt_step_lim_u3(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_LIM_U3_MASK) >> Self::CNT_STEP_LIM_U3_OFFSET
    }

    /// Write **CNT_STEP_LIM_U3** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 3.
    #[inline(always)]
    pub fn write_cnt_step_lim_u3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_LIM_U3_MASK)
                | ((value << Self::CNT_STEP_LIM_U3_OFFSET) & Self::CNT_STEP_LIM_U3_MASK)
        });
    }

    /// **CNT_STEP_U3** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 3.
    #[inline(always)]
    pub fn cnt_step_u3(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_U3_MASK) >> Self::CNT_STEP_U3_OFFSET
    }

    /// Write **CNT_STEP_U3** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 3.
    #[inline(always)]
    pub fn write_cnt_step_u3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_U3_MASK)
                | ((value << Self::CNT_STEP_U3_OFFSET) & Self::CNT_STEP_U3_MASK)
        });
    }
}

/// U3_CHANGE_CONF value type for batch read/modify/write operations
///
/// Use `u3changeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u3changeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U3ChangeConfVal {
    bits: u32,
}

impl U3ChangeConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_STEP_LIM_U3** `[31:16]`
    ///
    /// Configures the step limit value for unit 3.
    #[inline(always)]
    pub const fn cnt_step_lim_u3(&self) -> u32 {
        (self.bits & U3ChangeConf::CNT_STEP_LIM_U3_MASK) >> U3ChangeConf::CNT_STEP_LIM_U3_OFFSET
    }

    /// Read **CNT_STEP_U3** `[15:0]`
    ///
    /// Configures the step value for unit 3.
    #[inline(always)]
    pub const fn cnt_step_u3(&self) -> u32 {
        (self.bits & U3ChangeConf::CNT_STEP_U3_MASK) >> U3ChangeConf::CNT_STEP_U3_OFFSET
    }

    /// Set **CNT_STEP_LIM_U3** `[31:16]`
    ///
    /// Configures the step limit value for unit 3.
    #[inline(always)]
    pub const fn with_cnt_step_lim_u3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3ChangeConf::CNT_STEP_LIM_U3_MASK)
                | ((value << U3ChangeConf::CNT_STEP_LIM_U3_OFFSET)
                    & U3ChangeConf::CNT_STEP_LIM_U3_MASK),
        }
    }

    /// Set **CNT_STEP_U3** `[15:0]`
    ///
    /// Configures the step value for unit 3.
    #[inline(always)]
    pub const fn with_cnt_step_u3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U3ChangeConf::CNT_STEP_U3_MASK)
                | ((value << U3ChangeConf::CNT_STEP_U3_OFFSET) & U3ChangeConf::CNT_STEP_U3_MASK),
        }
    }
}

impl Default for U3ChangeConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U3ChangeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U3ChangeConfVal {
        U3ChangeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U3ChangeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U3ChangeConfVal) -> U3ChangeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U2_CHANGE_CONF Register
///
/// Configuration register for unit $n's step value.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **CNT_STEP_LIM_U2** `[31:16]` (RW)
///
///   Configures the step limit value for unit 2.
///
/// - **CNT_STEP_U2** `[15:0]` (RW)
///
///   Configures the step value for unit 2.
#[repr(transparent)]
pub struct U2ChangeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U2ChangeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U2ChangeConf {
    // ======== Field Constants ========

    /// **CNT_STEP_LIM_U2**
    ///
    /// Configures the step limit value for unit 2.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_STEP_LIM_U2_OFFSET: u32 = 16;
    pub const CNT_STEP_LIM_U2_MASK: u32 = 0xffff0000;

    /// **CNT_STEP_U2**
    ///
    /// Configures the step value for unit 2.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_STEP_U2_OFFSET: u32 = 0;
    pub const CNT_STEP_U2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_STEP_LIM_U2** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 2.
    #[inline(always)]
    pub fn cnt_step_lim_u2(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_LIM_U2_MASK) >> Self::CNT_STEP_LIM_U2_OFFSET
    }

    /// Write **CNT_STEP_LIM_U2** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 2.
    #[inline(always)]
    pub fn write_cnt_step_lim_u2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_LIM_U2_MASK)
                | ((value << Self::CNT_STEP_LIM_U2_OFFSET) & Self::CNT_STEP_LIM_U2_MASK)
        });
    }

    /// **CNT_STEP_U2** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 2.
    #[inline(always)]
    pub fn cnt_step_u2(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_U2_MASK) >> Self::CNT_STEP_U2_OFFSET
    }

    /// Write **CNT_STEP_U2** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 2.
    #[inline(always)]
    pub fn write_cnt_step_u2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_U2_MASK)
                | ((value << Self::CNT_STEP_U2_OFFSET) & Self::CNT_STEP_U2_MASK)
        });
    }
}

/// U2_CHANGE_CONF value type for batch read/modify/write operations
///
/// Use `u2changeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u2changeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U2ChangeConfVal {
    bits: u32,
}

impl U2ChangeConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_STEP_LIM_U2** `[31:16]`
    ///
    /// Configures the step limit value for unit 2.
    #[inline(always)]
    pub const fn cnt_step_lim_u2(&self) -> u32 {
        (self.bits & U2ChangeConf::CNT_STEP_LIM_U2_MASK) >> U2ChangeConf::CNT_STEP_LIM_U2_OFFSET
    }

    /// Read **CNT_STEP_U2** `[15:0]`
    ///
    /// Configures the step value for unit 2.
    #[inline(always)]
    pub const fn cnt_step_u2(&self) -> u32 {
        (self.bits & U2ChangeConf::CNT_STEP_U2_MASK) >> U2ChangeConf::CNT_STEP_U2_OFFSET
    }

    /// Set **CNT_STEP_LIM_U2** `[31:16]`
    ///
    /// Configures the step limit value for unit 2.
    #[inline(always)]
    pub const fn with_cnt_step_lim_u2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2ChangeConf::CNT_STEP_LIM_U2_MASK)
                | ((value << U2ChangeConf::CNT_STEP_LIM_U2_OFFSET)
                    & U2ChangeConf::CNT_STEP_LIM_U2_MASK),
        }
    }

    /// Set **CNT_STEP_U2** `[15:0]`
    ///
    /// Configures the step value for unit 2.
    #[inline(always)]
    pub const fn with_cnt_step_u2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U2ChangeConf::CNT_STEP_U2_MASK)
                | ((value << U2ChangeConf::CNT_STEP_U2_OFFSET) & U2ChangeConf::CNT_STEP_U2_MASK),
        }
    }
}

impl Default for U2ChangeConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U2ChangeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U2ChangeConfVal {
        U2ChangeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U2ChangeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U2ChangeConfVal) -> U2ChangeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U1_CHANGE_CONF Register
///
/// Configuration register for unit $n's step value.
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **CNT_STEP_LIM_U1** `[31:16]` (RW)
///
///   Configures the step limit value for unit 1.
///
/// - **CNT_STEP_U1** `[15:0]` (RW)
///
///   Configures the step value for unit 1.
#[repr(transparent)]
pub struct U1ChangeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U1ChangeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U1ChangeConf {
    // ======== Field Constants ========

    /// **CNT_STEP_LIM_U1**
    ///
    /// Configures the step limit value for unit 1.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_STEP_LIM_U1_OFFSET: u32 = 16;
    pub const CNT_STEP_LIM_U1_MASK: u32 = 0xffff0000;

    /// **CNT_STEP_U1**
    ///
    /// Configures the step value for unit 1.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_STEP_U1_OFFSET: u32 = 0;
    pub const CNT_STEP_U1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_STEP_LIM_U1** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 1.
    #[inline(always)]
    pub fn cnt_step_lim_u1(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_LIM_U1_MASK) >> Self::CNT_STEP_LIM_U1_OFFSET
    }

    /// Write **CNT_STEP_LIM_U1** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 1.
    #[inline(always)]
    pub fn write_cnt_step_lim_u1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_LIM_U1_MASK)
                | ((value << Self::CNT_STEP_LIM_U1_OFFSET) & Self::CNT_STEP_LIM_U1_MASK)
        });
    }

    /// **CNT_STEP_U1** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 1.
    #[inline(always)]
    pub fn cnt_step_u1(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_U1_MASK) >> Self::CNT_STEP_U1_OFFSET
    }

    /// Write **CNT_STEP_U1** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 1.
    #[inline(always)]
    pub fn write_cnt_step_u1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_U1_MASK)
                | ((value << Self::CNT_STEP_U1_OFFSET) & Self::CNT_STEP_U1_MASK)
        });
    }
}

/// U1_CHANGE_CONF value type for batch read/modify/write operations
///
/// Use `u1changeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u1changeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U1ChangeConfVal {
    bits: u32,
}

impl U1ChangeConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_STEP_LIM_U1** `[31:16]`
    ///
    /// Configures the step limit value for unit 1.
    #[inline(always)]
    pub const fn cnt_step_lim_u1(&self) -> u32 {
        (self.bits & U1ChangeConf::CNT_STEP_LIM_U1_MASK) >> U1ChangeConf::CNT_STEP_LIM_U1_OFFSET
    }

    /// Read **CNT_STEP_U1** `[15:0]`
    ///
    /// Configures the step value for unit 1.
    #[inline(always)]
    pub const fn cnt_step_u1(&self) -> u32 {
        (self.bits & U1ChangeConf::CNT_STEP_U1_MASK) >> U1ChangeConf::CNT_STEP_U1_OFFSET
    }

    /// Set **CNT_STEP_LIM_U1** `[31:16]`
    ///
    /// Configures the step limit value for unit 1.
    #[inline(always)]
    pub const fn with_cnt_step_lim_u1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1ChangeConf::CNT_STEP_LIM_U1_MASK)
                | ((value << U1ChangeConf::CNT_STEP_LIM_U1_OFFSET)
                    & U1ChangeConf::CNT_STEP_LIM_U1_MASK),
        }
    }

    /// Set **CNT_STEP_U1** `[15:0]`
    ///
    /// Configures the step value for unit 1.
    #[inline(always)]
    pub const fn with_cnt_step_u1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U1ChangeConf::CNT_STEP_U1_MASK)
                | ((value << U1ChangeConf::CNT_STEP_U1_OFFSET) & U1ChangeConf::CNT_STEP_U1_MASK),
        }
    }
}

impl Default for U1ChangeConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U1ChangeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U1ChangeConfVal {
        U1ChangeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U1ChangeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U1ChangeConfVal) -> U1ChangeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// U0_CHANGE_CONF Register
///
/// Configuration register for unit $n's step value.
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **CNT_STEP_LIM_U0** `[31:16]` (RW)
///
///   Configures the step limit value for unit 0.
///
/// - **CNT_STEP_U0** `[15:0]` (RW)
///
///   Configures the step value for unit 0.
#[repr(transparent)]
pub struct U0ChangeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for U0ChangeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl U0ChangeConf {
    // ======== Field Constants ========

    /// **CNT_STEP_LIM_U0**
    ///
    /// Configures the step limit value for unit 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const CNT_STEP_LIM_U0_OFFSET: u32 = 16;
    pub const CNT_STEP_LIM_U0_MASK: u32 = 0xffff0000;

    /// **CNT_STEP_U0**
    ///
    /// Configures the step value for unit 0.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_STEP_U0_OFFSET: u32 = 0;
    pub const CNT_STEP_U0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT_STEP_LIM_U0** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 0.
    #[inline(always)]
    pub fn cnt_step_lim_u0(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_LIM_U0_MASK) >> Self::CNT_STEP_LIM_U0_OFFSET
    }

    /// Write **CNT_STEP_LIM_U0** `[31:16]` (RW)
    ///
    /// Configures the step limit value for unit 0.
    #[inline(always)]
    pub fn write_cnt_step_lim_u0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_LIM_U0_MASK)
                | ((value << Self::CNT_STEP_LIM_U0_OFFSET) & Self::CNT_STEP_LIM_U0_MASK)
        });
    }

    /// **CNT_STEP_U0** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 0.
    #[inline(always)]
    pub fn cnt_step_u0(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_STEP_U0_MASK) >> Self::CNT_STEP_U0_OFFSET
    }

    /// Write **CNT_STEP_U0** `[15:0]` (RW)
    ///
    /// Configures the step value for unit 0.
    #[inline(always)]
    pub fn write_cnt_step_u0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_STEP_U0_MASK)
                | ((value << Self::CNT_STEP_U0_OFFSET) & Self::CNT_STEP_U0_MASK)
        });
    }
}

/// U0_CHANGE_CONF value type for batch read/modify/write operations
///
/// Use `u0changeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `u0changeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct U0ChangeConfVal {
    bits: u32,
}

impl U0ChangeConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CNT_STEP_LIM_U0** `[31:16]`
    ///
    /// Configures the step limit value for unit 0.
    #[inline(always)]
    pub const fn cnt_step_lim_u0(&self) -> u32 {
        (self.bits & U0ChangeConf::CNT_STEP_LIM_U0_MASK) >> U0ChangeConf::CNT_STEP_LIM_U0_OFFSET
    }

    /// Read **CNT_STEP_U0** `[15:0]`
    ///
    /// Configures the step value for unit 0.
    #[inline(always)]
    pub const fn cnt_step_u0(&self) -> u32 {
        (self.bits & U0ChangeConf::CNT_STEP_U0_MASK) >> U0ChangeConf::CNT_STEP_U0_OFFSET
    }

    /// Set **CNT_STEP_LIM_U0** `[31:16]`
    ///
    /// Configures the step limit value for unit 0.
    #[inline(always)]
    pub const fn with_cnt_step_lim_u0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0ChangeConf::CNT_STEP_LIM_U0_MASK)
                | ((value << U0ChangeConf::CNT_STEP_LIM_U0_OFFSET)
                    & U0ChangeConf::CNT_STEP_LIM_U0_MASK),
        }
    }

    /// Set **CNT_STEP_U0** `[15:0]`
    ///
    /// Configures the step value for unit 0.
    #[inline(always)]
    pub const fn with_cnt_step_u0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !U0ChangeConf::CNT_STEP_U0_MASK)
                | ((value << U0ChangeConf::CNT_STEP_U0_OFFSET) & U0ChangeConf::CNT_STEP_U0_MASK),
        }
    }
}

impl Default for U0ChangeConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl U0ChangeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> U0ChangeConfVal {
        U0ChangeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: U0ChangeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(U0ChangeConfVal) -> U0ChangeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// PCNT version control register
///
/// Address offset: 0x00fc
/// Reset value: 0x22091900
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   This is the PCNT version control register.
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
    /// This is the PCNT version control register.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// This is the PCNT version control register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// This is the PCNT version control register.
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
        Self { bits: 0x22091900 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// This is the PCNT version control register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// This is the PCNT version control register.
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
    /// Configuration register 0 for unit 0
    pub u0_conf0: U0Conf0,
    /// 0x0004
    /// Configuration register 1 for unit 0
    pub u0_conf1: U0Conf1,
    /// 0x0008
    /// Configuration register 2 for unit 0
    pub u0_conf2: U0Conf2,
    /// 0x000c
    /// Configuration register 0 for unit 1
    pub u1_conf0: U1Conf0,
    /// 0x0010
    /// Configuration register 1 for unit 1
    pub u1_conf1: U1Conf1,
    /// 0x0014
    /// Configuration register 2 for unit 1
    pub u1_conf2: U1Conf2,
    /// 0x0018
    /// Configuration register 0 for unit 2
    pub u2_conf0: U2Conf0,
    /// 0x001c
    /// Configuration register 1 for unit 2
    pub u2_conf1: U2Conf1,
    /// 0x0020
    /// Configuration register 2 for unit 2
    pub u2_conf2: U2Conf2,
    /// 0x0024
    /// Configuration register 0 for unit 3
    pub u3_conf0: U3Conf0,
    /// 0x0028
    /// Configuration register 1 for unit 3
    pub u3_conf1: U3Conf1,
    /// 0x002c
    /// Configuration register 2 for unit 3
    pub u3_conf2: U3Conf2,
    /// 0x0030
    /// Counter value for unit 0
    pub u0_cnt: U0Cnt,
    /// 0x0034
    /// Counter value for unit 1
    pub u1_cnt: U1Cnt,
    /// 0x0038
    /// Counter value for unit 2
    pub u2_cnt: U2Cnt,
    /// 0x003c
    /// Counter value for unit 3
    pub u3_cnt: U3Cnt,
    /// 0x0040
    /// Interrupt raw status register
    pub int_raw: IntRaw,
    /// 0x0044
    /// Interrupt status register
    pub int_st: IntSt,
    /// 0x0048
    /// Interrupt enable register
    pub int_ena: IntEna,
    /// 0x004c
    /// Interrupt clear register
    pub int_clr: IntClr,
    /// 0x0050
    /// PNCT UNIT0 status register
    pub u0_status: U0Status,
    /// 0x0054
    /// PNCT UNIT1 status register
    pub u1_status: U1Status,
    /// 0x0058
    /// PNCT UNIT2 status register
    pub u2_status: U2Status,
    /// 0x005c
    /// PNCT UNIT3 status register
    pub u3_status: U3Status,
    /// 0x0060
    /// Control register for all counters
    pub ctrl: Ctrl,
    /// 0x0064
    /// Configuration register for unit $n's step value.
    pub u3_change_conf: U3ChangeConf,
    /// 0x0068
    /// Configuration register for unit $n's step value.
    pub u2_change_conf: U2ChangeConf,
    /// 0x006c
    /// Configuration register for unit $n's step value.
    pub u1_change_conf: U1ChangeConf,
    /// 0x0070
    /// Configuration register for unit $n's step value.
    pub u0_change_conf: U0ChangeConf,
    _reserved0: [u8; 136],
    /// 0x00fc
    /// PCNT version control register
    pub date: Date,
}
