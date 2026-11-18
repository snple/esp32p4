//! LCD_CAM
//! Camera/LCD Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x500dc000;

/// LCD_CLOCK Register
///
/// LCD clock config register.
///
/// Address offset: 0x0000
/// Reset value: 0x00000843
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Set this bit to enable clk gate
///
/// - **LCD_CLK_SEL** `[30:29]` (RW)
///
///   Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
///
/// - **LCD_CLKM_DIV_A** `[28:23]` (RW)
///
///   Fractional clock divider denominator value
///
/// - **LCD_CLKM_DIV_B** `[22:17]` (RW)
///
///   Fractional clock divider numerator value
///
/// - **LCD_CLKM_DIV_NUM** `[16:9]` (RW)
///
///   Integral LCD clock divider value
///
/// - **LCD_CK_OUT_EDGE** `[8]` (RW)
///
///   1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle.
///
/// - **LCD_CK_IDLE_EDGE** `[7]` (RW)
///
///   1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle.
///
/// - **LCD_CLK_EQU_SYSCLK** `[6]` (RW)
///
///   1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1).
///
/// - **LCD_CLKCNT_N** `[5:0]` (RW)
///
///   f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
#[repr(transparent)]
pub struct LcdClock {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdClock {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdClock {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Set this bit to enable clk gate
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **LCD_CLK_SEL**
    ///
    /// Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const LCD_CLK_SEL_OFFSET: u32 = 29;
    pub const LCD_CLK_SEL_MASK: u32 = 0x60000000;

    /// **LCD_CLKM_DIV_A**
    ///
    /// Fractional clock divider denominator value
    ///
    /// Bits `[28:23]`, mask `0x1f800000`
    pub const LCD_CLKM_DIV_A_OFFSET: u32 = 23;
    pub const LCD_CLKM_DIV_A_MASK: u32 = 0x1f800000;

    /// **LCD_CLKM_DIV_B**
    ///
    /// Fractional clock divider numerator value
    ///
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const LCD_CLKM_DIV_B_OFFSET: u32 = 17;
    pub const LCD_CLKM_DIV_B_MASK: u32 = 0x007e0000;

    /// **LCD_CLKM_DIV_NUM**
    ///
    /// Integral LCD clock divider value
    ///
    /// Bits `[16:9]`, mask `0x0001fe00`
    pub const LCD_CLKM_DIV_NUM_OFFSET: u32 = 9;
    pub const LCD_CLKM_DIV_NUM_MASK: u32 = 0x0001fe00;

    /// **LCD_CK_OUT_EDGE**
    ///
    /// 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LCD_CK_OUT_EDGE_OFFSET: u32 = 8;
    pub const LCD_CK_OUT_EDGE_MASK: u32 = 0x00000100;
    pub const LCD_CK_OUT_EDGE: u32 = 1 << 8;

    /// **LCD_CK_IDLE_EDGE**
    ///
    /// 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LCD_CK_IDLE_EDGE_OFFSET: u32 = 7;
    pub const LCD_CK_IDLE_EDGE_MASK: u32 = 0x00000080;
    pub const LCD_CK_IDLE_EDGE: u32 = 1 << 7;

    /// **LCD_CLK_EQU_SYSCLK**
    ///
    /// 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1).
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LCD_CLK_EQU_SYSCLK_OFFSET: u32 = 6;
    pub const LCD_CLK_EQU_SYSCLK_MASK: u32 = 0x00000040;
    pub const LCD_CLK_EQU_SYSCLK: u32 = 1 << 6;

    /// **LCD_CLKCNT_N**
    ///
    /// f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const LCD_CLKCNT_N_OFFSET: u32 = 0;
    pub const LCD_CLKCNT_N_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Set this bit to enable clk gate
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

    /// **LCD_CLK_SEL** `[30:29]` (RW)
    ///
    /// Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub fn lcd_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CLK_SEL_MASK) >> Self::LCD_CLK_SEL_OFFSET
    }

    /// Write **LCD_CLK_SEL** `[30:29]` (RW)
    ///
    /// Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub fn write_lcd_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CLK_SEL_MASK)
                | ((value << Self::LCD_CLK_SEL_OFFSET) & Self::LCD_CLK_SEL_MASK)
        });
    }

    /// **LCD_CLKM_DIV_A** `[28:23]` (RW)
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub fn lcd_clkm_div_a(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CLKM_DIV_A_MASK) >> Self::LCD_CLKM_DIV_A_OFFSET
    }

    /// Write **LCD_CLKM_DIV_A** `[28:23]` (RW)
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub fn write_lcd_clkm_div_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CLKM_DIV_A_MASK)
                | ((value << Self::LCD_CLKM_DIV_A_OFFSET) & Self::LCD_CLKM_DIV_A_MASK)
        });
    }

    /// **LCD_CLKM_DIV_B** `[22:17]` (RW)
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub fn lcd_clkm_div_b(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CLKM_DIV_B_MASK) >> Self::LCD_CLKM_DIV_B_OFFSET
    }

    /// Write **LCD_CLKM_DIV_B** `[22:17]` (RW)
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub fn write_lcd_clkm_div_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CLKM_DIV_B_MASK)
                | ((value << Self::LCD_CLKM_DIV_B_OFFSET) & Self::LCD_CLKM_DIV_B_MASK)
        });
    }

    /// **LCD_CLKM_DIV_NUM** `[16:9]` (RW)
    ///
    /// Integral LCD clock divider value
    #[inline(always)]
    pub fn lcd_clkm_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CLKM_DIV_NUM_MASK) >> Self::LCD_CLKM_DIV_NUM_OFFSET
    }

    /// Write **LCD_CLKM_DIV_NUM** `[16:9]` (RW)
    ///
    /// Integral LCD clock divider value
    #[inline(always)]
    pub fn write_lcd_clkm_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CLKM_DIV_NUM_MASK)
                | ((value << Self::LCD_CLKM_DIV_NUM_OFFSET) & Self::LCD_CLKM_DIV_NUM_MASK)
        });
    }

    /// **LCD_CK_OUT_EDGE** `[8]` (RW)
    ///
    /// 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle.
    #[inline(always)]
    pub fn lcd_ck_out_edge(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CK_OUT_EDGE_MASK) != 0
    }

    /// Set LCD_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_ck_out_edge(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CK_OUT_EDGE_MASK);
    }

    /// Clear LCD_CK_OUT_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_ck_out_edge(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CK_OUT_EDGE_MASK);
    }

    /// **LCD_CK_IDLE_EDGE** `[7]` (RW)
    ///
    /// 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle.
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CK_IDLE_EDGE_MASK) != 0
    }

    /// Set LCD_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_ck_idle_edge(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CK_IDLE_EDGE_MASK);
    }

    /// Clear LCD_CK_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_ck_idle_edge(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CK_IDLE_EDGE_MASK);
    }

    /// **LCD_CLK_EQU_SYSCLK** `[6]` (RW)
    ///
    /// 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1).
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Set LCD_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CLK_EQU_SYSCLK_MASK);
    }

    /// Clear LCD_CLK_EQU_SYSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_clk_equ_sysclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CLK_EQU_SYSCLK_MASK);
    }

    /// **LCD_CLKCNT_N** `[5:0]` (RW)
    ///
    /// f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
    #[inline(always)]
    pub fn lcd_clkcnt_n(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CLKCNT_N_MASK) >> Self::LCD_CLKCNT_N_OFFSET
    }

    /// Write **LCD_CLKCNT_N** `[5:0]` (RW)
    ///
    /// f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
    #[inline(always)]
    pub fn write_lcd_clkcnt_n(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CLKCNT_N_MASK)
                | ((value << Self::LCD_CLKCNT_N_OFFSET) & Self::LCD_CLKCNT_N_MASK)
        });
    }
}

/// LCD_CLOCK value type for batch read/modify/write operations
///
/// Use `lcdclock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdclock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdClockVal {
    bits: u32,
}

impl LcdClockVal {
    /// Create from raw bits
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
        Self { bits: 0x00000843 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Set this bit to enable clk gate
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & LcdClock::CLK_EN_MASK) != 0
    }

    /// Read **LCD_CLK_SEL** `[30:29]`
    ///
    /// Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub const fn lcd_clk_sel(&self) -> u32 {
        (self.bits & LcdClock::LCD_CLK_SEL_MASK) >> LcdClock::LCD_CLK_SEL_OFFSET
    }

    /// Read **LCD_CLKM_DIV_A** `[28:23]`
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub const fn lcd_clkm_div_a(&self) -> u32 {
        (self.bits & LcdClock::LCD_CLKM_DIV_A_MASK) >> LcdClock::LCD_CLKM_DIV_A_OFFSET
    }

    /// Read **LCD_CLKM_DIV_B** `[22:17]`
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub const fn lcd_clkm_div_b(&self) -> u32 {
        (self.bits & LcdClock::LCD_CLKM_DIV_B_MASK) >> LcdClock::LCD_CLKM_DIV_B_OFFSET
    }

    /// Read **LCD_CLKM_DIV_NUM** `[16:9]`
    ///
    /// Integral LCD clock divider value
    #[inline(always)]
    pub const fn lcd_clkm_div_num(&self) -> u32 {
        (self.bits & LcdClock::LCD_CLKM_DIV_NUM_MASK) >> LcdClock::LCD_CLKM_DIV_NUM_OFFSET
    }

    /// Read **LCD_CK_OUT_EDGE** `[8:8]`
    ///
    /// 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle.
    #[inline(always)]
    pub const fn lcd_ck_out_edge(&self) -> bool {
        (self.bits & LcdClock::LCD_CK_OUT_EDGE_MASK) != 0
    }

    /// Read **LCD_CK_IDLE_EDGE** `[7:7]`
    ///
    /// 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle.
    #[inline(always)]
    pub const fn lcd_ck_idle_edge(&self) -> bool {
        (self.bits & LcdClock::LCD_CK_IDLE_EDGE_MASK) != 0
    }

    /// Read **LCD_CLK_EQU_SYSCLK** `[6:6]`
    ///
    /// 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1).
    #[inline(always)]
    pub const fn lcd_clk_equ_sysclk(&self) -> bool {
        (self.bits & LcdClock::LCD_CLK_EQU_SYSCLK_MASK) != 0
    }

    /// Read **LCD_CLKCNT_N** `[5:0]`
    ///
    /// f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
    #[inline(always)]
    pub const fn lcd_clkcnt_n(&self) -> u32 {
        (self.bits & LcdClock::LCD_CLKCNT_N_MASK) >> LcdClock::LCD_CLKCNT_N_OFFSET
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Set this bit to enable clk gate
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdClock::CLK_EN_MASK
            } else {
                self.bits & !LcdClock::CLK_EN_MASK
            },
        }
    }

    /// Set **LCD_CLK_SEL** `[30:29]`
    ///
    /// Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub const fn with_lcd_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdClock::LCD_CLK_SEL_MASK)
                | ((value << LcdClock::LCD_CLK_SEL_OFFSET) & LcdClock::LCD_CLK_SEL_MASK),
        }
    }

    /// Set **LCD_CLKM_DIV_A** `[28:23]`
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub const fn with_lcd_clkm_div_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdClock::LCD_CLKM_DIV_A_MASK)
                | ((value << LcdClock::LCD_CLKM_DIV_A_OFFSET) & LcdClock::LCD_CLKM_DIV_A_MASK),
        }
    }

    /// Set **LCD_CLKM_DIV_B** `[22:17]`
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub const fn with_lcd_clkm_div_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdClock::LCD_CLKM_DIV_B_MASK)
                | ((value << LcdClock::LCD_CLKM_DIV_B_OFFSET) & LcdClock::LCD_CLKM_DIV_B_MASK),
        }
    }

    /// Set **LCD_CLKM_DIV_NUM** `[16:9]`
    ///
    /// Integral LCD clock divider value
    #[inline(always)]
    pub const fn with_lcd_clkm_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdClock::LCD_CLKM_DIV_NUM_MASK)
                | ((value << LcdClock::LCD_CLKM_DIV_NUM_OFFSET) & LcdClock::LCD_CLKM_DIV_NUM_MASK),
        }
    }

    /// Set **LCD_CK_OUT_EDGE** `[8:8]`
    ///
    /// 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle.
    #[inline(always)]
    pub const fn with_lcd_ck_out_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdClock::LCD_CK_OUT_EDGE_MASK
            } else {
                self.bits & !LcdClock::LCD_CK_OUT_EDGE_MASK
            },
        }
    }

    /// Set **LCD_CK_IDLE_EDGE** `[7:7]`
    ///
    /// 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle.
    #[inline(always)]
    pub const fn with_lcd_ck_idle_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdClock::LCD_CK_IDLE_EDGE_MASK
            } else {
                self.bits & !LcdClock::LCD_CK_IDLE_EDGE_MASK
            },
        }
    }

    /// Set **LCD_CLK_EQU_SYSCLK** `[6:6]`
    ///
    /// 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1).
    #[inline(always)]
    pub const fn with_lcd_clk_equ_sysclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdClock::LCD_CLK_EQU_SYSCLK_MASK
            } else {
                self.bits & !LcdClock::LCD_CLK_EQU_SYSCLK_MASK
            },
        }
    }

    /// Set **LCD_CLKCNT_N** `[5:0]`
    ///
    /// f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0.
    #[inline(always)]
    pub const fn with_lcd_clkcnt_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdClock::LCD_CLKCNT_N_MASK)
                | ((value << LcdClock::LCD_CLKCNT_N_OFFSET) & LcdClock::LCD_CLKCNT_N_MASK),
        }
    }
}

impl Default for LcdClockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcdClock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdClockVal {
        LcdClockVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdClockVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdClockVal) -> LcdClockVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAM_CTRL Register
///
/// CAM config register.
///
/// Address offset: 0x0004
/// Reset value: 0x00000800
///
/// ## Fields
///
/// - **CAM_CLK_SEL** `[30:29]` (RW)
///
///   Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
///
/// - **CAM_CLKM_DIV_A** `[28:23]` (RW)
///
///   Fractional clock divider denominator value
///
/// - **CAM_CLKM_DIV_B** `[22:17]` (RW)
///
///   Fractional clock divider numerator value
///
/// - **CAM_CLKM_DIV_NUM** `[16:9]` (RW)
///
///   Integral Camera clock divider value
///
/// - **CAM_VS_EOF_EN** `[8]` (RW)
///
///   1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen.
///
/// - **CAM_LINE_INT_EN** `[7]` (RW)
///
///   1: Enable to generate CAM_HS_INT. 0: Disable.
///
/// - **CAM_BIT_ORDER** `[6]` (RW)
///
///   1: invert data byte order, only valid in 2 byte mode. 0: Not change.
///
/// - **CAM_BYTE_ORDER** `[5]` (RW)
///
///   1: Change data bit order, change CAM_DATA_in[7:0] to CAM_DATA_in[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
///
/// - **CAM_UPDATE** `[4]` (RW)
///
///   1: Update Camera registers, will be cleared by hardware. 0 : Not care.
///
/// - **CAM_VSYNC_FILTER_THRES** `[3:1]` (RW)
///
///   Filter threshold value for CAM_VSYNC signal.
///
/// - **CAM_STOP_EN** `[0]` (RW)
///
///   Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop.
#[repr(transparent)]
pub struct CamCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CamCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CamCtrl {
    // ======== Field Constants ========

    /// **CAM_CLK_SEL**
    ///
    /// Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const CAM_CLK_SEL_OFFSET: u32 = 29;
    pub const CAM_CLK_SEL_MASK: u32 = 0x60000000;

    /// **CAM_CLKM_DIV_A**
    ///
    /// Fractional clock divider denominator value
    ///
    /// Bits `[28:23]`, mask `0x1f800000`
    pub const CAM_CLKM_DIV_A_OFFSET: u32 = 23;
    pub const CAM_CLKM_DIV_A_MASK: u32 = 0x1f800000;

    /// **CAM_CLKM_DIV_B**
    ///
    /// Fractional clock divider numerator value
    ///
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const CAM_CLKM_DIV_B_OFFSET: u32 = 17;
    pub const CAM_CLKM_DIV_B_MASK: u32 = 0x007e0000;

    /// **CAM_CLKM_DIV_NUM**
    ///
    /// Integral Camera clock divider value
    ///
    /// Bits `[16:9]`, mask `0x0001fe00`
    pub const CAM_CLKM_DIV_NUM_OFFSET: u32 = 9;
    pub const CAM_CLKM_DIV_NUM_MASK: u32 = 0x0001fe00;

    /// **CAM_VS_EOF_EN**
    ///
    /// 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CAM_VS_EOF_EN_OFFSET: u32 = 8;
    pub const CAM_VS_EOF_EN_MASK: u32 = 0x00000100;
    pub const CAM_VS_EOF_EN: u32 = 1 << 8;

    /// **CAM_LINE_INT_EN**
    ///
    /// 1: Enable to generate CAM_HS_INT. 0: Disable.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CAM_LINE_INT_EN_OFFSET: u32 = 7;
    pub const CAM_LINE_INT_EN_MASK: u32 = 0x00000080;
    pub const CAM_LINE_INT_EN: u32 = 1 << 7;

    /// **CAM_BIT_ORDER**
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CAM_BIT_ORDER_OFFSET: u32 = 6;
    pub const CAM_BIT_ORDER_MASK: u32 = 0x00000040;
    pub const CAM_BIT_ORDER: u32 = 1 << 6;

    /// **CAM_BYTE_ORDER**
    ///
    /// 1: Change data bit order, change CAM_DATA_in[7:0] to CAM_DATA_in[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CAM_BYTE_ORDER_OFFSET: u32 = 5;
    pub const CAM_BYTE_ORDER_MASK: u32 = 0x00000020;
    pub const CAM_BYTE_ORDER: u32 = 1 << 5;

    /// **CAM_UPDATE**
    ///
    /// 1: Update Camera registers, will be cleared by hardware. 0 : Not care.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CAM_UPDATE_OFFSET: u32 = 4;
    pub const CAM_UPDATE_MASK: u32 = 0x00000010;
    pub const CAM_UPDATE: u32 = 1 << 4;

    /// **CAM_VSYNC_FILTER_THRES**
    ///
    /// Filter threshold value for CAM_VSYNC signal.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const CAM_VSYNC_FILTER_THRES_OFFSET: u32 = 1;
    pub const CAM_VSYNC_FILTER_THRES_MASK: u32 = 0x0000000e;

    /// **CAM_STOP_EN**
    ///
    /// Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CAM_STOP_EN_OFFSET: u32 = 0;
    pub const CAM_STOP_EN_MASK: u32 = 0x00000001;
    pub const CAM_STOP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CAM_CLK_SEL** `[30:29]` (RW)
    ///
    /// Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub fn cam_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CLK_SEL_MASK) >> Self::CAM_CLK_SEL_OFFSET
    }

    /// Write **CAM_CLK_SEL** `[30:29]` (RW)
    ///
    /// Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub fn write_cam_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CLK_SEL_MASK)
                | ((value << Self::CAM_CLK_SEL_OFFSET) & Self::CAM_CLK_SEL_MASK)
        });
    }

    /// **CAM_CLKM_DIV_A** `[28:23]` (RW)
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub fn cam_clkm_div_a(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CLKM_DIV_A_MASK) >> Self::CAM_CLKM_DIV_A_OFFSET
    }

    /// Write **CAM_CLKM_DIV_A** `[28:23]` (RW)
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub fn write_cam_clkm_div_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CLKM_DIV_A_MASK)
                | ((value << Self::CAM_CLKM_DIV_A_OFFSET) & Self::CAM_CLKM_DIV_A_MASK)
        });
    }

    /// **CAM_CLKM_DIV_B** `[22:17]` (RW)
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub fn cam_clkm_div_b(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CLKM_DIV_B_MASK) >> Self::CAM_CLKM_DIV_B_OFFSET
    }

    /// Write **CAM_CLKM_DIV_B** `[22:17]` (RW)
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub fn write_cam_clkm_div_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CLKM_DIV_B_MASK)
                | ((value << Self::CAM_CLKM_DIV_B_OFFSET) & Self::CAM_CLKM_DIV_B_MASK)
        });
    }

    /// **CAM_CLKM_DIV_NUM** `[16:9]` (RW)
    ///
    /// Integral Camera clock divider value
    #[inline(always)]
    pub fn cam_clkm_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CLKM_DIV_NUM_MASK) >> Self::CAM_CLKM_DIV_NUM_OFFSET
    }

    /// Write **CAM_CLKM_DIV_NUM** `[16:9]` (RW)
    ///
    /// Integral Camera clock divider value
    #[inline(always)]
    pub fn write_cam_clkm_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CLKM_DIV_NUM_MASK)
                | ((value << Self::CAM_CLKM_DIV_NUM_OFFSET) & Self::CAM_CLKM_DIV_NUM_MASK)
        });
    }

    /// **CAM_VS_EOF_EN** `[8]` (RW)
    ///
    /// 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen.
    #[inline(always)]
    pub fn cam_vs_eof_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VS_EOF_EN_MASK) != 0
    }

    /// Set CAM_VS_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_vs_eof_en(&self) {
        self.value.modify_bits(|r| r | Self::CAM_VS_EOF_EN_MASK);
    }

    /// Clear CAM_VS_EOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_vs_eof_en(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_VS_EOF_EN_MASK);
    }

    /// **CAM_LINE_INT_EN** `[7]` (RW)
    ///
    /// 1: Enable to generate CAM_HS_INT. 0: Disable.
    #[inline(always)]
    pub fn cam_line_int_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_LINE_INT_EN_MASK) != 0
    }

    /// Set CAM_LINE_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_line_int_en(&self) {
        self.value.modify_bits(|r| r | Self::CAM_LINE_INT_EN_MASK);
    }

    /// Clear CAM_LINE_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_line_int_en(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_LINE_INT_EN_MASK);
    }

    /// **CAM_BIT_ORDER** `[6]` (RW)
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub fn cam_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::CAM_BIT_ORDER_MASK) != 0
    }

    /// Set CAM_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_bit_order(&self) {
        self.value.modify_bits(|r| r | Self::CAM_BIT_ORDER_MASK);
    }

    /// Clear CAM_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_bit_order(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_BIT_ORDER_MASK);
    }

    /// **CAM_BYTE_ORDER** `[5]` (RW)
    ///
    /// 1: Change data bit order, change CAM_DATA_in[7:0] to CAM_DATA_in[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub fn cam_byte_order(&self) -> bool {
        (self.value.read_bits() & Self::CAM_BYTE_ORDER_MASK) != 0
    }

    /// Set CAM_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_byte_order(&self) {
        self.value.modify_bits(|r| r | Self::CAM_BYTE_ORDER_MASK);
    }

    /// Clear CAM_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_byte_order(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_BYTE_ORDER_MASK);
    }

    /// **CAM_UPDATE** `[4]` (RW)
    ///
    /// 1: Update Camera registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub fn cam_update(&self) -> bool {
        (self.value.read_bits() & Self::CAM_UPDATE_MASK) != 0
    }

    /// Set CAM_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_update(&self) {
        self.value.modify_bits(|r| r | Self::CAM_UPDATE_MASK);
    }

    /// Clear CAM_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_update(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_UPDATE_MASK);
    }

    /// **CAM_VSYNC_FILTER_THRES** `[3:1]` (RW)
    ///
    /// Filter threshold value for CAM_VSYNC signal.
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_VSYNC_FILTER_THRES_MASK)
            >> Self::CAM_VSYNC_FILTER_THRES_OFFSET
    }

    /// Write **CAM_VSYNC_FILTER_THRES** `[3:1]` (RW)
    ///
    /// Filter threshold value for CAM_VSYNC signal.
    #[inline(always)]
    pub fn write_cam_vsync_filter_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_VSYNC_FILTER_THRES_MASK)
                | ((value << Self::CAM_VSYNC_FILTER_THRES_OFFSET)
                    & Self::CAM_VSYNC_FILTER_THRES_MASK)
        });
    }

    /// **CAM_STOP_EN** `[0]` (RW)
    ///
    /// Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop.
    #[inline(always)]
    pub fn cam_stop_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_STOP_EN_MASK) != 0
    }

    /// Set CAM_STOP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_stop_en(&self) {
        self.value.modify_bits(|r| r | Self::CAM_STOP_EN_MASK);
    }

    /// Clear CAM_STOP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_stop_en(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_STOP_EN_MASK);
    }
}

/// CAM_CTRL value type for batch read/modify/write operations
///
/// Use `camctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `camctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CamCtrlVal {
    bits: u32,
}

