//! Device-mode driver for the ESP32-P4's two DWC OTG cores.
//!
//! ```text
//!                     ┌──────────────────────────────┐
//!   USB_OTG11 (FS) ───┤ D+/D- on GPIO27/26 (PHY 2)   │  full speed, 12 Mbit/s
//!   USB_OTG20 (HS) ───┤ internal UTMI+ PHY           │  high speed, 480 Mbit/s
//!                     └──────────────────────────────┘
//! ```
//!
//! Both cores are Synopsys DesignWare OTG controllers driven in **slave mode**:
//! the CPU pushes and pops every packet through the FIFOs, there is no DMA and
//! no interrupt — [`UsbBus::poll`] collects the events.
//!
//! # Full-speed port
//!
//! The full-speed core has no pads of its own: it is wired to one of the chip's
//! two full-speed PHYs through the GPIO matrix, and it needs five OTG status
//! inputs (`IDDIG`, `AVALID`, `BVALID`, `VBUSVALID`, `SESSEND`). [`UsbFs::new`]
//! forces those inputs to the values of a self-powered peripheral on a powered
//! bus, so the core sees a valid B-session without extra wiring.
//!
//! By default the OTG FS core uses PHY 2 (GPIO26/27) while the USB Serial/JTAG
//! controller uses PHY 1 (GPIO24/25). If an application uses both ports it can
//! swap them with `esp32p4_hal::usb_serial_jtag::UsbSerialJtag::select_phy`; this
//! driver follows whichever PHY the swap logic assigns, it does not touch it.
//!
//! # High-speed port
//!
//! The high-speed core drives an internal UTMI+ PHY, so no GPIOs are involved.
//! Device mode needs the PHY's 15 kΩ pull-downs off; on chip revisions 3.0 and
//! later those resistors are controlled by software through `LP_SYS` (see
//! [`HsResources`]), while earlier revisions leave them to the OTG core, which
//! ignores the write.
//!
//! # Limitations
//!
//! Transfers are transaction-level: one packet per [`UsbBus::write`] or
//! [`UsbBus::read`], with a fresh transfer descriptor for each packet. That is
//! what bulk, interrupt and control endpoints need. Isochronous endpoints are
//! rejected at allocation time, because they additionally require per-frame
//! toggle handling and dropping of unsent packets at the end of a frame.
//!
//! # FIFOs
//!
//! The shared data FIFO is 200 words (FS) or 896 words (HS). OUT packets need
//! staging space in the RX FIFO, each IN endpoint needs a dedicated TX FIFO of
//! at least 16 words. [`UsbBus::alloc_ep`] reserves the space as endpoints are
//! allocated and fails with [`UsbError::EndpointMemoryOverflow`] when the request
//! does not fit.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_usb::class::cdc_acm::CdcAcm;
//! use esp32p4_usb::p4::{FsResources, UsbFs};
//! use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};
//!
//! let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
//!     clkrst: &dp.HP_SYS_CLKRST,
//!     lp_aon_clkrst: &dp.LP_AON_CLKRST,
//!     usb_wrap: &dp.USB_WRAP,
//!     gpio: &dp.GPIO,
//!     io_mux: &dp.IO_MUX,
//! }));
//! let mut serial = CdcAcm::new(&allocator, 64);
//! let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
//!     .product("ESP32-P4 serial")
//!     .build();
//! ```

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use esp32p4_pac::usb_wrap::RegisterBlock as UsbWrap;
use esp32p4_pac::{GPIO, HP_SYS, HP_SYS_CLKRST, IO_MUX, LP_AON_CLKRST, LP_SYS};

use crate::bus::{PollResult, UsbBus};
use crate::dwc::{
    Dwc, dcfg, dctl::SFTRST_DISCON, dieptsiz0, doeptsiz0, dsts, ep0ctl, epctl, epint, eptsiz,
    fifo_siz, gintsts, gotgctl, grxstsp, gusbcfg, reg,
};
use crate::endpoint::{Direction, EndpointAddress, EndpointType};
use crate::error::{UsbError, UsbResult};

/// Register base of the full-speed OTG core (`USB_OTG11`).
pub const USB_OTG_FS_BASE: usize = 0x5004_0000;
/// Register base of the high-speed OTG core (`USB_OTG20`).
pub const USB_OTG_HS_BASE: usize = 0x5000_0000;

/// Highest endpoint number the driver manages.
///
/// The full-speed core implements EP0 plus six configurable endpoints; eight
/// numbers cover both controllers with some headroom without spending much RAM
/// on the OUT staging buffers.
pub const MAX_SUPPORTED_ENDPOINTS: usize = 8;

/// Largest packet an endpoint can be configured for (high-speed bulk).
pub const MAX_PACKET_SIZE: usize = 512;

/// Total staging space for received packets, in bytes.
///
/// One packet per OUT endpoint is staged while the stack is between
/// [`UsbBus::poll`] and [`UsbBus::read`]. Full speed needs 64 bytes per
/// endpoint, high speed up to 512; a common pool keeps the bus small while
/// still allowing, for example, two 512-byte high-speed endpoints or every
/// full-speed endpoint at once.
pub const OUT_STAGING_BYTES: usize = 1024;

/// Fallback packet size for endpoint 0 when the description asks for something
/// other than the 8/16/32/64 bytes a control endpoint supports.
const EP0_MAX_PACKET: u16 = 64;

/// `out_len` marker for "no packet staged".
const EMPTY: u16 = u16::MAX;

/// Extra RX FIFO words both controllers need on top of the OUT packet space.
const FS_EXTRA_RX_WORDS: u16 = 30;
const HS_EXTRA_RX_WORDS: u16 = 47;

