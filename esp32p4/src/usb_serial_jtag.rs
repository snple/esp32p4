//! USB Serial/JTAG (CDC-ACM) driver.
//!
//! The ESP32-P4 integrates a full-speed USB 2.0 device controller that
//! implements both a CDC-ACM serial port and a JTAG bridge, so a single USB
//! cable provides flashing (`espflash`), a console and a debugger — no UART
//! adapter is needed.
//!
//! # Data path
//!
//! ```text
//!        CPU                                   USB host
//!   ┌───────────┐   EP1 write    ┌──────────┐   bulk IN   ┌─────────┐
//!   │ write_byte├───────────────►│ TX FIFO  ├────────────►│ CDC-ACM │
//!   └───────────┘                │  64 byte │             │  device │
//!   ┌───────────┐   EP1 read     ├──────────┤   bulk OUT  │         │
//!   │ read_byte │◄───────────────┤ RX FIFO  │◄────────────┤         │
//!   └───────────┘                └──────────┘             └─────────┘
//! ```
//!
//! Bytes written to `EP1` land in the TX FIFO; they stay invisible to the host
//! until [`UsbSerialJtag::flush`] sets `EP1_CONF.WR_DONE`, which turns whatever
//! the FIFO holds into one USB packet (64 bytes at most). The FIFO reports
//! `SERIAL_IN_EP_DATA_FREE == 0` from then on until the host collected the
//! packet, so a full FIFO means "no host listening" just as much as "host is
//! busy".
//!
//! # Host presence
//!
//! There is no way to tell a busy host from an absent one, so the driver never
//! blocks forever unless asked to:
//!
//! | Method                          | Behaviour                                |
//! |---------------------------------|------------------------------------------|
//! | [`try_write_byte`](UsbSerialJtag::try_write_byte) | returns `false` when the FIFO is full |
//! | [`write`](UsbSerialJtag::write) | gives up after ~10 ms, returns the byte count |
//! | [`write_byte`](UsbSerialJtag::write_byte) | spins until there is room (may hang without a host) |
//! | [`flush`](UsbSerialJtag::flush) | gives up after ~10 ms, returns whether the packet was taken |
//!
//! # PHY sharing
//!
//! The chip has two full-speed PHYs shared by this controller and the OTG FS
//! core in [`USB_WRAP`]. By default the USB Serial/JTAG controller owns PHY 0;
//! [`UsbSerialJtag::select_phy`] moves it to PHY 1, which the OTG driver then
//! has to mirror. Do not touch it unless both ports are used at the same time.
//!
//! # Example
//!
//! ```ignore
//! use core::fmt::Write;
//! use esp32p4::usb_serial_jtag::UsbSerialJtag;
//!
//! let mut serial =
//!     UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
//! writeln!(serial, "hello from ESP32-P4").unwrap();
//! ```

use esp32p4_pac::{HP_SYS_CLKRST, LP_AON_CLKRST, LP_SYS, USB_DEVICE};

/// Bytes in one USB full-speed bulk packet, which is also the depth of each
/// CDC FIFO.
pub const MAX_PACKET_SIZE: usize = 64;

/// `EP1_CONF.WR_DONE`: hand the bytes stored in the TX FIFO to the host.
///
/// Written with a plain store instead of a read-modify-write: the other bits of
/// the register are read-only, and a RMW would sample `SERIAL_OUT_EP_DATA_AVAIL`
/// on the way through.
const EP1_CONF_WR_DONE: u32 = 1 << 0;

/// How long the driver waits for the host to collect a packet, in spin loops.
///
/// One USB full-speed frame is 1 ms, so a few milliseconds are plenty for a
/// host that is actually polling; roughly 10 ms at the default 360 MHz clock.
const HOST_TIMEOUT_SPINS: u32 = 1_000_000;

/// CDC-ACM serial port over the USB Serial/JTAG controller.
pub struct UsbSerialJtag {
    usb: USB_DEVICE,
}

