//! DesignWare Core (DWC) USB 2.0 OTG registers — device-mode subset.
//!
//! The DWC register map is not part of the chip's SVD (the IP vendor keeps the
//! detailed description under NDA), so it cannot be generated into
//! [`esp32p4_pac`]. The layout below is written by hand from the register
//! structures used by ESP-IDF (`soc/usb_dwc_struct.h`), limited to the registers
//! a device-mode, slave-mode (CPU-driven FIFO) driver needs.
//!
//! # Address map
//!
//! | Instance | Base       | FIFO depth (words) |
//! |----------|------------|--------------------|
//! | OTG FS   | `0x5004_0000` | 200             |
//! | OTG HS   | `0x5000_0000` | 896             |
//!
//! The exact FIFO depth is read back from `GHWCFG3.DFIFODEPTH` at run time; the
//! values in the table are documentation only.
//!
//! # FIFO windows
//!
//! ```text
//!   base + 0x1000 * (n + 1)   push into IN endpoint n's TX FIFO
//!   base + 0x1000             pop from the shared RX FIFO
//! ```
//!
//! Every access is a 32-bit word; a packet shorter than four bytes still
//! occupies one word, and the byte count comes from the RX status entry.

// The module spells out the device-mode register map, including bits the driver
// does not need yet; keeping them documents the hardware and makes the next
// feature cheap to add.
#![allow(dead_code)]

/// Byte offset of the first FIFO window (`EP0 IN`).
///
/// Window `n` is at `FIFO_BASE + 0x1000 * n`; the RX FIFO is window 0 and IN
/// endpoint `n` uses window `n + 1`.
pub(crate) const FIFO_WINDOW: usize = 0x1000;

/// Register offsets, relative to the controller base address.
pub(crate) mod reg {
    pub const GOTGCTL: usize = 0x000;
    pub const GOTGINT: usize = 0x004;
    pub const GAHBCFG: usize = 0x008;
    pub const GUSBCFG: usize = 0x00c;
    pub const GRSTCTL: usize = 0x010;
    pub const GINTSTS: usize = 0x014;
    pub const GINTMSK: usize = 0x018;
    pub const GRXSTSP: usize = 0x020;
    pub const GRXFSIZ: usize = 0x024;
    /// `DIEPTXF0` in device mode.
    pub const DIEPTXF0: usize = 0x028;
    pub const GHWCFG2: usize = 0x048;
    pub const GHWCFG3: usize = 0x04c;
    /// `DIEPTXF1..15` start here, four bytes apart.
    pub const DIEPTXF: usize = 0x104;

    pub const DCFG: usize = 0x800;
    pub const DCTL: usize = 0x804;
    pub const DSTS: usize = 0x808;
    pub const DIEPMSK: usize = 0x810;
    pub const DOEPMSK: usize = 0x814;
    pub const DAINT: usize = 0x818;
    pub const DAINTMSK: usize = 0x81c;

    /// IN endpoint 0 control register; endpoint `n` is at `DIEPCTL0 + 0x20 * n`.
    pub const DIEPCTL0: usize = 0x900;
    /// IN endpoint 0 interrupt register.
    pub const DIEPINT0: usize = 0x908;
    /// IN endpoint 0 transfer-size register.
    pub const DIEPTSIZ0: usize = 0x910;
    /// IN endpoint 0 FIFO-space register.
    pub const DTXFSTS0: usize = 0x918;

    /// OUT endpoint 0 control register; endpoint `n` is at `DOEPCTL0 + 0x20 * n`.
    pub const DOEPCTL0: usize = 0xb00;
    /// OUT endpoint 0 interrupt register.
    pub const DOEPINT0: usize = 0xb08;
    /// OUT endpoint 0 transfer-size register.
    pub const DOEPTSIZ0: usize = 0xb10;

    pub const PCGCCTL: usize = 0xe00;

    /// Address of `DIEPCTL<n>`.
    #[inline]
    pub const fn diepctl(n: usize) -> usize {
        DIEPCTL0 + 0x20 * n
    }

    /// Address of `DIEPINT<n>`.
    #[inline]
    pub const fn diepint(n: usize) -> usize {
        DIEPINT0 + 0x20 * n
    }

    /// Address of `DIEPTSIZ<n>`.
    #[inline]
    pub const fn dieptsiz(n: usize) -> usize {
        DIEPTSIZ0 + 0x20 * n
    }

