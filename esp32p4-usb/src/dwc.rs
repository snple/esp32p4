//! DesignWare Core (DWC) USB 2.0 OTG registers.
//!
//! The DWC register map is not part of the chip's SVD (the IP vendor keeps the
//! detailed description under NDA), so it cannot be generated into
//! [`esp32p4_pac`]. The layout below is written by hand from the register
//! structures used by ESP-IDF (`soc/usb_dwc_struct.h`), limited to the registers
//! a slave-mode (CPU-driven FIFO) driver needs in device and host mode.
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
//!                    device mode                  host mode
//!   base + 0x1000    IN endpoint 0 TX FIFO        non-periodic TX FIFO
//!   base + 0x1000*n  IN endpoint n-1 TX FIFO      -
//!   base + 0x2000    IN endpoint 1 TX FIFO        periodic TX FIFO
//! ```
//!
//! Reads always pop the shared RX FIFO, whatever window they address. Every
//! access is a 32-bit word; a packet shorter than four bytes still occupies one
//! word, and the byte count comes from the RX status entry.
//!
//! # Host channels
//!
//! Host mode replaces endpoints with sixteen channels, each with the same
//! register block layout at `0x500 + 0x20 * n`. A transfer is programmed through
//! `HCCHARn`/`HCTSIZn` and started by setting `ChEna`; the core halts the
//! channel again when the transfer completes.

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

    /// `GNPTXFSIZ` in host mode: the non-periodic TX FIFO start and depth.
    pub const GNPTXFSIZ: usize = 0x028;
    /// `GNPTXSTS` in host mode.
    pub const GNPTXSTS: usize = 0x02c;
    /// `HPTXFSIZ` in host mode.
    pub const HPTXFSIZ: usize = 0x100;

    pub const HCFG: usize = 0x400;
    pub const HFIR: usize = 0x404;
    pub const HFNUM: usize = 0x408;
    /// `HPTXSTS` in host mode.
    pub const HPTXSTS: usize = 0x410;
    pub const HAINT: usize = 0x414;
    pub const HAINTMSK: usize = 0x418;
    pub const HPRT: usize = 0x440;

    /// Host channel 0 control register; channel `n` is at `HCCHAR0 + 0x20 * n`.
    pub const HCCHAR0: usize = 0x500;
    /// Host channel 0 interrupt register.
    pub const HCINT0: usize = 0x508;
    /// Host channel 0 interrupt mask register.
    pub const HCINTMSK0: usize = 0x50c;
    /// Host channel 0 transfer-size register.
    pub const HCTSIZ0: usize = 0x510;

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

    /// Address of `HCCHAR<n>`.
    #[inline]
    pub const fn hcchar(n: usize) -> usize {
        HCCHAR0 + 0x20 * n
    }

    /// Address of `HCINT<n>`.
    #[inline]
    pub const fn hcint(n: usize) -> usize {
        HCINT0 + 0x20 * n
    }

    /// Address of `HCINTMSK<n>`.
    #[inline]
    pub const fn hcintmsk(n: usize) -> usize {
        HCINTMSK0 + 0x20 * n
    }

    /// Address of `HCTSIZ<n>`.
    #[inline]
    pub const fn hctsiz(n: usize) -> usize {
        HCTSIZ0 + 0x20 * n
    }
}

