//! Host-mode driver for the ESP32-P4's two DWC OTG cores.
//!
//! [`UsbHostFs`] and [`UsbHostHs`] implement [`HostController`] on top of one
//! shared `Core`: a transaction programs one channel, starts it and waits for
//! the core to halt it again, so the driver needs no scheduler and no buffering
//! of its own.
//!
//! # Powering the port
//!
//! The chip has no VBUS switch: `HPRT.PrtPwr` only tells the core that the port
//! is powered, it does not drive 5 V anywhere. A board that supplies VBUS from a
//! GPIO has to turn it on before [`Host::enumerate`](crate::host::Host::enumerate),
//! and a board with a permanently powered port needs nothing.
//!
//! # Timing
//!
//! The driver is synchronous and has no time base of its own, so the delays USB
//! mandates (port reset recovery, connect debounce) are spin loops whose length
//! is estimated from the default 360 MHz core clock. They are deliberately
//! generous: a reset that lasts a few milliseconds longer than the specification
//! requires is harmless.
//!
//! # Hubs
//!
//! The driver programs `HCCHAR.LSPDDev` for a low-speed device behind a hub, so
//! a full-speed hub and its low-speed devices work on either core. It does not
//! implement split transactions, so a FS/LS device behind a high-speed hub is
//! rejected with [`HostError::Unsupported`](crate::host::HostError::Unsupported).
//! [`UsbHostHs::new_fsls_only`] avoids the problem by keeping the high-speed
//! core at FS/LS, which makes the hub enumerate at full speed.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_usb::host::Host;
//! use esp32p4_usb::{FsResources, UsbHostFs};
//!
//! let host = UsbHostFs::new(FsResources {
//!     clkrst: &dp.HP_SYS_CLKRST,
//!     lp_aon_clkrst: &dp.LP_AON_CLKRST,
//!     usb_wrap: &dp.USB_WRAP,
//!     gpio: &dp.GPIO,
//!     io_mux: &dp.IO_MUX,
//! });
//! let bus = Host::new(&host);
//! let device = bus.enumerate().unwrap();
//! ```

use core::cell::RefCell;
use core::sync::atomic::Ordering;

use crate::dwc::{
    Dwc, gintsts, gotgctl, grxstsp, gusbcfg, hcchar, hcfg, hcint, hctsiz, hfir, hprt, htxsts, reg,
};
use crate::endpoint::{Direction, EndpointType};
use crate::host::controller::{
    Channel, DeviceAddress, EndpointConfig, HostController, PortStatus, Speed, Toggle,
};
use crate::host::error::{HostError, HostResult};
use crate::log::{debug, warn};
use crate::resources::Kind;
use crate::resources::{FS_TAKEN, FsResources, HS_TAKEN, HsResources, route_otg_input};

/// Channels the core can implement, and the size of the driver's table.
const MAX_CHANNELS: usize = 16;

/// Spin iterations per millisecond.
///
/// `spin_loop()` compiles to `pause`, which retires in a couple of cycles at the
/// default 360 MHz core clock. The exact figure does not matter: every delay in
/// the driver only has to be long enough.
const TICKS_PER_MS: u32 = 300_000;

/// Times a NAK is tolerated before the transfer is reported as timed out.
///
/// A HID endpoint that changed nothing answers every poll with a NAK, so this
/// has to cover a few hundred milliseconds.
const NAK_RETRIES: u32 = 20_000;

/// Transaction errors tolerated for one packet.
const ERROR_RETRIES: u32 = 3;

/// Spin iterations to wait for a channel to halt (about 20 ms).
const HALT_SPINS: u32 = 6_000_000;

/// Spin iterations to wait for the RX status entry after an IN transfer.
const RX_SETTLE_SPINS: u32 = 10_000;

/// Spin iterations between two NAK retries: about a microsecond.
const NAK_RETRY_TICKS: u32 = TICKS_PER_MS / 1000;

/// How long a port reset is held (USB 2.0 §7.1.7.5 asks for 10–20 ms).
const RESET_HOLD_MS: u32 = 20;

/// Time the device has to recover from a reset before it answers requests.
const RESET_RECOVERY_MS: u32 = 10;