impl CamCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000800 }
    }

    /// Read **CAM_CLK_SEL** `[30:29]`
    ///
    /// Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub const fn cam_clk_sel(&self) -> u32 {
        (self.bits & CamCtrl::CAM_CLK_SEL_MASK) >> CamCtrl::CAM_CLK_SEL_OFFSET
    }

    /// Read **CAM_CLKM_DIV_A** `[28:23]`
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub const fn cam_clkm_div_a(&self) -> u32 {
        (self.bits & CamCtrl::CAM_CLKM_DIV_A_MASK) >> CamCtrl::CAM_CLKM_DIV_A_OFFSET
    }

    /// Read **CAM_CLKM_DIV_B** `[22:17]`
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub const fn cam_clkm_div_b(&self) -> u32 {
        (self.bits & CamCtrl::CAM_CLKM_DIV_B_MASK) >> CamCtrl::CAM_CLKM_DIV_B_OFFSET
    }

    /// Read **CAM_CLKM_DIV_NUM** `[16:9]`
    ///
    /// Integral Camera clock divider value
    #[inline(always)]
    pub const fn cam_clkm_div_num(&self) -> u32 {
        (self.bits & CamCtrl::CAM_CLKM_DIV_NUM_MASK) >> CamCtrl::CAM_CLKM_DIV_NUM_OFFSET
    }

    /// Read **CAM_VS_EOF_EN** `[8:8]`
    ///
    /// 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen.
    #[inline(always)]
    pub const fn cam_vs_eof_en(&self) -> bool {
        (self.bits & CamCtrl::CAM_VS_EOF_EN_MASK) != 0
    }

    /// Read **CAM_LINE_INT_EN** `[7:7]`
    ///
    /// 1: Enable to generate CAM_HS_INT. 0: Disable.
    #[inline(always)]
    pub const fn cam_line_int_en(&self) -> bool {
        (self.bits & CamCtrl::CAM_LINE_INT_EN_MASK) != 0
    }

    /// Read **CAM_BIT_ORDER** `[6:6]`
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub const fn cam_bit_order(&self) -> bool {
        (self.bits & CamCtrl::CAM_BIT_ORDER_MASK) != 0
    }

    /// Read **CAM_BYTE_ORDER** `[5:5]`
    ///
    /// 1: Change data bit order, change CAM_DATA_in[7:0] to CAM_DATA_in[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub const fn cam_byte_order(&self) -> bool {
        (self.bits & CamCtrl::CAM_BYTE_ORDER_MASK) != 0
    }

    /// Read **CAM_UPDATE** `[4:4]`
    ///
    /// 1: Update Camera registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub const fn cam_update(&self) -> bool {
        (self.bits & CamCtrl::CAM_UPDATE_MASK) != 0
    }

    /// Read **CAM_VSYNC_FILTER_THRES** `[3:1]`
    ///
    /// Filter threshold value for CAM_VSYNC signal.
    #[inline(always)]
    pub const fn cam_vsync_filter_thres(&self) -> u32 {
        (self.bits & CamCtrl::CAM_VSYNC_FILTER_THRES_MASK) >> CamCtrl::CAM_VSYNC_FILTER_THRES_OFFSET
    }

    /// Read **CAM_STOP_EN** `[0:0]`
    ///
    /// Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop.
    #[inline(always)]
    pub const fn cam_stop_en(&self) -> bool {
        (self.bits & CamCtrl::CAM_STOP_EN_MASK) != 0
    }

    /// Set **CAM_CLK_SEL** `[30:29]`
    ///
    /// Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock.
    #[inline(always)]
    pub const fn with_cam_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl::CAM_CLK_SEL_MASK)
                | ((value << CamCtrl::CAM_CLK_SEL_OFFSET) & CamCtrl::CAM_CLK_SEL_MASK),
        }
    }

    /// Set **CAM_CLKM_DIV_A** `[28:23]`
    ///
    /// Fractional clock divider denominator value
    #[inline(always)]
    pub const fn with_cam_clkm_div_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl::CAM_CLKM_DIV_A_MASK)
                | ((value << CamCtrl::CAM_CLKM_DIV_A_OFFSET) & CamCtrl::CAM_CLKM_DIV_A_MASK),
        }
    }

    /// Set **CAM_CLKM_DIV_B** `[22:17]`
    ///
    /// Fractional clock divider numerator value
    #[inline(always)]
    pub const fn with_cam_clkm_div_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl::CAM_CLKM_DIV_B_MASK)
                | ((value << CamCtrl::CAM_CLKM_DIV_B_OFFSET) & CamCtrl::CAM_CLKM_DIV_B_MASK),
        }
    }

    /// Set **CAM_CLKM_DIV_NUM** `[16:9]`
    ///
    /// Integral Camera clock divider value
    #[inline(always)]
    pub const fn with_cam_clkm_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl::CAM_CLKM_DIV_NUM_MASK)
                | ((value << CamCtrl::CAM_CLKM_DIV_NUM_OFFSET) & CamCtrl::CAM_CLKM_DIV_NUM_MASK),
        }
    }

    /// Set **CAM_VS_EOF_EN** `[8:8]`
    ///
    /// 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen.
    #[inline(always)]
    pub const fn with_cam_vs_eof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_VS_EOF_EN_MASK
            } else {
                self.bits & !CamCtrl::CAM_VS_EOF_EN_MASK
            },
        }
    }

    /// Set **CAM_LINE_INT_EN** `[7:7]`
    ///
    /// 1: Enable to generate CAM_HS_INT. 0: Disable.
    #[inline(always)]
    pub const fn with_cam_line_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_LINE_INT_EN_MASK
            } else {
                self.bits & !CamCtrl::CAM_LINE_INT_EN_MASK
            },
        }
    }

    /// Set **CAM_BIT_ORDER** `[6:6]`
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub const fn with_cam_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_BIT_ORDER_MASK
            } else {
                self.bits & !CamCtrl::CAM_BIT_ORDER_MASK
            },
        }
    }

    /// Set **CAM_BYTE_ORDER** `[5:5]`
    ///
    /// 1: Change data bit order, change CAM_DATA_in[7:0] to CAM_DATA_in[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub const fn with_cam_byte_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_BYTE_ORDER_MASK
            } else {
                self.bits & !CamCtrl::CAM_BYTE_ORDER_MASK
            },
        }
    }

    /// Set **CAM_UPDATE** `[4:4]`
    ///
    /// 1: Update Camera registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub const fn with_cam_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_UPDATE_MASK
            } else {
                self.bits & !CamCtrl::CAM_UPDATE_MASK
            },
        }
    }

    /// Set **CAM_VSYNC_FILTER_THRES** `[3:1]`
    ///
    /// Filter threshold value for CAM_VSYNC signal.
    #[inline(always)]
    pub const fn with_cam_vsync_filter_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl::CAM_VSYNC_FILTER_THRES_MASK)
                | ((value << CamCtrl::CAM_VSYNC_FILTER_THRES_OFFSET)
                    & CamCtrl::CAM_VSYNC_FILTER_THRES_MASK),
        }
    }

    /// Set **CAM_STOP_EN** `[0:0]`
    ///
    /// Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop.
    #[inline(always)]
    pub const fn with_cam_stop_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl::CAM_STOP_EN_MASK
            } else {
                self.bits & !CamCtrl::CAM_STOP_EN_MASK
            },
        }
    }
}

impl Default for CamCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CamCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CamCtrlVal {
        CamCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CamCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CamCtrlVal) -> CamCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAM_CTRL1 Register
///
/// CAM config register.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CAM_AFIFO_RESET** `[31]` (WO)
///
///   Camera AFIFO reset signal.
///
/// - **CAM_RESET** `[30]` (WO)
///
///   Camera module reset signal.
///
/// - **CAM_START** `[29]` (RW)
///
///   Camera module start signal.
///
/// - **CAM_VH_DE_MODE_EN** `[28]` (RW)
///
///   1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC. 0: Input control signals are CAM_DE and CAM_VSYNC.
///
/// - **CAM_VSYNC_INV** `[27]` (RW)
///
///   CAM_VSYNC invert enable signal, valid in high level.
///
/// - **CAM_HSYNC_INV** `[26]` (RW)
///
///   CAM_HSYNC invert enable signal, valid in high level.
///
/// - **CAM_DE_INV** `[25]` (RW)
///
///   CAM_DE invert enable signal, valid in high level.
///
/// - **CAM_2BYTE_EN** `[24]` (RW)
///
///   1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8.
///
/// - **CAM_VSYNC_FILTER_EN** `[23]` (RW)
///
///   1: Enable CAM_VSYNC filter function. 0: bypass.
///
/// - **CAM_CLK_INV** `[22]` (RW)
///
///   1: Invert the input signal CAM_PCLK. 0: Not invert.
///
/// - **CAM_LINE_INT_NUM** `[21:16]` (RW)
///
///   The line number minus 1 to generate cam_hs_int.
///
/// - **CAM_REC_DATA_BYTELEN** `[15:0]` (RW)
///
///   Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
#[repr(transparent)]
pub struct CamCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CamCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CamCtrl1 {
    // ======== Field Constants ========

    /// **CAM_AFIFO_RESET**
    ///
    /// Camera AFIFO reset signal.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CAM_AFIFO_RESET_OFFSET: u32 = 31;
    pub const CAM_AFIFO_RESET_MASK: u32 = 0x80000000;
    pub const CAM_AFIFO_RESET: u32 = 1 << 31;

    /// **CAM_RESET**
    ///
    /// Camera module reset signal.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CAM_RESET_OFFSET: u32 = 30;
    pub const CAM_RESET_MASK: u32 = 0x40000000;
    pub const CAM_RESET: u32 = 1 << 30;

    /// **CAM_START**
    ///
    /// Camera module start signal.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CAM_START_OFFSET: u32 = 29;
    pub const CAM_START_MASK: u32 = 0x20000000;
    pub const CAM_START: u32 = 1 << 29;

    /// **CAM_VH_DE_MODE_EN**
    ///
    /// 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC. 0: Input control signals are CAM_DE and CAM_VSYNC.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CAM_VH_DE_MODE_EN_OFFSET: u32 = 28;
    pub const CAM_VH_DE_MODE_EN_MASK: u32 = 0x10000000;
    pub const CAM_VH_DE_MODE_EN: u32 = 1 << 28;

    /// **CAM_VSYNC_INV**
    ///
    /// CAM_VSYNC invert enable signal, valid in high level.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CAM_VSYNC_INV_OFFSET: u32 = 27;
    pub const CAM_VSYNC_INV_MASK: u32 = 0x08000000;
    pub const CAM_VSYNC_INV: u32 = 1 << 27;

    /// **CAM_HSYNC_INV**
    ///
    /// CAM_HSYNC invert enable signal, valid in high level.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CAM_HSYNC_INV_OFFSET: u32 = 26;
    pub const CAM_HSYNC_INV_MASK: u32 = 0x04000000;
    pub const CAM_HSYNC_INV: u32 = 1 << 26;

    /// **CAM_DE_INV**
    ///
    /// CAM_DE invert enable signal, valid in high level.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CAM_DE_INV_OFFSET: u32 = 25;
    pub const CAM_DE_INV_MASK: u32 = 0x02000000;
    pub const CAM_DE_INV: u32 = 1 << 25;

    /// **CAM_2BYTE_EN**
    ///
    /// 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CAM_2BYTE_EN_OFFSET: u32 = 24;
    pub const CAM_2BYTE_EN_MASK: u32 = 0x01000000;
    pub const CAM_2BYTE_EN: u32 = 1 << 24;

    /// **CAM_VSYNC_FILTER_EN**
    ///
    /// 1: Enable CAM_VSYNC filter function. 0: bypass.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CAM_VSYNC_FILTER_EN_OFFSET: u32 = 23;
    pub const CAM_VSYNC_FILTER_EN_MASK: u32 = 0x00800000;
    pub const CAM_VSYNC_FILTER_EN: u32 = 1 << 23;

    /// **CAM_CLK_INV**
    ///
    /// 1: Invert the input signal CAM_PCLK. 0: Not invert.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CAM_CLK_INV_OFFSET: u32 = 22;
    pub const CAM_CLK_INV_MASK: u32 = 0x00400000;
    pub const CAM_CLK_INV: u32 = 1 << 22;

    /// **CAM_LINE_INT_NUM**
    ///
    /// The line number minus 1 to generate cam_hs_int.
    ///
    /// Bits `[21:16]`, mask `0x003f0000`
    pub const CAM_LINE_INT_NUM_OFFSET: u32 = 16;
    pub const CAM_LINE_INT_NUM_MASK: u32 = 0x003f0000;

    /// **CAM_REC_DATA_BYTELEN**
    ///
    /// Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CAM_REC_DATA_BYTELEN_OFFSET: u32 = 0;
    pub const CAM_REC_DATA_BYTELEN_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **CAM_AFIFO_RESET** (WO)
    ///
    /// Camera AFIFO reset signal.
    #[inline(always)]
    pub fn trigger_cam_afifo_reset(&self) {
        self.value.modify_bits(|r| r | Self::CAM_AFIFO_RESET_MASK);
    }

    /// Trigger **CAM_RESET** (WO)
    ///
    /// Camera module reset signal.
    #[inline(always)]
    pub fn trigger_cam_reset(&self) {
        self.value.modify_bits(|r| r | Self::CAM_RESET_MASK);
    }

    /// **CAM_START** `[29]` (RW)
    ///
    /// Camera module start signal.
    #[inline(always)]
    pub fn cam_start(&self) -> bool {
        (self.value.read_bits() & Self::CAM_START_MASK) != 0
    }

    /// Set CAM_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_start(&self) {
        self.value.modify_bits(|r| r | Self::CAM_START_MASK);
    }

    /// Clear CAM_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_start(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_START_MASK);
    }

    /// **CAM_VH_DE_MODE_EN** `[28]` (RW)
    ///
    /// 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC. 0: Input control signals are CAM_DE and CAM_VSYNC.
    #[inline(always)]
    pub fn cam_vh_de_mode_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VH_DE_MODE_EN_MASK) != 0
    }

    /// Set CAM_VH_DE_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_vh_de_mode_en(&self) {
        self.value.modify_bits(|r| r | Self::CAM_VH_DE_MODE_EN_MASK);
    }

    /// Clear CAM_VH_DE_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_vh_de_mode_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_VH_DE_MODE_EN_MASK);
    }

    /// **CAM_VSYNC_INV** `[27]` (RW)
    ///
    /// CAM_VSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub fn cam_vsync_inv(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VSYNC_INV_MASK) != 0
    }

    /// Set CAM_VSYNC_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_vsync_inv(&self) {
        self.value.modify_bits(|r| r | Self::CAM_VSYNC_INV_MASK);
    }

    /// Clear CAM_VSYNC_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_vsync_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_VSYNC_INV_MASK);
    }

    /// **CAM_HSYNC_INV** `[26]` (RW)
    ///
    /// CAM_HSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub fn cam_hsync_inv(&self) -> bool {
        (self.value.read_bits() & Self::CAM_HSYNC_INV_MASK) != 0
    }

    /// Set CAM_HSYNC_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_hsync_inv(&self) {
        self.value.modify_bits(|r| r | Self::CAM_HSYNC_INV_MASK);
    }

    /// Clear CAM_HSYNC_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_hsync_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_HSYNC_INV_MASK);
    }

    /// **CAM_DE_INV** `[25]` (RW)
    ///
    /// CAM_DE invert enable signal, valid in high level.
    #[inline(always)]
    pub fn cam_de_inv(&self) -> bool {
        (self.value.read_bits() & Self::CAM_DE_INV_MASK) != 0
    }

    /// Set CAM_DE_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_de_inv(&self) {
        self.value.modify_bits(|r| r | Self::CAM_DE_INV_MASK);
    }

    /// Clear CAM_DE_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_de_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_DE_INV_MASK);
    }

    /// **CAM_2BYTE_EN** `[24]` (RW)
    ///
    /// 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8.
    #[inline(always)]
    pub fn cam_2byte_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_2BYTE_EN_MASK) != 0
    }

    /// Set CAM_2BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_2byte_en(&self) {
        self.value.modify_bits(|r| r | Self::CAM_2BYTE_EN_MASK);
    }

    /// Clear CAM_2BYTE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_2byte_en(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_2BYTE_EN_MASK);
    }

    /// **CAM_VSYNC_FILTER_EN** `[23]` (RW)
    ///
    /// 1: Enable CAM_VSYNC filter function. 0: bypass.
    #[inline(always)]
    pub fn cam_vsync_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VSYNC_FILTER_EN_MASK) != 0
    }

    /// Set CAM_VSYNC_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_vsync_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_VSYNC_FILTER_EN_MASK);
    }

    /// Clear CAM_VSYNC_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_vsync_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_VSYNC_FILTER_EN_MASK);
    }

    /// **CAM_CLK_INV** `[22]` (RW)
    ///
    /// 1: Invert the input signal CAM_PCLK. 0: Not invert.
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CLK_INV_MASK) != 0
    }

    /// Set CAM_CLK_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_clk_inv(&self) {
        self.value.modify_bits(|r| r | Self::CAM_CLK_INV_MASK);
    }

    /// Clear CAM_CLK_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_clk_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_CLK_INV_MASK);
    }

    /// **CAM_LINE_INT_NUM** `[21:16]` (RW)
    ///
    /// The line number minus 1 to generate cam_hs_int.
    #[inline(always)]
    pub fn cam_line_int_num(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_LINE_INT_NUM_MASK) >> Self::CAM_LINE_INT_NUM_OFFSET
    }

    /// Write **CAM_LINE_INT_NUM** `[21:16]` (RW)
    ///
    /// The line number minus 1 to generate cam_hs_int.
    #[inline(always)]
    pub fn write_cam_line_int_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_LINE_INT_NUM_MASK)
                | ((value << Self::CAM_LINE_INT_NUM_OFFSET) & Self::CAM_LINE_INT_NUM_MASK)
        });
    }

    /// **CAM_REC_DATA_BYTELEN** `[15:0]` (RW)
    ///
    /// Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
    #[inline(always)]
    pub fn cam_rec_data_bytelen(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_REC_DATA_BYTELEN_MASK)
            >> Self::CAM_REC_DATA_BYTELEN_OFFSET
    }

    /// Write **CAM_REC_DATA_BYTELEN** `[15:0]` (RW)
    ///
    /// Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
    #[inline(always)]
    pub fn write_cam_rec_data_bytelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_REC_DATA_BYTELEN_MASK)
                | ((value << Self::CAM_REC_DATA_BYTELEN_OFFSET) & Self::CAM_REC_DATA_BYTELEN_MASK)
        });
    }
}

/// CAM_CTRL1 value type for batch read/modify/write operations
///
/// Use `camctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `camctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CamCtrl1Val {
    bits: u32,
}

impl CamCtrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CAM_START** `[29:29]`
    ///
    /// Camera module start signal.
    #[inline(always)]
    pub const fn cam_start(&self) -> bool {
        (self.bits & CamCtrl1::CAM_START_MASK) != 0
    }

    /// Read **CAM_VH_DE_MODE_EN** `[28:28]`
    ///
    /// 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC. 0: Input control signals are CAM_DE and CAM_VSYNC.
    #[inline(always)]
    pub const fn cam_vh_de_mode_en(&self) -> bool {
        (self.bits & CamCtrl1::CAM_VH_DE_MODE_EN_MASK) != 0
    }

    /// Read **CAM_VSYNC_INV** `[27:27]`
    ///
    /// CAM_VSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn cam_vsync_inv(&self) -> bool {
        (self.bits & CamCtrl1::CAM_VSYNC_INV_MASK) != 0
    }

    /// Read **CAM_HSYNC_INV** `[26:26]`
    ///
    /// CAM_HSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn cam_hsync_inv(&self) -> bool {
        (self.bits & CamCtrl1::CAM_HSYNC_INV_MASK) != 0
    }

    /// Read **CAM_DE_INV** `[25:25]`
    ///
    /// CAM_DE invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn cam_de_inv(&self) -> bool {
        (self.bits & CamCtrl1::CAM_DE_INV_MASK) != 0
    }

    /// Read **CAM_2BYTE_EN** `[24:24]`
    ///
    /// 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8.
    #[inline(always)]
    pub const fn cam_2byte_en(&self) -> bool {
        (self.bits & CamCtrl1::CAM_2BYTE_EN_MASK) != 0
    }

    /// Read **CAM_VSYNC_FILTER_EN** `[23:23]`
    ///
    /// 1: Enable CAM_VSYNC filter function. 0: bypass.
    #[inline(always)]
    pub const fn cam_vsync_filter_en(&self) -> bool {
        (self.bits & CamCtrl1::CAM_VSYNC_FILTER_EN_MASK) != 0
    }

    /// Read **CAM_CLK_INV** `[22:22]`
    ///
    /// 1: Invert the input signal CAM_PCLK. 0: Not invert.
    #[inline(always)]
    pub const fn cam_clk_inv(&self) -> bool {
        (self.bits & CamCtrl1::CAM_CLK_INV_MASK) != 0
    }

    /// Read **CAM_LINE_INT_NUM** `[21:16]`
    ///
    /// The line number minus 1 to generate cam_hs_int.
    #[inline(always)]
    pub const fn cam_line_int_num(&self) -> u32 {
        (self.bits & CamCtrl1::CAM_LINE_INT_NUM_MASK) >> CamCtrl1::CAM_LINE_INT_NUM_OFFSET
    }

    /// Read **CAM_REC_DATA_BYTELEN** `[15:0]`
    ///
    /// Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
    #[inline(always)]
    pub const fn cam_rec_data_bytelen(&self) -> u32 {
        (self.bits & CamCtrl1::CAM_REC_DATA_BYTELEN_MASK) >> CamCtrl1::CAM_REC_DATA_BYTELEN_OFFSET
    }

    /// Set **CAM_AFIFO_RESET** `[31:31]`
    ///
    /// Camera AFIFO reset signal.
    #[inline(always)]
    pub const fn with_cam_afifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_AFIFO_RESET_MASK
            } else {
                self.bits & !CamCtrl1::CAM_AFIFO_RESET_MASK
            },
        }
    }

    /// Set **CAM_RESET** `[30:30]`
    ///
    /// Camera module reset signal.
    #[inline(always)]
    pub const fn with_cam_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_RESET_MASK
            } else {
                self.bits & !CamCtrl1::CAM_RESET_MASK
            },
        }
    }

    /// Set **CAM_START** `[29:29]`
    ///
    /// Camera module start signal.
    #[inline(always)]
    pub const fn with_cam_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_START_MASK
            } else {
                self.bits & !CamCtrl1::CAM_START_MASK
            },
        }
    }

    /// Set **CAM_VH_DE_MODE_EN** `[28:28]`
    ///
    /// 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC. 0: Input control signals are CAM_DE and CAM_VSYNC.
    #[inline(always)]
    pub const fn with_cam_vh_de_mode_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_VH_DE_MODE_EN_MASK
            } else {
                self.bits & !CamCtrl1::CAM_VH_DE_MODE_EN_MASK
            },
        }
    }

    /// Set **CAM_VSYNC_INV** `[27:27]`
    ///
    /// CAM_VSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn with_cam_vsync_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_VSYNC_INV_MASK
            } else {
                self.bits & !CamCtrl1::CAM_VSYNC_INV_MASK
            },
        }
    }

    /// Set **CAM_HSYNC_INV** `[26:26]`
    ///
    /// CAM_HSYNC invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn with_cam_hsync_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_HSYNC_INV_MASK
            } else {
                self.bits & !CamCtrl1::CAM_HSYNC_INV_MASK
            },
        }
    }

    /// Set **CAM_DE_INV** `[25:25]`
    ///
    /// CAM_DE invert enable signal, valid in high level.
    #[inline(always)]
    pub const fn with_cam_de_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_DE_INV_MASK
            } else {
                self.bits & !CamCtrl1::CAM_DE_INV_MASK
            },
        }
    }

    /// Set **CAM_2BYTE_EN** `[24:24]`
    ///
    /// 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8.
    #[inline(always)]
    pub const fn with_cam_2byte_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_2BYTE_EN_MASK
            } else {
                self.bits & !CamCtrl1::CAM_2BYTE_EN_MASK
            },
        }
    }

    /// Set **CAM_VSYNC_FILTER_EN** `[23:23]`
    ///
    /// 1: Enable CAM_VSYNC filter function. 0: bypass.
    #[inline(always)]
    pub const fn with_cam_vsync_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_VSYNC_FILTER_EN_MASK
            } else {
                self.bits & !CamCtrl1::CAM_VSYNC_FILTER_EN_MASK
            },
        }
    }

    /// Set **CAM_CLK_INV** `[22:22]`
    ///
    /// 1: Invert the input signal CAM_PCLK. 0: Not invert.
    #[inline(always)]
    pub const fn with_cam_clk_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamCtrl1::CAM_CLK_INV_MASK
            } else {
                self.bits & !CamCtrl1::CAM_CLK_INV_MASK
            },
        }
    }

    /// Set **CAM_LINE_INT_NUM** `[21:16]`
    ///
    /// The line number minus 1 to generate cam_hs_int.
    #[inline(always)]
    pub const fn with_cam_line_int_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl1::CAM_LINE_INT_NUM_MASK)
                | ((value << CamCtrl1::CAM_LINE_INT_NUM_OFFSET) & CamCtrl1::CAM_LINE_INT_NUM_MASK),
        }
    }

    /// Set **CAM_REC_DATA_BYTELEN** `[15:0]`
    ///
    /// Camera receive data byte length minus 1 to set DMA in_suc_eof_int.
    #[inline(always)]
    pub const fn with_cam_rec_data_bytelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamCtrl1::CAM_REC_DATA_BYTELEN_MASK)
                | ((value << CamCtrl1::CAM_REC_DATA_BYTELEN_OFFSET)
                    & CamCtrl1::CAM_REC_DATA_BYTELEN_MASK),
        }
    }
}

impl Default for CamCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CamCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CamCtrl1Val {
        CamCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CamCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CamCtrl1Val) -> CamCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAM_RGB_YUV Register
///
/// CAM YUV/RGB converter configuration register.
///
/// Address offset: 0x000c
/// Reset value: 0x00c00000
///
/// ## Fields
///
/// - **CAM_CONV_ENABLE** `[31]` (RW)
///
///   0: Bypass converter. 1: Enable converter.
///
/// - **CAM_CONV_TRANS_MODE** `[30]` (RW)
///
///   0: YUV to RGB. 1: RGB to YUV.
///
/// - **CAM_CONV_MODE_8BITS_ON** `[29]` (RW)
///
///   0: 16bits mode. 1: 8bits mode.
///
/// - **CAM_CONV_DATA_IN_MODE** `[28]` (RW)
///
///   LIMIT or FULL mode of Data in. 0: limit. 1: full
///
/// - **CAM_CONV_DATA_OUT_MODE** `[27]` (RW)
///
///   LIMIT or FULL mode of Data out. 0: limit. 1: full
///
/// - **CAM_CONV_PROTOCOL_MODE** `[26]` (RW)
///
///   0:BT601. 1:BT709.
///
/// - **CAM_CONV_YUV_MODE** `[25:24]` (RW)
///
///   0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
///
/// - **CAM_CONV_YUV2YUV_MODE** `[23:22]` (RW)
///
///   0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
///
/// - **CAM_CONV_8BITS_DATA_INV** `[21]` (RW)
///
///   1:invert every two 8bits input data. 2. disabled.
#[repr(transparent)]
pub struct CamRgbYuv {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CamRgbYuv {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CamRgbYuv {
    // ======== Field Constants ========

    /// **CAM_CONV_ENABLE**
    ///
    /// 0: Bypass converter. 1: Enable converter.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CAM_CONV_ENABLE_OFFSET: u32 = 31;
    pub const CAM_CONV_ENABLE_MASK: u32 = 0x80000000;
    pub const CAM_CONV_ENABLE: u32 = 1 << 31;

    /// **CAM_CONV_TRANS_MODE**
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CAM_CONV_TRANS_MODE_OFFSET: u32 = 30;
    pub const CAM_CONV_TRANS_MODE_MASK: u32 = 0x40000000;
    pub const CAM_CONV_TRANS_MODE: u32 = 1 << 30;