/// `GOTGCTL` bits.
pub(crate) mod gotgctl {
    /// Software override of the VBUS-valid input.
    pub const VBVALIDOVEN: u32 = 1 << 2;
    /// Value driven while [`VBVALIDOVEN`] is set.
    pub const VBVALIDOVVAL: u32 = 1 << 3;
    /// Software override of the A-session-valid input.
    pub const AVALIDOVEN: u32 = 1 << 4;
    /// Value driven while [`AVALIDOVEN`] is set.
    pub const AVALIDOVVAL: u32 = 1 << 5;
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
    /// Start of frame.
    pub const SOF: u32 = 1 << 3;
    pub const RXFLVL: u32 = 1 << 4;
    /// Non-periodic TX FIFO empty.
    pub const NPTXFEMP: u32 = 1 << 5;
    pub const ERLYSUSP: u32 = 1 << 10;
    pub const USBSUSP: u32 = 1 << 11;
    pub const USBRST: u32 = 1 << 12;
    pub const ENUMDNE: u32 = 1 << 13;
    pub const IEPINT: u32 = 1 << 18;
    pub const OEPINT: u32 = 1 << 19;
    /// Host port interrupt, decoded through `HPRT`.
    pub const PRTINT: u32 = 1 << 24;
    /// One or more host channels have a pending interrupt (`HAINT`).
    pub const HCHINT: u32 = 1 << 25;
    /// Periodic TX FIFO empty.
    pub const PTXFEMP: u32 = 1 << 26;
    /// Device disconnected from the host port.
    pub const DISCONNINT: u32 = 1 << 29;
    pub const WKUPINT: u32 = 1 << 31;
}

/// `GRXSTSP` fields (device mode).
pub(crate) mod grxstsp {
    /// Endpoint number, `[3:0]`. In host mode this field is the channel number.
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

    /// OUT data packet received (device mode).
    pub const PKTSTS_OUT_DATA: u32 = 0b0010;
    /// IN data packet received (host mode); same encoding as
    /// [`PKTSTS_OUT_DATA`].
    pub const PKTSTS_IN_DATA: u32 = 0b0010;
    /// OUT transfer completed (device mode).
    pub const PKTSTS_OUT_DONE: u32 = 0b0011;
    /// SETUP transaction completed (device mode).
    pub const PKTSTS_SETUP_DONE: u32 = 0b0100;
    /// SETUP data packet received (device mode).
    pub const PKTSTS_SETUP_DATA: u32 = 0b0110;