/// How long the driver waits for the port to be enabled after a reset.
const PORT_ENABLE_MS: u32 = 100;

// ── Channel bookkeeping ──────────────────────────────────────────────────────

/// What a channel is programmed for.
#[derive(Clone, Copy)]
struct ChannelState {
    used: bool,
    max_packet: u16,
    transfer: EndpointType,
    /// `HCCHAR` without the direction and enable bits.
    hcchar: u32,
}

impl ChannelState {
    const fn free() -> Self {
        Self {
            used: false,
            max_packet: 0,
            transfer: EndpointType::Control,
            hcchar: 0,
        }
    }
}

struct State {
    channels: [ChannelState; MAX_CHANNELS],
}

impl State {
    const fn new() -> Self {
        Self {
            channels: [ChannelState::free(); MAX_CHANNELS],
        }
    }
}

/// The PID a transaction starts with.
#[derive(Clone, Copy)]
enum Pid {
    Setup,
    Data(Toggle),
}

impl Pid {
    fn bits(self) -> u32 {
        match self {
            Self::Setup => hctsiz::PID_SETUP,
            Self::Data(Toggle::Data0) => hctsiz::PID_DATA0,
            Self::Data(Toggle::Data1) => hctsiz::PID_DATA1,
        }
    }
}

// ── Core ─────────────────────────────────────────────────────────────────────

/// Host-mode implementation shared by both OTG cores.
struct Core {
    dwc: Dwc,
    /// Which core this is.
    kind: Kind,
    /// Largest packet any endpoint of this core can use.
    max_packet: u16,
    /// Channels the core actually implements.
    channel_count: usize,
    /// The high-speed core is limited to full and low speed.
    fsls_only: bool,
    state: RefCell<State>,
}

impl Core {
    fn new(kind: Kind, fsls_only: bool) -> Self {
        let dwc = Dwc::new(kind.base());
        let channel_count = dwc.channel_count().min(MAX_CHANNELS);
        let core = Self {
            dwc,
            kind,
            max_packet: kind.max_packet(),
            channel_count,
            fsls_only,
            state: RefCell::new(State::new()),
        };
        core.init();
        core
    }

