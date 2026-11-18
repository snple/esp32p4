//! 10/100 Ethernet MAC (DesignWare GMAC) with an RMII PHY interface.
//!
//! The ESP32-P4 embeds a Synopsys DesignWare 10/100 MAC plus its own DMA engine.
//! The MAC talks to an external PHY over RMII (4 data lines, 50 MHz reference
//! clock) and configures that PHY through MDIO.
//!
//! # Wiring
//!
//! ```text
//!   ESP32-P4                 PHY (e.g. LAN8720 / DP83848)
//!   ─────────────────────    ────────────────────────────
//!   EMAC_TXEN    ──────────► TX_EN
//!   EMAC_TXD0/1  ──────────► TXD0/1
//!   EMAC_RXDV    ◄────────── CRS_DV
//!   EMAC_RXD0/1  ◄────────── RXD0/1
//!   EMAC_RMII_CLK◄────────── 50 MHz reference clock
//!   EMAC_MDC     ──────────► MDC
//!   EMAC_MDO/MDI ◄─────────► MDIO
//! ```
//!
//! On this chip the data and clock pads are **dedicated GMAC functions of the
//! IO MUX** (function 3), not GPIO-matrix signals: selecting the function wires
//! the pad straight to the MAC, output enable included. That also means
//! `GPIO_ENABLE` has no say over them — ESP-IDF notes in `gpio_hal_iomux_out`
//! that "as long as the func sel is not GPIO, the oe can only be controlled by
//! the peripheral" — so the driver leaves the GPIO output-enable register alone
//! for these pads.
//!
//! MDC and MDIO are the exception: they have no dedicated pad function and go
//! through the GPIO matrix, on any pin. Their output enable is still left to the
//! MAC (`OEN_SEL = 0`), because MDIO is bidirectional — the MAC has to stop
//! driving the bus before the PHY answers.
//!
//! # DMA
//!
//! Frames move through descriptor rings in memory. Provide the storage once,
//! from a `static`:
//!
//! ```ignore
//! static mut BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();
//! ```
//!
//! The rings are walked by the MAC, so the buffers must stay alive and must be
//! kept coherent with the CPU caches — the driver takes care of the latter
//! through [`crate::cache`]. Placing the storage in the PSRAM uncached window
//! works too and skips the cache maintenance.
//!
//! # Example
//!
//! ```ignore
//! let pins = RmiiPins::new(
//!     pins.pin::<33>(), // TXEN
//!     pins.pin::<34>(), // TXD0
//!     pins.pin::<35>(), // TXD1
//!     pins.pin::<30>(), // RXD1
//!     pins.pin::<29>(), // RXD0
//!     pins.pin::<28>(), // CRS_DV
//!     pins.pin::<32>(), // REF_CLK
//!     pins.pin::<20>(), // MDC
//!     pins.pin::<21>(), // MDIO
//! );
//!
//! let mut emac = Emac::init(
//!     dp.EMAC,
//!     pins,
//!     EmacConfig {
//!         mac_address: [0x02, 0, 0, 0, 0, 0x01],
//!         ..EmacConfig::default()
//!     },
//!     unsafe { &mut *core::ptr::addr_of_mut!(BUFFERS) },
//!     EmacResources {
//!         gpio: &gpio,
//!         io_mux: &io_mux,
//!         hp_sys: &dp.HP_SYS,
//!         clkrst: &dp.HP_SYS_CLKRST,
//!         lp_clkrst: &dp.LP_AON_CLKRST,
//!     },
//! );
//!
//! emac.transmit(&frame)?;
//! ```

pub mod dma;

use esp32p4_pac::{EMAC, GPIO, HP_SYS, HP_SYS_CLKRST, IO_MUX, LP_AON_CLKRST};

use crate::gpio::Pin;
pub use dma::{DmaBuffer, EmacBuffers, TxError};

/// GPIO matrix index of the MAC's MDIO input.
///
/// The GPIO matrix numbers input signals from 1, so the register index is one
/// less than the signal number used by the datasheet tables.
const SIGNAL_EMAC_MDI: u32 = 107;
/// GPIO matrix index of the MDC output.
const SIGNAL_EMAC_MDC: u32 = 108;
/// GPIO matrix index of the MDO output.
const SIGNAL_EMAC_MDO: u32 = 109;

/// IO MUX function number that connects a pad to the EMAC.
const IO_MUX_FUNCTION_EMAC: u32 = 3;
/// Drive strength setting used for EMAC pads (~20 mA), matching ESP-IDF.
const PAD_DRIVE_20MA: u32 = 2;