impl UsbSerialJtag {
    /// Enable the clocks and bring the internal PHY up.
    ///
    /// The clock/reset blocks are borrowed rather than taken: this driver only
    /// flips two clock-gate bits, so the same peripherals stay available for
    /// e.g. [`Pins::init`](crate::gpio::Pins::init).
    ///
    /// The module is *not* reset: a reset makes the device disappear from the
    /// host for a few hundred milliseconds, and `espflash monitor` would have to
    /// reopen the port. Everything the driver relies on is either idempotent or
    /// restored here, so a running bootloader session keeps working.
    pub fn init(usb: USB_DEVICE, clkrst: &HP_SYS_CLKRST, aon: &LP_AON_CLKRST) -> Self {
        // APB clock for the register block, plus the 48 MHz clock of the
        // internal full-speed PHY.
        clkrst.soc_clk_ctrl2.set_reg_usb_device_apb_clk_en();
        aon.lp_aonclkrst_hp_usb_clkrst_ctrl0
            .set_lp_aonclkrst_usb_device_48m_clk_en();

        // The driver polls, so keep every interrupt masked. The bootloader may
        // have armed some of them; a pending one would trap a core without a
        // registered handler.
        usb.int_ena.write_bits(0);
        usb.int_clr.write_bits(u32::MAX);

        // Internal PHY, pads enabled. Both are the documented reset values, but
        // a JTAG session or a network-less power cycle can leave them inverted.
        usb.conf0.clr_usb_serial_jtag_phy_sel();
        usb.conf0.set_usb_serial_jtag_usb_pad_enable();

        Self { usb }
    }

    /// Give the peripheral back.
    #[inline]
    pub fn free(self) -> USB_DEVICE {
        self.usb
    }

    /// Whether the TX FIFO has room for at least one byte.
    #[inline]
    pub fn is_tx_free(&self) -> bool {
        self.usb.ep1_conf.usb_serial_jtag_serial_in_ep_data_free()
    }

    /// Whether the RX FIFO holds a byte.
    #[inline]
    pub fn is_rx_ready(&self) -> bool {
        self.usb.ep1_conf.usb_serial_jtag_serial_out_ep_data_avail()
    }

    /// Queue one byte, returning `false` if the TX FIFO is full.
    pub fn try_write_byte(&self, byte: u8) -> bool {
        if !self.is_tx_free() {
            return false;
        }
        self.usb.ep1.write_bits(byte as u32);
        true
    }

    /// Queue one byte, spinning while the TX FIFO is full.
    ///
    /// This hangs forever when no host is attached and the FIFO is full, so
    /// prefer [`write`](Self::write) or [`try_write_byte`](Self::try_write_byte)
    /// in code that may run without a cable.
    pub fn write_byte(&self, byte: u8) {
        while !self.try_write_byte(byte) {
            self.flush();
        }
    }

    /// Queue as much of `data` as the host accepts.
    ///
    /// Full packets are handed over on the way; the rest is dropped once the
    /// host stops collecting. Flushing the tail is left to the caller.
    ///
    /// Returns the number of bytes queued.
    pub fn write(&self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            // A full FIFO means the 64-byte packet is waiting for the host:
            // hand it over and retry the byte that did not fit.
            if !self.try_write_byte(byte) && (!self.flush() || !self.try_write_byte(byte)) {
                break;
            }
            written += 1;
        }
        written
    }

    /// Queue a string.
    pub fn write_str(&self, s: &str) -> usize {
        self.write(s.as_bytes())
    }

    /// Hand everything queued so far to the host and wait until it is collected.
    ///
    /// Returns `false` if the host did not pick up the packet within ~10 ms,
    /// which normally means there is no cable attached. The bytes are then
    /// still in the FIFO; a later flush retries them.
    pub fn flush(&self) -> bool {
        self.usb.ep1_conf.write_bits(EP1_CONF_WR_DONE);

        // The host only gets to see `WR_DONE` after a few PHY cycles; without
        // this breather the FIFO may still advertise free space and the caller
        // would queue a second packet into the same 64-byte buffer.
        for _ in 0..16 {
            core::hint::spin_loop();
        }

        let mut spins = HOST_TIMEOUT_SPINS;
        while !self.is_tx_free() {
            if spins == 0 {
                return false;
            }
            spins -= 1;
            core::hint::spin_loop();
        }
        true
    }

    /// Pop one byte received from the host.
    pub fn read_byte(&self) -> Option<u8> {
        if self.is_rx_ready() {
            Some(self.usb.ep1.read_bits() as u8)
        } else {
            None
        }
    }

    /// Pop one byte, spinning until the host sends one.
    pub fn read_byte_blocking(&self) -> u8 {
        loop {
            if let Some(byte) = self.read_byte() {
                return byte;
            }
            core::hint::spin_loop();
        }
    }

    /// Pop everything the host has sent so far.
    ///
    /// Returns the number of bytes stored in `buffer`.
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        let mut read = 0;
        for slot in buffer.iter_mut() {
            match self.read_byte() {
                Some(byte) => {
                    *slot = byte;
                    read += 1;
                }
                None => break,
            }
        }
        read
    }

    /// Whether the host may reset the chip over the CDC port.
    ///
    /// The default (`true`) lets `espflash` drop the chip into download mode by
    /// toggling DTR/RTS. Disable it when a stray terminal closing would
    /// otherwise reboot the device.
    pub fn set_chip_reset_allowed(&self, allowed: bool) {
        if allowed {
            self.usb
                .chip_rst
                .clr_usb_serial_jtag_usb_uart_chip_rst_dis();
        } else {
            self.usb
                .chip_rst
                .set_usb_serial_jtag_usb_uart_chip_rst_dis();
        }
    }

    /// Map this controller onto PHY 0 (the default) or PHY 1.
    ///
    /// The other controller gets the remaining PHY, so this is only useful when
    /// the USB Serial/JTAG port and the OTG FS port are both active.
    pub fn select_phy(&self, phy: u8, lp_sys: &LP_SYS) {
        // Take the PHY mapping away from hardware arbitration before choosing.
        lp_sys.usb_ctrl.set_sw_hw_usb_phy_sel();
        if phy == 0 {
            lp_sys.usb_ctrl.clr_sw_usb_phy_sel();
        } else {
            lp_sys.usb_ctrl.set_sw_usb_phy_sel();
        }
    }
}

