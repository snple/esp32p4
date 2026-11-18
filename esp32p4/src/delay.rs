//! Blocking delays using the system timer.
//!
//! The ESP32-P4 `SYSTIMER` counts at an average of 16 MHz
//! (`XTAL_CLK / 2.5`), so one microsecond is 16 counter ticks.

use esp32p4_pac::SYSTIMER;

/// System timer ticks per microsecond.
const TICKS_PER_US: u64 = 16;

/// Blocking delay driver backed by `SYSTIMER` unit 0.
pub struct Delay {
    systimer: SYSTIMER,
}

impl Delay {
    /// Create a delay driver.
    ///
    /// Unit 0 is left free-running: the stall-on-core-halt bits are cleared so
    /// the counter keeps advancing while the CPU is idle.
    #[inline]
    pub fn init(systimer: SYSTIMER) -> Self {
        systimer.conf.set_timer_unit0_work_en();
        systimer.conf.clr_timer_unit0_core0_stall_en();
        systimer.conf.clr_timer_unit0_core1_stall_en();
        Self { systimer }
    }

    /// Read the current 52-bit counter value.
    #[inline]
    fn ticks(&self) -> u64 {
        // Latch the counter into UNIT0_VALUE_HI/LO and wait for VALUE_VALID.
        self.systimer.unit0_op.trigger_timer_unit0_update();
        while !self.systimer.unit0_op.timer_unit0_value_valid() {
            core::hint::spin_loop();
        }

        let hi = self.systimer.unit0_value_hi.read_bits() as u64;
        let lo = self.systimer.unit0_value_lo.read_bits() as u64;
        (hi << 32) | lo
    }

    /// Wait for at least `us` microseconds.
    #[inline]
    pub fn delay_us(&self, us: u32) {
        let start = self.ticks();
        let target = us as u64 * TICKS_PER_US;
        while self.ticks().wrapping_sub(start) < target {
            core::hint::spin_loop();
        }
    }

    /// Wait for at least `ms` milliseconds.
    #[inline]
    pub fn delay_ms(&self, ms: u32) {
        let start = self.ticks();
        let target = ms as u64 * 1000 * TICKS_PER_US;
        while self.ticks().wrapping_sub(start) < target {
            core::hint::spin_loop();
        }
    }

    /// Release the underlying `SYSTIMER` peripheral.
    #[inline]
    pub fn release(self) -> SYSTIMER {
        self.systimer
    }
}