/// The USB PHY block, which is not part of the SVD.
const USBPHY_BASE: usize = 0x5009_c000;
/// `USBPHY.FC_06`: line-state helpers used by the UTMI+ PHY.
const USBPHY_FC_06: usize = 0x18;
/// `LP_SYS.HP_USB_OTGHS_PHY_CTRL`, present from chip revision 3.0 on.
const LP_SYS_HS_PHY_CTRL: usize = 0x204;
const HS_PHY_CTRL_DP_PULLDOWN: u32 = 1 << 1;
const HS_PHY_CTRL_DM_PULLDOWN: u32 = 1 << 2;

/// Drive a GPIO-matrix peripheral input with a constant level.
///
/// Every `GPIO_FUNCn_IN_SEL_CFG` register has its own type in the PAC, so the
/// operation is a macro instead of a generic function.
macro_rules! route_otg_input {
    ($register:expr, $high:expr) => {{
        let register = &$register;
        // Route through the GPIO matrix and pick a constant instead of a pad.
        register.set_sel();
        register.clr_in_inv_sel();
        register.write_in_sel(if $high { 0x3f } else { 0x3e });
    }};
}

/// Switch a pad to its analog PHY function with maximum drive strength.
///
/// IO MUX function 0 is the analog function on every pad; for GPIO26/27 that is
/// `USB1P1_N1` / `USB1P1_P1`. The PAC gives each pad its own type, hence the
/// macro.
macro_rules! configure_usb_pad {
    ($pad:expr) => {{
        let pad = &$pad;
        pad.write_mcu_sel(0);
        pad.write_fun_drv(3);
    }};
}

/// Per-endpoint configuration captured at allocation time.
#[derive(Clone, Copy)]
struct EndpointConfig {
    allocated: bool,
    ep_type: EndpointType,
    max_packet: u16,
    /// TX FIFO size in words (IN endpoints only).
    fifo_words: u16,
    /// Byte offset of this endpoint's staging area (OUT endpoints only).
    staging: u16,
}

impl EndpointConfig {
    const fn free() -> Self {
        Self {
            allocated: false,
            ep_type: EndpointType::Bulk,
            max_packet: 0,
            fifo_words: 0,
            staging: 0,
        }
    }
}

/// Events collected from the hardware and not yet reported to the stack.
#[derive(Clone, Copy, Default)]
struct Pending {
    ep_out: u16,
    ep_in_complete: u16,
    ep_setup: u16,
    reset: bool,
    suspend: bool,
    resume: bool,
}

/// Driver state behind the shared bus reference.
struct State {
    in_eps: [EndpointConfig; MAX_SUPPORTED_ENDPOINTS],
    out_eps: [EndpointConfig; MAX_SUPPORTED_ENDPOINTS],
    /// One staged OUT packet per endpoint, packed back to back.
    out_pool: [u8; OUT_STAGING_BYTES],
    /// Staged OUT bytes per endpoint, or [`EMPTY`].
    out_len: [u16; MAX_SUPPORTED_ENDPOINTS],
    pending: Pending,
}

impl State {
    const fn new() -> Self {
        Self {
            in_eps: [EndpointConfig::free(); MAX_SUPPORTED_ENDPOINTS],
            out_eps: [EndpointConfig::free(); MAX_SUPPORTED_ENDPOINTS],
            out_pool: [0; OUT_STAGING_BYTES],
            out_len: [EMPTY; MAX_SUPPORTED_ENDPOINTS],
            pending: Pending {
                ep_out: 0,
                ep_in_complete: 0,
                ep_setup: 0,
                reset: false,
                suspend: false,
                resume: false,
            },
        }
    }
}

/// Which OTG core a [`Core`] drives.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Speed {
    Full,
    High,
}

impl Speed {
    /// `DCFG.DevSpd`: `0b11` selects the 48 MHz full-speed PHY, `0b00` the
    /// UTMI+ high-speed PHY.
    fn devspd(self) -> u32 {
        match self {
            Self::Full => 0b11,
            Self::High => 0b00,
        }
    }

    /// Extra RX FIFO words (status entries plus slack).
    fn extra_rx_words(self) -> u16 {
        match self {
            Self::Full => FS_EXTRA_RX_WORDS,
            Self::High => HS_EXTRA_RX_WORDS,
        }
    }

    fn base(self) -> usize {
        match self {
            Self::Full => USB_OTG_FS_BASE,
            Self::High => USB_OTG_HS_BASE,
        }
    }
}

/// Shared device-mode implementation for both OTG cores.
struct Core {
    dwc: Dwc,
    speed: Speed,
    fifo_depth: u16,
    /// Words reserved for IN endpoint TX FIFOs.
    tx_fifo_words: u16,
    /// Words reserved for OUT packets in the RX FIFO.
    rx_fifo_words: u16,
    /// Bytes of the OUT staging pool handed out so far.
    staging_used: u16,
    state: RefCell<State>,
}

impl Core {
    fn new(speed: Speed) -> Self {
        let dwc = Dwc::new(speed.base());
        let fifo_depth = dwc.fifo_depth();
        Self {
            dwc,
            speed,
            fifo_depth,
            tx_fifo_words: 0,
            rx_fifo_words: 0,
            staging_used: 0,
            state: RefCell::new(State::new()),
        }
    }

    // ── Configuration ───────────────────────────────────────────────────────