    /// Force host mode and bring the core up.
    fn init(&self) {
        let dwc = self.dwc;

        dwc.soft_reset();
        dwc.flush_tx_fifo(0);
        dwc.flush_tx_fifo(1);
        dwc.flush_rx_fifo();
        // The bootloader parks the PHY in suspend before handing over; take it
        // out of that state or every register access stalls.
        dwc.write(reg::PCGCCTL, 0);

        // Host mode. The full-speed core uses its embedded PHY, the high-speed
        // core the UTMI+ one with its 5-clock turnaround.
        let mut usbcfg = gusbcfg::FHMOD;
        match self.kind {
            // UTMI+ with the 8-bit interface, a 5-PHY-clock turnaround and the
            // timeout calibration the PHY datasheet asks for.
            Kind::High => {
                usbcfg |=
                    gusbcfg::PHYIF | (5 << gusbcfg::USBTRDTIM_SHIFT) | (5 << gusbcfg::TOCAL_SHIFT)
            }
            // Embedded full-speed PHY; 9 turnaround clocks is the reset value
            // and suits the 48 MHz clock.
            _ => usbcfg |= gusbcfg::PHYSEL | (9 << gusbcfg::USBTRDTIM_SHIFT),
        }
        dwc.write(reg::GUSBCFG, usbcfg);
        while dwc.read(reg::GINTSTS) & gintsts::CURMOD == 0 {
            core::hint::spin_loop();
        }

        // Slave mode, no interrupts: the driver polls everything it needs.
        dwc.write(reg::GAHBCFG, 0);
        dwc.write(reg::GINTMSK, 0);
        dwc.write(reg::HAINTMSK, 0);

        // FIFO split: two maximum-size packets of slack for the RX FIFO, one
        // packet each for the non-periodic and periodic TX FIFOs.
        let words = u32::from(self.max_packet).div_ceil(4);
        let rx = 2 * words + 8;
        let nptx = words + 8;
        let ptx = words + 8;
        assert!(
            rx + nptx + ptx <= u32::from(dwc.fifo_depth()),
            "the USB data FIFO is too small for {}-byte packets",
            self.max_packet
        );
        dwc.write(reg::GRXFSIZ, rx);
        dwc.write(reg::GNPTXFSIZ, rx | (nptx << 16));
        dwc.write(reg::HPTXFSIZ, (rx + nptx) | (ptx << 16));
        dwc.flush_tx_fifo(0);
        dwc.flush_tx_fifo(1);
        dwc.flush_rx_fifo();

        if !self.kind.is_high() {
            // The embedded full-speed PHY runs at 48 MHz.
            dwc.modify(reg::HCFG, hcfg::FSLSPCLKSEL_MASK, hcfg::FSLSPCLKSEL_48M);
            dwc.modify(reg::HFIR, hfir::FRINT_MASK, hfir::FRINT_48M);
        }

        // A high-speed core can be limited to FS/LS. That is what makes a hub
        // usable without a transaction translator: the hub enumerates at full
        // speed and its downstream FS/LS devices need no split transactions.
        if self.fsls_only {
            dwc.modify(reg::HCFG, hcfg::FSLSSUPP, hcfg::FSLSSUPP);
        }

        // A-session: the chip has no OTG pins, so present the levels of a host
        // that supplies VBUS itself instead of waiting for a session that can
        // never start.
        let control = dwc.read(reg::GOTGCTL);
        dwc.write(
            reg::GOTGCTL,
            (control & !(gotgctl::BVALIDOVEN | gotgctl::BVALIDOVVAL))
                | gotgctl::AVALIDOVEN
                | gotgctl::AVALIDOVVAL
                | gotgctl::VBVALIDOVEN
                | gotgctl::VBVALIDOVVAL,
        );

        self.update_port(|value| value | hprt::PRTPWR);
        debug!(
            "USB host: {} core up, {} channels, {} FIFO words",
            match self.kind {
                Kind::Full => "full-speed",
                Kind::High => "high-speed",
            },
            self.channel_count,
            dwc.fifo_depth()
        );
    }

    /// Spin for `ticks` iterations.
    fn wait_ticks(ticks: u32) {
        for _ in 0..ticks {
            core::hint::spin_loop();
        }
    }

    /// Update `HPRT` without acknowledging the write-one-to-clear bits.
    fn update_port(&self, f: impl FnOnce(u32) -> u32) {
        let hprt = self.dwc.read(reg::HPRT);
        self.dwc.write(reg::HPRT, f(hprt & !hprt::W1C_MASK));
    }

    // ── Port ────────────────────────────────────────────────────────────────

    fn port_status(&self) -> PortStatus {
        let hprt = self.dwc.read(reg::HPRT);
        let enabled = hprt & hprt::PRTENA != 0;
        PortStatus {
            connected: hprt & hprt::PRTCONNSTS != 0,
            enabled,
            speed: if enabled {
                speed_from(hprt)
            } else if self.fsls_only {
                Speed::Full
            } else {
                self.kind.speed()
            },
            overcurrent: hprt & hprt::PRTOVRCURRACT != 0,
        }
    }