    /// Address of `DTXFSTS<n>`.
    #[inline]
    pub const fn dtxfsts(n: usize) -> usize {
        DTXFSTS0 + 0x20 * n
    }

    /// Address of `DOEPCTL<n>`.
    #[inline]
    pub const fn doepctl(n: usize) -> usize {
        DOEPCTL0 + 0x20 * n
    }

    /// Address of `DOEPINT<n>`.
    #[inline]
    pub const fn doepint(n: usize) -> usize {
        DOEPINT0 + 0x20 * n
    }

    /// Address of `DOEPTSIZ<n>`.
    #[inline]
    pub const fn doeptsiz(n: usize) -> usize {
        DOEPTSIZ0 + 0x20 * n
    }

    /// Address of `DIEPTXF<n>`, the TX FIFO size of IN endpoint `n` (1..=15).
    #[inline]
    pub const fn dieptxf(n: usize) -> usize {
        debug_assert!(n >= 1);
        DIEPTXF + 4 * (n - 1)
    }
}

/// `GOTGCTL` bits.
pub(crate) mod gotgctl {
    /// Software override of the B-session-valid input.
    pub const BVALIDOVEN: u32 = 1 << 6;
    /// Value driven while [`BVALIDOVEN`] is set.
    pub const BVALIDOVVAL: u32 = 1 << 7;
    /// Connector ID status: 0 = A-device (host), 1 = B-device (peripheral).
    pub const CONIDSTS: u32 = 1 << 16;
    /// A-session valid.
    pub const ASESVLD: u32 = 1 << 18;
    /// B-session valid.
    pub const BSESVLD: u32 = 1 << 19;
}

/// `GAHBCFG` bits.
pub(crate) mod gahbcfg {
    /// Global interrupt enable. Left clear: this driver polls.
    pub const GLBLINTRMSK: u32 = 1 << 0;
    /// DMA enable. Left clear: slave mode, the CPU moves the payloads.
    pub const DMAEN: u32 = 1 << 5;
}

/// `GUSBCFG` bits.
pub(crate) mod gusbcfg {
    pub const TOCAL_SHIFT: u32 = 0;
    /// PHY interface: 1 = UTMI+, 0 = full-speed serial.
    pub const PHYIF: u32 = 1 << 3;
    /// ULPI/UTMI select.
    pub const ULPI_UTMI_SEL: u32 = 1 << 4;
    /// Full-speed serial interface select.
    pub const FSINTF: u32 = 1 << 5;
    /// PHY select: 1 = embedded full-speed PHY.
    pub const PHYSEL: u32 = 1 << 6;
    pub const DDR_SEL: u32 = 1 << 7;
    pub const SRPCAP: u32 = 1 << 8;
    pub const HNPCAP: u32 = 1 << 9;
    /// USB turnaround time, `[13:10]`.
    pub const USBTRDTIM_SHIFT: u32 = 10;
    pub const USBTRDTIM_MASK: u32 = 0xf << USBTRDTIM_SHIFT;
    pub const ULPIFSLS: u32 = 1 << 17;
    pub const ULPIAUTORES: u32 = 1 << 18;
    pub const ULPICLK_SUSP: u32 = 1 << 19;
    pub const ULPI_EXT_VBUS_DRV: u32 = 1 << 20;
    pub const ULPI_EXT_VBUS_IND: u32 = 1 << 21;
    pub const TERMSELDLPULSE: u32 = 1 << 22;
    pub const ULPI: u32 = 1 << 25;
    /// Force host mode.
    pub const FHMOD: u32 = 1 << 29;
    /// Force device mode.
    pub const FDMOD: u32 = 1 << 30;
}

/// `GRSTCTL` bits.
pub(crate) mod grstctl {
    /// Core soft reset.
    pub const CS_RST: u32 = 1 << 0;
    /// Flush the shared RX FIFO.
    pub const RXFFLSH: u32 = 1 << 4;
    /// Flush one or all TX FIFOs, selected by [`TXFNUM_SHIFT`].
    pub const TXFFLSH: u32 = 1 << 5;
    pub const TXFNUM_SHIFT: u32 = 6;
    /// `TXFNUM` value that selects every TX FIFO.
    pub const TXFNUM_ALL: u32 = 0x10;
    /// Set once the core soft reset completed (v4.30a and later).
    pub const CS_RSTDONE: u32 = 1 << 29;
    /// AHB master is idle.
    pub const AHBIDLE: u32 = 1 << 31;
}