/// `IN_SEL` for the input signal reverse-mapping register.
const FN_IN_SEL_SIGNAL_OFFSET: u32 = 1;

/// MDC clock divider encoding for a system clock of 250 MHz and above
/// (`SYS_CLK / 124`, about 2.9 MHz at 360 MHz). PHYs tolerate the slight
/// overshoot of the 2.5 MHz clause-22 limit.
const MDC_CSR_CLOCK_RANGE: u32 = 5;

/// Spin ticks to wait for the MDIO transaction to finish.
const MDIO_TIMEOUT: u32 = 100_000;

/// Link speed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speed {
    /// 10 Mbit/s.
    _10M,
    /// 100 Mbit/s.
    _100M,
}

/// Link duplex mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Duplex {
    /// Half duplex.
    Half,
    /// Full duplex.
    Full,
}

/// What the PHY reports about the link.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LinkState {
    /// Whether a link partner was detected.
    pub up: bool,
    /// Negotiated speed; meaningless while `up` is false.
    pub speed: Speed,
    /// Negotiated duplex mode; meaningless while `up` is false.
    pub duplex: Duplex,
}

impl LinkState {
    /// A link that is down, reported with the given fallback settings.
    const fn down() -> Self {
        Self {
            up: false,
            speed: Speed::_100M,
            duplex: Duplex::Full,
        }
    }
}

/// Where the 50 MHz RMII reference clock comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefClock {
    /// The PHY generates 50 MHz and drives it into the `REF_CLK` pad. The usual
    /// arrangement when the PHY has its own crystal or oscillator.
    External,
    /// The MPLL produces 500 MHz; a /10 divider puts 50 MHz on the
    /// `REF_50M_CLK` pad, which must be wired back to the `REF_CLK` pad
    /// externally. Use this when the PHY has no clock source of its own.
    InternalMpll,
}

/// Driver configuration.
#[derive(Clone, Copy, Debug)]
pub struct EmacConfig {
    /// Station address programmed into MAC filter slot 0.
    pub mac_address: [u8; 6],
    /// MDIO address of the PHY (usually 0 or 1, depends on the PHY strapping).
    pub phy_address: u8,
    /// Speed the MAC starts with, before the PHY reported anything.
    pub speed: Speed,
    /// Duplex mode the MAC starts with.
    pub duplex: Duplex,
    /// Accept every frame instead of filtering on the station address.
    pub promiscuous: bool,
    /// RMII reference clock source.
    pub clock: RefClock,
}

impl Default for EmacConfig {
    fn default() -> Self {
        Self {
            mac_address: [0x02, 0x00, 0x00, 0x00, 0x00, 0x01],
            phy_address: 0,
            speed: Speed::_100M,
            duplex: Duplex::Full,
            promiscuous: false,
            clock: RefClock::External,
        }
    }
}

/// The RMII pins, addressed by GPIO number.
///
/// The data and clock entries name pads that carry the **dedicated GMAC IO MUX
/// function**; the MAC drives their direction itself once that function is
/// selected, so they never behave like plain GPIO. MDC and MDIO are ordinary
/// GPIOs routed through the matrix and can be any pin.
///
/// [`RmiiPins::new`] takes [`Pin`] handles, so the numbers are checked by the
/// type system; the constructor additionally rejects pads that lack the GMAC
/// function at compile time.
pub struct RmiiPins {
    txen: u8,
    txd0: u8,
    txd1: u8,
    rx_d1: u8,
    rx_d0: u8,
    crs_dv: u8,
    ref_clk: u8,
    mdc: u8,
    mdio: u8,
}