    fn reset_port(&self) -> HostResult<PortStatus> {
        let dwc = self.dwc;
        if !self.port_status().connected {
            return Err(HostError::Disconnected);
        }

        // Select FS/LS-only before the reset, so the core never starts the
        // high-speed chirp handshake.
        if self.fsls_only {
            dwc.modify(reg::HCFG, hcfg::FSLSSUPP, hcfg::FSLSSUPP);
        }

        self.update_port(|value| value | hprt::PRTRST);
        Self::wait_ticks(RESET_HOLD_MS.saturating_mul(TICKS_PER_MS));
        self.update_port(|value| value & !hprt::PRTRST);

        let mut waited = 0;
        while waited < PORT_ENABLE_MS {
            if dwc.read(reg::HPRT) & hprt::PRTENA != 0 {
                break;
            }
            Self::wait_ticks(TICKS_PER_MS);
            waited += 1;
        }

        let status = self.port_status();
        if !status.enabled {
            warn!("USB host: port did not enable after reset");
            return Err(HostError::Disconnected);
        }

        // The low-speed PHY clock is the 48 MHz one divided by eight, and the
        // core has to be told so before the first transfer.
        if !self.kind.is_high() {
            let (clock, interval) = match status.speed {
                Speed::Low => (hcfg::FSLSPCLKSEL_6M, hfir::FRINT_6M),
                _ => (hcfg::FSLSPCLKSEL_48M, hfir::FRINT_48M),
            };
            dwc.modify(reg::HCFG, hcfg::FSLSPCLKSEL_MASK, clock);
            dwc.modify(reg::HFIR, hfir::FRINT_MASK, interval);
        }

        // Whatever the previous device left in the FIFOs is meaningless now.
        dwc.flush_tx_fifo(0);
        dwc.flush_tx_fifo(1);
        dwc.flush_rx_fifo();
        // Acknowledge the port events; PRTENA is write-one-to-disable, so it
        // has to be masked out of the write.
        let hprt = dwc.read(reg::HPRT);
        dwc.write(reg::HPRT, hprt & !hprt::PRTENA);

        // Reset recovery: the device needs time before it answers requests.
        Self::wait_ticks(RESET_RECOVERY_MS.saturating_mul(TICKS_PER_MS));
        debug!("USB host: port enabled, speed {:?}", status.speed);
        Ok(status)
    }

    // ── Channels ────────────────────────────────────────────────────────────

    fn alloc_channel(&self) -> HostResult<Channel> {
        let mut state = self.state.borrow_mut();
        for (index, channel) in state.channels.iter_mut().enumerate() {
            if index < self.channel_count && !channel.used {
                *channel = ChannelState::free();
                channel.used = true;
                return Ok(Channel::new(index as u8));
            }
        }
        Err(HostError::NoChannel)
    }

    fn free_channel(&self, channel: Channel) {
        self.halt(channel);
        self.state.borrow_mut().channels[channel.index()] = ChannelState::free();
    }

    fn configure(
        &self,
        channel: Channel,
        device: DeviceAddress,
        endpoint: &EndpointConfig,
    ) -> HostResult<()> {
        let ep_type = match endpoint.transfer {
            EndpointType::Control => hcchar::EPTYPE_CONTROL,
            EndpointType::Bulk => hcchar::EPTYPE_BULK,
            EndpointType::Interrupt => hcchar::EPTYPE_INTR,
            EndpointType::Isochronous { .. } => return Err(HostError::Unsupported),
        };
        if endpoint.max_packet == 0 || endpoint.max_packet > self.max_packet {
            return Err(HostError::Overflow);
        }
        // Split transactions (a FS/LS device behind a high-speed hub) need the
        // channel's split register; this driver does not implement them.
        if endpoint.route.and_then(|route| route.translator).is_some() {
            return Err(HostError::Unsupported);
        }

        self.halt(channel);

        let mut hcchar = (u32::from(endpoint.max_packet) & hcchar::MPS_MASK)
            | (u32::from(endpoint.address.number()) << hcchar::EPNUM_SHIFT)
            | ep_type
            // MC/EC must not stay at its reserved zero value.
            | (1 << hcchar::EC_SHIFT)
            | (u32::from(device.raw()) << hcchar::DEVADDR_SHIFT);
        // A low-speed device behind a hub needs the low-speed signalling bit.
        // On the root port the core reads the speed from `HPRT` itself.
        if endpoint.route.is_some() && endpoint.speed == Speed::Low {
            hcchar |= hcchar::LSPDDEV;
        }
        let index = channel.index();
        self.dwc.write(reg::hcchar(index), hcchar);
        self.dwc.write(reg::hctsiz(index), hctsiz::PID_DATA0);
        self.dwc.write(reg::hcint(index), u32::MAX);
        self.dwc.write(reg::hcintmsk(index), 0);

        self.state.borrow_mut().channels[index] = ChannelState {
            used: true,
            max_packet: endpoint.max_packet,
            transfer: endpoint.transfer,
            hcchar,
        };
        Ok(())
    }