/// `GINTSTS` bits.
pub(crate) mod gintsts {
    /// 0 = device mode, 1 = host mode.
    pub const CURMOD: u32 = 1 << 0;
    pub const RXFLVL: u32 = 1 << 4;
    pub const ERLYSUSP: u32 = 1 << 10;
    pub const USBSUSP: u32 = 1 << 11;
    pub const USBRST: u32 = 1 << 12;
    pub const ENUMDNE: u32 = 1 << 13;
    pub const IEPINT: u32 = 1 << 18;
    pub const OEPINT: u32 = 1 << 19;
    pub const WKUPINT: u32 = 1 << 31;
}

/// `GRXSTSP` fields (device mode).
pub(crate) mod grxstsp {
    /// Endpoint number, `[3:0]`.
    pub const EPNUM_MASK: u32 = 0xf;
    /// Byte count of the packet, `[14:4]`.
    pub const BCNT_SHIFT: u32 = 4;
    pub const BCNT_MASK: u32 = 0x7ff << BCNT_SHIFT;
    /// Data PID, `[16:15]`.
    pub const DPID_SHIFT: u32 = 15;
    pub const DPID_MASK: u32 = 0x3 << DPID_SHIFT;
    /// Packet status, `[20:17]`.
    pub const PKTSTS_SHIFT: u32 = 17;
    pub const PKTSTS_MASK: u32 = 0xf << PKTSTS_SHIFT;

    /// OUT data packet received.
    pub const PKTSTS_OUT_DATA: u32 = 0b0010;
    /// OUT transfer completed.
    pub const PKTSTS_OUT_DONE: u32 = 0b0011;
    /// SETUP transaction completed.
    pub const PKTSTS_SETUP_DONE: u32 = 0b0100;
    /// SETUP data packet received.
    pub const PKTSTS_SETUP_DATA: u32 = 0b0110;
}

/// `GRXFSIZ` / `DIEPTXF` fields.
pub(crate) mod fifo_siz {
    /// FIFO depth in words, `[31:16]` of `DIEPTXF` and `[15:0]` of `GRXFSIZ`.
    pub const DEPTH_SHIFT: u32 = 16;
    pub const DEPTH_MASK: u32 = 0xffff << DEPTH_SHIFT;
    /// Start address in words, `[15:0]` of `DIEPTXF`.
    pub const START_MASK: u32 = 0xffff;
}

/// `GHWCFG2` fields.
pub(crate) mod ghwcfg2 {
    /// High-speed PHY type, `[7:6]`: 0 = none, 1 = UTMI+.
    pub const HSPHYTYPE_SHIFT: u32 = 6;
    pub const HSPHYTYPE_MASK: u32 = 0x3 << HSPHYTYPE_SHIFT;
    /// Full-speed PHY type, `[9:8]`: 0 = none, 1 = full-speed serial.
    pub const FSPHYTYPE_SHIFT: u32 = 8;
    pub const FSPHYTYPE_MASK: u32 = 0x3 << FSPHYTYPE_SHIFT;
}

/// `GHWCFG3` fields.
pub(crate) mod ghwcfg3 {
    /// Total DFIFO depth in words, `[31:16]`.
    pub const DFIFO_DEPTH_SHIFT: u32 = 16;
    pub const DFIFO_DEPTH_MASK: u32 = 0xffff << DFIFO_DEPTH_SHIFT;
}

/// `DCFG` fields.
pub(crate) mod dcfg {
    /// Device speed, `[1:0]`.
    pub const DEVSPD_SHIFT: u32 = 0;
    pub const DEVSPD_MASK: u32 = 0x3;
    /// Device address, `[10:4]`.
    pub const DEVADDR_SHIFT: u32 = 4;
    pub const DEVADDR_MASK: u32 = 0x7f << DEVADDR_SHIFT;
    /// Periodic frame interval, `[12:11]`.
    pub const PERFRINT_SHIFT: u32 = 11;
}

/// `DCTL` bits.
pub(crate) mod dctl {
    /// Remote wake-up signalling.
    pub const RMTWKUPSIG: u32 = 1 << 0;
    /// Soft disconnect: 1 drives SE0 and removes the D+ pull-up.
    pub const SFTRST_DISCON: u32 = 1 << 1;
    /// Clear global NAK.
    pub const CGNPINNAK: u32 = 1 << 8;
}

