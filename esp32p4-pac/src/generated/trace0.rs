//! TRACE0
//! TRACE0 Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x3ff04000;

/// MEM_START_ADDR Register
///
/// mem start addr
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **MEM_START_ADDR** `[31:0]` (RW)
///
///   The start address of trace memory
#[repr(transparent)]
pub struct MemStartAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemStartAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemStartAddr {
    // ======== Field Constants ========

    /// **MEM_START_ADDR**
    ///
    /// The start address of trace memory
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MEM_START_ADDR_OFFSET: u32 = 0;
    pub const MEM_START_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MEM_START_ADDR** `[31:0]` (RW)
    ///
    /// The start address of trace memory
    #[inline(always)]
    pub fn mem_start_addr(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_START_ADDR_MASK) >> Self::MEM_START_ADDR_OFFSET
    }

    /// Write **MEM_START_ADDR** `[31:0]` (RW)
    ///
    /// The start address of trace memory
    #[inline(always)]
    pub fn write_mem_start_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_START_ADDR_MASK)
                | ((value << Self::MEM_START_ADDR_OFFSET) & Self::MEM_START_ADDR_MASK)
        });
    }
}

/// MEM_START_ADDR value type for batch read/modify/write operations
///
/// Use `memstartaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memstartaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemStartAddrVal {
    bits: u32,
}

impl MemStartAddrVal {
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

    /// Read **MEM_START_ADDR** `[31:0]`
    ///
    /// The start address of trace memory
    #[inline(always)]
    pub const fn mem_start_addr(&self) -> u32 {
        (self.bits & MemStartAddr::MEM_START_ADDR_MASK) >> MemStartAddr::MEM_START_ADDR_OFFSET
    }

    /// Set **MEM_START_ADDR** `[31:0]`
    ///
    /// The start address of trace memory
    #[inline(always)]
    pub const fn with_mem_start_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MemStartAddr::MEM_START_ADDR_MASK)
                | ((value << MemStartAddr::MEM_START_ADDR_OFFSET)
                    & MemStartAddr::MEM_START_ADDR_MASK),
        }
    }
}

impl Default for MemStartAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemStartAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemStartAddrVal {
        MemStartAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemStartAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemStartAddrVal) -> MemStartAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_END_ADDR Register
///
/// mem end addr
///
/// Address offset: 0x0004
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **MEM_END_ADDR** `[31:0]` (RW)
///
///   The end address of trace memory
#[repr(transparent)]
pub struct MemEndAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemEndAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemEndAddr {
    // ======== Field Constants ========

    /// **MEM_END_ADDR**
    ///
    /// The end address of trace memory
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MEM_END_ADDR_OFFSET: u32 = 0;
    pub const MEM_END_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MEM_END_ADDR** `[31:0]` (RW)
    ///
    /// The end address of trace memory
    #[inline(always)]
    pub fn mem_end_addr(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_END_ADDR_MASK) >> Self::MEM_END_ADDR_OFFSET
    }

    /// Write **MEM_END_ADDR** `[31:0]` (RW)
    ///
    /// The end address of trace memory
    #[inline(always)]
    pub fn write_mem_end_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_END_ADDR_MASK)
                | ((value << Self::MEM_END_ADDR_OFFSET) & Self::MEM_END_ADDR_MASK)
        });
    }
}

/// MEM_END_ADDR value type for batch read/modify/write operations
///
/// Use `memendaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memendaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemEndAddrVal {
    bits: u32,
}

impl MemEndAddrVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **MEM_END_ADDR** `[31:0]`
    ///
    /// The end address of trace memory
    #[inline(always)]
    pub const fn mem_end_addr(&self) -> u32 {
        (self.bits & MemEndAddr::MEM_END_ADDR_MASK) >> MemEndAddr::MEM_END_ADDR_OFFSET
    }

    /// Set **MEM_END_ADDR** `[31:0]`
    ///
    /// The end address of trace memory
    #[inline(always)]
    pub const fn with_mem_end_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MemEndAddr::MEM_END_ADDR_MASK)
                | ((value << MemEndAddr::MEM_END_ADDR_OFFSET) & MemEndAddr::MEM_END_ADDR_MASK),
        }
    }
}

impl Default for MemEndAddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MemEndAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemEndAddrVal {
        MemEndAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemEndAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemEndAddrVal) -> MemEndAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_CURRENT_ADDR Register
///
/// mem current addr
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **MEM_CURRENT_ADDR** `[31:0]` (RO)
///
///   current_mem_addr,indicate that next writing addr
#[repr(transparent)]
pub struct MemCurrentAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemCurrentAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemCurrentAddr {
    // ======== Field Constants ========

    /// **MEM_CURRENT_ADDR**
    ///
    /// current_mem_addr,indicate that next writing addr
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MEM_CURRENT_ADDR_OFFSET: u32 = 0;
    pub const MEM_CURRENT_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MEM_CURRENT_ADDR** `[31:0]` (RO)
    ///
    /// current_mem_addr,indicate that next writing addr
    #[inline(always)]
    pub fn mem_current_addr(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_CURRENT_ADDR_MASK) >> Self::MEM_CURRENT_ADDR_OFFSET
    }
}

/// MEM_CURRENT_ADDR value type for batch read/modify/write operations
///
/// Use `memcurrentaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memcurrentaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemCurrentAddrVal {
    bits: u32,
}

impl MemCurrentAddrVal {
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

    /// Read **MEM_CURRENT_ADDR** `[31:0]`
    ///
    /// current_mem_addr,indicate that next writing addr
    #[inline(always)]
    pub const fn mem_current_addr(&self) -> u32 {
        (self.bits & MemCurrentAddr::MEM_CURRENT_ADDR_MASK)
            >> MemCurrentAddr::MEM_CURRENT_ADDR_OFFSET
    }
}

impl Default for MemCurrentAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemCurrentAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemCurrentAddrVal {
        MemCurrentAddrVal::from_bits(self.value.read_bits())
    }
}

/// MEM_ADDR_UPDATE Register
///
/// mem addr update
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **MEM_CURRENT_ADDR_UPDATE** `[0]` (WO)
///
///   when set, the will \hyperref[fielddesc:TRACEMEMCURRENTADDR]{TRACE_MEM_CURRENT_ADDR} update to \hyperref[fielddesc:TRACEMEMSTARTADDR]{TRACE_MEM_START_ADDR}.
#[repr(transparent)]
pub struct MemAddrUpdate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemAddrUpdate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemAddrUpdate {
    // ======== Field Constants ========

    /// **MEM_CURRENT_ADDR_UPDATE**
    ///
    /// when set, the will \hyperref[fielddesc:TRACEMEMCURRENTADDR]{TRACE_MEM_CURRENT_ADDR} update to \hyperref[fielddesc:TRACEMEMSTARTADDR]{TRACE_MEM_START_ADDR}.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MEM_CURRENT_ADDR_UPDATE_OFFSET: u32 = 0;
    pub const MEM_CURRENT_ADDR_UPDATE_MASK: u32 = 0x00000001;
    pub const MEM_CURRENT_ADDR_UPDATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **MEM_CURRENT_ADDR_UPDATE** (WO)
    ///
    /// when set, the will \hyperref[fielddesc:TRACEMEMCURRENTADDR]{TRACE_MEM_CURRENT_ADDR} update to \hyperref[fielddesc:TRACEMEMSTARTADDR]{TRACE_MEM_START_ADDR}.
    #[inline(always)]
    pub fn trigger_mem_current_addr_update(&self) {
        self.value.write_bits(Self::MEM_CURRENT_ADDR_UPDATE_MASK);
    }
}

/// MEM_ADDR_UPDATE value type for batch read/modify/write operations
///
/// Use `memaddrupdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memaddrupdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemAddrUpdateVal {
    bits: u32,
}

impl MemAddrUpdateVal {
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

    /// Set **MEM_CURRENT_ADDR_UPDATE** `[0:0]`
    ///
    /// when set, the will \hyperref[fielddesc:TRACEMEMCURRENTADDR]{TRACE_MEM_CURRENT_ADDR} update to \hyperref[fielddesc:TRACEMEMSTARTADDR]{TRACE_MEM_START_ADDR}.
    #[inline(always)]
    pub const fn with_mem_current_addr_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemAddrUpdate::MEM_CURRENT_ADDR_UPDATE_MASK
            } else {
                self.bits & !MemAddrUpdate::MEM_CURRENT_ADDR_UPDATE_MASK
            },
        }
    }
}

impl Default for MemAddrUpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemAddrUpdate {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemAddrUpdateVal) {
        self.value.write_bits(val.bits())
    }
}

/// FIFO_STATUS Register
///
/// fifo status register
///
/// Address offset: 0x0010
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **WORK_STATUS** `[2:1]` (RO)
///
///   Represent trace work status: \\0: idle state \\1: working state\\ 2: wait state due to hart halted or havereset \\3: lost state
///
/// - **FIFO_EMPTY** `[0]` (RO)
///
///   Represent whether the fifo is empty. \\1: empty \\0: not empty
#[repr(transparent)]
pub struct FifoStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoStatus {
    // ======== Field Constants ========

    /// **WORK_STATUS**
    ///
    /// Represent trace work status: \\0: idle state \\1: working state\\ 2: wait state due to hart halted or havereset \\3: lost state
    ///
    /// Bits `[2:1]`, mask `0x00000006`
    pub const WORK_STATUS_OFFSET: u32 = 1;
    pub const WORK_STATUS_MASK: u32 = 0x00000006;

    /// **FIFO_EMPTY**
    ///
    /// Represent whether the fifo is empty. \\1: empty \\0: not empty
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FIFO_EMPTY_OFFSET: u32 = 0;
    pub const FIFO_EMPTY_MASK: u32 = 0x00000001;
    pub const FIFO_EMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WORK_STATUS** `[2:1]` (RO)
    ///
    /// Represent trace work status: \\0: idle state \\1: working state\\ 2: wait state due to hart halted or havereset \\3: lost state
    #[inline(always)]
    pub fn work_status(&self) -> u32 {
        (self.value.read_bits() & Self::WORK_STATUS_MASK) >> Self::WORK_STATUS_OFFSET
    }