    /// Ask the core to halt a channel and wait until it did.
    fn halt(&self, channel: Channel) {
        let dwc = self.dwc;
        let index = channel.index();
        let control = dwc.read(reg::hcchar(index));
        if control & hcchar::CHENA != 0 {
            // ChEna and ChDis together: the core flushes the pending requests,
            // finishes the transaction in flight and clears both bits.
            dwc.write(reg::hcchar(index), control | hcchar::CHENA | hcchar::CHDIS);
            let mut spins = HALT_SPINS;
            while dwc.read(reg::hcint(index)) & hcint::CHHLTD == 0 && spins > 0 {
                spins -= 1;
                core::hint::spin_loop();
            }
        }
        dwc.write(reg::hcint(index), u32::MAX);
    }

    /// Wait until the core halts the channel and report its interrupt flags.
    fn wait_halt(&self, channel: Channel) -> HostResult<u32> {
        let dwc = self.dwc;
        let index = channel.index();
        let mut spins = HALT_SPINS;

        loop {
            let flags = dwc.read(reg::hcint(index));
            if flags & hcint::CHHLTD != 0 {
                dwc.write(reg::hcint(index), flags);
                return Ok(flags);
            }
            if spins == 0 {
                // The channel is stuck: it has to be halted before it can be
                // used again, but whatever it did report is still interesting.
                let flags = dwc.read(reg::hcint(index));
                self.halt(channel);
                dwc.write(reg::hcint(index), u32::MAX);
                return if flags == 0 {
                    Err(HostError::Timeout)
                } else {
                    Ok(flags)
                };
            }
            spins -= 1;
            // A device that is pulled out of the port never completes anything.
            if spins.is_multiple_of(4096) && dwc.read(reg::HPRT) & hprt::PRTCONNSTS == 0 {
                self.halt(channel);
                dwc.write(reg::hcint(index), u32::MAX);
                return Err(HostError::Disconnected);
            }
            core::hint::spin_loop();
        }
    }

    /// Pop every pending RX status entry, copying the payload of `channel` into
    /// `buffer`.
    fn drain_rx(&self, channel: Channel, buffer: &mut [u8]) -> usize {
        let dwc = self.dwc;
        let mut received = 0;

        while dwc.rx_level() {
            let status = dwc.rx_status();
            let number = (status & grxstsp::EPNUM_MASK) as usize;
            let length = ((status & grxstsp::BCNT_MASK) >> grxstsp::BCNT_SHIFT) as usize;
            let packet = (status & grxstsp::PKTSTS_MASK) >> grxstsp::PKTSTS_SHIFT;

            // Only "IN data packet received" carries a payload; the entries
            // that follow a transfer are empty.
            if packet == grxstsp::PKTSTS_IN_DATA {
                if number == channel.index() && received + length <= buffer.len() {
                    dwc.rx_fifo()
                        .pop(&mut buffer[received..received + length], length);
                    received += length;
                } else {
                    dwc.rx_fifo().discard(length);
                }
            }
        }
        received
    }