impl RmiiPins {
    /// Collect the pins of an RMII interface.
    ///
    /// `rx_d1` comes before `rx_d0` on purpose: it keeps the argument order
    /// matching the pin numbering of the reference boards.
    #[allow(clippy::too_many_arguments)]
    pub fn new<
        const TXEN: u32,
        const TXD0: u32,
        const TXD1: u32,
        const RXD1: u32,
        const RXD0: u32,
        const CRS_DV: u32,
        const REF_CLK: u32,
        const MDC: u32,
        const MDIO: u32,
    >(
        _txen: Pin<TXEN>,
        _txd0: Pin<TXD0>,
        _txd1: Pin<TXD1>,
        _rx_d1: Pin<RXD1>,
        _rx_d0: Pin<RXD0>,
        _crs_dv: Pin<CRS_DV>,
        _ref_clk: Pin<REF_CLK>,
        _mdc: Pin<MDC>,
        _mdio: Pin<MDIO>,
    ) -> Self {
        // Only three GPIO groups carry the EMAC IO MUX function, and not every
        // signal is available on every pin of a group. These checks are const
        // evaluated, so a wrong pin is a compile error.
        const {
            assert!(
                matches!(TXEN, 33 | 40 | 49),
                "GPIO has no EMAC_TXEN function"
            )
        };
        const { assert!(matches!(TXD0, 34 | 41), "GPIO has no EMAC_TXD0 function") };
        const { assert!(matches!(TXD1, 35 | 42), "GPIO has no EMAC_TXD1 function") };
        const {
            assert!(
                matches!(RXD1, 30 | 47 | 53),
                "GPIO has no EMAC_RXD1 function"
            )
        };
        const {
            assert!(
                matches!(RXD0, 29 | 46 | 52),
                "GPIO has no EMAC_RXD0 function"
            )
        };
        const {
            assert!(
                matches!(CRS_DV, 28 | 45 | 51),
                "GPIO has no EMAC_RXDV function"
            )
        };
        const {
            assert!(
                matches!(REF_CLK, 32 | 44 | 50),
                "GPIO has no EMAC_RMII_CLK function"
            )
        };
        const { assert!(MDC < 54 && MDIO < 54, "MDC/MDIO must be a GPIO pin") };
        const { assert!(MDC != MDIO, "MDC and MDIO need separate pins") };

        Self {
            txen: TXEN as u8,
            txd0: TXD0 as u8,
            txd1: TXD1 as u8,
            rx_d1: RXD1 as u8,
            rx_d0: RXD0 as u8,
            crs_dv: CRS_DV as u8,
            ref_clk: REF_CLK as u8,
            mdc: MDC as u8,
            mdio: MDIO as u8,
        }
    }
}

/// Borrowed clock/reset/pin blocks the driver needs.
///
/// None of them are taken by value: the EMAC only flips a few clock-tree bits
/// and IO MUX pads, and other drivers (USB, GPIO) share the same blocks.
pub struct EmacResources<'a> {
    /// GPIO block, for the matrix routing of MDC and MDIO.
    pub gpio: &'a GPIO,
    /// IO MUX block, for the RMII pads.
    pub io_mux: &'a IO_MUX,
    /// `HP_SYS.GMAC_CTRL0` selects RMII versus MII.
    pub hp_sys: &'a HP_SYS,
    /// EMAC clock gates and dividers.
    pub clkrst: &'a HP_SYS_CLKRST,
    /// LP-domain pad gates and the EMAC reset.
    pub lp_clkrst: &'a LP_AON_CLKRST,
}

/// Ethernet MAC driver.
pub struct Emac<'d, const RX: usize, const TX: usize> {
    emac: EMAC,
    tx: dma::TxRing<'d>,
    rx: dma::RxRing<'d>,
    address: [u8; 6],
    phy_address: u8,
    speed: Speed,
    duplex: Duplex,
}

impl<'d, const RX: usize, const TX: usize> Emac<'d, RX, TX> {
    /// Bring the MAC, DMA and PHY interface up and start receiving.
    pub fn init(
        emac: EMAC,
        pins: RmiiPins,
        config: EmacConfig,
        buffers: &'d mut EmacBuffers<RX, TX>,
        res: EmacResources<'_>,
    ) -> Self {
        assert!(
            config.mac_address != [0; 6] && config.mac_address != [0xff; 6],
            "MAC address must be a unicast station address"
        );

        configure_clock(&res, config.clock);
        configure_pins(&res, &pins);

        // The DMA needs a soft reset before anything else; doing it after the
        // clocks are up also clears whatever the bootloader left behind.
        dma_soft_reset(&emac);

        // Program the ring pointers, then build the rings themselves.
        let (tx, rx) = dma::rings(buffers);
        emac.dmatxbaseaddr.write_bits(tx.base_ptr() as u32);
        emac.dmarxbaseaddr.write_bits(rx.base_ptr() as u32);

        // Keep the peripheral's interrupt output quiet: this driver polls.
        emac.dmain_en.write_bits(0);
        emac.emacintmask.write_bits(u32::MAX);

        configure_mac(&emac, &config);
        program_mac_address(&emac, &config.mac_address);

        // Start in the documented order: MAC transmitter, TX DMA, RX DMA, MAC
        // receiver. Starting the DMA first was seen to wedge the transmitter.
        emac.emacconfig.set_emactx();
        emac.dmaoperation_mode.set_start_stop_transmission_command();
        emac.dmaoperation_mode.set_start_stop_rx();
        emac.emacconfig.set_emacrx();

        // The receive path owns all descriptors from here on, so make sure the
        // compiler cannot reorder the ring writes past the start above.
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

        Self {
            emac,
            tx,
            rx,
            address: config.mac_address,
            phy_address: config.phy_address,
            speed: config.speed,
            duplex: config.duplex,
        }
    }

