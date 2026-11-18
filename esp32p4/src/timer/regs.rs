//! Register access for one general-purpose timer.
//!
//! `esp32p4-pac` cannot describe these registers: the vendor SVD has timer 0 of
//! each group but leaves timer 1 in a reserved hole, and the two timers are the
//! same nine registers 0x24 bytes apart (ESP-IDF models them as
//! `timg_hwtimer_reg_t hw_timer[2]`). The offsets and bit positions below are
//! from chapter 18 of the TRM and match ESP-IDF's
//! `soc/esp32p4/register/hw_ver1/soc/timer_group_struct.h`.
//!
//! Only the registers a general-purpose timer needs are here; the watchdog, the
//! RTC calibration and the version registers stay untouched.

use esp32p4_pac::common::VolatileCell;

/// Base address of `TIMG0`.
pub(crate) const GROUP0_BASE: usize = 0x500c_2000;
/// Base address of `TIMG1`.
pub(crate) const GROUP1_BASE: usize = 0x500c_3000;

/// Distance between the two timers of a group.
const UNIT_STRIDE: usize = 0x24;

// Timer registers, relative to the timer's own window.
const CONFIG: usize = 0x00;
const LO: usize = 0x04;
const HI: usize = 0x08;
const UPDATE: usize = 0x0c;
const ALARM_LO: usize = 0x10;
const ALARM_HI: usize = 0x14;
const LOAD_LO: usize = 0x18;
const LOAD_HI: usize = 0x1c;
const LOAD: usize = 0x20;

// Group-wide registers, shared by both timers and the watchdog.
const INT_ENA: usize = 0x70;
const INT_RAW: usize = 0x74;
const INT_CLR: usize = 0x7c;

/// `CONFIG.EN`: the counter runs.
const CONFIG_EN: u32 = 1 << 31;
/// `CONFIG.INCREASE`: count up instead of down.
const CONFIG_INCREASE: u32 = 1 << 30;
/// `CONFIG.AUTORELOAD`: reload the counter when the alarm fires.
const CONFIG_AUTORELOAD: u32 = 1 << 29;
/// `CONFIG.DIVIDER`: 16-bit prescaler, `0` meaning 65536.
const CONFIG_DIVIDER_SHIFT: u32 = 13;
const CONFIG_DIVIDER_MASK: u32 = 0x1fffe000;
/// `CONFIG.DIVCNT_RST`: reset the prescaler's own counter.
const CONFIG_DIVCNT_RST: u32 = 1 << 12;
/// `CONFIG.ALARM_EN`: arm the alarm; cleared by hardware when it fires.
const CONFIG_ALARM_EN: u32 = 1 << 10;

/// Widest counter and alarm value: 22 high bits and 32 low bits.
pub(crate) const MAX_VALUE: u64 = (1 << 54) - 1;

#[inline(always)]
fn reg(address: usize) -> &'static VolatileCell<u32> {
    // SAFETY: every address handed to this function is inside a `TIMG` block,
    // whose registers are 32-bit and live for the whole program.
    unsafe { &*(address as *const VolatileCell<u32>) }
}

/// The registers of one general-purpose timer.
#[derive(Clone, Copy)]
pub(crate) struct Regs {
    timer: usize,
    group: usize,
    unit: u32,
}

impl Regs {
    /// The registers of timer `unit` (0 or 1) of the group at `group_base`.
    pub(crate) const fn new(group_base: usize, unit: u8) -> Self {
        Self {
            timer: group_base + unit as usize * UNIT_STRIDE,
            group: group_base,
            unit: unit as u32,
        }
    }

    /// The registers of timer `index`, counted as `group * 2 + unit`.
    pub(crate) const fn at(index: usize) -> Self {
        let base = if index < 2 { GROUP0_BASE } else { GROUP1_BASE };
        Self::new(base, (index % 2) as u8)
    }

    #[inline(always)]
    fn read(&self, offset: usize) -> u32 {
        reg(self.timer + offset).read_bits()
    }

    #[inline(always)]
    fn write(&self, offset: usize, value: u32) {
        reg(self.timer + offset).write_bits(value);
    }

    #[inline(always)]
    fn modify(&self, offset: usize, f: impl FnOnce(u32) -> u32) {
        reg(self.timer + offset).modify_bits(f);
    }

    /// Start or stop the time-base counter.
    pub(crate) fn set_running(&self, running: bool) {
        self.modify(CONFIG, |v| {
            if running {
                v | CONFIG_EN
            } else {
                v & !CONFIG_EN
            }
        });
    }

    /// Whether the time-base counter is running.
    pub(crate) fn running(&self) -> bool {
        self.read(CONFIG) & CONFIG_EN != 0
    }