    /// **FIFO_EMPTY** `[0]` (RO)
    ///
    /// Represent whether the fifo is empty. \\1: empty \\0: not empty
    #[inline(always)]
    pub fn fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_EMPTY_MASK) != 0
    }
}

/// FIFO_STATUS value type for batch read/modify/write operations
///
/// Use `fifostatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifostatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoStatusVal {
    bits: u32,
}

impl FifoStatusVal {
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

    /// Read **WORK_STATUS** `[2:1]`
    ///
    /// Represent trace work status: \\0: idle state \\1: working state\\ 2: wait state due to hart halted or havereset \\3: lost state
    #[inline(always)]
    pub const fn work_status(&self) -> u32 {
        (self.bits & FifoStatus::WORK_STATUS_MASK) >> FifoStatus::WORK_STATUS_OFFSET
    }

    /// Read **FIFO_EMPTY** `[0:0]`
    ///
    /// Represent whether the fifo is empty. \\1: empty \\0: not empty
    #[inline(always)]
    pub const fn fifo_empty(&self) -> bool {
        (self.bits & FifoStatus::FIFO_EMPTY_MASK) != 0
    }
}

impl Default for FifoStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FifoStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoStatusVal {
        FifoStatusVal::from_bits(self.value.read_bits())
    }
}

/// INTR_ENA Register
///
/// interrupt enable register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **MEM_FULL_INTR_ENA** `[1]` (RW)
///
///   Set 1 enable mem_full interrupt
///
/// - **FIFO_OVERFLOW_INTR_ENA** `[0]` (RW)
///
///   Set 1 enable fifo_overflow interrupt
#[repr(transparent)]
pub struct IntrEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrEna {
    // ======== Field Constants ========

    /// **MEM_FULL_INTR_ENA**
    ///
    /// Set 1 enable mem_full interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_FULL_INTR_ENA_OFFSET: u32 = 1;
    pub const MEM_FULL_INTR_ENA_MASK: u32 = 0x00000002;
    pub const MEM_FULL_INTR_ENA: u32 = 1 << 1;

    /// **FIFO_OVERFLOW_INTR_ENA**
    ///
    /// Set 1 enable fifo_overflow interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FIFO_OVERFLOW_INTR_ENA_OFFSET: u32 = 0;
    pub const FIFO_OVERFLOW_INTR_ENA_MASK: u32 = 0x00000001;
    pub const FIFO_OVERFLOW_INTR_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM_FULL_INTR_ENA** `[1]` (RW)
    ///
    /// Set 1 enable mem_full interrupt
    #[inline(always)]
    pub fn mem_full_intr_ena(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_INTR_ENA_MASK) != 0
    }

    /// Set MEM_FULL_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_full_intr_ena(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FULL_INTR_ENA_MASK);
    }

    /// Clear MEM_FULL_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_full_intr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_FULL_INTR_ENA_MASK);
    }

    /// **FIFO_OVERFLOW_INTR_ENA** `[0]` (RW)
    ///
    /// Set 1 enable fifo_overflow interrupt
    #[inline(always)]
    pub fn fifo_overflow_intr_ena(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_OVERFLOW_INTR_ENA_MASK) != 0
    }

    /// Set FIFO_OVERFLOW_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fifo_overflow_intr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::FIFO_OVERFLOW_INTR_ENA_MASK);
    }

    /// Clear FIFO_OVERFLOW_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fifo_overflow_intr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::FIFO_OVERFLOW_INTR_ENA_MASK);
    }
}

/// INTR_ENA value type for batch read/modify/write operations
///
/// Use `intrena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrEnaVal {
    bits: u32,
}

impl IntrEnaVal {
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

    /// Read **MEM_FULL_INTR_ENA** `[1:1]`
    ///
    /// Set 1 enable mem_full interrupt
    #[inline(always)]
    pub const fn mem_full_intr_ena(&self) -> bool {
        (self.bits & IntrEna::MEM_FULL_INTR_ENA_MASK) != 0
    }

    /// Read **FIFO_OVERFLOW_INTR_ENA** `[0:0]`
    ///
    /// Set 1 enable fifo_overflow interrupt
    #[inline(always)]
    pub const fn fifo_overflow_intr_ena(&self) -> bool {
        (self.bits & IntrEna::FIFO_OVERFLOW_INTR_ENA_MASK) != 0
    }

    /// Set **MEM_FULL_INTR_ENA** `[1:1]`
    ///
    /// Set 1 enable mem_full interrupt
    #[inline(always)]
    pub const fn with_mem_full_intr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntrEna::MEM_FULL_INTR_ENA_MASK
            } else {
                self.bits & !IntrEna::MEM_FULL_INTR_ENA_MASK
            },
        }
    }

    /// Set **FIFO_OVERFLOW_INTR_ENA** `[0:0]`
    ///
    /// Set 1 enable fifo_overflow interrupt
    #[inline(always)]
    pub const fn with_fifo_overflow_intr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntrEna::FIFO_OVERFLOW_INTR_ENA_MASK
            } else {
                self.bits & !IntrEna::FIFO_OVERFLOW_INTR_ENA_MASK
            },
        }
    }
}

impl Default for IntrEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrEnaVal {
        IntrEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntrEnaVal) -> IntrEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR_RAW Register
///
/// interrupt status register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **MEM_FULL_INTR_RAW** `[1]` (RO)
///
///   mem_full interrupt status
///
/// - **FIFO_OVERFLOW_INTR_RAW** `[0]` (RO)
///
///   fifo_overflow interrupt status
#[repr(transparent)]
pub struct IntrRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrRaw {
    // ======== Field Constants ========

    /// **MEM_FULL_INTR_RAW**
    ///
    /// mem_full interrupt status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_FULL_INTR_RAW_OFFSET: u32 = 1;
    pub const MEM_FULL_INTR_RAW_MASK: u32 = 0x00000002;
    pub const MEM_FULL_INTR_RAW: u32 = 1 << 1;

    /// **FIFO_OVERFLOW_INTR_RAW**
    ///
    /// fifo_overflow interrupt status
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FIFO_OVERFLOW_INTR_RAW_OFFSET: u32 = 0;
    pub const FIFO_OVERFLOW_INTR_RAW_MASK: u32 = 0x00000001;
    pub const FIFO_OVERFLOW_INTR_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM_FULL_INTR_RAW** `[1]` (RO)
    ///
    /// mem_full interrupt status
    #[inline(always)]
    pub fn mem_full_intr_raw(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FULL_INTR_RAW_MASK) != 0
    }

    /// **FIFO_OVERFLOW_INTR_RAW** `[0]` (RO)
    ///
    /// fifo_overflow interrupt status
    #[inline(always)]
    pub fn fifo_overflow_intr_raw(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_OVERFLOW_INTR_RAW_MASK) != 0
    }
}

/// INTR_RAW value type for batch read/modify/write operations
///
/// Use `intrraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrRawVal {
    bits: u32,
}

impl IntrRawVal {
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

    /// Read **MEM_FULL_INTR_RAW** `[1:1]`
    ///
    /// mem_full interrupt status
    #[inline(always)]
    pub const fn mem_full_intr_raw(&self) -> bool {
        (self.bits & IntrRaw::MEM_FULL_INTR_RAW_MASK) != 0
    }

    /// Read **FIFO_OVERFLOW_INTR_RAW** `[0:0]`
    ///
    /// fifo_overflow interrupt status
    #[inline(always)]
    pub const fn fifo_overflow_intr_raw(&self) -> bool {
        (self.bits & IntrRaw::FIFO_OVERFLOW_INTR_RAW_MASK) != 0
    }
}

impl Default for IntrRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrRawVal {
        IntrRawVal::from_bits(self.value.read_bits())
    }
}

/// INTR_CLR Register
///
/// interrupt clear register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **MEM_FULL_INTR_CLR** `[1]` (WO)
///
///   Set 1 clear mem full interrupt
///
/// - **FIFO_OVERFLOW_INTR_CLR** `[0]` (WO)
///
///   Set 1 clear fifo overflow interrupt
#[repr(transparent)]
pub struct IntrClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrClr {
    // ======== Field Constants ========

    /// **MEM_FULL_INTR_CLR**
    ///
    /// Set 1 clear mem full interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_FULL_INTR_CLR_OFFSET: u32 = 1;
    pub const MEM_FULL_INTR_CLR_MASK: u32 = 0x00000002;
    pub const MEM_FULL_INTR_CLR: u32 = 1 << 1;

    /// **FIFO_OVERFLOW_INTR_CLR**
    ///
    /// Set 1 clear fifo overflow interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FIFO_OVERFLOW_INTR_CLR_OFFSET: u32 = 0;
    pub const FIFO_OVERFLOW_INTR_CLR_MASK: u32 = 0x00000001;
    pub const FIFO_OVERFLOW_INTR_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **MEM_FULL_INTR_CLR** (WO)
    ///
    /// Set 1 clear mem full interrupt
    #[inline(always)]
    pub fn trigger_mem_full_intr_clr(&self) {
        self.value.write_bits(Self::MEM_FULL_INTR_CLR_MASK);
    }

    /// Trigger **FIFO_OVERFLOW_INTR_CLR** (WO)
    ///
    /// Set 1 clear fifo overflow interrupt
    #[inline(always)]
    pub fn trigger_fifo_overflow_intr_clr(&self) {
        self.value.write_bits(Self::FIFO_OVERFLOW_INTR_CLR_MASK);
    }
}

/// INTR_CLR value type for batch read/modify/write operations
///
/// Use `intrclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrClrVal {
    bits: u32,
}

impl IntrClrVal {
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

    /// Set **MEM_FULL_INTR_CLR** `[1:1]`
    ///
    /// Set 1 clear mem full interrupt
    #[inline(always)]
    pub const fn with_mem_full_intr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntrClr::MEM_FULL_INTR_CLR_MASK
            } else {
                self.bits & !IntrClr::MEM_FULL_INTR_CLR_MASK
            },
        }
    }

    /// Set **FIFO_OVERFLOW_INTR_CLR** `[0:0]`
    ///
    /// Set 1 clear fifo overflow interrupt
    #[inline(always)]
    pub const fn with_fifo_overflow_intr_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntrClr::FIFO_OVERFLOW_INTR_CLR_MASK
            } else {
                self.bits & !IntrClr::FIFO_OVERFLOW_INTR_CLR_MASK
            },
        }
    }
}