    fn alloc_ep(
        &mut self,
        direction: Direction,
        address: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        let _ = interval;
        if max_packet == 0 || max_packet as usize > MAX_PACKET_SIZE {
            return Err(UsbError::BufferOverflow);
        }
        if matches!(ep_type, EndpointType::Isochronous { .. }) {
            // Slave-mode isochronous transfers need per-frame toggle handling
            // and packet dropping, which this driver does not implement.
            return Err(UsbError::Unsupported);
        }

        let state = self.state.get_mut();
        let configs = match direction {
            Direction::In => &mut state.in_eps,
            Direction::Out => &mut state.out_eps,
        };

        let number = match address {
            Some(address) => {
                if address.direction() != direction {
                    return Err(UsbError::InvalidEndpoint);
                }
                let number = address.number() as usize;
                if number >= MAX_SUPPORTED_ENDPOINTS {
                    return Err(UsbError::InvalidEndpoint);
                }
                if configs[number].allocated {
                    return Err(UsbError::EndpointOverflow);
                }
                number
            }
            None => {
                // Endpoint 0 is reserved for control transfers.
                match (1..MAX_SUPPORTED_ENDPOINTS).find(|&n| !configs[n].allocated) {
                    Some(number) => number,
                    None => return Err(UsbError::EndpointOverflow),
                }
            }
        };

        let (max_packet, fifo_words) = if number == 0 {
            // Control endpoints accept 8, 16, 32 or 64 bytes per packet; the
            // descriptor and the register have to agree.
            let max_packet = match max_packet {
                8 | 16 | 32 | 64 => max_packet,
                _ => EP0_MAX_PACKET,
            };
            (max_packet, max_packet.div_ceil(4).max(16))
        } else {
            // `DIEPTXFD` requires at least 16 words.
            (max_packet, max_packet.div_ceil(4).max(16))
        };

        let words = match direction {
            Direction::In => fifo_words,
            Direction::Out => max_packet.div_ceil(4),
        };
        let used = self
            .tx_fifo_words
            .saturating_add(words)
            .saturating_add(self.rx_fifo_words)
            .saturating_add(self.speed.extra_rx_words());
        if used > self.fifo_depth {
            return Err(UsbError::EndpointMemoryOverflow);
        }
        let staging = match direction {
            Direction::In => {
                self.tx_fifo_words += words;
                0
            }
            Direction::Out => {
                let staging = self.staging_used;
                if staging as usize + max_packet as usize > OUT_STAGING_BYTES {
                    return Err(UsbError::EndpointMemoryOverflow);
                }
                self.rx_fifo_words += words;
                self.staging_used += max_packet;
                staging
            }
        };

        configs[number] = EndpointConfig {
            allocated: true,
            ep_type,
            max_packet,
            fifo_words,
            staging,
        };
        debug!(
            "USB: allocated EP{} {} type {:?} mps {}",
            number,
            if direction == Direction::In {
                "IN"
            } else {
                "OUT"
            },
            ep_type,
            max_packet
        );
        Ok(EndpointAddress::new(number as u8, direction))
    }

    fn enable(&mut self) {
        let dwc = self.dwc;

        // Keep the device off the bus until everything is configured.
        dwc.set(reg::DCTL, SFTRST_DISCON);

        dwc.soft_reset();
        // The soft reset clears most of the core configuration, disconnect again
        // so nothing reaches the host before the FIFOs and endpoints are ready.
        dwc.set(reg::DCTL, SFTRST_DISCON);

        // Device mode with the right PHY.
        let mut usbcfg = gusbcfg::FDMOD;
        match self.speed {
            // The full-speed core uses its embedded full-speed PHY.
            Speed::Full => usbcfg |= gusbcfg::PHYSEL,
            // The high-speed core has a UTMI+ PHY. Keep 8-bit data width.
            Speed::High => usbcfg |= gusbcfg::PHYIF,
        }
        // USB turnaround time. Only host mode uses it, but esp-hal programs 5
        // for both P4 cores, so keep the same value.
        usbcfg |= 5 << gusbcfg::USBTRDTIM_SHIFT;
        dwc.write(reg::GUSBCFG, usbcfg);
        while dwc.read(reg::GINTSTS) & gintsts::CURMOD != 0 {
            core::hint::spin_loop();
        }

        // Slave mode: no DMA, no global interrupt, single-beat AHB bursts.
        dwc.write(reg::GAHBCFG, 0);

        // Device speed and address 0.
        dwc.write(reg::DCFG, self.speed.devspd() << dcfg::DEVSPD_SHIFT);

        // Force a valid B-session. The board does not feed VBUS into the PHY,
        // and a bus-powered device cannot be running without the cable anyway,
        // so there is nothing to gain from monitoring the session. This is also
        // what the proven ESP-IDF/esp-hal configuration does.
        let control = dwc.read(reg::GOTGCTL);
        dwc.write(
            reg::GOTGCTL,
            (control & !gotgctl::BVALIDOVVAL) | gotgctl::BVALIDOVEN | gotgctl::BVALIDOVVAL,
        );
        debug!(
            "USB: GOTGCTL {:08x}, connector ID {}",
            dwc.read(reg::GOTGCTL),
            if control & gotgctl::CONIDSTS != 0 {
                "B"
            } else {
                "A"
            }
        );

        // Clear the PHY suspend state the bootloader may have left behind.
        dwc.write(reg::PCGCCTL, 0);

        // Interrupt masks: the driver polls, but keep the bits that describe a
        // completed transfer enabled so the latches behave predictably.
        dwc.write(reg::DIEPMSK, epint::XFERCOMPL);
        dwc.write(reg::DOEPMSK, epint::XFERCOMPL | epint::SETUP);

        {
            let state = self.state.borrow();
            self.configure_fifos(&state);
            dwc.flush_tx_fifos();
            dwc.flush_rx_fifo();
            self.configure_endpoints(&state);
        }

        // Connect: this releases the D+ pull-up.
        dwc.clear(reg::DCTL, SFTRST_DISCON);
        debug!(
            "USB: {} core up, {} FIFO words",
            match self.speed {
                Speed::Full => "full-speed",
                Speed::High => "high-speed",
            },
            self.fifo_depth
        );
    }