    /// The station address programmed into the MAC.
    #[inline]
    pub fn mac_address(&self) -> [u8; 6] {
        self.address
    }

    /// Current MAC speed setting.
    #[inline]
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// Current MAC duplex setting.
    #[inline]
    pub fn duplex(&self) -> Duplex {
        self.duplex
    }

    /// Reconfigure the MAC for a new speed.
    pub fn set_speed(&mut self, speed: Speed) {
        if speed == self.speed {
            return;
        }
        self.speed = speed;
        let hundred = speed == Speed::_100M;
        self.emac.emacconfig.modify_bits(|value| {
            if hundred {
                value | EMACCONFIG_FESPEED
            } else {
                value & !EMACCONFIG_FESPEED
            }
        });
    }

    /// Reconfigure the MAC for a new duplex mode.
    pub fn set_duplex(&mut self, duplex: Duplex) {
        if duplex == self.duplex {
            return;
        }
        self.duplex = duplex;
        let full = duplex == Duplex::Full;
        self.emac.emacconfig.modify_bits(|value| {
            if full {
                value | EMACCONFIG_DUPLEX
            } else {
                value & !EMACCONFIG_DUPLEX
            }
        });
    }

    /// Queue one Ethernet frame (header, payload, no FCS).
    pub fn transmit(&mut self, frame: &[u8]) -> Result<(), TxError> {
        self.tx.transmit(frame)?;

        // A frame in the ring does not move until the DMA is poked.
        self.emac.dmatxpolldemand.write_bits(0);
        Ok(())
    }

    /// Whether a transmit slot is free.
    #[inline]
    pub fn transmit_ready(&self) -> bool {
        self.tx.has_capacity()
    }

    /// Return the next received frame, if any.
    ///
    /// The slice stays valid until [`Emac::release_frame`]; dropping the
    /// driver instead also returns the slot to the MAC.
    pub fn receive(&mut self) -> Option<&[u8]> {
        self.rx.receive()
    }

    /// Hand the frame returned by [`Emac::receive`] back to the MAC.
    pub fn release_frame(&mut self) {
        self.rx.release();
    }

    /// Read a PHY register over MDIO (IEEE 802.3 clause 22).
    pub fn mdio_read(&self, phy: u8, register: u8) -> u16 {
        self.emac.emacmiiaddr.write_bits(
            ((phy as u32 & 0x1f) << 11)
                | ((register as u32 & 0x1f) << 6)
                | (MDC_CSR_CLOCK_RANGE << 2),
        );
        // Bit 0 is the busy flag: writing it starts the transaction.
        self.emac.emacmiiaddr.modify_bits(|v| v & !0b11 | 0b01);

        if !self.wait_mdio() {
            return 0xffff;
        }
        self.emac.emacmiidata.read_bits() as u16
    }

    /// Write a PHY register over MDIO.
    ///
    /// Returns `false` if the PHY did not acknowledge within the timeout, which
    /// usually means the address is wrong or no PHY is wired up.
    pub fn mdio_write(&self, phy: u8, register: u8, value: u16) -> bool {
        self.emac.emacmiidata.write_bits(value as u32);
        self.emac.emacmiiaddr.write_bits(
            ((phy as u32 & 0x1f) << 11)
                | ((register as u32 & 0x1f) << 6)
                | (MDC_CSR_CLOCK_RANGE << 2)
                | 0b10,
        );

        if !self.wait_mdio() {
            return false;
        }
        self.emac.emacmiiaddr.read_bits() & 0b10 != 0
    }

    /// Spin until the MDIO transaction completes.
    fn wait_mdio(&self) -> bool {
        let mut spins = MDIO_TIMEOUT;
        while self.emac.emacmiiaddr.read_bits() & 0b01 != 0 {
            if spins == 0 {
                return false;
            }
            spins -= 1;
            core::hint::spin_loop();
        }
        true
    }