impl core::fmt::Debug for UsbSerialJtag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UsbSerialJtag")
            .field("tx_free", &self.is_tx_free())
            .field("rx_ready", &self.is_rx_ready())
            .finish()
    }
}

impl core::fmt::Write for UsbSerialJtag {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        self.flush();
        Ok(())
    }
}

// ============================================================================
// `defmt` transport
// ============================================================================

/// `defmt` transport over the CDC-ACM port.
///
/// `espflash monitor` decodes the frames without any extra hardware, which
/// makes this the most convenient logger on a board that only has a USB-C
/// connector. Enabling the `defmt-usb` feature registers it as *the*
/// global logger, so it cannot be combined with `defmt-rtt`.
#[cfg(feature = "defmt-usb")]
pub mod logger {
    use core::cell::UnsafeCell;
    use core::sync::atomic::{AtomicBool, Ordering};

    use esp32p4_pac::USB_DEVICE;

    use super::{EP1_CONF_WR_DONE, HOST_TIMEOUT_SPINS};

    /// Tells `espflash` that the bytes leaving this port are `defmt` frames, so
    /// its monitor decodes them instead of printing them raw.
    ///
    /// `espflash` reads the symbol from the ELF (the section is not part of the
    /// image). This is the marker `esp-println` emits for its `defmt-espflash`
    /// feature; without it a monitor needs an explicit `--log-format defmt`.
    #[unsafe(link_section = ".espressif.metadata")]
    #[used]
    #[unsafe(export_name = "espflash.LOG_FORMAT")]
    static LOG_FORMAT: [u8; 14] = *b"defmt-espflash";

    /// Global `defmt` logger writing to the USB Serial/JTAG port.
    #[defmt::global_logger]
    struct Logger;

    /// Encoder state shared by all execution contexts.
    struct State {
        /// `true` between `acquire` and `release`.
        taken: AtomicBool,
        /// Needed to leave the critical section on `release`.
        restore: UnsafeCell<critical_section::RestoreState>,
        /// `defmt` frame encoder.
        encoder: UnsafeCell<defmt::Encoder>,
    }

    // SAFETY: every access goes through a critical section, see `acquire`.
    unsafe impl Sync for State {}

    static STATE: State = State {
        taken: AtomicBool::new(false),
        restore: UnsafeCell::new(critical_section::RestoreState::invalid()),
        encoder: UnsafeCell::new(defmt::Encoder::new()),
    };