    /// **CAM_CONV_MODE_8BITS_ON**
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CAM_CONV_MODE_8BITS_ON_OFFSET: u32 = 29;
    pub const CAM_CONV_MODE_8BITS_ON_MASK: u32 = 0x20000000;
    pub const CAM_CONV_MODE_8BITS_ON: u32 = 1 << 29;

    /// **CAM_CONV_DATA_IN_MODE**
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CAM_CONV_DATA_IN_MODE_OFFSET: u32 = 28;
    pub const CAM_CONV_DATA_IN_MODE_MASK: u32 = 0x10000000;
    pub const CAM_CONV_DATA_IN_MODE: u32 = 1 << 28;

    /// **CAM_CONV_DATA_OUT_MODE**
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CAM_CONV_DATA_OUT_MODE_OFFSET: u32 = 27;
    pub const CAM_CONV_DATA_OUT_MODE_MASK: u32 = 0x08000000;
    pub const CAM_CONV_DATA_OUT_MODE: u32 = 1 << 27;

    /// **CAM_CONV_PROTOCOL_MODE**
    ///
    /// 0:BT601. 1:BT709.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CAM_CONV_PROTOCOL_MODE_OFFSET: u32 = 26;
    pub const CAM_CONV_PROTOCOL_MODE_MASK: u32 = 0x04000000;
    pub const CAM_CONV_PROTOCOL_MODE: u32 = 1 << 26;

    /// **CAM_CONV_YUV_MODE**
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const CAM_CONV_YUV_MODE_OFFSET: u32 = 24;
    pub const CAM_CONV_YUV_MODE_MASK: u32 = 0x03000000;

    /// **CAM_CONV_YUV2YUV_MODE**
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CAM_CONV_YUV2YUV_MODE_OFFSET: u32 = 22;
    pub const CAM_CONV_YUV2YUV_MODE_MASK: u32 = 0x00c00000;

    /// **CAM_CONV_8BITS_DATA_INV**
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CAM_CONV_8BITS_DATA_INV_OFFSET: u32 = 21;
    pub const CAM_CONV_8BITS_DATA_INV_MASK: u32 = 0x00200000;
    pub const CAM_CONV_8BITS_DATA_INV: u32 = 1 << 21;

    // ======== Field Accessors ========

    /// **CAM_CONV_ENABLE** `[31]` (RW)
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub fn cam_conv_enable(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_ENABLE_MASK) != 0
    }

    /// Set CAM_CONV_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_enable(&self) {
        self.value.modify_bits(|r| r | Self::CAM_CONV_ENABLE_MASK);
    }

    /// Clear CAM_CONV_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_enable(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_CONV_ENABLE_MASK);
    }

    /// **CAM_CONV_TRANS_MODE** `[30]` (RW)
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub fn cam_conv_trans_mode(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_TRANS_MODE_MASK) != 0
    }

    /// Set CAM_CONV_TRANS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_trans_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_TRANS_MODE_MASK);
    }

    /// Clear CAM_CONV_TRANS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_trans_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_TRANS_MODE_MASK);
    }

    /// **CAM_CONV_MODE_8BITS_ON** `[29]` (RW)
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub fn cam_conv_mode_8bits_on(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_MODE_8BITS_ON_MASK) != 0
    }

    /// Set CAM_CONV_MODE_8BITS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_mode_8bits_on(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_MODE_8BITS_ON_MASK);
    }

    /// Clear CAM_CONV_MODE_8BITS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_mode_8bits_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_MODE_8BITS_ON_MASK);
    }

    /// **CAM_CONV_DATA_IN_MODE** `[28]` (RW)
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub fn cam_conv_data_in_mode(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_DATA_IN_MODE_MASK) != 0
    }

    /// Set CAM_CONV_DATA_IN_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_data_in_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_DATA_IN_MODE_MASK);
    }

    /// Clear CAM_CONV_DATA_IN_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_data_in_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_DATA_IN_MODE_MASK);
    }

    /// **CAM_CONV_DATA_OUT_MODE** `[27]` (RW)
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub fn cam_conv_data_out_mode(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_DATA_OUT_MODE_MASK) != 0
    }

    /// Set CAM_CONV_DATA_OUT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_data_out_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_DATA_OUT_MODE_MASK);
    }

    /// Clear CAM_CONV_DATA_OUT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_data_out_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_DATA_OUT_MODE_MASK);
    }

    /// **CAM_CONV_PROTOCOL_MODE** `[26]` (RW)
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub fn cam_conv_protocol_mode(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_PROTOCOL_MODE_MASK) != 0
    }

    /// Set CAM_CONV_PROTOCOL_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_protocol_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_PROTOCOL_MODE_MASK);
    }

    /// Clear CAM_CONV_PROTOCOL_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_protocol_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_PROTOCOL_MODE_MASK);
    }

    /// **CAM_CONV_YUV_MODE** `[25:24]` (RW)
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub fn cam_conv_yuv_mode(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CONV_YUV_MODE_MASK) >> Self::CAM_CONV_YUV_MODE_OFFSET
    }

    /// Write **CAM_CONV_YUV_MODE** `[25:24]` (RW)
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub fn write_cam_conv_yuv_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CONV_YUV_MODE_MASK)
                | ((value << Self::CAM_CONV_YUV_MODE_OFFSET) & Self::CAM_CONV_YUV_MODE_MASK)
        });
    }

    /// **CAM_CONV_YUV2YUV_MODE** `[23:22]` (RW)
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub fn cam_conv_yuv2yuv_mode(&self) -> u32 {
        (self.value.read_bits() & Self::CAM_CONV_YUV2YUV_MODE_MASK)
            >> Self::CAM_CONV_YUV2YUV_MODE_OFFSET
    }

    /// Write **CAM_CONV_YUV2YUV_MODE** `[23:22]` (RW)
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub fn write_cam_conv_yuv2yuv_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CAM_CONV_YUV2YUV_MODE_MASK)
                | ((value << Self::CAM_CONV_YUV2YUV_MODE_OFFSET) & Self::CAM_CONV_YUV2YUV_MODE_MASK)
        });
    }

    /// **CAM_CONV_8BITS_DATA_INV** `[21]` (RW)
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub fn cam_conv_8bits_data_inv(&self) -> bool {
        (self.value.read_bits() & Self::CAM_CONV_8BITS_DATA_INV_MASK) != 0
    }

    /// Set CAM_CONV_8BITS_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_conv_8bits_data_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::CAM_CONV_8BITS_DATA_INV_MASK);
    }

    /// Clear CAM_CONV_8BITS_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_conv_8bits_data_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_CONV_8BITS_DATA_INV_MASK);
    }
}

/// CAM_RGB_YUV value type for batch read/modify/write operations
///
/// Use `camrgbyuv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `camrgbyuv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CamRgbYuvVal {
    bits: u32,
}

impl CamRgbYuvVal {
    /// Create from raw bits
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
        Self { bits: 0x00c00000 }
    }

    /// Read **CAM_CONV_ENABLE** `[31:31]`
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub const fn cam_conv_enable(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_ENABLE_MASK) != 0
    }

    /// Read **CAM_CONV_TRANS_MODE** `[30:30]`
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub const fn cam_conv_trans_mode(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_TRANS_MODE_MASK) != 0
    }

    /// Read **CAM_CONV_MODE_8BITS_ON** `[29:29]`
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub const fn cam_conv_mode_8bits_on(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_MODE_8BITS_ON_MASK) != 0
    }

    /// Read **CAM_CONV_DATA_IN_MODE** `[28:28]`
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub const fn cam_conv_data_in_mode(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_DATA_IN_MODE_MASK) != 0
    }

    /// Read **CAM_CONV_DATA_OUT_MODE** `[27:27]`
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub const fn cam_conv_data_out_mode(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_DATA_OUT_MODE_MASK) != 0
    }

    /// Read **CAM_CONV_PROTOCOL_MODE** `[26:26]`
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub const fn cam_conv_protocol_mode(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_PROTOCOL_MODE_MASK) != 0
    }

    /// Read **CAM_CONV_YUV_MODE** `[25:24]`
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub const fn cam_conv_yuv_mode(&self) -> u32 {
        (self.bits & CamRgbYuv::CAM_CONV_YUV_MODE_MASK) >> CamRgbYuv::CAM_CONV_YUV_MODE_OFFSET
    }

    /// Read **CAM_CONV_YUV2YUV_MODE** `[23:22]`
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub const fn cam_conv_yuv2yuv_mode(&self) -> u32 {
        (self.bits & CamRgbYuv::CAM_CONV_YUV2YUV_MODE_MASK)
            >> CamRgbYuv::CAM_CONV_YUV2YUV_MODE_OFFSET
    }

    /// Read **CAM_CONV_8BITS_DATA_INV** `[21:21]`
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub const fn cam_conv_8bits_data_inv(&self) -> bool {
        (self.bits & CamRgbYuv::CAM_CONV_8BITS_DATA_INV_MASK) != 0
    }

    /// Set **CAM_CONV_ENABLE** `[31:31]`
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub const fn with_cam_conv_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_ENABLE_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_ENABLE_MASK
            },
        }
    }

    /// Set **CAM_CONV_TRANS_MODE** `[30:30]`
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub const fn with_cam_conv_trans_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_TRANS_MODE_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_TRANS_MODE_MASK
            },
        }
    }

    /// Set **CAM_CONV_MODE_8BITS_ON** `[29:29]`
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub const fn with_cam_conv_mode_8bits_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_MODE_8BITS_ON_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_MODE_8BITS_ON_MASK
            },
        }
    }

    /// Set **CAM_CONV_DATA_IN_MODE** `[28:28]`
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub const fn with_cam_conv_data_in_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_DATA_IN_MODE_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_DATA_IN_MODE_MASK
            },
        }
    }

    /// Set **CAM_CONV_DATA_OUT_MODE** `[27:27]`
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub const fn with_cam_conv_data_out_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_DATA_OUT_MODE_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_DATA_OUT_MODE_MASK
            },
        }
    }

    /// Set **CAM_CONV_PROTOCOL_MODE** `[26:26]`
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub const fn with_cam_conv_protocol_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_PROTOCOL_MODE_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_PROTOCOL_MODE_MASK
            },
        }
    }

    /// Set **CAM_CONV_YUV_MODE** `[25:24]`
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub const fn with_cam_conv_yuv_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamRgbYuv::CAM_CONV_YUV_MODE_MASK)
                | ((value << CamRgbYuv::CAM_CONV_YUV_MODE_OFFSET)
                    & CamRgbYuv::CAM_CONV_YUV_MODE_MASK),
        }
    }

    /// Set **CAM_CONV_YUV2YUV_MODE** `[23:22]`
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub const fn with_cam_conv_yuv2yuv_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CamRgbYuv::CAM_CONV_YUV2YUV_MODE_MASK)
                | ((value << CamRgbYuv::CAM_CONV_YUV2YUV_MODE_OFFSET)
                    & CamRgbYuv::CAM_CONV_YUV2YUV_MODE_MASK),
        }
    }

    /// Set **CAM_CONV_8BITS_DATA_INV** `[21:21]`
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub const fn with_cam_conv_8bits_data_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CamRgbYuv::CAM_CONV_8BITS_DATA_INV_MASK
            } else {
                self.bits & !CamRgbYuv::CAM_CONV_8BITS_DATA_INV_MASK
            },
        }
    }
}

impl Default for CamRgbYuvVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CamRgbYuv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CamRgbYuvVal {
        CamRgbYuvVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CamRgbYuvVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CamRgbYuvVal) -> CamRgbYuvVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_RGB_YUV Register
///
/// LCD YUV/RGB converter configuration register.
///
/// Address offset: 0x0010
/// Reset value: 0x00c00000
///
/// ## Fields
///
/// - **LCD_CONV_ENABLE** `[31]` (RW)
///
///   0: Bypass converter. 1: Enable converter.
///
/// - **LCD_CONV_TRANS_MODE** `[30]` (RW)
///
///   0: YUV to RGB. 1: RGB to YUV.
///
/// - **LCD_CONV_MODE_8BITS_ON** `[29]` (RW)
///
///   0: 16bits mode. 1: 8bits mode.
///
/// - **LCD_CONV_DATA_IN_MODE** `[28]` (RW)
///
///   LIMIT or FULL mode of Data in. 0: limit. 1: full
///
/// - **LCD_CONV_DATA_OUT_MODE** `[27]` (RW)
///
///   LIMIT or FULL mode of Data out. 0: limit. 1: full
///
/// - **LCD_CONV_PROTOCOL_MODE** `[26]` (RW)
///
///   0:BT601. 1:BT709.
///
/// - **LCD_CONV_YUV_MODE** `[25:24]` (RW)
///
///   0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
///
/// - **LCD_CONV_YUV2YUV_MODE** `[23:22]` (RW)
///
///   0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
///
/// - **LCD_CONV_TXTORX** `[21]` (RW)
///
///   0: txtorx mode off. 1: txtorx mode on.
///
/// - **LCD_CONV_8BITS_DATA_INV** `[20]` (RW)
///
///   1:invert every two 8bits input data. 2. disabled.
#[repr(transparent)]
pub struct LcdRgbYuv {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdRgbYuv {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdRgbYuv {
    // ======== Field Constants ========

    /// **LCD_CONV_ENABLE**
    ///
    /// 0: Bypass converter. 1: Enable converter.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LCD_CONV_ENABLE_OFFSET: u32 = 31;
    pub const LCD_CONV_ENABLE_MASK: u32 = 0x80000000;
    pub const LCD_CONV_ENABLE: u32 = 1 << 31;

    /// **LCD_CONV_TRANS_MODE**
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LCD_CONV_TRANS_MODE_OFFSET: u32 = 30;
    pub const LCD_CONV_TRANS_MODE_MASK: u32 = 0x40000000;
    pub const LCD_CONV_TRANS_MODE: u32 = 1 << 30;

    /// **LCD_CONV_MODE_8BITS_ON**
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LCD_CONV_MODE_8BITS_ON_OFFSET: u32 = 29;
    pub const LCD_CONV_MODE_8BITS_ON_MASK: u32 = 0x20000000;
    pub const LCD_CONV_MODE_8BITS_ON: u32 = 1 << 29;

    /// **LCD_CONV_DATA_IN_MODE**
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LCD_CONV_DATA_IN_MODE_OFFSET: u32 = 28;
    pub const LCD_CONV_DATA_IN_MODE_MASK: u32 = 0x10000000;
    pub const LCD_CONV_DATA_IN_MODE: u32 = 1 << 28;

    /// **LCD_CONV_DATA_OUT_MODE**
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LCD_CONV_DATA_OUT_MODE_OFFSET: u32 = 27;
    pub const LCD_CONV_DATA_OUT_MODE_MASK: u32 = 0x08000000;
    pub const LCD_CONV_DATA_OUT_MODE: u32 = 1 << 27;

    /// **LCD_CONV_PROTOCOL_MODE**
    ///
    /// 0:BT601. 1:BT709.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LCD_CONV_PROTOCOL_MODE_OFFSET: u32 = 26;
    pub const LCD_CONV_PROTOCOL_MODE_MASK: u32 = 0x04000000;
    pub const LCD_CONV_PROTOCOL_MODE: u32 = 1 << 26;

    /// **LCD_CONV_YUV_MODE**
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const LCD_CONV_YUV_MODE_OFFSET: u32 = 24;
    pub const LCD_CONV_YUV_MODE_MASK: u32 = 0x03000000;

    /// **LCD_CONV_YUV2YUV_MODE**
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const LCD_CONV_YUV2YUV_MODE_OFFSET: u32 = 22;
    pub const LCD_CONV_YUV2YUV_MODE_MASK: u32 = 0x00c00000;

    /// **LCD_CONV_TXTORX**
    ///
    /// 0: txtorx mode off. 1: txtorx mode on.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LCD_CONV_TXTORX_OFFSET: u32 = 21;
    pub const LCD_CONV_TXTORX_MASK: u32 = 0x00200000;
    pub const LCD_CONV_TXTORX: u32 = 1 << 21;

    /// **LCD_CONV_8BITS_DATA_INV**
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LCD_CONV_8BITS_DATA_INV_OFFSET: u32 = 20;
    pub const LCD_CONV_8BITS_DATA_INV_MASK: u32 = 0x00100000;
    pub const LCD_CONV_8BITS_DATA_INV: u32 = 1 << 20;

    // ======== Field Accessors ========

    /// **LCD_CONV_ENABLE** `[31]` (RW)
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub fn lcd_conv_enable(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_ENABLE_MASK) != 0
    }

    /// Set LCD_CONV_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_enable(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CONV_ENABLE_MASK);
    }

    /// Clear LCD_CONV_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_enable(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CONV_ENABLE_MASK);
    }

    /// **LCD_CONV_TRANS_MODE** `[30]` (RW)
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_TRANS_MODE_MASK) != 0
    }

    /// Set LCD_CONV_TRANS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_trans_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_TRANS_MODE_MASK);
    }

    /// Clear LCD_CONV_TRANS_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_trans_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_TRANS_MODE_MASK);
    }

    /// **LCD_CONV_MODE_8BITS_ON** `[29]` (RW)
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_MODE_8BITS_ON_MASK) != 0
    }

    /// Set LCD_CONV_MODE_8BITS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_mode_8bits_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_MODE_8BITS_ON_MASK);
    }

    /// Clear LCD_CONV_MODE_8BITS_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_mode_8bits_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_MODE_8BITS_ON_MASK);
    }

    /// **LCD_CONV_DATA_IN_MODE** `[28]` (RW)
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_DATA_IN_MODE_MASK) != 0
    }

    /// Set LCD_CONV_DATA_IN_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_data_in_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_DATA_IN_MODE_MASK);
    }

    /// Clear LCD_CONV_DATA_IN_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_data_in_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_DATA_IN_MODE_MASK);
    }

    /// **LCD_CONV_DATA_OUT_MODE** `[27]` (RW)
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_DATA_OUT_MODE_MASK) != 0
    }

    /// Set LCD_CONV_DATA_OUT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_data_out_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_DATA_OUT_MODE_MASK);
    }

    /// Clear LCD_CONV_DATA_OUT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_data_out_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_DATA_OUT_MODE_MASK);
    }

    /// **LCD_CONV_PROTOCOL_MODE** `[26]` (RW)
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_PROTOCOL_MODE_MASK) != 0
    }

    /// Set LCD_CONV_PROTOCOL_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_protocol_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_PROTOCOL_MODE_MASK);
    }

    /// Clear LCD_CONV_PROTOCOL_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_protocol_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_PROTOCOL_MODE_MASK);
    }

    /// **LCD_CONV_YUV_MODE** `[25:24]` (RW)
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CONV_YUV_MODE_MASK) >> Self::LCD_CONV_YUV_MODE_OFFSET
    }

    /// Write **LCD_CONV_YUV_MODE** `[25:24]` (RW)
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub fn write_lcd_conv_yuv_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CONV_YUV_MODE_MASK)
                | ((value << Self::LCD_CONV_YUV_MODE_OFFSET) & Self::LCD_CONV_YUV_MODE_MASK)
        });
    }

    /// **LCD_CONV_YUV2YUV_MODE** `[23:22]` (RW)
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CONV_YUV2YUV_MODE_MASK)
            >> Self::LCD_CONV_YUV2YUV_MODE_OFFSET
    }

    /// Write **LCD_CONV_YUV2YUV_MODE** `[23:22]` (RW)
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub fn write_lcd_conv_yuv2yuv_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CONV_YUV2YUV_MODE_MASK)
                | ((value << Self::LCD_CONV_YUV2YUV_MODE_OFFSET) & Self::LCD_CONV_YUV2YUV_MODE_MASK)
        });
    }

    /// **LCD_CONV_TXTORX** `[21]` (RW)
    ///
    /// 0: txtorx mode off. 1: txtorx mode on.
    #[inline(always)]
    pub fn lcd_conv_txtorx(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_TXTORX_MASK) != 0
    }

    /// Set LCD_CONV_TXTORX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_txtorx(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CONV_TXTORX_MASK);
    }

    /// Clear LCD_CONV_TXTORX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_txtorx(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CONV_TXTORX_MASK);
    }

    /// **LCD_CONV_8BITS_DATA_INV** `[20]` (RW)
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CONV_8BITS_DATA_INV_MASK) != 0
    }

    /// Set LCD_CONV_8BITS_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_conv_8bits_data_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CONV_8BITS_DATA_INV_MASK);
    }

    /// Clear LCD_CONV_8BITS_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_conv_8bits_data_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CONV_8BITS_DATA_INV_MASK);
    }
}

/// LCD_RGB_YUV value type for batch read/modify/write operations
///
/// Use `lcdrgbyuv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdrgbyuv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdRgbYuvVal {
    bits: u32,
}

impl LcdRgbYuvVal {
    /// Create from raw bits
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
        Self { bits: 0x00c00000 }
    }

    /// Read **LCD_CONV_ENABLE** `[31:31]`
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub const fn lcd_conv_enable(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_ENABLE_MASK) != 0
    }

    /// Read **LCD_CONV_TRANS_MODE** `[30:30]`
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub const fn lcd_conv_trans_mode(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_TRANS_MODE_MASK) != 0
    }

    /// Read **LCD_CONV_MODE_8BITS_ON** `[29:29]`
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub const fn lcd_conv_mode_8bits_on(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_MODE_8BITS_ON_MASK) != 0
    }

    /// Read **LCD_CONV_DATA_IN_MODE** `[28:28]`
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub const fn lcd_conv_data_in_mode(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_DATA_IN_MODE_MASK) != 0
    }

    /// Read **LCD_CONV_DATA_OUT_MODE** `[27:27]`
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub const fn lcd_conv_data_out_mode(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_DATA_OUT_MODE_MASK) != 0
    }

    /// Read **LCD_CONV_PROTOCOL_MODE** `[26:26]`
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub const fn lcd_conv_protocol_mode(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_PROTOCOL_MODE_MASK) != 0
    }

    /// Read **LCD_CONV_YUV_MODE** `[25:24]`
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub const fn lcd_conv_yuv_mode(&self) -> u32 {
        (self.bits & LcdRgbYuv::LCD_CONV_YUV_MODE_MASK) >> LcdRgbYuv::LCD_CONV_YUV_MODE_OFFSET
    }

    /// Read **LCD_CONV_YUV2YUV_MODE** `[23:22]`
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub const fn lcd_conv_yuv2yuv_mode(&self) -> u32 {
        (self.bits & LcdRgbYuv::LCD_CONV_YUV2YUV_MODE_MASK)
            >> LcdRgbYuv::LCD_CONV_YUV2YUV_MODE_OFFSET
    }

    /// Read **LCD_CONV_TXTORX** `[21:21]`
    ///
    /// 0: txtorx mode off. 1: txtorx mode on.
    #[inline(always)]
    pub const fn lcd_conv_txtorx(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_TXTORX_MASK) != 0
    }

    /// Read **LCD_CONV_8BITS_DATA_INV** `[20:20]`
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub const fn lcd_conv_8bits_data_inv(&self) -> bool {
        (self.bits & LcdRgbYuv::LCD_CONV_8BITS_DATA_INV_MASK) != 0
    }

    /// Set **LCD_CONV_ENABLE** `[31:31]`
    ///
    /// 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub const fn with_lcd_conv_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_ENABLE_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_ENABLE_MASK
            },
        }
    }

    /// Set **LCD_CONV_TRANS_MODE** `[30:30]`
    ///
    /// 0: YUV to RGB. 1: RGB to YUV.
    #[inline(always)]
    pub const fn with_lcd_conv_trans_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_TRANS_MODE_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_TRANS_MODE_MASK
            },
        }
    }

    /// Set **LCD_CONV_MODE_8BITS_ON** `[29:29]`
    ///
    /// 0: 16bits mode. 1: 8bits mode.
    #[inline(always)]
    pub const fn with_lcd_conv_mode_8bits_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_MODE_8BITS_ON_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_MODE_8BITS_ON_MASK
            },
        }
    }

    /// Set **LCD_CONV_DATA_IN_MODE** `[28:28]`
    ///
    /// LIMIT or FULL mode of Data in. 0: limit. 1: full
    #[inline(always)]
    pub const fn with_lcd_conv_data_in_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_DATA_IN_MODE_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_DATA_IN_MODE_MASK
            },
        }
    }

    /// Set **LCD_CONV_DATA_OUT_MODE** `[27:27]`
    ///
    /// LIMIT or FULL mode of Data out. 0: limit. 1: full
    #[inline(always)]
    pub const fn with_lcd_conv_data_out_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_DATA_OUT_MODE_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_DATA_OUT_MODE_MASK
            },
        }
    }

    /// Set **LCD_CONV_PROTOCOL_MODE** `[26:26]`
    ///
    /// 0:BT601. 1:BT709.
    #[inline(always)]
    pub const fn with_lcd_conv_protocol_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_PROTOCOL_MODE_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_PROTOCOL_MODE_MASK
            },
        }
    }

    /// Set **LCD_CONV_YUV_MODE** `[25:24]`
    ///
    /// 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in
    #[inline(always)]
    pub const fn with_lcd_conv_yuv_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdRgbYuv::LCD_CONV_YUV_MODE_MASK)
                | ((value << LcdRgbYuv::LCD_CONV_YUV_MODE_OFFSET)
                    & LcdRgbYuv::LCD_CONV_YUV_MODE_MASK),
        }
    }

    /// Set **LCD_CONV_YUV2YUV_MODE** `[23:22]`
    ///
    /// 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1.
    #[inline(always)]
    pub const fn with_lcd_conv_yuv2yuv_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdRgbYuv::LCD_CONV_YUV2YUV_MODE_MASK)
                | ((value << LcdRgbYuv::LCD_CONV_YUV2YUV_MODE_OFFSET)
                    & LcdRgbYuv::LCD_CONV_YUV2YUV_MODE_MASK),
        }
    }

    /// Set **LCD_CONV_TXTORX** `[21:21]`
    ///
    /// 0: txtorx mode off. 1: txtorx mode on.
    #[inline(always)]
    pub const fn with_lcd_conv_txtorx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_TXTORX_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_TXTORX_MASK
            },
        }
    }

    /// Set **LCD_CONV_8BITS_DATA_INV** `[20:20]`
    ///
    /// 1:invert every two 8bits input data. 2. disabled.
    #[inline(always)]
    pub const fn with_lcd_conv_8bits_data_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdRgbYuv::LCD_CONV_8BITS_DATA_INV_MASK
            } else {
                self.bits & !LcdRgbYuv::LCD_CONV_8BITS_DATA_INV_MASK
            },
        }
    }
}