    /// Count up (true) or down (false).
    pub(crate) fn set_counting_up(&self, up: bool) {
        self.modify(CONFIG, |v| {
            if up {
                v | CONFIG_INCREASE
            } else {
                v & !CONFIG_INCREASE
            }
        });
    }

    /// Whether the counter counts up.
    pub(crate) fn counting_up(&self) -> bool {
        self.read(CONFIG) & CONFIG_INCREASE != 0
    }

    /// Reload the counter from the reload value when the alarm fires.
    pub(crate) fn set_auto_reload(&self, enable: bool) {
        self.modify(CONFIG, |v| {
            if enable {
                v | CONFIG_AUTORELOAD
            } else {
                v & !CONFIG_AUTORELOAD
            }
        });
    }

    /// Whether the counter reloads itself when the alarm fires.
    pub(crate) fn auto_reload(&self) -> bool {
        self.read(CONFIG) & CONFIG_AUTORELOAD != 0
    }

    /// Program the prescaler. The counter must be stopped.
    ///
    /// The divider runs from 2 to 65536 inclusive; 65536 is encoded as zero.
    pub(crate) fn set_divider(&self, divider: u32) {
        debug_assert!(!self.running(), "the prescaler needs a stopped counter");
        debug_assert!((2..=65536).contains(&divider), "divider out of range");
        let encoded = if divider >= 65536 { 0 } else { divider };

        // The TRM asks for the divisor first and the reset of the prescaler's
        // counter second, so the two land in separate writes.
        self.modify(CONFIG, |v| {
            (v & !CONFIG_DIVIDER_MASK) | (encoded << CONFIG_DIVIDER_SHIFT)
        });
        self.modify(CONFIG, |v| v | CONFIG_DIVCNT_RST);
    }

    /// Arm or disarm the alarm comparator.
    pub(crate) fn set_alarm_enabled(&self, enable: bool) {
        self.modify(CONFIG, |v| {
            if enable {
                v | CONFIG_ALARM_EN
            } else {
                v & !CONFIG_ALARM_EN
            }
        });
    }

    /// The programmed prescaler divisor, 2 to 65536.
    pub(crate) fn divider(&self) -> u32 {
        let encoded = (self.read(CONFIG) & CONFIG_DIVIDER_MASK) >> CONFIG_DIVIDER_SHIFT;
        if encoded == 0 { 65536 } else { encoded }
    }

    /// Whether the alarm is armed.
    pub(crate) fn alarm_enabled(&self) -> bool {
        self.read(CONFIG) & CONFIG_ALARM_EN != 0
    }

    /// Latch and return the 54-bit counter value.
    pub(crate) fn counter(&self) -> u64 {
        self.write(UPDATE, 1);
        while self.read(UPDATE) & 1 != 0 {
            core::hint::spin_loop();
        }
        let hi = self.read(HI) as u64;
        let lo = self.read(LO) as u64;
        (hi << 32) | lo
    }

    /// Write the reload value used by [`Regs::reload`] and by auto-reload.
    ///
    /// Writing it does not change the counter; only a reload event does.
    pub(crate) fn set_reload_value(&self, value: u64) {
        let value = value & MAX_VALUE;
        self.write(LOAD_HI, (value >> 32) as u32);
        self.write(LOAD_LO, value as u32);
    }

    /// Copy the reload value into the counter.
    pub(crate) fn reload(&self) {
        self.write(LOAD, 1);
    }

    /// Set the alarm value. It only takes effect once the alarm is armed.
    pub(crate) fn set_alarm_value(&self, value: u64) {
        let value = value & MAX_VALUE;
        self.write(ALARM_HI, (value >> 32) as u32);
        self.write(ALARM_LO, value as u32);
    }

    /// Unmask or mask this timer's alarm interrupt.
    pub(crate) fn enable_interrupt(&self, enable: bool) {
        let bit = 1 << self.unit;
        let address = self.group + INT_ENA;
        reg(address).modify_bits(|v| if enable { v | bit } else { v & !bit });
    }

    /// Whether this timer's alarm interrupt is unmasked.
    pub(crate) fn interrupt_enabled(&self) -> bool {
        reg(self.group + INT_ENA).read_bits() & (1 << self.unit) != 0
    }

    /// Whether the alarm has fired since the last clear, masked or not.
    pub(crate) fn interrupt_pending(&self) -> bool {
        reg(self.group + INT_RAW).read_bits() & (1 << self.unit) != 0
    }

    /// Drop the alarm request. `INT_CLR` is write-one-to-clear.
    pub(crate) fn clear_interrupt(&self) {
        reg(self.group + INT_CLR).write_bits(1 << self.unit);
    }
}