    /// Run one transaction on `channel`.
    ///
    /// `out` carries the payload of an OUT or SETUP transaction, `in_buffer`
    /// receives the payload of an IN one; exactly one of them is used, and
    /// neither may exceed the endpoint's maximum packet size.
    fn run(
        &self,
        channel: Channel,
        direction: Direction,
        pid: Pid,
        out: &[u8],
        in_buffer: &mut [u8],
    ) -> HostResult<usize> {
        let dwc = self.dwc;
        let index = channel.index();
        let config = self.state.borrow().channels[index];
        if !config.used {
            return Err(HostError::Protocol);
        }

        let is_out = direction == Direction::Out;
        let length = if is_out { out.len() } else { in_buffer.len() };
        if length > usize::from(config.max_packet) {
            return Err(HostError::Overflow);
        }

        // Control, bulk and the SETUP stage share the non-periodic TX FIFO;
        // interrupt OUT endpoints use the periodic one.
        let periodic = matches!(config.transfer, EndpointType::Interrupt);
        let direction_bits = if is_out { 0 } else { hcchar::EPDIR };
        let length = length as u32;
        let mut naks = 0;
        let mut errors = 0;

        loop {
            if dwc.read(reg::hcchar(index)) & hcchar::CHENA != 0 {
                self.halt(channel);
            }
            dwc.write(reg::hcint(index), u32::MAX);

            if is_out {
                // Both TX FIFOs are shared between channels, so start from a
                // clean slate instead of tracking what a previous, possibly
                // abandoned, transfer left behind.
                dwc.flush_tx_fifo(if periodic { 1 } else { 0 });
                let words = length.div_ceil(4);
                let available = (if periodic {
                    dwc.read(reg::HPTXSTS)
                } else {
                    dwc.read(reg::GNPTXSTS)
                }) & htxsts::SPCAVAIL_MASK;
                if available < words {
                    return Err(HostError::Overflow);
                }
                if periodic {
                    dwc.p_tx_fifo().push(out);
                } else {
                    dwc.np_tx_fifo().push(out);
                }
            } else {
                // Whatever the RX FIFO still holds belongs to a transfer that
                // was abandoned; it must not be mistaken for this one's data.
                self.drain_rx(channel, &mut []);
            }

            dwc.write(
                reg::hctsiz(index),
                (length & hctsiz::XFERSIZE_MASK) | (1 << hctsiz::PKTCNT_SHIFT) | pid.bits(),
            );
            dwc.write(
                reg::hcchar(index),
                config.hcchar | direction_bits | hcchar::CHENA,
            );

            let flags = self.wait_halt(channel)?;

            let received = if is_out {
                out.len()
            } else {
                // The core fills the FIFO before it halts the channel.
                let mut settle = RX_SETTLE_SPINS;
                while !dwc.rx_level() && settle > 0 {
                    settle -= 1;
                    core::hint::spin_loop();
                }
                self.drain_rx(channel, in_buffer)
            };

            if flags & hcint::XFERCOMPL != 0 {
                return Ok(received);
            }

            if flags & hcint::STALL != 0 {
                return Err(HostError::Stall);
            }
            if flags & (hcint::AHBERR | hcint::BBLERR | hcint::XCS_XACT_ERR) != 0 {
                return Err(HostError::Transaction);
            }
            if flags & hcint::NACK != 0 {
                naks += 1;
                if naks > NAK_RETRIES {
                    return Err(HostError::Timeout);
                }
                Self::wait_ticks(NAK_RETRY_TICKS);
                continue;
            }
            if flags & hcint::RETRY_MASK != 0 {
                errors += 1;
                if errors > ERROR_RETRIES {
                    return Err(HostError::Transaction);
                }
                continue;
            }
            warn!("USB host: channel halted without status");
            return Err(HostError::Protocol);
        }
    }
}

/// Decode `HPRT.PrtSpd`.
fn speed_from(hprt: u32) -> Speed {
    match (hprt & hprt::PRTSPD_MASK) >> hprt::PRTSPD_SHIFT {
        0 => Speed::High,
        2 => Speed::Low,
        _ => Speed::Full,
    }
}

/// Implement [`HostController`] for a core by forwarding to its [`Core`].
macro_rules! impl_host_controller {
    ($type:ty) => {
        impl HostController for $type {
            fn port_status(&self) -> PortStatus {
                self.core.port_status()
            }

            fn reset_port(&self) -> HostResult<PortStatus> {
                self.core.reset_port()
            }

            fn alloc_channel(&self) -> HostResult<Channel> {
                self.core.alloc_channel()
            }

            fn free_channel(&self, channel: Channel) {
                self.core.free_channel(channel);
            }

            fn configure(
                &self,
                channel: Channel,
                device: DeviceAddress,
                endpoint: &EndpointConfig,
            ) -> HostResult<()> {
                self.core.configure(channel, device, endpoint)
            }

            fn setup(&self, channel: Channel, packet: &[u8]) -> HostResult<()> {
                setup(&self.core, channel, packet)
            }

            fn transfer_in(
                &self,
                channel: Channel,
                toggle: Toggle,
                buffer: &mut [u8],
            ) -> HostResult<usize> {
                self.core
                    .run(channel, Direction::In, Pid::Data(toggle), &[], buffer)
            }

            fn transfer_out(
                &self,
                channel: Channel,
                toggle: Toggle,
                data: &[u8],
            ) -> HostResult<()> {
                self.core
                    .run(channel, Direction::Out, Pid::Data(toggle), data, &mut [])?;
                Ok(())
            }

            fn wait_millis(&self, millis: u32) {
                Core::wait_ticks(millis.saturating_mul(TICKS_PER_MS));
            }
        }
    };
}