impl Default for LcdRgbYuvVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcdRgbYuv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdRgbYuvVal {
        LcdRgbYuvVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdRgbYuvVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdRgbYuvVal) -> LcdRgbYuvVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_USER Register
///
/// LCD config register.
///
/// Address offset: 0x0014
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **LCD_CMD_2_CYCLE_EN** `[31]` (RW)
///
///   The cycle length of command phase. 1: 2 cycles. 0: 1 cycle.
///
/// - **LCD_DUMMY_CYCLELEN** `[30:29]` (RW)
///
///   The dummy cycle length minus 1.
///
/// - **LCD_RESET** `[28]` (WO)
///
///   The value of command.
///
/// - **LCD_START** `[27]` (RW)
///
///   LCD start sending data enable signal, valid in high level.
///
/// - **LCD_CMD** `[26]` (RW)
///
///   1: Be able to send command in LCD sequence when LCD starts. 0: Disable.
///
/// - **LCD_DUMMY** `[25]` (RW)
///
///   1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable.
///
/// - **LCD_DOUT** `[24]` (RW)
///
///   1: Be able to send data out in LCD sequence when LCD starts. 0: Disable.
///
/// - **LCD_BYTE_ORDER** `[23]` (RW)
///
///   1: invert data byte order, only valid in 2 byte mode. 0: Not change.
///
/// - **LCD_BIT_ORDER** `[22]` (RW)
///
///   1: Change data bit order, change LCD_DATA_out[7:0] to LCD_DATA_out[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
///
/// - **LCD_UPDATE** `[21]` (RW)
///
///   1: Update LCD registers, will be cleared by hardware. 0 : Not care.
///
/// - **LCD_BYTE_MODE** `[20:19]` (RW)
///
///   2: 24bit mode. 1: 16bit mode. 0: 8bit mode
///
/// - **LCD_DOUT_BIT_ORDER** `[18]` (RW)
///
///   1: change bit order in every byte. 0: Not change.
///
/// - **LCD_DOUT_BYTE_SWIZZLE_ENABLE** `[17]` (RW)
///
///   1: enable byte swizzle 0: disable
///
/// - **LCD_DOUT_BYTE_SWIZZLE_MODE** `[16:14]` (RW)
///
///   0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
///
/// - **LCD_ALWAYS_OUT_EN** `[13]` (RW)
///
///   LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set.
///
/// - **LCD_DOUT_CYCLELEN** `[12:0]` (RW)
///
///   The output data cycles minus 1 of LCD module.
#[repr(transparent)]
pub struct LcdUser {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdUser {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdUser {
    // ======== Field Constants ========

    /// **LCD_CMD_2_CYCLE_EN**
    ///
    /// The cycle length of command phase. 1: 2 cycles. 0: 1 cycle.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LCD_CMD_2_CYCLE_EN_OFFSET: u32 = 31;
    pub const LCD_CMD_2_CYCLE_EN_MASK: u32 = 0x80000000;
    pub const LCD_CMD_2_CYCLE_EN: u32 = 1 << 31;

    /// **LCD_DUMMY_CYCLELEN**
    ///
    /// The dummy cycle length minus 1.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const LCD_DUMMY_CYCLELEN_OFFSET: u32 = 29;
    pub const LCD_DUMMY_CYCLELEN_MASK: u32 = 0x60000000;

    /// **LCD_RESET**
    ///
    /// The value of command.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LCD_RESET_OFFSET: u32 = 28;
    pub const LCD_RESET_MASK: u32 = 0x10000000;
    pub const LCD_RESET: u32 = 1 << 28;

    /// **LCD_START**
    ///
    /// LCD start sending data enable signal, valid in high level.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LCD_START_OFFSET: u32 = 27;
    pub const LCD_START_MASK: u32 = 0x08000000;
    pub const LCD_START: u32 = 1 << 27;

    /// **LCD_CMD**
    ///
    /// 1: Be able to send command in LCD sequence when LCD starts. 0: Disable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LCD_CMD_OFFSET: u32 = 26;
    pub const LCD_CMD_MASK: u32 = 0x04000000;
    pub const LCD_CMD: u32 = 1 << 26;

    /// **LCD_DUMMY**
    ///
    /// 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LCD_DUMMY_OFFSET: u32 = 25;
    pub const LCD_DUMMY_MASK: u32 = 0x02000000;
    pub const LCD_DUMMY: u32 = 1 << 25;

    /// **LCD_DOUT**
    ///
    /// 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const LCD_DOUT_OFFSET: u32 = 24;
    pub const LCD_DOUT_MASK: u32 = 0x01000000;
    pub const LCD_DOUT: u32 = 1 << 24;

    /// **LCD_BYTE_ORDER**
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const LCD_BYTE_ORDER_OFFSET: u32 = 23;
    pub const LCD_BYTE_ORDER_MASK: u32 = 0x00800000;
    pub const LCD_BYTE_ORDER: u32 = 1 << 23;

    /// **LCD_BIT_ORDER**
    ///
    /// 1: Change data bit order, change LCD_DATA_out[7:0] to LCD_DATA_out[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const LCD_BIT_ORDER_OFFSET: u32 = 22;
    pub const LCD_BIT_ORDER_MASK: u32 = 0x00400000;
    pub const LCD_BIT_ORDER: u32 = 1 << 22;

    /// **LCD_UPDATE**
    ///
    /// 1: Update LCD registers, will be cleared by hardware. 0 : Not care.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LCD_UPDATE_OFFSET: u32 = 21;
    pub const LCD_UPDATE_MASK: u32 = 0x00200000;
    pub const LCD_UPDATE: u32 = 1 << 21;

    /// **LCD_BYTE_MODE**
    ///
    /// 2: 24bit mode. 1: 16bit mode. 0: 8bit mode
    ///
    /// Bits `[20:19]`, mask `0x00180000`
    pub const LCD_BYTE_MODE_OFFSET: u32 = 19;
    pub const LCD_BYTE_MODE_MASK: u32 = 0x00180000;

    /// **LCD_DOUT_BIT_ORDER**
    ///
    /// 1: change bit order in every byte. 0: Not change.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const LCD_DOUT_BIT_ORDER_OFFSET: u32 = 18;
    pub const LCD_DOUT_BIT_ORDER_MASK: u32 = 0x00040000;
    pub const LCD_DOUT_BIT_ORDER: u32 = 1 << 18;

    /// **LCD_DOUT_BYTE_SWIZZLE_ENABLE**
    ///
    /// 1: enable byte swizzle 0: disable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const LCD_DOUT_BYTE_SWIZZLE_ENABLE_OFFSET: u32 = 17;
    pub const LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK: u32 = 0x00020000;
    pub const LCD_DOUT_BYTE_SWIZZLE_ENABLE: u32 = 1 << 17;

    /// **LCD_DOUT_BYTE_SWIZZLE_MODE**
    ///
    /// 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
    ///
    /// Bits `[16:14]`, mask `0x0001c000`
    pub const LCD_DOUT_BYTE_SWIZZLE_MODE_OFFSET: u32 = 14;
    pub const LCD_DOUT_BYTE_SWIZZLE_MODE_MASK: u32 = 0x0001c000;

    /// **LCD_ALWAYS_OUT_EN**
    ///
    /// LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LCD_ALWAYS_OUT_EN_OFFSET: u32 = 13;
    pub const LCD_ALWAYS_OUT_EN_MASK: u32 = 0x00002000;
    pub const LCD_ALWAYS_OUT_EN: u32 = 1 << 13;

    /// **LCD_DOUT_CYCLELEN**
    ///
    /// The output data cycles minus 1 of LCD module.
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const LCD_DOUT_CYCLELEN_OFFSET: u32 = 0;
    pub const LCD_DOUT_CYCLELEN_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **LCD_CMD_2_CYCLE_EN** `[31]` (RW)
    ///
    /// The cycle length of command phase. 1: 2 cycles. 0: 1 cycle.
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CMD_2_CYCLE_EN_MASK) != 0
    }

    /// Set LCD_CMD_2_CYCLE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cmd_2_cycle_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_CMD_2_CYCLE_EN_MASK);
    }

    /// Clear LCD_CMD_2_CYCLE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cmd_2_cycle_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_CMD_2_CYCLE_EN_MASK);
    }

    /// **LCD_DUMMY_CYCLELEN** `[30:29]` (RW)
    ///
    /// The dummy cycle length minus 1.
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_DUMMY_CYCLELEN_MASK) >> Self::LCD_DUMMY_CYCLELEN_OFFSET
    }

    /// Write **LCD_DUMMY_CYCLELEN** `[30:29]` (RW)
    ///
    /// The dummy cycle length minus 1.
    #[inline(always)]
    pub fn write_lcd_dummy_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_DUMMY_CYCLELEN_MASK)
                | ((value << Self::LCD_DUMMY_CYCLELEN_OFFSET) & Self::LCD_DUMMY_CYCLELEN_MASK)
        });
    }

    /// Trigger **LCD_RESET** (WO)
    ///
    /// The value of command.
    #[inline(always)]
    pub fn trigger_lcd_reset(&self) {
        self.value.modify_bits(|r| r | Self::LCD_RESET_MASK);
    }

    /// **LCD_START** `[27]` (RW)
    ///
    /// LCD start sending data enable signal, valid in high level.
    #[inline(always)]
    pub fn lcd_start(&self) -> bool {
        (self.value.read_bits() & Self::LCD_START_MASK) != 0
    }

    /// Set LCD_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_start(&self) {
        self.value.modify_bits(|r| r | Self::LCD_START_MASK);
    }

    /// Clear LCD_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_start(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_START_MASK);
    }

    /// **LCD_CMD** `[26]` (RW)
    ///
    /// 1: Be able to send command in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub fn lcd_cmd(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CMD_MASK) != 0
    }

    /// Set LCD_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cmd(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CMD_MASK);
    }

    /// Clear LCD_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cmd(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CMD_MASK);
    }

    /// **LCD_DUMMY** `[25]` (RW)
    ///
    /// 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub fn lcd_dummy(&self) -> bool {
        (self.value.read_bits() & Self::LCD_DUMMY_MASK) != 0
    }

    /// Set LCD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_dummy(&self) {
        self.value.modify_bits(|r| r | Self::LCD_DUMMY_MASK);
    }

    /// Clear LCD_DUMMY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_dummy(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_DUMMY_MASK);
    }

    /// **LCD_DOUT** `[24]` (RW)
    ///
    /// 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub fn lcd_dout(&self) -> bool {
        (self.value.read_bits() & Self::LCD_DOUT_MASK) != 0
    }

    /// Set LCD_DOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_dout(&self) {
        self.value.modify_bits(|r| r | Self::LCD_DOUT_MASK);
    }

    /// Clear LCD_DOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_dout(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_DOUT_MASK);
    }

    /// **LCD_BYTE_ORDER** `[23]` (RW)
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> bool {
        (self.value.read_bits() & Self::LCD_BYTE_ORDER_MASK) != 0
    }

    /// Set LCD_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_byte_order(&self) {
        self.value.modify_bits(|r| r | Self::LCD_BYTE_ORDER_MASK);
    }

    /// Clear LCD_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_byte_order(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_BYTE_ORDER_MASK);
    }

    /// **LCD_BIT_ORDER** `[22]` (RW)
    ///
    /// 1: Change data bit order, change LCD_DATA_out[7:0] to LCD_DATA_out[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::LCD_BIT_ORDER_MASK) != 0
    }

    /// Set LCD_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_bit_order(&self) {
        self.value.modify_bits(|r| r | Self::LCD_BIT_ORDER_MASK);
    }

    /// Clear LCD_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_bit_order(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_BIT_ORDER_MASK);
    }

    /// **LCD_UPDATE** `[21]` (RW)
    ///
    /// 1: Update LCD registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub fn lcd_update(&self) -> bool {
        (self.value.read_bits() & Self::LCD_UPDATE_MASK) != 0
    }

    /// Set LCD_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_update(&self) {
        self.value.modify_bits(|r| r | Self::LCD_UPDATE_MASK);
    }

    /// Clear LCD_UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_update(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_UPDATE_MASK);
    }

    /// **LCD_BYTE_MODE** `[20:19]` (RW)
    ///
    /// 2: 24bit mode. 1: 16bit mode. 0: 8bit mode
    #[inline(always)]
    pub fn lcd_byte_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_BYTE_MODE_MASK) >> Self::LCD_BYTE_MODE_OFFSET
    }

    /// Write **LCD_BYTE_MODE** `[20:19]` (RW)
    ///
    /// 2: 24bit mode. 1: 16bit mode. 0: 8bit mode
    #[inline(always)]
    pub fn write_lcd_byte_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_BYTE_MODE_MASK)
                | ((value << Self::LCD_BYTE_MODE_OFFSET) & Self::LCD_BYTE_MODE_MASK)
        });
    }

    /// **LCD_DOUT_BIT_ORDER** `[18]` (RW)
    ///
    /// 1: change bit order in every byte. 0: Not change.
    #[inline(always)]
    pub fn lcd_dout_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::LCD_DOUT_BIT_ORDER_MASK) != 0
    }

    /// Set LCD_DOUT_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_dout_bit_order(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_DOUT_BIT_ORDER_MASK);
    }

    /// Clear LCD_DOUT_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_dout_bit_order(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_DOUT_BIT_ORDER_MASK);
    }

    /// **LCD_DOUT_BYTE_SWIZZLE_ENABLE** `[17]` (RW)
    ///
    /// 1: enable byte swizzle 0: disable
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_enable(&self) -> bool {
        (self.value.read_bits() & Self::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK) != 0
    }

    /// Set LCD_DOUT_BYTE_SWIZZLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_dout_byte_swizzle_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK);
    }

    /// Clear LCD_DOUT_BYTE_SWIZZLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_dout_byte_swizzle_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK);
    }

    /// **LCD_DOUT_BYTE_SWIZZLE_MODE** `[16:14]` (RW)
    ///
    /// 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK)
            >> Self::LCD_DOUT_BYTE_SWIZZLE_MODE_OFFSET
    }

    /// Write **LCD_DOUT_BYTE_SWIZZLE_MODE** `[16:14]` (RW)
    ///
    /// 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
    #[inline(always)]
    pub fn write_lcd_dout_byte_swizzle_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK)
                | ((value << Self::LCD_DOUT_BYTE_SWIZZLE_MODE_OFFSET)
                    & Self::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK)
        });
    }

    /// **LCD_ALWAYS_OUT_EN** `[13]` (RW)
    ///
    /// LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set.
    #[inline(always)]
    pub fn lcd_always_out_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_ALWAYS_OUT_EN_MASK) != 0
    }

    /// Set LCD_ALWAYS_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_always_out_en(&self) {
        self.value.modify_bits(|r| r | Self::LCD_ALWAYS_OUT_EN_MASK);
    }

    /// Clear LCD_ALWAYS_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_always_out_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_ALWAYS_OUT_EN_MASK);
    }

    /// **LCD_DOUT_CYCLELEN** `[12:0]` (RW)
    ///
    /// The output data cycles minus 1 of LCD module.
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_DOUT_CYCLELEN_MASK) >> Self::LCD_DOUT_CYCLELEN_OFFSET
    }

    /// Write **LCD_DOUT_CYCLELEN** `[12:0]` (RW)
    ///
    /// The output data cycles minus 1 of LCD module.
    #[inline(always)]
    pub fn write_lcd_dout_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_DOUT_CYCLELEN_MASK)
                | ((value << Self::LCD_DOUT_CYCLELEN_OFFSET) & Self::LCD_DOUT_CYCLELEN_MASK)
        });
    }
}

/// LCD_USER value type for batch read/modify/write operations
///
/// Use `lcduser.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcduser.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdUserVal {
    bits: u32,
}

impl LcdUserVal {
    /// Create from raw bits
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

    /// Read **LCD_CMD_2_CYCLE_EN** `[31:31]`
    ///
    /// The cycle length of command phase. 1: 2 cycles. 0: 1 cycle.
    #[inline(always)]
    pub const fn lcd_cmd_2_cycle_en(&self) -> bool {
        (self.bits & LcdUser::LCD_CMD_2_CYCLE_EN_MASK) != 0
    }

    /// Read **LCD_DUMMY_CYCLELEN** `[30:29]`
    ///
    /// The dummy cycle length minus 1.
    #[inline(always)]
    pub const fn lcd_dummy_cyclelen(&self) -> u32 {
        (self.bits & LcdUser::LCD_DUMMY_CYCLELEN_MASK) >> LcdUser::LCD_DUMMY_CYCLELEN_OFFSET
    }

    /// Read **LCD_START** `[27:27]`
    ///
    /// LCD start sending data enable signal, valid in high level.
    #[inline(always)]
    pub const fn lcd_start(&self) -> bool {
        (self.bits & LcdUser::LCD_START_MASK) != 0
    }

    /// Read **LCD_CMD** `[26:26]`
    ///
    /// 1: Be able to send command in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn lcd_cmd(&self) -> bool {
        (self.bits & LcdUser::LCD_CMD_MASK) != 0
    }

    /// Read **LCD_DUMMY** `[25:25]`
    ///
    /// 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn lcd_dummy(&self) -> bool {
        (self.bits & LcdUser::LCD_DUMMY_MASK) != 0
    }

    /// Read **LCD_DOUT** `[24:24]`
    ///
    /// 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn lcd_dout(&self) -> bool {
        (self.bits & LcdUser::LCD_DOUT_MASK) != 0
    }

    /// Read **LCD_BYTE_ORDER** `[23:23]`
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub const fn lcd_byte_order(&self) -> bool {
        (self.bits & LcdUser::LCD_BYTE_ORDER_MASK) != 0
    }

    /// Read **LCD_BIT_ORDER** `[22:22]`
    ///
    /// 1: Change data bit order, change LCD_DATA_out[7:0] to LCD_DATA_out[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub const fn lcd_bit_order(&self) -> bool {
        (self.bits & LcdUser::LCD_BIT_ORDER_MASK) != 0
    }

    /// Read **LCD_UPDATE** `[21:21]`
    ///
    /// 1: Update LCD registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub const fn lcd_update(&self) -> bool {
        (self.bits & LcdUser::LCD_UPDATE_MASK) != 0
    }

    /// Read **LCD_BYTE_MODE** `[20:19]`
    ///
    /// 2: 24bit mode. 1: 16bit mode. 0: 8bit mode
    #[inline(always)]
    pub const fn lcd_byte_mode(&self) -> u32 {
        (self.bits & LcdUser::LCD_BYTE_MODE_MASK) >> LcdUser::LCD_BYTE_MODE_OFFSET
    }

    /// Read **LCD_DOUT_BIT_ORDER** `[18:18]`
    ///
    /// 1: change bit order in every byte. 0: Not change.
    #[inline(always)]
    pub const fn lcd_dout_bit_order(&self) -> bool {
        (self.bits & LcdUser::LCD_DOUT_BIT_ORDER_MASK) != 0
    }

    /// Read **LCD_DOUT_BYTE_SWIZZLE_ENABLE** `[17:17]`
    ///
    /// 1: enable byte swizzle 0: disable
    #[inline(always)]
    pub const fn lcd_dout_byte_swizzle_enable(&self) -> bool {
        (self.bits & LcdUser::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK) != 0
    }

    /// Read **LCD_DOUT_BYTE_SWIZZLE_MODE** `[16:14]`
    ///
    /// 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
    #[inline(always)]
    pub const fn lcd_dout_byte_swizzle_mode(&self) -> u32 {
        (self.bits & LcdUser::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK)
            >> LcdUser::LCD_DOUT_BYTE_SWIZZLE_MODE_OFFSET
    }

    /// Read **LCD_ALWAYS_OUT_EN** `[13:13]`
    ///
    /// LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set.
    #[inline(always)]
    pub const fn lcd_always_out_en(&self) -> bool {
        (self.bits & LcdUser::LCD_ALWAYS_OUT_EN_MASK) != 0
    }

    /// Read **LCD_DOUT_CYCLELEN** `[12:0]`
    ///
    /// The output data cycles minus 1 of LCD module.
    #[inline(always)]
    pub const fn lcd_dout_cyclelen(&self) -> u32 {
        (self.bits & LcdUser::LCD_DOUT_CYCLELEN_MASK) >> LcdUser::LCD_DOUT_CYCLELEN_OFFSET
    }

    /// Set **LCD_CMD_2_CYCLE_EN** `[31:31]`
    ///
    /// The cycle length of command phase. 1: 2 cycles. 0: 1 cycle.
    #[inline(always)]
    pub const fn with_lcd_cmd_2_cycle_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_CMD_2_CYCLE_EN_MASK
            } else {
                self.bits & !LcdUser::LCD_CMD_2_CYCLE_EN_MASK
            },
        }
    }

    /// Set **LCD_DUMMY_CYCLELEN** `[30:29]`
    ///
    /// The dummy cycle length minus 1.
    #[inline(always)]
    pub const fn with_lcd_dummy_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdUser::LCD_DUMMY_CYCLELEN_MASK)
                | ((value << LcdUser::LCD_DUMMY_CYCLELEN_OFFSET)
                    & LcdUser::LCD_DUMMY_CYCLELEN_MASK),
        }
    }

    /// Set **LCD_RESET** `[28:28]`
    ///
    /// The value of command.
    #[inline(always)]
    pub const fn with_lcd_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_RESET_MASK
            } else {
                self.bits & !LcdUser::LCD_RESET_MASK
            },
        }
    }

    /// Set **LCD_START** `[27:27]`
    ///
    /// LCD start sending data enable signal, valid in high level.
    #[inline(always)]
    pub const fn with_lcd_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_START_MASK
            } else {
                self.bits & !LcdUser::LCD_START_MASK
            },
        }
    }

    /// Set **LCD_CMD** `[26:26]`
    ///
    /// 1: Be able to send command in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn with_lcd_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_CMD_MASK
            } else {
                self.bits & !LcdUser::LCD_CMD_MASK
            },
        }
    }

    /// Set **LCD_DUMMY** `[25:25]`
    ///
    /// 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn with_lcd_dummy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_DUMMY_MASK
            } else {
                self.bits & !LcdUser::LCD_DUMMY_MASK
            },
        }
    }

    /// Set **LCD_DOUT** `[24:24]`
    ///
    /// 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable.
    #[inline(always)]
    pub const fn with_lcd_dout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_DOUT_MASK
            } else {
                self.bits & !LcdUser::LCD_DOUT_MASK
            },
        }
    }

    /// Set **LCD_BYTE_ORDER** `[23:23]`
    ///
    /// 1: invert data byte order, only valid in 2 byte mode. 0: Not change.
    #[inline(always)]
    pub const fn with_lcd_byte_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_BYTE_ORDER_MASK
            } else {
                self.bits & !LcdUser::LCD_BYTE_ORDER_MASK
            },
        }
    }

    /// Set **LCD_BIT_ORDER** `[22:22]`
    ///
    /// 1: Change data bit order, change LCD_DATA_out[7:0] to LCD_DATA_out[0:7] in one byte mode, and bits[15:0] to bits[0:15] in two byte mode. 0: Not change.
    #[inline(always)]
    pub const fn with_lcd_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_BIT_ORDER_MASK
            } else {
                self.bits & !LcdUser::LCD_BIT_ORDER_MASK
            },
        }
    }

    /// Set **LCD_UPDATE** `[21:21]`
    ///
    /// 1: Update LCD registers, will be cleared by hardware. 0 : Not care.
    #[inline(always)]
    pub const fn with_lcd_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_UPDATE_MASK
            } else {
                self.bits & !LcdUser::LCD_UPDATE_MASK
            },
        }
    }

    /// Set **LCD_BYTE_MODE** `[20:19]`
    ///
    /// 2: 24bit mode. 1: 16bit mode. 0: 8bit mode
    #[inline(always)]
    pub const fn with_lcd_byte_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdUser::LCD_BYTE_MODE_MASK)
                | ((value << LcdUser::LCD_BYTE_MODE_OFFSET) & LcdUser::LCD_BYTE_MODE_MASK),
        }
    }

    /// Set **LCD_DOUT_BIT_ORDER** `[18:18]`
    ///
    /// 1: change bit order in every byte. 0: Not change.
    #[inline(always)]
    pub const fn with_lcd_dout_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_DOUT_BIT_ORDER_MASK
            } else {
                self.bits & !LcdUser::LCD_DOUT_BIT_ORDER_MASK
            },
        }
    }

    /// Set **LCD_DOUT_BYTE_SWIZZLE_ENABLE** `[17:17]`
    ///
    /// 1: enable byte swizzle 0: disable
    #[inline(always)]
    pub const fn with_lcd_dout_byte_swizzle_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK
            } else {
                self.bits & !LcdUser::LCD_DOUT_BYTE_SWIZZLE_ENABLE_MASK
            },
        }
    }

    /// Set **LCD_DOUT_BYTE_SWIZZLE_MODE** `[16:14]`
    ///
    /// 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA
    #[inline(always)]
    pub const fn with_lcd_dout_byte_swizzle_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdUser::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK)
                | ((value << LcdUser::LCD_DOUT_BYTE_SWIZZLE_MODE_OFFSET)
                    & LcdUser::LCD_DOUT_BYTE_SWIZZLE_MODE_MASK),
        }
    }

    /// Set **LCD_ALWAYS_OUT_EN** `[13:13]`
    ///
    /// LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set.
    #[inline(always)]
    pub const fn with_lcd_always_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdUser::LCD_ALWAYS_OUT_EN_MASK
            } else {
                self.bits & !LcdUser::LCD_ALWAYS_OUT_EN_MASK
            },
        }
    }

    /// Set **LCD_DOUT_CYCLELEN** `[12:0]`
    ///
    /// The output data cycles minus 1 of LCD module.
    #[inline(always)]
    pub const fn with_lcd_dout_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdUser::LCD_DOUT_CYCLELEN_MASK)
                | ((value << LcdUser::LCD_DOUT_CYCLELEN_OFFSET) & LcdUser::LCD_DOUT_CYCLELEN_MASK),
        }
    }
}