    /// IN transfer completed (host mode).
    pub const PKTSTS_IN_DONE: u32 = 0b0011;
    /// Data toggle error (host mode).
    pub const PKTSTS_DTGLERR: u32 = 0b0101;
    /// Channel halted (host mode).
    pub const PKTSTS_CHHLTD: u32 = 0b0111;
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
    /// Number of host channels minus one, `[30:27]`.
    pub const NUMHSTCHNL_SHIFT: u32 = 27;
    pub const NUMHSTCHNL_MASK: u32 = 0xf << NUMHSTCHNL_SHIFT;
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

// ── Host mode ────────────────────────────────────────────────────────────────

/// `HCFG` bits.
pub(crate) mod hcfg {
    /// PHY clock select, `[1:0]`: `0b01` = 48 MHz, `0b10` = 6 MHz (low speed
    /// behind the PHY's implicit 8× divider).
    pub const FSLSPCLKSEL_MASK: u32 = 0x3;
    pub const FSLSPCLKSEL_48M: u32 = 0b01;
    pub const FSLSPCLKSEL_6M: u32 = 0b10;
    /// The core is limited to full and low speed.
    pub const FSLSSUPP: u32 = 1 << 2;
    /// Scatter/gather DMA instead of buffer DMA. Left clear: slave mode.
    pub const DESCDMA: u32 = 1 << 23;
    /// Periodic scheduler enable. Left clear: periodic requests are queued as
    /// soon as software starts the channel.
    pub const PERSCHEDENA: u32 = 1 << 26;
}

/// `HFIR` fields.
pub(crate) mod hfir {
    /// Frame interval in PHY clocks (48 000 = 1 ms at 48 MHz).
    pub const FRINT_MASK: u32 = 0xffff;
    pub const FRINT_48M: u32 = 48_000;
    pub const FRINT_6M: u32 = 6_000;
}

/// `HFNUM` fields.
pub(crate) mod hfnum {
    /// Current frame number, `[15:0]`.
    pub const FRNUM_MASK: u32 = 0xffff;
    /// Time remaining in the current frame, `[31:16]`.
    pub const FRREM_SHIFT: u32 = 16;
    pub const FRREM_MASK: u32 = 0xffff << FRREM_SHIFT;
}

/// `HPRT` bits.
pub(crate) mod hprt {
    /// A device is attached.
    pub const PRTCONNSTS: u32 = 1 << 0;
    /// Connect status changed (write one to clear).
    pub const PRTCONNDET: u32 = 1 << 1;
    /// Port enabled (write one to disable).
    pub const PRTENA: u32 = 1 << 2;
    /// Port enabled or disabled (write one to clear).
    pub const PRTENCHNG: u32 = 1 << 3;
    /// Overcurrent active (read only).
    pub const PRTOVRCURRACT: u32 = 1 << 4;
    /// Overcurrent changed (write one to clear).
    pub const PRTOVRCURRCHNG: u32 = 1 << 5;
    /// Port resume signalling.
    pub const PRTRES: u32 = 1 << 6;
    /// Port suspended.
    pub const PRTSUSP: u32 = 1 << 7;
    /// Port reset.
    pub const PRTRST: u32 = 1 << 8;
    /// Line state, `[11:10]`.
    pub const PRTLNSTS_SHIFT: u32 = 10;
    pub const PRTLNSTS_MASK: u32 = 0x3 << PRTLNSTS_SHIFT;
    /// Port power.
    pub const PRTPWR: u32 = 1 << 12;
    /// Test control, `[16:13]`.
    pub const PRTTSTCTL_SHIFT: u32 = 13;
    /// Port speed, `[18:17]`: 0 = high, 1 = full, 2 = low.
    pub const PRTSPD_SHIFT: u32 = 17;
    pub const PRTSPD_MASK: u32 = 0x3 << PRTSPD_SHIFT;
    /// Bits that are cleared by writing one; a read-modify-write must clear
    /// them so the write does not acknowledge events it did not observe.
    pub const W1C_MASK: u32 = PRTCONNDET | PRTENA | PRTENCHNG | PRTOVRCURRCHNG;
}

/// `HAINT` / `HAINTMSK` fields.
pub(crate) mod haint {
    pub const CHANNEL_MASK: u32 = 0xffff;
}

/// `HCCHAR` fields.
pub(crate) mod hcchar {
    /// Maximum packet size, `[10:0]`.
    pub const MPS_MASK: u32 = 0x7ff;
    /// Endpoint number, `[14:11]`.
    pub const EPNUM_SHIFT: u32 = 11;
    pub const EPNUM_MASK: u32 = 0xf << EPNUM_SHIFT;
    /// Direction: 1 = IN.
    pub const EPDIR: u32 = 1 << 15;
    /// The device is low speed behind a full-speed hub. Zero for a device on
    /// the root port, which the core already knows about.
    pub const LSPDDEV: u32 = 1 << 17;
    /// Endpoint type, `[19:18]`.
    pub const EPTYPE_SHIFT: u32 = 18;
    pub const EPTYPE_MASK: u32 = 0x3 << EPTYPE_SHIFT;
    pub const EPTYPE_CONTROL: u32 = 0b00 << EPTYPE_SHIFT;
    pub const EPTYPE_ISO: u32 = 0b01 << EPTYPE_SHIFT;
    pub const EPTYPE_BULK: u32 = 0b10 << EPTYPE_SHIFT;
    pub const EPTYPE_INTR: u32 = 0b11 << EPTYPE_SHIFT;
    /// Multi-count / error-count, `[21:20]`. Must not stay at the reserved
    /// value 0.
    pub const EC_SHIFT: u32 = 20;
    pub const EC_MASK: u32 = 0x3 << EC_SHIFT;
    /// Device address, `[28:22]`.
    pub const DEVADDR_SHIFT: u32 = 22;
    pub const DEVADDR_MASK: u32 = 0x7f << DEVADDR_SHIFT;
    /// Send an OUT token in an odd frame (isochronous only).
    pub const ODDFRM: u32 = 1 << 29;
    /// Disable the channel (write one, together with `CHENA`, to halt it).
    pub const CHDIS: u32 = 1 << 30;
    /// Channel enable.
    pub const CHENA: u32 = 1 << 31;
}

/// `HCTSIZ` fields.
pub(crate) mod hctsiz {
    /// Transfer size in bytes, `[18:0]`.
    pub const XFERSIZE_MASK: u32 = 0x7_ffff;
    /// Packet count, `[28:19]`.
    pub const PKTCNT_SHIFT: u32 = 19;
    pub const PKTCNT_MASK: u32 = 0x3ff << PKTCNT_SHIFT;
    /// Data PID, `[30:29]`.
    pub const PID_SHIFT: u32 = 29;
    pub const PID_MASK: u32 = 0x3 << PID_SHIFT;
    pub const PID_DATA0: u32 = 0b00 << PID_SHIFT;
    pub const PID_DATA1: u32 = 0b10 << PID_SHIFT;
    pub const PID_SETUP: u32 = 0b11 << PID_SHIFT;
    /// Do ping (high-speed OUT only).
    pub const DOPNG: u32 = 1 << 31;
}

/// `HCINT` bits.
pub(crate) mod hcint {
    /// Transfer completed.
    pub const XFERCOMPL: u32 = 1 << 0;
    /// Channel halted.
    pub const CHHLTD: u32 = 1 << 1;
    /// AHB error.
    pub const AHBERR: u32 = 1 << 2;
    /// STALL handshake received.
    pub const STALL: u32 = 1 << 3;
    /// NAK handshake received.
    pub const NACK: u32 = 1 << 4;
    /// ACK handshake received.
    pub const ACK: u32 = 1 << 5;
    /// NYET handshake received.
    pub const NYET: u32 = 1 << 6;
    /// Transaction error: CRC, timeout, bit stuff or false EOP.
    pub const XACTERR: u32 = 1 << 7;
    /// Babble error.
    pub const BBLERR: u32 = 1 << 8;
    /// Frame overrun.
    pub const FRMOVRUN: u32 = 1 << 9;
    /// Data toggle error.
    pub const DATATGLERR: u32 = 1 << 10;
    /// Buffer not available (DMA modes only).
    pub const BNAINTR: u32 = 1 << 11;
    /// Excessive (three consecutive) transaction errors.
    pub const XCS_XACT_ERR: u32 = 1 << 12;