impl Default for IntrClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// TRIGGER Register
///
/// trigger register
///
/// Address offset: 0x0020
/// Reset value: 0x0000000c
///
/// ## Fields
///
/// - **RESTART_ENA** `[3]` (RW)
///
///   Configure whether or not enable auto-restart.\\1: enable\\0: disable\\
///
/// - **MEM_LOOP** `[2]` (RW)
///
///   Configure memory loop mode. \\1: trace will loop wrtie trace_mem. \\0: when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr\\
///
/// - **OFF** `[1]` (WO)
///
///   Configure whether or not stop trace.\\1: stop trace \\0: invalid\\
///
/// - **ON** `[0]` (WO)
///
///   Configure whether or not start trace.\\1: start trace \\0: invalid\\
#[repr(transparent)]
pub struct Trigger {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Trigger {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Trigger {
    // ======== Field Constants ========

    /// **RESTART_ENA**
    ///
    /// Configure whether or not enable auto-restart.\\1: enable\\0: disable\\
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RESTART_ENA_OFFSET: u32 = 3;
    pub const RESTART_ENA_MASK: u32 = 0x00000008;
    pub const RESTART_ENA: u32 = 1 << 3;

    /// **MEM_LOOP**
    ///
    /// Configure memory loop mode. \\1: trace will loop wrtie trace_mem. \\0: when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr\\
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MEM_LOOP_OFFSET: u32 = 2;
    pub const MEM_LOOP_MASK: u32 = 0x00000004;
    pub const MEM_LOOP: u32 = 1 << 2;

    /// **OFF**
    ///
    /// Configure whether or not stop trace.\\1: stop trace \\0: invalid\\
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OFF_OFFSET: u32 = 1;
    pub const OFF_MASK: u32 = 0x00000002;
    pub const OFF: u32 = 1 << 1;

    /// **ON**
    ///
    /// Configure whether or not start trace.\\1: start trace \\0: invalid\\
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ON_OFFSET: u32 = 0;
    pub const ON_MASK: u32 = 0x00000001;
    pub const ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RESTART_ENA** `[3]` (RW)
    ///
    /// Configure whether or not enable auto-restart.\\1: enable\\0: disable\\
    #[inline(always)]
    pub fn restart_ena(&self) -> bool {
        (self.value.read_bits() & Self::RESTART_ENA_MASK) != 0
    }

    /// Set RESTART_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_restart_ena(&self) {
        self.value.modify_bits(|r| r | Self::RESTART_ENA_MASK);
    }

    /// Clear RESTART_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_restart_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RESTART_ENA_MASK);
    }

    /// **MEM_LOOP** `[2]` (RW)
    ///
    /// Configure memory loop mode. \\1: trace will loop wrtie trace_mem. \\0: when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr\\
    #[inline(always)]
    pub fn mem_loop(&self) -> bool {
        (self.value.read_bits() & Self::MEM_LOOP_MASK) != 0
    }

    /// Set MEM_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_loop(&self) {
        self.value.modify_bits(|r| r | Self::MEM_LOOP_MASK);
    }

    /// Clear MEM_LOOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_loop(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_LOOP_MASK);
    }

    /// Trigger **OFF** (WO)
    ///
    /// Configure whether or not stop trace.\\1: stop trace \\0: invalid\\
    #[inline(always)]
    pub fn trigger_off(&self) {
        self.value.modify_bits(|r| r | Self::OFF_MASK);
    }

    /// Trigger **ON** (WO)
    ///
    /// Configure whether or not start trace.\\1: start trace \\0: invalid\\
    #[inline(always)]
    pub fn trigger_on(&self) {
        self.value.modify_bits(|r| r | Self::ON_MASK);
    }
}

/// TRIGGER value type for batch read/modify/write operations
///
/// Use `trigger.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trigger.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TriggerVal {
    bits: u32,
}

impl TriggerVal {
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
        Self { bits: 0x0000000c }
    }

    /// Read **RESTART_ENA** `[3:3]`
    ///
    /// Configure whether or not enable auto-restart.\\1: enable\\0: disable\\
    #[inline(always)]
    pub const fn restart_ena(&self) -> bool {
        (self.bits & Trigger::RESTART_ENA_MASK) != 0
    }

    /// Read **MEM_LOOP** `[2:2]`
    ///
    /// Configure memory loop mode. \\1: trace will loop wrtie trace_mem. \\0: when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr\\
    #[inline(always)]
    pub const fn mem_loop(&self) -> bool {
        (self.bits & Trigger::MEM_LOOP_MASK) != 0
    }

    /// Set **RESTART_ENA** `[3:3]`
    ///
    /// Configure whether or not enable auto-restart.\\1: enable\\0: disable\\
    #[inline(always)]
    pub const fn with_restart_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trigger::RESTART_ENA_MASK
            } else {
                self.bits & !Trigger::RESTART_ENA_MASK
            },
        }
    }

    /// Set **MEM_LOOP** `[2:2]`
    ///
    /// Configure memory loop mode. \\1: trace will loop wrtie trace_mem. \\0: when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr\\
    #[inline(always)]
    pub const fn with_mem_loop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trigger::MEM_LOOP_MASK
            } else {
                self.bits & !Trigger::MEM_LOOP_MASK
            },
        }
    }

    /// Set **OFF** `[1:1]`
    ///
    /// Configure whether or not stop trace.\\1: stop trace \\0: invalid\\
    #[inline(always)]
    pub const fn with_off(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trigger::OFF_MASK
            } else {
                self.bits & !Trigger::OFF_MASK
            },
        }
    }

    /// Set **ON** `[0:0]`
    ///
    /// Configure whether or not start trace.\\1: start trace \\0: invalid\\
    #[inline(always)]
    pub const fn with_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trigger::ON_MASK
            } else {
                self.bits & !Trigger::ON_MASK
            },
        }
    }
}

impl Default for TriggerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Trigger {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TriggerVal {
        TriggerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TriggerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TriggerVal) -> TriggerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONFIG Register
///
/// trace configuration register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **IMPLICIT_EXCEPT** `[5]` (RW)
///
///   Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\1: enabled\\0: disabled\\
///
/// - **FULL_ADDRESS** `[4]` (RW)
///
///   Configure whether or not enable full-address mode.\\1: full address mode.\\0: delta address mode\\
///
/// - **STALL_ENA** `[3]` (RW)
///
///   Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\1: enabled.\\0: disabled\\
///
/// - **HALT_ENA** `[2]` (RW)
///
///   Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\1: enabeld\\0: disabled\\
///
/// - **RESET_ENA** `[1]` (RW)
///
///   Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\1: enabeld\\0: disabled\\
///
/// - **DM_TRIGGER_ENA** `[0]` (RW)
///
///   Configure whether or not enable cpu trigger action.\\1: enable\\0:disable\\
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

    /// **IMPLICIT_EXCEPT**
    ///
    /// Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\1: enabled\\0: disabled\\
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IMPLICIT_EXCEPT_OFFSET: u32 = 5;
    pub const IMPLICIT_EXCEPT_MASK: u32 = 0x00000020;
    pub const IMPLICIT_EXCEPT: u32 = 1 << 5;

    /// **FULL_ADDRESS**
    ///
    /// Configure whether or not enable full-address mode.\\1: full address mode.\\0: delta address mode\\
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FULL_ADDRESS_OFFSET: u32 = 4;
    pub const FULL_ADDRESS_MASK: u32 = 0x00000010;
    pub const FULL_ADDRESS: u32 = 1 << 4;

    /// **STALL_ENA**
    ///
    /// Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\1: enabled.\\0: disabled\\
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const STALL_ENA_OFFSET: u32 = 3;
    pub const STALL_ENA_MASK: u32 = 0x00000008;
    pub const STALL_ENA: u32 = 1 << 3;

    /// **HALT_ENA**
    ///
    /// Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\1: enabeld\\0: disabled\\
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HALT_ENA_OFFSET: u32 = 2;
    pub const HALT_ENA_MASK: u32 = 0x00000004;
    pub const HALT_ENA: u32 = 1 << 2;

    /// **RESET_ENA**
    ///
    /// Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\1: enabeld\\0: disabled\\
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RESET_ENA_OFFSET: u32 = 1;
    pub const RESET_ENA_MASK: u32 = 0x00000002;
    pub const RESET_ENA: u32 = 1 << 1;

    /// **DM_TRIGGER_ENA**
    ///
    /// Configure whether or not enable cpu trigger action.\\1: enable\\0:disable\\
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DM_TRIGGER_ENA_OFFSET: u32 = 0;
    pub const DM_TRIGGER_ENA_MASK: u32 = 0x00000001;
    pub const DM_TRIGGER_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IMPLICIT_EXCEPT** `[5]` (RW)
    ///
    /// Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\1: enabled\\0: disabled\\
    #[inline(always)]
    pub fn implicit_except(&self) -> bool {
        (self.value.read_bits() & Self::IMPLICIT_EXCEPT_MASK) != 0
    }