impl Default for LcdUserVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcdUser {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdUserVal {
        LcdUserVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdUserVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdUserVal) -> LcdUserVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_MISC Register
///
/// LCD config register.
///
/// Address offset: 0x0018
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **LCD_CD_IDLE_EDGE** `[31]` (RW)
///
///   The default value of LCD_CD.
///
/// - **LCD_CD_CMD_SET** `[30]` (RW)
///
///   1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge.
///
/// - **LCD_CD_DUMMY_SET** `[29]` (RW)
///
///   1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge.
///
/// - **LCD_CD_DATA_SET** `[28]` (RW)
///
///   1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge.
///
/// - **LCD_AFIFO_RESET** `[27]` (WO)
///
///   LCD AFIFO reset signal.
///
/// - **LCD_BK_EN** `[26]` (RW)
///
///   1: Enable blank region when LCD sends data out. 0: No blank region.
///
/// - **LCD_NEXT_FRAME_EN** `[25]` (RW)
///
///   1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out.
///
/// - **LCD_VBK_CYCLELEN** `[24:12]` (RW)
///
///   The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
///
/// - **LCD_VFK_CYCLELEN** `[11:6]` (RW)
///
///   The setup cycle length minus 1 in LCD non-RGB mode.
///
/// - **LCD_WIRE_MODE** `[5:4]` (RW)
///
///   The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
#[repr(transparent)]
pub struct LcdMisc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdMisc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdMisc {
    // ======== Field Constants ========

    /// **LCD_CD_IDLE_EDGE**
    ///
    /// The default value of LCD_CD.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LCD_CD_IDLE_EDGE_OFFSET: u32 = 31;
    pub const LCD_CD_IDLE_EDGE_MASK: u32 = 0x80000000;
    pub const LCD_CD_IDLE_EDGE: u32 = 1 << 31;

    /// **LCD_CD_CMD_SET**
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LCD_CD_CMD_SET_OFFSET: u32 = 30;
    pub const LCD_CD_CMD_SET_MASK: u32 = 0x40000000;
    pub const LCD_CD_CMD_SET: u32 = 1 << 30;

    /// **LCD_CD_DUMMY_SET**
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LCD_CD_DUMMY_SET_OFFSET: u32 = 29;
    pub const LCD_CD_DUMMY_SET_MASK: u32 = 0x20000000;
    pub const LCD_CD_DUMMY_SET: u32 = 1 << 29;

    /// **LCD_CD_DATA_SET**
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LCD_CD_DATA_SET_OFFSET: u32 = 28;
    pub const LCD_CD_DATA_SET_MASK: u32 = 0x10000000;
    pub const LCD_CD_DATA_SET: u32 = 1 << 28;

    /// **LCD_AFIFO_RESET**
    ///
    /// LCD AFIFO reset signal.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LCD_AFIFO_RESET_OFFSET: u32 = 27;
    pub const LCD_AFIFO_RESET_MASK: u32 = 0x08000000;
    pub const LCD_AFIFO_RESET: u32 = 1 << 27;

    /// **LCD_BK_EN**
    ///
    /// 1: Enable blank region when LCD sends data out. 0: No blank region.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LCD_BK_EN_OFFSET: u32 = 26;
    pub const LCD_BK_EN_MASK: u32 = 0x04000000;
    pub const LCD_BK_EN: u32 = 1 << 26;

    /// **LCD_NEXT_FRAME_EN**
    ///
    /// 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LCD_NEXT_FRAME_EN_OFFSET: u32 = 25;
    pub const LCD_NEXT_FRAME_EN_MASK: u32 = 0x02000000;
    pub const LCD_NEXT_FRAME_EN: u32 = 1 << 25;

    /// **LCD_VBK_CYCLELEN**
    ///
    /// The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
    ///
    /// Bits `[24:12]`, mask `0x01fff000`
    pub const LCD_VBK_CYCLELEN_OFFSET: u32 = 12;
    pub const LCD_VBK_CYCLELEN_MASK: u32 = 0x01fff000;

    /// **LCD_VFK_CYCLELEN**
    ///
    /// The setup cycle length minus 1 in LCD non-RGB mode.
    ///
    /// Bits `[11:6]`, mask `0x00000fc0`
    pub const LCD_VFK_CYCLELEN_OFFSET: u32 = 6;
    pub const LCD_VFK_CYCLELEN_MASK: u32 = 0x00000fc0;

    /// **LCD_WIRE_MODE**
    ///
    /// The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const LCD_WIRE_MODE_OFFSET: u32 = 4;
    pub const LCD_WIRE_MODE_MASK: u32 = 0x00000030;

    // ======== Field Accessors ========

    /// **LCD_CD_IDLE_EDGE** `[31]` (RW)
    ///
    /// The default value of LCD_CD.
    #[inline(always)]
    pub fn lcd_cd_idle_edge(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CD_IDLE_EDGE_MASK) != 0
    }

    /// Set LCD_CD_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cd_idle_edge(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CD_IDLE_EDGE_MASK);
    }

    /// Clear LCD_CD_IDLE_EDGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cd_idle_edge(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CD_IDLE_EDGE_MASK);
    }

    /// **LCD_CD_CMD_SET** `[30]` (RW)
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub fn lcd_cd_cmd_set(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CD_CMD_SET_MASK) != 0
    }

    /// Set LCD_CD_CMD_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cd_cmd_set(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CD_CMD_SET_MASK);
    }

    /// Clear LCD_CD_CMD_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cd_cmd_set(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CD_CMD_SET_MASK);
    }

    /// **LCD_CD_DUMMY_SET** `[29]` (RW)
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub fn lcd_cd_dummy_set(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CD_DUMMY_SET_MASK) != 0
    }

    /// Set LCD_CD_DUMMY_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cd_dummy_set(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CD_DUMMY_SET_MASK);
    }

    /// Clear LCD_CD_DUMMY_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cd_dummy_set(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CD_DUMMY_SET_MASK);
    }

    /// **LCD_CD_DATA_SET** `[28]` (RW)
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub fn lcd_cd_data_set(&self) -> bool {
        (self.value.read_bits() & Self::LCD_CD_DATA_SET_MASK) != 0
    }

    /// Set LCD_CD_DATA_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_cd_data_set(&self) {
        self.value.modify_bits(|r| r | Self::LCD_CD_DATA_SET_MASK);
    }

    /// Clear LCD_CD_DATA_SET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_cd_data_set(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_CD_DATA_SET_MASK);
    }

    /// Trigger **LCD_AFIFO_RESET** (WO)
    ///
    /// LCD AFIFO reset signal.
    #[inline(always)]
    pub fn trigger_lcd_afifo_reset(&self) {
        self.value.modify_bits(|r| r | Self::LCD_AFIFO_RESET_MASK);
    }

    /// **LCD_BK_EN** `[26]` (RW)
    ///
    /// 1: Enable blank region when LCD sends data out. 0: No blank region.
    #[inline(always)]
    pub fn lcd_bk_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_BK_EN_MASK) != 0
    }

    /// Set LCD_BK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_bk_en(&self) {
        self.value.modify_bits(|r| r | Self::LCD_BK_EN_MASK);
    }

    /// Clear LCD_BK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_bk_en(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_BK_EN_MASK);
    }

    /// **LCD_NEXT_FRAME_EN** `[25]` (RW)
    ///
    /// 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out.
    #[inline(always)]
    pub fn lcd_next_frame_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_NEXT_FRAME_EN_MASK) != 0
    }

    /// Set LCD_NEXT_FRAME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_next_frame_en(&self) {
        self.value.modify_bits(|r| r | Self::LCD_NEXT_FRAME_EN_MASK);
    }

    /// Clear LCD_NEXT_FRAME_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_next_frame_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_NEXT_FRAME_EN_MASK);
    }

    /// **LCD_VBK_CYCLELEN** `[24:12]` (RW)
    ///
    /// The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
    #[inline(always)]
    pub fn lcd_vbk_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VBK_CYCLELEN_MASK) >> Self::LCD_VBK_CYCLELEN_OFFSET
    }

    /// Write **LCD_VBK_CYCLELEN** `[24:12]` (RW)
    ///
    /// The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
    #[inline(always)]
    pub fn write_lcd_vbk_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VBK_CYCLELEN_MASK)
                | ((value << Self::LCD_VBK_CYCLELEN_OFFSET) & Self::LCD_VBK_CYCLELEN_MASK)
        });
    }

    /// **LCD_VFK_CYCLELEN** `[11:6]` (RW)
    ///
    /// The setup cycle length minus 1 in LCD non-RGB mode.
    #[inline(always)]
    pub fn lcd_vfk_cyclelen(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VFK_CYCLELEN_MASK) >> Self::LCD_VFK_CYCLELEN_OFFSET
    }

    /// Write **LCD_VFK_CYCLELEN** `[11:6]` (RW)
    ///
    /// The setup cycle length minus 1 in LCD non-RGB mode.
    #[inline(always)]
    pub fn write_lcd_vfk_cyclelen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VFK_CYCLELEN_MASK)
                | ((value << Self::LCD_VFK_CYCLELEN_OFFSET) & Self::LCD_VFK_CYCLELEN_MASK)
        });
    }

    /// **LCD_WIRE_MODE** `[5:4]` (RW)
    ///
    /// The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
    #[inline(always)]
    pub fn lcd_wire_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_WIRE_MODE_MASK) >> Self::LCD_WIRE_MODE_OFFSET
    }

    /// Write **LCD_WIRE_MODE** `[5:4]` (RW)
    ///
    /// The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
    #[inline(always)]
    pub fn write_lcd_wire_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_WIRE_MODE_MASK)
                | ((value << Self::LCD_WIRE_MODE_OFFSET) & Self::LCD_WIRE_MODE_MASK)
        });
    }
}

/// LCD_MISC value type for batch read/modify/write operations
///
/// Use `lcdmisc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdmisc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdMiscVal {
    bits: u32,
}

impl LcdMiscVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c0 }
    }

    /// Read **LCD_CD_IDLE_EDGE** `[31:31]`
    ///
    /// The default value of LCD_CD.
    #[inline(always)]
    pub const fn lcd_cd_idle_edge(&self) -> bool {
        (self.bits & LcdMisc::LCD_CD_IDLE_EDGE_MASK) != 0
    }

    /// Read **LCD_CD_CMD_SET** `[30:30]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn lcd_cd_cmd_set(&self) -> bool {
        (self.bits & LcdMisc::LCD_CD_CMD_SET_MASK) != 0
    }

    /// Read **LCD_CD_DUMMY_SET** `[29:29]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn lcd_cd_dummy_set(&self) -> bool {
        (self.bits & LcdMisc::LCD_CD_DUMMY_SET_MASK) != 0
    }

    /// Read **LCD_CD_DATA_SET** `[28:28]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn lcd_cd_data_set(&self) -> bool {
        (self.bits & LcdMisc::LCD_CD_DATA_SET_MASK) != 0
    }

    /// Read **LCD_BK_EN** `[26:26]`
    ///
    /// 1: Enable blank region when LCD sends data out. 0: No blank region.
    #[inline(always)]
    pub const fn lcd_bk_en(&self) -> bool {
        (self.bits & LcdMisc::LCD_BK_EN_MASK) != 0
    }

    /// Read **LCD_NEXT_FRAME_EN** `[25:25]`
    ///
    /// 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out.
    #[inline(always)]
    pub const fn lcd_next_frame_en(&self) -> bool {
        (self.bits & LcdMisc::LCD_NEXT_FRAME_EN_MASK) != 0
    }

    /// Read **LCD_VBK_CYCLELEN** `[24:12]`
    ///
    /// The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
    #[inline(always)]
    pub const fn lcd_vbk_cyclelen(&self) -> u32 {
        (self.bits & LcdMisc::LCD_VBK_CYCLELEN_MASK) >> LcdMisc::LCD_VBK_CYCLELEN_OFFSET
    }

    /// Read **LCD_VFK_CYCLELEN** `[11:6]`
    ///
    /// The setup cycle length minus 1 in LCD non-RGB mode.
    #[inline(always)]
    pub const fn lcd_vfk_cyclelen(&self) -> u32 {
        (self.bits & LcdMisc::LCD_VFK_CYCLELEN_MASK) >> LcdMisc::LCD_VFK_CYCLELEN_OFFSET
    }

    /// Read **LCD_WIRE_MODE** `[5:4]`
    ///
    /// The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
    #[inline(always)]
    pub const fn lcd_wire_mode(&self) -> u32 {
        (self.bits & LcdMisc::LCD_WIRE_MODE_MASK) >> LcdMisc::LCD_WIRE_MODE_OFFSET
    }

    /// Set **LCD_CD_IDLE_EDGE** `[31:31]`
    ///
    /// The default value of LCD_CD.
    #[inline(always)]
    pub const fn with_lcd_cd_idle_edge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_CD_IDLE_EDGE_MASK
            } else {
                self.bits & !LcdMisc::LCD_CD_IDLE_EDGE_MASK
            },
        }
    }

    /// Set **LCD_CD_CMD_SET** `[30:30]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn with_lcd_cd_cmd_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_CD_CMD_SET_MASK
            } else {
                self.bits & !LcdMisc::LCD_CD_CMD_SET_MASK
            },
        }
    }

    /// Set **LCD_CD_DUMMY_SET** `[29:29]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn with_lcd_cd_dummy_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_CD_DUMMY_SET_MASK
            } else {
                self.bits & !LcdMisc::LCD_CD_DUMMY_SET_MASK
            },
        }
    }

    /// Set **LCD_CD_DATA_SET** `[28:28]`
    ///
    /// 1: LCD_CD = !reg_cd_idle_edge when lcd_st[2:0] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge.
    #[inline(always)]
    pub const fn with_lcd_cd_data_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_CD_DATA_SET_MASK
            } else {
                self.bits & !LcdMisc::LCD_CD_DATA_SET_MASK
            },
        }
    }

    /// Set **LCD_AFIFO_RESET** `[27:27]`
    ///
    /// LCD AFIFO reset signal.
    #[inline(always)]
    pub const fn with_lcd_afifo_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_AFIFO_RESET_MASK
            } else {
                self.bits & !LcdMisc::LCD_AFIFO_RESET_MASK
            },
        }
    }

    /// Set **LCD_BK_EN** `[26:26]`
    ///
    /// 1: Enable blank region when LCD sends data out. 0: No blank region.
    #[inline(always)]
    pub const fn with_lcd_bk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_BK_EN_MASK
            } else {
                self.bits & !LcdMisc::LCD_BK_EN_MASK
            },
        }
    }

    /// Set **LCD_NEXT_FRAME_EN** `[25:25]`
    ///
    /// 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out.
    #[inline(always)]
    pub const fn with_lcd_next_frame_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdMisc::LCD_NEXT_FRAME_EN_MASK
            } else {
                self.bits & !LcdMisc::LCD_NEXT_FRAME_EN_MASK
            },
        }
    }

    /// Set **LCD_VBK_CYCLELEN** `[24:12]`
    ///
    /// The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode.
    #[inline(always)]
    pub const fn with_lcd_vbk_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdMisc::LCD_VBK_CYCLELEN_MASK)
                | ((value << LcdMisc::LCD_VBK_CYCLELEN_OFFSET) & LcdMisc::LCD_VBK_CYCLELEN_MASK),
        }
    }

    /// Set **LCD_VFK_CYCLELEN** `[11:6]`
    ///
    /// The setup cycle length minus 1 in LCD non-RGB mode.
    #[inline(always)]
    pub const fn with_lcd_vfk_cyclelen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdMisc::LCD_VFK_CYCLELEN_MASK)
                | ((value << LcdMisc::LCD_VFK_CYCLELEN_OFFSET) & LcdMisc::LCD_VFK_CYCLELEN_MASK),
        }
    }

    /// Set **LCD_WIRE_MODE** `[5:4]`
    ///
    /// The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit
    #[inline(always)]
    pub const fn with_lcd_wire_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdMisc::LCD_WIRE_MODE_MASK)
                | ((value << LcdMisc::LCD_WIRE_MODE_OFFSET) & LcdMisc::LCD_WIRE_MODE_MASK),
        }
    }
}

impl Default for LcdMiscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcdMisc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdMiscVal {
        LcdMiscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdMiscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdMiscVal) -> LcdMiscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_CTRL Register
///
/// LCD config register.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **LCD_RGB_MODE_EN** `[31]` (RW)
///
///   1: Enable LCD RGB mode. 0: Disable LCD RGB mode.
///
/// - **LCD_VT_HEIGHT** `[30:21]` (RW)
///
///   It is the vertical total height of a frame.
///
/// - **LCD_VA_HEIGHT** `[20:11]` (RW)
///
///   It is the vertical active height of a frame.
///
/// - **LCD_HB_FRONT** `[10:0]` (RW)
///
///   It is the horizontal blank front porch of a frame.
#[repr(transparent)]
pub struct LcdCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdCtrl {
    // ======== Field Constants ========

    /// **LCD_RGB_MODE_EN**
    ///
    /// 1: Enable LCD RGB mode. 0: Disable LCD RGB mode.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LCD_RGB_MODE_EN_OFFSET: u32 = 31;
    pub const LCD_RGB_MODE_EN_MASK: u32 = 0x80000000;
    pub const LCD_RGB_MODE_EN: u32 = 1 << 31;

    /// **LCD_VT_HEIGHT**
    ///
    /// It is the vertical total height of a frame.
    ///
    /// Bits `[30:21]`, mask `0x7fe00000`
    pub const LCD_VT_HEIGHT_OFFSET: u32 = 21;
    pub const LCD_VT_HEIGHT_MASK: u32 = 0x7fe00000;

    /// **LCD_VA_HEIGHT**
    ///
    /// It is the vertical active height of a frame.
    ///
    /// Bits `[20:11]`, mask `0x001ff800`
    pub const LCD_VA_HEIGHT_OFFSET: u32 = 11;
    pub const LCD_VA_HEIGHT_MASK: u32 = 0x001ff800;

    /// **LCD_HB_FRONT**
    ///
    /// It is the horizontal blank front porch of a frame.
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const LCD_HB_FRONT_OFFSET: u32 = 0;
    pub const LCD_HB_FRONT_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **LCD_RGB_MODE_EN** `[31]` (RW)
    ///
    /// 1: Enable LCD RGB mode. 0: Disable LCD RGB mode.
    #[inline(always)]
    pub fn lcd_rgb_mode_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_RGB_MODE_EN_MASK) != 0
    }

    /// Set LCD_RGB_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_rgb_mode_en(&self) {
        self.value.modify_bits(|r| r | Self::LCD_RGB_MODE_EN_MASK);
    }

    /// Clear LCD_RGB_MODE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_rgb_mode_en(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_RGB_MODE_EN_MASK);
    }

    /// **LCD_VT_HEIGHT** `[30:21]` (RW)
    ///
    /// It is the vertical total height of a frame.
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VT_HEIGHT_MASK) >> Self::LCD_VT_HEIGHT_OFFSET
    }

    /// Write **LCD_VT_HEIGHT** `[30:21]` (RW)
    ///
    /// It is the vertical total height of a frame.
    #[inline(always)]
    pub fn write_lcd_vt_height(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VT_HEIGHT_MASK)
                | ((value << Self::LCD_VT_HEIGHT_OFFSET) & Self::LCD_VT_HEIGHT_MASK)
        });
    }

    /// **LCD_VA_HEIGHT** `[20:11]` (RW)
    ///
    /// It is the vertical active height of a frame.
    #[inline(always)]
    pub fn lcd_va_height(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VA_HEIGHT_MASK) >> Self::LCD_VA_HEIGHT_OFFSET
    }

    /// Write **LCD_VA_HEIGHT** `[20:11]` (RW)
    ///
    /// It is the vertical active height of a frame.
    #[inline(always)]
    pub fn write_lcd_va_height(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VA_HEIGHT_MASK)
                | ((value << Self::LCD_VA_HEIGHT_OFFSET) & Self::LCD_VA_HEIGHT_MASK)
        });
    }

    /// **LCD_HB_FRONT** `[10:0]` (RW)
    ///
    /// It is the horizontal blank front porch of a frame.
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HB_FRONT_MASK) >> Self::LCD_HB_FRONT_OFFSET
    }

    /// Write **LCD_HB_FRONT** `[10:0]` (RW)
    ///
    /// It is the horizontal blank front porch of a frame.
    #[inline(always)]
    pub fn write_lcd_hb_front(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HB_FRONT_MASK)
                | ((value << Self::LCD_HB_FRONT_OFFSET) & Self::LCD_HB_FRONT_MASK)
        });
    }
}

/// LCD_CTRL value type for batch read/modify/write operations
///
/// Use `lcdctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdCtrlVal {
    bits: u32,
}

impl LcdCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LCD_RGB_MODE_EN** `[31:31]`
    ///
    /// 1: Enable LCD RGB mode. 0: Disable LCD RGB mode.
    #[inline(always)]
    pub const fn lcd_rgb_mode_en(&self) -> bool {
        (self.bits & LcdCtrl::LCD_RGB_MODE_EN_MASK) != 0
    }

    /// Read **LCD_VT_HEIGHT** `[30:21]`
    ///
    /// It is the vertical total height of a frame.
    #[inline(always)]
    pub const fn lcd_vt_height(&self) -> u32 {
        (self.bits & LcdCtrl::LCD_VT_HEIGHT_MASK) >> LcdCtrl::LCD_VT_HEIGHT_OFFSET
    }

    /// Read **LCD_VA_HEIGHT** `[20:11]`
    ///
    /// It is the vertical active height of a frame.
    #[inline(always)]
    pub const fn lcd_va_height(&self) -> u32 {
        (self.bits & LcdCtrl::LCD_VA_HEIGHT_MASK) >> LcdCtrl::LCD_VA_HEIGHT_OFFSET
    }

    /// Read **LCD_HB_FRONT** `[10:0]`
    ///
    /// It is the horizontal blank front porch of a frame.
    #[inline(always)]
    pub const fn lcd_hb_front(&self) -> u32 {
        (self.bits & LcdCtrl::LCD_HB_FRONT_MASK) >> LcdCtrl::LCD_HB_FRONT_OFFSET
    }

    /// Set **LCD_RGB_MODE_EN** `[31:31]`
    ///
    /// 1: Enable LCD RGB mode. 0: Disable LCD RGB mode.
    #[inline(always)]
    pub const fn with_lcd_rgb_mode_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdCtrl::LCD_RGB_MODE_EN_MASK
            } else {
                self.bits & !LcdCtrl::LCD_RGB_MODE_EN_MASK
            },
        }
    }

    /// Set **LCD_VT_HEIGHT** `[30:21]`
    ///
    /// It is the vertical total height of a frame.
    #[inline(always)]
    pub const fn with_lcd_vt_height(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl::LCD_VT_HEIGHT_MASK)
                | ((value << LcdCtrl::LCD_VT_HEIGHT_OFFSET) & LcdCtrl::LCD_VT_HEIGHT_MASK),
        }
    }

    /// Set **LCD_VA_HEIGHT** `[20:11]`
    ///
    /// It is the vertical active height of a frame.
    #[inline(always)]
    pub const fn with_lcd_va_height(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl::LCD_VA_HEIGHT_MASK)
                | ((value << LcdCtrl::LCD_VA_HEIGHT_OFFSET) & LcdCtrl::LCD_VA_HEIGHT_MASK),
        }
    }

    /// Set **LCD_HB_FRONT** `[10:0]`
    ///
    /// It is the horizontal blank front porch of a frame.
    #[inline(always)]
    pub const fn with_lcd_hb_front(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl::LCD_HB_FRONT_MASK)
                | ((value << LcdCtrl::LCD_HB_FRONT_OFFSET) & LcdCtrl::LCD_HB_FRONT_MASK),
        }
    }
}

impl Default for LcdCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdCtrlVal {
        LcdCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdCtrlVal) -> LcdCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_CTRL1 Register
///
/// LCD config register.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **LCD_HT_WIDTH** `[31:20]` (RW)
///
///   It is the horizontal total width of a frame.
///
/// - **LCD_HA_WIDTH** `[19:8]` (RW)
///
///   It is the horizontal active width of a frame.
///
/// - **LCD_VB_FRONT** `[7:0]` (RW)
///
///   It is the vertical blank front porch of a frame.
#[repr(transparent)]
pub struct LcdCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdCtrl1 {
    // ======== Field Constants ========

    /// **LCD_HT_WIDTH**
    ///
    /// It is the horizontal total width of a frame.
    ///
    /// Bits `[31:20]`, mask `0xfff00000`
    pub const LCD_HT_WIDTH_OFFSET: u32 = 20;
    pub const LCD_HT_WIDTH_MASK: u32 = 0xfff00000;

    /// **LCD_HA_WIDTH**
    ///
    /// It is the horizontal active width of a frame.
    ///
    /// Bits `[19:8]`, mask `0x000fff00`
    pub const LCD_HA_WIDTH_OFFSET: u32 = 8;
    pub const LCD_HA_WIDTH_MASK: u32 = 0x000fff00;

    /// **LCD_VB_FRONT**
    ///
    /// It is the vertical blank front porch of a frame.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const LCD_VB_FRONT_OFFSET: u32 = 0;
    pub const LCD_VB_FRONT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **LCD_HT_WIDTH** `[31:20]` (RW)
    ///
    /// It is the horizontal total width of a frame.
    #[inline(always)]
    pub fn lcd_ht_width(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HT_WIDTH_MASK) >> Self::LCD_HT_WIDTH_OFFSET
    }

    /// Write **LCD_HT_WIDTH** `[31:20]` (RW)
    ///
    /// It is the horizontal total width of a frame.
    #[inline(always)]
    pub fn write_lcd_ht_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HT_WIDTH_MASK)
                | ((value << Self::LCD_HT_WIDTH_OFFSET) & Self::LCD_HT_WIDTH_MASK)
        });
    }

    /// **LCD_HA_WIDTH** `[19:8]` (RW)
    ///
    /// It is the horizontal active width of a frame.
    #[inline(always)]
    pub fn lcd_ha_width(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HA_WIDTH_MASK) >> Self::LCD_HA_WIDTH_OFFSET
    }

    /// Write **LCD_HA_WIDTH** `[19:8]` (RW)
    ///
    /// It is the horizontal active width of a frame.
    #[inline(always)]
    pub fn write_lcd_ha_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HA_WIDTH_MASK)
                | ((value << Self::LCD_HA_WIDTH_OFFSET) & Self::LCD_HA_WIDTH_MASK)
        });
    }

    /// **LCD_VB_FRONT** `[7:0]` (RW)
    ///
    /// It is the vertical blank front porch of a frame.
    #[inline(always)]
    pub fn lcd_vb_front(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VB_FRONT_MASK) >> Self::LCD_VB_FRONT_OFFSET
    }

    /// Write **LCD_VB_FRONT** `[7:0]` (RW)
    ///
    /// It is the vertical blank front porch of a frame.
    #[inline(always)]
    pub fn write_lcd_vb_front(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VB_FRONT_MASK)
                | ((value << Self::LCD_VB_FRONT_OFFSET) & Self::LCD_VB_FRONT_MASK)
        });
    }
}

