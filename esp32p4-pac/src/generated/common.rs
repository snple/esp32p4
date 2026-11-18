//! Common types and utilities for register access

#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> VolatileCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}