    /// Set IMPLICIT_EXCEPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_implicit_except(&self) {
        self.value.modify_bits(|r| r | Self::IMPLICIT_EXCEPT_MASK);
    }

    /// Clear IMPLICIT_EXCEPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_implicit_except(&self) {
        self.value.modify_bits(|r| r & !Self::IMPLICIT_EXCEPT_MASK);
    }

    /// **FULL_ADDRESS** `[4]` (RW)
    ///
    /// Configure whether or not enable full-address mode.\\1: full address mode.\\0: delta address mode\\
    #[inline(always)]
    pub fn full_address(&self) -> bool {
        (self.value.read_bits() & Self::FULL_ADDRESS_MASK) != 0
    }

    /// Set FULL_ADDRESS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_full_address(&self) {
        self.value.modify_bits(|r| r | Self::FULL_ADDRESS_MASK);
    }

    /// Clear FULL_ADDRESS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_full_address(&self) {
        self.value.modify_bits(|r| r & !Self::FULL_ADDRESS_MASK);
    }

    /// **STALL_ENA** `[3]` (RW)
    ///
    /// Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\1: enabled.\\0: disabled\\
    #[inline(always)]
    pub fn stall_ena(&self) -> bool {
        (self.value.read_bits() & Self::STALL_ENA_MASK) != 0
    }

    /// Set STALL_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stall_ena(&self) {
        self.value.modify_bits(|r| r | Self::STALL_ENA_MASK);
    }

    /// Clear STALL_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stall_ena(&self) {
        self.value.modify_bits(|r| r & !Self::STALL_ENA_MASK);
    }

    /// **HALT_ENA** `[2]` (RW)
    ///
    /// Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub fn halt_ena(&self) -> bool {
        (self.value.read_bits() & Self::HALT_ENA_MASK) != 0
    }

    /// Set HALT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_halt_ena(&self) {
        self.value.modify_bits(|r| r | Self::HALT_ENA_MASK);
    }

    /// Clear HALT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_halt_ena(&self) {
        self.value.modify_bits(|r| r & !Self::HALT_ENA_MASK);
    }

    /// **RESET_ENA** `[1]` (RW)
    ///
    /// Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub fn reset_ena(&self) -> bool {
        (self.value.read_bits() & Self::RESET_ENA_MASK) != 0
    }

    /// Set RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reset_ena(&self) {
        self.value.modify_bits(|r| r | Self::RESET_ENA_MASK);
    }

    /// Clear RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reset_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RESET_ENA_MASK);
    }

    /// **DM_TRIGGER_ENA** `[0]` (RW)
    ///
    /// Configure whether or not enable cpu trigger action.\\1: enable\\0:disable\\
    #[inline(always)]
    pub fn dm_trigger_ena(&self) -> bool {
        (self.value.read_bits() & Self::DM_TRIGGER_ENA_MASK) != 0
    }

    /// Set DM_TRIGGER_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dm_trigger_ena(&self) {
        self.value.modify_bits(|r| r | Self::DM_TRIGGER_ENA_MASK);
    }

    /// Clear DM_TRIGGER_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dm_trigger_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DM_TRIGGER_ENA_MASK);
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

    /// Read **IMPLICIT_EXCEPT** `[5:5]`
    ///
    /// Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\1: enabled\\0: disabled\\
    #[inline(always)]
    pub const fn implicit_except(&self) -> bool {
        (self.bits & Config::IMPLICIT_EXCEPT_MASK) != 0
    }

    /// Read **FULL_ADDRESS** `[4:4]`
    ///
    /// Configure whether or not enable full-address mode.\\1: full address mode.\\0: delta address mode\\
    #[inline(always)]
    pub const fn full_address(&self) -> bool {
        (self.bits & Config::FULL_ADDRESS_MASK) != 0
    }

    /// Read **STALL_ENA** `[3:3]`
    ///
    /// Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\1: enabled.\\0: disabled\\
    #[inline(always)]
    pub const fn stall_ena(&self) -> bool {
        (self.bits & Config::STALL_ENA_MASK) != 0
    }

    /// Read **HALT_ENA** `[2:2]`
    ///
    /// Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub const fn halt_ena(&self) -> bool {
        (self.bits & Config::HALT_ENA_MASK) != 0
    }

    /// Read **RESET_ENA** `[1:1]`
    ///
    /// Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub const fn reset_ena(&self) -> bool {
        (self.bits & Config::RESET_ENA_MASK) != 0
    }

    /// Read **DM_TRIGGER_ENA** `[0:0]`
    ///
    /// Configure whether or not enable cpu trigger action.\\1: enable\\0:disable\\
    #[inline(always)]
    pub const fn dm_trigger_ena(&self) -> bool {
        (self.bits & Config::DM_TRIGGER_ENA_MASK) != 0
    }

    /// Set **IMPLICIT_EXCEPT** `[5:5]`
    ///
    /// Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\1: enabled\\0: disabled\\
    #[inline(always)]
    pub const fn with_implicit_except(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IMPLICIT_EXCEPT_MASK
            } else {
                self.bits & !Config::IMPLICIT_EXCEPT_MASK
            },
        }
    }

    /// Set **FULL_ADDRESS** `[4:4]`
    ///
    /// Configure whether or not enable full-address mode.\\1: full address mode.\\0: delta address mode\\
    #[inline(always)]
    pub const fn with_full_address(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::FULL_ADDRESS_MASK
            } else {
                self.bits & !Config::FULL_ADDRESS_MASK
            },
        }
    }

    /// Set **STALL_ENA** `[3:3]`
    ///
    /// Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\1: enabled.\\0: disabled\\
    #[inline(always)]
    pub const fn with_stall_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::STALL_ENA_MASK
            } else {
                self.bits & !Config::STALL_ENA_MASK
            },
        }
    }

    /// Set **HALT_ENA** `[2:2]`
    ///
    /// Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub const fn with_halt_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::HALT_ENA_MASK
            } else {
                self.bits & !Config::HALT_ENA_MASK
            },
        }
    }

    /// Set **RESET_ENA** `[1:1]`
    ///
    /// Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\1: enabeld\\0: disabled\\
    #[inline(always)]
    pub const fn with_reset_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::RESET_ENA_MASK
            } else {
                self.bits & !Config::RESET_ENA_MASK
            },
        }
    }

    /// Set **DM_TRIGGER_ENA** `[0:0]`
    ///
    /// Configure whether or not enable cpu trigger action.\\1: enable\\0:disable\\
    #[inline(always)]
    pub const fn with_dm_trigger_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::DM_TRIGGER_ENA_MASK
            } else {
                self.bits & !Config::DM_TRIGGER_ENA_MASK
            },
        }
    }
}

impl Default for ConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
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

/// FILTER_CONTROL Register
///
/// filter control register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **MATCH_INTERRUPT** `[4]` (RW)
///
///   when set, start matching from a trap with the interrupt level codes specified by \hyperref[fielddesc:TRACEMATCHVALUEINTERRUPT]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap.
///
/// - **MATCH_ECAUSE** `[3]` (RW)
///
///   when set, start matching from exception cause codes specified by \hyperref[fielddesc:TRACEMATCHCHOICEECAUSE]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception.
///
/// - **MATCH_PRIVILEGE** `[2]` (RW)
///
///   when set, match privilege levels specified by \hyperref[fielddesc:TRACEMATCHCHOICEPRIVILEGE]{TRACE_MATCH_CHOICE_PRIVILEGE}.
///
/// - **MATCH_COMP** `[1]` (RW)
///
///   when set, the comparator must be high in order for the filter to match
///
/// - **FILTER_EN** `[0]` (RW)
///
///   Configure whether or not enable filter unit. \\1: enable filter.\\ 0: always match
#[repr(transparent)]
pub struct FilterControl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterControl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterControl {
    // ======== Field Constants ========

    /// **MATCH_INTERRUPT**
    ///
    /// when set, start matching from a trap with the interrupt level codes specified by \hyperref[fielddesc:TRACEMATCHVALUEINTERRUPT]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MATCH_INTERRUPT_OFFSET: u32 = 4;
    pub const MATCH_INTERRUPT_MASK: u32 = 0x00000010;
    pub const MATCH_INTERRUPT: u32 = 1 << 4;

    /// **MATCH_ECAUSE**
    ///
    /// when set, start matching from exception cause codes specified by \hyperref[fielddesc:TRACEMATCHCHOICEECAUSE]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MATCH_ECAUSE_OFFSET: u32 = 3;
    pub const MATCH_ECAUSE_MASK: u32 = 0x00000008;
    pub const MATCH_ECAUSE: u32 = 1 << 3;

    /// **MATCH_PRIVILEGE**
    ///
    /// when set, match privilege levels specified by \hyperref[fielddesc:TRACEMATCHCHOICEPRIVILEGE]{TRACE_MATCH_CHOICE_PRIVILEGE}.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MATCH_PRIVILEGE_OFFSET: u32 = 2;
    pub const MATCH_PRIVILEGE_MASK: u32 = 0x00000004;
    pub const MATCH_PRIVILEGE: u32 = 1 << 2;

    /// **MATCH_COMP**
    ///
    /// when set, the comparator must be high in order for the filter to match
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MATCH_COMP_OFFSET: u32 = 1;
    pub const MATCH_COMP_MASK: u32 = 0x00000002;
    pub const MATCH_COMP: u32 = 1 << 1;

    /// **FILTER_EN**
    ///
    /// Configure whether or not enable filter unit. \\1: enable filter.\\ 0: always match
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_EN_OFFSET: u32 = 0;
    pub const FILTER_EN_MASK: u32 = 0x00000001;
    pub const FILTER_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MATCH_INTERRUPT** `[4]` (RW)
    ///
    /// when set, start matching from a trap with the interrupt level codes specified by \hyperref[fielddesc:TRACEMATCHVALUEINTERRUPT]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap.
    #[inline(always)]
    pub fn match_interrupt(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_INTERRUPT_MASK) != 0
    }

    /// Set MATCH_INTERRUPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_interrupt(&self) {
        self.value.modify_bits(|r| r | Self::MATCH_INTERRUPT_MASK);
    }

    /// Clear MATCH_INTERRUPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_interrupt(&self) {
        self.value.modify_bits(|r| r & !Self::MATCH_INTERRUPT_MASK);
    }

    /// **MATCH_ECAUSE** `[3]` (RW)
    ///
    /// when set, start matching from exception cause codes specified by \hyperref[fielddesc:TRACEMATCHCHOICEECAUSE]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception.
    #[inline(always)]
    pub fn match_ecause(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_ECAUSE_MASK) != 0
    }

    /// Set MATCH_ECAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_ecause(&self) {
        self.value.modify_bits(|r| r | Self::MATCH_ECAUSE_MASK);
    }

    /// Clear MATCH_ECAUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_ecause(&self) {
        self.value.modify_bits(|r| r & !Self::MATCH_ECAUSE_MASK);
    }

    /// **MATCH_PRIVILEGE** `[2]` (RW)
    ///
    /// when set, match privilege levels specified by \hyperref[fielddesc:TRACEMATCHCHOICEPRIVILEGE]{TRACE_MATCH_CHOICE_PRIVILEGE}.
    #[inline(always)]
    pub fn match_privilege(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_PRIVILEGE_MASK) != 0
    }

    /// Set MATCH_PRIVILEGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_privilege(&self) {
        self.value.modify_bits(|r| r | Self::MATCH_PRIVILEGE_MASK);
    }

    /// Clear MATCH_PRIVILEGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_privilege(&self) {
        self.value.modify_bits(|r| r & !Self::MATCH_PRIVILEGE_MASK);
    }

    /// **MATCH_COMP** `[1]` (RW)
    ///
    /// when set, the comparator must be high in order for the filter to match
    #[inline(always)]
    pub fn match_comp(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_COMP_MASK) != 0
    }

    /// Set MATCH_COMP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_comp(&self) {
        self.value.modify_bits(|r| r | Self::MATCH_COMP_MASK);
    }

    /// Clear MATCH_COMP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_comp(&self) {
        self.value.modify_bits(|r| r & !Self::MATCH_COMP_MASK);
    }

    /// **FILTER_EN** `[0]` (RW)
    ///
    /// Configure whether or not enable filter unit. \\1: enable filter.\\ 0: always match
    #[inline(always)]
    pub fn filter_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_EN_MASK) != 0
    }

    /// Set FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_EN_MASK);
    }

    /// Clear FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_EN_MASK);
    }
}