/// `DSTS` fields.
pub(crate) mod dsts {
    pub const SUSPSTS: u32 = 1 << 0;
    /// Enumerated speed, `[2:1]`: 0 = high, 1 = full (30/60 MHz), 2 = low, 3 = full (48 MHz).
    pub const ENUMSPD_SHIFT: u32 = 1;
    pub const ENUMSPD_MASK: u32 = 0x3 << ENUMSPD_SHIFT;
    /// Frame number, `[21:8]`.
    pub const SOFFN_SHIFT: u32 = 8;
}

/// `DIEPCTL` / `DOEPCTL` bits (endpoints 1..=15).
pub(crate) mod epctl {
    /// Maximum packet size, `[10:0]`.
    pub const MPS_MASK: u32 = 0x7ff;
    /// USB active endpoint.
    pub const USBACTEP: u32 = 1 << 15;
    /// Data PID (read-only).
    pub const DPID: u32 = 1 << 16;
    /// NAK status (read-only).
    pub const NAKSTS: u32 = 1 << 17;
    /// Endpoint type, `[19:18]`.
    pub const EPTYPE_SHIFT: u32 = 18;
    pub const EPTYPE_CONTROL: u32 = 0b00 << EPTYPE_SHIFT;
    pub const EPTYPE_ISO: u32 = 0b01 << EPTYPE_SHIFT;
    pub const EPTYPE_BULK: u32 = 0b10 << EPTYPE_SHIFT;
    pub const EPTYPE_INTR: u32 = 0b11 << EPTYPE_SHIFT;
    /// Snoop mode.
    pub const SNP: u32 = 1 << 20;
    /// STALL handshake.
    pub const STALL: u32 = 1 << 21;
    /// TX FIFO number, `[25:22]`.
    pub const TXFNUM_SHIFT: u32 = 22;
    pub const TXFNUM_MASK: u32 = 0xf << TXFNUM_SHIFT;
    /// Clear NAK.
    pub const CNAK: u32 = 1 << 26;
    /// Set NAK.
    pub const SNAK: u32 = 1 << 27;
    /// Set DATA0 PID.
    pub const SETD0PID: u32 = 1 << 28;
    /// Set DATA1 PID.
    pub const SETD1PID: u32 = 1 << 29;
    /// Endpoint disable.
    pub const EPDIS: u32 = 1 << 30;
    /// Endpoint enable.
    pub const EPENA: u32 = 1 << 31;
}

/// `DIEPCTL0` / `DOEPCTL0` bits.
///
/// Endpoint 0 encodes the packet size in two bits instead of the eleven used by
/// the other endpoints.
pub(crate) mod ep0ctl {
    /// Packet size: `0b00` = 64, `0b01` = 32, `0b10` = 16, `0b11` = 8.
    pub const MPS_MASK: u32 = 0x3;
    pub const MPS_64: u32 = 0b00;
    pub const MPS_32: u32 = 0b01;
    pub const MPS_16: u32 = 0b10;
    pub const MPS_8: u32 = 0b11;
    pub const USBACTEP: u32 = 1 << 15;
    pub const NAKSTS: u32 = 1 << 17;
    pub const EPTYPE_CONTROL: u32 = 0b00 << 18;
    pub const STALL: u32 = 1 << 21;
    pub const CNAK: u32 = 1 << 26;
    pub const SNAK: u32 = 1 << 27;
    pub const EPDIS: u32 = 1 << 30;
    pub const EPENA: u32 = 1 << 31;
}