    /// Everything that ends a transaction one way or another.
    pub const COMPLETION_MASK: u32 = XFERCOMPL | CHHLTD;

    /// Errors that mean the packet did not transfer and may succeed when it is
    /// sent again.
    pub const RETRY_MASK: u32 = XACTERR | DATATGLERR | FRMOVRUN;
}

/// `GNPTXFSIZ` / `HPTXFSIZ` fields.
pub(crate) mod hptxfsiz {
    /// Start address in words, `[15:0]`.
    pub const START_MASK: u32 = 0xffff;
    /// Depth in words, `[31:16]`.
    pub const DEPTH_SHIFT: u32 = 16;
    pub const DEPTH_MASK: u32 = 0xffff << DEPTH_SHIFT;
}

/// `GNPTXSTS` / `HPTXSTS` fields.
pub(crate) mod htxsts {
    /// Free words in the TX FIFO, `[15:0]`.
    pub const SPCAVAIL_MASK: u32 = 0xffff;
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

    /// Number of host channels the core implements.
    pub(crate) fn channel_count(&self) -> usize {
        let raw = self.read(GHWCFG2);
        (((raw & ghwcfg2::NUMHSTCHNL_MASK) >> ghwcfg2::NUMHSTCHNL_SHIFT) + 1) as usize
    }

    /// The non-periodic TX FIFO push window (host mode).
    ///
    /// Reads through this window still pop the shared RX FIFO: the two TX FIFOs
    /// live in the same 4 KB push regions the device mode IN endpoints use.
    #[inline]
    pub(crate) fn np_tx_fifo(&self) -> Fifo {
        Fifo {
            ptr: self.ptr(FIFO_WINDOW),
        }
    }

    /// The periodic TX FIFO push window (host mode).
    #[inline]
    pub(crate) fn p_tx_fifo(&self) -> Fifo {
        Fifo {
            ptr: self.ptr(FIFO_WINDOW * 2),
        }
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