    /// Read the link state from the PHY and apply the negotiated speed/duplex.
    ///
    /// Reads the standard clause-22 registers only, so it works with any PHY.
    /// Poll it every second or so, or after an interrupt if the PHY has one.
    pub fn poll_link(&mut self) -> LinkState {
        // BMSR bit 2 latches low, so the first read may report a stale "down".
        let _ = self.mdio_read(self.phy_address, PHY_BMSR);
        let bmsr = self.mdio_read(self.phy_address, PHY_BMSR);

        if bmsr == u16::MAX || bmsr & BMSR_LINK_STATUS == 0 {
            return LinkState::down();
        }

        let bmcr = self.mdio_read(self.phy_address, PHY_BMCR);
        let (speed, duplex) = if bmcr & BMCR_ANENABLE == 0 {
            // Forced mode: the PHY reports what it was told to do.
            (
                if bmcr & BMCR_SPEED100 != 0 {
                    Speed::_100M
                } else {
                    Speed::_10M
                },
                if bmcr & BMCR_FULL_DUPLEX != 0 {
                    Duplex::Full
                } else {
                    Duplex::Half
                },
            )
        } else {
            // Everyone advertises 10/100, so the partner's ability word tells
            // us what was picked.
            let partner = self.mdio_read(self.phy_address, PHY_ANLPAR);
            match partner {
                _ if partner & ANLPAR_100FD != 0 => (Speed::_100M, Duplex::Full),
                _ if partner & ANLPAR_100HD != 0 => (Speed::_100M, Duplex::Half),
                _ if partner & ANLPAR_10FD != 0 => (Speed::_10M, Duplex::Full),
                _ => (Speed::_10M, Duplex::Half),
            }
        };

        self.set_speed(speed);
        self.set_duplex(duplex);

        LinkState {
            up: true,
            speed,
            duplex,
        }
    }

    /// Restart auto-negotiation and wait (up to ~5 s) for it to finish.
    ///
    /// Returns the resulting link state; `up == false` means either the link
    /// partner never appeared or no PHY answered on MDIO.
    pub fn autonegotiate(&mut self) -> LinkState {
        // Advertise everything we support, then restart negotiation.
        self.mdio_write(self.phy_address, PHY_ANAR, ANAR_ALL_10_100);
        let bmcr = self.mdio_read(self.phy_address, PHY_BMCR);
        self.mdio_write(
            self.phy_address,
            PHY_BMCR,
            (bmcr & !BMCR_SPEED_MASK) | BMCR_ANENABLE | BMCR_ANRESTART,
        );

        // 5 s at 1 ms per step.
        for _ in 0..5000 {
            let bmsr = self.mdio_read(self.phy_address, PHY_BMSR);
            if bmsr != u16::MAX && bmsr & BMSR_ANEGCOMPLETE != 0 {
                return self.poll_link();
            }
            spin_millis(1);
        }

        LinkState::down()
    }
}

// ── EMACCONFIG bit helpers ───────────────────────────────────────────────────

const EMACCONFIG_FESPEED: u32 = 1 << 14;
const EMACCONFIG_DUPLEX: u32 = 1 << 11;

// ── Standard PHY registers and bits (IEEE 802.3 clause 22) ───────────────────

const PHY_BMCR: u8 = 0;
const PHY_BMSR: u8 = 1;
const PHY_ANAR: u8 = 4;
const PHY_ANLPAR: u8 = 5;

const BMCR_SPEED_MASK: u16 = (1 << 13) | (1 << 6);
const BMCR_SPEED100: u16 = 1 << 13;
const BMCR_ANRESTART: u16 = 1 << 9;
const BMCR_ANENABLE: u16 = 1 << 12;
const BMCR_FULL_DUPLEX: u16 = 1 << 8;

const BMSR_LINK_STATUS: u16 = 1 << 2;
const BMSR_ANEGCOMPLETE: u16 = 1 << 5;

const ANAR_ALL_10_100: u16 = 0x01e1;

const ANLPAR_10FD: u16 = 1 << 6;
const ANLPAR_100HD: u16 = 1 << 7;
const ANLPAR_100FD: u16 = 1 << 8;

// ── Initialisation helpers ───────────────────────────────────────────────────