    /// Program the RX/TX FIFO sizes.
    fn configure_fifos(&self, state: &State) {
        let dwc = self.dwc;
        let rx_words = self.rx_fifo_words + self.speed.extra_rx_words();
        dwc.write(reg::GRXFSIZ, rx_words as u32);

        let mut top = rx_words as u32;
        for (number, config) in state.in_eps.iter().enumerate() {
            if !config.allocated {
                continue;
            }
            let size = config.fifo_words as u32;
            let address = if number == 0 {
                reg::DIEPTXF0
            } else {
                reg::dieptxf(number)
            };
            dwc.write(address, top | (size << fifo_siz::DEPTH_SHIFT));
            top += size;
        }
    }

    /// Program every allocated endpoint and arm OUT endpoint 0 for SETUP.
    fn configure_endpoints(&self, state: &State) {
        let dwc = self.dwc;

        for (number, config) in state.in_eps.iter().enumerate() {
            if !config.allocated {
                continue;
            }
            let mut control = if number == 0 {
                ep0_mps_code(config.max_packet) | ep0ctl::SNAK
            } else {
                (config.max_packet as u32 & epctl::MPS_MASK)
                    | ep_type_bits(config.ep_type)
                    | ((number as u32) << epctl::TXFNUM_SHIFT)
                    // Start every endpoint at DATA0 and NAK until data is queued.
                    | epctl::SETD0PID
                    | epctl::SNAK
            };
            control |= epctl::USBACTEP;
            dwc.write(reg::diepctl(number), control);
            dwc.write(reg::diepint(number), u32::MAX);
        }

        for (number, config) in state.out_eps.iter().enumerate() {
            if !config.allocated {
                continue;
            }
            let mut control = if number == 0 {
                ep0_mps_code(config.max_packet)
            } else {
                (config.max_packet as u32 & epctl::MPS_MASK)
                    | ep_type_bits(config.ep_type)
                    | epctl::SETD0PID
            };
            control |= epctl::USBACTEP;
            dwc.write(reg::doepctl(number), control);
            dwc.write(reg::doepint(number), u32::MAX);
        }

        // Arm the OUT endpoints: EP0 for SETUP packets and the data endpoints
        // so that the host can send a first packet before the application gets
        // around to calling `read`.
        for number in 0..MAX_SUPPORTED_ENDPOINTS {
            if state.out_eps[number].allocated {
                self.arm_out(state, number);
            }
        }
    }

    /// Arm an OUT endpoint for one packet (or, for EP0, for SETUP packets).
    fn arm_out(&self, state: &State, number: usize) {
        let config = state.out_eps[number];
        if !config.allocated {
            return;
        }
        let dwc = self.dwc;
        let mps = config.max_packet as u32;
        if number == 0 {
            // `SUPCNT` lets the core swallow up to three back-to-back SETUPs
            // without software re-arming between them.
            dwc.write(
                reg::DOEPTSIZ0,
                (mps & doeptsiz0::XFERSIZE_MASK)
                    | doeptsiz0::PKTCNT
                    | (3 << doeptsiz0::SUPCNT_SHIFT),
            );
            dwc.set(reg::DOEPCTL0, ep0ctl::CNAK | ep0ctl::EPENA);
        } else {
            dwc.write(
                reg::doeptsiz(number),
                (mps & eptsiz::XFERSIZE_MASK) | (1 << eptsiz::PKTCNT_SHIFT),
            );
            dwc.set(reg::doepctl(number), epctl::CNAK | epctl::EPENA);
        }
    }

    // ── Bus events ──────────────────────────────────────────────────────────

    fn poll(&self) -> PollResult {
        let dwc = self.dwc;
        let mut state = self.state.borrow_mut();

        let flags = dwc.read(reg::GINTSTS);

        if flags & gintsts::USBRST != 0 {
            // Write-one-to-clear; the mask covers only the bits we handle.
            dwc.write(
                reg::GINTSTS,
                gintsts::USBRST | gintsts::ENUMDNE | gintsts::USBSUSP | gintsts::WKUPINT,
            );
            self.handle_bus_reset(&mut state);
            state.pending = Pending {
                reset: true,
                ..Pending::default()
            };
        } else {
            let clears = flags & (gintsts::ENUMDNE | gintsts::USBSUSP | gintsts::WKUPINT);
            if clears != 0 {
                dwc.write(reg::GINTSTS, clears);
            }
            if flags & gintsts::ENUMDNE != 0 {
                let speed = (dwc.read(reg::DSTS) & dsts::ENUMSPD_MASK) >> dsts::ENUMSPD_SHIFT;
                debug!("USB: enumerated, speed code {}", speed);
                self.apply_enumerated_speed(speed);
            }
            if flags & gintsts::USBSUSP != 0 {
                state.pending.suspend = true;
            }
            if flags & gintsts::WKUPINT != 0 {
                state.pending.resume = true;
            }
        }

        self.drain_rx(&mut state);
        self.collect_in_completions(&mut state);

        Self::take_pending(&mut state)
    }

    /// Match `DCFG.DevSpd` to the speed the host negotiated.
    ///
    /// A high-speed core attached to a full-speed host port enumerates as a
    /// full-speed device; the core reports that in `DSTS` and expects the speed
    /// field to follow. The full-speed core only ever reports its own speed.
    fn apply_enumerated_speed(&self, speed_code: u32) {
        if self.speed == Speed::Full {
            return;
        }
        let devspd = match speed_code {
            0 => 0b00, // high speed
            2 => 0b10, // low speed
            _ => 0b01, // full speed through the UTMI+ PHY
        };
        self.dwc
            .modify(reg::DCFG, dcfg::DEVSPD_MASK, devspd << dcfg::DEVSPD_SHIFT);
    }