/// LCD_CTRL1 value type for batch read/modify/write operations
///
/// Use `lcdctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdCtrl1Val {
    bits: u32,
}

impl LcdCtrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LCD_HT_WIDTH** `[31:20]`
    ///
    /// It is the horizontal total width of a frame.
    #[inline(always)]
    pub const fn lcd_ht_width(&self) -> u32 {
        (self.bits & LcdCtrl1::LCD_HT_WIDTH_MASK) >> LcdCtrl1::LCD_HT_WIDTH_OFFSET
    }

    /// Read **LCD_HA_WIDTH** `[19:8]`
    ///
    /// It is the horizontal active width of a frame.
    #[inline(always)]
    pub const fn lcd_ha_width(&self) -> u32 {
        (self.bits & LcdCtrl1::LCD_HA_WIDTH_MASK) >> LcdCtrl1::LCD_HA_WIDTH_OFFSET
    }

    /// Read **LCD_VB_FRONT** `[7:0]`
    ///
    /// It is the vertical blank front porch of a frame.
    #[inline(always)]
    pub const fn lcd_vb_front(&self) -> u32 {
        (self.bits & LcdCtrl1::LCD_VB_FRONT_MASK) >> LcdCtrl1::LCD_VB_FRONT_OFFSET
    }

    /// Set **LCD_HT_WIDTH** `[31:20]`
    ///
    /// It is the horizontal total width of a frame.
    #[inline(always)]
    pub const fn with_lcd_ht_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl1::LCD_HT_WIDTH_MASK)
                | ((value << LcdCtrl1::LCD_HT_WIDTH_OFFSET) & LcdCtrl1::LCD_HT_WIDTH_MASK),
        }
    }

    /// Set **LCD_HA_WIDTH** `[19:8]`
    ///
    /// It is the horizontal active width of a frame.
    #[inline(always)]
    pub const fn with_lcd_ha_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl1::LCD_HA_WIDTH_MASK)
                | ((value << LcdCtrl1::LCD_HA_WIDTH_OFFSET) & LcdCtrl1::LCD_HA_WIDTH_MASK),
        }
    }

    /// Set **LCD_VB_FRONT** `[7:0]`
    ///
    /// It is the vertical blank front porch of a frame.
    #[inline(always)]
    pub const fn with_lcd_vb_front(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl1::LCD_VB_FRONT_MASK)
                | ((value << LcdCtrl1::LCD_VB_FRONT_OFFSET) & LcdCtrl1::LCD_VB_FRONT_MASK),
        }
    }
}

impl Default for LcdCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdCtrl1Val {
        LcdCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdCtrl1Val) -> LcdCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_CTRL2 Register
///
/// LCD config register.
///
/// Address offset: 0x0024
/// Reset value: 0x00010001
///
/// ## Fields
///
/// - **LCD_HSYNC_POSITION** `[31:24]` (RW)
///
///   It is the position of LCD_HSYNC active pulse in a line.
///
/// - **LCD_HSYNC_IDLE_POL** `[23]` (RW)
///
///   It is the idle value of LCD_HSYNC.
///
/// - **LCD_HSYNC_WIDTH** `[22:16]` (RW)
///
///   It is the position of LCD_HSYNC active pulse in a line.
///
/// - **LCD_HS_BLANK_EN** `[9]` (RW)
///
///   1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode.
///
/// - **LCD_DE_IDLE_POL** `[8]` (RW)
///
///   It is the idle value of LCD_DE.
///
/// - **LCD_VSYNC_IDLE_POL** `[7]` (RW)
///
///   It is the idle value of LCD_VSYNC.
///
/// - **LCD_VSYNC_WIDTH** `[6:0]` (RW)
///
///   It is the position of LCD_VSYNC active pulse in a line.
#[repr(transparent)]
pub struct LcdCtrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdCtrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdCtrl2 {
    // ======== Field Constants ========

    /// **LCD_HSYNC_POSITION**
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LCD_HSYNC_POSITION_OFFSET: u32 = 24;
    pub const LCD_HSYNC_POSITION_MASK: u32 = 0xff000000;

    /// **LCD_HSYNC_IDLE_POL**
    ///
    /// It is the idle value of LCD_HSYNC.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const LCD_HSYNC_IDLE_POL_OFFSET: u32 = 23;
    pub const LCD_HSYNC_IDLE_POL_MASK: u32 = 0x00800000;
    pub const LCD_HSYNC_IDLE_POL: u32 = 1 << 23;

    /// **LCD_HSYNC_WIDTH**
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    ///
    /// Bits `[22:16]`, mask `0x007f0000`
    pub const LCD_HSYNC_WIDTH_OFFSET: u32 = 16;
    pub const LCD_HSYNC_WIDTH_MASK: u32 = 0x007f0000;

    /// **LCD_HS_BLANK_EN**
    ///
    /// 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LCD_HS_BLANK_EN_OFFSET: u32 = 9;
    pub const LCD_HS_BLANK_EN_MASK: u32 = 0x00000200;
    pub const LCD_HS_BLANK_EN: u32 = 1 << 9;

    /// **LCD_DE_IDLE_POL**
    ///
    /// It is the idle value of LCD_DE.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LCD_DE_IDLE_POL_OFFSET: u32 = 8;
    pub const LCD_DE_IDLE_POL_MASK: u32 = 0x00000100;
    pub const LCD_DE_IDLE_POL: u32 = 1 << 8;

    /// **LCD_VSYNC_IDLE_POL**
    ///
    /// It is the idle value of LCD_VSYNC.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LCD_VSYNC_IDLE_POL_OFFSET: u32 = 7;
    pub const LCD_VSYNC_IDLE_POL_MASK: u32 = 0x00000080;
    pub const LCD_VSYNC_IDLE_POL: u32 = 1 << 7;

    /// **LCD_VSYNC_WIDTH**
    ///
    /// It is the position of LCD_VSYNC active pulse in a line.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const LCD_VSYNC_WIDTH_OFFSET: u32 = 0;
    pub const LCD_VSYNC_WIDTH_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **LCD_HSYNC_POSITION** `[31:24]` (RW)
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub fn lcd_hsync_position(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HSYNC_POSITION_MASK) >> Self::LCD_HSYNC_POSITION_OFFSET
    }

    /// Write **LCD_HSYNC_POSITION** `[31:24]` (RW)
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub fn write_lcd_hsync_position(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HSYNC_POSITION_MASK)
                | ((value << Self::LCD_HSYNC_POSITION_OFFSET) & Self::LCD_HSYNC_POSITION_MASK)
        });
    }

    /// **LCD_HSYNC_IDLE_POL** `[23]` (RW)
    ///
    /// It is the idle value of LCD_HSYNC.
    #[inline(always)]
    pub fn lcd_hsync_idle_pol(&self) -> bool {
        (self.value.read_bits() & Self::LCD_HSYNC_IDLE_POL_MASK) != 0
    }

    /// Set LCD_HSYNC_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_hsync_idle_pol(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_HSYNC_IDLE_POL_MASK);
    }

    /// Clear LCD_HSYNC_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_hsync_idle_pol(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_HSYNC_IDLE_POL_MASK);
    }

    /// **LCD_HSYNC_WIDTH** `[22:16]` (RW)
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub fn lcd_hsync_width(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HSYNC_WIDTH_MASK) >> Self::LCD_HSYNC_WIDTH_OFFSET
    }

    /// Write **LCD_HSYNC_WIDTH** `[22:16]` (RW)
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub fn write_lcd_hsync_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HSYNC_WIDTH_MASK)
                | ((value << Self::LCD_HSYNC_WIDTH_OFFSET) & Self::LCD_HSYNC_WIDTH_MASK)
        });
    }

    /// **LCD_HS_BLANK_EN** `[9]` (RW)
    ///
    /// 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode.
    #[inline(always)]
    pub fn lcd_hs_blank_en(&self) -> bool {
        (self.value.read_bits() & Self::LCD_HS_BLANK_EN_MASK) != 0
    }

    /// Set LCD_HS_BLANK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_hs_blank_en(&self) {
        self.value.modify_bits(|r| r | Self::LCD_HS_BLANK_EN_MASK);
    }

    /// Clear LCD_HS_BLANK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_hs_blank_en(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_HS_BLANK_EN_MASK);
    }

    /// **LCD_DE_IDLE_POL** `[8]` (RW)
    ///
    /// It is the idle value of LCD_DE.
    #[inline(always)]
    pub fn lcd_de_idle_pol(&self) -> bool {
        (self.value.read_bits() & Self::LCD_DE_IDLE_POL_MASK) != 0
    }

    /// Set LCD_DE_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_de_idle_pol(&self) {
        self.value.modify_bits(|r| r | Self::LCD_DE_IDLE_POL_MASK);
    }

    /// Clear LCD_DE_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_de_idle_pol(&self) {
        self.value.modify_bits(|r| r & !Self::LCD_DE_IDLE_POL_MASK);
    }

    /// **LCD_VSYNC_IDLE_POL** `[7]` (RW)
    ///
    /// It is the idle value of LCD_VSYNC.
    #[inline(always)]
    pub fn lcd_vsync_idle_pol(&self) -> bool {
        (self.value.read_bits() & Self::LCD_VSYNC_IDLE_POL_MASK) != 0
    }

    /// Set LCD_VSYNC_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_vsync_idle_pol(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_VSYNC_IDLE_POL_MASK);
    }

    /// Clear LCD_VSYNC_IDLE_POL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_vsync_idle_pol(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_VSYNC_IDLE_POL_MASK);
    }

    /// **LCD_VSYNC_WIDTH** `[6:0]` (RW)
    ///
    /// It is the position of LCD_VSYNC active pulse in a line.
    #[inline(always)]
    pub fn lcd_vsync_width(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VSYNC_WIDTH_MASK) >> Self::LCD_VSYNC_WIDTH_OFFSET
    }

    /// Write **LCD_VSYNC_WIDTH** `[6:0]` (RW)
    ///
    /// It is the position of LCD_VSYNC active pulse in a line.
    #[inline(always)]
    pub fn write_lcd_vsync_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VSYNC_WIDTH_MASK)
                | ((value << Self::LCD_VSYNC_WIDTH_OFFSET) & Self::LCD_VSYNC_WIDTH_MASK)
        });
    }
}

/// LCD_CTRL2 value type for batch read/modify/write operations
///
/// Use `lcdctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdCtrl2Val {
    bits: u32,
}

impl LcdCtrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x00010001 }
    }

    /// Read **LCD_HSYNC_POSITION** `[31:24]`
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub const fn lcd_hsync_position(&self) -> u32 {
        (self.bits & LcdCtrl2::LCD_HSYNC_POSITION_MASK) >> LcdCtrl2::LCD_HSYNC_POSITION_OFFSET
    }

    /// Read **LCD_HSYNC_IDLE_POL** `[23:23]`
    ///
    /// It is the idle value of LCD_HSYNC.
    #[inline(always)]
    pub const fn lcd_hsync_idle_pol(&self) -> bool {
        (self.bits & LcdCtrl2::LCD_HSYNC_IDLE_POL_MASK) != 0
    }

    /// Read **LCD_HSYNC_WIDTH** `[22:16]`
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub const fn lcd_hsync_width(&self) -> u32 {
        (self.bits & LcdCtrl2::LCD_HSYNC_WIDTH_MASK) >> LcdCtrl2::LCD_HSYNC_WIDTH_OFFSET
    }

    /// Read **LCD_HS_BLANK_EN** `[9:9]`
    ///
    /// 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode.
    #[inline(always)]
    pub const fn lcd_hs_blank_en(&self) -> bool {
        (self.bits & LcdCtrl2::LCD_HS_BLANK_EN_MASK) != 0
    }

    /// Read **LCD_DE_IDLE_POL** `[8:8]`
    ///
    /// It is the idle value of LCD_DE.
    #[inline(always)]
    pub const fn lcd_de_idle_pol(&self) -> bool {
        (self.bits & LcdCtrl2::LCD_DE_IDLE_POL_MASK) != 0
    }

    /// Read **LCD_VSYNC_IDLE_POL** `[7:7]`
    ///
    /// It is the idle value of LCD_VSYNC.
    #[inline(always)]
    pub const fn lcd_vsync_idle_pol(&self) -> bool {
        (self.bits & LcdCtrl2::LCD_VSYNC_IDLE_POL_MASK) != 0
    }

    /// Read **LCD_VSYNC_WIDTH** `[6:0]`
    ///
    /// It is the position of LCD_VSYNC active pulse in a line.
    #[inline(always)]
    pub const fn lcd_vsync_width(&self) -> u32 {
        (self.bits & LcdCtrl2::LCD_VSYNC_WIDTH_MASK) >> LcdCtrl2::LCD_VSYNC_WIDTH_OFFSET
    }

    /// Set **LCD_HSYNC_POSITION** `[31:24]`
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub const fn with_lcd_hsync_position(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl2::LCD_HSYNC_POSITION_MASK)
                | ((value << LcdCtrl2::LCD_HSYNC_POSITION_OFFSET)
                    & LcdCtrl2::LCD_HSYNC_POSITION_MASK),
        }
    }

    /// Set **LCD_HSYNC_IDLE_POL** `[23:23]`
    ///
    /// It is the idle value of LCD_HSYNC.
    #[inline(always)]
    pub const fn with_lcd_hsync_idle_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdCtrl2::LCD_HSYNC_IDLE_POL_MASK
            } else {
                self.bits & !LcdCtrl2::LCD_HSYNC_IDLE_POL_MASK
            },
        }
    }

    /// Set **LCD_HSYNC_WIDTH** `[22:16]`
    ///
    /// It is the position of LCD_HSYNC active pulse in a line.
    #[inline(always)]
    pub const fn with_lcd_hsync_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl2::LCD_HSYNC_WIDTH_MASK)
                | ((value << LcdCtrl2::LCD_HSYNC_WIDTH_OFFSET) & LcdCtrl2::LCD_HSYNC_WIDTH_MASK),
        }
    }

    /// Set **LCD_HS_BLANK_EN** `[9:9]`
    ///
    /// 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode.
    #[inline(always)]
    pub const fn with_lcd_hs_blank_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdCtrl2::LCD_HS_BLANK_EN_MASK
            } else {
                self.bits & !LcdCtrl2::LCD_HS_BLANK_EN_MASK
            },
        }
    }

    /// Set **LCD_DE_IDLE_POL** `[8:8]`
    ///
    /// It is the idle value of LCD_DE.
    #[inline(always)]
    pub const fn with_lcd_de_idle_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdCtrl2::LCD_DE_IDLE_POL_MASK
            } else {
                self.bits & !LcdCtrl2::LCD_DE_IDLE_POL_MASK
            },
        }
    }

    /// Set **LCD_VSYNC_IDLE_POL** `[7:7]`
    ///
    /// It is the idle value of LCD_VSYNC.
    #[inline(always)]
    pub const fn with_lcd_vsync_idle_pol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcdCtrl2::LCD_VSYNC_IDLE_POL_MASK
            } else {
                self.bits & !LcdCtrl2::LCD_VSYNC_IDLE_POL_MASK
            },
        }
    }

    /// Set **LCD_VSYNC_WIDTH** `[6:0]`
    ///
    /// It is the position of LCD_VSYNC active pulse in a line.
    #[inline(always)]
    pub const fn with_lcd_vsync_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCtrl2::LCD_VSYNC_WIDTH_MASK)
                | ((value << LcdCtrl2::LCD_VSYNC_WIDTH_OFFSET) & LcdCtrl2::LCD_VSYNC_WIDTH_MASK),
        }
    }
}

impl Default for LcdCtrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcdCtrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdCtrl2Val {
        LcdCtrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdCtrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdCtrl2Val) -> LcdCtrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_FIRST_CMD_VAL Register
///
/// LCD config register.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **LCD_FIRST_CMD_VALUE** `[31:0]` (RW)
///
///   The LCD write command value of first cmd cycle.
#[repr(transparent)]
pub struct LcdFirstCmdVal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdFirstCmdVal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdFirstCmdVal {
    // ======== Field Constants ========

    /// **LCD_FIRST_CMD_VALUE**
    ///
    /// The LCD write command value of first cmd cycle.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LCD_FIRST_CMD_VALUE_OFFSET: u32 = 0;
    pub const LCD_FIRST_CMD_VALUE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LCD_FIRST_CMD_VALUE** `[31:0]` (RW)
    ///
    /// The LCD write command value of first cmd cycle.
    #[inline(always)]
    pub fn lcd_first_cmd_value(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_FIRST_CMD_VALUE_MASK)
            >> Self::LCD_FIRST_CMD_VALUE_OFFSET
    }

    /// Write **LCD_FIRST_CMD_VALUE** `[31:0]` (RW)
    ///
    /// The LCD write command value of first cmd cycle.
    #[inline(always)]
    pub fn write_lcd_first_cmd_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_FIRST_CMD_VALUE_MASK)
                | ((value << Self::LCD_FIRST_CMD_VALUE_OFFSET) & Self::LCD_FIRST_CMD_VALUE_MASK)
        });
    }
}

/// LCD_FIRST_CMD_VAL value type for batch read/modify/write operations
///
/// Use `lcdfirstcmdval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdfirstcmdval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdFirstCmdValVal {
    bits: u32,
}

impl LcdFirstCmdValVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LCD_FIRST_CMD_VALUE** `[31:0]`
    ///
    /// The LCD write command value of first cmd cycle.
    #[inline(always)]
    pub const fn lcd_first_cmd_value(&self) -> u32 {
        (self.bits & LcdFirstCmdVal::LCD_FIRST_CMD_VALUE_MASK)
            >> LcdFirstCmdVal::LCD_FIRST_CMD_VALUE_OFFSET
    }

    /// Set **LCD_FIRST_CMD_VALUE** `[31:0]`
    ///
    /// The LCD write command value of first cmd cycle.
    #[inline(always)]
    pub const fn with_lcd_first_cmd_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdFirstCmdVal::LCD_FIRST_CMD_VALUE_MASK)
                | ((value << LcdFirstCmdVal::LCD_FIRST_CMD_VALUE_OFFSET)
                    & LcdFirstCmdVal::LCD_FIRST_CMD_VALUE_MASK),
        }
    }
}

impl Default for LcdFirstCmdValVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdFirstCmdVal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdFirstCmdValVal {
        LcdFirstCmdValVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdFirstCmdValVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdFirstCmdValVal) -> LcdFirstCmdValVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_LATTER_CMD_VAL Register
///
/// LCD config register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **LCD_LATTER_CMD_VALUE** `[31:0]` (RW)
///
///   The LCD write command value of latter cmd cycle.
#[repr(transparent)]
pub struct LcdLatterCmdVal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdLatterCmdVal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdLatterCmdVal {
    // ======== Field Constants ========

    /// **LCD_LATTER_CMD_VALUE**
    ///
    /// The LCD write command value of latter cmd cycle.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LCD_LATTER_CMD_VALUE_OFFSET: u32 = 0;
    pub const LCD_LATTER_CMD_VALUE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LCD_LATTER_CMD_VALUE** `[31:0]` (RW)
    ///
    /// The LCD write command value of latter cmd cycle.
    #[inline(always)]
    pub fn lcd_latter_cmd_value(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_LATTER_CMD_VALUE_MASK)
            >> Self::LCD_LATTER_CMD_VALUE_OFFSET
    }

    /// Write **LCD_LATTER_CMD_VALUE** `[31:0]` (RW)
    ///
    /// The LCD write command value of latter cmd cycle.
    #[inline(always)]
    pub fn write_lcd_latter_cmd_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_LATTER_CMD_VALUE_MASK)
                | ((value << Self::LCD_LATTER_CMD_VALUE_OFFSET) & Self::LCD_LATTER_CMD_VALUE_MASK)
        });
    }
}

/// LCD_LATTER_CMD_VAL value type for batch read/modify/write operations
///
/// Use `lcdlattercmdval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdlattercmdval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdLatterCmdValVal {
    bits: u32,
}

impl LcdLatterCmdValVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LCD_LATTER_CMD_VALUE** `[31:0]`
    ///
    /// The LCD write command value of latter cmd cycle.
    #[inline(always)]
    pub const fn lcd_latter_cmd_value(&self) -> u32 {
        (self.bits & LcdLatterCmdVal::LCD_LATTER_CMD_VALUE_MASK)
            >> LcdLatterCmdVal::LCD_LATTER_CMD_VALUE_OFFSET
    }

    /// Set **LCD_LATTER_CMD_VALUE** `[31:0]`
    ///
    /// The LCD write command value of latter cmd cycle.
    #[inline(always)]
    pub const fn with_lcd_latter_cmd_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdLatterCmdVal::LCD_LATTER_CMD_VALUE_MASK)
                | ((value << LcdLatterCmdVal::LCD_LATTER_CMD_VALUE_OFFSET)
                    & LcdLatterCmdVal::LCD_LATTER_CMD_VALUE_MASK),
        }
    }
}

impl Default for LcdLatterCmdValVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdLatterCmdVal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdLatterCmdValVal {
        LcdLatterCmdValVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdLatterCmdValVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdLatterCmdValVal) -> LcdLatterCmdValVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_DLY_MODE_CFG1 Register
///
/// LCD config register.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **LCD_VSYNC_MODE** `[23:22]` (RW)
///
///   The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **LCD_HSYNC_MODE** `[21:20]` (RW)
///
///   The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **LCD_DE_MODE** `[19:18]` (RW)
///
///   The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **LCD_CD_MODE** `[17:16]` (RW)
///
///   The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT23_MODE** `[15:14]` (RW)
///
///   The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT22_MODE** `[13:12]` (RW)
///
///   The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT21_MODE** `[11:10]` (RW)
///
///   The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT20_MODE** `[9:8]` (RW)
///
///   The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT19_MODE** `[7:6]` (RW)
///
///   The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT18_MODE** `[5:4]` (RW)
///
///   The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT17_MODE** `[3:2]` (RW)
///
///   The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT16_MODE** `[1:0]` (RW)
///
///   The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
#[repr(transparent)]
pub struct LcdDlyModeCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdDlyModeCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdDlyModeCfg1 {
    // ======== Field Constants ========

    /// **LCD_VSYNC_MODE**
    ///
    /// The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const LCD_VSYNC_MODE_OFFSET: u32 = 22;
    pub const LCD_VSYNC_MODE_MASK: u32 = 0x00c00000;

    /// **LCD_HSYNC_MODE**
    ///
    /// The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const LCD_HSYNC_MODE_OFFSET: u32 = 20;
    pub const LCD_HSYNC_MODE_MASK: u32 = 0x00300000;

    /// **LCD_DE_MODE**
    ///
    /// The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const LCD_DE_MODE_OFFSET: u32 = 18;
    pub const LCD_DE_MODE_MASK: u32 = 0x000c0000;

    /// **LCD_CD_MODE**
    ///
    /// The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const LCD_CD_MODE_OFFSET: u32 = 16;
    pub const LCD_CD_MODE_MASK: u32 = 0x00030000;

    /// **DOUT23_MODE**
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const DOUT23_MODE_OFFSET: u32 = 14;
    pub const DOUT23_MODE_MASK: u32 = 0x0000c000;

    /// **DOUT22_MODE**
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const DOUT22_MODE_OFFSET: u32 = 12;
    pub const DOUT22_MODE_MASK: u32 = 0x00003000;

    /// **DOUT21_MODE**
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const DOUT21_MODE_OFFSET: u32 = 10;
    pub const DOUT21_MODE_MASK: u32 = 0x00000c00;

    /// **DOUT20_MODE**
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DOUT20_MODE_OFFSET: u32 = 8;
    pub const DOUT20_MODE_MASK: u32 = 0x00000300;

    /// **DOUT19_MODE**
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DOUT19_MODE_OFFSET: u32 = 6;
    pub const DOUT19_MODE_MASK: u32 = 0x000000c0;

    /// **DOUT18_MODE**
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DOUT18_MODE_OFFSET: u32 = 4;
    pub const DOUT18_MODE_MASK: u32 = 0x00000030;

    /// **DOUT17_MODE**
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const DOUT17_MODE_OFFSET: u32 = 2;
    pub const DOUT17_MODE_MASK: u32 = 0x0000000c;