/// Enable the clock tree and release the MAC from reset.
fn configure_clock(res: &EmacResources<'_>, clock: RefClock) {
    // RMII instead of MII.
    res.hp_sys.gmac_ctrl0.write_phy_intf_sel(4);

    // Route the combined 50 MHz reference clock into both the RMII logic and the
    // receive path. Nothing is driven out yet; `InternalMpll` turns the pad
    // driver on further down.
    res.clkrst.peri_clk_ctrl00.clr_reg_pad_emac_ref_clk_en();
    res.clkrst
        .peri_clk_ctrl00
        .write_reg_emac_rmii_clk_src_sel(0);
    res.clkrst.peri_clk_ctrl00.set_reg_emac_rmii_clk_en();
    res.clkrst.peri_clk_ctrl00.clr_reg_emac_rx_clk_src_sel();
    res.clkrst.peri_clk_ctrl00.set_reg_emac_rx_clk_en();

    res.clkrst.peri_clk_ctrl01.clr_reg_emac_tx_clk_src_sel();
    res.clkrst.peri_clk_ctrl01.set_reg_emac_tx_clk_en();
    // Divide by one: 100 Mbit/s. 10 Mbit/s is handled by the MAC, not the pad.
    res.clkrst.peri_clk_ctrl01.write_reg_emac_rx_clk_div_num(1);
    res.clkrst.peri_clk_ctrl01.write_reg_emac_tx_clk_div_num(1);

    res.lp_clkrst
        .lp_aonclkrst_hp_clk_ctrl
        .clr_lp_aonclkrst_hp_pad_emac_tx_clk_en();
    res.lp_clkrst
        .lp_aonclkrst_hp_clk_ctrl
        .clr_lp_aonclkrst_hp_pad_emac_rx_clk_en();
    res.lp_clkrst
        .lp_aonclkrst_hp_clk_ctrl
        .set_lp_aonclkrst_hp_pad_emac_txrx_clk_en();

    if clock == RefClock::InternalMpll {
        res.lp_clkrst
            .lp_aonclkrst_hp_clk_ctrl
            .set_lp_aonclkrst_hp_mpll_500m_clk_en();
        // 500 MHz / (9 + 1) = 50 MHz.
        res.clkrst.ref_clk_ctrl0.write_reg_ref_50m_clk_div_num(9);
        res.clkrst.ref_clk_ctrl1.set_reg_ref_50m_clk_en();
    }

    // Pulse the EMAC reset; the HP reset gate does not cover this block.
    res.lp_clkrst
        .lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
        .set_lp_aonclkrst_rst_en_emac();
    res.lp_clkrst
        .lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
        .clr_lp_aonclkrst_force_norst_emac();
    res.lp_clkrst
        .lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
        .clr_lp_aonclkrst_rst_en_emac();

    if clock == RefClock::InternalMpll {
        // `configure` above cleared the pad driver, and the reset pulse needs
        // the clock to be running again before the pad can drive it.
        res.clkrst.peri_clk_ctrl00.set_reg_pad_emac_ref_clk_en();
    }

    // The clock tree, and the PHY's own oscillator, need a moment to settle.
    spin_millis(1);
}

/// Configure the IO MUX pads and the GPIO matrix routing of MDC/MDIO.
fn configure_pins(res: &EmacResources<'_>, pins: &RmiiPins) {
    // The IO MUX clock gate is shared with the GPIO driver; opening it twice is
    // harmless and keeps the EMAC usable without initialising `Pins`.
    res.clkrst.peri_clk_ctrl26.set_reg_iomux_clk_en();
    res.clkrst.soc_clk_ctrl3.set_reg_iomux_apb_clk_en();
    res.clkrst.hp_rst_en1.clr_reg_rst_en_iomux();

    // RMII data and clock pads are dedicated GMAC functions of the IO MUX, not
    // GPIO-matrix signals: selecting function 3 hard-wires the pad to the MAC,
    // including its output enable. GPIO_ENABLE is bypassed in that case, so
    // nothing here touches it.
    for pin in [
        pins.txen,
        pins.txd0,
        pins.txd1,
        pins.crs_dv,
        pins.rx_d0,
        pins.rx_d1,
        pins.ref_clk,
    ] {
        configure_emac_pad(pin);
    }

    // MDC and MDIO have no dedicated pad function, so they do go through the
    // GPIO matrix: MDC is output only, MDIO is bidirectional with the MAC
    // driving the enable.
    configure_matrix_pad(pins.mdc, false);
    configure_output_signal(pins.mdc, SIGNAL_EMAC_MDC);

    configure_matrix_pad(pins.mdio, true);
    configure_output_signal(pins.mdio, SIGNAL_EMAC_MDO);
    configure_input_signal(pins.mdio, SIGNAL_EMAC_MDI);
}