    /// Move everything the core received into the endpoint staging buffers.
    fn drain_rx(&self, state: &mut State) {
        let dwc = self.dwc;
        while dwc.rx_level() {
            let status = dwc.rx_status();
            let number = (status & grxstsp::EPNUM_MASK) as usize;
            let len = ((status & grxstsp::BCNT_MASK) >> grxstsp::BCNT_SHIFT) as usize;
            let packet = (status & grxstsp::PKTSTS_MASK) >> grxstsp::PKTSTS_SHIFT;

            if number >= MAX_SUPPORTED_ENDPOINTS {
                dwc.rx_fifo().discard(len);
                continue;
            }

            match packet {
                grxstsp::PKTSTS_SETUP_DATA if number == 0 && state.out_eps[0].allocated => {
                    // A SETUP aborts whatever control transfer was running.
                    self.abort_ep0_in();
                    if state.out_len[0] != EMPTY {
                        warn!("USB: EP0 SETUP replaced an unread packet");
                    }
                    let start = state.out_eps[0].staging as usize;
                    if len <= state.out_eps[0].max_packet as usize {
                        dwc.rx_fifo()
                            .pop(&mut state.out_pool[start..start + len], len);
                        state.out_len[0] = len as u16;
                        state.pending.ep_setup |= 1;
                    } else {
                        dwc.rx_fifo().discard(len);
                        warn!("USB: oversized SETUP packet dropped");
                    }
                    self.arm_out(state, 0);
                }
                grxstsp::PKTSTS_OUT_DATA if state.out_eps[number].allocated => {
                    let start = state.out_eps[number].staging as usize;
                    if state.out_len[number] != EMPTY
                        || len > state.out_eps[number].max_packet as usize
                    {
                        // The previous packet has not been handed to the stack
                        // yet; the core should not have sent another one, but
                        // dropping it keeps the FIFO moving.
                        warn!("USB: EP{} OUT overrun, packet dropped", number);
                        dwc.rx_fifo().discard(len);
                        continue;
                    }
                    dwc.rx_fifo()
                        .pop(&mut state.out_pool[start..start + len], len);
                    state.out_len[number] = len as u16;
                    state.pending.ep_out |= 1 << number;
                }
                _ => {
                    // OUT transfer complete / SETUP transaction complete: the
                    // per-packet status above already carried the payload.
                }
            }
        }
    }

    /// Latch IN-endpoint completions and clear the interrupt flags.
    fn collect_in_completions(&self, state: &mut State) {
        let dwc = self.dwc;
        for number in 0..MAX_SUPPORTED_ENDPOINTS {
            let flags = dwc.read(reg::diepint(number));
            if flags == 0 {
                continue;
            }
            // Write-one-to-clear.
            dwc.write(reg::diepint(number), flags);
            if flags & epint::XFERCOMPL != 0 {
                state.pending.ep_in_complete |= 1 << number;
            }
        }
        for number in 0..MAX_SUPPORTED_ENDPOINTS {
            let flags = dwc.read(reg::doepint(number));
            if flags == 0 {
                continue;
            }
            dwc.write(reg::doepint(number), flags);
        }
    }

    /// Hand one batch of events to the caller.
    fn take_pending(state: &mut State) -> PollResult {
        let pending = &mut state.pending;

        if pending.reset {
            *pending = Pending::default();
            return PollResult::Reset;
        }
        if pending.suspend {
            pending.suspend = false;
            return PollResult::Suspend;
        }
        if pending.resume {
            pending.resume = false;
            return PollResult::Resume;
        }

        let mut out = pending.ep_out;
        let mut in_complete = pending.ep_in_complete;
        let setup = pending.ep_setup;
        pending.ep_out = 0;
        pending.ep_in_complete = 0;
        pending.ep_setup = 0;

        if out == 0 && in_complete == 0 && setup == 0 {
            return PollResult::None;
        }

        // Endpoint 0 produces at most one event per report; the device stack
        // handles SETUP before OUT before IN, so keep the rest queued.
        if setup & 1 != 0 {
            pending.ep_out |= out & 1;
            pending.ep_in_complete |= in_complete & 1;
            out &= !1;
            in_complete &= !1;
        } else if out & 1 != 0 {
            pending.ep_in_complete |= in_complete & 1;
            in_complete &= !1;
        }

        PollResult::Data {
            ep_out: out,
            ep_in_complete: in_complete,
            ep_setup: setup,
        }
    }

    /// Reinitialise the core and the endpoints after a bus reset.
    fn handle_bus_reset(&self, state: &mut State) {
        let dwc = self.dwc;

        // The core resets the device address itself; make it explicit anyway.
        dwc.modify(reg::DCFG, dcfg::DEVADDR_MASK, 0);

        dwc.flush_tx_fifos();
        dwc.flush_rx_fifo();

        for len in state.out_len.iter_mut() {
            *len = EMPTY;
        }
        for number in 0..MAX_SUPPORTED_ENDPOINTS {
            dwc.write(reg::diepint(number), u32::MAX);
            dwc.write(reg::doepint(number), u32::MAX);
        }

        // The data toggle, NAK and packet-size settings do not survive a reset.
        self.configure_endpoints(state);
        state.pending = Pending::default();
        debug!("USB: reset");
    }