    /// **DOUT16_MODE**
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DOUT16_MODE_OFFSET: u32 = 0;
    pub const DOUT16_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LCD_VSYNC_MODE** `[23:22]` (RW)
    ///
    /// The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_vsync_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_VSYNC_MODE_MASK) >> Self::LCD_VSYNC_MODE_OFFSET
    }

    /// Write **LCD_VSYNC_MODE** `[23:22]` (RW)
    ///
    /// The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_lcd_vsync_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_VSYNC_MODE_MASK)
                | ((value << Self::LCD_VSYNC_MODE_OFFSET) & Self::LCD_VSYNC_MODE_MASK)
        });
    }

    /// **LCD_HSYNC_MODE** `[21:20]` (RW)
    ///
    /// The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_hsync_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_HSYNC_MODE_MASK) >> Self::LCD_HSYNC_MODE_OFFSET
    }

    /// Write **LCD_HSYNC_MODE** `[21:20]` (RW)
    ///
    /// The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_lcd_hsync_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_HSYNC_MODE_MASK)
                | ((value << Self::LCD_HSYNC_MODE_OFFSET) & Self::LCD_HSYNC_MODE_MASK)
        });
    }

    /// **LCD_DE_MODE** `[19:18]` (RW)
    ///
    /// The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_de_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_DE_MODE_MASK) >> Self::LCD_DE_MODE_OFFSET
    }

    /// Write **LCD_DE_MODE** `[19:18]` (RW)
    ///
    /// The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_lcd_de_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_DE_MODE_MASK)
                | ((value << Self::LCD_DE_MODE_OFFSET) & Self::LCD_DE_MODE_MASK)
        });
    }

    /// **LCD_CD_MODE** `[17:16]` (RW)
    ///
    /// The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_cd_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LCD_CD_MODE_MASK) >> Self::LCD_CD_MODE_OFFSET
    }

    /// Write **LCD_CD_MODE** `[17:16]` (RW)
    ///
    /// The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_lcd_cd_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LCD_CD_MODE_MASK)
                | ((value << Self::LCD_CD_MODE_OFFSET) & Self::LCD_CD_MODE_MASK)
        });
    }

    /// **DOUT23_MODE** `[15:14]` (RW)
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout23_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT23_MODE_MASK) >> Self::DOUT23_MODE_OFFSET
    }

    /// Write **DOUT23_MODE** `[15:14]` (RW)
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout23_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT23_MODE_MASK)
                | ((value << Self::DOUT23_MODE_OFFSET) & Self::DOUT23_MODE_MASK)
        });
    }

    /// **DOUT22_MODE** `[13:12]` (RW)
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout22_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT22_MODE_MASK) >> Self::DOUT22_MODE_OFFSET
    }

    /// Write **DOUT22_MODE** `[13:12]` (RW)
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout22_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT22_MODE_MASK)
                | ((value << Self::DOUT22_MODE_OFFSET) & Self::DOUT22_MODE_MASK)
        });
    }

    /// **DOUT21_MODE** `[11:10]` (RW)
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout21_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT21_MODE_MASK) >> Self::DOUT21_MODE_OFFSET
    }

    /// Write **DOUT21_MODE** `[11:10]` (RW)
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout21_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT21_MODE_MASK)
                | ((value << Self::DOUT21_MODE_OFFSET) & Self::DOUT21_MODE_MASK)
        });
    }

    /// **DOUT20_MODE** `[9:8]` (RW)
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout20_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT20_MODE_MASK) >> Self::DOUT20_MODE_OFFSET
    }

    /// Write **DOUT20_MODE** `[9:8]` (RW)
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout20_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT20_MODE_MASK)
                | ((value << Self::DOUT20_MODE_OFFSET) & Self::DOUT20_MODE_MASK)
        });
    }

    /// **DOUT19_MODE** `[7:6]` (RW)
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout19_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT19_MODE_MASK) >> Self::DOUT19_MODE_OFFSET
    }

    /// Write **DOUT19_MODE** `[7:6]` (RW)
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout19_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT19_MODE_MASK)
                | ((value << Self::DOUT19_MODE_OFFSET) & Self::DOUT19_MODE_MASK)
        });
    }

    /// **DOUT18_MODE** `[5:4]` (RW)
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout18_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT18_MODE_MASK) >> Self::DOUT18_MODE_OFFSET
    }

    /// Write **DOUT18_MODE** `[5:4]` (RW)
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout18_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT18_MODE_MASK)
                | ((value << Self::DOUT18_MODE_OFFSET) & Self::DOUT18_MODE_MASK)
        });
    }

    /// **DOUT17_MODE** `[3:2]` (RW)
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout17_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT17_MODE_MASK) >> Self::DOUT17_MODE_OFFSET
    }

    /// Write **DOUT17_MODE** `[3:2]` (RW)
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout17_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT17_MODE_MASK)
                | ((value << Self::DOUT17_MODE_OFFSET) & Self::DOUT17_MODE_MASK)
        });
    }

    /// **DOUT16_MODE** `[1:0]` (RW)
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout16_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT16_MODE_MASK) >> Self::DOUT16_MODE_OFFSET
    }

    /// Write **DOUT16_MODE** `[1:0]` (RW)
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout16_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT16_MODE_MASK)
                | ((value << Self::DOUT16_MODE_OFFSET) & Self::DOUT16_MODE_MASK)
        });
    }
}

/// LCD_DLY_MODE_CFG1 value type for batch read/modify/write operations
///
/// Use `lcddlymodecfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcddlymodecfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdDlyModeCfg1Val {
    bits: u32,
}

impl LcdDlyModeCfg1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LCD_VSYNC_MODE** `[23:22]`
    ///
    /// The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn lcd_vsync_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::LCD_VSYNC_MODE_MASK) >> LcdDlyModeCfg1::LCD_VSYNC_MODE_OFFSET
    }

    /// Read **LCD_HSYNC_MODE** `[21:20]`
    ///
    /// The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn lcd_hsync_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::LCD_HSYNC_MODE_MASK) >> LcdDlyModeCfg1::LCD_HSYNC_MODE_OFFSET
    }

    /// Read **LCD_DE_MODE** `[19:18]`
    ///
    /// The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn lcd_de_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::LCD_DE_MODE_MASK) >> LcdDlyModeCfg1::LCD_DE_MODE_OFFSET
    }

    /// Read **LCD_CD_MODE** `[17:16]`
    ///
    /// The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn lcd_cd_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::LCD_CD_MODE_MASK) >> LcdDlyModeCfg1::LCD_CD_MODE_OFFSET
    }

    /// Read **DOUT23_MODE** `[15:14]`
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout23_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT23_MODE_MASK) >> LcdDlyModeCfg1::DOUT23_MODE_OFFSET
    }

    /// Read **DOUT22_MODE** `[13:12]`
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout22_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT22_MODE_MASK) >> LcdDlyModeCfg1::DOUT22_MODE_OFFSET
    }

    /// Read **DOUT21_MODE** `[11:10]`
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout21_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT21_MODE_MASK) >> LcdDlyModeCfg1::DOUT21_MODE_OFFSET
    }

    /// Read **DOUT20_MODE** `[9:8]`
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout20_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT20_MODE_MASK) >> LcdDlyModeCfg1::DOUT20_MODE_OFFSET
    }

    /// Read **DOUT19_MODE** `[7:6]`
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout19_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT19_MODE_MASK) >> LcdDlyModeCfg1::DOUT19_MODE_OFFSET
    }

    /// Read **DOUT18_MODE** `[5:4]`
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout18_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT18_MODE_MASK) >> LcdDlyModeCfg1::DOUT18_MODE_OFFSET
    }

    /// Read **DOUT17_MODE** `[3:2]`
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout17_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT17_MODE_MASK) >> LcdDlyModeCfg1::DOUT17_MODE_OFFSET
    }

    /// Read **DOUT16_MODE** `[1:0]`
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout16_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg1::DOUT16_MODE_MASK) >> LcdDlyModeCfg1::DOUT16_MODE_OFFSET
    }

    /// Set **LCD_VSYNC_MODE** `[23:22]`
    ///
    /// The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_lcd_vsync_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::LCD_VSYNC_MODE_MASK)
                | ((value << LcdDlyModeCfg1::LCD_VSYNC_MODE_OFFSET)
                    & LcdDlyModeCfg1::LCD_VSYNC_MODE_MASK),
        }
    }

    /// Set **LCD_HSYNC_MODE** `[21:20]`
    ///
    /// The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_lcd_hsync_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::LCD_HSYNC_MODE_MASK)
                | ((value << LcdDlyModeCfg1::LCD_HSYNC_MODE_OFFSET)
                    & LcdDlyModeCfg1::LCD_HSYNC_MODE_MASK),
        }
    }

    /// Set **LCD_DE_MODE** `[19:18]`
    ///
    /// The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_lcd_de_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::LCD_DE_MODE_MASK)
                | ((value << LcdDlyModeCfg1::LCD_DE_MODE_OFFSET)
                    & LcdDlyModeCfg1::LCD_DE_MODE_MASK),
        }
    }

    /// Set **LCD_CD_MODE** `[17:16]`
    ///
    /// The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_lcd_cd_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::LCD_CD_MODE_MASK)
                | ((value << LcdDlyModeCfg1::LCD_CD_MODE_OFFSET)
                    & LcdDlyModeCfg1::LCD_CD_MODE_MASK),
        }
    }

    /// Set **DOUT23_MODE** `[15:14]`
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout23_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT23_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT23_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT23_MODE_MASK),
        }
    }

    /// Set **DOUT22_MODE** `[13:12]`
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout22_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT22_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT22_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT22_MODE_MASK),
        }
    }

    /// Set **DOUT21_MODE** `[11:10]`
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout21_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT21_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT21_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT21_MODE_MASK),
        }
    }

    /// Set **DOUT20_MODE** `[9:8]`
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout20_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT20_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT20_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT20_MODE_MASK),
        }
    }

    /// Set **DOUT19_MODE** `[7:6]`
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout19_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT19_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT19_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT19_MODE_MASK),
        }
    }

    /// Set **DOUT18_MODE** `[5:4]`
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout18_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT18_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT18_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT18_MODE_MASK),
        }
    }

    /// Set **DOUT17_MODE** `[3:2]`
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout17_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT17_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT17_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT17_MODE_MASK),
        }
    }

    /// Set **DOUT16_MODE** `[1:0]`
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout16_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg1::DOUT16_MODE_MASK)
                | ((value << LcdDlyModeCfg1::DOUT16_MODE_OFFSET)
                    & LcdDlyModeCfg1::DOUT16_MODE_MASK),
        }
    }
}

impl Default for LcdDlyModeCfg1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdDlyModeCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdDlyModeCfg1Val {
        LcdDlyModeCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdDlyModeCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdDlyModeCfg1Val) -> LcdDlyModeCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_DLY_MODE_CFG2 Register
///
/// LCD config register.
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **DOUT15_MODE** `[31:30]` (RW)
///
///   The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT14_MODE** `[29:28]` (RW)
///
///   The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT13_MODE** `[27:26]` (RW)
///
///   The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT12_MODE** `[25:24]` (RW)
///
///   The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT11_MODE** `[23:22]` (RW)
///
///   The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT10_MODE** `[21:20]` (RW)
///
///   The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT9_MODE** `[19:18]` (RW)
///
///   The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT8_MODE** `[17:16]` (RW)
///
///   The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT7_MODE** `[15:14]` (RW)
///
///   The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT6_MODE** `[13:12]` (RW)
///
///   The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT5_MODE** `[11:10]` (RW)
///
///   The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT4_MODE** `[9:8]` (RW)
///
///   The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT3_MODE** `[7:6]` (RW)
///
///   The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT2_MODE** `[5:4]` (RW)
///
///   The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT1_MODE** `[3:2]` (RW)
///
///   The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
///
/// - **DOUT0_MODE** `[1:0]` (RW)
///
///   The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
#[repr(transparent)]
pub struct LcdDlyModeCfg2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdDlyModeCfg2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdDlyModeCfg2 {
    // ======== Field Constants ========

    /// **DOUT15_MODE**
    ///
    /// The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const DOUT15_MODE_OFFSET: u32 = 30;
    pub const DOUT15_MODE_MASK: u32 = 0xc0000000;

    /// **DOUT14_MODE**
    ///
    /// The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const DOUT14_MODE_OFFSET: u32 = 28;
    pub const DOUT14_MODE_MASK: u32 = 0x30000000;

    /// **DOUT13_MODE**
    ///
    /// The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const DOUT13_MODE_OFFSET: u32 = 26;
    pub const DOUT13_MODE_MASK: u32 = 0x0c000000;

    /// **DOUT12_MODE**
    ///
    /// The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const DOUT12_MODE_OFFSET: u32 = 24;
    pub const DOUT12_MODE_MASK: u32 = 0x03000000;

    /// **DOUT11_MODE**
    ///
    /// The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const DOUT11_MODE_OFFSET: u32 = 22;
    pub const DOUT11_MODE_MASK: u32 = 0x00c00000;

    /// **DOUT10_MODE**
    ///
    /// The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const DOUT10_MODE_OFFSET: u32 = 20;
    pub const DOUT10_MODE_MASK: u32 = 0x00300000;

    /// **DOUT9_MODE**
    ///
    /// The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const DOUT9_MODE_OFFSET: u32 = 18;
    pub const DOUT9_MODE_MASK: u32 = 0x000c0000;

    /// **DOUT8_MODE**
    ///
    /// The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const DOUT8_MODE_OFFSET: u32 = 16;
    pub const DOUT8_MODE_MASK: u32 = 0x00030000;

    /// **DOUT7_MODE**
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const DOUT7_MODE_OFFSET: u32 = 14;
    pub const DOUT7_MODE_MASK: u32 = 0x0000c000;

    /// **DOUT6_MODE**
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const DOUT6_MODE_OFFSET: u32 = 12;
    pub const DOUT6_MODE_MASK: u32 = 0x00003000;

    /// **DOUT5_MODE**
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const DOUT5_MODE_OFFSET: u32 = 10;
    pub const DOUT5_MODE_MASK: u32 = 0x00000c00;

    /// **DOUT4_MODE**
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DOUT4_MODE_OFFSET: u32 = 8;
    pub const DOUT4_MODE_MASK: u32 = 0x00000300;

    /// **DOUT3_MODE**
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DOUT3_MODE_OFFSET: u32 = 6;
    pub const DOUT3_MODE_MASK: u32 = 0x000000c0;

    /// **DOUT2_MODE**
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DOUT2_MODE_OFFSET: u32 = 4;
    pub const DOUT2_MODE_MASK: u32 = 0x00000030;

    /// **DOUT1_MODE**
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const DOUT1_MODE_OFFSET: u32 = 2;
    pub const DOUT1_MODE_MASK: u32 = 0x0000000c;

    /// **DOUT0_MODE**
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DOUT0_MODE_OFFSET: u32 = 0;
    pub const DOUT0_MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DOUT15_MODE** `[31:30]` (RW)
    ///
    /// The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout15_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT15_MODE_MASK) >> Self::DOUT15_MODE_OFFSET
    }

    /// Write **DOUT15_MODE** `[31:30]` (RW)
    ///
    /// The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout15_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT15_MODE_MASK)
                | ((value << Self::DOUT15_MODE_OFFSET) & Self::DOUT15_MODE_MASK)
        });
    }

    /// **DOUT14_MODE** `[29:28]` (RW)
    ///
    /// The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout14_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT14_MODE_MASK) >> Self::DOUT14_MODE_OFFSET
    }

    /// Write **DOUT14_MODE** `[29:28]` (RW)
    ///
    /// The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout14_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT14_MODE_MASK)
                | ((value << Self::DOUT14_MODE_OFFSET) & Self::DOUT14_MODE_MASK)
        });
    }

    /// **DOUT13_MODE** `[27:26]` (RW)
    ///
    /// The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout13_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT13_MODE_MASK) >> Self::DOUT13_MODE_OFFSET
    }

    /// Write **DOUT13_MODE** `[27:26]` (RW)
    ///
    /// The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout13_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT13_MODE_MASK)
                | ((value << Self::DOUT13_MODE_OFFSET) & Self::DOUT13_MODE_MASK)
        });
    }

    /// **DOUT12_MODE** `[25:24]` (RW)
    ///
    /// The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout12_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT12_MODE_MASK) >> Self::DOUT12_MODE_OFFSET
    }

    /// Write **DOUT12_MODE** `[25:24]` (RW)
    ///
    /// The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout12_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT12_MODE_MASK)
                | ((value << Self::DOUT12_MODE_OFFSET) & Self::DOUT12_MODE_MASK)
        });
    }

    /// **DOUT11_MODE** `[23:22]` (RW)
    ///
    /// The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout11_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT11_MODE_MASK) >> Self::DOUT11_MODE_OFFSET
    }

    /// Write **DOUT11_MODE** `[23:22]` (RW)
    ///
    /// The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout11_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT11_MODE_MASK)
                | ((value << Self::DOUT11_MODE_OFFSET) & Self::DOUT11_MODE_MASK)
        });
    }

    /// **DOUT10_MODE** `[21:20]` (RW)
    ///
    /// The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout10_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT10_MODE_MASK) >> Self::DOUT10_MODE_OFFSET
    }

    /// Write **DOUT10_MODE** `[21:20]` (RW)
    ///
    /// The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout10_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT10_MODE_MASK)
                | ((value << Self::DOUT10_MODE_OFFSET) & Self::DOUT10_MODE_MASK)
        });
    }

    /// **DOUT9_MODE** `[19:18]` (RW)
    ///
    /// The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout9_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT9_MODE_MASK) >> Self::DOUT9_MODE_OFFSET
    }

    /// Write **DOUT9_MODE** `[19:18]` (RW)
    ///
    /// The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout9_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT9_MODE_MASK)
                | ((value << Self::DOUT9_MODE_OFFSET) & Self::DOUT9_MODE_MASK)
        });
    }

    /// **DOUT8_MODE** `[17:16]` (RW)
    ///
    /// The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout8_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT8_MODE_MASK) >> Self::DOUT8_MODE_OFFSET
    }

    /// Write **DOUT8_MODE** `[17:16]` (RW)
    ///
    /// The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout8_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT8_MODE_MASK)
                | ((value << Self::DOUT8_MODE_OFFSET) & Self::DOUT8_MODE_MASK)
        });
    }

    /// **DOUT7_MODE** `[15:14]` (RW)
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout7_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT7_MODE_MASK) >> Self::DOUT7_MODE_OFFSET
    }

    /// Write **DOUT7_MODE** `[15:14]` (RW)
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout7_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT7_MODE_MASK)
                | ((value << Self::DOUT7_MODE_OFFSET) & Self::DOUT7_MODE_MASK)
        });
    }

    /// **DOUT6_MODE** `[13:12]` (RW)
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout6_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT6_MODE_MASK) >> Self::DOUT6_MODE_OFFSET
    }

    /// Write **DOUT6_MODE** `[13:12]` (RW)
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout6_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT6_MODE_MASK)
                | ((value << Self::DOUT6_MODE_OFFSET) & Self::DOUT6_MODE_MASK)
        });
    }

    /// **DOUT5_MODE** `[11:10]` (RW)
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout5_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT5_MODE_MASK) >> Self::DOUT5_MODE_OFFSET
    }

    /// Write **DOUT5_MODE** `[11:10]` (RW)
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout5_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT5_MODE_MASK)
                | ((value << Self::DOUT5_MODE_OFFSET) & Self::DOUT5_MODE_MASK)
        });
    }

    /// **DOUT4_MODE** `[9:8]` (RW)
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout4_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT4_MODE_MASK) >> Self::DOUT4_MODE_OFFSET
    }

    /// Write **DOUT4_MODE** `[9:8]` (RW)
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout4_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT4_MODE_MASK)
                | ((value << Self::DOUT4_MODE_OFFSET) & Self::DOUT4_MODE_MASK)
        });
    }

    /// **DOUT3_MODE** `[7:6]` (RW)
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout3_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT3_MODE_MASK) >> Self::DOUT3_MODE_OFFSET
    }

    /// Write **DOUT3_MODE** `[7:6]` (RW)
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout3_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT3_MODE_MASK)
                | ((value << Self::DOUT3_MODE_OFFSET) & Self::DOUT3_MODE_MASK)
        });
    }

    /// **DOUT2_MODE** `[5:4]` (RW)
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout2_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT2_MODE_MASK) >> Self::DOUT2_MODE_OFFSET
    }

    /// Write **DOUT2_MODE** `[5:4]` (RW)
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout2_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT2_MODE_MASK)
                | ((value << Self::DOUT2_MODE_OFFSET) & Self::DOUT2_MODE_MASK)
        });
    }

    /// **DOUT1_MODE** `[3:2]` (RW)
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout1_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT1_MODE_MASK) >> Self::DOUT1_MODE_OFFSET
    }

    /// Write **DOUT1_MODE** `[3:2]` (RW)
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout1_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT1_MODE_MASK)
                | ((value << Self::DOUT1_MODE_OFFSET) & Self::DOUT1_MODE_MASK)
        });
    }

    /// **DOUT0_MODE** `[1:0]` (RW)
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn dout0_mode(&self) -> u32 {
        (self.value.read_bits() & Self::DOUT0_MODE_MASK) >> Self::DOUT0_MODE_OFFSET
    }

    /// Write **DOUT0_MODE** `[1:0]` (RW)
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub fn write_dout0_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DOUT0_MODE_MASK)
                | ((value << Self::DOUT0_MODE_OFFSET) & Self::DOUT0_MODE_MASK)
        });
    }
}

/// LCD_DLY_MODE_CFG2 value type for batch read/modify/write operations
///
/// Use `lcddlymodecfg2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcddlymodecfg2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdDlyModeCfg2Val {
    bits: u32,
}

impl LcdDlyModeCfg2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DOUT15_MODE** `[31:30]`
    ///
    /// The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout15_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT15_MODE_MASK) >> LcdDlyModeCfg2::DOUT15_MODE_OFFSET
    }

    /// Read **DOUT14_MODE** `[29:28]`
    ///
    /// The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout14_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT14_MODE_MASK) >> LcdDlyModeCfg2::DOUT14_MODE_OFFSET
    }

    /// Read **DOUT13_MODE** `[27:26]`
    ///
    /// The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout13_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT13_MODE_MASK) >> LcdDlyModeCfg2::DOUT13_MODE_OFFSET
    }

    /// Read **DOUT12_MODE** `[25:24]`
    ///
    /// The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout12_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT12_MODE_MASK) >> LcdDlyModeCfg2::DOUT12_MODE_OFFSET
    }

    /// Read **DOUT11_MODE** `[23:22]`
    ///
    /// The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout11_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT11_MODE_MASK) >> LcdDlyModeCfg2::DOUT11_MODE_OFFSET
    }

    /// Read **DOUT10_MODE** `[21:20]`
    ///
    /// The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout10_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT10_MODE_MASK) >> LcdDlyModeCfg2::DOUT10_MODE_OFFSET
    }

    /// Read **DOUT9_MODE** `[19:18]`
    ///
    /// The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout9_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT9_MODE_MASK) >> LcdDlyModeCfg2::DOUT9_MODE_OFFSET
    }

    /// Read **DOUT8_MODE** `[17:16]`
    ///
    /// The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout8_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT8_MODE_MASK) >> LcdDlyModeCfg2::DOUT8_MODE_OFFSET
    }

    /// Read **DOUT7_MODE** `[15:14]`
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout7_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT7_MODE_MASK) >> LcdDlyModeCfg2::DOUT7_MODE_OFFSET
    }

    /// Read **DOUT6_MODE** `[13:12]`
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout6_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT6_MODE_MASK) >> LcdDlyModeCfg2::DOUT6_MODE_OFFSET
    }

    /// Read **DOUT5_MODE** `[11:10]`
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout5_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT5_MODE_MASK) >> LcdDlyModeCfg2::DOUT5_MODE_OFFSET
    }

    /// Read **DOUT4_MODE** `[9:8]`
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout4_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT4_MODE_MASK) >> LcdDlyModeCfg2::DOUT4_MODE_OFFSET
    }

    /// Read **DOUT3_MODE** `[7:6]`
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout3_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT3_MODE_MASK) >> LcdDlyModeCfg2::DOUT3_MODE_OFFSET
    }

    /// Read **DOUT2_MODE** `[5:4]`
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout2_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT2_MODE_MASK) >> LcdDlyModeCfg2::DOUT2_MODE_OFFSET
    }

    /// Read **DOUT1_MODE** `[3:2]`
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout1_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT1_MODE_MASK) >> LcdDlyModeCfg2::DOUT1_MODE_OFFSET
    }

    /// Read **DOUT0_MODE** `[1:0]`
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn dout0_mode(&self) -> u32 {
        (self.bits & LcdDlyModeCfg2::DOUT0_MODE_MASK) >> LcdDlyModeCfg2::DOUT0_MODE_OFFSET
    }

    /// Set **DOUT15_MODE** `[31:30]`
    ///
    /// The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout15_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT15_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT15_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT15_MODE_MASK),
        }
    }

    /// Set **DOUT14_MODE** `[29:28]`
    ///
    /// The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout14_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT14_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT14_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT14_MODE_MASK),
        }
    }

    /// Set **DOUT13_MODE** `[27:26]`
    ///
    /// The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout13_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT13_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT13_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT13_MODE_MASK),
        }
    }

    /// Set **DOUT12_MODE** `[25:24]`
    ///
    /// The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout12_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT12_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT12_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT12_MODE_MASK),
        }
    }

    /// Set **DOUT11_MODE** `[23:22]`
    ///
    /// The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout11_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT11_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT11_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT11_MODE_MASK),
        }
    }

    /// Set **DOUT10_MODE** `[21:20]`
    ///
    /// The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout10_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT10_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT10_MODE_OFFSET)
                    & LcdDlyModeCfg2::DOUT10_MODE_MASK),
        }
    }

    /// Set **DOUT9_MODE** `[19:18]`
    ///
    /// The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout9_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT9_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT9_MODE_OFFSET) & LcdDlyModeCfg2::DOUT9_MODE_MASK),
        }
    }

    /// Set **DOUT8_MODE** `[17:16]`
    ///
    /// The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout8_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT8_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT8_MODE_OFFSET) & LcdDlyModeCfg2::DOUT8_MODE_MASK),
        }
    }

    /// Set **DOUT7_MODE** `[15:14]`
    ///
    /// The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout7_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT7_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT7_MODE_OFFSET) & LcdDlyModeCfg2::DOUT7_MODE_MASK),
        }
    }

    /// Set **DOUT6_MODE** `[13:12]`
    ///
    /// The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout6_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT6_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT6_MODE_OFFSET) & LcdDlyModeCfg2::DOUT6_MODE_MASK),
        }
    }

    /// Set **DOUT5_MODE** `[11:10]`
    ///
    /// The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout5_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT5_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT5_MODE_OFFSET) & LcdDlyModeCfg2::DOUT5_MODE_MASK),
        }
    }

    /// Set **DOUT4_MODE** `[9:8]`
    ///
    /// The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout4_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT4_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT4_MODE_OFFSET) & LcdDlyModeCfg2::DOUT4_MODE_MASK),
        }
    }

    /// Set **DOUT3_MODE** `[7:6]`
    ///
    /// The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout3_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT3_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT3_MODE_OFFSET) & LcdDlyModeCfg2::DOUT3_MODE_MASK),
        }
    }

    /// Set **DOUT2_MODE** `[5:4]`
    ///
    /// The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout2_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT2_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT2_MODE_OFFSET) & LcdDlyModeCfg2::DOUT2_MODE_MASK),
        }
    }

    /// Set **DOUT1_MODE** `[3:2]`
    ///
    /// The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout1_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT1_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT1_MODE_OFFSET) & LcdDlyModeCfg2::DOUT1_MODE_MASK),
        }
    }

    /// Set **DOUT0_MODE** `[1:0]`
    ///
    /// The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK.
    #[inline(always)]
    pub const fn with_dout0_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdDlyModeCfg2::DOUT0_MODE_MASK)
                | ((value << LcdDlyModeCfg2::DOUT0_MODE_OFFSET) & LcdDlyModeCfg2::DOUT0_MODE_MASK),
        }
    }
}