    /// The peripheral's register block.
    ///
    /// The logger is a global singleton without an owning handle, so it reaches
    /// the registers by pointer. Only the FIFO registers are touched, and only
    /// from within a critical section.
    #[inline]
    fn regs() -> &'static esp32p4_pac::usb_device::RegisterBlock {
        // SAFETY: the register block is plain MMIO at a fixed address.
        unsafe { &*USB_DEVICE::PTR }
    }

    /// Whether [`UsbSerialJtag::init`](super::UsbSerialJtag::init) has run.
    ///
    /// A log statement before the clock gate is open would touch an unclocked
    /// APB peripheral, so the frames are dropped until the driver owns it.
    #[inline]
    fn running() -> bool {
        // SAFETY: the clock/reset block is always clocked.
        let clkrst = unsafe { &*esp32p4_pac::HP_SYS_CLKRST::PTR };
        clkrst.soc_clk_ctrl2.reg_usb_device_apb_clk_en()
    }

    /// Set once a flush timed out, so the next frames are dropped without
    /// waiting again. Cleared as soon as the FIFO reports room, which means the
    /// host came back.
    static STALLED: AtomicBool = AtomicBool::new(false);

    /// Queue one byte, dropping it when the FIFO stays full.
    #[inline]
    fn push(byte: u8) {
        if !running() {
            return;
        }

        let usb = regs();
        if !usb.ep1_conf.usb_serial_jtag_serial_in_ep_data_free() {
            // No host, or one that stopped reading: drop the frame instead of
            // stalling the core inside a log statement.
            if STALLED.load(Ordering::Relaxed) {
                return;
            }
            let mut spins = HOST_TIMEOUT_SPINS;
            while !usb.ep1_conf.usb_serial_jtag_serial_in_ep_data_free() {
                if spins == 0 {
                    STALLED.store(true, Ordering::Relaxed);
                    return;
                }
                spins -= 1;
                core::hint::spin_loop();
            }
        } else {
            STALLED.store(false, Ordering::Relaxed);
        }

        usb.ep1.write_bits(byte as u32);
    }

    #[inline]
    fn sink(bytes: &[u8]) {
        for &byte in bytes {
            push(byte);
        }
    }

    /// Hand the queued bytes to the host as one packet.
    #[inline]
    fn hand_over() {
        if !running() || STALLED.load(Ordering::Relaxed) {
            return;
        }

        let usb = regs();
        usb.ep1_conf.write_bits(EP1_CONF_WR_DONE);
        for _ in 0..16 {
            core::hint::spin_loop();
        }
        let mut spins = HOST_TIMEOUT_SPINS;
        while !usb.ep1_conf.usb_serial_jtag_serial_in_ep_data_free() {
            if spins == 0 {
                STALLED.store(true, Ordering::Relaxed);
                return;
            }
            spins -= 1;
            core::hint::spin_loop();
        }
    }

    unsafe impl defmt::Logger for Logger {
        fn acquire() {
            // SAFETY: released by the matching `release`.
            let restore = unsafe { critical_section::acquire() };

            if STATE.taken.load(Ordering::Relaxed) {
                panic!("defmt logger taken reentrantly");
            }
            STATE.taken.store(true, Ordering::Relaxed);

            // SAFETY: the critical section guarantees exclusive access.
            unsafe {
                STATE.restore.get().write(restore);
                // Non-UTF-8 frame delimiter, the same one `esp-println` writes:
                // it lets the host find frame boundaries even when plain text is
                // interleaved on the same port.
                sink(&[0xff, 0x00]);
                (*STATE.encoder.get()).start_frame(sink);
            }
        }

        unsafe fn write(bytes: &[u8]) {
            // SAFETY: called between `acquire` and `release`.
            unsafe {
                (*STATE.encoder.get()).write(bytes, sink);
            }
        }

        unsafe fn flush() {
            hand_over();
        }

        unsafe fn release() {
            if !STATE.taken.load(Ordering::Relaxed) {
                panic!("defmt release out of context");
            }

            // SAFETY: called between `acquire` and `release`.
            unsafe {
                (*STATE.encoder.get()).end_frame(sink);
                let restore = STATE.restore.get().read();
                STATE.taken.store(false, Ordering::Relaxed);
                // One packet per frame keeps the monitor's output line-based.
                if running() {
                    regs().ep1_conf.write_bits(EP1_CONF_WR_DONE);
                }
                critical_section::release(restore);
            }
        }
    }
}