/// Point an IO MUX pad at the EMAC function.
///
/// Once `MCU_SEL` names a peripheral function the pad's direction is owned by
/// that peripheral — ESP-IDF spells this out in `gpio_hal_iomux_out`: "as long
/// as the func sel is not GPIO, the oe can only be controlled by the
/// peripheral". Setting `GPIO_ENABLE` would therefore do nothing, and the MAC
/// drives TXEN/TXD itself.
fn configure_emac_pad(pin: u8) {
    write_io_mux(pin, |value| {
        (value
            & !(esp32p4_pac::io_mux::Gpio0::MCU_SEL_MASK
                | esp32p4_pac::io_mux::Gpio0::FUN_DRV_MASK
                | esp32p4_pac::io_mux::Gpio0::FUN_WPU_MASK
                | esp32p4_pac::io_mux::Gpio0::FUN_WPD_MASK))
            | (IO_MUX_FUNCTION_EMAC << esp32p4_pac::io_mux::Gpio0::MCU_SEL_OFFSET)
            | (PAD_DRIVE_20MA << esp32p4_pac::io_mux::Gpio0::FUN_DRV_OFFSET)
            | esp32p4_pac::io_mux::Gpio0::FUN_IE_MASK
    });
}

/// Configure a pad that the GPIO matrix drives.
fn configure_matrix_pad(pin: u8, input: bool) {
    write_io_mux(pin, |value| {
        let mut value = value
            & !(esp32p4_pac::io_mux::Gpio0::MCU_SEL_MASK
                | esp32p4_pac::io_mux::Gpio0::FUN_DRV_MASK);
        // Function 1 is plain GPIO, which puts the pad behind the GPIO matrix.
        value |= 1 << esp32p4_pac::io_mux::Gpio0::MCU_SEL_OFFSET;
        value |= PAD_DRIVE_20MA << esp32p4_pac::io_mux::Gpio0::FUN_DRV_OFFSET;
        if input {
            value |= esp32p4_pac::io_mux::Gpio0::FUN_IE_MASK;
        } else {
            value &= !esp32p4_pac::io_mux::Gpio0::FUN_IE_MASK;
        }
        value
    });
}

/// Route `signal` out through `pin`.
fn configure_output_signal(pin: u8, signal: u32) {
    let reg = matrix_out_sel_cfg(pin);
    // `OEN_SEL` (bit 10) stays clear so the peripheral owns the output enable:
    // MDC's own enable is tied high, while MDO's follows `gmii_mdo_oe_pad_out`
    // and lets the MAC turn the bus around for the PHY's response.
    // SAFETY: `pin` is a valid GPIO number and the register takes any value.
    unsafe { core::ptr::write_volatile(reg, signal & 0x1ff) };
}

/// Route `pin` into `signal`.
fn configure_input_signal(pin: u8, signal: u32) {
    let reg = matrix_in_sel_cfg(signal);
    // Bit 7 (`SEL`) keeps the GPIO matrix in the path instead of bypassing it.
    // SAFETY: the signal index is fixed and in range.
    unsafe { core::ptr::write_volatile(reg, (pin as u32 & 0x3f) | (1 << 7)) };
}

/// `FUNC<pin>_OUT_SEL_CFG` for a GPIO number.
fn matrix_out_sel_cfg(pin: u8) -> *mut u32 {
    let base = GPIO::PTR as usize
        + core::mem::offset_of!(esp32p4_pac::gpio::RegisterBlock, func0_out_sel_cfg);
    (base + pin as usize * 4) as *mut u32
}

/// `FUNC<signal>_IN_SEL_CFG` for a peripheral input signal.
///
/// The array starts at signal 1, so the register for signal `n` sits at
/// `n - 1` slots from the base.
fn matrix_in_sel_cfg(signal: u32) -> *mut u32 {
    let base = GPIO::PTR as usize
        + core::mem::offset_of!(esp32p4_pac::gpio::RegisterBlock, func1_in_sel_cfg);
    (base + (signal as usize - FN_IN_SEL_SIGNAL_OFFSET as usize) * 4) as *mut u32
}