    /// Disable and flush EP0's IN direction after an unexpected SETUP.
    fn abort_ep0_in(&self) {
        let dwc = self.dwc;
        let control = dwc.read(reg::DIEPCTL0);
        if control & ep0ctl::EPENA != 0 {
            dwc.set(reg::DIEPCTL0, ep0ctl::SNAK | ep0ctl::EPDIS);
            let mut spins = 100_000u32;
            while dwc.read(reg::diepint(0)) & epint::EPDISBLD == 0 && spins > 0 {
                spins -= 1;
                core::hint::spin_loop();
            }
            dwc.write(reg::diepint(0), u32::MAX);
        }
        dwc.flush_tx_fifo(0);
    }

    // ── Endpoint I/O ────────────────────────────────────────────────────────

    fn write(&self, address: EndpointAddress, data: &[u8]) -> UsbResult<usize> {
        let number = address.number() as usize;
        let dwc = self.dwc;
        let state = self.state.borrow();

        let config = state
            .in_eps
            .get(number)
            .copied()
            .unwrap_or(EndpointConfig::free());
        if !config.allocated {
            return Err(UsbError::InvalidEndpoint);
        }
        if data.len() > config.max_packet as usize {
            return Err(UsbError::BufferOverflow);
        }
        if dwc.read(reg::diepctl(number)) & epctl::EPENA != 0 {
            return Err(UsbError::WouldBlock);
        }

        let size = data.len() as u32;
        if number == 0 {
            dwc.write(
                reg::DIEPTSIZ0,
                (size & dieptsiz0::XFERSIZE_MASK) | (1 << dieptsiz0::PKTCNT_SHIFT),
            );
            dwc.set(reg::DIEPCTL0, ep0ctl::CNAK | ep0ctl::EPENA);
        } else {
            dwc.write(
                reg::dieptsiz(number),
                (size & eptsiz::XFERSIZE_MASK) | (1 << eptsiz::PKTCNT_SHIFT),
            );
            dwc.set(reg::diepctl(number), epctl::CNAK | epctl::EPENA);
        }
        dwc.tx_fifo(number).push(data);
        Ok(data.len())
    }

    fn read(&self, address: EndpointAddress, buffer: &mut [u8]) -> UsbResult<usize> {
        let number = address.number() as usize;
        let mut state = self.state.borrow_mut();

        if !state.out_eps.get(number).is_some_and(|c| c.allocated) {
            return Err(UsbError::InvalidEndpoint);
        }

        let len = state.out_len[number];
        if len == EMPTY {
            // Nothing staged: make sure the endpoint can accept the next packet.
            self.arm_out(&state, number);
            return Err(UsbError::WouldBlock);
        }

        let len = len as usize;
        let start = state.out_eps[number].staging as usize;
        let result = if len > buffer.len() {
            Err(UsbError::BufferOverflow)
        } else {
            buffer[..len].copy_from_slice(&state.out_pool[start..start + len]);
            Ok(len)
        };
        // The packet is consumed either way: a too-small buffer is a caller bug,
        // but holding the packet forever would stall the endpoint.
        state.out_len[number] = EMPTY;
        self.arm_out(&state, number);
        result
    }

    fn set_stalled(&self, address: EndpointAddress, stalled: bool) {
        let number = address.number() as usize;
        if number >= MAX_SUPPORTED_ENDPOINTS {
            return;
        }
        let dwc = self.dwc;
        let state = self.state.borrow();

        match address.direction() {
            Direction::In => {
                if !state.in_eps[number].allocated {
                    return;
                }
                if stalled {
                    dwc.set(reg::diepctl(number), stall_bit(number) | nak_bit(number));
                } else {
                    dwc.clear(reg::diepctl(number), epctl::STALL);
                    // Restart the data toggle from DATA0.
                    if number != 0 {
                        dwc.set(reg::diepctl(number), epctl::SETD0PID);
                    }
                    dwc.set(reg::diepctl(number), epctl::CNAK);
                }
            }
            Direction::Out => {
                if !state.out_eps[number].allocated {
                    return;
                }
                if stalled {
                    dwc.set(reg::doepctl(number), stall_bit(number) | nak_bit(number));
                } else {
                    dwc.clear(reg::doepctl(number), epctl::STALL);
                    if number != 0 {
                        dwc.set(reg::doepctl(number), epctl::SETD0PID);
                    }
                    dwc.set(reg::doepctl(number), epctl::CNAK);
                    self.arm_out(&state, number);
                }
            }
        }
    }

    fn is_stalled(&self, address: EndpointAddress) -> bool {
        let number = address.number() as usize;
        if number >= MAX_SUPPORTED_ENDPOINTS {
            return false;
        }
        let dwc = self.dwc;
        let control = match address.direction() {
            Direction::In => dwc.read(reg::diepctl(number)),
            Direction::Out => dwc.read(reg::doepctl(number)),
        };
        control & epctl::STALL != 0
    }

    fn set_device_address(&self, address: u8) {
        self.dwc.modify(
            reg::DCFG,
            dcfg::DEVADDR_MASK,
            (address as u32) << dcfg::DEVADDR_SHIFT,
        );
    }

    fn reset(&self) {
        let mut state = self.state.borrow_mut();
        for len in state.out_len.iter_mut() {
            *len = EMPTY;
        }
        state.pending = Pending::default();
    }

    fn force_reset(&self) -> UsbResult<()> {
        let dwc = self.dwc;
        dwc.set(reg::DCTL, SFTRST_DISCON);
        // Hold SE0 long enough for the host's disconnect detector (a few
        // milliseconds at the default 360 MHz core clock).
        for _ in 0..1_000_000 {
            core::hint::spin_loop();
        }
        dwc.clear(reg::DCTL, SFTRST_DISCON);
        Ok(())
    }

    fn suspend(&self) {
        // The core stops its own clocks when the host stops sending SOFs; the
        // endpoints keep their configuration, so there is nothing to do here.
    }

