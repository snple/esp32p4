//! The HP scratchpad, the small RAM the CPU reaches without a cache.
//!
//! The chip documents two tightly coupled blocks: the 768 KB `L2MEM`, which the
//! CPU reaches through the L1/L2 caches like any other memory, and the 8 KB **HP
//! SPM** at `0x3010_0000`. The register set calls the latter the *HP TCM*, and it
//! is what this module is about:
//!
//! - it is mapped at `0x3010_0000..0x3010_1fff`, the CPU's reset vector address;
//! - instruction and data accesses go over the CPU's own buses, two cycles each,
//!   with no cache in the way;
//! - it is the only RAM that keeps running when the flash/L2 caches are off,
//!   because nothing about it depends on them.
//!
//! # Putting code in it
//!
//! Two pieces are enough:
//!
//! 1. the `SPM` region in `memory.x`, which the application owns anyway:
//!
//!    ```text
//!    MEMORY
//!    {
//!        ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
//!        RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
//!        SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
//!    }
//!    ```
//!
//! 2. `#[tcmfunc]` on the functions and `#[tcmdata]` on the statics that belong
//!    there. `link.x` collects them into `.tcm.text` / `.tcm.data`, and the
//!    2nd-stage bootloader copies that segment to `0x3010_0000` before `_start`,
//!    exactly like `.data`.
//!
//! ```ignore
//! #[esp32p4_hal::tcmfunc]
//! fn toggles() -> u32 {
//!     // Runs from the scratchpad.
//! }
//! ```
//!
//! `#[ramfunc]` is the cheaper option: the same copy mechanism, but into the
//! ordinary RAM region. Choose it when the goal is only "survives a flash cache
//! suspension"; choose `#[tcmfunc]` when a couple of cycles per access matter.
//!
//! # Parity
//!
//! The scratchpad can detect single-bit flips. Checking needs a one-off
//! initialisation pass, which the bootloader does not do: [`init_parity`] runs
//! it and turns the check on. Do not touch the scratchpad while it runs.

use esp32p4_pac::HP_SYS;

/// First address of the scratchpad.
pub const BASE: usize = 0x3010_0000;
/// Size of the scratchpad.
pub const SIZE: usize = 0x2000;

/// Whether `addr` points into the scratchpad.
#[inline]
pub fn contains(addr: usize) -> bool {
    (BASE..BASE + SIZE).contains(&addr)
}

/// Turn on single-bit parity detection for the scratchpad.
///
/// Follows the TRM: start the initialisation, wait for it to finish, enable the
/// check, then stop the initialisation. Nothing may read or execute from the
/// scratchpad while it runs, so call this from flash (the default) and before
/// the first `#[tcmfunc]` call.
///
/// The wait is a plain spin: the hardware finishes in a few thousand cycles and
/// a timeout would only hide a wrong clock gate.
#[inline(never)]
pub fn init_parity() {
    // SAFETY: `HP_SYS` is always mapped; only its TCM configuration registers
    // are touched, and they are not owned by any other driver.
    let hp_sys = unsafe { &*HP_SYS::PTR };

    hp_sys.tcm_init.set_reg_tcm_init_en();
    hp_sys.tcm_init.clr_reg_tcm_init_cnt_reset();

    while !hp_sys.tcm_init.reg_tcm_init_done() {
        core::hint::spin_loop();
    }

    hp_sys.tcm_parity_check_ctrl.set_tcm_parity_check_en();
    hp_sys.tcm_init.clr_reg_tcm_init_en();
    hp_sys.tcm_init.set_reg_tcm_init_cnt_reset();
}

/// Whether the parity check is enabled.
#[inline]
pub fn parity_enabled() -> bool {
    // SAFETY: as in `init_parity`.
    let hp_sys = unsafe { &*HP_SYS::PTR };
    hp_sys.tcm_parity_check_ctrl.tcm_parity_check_en()
}

/// Whether a parity error is pending.
#[inline]
pub fn parity_error_pending() -> bool {
    // SAFETY: as in `init_parity`.
    let hp_sys = unsafe { &*HP_SYS::PTR };
    hp_sys.tcm_int_raw.tcm_parity_err_int_raw()
}

/// Drop a pending parity error.
#[inline]
pub fn clear_parity_error() {
    // SAFETY: as in `init_parity`.
    let hp_sys = unsafe { &*HP_SYS::PTR };
    hp_sys.tcm_int_clr.trigger_tcm_parity_err_int_clr();
}

unsafe extern "C" {
    /// First byte of the `#[tcmfunc]` code the bootloader copied in.
    static _stcmtext: u8;
    /// One past the last byte of the `#[tcmfunc]` code.
    static _etcmtext: u8;
    /// First byte of the `.tcm.data` the bootloader copied in.
    static _stcmdata: u8;
    /// One past the last byte of the `.tcm.data`.
    static _etcmdata: u8;
}

/// The range the linked `#[tcmfunc]` and `.tcm.data` objects occupy.
///
/// This is what the image actually placed in the scratchpad, which is easier to
/// verify than assuming the section starts at [`BASE`].
#[inline]
pub fn used_range() -> core::ops::Range<usize> {
    // The symbols are address constants emitted by `link.x`; taking their
    // address does not read the memory behind them.
    let (text_start, text_end, data_start, data_end) = (
        core::ptr::addr_of!(_stcmtext) as usize,
        core::ptr::addr_of!(_etcmtext) as usize,
        core::ptr::addr_of!(_stcmdata) as usize,
        core::ptr::addr_of!(_etcmdata) as usize,
    );
    let start = text_start.min(data_start);
    let end = text_end.max(data_end);
    start..end
}