/// FILTER_CONTROL value type for batch read/modify/write operations
///
/// Use `filtercontrol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filtercontrol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterControlVal {
    bits: u32,
}

impl FilterControlVal {
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

    /// Read **MATCH_INTERRUPT** `[4:4]`
    ///
    /// when set, start matching from a trap with the interrupt level codes specified by \hyperref[fielddesc:TRACEMATCHVALUEINTERRUPT]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap.
    #[inline(always)]
    pub const fn match_interrupt(&self) -> bool {
        (self.bits & FilterControl::MATCH_INTERRUPT_MASK) != 0
    }

    /// Read **MATCH_ECAUSE** `[3:3]`
    ///
    /// when set, start matching from exception cause codes specified by \hyperref[fielddesc:TRACEMATCHCHOICEECAUSE]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception.
    #[inline(always)]
    pub const fn match_ecause(&self) -> bool {
        (self.bits & FilterControl::MATCH_ECAUSE_MASK) != 0
    }

    /// Read **MATCH_PRIVILEGE** `[2:2]`
    ///
    /// when set, match privilege levels specified by \hyperref[fielddesc:TRACEMATCHCHOICEPRIVILEGE]{TRACE_MATCH_CHOICE_PRIVILEGE}.
    #[inline(always)]
    pub const fn match_privilege(&self) -> bool {
        (self.bits & FilterControl::MATCH_PRIVILEGE_MASK) != 0
    }

    /// Read **MATCH_COMP** `[1:1]`
    ///
    /// when set, the comparator must be high in order for the filter to match
    #[inline(always)]
    pub const fn match_comp(&self) -> bool {
        (self.bits & FilterControl::MATCH_COMP_MASK) != 0
    }

    /// Read **FILTER_EN** `[0:0]`
    ///
    /// Configure whether or not enable filter unit. \\1: enable filter.\\ 0: always match
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        (self.bits & FilterControl::FILTER_EN_MASK) != 0
    }

    /// Set **MATCH_INTERRUPT** `[4:4]`
    ///
    /// when set, start matching from a trap with the interrupt level codes specified by \hyperref[fielddesc:TRACEMATCHVALUEINTERRUPT]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap.
    #[inline(always)]
    pub const fn with_match_interrupt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterControl::MATCH_INTERRUPT_MASK
            } else {
                self.bits & !FilterControl::MATCH_INTERRUPT_MASK
            },
        }
    }

    /// Set **MATCH_ECAUSE** `[3:3]`
    ///
    /// when set, start matching from exception cause codes specified by \hyperref[fielddesc:TRACEMATCHCHOICEECAUSE]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception.
    #[inline(always)]
    pub const fn with_match_ecause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterControl::MATCH_ECAUSE_MASK
            } else {
                self.bits & !FilterControl::MATCH_ECAUSE_MASK
            },
        }
    }

    /// Set **MATCH_PRIVILEGE** `[2:2]`
    ///
    /// when set, match privilege levels specified by \hyperref[fielddesc:TRACEMATCHCHOICEPRIVILEGE]{TRACE_MATCH_CHOICE_PRIVILEGE}.
    #[inline(always)]
    pub const fn with_match_privilege(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterControl::MATCH_PRIVILEGE_MASK
            } else {
                self.bits & !FilterControl::MATCH_PRIVILEGE_MASK
            },
        }
    }

    /// Set **MATCH_COMP** `[1:1]`
    ///
    /// when set, the comparator must be high in order for the filter to match
    #[inline(always)]
    pub const fn with_match_comp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterControl::MATCH_COMP_MASK
            } else {
                self.bits & !FilterControl::MATCH_COMP_MASK
            },
        }
    }

    /// Set **FILTER_EN** `[0:0]`
    ///
    /// Configure whether or not enable filter unit. \\1: enable filter.\\ 0: always match
    #[inline(always)]
    pub const fn with_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterControl::FILTER_EN_MASK
            } else {
                self.bits & !FilterControl::FILTER_EN_MASK
            },
        }
    }
}

impl Default for FilterControlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterControl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterControlVal {
        FilterControlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterControlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterControlVal) -> FilterControlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_MATCH_CONTROL Register
///
/// filter match control register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **MATCH_CHOICE_ECAUSE** `[7:2]` (RW)
///
///   specified which ecause matched.
///
/// - **MATCH_VALUE_INTERRUPT** `[1]` (RW)
///
///   Select which match which itype when \hyperref[fielddesc:TRACEMATCHINTERRUPT]{TRACE_MATCH_INTERRUP} is set. \\1: match itype of 2. \\0: match itype or 1.
///
/// - **MATCH_CHOICE_PRIVILEGE** `[0]` (RW)
///
///   Select match which privilege level when \hyperref[fielddesc:TRACEMATCHPRIVILEGE]{TRACE_MATCH_PRIVILEGE} is set. \\1: machine mode. \\0: user mode
#[repr(transparent)]
pub struct FilterMatchControl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterMatchControl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterMatchControl {
    // ======== Field Constants ========

    /// **MATCH_CHOICE_ECAUSE**
    ///
    /// specified which ecause matched.
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const MATCH_CHOICE_ECAUSE_OFFSET: u32 = 2;
    pub const MATCH_CHOICE_ECAUSE_MASK: u32 = 0x000000fc;

    /// **MATCH_VALUE_INTERRUPT**
    ///
    /// Select which match which itype when \hyperref[fielddesc:TRACEMATCHINTERRUPT]{TRACE_MATCH_INTERRUP} is set. \\1: match itype of 2. \\0: match itype or 1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MATCH_VALUE_INTERRUPT_OFFSET: u32 = 1;
    pub const MATCH_VALUE_INTERRUPT_MASK: u32 = 0x00000002;
    pub const MATCH_VALUE_INTERRUPT: u32 = 1 << 1;

    /// **MATCH_CHOICE_PRIVILEGE**
    ///
    /// Select match which privilege level when \hyperref[fielddesc:TRACEMATCHPRIVILEGE]{TRACE_MATCH_PRIVILEGE} is set. \\1: machine mode. \\0: user mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MATCH_CHOICE_PRIVILEGE_OFFSET: u32 = 0;
    pub const MATCH_CHOICE_PRIVILEGE_MASK: u32 = 0x00000001;
    pub const MATCH_CHOICE_PRIVILEGE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MATCH_CHOICE_ECAUSE** `[7:2]` (RW)
    ///
    /// specified which ecause matched.
    #[inline(always)]
    pub fn match_choice_ecause(&self) -> u32 {
        (self.value.read_bits() & Self::MATCH_CHOICE_ECAUSE_MASK)
            >> Self::MATCH_CHOICE_ECAUSE_OFFSET
    }

    /// Write **MATCH_CHOICE_ECAUSE** `[7:2]` (RW)
    ///
    /// specified which ecause matched.
    #[inline(always)]
    pub fn write_match_choice_ecause(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MATCH_CHOICE_ECAUSE_MASK)
                | ((value << Self::MATCH_CHOICE_ECAUSE_OFFSET) & Self::MATCH_CHOICE_ECAUSE_MASK)
        });
    }

    /// **MATCH_VALUE_INTERRUPT** `[1]` (RW)
    ///
    /// Select which match which itype when \hyperref[fielddesc:TRACEMATCHINTERRUPT]{TRACE_MATCH_INTERRUP} is set. \\1: match itype of 2. \\0: match itype or 1.
    #[inline(always)]
    pub fn match_value_interrupt(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_VALUE_INTERRUPT_MASK) != 0
    }

    /// Set MATCH_VALUE_INTERRUPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_value_interrupt(&self) {
        self.value
            .modify_bits(|r| r | Self::MATCH_VALUE_INTERRUPT_MASK);
    }

    /// Clear MATCH_VALUE_INTERRUPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_value_interrupt(&self) {
        self.value
            .modify_bits(|r| r & !Self::MATCH_VALUE_INTERRUPT_MASK);
    }

    /// **MATCH_CHOICE_PRIVILEGE** `[0]` (RW)
    ///
    /// Select match which privilege level when \hyperref[fielddesc:TRACEMATCHPRIVILEGE]{TRACE_MATCH_PRIVILEGE} is set. \\1: machine mode. \\0: user mode
    #[inline(always)]
    pub fn match_choice_privilege(&self) -> bool {
        (self.value.read_bits() & Self::MATCH_CHOICE_PRIVILEGE_MASK) != 0
    }

    /// Set MATCH_CHOICE_PRIVILEGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_match_choice_privilege(&self) {
        self.value
            .modify_bits(|r| r | Self::MATCH_CHOICE_PRIVILEGE_MASK);
    }

    /// Clear MATCH_CHOICE_PRIVILEGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_match_choice_privilege(&self) {
        self.value
            .modify_bits(|r| r & !Self::MATCH_CHOICE_PRIVILEGE_MASK);
    }
}