    fn resume(&self) {
        // Counters and PHY state are restored by the core on resume.
    }
}

/// `DIEPCTL0`/`DOEPCTL0` encode the packet size in two bits.
fn ep0_mps_code(max_packet: u16) -> u32 {
    let code = match max_packet {
        8 => ep0ctl::MPS_8,
        16 => ep0ctl::MPS_16,
        32 => ep0ctl::MPS_32,
        _ => ep0ctl::MPS_64,
    };
    code | ep0ctl::USBACTEP
}

/// `EPTYPE` field for endpoints 1..=15.
fn ep_type_bits(ep_type: EndpointType) -> u32 {
    match ep_type {
        EndpointType::Control => epctl::EPTYPE_CONTROL,
        EndpointType::Isochronous { .. } => epctl::EPTYPE_ISO,
        EndpointType::Bulk => epctl::EPTYPE_BULK,
        EndpointType::Interrupt => epctl::EPTYPE_INTR,
    }
}

/// `STALL` bit of an endpoint control register (EP0 uses a different layout).
fn stall_bit(number: usize) -> u32 {
    if number == 0 {
        ep0ctl::STALL
    } else {
        epctl::STALL
    }
}

/// `SNAK` bit of an endpoint control register.
fn nak_bit(number: usize) -> u32 {
    if number == 0 {
        ep0ctl::SNAK
    } else {
        epctl::SNAK
    }
}

// ── Resource bundles ─────────────────────────────────────────────────────────

/// Clock, pad and wrapper blocks the full-speed core needs.
pub struct FsResources<'a> {
    /// `HP_SYS_CLKRST`: bus clock gate.
    pub clkrst: &'a HP_SYS_CLKRST,
    /// `LP_AON_CLKRST`: PHY clock gate and module reset.
    pub lp_aon_clkrst: &'a LP_AON_CLKRST,
    /// `USB_WRAP`: PHY/pad control.
    pub usb_wrap: &'a UsbWrap,
    /// `GPIO`: routes the OTG status inputs.
    pub gpio: &'a GPIO,
    /// `IO_MUX`: switches GPIO26/27 to the PHY function.
    pub io_mux: &'a IO_MUX,
}

/// Clock and PHY blocks the high-speed core needs.
pub struct HsResources<'a> {
    /// `HP_SYS_CLKRST`: bus clock gate.
    pub clkrst: &'a HP_SYS_CLKRST,
    /// `LP_AON_CLKRST`: PHY reference clock and resets.
    pub lp_aon_clkrst: &'a LP_AON_CLKRST,
    /// `HP_SYS`: `OTG_SUSPENDM`, needed for VBUS detection.
    pub hp_sys: &'a HP_SYS,
    /// `LP_SYS`: host/device pull-down control on newer revisions.
    pub lp_sys: &'a LP_SYS,
}

// ── USB FS ───────────────────────────────────────────────────────────────────

static FS_TAKEN: AtomicBool = AtomicBool::new(false);

/// The full-speed OTG core (`USB_OTG11`).
pub struct UsbFs {
    core: Core,
}

impl UsbFs {
    /// Power up the full-speed core and return it ready for allocation.
    ///
    /// # Panics
    ///
    /// Panics when called twice: the controller is a singleton, and two drivers
    /// would fight over the same registers.
    pub fn new(resources: FsResources<'_>) -> Self {
        assert!(
            !FS_TAKEN.swap(true, Ordering::Relaxed),
            "UsbFs::new called twice: the OTG FS core is already in use"
        );

        let clkrst = resources.clkrst;
        let lp_aon = resources.lp_aon_clkrst;

        // Bus clock, PHY clock, then a reset pulse.
        clkrst.soc_clk_ctrl1.set_reg_usb_otg11_sys_clk_en();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl0
            .set_lp_aonclkrst_usb_otg11_48m_clk_en();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .set_lp_aonclkrst_rst_en_usb_otg11();
        for _ in 0..64 {
            core::hint::spin_loop();
        }
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .clr_lp_aonclkrst_rst_en_usb_otg11();

        // Wrapper: internal PHY, pads on, clocks forced on so register access
        // never depends on the automatic clock gating.
        let otg_conf = &resources.usb_wrap.otg_conf;
        otg_conf.set_usb_pad_enable();
        otg_conf.clr_phy_sel();
        otg_conf.set_clk_en();
        otg_conf.set_ahb_clk_force_on();
        otg_conf.set_phy_clk_force_on();
        // The PHY owns the D+/D- pull-ups and pull-downs in device mode.
        otg_conf.clr_pad_pull_override();

        // The OTG core takes its session inputs from the GPIO matrix; present
        // the levels of a self-powered peripheral on a powered bus.
        route_otg_input!(resources.gpio.func110_in_sel_cfg, true); // IDDIG
        route_otg_input!(resources.gpio.func111_in_sel_cfg, false); // AVALID
        route_otg_input!(resources.gpio.func112_in_sel_cfg, true); // BVALID
        route_otg_input!(resources.gpio.func113_in_sel_cfg, true); // VBUSVALID
        route_otg_input!(resources.gpio.func114_in_sel_cfg, false); // SESSEND

        // GPIO26/27 are the OTG FS data pads; IO MUX function 0 is the analog
        // PHY function (`USB1P1_N1` / `USB1P1_P1`).
        configure_usb_pad!(resources.io_mux.gpio26);
        configure_usb_pad!(resources.io_mux.gpio27);

        Self {
            core: Core::new(Speed::Full),
        }
    }

    /// Give the core back so it can be initialised again.
    pub fn free(self) {
        FS_TAKEN.store(false, Ordering::Relaxed);
    }
}

