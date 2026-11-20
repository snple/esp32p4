//! Which core to drive, and the clock, PHY and pad plumbing that takes.
//!
//! The two OTG cores need quite different plumbing, and each driver needs the
//! same plumbing in both directions. This module powers a core up and hands out
//! the blocks that takes; the drivers only add what their direction requires:
//! the OTG session levels ([`FsResources`]) or the pull resistors
//! (`HsResources::set_pull_downs`).

use crate::host::Speed;

use core::sync::atomic::AtomicBool;

use esp32p4_pac::usb_wrap::RegisterBlock as UsbWrap;
use esp32p4_pac::{GPIO, HP_SYS, HP_SYS_CLKRST, IO_MUX, LP_AON_CLKRST, LP_SYS};

/// The USB PHY block, which is not part of the SVD.
const USBPHY_BASE: usize = 0x5009_c000;
/// `USBPHY.FC_06`: line-state helpers used by the UTMI+ PHY.
const USBPHY_FC_06: usize = 0x18;
/// `LP_SYS.HP_USB_OTGHS_PHY_CTRL`, present from chip revision 3.0 on.
const LP_SYS_HS_PHY_CTRL: usize = 0x204;
const HS_PHY_CTRL_DP_PULLDOWN: u32 = 1 << 1;
const HS_PHY_CTRL_DM_PULLDOWN: u32 = 1 << 2;

/// Register base of the full-speed OTG core (`USB_OTG11`).
pub const USB_OTG_FS_BASE: usize = 0x5004_0000;
/// Register base of the high-speed OTG core (`USB_OTG20`).
pub const USB_OTG_HS_BASE: usize = 0x5000_0000;

/// The OTG core a driver runs on.
///
/// Only the facts both directions need: which register block to poke and how
/// wide a packet may be. The device driver adds what `DCFG` and the FIFOs want,
/// the host driver what the PHY clock wants.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    /// `USB_OTG11`: 12 Mbit/s through the internal full-speed PHY.
    Full,
    /// `USB_OTG20`: 480 Mbit/s through the internal UTMI+ PHY.
    High,
}

impl Kind {
    /// Register base of the core.
    #[inline]
    pub(crate) const fn base(self) -> usize {
        match self {
            Self::Full => USB_OTG_FS_BASE,
            Self::High => USB_OTG_HS_BASE,
        }
    }

    /// Largest packet any endpoint or channel of this core can move.
    #[inline]
    pub(crate) const fn max_packet(self) -> u16 {
        match self {
            Self::Full => 64,
            Self::High => 512,
        }
    }

    /// `true` for the high-speed core.
    #[inline]
    pub(crate) const fn is_high(self) -> bool {
        matches!(self, Self::High)
    }

    /// Speed the core itself runs at, for [`PortStatus`].
    #[inline]
    pub(crate) const fn speed(self) -> Speed {
        match self {
            Self::Full => Speed::Full,
            Self::High => Speed::High,
        }
    }
}

/// Set once a driver owns the full-speed core, whichever direction it drives.
pub(crate) static FS_TAKEN: AtomicBool = AtomicBool::new(false);
/// Set once a driver owns the high-speed core, whichever direction it drives.
pub(crate) static HS_TAKEN: AtomicBool = AtomicBool::new(false);

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

impl FsResources<'_> {
    /// Bus clock, PHY clock, a reset pulse, the wrapper and the data pads.
    ///
    /// The OTG session inputs are left to the driver: the peripheral role and
    /// the host role need opposite levels.
    pub(crate) fn power_up(&self) {
        let clkrst = self.clkrst;
        let lp_aon = self.lp_aon_clkrst;

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
        let otg_conf = &self.usb_wrap.otg_conf;
        otg_conf.set_usb_pad_enable();
        otg_conf.clr_phy_sel();
        otg_conf.set_clk_en();
        otg_conf.set_ahb_clk_force_on();
        otg_conf.set_phy_clk_force_on();
        // The PHY owns the pull-up and pull-down resistors; the direction
        // decides which ones it applies.
        otg_conf.clr_pad_pull_override();

        // GPIO26/27 are the OTG FS data pads; IO MUX function 0 is the analog
        // PHY function (`USB1P1_N1` / `USB1P1_P1`).
        configure_usb_pad!(self.io_mux.gpio26);
        configure_usb_pad!(self.io_mux.gpio27);
    }
}

impl HsResources<'_> {
    /// Bus clock, PHY reference clock, resets and the PHY's low-speed support.
    pub(crate) fn power_up(&self) {
        let clkrst = self.clkrst;
        let lp_aon = self.lp_aon_clkrst;

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
        self.hp_sys.usbotg20_ctrl.set_otg_suspendm();
        // Start out with the "in suspend" flag clear so the PMU wake-up logic
        // does not think the port is idle.
        self.lp_sys.usb_ctrl.clr_usbotg20_in_suspend();

        // Low-speed support: parallel mode plus the keep-alive timer.
        let fc_06 = (USBPHY_BASE + USBPHY_FC_06) as *mut u32;
        // SAFETY: `USBPHY` is a fixed MMIO block.
        unsafe { fc_06.write_volatile(fc_06.read_volatile() | (1 << 0) | (1 << 3)) };
    }

    /// Connect or disconnect the 15 kΩ pull-downs on D+/D−.
    ///
    /// Host mode needs them, device mode must not drive them: the pull-ups are
    /// what tells the host that a peripheral was plugged in.
    pub(crate) fn set_pull_downs(&self, enable: bool) {
        let phy_ctrl = (LP_SYS::PTR as usize + LP_SYS_HS_PHY_CTRL) as *mut u32;
        let mask = HS_PHY_CTRL_DP_PULLDOWN | HS_PHY_CTRL_DM_PULLDOWN;
        // SAFETY: fixed MMIO inside the LP_SYS block.
        unsafe {
            let bits = phy_ctrl.read_volatile();
            phy_ctrl.write_volatile(if enable { bits | mask } else { bits & !mask });
        }
    }
}

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
pub(crate) use route_otg_input;

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
pub(crate) use configure_usb_pad;