/// FILTER_MATCH_CONTROL value type for batch read/modify/write operations
///
/// Use `filtermatchcontrol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filtermatchcontrol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterMatchControlVal {
    bits: u32,
}

impl FilterMatchControlVal {
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

    /// Read **MATCH_CHOICE_ECAUSE** `[7:2]`
    ///
    /// specified which ecause matched.
    #[inline(always)]
    pub const fn match_choice_ecause(&self) -> u32 {
        (self.bits & FilterMatchControl::MATCH_CHOICE_ECAUSE_MASK)
            >> FilterMatchControl::MATCH_CHOICE_ECAUSE_OFFSET
    }

    /// Read **MATCH_VALUE_INTERRUPT** `[1:1]`
    ///
    /// Select which match which itype when \hyperref[fielddesc:TRACEMATCHINTERRUPT]{TRACE_MATCH_INTERRUP} is set. \\1: match itype of 2. \\0: match itype or 1.
    #[inline(always)]
    pub const fn match_value_interrupt(&self) -> bool {
        (self.bits & FilterMatchControl::MATCH_VALUE_INTERRUPT_MASK) != 0
    }

    /// Read **MATCH_CHOICE_PRIVILEGE** `[0:0]`
    ///
    /// Select match which privilege level when \hyperref[fielddesc:TRACEMATCHPRIVILEGE]{TRACE_MATCH_PRIVILEGE} is set. \\1: machine mode. \\0: user mode
    #[inline(always)]
    pub const fn match_choice_privilege(&self) -> bool {
        (self.bits & FilterMatchControl::MATCH_CHOICE_PRIVILEGE_MASK) != 0
    }

    /// Set **MATCH_CHOICE_ECAUSE** `[7:2]`
    ///
    /// specified which ecause matched.
    #[inline(always)]
    pub const fn with_match_choice_ecause(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterMatchControl::MATCH_CHOICE_ECAUSE_MASK)
                | ((value << FilterMatchControl::MATCH_CHOICE_ECAUSE_OFFSET)
                    & FilterMatchControl::MATCH_CHOICE_ECAUSE_MASK),
        }
    }

    /// Set **MATCH_VALUE_INTERRUPT** `[1:1]`
    ///
    /// Select which match which itype when \hyperref[fielddesc:TRACEMATCHINTERRUPT]{TRACE_MATCH_INTERRUP} is set. \\1: match itype of 2. \\0: match itype or 1.
    #[inline(always)]
    pub const fn with_match_value_interrupt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterMatchControl::MATCH_VALUE_INTERRUPT_MASK
            } else {
                self.bits & !FilterMatchControl::MATCH_VALUE_INTERRUPT_MASK
            },
        }
    }

    /// Set **MATCH_CHOICE_PRIVILEGE** `[0:0]`
    ///
    /// Select match which privilege level when \hyperref[fielddesc:TRACEMATCHPRIVILEGE]{TRACE_MATCH_PRIVILEGE} is set. \\1: machine mode. \\0: user mode
    #[inline(always)]
    pub const fn with_match_choice_privilege(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterMatchControl::MATCH_CHOICE_PRIVILEGE_MASK
            } else {
                self.bits & !FilterMatchControl::MATCH_CHOICE_PRIVILEGE_MASK
            },
        }
    }
}

impl Default for FilterMatchControlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterMatchControl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterMatchControlVal {
        FilterMatchControlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterMatchControlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterMatchControlVal) -> FilterMatchControlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_COMPARATOR_CONTROL Register
///
/// filter comparator match control register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **MATCH_MODE** `[17:16]` (RW)
///
///   0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
///
/// - **S_NOTIFY** `[13]` (RW)
///
///   Generate a trace packet explicitly reporting the address that cause the secondary match
///
/// - **S_FUNCTION** `[12:10]` (RW)
///
///   Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
///
/// - **S_INPUT** `[8]` (RW)
///
///   Determines which input to compare against the secondary comparator, \\0: iaddr, \\1: tval.
///
/// - **P_NOTIFY** `[5]` (RW)
///
///   Generate a trace packet explicitly reporting the address that cause the primary match
///
/// - **P_FUNCTION** `[4:2]` (RW)
///
///   Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
///
/// - **P_INPUT** `[0]` (RW)
///
///   Determines which input to compare against the primary comparator, \\0: iaddr, \\1: tval.
#[repr(transparent)]
pub struct FilterComparatorControl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterComparatorControl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterComparatorControl {
    // ======== Field Constants ========

    /// **MATCH_MODE**
    ///
    /// 0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const MATCH_MODE_OFFSET: u32 = 16;
    pub const MATCH_MODE_MASK: u32 = 0x00030000;

    /// **S_NOTIFY**
    ///
    /// Generate a trace packet explicitly reporting the address that cause the secondary match
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const S_NOTIFY_OFFSET: u32 = 13;
    pub const S_NOTIFY_MASK: u32 = 0x00002000;
    pub const S_NOTIFY: u32 = 1 << 13;

    /// **S_FUNCTION**
    ///
    /// Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    ///
    /// Bits `[12:10]`, mask `0x00001c00`
    pub const S_FUNCTION_OFFSET: u32 = 10;
    pub const S_FUNCTION_MASK: u32 = 0x00001c00;

    /// **S_INPUT**
    ///
    /// Determines which input to compare against the secondary comparator, \\0: iaddr, \\1: tval.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const S_INPUT_OFFSET: u32 = 8;
    pub const S_INPUT_MASK: u32 = 0x00000100;
    pub const S_INPUT: u32 = 1 << 8;

    /// **P_NOTIFY**
    ///
    /// Generate a trace packet explicitly reporting the address that cause the primary match
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const P_NOTIFY_OFFSET: u32 = 5;
    pub const P_NOTIFY_MASK: u32 = 0x00000020;
    pub const P_NOTIFY: u32 = 1 << 5;

    /// **P_FUNCTION**
    ///
    /// Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const P_FUNCTION_OFFSET: u32 = 2;
    pub const P_FUNCTION_MASK: u32 = 0x0000001c;

    /// **P_INPUT**
    ///
    /// Determines which input to compare against the primary comparator, \\0: iaddr, \\1: tval.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const P_INPUT_OFFSET: u32 = 0;
    pub const P_INPUT_MASK: u32 = 0x00000001;
    pub const P_INPUT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MATCH_MODE** `[17:16]` (RW)
    ///
    /// 0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
    #[inline(always)]
    pub fn match_mode(&self) -> u32 {
        (self.value.read_bits() & Self::MATCH_MODE_MASK) >> Self::MATCH_MODE_OFFSET
    }

    /// Write **MATCH_MODE** `[17:16]` (RW)
    ///
    /// 0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
    #[inline(always)]
    pub fn write_match_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MATCH_MODE_MASK)
                | ((value << Self::MATCH_MODE_OFFSET) & Self::MATCH_MODE_MASK)
        });
    }

    /// **S_NOTIFY** `[13]` (RW)
    ///
    /// Generate a trace packet explicitly reporting the address that cause the secondary match
    #[inline(always)]
    pub fn s_notify(&self) -> bool {
        (self.value.read_bits() & Self::S_NOTIFY_MASK) != 0
    }

    /// Set S_NOTIFY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_s_notify(&self) {
        self.value.modify_bits(|r| r | Self::S_NOTIFY_MASK);
    }

    /// Clear S_NOTIFY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_s_notify(&self) {
        self.value.modify_bits(|r| r & !Self::S_NOTIFY_MASK);
    }

    /// **S_FUNCTION** `[12:10]` (RW)
    ///
    /// Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub fn s_function(&self) -> u32 {
        (self.value.read_bits() & Self::S_FUNCTION_MASK) >> Self::S_FUNCTION_OFFSET
    }

    /// Write **S_FUNCTION** `[12:10]` (RW)
    ///
    /// Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub fn write_s_function(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::S_FUNCTION_MASK)
                | ((value << Self::S_FUNCTION_OFFSET) & Self::S_FUNCTION_MASK)
        });
    }

    /// **S_INPUT** `[8]` (RW)
    ///
    /// Determines which input to compare against the secondary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub fn s_input(&self) -> bool {
        (self.value.read_bits() & Self::S_INPUT_MASK) != 0
    }

    /// Set S_INPUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_s_input(&self) {
        self.value.modify_bits(|r| r | Self::S_INPUT_MASK);
    }

    /// Clear S_INPUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_s_input(&self) {
        self.value.modify_bits(|r| r & !Self::S_INPUT_MASK);
    }

    /// **P_NOTIFY** `[5]` (RW)
    ///
    /// Generate a trace packet explicitly reporting the address that cause the primary match
    #[inline(always)]
    pub fn p_notify(&self) -> bool {
        (self.value.read_bits() & Self::P_NOTIFY_MASK) != 0
    }

    /// Set P_NOTIFY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_p_notify(&self) {
        self.value.modify_bits(|r| r | Self::P_NOTIFY_MASK);
    }

    /// Clear P_NOTIFY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_p_notify(&self) {
        self.value.modify_bits(|r| r & !Self::P_NOTIFY_MASK);
    }

    /// **P_FUNCTION** `[4:2]` (RW)
    ///
    /// Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub fn p_function(&self) -> u32 {
        (self.value.read_bits() & Self::P_FUNCTION_MASK) >> Self::P_FUNCTION_OFFSET
    }

    /// Write **P_FUNCTION** `[4:2]` (RW)
    ///
    /// Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub fn write_p_function(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::P_FUNCTION_MASK)
                | ((value << Self::P_FUNCTION_OFFSET) & Self::P_FUNCTION_MASK)
        });
    }

    /// **P_INPUT** `[0]` (RW)
    ///
    /// Determines which input to compare against the primary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub fn p_input(&self) -> bool {
        (self.value.read_bits() & Self::P_INPUT_MASK) != 0
    }

    /// Set P_INPUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_p_input(&self) {
        self.value.modify_bits(|r| r | Self::P_INPUT_MASK);
    }

    /// Clear P_INPUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_p_input(&self) {
        self.value.modify_bits(|r| r & !Self::P_INPUT_MASK);
    }
}