/// `DIEPINT` / `DOEPINT` bits.
pub(crate) mod epint {
    /// Transfer completed.
    pub const XFERCOMPL: u32 = 1 << 0;
    /// Endpoint disabled.
    pub const EPDISBLD: u32 = 1 << 1;
    /// AHB error.
    pub const AHBERR: u32 = 1 << 2;
    /// SETUP phase done (OUT endpoints only).
    pub const SETUP: u32 = 1 << 3;
    /// Timeout condition (IN endpoints only).
    pub const TIMEOUT: u32 = 1 << 3;
    /// IN token received while the TX FIFO was empty (IN endpoints only).
    pub const INTKN_TXFEMP: u32 = 1 << 4;
    /// OUT token received when the endpoint was disabled (OUT endpoints only).
    pub const OUTTKNEPDIS: u32 = 1 << 4;
    /// IN token received while the endpoint was NAKing (IN endpoints only).
    pub const INEPNAKEFF: u32 = 1 << 6;
    /// Status phase received (OUT endpoints only).
    pub const STSPHSERCVD: u32 = 1 << 5;
    /// Back-to-back SETUP packets (OUT endpoints only).
    pub const B2BSETUP: u32 = 1 << 6;
    /// TX FIFO empty (IN endpoints only).
    pub const TXFEMP: u32 = 1 << 7;
    /// OUT packet error (OUT endpoints only).
    pub const OUTPKTERR: u32 = 1 << 8;
    /// TX FIFO underrun (IN endpoints only).
    pub const TXFIFOUNDRN: u32 = 1 << 8;
    /// BNA interrupt.
    pub const BNAINTR: u32 = 1 << 9;
    /// Packet dropped status.
    pub const PKTDRPSTS: u32 = 1 << 11;
    /// Babble error.
    pub const BBLEERR: u32 = 1 << 12;
    /// NAK interrupt.
    pub const NAK: u32 = 1 << 13;
    /// NYET interrupt.
    pub const NYET: u32 = 1 << 14;
    /// SETUP packet received (OUT endpoints only).
    pub const STUPPKTRCVD: u32 = 1 << 15;
}

/// `DIEPTSIZ0` fields.
pub(crate) mod dieptsiz0 {
    /// Transfer size in bytes, `[6:0]`.
    pub const XFERSIZE_MASK: u32 = 0x7f;
    /// Packet count, `[20:19]`.
    pub const PKTCNT_SHIFT: u32 = 19;
    pub const PKTCNT_MASK: u32 = 0x3 << PKTCNT_SHIFT;
}

/// `DOEPTSIZ0` fields.
pub(crate) mod doeptsiz0 {
    /// Transfer size in bytes, `[6:0]`.
    pub const XFERSIZE_MASK: u32 = 0x7f;
    /// Packet count, `[19]`.
    pub const PKTCNT: u32 = 1 << 19;
    /// SETUP packet count, `[30:29]`.
    pub const SUPCNT_SHIFT: u32 = 29;
    pub const SUPCNT_MASK: u32 = 0x3 << SUPCNT_SHIFT;
}

/// `DIEPTSIZ` / `DOEPTSIZ` fields (endpoints 1..=15).
pub(crate) mod eptsiz {
    /// Transfer size in bytes, `[18:0]`.
    pub const XFERSIZE_MASK: u32 = 0x7_ffff;
    /// Packet count, `[28:19]`.
    pub const PKTCNT_SHIFT: u32 = 19;
    pub const PKTCNT_MASK: u32 = 0x3ff << PKTCNT_SHIFT;
    /// Multi-count, `[30:29]` (IN endpoints only).
    pub const MC_SHIFT: u32 = 29;
}

/// `PCGCCTL` bits.
pub(crate) mod pcgcctl {
    pub const STOPPCLK: u32 = 1 << 0;
    pub const GATEHCLK: u32 = 1 << 1;
}

use reg::*;

/// A DWC OTG controller instance.
#[derive(Clone, Copy)]
pub(crate) struct Dwc {
    base: usize,
}

impl Dwc {
    /// Wrap a controller register block at `base`.
    pub(crate) const fn new(base: usize) -> Self {
        Self { base }
    }