/// Read-modify-write an IO MUX register addressed by GPIO number.
fn write_io_mux(pin: u8, f: impl FnOnce(u32) -> u32) {
    assert!(pin < 54, "ESP32-P4 has GPIO0..GPIO53");
    // The `IO_MUX_GPIO%s` array starts at offset 4 and steps by 4 bytes.
    let ptr = (IO_MUX::PTR as usize + 4 + pin as usize * 4) as *mut u32;
    // SAFETY: `pin` is bounds-checked and the register is plain MMIO.
    unsafe {
        let value = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, f(value));
    }
}

/// Put the DMA engine into its reset state and select the 32-byte descriptor.
fn dma_soft_reset(emac: &EMAC) {
    emac.dmabusmode.modify_bits(|v| v | 1 | (1 << 7)); // SW_RST | ALT_DESC_SIZE
    for _ in 0..1000 {
        if emac.dmabusmode.read_bits() & 1 == 0 {
            break;
        }
        core::hint::spin_loop();
    }

    // Bus mode defaults from ESP-IDF `emac_hal_init_dma_default`: 32-beat
    // bursts shared by both directions, address-aligned bursts, mixed burst,
    // round-robin arbitration.
    emac.dmabusmode.modify_bits(|v| {
        let mut v = v & !((0x3f << 8)    // PROG_BURST_LEN
                | (0x3 << 14)  // PRI_RATIO
                | (1 << 1)     // DMA_ARB_SCH
                | (1 << 16)    // FIXED_BURST
                | (1 << 23)    // USE_SEP_PBL
                | (1 << 24)); // PBLX8_MODE
        v |= 32 << 8; // 32 beats per burst
        v |= 1 << 25; // DMAADDRALIBEA: address-aligned bursts
        v |= 1 << 26; // DMAMIXEDBURST
        v
    });

    // Flush the TX FIFO, then set store-and-forward thresholds.
    emac.dmaoperation_mode.write_bits(1 << 20);
    for _ in 0..1000 {
        if emac.dmaoperation_mode.read_bits() & (1 << 20) == 0 {
            break;
        }
        core::hint::spin_loop();
    }

    emac.dmaoperation_mode.modify_bits(|v| {
        (v & !((0x7 << 14) | (0x3 << 3) | (1 << 26))) | (1 << 2) // OPT_SECOND_FRAME: overlap frame processing
    });
}

/// Program the MAC for the configured speed, duplex and filter mode.
fn configure_mac(emac: &EMAC, config: &EmacConfig) {
    emac.emacconfig.modify_bits(|v| {
        let mut v = v & !((1 << 3) | (1 << 2)); // TX and RX off while configuring
        // Port select: 1 = MII/RMII (10/100), 0 = GMII (1000).
        v |= 1 << 15;
        // Watchdog, jabber and retry stay at their enabled defaults.
        v |= (1 << 23) | (1 << 22) | (1 << 9);
        // Pass the FCS up; the driver strips it and never transmits one.
        v &= !(1 << 7);
        // Receive own transmission in half duplex.
        v |= 1 << 13;
        if config.speed == Speed::_100M {
            v |= EMACCONFIG_FESPEED;
        } else {
            v &= !EMACCONFIG_FESPEED;
        }
        if config.duplex == Duplex::Full {
            v |= EMACCONFIG_DUPLEX;
        } else {
            v &= !EMACCONFIG_DUPLEX;
        }
        v
    });

    emac.emacff.modify_bits(|v| {
        // Pass all multicast addresses; unicast filtering stays on slot 0.
        let mut v = v | (1 << 4);
        if config.promiscuous {
            v |= 1 << 31; // RECEIVE_ALL
        } else {
            v &= !(1 << 31);
        }
        v
    });
}

/// Program station address filter slot 0.
fn program_mac_address(emac: &EMAC, address: &[u8; 6]) {
    let high = (address[5] as u32) << 8 | address[4] as u32;
    let low = address[3] as u32
        | (address[2] as u32) << 8
        | (address[1] as u32) << 16
        | (address[0] as u32) << 24;
    emac.emacaddr0high.write_bits(high | 1 << 31); // ADDRESS_ENABLE0
    emac.emacaddr0low.write_bits(low);
}

/// Rough busy-wait, good enough for the sub-millisecond clock settling and the
/// auto-negotiation poll.
fn spin_millis(ms: u32) {
    let start = crate::delay::micros();
    while crate::delay::micros().wrapping_sub(start) < ms as u64 * 1000 {}
}