/// FILTER_COMPARATOR_CONTROL value type for batch read/modify/write operations
///
/// Use `filtercomparatorcontrol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filtercomparatorcontrol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterComparatorControlVal {
    bits: u32,
}

impl FilterComparatorControlVal {
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

    /// Read **MATCH_MODE** `[17:16]`
    ///
    /// 0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
    #[inline(always)]
    pub const fn match_mode(&self) -> u32 {
        (self.bits & FilterComparatorControl::MATCH_MODE_MASK)
            >> FilterComparatorControl::MATCH_MODE_OFFSET
    }

    /// Read **S_NOTIFY** `[13:13]`
    ///
    /// Generate a trace packet explicitly reporting the address that cause the secondary match
    #[inline(always)]
    pub const fn s_notify(&self) -> bool {
        (self.bits & FilterComparatorControl::S_NOTIFY_MASK) != 0
    }

    /// Read **S_FUNCTION** `[12:10]`
    ///
    /// Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub const fn s_function(&self) -> u32 {
        (self.bits & FilterComparatorControl::S_FUNCTION_MASK)
            >> FilterComparatorControl::S_FUNCTION_OFFSET
    }

    /// Read **S_INPUT** `[8:8]`
    ///
    /// Determines which input to compare against the secondary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub const fn s_input(&self) -> bool {
        (self.bits & FilterComparatorControl::S_INPUT_MASK) != 0
    }

    /// Read **P_NOTIFY** `[5:5]`
    ///
    /// Generate a trace packet explicitly reporting the address that cause the primary match
    #[inline(always)]
    pub const fn p_notify(&self) -> bool {
        (self.bits & FilterComparatorControl::P_NOTIFY_MASK) != 0
    }

    /// Read **P_FUNCTION** `[4:2]`
    ///
    /// Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub const fn p_function(&self) -> u32 {
        (self.bits & FilterComparatorControl::P_FUNCTION_MASK)
            >> FilterComparatorControl::P_FUNCTION_OFFSET
    }

    /// Read **P_INPUT** `[0:0]`
    ///
    /// Determines which input to compare against the primary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub const fn p_input(&self) -> bool {
        (self.bits & FilterComparatorControl::P_INPUT_MASK) != 0
    }

    /// Set **MATCH_MODE** `[17:16]`
    ///
    /// 0: only primary matches, \\1: primary and secondary comparator both matches(P\&\&S),\\ 2:either primary or secondary comparator matches !(P\&\&S), \\3: set when primary matches and continue to match until after secondary comparator matches
    #[inline(always)]
    pub const fn with_match_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterComparatorControl::MATCH_MODE_MASK)
                | ((value << FilterComparatorControl::MATCH_MODE_OFFSET)
                    & FilterComparatorControl::MATCH_MODE_MASK),
        }
    }

    /// Set **S_NOTIFY** `[13:13]`
    ///
    /// Generate a trace packet explicitly reporting the address that cause the secondary match
    #[inline(always)]
    pub const fn with_s_notify(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterComparatorControl::S_NOTIFY_MASK
            } else {
                self.bits & !FilterComparatorControl::S_NOTIFY_MASK
            },
        }
    }

    /// Set **S_FUNCTION** `[12:10]`
    ///
    /// Select the secondary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub const fn with_s_function(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterComparatorControl::S_FUNCTION_MASK)
                | ((value << FilterComparatorControl::S_FUNCTION_OFFSET)
                    & FilterComparatorControl::S_FUNCTION_MASK),
        }
    }

    /// Set **S_INPUT** `[8:8]`
    ///
    /// Determines which input to compare against the secondary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub const fn with_s_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterComparatorControl::S_INPUT_MASK
            } else {
                self.bits & !FilterComparatorControl::S_INPUT_MASK
            },
        }
    }

    /// Set **P_NOTIFY** `[5:5]`
    ///
    /// Generate a trace packet explicitly reporting the address that cause the primary match
    #[inline(always)]
    pub const fn with_p_notify(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterComparatorControl::P_NOTIFY_MASK
            } else {
                self.bits & !FilterComparatorControl::P_NOTIFY_MASK
            },
        }
    }

    /// Set **P_FUNCTION** `[4:2]`
    ///
    /// Select the primary comparator function. \\0: equal, \\1: not equal, \\2: less than, \\3: less than or equal, \\4: greater than, \\5: greater than or equal, \\other: always match
    #[inline(always)]
    pub const fn with_p_function(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterComparatorControl::P_FUNCTION_MASK)
                | ((value << FilterComparatorControl::P_FUNCTION_OFFSET)
                    & FilterComparatorControl::P_FUNCTION_MASK),
        }
    }

    /// Set **P_INPUT** `[0:0]`
    ///
    /// Determines which input to compare against the primary comparator, \\0: iaddr, \\1: tval.
    #[inline(always)]
    pub const fn with_p_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterComparatorControl::P_INPUT_MASK
            } else {
                self.bits & !FilterComparatorControl::P_INPUT_MASK
            },
        }
    }
}

impl Default for FilterComparatorControlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterComparatorControl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterComparatorControlVal {
        FilterComparatorControlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterComparatorControlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterComparatorControlVal) -> FilterComparatorControlVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_P_COMPARATOR_MATCH Register
///
/// primary comparator match value
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **P_MATCH** `[31:0]` (RW)
///
///   primary comparator match value
#[repr(transparent)]
pub struct FilterPComparatorMatch {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterPComparatorMatch {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterPComparatorMatch {
    // ======== Field Constants ========

    /// **P_MATCH**
    ///
    /// primary comparator match value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const P_MATCH_OFFSET: u32 = 0;
    pub const P_MATCH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **P_MATCH** `[31:0]` (RW)
    ///
    /// primary comparator match value
    #[inline(always)]
    pub fn p_match(&self) -> u32 {
        (self.value.read_bits() & Self::P_MATCH_MASK) >> Self::P_MATCH_OFFSET
    }

    /// Write **P_MATCH** `[31:0]` (RW)
    ///
    /// primary comparator match value
    #[inline(always)]
    pub fn write_p_match(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::P_MATCH_MASK) | ((value << Self::P_MATCH_OFFSET) & Self::P_MATCH_MASK)
        });
    }
}

/// FILTER_P_COMPARATOR_MATCH value type for batch read/modify/write operations
///
/// Use `filterpcomparatormatch.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filterpcomparatormatch.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterPComparatorMatchVal {
    bits: u32,
}

impl FilterPComparatorMatchVal {
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

    /// Read **P_MATCH** `[31:0]`
    ///
    /// primary comparator match value
    #[inline(always)]
    pub const fn p_match(&self) -> u32 {
        (self.bits & FilterPComparatorMatch::P_MATCH_MASK) >> FilterPComparatorMatch::P_MATCH_OFFSET
    }

    /// Set **P_MATCH** `[31:0]`
    ///
    /// primary comparator match value
    #[inline(always)]
    pub const fn with_p_match(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterPComparatorMatch::P_MATCH_MASK)
                | ((value << FilterPComparatorMatch::P_MATCH_OFFSET)
                    & FilterPComparatorMatch::P_MATCH_MASK),
        }
    }
}

impl Default for FilterPComparatorMatchVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterPComparatorMatch {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterPComparatorMatchVal {
        FilterPComparatorMatchVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterPComparatorMatchVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterPComparatorMatchVal) -> FilterPComparatorMatchVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_S_COMPARATOR_MATCH Register
///
/// secondary comparator match value
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **S_MATCH** `[31:0]` (RW)
///
///   secondary comparator match value
#[repr(transparent)]
pub struct FilterSComparatorMatch {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterSComparatorMatch {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterSComparatorMatch {
    // ======== Field Constants ========

    /// **S_MATCH**
    ///
    /// secondary comparator match value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const S_MATCH_OFFSET: u32 = 0;
    pub const S_MATCH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **S_MATCH** `[31:0]` (RW)
    ///
    /// secondary comparator match value
    #[inline(always)]
    pub fn s_match(&self) -> u32 {
        (self.value.read_bits() & Self::S_MATCH_MASK) >> Self::S_MATCH_OFFSET
    }

    /// Write **S_MATCH** `[31:0]` (RW)
    ///
    /// secondary comparator match value
    #[inline(always)]
    pub fn write_s_match(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::S_MATCH_MASK) | ((value << Self::S_MATCH_OFFSET) & Self::S_MATCH_MASK)
        });
    }
}

/// FILTER_S_COMPARATOR_MATCH value type for batch read/modify/write operations
///
/// Use `filterscomparatormatch.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filterscomparatormatch.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterSComparatorMatchVal {
    bits: u32,
}

impl FilterSComparatorMatchVal {
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

    /// Read **S_MATCH** `[31:0]`
    ///
    /// secondary comparator match value
    #[inline(always)]
    pub const fn s_match(&self) -> u32 {
        (self.bits & FilterSComparatorMatch::S_MATCH_MASK) >> FilterSComparatorMatch::S_MATCH_OFFSET
    }

    /// Set **S_MATCH** `[31:0]`
    ///
    /// secondary comparator match value
    #[inline(always)]
    pub const fn with_s_match(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterSComparatorMatch::S_MATCH_MASK)
                | ((value << FilterSComparatorMatch::S_MATCH_OFFSET)
                    & FilterSComparatorMatch::S_MATCH_MASK),
        }
    }
}

impl Default for FilterSComparatorMatchVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterSComparatorMatch {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterSComparatorMatchVal {
        FilterSComparatorMatchVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterSComparatorMatchVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterSComparatorMatchVal) -> FilterSComparatorMatchVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// RESYNC_PROLONGED Register
///
/// resync configuration register
///
/// Address offset: 0x003c
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **RESYNC_MODE** `[25:24]` (RW)
///
///   resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
///
/// - **RESYNC_PROLONGED** `[23:0]` (RW)
///
///   count number, when count to this value, send a sync package
#[repr(transparent)]
pub struct ResyncProlonged {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ResyncProlonged {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ResyncProlonged {
    // ======== Field Constants ========

