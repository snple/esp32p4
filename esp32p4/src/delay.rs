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

/// Reads the raw counter into the latch and returns it.
#[inline]
fn read_ticks(systimer: &SYSTIMER) -> u64 {
    // Latch the counter into UNIT0_VALUE_HI/LO and wait for VALUE_VALID.
    systimer.unit0_op.trigger_timer_unit0_update();
    while !systimer.unit0_op.timer_unit0_value_valid() {
        core::hint::spin_loop();
    }

    let hi = systimer.unit0_value_hi.read_bits() as u64;
    let lo = systimer.unit0_value_lo.read_bits() as u64;
    (hi << 32) | lo
}

/// Microseconds since boot, readable without owning the timer.
///
/// Handy as a `defmt` timestamp: `defmt::timestamp!("{=u64:us}", micros());`
#[inline]
pub fn micros() -> u64 {
    Delay::init(unsafe { SYSTIMER::steal() }).now_us()
}

impl Delay {
    /// Create a delay driver.
    ///
    /// Unit 0 is left free-running: the stall-on-core-halt bits are cleared so
    /// the counter keeps advancing while the CPU is idle.
    #[inline]
    pub fn init(systimer: SYSTIMER) -> Self {
        // The bootloader normally leaves the timer running, but a program that
        // was reset by software can find the clock gate closed.
        systimer.conf.set_clk_en();
        systimer.conf.set_timer_unit0_work_en();
        systimer.conf.clr_timer_unit0_core0_stall_en();
        systimer.conf.clr_timer_unit0_core1_stall_en();
        Self { systimer }
    }

    /// Read the current 52-bit counter value.
    #[inline]
    fn ticks(&self) -> u64 {
        read_ticks(&self.systimer)
    }

    /// Microseconds since boot.
    #[inline]
    pub fn now_us(&self) -> u64 {
        self.ticks() / TICKS_PER_US
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