impl UsbBus for UsbFs {
    fn alloc_ep(
        &mut self,
        direction: Direction,
        address: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        self.core
            .alloc_ep(direction, address, ep_type, max_packet_size, interval)
    }

    fn enable(&mut self) {
        self.core.enable();
    }

    fn reset(&self) {
        self.core.reset();
    }

    fn set_device_address(&self, address: u8) {
        self.core.set_device_address(address);
    }

    fn poll(&self) -> PollResult {
        self.core.poll()
    }

    fn write(&self, address: EndpointAddress, data: &[u8]) -> UsbResult<usize> {
        self.core.write(address, data)
    }

    fn read(&self, address: EndpointAddress, buffer: &mut [u8]) -> UsbResult<usize> {
        self.core.read(address, buffer)
    }

    fn set_stalled(&self, address: EndpointAddress, stalled: bool) {
        self.core.set_stalled(address, stalled);
    }

    fn is_stalled(&self, address: EndpointAddress) -> bool {
        self.core.is_stalled(address)
    }

    fn suspend(&self) {
        self.core.suspend();
    }

    fn resume(&self) {
        self.core.resume();
    }

    fn force_reset(&self) -> UsbResult<()> {
        self.core.force_reset()
    }

    // The DWC answers the status stage at the new address, so the address has
    // to be programmed before the ZLP goes out.
    const SET_ADDRESS_BEFORE_STATUS: bool = true;
}

// ── USB HS ───────────────────────────────────────────────────────────────────

static HS_TAKEN: AtomicBool = AtomicBool::new(false);

/// The high-speed OTG core (`USB_OTG20`).
pub struct UsbHs {
    core: Core,
}

impl UsbHs {
    /// Power up the high-speed core and return it ready for allocation.
    ///
    /// # Panics
    ///
    /// Panics when called twice.
    pub fn new(resources: HsResources<'_>) -> Self {
        assert!(
            !HS_TAKEN.swap(true, Ordering::Relaxed),
            "UsbHs::new called twice: the OTG HS core is already in use"
        );

        let clkrst = resources.clkrst;
        let lp_aon = resources.lp_aon_clkrst;

        // Bus clock and PHY reference clock, then reset the core and its PHY.
        clkrst.soc_clk_ctrl1.set_reg_usb_otg20_sys_clk_en();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .set_lp_aonclkrst_usb_otg20_phyref_clk_en();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .set_lp_aonclkrst_rst_en_usb_otg20();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .set_lp_aonclkrst_rst_en_usb_otg20_phy();
        for _ in 0..64 {
            core::hint::spin_loop();
        }
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .clr_lp_aonclkrst_rst_en_usb_otg20();
        lp_aon
            .lp_aonclkrst_hp_usb_clkrst_ctrl1
            .clr_lp_aonclkrst_rst_en_usb_otg20_phy();

        // Without `OTG_SUSPENDM` the PHY never reports a disconnect.
        resources.hp_sys.usbotg20_ctrl.set_otg_suspendm();
        // Start out with the "in suspend" flag clear so the PMU wake-up logic
        // does not think the port is idle.
        resources.lp_sys.usb_ctrl.clr_usbotg20_in_suspend();

        // Low-speed support: parallel mode plus the keep-alive timer. Both bits
        // are what ESP-IDF programs as well; the remaining defaults are kept.
        let fc_06 = (USBPHY_BASE + USBPHY_FC_06) as *mut u32;
        // SAFETY: `USBPHY` is a fixed MMIO block.
        unsafe { fc_06.write_volatile(fc_06.read_volatile() | (1 << 0) | (1 << 3)) };

        // Device mode: the 15 kΩ pull-downs have to be off. From chip revision
        // 3.0 they are software controlled; older revisions leave them to the
        // OTG core and ignore the write.
        let phy_ctrl = (LP_SYS::PTR as usize + LP_SYS_HS_PHY_CTRL) as *mut u32;
        // SAFETY: fixed MMIO inside the LP_SYS block.
        unsafe {
            let bits = phy_ctrl.read_volatile();
            phy_ctrl.write_volatile(bits & !(HS_PHY_CTRL_DP_PULLDOWN | HS_PHY_CTRL_DM_PULLDOWN));
        }
        Self {
            core: Core::new(Speed::High),
        }
    }

    /// Give the core back so it can be initialised again.
    pub fn free(self) {
        HS_TAKEN.store(false, Ordering::Relaxed);
    }
}

impl UsbBus for UsbHs {
    fn alloc_ep(
        &mut self,
        direction: Direction,
        address: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        self.core
            .alloc_ep(direction, address, ep_type, max_packet_size, interval)
    }

    fn enable(&mut self) {
        self.core.enable();
    }

    fn reset(&self) {
        self.core.reset();
    }

    fn set_device_address(&self, address: u8) {
        self.core.set_device_address(address);
    }

    fn poll(&self) -> PollResult {
        self.core.poll()
    }

    fn write(&self, address: EndpointAddress, data: &[u8]) -> UsbResult<usize> {
        self.core.write(address, data)
    }

    fn read(&self, address: EndpointAddress, buffer: &mut [u8]) -> UsbResult<usize> {
        self.core.read(address, buffer)
    }

    fn set_stalled(&self, address: EndpointAddress, stalled: bool) {
        self.core.set_stalled(address, stalled);
    }

    fn is_stalled(&self, address: EndpointAddress) -> bool {
        self.core.is_stalled(address)
    }

    fn suspend(&self) {
        self.core.suspend();
    }

    fn resume(&self) {
        self.core.resume();
    }

    fn force_reset(&self) -> UsbResult<()> {
        self.core.force_reset()
    }

    const SET_ADDRESS_BEFORE_STATUS: bool = true;
}