    /// **RESYNC_MODE**
    ///
    /// resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const RESYNC_MODE_OFFSET: u32 = 24;
    pub const RESYNC_MODE_MASK: u32 = 0x03000000;

    /// **RESYNC_PROLONGED**
    ///
    /// count number, when count to this value, send a sync package
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const RESYNC_PROLONGED_OFFSET: u32 = 0;
    pub const RESYNC_PROLONGED_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **RESYNC_MODE** `[25:24]` (RW)
    ///
    /// resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
    #[inline(always)]
    pub fn resync_mode(&self) -> u32 {
        (self.value.read_bits() & Self::RESYNC_MODE_MASK) >> Self::RESYNC_MODE_OFFSET
    }

    /// Write **RESYNC_MODE** `[25:24]` (RW)
    ///
    /// resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
    #[inline(always)]
    pub fn write_resync_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RESYNC_MODE_MASK)
                | ((value << Self::RESYNC_MODE_OFFSET) & Self::RESYNC_MODE_MASK)
        });
    }

    /// **RESYNC_PROLONGED** `[23:0]` (RW)
    ///
    /// count number, when count to this value, send a sync package
    #[inline(always)]
    pub fn resync_prolonged(&self) -> u32 {
        (self.value.read_bits() & Self::RESYNC_PROLONGED_MASK) >> Self::RESYNC_PROLONGED_OFFSET
    }

    /// Write **RESYNC_PROLONGED** `[23:0]` (RW)
    ///
    /// count number, when count to this value, send a sync package
    #[inline(always)]
    pub fn write_resync_prolonged(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RESYNC_PROLONGED_MASK)
                | ((value << Self::RESYNC_PROLONGED_OFFSET) & Self::RESYNC_PROLONGED_MASK)
        });
    }
}

/// RESYNC_PROLONGED value type for batch read/modify/write operations
///
/// Use `resyncprolonged.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resyncprolonged.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResyncProlongedVal {
    bits: u32,
}

impl ResyncProlongedVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **RESYNC_MODE** `[25:24]`
    ///
    /// resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
    #[inline(always)]
    pub const fn resync_mode(&self) -> u32 {
        (self.bits & ResyncProlonged::RESYNC_MODE_MASK) >> ResyncProlonged::RESYNC_MODE_OFFSET
    }

    /// Read **RESYNC_PROLONGED** `[23:0]`
    ///
    /// count number, when count to this value, send a sync package
    #[inline(always)]
    pub const fn resync_prolonged(&self) -> u32 {
        (self.bits & ResyncProlonged::RESYNC_PROLONGED_MASK)
            >> ResyncProlonged::RESYNC_PROLONGED_OFFSET
    }

    /// Set **RESYNC_MODE** `[25:24]`
    ///
    /// resyc mode sel: \\0: off, \\2: cycle count \\3: package num count
    #[inline(always)]
    pub const fn with_resync_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ResyncProlonged::RESYNC_MODE_MASK)
                | ((value << ResyncProlonged::RESYNC_MODE_OFFSET)
                    & ResyncProlonged::RESYNC_MODE_MASK),
        }
    }

    /// Set **RESYNC_PROLONGED** `[23:0]`
    ///
    /// count number, when count to this value, send a sync package
    #[inline(always)]
    pub const fn with_resync_prolonged(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ResyncProlonged::RESYNC_PROLONGED_MASK)
                | ((value << ResyncProlonged::RESYNC_PROLONGED_OFFSET)
                    & ResyncProlonged::RESYNC_PROLONGED_MASK),
        }
    }
}

impl Default for ResyncProlongedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ResyncProlonged {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResyncProlongedVal {
        ResyncProlongedVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ResyncProlongedVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ResyncProlongedVal) -> ResyncProlongedVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_CONFIG Register
///
/// AHB config register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **MAX_INCR** `[5:3]` (RW)
///
///   set max continuous access for incr mode
///
/// - **HBURST** `[2:0]` (RW)
///
///   set hburst
#[repr(transparent)]
pub struct AhbConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbConfig {
    // ======== Field Constants ========

    /// **MAX_INCR**
    ///
    /// set max continuous access for incr mode
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const MAX_INCR_OFFSET: u32 = 3;
    pub const MAX_INCR_MASK: u32 = 0x00000038;

    /// **HBURST**
    ///
    /// set hburst
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const HBURST_OFFSET: u32 = 0;
    pub const HBURST_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **MAX_INCR** `[5:3]` (RW)
    ///
    /// set max continuous access for incr mode
    #[inline(always)]
    pub fn max_incr(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_INCR_MASK) >> Self::MAX_INCR_OFFSET
    }

    /// Write **MAX_INCR** `[5:3]` (RW)
    ///
    /// set max continuous access for incr mode
    #[inline(always)]
    pub fn write_max_incr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAX_INCR_MASK) | ((value << Self::MAX_INCR_OFFSET) & Self::MAX_INCR_MASK)
        });
    }

    /// **HBURST** `[2:0]` (RW)
    ///
    /// set hburst
    #[inline(always)]
    pub fn hburst(&self) -> u32 {
        (self.value.read_bits() & Self::HBURST_MASK) >> Self::HBURST_OFFSET
    }

    /// Write **HBURST** `[2:0]` (RW)
    ///
    /// set hburst
    #[inline(always)]
    pub fn write_hburst(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HBURST_MASK) | ((value << Self::HBURST_OFFSET) & Self::HBURST_MASK)
        });
    }
}

/// AHB_CONFIG value type for batch read/modify/write operations
///
/// Use `ahbconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbConfigVal {
    bits: u32,
}

impl AhbConfigVal {
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

    /// Read **MAX_INCR** `[5:3]`
    ///
    /// set max continuous access for incr mode
    #[inline(always)]
    pub const fn max_incr(&self) -> u32 {
        (self.bits & AhbConfig::MAX_INCR_MASK) >> AhbConfig::MAX_INCR_OFFSET
    }

    /// Read **HBURST** `[2:0]`
    ///
    /// set hburst
    #[inline(always)]
    pub const fn hburst(&self) -> u32 {
        (self.bits & AhbConfig::HBURST_MASK) >> AhbConfig::HBURST_OFFSET
    }

    /// Set **MAX_INCR** `[5:3]`
    ///
    /// set max continuous access for incr mode
    #[inline(always)]
    pub const fn with_max_incr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbConfig::MAX_INCR_MASK)
                | ((value << AhbConfig::MAX_INCR_OFFSET) & AhbConfig::MAX_INCR_MASK),
        }
    }

    /// Set **HBURST** `[2:0]`
    ///
    /// set hburst
    #[inline(always)]
    pub const fn with_hburst(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbConfig::HBURST_MASK)
                | ((value << AhbConfig::HBURST_OFFSET) & AhbConfig::HBURST_MASK),
        }
    }
}

impl Default for AhbConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbConfigVal {
        AhbConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbConfigVal) -> AhbConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLOCK_GATE Register
///
/// Clock gate control register
///
/// Address offset: 0x0044
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   The bit is used to enable clock gate when access all registers in this module.
#[repr(transparent)]
pub struct ClockGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClockGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClockGate {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// The bit is used to enable clock gate when access all registers in this module.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// The bit is used to enable clock gate when access all registers in this module.
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
}

/// CLOCK_GATE value type for batch read/modify/write operations
///
/// Use `clockgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clockgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClockGateVal {
    bits: u32,
}

impl ClockGateVal {
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

    /// Read **CLK_EN** `[0:0]`
    ///
    /// The bit is used to enable clock gate when access all registers in this module.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClockGate::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// The bit is used to enable clock gate when access all registers in this module.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClockGate::CLK_EN_MASK
            } else {
                self.bits & !ClockGate::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClockGateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClockGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClockGateVal {
        ClockGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClockGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClockGateVal) -> ClockGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Version control register
///
/// Address offset: 0x03fc
/// Reset value: 0x02211300
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   version control register. Note that this default value stored is the latest date when the hardware logic was updated.
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
    /// version control register. Note that this default value stored is the latest date when the hardware logic was updated.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// version control register. Note that this default value stored is the latest date when the hardware logic was updated.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// version control register. Note that this default value stored is the latest date when the hardware logic was updated.
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
        Self { bits: 0x02211300 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// version control register. Note that this default value stored is the latest date when the hardware logic was updated.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// version control register. Note that this default value stored is the latest date when the hardware logic was updated.
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
    /// mem start addr
    pub mem_start_addr: MemStartAddr,
    /// 0x0004
    /// mem end addr
    pub mem_end_addr: MemEndAddr,
    /// 0x0008
    /// mem current addr
    pub mem_current_addr: MemCurrentAddr,
    /// 0x000c
    /// mem addr update
    pub mem_addr_update: MemAddrUpdate,
    /// 0x0010
    /// fifo status register
    pub fifo_status: FifoStatus,
    /// 0x0014
    /// interrupt enable register
    pub intr_ena: IntrEna,
    /// 0x0018
    /// interrupt status register
    pub intr_raw: IntrRaw,
    /// 0x001c
    /// interrupt clear register
    pub intr_clr: IntrClr,
    /// 0x0020
    /// trigger register
    pub trigger: Trigger,
    /// 0x0024
    /// trace configuration register
    pub config: Config,
    /// 0x0028
    /// filter control register
    pub filter_control: FilterControl,
    /// 0x002c
    /// filter match control register
    pub filter_match_control: FilterMatchControl,
    /// 0x0030
    /// filter comparator match control register
    pub filter_comparator_control: FilterComparatorControl,
    /// 0x0034
    /// primary comparator match value
    pub filter_p_comparator_match: FilterPComparatorMatch,
    /// 0x0038
    /// secondary comparator match value
    pub filter_s_comparator_match: FilterSComparatorMatch,
    /// 0x003c
    /// resync configuration register
    pub resync_prolonged: ResyncProlonged,
    /// 0x0040
    /// AHB config register
    pub ahb_config: AhbConfig,
    /// 0x0044
    /// Clock gate control register
    pub clock_gate: ClockGate,
    _reserved0: [u8; 948],
    /// 0x03fc
    /// Version control register
    pub date: Date,
}