impl Default for LcdDlyModeCfg2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdDlyModeCfg2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdDlyModeCfg2Val {
        LcdDlyModeCfg2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdDlyModeCfg2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdDlyModeCfg2Val) -> LcdDlyModeCfg2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LC_DMA_INT_ENA Register
///
/// LCDCAM interrupt enable register.
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **CAM_HS_INT_ENA** `[3]` (RW)
///
///   The enable bit for Camera line interrupt.
///
/// - **CAM_VSYNC_INT_ENA** `[2]` (RW)
///
///   The enable bit for Camera frame end interrupt.
///
/// - **LCD_TRANS_DONE_INT_ENA** `[1]` (RW)
///
///   The enable bit for lcd transfer end interrupt.
///
/// - **LCD_VSYNC_INT_ENA** `[0]` (RW)
///
///   The enable bit for LCD frame end interrupt.
#[repr(transparent)]
pub struct LcDmaIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcDmaIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcDmaIntEna {
    // ======== Field Constants ========

    /// **CAM_HS_INT_ENA**
    ///
    /// The enable bit for Camera line interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CAM_HS_INT_ENA_OFFSET: u32 = 3;
    pub const CAM_HS_INT_ENA_MASK: u32 = 0x00000008;
    pub const CAM_HS_INT_ENA: u32 = 1 << 3;

    /// **CAM_VSYNC_INT_ENA**
    ///
    /// The enable bit for Camera frame end interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CAM_VSYNC_INT_ENA_OFFSET: u32 = 2;
    pub const CAM_VSYNC_INT_ENA_MASK: u32 = 0x00000004;
    pub const CAM_VSYNC_INT_ENA: u32 = 1 << 2;

    /// **LCD_TRANS_DONE_INT_ENA**
    ///
    /// The enable bit for lcd transfer end interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LCD_TRANS_DONE_INT_ENA_OFFSET: u32 = 1;
    pub const LCD_TRANS_DONE_INT_ENA_MASK: u32 = 0x00000002;
    pub const LCD_TRANS_DONE_INT_ENA: u32 = 1 << 1;

    /// **LCD_VSYNC_INT_ENA**
    ///
    /// The enable bit for LCD frame end interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LCD_VSYNC_INT_ENA_OFFSET: u32 = 0;
    pub const LCD_VSYNC_INT_ENA_MASK: u32 = 0x00000001;
    pub const LCD_VSYNC_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CAM_HS_INT_ENA** `[3]` (RW)
    ///
    /// The enable bit for Camera line interrupt.
    #[inline(always)]
    pub fn cam_hs_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CAM_HS_INT_ENA_MASK) != 0
    }

    /// Set CAM_HS_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_hs_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CAM_HS_INT_ENA_MASK);
    }

    /// Clear CAM_HS_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_hs_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CAM_HS_INT_ENA_MASK);
    }

    /// **CAM_VSYNC_INT_ENA** `[2]` (RW)
    ///
    /// The enable bit for Camera frame end interrupt.
    #[inline(always)]
    pub fn cam_vsync_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VSYNC_INT_ENA_MASK) != 0
    }

    /// Set CAM_VSYNC_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cam_vsync_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CAM_VSYNC_INT_ENA_MASK);
    }

    /// Clear CAM_VSYNC_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cam_vsync_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CAM_VSYNC_INT_ENA_MASK);
    }

    /// **LCD_TRANS_DONE_INT_ENA** `[1]` (RW)
    ///
    /// The enable bit for lcd transfer end interrupt.
    #[inline(always)]
    pub fn lcd_trans_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LCD_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Set LCD_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LCD_TRANS_DONE_INT_ENA_MASK);
    }

    /// Clear LCD_TRANS_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_trans_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_TRANS_DONE_INT_ENA_MASK);
    }

    /// **LCD_VSYNC_INT_ENA** `[0]` (RW)
    ///
    /// The enable bit for LCD frame end interrupt.
    #[inline(always)]
    pub fn lcd_vsync_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LCD_VSYNC_INT_ENA_MASK) != 0
    }

    /// Set LCD_VSYNC_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lcd_vsync_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::LCD_VSYNC_INT_ENA_MASK);
    }

    /// Clear LCD_VSYNC_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lcd_vsync_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LCD_VSYNC_INT_ENA_MASK);
    }
}

/// LC_DMA_INT_ENA value type for batch read/modify/write operations
///
/// Use `lcdmaintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdmaintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcDmaIntEnaVal {
    bits: u32,
}

impl LcDmaIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CAM_HS_INT_ENA** `[3:3]`
    ///
    /// The enable bit for Camera line interrupt.
    #[inline(always)]
    pub const fn cam_hs_int_ena(&self) -> bool {
        (self.bits & LcDmaIntEna::CAM_HS_INT_ENA_MASK) != 0
    }

    /// Read **CAM_VSYNC_INT_ENA** `[2:2]`
    ///
    /// The enable bit for Camera frame end interrupt.
    #[inline(always)]
    pub const fn cam_vsync_int_ena(&self) -> bool {
        (self.bits & LcDmaIntEna::CAM_VSYNC_INT_ENA_MASK) != 0
    }

    /// Read **LCD_TRANS_DONE_INT_ENA** `[1:1]`
    ///
    /// The enable bit for lcd transfer end interrupt.
    #[inline(always)]
    pub const fn lcd_trans_done_int_ena(&self) -> bool {
        (self.bits & LcDmaIntEna::LCD_TRANS_DONE_INT_ENA_MASK) != 0
    }

    /// Read **LCD_VSYNC_INT_ENA** `[0:0]`
    ///
    /// The enable bit for LCD frame end interrupt.
    #[inline(always)]
    pub const fn lcd_vsync_int_ena(&self) -> bool {
        (self.bits & LcDmaIntEna::LCD_VSYNC_INT_ENA_MASK) != 0
    }

    /// Set **CAM_HS_INT_ENA** `[3:3]`
    ///
    /// The enable bit for Camera line interrupt.
    #[inline(always)]
    pub const fn with_cam_hs_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntEna::CAM_HS_INT_ENA_MASK
            } else {
                self.bits & !LcDmaIntEna::CAM_HS_INT_ENA_MASK
            },
        }
    }

    /// Set **CAM_VSYNC_INT_ENA** `[2:2]`
    ///
    /// The enable bit for Camera frame end interrupt.
    #[inline(always)]
    pub const fn with_cam_vsync_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntEna::CAM_VSYNC_INT_ENA_MASK
            } else {
                self.bits & !LcDmaIntEna::CAM_VSYNC_INT_ENA_MASK
            },
        }
    }

    /// Set **LCD_TRANS_DONE_INT_ENA** `[1:1]`
    ///
    /// The enable bit for lcd transfer end interrupt.
    #[inline(always)]
    pub const fn with_lcd_trans_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntEna::LCD_TRANS_DONE_INT_ENA_MASK
            } else {
                self.bits & !LcDmaIntEna::LCD_TRANS_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **LCD_VSYNC_INT_ENA** `[0:0]`
    ///
    /// The enable bit for LCD frame end interrupt.
    #[inline(always)]
    pub const fn with_lcd_vsync_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntEna::LCD_VSYNC_INT_ENA_MASK
            } else {
                self.bits & !LcDmaIntEna::LCD_VSYNC_INT_ENA_MASK
            },
        }
    }
}

impl Default for LcDmaIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcDmaIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcDmaIntEnaVal {
        LcDmaIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcDmaIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcDmaIntEnaVal) -> LcDmaIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LC_DMA_INT_RAW Register
///
/// LCDCAM interrupt raw register, valid in level.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **CAM_HS_INT_RAW** `[3]` (RO)
///
///   The raw bit for Camera line interrupt.
///
/// - **CAM_VSYNC_INT_RAW** `[2]` (RO)
///
///   The raw bit for Camera frame end interrupt.
///
/// - **LCD_TRANS_DONE_INT_RAW** `[1]` (RO)
///
///   The raw bit for lcd transfer end interrupt.
///
/// - **LCD_VSYNC_INT_RAW** `[0]` (RO)
///
///   The raw bit for LCD frame end interrupt.
#[repr(transparent)]
pub struct LcDmaIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcDmaIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcDmaIntRaw {
    // ======== Field Constants ========

    /// **CAM_HS_INT_RAW**
    ///
    /// The raw bit for Camera line interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CAM_HS_INT_RAW_OFFSET: u32 = 3;
    pub const CAM_HS_INT_RAW_MASK: u32 = 0x00000008;
    pub const CAM_HS_INT_RAW: u32 = 1 << 3;

    /// **CAM_VSYNC_INT_RAW**
    ///
    /// The raw bit for Camera frame end interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CAM_VSYNC_INT_RAW_OFFSET: u32 = 2;
    pub const CAM_VSYNC_INT_RAW_MASK: u32 = 0x00000004;
    pub const CAM_VSYNC_INT_RAW: u32 = 1 << 2;

    /// **LCD_TRANS_DONE_INT_RAW**
    ///
    /// The raw bit for lcd transfer end interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LCD_TRANS_DONE_INT_RAW_OFFSET: u32 = 1;
    pub const LCD_TRANS_DONE_INT_RAW_MASK: u32 = 0x00000002;
    pub const LCD_TRANS_DONE_INT_RAW: u32 = 1 << 1;

    /// **LCD_VSYNC_INT_RAW**
    ///
    /// The raw bit for LCD frame end interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LCD_VSYNC_INT_RAW_OFFSET: u32 = 0;
    pub const LCD_VSYNC_INT_RAW_MASK: u32 = 0x00000001;
    pub const LCD_VSYNC_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CAM_HS_INT_RAW** `[3]` (RO)
    ///
    /// The raw bit for Camera line interrupt.
    #[inline(always)]
    pub fn cam_hs_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CAM_HS_INT_RAW_MASK) != 0
    }

    /// **CAM_VSYNC_INT_RAW** `[2]` (RO)
    ///
    /// The raw bit for Camera frame end interrupt.
    #[inline(always)]
    pub fn cam_vsync_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VSYNC_INT_RAW_MASK) != 0
    }

    /// **LCD_TRANS_DONE_INT_RAW** `[1]` (RO)
    ///
    /// The raw bit for lcd transfer end interrupt.
    #[inline(always)]
    pub fn lcd_trans_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LCD_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// **LCD_VSYNC_INT_RAW** `[0]` (RO)
    ///
    /// The raw bit for LCD frame end interrupt.
    #[inline(always)]
    pub fn lcd_vsync_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LCD_VSYNC_INT_RAW_MASK) != 0
    }
}

/// LC_DMA_INT_RAW value type for batch read/modify/write operations
///
/// Use `lcdmaintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdmaintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcDmaIntRawVal {
    bits: u32,
}

impl LcDmaIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CAM_HS_INT_RAW** `[3:3]`
    ///
    /// The raw bit for Camera line interrupt.
    #[inline(always)]
    pub const fn cam_hs_int_raw(&self) -> bool {
        (self.bits & LcDmaIntRaw::CAM_HS_INT_RAW_MASK) != 0
    }

    /// Read **CAM_VSYNC_INT_RAW** `[2:2]`
    ///
    /// The raw bit for Camera frame end interrupt.
    #[inline(always)]
    pub const fn cam_vsync_int_raw(&self) -> bool {
        (self.bits & LcDmaIntRaw::CAM_VSYNC_INT_RAW_MASK) != 0
    }

    /// Read **LCD_TRANS_DONE_INT_RAW** `[1:1]`
    ///
    /// The raw bit for lcd transfer end interrupt.
    #[inline(always)]
    pub const fn lcd_trans_done_int_raw(&self) -> bool {
        (self.bits & LcDmaIntRaw::LCD_TRANS_DONE_INT_RAW_MASK) != 0
    }

    /// Read **LCD_VSYNC_INT_RAW** `[0:0]`
    ///
    /// The raw bit for LCD frame end interrupt.
    #[inline(always)]
    pub const fn lcd_vsync_int_raw(&self) -> bool {
        (self.bits & LcDmaIntRaw::LCD_VSYNC_INT_RAW_MASK) != 0
    }
}

impl Default for LcDmaIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcDmaIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcDmaIntRawVal {
        LcDmaIntRawVal::from_bits(self.value.read_bits())
    }
}

/// LC_DMA_INT_ST Register
///
/// LCDCAM interrupt status register.
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **CAM_HS_INT_ST** `[3]` (RO)
///
///   The status bit for Camera transfer end interrupt.
///
/// - **CAM_VSYNC_INT_ST** `[2]` (RO)
///
///   The status bit for Camera frame end interrupt.
///
/// - **LCD_TRANS_DONE_INT_ST** `[1]` (RO)
///
///   The status bit for lcd transfer end interrupt.
///
/// - **LCD_VSYNC_INT_ST** `[0]` (RO)
///
///   The status bit for LCD frame end interrupt.
#[repr(transparent)]
pub struct LcDmaIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcDmaIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcDmaIntSt {
    // ======== Field Constants ========

    /// **CAM_HS_INT_ST**
    ///
    /// The status bit for Camera transfer end interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CAM_HS_INT_ST_OFFSET: u32 = 3;
    pub const CAM_HS_INT_ST_MASK: u32 = 0x00000008;
    pub const CAM_HS_INT_ST: u32 = 1 << 3;

    /// **CAM_VSYNC_INT_ST**
    ///
    /// The status bit for Camera frame end interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CAM_VSYNC_INT_ST_OFFSET: u32 = 2;
    pub const CAM_VSYNC_INT_ST_MASK: u32 = 0x00000004;
    pub const CAM_VSYNC_INT_ST: u32 = 1 << 2;

    /// **LCD_TRANS_DONE_INT_ST**
    ///
    /// The status bit for lcd transfer end interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LCD_TRANS_DONE_INT_ST_OFFSET: u32 = 1;
    pub const LCD_TRANS_DONE_INT_ST_MASK: u32 = 0x00000002;
    pub const LCD_TRANS_DONE_INT_ST: u32 = 1 << 1;

    /// **LCD_VSYNC_INT_ST**
    ///
    /// The status bit for LCD frame end interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LCD_VSYNC_INT_ST_OFFSET: u32 = 0;
    pub const LCD_VSYNC_INT_ST_MASK: u32 = 0x00000001;
    pub const LCD_VSYNC_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CAM_HS_INT_ST** `[3]` (RO)
    ///
    /// The status bit for Camera transfer end interrupt.
    #[inline(always)]
    pub fn cam_hs_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CAM_HS_INT_ST_MASK) != 0
    }

    /// **CAM_VSYNC_INT_ST** `[2]` (RO)
    ///
    /// The status bit for Camera frame end interrupt.
    #[inline(always)]
    pub fn cam_vsync_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CAM_VSYNC_INT_ST_MASK) != 0
    }

    /// **LCD_TRANS_DONE_INT_ST** `[1]` (RO)
    ///
    /// The status bit for lcd transfer end interrupt.
    #[inline(always)]
    pub fn lcd_trans_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LCD_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// **LCD_VSYNC_INT_ST** `[0]` (RO)
    ///
    /// The status bit for LCD frame end interrupt.
    #[inline(always)]
    pub fn lcd_vsync_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LCD_VSYNC_INT_ST_MASK) != 0
    }
}

/// LC_DMA_INT_ST value type for batch read/modify/write operations
///
/// Use `lcdmaintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdmaintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcDmaIntStVal {
    bits: u32,
}

impl LcDmaIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CAM_HS_INT_ST** `[3:3]`
    ///
    /// The status bit for Camera transfer end interrupt.
    #[inline(always)]
    pub const fn cam_hs_int_st(&self) -> bool {
        (self.bits & LcDmaIntSt::CAM_HS_INT_ST_MASK) != 0
    }

    /// Read **CAM_VSYNC_INT_ST** `[2:2]`
    ///
    /// The status bit for Camera frame end interrupt.
    #[inline(always)]
    pub const fn cam_vsync_int_st(&self) -> bool {
        (self.bits & LcDmaIntSt::CAM_VSYNC_INT_ST_MASK) != 0
    }

    /// Read **LCD_TRANS_DONE_INT_ST** `[1:1]`
    ///
    /// The status bit for lcd transfer end interrupt.
    #[inline(always)]
    pub const fn lcd_trans_done_int_st(&self) -> bool {
        (self.bits & LcDmaIntSt::LCD_TRANS_DONE_INT_ST_MASK) != 0
    }

    /// Read **LCD_VSYNC_INT_ST** `[0:0]`
    ///
    /// The status bit for LCD frame end interrupt.
    #[inline(always)]
    pub const fn lcd_vsync_int_st(&self) -> bool {
        (self.bits & LcDmaIntSt::LCD_VSYNC_INT_ST_MASK) != 0
    }
}

impl Default for LcDmaIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcDmaIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcDmaIntStVal {
        LcDmaIntStVal::from_bits(self.value.read_bits())
    }
}

/// LC_DMA_INT_CLR Register
///
/// LCDCAM interrupt clear register.
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **CAM_HS_INT_CLR** `[3]` (WO)
///
///   The clear bit for Camera line interrupt.
///
/// - **CAM_VSYNC_INT_CLR** `[2]` (WO)
///
///   The clear bit for Camera frame end interrupt.
///
/// - **LCD_TRANS_DONE_INT_CLR** `[1]` (WO)
///
///   The clear bit for lcd transfer end interrupt.
///
/// - **LCD_VSYNC_INT_CLR** `[0]` (WO)
///
///   The clear bit for LCD frame end interrupt.
#[repr(transparent)]
pub struct LcDmaIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcDmaIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcDmaIntClr {
    // ======== Field Constants ========

    /// **CAM_HS_INT_CLR**
    ///
    /// The clear bit for Camera line interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CAM_HS_INT_CLR_OFFSET: u32 = 3;
    pub const CAM_HS_INT_CLR_MASK: u32 = 0x00000008;
    pub const CAM_HS_INT_CLR: u32 = 1 << 3;

    /// **CAM_VSYNC_INT_CLR**
    ///
    /// The clear bit for Camera frame end interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CAM_VSYNC_INT_CLR_OFFSET: u32 = 2;
    pub const CAM_VSYNC_INT_CLR_MASK: u32 = 0x00000004;
    pub const CAM_VSYNC_INT_CLR: u32 = 1 << 2;

    /// **LCD_TRANS_DONE_INT_CLR**
    ///
    /// The clear bit for lcd transfer end interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LCD_TRANS_DONE_INT_CLR_OFFSET: u32 = 1;
    pub const LCD_TRANS_DONE_INT_CLR_MASK: u32 = 0x00000002;
    pub const LCD_TRANS_DONE_INT_CLR: u32 = 1 << 1;

    /// **LCD_VSYNC_INT_CLR**
    ///
    /// The clear bit for LCD frame end interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LCD_VSYNC_INT_CLR_OFFSET: u32 = 0;
    pub const LCD_VSYNC_INT_CLR_MASK: u32 = 0x00000001;
    pub const LCD_VSYNC_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CAM_HS_INT_CLR** (WO)
    ///
    /// The clear bit for Camera line interrupt.
    #[inline(always)]
    pub fn trigger_cam_hs_int_clr(&self) {
        self.value.write_bits(Self::CAM_HS_INT_CLR_MASK);
    }

    /// Trigger **CAM_VSYNC_INT_CLR** (WO)
    ///
    /// The clear bit for Camera frame end interrupt.
    #[inline(always)]
    pub fn trigger_cam_vsync_int_clr(&self) {
        self.value.write_bits(Self::CAM_VSYNC_INT_CLR_MASK);
    }

    /// Trigger **LCD_TRANS_DONE_INT_CLR** (WO)
    ///
    /// The clear bit for lcd transfer end interrupt.
    #[inline(always)]
    pub fn trigger_lcd_trans_done_int_clr(&self) {
        self.value.write_bits(Self::LCD_TRANS_DONE_INT_CLR_MASK);
    }

    /// Trigger **LCD_VSYNC_INT_CLR** (WO)
    ///
    /// The clear bit for LCD frame end interrupt.
    #[inline(always)]
    pub fn trigger_lcd_vsync_int_clr(&self) {
        self.value.write_bits(Self::LCD_VSYNC_INT_CLR_MASK);
    }
}

/// LC_DMA_INT_CLR value type for batch read/modify/write operations
///
/// Use `lcdmaintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdmaintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcDmaIntClrVal {
    bits: u32,
}

impl LcDmaIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **CAM_HS_INT_CLR** `[3:3]`
    ///
    /// The clear bit for Camera line interrupt.
    #[inline(always)]
    pub const fn with_cam_hs_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntClr::CAM_HS_INT_CLR_MASK
            } else {
                self.bits & !LcDmaIntClr::CAM_HS_INT_CLR_MASK
            },
        }
    }

    /// Set **CAM_VSYNC_INT_CLR** `[2:2]`
    ///
    /// The clear bit for Camera frame end interrupt.
    #[inline(always)]
    pub const fn with_cam_vsync_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntClr::CAM_VSYNC_INT_CLR_MASK
            } else {
                self.bits & !LcDmaIntClr::CAM_VSYNC_INT_CLR_MASK
            },
        }
    }

    /// Set **LCD_TRANS_DONE_INT_CLR** `[1:1]`
    ///
    /// The clear bit for lcd transfer end interrupt.
    #[inline(always)]
    pub const fn with_lcd_trans_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntClr::LCD_TRANS_DONE_INT_CLR_MASK
            } else {
                self.bits & !LcDmaIntClr::LCD_TRANS_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **LCD_VSYNC_INT_CLR** `[0:0]`
    ///
    /// The clear bit for LCD frame end interrupt.
    #[inline(always)]
    pub const fn with_lcd_vsync_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LcDmaIntClr::LCD_VSYNC_INT_CLR_MASK
            } else {
                self.bits & !LcDmaIntClr::LCD_VSYNC_INT_CLR_MASK
            },
        }
    }
}

impl Default for LcDmaIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcDmaIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcDmaIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// LC_REG_DATE Register
///
/// Version register
///
/// Address offset: 0x00fc
/// Reset value: 0x02303090
///
/// ## Fields
///
/// - **LC_DATE** `[27:0]` (RW)
///
///   LCD_CAM version control register
#[repr(transparent)]
pub struct LcRegDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcRegDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcRegDate {
    // ======== Field Constants ========

    /// **LC_DATE**
    ///
    /// LCD_CAM version control register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const LC_DATE_OFFSET: u32 = 0;
    pub const LC_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **LC_DATE** `[27:0]` (RW)
    ///
    /// LCD_CAM version control register
    #[inline(always)]
    pub fn lc_date(&self) -> u32 {
        (self.value.read_bits() & Self::LC_DATE_MASK) >> Self::LC_DATE_OFFSET
    }

    /// Write **LC_DATE** `[27:0]` (RW)
    ///
    /// LCD_CAM version control register
    #[inline(always)]
    pub fn write_lc_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LC_DATE_MASK) | ((value << Self::LC_DATE_OFFSET) & Self::LC_DATE_MASK)
        });
    }
}

/// LC_REG_DATE value type for batch read/modify/write operations
///
/// Use `lcregdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcregdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcRegDateVal {
    bits: u32,
}

impl LcRegDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02303090 }
    }

    /// Read **LC_DATE** `[27:0]`
    ///
    /// LCD_CAM version control register
    #[inline(always)]
    pub const fn lc_date(&self) -> u32 {
        (self.bits & LcRegDate::LC_DATE_MASK) >> LcRegDate::LC_DATE_OFFSET
    }

    /// Set **LC_DATE** `[27:0]`
    ///
    /// LCD_CAM version control register
    #[inline(always)]
    pub const fn with_lc_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcRegDate::LC_DATE_MASK)
                | ((value << LcRegDate::LC_DATE_OFFSET) & LcRegDate::LC_DATE_MASK),
        }
    }
}

impl Default for LcRegDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LcRegDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcRegDateVal {
        LcRegDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcRegDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcRegDateVal) -> LcRegDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// LCD clock config register.
    pub lcd_clock: LcdClock,
    /// 0x0004
    /// CAM config register.
    pub cam_ctrl: CamCtrl,
    /// 0x0008
    /// CAM config register.
    pub cam_ctrl1: CamCtrl1,
    /// 0x000c
    /// CAM YUV/RGB converter configuration register.
    pub cam_rgb_yuv: CamRgbYuv,
    /// 0x0010
    /// LCD YUV/RGB converter configuration register.
    pub lcd_rgb_yuv: LcdRgbYuv,
    /// 0x0014
    /// LCD config register.
    pub lcd_user: LcdUser,
    /// 0x0018
    /// LCD config register.
    pub lcd_misc: LcdMisc,
    /// 0x001c
    /// LCD config register.
    pub lcd_ctrl: LcdCtrl,
    /// 0x0020
    /// LCD config register.
    pub lcd_ctrl1: LcdCtrl1,
    /// 0x0024
    /// LCD config register.
    pub lcd_ctrl2: LcdCtrl2,
    /// 0x0028
    /// LCD config register.
    pub lcd_first_cmd_val: LcdFirstCmdVal,
    /// 0x002c
    /// LCD config register.
    pub lcd_latter_cmd_val: LcdLatterCmdVal,
    /// 0x0030
    /// LCD config register.
    pub lcd_dly_mode_cfg1: LcdDlyModeCfg1,
    _reserved0: [u8; 4],
    /// 0x0038
    /// LCD config register.
    pub lcd_dly_mode_cfg2: LcdDlyModeCfg2,
    _reserved1: [u8; 40],
    /// 0x0064
    /// LCDCAM interrupt enable register.
    pub lc_dma_int_ena: LcDmaIntEna,
    /// 0x0068
    /// LCDCAM interrupt raw register, valid in level.
    pub lc_dma_int_raw: LcDmaIntRaw,
    /// 0x006c
    /// LCDCAM interrupt status register.
    pub lc_dma_int_st: LcDmaIntSt,
    /// 0x0070
    /// LCDCAM interrupt clear register.
    pub lc_dma_int_clr: LcDmaIntClr,
    _reserved2: [u8; 136],
    /// 0x00fc
    /// Version register
    pub lc_reg_date: LcRegDate,
}
