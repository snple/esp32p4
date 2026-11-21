//! Event task matrix (SOC_ETM).
//!
//! The ETM wires a peripheral *event* to a peripheral *task* without involving
//! the CPU: every channel holds one event id and one task id, and the channel's
//! enable bit arms it. The ids belong to the peripherals, so a driver exports
//! them — for example [`Mcpwm0::event_id`](crate::mcpwm::Mcpwm0::event_id) and
//! [`Mcpwm0::task_id`](crate::mcpwm::Mcpwm0::task_id) — and also has to enable
//! the event/task inside the peripheral itself (an MCPWM event only reaches the
//! matrix once its `EVT_EN` bit is set, and a task only takes effect once its
//! `TASK_EN` bit is).
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::mcpwm::{CaptureChannel, Event, Task, TimerIndex};
//!
//! let etm = Etm::init(dp.SOC_ETM, &dp.HP_SYS_CLKRST);
//! let mut channel = etm.free_channel().unwrap();
//!
//! // Every timer-zero event triggers a capture on channel 0.
//! mcpwm.enable_event(Event::TimerZero(TimerIndex::Zero), true);
//! mcpwm.enable_task(Task::Capture(CaptureChannel::Zero), true);
//! channel.connect(
//!     mcpwm.event_id(Event::TimerZero(TimerIndex::Zero)),
//!     mcpwm.task_id(Task::Capture(CaptureChannel::Zero)),
//! );
//! channel.enable(true);
//! ```
//!
//! # Hardware Notes
//!
//! - A channel is a plain trigger: it keeps no state beyond the two ids, so one
//!   event may feed several channels and several channels may drive one task.
//! - The `EVT_ST*`/`TASK_ST*` registers latch which signals fired, but their bit
//!   numbering follows the signal table (the `TEE1`/`TEE2` ids are interleaved
//!   between other groups) rather than the id, so this driver leaves them out:
//!   check the effect of the task instead.
//! - `CH_ENA_AD0`/`CH_ENA_AD1` enable channels 0–31 and 32–49. The PAC spells the
//!   channel registers out one field at a time, so this module addresses the
//!   block by its published layout, the same way [`crate::interrupt`] drives the
//!   interrupt matrix.

use core::ptr::{read_volatile, write_volatile};

use esp32p4_pac::{HP_SYS_CLKRST, SOC_ETM, soc_etm};

/// Number of channels in the matrix.
pub const CHANNELS: usize = 50;

/// Id of a peripheral event, as the ETM channel registers want it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EventId(u8);

impl EventId {
    /// Wraps a raw event id.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw id.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Id of a peripheral task, as the ETM channel registers want it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TaskId(u8);

impl TaskId {
    /// Wraps a raw task id.
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }

    /// The raw id.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

// Register offsets and strides inside `SOC_ETM`, from the PAC's layout:
// `CH_ENA_AD0` at 0x00 with its set/clear at 0x04/0x08, the second bank at
// 0x0c/0x10/0x14, channel `n`'s ids at 0x18 + 8n and 0x1c + 8n, the event
// and the trigger status banks start at 0x1a8 (`EVT_ST*`) and 0x1e8
// (`TASK_ST*`).
const CH_ENA_AD0: usize = 0x00;
const CH_ENA_AD0_SET: usize = 0x04;
const CH_ENA_AD0_CLR: usize = 0x08;
const CH_ENA_AD1: usize = 0x0c;
const CH_ENA_AD1_SET: usize = 0x10;
const CH_ENA_AD1_CLR: usize = 0x14;
const CH_EVT_ID: usize = 0x18;
const CH_TASK_ID: usize = 0x1c;
const CH_STRIDE: usize = 8;

fn reg(offset: usize) -> *mut u32 {
    (soc_etm::BASE as usize + offset) as *mut u32
}

/// Event task matrix, owning the `SOC_ETM` peripheral.
pub struct Etm {
    etm: SOC_ETM,
}

impl Etm {
    /// Enables the matrix: APB clock for the registers, the system clock for the
    /// triggers, releases the reset and forces the register gate open.
    pub fn init(etm: SOC_ETM, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl3.set_reg_etm_apb_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_etm();
        clkrst.hp_rst_en1.clr_reg_rst_en_etm();
        clkrst.soc_clk_ctrl1.set_reg_etm_sys_clk_en();
        etm.clk_en.set_clk_en();
        Self { etm }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the matrix was initialised and that the returned
    /// handle is not used to reconfigure it concurrently.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            etm: unsafe { SOC_ETM::steal() },
        })
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> SOC_ETM {
        self.etm
    }

    /// The channel with this index, or `None` when it is out of range.
    #[inline]
    pub fn channel(&self, index: usize) -> Option<Channel> {
        (index < CHANNELS).then_some(Channel { index })
    }

    /// The first channel that is not armed.
    pub fn free_channel(&self) -> Option<Channel> {
        (0..CHANNELS)
            .map(|index| Channel { index })
            .find(|channel| !channel.is_enabled())
    }
}

/// One channel of the matrix: an event id, a task id and an enable bit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Channel {
    index: usize,
}

impl Channel {
    /// Channel index, 0–49.
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Points the channel at an event source and a task sink.
    ///
    /// Writing the ids does not arm the channel; call [`enable`](Self::enable)
    /// once both peripherals are ready.
    pub fn connect(&mut self, event: EventId, task: TaskId) {
        let base = CH_EVT_ID + CH_STRIDE * self.index;
        unsafe {
            write_volatile(reg(base), u32::from(event.raw()));
            write_volatile(reg(base + 4), u32::from(task.raw()));
        }
    }

    /// The event id the channel listens to.
    pub fn event(&self) -> EventId {
        let base = CH_EVT_ID + CH_STRIDE * self.index;
        EventId(unsafe { read_volatile(reg(base)) } as u8)
    }

    /// The task id the channel drives.
    pub fn task(&self) -> TaskId {
        let base = CH_TASK_ID + CH_STRIDE * self.index;
        TaskId(unsafe { read_volatile(reg(base)) } as u8)
    }

    /// Arms or disarms the channel.
    pub fn enable(&mut self, enable: bool) {
        let (offset, bit) = if self.index < 32 {
            let offset = match enable {
                true => CH_ENA_AD0_SET,
                false => CH_ENA_AD0_CLR,
            };
            (offset, self.index)
        } else {
            let offset = match enable {
                true => CH_ENA_AD1_SET,
                false => CH_ENA_AD1_CLR,
            };
            (offset, self.index - 32)
        };
        unsafe { write_volatile(reg(offset), 1 << bit) };
    }

    /// Whether the channel is armed.
    pub fn is_enabled(&self) -> bool {
        let (offset, bit) = if self.index < 32 {
            (CH_ENA_AD0, self.index)
        } else {
            (CH_ENA_AD1, self.index - 32)
        };
        unsafe { read_volatile(reg(offset)) & (1 << bit) != 0 }
    }
}