// ── USB FS ───────────────────────────────────────────────────────────────────

/// The full-speed OTG core (`USB_OTG11`) in host mode.
pub struct UsbHostFs {
    core: Core,
}

impl UsbHostFs {
    /// Power up the full-speed core and put it in host mode.
    ///
    /// # Panics
    ///
    /// Panics when called twice, or when the device driver already owns the
    /// core: they would fight over the same registers.
    pub fn new(resources: FsResources<'_>) -> Self {
        assert!(
            !FS_TAKEN.swap(true, Ordering::Relaxed),
            "the OTG FS core is already in use"
        );
        resources.power_up();

        // There is no VBUS sense pin, so tell the PHY the session is alive.
        let otg_conf = &resources.usb_wrap.otg_conf;
        otg_conf.clr_srp_sessend_value();
        otg_conf.set_srp_sessend_override();

        // The OTG core takes its session inputs from the GPIO matrix; present
        // the levels of an A-device (a host with VBUS on the connector).
        route_otg_input!(resources.gpio.func110_in_sel_cfg, false); // IDDIG
        route_otg_input!(resources.gpio.func111_in_sel_cfg, true); // AVALID
        route_otg_input!(resources.gpio.func112_in_sel_cfg, false); // BVALID
        route_otg_input!(resources.gpio.func113_in_sel_cfg, true); // VBUSVALID
        route_otg_input!(resources.gpio.func114_in_sel_cfg, false); // SESSEND

        Self {
            core: Core::new(Kind::Full, false),
        }
    }

    /// Give the core back so it can be initialised again.
    pub fn free(self) {
        FS_TAKEN.store(false, Ordering::Relaxed);
    }
}

impl_host_controller!(UsbHostFs);

// ── USB HS ───────────────────────────────────────────────────────────────────

/// The high-speed OTG core (`USB_OTG20`) in host mode.
pub struct UsbHostHs {
    core: Core,
}

impl UsbHostHs {
    /// Power up the high-speed core and put it in host mode.
    ///
    /// # Panics
    ///
    /// Panics when called twice, or when the device driver already owns the
    /// core.
    pub fn new(resources: HsResources<'_>) -> Self {
        Self::with_mode(resources, false)
    }

    /// Power up the high-speed core as a full/low-speed only host.
    ///
    /// The core stops chirping and never negotiates high speed, so every
    /// device (a hub included) enumerates at full speed and its downstream
    /// FS/LS devices need no split transactions. This is how a hub works on
    /// this port without a transaction translator; the cost is that a
    /// high-speed device falls back to full speed, or does not work at all if
    /// it has no full-speed mode.
    ///
    /// # Panics
    ///
    /// Panics when called twice, or when the device driver already owns the
    /// core.
    pub fn new_fsls_only(resources: HsResources<'_>) -> Self {
        Self::with_mode(resources, true)
    }

    fn with_mode(resources: HsResources<'_>, fsls_only: bool) -> Self {
        assert!(
            !HS_TAKEN.swap(true, Ordering::Relaxed),
            "the OTG HS core is already in use"
        );
        resources.power_up();
        // A host pulls both lines down to detect what was plugged in.
        resources.set_pull_downs(true);

        Self {
            core: Core::new(Kind::High, fsls_only),
        }
    }

    /// Give the core back so it can be initialised again.
    pub fn free(self) {
        HS_TAKEN.store(false, Ordering::Relaxed);
    }
}

impl_host_controller!(UsbHostHs);

// ── Shared pieces ────────────────────────────────────────────────────────────

/// Send a SETUP packet: always eight bytes, always with the SETUP PID.
fn setup(core: &Core, channel: Channel, packet: &[u8]) -> HostResult<()> {
    if packet.len() != 8 {
        return Err(HostError::Protocol);
    }
    core.run(channel, Direction::Out, Pid::Setup, packet, &mut [])?;
    Ok(())
}