    #[inline]
    fn ptr(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    /// Read `offset`.
    #[inline]
    pub(crate) fn read(&self, offset: usize) -> u32 {
        // SAFETY: `offset` is one of the register offsets above and the block is
        // mapped for the lifetime of the program.
        unsafe { self.ptr(offset).read_volatile() }
    }

    /// Write `value` to `offset`.
    #[inline]
    pub(crate) fn write(&self, offset: usize, value: u32) {
        // SAFETY: see `read`.
        unsafe { self.ptr(offset).write_volatile(value) }
    }

    /// Set the bits of `mask`.
    #[inline]
    pub(crate) fn set(&self, offset: usize, mask: u32) {
        self.write(offset, self.read(offset) | mask);
    }

    /// Clear the bits of `mask`.
    #[inline]
    pub(crate) fn clear(&self, offset: usize, mask: u32) {
        self.write(offset, self.read(offset) & !mask);
    }

    /// Replace the bits of `mask` with `value`.
    #[inline]
    pub(crate) fn modify(&self, offset: usize, mask: u32, value: u32) {
        let old = self.read(offset) & !mask;
        self.write(offset, old | (value & mask));
    }

    /// A window that pushes into IN endpoint `ep`'s TX FIFO.
    #[inline]
    pub(crate) fn tx_fifo(&self, ep: usize) -> Fifo {
        Fifo {
            ptr: self.ptr(FIFO_WINDOW * (ep + 1)),
        }
    }

    /// The shared RX FIFO window.
    #[inline]
    pub(crate) fn rx_fifo(&self) -> Fifo {
        Fifo {
            ptr: self.ptr(FIFO_WINDOW),
        }
    }

    /// Pop the next RX status entry.
    #[inline]
    pub(crate) fn rx_status(&self) -> u32 {
        self.read(GRXSTSP)
    }

    /// Whether the RX FIFO holds at least one status entry.
    #[inline]
    pub(crate) fn rx_level(&self) -> bool {
        self.read(GINTSTS) & gintsts::RXFLVL != 0
    }

    /// Soft-reset the core and wait until the AHB master is idle again.
    pub(crate) fn soft_reset(&self) {
        self.set(GRSTCTL, grstctl::CS_RST);
        // `CSftRstDone` only exists on v4.30a and later (the HS instance), so
        // wait for the bus-idle bit that every version implements.
        while self.read(GRSTCTL) & grstctl::AHBIDLE == 0 {
            core::hint::spin_loop();
        }
    }

    /// Flush every TX FIFO.
    pub(crate) fn flush_tx_fifos(&self) {
        self.write(
            GRSTCTL,
            grstctl::TXFFLSH | (grstctl::TXFNUM_ALL << grstctl::TXFNUM_SHIFT),
        );
        while self.read(GRSTCTL) & grstctl::TXFFLSH != 0 {
            core::hint::spin_loop();
        }
    }

    /// Flush one TX FIFO.
    pub(crate) fn flush_tx_fifo(&self, ep: usize) {
        self.write(
            GRSTCTL,
            grstctl::TXFFLSH | ((ep as u32) << grstctl::TXFNUM_SHIFT),
        );
        while self.read(GRSTCTL) & grstctl::TXFFLSH != 0 {
            core::hint::spin_loop();
        }
    }

    /// Flush the shared RX FIFO.
    pub(crate) fn flush_rx_fifo(&self) {
        self.write(GRSTCTL, grstctl::RXFFLSH);
        while self.read(GRSTCTL) & grstctl::RXFFLSH != 0 {
            core::hint::spin_loop();
        }
    }

    /// Words available in the total data FIFO.
    pub(crate) fn fifo_depth(&self) -> u16 {
        ((self.read(GHWCFG3) & ghwcfg3::DFIFO_DEPTH_MASK) >> ghwcfg3::DFIFO_DEPTH_SHIFT) as u16
    }
}

/// A FIFO push/pop window.
///
/// The FIFOs are 32 bits wide. A short packet still occupies a whole word; the
/// unused bytes are ignored on read and zero-filled on write.
#[derive(Clone, Copy)]
pub(crate) struct Fifo {
    ptr: *mut u32,
}

impl Fifo {
    /// Push `data`, padding the last word with zeros.
    pub(crate) fn push(&self, data: &[u8]) {
        let (words, rest) = data.as_chunks::<4>();
        for word in words {
            // SAFETY: the window is mapped and `word` is exactly four bytes.
            unsafe { self.ptr.write_volatile(u32::from_le_bytes(*word)) };
        }
        if !rest.is_empty() {
            let mut word = [0u8; 4];
            word[..rest.len()].copy_from_slice(rest);
            // SAFETY: see above.
            unsafe { self.ptr.write_volatile(u32::from_le_bytes(word)) };
        }
    }

    /// Pop `len` bytes into `buf`.
    ///
    /// `buf` must be at least `len` bytes long; every word is read even when
    /// only part of it is used.
    pub(crate) fn pop(&self, buf: &mut [u8], len: usize) {
        let (words, rest) = buf[..len].as_chunks_mut::<4>();
        for word in words {
            // SAFETY: the window is mapped.
            let value = unsafe { self.ptr.read_volatile() };
            word.copy_from_slice(&value.to_le_bytes());
        }
        if !rest.is_empty() {
            // SAFETY: see above.
            let value = unsafe { self.ptr.read_volatile() };
            rest.copy_from_slice(&value.to_le_bytes()[..rest.len()]);
        }
    }

    /// Pop and discard `len` bytes.
    pub(crate) fn discard(&self, len: usize) {
        for _ in 0..len.div_ceil(4) {
            // SAFETY: the window is mapped.
            unsafe { self.ptr.read_volatile() };
        }
    }
}
